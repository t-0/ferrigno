use crate::expressiondescription::*;
pub struct LHSAssign {
    pub prev: *mut LHSAssign,
    pub v: ExpressionDescription,
}
