use crate::expressiondescription::*;
use crate::tdefaultnew::*;
use std::ptr::*;
#[repr(C)]
pub struct ConstructorControl {
    pub constructorcontrol_expressiondescription: ExpressionDescription,
    pub constructorcontrol_table: *mut ExpressionDescription,
    pub constructorcontrol_count_table: i32,
    pub constructorcontrol_count_array: i32,
    pub constructorcontrol_count_to_store: i32,
    pub constructorcontrol_max_to_store: i32,
}
impl TDefaultNew for ConstructorControl {
    fn new() -> Self {
        ConstructorControl {
            constructorcontrol_expressiondescription: ExpressionDescription::new(),
            constructorcontrol_table: null_mut(),
            constructorcontrol_count_table: 0,
            constructorcontrol_count_array: 0,
            constructorcontrol_count_to_store: 0,
            constructorcontrol_max_to_store: 0,
        }
    }
}
impl ConstructorControl {}
