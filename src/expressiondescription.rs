use crate::expressionkind::*;
use crate::f2i::*;
use crate::functionstate::*;
use crate::interpreter::*;
use crate::lexical::lexicalstate::LexicalState;
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
    expression_description: *mut ExpressionDescription,
    expression_kind: ExpressionKind,
    i: i32,
) {
    unsafe {
        (*expression_description).t = -1;
        (*expression_description).f = (*expression_description).t;
        (*expression_description).expression_kind = expression_kind;
        (*expression_description).value.info = i;
    }
}
pub unsafe fn codestring(expression_description: *mut ExpressionDescription, s: *mut TString) {
    unsafe {
        (*expression_description).t = -1;
        (*expression_description).f = (*expression_description).t;
        (*expression_description).expression_kind = ExpressionKind::ConstantString;
        (*expression_description).value.tstring = s;
    }
}
pub unsafe fn tonumeral(expression_description: *const ExpressionDescription, v: *mut TValue) -> bool {
    unsafe {
        if (*expression_description).t == (*expression_description).f {
            match (*expression_description).expression_kind {
                ExpressionKind::ConstantInteger => {
                    if !v.is_null() {
                        (*v).value.integer = (*expression_description).value.integer;
                        (*v).set_tag_variant(TAG_VARIANT_NUMERIC_INTEGER);
                    }
                    return true;
                }
                ExpressionKind::ConstantNumber => {
                    if !v.is_null() {
                        (*v).value.number = (*expression_description).value.number;
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
    lexical_state: *mut LexicalState,
    _function_state: *mut FunctionState,
    expression_description: *const ExpressionDescription,
) -> *mut TValue {
    unsafe {
        return &mut (*((*(*lexical_state).dynamic_data)
            .active_variables
            .vectort_pointer)
            .offset((*expression_description).value.info as isize))
        .k;
    }
}
pub unsafe fn luak_exp2const(
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    expression_description: *const ExpressionDescription,
    v: *mut TValue,
) -> bool {
    unsafe {
        if (*expression_description).t != (*expression_description).f {
            return false;
        }
        match (*expression_description).expression_kind {
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
                let ts: *mut TString = (*expression_description).value.tstring;
                (*v).value.object = &mut (*(ts as *mut Object));
                (*v).set_tag_variant((*ts).get_tag_variant());
                (*v).set_collectable(true);
                return true;
            }
            ExpressionKind::Constant2 => {
                let io2: *const TValue = const2val(lexical_state, function_state, expression_description);
                (*v).copy_from(&*io2);
                return true;
            }
            _ => return tonumeral(expression_description, v),
        };
    }
}
pub unsafe fn const2exp(v: *mut TValue, expression_description: *mut ExpressionDescription) {
    unsafe {
        match (*v).get_tag_variant() {
            TAG_VARIANT_NUMERIC_INTEGER => {
                (*expression_description).expression_kind = ExpressionKind::ConstantInteger;
                (*expression_description).value.integer = (*v).value.integer;
            }
            TAG_VARIANT_NUMERIC_NUMBER => {
                (*expression_description).expression_kind = ExpressionKind::ConstantNumber;
                (*expression_description).value.number = (*v).value.number;
            }
            TAG_VARIANT_BOOLEAN_FALSE => {
                (*expression_description).expression_kind = ExpressionKind::False;
            }
            TAG_VARIANT_BOOLEAN_TRUE => {
                (*expression_description).expression_kind = ExpressionKind::True;
            }
            TAG_VARIANT_NIL_NIL => {
                (*expression_description).expression_kind = ExpressionKind::Nil;
            }
            TAG_VARIANT_STRING_SHORT | TAG_VARIANT_STRING_LONG => {
                (*expression_description).expression_kind = ExpressionKind::ConstantString;
                (*expression_description).value.tstring = &mut (*((*v).value.object as *mut TString));
            }
            _ => {}
        };
    }
}
pub unsafe fn is_k_int(expression_description: *mut ExpressionDescription) -> bool {
    unsafe {
        return (*expression_description).expression_kind == ExpressionKind::ConstantInteger && !((*expression_description).t != (*expression_description).f);
    }
}
pub unsafe fn is_c_int(expression_description: *mut ExpressionDescription) -> bool {
    unsafe {
        return is_k_int(expression_description) && (*expression_description).value.integer as usize <= ((1 << 8) - 1) as usize;
    }
}
pub unsafe fn is_sc_int(expression_description: *mut ExpressionDescription) -> bool {
    unsafe {
        return is_k_int(expression_description) && fits_c((*expression_description).value.integer);
    }
}
pub unsafe fn is_sc_number(
    expression_description: *mut ExpressionDescription,
    pi: *mut i32,
    is_float: *mut bool,
) -> i32 {
    unsafe {
        let mut i: i64 = 0;
        if (*expression_description).expression_kind == ExpressionKind::ConstantInteger {
            i = (*expression_description).value.integer;
        } else if (*expression_description).expression_kind == ExpressionKind::ConstantNumber
            && luav_flttointeger((*expression_description).value.number, &mut i, F2I::Equal)
        {
            *is_float = true;
        } else {
            return 0;
        }
        if !((*expression_description).t != (*expression_description).f) && fits_c(i) {
            *pi = i as i32 + ((1 << 8) - 1 >> 1);
            return 1;
        } else {
            return 0;
        };
    }
}
pub unsafe fn luak_indexed(interpreter: * mut Interpreter,
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    t: *mut ExpressionDescription,
    k: *mut ExpressionDescription,
) {
    unsafe {
        if (*k).expression_kind == ExpressionKind::ConstantString {
            string_to_constant(interpreter, lexical_state, function_state, k);
        }
        if (*t).expression_kind == ExpressionKind::UpValue
            && !is_k_string(function_state, k)
        {
            luak_exp2anyreg(interpreter, lexical_state, function_state, t);
        }
        if (*t).expression_kind == ExpressionKind::UpValue {
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
                (*t).expression_kind = ExpressionKind::Field;
            } else if is_c_int(k) {
                (*t).value.index.reference_index = (*k).value.integer as i16;
                (*t).expression_kind = ExpressionKind::IndexInteger;
            } else {
                (*t).value.index.reference_index = luak_exp2anyreg(interpreter, lexical_state, function_state, k) as i16;
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
