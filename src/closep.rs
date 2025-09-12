use crate::new::*;
use crate::stackvalue::*;
use std::ptr::*;
#[repr(C)]
pub struct CloseP {
    pub level: StackValuePointer,
    pub status: i32,
}
impl New for CloseP {
    fn new() -> Self {
        return CloseP {
            level: null_mut(),
            status: 0,
        };
    }
}
