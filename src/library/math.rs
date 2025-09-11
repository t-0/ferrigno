use std::ptr::*;
use crate::utility::c::*;
use crate::randomstate::*;
use crate::utility::*;
use crate::tag::*;
use crate::user::*;
use crate::registeredfunction::*;
use crate::interpreter::*;
pub const PI: f64 = 3.141592653589793238462643383279502884f64;
pub unsafe extern "C" fn push_numericcc(interpreter: *mut Interpreter, d: f64) {
    unsafe {
        let mut n: i64 = 0;
        if d >= (-(MAXIMUM_SIZE as i64) - 1 as i64) as f64
            && d < -((-(MAXIMUM_SIZE as i64) - 1 as i64) as f64)
            && {
                n = d as i64;
                1 != 0
            }
        {
            (*interpreter).push_integer(n);
        } else {
            (*interpreter).push_number(d);
        };
    }
}
pub unsafe extern "C" fn rotate_left(x: usize, n: i32) -> usize {
    (x << n) | ((x & 0xffffffffffffffff as usize) >> (64 - n))
}
pub unsafe extern "C" fn next_random(randomstate: *mut usize) -> usize {
    unsafe {
        let state0: usize = *randomstate.offset(0 as isize);
        let state1: usize = *randomstate.offset(1 as isize);
        let state2: usize = *randomstate.offset(2 as isize) ^ state0;
        let state3: usize = *randomstate.offset(3 as isize) ^ state1;
        let res: usize =
            (rotate_left(state1.wrapping_mul(5 as usize), 7)).wrapping_mul(9 as usize);
        *randomstate.offset(0 as isize) = state0 ^ state3;
        *randomstate.offset(1 as isize) = state1 ^ state2;
        *randomstate.offset(2 as isize) = state2 ^ state1 << 17 as i32;
        *randomstate.offset(3 as isize) = rotate_left(state3, 45 as i32);
        res
    }
}
pub unsafe extern "C" fn i2d(x: usize) -> f64 {
    let sx: i64 = ((x & 0xffffffffffffffff as usize) >> (64 - 53)) as i64;
    let mut res: f64 = sx as f64 * (0.5f64 / ((1 as usize) << (53 - 1)) as f64);
    if sx < 0 {
        res += 1.0f64;
    }
    res
}
pub unsafe extern "C" fn project(mut ran: usize, n: usize, ransate: *mut RandomState) -> usize {
    unsafe {
        if n & n.wrapping_add(1 as usize) == 0 {
            return ran & n;
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
                if !(ran > n) {
                    break;
                }
                ran = (next_random(((*ransate).data).as_mut_ptr()) & 0xffffffffffffffff as usize) as usize;
            }
            return ran;
        };
    }
}
pub unsafe extern "C" fn set_seed(interpreter: *mut Interpreter, randomstate: *mut usize, n1: usize, n2: usize) {
    unsafe {
        *randomstate.offset(0 as isize) = n1 as usize;
        *randomstate.offset(1 as isize) = 0xFF as usize;
        *randomstate.offset(2 as isize) = n2 as usize;
        *randomstate.offset(3 as isize) = 0;
        for _ in 0..16 {
            next_random(randomstate);
        }
        (*interpreter).push_integer(n1 as i64);
        (*interpreter).push_integer(n2 as i64);
    }
}
pub unsafe extern "C" fn random_seed(interpreter: *mut Interpreter, randomstate: *mut RandomState) {
    unsafe {
        let seed1: usize = time(null_mut()) as usize;
        let seed2: usize = interpreter as usize;
        set_seed(interpreter, ((*randomstate).data).as_mut_ptr(), seed1, seed2);
    }
}
unsafe extern "C" fn math_abs(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        if lua_isinteger(interpreter, 1) {
            let mut n: i64 = lua_tointegerx(interpreter, 1, null_mut());
            if n < 0 {
                n = (0usize).wrapping_sub(n as usize) as i64;
            }
            (*interpreter).push_integer(n);
        } else {
            (*interpreter).push_number(lual_checknumber(interpreter, 1).abs());
        }
        1
    }
}
unsafe extern "C" fn math_sin(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        (*interpreter).push_number(lual_checknumber(interpreter, 1).sin());
        1
    }
}
unsafe extern "C" fn math_cos(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        (*interpreter).push_number(lual_checknumber(interpreter, 1).cos());
        1
    }
}
unsafe extern "C" fn math_tan(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        (*interpreter).push_number(lual_checknumber(interpreter, 1).tan());
        1
    }
}
unsafe extern "C" fn math_asin(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        (*interpreter).push_number(lual_checknumber(interpreter, 1).asin());
        1
    }
}
unsafe extern "C" fn math_acos(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        (*interpreter).push_number(lual_checknumber(interpreter, 1).acos());
        1
    }
}
unsafe extern "C" fn math_atan(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let y: f64 = lual_checknumber(interpreter, 1);
        let x: f64 = lual_optnumber(interpreter, 2, 1.0);
        (*interpreter).push_number(y.atan2(x));
        1
    }
}
unsafe extern "C" fn math_toint(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let mut is_number= false;
        let n: i64 = lua_tointegerx(interpreter, 1, &mut is_number);
        if is_number {
            (*interpreter).push_integer(n);
        } else {
            lual_checkany(interpreter, 1);
            (*interpreter).push_nil();
        }
        1
    }
}
unsafe extern "C" fn math_floor(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        if lua_isinteger(interpreter, 1) {
            lua_settop(interpreter, 1);
        } else {
            let d: f64 = lual_checknumber(interpreter, 1).floor();
            push_numericcc(interpreter, d);
        }
        1
    }
}
unsafe extern "C" fn math_ceil(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        if lua_isinteger(interpreter, 1) {
            lua_settop(interpreter, 1);
        } else {
            let d: f64 = lual_checknumber(interpreter, 1).ceil();
            push_numericcc(interpreter, d);
        }
        1
    }
}
unsafe extern "C" fn math_fmod(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        if lua_isinteger(interpreter, 1) && lua_isinteger(interpreter, 2) {
            let d: i64 = lua_tointegerx(interpreter, 2, null_mut());
            if (d as usize).wrapping_add(1 as usize) <= 1 as usize {
                (((d != 0) as i32 != 0) as i64 != 0
                    || lual_argerror(interpreter, 2, b"zero\0" as *const u8 as *const i8) != 0)
                    as i32;
                (*interpreter).push_integer(0);
            } else {
                (*interpreter).push_integer(lua_tointegerx(interpreter, 1, null_mut()) % d);
            }
        } else {
            (*interpreter).push_number(fmod(lual_checknumber(interpreter, 1), lual_checknumber(interpreter, 2)));
        }
        1
    }
}
unsafe extern "C" fn math_modf(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        if lua_isinteger(interpreter, 1) {
            lua_settop(interpreter, 1);
            (*interpreter).push_number(0.0);
        } else {
            let n: f64 = lual_checknumber(interpreter, 1);
            let ip: f64 = if n < 0.0 { n.ceil() } else { n.floor() };
            push_numericcc(interpreter, ip);
            (*interpreter).push_number(if n == ip { 0.0 } else { n - ip });
        }
        2
    }
}
unsafe extern "C" fn math_sqrt(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        (*interpreter).push_number(lual_checknumber(interpreter, 1).sqrt());
        1
    }
}
unsafe extern "C" fn math_ult(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let a: i64 = lual_checkinteger(interpreter, 1);
        let b: i64 = lual_checkinteger(interpreter, 2);
        (*interpreter).push_boolean((a as usize) < (b as usize));
        1
    }
}
unsafe extern "C" fn math_log(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let x: f64 = lual_checknumber(interpreter, 1);
        let res: f64;

        match lua_type(interpreter, 2) {
            None | Some(TagType::Nil) => {
                res = x.ln();
            },
            _ => {
                let base: f64 = lual_checknumber(interpreter, 2);
                if base == 2.0f64 {
                    res = x.log2();
                } else if base == 10.0f64 {
                    res = x.log10();
                } else {
                    res = x.ln() / base.ln();
                }
            }
        }
        (*interpreter).push_number(res);
        1
    }
}
unsafe extern "C" fn math_exp(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        (*interpreter).push_number(lual_checknumber(interpreter, 1).exp());
        1
    }
}
unsafe extern "C" fn math_deg(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        (*interpreter).push_number(lual_checknumber(interpreter, 1) * (180.0f64 / PI));
        1
    }
}
unsafe extern "C" fn math_rad(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        (*interpreter).push_number(lual_checknumber(interpreter, 1) * (PI / 180.0f64));
        1
    }
}
unsafe extern "C" fn math_min(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let n: i32 = (*interpreter).get_top();
        let mut imin: i32 = 1;
        (((n >= 1) as i32 != 0) as i64 != 0
            || lual_argerror(interpreter, 1, b"value expected\0" as *const u8 as *const i8) != 0)
            as i32;
        for i in 2..(1 + n) {
            if lua_compare(interpreter, i, imin, 1) != 0 {
                imin = i;
            }
        }
        lua_pushvalue(interpreter, imin);
        1
    }
}
unsafe extern "C" fn math_max(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let n: i32 = (*interpreter).get_top();
        let mut imax: i32 = 1;
        (((n >= 1) as i32 != 0) as i64 != 0
            || lual_argerror(interpreter, 1, b"value expected\0" as *const u8 as *const i8) != 0)
            as i32;
        for i in 2..(1 + n) {
            if lua_compare(interpreter, imax, i, 1) != 0 {
                imax = i;
            }
        }
        lua_pushvalue(interpreter, imax);
        1
    }
}
unsafe extern "C" fn math_type(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        if lua_type(interpreter, 1) == Some(TagType::Numeric) {
            lua_pushstring(
                interpreter,
                if lua_isinteger(interpreter, 1) {
                    b"integer\0" as *const u8 as *const i8
                } else {
                    b"float\0" as *const u8 as *const i8
                },
            );
        } else {
            lual_checkany(interpreter, 1);
            (*interpreter).push_nil();
        }
        1
    }
}
unsafe extern "C" fn math_random(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let low: i64;
        let up: i64;
        let p: usize;
        let ransate: *mut RandomState =
            lua_touserdata(interpreter, -(1000000 as i32) - 1000 as i32 - 1) as *mut RandomState;
        let rv: usize = next_random(((*ransate).data).as_mut_ptr());
        match (*interpreter).get_top() {
            0 => {
                (*interpreter).push_number(i2d(rv));
                return 1;
            }
            1 => {
                low = 1;
                up = lual_checkinteger(interpreter, 1);
                if up == 0 {
                    (*interpreter).push_integer((rv & 0xffffffffffffffff as usize) as i64);
                    return 1;
                }
            }
            2 => {
                low = lual_checkinteger(interpreter, 1);
                up = lual_checkinteger(interpreter, 2);
            }
            _ => {
                return lual_error(
                    interpreter,
                    b"wrong number of arguments\0".as_ptr(),
                );
            }
        }
        (((low <= up) as i32 != 0) as i64 != 0
            || lual_argerror(interpreter, 1, b"interval is empty\0" as *const u8 as *const i8) != 0)
            as i32;
        p = project(
            (rv & 0xffffffffffffffff as usize) as usize,
            (up as usize).wrapping_sub(low as usize),
            ransate,
        );
        (*interpreter).push_integer(p.wrapping_add(low as usize) as i64);
        return 1;
    }
}
unsafe extern "C" fn math_randomseed(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let randomstate: *mut RandomState =
            lua_touserdata(interpreter, -(1000000 as i32) - 1000 as i32 - 1) as *mut RandomState;
        if lua_type(interpreter, 1) == None {
            random_seed(interpreter, randomstate);
        } else {
            let n1: i64 = lual_checkinteger(interpreter, 1);
            let n2: i64 = lual_optinteger(interpreter, 2, 0);
            set_seed(interpreter, ((*randomstate).data).as_mut_ptr(), n1 as usize, n2 as usize);
        }
        return 2;
    }
}
const MATH_RANDOM_FUNCTIONS: [RegisteredFunction; 3] = {
    [
        {
            RegisteredFunction {
                name: b"random\0" as *const u8 as *const i8,
                function: Some(math_random as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"randomseed\0" as *const u8 as *const i8,
                function: Some(math_randomseed as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: null(),
                function: None,
            }
        },
    ]
};
unsafe extern "C" fn set_random_function(interpreter: *mut Interpreter) {
    unsafe {
        let ranstate: *mut RandomState =
            User::lua_newuserdatauv(interpreter, size_of::<RandomState>(), 0)
                as *mut RandomState;
        random_seed(interpreter, ranstate);
        lua_settop(interpreter, -3);
        lual_setfuncs(interpreter, MATH_RANDOM_FUNCTIONS.as_ptr(), 1);
    }
}
const MATH_FUNCTIONS: [RegisteredFunction; 28] = {
    [
        {
            RegisteredFunction {
                name: b"abs\0" as *const u8 as *const i8,
                function: Some(math_abs as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"acos\0" as *const u8 as *const i8,
                function: Some(math_acos as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"asin\0" as *const u8 as *const i8,
                function: Some(math_asin as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"atan\0" as *const u8 as *const i8,
                function: Some(math_atan as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"ceil\0" as *const u8 as *const i8,
                function: Some(math_ceil as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"cos\0" as *const u8 as *const i8,
                function: Some(math_cos as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"deg\0" as *const u8 as *const i8,
                function: Some(math_deg as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"exp\0" as *const u8 as *const i8,
                function: Some(math_exp as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"tointeger\0" as *const u8 as *const i8,
                function: Some(math_toint as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"floor\0" as *const u8 as *const i8,
                function: Some(math_floor as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"fmod\0" as *const u8 as *const i8,
                function: Some(math_fmod as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"ult\0" as *const u8 as *const i8,
                function: Some(math_ult as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"log\0" as *const u8 as *const i8,
                function: Some(math_log as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"max\0" as *const u8 as *const i8,
                function: Some(math_max as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"min\0" as *const u8 as *const i8,
                function: Some(math_min as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"modf\0" as *const u8 as *const i8,
                function: Some(math_modf as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"rad\0" as *const u8 as *const i8,
                function: Some(math_rad as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"sin\0" as *const u8 as *const i8,
                function: Some(math_sin as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"sqrt\0" as *const u8 as *const i8,
                function: Some(math_sqrt as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"tan\0" as *const u8 as *const i8,
                function: Some(math_tan as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"type\0" as *const u8 as *const i8,
                function: Some(math_type as unsafe extern "C" fn(*mut Interpreter) -> i32),
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
                name: null(),
                function: None,
            }
        },
    ]
};
pub unsafe extern "C" fn luaopen_math(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        (*interpreter).lua_createtable();
        lual_setfuncs(interpreter, MATH_FUNCTIONS.as_ptr(), 0);
        (*interpreter).push_number(PI);
        lua_setfield(interpreter, -2, b"pi\0" as *const u8 as *const i8);
        (*interpreter).push_number(::core::f64::INFINITY);
        lua_setfield(interpreter, -2, b"huge\0" as *const u8 as *const i8);
        (*interpreter).push_integer(::core::i64::MAX);
        lua_setfield(interpreter, -2, b"maxinteger\0" as *const u8 as *const i8);
        (*interpreter).push_integer(::core::i64::MIN);
        lua_setfield(interpreter, -2, b"mininteger\0" as *const u8 as *const i8);
        set_random_function(interpreter);
        1
    }
}
