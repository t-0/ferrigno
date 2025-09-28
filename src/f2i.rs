use crate::character::*;
use crate::interpreter::*;
use crate::tag::*;
use crate::tvalue::*;
use crate::utility::c::*;
use crate::utility::*;
use libc::toupper;
use std::ptr::*;
#[derive(PartialEq)]
#[repr(C)]
pub enum F2I {
    Equal,
    Floor,
    Ceiling,
}
pub unsafe fn luav_flttointeger(n: f64, p: *mut i64, mode: F2I) -> bool {
    unsafe {
        let mut number: f64 = n.floor();
        if n != number {
            if mode == F2I::Equal {
                return false;
            } else if mode == F2I::Ceiling {
                number += 1.0;
            }
        }
        return number >= (-(MAXIMUM_SIZE as i64) - 1) as f64 && number < -((-(MAXIMUM_SIZE as i64) - 1) as f64) && {
            *p = number as i64;
            true
        };
    }
}
pub unsafe fn luav_tointegerns(obj: *const TValue, p: *mut i64, mode: F2I) -> i32 {
    unsafe {
        if (*obj).get_tag_variant() == TAG_VARIANT_NUMERIC_NUMBER {
            return if luav_flttointeger((*obj).value.value_number, p, mode) { 1 } else { 0 };
        } else if (*obj).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER {
            *p = (*obj).value.value_integer;
            return 1;
        } else {
            return 0;
        };
    }
}
pub unsafe fn luav_tointeger(mut obj: *const TValue, p: *mut i64, mode: F2I) -> i32 {
    unsafe {
        let mut tvalue = TValue::new(TAG_VARIANT_NIL_NIL);
        if l_strton(obj, &mut tvalue) {
            obj = &mut tvalue;
        }
        return luav_tointegerns(obj, p, mode);
    }
}
pub unsafe fn ltintfloat(i: i64, number: f64) -> bool {
    unsafe {
        if ((1 as usize) << 53 as i32).wrapping_add(i as usize) <= (2 as usize).wrapping_mul((1 as usize) << 53 as i32) {
            return (i as f64) < number;
        } else {
            let mut fi: i64 = 0;
            if luav_flttointeger(number, &mut fi, F2I::Ceiling) {
                return i < fi;
            } else {
                return number > 0.0;
            }
        };
    }
}
pub unsafe fn leintfloat(i: i64, number: f64) -> bool {
    unsafe {
        if ((1 as usize) << 53 as i32).wrapping_add(i as usize) <= (2 as usize).wrapping_mul((1 as usize) << 53 as i32) {
            return i as f64 <= number;
        } else {
            let mut fi: i64 = 0;
            if luav_flttointeger(number, &mut fi, F2I::Floor) {
                return i <= fi;
            } else {
                return number > 0.0;
            }
        };
    }
}
pub unsafe fn ltfloatint(number: f64, i: i64) -> bool {
    unsafe {
        if ((1 as usize) << 53 as i32).wrapping_add(i as usize) <= (2 as usize).wrapping_mul((1 as usize) << 53 as i32) {
            return number < i as f64;
        } else {
            let mut fi: i64 = 0;
            if luav_flttointeger(number, &mut fi, F2I::Floor) {
                return fi < i;
            } else {
                return number < 0.0;
            }
        };
    }
}
pub unsafe fn lefloatint(number: f64, i: i64) -> bool {
    unsafe {
        if ((1 as usize) << 53 as i32).wrapping_add(i as usize) <= (2 as usize).wrapping_mul((1 as usize) << 53 as i32) {
            return number <= i as f64;
        } else {
            let mut fi: i64 = 0;
            if luav_flttointeger(number, &mut fi, F2I::Ceiling) {
                return fi <= i;
            } else {
                return number < 0.0;
            }
        };
    }
}
pub unsafe fn ltnum(l: *const TValue, r: *const TValue) -> bool {
    unsafe {
        if (*l).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER {
            let li: i64 = (*l).value.value_integer;
            if (*r).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER {
                return li < (*r).value.value_integer;
            } else {
                return ltintfloat(li, (*r).value.value_number);
            }
        } else {
            let lf: f64 = (*l).value.value_number;
            if (*r).get_tag_variant() == TAG_VARIANT_NUMERIC_NUMBER {
                return lf < (*r).value.value_number;
            } else {
                return ltfloatint(lf, (*r).value.value_integer);
            }
        };
    }
}
pub unsafe fn lenum(l: *const TValue, r: *const TValue) -> bool {
    unsafe {
        if (*l).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER {
            let li: i64 = (*l).value.value_integer;
            if (*r).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER {
                return li <= (*r).value.value_integer;
            } else {
                return leintfloat(li, (*r).value.value_number);
            }
        } else {
            let lf: f64 = (*l).value.value_number;
            if (*r).get_tag_variant() == TAG_VARIANT_NUMERIC_NUMBER {
                return lf <= (*r).value.value_number;
            } else {
                return lefloatint(lf, (*r).value.value_integer);
            }
        };
    }
}
pub unsafe fn luav_idiv(interpreter: *mut Interpreter, m: i64, n: i64) -> i64 {
    unsafe {
        if (((n as usize).wrapping_add(1 as usize) <= 1 as usize) as i32 != 0) as i64 != 0 {
            if n == 0 {
                luag_runerror(interpreter, c"attempt to divide by zero".as_ptr());
            }
            return (0usize).wrapping_sub(m as usize) as i64;
        } else {
            let mut q: i64 = m / n;
            if m ^ n < 0 && m % n != 0 {
                q -= 1;
            }
            return q;
        };
    }
}
pub unsafe fn luav_mod(interpreter: *mut Interpreter, m: i64, n: i64) -> i64 {
    unsafe {
        if (((n as usize).wrapping_add(1 as usize) <= 1 as usize) as i32 != 0) as i64 != 0 {
            if n == 0 {
                luag_runerror(interpreter, c"attempt to perform 'n%%0'".as_ptr());
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
pub unsafe fn luav_modf(mut _state: *mut Interpreter, m: f64, n: f64) -> f64 {
    unsafe {
        let mut r: f64 = fmod(m, n);
        if if r > 0.0 { (n < 0.0) as i32 } else { (r < 0.0 && n > 0.0) as i32 } != 0 {
            r += n;
        }
        return r;
    }
}
pub unsafe fn luav_shiftl(x: i64, y: i64) -> i64 {
    if y < 0 {
        if y <= -((size_of::<i64>() as usize).wrapping_mul(8 as usize) as i32) as i64 {
            return 0;
        } else {
            return (x as usize >> -y as usize) as i64;
        }
    } else if y >= (size_of::<i64>() as usize).wrapping_mul(8 as usize) as i64 {
        return 0;
    } else {
        return ((x as usize) << y as usize) as i64;
    };
}
pub unsafe fn b_str2int(mut s: *const i8, base: i32, pn: *mut i64) -> *const i8 {
    unsafe {
        let mut n: usize = 0;
        let mut is_negative_: i32 = 0;
        s = s.offset(strspn(s, c" \x0C\n\r\t\x0B".as_ptr()) as isize);
        if *s as i32 == Character::Hyphen as i32 {
            s = s.offset(1);
            is_negative_ = 1;
        } else if *s as i32 == Character::Plus as i32 {
            s = s.offset(1);
        }
        if *(*__ctype_b_loc()).offset(*s as u8 as isize) as i32 & _ISALPHANUMERIC as i32 == 0 {
            return null();
        }
        loop {
            let digit_0: i32 = if *(*__ctype_b_loc()).offset(*s as u8 as isize) as i32 & _ISDIGIT as i32 != 0 {
                *s as i32 - Character::Digit0 as i32
            } else {
                toupper(*s as u8 as i32) - Character::UpperA as i32 + 10 as i32
            };
            if digit_0 >= base {
                return null();
            }
            n = n.wrapping_mul(base as usize).wrapping_add(digit_0 as usize);
            s = s.offset(1);
            if !(*(*__ctype_b_loc()).offset(*s as u8 as isize) as i32 & _ISALPHANUMERIC as i32 != 0) {
                break;
            }
        }
        s = s.offset(strspn(s, c" \x0C\n\r\t\x0B".as_ptr()) as isize);
        *pn = (if is_negative_ != 0 { (0usize).wrapping_sub(n) } else { n }) as i64;
        return s;
    }
}
