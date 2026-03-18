use crate::buffer::*;
use crate::character::*;
use crate::functions::*;
use crate::registeredfunction::*;
use crate::state::*;
use crate::status::*;
use crate::tagtype::*;
use crate::tdefaultnew::*;
use crate::utility::*;
unsafe extern "C" {
    fn dlclose(handle: *mut std::ffi::c_void) -> i32;
    fn dlerror() -> *mut i8;
    fn dlopen(filename: *const i8, flags: i32) -> *mut std::ffi::c_void;
    fn dlsym(handle: *mut std::ffi::c_void, symbol: *const i8) -> *mut std::ffi::c_void;
}
use std::ptr::*;
pub const CLIBS: *const i8 = c"_CLIBS".as_ptr();
pub unsafe fn lsys_unloadlib(lib: *mut std::ffi::c_void) {
    unsafe {
        dlclose(lib);
    }
}
pub unsafe fn lsys_load(state: *mut State, path: *const i8, seeglb: i32) -> *mut std::ffi::c_void {
    unsafe {
        let lib: *mut std::ffi::c_void = dlopen(path, 0x2_i32 | (if seeglb != 0 { 0x100_i32 } else { 0 }));
        if lib.is_null() {
            lua_pushstring(state, dlerror());
        }
        lib
    }
}
pub unsafe fn lsys_sym(state: *mut State, lib: *mut std::ffi::c_void, sym: *const i8) -> CFunction {
    unsafe {
        let cfunction: CFunction = ::core::mem::transmute::<*mut std::ffi::c_void, CFunction>(dlsym(lib, sym));
        if cfunction.is_none() {
            lua_pushstring(state, dlerror());
        }
        cfunction
    }
}
pub unsafe fn noenv(state: *mut State) -> bool {
    unsafe {
        lua_getfield(state, LUA_REGISTRYINDEX, c"LUA_NOENV".as_ptr());
        let b = lua_toboolean(state, -1);
        lua_settop(state, -2);
        b
    }
}
pub unsafe fn setpath(state: *mut State, fieldname: *const i8, envname: *const i8, dft: *const i8) {
    unsafe {
        let dftmark: *const i8;
        let nver: *const i8 = lua_pushfstring(state, c"%s%s".as_ptr(), &[envname.into(), c"_5_5".as_ptr().into()]);
        let mut path: *const i8 = os_getenv(nver);
        if path.is_null() {
            path = os_getenv(envname);
        }
        if path.is_null() || noenv(state) {
            lua_pushstring(state, dft);
        } else {
            dftmark = cstr_str(path, c";;".as_ptr());
            if dftmark.is_null() {
                lua_pushstring(state, path);
            } else {
                let length: usize = cstr_len(path);
                let mut b = Buffer::new();
                b.initialize(state);
                if path < dftmark {
                    b.add_string_with_length(path, dftmark.offset_from(path) as usize);
                    (b.buffer_loads.get_length() < b.buffer_loads.get_size() || !(b.prepare_with_size(1)).is_null()) as i32;
                    let write_offset = b.buffer_loads.get_length();
                    b.buffer_loads.set_length((b.buffer_loads.get_length() + 1) as usize);
                    *(b.buffer_loads.loads_pointer).add(write_offset as usize) = *(c";".as_ptr());
                }
                b.add_string(dft);
                if dftmark < path.add(length).sub(2) {
                    (b.buffer_loads.get_length() < b.buffer_loads.get_size() || !(b.prepare_with_size(1)).is_null()) as i32;
                    let write_offset = b.buffer_loads.get_length();
                    b.buffer_loads.set_length((b.buffer_loads.get_length() + 1) as usize);
                    *(b.buffer_loads.loads_pointer).add(write_offset as usize) = *(c";".as_ptr());
                    b.add_string_with_length(dftmark.add(2), path.add(length).sub(2).offset_from(dftmark) as usize);
                }
                b.push_result();
            }
        }
        lua_setfield(state, -3, fieldname);
        lua_settop(state, -2);
    }
}
pub unsafe fn checkclib(state: *mut State, path: *const i8) -> *mut std::ffi::c_void {
    unsafe {
        lua_getfield(state, LUA_REGISTRYINDEX, CLIBS);
        lua_getfield(state, -1, path);
        let plib: *mut std::ffi::c_void = (*state).to_pointer(-1);
        lua_settop(state, -2 - 1);
        plib
    }
}
pub unsafe fn addtoclib(state: *mut State, path: *const i8, plib: *mut std::ffi::c_void) {
    unsafe {
        lua_getfield(state, LUA_REGISTRYINDEX, CLIBS);
        lua_pushlightuserdata(state, plib);
        lua_pushvalue(state, -1);
        lua_setfield(state, -3, path);
        lua_rawseti(state, -2, lual_len(state, -2) + 1);
        lua_settop(state, -2);
    }
}
pub unsafe fn gctm(state: *mut State) -> i32 {
    unsafe {
        let mut n: i64 = lual_len(state, 1);
        while n >= 1 {
            lua_rawgeti(state, 1, n);
            lsys_unloadlib((*state).to_pointer(-1));
            lua_settop(state, -2);
            n -= 1;
        }
        0
    }
}
pub unsafe fn lookforfunc(state: *mut State, path: *const i8, sym: *const i8) -> i32 {
    unsafe {
        let mut reg: *mut std::ffi::c_void = checkclib(state, path);
        if reg.is_null() {
            reg = lsys_load(state, path, (*sym as i32 == Character::Asterisk as i32) as i32);
            if reg.is_null() {
                return 1;
            }
            addtoclib(state, path, reg);
        }
        if *sym as i32 == Character::Asterisk as i32 {
            (*state).push_boolean(true);
            0
        } else {
            let cfunction: CFunction = lsys_sym(state, reg, sym);
            if cfunction.is_none() {
                return 2;
            }
            lua_pushcclosure(state, cfunction, 0);
            0
        }
    }
}
pub unsafe fn ll_loadlib(state: *mut State) -> i32 {
    unsafe {
        let path: *const i8 = lual_checklstring(state, 1, null_mut());
        let initial: *const i8 = lual_checklstring(state, 2, null_mut());
        let stat: i32 = lookforfunc(state, path, initial);
        if stat == 0 {
            1
        } else {
            (*state).push_nil();
            lua_rotate(state, -2, 1);
            lua_pushstring(state, if stat == 1 { c"absent".as_ptr() } else { c"init".as_ptr() });
            3
        }
    }
}
pub unsafe fn readable(filename: *const i8) -> i32 {
    unsafe {
        let cstr = std::ffi::CStr::from_ptr(filename);
        match cstr.to_str() {
            | Ok(path) => std::fs::File::open(path).is_ok() as i32,
            | Err(_) => 0,
        }
    }
}
pub unsafe fn getnextfilename(path: *mut *mut i8, end: *mut i8) -> *const i8 {
    unsafe {
        let mut name: *mut i8 = *path;
        if name == end {
            return null();
        } else if *name as i32 == Character::Null as i32 {
            *name = *(c";".as_ptr());
            name = name.add(1);
        }
        let mut sep: *mut i8 = cstr_chr(name, *(c";".as_ptr())) as *mut i8;
        if sep.is_null() {
            sep = end;
        }
        *sep = Character::Null as i8;
        *path = sep;
        name
    }
}
pub unsafe fn pusherrornotfound(state: *mut State, path: *const i8) {
    unsafe {
        let mut buffer = Buffer::new();
        buffer.initialize(state);
        buffer.add_string(c"no file '".as_ptr());
        lual_addgsub(&mut buffer, path, c";".as_ptr(), c"'\n\tno file '".as_ptr());
        buffer.add_string(c"'".as_ptr());
        buffer.push_result();
    }
}
pub unsafe fn searchpath(state: *mut State, mut name: *const i8, path: *const i8, sep: *const i8, dirsep: *const i8) -> *const i8 {
    unsafe {
        let mut pathname;

        let mut filename;
        if *sep as i32 != Character::Null as i32 && !(cstr_chr(name, *sep)).is_null() {
            name = lual_gsub(state, name, sep, dirsep);
        }
        let mut buffer = Buffer::new();
        buffer.initialize(state);
        lual_addgsub(&mut buffer, path, c"?".as_ptr(), name);
        (buffer.buffer_loads.get_length() < buffer.buffer_loads.get_size() || !(buffer.prepare_with_size(1)).is_null()) as i32;
        let write_offset = buffer.buffer_loads.get_length();
        buffer.buffer_loads.set_length((buffer.buffer_loads.get_length() + 1) as usize);
        *(buffer.buffer_loads.loads_pointer).add(write_offset as usize) = Character::Null as i8;
        pathname = buffer.buffer_loads.loads_pointer;
        let endpathname = pathname.add(buffer.buffer_loads.get_length() as usize).sub(1);
        loop {
            filename = getnextfilename(&mut pathname, endpathname);
            if filename.is_null() {
                break;
            }
            if readable(filename) != 0 {
                return lua_pushstring(state, filename);
            }
        }
        buffer.push_result();
        pusherrornotfound(state, lua_tolstring(state, -1, null_mut()));
        null()
    }
}
pub unsafe fn ll_searchpath(state: *mut State) -> i32 {
    unsafe {
        let f: *const i8 = searchpath(
            state,
            lual_checklstring(state, 1, null_mut()),
            lual_checklstring(state, 2, null_mut()),
            lual_optlstring(state, 3, c".".as_ptr(), null_mut()),
            lual_optlstring(state, 4, c"/".as_ptr(), null_mut()),
        );
        if !f.is_null() {
            1
        } else {
            (*state).push_nil();
            lua_rotate(state, -2, 1);
            2
        }
    }
}
pub unsafe fn findfile(state: *mut State, name: *const i8, pname: *const i8, dirsep: *const i8) -> *const i8 {
    unsafe {
        lua_getfield(state, LUA_REGISTRYINDEX - 1, pname);
        let path: *const i8 = lua_tolstring(state, -1, null_mut());
        if path.is_null() {
            lual_error(state, c"'package.%s' must be a string".as_ptr(), &[pname.into()]);
        }
        searchpath(state, name, path, c".".as_ptr(), dirsep)
    }
}
pub unsafe fn checkload(state: *mut State, stat: i32, filename: *const i8) -> i32 {
    unsafe {
        if stat != 0 {
            lua_pushstring(state, filename);
            2
        } else {
            lual_error(
                state,
                c"error loading module '%s' from file '%s':\n\t%s".as_ptr(),
                &[lua_tolstring(state, 1, null_mut()).into(),
                filename.into(),
                lua_tolstring(state, -1, null_mut()).into()],
            )
        }
    }
}
pub unsafe fn searcher_embedded(state: *mut State) -> i32 {
    unsafe {
        let name: *const i8 = lual_checklstring(state, 1, null_mut());
        let name_str = match std::ffi::CStr::from_ptr(name).to_str() {
            | Ok(s) => s,
            | Err(_) => return 1,
        };
        // Try module_name.lua in embedded resources (relative to embedded base dir)
        let filename = format!("{}.lua", name_str.replace('.', "/"));
        if let Some(raw) = crate::state::resolve_embedded(&filename) {
            let content = crate::state::skip_shebang(raw);
            let c_filename = std::ffi::CString::new(format!("@embedded:{}", filename)).unwrap();
            let status = lual_loadbufferx(state, content.as_ptr() as *const i8, content.len(), c_filename.as_ptr(), null());
            if status == Status::OK {
                lua_pushstring(state, c_filename.as_ptr());
                return 2;
            } else {
                return lual_error(
                    state,
                    c"error loading module '%s' from embedded resource '%s':\n\t%s".as_ptr(),
                    &[name.into(),
                    c_filename.as_ptr().into(),
                    lua_tolstring(state, -1, null_mut()).into()],
                );
            }
        }
        // Not found in embedded resources — return 0 to silently skip
        // (returning 1 without pushing a value would leak the argument as
        // a spurious error message line)
        0
    }
}
pub unsafe fn searcher_lua(state: *mut State) -> i32 {
    unsafe {
        let name: *const i8 = lual_checklstring(state, 1, null_mut());
        let filename: *const i8 = findfile(state, name, c"path".as_ptr(), c"/".as_ptr());
        if filename.is_null() {
            return 1;
        }
        checkload(state, (lual_loadfilex(state, filename, null()) == Status::OK) as i32, filename)
    }
}
pub unsafe fn loadfunc(state: *mut State, filename: *const i8, mut modname: *const i8) -> i32 {
    unsafe {
        modname = lual_gsub(state, modname, c".".as_ptr(), c"_".as_ptr());
        let mut openfunc: *const i8;
        let mark: *const i8 = cstr_chr(modname, *(c"-".as_ptr()));
        if !mark.is_null() {
            openfunc = lua_pushlstring(state, modname, mark.offset_from(modname) as usize);
            openfunc = lua_pushfstring(state, c"luaopen_%s".as_ptr(), &[openfunc.into()]);
            let stat: i32 = lookforfunc(state, filename, openfunc);
            if stat != 2 {
                return stat;
            }
            modname = mark.add(1);
        }
        openfunc = lua_pushfstring(state, c"luaopen_%s".as_ptr(), &[modname.into()]);
        lookforfunc(state, filename, openfunc)
    }
}
pub unsafe fn searcher_c(state: *mut State) -> i32 {
    unsafe {
        let name: *const i8 = lual_checklstring(state, 1, null_mut());
        let filename: *const i8 = findfile(state, name, c"cpath".as_ptr(), c"/".as_ptr());
        if filename.is_null() {
            return 1;
        }
        checkload(state, (loadfunc(state, filename, name) == 0) as i32, filename)
    }
}
pub unsafe fn searcher_croot(state: *mut State) -> i32 {
    unsafe {
        let name: *const i8 = lual_checklstring(state, 1, null_mut());
        let p: *const i8 = cstr_chr(name, Character::Period as i8);
        if p.is_null() {
            return 0;
        }
        lua_pushlstring(state, name, p.offset_from(name) as usize);
        let filename: *const i8 = findfile(state, lua_tolstring(state, -1, null_mut()), c"cpath".as_ptr(), c"/".as_ptr());
        if filename.is_null() {
            return 1;
        }
        let stat: i32 = loadfunc(state, filename, name);
        if stat != 0 {
            if stat != 2 {
                return checkload(state, 0, filename);
            } else {
                lua_pushfstring(state, c"no module '%s' in file '%s'".as_ptr(), &[name.into(), filename.into()]);
                return 1;
            }
        }
        lua_pushstring(state, filename);
        2
    }
}
pub unsafe fn searcher_preload(state: *mut State) -> i32 {
    unsafe {
        let name: *const i8 = lual_checklstring(state, 1, null_mut());
        lua_getfield(state, LUA_REGISTRYINDEX, c"_PRELOAD".as_ptr());
        if lua_getfield(state, -1, name) == TagType::Nil {
            lua_pushfstring(state, c"no field package.preload['%s']".as_ptr(), &[name.into()]);
            1
        } else {
            lua_pushstring(state, c":preload:".as_ptr());
            2
        }
    }
}
pub unsafe fn findloader(state: *mut State, name: *const i8) {
    unsafe {
        let mut i: i32;
        let mut message = Buffer::new();
        if lua_getfield(state, LUA_REGISTRYINDEX - 1, c"searchers".as_ptr()) != TagType::Table {
            lual_error(state, c"'package.searchers' must be a table".as_ptr(), &[]);
        }
        message.initialize(state);
        i = 1;
        loop {
            message.add_string(c"\n\t".as_ptr());
            if lua_rawgeti(state, 3, i as i64) == TagType::Nil {
                lua_settop(state, -2);
                message
                    .buffer_loads
                    .set_length((message.buffer_loads.get_length() - 2) as usize);
                message.push_result();
                lual_error(
                    state,
                    c"module '%s' not found:%s".as_ptr(),
                    &[name.into(),
                    lua_tolstring(state, -1, null_mut()).into()],
                );
            }
            lua_pushstring(state, name);
            (*state).lua_callk(1, 2, 0, None);
            if lua_type(state, -2) == Some(TagType::Closure) {
                return;
            } else if lua_isstring(state, -2) {
                lua_settop(state, -2);
                message.add_value();
            } else {
                lua_settop(state, -2 - 1);
                message
                    .buffer_loads
                    .set_length((message.buffer_loads.get_length() - 2) as usize);
            }
            i += 1;
        }
    }
}
pub unsafe fn ll_require(state: *mut State) -> i32 {
    unsafe {
        let name: *const i8 = lual_checklstring(state, 1, null_mut());
        lua_settop(state, 1);
        lua_getfield(state, LUA_REGISTRYINDEX, c"_LOADED".as_ptr());
        lua_getfield(state, 2, name);
        if lua_toboolean(state, -1) {
            return 1;
        }
        lua_settop(state, -2);
        findloader(state, name);
        lua_rotate(state, -2, 1);
        lua_pushvalue(state, 1);
        lua_pushvalue(state, -3);
        (*state).lua_callk(2, 1, 0, None);
        if !(lua_type(state, -1) == Some(TagType::Nil)) {
            lua_setfield(state, 2, name);
        } else {
            lua_settop(state, -2);
        }
        if lua_getfield(state, 2, name) == TagType::Nil {
            (*state).push_boolean(true);
            lua_copy(state, -1, -2);
            lua_setfield(state, 2, name);
        }
        lua_rotate(state, -2, 1);
        2
    }
}
pub const PACKAGE_FUNCTIONS: [RegisteredFunction; 2] = {
    [
        {
            RegisteredFunction {
                registeredfunction_name: c"loadlib".as_ptr(),
                registeredfunction_function: Some(ll_loadlib as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"searchpath".as_ptr(),
                registeredfunction_function: Some(ll_searchpath as unsafe fn(*mut State) -> i32),
            }
        },
    ]
};
pub const LL_FUNCTIONS: [RegisteredFunction; 1] = [{
    RegisteredFunction {
        registeredfunction_name: c"require".as_ptr(),
        registeredfunction_function: Some(ll_require as unsafe fn(*mut State) -> i32),
    }
}];
pub unsafe fn createsearcherstable(state: *mut State) {
    unsafe {
        pub const SEARCHERS: [CFunction; 6] = {
            [
                Some(searcher_preload as unsafe fn(*mut State) -> i32),
                Some(searcher_embedded as unsafe fn(*mut State) -> i32),
                Some(searcher_lua as unsafe fn(*mut State) -> i32),
                Some(searcher_c as unsafe fn(*mut State) -> i32),
                Some(searcher_croot as unsafe fn(*mut State) -> i32),
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
        lua_setfield(state, -2, c"searchers".as_ptr());
    }
}
pub unsafe fn createclibstable(state: *mut State) {
    unsafe {
        lual_getsubtable(state, LUA_REGISTRYINDEX, CLIBS);
        (*state).lua_createtable();
        lua_pushcclosure(state, Some(gctm as unsafe fn(*mut State) -> i32), 0);
        lua_setfield(state, -2, c"__gc".as_ptr());
        lua_setmetatable(state, -2);
    }
}
pub unsafe fn luaopen_package(state: *mut State) -> i32 {
    unsafe {
        createclibstable(state);
        (*state).lua_createtable();
        lual_setfuncs(state, PACKAGE_FUNCTIONS.as_ptr(), PACKAGE_FUNCTIONS.len(), 0);
        createsearcherstable(state);
        setpath(
            state,
            c"path".as_ptr(),
            c"LUA_PATH".as_ptr(),
            c"/usr/local/share/lua/5.4/?.lua;/usr/local/share/lua/5.4/?/initial.lua;/usr/local/lib/lua/5.4/?.lua;/usr/local/lib/lua/5.4/?/initial.lua;./?.lua;./?/initial.lua".as_ptr() as *const u8 as *const i8,
        );
        setpath(
            state,
            c"cpath".as_ptr(),
            c"LUA_CPATH".as_ptr(),
            c"/usr/local/lib/lua/5.4/?.so;/usr/local/lib/lua/5.4/loadall.so;./?.so".as_ptr(),
        );
        lua_pushstring(state, c"/\n;\n?\n!\n-\n".as_ptr());
        lua_setfield(state, -2, c"config".as_ptr());
        lual_getsubtable(state, LUA_REGISTRYINDEX, c"_LOADED".as_ptr());
        lua_setfield(state, -2, c"loaded".as_ptr());
        lual_getsubtable(state, LUA_REGISTRYINDEX, c"_PRELOAD".as_ptr());
        lua_setfield(state, -2, c"preload".as_ptr());
        lua_rawgeti(state, LUA_REGISTRYINDEX, 2_i64);
        lua_pushvalue(state, -2);
        lual_setfuncs(state, LL_FUNCTIONS.as_ptr(), LL_FUNCTIONS.len(), 1);
        lua_settop(state, -2);
        1
    }
}
