use crate::debugger::absolutelineinfo::*;
use crate::localvariable::*;
use crate::object::*;
use crate::global::*;
use crate::state::*;
use crate::table::*;
use crate::tag::*;
use crate::tstring::*;
use crate::tvalue::*;
use crate::tm::*;
use crate::utility::c::*;
use crate::vm::instruction::*;
use crate::upvaluedescription::*;
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct Prototype {
    pub next: *mut Object,
    pub tag: u8,
    pub marked: u8,
    pub count_parameters: u8,
    pub is_variable_arguments: bool,
    pub maximum_stack_size: u8,
    pub dummy1: u8 = 0,
    pub dummy2: u8 = 0,
    pub dummy3: u8 = 0,
    pub size_upvalues: i32,
    pub size_k: i32,
    pub size_code: i32,
    pub size_line_info: i32,
    pub size_p: i32,
    pub size_local_variables: i32,
    pub size_absolute_line_info: i32,
    pub line_defined: i32,
    pub last_line_defined: i32,
    pub k: *mut TValue,
    pub code: *mut u32,
    pub p: *mut *mut Prototype,
    pub upvalues: *mut UpValueDescription,
    pub line_info: *mut i8,
    pub absolute_line_info: *mut AbsoluteLineInfo,
    pub local_variables: *mut LocalVariable,
    pub source: *mut TString,
    pub gc_list: *mut Object,
}
impl TObject for Prototype {
    fn get_marked(&self) -> u8 {
        self.marked
    }
    fn set_marked(&mut self, marked_: u8) {
        self.marked = marked_;
    }
    fn set_tag(&mut self, tag: u8) {
        self.tag = tag;
    }
    fn set_collectable(&mut self) {
        self.set_tag(set_collectable(self.get_tag()));
    }
    fn is_collectable(&self) -> bool {
        return is_collectable(self.get_tag());
    }
    fn get_tag(&self) -> u8 {
        self.tag
    }
    fn get_tag_type(&self) -> u8 {
        get_tag_type(self.get_tag())
    }
    fn get_tag_variant(&self) -> u8 {
        get_tag_variant(self.get_tag())
    }
    fn get_class_name(&mut self) -> String {
        "prototype".to_string()
    }
    fn get_metatable(&mut self) -> *mut Table {
        std::ptr::null_mut()
    }
}
impl Prototype {
    pub unsafe extern "C" fn free_prototype(&mut self, state: *mut State) {
        unsafe {
            (*state).free_memory(
                self.code as *mut libc::c_void,
                (self.size_code as u64).wrapping_mul(::core::mem::size_of::<u32>() as u64),
            );
            (*state).free_memory(
                self.p as *mut libc::c_void,
                (self.size_p as u64).wrapping_mul(::core::mem::size_of::<*mut Prototype>() as u64),
            );
            (*state).free_memory(
                self.k as *mut libc::c_void,
                (self.size_k as u64).wrapping_mul(::core::mem::size_of::<TValue>() as u64),
            );
            (*state).free_memory(
                self.line_info as *mut libc::c_void,
                (self.size_line_info as u64).wrapping_mul(::core::mem::size_of::<i8>() as u64),
            );
            (*state).free_memory(
                self.absolute_line_info as *mut libc::c_void,
                (self.size_absolute_line_info as u64)
                    .wrapping_mul(::core::mem::size_of::<AbsoluteLineInfo>() as u64),
            );
            (*state).free_memory(
                self.local_variables as *mut libc::c_void,
                (self.size_local_variables as u64)
                    .wrapping_mul(::core::mem::size_of::<LocalVariable>() as u64),
            );
            (*state).free_memory(
                self.upvalues as *mut libc::c_void,
                (self.size_upvalues as u64)
                    .wrapping_mul(::core::mem::size_of::<UpValueDescription>() as u64),
            );
            (*state).free_memory(
                self as *mut Prototype as *mut libc::c_void,
                ::core::mem::size_of::<Prototype>() as u64,
            );
        }
    }
}
pub unsafe extern "C" fn getbaseline(
    f: *const Prototype,
    program_counter: i32,
    basepc: *mut i32,
) -> i32 {
    unsafe {
        if (*f).size_absolute_line_info == 0
            || program_counter < (*((*f).absolute_line_info).offset(0 as isize)).program_counter
        {
            *basepc = -1;
            return (*f).line_defined;
        } else {
            let mut i: i32 = (program_counter as u32)
                .wrapping_div(128u32)
                .wrapping_sub(1u32) as i32;
            while (i + 1) < (*f).size_absolute_line_info
                && program_counter
                    >= (*((*f).absolute_line_info).offset((i + 1) as isize)).program_counter
            {
                i += 1;
            }
            *basepc = (*((*f).absolute_line_info).offset(i as isize)).program_counter;
            return (*((*f).absolute_line_info).offset(i as isize)).line;
        };
    }
}
pub unsafe extern "C" fn luag_getfuncline(f: *const Prototype, program_counter: i32) -> i32 {
    unsafe {
        if ((*f).line_info).is_null() {
            return -1;
        } else {
            let mut basepc: i32 = 0;
            let mut baseline: i32 = getbaseline(f, program_counter, &mut basepc);
            loop {
                let fresh8 = basepc;
                basepc = basepc + 1;
                if !(fresh8 < program_counter) {
                    break;
                }
                baseline += *((*f).line_info).offset(basepc as isize) as i32;
            }
            return baseline;
        };
    }
}
pub unsafe extern "C" fn upvalname(p: *const Prototype, uv: i32) -> *const i8 {
    unsafe {
        let s: *mut TString = (*((*p).upvalues).offset(uv as isize)).name;
        if s.is_null() {
            return b"?\0" as *const u8 as *const i8;
        } else {
            return (*s).get_contents();
        };
    }
}
pub unsafe extern "C" fn nextline(
    p: *const Prototype,
    currentline: i32,
    program_counter: i32,
) -> i32 {
    unsafe {
        if *((*p).line_info).offset(program_counter as isize) as i32 != -(0x80 as i32) {
            return currentline + *((*p).line_info).offset(program_counter as isize) as i32;
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
            [(*((*p).code).offset(lastpc as isize) >> 0 & !(!(0) << 7) << 0) as usize]
            as i32
            & 1 << 7
            != 0
        {
            lastpc -= 1;
        }
        let mut program_counter: i32 = 0;
        while program_counter < lastpc {
            let i: u32 = *((*p).code).offset(program_counter as isize);
            let op: u32 = (i >> 0 & !(!(0) << 7) << 0) as u32;
            let a: i32 = (i >> 0 + 7 & !(!(0) << 8) << 0) as i32;
            let change: i32;
            match op as u32 {
                8 => {
                    let b: i32 = (i >> 0 + 7 + 8 + 1 & !(!(0) << 8) << 0) as i32;
                    change = (a <= reg && reg <= a + b) as i32;
                }
                76 => {
                    change = (reg >= a + 2) as i32;
                }
                68 | 69 => {
                    change = (reg >= a) as i32;
                }
                56 => {
                    let b_0: i32 = (i >> 0 + 7 & !(!(0u32) << 8 + 8 + 1 + 8) << 0) as i32
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
                setreg = filterpc(program_counter, jmptarget);
            }
            program_counter += 1;
        }
        return setreg;
    }
}
pub unsafe extern "C" fn kname(p: *const Prototype, index: i32, name: *mut *const i8) -> *const i8 {
    unsafe {
        let kvalue: *mut TValue = &mut *((*p).k).offset(index as isize) as *mut TValue;
        if get_tag_type((*kvalue).get_tag()) == TAG_TYPE_STRING {
            *name = (*((*kvalue).value.object as *mut TString)).get_contents();
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
            let i: u32 = *((*p).code).offset(program_counter as isize);
            let op: u32 = (i >> 0 & !(!(0u32) << 7) << 0) as u32;
            match op as u32 {
                0 => {
                    let b: i32 = (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32;
                    if b < (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32 {
                        return basicgetobjname(p, ppc, b, name);
                    }
                }
                9 => {
                    *name = upvalname(p, (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32);
                    return STRING_UPVALUE.as_ptr();
                }
                3 => {
                    return kname(
                        p,
                        (i >> 0 + 7 + 8 & !(!(0u32) << 8 + 8 + 1) << 0) as i32,
                        name,
                    );
                }
                4 => {
                    return kname(
                        p,
                        (*((*p).code).offset((program_counter + 1) as isize) >> 0 + 7
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
        if !(!what.is_null() && *what as i32 == 'c' as i32) {
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
        let c: i32 = (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0) as i32;
        if (i >> 0 + 7 + 8 & !(!(0u32) << 1) << 0) as i32 != 0 {
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
        let t: i32 = (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32;
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
            let i: u32 = *((*p).code).offset(lastpc as isize);
            let op: u32 = (i >> 0 & !(!(0u32) << 7) << 0) as u32;
            match op as u32 {
                11 => {
                    let k: i32 = (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0) as i32;
                    kname(p, k, name);
                    return is_environment(p, lastpc, i, 1);
                }
                12 => {
                    let k_0: i32 = (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0) as i32;
                    rname(p, lastpc, k_0, name);
                    return is_environment(p, lastpc, i, 0);
                }
                13 => {
                    *name = b"integer index\0" as *const u8 as *const i8;
                    return b"field\0" as *const u8 as *const i8;
                }
                14 => {
                    let k_1: i32 = (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0) as i32;
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
    state: *mut State,
    p: *const Prototype,
    program_counter: i32,
    name: *mut *const i8,
) -> *const i8 {
    unsafe {
        let tm: u32;
        let i: u32 = *((*p).code).offset(program_counter as isize);
        match (i >> 0 & !(!(0u32) << 7) << 0) as u32 {
            OP_CALL | OP_TAILCALL => {
                return getobjname(
                    p,
                    program_counter,
                    (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32,
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
                tm = (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0) as u32;
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
        *name = ((*(*(*state).global).tm_name[tm as usize]).get_contents2())
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
        if ((*p).line_info).is_null() {
            return 0;
        }
        if newpc - old_program_counter < 128 as i32 / 2 {
            let mut delta: i32 = 0;
            let mut program_counter: i32 = old_program_counter;
            loop {
                program_counter += 1;
                let line_info: i32 = *((*p).line_info).offset(program_counter as isize) as i32;
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
pub unsafe extern "C" fn traverseproto(g: *mut Global, f: *mut Prototype) -> u64 {
    unsafe {
        if !((*f).source).is_null() {
            if (*(*f).source).get_marked() & (1 << 3 | 1 << 4) != 0 {
                reallymarkobject(g, &mut (*((*f).source as *mut Object)));
            }
        }
        let mut i: u64 = 0;
        while i < (*f).size_k as u64 {
            if ((*((*f).k).offset(i as isize)).is_collectable())
                && (*(*((*f).k).offset(i as isize)).value.object).get_marked() & (1 << 3 | 1 << 4)
                    != 0
            {
                reallymarkobject(g, (*((*f).k).offset(i as isize)).value.object);
            }
            i += 1;
        }
        i = 0;
        while i < (*f).size_upvalues as u64 {
            if !((*((*f).upvalues).offset(i as isize)).name).is_null() {
                if (*(*((*f).upvalues).offset(i as isize)).name).get_marked() & (1 << 3 | 1 << 4)
                    != 0
                {
                    reallymarkobject(
                        g,
                        &mut (*((*((*f).upvalues).offset(i as isize)).name as *mut Object)),
                    );
                }
            }
            i += 1;
        }
        i = 0;
        while i < (*f).size_p as u64 {
            if !(*((*f).p).offset(i as isize)).is_null() {
                if (**((*f).p).offset(i as isize)).get_marked() & (1 << 3 | 1 << 4) != 0 {
                    reallymarkobject(
                        g,
                        &mut (*(*((*f).p).offset(i as isize) as *mut Object)),
                    );
                }
            }
            i += 1;
        }
        i = 0;
        while i < (*f).size_local_variables as u64 {
            if !((*((*f).local_variables).offset(i as isize)).variable_name).is_null() {
                if (*(*((*f).local_variables).offset(i as isize)).variable_name).get_marked()
                    & (1 << 3 | 1 << 4)
                    != 0
                {
                    reallymarkobject(
                        g,
                        &mut (*((*((*f).local_variables).offset(i as isize)).variable_name
                            as *mut Object)),
                    );
                }
            }
            i += 1;
        }
        return (1 + (*f).size_k + (*f).size_upvalues + (*f).size_p + (*f).size_local_variables) as u64
    }
}
pub unsafe extern "C" fn luaf_newproto(state: *mut State) -> *mut Prototype {
    unsafe {
        let o: *mut Object = luac_newobj(
            state,
            TAG_TYPE_PROTOTYPE,
            ::core::mem::size_of::<Prototype>() as u64,
        );
        let f: *mut Prototype = &mut (*(o as *mut Prototype));
        (*f).k = std::ptr::null_mut();
        (*f).size_k = 0;
        (*f).p = std::ptr::null_mut();
        (*f).size_p = 0;
        (*f).code = std::ptr::null_mut();
        (*f).size_code = 0;
        (*f).line_info = std::ptr::null_mut();
        (*f).size_line_info = 0;
        (*f).absolute_line_info = std::ptr::null_mut();
        (*f).size_absolute_line_info = 0;
        (*f).upvalues = std::ptr::null_mut();
        (*f).size_upvalues = 0;
        (*f).count_parameters = 0;
        (*f).is_variable_arguments = false;
        (*f).maximum_stack_size = 0;
        (*f).local_variables = std::ptr::null_mut();
        (*f).size_local_variables = 0;
        (*f).line_defined = 0;
        (*f).last_line_defined = 0;
        (*f).source = std::ptr::null_mut();
        return f;
    }
}
pub unsafe extern "C" fn luaf_getlocalname(
    f: *const Prototype,
    mut local_number: i32,
    program_counter: i32,
) -> *const i8 {
    unsafe {
        let mut i: i32;
        i = 0;
        while i < (*f).size_local_variables
            && (*((*f).local_variables).offset(i as isize)).start_program_counter <= program_counter
        {
            if program_counter < (*((*f).local_variables).offset(i as isize)).end_program_counter {
                local_number -= 1;
                if local_number == 0 {
                    return (*(*((*f).local_variables).offset(i as isize)).variable_name)
                        .get_contents();
                }
            }
            i += 1;
        }
        return std::ptr::null();
    }
}
