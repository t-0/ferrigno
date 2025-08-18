use crate::state::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LX {
    pub extra_: [u8; 8],
    pub l: State,
}
