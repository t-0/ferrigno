use crate::interpreter::*;
use crate::new::*;
use crate::tvalue::*;
use std::ptr::*;
#[repr(C)]
struct CloseProtected {
    level: *mut TValue,
    status: i32,
}
impl New for CloseProtected {
    fn new() -> Self {
        return CloseProtected { level: null_mut(), status: 0 };
    }
}
impl CloseProtected {
    unsafe fn auxiliary(interpreter: *mut Interpreter, arbitrary_data: *mut libc::c_void) {
        unsafe {
            let close_protected: *mut CloseProtected = arbitrary_data as *mut CloseProtected;
            luaf_close(interpreter, (*close_protected).level, (*close_protected).status, 0);
        }
    }
}
pub unsafe fn do_close_protected(interpreter: *mut Interpreter, level: i64, mut status: i32) -> i32 {
    unsafe {
        let old_call_info = (*interpreter).call_info;
        let old_allowhooks: u8 = (*interpreter).allow_hook;
        loop {
            let mut close_protected = CloseProtected::new();
            close_protected.level = ((*interpreter).stack.stkidrel_pointer as *mut i8).offset(level as isize) as *mut TValue;
            close_protected.status = status;
            status = luad_rawrunprotected(
                interpreter,
                Some(CloseProtected::auxiliary as unsafe fn(*mut Interpreter, *mut libc::c_void) -> ()),
                &mut close_protected as *mut CloseProtected as *mut libc::c_void,
            );
            if status == 0 {
                return close_protected.status;
            } else {
                (*interpreter).call_info = old_call_info;
                (*interpreter).allow_hook = old_allowhooks;
            }
        }
    }
}
