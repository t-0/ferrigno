use crate::state::*;
use crate::new::*;
#[derive(Copy, Clone)]
pub struct Buffer {
    pub pointer: *mut i8,
    pub size: u64,
    pub length: u64,
    pub state: *mut State,
    pub buffer_initial: BufferInitial,
}
#[derive(Copy, Clone)]
pub union BufferInitial {
    pub block: [i8; 1024],
}
impl New for Buffer {
    fn new() -> Self {
        return Buffer {
            pointer: std::ptr::null_mut(),
            size: 0,
            length: 0,
            state: std::ptr::null_mut(),
            buffer_initial: BufferInitial { block: [0; 1024] },
        };
    }
}
