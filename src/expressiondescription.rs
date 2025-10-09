use crate::expressionkind::*;
use crate::f2i::*;
use crate::functionstate::*;
use crate::interpreter::*;
use crate::lexicalstate::LexicalState;
use crate::object::*;
use crate::tagvariant::*;
use crate::tdefaultnew::*;
use crate::tobject::*;
use crate::tstring::*;
use crate::tvalue::*;
use crate::utility::*;
use crate::value::*;
use std::ptr::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExpressionDescription {
    pub expressiondescription_expressionkind: ExpressionKind,
    pub expressiondescription_value: Value,
    pub expressiondescription_t: i32,
    pub expressiondescription_f: i32,
    pub expressiondescription_previous: *mut ExpressionDescription,
}
impl TDefaultNew for ExpressionDescription {
    fn new() -> Self {
        ExpressionDescription {
            expressiondescription_expressionkind: ExpressionKind::Void,
            expressiondescription_value: Value::new_integer(0),
            expressiondescription_t: 0,
            expressiondescription_f: 0,
            expressiondescription_previous: null_mut(),
        }
    }
}
impl ExpressionDescription {
    pub const fn new_from_integer(integer: i64) -> Self {
        ExpressionDescription {
            expressiondescription_expressionkind: ExpressionKind::ConstantInteger,
            expressiondescription_value: Value::new_integer(integer),
            expressiondescription_t: -1,
            expressiondescription_f: -1,
            expressiondescription_previous: null_mut(),
        }
    }
    pub const fn new_with_previous(previous: *mut ExpressionDescription) -> Self {
        ExpressionDescription {
            expressiondescription_expressionkind: ExpressionKind::Void,
            expressiondescription_value: Value::new_integer(0),
            expressiondescription_t: 0,
            expressiondescription_f: 0,
            expressiondescription_previous: previous,
        }
    }
    pub unsafe fn is_k_int(expression_description: *mut ExpressionDescription) -> bool {
        unsafe {
            return (*expression_description).expressiondescription_expressionkind == ExpressionKind::ConstantInteger
                && !((*expression_description).expressiondescription_t != (*expression_description).expressiondescription_f);
        }
    }
    pub unsafe fn is_c_int(expression_description: *mut ExpressionDescription) -> bool {
        unsafe {
            return ExpressionDescription::is_k_int(expression_description)
                && (*expression_description).expressiondescription_value.value_integer as usize <= ((1 << 8) - 1) as usize;
        }
    }
    pub unsafe fn is_sc_int(expression_description: *mut ExpressionDescription) -> bool {
        unsafe {
            return ExpressionDescription::is_k_int(expression_description)
                && fits_c((*expression_description).expressiondescription_value.value_integer);
        }
    }
    pub unsafe fn init_exp(expression_description: *mut ExpressionDescription, expression_kind: ExpressionKind, i: i32) {
        unsafe {
            (*expression_description).expressiondescription_t = -1;
            (*expression_description).expressiondescription_f = (*expression_description).expressiondescription_t;
            (*expression_description).expressiondescription_expressionkind = expression_kind;
            (*expression_description).expressiondescription_value.value_info = i;
        }
    }
    pub unsafe fn codestring(expression_description: *mut ExpressionDescription, s: *mut TString) {
        unsafe {
            (*expression_description).expressiondescription_t = -1;
            (*expression_description).expressiondescription_f = (*expression_description).expressiondescription_t;
            (*expression_description).expressiondescription_expressionkind = ExpressionKind::ConstantString;
            (*expression_description).expressiondescription_value.value_tstring = s;
        }
    }
    pub unsafe fn tonumeral(expression_description: *const ExpressionDescription, v: *mut TValue) -> bool {
        unsafe {
            if (*expression_description).expressiondescription_t == (*expression_description).expressiondescription_f {
                match (*expression_description).expressiondescription_expressionkind {
                    | ExpressionKind::ConstantInteger => {
                        if !v.is_null() {
                            (*v).tvalue_value.value_integer = (*expression_description).expressiondescription_value.value_integer;
                            (*v).tvalue_set_tag_variant(TagVariant::NumericInteger);
                        }
                        return true;
                    },
                    | ExpressionKind::ConstantNumber => {
                        if !v.is_null() {
                            (*v).tvalue_value.value_number = (*expression_description).expressiondescription_value.value_number;
                            (*v).tvalue_set_tag_variant(TagVariant::NumericNumber);
                        }
                        return true;
                    },
                    | _ => return false,
                };
            } else {
                return false;
            }
        }
    }
    pub unsafe fn const2val(
        lexical_state: *mut LexicalState, _function_state: *mut FunctionState, expression_description: *const ExpressionDescription,
    ) -> *mut TValue {
        unsafe {
            return &mut (*((*(*lexical_state).lexicalstate_dynamicdata)
                .dynamicdata_activevariables
                .vectort_pointer)
                .offset((*expression_description).expressiondescription_value.value_info as isize))
            .variabledescription_k;
        }
    }
    pub unsafe fn luak_exp2const(&self, lexical_state: *mut LexicalState, function_state: *mut FunctionState, output: *mut TValue) -> bool {
        unsafe {
            if self.expressiondescription_t != self.expressiondescription_f {
                return false;
            } else {
                match self.expressiondescription_expressionkind {
                    | ExpressionKind::False => {
                        (*output).tvalue_set_tag_variant(TagVariant::BooleanFalse);
                        return true;
                    },
                    | ExpressionKind::True => {
                        (*output).tvalue_set_tag_variant(TagVariant::BooleanTrue);
                        return true;
                    },
                    | ExpressionKind::Nil => {
                        (*output).tvalue_set_tag_variant(TagVariant::NilNil);
                        return true;
                    },
                    | ExpressionKind::ConstantString => {
                        let tstring: *mut TString = self.expressiondescription_value.value_tstring;
                        (*output).tvalue_value.value_object = &mut (*(tstring as *mut Object));
                        (*output).tvalue_set_tag_variant((*tstring).get_tagvariant());
                        (*output).set_collectable(true);
                        return true;
                    },
                    | ExpressionKind::Constant2 => {
                        let io2: *const TValue = ExpressionDescription::const2val(lexical_state, function_state, self);
                        (*output).copy_from(&*io2);
                        return true;
                    },
                    | _ => return ExpressionDescription::tonumeral(self, output),
                }
            }
        }
    }
    pub unsafe fn const2exp(v: *mut TValue, expression_description: *mut ExpressionDescription) {
        unsafe {
            match (*v).get_tagvariant() {
                | TagVariant::NumericInteger => {
                    (*expression_description).expressiondescription_expressionkind = ExpressionKind::ConstantInteger;
                    (*expression_description).expressiondescription_value.value_integer = (*v).tvalue_value.value_integer;
                },
                | TagVariant::NumericNumber => {
                    (*expression_description).expressiondescription_expressionkind = ExpressionKind::ConstantNumber;
                    (*expression_description).expressiondescription_value.value_number = (*v).tvalue_value.value_number;
                },
                | TagVariant::BooleanFalse => {
                    (*expression_description).expressiondescription_expressionkind = ExpressionKind::False;
                },
                | TagVariant::BooleanTrue => {
                    (*expression_description).expressiondescription_expressionkind = ExpressionKind::True;
                },
                | TagVariant::NilNil => {
                    (*expression_description).expressiondescription_expressionkind = ExpressionKind::Nil;
                },
                | TagVariant::StringShort | TagVariant::StringLong => {
                    (*expression_description).expressiondescription_expressionkind = ExpressionKind::ConstantString;
                    (*expression_description).expressiondescription_value.value_tstring =
                        &mut (*((*v).tvalue_value.value_object as *mut TString));
                },
                | _ => {},
            };
        }
    }
    pub unsafe fn is_sc_number(expression_description: *mut ExpressionDescription, pi: *mut i64, is_float: *mut bool) -> i32 {
        unsafe {
            let mut i: i64 = 0;
            if (*expression_description).expressiondescription_expressionkind == ExpressionKind::ConstantInteger {
                i = (*expression_description).expressiondescription_value.value_integer;
            } else if (*expression_description).expressiondescription_expressionkind == ExpressionKind::ConstantNumber
                && F2I::Equal.convert_f64_i64(
                    (*expression_description).expressiondescription_value.value_number,
                    &mut i,
                )
            {
                *is_float = true;
            } else {
                return 0;
            }
            if !((*expression_description).expressiondescription_t != (*expression_description).expressiondescription_f)
                && fits_c(i)
            {
                *pi = i + ((1 << 8) - 1 >> 1);
                return 1;
            } else {
                return 0;
            };
        }
    }
    pub unsafe fn luak_indexed(
        interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState,
        t: *mut ExpressionDescription, k: *mut ExpressionDescription,
    ) {
        unsafe {
            if (*k).expressiondescription_expressionkind == ExpressionKind::ConstantString {
                string_to_constant(interpreter, lexical_state, function_state, k);
            }
            if (*t).expressiondescription_expressionkind == ExpressionKind::UpValue && !is_k_string(function_state, k) {
                luak_exp2anyreg(interpreter, lexical_state, function_state, t);
            }
            if (*t).expressiondescription_expressionkind == ExpressionKind::UpValue {
                let temp: i32 = (*t).expressiondescription_value.value_info;
                (*t).expressiondescription_value.value_index.valuereference_tag = temp as u8;
                (*t).expressiondescription_value.value_index.valuereference_index =
                    (*k).expressiondescription_value.value_info as i16;
                (*t).expressiondescription_expressionkind = ExpressionKind::IndexUpValue;
            } else {
                (*t).expressiondescription_value.value_index.valuereference_tag =
                    (if (*t).expressiondescription_expressionkind == ExpressionKind::Local {
                        (*t).expressiondescription_value.value_variable.valueregister_registerindex as i32
                    } else {
                        (*t).expressiondescription_value.value_info
                    }) as u8;
                if is_k_string(function_state, k) {
                    (*t).expressiondescription_value.value_index.valuereference_index =
                        (*k).expressiondescription_value.value_info as i16;
                    (*t).expressiondescription_expressionkind = ExpressionKind::Field;
                } else if ExpressionDescription::is_c_int(k) {
                    (*t).expressiondescription_value.value_index.valuereference_index =
                        (*k).expressiondescription_value.value_integer as i16;
                    (*t).expressiondescription_expressionkind = ExpressionKind::IndexInteger;
                } else {
                    (*t).expressiondescription_value.value_index.valuereference_index =
                        luak_exp2anyreg(interpreter, lexical_state, function_state, k) as i16;
                    (*t).expressiondescription_expressionkind = ExpressionKind::Indexed;
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
}
