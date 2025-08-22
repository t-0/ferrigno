use crate::state::*;
pub type WriteFunction =
    Option<unsafe extern "C" fn(*mut State, *const libc::c_void, u64, *mut libc::c_void) -> i32>;
