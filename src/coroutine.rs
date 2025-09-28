use crate::status::*;
use crate::debuginfo::*;
use crate::interpreter::*;
pub const COROUTINE_STATUS_NAMES: [*const i8; 4] = [c"running".as_ptr(), c"dead".as_ptr(), c"suspended".as_ptr(), c"normal".as_ptr()];
pub unsafe fn auxstatus(interpreter: *mut Interpreter, co: *mut Interpreter) -> i32 {
    unsafe {
        if interpreter == co {
            return 0;
        } else {
            match (*co).get_status() {
                Status::Yield => return 2,
                Status::OK => {
                    let mut debuginfo =  DebugInfo::new();
                    if lua_getstack(co, 0, &mut debuginfo) != 0 {
                        return 3;
                    } else if (*co).get_top() == 0 {
                        return 1;
                    } else {
                        return 2;
                    }
                },
                _ => return 1,
            }
        };
    }
}
