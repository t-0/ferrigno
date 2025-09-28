use crate::status::*;
use libc::*;
use crate::debuginfo::*;
use crate::interpreter::*;
pub type ReadFunction = Option<unsafe fn(*mut Interpreter, *mut c_void, *mut usize) -> *const i8>;
pub type HookFunction = Option<unsafe fn(*mut Interpreter, *mut DebugInfo) -> ()>;
pub type WarnFunction = Option<unsafe fn(*mut c_void, *const i8, i32) -> ()>;
pub type ContextFunction = Option<unsafe fn(*mut Interpreter, Status, i64) -> i32>;
pub type CFunction = Option<unsafe fn(*mut Interpreter) -> i32>;
pub type WriteFunction = Option<unsafe fn(*mut Interpreter, *const c_void, usize, *mut c_void) -> i32>;
pub type ProtectedFunction = Option<unsafe fn(*mut Interpreter, *mut c_void) -> ()>;
#[derive(Copy, Clone)]
pub struct Reader {
    pub reader_readfunction: ReadFunction,
}
impl Reader {
    pub fn new (function: ReadFunction) -> Self {
        Reader {
            reader_readfunction: function,
        }
    }
}
