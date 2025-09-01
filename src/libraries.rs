use crate::onelua::*;
use crate::registeredfunction::*;
use crate::state::*;
use crate::coroutine::*;
use crate::debuginfo::*;
use crate::librarymath::*;
use crate::librarybase::*;
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
