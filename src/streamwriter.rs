#![allow (
    non_snake_case,
)]
use crate::buffer::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StreamWriter {
    pub init: i32,
    pub B: Buffer,
}
