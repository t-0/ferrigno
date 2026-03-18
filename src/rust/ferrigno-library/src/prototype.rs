use crate::absolutelineinfo::*;
use crate::character::*;
use crate::dumpstate::*;
use crate::functionstate::*;
use crate::global::*;
use crate::instruction::*;
use crate::localvariable::*;
use crate::object::*;
use crate::objectwithgclist::*;
use crate::opcode::*;
use crate::opmode::*;
use crate::state::*;
use crate::strings::*;
use crate::tagtype::*;
use crate::tagvariant::*;
use crate::tloadable::*;
use crate::tm::*;
use crate::tobject::*;
use crate::tobjectwithgclist::TObjectWithGCList;
use crate::tstring::*;
use crate::tvalue::*;
use crate::upvaluedescription::*;
use crate::vectort::*;
use std::ptr::*;
type PrototypeSuper = ObjectWithGCList;
pub const PF_FIXED: u8 = 4;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Prototype {
    pub prototype_super: PrototypeSuper,
    pub prototype_flag: u8,
    pub prototype_isvariablearguments: bool,
    pub prototype_needsvarargtable: bool,
    pub prototype_countparameters: usize,
    pub prototype_maximumstacksize: u8,
    pub prototype_source: *mut TString,
    pub prototype_constants: VectorT<TValue>,
    pub prototype_code: VectorT<u32>,
    pub prototype_prototypes: VectorT<*mut Prototype>,
    pub prototype_upvalues: VectorT<UpValueDescription>,
    pub prototype_localvariables: VectorT<LocalVariable>,
    pub prototype_linedefined: i32,
    pub prototype_lastlinedefined: i32,
    pub prototype_lineinfo: VectorT<i8>,
    pub prototype_absolutelineinfo: VectorT<AbsoluteLineInfo>,
}
impl TObject for Prototype {
    fn as_object(&self) -> &Object {
        self.prototype_super.as_object()
    }
    fn as_object_mut(&mut self) -> &mut Object {
        self.prototype_super.as_object_mut()
    }
}
impl TObjectWithGCList for Prototype {
    fn getgclist(&mut self) -> *mut *mut PrototypeSuper {
        self.prototype_super.getgclist()
    }
}
impl Prototype {
    pub unsafe fn dump_code(&self, dump_state: &mut DumpState) {
        unsafe {
            dump_state.dump_int(self.prototype_code.get_size() as i32);
            dump_state.dump_block(
                self.prototype_code.vectort_pointer as *const std::ffi::c_void,
                self.prototype_code.get_size().wrapping_mul(size_of::<u32>()),
            );
        }
    }
    pub unsafe fn dump_function(&self, dump_state: &mut DumpState, source: *mut TString) {
        unsafe {
            if dump_state.is_strip() || self.prototype_source == source {
                TString::dump_string(dump_state, null());
            } else {
                TString::dump_string(dump_state, self.prototype_source);
            }
            dump_state.dump_int(self.prototype_linedefined);
            dump_state.dump_int(self.prototype_lastlinedefined);
            dump_state.dump_byte(self.prototype_countparameters as u8);
            let mut flag: u8 = 0;
            if self.prototype_isvariablearguments {
                flag |= 1;
            }
            if self.prototype_needsvarargtable {
                flag |= 2;
            }
            dump_state.dump_byte(flag);
            dump_state.dump_byte(self.prototype_maximumstacksize);
            self.dump_code(dump_state);
            self.dump_constants(dump_state);
            self.dump_upvalues(dump_state);
            self.dump_prototypes(dump_state);
            self.dump_debug(dump_state);
        }
    }
    pub unsafe fn dump_debug(&self, dump_state: &mut DumpState) {
        unsafe {
            let n = if dump_state.is_strip() { 0 } else { self.prototype_lineinfo.get_size() };
            dump_state.dump_int(n as i32);
            dump_state.dump_block(self.prototype_lineinfo.vectort_pointer as *const std::ffi::c_void, n);
        }
        unsafe {
            let n = if dump_state.is_strip() {
                0
            } else {
                self.prototype_absolutelineinfo.get_size()
            };
            dump_state.dump_int(n as i32);
            for i in 0..n {
                dump_state.dump_int((*(self.prototype_absolutelineinfo.vectort_pointer).add(i)).absolutelineinfo_program_counter);
                dump_state.dump_int((*(self.prototype_absolutelineinfo.vectort_pointer).add(i)).absolutelineinfo_line);
            }
        }
        unsafe {
            let n = if dump_state.is_strip() { 0 } else { self.prototype_localvariables.get_size() };
            dump_state.dump_int(n as i32);
            for i in 0..n {
                TString::dump_string(
                    dump_state,
                    (*(self.prototype_localvariables.at(i as isize))).localvariable_variablename,
                );
                dump_state.dump_int((*(self.prototype_localvariables.at(i as isize))).localvariable_startprogramcounter);
                dump_state.dump_int((*(self.prototype_localvariables.at(i as isize))).localvariable_endprogramcounter);
            }
        }
        unsafe {
            let n = if dump_state.is_strip() { 0 } else { self.prototype_upvalues.get_size() };
            dump_state.dump_int(n as i32);
            for i in 0..n {
                TString::dump_string(dump_state, (*(self.prototype_upvalues.at(i as isize))).upvaluedescription_name);
            }
        }
    }
    pub unsafe fn dump_prototypes(&self, dump_state: &mut DumpState) {
        unsafe {
            let n = self.prototype_prototypes.get_size();
            dump_state.dump_int(n as i32);
            for i in 0..n {
                (*(*(self.prototype_prototypes.at(i as isize)))).dump_function(dump_state, self.prototype_source);
            }
        }
    }
    pub unsafe fn dump_upvalues(&self, dump_state: &mut DumpState) {
        unsafe {
            let n = self.prototype_upvalues.get_size();
            dump_state.dump_int(n as i32);
            for i in 0..n {
                let upvalue_description = *(self.prototype_upvalues.at(i as isize));
                upvalue_description.dump(dump_state);
            }
        }
    }
    pub unsafe fn dump_constants(&self, dump_state: &mut DumpState) {
        unsafe {
            let n = self.prototype_constants.get_size();
            dump_state.dump_int(n as i32);
            for i in 0..n {
                let tvalue: *const TValue = &mut *(self.prototype_constants.vectort_pointer).add(i) as *mut TValue;
                let tagvariant = (*tvalue).get_tagvariant();
                dump_state.dump_byte(tagvariant as u8);
                match tagvariant {
                    | TagVariant::NumericNumber => {
                        dump_state.dump_number((*tvalue).as_number().unwrap());
                    },
                    | TagVariant::NumericInteger => {
                        dump_state.dump_integer((*tvalue).as_integer().unwrap());
                    },
                    | TagVariant::StringShort | TagVariant::StringLong => {
                        TString::dump_string(dump_state, &*(*tvalue).as_string().unwrap());
                    },
                    | _ => {},
                }
            }
        }
    }
    pub unsafe fn prototype_traverse(&mut self, global: &mut Global) -> usize {
        unsafe {
            if !self.prototype_source.is_null() && (*self.prototype_source).get_marked() & WHITEBITS != 0 {
                Object::really_mark_object(global, &mut *(self.prototype_source as *mut Object));
            }
            for i in 0..self.prototype_constants.get_size() {
                let kv = &*(self.prototype_constants.vectort_pointer).add(i);
                if let Some(obj) = kv.as_object()
                    && (*obj).get_marked() & WHITEBITS != 0
                {
                    Object::really_mark_object(global, obj);
                }
            }
            for i in 0..self.prototype_upvalues.get_size() {
                if !((*(self.prototype_upvalues.vectort_pointer).add(i)).upvaluedescription_name).is_null()
                    && (*(*(self.prototype_upvalues.vectort_pointer).add(i)).upvaluedescription_name).get_marked() & WHITEBITS != 0
                {
                    Object::really_mark_object(
                        global,
                        &mut *((*(self.prototype_upvalues.vectort_pointer).add(i)).upvaluedescription_name as *mut Object),
                    );
                }
            }
            for i in 0..self.prototype_prototypes.get_size() {
                if !(*(self.prototype_prototypes.vectort_pointer).add(i)).is_null()
                    && (**(self.prototype_prototypes.vectort_pointer).add(i)).get_marked() & WHITEBITS != 0
                {
                    Object::really_mark_object(
                        global,
                        &mut *(*(self.prototype_prototypes.vectort_pointer).add(i) as *mut Object),
                    );
                }
            }
            for i in 0..self.prototype_localvariables.get_size() {
                if !((*(self.prototype_localvariables.vectort_pointer).add(i)).localvariable_variablename).is_null()
                    && (*(*(self.prototype_localvariables.vectort_pointer).add(i)).localvariable_variablename).get_marked()
                        & WHITEBITS
                        != 0
                {
                    Object::really_mark_object(
                        global,
                        &mut *((*(self.prototype_localvariables.vectort_pointer).add(i)).localvariable_variablename as *mut Object),
                    );
                }
            }
            1 + self.prototype_constants.get_size()
                + self.prototype_upvalues.get_size()
                + self.prototype_prototypes.get_size()
                + self.prototype_localvariables.get_size()
        }
    }
    pub unsafe fn prototype_free(&mut self, state: *mut State) {
        unsafe {
            let fixed = self.prototype_flag & PF_FIXED != 0;
            if !fixed {
                (*state).free_memory(
                    self.prototype_code.vectort_pointer as *mut std::ffi::c_void,
                    self.prototype_code.get_size().wrapping_mul(size_of::<u32>()),
                );
            }
            (*state).free_memory(
                self.prototype_prototypes.vectort_pointer as *mut std::ffi::c_void,
                self.prototype_prototypes.get_size().wrapping_mul(size_of::<*mut Prototype>()),
            );
            (*state).free_memory(
                self.prototype_constants.vectort_pointer as *mut std::ffi::c_void,
                self.prototype_constants.get_size().wrapping_mul(size_of::<TValue>()),
            );
            if !fixed {
                (*state).free_memory(
                    self.prototype_lineinfo.vectort_pointer as *mut std::ffi::c_void,
                    self.prototype_lineinfo.get_size(),
                );
                (*state).free_memory(
                    self.prototype_absolutelineinfo.vectort_pointer as *mut std::ffi::c_void,
                    self.prototype_absolutelineinfo
                        .get_size()
                        .wrapping_mul(size_of::<AbsoluteLineInfo>()),
                );
            }
            (*state).free_memory(
                self.prototype_localvariables.vectort_pointer as *mut std::ffi::c_void,
                self.prototype_localvariables
                    .get_size()
                    .wrapping_mul(size_of::<LocalVariable>()),
            );
            (*state).free_memory(
                self.prototype_upvalues.vectort_pointer as *mut std::ffi::c_void,
                self.prototype_upvalues.get_size().wrapping_mul(size_of::<UpValueDescription>()),
            );
            (*state).free_memory(self as *mut Prototype as *mut std::ffi::c_void, size_of::<Prototype>());
        }
    }
}
pub unsafe fn getbaseline(prototype: *const Prototype, program_counter: i32, basepc: *mut i32) -> i32 {
    unsafe {
        if (*prototype).prototype_absolutelineinfo.get_size() == 0
            || program_counter
                < (*((*prototype).prototype_absolutelineinfo.vectort_pointer).add(0)).absolutelineinfo_program_counter
        {
            *basepc = -1;
            (*prototype).prototype_linedefined
        } else {
            let mut i = program_counter / MAXIWTHABS - 1;
            while (i + 1) < (*prototype).prototype_absolutelineinfo.get_size() as i32
                && program_counter
                    >= (*((*prototype).prototype_absolutelineinfo.vectort_pointer).add((i + 1) as usize))
                        .absolutelineinfo_program_counter
            {
                i += 1;
            }
            *basepc = (*((*prototype).prototype_absolutelineinfo.vectort_pointer).add(i as usize)).absolutelineinfo_program_counter;
            (*((*prototype).prototype_absolutelineinfo.vectort_pointer).add(i as usize)).absolutelineinfo_line
        }
    }
}
pub unsafe fn luag_getfuncline(prototype: *const Prototype, program_counter: i32) -> i32 {
    unsafe {
        if ((*prototype).prototype_lineinfo.vectort_pointer).is_null() {
            -1
        } else {
            let mut basepc: i32 = 0;
            let mut baseline: i32 = getbaseline(prototype, program_counter, &mut basepc);
            loop {
                let prev_basepc = basepc;
                basepc += 1;
                if prev_basepc >= program_counter {
                    break;
                }
                baseline += *((*prototype).prototype_lineinfo.vectort_pointer).add(basepc as usize) as i32;
            }
            baseline
        }
    }
}
pub unsafe fn upvalname(p: *const Prototype, uv: i32) -> *const i8 {
    unsafe {
        let s: *mut TString = (*((*p).prototype_upvalues.vectort_pointer).add(uv as usize)).upvaluedescription_name;
        if s.is_null() { c"?".as_ptr() } else { (*s).get_contents_mut() }
    }
}
pub unsafe fn nextline(p: *const Prototype, currentline: i32, program_counter: i32) -> i32 {
    unsafe {
        if *((*p).prototype_lineinfo.vectort_pointer).add(program_counter as usize) as i32 != -MAXLINEDIFF {
            currentline + *((*p).prototype_lineinfo.vectort_pointer).add(program_counter as usize) as i32
        } else {
            luag_getfuncline(p, program_counter)
        }
    }
}
pub unsafe fn findsetreg(p: *const Prototype, mut lastpc: i32, reg: i32) -> i32 {
    unsafe {
        let mut setreg: i32 = -1;
        let mut jmptarget: i32 = 0;
        if OPMODES[(*((*p).prototype_code.vectort_pointer).add(lastpc as usize) & MASK_OP) as usize] as i32 & OPMODE_MM != 0 {
            lastpc -= 1;
        }
        let mut program_counter: i32 = 0;
        while program_counter < lastpc {
            let i: u32 = *((*p).prototype_code.vectort_pointer).add(program_counter as usize);
            let op: u32 = i & MASK_OP;
            let a: i32 = (i >> POSITION_A & MASK_A) as i32;
            let change: i32;
            match op {
                | OPCODE_LOADNIL => {
                    let b: i32 = (i >> POSITION_B & MASK_A) as i32;
                    change = (a <= reg && reg <= a + b) as i32;
                },
                | OPCODE_TFORCALL => {
                    change = (reg >= a + 2) as i32;
                },
                | OPCODE_CALL | OPCODE_TAILCALL => {
                    change = (reg >= a) as i32;
                },
                | OPCODE_JMP => {
                    let offset: i32 = (i >> POSITION_A & MASK_AX) as i32 - (OFFSET_SJ);
                    let dest: i32 = program_counter + 1 + offset;
                    if dest <= lastpc && dest > jmptarget {
                        jmptarget = dest;
                    }
                    change = 0;
                },
                | _ => {
                    change = (OPMODES[op as usize] as i32 & OPMODE_A != 0 && reg == a) as i32;
                },
            }
            if change != 0 {
                setreg = filter_program_counter(program_counter, jmptarget);
            }
            program_counter += 1;
        }
        setreg
    }
}
pub unsafe fn kname(p: *const Prototype, index: i32, name: *mut *const i8) -> *const i8 {
    unsafe {
        let kvalue: *mut TValue = &mut *((*p).prototype_constants.vectort_pointer).add(index as usize) as *mut TValue;
        if (*kvalue).get_tagvariant().to_tag_type().is_string() {
            *name = (*(*kvalue).as_string().unwrap()).get_contents_mut();
            c"code_constant".as_ptr()
        } else {
            *name = c"?".as_ptr();
            null()
        }
    }
}
pub unsafe fn basicgetobjname(p: *const Prototype, ppc: *mut i32, reg: i32, name: *mut *const i8) -> *const i8 {
    unsafe {
        let mut program_counter: i32 = *ppc;
        *name = luaf_getlocalname(p, reg + 1, program_counter);
        if !(*name).is_null() {
            return Strings::STRING_LOCAL;
        }
        program_counter = findsetreg(p, program_counter, reg);
        *ppc = program_counter;
        if program_counter != -1 {
            let i: u32 = *((*p).prototype_code.vectort_pointer).add(program_counter as usize);
            let op: u32 = i & MASK_OP;
            match op {
                | OPCODE_MOVE => {
                    let b: i32 = (i >> POSITION_B & MASK_A) as i32;
                    if b < (i >> POSITION_A & MASK_A) as i32 {
                        return basicgetobjname(p, ppc, b, name);
                    }
                },
                | OPCODE_GET_UPVALUE => {
                    *name = upvalname(p, (i >> POSITION_B & MASK_A) as i32);
                    return TagType::STRING_UPVALUE;
                },
                | OPCODE_LOADK => {
                    return kname(p, (i >> POSITION_K & MASK_BX) as i32, name);
                },
                | OPCODE_LOADKX => {
                    return kname(
                        p,
                        (*((*p).prototype_code.vectort_pointer).add((program_counter + 1) as usize) >> POSITION_A & MASK_AX) as i32,
                        name,
                    );
                },
                | _ => {},
            }
        }
        null()
    }
}
pub unsafe fn rname(p: *const Prototype, mut program_counter: i32, c: i32, name: *mut *const i8) {
    unsafe {
        let what: *const i8 = basicgetobjname(p, &mut program_counter, c, name);
        if !(!what.is_null() && *what as i32 == Character::LowerC as i32) {
            *name = c"?".as_ptr();
        }
    }
}
pub unsafe fn rkname(p: *const Prototype, program_counter: i32, i: u32, name: *mut *const i8) {
    unsafe {
        let c: i32 = (i >> POSITION_C & MASK_A) as i32;
        if (i >> POSITION_K & MASK_K) as i32 != 0 {
            kname(p, c, name);
        } else {
            rname(p, program_counter, c, name);
        };
    }
}
pub unsafe fn is_environment(p: *const Prototype, mut program_counter: i32, i: u32, isup: i32) -> *const i8 {
    unsafe {
        let t: i32 = (i >> POSITION_B & MASK_A) as i32;
        let mut name: *const i8 = null();
        if isup != 0 {
            name = upvalname(p, t);
        } else {
            let what: *const i8 = basicgetobjname(p, &mut program_counter, t, &mut name);
            if what != Strings::STRING_LOCAL && what != TagType::STRING_UPVALUE {
                name = null();
            }
        }
        if !name.is_null() && std::ffi::CStr::from_ptr(name) == c"_ENV" {
            c"global".as_ptr()
        } else {
            c"field".as_ptr()
        }
    }
}
pub unsafe fn getobjname(p: *const Prototype, mut lastpc: i32, reg: i32, name: *mut *const i8) -> *const i8 {
    unsafe {
        let kind: *const i8 = basicgetobjname(p, &mut lastpc, reg, name);
        if !kind.is_null() {
            return kind;
        } else if lastpc != -1 {
            let i: u32 = *((*p).prototype_code.vectort_pointer).add(lastpc as usize);
            let op: u32 = i & MASK_OP;
            match op {
                | OPCODE_GET_TABLE_UPVALUE => {
                    let k: i32 = (i >> POSITION_C & MASK_A) as i32;
                    kname(p, k, name);
                    return is_environment(p, lastpc, i, 1);
                },
                | OPCODE_GET_TABLE => {
                    let key: i32 = (i >> POSITION_C & MASK_A) as i32;
                    rname(p, lastpc, key, name);
                    return is_environment(p, lastpc, i, 0);
                },
                | OPCODE_INDEX_INTEGER => {
                    *name = c"integer index".as_ptr();
                    return c"field".as_ptr();
                },
                | OPCODE_GET_FIELD => {
                    let field_key: i32 = (i >> POSITION_C & MASK_A) as i32;
                    kname(p, field_key, name);
                    return is_environment(p, lastpc, i, 0);
                },
                | OPCODE_SELF => {
                    let method_key: i32 = (i >> POSITION_C & MASK_A) as i32;
                    kname(p, method_key, name);
                    return c"method".as_ptr();
                },
                | _ => {},
            }
        }
        null()
    }
}
pub unsafe fn funcnamefromcode(state: *mut State, p: *const Prototype, program_counter: i32, name: *mut *const i8) -> *const i8 {
    unsafe {
        let tm: u32;
        let i: u32 = *((*p).prototype_code.vectort_pointer).add(program_counter as usize);
        match i & MASK_OP {
            | OPCODE_CALL | OPCODE_TAILCALL => {
                return getobjname(p, program_counter, (i >> POSITION_A & MASK_A) as i32, name);
            },
            | OPCODE_TFORCALL => {
                *name = c"for iterator".as_ptr();
                return c"for iterator".as_ptr();
            },
            | OPCODE_SELF | OPCODE_GET_TABLE_UPVALUE | OPCODE_GET_TABLE | OPCODE_INDEX_INTEGER | OPCODE_GET_FIELD => {
                tm = TM_INDEX;
            },
            | OPCODE_SETTABUP | OPCODE_SETTABLE | OPCODE_SETI | OPCODE_SETFIELD => {
                tm = TM_NEWINDEX;
            },
            | OPCODE_MMBIN | OPCODE_MMBINI | OPCODE_MMBINK => {
                tm = i >> POSITION_C & MASK_A;
            },
            | OPCODE_UNM => {
                tm = TM_UNM;
            },
            | OPCODE_BNOT => {
                tm = TM_BNOT;
            },
            | OPCODE_LEN => {
                tm = TM_LEN;
            },
            | OPCODE_CONCAT => {
                tm = TM_CONCAT;
            },
            | OPCODE_EQ => {
                tm = TM_EQ;
            },
            | OPCODE_LT | OPCODE_LTI | OPCODE_GTI => {
                tm = TM_LT;
            },
            | OPCODE_LE | OPCODE_LEI | OPCODE_GEI => {
                tm = TM_LE;
            },
            | OPCODE_CLOSE | OPCODE_RETURN => {
                tm = TM_CLOSE;
            },
            | _ => return null(),
        }
        *name = ((*(*(*state).interpreter_global).global_tmname[tm as usize]).get_contents()).add(2);
        c"metamethod".as_ptr()
    }
}
pub unsafe fn changedline(p: *const Prototype, old_program_counter: i32, newpc: i32) -> i32 {
    unsafe {
        if ((*p).prototype_lineinfo.vectort_pointer).is_null() {
            return 0;
        }
        if newpc - old_program_counter < MAXLINEDIFF / 2 {
            let mut delta: i32 = 0;
            let mut program_counter: i32 = old_program_counter;
            loop {
                program_counter += 1;
                let line_info: i32 = *((*p).prototype_lineinfo.vectort_pointer).add(program_counter as usize) as i32;
                if line_info == -MAXLINEDIFF {
                    break;
                }
                delta += line_info;
                if program_counter == newpc {
                    return (delta != 0) as i32;
                }
            }
        }
        (luag_getfuncline(p, old_program_counter) != luag_getfuncline(p, newpc)) as i32
    }
}
pub unsafe fn luaf_newproto(state: *mut State) -> *mut Prototype {
    unsafe {
        let object: *mut Object = luac_newobj(state, TagVariant::Prototype, size_of::<Prototype>());
        let prototype: *mut Prototype = &mut *(object as *mut Prototype);
        (*prototype).prototype_constants.initialize();
        (*prototype).prototype_prototypes.initialize();
        (*prototype).prototype_code.initialize();
        (*prototype).prototype_lineinfo.initialize();
        (*prototype).prototype_absolutelineinfo.initialize();
        (*prototype).prototype_upvalues.initialize();
        (*prototype).prototype_flag = 0;
        (*prototype).prototype_countparameters = 0;
        (*prototype).prototype_isvariablearguments = false;
        (*prototype).prototype_needsvarargtable = false;
        (*prototype).prototype_maximumstacksize = 0;
        (*prototype).prototype_localvariables.initialize();
        (*prototype).prototype_linedefined = 0;
        (*prototype).prototype_lastlinedefined = 0;
        (*prototype).prototype_source = null_mut();
        prototype
    }
}
pub unsafe fn luaf_getlocalname(prototype: *const Prototype, mut local_number: i32, program_counter: i32) -> *const i8 {
    unsafe {
        for i in 0..(*prototype).prototype_localvariables.get_size() {
            if (*((*prototype).prototype_localvariables.vectort_pointer).add(i)).localvariable_startprogramcounter > program_counter
            {
                return null();
            } else if program_counter
                < (*((*prototype).prototype_localvariables.vectort_pointer).add(i)).localvariable_endprogramcounter
            {
                local_number -= 1;
                if local_number == 0 {
                    return (*(*((*prototype).prototype_localvariables.vectort_pointer).add(i)).localvariable_variablename)
                        .get_contents_mut();
                }
            }
        }
        null()
    }
}
