use crate::expressiondescription::*;
#[repr(C)]
pub struct ConstructorControl {
    pub expression_description: ExpressionDescription,
    pub t: *mut ExpressionDescription,
    pub nh: i32,
    pub na: i32,
    pub to_store: i32,
}
