use crate::character::*;
use crate::debugger::absolutelineinfo::*;
use crate::dumpstate::*;
use crate::functionstate::*;
use crate::global::*;
use crate::interpreter::*;
use crate::loadable::*;
use crate::localvariable::*;
use crate::object::*;
use crate::tag::*;
use crate::tm::*;
use crate::tstring::*;
use crate::tvalue::*;
use crate::upvaluedescription::*;
use crate::utility::c::*;
use crate::vectort::*;
use crate::vm::instruction::*;
use crate::vm::opcode::*;
use crate::vm::opmode::*;
use crate::objectwithgclist::*;
use std::ptr::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Prototype {
    pub object: ObjectWithGCList,
    pub prototype_is_variable_arguments: bool,
    pub prototype_count_parameters: u8,
    pub prototype_maximum_stack_size: u8,
    pub prototype_source: *mut TString,
    pub prototype_constants: VectorT<TValue>,
    pub prototype_code: VectorT<u32>,
    pub prototype_prototypes: VectorT<*mut Prototype>,
    pub prototype_upvalues: VectorT<UpValueDescription>,
    pub prototype_local_variables: VectorT<LocalVariable>,
    pub prototype_line_defined: i32,
    pub prototype_last_line_defined: i32,
    pub prototype_line_info: VectorT<i8>,
    pub prototype_absolute_line_info: VectorT<AbsoluteLineInfo>,
}
impl TObject for Prototype {
    fn as_object(&self) -> &ObjectBase {
        self.object.as_object()
    }
    fn as_object_mut(&mut self) -> &mut ObjectBase {
        self.object.as_object_mut()
    }
    fn get_class_name(&mut self) -> String {
        "prototype".to_string()
    }
    fn getgclist(& mut self) -> *mut *mut ObjectBase {
        self.object.getgclist()
    }
}
impl Prototype {
    pub unsafe fn dump_code(&self, dump_state: &mut DumpState) {
        unsafe {
            dump_state.dump_int(self.prototype_code.get_size() as i32);
            dump_state.dump_block(self.prototype_code.vectort_pointer as *const libc::c_void, (self.prototype_code.get_size() as usize).wrapping_mul(size_of::<u32>()));
        }
    }
    pub unsafe fn dump_function(&self, dump_state: &mut DumpState, source: *mut TString) {
        unsafe {
            if dump_state.is_strip || self.prototype_source == source {
                TString::dump_string(dump_state, null());
            } else {
                TString::dump_string(dump_state, self.prototype_source);
            }
            dump_state.dump_int(self.prototype_line_defined);
            dump_state.dump_int(self.prototype_last_line_defined);
            dump_state.dump_byte(self.prototype_count_parameters);
            dump_state.dump_byte(if self.prototype_is_variable_arguments { 1 } else { 0 });
            dump_state.dump_byte(self.prototype_maximum_stack_size);
            self.dump_code(dump_state);
            self.dump_constants(dump_state);
            self.dump_upvalues(dump_state);
            self.dump_prototypes(dump_state);
            self.dump_debug(dump_state);
        }
    }
    pub unsafe fn dump_debug(&self, dump_state: &mut DumpState) {
        unsafe {
            let n = if dump_state.is_strip { 0 } else { self.prototype_line_info.get_size() as usize };
            dump_state.dump_int(n as i32);
            dump_state.dump_block(self.prototype_line_info.vectort_pointer as *const libc::c_void, n);
        }
        unsafe {
            let n = if dump_state.is_strip { 0 } else { self.prototype_absolute_line_info.get_size() as usize };
            dump_state.dump_int(n as i32);
            for i in 0..n {
                dump_state.dump_int((*(self.prototype_absolute_line_info.vectort_pointer).offset(i as isize)).program_counter);
                dump_state.dump_int((*(self.prototype_absolute_line_info.vectort_pointer).offset(i as isize)).line);
            }
        }
        unsafe {
            let n = if dump_state.is_strip { 0 } else { self.prototype_local_variables.get_size() as usize };
            dump_state.dump_int(n as i32);
            for i in 0..n {
                TString::dump_string(dump_state, (*(self.prototype_local_variables.at(i as isize))).variable_name);
                dump_state.dump_int((*(self.prototype_local_variables.at(i as isize))).start_program_counter);
                dump_state.dump_int((*(self.prototype_local_variables.at(i as isize))).end_program_counter);
            }
        }
        unsafe {
            let n = if dump_state.is_strip { 0 } else { self.prototype_upvalues.get_size() as usize };
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
                let tvalue: *const TValue = &mut *(self.prototype_constants.vectort_pointer).offset(i as isize) as *mut TValue;
                let tagvariant = (*tvalue).get_tag_variant();
                dump_state.dump_byte(tagvariant as u8);
                match tagvariant {
                    TagVariant::NumericNumber => {
                        dump_state.dump_number((*tvalue).value.value_number);
                    },
                    TagVariant::NumericInteger => {
                        dump_state.dump_integer((*tvalue).value.value_integer);
                    },
                    TagVariant::StringShort | TagVariant::StringLong => {
                        TString::dump_string(dump_state, &mut (*((*tvalue).value.value_object as *mut TString)));
                    },
                    _ => {},
                }
            }
        }
    }
    pub unsafe fn prototype_traverse(&mut self, global: &mut Global) -> usize {
        unsafe {
            if !self.prototype_source.is_null() {
                if (*self.prototype_source).get_marked() & (1 << 3 | 1 << 4) != 0 {
                    really_mark_object(global, &mut (*(self.prototype_source as *mut ObjectBase)));
                }
            }
            for i in 0..self.prototype_constants.get_size() {
                if ((*(self.prototype_constants.vectort_pointer).offset(i as isize)).is_collectable()) && (*(*(self.prototype_constants.vectort_pointer).offset(i as isize)).value.value_object).get_marked() & (1 << 3 | 1 << 4) != 0 {
                    really_mark_object(global, (*(self.prototype_constants.vectort_pointer).offset(i as isize)).value.value_object);
                }
            }
            for i in 0..self.prototype_upvalues.get_size() {
                if !((*(self.prototype_upvalues.vectort_pointer).offset(i as isize)).upvaluedescription_name).is_null() {
                    if (*(*(self.prototype_upvalues.vectort_pointer).offset(i as isize)).upvaluedescription_name).get_marked() & (1 << 3 | 1 << 4) != 0 {
                        really_mark_object(global, &mut (*((*(self.prototype_upvalues.vectort_pointer).offset(i as isize)).upvaluedescription_name as *mut ObjectBase)));
                    }
                }
            }
            for i in 0..self.prototype_prototypes.get_size() {
                if !(*(self.prototype_prototypes.vectort_pointer).offset(i as isize)).is_null() {
                    if (**(self.prototype_prototypes.vectort_pointer).offset(i as isize)).get_marked() & (1 << 3 | 1 << 4) != 0 {
                        really_mark_object(global, &mut (*(*(self.prototype_prototypes.vectort_pointer).offset(i as isize) as *mut ObjectBase)));
                    }
                }
            }
            for i in 0..self.prototype_local_variables.get_size() {
                if !((*(self.prototype_local_variables.vectort_pointer).offset(i as isize)).variable_name).is_null() {
                    if (*(*(self.prototype_local_variables.vectort_pointer).offset(i as isize)).variable_name).get_marked() & (1 << 3 | 1 << 4) != 0 {
                        really_mark_object(global, &mut (*((*(self.prototype_local_variables.vectort_pointer).offset(i as isize)).variable_name as *mut ObjectBase)));
                    }
                }
            }
            return (1 + self.prototype_constants.get_size() + self.prototype_upvalues.get_size() + self.prototype_prototypes.get_size() + self.prototype_local_variables.get_size()) as usize;
        }
    }
    pub unsafe fn prototype_free(&mut self, interpreter: *mut Interpreter) {
        unsafe {
            (*interpreter).free_memory(
                self.prototype_code.vectort_pointer as *mut libc::c_void,
                (self.prototype_code.get_size() as usize).wrapping_mul(size_of::<u32>() as usize) as usize,
            );
            (*interpreter).free_memory(
                self.prototype_prototypes.vectort_pointer as *mut libc::c_void,
                (self.prototype_prototypes.get_size() as usize).wrapping_mul(size_of::<*mut Prototype>() as usize) as usize,
            );
            (*interpreter).free_memory(
                self.prototype_constants.vectort_pointer as *mut libc::c_void,
                (self.prototype_constants.get_size() as usize).wrapping_mul(size_of::<TValue>() as usize) as usize,
            );
            (*interpreter).free_memory(
                self.prototype_line_info.vectort_pointer as *mut libc::c_void,
                (self.prototype_line_info.get_size() as usize).wrapping_mul(1 as usize) as usize,
            );
            (*interpreter).free_memory(
                self.prototype_absolute_line_info.vectort_pointer as *mut libc::c_void,
                (self.prototype_absolute_line_info.get_size() as usize).wrapping_mul(size_of::<AbsoluteLineInfo>() as usize) as usize,
            );
            (*interpreter).free_memory(
                self.prototype_local_variables.vectort_pointer as *mut libc::c_void,
                (self.prototype_local_variables.get_size() as usize).wrapping_mul(size_of::<LocalVariable>() as usize) as usize,
            );
            (*interpreter).free_memory(
                self.prototype_upvalues.vectort_pointer as *mut libc::c_void,
                (self.prototype_upvalues.get_size() as usize).wrapping_mul(size_of::<UpValueDescription>() as usize) as usize,
            );
            (*interpreter).free_memory(self as *mut Prototype as *mut libc::c_void, size_of::<Prototype>());
        }
    }
}
pub unsafe fn getbaseline(prototype: *const Prototype, program_counter: i32, basepc: *mut i32) -> i32 {
    unsafe {
        if (*prototype).prototype_absolute_line_info.get_size() == 0 || program_counter < (*((*prototype).prototype_absolute_line_info.vectort_pointer).offset(0 as isize)).program_counter {
            *basepc = -1;
            return (*prototype).prototype_line_defined;
        } else {
            let mut i = program_counter / 128 - 1;
            while (i + 1) < (*prototype).prototype_absolute_line_info.get_size() as i32 && program_counter >= (*((*prototype).prototype_absolute_line_info.vectort_pointer).offset((i + 1) as isize)).program_counter {
                i += 1;
            }
            *basepc = (*((*prototype).prototype_absolute_line_info.vectort_pointer).offset(i as isize)).program_counter;
            return (*((*prototype).prototype_absolute_line_info.vectort_pointer).offset(i as isize)).line;
        };
    }
}
pub unsafe fn luag_getfuncline(prototype: *const Prototype, program_counter: i32) -> i32 {
    unsafe {
        if ((*prototype).prototype_line_info.vectort_pointer).is_null() {
            return -1;
        } else {
            let mut basepc: i32 = 0;
            let mut baseline: i32 = getbaseline(prototype, program_counter, &mut basepc);
            loop {
                let fresh8 = basepc;
                basepc = basepc + 1;
                if !(fresh8 < program_counter) {
                    break;
                }
                baseline += *((*prototype).prototype_line_info.vectort_pointer).offset(basepc as isize) as i32;
            }
            return baseline;
        };
    }
}
pub unsafe fn upvalname(p: *const Prototype, uv: i32) -> *const i8 {
    unsafe {
        let s: *mut TString = (*((*p).prototype_upvalues.vectort_pointer).offset(uv as isize)).upvaluedescription_name;
        if s.is_null() {
            return c"?".as_ptr();
        } else {
            return (*s).get_contents_mut();
        };
    }
}
pub unsafe fn nextline(p: *const Prototype, currentline: i32, program_counter: i32) -> i32 {
    unsafe {
        if *((*p).prototype_line_info.vectort_pointer).offset(program_counter as isize) as i32 != -(0x80 as i32) {
            return currentline + *((*p).prototype_line_info.vectort_pointer).offset(program_counter as isize) as i32;
        } else {
            return luag_getfuncline(p, program_counter);
        };
    }
}
pub unsafe fn findsetreg(p: *const Prototype, mut lastpc: i32, reg: i32) -> i32 {
    unsafe {
        let mut setreg: i32 = -1;
        let mut jmptarget: i32 = 0;
        if OPMODES[(*((*p).prototype_code.vectort_pointer).offset(lastpc as isize) >> 0 & !(!(0) << 7) << 0) as usize] as i32 & 1 << 7 != 0 {
            lastpc -= 1;
        }
        let mut program_counter: i32 = 0;
        while program_counter < lastpc {
            let i: u32 = *((*p).prototype_code.vectort_pointer).offset(program_counter as isize);
            let op: u32 = (i >> 0 & !(!(0) << 7) << 0) as u32;
            let a: i32 = (i >> POSITION_A & !(!(0) << 8) << 0) as i32;
            let change: i32;
            match op as u32 {
                8 => {
                    let b: i32 = (i >> POSITION_B & !(!(0) << 8) << 0) as i32;
                    change = (a <= reg && reg <= a + b) as i32;
                },
                76 => {
                    change = (reg >= a + 2) as i32;
                },
                68 | 69 => {
                    change = (reg >= a) as i32;
                },
                56 => {
                    let b_0: i32 = (i >> POSITION_A & !(!(0u32) << 8 + 8 + 1 + 8) << 0) as i32 - ((1 << 8 + 8 + 1 + 8) - 1 >> 1);
                    let dest: i32 = program_counter + 1 + b_0;
                    if dest <= lastpc && dest > jmptarget {
                        jmptarget = dest;
                    }
                    change = 0;
                },
                _ => {
                    change = (OPMODES[op as usize] as i32 & 1 << 3 != 0 && reg == a) as i32;
                },
            }
            if change != 0 {
                setreg = filter_program_counter(program_counter, jmptarget);
            }
            program_counter += 1;
        }
        return setreg;
    }
}
pub unsafe fn kname(p: *const Prototype, index: i32, name: *mut *const i8) -> *const i8 {
    unsafe {
        let kvalue: *mut TValue = &mut *((*p).prototype_constants.vectort_pointer).offset(index as isize) as *mut TValue;
        if (*kvalue).is_tagtype_string() {
            *name = (*((*kvalue).value.value_object as *mut TString)).get_contents_mut();
            return c"code_constant".as_ptr();
        } else {
            *name = c"?".as_ptr();
            return null();
        };
    }
}
pub unsafe fn basicgetobjname(p: *const Prototype, ppc: *mut i32, reg: i32, name: *mut *const i8) -> *const i8 {
    unsafe {
        let mut program_counter: i32 = *ppc;
        *name = luaf_getlocalname(p, reg + 1, program_counter);
        if !(*name).is_null() {
            return STRING_LOCAL;
        }
        program_counter = findsetreg(p, program_counter, reg);
        *ppc = program_counter;
        if program_counter != -1 {
            let i: u32 = *((*p).prototype_code.vectort_pointer).offset(program_counter as isize);
            let op: u32 = (i >> 0 & !(!(0u32) << 7) << 0) as u32;
            match op as u32 {
                0 => {
                    let b: i32 = (i >> POSITION_B & !(!(0u32) << 8) << 0) as i32;
                    if b < (i >> POSITION_A & !(!(0u32) << 8) << 0) as i32 {
                        return basicgetobjname(p, ppc, b, name);
                    }
                },
                9 => {
                    *name = upvalname(p, (i >> POSITION_B & !(!(0u32) << 8) << 0) as i32);
                    return STRING_UPVALUE;
                },
                3 => {
                    return kname(p, (i >> POSITION_K & !(!(0u32) << 8 + 8 + 1) << 0) as i32, name);
                },
                4 => {
                    return kname(
                        p,
                        (*((*p).prototype_code.vectort_pointer).offset((program_counter + 1) as isize) >> POSITION_A & !(!(0u32) << 8 + 8 + 1 + 8) << 0) as i32,
                        name,
                    );
                },
                _ => {},
            }
        }
        return null();
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
        let c: i32 = (i >> POSITION_C & !(!(0u32) << 8) << 0) as i32;
        if (i >> POSITION_K & !(!(0u32) << 1) << 0) as i32 != 0 {
            kname(p, c, name);
        } else {
            rname(p, program_counter, c, name);
        };
    }
}
pub unsafe fn is_environment(p: *const Prototype, mut program_counter: i32, i: u32, isup: i32) -> *const i8 {
    unsafe {
        let t: i32 = (i >> POSITION_B & !(!(0u32) << 8) << 0) as i32;
        let mut name: *const i8 = null();
        if isup != 0 {
            name = upvalname(p, t);
        } else {
            let what: *const i8 = basicgetobjname(p, &mut program_counter, t, &mut name);
            if what != STRING_LOCAL && what != STRING_UPVALUE {
                name = null();
            }
        }
        return if !name.is_null() && strcmp(name, c"_ENV".as_ptr()) == 0 { c"global".as_ptr() } else { c"field".as_ptr() };
    }
}
pub unsafe fn getobjname(p: *const Prototype, mut lastpc: i32, reg: i32, name: *mut *const i8) -> *const i8 {
    unsafe {
        let kind: *const i8 = basicgetobjname(p, &mut lastpc, reg, name);
        if !kind.is_null() {
            return kind;
        } else if lastpc != -1 {
            let i: u32 = *((*p).prototype_code.vectort_pointer).offset(lastpc as isize);
            let op: u32 = (i >> 0 & !(!(0u32) << 7) << 0) as u32;
            match op as u32 {
                11 => {
                    let k: i32 = (i >> POSITION_C & !(!(0u32) << 8) << 0) as i32;
                    kname(p, k, name);
                    return is_environment(p, lastpc, i, 1);
                },
                12 => {
                    let k_0: i32 = (i >> POSITION_C & !(!(0u32) << 8) << 0) as i32;
                    rname(p, lastpc, k_0, name);
                    return is_environment(p, lastpc, i, 0);
                },
                13 => {
                    *name = c"integer index".as_ptr();
                    return c"field".as_ptr();
                },
                14 => {
                    let k_1: i32 = (i >> POSITION_C & !(!(0u32) << 8) << 0) as i32;
                    kname(p, k_1, name);
                    return is_environment(p, lastpc, i, 0);
                },
                20 => {
                    rkname(p, lastpc, i, name);
                    return c"method".as_ptr();
                },
                _ => {},
            }
        }
        return null();
    }
}
pub unsafe fn funcnamefromcode(interpreter: *mut Interpreter, p: *const Prototype, program_counter: i32, name: *mut *const i8) -> *const i8 {
    unsafe {
        let tm: u32;
        let i: u32 = *((*p).prototype_code.vectort_pointer).offset(program_counter as isize);
        match (i >> 0 & !(!(0u32) << 7) << 0) as u32 {
            OPCODE_CALL | OPCODE_TAILCALL => {
                return getobjname(p, program_counter, (i >> POSITION_A & !(!(0u32) << 8) << 0) as i32, name);
            },
            OPCODE_TFORCALL => {
                *name = c"for iterator".as_ptr();
                return c"for iterator".as_ptr();
            },
            OPCODE_SELF | OPCODE_GET_TABLE_UPVALUE | OPCODE_GET_TABLE | OPCODE_INDEX_INTEGER | OPCODE_GET_FIELD => {
                tm = TM_INDEX;
            },
            OPCODE_SETTABUP | OPCODE_SETTABLE | OPCODE_SETI | OPCODE_SETFIELD => {
                tm = TM_NEWINDEX;
            },
            OPCODE_MMBIN | OPCODE_MMBINI | OPCODE_MMBINK => {
                tm = (i >> POSITION_C & !(!(0u32) << 8) << 0) as u32;
            },
            OPCODE_UNM => {
                tm = TM_UNM;
            },
            OPCODE_BNOT => {
                tm = TM_BNOT;
            },
            OPCODE_LEN => {
                tm = TM_LEN;
            },
            OPCODE_CONCAT => {
                tm = TM_CONCAT;
            },
            OPCODE_EQ => {
                tm = TM_EQ;
            },
            OPCODE_LT | OPCODE_LTI | OPCODE_GTI => {
                tm = TM_LT;
            },
            OPCODE_LE | OPCODE_LEI | OPCODE_GEI => {
                tm = TM_LE;
            },
            OPCODE_CLOSE | OPCODE_RETURN => {
                tm = TM_CLOSE;
            },
            _ => return null(),
        }
        *name = ((*(*(*interpreter).global).global_tmname[tm as usize]).get_contents()).offset(2 as isize);
        return c"metamethod".as_ptr();
    }
}
pub unsafe fn changedline(p: *const Prototype, old_program_counter: i32, newpc: i32) -> i32 {
    unsafe {
        if ((*p).prototype_line_info.vectort_pointer).is_null() {
            return 0;
        }
        if newpc - old_program_counter < 128 as i32 / 2 {
            let mut delta: i32 = 0;
            let mut program_counter: i32 = old_program_counter;
            loop {
                program_counter += 1;
                let line_info: i32 = *((*p).prototype_line_info.vectort_pointer).offset(program_counter as isize) as i32;
                if line_info == -(0x80 as i32) {
                    break;
                }
                delta += line_info;
                if program_counter == newpc {
                    return (delta != 0) as i32;
                }
            }
        }
        return (luag_getfuncline(p, old_program_counter) != luag_getfuncline(p, newpc)) as i32;
    }
}
pub unsafe fn luaf_newproto(interpreter: *mut Interpreter) -> *mut Prototype {
    unsafe {
        let object: *mut ObjectBase = luac_newobj(interpreter, TagVariant::Prototype, size_of::<Prototype>());
        let prototype: *mut Prototype = &mut (*(object as *mut Prototype));
        (*prototype).prototype_constants.initialize();
        (*prototype).prototype_prototypes.initialize();
        (*prototype).prototype_code.initialize();
        (*prototype).prototype_line_info.initialize();
        (*prototype).prototype_absolute_line_info.initialize();
        (*prototype).prototype_upvalues.initialize();
        (*prototype).prototype_count_parameters = 0;
        (*prototype).prototype_is_variable_arguments = false;
        (*prototype).prototype_maximum_stack_size = 0;
        (*prototype).prototype_local_variables.initialize();
        (*prototype).prototype_line_defined = 0;
        (*prototype).prototype_last_line_defined = 0;
        (*prototype).prototype_source = null_mut();
        return prototype;
    }
}
pub unsafe fn luaf_getlocalname(prototype: *const Prototype, mut local_number: i32, program_counter: i32) -> *const i8 {
    unsafe {
        for i in 0..(*prototype).prototype_local_variables.get_size() {
            if (*((*prototype).prototype_local_variables.vectort_pointer).offset(i as isize)).start_program_counter > program_counter {
                return null();
            } else if program_counter < (*((*prototype).prototype_local_variables.vectort_pointer).offset(i as isize)).end_program_counter {
                local_number -= 1;
                if local_number == 0 {
                    return (*(*((*prototype).prototype_local_variables.vectort_pointer).offset(i as isize)).variable_name).get_contents_mut();
                }
            }
        }
        return null();
    }
}
