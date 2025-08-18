use crate::state::*;
use crate::writefunction::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DumpState {
    pub state: *mut State,
    pub writer: WriteFunction,
    pub data: *mut libc::c_void,
    pub strip: i32,
    pub status: i32,
}
