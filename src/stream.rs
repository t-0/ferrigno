use crate::c::*;
use crate::stkidrel::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Stream {
    pub f: *mut FILE,
    pub closef: CFunction,
}
