#[derive(Copy, Clone)]
#[repr(C)]
pub struct RN {
    pub rn_file: *mut libc::FILE,
    pub rn_c: i32,
    pub rn_n: i32,
    pub rn_buffer: [i8; 201],
}
