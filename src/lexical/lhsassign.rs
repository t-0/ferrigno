use std::ptr::*;
use crate::expressiondescription::*;
use crate::expressionkind::*;
use crate::value::*;
#[repr(C)]
pub struct LHSAssign {
    pub previous: *mut LHSAssign,
    pub expression_description: ExpressionDescription,
}
impl LHSAssign {
    pub fn new() -> Self {
        LHSAssign {
            previous: null_mut(),
            expression_description: ExpressionDescription {
                expression_kind: ExpressionKind::VVOID,
                value: Value::new_integer(0),
                t: 0,
                f: 0,
            },
        }
    }
}
