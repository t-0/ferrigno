use crate::character::*;
use libc::*;
use rlua::*;
use std::ptr::*;
pub const MAXIMUM_SIZE: usize = 0x7FFFFFFFFFFFFFFF;
pub fn ceiling_log2(input: usize) -> usize {
    if input == 0 {
        return 0;
    } else {
        return 1 + input.ilog2() as usize;
    }
}
pub unsafe fn is_negative(s: *mut *const i8) -> bool {
    unsafe {
        if **s as i32 == Character::Hyphen as i32 {
            *s = (*s).offset(1);
            return true;
        } else {
            if **s as i32 == Character::Plus as i32 {
                *s = (*s).offset(1);
            }
            return false;
        }
    }
}
pub unsafe fn l_str2dloc(s: *const i8, result: *mut f64, mode: i32) -> *const i8 {
    unsafe {
        let mut endptr: *mut i8 = null_mut();
        *result = if mode == Character::LowerX as i32 {
            strtod(s, &mut endptr)
        } else {
            strtod(s, &mut endptr)
        };
        if endptr == s as *mut i8 {
            return null();
        }
        while Character::from(*endptr as i32).is_whitespace() {
            endptr = endptr.offset(1);
        }
        return if *endptr as i32 == Character::Null as i32 { endptr } else { null_mut() };
    }
}
pub unsafe fn l_str2d(s: *const i8, result: *mut f64) -> *const i8 {
    unsafe {
        let pmode: *const i8 = libc::strpbrk(s, c".xXnN".as_ptr());
        let mode: i32 = if !pmode.is_null() {
            *pmode as u8 as i32 | Character::UpperA as i32 ^ Character::LowerA as i32
        } else {
            0
        };
        if mode == Character::LowerN as i32 {
            return null();
        }
        let mut endptr: *const i8 = l_str2dloc(s, result, mode);
        if endptr.is_null() {
            let mut buffer: [i8; 201] = [0; 201];
            let pdot: *const i8 = libc::strchr(s, Character::Period as i32);
            if pdot.is_null() || libc::strlen(s) > 200 {
                return null();
            }
            libc::strcpy(buffer.as_mut_ptr(), s);
            buffer[pdot.offset_from(s) as usize] = Character::Period as i8;
            endptr = l_str2dloc(buffer.as_mut_ptr(), result, mode);
            if !endptr.is_null() {
                endptr = s.offset(endptr.offset_from(buffer.as_mut_ptr()) as isize);
            }
        }
        return endptr;
    }
}
pub unsafe fn l_str2int(mut s: *const i8, result: *mut i64) -> *const i8 {
    unsafe {
        let mut a: usize = 0;
        let mut empty: i32 = 1;
        while Character::from(*s as i32).is_whitespace() {
            s = s.offset(1);
        }
        let is_negative_: bool = is_negative(&mut s);
        if *s.offset(0 as isize) as i32 == Character::Digit0 as i32
            && (*s.offset(1 as isize) as i32 == Character::LowerX as i32
                || *s.offset(1 as isize) as i32 == Character::UpperX as i32)
        {
            s = s.offset(2 as isize);
            while Character::from(*s as i32).is_digit_hexadecimal() {
                a = a
                    .wrapping_mul(16 as usize)
                    .wrapping_add(Character::from(*s as i32).get_hexadecimal_digit_value() as usize);
                empty = 0;
                s = s.offset(1);
            }
        } else {
            while Character::from(*s as i32).is_digit_decimal() {
                let d: i32 = *s as i32 - Character::Digit0 as i32;
                if a >= (MAXIMUM_SIZE as i64 / 10 as i64) as usize
                    && (a > (MAXIMUM_SIZE as i64 / 10 as i64) as usize
                        || d > (MAXIMUM_SIZE as i64 % 10 as i64) as i32 + if is_negative_ { 1 } else { 0 })
                {
                    return null();
                }
                a = a.wrapping_mul(10 as usize).wrapping_add(d as usize);
                empty = 0;
                s = s.offset(1);
            }
        }
        while Character::from(*s as i32).is_whitespace() {
            s = s.offset(1);
        }
        if empty != 0 || *s as i32 != Character::Null as i32 {
            return null();
        } else {
            *result = (if is_negative_ { (0usize).wrapping_sub(a) } else { a }) as i64;
            return s;
        };
    }
}
pub unsafe fn luao_chunkid(mut out: *mut i8, source: *const i8, mut source_length: usize) {
    unsafe {
        let mut bufflen: usize = 60 as usize;
        if *source as i32 == Character::Equal as i32 {
            if source_length <= bufflen {
                libc::memcpy(
                    out as *mut libc::c_void,
                    source.offset(1 as isize) as *const libc::c_void,
                    source_length,
                );
            } else {
                libc::memcpy(
                    out as *mut libc::c_void,
                    source.offset(1 as isize) as *const libc::c_void,
                    (bufflen as usize).wrapping_sub(1),
                );
                out = out.offset(bufflen.wrapping_sub(1 as usize) as isize);
                *out = Character::Null as i8;
            }
        } else if *source as i32 == Character::At as i32 {
            if source_length <= bufflen {
                libc::memcpy(out as *mut libc::c_void, source.offset(1) as *const libc::c_void, source_length);
            } else {
                libc::memcpy(
                    out as *mut libc::c_void,
                    c"...".as_ptr() as *const libc::c_void,
                    (size_of::<[i8; 4]>()).wrapping_sub(1),
                );
                out = out.offset((size_of::<[i8; 4]>() as usize).wrapping_sub(1 as usize) as isize);
                bufflen = (bufflen as usize).wrapping_sub((size_of::<[i8; 4]>() as usize).wrapping_sub(1 as usize)) as usize;
                libc::memcpy(
                    out as *mut libc::c_void,
                    source
                        .offset(1 as isize)
                        .offset(source_length as isize)
                        .offset(-(bufflen as isize)) as *const libc::c_void,
                    bufflen,
                );
            }
        } else {
            let nl = libc::strchr(source, Character::LineFeed as i32);
            libc::memcpy(
                out as *mut libc::c_void,
                b"[string \"".as_ptr() as *const libc::c_void,
                (size_of::<[i8; 10]>()).wrapping_sub(1),
            );
            out = out.offset((size_of::<[i8; 10]>() as usize).wrapping_sub(1 as usize) as isize);
            bufflen = (bufflen as usize).wrapping_sub(
                (size_of::<[i8; 15]>() as usize)
                    .wrapping_sub(1 as usize)
                    .wrapping_add(1 as usize),
            ) as usize;
            if source_length < bufflen && nl.is_null() {
                libc::memcpy(out as *mut libc::c_void, source as *const libc::c_void, source_length);
                out = out.offset(source_length as isize);
            } else {
                if !nl.is_null() {
                    source_length = nl.offset_from(source) as usize;
                }
                if source_length > bufflen {
                    source_length = bufflen;
                }
                libc::memcpy(out as *mut libc::c_void, source as *const libc::c_void, source_length);
                out = out.offset(source_length as isize);
                libc::memcpy(
                    out as *mut libc::c_void,
                    c"...".as_ptr() as *const libc::c_void,
                    (size_of::<[i8; 4]>()).wrapping_sub(1),
                );
                out = out.offset((size_of::<[i8; 4]>() as usize).wrapping_sub(1 as usize) as isize);
            }
            libc::memcpy(
                out as *mut libc::c_void,
                make_cstring!("\"]") as *const libc::c_void,
                (size_of::<[i8; 3]>()).wrapping_sub(1).wrapping_add(1),
            );
        };
    }
}
pub fn frexp_(x: f64) -> (f64, i32) {
    if x == 0.0 {
        return (0.0, 0);
    } else {
        let bits = x.to_bits();
        let sign = if (bits >> 63) != 0 { -1.0 } else { 1.0 };
        let exponent = ((bits >> 52) & 0x7ff) as i32 - 1023;
        let mantissa = sign * f64::from_bits((bits & 0xfffffffffffff) | 0x3fe0000000000000);
        return (mantissa, exponent + 1);
    }
}
pub fn ldexp_(x: f64, exp: i32) -> f64 {
    if x == 0.0 || exp == 0 {
        return x;
    } else {
        let bits = x.to_bits();
        let exponent = ((bits >> 52) & 0x7ff) as i32;
        let new_exponent = exponent + exp;
        if !(0..=0x7ff).contains(&new_exponent) {
            return if (bits >> 63) != 0 { f64::NEG_INFINITY } else { f64::INFINITY };
        } else {
            let result_bits = (bits & 0x800fffffffffffff) | ((new_exponent as u64) << 52);
            return f64::from_bits(result_bits);
        }
    }
}
pub unsafe fn l_hashfloat(mut n: f64) -> i32 {
    let i: i32;
    let mut ni: i64 = 0;
    (n, i) = frexp_(n);
    n = n * -((-(0x7FFFFFFF as i32) - 1) as f64);
    if !(n >= (-(MAXIMUM_SIZE as i64) - 1 as i64) as f64 && n < -((-(MAXIMUM_SIZE as i64) - 1 as i64) as f64) && {
        ni = n as i64;
        1 != 0
    }) {
        return 0;
    } else {
        let u: u32 = (i as u32).wrapping_add(ni as u32);
        return (if u <= 0x7FFFFFFF as u32 { u } else { !u }) as i32;
    };
}
pub unsafe fn fits_c(i: i64) -> bool {
    return (i as usize).wrapping_add(((1 << 8) - 1 >> 1) as usize) <= ((1 << 8) - 1) as usize;
}
pub unsafe fn fits_bx(i: i64) -> bool {
    return -((1 << 8 + 8 + 1) - 1 >> 1) as i64 <= i && i <= ((1 << 8 + 8 + 1) - 1 - ((1 << 8 + 8 + 1) - 1 >> 1)) as i64;
}
pub unsafe fn getnum(fmt: *mut *const i8, df: i32) -> i32 {
    unsafe {
        if Character::from(**fmt as i32).is_digit_decimal() {
            let mut a: i32 = 0;
            loop {
                let fresh179 = *fmt;
                *fmt = (*fmt).offset(1);
                a = a * 10 as i32 + (*fresh179 as i32 - Character::Digit0 as i32);
                if !(Character::from(**fmt as i32).is_digit_decimal()
                    && a <= ((if (size_of::<usize>() as usize) < size_of::<i32>() as usize {
                        !0usize
                    } else {
                        0x7FFFFFFF as usize
                    }) as i32
                        - 9 as i32)
                        / 10 as i32)
                {
                    break;
                }
            }
            return a;
        } else {
            return df;
        };
    }
}
