use crate::labeldescription::*;
use crate::tdefaultnew::*;
use crate::variabledescription::*;
use crate::vectort::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DynamicData {
    pub dynamicdata_active_variables: VectorT<VariableDescription>,
    pub dynamicdata_goto: VectorT<LabelDescription>,
    pub dynamicdata_labels: VectorT<LabelDescription>,
}
impl DynamicData {}
impl TDefaultNew for DynamicData {
    fn new() -> Self {
        DynamicData {
            dynamicdata_active_variables: VectorT::<VariableDescription>::new(),
            dynamicdata_goto: VectorT::<LabelDescription>::new(),
            dynamicdata_labels: VectorT::<LabelDescription>::new(),
        }
    }
}
