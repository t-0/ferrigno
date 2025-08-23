use crate::state::*;
pub trait New {
    fn new() -> Self;
}
#[derive(Copy, Clone)]
pub struct Buffer {
    pub pointer: *mut i8,
    pub allocated: u64,
    pub length: u64,
    pub state: *mut State,
    pub initial_data: BufferInitial,
}
#[derive(Copy, Clone)]
pub union BufferInitial {
    pub block: [i8; 1024],
}
impl New for Buffer {
    fn new() -> Self {
        return Buffer {
            pointer: 0 as *mut i8,
            allocated: 0,
            length: 0,
            state: 0 as *mut State,
            initial_data: BufferInitial { block: [0; 1024]},
        };
    }
}
