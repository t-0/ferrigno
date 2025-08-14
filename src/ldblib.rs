#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use crate::types::*;
unsafe extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type lua_State;
    pub type CallInfo;
    static mut stdin: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> i32;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> i32;
    fn fgets(__s: *mut libc::c_char, __n: i32, __stream: *mut FILE) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> i32;
    fn strchr(_: *const libc::c_char, _: i32) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn lua_settop(L: *mut lua_State, index: i32);
    fn lua_pushvalue(L: *mut lua_State, index: i32);
    fn lua_rotate(L: *mut lua_State, index: i32, n: i32);
    fn lua_checkstack(L: *mut lua_State, n: i32) -> i32;
    fn lua_xmove(from: *mut lua_State, to: *mut lua_State, n: i32);
    fn lua_iscfunction(L: *mut lua_State, index: i32) -> i32;
    fn lua_type(L: *mut lua_State, index: i32) -> i32;
    fn lua_tolstring(L: *mut lua_State, index: i32, len: *mut u64) -> *const libc::c_char;
    fn lua_tothread(L: *mut lua_State, index: i32) -> *mut lua_State;
    fn lua_pushnil(L: *mut lua_State);
    fn lua_pushinteger(L: *mut lua_State, n: i64);
    fn lua_pushlstring(L: *mut lua_State, s: *const libc::c_char, len: u64) -> *const libc::c_char;
    fn lua_pushstring(L: *mut lua_State, s: *const libc::c_char) -> *const libc::c_char;
    fn lua_pushfstring(L: *mut lua_State, fmt: *const libc::c_char, _: ...) -> *const libc::c_char;
    fn lua_pushboolean(L: *mut lua_State, b: i32);
    fn lua_pushlightuserdata(L: *mut lua_State, p: *mut libc::c_void);
    fn lua_pushthread(L: *mut lua_State) -> i32;
    fn lua_getfield(L: *mut lua_State, index: i32, k: *const libc::c_char) -> i32;
    fn lua_rawget(L: *mut lua_State, index: i32) -> i32;
    fn lua_createtable(L: *mut lua_State, narr: i32, nrec: i32);
    fn lua_getmetatable(L: *mut lua_State, objindex: i32) -> i32;
    fn lua_getiuservalue(L: *mut lua_State, index: i32, n: i32) -> i32;
    fn lua_setfield(L: *mut lua_State, index: i32, k: *const libc::c_char);
    fn lua_rawset(L: *mut lua_State, index: i32);
    fn lua_setmetatable(L: *mut lua_State, objindex: i32) -> i32;
    fn lua_setiuservalue(L: *mut lua_State, index: i32, n: i32) -> i32;
    fn lua_callk(L: *mut lua_State, nargs: i32, nresults: i32, ctx: lua_KContext, k: lua_KFunction);
    fn lua_pcallk(
        L: *mut lua_State,
        nargs: i32,
        nresults: i32,
        errfunc: i32,
        ctx: lua_KContext,
        k: lua_KFunction,
    ) -> i32;
    fn lua_getstack(L: *mut lua_State, level: i32, ar: *mut lua_Debug) -> i32;
    fn lua_getinfo(L: *mut lua_State, what: *const libc::c_char, ar: *mut lua_Debug) -> i32;
    fn lua_getlocal(L: *mut lua_State, ar: *const lua_Debug, n: i32) -> *const libc::c_char;
    fn lua_setlocal(L: *mut lua_State, ar: *const lua_Debug, n: i32) -> *const libc::c_char;
    fn lua_getupvalue(L: *mut lua_State, funcindex: i32, n: i32) -> *const libc::c_char;
    fn lua_setupvalue(L: *mut lua_State, funcindex: i32, n: i32) -> *const libc::c_char;
    fn lua_upvalueid(L: *mut lua_State, fidx: i32, n: i32) -> *mut libc::c_void;
    fn lua_upvaluejoin(L: *mut lua_State, fidx1: i32, n1: i32, fidx2: i32, n2: i32);
    fn lua_sethook(L: *mut lua_State, func: lua_Hook, mask: i32, count: i32);
    fn lua_gethook(L: *mut lua_State) -> lua_Hook;
    fn lua_gethookmask(L: *mut lua_State) -> i32;
    fn lua_gethookcount(L: *mut lua_State) -> i32;
    fn lua_setcstacklimit(L: *mut lua_State, limit: u32) -> i32;
    fn luaL_checkversion_(L: *mut lua_State, ver: f64, sz: u64);
    fn luaL_tolstring(L: *mut lua_State, index: i32, len: *mut u64) -> *const libc::c_char;
    fn luaL_argerror(L: *mut lua_State, arg: i32, extramsg: *const libc::c_char) -> i32;
    fn luaL_typeerror(L: *mut lua_State, arg: i32, tname: *const libc::c_char) -> i32;
    fn luaL_checklstring(L: *mut lua_State, arg: i32, l: *mut u64) -> *const libc::c_char;
    fn luaL_optlstring(
        L: *mut lua_State,
        arg: i32,
        def: *const libc::c_char,
        l: *mut u64,
    ) -> *const libc::c_char;
    fn luaL_checkinteger(L: *mut lua_State, arg: i32) -> i64;
    fn luaL_optinteger(L: *mut lua_State, arg: i32, def: i64) -> i64;
    fn luaL_checktype(L: *mut lua_State, arg: i32, t: i32);
    fn luaL_checkany(L: *mut lua_State, arg: i32);
    fn luaL_error(L: *mut lua_State, fmt: *const libc::c_char, _: ...) -> i32;
    fn luaL_loadbufferx(
        L: *mut lua_State,
        buff: *const libc::c_char,
        sz: u64,
        name: *const libc::c_char,
        mode: *const libc::c_char,
    ) -> i32;
    fn luaL_setfuncs(L: *mut lua_State, l: *const luaL_Reg, nup: i32);
    fn luaL_getsubtable(L: *mut lua_State, index: i32, fname: *const libc::c_char) -> i32;
    fn luaL_traceback(L: *mut lua_State, L1: *mut lua_State, msg: *const libc::c_char, level: i32);
}
pub type __off_t = i64;
pub type __off64_t = i64;
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
    pub __pad5: u64,
    pub _mode: i32,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;

pub type CFunction = Option<unsafe extern "C" fn(*mut lua_State) -> i32>;
pub type lua_KFunction = Option<unsafe extern "C" fn(*mut lua_State, i32, lua_KContext) -> i32>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lua_Debug {
    pub event: i32,
    pub name: *const libc::c_char,
    pub namewhat: *const libc::c_char,
    pub what: *const libc::c_char,
    pub source: *const libc::c_char,
    pub srclen: u64,
    pub currentline: i32,
    pub linedefined: i32,
    pub lastlinedefined: i32,
    pub nups: u8,
    pub nparams: u8,
    pub isvararg: libc::c_char,
    pub istailcall: libc::c_char,
    pub ftransfer: libc::c_ushort,
    pub ntransfer: libc::c_ushort,
    pub short_src: [libc::c_char; 60],
    pub i_ci: *mut CallInfo,
}
pub type lua_Hook = Option<unsafe extern "C" fn(*mut lua_State, *mut lua_Debug) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaL_Reg {
    pub name: *const libc::c_char,
    pub func: CFunction,
}
static mut HOOKKEY: *const libc::c_char = b"_HOOKKEY\0" as *const u8 as *const libc::c_char;
unsafe extern "C" fn checkstack(mut L: *mut lua_State, mut L1: *mut lua_State, mut n: i32) {
    if ((L != L1 && lua_checkstack(L1, n) == 0) as i32 != 0i32) as i32 as i64 != 0 {
        luaL_error(L, b"stack overflow\0" as *const u8 as *const libc::c_char);
    }
}
unsafe extern "C" fn db_getregistry(mut L: *mut lua_State) -> i32 {
    lua_pushvalue(L, -(1000000i32) - 1000i32);
    return 1i32;
}
unsafe extern "C" fn db_getmetatable(mut L: *mut lua_State) -> i32 {
    luaL_checkany(L, 1i32);
    if lua_getmetatable(L, 1i32) == 0 {
        lua_pushnil(L);
    }
    return 1i32;
}
unsafe extern "C" fn db_setmetatable(mut L: *mut lua_State) -> i32 {
    let mut t: i32 = lua_type(L, 2i32);
    (((t == 0i32 || t == 5i32) as i32 != 0i32) as i32 as i64 != 0
        || luaL_typeerror(
            L,
            2i32,
            b"nil or table\0" as *const u8 as *const libc::c_char,
        ) != 0) as i32;
    lua_settop(L, 2i32);
    lua_setmetatable(L, 1i32);
    return 1i32;
}
unsafe extern "C" fn db_getuservalue(mut L: *mut lua_State) -> i32 {
    let mut n: i32 = luaL_optinteger(L, 2i32, 1i32 as i64) as i32;
    if lua_type(L, 1i32) != 7i32 {
        lua_pushnil(L);
    } else if lua_getiuservalue(L, 1i32, n) != -(1i32) {
        lua_pushboolean(L, 1i32);
        return 2i32;
    }
    return 1i32;
}
unsafe extern "C" fn db_setuservalue(mut L: *mut lua_State) -> i32 {
    let mut n: i32 = luaL_optinteger(L, 3i32, 1i32 as i64) as i32;
    luaL_checktype(L, 1i32, 7i32);
    luaL_checkany(L, 2i32);
    lua_settop(L, 2i32);
    if lua_setiuservalue(L, 1i32, n) == 0 {
        lua_pushnil(L);
    }
    return 1i32;
}
unsafe extern "C" fn getthread(mut L: *mut lua_State, mut arg: *mut i32) -> *mut lua_State {
    if lua_type(L, 1i32) == 8i32 {
        *arg = 1i32;
        return lua_tothread(L, 1i32);
    } else {
        *arg = 0i32;
        return L;
    };
}
unsafe extern "C" fn settabss(
    mut L: *mut lua_State,
    mut k: *const libc::c_char,
    mut v: *const libc::c_char,
) {
    lua_pushstring(L, v);
    lua_setfield(L, -(2i32), k);
}
unsafe extern "C" fn settabsi(mut L: *mut lua_State, mut k: *const libc::c_char, mut v: i32) {
    lua_pushinteger(L, v as i64);
    lua_setfield(L, -(2i32), k);
}
unsafe extern "C" fn settabsb(mut L: *mut lua_State, mut k: *const libc::c_char, mut v: i32) {
    lua_pushboolean(L, v);
    lua_setfield(L, -(2i32), k);
}
unsafe extern "C" fn treatstackoption(
    mut L: *mut lua_State,
    mut L1: *mut lua_State,
    mut fname: *const libc::c_char,
) {
    if L == L1 {
        lua_rotate(L, -(2i32), 1i32);
    } else {
        lua_xmove(L1, L, 1i32);
    }
    lua_setfield(L, -(2i32), fname);
}
unsafe extern "C" fn db_getinfo(mut L: *mut lua_State) -> i32 {
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
    let mut arg: i32 = 0;
    let mut L1: *mut lua_State = getthread(L, &mut arg);
    let mut options: *const libc::c_char = luaL_optlstring(
        L,
        arg + 2i32,
        b"flnSrtu\0" as *const u8 as *const libc::c_char,
        0 as *mut u64,
    );
    checkstack(L, L1, 3i32);
    (((*options.offset(0i32 as isize) as i32 != '>' as i32) as i32 != 0i32) as i32 as i64 != 0
        || luaL_argerror(
            L,
            arg + 2i32,
            b"invalid option '>'\0" as *const u8 as *const libc::c_char,
        ) != 0) as i32;
    if lua_type(L, arg + 1i32) == 6i32 {
        options = lua_pushfstring(L, b">%s\0" as *const u8 as *const libc::c_char, options);
        lua_pushvalue(L, arg + 1i32);
        lua_xmove(L, L1, 1i32);
    } else if lua_getstack(L1, luaL_checkinteger(L, arg + 1i32) as i32, &mut ar) == 0 {
        lua_pushnil(L);
        return 1i32;
    }
    if lua_getinfo(L1, options, &mut ar) == 0 {
        return luaL_argerror(
            L,
            arg + 2i32,
            b"invalid option\0" as *const u8 as *const libc::c_char,
        );
    }
    lua_createtable(L, 0i32, 0i32);
    if !(strchr(options, 'S' as i32)).is_null() {
        lua_pushlstring(L, ar.source, ar.srclen);
        lua_setfield(L, -(2i32), b"source\0" as *const u8 as *const libc::c_char);
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
            ar.nups as i32,
        );
        settabsi(
            L,
            b"nparams\0" as *const u8 as *const libc::c_char,
            ar.nparams as i32,
        );
        settabsb(
            L,
            b"isvararg\0" as *const u8 as *const libc::c_char,
            ar.isvararg as i32,
        );
    }
    if !(strchr(options, 'n' as i32)).is_null() {
        settabss(L, b"name\0" as *const u8 as *const libc::c_char, ar.name);
        settabss(
            L,
            b"namewhat\0" as *const u8 as *const libc::c_char,
            ar.namewhat,
        );
    }
    if !(strchr(options, 'r' as i32)).is_null() {
        settabsi(
            L,
            b"ftransfer\0" as *const u8 as *const libc::c_char,
            ar.ftransfer as i32,
        );
        settabsi(
            L,
            b"ntransfer\0" as *const u8 as *const libc::c_char,
            ar.ntransfer as i32,
        );
    }
    if !(strchr(options, 't' as i32)).is_null() {
        settabsb(
            L,
            b"istailcall\0" as *const u8 as *const libc::c_char,
            ar.istailcall as i32,
        );
    }
    if !(strchr(options, 'L' as i32)).is_null() {
        treatstackoption(L, L1, b"activelines\0" as *const u8 as *const libc::c_char);
    }
    if !(strchr(options, 'f' as i32)).is_null() {
        treatstackoption(L, L1, b"func\0" as *const u8 as *const libc::c_char);
    }
    return 1i32;
}
unsafe extern "C" fn db_getlocal(mut L: *mut lua_State) -> i32 {
    let mut arg: i32 = 0;
    let mut L1: *mut lua_State = getthread(L, &mut arg);
    let mut nvar: i32 = luaL_checkinteger(L, arg + 2i32) as i32;
    if lua_type(L, arg + 1i32) == 6i32 {
        lua_pushvalue(L, arg + 1i32);
        lua_pushstring(L, lua_getlocal(L, 0 as *const lua_Debug, nvar));
        return 1i32;
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
        let mut level: i32 = luaL_checkinteger(L, arg + 1i32) as i32;
        if ((lua_getstack(L1, level, &mut ar) == 0) as i32 != 0i32) as i32 as i64 != 0 {
            return luaL_argerror(
                L,
                arg + 1i32,
                b"level out of range\0" as *const u8 as *const libc::c_char,
            );
        }
        checkstack(L, L1, 1i32);
        name = lua_getlocal(L1, &mut ar, nvar);
        if !name.is_null() {
            lua_xmove(L1, L, 1i32);
            lua_pushstring(L, name);
            lua_rotate(L, -(2i32), 1i32);
            return 2i32;
        } else {
            lua_pushnil(L);
            return 1i32;
        }
    };
}
unsafe extern "C" fn db_setlocal(mut L: *mut lua_State) -> i32 {
    let mut arg: i32 = 0;
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
    let mut level: i32 = luaL_checkinteger(L, arg + 1i32) as i32;
    let mut nvar: i32 = luaL_checkinteger(L, arg + 2i32) as i32;
    if ((lua_getstack(L1, level, &mut ar) == 0) as i32 != 0i32) as i32 as i64 != 0 {
        return luaL_argerror(
            L,
            arg + 1i32,
            b"level out of range\0" as *const u8 as *const libc::c_char,
        );
    }
    luaL_checkany(L, arg + 3i32);
    lua_settop(L, arg + 3i32);
    checkstack(L, L1, 1i32);
    lua_xmove(L, L1, 1i32);
    name = lua_setlocal(L1, &mut ar, nvar);
    if name.is_null() {
        lua_settop(L1, -(1i32) - 1i32);
    }
    lua_pushstring(L, name);
    return 1i32;
}
unsafe extern "C" fn auxupvalue(mut L: *mut lua_State, mut get: i32) -> i32 {
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut n: i32 = luaL_checkinteger(L, 2i32) as i32;
    luaL_checktype(L, 1i32, 6i32);
    name = if get != 0 {
        lua_getupvalue(L, 1i32, n)
    } else {
        lua_setupvalue(L, 1i32, n)
    };
    if name.is_null() {
        return 0i32;
    }
    lua_pushstring(L, name);
    lua_rotate(L, -(get + 1i32), 1i32);
    return get + 1i32;
}
unsafe extern "C" fn db_getupvalue(mut L: *mut lua_State) -> i32 {
    return auxupvalue(L, 1i32);
}
unsafe extern "C" fn db_setupvalue(mut L: *mut lua_State) -> i32 {
    luaL_checkany(L, 3i32);
    return auxupvalue(L, 0i32);
}
unsafe extern "C" fn checkupval(
    mut L: *mut lua_State,
    mut argf: i32,
    mut argnup: i32,
    mut pnup: *mut i32,
) -> *mut libc::c_void {
    let mut id: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut nup: i32 = luaL_checkinteger(L, argnup) as i32;
    luaL_checktype(L, argf, 6i32);
    id = lua_upvalueid(L, argf, nup);
    if !pnup.is_null() {
        (((id != 0 as *mut libc::c_void) as i32 != 0i32) as i32 as i64 != 0
            || luaL_argerror(
                L,
                argnup,
                b"invalid upvalue index\0" as *const u8 as *const libc::c_char,
            ) != 0) as i32;
        *pnup = nup;
    }
    return id;
}
unsafe extern "C" fn db_upvalueid(mut L: *mut lua_State) -> i32 {
    let mut id: *mut libc::c_void = checkupval(L, 1i32, 2i32, 0 as *mut i32);
    if !id.is_null() {
        lua_pushlightuserdata(L, id);
    } else {
        lua_pushnil(L);
    }
    return 1i32;
}
unsafe extern "C" fn db_upvaluejoin(mut L: *mut lua_State) -> i32 {
    let mut n1: i32 = 0;
    let mut n2: i32 = 0;
    checkupval(L, 1i32, 2i32, &mut n1);
    checkupval(L, 3i32, 4i32, &mut n2);
    (((lua_iscfunction(L, 1i32) == 0) as i32 != 0i32) as i32 as i64 != 0
        || luaL_argerror(
            L,
            1i32,
            b"Lua function expected\0" as *const u8 as *const libc::c_char,
        ) != 0) as i32;
    (((lua_iscfunction(L, 3i32) == 0) as i32 != 0i32) as i32 as i64 != 0
        || luaL_argerror(
            L,
            3i32,
            b"Lua function expected\0" as *const u8 as *const libc::c_char,
        ) != 0) as i32;
    lua_upvaluejoin(L, 1i32, n1, 3i32, n2);
    return 0i32;
}
unsafe extern "C" fn hookf(mut L: *mut lua_State, mut ar: *mut lua_Debug) {
    static mut hooknames: [*const libc::c_char; 5] = [
        b"call\0" as *const u8 as *const libc::c_char,
        b"return\0" as *const u8 as *const libc::c_char,
        b"line\0" as *const u8 as *const libc::c_char,
        b"count\0" as *const u8 as *const libc::c_char,
        b"tail call\0" as *const u8 as *const libc::c_char,
    ];
    lua_getfield(L, -(1000000i32) - 1000i32, HOOKKEY);
    lua_pushthread(L);
    if lua_rawget(L, -(2i32)) == 6i32 {
        lua_pushstring(L, hooknames[(*ar).event as usize]);
        if (*ar).currentline >= 0i32 {
            lua_pushinteger(L, (*ar).currentline as i64);
        } else {
            lua_pushnil(L);
        }
        lua_callk(L, 2i32, 0i32, 0i32 as lua_KContext, None);
    }
}
unsafe extern "C" fn makemask(mut smask: *const libc::c_char, mut count: i32) -> i32 {
    let mut mask: i32 = 0i32;
    if !(strchr(smask, 'c' as i32)).is_null() {
        mask |= (1i32) << 0i32;
    }
    if !(strchr(smask, 'r' as i32)).is_null() {
        mask |= (1i32) << 1i32;
    }
    if !(strchr(smask, 'l' as i32)).is_null() {
        mask |= (1i32) << 2i32;
    }
    if count > 0i32 {
        mask |= (1i32) << 3i32;
    }
    return mask;
}
unsafe extern "C" fn unmakemask(mut mask: i32, mut smask: *mut libc::c_char) -> *mut libc::c_char {
    let mut i: i32 = 0i32;
    if mask & (1i32) << 0i32 != 0 {
        let fresh0 = i;
        i = i + 1;
        *smask.offset(fresh0 as isize) = 'c' as i32 as libc::c_char;
    }
    if mask & (1i32) << 1i32 != 0 {
        let fresh1 = i;
        i = i + 1;
        *smask.offset(fresh1 as isize) = 'r' as i32 as libc::c_char;
    }
    if mask & (1i32) << 2i32 != 0 {
        let fresh2 = i;
        i = i + 1;
        *smask.offset(fresh2 as isize) = 'l' as i32 as libc::c_char;
    }
    *smask.offset(i as isize) = '\0' as i32 as libc::c_char;
    return smask;
}
unsafe extern "C" fn db_sethook(mut L: *mut lua_State) -> i32 {
    let mut arg: i32 = 0;
    let mut mask: i32 = 0;
    let mut count: i32 = 0;
    let mut func: lua_Hook = None;
    let mut L1: *mut lua_State = getthread(L, &mut arg);
    if lua_type(L, arg + 1i32) <= 0i32 {
        lua_settop(L, arg + 1i32);
        func = None;
        mask = 0i32;
        count = 0i32;
    } else {
        let mut smask: *const libc::c_char = luaL_checklstring(L, arg + 2i32, 0 as *mut u64);
        luaL_checktype(L, arg + 1i32, 6i32);
        count = luaL_optinteger(L, arg + 3i32, 0i32 as i64) as i32;
        func = Some(hookf as unsafe extern "C" fn(*mut lua_State, *mut lua_Debug) -> ());
        mask = makemask(smask, count);
    }
    if luaL_getsubtable(L, -(1000000i32) - 1000i32, HOOKKEY) == 0 {
        lua_pushstring(L, b"k\0" as *const u8 as *const libc::c_char);
        lua_setfield(L, -(2i32), b"__mode\0" as *const u8 as *const libc::c_char);
        lua_pushvalue(L, -(1i32));
        lua_setmetatable(L, -(2i32));
    }
    checkstack(L, L1, 1i32);
    lua_pushthread(L1);
    lua_xmove(L1, L, 1i32);
    lua_pushvalue(L, arg + 1i32);
    lua_rawset(L, -(3i32));
    lua_sethook(L1, func, mask, count);
    return 0i32;
}
unsafe extern "C" fn db_gethook(mut L: *mut lua_State) -> i32 {
    let mut arg: i32 = 0;
    let mut L1: *mut lua_State = getthread(L, &mut arg);
    let mut buff: [libc::c_char; 5] = [0; 5];
    let mut mask: i32 = lua_gethookmask(L1);
    let mut hook: lua_Hook = lua_gethook(L1);
    if hook.is_none() {
        lua_pushnil(L);
        return 1i32;
    } else if hook != Some(hookf as unsafe extern "C" fn(*mut lua_State, *mut lua_Debug) -> ()) {
        lua_pushstring(L, b"external hook\0" as *const u8 as *const libc::c_char);
    } else {
        lua_getfield(L, -(1000000i32) - 1000i32, HOOKKEY);
        checkstack(L, L1, 1i32);
        lua_pushthread(L1);
        lua_xmove(L1, L, 1i32);
        lua_rawget(L, -(2i32));
        lua_rotate(L, -(2i32), -(1i32));
        lua_settop(L, -(1i32) - 1i32);
    }
    lua_pushstring(L, unmakemask(mask, buff.as_mut_ptr()));
    lua_pushinteger(L, lua_gethookcount(L1) as i64);
    return 3i32;
}
unsafe extern "C" fn db_debug(mut L: *mut lua_State) -> i32 {
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
            ::core::mem::size_of::<[libc::c_char; 250]>() as libc::c_ulong as i32,
            stdin,
        ))
        .is_null()
            || strcmp(
                buffer.as_mut_ptr(),
                b"cont\n\0" as *const u8 as *const libc::c_char,
            ) == 0i32
        {
            return 0i32;
        }
        if luaL_loadbufferx(
            L,
            buffer.as_mut_ptr(),
            strlen(buffer.as_mut_ptr()),
            b"=(debug command)\0" as *const u8 as *const libc::c_char,
            0 as *const libc::c_char,
        ) != 0
            || lua_pcallk(L, 0i32, 0i32, 0i32, 0i32 as lua_KContext, None) != 0
        {
            fprintf(
                stderr,
                b"%s\n\0" as *const u8 as *const libc::c_char,
                luaL_tolstring(L, -(1i32), 0 as *mut u64),
            );
            fflush(stderr);
        }
        lua_settop(L, 0i32);
    }
}
unsafe extern "C" fn db_traceback(mut L: *mut lua_State) -> i32 {
    let mut arg: i32 = 0;
    let mut L1: *mut lua_State = getthread(L, &mut arg);
    let mut msg: *const libc::c_char = lua_tolstring(L, arg + 1i32, 0 as *mut u64);
    if msg.is_null() && !(lua_type(L, arg + 1i32) <= 0i32) {
        lua_pushvalue(L, arg + 1i32);
    } else {
        let mut level: i32 =
            luaL_optinteger(L, arg + 2i32, (if L == L1 { 1i32 } else { 0i32 }) as i64) as i32;
        luaL_traceback(L, L1, msg, level);
    }
    return 1i32;
}
unsafe extern "C" fn db_setcstacklimit(mut L: *mut lua_State) -> i32 {
    let mut limit: i32 = luaL_checkinteger(L, 1i32) as i32;
    let mut res: i32 = lua_setcstacklimit(L, limit as u32);
    lua_pushinteger(L, res as i64);
    return 1i32;
}
static mut dblib: [luaL_Reg; 18] = {
    [
        {
            let mut init = luaL_Reg {
                name: b"debug\0" as *const u8 as *const libc::c_char,
                func: Some(db_debug as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"getuservalue\0" as *const u8 as *const libc::c_char,
                func: Some(db_getuservalue as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"gethook\0" as *const u8 as *const libc::c_char,
                func: Some(db_gethook as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"getinfo\0" as *const u8 as *const libc::c_char,
                func: Some(db_getinfo as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"getlocal\0" as *const u8 as *const libc::c_char,
                func: Some(db_getlocal as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"getregistry\0" as *const u8 as *const libc::c_char,
                func: Some(db_getregistry as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"getmetatable\0" as *const u8 as *const libc::c_char,
                func: Some(db_getmetatable as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"getupvalue\0" as *const u8 as *const libc::c_char,
                func: Some(db_getupvalue as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"upvaluejoin\0" as *const u8 as *const libc::c_char,
                func: Some(db_upvaluejoin as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"upvalueid\0" as *const u8 as *const libc::c_char,
                func: Some(db_upvalueid as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"setuservalue\0" as *const u8 as *const libc::c_char,
                func: Some(db_setuservalue as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"sethook\0" as *const u8 as *const libc::c_char,
                func: Some(db_sethook as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"setlocal\0" as *const u8 as *const libc::c_char,
                func: Some(db_setlocal as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"setmetatable\0" as *const u8 as *const libc::c_char,
                func: Some(db_setmetatable as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"setupvalue\0" as *const u8 as *const libc::c_char,
                func: Some(db_setupvalue as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"traceback\0" as *const u8 as *const libc::c_char,
                func: Some(db_traceback as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"setcstacklimit\0" as *const u8 as *const libc::c_char,
                func: Some(db_setcstacklimit as unsafe extern "C" fn(*mut lua_State) -> i32),
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaopen_debug(mut L: *mut lua_State) -> i32 {
    luaL_checkversion_(
        L,
        504i32 as f64,
        (::core::mem::size_of::<i64>() as libc::c_ulong)
            .wrapping_mul(16i32 as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<f64>() as libc::c_ulong),
    );
    lua_createtable(
        L,
        0i32,
        (::core::mem::size_of::<[luaL_Reg; 18]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<luaL_Reg>() as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong) as i32,
    );
    luaL_setfuncs(L, dblib.as_ptr(), 0i32);
    return 1i32;
}
