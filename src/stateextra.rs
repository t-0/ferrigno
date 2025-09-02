use crate::state::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StateExtra {
    pub extra_space: [u8; 8],
    pub state: State,
}
