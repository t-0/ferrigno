use crate::expressiondescription::*;
#[repr(C)]
pub struct ConstructorControl {
    pub v: ExpressionDescription,
    pub t: *mut ExpressionDescription,
    pub nh: i32,
    pub na: i32,
    pub tostore: i32,
}
