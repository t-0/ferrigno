use crate::c::*;
use crate::new::*;
pub struct LongJump {
    pub previous: *mut LongJump,
    pub b: [JumpBufferTag; 1],
    pub status: i32,
}
impl New for LongJump {
    fn new () -> Self {
        return LongJump {
            previous: std::ptr::null_mut(),
            b: [JumpBufferTag {
                __mask_was_saved: 0,
                __saved_mask: SIgnalSet { __val: [0; 16] },
            }; 1],
            status: 0,
        };
    }
}
