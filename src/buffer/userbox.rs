#![allow(unpredictable_function_pointer_comparisons,unsafe_code)]
use crate::registeredfunction::*;
use crate::state::*;
use crate::user::*;
#[repr(C)]
pub struct UserBox {
    pub pointer: *mut libc::c_void,
    pub size: u64,
}
impl UserBox {
    pub unsafe extern "C" fn resize_userbox(
        state: *mut State,
        index: i32,
        new_size: u64,
    ) -> *mut libc::c_void {
        unsafe {
            let box_0: *mut UserBox = lua_touserdata(state, index) as *mut UserBox;
            let temp: *mut libc::c_void = raw_allocate((*box_0).pointer, (*box_0).size, new_size);
            if ((temp.is_null() && new_size > 0) as i32 != 0) as i64 != 0 {
                lua_pushstring(state, b"not enough memory\0" as *const u8 as *const i8);
                lua_error(state);
            }
            (*box_0).pointer = temp;
            (*box_0).size = new_size;
            return temp;
        }
    }
    pub unsafe extern "C" fn userbox_gc(state: *mut State) -> i32 {
        unsafe {
            UserBox::resize_userbox(state, 1, 0u64);
            return 0;
        }
    }
    pub const USERBOX_METATABLE: [RegisteredFunction; 3] = {
        [
            {
                RegisteredFunction {
                    name: b"__gc\0" as *const u8 as *const i8,
                    function: Some(UserBox::userbox_gc as unsafe extern "C" fn(*mut State) -> i32),
                }
            },
            {
                RegisteredFunction {
                    name: b"__close\0" as *const u8 as *const i8,
                    function: Some(UserBox::userbox_gc as unsafe extern "C" fn(*mut State) -> i32),
                }
            },
            {
                RegisteredFunction {
                    name: std::ptr::null(),
                    function: None,
                }
            },
        ]
    };
    pub unsafe extern "C" fn new_userbox(state: *mut State) {
        unsafe {
            let box_0: *mut UserBox =
                User::lua_newuserdatauv(state, ::core::mem::size_of::<UserBox>() as u64, 0) as *mut UserBox;
            (*box_0).pointer = std::ptr::null_mut();
            (*box_0).size = 0;
            if lual_newmetatable(state, b"_UBOX*\0" as *const u8 as *const i8) != 0 {
                lual_setfuncs(state, UserBox::USERBOX_METATABLE.as_ptr(), 0);
            }
            lua_setmetatable(state, -2);
        }
    }
}
