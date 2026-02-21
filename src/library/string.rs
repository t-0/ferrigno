use crate::buffer::*;
use crate::character::*;
use crate::gmatchstate::*;
use crate::header::*;
use crate::interpreter::*;
use crate::k::*;
use crate::matchstate::*;
use crate::nativeendian::*;
use crate::registeredfunction::*;
use crate::streamwriter::*;
use crate::tagtype::*;
use crate::tdefaultnew::*;
use crate::tm::*;
use crate::tstring::*;
use crate::tvalue::*;
use crate::utility::*;
use libc::{memchr, tolower, toupper};
use std::ptr::*;
pub unsafe fn str_len(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let mut l: usize = 0;
        lual_checklstring(interpreter, 1, &mut l);
        (*interpreter).push_integer(l as i64);
        return 1;
    }
}
pub unsafe fn str_sub(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let mut l: usize = 0;
        let s: *const i8 = lual_checklstring(interpreter, 1, &mut l);
        let start: usize = get_position_relative(lual_checkinteger(interpreter, 2), l);
        let end: usize = get_position_end(interpreter, 3, -1 as i64, l);
        if start <= end {
            lua_pushlstring(
                interpreter,
                s.offset(start as isize).offset(-(1 as isize)),
                end.wrapping_sub(start).wrapping_add(1 as usize),
            );
        } else {
            lua_pushstring(interpreter, c"".as_ptr());
        }
        return 1;
    }
}
pub unsafe fn str_reverse(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let mut l: usize = 0;
        let mut b = Buffer::new();
        let s: *const i8 = lual_checklstring(interpreter, 1, &mut l);
        let p: *mut i8 = b.initialize_with_size(interpreter, l as usize);
        for i in 0..l {
            *p.offset(i as isize) = *s.offset(l.wrapping_sub(i).wrapping_sub(1 as usize) as isize);
        }
        b.push_result_with_size(l as usize);
        return 1;
    }
}
pub unsafe fn str_lower(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let mut l: usize = 0;
        let mut b = Buffer::new();
        let s: *const i8 = lual_checklstring(interpreter, 1, &mut l);
        let p: *mut i8 = b.initialize_with_size(interpreter, l as usize);
        for i in 0..l {
            *p.offset(i as isize) = tolower(*s.offset(i as isize) as u8 as i32) as i8;
        }
        b.push_result_with_size(l as usize);
        return 1;
    }
}
pub unsafe fn str_upper(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let mut l: usize = 0;
        let mut b = Buffer::new();
        let s: *const i8 = lual_checklstring(interpreter, 1, &mut l);
        let p: *mut i8 = b.initialize_with_size(interpreter, l as usize);
        for i in 0..l {
            *p.offset(i as isize) = toupper(*s.offset(i as isize) as u8 as i32) as i8;
        }
        b.push_result_with_size(l as usize);
        return 1;
    }
}
pub unsafe fn str_rep(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let mut l: usize = 0;
        let mut lsep: usize = 0;
        let s: *const i8 = lual_checklstring(interpreter, 1, &mut l);
        let mut n: i64 = lual_checkinteger(interpreter, 2);
        let sep: *const i8 = lual_optlstring(interpreter, 3, c"".as_ptr(), &mut lsep);
        if n <= 0 {
            lua_pushstring(interpreter, c"".as_ptr());
        } else if l.wrapping_add(lsep) < l
            || l.wrapping_add(lsep) as usize
                > ((if (size_of::<usize>() as usize) < size_of::<i32>() as usize {
                    !(0usize)
                } else {
                    0x7FFFFFFF as usize
                }) as usize)
                    / n as usize
        {
            return lual_error(interpreter, c"resulting string too large".as_ptr());
        } else {
            let totallen: usize = (n as usize).wrapping_mul(l).wrapping_add(((n - 1) as usize) * lsep);
            let mut b = Buffer::new();
            let mut p: *mut i8 = b.initialize_with_size(interpreter, totallen as usize);
            loop {
                let fresh = n;
                n = n - 1;
                if !(fresh > 1) {
                    break;
                }
                libc::memcpy(p as *mut libc::c_void, s as *const libc::c_void, l);
                p = p.offset(l as isize);
                if lsep > 0 {
                    libc::memcpy(p as *mut libc::c_void, sep as *const libc::c_void, lsep);
                    p = p.offset(lsep as isize);
                }
            }
            libc::memcpy(p as *mut libc::c_void, s as *const libc::c_void, l);
            b.push_result_with_size(totallen as usize);
        }
        return 1;
    }
}
pub unsafe fn str_byte(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let mut l: usize = 0;
        let s: *const i8 = lual_checklstring(interpreter, 1, &mut l);
        let pi: i64 = lual_optinteger(interpreter, 2, 1);
        let posi: usize = get_position_relative(pi, l);
        let pose: usize = get_position_end(interpreter, 3, pi, l);
        let n: i32;
        if posi > pose {
            return 0;
        }
        if pose.wrapping_sub(posi) >= 0x7FFFFFFF as usize {
            return lual_error(interpreter, c"string slice too long".as_ptr());
        }
        n = pose.wrapping_sub(posi) as i32 + 1;
        lual_checkstack(interpreter, n, c"string slice too long".as_ptr());
        for i in 0..n {
            (*interpreter).push_integer(*s.offset(posi.wrapping_add(i as usize).wrapping_sub(1 as usize) as isize) as u8 as i64);
        }
        return n;
    }
}
pub unsafe fn str_char(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let n: i32 = (*interpreter).get_top();
        let mut buffer = Buffer::new();
        let p: *mut i8 = buffer.initialize_with_size(interpreter, n as usize);
        for i in 1..(1 + n) {
            let c: usize = lual_checkinteger(interpreter, i) as usize;
            (((c <= (127 as i32 * 2 + 1) as usize) as i32 != 0) as i64 != 0
                || lual_argerror(interpreter, i, c"value out of range".as_ptr()) != 0) as i32;
            *p.offset((i - 1) as isize) = c as u8 as i8;
        }
        buffer.push_result_with_size(n as usize);
        return 1;
    }
}
pub unsafe fn tonum(interpreter: *mut Interpreter, arg: i32) -> i32 {
    unsafe {
        if lua_type(interpreter, arg) == Some(TagType::Numeric) {
            lua_pushvalue(interpreter, arg);
            return 1;
        } else {
            let mut length: usize = 0;
            let s: *const i8 = lua_tolstring(interpreter, arg, &mut length);
            return (!s.is_null() && lua_stringtonumber(interpreter, s) == length.wrapping_add(1 as usize)) as i32;
        };
    }
}
pub unsafe fn trymt(interpreter: *mut Interpreter, mtname: *const i8) {
    unsafe {
        lua_settop(interpreter, 2);
        if lua_type(interpreter, 2) == Some(TagType::String) || lual_getmetafield(interpreter, 2, mtname) == TagType::Nil {
            lual_error(
                interpreter,
                c"attempt to %s a '%s' with a '%s'".as_ptr(),
                mtname.offset(2 as isize),
                lua_typename(interpreter, lua_type(interpreter, -2)),
                lua_typename(interpreter, lua_type(interpreter, -1)),
            );
        }
        lua_rotate(interpreter, -3, 1);
        (*interpreter).lua_callk(2, 1, 0, None);
    }
}
pub unsafe fn arith(interpreter: *mut Interpreter, op: i32, mtname: *const i8) -> i32 {
    unsafe {
        if tonum(interpreter, 1) != 0 && tonum(interpreter, 2) != 0 {
            if !(op != 12 as i32 && op != 13 as i32) {
                let io1: *mut TValue = &mut (*(*interpreter).interpreter_top.stkidrel_pointer);
                let io2: *const TValue = &mut (*(*interpreter).interpreter_top.stkidrel_pointer.offset(-(1 as isize)));
                (*io1).copy_from(&*io2);
                (*interpreter).interpreter_top.stkidrel_pointer = (*interpreter).interpreter_top.stkidrel_pointer.offset(1);
            }
            let p1 = &mut (*(*interpreter).interpreter_top.stkidrel_pointer.offset(-(2 as isize)));
            let p2 = &mut (*(*interpreter).interpreter_top.stkidrel_pointer.offset(-(1 as isize)));
            let res = (*interpreter).interpreter_top.stkidrel_pointer.offset(-(2 as isize));
            if luao_rawarith(interpreter, op, p1, p2, &mut (*res)) == 0 {
                luat_trybintm(interpreter, p1, p2, res, (op - 0 + TM_ADD as i32) as u32);
            }
            (*interpreter).interpreter_top.stkidrel_pointer = (*interpreter).interpreter_top.stkidrel_pointer.offset(-1);
        } else {
            trymt(interpreter, mtname);
        }
        return 1;
    }
}
pub unsafe fn arith_add(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        return arith(interpreter, 0, c"__add".as_ptr());
    }
}
pub unsafe fn arith_sub(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        return arith(interpreter, 1, c"__sub".as_ptr());
    }
}
pub unsafe fn arith_mul(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        return arith(interpreter, 2, c"__mul".as_ptr());
    }
}
pub unsafe fn arith_mod(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        return arith(interpreter, 3, c"__mod".as_ptr());
    }
}
pub unsafe fn arith_pow(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        return arith(interpreter, 4, c"__pow".as_ptr());
    }
}
pub unsafe fn arith_div(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        return arith(interpreter, 5, c"__div".as_ptr());
    }
}
pub unsafe fn arith_idiv(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        return arith(interpreter, 6, c"__idiv".as_ptr());
    }
}
pub unsafe fn arith_unm(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        return arith(interpreter, 12 as i32, c"__unm".as_ptr());
    }
}
pub const STRING_METAMETHODS: [RegisteredFunction; 8] = {
    [
        {
            RegisteredFunction {
                registeredfunction_name: c"__add".as_ptr(),
                registeredfunction_function: Some(arith_add as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"__sub".as_ptr(),
                registeredfunction_function: Some(arith_sub as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"__mul".as_ptr(),
                registeredfunction_function: Some(arith_mul as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"__mod".as_ptr(),
                registeredfunction_function: Some(arith_mod as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"__pow".as_ptr(),
                registeredfunction_function: Some(arith_pow as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"__div".as_ptr(),
                registeredfunction_function: Some(arith_div as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"__idiv".as_ptr(),
                registeredfunction_function: Some(arith_idiv as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"__unm".as_ptr(),
                registeredfunction_function: Some(arith_unm as unsafe fn(*mut Interpreter) -> i32),
            }
        },
    ]
};
pub unsafe fn lmemfind(mut s1: *const i8, mut l1: usize, s2: *const i8, mut l2: usize) -> *const i8 {
    unsafe {
        if l2 == 0 {
            return s1;
        } else if l2 > l1 {
            return null();
        } else {
            let mut initial: *const i8;
            l2 = l2.wrapping_sub(1);
            l1 = l1.wrapping_sub(l2);
            while l1 > 0 && {
                initial = memchr(s1 as *const libc::c_void, *s2 as i32, l1 as usize) as *const i8;
                !initial.is_null()
            } {
                initial = initial.offset(1);
                if libc::memcmp(
                    initial as *const libc::c_void,
                    s2.offset(1 as isize) as *const libc::c_void,
                    l2 as usize,
                ) == 0
                {
                    return initial.offset(-(1 as isize));
                } else {
                    l1 = (l1 as usize).wrapping_sub(initial.offset_from(s1) as usize) as usize;
                    s1 = initial;
                }
            }
            return null();
        };
    }
}
pub unsafe fn nospecials(p: *const i8, l: usize) -> i32 {
    unsafe {
        let mut upto: usize = 0;
        loop {
            if !(libc::strpbrk(p.offset(upto as isize), c"^$*+?.([%-".as_ptr())).is_null() {
                return 0;
            }
            upto = upto.wrapping_add((libc::strlen(p.offset(upto as isize)) as usize).wrapping_add(1));
            if !(upto <= l) {
                break;
            }
        }
        return 1;
    }
}
pub unsafe fn str_find_aux(interpreter: *mut Interpreter, find: i32) -> i32 {
    unsafe {
        let mut lexical_state: usize = 0;
        let mut lp: usize = 0;
        let s: *const i8 = lual_checklstring(interpreter, 1, &mut lexical_state);
        let mut p: *const i8 = lual_checklstring(interpreter, 2, &mut lp);
        let initial: usize =
            (get_position_relative(lual_optinteger(interpreter, 3, 1 as i64), lexical_state)).wrapping_sub(1 as usize);
        if initial > lexical_state {
            (*interpreter).push_nil();
            return 1;
        }
        if find != 0 && (lua_toboolean(interpreter, 4) || nospecials(p, lp) != 0) {
            let s2: *const i8 = lmemfind(s.offset(initial as isize), lexical_state.wrapping_sub(initial), p, lp);
            if !s2.is_null() {
                (*interpreter).push_integer((s2.offset_from(s) as i64 + 1) as i64);
                (*interpreter).push_integer((s2.offset_from(s) as usize).wrapping_add(lp) as i64);
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
                capture: [MatchStateCapture { matchstatecapture_initial: null(), matchstatecapture_length: 0 }; 32],
            };
            let mut s1: *const i8 = s.offset(initial as isize);
            let anchor: i32 = (*p as i32 == Character::Caret as i32) as i32;
            if anchor != 0 {
                p = p.offset(1);
                lp = lp.wrapping_sub(1);
            }
            match_state.prepstate(interpreter, s, lexical_state, p, lp);
            loop {
                let res: *const i8;
                match_state.reprepstate();
                res = match_state.match_0(s1, p);
                if !res.is_null() {
                    if find != 0 {
                        (*interpreter).push_integer((s1.offset_from(s) as i64 + 1) as i64);
                        (*interpreter).push_integer(res.offset_from(s) as i64);
                        return match_state.push_captures(null(), null()) + 2;
                    } else {
                        return match_state.push_captures(s1, res);
                    }
                }
                let fresh163 = s1;
                s1 = s1.offset(1);
                if !(fresh163 < match_state.src_end && anchor == 0) {
                    break;
                }
            }
        }
        (*interpreter).push_nil();
        return 1;
    }
}
pub unsafe fn str_find(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        return str_find_aux(interpreter, 1);
    }
}
pub unsafe fn str_match(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        return str_find_aux(interpreter, 0);
    }
}
pub unsafe fn str_gsub(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let mut srcl: usize = 0;
        let mut lp: usize = 0;
        let mut src: *const i8 = lual_checklstring(interpreter, 1, &mut srcl);
        let mut p: *const i8 = lual_checklstring(interpreter, 2, &mut lp);
        let mut lastmatch: *const i8 = null();
        let tr = lua_type(interpreter, 3);
        let max_s: i64 = lual_optinteger(interpreter, 4, srcl.wrapping_add(1 as usize) as i64);
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
            capture: [MatchStateCapture { matchstatecapture_initial: null(), matchstatecapture_length: 0 }; 32],
        };
        let mut b = Buffer::new();
        (((tr == Some(TagType::Numeric)
            || tr == Some(TagType::String)
            || tr == Some(TagType::Closure)
            || tr == Some(TagType::Table)) as i32
            != 0) as i64
            != 0
            || lual_typeerror(interpreter, 3, c"string/function/table".as_ptr()) != 0) as i32;
        b.initialize(interpreter);
        if anchor != 0 {
            p = p.offset(1);
            lp = lp.wrapping_sub(1);
        }
        match_state.prepstate(interpreter, src, srcl, p, lp);
        while n < max_s {
            let e: *const i8;
            match_state.reprepstate();
            e = match_state.match_0(src, p);
            if !e.is_null() && e != lastmatch {
                n += 1;
                changed = match_state.add_value(&mut b, src, e, tr.unwrap()) | changed;
                lastmatch = e;
                src = lastmatch;
            } else {
                if !(src < match_state.src_end) {
                    break;
                }
                (b.buffer_loads.get_length() < b.buffer_loads.get_size() || !(b.prepare_with_size(1)).is_null()) as i32;
                let fresh165 = src;
                src = src.offset(1);
                let fresh166 = b.buffer_loads.get_length();
                b.buffer_loads
                    .set_length(((b.buffer_loads.get_length()).wrapping_add(1)) as usize);
                *(b.buffer_loads.loads_pointer).offset(fresh166 as isize) = *fresh165;
            }
            if anchor != 0 {
                break;
            }
        }
        if changed == 0 {
            lua_pushvalue(interpreter, 1);
        } else {
            b.add_string_with_length(src, (match_state.src_end).offset_from(src) as usize);
            b.push_result();
        }
        (*interpreter).push_integer(n);
        return 2;
    }
}
pub unsafe fn addquoted(b: *mut Buffer, mut s: *const i8, mut length: usize) {
    unsafe {
        ((*b).buffer_loads.get_length() < (*b).buffer_loads.get_size() || !((*b).prepare_with_size(1)).is_null()) as i32;
        let fresh167 = (*b).buffer_loads.get_length();
        (*b).buffer_loads
            .set_length((((*b).buffer_loads.get_length()).wrapping_add(1)) as usize);
        *((*b).buffer_loads.loads_pointer).offset(fresh167 as isize) = '"' as i8;
        loop {
            let fresh168 = length;
            length = length.wrapping_sub(1);
            if !(fresh168 != 0) {
                break;
            }
            if *s as i32 == '"' as i32 || *s as i32 == Character::Backslash as i32 || *s as i32 == Character::LineFeed as i32 {
                ((*b).buffer_loads.get_length() < (*b).buffer_loads.get_size() || !((*b).prepare_with_size(1)).is_null()) as i32;
                let fresh169 = (*b).buffer_loads.get_length();
                (*b).buffer_loads
                    .set_length((((*b).buffer_loads.get_length()).wrapping_add(1)) as usize);
                *((*b).buffer_loads.loads_pointer).offset(fresh169 as isize) = Character::Backslash as i8;
                ((*b).buffer_loads.get_length() < (*b).buffer_loads.get_size() || !((*b).prepare_with_size(1)).is_null()) as i32;
                let fresh170 = (*b).buffer_loads.get_length();
                (*b).buffer_loads
                    .set_length((((*b).buffer_loads.get_length()).wrapping_add(1)) as usize);
                *((*b).buffer_loads.loads_pointer).offset(fresh170 as isize) = *s;
            } else if Character::from(*s as u8 as i32).is_control() {
                let mut buffer: [i8; 10] = [0; 10];
                if Character::from(*s.offset(1 as isize) as u8 as i32).is_digit_decimal() {
                    libc::snprintf(buffer.as_mut_ptr(), size_of::<[i8; 10]>(), c"\\%03d".as_ptr(), *s as u8 as i32);
                } else {
                    libc::snprintf(buffer.as_mut_ptr(), size_of::<[i8; 10]>(), c"\\%d".as_ptr(), *s as u8 as i32);
                }
                (*b).add_string(buffer.as_mut_ptr());
            } else {
                ((*b).buffer_loads.get_length() < (*b).buffer_loads.get_size() || !((*b).prepare_with_size(1)).is_null()) as i32;
                let fresh171 = (*b).buffer_loads.get_length();
                (*b).buffer_loads
                    .set_length((((*b).buffer_loads.get_length()).wrapping_add(1)) as usize);
                *((*b).buffer_loads.loads_pointer).offset(fresh171 as isize) = *s;
            }
            s = s.offset(1);
        }
        ((*b).buffer_loads.get_length() < (*b).buffer_loads.get_size() || !((*b).prepare_with_size(1)).is_null()) as i32;
        let fresh172 = (*b).buffer_loads.get_length();
        (*b).buffer_loads
            .set_length((((*b).buffer_loads.get_length()).wrapping_add(1)) as usize);
        *((*b).buffer_loads.loads_pointer).offset(fresh172 as isize) = '"' as i8;
    }
}
pub unsafe fn quotefloat(mut _state: *mut Interpreter, buffer: *mut i8, n: f64) -> i32 {
    unsafe {
        let s: *const i8;
        if n == ::core::f64::INFINITY {
            s = c"1e9999".as_ptr();
        } else if n == -::core::f64::INFINITY {
            s = c"-1e9999".as_ptr();
        } else if n != n {
            s = c"(0/0)".as_ptr();
        } else {
            let nb: i32 = libc::snprintf(buffer, 120, c"%a".as_ptr(), n);
            if (memchr(buffer as *const libc::c_void, Character::Period as i32, nb as usize)).is_null() {
                // locale decimal point may differ from '.'; find and replace it
                let lc = libc::localeconv();
                let locale_dec = if !lc.is_null() && !(*lc).decimal_point.is_null() {
                    *(*lc).decimal_point as i32
                } else {
                    Character::Period as i32
                };
                let ppoint: *mut i8 = memchr(buffer as *const libc::c_void, locale_dec, nb as usize) as *mut i8;
                if !ppoint.is_null() {
                    *ppoint = Character::Period as i8;
                }
            }
            return nb;
        }
        return libc::snprintf(buffer, 120, c"%s".as_ptr(), s);
    }
}
pub unsafe fn addliteral(interpreter: *mut Interpreter, b: *mut Buffer, arg: i32) {
    unsafe {
        match lua_type(interpreter, arg) {
            | Some(TagType::String) => {
                let mut length: usize = 0;
                let s: *const i8 = lua_tolstring(interpreter, arg, &mut length);
                addquoted(b, s, length);
            },
            | Some(TagType::Numeric) => {
                let buffer: *mut i8 = (*b).prepare_with_size(120);
                let nb: i32;
                if lua_isinteger(interpreter, arg) {
                    let n: i64 = lua_tointegerx(interpreter, arg, null_mut());
                    let format: *const i8 = if n == -(MAXIMUM_SIZE as i64) - 1 as i64 {
                        c"0x%llx".as_ptr()
                    } else {
                        c"%lld".as_ptr()
                    };
                    nb = libc::snprintf(buffer, 120, format, n);
                } else {
                    nb = quotefloat(interpreter, buffer, lua_tonumberx(interpreter, arg, null_mut()));
                }
                (*b).buffer_loads
                    .set_length((((*b).buffer_loads.get_length() as usize).wrapping_add(nb as usize) as i32) as usize);
            },
            | Some(TagType::Nil) | Some(TagType::Boolean) => {
                lual_tolstring(interpreter, arg, null_mut());
                (*b).add_value();
            },
            | _ => {
                lual_argerror(interpreter, arg, c"value has no literal form".as_ptr());
            },
        };
    }
}
pub unsafe fn get2digits(mut s: *const i8) -> *const i8 {
    unsafe {
        if Character::from(*s as u8 as i32).is_digit_decimal() {
            s = s.offset(1);
            if Character::from(*s as u8 as i32).is_digit_decimal() {
                s = s.offset(1);
            }
        }
        return s;
    }
}
pub unsafe fn checkformat(interpreter: *mut Interpreter, form: *const i8, flags: *const i8, precision: i32) {
    unsafe {
        let mut spec: *const i8 = form.offset(1 as isize);
        spec = spec.offset(libc::strspn(spec, flags) as isize);
        if *spec as i32 != Character::Digit0 as i32 {
            spec = get2digits(spec);
            if *spec as i32 == Character::Period as i32 && precision != 0 {
                spec = spec.offset(1);
                spec = get2digits(spec);
            }
        }
        if !Character::from(*spec as u8 as i32).is_alpha() {
            lual_error(interpreter, c"invalid conversion specification: '%s'".as_ptr(), form);
        }
    }
}
pub unsafe fn getformat(interpreter: *mut Interpreter, strfrmt: *const i8, mut form: *mut i8) -> *const i8 {
    unsafe {
        let mut length = libc::strspn(strfrmt, c"-+#0 123456789.".as_ptr());
        length = length.wrapping_add(1);
        if length >= 22 {
            lual_error(interpreter, c"invalid format (too long)".as_ptr());
        }
        let fresh173 = form;
        form = form.offset(1);
        *fresh173 = Character::Percent as i8;
        libc::memcpy(form as *mut libc::c_void, strfrmt as *const libc::c_void, length);
        *form.offset(length as isize) = Character::Null as i8;
        return strfrmt.offset(length as isize).offset(-(1 as isize));
    }
}
pub unsafe fn addlenmod(form: *mut i8, lenmod: *const i8) {
    unsafe {
        let length: usize = libc::strlen(form) as usize;
        let mode_length: usize = libc::strlen(lenmod) as usize;
        let spec: i8 = *form.offset(length.wrapping_sub(1 as usize) as isize);
        libc::strcpy(form.offset(length as isize).offset(-(1 as isize)), lenmod);
        *form.offset(length.wrapping_add(mode_length).wrapping_sub(1 as usize) as isize) = spec;
        *form.offset(length.wrapping_add(mode_length) as isize) = Character::Null as i8;
    }
}
pub unsafe fn str_format(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let mut current_block: usize;
        let top: i32 = (*interpreter).get_top();
        let mut arg: i32 = 1;
        let mut sfl: usize = 0;
        let mut strfrmt: *const i8 = lual_checklstring(interpreter, arg, &mut sfl);
        let strfrmt_end: *const i8 = strfrmt.offset(sfl as isize);
        let mut flags: *const i8 = null();
        let mut b = Buffer::new();
        b.initialize(interpreter);
        while strfrmt < strfrmt_end {
            if *strfrmt as i32 != Character::Percent as i32 {
                (b.buffer_loads.get_length() < b.buffer_loads.get_size() || !(b.prepare_with_size(1)).is_null()) as i32;
                let fresh174 = strfrmt;
                strfrmt = strfrmt.offset(1);
                let fresh175 = b.buffer_loads.get_length();
                b.buffer_loads
                    .set_length(((b.buffer_loads.get_length()).wrapping_add(1)) as usize);
                *(b.buffer_loads.loads_pointer).offset(fresh175 as isize) = *fresh174;
            } else {
                strfrmt = strfrmt.offset(1);
                if *strfrmt as i32 == Character::Percent as i32 {
                    (b.buffer_loads.get_length() < b.buffer_loads.get_size() || !(b.prepare_with_size(1)).is_null()) as i32;
                    let fresh176 = strfrmt;
                    strfrmt = strfrmt.offset(1);
                    let fresh177 = b.buffer_loads.get_length();
                    b.buffer_loads
                        .set_length(((b.buffer_loads.get_length()).wrapping_add(1)) as usize);
                    *(b.buffer_loads.loads_pointer).offset(fresh177 as isize) = *fresh176;
                } else {
                    let mut form: [i8; 32] = [0; 32];
                    let mut maxitem: i32 = 120 as i32;
                    let mut buffer: *mut i8 = b.prepare_with_size(maxitem as usize);
                    let mut nb: i32 = 0;
                    arg += 1;
                    if arg > top {
                        return lual_argerror(interpreter, arg, c"no value".as_ptr());
                    }
                    strfrmt = getformat(interpreter, strfrmt, form.as_mut_ptr());
                    let fresh178 = strfrmt;
                    strfrmt = strfrmt.offset(1);
                    match Character::from(*fresh178 as i32) {
                        | Character::LowerC => {
                            checkformat(interpreter, form.as_mut_ptr(), c"-".as_ptr(), 0);
                            nb = libc::snprintf(
                                buffer,
                                maxitem as usize,
                                form.as_mut_ptr(),
                                lual_checkinteger(interpreter, arg) as i32,
                            );
                            current_block = 11793792312832361944;
                        },
                        | Character::LowerD | Character::LowerI => {
                            flags = c"-+0 ".as_ptr();
                            current_block = 5689001924483802034;
                        },
                        | Character::LowerU => {
                            flags = c"-0".as_ptr();
                            current_block = 5689001924483802034;
                        },
                        | Character::LowerO | Character::LowerX | Character::UpperX => {
                            flags = c"-#0".as_ptr();
                            current_block = 5689001924483802034;
                        },
                        | Character::LowerA | Character::UpperA => {
                            checkformat(interpreter, form.as_mut_ptr(), c"-+#0 ".as_ptr(), 1);
                            addlenmod(form.as_mut_ptr(), c"".as_ptr());
                            nb = libc::snprintf(buffer, maxitem as usize, form.as_mut_ptr(), lual_checknumber(interpreter, arg));
                            current_block = 11793792312832361944;
                        },
                        | Character::LowerF => {
                            maxitem = 110 as i32 + 308 as i32;
                            buffer = b.prepare_with_size(maxitem as usize);
                            current_block = 6669252993407410313;
                        },
                        | Character::LowerE | Character::UpperE | Character::LowerG | Character::UpperG => {
                            current_block = 6669252993407410313;
                        },
                        | Character::LowerP => {
                            let mut p: *const libc::c_void = (*interpreter).to_pointer(arg);
                            checkformat(interpreter, form.as_mut_ptr(), c"-".as_ptr(), 0);
                            if p.is_null() {
                                p = c"(null)".as_ptr() as *const libc::c_void;
                                form[(libc::strlen(form.as_mut_ptr())).wrapping_sub(1) as usize] = Character::LowerS as i8;
                            }
                            nb = libc::snprintf(buffer, maxitem as usize, form.as_mut_ptr(), p);
                            current_block = 11793792312832361944;
                        },
                        | Character::LowerQ => {
                            if form[2 as usize] as i32 != Character::Null as i32 {
                                return lual_error(interpreter, c"specifier '%%q' cannot have modifiers".as_ptr());
                            }
                            addliteral(interpreter, &mut b, arg);
                            current_block = 11793792312832361944;
                        },
                        | Character::LowerS => {
                            let mut l: usize = 0;
                            let s: *const i8 = lual_tolstring(interpreter, arg, &mut l);
                            if form[2 as usize] as i32 == Character::Null as i32 {
                                b.add_value();
                            } else {
                                (((l == libc::strlen(s) as usize) as i32 != 0) as i64 != 0
                                    || lual_argerror(interpreter, arg, c"string contains zeros".as_ptr()) != 0)
                                    as i32;
                                checkformat(interpreter, form.as_mut_ptr(), c"-".as_ptr(), 1);
                                if (libc::strchr(form.as_mut_ptr(), Character::Period as i32)).is_null() && l >= 100 as usize {
                                    b.add_value();
                                } else {
                                    nb = libc::snprintf(buffer, maxitem as usize, form.as_mut_ptr(), s);
                                    lua_settop(interpreter, -2);
                                }
                            }
                            current_block = 11793792312832361944;
                        },
                        | _ => {
                            return lual_error(interpreter, c"invalid conversion '%s' to 'format'".as_ptr(), form.as_mut_ptr());
                        },
                    }
                    match current_block {
                        | 5689001924483802034 => {
                            let n: i64 = lual_checkinteger(interpreter, arg);
                            checkformat(interpreter, form.as_mut_ptr(), flags, 1);
                            addlenmod(form.as_mut_ptr(), c"ll".as_ptr());
                            nb = libc::snprintf(buffer, maxitem as usize, form.as_mut_ptr(), n);
                        },
                        | 6669252993407410313 => {
                            let n_0: f64 = lual_checknumber(interpreter, arg);
                            checkformat(interpreter, form.as_mut_ptr(), c"-+#0 ".as_ptr(), 1);
                            addlenmod(form.as_mut_ptr(), c"".as_ptr());
                            nb = libc::snprintf(buffer, maxitem as usize, form.as_mut_ptr(), n_0);
                        },
                        | _ => {},
                    }
                    b.buffer_loads
                        .set_length(((b.buffer_loads.get_length() as usize).wrapping_add(nb as usize) as i32) as usize);
                }
            }
        }
        b.push_result();
        return 1;
    }
}
pub unsafe fn packint(b: *mut Buffer, mut n: usize, islittle: bool, size: i32, is_negative_: i32) {
    unsafe {
        let buffer: *mut i8 = (*b).prepare_with_size(size as usize);
        *buffer.offset((if islittle { 0 } else { size - 1 }) as isize) = (n & ((1 << 8) - 1) as usize) as i8;
        for i in 1..size {
            n >>= 8;
            *buffer.offset((if islittle { i } else { size - 1 - i }) as isize) = (n & ((1 << 8) - 1) as usize) as i8;
        }
        if is_negative_ != 0 && size > size_of::<i64>() as i32 {
            for i in (size_of::<i64>() as i32)..size {
                *buffer.offset((if islittle { i } else { size - 1 - i }) as isize) = ((1 << 8) - 1) as i8;
            }
        }
        (*b).buffer_loads
            .set_length((((*b).buffer_loads.get_length() as usize).wrapping_add(size as usize) as i32) as usize);
    }
}
pub unsafe fn copywithendian(mut dest: *mut i8, mut src: *const i8, mut size: i32, islittle: bool) {
    unsafe {
        if islittle as i32 == NATIVE_ENDIAN.nativeendian_little as i32 {
            libc::memcpy(dest as *mut libc::c_void, src as *const libc::c_void, size as usize);
        } else {
            dest = dest.offset((size - 1) as isize);
            loop {
                let fresh181 = size;
                size = size - 1;
                if !(fresh181 != 0) {
                    break;
                }
                let fresh182 = src;
                src = src.offset(1);
                let fresh183 = dest;
                dest = dest.offset(-1);
                *fresh183 = *fresh182;
            }
        };
    }
}
pub unsafe fn str_pack(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let mut b = Buffer::new();
        let mut h: Header = Header::new(interpreter);
        let mut fmt: *const i8 = lual_checklstring(interpreter, 1, null_mut());
        let mut arg: i32 = 1;
        let mut totalsize: usize = 0;
        (*interpreter).push_nil();
        b.initialize(interpreter);
        while *fmt as i32 != Character::Null as i32 {
            let mut size: i32 = 0;
            let mut ntoalign: i32 = 0;
            let opt: K = h.getdetails(totalsize, &mut fmt, &mut size, &mut ntoalign);
            totalsize = (totalsize as usize).wrapping_add((ntoalign + size) as usize) as usize;
            loop {
                let fresh184 = ntoalign;
                ntoalign = ntoalign - 1;
                if !(fresh184 > 0) {
                    break;
                }
                (b.buffer_loads.get_length() < b.buffer_loads.get_size() || !(b.prepare_with_size(1)).is_null()) as i32;
                let fresh185 = b.buffer_loads.get_length();
                b.buffer_loads
                    .set_length(((b.buffer_loads.get_length()).wrapping_add(1)) as usize);
                *(b.buffer_loads.loads_pointer).offset(fresh185 as isize) = 0 as i8;
            }
            arg += 1;
            let current_block_33: usize;
            match opt as u32 {
                | 0 => {
                    let n: i64 = lual_checkinteger(interpreter, arg);
                    if size < size_of::<i64>() as i32 {
                        let lim: i64 = 1 << size * 8 - 1;
                        (((-lim <= n && n < lim) as i32 != 0) as i64 != 0
                            || lual_argerror(interpreter, arg, c"integer overflow".as_ptr()) != 0) as i32;
                    }
                    packint(&mut b, n as usize, h.is_little_endian(), size, (n < 0) as i32);
                    current_block_33 = 3222590281903869779;
                },
                | 1 => {
                    let n_0: i64 = lual_checkinteger(interpreter, arg);
                    if size < size_of::<i64>() as i32 {
                        ((((n_0 as usize) < (1 as usize) << size * 8) as i32 != 0) as i64 != 0
                            || lual_argerror(interpreter, arg, c"unsigned overflow".as_ptr()) != 0) as i32;
                    }
                    packint(&mut b, n_0 as usize, h.is_little_endian(), size, 0);
                    current_block_33 = 3222590281903869779;
                },
                | 2 => {
                    let mut f: f32 = lual_checknumber(interpreter, arg) as f32;
                    let buffer: *mut i8 = b.prepare_with_size(size_of::<f32>());
                    copywithendian(
                        buffer,
                        &mut f as *mut f32 as *mut i8,
                        size_of::<f32>() as i32,
                        h.is_little_endian(),
                    );
                    b.buffer_loads
                        .set_length(((b.buffer_loads.get_length() as usize).wrapping_add(size as usize) as i32) as usize);
                    current_block_33 = 3222590281903869779;
                },
                | 3 => {
                    let mut f_0: f64 = lual_checknumber(interpreter, arg);
                    let buff_0: *mut i8 = b.prepare_with_size(size_of::<f64>());
                    copywithendian(
                        buff_0,
                        &mut f_0 as *mut f64 as *mut i8,
                        size_of::<f64>() as i32,
                        h.is_little_endian(),
                    );
                    b.buffer_loads
                        .set_length(((b.buffer_loads.get_length() as usize).wrapping_add(size as usize) as i32) as usize);
                    current_block_33 = 3222590281903869779;
                },
                | 4 => {
                    let mut f_1: f64 = lual_checknumber(interpreter, arg);
                    let buff_1: *mut i8 = b.prepare_with_size(size_of::<f64>());
                    copywithendian(
                        buff_1,
                        &mut f_1 as *mut f64 as *mut i8,
                        size_of::<f64>() as i32,
                        h.is_little_endian(),
                    );
                    b.buffer_loads
                        .set_length(((b.buffer_loads.get_length() as usize).wrapping_add(size as usize) as i32) as usize);
                    current_block_33 = 3222590281903869779;
                },
                | 5 => {
                    let mut length: usize = 0;
                    let s: *const i8 = lual_checklstring(interpreter, arg, &mut length);
                    (((length <= size as usize) as i32 != 0) as i64 != 0
                        || lual_argerror(interpreter, arg, c"string longer than given size".as_ptr()) != 0)
                        as i32;
                    b.add_string_with_length(s, length as usize);
                    loop {
                        let fresh186 = length;
                        length = length.wrapping_add(1);
                        if !(fresh186 < size as usize) {
                            break;
                        }
                        (b.buffer_loads.get_length() < b.buffer_loads.get_size() || !(b.prepare_with_size(1)).is_null()) as i32;
                        let fresh187 = b.buffer_loads.get_length();
                        b.buffer_loads
                            .set_length(((b.buffer_loads.get_length()).wrapping_add(1)) as usize);
                        *(b.buffer_loads.loads_pointer).offset(fresh187 as isize) = 0 as i8;
                    }
                    current_block_33 = 3222590281903869779;
                },
                | 6 => {
                    let mut length: usize = 0;
                    let s_0: *const i8 = lual_checklstring(interpreter, arg, &mut length);
                    (((size >= size_of::<usize>() as i32 || length < (1 as usize) << size * 8) as i32 != 0) as i64 != 0
                        || lual_argerror(interpreter, arg, c"string length does not fit in given size".as_ptr()) != 0)
                        as i32;
                    packint(&mut b, length as usize, h.is_little_endian(), size, 0);
                    b.add_string_with_length(s_0, length as usize);
                    totalsize = (totalsize as usize).wrapping_add(length) as usize;
                    current_block_33 = 3222590281903869779;
                },
                | 7 => {
                    let mut length: usize = 0;
                    let s_1: *const i8 = lual_checklstring(interpreter, arg, &mut length);
                    (((libc::strlen(s_1) as usize == length) as i32 != 0) as i64 != 0
                        || lual_argerror(interpreter, arg, c"string contains zeros".as_ptr()) != 0) as i32;
                    b.add_string_with_length(s_1, length as usize);
                    (b.buffer_loads.get_length() < b.buffer_loads.get_size() || !(b.prepare_with_size(1)).is_null()) as i32;
                    let fresh188 = b.buffer_loads.get_length();
                    b.buffer_loads
                        .set_length(((b.buffer_loads.get_length()).wrapping_add(1)) as usize);
                    *(b.buffer_loads.loads_pointer).offset(fresh188 as isize) = Character::Null as i8;
                    totalsize = (totalsize as usize).wrapping_add(length.wrapping_add(1 as usize)) as usize;
                    current_block_33 = 3222590281903869779;
                },
                | 8 => {
                    (b.buffer_loads.get_length() < b.buffer_loads.get_size() || !(b.prepare_with_size(1)).is_null()) as i32;
                    let fresh189 = b.buffer_loads.get_length();
                    b.buffer_loads
                        .set_length(((b.buffer_loads.get_length()).wrapping_add(1)) as usize);
                    *(b.buffer_loads.loads_pointer).offset(fresh189 as isize) = 0 as i8;
                    current_block_33 = 7383952003695197780;
                },
                | 9 | 10 => {
                    current_block_33 = 7383952003695197780;
                },
                | _ => {
                    current_block_33 = 3222590281903869779;
                },
            }
            match current_block_33 {
                | 7383952003695197780 => {
                    arg -= 1;
                },
                | _ => {},
            }
        }
        b.push_result();
        return 1;
    }
}
pub unsafe fn str_packsize(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let mut h: Header = Header::new(interpreter);
        let mut fmt: *const i8 = lual_checklstring(interpreter, 1, null_mut());
        let mut totalsize: usize = 0;
        while *fmt as i32 != Character::Null as i32 {
            let mut size: i32 = 0;
            let mut ntoalign: i32 = 0;
            let opt: K = h.getdetails(totalsize, &mut fmt, &mut size, &mut ntoalign);
            (((opt as u32 != K::String as u32 && opt as u32 != K::ZString as u32) as i32 != 0) as i64 != 0
                || lual_argerror(interpreter, 1, c"variable-length format".as_ptr()) != 0) as i32;
            size += ntoalign;
            (((totalsize
                <= (if (size_of::<usize>() as usize) < size_of::<i32>() as usize {
                    !(0usize)
                } else {
                    0x7FFFFFFF as usize
                })
                .wrapping_sub(size as usize)) as i32
                != 0) as i64
                != 0
                || lual_argerror(interpreter, 1, c"format result too large".as_ptr()) != 0) as i32;
            totalsize = (totalsize as usize).wrapping_add(size as usize) as usize;
        }
        (*interpreter).push_integer(totalsize as i64);
        return 1;
    }
}
pub unsafe fn unpackint(interpreter: *mut Interpreter, str: *const i8, islittle: bool, size: i32, issigned: i32) -> i64 {
    unsafe {
        let mut res: u64 = 0;
        let mut i: i32;
        let limit: i32 = if size <= size_of::<i64>() as i32 { size } else { size_of::<i64>() as i32 };
        i = limit - 1;
        while i >= 0 {
            res <<= 8;
            res |= *str.offset((if islittle { i } else { size - 1 - i }) as isize) as u8 as u64;
            i -= 1;
        }
        if size < size_of::<i64>() as i32 {
            if issigned != 0 {
                let mask: u64 = 1u64 << size * 8 - 1;
                res = (res ^ mask).wrapping_sub(mask);
            }
        } else if size > size_of::<i64>() as i32 {
            let mask_0: i32 = if issigned == 0 || res as i64 >= 0 { 0 } else { (1 << 8) - 1 };
            for i in limit..size {
                if ((*str.offset((if islittle { i } else { size - 1 - i }) as isize) as u8 as i32 != mask_0) as i32 != 0)
                    as i64
                    != 0
                {
                    lual_error(interpreter, c"%d-byte integer does not fit into Lua Integer".as_ptr(), size);
                }
            }
        }
        return res as i64;
    }
}
pub unsafe fn str_unpack(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let mut h: Header = Header::new(interpreter);
        let mut fmt: *const i8 = lual_checklstring(interpreter, 1, null_mut());
        let mut ld: usize = 0;
        let data: *const i8 = lual_checklstring(interpreter, 2, &mut ld);
        let mut position: usize = (get_position_relative(lual_optinteger(interpreter, 3, 1 as i64), ld)).wrapping_sub(1 as usize);
        let mut n: i32 = 0;
        (((position <= ld) as i32 != 0) as i64 != 0
            || lual_argerror(interpreter, 3, c"initial position out of string".as_ptr()) != 0) as i32;
        while *fmt as i32 != Character::Null as i32 {
            let mut size: i32 = 0;
            let mut ntoalign: i32 = 0;
            let opt: K = h.getdetails(position, &mut fmt, &mut size, &mut ntoalign);
            ((((ntoalign as usize).wrapping_add(size as usize) <= ld.wrapping_sub(position)) as i32 != 0) as i64 != 0
                || lual_argerror(interpreter, 2, c"data string too short".as_ptr()) != 0) as i32;
            position = (position as usize).wrapping_add(ntoalign as usize) as usize;
            lual_checkstack(interpreter, 2, c"too many results".as_ptr());
            n += 1;
            match opt as u32 {
                | 0 | 1 => {
                    let res: i64 = unpackint(
                        interpreter,
                        data.offset(position as isize),
                        h.is_little_endian(),
                        size,
                        (opt as u32 == K::Integer as u32) as i32,
                    );
                    (*interpreter).push_integer(res);
                },
                | 2 => {
                    let mut f: f32 = 0.0;
                    copywithendian(
                        &mut f as *mut f32 as *mut i8,
                        data.offset(position as isize),
                        size_of::<f32>() as i32,
                        h.is_little_endian(),
                    );
                    (*interpreter).push_number(f as f64);
                },
                | 3 => {
                    let mut f_0: f64 = 0.0;
                    copywithendian(
                        &mut f_0 as *mut f64 as *mut i8,
                        data.offset(position as isize),
                        size_of::<f64>() as i32,
                        h.is_little_endian(),
                    );
                    (*interpreter).push_number(f_0);
                },
                | 4 => {
                    let mut f_1: f64 = 0.0;
                    copywithendian(
                        &mut f_1 as *mut f64 as *mut i8,
                        data.offset(position as isize),
                        size_of::<f64>() as i32,
                        h.is_little_endian(),
                    );
                    (*interpreter).push_number(f_1);
                },
                | 5 => {
                    lua_pushlstring(interpreter, data.offset(position as isize), size as usize);
                },
                | 6 => {
                    let length: usize =
                        unpackint(interpreter, data.offset(position as isize), h.is_little_endian(), size, 0) as usize;
                    (((length <= ld.wrapping_sub(position).wrapping_sub(size as usize)) as i32 != 0) as i32 as i64 != 0
                        || lual_argerror(interpreter, 2, c"data string too short".as_ptr()) != 0) as i32;
                    lua_pushlstring(interpreter, data.offset(position as isize).offset(size as isize), length);
                    position = (position as usize).wrapping_add(length) as usize;
                },
                | 7 => {
                    let length_0: usize = libc::strlen(data.offset(position as isize)) as usize;
                    (((position.wrapping_add(length_0) < ld) as i32 != 0) as i64 != 0
                        || lual_argerror(interpreter, 2, c"unfinished string for format 'zio'".as_ptr()) != 0)
                        as i32;
                    lua_pushlstring(interpreter, data.offset(position as isize), length_0);
                    position = (position as usize).wrapping_add(length_0.wrapping_add(1 as usize)) as usize;
                },
                | 9 | 8 | 10 => {
                    n -= 1;
                },
                | _ => {},
            }
            position = (position as usize).wrapping_add(size as usize) as usize;
        }
        (*interpreter).push_integer(position.wrapping_add(1 as usize) as i64);
        return n + 1;
    }
}
pub const STRING_FUNCTIONS: [RegisteredFunction; 17] = {
    [
        {
            RegisteredFunction {
                registeredfunction_name: c"byte".as_ptr(),
                registeredfunction_function: Some(str_byte as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"char".as_ptr(),
                registeredfunction_function: Some(str_char as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"dump".as_ptr(),
                registeredfunction_function: Some(StreamWriter::str_dump as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"find".as_ptr(),
                registeredfunction_function: Some(str_find as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"format".as_ptr(),
                registeredfunction_function: Some(str_format as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"gmatch".as_ptr(),
                registeredfunction_function: Some(GMatchState::gmatch as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"gsub".as_ptr(),
                registeredfunction_function: Some(str_gsub as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"len".as_ptr(),
                registeredfunction_function: Some(str_len as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"lower".as_ptr(),
                registeredfunction_function: Some(str_lower as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"match".as_ptr(),
                registeredfunction_function: Some(str_match as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"rep".as_ptr(),
                registeredfunction_function: Some(str_rep as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"reverse".as_ptr(),
                registeredfunction_function: Some(str_reverse as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"sub".as_ptr(),
                registeredfunction_function: Some(str_sub as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"upper".as_ptr(),
                registeredfunction_function: Some(str_upper as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"pack".as_ptr(),
                registeredfunction_function: Some(str_pack as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"packsize".as_ptr(),
                registeredfunction_function: Some(str_packsize as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"unpack".as_ptr(),
                registeredfunction_function: Some(str_unpack as unsafe fn(*mut Interpreter) -> i32),
            }
        },
    ]
};
pub unsafe fn createmetatable(interpreter: *mut Interpreter) {
    unsafe {
        (*interpreter).lua_createtable();
        lual_setfuncs(interpreter, STRING_METAMETHODS.as_ptr(), STRING_METAMETHODS.len(), 0);
        lua_pushstring(interpreter, c"".as_ptr());
        lua_pushvalue(interpreter, -2);
        lua_setmetatable(interpreter, -2);
        lua_settop(interpreter, -2);
        lua_pushvalue(interpreter, -2);
        lua_setfield(interpreter, -2, c"__index".as_ptr());
        lua_settop(interpreter, -2);
    }
}
pub unsafe fn luaopen_string(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        (*interpreter).lua_createtable();
        lual_setfuncs(interpreter, STRING_FUNCTIONS.as_ptr(), STRING_FUNCTIONS.len(), 0);
        createmetatable(interpreter);
        return 1;
    }
}
