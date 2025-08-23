use crate::debug::*;
use crate::state::*;
pub type HookFunction = Option<unsafe extern "C" fn(*mut State, *mut Debug) -> ()>;
pub type WarnFunction = Option<unsafe extern "C" fn(*mut libc::c_void, *const i8, i32) -> ()>;
pub type AllocationFunction = Option<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void, u64, u64) -> *mut libc::c_void,
>;
