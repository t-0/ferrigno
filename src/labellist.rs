use crate::labeldescription::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Labellist {
    pub arr: *mut LabelDescription,
    pub n: i32,
    pub size: i32,
}
