use crate::state::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BuffFS {
    pub state: *mut State,
    pub pushed: i32,
    pub blen: i32,
    pub space: [i8; 199],
}
