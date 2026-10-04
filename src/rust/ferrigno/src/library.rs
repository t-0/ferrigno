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
#[cfg(feature = "midi")]
mod midi;
mod os;
mod package;
#[cfg(feature = "requests")]
mod requests;
mod sh;
#[cfg(feature = "sqlite")]
mod sqlite;
mod string;
mod table;
mod toml;
#[cfg(feature = "tui")]
mod tui;
#[cfg(feature = "urllib")]
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
#[cfg(feature = "midi")]
use crate::library::midi::*;
use crate::library::os::*;
use crate::library::package::*;
#[cfg(feature = "requests")]
use crate::library::requests::*;
use crate::library::sh::*;
#[cfg(feature = "sqlite")]
use crate::library::sqlite::*;
use crate::library::string::*;
use crate::library::table::*;
use crate::library::toml::*;
#[cfg(feature = "tui")]
use crate::library::tui::*;
#[cfg(feature = "urllib")]
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
macro_rules! library {
    ($name:literal, $open:ident) => {
        RegisteredFunction {
            registeredfunction_name: $name.as_ptr(),
            registeredfunction_function: Some($open as unsafe fn(*mut State) -> i32),
        }
    };
}
/// Non-standard libraries shipped with ferrigno, in load order.
///
/// Libraries that link native system libraries are gated behind cargo
/// features and only appear here when their feature is enabled.
fn extension_libraries() -> Vec<RegisteredFunction> {
    let mut libraries = vec![library!(c"sh", luaopen_sh), library!(c"toml", luaopen_toml)];
    #[cfg(feature = "urllib")]
    libraries.push(library!(c"urllib", luaopen_urllib));
    #[cfg(feature = "sqlite")]
    libraries.push(library!(c"sqlite", luaopen_sqlite));
    libraries.push(library!(c"json", luaopen_json));
    #[cfg(feature = "requests")]
    libraries.push(library!(c"requests", luaopen_requests));
    #[cfg(feature = "tui")]
    libraries.push(library!(c"tui", luaopen_tui));
    #[cfg(feature = "midi")]
    libraries.push(library!(c"midi", luaopen_midi));
    libraries.push(library!(c"dis", luaopen_dis));
    libraries.push(library!(c"functools", luaopen_functools));
    libraries.push(library!(c"fmath", luaopen_fmath));
    libraries.push(library!(c"itertools", luaopen_itertools));
    libraries
}
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
                lual_requiref(
                    state,
                    lib.registeredfunction_name,
                    lib.registeredfunction_function,
                    1,
                );
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
            for it in &extension_libraries() {
                lual_requiref(
                    state,
                    it.registeredfunction_name,
                    it.registeredfunction_function,
                    1,
                );
                lua_settop(state, -2);
            }
        }
    }
}
