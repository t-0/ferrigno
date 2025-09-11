use crate::debuginfo::*;
use crate::interpreter::*;
pub type ReadFunction =
    Option<unsafe extern "C" fn(*mut Interpreter, *mut libc::c_void, *mut usize) -> *const libc::c_char>;
pub type HookFunction = Option<unsafe extern "C" fn(*mut Interpreter, *mut DebugInfo) -> ()>;
pub type WarnFunction = Option<unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, i32) -> ()>;
pub type ContextFunction = Option<unsafe extern "C" fn(*mut Interpreter, i32, i64) -> i32>;
pub type CFunction = Option<unsafe extern "C" fn(*mut Interpreter) -> i32>;
pub type WriteFunction =
    Option<unsafe extern "C" fn(*mut Interpreter, *const libc::c_void, usize, *mut libc::c_void) -> i32>;
pub type ProtectedFunction = Option<unsafe extern "C" fn(*mut Interpreter, *mut libc::c_void) -> ()>;
