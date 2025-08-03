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
    pub type CallInfo;
    static mut stdin: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn lua_settop(L: *mut lua_State, index: libc::c_int);
    fn lua_pushvalue(L: *mut lua_State, index: libc::c_int);
    fn lua_rotate(L: *mut lua_State, index: libc::c_int, n: libc::c_int);
    fn lua_checkstack(L: *mut lua_State, n: libc::c_int) -> libc::c_int;
    fn lua_xmove(from: *mut lua_State, to: *mut lua_State, n: libc::c_int);
    fn lua_iscfunction(L: *mut lua_State, index: libc::c_int) -> libc::c_int;
    fn lua_type(L: *mut lua_State, index: libc::c_int) -> libc::c_int;
    fn lua_tolstring(
        L: *mut lua_State,
        index: libc::c_int,
        len: *mut size_t,
    ) -> *const libc::c_char;
    fn lua_tothread(L: *mut lua_State, index: libc::c_int) -> *mut lua_State;
    fn lua_pushnil(L: *mut lua_State);
    fn lua_pushinteger(L: *mut lua_State, n: Integer);
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
    fn lua_pushboolean(L: *mut lua_State, b: libc::c_int);
    fn lua_pushlightuserdata(L: *mut lua_State, p: *mut libc::c_void);
    fn lua_pushthread(L: *mut lua_State) -> libc::c_int;
    fn lua_getfield(
        L: *mut lua_State,
        index: libc::c_int,
        k: *const libc::c_char,
    ) -> libc::c_int;
    fn lua_rawget(L: *mut lua_State, index: libc::c_int) -> libc::c_int;
    fn lua_createtable(L: *mut lua_State, narr: libc::c_int, nrec: libc::c_int);
    fn lua_getmetatable(L: *mut lua_State, objindex: libc::c_int) -> libc::c_int;
    fn lua_getiuservalue(
        L: *mut lua_State,
        index: libc::c_int,
        n: libc::c_int,
    ) -> libc::c_int;
    fn lua_setfield(L: *mut lua_State, index: libc::c_int, k: *const libc::c_char);
    fn lua_rawset(L: *mut lua_State, index: libc::c_int);
    fn lua_setmetatable(L: *mut lua_State, objindex: libc::c_int) -> libc::c_int;
    fn lua_setiuservalue(
        L: *mut lua_State,
        index: libc::c_int,
        n: libc::c_int,
    ) -> libc::c_int;
    fn lua_callk(
        L: *mut lua_State,
        nargs: libc::c_int,
        nresults: libc::c_int,
        ctx: lua_KContext,
        k: lua_KFunction,
    );
    fn lua_pcallk(
        L: *mut lua_State,
        nargs: libc::c_int,
        nresults: libc::c_int,
        errfunc: libc::c_int,
        ctx: lua_KContext,
        k: lua_KFunction,
    ) -> libc::c_int;
    fn lua_getstack(
        L: *mut lua_State,
        level: libc::c_int,
        ar: *mut lua_Debug,
    ) -> libc::c_int;
    fn lua_getinfo(
        L: *mut lua_State,
        what: *const libc::c_char,
        ar: *mut lua_Debug,
    ) -> libc::c_int;
    fn lua_getlocal(
        L: *mut lua_State,
        ar: *const lua_Debug,
        n: libc::c_int,
    ) -> *const libc::c_char;
    fn lua_setlocal(
        L: *mut lua_State,
        ar: *const lua_Debug,
        n: libc::c_int,
    ) -> *const libc::c_char;
    fn lua_getupvalue(
        L: *mut lua_State,
        funcindex: libc::c_int,
        n: libc::c_int,
    ) -> *const libc::c_char;
    fn lua_setupvalue(
        L: *mut lua_State,
        funcindex: libc::c_int,
        n: libc::c_int,
    ) -> *const libc::c_char;
    fn lua_upvalueid(
        L: *mut lua_State,
        fidx: libc::c_int,
        n: libc::c_int,
    ) -> *mut libc::c_void;
    fn lua_upvaluejoin(
        L: *mut lua_State,
        fidx1: libc::c_int,
        n1: libc::c_int,
        fidx2: libc::c_int,
        n2: libc::c_int,
    );
    fn lua_sethook(
        L: *mut lua_State,
        func: lua_Hook,
        mask: libc::c_int,
        count: libc::c_int,
    );
    fn lua_gethook(L: *mut lua_State) -> lua_Hook;
    fn lua_gethookmask(L: *mut lua_State) -> libc::c_int;
    fn lua_gethookcount(L: *mut lua_State) -> libc::c_int;
    fn lua_setcstacklimit(L: *mut lua_State, limit: libc::c_uint) -> libc::c_int;
    fn luaL_checkversion_(L: *mut lua_State, ver: Number, sz: size_t);
    fn luaL_tolstring(
        L: *mut lua_State,
        index: libc::c_int,
        len: *mut size_t,
    ) -> *const libc::c_char;
    fn luaL_argerror(
        L: *mut lua_State,
        arg: libc::c_int,
        extramsg: *const libc::c_char,
    ) -> libc::c_int;
    fn luaL_typeerror(
        L: *mut lua_State,
        arg: libc::c_int,
        tname: *const libc::c_char,
    ) -> libc::c_int;
    fn luaL_checklstring(
        L: *mut lua_State,
        arg: libc::c_int,
        l: *mut size_t,
    ) -> *const libc::c_char;
    fn luaL_optlstring(
        L: *mut lua_State,
        arg: libc::c_int,
        def: *const libc::c_char,
        l: *mut size_t,
    ) -> *const libc::c_char;
    fn luaL_checkinteger(L: *mut lua_State, arg: libc::c_int) -> Integer;
    fn luaL_optinteger(
        L: *mut lua_State,
        arg: libc::c_int,
        def: Integer,
    ) -> Integer;
    fn luaL_checktype(L: *mut lua_State, arg: libc::c_int, t: libc::c_int);
    fn luaL_checkany(L: *mut lua_State, arg: libc::c_int);
    fn luaL_error(L: *mut lua_State, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn luaL_loadbufferx(
        L: *mut lua_State,
        buff: *const libc::c_char,
        sz: size_t,
        name: *const libc::c_char,
        mode: *const libc::c_char,
    ) -> libc::c_int;
    fn luaL_setfuncs(L: *mut lua_State, l: *const luaL_Reg, nup: libc::c_int);
    fn luaL_getsubtable(
        L: *mut lua_State,
        index: libc::c_int,
        fname: *const libc::c_char,
    ) -> libc::c_int;
    fn luaL_traceback(
        L: *mut lua_State,
        L1: *mut lua_State,
        msg: *const libc::c_char,
        level: libc::c_int,
    );
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
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
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
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
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type intptr_t = libc::c_long;
pub type Number = f64;
pub type Integer = i64;
pub type lua_KContext = intptr_t;
pub type CFunction = Option::<unsafe extern "C" fn(*mut lua_State) -> libc::c_int>;
pub type lua_KFunction = Option::<
    unsafe extern "C" fn(*mut lua_State, libc::c_int, lua_KContext) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lua_Debug {
    pub event: libc::c_int,
    pub name: *const libc::c_char,
    pub namewhat: *const libc::c_char,
    pub what: *const libc::c_char,
    pub source: *const libc::c_char,
    pub srclen: size_t,
    pub currentline: libc::c_int,
    pub linedefined: libc::c_int,
    pub lastlinedefined: libc::c_int,
    pub nups: u8,
    pub nparams: u8,
    pub isvararg: libc::c_char,
    pub istailcall: libc::c_char,
    pub ftransfer: libc::c_ushort,
    pub ntransfer: libc::c_ushort,
    pub short_src: [libc::c_char; 60],
    pub i_ci: *mut CallInfo,
}
pub type lua_Hook = Option::<unsafe extern "C" fn(*mut lua_State, *mut lua_Debug) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaL_Reg {
    pub name: *const libc::c_char,
    pub func: CFunction,
}
static mut HOOKKEY: *const libc::c_char = b"_HOOKKEY\0" as *const u8
    as *const libc::c_char;
unsafe extern "C" fn checkstack(
    mut L: *mut lua_State,
    mut L1: *mut lua_State,
    mut n: libc::c_int,
) {
    if ((L != L1 && lua_checkstack(L1, n) == 0) as libc::c_int != 0 as libc::c_int)
        as libc::c_int as libc::c_long != 0
    {
        luaL_error(L, b"stack overflow\0" as *const u8 as *const libc::c_char);
    }
}
unsafe extern "C" fn db_getregistry(mut L: *mut lua_State) -> libc::c_int {
    lua_pushvalue(L, -(1000000 as libc::c_int) - 1000 as libc::c_int);
    return 1 as libc::c_int;
}
unsafe extern "C" fn db_getmetatable(mut L: *mut lua_State) -> libc::c_int {
    luaL_checkany(L, 1 as libc::c_int);
    if lua_getmetatable(L, 1 as libc::c_int) == 0 {
        lua_pushnil(L);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn db_setmetatable(mut L: *mut lua_State) -> libc::c_int {
    let mut t: libc::c_int = lua_type(L, 2 as libc::c_int);
    (((t == 0 as libc::c_int || t == 5 as libc::c_int) as libc::c_int
        != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
        || luaL_typeerror(
            L,
            2 as libc::c_int,
            b"nil or table\0" as *const u8 as *const libc::c_char,
        ) != 0) as libc::c_int;
    lua_settop(L, 2 as libc::c_int);
    lua_setmetatable(L, 1 as libc::c_int);
    return 1 as libc::c_int;
}
unsafe extern "C" fn db_getuservalue(mut L: *mut lua_State) -> libc::c_int {
    let mut n: libc::c_int = luaL_optinteger(
        L,
        2 as libc::c_int,
        1 as libc::c_int as Integer,
    ) as libc::c_int;
    if lua_type(L, 1 as libc::c_int) != 7 as libc::c_int {
        lua_pushnil(L);
    } else if lua_getiuservalue(L, 1 as libc::c_int, n) != -(1 as libc::c_int) {
        lua_pushboolean(L, 1 as libc::c_int);
        return 2 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn db_setuservalue(mut L: *mut lua_State) -> libc::c_int {
    let mut n: libc::c_int = luaL_optinteger(
        L,
        3 as libc::c_int,
        1 as libc::c_int as Integer,
    ) as libc::c_int;
    luaL_checktype(L, 1 as libc::c_int, 7 as libc::c_int);
    luaL_checkany(L, 2 as libc::c_int);
    lua_settop(L, 2 as libc::c_int);
    if lua_setiuservalue(L, 1 as libc::c_int, n) == 0 {
        lua_pushnil(L);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn getthread(
    mut L: *mut lua_State,
    mut arg: *mut libc::c_int,
) -> *mut lua_State {
    if lua_type(L, 1 as libc::c_int) == 8 as libc::c_int {
        *arg = 1 as libc::c_int;
        return lua_tothread(L, 1 as libc::c_int);
    } else {
        *arg = 0 as libc::c_int;
        return L;
    };
}
unsafe extern "C" fn settabss(
    mut L: *mut lua_State,
    mut k: *const libc::c_char,
    mut v: *const libc::c_char,
) {
    lua_pushstring(L, v);
    lua_setfield(L, -(2 as libc::c_int), k);
}
unsafe extern "C" fn settabsi(
    mut L: *mut lua_State,
    mut k: *const libc::c_char,
    mut v: libc::c_int,
) {
    lua_pushinteger(L, v as Integer);
    lua_setfield(L, -(2 as libc::c_int), k);
}
unsafe extern "C" fn settabsb(
    mut L: *mut lua_State,
    mut k: *const libc::c_char,
    mut v: libc::c_int,
) {
    lua_pushboolean(L, v);
    lua_setfield(L, -(2 as libc::c_int), k);
}
unsafe extern "C" fn treatstackoption(
    mut L: *mut lua_State,
    mut L1: *mut lua_State,
    mut fname: *const libc::c_char,
) {
    if L == L1 {
        lua_rotate(L, -(2 as libc::c_int), 1 as libc::c_int);
    } else {
        lua_xmove(L1, L, 1 as libc::c_int);
    }
    lua_setfield(L, -(2 as libc::c_int), fname);
}
unsafe extern "C" fn db_getinfo(mut L: *mut lua_State) -> libc::c_int {
    let mut ar: lua_Debug = lua_Debug {
        event: 0,
        name: 0 as *const libc::c_char,
        namewhat: 0 as *const libc::c_char,
        what: 0 as *const libc::c_char,
        source: 0 as *const libc::c_char,
        srclen: 0,
        currentline: 0,
        linedefined: 0,
        lastlinedefined: 0,
        nups: 0,
        nparams: 0,
        isvararg: 0,
        istailcall: 0,
        ftransfer: 0,
        ntransfer: 0,
        short_src: [0; 60],
        i_ci: 0 as *mut CallInfo,
    };
    let mut arg: libc::c_int = 0;
    let mut L1: *mut lua_State = getthread(L, &mut arg);
    let mut options: *const libc::c_char = luaL_optlstring(
        L,
        arg + 2 as libc::c_int,
        b"flnSrtu\0" as *const u8 as *const libc::c_char,
        0 as *mut size_t,
    );
    checkstack(L, L1, 3 as libc::c_int);
    (((*options.offset(0 as libc::c_int as isize) as libc::c_int != '>' as i32)
        as libc::c_int != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
        || luaL_argerror(
            L,
            arg + 2 as libc::c_int,
            b"invalid option '>'\0" as *const u8 as *const libc::c_char,
        ) != 0) as libc::c_int;
    if lua_type(L, arg + 1 as libc::c_int) == 6 as libc::c_int {
        options = lua_pushfstring(
            L,
            b">%s\0" as *const u8 as *const libc::c_char,
            options,
        );
        lua_pushvalue(L, arg + 1 as libc::c_int);
        lua_xmove(L, L1, 1 as libc::c_int);
    } else if lua_getstack(
        L1,
        luaL_checkinteger(L, arg + 1 as libc::c_int) as libc::c_int,
        &mut ar,
    ) == 0
    {
        lua_pushnil(L);
        return 1 as libc::c_int;
    }
    if lua_getinfo(L1, options, &mut ar) == 0 {
        return luaL_argerror(
            L,
            arg + 2 as libc::c_int,
            b"invalid option\0" as *const u8 as *const libc::c_char,
        );
    }
    lua_createtable(L, 0 as libc::c_int, 0 as libc::c_int);
    if !(strchr(options, 'S' as i32)).is_null() {
        lua_pushlstring(L, ar.source, ar.srclen);
        lua_setfield(
            L,
            -(2 as libc::c_int),
            b"source\0" as *const u8 as *const libc::c_char,
        );
        settabss(
            L,
            b"short_src\0" as *const u8 as *const libc::c_char,
            (ar.short_src).as_mut_ptr(),
        );
        settabsi(
            L,
            b"linedefined\0" as *const u8 as *const libc::c_char,
            ar.linedefined,
        );
        settabsi(
            L,
            b"lastlinedefined\0" as *const u8 as *const libc::c_char,
            ar.lastlinedefined,
        );
        settabss(L, b"what\0" as *const u8 as *const libc::c_char, ar.what);
    }
    if !(strchr(options, 'l' as i32)).is_null() {
        settabsi(
            L,
            b"currentline\0" as *const u8 as *const libc::c_char,
            ar.currentline,
        );
    }
    if !(strchr(options, 'u' as i32)).is_null() {
        settabsi(
            L,
            b"nups\0" as *const u8 as *const libc::c_char,
            ar.nups as libc::c_int,
        );
        settabsi(
            L,
            b"nparams\0" as *const u8 as *const libc::c_char,
            ar.nparams as libc::c_int,
        );
        settabsb(
            L,
            b"isvararg\0" as *const u8 as *const libc::c_char,
            ar.isvararg as libc::c_int,
        );
    }
    if !(strchr(options, 'n' as i32)).is_null() {
        settabss(L, b"name\0" as *const u8 as *const libc::c_char, ar.name);
        settabss(L, b"namewhat\0" as *const u8 as *const libc::c_char, ar.namewhat);
    }
    if !(strchr(options, 'r' as i32)).is_null() {
        settabsi(
            L,
            b"ftransfer\0" as *const u8 as *const libc::c_char,
            ar.ftransfer as libc::c_int,
        );
        settabsi(
            L,
            b"ntransfer\0" as *const u8 as *const libc::c_char,
            ar.ntransfer as libc::c_int,
        );
    }
    if !(strchr(options, 't' as i32)).is_null() {
        settabsb(
            L,
            b"istailcall\0" as *const u8 as *const libc::c_char,
            ar.istailcall as libc::c_int,
        );
    }
    if !(strchr(options, 'L' as i32)).is_null() {
        treatstackoption(L, L1, b"activelines\0" as *const u8 as *const libc::c_char);
    }
    if !(strchr(options, 'f' as i32)).is_null() {
        treatstackoption(L, L1, b"func\0" as *const u8 as *const libc::c_char);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn db_getlocal(mut L: *mut lua_State) -> libc::c_int {
    let mut arg: libc::c_int = 0;
    let mut L1: *mut lua_State = getthread(L, &mut arg);
    let mut nvar: libc::c_int = luaL_checkinteger(L, arg + 2 as libc::c_int)
        as libc::c_int;
    if lua_type(L, arg + 1 as libc::c_int) == 6 as libc::c_int {
        lua_pushvalue(L, arg + 1 as libc::c_int);
        lua_pushstring(L, lua_getlocal(L, 0 as *const lua_Debug, nvar));
        return 1 as libc::c_int;
    } else {
        let mut ar: lua_Debug = lua_Debug {
            event: 0,
            name: 0 as *const libc::c_char,
            namewhat: 0 as *const libc::c_char,
            what: 0 as *const libc::c_char,
            source: 0 as *const libc::c_char,
            srclen: 0,
            currentline: 0,
            linedefined: 0,
            lastlinedefined: 0,
            nups: 0,
            nparams: 0,
            isvararg: 0,
            istailcall: 0,
            ftransfer: 0,
            ntransfer: 0,
            short_src: [0; 60],
            i_ci: 0 as *mut CallInfo,
        };
        let mut name: *const libc::c_char = 0 as *const libc::c_char;
        let mut level: libc::c_int = luaL_checkinteger(L, arg + 1 as libc::c_int)
            as libc::c_int;
        if ((lua_getstack(L1, level, &mut ar) == 0) as libc::c_int != 0 as libc::c_int)
            as libc::c_int as libc::c_long != 0
        {
            return luaL_argerror(
                L,
                arg + 1 as libc::c_int,
                b"level out of range\0" as *const u8 as *const libc::c_char,
            );
        }
        checkstack(L, L1, 1 as libc::c_int);
        name = lua_getlocal(L1, &mut ar, nvar);
        if !name.is_null() {
            lua_xmove(L1, L, 1 as libc::c_int);
            lua_pushstring(L, name);
            lua_rotate(L, -(2 as libc::c_int), 1 as libc::c_int);
            return 2 as libc::c_int;
        } else {
            lua_pushnil(L);
            return 1 as libc::c_int;
        }
    };
}
unsafe extern "C" fn db_setlocal(mut L: *mut lua_State) -> libc::c_int {
    let mut arg: libc::c_int = 0;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut L1: *mut lua_State = getthread(L, &mut arg);
    let mut ar: lua_Debug = lua_Debug {
        event: 0,
        name: 0 as *const libc::c_char,
        namewhat: 0 as *const libc::c_char,
        what: 0 as *const libc::c_char,
        source: 0 as *const libc::c_char,
        srclen: 0,
        currentline: 0,
        linedefined: 0,
        lastlinedefined: 0,
        nups: 0,
        nparams: 0,
        isvararg: 0,
        istailcall: 0,
        ftransfer: 0,
        ntransfer: 0,
        short_src: [0; 60],
        i_ci: 0 as *mut CallInfo,
    };
    let mut level: libc::c_int = luaL_checkinteger(L, arg + 1 as libc::c_int)
        as libc::c_int;
    let mut nvar: libc::c_int = luaL_checkinteger(L, arg + 2 as libc::c_int)
        as libc::c_int;
    if ((lua_getstack(L1, level, &mut ar) == 0) as libc::c_int != 0 as libc::c_int)
        as libc::c_int as libc::c_long != 0
    {
        return luaL_argerror(
            L,
            arg + 1 as libc::c_int,
            b"level out of range\0" as *const u8 as *const libc::c_char,
        );
    }
    luaL_checkany(L, arg + 3 as libc::c_int);
    lua_settop(L, arg + 3 as libc::c_int);
    checkstack(L, L1, 1 as libc::c_int);
    lua_xmove(L, L1, 1 as libc::c_int);
    name = lua_setlocal(L1, &mut ar, nvar);
    if name.is_null() {
        lua_settop(L1, -(1 as libc::c_int) - 1 as libc::c_int);
    }
    lua_pushstring(L, name);
    return 1 as libc::c_int;
}
unsafe extern "C" fn auxupvalue(
    mut L: *mut lua_State,
    mut get: libc::c_int,
) -> libc::c_int {
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut n: libc::c_int = luaL_checkinteger(L, 2 as libc::c_int) as libc::c_int;
    luaL_checktype(L, 1 as libc::c_int, 6 as libc::c_int);
    name = if get != 0 {
        lua_getupvalue(L, 1 as libc::c_int, n)
    } else {
        lua_setupvalue(L, 1 as libc::c_int, n)
    };
    if name.is_null() {
        return 0 as libc::c_int;
    }
    lua_pushstring(L, name);
    lua_rotate(L, -(get + 1 as libc::c_int), 1 as libc::c_int);
    return get + 1 as libc::c_int;
}
unsafe extern "C" fn db_getupvalue(mut L: *mut lua_State) -> libc::c_int {
    return auxupvalue(L, 1 as libc::c_int);
}
unsafe extern "C" fn db_setupvalue(mut L: *mut lua_State) -> libc::c_int {
    luaL_checkany(L, 3 as libc::c_int);
    return auxupvalue(L, 0 as libc::c_int);
}
unsafe extern "C" fn checkupval(
    mut L: *mut lua_State,
    mut argf: libc::c_int,
    mut argnup: libc::c_int,
    mut pnup: *mut libc::c_int,
) -> *mut libc::c_void {
    let mut id: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut nup: libc::c_int = luaL_checkinteger(L, argnup) as libc::c_int;
    luaL_checktype(L, argf, 6 as libc::c_int);
    id = lua_upvalueid(L, argf, nup);
    if !pnup.is_null() {
        (((id != 0 as *mut libc::c_void) as libc::c_int != 0 as libc::c_int)
            as libc::c_int as libc::c_long != 0
            || luaL_argerror(
                L,
                argnup,
                b"invalid upvalue index\0" as *const u8 as *const libc::c_char,
            ) != 0) as libc::c_int;
        *pnup = nup;
    }
    return id;
}
unsafe extern "C" fn db_upvalueid(mut L: *mut lua_State) -> libc::c_int {
    let mut id: *mut libc::c_void = checkupval(
        L,
        1 as libc::c_int,
        2 as libc::c_int,
        0 as *mut libc::c_int,
    );
    if !id.is_null() {
        lua_pushlightuserdata(L, id);
    } else {
        lua_pushnil(L);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn db_upvaluejoin(mut L: *mut lua_State) -> libc::c_int {
    let mut n1: libc::c_int = 0;
    let mut n2: libc::c_int = 0;
    checkupval(L, 1 as libc::c_int, 2 as libc::c_int, &mut n1);
    checkupval(L, 3 as libc::c_int, 4 as libc::c_int, &mut n2);
    (((lua_iscfunction(L, 1 as libc::c_int) == 0) as libc::c_int != 0 as libc::c_int)
        as libc::c_int as libc::c_long != 0
        || luaL_argerror(
            L,
            1 as libc::c_int,
            b"Lua function expected\0" as *const u8 as *const libc::c_char,
        ) != 0) as libc::c_int;
    (((lua_iscfunction(L, 3 as libc::c_int) == 0) as libc::c_int != 0 as libc::c_int)
        as libc::c_int as libc::c_long != 0
        || luaL_argerror(
            L,
            3 as libc::c_int,
            b"Lua function expected\0" as *const u8 as *const libc::c_char,
        ) != 0) as libc::c_int;
    lua_upvaluejoin(L, 1 as libc::c_int, n1, 3 as libc::c_int, n2);
    return 0 as libc::c_int;
}
unsafe extern "C" fn hookf(mut L: *mut lua_State, mut ar: *mut lua_Debug) {
    static mut hooknames: [*const libc::c_char; 5] = [
        b"call\0" as *const u8 as *const libc::c_char,
        b"return\0" as *const u8 as *const libc::c_char,
        b"line\0" as *const u8 as *const libc::c_char,
        b"count\0" as *const u8 as *const libc::c_char,
        b"tail call\0" as *const u8 as *const libc::c_char,
    ];
    lua_getfield(L, -(1000000 as libc::c_int) - 1000 as libc::c_int, HOOKKEY);
    lua_pushthread(L);
    if lua_rawget(L, -(2 as libc::c_int)) == 6 as libc::c_int {
        lua_pushstring(L, hooknames[(*ar).event as usize]);
        if (*ar).currentline >= 0 as libc::c_int {
            lua_pushinteger(L, (*ar).currentline as Integer);
        } else {
            lua_pushnil(L);
        }
        lua_callk(
            L,
            2 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int as lua_KContext,
            None,
        );
    }
}
unsafe extern "C" fn makemask(
    mut smask: *const libc::c_char,
    mut count: libc::c_int,
) -> libc::c_int {
    let mut mask: libc::c_int = 0 as libc::c_int;
    if !(strchr(smask, 'c' as i32)).is_null() {
        mask |= (1 as libc::c_int) << 0 as libc::c_int;
    }
    if !(strchr(smask, 'r' as i32)).is_null() {
        mask |= (1 as libc::c_int) << 1 as libc::c_int;
    }
    if !(strchr(smask, 'l' as i32)).is_null() {
        mask |= (1 as libc::c_int) << 2 as libc::c_int;
    }
    if count > 0 as libc::c_int {
        mask |= (1 as libc::c_int) << 3 as libc::c_int;
    }
    return mask;
}
unsafe extern "C" fn unmakemask(
    mut mask: libc::c_int,
    mut smask: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut i: libc::c_int = 0 as libc::c_int;
    if mask & (1 as libc::c_int) << 0 as libc::c_int != 0 {
        let fresh0 = i;
        i = i + 1;
        *smask.offset(fresh0 as isize) = 'c' as i32 as libc::c_char;
    }
    if mask & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        let fresh1 = i;
        i = i + 1;
        *smask.offset(fresh1 as isize) = 'r' as i32 as libc::c_char;
    }
    if mask & (1 as libc::c_int) << 2 as libc::c_int != 0 {
        let fresh2 = i;
        i = i + 1;
        *smask.offset(fresh2 as isize) = 'l' as i32 as libc::c_char;
    }
    *smask.offset(i as isize) = '\0' as i32 as libc::c_char;
    return smask;
}
unsafe extern "C" fn db_sethook(mut L: *mut lua_State) -> libc::c_int {
    let mut arg: libc::c_int = 0;
    let mut mask: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut func: lua_Hook = None;
    let mut L1: *mut lua_State = getthread(L, &mut arg);
    if lua_type(L, arg + 1 as libc::c_int) <= 0 as libc::c_int {
        lua_settop(L, arg + 1 as libc::c_int);
        func = None;
        mask = 0 as libc::c_int;
        count = 0 as libc::c_int;
    } else {
        let mut smask: *const libc::c_char = luaL_checklstring(
            L,
            arg + 2 as libc::c_int,
            0 as *mut size_t,
        );
        luaL_checktype(L, arg + 1 as libc::c_int, 6 as libc::c_int);
        count = luaL_optinteger(
            L,
            arg + 3 as libc::c_int,
            0 as libc::c_int as Integer,
        ) as libc::c_int;
        func = Some(hookf as unsafe extern "C" fn(*mut lua_State, *mut lua_Debug) -> ());
        mask = makemask(smask, count);
    }
    if luaL_getsubtable(L, -(1000000 as libc::c_int) - 1000 as libc::c_int, HOOKKEY) == 0
    {
        lua_pushstring(L, b"k\0" as *const u8 as *const libc::c_char);
        lua_setfield(
            L,
            -(2 as libc::c_int),
            b"__mode\0" as *const u8 as *const libc::c_char,
        );
        lua_pushvalue(L, -(1 as libc::c_int));
        lua_setmetatable(L, -(2 as libc::c_int));
    }
    checkstack(L, L1, 1 as libc::c_int);
    lua_pushthread(L1);
    lua_xmove(L1, L, 1 as libc::c_int);
    lua_pushvalue(L, arg + 1 as libc::c_int);
    lua_rawset(L, -(3 as libc::c_int));
    lua_sethook(L1, func, mask, count);
    return 0 as libc::c_int;
}
unsafe extern "C" fn db_gethook(mut L: *mut lua_State) -> libc::c_int {
    let mut arg: libc::c_int = 0;
    let mut L1: *mut lua_State = getthread(L, &mut arg);
    let mut buff: [libc::c_char; 5] = [0; 5];
    let mut mask: libc::c_int = lua_gethookmask(L1);
    let mut hook: lua_Hook = lua_gethook(L1);
    if hook.is_none() {
        lua_pushnil(L);
        return 1 as libc::c_int;
    } else if hook
        != Some(hookf as unsafe extern "C" fn(*mut lua_State, *mut lua_Debug) -> ())
    {
        lua_pushstring(L, b"external hook\0" as *const u8 as *const libc::c_char);
    } else {
        lua_getfield(L, -(1000000 as libc::c_int) - 1000 as libc::c_int, HOOKKEY);
        checkstack(L, L1, 1 as libc::c_int);
        lua_pushthread(L1);
        lua_xmove(L1, L, 1 as libc::c_int);
        lua_rawget(L, -(2 as libc::c_int));
        lua_rotate(L, -(2 as libc::c_int), -(1 as libc::c_int));
        lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
    }
    lua_pushstring(L, unmakemask(mask, buff.as_mut_ptr()));
    lua_pushinteger(L, lua_gethookcount(L1) as Integer);
    return 3 as libc::c_int;
}
unsafe extern "C" fn db_debug(mut L: *mut lua_State) -> libc::c_int {
    loop {
        let mut buffer: [libc::c_char; 250] = [0; 250];
        fprintf(
            stderr,
            b"%s\0" as *const u8 as *const libc::c_char,
            b"lua_debug> \0" as *const u8 as *const libc::c_char,
        );
        fflush(stderr);
        if (fgets(
            buffer.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 250]>() as libc::c_ulong
                as libc::c_int,
            stdin,
        ))
            .is_null()
            || strcmp(
                buffer.as_mut_ptr(),
                b"cont\n\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
        {
            return 0 as libc::c_int;
        }
        if luaL_loadbufferx(
            L,
            buffer.as_mut_ptr(),
            strlen(buffer.as_mut_ptr()),
            b"=(debug command)\0" as *const u8 as *const libc::c_char,
            0 as *const libc::c_char,
        ) != 0
            || lua_pcallk(
                L,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int as lua_KContext,
                None,
            ) != 0
        {
            fprintf(
                stderr,
                b"%s\n\0" as *const u8 as *const libc::c_char,
                luaL_tolstring(L, -(1 as libc::c_int), 0 as *mut size_t),
            );
            fflush(stderr);
        }
        lua_settop(L, 0 as libc::c_int);
    };
}
unsafe extern "C" fn db_traceback(mut L: *mut lua_State) -> libc::c_int {
    let mut arg: libc::c_int = 0;
    let mut L1: *mut lua_State = getthread(L, &mut arg);
    let mut msg: *const libc::c_char = lua_tolstring(
        L,
        arg + 1 as libc::c_int,
        0 as *mut size_t,
    );
    if msg.is_null() && !(lua_type(L, arg + 1 as libc::c_int) <= 0 as libc::c_int) {
        lua_pushvalue(L, arg + 1 as libc::c_int);
    } else {
        let mut level: libc::c_int = luaL_optinteger(
            L,
            arg + 2 as libc::c_int,
            (if L == L1 { 1 as libc::c_int } else { 0 as libc::c_int }) as Integer,
        ) as libc::c_int;
        luaL_traceback(L, L1, msg, level);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn db_setcstacklimit(mut L: *mut lua_State) -> libc::c_int {
    let mut limit: libc::c_int = luaL_checkinteger(L, 1 as libc::c_int) as libc::c_int;
    let mut res: libc::c_int = lua_setcstacklimit(L, limit as libc::c_uint);
    lua_pushinteger(L, res as Integer);
    return 1 as libc::c_int;
}
static mut dblib: [luaL_Reg; 18] = unsafe {
    [
        {
            let mut init = luaL_Reg {
                name: b"debug\0" as *const u8 as *const libc::c_char,
                func: Some(
                    db_debug as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"getuservalue\0" as *const u8 as *const libc::c_char,
                func: Some(
                    db_getuservalue
                        as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"gethook\0" as *const u8 as *const libc::c_char,
                func: Some(
                    db_gethook as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"getinfo\0" as *const u8 as *const libc::c_char,
                func: Some(
                    db_getinfo as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"getlocal\0" as *const u8 as *const libc::c_char,
                func: Some(
                    db_getlocal as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"getregistry\0" as *const u8 as *const libc::c_char,
                func: Some(
                    db_getregistry as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"getmetatable\0" as *const u8 as *const libc::c_char,
                func: Some(
                    db_getmetatable
                        as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"getupvalue\0" as *const u8 as *const libc::c_char,
                func: Some(
                    db_getupvalue as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"upvaluejoin\0" as *const u8 as *const libc::c_char,
                func: Some(
                    db_upvaluejoin as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"upvalueid\0" as *const u8 as *const libc::c_char,
                func: Some(
                    db_upvalueid as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"setuservalue\0" as *const u8 as *const libc::c_char,
                func: Some(
                    db_setuservalue
                        as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"sethook\0" as *const u8 as *const libc::c_char,
                func: Some(
                    db_sethook as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"setlocal\0" as *const u8 as *const libc::c_char,
                func: Some(
                    db_setlocal as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"setmetatable\0" as *const u8 as *const libc::c_char,
                func: Some(
                    db_setmetatable
                        as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"setupvalue\0" as *const u8 as *const libc::c_char,
                func: Some(
                    db_setupvalue as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"traceback\0" as *const u8 as *const libc::c_char,
                func: Some(
                    db_traceback as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"setcstacklimit\0" as *const u8 as *const libc::c_char,
                func: Some(
                    db_setcstacklimit
                        as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
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
pub unsafe extern "C" fn luaopen_debug(mut L: *mut lua_State) -> libc::c_int {
    luaL_checkversion_(
        L,
        504 as libc::c_int as Number,
        (::core::mem::size_of::<Integer>() as libc::c_ulong)
            .wrapping_mul(16 as libc::c_int as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<Number>() as libc::c_ulong),
    );
    lua_createtable(
        L,
        0 as libc::c_int,
        (::core::mem::size_of::<[luaL_Reg; 18]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<luaL_Reg>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
    );
    luaL_setfuncs(L, dblib.as_ptr(), 0 as libc::c_int);
    return 1 as libc::c_int;
}
