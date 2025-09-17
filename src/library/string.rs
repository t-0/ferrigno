use crate::buffer::*;
use crate::character::*;
use crate::gmatchstate::*;
use crate::header::*;
use crate::interpreter::*;
use crate::k::*;
use crate::matchstate::*;
use crate::nativeendian::*;
use crate::new::*;
use crate::registeredfunction::*;
use crate::streamwriter::*;
use crate::tag::*;
use crate::tstring::*;
use crate::utility::c::*;
use crate::utility::*;
use libc::{memcpy, tolower, toupper};
use rlua::*;
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
            lua_pushstring(interpreter, make_cstring!(""));
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
        let sep: *const i8 = lual_optlstring(interpreter, 3, make_cstring!(""), &mut lsep);
        if n <= 0 {
            lua_pushstring(interpreter, make_cstring!(""));
        } else if ((l.wrapping_add(lsep) < l
            || l.wrapping_add(lsep) as usize
                > ((if (size_of::<usize>() as usize) < size_of::<i32>() as usize {
                    !(0usize)
                } else {
                    0x7FFFFFFF as usize
                }) as usize)
                    .wrapping_div(n as usize)) as i32
            != 0) as i64
            != 0
        {
            return lual_error(interpreter, make_cstring!("resulting string too large"));
        } else {
            let totallen: usize = (n as usize).wrapping_mul(l).wrapping_add(((n - 1) as usize).wrapping_mul(lsep));
            let mut b = Buffer::new();
            let mut p: *mut i8 = b.initialize_with_size(interpreter, totallen as usize);
            loop {
                let fresh159 = n;
                n = n - 1;
                if !(fresh159 > 1) {
                    break;
                }
                memcpy(p as *mut libc::c_void, s as *const libc::c_void, (l as usize).wrapping_mul(size_of::<i8>()));
                p = p.offset(l as isize);
                if lsep > 0 {
                    memcpy(p as *mut libc::c_void, sep as *const libc::c_void, (lsep as usize).wrapping_mul(size_of::<i8>()));
                    p = p.offset(lsep as isize);
                }
            }
            memcpy(p as *mut libc::c_void, s as *const libc::c_void, (l as usize).wrapping_mul(size_of::<i8>()));
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
            return lual_error(interpreter, make_cstring!("string slice too long"));
        }
        n = pose.wrapping_sub(posi) as i32 + 1;
        lual_checkstack(interpreter, n, make_cstring!("string slice too long"));
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
                || lual_argerror(interpreter, i, make_cstring!("value out of range")) != 0) as i32;
            *p.offset((i - 1) as isize) = c as u8 as i8;
        }
        buffer.push_result_with_size(n as usize);
        return 1;
    }
}
pub unsafe fn writer(interpreter: *mut Interpreter, b: *const libc::c_void, size: usize, arbitrary_data: *mut libc::c_void) -> i32 {
    unsafe {
        let stream_writer: *mut StreamWriter = arbitrary_data as *mut StreamWriter;
        if !(*stream_writer).is_initialized {
            (*stream_writer).is_initialized = true;
            (*stream_writer).buffer.initialize(interpreter);
        }
        (*stream_writer).buffer.add_string_with_length(b as *const i8, size as usize);
        return 0;
    }
}
pub unsafe fn str_dump(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let mut stream_writer: StreamWriter = StreamWriter { is_initialized: false, buffer: Buffer::new() };
        let is_strip = 0 != lua_toboolean(interpreter, 2);
        lual_checktype(interpreter, 1, TagType::Closure);
        lua_settop(interpreter, 1);
        stream_writer.is_initialized = false;
        if ((lua_dump(
            interpreter,
            Some(writer as unsafe fn(*mut Interpreter, *const libc::c_void, usize, *mut libc::c_void) -> i32),
            &mut stream_writer as *mut StreamWriter as *mut libc::c_void,
            is_strip,
        ) != 0) as i32
            != 0) as i64
            != 0
        {
            return lual_error(interpreter, make_cstring!("unable to dump given function"));
        }
        stream_writer.buffer.push_result();
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
                make_cstring!("attempt to %s a '%s' with a '%s'"),
                mtname.offset(2 as isize),
                lua_typename(interpreter, lua_type(interpreter, -2)),
                lua_typename(interpreter, lua_type(interpreter, -1)),
            );
        }
        lua_rotate(interpreter, -3, 1);
        lua_callk(interpreter, 2, 1, 0, None);
    }
}
pub unsafe fn arith(interpreter: *mut Interpreter, op: i32, mtname: *const i8) -> i32 {
    unsafe {
        if tonum(interpreter, 1) != 0 && tonum(interpreter, 2) != 0 {
            lua_arith(interpreter, op);
        } else {
            trymt(interpreter, mtname);
        }
        return 1;
    }
}
pub unsafe fn arith_add(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        return arith(interpreter, 0, make_cstring!("__add"));
    }
}
pub unsafe fn arith_sub(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        return arith(interpreter, 1, make_cstring!("__sub"));
    }
}
pub unsafe fn arith_mul(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        return arith(interpreter, 2, make_cstring!("__mul"));
    }
}
pub unsafe fn arith_mod(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        return arith(interpreter, 3, make_cstring!("__mod"));
    }
}
pub unsafe fn arith_pow(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        return arith(interpreter, 4, make_cstring!("__pow"));
    }
}
pub unsafe fn arith_div(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        return arith(interpreter, 5, make_cstring!("__div"));
    }
}
pub unsafe fn arith_idiv(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        return arith(interpreter, 6, make_cstring!("__idiv"));
    }
}
pub unsafe fn arith_unm(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        return arith(interpreter, 12 as i32, make_cstring!("__unm"));
    }
}
pub const STRING_METAMETHODS: [RegisteredFunction; 8] = {
    [
        { RegisteredFunction { name: make_cstring!("__add"), function: Some(arith_add as unsafe fn(*mut Interpreter) -> i32) } },
        { RegisteredFunction { name: make_cstring!("__sub"), function: Some(arith_sub as unsafe fn(*mut Interpreter) -> i32) } },
        { RegisteredFunction { name: make_cstring!("__mul"), function: Some(arith_mul as unsafe fn(*mut Interpreter) -> i32) } },
        { RegisteredFunction { name: make_cstring!("__mod"), function: Some(arith_mod as unsafe fn(*mut Interpreter) -> i32) } },
        { RegisteredFunction { name: make_cstring!("__pow"), function: Some(arith_pow as unsafe fn(*mut Interpreter) -> i32) } },
        { RegisteredFunction { name: make_cstring!("__div"), function: Some(arith_div as unsafe fn(*mut Interpreter) -> i32) } },
        { RegisteredFunction { name: make_cstring!("__idiv"), function: Some(arith_idiv as unsafe fn(*mut Interpreter) -> i32) } },
        { RegisteredFunction { name: make_cstring!("__unm"), function: Some(arith_unm as unsafe fn(*mut Interpreter) -> i32) } },
    ]
};
pub unsafe fn lmemfind(mut s1: *const i8, mut l1: usize, s2: *const i8, mut l2: usize) -> *const i8 {
    unsafe {
        if l2 == 0 {
            return s1;
        } else if l2 > l1 {
            return null();
        } else {
            let mut init: *const i8 = null();
            l2 = l2.wrapping_sub(1);
            l1 = l1.wrapping_sub(l2);
            while l1 > 0 && {
                init = memchr(s1 as *const libc::c_void, *s2 as i32, l1 as usize) as *const i8;
                !init.is_null()
            } {
                init = init.offset(1);
                if memcmp(init as *const libc::c_void, s2.offset(1 as isize) as *const libc::c_void, l2 as usize) == 0 {
                    return init.offset(-(1 as isize));
                } else {
                    l1 = (l1 as usize).wrapping_sub(init.offset_from(s1) as usize) as usize;
                    s1 = init;
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
            if !(strpbrk(p.offset(upto as isize), make_cstring!("^$*+?.([%-"))).is_null() {
                return 0;
            }
            upto = upto.wrapping_add((strlen(p.offset(upto as isize)) as usize).wrapping_add(1));
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
        let init: usize =
            (get_position_relative(lual_optinteger(interpreter, 3, 1 as i64), lexical_state)).wrapping_sub(1 as usize);
        if init > lexical_state {
            (*interpreter).push_nil();
            return 1;
        }
        if find != 0 && (lua_toboolean(interpreter, 4) != 0 || nospecials(p, lp) != 0) {
            let s2: *const i8 = lmemfind(s.offset(init as isize), lexical_state.wrapping_sub(init), p, lp);
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
                capture: [MatchStateCapture { init: null(), length: 0 }; 32],
            };
            let mut s1: *const i8 = s.offset(init as isize);
            let anchor: i32 = (*p as i32 == CHARACTER_CARET as i32) as i32;
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
        let anchor: i32 = (*p as i32 == CHARACTER_CARET as i32) as i32;
        let mut n: i64 = 0;
        let mut changed: i32 = 0;
        let mut match_state: MatchState = MatchState {
            src_init: null(),
            src_end: null(),
            p_end: null(),
            matchstate_interpreter: null_mut(),
            matchdepth: 0,
            level: 0,
            capture: [MatchStateCapture { init: null(), length: 0 }; 32],
        };
        let mut b = Buffer::new();
        (((tr == Some(TagType::Numeric)
            || tr == Some(TagType::String)
            || tr == Some(TagType::Closure)
            || tr == Some(TagType::Table)) as i32
            != 0) as i64
            != 0
            || lual_typeerror(interpreter, 3, make_cstring!("string/function/table")) != 0) as i32;
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
                (b.loads.get_length() < b.loads.get_size() || !(b.prepare_with_size(1)).is_null()) as i32;
                let fresh165 = src;
                src = src.offset(1);
                let fresh166 = b.loads.get_length();
                b.loads.set_length(((b.loads.get_length()).wrapping_add(1)) as usize);
                *(b.loads.loads_pointer).offset(fresh166 as isize) = *fresh165;
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
        ((*b).loads.get_length() < (*b).loads.get_size() || !((*b).prepare_with_size(1)).is_null()) as i32;
        let fresh167 = (*b).loads.get_length();
        (*b).loads.set_length((((*b).loads.get_length()).wrapping_add(1)) as usize);
        *((*b).loads.loads_pointer).offset(fresh167 as isize) = '"' as i8;
        loop {
            let fresh168 = length;
            length = length.wrapping_sub(1);
            if !(fresh168 != 0) {
                break;
            }
            if *s as i32 == '"' as i32 || *s as i32 == CHARACTER_BACKSLASH as i32 || *s as i32 == CHARACTER_LF as i32 {
                ((*b).loads.get_length() < (*b).loads.get_size() || !((*b).prepare_with_size(1)).is_null()) as i32;
                let fresh169 = (*b).loads.get_length();
                (*b).loads.set_length((((*b).loads.get_length()).wrapping_add(1)) as usize);
                *((*b).loads.loads_pointer).offset(fresh169 as isize) = CHARACTER_BACKSLASH as i8;
                ((*b).loads.get_length() < (*b).loads.get_size() || !((*b).prepare_with_size(1)).is_null()) as i32;
                let fresh170 = (*b).loads.get_length();
                (*b).loads.set_length((((*b).loads.get_length()).wrapping_add(1)) as usize);
                *((*b).loads.loads_pointer).offset(fresh170 as isize) = *s;
            } else if *(*__ctype_b_loc()).offset(*s as u8 as isize) as i32 & _ISCONTROL as i32 != 0 {
                let mut buffer: [i8; 10] = [0; 10];
                if *(*__ctype_b_loc()).offset(*s.offset(1 as isize) as u8 as isize) as i32 & _ISDIGIT as i32 == 0 {
                    snprintf(buffer.as_mut_ptr(), size_of::<[i8; 10]>(), make_cstring!("\\%d"), *s as u8 as i32);
                } else {
                    snprintf(buffer.as_mut_ptr(), size_of::<[i8; 10]>(), make_cstring!("\\%03d"), *s as u8 as i32);
                }
                (*b).add_string(buffer.as_mut_ptr());
            } else {
                ((*b).loads.get_length() < (*b).loads.get_size() || !((*b).prepare_with_size(1)).is_null()) as i32;
                let fresh171 = (*b).loads.get_length();
                (*b).loads.set_length((((*b).loads.get_length()).wrapping_add(1)) as usize);
                *((*b).loads.loads_pointer).offset(fresh171 as isize) = *s;
            }
            s = s.offset(1);
        }
        ((*b).loads.get_length() < (*b).loads.get_size() || !((*b).prepare_with_size(1)).is_null()) as i32;
        let fresh172 = (*b).loads.get_length();
        (*b).loads.set_length((((*b).loads.get_length()).wrapping_add(1)) as usize);
        *((*b).loads.loads_pointer).offset(fresh172 as isize) = '"' as i8;
    }
}
pub unsafe fn quotefloat(mut _state: *mut Interpreter, buffer: *mut i8, n: f64) -> i32 {
    unsafe {
        let s: *const i8;
        if n == ::core::f64::INFINITY {
            s = make_cstring!("1e9999");
        } else if n == -::core::f64::INFINITY {
            s = make_cstring!("-1e9999");
        } else if n != n {
            s = make_cstring!("(0/0)");
        } else {
            let nb: i32 = snprintf(buffer, 120, make_cstring!("%a"), n);
            if (memchr(buffer as *const libc::c_void, CHARACTER_PERIOD as i32, nb as usize)).is_null() {
                let point: i8 = CHARACTER_PERIOD as i8;
                let ppoint: *mut i8 = memchr(buffer as *const libc::c_void, point as i32, nb as usize) as *mut i8;
                if !ppoint.is_null() {
                    *ppoint = CHARACTER_PERIOD as i8;
                }
            }
            return nb;
        }
        return snprintf(buffer, 120, make_cstring!("%s"), s);
    }
}
pub unsafe fn addliteral(interpreter: *mut Interpreter, b: *mut Buffer, arg: i32) {
    unsafe {
        match lua_type(interpreter, arg) {
            Some(TagType::String) => {
                let mut length: usize = 0;
                let s: *const i8 = lua_tolstring(interpreter, arg, &mut length);
                addquoted(b, s, length);
            },
            Some(TagType::Numeric) => {
                let buffer: *mut i8 = (*b).prepare_with_size(120);
                let nb: i32;
                if lua_isinteger(interpreter, arg) {
                    let n: i64 = lua_tointegerx(interpreter, arg, null_mut());
                    let format: *const i8 =
                        if n == -(MAXIMUM_SIZE as i64) - 1 as i64 { make_cstring!("0x%llx") } else { make_cstring!("%lld") };
                    nb = snprintf(buffer, 120, format, n);
                } else {
                    nb = quotefloat(interpreter, buffer, lua_tonumberx(interpreter, arg, null_mut()));
                }
                (*b).loads
                    .set_length((((*b).loads.get_length() as usize).wrapping_add(nb as usize) as i32) as usize);
            },
            Some(TagType::Nil) | Some(TagType::Boolean) => {
                lual_tolstring(interpreter, arg, null_mut());
                (*b).add_value();
            },
            _ => {
                lual_argerror(interpreter, arg, make_cstring!("value has no literal form"));
            },
        };
    }
}
pub unsafe fn get2digits(mut s: *const i8) -> *const i8 {
    unsafe {
        if *(*__ctype_b_loc()).offset(*s as u8 as isize) as i32 & _ISDIGIT as i32 != 0 {
            s = s.offset(1);
            if *(*__ctype_b_loc()).offset(*s as u8 as isize) as i32 & _ISDIGIT as i32 != 0 {
                s = s.offset(1);
            }
        }
        return s;
    }
}
pub unsafe fn checkformat(interpreter: *mut Interpreter, form: *const i8, flags: *const i8, precision: i32) {
    unsafe {
        let mut spec: *const i8 = form.offset(1 as isize);
        spec = spec.offset(strspn(spec, flags) as isize);
        if *spec as i32 != CHARACTER_0 as i32 {
            spec = get2digits(spec);
            if *spec as i32 == CHARACTER_PERIOD as i32 && precision != 0 {
                spec = spec.offset(1);
                spec = get2digits(spec);
            }
        }
        if *(*__ctype_b_loc()).offset(*spec as u8 as isize) as i32 & _ISALPHA as i32 == 0 {
            lual_error(interpreter, make_cstring!("invalid conversion specification: '%s'"), form);
        }
    }
}
pub unsafe fn getformat(interpreter: *mut Interpreter, strfrmt: *const i8, mut form: *mut i8) -> *const i8 {
    unsafe {
        let mut length = strspn(strfrmt, make_cstring!("-+#0 123456789."));
        length = length.wrapping_add(1);
        if length >= 22 {
            lual_error(interpreter, make_cstring!("invalid format (too long)"));
        }
        let fresh173 = form;
        form = form.offset(1);
        *fresh173 = CHARACTER_PERCENT as i8;
        memcpy(
            form as *mut libc::c_void,
            strfrmt as *const libc::c_void,
            (length as usize).wrapping_mul(size_of::<i8>()),
        );
        *form.offset(length as isize) = Character::Null as i8;
        return strfrmt.offset(length as isize).offset(-(1 as isize));
    }
}
pub unsafe fn addlenmod(form: *mut i8, lenmod: *const i8) {
    unsafe {
        let length: usize = strlen(form) as usize;
        let mode_length: usize = strlen(lenmod) as usize;
        let spec: i8 = *form.offset(length.wrapping_sub(1 as usize) as isize);
        strcpy(form.offset(length as isize).offset(-(1 as isize)), lenmod);
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
            if *strfrmt as i32 != CHARACTER_PERCENT as i32 {
                (b.loads.get_length() < b.loads.get_size() || !(b.prepare_with_size(1)).is_null()) as i32;
                let fresh174 = strfrmt;
                strfrmt = strfrmt.offset(1);
                let fresh175 = b.loads.get_length();
                b.loads.set_length(((b.loads.get_length()).wrapping_add(1)) as usize);
                *(b.loads.loads_pointer).offset(fresh175 as isize) = *fresh174;
            } else {
                strfrmt = strfrmt.offset(1);
                if *strfrmt as i32 == CHARACTER_PERCENT as i32 {
                    (b.loads.get_length() < b.loads.get_size() || !(b.prepare_with_size(1)).is_null()) as i32;
                    let fresh176 = strfrmt;
                    strfrmt = strfrmt.offset(1);
                    let fresh177 = b.loads.get_length();
                    b.loads.set_length(((b.loads.get_length()).wrapping_add(1)) as usize);
                    *(b.loads.loads_pointer).offset(fresh177 as isize) = *fresh176;
                } else {
                    let mut form: [i8; 32] = [0; 32];
                    let mut maxitem: i32 = 120 as i32;
                    let mut buffer: *mut i8 = b.prepare_with_size(maxitem as usize);
                    let mut nb: i32 = 0;
                    arg += 1;
                    if arg > top {
                        return lual_argerror(interpreter, arg, make_cstring!("no value"));
                    }
                    strfrmt = getformat(interpreter, strfrmt, form.as_mut_ptr());
                    let fresh178 = strfrmt;
                    strfrmt = strfrmt.offset(1);
                    match *fresh178 as i32 {
                        CHARACTER_LOWER_C => {
                            checkformat(interpreter, form.as_mut_ptr(), make_cstring!("-"), 0);
                            nb = snprintf(buffer, maxitem as usize, form.as_mut_ptr(), lual_checkinteger(interpreter, arg) as i32);
                            current_block = 11793792312832361944;
                        },
                        CHARACTER_LOWER_D | CHARACTER_LOWER_I => {
                            flags = make_cstring!("-+0 ");
                            current_block = 5689001924483802034;
                        },
                        CHARACTER_LOWER_U => {
                            flags = make_cstring!("-0");
                            current_block = 5689001924483802034;
                        },
                        CHARACTER_LOWER_O | CHARACTER_LOWER_X | CHARACTER_UPPER_X => {
                            flags = make_cstring!("-#0");
                            current_block = 5689001924483802034;
                        },
                        CHARACTER_LOWER_A | CHARACTER_UPPER_A => {
                            checkformat(interpreter, form.as_mut_ptr(), make_cstring!("-+#0 "), 1);
                            addlenmod(form.as_mut_ptr(), make_cstring!(""));
                            nb = snprintf(buffer, maxitem as usize, form.as_mut_ptr(), lual_checknumber(interpreter, arg));
                            current_block = 11793792312832361944;
                        },
                        CHARACTER_LOWER_F => {
                            maxitem = 110 as i32 + 308 as i32;
                            buffer = b.prepare_with_size(maxitem as usize);
                            current_block = 6669252993407410313;
                        },
                        CHARACTER_LOWER_E | CHARACTER_UPPER_E | CHARACTER_LOWER_G | CHARACTER_UPPER_G => {
                            current_block = 6669252993407410313;
                        },
                        CHARACTER_LOWER_P => {
                            let mut p: *const libc::c_void = (*interpreter).to_pointer(arg);
                            checkformat(interpreter, form.as_mut_ptr(), make_cstring!("-"), 0);
                            if p.is_null() {
                                p = make_cstring!("(null)") as *const libc::c_void;
                                form[(strlen(form.as_mut_ptr())).wrapping_sub(1) as usize] = CHARACTER_LOWER_S as i8;
                            }
                            nb = snprintf(buffer, maxitem as usize, form.as_mut_ptr(), p);
                            current_block = 11793792312832361944;
                        },
                        CHARACTER_LOWER_Q => {
                            if form[2 as usize] as i32 != Character::Null as i32 {
                                return lual_error(interpreter, make_cstring!("specifier '%%q' cannot have modifiers"));
                            }
                            addliteral(interpreter, &mut b, arg);
                            current_block = 11793792312832361944;
                        },
                        CHARACTER_LOWER_S => {
                            let mut l: usize = 0;
                            let s: *const i8 = lual_tolstring(interpreter, arg, &mut l);
                            if form[2 as usize] as i32 == Character::Null as i32 {
                                b.add_value();
                            } else {
                                (((l == strlen(s) as usize) as i32 != 0) as i64 != 0
                                    || lual_argerror(interpreter, arg, make_cstring!("string contains zeros")) != 0)
                                    as i32;
                                checkformat(interpreter, form.as_mut_ptr(), make_cstring!("-"), 1);
                                if (strchr(form.as_mut_ptr(), CHARACTER_PERIOD as i32)).is_null() && l >= 100 as usize {
                                    b.add_value();
                                } else {
                                    nb = snprintf(buffer, maxitem as usize, form.as_mut_ptr(), s);
                                    lua_settop(interpreter, -2);
                                }
                            }
                            current_block = 11793792312832361944;
                        },
                        _ => {
                            return lual_error(
                                interpreter,
                                make_cstring!("invalid conversion '%s' to 'format'"),
                                form.as_mut_ptr(),
                            );
                        },
                    }
                    match current_block {
                        5689001924483802034 => {
                            let n: i64 = lual_checkinteger(interpreter, arg);
                            checkformat(interpreter, form.as_mut_ptr(), flags, 1);
                            addlenmod(form.as_mut_ptr(), make_cstring!("ll"));
                            nb = snprintf(buffer, maxitem as usize, form.as_mut_ptr(), n);
                        },
                        6669252993407410313 => {
                            let n_0: f64 = lual_checknumber(interpreter, arg);
                            checkformat(interpreter, form.as_mut_ptr(), make_cstring!("-+#0 "), 1);
                            addlenmod(form.as_mut_ptr(), make_cstring!(""));
                            nb = snprintf(buffer, maxitem as usize, form.as_mut_ptr(), n_0);
                        },
                        _ => {},
                    }
                    b.loads.set_length(((b.loads.get_length() as usize).wrapping_add(nb as usize) as i32) as usize);
                }
            }
        }
        b.push_result();
        return 1;
    }
}
pub const NATIVE_ENDIAN: NativeEndian = NativeEndian { dummy: 1 };
pub unsafe fn getnum(fmt: *mut *const i8, df: i32) -> i32 {
    unsafe {
        if is_digit(**fmt as i32) {
            let mut a: i32 = 0;
            loop {
                let fresh179 = *fmt;
                *fmt = (*fmt).offset(1);
                a = a * 10 as i32 + (*fresh179 as i32 - CHARACTER_0);
                if !(is_digit(**fmt as i32)
                    && a <= ((if (size_of::<usize>() as usize) < size_of::<i32>() as usize {
                        !(0usize)
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
pub unsafe fn getnumlimit(header: *mut Header, fmt: *mut *const i8, df: i32) -> i32 {
    unsafe {
        let size: i32 = getnum(fmt, df);
        if size > 16 as i32 || size <= 0 {
            return lual_error(
                (*header).header_interpreter,
                make_cstring!("integral size (%d) out of limits [1,%d]"),
                size,
                16 as i32,
            );
        }
        return size;
    }
}
pub unsafe fn initheader(interpreter: *mut Interpreter, header: *mut Header) {
    unsafe {
        (*header).header_interpreter = interpreter;
        (*header).is_little_endian = NATIVE_ENDIAN.little as i32;
        (*header).maxmimum_alignment = 1;
    }
}
pub unsafe fn getoption(header: *mut Header, fmt: *mut *const i8, size: *mut i32) -> K {
    unsafe {
        let fresh180 = *fmt;
        *fmt = (*fmt).offset(1);
        let opt: i32 = *fresh180 as i32;
        *size = 0;
        match opt {
            CHARACTER_LOWER_B => {
                *size = size_of::<i8>() as i32;
                return K::Integer;
            },
            CHARACTER_UPPER_B => {
                *size = size_of::<i8>() as i32;
                return K::Unsigned;
            },
            CHARACTER_LOWER_H => {
                *size = size_of::<i16>() as i32;
                return K::Integer;
            },
            CHARACTER_UPPER_H => {
                *size = size_of::<i16>() as i32;
                return K::Unsigned;
            },
            CHARACTER_LOWER_L => {
                *size = size_of::<i64>() as i32;
                return K::Integer;
            },
            CHARACTER_UPPER_L => {
                *size = size_of::<i64>() as i32;
                return K::Unsigned;
            },
            CHARACTER_LOWER_J => {
                *size = size_of::<i64>() as i32;
                return K::Integer;
            },
            CHARACTER_UPPER_J => {
                *size = size_of::<i64>() as i32;
                return K::Unsigned;
            },
            CHARACTER_UPPER_T => {
                *size = size_of::<usize>() as i32;
                return K::Unsigned;
            },
            CHARACTER_LOWER_F => {
                *size = size_of::<libc::c_float>() as i32;
                return K::Float;
            },
            CHARACTER_LOWER_N => {
                *size = size_of::<f64>() as i32;
                return K::Number;
            },
            CHARACTER_LOWER_D => {
                *size = size_of::<f64>() as i32;
                return K::Double;
            },
            CHARACTER_LOWER_I => {
                *size = getnumlimit(header, fmt, size_of::<i32>() as i32);
                return K::Integer;
            },
            CHARACTER_UPPER_I => {
                *size = getnumlimit(header, fmt, size_of::<i32>() as i32);
                return K::Unsigned;
            },
            CHARACTER_LOWER_S => {
                *size = getnumlimit(header, fmt, size_of::<usize>() as i32);
                return K::String;
            },
            CHARACTER_LOWER_C => {
                *size = getnum(fmt, -1);
                if *size == -1 {
                    lual_error((*header).header_interpreter, make_cstring!("missing size for format option CHARACTER_LOWER_C"));
                }
                return K::Character;
            },
            CHARACTER_LOWER_Z => return K::ZString,
            CHARACTER_LOWER_X => {
                *size = 1;
                return K::Padding;
            },
            CHARACTER_UPPER_X => return K::PaddingAlignment,
            CHARACTER_SPACE => {},
            CHARACTER_ANGLE_LEFT => {
                (*header).is_little_endian = 1;
            },
            CHARACTER_ANGLE_RIGHT => {
                (*header).is_little_endian = 0;
            },
            CHARACTER_EQUAL => {
                (*header).is_little_endian = NATIVE_ENDIAN.little as i32;
            },
            CHARACTER_EXCLAMATION => {
                let maxalign: i32 = 8;
                (*header).maxmimum_alignment = getnumlimit(header, fmt, maxalign);
            },
            _ => {
                lual_error((*header).header_interpreter, make_cstring!("invalid format option '%c'"), opt);
            },
        }
        return K::NoOperator;
    }
}
pub unsafe fn getdetails(
    header: *mut Header, totalsize: usize, fmt: *mut *const i8, total_size: *mut i32, ntoalign: *mut i32,
) -> K {
    unsafe {
        let opt: K = getoption(header, fmt, total_size);
        let mut align: i32 = *total_size;
        if opt as u32 == K::PaddingAlignment as u32 {
            if **fmt as i32 == Character::Null as i32
                || getoption(header, fmt, &mut align) as u32 == K::Character as u32
                || align == 0
            {
                lual_argerror((*header).header_interpreter, 1, make_cstring!("invalid next option for option CHARACTER_UPPER_X"));
            }
        }
        if align <= 1 || opt as u32 == K::Character as u32 {
            *ntoalign = 0;
        } else {
            if align > (*header).maxmimum_alignment {
                align = (*header).maxmimum_alignment;
            }
            if align & align - 1 != 0 {
                lual_argerror((*header).header_interpreter, 1, make_cstring!("format asks for alignment not power of 2"));
            }
            *ntoalign = align - (totalsize & (align - 1) as usize) as i32 & align - 1;
        }
        return opt;
    }
}
pub unsafe fn packint(b: *mut Buffer, mut n: usize, islittle: i32, size: i32, is_negative_: i32) {
    unsafe {
        let buffer: *mut i8 = (*b).prepare_with_size(size as usize);
        *buffer.offset((if islittle != 0 { 0 } else { size - 1 }) as isize) = (n & ((1 << 8) - 1) as usize) as i8;
        for i in 1..size {
            n >>= 8;
            *buffer.offset((if islittle != 0 { i } else { size - 1 - i }) as isize) = (n & ((1 << 8) - 1) as usize) as i8;
        }
        if is_negative_ != 0 && size > size_of::<i64>() as i32 {
            for i in (size_of::<i64>() as i32)..size {
                *buffer.offset((if islittle != 0 { i } else { size - 1 - i }) as isize) = ((1 << 8) - 1) as i8;
            }
        }
        (*b).loads
            .set_length((((*b).loads.get_length() as usize).wrapping_add(size as usize) as i32) as usize);
    }
}
pub unsafe fn copywithendian(mut dest: *mut i8, mut src: *const i8, mut size: i32, islittle: i32) {
    unsafe {
        if islittle == NATIVE_ENDIAN.little as i32 {
            memcpy(dest as *mut libc::c_void, src as *const libc::c_void, size as usize);
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
        let mut h: Header = Header { header_interpreter: null_mut(), is_little_endian: 0, maxmimum_alignment: 0 };
        let mut fmt: *const i8 = lual_checklstring(interpreter, 1, null_mut());
        let mut arg: i32 = 1;
        let mut totalsize: usize = 0;
        initheader(interpreter, &mut h);
        (*interpreter).push_nil();
        b.initialize(interpreter);
        while *fmt as i32 != Character::Null as i32 {
            let mut size: i32 = 0;
            let mut ntoalign: i32 = 0;
            let opt: K = getdetails(&mut h, totalsize, &mut fmt, &mut size, &mut ntoalign);
            totalsize = (totalsize as usize).wrapping_add((ntoalign + size) as usize) as usize;
            loop {
                let fresh184 = ntoalign;
                ntoalign = ntoalign - 1;
                if !(fresh184 > 0) {
                    break;
                }
                (b.loads.get_length() < b.loads.get_size() || !(b.prepare_with_size(1)).is_null()) as i32;
                let fresh185 = b.loads.get_length();
                b.loads.set_length(((b.loads.get_length()).wrapping_add(1)) as usize);
                *(b.loads.loads_pointer).offset(fresh185 as isize) = 0 as i8;
            }
            arg += 1;
            let current_block_33: usize;
            match opt as u32 {
                0 => {
                    let n: i64 = lual_checkinteger(interpreter, arg);
                    if size < size_of::<i64>() as i32 {
                        let lim: i64 = 1 << size * 8 - 1;
                        (((-lim <= n && n < lim) as i32 != 0) as i64 != 0
                            || lual_argerror(interpreter, arg, make_cstring!("integer overflow")) != 0)
                            as i32;
                    }
                    packint(&mut b, n as usize, h.is_little_endian, size, (n < 0) as i32);
                    current_block_33 = 3222590281903869779;
                },
                1 => {
                    let n_0: i64 = lual_checkinteger(interpreter, arg);
                    if size < size_of::<i64>() as i32 {
                        ((((n_0 as usize) < (1 as usize) << size * 8) as i32 != 0) as i64 != 0
                            || lual_argerror(interpreter, arg, make_cstring!("unsigned overflow")) != 0)
                            as i32;
                    }
                    packint(&mut b, n_0 as usize, h.is_little_endian, size, 0);
                    current_block_33 = 3222590281903869779;
                },
                2 => {
                    let mut f: libc::c_float = lual_checknumber(interpreter, arg) as libc::c_float;
                    let buffer: *mut i8 = b.prepare_with_size(size_of::<libc::c_float>());
                    copywithendian(
                        buffer,
                        &mut f as *mut libc::c_float as *mut i8,
                        size_of::<libc::c_float>() as i32,
                        h.is_little_endian,
                    );
                    b.loads.set_length(((b.loads.get_length() as usize).wrapping_add(size as usize) as i32) as usize);
                    current_block_33 = 3222590281903869779;
                },
                3 => {
                    let mut f_0: f64 = lual_checknumber(interpreter, arg);
                    let buff_0: *mut i8 = b.prepare_with_size(size_of::<f64>());
                    copywithendian(buff_0, &mut f_0 as *mut f64 as *mut i8, size_of::<f64>() as i32, h.is_little_endian);
                    b.loads.set_length(((b.loads.get_length() as usize).wrapping_add(size as usize) as i32) as usize);
                    current_block_33 = 3222590281903869779;
                },
                4 => {
                    let mut f_1: f64 = lual_checknumber(interpreter, arg);
                    let buff_1: *mut i8 = b.prepare_with_size(size_of::<f64>());
                    copywithendian(buff_1, &mut f_1 as *mut f64 as *mut i8, size_of::<f64>() as i32, h.is_little_endian);
                    b.loads.set_length(((b.loads.get_length() as usize).wrapping_add(size as usize) as i32) as usize);
                    current_block_33 = 3222590281903869779;
                },
                5 => {
                    let mut length: usize = 0;
                    let s: *const i8 = lual_checklstring(interpreter, arg, &mut length);
                    (((length <= size as usize) as i32 != 0) as i64 != 0
                        || lual_argerror(interpreter, arg, make_cstring!("string longer than given size")) != 0)
                        as i32;
                    b.add_string_with_length(s, length as usize);
                    loop {
                        let fresh186 = length;
                        length = length.wrapping_add(1);
                        if !(fresh186 < size as usize) {
                            break;
                        }
                        (b.loads.get_length() < b.loads.get_size() || !(b.prepare_with_size(1)).is_null()) as i32;
                        let fresh187 = b.loads.get_length();
                        b.loads.set_length(((b.loads.get_length()).wrapping_add(1)) as usize);
                        *(b.loads.loads_pointer).offset(fresh187 as isize) = 0 as i8;
                    }
                    current_block_33 = 3222590281903869779;
                },
                6 => {
                    let mut length: usize = 0;
                    let s_0: *const i8 = lual_checklstring(interpreter, arg, &mut length);
                    (((size >= size_of::<usize>() as i32 || length < (1 as usize) << size * 8) as i32 != 0) as i64 != 0
                        || lual_argerror(interpreter, arg, make_cstring!("string length does not fit in given size")) != 0)
                        as i32;
                    packint(&mut b, length as usize, h.is_little_endian, size, 0);
                    b.add_string_with_length(s_0, length as usize);
                    totalsize = (totalsize as usize).wrapping_add(length) as usize;
                    current_block_33 = 3222590281903869779;
                },
                7 => {
                    let mut length: usize = 0;
                    let s_1: *const i8 = lual_checklstring(interpreter, arg, &mut length);
                    (((strlen(s_1) as usize == length) as i32 != 0) as i64 != 0
                        || lual_argerror(interpreter, arg, make_cstring!("string contains zeros")) != 0) as i32;
                    b.add_string_with_length(s_1, length as usize);
                    (b.loads.get_length() < b.loads.get_size() || !(b.prepare_with_size(1)).is_null()) as i32;
                    let fresh188 = b.loads.get_length();
                    b.loads.set_length(((b.loads.get_length()).wrapping_add(1)) as usize);
                    *(b.loads.loads_pointer).offset(fresh188 as isize) = Character::Null as i8;
                    totalsize = (totalsize as usize).wrapping_add(length.wrapping_add(1 as usize)) as usize;
                    current_block_33 = 3222590281903869779;
                },
                8 => {
                    (b.loads.get_length() < b.loads.get_size() || !(b.prepare_with_size(1)).is_null()) as i32;
                    let fresh189 = b.loads.get_length();
                    b.loads.set_length(((b.loads.get_length()).wrapping_add(1)) as usize);
                    *(b.loads.loads_pointer).offset(fresh189 as isize) = 0 as i8;
                    current_block_33 = 7383952003695197780;
                },
                9 | 10 => {
                    current_block_33 = 7383952003695197780;
                },
                _ => {
                    current_block_33 = 3222590281903869779;
                },
            }
            match current_block_33 {
                7383952003695197780 => {
                    arg -= 1;
                },
                _ => {},
            }
        }
        b.push_result();
        return 1;
    }
}
pub unsafe fn str_packsize(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let mut h: Header = Header { header_interpreter: null_mut(), is_little_endian: 0, maxmimum_alignment: 0 };
        let mut fmt: *const i8 = lual_checklstring(interpreter, 1, null_mut());
        let mut totalsize: usize = 0;
        initheader(interpreter, &mut h);
        while *fmt as i32 != Character::Null as i32 {
            let mut size: i32 = 0;
            let mut ntoalign: i32 = 0;
            let opt: K = getdetails(&mut h, totalsize, &mut fmt, &mut size, &mut ntoalign);
            (((opt as u32 != K::String as u32 && opt as u32 != K::ZString as u32) as i32 != 0) as i64 != 0
                || lual_argerror(interpreter, 1, make_cstring!("variable-length format")) != 0) as i32;
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
                || lual_argerror(interpreter, 1, make_cstring!("format result too large")) != 0) as i32;
            totalsize = (totalsize as usize).wrapping_add(size as usize) as usize;
        }
        (*interpreter).push_integer(totalsize as i64);
        return 1;
    }
}
pub unsafe fn unpackint(interpreter: *mut Interpreter, str: *const i8, islittle: i32, size: i32, issigned: i32) -> i64 {
    unsafe {
        let mut res: u64 = 0;
        let mut i: i32;
        let limit: i32 = if size <= size_of::<i64>() as i32 { size } else { size_of::<i64>() as i32 };
        i = limit - 1;
        while i >= 0 {
            res <<= 8;
            res |= *str.offset((if islittle != 0 { i } else { size - 1 - i }) as isize) as u8 as u64;
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
                if ((*str.offset((if islittle != 0 { i } else { size - 1 - i }) as isize) as u8 as i32 != mask_0) as i32 != 0)
                    as i64
                    != 0
                {
                    lual_error(interpreter, make_cstring!("%d-byte integer does not fit into Lua Integer"), size);
                }
            }
        }
        return res as i64;
    }
}
pub unsafe fn str_unpack(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let mut h: Header = Header { header_interpreter: null_mut(), is_little_endian: 0, maxmimum_alignment: 0 };
        let mut fmt: *const i8 = lual_checklstring(interpreter, 1, null_mut());
        let mut ld: usize = 0;
        let data: *const i8 = lual_checklstring(interpreter, 2, &mut ld);
        let mut pos: usize = (get_position_relative(lual_optinteger(interpreter, 3, 1 as i64), ld)).wrapping_sub(1 as usize);
        let mut n: i32 = 0;
        (((pos <= ld) as i32 != 0) as i64 != 0
            || lual_argerror(interpreter, 3, make_cstring!("initial position out of string")) != 0) as i32;
        initheader(interpreter, &mut h);
        while *fmt as i32 != Character::Null as i32 {
            let mut size: i32 = 0;
            let mut ntoalign: i32 = 0;
            let opt: K = getdetails(&mut h, pos, &mut fmt, &mut size, &mut ntoalign);
            ((((ntoalign as usize).wrapping_add(size as usize) <= ld.wrapping_sub(pos)) as i32 != 0) as i64 != 0
                || lual_argerror(interpreter, 2, make_cstring!("data string too short")) != 0) as i32;
            pos = (pos as usize).wrapping_add(ntoalign as usize) as usize;
            lual_checkstack(interpreter, 2, make_cstring!("too many results"));
            n += 1;
            match opt as u32 {
                0 | 1 => {
                    let res: i64 = unpackint(
                        interpreter,
                        data.offset(pos as isize),
                        h.is_little_endian,
                        size,
                        (opt as u32 == K::Integer as u32) as i32,
                    );
                    (*interpreter).push_integer(res);
                },
                2 => {
                    let mut f: libc::c_float = 0.0;
                    copywithendian(
                        &mut f as *mut libc::c_float as *mut i8,
                        data.offset(pos as isize),
                        size_of::<libc::c_float>() as i32,
                        h.is_little_endian,
                    );
                    (*interpreter).push_number(f as f64);
                },
                3 => {
                    let mut f_0: f64 = 0.0;
                    copywithendian(
                        &mut f_0 as *mut f64 as *mut i8,
                        data.offset(pos as isize),
                        size_of::<f64>() as i32,
                        h.is_little_endian,
                    );
                    (*interpreter).push_number(f_0);
                },
                4 => {
                    let mut f_1: f64 = 0.0;
                    copywithendian(
                        &mut f_1 as *mut f64 as *mut i8,
                        data.offset(pos as isize),
                        size_of::<f64>() as i32,
                        h.is_little_endian,
                    );
                    (*interpreter).push_number(f_1);
                },
                5 => {
                    lua_pushlstring(interpreter, data.offset(pos as isize), size as usize);
                },
                6 => {
                    let length: usize = unpackint(interpreter, data.offset(pos as isize), h.is_little_endian, size, 0) as usize;
                    (((length <= ld.wrapping_sub(pos).wrapping_sub(size as usize)) as i32 != 0) as i32 as i64 != 0
                        || lual_argerror(interpreter, 2, make_cstring!("data string too short")) != 0) as i32;
                    lua_pushlstring(interpreter, data.offset(pos as isize).offset(size as isize), length);
                    pos = (pos as usize).wrapping_add(length) as usize;
                },
                7 => {
                    let length_0: usize = strlen(data.offset(pos as isize)) as usize;
                    (((pos.wrapping_add(length_0) < ld) as i32 != 0) as i64 != 0
                        || lual_argerror(interpreter, 2, make_cstring!("unfinished string for format 'zio'")) != 0)
                        as i32;
                    lua_pushlstring(interpreter, data.offset(pos as isize), length_0);
                    pos = (pos as usize).wrapping_add(length_0.wrapping_add(1 as usize)) as usize;
                },
                9 | 8 | 10 => {
                    n -= 1;
                },
                _ => {},
            }
            pos = (pos as usize).wrapping_add(size as usize) as usize;
        }
        (*interpreter).push_integer(pos.wrapping_add(1 as usize) as i64);
        return n + 1;
    }
}
pub const STRING_FUNCTIONS: [RegisteredFunction; 17] = {
    [
        { RegisteredFunction { name: make_cstring!("byte"), function: Some(str_byte as unsafe fn(*mut Interpreter) -> i32) } },
        { RegisteredFunction { name: make_cstring!("char"), function: Some(str_char as unsafe fn(*mut Interpreter) -> i32) } },
        { RegisteredFunction { name: make_cstring!("dump"), function: Some(str_dump as unsafe fn(*mut Interpreter) -> i32) } },
        { RegisteredFunction { name: make_cstring!("find"), function: Some(str_find as unsafe fn(*mut Interpreter) -> i32) } },
        { RegisteredFunction { name: make_cstring!("format"), function: Some(str_format as unsafe fn(*mut Interpreter) -> i32) } },
        {
            RegisteredFunction {
                name: make_cstring!("gmatch"),
                function: Some(GMatchState::gmatch as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        { RegisteredFunction { name: make_cstring!("gsub"), function: Some(str_gsub as unsafe fn(*mut Interpreter) -> i32) } },
        { RegisteredFunction { name: make_cstring!("len"), function: Some(str_len as unsafe fn(*mut Interpreter) -> i32) } },
        { RegisteredFunction { name: make_cstring!("lower"), function: Some(str_lower as unsafe fn(*mut Interpreter) -> i32) } },
        { RegisteredFunction { name: make_cstring!("match"), function: Some(str_match as unsafe fn(*mut Interpreter) -> i32) } },
        { RegisteredFunction { name: make_cstring!("rep"), function: Some(str_rep as unsafe fn(*mut Interpreter) -> i32) } },
        {
            RegisteredFunction { name: make_cstring!("reverse"), function: Some(str_reverse as unsafe fn(*mut Interpreter) -> i32) }
        },
        { RegisteredFunction { name: make_cstring!("sub"), function: Some(str_sub as unsafe fn(*mut Interpreter) -> i32) } },
        { RegisteredFunction { name: make_cstring!("upper"), function: Some(str_upper as unsafe fn(*mut Interpreter) -> i32) } },
        { RegisteredFunction { name: make_cstring!("pack"), function: Some(str_pack as unsafe fn(*mut Interpreter) -> i32) } },
        {
            RegisteredFunction {
                name: make_cstring!("packsize"),
                function: Some(str_packsize as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        { RegisteredFunction { name: make_cstring!("unpack"), function: Some(str_unpack as unsafe fn(*mut Interpreter) -> i32) } },
    ]
};
pub unsafe fn createmetatable(interpreter: *mut Interpreter) {
    unsafe {
        (*interpreter).lua_createtable();
        lual_setfuncs(interpreter, STRING_METAMETHODS.as_ptr(), STRING_METAMETHODS.len(), 0);
        lua_pushstring(interpreter, make_cstring!(""));
        lua_pushvalue(interpreter, -2);
        lua_setmetatable(interpreter, -2);
        lua_settop(interpreter, -2);
        lua_pushvalue(interpreter, -2);
        lua_setfield(interpreter, -2, make_cstring!("__index"));
        lua_settop(interpreter, -2);
    }
}
pub unsafe fn luaopen_string(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        lual_checkversion_(
            interpreter,
            504.0,
            (size_of::<i64>() as usize).wrapping_mul(16 as usize).wrapping_add(size_of::<f64>() as usize),
        );
        (*interpreter).lua_createtable();
        lual_setfuncs(interpreter, STRING_FUNCTIONS.as_ptr(), STRING_FUNCTIONS.len(), 0);
        createmetatable(interpreter);
        return 1;
    }
}
