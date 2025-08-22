use crate::state::*;
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
    pub n: f64,
    pub u: f64,
    pub s: *mut libc::c_void,
    pub i: i64,
    pub l: i64,
    pub b: [i8; 1024],
}
