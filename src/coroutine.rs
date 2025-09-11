use std::ptr::*;
use crate::debuginfo::*;
use crate::interpreter::*;
pub const COROUTINE_STATUS_NAMES: [*const i8; 4] = [
    b"running\0" as *const u8 as *const i8,
    b"dead\0" as *const u8 as *const i8,
    b"suspended\0" as *const u8 as *const i8,
    b"normal\0" as *const u8 as *const i8,
];
pub unsafe extern "C" fn auxstatus(interpreter: *mut Interpreter, co: *mut Interpreter) -> i32 {
    unsafe {
        if interpreter == co {
            return 0;
        } else {
            match (*co).get_status() {
                1 => return 2,
                0 => {
                    let mut ar: DebugInfo = DebugInfo {
                        event: 0,
                        name: null(),
                        namewhat: null(),
                        what: null(),
                        source: null(),
                        source_length: 0,
                        currentline: 0,
                        line_defined: 0,
                        last_line_defined: 0,
                        nups: 0,
                        nparams: 0,
                        is_variable_arguments: false,
                        is_tail_call: false,
                        ftransfer: 0,
                        ntransfer: 0,
                        short_src: [0; 60],
                        i_ci: null_mut(),
                    };
                    if lua_getstack(co, 0, &mut ar) != 0 {
                        return 3;
                    } else if (*co).get_top() == 0 {
                        return 1;
                    } else {
                        return 2;
                    }
                }
                _ => return 1,
            }
        };
    }
}
