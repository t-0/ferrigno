#![allow(unpredictable_function_pointer_comparisons, unsafe_code)]
use crate::interpreter::*;
use crate::registeredfunction::*;
use crate::user::*;
use rlua::*;
use std::ptr::*;
#[repr(C)]
pub struct UserBox {
    pub pointer: *mut libc::c_void,
    pub size: usize,
}
impl UserBox {
    pub unsafe fn resize_userbox(
        interpreter: *mut Interpreter,
        index: i32,
        new_size: usize,
    ) -> *mut libc::c_void {
        unsafe {
            let user_box: *mut UserBox = (*interpreter).to_pointer(index) as *mut UserBox;
            let temp: *mut libc::c_void =
                raw_allocate((*user_box).pointer, (*user_box).size as usize, new_size);
            if temp.is_null() && new_size > 0 {
                lua_pushstring(interpreter, make_cstring!("not enough memory"));
                lua_error(interpreter);
            }
            (*user_box).pointer = temp;
            (*user_box).size = new_size;
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
                    name: make_cstring!("__gc"),
                    function: Some(UserBox::userbox_gc as unsafe fn(*mut Interpreter) -> i32),
                }
            },
            {
                RegisteredFunction {
                    name: make_cstring!("__close"),
                    function: Some(UserBox::userbox_gc as unsafe fn(*mut Interpreter) -> i32),
                }
            },
        ]
    };
    pub unsafe fn new_userbox(interpreter: *mut Interpreter) {
        unsafe {
            let box_0: *mut UserBox =
                User::lua_newuserdatauv(interpreter, size_of::<UserBox>(), 0) as *mut UserBox;
            (*box_0).pointer = null_mut();
            (*box_0).size = 0;
            if lual_newmetatable(interpreter, make_cstring!("_UBOX*")) != 0 {
                lual_setfuncs2(interpreter, UserBox::USERBOX_METATABLE.as_ptr(), UserBox::USERBOX_METATABLE.len(), 0);
            }
            lua_setmetatable(interpreter, -2);
        }
    }
}
