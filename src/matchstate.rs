#![allow(
    non_snake_case,
)]
use crate::state::*;
use crate::c2rustunnamed_30::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MatchState {
    pub src_init: *const i8,
    pub src_end: *const i8,
    pub p_end: *const i8,
    pub L: *mut State,
    pub matchdepth: i32,
    pub level: u8,
    pub capture: [C2RustUnnamed_30; 32],
}
