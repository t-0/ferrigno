use rlua::*;
use crate::closure::*;
use crate::functions::*;
use crate::interpreter::*;
use crate::prototype::*;
use crate::tvalue::*;
use crate::stkidrel::*;
use crate::tag::*;
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
pub unsafe fn currentpc(call_info: *mut CallInfo) -> i32 {
    unsafe {
        return ((*call_info).call_info_u.l.saved_program_counter).offset_from(
            (*(*((*(*call_info).call_info_function.stkidrel_pointer)
                .value
                .object as *mut Closure))
                .payload
                .l_prototype)
                .prototype_code
                .vectort_pointer,
        ) as i32
            - 1;
    }
}
pub unsafe fn getcurrentline(call_info: *mut CallInfo) -> i32 {
    unsafe {
        return luag_getfuncline(
            (*((*(*call_info).call_info_function.stkidrel_pointer)
                .value
                .object as *mut Closure))
                .payload
                .l_prototype,
            currentpc(call_info),
        );
    }
}
pub unsafe fn settraps(mut call_info: *mut CallInfo) {
    unsafe {
        loop {
            if call_info.is_null() {
                break;
            } else {
                if (*call_info).call_info_call_status as i32 & (1 << 1) == 0 {
                    ::core::ptr::write_volatile(&mut (*call_info).call_info_u.l.trap as *mut i32, 1);
                }
                call_info = (*call_info).call_info_previous;
            }
        }
    }
}
pub unsafe fn luag_findlocal(
    interpreter: *mut Interpreter,
    call_info: *mut CallInfo,
    n: i32,
    pos: *mut *mut TValue,
) -> *const i8 {
    unsafe {
        let base: *mut TValue = ((*call_info).call_info_function.stkidrel_pointer).offset(1 as isize);
        let mut name: *const i8 = null();
        if (*call_info).call_info_call_status as i32 & 1 << 1 == 0 {
            if n < 0 {
                return findvararg(call_info, n, pos);
            } else {
                name = luaf_getlocalname(
                    (*((*(*call_info).call_info_function.stkidrel_pointer)
                        .value
                        .object as *mut Closure))
                        .payload
                        .l_prototype,
                    n,
                    currentpc(call_info),
                );
            }
        }
        if name.is_null() {
            let limit: *mut TValue = if call_info == (*interpreter).call_info {
                (*interpreter).top.stkidrel_pointer
            } else {
                (*(*call_info).call_info_next).call_info_function.stkidrel_pointer
            };
            if limit.offset_from(base) as i64 >= n as i64 && n > 0 {
                name = if (*call_info).call_info_call_status as i32 & 1 << 1 == 0 {
                    make_cstring!("(temporary)")
                } else {
                    make_cstring!("(C temporary)")
                };
            } else {
                return null();
            }
        }
        if !pos.is_null() {
            *pos = base.offset((n - 1) as isize);
        }
        return name;
    }
}
pub unsafe fn findvararg(
    call_info: *mut CallInfo,
    n: i32,
    pos: *mut *mut TValue,
) -> *const i8 {
    unsafe {
        if (*(*((*(*call_info).call_info_function.stkidrel_pointer)
            .value
            .object as *mut Closure))
            .payload
            .l_prototype)
            .prototype_is_variable_arguments
        {
            let nextra = (*call_info).call_info_u.l.count_extra_arguments;
            if n >= -nextra {
                *pos = ((*call_info).call_info_function.stkidrel_pointer)
                    .offset(-nextra as isize)
                    .offset(-((n + 1) as isize));
                return make_cstring!("(vararg)");
            }
        }
        return null();
    }
}
pub unsafe fn getfuncname(
    interpreter: *mut Interpreter,
    call_info: *mut CallInfo,
    name: *mut *const i8,
) -> *const i8 {
    unsafe {
        if !call_info.is_null() && (*call_info).call_info_call_status as i32 & 1 << 5 == 0 {
            return funcnamefromcall(interpreter, (*call_info).call_info_previous, name);
        } else {
            return null();
        };
    }
}
pub unsafe fn funcnamefromcall(
    interpreter: *mut Interpreter,
    call_info: *mut CallInfo,
    name: *mut *const i8,
) -> *const i8 {
    unsafe {
        if (*call_info).call_info_call_status as i32 & 1 << 3 != 0 {
            *name = make_cstring!("?");
            return make_cstring!("hook");
        } else if (*call_info).call_info_call_status as i32 & 1 << 7 != 0 {
            *name = make_cstring!("__gc");
            return make_cstring!("metamethod");
        } else if (*call_info).call_info_call_status as i32 & 1 << 1 == 0 {
            return funcnamefromcode(
                interpreter,
                (*((*(*call_info).call_info_function.stkidrel_pointer)
                    .value
                    .object as *mut Closure))
                    .payload
                    .l_prototype,
                currentpc(call_info),
                name,
            );
        } else {
            return null();
        };
    }
}
pub unsafe fn in_stack(call_info: *mut CallInfo, tvalue: *const TValue) -> i32 {
    unsafe {
        let base: *mut TValue = ((*call_info).call_info_function.stkidrel_pointer).offset(1 as isize);
        let mut pos: i32 = 0;
        loop {
            if base.offset(pos as isize) < (*call_info).call_info_top.stkidrel_pointer {
                if tvalue
                    == &mut (*base.offset(pos as isize)) as *mut TValue as *const TValue
                {
                    return pos;
                } else {
                    pos += 1;
                }
            } else {
                return -1;
            }
        }
    }
}
pub unsafe fn getupvalname(
    call_info: *mut CallInfo,
    tvalue: *const TValue,
    name: *mut *const i8,
) -> *const i8 {
    unsafe {
        let c: *mut Closure = &mut (*((*(*call_info).call_info_function.stkidrel_pointer)
            .value
            .object as *mut Closure));
        for i in 0..(*c).count_upvalues {
            if (**((*c).upvalues).l_upvalues.as_mut_ptr().offset(i as isize))
                .v
                .p
                == tvalue as *mut TValue
            {
                *name = upvalname((*c).payload.l_prototype, i as i32);
                return STRING_UPVALUE;
            }
        }
        return null();
    }
}
