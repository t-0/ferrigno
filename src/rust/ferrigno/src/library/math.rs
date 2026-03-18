use crate::randomstate::*;
use crate::registeredfunction::*;
use crate::state::*;
use crate::tagtype::*;
use crate::user::*;
use crate::utility::*;
use std::ptr::*;
pub const PI: f64 = 3.141592653589793238462643383279502884f64;
pub unsafe fn push_numericcc(state: *mut State, d: f64) {
    unsafe {
        if d >= (-(MAXIMUM_SIZE as i64) - 1_i64) as f64 && d < -((-(MAXIMUM_SIZE as i64) - 1_i64) as f64) {
            (*state).push_integer(d as i64);
        } else {
            (*state).push_number(d);
        };
    }
}
pub fn rotate_left(x: usize, n: i32) -> usize {
    (x << n) | (x >> (64 - n))
}
pub unsafe fn next_random(randomstate: *mut usize) -> usize {
    unsafe {
        let state0: usize = *randomstate.add(0);
        let state1: usize = *randomstate.add(1);
        let state2: usize = *randomstate.add(2) ^ state0;
        let state3: usize = *randomstate.add(3) ^ state1;
        let res: usize = (rotate_left(state1.wrapping_mul(5_usize), 7)).wrapping_mul(9_usize);
        *randomstate.add(0) = state0 ^ state3;
        *randomstate.add(1) = state1 ^ state2;
        *randomstate.add(2) = state2 ^ state1 << 17_i32;
        *randomstate.add(3) = rotate_left(state3, 45_i32);
        res
    }
}
pub fn i2d(x: usize) -> f64 {
    let sx: i64 = (x >> (64 - 53)) as i64;
    let mut res: f64 = sx as f64 * (0.5f64 / (1_usize << (53 - 1)) as f64);
    if sx < 0 {
        res += 1.0;
    }
    res
}
pub unsafe fn project(mut ran: usize, n: usize, ransate: *mut RandomState) -> usize {
    unsafe {
        if n & n.wrapping_add(1_usize) == 0 {
            ran & n
        } else {
            let mut lim: usize = n;
            lim |= lim >> 1;
            lim |= lim >> 2;
            lim |= lim >> 4;
            lim |= lim >> 8;
            lim |= lim >> 16;
            lim |= lim >> 32;
            loop {
                ran &= lim;
                if ran <= n {
                    break;
                }
                ran = next_random(((*ransate).randomstate_data).as_mut_ptr());
            }
            ran
        }
    }
}
pub unsafe fn set_seed(state: *mut State, randomstate: *mut usize, n1: usize, n2: usize) {
    unsafe {
        *randomstate.add(0) = n1;
        *randomstate.add(1) = 0xFF_usize;
        *randomstate.add(2) = n2;
        *randomstate.add(3) = 0;
        for _ in 0..16 {
            next_random(randomstate);
        }
        (*state).push_integer(n1 as i64);
        (*state).push_integer(n2 as i64);
    }
}
pub unsafe fn random_seed(state: *mut State, randomstate: *mut RandomState) {
    unsafe {
        let seed1: usize = os_time_now() as usize;
        let seed2: usize = state as usize;
        set_seed(state, ((*randomstate).randomstate_data).as_mut_ptr(), seed1, seed2);
    }
}
unsafe fn math_abs(state: *mut State) -> i32 {
    unsafe {
        if lua_isinteger(state, 1) {
            let mut n: i64 = lua_tointegerx(state, 1, null_mut());
            if n < 0 {
                n = (0usize).wrapping_sub(n as usize) as i64;
            }
            (*state).push_integer(n);
        } else {
            (*state).push_number(lual_checknumber(state, 1).abs());
        }
        1
    }
}
unsafe fn math_sin(state: *mut State) -> i32 {
    unsafe {
        (*state).push_number(lual_checknumber(state, 1).sin());
        1
    }
}
unsafe fn math_cos(state: *mut State) -> i32 {
    unsafe {
        (*state).push_number(lual_checknumber(state, 1).cos());
        1
    }
}
unsafe fn math_tan(state: *mut State) -> i32 {
    unsafe {
        (*state).push_number(lual_checknumber(state, 1).tan());
        1
    }
}
unsafe fn math_asin(state: *mut State) -> i32 {
    unsafe {
        (*state).push_number(lual_checknumber(state, 1).asin());
        1
    }
}
unsafe fn math_acos(state: *mut State) -> i32 {
    unsafe {
        (*state).push_number(lual_checknumber(state, 1).acos());
        1
    }
}
unsafe fn math_atan(state: *mut State) -> i32 {
    unsafe {
        let y: f64 = lual_checknumber(state, 1);
        let x: f64 = lual_optnumber(state, 2, 1.0);
        (*state).push_number(y.atan2(x));
        1
    }
}
unsafe fn math_toint(state: *mut State) -> i32 {
    unsafe {
        let mut is_number = false;
        let n: i64 = lua_tointegerx(state, 1, &mut is_number);
        if is_number {
            (*state).push_integer(n);
        } else {
            lual_checkany(state, 1);
            (*state).push_nil();
        }
        1
    }
}
unsafe fn math_floor(state: *mut State) -> i32 {
    unsafe {
        if lua_isinteger(state, 1) {
            lua_settop(state, 1);
        } else {
            let d: f64 = lual_checknumber(state, 1).floor();
            push_numericcc(state, d);
        }
        1
    }
}
unsafe fn math_ceil(state: *mut State) -> i32 {
    unsafe {
        if lua_isinteger(state, 1) {
            lua_settop(state, 1);
        } else {
            let d: f64 = lual_checknumber(state, 1).ceil();
            push_numericcc(state, d);
        }
        1
    }
}
unsafe fn math_fmod(state: *mut State) -> i32 {
    unsafe {
        if lua_isinteger(state, 1) && lua_isinteger(state, 2) {
            let d: i64 = lua_tointegerx(state, 2, null_mut());
            if (d as usize).wrapping_add(1_usize) <= 1_usize {
                if d == 0 {
                    lual_argerror(state, 2, c"zero".as_ptr());
                    0;
                }
                (*state).push_integer(0);
            } else {
                (*state).push_integer(lua_tointegerx(state, 1, null_mut()) % d);
            }
        } else {
            (*state).push_number(lual_checknumber(state, 1) % lual_checknumber(state, 2));
        }
        1
    }
}
unsafe fn math_modf(state: *mut State) -> i32 {
    unsafe {
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
    }
}
unsafe fn math_sqrt(state: *mut State) -> i32 {
    unsafe {
        (*state).push_number(lual_checknumber(state, 1).sqrt());
        1
    }
}
unsafe fn math_ult(state: *mut State) -> i32 {
    unsafe {
        let a: i64 = lual_checkinteger(state, 1);
        let b: i64 = lual_checkinteger(state, 2);
        (*state).push_boolean((a as usize) < (b as usize));
        1
    }
}
unsafe fn math_log(state: *mut State) -> i32 {
    unsafe {
        let x: f64 = lual_checknumber(state, 1);
        let res: f64;

        match lua_type(state, 2) {
            | None | Some(TagType::Nil) => {
                res = x.ln();
            },
            | _ => {
                let base: f64 = lual_checknumber(state, 2);
                if base == 2.0f64 {
                    res = x.log2();
                } else if base == 10.0f64 {
                    res = x.log10();
                } else {
                    res = x.ln() / base.ln();
                }
            },
        }
        (*state).push_number(res);
        1
    }
}
unsafe fn math_exp(state: *mut State) -> i32 {
    unsafe {
        (*state).push_number(lual_checknumber(state, 1).exp());
        1
    }
}
unsafe fn math_deg(state: *mut State) -> i32 {
    unsafe {
        (*state).push_number(lual_checknumber(state, 1) * (180.0f64 / PI));
        1
    }
}
unsafe fn math_rad(state: *mut State) -> i32 {
    unsafe {
        (*state).push_number(lual_checknumber(state, 1) * (PI / 180.0f64));
        1
    }
}
unsafe fn math_min(state: *mut State) -> i32 {
    unsafe {
        let n: i32 = (*state).get_top();
        let mut imin: i32 = 1;
        if n < 1 {
            lual_argerror(state, 1, c"value expected".as_ptr());
            0;
        }
        for i in 2..(1 + n) {
            if lua_compare(state, i, imin, 1) {
                imin = i;
            }
        }
        lua_pushvalue(state, imin);
        1
    }
}
unsafe fn math_max(state: *mut State) -> i32 {
    unsafe {
        let n: i32 = (*state).get_top();
        let mut imax: i32 = 1;
        if n < 1 {
            lual_argerror(state, 1, c"value expected".as_ptr());
            0;
        }
        for i in 2..(1 + n) {
            if lua_compare(state, imax, i, 1) {
                imax = i;
            }
        }
        lua_pushvalue(state, imax);
        1
    }
}
unsafe fn math_type(state: *mut State) -> i32 {
    unsafe {
        if lua_type(state, 1) == Some(TagType::Numeric) {
            lua_pushstring(
                state,
                if lua_isinteger(state, 1) { c"integer".as_ptr() } else { c"float".as_ptr() },
            );
        } else {
            lual_checkany(state, 1);
            (*state).push_nil();
        }
        1
    }
}
unsafe fn math_random(state: *mut State) -> i32 {
    unsafe {
        let low: i64;
        let high: i64;

        let ransate: *mut RandomState = (*state).to_pointer(LUA_REGISTRYINDEX - 1) as *mut RandomState;
        let rv: usize = next_random(((*ransate).randomstate_data).as_mut_ptr());
        match (*state).get_top() {
            | 0 => {
                (*state).push_number(i2d(rv));
                return 1;
            },
            | 1 => {
                low = 1;
                high = lual_checkinteger(state, 1);
                if high == 0 {
                    (*state).push_integer(rv as i64);
                    return 1;
                }
            },
            | 2 => {
                low = lual_checkinteger(state, 1);
                high = lual_checkinteger(state, 2);
            },
            | _ => {
                return lual_error(state, c"wrong number of arguments".as_ptr(), &[]);
            },
        }
        if low > high {
            lual_argerror(state, 1, c"interval is empty".as_ptr());
            0;
        }
        let p: usize = project(rv, (high as usize).wrapping_sub(low as usize), ransate);
        (*state).push_integer(p.wrapping_add(low as usize) as i64);
        1
    }
}
unsafe fn math_randomseed(state: *mut State) -> i32 {
    unsafe {
        let randomstate: *mut RandomState = (*state).to_pointer(LUA_REGISTRYINDEX - 1) as *mut RandomState;
        if lua_type(state, 1).is_none() {
            random_seed(state, randomstate);
        } else {
            let n1: i64 = lual_checkinteger(state, 1);
            let n2: i64 = lual_optinteger(state, 2, 0);
            set_seed(state, ((*randomstate).randomstate_data).as_mut_ptr(), n1 as usize, n2 as usize);
        }
        2
    }
}
const MATH_RANDOM_FUNCTIONS: [RegisteredFunction; 2] = {
    [
        {
            RegisteredFunction {
                registeredfunction_name: c"random".as_ptr(),
                registeredfunction_function: Some(math_random as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"randomseed".as_ptr(),
                registeredfunction_function: Some(math_randomseed as unsafe fn(*mut State) -> i32),
            }
        },
    ]
};
unsafe fn set_random_function(state: *mut State) {
    unsafe {
        let ranstate: *mut RandomState = User::lua_newuserdatauv(state, size_of::<RandomState>(), 0) as *mut RandomState;
        random_seed(state, ranstate);
        lua_settop(state, -3);
        lual_setfuncs(state, MATH_RANDOM_FUNCTIONS.as_ptr(), MATH_RANDOM_FUNCTIONS.len(), 1);
    }
}
unsafe fn math_frexp(state: *mut State) -> i32 {
    unsafe {
        let x = lual_checknumber(state, 1);
        let (m, e) = frexp_(x);
        (*state).push_number(m);
        (*state).push_integer(e as i64);
        2
    }
}
unsafe fn math_ldexp(state: *mut State) -> i32 {
    unsafe {
        let x = lual_checknumber(state, 1);
        let e = lual_checkinteger(state, 2) as i32;
        (*state).push_number(ldexp_(x, e));
        1
    }
}
const MATH_FUNCTIONS: [RegisteredFunction; 23] = {
    [
        {
            RegisteredFunction {
                registeredfunction_name: c"abs".as_ptr(),
                registeredfunction_function: Some(math_abs as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"acos".as_ptr(),
                registeredfunction_function: Some(math_acos as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"asin".as_ptr(),
                registeredfunction_function: Some(math_asin as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"atan".as_ptr(),
                registeredfunction_function: Some(math_atan as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"ceil".as_ptr(),
                registeredfunction_function: Some(math_ceil as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"cos".as_ptr(),
                registeredfunction_function: Some(math_cos as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"deg".as_ptr(),
                registeredfunction_function: Some(math_deg as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"exp".as_ptr(),
                registeredfunction_function: Some(math_exp as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"tointeger".as_ptr(),
                registeredfunction_function: Some(math_toint as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"floor".as_ptr(),
                registeredfunction_function: Some(math_floor as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"fmod".as_ptr(),
                registeredfunction_function: Some(math_fmod as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"frexp".as_ptr(),
                registeredfunction_function: Some(math_frexp as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"ldexp".as_ptr(),
                registeredfunction_function: Some(math_ldexp as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"ult".as_ptr(),
                registeredfunction_function: Some(math_ult as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"log".as_ptr(),
                registeredfunction_function: Some(math_log as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"max".as_ptr(),
                registeredfunction_function: Some(math_max as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"min".as_ptr(),
                registeredfunction_function: Some(math_min as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"modf".as_ptr(),
                registeredfunction_function: Some(math_modf as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"rad".as_ptr(),
                registeredfunction_function: Some(math_rad as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"sin".as_ptr(),
                registeredfunction_function: Some(math_sin as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"sqrt".as_ptr(),
                registeredfunction_function: Some(math_sqrt as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"tan".as_ptr(),
                registeredfunction_function: Some(math_tan as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"type".as_ptr(),
                registeredfunction_function: Some(math_type as unsafe fn(*mut State) -> i32),
            }
        },
    ]
};
pub unsafe fn luaopen_math(state: *mut State) -> i32 {
    unsafe {
        (*state).lua_createtable();
        lual_setfuncs(state, MATH_FUNCTIONS.as_ptr(), MATH_FUNCTIONS.len(), 0);
        (*state).push_number(PI);
        lua_setfield(state, -2, c"pi".as_ptr());
        (*state).push_number(::core::f64::INFINITY);
        lua_setfield(state, -2, c"huge".as_ptr());
        (*state).push_integer(::core::i64::MAX);
        lua_setfield(state, -2, c"maxinteger".as_ptr());
        (*state).push_integer(::core::i64::MIN);
        lua_setfield(state, -2, c"mininteger".as_ptr());
        set_random_function(state);
        1
    }
}
