use crate::c::*;
use crate::status::*;
use crate::tdefaultnew::*;
use std::ptr::*;
#[repr(C)]
pub struct LongJump {
    pub longjump_previous: *mut LongJump,
    pub longjump_jumpbuffer: JumpBuffer,
    pub longjump_status: Status,
}
impl TDefaultNew for LongJump {
    fn new() -> Self {
        return LongJump {
            longjump_previous: null_mut(),
            longjump_jumpbuffer: JumpBuffer::new(),
            longjump_status: Status::OK,
        };
    }
}
impl LongJump {}
