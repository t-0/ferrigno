use crate::variabledescription::*;
use crate::labellist::*;
#[derive(Copy, Clone)]
pub struct DynamicData {
    pub active_variable: DynamicDataActiveVariable,
    pub gt: Labellist,
    pub label: Labellist,
}
#[derive(Copy, Clone)]
pub struct DynamicDataActiveVariable {
    pub arr: *mut VariableDescription,
    pub n: i32,
    pub size: i32,
}
