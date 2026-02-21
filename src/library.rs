mod base;
mod coroutine;
mod debug;
mod io;
mod math;
mod os;
mod package;
mod sh;
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
use crate::library::sh::*;
use crate::library::string::*;
use crate::library::table::*;
use crate::library::utf8::*;
use crate::registeredfunction::*;
pub const LOADED_FUNCTIONS: [RegisteredFunction; 11] = {
    [
        {
            RegisteredFunction {
                registeredfunction_name: c"_G".as_ptr(),
                registeredfunction_function: Some(luaopen_base as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"package".as_ptr(),
                registeredfunction_function: Some(luaopen_package as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"coroutine".as_ptr(),
                registeredfunction_function: Some(luaopen_coroutine as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"table".as_ptr(),
                registeredfunction_function: Some(luaopen_table as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"io".as_ptr(),
                registeredfunction_function: Some(luaopen_io as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"os".as_ptr(),
                registeredfunction_function: Some(luaopen_os as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"string".as_ptr(),
                registeredfunction_function: Some(luaopen_string as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"math".as_ptr(),
                registeredfunction_function: Some(luaopen_math as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"utf8".as_ptr(),
                registeredfunction_function: Some(luaopen_utf8 as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"debug".as_ptr(),
                registeredfunction_function: Some(luaopen_debug as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"sh".as_ptr(),
                registeredfunction_function: Some(luaopen_sh as unsafe fn(*mut Interpreter) -> i32),
            }
        },
    ]
};
pub unsafe fn lual_openlibs(interpreter: *mut Interpreter) {
    unsafe {
        for it in LOADED_FUNCTIONS {
            lual_requiref(interpreter, it.registeredfunction_name, it.registeredfunction_function, 1);
            lua_settop(interpreter, -2);
        }
    }
}
