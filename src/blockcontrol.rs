use crate::tdefaultnew::*;
use std::ptr::*;
#[repr(C)]
pub struct BlockControl {
    pub blockcontrol_previous: *mut BlockControl,
    pub blockcontrol_firstlabel: i32,
    pub blockcontrol_firstgoto: i32,
    pub blockcontrol_countactivevariables: u8,
    pub blockcontrol_countupvalues: u8,
    pub blockcontrol_isloop: bool,
    pub blockcontrol_isinsidetbc: bool,
}
impl TDefaultNew for BlockControl {
    fn new() -> Self {
        return BlockControl {
            blockcontrol_previous: null_mut(),
            blockcontrol_firstlabel: 0,
            blockcontrol_firstgoto: 0,
            blockcontrol_countactivevariables: 0,
            blockcontrol_countupvalues: 0,
            blockcontrol_isloop: false,
            blockcontrol_isinsidetbc: false,
            ..
        };
    }
}
impl BlockControl {
    pub unsafe fn mark_upvalue_delegated(&mut self, level: i32) {
        unsafe {
            let mut block_control: *mut BlockControl = self;
            while (*block_control).blockcontrol_countactivevariables as i32 > level {
                block_control = (*block_control).blockcontrol_previous;
            }
            (*block_control).blockcontrol_countupvalues = 1;
        }
    }
}
