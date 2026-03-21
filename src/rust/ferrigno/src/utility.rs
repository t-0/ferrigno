use crate::character::*;
use crate::functionstate::LUA_IDSIZE;
/// Parse a hex float string like "1.8p+1" or "FF" (without the "0x" prefix).
/// The sign and "0x" should already be stripped.
fn parse_hex_float_value(s: &str) -> f64 {
    let (mantissa_str, exp_str) = match s.find(['p', 'P']) {
        Some(p) => (&s[..p], &s[p + 1..]),
        None => (s, "0"),
    };
    let (int_str, frac_str) = match mantissa_str.find('.') {
        Some(d) => (&mantissa_str[..d], &mantissa_str[d + 1..]),
        None => (mantissa_str, ""),
    };
    let explicit_exp: i64 = exp_str.parse().unwrap_or(0);
    // Accumulate up to 15 significant hex digits (60 bits > 53-bit mantissa).
    // Track excess integer digits and consumed fractional digits for exponent adjustment.
    const MAX_SIG: usize = 15;
    let mut value: f64 = 0.0;
    let mut sig_count: usize = 0;
    let mut extra_int: i64 = 0;
    for c in int_str.chars() {
        let d = c.to_digit(16).unwrap_or(0);
        if sig_count < MAX_SIG {
            value = value * 16.0 + d as f64;
            if value != 0.0 || d != 0 {
                sig_count += 1;
            }
        } else {
            extra_int += 1;
        }
    }
    let mut frac_consumed: i64 = 0;
    for c in frac_str.chars() {
        let d = c.to_digit(16).unwrap_or(0);
        if sig_count < MAX_SIG {
            value = value * 16.0 + d as f64;
            frac_consumed += 1;
            if value != 0.0 || d != 0 {
                sig_count += 1;
            }
        }
    }
    if value == 0.0 {
        return 0.0;
    }
    // result = value * 2^(4*extra_int - 4*frac_consumed + explicit_exp)
    let total_exp = (4 * extra_int - 4 * frac_consumed + explicit_exp).clamp(i32::MIN as i64, i32::MAX as i64) as i32;
    value * (2.0f64).powi(total_exp)
}

/// Rust replacement for C's strtod. Parses a float from a C string.
/// Returns (value, endptr) where endptr points past the last consumed char.
/// If nothing was parsed, endptr == s.
unsafe fn rust_strtod(s: *const i8, endptr: *mut *mut i8) -> f64 {
    unsafe {
        let mut p = s;

        // Skip leading whitespace
        while Character::from(*p as i32).is_whitespace() {
            p = p.add(1);
        }
        let start = p;

        // Optional sign
        let neg = *p as u8 == b'-';
        if *p as u8 == b'+' || *p as u8 == b'-' {
            p = p.add(1);
        }

        // Detect and scan the number
        if *p as u8 == b'0' && (*p.add(1) as u8 | 0x20) == b'x' {
            // Hex float: 0x[hexdigits][.hexdigits][p[+-]digits]
            p = p.add(2);
            let hex_start = p;
            while (*p as u8).is_ascii_hexdigit() {
                p = p.add(1);
            }
            let has_dot = *p as u8 == b'.';
            if has_dot {
                p = p.add(1);
                while (*p as u8).is_ascii_hexdigit() {
                    p = p.add(1);
                }
            }
            // Must have at least one hex digit
            let digits_end = p;
            let digit_count = if has_dot {
                digits_end.offset_from(hex_start) as usize - 1
            } else {
                digits_end.offset_from(hex_start) as usize
            };
            if digit_count == 0 {
                *endptr = s as *mut i8;
                return 0.0;
            }
            if (*p as u8 | 0x20) == b'p' {
                let e = p;
                p = p.add(1);
                if *p as u8 == b'+' || *p as u8 == b'-' {
                    p = p.add(1);
                }
                if (*p as u8).is_ascii_digit() {
                    while (*p as u8).is_ascii_digit() {
                        p = p.add(1);
                    }
                } else {
                    p = e; // 'p' not followed by digits — not part of the number
                }
            }
            // Parse the hex float from the scanned substring (after "0x")
            let hex_len = p.offset_from(hex_start) as usize;
            let hex_bytes = std::slice::from_raw_parts(hex_start as *const u8, hex_len);
            let hex_str = std::str::from_utf8(hex_bytes).unwrap_or("");
            let mut value = parse_hex_float_value(hex_str);
            if neg {
                value = -value;
            }
            *endptr = p as *mut i8;
            value
        } else if (*p as u8 | 0x20) == b'i' {
            // inf / infinity (case-insensitive)
            let at = |off: usize| (*p.add(off) as u8) | 0x20;
            if at(0) == b'i' && at(1) == b'n' && at(2) == b'f' {
                if at(3) == b'i' && at(4) == b'n' && at(5) == b'i' && at(6) == b't' && at(7) == b'y' {
                    p = p.add(8);
                } else {
                    p = p.add(3);
                }
                *endptr = p as *mut i8;
                if neg {
                    f64::NEG_INFINITY
                } else {
                    f64::INFINITY
                }
            } else {
                *endptr = s as *mut i8;
                0.0
            }
        } else {
            // Decimal float: [digits][.digits][e/E[+-]digits]
            let mut has_digit = false;
            while (*p as u8).is_ascii_digit() {
                has_digit = true;
                p = p.add(1);
            }
            if *p as u8 == b'.' {
                p = p.add(1);
                while (*p as u8).is_ascii_digit() {
                    has_digit = true;
                    p = p.add(1);
                }
            }
            if !has_digit {
                *endptr = s as *mut i8;
                return 0.0;
            }
            if (*p as u8 | 0x20) == b'e' {
                let e = p;
                p = p.add(1);
                if *p as u8 == b'+' || *p as u8 == b'-' {
                    p = p.add(1);
                }
                if (*p as u8).is_ascii_digit() {
                    while (*p as u8).is_ascii_digit() {
                        p = p.add(1);
                    }
                } else {
                    p = e; // 'e' not followed by digits — not part of the number
                }
            }
            let len = p.offset_from(start) as usize;
            let bytes = std::slice::from_raw_parts(start as *const u8, len);
            let num_str = std::str::from_utf8(bytes).unwrap_or("");
            let value = num_str.parse::<f64>().unwrap_or(0.0);
            *endptr = p as *mut i8;
            value
        }
    }
}
/// Minimal representation of C's `struct lconv`, only the first field is needed.
#[repr(C)]
pub struct LocaleConv {
    pub decimal_point: *mut i8,
}
unsafe extern "C" {
    pub fn localeconv() -> *mut LocaleConv;
}
use std::ptr::*;
pub const MAXIMUM_SIZE: usize = 0x7FFFFFFFFFFFFFFF;
pub unsafe fn cstr_chr(s: *const i8, c: i8) -> *const i8 {
    unsafe {
        let mut p = s;
        loop {
            if *p == c {
                return p;
            }
            if *p == 0 {
                return null();
            }
            p = p.add(1);
        }
    }
}
pub unsafe fn cstr_len(s: *const i8) -> usize {
    unsafe {
        let mut n = 0usize;
        while *s.add(n) != 0 {
            n += 1;
        }
        n
    }
}
pub unsafe fn cstr_span(s: *const i8, accept: *const i8) -> usize {
    unsafe {
        let mut count = 0usize;
        'outer: loop {
            let ch = *s.add(count);
            if ch == 0 {
                break;
            }
            let mut a = accept;
            while *a != 0 {
                if *a == ch {
                    count += 1;
                    continue 'outer;
                }
                a = a.add(1);
            }
            break;
        }
        count
    }
}
pub unsafe fn cstr_pbrk(s: *const i8, accept: *const i8) -> *const i8 {
    unsafe {
        let mut p = s;
        while *p != 0 {
            let mut a = accept;
            while *a != 0 {
                if *p == *a {
                    return p;
                }
                a = a.add(1);
            }
            p = p.add(1);
        }
        null()
    }
}
pub unsafe fn cstr_str(haystack: *const i8, needle: *const i8) -> *const i8 {
    unsafe {
        let nlen = cstr_len(needle);
        if nlen == 0 {
            return haystack;
        }
        let hlen = cstr_len(haystack);
        if nlen > hlen {
            return null();
        }
        for i in 0..=(hlen - nlen) {
            if std::slice::from_raw_parts(haystack.add(i) as *const u8, nlen)
                == std::slice::from_raw_parts(needle as *const u8, nlen)
            {
                return haystack.add(i);
            }
        }
        null()
    }
}
pub unsafe fn mem_chr(s: *const u8, c: u8, n: usize) -> *const u8 {
    unsafe {
        let slice = std::slice::from_raw_parts(s, n);
        match slice.iter().position(|&b| b == c) {
            Some(i) => s.add(i),
            None => null(),
        }
    }
}
/// Returns a pointer to a thread-local buffer containing the error message for `errno`.
/// The pointer is valid until the next call to `os_strerror` on the same thread.
pub fn os_strerror(errno: i32) -> *const i8 {
    use std::io::Write;
    thread_local! {
        static BUF: std::cell::RefCell<Vec<u8>> = std::cell::RefCell::new(Vec::with_capacity(128));
    }
    BUF.with(|buf| {
        let mut buf = buf.borrow_mut();
        buf.clear();
        let err = std::io::Error::from_raw_os_error(errno);
        let _ = write!(buf, "{}\0", err);
        buf.as_ptr() as *const i8
    })
}
/// Returns the current Unix timestamp in seconds since epoch.
pub fn os_time_now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}
/// Looks up an environment variable by C string name.
/// Returns a pointer to the value as a C string, or null if not found.
/// The returned pointer is valid until the next call to `os_getenv` on the same thread.
pub unsafe fn os_getenv(name: *const i8) -> *const i8 {
    thread_local! {
        static BUF: std::cell::RefCell<Vec<u8>> = std::cell::RefCell::new(Vec::with_capacity(256));
    }
    unsafe {
        let cname = std::ffi::CStr::from_ptr(name);
        use std::os::unix::ffi::OsStrExt;
        let key = std::ffi::OsStr::from_bytes(cname.to_bytes());
        match std::env::var_os(key) {
            Some(val) => BUF.with(|buf| {
                let mut buf = buf.borrow_mut();
                buf.clear();
                buf.extend_from_slice(val.as_encoded_bytes());
                buf.push(0);
                buf.as_ptr() as *const i8
            }),
            None => null(),
        }
    }
}
/// Formats a pointer as "0x..." into a C string buffer, like snprintf(buf, size, "%p", p).
/// Returns the number of bytes written (not including the null terminator).
pub unsafe fn snprintf_pointer(buf: *mut i8, size: usize, p: *const std::ffi::c_void) -> i32 {
    unsafe {
        use std::io::Write;
        let mut tmp = [0u8; 64];
        let len = {
            let mut cursor = std::io::Cursor::new(&mut tmp[..]);
            if p.is_null() {
                let _ = write!(cursor, "0x0");
            } else {
                let _ = write!(cursor, "{:p}", p);
            }
            cursor.position() as usize
        };
        let copy_len = len.min(size.saturating_sub(1));
        if size > 0 {
            std::ptr::copy_nonoverlapping(tmp.as_ptr(), buf as *mut u8, copy_len);
            *buf.add(copy_len) = 0;
        }
        len as i32
    }
}
pub fn ceiling_log2(input: usize) -> usize {
    if input <= 1 {
        0
    } else {
        (input - 1).ilog2() as usize + 1
    }
}
pub unsafe fn is_negative(s: *mut *const i8) -> bool {
    unsafe {
        if **s as i32 == Character::Hyphen as i32 {
            *s = (*s).add(1);
            true
        } else {
            if **s as i32 == Character::Plus as i32 {
                *s = (*s).add(1);
            }
            false
        }
    }
}
pub unsafe fn l_str2dloc(s: *const i8, result: *mut f64, _mode: i32) -> *const i8 {
    unsafe {
        let mut endptr: *mut i8 = null_mut();
        *result = rust_strtod(s, &mut endptr);
        if std::ptr::eq(endptr, s) {
            return null();
        }
        while Character::from(*endptr as i32).is_whitespace() {
            endptr = endptr.add(1);
        }
        if *endptr as i32 == Character::Null as i32 {
            endptr
        } else {
            null_mut()
        }
    }
}
pub unsafe fn l_str2d(s: *const i8, result: *mut f64) -> *const i8 {
    unsafe {
        let pmode: *const i8 = cstr_pbrk(s, c".xXnN".as_ptr());
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
            // rust_strtod always uses '.'; if locale uses a different decimal
            // separator (e.g. ','), replace it with '.' and retry
            let lc = localeconv();
            let locale_dec = if !lc.is_null() && !(*lc).decimal_point.is_null() {
                *(*lc).decimal_point
            } else {
                Character::Period as i8
            };
            if locale_dec != Character::Period as i8 {
                let pdot: *const i8 = cstr_chr(s, locale_dec);
                if pdot.is_null() || cstr_len(s) > 200 {
                    return null();
                }
                let mut buffer: [i8; 201] = [0; 201];
                let src = std::ffi::CStr::from_ptr(s).to_bytes_with_nul();
                std::ptr::copy_nonoverlapping(src.as_ptr(), buffer.as_mut_ptr() as *mut u8, src.len());
                buffer[pdot.offset_from(s) as usize] = Character::Period as i8;
                endptr = l_str2dloc(buffer.as_mut_ptr(), result, mode);
                if !endptr.is_null() {
                    endptr = s.add(endptr.offset_from(buffer.as_mut_ptr()) as usize);
                }
            }
        }
        endptr
    }
}
pub unsafe fn l_str2int(mut s: *const i8, result: *mut i64) -> *const i8 {
    unsafe {
        let mut a: usize = 0;
        let mut empty: i32 = 1;
        while Character::from(*s as i32).is_whitespace() {
            s = s.add(1);
        }
        let is_negative_: bool = is_negative(&mut s);
        if *s as i32 == Character::Digit0 as i32
            && (*s.add(1) as i32 == Character::LowerX as i32 || *s.add(1) as i32 == Character::UpperX as i32)
        {
            s = s.add(2);
            while Character::from(*s as i32).is_digit_hexadecimal() {
                a = a
                    .wrapping_mul(16_usize)
                    .wrapping_add(Character::from(*s as i32).get_hexadecimal_digit_value() as usize);
                empty = 0;
                s = s.add(1);
            }
        } else {
            while Character::from(*s as i32).is_digit_decimal() {
                let d: i32 = *s as i32 - Character::Digit0 as i32;
                if a >= (MAXIMUM_SIZE as i64 / 10_i64) as usize
                    && (a > (MAXIMUM_SIZE as i64 / 10_i64) as usize
                        || d > (MAXIMUM_SIZE as i64 % 10_i64) as i32 + if is_negative_ { 1 } else { 0 })
                {
                    return null();
                }
                a = a.wrapping_mul(10_usize).wrapping_add(d as usize);
                empty = 0;
                s = s.add(1);
            }
        }
        while Character::from(*s as i32).is_whitespace() {
            s = s.add(1);
        }
        if empty != 0 || *s as i32 != Character::Null as i32 {
            null()
        } else {
            *result = (if is_negative_ {
                (0usize).wrapping_sub(a)
            } else {
                a
            }) as i64;
            s
        }
    }
}
pub unsafe fn luao_chunkid(out: *mut i8, source: *const i8, mut source_length: usize) {
    unsafe {
        let mut bufflen: usize = LUA_IDSIZE;
        // Include the null terminator at source[source_length] — the original C code
        // copies source_length bytes from source+1, which reaches into the null byte.
        let src = std::slice::from_raw_parts(source as *const u8, source_length + 1);
        let mut pos: usize = 0;
        let write = |out: *mut i8, pos: &mut usize, data: &[u8]| {
            let n = data.len();
            std::ptr::copy_nonoverlapping(data.as_ptr(), out.add(*pos) as *mut u8, n);
            *pos += n;
        };
        if src[0] == 0x3D {
            // source starts with '='
            if source_length <= bufflen {
                write(out, &mut pos, &src[1..source_length + 1]);
            } else {
                write(out, &mut pos, &src[1..bufflen]);
                write(out, &mut pos, &[Character::Null as u8]);
            }
        } else if src[0] == 0x40 {
            // source starts with '@'
            if source_length <= bufflen {
                write(out, &mut pos, &src[1..source_length + 1]);
            } else {
                let dots_len = size_of::<[i8; 4]>() - 1;
                write(out, &mut pos, b"...");
                bufflen -= dots_len;
                let tail_start = 1 + source_length - bufflen;
                write(out, &mut pos, &src[tail_start..tail_start + bufflen]);
            }
        } else {
            let nl = cstr_chr(source, Character::LineFeed as i8);
            write(out, &mut pos, b"[string \"");
            bufflen -= size_of::<[i8; 15]>();
            if source_length < bufflen && nl.is_null() {
                write(out, &mut pos, &src[..source_length]);
            } else {
                if !nl.is_null() {
                    source_length = nl.offset_from(source) as usize;
                }
                if source_length > bufflen {
                    source_length = bufflen;
                }
                write(out, &mut pos, &src[..source_length]);
                write(out, &mut pos, b"...");
            }
            let suffix_len = size_of::<[i8; 3]>();
            write(
                out,
                &mut pos,
                std::slice::from_raw_parts(c"\"]".as_ptr() as *const u8, suffix_len),
            );
        };
    }
}
pub fn frexp_(x: f64) -> (f64, i32) {
    if x == 0.0 {
        (0.0, 0)
    } else {
        let bits = x.to_bits();
        let sign = if (bits >> 63) != 0 { -1.0 } else { 1.0 };
        let exponent = ((bits >> 52) & 0x7ff) as i32 - 1023;
        let mantissa = sign * f64::from_bits((bits & 0xfffffffffffff) | 0x3fe0000000000000);
        (mantissa, exponent + 1)
    }
}
pub fn ldexp_(x: f64, exp: i32) -> f64 {
    if x == 0.0 || exp == 0 {
        x
    } else {
        let bits = x.to_bits();
        let exponent = ((bits >> 52) & 0x7ff) as i32;
        let new_exponent = exponent + exp;
        if !(0..=0x7ff).contains(&new_exponent) {
            if (bits >> 63) != 0 {
                f64::NEG_INFINITY
            } else {
                f64::INFINITY
            }
        } else {
            let result_bits = (bits & 0x800fffffffffffff) | ((new_exponent as u64) << 52);
            f64::from_bits(result_bits)
        }
    }
}
pub fn l_hashfloat(n: f64) -> i32 {
    let (m, e) = frexp_(n);
    let scaled = m * -(i32::MIN as f64);
    if scaled >= (i64::MIN as f64) && scaled < -(i64::MIN as f64) {
        let u = (e as u32).wrapping_add(scaled as i64 as u32);
        (if u <= i32::MAX as u32 { u } else { !u }) as i32
    } else {
        0
    }
}
/// Format f64 using C's `%g` conventions with the given number of significant digits.
/// Writes into `out` (should be at least 32 bytes). Returns bytes written.
pub fn format_float_g(f: f64, sig_digits: usize, out: &mut [u8]) -> usize {
    use std::io::Write;

    if f.is_nan() {
        out[..3].copy_from_slice(b"nan");
        return 3;
    }
    if f.is_infinite() {
        let s = if f > 0.0 { &b"inf"[..] } else { &b"-inf"[..] };
        out[..s.len()].copy_from_slice(s);
        return s.len();
    }
    if f == 0.0 {
        if f.is_sign_negative() {
            out[..2].copy_from_slice(b"-0");
            return 2;
        }
        out[0] = b'0';
        return 1;
    }

    let abs = f.abs();
    let exp = abs.log10().floor() as i32;
    let out_len = out.len();

    if exp >= sig_digits as i32 || exp < -4 {
        // Scientific notation
        let n = {
            let mut pos: &mut [u8] = &mut *out;
            write!(pos, "{:.prec$e}", f, prec = sig_digits - 1).ok();
            out_len - pos.len()
        };
        let e_pos = out[..n].iter().position(|&b| b == b'e').unwrap_or(n);
        let mut mant_end = e_pos;
        while mant_end > 0 && out[mant_end - 1] == b'0' {
            mant_end -= 1;
        }
        if mant_end > 0 && out[mant_end - 1] == b'.' {
            mant_end -= 1;
        }
        let exp_val: i32 = std::str::from_utf8(&out[e_pos + 1..n])
            .unwrap_or("0")
            .parse()
            .unwrap_or(0);
        let exp_n = {
            let mut pos: &mut [u8] = &mut out[mant_end..];
            if exp_val >= 0 {
                write!(pos, "e+{:02}", exp_val).ok();
            } else {
                write!(pos, "e-{:02}", -exp_val).ok();
            }
            (out_len - mant_end) - pos.len()
        };
        mant_end + exp_n
    } else {
        // Fixed notation
        let dec_places = if exp >= 0 {
            (sig_digits as i32 - 1 - exp).max(0) as usize
        } else {
            sig_digits - 1 + (-exp) as usize
        };
        let n = {
            let mut pos: &mut [u8] = &mut *out;
            write!(pos, "{:.prec$}", f, prec = dec_places).ok();
            out_len - pos.len()
        };
        if !out[..n].contains(&b'.') {
            return n;
        }
        let mut end = n;
        while end > 0 && out[end - 1] == b'0' {
            end -= 1;
        }
        if end > 0 && out[end - 1] == b'.' {
            end -= 1;
        }
        end
    }
}

/// Format f64 with %.Ng, roundtrip-check, fallback to %.17g. Returns bytes written.
pub fn format_float_roundtrip(f: f64, out: &mut [u8]) -> usize {
    let mut n = format_float_g(f, 15, out);
    let check: f64 = std::str::from_utf8(&out[..n])
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(f64::NAN);
    if check != f {
        n = format_float_g(f, 17, out);
    }
    n
}

pub fn fits_c(i: i64) -> bool {
    (i as usize).wrapping_add((((1 << 8) - 1) >> 1) as usize) <= ((1 << 8) - 1) as usize
}
pub fn fits_bx(i: i64) -> bool {
    -(((1 << (8 + 8 + 1)) - 1) >> 1) as i64 <= i && i <= ((1 << (8 + 8 + 1)) - 1 - (((1 << (8 + 8 + 1)) - 1) >> 1)) as i64
}
pub unsafe fn getnum(fmt: *mut *const i8, df: usize) -> usize {
    unsafe {
        if Character::from(**fmt as i32).is_digit_decimal() {
            let mut a: usize = 0;
            loop {
                let current_char = *fmt;
                *fmt = (*fmt).add(1);
                a = a
                    .wrapping_mul(10)
                    .wrapping_add((*current_char as i32 - Character::Digit0 as i32) as usize);
                if !(Character::from(**fmt as i32).is_digit_decimal() && a <= (MAXIMUM_SIZE - 9) / 10) {
                    break;
                }
            }
            a
        } else {
            df
        }
    }
}
