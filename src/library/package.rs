use crate::c::*;
use crate::state::*;
use crate::new::*;
use crate::tag::*;
use crate::registeredfunction::*;
use crate::buffer::*;
use crate::functions::*;
use libc::{dlopen, dlclose, dlsym, dlerror,};
pub const CLIBS: *const i8 = b"_CLIBS\0" as *const u8 as *const i8;
pub unsafe extern "C" fn lsys_unloadlib(lib: *mut libc::c_void) {
    unsafe {
        dlclose(lib);
    }
}
pub unsafe extern "C" fn lsys_load(
    state: *mut State,
    path: *const i8,
    seeglb: i32,
) -> *mut libc::c_void {
    unsafe {
        let lib: *mut libc::c_void = dlopen(
            path,
            0x2 as i32 | (if seeglb != 0 { 0x100 as i32 } else { 0 }),
        );
        if ((lib == std::ptr::null_mut()) as i32 != 0) as i64 != 0 {
            lua_pushstring(state, dlerror());
        }
        return lib;
    }
}
pub unsafe extern "C" fn lsys_sym(
    state: *mut State,
    lib: *mut libc::c_void,
    sym: *const i8,
) -> CFunction {
    unsafe {
        let f: CFunction = ::core::mem::transmute::<*mut libc::c_void, CFunction>(dlsym(lib, sym));
        if (f.is_none() as i32 != 0) as i64 != 0 {
            lua_pushstring(state, dlerror());
        }
        return f;
    }
}
pub unsafe extern "C" fn noenv(state: *mut State) -> i32 {
    unsafe {
        let b: i32;
        lua_getfield(
            state,
            -(1000000 as i32) - 1000 as i32,
            b"LUA_NOENV\0" as *const u8 as *const i8,
        );
        b = lua_toboolean(state, -1);
        lua_settop(state, -2);
        return b;
    }
}
pub unsafe extern "C" fn setpath(
    state: *mut State,
    fieldname: *const i8,
    envname: *const i8,
    dft: *const i8,
) {
    unsafe {
        let dftmark: *const i8;
        let nver: *const i8 = lua_pushfstring(
            state,
            b"%s%s\0" as *const u8 as *const i8,
            envname,
            b"_5_4\0" as *const u8 as *const i8,
        );
        let mut path: *const i8 = getenv(nver);
        if path.is_null() {
            path = getenv(envname);
        }
        if path.is_null() || noenv(state) != 0 {
            lua_pushstring(state, dft);
        } else {
            dftmark = strstr(path, b";;\0" as *const u8 as *const i8);
            if dftmark.is_null() {
                lua_pushstring(state, path);
            } else {
                let length: u64 = strlen(path);
                let mut b = Buffer::new();
                b.lual_buffinit(state);
                if path < dftmark {
                    b.lual_addlstring(path, dftmark.offset_from(path) as u64);
                    (b.length < b.size || !(b.lual_prepbuffsize(1 as u64)).is_null()) as i32;
                    let fresh193 = b.length;
                    b.length = (b.length).wrapping_add(1);
                    *(b.pointer).offset(fresh193 as isize) = *(b";\0" as *const u8 as *const i8);
                }
                b.lual_addstring(dft);
                if dftmark < path.offset(length as isize).offset(-(2 as isize)) {
                    (b.length < b.size || !(b.lual_prepbuffsize(1 as u64)).is_null()) as i32;
                    let fresh194 = b.length;
                    b.length = (b.length).wrapping_add(1);
                    *(b.pointer).offset(fresh194 as isize) = *(b";\0" as *const u8 as *const i8);
                    b.lual_addlstring(
                        dftmark.offset(2 as isize),
                        path.offset(length as isize)
                            .offset(-(2 as isize))
                            .offset_from(dftmark) as u64,
                    );
                }
                b.lual_pushresult();
            }
        }
        lua_setfield(state, -3, fieldname);
        lua_settop(state, -2);
    }
}
pub unsafe extern "C" fn checkclib(state: *mut State, path: *const i8) -> *mut libc::c_void {
    unsafe {
        let plib: *mut libc::c_void;
        lua_getfield(state, -(1000000 as i32) - 1000 as i32, CLIBS);
        lua_getfield(state, -1, path);
        plib = lua_touserdata(state, -1);
        lua_settop(state, -2 - 1);
        return plib;
    }
}
pub unsafe extern "C" fn addtoclib(state: *mut State, path: *const i8, plib: *mut libc::c_void) {
    unsafe {
        lua_getfield(state, -(1000000 as i32) - 1000 as i32, CLIBS);
        lua_pushlightuserdata(state, plib);
        lua_pushvalue(state, -1);
        lua_setfield(state, -3, path);
        lua_rawseti(state, -2, lual_len(state, -2) + 1);
        lua_settop(state, -2);
    }
}
pub unsafe extern "C" fn gctm(state: *mut State) -> i32 {
    unsafe {
        let mut n: i64 = lual_len(state, 1);
        while n >= 1 {
            lua_rawgeti(state, 1, n);
            lsys_unloadlib(lua_touserdata(state, -1));
            lua_settop(state, -2);
            n -= 1;
        }
        return 0;
    }
}
pub unsafe extern "C" fn lookforfunc(state: *mut State, path: *const i8, sym: *const i8) -> i32 {
    unsafe {
        let mut reg: *mut libc::c_void = checkclib(state, path);
        if reg.is_null() {
            reg = lsys_load(state, path, (*sym as i32 == '*' as i32) as i32);
            if reg.is_null() {
                return 1;
            }
            addtoclib(state, path, reg);
        }
        if *sym as i32 == '*' as i32 {
            (*state).push_boolean(true);
            return 0;
        } else {
            let f: CFunction = lsys_sym(state, reg, sym);
            if f.is_none() {
                return 2;
            }
            lua_pushcclosure(state, f, 0);
            return 0;
        };
    }
}
pub unsafe extern "C" fn ll_loadlib(state: *mut State) -> i32 {
    unsafe {
        let path: *const i8 = lual_checklstring(state, 1, std::ptr::null_mut());
        let init: *const i8 = lual_checklstring(state, 2, std::ptr::null_mut());
        let stat: i32 = lookforfunc(state, path, init);
        if ((stat == 0) as i32 != 0) as i64 != 0 {
            return 1;
        } else {
            (*state).push_nil();
            lua_rotate(state, -2, 1);
            lua_pushstring(
                state,
                if stat == 1 {
                    b"open\0" as *const u8 as *const i8
                } else {
                    b"init\0" as *const u8 as *const i8
                },
            );
            return 3;
        };
    }
}
pub unsafe extern "C" fn readable(filename: *const i8) -> i32 {
    unsafe {
        let f: *mut FILE = fopen(filename, b"r\0" as *const u8 as *const i8);
        if f.is_null() {
            return 0;
        }
        fclose(f);
        return 1;
    }
}
pub unsafe extern "C" fn getnextfilename(path: *mut *mut i8, end: *mut i8) -> *const i8 {
    unsafe {
        let mut name: *mut i8 = *path;
        if name == end {
            return std::ptr::null();
        } else if *name as i32 == '\0' as i32 {
            *name = *(b";\0" as *const u8 as *const i8);
            name = name.offset(1);
        }
        let mut sep: *mut i8 = strchr(name, *(b";\0" as *const u8 as *const i8) as i32);
        if sep.is_null() {
            sep = end;
        }
        *sep = '\0' as i8;
        *path = sep;
        return name;
    }
}
pub unsafe extern "C" fn pusherrornotfound(state: *mut State, path: *const i8) {
    unsafe {
        let mut b = Buffer::new();
        b.lual_buffinit(state);
        b.lual_addstring(b"no file '\0" as *const u8 as *const i8);
        lual_addgsub(
            &mut b,
            path,
            b";\0" as *const u8 as *const i8,
            b"'\n\tno file '\0" as *const u8 as *const i8,
        );
        b.lual_addstring(b"'\0" as *const u8 as *const i8);
        b.lual_pushresult();
    }
}
pub unsafe extern "C" fn searchpath(
    state: *mut State,
    mut name: *const i8,
    path: *const i8,
    sep: *const i8,
    dirsep: *const i8,
) -> *const i8 {
    unsafe {
        let mut pathname;
        let endpathname;
        let mut filename;
        if *sep as i32 != '\0' as i32 && !(strchr(name, *sep as i32)).is_null() {
            name = lual_gsub(state, name, sep, dirsep);
        }
        let mut buffer = Buffer::new();
        buffer.lual_buffinit(state);
        lual_addgsub(&mut buffer, path, b"?\0" as *const u8 as *const i8, name);
        (buffer.length < buffer.size || !(buffer.lual_prepbuffsize(1 as u64)).is_null()) as i32;
        let fresh195 = buffer.length;
        buffer.length = (buffer.length).wrapping_add(1);
        *(buffer.pointer).offset(fresh195 as isize) = '\0' as i8;
        pathname = buffer.pointer;
        endpathname = pathname
            .offset(buffer.length as isize)
            .offset(-(1 as isize));
        loop {
            filename = getnextfilename(&mut pathname, endpathname);
            if filename.is_null() {
                break;
            }
            if readable(filename) != 0 {
                return lua_pushstring(state, filename);
            }
        }
        buffer.lual_pushresult();
        pusherrornotfound(state, lua_tolstring(state, -1, std::ptr::null_mut()));
        return std::ptr::null();
    }
}
pub unsafe extern "C" fn ll_searchpath(state: *mut State) -> i32 {
    unsafe {
        let f: *const i8 = searchpath(
            state,
            lual_checklstring(state, 1, std::ptr::null_mut()),
            lual_checklstring(state, 2, std::ptr::null_mut()),
            lual_optlstring(
                state,
                3,
                b".\0" as *const u8 as *const i8,
                std::ptr::null_mut(),
            ),
            lual_optlstring(
                state,
                4,
                b"/\0" as *const u8 as *const i8,
                std::ptr::null_mut(),
            ),
        );
        if !f.is_null() {
            return 1;
        } else {
            (*state).push_nil();
            lua_rotate(state, -2, 1);
            return 2;
        };
    }
}
pub unsafe extern "C" fn findfile(
    state: *mut State,
    name: *const i8,
    pname: *const i8,
    dirsep: *const i8,
) -> *const i8 {
    unsafe {
        lua_getfield(state, -(1000000 as i32) - 1000 as i32 - 1, pname);
        let path: *const i8 = lua_tolstring(state, -1, std::ptr::null_mut());
        if ((path == std::ptr::null_mut() as *const i8) as i32 != 0) as i64 != 0 {
            lual_error(
                state,
                b"'package.%s' must be a string\0" as *const u8 as *const i8,
                pname,
            );
        }
        return searchpath(state, name, path, b".\0" as *const u8 as *const i8, dirsep);
    }
}
pub unsafe extern "C" fn checkload(state: *mut State, stat: i32, filename: *const i8) -> i32 {
    unsafe {
        if (stat != 0) as i64 != 0 {
            lua_pushstring(state, filename);
            return 2;
        } else {
            return lual_error(
                state,
                b"error loading module '%s' from file '%s':\n\t%s\0" as *const u8 as *const i8,
                lua_tolstring(state, 1, std::ptr::null_mut()),
                filename,
                lua_tolstring(state, -1, std::ptr::null_mut()),
            );
        };
    }
}
pub unsafe extern "C" fn searcher_lua(state: *mut State) -> i32 {
    unsafe {
        let name: *const i8 = lual_checklstring(state, 1, std::ptr::null_mut());
        let filename: *const i8 = findfile(
            state,
            name,
            b"path\0" as *const u8 as *const i8,
            b"/\0" as *const u8 as *const i8,
        );
        if filename.is_null() {
            return 1;
        }
        return checkload(
            state,
            (lual_loadfilex(state, filename, std::ptr::null()) == 0) as i32,
            filename,
        );
    }
}
pub unsafe extern "C" fn loadfunc(
    state: *mut State,
    filename: *const i8,
    mut modname: *const i8,
) -> i32 {
    unsafe {
        modname = lual_gsub(
            state,
            modname,
            b".\0" as *const u8 as *const i8,
            b"_\0" as *const u8 as *const i8,
        );
        let mut openfunc: *const i8;
        let mark: *const i8 = strchr(modname, *(b"-\0" as *const u8 as *const i8) as i32);
        if !mark.is_null() {
            openfunc = lua_pushlstring(state, modname, mark.offset_from(modname) as u64);
            openfunc = lua_pushfstring(state, b"luaopen_%s\0" as *const u8 as *const i8, openfunc);
            let stat: i32 = lookforfunc(state, filename, openfunc);
            if stat != 2 {
                return stat;
            }
            modname = mark.offset(1 as isize);
        }
        openfunc = lua_pushfstring(state, b"luaopen_%s\0" as *const u8 as *const i8, modname);
        return lookforfunc(state, filename, openfunc);
    }
}
pub unsafe extern "C" fn searcher_c(state: *mut State) -> i32 {
    unsafe {
        let name: *const i8 = lual_checklstring(state, 1, std::ptr::null_mut());
        let filename: *const i8 = findfile(
            state,
            name,
            b"cpath\0" as *const u8 as *const i8,
            b"/\0" as *const u8 as *const i8,
        );
        if filename.is_null() {
            return 1;
        }
        return checkload(
            state,
            (loadfunc(state, filename, name) == 0) as i32,
            filename,
        );
    }
}
pub unsafe extern "C" fn searcher_croot(state: *mut State) -> i32 {
    unsafe {
        let name: *const i8 = lual_checklstring(state, 1, std::ptr::null_mut());
        let p: *const i8 = strchr(name, '.' as i32);
        if p.is_null() {
            return 0;
        }
        lua_pushlstring(state, name, p.offset_from(name) as u64);
        let filename: *const i8 = findfile(
            state,
            lua_tolstring(state, -1, std::ptr::null_mut()),
            b"cpath\0" as *const u8 as *const i8,
            b"/\0" as *const u8 as *const i8,
        );
        if filename.is_null() {
            return 1;
        }
        let stat: i32 = loadfunc(state, filename, name);
        if stat != 0 {
            if stat != 2 {
                return checkload(state, 0, filename);
            } else {
                lua_pushfstring(
                    state,
                    b"no module '%s' in file '%s'\0" as *const u8 as *const i8,
                    name,
                    filename,
                );
                return 1;
            }
        }
        lua_pushstring(state, filename);
        return 2;
    }
}
pub unsafe extern "C" fn searcher_preload(state: *mut State) -> i32 {
    unsafe {
        let name: *const i8 = lual_checklstring(state, 1, std::ptr::null_mut());
        lua_getfield(
            state,
            -(1000000 as i32) - 1000 as i32,
            b"_PRELOAD\0" as *const u8 as *const i8,
        );
        if lua_getfield(state, -1, name) == 0 {
            lua_pushfstring(
                state,
                b"no field package.preload['%s']\0" as *const u8 as *const i8,
                name,
            );
            return 1;
        } else {
            lua_pushstring(state, b":preload:\0" as *const u8 as *const i8);
            return 2;
        };
    }
}
pub unsafe extern "C" fn findloader(state: *mut State, name: *const i8) {
    unsafe {
        let mut i: i32;
        let mut message = Buffer::new();
        if ((lua_getfield(
            state,
            -(1000000 as i32) - 1000 as i32 - 1,
            b"searchers\0" as *const u8 as *const i8,
        ) != 5) as i32
            != 0) as i64
            != 0
        {
            lual_error(
                state,
                b"'package.searchers' must be a table\0" as *const u8 as *const i8,
            );
        }
        message.lual_buffinit(state);
        i = 1;
        loop {
            message.lual_addstring(b"\n\t\0" as *const u8 as *const i8);
            if ((lua_rawgeti(state, 3, i as i64) == 0) as i32 != 0) as i64 != 0 {
                lua_settop(state, -2);
                message.length = (message.length as u64).wrapping_sub(2 as u64) as u64;
                message.lual_pushresult();
                lual_error(
                    state,
                    b"module '%s' not found:%s\0" as *const u8 as *const i8,
                    name,
                    lua_tolstring(state, -1, std::ptr::null_mut()),
                );
            }
            lua_pushstring(state, name);
            lua_callk(state, 1, 2, 0, None);
            if lua_type(state, -2) == Some(TAG_TYPE_CLOSURE) {
                return;
            } else if lua_isstring(state, -2) {
                lua_settop(state, -2);
                message.lual_addvalue();
            } else {
                lua_settop(state, -2 - 1);
                message.length = (message.length as u64).wrapping_sub(2 as u64) as u64;
            }
            i += 1;
        }
    }
}
pub unsafe extern "C" fn ll_require(state: *mut State) -> i32 {
    unsafe {
        let name: *const i8 = lual_checklstring(state, 1, std::ptr::null_mut());
        lua_settop(state, 1);
        lua_getfield(
            state,
            -(1000000 as i32) - 1000 as i32,
            b"_LOADED\0" as *const u8 as *const i8,
        );
        lua_getfield(state, 2, name);
        if lua_toboolean(state, -1) != 0 {
            return 1;
        }
        lua_settop(state, -2);
        findloader(state, name);
        lua_rotate(state, -2, 1);
        lua_pushvalue(state, 1);
        lua_pushvalue(state, -3);
        lua_callk(state, 2, 1, 0, None);
        if !(lua_type(state, -1) == Some(TAG_TYPE_NIL)) {
            lua_setfield(state, 2, name);
        } else {
            lua_settop(state, -2);
        }
        if lua_getfield(state, 2, name) == 0 {
            (*state).push_boolean(true);
            lua_copy(state, -1, -2);
            lua_setfield(state, 2, name);
        }
        lua_rotate(state, -2, 1);
        return 2;
    }
}
pub const PACKAGE_FUNCTIONS: [RegisteredFunction; 8] = {
    [
        {
            RegisteredFunction {
                name: b"loadlib\0" as *const u8 as *const i8,
                function: Some(ll_loadlib as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"searchpath\0" as *const u8 as *const i8,
                function: Some(ll_searchpath as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"preload\0" as *const u8 as *const i8,
                function: None,
            }
        },
        {
            RegisteredFunction {
                name: b"cpath\0" as *const u8 as *const i8,
                function: None,
            }
        },
        {
            RegisteredFunction {
                name: b"path\0" as *const u8 as *const i8,
                function: None,
            }
        },
        {
            RegisteredFunction {
                name: b"searchers\0" as *const u8 as *const i8,
                function: None,
            }
        },
        {
            RegisteredFunction {
                name: b"loaded\0" as *const u8 as *const i8,
                function: None,
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
pub const LL_FUNCTIONS: [RegisteredFunction; 2] = {
    [
        {
            RegisteredFunction {
                name: b"require\0" as *const u8 as *const i8,
                function: Some(ll_require as unsafe extern "C" fn(*mut State) -> i32),
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
pub unsafe extern "C" fn createsearcherstable(state: *mut State) {
    unsafe {
        pub const SEARCHERS: [CFunction; 5] = {
            [
                Some(searcher_preload as unsafe extern "C" fn(*mut State) -> i32),
                Some(searcher_lua as unsafe extern "C" fn(*mut State) -> i32),
                Some(searcher_c as unsafe extern "C" fn(*mut State) -> i32),
                Some(searcher_croot as unsafe extern "C" fn(*mut State) -> i32),
                None,
            ]
        };
        let mut i: i32;
        (*state).lua_createtable();
        i = 0;
        while (SEARCHERS[i as usize]).is_some() {
            lua_pushvalue(state, -2);
            lua_pushcclosure(state, SEARCHERS[i as usize], 1);
            lua_rawseti(state, -2, (i + 1) as i64);
            i += 1;
        }
        lua_setfield(state, -2, b"searchers\0" as *const u8 as *const i8);
    }
}
pub unsafe extern "C" fn createclibstable(state: *mut State) {
    unsafe {
        lual_getsubtable(state, -(1000000 as i32) - 1000 as i32, CLIBS);
        (*state).lua_createtable();
        lua_pushcclosure(
            state,
            Some(gctm as unsafe extern "C" fn(*mut State) -> i32),
            0,
        );
        lua_setfield(state, -2, b"__gc\0" as *const u8 as *const i8);
        lua_setmetatable(state, -2);
    }
}
pub unsafe extern "C" fn luaopen_package(state: *mut State) -> i32 {
    unsafe {
        createclibstable(state);
        lual_checkversion_(
            state,
            504.0,
            (::core::mem::size_of::<i64>() as u64)
                .wrapping_mul(16 as u64)
                .wrapping_add(::core::mem::size_of::<f64>() as u64),
        );
        (*state).lua_createtable();
        lual_setfuncs(state, PACKAGE_FUNCTIONS.as_ptr(), 0);
        createsearcherstable(state);
        setpath(
        state,
        b"path\0" as *const u8 as *const i8,
        b"LUA_PATH\0" as *const u8 as *const i8,
        b"/usr/local/share/lua/5.4/?.lua;/usr/local/share/lua/5.4/?/init.lua;/usr/local/lib/lua/5.4/?.lua;/usr/local/lib/lua/5.4/?/init.lua;./?.lua;./?/init.lua\0"
            as *const u8 as *const i8,
    );
        setpath(
            state,
            b"cpath\0" as *const u8 as *const i8,
            b"LUA_CPATH\0" as *const u8 as *const i8,
            b"/usr/local/lib/lua/5.4/?.so;/usr/local/lib/lua/5.4/loadall.so;./?.so\0" as *const u8
                as *const i8,
        );
        lua_pushstring(state, b"/\n;\n?\n!\n-\n\0" as *const u8 as *const i8);
        lua_setfield(state, -2, b"config\0" as *const u8 as *const i8);
        lual_getsubtable(
            state,
            -(1000000 as i32) - 1000 as i32,
            b"_LOADED\0" as *const u8 as *const i8,
        );
        lua_setfield(state, -2, b"loaded\0" as *const u8 as *const i8);
        lual_getsubtable(
            state,
            -(1000000 as i32) - 1000 as i32,
            b"_PRELOAD\0" as *const u8 as *const i8,
        );
        lua_setfield(state, -2, b"preload\0" as *const u8 as *const i8);
        lua_rawgeti(state, -(1000000 as i32) - 1000 as i32, 2 as i64);
        lua_pushvalue(state, -2);
        lual_setfuncs(state, LL_FUNCTIONS.as_ptr(), 1);
        lua_settop(state, -2);
        return 1;
    }
}
