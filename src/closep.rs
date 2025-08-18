use crate::stkidrel::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CloseP {
    pub level: StkId,
    pub status: i32,
}
