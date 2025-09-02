use crate::expressiondescription::*;
#[repr(C)]
pub struct LHSAssign {
    pub previous: *mut LHSAssign,
    pub expression_description: ExpressionDescription,
}
