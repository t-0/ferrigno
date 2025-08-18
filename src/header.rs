#![allow(
    non_snake_case,
)]
use crate::state::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Header {
    pub L: *mut State,
    pub islittle: i32,
    pub maxalign: i32,
}
