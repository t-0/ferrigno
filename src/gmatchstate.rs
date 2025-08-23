use crate::matchstate::*;
#[derive(Copy, Clone)]
pub struct GMatchState {
    pub src: *const i8,
    pub p: *const i8,
    pub lastmatch: *const i8,
    pub ms: MatchState,
}
