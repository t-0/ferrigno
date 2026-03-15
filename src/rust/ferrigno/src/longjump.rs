use crate::status::*;
use std::ptr::*;

pub struct LuaError {
    pub status: Status,
    pub unwind_to_base: bool,
}

pub struct LongJump {
    pub longjump_previous: *mut LongJump,
}
impl Default for LongJump {
    fn default() -> Self {
        Self::new()
    }
}

impl LongJump {
    pub fn new() -> Self {
        LongJump { longjump_previous: null_mut() }
    }
}
