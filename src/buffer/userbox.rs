#![allow(unpredictable_function_pointer_comparisons,unsafe_code)]
use std::ptr::*;
use crate::registeredfunction::*;
use crate::interpreter::*;
use crate::user::*;
#[repr(C)]
pub struct UserBox {
    pub pointer: *mut libc::c_void,
    pub size: usize,
}
impl UserBox {
    pub unsafe extern "C" fn resize_userbox(
        interpreter: *mut Interpreter,
        index: i32,
        new_size: usize,
    ) -> *mut libc::c_void {
        unsafe {
            let user_box: *mut UserBox = lua_touserdata(interpreter, index) as *mut UserBox;
            let temp: *mut libc::c_void = raw_allocate((*user_box).pointer, (*user_box).size as usize, new_size);
            if temp.is_null() && new_size > 0 {
                lua_pushstring(interpreter, b"not enough memory\0" as *const u8 as *const i8);
                lua_error(interpreter);
            }
            (*user_box).pointer = temp;
            (*user_box).size = new_size;
            return temp;
        }
    }
    pub unsafe extern "C" fn userbox_gc(interpreter: *mut Interpreter) -> i32 {
        unsafe {
            UserBox::resize_userbox(interpreter, 1, 0);
            return 0;
        }
    }
    pub const USERBOX_METATABLE: [RegisteredFunction; 3] = {
        [
            {
                RegisteredFunction {
                    name: b"__gc\0" as *const u8 as *const i8,
                    function: Some(UserBox::userbox_gc as unsafe extern "C" fn(*mut Interpreter) -> i32),
                }
            },
            {
                RegisteredFunction {
                    name: b"__close\0" as *const u8 as *const i8,
                    function: Some(UserBox::userbox_gc as unsafe extern "C" fn(*mut Interpreter) -> i32),
                }
            },
            {
                RegisteredFunction {
                    name: null(),
                    function: None,
                }
            },
        ]
    };
    pub unsafe extern "C" fn new_userbox(interpreter: *mut Interpreter) {
        unsafe {
            let box_0: *mut UserBox =
                User::lua_newuserdatauv(interpreter, ::core::mem::size_of::<UserBox>(), 0) as *mut UserBox;
            (*box_0).pointer = null_mut();
            (*box_0).size = 0;
            if lual_newmetatable(interpreter, b"_UBOX*\0" as *const u8 as *const i8) != 0 {
                lual_setfuncs(interpreter, UserBox::USERBOX_METATABLE.as_ptr(), 0);
            }
            lua_setmetatable(interpreter, -2);
        }
    }
}
