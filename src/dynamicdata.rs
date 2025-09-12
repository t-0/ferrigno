use crate::labeldescription::*;
use crate::variabledescription::*;
use crate::vectort::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DynamicData {
    pub active_variable: VectorT<VariableDescription>,
    pub gt: VectorT<LabelDescription>,
    pub label: VectorT<LabelDescription>,
}
