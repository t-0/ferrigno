use crate::status::*;
use crate::debuginfo::*;
use crate::interpreter::*;
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum CoroutineStatus {
    Running = 0,
    Dead = 1,
    Yield = 2,
    Normal = 3,
}
pub const COROUTINE_STATUS_NAMES: [*const i8; 4] = [c"running".as_ptr(), c"dead".as_ptr(), c"suspended".as_ptr(), c"normal".as_ptr()];
pub unsafe fn auxstatus(interpreter: *mut Interpreter, co: *mut Interpreter) -> CoroutineStatus {
    unsafe {
        if interpreter == co {
            return CoroutineStatus::Running;
        } else {
            match (*co).get_status() {
                Status::Yield => return CoroutineStatus::Yield,
                Status::OK => {
                    let mut debuginfo =  DebugInfo::new();
                    if lua_getstack(co, 0, &mut debuginfo) != 0 {
                        return CoroutineStatus::Normal;
                    } else if (*co).get_top() == 0 {
                        return CoroutineStatus::Dead;
                    } else {
                        return CoroutineStatus::Yield;
                    }
                },
                _ => return CoroutineStatus::Dead,
            }
        };
    }
}
