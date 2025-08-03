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
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type lua_State;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: i32) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn lua_settop(L: *mut lua_State, index: i32);
    fn lua_pushvalue(L: *mut lua_State, index: i32);
    fn lua_rotate(L: *mut lua_State, index: i32, n: i32);
    fn lua_copy(L: *mut lua_State, fromidx: i32, toidx: i32);
    fn lua_isstring(L: *mut lua_State, index: i32) -> i32;
    fn lua_type(L: *mut lua_State, index: i32) -> i32;
    fn lua_toboolean(L: *mut lua_State, index: i32) -> i32;
    fn lua_tolstring(
        L: *mut lua_State,
        index: i32,
        len: *mut size_t,
    ) -> *const libc::c_char;
    fn lua_touserdata(L: *mut lua_State, index: i32) -> *mut libc::c_void;
    fn lua_pushnil(L: *mut lua_State);
    fn lua_pushlstring(
        L: *mut lua_State,
        s: *const libc::c_char,
        len: size_t,
    ) -> *const libc::c_char;
    fn lua_pushstring(L: *mut lua_State, s: *const libc::c_char) -> *const libc::c_char;
    fn lua_pushfstring(
        L: *mut lua_State,
        fmt: *const libc::c_char,
        _: ...
    ) -> *const libc::c_char;
    fn lua_pushcclosure(L: *mut lua_State, fn_0: CFunction, n: i32);
    fn lua_pushboolean(L: *mut lua_State, b: i32);
    fn lua_pushlightuserdata(L: *mut lua_State, p: *mut libc::c_void);
    fn lua_getfield(
        L: *mut lua_State,
        index: i32,
        k: *const libc::c_char,
    ) -> i32;
    fn lua_rawgeti(L: *mut lua_State, index: i32, n: Integer) -> i32;
    fn lua_createtable(L: *mut lua_State, narr: i32, nrec: i32);
    fn lua_setfield(L: *mut lua_State, index: i32, k: *const libc::c_char);
    fn lua_rawseti(L: *mut lua_State, index: i32, n: Integer);
    fn lua_setmetatable(L: *mut lua_State, objindex: i32) -> i32;
    fn lua_callk(
        L: *mut lua_State,
        nargs: i32,
        nresults: i32,
        ctx: lua_KContext,
        k: lua_KFunction,
    );
    fn luaL_checkversion_(L: *mut lua_State, ver: Number, sz: size_t);
    fn luaL_checklstring(
        L: *mut lua_State,
        arg: i32,
        l: *mut size_t,
    ) -> *const libc::c_char;
    fn luaL_optlstring(
        L: *mut lua_State,
        arg: i32,
        def: *const libc::c_char,
        l: *mut size_t,
    ) -> *const libc::c_char;
    fn luaL_error(L: *mut lua_State, fmt: *const libc::c_char, _: ...) -> i32;
    fn luaL_loadfilex(
        L: *mut lua_State,
        filename: *const libc::c_char,
        mode: *const libc::c_char,
    ) -> i32;
    fn luaL_len(L: *mut lua_State, index: i32) -> Integer;
    fn luaL_addgsub(
        b: *mut luaL_Buffer,
        s: *const libc::c_char,
        p: *const libc::c_char,
        r: *const libc::c_char,
    );
    fn luaL_gsub(
        L: *mut lua_State,
        s: *const libc::c_char,
        p: *const libc::c_char,
        r: *const libc::c_char,
    ) -> *const libc::c_char;
    fn luaL_setfuncs(L: *mut lua_State, l: *const luaL_Reg, nup: i32);
    fn luaL_getsubtable(
        L: *mut lua_State,
        index: i32,
        fname: *const libc::c_char,
    ) -> i32;
    fn luaL_buffinit(L: *mut lua_State, B: *mut luaL_Buffer);
    fn luaL_prepbuffsize(B: *mut luaL_Buffer, sz: size_t) -> *mut libc::c_char;
    fn luaL_addlstring(B: *mut luaL_Buffer, s: *const libc::c_char, l: size_t);
    fn luaL_addstring(B: *mut luaL_Buffer, s: *const libc::c_char);
    fn luaL_addvalue(B: *mut luaL_Buffer);
    fn luaL_pushresult(B: *mut luaL_Buffer);
    fn dlopen(__file: *const libc::c_char, __mode: i32) -> *mut libc::c_void;
    fn dlclose(__handle: *mut libc::c_void) -> i32;
    fn dlsym(
        __handle: *mut libc::c_void,
        __name: *const libc::c_char,
    ) -> *mut libc::c_void;
    fn dlerror() -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: i32,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: i32,
    pub _flags2: i32,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: i32,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type intptr_t = libc::c_long;
pub type Number = f64;
pub type Integer = i64;
pub type lua_KContext = intptr_t;
pub type CFunction = Option::<unsafe extern "C" fn(*mut lua_State) -> i32>;
pub type lua_KFunction = Option::<
    unsafe extern "C" fn(*mut lua_State, i32, lua_KContext) -> i32,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaL_Buffer {
    pub b: *mut libc::c_char,
    pub size: size_t,
    pub n: size_t,
    pub L: *mut lua_State,
    pub init: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub n: Number,
    pub u: f64,
    pub s: *mut libc::c_void,
    pub i: Integer,
    pub l: libc::c_long,
    pub b: [libc::c_char; 1024],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaL_Reg {
    pub name: *const libc::c_char,
    pub func: CFunction,
}
static mut CLIBS: *const libc::c_char = b"_CLIBS\0" as *const u8 as *const libc::c_char;
unsafe extern "C" fn lsys_unloadlib(mut lib: *mut libc::c_void) {
    dlclose(lib);
}
unsafe extern "C" fn lsys_load(
    mut L: *mut lua_State,
    mut path: *const libc::c_char,
    mut seeglb: i32,
) -> *mut libc::c_void {
    let mut lib: *mut libc::c_void = dlopen(
        path,
        0x2 as i32
            | (if seeglb != 0 { 0x100 as i32 } else { 0 as i32 }),
    );
    if ((lib == 0 as *mut libc::c_void) as i32 != 0 as i32)
        as i32 as libc::c_long != 0
    {
        lua_pushstring(L, dlerror());
    }
    return lib;
}
unsafe extern "C" fn lsys_sym(
    mut L: *mut lua_State,
    mut lib: *mut libc::c_void,
    mut sym: *const libc::c_char,
) -> CFunction {
    let mut f: CFunction = ::core::mem::transmute::<
        *mut libc::c_void,
        CFunction,
    >(dlsym(lib, sym));
    if (f.is_none() as i32 != 0 as i32) as i32 as libc::c_long
        != 0
    {
        lua_pushstring(L, dlerror());
    }
    return f;
}
unsafe extern "C" fn noenv(mut L: *mut lua_State) -> i32 {
    let mut b: i32 = 0;
    lua_getfield(
        L,
        -(1000000 as i32) - 1000 as i32,
        b"LUA_NOENV\0" as *const u8 as *const libc::c_char,
    );
    b = lua_toboolean(L, -(1 as i32));
    lua_settop(L, -(1 as i32) - 1 as i32);
    return b;
}
unsafe extern "C" fn setpath(
    mut L: *mut lua_State,
    mut fieldname: *const libc::c_char,
    mut envname: *const libc::c_char,
    mut dft: *const libc::c_char,
) {
    let mut dftmark: *const libc::c_char = 0 as *const libc::c_char;
    let mut nver: *const libc::c_char = lua_pushfstring(
        L,
        b"%s%s\0" as *const u8 as *const libc::c_char,
        envname,
        b"_5_4\0" as *const u8 as *const libc::c_char,
    );
    let mut path: *const libc::c_char = getenv(nver);
    if path.is_null() {
        path = getenv(envname);
    }
    if path.is_null() || noenv(L) != 0 {
        lua_pushstring(L, dft);
    } else {
        dftmark = strstr(path, b";;\0" as *const u8 as *const libc::c_char);
        if dftmark.is_null() {
            lua_pushstring(L, path);
        } else {
            let mut len: size_t = strlen(path);
            let mut b: luaL_Buffer = luaL_Buffer {
                b: 0 as *mut libc::c_char,
                size: 0,
                n: 0,
                L: 0 as *mut lua_State,
                init: C2RustUnnamed { n: 0. },
            };
            luaL_buffinit(L, &mut b);
            if path < dftmark {
                luaL_addlstring(
                    &mut b,
                    path,
                    dftmark.offset_from(path) as libc::c_long as size_t,
                );
                (b.n < b.size
                    || !(luaL_prepbuffsize(&mut b, 1 as i32 as size_t))
                        .is_null()) as i32;
                let fresh0 = b.n;
                b.n = (b.n).wrapping_add(1);
                *(b.b)
                    .offset(
                        fresh0 as isize,
                    ) = *(b";\0" as *const u8 as *const libc::c_char);
            }
            luaL_addstring(&mut b, dft);
            if dftmark < path.offset(len as isize).offset(-(2 as i32 as isize)) {
                (b.n < b.size
                    || !(luaL_prepbuffsize(&mut b, 1 as i32 as size_t))
                        .is_null()) as i32;
                let fresh1 = b.n;
                b.n = (b.n).wrapping_add(1);
                *(b.b)
                    .offset(
                        fresh1 as isize,
                    ) = *(b";\0" as *const u8 as *const libc::c_char);
                luaL_addlstring(
                    &mut b,
                    dftmark.offset(2 as i32 as isize),
                    path
                        .offset(len as isize)
                        .offset(-(2 as i32 as isize))
                        .offset_from(dftmark) as libc::c_long as size_t,
                );
            }
            luaL_pushresult(&mut b);
        }
    }
    lua_setfield(L, -(3 as i32), fieldname);
    lua_settop(L, -(1 as i32) - 1 as i32);
}
unsafe extern "C" fn checkclib(
    mut L: *mut lua_State,
    mut path: *const libc::c_char,
) -> *mut libc::c_void {
    let mut plib: *mut libc::c_void = 0 as *mut libc::c_void;
    lua_getfield(L, -(1000000 as i32) - 1000 as i32, CLIBS);
    lua_getfield(L, -(1 as i32), path);
    plib = lua_touserdata(L, -(1 as i32));
    lua_settop(L, -(2 as i32) - 1 as i32);
    return plib;
}
unsafe extern "C" fn addtoclib(
    mut L: *mut lua_State,
    mut path: *const libc::c_char,
    mut plib: *mut libc::c_void,
) {
    lua_getfield(L, -(1000000 as i32) - 1000 as i32, CLIBS);
    lua_pushlightuserdata(L, plib);
    lua_pushvalue(L, -(1 as i32));
    lua_setfield(L, -(3 as i32), path);
    lua_rawseti(
        L,
        -(2 as i32),
        luaL_len(L, -(2 as i32)) + 1 as i32 as i64,
    );
    lua_settop(L, -(1 as i32) - 1 as i32);
}
unsafe extern "C" fn gctm(mut L: *mut lua_State) -> i32 {
    let mut n: Integer = luaL_len(L, 1 as i32);
    while n >= 1 as i32 as i64 {
        lua_rawgeti(L, 1 as i32, n);
        lsys_unloadlib(lua_touserdata(L, -(1 as i32)));
        lua_settop(L, -(1 as i32) - 1 as i32);
        n -= 1;
        n;
    }
    return 0 as i32;
}
unsafe extern "C" fn lookforfunc(
    mut L: *mut lua_State,
    mut path: *const libc::c_char,
    mut sym: *const libc::c_char,
) -> i32 {
    let mut reg: *mut libc::c_void = checkclib(L, path);
    if reg.is_null() {
        reg = lsys_load(L, path, (*sym as i32 == '*' as i32) as i32);
        if reg.is_null() {
            return 1 as i32;
        }
        addtoclib(L, path, reg);
    }
    if *sym as i32 == '*' as i32 {
        lua_pushboolean(L, 1 as i32);
        return 0 as i32;
    } else {
        let mut f: CFunction = lsys_sym(L, reg, sym);
        if f.is_none() {
            return 2 as i32;
        }
        lua_pushcclosure(L, f, 0 as i32);
        return 0 as i32;
    };
}
unsafe extern "C" fn ll_loadlib(mut L: *mut lua_State) -> i32 {
    let mut path: *const libc::c_char = luaL_checklstring(
        L,
        1 as i32,
        0 as *mut size_t,
    );
    let mut init: *const libc::c_char = luaL_checklstring(
        L,
        2 as i32,
        0 as *mut size_t,
    );
    let mut stat: i32 = lookforfunc(L, path, init);
    if ((stat == 0 as i32) as i32 != 0 as i32) as i32
        as libc::c_long != 0
    {
        return 1 as i32
    } else {
        lua_pushnil(L);
        lua_rotate(L, -(2 as i32), 1 as i32);
        lua_pushstring(
            L,
            if stat == 1 as i32 {
                b"open\0" as *const u8 as *const libc::c_char
            } else {
                b"init\0" as *const u8 as *const libc::c_char
            },
        );
        return 3 as i32;
    };
}
unsafe extern "C" fn readable(mut filename: *const libc::c_char) -> i32 {
    let mut f: *mut FILE = fopen(filename, b"r\0" as *const u8 as *const libc::c_char);
    if f.is_null() {
        return 0 as i32;
    }
    fclose(f);
    return 1 as i32;
}
unsafe extern "C" fn getnextfilename(
    mut path: *mut *mut libc::c_char,
    mut end: *mut libc::c_char,
) -> *const libc::c_char {
    let mut sep: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut name: *mut libc::c_char = *path;
    if name == end {
        return 0 as *const libc::c_char
    } else if *name as i32 == '\0' as i32 {
        *name = *(b";\0" as *const u8 as *const libc::c_char);
        name = name.offset(1);
        name;
    }
    sep = strchr(name, *(b";\0" as *const u8 as *const libc::c_char) as i32);
    if sep.is_null() {
        sep = end;
    }
    *sep = '\0' as i32 as libc::c_char;
    *path = sep;
    return name;
}
unsafe extern "C" fn pusherrornotfound(
    mut L: *mut lua_State,
    mut path: *const libc::c_char,
) {
    let mut b: luaL_Buffer = luaL_Buffer {
        b: 0 as *mut libc::c_char,
        size: 0,
        n: 0,
        L: 0 as *mut lua_State,
        init: C2RustUnnamed { n: 0. },
    };
    luaL_buffinit(L, &mut b);
    luaL_addstring(&mut b, b"no file '\0" as *const u8 as *const libc::c_char);
    luaL_addgsub(
        &mut b,
        path,
        b";\0" as *const u8 as *const libc::c_char,
        b"'\n\tno file '\0" as *const u8 as *const libc::c_char,
    );
    luaL_addstring(&mut b, b"'\0" as *const u8 as *const libc::c_char);
    luaL_pushresult(&mut b);
}
unsafe extern "C" fn searchpath(
    mut L: *mut lua_State,
    mut name: *const libc::c_char,
    mut path: *const libc::c_char,
    mut sep: *const libc::c_char,
    mut dirsep: *const libc::c_char,
) -> *const libc::c_char {
    let mut buff: luaL_Buffer = luaL_Buffer {
        b: 0 as *mut libc::c_char,
        size: 0,
        n: 0,
        L: 0 as *mut lua_State,
        init: C2RustUnnamed { n: 0. },
    };
    let mut pathname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut endpathname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut filename: *const libc::c_char = 0 as *const libc::c_char;
    if *sep as i32 != '\0' as i32
        && !(strchr(name, *sep as i32)).is_null()
    {
        name = luaL_gsub(L, name, sep, dirsep);
    }
    luaL_buffinit(L, &mut buff);
    luaL_addgsub(&mut buff, path, b"?\0" as *const u8 as *const libc::c_char, name);
    (buff.n < buff.size
        || !(luaL_prepbuffsize(&mut buff, 1 as i32 as size_t)).is_null())
        as i32;
    let fresh2 = buff.n;
    buff.n = (buff.n).wrapping_add(1);
    *(buff.b).offset(fresh2 as isize) = '\0' as i32 as libc::c_char;
    pathname = buff.b;
    endpathname = pathname.offset(buff.n as isize).offset(-(1 as i32 as isize));
    loop {
        filename = getnextfilename(&mut pathname, endpathname);
        if filename.is_null() {
            break;
        }
        if readable(filename) != 0 {
            return lua_pushstring(L, filename);
        }
    }
    luaL_pushresult(&mut buff);
    pusherrornotfound(L, lua_tolstring(L, -(1 as i32), 0 as *mut size_t));
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn ll_searchpath(mut L: *mut lua_State) -> i32 {
    let mut f: *const libc::c_char = searchpath(
        L,
        luaL_checklstring(L, 1 as i32, 0 as *mut size_t),
        luaL_checklstring(L, 2 as i32, 0 as *mut size_t),
        luaL_optlstring(
            L,
            3 as i32,
            b".\0" as *const u8 as *const libc::c_char,
            0 as *mut size_t,
        ),
        luaL_optlstring(
            L,
            4 as i32,
            b"/\0" as *const u8 as *const libc::c_char,
            0 as *mut size_t,
        ),
    );
    if !f.is_null() {
        return 1 as i32
    } else {
        lua_pushnil(L);
        lua_rotate(L, -(2 as i32), 1 as i32);
        return 2 as i32;
    };
}
unsafe extern "C" fn findfile(
    mut L: *mut lua_State,
    mut name: *const libc::c_char,
    mut pname: *const libc::c_char,
    mut dirsep: *const libc::c_char,
) -> *const libc::c_char {
    let mut path: *const libc::c_char = 0 as *const libc::c_char;
    lua_getfield(
        L,
        -(1000000 as i32) - 1000 as i32 - 1 as i32,
        pname,
    );
    path = lua_tolstring(L, -(1 as i32), 0 as *mut size_t);
    if ((path == 0 as *mut libc::c_void as *const libc::c_char) as i32
        != 0 as i32) as i32 as libc::c_long != 0
    {
        luaL_error(
            L,
            b"'package.%s' must be a string\0" as *const u8 as *const libc::c_char,
            pname,
        );
    }
    return searchpath(L, name, path, b".\0" as *const u8 as *const libc::c_char, dirsep);
}
unsafe extern "C" fn checkload(
    mut L: *mut lua_State,
    mut stat: i32,
    mut filename: *const libc::c_char,
) -> i32 {
    if (stat != 0 as i32) as i32 as libc::c_long != 0 {
        lua_pushstring(L, filename);
        return 2 as i32;
    } else {
        return luaL_error(
            L,
            b"error loading module '%s' from file '%s':\n\t%s\0" as *const u8
                as *const libc::c_char,
            lua_tolstring(L, 1 as i32, 0 as *mut size_t),
            filename,
            lua_tolstring(L, -(1 as i32), 0 as *mut size_t),
        )
    };
}
unsafe extern "C" fn searcher_Lua(mut L: *mut lua_State) -> i32 {
    let mut filename: *const libc::c_char = 0 as *const libc::c_char;
    let mut name: *const libc::c_char = luaL_checklstring(
        L,
        1 as i32,
        0 as *mut size_t,
    );
    filename = findfile(
        L,
        name,
        b"path\0" as *const u8 as *const libc::c_char,
        b"/\0" as *const u8 as *const libc::c_char,
    );
    if filename.is_null() {
        return 1 as i32;
    }
    return checkload(
        L,
        (luaL_loadfilex(L, filename, 0 as *const libc::c_char) == 0 as i32)
            as i32,
        filename,
    );
}
unsafe extern "C" fn loadfunc(
    mut L: *mut lua_State,
    mut filename: *const libc::c_char,
    mut modname: *const libc::c_char,
) -> i32 {
    let mut openfunc: *const libc::c_char = 0 as *const libc::c_char;
    let mut mark: *const libc::c_char = 0 as *const libc::c_char;
    modname = luaL_gsub(
        L,
        modname,
        b".\0" as *const u8 as *const libc::c_char,
        b"_\0" as *const u8 as *const libc::c_char,
    );
    mark = strchr(modname, *(b"-\0" as *const u8 as *const libc::c_char) as i32);
    if !mark.is_null() {
        let mut stat: i32 = 0;
        openfunc = lua_pushlstring(
            L,
            modname,
            mark.offset_from(modname) as libc::c_long as size_t,
        );
        openfunc = lua_pushfstring(
            L,
            b"luaopen_%s\0" as *const u8 as *const libc::c_char,
            openfunc,
        );
        stat = lookforfunc(L, filename, openfunc);
        if stat != 2 as i32 {
            return stat;
        }
        modname = mark.offset(1 as i32 as isize);
    }
    openfunc = lua_pushfstring(
        L,
        b"luaopen_%s\0" as *const u8 as *const libc::c_char,
        modname,
    );
    return lookforfunc(L, filename, openfunc);
}
unsafe extern "C" fn searcher_C(mut L: *mut lua_State) -> i32 {
    let mut name: *const libc::c_char = luaL_checklstring(
        L,
        1 as i32,
        0 as *mut size_t,
    );
    let mut filename: *const libc::c_char = findfile(
        L,
        name,
        b"cpath\0" as *const u8 as *const libc::c_char,
        b"/\0" as *const u8 as *const libc::c_char,
    );
    if filename.is_null() {
        return 1 as i32;
    }
    return checkload(
        L,
        (loadfunc(L, filename, name) == 0 as i32) as i32,
        filename,
    );
}
unsafe extern "C" fn searcher_Croot(mut L: *mut lua_State) -> i32 {
    let mut filename: *const libc::c_char = 0 as *const libc::c_char;
    let mut name: *const libc::c_char = luaL_checklstring(
        L,
        1 as i32,
        0 as *mut size_t,
    );
    let mut p: *const libc::c_char = strchr(name, '.' as i32);
    let mut stat: i32 = 0;
    if p.is_null() {
        return 0 as i32;
    }
    lua_pushlstring(L, name, p.offset_from(name) as libc::c_long as size_t);
    filename = findfile(
        L,
        lua_tolstring(L, -(1 as i32), 0 as *mut size_t),
        b"cpath\0" as *const u8 as *const libc::c_char,
        b"/\0" as *const u8 as *const libc::c_char,
    );
    if filename.is_null() {
        return 1 as i32;
    }
    stat = loadfunc(L, filename, name);
    if stat != 0 as i32 {
        if stat != 2 as i32 {
            return checkload(L, 0 as i32, filename)
        } else {
            lua_pushfstring(
                L,
                b"no module '%s' in file '%s'\0" as *const u8 as *const libc::c_char,
                name,
                filename,
            );
            return 1 as i32;
        }
    }
    lua_pushstring(L, filename);
    return 2 as i32;
}
unsafe extern "C" fn searcher_preload(mut L: *mut lua_State) -> i32 {
    let mut name: *const libc::c_char = luaL_checklstring(
        L,
        1 as i32,
        0 as *mut size_t,
    );
    lua_getfield(
        L,
        -(1000000 as i32) - 1000 as i32,
        b"_PRELOAD\0" as *const u8 as *const libc::c_char,
    );
    if lua_getfield(L, -(1 as i32), name) == 0 as i32 {
        lua_pushfstring(
            L,
            b"no field package.preload['%s']\0" as *const u8 as *const libc::c_char,
            name,
        );
        return 1 as i32;
    } else {
        lua_pushstring(L, b":preload:\0" as *const u8 as *const libc::c_char);
        return 2 as i32;
    };
}
unsafe extern "C" fn findloader(mut L: *mut lua_State, mut name: *const libc::c_char) {
    let mut i: i32 = 0;
    let mut msg: luaL_Buffer = luaL_Buffer {
        b: 0 as *mut libc::c_char,
        size: 0,
        n: 0,
        L: 0 as *mut lua_State,
        init: C2RustUnnamed { n: 0. },
    };
    if ((lua_getfield(
        L,
        -(1000000 as i32) - 1000 as i32 - 1 as i32,
        b"searchers\0" as *const u8 as *const libc::c_char,
    ) != 5 as i32) as i32 != 0 as i32) as i32
        as libc::c_long != 0
    {
        luaL_error(
            L,
            b"'package.searchers' must be a table\0" as *const u8 as *const libc::c_char,
        );
    }
    luaL_buffinit(L, &mut msg);
    i = 1 as i32;
    loop {
        luaL_addstring(&mut msg, b"\n\t\0" as *const u8 as *const libc::c_char);
        if ((lua_rawgeti(L, 3 as i32, i as Integer) == 0 as i32)
            as i32 != 0 as i32) as i32 as libc::c_long != 0
        {
            lua_settop(L, -(1 as i32) - 1 as i32);
            msg
                .n = (msg.n as libc::c_ulong)
                .wrapping_sub(2 as i32 as libc::c_ulong) as size_t as size_t;
            luaL_pushresult(&mut msg);
            luaL_error(
                L,
                b"module '%s' not found:%s\0" as *const u8 as *const libc::c_char,
                name,
                lua_tolstring(L, -(1 as i32), 0 as *mut size_t),
            );
        }
        lua_pushstring(L, name);
        lua_callk(
            L,
            1 as i32,
            2 as i32,
            0 as i32 as lua_KContext,
            None,
        );
        if lua_type(L, -(2 as i32)) == 6 as i32 {
            return
        } else if lua_isstring(L, -(2 as i32)) != 0 {
            lua_settop(L, -(1 as i32) - 1 as i32);
            luaL_addvalue(&mut msg);
        } else {
            lua_settop(L, -(2 as i32) - 1 as i32);
            msg
                .n = (msg.n as libc::c_ulong)
                .wrapping_sub(2 as i32 as libc::c_ulong) as size_t as size_t;
        }
        i += 1;
        i;
    };
}
unsafe extern "C" fn ll_require(mut L: *mut lua_State) -> i32 {
    let mut name: *const libc::c_char = luaL_checklstring(
        L,
        1 as i32,
        0 as *mut size_t,
    );
    lua_settop(L, 1 as i32);
    lua_getfield(
        L,
        -(1000000 as i32) - 1000 as i32,
        b"_LOADED\0" as *const u8 as *const libc::c_char,
    );
    lua_getfield(L, 2 as i32, name);
    if lua_toboolean(L, -(1 as i32)) != 0 {
        return 1 as i32;
    }
    lua_settop(L, -(1 as i32) - 1 as i32);
    findloader(L, name);
    lua_rotate(L, -(2 as i32), 1 as i32);
    lua_pushvalue(L, 1 as i32);
    lua_pushvalue(L, -(3 as i32));
    lua_callk(
        L,
        2 as i32,
        1 as i32,
        0 as i32 as lua_KContext,
        None,
    );
    if !(lua_type(L, -(1 as i32)) == 0 as i32) {
        lua_setfield(L, 2 as i32, name);
    } else {
        lua_settop(L, -(1 as i32) - 1 as i32);
    }
    if lua_getfield(L, 2 as i32, name) == 0 as i32 {
        lua_pushboolean(L, 1 as i32);
        lua_copy(L, -(1 as i32), -(2 as i32));
        lua_setfield(L, 2 as i32, name);
    }
    lua_rotate(L, -(2 as i32), 1 as i32);
    return 2 as i32;
}
static mut pk_funcs: [luaL_Reg; 8] = unsafe {
    [
        {
            let mut init = luaL_Reg {
                name: b"loadlib\0" as *const u8 as *const libc::c_char,
                func: Some(
                    ll_loadlib as unsafe extern "C" fn(*mut lua_State) -> i32,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"searchpath\0" as *const u8 as *const libc::c_char,
                func: Some(
                    ll_searchpath as unsafe extern "C" fn(*mut lua_State) -> i32,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"preload\0" as *const u8 as *const libc::c_char,
                func: None,
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"cpath\0" as *const u8 as *const libc::c_char,
                func: None,
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"path\0" as *const u8 as *const libc::c_char,
                func: None,
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"searchers\0" as *const u8 as *const libc::c_char,
                func: None,
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"loaded\0" as *const u8 as *const libc::c_char,
                func: None,
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
static mut ll_funcs: [luaL_Reg; 2] = unsafe {
    [
        {
            let mut init = luaL_Reg {
                name: b"require\0" as *const u8 as *const libc::c_char,
                func: Some(
                    ll_require as unsafe extern "C" fn(*mut lua_State) -> i32,
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
unsafe extern "C" fn createsearcherstable(mut L: *mut lua_State) {
    static mut searchers: [CFunction; 5] = unsafe {
        [
            Some(
                searcher_preload as unsafe extern "C" fn(*mut lua_State) -> i32,
            ),
            Some(searcher_Lua as unsafe extern "C" fn(*mut lua_State) -> i32),
            Some(searcher_C as unsafe extern "C" fn(*mut lua_State) -> i32),
            Some(searcher_Croot as unsafe extern "C" fn(*mut lua_State) -> i32),
            None,
        ]
    };
    let mut i: i32 = 0;
    lua_createtable(
        L,
        (::core::mem::size_of::<[CFunction; 5]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<CFunction>() as libc::c_ulong)
            .wrapping_sub(1 as i32 as libc::c_ulong) as i32,
        0 as i32,
    );
    i = 0 as i32;
    while (searchers[i as usize]).is_some() {
        lua_pushvalue(L, -(2 as i32));
        lua_pushcclosure(L, searchers[i as usize], 1 as i32);
        lua_rawseti(L, -(2 as i32), (i + 1 as i32) as Integer);
        i += 1;
        i;
    }
    lua_setfield(
        L,
        -(2 as i32),
        b"searchers\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn createclibstable(mut L: *mut lua_State) {
    luaL_getsubtable(L, -(1000000 as i32) - 1000 as i32, CLIBS);
    lua_createtable(L, 0 as i32, 1 as i32);
    lua_pushcclosure(
        L,
        Some(gctm as unsafe extern "C" fn(*mut lua_State) -> i32),
        0 as i32,
    );
    lua_setfield(L, -(2 as i32), b"__gc\0" as *const u8 as *const libc::c_char);
    lua_setmetatable(L, -(2 as i32));
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaopen_package(mut L: *mut lua_State) -> i32 {
    createclibstable(L);
    luaL_checkversion_(
        L,
        504 as i32 as Number,
        (::core::mem::size_of::<Integer>() as libc::c_ulong)
            .wrapping_mul(16 as i32 as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<Number>() as libc::c_ulong),
    );
    lua_createtable(
        L,
        0 as i32,
        (::core::mem::size_of::<[luaL_Reg; 8]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<luaL_Reg>() as libc::c_ulong)
            .wrapping_sub(1 as i32 as libc::c_ulong) as i32,
    );
    luaL_setfuncs(L, pk_funcs.as_ptr(), 0 as i32);
    createsearcherstable(L);
    setpath(
        L,
        b"path\0" as *const u8 as *const libc::c_char,
        b"LUA_PATH\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/lua/5.4/?.lua;/usr/local/share/lua/5.4/?/init.lua;/usr/local/lib/lua/5.4/?.lua;/usr/local/lib/lua/5.4/?/init.lua;./?.lua;./?/init.lua\0"
            as *const u8 as *const libc::c_char,
    );
    setpath(
        L,
        b"cpath\0" as *const u8 as *const libc::c_char,
        b"LUA_CPATH\0" as *const u8 as *const libc::c_char,
        b"/usr/local/lib/lua/5.4/?.so;/usr/local/lib/lua/5.4/loadall.so;./?.so\0"
            as *const u8 as *const libc::c_char,
    );
    lua_pushstring(L, b"/\n;\n?\n!\n-\n\0" as *const u8 as *const libc::c_char);
    lua_setfield(
        L,
        -(2 as i32),
        b"config\0" as *const u8 as *const libc::c_char,
    );
    luaL_getsubtable(
        L,
        -(1000000 as i32) - 1000 as i32,
        b"_LOADED\0" as *const u8 as *const libc::c_char,
    );
    lua_setfield(
        L,
        -(2 as i32),
        b"loaded\0" as *const u8 as *const libc::c_char,
    );
    luaL_getsubtable(
        L,
        -(1000000 as i32) - 1000 as i32,
        b"_PRELOAD\0" as *const u8 as *const libc::c_char,
    );
    lua_setfield(
        L,
        -(2 as i32),
        b"preload\0" as *const u8 as *const libc::c_char,
    );
    lua_rawgeti(
        L,
        -(1000000 as i32) - 1000 as i32,
        2 as i32 as Integer,
    );
    lua_pushvalue(L, -(2 as i32));
    luaL_setfuncs(L, ll_funcs.as_ptr(), 1 as i32);
    lua_settop(L, -(1 as i32) - 1 as i32);
    return 1 as i32;
}
