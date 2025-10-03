use crate::expressiondescription::*;
use crate::tdefaultnew::*;
use std::ptr::*;
#[repr(C)]
pub struct ConstructorControl {
    pub constructorcontrol_expressiondescription: ExpressionDescription,
    pub constructorcontrol_table: *mut ExpressionDescription,
    pub constructorcontrol_counttable: i32,
    pub constructorcontrol_countarray: i32,
    pub constructorcontrol_counttostore: i32,
}
impl TDefaultNew for ConstructorControl {
    fn new() -> Self {
        ConstructorControl {
            constructorcontrol_expressiondescription: ExpressionDescription::new(),
            constructorcontrol_table: null_mut(),
            constructorcontrol_counttable: 0,
            constructorcontrol_countarray: 0,
            constructorcontrol_counttostore: 0,
        }
    }
}
impl ConstructorControl {}
