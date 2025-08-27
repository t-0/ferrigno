#![allow(
    static_mut_refs,
    unsafe_code,
)]
use crate::c::*;
use crate::randomstate::*;
use crate::onelua::*;
use crate::state::*;
use crate::registeredfunction::*;
const PI: f64 = 3.141592653589793238462643383279502884f64;
unsafe extern "C" fn math_abs(state: *mut State) -> i32 { unsafe {
    if lua_isinteger(state, 1) {
        let mut n: i64 = lua_tointegerx(state, 1, std::ptr::null_mut());
        if n < 0 {
            n = (0u64).wrapping_sub(n as u64) as i64;
        }
        (*state).push_integer(n);
    } else {
        (*state).push_number(lual_checknumber(state, 1).abs());
    }
    1
}}
unsafe extern "C" fn math_sin(state: *mut State) -> i32 { unsafe {
    (*state).push_number(lual_checknumber(state, 1).sin());
    1
}}
unsafe extern "C" fn math_cos(state: *mut State) -> i32 { unsafe {
    (*state).push_number(lual_checknumber(state, 1).cos());
    1
}}
unsafe extern "C" fn math_tan(state: *mut State) -> i32 { unsafe {
    (*state).push_number(lual_checknumber(state, 1).tan());
    1
}}
unsafe extern "C" fn math_asin(state: *mut State) -> i32 { unsafe {
    (*state).push_number(lual_checknumber(state, 1).asin());
    1
}}
unsafe extern "C" fn math_acos(state: *mut State) -> i32 { unsafe {
    (*state).push_number(lual_checknumber(state, 1).acos());
    1
}}
unsafe extern "C" fn math_atan(state: *mut State) -> i32 { unsafe {
    let y: f64 = lual_checknumber(state, 1);
    let x: f64 = lual_optnumber(state, 2, 1.0);
    (*state).push_number(y.atan2(x));
    1
}}
unsafe extern "C" fn math_toint(state: *mut State) -> i32 { unsafe {
    let mut is_number: bool = false;
    let n: i64 = lua_tointegerx(state, 1, &mut is_number);
    if is_number {
        (*state).push_integer(n);
    } else {
        lual_checkany(state, 1);
        (*state).push_nil();
    }
    1
}}
unsafe extern "C" fn push_numericcc(state: *mut State, d: f64) { unsafe {
    let mut n: i64 = 0;
    if d >= (-(9223372036854775807 as i64) - 1 as i64) as f64
        && d < -((-(9223372036854775807 as i64) - 1 as i64) as f64)
        && {
            n = d as i64;
            1 != 0
        }
    {
        (*state).push_integer(n);
    } else {
        (*state).push_number(d);
    };
}}
unsafe extern "C" fn math_floor(state: *mut State) -> i32 { unsafe {
    if lua_isinteger(state, 1) {
        lua_settop(state, 1);
    } else {
        let d: f64 = lual_checknumber(state, 1).floor();
        push_numericcc(state, d);
    }
    1
}}
unsafe extern "C" fn math_ceil(state: *mut State) -> i32 { unsafe {
    if lua_isinteger(state, 1) {
        lua_settop(state, 1);
    } else {
        let d: f64 = lual_checknumber(state, 1).ceil();
        push_numericcc(state, d);
    }
    1
}}
unsafe extern "C" fn math_fmod(state: *mut State) -> i32 { unsafe {
    if lua_isinteger(state, 1) && lua_isinteger(state, 2) {
        let d: i64 = lua_tointegerx(state, 2, std::ptr::null_mut());
        if (d as u64).wrapping_add(1 as u32 as u64) <= 1 as u32 as u64 {
            (((d != 0) as i32 != 0) as i32 as i64 != 0
                || lual_argerror(state, 2, b"zero\0" as *const u8 as *const i8) != 0)
                as i32;
            (*state).push_integer(0);
        } else {
            (*state).push_integer(lua_tointegerx(state, 1, std::ptr::null_mut()) % d);
        }
    } else {
        (*state).push_number(
            fmod(lual_checknumber(state, 1), lual_checknumber(state, 2)),
        );
    }
    1
}}
unsafe extern "C" fn math_modf(state: *mut State) -> i32 { unsafe {
    if lua_isinteger(state, 1) {
        lua_settop(state, 1);
        (*state).push_number(0.0);
    } else {
        let n: f64 = lual_checknumber(state, 1);
        let ip: f64 = if n < 0.0 { n.ceil() } else { n.floor() };
        push_numericcc(state, ip);
        (*state).push_number(if n == ip { 0.0 } else { n - ip });
    }
    2
}}
unsafe extern "C" fn math_sqrt(state: *mut State) -> i32 { unsafe {
    (*state).push_number(lual_checknumber(state, 1).sqrt());
    1
}}
unsafe extern "C" fn math_ult(state: *mut State) -> i32 { unsafe {
    let a: i64 = lual_checkinteger(state, 1);
    let b: i64 = lual_checkinteger(state, 2);
    (*state).push_boolean((a as u64) < (b as u64));
    1
}}
unsafe extern "C" fn math_log(state: *mut State) -> i32 { unsafe {
    let x: f64 = lual_checknumber(state, 1);
    let res: f64;
    if lua_type(state, 2) <= 0 {
        res = x.ln();
    } else {
        let base: f64 = lual_checknumber(state, 2);
        if base == 2.0f64 {
            res = x.log2();
        } else if base == 10.0f64 {
            res = x.log10();
        } else {
            res = x.ln() / base.ln();
        }
    }
    (*state).push_number(res);
    1
}}
unsafe extern "C" fn math_exp(state: *mut State) -> i32 { unsafe {
    (*state).push_number(lual_checknumber(state, 1).exp());
    1
}}
unsafe extern "C" fn math_deg(state: *mut State) -> i32 { unsafe {
    (*state).push_number(lual_checknumber(state, 1) * (180.0f64 / PI));
    1
}}
unsafe extern "C" fn math_rad(state: *mut State) -> i32 { unsafe {
    (*state).push_number(lual_checknumber(state, 1) * (PI / 180.0f64));
    1
}}
unsafe extern "C" fn math_min(state: *mut State) -> i32 { unsafe {
    let n: i32 = (*state).get_top();
    let mut imin: i32 = 1;
    let mut i: i32;
    (((n >= 1) as i32 != 0) as i32 as i64 != 0
        || lual_argerror(state, 1, b"value expected\0" as *const u8 as *const i8) != 0)
        as i32;
    i = 2;
    while i <= n {
        if lua_compare(state, i, imin, 1) != 0 {
            imin = i;
        }
        i += 1;
    }
    lua_pushvalue(state, imin);
    1
}}
unsafe extern "C" fn math_max(state: *mut State) -> i32 { unsafe {
    let n: i32 = (*state).get_top();
    let mut imax: i32 = 1;
    let mut i: i32;
    (((n >= 1) as i32 != 0) as i32 as i64 != 0
        || lual_argerror(state, 1, b"value expected\0" as *const u8 as *const i8) != 0)
        as i32;
    i = 2;
    while i <= n {
        if lua_compare(state, imax, i, 1) != 0 {
            imax = i;
        }
        i += 1;
    }
    lua_pushvalue(state, imax);
    1
}}
unsafe extern "C" fn math_type(state: *mut State) -> i32 { unsafe {
    if lua_type(state, 1) == 3 {
        lua_pushstring(
            state,
            if lua_isinteger(state, 1) {
                b"integer\0" as *const u8 as *const i8
            } else {
                b"float\0" as *const u8 as *const i8
            },
        );
    } else {
        lual_checkany(state, 1);
        (*state).push_nil();
    }
    1
}}
unsafe extern "C" fn rotate_left(x: u64, n: i32) -> u64 {
    (x << n) | ((x & 0xffffffffffffffff as u64) >> (64 - n))
}
unsafe extern "C" fn next_random(randomstate: *mut u64) -> u64 { unsafe {
    let state0: u64 = *randomstate.offset(0 as isize);
    let state1: u64 = *randomstate.offset(1 as isize);
    let state2: u64 = *randomstate.offset(2 as isize) ^ state0;
    let state3: u64 = *randomstate.offset(3 as isize) ^ state1;
    let res: u64 = (rotate_left(state1.wrapping_mul(5 as u64), 7)).wrapping_mul(9 as i32 as u64);
    *randomstate.offset(0 as isize) = state0 ^ state3;
    *randomstate.offset(1 as isize) = state1 ^ state2;
    *randomstate.offset(2 as isize) = state2 ^ state1 << 17 as i32;
    *randomstate.offset(3 as isize) = rotate_left(state3, 45 as i32);
    res
}}
unsafe extern "C" fn i2d(x: u64) -> f64 {
    let sx: i64 = ((x & 0xffffffffffffffff as u64) >> (64 - 53)) as i64;
    let mut res: f64 = sx as f64 * (0.5f64 / ((1 as u64) << (53 - 1)) as f64);
    if sx < 0 {
        res += 1.0f64;
    }
    res
}
unsafe extern "C" fn project(mut ran: u64, n: u64, ransate: *mut RandomState) -> u64 { unsafe {
    if n & n.wrapping_add(1 as u64) == 0u64 {
        return ran & n;
    } else {
        let mut lim: u64 = n;
        lim |= lim >> 1;
        lim |= lim >> 2;
        lim |= lim >> 4;
        lim |= lim >> 8;
        lim |= lim >> 16;
        lim |= lim >> 32;
        loop {
            ran &= lim;
            if !(ran > n) {
                break;
            }
            ran = (next_random(((*ransate).s).as_mut_ptr()) & 0xffffffffffffffff as u64) as u64;
        }
        return ran;
    };
}}
unsafe extern "C" fn math_random(state: *mut State) -> i32 { unsafe {
    let low: i64;
    let up: i64;
    let p: u64;
    let ransate: *mut RandomState =
        lua_touserdata(state, -(1000000 as i32) - 1000 as i32 - 1) as *mut RandomState;
    let rv: u64 = next_random(((*ransate).s).as_mut_ptr());
    match (*state).get_top() {
        0 => {
            (*state).push_number(i2d(rv));
            return 1;
        }
        1 => {
            low = 1;
            up = lual_checkinteger(state, 1);
            if up == 0 {
                (*state).push_integer((rv & 0xffffffffffffffff as u64) as u64 as i64);
                return 1;
            }
        }
        2 => {
            low = lual_checkinteger(state, 1);
            up = lual_checkinteger(state, 2);
        }
        _ => {
            return lual_error(
                state,
                b"wrong number of arguments\0" as *const u8 as *const i8,
            );
        }
    }
    (((low <= up) as i32 != 0) as i32 as i64 != 0
        || lual_argerror(
            state,
            1,
            b"interval is empty\0" as *const u8 as *const i8,
        ) != 0) as i32;
    p = project(
        (rv & 0xffffffffffffffff as u64) as u64,
        (up as u64).wrapping_sub(low as u64),
        ransate,
    );
    (*state).push_integer(p.wrapping_add(low as u64) as i64);
    return 1;
}}
unsafe extern "C" fn set_seed(
    state: *mut State,
    randomstate: *mut u64,
    n1: u64,
    n2: u64,
) { unsafe {
    let mut i: i32;
    *randomstate.offset(0 as isize) = n1 as u64;
    *randomstate.offset(1 as isize) = 0xFF as i32 as u64;
    *randomstate.offset(2 as isize) = n2 as u64;
    *randomstate.offset(3 as isize) = 0;
    i = 0;
    while i < 16 as i32 {
        next_random(randomstate);
        i += 1;
    }
    (*state).push_integer(n1 as i64);
    (*state).push_integer(n2 as i64);
}}
unsafe extern "C" fn random_seed(state: *mut State, randomstate: *mut RandomState) { unsafe {
    let seed1: u64 = time(std::ptr::null_mut()) as u64;
    let seed2: u64 = state as u64 as u64;
    set_seed(state, ((*randomstate).s).as_mut_ptr(), seed1, seed2);
}}
unsafe extern "C" fn math_randomseed(state: *mut State) -> i32 { unsafe {
    let randomstate: *mut RandomState =
        lua_touserdata(state, -(1000000 as i32) - 1000 as i32 - 1) as *mut RandomState;
    if lua_type(state, 1) == -1 {
        random_seed(state, randomstate);
    } else {
        let n1: i64 = lual_checkinteger(state, 1);
        let n2: i64 = lual_optinteger(state, 2, 0);
        set_seed(state, ((*randomstate).s).as_mut_ptr(), n1 as u64, n2 as u64);
    }
    return 2;
}}
static mut MATH_RANDOM_FUNCTIONS: [RegisteredFunction; 3] = {
    [
        {
            RegisteredFunction {
                name: b"random\0" as *const u8 as *const i8,
                function: Some(math_random as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"randomseed\0" as *const u8 as *const i8,
                function: Some(math_randomseed as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: std::ptr::null(),
                function: None,
            }
        },
    ]
};
unsafe extern "C" fn set_random_function(state: *mut State) { unsafe {
    let ranstate: *mut RandomState =
        lua_newuserdatauv(state, ::core::mem::size_of::<RandomState>() as u64, 0)
            as *mut RandomState;
    random_seed(state, ranstate);
    lua_settop(state, -3);
    lual_setfuncs(state, MATH_RANDOM_FUNCTIONS.as_ptr(), 1);
}}
static mut MATH_FUNCTIONS: [RegisteredFunction; 28] = {
    [
        {
            RegisteredFunction {
                name: b"abs\0" as *const u8 as *const i8,
                function: Some(math_abs as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"acos\0" as *const u8 as *const i8,
                function: Some(math_acos as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"asin\0" as *const u8 as *const i8,
                function: Some(math_asin as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"atan\0" as *const u8 as *const i8,
                function: Some(math_atan as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"ceil\0" as *const u8 as *const i8,
                function: Some(math_ceil as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"cos\0" as *const u8 as *const i8,
                function: Some(math_cos as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"deg\0" as *const u8 as *const i8,
                function: Some(math_deg as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"exp\0" as *const u8 as *const i8,
                function: Some(math_exp as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"tointeger\0" as *const u8 as *const i8,
                function: Some(math_toint as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"floor\0" as *const u8 as *const i8,
                function: Some(math_floor as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"fmod\0" as *const u8 as *const i8,
                function: Some(math_fmod as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"ult\0" as *const u8 as *const i8,
                function: Some(math_ult as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"log\0" as *const u8 as *const i8,
                function: Some(math_log as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"max\0" as *const u8 as *const i8,
                function: Some(math_max as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"min\0" as *const u8 as *const i8,
                function: Some(math_min as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"modf\0" as *const u8 as *const i8,
                function: Some(math_modf as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"rad\0" as *const u8 as *const i8,
                function: Some(math_rad as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"sin\0" as *const u8 as *const i8,
                function: Some(math_sin as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"sqrt\0" as *const u8 as *const i8,
                function: Some(math_sqrt as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"tan\0" as *const u8 as *const i8,
                function: Some(math_tan as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"type\0" as *const u8 as *const i8,
                function: Some(math_type as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"random\0" as *const u8 as *const i8,
                function: None,
            }
        },
        {
            RegisteredFunction {
                name: b"randomseed\0" as *const u8 as *const i8,
                function: None,
            }
        },
        {
            RegisteredFunction {
                name: b"pi\0" as *const u8 as *const i8,
                function: None,
            }
        },
        {
            RegisteredFunction {
                name: b"huge\0" as *const u8 as *const i8,
                function: None,
            }
        },
        {
            RegisteredFunction {
                name: b"maxinteger\0" as *const u8 as *const i8,
                function: None,
            }
        },
        {
            RegisteredFunction {
                name: b"mininteger\0" as *const u8 as *const i8,
                function: None,
            }
        },
        {
            RegisteredFunction {
                name: std::ptr::null(),
                function: None,
            }
        },
    ]
};
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaopen_math(state: *mut State) -> i32 { unsafe {
    lua_createtable(state, 0, 0);
    lual_setfuncs(state, MATH_FUNCTIONS.as_ptr(), 0);
    (*state).push_number(PI);
    lua_setfield(state, -2, b"pi\0" as *const u8 as *const i8);
    (*state).push_number(::core::f64::INFINITY);
    lua_setfield(state, -2, b"huge\0" as *const u8 as *const i8);
    (*state).push_integer(::core::i64::MAX);
    lua_setfield(state, -2, b"maxinteger\0" as *const u8 as *const i8);
    (*state).push_integer(::core::i64::MIN);
    lua_setfield(state, -2, b"mininteger\0" as *const u8 as *const i8);
    set_random_function(state);
    1
}}
