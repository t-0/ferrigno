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
    pub type lua_State;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn localeconv() -> *mut lconv;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strpbrk(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn lua_gettop(L: *mut lua_State) -> libc::c_int;
    fn lua_settop(L: *mut lua_State, idx: libc::c_int);
    fn lua_pushvalue(L: *mut lua_State, idx: libc::c_int);
    fn lua_rotate(L: *mut lua_State, idx: libc::c_int, n: libc::c_int);
    fn lua_isstring(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    fn lua_isinteger(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    fn lua_type(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    fn lua_typename(L: *mut lua_State, tp: libc::c_int) -> *const libc::c_char;
    fn lua_tonumberx(
        L: *mut lua_State,
        idx: libc::c_int,
        isnum: *mut libc::c_int,
    ) -> lua_Number;
    fn lua_tointegerx(
        L: *mut lua_State,
        idx: libc::c_int,
        isnum: *mut libc::c_int,
    ) -> lua_Integer;
    fn lua_toboolean(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    fn lua_tolstring(
        L: *mut lua_State,
        idx: libc::c_int,
        len: *mut size_t,
    ) -> *const libc::c_char;
    fn lua_touserdata(L: *mut lua_State, idx: libc::c_int) -> *mut libc::c_void;
    fn lua_topointer(L: *mut lua_State, idx: libc::c_int) -> *const libc::c_void;
    fn lua_arith(L: *mut lua_State, op: libc::c_int);
    fn lua_pushnil(L: *mut lua_State);
    fn lua_pushnumber(L: *mut lua_State, n: lua_Number);
    fn lua_pushinteger(L: *mut lua_State, n: lua_Integer);
    fn lua_pushlstring(
        L: *mut lua_State,
        s: *const libc::c_char,
        len: size_t,
    ) -> *const libc::c_char;
    fn lua_pushstring(L: *mut lua_State, s: *const libc::c_char) -> *const libc::c_char;
    fn lua_pushcclosure(L: *mut lua_State, fn_0: lua_CFunction, n: libc::c_int);
    fn lua_gettable(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    fn lua_createtable(L: *mut lua_State, narr: libc::c_int, nrec: libc::c_int);
    fn lua_newuserdatauv(
        L: *mut lua_State,
        sz: size_t,
        nuvalue: libc::c_int,
    ) -> *mut libc::c_void;
    fn lua_setfield(L: *mut lua_State, idx: libc::c_int, k: *const libc::c_char);
    fn lua_setmetatable(L: *mut lua_State, objindex: libc::c_int) -> libc::c_int;
    fn lua_callk(
        L: *mut lua_State,
        nargs: libc::c_int,
        nresults: libc::c_int,
        ctx: lua_KContext,
        k: lua_KFunction,
    );
    fn lua_dump(
        L: *mut lua_State,
        writer_0: lua_Writer,
        data: *mut libc::c_void,
        strip: libc::c_int,
    ) -> libc::c_int;
    fn lua_stringtonumber(L: *mut lua_State, s: *const libc::c_char) -> size_t;
    fn luaL_checkversion_(L: *mut lua_State, ver: lua_Number, sz: size_t);
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
    fn luaL_checknumber(L: *mut lua_State, arg: libc::c_int) -> lua_Number;
    fn luaL_checkinteger(L: *mut lua_State, arg: libc::c_int) -> lua_Integer;
    fn luaL_optinteger(
        L: *mut lua_State,
        arg: libc::c_int,
        def: lua_Integer,
    ) -> lua_Integer;
    fn luaL_checkstack(L: *mut lua_State, sz: libc::c_int, msg: *const libc::c_char);
    fn luaL_checktype(L: *mut lua_State, arg: libc::c_int, t: libc::c_int);
    fn luaL_error(L: *mut lua_State, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn luaL_setfuncs(L: *mut lua_State, l: *const luaL_Reg, nup: libc::c_int);
    fn luaL_buffinit(L: *mut lua_State, B: *mut luaL_Buffer);
    fn luaL_prepbuffsize(B: *mut luaL_Buffer, sz: size_t) -> *mut libc::c_char;
    fn luaL_addlstring(B: *mut luaL_Buffer, s: *const libc::c_char, l: size_t);
    fn luaL_addstring(B: *mut luaL_Buffer, s: *const libc::c_char);
    fn luaL_addvalue(B: *mut luaL_Buffer);
    fn luaL_pushresult(B: *mut luaL_Buffer);
    fn luaL_pushresultsize(B: *mut luaL_Buffer, sz: size_t);
    fn luaL_buffinitsize(
        L: *mut lua_State,
        B: *mut luaL_Buffer,
        sz: size_t,
    ) -> *mut libc::c_char;
}
pub type __int32_t = libc::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lconv {
    pub decimal_point: *mut libc::c_char,
    pub thousands_sep: *mut libc::c_char,
    pub grouping: *mut libc::c_char,
    pub int_curr_symbol: *mut libc::c_char,
    pub currency_symbol: *mut libc::c_char,
    pub mon_decimal_point: *mut libc::c_char,
    pub mon_thousands_sep: *mut libc::c_char,
    pub mon_grouping: *mut libc::c_char,
    pub positive_sign: *mut libc::c_char,
    pub negative_sign: *mut libc::c_char,
    pub int_frac_digits: libc::c_char,
    pub frac_digits: libc::c_char,
    pub p_cs_precedes: libc::c_char,
    pub p_sep_by_space: libc::c_char,
    pub n_cs_precedes: libc::c_char,
    pub n_sep_by_space: libc::c_char,
    pub p_sign_posn: libc::c_char,
    pub n_sign_posn: libc::c_char,
    pub int_p_cs_precedes: libc::c_char,
    pub int_p_sep_by_space: libc::c_char,
    pub int_n_cs_precedes: libc::c_char,
    pub int_n_sep_by_space: libc::c_char,
    pub int_p_sign_posn: libc::c_char,
    pub int_n_sign_posn: libc::c_char,
}
pub type ptrdiff_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type intptr_t = libc::c_long;
pub type lua_Number = libc::c_double;
pub type lua_Integer = libc::c_longlong;
pub type lua_Unsigned = libc::c_ulonglong;
pub type lua_KContext = intptr_t;
pub type lua_CFunction = Option::<unsafe extern "C" fn(*mut lua_State) -> libc::c_int>;
pub type lua_KFunction = Option::<
    unsafe extern "C" fn(*mut lua_State, libc::c_int, lua_KContext) -> libc::c_int,
>;
pub type lua_Writer = Option::<
    unsafe extern "C" fn(
        *mut lua_State,
        *const libc::c_void,
        size_t,
        *mut libc::c_void,
    ) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaL_Buffer {
    pub b: *mut libc::c_char,
    pub size: size_t,
    pub n: size_t,
    pub L: *mut lua_State,
    pub init: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub n: lua_Number,
    pub u: libc::c_double,
    pub s: *mut libc::c_void,
    pub i: lua_Integer,
    pub l: libc::c_long,
    pub b: [libc::c_char; 1024],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaL_Reg {
    pub name: *const libc::c_char,
    pub func: lua_CFunction,
}
pub const Knop: KOption = 10;
pub const Kpadding: KOption = 8;
pub const Kpaddalign: KOption = 9;
pub const Kzstr: KOption = 7;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Header {
    pub L: *mut lua_State,
    pub islittle: libc::c_int,
    pub maxalign: libc::c_int,
}
pub const Kstring: KOption = 6;
pub const Kchar: KOption = 5;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub dummy: libc::c_int,
    pub little: libc::c_char,
}
pub const Kdouble: KOption = 4;
pub const Knumber: KOption = 3;
pub const Kfloat: KOption = 2;
pub const Kint: KOption = 0;
pub type KOption = libc::c_uint;
pub const Kuint: KOption = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cD {
    pub c: libc::c_char,
    pub u: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub n: lua_Number,
    pub u: libc::c_double,
    pub s: *mut libc::c_void,
    pub i: lua_Integer,
    pub l: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MatchState {
    pub src_init: *const libc::c_char,
    pub src_end: *const libc::c_char,
    pub p_end: *const libc::c_char,
    pub L: *mut lua_State,
    pub matchdepth: libc::c_int,
    pub level: libc::c_uchar,
    pub capture: [C2RustUnnamed_3; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub init: *const libc::c_char,
    pub len: ptrdiff_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GMatchState {
    pub src: *const libc::c_char,
    pub p: *const libc::c_char,
    pub lastmatch: *const libc::c_char,
    pub ms: MatchState,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct str_Writer {
    pub init: libc::c_int,
    pub B: luaL_Buffer,
}
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
unsafe extern "C" fn str_len(mut L: *mut lua_State) -> libc::c_int {
    let mut l: size_t = 0;
    luaL_checklstring(L, 1 as libc::c_int, &mut l);
    lua_pushinteger(L, l as lua_Integer);
    return 1 as libc::c_int;
}
unsafe extern "C" fn posrelatI(mut pos: lua_Integer, mut len: size_t) -> size_t {
    if pos > 0 as libc::c_int as libc::c_longlong {
        return pos as size_t
    } else if pos == 0 as libc::c_int as libc::c_longlong {
        return 1 as libc::c_int as size_t
    } else if pos < -(len as lua_Integer) {
        return 1 as libc::c_int as size_t
    } else {
        return len
            .wrapping_add(pos as size_t)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
    };
}
unsafe extern "C" fn getendpos(
    mut L: *mut lua_State,
    mut arg: libc::c_int,
    mut def: lua_Integer,
    mut len: size_t,
) -> size_t {
    let mut pos: lua_Integer = luaL_optinteger(L, arg, def);
    if pos > len as lua_Integer {
        return len
    } else if pos >= 0 as libc::c_int as libc::c_longlong {
        return pos as size_t
    } else if pos < -(len as lua_Integer) {
        return 0 as libc::c_int as size_t
    } else {
        return len
            .wrapping_add(pos as size_t)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
    };
}
unsafe extern "C" fn str_sub(mut L: *mut lua_State) -> libc::c_int {
    let mut l: size_t = 0;
    let mut s: *const libc::c_char = luaL_checklstring(L, 1 as libc::c_int, &mut l);
    let mut start: size_t = posrelatI(luaL_checkinteger(L, 2 as libc::c_int), l);
    let mut end: size_t = getendpos(
        L,
        3 as libc::c_int,
        -(1 as libc::c_int) as lua_Integer,
        l,
    );
    if start <= end {
        lua_pushlstring(
            L,
            s.offset(start as isize).offset(-(1 as libc::c_int as isize)),
            end.wrapping_sub(start).wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
    } else {
        lua_pushstring(L, b"\0" as *const u8 as *const libc::c_char);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn str_reverse(mut L: *mut lua_State) -> libc::c_int {
    let mut l: size_t = 0;
    let mut i: size_t = 0;
    let mut b: luaL_Buffer = luaL_Buffer {
        b: 0 as *mut libc::c_char,
        size: 0,
        n: 0,
        L: 0 as *mut lua_State,
        init: C2RustUnnamed_0 { n: 0. },
    };
    let mut s: *const libc::c_char = luaL_checklstring(L, 1 as libc::c_int, &mut l);
    let mut p: *mut libc::c_char = luaL_buffinitsize(L, &mut b, l);
    i = 0 as libc::c_int as size_t;
    while i < l {
        *p
            .offset(
                i as isize,
            ) = *s
            .offset(
                l.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    as isize,
            );
        i = i.wrapping_add(1);
        i;
    }
    luaL_pushresultsize(&mut b, l);
    return 1 as libc::c_int;
}
unsafe extern "C" fn str_lower(mut L: *mut lua_State) -> libc::c_int {
    let mut l: size_t = 0;
    let mut i: size_t = 0;
    let mut b: luaL_Buffer = luaL_Buffer {
        b: 0 as *mut libc::c_char,
        size: 0,
        n: 0,
        L: 0 as *mut lua_State,
        init: C2RustUnnamed_0 { n: 0. },
    };
    let mut s: *const libc::c_char = luaL_checklstring(L, 1 as libc::c_int, &mut l);
    let mut p: *mut libc::c_char = luaL_buffinitsize(L, &mut b, l);
    i = 0 as libc::c_int as size_t;
    while i < l {
        *p
            .offset(
                i as isize,
            ) = ({
            let mut __res: libc::c_int = 0;
            if ::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
            {
                if 0 != 0 {
                    let mut __c: libc::c_int = *s.offset(i as isize) as libc::c_uchar
                        as libc::c_int;
                    __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*__ctype_tolower_loc()).offset(__c as isize)
                    };
                } else {
                    __res = tolower(
                        *s.offset(i as isize) as libc::c_uchar as libc::c_int,
                    );
                }
            } else {
                __res = *(*__ctype_tolower_loc())
                    .offset(
                        *s.offset(i as isize) as libc::c_uchar as libc::c_int as isize,
                    );
            }
            __res
        }) as libc::c_char;
        i = i.wrapping_add(1);
        i;
    }
    luaL_pushresultsize(&mut b, l);
    return 1 as libc::c_int;
}
unsafe extern "C" fn str_upper(mut L: *mut lua_State) -> libc::c_int {
    let mut l: size_t = 0;
    let mut i: size_t = 0;
    let mut b: luaL_Buffer = luaL_Buffer {
        b: 0 as *mut libc::c_char,
        size: 0,
        n: 0,
        L: 0 as *mut lua_State,
        init: C2RustUnnamed_0 { n: 0. },
    };
    let mut s: *const libc::c_char = luaL_checklstring(L, 1 as libc::c_int, &mut l);
    let mut p: *mut libc::c_char = luaL_buffinitsize(L, &mut b, l);
    i = 0 as libc::c_int as size_t;
    while i < l {
        *p
            .offset(
                i as isize,
            ) = ({
            let mut __res: libc::c_int = 0;
            if ::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
            {
                if 0 != 0 {
                    let mut __c: libc::c_int = *s.offset(i as isize) as libc::c_uchar
                        as libc::c_int;
                    __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*__ctype_toupper_loc()).offset(__c as isize)
                    };
                } else {
                    __res = toupper(
                        *s.offset(i as isize) as libc::c_uchar as libc::c_int,
                    );
                }
            } else {
                __res = *(*__ctype_toupper_loc())
                    .offset(
                        *s.offset(i as isize) as libc::c_uchar as libc::c_int as isize,
                    );
            }
            __res
        }) as libc::c_char;
        i = i.wrapping_add(1);
        i;
    }
    luaL_pushresultsize(&mut b, l);
    return 1 as libc::c_int;
}
unsafe extern "C" fn str_rep(mut L: *mut lua_State) -> libc::c_int {
    let mut l: size_t = 0;
    let mut lsep: size_t = 0;
    let mut s: *const libc::c_char = luaL_checklstring(L, 1 as libc::c_int, &mut l);
    let mut n: lua_Integer = luaL_checkinteger(L, 2 as libc::c_int);
    let mut sep: *const libc::c_char = luaL_optlstring(
        L,
        3 as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char,
        &mut lsep,
    );
    if n <= 0 as libc::c_int as libc::c_longlong {
        lua_pushstring(L, b"\0" as *const u8 as *const libc::c_char);
    } else if ((l.wrapping_add(lsep) < l
        || l.wrapping_add(lsep) as libc::c_ulonglong
            > ((if (::core::mem::size_of::<size_t>() as libc::c_ulong)
                < ::core::mem::size_of::<libc::c_int>() as libc::c_ulong
            {
                !(0 as libc::c_int as size_t)
            } else {
                2147483647 as libc::c_int as size_t
            }) as libc::c_ulonglong)
                .wrapping_div(n as libc::c_ulonglong)) as libc::c_int
        != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
    {
        return luaL_error(
            L,
            b"resulting string too large\0" as *const u8 as *const libc::c_char,
        )
    } else {
        let mut totallen: size_t = (n as size_t)
            .wrapping_mul(l)
            .wrapping_add(
                ((n - 1 as libc::c_int as libc::c_longlong) as size_t).wrapping_mul(lsep),
            );
        let mut b: luaL_Buffer = luaL_Buffer {
            b: 0 as *mut libc::c_char,
            size: 0,
            n: 0,
            L: 0 as *mut lua_State,
            init: C2RustUnnamed_0 { n: 0. },
        };
        let mut p: *mut libc::c_char = luaL_buffinitsize(L, &mut b, totallen);
        loop {
            let fresh0 = n;
            n = n - 1;
            if !(fresh0 > 1 as libc::c_int as libc::c_longlong) {
                break;
            }
            memcpy(
                p as *mut libc::c_void,
                s as *const libc::c_void,
                l.wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
            );
            p = p.offset(l as isize);
            if lsep > 0 as libc::c_int as libc::c_ulong {
                memcpy(
                    p as *mut libc::c_void,
                    sep as *const libc::c_void,
                    lsep
                        .wrapping_mul(
                            ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                        ),
                );
                p = p.offset(lsep as isize);
            }
        }
        memcpy(
            p as *mut libc::c_void,
            s as *const libc::c_void,
            l.wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
        );
        luaL_pushresultsize(&mut b, totallen);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn str_byte(mut L: *mut lua_State) -> libc::c_int {
    let mut l: size_t = 0;
    let mut s: *const libc::c_char = luaL_checklstring(L, 1 as libc::c_int, &mut l);
    let mut pi: lua_Integer = luaL_optinteger(
        L,
        2 as libc::c_int,
        1 as libc::c_int as lua_Integer,
    );
    let mut posi: size_t = posrelatI(pi, l);
    let mut pose: size_t = getendpos(L, 3 as libc::c_int, pi, l);
    let mut n: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    if posi > pose {
        return 0 as libc::c_int;
    }
    if ((pose.wrapping_sub(posi) >= 2147483647 as libc::c_int as size_t) as libc::c_int
        != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
    {
        return luaL_error(
            L,
            b"string slice too long\0" as *const u8 as *const libc::c_char,
        );
    }
    n = pose.wrapping_sub(posi) as libc::c_int + 1 as libc::c_int;
    luaL_checkstack(
        L,
        n,
        b"string slice too long\0" as *const u8 as *const libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < n {
        lua_pushinteger(
            L,
            *s
                .offset(
                    posi
                        .wrapping_add(i as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                ) as libc::c_uchar as lua_Integer,
        );
        i += 1;
        i;
    }
    return n;
}
unsafe extern "C" fn str_char(mut L: *mut lua_State) -> libc::c_int {
    let mut n: libc::c_int = lua_gettop(L);
    let mut i: libc::c_int = 0;
    let mut b: luaL_Buffer = luaL_Buffer {
        b: 0 as *mut libc::c_char,
        size: 0,
        n: 0,
        L: 0 as *mut lua_State,
        init: C2RustUnnamed_0 { n: 0. },
    };
    let mut p: *mut libc::c_char = luaL_buffinitsize(L, &mut b, n as size_t);
    i = 1 as libc::c_int;
    while i <= n {
        let mut c: lua_Unsigned = luaL_checkinteger(L, i) as lua_Unsigned;
        (((c
            <= (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                as lua_Unsigned) as libc::c_int != 0 as libc::c_int) as libc::c_int
            as libc::c_long != 0
            || luaL_argerror(
                L,
                i,
                b"value out of range\0" as *const u8 as *const libc::c_char,
            ) != 0) as libc::c_int;
        *p.offset((i - 1 as libc::c_int) as isize) = c as libc::c_uchar as libc::c_char;
        i += 1;
        i;
    }
    luaL_pushresultsize(&mut b, n as size_t);
    return 1 as libc::c_int;
}
unsafe extern "C" fn writer(
    mut L: *mut lua_State,
    mut b: *const libc::c_void,
    mut size: size_t,
    mut ud: *mut libc::c_void,
) -> libc::c_int {
    let mut state: *mut str_Writer = ud as *mut str_Writer;
    if (*state).init == 0 {
        (*state).init = 1 as libc::c_int;
        luaL_buffinit(L, &mut (*state).B);
    }
    luaL_addlstring(&mut (*state).B, b as *const libc::c_char, size);
    return 0 as libc::c_int;
}
unsafe extern "C" fn str_dump(mut L: *mut lua_State) -> libc::c_int {
    let mut state: str_Writer = str_Writer {
        init: 0,
        B: luaL_Buffer {
            b: 0 as *mut libc::c_char,
            size: 0,
            n: 0,
            L: 0 as *mut lua_State,
            init: C2RustUnnamed_0 { n: 0. },
        },
    };
    let mut strip: libc::c_int = lua_toboolean(L, 2 as libc::c_int);
    luaL_checktype(L, 1 as libc::c_int, 6 as libc::c_int);
    lua_settop(L, 1 as libc::c_int);
    state.init = 0 as libc::c_int;
    if ((lua_dump(
        L,
        Some(
            writer
                as unsafe extern "C" fn(
                    *mut lua_State,
                    *const libc::c_void,
                    size_t,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        &mut state as *mut str_Writer as *mut libc::c_void,
        strip,
    ) != 0 as libc::c_int) as libc::c_int != 0 as libc::c_int) as libc::c_int
        as libc::c_long != 0
    {
        return luaL_error(
            L,
            b"unable to dump given function\0" as *const u8 as *const libc::c_char,
        );
    }
    luaL_pushresult(&mut state.B);
    return 1 as libc::c_int;
}
unsafe extern "C" fn tonum(mut L: *mut lua_State, mut arg: libc::c_int) -> libc::c_int {
    if lua_type(L, arg) == 3 as libc::c_int {
        lua_pushvalue(L, arg);
        return 1 as libc::c_int;
    } else {
        let mut len: size_t = 0;
        let mut s: *const libc::c_char = lua_tolstring(L, arg, &mut len);
        return (!s.is_null()
            && lua_stringtonumber(L, s)
                == len.wrapping_add(1 as libc::c_int as libc::c_ulong)) as libc::c_int;
    };
}
unsafe extern "C" fn trymt(mut L: *mut lua_State, mut mtname: *const libc::c_char) {
    lua_settop(L, 2 as libc::c_int);
    if ((lua_type(L, 2 as libc::c_int) == 4 as libc::c_int
        || luaL_getmetafield(L, 2 as libc::c_int, mtname) == 0) as libc::c_int
        != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
    {
        luaL_error(
            L,
            b"attempt to %s a '%s' with a '%s'\0" as *const u8 as *const libc::c_char,
            mtname.offset(2 as libc::c_int as isize),
            lua_typename(L, lua_type(L, -(2 as libc::c_int))),
            lua_typename(L, lua_type(L, -(1 as libc::c_int))),
        );
    }
    lua_rotate(L, -(3 as libc::c_int), 1 as libc::c_int);
    lua_callk(
        L,
        2 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int as lua_KContext,
        None,
    );
}
unsafe extern "C" fn arith(
    mut L: *mut lua_State,
    mut op: libc::c_int,
    mut mtname: *const libc::c_char,
) -> libc::c_int {
    if tonum(L, 1 as libc::c_int) != 0 && tonum(L, 2 as libc::c_int) != 0 {
        lua_arith(L, op);
    } else {
        trymt(L, mtname);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn arith_add(mut L: *mut lua_State) -> libc::c_int {
    return arith(L, 0 as libc::c_int, b"__add\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn arith_sub(mut L: *mut lua_State) -> libc::c_int {
    return arith(L, 1 as libc::c_int, b"__sub\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn arith_mul(mut L: *mut lua_State) -> libc::c_int {
    return arith(L, 2 as libc::c_int, b"__mul\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn arith_mod(mut L: *mut lua_State) -> libc::c_int {
    return arith(L, 3 as libc::c_int, b"__mod\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn arith_pow(mut L: *mut lua_State) -> libc::c_int {
    return arith(L, 4 as libc::c_int, b"__pow\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn arith_div(mut L: *mut lua_State) -> libc::c_int {
    return arith(L, 5 as libc::c_int, b"__div\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn arith_idiv(mut L: *mut lua_State) -> libc::c_int {
    return arith(L, 6 as libc::c_int, b"__idiv\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn arith_unm(mut L: *mut lua_State) -> libc::c_int {
    return arith(L, 12 as libc::c_int, b"__unm\0" as *const u8 as *const libc::c_char);
}
static mut stringmetamethods: [luaL_Reg; 10] = unsafe {
    [
        {
            let mut init = luaL_Reg {
                name: b"__add\0" as *const u8 as *const libc::c_char,
                func: Some(
                    arith_add as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"__sub\0" as *const u8 as *const libc::c_char,
                func: Some(
                    arith_sub as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"__mul\0" as *const u8 as *const libc::c_char,
                func: Some(
                    arith_mul as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"__mod\0" as *const u8 as *const libc::c_char,
                func: Some(
                    arith_mod as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"__pow\0" as *const u8 as *const libc::c_char,
                func: Some(
                    arith_pow as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"__div\0" as *const u8 as *const libc::c_char,
                func: Some(
                    arith_div as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"__idiv\0" as *const u8 as *const libc::c_char,
                func: Some(
                    arith_idiv as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"__unm\0" as *const u8 as *const libc::c_char,
                func: Some(
                    arith_unm as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"__index\0" as *const u8 as *const libc::c_char,
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
unsafe extern "C" fn check_capture(
    mut ms: *mut MatchState,
    mut l: libc::c_int,
) -> libc::c_int {
    l -= '1' as i32;
    if ((l < 0 as libc::c_int || l >= (*ms).level as libc::c_int
        || (*ms).capture[l as usize].len == -(1 as libc::c_int) as libc::c_long)
        as libc::c_int != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
    {
        return luaL_error(
            (*ms).L,
            b"invalid capture index %%%d\0" as *const u8 as *const libc::c_char,
            l + 1 as libc::c_int,
        );
    }
    return l;
}
unsafe extern "C" fn capture_to_close(mut ms: *mut MatchState) -> libc::c_int {
    let mut level: libc::c_int = (*ms).level as libc::c_int;
    level -= 1;
    level;
    while level >= 0 as libc::c_int {
        if (*ms).capture[level as usize].len == -(1 as libc::c_int) as libc::c_long {
            return level;
        }
        level -= 1;
        level;
    }
    return luaL_error(
        (*ms).L,
        b"invalid pattern capture\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn classend(
    mut ms: *mut MatchState,
    mut p: *const libc::c_char,
) -> *const libc::c_char {
    let fresh1 = p;
    p = p.offset(1);
    match *fresh1 as libc::c_int {
        37 => {
            if ((p == (*ms).p_end) as libc::c_int != 0 as libc::c_int) as libc::c_int
                as libc::c_long != 0
            {
                luaL_error(
                    (*ms).L,
                    b"malformed pattern (ends with '%%')\0" as *const u8
                        as *const libc::c_char,
                );
            }
            return p.offset(1 as libc::c_int as isize);
        }
        91 => {
            if *p as libc::c_int == '^' as i32 {
                p = p.offset(1);
                p;
            }
            loop {
                if ((p == (*ms).p_end) as libc::c_int != 0 as libc::c_int) as libc::c_int
                    as libc::c_long != 0
                {
                    luaL_error(
                        (*ms).L,
                        b"malformed pattern (missing ']')\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                let fresh2 = p;
                p = p.offset(1);
                if *fresh2 as libc::c_int == '%' as i32 && p < (*ms).p_end {
                    p = p.offset(1);
                    p;
                }
                if !(*p as libc::c_int != ']' as i32) {
                    break;
                }
            }
            return p.offset(1 as libc::c_int as isize);
        }
        _ => return p,
    };
}
unsafe extern "C" fn match_class(
    mut c: libc::c_int,
    mut cl: libc::c_int,
) -> libc::c_int {
    let mut res: libc::c_int = 0;
    match ({
        let mut __res: libc::c_int = 0;
        if ::core::mem::size_of::<libc::c_int>() as libc::c_ulong
            > 1 as libc::c_int as libc::c_ulong
        {
            if 0 != 0 {
                let mut __c: libc::c_int = cl;
                __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                    __c
                } else {
                    *(*__ctype_tolower_loc()).offset(__c as isize)
                };
            } else {
                __res = tolower(cl);
            }
        } else {
            __res = *(*__ctype_tolower_loc()).offset(cl as isize);
        }
        __res
    }) {
        97 => {
            res = *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
                & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int;
        }
        99 => {
            res = *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
                & _IScntrl as libc::c_int as libc::c_ushort as libc::c_int;
        }
        100 => {
            res = *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int;
        }
        103 => {
            res = *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
                & _ISgraph as libc::c_int as libc::c_ushort as libc::c_int;
        }
        108 => {
            res = *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
                & _ISlower as libc::c_int as libc::c_ushort as libc::c_int;
        }
        112 => {
            res = *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
                & _ISpunct as libc::c_int as libc::c_ushort as libc::c_int;
        }
        115 => {
            res = *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int;
        }
        117 => {
            res = *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
                & _ISupper as libc::c_int as libc::c_ushort as libc::c_int;
        }
        119 => {
            res = *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
                & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int;
        }
        120 => {
            res = *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
                & _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int;
        }
        122 => {
            res = (c == 0 as libc::c_int) as libc::c_int;
        }
        _ => return (cl == c) as libc::c_int,
    }
    return if *(*__ctype_b_loc()).offset(cl as isize) as libc::c_int
        & _ISlower as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        res
    } else {
        (res == 0) as libc::c_int
    };
}
unsafe extern "C" fn matchbracketclass(
    mut c: libc::c_int,
    mut p: *const libc::c_char,
    mut ec: *const libc::c_char,
) -> libc::c_int {
    let mut sig: libc::c_int = 1 as libc::c_int;
    if *p.offset(1 as libc::c_int as isize) as libc::c_int == '^' as i32 {
        sig = 0 as libc::c_int;
        p = p.offset(1);
        p;
    }
    loop {
        p = p.offset(1);
        if !(p < ec) {
            break;
        }
        if *p as libc::c_int == '%' as i32 {
            p = p.offset(1);
            p;
            if match_class(c, *p as libc::c_uchar as libc::c_int) != 0 {
                return sig;
            }
        } else if *p.offset(1 as libc::c_int as isize) as libc::c_int == '-' as i32
            && p.offset(2 as libc::c_int as isize) < ec
        {
            p = p.offset(2 as libc::c_int as isize);
            if *p.offset(-(2 as libc::c_int as isize)) as libc::c_uchar as libc::c_int
                <= c && c <= *p as libc::c_uchar as libc::c_int
            {
                return sig;
            }
        } else if *p as libc::c_uchar as libc::c_int == c {
            return sig
        }
    }
    return (sig == 0) as libc::c_int;
}
unsafe extern "C" fn singlematch(
    mut ms: *mut MatchState,
    mut s: *const libc::c_char,
    mut p: *const libc::c_char,
    mut ep: *const libc::c_char,
) -> libc::c_int {
    if s >= (*ms).src_end {
        return 0 as libc::c_int
    } else {
        let mut c: libc::c_int = *s as libc::c_uchar as libc::c_int;
        match *p as libc::c_int {
            46 => return 1 as libc::c_int,
            37 => {
                return match_class(
                    c,
                    *p.offset(1 as libc::c_int as isize) as libc::c_uchar as libc::c_int,
                );
            }
            91 => return matchbracketclass(c, p, ep.offset(-(1 as libc::c_int as isize))),
            _ => return (*p as libc::c_uchar as libc::c_int == c) as libc::c_int,
        }
    };
}
unsafe extern "C" fn matchbalance(
    mut ms: *mut MatchState,
    mut s: *const libc::c_char,
    mut p: *const libc::c_char,
) -> *const libc::c_char {
    if ((p >= ((*ms).p_end).offset(-(1 as libc::c_int as isize))) as libc::c_int
        != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
    {
        luaL_error(
            (*ms).L,
            b"malformed pattern (missing arguments to '%%b')\0" as *const u8
                as *const libc::c_char,
        );
    }
    if *s as libc::c_int != *p as libc::c_int {
        return 0 as *const libc::c_char
    } else {
        let mut b: libc::c_int = *p as libc::c_int;
        let mut e: libc::c_int = *p.offset(1 as libc::c_int as isize) as libc::c_int;
        let mut cont: libc::c_int = 1 as libc::c_int;
        loop {
            s = s.offset(1);
            if !(s < (*ms).src_end) {
                break;
            }
            if *s as libc::c_int == e {
                cont -= 1;
                if cont == 0 as libc::c_int {
                    return s.offset(1 as libc::c_int as isize);
                }
            } else if *s as libc::c_int == b {
                cont += 1;
                cont;
            }
        }
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn max_expand(
    mut ms: *mut MatchState,
    mut s: *const libc::c_char,
    mut p: *const libc::c_char,
    mut ep: *const libc::c_char,
) -> *const libc::c_char {
    let mut i: ptrdiff_t = 0 as libc::c_int as ptrdiff_t;
    while singlematch(ms, s.offset(i as isize), p, ep) != 0 {
        i += 1;
        i;
    }
    while i >= 0 as libc::c_int as libc::c_long {
        let mut res: *const libc::c_char = match_0(
            ms,
            s.offset(i as isize),
            ep.offset(1 as libc::c_int as isize),
        );
        if !res.is_null() {
            return res;
        }
        i -= 1;
        i;
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn min_expand(
    mut ms: *mut MatchState,
    mut s: *const libc::c_char,
    mut p: *const libc::c_char,
    mut ep: *const libc::c_char,
) -> *const libc::c_char {
    loop {
        let mut res: *const libc::c_char = match_0(
            ms,
            s,
            ep.offset(1 as libc::c_int as isize),
        );
        if !res.is_null() {
            return res
        } else if singlematch(ms, s, p, ep) != 0 {
            s = s.offset(1);
            s;
        } else {
            return 0 as *const libc::c_char
        }
    };
}
unsafe extern "C" fn start_capture(
    mut ms: *mut MatchState,
    mut s: *const libc::c_char,
    mut p: *const libc::c_char,
    mut what: libc::c_int,
) -> *const libc::c_char {
    let mut res: *const libc::c_char = 0 as *const libc::c_char;
    let mut level: libc::c_int = (*ms).level as libc::c_int;
    if level >= 32 as libc::c_int {
        luaL_error((*ms).L, b"too many captures\0" as *const u8 as *const libc::c_char);
    }
    (*ms).capture[level as usize].init = s;
    (*ms).capture[level as usize].len = what as ptrdiff_t;
    (*ms).level = (level + 1 as libc::c_int) as libc::c_uchar;
    res = match_0(ms, s, p);
    if res.is_null() {
        (*ms).level = ((*ms).level).wrapping_sub(1);
        (*ms).level;
    }
    return res;
}
unsafe extern "C" fn end_capture(
    mut ms: *mut MatchState,
    mut s: *const libc::c_char,
    mut p: *const libc::c_char,
) -> *const libc::c_char {
    let mut l: libc::c_int = capture_to_close(ms);
    let mut res: *const libc::c_char = 0 as *const libc::c_char;
    (*ms)
        .capture[l as usize]
        .len = s.offset_from((*ms).capture[l as usize].init) as libc::c_long;
    res = match_0(ms, s, p);
    if res.is_null() {
        (*ms).capture[l as usize].len = -(1 as libc::c_int) as ptrdiff_t;
    }
    return res;
}
unsafe extern "C" fn match_capture(
    mut ms: *mut MatchState,
    mut s: *const libc::c_char,
    mut l: libc::c_int,
) -> *const libc::c_char {
    let mut len: size_t = 0;
    l = check_capture(ms, l);
    len = (*ms).capture[l as usize].len as size_t;
    if ((*ms).src_end).offset_from(s) as libc::c_long as size_t >= len
        && memcmp(
            (*ms).capture[l as usize].init as *const libc::c_void,
            s as *const libc::c_void,
            len,
        ) == 0 as libc::c_int
    {
        return s.offset(len as isize)
    } else {
        return 0 as *const libc::c_char
    };
}
unsafe extern "C" fn match_0(
    mut ms: *mut MatchState,
    mut s: *const libc::c_char,
    mut p: *const libc::c_char,
) -> *const libc::c_char {
    let mut ep_0: *const libc::c_char = 0 as *const libc::c_char;
    let mut current_block: u64;
    let fresh3 = (*ms).matchdepth;
    (*ms).matchdepth = (*ms).matchdepth - 1;
    if ((fresh3 == 0 as libc::c_int) as libc::c_int != 0 as libc::c_int) as libc::c_int
        as libc::c_long != 0
    {
        luaL_error(
            (*ms).L,
            b"pattern too complex\0" as *const u8 as *const libc::c_char,
        );
    }
    loop {
        if !(p != (*ms).p_end) {
            current_block = 6476622998065200121;
            break;
        }
        match *p as libc::c_int {
            40 => {
                if *p.offset(1 as libc::c_int as isize) as libc::c_int == ')' as i32 {
                    s = start_capture(
                        ms,
                        s,
                        p.offset(2 as libc::c_int as isize),
                        -(2 as libc::c_int),
                    );
                } else {
                    s = start_capture(
                        ms,
                        s,
                        p.offset(1 as libc::c_int as isize),
                        -(1 as libc::c_int),
                    );
                }
                current_block = 6476622998065200121;
                break;
            }
            41 => {
                s = end_capture(ms, s, p.offset(1 as libc::c_int as isize));
                current_block = 6476622998065200121;
                break;
            }
            36 => {
                if !(p.offset(1 as libc::c_int as isize) != (*ms).p_end) {
                    s = if s == (*ms).src_end { s } else { 0 as *const libc::c_char };
                    current_block = 6476622998065200121;
                    break;
                }
            }
            37 => {
                match *p.offset(1 as libc::c_int as isize) as libc::c_int {
                    98 => {
                        current_block = 17965632435239708295;
                        match current_block {
                            17965632435239708295 => {
                                s = matchbalance(
                                    ms,
                                    s,
                                    p.offset(2 as libc::c_int as isize),
                                );
                                if s.is_null() {
                                    current_block = 6476622998065200121;
                                    break;
                                }
                                p = p.offset(4 as libc::c_int as isize);
                                continue;
                            }
                            8236137900636309791 => {
                                let mut ep: *const libc::c_char = 0 as *const libc::c_char;
                                let mut previous: libc::c_char = 0;
                                p = p.offset(2 as libc::c_int as isize);
                                if ((*p as libc::c_int != '[' as i32) as libc::c_int
                                    != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
                                {
                                    luaL_error(
                                        (*ms).L,
                                        b"missing '[' after '%%f' in pattern\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                }
                                ep = classend(ms, p);
                                previous = (if s == (*ms).src_init {
                                    '\0' as i32
                                } else {
                                    *s.offset(-(1 as libc::c_int as isize)) as libc::c_int
                                }) as libc::c_char;
                                if matchbracketclass(
                                    previous as libc::c_uchar as libc::c_int,
                                    p,
                                    ep.offset(-(1 as libc::c_int as isize)),
                                ) == 0
                                    && matchbracketclass(
                                        *s as libc::c_uchar as libc::c_int,
                                        p,
                                        ep.offset(-(1 as libc::c_int as isize)),
                                    ) != 0
                                {
                                    p = ep;
                                    continue;
                                } else {
                                    s = 0 as *const libc::c_char;
                                    current_block = 6476622998065200121;
                                    break;
                                }
                            }
                            _ => {
                                s = match_capture(
                                    ms,
                                    s,
                                    *p.offset(1 as libc::c_int as isize) as libc::c_uchar
                                        as libc::c_int,
                                );
                                if s.is_null() {
                                    current_block = 6476622998065200121;
                                    break;
                                }
                                p = p.offset(2 as libc::c_int as isize);
                                continue;
                            }
                        }
                    }
                    102 => {
                        current_block = 8236137900636309791;
                        match current_block {
                            17965632435239708295 => {
                                s = matchbalance(
                                    ms,
                                    s,
                                    p.offset(2 as libc::c_int as isize),
                                );
                                if s.is_null() {
                                    current_block = 6476622998065200121;
                                    break;
                                }
                                p = p.offset(4 as libc::c_int as isize);
                                continue;
                            }
                            8236137900636309791 => {
                                let mut ep: *const libc::c_char = 0 as *const libc::c_char;
                                let mut previous: libc::c_char = 0;
                                p = p.offset(2 as libc::c_int as isize);
                                if ((*p as libc::c_int != '[' as i32) as libc::c_int
                                    != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
                                {
                                    luaL_error(
                                        (*ms).L,
                                        b"missing '[' after '%%f' in pattern\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                }
                                ep = classend(ms, p);
                                previous = (if s == (*ms).src_init {
                                    '\0' as i32
                                } else {
                                    *s.offset(-(1 as libc::c_int as isize)) as libc::c_int
                                }) as libc::c_char;
                                if matchbracketclass(
                                    previous as libc::c_uchar as libc::c_int,
                                    p,
                                    ep.offset(-(1 as libc::c_int as isize)),
                                ) == 0
                                    && matchbracketclass(
                                        *s as libc::c_uchar as libc::c_int,
                                        p,
                                        ep.offset(-(1 as libc::c_int as isize)),
                                    ) != 0
                                {
                                    p = ep;
                                    continue;
                                } else {
                                    s = 0 as *const libc::c_char;
                                    current_block = 6476622998065200121;
                                    break;
                                }
                            }
                            _ => {
                                s = match_capture(
                                    ms,
                                    s,
                                    *p.offset(1 as libc::c_int as isize) as libc::c_uchar
                                        as libc::c_int,
                                );
                                if s.is_null() {
                                    current_block = 6476622998065200121;
                                    break;
                                }
                                p = p.offset(2 as libc::c_int as isize);
                                continue;
                            }
                        }
                    }
                    48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                        current_block = 14576567515993809846;
                        match current_block {
                            17965632435239708295 => {
                                s = matchbalance(
                                    ms,
                                    s,
                                    p.offset(2 as libc::c_int as isize),
                                );
                                if s.is_null() {
                                    current_block = 6476622998065200121;
                                    break;
                                }
                                p = p.offset(4 as libc::c_int as isize);
                                continue;
                            }
                            8236137900636309791 => {
                                let mut ep: *const libc::c_char = 0 as *const libc::c_char;
                                let mut previous: libc::c_char = 0;
                                p = p.offset(2 as libc::c_int as isize);
                                if ((*p as libc::c_int != '[' as i32) as libc::c_int
                                    != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
                                {
                                    luaL_error(
                                        (*ms).L,
                                        b"missing '[' after '%%f' in pattern\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                }
                                ep = classend(ms, p);
                                previous = (if s == (*ms).src_init {
                                    '\0' as i32
                                } else {
                                    *s.offset(-(1 as libc::c_int as isize)) as libc::c_int
                                }) as libc::c_char;
                                if matchbracketclass(
                                    previous as libc::c_uchar as libc::c_int,
                                    p,
                                    ep.offset(-(1 as libc::c_int as isize)),
                                ) == 0
                                    && matchbracketclass(
                                        *s as libc::c_uchar as libc::c_int,
                                        p,
                                        ep.offset(-(1 as libc::c_int as isize)),
                                    ) != 0
                                {
                                    p = ep;
                                    continue;
                                } else {
                                    s = 0 as *const libc::c_char;
                                    current_block = 6476622998065200121;
                                    break;
                                }
                            }
                            _ => {
                                s = match_capture(
                                    ms,
                                    s,
                                    *p.offset(1 as libc::c_int as isize) as libc::c_uchar
                                        as libc::c_int,
                                );
                                if s.is_null() {
                                    current_block = 6476622998065200121;
                                    break;
                                }
                                p = p.offset(2 as libc::c_int as isize);
                                continue;
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        ep_0 = classend(ms, p);
        if singlematch(ms, s, p, ep_0) == 0 {
            if *ep_0 as libc::c_int == '*' as i32 || *ep_0 as libc::c_int == '?' as i32
                || *ep_0 as libc::c_int == '-' as i32
            {
                p = ep_0.offset(1 as libc::c_int as isize);
            } else {
                s = 0 as *const libc::c_char;
                current_block = 6476622998065200121;
                break;
            }
        } else {
            match *ep_0 as libc::c_int {
                63 => {
                    let mut res: *const libc::c_char = 0 as *const libc::c_char;
                    res = match_0(
                        ms,
                        s.offset(1 as libc::c_int as isize),
                        ep_0.offset(1 as libc::c_int as isize),
                    );
                    if !res.is_null() {
                        s = res;
                        current_block = 6476622998065200121;
                        break;
                    } else {
                        p = ep_0.offset(1 as libc::c_int as isize);
                    }
                }
                43 => {
                    s = s.offset(1);
                    s;
                    current_block = 417003359394161306;
                    break;
                }
                42 => {
                    current_block = 417003359394161306;
                    break;
                }
                45 => {
                    s = min_expand(ms, s, p, ep_0);
                    current_block = 6476622998065200121;
                    break;
                }
                _ => {
                    s = s.offset(1);
                    s;
                    p = ep_0;
                }
            }
        }
    }
    match current_block {
        417003359394161306 => {
            s = max_expand(ms, s, p, ep_0);
        }
        _ => {}
    }
    (*ms).matchdepth += 1;
    (*ms).matchdepth;
    return s;
}
unsafe extern "C" fn lmemfind(
    mut s1: *const libc::c_char,
    mut l1: size_t,
    mut s2: *const libc::c_char,
    mut l2: size_t,
) -> *const libc::c_char {
    if l2 == 0 as libc::c_int as libc::c_ulong {
        return s1
    } else if l2 > l1 {
        return 0 as *const libc::c_char
    } else {
        let mut init: *const libc::c_char = 0 as *const libc::c_char;
        l2 = l2.wrapping_sub(1);
        l2;
        l1 = l1.wrapping_sub(l2);
        while l1 > 0 as libc::c_int as libc::c_ulong
            && {
                init = memchr(s1 as *const libc::c_void, *s2 as libc::c_int, l1)
                    as *const libc::c_char;
                !init.is_null()
            }
        {
            init = init.offset(1);
            init;
            if memcmp(
                init as *const libc::c_void,
                s2.offset(1 as libc::c_int as isize) as *const libc::c_void,
                l2,
            ) == 0 as libc::c_int
            {
                return init.offset(-(1 as libc::c_int as isize))
            } else {
                l1 = (l1 as libc::c_ulong)
                    .wrapping_sub(init.offset_from(s1) as libc::c_long as libc::c_ulong)
                    as size_t as size_t;
                s1 = init;
            }
        }
        return 0 as *const libc::c_char;
    };
}
unsafe extern "C" fn get_onecapture(
    mut ms: *mut MatchState,
    mut i: libc::c_int,
    mut s: *const libc::c_char,
    mut e: *const libc::c_char,
    mut cap: *mut *const libc::c_char,
) -> size_t {
    if i >= (*ms).level as libc::c_int {
        if ((i != 0 as libc::c_int) as libc::c_int != 0 as libc::c_int) as libc::c_int
            as libc::c_long != 0
        {
            luaL_error(
                (*ms).L,
                b"invalid capture index %%%d\0" as *const u8 as *const libc::c_char,
                i + 1 as libc::c_int,
            );
        }
        *cap = s;
        return e.offset_from(s) as libc::c_long as size_t;
    } else {
        let mut capl: ptrdiff_t = (*ms).capture[i as usize].len;
        *cap = (*ms).capture[i as usize].init;
        if ((capl == -(1 as libc::c_int) as libc::c_long) as libc::c_int
            != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
        {
            luaL_error(
                (*ms).L,
                b"unfinished capture\0" as *const u8 as *const libc::c_char,
            );
        } else if capl == -(2 as libc::c_int) as libc::c_long {
            lua_pushinteger(
                (*ms).L,
                (((*ms).capture[i as usize].init).offset_from((*ms).src_init)
                    as libc::c_long + 1 as libc::c_int as libc::c_long) as lua_Integer,
            );
        }
        return capl as size_t;
    };
}
unsafe extern "C" fn push_onecapture(
    mut ms: *mut MatchState,
    mut i: libc::c_int,
    mut s: *const libc::c_char,
    mut e: *const libc::c_char,
) {
    let mut cap: *const libc::c_char = 0 as *const libc::c_char;
    let mut l: ptrdiff_t = get_onecapture(ms, i, s, e, &mut cap) as ptrdiff_t;
    if l != -(2 as libc::c_int) as libc::c_long {
        lua_pushlstring((*ms).L, cap, l as size_t);
    }
}
unsafe extern "C" fn push_captures(
    mut ms: *mut MatchState,
    mut s: *const libc::c_char,
    mut e: *const libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut nlevels: libc::c_int = if (*ms).level as libc::c_int == 0 as libc::c_int
        && !s.is_null()
    {
        1 as libc::c_int
    } else {
        (*ms).level as libc::c_int
    };
    luaL_checkstack(
        (*ms).L,
        nlevels,
        b"too many captures\0" as *const u8 as *const libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < nlevels {
        push_onecapture(ms, i, s, e);
        i += 1;
        i;
    }
    return nlevels;
}
unsafe extern "C" fn nospecials(
    mut p: *const libc::c_char,
    mut l: size_t,
) -> libc::c_int {
    let mut upto: size_t = 0 as libc::c_int as size_t;
    loop {
        if !(strpbrk(
            p.offset(upto as isize),
            b"^$*+?.([%-\0" as *const u8 as *const libc::c_char,
        ))
            .is_null()
        {
            return 0 as libc::c_int;
        }
        upto = (upto as libc::c_ulong)
            .wrapping_add(
                (strlen(p.offset(upto as isize)))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as size_t as size_t;
        if !(upto <= l) {
            break;
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn prepstate(
    mut ms: *mut MatchState,
    mut L: *mut lua_State,
    mut s: *const libc::c_char,
    mut ls: size_t,
    mut p: *const libc::c_char,
    mut lp: size_t,
) {
    (*ms).L = L;
    (*ms).matchdepth = 200 as libc::c_int;
    (*ms).src_init = s;
    (*ms).src_end = s.offset(ls as isize);
    (*ms).p_end = p.offset(lp as isize);
}
unsafe extern "C" fn reprepstate(mut ms: *mut MatchState) {
    (*ms).level = 0 as libc::c_int as libc::c_uchar;
}
unsafe extern "C" fn str_find_aux(
    mut L: *mut lua_State,
    mut find: libc::c_int,
) -> libc::c_int {
    let mut ls: size_t = 0;
    let mut lp: size_t = 0;
    let mut s: *const libc::c_char = luaL_checklstring(L, 1 as libc::c_int, &mut ls);
    let mut p: *const libc::c_char = luaL_checklstring(L, 2 as libc::c_int, &mut lp);
    let mut init: size_t = (posrelatI(
        luaL_optinteger(L, 3 as libc::c_int, 1 as libc::c_int as lua_Integer),
        ls,
    ))
        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    if init > ls {
        lua_pushnil(L);
        return 1 as libc::c_int;
    }
    if find != 0 && (lua_toboolean(L, 4 as libc::c_int) != 0 || nospecials(p, lp) != 0) {
        let mut s2: *const libc::c_char = lmemfind(
            s.offset(init as isize),
            ls.wrapping_sub(init),
            p,
            lp,
        );
        if !s2.is_null() {
            lua_pushinteger(
                L,
                (s2.offset_from(s) as libc::c_long + 1 as libc::c_int as libc::c_long)
                    as lua_Integer,
            );
            lua_pushinteger(
                L,
                (s2.offset_from(s) as libc::c_long as libc::c_ulong).wrapping_add(lp)
                    as lua_Integer,
            );
            return 2 as libc::c_int;
        }
    } else {
        let mut ms: MatchState = MatchState {
            src_init: 0 as *const libc::c_char,
            src_end: 0 as *const libc::c_char,
            p_end: 0 as *const libc::c_char,
            L: 0 as *mut lua_State,
            matchdepth: 0,
            level: 0,
            capture: [C2RustUnnamed_3 {
                init: 0 as *const libc::c_char,
                len: 0,
            }; 32],
        };
        let mut s1: *const libc::c_char = s.offset(init as isize);
        let mut anchor: libc::c_int = (*p as libc::c_int == '^' as i32) as libc::c_int;
        if anchor != 0 {
            p = p.offset(1);
            p;
            lp = lp.wrapping_sub(1);
            lp;
        }
        prepstate(&mut ms, L, s, ls, p, lp);
        loop {
            let mut res: *const libc::c_char = 0 as *const libc::c_char;
            reprepstate(&mut ms);
            res = match_0(&mut ms, s1, p);
            if !res.is_null() {
                if find != 0 {
                    lua_pushinteger(
                        L,
                        (s1.offset_from(s) as libc::c_long
                            + 1 as libc::c_int as libc::c_long) as lua_Integer,
                    );
                    lua_pushinteger(
                        L,
                        res.offset_from(s) as libc::c_long as lua_Integer,
                    );
                    return push_captures(
                        &mut ms,
                        0 as *const libc::c_char,
                        0 as *const libc::c_char,
                    ) + 2 as libc::c_int;
                } else {
                    return push_captures(&mut ms, s1, res)
                }
            }
            let fresh4 = s1;
            s1 = s1.offset(1);
            if !(fresh4 < ms.src_end && anchor == 0) {
                break;
            }
        }
    }
    lua_pushnil(L);
    return 1 as libc::c_int;
}
unsafe extern "C" fn str_find(mut L: *mut lua_State) -> libc::c_int {
    return str_find_aux(L, 1 as libc::c_int);
}
unsafe extern "C" fn str_match(mut L: *mut lua_State) -> libc::c_int {
    return str_find_aux(L, 0 as libc::c_int);
}
unsafe extern "C" fn gmatch_aux(mut L: *mut lua_State) -> libc::c_int {
    let mut gm: *mut GMatchState = lua_touserdata(
        L,
        -(1000000 as libc::c_int) - 1000 as libc::c_int - 3 as libc::c_int,
    ) as *mut GMatchState;
    let mut src: *const libc::c_char = 0 as *const libc::c_char;
    (*gm).ms.L = L;
    src = (*gm).src;
    while src <= (*gm).ms.src_end {
        let mut e: *const libc::c_char = 0 as *const libc::c_char;
        reprepstate(&mut (*gm).ms);
        e = match_0(&mut (*gm).ms, src, (*gm).p);
        if !e.is_null() && e != (*gm).lastmatch {
            (*gm).lastmatch = e;
            (*gm).src = (*gm).lastmatch;
            return push_captures(&mut (*gm).ms, src, e);
        }
        src = src.offset(1);
        src;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn gmatch(mut L: *mut lua_State) -> libc::c_int {
    let mut ls: size_t = 0;
    let mut lp: size_t = 0;
    let mut s: *const libc::c_char = luaL_checklstring(L, 1 as libc::c_int, &mut ls);
    let mut p: *const libc::c_char = luaL_checklstring(L, 2 as libc::c_int, &mut lp);
    let mut init: size_t = (posrelatI(
        luaL_optinteger(L, 3 as libc::c_int, 1 as libc::c_int as lua_Integer),
        ls,
    ))
        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    let mut gm: *mut GMatchState = 0 as *mut GMatchState;
    lua_settop(L, 2 as libc::c_int);
    gm = lua_newuserdatauv(
        L,
        ::core::mem::size_of::<GMatchState>() as libc::c_ulong,
        0 as libc::c_int,
    ) as *mut GMatchState;
    if init > ls {
        init = ls.wrapping_add(1 as libc::c_int as libc::c_ulong);
    }
    prepstate(&mut (*gm).ms, L, s, ls, p, lp);
    (*gm).src = s.offset(init as isize);
    (*gm).p = p;
    (*gm).lastmatch = 0 as *const libc::c_char;
    lua_pushcclosure(
        L,
        Some(gmatch_aux as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
        3 as libc::c_int,
    );
    return 1 as libc::c_int;
}
unsafe extern "C" fn add_s(
    mut ms: *mut MatchState,
    mut b: *mut luaL_Buffer,
    mut s: *const libc::c_char,
    mut e: *const libc::c_char,
) {
    let mut l: size_t = 0;
    let mut L: *mut lua_State = (*ms).L;
    let mut news: *const libc::c_char = lua_tolstring(L, 3 as libc::c_int, &mut l);
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    loop {
        p = memchr(news as *const libc::c_void, '%' as i32, l) as *mut libc::c_char;
        if p.is_null() {
            break;
        }
        luaL_addlstring(b, news, p.offset_from(news) as libc::c_long as size_t);
        p = p.offset(1);
        p;
        if *p as libc::c_int == '%' as i32 {
            ((*b).n < (*b).size
                || !(luaL_prepbuffsize(b, 1 as libc::c_int as size_t)).is_null())
                as libc::c_int;
            let fresh5 = (*b).n;
            (*b).n = ((*b).n).wrapping_add(1);
            *((*b).b).offset(fresh5 as isize) = *p;
        } else if *p as libc::c_int == '0' as i32 {
            luaL_addlstring(b, s, e.offset_from(s) as libc::c_long as size_t);
        } else if *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            let mut cap: *const libc::c_char = 0 as *const libc::c_char;
            let mut resl: ptrdiff_t = get_onecapture(
                ms,
                *p as libc::c_int - '1' as i32,
                s,
                e,
                &mut cap,
            ) as ptrdiff_t;
            if resl == -(2 as libc::c_int) as libc::c_long {
                luaL_addvalue(b);
            } else {
                luaL_addlstring(b, cap, resl as size_t);
            }
        } else {
            luaL_error(
                L,
                b"invalid use of '%c' in replacement string\0" as *const u8
                    as *const libc::c_char,
                '%' as i32,
            );
        }
        l = (l as libc::c_ulong)
            .wrapping_sub(
                p.offset(1 as libc::c_int as isize).offset_from(news) as libc::c_long
                    as libc::c_ulong,
            ) as size_t as size_t;
        news = p.offset(1 as libc::c_int as isize);
    }
    luaL_addlstring(b, news, l);
}
unsafe extern "C" fn add_value(
    mut ms: *mut MatchState,
    mut b: *mut luaL_Buffer,
    mut s: *const libc::c_char,
    mut e: *const libc::c_char,
    mut tr: libc::c_int,
) -> libc::c_int {
    let mut L: *mut lua_State = (*ms).L;
    match tr {
        6 => {
            let mut n: libc::c_int = 0;
            lua_pushvalue(L, 3 as libc::c_int);
            n = push_captures(ms, s, e);
            lua_callk(L, n, 1 as libc::c_int, 0 as libc::c_int as lua_KContext, None);
        }
        5 => {
            push_onecapture(ms, 0 as libc::c_int, s, e);
            lua_gettable(L, 3 as libc::c_int);
        }
        _ => {
            add_s(ms, b, s, e);
            return 1 as libc::c_int;
        }
    }
    if lua_toboolean(L, -(1 as libc::c_int)) == 0 {
        lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
        luaL_addlstring(b, s, e.offset_from(s) as libc::c_long as size_t);
        return 0 as libc::c_int;
    } else if ((lua_isstring(L, -(1 as libc::c_int)) == 0) as libc::c_int
        != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
    {
        return luaL_error(
            L,
            b"invalid replacement value (a %s)\0" as *const u8 as *const libc::c_char,
            lua_typename(L, lua_type(L, -(1 as libc::c_int))),
        )
    } else {
        luaL_addvalue(b);
        return 1 as libc::c_int;
    };
}
unsafe extern "C" fn str_gsub(mut L: *mut lua_State) -> libc::c_int {
    let mut srcl: size_t = 0;
    let mut lp: size_t = 0;
    let mut src: *const libc::c_char = luaL_checklstring(L, 1 as libc::c_int, &mut srcl);
    let mut p: *const libc::c_char = luaL_checklstring(L, 2 as libc::c_int, &mut lp);
    let mut lastmatch: *const libc::c_char = 0 as *const libc::c_char;
    let mut tr: libc::c_int = lua_type(L, 3 as libc::c_int);
    let mut max_s: lua_Integer = luaL_optinteger(
        L,
        4 as libc::c_int,
        srcl.wrapping_add(1 as libc::c_int as libc::c_ulong) as lua_Integer,
    );
    let mut anchor: libc::c_int = (*p as libc::c_int == '^' as i32) as libc::c_int;
    let mut n: lua_Integer = 0 as libc::c_int as lua_Integer;
    let mut changed: libc::c_int = 0 as libc::c_int;
    let mut ms: MatchState = MatchState {
        src_init: 0 as *const libc::c_char,
        src_end: 0 as *const libc::c_char,
        p_end: 0 as *const libc::c_char,
        L: 0 as *mut lua_State,
        matchdepth: 0,
        level: 0,
        capture: [C2RustUnnamed_3 {
            init: 0 as *const libc::c_char,
            len: 0,
        }; 32],
    };
    let mut b: luaL_Buffer = luaL_Buffer {
        b: 0 as *mut libc::c_char,
        size: 0,
        n: 0,
        L: 0 as *mut lua_State,
        init: C2RustUnnamed_0 { n: 0. },
    };
    (((tr == 3 as libc::c_int || tr == 4 as libc::c_int || tr == 6 as libc::c_int
        || tr == 5 as libc::c_int) as libc::c_int != 0 as libc::c_int) as libc::c_int
        as libc::c_long != 0
        || luaL_typeerror(
            L,
            3 as libc::c_int,
            b"string/function/table\0" as *const u8 as *const libc::c_char,
        ) != 0) as libc::c_int;
    luaL_buffinit(L, &mut b);
    if anchor != 0 {
        p = p.offset(1);
        p;
        lp = lp.wrapping_sub(1);
        lp;
    }
    prepstate(&mut ms, L, src, srcl, p, lp);
    while n < max_s {
        let mut e: *const libc::c_char = 0 as *const libc::c_char;
        reprepstate(&mut ms);
        e = match_0(&mut ms, src, p);
        if !e.is_null() && e != lastmatch {
            n += 1;
            n;
            changed = add_value(&mut ms, &mut b, src, e, tr) | changed;
            lastmatch = e;
            src = lastmatch;
        } else {
            if !(src < ms.src_end) {
                break;
            }
            (b.n < b.size
                || !(luaL_prepbuffsize(&mut b, 1 as libc::c_int as size_t)).is_null())
                as libc::c_int;
            let fresh6 = src;
            src = src.offset(1);
            let fresh7 = b.n;
            b.n = (b.n).wrapping_add(1);
            *(b.b).offset(fresh7 as isize) = *fresh6;
        }
        if anchor != 0 {
            break;
        }
    }
    if changed == 0 {
        lua_pushvalue(L, 1 as libc::c_int);
    } else {
        luaL_addlstring(
            &mut b,
            src,
            (ms.src_end).offset_from(src) as libc::c_long as size_t,
        );
        luaL_pushresult(&mut b);
    }
    lua_pushinteger(L, n);
    return 2 as libc::c_int;
}
unsafe extern "C" fn addquoted(
    mut b: *mut luaL_Buffer,
    mut s: *const libc::c_char,
    mut len: size_t,
) {
    ((*b).n < (*b).size || !(luaL_prepbuffsize(b, 1 as libc::c_int as size_t)).is_null())
        as libc::c_int;
    let fresh8 = (*b).n;
    (*b).n = ((*b).n).wrapping_add(1);
    *((*b).b).offset(fresh8 as isize) = '"' as i32 as libc::c_char;
    loop {
        let fresh9 = len;
        len = len.wrapping_sub(1);
        if !(fresh9 != 0) {
            break;
        }
        if *s as libc::c_int == '"' as i32 || *s as libc::c_int == '\\' as i32
            || *s as libc::c_int == '\n' as i32
        {
            ((*b).n < (*b).size
                || !(luaL_prepbuffsize(b, 1 as libc::c_int as size_t)).is_null())
                as libc::c_int;
            let fresh10 = (*b).n;
            (*b).n = ((*b).n).wrapping_add(1);
            *((*b).b).offset(fresh10 as isize) = '\\' as i32 as libc::c_char;
            ((*b).n < (*b).size
                || !(luaL_prepbuffsize(b, 1 as libc::c_int as size_t)).is_null())
                as libc::c_int;
            let fresh11 = (*b).n;
            (*b).n = ((*b).n).wrapping_add(1);
            *((*b).b).offset(fresh11 as isize) = *s;
        } else if *(*__ctype_b_loc()).offset(*s as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _IScntrl as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            let mut buff: [libc::c_char; 10] = [0; 10];
            if *(*__ctype_b_loc())
                .offset(
                    *s.offset(1 as libc::c_int as isize) as libc::c_uchar as libc::c_int
                        as isize,
                ) as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
            {
                snprintf(
                    buff.as_mut_ptr(),
                    ::core::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong,
                    b"\\%d\0" as *const u8 as *const libc::c_char,
                    *s as libc::c_uchar as libc::c_int,
                );
            } else {
                snprintf(
                    buff.as_mut_ptr(),
                    ::core::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong,
                    b"\\%03d\0" as *const u8 as *const libc::c_char,
                    *s as libc::c_uchar as libc::c_int,
                );
            }
            luaL_addstring(b, buff.as_mut_ptr());
        } else {
            ((*b).n < (*b).size
                || !(luaL_prepbuffsize(b, 1 as libc::c_int as size_t)).is_null())
                as libc::c_int;
            let fresh12 = (*b).n;
            (*b).n = ((*b).n).wrapping_add(1);
            *((*b).b).offset(fresh12 as isize) = *s;
        }
        s = s.offset(1);
        s;
    }
    ((*b).n < (*b).size || !(luaL_prepbuffsize(b, 1 as libc::c_int as size_t)).is_null())
        as libc::c_int;
    let fresh13 = (*b).n;
    (*b).n = ((*b).n).wrapping_add(1);
    *((*b).b).offset(fresh13 as isize) = '"' as i32 as libc::c_char;
}
unsafe extern "C" fn quotefloat(
    mut L: *mut lua_State,
    mut buff: *mut libc::c_char,
    mut n: lua_Number,
) -> libc::c_int {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    if n == ::core::f64::INFINITY {
        s = b"1e9999\0" as *const u8 as *const libc::c_char;
    } else if n == -::core::f64::INFINITY {
        s = b"-1e9999\0" as *const u8 as *const libc::c_char;
    } else if n != n {
        s = b"(0/0)\0" as *const u8 as *const libc::c_char;
    } else {
        let mut nb: libc::c_int = snprintf(
            buff,
            120 as libc::c_int as libc::c_ulong,
            b"%a\0" as *const u8 as *const libc::c_char,
            n,
        );
        if (memchr(buff as *const libc::c_void, '.' as i32, nb as libc::c_ulong))
            .is_null()
        {
            let mut point: libc::c_char = *((*localeconv()).decimal_point)
                .offset(0 as libc::c_int as isize);
            let mut ppoint: *mut libc::c_char = memchr(
                buff as *const libc::c_void,
                point as libc::c_int,
                nb as libc::c_ulong,
            ) as *mut libc::c_char;
            if !ppoint.is_null() {
                *ppoint = '.' as i32 as libc::c_char;
            }
        }
        return nb;
    }
    return snprintf(
        buff,
        120 as libc::c_int as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        s,
    );
}
unsafe extern "C" fn addliteral(
    mut L: *mut lua_State,
    mut b: *mut luaL_Buffer,
    mut arg: libc::c_int,
) {
    match lua_type(L, arg) {
        4 => {
            let mut len: size_t = 0;
            let mut s: *const libc::c_char = lua_tolstring(L, arg, &mut len);
            addquoted(b, s, len);
        }
        3 => {
            let mut buff: *mut libc::c_char = luaL_prepbuffsize(
                b,
                120 as libc::c_int as size_t,
            );
            let mut nb: libc::c_int = 0;
            if lua_isinteger(L, arg) == 0 {
                nb = quotefloat(L, buff, lua_tonumberx(L, arg, 0 as *mut libc::c_int));
            } else {
                let mut n: lua_Integer = lua_tointegerx(L, arg, 0 as *mut libc::c_int);
                let mut format: *const libc::c_char = if n
                    == -(9223372036854775807 as libc::c_longlong) - 1 as libc::c_longlong
                {
                    b"0x%llx\0" as *const u8 as *const libc::c_char
                } else {
                    b"%lld\0" as *const u8 as *const libc::c_char
                };
                nb = snprintf(buff, 120 as libc::c_int as libc::c_ulong, format, n);
            }
            (*b)
                .n = ((*b).n as libc::c_ulong).wrapping_add(nb as libc::c_ulong)
                as size_t as size_t;
        }
        0 | 1 => {
            luaL_tolstring(L, arg, 0 as *mut size_t);
            luaL_addvalue(b);
        }
        _ => {
            luaL_argerror(
                L,
                arg,
                b"value has no literal form\0" as *const u8 as *const libc::c_char,
            );
        }
    };
}
unsafe extern "C" fn get2digits(mut s: *const libc::c_char) -> *const libc::c_char {
    if *(*__ctype_b_loc()).offset(*s as libc::c_uchar as libc::c_int as isize)
        as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        s = s.offset(1);
        s;
        if *(*__ctype_b_loc()).offset(*s as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            s = s.offset(1);
            s;
        }
    }
    return s;
}
unsafe extern "C" fn checkformat(
    mut L: *mut lua_State,
    mut form: *const libc::c_char,
    mut flags: *const libc::c_char,
    mut precision: libc::c_int,
) {
    let mut spec: *const libc::c_char = form.offset(1 as libc::c_int as isize);
    spec = spec.offset(strspn(spec, flags) as isize);
    if *spec as libc::c_int != '0' as i32 {
        spec = get2digits(spec);
        if *spec as libc::c_int == '.' as i32 && precision != 0 {
            spec = spec.offset(1);
            spec;
            spec = get2digits(spec);
        }
    }
    if *(*__ctype_b_loc()).offset(*spec as libc::c_uchar as libc::c_int as isize)
        as libc::c_int & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int == 0
    {
        luaL_error(
            L,
            b"invalid conversion specification: '%s'\0" as *const u8
                as *const libc::c_char,
            form,
        );
    }
}
unsafe extern "C" fn getformat(
    mut L: *mut lua_State,
    mut strfrmt: *const libc::c_char,
    mut form: *mut libc::c_char,
) -> *const libc::c_char {
    let mut len: size_t = strspn(
        strfrmt,
        b"-+#0 123456789.\0" as *const u8 as *const libc::c_char,
    );
    len = len.wrapping_add(1);
    len;
    if len >= (32 as libc::c_int - 10 as libc::c_int) as libc::c_ulong {
        luaL_error(
            L,
            b"invalid format (too long)\0" as *const u8 as *const libc::c_char,
        );
    }
    let fresh14 = form;
    form = form.offset(1);
    *fresh14 = '%' as i32 as libc::c_char;
    memcpy(
        form as *mut libc::c_void,
        strfrmt as *const libc::c_void,
        len.wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
    );
    *form.offset(len as isize) = '\0' as i32 as libc::c_char;
    return strfrmt.offset(len as isize).offset(-(1 as libc::c_int as isize));
}
unsafe extern "C" fn addlenmod(
    mut form: *mut libc::c_char,
    mut lenmod: *const libc::c_char,
) {
    let mut l: size_t = strlen(form);
    let mut lm: size_t = strlen(lenmod);
    let mut spec: libc::c_char = *form
        .offset(l.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
    strcpy(form.offset(l as isize).offset(-(1 as libc::c_int as isize)), lenmod);
    *form
        .offset(
            l.wrapping_add(lm).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        ) = spec;
    *form.offset(l.wrapping_add(lm) as isize) = '\0' as i32 as libc::c_char;
}
unsafe extern "C" fn str_format(mut L: *mut lua_State) -> libc::c_int {
    let mut current_block: u64;
    let mut top: libc::c_int = lua_gettop(L);
    let mut arg: libc::c_int = 1 as libc::c_int;
    let mut sfl: size_t = 0;
    let mut strfrmt: *const libc::c_char = luaL_checklstring(L, arg, &mut sfl);
    let mut strfrmt_end: *const libc::c_char = strfrmt.offset(sfl as isize);
    let mut flags: *const libc::c_char = 0 as *const libc::c_char;
    let mut b: luaL_Buffer = luaL_Buffer {
        b: 0 as *mut libc::c_char,
        size: 0,
        n: 0,
        L: 0 as *mut lua_State,
        init: C2RustUnnamed_0 { n: 0. },
    };
    luaL_buffinit(L, &mut b);
    while strfrmt < strfrmt_end {
        if *strfrmt as libc::c_int != '%' as i32 {
            (b.n < b.size
                || !(luaL_prepbuffsize(&mut b, 1 as libc::c_int as size_t)).is_null())
                as libc::c_int;
            let fresh15 = strfrmt;
            strfrmt = strfrmt.offset(1);
            let fresh16 = b.n;
            b.n = (b.n).wrapping_add(1);
            *(b.b).offset(fresh16 as isize) = *fresh15;
        } else {
            strfrmt = strfrmt.offset(1);
            if *strfrmt as libc::c_int == '%' as i32 {
                (b.n < b.size
                    || !(luaL_prepbuffsize(&mut b, 1 as libc::c_int as size_t))
                        .is_null()) as libc::c_int;
                let fresh17 = strfrmt;
                strfrmt = strfrmt.offset(1);
                let fresh18 = b.n;
                b.n = (b.n).wrapping_add(1);
                *(b.b).offset(fresh18 as isize) = *fresh17;
            } else {
                let mut form: [libc::c_char; 32] = [0; 32];
                let mut maxitem: libc::c_int = 120 as libc::c_int;
                let mut buff: *mut libc::c_char = luaL_prepbuffsize(
                    &mut b,
                    maxitem as size_t,
                );
                let mut nb: libc::c_int = 0 as libc::c_int;
                arg += 1;
                if arg > top {
                    return luaL_argerror(
                        L,
                        arg,
                        b"no value\0" as *const u8 as *const libc::c_char,
                    );
                }
                strfrmt = getformat(L, strfrmt, form.as_mut_ptr());
                let fresh19 = strfrmt;
                strfrmt = strfrmt.offset(1);
                match *fresh19 as libc::c_int {
                    99 => {
                        checkformat(
                            L,
                            form.as_mut_ptr(),
                            b"-\0" as *const u8 as *const libc::c_char,
                            0 as libc::c_int,
                        );
                        nb = snprintf(
                            buff,
                            maxitem as libc::c_ulong,
                            form.as_mut_ptr(),
                            luaL_checkinteger(L, arg) as libc::c_int,
                        );
                        current_block = 11793792312832361944;
                    }
                    100 | 105 => {
                        flags = b"-+0 \0" as *const u8 as *const libc::c_char;
                        current_block = 5689001924483802034;
                    }
                    117 => {
                        flags = b"-0\0" as *const u8 as *const libc::c_char;
                        current_block = 5689001924483802034;
                    }
                    111 | 120 | 88 => {
                        flags = b"-#0\0" as *const u8 as *const libc::c_char;
                        current_block = 5689001924483802034;
                    }
                    97 | 65 => {
                        checkformat(
                            L,
                            form.as_mut_ptr(),
                            b"-+#0 \0" as *const u8 as *const libc::c_char,
                            1 as libc::c_int,
                        );
                        addlenmod(
                            form.as_mut_ptr(),
                            b"\0" as *const u8 as *const libc::c_char,
                        );
                        nb = snprintf(
                            buff,
                            maxitem as libc::c_ulong,
                            form.as_mut_ptr(),
                            luaL_checknumber(L, arg),
                        );
                        current_block = 11793792312832361944;
                    }
                    102 => {
                        maxitem = 110 as libc::c_int + 308 as libc::c_int;
                        buff = luaL_prepbuffsize(&mut b, maxitem as size_t);
                        current_block = 6669252993407410313;
                    }
                    101 | 69 | 103 | 71 => {
                        current_block = 6669252993407410313;
                    }
                    112 => {
                        let mut p: *const libc::c_void = lua_topointer(L, arg);
                        checkformat(
                            L,
                            form.as_mut_ptr(),
                            b"-\0" as *const u8 as *const libc::c_char,
                            0 as libc::c_int,
                        );
                        if p.is_null() {
                            p = b"(null)\0" as *const u8 as *const libc::c_char
                                as *const libc::c_void;
                            form[(strlen(form.as_mut_ptr()))
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                as usize] = 's' as i32 as libc::c_char;
                        }
                        nb = snprintf(
                            buff,
                            maxitem as libc::c_ulong,
                            form.as_mut_ptr(),
                            p,
                        );
                        current_block = 11793792312832361944;
                    }
                    113 => {
                        if form[2 as libc::c_int as usize] as libc::c_int != '\0' as i32
                        {
                            return luaL_error(
                                L,
                                b"specifier '%%q' cannot have modifiers\0" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                        addliteral(L, &mut b, arg);
                        current_block = 11793792312832361944;
                    }
                    115 => {
                        let mut l: size_t = 0;
                        let mut s: *const libc::c_char = luaL_tolstring(L, arg, &mut l);
                        if form[2 as libc::c_int as usize] as libc::c_int == '\0' as i32
                        {
                            luaL_addvalue(&mut b);
                        } else {
                            (((l == strlen(s)) as libc::c_int != 0 as libc::c_int)
                                as libc::c_int as libc::c_long != 0
                                || luaL_argerror(
                                    L,
                                    arg,
                                    b"string contains zeros\0" as *const u8
                                        as *const libc::c_char,
                                ) != 0) as libc::c_int;
                            checkformat(
                                L,
                                form.as_mut_ptr(),
                                b"-\0" as *const u8 as *const libc::c_char,
                                1 as libc::c_int,
                            );
                            if (strchr(form.as_mut_ptr(), '.' as i32)).is_null()
                                && l >= 100 as libc::c_int as libc::c_ulong
                            {
                                luaL_addvalue(&mut b);
                            } else {
                                nb = snprintf(
                                    buff,
                                    maxitem as libc::c_ulong,
                                    form.as_mut_ptr(),
                                    s,
                                );
                                lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
                            }
                        }
                        current_block = 11793792312832361944;
                    }
                    _ => {
                        return luaL_error(
                            L,
                            b"invalid conversion '%s' to 'format'\0" as *const u8
                                as *const libc::c_char,
                            form.as_mut_ptr(),
                        );
                    }
                }
                match current_block {
                    5689001924483802034 => {
                        let mut n: lua_Integer = luaL_checkinteger(L, arg);
                        checkformat(L, form.as_mut_ptr(), flags, 1 as libc::c_int);
                        addlenmod(
                            form.as_mut_ptr(),
                            b"ll\0" as *const u8 as *const libc::c_char,
                        );
                        nb = snprintf(
                            buff,
                            maxitem as libc::c_ulong,
                            form.as_mut_ptr(),
                            n,
                        );
                    }
                    6669252993407410313 => {
                        let mut n_0: lua_Number = luaL_checknumber(L, arg);
                        checkformat(
                            L,
                            form.as_mut_ptr(),
                            b"-+#0 \0" as *const u8 as *const libc::c_char,
                            1 as libc::c_int,
                        );
                        addlenmod(
                            form.as_mut_ptr(),
                            b"\0" as *const u8 as *const libc::c_char,
                        );
                        nb = snprintf(
                            buff,
                            maxitem as libc::c_ulong,
                            form.as_mut_ptr(),
                            n_0,
                        );
                    }
                    _ => {}
                }
                b
                    .n = (b.n as libc::c_ulong).wrapping_add(nb as libc::c_ulong)
                    as size_t as size_t;
            }
        }
    }
    luaL_pushresult(&mut b);
    return 1 as libc::c_int;
}
static mut nativeendian: C2RustUnnamed_1 = C2RustUnnamed_1 {
    dummy: 1 as libc::c_int,
};
unsafe extern "C" fn digit(mut c: libc::c_int) -> libc::c_int {
    return ('0' as i32 <= c && c <= '9' as i32) as libc::c_int;
}
unsafe extern "C" fn getnum(
    mut fmt: *mut *const libc::c_char,
    mut df: libc::c_int,
) -> libc::c_int {
    if digit(**fmt as libc::c_int) == 0 {
        return df
    } else {
        let mut a: libc::c_int = 0 as libc::c_int;
        loop {
            let fresh20 = *fmt;
            *fmt = (*fmt).offset(1);
            a = a * 10 as libc::c_int + (*fresh20 as libc::c_int - '0' as i32);
            if !(digit(**fmt as libc::c_int) != 0
                && a
                    <= ((if (::core::mem::size_of::<size_t>() as libc::c_ulong)
                        < ::core::mem::size_of::<libc::c_int>() as libc::c_ulong
                    {
                        !(0 as libc::c_int as size_t)
                    } else {
                        2147483647 as libc::c_int as size_t
                    }) as libc::c_int - 9 as libc::c_int) / 10 as libc::c_int)
            {
                break;
            }
        }
        return a;
    };
}
unsafe extern "C" fn getnumlimit(
    mut h: *mut Header,
    mut fmt: *mut *const libc::c_char,
    mut df: libc::c_int,
) -> libc::c_int {
    let mut sz: libc::c_int = getnum(fmt, df);
    if ((sz > 16 as libc::c_int || sz <= 0 as libc::c_int) as libc::c_int
        != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
    {
        return luaL_error(
            (*h).L,
            b"integral size (%d) out of limits [1,%d]\0" as *const u8
                as *const libc::c_char,
            sz,
            16 as libc::c_int,
        );
    }
    return sz;
}
unsafe extern "C" fn initheader(mut L: *mut lua_State, mut h: *mut Header) {
    (*h).L = L;
    (*h).islittle = nativeendian.little as libc::c_int;
    (*h).maxalign = 1 as libc::c_int;
}
unsafe extern "C" fn getoption(
    mut h: *mut Header,
    mut fmt: *mut *const libc::c_char,
    mut size: *mut libc::c_int,
) -> KOption {
    let fresh21 = *fmt;
    *fmt = (*fmt).offset(1);
    let mut opt: libc::c_int = *fresh21 as libc::c_int;
    *size = 0 as libc::c_int;
    match opt {
        98 => {
            *size = ::core::mem::size_of::<libc::c_char>() as libc::c_ulong
                as libc::c_int;
            return Kint;
        }
        66 => {
            *size = ::core::mem::size_of::<libc::c_char>() as libc::c_ulong
                as libc::c_int;
            return Kuint;
        }
        104 => {
            *size = ::core::mem::size_of::<libc::c_short>() as libc::c_ulong
                as libc::c_int;
            return Kint;
        }
        72 => {
            *size = ::core::mem::size_of::<libc::c_short>() as libc::c_ulong
                as libc::c_int;
            return Kuint;
        }
        108 => {
            *size = ::core::mem::size_of::<libc::c_long>() as libc::c_ulong
                as libc::c_int;
            return Kint;
        }
        76 => {
            *size = ::core::mem::size_of::<libc::c_long>() as libc::c_ulong
                as libc::c_int;
            return Kuint;
        }
        106 => {
            *size = ::core::mem::size_of::<lua_Integer>() as libc::c_ulong
                as libc::c_int;
            return Kint;
        }
        74 => {
            *size = ::core::mem::size_of::<lua_Integer>() as libc::c_ulong
                as libc::c_int;
            return Kuint;
        }
        84 => {
            *size = ::core::mem::size_of::<size_t>() as libc::c_ulong as libc::c_int;
            return Kuint;
        }
        102 => {
            *size = ::core::mem::size_of::<libc::c_float>() as libc::c_ulong
                as libc::c_int;
            return Kfloat;
        }
        110 => {
            *size = ::core::mem::size_of::<lua_Number>() as libc::c_ulong as libc::c_int;
            return Knumber;
        }
        100 => {
            *size = ::core::mem::size_of::<libc::c_double>() as libc::c_ulong
                as libc::c_int;
            return Kdouble;
        }
        105 => {
            *size = getnumlimit(
                h,
                fmt,
                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
            );
            return Kint;
        }
        73 => {
            *size = getnumlimit(
                h,
                fmt,
                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
            );
            return Kuint;
        }
        115 => {
            *size = getnumlimit(
                h,
                fmt,
                ::core::mem::size_of::<size_t>() as libc::c_ulong as libc::c_int,
            );
            return Kstring;
        }
        99 => {
            *size = getnum(fmt, -(1 as libc::c_int));
            if ((*size == -(1 as libc::c_int)) as libc::c_int != 0 as libc::c_int)
                as libc::c_int as libc::c_long != 0
            {
                luaL_error(
                    (*h).L,
                    b"missing size for format option 'c'\0" as *const u8
                        as *const libc::c_char,
                );
            }
            return Kchar;
        }
        122 => return Kzstr,
        120 => {
            *size = 1 as libc::c_int;
            return Kpadding;
        }
        88 => return Kpaddalign,
        32 => {}
        60 => {
            (*h).islittle = 1 as libc::c_int;
        }
        62 => {
            (*h).islittle = 0 as libc::c_int;
        }
        61 => {
            (*h).islittle = nativeendian.little as libc::c_int;
        }
        33 => {
            let maxalign: libc::c_int = 8 as libc::c_ulong as libc::c_int;
            (*h).maxalign = getnumlimit(h, fmt, maxalign);
        }
        _ => {
            luaL_error(
                (*h).L,
                b"invalid format option '%c'\0" as *const u8 as *const libc::c_char,
                opt,
            );
        }
    }
    return Knop;
}
unsafe extern "C" fn getdetails(
    mut h: *mut Header,
    mut totalsize: size_t,
    mut fmt: *mut *const libc::c_char,
    mut psize: *mut libc::c_int,
    mut ntoalign: *mut libc::c_int,
) -> KOption {
    let mut opt: KOption = getoption(h, fmt, psize);
    let mut align: libc::c_int = *psize;
    if opt as libc::c_uint == Kpaddalign as libc::c_int as libc::c_uint {
        if **fmt as libc::c_int == '\0' as i32
            || getoption(h, fmt, &mut align) as libc::c_uint
                == Kchar as libc::c_int as libc::c_uint || align == 0 as libc::c_int
        {
            luaL_argerror(
                (*h).L,
                1 as libc::c_int,
                b"invalid next option for option 'X'\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    if align <= 1 as libc::c_int
        || opt as libc::c_uint == Kchar as libc::c_int as libc::c_uint
    {
        *ntoalign = 0 as libc::c_int;
    } else {
        if align > (*h).maxalign {
            align = (*h).maxalign;
        }
        if ((align & align - 1 as libc::c_int != 0 as libc::c_int) as libc::c_int
            != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
        {
            luaL_argerror(
                (*h).L,
                1 as libc::c_int,
                b"format asks for alignment not power of 2\0" as *const u8
                    as *const libc::c_char,
            );
        }
        *ntoalign = align
            - (totalsize & (align - 1 as libc::c_int) as libc::c_ulong) as libc::c_int
            & align - 1 as libc::c_int;
    }
    return opt;
}
unsafe extern "C" fn packint(
    mut b: *mut luaL_Buffer,
    mut n: lua_Unsigned,
    mut islittle: libc::c_int,
    mut size: libc::c_int,
    mut neg: libc::c_int,
) {
    let mut buff: *mut libc::c_char = luaL_prepbuffsize(b, size as size_t);
    let mut i: libc::c_int = 0;
    *buff
        .offset(
            (if islittle != 0 { 0 as libc::c_int } else { size - 1 as libc::c_int })
                as isize,
        ) = (n
        & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
            as libc::c_ulonglong) as libc::c_char;
    i = 1 as libc::c_int;
    while i < size {
        n >>= 8 as libc::c_int;
        *buff
            .offset(
                (if islittle != 0 { i } else { size - 1 as libc::c_int - i }) as isize,
            ) = (n
            & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                as libc::c_ulonglong) as libc::c_char;
        i += 1;
        i;
    }
    if neg != 0
        && size > ::core::mem::size_of::<lua_Integer>() as libc::c_ulong as libc::c_int
    {
        i = ::core::mem::size_of::<lua_Integer>() as libc::c_ulong as libc::c_int;
        while i < size {
            *buff
                .offset(
                    (if islittle != 0 { i } else { size - 1 as libc::c_int - i })
                        as isize,
                ) = (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                as libc::c_char;
            i += 1;
            i;
        }
    }
    (*b)
        .n = ((*b).n as libc::c_ulong).wrapping_add(size as libc::c_ulong) as size_t
        as size_t;
}
unsafe extern "C" fn copywithendian(
    mut dest: *mut libc::c_char,
    mut src: *const libc::c_char,
    mut size: libc::c_int,
    mut islittle: libc::c_int,
) {
    if islittle == nativeendian.little as libc::c_int {
        memcpy(
            dest as *mut libc::c_void,
            src as *const libc::c_void,
            size as libc::c_ulong,
        );
    } else {
        dest = dest.offset((size - 1 as libc::c_int) as isize);
        loop {
            let fresh22 = size;
            size = size - 1;
            if !(fresh22 != 0 as libc::c_int) {
                break;
            }
            let fresh23 = src;
            src = src.offset(1);
            let fresh24 = dest;
            dest = dest.offset(-1);
            *fresh24 = *fresh23;
        }
    };
}
unsafe extern "C" fn str_pack(mut L: *mut lua_State) -> libc::c_int {
    let mut b: luaL_Buffer = luaL_Buffer {
        b: 0 as *mut libc::c_char,
        size: 0,
        n: 0,
        L: 0 as *mut lua_State,
        init: C2RustUnnamed_0 { n: 0. },
    };
    let mut h: Header = Header {
        L: 0 as *mut lua_State,
        islittle: 0,
        maxalign: 0,
    };
    let mut fmt: *const libc::c_char = luaL_checklstring(
        L,
        1 as libc::c_int,
        0 as *mut size_t,
    );
    let mut arg: libc::c_int = 1 as libc::c_int;
    let mut totalsize: size_t = 0 as libc::c_int as size_t;
    initheader(L, &mut h);
    lua_pushnil(L);
    luaL_buffinit(L, &mut b);
    while *fmt as libc::c_int != '\0' as i32 {
        let mut size: libc::c_int = 0;
        let mut ntoalign: libc::c_int = 0;
        let mut opt: KOption = getdetails(
            &mut h,
            totalsize,
            &mut fmt,
            &mut size,
            &mut ntoalign,
        );
        totalsize = (totalsize as libc::c_ulong)
            .wrapping_add((ntoalign + size) as libc::c_ulong) as size_t as size_t;
        loop {
            let fresh25 = ntoalign;
            ntoalign = ntoalign - 1;
            if !(fresh25 > 0 as libc::c_int) {
                break;
            }
            (b.n < b.size
                || !(luaL_prepbuffsize(&mut b, 1 as libc::c_int as size_t)).is_null())
                as libc::c_int;
            let fresh26 = b.n;
            b.n = (b.n).wrapping_add(1);
            *(b.b).offset(fresh26 as isize) = 0 as libc::c_int as libc::c_char;
        }
        arg += 1;
        arg;
        let mut current_block_33: u64;
        match opt as libc::c_uint {
            0 => {
                let mut n: lua_Integer = luaL_checkinteger(L, arg);
                if size
                    < ::core::mem::size_of::<lua_Integer>() as libc::c_ulong
                        as libc::c_int
                {
                    let mut lim: lua_Integer = (1 as libc::c_int as lua_Integer)
                        << size * 8 as libc::c_int - 1 as libc::c_int;
                    (((-lim <= n && n < lim) as libc::c_int != 0 as libc::c_int)
                        as libc::c_int as libc::c_long != 0
                        || luaL_argerror(
                            L,
                            arg,
                            b"integer overflow\0" as *const u8 as *const libc::c_char,
                        ) != 0) as libc::c_int;
                }
                packint(
                    &mut b,
                    n as lua_Unsigned,
                    h.islittle,
                    size,
                    (n < 0 as libc::c_int as libc::c_longlong) as libc::c_int,
                );
                current_block_33 = 3222590281903869779;
            }
            1 => {
                let mut n_0: lua_Integer = luaL_checkinteger(L, arg);
                if size
                    < ::core::mem::size_of::<lua_Integer>() as libc::c_ulong
                        as libc::c_int
                {
                    ((((n_0 as lua_Unsigned)
                        < (1 as libc::c_int as lua_Unsigned) << size * 8 as libc::c_int)
                        as libc::c_int != 0 as libc::c_int) as libc::c_int
                        as libc::c_long != 0
                        || luaL_argerror(
                            L,
                            arg,
                            b"unsigned overflow\0" as *const u8 as *const libc::c_char,
                        ) != 0) as libc::c_int;
                }
                packint(&mut b, n_0 as lua_Unsigned, h.islittle, size, 0 as libc::c_int);
                current_block_33 = 3222590281903869779;
            }
            2 => {
                let mut f: libc::c_float = luaL_checknumber(L, arg) as libc::c_float;
                let mut buff: *mut libc::c_char = luaL_prepbuffsize(
                    &mut b,
                    ::core::mem::size_of::<libc::c_float>() as libc::c_ulong,
                );
                copywithendian(
                    buff,
                    &mut f as *mut libc::c_float as *mut libc::c_char,
                    ::core::mem::size_of::<libc::c_float>() as libc::c_ulong
                        as libc::c_int,
                    h.islittle,
                );
                b
                    .n = (b.n as libc::c_ulong).wrapping_add(size as libc::c_ulong)
                    as size_t as size_t;
                current_block_33 = 3222590281903869779;
            }
            3 => {
                let mut f_0: lua_Number = luaL_checknumber(L, arg);
                let mut buff_0: *mut libc::c_char = luaL_prepbuffsize(
                    &mut b,
                    ::core::mem::size_of::<lua_Number>() as libc::c_ulong,
                );
                copywithendian(
                    buff_0,
                    &mut f_0 as *mut lua_Number as *mut libc::c_char,
                    ::core::mem::size_of::<lua_Number>() as libc::c_ulong as libc::c_int,
                    h.islittle,
                );
                b
                    .n = (b.n as libc::c_ulong).wrapping_add(size as libc::c_ulong)
                    as size_t as size_t;
                current_block_33 = 3222590281903869779;
            }
            4 => {
                let mut f_1: libc::c_double = luaL_checknumber(L, arg);
                let mut buff_1: *mut libc::c_char = luaL_prepbuffsize(
                    &mut b,
                    ::core::mem::size_of::<libc::c_double>() as libc::c_ulong,
                );
                copywithendian(
                    buff_1,
                    &mut f_1 as *mut libc::c_double as *mut libc::c_char,
                    ::core::mem::size_of::<libc::c_double>() as libc::c_ulong
                        as libc::c_int,
                    h.islittle,
                );
                b
                    .n = (b.n as libc::c_ulong).wrapping_add(size as libc::c_ulong)
                    as size_t as size_t;
                current_block_33 = 3222590281903869779;
            }
            5 => {
                let mut len: size_t = 0;
                let mut s: *const libc::c_char = luaL_checklstring(L, arg, &mut len);
                (((len <= size as size_t) as libc::c_int != 0 as libc::c_int)
                    as libc::c_int as libc::c_long != 0
                    || luaL_argerror(
                        L,
                        arg,
                        b"string longer than given size\0" as *const u8
                            as *const libc::c_char,
                    ) != 0) as libc::c_int;
                luaL_addlstring(&mut b, s, len);
                loop {
                    let fresh27 = len;
                    len = len.wrapping_add(1);
                    if !(fresh27 < size as size_t) {
                        break;
                    }
                    (b.n < b.size
                        || !(luaL_prepbuffsize(&mut b, 1 as libc::c_int as size_t))
                            .is_null()) as libc::c_int;
                    let fresh28 = b.n;
                    b.n = (b.n).wrapping_add(1);
                    *(b.b).offset(fresh28 as isize) = 0 as libc::c_int as libc::c_char;
                }
                current_block_33 = 3222590281903869779;
            }
            6 => {
                let mut len_0: size_t = 0;
                let mut s_0: *const libc::c_char = luaL_checklstring(L, arg, &mut len_0);
                (((size
                    >= ::core::mem::size_of::<size_t>() as libc::c_ulong as libc::c_int
                    || len_0 < (1 as libc::c_int as size_t) << size * 8 as libc::c_int)
                    as libc::c_int != 0 as libc::c_int) as libc::c_int as libc::c_long
                    != 0
                    || luaL_argerror(
                        L,
                        arg,
                        b"string length does not fit in given size\0" as *const u8
                            as *const libc::c_char,
                    ) != 0) as libc::c_int;
                packint(
                    &mut b,
                    len_0 as lua_Unsigned,
                    h.islittle,
                    size,
                    0 as libc::c_int,
                );
                luaL_addlstring(&mut b, s_0, len_0);
                totalsize = (totalsize as libc::c_ulong).wrapping_add(len_0) as size_t
                    as size_t;
                current_block_33 = 3222590281903869779;
            }
            7 => {
                let mut len_1: size_t = 0;
                let mut s_1: *const libc::c_char = luaL_checklstring(L, arg, &mut len_1);
                (((strlen(s_1) == len_1) as libc::c_int != 0 as libc::c_int)
                    as libc::c_int as libc::c_long != 0
                    || luaL_argerror(
                        L,
                        arg,
                        b"string contains zeros\0" as *const u8 as *const libc::c_char,
                    ) != 0) as libc::c_int;
                luaL_addlstring(&mut b, s_1, len_1);
                (b.n < b.size
                    || !(luaL_prepbuffsize(&mut b, 1 as libc::c_int as size_t))
                        .is_null()) as libc::c_int;
                let fresh29 = b.n;
                b.n = (b.n).wrapping_add(1);
                *(b.b).offset(fresh29 as isize) = '\0' as i32 as libc::c_char;
                totalsize = (totalsize as libc::c_ulong)
                    .wrapping_add(len_1.wrapping_add(1 as libc::c_int as libc::c_ulong))
                    as size_t as size_t;
                current_block_33 = 3222590281903869779;
            }
            8 => {
                (b.n < b.size
                    || !(luaL_prepbuffsize(&mut b, 1 as libc::c_int as size_t))
                        .is_null()) as libc::c_int;
                let fresh30 = b.n;
                b.n = (b.n).wrapping_add(1);
                *(b.b).offset(fresh30 as isize) = 0 as libc::c_int as libc::c_char;
                current_block_33 = 16285809747685596942;
            }
            9 | 10 => {
                current_block_33 = 16285809747685596942;
            }
            _ => {
                current_block_33 = 3222590281903869779;
            }
        }
        match current_block_33 {
            16285809747685596942 => {
                arg -= 1;
                arg;
            }
            _ => {}
        }
    }
    luaL_pushresult(&mut b);
    return 1 as libc::c_int;
}
unsafe extern "C" fn str_packsize(mut L: *mut lua_State) -> libc::c_int {
    let mut h: Header = Header {
        L: 0 as *mut lua_State,
        islittle: 0,
        maxalign: 0,
    };
    let mut fmt: *const libc::c_char = luaL_checklstring(
        L,
        1 as libc::c_int,
        0 as *mut size_t,
    );
    let mut totalsize: size_t = 0 as libc::c_int as size_t;
    initheader(L, &mut h);
    while *fmt as libc::c_int != '\0' as i32 {
        let mut size: libc::c_int = 0;
        let mut ntoalign: libc::c_int = 0;
        let mut opt: KOption = getdetails(
            &mut h,
            totalsize,
            &mut fmt,
            &mut size,
            &mut ntoalign,
        );
        (((opt as libc::c_uint != Kstring as libc::c_int as libc::c_uint
            && opt as libc::c_uint != Kzstr as libc::c_int as libc::c_uint)
            as libc::c_int != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
            || luaL_argerror(
                L,
                1 as libc::c_int,
                b"variable-length format\0" as *const u8 as *const libc::c_char,
            ) != 0) as libc::c_int;
        size += ntoalign;
        (((totalsize
            <= (if (::core::mem::size_of::<size_t>() as libc::c_ulong)
                < ::core::mem::size_of::<libc::c_int>() as libc::c_ulong
            {
                !(0 as libc::c_int as size_t)
            } else {
                2147483647 as libc::c_int as size_t
            })
                .wrapping_sub(size as libc::c_ulong)) as libc::c_int != 0 as libc::c_int)
            as libc::c_int as libc::c_long != 0
            || luaL_argerror(
                L,
                1 as libc::c_int,
                b"format result too large\0" as *const u8 as *const libc::c_char,
            ) != 0) as libc::c_int;
        totalsize = (totalsize as libc::c_ulong).wrapping_add(size as libc::c_ulong)
            as size_t as size_t;
    }
    lua_pushinteger(L, totalsize as lua_Integer);
    return 1 as libc::c_int;
}
unsafe extern "C" fn unpackint(
    mut L: *mut lua_State,
    mut str: *const libc::c_char,
    mut islittle: libc::c_int,
    mut size: libc::c_int,
    mut issigned: libc::c_int,
) -> lua_Integer {
    let mut res: lua_Unsigned = 0 as libc::c_int as lua_Unsigned;
    let mut i: libc::c_int = 0;
    let mut limit: libc::c_int = if size
        <= ::core::mem::size_of::<lua_Integer>() as libc::c_ulong as libc::c_int
    {
        size
    } else {
        ::core::mem::size_of::<lua_Integer>() as libc::c_ulong as libc::c_int
    };
    i = limit - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        res <<= 8 as libc::c_int;
        res
            |= *str
                .offset(
                    (if islittle != 0 { i } else { size - 1 as libc::c_int - i })
                        as isize,
                ) as libc::c_uchar as lua_Unsigned;
        i -= 1;
        i;
    }
    if size < ::core::mem::size_of::<lua_Integer>() as libc::c_ulong as libc::c_int {
        if issigned != 0 {
            let mut mask: lua_Unsigned = (1 as libc::c_int as lua_Unsigned)
                << size * 8 as libc::c_int - 1 as libc::c_int;
            res = (res ^ mask).wrapping_sub(mask);
        }
    } else if size
        > ::core::mem::size_of::<lua_Integer>() as libc::c_ulong as libc::c_int
    {
        let mut mask_0: libc::c_int = if issigned == 0
            || res as lua_Integer >= 0 as libc::c_int as libc::c_longlong
        {
            0 as libc::c_int
        } else {
            ((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int
        };
        i = limit;
        while i < size {
            if ((*str
                .offset(
                    (if islittle != 0 { i } else { size - 1 as libc::c_int - i })
                        as isize,
                ) as libc::c_uchar as libc::c_int != mask_0) as libc::c_int
                != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
            {
                luaL_error(
                    L,
                    b"%d-byte integer does not fit into Lua Integer\0" as *const u8
                        as *const libc::c_char,
                    size,
                );
            }
            i += 1;
            i;
        }
    }
    return res as lua_Integer;
}
unsafe extern "C" fn str_unpack(mut L: *mut lua_State) -> libc::c_int {
    let mut h: Header = Header {
        L: 0 as *mut lua_State,
        islittle: 0,
        maxalign: 0,
    };
    let mut fmt: *const libc::c_char = luaL_checklstring(
        L,
        1 as libc::c_int,
        0 as *mut size_t,
    );
    let mut ld: size_t = 0;
    let mut data: *const libc::c_char = luaL_checklstring(L, 2 as libc::c_int, &mut ld);
    let mut pos: size_t = (posrelatI(
        luaL_optinteger(L, 3 as libc::c_int, 1 as libc::c_int as lua_Integer),
        ld,
    ))
        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    let mut n: libc::c_int = 0 as libc::c_int;
    (((pos <= ld) as libc::c_int != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
        || luaL_argerror(
            L,
            3 as libc::c_int,
            b"initial position out of string\0" as *const u8 as *const libc::c_char,
        ) != 0) as libc::c_int;
    initheader(L, &mut h);
    while *fmt as libc::c_int != '\0' as i32 {
        let mut size: libc::c_int = 0;
        let mut ntoalign: libc::c_int = 0;
        let mut opt: KOption = getdetails(
            &mut h,
            pos,
            &mut fmt,
            &mut size,
            &mut ntoalign,
        );
        ((((ntoalign as size_t).wrapping_add(size as libc::c_ulong)
            <= ld.wrapping_sub(pos)) as libc::c_int != 0 as libc::c_int) as libc::c_int
            as libc::c_long != 0
            || luaL_argerror(
                L,
                2 as libc::c_int,
                b"data string too short\0" as *const u8 as *const libc::c_char,
            ) != 0) as libc::c_int;
        pos = (pos as libc::c_ulong).wrapping_add(ntoalign as libc::c_ulong) as size_t
            as size_t;
        luaL_checkstack(
            L,
            2 as libc::c_int,
            b"too many results\0" as *const u8 as *const libc::c_char,
        );
        n += 1;
        n;
        match opt as libc::c_uint {
            0 | 1 => {
                let mut res: lua_Integer = unpackint(
                    L,
                    data.offset(pos as isize),
                    h.islittle,
                    size,
                    (opt as libc::c_uint == Kint as libc::c_int as libc::c_uint)
                        as libc::c_int,
                );
                lua_pushinteger(L, res);
            }
            2 => {
                let mut f: libc::c_float = 0.;
                copywithendian(
                    &mut f as *mut libc::c_float as *mut libc::c_char,
                    data.offset(pos as isize),
                    ::core::mem::size_of::<libc::c_float>() as libc::c_ulong
                        as libc::c_int,
                    h.islittle,
                );
                lua_pushnumber(L, f as lua_Number);
            }
            3 => {
                let mut f_0: lua_Number = 0.;
                copywithendian(
                    &mut f_0 as *mut lua_Number as *mut libc::c_char,
                    data.offset(pos as isize),
                    ::core::mem::size_of::<lua_Number>() as libc::c_ulong as libc::c_int,
                    h.islittle,
                );
                lua_pushnumber(L, f_0);
            }
            4 => {
                let mut f_1: libc::c_double = 0.;
                copywithendian(
                    &mut f_1 as *mut libc::c_double as *mut libc::c_char,
                    data.offset(pos as isize),
                    ::core::mem::size_of::<libc::c_double>() as libc::c_ulong
                        as libc::c_int,
                    h.islittle,
                );
                lua_pushnumber(L, f_1);
            }
            5 => {
                lua_pushlstring(L, data.offset(pos as isize), size as size_t);
            }
            6 => {
                let mut len: size_t = unpackint(
                    L,
                    data.offset(pos as isize),
                    h.islittle,
                    size,
                    0 as libc::c_int,
                ) as size_t;
                (((len <= ld.wrapping_sub(pos).wrapping_sub(size as libc::c_ulong))
                    as libc::c_int != 0 as libc::c_int) as libc::c_int as libc::c_long
                    != 0
                    || luaL_argerror(
                        L,
                        2 as libc::c_int,
                        b"data string too short\0" as *const u8 as *const libc::c_char,
                    ) != 0) as libc::c_int;
                lua_pushlstring(L, data.offset(pos as isize).offset(size as isize), len);
                pos = (pos as libc::c_ulong).wrapping_add(len) as size_t as size_t;
            }
            7 => {
                let mut len_0: size_t = strlen(data.offset(pos as isize));
                (((pos.wrapping_add(len_0) < ld) as libc::c_int != 0 as libc::c_int)
                    as libc::c_int as libc::c_long != 0
                    || luaL_argerror(
                        L,
                        2 as libc::c_int,
                        b"unfinished string for format 'z'\0" as *const u8
                            as *const libc::c_char,
                    ) != 0) as libc::c_int;
                lua_pushlstring(L, data.offset(pos as isize), len_0);
                pos = (pos as libc::c_ulong)
                    .wrapping_add(len_0.wrapping_add(1 as libc::c_int as libc::c_ulong))
                    as size_t as size_t;
            }
            9 | 8 | 10 => {
                n -= 1;
                n;
            }
            _ => {}
        }
        pos = (pos as libc::c_ulong).wrapping_add(size as libc::c_ulong) as size_t
            as size_t;
    }
    lua_pushinteger(
        L,
        pos.wrapping_add(1 as libc::c_int as libc::c_ulong) as lua_Integer,
    );
    return n + 1 as libc::c_int;
}
static mut strlib: [luaL_Reg; 18] = unsafe {
    [
        {
            let mut init = luaL_Reg {
                name: b"byte\0" as *const u8 as *const libc::c_char,
                func: Some(
                    str_byte as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"char\0" as *const u8 as *const libc::c_char,
                func: Some(
                    str_char as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"dump\0" as *const u8 as *const libc::c_char,
                func: Some(
                    str_dump as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"find\0" as *const u8 as *const libc::c_char,
                func: Some(
                    str_find as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"format\0" as *const u8 as *const libc::c_char,
                func: Some(
                    str_format as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"gmatch\0" as *const u8 as *const libc::c_char,
                func: Some(gmatch as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"gsub\0" as *const u8 as *const libc::c_char,
                func: Some(
                    str_gsub as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"len\0" as *const u8 as *const libc::c_char,
                func: Some(
                    str_len as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"lower\0" as *const u8 as *const libc::c_char,
                func: Some(
                    str_lower as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"match\0" as *const u8 as *const libc::c_char,
                func: Some(
                    str_match as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"rep\0" as *const u8 as *const libc::c_char,
                func: Some(
                    str_rep as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"reverse\0" as *const u8 as *const libc::c_char,
                func: Some(
                    str_reverse as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"sub\0" as *const u8 as *const libc::c_char,
                func: Some(
                    str_sub as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"upper\0" as *const u8 as *const libc::c_char,
                func: Some(
                    str_upper as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"pack\0" as *const u8 as *const libc::c_char,
                func: Some(
                    str_pack as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"packsize\0" as *const u8 as *const libc::c_char,
                func: Some(
                    str_packsize as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"unpack\0" as *const u8 as *const libc::c_char,
                func: Some(
                    str_unpack as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
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
unsafe extern "C" fn createmetatable(mut L: *mut lua_State) {
    lua_createtable(
        L,
        0 as libc::c_int,
        (::core::mem::size_of::<[luaL_Reg; 10]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<luaL_Reg>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
    );
    luaL_setfuncs(L, stringmetamethods.as_ptr(), 0 as libc::c_int);
    lua_pushstring(L, b"\0" as *const u8 as *const libc::c_char);
    lua_pushvalue(L, -(2 as libc::c_int));
    lua_setmetatable(L, -(2 as libc::c_int));
    lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
    lua_pushvalue(L, -(2 as libc::c_int));
    lua_setfield(
        L,
        -(2 as libc::c_int),
        b"__index\0" as *const u8 as *const libc::c_char,
    );
    lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaopen_string(mut L: *mut lua_State) -> libc::c_int {
    luaL_checkversion_(
        L,
        504 as libc::c_int as lua_Number,
        (::core::mem::size_of::<lua_Integer>() as libc::c_ulong)
            .wrapping_mul(16 as libc::c_int as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<lua_Number>() as libc::c_ulong),
    );
    lua_createtable(
        L,
        0 as libc::c_int,
        (::core::mem::size_of::<[luaL_Reg; 18]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<luaL_Reg>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
    );
    luaL_setfuncs(L, strlib.as_ptr(), 0 as libc::c_int);
    createmetatable(L);
    return 1 as libc::c_int;
}
