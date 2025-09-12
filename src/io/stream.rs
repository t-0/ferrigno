use crate::functions::*;
use crate::utility::c::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Stream {
    pub file: *mut FILE,
    pub close_function: CFunction,
}
