use crate::state::*;
use crate::lx::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LG {
    pub l: LX,
    pub g: Global,
}
