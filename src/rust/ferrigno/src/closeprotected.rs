use crate::state::*;
use crate::status::*;
use crate::tdefaultnew::*;
use crate::tvalue::*;
use std::ptr::*;
#[repr(C)]
struct CloseProtected {
    closeprotected_level: *mut TValue,
    closeprotected_status: Status,
}
impl TDefaultNew for CloseProtected {
    fn new() -> Self {
        CloseProtected {
            closeprotected_level: null_mut(),
            closeprotected_status: Status::OK,
        }
    }
}
impl CloseProtected {
    unsafe fn auxiliary(state: *mut State, arbitrary_data: *mut std::ffi::c_void) {
        unsafe {
            let close_protected: *mut CloseProtected = arbitrary_data as *mut CloseProtected;
            luaf_close(
                state,
                (*close_protected).closeprotected_level,
                (*close_protected).closeprotected_status,
                0,
            );
        }
    }
}
pub unsafe fn do_close_protected(state: *mut State, level: i64, mut status: Status) -> Status {
    unsafe {
        let old_call_info = (*state).interpreter_callinfo;
        let old_allowhooks: u8 = (*state).interpreter_allow_hook;
        loop {
            let mut close_protected = CloseProtected::new();
            close_protected.closeprotected_level =
                ((*state).interpreter_stack.stkidrel_pointer as *mut i8).add(level as usize) as *mut TValue;
            close_protected.closeprotected_status = status;
            status = luad_rawrunprotected(
                state,
                Some(CloseProtected::auxiliary as unsafe fn(*mut State, *mut std::ffi::c_void) -> ()),
                &mut close_protected as *mut CloseProtected as *mut std::ffi::c_void,
            );
            if status == Status::OK {
                return close_protected.closeprotected_status;
            } else {
                (*state).interpreter_callinfo = old_call_info;
                (*state).interpreter_allow_hook = old_allowhooks;
            }
        }
    }
}
