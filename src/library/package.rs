use std::ptr::*;
use crate::utility::c::*;
use crate::interpreter::*;
use crate::character::*;
use crate::new::*;
use crate::tag::*;
use crate::registeredfunction::*;
use crate::buffer::*;
use crate::functions::*;
use libc::{dlopen, dlclose, dlsym, dlerror,};
pub const CLIBS: *const libc::c_char = b"_CLIBS\0" as *const u8 as *const libc::c_char;
pub unsafe extern "C" fn lsys_unloadlib(lib: *mut libc::c_void) {
    unsafe {
        dlclose(lib);
    }
}
pub unsafe extern "C" fn lsys_load(
    interpreter: *mut Interpreter,
    path: *const libc::c_char,
    seeglb: i32,
) -> *mut libc::c_void {
    unsafe {
        let lib: *mut libc::c_void = dlopen(
            path,
            0x2 as i32 | (if seeglb != 0 { 0x100 as i32 } else { 0 }),
        );
        if lib.is_null() {
            lua_pushstring(interpreter, dlerror());
        }
        return lib;
    }
}
pub unsafe extern "C" fn lsys_sym(
    interpreter: *mut Interpreter,
    lib: *mut libc::c_void,
    sym: *const libc::c_char,
) -> CFunction {
    unsafe {
        let cfunction: CFunction = ::core::mem::transmute::<*mut libc::c_void, CFunction>(dlsym(lib, sym));
        if cfunction.is_none() {
            lua_pushstring(interpreter, dlerror());
        }
        return cfunction;
    }
}
pub unsafe extern "C" fn noenv(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let b: i32;
        lua_getfield(
            interpreter,
            -(1000000 as i32) - 1000 as i32,
            b"LUA_NOENV\0" as *const u8 as *const libc::c_char,
        );
        b = lua_toboolean(interpreter, -1);
        lua_settop(interpreter, -2);
        return b;
    }
}
pub unsafe extern "C" fn setpath(
    interpreter: *mut Interpreter,
    fieldname: *const libc::c_char,
    envname: *const libc::c_char,
    dft: *const libc::c_char,
) {
    unsafe {
        let dftmark: *const libc::c_char;
        let nver: *const libc::c_char = lua_pushfstring(
            interpreter,
            b"%s%s\0" as *const u8 as *const libc::c_char,
            envname,
            b"_5_4\0" as *const u8 as *const libc::c_char,
        );
        let mut path: *const libc::c_char = getenv(nver);
        if path.is_null() {
            path = getenv(envname);
        }
        if path.is_null() || noenv(interpreter) != 0 {
            lua_pushstring(interpreter, dft);
        } else {
            dftmark = strstr(path, b";;\0" as *const u8 as *const libc::c_char);
            if dftmark.is_null() {
                lua_pushstring(interpreter, path);
            } else {
                let length: usize = strlen(path) as usize;
                let mut b = Buffer::new();
                b.initialize(interpreter);
                if path < dftmark {
                    b.add_string_with_length(path, dftmark.offset_from(path) as usize);
                    (b.vector.length < b.vector.size || !(b.prepare_with_size(1)).is_null()) as i32;
                    let fresh193 = b.vector.length;
                    b.vector.length = (b.vector.length).wrapping_add(1);
                    *(b.vector.pointer).offset(fresh193 as isize) = *(b";\0" as *const u8 as *const libc::c_char);
                }
                b.add_string(dft);
                if dftmark < path.offset(length as isize).offset(-(2 as isize)) {
                    (b.vector.length < b.vector.size || !(b.prepare_with_size(1)).is_null()) as i32;
                    let fresh194 = b.vector.length;
                    b.vector.length = (b.vector.length).wrapping_add(1);
                    *(b.vector.pointer).offset(fresh194 as isize) = *(b";\0" as *const u8 as *const libc::c_char);
                    b.add_string_with_length(
                        dftmark.offset(2 as isize),
                        path.offset(length as isize)
                            .offset(-(2 as isize))
                            .offset_from(dftmark) as usize,
                    );
                }
                b.push_result();
            }
        }
        lua_setfield(interpreter, -3, fieldname);
        lua_settop(interpreter, -2);
    }
}
pub unsafe extern "C" fn checkclib(interpreter: *mut Interpreter, path: *const libc::c_char) -> *mut libc::c_void {
    unsafe {
        let plib: *mut libc::c_void;
        lua_getfield(interpreter, -(1000000 as i32) - 1000 as i32, CLIBS);
        lua_getfield(interpreter, -1, path);
        plib = lua_touserdata(interpreter, -1);
        lua_settop(interpreter, -2 - 1);
        return plib;
    }
}
pub unsafe extern "C" fn addtoclib(interpreter: *mut Interpreter, path: *const libc::c_char, plib: *mut libc::c_void) {
    unsafe {
        lua_getfield(interpreter, -(1000000 as i32) - 1000 as i32, CLIBS);
        lua_pushlightuserdata(interpreter, plib);
        lua_pushvalue(interpreter, -1);
        lua_setfield(interpreter, -3, path);
        lua_rawseti(interpreter, -2, lual_len(interpreter, -2) + 1);
        lua_settop(interpreter, -2);
    }
}
pub unsafe extern "C" fn gctm(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let mut n: i64 = lual_len(interpreter, 1);
        while n >= 1 {
            lua_rawgeti(interpreter, 1, n);
            lsys_unloadlib(lua_touserdata(interpreter, -1));
            lua_settop(interpreter, -2);
            n -= 1;
        }
        return 0;
    }
}
pub unsafe extern "C" fn lookforfunc(interpreter: *mut Interpreter, path: *const libc::c_char, sym: *const libc::c_char) -> i32 {
    unsafe {
        let mut reg: *mut libc::c_void = checkclib(interpreter, path);
        if reg.is_null() {
            reg = lsys_load(interpreter, path, (*sym as i32 == CHARACTER_ASTERISK as i32) as i32);
            if reg.is_null() {
                return 1;
            }
            addtoclib(interpreter, path, reg);
        }
        if *sym as i32 == CHARACTER_ASTERISK as i32 {
            (*interpreter).push_boolean(true);
            return 0;
        } else {
            let cfunction: CFunction = lsys_sym(interpreter, reg, sym);
            if cfunction.is_none() {
                return 2;
            }
            lua_pushcclosure(interpreter, cfunction, 0);
            return 0;
        };
    }
}
pub unsafe extern "C" fn ll_loadlib(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let path: *const libc::c_char = lual_checklstring(interpreter, 1, null_mut());
        let init: *const libc::c_char = lual_checklstring(interpreter, 2, null_mut());
        let stat: i32 = lookforfunc(interpreter, path, init);
        if stat == 0 {
            return 1;
        } else {
            (*interpreter).push_nil();
            lua_rotate(interpreter, -2, 1);
            lua_pushstring(
                interpreter,
                if stat == 1 {
                    b"open\0" as *const u8 as *const libc::c_char
                } else {
                    b"init\0" as *const u8 as *const libc::c_char
                },
            );
            return 3;
        };
    }
}
pub unsafe extern "C" fn readable(filename: *const libc::c_char) -> i32 {
    unsafe {
        let file: *mut FILE = fopen(filename, b"r\0" as *const u8 as *const libc::c_char);
        if file.is_null() {
            return 0;
        }
        fclose(file);
        return 1;
    }
}
pub unsafe extern "C" fn getnextfilename(path: *mut *mut libc::c_char, end: *mut libc::c_char) -> *const libc::c_char {
    unsafe {
        let mut name: *mut libc::c_char = *path;
        if name == end {
            return null();
        } else if *name as i32 == Character::Null as i32 {
            *name = *(b";\0" as *const u8 as *const libc::c_char);
            name = name.offset(1);
        }
        let mut sep: *mut libc::c_char = strchr(name, *(b";\0" as *const u8 as *const libc::c_char) as i32);
        if sep.is_null() {
            sep = end;
        }
        *sep = Character::Null as libc::c_char;
        *path = sep;
        return name;
    }
}
pub unsafe extern "C" fn pusherrornotfound(interpreter: *mut Interpreter, path: *const libc::c_char) {
    unsafe {
        let mut b = Buffer::new();
        b.initialize(interpreter);
        b.add_string(b"no file '\0" as *const u8 as *const libc::c_char);
        lual_addgsub(
            &mut b,
            path,
            b";\0" as *const u8 as *const libc::c_char,
            b"'\n\tno file '\0" as *const u8 as *const libc::c_char,
        );
        b.add_string(b"'\0" as *const u8 as *const libc::c_char);
        b.push_result();
    }
}
pub unsafe extern "C" fn searchpath(
    interpreter: *mut Interpreter,
    mut name: *const libc::c_char,
    path: *const libc::c_char,
    sep: *const libc::c_char,
    dirsep: *const libc::c_char,
) -> *const libc::c_char {
    unsafe {
        let mut pathname;
        let endpathname;
        let mut filename;
        if *sep as i32 != Character::Null as i32 && !(strchr(name, *sep as i32)).is_null() {
            name = lual_gsub(interpreter, name, sep, dirsep);
        }
        let mut buffer = Buffer::new();
        buffer.initialize(interpreter);
        lual_addgsub(&mut buffer, path, b"?\0" as *const u8 as *const libc::c_char, name);
        (buffer.vector.length < buffer.vector.size || !(buffer.prepare_with_size(1)).is_null()) as i32;
        let fresh195 = buffer.vector.length;
        buffer.vector.length = (buffer.vector.length).wrapping_add(1);
        *(buffer.vector.pointer).offset(fresh195 as isize) = Character::Null as libc::c_char;
        pathname = buffer.vector.pointer;
        endpathname = pathname
            .offset(buffer.vector.length as isize)
            .offset(-(1 as isize));
        loop {
            filename = getnextfilename(&mut pathname, endpathname);
            if filename.is_null() {
                break;
            }
            if readable(filename) != 0 {
                return lua_pushstring(interpreter, filename);
            }
        }
        buffer.push_result();
        pusherrornotfound(interpreter, lua_tolstring(interpreter, -1, null_mut()));
        return null();
    }
}
pub unsafe extern "C" fn ll_searchpath(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let f: *const libc::c_char = searchpath(
            interpreter,
            lual_checklstring(interpreter, 1, null_mut()),
            lual_checklstring(interpreter, 2, null_mut()),
            lual_optlstring(
                interpreter,
                3,
                b".\0" as *const u8 as *const libc::c_char,
                null_mut(),
            ),
            lual_optlstring(
                interpreter,
                4,
                b"/\0" as *const u8 as *const libc::c_char,
                null_mut(),
            ),
        );
        if !f.is_null() {
            return 1;
        } else {
            (*interpreter).push_nil();
            lua_rotate(interpreter, -2, 1);
            return 2;
        };
    }
}
pub unsafe extern "C" fn findfile(
    interpreter: *mut Interpreter,
    name: *const libc::c_char,
    pname: *const libc::c_char,
    dirsep: *const libc::c_char,
) -> *const libc::c_char {
    unsafe {
        lua_getfield(interpreter, -(1000000 as i32) - 1000 as i32 - 1, pname);
        let path: *const libc::c_char = lua_tolstring(interpreter, -1, null_mut());
        if path.is_null() {
            lual_error(
                interpreter,
                b"'package.%s' must be a string\0".as_ptr(),
                pname,
            );
        }
        return searchpath(interpreter, name, path, b".\0" as *const u8 as *const libc::c_char, dirsep);
    }
}
pub unsafe extern "C" fn checkload(interpreter: *mut Interpreter, stat: i32, filename: *const libc::c_char) -> i32 {
    unsafe {
        if (stat != 0) as i64 != 0 {
            lua_pushstring(interpreter, filename);
            return 2;
        } else {
            return lual_error(
                interpreter,
                b"error loading module '%s' from file '%s':\n\t%s\0".as_ptr(),
                lua_tolstring(interpreter, 1, null_mut()),
                filename,
                lua_tolstring(interpreter, -1, null_mut()),
            );
        };
    }
}
pub unsafe extern "C" fn searcher_lua(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let name: *const libc::c_char = lual_checklstring(interpreter, 1, null_mut());
        let filename: *const libc::c_char = findfile(
            interpreter,
            name,
            b"path\0" as *const u8 as *const libc::c_char,
            b"/\0" as *const u8 as *const libc::c_char,
        );
        if filename.is_null() {
            return 1;
        }
        return checkload(
            interpreter,
            (lual_loadfilex(interpreter, filename, null()) == 0) as i32,
            filename,
        );
    }
}
pub unsafe extern "C" fn loadfunc(
    interpreter: *mut Interpreter,
    filename: *const libc::c_char,
    mut modname: *const libc::c_char,
) -> i32 {
    unsafe {
        modname = lual_gsub(
            interpreter,
            modname,
            b".\0" as *const u8 as *const libc::c_char,
            b"_\0" as *const u8 as *const libc::c_char,
        );
        let mut openfunc: *const libc::c_char;
        let mark: *const libc::c_char = strchr(modname, *(b"-\0" as *const u8 as *const libc::c_char) as i32);
        if !mark.is_null() {
            openfunc = lua_pushlstring(interpreter, modname, mark.offset_from(modname) as usize);
            openfunc = lua_pushfstring(interpreter, b"luaopen_%s\0" as *const u8 as *const libc::c_char, openfunc);
            let stat: i32 = lookforfunc(interpreter, filename, openfunc);
            if stat != 2 {
                return stat;
            }
            modname = mark.offset(1 as isize);
        }
        openfunc = lua_pushfstring(interpreter, b"luaopen_%s\0" as *const u8 as *const libc::c_char, modname);
        return lookforfunc(interpreter, filename, openfunc);
    }
}
pub unsafe extern "C" fn searcher_c(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let name: *const libc::c_char = lual_checklstring(interpreter, 1, null_mut());
        let filename: *const libc::c_char = findfile(
            interpreter,
            name,
            b"cpath\0" as *const u8 as *const libc::c_char,
            b"/\0" as *const u8 as *const libc::c_char,
        );
        if filename.is_null() {
            return 1;
        }
        return checkload(
            interpreter,
            (loadfunc(interpreter, filename, name) == 0) as i32,
            filename,
        );
    }
}
pub unsafe extern "C" fn searcher_croot(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let name: *const libc::c_char = lual_checklstring(interpreter, 1, null_mut());
        let p: *const libc::c_char = strchr(name, CHARACTER_PERIOD as i32);
        if p.is_null() {
            return 0;
        }
        lua_pushlstring(interpreter, name, p.offset_from(name) as usize);
        let filename: *const libc::c_char = findfile(
            interpreter,
            lua_tolstring(interpreter, -1, null_mut()),
            b"cpath\0" as *const u8 as *const libc::c_char,
            b"/\0" as *const u8 as *const libc::c_char,
        );
        if filename.is_null() {
            return 1;
        }
        let stat: i32 = loadfunc(interpreter, filename, name);
        if stat != 0 {
            if stat != 2 {
                return checkload(interpreter, 0, filename);
            } else {
                lua_pushfstring(
                    interpreter,
                    b"no module '%s' in file '%s'\0" as *const u8 as *const libc::c_char,
                    name,
                    filename,
                );
                return 1;
            }
        }
        lua_pushstring(interpreter, filename);
        return 2;
    }
}
pub unsafe extern "C" fn searcher_preload(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let name: *const libc::c_char = lual_checklstring(interpreter, 1, null_mut());
        lua_getfield(
            interpreter,
            -(1000000 as i32) - 1000 as i32,
            b"_PRELOAD\0" as *const u8 as *const libc::c_char,
        );
        if lua_getfield(interpreter, -1, name) == TagType::Nil {
            lua_pushfstring(
                interpreter,
                b"no field package.preload['%s']\0" as *const u8 as *const libc::c_char,
                name,
            );
            return 1;
        } else {
            lua_pushstring(interpreter, b":preload:\0" as *const u8 as *const libc::c_char);
            return 2;
        };
    }
}
pub unsafe extern "C" fn findloader(interpreter: *mut Interpreter, name: *const libc::c_char) {
    unsafe {
        let mut i: i32;
        let mut message = Buffer::new();
        if ((lua_getfield(
            interpreter,
            -(1000000 as i32) - 1000 as i32 - 1,
            b"searchers\0" as *const u8 as *const libc::c_char,
        ) != TagType::Table) as i32
            != 0) as i64
            != 0
        {
            lual_error(
                interpreter,
                b"'package.searchers' must be a table\0".as_ptr(),
            );
        }
        message.initialize(interpreter);
        i = 1;
        loop {
            message.add_string(b"\n\t\0" as *const u8 as *const libc::c_char);
            if lua_rawgeti(interpreter, 3, i as i64) == TagType::Nil {
                lua_settop(interpreter, -2);
                message.vector.length = message.vector.length.wrapping_sub(2);
                message.push_result();
                lual_error(
                    interpreter,
                    b"module '%s' not found:%s\0".as_ptr(),
                    name,
                    lua_tolstring(interpreter, -1, null_mut()),
                );
            }
            lua_pushstring(interpreter, name);
            lua_callk(interpreter, 1, 2, 0, None);
            if lua_type(interpreter, -2) == Some(TagType::Closure) {
                return;
            } else if lua_isstring(interpreter, -2) {
                lua_settop(interpreter, -2);
                message.add_value();
            } else {
                lua_settop(interpreter, -2 - 1);
                message.vector.length = message.vector.length.wrapping_sub(2);
            }
            i += 1;
        }
    }
}
pub unsafe extern "C" fn ll_require(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let name: *const libc::c_char = lual_checklstring(interpreter, 1, null_mut());
        lua_settop(interpreter, 1);
        lua_getfield(
            interpreter,
            -(1000000 as i32) - 1000 as i32,
            b"_LOADED\0" as *const u8 as *const libc::c_char,
        );
        lua_getfield(interpreter, 2, name);
        if lua_toboolean(interpreter, -1) != 0 {
            return 1;
        }
        lua_settop(interpreter, -2);
        findloader(interpreter, name);
        lua_rotate(interpreter, -2, 1);
        lua_pushvalue(interpreter, 1);
        lua_pushvalue(interpreter, -3);
        lua_callk(interpreter, 2, 1, 0, None);
        if !(lua_type(interpreter, -1) == Some(TagType::Nil)) {
            lua_setfield(interpreter, 2, name);
        } else {
            lua_settop(interpreter, -2);
        }
        if lua_getfield(interpreter, 2, name) == TagType::Nil {
            (*interpreter).push_boolean(true);
            lua_copy(interpreter, -1, -2);
            lua_setfield(interpreter, 2, name);
        }
        lua_rotate(interpreter, -2, 1);
        return 2;
    }
}
pub const PACKAGE_FUNCTIONS: [RegisteredFunction; 8] = {
    [
        {
            RegisteredFunction {
                name: b"loadlib\0" as *const u8 as *const libc::c_char,
                function: Some(ll_loadlib as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"searchpath\0" as *const u8 as *const libc::c_char,
                function: Some(ll_searchpath as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"preload\0" as *const u8 as *const libc::c_char,
                function: None,
            }
        },
        {
            RegisteredFunction {
                name: b"cpath\0" as *const u8 as *const libc::c_char,
                function: None,
            }
        },
        {
            RegisteredFunction {
                name: b"path\0" as *const u8 as *const libc::c_char,
                function: None,
            }
        },
        {
            RegisteredFunction {
                name: b"searchers\0" as *const u8 as *const libc::c_char,
                function: None,
            }
        },
        {
            RegisteredFunction {
                name: b"loaded\0" as *const u8 as *const libc::c_char,
                function: None,
            }
        },
        {
            RegisteredFunction {
                name: null(),
                function: None,
            }
        },
    ]
};
pub const LL_FUNCTIONS: [RegisteredFunction; 2] = {
    [
        {
            RegisteredFunction {
                name: b"require\0" as *const u8 as *const libc::c_char,
                function: Some(ll_require as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: null(),
                function: None,
            }
        },
    ]
};
pub unsafe extern "C" fn createsearcherstable(interpreter: *mut Interpreter) {
    unsafe {
        pub const SEARCHERS: [CFunction; 5] = {
            [
                Some(searcher_preload as unsafe extern "C" fn(*mut Interpreter) -> i32),
                Some(searcher_lua as unsafe extern "C" fn(*mut Interpreter) -> i32),
                Some(searcher_c as unsafe extern "C" fn(*mut Interpreter) -> i32),
                Some(searcher_croot as unsafe extern "C" fn(*mut Interpreter) -> i32),
                None,
            ]
        };
        let mut i: i32;
        (*interpreter).lua_createtable();
        i = 0;
        while (SEARCHERS[i as usize]).is_some() {
            lua_pushvalue(interpreter, -2);
            lua_pushcclosure(interpreter, SEARCHERS[i as usize], 1);
            lua_rawseti(interpreter, -2, (i + 1) as i64);
            i += 1;
        }
        lua_setfield(interpreter, -2, b"searchers\0" as *const u8 as *const libc::c_char);
    }
}
pub unsafe extern "C" fn createclibstable(interpreter: *mut Interpreter) {
    unsafe {
        lual_getsubtable(interpreter, -(1000000 as i32) - 1000 as i32, CLIBS);
        (*interpreter).lua_createtable();
        lua_pushcclosure(
            interpreter,
            Some(gctm as unsafe extern "C" fn(*mut Interpreter) -> i32),
            0,
        );
        lua_setfield(interpreter, -2, b"__gc\0" as *const u8 as *const libc::c_char);
        lua_setmetatable(interpreter, -2);
    }
}
pub unsafe extern "C" fn luaopen_package(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        createclibstable(interpreter);
        lual_checkversion_(
            interpreter,
            504.0,
            (::core::mem::size_of::<i64>() as usize)
                .wrapping_mul(16 as usize)
                .wrapping_add(::core::mem::size_of::<f64>() as usize),
        );
        (*interpreter).lua_createtable();
        lual_setfuncs(interpreter, PACKAGE_FUNCTIONS.as_ptr(), 0);
        createsearcherstable(interpreter);
        setpath(
        interpreter,
        b"path\0" as *const u8 as *const libc::c_char,
        b"LUA_PATH\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/lua/5.4/?.lua;/usr/local/share/lua/5.4/?/init.lua;/usr/local/lib/lua/5.4/?.lua;/usr/local/lib/lua/5.4/?/init.lua;./?.lua;./?/init.lua\0"
            as *const u8 as *const libc::c_char,
    );
        setpath(
            interpreter,
            b"cpath\0" as *const u8 as *const libc::c_char,
            b"LUA_CPATH\0" as *const u8 as *const libc::c_char,
            b"/usr/local/lib/lua/5.4/?.so;/usr/local/lib/lua/5.4/loadall.so;./?.so\0" as *const u8
                as *const libc::c_char,
        );
        lua_pushstring(interpreter, b"/\n;\n?\n!\n-\n\0" as *const u8 as *const libc::c_char);
        lua_setfield(interpreter, -2, b"config\0" as *const u8 as *const libc::c_char);
        lual_getsubtable(
            interpreter,
            -(1000000 as i32) - 1000 as i32,
            b"_LOADED\0" as *const u8 as *const libc::c_char,
        );
        lua_setfield(interpreter, -2, b"loaded\0" as *const u8 as *const libc::c_char);
        lual_getsubtable(
            interpreter,
            -(1000000 as i32) - 1000 as i32,
            b"_PRELOAD\0" as *const u8 as *const libc::c_char,
        );
        lua_setfield(interpreter, -2, b"preload\0" as *const u8 as *const libc::c_char);
        lua_rawgeti(interpreter, -(1000000 as i32) - 1000 as i32, 2 as i64);
        lua_pushvalue(interpreter, -2);
        lual_setfuncs(interpreter, LL_FUNCTIONS.as_ptr(), 1);
        lua_settop(interpreter, -2);
        return 1;
    }
}
