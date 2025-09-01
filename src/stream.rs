use crate::utility::c::*;
use crate::functions::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Stream {
    pub f: *mut FILE,
    pub closef: CFunction,
}
