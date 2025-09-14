use crate::functions::*;
use crate::utility::c::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Stream {
    pub file: *mut FILE,
    pub stream_cfunction_close: CFunction,
}
