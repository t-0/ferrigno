use crate::rawvalue::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExpressionDescription {
    pub k: u32,
    pub u: RawValue,
    pub t: i32,
    pub f: i32,
}
