use crate::c::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LoadF {
    pub n: i32,
    pub f: *mut FILE,
    pub buff: [i8; 8192],
}
