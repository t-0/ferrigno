use crate::absolutelineinfo::*;
use crate::character::*;
use crate::closure::*;
use crate::dumpstate::*;
use crate::functionstate::MAX_INT;
use crate::localvariable::*;
use crate::object::*;
use crate::prototype::PF_FIXED;
use crate::prototype::*;
use crate::state::*;
use crate::status::*;
use crate::table::*;
use crate::tagvariant::*;
use crate::tloadable::*;
use crate::tobject::*;
use crate::tstring::*;
use crate::tvalue::*;
use crate::upvaluedescription::*;
use crate::utility::*;
use crate::zio::*;
use core::mem::*;
use std::ptr::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LoadState {
    loadstate_interpreter: *mut State,
    loadstate_zio: *mut ZIO,
    loadstate_name: *const i8,
    loadstate_table: *mut Table,
    loadstate_nstr: u64,
    loadstate_fixed: bool,
}
impl LoadState {
    pub unsafe fn error(&mut self, why: *const i8) -> ! {
        unsafe {
            luao_pushfstring(
                self.loadstate_interpreter,
                c"%s: bad binary format (%s)".as_ptr(),
                &[self.loadstate_name.into(), why.into()],
            );
            luad_throw(self.loadstate_interpreter, Status::SyntaxError);
        }
    }
    pub unsafe fn load_block(&mut self, b: *mut std::ffi::c_void, size: usize) {
        unsafe {
            if (*self.loadstate_zio).luaz_read(b, size) != 0 {
                self.error(c"truncated chunk".as_ptr());
            }
        }
    }
    pub unsafe fn load_byte(&mut self) -> u8 {
        unsafe {
            match (*(self.loadstate_zio)).load_byte() {
                | None => {
                    self.error(c"truncated chunk".as_ptr());
                },
                | Some(x) => x,
            }
        }
    }
    pub unsafe fn load_unsigned(&mut self, mut limit: usize) -> usize {
        unsafe {
            let mut ret: usize = 0;
            limit >>= 7;
            loop {
                let b: i32 = self.load_byte() as i32;
                if ret >= limit {
                    self.error(c"integer overflow".as_ptr());
                }
                ret = ret << 7 | (b & 0x7f_i32) as usize;
                if b & 0x80_i32 != 0 {
                    break;
                }
            }
            ret
        }
    }
    pub unsafe fn load_size(&mut self) -> usize {
        unsafe { self.load_unsigned(!0usize) }
    }
    pub unsafe fn load_int(&mut self) -> i32 {
        unsafe { self.load_unsigned(MAX_INT) as i32 }
    }
    pub unsafe fn load_number(&mut self) -> f64 {
        unsafe {
            let mut x: f64 = 0.0;
            self.load_block(&mut x as *mut f64 as *mut std::ffi::c_void, size_of::<f64>());
            x
        }
    }
    pub unsafe fn load_integer(&mut self) -> i64 {
        unsafe {
            let mut x: i64 = 0;
            self.load_block(&mut x as *mut i64 as *mut std::ffi::c_void, size_of::<i64>());
            x
        }
    }
    pub unsafe fn load_string_n(&mut self, p: *mut Prototype) -> *mut TString {
        unsafe {
            let state: *mut State = self.loadstate_interpreter;
            let tstring: *mut TString;
            let mut size: usize = self.load_size();
            if size == 0 {
                let idx: usize = self.load_size();
                if idx == 0 {
                    return null_mut();
                }
                let mut slot: TValue = TValue::new(TagVariant::NilNil);
                let tag = luah_getint(self.loadstate_table, idx as i64, &mut slot);
                if !tag.to_tag_type().is_string() {
                    self.error(c"invalid string index".as_ptr());
                }
                tstring = slot.as_string().unwrap();
                if (*p).get_marked() & BLACKBIT != 0 && (*tstring).get_marked() & WHITEBITS != 0 {
                    Object::luac_barrier_(state, p as *mut Object, tstring as *mut Object);
                }
                return tstring;
            } else {
                size -= 1;
                if self.loadstate_fixed {
                    let addr = (*self.loadstate_zio).luaz_getaddr(size + 1);
                    if addr.is_null() {
                        self.error(c"truncated fixed chunk".as_ptr());
                    }
                    tstring = TString::create_external(state, addr as *const i8, size, None, null_mut());
                } else if size <= LUAI_MAXSHORTLEN {
                    let mut buffer: [i8; LUAI_MAXSHORTLEN + 1] = [0; LUAI_MAXSHORTLEN + 1];
                    self.load_block(buffer.as_mut_ptr() as *mut std::ffi::c_void, size + 1);
                    tstring = luas_newlstr(state, buffer.as_mut_ptr(), size);
                } else {
                    tstring = TString::create_long(state, size);
                    let io: *mut TValue = (*state).interpreter_top.stkidrel_pointer;
                    (*io).set_object(tstring as *mut Object, (*tstring).get_tagvariant());
                    (*state).luad_inctop();
                    self.load_block(((*tstring).get_contents_mut()) as *mut std::ffi::c_void, size + 1);
                    (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.sub(1);
                }
            }
            if (*p).get_marked() & BLACKBIT != 0 && (*tstring).get_marked() & WHITEBITS != 0 {
                Object::luac_barrier_(state, p as *mut Object, tstring as *mut Object);
            }
            self.loadstate_nstr += 1;
            let mut sv = TValue::new(TagVariant::NilNil);
            sv.set_object(tstring as *mut Object, (*tstring).get_tagvariant());
            luah_setint(state, self.loadstate_table, self.loadstate_nstr as i64, &mut sv);
            tstring
        }
    }
    pub unsafe fn load_string(&mut self, p: *mut Prototype) -> *mut TString {
        unsafe {
            let tstring = self.load_string_n(p);
            if tstring.is_null() {
                self.error(c"bad format for code_constant string".as_ptr());
            }
            tstring
        }
    }
    pub unsafe fn load_code(&mut self, prototype: *mut Prototype) {
        unsafe {
            let n = self.load_int();
            if size_of::<i32>() >= size_of::<usize>() && (n as usize) + 1 > ((!0usize) / size_of::<u32>()) {
                (*(self.loadstate_interpreter)).too_big();
            }
            if self.loadstate_fixed {
                let addr = (*self.loadstate_zio).luaz_getaddr((n as usize).wrapping_mul(size_of::<u32>()));
                if addr.is_null() {
                    self.error(c"truncated fixed chunk".as_ptr());
                }
                (*prototype).prototype_code.vectort_pointer = addr as *mut u32;
                (*prototype).prototype_code.set_size(n as usize * size_of::<u32>());
            } else {
                (*prototype)
                    .prototype_code
                    .initialize_size(self.loadstate_interpreter, (n as usize).wrapping_mul(size_of::<u32>()));
                self.load_block(
                    (*prototype).prototype_code.vectort_pointer as *mut std::ffi::c_void,
                    (n as usize).wrapping_mul(size_of::<u32>()),
                );
            }
        }
    }
    pub unsafe fn load_constants(&mut self, prototype: *mut Prototype) {
        unsafe {
            let n = self.load_int();
            if size_of::<i32>() >= size_of::<usize>() && (n as usize) + 1 > ((!0usize) / size_of::<TValue>()) {
                (*(self.loadstate_interpreter)).too_big();
            }
            (*prototype)
                .prototype_constants
                .initialize_size(self.loadstate_interpreter, n as usize);
            for i in 0..n {
                (*((*prototype).prototype_constants.vectort_pointer).add(i as usize)).tvalue_set_tag_variant(TagVariant::NilNil);
            }
            for i in 0..n {
                let tvalue: *mut TValue = &mut *((*prototype).prototype_constants.vectort_pointer).add(i as usize) as *mut TValue;
                let tagvariant = TagVariant::from(self.load_byte());
                match tagvariant {
                    | TagVariant::NilNil => {
                        (*tvalue).tvalue_set_tag_variant(TagVariant::NilNil);
                    },
                    | TagVariant::BooleanFalse => {
                        (*tvalue).tvalue_set_tag_variant(TagVariant::BooleanFalse);
                    },
                    | TagVariant::BooleanTrue => {
                        (*tvalue).tvalue_set_tag_variant(TagVariant::BooleanTrue);
                    },
                    | TagVariant::NumericNumber => {
                        (*tvalue).set_number(self.load_number());
                    },
                    | TagVariant::NumericInteger => {
                        (*tvalue).set_integer(self.load_integer());
                    },
                    | TagVariant::StringLong | TagVariant::StringShort => {
                        let tstring: *mut TString = self.load_string(prototype);
                        (*tvalue).set_object(tstring as *mut Object, (*tstring).get_tagvariant());
                    },
                    | _ => {},
                }
            }
        }
    }
    pub unsafe fn load_prototypes(&mut self, prototype: *mut Prototype) {
        unsafe {
            let n = self.load_int();
            if size_of::<i32>() >= size_of::<usize>() && (n as usize) + 1 > ((!0usize) / size_of::<*mut Prototype>()) {
                (*(self.loadstate_interpreter)).too_big();
            }
            (*prototype)
                .prototype_prototypes
                .initialize_size(self.loadstate_interpreter, n as usize);
            for i in 0..n {
                *((*prototype).prototype_prototypes.vectort_pointer).add(i as usize) = null_mut();
            }
            for i in 0..n {
                *((*prototype).prototype_prototypes.vectort_pointer).add(i as usize) = luaf_newproto(self.loadstate_interpreter);
                if (*prototype).get_marked() & BLACKBIT != 0
                    && (**((*prototype).prototype_prototypes.vectort_pointer).add(i as usize)).get_marked() & WHITEBITS != 0
                {
                    Object::luac_barrier_(
                        self.loadstate_interpreter,
                        prototype as *mut Object,
                        *((*prototype).prototype_prototypes.vectort_pointer).add(i as usize) as *mut Object,
                    );
                }
                self.load_function(
                    *((*prototype).prototype_prototypes.vectort_pointer).add(i as usize),
                    (*prototype).prototype_source,
                );
            }
        }
    }
    pub unsafe fn load_upvalues(&mut self, prototype: *mut Prototype) {
        unsafe {
            let n = self.load_int();
            if size_of::<i32>() >= size_of::<usize>() && (n as usize) + 1 > ((!0usize) / size_of::<UpValueDescription>()) {
                (*(self.loadstate_interpreter)).too_big();
            }
            (*prototype)
                .prototype_upvalues
                .initialize_size(self.loadstate_interpreter, n as usize);
            for i in 0..n {
                let name_slot = &mut (*((*prototype).prototype_upvalues.vectort_pointer).add(i as usize)).upvaluedescription_name;
                *name_slot = null_mut();
            }
            for i in 0..n {
                let upvalue_description = &mut *((*prototype).prototype_upvalues.at_mut(i as isize));
                upvalue_description.load(self);
            }
        }
    }
    pub unsafe fn load_debug(&mut self, prototype: *mut Prototype) {
        unsafe {
            let mut n = self.load_int();
            if size_of::<i32>() >= size_of::<usize>() && (n as usize) + 1 > (!0usize) {
                (*(self.loadstate_interpreter)).too_big();
            }
            if self.loadstate_fixed {
                let addr = (*self.loadstate_zio).luaz_getaddr(n as usize);
                if addr.is_null() && n > 0 {
                    self.error(c"truncated fixed chunk".as_ptr());
                }
                (*prototype).prototype_lineinfo.vectort_pointer = addr as *mut i8;
                (*prototype).prototype_lineinfo.set_size(n as usize);
            } else {
                (*prototype)
                    .prototype_lineinfo
                    .initialize_size(self.loadstate_interpreter, n as usize);
                self.load_block(
                    (*prototype).prototype_lineinfo.vectort_pointer as *mut std::ffi::c_void,
                    n as usize,
                );
            }
            n = self.load_int();
            if size_of::<i32>() >= size_of::<usize>() && (n as usize) + 1 > ((!0usize) / size_of::<AbsoluteLineInfo>()) {
                (*(self.loadstate_interpreter)).too_big();
            }
            if self.loadstate_fixed {
                let addr = (*self.loadstate_zio).luaz_getaddr((n as usize).wrapping_mul(size_of::<AbsoluteLineInfo>()));
                if addr.is_null() && n > 0 {
                    self.error(c"truncated fixed chunk".as_ptr());
                }
                (*prototype).prototype_absolutelineinfo.vectort_pointer = addr as *mut AbsoluteLineInfo;
                (*prototype).prototype_absolutelineinfo.set_size(n as usize);
            } else {
                (*prototype)
                    .prototype_absolutelineinfo
                    .initialize_size(self.loadstate_interpreter, n as usize);
                for i in 0..n {
                    (*((*prototype).prototype_absolutelineinfo.vectort_pointer).add(i as usize)).absolutelineinfo_program_counter =
                        self.load_int();
                    (*((*prototype).prototype_absolutelineinfo.vectort_pointer).add(i as usize)).absolutelineinfo_line =
                        self.load_int();
                }
            }
            n = self.load_int();
            if size_of::<i32>() >= size_of::<usize>() && (n as usize) + 1 > ((!0usize) / size_of::<LocalVariable>()) {
                (*(self.loadstate_interpreter)).too_big();
            }
            (*prototype)
                .prototype_localvariables
                .initialize_size(self.loadstate_interpreter, n as usize);
            for i in 0..n {
                let name_slot = &mut (*((*prototype).prototype_localvariables.at_mut(i as isize))).localvariable_variablename;
                *name_slot = null_mut();
            }
            for i in 0..n {
                let name_slot = &mut (*((*prototype).prototype_localvariables.at_mut(i as isize))).localvariable_variablename;
                *name_slot = self.load_string_n(prototype);
                (*((*prototype).prototype_localvariables.at_mut(i as isize))).localvariable_startprogramcounter = self.load_int();
                (*((*prototype).prototype_localvariables.at_mut(i as isize))).localvariable_endprogramcounter = self.load_int();
            }
            n = self.load_int();
            if n != 0 {
                n = (*prototype).prototype_upvalues.get_size() as i32;
            }
            for i in 0..n {
                let name_slot = &mut (*((*prototype).prototype_upvalues.vectort_pointer).add(i as usize)).upvaluedescription_name;
                *name_slot = self.load_string_n(prototype);
            }
        }
    }
    pub unsafe fn load_function(&mut self, prototype: *mut Prototype, psource: *mut TString) {
        unsafe {
            (*prototype).prototype_source = self.load_string_n(prototype);
            if ((*prototype).prototype_source).is_null() {
                (*prototype).prototype_source = psource;
            }
            (*prototype).prototype_linedefined = self.load_int();
            (*prototype).prototype_lastlinedefined = self.load_int();
            (*prototype).prototype_countparameters = self.load_byte() as usize;
            let flag_byte = self.load_byte();
            (*prototype).prototype_flag = flag_byte & !PF_FIXED;
            if self.loadstate_fixed {
                (*prototype).prototype_flag |= PF_FIXED;
            }
            (*prototype).prototype_isvariablearguments = flag_byte & 0x03 != 0;
            (*prototype).prototype_needsvarargtable = flag_byte & 0x02 != 0;
            (*prototype).prototype_maximumstacksize = self.load_byte();
            self.load_code(prototype);
            self.load_constants(prototype);
            self.load_upvalues(prototype);
            self.load_prototypes(prototype);
            self.load_debug(prototype);
        }
    }
    pub unsafe fn check_literal(&mut self, s: *const i8, message: *const i8) {
        unsafe {
            let mut buffer: [i8; 12] = [0; 12];
            let length: usize = cstr_len(s);
            self.load_block(buffer.as_mut_ptr() as *mut std::ffi::c_void, length);
            if std::slice::from_raw_parts(s as *const u8, length)
                != std::slice::from_raw_parts(buffer.as_ptr() as *const u8, length)
            {
                self.error(message);
            }
        }
    }
    pub unsafe fn check_header(&mut self) {
        unsafe {
            self.check_literal(&*(DumpState::LUA_SIGNATURE).add(1), c"not a binary chunk".as_ptr());
            if self.load_byte() != DumpState::LUAC_VERSION {
                self.error(c"version mismatch".as_ptr());
            }
            if self.load_byte() != DumpState::LUAC_FORMAT {
                self.error(c"format mismatch".as_ptr());
            }
            self.check_literal(DumpState::LUAC_DATA, c"corrupted chunk".as_ptr());
            if self.load_byte() as usize != size_of::<i32>() {
                self.error(c"int size mismatch".as_ptr());
            }
            let mut int_val: i32 = 0;
            self.load_block(&mut int_val as *mut i32 as *mut std::ffi::c_void, size_of::<i32>());
            if int_val != DumpState::LUAC_INT {
                self.error(c"int format mismatch".as_ptr());
            }
            if self.load_byte() as usize != size_of::<u32>() {
                self.error(c"instruction size mismatch".as_ptr());
            }
            let mut inst_val: u32 = 0;
            self.load_block(&mut inst_val as *mut u32 as *mut std::ffi::c_void, size_of::<u32>());
            if inst_val != DumpState::LUAC_INST {
                self.error(c"instruction format mismatch".as_ptr());
            }
            if self.load_byte() as usize != size_of::<i64>() {
                self.error(c"Lua integer size mismatch".as_ptr());
            }
            let mut integer_val: i64 = 0;
            self.load_block(&mut integer_val as *mut i64 as *mut std::ffi::c_void, size_of::<i64>());
            if integer_val != DumpState::LUAC_INTEGER {
                self.error(c"Lua integer format mismatch".as_ptr());
            }
            if self.load_byte() as usize != size_of::<f64>() {
                self.error(c"Lua number size mismatch".as_ptr());
            }
            let mut number_val: f64 = 0.0;
            self.load_block(&mut number_val as *mut f64 as *mut std::ffi::c_void, size_of::<f64>());
            if number_val != DumpState::LUAC_NUM {
                self.error(c"Lua number format mismatch".as_ptr());
            }
        }
    }
}
pub unsafe fn load_closure(state: *mut State, zio: *mut ZIO, name: *const i8) -> *mut Closure {
    unsafe { load_closure_fixed(state, zio, name, false) }
}
pub unsafe fn load_closure_fixed(state: *mut State, zio: *mut ZIO, name: *const i8, fixed: bool) -> *mut Closure {
    unsafe {
        let mut load_state = LoadState {
            loadstate_interpreter: null_mut(),
            loadstate_zio: null_mut(),
            loadstate_name: null(),
            loadstate_table: null_mut(),
            loadstate_nstr: 0,
            loadstate_fixed: fixed,
        };
        if *name as i32 == Character::At as i32 || *name as i32 == Character::Equal as i32 {
            load_state.loadstate_name = name.add(1);
        } else if *name as i32 == 0x1B {
            load_state.loadstate_name = c"binary string".as_ptr();
        } else {
            load_state.loadstate_name = name;
        }
        load_state.loadstate_interpreter = state;
        load_state.loadstate_zio = zio;
        load_state.check_header();
        let ret: *mut Closure = Closure::luaf_newlclosure(state, load_state.load_byte() as i32);
        let io: *mut TValue = (*state).interpreter_top.stkidrel_pointer;
        (*io).set_object(ret as *mut Object, TagVariant::ClosureL);
        (*state).luad_inctop();
        load_state.loadstate_table = luah_new(state);
        load_state.loadstate_nstr = 0;
        let io2: *mut TValue = (*state).interpreter_top.stkidrel_pointer;
        (*io2).set_table(load_state.loadstate_table);
        (*state).luad_inctop();
        (*ret).closure_payload.closurepayload_lprototype = luaf_newproto(state);
        if (*ret).get_marked() & BLACKBIT != 0 && (*(*ret).closure_payload.closurepayload_lprototype).get_marked() & WHITEBITS != 0
        {
            Object::luac_barrier_(
                state,
                &mut *(ret as *mut Object),
                &mut *((*ret).closure_payload.closurepayload_lprototype as *mut Object),
            );
        }
        load_state.load_function((*ret).closure_payload.closurepayload_lprototype, null_mut());
        (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.sub(1);
        ret
    }
}
