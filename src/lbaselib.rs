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
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type lua_State;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    static mut stdout: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn lua_gettop(L: *mut lua_State) -> libc::c_int;
    fn lua_settop(L: *mut lua_State, idx: libc::c_int);
    fn lua_pushvalue(L: *mut lua_State, idx: libc::c_int);
    fn lua_rotate(L: *mut lua_State, idx: libc::c_int, n: libc::c_int);
    fn lua_copy(L: *mut lua_State, fromidx: libc::c_int, toidx: libc::c_int);
    fn lua_isstring(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    fn lua_type(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    fn lua_typename(L: *mut lua_State, tp: libc::c_int) -> *const libc::c_char;
    fn lua_toboolean(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    fn lua_tolstring(
        L: *mut lua_State,
        idx: libc::c_int,
        len: *mut size_t,
    ) -> *const libc::c_char;
    fn lua_rawlen(L: *mut lua_State, idx: libc::c_int) -> lua_Unsigned;
    fn lua_rawequal(
        L: *mut lua_State,
        idx1: libc::c_int,
        idx2: libc::c_int,
    ) -> libc::c_int;
    fn lua_pushnil(L: *mut lua_State);
    fn lua_pushnumber(L: *mut lua_State, n: lua_Number);
    fn lua_pushinteger(L: *mut lua_State, n: lua_Integer);
    fn lua_pushstring(L: *mut lua_State, s: *const libc::c_char) -> *const libc::c_char;
    fn lua_pushcclosure(L: *mut lua_State, fn_0: lua_CFunction, n: libc::c_int);
    fn lua_pushboolean(L: *mut lua_State, b: libc::c_int);
    fn lua_geti(L: *mut lua_State, idx: libc::c_int, n: lua_Integer) -> libc::c_int;
    fn lua_rawget(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    fn lua_rawgeti(L: *mut lua_State, idx: libc::c_int, n: lua_Integer) -> libc::c_int;
    fn lua_getmetatable(L: *mut lua_State, objindex: libc::c_int) -> libc::c_int;
    fn lua_setfield(L: *mut lua_State, idx: libc::c_int, k: *const libc::c_char);
    fn lua_rawset(L: *mut lua_State, idx: libc::c_int);
    fn lua_setmetatable(L: *mut lua_State, objindex: libc::c_int) -> libc::c_int;
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
    fn lua_load(
        L: *mut lua_State,
        reader: lua_Reader,
        dt: *mut libc::c_void,
        chunkname: *const libc::c_char,
        mode: *const libc::c_char,
    ) -> libc::c_int;
    fn lua_warning(L: *mut lua_State, msg: *const libc::c_char, tocont: libc::c_int);
    fn lua_gc(L: *mut lua_State, what: libc::c_int, _: ...) -> libc::c_int;
    fn lua_error(L: *mut lua_State) -> libc::c_int;
    fn lua_next(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    fn lua_concat(L: *mut lua_State, n: libc::c_int);
    fn lua_stringtonumber(L: *mut lua_State, s: *const libc::c_char) -> size_t;
    fn lua_setupvalue(
        L: *mut lua_State,
        funcindex: libc::c_int,
        n: libc::c_int,
    ) -> *const libc::c_char;
    fn luaL_getmetafield(
        L: *mut lua_State,
        obj: libc::c_int,
        e: *const libc::c_char,
    ) -> libc::c_int;
    fn luaL_tolstring(
        L: *mut lua_State,
        idx: libc::c_int,
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
    fn luaL_checkinteger(L: *mut lua_State, arg: libc::c_int) -> lua_Integer;
    fn luaL_optinteger(
        L: *mut lua_State,
        arg: libc::c_int,
        def: lua_Integer,
    ) -> lua_Integer;
    fn luaL_checkstack(L: *mut lua_State, sz: libc::c_int, msg: *const libc::c_char);
    fn luaL_checktype(L: *mut lua_State, arg: libc::c_int, t: libc::c_int);
    fn luaL_checkany(L: *mut lua_State, arg: libc::c_int);
    fn luaL_where(L: *mut lua_State, lvl: libc::c_int);
    fn luaL_error(L: *mut lua_State, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn luaL_checkoption(
        L: *mut lua_State,
        arg: libc::c_int,
        def: *const libc::c_char,
        lst: *const *const libc::c_char,
    ) -> libc::c_int;
    fn luaL_loadfilex(
        L: *mut lua_State,
        filename: *const libc::c_char,
        mode: *const libc::c_char,
    ) -> libc::c_int;
    fn luaL_loadbufferx(
        L: *mut lua_State,
        buff: *const libc::c_char,
        sz: size_t,
        name: *const libc::c_char,
        mode: *const libc::c_char,
    ) -> libc::c_int;
    fn luaL_setfuncs(L: *mut lua_State, l: *const luaL_Reg, nup: libc::c_int);
}
pub type __int32_t = libc::c_int;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type C2RustUnnamed = libc::c_uint;
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
pub type size_t = libc::c_ulong;
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
pub type lua_Number = libc::c_double;
pub type lua_Integer = libc::c_longlong;
pub type lua_Unsigned = libc::c_ulonglong;
pub type lua_KContext = intptr_t;
pub type lua_CFunction = Option::<unsafe extern "C" fn(*mut lua_State) -> libc::c_int>;
pub type lua_KFunction = Option::<
    unsafe extern "C" fn(*mut lua_State, libc::c_int, lua_KContext) -> libc::c_int,
>;
pub type lua_Reader = Option::<
    unsafe extern "C" fn(
        *mut lua_State,
        *mut libc::c_void,
        *mut size_t,
    ) -> *const libc::c_char,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaL_Reg {
    pub name: *const libc::c_char,
    pub func: lua_CFunction,
}
#[inline]
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
unsafe extern "C" fn luaB_print(mut L: *mut lua_State) -> libc::c_int {
    let mut n: libc::c_int = lua_gettop(L);
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i <= n {
        let mut l: size_t = 0;
        let mut s: *const libc::c_char = luaL_tolstring(L, i, &mut l);
        if i > 1 as libc::c_int {
            fwrite(
                b"\t\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                1 as libc::c_int as libc::c_ulong,
                stdout,
            );
        }
        fwrite(
            s as *const libc::c_void,
            ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
            l,
            stdout,
        );
        lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
        i += 1;
        i;
    }
    fwrite(
        b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stdout,
    );
    fflush(stdout);
    return 0 as libc::c_int;
}
unsafe extern "C" fn luaB_warn(mut L: *mut lua_State) -> libc::c_int {
    let mut n: libc::c_int = lua_gettop(L);
    let mut i: libc::c_int = 0;
    luaL_checklstring(L, 1 as libc::c_int, 0 as *mut size_t);
    i = 2 as libc::c_int;
    while i <= n {
        luaL_checklstring(L, i, 0 as *mut size_t);
        i += 1;
        i;
    }
    i = 1 as libc::c_int;
    while i < n {
        lua_warning(L, lua_tolstring(L, i, 0 as *mut size_t), 1 as libc::c_int);
        i += 1;
        i;
    }
    lua_warning(L, lua_tolstring(L, n, 0 as *mut size_t), 0 as libc::c_int);
    return 0 as libc::c_int;
}
unsafe extern "C" fn b_str2int(
    mut s: *const libc::c_char,
    mut base: libc::c_int,
    mut pn: *mut lua_Integer,
) -> *const libc::c_char {
    let mut n: lua_Unsigned = 0 as libc::c_int as lua_Unsigned;
    let mut neg: libc::c_int = 0 as libc::c_int;
    s = s
        .offset(
            strspn(s, b" \x0C\n\r\t\x0B\0" as *const u8 as *const libc::c_char) as isize,
        );
    if *s as libc::c_int == '-' as i32 {
        s = s.offset(1);
        s;
        neg = 1 as libc::c_int;
    } else if *s as libc::c_int == '+' as i32 {
        s = s.offset(1);
        s;
    }
    if *(*__ctype_b_loc()).offset(*s as libc::c_uchar as libc::c_int as isize)
        as libc::c_int & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int == 0
    {
        return 0 as *const libc::c_char;
    }
    loop {
        let mut digit: libc::c_int = if *(*__ctype_b_loc())
            .offset(*s as libc::c_uchar as libc::c_int as isize) as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            *s as libc::c_int - '0' as i32
        } else {
            ({
                let mut __res: libc::c_int = 0;
                if ::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: libc::c_int = *s as libc::c_uchar as libc::c_int;
                        __res = (if __c < -(128 as libc::c_int)
                            || __c > 255 as libc::c_int
                        {
                            __c
                        } else {
                            *(*__ctype_toupper_loc()).offset(__c as isize)
                        });
                    } else {
                        __res = toupper(*s as libc::c_uchar as libc::c_int);
                    }
                } else {
                    __res = *(*__ctype_toupper_loc())
                        .offset(*s as libc::c_uchar as libc::c_int as isize);
                }
                __res
            }) - 'A' as i32 + 10 as libc::c_int
        };
        if digit >= base {
            return 0 as *const libc::c_char;
        }
        n = n
            .wrapping_mul(base as libc::c_ulonglong)
            .wrapping_add(digit as libc::c_ulonglong);
        s = s.offset(1);
        s;
        if !(*(*__ctype_b_loc()).offset(*s as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int
            != 0)
        {
            break;
        }
    }
    s = s
        .offset(
            strspn(s, b" \x0C\n\r\t\x0B\0" as *const u8 as *const libc::c_char) as isize,
        );
    *pn = (if neg != 0 {
        (0 as libc::c_uint as libc::c_ulonglong).wrapping_sub(n)
    } else {
        n
    }) as lua_Integer;
    return s;
}
unsafe extern "C" fn luaB_tonumber(mut L: *mut lua_State) -> libc::c_int {
    if lua_type(L, 2 as libc::c_int) <= 0 as libc::c_int {
        if lua_type(L, 1 as libc::c_int) == 3 as libc::c_int {
            lua_settop(L, 1 as libc::c_int);
            return 1 as libc::c_int;
        } else {
            let mut l: size_t = 0;
            let mut s: *const libc::c_char = lua_tolstring(L, 1 as libc::c_int, &mut l);
            if !s.is_null()
                && lua_stringtonumber(L, s)
                    == l.wrapping_add(1 as libc::c_int as libc::c_ulong)
            {
                return 1 as libc::c_int;
            }
            luaL_checkany(L, 1 as libc::c_int);
        }
    } else {
        let mut l_0: size_t = 0;
        let mut s_0: *const libc::c_char = 0 as *const libc::c_char;
        let mut n: lua_Integer = 0 as libc::c_int as lua_Integer;
        let mut base: lua_Integer = luaL_checkinteger(L, 2 as libc::c_int);
        luaL_checktype(L, 1 as libc::c_int, 4 as libc::c_int);
        s_0 = lua_tolstring(L, 1 as libc::c_int, &mut l_0);
        (((2 as libc::c_int as libc::c_longlong <= base
            && base <= 36 as libc::c_int as libc::c_longlong) as libc::c_int
            != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
            || luaL_argerror(
                L,
                2 as libc::c_int,
                b"base out of range\0" as *const u8 as *const libc::c_char,
            ) != 0) as libc::c_int;
        if b_str2int(s_0, base as libc::c_int, &mut n) == s_0.offset(l_0 as isize) {
            lua_pushinteger(L, n);
            return 1 as libc::c_int;
        }
    }
    lua_pushnil(L);
    return 1 as libc::c_int;
}
unsafe extern "C" fn luaB_error(mut L: *mut lua_State) -> libc::c_int {
    let mut level: libc::c_int = luaL_optinteger(
        L,
        2 as libc::c_int,
        1 as libc::c_int as lua_Integer,
    ) as libc::c_int;
    lua_settop(L, 1 as libc::c_int);
    if lua_type(L, 1 as libc::c_int) == 4 as libc::c_int && level > 0 as libc::c_int {
        luaL_where(L, level);
        lua_pushvalue(L, 1 as libc::c_int);
        lua_concat(L, 2 as libc::c_int);
    }
    return lua_error(L);
}
unsafe extern "C" fn luaB_getmetatable(mut L: *mut lua_State) -> libc::c_int {
    luaL_checkany(L, 1 as libc::c_int);
    if lua_getmetatable(L, 1 as libc::c_int) == 0 {
        lua_pushnil(L);
        return 1 as libc::c_int;
    }
    luaL_getmetafield(
        L,
        1 as libc::c_int,
        b"__metatable\0" as *const u8 as *const libc::c_char,
    );
    return 1 as libc::c_int;
}
unsafe extern "C" fn luaB_setmetatable(mut L: *mut lua_State) -> libc::c_int {
    let mut t: libc::c_int = lua_type(L, 2 as libc::c_int);
    luaL_checktype(L, 1 as libc::c_int, 5 as libc::c_int);
    (((t == 0 as libc::c_int || t == 5 as libc::c_int) as libc::c_int
        != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
        || luaL_typeerror(
            L,
            2 as libc::c_int,
            b"nil or table\0" as *const u8 as *const libc::c_char,
        ) != 0) as libc::c_int;
    if ((luaL_getmetafield(
        L,
        1 as libc::c_int,
        b"__metatable\0" as *const u8 as *const libc::c_char,
    ) != 0 as libc::c_int) as libc::c_int != 0 as libc::c_int) as libc::c_int
        as libc::c_long != 0
    {
        return luaL_error(
            L,
            b"cannot change a protected metatable\0" as *const u8 as *const libc::c_char,
        );
    }
    lua_settop(L, 2 as libc::c_int);
    lua_setmetatable(L, 1 as libc::c_int);
    return 1 as libc::c_int;
}
unsafe extern "C" fn luaB_rawequal(mut L: *mut lua_State) -> libc::c_int {
    luaL_checkany(L, 1 as libc::c_int);
    luaL_checkany(L, 2 as libc::c_int);
    lua_pushboolean(L, lua_rawequal(L, 1 as libc::c_int, 2 as libc::c_int));
    return 1 as libc::c_int;
}
unsafe extern "C" fn luaB_rawlen(mut L: *mut lua_State) -> libc::c_int {
    let mut t: libc::c_int = lua_type(L, 1 as libc::c_int);
    (((t == 5 as libc::c_int || t == 4 as libc::c_int) as libc::c_int
        != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
        || luaL_typeerror(
            L,
            1 as libc::c_int,
            b"table or string\0" as *const u8 as *const libc::c_char,
        ) != 0) as libc::c_int;
    lua_pushinteger(L, lua_rawlen(L, 1 as libc::c_int) as lua_Integer);
    return 1 as libc::c_int;
}
unsafe extern "C" fn luaB_rawget(mut L: *mut lua_State) -> libc::c_int {
    luaL_checktype(L, 1 as libc::c_int, 5 as libc::c_int);
    luaL_checkany(L, 2 as libc::c_int);
    lua_settop(L, 2 as libc::c_int);
    lua_rawget(L, 1 as libc::c_int);
    return 1 as libc::c_int;
}
unsafe extern "C" fn luaB_rawset(mut L: *mut lua_State) -> libc::c_int {
    luaL_checktype(L, 1 as libc::c_int, 5 as libc::c_int);
    luaL_checkany(L, 2 as libc::c_int);
    luaL_checkany(L, 3 as libc::c_int);
    lua_settop(L, 3 as libc::c_int);
    lua_rawset(L, 1 as libc::c_int);
    return 1 as libc::c_int;
}
unsafe extern "C" fn pushmode(
    mut L: *mut lua_State,
    mut oldmode: libc::c_int,
) -> libc::c_int {
    if oldmode == -(1 as libc::c_int) {
        lua_pushnil(L);
    } else {
        lua_pushstring(
            L,
            if oldmode == 11 as libc::c_int {
                b"incremental\0" as *const u8 as *const libc::c_char
            } else {
                b"generational\0" as *const u8 as *const libc::c_char
            },
        );
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn luaB_collectgarbage(mut L: *mut lua_State) -> libc::c_int {
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
    static mut optsnum: [libc::c_int; 10] = [
        0 as libc::c_int,
        1 as libc::c_int,
        2 as libc::c_int,
        3 as libc::c_int,
        5 as libc::c_int,
        6 as libc::c_int,
        7 as libc::c_int,
        9 as libc::c_int,
        10 as libc::c_int,
        11 as libc::c_int,
    ];
    let mut o: libc::c_int = optsnum[luaL_checkoption(
        L,
        1 as libc::c_int,
        b"collect\0" as *const u8 as *const libc::c_char,
        opts.as_ptr(),
    ) as usize];
    match o {
        3 => {
            let mut k: libc::c_int = lua_gc(L, o);
            let mut b: libc::c_int = lua_gc(L, 4 as libc::c_int);
            if !(k == -(1 as libc::c_int)) {
                lua_pushnumber(
                    L,
                    k as lua_Number
                        + b as lua_Number / 1024 as libc::c_int as libc::c_double,
                );
                return 1 as libc::c_int;
            }
        }
        5 => {
            let mut step: libc::c_int = luaL_optinteger(
                L,
                2 as libc::c_int,
                0 as libc::c_int as lua_Integer,
            ) as libc::c_int;
            let mut res: libc::c_int = lua_gc(L, o, step);
            if !(res == -(1 as libc::c_int)) {
                lua_pushboolean(L, res);
                return 1 as libc::c_int;
            }
        }
        6 | 7 => {
            let mut p: libc::c_int = luaL_optinteger(
                L,
                2 as libc::c_int,
                0 as libc::c_int as lua_Integer,
            ) as libc::c_int;
            let mut previous: libc::c_int = lua_gc(L, o, p);
            if !(previous == -(1 as libc::c_int)) {
                lua_pushinteger(L, previous as lua_Integer);
                return 1 as libc::c_int;
            }
        }
        9 => {
            let mut res_0: libc::c_int = lua_gc(L, o);
            if !(res_0 == -(1 as libc::c_int)) {
                lua_pushboolean(L, res_0);
                return 1 as libc::c_int;
            }
        }
        10 => {
            let mut minormul: libc::c_int = luaL_optinteger(
                L,
                2 as libc::c_int,
                0 as libc::c_int as lua_Integer,
            ) as libc::c_int;
            let mut majormul: libc::c_int = luaL_optinteger(
                L,
                3 as libc::c_int,
                0 as libc::c_int as lua_Integer,
            ) as libc::c_int;
            return pushmode(L, lua_gc(L, o, minormul, majormul));
        }
        11 => {
            let mut pause: libc::c_int = luaL_optinteger(
                L,
                2 as libc::c_int,
                0 as libc::c_int as lua_Integer,
            ) as libc::c_int;
            let mut stepmul: libc::c_int = luaL_optinteger(
                L,
                3 as libc::c_int,
                0 as libc::c_int as lua_Integer,
            ) as libc::c_int;
            let mut stepsize: libc::c_int = luaL_optinteger(
                L,
                4 as libc::c_int,
                0 as libc::c_int as lua_Integer,
            ) as libc::c_int;
            return pushmode(L, lua_gc(L, o, pause, stepmul, stepsize));
        }
        _ => {
            let mut res_1: libc::c_int = lua_gc(L, o);
            if !(res_1 == -(1 as libc::c_int)) {
                lua_pushinteger(L, res_1 as lua_Integer);
                return 1 as libc::c_int;
            }
        }
    }
    lua_pushnil(L);
    return 1 as libc::c_int;
}
unsafe extern "C" fn luaB_type(mut L: *mut lua_State) -> libc::c_int {
    let mut t: libc::c_int = lua_type(L, 1 as libc::c_int);
    (((t != -(1 as libc::c_int)) as libc::c_int != 0 as libc::c_int) as libc::c_int
        as libc::c_long != 0
        || luaL_argerror(
            L,
            1 as libc::c_int,
            b"value expected\0" as *const u8 as *const libc::c_char,
        ) != 0) as libc::c_int;
    lua_pushstring(L, lua_typename(L, t));
    return 1 as libc::c_int;
}
unsafe extern "C" fn luaB_next(mut L: *mut lua_State) -> libc::c_int {
    luaL_checktype(L, 1 as libc::c_int, 5 as libc::c_int);
    lua_settop(L, 2 as libc::c_int);
    if lua_next(L, 1 as libc::c_int) != 0 {
        return 2 as libc::c_int
    } else {
        lua_pushnil(L);
        return 1 as libc::c_int;
    };
}
unsafe extern "C" fn pairscont(
    mut L: *mut lua_State,
    mut status: libc::c_int,
    mut k: lua_KContext,
) -> libc::c_int {
    return 3 as libc::c_int;
}
unsafe extern "C" fn luaB_pairs(mut L: *mut lua_State) -> libc::c_int {
    luaL_checkany(L, 1 as libc::c_int);
    if luaL_getmetafield(
        L,
        1 as libc::c_int,
        b"__pairs\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        lua_pushcclosure(
            L,
            Some(luaB_next as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
            0 as libc::c_int,
        );
        lua_pushvalue(L, 1 as libc::c_int);
        lua_pushnil(L);
    } else {
        lua_pushvalue(L, 1 as libc::c_int);
        lua_callk(
            L,
            1 as libc::c_int,
            3 as libc::c_int,
            0 as libc::c_int as lua_KContext,
            Some(
                pairscont
                    as unsafe extern "C" fn(
                        *mut lua_State,
                        libc::c_int,
                        lua_KContext,
                    ) -> libc::c_int,
            ),
        );
    }
    return 3 as libc::c_int;
}
unsafe extern "C" fn ipairsaux(mut L: *mut lua_State) -> libc::c_int {
    let mut i: lua_Integer = luaL_checkinteger(L, 2 as libc::c_int);
    i = (i as lua_Unsigned).wrapping_add(1 as libc::c_int as lua_Unsigned)
        as lua_Integer;
    lua_pushinteger(L, i);
    return if lua_geti(L, 1 as libc::c_int, i) == 0 as libc::c_int {
        1 as libc::c_int
    } else {
        2 as libc::c_int
    };
}
unsafe extern "C" fn luaB_ipairs(mut L: *mut lua_State) -> libc::c_int {
    luaL_checkany(L, 1 as libc::c_int);
    lua_pushcclosure(
        L,
        Some(ipairsaux as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
        0 as libc::c_int,
    );
    lua_pushvalue(L, 1 as libc::c_int);
    lua_pushinteger(L, 0 as libc::c_int as lua_Integer);
    return 3 as libc::c_int;
}
unsafe extern "C" fn load_aux(
    mut L: *mut lua_State,
    mut status: libc::c_int,
    mut envidx: libc::c_int,
) -> libc::c_int {
    if ((status == 0 as libc::c_int) as libc::c_int != 0 as libc::c_int) as libc::c_int
        as libc::c_long != 0
    {
        if envidx != 0 as libc::c_int {
            lua_pushvalue(L, envidx);
            if (lua_setupvalue(L, -(2 as libc::c_int), 1 as libc::c_int)).is_null() {
                lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
            }
        }
        return 1 as libc::c_int;
    } else {
        lua_pushnil(L);
        lua_rotate(L, -(2 as libc::c_int), 1 as libc::c_int);
        return 2 as libc::c_int;
    };
}
unsafe extern "C" fn luaB_loadfile(mut L: *mut lua_State) -> libc::c_int {
    let mut fname: *const libc::c_char = luaL_optlstring(
        L,
        1 as libc::c_int,
        0 as *const libc::c_char,
        0 as *mut size_t,
    );
    let mut mode: *const libc::c_char = luaL_optlstring(
        L,
        2 as libc::c_int,
        0 as *const libc::c_char,
        0 as *mut size_t,
    );
    let mut env: libc::c_int = if !(lua_type(L, 3 as libc::c_int) == -(1 as libc::c_int))
    {
        3 as libc::c_int
    } else {
        0 as libc::c_int
    };
    let mut status: libc::c_int = luaL_loadfilex(L, fname, mode);
    return load_aux(L, status, env);
}
unsafe extern "C" fn generic_reader(
    mut L: *mut lua_State,
    mut ud: *mut libc::c_void,
    mut size: *mut size_t,
) -> *const libc::c_char {
    luaL_checkstack(
        L,
        2 as libc::c_int,
        b"too many nested functions\0" as *const u8 as *const libc::c_char,
    );
    lua_pushvalue(L, 1 as libc::c_int);
    lua_callk(
        L,
        0 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int as lua_KContext,
        None,
    );
    if lua_type(L, -(1 as libc::c_int)) == 0 as libc::c_int {
        lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
        *size = 0 as libc::c_int as size_t;
        return 0 as *const libc::c_char;
    } else if ((lua_isstring(L, -(1 as libc::c_int)) == 0) as libc::c_int
        != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
    {
        luaL_error(
            L,
            b"reader function must return a string\0" as *const u8 as *const libc::c_char,
        );
    }
    lua_copy(L, -(1 as libc::c_int), 5 as libc::c_int);
    lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
    return lua_tolstring(L, 5 as libc::c_int, size);
}
unsafe extern "C" fn luaB_load(mut L: *mut lua_State) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut l: size_t = 0;
    let mut s: *const libc::c_char = lua_tolstring(L, 1 as libc::c_int, &mut l);
    let mut mode: *const libc::c_char = luaL_optlstring(
        L,
        3 as libc::c_int,
        b"bt\0" as *const u8 as *const libc::c_char,
        0 as *mut size_t,
    );
    let mut env: libc::c_int = if !(lua_type(L, 4 as libc::c_int) == -(1 as libc::c_int))
    {
        4 as libc::c_int
    } else {
        0 as libc::c_int
    };
    if !s.is_null() {
        let mut chunkname: *const libc::c_char = luaL_optlstring(
            L,
            2 as libc::c_int,
            s,
            0 as *mut size_t,
        );
        status = luaL_loadbufferx(L, s, l, chunkname, mode);
    } else {
        let mut chunkname_0: *const libc::c_char = luaL_optlstring(
            L,
            2 as libc::c_int,
            b"=(load)\0" as *const u8 as *const libc::c_char,
            0 as *mut size_t,
        );
        luaL_checktype(L, 1 as libc::c_int, 6 as libc::c_int);
        lua_settop(L, 5 as libc::c_int);
        status = lua_load(
            L,
            Some(
                generic_reader
                    as unsafe extern "C" fn(
                        *mut lua_State,
                        *mut libc::c_void,
                        *mut size_t,
                    ) -> *const libc::c_char,
            ),
            0 as *mut libc::c_void,
            chunkname_0,
            mode,
        );
    }
    return load_aux(L, status, env);
}
unsafe extern "C" fn dofilecont(
    mut L: *mut lua_State,
    mut d1: libc::c_int,
    mut d2: lua_KContext,
) -> libc::c_int {
    return lua_gettop(L) - 1 as libc::c_int;
}
unsafe extern "C" fn luaB_dofile(mut L: *mut lua_State) -> libc::c_int {
    let mut fname: *const libc::c_char = luaL_optlstring(
        L,
        1 as libc::c_int,
        0 as *const libc::c_char,
        0 as *mut size_t,
    );
    lua_settop(L, 1 as libc::c_int);
    if ((luaL_loadfilex(L, fname, 0 as *const libc::c_char) != 0 as libc::c_int)
        as libc::c_int != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
    {
        return lua_error(L);
    }
    lua_callk(
        L,
        0 as libc::c_int,
        -(1 as libc::c_int),
        0 as libc::c_int as lua_KContext,
        Some(
            dofilecont
                as unsafe extern "C" fn(
                    *mut lua_State,
                    libc::c_int,
                    lua_KContext,
                ) -> libc::c_int,
        ),
    );
    return dofilecont(L, 0 as libc::c_int, 0 as libc::c_int as lua_KContext);
}
unsafe extern "C" fn luaB_assert(mut L: *mut lua_State) -> libc::c_int {
    if (lua_toboolean(L, 1 as libc::c_int) != 0 as libc::c_int) as libc::c_int
        as libc::c_long != 0
    {
        return lua_gettop(L)
    } else {
        luaL_checkany(L, 1 as libc::c_int);
        lua_rotate(L, 1 as libc::c_int, -(1 as libc::c_int));
        lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
        lua_pushstring(L, b"assertion failed!\0" as *const u8 as *const libc::c_char);
        lua_settop(L, 1 as libc::c_int);
        return luaB_error(L);
    };
}
unsafe extern "C" fn luaB_select(mut L: *mut lua_State) -> libc::c_int {
    let mut n: libc::c_int = lua_gettop(L);
    if lua_type(L, 1 as libc::c_int) == 4 as libc::c_int
        && *lua_tolstring(L, 1 as libc::c_int, 0 as *mut size_t) as libc::c_int
            == '#' as i32
    {
        lua_pushinteger(L, (n - 1 as libc::c_int) as lua_Integer);
        return 1 as libc::c_int;
    } else {
        let mut i: lua_Integer = luaL_checkinteger(L, 1 as libc::c_int);
        if i < 0 as libc::c_int as libc::c_longlong {
            i = n as libc::c_longlong + i;
        } else if i > n as libc::c_longlong {
            i = n as lua_Integer;
        }
        (((1 as libc::c_int as libc::c_longlong <= i) as libc::c_int != 0 as libc::c_int)
            as libc::c_int as libc::c_long != 0
            || luaL_argerror(
                L,
                1 as libc::c_int,
                b"index out of range\0" as *const u8 as *const libc::c_char,
            ) != 0) as libc::c_int;
        return n - i as libc::c_int;
    };
}
unsafe extern "C" fn finishpcall(
    mut L: *mut lua_State,
    mut status: libc::c_int,
    mut extra: lua_KContext,
) -> libc::c_int {
    if ((status != 0 as libc::c_int && status != 1 as libc::c_int) as libc::c_int
        != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
    {
        lua_pushboolean(L, 0 as libc::c_int);
        lua_pushvalue(L, -(2 as libc::c_int));
        return 2 as libc::c_int;
    } else {
        return lua_gettop(L) - extra as libc::c_int
    };
}
unsafe extern "C" fn luaB_pcall(mut L: *mut lua_State) -> libc::c_int {
    let mut status: libc::c_int = 0;
    luaL_checkany(L, 1 as libc::c_int);
    lua_pushboolean(L, 1 as libc::c_int);
    lua_rotate(L, 1 as libc::c_int, 1 as libc::c_int);
    status = lua_pcallk(
        L,
        lua_gettop(L) - 2 as libc::c_int,
        -(1 as libc::c_int),
        0 as libc::c_int,
        0 as libc::c_int as lua_KContext,
        Some(
            finishpcall
                as unsafe extern "C" fn(
                    *mut lua_State,
                    libc::c_int,
                    lua_KContext,
                ) -> libc::c_int,
        ),
    );
    return finishpcall(L, status, 0 as libc::c_int as lua_KContext);
}
unsafe extern "C" fn luaB_xpcall(mut L: *mut lua_State) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut n: libc::c_int = lua_gettop(L);
    luaL_checktype(L, 2 as libc::c_int, 6 as libc::c_int);
    lua_pushboolean(L, 1 as libc::c_int);
    lua_pushvalue(L, 1 as libc::c_int);
    lua_rotate(L, 3 as libc::c_int, 2 as libc::c_int);
    status = lua_pcallk(
        L,
        n - 2 as libc::c_int,
        -(1 as libc::c_int),
        2 as libc::c_int,
        2 as libc::c_int as lua_KContext,
        Some(
            finishpcall
                as unsafe extern "C" fn(
                    *mut lua_State,
                    libc::c_int,
                    lua_KContext,
                ) -> libc::c_int,
        ),
    );
    return finishpcall(L, status, 2 as libc::c_int as lua_KContext);
}
unsafe extern "C" fn luaB_tostring(mut L: *mut lua_State) -> libc::c_int {
    luaL_checkany(L, 1 as libc::c_int);
    luaL_tolstring(L, 1 as libc::c_int, 0 as *mut size_t);
    return 1 as libc::c_int;
}
static mut base_funcs: [luaL_Reg; 26] = unsafe {
    [
        {
            let mut init = luaL_Reg {
                name: b"assert\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_assert as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"collectgarbage\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_collectgarbage
                        as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"dofile\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_dofile as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"error\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_error as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"getmetatable\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_getmetatable
                        as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"ipairs\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_ipairs as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"loadfile\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_loadfile as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"load\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_load as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"next\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_next as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"pairs\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_pairs as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"pcall\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_pcall as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"print\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_print as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"warn\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_warn as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"rawequal\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_rawequal as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"rawlen\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_rawlen as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"rawget\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_rawget as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"rawset\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_rawset as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"select\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_select as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"setmetatable\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_setmetatable
                        as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"tonumber\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_tonumber as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"tostring\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_tostring as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"type\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_type as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"xpcall\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_xpcall as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
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
#[no_mangle]
pub unsafe extern "C" fn luaopen_base(mut L: *mut lua_State) -> libc::c_int {
    lua_rawgeti(
        L,
        -(1000000 as libc::c_int) - 1000 as libc::c_int,
        2 as libc::c_int as lua_Integer,
    );
    luaL_setfuncs(L, base_funcs.as_ptr(), 0 as libc::c_int);
    lua_pushvalue(L, -(1 as libc::c_int));
    lua_setfield(L, -(2 as libc::c_int), b"_G\0" as *const u8 as *const libc::c_char);
    lua_pushstring(L, b"Lua 5.4\0" as *const u8 as *const libc::c_char);
    lua_setfield(
        L,
        -(2 as libc::c_int),
        b"_VERSION\0" as *const u8 as *const libc::c_char,
    );
    return 1 as libc::c_int;
}
