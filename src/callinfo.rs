use crate::closure::*;
use crate::functions::*;
use crate::interpreter::*;
use crate::prototype::*;
use crate::stkidrel::*;
use crate::tag::*;
use crate::tvalue::*;
use std::ptr::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CallInfo {
    pub call_info_function: StkIdRel,
    pub call_info_top: StkIdRel,
    pub call_info_previous: *mut CallInfo,
    pub call_info_next: *mut CallInfo,
    pub call_info_u: CallInfoConstituentA,
    pub call_info_u2: CallInfoConsistuentB,
    pub call_info_count_results: i32,
    pub call_info_call_status: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union CallInfoConstituentA {
    pub l: CallInfoConstituentAL,
    pub c: CallInfoConstituentAC,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CallInfoConstituentAL {
    pub saved_program_counter: *const u32,
    pub trap: i32,
    pub count_extra_arguments: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CallInfoConstituentAC {
    pub context_function: ContextFunction,
    pub old_error_function: i64,
    pub context: i64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union CallInfoConsistuentB {
    pub funcidx: i32,
    pub nyield: i32,
    pub nres: i32,
    pub transferinfo: CallInfoConsistuentBTransferInfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CallInfoConsistuentBTransferInfo {
    pub ftransfer: u16,
    pub ntransfer: u16,
}
pub unsafe fn currentpc(callinfo: *mut CallInfo) -> i32 {
    unsafe {
        ((*callinfo).call_info_u.l.saved_program_counter).offset_from((*(*((*(*callinfo).call_info_function.stkidrel_pointer).value.value_object as *mut Closure)).payload.l_prototype).prototype_code.vectort_pointer) as i32 - 1
    }
}
pub unsafe fn getcurrentline(callinfo: *mut CallInfo) -> i32 {
    unsafe {
        luag_getfuncline((*((*(*callinfo).call_info_function.stkidrel_pointer).value.value_object as *mut Closure)).payload.l_prototype, currentpc(callinfo))
    }
}
pub unsafe fn settraps(mut callinfo: *mut CallInfo) {
    unsafe {
        loop {
            if callinfo.is_null() {
                break;
            } else {
                if (*callinfo).call_info_call_status as i32 & (1 << 1) == 0 {
                    write_volatile(&mut (*callinfo).call_info_u.l.trap as *mut i32, 1);
                }
                callinfo = (*callinfo).call_info_previous;
            }
        }
    }
}
pub unsafe fn luag_findlocal(interpreter: *mut Interpreter, callinfo: *mut CallInfo, n: i32, position: *mut *mut TValue) -> *const i8 {
    unsafe {
        let base: *mut TValue = ((*callinfo).call_info_function.stkidrel_pointer).offset(1 as isize);
        let mut name: *const i8 = null();
        if (*callinfo).call_info_call_status as i32 & 1 << 1 == 0 {
            if n < 0 {
                return findvararg(callinfo, n, position);
            } else {
                name = luaf_getlocalname((*((*(*callinfo).call_info_function.stkidrel_pointer).value.value_object as *mut Closure)).payload.l_prototype, n, currentpc(callinfo));
            }
        }
        if name.is_null() {
            let limit: *mut TValue = if callinfo == (*interpreter).callinfo {
                (*interpreter).top.stkidrel_pointer
            } else {
                (*(*callinfo).call_info_next).call_info_function.stkidrel_pointer
            };
            if limit.offset_from(base) as i64 >= n as i64 && n > 0 {
                name = if (*callinfo).call_info_call_status as i32 & 1 << 1 == 0 { c"(temporary)".as_ptr() } else { c"(C temporary)".as_ptr() };
            } else {
                return null();
            }
        }
        if !position.is_null() {
            *position = base.offset((n - 1) as isize);
        }
        name
    }
}
pub unsafe fn findvararg(callinfo: *mut CallInfo, n: i32, position: *mut *mut TValue) -> *const i8 {
    unsafe {
        if (*(*((*(*callinfo).call_info_function.stkidrel_pointer).value.value_object as *mut Closure)).payload.l_prototype).prototype_is_variable_arguments {
            let nextra = (*callinfo).call_info_u.l.count_extra_arguments;
            if n >= -nextra {
                *position = ((*callinfo).call_info_function.stkidrel_pointer).offset(-nextra as isize).offset(-((n + 1) as isize));
                return c"(vararg)".as_ptr();
            }
        }
        null()
    }
}
pub unsafe fn getfuncname(interpreter: *mut Interpreter, callinfo: *mut CallInfo, name: *mut *const i8) -> *const i8 {
    unsafe {
        if !callinfo.is_null() && (*callinfo).call_info_call_status as i32 & 1 << 5 == 0 {
            return funcnamefromcall(interpreter, (*callinfo).call_info_previous, name);
        } else {
            return null();
        };
    }
}
pub unsafe fn funcnamefromcall(interpreter: *mut Interpreter, callinfo: *mut CallInfo, name: *mut *const i8) -> *const i8 {
    unsafe {
        if (*callinfo).call_info_call_status as i32 & 1 << 3 != 0 {
            *name = c"?".as_ptr();
            return c"hook".as_ptr();
        } else if (*callinfo).call_info_call_status as i32 & 1 << 7 != 0 {
            *name = c"__gc".as_ptr();
            return c"metamethod".as_ptr();
        } else if (*callinfo).call_info_call_status as i32 & 1 << 1 == 0 {
            return funcnamefromcode(interpreter, (*((*(*callinfo).call_info_function.stkidrel_pointer).value.value_object as *mut Closure)).payload.l_prototype, currentpc(callinfo), name);
        } else {
            return null();
        };
    }
}
pub unsafe fn in_stack(callinfo: *mut CallInfo, tvalue: *const TValue) -> i32 {
    unsafe {
        let base: *mut TValue = ((*callinfo).call_info_function.stkidrel_pointer).offset(1 as isize);
        let mut position: i32 = 0;
        loop {
            if base.offset(position as isize) < (*callinfo).call_info_top.stkidrel_pointer {
                if tvalue == &mut (*base.offset(position as isize)) as *mut TValue as *const TValue {
                    return position;
                } else {
                    position += 1;
                }
            } else {
                return -1;
            }
        }
    }
}
pub unsafe fn getupvalname(callinfo: *mut CallInfo, tvalue: *const TValue, name: *mut *const i8) -> *const i8 {
    unsafe {
        let closure: *mut Closure = &mut (*((*(*callinfo).call_info_function.stkidrel_pointer).value.value_object as *mut Closure));
        for it in 0..(*closure).count_upvalues {
            if (**((*closure).upvalues).l_upvalues.as_mut_ptr().offset(it as isize)).v.p == tvalue as *mut TValue {
                *name = upvalname((*closure).payload.l_prototype, it as i32);
                return STRING_UPVALUE;
            }
        }
        null()
    }
}
