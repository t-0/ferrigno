use crate::closure::*;
use crate::functions::*;
use crate::interpreter::*;
use crate::prototype::*;
use crate::stkidrel::*;
use crate::tag::*;
use crate::tvalue::*;
use rlua::*;
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
pub unsafe fn currentpc(ci: *mut CallInfo) -> i32 {
    unsafe {
        return ((*ci).call_info_u.l.saved_program_counter).offset_from((*(*((*(*ci).call_info_function.stkidrel_pointer).value.object as *mut Closure)).payload.l_prototype).prototype_code.vectort_pointer) as i32 - 1;
    }
}
pub unsafe fn getcurrentline(ci: *mut CallInfo) -> i32 {
    unsafe {
        return luag_getfuncline((*((*(*ci).call_info_function.stkidrel_pointer).value.object as *mut Closure)).payload.l_prototype, currentpc(ci));
    }
}
pub unsafe fn settraps(mut ci: *mut CallInfo) {
    unsafe {
        loop {
            if ci.is_null() {
                break;
            } else {
                if (*ci).call_info_call_status as i32 & (1 << 1) == 0 {
                    write_volatile(&mut (*ci).call_info_u.l.trap as *mut i32, 1);
                }
                ci = (*ci).call_info_previous;
            }
        }
    }
}
pub unsafe fn luag_findlocal(interpreter: *mut Interpreter, ci: *mut CallInfo, n: i32, position: *mut *mut TValue) -> *const i8 {
    unsafe {
        let base: *mut TValue = ((*ci).call_info_function.stkidrel_pointer).offset(1 as isize);
        let mut name: *const i8 = null();
        if (*ci).call_info_call_status as i32 & 1 << 1 == 0 {
            if n < 0 {
                return findvararg(ci, n, position);
            } else {
                name = luaf_getlocalname((*((*(*ci).call_info_function.stkidrel_pointer).value.object as *mut Closure)).payload.l_prototype, n, currentpc(ci));
            }
        }
        if name.is_null() {
            let limit: *mut TValue = if ci == (*interpreter).ci {
                (*interpreter).top.stkidrel_pointer
            } else {
                (*(*ci).call_info_next).call_info_function.stkidrel_pointer
            };
            if limit.offset_from(base) as i64 >= n as i64 && n > 0 {
                name = if (*ci).call_info_call_status as i32 & 1 << 1 == 0 { make_cstring!("(temporary)") } else { make_cstring!("(C temporary)") };
            } else {
                return null();
            }
        }
        if !position.is_null() {
            *position = base.offset((n - 1) as isize);
        }
        return name;
    }
}
pub unsafe fn findvararg(ci: *mut CallInfo, n: i32, position: *mut *mut TValue) -> *const i8 {
    unsafe {
        if (*(*((*(*ci).call_info_function.stkidrel_pointer).value.object as *mut Closure)).payload.l_prototype).prototype_is_variable_arguments {
            let nextra = (*ci).call_info_u.l.count_extra_arguments;
            if n >= -nextra {
                *position = ((*ci).call_info_function.stkidrel_pointer).offset(-nextra as isize).offset(-((n + 1) as isize));
                return make_cstring!("(vararg)");
            }
        }
        return null();
    }
}
pub unsafe fn getfuncname(interpreter: *mut Interpreter, ci: *mut CallInfo, name: *mut *const i8) -> *const i8 {
    unsafe {
        if !ci.is_null() && (*ci).call_info_call_status as i32 & 1 << 5 == 0 {
            return funcnamefromcall(interpreter, (*ci).call_info_previous, name);
        } else {
            return null();
        };
    }
}
pub unsafe fn funcnamefromcall(interpreter: *mut Interpreter, ci: *mut CallInfo, name: *mut *const i8) -> *const i8 {
    unsafe {
        if (*ci).call_info_call_status as i32 & 1 << 3 != 0 {
            *name = make_cstring!("?");
            return make_cstring!("hook");
        } else if (*ci).call_info_call_status as i32 & 1 << 7 != 0 {
            *name = make_cstring!("__gc");
            return make_cstring!("metamethod");
        } else if (*ci).call_info_call_status as i32 & 1 << 1 == 0 {
            return funcnamefromcode(interpreter, (*((*(*ci).call_info_function.stkidrel_pointer).value.object as *mut Closure)).payload.l_prototype, currentpc(ci), name);
        } else {
            return null();
        };
    }
}
pub unsafe fn in_stack(ci: *mut CallInfo, tvalue: *const TValue) -> i32 {
    unsafe {
        let base: *mut TValue = ((*ci).call_info_function.stkidrel_pointer).offset(1 as isize);
        let mut position: i32 = 0;
        loop {
            if base.offset(position as isize) < (*ci).call_info_top.stkidrel_pointer {
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
pub unsafe fn getupvalname(ci: *mut CallInfo, tvalue: *const TValue, name: *mut *const i8) -> *const i8 {
    unsafe {
        let c: *mut Closure = &mut (*((*(*ci).call_info_function.stkidrel_pointer).value.object as *mut Closure));
        for i in 0..(*c).count_upvalues {
            if (**((*c).upvalues).l_upvalues.as_mut_ptr().offset(i as isize)).v.p == tvalue as *mut TValue {
                *name = upvalname((*c).payload.l_prototype, i as i32);
                return STRING_UPVALUE;
            }
        }
        return null();
    }
}
