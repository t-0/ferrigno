use crate::debugger::absolutelineinfo::*;
use crate::dumpstate::*;
use crate::vm::opcode::*;
use crate::character::*;
use crate::localvariable::*;
use crate::vm::opmode::*;
use crate::functionstate::*;
use crate::object::*;
use crate::global::*;
use crate::interpreter::*;
use crate::loadable::*;
use crate::table::*;
use crate::tag::*;
use crate::tstring::*;
use crate::tvalue::*;
use crate::tm::*;
use crate::utility::c::*;
use crate::vm::instruction::*;
use crate::upvaluedescription::*;
use crate::vectort::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Prototype {
    pub object: Object,
    pub prototype_gc_list: *mut Object,
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
    fn as_object(&self) -> &Object {
        &self.object
    }
    fn as_object_mut(&mut self) -> &mut Object {
        &mut self.object
    }
    fn get_class_name(&mut self) -> String {
        "prototype".to_string()
    }
    fn get_metatable(&mut self) -> *mut Table {
        std::ptr::null_mut()
    }
}
impl Prototype {
    pub unsafe extern "C" fn dump_code(&self, dump_state: &mut DumpState) {
        unsafe {
            dump_state.dump_int(self.prototype_code.size);
            dump_state.dump_block(
                self.prototype_code.pointer as *const libc::c_void,
                (self.prototype_code.size as usize).wrapping_mul(::core::mem::size_of::<u32>()),
            );
        }
    }
    pub unsafe extern "C" fn dump_function(
        &self,
        dump_state: &mut DumpState,
        source: *mut TString,
    ) {
        unsafe {
            if dump_state.is_strip || self.prototype_source == source {
                dump_state.dump_string(std::ptr::null());
            } else {
                dump_state.dump_string(self.prototype_source);
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
    pub unsafe extern "C" fn dump_debug(&self, dump_state: &mut DumpState) {
        unsafe {
            let n = if dump_state.is_strip {
                0
            } else {
                self.prototype_line_info.size as usize
            };
            dump_state.dump_int(n as i32);
            dump_state.dump_block(
                self.prototype_line_info.pointer as *const libc::c_void,
                n.wrapping_mul(::core::mem::size_of::<i8>()),
            );
        }
        unsafe {
            let n = if dump_state.is_strip {
                0
            } else {
                self.prototype_absolute_line_info.size as usize
            };
            dump_state.dump_int(n as i32);
            for i in 0..n {
                dump_state.dump_int(
                    (*(self.prototype_absolute_line_info.pointer).offset(i as isize)).program_counter,
                );
                dump_state.dump_int(
                    (*(self.prototype_absolute_line_info.pointer).offset(i as isize)).line,
                );
            }
        }
        unsafe {
            let n = if dump_state.is_strip {
                0
            } else {
                self.prototype_local_variables.size as usize
            };
            dump_state.dump_int(n as i32);
            for i in 0..n {
                dump_state.dump_string(
                    (*(self.prototype_local_variables.at(i as isize))).variable_name,
                );
                dump_state.dump_int(
                    (*(self.prototype_local_variables.at(i as isize))).start_program_counter,
                );
                dump_state.dump_int(
                    (*(self.prototype_local_variables.at(i as isize))).end_program_counter,
                );
            }
        }
        unsafe {
            let n = if dump_state.is_strip {
                0
            } else {
                self.prototype_upvalues.size as usize
            };
            dump_state.dump_int(n as i32);
            for i in 0..n {
                dump_state.dump_string((*(self.prototype_upvalues.at(i as isize))).name);
            }
        }
    }
    pub unsafe extern "C" fn dump_prototypes(&self, dump_state: &mut DumpState) {
        unsafe {
            let n: i32 = self.prototype_prototypes.size;
            dump_state.dump_int(n);
            for i in 0..n {
                (*(*(self.prototype_prototypes.at(i as isize)))).dump_function(dump_state, self.prototype_source);
            }
        }
    }
    pub unsafe extern "C" fn dump_upvalues(&self, dump_state: &mut DumpState) {
        unsafe {
            let n: i32 = self.prototype_upvalues.size;
            dump_state.dump_int(n);
            for i in 0..n {
                let upvalue_description = *(self.prototype_upvalues.at(i as isize));
                upvalue_description.dump(dump_state);
            }
        }
    }
    pub unsafe extern "C" fn dump_constants(&self, dump_state: &mut DumpState) {
        unsafe {
            let n: i32 = self.prototype_constants.size;
            dump_state.dump_int(n);
            for i in 0..n {
                let tvalue: *const TValue = &mut *(self.prototype_constants.pointer).offset(i as isize) as *mut TValue;
                let tag = (*tvalue).get_tag_variant();
                dump_state.dump_byte(tag);
                match tag {
                    TAG_VARIANT_NUMERIC_NUMBER => {
                        dump_state.dump_number((*tvalue).value.number);
                    }
                    TAG_VARIANT_NUMERIC_INTEGER => {
                        dump_state.dump_integer((*tvalue).value.integer);
                    }
                    TAG_VARIANT_STRING_SHORT | TAG_VARIANT_STRING_LONG => {
                        dump_state.dump_string(&mut (*((*tvalue).value.object as *mut TString)));
                    }
                    _ => {}
                }
            }
        }
    }
    pub unsafe extern "C" fn prototype_traverse(&mut self, global: &mut Global) -> u64 {
        unsafe {
            if !self.prototype_source.is_null() {
                if (*self.prototype_source).get_marked() & (1 << 3 | 1 << 4) != 0 {
                    really_mark_object(global, &mut (*(self.prototype_source as *mut Object)));
                }
            }
            for i in 0..self.prototype_constants.size {
                if ((*(self.prototype_constants.pointer).offset(i as isize)).is_collectable())
                    && (*(*(self.prototype_constants.pointer).offset(i as isize)).value.object).get_marked() & (1 << 3 | 1 << 4)
                        != 0
                {
                    really_mark_object(global, (*(self.prototype_constants.pointer).offset(i as isize)).value.object);
                }
            }
            for i in 0..self.prototype_upvalues.size {
                if !((*(self.prototype_upvalues.pointer).offset(i as isize)).name).is_null() {
                    if (*(*(self.prototype_upvalues.pointer).offset(i as isize)).name).get_marked() & (1 << 3 | 1 << 4)
                        != 0
                    {
                        really_mark_object(
                            global,
                            &mut (*((*(self.prototype_upvalues.pointer).offset(i as isize)).name as *mut Object)),
                        );
                    }
                }
            }
            for i in 0..self.prototype_prototypes.size {
                if !(*(self.prototype_prototypes.pointer).offset(i as isize)).is_null() {
                    if (**(self.prototype_prototypes.pointer).offset(i as isize)).get_marked() & (1 << 3 | 1 << 4) != 0 {
                        really_mark_object(
                            global,
                            &mut (*(*(self.prototype_prototypes.pointer).offset(i as isize) as *mut Object)),
                        );
                    }
                }
            }
            for i in 0..self.prototype_local_variables.size {
                if !((*(self.prototype_local_variables.pointer).offset(i as isize)).variable_name).is_null() {
                    if (*(*(self.prototype_local_variables.pointer).offset(i as isize)).variable_name).get_marked()
                        & (1 << 3 | 1 << 4)
                        != 0
                    {
                        really_mark_object(
                            global,
                            &mut (*((*(self.prototype_local_variables.pointer).offset(i as isize)).variable_name
                                as *mut Object)),
                        );
                    }
                }
            }
            return (1 + self.prototype_constants.size + self.prototype_upvalues.size + self.prototype_prototypes.size + self.prototype_local_variables.size) as u64
        }
    }
    pub unsafe extern "C" fn prototype_free(&mut self, interpreter: *mut Interpreter) {
        unsafe {
            (*interpreter).free_memory(
                self.prototype_code.pointer as *mut libc::c_void,
                (self.prototype_code.size as u64).wrapping_mul(::core::mem::size_of::<u32>() as u64) as usize,
            );
            (*interpreter).free_memory(
                self.prototype_prototypes.pointer as *mut libc::c_void,
                (self.prototype_prototypes.size as u64).wrapping_mul(::core::mem::size_of::<*mut Prototype>() as u64) as usize,
            );
            (*interpreter).free_memory(
                self.prototype_constants.pointer as *mut libc::c_void,
                (self.prototype_constants.size as u64).wrapping_mul(::core::mem::size_of::<TValue>() as u64) as usize,
            );
            (*interpreter).free_memory(
                self.prototype_line_info.pointer as *mut libc::c_void,
                (self.prototype_line_info.size as u64).wrapping_mul(::core::mem::size_of::<i8>() as u64) as usize,
            );
            (*interpreter).free_memory(
                self.prototype_absolute_line_info.pointer as *mut libc::c_void,
                (self.prototype_absolute_line_info.size as u64)
                    .wrapping_mul(::core::mem::size_of::<AbsoluteLineInfo>() as u64) as usize,
            );
            (*interpreter).free_memory(
                self.prototype_local_variables.pointer as *mut libc::c_void,
                (self.prototype_local_variables.size as u64)
                    .wrapping_mul(::core::mem::size_of::<LocalVariable>() as u64) as usize,
            );
            (*interpreter).free_memory(
                self.prototype_upvalues.pointer as *mut libc::c_void,
                (self.prototype_upvalues.size as u64)
                    .wrapping_mul(::core::mem::size_of::<UpValueDescription>() as u64) as usize,
            );
            (*interpreter).free_memory(
                self as *mut Prototype as *mut libc::c_void,
                ::core::mem::size_of::<Prototype>(),
            );
        }
    }
}
pub unsafe extern "C" fn getbaseline(
    prototype: *const Prototype,
    program_counter: i32,
    basepc: *mut i32,
) -> i32 {
    unsafe {
        if (*prototype).prototype_absolute_line_info.size == 0
            || program_counter < (*((*prototype).prototype_absolute_line_info.pointer).offset(0 as isize)).program_counter
        {
            *basepc = -1;
            return (*prototype).prototype_line_defined;
        } else {
            let mut i: i32 = (program_counter as u32)
                .wrapping_div(128u32)
                .wrapping_sub(1u32) as i32;
            while (i + 1) < (*prototype).prototype_absolute_line_info.size
                && program_counter
                    >= (*((*prototype).prototype_absolute_line_info.pointer).offset((i + 1) as isize)).program_counter
            {
                i += 1;
            }
            *basepc = (*((*prototype).prototype_absolute_line_info.pointer).offset(i as isize)).program_counter;
            return (*((*prototype).prototype_absolute_line_info.pointer).offset(i as isize)).line;
        };
    }
}
pub unsafe extern "C" fn luag_getfuncline(prototype: *const Prototype, program_counter: i32) -> i32 {
    unsafe {
        if ((*prototype).prototype_line_info.pointer).is_null() {
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
                baseline += *((*prototype).prototype_line_info.pointer).offset(basepc as isize) as i32;
            }
            return baseline;
        };
    }
}
pub unsafe extern "C" fn upvalname(p: *const Prototype, uv: i32) -> *const i8 {
    unsafe {
        let s: *mut TString = (*((*p).prototype_upvalues.pointer).offset(uv as isize)).name;
        if s.is_null() {
            return b"?\0" as *const u8 as *const i8;
        } else {
            return (*s).get_contents_mut();
        };
    }
}
pub unsafe extern "C" fn nextline(
    p: *const Prototype,
    currentline: i32,
    program_counter: i32,
) -> i32 {
    unsafe {
        if *((*p).prototype_line_info.pointer).offset(program_counter as isize) as i32 != -(0x80 as i32) {
            return currentline + *((*p).prototype_line_info.pointer).offset(program_counter as isize) as i32;
        } else {
            return luag_getfuncline(p, program_counter);
        };
    }
}
pub unsafe extern "C" fn findsetreg(p: *const Prototype, mut lastpc: i32, reg: i32) -> i32 {
    unsafe {
        let mut setreg: i32 = -1;
        let mut jmptarget: i32 = 0;
        if OPMODES
            [(*((*p).prototype_code.pointer).offset(lastpc as isize) >> 0 & !(!(0) << 7) << 0) as usize]
            as i32
            & 1 << 7
            != 0
        {
            lastpc -= 1;
        }
        let mut program_counter: i32 = 0;
        while program_counter < lastpc {
            let i: u32 = *((*p).prototype_code.pointer).offset(program_counter as isize);
            let op: u32 = (i >> 0 & !(!(0) << 7) << 0) as u32;
            let a: i32 = (i >> POSITION_A & !(!(0) << 8) << 0) as i32;
            let change: i32;
            match op as u32 {
                8 => {
                    let b: i32 = (i >> POSITION_B & !(!(0) << 8) << 0) as i32;
                    change = (a <= reg && reg <= a + b) as i32;
                }
                76 => {
                    change = (reg >= a + 2) as i32;
                }
                68 | 69 => {
                    change = (reg >= a) as i32;
                }
                56 => {
                    let b_0: i32 = (i >> POSITION_A & !(!(0u32) << 8 + 8 + 1 + 8) << 0) as i32
                        - ((1 << 8 + 8 + 1 + 8) - 1 >> 1);
                    let dest: i32 = program_counter + 1 + b_0;
                    if dest <= lastpc && dest > jmptarget {
                        jmptarget = dest;
                    }
                    change = 0;
                }
                _ => {
                    change = (OPMODES[op as usize] as i32 & 1 << 3 != 0 && reg == a) as i32;
                }
            }
            if change != 0 {
                setreg = filter_program_counter(program_counter, jmptarget);
            }
            program_counter += 1;
        }
        return setreg;
    }
}
pub unsafe extern "C" fn kname(p: *const Prototype, index: i32, name: *mut *const i8) -> *const i8 {
    unsafe {
        let kvalue: *mut TValue = &mut *((*p).prototype_constants.pointer).offset(index as isize) as *mut TValue;
        if (*kvalue).is_tagtype_string() {
            *name = (*((*kvalue).value.object as *mut TString)).get_contents_mut();
            return b"constant\0" as *const u8 as *const i8;
        } else {
            *name = b"?\0" as *const u8 as *const i8;
            return std::ptr::null();
        };
    }
}
pub unsafe extern "C" fn basicgetobjname(
    p: *const Prototype,
    ppc: *mut i32,
    reg: i32,
    name: *mut *const i8,
) -> *const i8 {
    unsafe {
        let mut program_counter: i32 = *ppc;
        *name = luaf_getlocalname(p, reg + 1, program_counter);
        if !(*name).is_null() {
            return STRING_LOCAL.as_ptr();
        }
        program_counter = findsetreg(p, program_counter, reg);
        *ppc = program_counter;
        if program_counter != -1 {
            let i: u32 = *((*p).prototype_code.pointer).offset(program_counter as isize);
            let op: u32 = (i >> 0 & !(!(0u32) << 7) << 0) as u32;
            match op as u32 {
                0 => {
                    let b: i32 = (i >> POSITION_B & !(!(0u32) << 8) << 0) as i32;
                    if b < (i >> POSITION_A & !(!(0u32) << 8) << 0) as i32 {
                        return basicgetobjname(p, ppc, b, name);
                    }
                }
                9 => {
                    *name = upvalname(p, (i >> POSITION_B & !(!(0u32) << 8) << 0) as i32);
                    return STRING_UPVALUE.as_ptr();
                }
                3 => {
                    return kname(
                        p,
                        (i >> POSITION_K & !(!(0u32) << 8 + 8 + 1) << 0) as i32,
                        name,
                    );
                }
                4 => {
                    return kname(
                        p,
                        (*((*p).prototype_code.pointer).offset((program_counter + 1) as isize) >> POSITION_A
                            & !(!(0u32) << 8 + 8 + 1 + 8) << 0) as i32,
                        name,
                    );
                }
                _ => {}
            }
        }
        return std::ptr::null();
    }
}
pub unsafe extern "C" fn rname(
    p: *const Prototype,
    mut program_counter: i32,
    c: i32,
    name: *mut *const i8,
) {
    unsafe {
        let what: *const i8 = basicgetobjname(p, &mut program_counter, c, name);
        if !(!what.is_null() && *what as i32 == CHARACTER_LOWER_C as i32) {
            *name = b"?\0" as *const u8 as *const i8;
        }
    }
}
pub unsafe extern "C" fn rkname(
    p: *const Prototype,
    program_counter: i32,
    i: u32,
    name: *mut *const i8,
) {
    unsafe {
        let c: i32 = (i >> POSITION_C & !(!(0u32) << 8) << 0) as i32;
        if (i >> POSITION_K & !(!(0u32) << 1) << 0) as i32 != 0 {
            kname(p, c, name);
        } else {
            rname(p, program_counter, c, name);
        };
    }
}
pub unsafe extern "C" fn is_environment(
    p: *const Prototype,
    mut program_counter: i32,
    i: u32,
    isup: i32,
) -> *const i8 {
    unsafe {
        let t: i32 = (i >> POSITION_B & !(!(0u32) << 8) << 0) as i32;
        let mut name: *const i8 = std::ptr::null();
        if isup != 0 {
            name = upvalname(p, t);
        } else {
            let what: *const i8 = basicgetobjname(p, &mut program_counter, t, &mut name);
            if what != STRING_LOCAL.as_ptr() && what != STRING_UPVALUE.as_ptr() {
                name = std::ptr::null();
            }
        }
        return if !name.is_null() && strcmp(name, b"_ENV\0" as *const u8 as *const i8) == 0 {
            b"global\0" as *const u8 as *const i8
        } else {
            b"field\0" as *const u8 as *const i8
        };
    }
}
pub unsafe extern "C" fn getobjname(
    p: *const Prototype,
    mut lastpc: i32,
    reg: i32,
    name: *mut *const i8,
) -> *const i8 {
    unsafe {
        let kind: *const i8 = basicgetobjname(p, &mut lastpc, reg, name);
        if !kind.is_null() {
            return kind;
        } else if lastpc != -1 {
            let i: u32 = *((*p).prototype_code.pointer).offset(lastpc as isize);
            let op: u32 = (i >> 0 & !(!(0u32) << 7) << 0) as u32;
            match op as u32 {
                11 => {
                    let k: i32 = (i >> POSITION_C & !(!(0u32) << 8) << 0) as i32;
                    kname(p, k, name);
                    return is_environment(p, lastpc, i, 1);
                }
                12 => {
                    let k_0: i32 = (i >> POSITION_C & !(!(0u32) << 8) << 0) as i32;
                    rname(p, lastpc, k_0, name);
                    return is_environment(p, lastpc, i, 0);
                }
                13 => {
                    *name = b"integer index\0" as *const u8 as *const i8;
                    return b"field\0" as *const u8 as *const i8;
                }
                14 => {
                    let k_1: i32 = (i >> POSITION_C & !(!(0u32) << 8) << 0) as i32;
                    kname(p, k_1, name);
                    return is_environment(p, lastpc, i, 0);
                }
                20 => {
                    rkname(p, lastpc, i, name);
                    return b"method\0" as *const u8 as *const i8;
                }
                _ => {}
            }
        }
        return std::ptr::null();
    }
}
pub unsafe extern "C" fn funcnamefromcode(
    interpreter: *mut Interpreter,
    p: *const Prototype,
    program_counter: i32,
    name: *mut *const i8,
) -> *const i8 {
    unsafe {
        let tm: u32;
        let i: u32 = *((*p).prototype_code.pointer).offset(program_counter as isize);
        match (i >> 0 & !(!(0u32) << 7) << 0) as u32 {
            OP_CALL | OP_TAILCALL => {
                return getobjname(
                    p,
                    program_counter,
                    (i >> POSITION_A & !(!(0u32) << 8) << 0) as i32,
                    name,
                );
            }
            OP_TFORCALL => {
                *name = b"for iterator\0" as *const u8 as *const i8;
                return b"for iterator\0" as *const u8 as *const i8;
            }
            OP_SELF | OP_GETTABUP | OP_GETTABLE | OP_GETI | OP_GETFIELD => {
                tm = TM_INDEX;
            }
            OP_SETTABUP | OP_SETTABLE | OP_SETI | OP_SETFIELD => {
                tm = TM_NEWINDEX;
            }
            OP_MMBIN | OP_MMBINI | OP_MMBINK => {
                tm = (i >> POSITION_C & !(!(0u32) << 8) << 0) as u32;
            }
            OP_UNM => {
                tm = TM_UNM;
            }
            OP_BNOT => {
                tm = TM_BNOT;
            }
            OP_LEN => {
                tm = TM_LEN;
            }
            OP_CONCAT => {
                tm = TM_CONCAT;
            }
            OP_EQ => {
                tm = TM_EQ;
            }
            OP_LT | OP_LTI | OP_GTI => {
                tm = TM_LT;
            }
            OP_LE | OP_LEI | OP_GEI => {
                tm = TM_LE;
            }
            OP_CLOSE | OP_RETURN => {
                tm = TM_CLOSE;
            }
            _ => return std::ptr::null(),
        }
        *name = ((*(*(*interpreter).global).tm_name[tm as usize]).get_contents())
            .offset(2 as isize);
        return b"metamethod\0" as *const u8 as *const i8;
    }
}
pub unsafe extern "C" fn changedline(
    p: *const Prototype,
    old_program_counter: i32,
    newpc: i32,
) -> i32 {
    unsafe {
        if ((*p).prototype_line_info.pointer).is_null() {
            return 0;
        }
        if newpc - old_program_counter < 128 as i32 / 2 {
            let mut delta: i32 = 0;
            let mut program_counter: i32 = old_program_counter;
            loop {
                program_counter += 1;
                let line_info: i32 = *((*p).prototype_line_info.pointer).offset(program_counter as isize) as i32;
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
pub unsafe extern "C" fn luaf_newproto(interpreter: *mut Interpreter) -> *mut Prototype {
    unsafe {
        let object: *mut Object = luac_newobj(
            interpreter,
            TAG_VARIANT_PROTOTYPE,
            ::core::mem::size_of::<Prototype>(),
        );
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
        (*prototype).prototype_source = std::ptr::null_mut();
        return prototype;
    }
}
pub unsafe extern "C" fn luaf_getlocalname(
    prototype: *const Prototype,
    mut local_number: i32,
    program_counter: i32,
) -> *const i8 {
    unsafe {
        for i in 0..(*prototype).prototype_local_variables.size {
            if (*((*prototype).prototype_local_variables.pointer).offset(i as isize)).start_program_counter > program_counter {
                return std::ptr::null();
            } else if program_counter < (*((*prototype).prototype_local_variables.pointer).offset(i as isize)).end_program_counter {
                local_number -= 1;
                if local_number == 0 {
                    return (*(*((*prototype).prototype_local_variables.pointer).offset(i as isize)).variable_name)
                        .get_contents_mut();
                }
            }
        }
        return std::ptr::null();
    }
}
