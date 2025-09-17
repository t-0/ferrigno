use crate::debuginfo::*;
use crate::interpreter::*;
pub type ReadFunction = Option<unsafe fn(*mut Interpreter, *mut libc::c_void, *mut usize) -> *const i8>;
pub type HookFunction = Option<unsafe fn(*mut Interpreter, *mut DebugInfo) -> ()>;
pub type WarnFunction = Option<unsafe fn(*mut libc::c_void, *const i8, i32) -> ()>;
pub type ContextFunction = Option<unsafe fn(*mut Interpreter, i32, i64) -> i32>;
pub type CFunction = Option<unsafe fn(*mut Interpreter) -> i32>;
pub type WriteFunction = Option<unsafe fn(*mut Interpreter, *const libc::c_void, usize, *mut libc::c_void) -> i32>;
pub type ProtectedFunction = Option<unsafe fn(*mut Interpreter, *mut libc::c_void) -> ()>;
