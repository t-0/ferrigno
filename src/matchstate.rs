use crate::state::*;
#[derive(Copy, Clone)]
pub struct MatchState {
    pub src_init: *const i8,
    pub src_end: *const i8,
    pub p_end: *const i8,
    pub state: *mut State,
    pub matchdepth: i32,
    pub level: u8,
    pub capture: [MatchStateCapture; 32],
}
#[derive(Copy, Clone)]
pub struct MatchStateCapture {
    pub init: *const i8,
    pub len: i64,
}
