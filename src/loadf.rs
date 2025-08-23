use crate::c::*;
pub struct LoadF {
    pub n: i32,
    pub f: *mut FILE,
    pub buff: [i8; 8192],
}
