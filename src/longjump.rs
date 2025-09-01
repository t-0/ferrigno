use crate::utility::c::*;
use crate::new::*;
#[repr(C)]
pub struct LongJump {
    pub previous: *mut LongJump,
    pub jbt: [JumpBufferTag; 1],
    pub status: i32,
}
impl New for LongJump {
    fn new() -> Self {
        return LongJump {
            previous: std::ptr::null_mut(),
            jbt: [JumpBufferTag {
                __mask_was_saved: 0,
                __saved_mask: SIgnalSet { __val: [0; 16] },
            }; 1],
            status: 0,
        };
    }
}
