use crate::utility::c::*;
#[repr(C)]
pub struct LoadF {
    pub n: i32,
    pub file: *mut FILE,
    pub buffer: [libc::c_char; 8192],
}
