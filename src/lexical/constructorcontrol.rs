use std::ptr::*;
use crate::expressiondescription::*;
#[repr(C)]
pub struct ConstructorControl {
    pub expression_description: ExpressionDescription,
    pub constructor_control_table: *mut ExpressionDescription,
    pub count_table: i32,
    pub count_array: i32,
    pub count_to_store: i32,
}
impl ConstructorControl {
    pub fn new() -> Self {
        ConstructorControl {
            expression_description: ExpressionDescription::new(),
            constructor_control_table: null_mut(),
            count_table: 0,
            count_array: 0,
            count_to_store: 0,
        }
    }
}
