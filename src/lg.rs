use crate::lx::*;
use crate::state::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LG {
    pub l: LX,
    pub g: Global,
}
