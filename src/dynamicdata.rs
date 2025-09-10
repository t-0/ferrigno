use crate::vectort::*;
use crate::variabledescription::*;
use crate::labeldescription::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DynamicData {
    pub active_variable: DynamicDataActiveVariable,
    pub gt: VectorT<LabelDescription>,
    pub label: VectorT<LabelDescription>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DynamicDataActiveVariable {
    pub pointer: *mut VariableDescription,
    pub length: i32,
    pub size: i32,
}
