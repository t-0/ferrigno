use crate::buffer::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StreamWriter {
    pub is_initialized: bool,
    pub buffer: Buffer,
}
