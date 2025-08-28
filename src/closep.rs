use crate::new::*;
use crate::stackvalue::*;
#[repr(C)]
pub struct CloseP {
    pub level: StkId,
    pub status: i32,
}
impl New for CloseP {
    fn new() -> Self {
        return CloseP {
            level: std::ptr::null_mut(),
            status: 0,
        };
    }
}
