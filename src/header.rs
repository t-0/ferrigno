use crate::state::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Header {
    pub state: *mut State,
    pub is_little_endian: i32,
    pub maxmimum_alignment: i32,
}
