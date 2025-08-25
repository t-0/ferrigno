use crate::state::*;
#[derive(Copy, Clone)]
pub struct Header {
    pub state: *mut State,
    pub islittle: i32,
    pub maxalign: i32,
}
