use crate::state::*;
#[derive(Copy, Clone)]
pub struct Header {
    pub state: *mut State,
    pub is_little: bool,
    pub maxalign: i32,
}
