use crate::state::*;
use crate::readfunction::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZIO {
    pub n: u64,
    pub p: *const i8,
    pub reader: ReadFunction,
    pub data: *mut libc::c_void,
    pub state: *mut State,
}
