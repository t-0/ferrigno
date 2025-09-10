use crate::vectort::*;
use crate::variabledescription::*;
use crate::labeldescription::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DynamicData {
    pub active_variable: VectorT<VariableDescription>,
    pub gt: VectorT<LabelDescription>,
    pub label: VectorT<LabelDescription>,
}
