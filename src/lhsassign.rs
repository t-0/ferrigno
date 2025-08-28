use crate::expressiondescription::*;
#[repr(C)]
pub struct LHSAssign {
    pub previous: *mut LHSAssign,
    pub v: ExpressionDescription,
}
