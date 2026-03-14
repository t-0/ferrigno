#![allow(unpredictable_function_pointer_comparisons, unsafe_code)]
use crate::registeredfunction::*;
use crate::state::*;
use crate::user::*;
use std::ptr::*;
#[repr(C)]
pub struct UserBox {
    pub userbox_pointer: *mut std::ffi::c_void,
    pub userbox_size: usize,
}
impl UserBox {
    pub unsafe fn resize_userbox(state: *mut State, index: i32, newsize: usize) -> *mut std::ffi::c_void {
        unsafe {
            let user_box: *mut UserBox = (*state).to_pointer(index) as *mut UserBox;
            let temp: *mut std::ffi::c_void = raw_allocate((*user_box).userbox_pointer, (*user_box).userbox_size, newsize);
            if temp.is_null() && newsize > 0 {
                lua_pushstring(state, c"not enough memory".as_ptr());
                lua_error(state);
            }
            (*user_box).userbox_pointer = temp;
            (*user_box).userbox_size = newsize;
            temp
        }
    }
    pub unsafe fn userbox_gc(state: *mut State) -> i32 {
        unsafe {
            UserBox::resize_userbox(state, 1, 0);
            0
        }
    }
    pub const USERBOX_METATABLE: [RegisteredFunction; 2] = {
        [
            {
                RegisteredFunction {
                    registeredfunction_name: c"__gc".as_ptr(),
                    registeredfunction_function: Some(UserBox::userbox_gc as unsafe fn(*mut State) -> i32),
                }
            },
            {
                RegisteredFunction {
                    registeredfunction_name: c"__close".as_ptr(),
                    registeredfunction_function: Some(UserBox::userbox_gc as unsafe fn(*mut State) -> i32),
                }
            },
        ]
    };
    pub unsafe fn new_userbox(state: *mut State) {
        unsafe {
            let ubox: *mut UserBox = User::lua_newuserdatauv(state, size_of::<UserBox>(), 0) as *mut UserBox;
            (*ubox).userbox_pointer = null_mut();
            (*ubox).userbox_size = 0;
            if lual_newmetatable(state, c"_UBOX*".as_ptr()) != 0 {
                lual_setfuncs(state, UserBox::USERBOX_METATABLE.as_ptr(), UserBox::USERBOX_METATABLE.len(), 0);
            }
            lua_setmetatable(state, -2);
        }
    }
}
