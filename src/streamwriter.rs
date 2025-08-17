#![allow (
    non_snake_case,
)]
use crate::lual_buffer::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StreamWriter {
    pub init: i32,
    pub B: luaL_Buffer,
}
