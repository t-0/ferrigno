#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types)]
unsafe extern "C" {
    pub type lua_State;
    fn lua_settop(L: *mut lua_State, index: libc::c_int);
    fn luaopen_base(L: *mut lua_State) -> libc::c_int;
    fn luaopen_coroutine(L: *mut lua_State) -> libc::c_int;
    fn luaopen_table(L: *mut lua_State) -> libc::c_int;
    fn luaopen_io(L: *mut lua_State) -> libc::c_int;
    fn luaopen_os(L: *mut lua_State) -> libc::c_int;
    fn luaopen_string(L: *mut lua_State) -> libc::c_int;
    fn luaopen_utf8(L: *mut lua_State) -> libc::c_int;
    fn luaopen_math(L: *mut lua_State) -> libc::c_int;
    fn luaopen_debug(L: *mut lua_State) -> libc::c_int;
    fn luaopen_package(L: *mut lua_State) -> libc::c_int;
    fn luaL_requiref(
        L: *mut lua_State,
        modname: *const libc::c_char,
        openf: lua_CFunction,
        glb: libc::c_int,
    );
}
pub type lua_CFunction = Option::<unsafe extern "C" fn(*mut lua_State) -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaL_Reg {
    pub name: *const libc::c_char,
    pub func: lua_CFunction,
}
static mut loadedlibs: [luaL_Reg; 11] = unsafe {
    [
        {
            let mut init = luaL_Reg {
                name: b"_G\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaopen_base as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"package\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaopen_package
                        as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"coroutine\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaopen_coroutine
                        as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"table\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaopen_table as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"io\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaopen_io as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"os\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaopen_os as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"string\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaopen_string as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"math\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaopen_math as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"utf8\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaopen_utf8 as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"debug\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaopen_debug as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: 0 as *const libc::c_char,
                func: None,
            };
            init
        },
    ]
};
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaL_openlibs(mut L: *mut lua_State) {
    let mut lib: *const luaL_Reg = 0 as *const luaL_Reg;
    lib = loadedlibs.as_ptr();
    while ((*lib).func).is_some() {
        luaL_requiref(L, (*lib).name, (*lib).func, 1 as libc::c_int);
        lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
        lib = lib.offset(1);
        lib;
    }
}
