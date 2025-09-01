use crate::cclosure::*;
use crate::lclosure::*;
use crate::tag::*;
use crate::object::*;
use crate::state::*;
use crate::debuginfo::*;
use crate::value::*;
use crate::table::*;
use crate::callinfo::*;
use crate::tvalue::*;
use crate::prototype::*;
#[derive(Copy, Clone)]
pub union UClosure {
    pub c: CClosure,
    pub l: LClosure,
}
pub unsafe extern "C" fn collectvalidlines(state: *mut State, f: *mut UClosure) {
    unsafe {
        if !(!f.is_null() && (*f).c.get_tag() == TAG_VARIANT_CLOSURE_L) {
            (*(*state).top.p).value.set_tag(TAG_VARIANT_NIL_NIL);
            (*state).top.p = (*state).top.p.offset(1);
        } else {
            let p: *const Prototype = (*f).l.p;
            let mut currentline: i32 = (*p).line_defined;
            let table: *mut Table = luah_new(state);
            let io: *mut TValue = &mut (*(*state).top.p).value;
            let x_: *mut Table = table;
            (*io).value.object = &mut (*(x_ as *mut Object));
            (*io).set_tag(TAG_VARIANT_TABLE);
            (*io).set_collectable();
            (*state).top.p = (*state).top.p.offset(1);
            if !((*p).line_info).is_null() {
                let mut i: i32;
                let mut v: TValue = TValue {
                    value: Value {
                        object: std::ptr::null_mut(),
                    },
                    tag: 0,
                };
                v.set_tag(TAG_VARIANT_BOOLEAN_TRUE);
                if !(*p).is_variable_arguments {
                    i = 0;
                } else {
                    currentline = nextline(p, currentline, 0);
                    i = 1;
                }
                while i < (*p).size_line_info {
                    currentline = nextline(p, currentline, i);
                    luah_setint(state, table, currentline as i64, &mut v);
                    i += 1;
                }
            }
        };
    }
}
pub unsafe extern "C" fn auxgetinfo(
    state: *mut State,
    mut what: *const i8,
    ar: *mut DebugInfo,
    f: *mut UClosure,
    call_info: *mut CallInfo,
) -> i32 {
    unsafe {
        let mut status: i32 = 1;
        while *what != 0 {
            match *what as i32 {
                83 => {
                    funcinfo(ar, f);
                }
                108 => {
                    (*ar).currentline =
                        if !call_info.is_null() && (*call_info).call_status as i32 & 1 << 1 == 0 {
                            getcurrentline(call_info)
                        } else {
                            -1
                        };
                }
                117 => {
                    (*ar).nups = (if f.is_null() {
                        0
                    } else {
                        (*f).c.count_upvalues as i32
                    }) as u8;
                    if !(!f.is_null() && (*f).c.get_tag() == TAG_VARIANT_CLOSURE_L) {
                        (*ar).is_variable_arguments = true;
                        (*ar).nparams = 0;
                    } else {
                        (*ar).is_variable_arguments = (*(*f).l.p).is_variable_arguments;
                        (*ar).nparams = (*(*f).l.p).count_parameters;
                    }
                }
                116 => {
                    (*ar).is_tail_call = if !call_info.is_null() {
                        0 != ((*call_info).call_status as i32 & 1 << 5)
                    } else {
                        false
                    };
                }
                110 => {
                    (*ar).namewhat = getfuncname(state, call_info, &mut (*ar).name);
                    if ((*ar).namewhat).is_null() {
                        (*ar).namewhat = b"\0" as *const u8 as *const i8;
                        (*ar).name = std::ptr::null();
                    }
                }
                114 => {
                    if call_info.is_null() || (*call_info).call_status as i32 & 1 << 8 == 0 {
                        (*ar).ntransfer = 0;
                        (*ar).ftransfer = (*ar).ntransfer;
                    } else {
                        (*ar).ftransfer = (*call_info).u2.transferinfo.ftransfer;
                        (*ar).ntransfer = (*call_info).u2.transferinfo.ntransfer;
                    }
                }
                76 | 102 => {}
                _ => {
                    status = 0;
                }
            }
            what = what.offset(1);
        }
        return status;
    }
}
