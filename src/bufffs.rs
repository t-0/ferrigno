use crate::state::*;
pub struct BuffFS {
    pub state: *mut State,
    pub is_pushed: bool,
    pub size: i32,
    pub space: [i8; 199],
}
