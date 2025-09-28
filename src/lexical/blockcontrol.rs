use crate::new::*;
use std::ptr::*;
#[repr(C)]
pub struct BlockControl {
    pub previous: *mut BlockControl,
    pub first_label: i32,
    pub first_goto: i32,
    pub count_active_variables: u8,
    pub count_upvalues: u8,
    pub is_loop: bool,
    pub is_inside_tbc: bool,
}
impl New for BlockControl {
    fn new() -> Self {
        return BlockControl {
            previous: null_mut(),
            first_label: 0,
            first_goto: 0,
            count_active_variables: 0,
            count_upvalues: 0,
            is_loop: false,
            is_inside_tbc: false,
            ..
        };
    }
}
impl BlockControl {
    pub unsafe fn mark_upvalue_delegated(&mut self, level: i32) {
        unsafe {
            let mut block_control: *mut BlockControl = self;
            while (*block_control).count_active_variables as i32 > level {
                block_control = (*block_control).previous;
            }
            (*block_control).count_upvalues = 1;
        }
    }
}
