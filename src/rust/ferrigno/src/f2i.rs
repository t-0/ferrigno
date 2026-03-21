use crate::character::*;
use crate::functionstate::DOUBLE_MANTISSA_BITS;
use crate::state::*;
use crate::tagvariant::*;
use crate::tvalue::*;
use crate::utility::*;
use std::ptr::*;
const HEX_DIGIT_OFFSET: i32 = 10;
const WHITESPACE: &core::ffi::CStr = c" \x0C\n\r\t\x0B";
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum F2I {
    Equal,
    Floor,
    Ceiling,
}
impl F2I {
    pub unsafe fn convert_f64_i64(&self, input: f64, result: *mut i64) -> bool {
        unsafe {
            let mut number: f64 = input.floor();
            if input != number {
                if *self == F2I::Equal {
                    return false;
                } else if *self == F2I::Ceiling {
                    number += 1.0;
                }
            }
            number >= (-(MAXIMUM_SIZE as i64) - 1) as f64 && number < -((-(MAXIMUM_SIZE as i64) - 1) as f64) && {
                *result = number as i64;
                true
            }
        }
    }
    pub unsafe fn convert_tv_i64(&self, mut obj: *const TValue, result: *mut i64) -> i32 {
        unsafe {
            let mut tvalue = TValue::new(TagVariant::NilNil);
            if tvalue.from_string_to_number(obj) {
                obj = &mut tvalue;
            }
            if let Some(n) = (*obj).as_number() {
                if (*self).convert_f64_i64(n, result) {
                    1
                } else {
                    0
                }
            } else if let Some(i) = (*obj).as_integer() {
                *result = i;
                1
            } else {
                0
            }
        }
    }
}
pub unsafe fn leintfloat(i: i64, number: f64) -> bool {
    unsafe {
        if (1_usize << DOUBLE_MANTISSA_BITS).wrapping_add(i as usize) <= 2 * (1_usize << DOUBLE_MANTISSA_BITS) {
            i as f64 <= number
        } else {
            let mut fi: i64 = 0;
            if F2I::Floor.convert_f64_i64(number, &mut fi) {
                i <= fi
            } else {
                number > 0.0
            }
        }
    }
}
pub unsafe fn ltfloatint(number: f64, i: i64) -> bool {
    unsafe {
        if (1_usize << DOUBLE_MANTISSA_BITS).wrapping_add(i as usize) <= 2 * (1_usize << DOUBLE_MANTISSA_BITS) {
            number < i as f64
        } else {
            let mut fi: i64 = 0;
            if F2I::Floor.convert_f64_i64(number, &mut fi) {
                fi < i
            } else {
                number < 0.0
            }
        }
    }
}
pub unsafe fn lefloatint(number: f64, i: i64) -> bool {
    unsafe {
        if (1_usize << DOUBLE_MANTISSA_BITS).wrapping_add(i as usize) <= 2 * (1_usize << DOUBLE_MANTISSA_BITS) {
            number <= i as f64
        } else {
            let mut fi: i64 = 0;
            if F2I::Ceiling.convert_f64_i64(number, &mut fi) {
                fi <= i
            } else {
                number < 0.0
            }
        }
    }
}
pub unsafe fn ltintfloat(i: i64, number: f64) -> bool {
    unsafe {
        if (1_usize << DOUBLE_MANTISSA_BITS).wrapping_add(i as usize) <= 2 * (1_usize << DOUBLE_MANTISSA_BITS) {
            (i as f64) < number
        } else {
            let mut fi: i64 = 0;
            if F2I::Ceiling.convert_f64_i64(number, &mut fi) {
                i < fi
            } else {
                number > 0.0
            }
        }
    }
}
pub unsafe fn ltnum(l: *const TValue, r: *const TValue) -> bool {
    unsafe {
        if let Some(li) = (*l).as_integer() {
            if let Some(ri) = (*r).as_integer() {
                li < ri
            } else {
                ltintfloat(li, (*r).as_number().unwrap())
            }
        } else {
            let lf: f64 = (*l).as_number().unwrap();
            if let Some(rn) = (*r).as_number() {
                lf < rn
            } else {
                ltfloatint(lf, (*r).as_integer().unwrap())
            }
        }
    }
}
pub unsafe fn lenum(l: *const TValue, r: *const TValue) -> bool {
    unsafe {
        if let Some(li) = (*l).as_integer() {
            if let Some(ri) = (*r).as_integer() {
                li <= ri
            } else {
                leintfloat(li, (*r).as_number().unwrap())
            }
        } else {
            let lf: f64 = (*l).as_number().unwrap();
            if let Some(rn) = (*r).as_number() {
                lf <= rn
            } else {
                lefloatint(lf, (*r).as_integer().unwrap())
            }
        }
    }
}
pub unsafe fn luav_idiv(state: *mut State, m: i64, n: i64) -> i64 {
    unsafe {
        if (n as usize).wrapping_add(1_usize) <= 1 {
            if n == 0 {
                luag_runerror(state, c"attempt to divide by zero".as_ptr(), &[]);
            }
            (0usize).wrapping_sub(m as usize) as i64
        } else {
            let mut q: i64 = m / n;
            if m ^ n < 0 && m % n != 0 {
                q -= 1;
            }
            q
        }
    }
}
pub unsafe fn luav_mod(state: *mut State, m: i64, n: i64) -> i64 {
    unsafe {
        if (n as usize).wrapping_add(1_usize) <= 1 {
            if n == 0 {
                luag_runerror(state, c"attempt to perform 'n%%0'".as_ptr(), &[]);
            }
            0
        } else {
            let mut r: i64 = m % n;
            if r != 0 && r ^ n < 0 {
                r += n;
            }
            r
        }
    }
}
pub unsafe fn luav_modf(mut _state: *mut State, m: f64, n: f64) -> f64 {
    let mut r: f64 = m % n;
    if if r > 0.0 {
        (n < 0.0) as i32
    } else {
        (r < 0.0 && n > 0.0) as i32
    } != 0
    {
        r += n;
    }
    r
}
pub unsafe fn luav_shiftl(x: i64, y: i64) -> i64 {
    if y < 0 {
        if y <= -(i64::BITS as i32) as i64 {
            0
        } else {
            (x as usize >> -y as usize) as i64
        }
    } else if y >= i64::BITS as i64 {
        0
    } else {
        ((x as usize) << y as usize) as i64
    }
}
pub unsafe fn b_str2int(mut s: *const i8, base: i32, pn: *mut i64) -> *const i8 {
    unsafe {
        let mut n: usize = 0;
        let mut is_negative: i32 = 0;
        s = s.add(cstr_span(s, WHITESPACE.as_ptr()));
        if *s as i32 == Character::Hyphen as i32 {
            s = s.add(1);
            is_negative = 1;
        } else if *s as i32 == Character::Plus as i32 {
            s = s.add(1);
        }
        if !Character::from(*s as i32).is_alphanumeric() {
            return null();
        }
        loop {
            let digit: i32 = if Character::from(*s as u8 as i32).is_digit_decimal() {
                *s as i32 - Character::Digit0 as i32
            } else {
                (*s as u8).to_ascii_uppercase() as i32 - Character::UpperA as i32 + HEX_DIGIT_OFFSET
            };
            if digit >= base {
                return null();
            }
            n = n * base as usize + digit as usize;
            s = s.add(1);
            if !Character::from(*s as i32).is_alphanumeric() {
                break;
            }
        }
        s = s.add(cstr_span(s, WHITESPACE.as_ptr()));
        *pn = (if is_negative != 0 {
            (0usize).wrapping_sub(n)
        } else {
            n
        }) as i64;
        s
    }
}
