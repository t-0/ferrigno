use crate::c::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RN {
    pub f: *mut FILE,
    pub c: i32,
    pub n: i32,
    pub buffer: [i8; 201],
}
