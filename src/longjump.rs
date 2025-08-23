use crate::c::*;
#[derive(Copy, Clone)]
pub struct LongJump {
    pub previous: *mut LongJump,
    pub b: [__jmp_buf_tag; 1],
    pub status: i32,
}
