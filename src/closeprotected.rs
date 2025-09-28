use crate::interpreter::*;
use crate::new::*;
use crate::tvalue::*;
use libc::*;
use std::ptr::*;
#[repr(C)]
struct CloseProtected {
    closeprotected_level: *mut TValue,
    closeprotected_status: Status,
}
impl New for CloseProtected {
    fn new() -> Self {
        return CloseProtected { closeprotected_level: null_mut(), closeprotected_status: Status::OK };
    }
}
impl CloseProtected {
    unsafe fn auxiliary(interpreter: *mut Interpreter, arbitrary_data: *mut c_void) {
        unsafe {
            let close_protected: *mut CloseProtected = arbitrary_data as *mut CloseProtected;
            luaf_close(interpreter, (*close_protected).closeprotected_level, (*close_protected).closeprotected_status, 0);
        }
    }
}
pub unsafe fn do_close_protected(interpreter: *mut Interpreter, level: i64, mut status: Status) -> Status {
    unsafe {
        let old_call_info = (*interpreter).callinfo;
        let old_allowhooks: u8 = (*interpreter).allow_hook;
        loop {
            let mut close_protected = CloseProtected::new();
            close_protected.closeprotected_level = ((*interpreter).stack.stkidrel_pointer as *mut i8).offset(level as isize) as *mut TValue;
            close_protected.closeprotected_status = status;
            status = luad_rawrunprotected(
                interpreter,
                Some(CloseProtected::auxiliary as unsafe fn(*mut Interpreter, *mut c_void) -> ()),
                &mut close_protected as *mut CloseProtected as *mut c_void,
            );
            if status == Status::OK {
                return close_protected.closeprotected_status;
            } else {
                (*interpreter).callinfo = old_call_info;
                (*interpreter).allow_hook = old_allowhooks;
            }
        }
    }
}
