use crate::c::*;
use crate::functions::*;
#[derive(Copy, Clone)]
pub struct Stream {
    pub f: *mut FILE,
    pub closef: CFunction,
}
