use crate::interpreter::*;
use crate::randomstate::*;
use crate::registeredfunction::*;
use crate::tag::*;
use crate::user::*;
use crate::utility::*;
use libc::time;
use std::ptr::*;
pub const PI: f64 = 3.141592653589793238462643383279502884f64;
pub unsafe fn push_numericcc(interpreter: *mut Interpreter, d: f64) {
    unsafe {
        let mut n: i64 = 0;
        if d >= (-(MAXIMUM_SIZE as i64) - 1 as i64) as f64 && d < -((-(MAXIMUM_SIZE as i64) - 1 as i64) as f64) && {
            n = d as i64;
            1 != 0
        } {
            (*interpreter).push_integer(n);
        } else {
            (*interpreter).push_number(d);
        };
    }
}
pub unsafe fn rotate_left(x: usize, n: i32) -> usize {
    (x << n) | ((x & 0xffffffffffffffff as usize) >> (64 - n))
}
pub unsafe fn next_random(randomstate: *mut usize) -> usize {
    unsafe {
        let state0: usize = *randomstate.offset(0 as isize);
        let state1: usize = *randomstate.offset(1 as isize);
        let state2: usize = *randomstate.offset(2 as isize) ^ state0;
        let state3: usize = *randomstate.offset(3 as isize) ^ state1;
        let res: usize = (rotate_left(state1.wrapping_mul(5 as usize), 7)).wrapping_mul(9 as usize);
        *randomstate.offset(0 as isize) = state0 ^ state3;
        *randomstate.offset(1 as isize) = state1 ^ state2;
        *randomstate.offset(2 as isize) = state2 ^ state1 << 17 as i32;
        *randomstate.offset(3 as isize) = rotate_left(state3, 45 as i32);
        res
    }
}
pub unsafe fn i2d(x: usize) -> f64 {
    let sx: i64 = ((x & 0xffffffffffffffff as usize) >> (64 - 53)) as i64;
    let mut res: f64 = sx as f64 * (0.5f64 / ((1 as usize) << (53 - 1)) as f64);
    if sx < 0 {
        res += 1.0;
    }
    res
}
pub unsafe fn project(mut ran: usize, n: usize, ransate: *mut RandomState) -> usize {
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
                ran = (next_random(((*ransate).randomstate_data).as_mut_ptr()) & 0xffffffffffffffff as usize) as usize;
            }
            return ran;
        };
    }
}
pub unsafe fn set_seed(interpreter: *mut Interpreter, randomstate: *mut usize, n1: usize, n2: usize) {
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
pub unsafe fn random_seed(interpreter: *mut Interpreter, randomstate: *mut RandomState) {
    unsafe {
        let seed1: usize = time(null_mut()) as usize;
        let seed2: usize = interpreter as usize;
        set_seed(interpreter, ((*randomstate).randomstate_data).as_mut_ptr(), seed1, seed2);
    }
}
unsafe fn math_abs(interpreter: *mut Interpreter) -> i32 {
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
unsafe fn math_sin(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        (*interpreter).push_number(lual_checknumber(interpreter, 1).sin());
        1
    }
}
unsafe fn math_cos(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        (*interpreter).push_number(lual_checknumber(interpreter, 1).cos());
        1
    }
}
unsafe fn math_tan(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        (*interpreter).push_number(lual_checknumber(interpreter, 1).tan());
        1
    }
}
unsafe fn math_asin(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        (*interpreter).push_number(lual_checknumber(interpreter, 1).asin());
        1
    }
}
unsafe fn math_acos(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        (*interpreter).push_number(lual_checknumber(interpreter, 1).acos());
        1
    }
}
unsafe fn math_atan(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let y: f64 = lual_checknumber(interpreter, 1);
        let x: f64 = lual_optnumber(interpreter, 2, 1.0);
        (*interpreter).push_number(y.atan2(x));
        1
    }
}
unsafe fn math_toint(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let mut is_number = false;
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
unsafe fn math_floor(interpreter: *mut Interpreter) -> i32 {
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
unsafe fn math_ceil(interpreter: *mut Interpreter) -> i32 {
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
unsafe fn math_fmod(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        if lua_isinteger(interpreter, 1) && lua_isinteger(interpreter, 2) {
            let d: i64 = lua_tointegerx(interpreter, 2, null_mut());
            if (d as usize).wrapping_add(1 as usize) <= 1 as usize {
                (((d != 0) as i32 != 0) as i64 != 0 || lual_argerror(interpreter, 2, c"zero".as_ptr()) != 0) as i32;
                (*interpreter).push_integer(0);
            } else {
                (*interpreter).push_integer(lua_tointegerx(interpreter, 1, null_mut()) % d);
            }
        } else {
            (*interpreter).push_number(lual_checknumber(interpreter, 1)  % lual_checknumber(interpreter, 2));
        }
        1
    }
}
unsafe fn math_modf(interpreter: *mut Interpreter) -> i32 {
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
unsafe fn math_sqrt(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        (*interpreter).push_number(lual_checknumber(interpreter, 1).sqrt());
        1
    }
}
unsafe fn math_ult(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let a: i64 = lual_checkinteger(interpreter, 1);
        let b: i64 = lual_checkinteger(interpreter, 2);
        (*interpreter).push_boolean((a as usize) < (b as usize));
        1
    }
}
unsafe fn math_log(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let x: f64 = lual_checknumber(interpreter, 1);
        let res: f64;

        match lua_type(interpreter, 2) {
            | None | Some(TagType::Nil) => {
                res = x.ln();
            },
            | _ => {
                let base: f64 = lual_checknumber(interpreter, 2);
                if base == 2.0f64 {
                    res = x.log2();
                } else if base == 10.0f64 {
                    res = x.log10();
                } else {
                    res = x.ln() / base.ln();
                }
            },
        }
        (*interpreter).push_number(res);
        1
    }
}
unsafe fn math_exp(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        (*interpreter).push_number(lual_checknumber(interpreter, 1).exp());
        1
    }
}
unsafe fn math_deg(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        (*interpreter).push_number(lual_checknumber(interpreter, 1) * (180.0f64 / PI));
        1
    }
}
unsafe fn math_rad(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        (*interpreter).push_number(lual_checknumber(interpreter, 1) * (PI / 180.0f64));
        1
    }
}
unsafe fn math_min(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let n: i32 = (*interpreter).get_top();
        let mut imin: i32 = 1;
        (((n >= 1) as i32 != 0) as i64 != 0 || lual_argerror(interpreter, 1, c"value expected".as_ptr()) != 0) as i32;
        for i in 2..(1 + n) {
            if lua_compare(interpreter, i, imin, 1) {
                imin = i;
            }
        }
        lua_pushvalue(interpreter, imin);
        1
    }
}
unsafe fn math_max(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let n: i32 = (*interpreter).get_top();
        let mut imax: i32 = 1;
        (((n >= 1) as i32 != 0) as i64 != 0 || lual_argerror(interpreter, 1, c"value expected".as_ptr()) != 0) as i32;
        for i in 2..(1 + n) {
            if lua_compare(interpreter, imax, i, 1) {
                imax = i;
            }
        }
        lua_pushvalue(interpreter, imax);
        1
    }
}
unsafe fn math_type(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        if lua_type(interpreter, 1) == Some(TagType::Numeric) {
            lua_pushstring(
                interpreter,
                if lua_isinteger(interpreter, 1) {
                    c"integer".as_ptr()
                } else {
                    c"float".as_ptr()
                },
            );
        } else {
            lual_checkany(interpreter, 1);
            (*interpreter).push_nil();
        }
        1
    }
}
unsafe fn math_random(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let low: i64;
        let high: i64;
        let p: usize;
        let ransate: *mut RandomState = (*interpreter).to_pointer(-(1000000 as i32) - 1000 as i32 - 1) as *mut RandomState;
        let rv: usize = next_random(((*ransate).randomstate_data).as_mut_ptr());
        match (*interpreter).get_top() {
            | 0 => {
                (*interpreter).push_number(i2d(rv));
                return 1;
            },
            | 1 => {
                low = 1;
                high = lual_checkinteger(interpreter, 1);
                if high == 0 {
                    (*interpreter).push_integer((rv & 0xffffffffffffffff as usize) as i64);
                    return 1;
                }
            },
            | 2 => {
                low = lual_checkinteger(interpreter, 1);
                high = lual_checkinteger(interpreter, 2);
            },
            | _ => {
                return lual_error(interpreter, c"wrong number of arguments".as_ptr());
            },
        }
        (((low <= high) as i32 != 0) as i64 != 0 || lual_argerror(interpreter, 1, c"interval is empty".as_ptr()) != 0) as i32;
        p = project(
            (rv & 0xffffffffffffffff as usize) as usize,
            (high as usize).wrapping_sub(low as usize),
            ransate,
        );
        (*interpreter).push_integer(p.wrapping_add(low as usize) as i64);
        return 1;
    }
}
unsafe fn math_randomseed(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let randomstate: *mut RandomState = (*interpreter).to_pointer(-(1000000 as i32) - 1000 as i32 - 1) as *mut RandomState;
        if lua_type(interpreter, 1) == None {
            random_seed(interpreter, randomstate);
        } else {
            let n1: i64 = lual_checkinteger(interpreter, 1);
            let n2: i64 = lual_optinteger(interpreter, 2, 0);
            set_seed(
                interpreter,
                ((*randomstate).randomstate_data).as_mut_ptr(),
                n1 as usize,
                n2 as usize,
            );
        }
        return 2;
    }
}
const MATH_RANDOM_FUNCTIONS: [RegisteredFunction; 2] = {
    [
        {
            RegisteredFunction {
                registeredfunction_name: c"random".as_ptr(),
                registeredfunction_function: Some(math_random as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"randomseed".as_ptr(),
                registeredfunction_function: Some(math_randomseed as unsafe fn(*mut Interpreter) -> i32),
            }
        },
    ]
};
unsafe fn set_random_function(interpreter: *mut Interpreter) {
    unsafe {
        let ranstate: *mut RandomState = User::lua_newuserdatauv(interpreter, size_of::<RandomState>(), 0) as *mut RandomState;
        random_seed(interpreter, ranstate);
        lua_settop(interpreter, -3);
        lual_setfuncs(interpreter, MATH_RANDOM_FUNCTIONS.as_ptr(), MATH_RANDOM_FUNCTIONS.len(), 1);
    }
}
const MATH_FUNCTIONS: [RegisteredFunction; 21] = {
    [
        {
            RegisteredFunction {
                registeredfunction_name: c"abs".as_ptr(),
                registeredfunction_function: Some(math_abs as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"acos".as_ptr(),
                registeredfunction_function: Some(math_acos as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"asin".as_ptr(),
                registeredfunction_function: Some(math_asin as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"atan".as_ptr(),
                registeredfunction_function: Some(math_atan as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"ceil".as_ptr(),
                registeredfunction_function: Some(math_ceil as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"cos".as_ptr(),
                registeredfunction_function: Some(math_cos as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"deg".as_ptr(),
                registeredfunction_function: Some(math_deg as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"exp".as_ptr(),
                registeredfunction_function: Some(math_exp as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"tointeger".as_ptr(),
                registeredfunction_function: Some(math_toint as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"floor".as_ptr(),
                registeredfunction_function: Some(math_floor as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"fmod".as_ptr(),
                registeredfunction_function: Some(math_fmod as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"ult".as_ptr(),
                registeredfunction_function: Some(math_ult as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"log".as_ptr(),
                registeredfunction_function: Some(math_log as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"max".as_ptr(),
                registeredfunction_function: Some(math_max as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"min".as_ptr(),
                registeredfunction_function: Some(math_min as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"modf".as_ptr(),
                registeredfunction_function: Some(math_modf as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"rad".as_ptr(),
                registeredfunction_function: Some(math_rad as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"sin".as_ptr(),
                registeredfunction_function: Some(math_sin as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"sqrt".as_ptr(),
                registeredfunction_function: Some(math_sqrt as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"tan".as_ptr(),
                registeredfunction_function: Some(math_tan as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"type".as_ptr(),
                registeredfunction_function: Some(math_type as unsafe fn(*mut Interpreter) -> i32),
            }
        },
    ]
};
pub unsafe fn luaopen_math(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        (*interpreter).lua_createtable();
        lual_setfuncs(interpreter, MATH_FUNCTIONS.as_ptr(), MATH_FUNCTIONS.len(), 0);
        (*interpreter).push_number(PI);
        lua_setfield(interpreter, -2, c"pi".as_ptr());
        (*interpreter).push_number(::core::f64::INFINITY);
        lua_setfield(interpreter, -2, c"huge".as_ptr());
        (*interpreter).push_integer(::core::i64::MAX);
        lua_setfield(interpreter, -2, c"maxinteger".as_ptr());
        (*interpreter).push_integer(::core::i64::MIN);
        lua_setfield(interpreter, -2, c"mininteger".as_ptr());
        set_random_function(interpreter);
        1
    }
}
