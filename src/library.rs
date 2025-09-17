mod base;
mod coroutine;
mod debug;
mod io;
mod math;
mod os;
mod package;
mod string;
mod table;
mod utf8;
use crate::interpreter::*;
use crate::library::base::*;
use crate::library::coroutine::*;
use crate::library::debug::*;
use crate::library::io::*;
use crate::library::math::*;
use crate::library::os::*;
use crate::library::package::*;
use crate::library::string::*;
use crate::library::table::*;
use crate::library::utf8::*;
use crate::registeredfunction::*;
use rlua::*;
pub const LOADED_FUNCTIONS: [RegisteredFunction; 10] = {
    [
        {
            RegisteredFunction {
                name: make_cstring!("_G"),
                function: Some(luaopen_base as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: make_cstring!("package"),
                function: Some(luaopen_package as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: make_cstring!("coroutine"),
                function: Some(luaopen_coroutine as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: make_cstring!("table"),
                function: Some(luaopen_table as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: make_cstring!("io"),
                function: Some(luaopen_io as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: make_cstring!("os"),
                function: Some(luaopen_os as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: make_cstring!("string"),
                function: Some(luaopen_string as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: make_cstring!("math"),
                function: Some(luaopen_math as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: make_cstring!("utf8"),
                function: Some(luaopen_utf8 as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: make_cstring!("debug"),
                function: Some(luaopen_debug as unsafe fn(*mut Interpreter) -> i32),
            }
        },
    ]
};
pub unsafe fn lual_openlibs(interpreter: *mut Interpreter) {
    unsafe {
        for it in LOADED_FUNCTIONS {
            lual_requiref(interpreter, it.name, it.function, 1);
            lua_settop(interpreter, -2);
        }
    }
}
