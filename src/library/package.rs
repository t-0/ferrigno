use rlua::*;
use crate::buffer::*;
use crate::character::*;
use crate::functions::*;
use crate::interpreter::*;
use crate::new::*;
use crate::registeredfunction::*;
use crate::tag::*;
use crate::utility::c::*;
use libc::{dlclose, dlerror, dlopen, dlsym};
use std::ptr::*;
pub const CLIBS: *const i8 = make_cstring!("_CLIBS");
pub unsafe fn lsys_unloadlib(lib: *mut libc::c_void) {
    unsafe {
        dlclose(lib);
    }
}
pub unsafe fn lsys_load(
    interpreter: *mut Interpreter,
    path: *const i8,
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
pub unsafe fn lsys_sym(
    interpreter: *mut Interpreter,
    lib: *mut libc::c_void,
    sym: *const i8,
) -> CFunction {
    unsafe {
        let cfunction: CFunction =
            ::core::mem::transmute::<*mut libc::c_void, CFunction>(dlsym(lib, sym));
        if cfunction.is_none() {
            lua_pushstring(interpreter, dlerror());
        }
        return cfunction;
    }
}
pub unsafe fn noenv(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let b: i32;
        lua_getfield(
            interpreter,
            -(1000000 as i32) - 1000 as i32,
            make_cstring!("LUA_NOENV"),
        );
        b = lua_toboolean(interpreter, -1);
        lua_settop(interpreter, -2);
        return b;
    }
}
pub unsafe fn setpath(
    interpreter: *mut Interpreter,
    fieldname: *const i8,
    envname: *const i8,
    dft: *const i8,
) {
    unsafe {
        let dftmark: *const i8;
        let nver: *const i8 = lua_pushfstring(
            interpreter,
            make_cstring!("%s%s"),
            envname,
            make_cstring!("_5_4"),
        );
        let mut path: *const i8 = getenv(nver);
        if path.is_null() {
            path = getenv(envname);
        }
        if path.is_null() || noenv(interpreter) != 0 {
            lua_pushstring(interpreter, dft);
        } else {
            dftmark = strstr(path, make_cstring!(";;"));
            if dftmark.is_null() {
                lua_pushstring(interpreter, path);
            } else {
                let length: usize = strlen(path) as usize;
                let mut b = Buffer::new();
                b.initialize(interpreter);
                if path < dftmark {
                    b.add_string_with_length(path, dftmark.offset_from(path) as usize);
                    (b.loads.get_length() < b.loads.get_size()
                        || !(b.prepare_with_size(1)).is_null()) as i32;
                    let fresh193 = b.loads.get_length();
                    b.loads
                        .set_length(((b.loads.get_length()).wrapping_add(1)) as usize);
                    *(b.loads.loads_pointer).offset(fresh193 as isize) =
                        *(make_cstring!(";"));
                }
                b.add_string(dft);
                if dftmark < path.offset(length as isize).offset(-(2 as isize)) {
                    (b.loads.get_length() < b.loads.get_size()
                        || !(b.prepare_with_size(1)).is_null()) as i32;
                    let fresh194 = b.loads.get_length();
                    b.loads
                        .set_length(((b.loads.get_length()).wrapping_add(1)) as usize);
                    *(b.loads.loads_pointer).offset(fresh194 as isize) =
                        *(make_cstring!(";"));
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
pub unsafe fn checkclib(
    interpreter: *mut Interpreter,
    path: *const i8,
) -> *mut libc::c_void {
    unsafe {
        let plib: *mut libc::c_void;
        lua_getfield(interpreter, -(1000000 as i32) - 1000 as i32, CLIBS);
        lua_getfield(interpreter, -1, path);
        plib = (*interpreter).to_pointer(-1);
        lua_settop(interpreter, -2 - 1);
        return plib;
    }
}
pub unsafe fn addtoclib(
    interpreter: *mut Interpreter,
    path: *const i8,
    plib: *mut libc::c_void,
) {
    unsafe {
        lua_getfield(interpreter, -(1000000 as i32) - 1000 as i32, CLIBS);
        lua_pushlightuserdata(interpreter, plib);
        lua_pushvalue(interpreter, -1);
        lua_setfield(interpreter, -3, path);
        lua_rawseti(interpreter, -2, lual_len(interpreter, -2) + 1);
        lua_settop(interpreter, -2);
    }
}
pub unsafe fn gctm(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let mut n: i64 = lual_len(interpreter, 1);
        while n >= 1 {
            lua_rawgeti(interpreter, 1, n);
            lsys_unloadlib((*interpreter).to_pointer(-1));
            lua_settop(interpreter, -2);
            n -= 1;
        }
        return 0;
    }
}
pub unsafe fn lookforfunc(
    interpreter: *mut Interpreter,
    path: *const i8,
    sym: *const i8,
) -> i32 {
    unsafe {
        let mut reg: *mut libc::c_void = checkclib(interpreter, path);
        if reg.is_null() {
            reg = lsys_load(
                interpreter,
                path,
                (*sym as i32 == CHARACTER_ASTERISK as i32) as i32,
            );
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
pub unsafe fn ll_loadlib(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let path: *const i8 = lual_checklstring(interpreter, 1, null_mut());
        let init: *const i8 = lual_checklstring(interpreter, 2, null_mut());
        let stat: i32 = lookforfunc(interpreter, path, init);
        if stat == 0 {
            return 1;
        } else {
            (*interpreter).push_nil();
            lua_rotate(interpreter, -2, 1);
            lua_pushstring(
                interpreter,
                if stat == 1 {
                    make_cstring!("open")
                } else {
                    make_cstring!("init")
                },
            );
            return 3;
        };
    }
}
pub unsafe fn readable(filename: *const i8) -> i32 {
    unsafe {
        let file: *mut FILE = fopen(filename, make_cstring!("r"));
        if file.is_null() {
            return 0;
        }
        fclose(file);
        return 1;
    }
}
pub unsafe fn getnextfilename(path: *mut *mut i8, end: *mut i8) -> *const i8 {
    unsafe {
        let mut name: *mut i8 = *path;
        if name == end {
            return null();
        } else if *name as i32 == Character::Null as i32 {
            *name = *(make_cstring!(";"));
            name = name.offset(1);
        }
        let mut sep: *mut i8 = strchr(name, *(make_cstring!(";")) as i32);
        if sep.is_null() {
            sep = end;
        }
        *sep = Character::Null as i8;
        *path = sep;
        return name;
    }
}
pub unsafe fn pusherrornotfound(interpreter: *mut Interpreter, path: *const i8) {
    unsafe {
        let mut b = Buffer::new();
        b.initialize(interpreter);
        b.add_string(make_cstring!("no file '"));
        lual_addgsub(
            &mut b,
            path,
            make_cstring!(";"),
            make_cstring!("'\n\tno file '"),
        );
        b.add_string(make_cstring!("'"));
        b.push_result();
    }
}
pub unsafe fn searchpath(
    interpreter: *mut Interpreter,
    mut name: *const i8,
    path: *const i8,
    sep: *const i8,
    dirsep: *const i8,
) -> *const i8 {
    unsafe {
        let mut pathname;
        let endpathname;
        let mut filename;
        if *sep as i32 != Character::Null as i32 && !(strchr(name, *sep as i32)).is_null() {
            name = lual_gsub(interpreter, name, sep, dirsep);
        }
        let mut buffer = Buffer::new();
        buffer.initialize(interpreter);
        lual_addgsub(&mut buffer, path, make_cstring!("?"), name);
        (buffer.loads.get_length() < buffer.loads.get_size()
            || !(buffer.prepare_with_size(1)).is_null()) as i32;
        let fresh195 = buffer.loads.get_length();
        buffer
            .loads
            .set_length(((buffer.loads.get_length()).wrapping_add(1)) as usize);
        *(buffer.loads.loads_pointer).offset(fresh195 as isize) = Character::Null as i8;
        pathname = buffer.loads.loads_pointer;
        endpathname = pathname
            .offset(buffer.loads.get_length() as isize)
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
pub unsafe fn ll_searchpath(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let f: *const i8 = searchpath(
            interpreter,
            lual_checklstring(interpreter, 1, null_mut()),
            lual_checklstring(interpreter, 2, null_mut()),
            lual_optlstring(interpreter, 3, make_cstring!("."), null_mut()),
            lual_optlstring(interpreter, 4, make_cstring!("/"), null_mut()),
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
pub unsafe fn findfile(
    interpreter: *mut Interpreter,
    name: *const i8,
    pname: *const i8,
    dirsep: *const i8,
) -> *const i8 {
    unsafe {
        lua_getfield(interpreter, -(1000000 as i32) - 1000 as i32 - 1, pname);
        let path: *const i8 = lua_tolstring(interpreter, -1, null_mut());
        if path.is_null() {
            lual_error(
                interpreter,
                make_cstring!("'package.%s' must be a string"),
                pname,
            );
        }
        return searchpath(
            interpreter,
            name,
            path,
            make_cstring!("."),
            dirsep,
        );
    }
}
pub unsafe fn checkload(
    interpreter: *mut Interpreter,
    stat: i32,
    filename: *const i8,
) -> i32 {
    unsafe {
        if (stat != 0) as i64 != 0 {
            lua_pushstring(interpreter, filename);
            return 2;
        } else {
            return lual_error(
                interpreter,
                make_cstring!("error loading module '%s' from file '%s':\n\t%s"),
                lua_tolstring(interpreter, 1, null_mut()),
                filename,
                lua_tolstring(interpreter, -1, null_mut()),
            );
        };
    }
}
pub unsafe fn searcher_lua(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let name: *const i8 = lual_checklstring(interpreter, 1, null_mut());
        let filename: *const i8 = findfile(
            interpreter,
            name,
            make_cstring!("path"),
            make_cstring!("/"),
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
pub unsafe fn loadfunc(
    interpreter: *mut Interpreter,
    filename: *const i8,
    mut modname: *const i8,
) -> i32 {
    unsafe {
        modname = lual_gsub(
            interpreter,
            modname,
            make_cstring!("."),
            make_cstring!("_"),
        );
        let mut openfunc: *const i8;
        let mark: *const i8 = strchr(modname, *(make_cstring!("-")) as i32);
        if !mark.is_null() {
            openfunc = lua_pushlstring(interpreter, modname, mark.offset_from(modname) as usize);
            openfunc = lua_pushfstring(
                interpreter,
                make_cstring!("luaopen_%s"),
                openfunc,
            );
            let stat: i32 = lookforfunc(interpreter, filename, openfunc);
            if stat != 2 {
                return stat;
            }
            modname = mark.offset(1 as isize);
        }
        openfunc = lua_pushfstring(
            interpreter,
            make_cstring!("luaopen_%s"),
            modname,
        );
        return lookforfunc(interpreter, filename, openfunc);
    }
}
pub unsafe fn searcher_c(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let name: *const i8 = lual_checklstring(interpreter, 1, null_mut());
        let filename: *const i8 = findfile(
            interpreter,
            name,
            make_cstring!("cpath"),
            make_cstring!("/"),
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
pub unsafe fn searcher_croot(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let name: *const i8 = lual_checklstring(interpreter, 1, null_mut());
        let p: *const i8 = strchr(name, CHARACTER_PERIOD as i32);
        if p.is_null() {
            return 0;
        }
        lua_pushlstring(interpreter, name, p.offset_from(name) as usize);
        let filename: *const i8 = findfile(
            interpreter,
            lua_tolstring(interpreter, -1, null_mut()),
            make_cstring!("cpath"),
            make_cstring!("/"),
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
                    make_cstring!("no module '%s' in file '%s'"),
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
pub unsafe fn searcher_preload(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let name: *const i8 = lual_checklstring(interpreter, 1, null_mut());
        lua_getfield(
            interpreter,
            -(1000000 as i32) - 1000 as i32,
            make_cstring!("_PRELOAD"),
        );
        if lua_getfield(interpreter, -1, name) == TagType::Nil {
            lua_pushfstring(
                interpreter,
                make_cstring!("no field package.preload['%s']"),
                name,
            );
            return 1;
        } else {
            lua_pushstring(interpreter, make_cstring!(":preload:"));
            return 2;
        };
    }
}
pub unsafe fn findloader(interpreter: *mut Interpreter, name: *const i8) {
    unsafe {
        let mut i: i32;
        let mut message = Buffer::new();
        if ((lua_getfield(
            interpreter,
            -(1000000 as i32) - 1000 as i32 - 1,
            make_cstring!("searchers"),
        ) != TagType::Table) as i32
            != 0) as i64
            != 0
        {
            lual_error(
                interpreter,
                make_cstring!("'package.searchers' must be a table"),
            );
        }
        message.initialize(interpreter);
        i = 1;
        loop {
            message.add_string(make_cstring!("\n\t"));
            if lua_rawgeti(interpreter, 3, i as i64) == TagType::Nil {
                lua_settop(interpreter, -2);
                message
                    .loads
                    .set_length((message.loads.get_length().wrapping_sub(2)) as usize);
                message.push_result();
                lual_error(
                    interpreter,
                    make_cstring!("module '%s' not found:%s"),
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
                message
                    .loads
                    .set_length((message.loads.get_length().wrapping_sub(2)) as usize);
            }
            i += 1;
        }
    }
}
pub unsafe fn ll_require(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let name: *const i8 = lual_checklstring(interpreter, 1, null_mut());
        lua_settop(interpreter, 1);
        lua_getfield(
            interpreter,
            -(1000000 as i32) - 1000 as i32,
            make_cstring!("_LOADED"),
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
                name: make_cstring!("loadlib"),
                function: Some(ll_loadlib as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: make_cstring!("searchpath"),
                function: Some(ll_searchpath as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: make_cstring!("preload"),
                function: None,
            }
        },
        {
            RegisteredFunction {
                name: make_cstring!("cpath"),
                function: None,
            }
        },
        {
            RegisteredFunction {
                name: make_cstring!("path"),
                function: None,
            }
        },
        {
            RegisteredFunction {
                name: make_cstring!("searchers"),
                function: None,
            }
        },
        {
            RegisteredFunction {
                name: make_cstring!("loaded"),
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
                name: make_cstring!("require"),
                function: Some(ll_require as unsafe fn(*mut Interpreter) -> i32),
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
pub unsafe fn createsearcherstable(interpreter: *mut Interpreter) {
    unsafe {
        pub const SEARCHERS: [CFunction; 5] = {
            [
                Some(searcher_preload as unsafe fn(*mut Interpreter) -> i32),
                Some(searcher_lua as unsafe fn(*mut Interpreter) -> i32),
                Some(searcher_c as unsafe fn(*mut Interpreter) -> i32),
                Some(searcher_croot as unsafe fn(*mut Interpreter) -> i32),
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
        lua_setfield(interpreter, -2, make_cstring!("searchers"));
    }
}
pub unsafe fn createclibstable(interpreter: *mut Interpreter) {
    unsafe {
        lual_getsubtable(interpreter, -(1000000 as i32) - 1000 as i32, CLIBS);
        (*interpreter).lua_createtable();
        lua_pushcclosure(
            interpreter,
            Some(gctm as unsafe fn(*mut Interpreter) -> i32),
            0,
        );
        lua_setfield(interpreter, -2, make_cstring!("__gc"));
        lua_setmetatable(interpreter, -2);
    }
}
pub unsafe fn luaopen_package(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        createclibstable(interpreter);
        lual_checkversion_(
            interpreter,
            504.0,
            (size_of::<i64>() as usize)
                .wrapping_mul(16 as usize)
                .wrapping_add(size_of::<f64>() as usize),
        );
        (*interpreter).lua_createtable();
        lual_setfuncs(interpreter, PACKAGE_FUNCTIONS.as_ptr(), 0);
        createsearcherstable(interpreter);
        setpath(
        interpreter,
        make_cstring!("path"),
        make_cstring!("LUA_PATH"),
        make_cstring!("/usr/local/share/lua/5.4/?.lua;/usr/local/share/lua/5.4/?/init.lua;/usr/local/lib/lua/5.4/?.lua;/usr/local/lib/lua/5.4/?/init.lua;./?.lua;./?/init.lua")
            as *const u8 as *const i8,
    );
        setpath(
            interpreter,
            make_cstring!("cpath"),
            make_cstring!("LUA_CPATH"),
            make_cstring!("/usr/local/lib/lua/5.4/?.so;/usr/local/lib/lua/5.4/loadall.so;./?.so\0"),
        );
        lua_pushstring(interpreter, make_cstring!("/\n;\n?\n!\n-\n"));
        lua_setfield(interpreter, -2, make_cstring!("config"));
        lual_getsubtable(
            interpreter,
            -1000000 - 1000,
            make_cstring!("_LOADED"),
        );
        lua_setfield(interpreter, -2, make_cstring!("loaded"));
        lual_getsubtable(
            interpreter,
            -1000000 - 1000,
            make_cstring!("_PRELOAD"),
        );
        lua_setfield(interpreter, -2, make_cstring!("preload"));
        lua_rawgeti(interpreter, -1000000 - 1000, 2 as i64);
        lua_pushvalue(interpreter, -2);
        lual_setfuncs(interpreter, LL_FUNCTIONS.as_ptr(), 1);
        lua_settop(interpreter, -2);
        return 1;
    }
}
