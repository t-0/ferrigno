use crate::c::*;
#[repr(C)]
pub struct LoadF {
    pub loadf_n: i32,
    pub loadf_file: *mut libc::FILE,
    pub loadf_buffer: [i8; 8192],
}
