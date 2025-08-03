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
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    static mut stdout: *mut FILE;
    fn fflush(__stream: *mut FILE) -> i32;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn lua_gettop(L: *mut lua_State) -> i32;
    fn lua_settop(L: *mut lua_State, index: i32);
    fn lua_pushvalue(L: *mut lua_State, index: i32);
    fn lua_rotate(L: *mut lua_State, index: i32, n: i32);
    fn lua_copy(L: *mut lua_State, fromidx: i32, toidx: i32);
    fn lua_isstring(L: *mut lua_State, index: i32) -> i32;
    fn lua_type(L: *mut lua_State, index: i32) -> i32;
    fn lua_typename(L: *mut lua_State, tp: i32) -> *const libc::c_char;
    fn lua_toboolean(L: *mut lua_State, index: i32) -> i32;
    fn lua_tolstring(L: *mut lua_State, index: i32, len: *mut u64) -> *const libc::c_char;
    fn lua_rawlen(L: *mut lua_State, index: i32) -> u64;
    fn lua_rawequal(L: *mut lua_State, index1: i32, index2: i32) -> i32;
    fn lua_pushnil(L: *mut lua_State);
    fn lua_pushnumber(L: *mut lua_State, n: f64);
    fn lua_pushinteger(L: *mut lua_State, n: i64);
    fn lua_pushstring(L: *mut lua_State, s: *const libc::c_char) -> *const libc::c_char;
    fn lua_pushcclosure(L: *mut lua_State, fn_0: CFunction, n: i32);
    fn lua_pushboolean(L: *mut lua_State, b: i32);
    fn lua_geti(L: *mut lua_State, index: i32, n: i64) -> i32;
    fn lua_rawget(L: *mut lua_State, index: i32) -> i32;
    fn lua_rawgeti(L: *mut lua_State, index: i32, n: i64) -> i32;
    fn lua_getmetatable(L: *mut lua_State, objindex: i32) -> i32;
    fn lua_setfield(L: *mut lua_State, index: i32, k: *const libc::c_char);
    fn lua_rawset(L: *mut lua_State, index: i32);
    fn lua_setmetatable(L: *mut lua_State, objindex: i32) -> i32;
    fn lua_callk(L: *mut lua_State, nargs: i32, nresults: i32, ctx: lua_KContext, k: lua_KFunction);
    fn lua_pcallk(
        L: *mut lua_State,
        nargs: i32,
        nresults: i32,
        errfunc: i32,
        ctx: lua_KContext,
        k: lua_KFunction,
    ) -> i32;
    fn lua_load(
        L: *mut lua_State,
        reader: lua_Reader,
        dt: *mut libc::c_void,
        chunkname: *const libc::c_char,
        mode: *const libc::c_char,
    ) -> i32;
    fn lua_warning(L: *mut lua_State, msg: *const libc::c_char, tocont: i32);
    fn lua_gc(L: *mut lua_State, what: i32, _: ...) -> i32;
    fn lua_error(L: *mut lua_State) -> i32;
    fn lua_next(L: *mut lua_State, index: i32) -> i32;
    fn lua_concat(L: *mut lua_State, n: i32);
    fn lua_stringtonumber(L: *mut lua_State, s: *const libc::c_char) -> u64;
    fn lua_setupvalue(L: *mut lua_State, funcindex: i32, n: i32) -> *const libc::c_char;
    fn luaL_getmetafield(L: *mut lua_State, obj: i32, e: *const libc::c_char) -> i32;
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
    fn luaL_checkstack(L: *mut lua_State, sz: i32, msg: *const libc::c_char);
    fn luaL_checktype(L: *mut lua_State, arg: i32, t: i32);
    fn luaL_checkany(L: *mut lua_State, arg: i32);
    fn luaL_where(L: *mut lua_State, lvl: i32);
    fn luaL_error(L: *mut lua_State, fmt: *const libc::c_char, _: ...) -> i32;
    fn luaL_checkoption(
        L: *mut lua_State,
        arg: i32,
        def: *const libc::c_char,
        lst: *const *const libc::c_char,
    ) -> i32;
    fn luaL_loadfilex(
        L: *mut lua_State,
        filename: *const libc::c_char,
        mode: *const libc::c_char,
    ) -> i32;
    fn luaL_loadbufferx(
        L: *mut lua_State,
        buff: *const libc::c_char,
        sz: u64,
        name: *const libc::c_char,
        mode: *const libc::c_char,
    ) -> i32;
    fn luaL_setfuncs(L: *mut lua_State, l: *const luaL_Reg, nup: i32);
}
pub type __int32_t = i32;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type C2RustUnnamed = u32;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaL_Reg {
    pub name: *const libc::c_char,
    pub func: CFunction,
}
#[inline]
unsafe extern "C" fn toupper(mut __c: i32) -> i32 {
    return if __c >= -(128i32) && __c < 256i32 {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
unsafe extern "C" fn luaB_print(mut L: *mut lua_State) -> i32 {
    let mut n: i32 = lua_gettop(L);
    let mut i: i32 = 0;
    i = 1i32;
    while i <= n {
        let mut l: u64 = 0;
        let mut s: *const libc::c_char = luaL_tolstring(L, i, &mut l);
        if i > 1i32 {
            fwrite(
                b"\t\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                1i32 as libc::c_ulong,
                stdout,
            );
        }
        fwrite(
            s as *const libc::c_void,
            ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
            l,
            stdout,
        );
        lua_settop(L, -(1i32) - 1i32);
        i += 1;
    }
    fwrite(
        b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
        1i32 as libc::c_ulong,
        stdout,
    );
    fflush(stdout);
    return 0i32;
}
unsafe extern "C" fn luaB_warn(mut L: *mut lua_State) -> i32 {
    let mut n: i32 = lua_gettop(L);
    let mut i: i32 = 0;
    luaL_checklstring(L, 1i32, 0 as *mut u64);
    i = 2i32;
    while i <= n {
        luaL_checklstring(L, i, 0 as *mut u64);
        i += 1;
    }
    i = 1i32;
    while i < n {
        lua_warning(L, lua_tolstring(L, i, 0 as *mut u64), 1i32);
        i += 1;
    }
    lua_warning(L, lua_tolstring(L, n, 0 as *mut u64), 0i32);
    return 0i32;
}
unsafe extern "C" fn b_str2int(
    mut s: *const libc::c_char,
    mut base: i32,
    mut pn: *mut i64,
) -> *const libc::c_char {
    let mut n: u64 = 0i32 as u64;
    let mut neg: i32 = 0i32;
    s = s.offset(strspn(s, b" \x0C\n\r\t\x0B\0" as *const u8 as *const libc::c_char) as isize);
    if *s as i32 == '-' as i32 {
        s = s.offset(1);
        neg = 1i32;
    } else if *s as i32 == '+' as i32 {
        s = s.offset(1);
    }
    if *(*__ctype_b_loc()).offset(*s as u8 as i32 as isize) as i32
        & _ISalnum as i32 as libc::c_ushort as i32
        == 0
    {
        return 0 as *const libc::c_char;
    }
    loop {
        let mut digit: i32 = if *(*__ctype_b_loc()).offset(*s as u8 as i32 as isize) as i32
            & _ISdigit as i32 as libc::c_ushort as i32
            != 0
        {
            *s as i32 - '0' as i32
        } else {
            ({
                let mut __res: i32 = 0;
                if ::core::mem::size_of::<u8>() as libc::c_ulong > 1i32 as libc::c_ulong {
                    if 0 != 0 {
                        let mut __c: i32 = *s as u8 as i32;
                        __res = if __c < -(128i32) || __c > 255i32 {
                            __c
                        } else {
                            *(*__ctype_toupper_loc()).offset(__c as isize)
                        }
                    } else {
                        __res = toupper(*s as u8 as i32);
                    }
                } else {
                    __res = *(*__ctype_toupper_loc()).offset(*s as u8 as i32 as isize);
                }
                __res
            }) - 'A' as i32
                + 10i32
        };
        if digit >= base {
            return 0 as *const libc::c_char;
        }
        n = n.wrapping_mul(base as u64).wrapping_add(digit as u64);
        s = s.offset(1);
        if !(*(*__ctype_b_loc()).offset(*s as u8 as i32 as isize) as i32
            & _ISalnum as i32 as libc::c_ushort as i32
            != 0)
        {
            break;
        }
    }
    s = s.offset(strspn(s, b" \x0C\n\r\t\x0B\0" as *const u8 as *const libc::c_char) as isize);
    *pn = (if neg != 0 {
        (0 as u32 as u64).wrapping_sub(n)
    } else {
        n
    }) as i64;
    return s;
}
unsafe extern "C" fn luaB_tonumber(mut L: *mut lua_State) -> i32 {
    if lua_type(L, 2i32) <= 0i32 {
        if lua_type(L, 1i32) == 3i32 {
            lua_settop(L, 1i32);
            return 1i32;
        } else {
            let mut l: u64 = 0;
            let mut s: *const libc::c_char = lua_tolstring(L, 1i32, &mut l);
            if !s.is_null() && lua_stringtonumber(L, s) == l.wrapping_add(1i32 as libc::c_ulong) {
                return 1i32;
            }
            luaL_checkany(L, 1i32);
        }
    } else {
        let mut l_0: u64 = 0;
        let mut s_0: *const libc::c_char = 0 as *const libc::c_char;
        let mut n: i64 = 0i32 as i64;
        let mut base: i64 = luaL_checkinteger(L, 2i32);
        luaL_checktype(L, 1i32, 4i32);
        s_0 = lua_tolstring(L, 1i32, &mut l_0);
        (((2i32 as i64 <= base && base <= 36i32 as i64) as i32 != 0i32) as i32 as i64 != 0
            || luaL_argerror(
                L,
                2i32,
                b"base out of range\0" as *const u8 as *const libc::c_char,
            ) != 0) as i32;
        if b_str2int(s_0, base as i32, &mut n) == s_0.offset(l_0 as isize) {
            lua_pushinteger(L, n);
            return 1i32;
        }
    }
    lua_pushnil(L);
    return 1i32;
}
unsafe extern "C" fn luaB_error(mut L: *mut lua_State) -> i32 {
    let mut level: i32 = luaL_optinteger(L, 2i32, 1i32 as i64) as i32;
    lua_settop(L, 1i32);
    if lua_type(L, 1i32) == 4i32 && level > 0i32 {
        luaL_where(L, level);
        lua_pushvalue(L, 1i32);
        lua_concat(L, 2i32);
    }
    return lua_error(L);
}
unsafe extern "C" fn luaB_getmetatable(mut L: *mut lua_State) -> i32 {
    luaL_checkany(L, 1i32);
    if lua_getmetatable(L, 1i32) == 0 {
        lua_pushnil(L);
        return 1i32;
    }
    luaL_getmetafield(
        L,
        1i32,
        b"__metatable\0" as *const u8 as *const libc::c_char,
    );
    return 1i32;
}
unsafe extern "C" fn luaB_setmetatable(mut L: *mut lua_State) -> i32 {
    let mut t: i32 = lua_type(L, 2i32);
    luaL_checktype(L, 1i32, 5i32);
    (((t == 0i32 || t == 5i32) as i32 != 0i32) as i32 as i64 != 0
        || luaL_typeerror(
            L,
            2i32,
            b"nil or table\0" as *const u8 as *const libc::c_char,
        ) != 0) as i32;
    if ((luaL_getmetafield(
        L,
        1i32,
        b"__metatable\0" as *const u8 as *const libc::c_char,
    ) != 0i32) as i32
        != 0i32) as i32 as i64
        != 0
    {
        return luaL_error(
            L,
            b"cannot change a protected metatable\0" as *const u8 as *const libc::c_char,
        );
    }
    lua_settop(L, 2i32);
    lua_setmetatable(L, 1i32);
    return 1i32;
}
unsafe extern "C" fn luaB_rawequal(mut L: *mut lua_State) -> i32 {
    luaL_checkany(L, 1i32);
    luaL_checkany(L, 2i32);
    lua_pushboolean(L, lua_rawequal(L, 1i32, 2i32));
    return 1i32;
}
unsafe extern "C" fn luaB_rawlen(mut L: *mut lua_State) -> i32 {
    let mut t: i32 = lua_type(L, 1i32);
    (((t == 5i32 || t == 4i32) as i32 != 0i32) as i32 as i64 != 0
        || luaL_typeerror(
            L,
            1i32,
            b"table or string\0" as *const u8 as *const libc::c_char,
        ) != 0) as i32;
    lua_pushinteger(L, lua_rawlen(L, 1i32) as i64);
    return 1i32;
}
unsafe extern "C" fn luaB_rawget(mut L: *mut lua_State) -> i32 {
    luaL_checktype(L, 1i32, 5i32);
    luaL_checkany(L, 2i32);
    lua_settop(L, 2i32);
    lua_rawget(L, 1i32);
    return 1i32;
}
unsafe extern "C" fn luaB_rawset(mut L: *mut lua_State) -> i32 {
    luaL_checktype(L, 1i32, 5i32);
    luaL_checkany(L, 2i32);
    luaL_checkany(L, 3i32);
    lua_settop(L, 3i32);
    lua_rawset(L, 1i32);
    return 1i32;
}
unsafe extern "C" fn pushmode(mut L: *mut lua_State, mut oldmode: i32) -> i32 {
    if oldmode == -(1i32) {
        lua_pushnil(L);
    } else {
        lua_pushstring(
            L,
            if oldmode == 11i32 {
                b"incremental\0" as *const u8 as *const libc::c_char
            } else {
                b"generational\0" as *const u8 as *const libc::c_char
            },
        );
    }
    return 1i32;
}
unsafe extern "C" fn luaB_collectgarbage(mut L: *mut lua_State) -> i32 {
    static mut opts: [*const libc::c_char; 11] = [
        b"stop\0" as *const u8 as *const libc::c_char,
        b"restart\0" as *const u8 as *const libc::c_char,
        b"collect\0" as *const u8 as *const libc::c_char,
        b"count\0" as *const u8 as *const libc::c_char,
        b"step\0" as *const u8 as *const libc::c_char,
        b"setpause\0" as *const u8 as *const libc::c_char,
        b"setstepmul\0" as *const u8 as *const libc::c_char,
        b"isrunning\0" as *const u8 as *const libc::c_char,
        b"generational\0" as *const u8 as *const libc::c_char,
        b"incremental\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    static mut optsnum: [i32; 10] = [0i32, 1i32, 2i32, 3i32, 5i32, 6i32, 7i32, 9i32, 10i32, 11i32];
    let mut o: i32 = optsnum[luaL_checkoption(
        L,
        1i32,
        b"collect\0" as *const u8 as *const libc::c_char,
        opts.as_ptr(),
    ) as usize];
    match o {
        3 => {
            let mut k: i32 = lua_gc(L, o);
            let mut b: i32 = lua_gc(L, 4i32);
            if !(k == -(1i32)) {
                lua_pushnumber(L, k as f64 + b as f64 / 1024i32 as f64);
                return 1i32;
            }
        }
        5 => {
            let mut step: i32 = luaL_optinteger(L, 2i32, 0i32 as i64) as i32;
            let mut res: i32 = lua_gc(L, o, step);
            if !(res == -(1i32)) {
                lua_pushboolean(L, res);
                return 1i32;
            }
        }
        6 | 7 => {
            let mut p: i32 = luaL_optinteger(L, 2i32, 0i32 as i64) as i32;
            let mut previous: i32 = lua_gc(L, o, p);
            if !(previous == -(1i32)) {
                lua_pushinteger(L, previous as i64);
                return 1i32;
            }
        }
        9 => {
            let mut res_0: i32 = lua_gc(L, o);
            if !(res_0 == -(1i32)) {
                lua_pushboolean(L, res_0);
                return 1i32;
            }
        }
        10 => {
            let mut minormul: i32 = luaL_optinteger(L, 2i32, 0i32 as i64) as i32;
            let mut majormul: i32 = luaL_optinteger(L, 3i32, 0i32 as i64) as i32;
            return pushmode(L, lua_gc(L, o, minormul, majormul));
        }
        11 => {
            let mut pause: i32 = luaL_optinteger(L, 2i32, 0i32 as i64) as i32;
            let mut stepmul: i32 = luaL_optinteger(L, 3i32, 0i32 as i64) as i32;
            let mut stepsize: i32 = luaL_optinteger(L, 4i32, 0i32 as i64) as i32;
            return pushmode(L, lua_gc(L, o, pause, stepmul, stepsize));
        }
        _ => {
            let mut res_1: i32 = lua_gc(L, o);
            if !(res_1 == -(1i32)) {
                lua_pushinteger(L, res_1 as i64);
                return 1i32;
            }
        }
    }
    lua_pushnil(L);
    return 1i32;
}
unsafe extern "C" fn luaB_type(mut L: *mut lua_State) -> i32 {
    let mut t: i32 = lua_type(L, 1i32);
    (((t != -(1i32)) as i32 != 0i32) as i32 as i64 != 0
        || luaL_argerror(
            L,
            1i32,
            b"value expected\0" as *const u8 as *const libc::c_char,
        ) != 0) as i32;
    lua_pushstring(L, lua_typename(L, t));
    return 1i32;
}
unsafe extern "C" fn luaB_next(mut L: *mut lua_State) -> i32 {
    luaL_checktype(L, 1i32, 5i32);
    lua_settop(L, 2i32);
    if lua_next(L, 1i32) != 0 {
        return 2i32;
    } else {
        lua_pushnil(L);
        return 1i32;
    };
}
unsafe extern "C" fn pairscont(
    mut _L: *mut lua_State,
    mut _status: i32,
    mut _k: lua_KContext,
) -> i32 {
    return 3i32;
}
unsafe extern "C" fn luaB_pairs(mut L: *mut lua_State) -> i32 {
    luaL_checkany(L, 1i32);
    if luaL_getmetafield(L, 1i32, b"__pairs\0" as *const u8 as *const libc::c_char) == 0i32 {
        lua_pushcclosure(
            L,
            Some(luaB_next as unsafe extern "C" fn(*mut lua_State) -> i32),
            0i32,
        );
        lua_pushvalue(L, 1i32);
        lua_pushnil(L);
    } else {
        lua_pushvalue(L, 1i32);
        lua_callk(
            L,
            1i32,
            3i32,
            0i32 as lua_KContext,
            Some(pairscont as unsafe extern "C" fn(*mut lua_State, i32, lua_KContext) -> i32),
        );
    }
    return 3i32;
}
unsafe extern "C" fn ipairsaux(mut L: *mut lua_State) -> i32 {
    let mut i: i64 = luaL_checkinteger(L, 2i32);
    i = (i as u64).wrapping_add(1i32 as u64) as i64;
    lua_pushinteger(L, i);
    return if lua_geti(L, 1i32, i) == 0i32 {
        1i32
    } else {
        2i32
    };
}
unsafe extern "C" fn luaB_ipairs(mut L: *mut lua_State) -> i32 {
    luaL_checkany(L, 1i32);
    lua_pushcclosure(
        L,
        Some(ipairsaux as unsafe extern "C" fn(*mut lua_State) -> i32),
        0i32,
    );
    lua_pushvalue(L, 1i32);
    lua_pushinteger(L, 0i32 as i64);
    return 3i32;
}
unsafe extern "C" fn load_aux(mut L: *mut lua_State, mut status: i32, mut envidx: i32) -> i32 {
    if ((status == 0i32) as i32 != 0i32) as i32 as i64 != 0 {
        if envidx != 0i32 {
            lua_pushvalue(L, envidx);
            if (lua_setupvalue(L, -(2i32), 1i32)).is_null() {
                lua_settop(L, -(1i32) - 1i32);
            }
        }
        return 1i32;
    } else {
        lua_pushnil(L);
        lua_rotate(L, -(2i32), 1i32);
        return 2i32;
    };
}
unsafe extern "C" fn luaB_loadfile(mut L: *mut lua_State) -> i32 {
    let mut fname: *const libc::c_char =
        luaL_optlstring(L, 1i32, 0 as *const libc::c_char, 0 as *mut u64);
    let mut mode: *const libc::c_char =
        luaL_optlstring(L, 2i32, 0 as *const libc::c_char, 0 as *mut u64);
    let mut env: i32 = if !(lua_type(L, 3i32) == -(1i32)) {
        3i32
    } else {
        0i32
    };
    let mut status: i32 = luaL_loadfilex(L, fname, mode);
    return load_aux(L, status, env);
}
unsafe extern "C" fn generic_reader(
    mut L: *mut lua_State,
    mut _ud: *mut libc::c_void,
    mut size: *mut u64,
) -> *const libc::c_char {
    luaL_checkstack(
        L,
        2i32,
        b"too many nested functions\0" as *const u8 as *const libc::c_char,
    );
    lua_pushvalue(L, 1i32);
    lua_callk(L, 0i32, 1i32, 0i32 as lua_KContext, None);
    if lua_type(L, -(1i32)) == 0i32 {
        lua_settop(L, -(1i32) - 1i32);
        *size = 0i32 as u64;
        return 0 as *const libc::c_char;
    } else if ((lua_isstring(L, -(1i32)) == 0) as i32 != 0i32) as i32 as i64 != 0 {
        luaL_error(
            L,
            b"reader function must return a string\0" as *const u8 as *const libc::c_char,
        );
    }
    lua_copy(L, -(1i32), 5i32);
    lua_settop(L, -(1i32) - 1i32);
    return lua_tolstring(L, 5i32, size);
}
unsafe extern "C" fn luaB_load(mut L: *mut lua_State) -> i32 {
    let mut status: i32 = 0;
    let mut l: u64 = 0;
    let mut s: *const libc::c_char = lua_tolstring(L, 1i32, &mut l);
    let mut mode: *const libc::c_char = luaL_optlstring(
        L,
        3i32,
        b"bt\0" as *const u8 as *const libc::c_char,
        0 as *mut u64,
    );
    let mut env: i32 = if !(lua_type(L, 4i32) == -(1i32)) {
        4i32
    } else {
        0i32
    };
    if !s.is_null() {
        let mut chunkname: *const libc::c_char = luaL_optlstring(L, 2i32, s, 0 as *mut u64);
        status = luaL_loadbufferx(L, s, l, chunkname, mode);
    } else {
        let mut chunkname_0: *const libc::c_char = luaL_optlstring(
            L,
            2i32,
            b"=(load)\0" as *const u8 as *const libc::c_char,
            0 as *mut u64,
        );
        luaL_checktype(L, 1i32, 6i32);
        lua_settop(L, 5i32);
        status = lua_load(
            L,
            Some(
                generic_reader
                    as unsafe extern "C" fn(
                        *mut lua_State,
                        *mut libc::c_void,
                        *mut u64,
                    ) -> *const libc::c_char,
            ),
            0 as *mut libc::c_void,
            chunkname_0,
            mode,
        );
    }
    return load_aux(L, status, env);
}
unsafe extern "C" fn dofilecont(mut L: *mut lua_State, mut _d1: i32, mut _d2: lua_KContext) -> i32 {
    return lua_gettop(L) - 1i32;
}
unsafe extern "C" fn luaB_dofile(mut L: *mut lua_State) -> i32 {
    let mut fname: *const libc::c_char =
        luaL_optlstring(L, 1i32, 0 as *const libc::c_char, 0 as *mut u64);
    lua_settop(L, 1i32);
    if ((luaL_loadfilex(L, fname, 0 as *const libc::c_char) != 0i32) as i32 != 0i32) as i32 as i64
        != 0
    {
        return lua_error(L);
    }
    lua_callk(
        L,
        0i32,
        -(1i32),
        0i32 as lua_KContext,
        Some(dofilecont as unsafe extern "C" fn(*mut lua_State, i32, lua_KContext) -> i32),
    );
    return dofilecont(L, 0i32, 0i32 as lua_KContext);
}
unsafe extern "C" fn luaB_assert(mut L: *mut lua_State) -> i32 {
    if (lua_toboolean(L, 1i32) != 0i32) as i32 as i64 != 0 {
        return lua_gettop(L);
    } else {
        luaL_checkany(L, 1i32);
        lua_rotate(L, 1i32, -(1i32));
        lua_settop(L, -(1i32) - 1i32);
        lua_pushstring(
            L,
            b"assertion failed!\0" as *const u8 as *const libc::c_char,
        );
        lua_settop(L, 1i32);
        return luaB_error(L);
    };
}
unsafe extern "C" fn luaB_select(mut L: *mut lua_State) -> i32 {
    let mut n: i32 = lua_gettop(L);
    if lua_type(L, 1i32) == 4i32 && *lua_tolstring(L, 1i32, 0 as *mut u64) as i32 == '#' as i32 {
        lua_pushinteger(L, (n - 1i32) as i64);
        return 1i32;
    } else {
        let mut i: i64 = luaL_checkinteger(L, 1i32);
        if i < 0i32 as i64 {
            i = n as i64 + i;
        } else if i > n as i64 {
            i = n as i64;
        }
        (((1i32 as i64 <= i) as i32 != 0i32) as i32 as i64 != 0
            || luaL_argerror(
                L,
                1i32,
                b"index out of range\0" as *const u8 as *const libc::c_char,
            ) != 0) as i32;
        return n - i as i32;
    };
}
unsafe extern "C" fn finishpcall(
    mut L: *mut lua_State,
    mut status: i32,
    mut extra: lua_KContext,
) -> i32 {
    if ((status != 0i32 && status != 1i32) as i32 != 0i32) as i32 as i64 != 0 {
        lua_pushboolean(L, 0i32);
        lua_pushvalue(L, -(2i32));
        return 2i32;
    } else {
        return lua_gettop(L) - extra as i32;
    };
}
unsafe extern "C" fn luaB_pcall(mut L: *mut lua_State) -> i32 {
    let mut status: i32 = 0;
    luaL_checkany(L, 1i32);
    lua_pushboolean(L, 1i32);
    lua_rotate(L, 1i32, 1i32);
    status = lua_pcallk(
        L,
        lua_gettop(L) - 2i32,
        -(1i32),
        0i32,
        0i32 as lua_KContext,
        Some(finishpcall as unsafe extern "C" fn(*mut lua_State, i32, lua_KContext) -> i32),
    );
    return finishpcall(L, status, 0i32 as lua_KContext);
}
unsafe extern "C" fn luaB_xpcall(mut L: *mut lua_State) -> i32 {
    let mut status: i32 = 0;
    let mut n: i32 = lua_gettop(L);
    luaL_checktype(L, 2i32, 6i32);
    lua_pushboolean(L, 1i32);
    lua_pushvalue(L, 1i32);
    lua_rotate(L, 3i32, 2i32);
    status = lua_pcallk(
        L,
        n - 2i32,
        -(1i32),
        2i32,
        2i32 as lua_KContext,
        Some(finishpcall as unsafe extern "C" fn(*mut lua_State, i32, lua_KContext) -> i32),
    );
    return finishpcall(L, status, 2i32 as lua_KContext);
}
unsafe extern "C" fn luaB_tostring(mut L: *mut lua_State) -> i32 {
    luaL_checkany(L, 1i32);
    luaL_tolstring(L, 1i32, 0 as *mut u64);
    return 1i32;
}
static mut base_funcs: [luaL_Reg; 26] = {
    [
        {
            let mut init = luaL_Reg {
                name: b"assert\0" as *const u8 as *const libc::c_char,
                func: Some(luaB_assert as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"collectgarbage\0" as *const u8 as *const libc::c_char,
                func: Some(luaB_collectgarbage as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"dofile\0" as *const u8 as *const libc::c_char,
                func: Some(luaB_dofile as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"error\0" as *const u8 as *const libc::c_char,
                func: Some(luaB_error as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"getmetatable\0" as *const u8 as *const libc::c_char,
                func: Some(luaB_getmetatable as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"ipairs\0" as *const u8 as *const libc::c_char,
                func: Some(luaB_ipairs as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"loadfile\0" as *const u8 as *const libc::c_char,
                func: Some(luaB_loadfile as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"load\0" as *const u8 as *const libc::c_char,
                func: Some(luaB_load as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"next\0" as *const u8 as *const libc::c_char,
                func: Some(luaB_next as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"pairs\0" as *const u8 as *const libc::c_char,
                func: Some(luaB_pairs as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"pcall\0" as *const u8 as *const libc::c_char,
                func: Some(luaB_pcall as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"print\0" as *const u8 as *const libc::c_char,
                func: Some(luaB_print as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"warn\0" as *const u8 as *const libc::c_char,
                func: Some(luaB_warn as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"rawequal\0" as *const u8 as *const libc::c_char,
                func: Some(luaB_rawequal as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"rawlen\0" as *const u8 as *const libc::c_char,
                func: Some(luaB_rawlen as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"rawget\0" as *const u8 as *const libc::c_char,
                func: Some(luaB_rawget as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"rawset\0" as *const u8 as *const libc::c_char,
                func: Some(luaB_rawset as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"select\0" as *const u8 as *const libc::c_char,
                func: Some(luaB_select as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"setmetatable\0" as *const u8 as *const libc::c_char,
                func: Some(luaB_setmetatable as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"tonumber\0" as *const u8 as *const libc::c_char,
                func: Some(luaB_tonumber as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"tostring\0" as *const u8 as *const libc::c_char,
                func: Some(luaB_tostring as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"type\0" as *const u8 as *const libc::c_char,
                func: Some(luaB_type as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"xpcall\0" as *const u8 as *const libc::c_char,
                func: Some(luaB_xpcall as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"_G\0" as *const u8 as *const libc::c_char,
                func: None,
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"_VERSION\0" as *const u8 as *const libc::c_char,
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaopen_base(mut L: *mut lua_State) -> i32 {
    lua_rawgeti(L, -(1000000i32) - 1000i32, 2i32 as i64);
    luaL_setfuncs(L, base_funcs.as_ptr(), 0i32);
    lua_pushvalue(L, -(1i32));
    lua_setfield(L, -(2i32), b"_G\0" as *const u8 as *const libc::c_char);
    lua_pushstring(L, b"Lua 5.4\0" as *const u8 as *const libc::c_char);
    lua_setfield(
        L,
        -(2i32),
        b"_VERSION\0" as *const u8 as *const libc::c_char,
    );
    return 1i32;
}
