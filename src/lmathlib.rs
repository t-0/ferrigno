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
    fn atan2(_: f64, _: f64) -> f64;
    fn exp(_: f64) -> f64;
    fn log(_: f64) -> f64;
    fn log10(_: f64) -> f64;
    fn log2(_: f64) -> f64;
    fn fmod(_: f64, _: f64) -> f64;
    fn time(__timer: *mut time_t) -> time_t;
    fn lua_gettop(L: *mut lua_State) -> i32;
    fn lua_settop(L: *mut lua_State, index: i32);
    fn lua_pushvalue(L: *mut lua_State, index: i32);
    fn lua_isinteger(L: *mut lua_State, index: i32) -> i32;
    fn lua_type(L: *mut lua_State, index: i32) -> i32;
    fn lua_tointegerx(
        L: *mut lua_State,
        index: i32,
        isnum: *mut i32,
    ) -> Integer;
    fn lua_touserdata(L: *mut lua_State, index: i32) -> *mut libc::c_void;
    fn lua_compare(
        L: *mut lua_State,
        index1: i32,
        index2: i32,
        op: i32,
    ) -> i32;
    fn lua_pushnil(L: *mut lua_State);
    fn lua_pushnumber(L: *mut lua_State, n: Number);
    fn lua_pushinteger(L: *mut lua_State, n: Integer);
    fn lua_pushstring(L: *mut lua_State, s: *const libc::c_char) -> *const libc::c_char;
    fn lua_pushboolean(L: *mut lua_State, b: i32);
    fn lua_createtable(L: *mut lua_State, narr: i32, nrec: i32);
    fn lua_newuserdatauv(
        L: *mut lua_State,
        sz: size_t,
        nuvalue: i32,
    ) -> *mut libc::c_void;
    fn lua_setfield(L: *mut lua_State, index: i32, k: *const libc::c_char);
    fn luaL_checkversion_(L: *mut lua_State, ver: Number, sz: size_t);
    fn luaL_argerror(
        L: *mut lua_State,
        arg: i32,
        extramsg: *const libc::c_char,
    ) -> i32;
    fn luaL_checknumber(L: *mut lua_State, arg: i32) -> Number;
    fn luaL_optnumber(
        L: *mut lua_State,
        arg: i32,
        def: Number,
    ) -> Number;
    fn luaL_checkinteger(L: *mut lua_State, arg: i32) -> Integer;
    fn luaL_optinteger(
        L: *mut lua_State,
        arg: i32,
        def: Integer,
    ) -> Integer;
    fn luaL_checkany(L: *mut lua_State, arg: i32);
    fn luaL_error(L: *mut lua_State, fmt: *const libc::c_char, _: ...) -> i32;
    fn luaL_setfuncs(L: *mut lua_State, l: *const luaL_Reg, nup: i32);
}
pub type __time_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type time_t = __time_t;
pub type Number = f64;
pub type Integer = i64;
pub type lua_Unsigned = libc::c_ulonglong;
pub type CFunction = Option::<unsafe extern "C" fn(*mut lua_State) -> i32>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaL_Reg {
    pub name: *const libc::c_char,
    pub func: CFunction,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RanState {
    pub s: [libc::c_ulong; 4],
}
unsafe extern "C" fn math_abs(mut L: *mut lua_State) -> i32 {
    if lua_isinteger(L, 1 as i32) != 0 {
        let mut n: Integer = lua_tointegerx(
            L,
            1 as i32,
            0 as *mut i32,
        );
        if n < 0 as i32 as i64 {
            n = (0 as libc::c_uint as libc::c_ulonglong).wrapping_sub(n as lua_Unsigned)
                as Integer;
        }
        lua_pushinteger(L, n);
    } else {
        lua_pushnumber(L, luaL_checknumber(L, 1 as i32).abs());
    }
    return 1 as i32;
}
unsafe extern "C" fn math_sin(mut L: *mut lua_State) -> i32 {
    lua_pushnumber(L, luaL_checknumber(L, 1 as i32).sin());
    return 1 as i32;
}
unsafe extern "C" fn math_cos(mut L: *mut lua_State) -> i32 {
    lua_pushnumber(L, luaL_checknumber(L, 1 as i32).cos());
    return 1 as i32;
}
unsafe extern "C" fn math_tan(mut L: *mut lua_State) -> i32 {
    lua_pushnumber(L, luaL_checknumber(L, 1 as i32).tan());
    return 1 as i32;
}
unsafe extern "C" fn math_asin(mut L: *mut lua_State) -> i32 {
    lua_pushnumber(L, luaL_checknumber(L, 1 as i32).asin());
    return 1 as i32;
}
unsafe extern "C" fn math_acos(mut L: *mut lua_State) -> i32 {
    lua_pushnumber(L, luaL_checknumber(L, 1 as i32).acos());
    return 1 as i32;
}
unsafe extern "C" fn math_atan(mut L: *mut lua_State) -> i32 {
    let mut y: Number = luaL_checknumber(L, 1 as i32);
    let mut x: Number = luaL_optnumber(
        L,
        2 as i32,
        1 as i32 as Number,
    );
    lua_pushnumber(L, atan2(y, x));
    return 1 as i32;
}
unsafe extern "C" fn math_toint(mut L: *mut lua_State) -> i32 {
    let mut valid: i32 = 0;
    let mut n: Integer = lua_tointegerx(L, 1 as i32, &mut valid);
    if (valid != 0 as i32) as i32 as libc::c_long != 0 {
        lua_pushinteger(L, n);
    } else {
        luaL_checkany(L, 1 as i32);
        lua_pushnil(L);
    }
    return 1 as i32;
}
unsafe extern "C" fn pushnumint(mut L: *mut lua_State, mut d: Number) {
    let mut n: Integer = 0;
    if d
        >= (-(9223372036854775807 as i64) - 1 as i64)
            as f64
        && d
            < -((-(9223372036854775807 as i64) - 1 as i64)
                as f64)
        && {
            n = d as i64;
            1 as i32 != 0
        }
    {
        lua_pushinteger(L, n);
    } else {
        lua_pushnumber(L, d);
    };
}
unsafe extern "C" fn math_floor(mut L: *mut lua_State) -> i32 {
    if lua_isinteger(L, 1 as i32) != 0 {
        lua_settop(L, 1 as i32);
    } else {
        let mut d: Number = luaL_checknumber(L, 1 as i32).floor();
        pushnumint(L, d);
    }
    return 1 as i32;
}
unsafe extern "C" fn math_ceil(mut L: *mut lua_State) -> i32 {
    if lua_isinteger(L, 1 as i32) != 0 {
        lua_settop(L, 1 as i32);
    } else {
        let mut d: Number = luaL_checknumber(L, 1 as i32).ceil();
        pushnumint(L, d);
    }
    return 1 as i32;
}
unsafe extern "C" fn math_fmod(mut L: *mut lua_State) -> i32 {
    if lua_isinteger(L, 1 as i32) != 0 && lua_isinteger(L, 2 as i32) != 0
    {
        let mut d: Integer = lua_tointegerx(
            L,
            2 as i32,
            0 as *mut i32,
        );
        if (d as lua_Unsigned).wrapping_add(1 as libc::c_uint as libc::c_ulonglong)
            <= 1 as libc::c_uint as libc::c_ulonglong
        {
            (((d != 0 as i32 as i64) as i32
                != 0 as i32) as i32 as libc::c_long != 0
                || luaL_argerror(
                    L,
                    2 as i32,
                    b"zero\0" as *const u8 as *const libc::c_char,
                ) != 0) as i32;
            lua_pushinteger(L, 0 as i32 as Integer);
        } else {
            lua_pushinteger(
                L,
                lua_tointegerx(L, 1 as i32, 0 as *mut i32) % d,
            );
        }
    } else {
        lua_pushnumber(
            L,
            fmod(
                luaL_checknumber(L, 1 as i32),
                luaL_checknumber(L, 2 as i32),
            ),
        );
    }
    return 1 as i32;
}
unsafe extern "C" fn math_modf(mut L: *mut lua_State) -> i32 {
    if lua_isinteger(L, 1 as i32) != 0 {
        lua_settop(L, 1 as i32);
        lua_pushnumber(L, 0 as i32 as Number);
    } else {
        let mut n: Number = luaL_checknumber(L, 1 as i32);
        let mut ip: Number = if n < 0 as i32 as f64 {
            n.ceil()
        } else {
            n.floor()
        };
        pushnumint(L, ip);
        lua_pushnumber(L, if n == ip { 0.0f64 } else { n - ip });
    }
    return 2 as i32;
}
unsafe extern "C" fn math_sqrt(mut L: *mut lua_State) -> i32 {
    lua_pushnumber(L, luaL_checknumber(L, 1 as i32).sqrt());
    return 1 as i32;
}
unsafe extern "C" fn math_ult(mut L: *mut lua_State) -> i32 {
    let mut a: Integer = luaL_checkinteger(L, 1 as i32);
    let mut b: Integer = luaL_checkinteger(L, 2 as i32);
    lua_pushboolean(L, ((a as lua_Unsigned) < b as lua_Unsigned) as i32);
    return 1 as i32;
}
unsafe extern "C" fn math_log(mut L: *mut lua_State) -> i32 {
    let mut x: Number = luaL_checknumber(L, 1 as i32);
    let mut res: Number = 0.;
    if lua_type(L, 2 as i32) <= 0 as i32 {
        res = log(x);
    } else {
        let mut base: Number = luaL_checknumber(L, 2 as i32);
        if base == 2.0f64 {
            res = log2(x);
        } else if base == 10.0f64 {
            res = log10(x);
        } else {
            res = log(x) / log(base);
        }
    }
    lua_pushnumber(L, res);
    return 1 as i32;
}
unsafe extern "C" fn math_exp(mut L: *mut lua_State) -> i32 {
    lua_pushnumber(L, exp(luaL_checknumber(L, 1 as i32)));
    return 1 as i32;
}
unsafe extern "C" fn math_deg(mut L: *mut lua_State) -> i32 {
    lua_pushnumber(
        L,
        luaL_checknumber(L, 1 as i32)
            * (180.0f64 / 3.141592653589793238462643383279502884f64),
    );
    return 1 as i32;
}
unsafe extern "C" fn math_rad(mut L: *mut lua_State) -> i32 {
    lua_pushnumber(
        L,
        luaL_checknumber(L, 1 as i32)
            * (3.141592653589793238462643383279502884f64 / 180.0f64),
    );
    return 1 as i32;
}
unsafe extern "C" fn math_min(mut L: *mut lua_State) -> i32 {
    let mut n: i32 = lua_gettop(L);
    let mut imin: i32 = 1 as i32;
    let mut i: i32 = 0;
    (((n >= 1 as i32) as i32 != 0 as i32) as i32
        as libc::c_long != 0
        || luaL_argerror(
            L,
            1 as i32,
            b"value expected\0" as *const u8 as *const libc::c_char,
        ) != 0) as i32;
    i = 2 as i32;
    while i <= n {
        if lua_compare(L, i, imin, 1 as i32) != 0 {
            imin = i;
        }
        i += 1;
        i;
    }
    lua_pushvalue(L, imin);
    return 1 as i32;
}
unsafe extern "C" fn math_max(mut L: *mut lua_State) -> i32 {
    let mut n: i32 = lua_gettop(L);
    let mut imax: i32 = 1 as i32;
    let mut i: i32 = 0;
    (((n >= 1 as i32) as i32 != 0 as i32) as i32
        as libc::c_long != 0
        || luaL_argerror(
            L,
            1 as i32,
            b"value expected\0" as *const u8 as *const libc::c_char,
        ) != 0) as i32;
    i = 2 as i32;
    while i <= n {
        if lua_compare(L, imax, i, 1 as i32) != 0 {
            imax = i;
        }
        i += 1;
        i;
    }
    lua_pushvalue(L, imax);
    return 1 as i32;
}
unsafe extern "C" fn math_type(mut L: *mut lua_State) -> i32 {
    if lua_type(L, 1 as i32) == 3 as i32 {
        lua_pushstring(
            L,
            if lua_isinteger(L, 1 as i32) != 0 {
                b"integer\0" as *const u8 as *const libc::c_char
            } else {
                b"float\0" as *const u8 as *const libc::c_char
            },
        );
    } else {
        luaL_checkany(L, 1 as i32);
        lua_pushnil(L);
    }
    return 1 as i32;
}
unsafe extern "C" fn rotl(mut x: libc::c_ulong, mut n: i32) -> libc::c_ulong {
    return x << n | (x & 0xffffffffffffffff as libc::c_ulong) >> 64 as i32 - n;
}
unsafe extern "C" fn nextrand(mut state: *mut libc::c_ulong) -> libc::c_ulong {
    let mut state0: libc::c_ulong = *state.offset(0 as i32 as isize);
    let mut state1: libc::c_ulong = *state.offset(1 as i32 as isize);
    let mut state2: libc::c_ulong = *state.offset(2 as i32 as isize) ^ state0;
    let mut state3: libc::c_ulong = *state.offset(3 as i32 as isize) ^ state1;
    let mut res: libc::c_ulong = (rotl(
        state1.wrapping_mul(5 as i32 as libc::c_ulong),
        7 as i32,
    ))
        .wrapping_mul(9 as i32 as libc::c_ulong);
    *state.offset(0 as i32 as isize) = state0 ^ state3;
    *state.offset(1 as i32 as isize) = state1 ^ state2;
    *state.offset(2 as i32 as isize) = state2 ^ state1 << 17 as i32;
    *state.offset(3 as i32 as isize) = rotl(state3, 45 as i32);
    return res;
}
unsafe extern "C" fn I2d(mut x: libc::c_ulong) -> Number {
    let mut sx: libc::c_long = ((x & 0xffffffffffffffff as libc::c_ulong)
        >> 64 as i32 - 53 as i32) as libc::c_long;
    let mut res: Number = sx as Number
        * (0.5f64
            / ((1 as i32 as libc::c_ulong)
                << 53 as i32 - 1 as i32) as f64);
    if sx < 0 as i32 as libc::c_long {
        res += 1.0f64;
    }
    return res;
}
unsafe extern "C" fn project(
    mut ran: lua_Unsigned,
    mut n: lua_Unsigned,
    mut state: *mut RanState,
) -> lua_Unsigned {
    if n & n.wrapping_add(1 as i32 as libc::c_ulonglong)
        == 0 as i32 as libc::c_ulonglong
    {
        return ran & n
    } else {
        let mut lim: lua_Unsigned = n;
        lim |= lim >> 1 as i32;
        lim |= lim >> 2 as i32;
        lim |= lim >> 4 as i32;
        lim |= lim >> 8 as i32;
        lim |= lim >> 16 as i32;
        lim |= lim >> 32 as i32;
        loop {
            ran &= lim;
            if !(ran > n) {
                break;
            }
            ran = (nextrand(((*state).s).as_mut_ptr())
                & 0xffffffffffffffff as libc::c_ulong) as lua_Unsigned;
        }
        return ran;
    };
}
unsafe extern "C" fn math_random(mut L: *mut lua_State) -> i32 {
    let mut low: Integer = 0;
    let mut up: Integer = 0;
    let mut p: lua_Unsigned = 0;
    let mut state: *mut RanState = lua_touserdata(
        L,
        -(1000000 as i32) - 1000 as i32 - 1 as i32,
    ) as *mut RanState;
    let mut rv: libc::c_ulong = nextrand(((*state).s).as_mut_ptr());
    match lua_gettop(L) {
        0 => {
            lua_pushnumber(L, I2d(rv));
            return 1 as i32;
        }
        1 => {
            low = 1 as i32 as Integer;
            up = luaL_checkinteger(L, 1 as i32);
            if up == 0 as i32 as i64 {
                lua_pushinteger(
                    L,
                    (rv & 0xffffffffffffffff as libc::c_ulong) as lua_Unsigned
                        as Integer,
                );
                return 1 as i32;
            }
        }
        2 => {
            low = luaL_checkinteger(L, 1 as i32);
            up = luaL_checkinteger(L, 2 as i32);
        }
        _ => {
            return luaL_error(
                L,
                b"wrong number of arguments\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    (((low <= up) as i32 != 0 as i32) as i32 as libc::c_long != 0
        || luaL_argerror(
            L,
            1 as i32,
            b"interval is empty\0" as *const u8 as *const libc::c_char,
        ) != 0) as i32;
    p = project(
        (rv & 0xffffffffffffffff as libc::c_ulong) as lua_Unsigned,
        (up as lua_Unsigned).wrapping_sub(low as lua_Unsigned),
        state,
    );
    lua_pushinteger(L, p.wrapping_add(low as lua_Unsigned) as Integer);
    return 1 as i32;
}
unsafe extern "C" fn setseed(
    mut L: *mut lua_State,
    mut state: *mut libc::c_ulong,
    mut n1: lua_Unsigned,
    mut n2: lua_Unsigned,
) {
    let mut i: i32 = 0;
    *state.offset(0 as i32 as isize) = n1 as libc::c_ulong;
    *state.offset(1 as i32 as isize) = 0xff as i32 as libc::c_ulong;
    *state.offset(2 as i32 as isize) = n2 as libc::c_ulong;
    *state.offset(3 as i32 as isize) = 0 as i32 as libc::c_ulong;
    i = 0 as i32;
    while i < 16 as i32 {
        nextrand(state);
        i += 1;
        i;
    }
    lua_pushinteger(L, n1 as Integer);
    lua_pushinteger(L, n2 as Integer);
}
unsafe extern "C" fn randseed(mut L: *mut lua_State, mut state: *mut RanState) {
    let mut seed1: lua_Unsigned = time(0 as *mut time_t) as lua_Unsigned;
    let mut seed2: lua_Unsigned = L as size_t as lua_Unsigned;
    setseed(L, ((*state).s).as_mut_ptr(), seed1, seed2);
}
unsafe extern "C" fn math_randomseed(mut L: *mut lua_State) -> i32 {
    let mut state: *mut RanState = lua_touserdata(
        L,
        -(1000000 as i32) - 1000 as i32 - 1 as i32,
    ) as *mut RanState;
    if lua_type(L, 1 as i32) == -(1 as i32) {
        randseed(L, state);
    } else {
        let mut n1: Integer = luaL_checkinteger(L, 1 as i32);
        let mut n2: Integer = luaL_optinteger(
            L,
            2 as i32,
            0 as i32 as Integer,
        );
        setseed(L, ((*state).s).as_mut_ptr(), n1 as lua_Unsigned, n2 as lua_Unsigned);
    }
    return 2 as i32;
}
static mut randfuncs: [luaL_Reg; 3] = unsafe {
    [
        {
            let mut init = luaL_Reg {
                name: b"random\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_random as unsafe extern "C" fn(*mut lua_State) -> i32,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"randomseed\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_randomseed
                        as unsafe extern "C" fn(*mut lua_State) -> i32,
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
unsafe extern "C" fn setrandfunc(mut L: *mut lua_State) {
    let mut state: *mut RanState = lua_newuserdatauv(
        L,
        ::core::mem::size_of::<RanState>() as libc::c_ulong,
        0 as i32,
    ) as *mut RanState;
    randseed(L, state);
    lua_settop(L, -(2 as i32) - 1 as i32);
    luaL_setfuncs(L, randfuncs.as_ptr(), 1 as i32);
}
static mut mathlib: [luaL_Reg; 28] = unsafe {
    [
        {
            let mut init = luaL_Reg {
                name: b"abs\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_abs as unsafe extern "C" fn(*mut lua_State) -> i32,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"acos\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_acos as unsafe extern "C" fn(*mut lua_State) -> i32,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"asin\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_asin as unsafe extern "C" fn(*mut lua_State) -> i32,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"atan\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_atan as unsafe extern "C" fn(*mut lua_State) -> i32,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"ceil\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_ceil as unsafe extern "C" fn(*mut lua_State) -> i32,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"cos\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_cos as unsafe extern "C" fn(*mut lua_State) -> i32,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"deg\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_deg as unsafe extern "C" fn(*mut lua_State) -> i32,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"exp\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_exp as unsafe extern "C" fn(*mut lua_State) -> i32,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"tointeger\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_toint as unsafe extern "C" fn(*mut lua_State) -> i32,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"floor\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_floor as unsafe extern "C" fn(*mut lua_State) -> i32,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"fmod\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_fmod as unsafe extern "C" fn(*mut lua_State) -> i32,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"ult\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_ult as unsafe extern "C" fn(*mut lua_State) -> i32,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"log\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_log as unsafe extern "C" fn(*mut lua_State) -> i32,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"max\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_max as unsafe extern "C" fn(*mut lua_State) -> i32,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"min\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_min as unsafe extern "C" fn(*mut lua_State) -> i32,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"modf\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_modf as unsafe extern "C" fn(*mut lua_State) -> i32,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"rad\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_rad as unsafe extern "C" fn(*mut lua_State) -> i32,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"sin\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_sin as unsafe extern "C" fn(*mut lua_State) -> i32,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"sqrt\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_sqrt as unsafe extern "C" fn(*mut lua_State) -> i32,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"tan\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_tan as unsafe extern "C" fn(*mut lua_State) -> i32,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"type\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_type as unsafe extern "C" fn(*mut lua_State) -> i32,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"random\0" as *const u8 as *const libc::c_char,
                func: None,
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"randomseed\0" as *const u8 as *const libc::c_char,
                func: None,
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"pi\0" as *const u8 as *const libc::c_char,
                func: None,
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"huge\0" as *const u8 as *const libc::c_char,
                func: None,
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"maxinteger\0" as *const u8 as *const libc::c_char,
                func: None,
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"mininteger\0" as *const u8 as *const libc::c_char,
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
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaopen_math(mut L: *mut lua_State) -> i32 {
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
        (::core::mem::size_of::<[luaL_Reg; 28]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<luaL_Reg>() as libc::c_ulong)
            .wrapping_sub(1 as i32 as libc::c_ulong) as i32,
    );
    luaL_setfuncs(L, mathlib.as_ptr(), 0 as i32);
    lua_pushnumber(L, 3.141592653589793238462643383279502884f64);
    lua_setfield(L, -(2 as i32), b"pi\0" as *const u8 as *const libc::c_char);
    lua_pushnumber(L, ::core::f64::INFINITY);
    lua_setfield(L, -(2 as i32), b"huge\0" as *const u8 as *const libc::c_char);
    lua_pushinteger(L, 9223372036854775807 as i64);
    lua_setfield(
        L,
        -(2 as i32),
        b"maxinteger\0" as *const u8 as *const libc::c_char,
    );
    lua_pushinteger(
        L,
        -(9223372036854775807 as i64) - 1 as i64,
    );
    lua_setfield(
        L,
        -(2 as i32),
        b"mininteger\0" as *const u8 as *const libc::c_char,
    );
    setrandfunc(L);
    return 1 as i32;
}
