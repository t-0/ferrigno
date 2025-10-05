use crate::c::*;
use crate::status::*;
use crate::tdefaultnew::*;
use std::ptr::*;
#[repr(C)]
pub struct LongJump {
    pub longjump_previous: *mut LongJump,
    pub longjump_jbt: [JumpBufferTag; 1],
    pub longjump_status: Status,
}
impl TDefaultNew for LongJump {
    fn new() -> Self {
        return LongJump {
            longjump_previous: null_mut(),
            longjump_jbt: [JumpBufferTag { __mask_was_saved: 0, __saved_mask: SignalSet { __val: [0; 16] } }; 1],
            longjump_status: Status::OK,
        };
    }
}
impl LongJump {}
