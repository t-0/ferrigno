use crate::state::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BuffFS {
    pub state: *mut State,
    pub is_pushed: bool,
    pub blen: i32,
    pub space: [i8; 199],
}
