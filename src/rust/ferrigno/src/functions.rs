use crate::debuginfo::*;
use crate::state::*;
use crate::status::*;
pub type SignalHandlerFunction = Option<unsafe extern "C" fn(i32) -> ()>;
pub type ReadFunction = Option<unsafe fn(*mut State, *mut std::ffi::c_void, *mut usize) -> *const i8>;
pub type HookFunction = Option<unsafe fn(*mut State, *mut DebugInfo) -> ()>;
pub type WarnFunction = Option<unsafe fn(*mut std::ffi::c_void, *const i8, i32) -> ()>;
pub type ContextFunction = Option<unsafe fn(*mut State, Status, i64) -> i32>;
pub type CFunction = Option<unsafe fn(*mut State) -> i32>;
pub type WriteFunction = Option<unsafe fn(*mut State, *const std::ffi::c_void, usize, *mut std::ffi::c_void) -> i32>;
pub type ProtectedFunction = Option<unsafe fn(*mut State, *mut std::ffi::c_void) -> ()>;
pub type AllocationFunction =
    Option<unsafe fn(*mut std::ffi::c_void, *mut std::ffi::c_void, usize, usize) -> *mut std::ffi::c_void>;
#[derive(Copy, Clone)]
pub struct Reader {
    pub reader_read_function: ReadFunction,
}
impl Reader {
    pub fn new(function: ReadFunction) -> Self {
        Reader {
            reader_read_function: function,
        }
    }
}
