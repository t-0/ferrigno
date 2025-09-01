use crate::rawvalue::*;
use crate::tstring::*;
use crate::v::*;
use crate::tvalue::*;
use crate::object::*;
use crate::functionstate::*;
use crate::tag::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExpressionDescription {
    pub k: u32,
    pub u: RawValue,
    pub t: i32,
    pub f: i32,
}
pub unsafe extern "C" fn init_exp(e: *mut ExpressionDescription, k: u32, i: i32) {
    unsafe {
        (*e).t = -1;
        (*e).f = (*e).t;
        (*e).k = k;
        (*e).u.info = i;
    }
}
pub unsafe extern "C" fn codestring(e: *mut ExpressionDescription, s: *mut TString) {
    unsafe {
        (*e).t = -1;
        (*e).f = (*e).t;
        (*e).k = VKSTR;
        (*e).u.strval = s;
    }
}
pub unsafe extern "C" fn tonumeral(e: *const ExpressionDescription, v: *mut TValue) -> i32 {
    unsafe {
        if (*e).t == (*e).f {
            match (*e).k as u32 {
                6 => {
                    if !v.is_null() {
                        (*v).value.i = (*e).u.ival;
                        (*v).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                    }
                    return 1;
                }
                5 => {
                    if !v.is_null() {
                        (*v).value.n = (*e).u.nval;
                        (*v).set_tag(TAG_VARIANT_NUMERIC_NUMBER);
                    }
                    return 1;
                }
                _ => return 0,
            };
        } else {
            return 0;
        }
    }
}
pub unsafe extern "C" fn const2val(
    fs: *mut FunctionState,
    e: *const ExpressionDescription,
) -> *mut TValue {
    unsafe {
        return &mut (*((*(*(*fs).lexical_state).dynamic_data)
            .active_variable
            .pointer)
            .offset((*e).u.info as isize))
        .k;
    }
}
pub unsafe extern "C" fn luak_exp2const(
    fs: *mut FunctionState,
    e: *const ExpressionDescription,
    v: *mut TValue,
) -> i32 {
    unsafe {
        if (*e).t != (*e).f {
            return 0;
        }
        match (*e).k as u32 {
            3 => {
                (*v).set_tag(TAG_VARIANT_BOOLEAN_FALSE);
                return 1;
            }
            2 => {
                (*v).set_tag(TAG_VARIANT_BOOLEAN_TRUE);
                return 1;
            }
            1 => {
                (*v).set_tag(TAG_VARIANT_NIL_NIL);
                return 1;
            }
            7 => {
                let io: *mut TValue = v;
                let x_: *mut TString = (*e).u.strval;
                (*io).value.object = &mut (*(x_ as *mut Object));
                (*io).set_tag((*x_).get_tag());
                (*io).set_collectable();
                return 1;
            }
            11 => {
                let io1: *mut TValue = v;
                let io2: *const TValue = const2val(fs, e);
                (*io1).value = (*io2).value;
                (*io1).set_tag((*io2).get_tag());
                return 1;
            }
            _ => return tonumeral(e, v),
        };
    }
}
