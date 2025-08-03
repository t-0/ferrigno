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
    fn __errno_location() -> *mut i32;
    static mut stdin: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fflush(__stream: *mut FILE) -> i32;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn freopen(
        __filename: *const libc::c_char,
        __modes: *const libc::c_char,
        __stream: *mut FILE,
    ) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> i32;
    fn getc(__stream: *mut FILE) -> i32;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn feof(__stream: *mut FILE) -> i32;
    fn ferror(__stream: *mut FILE) -> i32;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> i32;
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> i32;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: i32) -> *mut libc::c_char;
    fn lua_newstate(f: lua_Alloc, ud: *mut libc::c_void) -> *mut lua_State;
    fn lua_atpanic(L: *mut lua_State, panicf: CFunction) -> CFunction;
    fn lua_version(L: *mut lua_State) -> f64;
    fn lua_absindex(L: *mut lua_State, index: i32) -> i32;
    fn lua_gettop(L: *mut lua_State) -> i32;
    fn lua_settop(L: *mut lua_State, index: i32);
    fn lua_pushvalue(L: *mut lua_State, index: i32);
    fn lua_rotate(L: *mut lua_State, index: i32, n: i32);
    fn lua_copy(L: *mut lua_State, fromidx: i32, toidx: i32);
    fn lua_checkstack(L: *mut lua_State, n: i32) -> i32;
    fn lua_isnumber(L: *mut lua_State, index: i32) -> i32;
    fn lua_isstring(L: *mut lua_State, index: i32) -> i32;
    fn lua_isinteger(L: *mut lua_State, index: i32) -> i32;
    fn lua_type(L: *mut lua_State, index: i32) -> i32;
    fn lua_typename(L: *mut lua_State, tp: i32) -> *const libc::c_char;
    fn lua_tonumberx(L: *mut lua_State, index: i32, isnum: *mut i32) -> f64;
    fn lua_tointegerx(L: *mut lua_State, index: i32, isnum: *mut i32) -> i64;
    fn lua_toboolean(L: *mut lua_State, index: i32) -> i32;
    fn lua_tolstring(L: *mut lua_State, index: i32, len: *mut u64) -> *const libc::c_char;
    fn lua_rawlen(L: *mut lua_State, index: i32) -> u64;
    fn lua_touserdata(L: *mut lua_State, index: i32) -> *mut libc::c_void;
    fn lua_topointer(L: *mut lua_State, index: i32) -> *const libc::c_void;
    fn lua_rawequal(L: *mut lua_State, index1: i32, index2: i32) -> i32;
    fn lua_pushnil(L: *mut lua_State);
    fn lua_pushinteger(L: *mut lua_State, n: i64);
    fn lua_pushlstring(L: *mut lua_State, s: *const libc::c_char, len: u64) -> *const libc::c_char;
    fn lua_pushstring(L: *mut lua_State, s: *const libc::c_char) -> *const libc::c_char;
    fn lua_pushvfstring(
        L: *mut lua_State,
        fmt: *const libc::c_char,
        argp: ::core::ffi::VaList,
    ) -> *const libc::c_char;
    fn lua_pushfstring(L: *mut lua_State, fmt: *const libc::c_char, _: ...) -> *const libc::c_char;
    fn lua_pushcclosure(L: *mut lua_State, fn_0: CFunction, n: i32);
    fn lua_pushboolean(L: *mut lua_State, b: i32);
    fn lua_pushlightuserdata(L: *mut lua_State, p: *mut libc::c_void);
    fn lua_getfield(L: *mut lua_State, index: i32, k: *const libc::c_char) -> i32;
    fn lua_rawget(L: *mut lua_State, index: i32) -> i32;
    fn lua_rawgeti(L: *mut lua_State, index: i32, n: i64) -> i32;
    fn lua_createtable(L: *mut lua_State, narr: i32, nrec: i32);
    fn lua_newuserdatauv(L: *mut lua_State, sz: u64, nuvalue: i32) -> *mut libc::c_void;
    fn lua_getmetatable(L: *mut lua_State, objindex: i32) -> i32;
    fn lua_setglobal(L: *mut lua_State, name: *const libc::c_char);
    fn lua_setfield(L: *mut lua_State, index: i32, k: *const libc::c_char);
    fn lua_rawseti(L: *mut lua_State, index: i32, n: i64);
    fn lua_setmetatable(L: *mut lua_State, objindex: i32) -> i32;
    fn lua_callk(L: *mut lua_State, nargs: i32, nresults: i32, ctx: lua_KContext, k: lua_KFunction);
    fn lua_load(
        L: *mut lua_State,
        reader: lua_Reader,
        dt: *mut libc::c_void,
        chunkname: *const libc::c_char,
        mode: *const libc::c_char,
    ) -> i32;
    fn lua_setwarnf(L: *mut lua_State, f: lua_WarnFunction, ud: *mut libc::c_void);
    fn lua_error(L: *mut lua_State) -> i32;
    fn lua_next(L: *mut lua_State, index: i32) -> i32;
    fn lua_concat(L: *mut lua_State, n: i32);
    fn lua_len(L: *mut lua_State, index: i32);
    fn lua_getallocf(L: *mut lua_State, ud: *mut *mut libc::c_void) -> lua_Alloc;
    fn lua_toclose(L: *mut lua_State, index: i32);
    fn lua_closeslot(L: *mut lua_State, index: i32);
    fn lua_getstack(L: *mut lua_State, level: i32, ar: *mut lua_Debug) -> i32;
    fn lua_getinfo(L: *mut lua_State, what: *const libc::c_char, ar: *mut lua_Debug) -> i32;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: u32,
    pub fp_offset: u32,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type va_list = __builtin_va_list;
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

pub type lua_KContext = i64;
pub type CFunction = Option<unsafe extern "C" fn(*mut lua_State) -> i32>;
pub type lua_KFunction = Option<unsafe extern "C" fn(*mut lua_State, i32, lua_KContext) -> i32>;
pub type lua_Reader = Option<
    unsafe extern "C" fn(*mut lua_State, *mut libc::c_void, *mut u64) -> *const libc::c_char,
>;
pub type lua_Alloc = Option<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void, u64, u64) -> *mut libc::c_void,
>;
pub type lua_WarnFunction =
    Option<unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, i32) -> ()>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaL_Buffer {
    pub b: *mut libc::c_char,
    pub size: u64,
    pub n: u64,
    pub L: *mut lua_State,
    pub init: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub n: f64,
    pub u: f64,
    pub s: *mut libc::c_void,
    pub i: i64,
    pub l: i64,
    pub b: [libc::c_char; 1024],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaL_Reg {
    pub name: *const libc::c_char,
    pub func: CFunction,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LoadF {
    pub n: i32,
    pub f: *mut FILE,
    pub buff: [libc::c_char; 8192],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LoadS {
    pub s: *const libc::c_char,
    pub size: u64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UBox {
    pub box_0: *mut libc::c_void,
    pub bsize: u64,
}
unsafe extern "C" fn findfield(mut L: *mut lua_State, mut objidx: i32, mut level: i32) -> i32 {
    if level == 0i32 || !(lua_type(L, -(1i32)) == 5i32) {
        return 0i32;
    }
    lua_pushnil(L);
    while lua_next(L, -(2i32)) != 0 {
        if lua_type(L, -(2i32)) == 4i32 {
            if lua_rawequal(L, objidx, -(1i32)) != 0 {
                lua_settop(L, -(1i32) - 1i32);
                return 1i32;
            } else if findfield(L, objidx, level - 1i32) != 0 {
                lua_pushstring(L, b".\0" as *const u8 as *const libc::c_char);
                lua_copy(L, -(1i32), -(3i32));
                lua_settop(L, -(1i32) - 1i32);
                lua_concat(L, 3i32);
                return 1i32;
            }
        }
        lua_settop(L, -(1i32) - 1i32);
    }
    return 0i32;
}
unsafe extern "C" fn pushglobalfuncname(mut L: *mut lua_State, mut ar: *mut lua_Debug) -> i32 {
    let mut top: i32 = lua_gettop(L);
    lua_getinfo(L, b"f\0" as *const u8 as *const libc::c_char, ar);
    lua_getfield(
        L,
        -(1000000i32) - 1000i32,
        b"_LOADED\0" as *const u8 as *const libc::c_char,
    );
    luaL_checkstack(
        L,
        6i32,
        b"not enough stack\0" as *const u8 as *const libc::c_char,
    );
    if findfield(L, top + 1i32, 2i32) != 0 {
        let mut name: *const libc::c_char = lua_tolstring(L, -(1i32), 0 as *mut u64);
        if strncmp(
            name,
            b"_G.\0" as *const u8 as *const libc::c_char,
            3i32 as libc::c_ulong,
        ) == 0i32
        {
            lua_pushstring(L, name.offset(3i32 as isize));
            lua_rotate(L, -(2i32), -(1i32));
            lua_settop(L, -(1i32) - 1i32);
        }
        lua_copy(L, -(1i32), top + 1i32);
        lua_settop(L, top + 1i32);
        return 1i32;
    } else {
        lua_settop(L, top);
        return 0i32;
    };
}
unsafe extern "C" fn pushfuncname(mut L: *mut lua_State, mut ar: *mut lua_Debug) {
    if pushglobalfuncname(L, ar) != 0 {
        lua_pushfstring(
            L,
            b"function '%s'\0" as *const u8 as *const libc::c_char,
            lua_tolstring(L, -(1i32), 0 as *mut u64),
        );
        lua_rotate(L, -(2i32), -(1i32));
        lua_settop(L, -(1i32) - 1i32);
    } else if *(*ar).namewhat as i32 != '\0' as i32 {
        lua_pushfstring(
            L,
            b"%s '%s'\0" as *const u8 as *const libc::c_char,
            (*ar).namewhat,
            (*ar).name,
        );
    } else if *(*ar).what as i32 == 'm' as i32 {
        lua_pushstring(L, b"main chunk\0" as *const u8 as *const libc::c_char);
    } else if *(*ar).what as i32 != 'C' as i32 {
        lua_pushfstring(
            L,
            b"function <%s:%d>\0" as *const u8 as *const libc::c_char,
            ((*ar).short_src).as_mut_ptr(),
            (*ar).linedefined,
        );
    } else {
        lua_pushstring(L, b"?\0" as *const u8 as *const libc::c_char);
    };
}
unsafe extern "C" fn lastlevel(mut L: *mut lua_State) -> i32 {
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
    let mut li: i32 = 1i32;
    let mut le: i32 = 1i32;
    while lua_getstack(L, le, &mut ar) != 0 {
        li = le;
        le *= 2i32;
    }
    while li < le {
        let mut m: i32 = (li + le) / 2i32;
        if lua_getstack(L, m, &mut ar) != 0 {
            li = m + 1i32;
        } else {
            le = m;
        }
    }
    return le - 1i32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaL_traceback(
    mut L: *mut lua_State,
    mut L1: *mut lua_State,
    mut msg: *const libc::c_char,
    mut level: i32,
) {
    let mut b: luaL_Buffer = luaL_Buffer {
        b: 0 as *mut libc::c_char,
        size: 0,
        n: 0,
        L: 0 as *mut lua_State,
        init: C2RustUnnamed { n: 0. },
    };
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
    let mut last: i32 = lastlevel(L1);
    let mut limit2show: i32 = if last - level > 10i32 + 11i32 {
        10i32
    } else {
        -(1i32)
    };
    luaL_buffinit(L, &mut b);
    if !msg.is_null() {
        luaL_addstring(&mut b, msg);
        (b.n < b.size || !(luaL_prepbuffsize(&mut b, 1i32 as u64)).is_null()) as i32;
        let fresh0 = b.n;
        b.n = (b.n).wrapping_add(1);
        *(b.b).offset(fresh0 as isize) = '\n' as i32 as libc::c_char;
    }
    luaL_addstring(
        &mut b,
        b"stack traceback:\0" as *const u8 as *const libc::c_char,
    );
    loop {
        let fresh1 = level;
        level = level + 1;
        if !(lua_getstack(L1, fresh1, &mut ar) != 0) {
            break;
        }
        let fresh2 = limit2show;
        limit2show = limit2show - 1;
        if fresh2 == 0i32 {
            let mut n: i32 = last - level - 11i32 + 1i32;
            lua_pushfstring(
                L,
                b"\n\t...\t(skipping %d levels)\0" as *const u8 as *const libc::c_char,
                n,
            );
            luaL_addvalue(&mut b);
            level += n;
        } else {
            lua_getinfo(L1, b"Slnt\0" as *const u8 as *const libc::c_char, &mut ar);
            if ar.currentline <= 0i32 {
                lua_pushfstring(
                    L,
                    b"\n\t%s: in \0" as *const u8 as *const libc::c_char,
                    (ar.short_src).as_mut_ptr(),
                );
            } else {
                lua_pushfstring(
                    L,
                    b"\n\t%s:%d: in \0" as *const u8 as *const libc::c_char,
                    (ar.short_src).as_mut_ptr(),
                    ar.currentline,
                );
            }
            luaL_addvalue(&mut b);
            pushfuncname(L, &mut ar);
            luaL_addvalue(&mut b);
            if ar.istailcall != 0 {
                luaL_addstring(
                    &mut b,
                    b"\n\t(...tail calls...)\0" as *const u8 as *const libc::c_char,
                );
            }
        }
    }
    luaL_pushresult(&mut b);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaL_argerror(
    mut L: *mut lua_State,
    mut arg: i32,
    mut extramsg: *const libc::c_char,
) -> i32 {
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
    if lua_getstack(L, 0i32, &mut ar) == 0 {
        return luaL_error(
            L,
            b"bad argument #%d (%s)\0" as *const u8 as *const libc::c_char,
            arg,
            extramsg,
        );
    }
    lua_getinfo(L, b"n\0" as *const u8 as *const libc::c_char, &mut ar);
    if strcmp(ar.namewhat, b"method\0" as *const u8 as *const libc::c_char) == 0i32 {
        arg -= 1;
        if arg == 0i32 {
            return luaL_error(
                L,
                b"calling '%s' on bad self (%s)\0" as *const u8 as *const libc::c_char,
                ar.name,
                extramsg,
            );
        }
    }
    if (ar.name).is_null() {
        ar.name = if pushglobalfuncname(L, &mut ar) != 0 {
            lua_tolstring(L, -(1i32), 0 as *mut u64)
        } else {
            b"?\0" as *const u8 as *const libc::c_char
        };
    }
    return luaL_error(
        L,
        b"bad argument #%d to '%s' (%s)\0" as *const u8 as *const libc::c_char,
        arg,
        ar.name,
        extramsg,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaL_typeerror(
    mut L: *mut lua_State,
    mut arg: i32,
    mut tname: *const libc::c_char,
) -> i32 {
    let mut msg: *const libc::c_char = 0 as *const libc::c_char;
    let mut typearg: *const libc::c_char = 0 as *const libc::c_char;
    if luaL_getmetafield(L, arg, b"__name\0" as *const u8 as *const libc::c_char) == 4i32 {
        typearg = lua_tolstring(L, -(1i32), 0 as *mut u64);
    } else if lua_type(L, arg) == 2i32 {
        typearg = b"light userdata\0" as *const u8 as *const libc::c_char;
    } else {
        typearg = lua_typename(L, lua_type(L, arg));
    }
    msg = lua_pushfstring(
        L,
        b"%s expected, got %s\0" as *const u8 as *const libc::c_char,
        tname,
        typearg,
    );
    return luaL_argerror(L, arg, msg);
}
unsafe extern "C" fn tag_error(mut L: *mut lua_State, mut arg: i32, mut tag: i32) {
    luaL_typeerror(L, arg, lua_typename(L, tag));
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaL_where(mut L: *mut lua_State, mut level: i32) {
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
    if lua_getstack(L, level, &mut ar) != 0 {
        lua_getinfo(L, b"Sl\0" as *const u8 as *const libc::c_char, &mut ar);
        if ar.currentline > 0i32 {
            lua_pushfstring(
                L,
                b"%s:%d: \0" as *const u8 as *const libc::c_char,
                (ar.short_src).as_mut_ptr(),
                ar.currentline,
            );
            return;
        }
    }
    lua_pushfstring(L, b"\0" as *const u8 as *const libc::c_char);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaL_error(
    mut L: *mut lua_State,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> i32 {
    let mut argp: ::core::ffi::VaListImpl;
    argp = args.clone();
    luaL_where(L, 1i32);
    lua_pushvfstring(L, fmt, argp.as_va_list());
    lua_concat(L, 2i32);
    return lua_error(L);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaL_fileresult(
    mut L: *mut lua_State,
    mut stat: i32,
    mut fname: *const libc::c_char,
) -> i32 {
    let mut en: i32 = *__errno_location();
    if stat != 0 {
        lua_pushboolean(L, 1i32);
        return 1i32;
    } else {
        let mut msg: *const libc::c_char = 0 as *const libc::c_char;
        lua_pushnil(L);
        msg = if en != 0i32 {
            strerror(en) as *const libc::c_char
        } else {
            b"(no extra info)\0" as *const u8 as *const libc::c_char
        };
        if !fname.is_null() {
            lua_pushfstring(
                L,
                b"%s: %s\0" as *const u8 as *const libc::c_char,
                fname,
                msg,
            );
        } else {
            lua_pushstring(L, msg);
        }
        lua_pushinteger(L, en as i64);
        return 3i32;
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaL_execresult(mut L: *mut lua_State, mut stat: i32) -> i32 {
    if stat != 0i32 && *__errno_location() != 0i32 {
        return luaL_fileresult(L, 0i32, 0 as *const libc::c_char);
    } else {
        let mut what: *const libc::c_char = b"exit\0" as *const u8 as *const libc::c_char;
        if stat & 0x7f as i32 == 0i32 {
            stat = (stat & 0xff00 as i32) >> 8i32;
        } else if ((stat & 0x7f as i32) + 1i32) as libc::c_schar as i32 >> 1i32 > 0i32 {
            stat = stat & 0x7f as i32;
            what = b"signal\0" as *const u8 as *const libc::c_char;
        }
        if *what as i32 == 'e' as i32 && stat == 0i32 {
            lua_pushboolean(L, 1i32);
        } else {
            lua_pushnil(L);
        }
        lua_pushstring(L, what);
        lua_pushinteger(L, stat as i64);
        return 3i32;
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaL_newmetatable(
    mut L: *mut lua_State,
    mut tname: *const libc::c_char,
) -> i32 {
    if lua_getfield(L, -(1000000i32) - 1000i32, tname) != 0i32 {
        return 0i32;
    }
    lua_settop(L, -(1i32) - 1i32);
    lua_createtable(L, 0i32, 2i32);
    lua_pushstring(L, tname);
    lua_setfield(L, -(2i32), b"__name\0" as *const u8 as *const libc::c_char);
    lua_pushvalue(L, -(1i32));
    lua_setfield(L, -(1000000i32) - 1000i32, tname);
    return 1i32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaL_setmetatable(mut L: *mut lua_State, mut tname: *const libc::c_char) {
    lua_getfield(L, -(1000000i32) - 1000i32, tname);
    lua_setmetatable(L, -(2i32));
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaL_testudata(
    mut L: *mut lua_State,
    mut ud: i32,
    mut tname: *const libc::c_char,
) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = lua_touserdata(L, ud);
    if !p.is_null() {
        if lua_getmetatable(L, ud) != 0 {
            lua_getfield(L, -(1000000i32) - 1000i32, tname);
            if lua_rawequal(L, -(1i32), -(2i32)) == 0 {
                p = 0 as *mut libc::c_void;
            }
            lua_settop(L, -(2i32) - 1i32);
            return p;
        }
    }
    return 0 as *mut libc::c_void;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaL_checkudata(
    mut L: *mut lua_State,
    mut ud: i32,
    mut tname: *const libc::c_char,
) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = luaL_testudata(L, ud, tname);
    (((p != 0 as *mut libc::c_void) as i32 != 0i32) as i32 as i64 != 0
        || luaL_typeerror(L, ud, tname) != 0) as i32;
    return p;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaL_checkoption(
    mut L: *mut lua_State,
    mut arg: i32,
    mut def: *const libc::c_char,
    mut lst: *const *const libc::c_char,
) -> i32 {
    let mut name: *const libc::c_char = if !def.is_null() {
        luaL_optlstring(L, arg, def, 0 as *mut u64)
    } else {
        luaL_checklstring(L, arg, 0 as *mut u64)
    };
    let mut i: i32 = 0;
    i = 0i32;
    while !(*lst.offset(i as isize)).is_null() {
        if strcmp(*lst.offset(i as isize), name) == 0i32 {
            return i;
        }
        i += 1;
    }
    return luaL_argerror(
        L,
        arg,
        lua_pushfstring(
            L,
            b"invalid option '%s'\0" as *const u8 as *const libc::c_char,
            name,
        ),
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaL_checkstack(
    mut L: *mut lua_State,
    mut space: i32,
    mut msg: *const libc::c_char,
) {
    if ((lua_checkstack(L, space) == 0) as i32 != 0i32) as i32 as i64 != 0 {
        if !msg.is_null() {
            luaL_error(
                L,
                b"stack overflow (%s)\0" as *const u8 as *const libc::c_char,
                msg,
            );
        } else {
            luaL_error(L, b"stack overflow\0" as *const u8 as *const libc::c_char);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaL_checktype(mut L: *mut lua_State, mut arg: i32, mut t: i32) {
    if ((lua_type(L, arg) != t) as i32 != 0i32) as i32 as i64 != 0 {
        tag_error(L, arg, t);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaL_checkany(mut L: *mut lua_State, mut arg: i32) {
    if ((lua_type(L, arg) == -(1i32)) as i32 != 0i32) as i32 as i64 != 0 {
        luaL_argerror(
            L,
            arg,
            b"value expected\0" as *const u8 as *const libc::c_char,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaL_checklstring(
    mut L: *mut lua_State,
    mut arg: i32,
    mut len: *mut u64,
) -> *const libc::c_char {
    let mut s: *const libc::c_char = lua_tolstring(L, arg, len);
    if (s.is_null() as i32 != 0i32) as i32 as i64 != 0 {
        tag_error(L, arg, 4i32);
    }
    return s;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaL_optlstring(
    mut L: *mut lua_State,
    mut arg: i32,
    mut def: *const libc::c_char,
    mut len: *mut u64,
) -> *const libc::c_char {
    if lua_type(L, arg) <= 0i32 {
        if !len.is_null() {
            *len = if !def.is_null() {
                strlen(def)
            } else {
                0i32 as libc::c_ulong
            };
        }
        return def;
    } else {
        return luaL_checklstring(L, arg, len);
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaL_checknumber(mut L: *mut lua_State, mut arg: i32) -> f64 {
    let mut isnum: i32 = 0;
    let mut d: f64 = lua_tonumberx(L, arg, &mut isnum);
    if ((isnum == 0) as i32 != 0i32) as i32 as i64 != 0 {
        tag_error(L, arg, 3i32);
    }
    return d;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaL_optnumber(mut L: *mut lua_State, mut arg: i32, mut def: f64) -> f64 {
    return if lua_type(L, arg) <= 0i32 {
        def
    } else {
        luaL_checknumber(L, arg)
    };
}
unsafe extern "C" fn interror(mut L: *mut lua_State, mut arg: i32) {
    if lua_isnumber(L, arg) != 0 {
        luaL_argerror(
            L,
            arg,
            b"number has no integer representation\0" as *const u8 as *const libc::c_char,
        );
    } else {
        tag_error(L, arg, 3i32);
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaL_checkinteger(mut L: *mut lua_State, mut arg: i32) -> i64 {
    let mut isnum: i32 = 0;
    let mut d: i64 = lua_tointegerx(L, arg, &mut isnum);
    if ((isnum == 0) as i32 != 0i32) as i32 as i64 != 0 {
        interror(L, arg);
    }
    return d;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaL_optinteger(mut L: *mut lua_State, mut arg: i32, mut def: i64) -> i64 {
    return if lua_type(L, arg) <= 0i32 {
        def
    } else {
        luaL_checkinteger(L, arg)
    };
}
unsafe extern "C" fn resizebox(
    mut L: *mut lua_State,
    mut index: i32,
    mut newsize: u64,
) -> *mut libc::c_void {
    let mut ud: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut allocf: lua_Alloc = lua_getallocf(L, &mut ud);
    let mut box_0: *mut UBox = lua_touserdata(L, index) as *mut UBox;
    let mut temp: *mut libc::c_void =
        allocf.expect("non-null function pointer")(ud, (*box_0).box_0, (*box_0).bsize, newsize);
    if ((temp.is_null() && newsize > 0i32 as libc::c_ulong) as i32 != 0i32) as i32 as i64 != 0 {
        lua_pushstring(
            L,
            b"not enough memory\0" as *const u8 as *const libc::c_char,
        );
        lua_error(L);
    }
    (*box_0).box_0 = temp;
    (*box_0).bsize = newsize;
    return temp;
}
unsafe extern "C" fn boxgc(mut L: *mut lua_State) -> i32 {
    resizebox(L, 1i32, 0i32 as u64);
    return 0i32;
}
static mut boxmt: [luaL_Reg; 3] = {
    [
        {
            let mut init = luaL_Reg {
                name: b"__gc\0" as *const u8 as *const libc::c_char,
                func: Some(boxgc as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"__close\0" as *const u8 as *const libc::c_char,
                func: Some(boxgc as unsafe extern "C" fn(*mut lua_State) -> i32),
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
unsafe extern "C" fn newbox(mut L: *mut lua_State) {
    let mut box_0: *mut UBox =
        lua_newuserdatauv(L, ::core::mem::size_of::<UBox>() as libc::c_ulong, 0i32) as *mut UBox;
    (*box_0).box_0 = 0 as *mut libc::c_void;
    (*box_0).bsize = 0i32 as u64;
    if luaL_newmetatable(L, b"_UBOX*\0" as *const u8 as *const libc::c_char) != 0 {
        luaL_setfuncs(L, boxmt.as_ptr(), 0i32);
    }
    lua_setmetatable(L, -(2i32));
}
unsafe extern "C" fn newbuffsize(mut B: *mut luaL_Buffer, mut sz: u64) -> u64 {
    let mut newsize: u64 = ((*B).size)
        .wrapping_div(2i32 as libc::c_ulong)
        .wrapping_mul(3i32 as libc::c_ulong);
    if (((!(0i32 as u64)).wrapping_sub(sz) < (*B).n) as i32 != 0i32) as i32 as i64 != 0 {
        return luaL_error(
            (*B).L,
            b"buffer too large\0" as *const u8 as *const libc::c_char,
        ) as u64;
    }
    if newsize < ((*B).n).wrapping_add(sz) {
        newsize = ((*B).n).wrapping_add(sz);
    }
    return newsize;
}
unsafe extern "C" fn prepbuffsize(
    mut B: *mut luaL_Buffer,
    mut sz: u64,
    mut boxidx: i32,
) -> *mut libc::c_char {
    if ((*B).size).wrapping_sub((*B).n) >= sz {
        return ((*B).b).offset((*B).n as isize);
    } else {
        let mut L: *mut lua_State = (*B).L;
        let mut newbuff: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut newsize: u64 = newbuffsize(B, sz);
        if (*B).b != ((*B).init.b).as_mut_ptr() {
            newbuff = resizebox(L, boxidx, newsize) as *mut libc::c_char;
        } else {
            lua_rotate(L, boxidx, -(1i32));
            lua_settop(L, -(1i32) - 1i32);
            newbox(L);
            lua_rotate(L, boxidx, 1i32);
            lua_toclose(L, boxidx);
            newbuff = resizebox(L, boxidx, newsize) as *mut libc::c_char;
            memcpy(
                newbuff as *mut libc::c_void,
                (*B).b as *const libc::c_void,
                ((*B).n).wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
            );
        }
        (*B).b = newbuff;
        (*B).size = newsize;
        return newbuff.offset((*B).n as isize);
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaL_prepbuffsize(
    mut B: *mut luaL_Buffer,
    mut sz: u64,
) -> *mut libc::c_char {
    return prepbuffsize(B, sz, -(1i32));
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaL_addlstring(
    mut B: *mut luaL_Buffer,
    mut s: *const libc::c_char,
    mut l: u64,
) {
    if l > 0i32 as libc::c_ulong {
        let mut b: *mut libc::c_char = prepbuffsize(B, l, -(1i32));
        memcpy(
            b as *mut libc::c_void,
            s as *const libc::c_void,
            l.wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
        );
        (*B).n = ((*B).n as libc::c_ulong).wrapping_add(l) as u64 as u64;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaL_addstring(mut B: *mut luaL_Buffer, mut s: *const libc::c_char) {
    luaL_addlstring(B, s, strlen(s));
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaL_pushresult(mut B: *mut luaL_Buffer) {
    let mut L: *mut lua_State = (*B).L;
    lua_pushlstring(L, (*B).b, (*B).n);
    if (*B).b != ((*B).init.b).as_mut_ptr() {
        lua_closeslot(L, -(2i32));
    }
    lua_rotate(L, -(2i32), -(1i32));
    lua_settop(L, -(1i32) - 1i32);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaL_pushresultsize(mut B: *mut luaL_Buffer, mut sz: u64) {
    (*B).n = ((*B).n as libc::c_ulong).wrapping_add(sz) as u64 as u64;
    luaL_pushresult(B);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaL_addvalue(mut B: *mut luaL_Buffer) {
    let mut L: *mut lua_State = (*B).L;
    let mut len: u64 = 0;
    let mut s: *const libc::c_char = lua_tolstring(L, -(1i32), &mut len);
    let mut b: *mut libc::c_char = prepbuffsize(B, len, -(2i32));
    memcpy(
        b as *mut libc::c_void,
        s as *const libc::c_void,
        len.wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
    );
    (*B).n = ((*B).n as libc::c_ulong).wrapping_add(len) as u64 as u64;
    lua_settop(L, -(1i32) - 1i32);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaL_buffinit(mut L: *mut lua_State, mut B: *mut luaL_Buffer) {
    (*B).L = L;
    (*B).b = ((*B).init.b).as_mut_ptr();
    (*B).n = 0i32 as u64;
    (*B).size = (16i32 as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<f64>() as libc::c_ulong) as i32 as u64;
    lua_pushlightuserdata(L, B as *mut libc::c_void);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaL_buffinitsize(
    mut L: *mut lua_State,
    mut B: *mut luaL_Buffer,
    mut sz: u64,
) -> *mut libc::c_char {
    luaL_buffinit(L, B);
    return prepbuffsize(B, sz, -(1i32));
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaL_ref(mut L: *mut lua_State, mut t: i32) -> i32 {
    let mut ref_0: i32 = 0;
    if lua_type(L, -(1i32)) == 0i32 {
        lua_settop(L, -(1i32) - 1i32);
        return -(1i32);
    }
    t = lua_absindex(L, t);
    if lua_rawgeti(L, t, (2i32 + 1i32) as i64) == 0i32 {
        ref_0 = 0i32;
        lua_pushinteger(L, 0i32 as i64);
        lua_rawseti(L, t, (2i32 + 1i32) as i64);
    } else {
        ref_0 = lua_tointegerx(L, -(1i32), 0 as *mut i32) as i32;
    }
    lua_settop(L, -(1i32) - 1i32);
    if ref_0 != 0i32 {
        lua_rawgeti(L, t, ref_0 as i64);
        lua_rawseti(L, t, (2i32 + 1i32) as i64);
    } else {
        ref_0 = lua_rawlen(L, t) as i32 + 1i32;
    }
    lua_rawseti(L, t, ref_0 as i64);
    return ref_0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaL_unref(mut L: *mut lua_State, mut t: i32, mut ref_0: i32) {
    if ref_0 >= 0i32 {
        t = lua_absindex(L, t);
        lua_rawgeti(L, t, (2i32 + 1i32) as i64);
        lua_rawseti(L, t, ref_0 as i64);
        lua_pushinteger(L, ref_0 as i64);
        lua_rawseti(L, t, (2i32 + 1i32) as i64);
    }
}
unsafe extern "C" fn getF(
    mut _L: *mut lua_State,
    mut ud: *mut libc::c_void,
    mut size: *mut u64,
) -> *const libc::c_char {
    let mut lf: *mut LoadF = ud as *mut LoadF;
    if (*lf).n > 0i32 {
        *size = (*lf).n as u64;
        (*lf).n = 0i32;
    } else {
        if feof((*lf).f) != 0 {
            return 0 as *const libc::c_char;
        }
        *size = fread(
            ((*lf).buff).as_mut_ptr() as *mut libc::c_void,
            1i32 as libc::c_ulong,
            ::core::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong,
            (*lf).f,
        );
    }
    return ((*lf).buff).as_mut_ptr();
}
unsafe extern "C" fn errfile(
    mut L: *mut lua_State,
    mut what: *const libc::c_char,
    mut fnameindex: i32,
) -> i32 {
    let mut err: i32 = *__errno_location();
    let mut filename: *const libc::c_char =
        (lua_tolstring(L, fnameindex, 0 as *mut u64)).offset(1i32 as isize);
    if err != 0i32 {
        lua_pushfstring(
            L,
            b"cannot %s %s: %s\0" as *const u8 as *const libc::c_char,
            what,
            filename,
            strerror(err),
        );
    } else {
        lua_pushfstring(
            L,
            b"cannot %s %s\0" as *const u8 as *const libc::c_char,
            what,
            filename,
        );
    }
    lua_rotate(L, fnameindex, -(1i32));
    lua_settop(L, -(1i32) - 1i32);
    return 5i32 + 1i32;
}
unsafe extern "C" fn skipBOM(mut f: *mut FILE) -> i32 {
    let mut c: i32 = getc(f);
    if c == 0xef as i32 && getc(f) == 0xbb as i32 && getc(f) == 0xbf as i32 {
        return getc(f);
    } else {
        return c;
    };
}
unsafe extern "C" fn skipcomment(mut f: *mut FILE, mut cp: *mut i32) -> i32 {
    *cp = skipBOM(f);
    let mut c: i32 = *cp;
    if c == '#' as i32 {
        loop {
            c = getc(f);
            if !(c != -(1i32) && c != '\n' as i32) {
                break;
            }
        }
        *cp = getc(f);
        return 1i32;
    } else {
        return 0i32;
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaL_loadfilex(
    mut L: *mut lua_State,
    mut filename: *const libc::c_char,
    mut mode: *const libc::c_char,
) -> i32 {
    let mut lf: LoadF = LoadF {
        n: 0,
        f: 0 as *mut FILE,
        buff: [0; 8192],
    };
    let mut status: i32 = 0;
    let mut readstatus: i32 = 0;
    let mut c: i32 = 0;
    let mut fnameindex: i32 = lua_gettop(L) + 1i32;
    if filename.is_null() {
        lua_pushstring(L, b"=stdin\0" as *const u8 as *const libc::c_char);
        lf.f = stdin;
    } else {
        lua_pushfstring(L, b"@%s\0" as *const u8 as *const libc::c_char, filename);
        *__errno_location() = 0i32;
        lf.f = fopen(filename, b"r\0" as *const u8 as *const libc::c_char);
        if (lf.f).is_null() {
            return errfile(L, b"open\0" as *const u8 as *const libc::c_char, fnameindex);
        }
    }
    lf.n = 0i32;
    if skipcomment(lf.f, &mut c) != 0 {
        let fresh3 = lf.n;
        lf.n = lf.n + 1;
        lf.buff[fresh3 as usize] = '\n' as i32 as libc::c_char;
    }
    if c == (*::core::mem::transmute::<&[u8; 5], &[libc::c_char; 5]>(b"\x1BLua\0"))[0i32 as usize]
        as i32
    {
        lf.n = 0i32;
        if !filename.is_null() {
            *__errno_location() = 0i32;
            lf.f = freopen(filename, b"rb\0" as *const u8 as *const libc::c_char, lf.f);
            if (lf.f).is_null() {
                return errfile(
                    L,
                    b"reopen\0" as *const u8 as *const libc::c_char,
                    fnameindex,
                );
            }
            skipcomment(lf.f, &mut c);
        }
    }
    if c != -(1i32) {
        let fresh4 = lf.n;
        lf.n = lf.n + 1;
        lf.buff[fresh4 as usize] = c as libc::c_char;
    }
    *__errno_location() = 0i32;
    status = lua_load(
        L,
        Some(
            getF as unsafe extern "C" fn(
                *mut lua_State,
                *mut libc::c_void,
                *mut u64,
            ) -> *const libc::c_char,
        ),
        &mut lf as *mut LoadF as *mut libc::c_void,
        lua_tolstring(L, -(1i32), 0 as *mut u64),
        mode,
    );
    readstatus = ferror(lf.f);
    if !filename.is_null() {
        fclose(lf.f);
    }
    if readstatus != 0 {
        lua_settop(L, fnameindex);
        return errfile(L, b"read\0" as *const u8 as *const libc::c_char, fnameindex);
    }
    lua_rotate(L, fnameindex, -(1i32));
    lua_settop(L, -(1i32) - 1i32);
    return status;
}
unsafe extern "C" fn getS(
    mut _L: *mut lua_State,
    mut ud: *mut libc::c_void,
    mut size: *mut u64,
) -> *const libc::c_char {
    let mut ls: *mut LoadS = ud as *mut LoadS;
    if (*ls).size == 0i32 as libc::c_ulong {
        return 0 as *const libc::c_char;
    }
    *size = (*ls).size;
    (*ls).size = 0i32 as u64;
    return (*ls).s;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaL_loadbufferx(
    mut L: *mut lua_State,
    mut buff: *const libc::c_char,
    mut size: u64,
    mut name: *const libc::c_char,
    mut mode: *const libc::c_char,
) -> i32 {
    let mut ls: LoadS = LoadS {
        s: 0 as *const libc::c_char,
        size: 0,
    };
    ls.s = buff;
    ls.size = size;
    return lua_load(
        L,
        Some(
            getS as unsafe extern "C" fn(
                *mut lua_State,
                *mut libc::c_void,
                *mut u64,
            ) -> *const libc::c_char,
        ),
        &mut ls as *mut LoadS as *mut libc::c_void,
        name,
        mode,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaL_loadstring(mut L: *mut lua_State, mut s: *const libc::c_char) -> i32 {
    return luaL_loadbufferx(L, s, strlen(s), s, 0 as *const libc::c_char);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaL_getmetafield(
    mut L: *mut lua_State,
    mut obj: i32,
    mut event: *const libc::c_char,
) -> i32 {
    if lua_getmetatable(L, obj) == 0 {
        return 0i32;
    } else {
        let mut tt: i32 = 0;
        lua_pushstring(L, event);
        tt = lua_rawget(L, -(2i32));
        if tt == 0i32 {
            lua_settop(L, -(2i32) - 1i32);
        } else {
            lua_rotate(L, -(2i32), -(1i32));
            lua_settop(L, -(1i32) - 1i32);
        }
        return tt;
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaL_callmeta(
    mut L: *mut lua_State,
    mut obj: i32,
    mut event: *const libc::c_char,
) -> i32 {
    obj = lua_absindex(L, obj);
    if luaL_getmetafield(L, obj, event) == 0i32 {
        return 0i32;
    }
    lua_pushvalue(L, obj);
    lua_callk(L, 1i32, 1i32, 0i32 as lua_KContext, None);
    return 1i32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaL_len(mut L: *mut lua_State, mut index: i32) -> i64 {
    let mut l: i64 = 0;
    let mut isnum: i32 = 0;
    lua_len(L, index);
    l = lua_tointegerx(L, -(1i32), &mut isnum);
    if ((isnum == 0) as i32 != 0i32) as i32 as i64 != 0 {
        luaL_error(
            L,
            b"object length is not an integer\0" as *const u8 as *const libc::c_char,
        );
    }
    lua_settop(L, -(1i32) - 1i32);
    return l;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaL_tolstring(
    mut L: *mut lua_State,
    mut index: i32,
    mut len: *mut u64,
) -> *const libc::c_char {
    index = lua_absindex(L, index);
    if luaL_callmeta(
        L,
        index,
        b"__tostring\0" as *const u8 as *const libc::c_char,
    ) != 0
    {
        if lua_isstring(L, -(1i32)) == 0 {
            luaL_error(
                L,
                b"'__tostring' must return a string\0" as *const u8 as *const libc::c_char,
            );
        }
    } else {
        match lua_type(L, index) {
            3 => {
                if lua_isinteger(L, index) != 0 {
                    lua_pushfstring(
                        L,
                        b"%I\0" as *const u8 as *const libc::c_char,
                        lua_tointegerx(L, index, 0 as *mut i32),
                    );
                } else {
                    lua_pushfstring(
                        L,
                        b"%f\0" as *const u8 as *const libc::c_char,
                        lua_tonumberx(L, index, 0 as *mut i32),
                    );
                }
            }
            4 => {
                lua_pushvalue(L, index);
            }
            1 => {
                lua_pushstring(
                    L,
                    if lua_toboolean(L, index) != 0 {
                        b"true\0" as *const u8 as *const libc::c_char
                    } else {
                        b"false\0" as *const u8 as *const libc::c_char
                    },
                );
            }
            0 => {
                lua_pushstring(L, b"nil\0" as *const u8 as *const libc::c_char);
            }
            _ => {
                let mut tt: i32 =
                    luaL_getmetafield(L, index, b"__name\0" as *const u8 as *const libc::c_char);
                let mut kind: *const libc::c_char = if tt == 4i32 {
                    lua_tolstring(L, -(1i32), 0 as *mut u64)
                } else {
                    lua_typename(L, lua_type(L, index))
                };
                lua_pushfstring(
                    L,
                    b"%s: %p\0" as *const u8 as *const libc::c_char,
                    kind,
                    lua_topointer(L, index),
                );
                if tt != 0i32 {
                    lua_rotate(L, -(2i32), -(1i32));
                    lua_settop(L, -(1i32) - 1i32);
                }
            }
        }
    }
    return lua_tolstring(L, -(1i32), len);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaL_setfuncs(
    mut L: *mut lua_State,
    mut l: *const luaL_Reg,
    mut nup: i32,
) {
    luaL_checkstack(
        L,
        nup,
        b"too many upvalues\0" as *const u8 as *const libc::c_char,
    );
    while !((*l).name).is_null() {
        if ((*l).func).is_none() {
            lua_pushboolean(L, 0i32);
        } else {
            let mut i: i32 = 0;
            i = 0i32;
            while i < nup {
                lua_pushvalue(L, -nup);
                i += 1;
            }
            lua_pushcclosure(L, (*l).func, nup);
        }
        lua_setfield(L, -(nup + 2i32), (*l).name);
        l = l.offset(1);
    }
    lua_settop(L, -nup - 1i32);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaL_getsubtable(
    mut L: *mut lua_State,
    mut index: i32,
    mut fname: *const libc::c_char,
) -> i32 {
    if lua_getfield(L, index, fname) == 5i32 {
        return 1i32;
    } else {
        lua_settop(L, -(1i32) - 1i32);
        index = lua_absindex(L, index);
        lua_createtable(L, 0i32, 0i32);
        lua_pushvalue(L, -(1i32));
        lua_setfield(L, index, fname);
        return 0i32;
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaL_requiref(
    mut L: *mut lua_State,
    mut modname: *const libc::c_char,
    mut openf: CFunction,
    mut glb: i32,
) {
    luaL_getsubtable(
        L,
        -(1000000i32) - 1000i32,
        b"_LOADED\0" as *const u8 as *const libc::c_char,
    );
    lua_getfield(L, -(1i32), modname);
    if lua_toboolean(L, -(1i32)) == 0 {
        lua_settop(L, -(1i32) - 1i32);
        lua_pushcclosure(L, openf, 0i32);
        lua_pushstring(L, modname);
        lua_callk(L, 1i32, 1i32, 0i32 as lua_KContext, None);
        lua_pushvalue(L, -(1i32));
        lua_setfield(L, -(3i32), modname);
    }
    lua_rotate(L, -(2i32), -(1i32));
    lua_settop(L, -(1i32) - 1i32);
    if glb != 0 {
        lua_pushvalue(L, -(1i32));
        lua_setglobal(L, modname);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaL_addgsub(
    mut b: *mut luaL_Buffer,
    mut s: *const libc::c_char,
    mut p: *const libc::c_char,
    mut r: *const libc::c_char,
) {
    let mut wild: *const libc::c_char = 0 as *const libc::c_char;
    let mut l: u64 = strlen(p);
    loop {
        wild = strstr(s, p);
        if wild.is_null() {
            break;
        }
        luaL_addlstring(b, s, wild.offset_from(s) as i64 as u64);
        luaL_addstring(b, r);
        s = wild.offset(l as isize);
    }
    luaL_addstring(b, s);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaL_gsub(
    mut L: *mut lua_State,
    mut s: *const libc::c_char,
    mut p: *const libc::c_char,
    mut r: *const libc::c_char,
) -> *const libc::c_char {
    let mut b: luaL_Buffer = luaL_Buffer {
        b: 0 as *mut libc::c_char,
        size: 0,
        n: 0,
        L: 0 as *mut lua_State,
        init: C2RustUnnamed { n: 0. },
    };
    luaL_buffinit(L, &mut b);
    luaL_addgsub(&mut b, s, p, r);
    luaL_pushresult(&mut b);
    return lua_tolstring(L, -(1i32), 0 as *mut u64);
}
unsafe extern "C" fn l_alloc(
    mut _ud: *mut libc::c_void,
    mut ptr: *mut libc::c_void,
    mut _osize: u64,
    mut nsize: u64,
) -> *mut libc::c_void {
    if nsize == 0i32 as libc::c_ulong {
        free(ptr);
        return 0 as *mut libc::c_void;
    } else {
        return realloc(ptr, nsize);
    };
}
unsafe extern "C" fn panic(mut L: *mut lua_State) -> i32 {
    let mut msg: *const libc::c_char = if lua_type(L, -(1i32)) == 4i32 {
        lua_tolstring(L, -(1i32), 0 as *mut u64)
    } else {
        b"error object is not a string\0" as *const u8 as *const libc::c_char
    };
    fprintf(
        stderr,
        b"PANIC: unprotected error in call to Lua API (%s)\n\0" as *const u8 as *const libc::c_char,
        msg,
    );
    fflush(stderr);
    return 0i32;
}
unsafe extern "C" fn checkcontrol(
    mut L: *mut lua_State,
    mut message: *const libc::c_char,
    mut tocont: i32,
) -> i32 {
    if tocont != 0 || {
        let fresh5 = message;
        message = message.offset(1);
        *fresh5 as i32 != '@' as i32
    } {
        return 0i32;
    } else {
        if strcmp(message, b"off\0" as *const u8 as *const libc::c_char) == 0i32 {
            lua_setwarnf(
                L,
                Some(
                    warnfoff
                        as unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, i32) -> (),
                ),
                L as *mut libc::c_void,
            );
        } else if strcmp(message, b"on\0" as *const u8 as *const libc::c_char) == 0i32 {
            lua_setwarnf(
                L,
                Some(
                    warnfon
                        as unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, i32) -> (),
                ),
                L as *mut libc::c_void,
            );
        }
        return 1i32;
    };
}
unsafe extern "C" fn warnfoff(
    mut ud: *mut libc::c_void,
    mut message: *const libc::c_char,
    mut tocont: i32,
) {
    checkcontrol(ud as *mut lua_State, message, tocont);
}
unsafe extern "C" fn warnfcont(
    mut ud: *mut libc::c_void,
    mut message: *const libc::c_char,
    mut tocont: i32,
) {
    let mut L: *mut lua_State = ud as *mut lua_State;
    fprintf(stderr, b"%s\0" as *const u8 as *const libc::c_char, message);
    fflush(stderr);
    if tocont != 0 {
        lua_setwarnf(
            L,
            Some(
                warnfcont
                    as unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, i32) -> (),
            ),
            L as *mut libc::c_void,
        );
    } else {
        fprintf(
            stderr,
            b"%s\0" as *const u8 as *const libc::c_char,
            b"\n\0" as *const u8 as *const libc::c_char,
        );
        fflush(stderr);
        lua_setwarnf(
            L,
            Some(
                warnfon as unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, i32) -> (),
            ),
            L as *mut libc::c_void,
        );
    };
}
unsafe extern "C" fn warnfon(
    mut ud: *mut libc::c_void,
    mut message: *const libc::c_char,
    mut tocont: i32,
) {
    if checkcontrol(ud as *mut lua_State, message, tocont) != 0 {
        return;
    }
    fprintf(
        stderr,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"Lua warning: \0" as *const u8 as *const libc::c_char,
    );
    fflush(stderr);
    warnfcont(ud, message, tocont);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaL_newstate() -> *mut lua_State {
    let mut L: *mut lua_State = lua_newstate(
        Some(
            l_alloc
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                    u64,
                    u64,
                ) -> *mut libc::c_void,
        ),
        0 as *mut libc::c_void,
    );
    if L != (0 as *mut lua_State) {
        lua_atpanic(
            L,
            Some(panic as unsafe extern "C" fn(*mut lua_State) -> i32),
        );
        lua_setwarnf(
            L,
            Some(
                warnfoff as unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, i32) -> (),
            ),
            L as *mut libc::c_void,
        );
    }
    return L;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaL_checkversion_(mut L: *mut lua_State, mut ver: f64, mut sz: u64) {
    let mut v: f64 = lua_version(L);
    if sz
        != (::core::mem::size_of::<i64>() as libc::c_ulong)
            .wrapping_mul(16i32 as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<f64>() as libc::c_ulong)
    {
        luaL_error(
            L,
            b"core and library have incompatible numeric types\0" as *const u8
                as *const libc::c_char,
        );
    } else if v != ver {
        luaL_error(
            L,
            b"version mismatch: app. needs %f, Lua core provides %f\0" as *const u8
                as *const libc::c_char,
            ver,
            v,
        );
    }
}
