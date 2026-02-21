mod base;
mod coroutine;
mod debug;
mod io;
pub mod json;
mod math;
mod midi;
mod os;
mod package;
mod requests;
mod sh;
mod sqlite;
mod string;
mod toml;
mod table;
mod tui;
mod urllib;
mod utf8;
use crate::interpreter::*;
use crate::library::base::*;
use crate::library::coroutine::*;
use crate::library::debug::*;
use crate::library::io::*;
use crate::library::json::*;
use crate::library::math::*;
use crate::library::midi::*;
use crate::library::os::*;
use crate::library::package::*;
use crate::library::requests::*;
use crate::library::sh::*;
use crate::library::sqlite::*;
use crate::library::string::*;
use crate::library::urllib::*;
use crate::library::toml::*;
use crate::library::table::*;
use crate::library::tui::*;
use crate::library::utf8::*;
use crate::registeredfunction::*;
pub const LOADED_FUNCTIONS: [RegisteredFunction; 18] = {
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
        {
            RegisteredFunction {
                registeredfunction_name: c"toml".as_ptr(),
                registeredfunction_function: Some(luaopen_toml as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"urllib".as_ptr(),
                registeredfunction_function: Some(luaopen_urllib as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"sqlite".as_ptr(),
                registeredfunction_function: Some(luaopen_sqlite as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"json".as_ptr(),
                registeredfunction_function: Some(luaopen_json as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"requests".as_ptr(),
                registeredfunction_function: Some(luaopen_requests as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"tui".as_ptr(),
                registeredfunction_function: Some(luaopen_tui as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"midi".as_ptr(),
                registeredfunction_function: Some(luaopen_midi as unsafe fn(*mut Interpreter) -> i32),
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
