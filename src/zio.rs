#![allow(
    non_snake_case,
)]
use crate::state::*;
use crate::lua_reader::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZIO {
    pub n: u64,
    pub p: *const i8,
    pub reader: lua_Reader,
    pub data: *mut libc::c_void,
    pub L: *mut State,
}
