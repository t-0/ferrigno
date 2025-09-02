use crate::global::*;
use crate::stateextra::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Interpreter {
    pub state_extra: StateExtra,
    pub global: Global,
}
