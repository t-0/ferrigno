use crate::utility::c::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RN {
    pub file: *mut FILE,
    pub c: i32,
    pub n: i32,
    pub buffer: [i8; 201],
}
