#![allow(
    non_snake_case,
)]
use crate::state::*;
use crate::c2rustunnamed_27::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Buffer {
    pub b: *mut i8,
    pub size: u64,
    pub n: u64,
    pub L: *mut State,
    pub init: C2RustUnnamed27,
}
