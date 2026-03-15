mod base;
mod coroutine;
mod debug;
mod dis;
mod fmath;
mod functools;
mod io;
mod itertools;
pub mod json;
mod math;
mod midi;
mod os;
mod package;
mod requests;
mod sh;
mod sqlite;
mod string;
mod table;
mod toml;
mod tui;
mod urllib;
mod utf8;
use crate::library::base::*;
use crate::library::coroutine::*;
use crate::library::debug::*;
use crate::library::dis::*;
use crate::library::fmath::*;
use crate::library::functools::*;
use crate::library::io::*;
use crate::library::itertools::*;
use crate::library::json::*;
use crate::library::math::*;
use crate::library::midi::*;
use crate::library::os::*;
use crate::library::package::*;
use crate::library::requests::*;
use crate::library::sh::*;
use crate::library::sqlite::*;
use crate::library::string::*;
use crate::library::table::*;
use crate::library::toml::*;
use crate::library::tui::*;
use crate::library::urllib::*;
use crate::library::utf8::*;
use crate::registeredfunction::*;
use crate::state::*;
// Library selection bitmask constants (standard Lua 5.5 libraries)
pub const LUA_GLIBK: i32 = 1;
pub const LUA_LOADLIBK: i32 = LUA_GLIBK << 1;
pub const LUA_COLIBK: i32 = LUA_LOADLIBK << 1;
pub const LUA_TABLIBK: i32 = LUA_COLIBK << 1;
pub const LUA_IOLIBK: i32 = LUA_TABLIBK << 1;
pub const LUA_OSLIBK: i32 = LUA_IOLIBK << 1;
pub const LUA_STRLIBK: i32 = LUA_OSLIBK << 1;
pub const LUA_MATHLIBK: i32 = LUA_STRLIBK << 1;
pub const LUA_UTF8LIBK: i32 = LUA_MATHLIBK << 1;
pub const LUA_DBLIBK: i32 = LUA_UTF8LIBK << 1;
pub const LOADED_FUNCTIONS: [RegisteredFunction; 22] = {
    [
        {
            RegisteredFunction {
                registeredfunction_name: c"_G".as_ptr(),
                registeredfunction_function: Some(luaopen_base as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"package".as_ptr(),
                registeredfunction_function: Some(luaopen_package as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"coroutine".as_ptr(),
                registeredfunction_function: Some(luaopen_coroutine as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"table".as_ptr(),
                registeredfunction_function: Some(luaopen_table as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"io".as_ptr(),
                registeredfunction_function: Some(luaopen_io as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"os".as_ptr(),
                registeredfunction_function: Some(luaopen_os as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"string".as_ptr(),
                registeredfunction_function: Some(luaopen_string as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"math".as_ptr(),
                registeredfunction_function: Some(luaopen_math as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"utf8".as_ptr(),
                registeredfunction_function: Some(luaopen_utf8 as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"debug".as_ptr(),
                registeredfunction_function: Some(luaopen_debug as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"sh".as_ptr(),
                registeredfunction_function: Some(luaopen_sh as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"toml".as_ptr(),
                registeredfunction_function: Some(luaopen_toml as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"urllib".as_ptr(),
                registeredfunction_function: Some(luaopen_urllib as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"sqlite".as_ptr(),
                registeredfunction_function: Some(luaopen_sqlite as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"json".as_ptr(),
                registeredfunction_function: Some(luaopen_json as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"requests".as_ptr(),
                registeredfunction_function: Some(luaopen_requests as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"tui".as_ptr(),
                registeredfunction_function: Some(luaopen_tui as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"midi".as_ptr(),
                registeredfunction_function: Some(luaopen_midi as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"dis".as_ptr(),
                registeredfunction_function: Some(luaopen_dis as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"functools".as_ptr(),
                registeredfunction_function: Some(luaopen_functools as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"fmath".as_ptr(),
                registeredfunction_function: Some(luaopen_fmath as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"itertools".as_ptr(),
                registeredfunction_function: Some(luaopen_itertools as unsafe fn(*mut State) -> i32),
            }
        },
    ]
};
pub unsafe fn lual_openlibs(state: *mut State) {
    unsafe {
        lual_openselectedlibs(state, !0, 0);
    }
}
/// Standard library entries for bitmask-based selection.
/// Order matches bitmask constants: _G, package, coroutine, table, io, os, string, math, utf8, debug.
const STDLIBS: [RegisteredFunction; 10] = [
    RegisteredFunction {
        registeredfunction_name: c"_G".as_ptr(),
        registeredfunction_function: Some(luaopen_base as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"package".as_ptr(),
        registeredfunction_function: Some(luaopen_package as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"coroutine".as_ptr(),
        registeredfunction_function: Some(luaopen_coroutine as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"table".as_ptr(),
        registeredfunction_function: Some(luaopen_table as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"io".as_ptr(),
        registeredfunction_function: Some(luaopen_io as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"os".as_ptr(),
        registeredfunction_function: Some(luaopen_os as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"string".as_ptr(),
        registeredfunction_function: Some(luaopen_string as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"math".as_ptr(),
        registeredfunction_function: Some(luaopen_math as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"utf8".as_ptr(),
        registeredfunction_function: Some(luaopen_utf8 as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"debug".as_ptr(),
        registeredfunction_function: Some(luaopen_debug as unsafe fn(*mut State) -> i32),
    },
];
pub unsafe fn lual_openselectedlibs(state: *mut State, load: i32, preload: i32) {
    unsafe {
        lual_getsubtable(state, LUA_REGISTRYINDEX, c"_PRELOAD".as_ptr());
        let mut mask: i32 = 1;
        for lib in &STDLIBS {
            if load & mask != 0 {
                lual_requiref(state, lib.registeredfunction_name, lib.registeredfunction_function, 1);
                lua_settop(state, -2);
            } else if preload & mask != 0 {
                lua_pushcclosure(state, lib.registeredfunction_function, 0);
                lua_setfield(state, -2, lib.registeredfunction_name);
            }
            mask <<= 1;
        }
        lua_settop(state, -2);
        // Load custom (non-standard) libraries when load has all bits set
        if load == !0 {
            for it in &LOADED_FUNCTIONS[10..] {
                lual_requiref(state, it.registeredfunction_name, it.registeredfunction_function, 1);
                lua_settop(state, -2);
            }
        }
    }
}
