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
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn lua_gettop(L: *mut lua_State) -> i32;
    fn lua_settop(L: *mut lua_State, index: i32);
    fn lua_pushvalue(L: *mut lua_State, index: i32);
    fn lua_rotate(L: *mut lua_State, index: i32, n: i32);
    fn lua_checkstack(L: *mut lua_State, n: i32) -> i32;
    fn lua_isstring(L: *mut lua_State, index: i32) -> i32;
    fn lua_type(L: *mut lua_State, index: i32) -> i32;
    fn lua_typename(L: *mut lua_State, tp: i32) -> *const libc::c_char;
    fn lua_toboolean(L: *mut lua_State, index: i32) -> i32;
    fn lua_compare(
        L: *mut lua_State,
        index1: i32,
        index2: i32,
        op: i32,
    ) -> i32;
    fn lua_pushnil(L: *mut lua_State);
    fn lua_pushinteger(L: *mut lua_State, n: Integer);
    fn lua_pushstring(L: *mut lua_State, s: *const libc::c_char) -> *const libc::c_char;
    fn lua_geti(L: *mut lua_State, index: i32, n: Integer) -> i32;
    fn lua_rawget(L: *mut lua_State, index: i32) -> i32;
    fn lua_createtable(L: *mut lua_State, narr: i32, nrec: i32);
    fn lua_getmetatable(L: *mut lua_State, objindex: i32) -> i32;
    fn lua_setfield(L: *mut lua_State, index: i32, k: *const libc::c_char);
    fn lua_seti(L: *mut lua_State, index: i32, n: Integer);
    fn lua_callk(
        L: *mut lua_State,
        nargs: i32,
        nresults: i32,
        ctx: lua_KContext,
        k: lua_KFunction,
    );
    fn luaL_checkversion_(L: *mut lua_State, ver: Number, sz: size_t);
    fn luaL_argerror(
        L: *mut lua_State,
        arg: i32,
        extramsg: *const libc::c_char,
    ) -> i32;
    fn luaL_optlstring(
        L: *mut lua_State,
        arg: i32,
        def: *const libc::c_char,
        l: *mut size_t,
    ) -> *const libc::c_char;
    fn luaL_checkinteger(L: *mut lua_State, arg: i32) -> Integer;
    fn luaL_optinteger(
        L: *mut lua_State,
        arg: i32,
        def: Integer,
    ) -> Integer;
    fn luaL_checktype(L: *mut lua_State, arg: i32, t: i32);
    fn luaL_error(L: *mut lua_State, fmt: *const libc::c_char, _: ...) -> i32;
    fn luaL_len(L: *mut lua_State, index: i32) -> Integer;
    fn luaL_setfuncs(L: *mut lua_State, l: *const luaL_Reg, nup: i32);
    fn luaL_buffinit(L: *mut lua_State, B: *mut luaL_Buffer);
    fn luaL_addlstring(B: *mut luaL_Buffer, s: *const libc::c_char, l: size_t);
    fn luaL_addvalue(B: *mut luaL_Buffer);
    fn luaL_pushresult(B: *mut luaL_Buffer);
    fn clock() -> clock_t;
    fn time(__timer: *mut time_t) -> time_t;
}
pub type size_t = libc::c_ulong;
pub type __clock_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type intptr_t = libc::c_long;
pub type Number = f64;
pub type Integer = i64;
pub type lua_Unsigned = libc::c_ulonglong;
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
pub type IdxT = libc::c_uint;
pub type time_t = __time_t;
pub type clock_t = __clock_t;
unsafe extern "C" fn checkfield(
    mut L: *mut lua_State,
    mut key: *const libc::c_char,
    mut n: i32,
) -> i32 {
    lua_pushstring(L, key);
    return (lua_rawget(L, -n) != 0 as i32) as i32;
}
unsafe extern "C" fn checktab(
    mut L: *mut lua_State,
    mut arg: i32,
    mut what: i32,
) {
    if lua_type(L, arg) != 5 as i32 {
        let mut n: i32 = 1 as i32;
        if lua_getmetatable(L, arg) != 0
            && (what & 1 as i32 == 0
                || {
                    n += 1;
                    checkfield(L, b"__index\0" as *const u8 as *const libc::c_char, n)
                        != 0
                })
            && (what & 2 as i32 == 0
                || {
                    n += 1;
                    checkfield(L, b"__newindex\0" as *const u8 as *const libc::c_char, n)
                        != 0
                })
            && (what & 4 as i32 == 0
                || {
                    n += 1;
                    checkfield(L, b"__len\0" as *const u8 as *const libc::c_char, n) != 0
                })
        {
            lua_settop(L, -n - 1 as i32);
        } else {
            luaL_checktype(L, arg, 5 as i32);
        }
    }
}
unsafe extern "C" fn tinsert(mut L: *mut lua_State) -> i32 {
    let mut pos: Integer = 0;
    checktab(
        L,
        1 as i32,
        1 as i32 | 2 as i32 | 4 as i32,
    );
    let mut e: Integer = luaL_len(L, 1 as i32);
    e = (e as lua_Unsigned).wrapping_add(1 as i32 as lua_Unsigned)
        as Integer;
    match lua_gettop(L) {
        2 => {
            pos = e;
        }
        3 => {
            let mut i: Integer = 0;
            pos = luaL_checkinteger(L, 2 as i32);
            ((((pos as lua_Unsigned).wrapping_sub(1 as libc::c_uint as libc::c_ulonglong)
                < e as lua_Unsigned) as i32 != 0 as i32) as i32
                as libc::c_long != 0
                || luaL_argerror(
                    L,
                    2 as i32,
                    b"position out of bounds\0" as *const u8 as *const libc::c_char,
                ) != 0) as i32;
            i = e;
            while i > pos {
                lua_geti(L, 1 as i32, i - 1 as i32 as i64);
                lua_seti(L, 1 as i32, i);
                i -= 1;
                i;
            }
        }
        _ => {
            return luaL_error(
                L,
                b"wrong number of arguments to 'insert'\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    lua_seti(L, 1 as i32, pos);
    return 0 as i32;
}
unsafe extern "C" fn tremove(mut L: *mut lua_State) -> i32 {
    checktab(
        L,
        1 as i32,
        1 as i32 | 2 as i32 | 4 as i32,
    );
    let mut size: Integer = luaL_len(L, 1 as i32);
    let mut pos: Integer = luaL_optinteger(L, 2 as i32, size);
    if pos != size {
        ((((pos as lua_Unsigned).wrapping_sub(1 as libc::c_uint as libc::c_ulonglong)
            <= size as lua_Unsigned) as i32 != 0 as i32) as i32
            as libc::c_long != 0
            || luaL_argerror(
                L,
                2 as i32,
                b"position out of bounds\0" as *const u8 as *const libc::c_char,
            ) != 0) as i32;
    }
    lua_geti(L, 1 as i32, pos);
    while pos < size {
        lua_geti(L, 1 as i32, pos + 1 as i32 as i64);
        lua_seti(L, 1 as i32, pos);
        pos += 1;
        pos;
    }
    lua_pushnil(L);
    lua_seti(L, 1 as i32, pos);
    return 1 as i32;
}
unsafe extern "C" fn tmove(mut L: *mut lua_State) -> i32 {
    let mut f: Integer = luaL_checkinteger(L, 2 as i32);
    let mut e: Integer = luaL_checkinteger(L, 3 as i32);
    let mut t: Integer = luaL_checkinteger(L, 4 as i32);
    let mut tt: i32 = if !(lua_type(L, 5 as i32) <= 0 as i32) {
        5 as i32
    } else {
        1 as i32
    };
    checktab(L, 1 as i32, 1 as i32);
    checktab(L, tt, 2 as i32);
    if e >= f {
        let mut n: Integer = 0;
        let mut i: Integer = 0;
        (((f > 0 as i32 as i64
            || e < 9223372036854775807 as i64 + f) as i32
            != 0 as i32) as i32 as libc::c_long != 0
            || luaL_argerror(
                L,
                3 as i32,
                b"too many elements to move\0" as *const u8 as *const libc::c_char,
            ) != 0) as i32;
        n = e - f + 1 as i32 as i64;
        (((t
            <= 9223372036854775807 as i64 - n
                + 1 as i32 as i64) as i32
            != 0 as i32) as i32 as libc::c_long != 0
            || luaL_argerror(
                L,
                4 as i32,
                b"destination wrap around\0" as *const u8 as *const libc::c_char,
            ) != 0) as i32;
        if t > e || t <= f
            || tt != 1 as i32
                && lua_compare(L, 1 as i32, tt, 0 as i32) == 0
        {
            i = 0 as i32 as Integer;
            while i < n {
                lua_geti(L, 1 as i32, f + i);
                lua_seti(L, tt, t + i);
                i += 1;
                i;
            }
        } else {
            i = n - 1 as i32 as i64;
            while i >= 0 as i32 as i64 {
                lua_geti(L, 1 as i32, f + i);
                lua_seti(L, tt, t + i);
                i -= 1;
                i;
            }
        }
    }
    lua_pushvalue(L, tt);
    return 1 as i32;
}
unsafe extern "C" fn addfield(
    mut L: *mut lua_State,
    mut b: *mut luaL_Buffer,
    mut i: Integer,
) {
    lua_geti(L, 1 as i32, i);
    if ((lua_isstring(L, -(1 as i32)) == 0) as i32 != 0 as i32)
        as i32 as libc::c_long != 0
    {
        luaL_error(
            L,
            b"invalid value (%s) at index %I in table for 'concat'\0" as *const u8
                as *const libc::c_char,
            lua_typename(L, lua_type(L, -(1 as i32))),
            i,
        );
    }
    luaL_addvalue(b);
}
unsafe extern "C" fn tconcat(mut L: *mut lua_State) -> i32 {
    let mut b: luaL_Buffer = luaL_Buffer {
        b: 0 as *mut libc::c_char,
        size: 0,
        n: 0,
        L: 0 as *mut lua_State,
        init: C2RustUnnamed { n: 0. },
    };
    checktab(L, 1 as i32, 1 as i32 | 4 as i32);
    let mut last: Integer = luaL_len(L, 1 as i32);
    let mut lsep: size_t = 0;
    let mut sep: *const libc::c_char = luaL_optlstring(
        L,
        2 as i32,
        b"\0" as *const u8 as *const libc::c_char,
        &mut lsep,
    );
    let mut i: Integer = luaL_optinteger(
        L,
        3 as i32,
        1 as i32 as Integer,
    );
    last = luaL_optinteger(L, 4 as i32, last);
    luaL_buffinit(L, &mut b);
    while i < last {
        addfield(L, &mut b, i);
        luaL_addlstring(&mut b, sep, lsep);
        i += 1;
        i;
    }
    if i == last {
        addfield(L, &mut b, i);
    }
    luaL_pushresult(&mut b);
    return 1 as i32;
}
unsafe extern "C" fn tpack(mut L: *mut lua_State) -> i32 {
    let mut i: i32 = 0;
    let mut n: i32 = lua_gettop(L);
    lua_createtable(L, n, 1 as i32);
    lua_rotate(L, 1 as i32, 1 as i32);
    i = n;
    while i >= 1 as i32 {
        lua_seti(L, 1 as i32, i as Integer);
        i -= 1;
        i;
    }
    lua_pushinteger(L, n as Integer);
    lua_setfield(L, 1 as i32, b"n\0" as *const u8 as *const libc::c_char);
    return 1 as i32;
}
unsafe extern "C" fn tunpack(mut L: *mut lua_State) -> i32 {
    let mut n: lua_Unsigned = 0;
    let mut i: Integer = luaL_optinteger(
        L,
        2 as i32,
        1 as i32 as Integer,
    );
    let mut e: Integer = if lua_type(L, 3 as i32) <= 0 as i32 {
        luaL_len(L, 1 as i32)
    } else {
        luaL_checkinteger(L, 3 as i32)
    };
    if i > e {
        return 0 as i32;
    }
    n = (e as lua_Unsigned).wrapping_sub(i as libc::c_ulonglong);
    if ((n >= 2147483647 as i32 as libc::c_uint as libc::c_ulonglong
        || {
            n = n.wrapping_add(1);
            lua_checkstack(L, n as i32) == 0
        }) as i32 != 0 as i32) as i32 as libc::c_long != 0
    {
        return luaL_error(
            L,
            b"too many results to unpack\0" as *const u8 as *const libc::c_char,
        );
    }
    while i < e {
        lua_geti(L, 1 as i32, i);
        i += 1;
        i;
    }
    lua_geti(L, 1 as i32, e);
    return n as i32;
}
unsafe extern "C" fn l_randomizePivot() -> libc::c_uint {
    let mut c: clock_t = clock();
    let mut t: time_t = time(0 as *mut time_t);
    let mut buff: [libc::c_uint; 4] = [0; 4];
    let mut i: libc::c_uint = 0;
    let mut rnd: libc::c_uint = 0 as i32 as libc::c_uint;
    memcpy(
        buff.as_mut_ptr() as *mut libc::c_void,
        &mut c as *mut clock_t as *const libc::c_void,
        (::core::mem::size_of::<clock_t>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<libc::c_uint>() as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_uint>() as libc::c_ulong),
    );
    memcpy(
        buff
            .as_mut_ptr()
            .offset(
                (::core::mem::size_of::<clock_t>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<libc::c_uint>() as libc::c_ulong,
                    ) as isize,
            ) as *mut libc::c_void,
        &mut t as *mut time_t as *const libc::c_void,
        (::core::mem::size_of::<time_t>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<libc::c_uint>() as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_uint>() as libc::c_ulong),
    );
    i = 0 as i32 as libc::c_uint;
    while (i as libc::c_ulong)
        < (::core::mem::size_of::<[libc::c_uint; 4]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<libc::c_uint>() as libc::c_ulong)
    {
        rnd = rnd.wrapping_add(buff[i as usize]);
        i = i.wrapping_add(1);
        i;
    }
    return rnd;
}
unsafe extern "C" fn set2(mut L: *mut lua_State, mut i: IdxT, mut j: IdxT) {
    lua_seti(L, 1 as i32, i as Integer);
    lua_seti(L, 1 as i32, j as Integer);
}
unsafe extern "C" fn sort_comp(
    mut L: *mut lua_State,
    mut a: i32,
    mut b: i32,
) -> i32 {
    if lua_type(L, 2 as i32) == 0 as i32 {
        return lua_compare(L, a, b, 1 as i32)
    } else {
        let mut res: i32 = 0;
        lua_pushvalue(L, 2 as i32);
        lua_pushvalue(L, a - 1 as i32);
        lua_pushvalue(L, b - 2 as i32);
        lua_callk(
            L,
            2 as i32,
            1 as i32,
            0 as i32 as lua_KContext,
            None,
        );
        res = lua_toboolean(L, -(1 as i32));
        lua_settop(L, -(1 as i32) - 1 as i32);
        return res;
    };
}
unsafe extern "C" fn partition(
    mut L: *mut lua_State,
    mut lo: IdxT,
    mut up: IdxT,
) -> IdxT {
    let mut i: IdxT = lo;
    let mut j: IdxT = up.wrapping_sub(1 as i32 as libc::c_uint);
    loop {
        loop {
            i = i.wrapping_add(1);
            lua_geti(L, 1 as i32, i as Integer);
            if !(sort_comp(L, -(1 as i32), -(2 as i32)) != 0) {
                break;
            }
            if ((i == up.wrapping_sub(1 as i32 as libc::c_uint)) as i32
                != 0 as i32) as i32 as libc::c_long != 0
            {
                luaL_error(
                    L,
                    b"invalid order function for sorting\0" as *const u8
                        as *const libc::c_char,
                );
            }
            lua_settop(L, -(1 as i32) - 1 as i32);
        }
        loop {
            j = j.wrapping_sub(1);
            lua_geti(L, 1 as i32, j as Integer);
            if !(sort_comp(L, -(3 as i32), -(1 as i32)) != 0) {
                break;
            }
            if ((j < i) as i32 != 0 as i32) as i32
                as libc::c_long != 0
            {
                luaL_error(
                    L,
                    b"invalid order function for sorting\0" as *const u8
                        as *const libc::c_char,
                );
            }
            lua_settop(L, -(1 as i32) - 1 as i32);
        }
        if j < i {
            lua_settop(L, -(1 as i32) - 1 as i32);
            set2(L, up.wrapping_sub(1 as i32 as libc::c_uint), i);
            return i;
        }
        set2(L, i, j);
    };
}
unsafe extern "C" fn choosePivot(
    mut lo: IdxT,
    mut up: IdxT,
    mut rnd: libc::c_uint,
) -> IdxT {
    let mut r4: IdxT = up
        .wrapping_sub(lo)
        .wrapping_div(4 as i32 as libc::c_uint);
    let mut p: IdxT = rnd
        .wrapping_rem(r4.wrapping_mul(2 as i32 as libc::c_uint))
        .wrapping_add(lo.wrapping_add(r4));
    return p;
}
unsafe extern "C" fn auxsort(
    mut L: *mut lua_State,
    mut lo: IdxT,
    mut up: IdxT,
    mut rnd: libc::c_uint,
) {
    while lo < up {
        let mut p: IdxT = 0;
        let mut n: IdxT = 0;
        lua_geti(L, 1 as i32, lo as Integer);
        lua_geti(L, 1 as i32, up as Integer);
        if sort_comp(L, -(1 as i32), -(2 as i32)) != 0 {
            set2(L, lo, up);
        } else {
            lua_settop(L, -(2 as i32) - 1 as i32);
        }
        if up.wrapping_sub(lo) == 1 as i32 as libc::c_uint {
            return;
        }
        if up.wrapping_sub(lo) < 100 as libc::c_uint
            || rnd == 0 as i32 as libc::c_uint
        {
            p = lo.wrapping_add(up).wrapping_div(2 as i32 as libc::c_uint);
        } else {
            p = choosePivot(lo, up, rnd);
        }
        lua_geti(L, 1 as i32, p as Integer);
        lua_geti(L, 1 as i32, lo as Integer);
        if sort_comp(L, -(2 as i32), -(1 as i32)) != 0 {
            set2(L, p, lo);
        } else {
            lua_settop(L, -(1 as i32) - 1 as i32);
            lua_geti(L, 1 as i32, up as Integer);
            if sort_comp(L, -(1 as i32), -(2 as i32)) != 0 {
                set2(L, p, up);
            } else {
                lua_settop(L, -(2 as i32) - 1 as i32);
            }
        }
        if up.wrapping_sub(lo) == 2 as i32 as libc::c_uint {
            return;
        }
        lua_geti(L, 1 as i32, p as Integer);
        lua_pushvalue(L, -(1 as i32));
        lua_geti(
            L,
            1 as i32,
            up.wrapping_sub(1 as i32 as libc::c_uint) as Integer,
        );
        set2(L, p, up.wrapping_sub(1 as i32 as libc::c_uint));
        p = partition(L, lo, up);
        if p.wrapping_sub(lo) < up.wrapping_sub(p) {
            auxsort(L, lo, p.wrapping_sub(1 as i32 as libc::c_uint), rnd);
            n = p.wrapping_sub(lo);
            lo = p.wrapping_add(1 as i32 as libc::c_uint);
        } else {
            auxsort(L, p.wrapping_add(1 as i32 as libc::c_uint), up, rnd);
            n = up.wrapping_sub(p);
            up = p.wrapping_sub(1 as i32 as libc::c_uint);
        }
        if up.wrapping_sub(lo).wrapping_div(128 as i32 as libc::c_uint) > n {
            rnd = l_randomizePivot();
        }
    }
}
unsafe extern "C" fn sort(mut L: *mut lua_State) -> i32 {
    checktab(
        L,
        1 as i32,
        1 as i32 | 2 as i32 | 4 as i32,
    );
    let mut n: Integer = luaL_len(L, 1 as i32);
    if n > 1 as i32 as i64 {
        (((n < 2147483647 as i32 as i64) as i32
            != 0 as i32) as i32 as libc::c_long != 0
            || luaL_argerror(
                L,
                1 as i32,
                b"array too big\0" as *const u8 as *const libc::c_char,
            ) != 0) as i32;
        if !(lua_type(L, 2 as i32) <= 0 as i32) {
            luaL_checktype(L, 2 as i32, 6 as i32);
        }
        lua_settop(L, 2 as i32);
        auxsort(
            L,
            1 as i32 as IdxT,
            n as IdxT,
            0 as i32 as libc::c_uint,
        );
    }
    return 0 as i32;
}
static mut tab_funcs: [luaL_Reg; 8] = unsafe {
    [
        {
            let mut init = luaL_Reg {
                name: b"concat\0" as *const u8 as *const libc::c_char,
                func: Some(
                    tconcat as unsafe extern "C" fn(*mut lua_State) -> i32,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"insert\0" as *const u8 as *const libc::c_char,
                func: Some(
                    tinsert as unsafe extern "C" fn(*mut lua_State) -> i32,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"pack\0" as *const u8 as *const libc::c_char,
                func: Some(tpack as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"unpack\0" as *const u8 as *const libc::c_char,
                func: Some(
                    tunpack as unsafe extern "C" fn(*mut lua_State) -> i32,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"remove\0" as *const u8 as *const libc::c_char,
                func: Some(
                    tremove as unsafe extern "C" fn(*mut lua_State) -> i32,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"move\0" as *const u8 as *const libc::c_char,
                func: Some(tmove as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"sort\0" as *const u8 as *const libc::c_char,
                func: Some(sort as unsafe extern "C" fn(*mut lua_State) -> i32),
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
pub unsafe extern "C" fn luaopen_table(mut L: *mut lua_State) -> i32 {
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
    luaL_setfuncs(L, tab_funcs.as_ptr(), 0 as i32);
    return 1 as i32;
}
