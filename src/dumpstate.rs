use crate::state::*;
use crate::writefunction::*;
pub struct DumpState {
    pub state: *mut State,
    pub writer: WriteFunction,
    pub data: *mut libc::c_void,
    pub is_strip: bool,
    pub status: i32,
}
