use crate::c::*;
#[derive(Copy, Clone)]
pub struct RN {
    pub f: *mut FILE,
    pub c: i32,
    pub n: i32,
    pub buff: [i8; 201],
}
