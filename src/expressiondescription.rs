use crate::expressionkind::*;
use crate::f2i::*;
use crate::functionstate::*;
use crate::object::*;
use crate::tag::*;
use crate::tstring::*;
use crate::tvalue::*;
use crate::utility::*;
use crate::value::*;
use std::ptr::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExpressionDescription {
    pub expression_kind: ExpressionKind,
    pub value: Value,
    pub t: i32,
    pub f: i32,
    pub previous: *mut ExpressionDescription,
}
impl ExpressionDescription {
    pub fn new () -> Self {
        ExpressionDescription {
            expression_kind: ExpressionKind::Void,
            value: Value::new_integer (0),
            t: 0,
            f: 0,
            previous: null_mut(),
        }
    }
    pub const fn new_from_integer (integer: i64) -> Self {
        ExpressionDescription {
            expression_kind: ExpressionKind::ConstantInteger,
            value: Value::new_integer (integer),
            t: -1,
            f: -1,
            previous: null_mut(),
        }
    }
    pub const fn new_with_previous (previous: *mut ExpressionDescription) -> Self {
        ExpressionDescription {
            expression_kind: ExpressionKind::Void,
            value: Value::new_integer (0),
            t: 0,
            f: 0,
            previous: previous,
        }
    }
}
pub unsafe fn init_exp(
    e: *mut ExpressionDescription,
    expression_kind: ExpressionKind,
    i: i32,
) {
    unsafe {
        (*e).t = -1;
        (*e).f = (*e).t;
        (*e).expression_kind = expression_kind;
        (*e).value.info = i;
    }
}
pub unsafe fn codestring(e: *mut ExpressionDescription, s: *mut TString) {
    unsafe {
        (*e).t = -1;
        (*e).f = (*e).t;
        (*e).expression_kind = ExpressionKind::ConstantString;
        (*e).value.tstring = s;
    }
}
pub unsafe fn tonumeral(e: *const ExpressionDescription, v: *mut TValue) -> bool {
    unsafe {
        if (*e).t == (*e).f {
            match (*e).expression_kind {
                ExpressionKind::ConstantInteger => {
                    if !v.is_null() {
                        (*v).value.integer = (*e).value.integer;
                        (*v).set_tag_variant(TAG_VARIANT_NUMERIC_INTEGER);
                    }
                    return true;
                }
                ExpressionKind::ConstantNumber => {
                    if !v.is_null() {
                        (*v).value.number = (*e).value.number;
                        (*v).set_tag_variant(TAG_VARIANT_NUMERIC_NUMBER);
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
pub unsafe fn const2val(
    function_state: *mut FunctionState,
    e: *const ExpressionDescription,
) -> *mut TValue {
    unsafe {
        return &mut (*((*(*(*function_state).lexical_state).dynamic_data)
            .active_variables
            .vectort_pointer)
            .offset((*e).value.info as isize))
        .k;
    }
}
pub unsafe fn luak_exp2const(
    function_state: *mut FunctionState,
    e: *const ExpressionDescription,
    v: *mut TValue,
) -> bool {
    unsafe {
        if (*e).t != (*e).f {
            return false;
        }
        match (*e).expression_kind {
            ExpressionKind::False => {
                (*v).set_tag_variant(TAG_VARIANT_BOOLEAN_FALSE);
                return true;
            }
            ExpressionKind::True => {
                (*v).set_tag_variant(TAG_VARIANT_BOOLEAN_TRUE);
                return true;
            }
            ExpressionKind::Nil => {
                (*v).set_tag_variant(TagVariant::NilNil as u8);
                return true;
            }
            ExpressionKind::ConstantString => {
                let ts: *mut TString = (*e).value.tstring;
                (*v).value.object = &mut (*(ts as *mut Object));
                (*v).set_tag_variant((*ts).get_tag_variant());
                (*v).set_collectable(true);
                return true;
            }
            ExpressionKind::Constant2 => {
                let io2: *const TValue = const2val(function_state, e);
                (*v).copy_from(&*io2);
                return true;
            }
            _ => return tonumeral(e, v),
        };
    }
}
pub unsafe fn const2exp(v: *mut TValue, e: *mut ExpressionDescription) {
    unsafe {
        match (*v).get_tag_variant() {
            TAG_VARIANT_NUMERIC_INTEGER => {
                (*e).expression_kind = ExpressionKind::ConstantInteger;
                (*e).value.integer = (*v).value.integer;
            }
            TAG_VARIANT_NUMERIC_NUMBER => {
                (*e).expression_kind = ExpressionKind::ConstantNumber;
                (*e).value.number = (*v).value.number;
            }
            TAG_VARIANT_BOOLEAN_FALSE => {
                (*e).expression_kind = ExpressionKind::False;
            }
            TAG_VARIANT_BOOLEAN_TRUE => {
                (*e).expression_kind = ExpressionKind::True;
            }
            TAG_VARIANT_NIL_NIL => {
                (*e).expression_kind = ExpressionKind::Nil;
            }
            TAG_VARIANT_STRING_SHORT | TAG_VARIANT_STRING_LONG => {
                (*e).expression_kind = ExpressionKind::ConstantString;
                (*e).value.tstring = &mut (*((*v).value.object as *mut TString));
            }
            _ => {}
        };
    }
}
pub unsafe fn is_k_int(e: *mut ExpressionDescription) -> bool {
    unsafe {
        return (*e).expression_kind as u32 == ExpressionKind::ConstantInteger as u32 && !((*e).t != (*e).f);
    }
}
pub unsafe fn is_c_int(e: *mut ExpressionDescription) -> bool {
    unsafe {
        return is_k_int(e) && (*e).value.integer as usize <= ((1 << 8) - 1) as usize;
    }
}
pub unsafe fn is_sc_int(e: *mut ExpressionDescription) -> bool {
    unsafe {
        return is_k_int(e) && fits_c((*e).value.integer);
    }
}
pub unsafe fn is_sc_number(
    e: *mut ExpressionDescription,
    pi: *mut i32,
    is_float: *mut bool,
) -> i32 {
    unsafe {
        let mut i: i64 = 0;
        if (*e).expression_kind as u32 == ExpressionKind::ConstantInteger as u32 {
            i = (*e).value.integer;
        } else if (*e).expression_kind as u32 == ExpressionKind::ConstantNumber as u32
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
pub unsafe fn luak_indexed(
    function_state: *mut FunctionState,
    t: *mut ExpressionDescription,
    k: *mut ExpressionDescription,
) {
    unsafe {
        if (*k).expression_kind as u32 == ExpressionKind::ConstantString as u32 {
            str_to_k(function_state, k);
        }
        if (*t).expression_kind as u32 == ExpressionKind::UpValue as u32
            && !is_k_string(function_state, k)
        {
            luak_exp2anyreg(function_state, t);
        }
        if (*t).expression_kind as u32 == ExpressionKind::UpValue as u32 {
            let temp: i32 = (*t).value.info;
            (*t).value.index.reference_tag = temp as u8;
            (*t).value.index.reference_index = (*k).value.info as i16;
            (*t).expression_kind = ExpressionKind::IndexUpValue;
        } else {
            (*t).value.index.reference_tag = (if (*t).expression_kind == ExpressionKind::Local {
                (*t).value.variable.register_index as i32
            } else {
                (*t).value.info
            }) as u8;
            if is_k_string(function_state, k) {
                (*t).value.index.reference_index = (*k).value.info as i16;
                (*t).expression_kind = ExpressionKind::IndexString;
            } else if is_c_int(k) {
                (*t).value.index.reference_index = (*k).value.integer as i16;
                (*t).expression_kind = ExpressionKind::IndexInteger;
            } else {
                (*t).value.index.reference_index = luak_exp2anyreg(function_state, k) as i16;
                (*t).expression_kind = ExpressionKind::Indexed;
            }
        };
    }
}
pub unsafe fn swapexps(e1: *mut ExpressionDescription, e2: *mut ExpressionDescription) {
    unsafe {
        let temp: ExpressionDescription = *e1;
        *e1 = *e2;
        *e2 = temp;
    }
}
