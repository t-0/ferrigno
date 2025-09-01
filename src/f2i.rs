use crate::tvalue::*;
use crate::tag::*;
use crate::value::*;
use crate::state::*;
use crate::utility::c::*;
use libc::{toupper};
#[derive(PartialEq)]
#[repr(C)]
pub enum F2I {
    Equal,
    Floor,
    Ceiling,
}
pub unsafe extern "C" fn luav_flttointeger(n: f64, p: *mut i64, mode: F2I) -> i32 {
    unsafe {
        let mut f: f64 = n.floor();
        if n != f {
            if mode == F2I::Equal {
                return 0;
            } else if mode == F2I::Ceiling {
                f += 1.0;
            }
        }
        return (f >= (-(0x7FFFFFFFFFFFFFFF as i64) - 1 as i64) as f64
            && f < -((-(0x7FFFFFFFFFFFFFFF as i64) - 1 as i64) as f64)
            && {
                *p = f as i64;
                1 != 0
            }) as i32;
    }
}
pub unsafe extern "C" fn luav_tointegerns(obj: *const TValue, p: *mut i64, mode: F2I) -> i32 {
    unsafe {
        if (*obj).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
            return luav_flttointeger((*obj).value.n, p, mode);
        } else if (*obj).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
            *p = (*obj).value.i;
            return 1;
        } else {
            return 0;
        };
    }
}
pub unsafe extern "C" fn luav_tointeger(mut obj: *const TValue, p: *mut i64, mode: F2I) -> i32 {
    unsafe {
        let mut v: TValue = TValue {
            value: Value {
                object: std::ptr::null_mut(),
            },
            tag: 0,
        };
        if l_strton(obj, &mut v) != 0 {
            obj = &mut v;
        }
        return luav_tointegerns(obj, p, mode);
    }
}
pub unsafe extern "C" fn ltintfloat(i: i64, f: f64) -> bool {
    unsafe {
        if ((1 as u64) << 53 as i32).wrapping_add(i as u64)
            <= (2 as u64).wrapping_mul((1 as u64) << 53 as i32)
        {
            return (i as f64) < f;
        } else {
            let mut fi: i64 = 0;
            if luav_flttointeger(f, &mut fi, F2I::Ceiling) != 0 {
                return i < fi;
            } else {
                return f > 0.0;
            }
        };
    }
}
pub unsafe extern "C" fn leintfloat(i: i64, f: f64) -> bool {
    unsafe {
        if ((1 as u64) << 53 as i32).wrapping_add(i as u64)
            <= (2 as u64).wrapping_mul((1 as u64) << 53 as i32)
        {
            return i as f64 <= f;
        } else {
            let mut fi: i64 = 0;
            if luav_flttointeger(f, &mut fi, F2I::Floor) != 0 {
                return i <= fi;
            } else {
                return f > 0.0;
            }
        };
    }
}
pub unsafe extern "C" fn ltfloatint(f: f64, i: i64) -> bool {
    unsafe {
        if ((1 as u64) << 53 as i32).wrapping_add(i as u64)
            <= (2 as u64).wrapping_mul((1 as u64) << 53 as i32)
        {
            return f < i as f64;
        } else {
            let mut fi: i64 = 0;
            if luav_flttointeger(f, &mut fi, F2I::Floor) != 0 {
                return fi < i;
            } else {
                return f < 0.0;
            }
        };
    }
}
pub unsafe extern "C" fn lefloatint(f: f64, i: i64) -> bool {
    unsafe {
        if ((1 as u64) << 53 as i32).wrapping_add(i as u64)
            <= (2 as u64).wrapping_mul((1 as u64) << 53 as i32)
        {
            return f <= i as f64;
        } else {
            let mut fi: i64 = 0;
            if luav_flttointeger(f, &mut fi, F2I::Ceiling) != 0 {
                return fi <= i;
            } else {
                return f < 0.0;
            }
        };
    }
}
pub unsafe extern "C" fn ltnum(l: *const TValue, r: *const TValue) -> bool {
    unsafe {
        if (*l).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
            let li: i64 = (*l).value.i;
            if (*r).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                return li < (*r).value.i;
            } else {
                return ltintfloat(li, (*r).value.n);
            }
        } else {
            let lf: f64 = (*l).value.n;
            if (*r).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                return lf < (*r).value.n;
            } else {
                return ltfloatint(lf, (*r).value.i);
            }
        };
    }
}
pub unsafe extern "C" fn lenum(l: *const TValue, r: *const TValue) -> bool {
    unsafe {
        if (*l).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
            let li: i64 = (*l).value.i;
            if (*r).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                return li <= (*r).value.i;
            } else {
                return leintfloat(li, (*r).value.n);
            }
        } else {
            let lf: f64 = (*l).value.n;
            if (*r).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                return lf <= (*r).value.n;
            } else {
                return lefloatint(lf, (*r).value.i);
            }
        };
    }
}
pub unsafe extern "C" fn luav_idiv(state: *mut State, m: i64, n: i64) -> i64 {
    unsafe {
        if (((n as u64).wrapping_add(1 as u64) <= 1 as u64) as i32 != 0) as i64
            != 0
        {
            if n == 0 {
                luag_runerror(
                    state,
                    b"attempt to divide by zero\0" as *const u8 as *const i8,
                );
            }
            return (0u64).wrapping_sub(m as u64) as i64;
        } else {
            let mut q: i64 = m / n;
            if m ^ n < 0 && m % n != 0 {
                q -= 1;
            }
            return q;
        };
    }
}
pub unsafe extern "C" fn luav_mod(state: *mut State, m: i64, n: i64) -> i64 {
    unsafe {
        if (((n as u64).wrapping_add(1 as u64) <= 1 as u64) as i32 != 0) as i64
            != 0
        {
            if n == 0 {
                luag_runerror(
                    state,
                    b"attempt to perform 'n%%0'\0" as *const u8 as *const i8,
                );
            }
            return 0;
        } else {
            let mut r: i64 = m % n;
            if r != 0 && r ^ n < 0 {
                r += n;
            }
            return r;
        };
    }
}
pub unsafe extern "C" fn luav_modf(mut _state: *mut State, m: f64, n: f64) -> f64 {
    unsafe {
        let mut r: f64 = fmod(m, n);
        if if r > 0.0 {
            (n < 0.0) as i32
        } else {
            (r < 0.0 && n > 0.0) as i32
        } != 0
        {
            r += n;
        }
        return r;
    }
}
pub unsafe extern "C" fn luav_shiftl(x: i64, y: i64) -> i64 {
    if y < 0 {
        if y <= -((::core::mem::size_of::<i64>() as u64).wrapping_mul(8 as u64) as i32) as i64 {
            return 0;
        } else {
            return (x as u64 >> -y as u64) as i64;
        }
    } else if y >= (::core::mem::size_of::<i64>() as u64).wrapping_mul(8 as u64) as i64 {
        return 0;
    } else {
        return ((x as u64) << y as u64) as i64;
    };
}
pub unsafe extern "C" fn b_str2int(mut s: *const i8, base: i32, pn: *mut i64) -> *const i8 {
    unsafe {
        let mut n: u64 = 0;
        let mut is_negative_: i32 = 0;
        s = s.offset(strspn(s, b" \x0C\n\r\t\x0B\0" as *const u8 as *const i8) as isize);
        if *s as i32 == '-' as i32 {
            s = s.offset(1);
            is_negative_ = 1;
        } else if *s as i32 == '+' as i32 {
            s = s.offset(1);
        }
        if *(*__ctype_b_loc()).offset(*s as u8 as isize) as i32
            & _ISALPHANUMERIC as i32
            == 0
        {
            return std::ptr::null();
        }
        loop {
            let digit_0: i32 = if *(*__ctype_b_loc()).offset(*s as u8 as isize) as i32
                & _ISDIGIT as i32
                != 0
            {
                *s as i32 - '0' as i32
            } else {
                toupper(*s as u8 as i32) - 'A' as i32 + 10 as i32
            };
            if digit_0 >= base {
                return std::ptr::null();
            }
            n = n.wrapping_mul(base as u64).wrapping_add(digit_0 as u64);
            s = s.offset(1);
            if !(*(*__ctype_b_loc()).offset(*s as u8 as isize) as i32
                & _ISALPHANUMERIC as i32
                != 0)
            {
                break;
            }
        }
        s = s.offset(strspn(s, b" \x0C\n\r\t\x0B\0" as *const u8 as *const i8) as isize);
        *pn = (if is_negative_ != 0 {
            (0u64).wrapping_sub(n)
        } else {
            n
        }) as i64;
        return s;
    }
}
