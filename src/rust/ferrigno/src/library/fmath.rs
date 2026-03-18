#![allow(unpredictable_function_pointer_comparisons)]
use crate::library::math::*;
use crate::randomstate::*;
use crate::registeredfunction::*;
use crate::state::*;
use crate::tagtype::*;
use crate::user::*;
use crate::utility::*;
use std::ptr::*;

const TAU: f64 = 2.0 * PI;
const E: f64 = std::f64::consts::E;

// ── trig ────────────────────────────────────────────────────────────────────
unsafe fn fm_sin(state: *mut State) -> i32 { unsafe { (*state).push_number(lual_checknumber(state, 1).sin()); 1 } }
unsafe fn fm_cos(state: *mut State) -> i32 { unsafe { (*state).push_number(lual_checknumber(state, 1).cos()); 1 } }
unsafe fn fm_tan(state: *mut State) -> i32 { unsafe { (*state).push_number(lual_checknumber(state, 1).tan()); 1 } }
unsafe fn fm_asin(state: *mut State) -> i32 { unsafe { (*state).push_number(lual_checknumber(state, 1).asin()); 1 } }
unsafe fn fm_acos(state: *mut State) -> i32 { unsafe { (*state).push_number(lual_checknumber(state, 1).acos()); 1 } }
unsafe fn fm_atan(state: *mut State) -> i32 {
    unsafe {
        let y = lual_checknumber(state, 1);
        let x = lual_optnumber(state, 2, 1.0);
        (*state).push_number(y.atan2(x));
        1
    }
}

// ── hyperbolic ──────────────────────────────────────────────────────────────
unsafe fn fm_sinh(state: *mut State) -> i32 { unsafe { (*state).push_number(lual_checknumber(state, 1).sinh()); 1 } }
unsafe fn fm_cosh(state: *mut State) -> i32 { unsafe { (*state).push_number(lual_checknumber(state, 1).cosh()); 1 } }
unsafe fn fm_tanh(state: *mut State) -> i32 { unsafe { (*state).push_number(lual_checknumber(state, 1).tanh()); 1 } }
unsafe fn fm_asinh(state: *mut State) -> i32 { unsafe { (*state).push_number(lual_checknumber(state, 1).asinh()); 1 } }
unsafe fn fm_acosh(state: *mut State) -> i32 { unsafe { (*state).push_number(lual_checknumber(state, 1).acosh()); 1 } }
unsafe fn fm_atanh(state: *mut State) -> i32 { unsafe { (*state).push_number(lual_checknumber(state, 1).atanh()); 1 } }

// ── exponential / logarithmic ───────────────────────────────────────────────
unsafe fn fm_exp(state: *mut State) -> i32 { unsafe { (*state).push_number(lual_checknumber(state, 1).exp()); 1 } }
unsafe fn fm_expm1(state: *mut State) -> i32 { unsafe { (*state).push_number(lual_checknumber(state, 1).exp_m1()); 1 } }
unsafe fn fm_log(state: *mut State) -> i32 {
    unsafe {
        let x = lual_checknumber(state, 1);
        let res = match lua_type(state, 2) {
            | None | Some(TagType::Nil) => x.ln(),
            | _ => {
                let base = lual_checknumber(state, 2);
                if base == 2.0 { x.log2() } else if base == 10.0 { x.log10() } else { x.ln() / base.ln() }
            },
        };
        (*state).push_number(res);
        1
    }
}
unsafe fn fm_log2(state: *mut State) -> i32 { unsafe { (*state).push_number(lual_checknumber(state, 1).log2()); 1 } }
unsafe fn fm_log10(state: *mut State) -> i32 { unsafe { (*state).push_number(lual_checknumber(state, 1).log10()); 1 } }
unsafe fn fm_log1p(state: *mut State) -> i32 { unsafe { (*state).push_number(lual_checknumber(state, 1).ln_1p()); 1 } }
unsafe fn fm_pow(state: *mut State) -> i32 {
    unsafe { (*state).push_number(lual_checknumber(state, 1).powf(lual_checknumber(state, 2))); 1 }
}
unsafe fn fm_sqrt(state: *mut State) -> i32 { unsafe { (*state).push_number(lual_checknumber(state, 1).sqrt()); 1 } }
unsafe fn fm_cbrt(state: *mut State) -> i32 { unsafe { (*state).push_number(lual_checknumber(state, 1).cbrt()); 1 } }
unsafe fn fm_hypot(state: *mut State) -> i32 {
    unsafe { (*state).push_number(lual_checknumber(state, 1).hypot(lual_checknumber(state, 2))); 1 }
}

// ── rounding / truncation ───────────────────────────────────────────────────
unsafe fn fm_floor(state: *mut State) -> i32 {
    unsafe { if lua_isinteger(state, 1) { lua_settop(state, 1); } else { push_numericcc(state, lual_checknumber(state, 1).floor()); } 1 }
}
unsafe fn fm_ceil(state: *mut State) -> i32 {
    unsafe { if lua_isinteger(state, 1) { lua_settop(state, 1); } else { push_numericcc(state, lual_checknumber(state, 1).ceil()); } 1 }
}
unsafe fn fm_trunc(state: *mut State) -> i32 {
    unsafe { if lua_isinteger(state, 1) { lua_settop(state, 1); } else { push_numericcc(state, lual_checknumber(state, 1).trunc()); } 1 }
}
unsafe fn fm_round(state: *mut State) -> i32 {
    unsafe { if lua_isinteger(state, 1) { lua_settop(state, 1); } else { push_numericcc(state, lual_checknumber(state, 1).round()); } 1 }
}

// ── classification / predicates ─────────────────────────────────────────────
unsafe fn fm_abs(state: *mut State) -> i32 {
    unsafe {
        if lua_isinteger(state, 1) {
            let mut n = lua_tointegerx(state, 1, null_mut());
            if n < 0 { n = (0usize).wrapping_sub(n as usize) as i64; }
            (*state).push_integer(n);
        } else { (*state).push_number(lual_checknumber(state, 1).abs()); }
        1
    }
}
unsafe fn fm_isnan(state: *mut State) -> i32 {
    unsafe {
        lual_checkany(state, 1);
        (*state).push_boolean(lua_type(state, 1) == Some(TagType::Numeric) && !lua_isinteger(state, 1) && lual_checknumber(state, 1).is_nan());
        1
    }
}
unsafe fn fm_isinf(state: *mut State) -> i32 {
    unsafe {
        lual_checkany(state, 1);
        (*state).push_boolean(lua_type(state, 1) == Some(TagType::Numeric) && !lua_isinteger(state, 1) && lual_checknumber(state, 1).is_infinite());
        1
    }
}
unsafe fn fm_isfinite(state: *mut State) -> i32 {
    unsafe {
        lual_checkany(state, 1);
        (*state).push_boolean(
            lua_type(state, 1) == Some(TagType::Numeric) && (lua_isinteger(state, 1) || lual_checknumber(state, 1).is_finite()),
        );
        1
    }
}
unsafe fn fm_copysign(state: *mut State) -> i32 {
    unsafe { (*state).push_number(lual_checknumber(state, 1).copysign(lual_checknumber(state, 2))); 1 }
}
unsafe fn fm_sign(state: *mut State) -> i32 {
    unsafe {
        if lua_isinteger(state, 1) {
            let n = lua_tointegerx(state, 1, null_mut());
            (*state).push_integer(if n > 0 { 1 } else if n < 0 { -1 } else { 0 });
        } else {
            let x = lual_checknumber(state, 1);
            (*state).push_number(if x > 0.0 { 1.0 } else if x < 0.0 { -1.0 } else if x == 0.0 { 0.0 } else { f64::NAN });
        }
        1
    }
}

// ── remainders / modular ────────────────────────────────────────────────────
unsafe fn fm_fmod(state: *mut State) -> i32 {
    unsafe {
        if lua_isinteger(state, 1) && lua_isinteger(state, 2) {
            let d = lua_tointegerx(state, 2, null_mut());
            if (d as usize).wrapping_add(1) <= 1 {
                if d == 0 { lual_argerror(state, 2, c"zero".as_ptr()); }
                (*state).push_integer(0);
            } else { (*state).push_integer(lua_tointegerx(state, 1, null_mut()) % d); }
        } else { (*state).push_number(lual_checknumber(state, 1) % lual_checknumber(state, 2)); }
        1
    }
}
unsafe fn fm_remainder(state: *mut State) -> i32 {
    unsafe {
        let x = lual_checknumber(state, 1);
        let y = lual_checknumber(state, 2);
        (*state).push_number(x - (x / y).round() * y);
        1
    }
}
unsafe fn fm_modf(state: *mut State) -> i32 {
    unsafe {
        if lua_isinteger(state, 1) { lua_settop(state, 1); (*state).push_number(0.0); }
        else {
            let n = lual_checknumber(state, 1);
            let ip = if n < 0.0 { n.ceil() } else { n.floor() };
            push_numericcc(state, ip);
            (*state).push_number(if n == ip { 0.0 } else { n - ip });
        }
        2
    }
}
unsafe fn fm_frexp(state: *mut State) -> i32 {
    unsafe { let (m, e) = frexp_(lual_checknumber(state, 1)); (*state).push_number(m); (*state).push_integer(e as i64); 2 }
}
unsafe fn fm_ldexp(state: *mut State) -> i32 {
    unsafe { (*state).push_number(ldexp_(lual_checknumber(state, 1), lual_checkinteger(state, 2) as i32)); 1 }
}

// ── angular ─────────────────────────────────────────────────────────────────
unsafe fn fm_deg(state: *mut State) -> i32 { unsafe { (*state).push_number(lual_checknumber(state, 1) * (180.0 / PI)); 1 } }
unsafe fn fm_rad(state: *mut State) -> i32 { unsafe { (*state).push_number(lual_checknumber(state, 1) * (PI / 180.0)); 1 } }

// ── comparison / clamping ───────────────────────────────────────────────────
unsafe fn fm_min(state: *mut State) -> i32 {
    unsafe {
        let n = (*state).get_top();
        if n < 1 { lual_argerror(state, 1, c"value expected".as_ptr()); }
        let mut imin = 1;
        for i in 2..=n { if lua_compare(state, i, imin, 1) { imin = i; } }
        lua_pushvalue(state, imin); 1
    }
}
unsafe fn fm_max(state: *mut State) -> i32 {
    unsafe {
        let n = (*state).get_top();
        if n < 1 { lual_argerror(state, 1, c"value expected".as_ptr()); }
        let mut imax = 1;
        for i in 2..=n { if lua_compare(state, imax, i, 1) { imax = i; } }
        lua_pushvalue(state, imax); 1
    }
}
unsafe fn fm_clamp(state: *mut State) -> i32 {
    unsafe {
        let x = lual_checknumber(state, 1);
        let lo = lual_checknumber(state, 2);
        let hi = lual_checknumber(state, 3);
        (*state).push_number(if x < lo { lo } else if x > hi { hi } else { x }); 1
    }
}

// ── integer arithmetic ──────────────────────────────────────────────────────
unsafe fn fm_gcd(state: *mut State) -> i32 {
    unsafe {
        let mut a = lual_checkinteger(state, 1).unsigned_abs();
        let mut b = lual_checkinteger(state, 2).unsigned_abs();
        while b != 0 { let t = b; b = a % b; a = t; }
        (*state).push_integer(a as i64); 1
    }
}
unsafe fn fm_lcm(state: *mut State) -> i32 {
    unsafe {
        let a = lual_checkinteger(state, 1).unsigned_abs();
        let b = lual_checkinteger(state, 2).unsigned_abs();
        if a == 0 || b == 0 { (*state).push_integer(0); }
        else {
            let mut ga = a; let mut gb = b;
            while gb != 0 { let t = gb; gb = ga % gb; ga = t; }
            (*state).push_integer((a / ga * b) as i64);
        }
        1
    }
}
unsafe fn fm_factorial(state: *mut State) -> i32 {
    unsafe {
        let n = lual_checkinteger(state, 1);
        if n < 0 { return lual_error(state, c"factorial of negative number".as_ptr(), &[]); }
        if n > 20 { return lual_error(state, c"factorial argument too large".as_ptr(), &[]); }
        let mut r: i64 = 1;
        for i in 2..=n { r *= i; }
        (*state).push_integer(r); 1
    }
}
unsafe fn fm_comb(state: *mut State) -> i32 {
    unsafe {
        let n = lual_checkinteger(state, 1);
        let k = lual_checkinteger(state, 2);
        if k < 0 || k > n { (*state).push_integer(0); return 1; }
        let k = k.min(n - k);
        let mut r: i64 = 1;
        for i in 0..k { r = r * (n - i) / (i + 1); }
        (*state).push_integer(r); 1
    }
}
unsafe fn fm_perm(state: *mut State) -> i32 {
    unsafe {
        let n = lual_checkinteger(state, 1);
        let k = lual_optinteger(state, 2, n);
        if k < 0 || k > n { (*state).push_integer(0); return 1; }
        let mut r: i64 = 1;
        for i in 0..k { r *= n - i; }
        (*state).push_integer(r); 1
    }
}

// ── special functions ───────────────────────────────────────────────────────
fn erf_approx(x: f64) -> f64 {
    if x == 0.0 { return 0.0; }
    let sign = if x >= 0.0 { 1.0 } else { -1.0 };
    let x = x.abs();
    let t = 1.0 / (1.0 + 0.3275911 * x);
    let y = 1.0
        - (((((1.061405429 * t - 1.453152027) * t) + 1.421413741) * t - 0.284496736) * t + 0.254829592)
            * t * (-x * x).exp();
    sign * y
}
unsafe fn fm_erf(state: *mut State) -> i32 { unsafe { (*state).push_number(erf_approx(lual_checknumber(state, 1))); 1 } }
unsafe fn fm_erfc(state: *mut State) -> i32 { unsafe { (*state).push_number(1.0 - erf_approx(lual_checknumber(state, 1))); 1 } }

fn gamma_lanczos(x: f64) -> f64 {
    if x < 0.5 {
        PI / ((PI * x).sin() * gamma_lanczos(1.0 - x))
    } else {
        const P: [f64; 8] = [
            676.5203681218851, -1259.1392167224028, 771.32342877765313, -176.61502916214059,
            12.507343278686905, -0.13857109526572012, 9.9843695780195716e-6, 1.5056327351493116e-7,
        ];
        let x = x - 1.0;
        let mut a = 0.99999999999980993;
        let t = x + 7.5;
        for (i, &p) in P.iter().enumerate() { a += p / (x + i as f64 + 1.0); }
        (2.0 * PI).sqrt() * t.powf(x + 0.5) * (-t).exp() * a
    }
}
unsafe fn fm_gamma(state: *mut State) -> i32 { unsafe { (*state).push_number(gamma_lanczos(lual_checknumber(state, 1))); 1 } }
unsafe fn fm_lgamma(state: *mut State) -> i32 { unsafe { (*state).push_number(gamma_lanczos(lual_checknumber(state, 1)).abs().ln()); 1 } }

// ── float introspection ─────────────────────────────────────────────────────
unsafe fn fm_nextafter(state: *mut State) -> i32 {
    unsafe {
        let x = lual_checknumber(state, 1);
        let y = lual_checknumber(state, 2);
        let r = if x == y { y } else if x.is_nan() || y.is_nan() { f64::NAN }
        else if x == 0.0 { if y > 0.0 { f64::MIN_POSITIVE * f64::EPSILON } else { -f64::MIN_POSITIVE * f64::EPSILON } }
        else { let b = x.to_bits(); f64::from_bits(if (x < y) == (x > 0.0) { b + 1 } else { b - 1 }) };
        (*state).push_number(r); 1
    }
}
unsafe fn fm_ulp(state: *mut State) -> i32 {
    unsafe {
        let x = lual_checknumber(state, 1);
        let r = if x.is_nan() || x.is_infinite() { f64::NAN }
        else if x == 0.0 { f64::MIN_POSITIVE * f64::EPSILON }
        else { f64::from_bits(x.abs().to_bits() + 1) - x.abs() };
        (*state).push_number(r); 1
    }
}

// ── aggregate ───────────────────────────────────────────────────────────────
unsafe fn fm_sum(state: *mut State) -> i32 {
    unsafe {
        (*state).lual_checktype(1, TagType::Table);
        let len = get_length_raw(state, 1) as i64;
        let mut total = 0.0_f64;
        for i in 1..=len { lua_rawgeti(state, 1, i); total += lual_checknumber(state, -1); lua_settop(state, -2); }
        (*state).push_number(total); 1
    }
}
unsafe fn fm_prod(state: *mut State) -> i32 {
    unsafe {
        (*state).lual_checktype(1, TagType::Table);
        let len = get_length_raw(state, 1) as i64;
        let mut total = 1.0_f64;
        for i in 1..=len { lua_rawgeti(state, 1, i); total *= lual_checknumber(state, -1); lua_settop(state, -2); }
        (*state).push_number(total); 1
    }
}
unsafe fn fm_dist(state: *mut State) -> i32 {
    unsafe {
        (*state).lual_checktype(1, TagType::Table);
        (*state).lual_checktype(2, TagType::Table);
        let len = get_length_raw(state, 1) as i64;
        let mut total = 0.0_f64;
        for i in 1..=len {
            lua_rawgeti(state, 1, i); lua_rawgeti(state, 2, i);
            let d = lual_checknumber(state, -2) - lual_checknumber(state, -1);
            total += d * d;
            lua_settop(state, -3);
        }
        (*state).push_number(total.sqrt()); 1
    }
}

// ── Lua compat ──────────────────────────────────────────────────────────────
unsafe fn fm_tointeger(state: *mut State) -> i32 {
    unsafe {
        let mut ok = false;
        let n = lua_tointegerx(state, 1, &mut ok);
        if ok { (*state).push_integer(n); } else { lual_checkany(state, 1); (*state).push_nil(); }
        1
    }
}
unsafe fn fm_type(state: *mut State) -> i32 {
    unsafe {
        if lua_type(state, 1) == Some(TagType::Numeric) {
            lua_pushstring(state, if lua_isinteger(state, 1) { c"integer".as_ptr() } else { c"float".as_ptr() });
        } else { lual_checkany(state, 1); (*state).push_nil(); }
        1
    }
}
unsafe fn fm_ult(state: *mut State) -> i32 {
    unsafe { (*state).push_boolean((lual_checkinteger(state, 1) as u64) < (lual_checkinteger(state, 2) as u64)); 1 }
}

// ── random ──────────────────────────────────────────────────────────────────
unsafe fn fm_random(state: *mut State) -> i32 {
    unsafe {
        let rs = (*state).to_pointer(LUA_REGISTRYINDEX - 1) as *mut RandomState;
        let rv = next_random(((*rs).randomstate_data).as_mut_ptr());
        match (*state).get_top() {
            | 0 => { (*state).push_number(i2d(rv)); 1 },
            | 1 => {
                let high = lual_checkinteger(state, 1);
                if high == 0 { (*state).push_integer(rv as i64); return 1; }
                (*state).push_integer(project(rv, (high as usize).wrapping_sub(1), rs).wrapping_add(1) as i64); 1
            },
            | 2 => {
                let lo = lual_checkinteger(state, 1); let hi = lual_checkinteger(state, 2);
                if lo > hi { lual_argerror(state, 1, c"interval is empty".as_ptr()); }
                (*state).push_integer(project(rv, (hi as usize).wrapping_sub(lo as usize), rs).wrapping_add(lo as usize) as i64); 1
            },
            | _ => lual_error(state, c"wrong number of arguments".as_ptr(), &[]),
        }
    }
}
unsafe fn fm_randomseed(state: *mut State) -> i32 {
    unsafe {
        let rs = (*state).to_pointer(LUA_REGISTRYINDEX - 1) as *mut RandomState;
        if lua_type(state, 1).is_none() { random_seed(state, rs); }
        else { set_seed(state, ((*rs).randomstate_data).as_mut_ptr(), lual_checkinteger(state, 1) as usize, lual_optinteger(state, 2, 0) as usize); }
        2
    }
}

const FMATH_RANDOM_FUNCTIONS: [RegisteredFunction; 2] = [
    RegisteredFunction { registeredfunction_name: c"random".as_ptr(), registeredfunction_function: Some(fm_random as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"randomseed".as_ptr(), registeredfunction_function: Some(fm_randomseed as unsafe fn(*mut State) -> i32) },
];

const FMATH_FUNCTIONS: [RegisteredFunction; 59] = [
    RegisteredFunction { registeredfunction_name: c"sin".as_ptr(), registeredfunction_function: Some(fm_sin as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"cos".as_ptr(), registeredfunction_function: Some(fm_cos as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"tan".as_ptr(), registeredfunction_function: Some(fm_tan as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"asin".as_ptr(), registeredfunction_function: Some(fm_asin as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"acos".as_ptr(), registeredfunction_function: Some(fm_acos as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"atan".as_ptr(), registeredfunction_function: Some(fm_atan as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"sinh".as_ptr(), registeredfunction_function: Some(fm_sinh as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"cosh".as_ptr(), registeredfunction_function: Some(fm_cosh as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"tanh".as_ptr(), registeredfunction_function: Some(fm_tanh as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"asinh".as_ptr(), registeredfunction_function: Some(fm_asinh as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"acosh".as_ptr(), registeredfunction_function: Some(fm_acosh as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"atanh".as_ptr(), registeredfunction_function: Some(fm_atanh as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"exp".as_ptr(), registeredfunction_function: Some(fm_exp as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"expm1".as_ptr(), registeredfunction_function: Some(fm_expm1 as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"log".as_ptr(), registeredfunction_function: Some(fm_log as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"log2".as_ptr(), registeredfunction_function: Some(fm_log2 as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"log10".as_ptr(), registeredfunction_function: Some(fm_log10 as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"log1p".as_ptr(), registeredfunction_function: Some(fm_log1p as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"pow".as_ptr(), registeredfunction_function: Some(fm_pow as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"sqrt".as_ptr(), registeredfunction_function: Some(fm_sqrt as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"cbrt".as_ptr(), registeredfunction_function: Some(fm_cbrt as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"hypot".as_ptr(), registeredfunction_function: Some(fm_hypot as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"floor".as_ptr(), registeredfunction_function: Some(fm_floor as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"ceil".as_ptr(), registeredfunction_function: Some(fm_ceil as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"trunc".as_ptr(), registeredfunction_function: Some(fm_trunc as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"round".as_ptr(), registeredfunction_function: Some(fm_round as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"abs".as_ptr(), registeredfunction_function: Some(fm_abs as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"isnan".as_ptr(), registeredfunction_function: Some(fm_isnan as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"isinf".as_ptr(), registeredfunction_function: Some(fm_isinf as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"isfinite".as_ptr(), registeredfunction_function: Some(fm_isfinite as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"copysign".as_ptr(), registeredfunction_function: Some(fm_copysign as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"sign".as_ptr(), registeredfunction_function: Some(fm_sign as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"fmod".as_ptr(), registeredfunction_function: Some(fm_fmod as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"remainder".as_ptr(), registeredfunction_function: Some(fm_remainder as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"modf".as_ptr(), registeredfunction_function: Some(fm_modf as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"frexp".as_ptr(), registeredfunction_function: Some(fm_frexp as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"ldexp".as_ptr(), registeredfunction_function: Some(fm_ldexp as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"deg".as_ptr(), registeredfunction_function: Some(fm_deg as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"rad".as_ptr(), registeredfunction_function: Some(fm_rad as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"min".as_ptr(), registeredfunction_function: Some(fm_min as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"max".as_ptr(), registeredfunction_function: Some(fm_max as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"clamp".as_ptr(), registeredfunction_function: Some(fm_clamp as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"gcd".as_ptr(), registeredfunction_function: Some(fm_gcd as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"lcm".as_ptr(), registeredfunction_function: Some(fm_lcm as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"factorial".as_ptr(), registeredfunction_function: Some(fm_factorial as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"comb".as_ptr(), registeredfunction_function: Some(fm_comb as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"perm".as_ptr(), registeredfunction_function: Some(fm_perm as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"erf".as_ptr(), registeredfunction_function: Some(fm_erf as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"erfc".as_ptr(), registeredfunction_function: Some(fm_erfc as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"gamma".as_ptr(), registeredfunction_function: Some(fm_gamma as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"lgamma".as_ptr(), registeredfunction_function: Some(fm_lgamma as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"nextafter".as_ptr(), registeredfunction_function: Some(fm_nextafter as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"ulp".as_ptr(), registeredfunction_function: Some(fm_ulp as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"sum".as_ptr(), registeredfunction_function: Some(fm_sum as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"prod".as_ptr(), registeredfunction_function: Some(fm_prod as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"dist".as_ptr(), registeredfunction_function: Some(fm_dist as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"tointeger".as_ptr(), registeredfunction_function: Some(fm_tointeger as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"type".as_ptr(), registeredfunction_function: Some(fm_type as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"ult".as_ptr(), registeredfunction_function: Some(fm_ult as unsafe fn(*mut State) -> i32) },
];

pub unsafe fn luaopen_fmath(state: *mut State) -> i32 {
    unsafe {
        (*state).lua_createtable();
        lual_setfuncs(state, FMATH_FUNCTIONS.as_ptr(), FMATH_FUNCTIONS.len(), 0);
        (*state).push_number(PI);       lua_setfield(state, -2, c"pi".as_ptr());
        (*state).push_number(TAU);      lua_setfield(state, -2, c"tau".as_ptr());
        (*state).push_number(E);        lua_setfield(state, -2, c"e".as_ptr());
        (*state).push_number(f64::INFINITY); lua_setfield(state, -2, c"huge".as_ptr());
        (*state).push_number(f64::INFINITY); lua_setfield(state, -2, c"inf".as_ptr());
        (*state).push_number(f64::NAN); lua_setfield(state, -2, c"nan".as_ptr());
        (*state).push_integer(i64::MAX); lua_setfield(state, -2, c"maxinteger".as_ptr());
        (*state).push_integer(i64::MIN); lua_setfield(state, -2, c"mininteger".as_ptr());
        let rs = User::lua_newuserdatauv(state, size_of::<RandomState>(), 0) as *mut RandomState;
        random_seed(state, rs);
        lua_settop(state, -3);
        lual_setfuncs(state, FMATH_RANDOM_FUNCTIONS.as_ptr(), FMATH_RANDOM_FUNCTIONS.len(), 1);
        1
    }
}
