use crate::c::*;
use crate::character::*;
use crate::interpreter::*;
use crate::tag::*;
use crate::tvalue::*;
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
pub unsafe fn luav_flttointeger(input: f64, result: *mut i64, mode: F2I) -> bool {
    unsafe {
        let mut number: f64 = input.floor();
        if input != number {
            if mode == F2I::Equal {
                return false;
            } else if mode == F2I::Ceiling {
                number += 1.0;
            }
        }
        return number >= (-(MAXIMUM_SIZE as i64) - 1) as f64 && number < -((-(MAXIMUM_SIZE as i64) - 1) as f64) && {
            *result = number as i64;
            true
        };
    }
}
pub unsafe fn luav_tointegerns(obj: *const TValue, result: *mut i64, mode: F2I) -> i32 {
    unsafe {
        if (*obj).get_tagvariant() == TagVariant::NumericNumber {
            return if luav_flttointeger((*obj).tvalue_value.value_number, result, mode) { 1 } else { 0 };
        } else if (*obj).get_tagvariant() == TagVariant::NumericInteger {
            *result = (*obj).tvalue_value.value_integer;
            return 1;
        } else {
            return 0;
        };
    }
}
pub unsafe fn luav_tointeger(mut obj: *const TValue, result: *mut i64, mode: F2I) -> i32 {
    unsafe {
        let mut tvalue = TValue::new(TagVariant::NilNil);
        if tvalue.from_string_to_number(obj) {
            obj = &mut tvalue;
        }
        luav_tointegerns(obj, result, mode)
    }
}
pub unsafe fn ltintfloat(i: i64, number: f64) -> bool {
    unsafe {
        if ((1 as usize) << 53 as i32).wrapping_add(i as usize) <= 2 * ((1 as usize) << 53 as i32) {
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
        if ((1 as usize) << 53 as i32).wrapping_add(i as usize) <= 2 * ((1 as usize) << 53 as i32) {
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
        if ((1 as usize) << 53 as i32).wrapping_add(i as usize) <= 2 * ((1 as usize) << 53 as i32) {
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
        if ((1 as usize) << 53 as i32).wrapping_add(i as usize) <= 2 * ((1 as usize) << 53 as i32) {
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
        if (*l).get_tagvariant() == TagVariant::NumericInteger {
            let li: i64 = (*l).tvalue_value.value_integer;
            if (*r).get_tagvariant() == TagVariant::NumericInteger {
                return li < (*r).tvalue_value.value_integer;
            } else {
                return ltintfloat(li, (*r).tvalue_value.value_number);
            }
        } else {
            let lf: f64 = (*l).tvalue_value.value_number;
            if (*r).get_tagvariant() == TagVariant::NumericNumber {
                return lf < (*r).tvalue_value.value_number;
            } else {
                return ltfloatint(lf, (*r).tvalue_value.value_integer);
            }
        };
    }
}
pub unsafe fn lenum(l: *const TValue, r: *const TValue) -> bool {
    unsafe {
        if (*l).get_tagvariant() == TagVariant::NumericInteger {
            let li: i64 = (*l).tvalue_value.value_integer;
            if (*r).get_tagvariant() == TagVariant::NumericInteger {
                return li <= (*r).tvalue_value.value_integer;
            } else {
                return leintfloat(li, (*r).tvalue_value.value_number);
            }
        } else {
            let lf: f64 = (*l).tvalue_value.value_number;
            if (*r).get_tagvariant() == TagVariant::NumericNumber {
                return lf <= (*r).tvalue_value.value_number;
            } else {
                return lefloatint(lf, (*r).tvalue_value.value_integer);
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
        if y <= -(((size_of::<i64>() as usize) * 8) as i32) as i64 {
            return 0;
        } else {
            return (x as usize >> -y as usize) as i64;
        }
    } else if y >= ((size_of::<i64>() as usize) * 8) as i64 {
        return 0;
    } else {
        return ((x as usize) << y as usize) as i64;
    };
}
pub unsafe fn b_str2int(mut s: *const i8, base: i32, pn: *mut i64) -> *const i8 {
    unsafe {
        let mut n: usize = 0;
        let mut is_negative_: i32 = 0;
        s = s.offset(libc::strspn(s, c" \x0C\n\r\t\x0B".as_ptr()) as isize);
        if *s as i32 == Character::Hyphen as i32 {
            s = s.offset(1);
            is_negative_ = 1;
        } else if *s as i32 == Character::Plus as i32 {
            s = s.offset(1);
        }
        if !Character::from(*s as i32).is_alphanumeric() {
            return null();
        }
        loop {
            let digit_0: i32 = if Character::from(*s as u8 as i32).is_digit_decimal() {
                *s as i32 - Character::Digit0 as i32
            } else {
                toupper(*s as u8 as i32) - Character::UpperA as i32 + 10 as i32
            };
            if digit_0 >= base {
                return null();
            }
            n = n * base as usize + digit_0 as usize;
            s = s.offset(1);
            if !Character::from(*s as i32).is_alphanumeric() {
                break;
            }
        }
        s = s.offset(libc::strspn(s, c" \x0C\n\r\t\x0B".as_ptr()) as isize);
        *pn = (if is_negative_ != 0 { (0usize).wrapping_sub(n) } else { n }) as i64;
        return s;
    }
}
