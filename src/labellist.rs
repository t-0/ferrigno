use crate::labeldesc::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Labellist {
    pub arr: *mut Labeldesc,
    pub n: i32,
    pub size: i32,
}
