#![allow(
    non_snake_case,
)]
use crate::state::*;
use crate::writefunction::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DumpState {
    pub L: *mut State,
    pub writer: WriteFunction,
    pub data: *mut libc::c_void,
    pub strip: i32,
    pub status: i32,
}
