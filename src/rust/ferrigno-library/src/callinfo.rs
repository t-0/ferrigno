use crate::closure::*;
use crate::functions::*;
use crate::prototype::*;
use crate::state::*;
use crate::stkidrel::*;
use crate::tagtype::*;
use crate::tvalue::*;
use std::ptr::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CallInfo {
    pub callinfo_function: StkIdRel,
    pub callinfo_top: StkIdRel,
    pub callinfo_previous: *mut CallInfo,
    pub callinfo_next: *mut CallInfo,
    pub callinfo_u: CallInfoConstituentA,
    pub callinfo_u2: CallInfoConsistuentB,
    pub callinfo_count_results: i32,
    pub callinfo_callstatus: u16,
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
    pub callinfoconstituentb_funcidx: i32,
    pub callinfoconstituentb_nyield: i32,
    pub callinfoconstituentb_nres: i32,
    pub callinfoconstituentb_transferinfo: CallInfoConsistuentBTransferInfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CallInfoConsistuentBTransferInfo {
    pub callinfoconsistuentbtransferinfo_ftransfer: u16,
    pub callinfoconsistuentbtransferinfo_ntransfer: u16,
}
impl CallInfo {
    pub unsafe fn currentpc(callinfo: *mut CallInfo) -> i32 {
        unsafe {
            ((*callinfo).callinfo_u.l.saved_program_counter).offset_from(
                (*(*(*(*callinfo).callinfo_function.stkidrel_pointer).as_closure().unwrap())
                    .closure_payload
                    .closurepayload_lprototype)
                    .prototype_code
                    .vectort_pointer,
            ) as i32
                - 1
        }
    }
    pub unsafe fn getcurrentline(callinfo: *mut CallInfo) -> i32 {
        unsafe {
            luag_getfuncline(
                (*(*(*callinfo).callinfo_function.stkidrel_pointer).as_closure().unwrap())
                    .closure_payload
                    .closurepayload_lprototype,
                CallInfo::currentpc(callinfo),
            )
        }
    }
    pub unsafe fn settraps(mut callinfo: *mut CallInfo) {
        unsafe {
            loop {
                if callinfo.is_null() {
                    break;
                } else {
                    if (*callinfo).callinfo_callstatus as i32 & CALLSTATUS_LUA == 0 {
                        write_volatile(&mut (*callinfo).callinfo_u.l.trap as *mut i32, 1);
                    }
                    callinfo = (*callinfo).callinfo_previous;
                }
            }
        }
    }
    pub unsafe fn luag_findlocal(state: *mut State, callinfo: *mut CallInfo, n: i32, position: *mut *mut TValue) -> *const i8 {
        unsafe {
            let base: *mut TValue = ((*callinfo).callinfo_function.stkidrel_pointer).add(1);
            let mut name: *const i8 = null();
            if (*callinfo).callinfo_callstatus as i32 & CALLSTATUS_LUA == 0 {
                if n < 0 {
                    return CallInfo::findvararg(callinfo, n, position);
                } else {
                    name = luaf_getlocalname(
                        (*(*(*callinfo).callinfo_function.stkidrel_pointer).as_closure().unwrap())
                            .closure_payload
                            .closurepayload_lprototype,
                        n,
                        CallInfo::currentpc(callinfo),
                    );
                }
            }
            if name.is_null() {
                let limit: *mut TValue = if callinfo == (*state).interpreter_callinfo {
                    (*state).interpreter_top.stkidrel_pointer
                } else {
                    (*(*callinfo).callinfo_next).callinfo_function.stkidrel_pointer
                };
                if limit.offset_from(base) as i64 >= n as i64 && n > 0 {
                    name = if (*callinfo).callinfo_callstatus as i32 & CALLSTATUS_LUA == 0 {
                        c"(temporary)".as_ptr()
                    } else {
                        c"(C temporary)".as_ptr()
                    };
                } else {
                    return null();
                }
            }
            if !position.is_null() {
                *position = base.add((n - 1) as usize);
            }
            name
        }
    }
    pub unsafe fn findvararg(callinfo: *mut CallInfo, n: i32, position: *mut *mut TValue) -> *const i8 {
        unsafe {
            if (*(*(*(*callinfo).callinfo_function.stkidrel_pointer).as_closure().unwrap())
                .closure_payload
                .closurepayload_lprototype)
                .prototype_isvariablearguments
            {
                let nextra = (*callinfo).callinfo_u.l.count_extra_arguments;
                if n >= -nextra {
                    *position = ((*callinfo).callinfo_function.stkidrel_pointer)
                        .sub(nextra as usize)
                        .sub((n + 1) as usize);
                    return c"(vararg)".as_ptr();
                }
            }
            null()
        }
    }
    pub unsafe fn getfuncname(state: *mut State, callinfo: *mut CallInfo, name: *mut *const i8) -> *const i8 {
        unsafe {
            if !callinfo.is_null() && (*callinfo).callinfo_callstatus as i32 & CALLSTATUS_TAIL == 0 {
                CallInfo::funcnamefromcall(state, (*callinfo).callinfo_previous, name)
            } else {
                null()
            }
        }
    }
    pub unsafe fn funcnamefromcall(state: *mut State, callinfo: *mut CallInfo, name: *mut *const i8) -> *const i8 {
        unsafe {
            if (*callinfo).callinfo_callstatus as i32 & CALLSTATUS_FRESH != 0 {
                *name = c"?".as_ptr();
                c"hook".as_ptr()
            } else if (*callinfo).callinfo_callstatus as i32 & CALLSTATUS_LEQ != 0 {
                *name = c"__gc".as_ptr();
                c"metamethod".as_ptr()
            } else if (*callinfo).callinfo_callstatus as i32 & CALLSTATUS_LUA == 0 {
                funcnamefromcode(
                    state,
                    (*(*(*callinfo).callinfo_function.stkidrel_pointer).as_closure().unwrap())
                        .closure_payload
                        .closurepayload_lprototype,
                    CallInfo::currentpc(callinfo),
                    name,
                )
            } else {
                null()
            }
        }
    }
    pub unsafe fn in_stack(callinfo: *mut CallInfo, tvalue: *const TValue) -> i32 {
        unsafe {
            let base: *mut TValue = ((*callinfo).callinfo_function.stkidrel_pointer).add(1);
            let mut position: i32 = 0;
            loop {
                if base.add(position as usize) < (*callinfo).callinfo_top.stkidrel_pointer {
                    if std::ptr::eq(tvalue, &(*base.add(position as usize))) {
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
            let closure: *mut Closure = (*(*callinfo).callinfo_function.stkidrel_pointer).as_closure().unwrap();
            for it in 0..(*closure).closure_count_upvalues {
                if std::ptr::eq(
                    (**((*closure).closure_upvalues)
                        .closureupvalue_lvalues
                        .as_mut_ptr()
                        .add(it as usize))
                    .upvalue_v
                    .upvaluea_p,
                    tvalue,
                ) {
                    *name = upvalname((*closure).closure_payload.closurepayload_lprototype, it as i32);
                    return TagType::STRING_UPVALUE;
                }
            }
            null()
        }
    }
}
