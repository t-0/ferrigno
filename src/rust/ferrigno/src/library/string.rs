use crate::buffer::*;
use crate::character::*;
use crate::gmatchstate::*;
use crate::header::*;
use crate::matchstate::*;
use crate::nativeendian::*;
use crate::packingtype::*;
use crate::registeredfunction::*;
use crate::state::*;
use crate::streamwriter::*;
use crate::tagtype::*;
use crate::tdefaultnew::*;
use crate::tm::*;
use crate::tstring::*;
use crate::tvalue::*;
use crate::utility::*;
use std::ptr::*;
pub unsafe fn str_len(state: *mut State) -> i32 {
    unsafe {
        let mut l: usize = 0;
        lual_checklstring(state, 1, &mut l);
        (*state).push_integer(l as i64);
        1
    }
}
pub unsafe fn str_sub(state: *mut State) -> i32 {
    unsafe {
        let mut l: usize = 0;
        let s: *const i8 = lual_checklstring(state, 1, &mut l);
        let start: usize = get_position_relative(lual_checkinteger(state, 2), l);
        let end: usize = get_position_end(state, 3, -1_i64, l);
        if start <= end {
            lua_pushlstring(state, s.add(start).sub(1), end - start + 1);
        } else {
            lua_pushstring(state, c"".as_ptr());
        }
        1
    }
}
pub unsafe fn str_reverse(state: *mut State) -> i32 {
    unsafe {
        let mut l: usize = 0;
        let mut b = Buffer::new();
        let s: *const i8 = lual_checklstring(state, 1, &mut l);
        let p: *mut i8 = b.initialize_with_size(state, l);
        for i in 0..l {
            *p.add(i) = *s.add(l - i - 1);
        }
        b.push_result_with_size(l);
        1
    }
}
pub unsafe fn str_lower(state: *mut State) -> i32 {
    unsafe {
        let mut l: usize = 0;
        let mut b = Buffer::new();
        let s: *const i8 = lual_checklstring(state, 1, &mut l);
        let p: *mut i8 = b.initialize_with_size(state, l);
        for i in 0..l {
            *p.add(i) = (*s.add(i) as u8).to_ascii_lowercase() as i8;
        }
        b.push_result_with_size(l);
        1
    }
}
pub unsafe fn str_upper(state: *mut State) -> i32 {
    unsafe {
        let mut l: usize = 0;
        let mut b = Buffer::new();
        let s: *const i8 = lual_checklstring(state, 1, &mut l);
        let p: *mut i8 = b.initialize_with_size(state, l);
        for i in 0..l {
            *p.add(i) = (*s.add(i) as u8).to_ascii_uppercase() as i8;
        }
        b.push_result_with_size(l);
        1
    }
}
pub unsafe fn str_rep(state: *mut State) -> i32 {
    unsafe {
        let mut l: usize = 0;
        let mut lsep: usize = 0;
        let s: *const i8 = lual_checklstring(state, 1, &mut l);
        let mut n: i64 = lual_checkinteger(state, 2);
        let sep: *const i8 = lual_optlstring(state, 3, c"".as_ptr(), &mut lsep);
        if n <= 0 {
            lua_pushstring(state, c"".as_ptr());
        } else if l.wrapping_add(lsep) < l
            || l.wrapping_add(lsep)
                > ((if size_of::<usize>() < size_of::<i32>() {
                    !(0usize)
                } else {
                    MAX_INT
                }) as usize)
                    / n as usize
        {
            return lual_error(state, c"resulting string too large".as_ptr(), &[]);
        } else {
            let totallen: usize = (n as usize)
                .wrapping_mul(l)
                .wrapping_add(((n - 1) as usize) * lsep);
            let mut b = Buffer::new();
            let mut p: *mut i8 = b.initialize_with_size(state, totallen);
            loop {
                let fresh = n;
                n -= 1;
                if fresh <= 1 {
                    break;
                }
                std::ptr::copy_nonoverlapping(s as *const u8, p as *mut u8, l);
                p = p.add(l);
                if lsep > 0 {
                    std::ptr::copy_nonoverlapping(sep as *const u8, p as *mut u8, lsep);
                    p = p.add(lsep);
                }
            }
            std::ptr::copy_nonoverlapping(s as *const u8, p as *mut u8, l);
            b.push_result_with_size(totallen);
        }
        1
    }
}
pub unsafe fn str_byte(state: *mut State) -> i32 {
    unsafe {
        let mut l: usize = 0;
        let s: *const i8 = lual_checklstring(state, 1, &mut l);
        let pi: i64 = lual_optinteger(state, 2, 1);
        let posi: usize = get_position_relative(pi, l);
        let pose: usize = get_position_end(state, 3, pi, l);

        if posi > pose {
            return 0;
        }
        if pose - posi >= MAX_INT {
            return lual_error(state, c"string slice too long".as_ptr(), &[]);
        }
        let n: i32 = (pose - posi) as i32 + 1;
        lual_checkstack(state, n, c"string slice too long".as_ptr());
        for i in 0..n {
            (*state).push_integer(*s.add(posi + i as usize - 1) as u8 as i64);
        }
        n
    }
}
pub unsafe fn str_char(state: *mut State) -> i32 {
    unsafe {
        let n: i32 = (*state).get_top();
        let mut buffer = Buffer::new();
        let p: *mut i8 = buffer.initialize_with_size(state, n as usize);
        for i in 1..(1 + n) {
            let c: usize = lual_checkinteger(state, i) as usize;
            if c > (127_i32 * 2 + 1) as usize {
                lual_argerror(state, i, c"value out of range".as_ptr());
                0;
            }
            *p.add((i - 1) as usize) = c as u8 as i8;
        }
        buffer.push_result_with_size(n as usize);
        1
    }
}
pub unsafe fn tonum(state: *mut State, arg: i32) -> i32 {
    unsafe {
        if lua_type(state, arg) == Some(TagType::Numeric) {
            lua_pushvalue(state, arg);
            1
        } else {
            let mut length: usize = 0;
            let s: *const i8 = lua_tolstring(state, arg, &mut length);
            (!s.is_null() && lua_stringtonumber(state, s) == length + 1) as i32
        }
    }
}
pub unsafe fn trymt(state: *mut State, mtname: *const i8) {
    unsafe {
        lua_settop(state, 2);
        if lua_type(state, 2) == Some(TagType::String) || lual_getmetafield(state, 2, mtname) == TagType::Nil {
            lual_error(
                state,
                c"attempt to %s a '%s' with a '%s'".as_ptr(),
                &[
                    mtname.add(2).into(),
                    lua_typename(state, lua_type(state, -2)).into(),
                    lua_typename(state, lua_type(state, -1)).into(),
                ],
            );
        }
        lua_rotate(state, -3, 1);
        (*state).lua_callk(2, 1, 0, None);
    }
}
pub unsafe fn arith(state: *mut State, op: i32, mtname: *const i8) -> i32 {
    unsafe {
        if tonum(state, 1) != 0 && tonum(state, 2) != 0 {
            if !(op != 12_i32 && op != 13_i32) {
                let io1: *mut TValue = &mut (*(*state).interpreter_top.stkidrel_pointer);
                let io2: *const TValue = &mut (*(*state).interpreter_top.stkidrel_pointer.sub(1));
                (*io1).copy_from(&*io2);
                (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.add(1);
            }
            let p1 = &mut (*(*state).interpreter_top.stkidrel_pointer.sub(2));
            let p2 = &mut (*(*state).interpreter_top.stkidrel_pointer.sub(1));
            let res = (*state).interpreter_top.stkidrel_pointer.sub(2);
            if luao_rawarith(state, op, p1, p2, &mut (*res)) == 0 {
                luat_trybintm(state, p1, p2, res, (op + TM_ADD as i32) as u32);
            }
            (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.sub(1);
        } else {
            trymt(state, mtname);
        }
        1
    }
}
pub unsafe fn arith_add(state: *mut State) -> i32 {
    unsafe { arith(state, 0, c"__add".as_ptr()) }
}
pub unsafe fn arith_sub(state: *mut State) -> i32 {
    unsafe { arith(state, 1, c"__sub".as_ptr()) }
}
pub unsafe fn arith_mul(state: *mut State) -> i32 {
    unsafe { arith(state, 2, c"__mul".as_ptr()) }
}
pub unsafe fn arith_mod(state: *mut State) -> i32 {
    unsafe { arith(state, 3, c"__mod".as_ptr()) }
}
pub unsafe fn arith_pow(state: *mut State) -> i32 {
    unsafe { arith(state, 4, c"__pow".as_ptr()) }
}
pub unsafe fn arith_div(state: *mut State) -> i32 {
    unsafe { arith(state, 5, c"__div".as_ptr()) }
}
pub unsafe fn arith_idiv(state: *mut State) -> i32 {
    unsafe { arith(state, 6, c"__idiv".as_ptr()) }
}
pub unsafe fn arith_unm(state: *mut State) -> i32 {
    unsafe { arith(state, 12_i32, c"__unm".as_ptr()) }
}
pub const STRING_METAMETHODS: [RegisteredFunction; 8] = {
    [
        {
            RegisteredFunction {
                registeredfunction_name: c"__add".as_ptr(),
                registeredfunction_function: Some(arith_add as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"__sub".as_ptr(),
                registeredfunction_function: Some(arith_sub as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"__mul".as_ptr(),
                registeredfunction_function: Some(arith_mul as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"__mod".as_ptr(),
                registeredfunction_function: Some(arith_mod as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"__pow".as_ptr(),
                registeredfunction_function: Some(arith_pow as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"__div".as_ptr(),
                registeredfunction_function: Some(arith_div as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"__idiv".as_ptr(),
                registeredfunction_function: Some(arith_idiv as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"__unm".as_ptr(),
                registeredfunction_function: Some(arith_unm as unsafe fn(*mut State) -> i32),
            }
        },
    ]
};
pub unsafe fn lmemfind(mut s1: *const i8, mut l1: usize, s2: *const i8, mut l2: usize) -> *const i8 {
    unsafe {
        if l2 == 0 {
            s1
        } else if l2 > l1 {
            null()
        } else {
            let mut initial: *const i8;
            l2 -= 1;
            l1 -= l2;
            while l1 > 0 && {
                initial = mem_chr(s1 as *const u8, *s2 as u8, l1) as *const i8;
                !initial.is_null()
            } {
                initial = initial.add(1);
                if std::slice::from_raw_parts(initial as *const u8, l2) == std::slice::from_raw_parts(s2.add(1) as *const u8, l2) {
                    return initial.sub(1);
                } else {
                    l1 -= initial.offset_from(s1) as usize;
                    s1 = initial;
                }
            }
            null()
        }
    }
}
pub unsafe fn nospecials(p: *const i8, l: usize) -> i32 {
    unsafe {
        let mut upto: usize = 0;
        loop {
            if !(cstr_pbrk(p.add(upto), c"^$*+?.([%-".as_ptr())).is_null() {
                return 0;
            }
            upto += cstr_len(p.add(upto)) + 1;
            if upto > l {
                break;
            }
        }
        1
    }
}
pub unsafe fn str_find_aux(state: *mut State, find: i32) -> i32 {
    unsafe {
        let mut lexical_state: usize = 0;
        let mut lp: usize = 0;
        let s: *const i8 = lual_checklstring(state, 1, &mut lexical_state);
        let mut p: *const i8 = lual_checklstring(state, 2, &mut lp);
        let initial: usize = get_position_relative(lual_optinteger(state, 3, 1_i64), lexical_state) - 1;
        if initial > lexical_state {
            (*state).push_nil();
            return 1;
        }
        if find != 0 && (lua_toboolean(state, 4) || nospecials(p, lp) != 0) {
            let s2: *const i8 = lmemfind(s.add(initial), lexical_state - initial, p, lp);
            if !s2.is_null() {
                (*state).push_integer(s2.offset_from(s) as i64 + 1);
                (*state).push_integer((s2.offset_from(s) as usize + lp) as i64);
                return 2;
            }
        } else {
            let mut match_state: MatchState = MatchState {
                src_init: null(),
                src_end: null(),
                p_end: null(),
                matchstate_interpreter: null_mut(),
                matchdepth: 0,
                level: 0,
                capture: [MatchStateCapture {
                    matchstatecapture_initial: null(),
                    matchstatecapture_length: 0,
                }; 32],
            };
            let mut s1: *const i8 = s.add(initial);
            let anchor: i32 = (*p as i32 == Character::Caret as i32) as i32;
            if anchor != 0 {
                p = p.add(1);
                lp -= 1;
            }
            match_state.prepstate(state, s, lexical_state, p, lp);
            loop {
                match_state.reprepstate();
                let res: *const i8 = match_state.match_pattern(s1, p);
                if !res.is_null() {
                    if find != 0 {
                        (*state).push_integer(s1.offset_from(s) as i64 + 1);
                        (*state).push_integer(res.offset_from(s) as i64);
                        return match_state.push_captures(null(), null()) + 2;
                    } else {
                        return match_state.push_captures(s1, res);
                    }
                }
                let current_s1 = s1;
                s1 = s1.add(1);
                if !(current_s1 < match_state.src_end && anchor == 0) {
                    break;
                }
            }
        }
        (*state).push_nil();
        1
    }
}
pub unsafe fn str_find(state: *mut State) -> i32 {
    unsafe { str_find_aux(state, 1) }
}
pub unsafe fn str_match(state: *mut State) -> i32 {
    unsafe { str_find_aux(state, 0) }
}
pub unsafe fn str_gsub(state: *mut State) -> i32 {
    unsafe {
        let mut srcl: usize = 0;
        let mut lp: usize = 0;
        let mut src: *const i8 = lual_checklstring(state, 1, &mut srcl);
        let mut p: *const i8 = lual_checklstring(state, 2, &mut lp);
        let mut lastmatch: *const i8 = null();
        let tr = lua_type(state, 3);
        let max_s: i64 = lual_optinteger(state, 4, (srcl + 1) as i64);
        let anchor: i32 = (*p as i32 == Character::Caret as i32) as i32;
        let mut n: i64 = 0;
        let mut changed: i32 = 0;
        let mut match_state: MatchState = MatchState {
            src_init: null(),
            src_end: null(),
            p_end: null(),
            matchstate_interpreter: null_mut(),
            matchdepth: 0,
            level: 0,
            capture: [MatchStateCapture {
                matchstatecapture_initial: null(),
                matchstatecapture_length: 0,
            }; 32],
        };
        let mut b = Buffer::new();
        if !(tr == Some(TagType::Numeric)
            || tr == Some(TagType::String)
            || tr == Some(TagType::Closure)
            || tr == Some(TagType::Table))
        {
            lual_typeerror(state, 3, c"string/function/table".as_ptr());
            0;
        }
        b.initialize(state);
        if anchor != 0 {
            p = p.add(1);
            lp -= 1;
        }
        match_state.prepstate(state, src, srcl, p, lp);
        while n < max_s {
            match_state.reprepstate();
            let e: *const i8 = match_state.match_pattern(src, p);
            if !e.is_null() && e != lastmatch {
                n += 1;
                changed |= match_state.add_value(&mut b, src, e, tr.unwrap());
                lastmatch = e;
                src = lastmatch;
            } else {
                if src >= match_state.src_end {
                    break;
                }
                (b.buffer_loads.get_length() < b.buffer_loads.get_size() || !(b.prepare_with_size(1)).is_null()) as i32;
                let current_src = src;
                src = src.add(1);
                let write_offset = b.buffer_loads.get_length();
                b.buffer_loads
                    .set_length((b.buffer_loads.get_length() + 1) as usize);
                *(b.buffer_loads.loads_pointer).add(write_offset as usize) = *current_src;
            }
            if anchor != 0 {
                break;
            }
        }
        if changed == 0 {
            lua_pushvalue(state, 1);
        } else {
            b.add_string_with_length(src, (match_state.src_end).offset_from(src) as usize);
            b.push_result();
        }
        (*state).push_integer(n);
        2
    }
}
pub unsafe fn addquoted(b: *mut Buffer, mut s: *const i8, mut length: usize) {
    unsafe {
        if (*b).buffer_loads.get_length() >= (*b).buffer_loads.get_size() {
            ((*b).prepare_with_size(1)).is_null();
        }
        let write_offset = (*b).buffer_loads.get_length();
        (*b).buffer_loads
            .set_length(((*b).buffer_loads.get_length() + 1) as usize);
        *((*b).buffer_loads.loads_pointer).add(write_offset as usize) = '"' as i8;
        loop {
            if length == 0 {
                break;
            }
            length -= 1;
            if *s as i32 == '"' as i32 || *s as i32 == Character::Backslash as i32 || *s as i32 == Character::LineFeed as i32 {
                if (*b).buffer_loads.get_length() >= (*b).buffer_loads.get_size() {
                    ((*b).prepare_with_size(1)).is_null();
                }
                let write_offset = (*b).buffer_loads.get_length();
                (*b).buffer_loads
                    .set_length(((*b).buffer_loads.get_length() + 1) as usize);
                *((*b).buffer_loads.loads_pointer).add(write_offset as usize) = Character::Backslash as i8;
                if (*b).buffer_loads.get_length() >= (*b).buffer_loads.get_size() {
                    ((*b).prepare_with_size(1)).is_null();
                }
                let write_offset = (*b).buffer_loads.get_length();
                (*b).buffer_loads
                    .set_length(((*b).buffer_loads.get_length() + 1) as usize);
                *((*b).buffer_loads.loads_pointer).add(write_offset as usize) = *s;
            } else if Character::from(*s as u8 as i32).is_control() {
                let mut buffer: [i8; 10] = [0; 10];
                {
                    use std::io::Write;
                    let val = *s as u8;
                    let mut cursor = std::io::Cursor::new(&mut *(&mut buffer[..] as *mut [i8] as *mut [u8]));
                    if Character::from(*s.add(1) as u8 as i32).is_digit_decimal() {
                        let _ = write!(cursor, "\\{:03}", val);
                    } else {
                        let _ = write!(cursor, "\\{}", val);
                    }
                    let pos = cursor.position() as usize;
                    buffer[pos] = 0;
                }
                (*b).add_string(buffer.as_mut_ptr());
            } else {
                if (*b).buffer_loads.get_length() >= (*b).buffer_loads.get_size() {
                    ((*b).prepare_with_size(1)).is_null();
                }
                let write_offset = (*b).buffer_loads.get_length();
                (*b).buffer_loads
                    .set_length(((*b).buffer_loads.get_length() + 1) as usize);
                *((*b).buffer_loads.loads_pointer).add(write_offset as usize) = *s;
            }
            s = s.add(1);
        }
        if (*b).buffer_loads.get_length() >= (*b).buffer_loads.get_size() {
            ((*b).prepare_with_size(1)).is_null();
        }
        let write_offset = (*b).buffer_loads.get_length();
        (*b).buffer_loads
            .set_length(((*b).buffer_loads.get_length() + 1) as usize);
        *((*b).buffer_loads.loads_pointer).add(write_offset as usize) = '"' as i8;
    }
}
// ─── C printf replacement helpers ────────────────────────────────────────────

/// Parsed C printf format specification.
struct FmtSpec {
    left: bool,
    sign: bool,
    space: bool,
    zero: bool,
    alt: bool,
    width: usize,
    prec: i32, // -1 = default
    spec: u8,
}

/// Parse a C format string starting with '%'.
unsafe fn parse_fmt_spec(form: *const i8) -> FmtSpec {
    unsafe {
        let mut s = FmtSpec {
            left: false,
            sign: false,
            space: false,
            zero: false,
            alt: false,
            width: 0,
            prec: -1,
            spec: 0,
        };
        let mut i = 1usize;
        loop {
            match *form.add(i) as u8 {
                b'-' => s.left = true,
                b'+' => s.sign = true,
                b' ' => s.space = true,
                b'0' => s.zero = true,
                b'#' => s.alt = true,
                _ => break,
            }
            i += 1;
        }
        while (*form.add(i) as u8).is_ascii_digit() {
            s.width = s.width * 10 + (*form.add(i) as u8 - b'0') as usize;
            i += 1;
        }
        if *form.add(i) as u8 == b'.' {
            i += 1;
            s.prec = 0;
            while (*form.add(i) as u8).is_ascii_digit() {
                s.prec = s.prec * 10 + (*form.add(i) as u8 - b'0') as i32;
                i += 1;
            }
        }
        while matches!(
            *form.add(i) as u8,
            b'l' | b'h' | b'L' | b'z' | b'j' | b't' | b'q'
        ) {
            i += 1;
        }
        if *form.add(i) != 0 {
            s.spec = *form.add(i) as u8;
        }
        s
    }
}

/// Apply width/alignment padding. Returns bytes written.
fn pad_formatted(out: &mut [u8], content: &[u8], width: usize, left: bool, zero_pad: bool) -> usize {
    let clen = content.len();
    if clen >= width {
        let n = clen.min(out.len());
        out[..n].copy_from_slice(&content[..n]);
        return n;
    }
    let total = width.min(out.len());
    let pad = total - clen;
    if left {
        out[..clen].copy_from_slice(content);
        for b in &mut out[clen..total] {
            *b = b' ';
        }
    } else if zero_pad {
        // Detect prefix (sign and/or 0x) to place before zero padding
        let mut pfx = 0;
        if clen > 0 && matches!(content[0], b'-' | b'+' | b' ') {
            pfx = 1;
            if clen > 2 && content[1] == b'0' && matches!(content[2], b'x' | b'X') {
                pfx = 3;
            }
        } else if clen > 1 && content[0] == b'0' && matches!(content[1], b'x' | b'X') {
            pfx = 2;
        }
        out[..pfx].copy_from_slice(&content[..pfx]);
        for b in &mut out[pfx..pfx + pad] {
            *b = b'0';
        }
        out[pfx + pad..total].copy_from_slice(&content[pfx..]);
    } else {
        for b in &mut out[..pad] {
            *b = b' ';
        }
        out[pad..total].copy_from_slice(content);
    }
    total
}

/// Format a hex float value (%a/%A). Always uses '.' as decimal point.
fn fmt_hex_float(out: &mut [u8], value: f64, prec: i32, uppercase: bool) -> usize {
    use std::io::Write;
    let bits = value.to_bits();
    let is_neg = (bits >> 63) != 0;
    let raw_exp = ((bits >> 52) & 0x7ff) as i32;
    let raw_mant = bits & 0x000fffffffffffff;
    let hex: &[u8; 16] = if uppercase {
        b"0123456789ABCDEF"
    } else {
        b"0123456789abcdef"
    };
    let mut pos = 0;

    if is_neg {
        out[pos] = b'-';
        pos += 1;
    }
    out[pos] = b'0';
    pos += 1;
    out[pos] = if uppercase { b'X' } else { b'x' };
    pos += 1;

    if raw_exp == 0 && raw_mant == 0 {
        out[pos] = b'0';
        pos += 1;
        let p = if prec < 0 { 0 } else { prec as usize };
        if p > 0 {
            out[pos] = b'.';
            pos += 1;
            for _ in 0..p {
                out[pos] = b'0';
                pos += 1;
            }
        }
        out[pos] = if uppercase { b'P' } else { b'p' };
        pos += 1;
        out[pos] = b'+';
        pos += 1;
        out[pos] = b'0';
        pos += 1;
        return pos;
    }

    let (mut lead, mut exp) = if raw_exp == 0 {
        (0u64, -1022i32)
    } else {
        (1u64, raw_exp - 1023)
    };
    let mut mant = raw_mant;

    let ndigits = if prec >= 0 {
        let p = prec as usize;
        if p < 13 {
            let shift = (13 - p) * 4;
            let dropped = mant & ((1u64 << shift) - 1);
            let half = 1u64 << (shift - 1);
            mant >>= shift;
            if dropped > half || (dropped == half && (mant & 1) != 0) {
                mant += 1;
                if mant >= (1u64 << (p * 4)) {
                    mant = 0;
                    lead += 1;
                    if lead >= 2 {
                        lead = 1;
                        exp += 1;
                    }
                }
            }
            mant <<= shift;
        }
        p
    } else {
        let mut m = mant;
        let mut trailing = 0usize;
        while trailing < 13 && (m & 0xf) == 0 {
            m >>= 4;
            trailing += 1;
        }
        13 - trailing
    };

    out[pos] = hex[lead as usize];
    pos += 1;
    if ndigits > 0 {
        out[pos] = b'.';
        pos += 1;
        for i in 0..ndigits {
            let nibble = if i < 13 {
                (mant >> (48 - 4 * i)) & 0xf
            } else {
                0
            };
            out[pos] = hex[nibble as usize];
            pos += 1;
        }
    }

    out[pos] = if uppercase { b'P' } else { b'p' };
    pos += 1;
    if exp >= 0 {
        out[pos] = b'+';
    } else {
        out[pos] = b'-';
    }
    pos += 1;
    let abs_exp = exp.unsigned_abs();
    let mut cursor = std::io::Cursor::new(&mut out[pos..]);
    write!(cursor, "{}", abs_exp).ok();
    pos += cursor.position() as usize;
    pos
}

/// Fix Rust scientific exponent format (e.g. `e1`) to C-style (e.g. `e+01`).
fn fix_sci_exponent(buf: &mut [u8], len: usize) -> usize {
    use std::io::Write;
    let e_pos = match buf[..len].iter().position(|&b| b == b'e' || b == b'E') {
        Some(p) => p,
        None => return len,
    };
    let e_char = buf[e_pos];
    let exp_str = std::str::from_utf8(&buf[e_pos + 1..len]).unwrap_or("0");
    let exp_val: i32 = exp_str.parse().unwrap_or(0);
    let mut pos = e_pos;
    buf[pos] = e_char;
    pos += 1;
    if exp_val >= 0 {
        buf[pos] = b'+';
    } else {
        buf[pos] = b'-';
    }
    pos += 1;
    let abs_exp = exp_val.unsigned_abs();
    if abs_exp < 10 {
        buf[pos] = b'0';
        pos += 1;
    }
    let mut cursor = std::io::Cursor::new(&mut buf[pos..]);
    write!(cursor, "{}", abs_exp).ok();
    pos += cursor.position() as usize;
    pos
}

/// Format %c
unsafe fn sprintf_char(buf: *mut i8, size: usize, form: *const i8, value: i32) -> i32 {
    let spec = unsafe { parse_fmt_spec(form) };
    let ch = [value as u8];
    let out = unsafe { std::slice::from_raw_parts_mut(buf as *mut u8, size) };
    pad_formatted(out, &ch, spec.width, spec.left, false) as i32
}

/// Format %d, %i, %u, %o, %x, %X
unsafe fn sprintf_int(buf: *mut i8, size: usize, form: *const i8, value: i64) -> i32 {
    use std::io::Write;
    let spec = unsafe { parse_fmt_spec(form) };
    let out = unsafe { std::slice::from_raw_parts_mut(buf as *mut u8, size) };

    let is_signed = matches!(spec.spec, b'd' | b'i');
    let uval = value as u64;
    let is_neg = value < 0 && is_signed;
    let abs_val = if is_neg {
        (0u64).wrapping_sub(uval)
    } else {
        uval
    };

    let mut digits = [0u8; 64];
    let dlen = {
        let mut c = std::io::Cursor::new(&mut digits[..]);
        match spec.spec {
            b'd' | b'i' => write!(c, "{}", abs_val).ok(),
            b'u' => write!(c, "{}", uval).ok(),
            b'o' => write!(c, "{:o}", uval).ok(),
            b'x' => write!(c, "{:x}", uval).ok(),
            b'X' => write!(c, "{:X}", uval).ok(),
            _ => None,
        };
        c.position() as usize
    };

    let mut prec = if spec.prec < 0 { 1 } else { spec.prec as usize };
    if spec.alt && spec.spec == b'o' && dlen >= prec && (dlen == 0 || digits[0] != b'0') {
        prec = dlen + 1;
    }

    let mut tmp = [0u8; 96];
    let mut pos = 0;

    if is_neg {
        tmp[pos] = b'-';
        pos += 1;
    } else if spec.sign && is_signed {
        tmp[pos] = b'+';
        pos += 1;
    } else if spec.space && is_signed {
        tmp[pos] = b' ';
        pos += 1;
    }

    if spec.alt && uval != 0 {
        match spec.spec {
            b'x' => {
                tmp[pos] = b'0';
                tmp[pos + 1] = b'x';
                pos += 2;
            }
            b'X' => {
                tmp[pos] = b'0';
                tmp[pos + 1] = b'X';
                pos += 2;
            }
            _ => {}
        }
    }

    let print_digits = !(spec.prec == 0 && uval == 0 && !(spec.alt && spec.spec == b'o'));
    if print_digits {
        for _ in dlen..prec {
            tmp[pos] = b'0';
            pos += 1;
        }
        tmp[pos..pos + dlen].copy_from_slice(&digits[..dlen]);
        pos += dlen;
    }

    let use_zero = spec.zero && spec.prec < 0 && !spec.left;
    pad_formatted(out, &tmp[..pos], spec.width, spec.left, use_zero) as i32
}

/// Format %f, %e, %E, %g, %G
unsafe fn sprintf_float(buf: *mut i8, size: usize, form: *const i8, value: f64) -> i32 {
    use std::io::Write;
    let spec = unsafe { parse_fmt_spec(form) };
    let out = unsafe { std::slice::from_raw_parts_mut(buf as *mut u8, size) };

    if value.is_nan() {
        let s: &[u8] = if spec.spec.is_ascii_uppercase() {
            b"NAN"
        } else {
            b"nan"
        };
        return pad_formatted(out, s, spec.width, spec.left, false) as i32;
    }
    if value.is_infinite() {
        let mut tmp = [0u8; 8];
        let mut pos = 0;
        if value < 0.0 {
            tmp[pos] = b'-';
            pos += 1;
        } else if spec.sign {
            tmp[pos] = b'+';
            pos += 1;
        } else if spec.space {
            tmp[pos] = b' ';
            pos += 1;
        }
        let s: &[u8] = if spec.spec.is_ascii_uppercase() {
            b"INF"
        } else {
            b"inf"
        };
        tmp[pos..pos + 3].copy_from_slice(s);
        pos += 3;
        return pad_formatted(out, &tmp[..pos], spec.width, spec.left, false) as i32;
    }

    let is_neg = value < 0.0 || (value == 0.0 && value.is_sign_negative());
    let abs = value.abs();
    let prec = if spec.prec < 0 { 6 } else { spec.prec as usize };

    let mut tmp = [0u8; 512];
    let mut pos = 0;

    if is_neg {
        tmp[pos] = b'-';
        pos += 1;
    } else if spec.sign {
        tmp[pos] = b'+';
        pos += 1;
    } else if spec.space {
        tmp[pos] = b' ';
        pos += 1;
    }

    match spec.spec {
        b'f' | b'F' => {
            let mut c = std::io::Cursor::new(&mut tmp[pos..]);
            write!(c, "{:.prec$}", abs, prec = prec).ok();
            pos += c.position() as usize;
            if spec.alt && prec == 0 && !tmp[..pos].contains(&b'.') {
                tmp[pos] = b'.';
                pos += 1;
            }
        }
        b'e' | b'E' => {
            let mut sci = [0u8; 256];
            let mut c = std::io::Cursor::new(&mut sci[..]);
            if spec.spec == b'E' {
                write!(c, "{:.prec$E}", abs, prec = prec).ok();
            } else {
                write!(c, "{:.prec$e}", abs, prec = prec).ok();
            }
            let slen = c.position() as usize;
            let slen = fix_sci_exponent(&mut sci, slen);
            if spec.alt && prec == 0 {
                let e_pos = sci[..slen]
                    .iter()
                    .position(|&b| b == b'e' || b == b'E')
                    .unwrap_or(slen);
                if !sci[..e_pos].contains(&b'.') {
                    sci.copy_within(e_pos..slen, e_pos + 1);
                    sci[e_pos] = b'.';
                    let slen = slen + 1;
                    tmp[pos..pos + slen].copy_from_slice(&sci[..slen]);
                    pos += slen;
                } else {
                    tmp[pos..pos + slen].copy_from_slice(&sci[..slen]);
                    pos += slen;
                }
            } else {
                tmp[pos..pos + slen].copy_from_slice(&sci[..slen]);
                pos += slen;
            }
        }
        b'g' | b'G' => {
            let prec_g = if prec == 0 { 1 } else { prec };
            if spec.alt {
                let exp_val = if abs == 0.0 {
                    0
                } else {
                    abs.log10().floor() as i32
                };
                if exp_val >= prec_g as i32 || exp_val < -4 {
                    let eprec = prec_g - 1;
                    let mut sci = [0u8; 256];
                    let mut c = std::io::Cursor::new(&mut sci[..]);
                    if spec.spec == b'G' {
                        write!(c, "{:.prec$E}", abs, prec = eprec).ok();
                    } else {
                        write!(c, "{:.prec$e}", abs, prec = eprec).ok();
                    }
                    let slen = c.position() as usize;
                    let slen = fix_sci_exponent(&mut sci, slen);
                    tmp[pos..pos + slen].copy_from_slice(&sci[..slen]);
                    pos += slen;
                } else {
                    let fprec = if exp_val >= 0 {
                        (prec_g as i32 - 1 - exp_val).max(0) as usize
                    } else {
                        prec_g - 1 + (-exp_val) as usize
                    };
                    let mut c = std::io::Cursor::new(&mut tmp[pos..]);
                    write!(c, "{:.prec$}", abs, prec = fprec).ok();
                    pos += c.position() as usize;
                }
            } else {
                let mut flt = [0u8; 512];
                let flen = format_float_g(abs, prec_g, &mut flt);
                if spec.spec == b'G'
                    && let Some(e_pos) = flt[..flen].iter().position(|&b| b == b'e')
                {
                    flt[e_pos] = b'E';
                }
                tmp[pos..pos + flen].copy_from_slice(&flt[..flen]);
                pos += flen;
            }
        }
        _ => {}
    }

    let use_zero = spec.zero && !spec.left;
    pad_formatted(out, &tmp[..pos], spec.width, spec.left, use_zero) as i32
}

/// Format %a, %A with flags and width
unsafe fn sprintf_hex_float_full(buf: *mut i8, size: usize, form: *const i8, value: f64) -> i32 {
    let spec = unsafe { parse_fmt_spec(form) };
    let out = unsafe { std::slice::from_raw_parts_mut(buf as *mut u8, size) };
    let uppercase = spec.spec == b'A';

    // Handle special values (inf, nan)
    if value.is_nan() {
        let s: &[u8] = if uppercase { b"NAN" } else { b"nan" };
        return pad_formatted(out, s, spec.width, spec.left, false) as i32;
    }
    if value.is_infinite() {
        let mut tmp = [0u8; 8];
        let mut pos = 0;
        if value < 0.0 {
            tmp[pos] = b'-';
            pos += 1;
        } else if spec.sign {
            tmp[pos] = b'+';
            pos += 1;
        } else if spec.space {
            tmp[pos] = b' ';
            pos += 1;
        }
        let s: &[u8] = if uppercase { b"INF" } else { b"inf" };
        tmp[pos..pos + 3].copy_from_slice(s);
        pos += 3;
        return pad_formatted(out, &tmp[..pos], spec.width, spec.left, false) as i32;
    }

    let mut tmp = [0u8; 128];
    let len = fmt_hex_float(&mut tmp, value, spec.prec, uppercase);

    if !value.is_sign_negative() && (spec.sign || spec.space) {
        let mut with_sign = [0u8; 129];
        with_sign[0] = if spec.sign { b'+' } else { b' ' };
        with_sign[1..1 + len].copy_from_slice(&tmp[..len]);
        let use_zero = spec.zero && !spec.left;
        return pad_formatted(out, &with_sign[..len + 1], spec.width, spec.left, use_zero) as i32;
    }

    let use_zero = spec.zero && !spec.left;
    pad_formatted(out, &tmp[..len], spec.width, spec.left, use_zero) as i32
}

/// Format %s with width and precision
unsafe fn sprintf_str(buf: *mut i8, size: usize, form: *const i8, value: *const i8) -> i32 {
    let spec = unsafe { parse_fmt_spec(form) };
    let out = unsafe { std::slice::from_raw_parts_mut(buf as *mut u8, size) };
    let slen = unsafe { cstr_len(value) };
    let effective_len = if spec.prec >= 0 {
        slen.min(spec.prec as usize)
    } else {
        slen
    };
    let content = unsafe { std::slice::from_raw_parts(value as *const u8, effective_len) };
    pad_formatted(out, content, spec.width, spec.left, false) as i32
}

/// Format %p
unsafe fn sprintf_ptr(buf: *mut i8, size: usize, form: *const i8, value: *const std::ffi::c_void) -> i32 {
    use std::io::Write;
    let spec = unsafe { parse_fmt_spec(form) };
    let out = unsafe { std::slice::from_raw_parts_mut(buf as *mut u8, size) };
    let mut tmp = [0u8; 64];
    let len = {
        let mut c = std::io::Cursor::new(&mut tmp[..]);
        write!(c, "{:p}", value).ok();
        c.position() as usize
    };
    pad_formatted(out, &tmp[..len], spec.width, spec.left, false) as i32
}

// ─── End C printf replacement helpers ────────────────────────────────────────

pub unsafe fn quotefloat(mut _state: *mut State, buffer: *mut i8, n: f64) -> i32 {
    unsafe {
        let s: *const i8;
        if n == ::core::f64::INFINITY {
            s = c"1e9999".as_ptr();
        } else if n == -::core::f64::INFINITY {
            s = c"-1e9999".as_ptr();
        } else if n != n {
            s = c"(0/0)".as_ptr();
        } else {
            let out = std::slice::from_raw_parts_mut(buffer as *mut u8, 120);
            return fmt_hex_float(out, n, -1, false) as i32;
        }
        {
            let len = cstr_len(s);
            let copy_len = len.min(119);
            std::ptr::copy_nonoverlapping(s, buffer, copy_len);
            *buffer.add(copy_len) = 0;
            copy_len as i32
        }
    }
}
pub unsafe fn addliteral(state: *mut State, b: *mut Buffer, arg: i32) {
    unsafe {
        match lua_type(state, arg) {
            Some(TagType::String) => {
                let mut length: usize = 0;
                let s: *const i8 = lua_tolstring(state, arg, &mut length);
                addquoted(b, s, length);
            }
            Some(TagType::Numeric) => {
                let buffer: *mut i8 = (*b).prepare_with_size(120);
                let nb: i32;
                if lua_isinteger(state, arg) {
                    let n: i64 = lua_tointegerx(state, arg, null_mut());
                    {
                        use std::io::Write;
                        let mut cursor = std::io::Cursor::new(std::slice::from_raw_parts_mut(buffer as *mut u8, 120));
                        if n == -(MAXIMUM_SIZE as i64) - 1_i64 {
                            let _ = write!(cursor, "0x{:x}", n as u64);
                        } else {
                            let _ = write!(cursor, "{}", n);
                        }
                        let pos = cursor.position() as usize;
                        *buffer.add(pos) = 0;
                        nb = pos as i32;
                    }
                } else {
                    nb = quotefloat(state, buffer, lua_tonumberx(state, arg, null_mut()));
                }
                (*b).buffer_loads
                    .set_length(((*b).buffer_loads.get_length() as usize + nb as usize) as usize);
            }
            Some(TagType::Nil) | Some(TagType::Boolean) => {
                lual_tolstring(state, arg, null_mut());
                (*b).add_value();
            }
            _ => {
                lual_argerror(state, arg, c"value has no literal form".as_ptr());
            }
        };
    }
}
pub unsafe fn get2digits(mut s: *const i8) -> *const i8 {
    unsafe {
        if Character::from(*s as u8 as i32).is_digit_decimal() {
            s = s.add(1);
            if Character::from(*s as u8 as i32).is_digit_decimal() {
                s = s.add(1);
            }
        }
        s
    }
}
pub unsafe fn checkformat(state: *mut State, form: *const i8, flags: *const i8, precision: i32) {
    unsafe {
        let mut spec: *const i8 = form.add(1);
        spec = spec.add(cstr_span(spec, flags));
        if *spec as i32 != Character::Digit0 as i32 {
            spec = get2digits(spec);
            if *spec as i32 == Character::Period as i32 && precision != 0 {
                spec = spec.add(1);
                spec = get2digits(spec);
            }
        }
        if !Character::from(*spec as u8 as i32).is_alpha() {
            lual_error(
                state,
                c"invalid conversion specification: '%s'".as_ptr(),
                &[form.into()],
            );
        }
    }
}
pub unsafe fn getformat(state: *mut State, strfrmt: *const i8, mut form: *mut i8) -> *const i8 {
    unsafe {
        let mut length = cstr_span(strfrmt, c"-+#0 123456789.".as_ptr());
        length += 1;
        if length >= 22 {
            lual_error(state, c"invalid format (too long)".as_ptr(), &[]);
        }
        let current_form = form;
        form = form.add(1);
        *current_form = Character::Percent as i8;
        std::ptr::copy_nonoverlapping(strfrmt as *const u8, form as *mut u8, length);
        *form.add(length) = Character::Null as i8;
        strfrmt.add(length).sub(1)
    }
}
pub unsafe fn str_format(state: *mut State) -> i32 {
    unsafe {
        const FMT_DONE: usize = 0;
        const FMT_INTEGER: usize = 1;
        const FMT_FLOAT: usize = 2;
        let mut fmt_action: usize;
        let top: i32 = (*state).get_top();
        let mut arg: i32 = 1;
        let mut sfl: usize = 0;
        let mut strfrmt: *const i8 = lual_checklstring(state, arg, &mut sfl);
        let strfrmt_end: *const i8 = strfrmt.add(sfl);
        let mut flags: *const i8 = null();
        let mut b = Buffer::new();
        b.initialize(state);
        while strfrmt < strfrmt_end {
            if *strfrmt as i32 != Character::Percent as i32 {
                (b.buffer_loads.get_length() < b.buffer_loads.get_size() || !(b.prepare_with_size(1)).is_null()) as i32;
                let current_strfrmt = strfrmt;
                strfrmt = strfrmt.add(1);
                let write_offset = b.buffer_loads.get_length();
                b.buffer_loads
                    .set_length((b.buffer_loads.get_length() + 1) as usize);
                *(b.buffer_loads.loads_pointer).add(write_offset as usize) = *current_strfrmt;
            } else {
                strfrmt = strfrmt.add(1);
                if *strfrmt as i32 == Character::Percent as i32 {
                    (b.buffer_loads.get_length() < b.buffer_loads.get_size() || !(b.prepare_with_size(1)).is_null()) as i32;
                    let current_strfrmt = strfrmt;
                    strfrmt = strfrmt.add(1);
                    let write_offset = b.buffer_loads.get_length();
                    b.buffer_loads
                        .set_length((b.buffer_loads.get_length() + 1) as usize);
                    *(b.buffer_loads.loads_pointer).add(write_offset as usize) = *current_strfrmt;
                } else {
                    let mut form: [i8; 32] = [0; 32];
                    let mut maxitem: i32 = 120_i32;
                    let mut buffer: *mut i8 = b.prepare_with_size(maxitem as usize);
                    let mut nb: i32 = 0;
                    arg += 1;
                    if arg > top {
                        return lual_argerror(state, arg, c"no value".as_ptr());
                    }
                    strfrmt = getformat(state, strfrmt, form.as_mut_ptr());
                    let current_strfrmt = strfrmt;
                    strfrmt = strfrmt.add(1);
                    match Character::from(*current_strfrmt as i32) {
                        Character::LowerC => {
                            checkformat(state, form.as_mut_ptr(), c"-".as_ptr(), 0);
                            nb = sprintf_char(
                                buffer,
                                maxitem as usize,
                                form.as_mut_ptr(),
                                lual_checkinteger(state, arg) as i32,
                            );
                            fmt_action = FMT_DONE;
                        }
                        Character::LowerD | Character::LowerI => {
                            flags = c"-+0 ".as_ptr();
                            fmt_action = FMT_INTEGER;
                        }
                        Character::LowerU => {
                            flags = c"-0".as_ptr();
                            fmt_action = FMT_INTEGER;
                        }
                        Character::LowerO | Character::LowerX | Character::UpperX => {
                            flags = c"-#0".as_ptr();
                            fmt_action = FMT_INTEGER;
                        }
                        Character::LowerA | Character::UpperA => {
                            checkformat(state, form.as_mut_ptr(), c"-+#0 ".as_ptr(), 1);
                            nb = sprintf_hex_float_full(
                                buffer,
                                maxitem as usize,
                                form.as_mut_ptr(),
                                lual_checknumber(state, arg),
                            );
                            fmt_action = FMT_DONE;
                        }
                        Character::LowerF => {
                            maxitem = 110_i32 + 308_i32;
                            buffer = b.prepare_with_size(maxitem as usize);
                            fmt_action = FMT_FLOAT;
                        }
                        Character::LowerE | Character::UpperE | Character::LowerG | Character::UpperG => {
                            fmt_action = FMT_FLOAT;
                        }
                        Character::LowerP => {
                            let p: *const std::ffi::c_void = (*state).to_pointer(arg);
                            checkformat(state, form.as_mut_ptr(), c"-".as_ptr(), 0);
                            if p.is_null() {
                                form[cstr_len(form.as_mut_ptr()) - 1] = Character::LowerS as i8;
                                nb = sprintf_str(
                                    buffer,
                                    maxitem as usize,
                                    form.as_mut_ptr(),
                                    c"(null)".as_ptr(),
                                );
                            } else {
                                nb = sprintf_ptr(buffer, maxitem as usize, form.as_mut_ptr(), p);
                            }
                            fmt_action = FMT_DONE;
                        }
                        Character::LowerQ => {
                            if form[2_usize] as i32 != Character::Null as i32 {
                                return lual_error(
                                    state,
                                    c"specifier '%%q' cannot have modifiers".as_ptr(),
                                    &[],
                                );
                            }
                            addliteral(state, &mut b, arg);
                            fmt_action = FMT_DONE;
                        }
                        Character::LowerS => {
                            let mut l: usize = 0;
                            let s: *const i8 = lual_tolstring(state, arg, &mut l);
                            if form[2_usize] as i32 == Character::Null as i32 {
                                b.add_value();
                            } else {
                                if l != cstr_len(s) {
                                    lual_argerror(state, arg, c"string contains zeros".as_ptr());
                                    0;
                                }
                                checkformat(state, form.as_mut_ptr(), c"-".as_ptr(), 1);
                                if (cstr_chr(form.as_mut_ptr(), Character::Period as i8)).is_null() && l >= 100_usize {
                                    b.add_value();
                                } else {
                                    nb = sprintf_str(buffer, maxitem as usize, form.as_mut_ptr(), s);
                                    lua_settop(state, -2);
                                }
                            }
                            fmt_action = FMT_DONE;
                        }
                        _ => {
                            return lual_error(
                                state,
                                c"invalid conversion '%s' to 'format'".as_ptr(),
                                &[form.as_mut_ptr().into()],
                            );
                        }
                    }
                    match fmt_action {
                        FMT_INTEGER => {
                            let n: i64 = lual_checkinteger(state, arg);
                            checkformat(state, form.as_mut_ptr(), flags, 1);
                            nb = sprintf_int(buffer, maxitem as usize, form.as_mut_ptr(), n);
                        }
                        FMT_FLOAT => {
                            let n: f64 = lual_checknumber(state, arg);
                            checkformat(state, form.as_mut_ptr(), c"-+#0 ".as_ptr(), 1);
                            nb = sprintf_float(buffer, maxitem as usize, form.as_mut_ptr(), n);
                        }
                        _ => {}
                    }
                    b.buffer_loads
                        .set_length((b.buffer_loads.get_length() as usize + nb as usize) as usize);
                }
            }
        }
        b.push_result();
        1
    }
}
pub unsafe fn packint(b: *mut Buffer, mut n: usize, islittle: bool, size: i32, is_negative_: i32) {
    unsafe {
        let buffer: *mut i8 = (*b).prepare_with_size(size as usize);
        *buffer.add((if islittle { 0 } else { size - 1 }) as usize) = (n & ((1 << 8) - 1) as usize) as i8;
        for i in 1..size {
            n >>= 8;
            *buffer.add((if islittle { i } else { size - 1 - i }) as usize) = (n & ((1 << 8) - 1) as usize) as i8;
        }
        if is_negative_ != 0 && size > size_of::<i64>() as i32 {
            for i in (size_of::<i64>() as i32)..size {
                *buffer.add((if islittle { i } else { size - 1 - i }) as usize) = ((1 << 8) - 1) as i8;
            }
        }
        (*b).buffer_loads
            .set_length(((*b).buffer_loads.get_length() as usize + size as usize) as usize);
    }
}
pub unsafe fn copywithendian(mut dest: *mut i8, mut src: *const i8, mut size: i32, islittle: bool) {
    unsafe {
        if islittle as i32 == NATIVE_ENDIAN.nativeendian_little as i32 {
            std::ptr::copy_nonoverlapping(src as *const u8, dest as *mut u8, size as usize);
        } else {
            dest = dest.add((size - 1) as usize);
            loop {
                let prev_size = size;
                size -= 1;
                if prev_size == 0 {
                    break;
                }
                let current_src = src;
                src = src.add(1);
                let current_dest = dest;
                dest = dest.sub(1);
                *current_dest = *current_src;
            }
        };
    }
}
pub unsafe fn str_pack(state: *mut State) -> i32 {
    unsafe {
        let mut b = Buffer::new();
        let mut h: Header = Header::new(state);
        let mut fmt: *const i8 = lual_checklstring(state, 1, null_mut());
        let mut arg: i32 = 1;
        let mut totalsize: usize = 0;
        (*state).push_nil();
        b.initialize(state);
        while *fmt as i32 != Character::Null as i32 {
            let mut size: usize = 0;
            let mut ntoalign: usize = 0;
            let opt: PackingType = h.getdetails(totalsize, &mut fmt, &mut size, &mut ntoalign);
            if size + ntoalign > MAXIMUM_SIZE - totalsize {
                lual_argerror(state, 1, c"result too long".as_ptr());
                0;
            }
            totalsize += ntoalign + size;
            while ntoalign > 0 {
                ntoalign -= 1;
                (b.buffer_loads.get_length() < b.buffer_loads.get_size() || !(b.prepare_with_size(1)).is_null()) as i32;
                let write_offset = b.buffer_loads.get_length();
                b.buffer_loads
                    .set_length((b.buffer_loads.get_length() + 1) as usize);
                *(b.buffer_loads.loads_pointer).add(write_offset as usize) = 0_i8;
            }
            arg += 1;
            const PACK_DONE: usize = 0;
            const PACK_NO_ARG: usize = 1;
            let pack_action: usize;
            match opt {
                PackingType::Integer => {
                    let n: i64 = lual_checkinteger(state, arg);
                    if (size as i32) < size_of::<i64>() as i32 {
                        let lim: i64 = 1 << (size as i32 * 8 - 1);
                        if !(-lim <= n && n < lim) {
                            lual_argerror(state, arg, c"integer overflow".as_ptr());
                            0;
                        }
                    }
                    packint(
                        &mut b,
                        n as usize,
                        h.is_little_endian(),
                        size as i32,
                        (n < 0) as i32,
                    );
                    pack_action = PACK_DONE;
                }
                PackingType::Unsigned => {
                    let n: i64 = lual_checkinteger(state, arg);
                    if (size as i32) < size_of::<i64>() as i32 && (n as usize) >= 1_usize << (size as i32 * 8) {
                        lual_argerror(state, arg, c"unsigned overflow".as_ptr());
                        0;
                    }
                    packint(&mut b, n as usize, h.is_little_endian(), size as i32, 0);
                    pack_action = PACK_DONE;
                }
                PackingType::Float => {
                    let mut f: f32 = lual_checknumber(state, arg) as f32;
                    let buffer: *mut i8 = b.prepare_with_size(size_of::<f32>());
                    copywithendian(
                        buffer,
                        &mut f as *mut f32 as *mut i8,
                        size_of::<f32>() as i32,
                        h.is_little_endian(),
                    );
                    b.buffer_loads
                        .set_length(b.buffer_loads.get_length() as usize + size);
                    pack_action = PACK_DONE;
                }
                PackingType::Number => {
                    let mut f: f64 = lual_checknumber(state, arg);
                    let buffer: *mut i8 = b.prepare_with_size(size_of::<f64>());
                    copywithendian(
                        buffer,
                        &mut f as *mut f64 as *mut i8,
                        size_of::<f64>() as i32,
                        h.is_little_endian(),
                    );
                    b.buffer_loads
                        .set_length(b.buffer_loads.get_length() as usize + size);
                    pack_action = PACK_DONE;
                }
                PackingType::Double => {
                    let mut f: f64 = lual_checknumber(state, arg);
                    let buffer: *mut i8 = b.prepare_with_size(size_of::<f64>());
                    copywithendian(
                        buffer,
                        &mut f as *mut f64 as *mut i8,
                        size_of::<f64>() as i32,
                        h.is_little_endian(),
                    );
                    b.buffer_loads
                        .set_length(b.buffer_loads.get_length() as usize + size);
                    pack_action = PACK_DONE;
                }
                PackingType::Character => {
                    let mut length: usize = 0;
                    let s: *const i8 = lual_checklstring(state, arg, &mut length);
                    if length > size {
                        lual_argerror(state, arg, c"string longer than given size".as_ptr());
                        0;
                    }
                    b.add_string_with_length(s, length);
                    loop {
                        let prev_length = length;
                        length += 1;
                        if prev_length >= size {
                            break;
                        }
                        (b.buffer_loads.get_length() < b.buffer_loads.get_size() || !(b.prepare_with_size(1)).is_null()) as i32;
                        let write_offset = b.buffer_loads.get_length();
                        b.buffer_loads
                            .set_length((b.buffer_loads.get_length() + 1) as usize);
                        *(b.buffer_loads.loads_pointer).add(write_offset as usize) = 0_i8;
                    }
                    pack_action = PACK_DONE;
                }
                PackingType::String => {
                    let mut length: usize = 0;
                    let s: *const i8 = lual_checklstring(state, arg, &mut length);
                    if !(size >= size_of::<usize>() || length < 1_usize << ((size as i32) * 8)) {
                        lual_argerror(
                            state,
                            arg,
                            c"string length does not fit in given size".as_ptr(),
                        );
                        0;
                    }
                    packint(&mut b, length, h.is_little_endian(), size as i32, 0);
                    b.add_string_with_length(s, length);
                    totalsize += length;
                    pack_action = PACK_DONE;
                }
                PackingType::ZString => {
                    let mut length: usize = 0;
                    let s: *const i8 = lual_checklstring(state, arg, &mut length);
                    if cstr_len(s) != length {
                        lual_argerror(state, arg, c"string contains zeros".as_ptr());
                        0;
                    }
                    b.add_string_with_length(s, length);
                    (b.buffer_loads.get_length() < b.buffer_loads.get_size() || !(b.prepare_with_size(1)).is_null()) as i32;
                    let write_offset = b.buffer_loads.get_length();
                    b.buffer_loads
                        .set_length((b.buffer_loads.get_length() + 1) as usize);
                    *(b.buffer_loads.loads_pointer).add(write_offset as usize) = Character::Null as i8;
                    totalsize += length + 1;
                    pack_action = PACK_DONE;
                }
                PackingType::Padding => {
                    (b.buffer_loads.get_length() < b.buffer_loads.get_size() || !(b.prepare_with_size(1)).is_null()) as i32;
                    let write_offset = b.buffer_loads.get_length();
                    b.buffer_loads
                        .set_length((b.buffer_loads.get_length() + 1) as usize);
                    *(b.buffer_loads.loads_pointer).add(write_offset as usize) = 0_i8;
                    pack_action = PACK_NO_ARG;
                }
                PackingType::PaddingAlignment | PackingType::NoOperator => {
                    pack_action = PACK_NO_ARG;
                }
            }
            match pack_action {
                PACK_NO_ARG => {
                    arg -= 1;
                }
                _ => {}
            }
        }
        b.push_result();
        1
    }
}
pub unsafe fn str_packsize(state: *mut State) -> i32 {
    unsafe {
        let mut h: Header = Header::new(state);
        let mut fmt: *const i8 = lual_checklstring(state, 1, null_mut());
        let mut totalsize: usize = 0;
        while *fmt as i32 != Character::Null as i32 {
            let mut size: usize = 0;
            let mut ntoalign: usize = 0;
            let opt: PackingType = h.getdetails(totalsize, &mut fmt, &mut size, &mut ntoalign);
            if !(opt as u32 != PackingType::String as u32 && opt as u32 != PackingType::ZString as u32) {
                lual_argerror(state, 1, c"variable-length format".as_ptr());
                0;
            }
            size += ntoalign;
            if totalsize > MAXIMUM_SIZE - size {
                lual_argerror(state, 1, c"format result too large".as_ptr());
                0;
            }
            totalsize += size;
        }
        (*state).push_integer(totalsize as i64);
        1
    }
}
pub unsafe fn unpackint(state: *mut State, str: *const i8, islittle: bool, size: i32, issigned: i32) -> i64 {
    unsafe {
        let mut res: u64 = 0;
        let mut i: i32;
        let limit: i32 = if size <= size_of::<i64>() as i32 {
            size
        } else {
            size_of::<i64>() as i32
        };
        i = limit - 1;
        while i >= 0 {
            res <<= 8;
            res |= *str.add((if islittle { i } else { size - 1 - i }) as usize) as u8 as u64;
            i -= 1;
        }
        if size < size_of::<i64>() as i32 {
            if issigned != 0 {
                let mask: u64 = 1u64 << (size * 8 - 1);
                res = (res ^ mask).wrapping_sub(mask);
            }
        } else if size > size_of::<i64>() as i32 {
            let fill_mask: i32 = if issigned == 0 || res as i64 >= 0 {
                0
            } else {
                (1 << 8) - 1
            };
            for i in limit..size {
                if *str.add((if islittle { i } else { size - 1 - i }) as usize) as u8 as i32 != fill_mask {
                    lual_error(
                        state,
                        c"%d-byte integer does not fit into Lua Integer".as_ptr(),
                        &[size.into()],
                    );
                }
            }
        }
        res as i64
    }
}
pub unsafe fn str_unpack(state: *mut State) -> i32 {
    unsafe {
        let mut h: Header = Header::new(state);
        let mut fmt: *const i8 = lual_checklstring(state, 1, null_mut());
        let mut ld: usize = 0;
        let data: *const i8 = lual_checklstring(state, 2, &mut ld);
        let mut position: usize = get_position_relative(lual_optinteger(state, 3, 1_i64), ld) - 1;
        let mut n: i32 = 0;
        if position > ld {
            lual_argerror(state, 3, c"initial position out of string".as_ptr());
            0;
        }
        while *fmt as i32 != Character::Null as i32 {
            let mut size: usize = 0;
            let mut ntoalign: usize = 0;
            let opt: PackingType = h.getdetails(position, &mut fmt, &mut size, &mut ntoalign);
            if ntoalign + size > ld - position {
                lual_argerror(state, 2, c"data string too short".as_ptr());
                0;
            }
            position += ntoalign;
            lual_checkstack(state, 2, c"too many results".as_ptr());
            n += 1;
            match opt {
                PackingType::Integer | PackingType::Unsigned => {
                    let res: i64 = unpackint(
                        state,
                        data.add(position),
                        h.is_little_endian(),
                        size as i32,
                        (opt == PackingType::Integer) as i32,
                    );
                    (*state).push_integer(res);
                }
                PackingType::Float => {
                    let mut f: f32 = 0.0;
                    copywithendian(
                        &mut f as *mut f32 as *mut i8,
                        data.add(position),
                        size_of::<f32>() as i32,
                        h.is_little_endian(),
                    );
                    (*state).push_number(f as f64);
                }
                PackingType::Number => {
                    let mut f: f64 = 0.0;
                    copywithendian(
                        &mut f as *mut f64 as *mut i8,
                        data.add(position),
                        size_of::<f64>() as i32,
                        h.is_little_endian(),
                    );
                    (*state).push_number(f);
                }
                PackingType::Double => {
                    let mut f: f64 = 0.0;
                    copywithendian(
                        &mut f as *mut f64 as *mut i8,
                        data.add(position),
                        size_of::<f64>() as i32,
                        h.is_little_endian(),
                    );
                    (*state).push_number(f);
                }
                PackingType::Character => {
                    lua_pushlstring(state, data.add(position), size);
                }
                PackingType::String => {
                    let length: usize = unpackint(
                        state,
                        data.add(position),
                        h.is_little_endian(),
                        size as i32,
                        0,
                    ) as usize;
                    if length > ld - position - size {
                        lual_argerror(state, 2, c"data string too short".as_ptr());
                        0;
                    }
                    lua_pushlstring(state, data.add(position).add(size), length);
                    position += length;
                }
                PackingType::ZString => {
                    let length: usize = cstr_len(data.add(position));
                    if position + length >= ld {
                        lual_argerror(state, 2, c"unfinished string for format 'zio'".as_ptr());
                        0;
                    }
                    lua_pushlstring(state, data.add(position), length);
                    position += length + 1;
                }
                PackingType::PaddingAlignment | PackingType::Padding | PackingType::NoOperator => {
                    n -= 1;
                }
            }
            position += size;
        }
        (*state).push_integer((position + 1) as i64);
        n + 1
    }
}
pub const STRING_FUNCTIONS: [RegisteredFunction; 17] = {
    [
        {
            RegisteredFunction {
                registeredfunction_name: c"byte".as_ptr(),
                registeredfunction_function: Some(str_byte as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"char".as_ptr(),
                registeredfunction_function: Some(str_char as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"dump".as_ptr(),
                registeredfunction_function: Some(StreamWriter::str_dump as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"find".as_ptr(),
                registeredfunction_function: Some(str_find as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"format".as_ptr(),
                registeredfunction_function: Some(str_format as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"gmatch".as_ptr(),
                registeredfunction_function: Some(GMatchState::gmatch as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"gsub".as_ptr(),
                registeredfunction_function: Some(str_gsub as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"len".as_ptr(),
                registeredfunction_function: Some(str_len as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"lower".as_ptr(),
                registeredfunction_function: Some(str_lower as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"match".as_ptr(),
                registeredfunction_function: Some(str_match as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"rep".as_ptr(),
                registeredfunction_function: Some(str_rep as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"reverse".as_ptr(),
                registeredfunction_function: Some(str_reverse as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"sub".as_ptr(),
                registeredfunction_function: Some(str_sub as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"upper".as_ptr(),
                registeredfunction_function: Some(str_upper as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"pack".as_ptr(),
                registeredfunction_function: Some(str_pack as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"packsize".as_ptr(),
                registeredfunction_function: Some(str_packsize as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"unpack".as_ptr(),
                registeredfunction_function: Some(str_unpack as unsafe fn(*mut State) -> i32),
            }
        },
    ]
};
pub unsafe fn createmetatable(state: *mut State) {
    unsafe {
        (*state).lua_createtable();
        lual_setfuncs(
            state,
            STRING_METAMETHODS.as_ptr(),
            STRING_METAMETHODS.len(),
            0,
        );
        lua_pushstring(state, c"".as_ptr());
        lua_pushvalue(state, -2);
        lua_setmetatable(state, -2);
        lua_settop(state, -2);
        lua_pushvalue(state, -2);
        lua_setfield(state, -2, c"__index".as_ptr());
        lua_settop(state, -2);
    }
}
pub unsafe fn luaopen_string(state: *mut State) -> i32 {
    unsafe {
        (*state).lua_createtable();
        lual_setfuncs(state, STRING_FUNCTIONS.as_ptr(), STRING_FUNCTIONS.len(), 0);
        createmetatable(state);
        1
    }
}
