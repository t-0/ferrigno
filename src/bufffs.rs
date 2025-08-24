use crate::state::*;
use crate::new::*;
pub const BUFFFS_SIZE: usize = 199;
pub struct BuffFS {
    pub state: *mut State,
    pub is_pushed: bool,
    pub size: i32,
    pub block: [i8; BUFFFS_SIZE],
}
impl New for BuffFS {
    fn new() -> Self {
        return BuffFS {
            state: std::ptr::null_mut(),
            is_pushed: false,
            size: 0,
            block: [0; BUFFFS_SIZE],
        }
    }
}
