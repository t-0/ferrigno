use crate::global::*;
use crate::state::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Interpreter {
    pub extra_space: [u8; 8],
    pub state: State,
    pub global: Global,
}
