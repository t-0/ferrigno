use crate::utility::c::*;
use crate::functions::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Stream {
    pub file: *mut FILE,
    pub close_function: CFunction,
}
