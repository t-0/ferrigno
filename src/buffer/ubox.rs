#![allow(unpredictable_function_pointer_comparisons,unsafe_code)]
#[repr(C)]
pub struct UBox {
    pub box_0: *mut libc::c_void,
    pub bsize: u64,
}
use crate::registeredfunction::*;
use crate::state::*;
use crate::user::*;
pub unsafe extern "C" fn resizebox(
    state: *mut State,
    index: i32,
    new_size: u64,
) -> *mut libc::c_void {
    unsafe {
        let box_0: *mut UBox = lua_touserdata(state, index) as *mut UBox;
        let temp: *mut libc::c_void = raw_allocate((*box_0).box_0, (*box_0).bsize, new_size);
        if ((temp.is_null() && new_size > 0u64) as i32 != 0) as i64 != 0 {
            lua_pushstring(state, b"not enough memory\0" as *const u8 as *const i8);
            lua_error(state);
        }
        (*box_0).box_0 = temp;
        (*box_0).bsize = new_size;
        return temp;
    }
}
pub unsafe extern "C" fn boxgc(state: *mut State) -> i32 {
    unsafe {
        resizebox(state, 1, 0u64);
        return 0;
    }
}
pub const BOX_METATABLE: [RegisteredFunction; 3] = {
    [
        {
            RegisteredFunction {
                name: b"__gc\0" as *const u8 as *const i8,
                function: Some(boxgc as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"__close\0" as *const u8 as *const i8,
                function: Some(boxgc as unsafe extern "C" fn(*mut State) -> i32),
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
pub unsafe extern "C" fn newbox(state: *mut State) {
    unsafe {
        let box_0: *mut UBox =
            User::lua_newuserdatauv(state, ::core::mem::size_of::<UBox>() as u64, 0) as *mut UBox;
        (*box_0).box_0 = std::ptr::null_mut();
        (*box_0).bsize = 0;
        if lual_newmetatable(state, b"_UBOX*\0" as *const u8 as *const i8) != 0 {
            lual_setfuncs(state, BOX_METATABLE.as_ptr(), 0);
        }
        lua_setmetatable(state, -2);
    }
}
