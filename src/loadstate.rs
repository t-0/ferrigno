use crate::absolutelineinfo::*;
use crate::character::*;
use crate::closure::*;
use crate::dumpstate::*;
use crate::interpreter::*;
use crate::tloadable::*;
use crate::localvariable::*;
use crate::object::*;
use crate::prototype::*;
use crate::status::*;
use crate::tagvariant::*;
use crate::tobject::*;
use crate::tstring::*;
use crate::tvalue::*;
use crate::upvaluedescription::*;
use crate::zio::*;
use core::mem::*;
use std::ptr::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LoadState {
    loadstate_interpreter: *mut Interpreter,
    loadstate_zio: *mut ZIO,
    loadstate_name: *const i8,
}
impl LoadState {
    pub unsafe fn error(&mut self, why: *const i8) -> ! {
        unsafe {
            luao_pushfstring(
                self.loadstate_interpreter,
                c"%s: bad binary format (%s)".as_ptr(),
                self.loadstate_name,
                why,
            );
            luad_throw(self.loadstate_interpreter, Status::SyntaxError);
        }
    }
    pub unsafe fn load_block(&mut self, b: *mut libc::c_void, size: usize) {
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
                | Some(x) => {
                    return x;
                },
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
                ret = ret << 7 | (b & 0x7f as i32) as usize;
                if !(b & 0x80 as i32 == 0) {
                    break;
                }
            }
            return ret;
        }
    }
    pub unsafe fn load_size(&mut self) -> usize {
        unsafe {
            return self.load_unsigned(!0usize);
        }
    }
    pub unsafe fn load_int(&mut self) -> i32 {
        unsafe {
            return self.load_unsigned(0x7FFFFFFF as usize) as i32;
        }
    }
    pub unsafe fn load_number(&mut self) -> f64 {
        unsafe {
            let mut x: f64 = 0.0;
            self.load_block(&mut x as *mut f64 as *mut libc::c_void, size_of::<f64>());
            return x;
        }
    }
    pub unsafe fn load_integer(&mut self) -> i64 {
        unsafe {
            let mut x: i64 = 0;
            self.load_block(&mut x as *mut i64 as *mut libc::c_void, size_of::<i64>());
            return x;
        }
    }
    pub unsafe fn load_string_n(&mut self, p: *mut Prototype) -> *mut TString {
        unsafe {
            let interpreter: *mut Interpreter = self.loadstate_interpreter;
            let tstring: *mut TString;
            let mut size: usize = self.load_size();
            if size == 0 {
                return null_mut();
            } else {
                size -= 1;
                if size <= 40 as usize {
                    let mut buffer: [i8; 40] = [0; 40];
                    self.load_block(buffer.as_mut_ptr() as *mut libc::c_void, size.wrapping_mul(1 as usize));
                    tstring = luas_newlstr(interpreter, buffer.as_mut_ptr(), size as usize);
                } else {
                    tstring = TString::create_long(interpreter, size as usize);
                    let io: *mut TValue = &mut (*(*interpreter).interpreter_top.stkidrel_pointer);
                    (*io).tvalue_value.value_object = &mut (*(tstring as *mut Object));
                    (*io).tvalue_set_tag_variant((*tstring).get_tagvariant());
                    (*io).set_collectable(true);
                    (*interpreter).luad_inctop();
                    self.load_block(
                        ((*tstring).get_contents_mut()) as *mut libc::c_void,
                        size.wrapping_mul(1 as usize),
                    );
                    (*interpreter).interpreter_top.stkidrel_pointer = (*interpreter).interpreter_top.stkidrel_pointer.offset(-1);
                }
            }
            if (*p).get_marked() & 1 << 5 != 0 && (*tstring).get_marked() & (1 << 3 | 1 << 4) != 0 {
                Object::luac_barrier_(interpreter, &mut (*(p as *mut Object)), &mut (*(tstring as *mut Object)));
            } else {
            };
            return tstring;
        }
    }
    pub unsafe fn load_string(&mut self, p: *mut Prototype) -> *mut TString {
        unsafe {
            let tstring = self.load_string_n(p);
            if tstring.is_null() {
                self.error(c"bad format for code_constant string".as_ptr());
            }
            return tstring;
        }
    }
    pub unsafe fn load_code(&mut self, prototype: *mut Prototype) {
        unsafe {
            let n = self.load_int();
            if size_of::<i32>() as usize >= size_of::<usize>() as usize && (n as usize) + 1 > ((!0usize) / size_of::<u32>()) {
                (*(self.loadstate_interpreter)).too_big();
            }
            (*prototype)
                .prototype_code
                .initialize_size(self.loadstate_interpreter, (n as usize).wrapping_mul(size_of::<u32>()));
            self.load_block(
                (*prototype).prototype_code.vectort_pointer as *mut libc::c_void,
                (n as usize).wrapping_mul(size_of::<u32>()) as usize,
            );
        }
    }
    pub unsafe fn load_constants(&mut self, prototype: *mut Prototype) {
        unsafe {
            let n = self.load_int();
            if size_of::<i32>() as usize >= size_of::<usize>() as usize && (n as usize) + 1 > ((!0usize) / size_of::<TValue>()) {
                (*(self.loadstate_interpreter)).too_big();
            }
            (*prototype)
                .prototype_constants
                .initialize_size(self.loadstate_interpreter, n as usize);
            for i in 0..n {
                (*((*prototype).prototype_constants.vectort_pointer).offset(i as isize)).tvalue_set_tag_variant(TagVariant::NilNil);
            }
            for i in 0..n {
                let tvalue: *mut TValue =
                    &mut *((*prototype).prototype_constants.vectort_pointer).offset(i as isize) as *mut TValue;
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
                        let io: *mut TValue = tvalue;
                        (*io).tvalue_value.value_number = self.load_number();
                        (*io).tvalue_set_tag_variant(TagVariant::NumericNumber);
                    },
                    | TagVariant::NumericInteger => {
                        let io_0: *mut TValue = tvalue;
                        (*io_0).tvalue_value.value_integer = self.load_integer();
                        (*io_0).tvalue_set_tag_variant(TagVariant::NumericInteger);
                    },
                    | TagVariant::StringLong | TagVariant::StringShort => {
                        let io_1: *mut TValue = tvalue;
                        let tstring: *mut TString = self.load_string(prototype);
                        (*io_1).tvalue_value.value_object = &mut (*(tstring as *mut Object));
                        (*io_1).tvalue_set_tag_variant((*tstring).get_tagvariant());
                        (*io_1).set_collectable(true);
                    },
                    | _ => {},
                }
            }
        }
    }
    pub unsafe fn load_prototypes(&mut self, prototype: *mut Prototype) {
        unsafe {
            let n = self.load_int();
            if size_of::<i32>() as usize >= size_of::<usize>() as usize
                && (n as usize) + 1 > ((!0usize) / size_of::<*mut Prototype>())
            {
                (*(self.loadstate_interpreter)).too_big();
            }
            (*prototype)
                .prototype_prototypes
                .initialize_size(self.loadstate_interpreter, n as usize);
            for i in 0..n {
                *((*prototype).prototype_prototypes.vectort_pointer).offset(i as isize) = null_mut();
            }
            for i in 0..n {
                *((*prototype).prototype_prototypes.vectort_pointer).offset(i as isize) = luaf_newproto(self.loadstate_interpreter);
                if (*prototype).get_marked() & 1 << 5 != 0
                    && (**((*prototype).prototype_prototypes.vectort_pointer).offset(i as isize)).get_marked() & (1 << 3 | 1 << 4)
                        != 0
                {
                    Object::luac_barrier_(
                        self.loadstate_interpreter,
                        &mut (*(prototype as *mut Object)),
                        &mut (*(*((*prototype).prototype_prototypes.vectort_pointer).offset(i as isize) as *mut Object)),
                    );
                } else {
                }
                self.load_function(
                    *((*prototype).prototype_prototypes.vectort_pointer).offset(i as isize),
                    (*prototype).prototype_source,
                );
            }
        }
    }
    pub unsafe fn load_upvalues(&mut self, prototype: *mut Prototype) {
        unsafe {
            let n = self.load_int();
            if size_of::<i32>() as usize >= size_of::<usize>() as usize
                && (n as usize) + 1 > ((!0usize) / size_of::<UpValueDescription>())
            {
                (*(self.loadstate_interpreter)).too_big();
            }
            (*prototype)
                .prototype_upvalues
                .initialize_size(self.loadstate_interpreter, n as usize);
            for i in 0..n {
                let ref mut fresh29 =
                    (*((*prototype).prototype_upvalues.vectort_pointer).offset(i as isize)).upvaluedescription_name;
                *fresh29 = null_mut();
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
            if size_of::<i32>() as usize >= size_of::<usize>() as usize && (n as usize) + 1 > (!0usize) {
                (*(self.loadstate_interpreter)).too_big();
            }
            (*prototype)
                .prototype_lineinfo
                .initialize_size(self.loadstate_interpreter, n as usize);
            self.load_block(
                (*prototype).prototype_lineinfo.vectort_pointer as *mut libc::c_void,
                (n as usize).wrapping_mul(1 as usize),
            );
            n = self.load_int();
            if size_of::<i32>() as usize >= size_of::<usize>() as usize
                && (n as usize) + 1 > ((!0usize) / size_of::<AbsoluteLineInfo>())
            {
                (*(self.loadstate_interpreter)).too_big();
            } else {
            };
            (*prototype)
                .prototype_absolutelineinfo
                .initialize_size(self.loadstate_interpreter, n as usize);
            for i in 0..n {
                (*((*prototype).prototype_absolutelineinfo.vectort_pointer).offset(i as isize)).absolutelineinfo_programcounter =
                    self.load_int();
                (*((*prototype).prototype_absolutelineinfo.vectort_pointer).offset(i as isize)).absolutelineinfo_line =
                    self.load_int();
            }
            n = self.load_int();
            if size_of::<i32>() as usize >= size_of::<usize>() as usize
                && (n as usize) + 1 > ((!0usize) / size_of::<LocalVariable>())
            {
                (*(self.loadstate_interpreter)).too_big();
            } else {
            };
            (*prototype)
                .prototype_localvariables
                .initialize_size(self.loadstate_interpreter, n as usize);
            for i in 0..n {
                let ref mut fresh30 = (*((*prototype).prototype_localvariables.at_mut(i as isize))).localvariable_variablename;
                *fresh30 = null_mut();
            }
            for i in 0..n {
                let ref mut fresh31 = (*((*prototype).prototype_localvariables.at_mut(i as isize))).localvariable_variablename;
                *fresh31 = self.load_string_n(prototype);
                (*((*prototype).prototype_localvariables.at_mut(i as isize))).localvariable_startprogramcounter = self.load_int();
                (*((*prototype).prototype_localvariables.at_mut(i as isize))).localvariable_endprogramcounter = self.load_int();
            }
            n = self.load_int();
            if n != 0 {
                n = (*prototype).prototype_upvalues.get_size() as i32;
            }
            for i in 0..n {
                let ref mut fresh32 =
                    (*((*prototype).prototype_upvalues.vectort_pointer).offset(i as isize)).upvaluedescription_name;
                *fresh32 = self.load_string_n(prototype);
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
            (*prototype).prototype_isvariablearguments = 0 != self.load_byte();
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
            let length: usize = libc::strlen(s) as usize;
            self.load_block(buffer.as_mut_ptr() as *mut libc::c_void, length.wrapping_mul(1 as usize));
            if libc::memcmp(
                s as *const libc::c_void,
                buffer.as_mut_ptr() as *const libc::c_void,
                length as usize,
            ) != 0
            {
                self.error(message);
            }
        }
    }
    pub unsafe fn check_header(&mut self) {
        unsafe {
            self.check_literal(&*(LUA_SIGNATURE).offset(1 as isize), c"not a binary chunk".as_ptr());
            if self.load_byte() as i32 != 504 as i32 / 100 as i32 * 16 as i32 + 504 as i32 % 100 as i32 {
                self.error(c"version mismatch".as_ptr());
            }
            if self.load_byte() as i32 != 0 {
                self.error(c"format mismatch".as_ptr());
            }
            self.check_literal(c"\x19\x7F\r\n\x1A\n".as_ptr(), c"corrupted chunk".as_ptr());
            if self.load_integer() != 0x5678 as i64 {
                self.error(c"integer format mismatch".as_ptr());
            }
            if self.load_number() != 370.5f64 {
                self.error(c"float format mismatch".as_ptr());
            }
        }
    }
}
pub unsafe fn load_closure(interpreter: *mut Interpreter, zio: *mut ZIO, name: *const i8) -> *mut Closure {
    unsafe {
        let mut load_state = LoadState {
            loadstate_interpreter: null_mut(),
            loadstate_zio: null_mut(),
            loadstate_name: null(),
        };
        if *name as i32 == Character::At as i32 || *name as i32 == Character::Equal as i32 {
            load_state.loadstate_name = name.offset(1 as isize);
        } else if *name as i32 == (*::core::mem::transmute::<&[u8; 5], &[i8; 5]>(b"\x1BLua\0"))[0] as i32 {
            load_state.loadstate_name = c"binary string".as_ptr();
        } else {
            load_state.loadstate_name = name;
        }
        load_state.loadstate_interpreter = interpreter;
        load_state.loadstate_zio = zio;
        load_state.check_header();
        let ret: *mut Closure = Closure::luaf_newlclosure(interpreter, load_state.load_byte() as i32);
        let io: *mut TValue = &mut (*(*interpreter).interpreter_top.stkidrel_pointer);
        (*io).tvalue_value.value_object = &mut (*(ret as *mut Object));
        (*io).tvalue_set_tag_variant(TagVariant::ClosureL);
        (*io).set_collectable(true);
        (*interpreter).luad_inctop();
        (*ret).payload.l_prototype = luaf_newproto(interpreter);
        if (*ret).get_marked() & 1 << 5 != 0 && (*(*ret).payload.l_prototype).get_marked() & (1 << 3 | 1 << 4) != 0 {
            Object::luac_barrier_(
                interpreter,
                &mut (*(ret as *mut Object)),
                &mut (*((*ret).payload.l_prototype as *mut Object)),
            );
        } else {
        };
        load_state.load_function((*ret).payload.l_prototype, null_mut());
        return ret;
    }
}
