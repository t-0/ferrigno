use crate::c::*;
pub struct LoadF {
    pub n: i32,
    pub f: *mut FILE,
    pub buffer: [i8; 8192],
}
