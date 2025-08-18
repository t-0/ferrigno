use crate::expdesc::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LHSAssign {
    pub prev: *mut LHSAssign,
    pub v: expdesc,
}
