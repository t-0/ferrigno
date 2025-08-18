#![allow(
    non_snake_case,
)]
use crate::state::*;
use crate::zio::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LoadState {
    pub L: *mut State,
    pub Z: *mut ZIO,
    pub name: *const i8,
}
