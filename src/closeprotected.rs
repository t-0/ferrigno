use crate::interpreter::*;
use crate::status::*;
use crate::tdefaultnew::*;
use crate::tvalue::*;
use libc::*;
use std::ptr::*;
#[repr(C)]
struct CloseProtected {
    closeprotected_level: *mut TValue,
    closeprotected_status: Status,
}
impl TDefaultNew for CloseProtected {
    fn new() -> Self {
        return CloseProtected { closeprotected_level: null_mut(), closeprotected_status: Status::OK };
    }
}
impl CloseProtected {
    unsafe fn auxiliary(interpreter: *mut Interpreter, arbitrary_data: *mut c_void) {
        unsafe {
            let close_protected: *mut CloseProtected = arbitrary_data as *mut CloseProtected;
            luaf_close(
                interpreter,
                (*close_protected).closeprotected_level,
                (*close_protected).closeprotected_status,
                0,
            );
        }
    }
}
pub unsafe fn do_close_protected(interpreter: *mut Interpreter, level: i64, mut status: Status) -> Status {
    unsafe {
        let old_call_info = (*interpreter).interpreter_callinfo;
        let old_allowhooks: u8 = (*interpreter).interpreter_allowhook;
        loop {
            let mut close_protected = CloseProtected::new();
            close_protected.closeprotected_level =
                ((*interpreter).interpreter_stack.stkidrel_pointer as *mut i8).offset(level as isize) as *mut TValue;
            close_protected.closeprotected_status = status;
            status = luad_rawrunprotected(
                interpreter,
                Some(CloseProtected::auxiliary as unsafe fn(*mut Interpreter, *mut c_void) -> ()),
                &mut close_protected as *mut CloseProtected as *mut c_void,
            );
            if status == Status::OK {
                return close_protected.closeprotected_status;
            } else {
                (*interpreter).interpreter_callinfo = old_call_info;
                (*interpreter).interpreter_allowhook = old_allowhooks;
            }
        }
    }
}
