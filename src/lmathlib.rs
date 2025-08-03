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
    pub type lua_State;
    fn acos(_: libc::c_double) -> libc::c_double;
    fn asin(_: libc::c_double) -> libc::c_double;
    fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn tan(_: libc::c_double) -> libc::c_double;
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn log10(_: libc::c_double) -> libc::c_double;
    fn log2(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn fmod(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn time(__timer: *mut time_t) -> time_t;
    fn lua_gettop(L: *mut lua_State) -> libc::c_int;
    fn lua_settop(L: *mut lua_State, idx: libc::c_int);
    fn lua_pushvalue(L: *mut lua_State, idx: libc::c_int);
    fn lua_isinteger(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    fn lua_type(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    fn lua_tointegerx(
        L: *mut lua_State,
        idx: libc::c_int,
        isnum: *mut libc::c_int,
    ) -> lua_Integer;
    fn lua_touserdata(L: *mut lua_State, idx: libc::c_int) -> *mut libc::c_void;
    fn lua_compare(
        L: *mut lua_State,
        idx1: libc::c_int,
        idx2: libc::c_int,
        op: libc::c_int,
    ) -> libc::c_int;
    fn lua_pushnil(L: *mut lua_State);
    fn lua_pushnumber(L: *mut lua_State, n: lua_Number);
    fn lua_pushinteger(L: *mut lua_State, n: lua_Integer);
    fn lua_pushstring(L: *mut lua_State, s: *const libc::c_char) -> *const libc::c_char;
    fn lua_pushboolean(L: *mut lua_State, b: libc::c_int);
    fn lua_createtable(L: *mut lua_State, narr: libc::c_int, nrec: libc::c_int);
    fn lua_newuserdatauv(
        L: *mut lua_State,
        sz: size_t,
        nuvalue: libc::c_int,
    ) -> *mut libc::c_void;
    fn lua_setfield(L: *mut lua_State, idx: libc::c_int, k: *const libc::c_char);
    fn luaL_checkversion_(L: *mut lua_State, ver: lua_Number, sz: size_t);
    fn luaL_argerror(
        L: *mut lua_State,
        arg: libc::c_int,
        extramsg: *const libc::c_char,
    ) -> libc::c_int;
    fn luaL_checknumber(L: *mut lua_State, arg: libc::c_int) -> lua_Number;
    fn luaL_optnumber(
        L: *mut lua_State,
        arg: libc::c_int,
        def: lua_Number,
    ) -> lua_Number;
    fn luaL_checkinteger(L: *mut lua_State, arg: libc::c_int) -> lua_Integer;
    fn luaL_optinteger(
        L: *mut lua_State,
        arg: libc::c_int,
        def: lua_Integer,
    ) -> lua_Integer;
    fn luaL_checkany(L: *mut lua_State, arg: libc::c_int);
    fn luaL_error(L: *mut lua_State, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn luaL_setfuncs(L: *mut lua_State, l: *const luaL_Reg, nup: libc::c_int);
}
pub type __time_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type time_t = __time_t;
pub type lua_Number = libc::c_double;
pub type lua_Integer = libc::c_longlong;
pub type lua_Unsigned = libc::c_ulonglong;
pub type lua_CFunction = Option::<unsafe extern "C" fn(*mut lua_State) -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaL_Reg {
    pub name: *const libc::c_char,
    pub func: lua_CFunction,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RanState {
    pub s: [libc::c_ulong; 4],
}
unsafe extern "C" fn math_abs(mut L: *mut lua_State) -> libc::c_int {
    if lua_isinteger(L, 1 as libc::c_int) != 0 {
        let mut n: lua_Integer = lua_tointegerx(
            L,
            1 as libc::c_int,
            0 as *mut libc::c_int,
        );
        if n < 0 as libc::c_int as libc::c_longlong {
            n = (0 as libc::c_uint as libc::c_ulonglong).wrapping_sub(n as lua_Unsigned)
                as lua_Integer;
        }
        lua_pushinteger(L, n);
    } else {
        lua_pushnumber(L, fabs(luaL_checknumber(L, 1 as libc::c_int)));
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn math_sin(mut L: *mut lua_State) -> libc::c_int {
    lua_pushnumber(L, sin(luaL_checknumber(L, 1 as libc::c_int)));
    return 1 as libc::c_int;
}
unsafe extern "C" fn math_cos(mut L: *mut lua_State) -> libc::c_int {
    lua_pushnumber(L, cos(luaL_checknumber(L, 1 as libc::c_int)));
    return 1 as libc::c_int;
}
unsafe extern "C" fn math_tan(mut L: *mut lua_State) -> libc::c_int {
    lua_pushnumber(L, tan(luaL_checknumber(L, 1 as libc::c_int)));
    return 1 as libc::c_int;
}
unsafe extern "C" fn math_asin(mut L: *mut lua_State) -> libc::c_int {
    lua_pushnumber(L, asin(luaL_checknumber(L, 1 as libc::c_int)));
    return 1 as libc::c_int;
}
unsafe extern "C" fn math_acos(mut L: *mut lua_State) -> libc::c_int {
    lua_pushnumber(L, acos(luaL_checknumber(L, 1 as libc::c_int)));
    return 1 as libc::c_int;
}
unsafe extern "C" fn math_atan(mut L: *mut lua_State) -> libc::c_int {
    let mut y: lua_Number = luaL_checknumber(L, 1 as libc::c_int);
    let mut x: lua_Number = luaL_optnumber(
        L,
        2 as libc::c_int,
        1 as libc::c_int as lua_Number,
    );
    lua_pushnumber(L, atan2(y, x));
    return 1 as libc::c_int;
}
unsafe extern "C" fn math_toint(mut L: *mut lua_State) -> libc::c_int {
    let mut valid: libc::c_int = 0;
    let mut n: lua_Integer = lua_tointegerx(L, 1 as libc::c_int, &mut valid);
    if (valid != 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        lua_pushinteger(L, n);
    } else {
        luaL_checkany(L, 1 as libc::c_int);
        lua_pushnil(L);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn pushnumint(mut L: *mut lua_State, mut d: lua_Number) {
    let mut n: lua_Integer = 0;
    if d
        >= (-(9223372036854775807 as libc::c_longlong) - 1 as libc::c_longlong)
            as libc::c_double
        && d
            < -((-(9223372036854775807 as libc::c_longlong) - 1 as libc::c_longlong)
                as libc::c_double)
        && {
            n = d as libc::c_longlong;
            1 as libc::c_int != 0
        }
    {
        lua_pushinteger(L, n);
    } else {
        lua_pushnumber(L, d);
    };
}
unsafe extern "C" fn math_floor(mut L: *mut lua_State) -> libc::c_int {
    if lua_isinteger(L, 1 as libc::c_int) != 0 {
        lua_settop(L, 1 as libc::c_int);
    } else {
        let mut d: lua_Number = floor(luaL_checknumber(L, 1 as libc::c_int));
        pushnumint(L, d);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn math_ceil(mut L: *mut lua_State) -> libc::c_int {
    if lua_isinteger(L, 1 as libc::c_int) != 0 {
        lua_settop(L, 1 as libc::c_int);
    } else {
        let mut d: lua_Number = ceil(luaL_checknumber(L, 1 as libc::c_int));
        pushnumint(L, d);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn math_fmod(mut L: *mut lua_State) -> libc::c_int {
    if lua_isinteger(L, 1 as libc::c_int) != 0 && lua_isinteger(L, 2 as libc::c_int) != 0
    {
        let mut d: lua_Integer = lua_tointegerx(
            L,
            2 as libc::c_int,
            0 as *mut libc::c_int,
        );
        if (d as lua_Unsigned).wrapping_add(1 as libc::c_uint as libc::c_ulonglong)
            <= 1 as libc::c_uint as libc::c_ulonglong
        {
            (((d != 0 as libc::c_int as libc::c_longlong) as libc::c_int
                != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
                || luaL_argerror(
                    L,
                    2 as libc::c_int,
                    b"zero\0" as *const u8 as *const libc::c_char,
                ) != 0) as libc::c_int;
            lua_pushinteger(L, 0 as libc::c_int as lua_Integer);
        } else {
            lua_pushinteger(
                L,
                lua_tointegerx(L, 1 as libc::c_int, 0 as *mut libc::c_int) % d,
            );
        }
    } else {
        lua_pushnumber(
            L,
            fmod(
                luaL_checknumber(L, 1 as libc::c_int),
                luaL_checknumber(L, 2 as libc::c_int),
            ),
        );
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn math_modf(mut L: *mut lua_State) -> libc::c_int {
    if lua_isinteger(L, 1 as libc::c_int) != 0 {
        lua_settop(L, 1 as libc::c_int);
        lua_pushnumber(L, 0 as libc::c_int as lua_Number);
    } else {
        let mut n: lua_Number = luaL_checknumber(L, 1 as libc::c_int);
        let mut ip: lua_Number = if n < 0 as libc::c_int as libc::c_double {
            ceil(n)
        } else {
            floor(n)
        };
        pushnumint(L, ip);
        lua_pushnumber(L, if n == ip { 0.0f64 } else { n - ip });
    }
    return 2 as libc::c_int;
}
unsafe extern "C" fn math_sqrt(mut L: *mut lua_State) -> libc::c_int {
    lua_pushnumber(L, sqrt(luaL_checknumber(L, 1 as libc::c_int)));
    return 1 as libc::c_int;
}
unsafe extern "C" fn math_ult(mut L: *mut lua_State) -> libc::c_int {
    let mut a: lua_Integer = luaL_checkinteger(L, 1 as libc::c_int);
    let mut b: lua_Integer = luaL_checkinteger(L, 2 as libc::c_int);
    lua_pushboolean(L, ((a as lua_Unsigned) < b as lua_Unsigned) as libc::c_int);
    return 1 as libc::c_int;
}
unsafe extern "C" fn math_log(mut L: *mut lua_State) -> libc::c_int {
    let mut x: lua_Number = luaL_checknumber(L, 1 as libc::c_int);
    let mut res: lua_Number = 0.;
    if lua_type(L, 2 as libc::c_int) <= 0 as libc::c_int {
        res = log(x);
    } else {
        let mut base: lua_Number = luaL_checknumber(L, 2 as libc::c_int);
        if base == 2.0f64 {
            res = log2(x);
        } else if base == 10.0f64 {
            res = log10(x);
        } else {
            res = log(x) / log(base);
        }
    }
    lua_pushnumber(L, res);
    return 1 as libc::c_int;
}
unsafe extern "C" fn math_exp(mut L: *mut lua_State) -> libc::c_int {
    lua_pushnumber(L, exp(luaL_checknumber(L, 1 as libc::c_int)));
    return 1 as libc::c_int;
}
unsafe extern "C" fn math_deg(mut L: *mut lua_State) -> libc::c_int {
    lua_pushnumber(
        L,
        luaL_checknumber(L, 1 as libc::c_int)
            * (180.0f64 / 3.141592653589793238462643383279502884f64),
    );
    return 1 as libc::c_int;
}
unsafe extern "C" fn math_rad(mut L: *mut lua_State) -> libc::c_int {
    lua_pushnumber(
        L,
        luaL_checknumber(L, 1 as libc::c_int)
            * (3.141592653589793238462643383279502884f64 / 180.0f64),
    );
    return 1 as libc::c_int;
}
unsafe extern "C" fn math_min(mut L: *mut lua_State) -> libc::c_int {
    let mut n: libc::c_int = lua_gettop(L);
    let mut imin: libc::c_int = 1 as libc::c_int;
    let mut i: libc::c_int = 0;
    (((n >= 1 as libc::c_int) as libc::c_int != 0 as libc::c_int) as libc::c_int
        as libc::c_long != 0
        || luaL_argerror(
            L,
            1 as libc::c_int,
            b"value expected\0" as *const u8 as *const libc::c_char,
        ) != 0) as libc::c_int;
    i = 2 as libc::c_int;
    while i <= n {
        if lua_compare(L, i, imin, 1 as libc::c_int) != 0 {
            imin = i;
        }
        i += 1;
        i;
    }
    lua_pushvalue(L, imin);
    return 1 as libc::c_int;
}
unsafe extern "C" fn math_max(mut L: *mut lua_State) -> libc::c_int {
    let mut n: libc::c_int = lua_gettop(L);
    let mut imax: libc::c_int = 1 as libc::c_int;
    let mut i: libc::c_int = 0;
    (((n >= 1 as libc::c_int) as libc::c_int != 0 as libc::c_int) as libc::c_int
        as libc::c_long != 0
        || luaL_argerror(
            L,
            1 as libc::c_int,
            b"value expected\0" as *const u8 as *const libc::c_char,
        ) != 0) as libc::c_int;
    i = 2 as libc::c_int;
    while i <= n {
        if lua_compare(L, imax, i, 1 as libc::c_int) != 0 {
            imax = i;
        }
        i += 1;
        i;
    }
    lua_pushvalue(L, imax);
    return 1 as libc::c_int;
}
unsafe extern "C" fn math_type(mut L: *mut lua_State) -> libc::c_int {
    if lua_type(L, 1 as libc::c_int) == 3 as libc::c_int {
        lua_pushstring(
            L,
            if lua_isinteger(L, 1 as libc::c_int) != 0 {
                b"integer\0" as *const u8 as *const libc::c_char
            } else {
                b"float\0" as *const u8 as *const libc::c_char
            },
        );
    } else {
        luaL_checkany(L, 1 as libc::c_int);
        lua_pushnil(L);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn rotl(mut x: libc::c_ulong, mut n: libc::c_int) -> libc::c_ulong {
    return x << n | (x & 0xffffffffffffffff as libc::c_ulong) >> 64 as libc::c_int - n;
}
unsafe extern "C" fn nextrand(mut state: *mut libc::c_ulong) -> libc::c_ulong {
    let mut state0: libc::c_ulong = *state.offset(0 as libc::c_int as isize);
    let mut state1: libc::c_ulong = *state.offset(1 as libc::c_int as isize);
    let mut state2: libc::c_ulong = *state.offset(2 as libc::c_int as isize) ^ state0;
    let mut state3: libc::c_ulong = *state.offset(3 as libc::c_int as isize) ^ state1;
    let mut res: libc::c_ulong = (rotl(
        state1.wrapping_mul(5 as libc::c_int as libc::c_ulong),
        7 as libc::c_int,
    ))
        .wrapping_mul(9 as libc::c_int as libc::c_ulong);
    *state.offset(0 as libc::c_int as isize) = state0 ^ state3;
    *state.offset(1 as libc::c_int as isize) = state1 ^ state2;
    *state.offset(2 as libc::c_int as isize) = state2 ^ state1 << 17 as libc::c_int;
    *state.offset(3 as libc::c_int as isize) = rotl(state3, 45 as libc::c_int);
    return res;
}
unsafe extern "C" fn I2d(mut x: libc::c_ulong) -> lua_Number {
    let mut sx: libc::c_long = ((x & 0xffffffffffffffff as libc::c_ulong)
        >> 64 as libc::c_int - 53 as libc::c_int) as libc::c_long;
    let mut res: lua_Number = sx as lua_Number
        * (0.5f64
            / ((1 as libc::c_int as libc::c_ulong)
                << 53 as libc::c_int - 1 as libc::c_int) as libc::c_double);
    if sx < 0 as libc::c_int as libc::c_long {
        res += 1.0f64;
    }
    return res;
}
unsafe extern "C" fn project(
    mut ran: lua_Unsigned,
    mut n: lua_Unsigned,
    mut state: *mut RanState,
) -> lua_Unsigned {
    if n & n.wrapping_add(1 as libc::c_int as libc::c_ulonglong)
        == 0 as libc::c_int as libc::c_ulonglong
    {
        return ran & n
    } else {
        let mut lim: lua_Unsigned = n;
        lim |= lim >> 1 as libc::c_int;
        lim |= lim >> 2 as libc::c_int;
        lim |= lim >> 4 as libc::c_int;
        lim |= lim >> 8 as libc::c_int;
        lim |= lim >> 16 as libc::c_int;
        lim |= lim >> 32 as libc::c_int;
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
unsafe extern "C" fn math_random(mut L: *mut lua_State) -> libc::c_int {
    let mut low: lua_Integer = 0;
    let mut up: lua_Integer = 0;
    let mut p: lua_Unsigned = 0;
    let mut state: *mut RanState = lua_touserdata(
        L,
        -(1000000 as libc::c_int) - 1000 as libc::c_int - 1 as libc::c_int,
    ) as *mut RanState;
    let mut rv: libc::c_ulong = nextrand(((*state).s).as_mut_ptr());
    match lua_gettop(L) {
        0 => {
            lua_pushnumber(L, I2d(rv));
            return 1 as libc::c_int;
        }
        1 => {
            low = 1 as libc::c_int as lua_Integer;
            up = luaL_checkinteger(L, 1 as libc::c_int);
            if up == 0 as libc::c_int as libc::c_longlong {
                lua_pushinteger(
                    L,
                    (rv & 0xffffffffffffffff as libc::c_ulong) as lua_Unsigned
                        as lua_Integer,
                );
                return 1 as libc::c_int;
            }
        }
        2 => {
            low = luaL_checkinteger(L, 1 as libc::c_int);
            up = luaL_checkinteger(L, 2 as libc::c_int);
        }
        _ => {
            return luaL_error(
                L,
                b"wrong number of arguments\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    (((low <= up) as libc::c_int != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
        || luaL_argerror(
            L,
            1 as libc::c_int,
            b"interval is empty\0" as *const u8 as *const libc::c_char,
        ) != 0) as libc::c_int;
    p = project(
        (rv & 0xffffffffffffffff as libc::c_ulong) as lua_Unsigned,
        (up as lua_Unsigned).wrapping_sub(low as lua_Unsigned),
        state,
    );
    lua_pushinteger(L, p.wrapping_add(low as lua_Unsigned) as lua_Integer);
    return 1 as libc::c_int;
}
unsafe extern "C" fn setseed(
    mut L: *mut lua_State,
    mut state: *mut libc::c_ulong,
    mut n1: lua_Unsigned,
    mut n2: lua_Unsigned,
) {
    let mut i: libc::c_int = 0;
    *state.offset(0 as libc::c_int as isize) = n1 as libc::c_ulong;
    *state.offset(1 as libc::c_int as isize) = 0xff as libc::c_int as libc::c_ulong;
    *state.offset(2 as libc::c_int as isize) = n2 as libc::c_ulong;
    *state.offset(3 as libc::c_int as isize) = 0 as libc::c_int as libc::c_ulong;
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        nextrand(state);
        i += 1;
        i;
    }
    lua_pushinteger(L, n1 as lua_Integer);
    lua_pushinteger(L, n2 as lua_Integer);
}
unsafe extern "C" fn randseed(mut L: *mut lua_State, mut state: *mut RanState) {
    let mut seed1: lua_Unsigned = time(0 as *mut time_t) as lua_Unsigned;
    let mut seed2: lua_Unsigned = L as size_t as lua_Unsigned;
    setseed(L, ((*state).s).as_mut_ptr(), seed1, seed2);
}
unsafe extern "C" fn math_randomseed(mut L: *mut lua_State) -> libc::c_int {
    let mut state: *mut RanState = lua_touserdata(
        L,
        -(1000000 as libc::c_int) - 1000 as libc::c_int - 1 as libc::c_int,
    ) as *mut RanState;
    if lua_type(L, 1 as libc::c_int) == -(1 as libc::c_int) {
        randseed(L, state);
    } else {
        let mut n1: lua_Integer = luaL_checkinteger(L, 1 as libc::c_int);
        let mut n2: lua_Integer = luaL_optinteger(
            L,
            2 as libc::c_int,
            0 as libc::c_int as lua_Integer,
        );
        setseed(L, ((*state).s).as_mut_ptr(), n1 as lua_Unsigned, n2 as lua_Unsigned);
    }
    return 2 as libc::c_int;
}
static mut randfuncs: [luaL_Reg; 3] = unsafe {
    [
        {
            let mut init = luaL_Reg {
                name: b"random\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_random as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"randomseed\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_randomseed
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
unsafe extern "C" fn setrandfunc(mut L: *mut lua_State) {
    let mut state: *mut RanState = lua_newuserdatauv(
        L,
        ::core::mem::size_of::<RanState>() as libc::c_ulong,
        0 as libc::c_int,
    ) as *mut RanState;
    randseed(L, state);
    lua_settop(L, -(2 as libc::c_int) - 1 as libc::c_int);
    luaL_setfuncs(L, randfuncs.as_ptr(), 1 as libc::c_int);
}
static mut mathlib: [luaL_Reg; 28] = unsafe {
    [
        {
            let mut init = luaL_Reg {
                name: b"abs\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_abs as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"acos\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_acos as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"asin\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_asin as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"atan\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_atan as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"ceil\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_ceil as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"cos\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_cos as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"deg\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_deg as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"exp\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_exp as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"tointeger\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_toint as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"floor\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_floor as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"fmod\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_fmod as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"ult\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_ult as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"log\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_log as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"max\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_max as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"min\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_min as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"modf\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_modf as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"rad\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_rad as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"sin\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_sin as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"sqrt\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_sqrt as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"tan\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_tan as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"type\0" as *const u8 as *const libc::c_char,
                func: Some(
                    math_type as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
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
#[no_mangle]
pub unsafe extern "C" fn luaopen_math(mut L: *mut lua_State) -> libc::c_int {
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
        (::core::mem::size_of::<[luaL_Reg; 28]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<luaL_Reg>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
    );
    luaL_setfuncs(L, mathlib.as_ptr(), 0 as libc::c_int);
    lua_pushnumber(L, 3.141592653589793238462643383279502884f64);
    lua_setfield(L, -(2 as libc::c_int), b"pi\0" as *const u8 as *const libc::c_char);
    lua_pushnumber(L, ::core::f64::INFINITY);
    lua_setfield(L, -(2 as libc::c_int), b"huge\0" as *const u8 as *const libc::c_char);
    lua_pushinteger(L, 9223372036854775807 as libc::c_longlong);
    lua_setfield(
        L,
        -(2 as libc::c_int),
        b"maxinteger\0" as *const u8 as *const libc::c_char,
    );
    lua_pushinteger(
        L,
        -(9223372036854775807 as libc::c_longlong) - 1 as libc::c_longlong,
    );
    lua_setfield(
        L,
        -(2 as libc::c_int),
        b"mininteger\0" as *const u8 as *const libc::c_char,
    );
    setrandfunc(L);
    return 1 as libc::c_int;
}
