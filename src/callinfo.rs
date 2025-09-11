use crate::functions::*;
use crate::stkidrel::*;
use crate::prototype::*;
use crate::tvalue::*;
use crate::closure::*;
use crate::tag::*;
use crate::interpreter::*;
use crate::stackvalue::*;
use std::ptr::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CallInfo {
    pub function: StkIdRel,
    pub top: StkIdRel,
    pub previous: *mut CallInfo,
    pub next: *mut CallInfo,
    pub u: CallInfoConstituentA,
    pub u2: CallInfoConsistuentB,
    pub count_results: i32,
    pub call_status: u16,
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
pub unsafe extern "C" fn currentpc(call_info: *mut CallInfo) -> i32 {
    unsafe {
        return ((*call_info).u.l.saved_program_counter).offset_from(
            (*(*((*(*call_info).function.stkidrel_pointer).tvalue.value.object as *mut Closure))
                .payload.l_prototype)
                .prototype_code.pointer,
        ) as i32
            - 1;
    }
}
pub unsafe extern "C" fn getcurrentline(call_info: *mut CallInfo) -> i32 {
    unsafe {
        return luag_getfuncline(
            (*((*(*call_info).function.stkidrel_pointer).tvalue.value.object as *mut Closure))
                .payload.l_prototype,
            currentpc(call_info),
        );
    }
}
pub unsafe extern "C" fn settraps(mut call_info: *mut CallInfo) {
    unsafe {
        loop {
            if call_info.is_null() {
                break;
            } else {
                if (*call_info).call_status as i32 & (1 << 1) == 0 {
                    ::core::ptr::write_volatile(&mut (*call_info).u.l.trap as *mut i32, 1);
                }
                call_info = (*call_info).previous;
            }
        }
    }
}
pub unsafe extern "C" fn luag_findlocal(
    interpreter: *mut Interpreter,
    call_info: *mut CallInfo,
    n: i32,
    pos: *mut StackValuePointer,
) -> *const libc::c_char {
    unsafe {
        let base: StackValuePointer = ((*call_info).function.stkidrel_pointer).offset(1 as isize);
        let mut name: *const libc::c_char = null();
        if (*call_info).call_status as i32 & 1 << 1 == 0 {
            if n < 0 {
                return findvararg(call_info, n, pos);
            } else {
                name = luaf_getlocalname(
                    (*((*(*call_info).function.stkidrel_pointer).tvalue.value.object as *mut Closure)).payload.l_prototype,
                    n,
                    currentpc(call_info),
                );
            }
        }
        if name.is_null() {
            let limit: StackValuePointer = if call_info == (*interpreter).call_info {
                (*interpreter).top.stkidrel_pointer
            } else {
                (*(*call_info).next).function.stkidrel_pointer
            };
            if limit.offset_from(base) as i64 >= n as i64 && n > 0 {
                name = if (*call_info).call_status as i32 & 1 << 1 == 0 {
                    b"(temporary)\0" as *const u8 as *const libc::c_char
                } else {
                    b"(C temporary)\0" as *const u8 as *const libc::c_char
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
pub unsafe extern "C" fn findvararg(
    call_info: *mut CallInfo,
    n: i32,
    pos: *mut StackValuePointer,
) -> *const libc::c_char {
    unsafe {
        if (*(*((*(*call_info).function.stkidrel_pointer).tvalue.value.object as *mut Closure))
            .payload.l_prototype)
            .prototype_is_variable_arguments
        {
            let nextra = (*call_info).u.l.count_extra_arguments;
            if n >= -nextra {
                *pos = ((*call_info).function.stkidrel_pointer)
                    .offset(-nextra as isize)
                    .offset(-((n + 1) as isize));
                return b"(vararg)\0" as *const u8 as *const libc::c_char;
            }
        }
        return null();
    }
}
pub unsafe extern "C" fn getfuncname(
    interpreter: *mut Interpreter,
    call_info: *mut CallInfo,
    name: *mut *const libc::c_char,
) -> *const libc::c_char {
    unsafe {
        if !call_info.is_null() && (*call_info).call_status as i32 & 1 << 5 == 0 {
            return funcnamefromcall(interpreter, (*call_info).previous, name);
        } else {
            return null();
        };
    }
}
pub unsafe extern "C" fn funcnamefromcall(
    interpreter: *mut Interpreter,
    call_info: *mut CallInfo,
    name: *mut *const libc::c_char,
) -> *const libc::c_char {
    unsafe {
        if (*call_info).call_status as i32 & 1 << 3 != 0 {
            *name = b"?\0" as *const u8 as *const libc::c_char;
            return b"hook\0" as *const u8 as *const libc::c_char;
        } else if (*call_info).call_status as i32 & 1 << 7 != 0 {
            *name = b"__gc\0" as *const u8 as *const libc::c_char;
            return b"metamethod\0" as *const u8 as *const libc::c_char;
        } else if (*call_info).call_status as i32 & 1 << 1 == 0 {
            return funcnamefromcode(
                interpreter,
                (*((*(*call_info).function.stkidrel_pointer).tvalue.value.object as *mut Closure))
                    .payload.l_prototype,
                currentpc(call_info),
                name,
            );
        } else {
            return null();
        };
    }
}
pub unsafe extern "C" fn in_stack(call_info: *mut CallInfo, tvalue: *const TValue) -> i32 {
    unsafe {
        let base: StackValuePointer = ((*call_info).function.stkidrel_pointer).offset(1 as isize);
        let mut pos: i32 = 0;
        loop {
            if base.offset(pos as isize) < (*call_info).top.stkidrel_pointer {
                if tvalue == &mut (*base.offset(pos as isize)).tvalue as *mut TValue as *const TValue {
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
pub unsafe extern "C" fn getupvalname(
    call_info: *mut CallInfo,
    tvalue: *const TValue,
    name: *mut *const libc::c_char,
) -> *const libc::c_char {
    unsafe {
        let c: *mut Closure =
            &mut (*((*(*call_info).function.stkidrel_pointer).tvalue.value.object as *mut Closure));
        for i in 0..(*c).count_upvalues {
            if (**((*c).upvalues).l_upvalues.as_mut_ptr().offset(i as isize)).v.p == tvalue as *mut TValue {
                *name = upvalname((*c).payload.l_prototype, i as i32);
                return STRING_UPVALUE.as_ptr();
            }
        }
        return null();
    }
}
