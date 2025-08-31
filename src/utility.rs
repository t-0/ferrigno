use crate::character::*;
use libc::*;
pub fn ceiling_log2(input: u64) -> u64 {
    const BITS: u64 = 8;
    const LOG2: [u8; 256] = [
        0, 1, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
        5, 5, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
        6, 6, 6, 6, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
        7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
        7, 7, 7, 7, 7, 7, 7, 7, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
    ];
    let mut accumulator: u64 = 0;
    let mut remainder: u64 = input.wrapping_sub(1);
    while remainder >= 256 {
        accumulator += BITS;
        remainder >>= BITS;
    }
    return accumulator + (LOG2[remainder as usize] as u64);
}
pub unsafe extern "C" fn is_negative(s: *mut *const i8) -> bool {
    unsafe {
        if **s as i32 == '-' as i32 {
            *s = (*s).offset(1);
            return true;
        } else {
            if **s as i32 == '+' as i32 {
                *s = (*s).offset(1);
            }
            return false;
        }
    }
}
pub unsafe extern "C" fn l_str2dloc(s: *const i8, result: *mut f64, mode: i32) -> *const i8 {
    unsafe {
        let mut endptr: *mut i8 = std::ptr::null_mut();
        *result = if mode == 'x' as i32 {
            strtod(s, &mut endptr)
        } else {
            strtod(s, &mut endptr)
        };
        if endptr == s as *mut i8 {
            return std::ptr::null();
        }
        while is_whitespace(*endptr as i32 + 1) {
            endptr = endptr.offset(1);
        }
        return if *endptr as i32 == '\0' as i32 {
            endptr
        } else {
            std::ptr::null_mut()
        };
    }
}
pub unsafe extern "C" fn l_str2d(s: *const i8, result: *mut f64) -> *const i8 {
    unsafe {
        let pmode: *const i8 = strpbrk(s, b".xXnN\0" as *const u8 as *const i8);
        let mode: i32 = if !pmode.is_null() {
            *pmode as u8 as i32 | 'A' as i32 ^ 'a' as i32
        } else {
            0
        };
        if mode == 'n' as i32 {
            return std::ptr::null();
        }
        let mut endptr: *const i8 = l_str2dloc(s, result, mode);
        if endptr.is_null() {
            let mut buffer: [i8; 201] = [0; 201];
            let pdot: *const i8 = strchr(s, '.' as i32);
            if pdot.is_null() || strlen(s) > 200 {
                return std::ptr::null();
            }
            strcpy(buffer.as_mut_ptr(), s);
            buffer[pdot.offset_from(s) as usize] = '.' as i8;
            endptr = l_str2dloc(buffer.as_mut_ptr(), result, mode);
            if !endptr.is_null() {
                endptr = s.offset(endptr.offset_from(buffer.as_mut_ptr()) as isize);
            }
        }
        return endptr;
    }
}
pub unsafe extern "C" fn l_str2int(mut s: *const i8, result: *mut i64) -> *const i8 {
    unsafe {
        let mut a: u64 = 0;
        let mut empty: i32 = 1;
        while is_whitespace(*s as i32 + 1) {
            s = s.offset(1);
        }
        let is_negative_: bool = is_negative(&mut s);
        if *s.offset(0 as isize) as i32 == '0' as i32
            && (*s.offset(1 as isize) as i32 == 'x' as i32
                || *s.offset(1 as isize) as i32 == 'X' as i32)
        {
            s = s.offset(2 as isize);
            while is_digit_hexadecimal(*s as i32 + 1) {
                a = a
                    .wrapping_mul(16 as u64)
                    .wrapping_add(get_hexadecimal_digit_value(*s as i32) as u64);
                empty = 0;
                s = s.offset(1);
            }
        } else {
            while is_digit_decimal(*s as i32 + 1) {
                let d: i32 = *s as i32 - '0' as i32;
                if a >= (0x7FFFFFFFFFFFFFFF as i64 / 10 as i64) as u64
                    && (a > (0x7FFFFFFFFFFFFFFF as i64 / 10 as i64) as u64
                        || d > (0x7FFFFFFFFFFFFFFF as i64 % 10 as i64) as i32
                            + if is_negative_ { 1 } else { 0 })
                {
                    return std::ptr::null();
                }
                a = a.wrapping_mul(10 as u64).wrapping_add(d as u64);
                empty = 0;
                s = s.offset(1);
            }
        }
        while is_whitespace(*s as i32 + 1) {
            s = s.offset(1);
        }
        if empty != 0 || *s as i32 != '\0' as i32 {
            return std::ptr::null();
        } else {
            *result = (if is_negative_ {
                (0u64).wrapping_sub(a)
            } else {
                a
            }) as i64;
            return s;
        };
    }
}
pub unsafe extern "C" fn luao_chunkid(mut out: *mut i8, source: *const i8, mut source_length: u64) {
    unsafe {
        let mut bufflen: u64 = 60 as u64;
        if *source as i32 == '=' as i32 {
            if source_length <= bufflen {
                memcpy(
                    out as *mut libc::c_void,
                    source.offset(1 as isize) as *const libc::c_void,
                    (source_length as usize).wrapping_mul(::core::mem::size_of::<i8>()),
                );
            } else {
                memcpy(
                    out as *mut libc::c_void,
                    source.offset(1 as isize) as *const libc::c_void,
                    (bufflen as usize)
                        .wrapping_sub(1)
                        .wrapping_mul(::core::mem::size_of::<i8>()),
                );
                out = out.offset(bufflen.wrapping_sub(1 as u64) as isize);
                *out = '\0' as i8;
            }
        } else if *source as i32 == '@' as i32 {
            if source_length <= bufflen {
                memcpy(
                    out as *mut libc::c_void,
                    source.offset(1 as isize) as *const libc::c_void,
                    (source_length as usize).wrapping_mul(::core::mem::size_of::<i8>()),
                );
            } else {
                memcpy(
                    out as *mut libc::c_void,
                    b"...\0" as *const u8 as *const i8 as *const libc::c_void,
                    (::core::mem::size_of::<[i8; 4]>())
                        .wrapping_div(::core::mem::size_of::<i8>())
                        .wrapping_sub(1)
                        .wrapping_mul(::core::mem::size_of::<i8>()),
                );
                out = out.offset(
                    (::core::mem::size_of::<[i8; 4]>() as u64)
                        .wrapping_div(::core::mem::size_of::<i8>() as u64)
                        .wrapping_sub(1 as u64) as isize,
                );
                bufflen = (bufflen as u64).wrapping_sub(
                    (::core::mem::size_of::<[i8; 4]>() as u64)
                        .wrapping_div(::core::mem::size_of::<i8>() as u64)
                        .wrapping_sub(1 as u64),
                ) as u64;
                memcpy(
                    out as *mut libc::c_void,
                    source
                        .offset(1 as isize)
                        .offset(source_length as isize)
                        .offset(-(bufflen as isize)) as *const libc::c_void,
                    (bufflen as usize).wrapping_mul(::core::mem::size_of::<i8>()),
                );
            }
        } else {
            let nl: *const i8 = strchr(source, '\n' as i32);
            memcpy(
                out as *mut libc::c_void,
                b"[string \"\0" as *const u8 as *const i8 as *const libc::c_void,
                (::core::mem::size_of::<[i8; 10]>())
                    .wrapping_div(::core::mem::size_of::<i8>())
                    .wrapping_sub(1)
                    .wrapping_mul(::core::mem::size_of::<i8>()),
            );
            out = out.offset(
                (::core::mem::size_of::<[i8; 10]>() as u64)
                    .wrapping_div(::core::mem::size_of::<i8>() as u64)
                    .wrapping_sub(1 as u64) as isize,
            );
            bufflen = (bufflen as u64).wrapping_sub(
                (::core::mem::size_of::<[i8; 15]>() as u64)
                    .wrapping_div(::core::mem::size_of::<i8>() as u64)
                    .wrapping_sub(1 as u64)
                    .wrapping_add(1 as u64),
            ) as u64;
            if source_length < bufflen && nl.is_null() {
                memcpy(
                    out as *mut libc::c_void,
                    source as *const libc::c_void,
                    (source_length as usize).wrapping_mul(::core::mem::size_of::<i8>()),
                );
                out = out.offset(source_length as isize);
            } else {
                if !nl.is_null() {
                    source_length = nl.offset_from(source) as u64;
                }
                if source_length > bufflen {
                    source_length = bufflen;
                }
                memcpy(
                    out as *mut libc::c_void,
                    source as *const libc::c_void,
                    (source_length as usize).wrapping_mul(::core::mem::size_of::<i8>())
                );
                out = out.offset(source_length as isize);
                memcpy(
                    out as *mut libc::c_void,
                    b"...\0" as *const u8 as *const i8 as *const libc::c_void,
                    (::core::mem::size_of::<[i8; 4]>())
                        .wrapping_div(::core::mem::size_of::<i8>())
                        .wrapping_sub(1)
                        .wrapping_mul(::core::mem::size_of::<i8>()),
                );
                out = out.offset(
                    (::core::mem::size_of::<[i8; 4]>() as u64)
                        .wrapping_div(::core::mem::size_of::<i8>() as u64)
                        .wrapping_sub(1 as u64) as isize,
                );
            }
            memcpy(
                out as *mut libc::c_void,
                b"\"]\0" as *const u8 as *const i8 as *const libc::c_void,
                (::core::mem::size_of::<[i8; 3]>())
                    .wrapping_div(::core::mem::size_of::<i8>())
                    .wrapping_sub(1)
                    .wrapping_add(1)
                    .wrapping_mul(::core::mem::size_of::<i8>()),
            );
        };
    }
}
