#![allow(
    non_snake_case,
)]
use crate::state::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Buffer {
    pub b: *mut i8,
    pub size: u64,
    pub n: u64,
    pub L: *mut State,
    pub init: BufferInit,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union BufferInit {
    pub n: f64,
    pub u: f64,
    pub s: *mut libc::c_void,
    pub i: i64,
    pub l: i64,
    pub b: [i8; 1024],
}
