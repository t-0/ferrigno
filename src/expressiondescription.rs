use crate::tstring::*;
use crate::v::*;
use crate::tvalue::*;
use crate::object::*;
use crate::functionstate::*;
use crate::tag::*;
use crate::utility::*;
use crate::value::*;
use crate::f2i::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExpressionDescription {
    pub kind: V,
    pub value: Value,
    pub t: i32,
    pub f: i32,
}
pub unsafe extern "C" fn init_exp(e: *mut ExpressionDescription, k: V, i: i32) {
    unsafe {
        (*e).t = -1;
        (*e).f = (*e).t;
        (*e).kind = k;
        (*e).value.info = i;
    }
}
pub unsafe extern "C" fn codestring(e: *mut ExpressionDescription, s: *mut TString) {
    unsafe {
        (*e).t = -1;
        (*e).f = (*e).t;
        (*e).kind = V::VKSTR;
        (*e).value.tstring = s;
    }
}
pub unsafe extern "C" fn tonumeral(e: *const ExpressionDescription, v: *mut TValue) -> bool {
    unsafe {
        if (*e).t == (*e).f {
            match (*e).kind as u32 {
                6 => {
                    if !v.is_null() {
                        (*v).value.integer = (*e).value.integer;
                        (*v).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                    }
                    return true;
                }
                5 => {
                    if !v.is_null() {
                        (*v).value.number = (*e).value.number;
                        (*v).set_tag(TAG_VARIANT_NUMERIC_NUMBER);
                    }
                    return true;
                }
                _ => return false,
            };
        } else {
            return false;
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
            .offset((*e).value.info as isize))
        .k;
    }
}
pub unsafe extern "C" fn luak_exp2const(
    fs: *mut FunctionState,
    e: *const ExpressionDescription,
    v: *mut TValue,
) -> bool {
    unsafe {
        if (*e).t != (*e).f {
            return false;
        }
        match (*e).kind {
            V::VFALSE => {
                (*v).set_tag(TAG_VARIANT_BOOLEAN_FALSE);
                return true;
            }
            V::VTRUE => {
                (*v).set_tag(TAG_VARIANT_BOOLEAN_TRUE);
                return true;
            }
            V::VNIL => {
                (*v).set_tag(TAG_VARIANT_NIL_NIL);
                return true;
            }
            V::VKSTR => {
                let x_: *mut TString = (*e).value.tstring;
                (*v).value.object = &mut (*(x_ as *mut Object));
                (*v).set_tag((*x_).get_tag());
                (*v).set_collectable();
                return true;
            }
            V::VCONST => {
                let io2: *const TValue = const2val(fs, e);
                (*v).value = (*io2).value;
                (*v).set_tag((*io2).get_tag());
                return true;
            }
            _ => return tonumeral(e, v),
        };
    }
}
pub unsafe extern "C" fn const2exp(v: *mut TValue, e: *mut ExpressionDescription) {
    unsafe {
        match (*v).get_tag_variant() {
            TAG_VARIANT_NUMERIC_INTEGER => {
                (*e).kind = V::VKINT;
                (*e).value.integer = (*v).value.integer;
            }
            TAG_VARIANT_NUMERIC_NUMBER => {
                (*e).kind = V::VKFLT;
                (*e).value.number = (*v).value.number;
            }
            TAG_VARIANT_BOOLEAN_FALSE => {
                (*e).kind = V::VFALSE;
            }
            TAG_VARIANT_BOOLEAN_TRUE => {
                (*e).kind = V::VTRUE;
            }
            TAG_VARIANT_NIL_NIL => {
                (*e).kind = V::VNIL;
            }
            TAG_VARIANT_STRING_SHORT | TAG_VARIANT_STRING_LONG => {
                (*e).kind = V::VKSTR;
                (*e).value.tstring = &mut (*((*v).value.object as *mut TString));
            }
            _ => {}
        };
    }
}
pub unsafe extern "C" fn is_k_int(e: *mut ExpressionDescription) -> bool {
    unsafe {
        return (*e).kind as u32 == V::VKINT as u32 && !((*e).t != (*e).f);
    }
}
pub unsafe extern "C" fn is_c_int(e: *mut ExpressionDescription) -> bool{
    unsafe {
        return is_k_int(e) && (*e).value.integer as u64 <= ((1 << 8) - 1) as u64;
    }
}
pub unsafe extern "C" fn is_sc_int(e: *mut ExpressionDescription) -> bool {
    unsafe {
        return is_k_int(e) && fits_c((*e).value.integer);
    }
}
pub unsafe extern "C" fn is_sc_number(
    e: *mut ExpressionDescription,
    pi: *mut i32,
    is_float: *mut bool,
) -> i32 {
    unsafe {
        let mut i: i64 = 0;
        if (*e).kind as u32 == V::VKINT as u32 {
            i = (*e).value.integer;
        } else if (*e).kind as u32 == V::VKFLT as u32
            && luav_flttointeger((*e).value.number, &mut i, F2I::Equal)
        {
            *is_float = true;
        } else {
            return 0;
        }
        if !((*e).t != (*e).f) && fits_c(i) {
            *pi = i as i32 + ((1 << 8) - 1 >> 1);
            return 1;
        } else {
            return 0;
        };
    }
}
pub unsafe extern "C" fn luak_indexed(
    fs: *mut FunctionState,
    t: *mut ExpressionDescription,
    k: *mut ExpressionDescription,
) {
    unsafe {
        if (*k).kind as u32 == V::VKSTR as u32 {
            str_to_k(fs, k);
        }
        if (*t).kind as u32 == V::VUPVAL as u32 && !is_k_string(fs, k) {
            luak_exp2anyreg(fs, t);
        }
        if (*t).kind as u32 == V::VUPVAL as u32 {
            let temp: i32 = (*t).value.info;
            (*t).value.index.reference_tag = temp as u8;
            (*t).value.index.reference_index = (*k).value.info as i16;
            (*t).kind = V::VINDEXUP;
        } else {
            (*t).value.index.reference_tag = (if (*t).kind == V::VLOCAL {
                (*t).value.variable.register_index as i32
            } else {
                (*t).value.info
            }) as u8;
            if is_k_string(fs, k) {
                (*t).value.index.reference_index = (*k).value.info as i16;
                (*t).kind = V::VINDEXSTR;
            } else if is_c_int(k) {
                (*t).value.index.reference_index = (*k).value.integer as i16;
                (*t).kind = V::VINDEXI;
            } else {
                (*t).value.index.reference_index = luak_exp2anyreg(fs, k) as i16;
                (*t).kind = V::VINDEXED;
            }
        };
    }
}
pub unsafe extern "C" fn swapexps(e1: *mut ExpressionDescription, e2: *mut ExpressionDescription) {
    unsafe {
        let temp: ExpressionDescription = *e1;
        *e1 = *e2;
        *e2 = temp;
    }
}
