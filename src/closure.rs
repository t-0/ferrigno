use crate::tag::*;
use crate::object::*;
use crate::interpreter::*;
use crate::global::*;
use crate::debuginfo::*;
use crate::value::*;
use crate::table::*;
use crate::character::*;
use crate::callinfo::*;
use crate::tvalue::*;
use crate::prototype::*;
use crate::functions::*;
use crate::upvalue::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub union ClosureUpValue{
    pub c_tvalues: [TValue; 1],
    pub l_upvalues: [*mut UpValue; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union ClosurePayload {
    pub c_cfunction: CFunction,
    pub l_prototype: *mut Prototype,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Closure {
    pub next: *mut Object,
    pub tag: u8,
    pub marked: u8,
    //PROBLEM
    pub count_upvalues: u8,
    pub dummy1: u8,
    pub dummy2: u32,
    pub gc_list: *mut Object,
    pub payload: ClosurePayload,
    pub upvalues: ClosureUpValue,
}
impl TObject for Closure {
    fn get_tag(&self) -> u8 {
        return self.tag;
    }
    fn set_tag(&mut self, tag: u8) {
        self.tag = tag;
    }
    fn get_marked(&self) -> u8 {
        self.marked
    }
    fn set_marked(&mut self, marked_: u8) {
        self.marked = marked_;
    }
    fn get_class_name(&mut self) -> String {
        "closure".to_string()
    }
    fn get_metatable(&mut self) -> *mut Table {
        std::ptr::null_mut()
    }
}
impl Closure {
    pub unsafe fn free_closure(& mut self, state: * mut Interpreter) {
        unsafe {
            let size = match self.get_tag_variant() {
                TAG_VARIANT_CLOSURE_C => size_cclosure(self.count_upvalues as usize),
                TAG_VARIANT_CLOSURE_L => size_lclosure(self.count_upvalues as usize),
                _ => 0,
            };
            (*state).free_memory(
                self as * mut Closure as *mut libc::c_void,
                size,
            );
        }
    }
    pub unsafe extern "C" fn traversecclosure(global: *mut Global, closure: *mut Closure) -> u64 {
        unsafe {
            for i in 0..(*closure).count_upvalues {
                if ((*((*closure).upvalues).c_tvalues.as_mut_ptr().offset(i as isize)).is_collectable())
                    && (*(*((*closure).upvalues).c_tvalues.as_mut_ptr().offset(i as isize))
                        .value
                        .object)
                        .get_marked()
                        & (1 << 3 | 1 << 4)
                        != 0
                {
                    really_mark_object(
                        global,
                        (*((*closure).upvalues).c_tvalues.as_mut_ptr().offset(i as isize))
                            .value
                            .object,
                    );
                }
            }
            return 1 + (*closure).count_upvalues as u64;
        }
    }
    pub unsafe extern "C" fn traverselclosure(global: *mut Global, closure: *mut Closure) -> u64 {
        unsafe {
            if !((*closure).payload.l_prototype).is_null() {
                if (*(*closure).payload.l_prototype).get_marked() & (1 << 3 | 1 << 4) != 0 {
                    really_mark_object(global, &mut (*((*closure).payload.l_prototype as *mut Object)));
                }
            }
            for i in 0..(*closure).count_upvalues {
                let upvalue: *mut UpValue = *((*closure).upvalues).l_upvalues.as_mut_ptr().offset(i as isize);
                if !upvalue.is_null() {
                    if (*upvalue).get_marked() & (1 << 3 | 1 << 4) != 0 {
                        really_mark_object(global, &mut (*(upvalue as *mut Object)));
                    }
                }
            }
            return 1 + (*closure).count_upvalues as u64;
        }
    }
}
pub unsafe extern "C" fn collectvalidlines(state: *mut Interpreter, closure: *mut Closure) {
    unsafe {
        if !(!closure.is_null() && (*closure).get_tag() == TAG_VARIANT_CLOSURE_L) {
            (*(*state).top.stkidrel_pointer).tvalue.set_tag(TAG_VARIANT_NIL_NIL);
            (*state).top.stkidrel_pointer = (*state).top.stkidrel_pointer.offset(1);
        } else {
            let prototype: *const Prototype = (*closure).payload.l_prototype;
            let mut current_line: i32 = (*prototype).line_defined;
            let table: *mut Table = luah_new(state);
            let io: *mut TValue = &mut (*(*state).top.stkidrel_pointer).tvalue;
            (*io).value.object = &mut (*(table as *mut Object));
            (*io).set_tag(TAG_VARIANT_TABLE);
            (*io).set_collectable();
            (*state).top.stkidrel_pointer = (*state).top.stkidrel_pointer.offset(1);
            if !((*prototype).line_info).is_null() {
                let mut v: TValue = TValue {
                    value: Value {
                        object: std::ptr::null_mut(),
                    },
                    tag: 0,
                };
                v.set_tag(TAG_VARIANT_BOOLEAN_TRUE);
                let start: i32 = if !(*prototype).is_variable_arguments {
                    0
                } else {
                    current_line = nextline(prototype, current_line, 0);
                    1
                };
                for i in start..(*prototype).size_line_info {
                    current_line = nextline(prototype, current_line, i);
                    luah_setint(state, table, current_line as i64, &mut v);
                }
            }
        };
    }
}
pub unsafe extern "C" fn auxgetinfo(
    state: *mut Interpreter,
    mut what: *const i8,
    ar: *mut DebugInfo,
    closure: *mut Closure,
    call_info: *mut CallInfo,
) -> i32 {
    unsafe {
        let mut status: i32 = 1;
        while *what != 0 {
            match *what as i32 {
                CHARACTER_UPPER_S => {
                    funcinfo(ar, closure);
                },
                CHARACTER_LOWER_L => {
                    (*ar).currentline =
                        if !call_info.is_null() && (*call_info).call_status as i32 & 1 << 1 == 0 {
                            getcurrentline(call_info)
                        } else {
                            -1
                        };
                },
                CHARACTER_LOWER_U => {
                    (*ar).nups = (if closure.is_null() {
                        0
                    } else {
                        (*closure).count_upvalues as i32
                    }) as u8;
                    if !(!closure.is_null() && (*closure).get_tag() == TAG_VARIANT_CLOSURE_L) {
                        (*ar).is_variable_arguments = true;
                        (*ar).nparams = 0;
                    } else {
                        (*ar).is_variable_arguments = (*(*closure).payload.l_prototype).is_variable_arguments;
                        (*ar).nparams = (*(*closure).payload.l_prototype).count_parameters;
                    }
                },
                CHARACTER_LOWER_T => {
                    (*ar).is_tail_call = if !call_info.is_null() {
                        0 != ((*call_info).call_status as i32 & 1 << 5)
                    } else {
                        false
                    };
                },
                CHARACTER_LOWER_N => {
                    (*ar).namewhat = getfuncname(state, call_info, &mut (*ar).name);
                    if ((*ar).namewhat).is_null() {
                        (*ar).namewhat = b"\0" as *const u8 as *const i8;
                        (*ar).name = std::ptr::null();
                    }
                },
                CHARACTER_LOWER_R => {
                    if call_info.is_null() || (*call_info).call_status as i32 & 1 << 8 == 0 {
                        (*ar).ntransfer = 0;
                        (*ar).ftransfer = (*ar).ntransfer;
                    } else {
                        (*ar).ftransfer = (*call_info).u2.transferinfo.ftransfer;
                        (*ar).ntransfer = (*call_info).u2.transferinfo.ntransfer;
                    }
                },
                CHARACTER_UPPER_L | CHARACTER_LOWER_F => {},
                _ => {
                    status = 0;
                },
            }
            what = what.offset(1);
        }
        return status;
    }
}
pub unsafe extern "C" fn size_cclosure(count_upvalues: usize) -> usize {
    32usize + ::core::mem::size_of::<TValue>() * count_upvalues
}
pub unsafe extern "C" fn size_lclosure(count_upvalues: usize) -> usize {
    32usize + ::core::mem::size_of::<*mut TValue>() * count_upvalues
}
pub unsafe extern "C" fn luaf_newcclosure(state: *mut Interpreter, count_upvalues: i32) -> *mut Closure {
    unsafe {
        let object: *mut Object = luac_newobj(
            state,
            TAG_VARIANT_CLOSURE_C,
            size_cclosure(count_upvalues as usize),
        );
        let ret: *mut Closure = &mut (*(object as *mut Closure));
        (*ret).count_upvalues = count_upvalues as u8;
        return ret;
    }
}
pub unsafe extern "C" fn luaf_newlclosure(state: *mut Interpreter, mut count_upvalues: i32) -> *mut Closure {
    unsafe {
        let object: *mut Object = luac_newobj(
            state,
            TAG_VARIANT_CLOSURE_L,
            size_lclosure(count_upvalues as usize),
        );
        let ret: *mut Closure = &mut (*(object as *mut Closure));
        (*ret).payload.l_prototype = std::ptr::null_mut();
        (*ret).count_upvalues = count_upvalues as u8;
        loop {
            let fresh = count_upvalues;
            count_upvalues = count_upvalues - 1;
            if fresh == 0 {
                break;
            }
            let ref mut fresh18 = *((*ret).upvalues).l_upvalues.as_mut_ptr().offset(count_upvalues as isize);
            *fresh18 = std::ptr::null_mut();
        }
        return ret;
    }
}
pub unsafe extern "C" fn luaf_initupvals(state: *mut Interpreter, cl: *mut Closure) {
    unsafe {
        for i in 0..(*cl).count_upvalues {
            let object: *mut Object = luac_newobj(
                state,
                TAG_TYPE_UPVALUE,
                ::core::mem::size_of::<UpValue>(),
            );
            let upvalue: *mut UpValue = &mut (*(object as *mut UpValue));
            (*upvalue).v.p = &mut (*upvalue).u.value;
            (*(*upvalue).v.p).set_tag(TAG_VARIANT_NIL_NIL);
            let ref mut fresh = *((*cl).upvalues).l_upvalues.as_mut_ptr().offset(i as isize);
            *fresh = upvalue;
            if (*cl).get_marked() & 1 << 5 != 0 && (*upvalue).get_marked() & (1 << 3 | 1 << 4) != 0 {
                luac_barrier_(
                    state,
                    &mut (*(cl as *mut Object)),
                    &mut (*(upvalue as *mut Object)),
                );
            }
        }
    }
}
