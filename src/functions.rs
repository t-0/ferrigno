use crate::debug::*;
use crate::state::*;
pub type ReadFunction =
    Option<unsafe extern "C" fn(*mut State, *mut libc::c_void, *mut u64) -> *const i8>;
pub type HookFunction = Option<unsafe extern "C" fn(*mut State, *mut Debug) -> ()>;
pub type WarnFunction = Option<unsafe extern "C" fn(*mut libc::c_void, *const i8, i32) -> ()>;
pub type AllocationFunction = Option<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void, u64, u64) -> *mut libc::c_void,
>;
pub type ContextFunction = Option<unsafe extern "C" fn(*mut State, i32, i64) -> i32>;
pub type CFunction = Option<unsafe extern "C" fn(*mut State) -> i32>;
pub type WriteFunction =
    Option<unsafe extern "C" fn(*mut State, *const libc::c_void, u64, *mut libc::c_void) -> i32>;
pub type Pfunc = Option<unsafe extern "C" fn(*mut State, *mut libc::c_void) -> ()>;
