use crate::state::*;
pub const MAX_CAPTURES: usize = 32;
#[derive(Copy, Clone)]
pub struct MatchState {
    pub src_init: *const i8,
    pub src_end: *const i8,
    pub p_end: *const i8,
    pub state: *mut State,
    pub matchdepth: i32,
    pub level: usize,
    pub capture: [MatchStateCapture; MAX_CAPTURES],
}
#[derive(Copy, Clone)]
pub struct MatchStateCapture {
    pub init: *const i8,
    pub len: i64,
}
