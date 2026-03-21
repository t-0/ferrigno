use crate::debuginfo::*;
use crate::state::*;
use crate::status::*;
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum CoroutineStatus {
    Running = 0,
    Dead = 1,
    Yield = 2,
    Normal = 3,
}
impl CoroutineStatus {
    pub fn get_name(&self) -> *const i8 {
        match *self {
            CoroutineStatus::Dead => c"dead".as_ptr(),
            CoroutineStatus::Yield => c"suspended".as_ptr(),
            CoroutineStatus::Running => c"running".as_ptr(),
            CoroutineStatus::Normal => c"normal".as_ptr(),
        }
    }
    pub unsafe fn auxiliary_status(state: *mut State, coroutine: *mut State) -> CoroutineStatus {
        unsafe {
            if state == coroutine {
                CoroutineStatus::Running
            } else {
                match (*coroutine).get_status() {
                    Status::Yield => CoroutineStatus::Yield,
                    Status::OK => {
                        let mut debuginfo = DebugInfo::new();
                        if lua_getstack(coroutine, 0, &mut debuginfo) != 0 {
                            CoroutineStatus::Normal
                        } else if (*coroutine).get_top() == 0 {
                            CoroutineStatus::Dead
                        } else {
                            CoroutineStatus::Yield
                        }
                    }
                    _ => CoroutineStatus::Dead,
                }
            }
        }
    }
}
