#![allow(unpredictable_function_pointer_comparisons, unsafe_code)]
use crate::interpreter::*;
use crate::registeredfunction::*;
use crate::user::*;
use std::ptr::*;
#[repr(C)]
pub struct UserBox {
    pub userbox_pointer: *mut libc::c_void,
    pub userbox_size: usize,
}
impl UserBox {
    pub unsafe fn resize_userbox(interpreter: *mut Interpreter, index: i32, newsize: usize) -> *mut libc::c_void {
        unsafe {
            let user_box: *mut UserBox = (*interpreter).to_pointer(index) as *mut UserBox;
            let temp: *mut libc::c_void = raw_allocate((*user_box).userbox_pointer, (*user_box).userbox_size as usize, newsize);
            if temp.is_null() && newsize > 0 {
                lua_pushstring(interpreter, c"not enough memory".as_ptr());
                lua_error(interpreter);
            }
            (*user_box).userbox_pointer = temp;
            (*user_box).userbox_size = newsize;
            return temp;
        }
    }
    pub unsafe fn userbox_gc(interpreter: *mut Interpreter) -> i32 {
        unsafe {
            UserBox::resize_userbox(interpreter, 1, 0);
            return 0;
        }
    }
    pub const USERBOX_METATABLE: [RegisteredFunction; 2] = {
        [
            {
                RegisteredFunction {
                    registeredfunction_name: c"__gc".as_ptr(),
                    registeredfunction_function: Some(UserBox::userbox_gc as unsafe fn(*mut Interpreter) -> i32),
                }
            },
            {
                RegisteredFunction {
                    registeredfunction_name: c"__close".as_ptr(),
                    registeredfunction_function: Some(UserBox::userbox_gc as unsafe fn(*mut Interpreter) -> i32),
                }
            },
        ]
    };
    pub unsafe fn new_userbox(interpreter: *mut Interpreter) {
        unsafe {
            let box_0: *mut UserBox = User::lua_newuserdatauv(interpreter, size_of::<UserBox>(), 0) as *mut UserBox;
            (*box_0).userbox_pointer = null_mut();
            (*box_0).userbox_size = 0;
            if lual_newmetatable(interpreter, c"_UBOX*".as_ptr()) != 0 {
                lual_setfuncs(
                    interpreter,
                    UserBox::USERBOX_METATABLE.as_ptr(),
                    UserBox::USERBOX_METATABLE.len(),
                    0,
                );
            }
            lua_setmetatable(interpreter, -2);
        }
    }
}
