use crate::tdefaultnew::*;
use crate::utility::c::*;
use std::ptr::*;
use crate::status::*;
#[repr(C)]
pub struct LongJump {
    pub previous: *mut LongJump,
    pub jbt: [JumpBufferTag; 1],
    pub status: Status,
}
impl TDefaultNew for LongJump {
    fn new() -> Self {
        return LongJump { previous: null_mut(), jbt: [JumpBufferTag { __mask_was_saved: 0, __saved_mask: SIgnalSet { __val: [0; 16] } }; 1], status: Status::OK };
    }
}
impl LongJump {
}
