use crate::functions::*;
use crate::stkidrel::*;
use crate::prototype::*;
use crate::tvalue::*;
use crate::lclosure::*;
use crate::onelua::*;
use crate::state::*;
use crate::stackvalue::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CallInfo {
    pub function: StkIdRel,
    pub top: StkIdRel,
    pub previous: *mut CallInfo,
    pub next: *mut CallInfo,
    pub u: CallInfoConstituentA,
    pub u2: CallInfoConsistuentB,
    pub count_results: i16,
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
    pub k: ContextFunction,
    pub old_error_function: i64,
    pub ctx: i64,
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
            (*(*((*(*call_info).function.p).value.value.object as *mut LClosure))
                .p)
                .code,
        ) as i32
            - 1;
    }
}
pub unsafe extern "C" fn getcurrentline(call_info: *mut CallInfo) -> i32 {
    unsafe {
        return luag_getfuncline(
            (*((*(*call_info).function.p).value.value.object as *mut LClosure))
                .p,
            currentpc(call_info),
        );
    }
}
pub unsafe extern "C" fn settraps(mut call_info: *mut CallInfo) {
    unsafe {
        while !call_info.is_null() {
            if (*call_info).call_status as i32 & 1 << 1 == 0 {
                ::core::ptr::write_volatile(&mut (*call_info).u.l.trap as *mut i32, 1);
            }
            call_info = (*call_info).previous;
        }
    }
}
pub unsafe extern "C" fn luag_findlocal(
    state: *mut State,
    call_info: *mut CallInfo,
    n: i32,
    pos: *mut StkId,
) -> *const i8 {
    unsafe {
        let base: StkId = ((*call_info).function.p).offset(1 as isize);
        let mut name: *const i8 = std::ptr::null();
        if (*call_info).call_status as i32 & 1 << 1 == 0 {
            if n < 0 {
                return findvararg(call_info, n, pos);
            } else {
                name = luaf_getlocalname(
                    (*((*(*call_info).function.p).value.value.object as *mut LClosure)).p,
                    n,
                    currentpc(call_info),
                );
            }
        }
        if name.is_null() {
            let limit: StkId = if call_info == (*state).call_info {
                (*state).top.p
            } else {
                (*(*call_info).next).function.p
            };
            if limit.offset_from(base) as i64 >= n as i64 && n > 0 {
                name = if (*call_info).call_status as i32 & 1 << 1 == 0 {
                    b"(temporary)\0" as *const u8 as *const i8
                } else {
                    b"(C temporary)\0" as *const u8 as *const i8
                };
            } else {
                return std::ptr::null();
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
    pos: *mut StkId,
) -> *const i8 {
    unsafe {
        if (*(*((*(*call_info).function.p).value.value.object as *mut LClosure))
            .p)
            .is_variable_arguments
        {
            let nextra: i32 = (*call_info).u.l.count_extra_arguments;
            if n >= -nextra {
                *pos = ((*call_info).function.p)
                    .offset(-(nextra as isize))
                    .offset(-((n + 1) as isize));
                return b"(vararg)\0" as *const u8 as *const i8;
            }
        }
        return std::ptr::null();
    }
}
pub unsafe extern "C" fn getfuncname(
    state: *mut State,
    call_info: *mut CallInfo,
    name: *mut *const i8,
) -> *const i8 {
    unsafe {
        if !call_info.is_null() && (*call_info).call_status as i32 & 1 << 5 == 0 {
            return funcnamefromcall(state, (*call_info).previous, name);
        } else {
            return std::ptr::null();
        };
    }
}
pub unsafe extern "C" fn funcnamefromcall(
    state: *mut State,
    call_info: *mut CallInfo,
    name: *mut *const i8,
) -> *const i8 {
    unsafe {
        if (*call_info).call_status as i32 & 1 << 3 != 0 {
            *name = b"?\0" as *const u8 as *const i8;
            return b"hook\0" as *const u8 as *const i8;
        } else if (*call_info).call_status as i32 & 1 << 7 != 0 {
            *name = b"__gc\0" as *const u8 as *const i8;
            return b"metamethod\0" as *const u8 as *const i8;
        } else if (*call_info).call_status as i32 & 1 << 1 == 0 {
            return funcnamefromcode(
                state,
                (*((*(*call_info).function.p).value.value.object as *mut LClosure))
                    .p,
                currentpc(call_info),
                name,
            );
        } else {
            return std::ptr::null();
        };
    }
}
pub unsafe extern "C" fn in_stack(call_info: *mut CallInfo, o: *const TValue) -> i32 {
    unsafe {
        let base: StkId = ((*call_info).function.p).offset(1 as isize);
        let mut pos: i32 = 0;
        while base.offset(pos as isize) < (*call_info).top.p {
            if o == &mut (*base.offset(pos as isize)).value as *mut TValue as *const TValue {
                return pos;
            }
            pos += 1;
        }
        return -1;
    }
}
pub unsafe extern "C" fn getupvalname(
    call_info: *mut CallInfo,
    o: *const TValue,
    name: *mut *const i8,
) -> *const i8 {
    unsafe {
        let c: *mut LClosure =
            &mut (*((*(*call_info).function.p).value.value.object as *mut LClosure));
        let mut i: i32;
        i = 0;
        while i < (*c).count_upvalues as i32 {
            if (**((*c).upvalues).as_mut_ptr().offset(i as isize)).v.p == o as *mut TValue {
                *name = upvalname((*c).p, i);
                return STRING_UPVALUE.as_ptr();
            }
            i += 1;
        }
        return std::ptr::null();
    }
}
