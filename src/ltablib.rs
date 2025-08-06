#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use libc::{tm, clock_t, time_t};
unsafe extern "C" {
    pub type lua_State;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn lua_gettop(L: *mut lua_State) -> i32;
    fn lua_settop(L: *mut lua_State, index: i32);
    fn lua_pushvalue(L: *mut lua_State, index: i32);
    fn lua_rotate(L: *mut lua_State, index: i32, n: i32);
    fn lua_checkstack(L: *mut lua_State, n: i32) -> i32;
    fn lua_isstring(L: *mut lua_State, index: i32) -> i32;
    fn lua_type(L: *mut lua_State, index: i32) -> i32;
    fn lua_typename(L: *mut lua_State, tp: i32) -> *const libc::c_char;
    fn lua_toboolean(L: *mut lua_State, index: i32) -> i32;
    fn lua_compare(L: *mut lua_State, index1: i32, index2: i32, op: i32) -> i32;
    fn lua_pushnil(L: *mut lua_State);
    fn lua_pushinteger(L: *mut lua_State, n: i64);
    fn lua_pushstring(L: *mut lua_State, s: *const libc::c_char) -> *const libc::c_char;
    fn lua_geti(L: *mut lua_State, index: i32, n: i64) -> i32;
    fn lua_rawget(L: *mut lua_State, index: i32) -> i32;
    fn lua_createtable(L: *mut lua_State, narr: i32, nrec: i32);
    fn lua_getmetatable(L: *mut lua_State, objindex: i32) -> i32;
    fn lua_setfield(L: *mut lua_State, index: i32, k: *const libc::c_char);
    fn lua_seti(L: *mut lua_State, index: i32, n: i64);
    fn lua_callk(L: *mut lua_State, nargs: i32, nresults: i32, ctx: lua_KContext, k: lua_KFunction);
    fn luaL_checkversion_(L: *mut lua_State, ver: f64, sz: u64);
    fn luaL_argerror(L: *mut lua_State, arg: i32, extramsg: *const libc::c_char) -> i32;
    fn luaL_optlstring(
        L: *mut lua_State,
        arg: i32,
        def: *const libc::c_char,
        l: *mut u64,
    ) -> *const libc::c_char;
    fn luaL_checkinteger(L: *mut lua_State, arg: i32) -> i64;
    fn luaL_optinteger(L: *mut lua_State, arg: i32, def: i64) -> i64;
    fn luaL_checktype(L: *mut lua_State, arg: i32, t: i32);
    fn luaL_error(L: *mut lua_State, fmt: *const libc::c_char, _: ...) -> i32;
    fn luaL_len(L: *mut lua_State, index: i32) -> i64;
    fn luaL_setfuncs(L: *mut lua_State, l: *const luaL_Reg, nup: i32);
    fn luaL_buffinit(L: *mut lua_State, B: *mut luaL_Buffer);
    fn luaL_addlstring(B: *mut luaL_Buffer, s: *const libc::c_char, l: u64);
    fn luaL_addvalue(B: *mut luaL_Buffer);
    fn luaL_pushresult(B: *mut luaL_Buffer);
    fn clock() -> clock_t;
    fn time(__timer: *mut time_t) -> time_t;
}
pub type lua_KContext = i64;
pub type CFunction = Option<unsafe extern "C" fn(*mut lua_State) -> i32>;
pub type lua_KFunction = Option<unsafe extern "C" fn(*mut lua_State, i32, lua_KContext) -> i32>;
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
pub type IdxT = u32;
unsafe extern "C" fn checkfield(
    mut L: *mut lua_State,
    mut key: *const libc::c_char,
    mut n: i32,
) -> i32 {
    lua_pushstring(L, key);
    return (lua_rawget(L, -n) != 0i32) as i32;
}
unsafe extern "C" fn checktab(mut L: *mut lua_State, mut arg: i32, mut what: i32) {
    if lua_type(L, arg) != 5i32 {
        let mut n: i32 = 1i32;
        if lua_getmetatable(L, arg) != 0
            && (what & 1i32 == 0 || {
                n += 1;
                checkfield(L, b"__index\0" as *const u8 as *const libc::c_char, n) != 0
            })
            && (what & 2i32 == 0 || {
                n += 1;
                checkfield(L, b"__newindex\0" as *const u8 as *const libc::c_char, n) != 0
            })
            && (what & 4i32 == 0 || {
                n += 1;
                checkfield(L, b"__len\0" as *const u8 as *const libc::c_char, n) != 0
            })
        {
            lua_settop(L, -n - 1i32);
        } else {
            luaL_checktype(L, arg, 5i32);
        }
    }
}
unsafe extern "C" fn tinsert(mut L: *mut lua_State) -> i32 {
    let mut pos: i64 = 0;
    checktab(L, 1i32, 1i32 | 2i32 | 4i32);
    let mut e: i64 = luaL_len(L, 1i32);
    e = (e as u64).wrapping_add(1i32 as u64) as i64;
    match lua_gettop(L) {
        2 => {
            pos = e;
        }
        3 => {
            let mut i: i64 = 0;
            pos = luaL_checkinteger(L, 2i32);
            ((((pos as u64).wrapping_sub(1 as u32 as u64) < e as u64) as i32 != 0i32) as i32 as i64
                != 0
                || luaL_argerror(
                    L,
                    2i32,
                    b"position out of bounds\0" as *const u8 as *const libc::c_char,
                ) != 0) as i32;
            i = e;
            while i > pos {
                lua_geti(L, 1i32, i - 1i32 as i64);
                lua_seti(L, 1i32, i);
                i -= 1;
            }
        }
        _ => {
            return luaL_error(
                L,
                b"wrong number of arguments to 'insert'\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    lua_seti(L, 1i32, pos);
    return 0i32;
}
unsafe extern "C" fn tremove(mut L: *mut lua_State) -> i32 {
    checktab(L, 1i32, 1i32 | 2i32 | 4i32);
    let mut size: i64 = luaL_len(L, 1i32);
    let mut pos: i64 = luaL_optinteger(L, 2i32, size);
    if pos != size {
        ((((pos as u64).wrapping_sub(1 as u32 as u64) <= size as u64) as i32 != 0i32) as i32 as i64
            != 0
            || luaL_argerror(
                L,
                2i32,
                b"position out of bounds\0" as *const u8 as *const libc::c_char,
            ) != 0) as i32;
    }
    lua_geti(L, 1i32, pos);
    while pos < size {
        lua_geti(L, 1i32, pos + 1i32 as i64);
        lua_seti(L, 1i32, pos);
        pos += 1;
    }
    lua_pushnil(L);
    lua_seti(L, 1i32, pos);
    return 1i32;
}
unsafe extern "C" fn tmove(mut L: *mut lua_State) -> i32 {
    let mut f: i64 = luaL_checkinteger(L, 2i32);
    let mut e: i64 = luaL_checkinteger(L, 3i32);
    let mut t: i64 = luaL_checkinteger(L, 4i32);
    let mut tt: i32 = if !(lua_type(L, 5i32) <= 0i32) {
        5i32
    } else {
        1i32
    };
    checktab(L, 1i32, 1i32);
    checktab(L, tt, 2i32);
    if e >= f {
        let mut n: i64 = 0;
        let mut i: i64 = 0;
        (((f > 0i32 as i64 || e < 9223372036854775807i64 + f) as i32 != 0i32) as i32 as i64 != 0
            || luaL_argerror(
                L,
                3i32,
                b"too many elements to move\0" as *const u8 as *const libc::c_char,
            ) != 0) as i32;
        n = e - f + 1i32 as i64;
        (((t <= 9223372036854775807i64 - n + 1i32 as i64) as i32 != 0i32) as i32 as i64 != 0
            || luaL_argerror(
                L,
                4i32,
                b"destination wrap around\0" as *const u8 as *const libc::c_char,
            ) != 0) as i32;
        if t > e || t <= f || tt != 1i32 && lua_compare(L, 1i32, tt, 0i32) == 0 {
            i = 0i32 as i64;
            while i < n {
                lua_geti(L, 1i32, f + i);
                lua_seti(L, tt, t + i);
                i += 1;
            }
        } else {
            i = n - 1i32 as i64;
            while i >= 0i32 as i64 {
                lua_geti(L, 1i32, f + i);
                lua_seti(L, tt, t + i);
                i -= 1;
            }
        }
    }
    lua_pushvalue(L, tt);
    return 1i32;
}
unsafe extern "C" fn addfield(mut L: *mut lua_State, mut b: *mut luaL_Buffer, mut i: i64) {
    lua_geti(L, 1i32, i);
    if ((lua_isstring(L, -(1i32)) == 0) as i32 != 0i32) as i32 as i64 != 0 {
        luaL_error(
            L,
            b"invalid value (%s) at index %I in table for 'concat'\0" as *const u8
                as *const libc::c_char,
            lua_typename(L, lua_type(L, -(1i32))),
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
    checktab(L, 1i32, 1i32 | 4i32);
    let mut last: i64 = luaL_len(L, 1i32);
    let mut lsep: u64 = 0;
    let mut sep: *const libc::c_char = luaL_optlstring(
        L,
        2i32,
        b"\0" as *const u8 as *const libc::c_char,
        &mut lsep,
    );
    let mut i: i64 = luaL_optinteger(L, 3i32, 1i32 as i64);
    last = luaL_optinteger(L, 4i32, last);
    luaL_buffinit(L, &mut b);
    while i < last {
        addfield(L, &mut b, i);
        luaL_addlstring(&mut b, sep, lsep);
        i += 1;
    }
    if i == last {
        addfield(L, &mut b, i);
    }
    luaL_pushresult(&mut b);
    return 1i32;
}
unsafe extern "C" fn tpack(mut L: *mut lua_State) -> i32 {
    let mut i: i32 = 0;
    let mut n: i32 = lua_gettop(L);
    lua_createtable(L, n, 1i32);
    lua_rotate(L, 1i32, 1i32);
    i = n;
    while i >= 1i32 {
        lua_seti(L, 1i32, i as i64);
        i -= 1;
    }
    lua_pushinteger(L, n as i64);
    lua_setfield(L, 1i32, b"n\0" as *const u8 as *const libc::c_char);
    return 1i32;
}
unsafe extern "C" fn tunpack(mut L: *mut lua_State) -> i32 {
    let mut n: u64 = 0;
    let mut i: i64 = luaL_optinteger(L, 2i32, 1i32 as i64);
    let mut e: i64 = if lua_type(L, 3i32) <= 0i32 {
        luaL_len(L, 1i32)
    } else {
        luaL_checkinteger(L, 3i32)
    };
    if i > e {
        return 0i32;
    }
    n = (e as u64).wrapping_sub(i as u64);
    if ((n >= 2147483647i32 as u32 as u64 || {
        n = n.wrapping_add(1);
        lua_checkstack(L, n as i32) == 0
    }) as i32
        != 0i32) as i32 as i64
        != 0
    {
        return luaL_error(
            L,
            b"too many results to unpack\0" as *const u8 as *const libc::c_char,
        );
    }
    while i < e {
        lua_geti(L, 1i32, i);
        i += 1;
    }
    lua_geti(L, 1i32, e);
    return n as i32;
}
unsafe extern "C" fn l_randomizePivot() -> u32 {
    let mut c: clock_t = clock();
    let mut t: time_t = time(0 as *mut time_t);
    let mut buff: [u32; 4] = [0; 4];
    let mut i: u32 = 0;
    let mut rnd: u32 = 0i32 as u32;
    memcpy(
        buff.as_mut_ptr() as *mut libc::c_void,
        &mut c as *mut clock_t as *const libc::c_void,
        (::core::mem::size_of::<clock_t>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<u32>() as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<u32>() as libc::c_ulong),
    );
    memcpy(
        buff.as_mut_ptr().offset(
            (::core::mem::size_of::<clock_t>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<u32>() as libc::c_ulong) as isize,
        ) as *mut libc::c_void,
        &mut t as *mut time_t as *const libc::c_void,
        (::core::mem::size_of::<time_t>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<u32>() as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<u32>() as libc::c_ulong),
    );
    i = 0i32 as u32;
    while (i as libc::c_ulong)
        < (::core::mem::size_of::<[u32; 4]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<u32>() as libc::c_ulong)
    {
        rnd = rnd.wrapping_add(buff[i as usize]);
        i = i.wrapping_add(1);
    }
    return rnd;
}
unsafe extern "C" fn set2(mut L: *mut lua_State, mut i: IdxT, mut j: IdxT) {
    lua_seti(L, 1i32, i as i64);
    lua_seti(L, 1i32, j as i64);
}
unsafe extern "C" fn sort_comp(mut L: *mut lua_State, mut a: i32, mut b: i32) -> i32 {
    if lua_type(L, 2i32) == 0i32 {
        return lua_compare(L, a, b, 1i32);
    } else {
        let mut res: i32 = 0;
        lua_pushvalue(L, 2i32);
        lua_pushvalue(L, a - 1i32);
        lua_pushvalue(L, b - 2i32);
        lua_callk(L, 2i32, 1i32, 0i32 as lua_KContext, None);
        res = lua_toboolean(L, -(1i32));
        lua_settop(L, -(1i32) - 1i32);
        return res;
    };
}
unsafe extern "C" fn partition(mut L: *mut lua_State, mut lo: IdxT, mut up: IdxT) -> IdxT {
    let mut i: IdxT = lo;
    let mut j: IdxT = up.wrapping_sub(1i32 as u32);
    loop {
        loop {
            i = i.wrapping_add(1);
            lua_geti(L, 1i32, i as i64);
            if !(sort_comp(L, -(1i32), -(2i32)) != 0) {
                break;
            }
            if ((i == up.wrapping_sub(1i32 as u32)) as i32 != 0i32) as i32 as i64 != 0 {
                luaL_error(
                    L,
                    b"invalid order function for sorting\0" as *const u8 as *const libc::c_char,
                );
            }
            lua_settop(L, -(1i32) - 1i32);
        }
        loop {
            j = j.wrapping_sub(1);
            lua_geti(L, 1i32, j as i64);
            if !(sort_comp(L, -(3i32), -(1i32)) != 0) {
                break;
            }
            if ((j < i) as i32 != 0i32) as i32 as i64 != 0 {
                luaL_error(
                    L,
                    b"invalid order function for sorting\0" as *const u8 as *const libc::c_char,
                );
            }
            lua_settop(L, -(1i32) - 1i32);
        }
        if j < i {
            lua_settop(L, -(1i32) - 1i32);
            set2(L, up.wrapping_sub(1i32 as u32), i);
            return i;
        }
        set2(L, i, j);
    }
}
unsafe extern "C" fn choosePivot(mut lo: IdxT, mut up: IdxT, mut rnd: u32) -> IdxT {
    let mut r4: IdxT = up.wrapping_sub(lo).wrapping_div(4i32 as u32);
    let mut p: IdxT = rnd
        .wrapping_rem(r4.wrapping_mul(2i32 as u32))
        .wrapping_add(lo.wrapping_add(r4));
    return p;
}
unsafe extern "C" fn auxsort(mut L: *mut lua_State, mut lo: IdxT, mut up: IdxT, mut rnd: u32) {
    while lo < up {
        let mut p: IdxT = 0;
        let mut n: IdxT = 0;
        lua_geti(L, 1i32, lo as i64);
        lua_geti(L, 1i32, up as i64);
        if sort_comp(L, -(1i32), -(2i32)) != 0 {
            set2(L, lo, up);
        } else {
            lua_settop(L, -(2i32) - 1i32);
        }
        if up.wrapping_sub(lo) == 1i32 as u32 {
            return;
        }
        if up.wrapping_sub(lo) < 100 as u32 || rnd == 0i32 as u32 {
            p = lo.wrapping_add(up).wrapping_div(2i32 as u32);
        } else {
            p = choosePivot(lo, up, rnd);
        }
        lua_geti(L, 1i32, p as i64);
        lua_geti(L, 1i32, lo as i64);
        if sort_comp(L, -(2i32), -(1i32)) != 0 {
            set2(L, p, lo);
        } else {
            lua_settop(L, -(1i32) - 1i32);
            lua_geti(L, 1i32, up as i64);
            if sort_comp(L, -(1i32), -(2i32)) != 0 {
                set2(L, p, up);
            } else {
                lua_settop(L, -(2i32) - 1i32);
            }
        }
        if up.wrapping_sub(lo) == 2i32 as u32 {
            return;
        }
        lua_geti(L, 1i32, p as i64);
        lua_pushvalue(L, -(1i32));
        lua_geti(L, 1i32, up.wrapping_sub(1i32 as u32) as i64);
        set2(L, p, up.wrapping_sub(1i32 as u32));
        p = partition(L, lo, up);
        if p.wrapping_sub(lo) < up.wrapping_sub(p) {
            auxsort(L, lo, p.wrapping_sub(1i32 as u32), rnd);
            n = p.wrapping_sub(lo);
            lo = p.wrapping_add(1i32 as u32);
        } else {
            auxsort(L, p.wrapping_add(1i32 as u32), up, rnd);
            n = up.wrapping_sub(p);
            up = p.wrapping_sub(1i32 as u32);
        }
        if up.wrapping_sub(lo).wrapping_div(128i32 as u32) > n {
            rnd = l_randomizePivot();
        }
    }
}
unsafe extern "C" fn sort(mut L: *mut lua_State) -> i32 {
    checktab(L, 1i32, 1i32 | 2i32 | 4i32);
    let mut n: i64 = luaL_len(L, 1i32);
    if n > 1i32 as i64 {
        (((n < 2147483647i32 as i64) as i32 != 0i32) as i32 as i64 != 0
            || luaL_argerror(
                L,
                1i32,
                b"array too big\0" as *const u8 as *const libc::c_char,
            ) != 0) as i32;
        if !(lua_type(L, 2i32) <= 0i32) {
            luaL_checktype(L, 2i32, 6i32);
        }
        lua_settop(L, 2i32);
        auxsort(L, 1i32 as IdxT, n as IdxT, 0i32 as u32);
    }
    return 0i32;
}
static mut tab_funcs: [luaL_Reg; 8] = {
    [
        {
            let mut init = luaL_Reg {
                name: b"concat\0" as *const u8 as *const libc::c_char,
                func: Some(tconcat as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"insert\0" as *const u8 as *const libc::c_char,
                func: Some(tinsert as unsafe extern "C" fn(*mut lua_State) -> i32),
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
                func: Some(tunpack as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"remove\0" as *const u8 as *const libc::c_char,
                func: Some(tremove as unsafe extern "C" fn(*mut lua_State) -> i32),
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaopen_table(mut L: *mut lua_State) -> i32 {
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
        (::core::mem::size_of::<[luaL_Reg; 8]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<luaL_Reg>() as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong) as i32,
    );
    luaL_setfuncs(L, tab_funcs.as_ptr(), 0i32);
    return 1i32;
}
