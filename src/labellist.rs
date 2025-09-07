use crate::labeldescription::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LabelList {
    pub pointer: *mut LabelDescription,
    pub length: i32,
    pub size: i32,
}
