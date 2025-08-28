use crate::state::*;
use crate::functions::*;
#[repr(C)]
pub struct DumpState {
    pub state: *mut State,
    pub write_function: WriteFunction,
    pub data: *mut libc::c_void,
    pub is_strip: bool,
    pub status: i32,
}
