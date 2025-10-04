use crate::functions::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Stream {
    pub stream_file: *mut libc::FILE,
    pub stream_cfunctionclose: CFunction,
}
