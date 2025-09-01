mod debug;
mod math;
mod base;
mod coroutine;
mod table;
mod io;
use crate::onelua::*;
use crate::registeredfunction::*;
use crate::state::*;
use crate::library::coroutine::*;
use crate::library::debug::*;
use crate::library::math::*;
use crate::library::base::*;
use crate::library::table::*;
use crate::library::io::*;
pub const LOADED_FUNCTIONS: [RegisteredFunction; 11] = {
    [
        {
            RegisteredFunction {
                name: b"_G\0" as *const u8 as *const i8,
                function: Some(luaopen_base as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"package\0" as *const u8 as *const i8,
                function: Some(luaopen_package as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"coroutine\0" as *const u8 as *const i8,
                function: Some(luaopen_coroutine as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"table\0" as *const u8 as *const i8,
                function: Some(luaopen_table as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"io\0" as *const u8 as *const i8,
                function: Some(luaopen_io as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"os\0" as *const u8 as *const i8,
                function: Some(luaopen_os as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"string\0" as *const u8 as *const i8,
                function: Some(luaopen_string as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"math\0" as *const u8 as *const i8,
                function: Some(luaopen_math as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"utf8\0" as *const u8 as *const i8,
                function: Some(luaopen_utf8 as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"debug\0" as *const u8 as *const i8,
                function: Some(luaopen_debug as unsafe extern "C" fn(*mut State) -> i32),
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
pub unsafe extern "C" fn lual_openlibs(state: *mut State) {
    unsafe {
        let mut lib: *const RegisteredFunction = LOADED_FUNCTIONS.as_ptr();
        while ((*lib).function).is_some() {
            lual_requiref(state, (*lib).name, (*lib).function, 1);
            lua_settop(state, -1 - 1);
            lib = lib.offset(1);
        }
    }
}
