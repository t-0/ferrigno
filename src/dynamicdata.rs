use crate::labeldescription::*;
use crate::variabledescription::*;
use crate::vectort::*;
use crate::tdefaultnew::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DynamicData {
    pub active_variables: VectorT<VariableDescription>,
    pub goto_: VectorT<LabelDescription>,
    pub labels: VectorT<LabelDescription>,
}
impl DynamicData {
}
impl TDefaultNew for DynamicData {
    fn new() -> Self {
        DynamicData {
            active_variables: VectorT::<VariableDescription>::new(),
            goto_: VectorT::<LabelDescription>::new(),
            labels: VectorT::<LabelDescription>::new(),
        }
    }
}
