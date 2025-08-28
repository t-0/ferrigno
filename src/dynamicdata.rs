use crate::labellist::*;
use crate::variabledescription::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DynamicData {
    pub active_variable: DynamicDataActiveVariable,
    pub gt: LabelList,
    pub label: LabelList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DynamicDataActiveVariable {
    pub pointer: *mut VariableDescription,
    pub n: i32,
    pub size: i32,
}
