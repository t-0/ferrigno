use crate::c::*;
use crate::new::*;
pub struct LongJump {
    pub previous: *mut LongJump,
    pub b: [__jmp_buf_tag; 1],
    pub status: i32,
}
impl New for LongJump {
    fn new () -> Self {
        return LongJump {
            previous: 0 as *mut LongJump,
            b: [__jmp_buf_tag {
                __mask_was_saved: 0,
                __saved_mask: __sigset_t { __val: [0; 16] },
            }; 1],
            status: 0,
        };
    }
}
