use crate::tstring::*;
use crate::buffer::*;
use crate::gmatchstate::*;
use crate::registeredfunction::*;
use crate::new::*;
use crate::utility::c::*;
use crate::matchstate::*;
use crate::k::*;
use crate::header::*;
use crate::nativeendian::*;
use crate::user::*;
use crate::streamwriter::*;
use crate::state::*;
use crate::tag::*;
use libc::{tolower, toupper, memcpy};
pub unsafe extern "C" fn str_len(state: *mut State) -> i32 {
    unsafe {
        let mut l: u64 = 0;
        lual_checklstring(state, 1, &mut l);
        (*state).push_integer(l as i64);
        return 1;
    }
}
pub unsafe extern "C" fn str_sub(state: *mut State) -> i32 {
    unsafe {
        let mut l: u64 = 0;
        let s: *const i8 = lual_checklstring(state, 1, &mut l);
        let start: u64 = get_position_relative(lual_checkinteger(state, 2), l);
        let end: u64 = get_position_end(state, 3, -1 as i64, l);
        if start <= end {
            lua_pushlstring(
                state,
                s.offset(start as isize).offset(-(1 as isize)),
                end.wrapping_sub(start).wrapping_add(1 as u64),
            );
        } else {
            lua_pushstring(state, b"\0" as *const u8 as *const i8);
        }
        return 1;
    }
}
pub unsafe extern "C" fn str_reverse(state: *mut State) -> i32 {
    unsafe {
        let mut l: u64 = 0;
        let mut i: u64;
        let mut b = Buffer::new();
        let s: *const i8 = lual_checklstring(state, 1, &mut l);
        let p: *mut i8 = b.lual_buffinitsize(state, l);
        i = 0;
        while i < l {
            *p.offset(i as isize) = *s.offset(l.wrapping_sub(i).wrapping_sub(1 as u64) as isize);
            i = i.wrapping_add(1);
        }
        b.lual_pushresultsize(l);
        return 1;
    }
}
pub unsafe extern "C" fn str_lower(state: *mut State) -> i32 {
    unsafe {
        let mut l: u64 = 0;
        let mut i: u64;
        let mut b = Buffer::new();
        let s: *const i8 = lual_checklstring(state, 1, &mut l);
        let p: *mut i8 = b.lual_buffinitsize(state, l);
        i = 0;
        while i < l {
            *p.offset(i as isize) = tolower(*s.offset(i as isize) as u8 as i32) as i8;
            i = i.wrapping_add(1);
        }
        b.lual_pushresultsize(l);
        return 1;
    }
}
pub unsafe extern "C" fn str_upper(state: *mut State) -> i32 {
    unsafe {
        let mut l: u64 = 0;
        let mut i: u64;
        let mut b = Buffer::new();
        let s: *const i8 = lual_checklstring(state, 1, &mut l);
        let p: *mut i8 = b.lual_buffinitsize(state, l);
        i = 0;
        while i < l {
            *p.offset(i as isize) = toupper(*s.offset(i as isize) as u8 as i32) as i8;
            i = i.wrapping_add(1);
        }
        b.lual_pushresultsize(l);
        return 1;
    }
}
pub unsafe extern "C" fn str_rep(state: *mut State) -> i32 {
    unsafe {
        let mut l: u64 = 0;
        let mut lsep: u64 = 0;
        let s: *const i8 = lual_checklstring(state, 1, &mut l);
        let mut n: i64 = lual_checkinteger(state, 2);
        let sep: *const i8 = lual_optlstring(state, 3, b"\0" as *const u8 as *const i8, &mut lsep);
        if n <= 0 {
        lua_pushstring(state, b"\0" as *const u8 as *const i8);
    } else if ((l.wrapping_add(lsep) < l
        || l.wrapping_add(lsep) as u64
            > ((if (::core::mem::size_of::<u64>() as u64) < ::core::mem::size_of::<i32>() as u64 {
                !(0u64)
            } else {
                0x7FFFFFFF as u64
            }) as u64)
                .wrapping_div(n as u64)) as i32
        != 0) as i64
        != 0
    {
        return lual_error(
            state,
            b"resulting string too large\0" as *const u8 as *const i8,
        );
    } else {
        let totallen: u64 = (n as u64)
            .wrapping_mul(l)
            .wrapping_add(((n - 1) as u64).wrapping_mul(lsep));
        let mut b = Buffer::new();
        let mut p: *mut i8 = b.lual_buffinitsize(state, totallen);
        loop {
            let fresh159 = n;
            n = n - 1;
            if !(fresh159 > 1) {
                break;
            }
            memcpy(
                p as *mut libc::c_void,
                s as *const libc::c_void,
                (l as usize).wrapping_mul(::core::mem::size_of::<i8>()),
            );
            p = p.offset(l as isize);
            if lsep > 0u64 {
                memcpy(
                    p as *mut libc::c_void,
                    sep as *const libc::c_void,
                    (lsep as usize).wrapping_mul(::core::mem::size_of::<i8>()),
                );
                p = p.offset(lsep as isize);
            }
        }
        memcpy(
            p as *mut libc::c_void,
            s as *const libc::c_void,
            (l as usize).wrapping_mul(::core::mem::size_of::<i8>()),
        );
        b.lual_pushresultsize(totallen);
    }
        return 1;
    }
}
pub unsafe extern "C" fn str_byte(state: *mut State) -> i32 {
    unsafe {
        let mut l: u64 = 0;
        let s: *const i8 = lual_checklstring(state, 1, &mut l);
        let pi: i64 = lual_optinteger(state, 2, 1);
        let posi: u64 = get_position_relative(pi, l);
        let pose: u64 = get_position_end(state, 3, pi, l);
        let n: i32;
        let mut i: i32;
        if posi > pose {
            return 0;
        }
        if ((pose.wrapping_sub(posi) >= 0x7FFFFFFF as u64) as i32 != 0) as i64 != 0 {
            return lual_error(state, b"string slice too long\0" as *const u8 as *const i8);
        }
        n = pose.wrapping_sub(posi) as i32 + 1;
        lual_checkstack(
            state,
            n,
            b"string slice too long\0" as *const u8 as *const i8,
        );
        i = 0;
        while i < n {
            (*state).push_integer(
                *s.offset(posi.wrapping_add(i as u64).wrapping_sub(1 as u64) as isize) as u8 as i64,
            );
            i += 1;
        }
        return n;
    }
}
pub unsafe extern "C" fn str_char(state: *mut State) -> i32 {
    unsafe {
        let n: i32 = (*state).get_top();
        let mut i: i32;
        let mut b = Buffer::new();
        let p: *mut i8 = b.lual_buffinitsize(state, n as u64);
        i = 1;
        while i <= n {
            let c: u64 = lual_checkinteger(state, i) as u64;
            (((c <= (127 as i32 * 2 + 1) as u64) as i32 != 0) as i64 != 0
                || lual_argerror(state, i, b"value out of range\0" as *const u8 as *const i8) != 0)
                as i32;
            *p.offset((i - 1) as isize) = c as u8 as i8;
            i += 1;
        }
        b.lual_pushresultsize(n as u64);
        return 1;
    }
}
pub unsafe extern "C" fn writer(
    state: *mut State,
    b: *const libc::c_void,
    size: u64,
    arbitrary_data: *mut libc::c_void,
) -> i32 {
    unsafe {
        let stream_writer: *mut StreamWriter = arbitrary_data as *mut StreamWriter;
        if (*stream_writer).init == 0 {
            (*stream_writer).init = 1;
            (*stream_writer).buffer.lual_buffinit(state);
        }
        (*stream_writer)
            .buffer
            .lual_addlstring(b as *const i8, size);
        return 0;
    }
}
pub unsafe extern "C" fn str_dump(state: *mut State) -> i32 {
    unsafe {
        let mut stream_writer: StreamWriter = StreamWriter {
            init: 0,
            buffer: Buffer::new(),
        };
        let is_strip = 0 != lua_toboolean(state, 2);
        lual_checktype(state, 1, TAG_TYPE_CLOSURE);
        lua_settop(state, 1);
        stream_writer.init = 0;
        if ((lua_dump(
            state,
            Some(
                writer
                    as unsafe extern "C" fn(
                        *mut State,
                        *const libc::c_void,
                        u64,
                        *mut libc::c_void,
                    ) -> i32,
            ),
            &mut stream_writer as *mut StreamWriter as *mut libc::c_void,
            is_strip,
        ) != 0) as i32
            != 0) as i64
            != 0
        {
            return lual_error(
                state,
                b"unable to dump given function\0" as *const u8 as *const i8,
            );
        }
        stream_writer.buffer.lual_pushresult();
        return 1;
    }
}
pub unsafe extern "C" fn tonum(state: *mut State, arg: i32) -> i32 {
    unsafe {
        if lua_type(state, arg) == Some(TAG_TYPE_NUMERIC) {
            lua_pushvalue(state, arg);
            return 1;
        } else {
            let mut length: u64 = 0;
            let s: *const i8 = lua_tolstring(state, arg, &mut length);
            return (!s.is_null() && lua_stringtonumber(state, s) == length.wrapping_add(1 as u64))
                as i32;
        };
    }
}
pub unsafe extern "C" fn trymt(state: *mut State, mtname: *const i8) {
    unsafe {
        lua_settop(state, 2);
        if ((lua_type(state, 2) == Some(TAG_TYPE_STRING) || lual_getmetafield(state, 2, mtname) == 0) as i32 != 0)
            as i64
            != 0
        {
            lual_error(
                state,
                b"attempt to %s a '%s' with a '%s'\0" as *const u8 as *const i8,
                mtname.offset(2 as isize),
                lua_typename(state, lua_type(state, -2)),
                lua_typename(state, lua_type(state, -1)),
            );
        }
        lua_rotate(state, -3, 1);
        lua_callk(state, 2, 1, 0, None);
    }
}
pub unsafe extern "C" fn arith(state: *mut State, op: i32, mtname: *const i8) -> i32 {
    unsafe {
        if tonum(state, 1) != 0 && tonum(state, 2) != 0 {
            lua_arith(state, op);
        } else {
            trymt(state, mtname);
        }
        return 1;
    }
}
pub unsafe extern "C" fn arith_add(state: *mut State) -> i32 {
    unsafe {
        return arith(state, 0, b"__add\0" as *const u8 as *const i8);
    }
}
pub unsafe extern "C" fn arith_sub(state: *mut State) -> i32 {
    unsafe {
        return arith(state, 1, b"__sub\0" as *const u8 as *const i8);
    }
}
pub unsafe extern "C" fn arith_mul(state: *mut State) -> i32 {
    unsafe {
        return arith(state, 2, b"__mul\0" as *const u8 as *const i8);
    }
}
pub unsafe extern "C" fn arith_mod(state: *mut State) -> i32 {
    unsafe {
        return arith(state, 3, b"__mod\0" as *const u8 as *const i8);
    }
}
pub unsafe extern "C" fn arith_pow(state: *mut State) -> i32 {
    unsafe {
        return arith(state, 4, b"__pow\0" as *const u8 as *const i8);
    }
}
pub unsafe extern "C" fn arith_div(state: *mut State) -> i32 {
    unsafe {
        return arith(state, 5, b"__div\0" as *const u8 as *const i8);
    }
}
pub unsafe extern "C" fn arith_idiv(state: *mut State) -> i32 {
    unsafe {
        return arith(state, 6, b"__idiv\0" as *const u8 as *const i8);
    }
}
pub unsafe extern "C" fn arith_unm(state: *mut State) -> i32 {
    unsafe {
        return arith(state, 12 as i32, b"__unm\0" as *const u8 as *const i8);
    }
}
pub const STRING_METAMETHODS: [RegisteredFunction; 10] = {
    [
        {
            RegisteredFunction {
                name: b"__add\0" as *const u8 as *const i8,
                function: Some(arith_add as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"__sub\0" as *const u8 as *const i8,
                function: Some(arith_sub as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"__mul\0" as *const u8 as *const i8,
                function: Some(arith_mul as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"__mod\0" as *const u8 as *const i8,
                function: Some(arith_mod as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"__pow\0" as *const u8 as *const i8,
                function: Some(arith_pow as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"__div\0" as *const u8 as *const i8,
                function: Some(arith_div as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"__idiv\0" as *const u8 as *const i8,
                function: Some(arith_idiv as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"__unm\0" as *const u8 as *const i8,
                function: Some(arith_unm as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"__index\0" as *const u8 as *const i8,
                function: None,
            }
        },
        {
            RegisteredFunction {
                name: std::ptr::null(),
                function: None,
            }
        },
    ]
};
pub unsafe extern "C" fn lmemfind(
    mut s1: *const i8,
    mut l1: u64,
    s2: *const i8,
    mut l2: u64,
) -> *const i8 {
    unsafe {
        if l2 == 0u64 {
            return s1;
        } else if l2 > l1 {
            return std::ptr::null();
        } else {
            let mut init: *const i8 = std::ptr::null();
            l2 = l2.wrapping_sub(1);
            l1 = l1.wrapping_sub(l2);
            while l1 > 0u64 && {
                init = memchr(s1 as *const libc::c_void, *s2 as i32, l1) as *const i8;
                !init.is_null()
            } {
                init = init.offset(1);
                if memcmp(
                    init as *const libc::c_void,
                    s2.offset(1 as isize) as *const libc::c_void,
                    l2,
                ) == 0
                {
                    return init.offset(-(1 as isize));
                } else {
                    l1 = (l1 as u64).wrapping_sub(init.offset_from(s1) as u64) as u64;
                    s1 = init;
                }
            }
            return std::ptr::null();
        };
    }
}
pub unsafe extern "C" fn nospecials(p: *const i8, l: u64) -> i32 {
    unsafe {
        let mut upto: u64 = 0;
        loop {
            if !(strpbrk(
                p.offset(upto as isize),
                b"^$*+?.([%-\0" as *const u8 as *const i8,
            ))
            .is_null()
            {
                return 0;
            }
            upto = (upto as u64)
                .wrapping_add((strlen(p.offset(upto as isize))).wrapping_add(1 as u64))
                as u64;
            if !(upto <= l) {
                break;
            }
        }
        return 1;
    }
}
pub unsafe extern "C" fn str_find_aux(state: *mut State, find: i32) -> i32 {
    unsafe {
        let mut lexical_state: u64 = 0;
        let mut lp: u64 = 0;
        let s: *const i8 = lual_checklstring(state, 1, &mut lexical_state);
        let mut p: *const i8 = lual_checklstring(state, 2, &mut lp);
        let init: u64 =
            (get_position_relative(lual_optinteger(state, 3, 1 as i64), lexical_state)).wrapping_sub(1 as u64);
        if init > lexical_state {
            (*state).push_nil();
            return 1;
        }
        if find != 0 && (lua_toboolean(state, 4) != 0 || nospecials(p, lp) != 0) {
            let s2: *const i8 = lmemfind(
                s.offset(init as isize),
                lexical_state.wrapping_sub(init),
                p,
                lp,
            );
            if !s2.is_null() {
                (*state).push_integer((s2.offset_from(s) as i64 + 1) as i64);
                (*state).push_integer((s2.offset_from(s) as u64).wrapping_add(lp) as i64);
                return 2;
            }
        } else {
            let mut match_state: MatchState = MatchState {
                src_init: std::ptr::null(),
                src_end: std::ptr::null(),
                p_end: std::ptr::null(),
                state: std::ptr::null_mut(),
                matchdepth: 0,
                level: 0,
                capture: [MatchStateCapture {
                    init: std::ptr::null(),
                    length: 0,
                }; 32],
            };
            let mut s1: *const i8 = s.offset(init as isize);
            let anchor: i32 = (*p as i32 == '^' as i32) as i32;
            if anchor != 0 {
                p = p.offset(1);
                lp = lp.wrapping_sub(1);
            }
            match_state.prepstate(state, s, lexical_state, p, lp);
            loop {
                let res: *const i8;
                match_state.reprepstate();
                res = match_state.match_0(s1, p);
                if !res.is_null() {
                    if find != 0 {
                        (*state).push_integer((s1.offset_from(s) as i64 + 1) as i64);
                        (*state).push_integer(res.offset_from(s) as i64);
                        return match_state.push_captures(std::ptr::null(), std::ptr::null()) + 2;
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
        (*state).push_nil();
        return 1;
    }
}
pub unsafe extern "C" fn str_find(state: *mut State) -> i32 {
    unsafe {
        return str_find_aux(state, 1);
    }
}
pub unsafe extern "C" fn str_match(state: *mut State) -> i32 {
    unsafe {
        return str_find_aux(state, 0);
    }
}
pub unsafe extern "C" fn str_gsub(state: *mut State) -> i32 {
    unsafe {
        let mut srcl: u64 = 0;
        let mut lp: u64 = 0;
        let mut src: *const i8 = lual_checklstring(state, 1, &mut srcl);
        let mut p: *const i8 = lual_checklstring(state, 2, &mut lp);
        let mut lastmatch: *const i8 = std::ptr::null();
        let tr = lua_type(state, 3);
        let max_s: i64 = lual_optinteger(state, 4, srcl.wrapping_add(1 as u64) as i64);
        let anchor: i32 = (*p as i32 == '^' as i32) as i32;
        let mut n: i64 = 0;
        let mut changed: i32 = 0;
        let mut match_state: MatchState = MatchState {
            src_init: std::ptr::null(),
            src_end: std::ptr::null(),
            p_end: std::ptr::null(),
            state: std::ptr::null_mut(),
            matchdepth: 0,
            level: 0,
            capture: [MatchStateCapture {
                init: std::ptr::null(),
                length: 0,
            }; 32],
        };
        let mut b = Buffer::new();
        (((tr == Some(TAG_TYPE_NUMERIC) || tr == Some(TAG_TYPE_STRING) || tr == Some(TAG_TYPE_CLOSURE) || tr == Some(TAG_TYPE_TABLE)) as i32 != 0) as i64 != 0
            || lual_typeerror(
                state,
                3,
                b"string/function/table\0" as *const u8 as *const i8,
            ) != 0) as i32;
        b.lual_buffinit(state);
        if anchor != 0 {
            p = p.offset(1);
            lp = lp.wrapping_sub(1);
        }
        match_state.prepstate(state, src, srcl, p, lp);
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
                (b.length < b.size || !(b.lual_prepbuffsize(1 as u64)).is_null()) as i32;
                let fresh165 = src;
                src = src.offset(1);
                let fresh166 = b.length;
                b.length = (b.length).wrapping_add(1);
                *(b.pointer).offset(fresh166 as isize) = *fresh165;
            }
            if anchor != 0 {
                break;
            }
        }
        if changed == 0 {
            lua_pushvalue(state, 1);
        } else {
            b.lual_addlstring(src, (match_state.src_end).offset_from(src) as u64);
            b.lual_pushresult();
        }
        (*state).push_integer(n);
        return 2;
    }
}
pub unsafe extern "C" fn addquoted(b: *mut Buffer, mut s: *const i8, mut length: u64) {
    unsafe {
        ((*b).length < (*b).size || !((*b).lual_prepbuffsize(1 as u64)).is_null()) as i32;
        let fresh167 = (*b).length;
        (*b).length = ((*b).length).wrapping_add(1);
        *((*b).pointer).offset(fresh167 as isize) = '"' as i8;
        loop {
            let fresh168 = length;
            length = length.wrapping_sub(1);
            if !(fresh168 != 0) {
                break;
            }
            if *s as i32 == '"' as i32 || *s as i32 == '\\' as i32 || *s as i32 == '\n' as i32 {
                ((*b).length < (*b).size || !((*b).lual_prepbuffsize(1 as u64)).is_null()) as i32;
                let fresh169 = (*b).length;
                (*b).length = ((*b).length).wrapping_add(1);
                *((*b).pointer).offset(fresh169 as isize) = '\\' as i8;
                ((*b).length < (*b).size || !((*b).lual_prepbuffsize(1 as u64)).is_null()) as i32;
                let fresh170 = (*b).length;
                (*b).length = ((*b).length).wrapping_add(1);
                *((*b).pointer).offset(fresh170 as isize) = *s;
            } else if *(*__ctype_b_loc()).offset(*s as u8 as isize) as i32
                & _ISCONTROL as i32
                != 0
            {
                let mut buffer: [i8; 10] = [0; 10];
                if *(*__ctype_b_loc()).offset(*s.offset(1 as isize) as u8 as isize) as i32
                    & _ISDIGIT as i32
                    == 0
                {
                    snprintf(
                        buffer.as_mut_ptr(),
                        ::core::mem::size_of::<[i8; 10]>() as u64,
                        b"\\%d\0" as *const u8 as *const i8,
                        *s as u8 as i32,
                    );
                } else {
                    snprintf(
                        buffer.as_mut_ptr(),
                        ::core::mem::size_of::<[i8; 10]>() as u64,
                        b"\\%03d\0" as *const u8 as *const i8,
                        *s as u8 as i32,
                    );
                }
                (*b).lual_addstring(buffer.as_mut_ptr());
            } else {
                ((*b).length < (*b).size || !((*b).lual_prepbuffsize(1 as u64)).is_null()) as i32;
                let fresh171 = (*b).length;
                (*b).length = ((*b).length).wrapping_add(1);
                *((*b).pointer).offset(fresh171 as isize) = *s;
            }
            s = s.offset(1);
        }
        ((*b).length < (*b).size || !((*b).lual_prepbuffsize(1 as u64)).is_null()) as i32;
        let fresh172 = (*b).length;
        (*b).length = ((*b).length).wrapping_add(1);
        *((*b).pointer).offset(fresh172 as isize) = '"' as i8;
    }
}
pub unsafe extern "C" fn quotefloat(mut _state: *mut State, buffer: *mut i8, n: f64) -> i32 {
    unsafe {
        let s: *const i8;
        if n == ::core::f64::INFINITY {
            s = b"1e9999\0" as *const u8 as *const i8;
        } else if n == -::core::f64::INFINITY {
            s = b"-1e9999\0" as *const u8 as *const i8;
        } else if n != n {
            s = b"(0/0)\0" as *const u8 as *const i8;
        } else {
            let nb: i32 = snprintf(
                buffer,
                120 as u64,
                b"%a\0" as *const u8 as *const i8,
                n,
            );
            if (memchr(buffer as *const libc::c_void, '.' as i32, nb as u64)).is_null() {
                let point: i8 = '.' as i8;
                let ppoint: *mut i8 =
                    memchr(buffer as *const libc::c_void, point as i32, nb as u64) as *mut i8;
                if !ppoint.is_null() {
                    *ppoint = '.' as i8;
                }
            }
            return nb;
        }
        return snprintf(
            buffer,
            120 as u64,
            b"%s\0" as *const u8 as *const i8,
            s,
        );
    }
}
pub unsafe extern "C" fn addliteral(state: *mut State, b: *mut Buffer, arg: i32) {
    unsafe {
        match lua_type(state, arg) {
            Some(TAG_TYPE_STRING) => {
                let mut length: u64 = 0;
                let s: *const i8 = lua_tolstring(state, arg, &mut length);
                addquoted(b, s, length);
            },
            Some(TAG_TYPE_NUMERIC) => {
                let buffer: *mut i8 = (*b).lual_prepbuffsize(120 as u64);
                let nb: i32;
                if lua_isinteger(state, arg) {
                    let n: i64 = lua_tointegerx(state, arg, std::ptr::null_mut());
                    let format: *const i8 = if n == -(0x7FFFFFFFFFFFFFFF as i64) - 1 as i64 {
                        b"0x%llx\0" as *const u8 as *const i8
                    } else {
                        b"%lld\0" as *const u8 as *const i8
                    };
                    nb = snprintf(buffer, 120 as u64, format, n);
                } else {
                    nb = quotefloat(
                        state,
                        buffer,
                        lua_tonumberx(state, arg, std::ptr::null_mut()),
                    );
                }
                (*b).length = ((*b).length as u64).wrapping_add(nb as u64) as u64;
            },
            Some(TAG_TYPE_NIL) | Some(TAG_TYPE_BOOLEAN) => {
                lual_tolstring(state, arg, std::ptr::null_mut());
                (*b).lual_addvalue();
            },
            _ => {
                lual_argerror(
                    state,
                    arg,
                    b"value has no literal form\0" as *const u8 as *const i8,
                );
            }
        };
    }
}
pub unsafe extern "C" fn get2digits(mut s: *const i8) -> *const i8 {
    unsafe {
        if *(*__ctype_b_loc()).offset(*s as u8 as isize) as i32
            & _ISDIGIT as i32
            != 0
        {
            s = s.offset(1);
            if *(*__ctype_b_loc()).offset(*s as u8 as isize) as i32
                & _ISDIGIT as i32
                != 0
            {
                s = s.offset(1);
            }
        }
        return s;
    }
}
pub unsafe extern "C" fn checkformat(
    state: *mut State,
    form: *const i8,
    flags: *const i8,
    precision: i32,
) {
    unsafe {
        let mut spec: *const i8 = form.offset(1 as isize);
        spec = spec.offset(strspn(spec, flags) as isize);
        if *spec as i32 != '0' as i32 {
            spec = get2digits(spec);
            if *spec as i32 == '.' as i32 && precision != 0 {
                spec = spec.offset(1);
                spec = get2digits(spec);
            }
        }
        if *(*__ctype_b_loc()).offset(*spec as u8 as isize) as i32
            & _ISALPHA as i32
            == 0
        {
            lual_error(
                state,
                b"invalid conversion specification: '%s'\0" as *const u8 as *const i8,
                form,
            );
        }
    }
}
pub unsafe extern "C" fn getformat(
    state: *mut State,
    strfrmt: *const i8,
    mut form: *mut i8,
) -> *const i8 {
    unsafe {
        let mut length: u64 = strspn(strfrmt, b"-+#0 123456789.\0" as *const u8 as *const i8);
        length = length.wrapping_add(1);
        if length >= (32 as i32 - 10 as i32) as u64 {
            lual_error(
                state,
                b"invalid format (too long)\0" as *const u8 as *const i8,
            );
        }
        let fresh173 = form;
        form = form.offset(1);
        *fresh173 = '%' as i8;
        memcpy(
            form as *mut libc::c_void,
            strfrmt as *const libc::c_void,
            (length as usize).wrapping_mul(::core::mem::size_of::<i8>()),
        );
        *form.offset(length as isize) = '\0' as i8;
        return strfrmt.offset(length as isize).offset(-(1 as isize));
    }
}
pub unsafe extern "C" fn addlenmod(form: *mut i8, lenmod: *const i8) {
    unsafe {
        let l: u64 = strlen(form);
        let lm: u64 = strlen(lenmod);
        let spec: i8 = *form.offset(l.wrapping_sub(1 as u64) as isize);
        strcpy(form.offset(l as isize).offset(-(1 as isize)), lenmod);
        *form.offset(l.wrapping_add(lm).wrapping_sub(1 as u64) as isize) = spec;
        *form.offset(l.wrapping_add(lm) as isize) = '\0' as i8;
    }
}
pub unsafe extern "C" fn str_format(state: *mut State) -> i32 {
    unsafe {
        let mut current_block: u64;
        let top: i32 = (*state).get_top();
        let mut arg: i32 = 1;
        let mut sfl: u64 = 0;
        let mut strfrmt: *const i8 = lual_checklstring(state, arg, &mut sfl);
        let strfrmt_end: *const i8 = strfrmt.offset(sfl as isize);
        let mut flags: *const i8 = std::ptr::null();
        let mut b = Buffer::new();
        b.lual_buffinit(state);
        while strfrmt < strfrmt_end {
            if *strfrmt as i32 != '%' as i32 {
                (b.length < b.size || !(b.lual_prepbuffsize(1 as u64)).is_null()) as i32;
                let fresh174 = strfrmt;
                strfrmt = strfrmt.offset(1);
                let fresh175 = b.length;
                b.length = (b.length).wrapping_add(1);
                *(b.pointer).offset(fresh175 as isize) = *fresh174;
            } else {
                strfrmt = strfrmt.offset(1);
                if *strfrmt as i32 == '%' as i32 {
                    (b.length < b.size || !(b.lual_prepbuffsize(1 as u64)).is_null()) as i32;
                    let fresh176 = strfrmt;
                    strfrmt = strfrmt.offset(1);
                    let fresh177 = b.length;
                    b.length = (b.length).wrapping_add(1);
                    *(b.pointer).offset(fresh177 as isize) = *fresh176;
                } else {
                    let mut form: [i8; 32] = [0; 32];
                    let mut maxitem: i32 = 120 as i32;
                    let mut buffer: *mut i8 = b.lual_prepbuffsize(maxitem as u64);
                    let mut nb: i32 = 0;
                    arg += 1;
                    if arg > top {
                        return lual_argerror(state, arg, b"no value\0" as *const u8 as *const i8);
                    }
                    strfrmt = getformat(state, strfrmt, form.as_mut_ptr());
                    let fresh178 = strfrmt;
                    strfrmt = strfrmt.offset(1);
                    match *fresh178 as i32 {
                        99 => {
                            checkformat(
                                state,
                                form.as_mut_ptr(),
                                b"-\0" as *const u8 as *const i8,
                                0,
                            );
                            nb = snprintf(
                                buffer,
                                maxitem as u64,
                                form.as_mut_ptr(),
                                lual_checkinteger(state, arg) as i32,
                            );
                            current_block = 11793792312832361944;
                        }
                        100 | 105 => {
                            flags = b"-+0 \0" as *const u8 as *const i8;
                            current_block = 5689001924483802034;
                        }
                        117 => {
                            flags = b"-0\0" as *const u8 as *const i8;
                            current_block = 5689001924483802034;
                        }
                        111 | 120 | 88 => {
                            flags = b"-#0\0" as *const u8 as *const i8;
                            current_block = 5689001924483802034;
                        }
                        97 | 65 => {
                            checkformat(
                                state,
                                form.as_mut_ptr(),
                                b"-+#0 \0" as *const u8 as *const i8,
                                1,
                            );
                            addlenmod(form.as_mut_ptr(), b"\0" as *const u8 as *const i8);
                            nb = snprintf(
                                buffer,
                                maxitem as u64,
                                form.as_mut_ptr(),
                                lual_checknumber(state, arg),
                            );
                            current_block = 11793792312832361944;
                        }
                        102 => {
                            maxitem = 110 as i32 + 308 as i32;
                            buffer = b.lual_prepbuffsize(maxitem as u64);
                            current_block = 6669252993407410313;
                        }
                        101 | 69 | 103 | 71 => {
                            current_block = 6669252993407410313;
                        }
                        112 => {
                            let mut p: *const libc::c_void = User::lua_topointer(state, arg);
                            checkformat(
                                state,
                                form.as_mut_ptr(),
                                b"-\0" as *const u8 as *const i8,
                                0,
                            );
                            if p.is_null() {
                                p = b"(null)\0" as *const u8 as *const i8 as *const libc::c_void;
                                form[(strlen(form.as_mut_ptr())).wrapping_sub(1 as u64) as usize] =
                                    's' as i8;
                            }
                            nb = snprintf(buffer, maxitem as u64, form.as_mut_ptr(), p);
                            current_block = 11793792312832361944;
                        }
                        113 => {
                            if form[2 as usize] as i32 != '\0' as i32 {
                                return lual_error(
                                    state,
                                    b"specifier '%%q' cannot have modifiers\0" as *const u8
                                        as *const i8,
                                );
                            }
                            addliteral(state, &mut b, arg);
                            current_block = 11793792312832361944;
                        }
                        115 => {
                            let mut l: u64 = 0;
                            let s: *const i8 = lual_tolstring(state, arg, &mut l);
                            if form[2 as usize] as i32 == '\0' as i32 {
                                b.lual_addvalue();
                            } else {
                                (((l == strlen(s)) as i32 != 0) as i64 != 0
                                    || lual_argerror(
                                        state,
                                        arg,
                                        b"string contains zeros\0" as *const u8 as *const i8,
                                    ) != 0) as i32;
                                checkformat(
                                    state,
                                    form.as_mut_ptr(),
                                    b"-\0" as *const u8 as *const i8,
                                    1,
                                );
                                if (strchr(form.as_mut_ptr(), '.' as i32)).is_null()
                                    && l >= 100 as u64
                                {
                                    b.lual_addvalue();
                                } else {
                                    nb = snprintf(buffer, maxitem as u64, form.as_mut_ptr(), s);
                                    lua_settop(state, -2);
                                }
                            }
                            current_block = 11793792312832361944;
                        }
                        _ => {
                            return lual_error(
                                state,
                                b"invalid conversion '%s' to 'format'\0" as *const u8 as *const i8,
                                form.as_mut_ptr(),
                            );
                        }
                    }
                    match current_block {
                        5689001924483802034 => {
                            let n: i64 = lual_checkinteger(state, arg);
                            checkformat(state, form.as_mut_ptr(), flags, 1);
                            addlenmod(form.as_mut_ptr(), b"ll\0" as *const u8 as *const i8);
                            nb = snprintf(buffer, maxitem as u64, form.as_mut_ptr(), n);
                        }
                        6669252993407410313 => {
                            let n_0: f64 = lual_checknumber(state, arg);
                            checkformat(
                                state,
                                form.as_mut_ptr(),
                                b"-+#0 \0" as *const u8 as *const i8,
                                1,
                            );
                            addlenmod(form.as_mut_ptr(), b"\0" as *const u8 as *const i8);
                            nb = snprintf(buffer, maxitem as u64, form.as_mut_ptr(), n_0);
                        }
                        _ => {}
                    }
                    b.length = (b.length as u64).wrapping_add(nb as u64) as u64;
                }
            }
        }
        b.lual_pushresult();
        return 1;
    }
}
pub const NATIVE_ENDIAN: NativeEndian = NativeEndian { dummy: 1 };
pub unsafe extern "C" fn digit(c: i32) -> i32 {
    return ('0' as i32 <= c && c <= '9' as i32) as i32;
}
pub unsafe extern "C" fn getnum(fmt: *mut *const i8, df: i32) -> i32 {
    unsafe {
        if digit(**fmt as i32) == 0 {
            return df;
        } else {
            let mut a: i32 = 0;
            loop {
                let fresh179 = *fmt;
                *fmt = (*fmt).offset(1);
                a = a * 10 as i32 + (*fresh179 as i32 - '0' as i32);
                if !(digit(**fmt as i32) != 0
                    && a <= ((if (::core::mem::size_of::<u64>() as u64)
                        < ::core::mem::size_of::<i32>() as u64
                    {
                        !(0u64)
                    } else {
                        0x7FFFFFFF as u64
                    }) as i32
                        - 9 as i32)
                        / 10 as i32)
                {
                    break;
                }
            }
            return a;
        };
    }
}
pub unsafe extern "C" fn getnumlimit(h: *mut Header, fmt: *mut *const i8, df: i32) -> i32 {
    unsafe {
        let size: i32 = getnum(fmt, df);
        if ((size > 16 as i32 || size <= 0) as i32 != 0) as i64 != 0 {
            return lual_error(
                (*h).state,
                b"integral size (%d) out of limits [1,%d]\0" as *const u8 as *const i8,
                size,
                16 as i32,
            );
        }
        return size;
    }
}
pub unsafe extern "C" fn initheader(state: *mut State, h: *mut Header) {
    unsafe {
        (*h).state = state;
        (*h).islittle = NATIVE_ENDIAN.little as i32;
        (*h).maxalign = 1;
    }
}
pub unsafe extern "C" fn getoption(h: *mut Header, fmt: *mut *const i8, size: *mut i32) -> K {
    unsafe {
        let fresh180 = *fmt;
        *fmt = (*fmt).offset(1);
        let opt: i32 = *fresh180 as i32;
        *size = 0;
        match opt {
            98 => {
                *size = ::core::mem::size_of::<i8>() as i32;
                return K::Integer;
            }
            66 => {
                *size = ::core::mem::size_of::<i8>() as i32;
                return K::Unsigned;
            }
            104 => {
                *size = ::core::mem::size_of::<i16>() as i32;
                return K::Integer;
            }
            72 => {
                *size = ::core::mem::size_of::<i16>() as i32;
                return K::Unsigned;
            }
            108 => {
                *size = ::core::mem::size_of::<i64>() as i32;
                return K::Integer;
            }
            76 => {
                *size = ::core::mem::size_of::<i64>() as i32;
                return K::Unsigned;
            }
            106 => {
                *size = ::core::mem::size_of::<i64>() as i32;
                return K::Integer;
            }
            74 => {
                *size = ::core::mem::size_of::<i64>() as i32;
                return K::Unsigned;
            }
            84 => {
                *size = ::core::mem::size_of::<u64>() as i32;
                return K::Unsigned;
            }
            102 => {
                *size = ::core::mem::size_of::<libc::c_float>() as i32;
                return K::Float;
            }
            110 => {
                *size = ::core::mem::size_of::<f64>() as i32;
                return K::Number;
            }
            100 => {
                *size = ::core::mem::size_of::<f64>() as i32;
                return K::Double;
            }
            105 => {
                *size = getnumlimit(h, fmt, ::core::mem::size_of::<i32>() as i32);
                return K::Integer;
            }
            73 => {
                *size = getnumlimit(h, fmt, ::core::mem::size_of::<i32>() as i32);
                return K::Unsigned;
            }
            115 => {
                *size = getnumlimit(h, fmt, ::core::mem::size_of::<u64>() as i32);
                return K::String;
            }
            99 => {
                *size = getnum(fmt, -1);
                if ((*size == -1) as i32 != 0) as i64 != 0 {
                    lual_error(
                        (*h).state,
                        b"missing size for format option 'c'\0" as *const u8 as *const i8,
                    );
                }
                return K::Character;
            }
            122 => return K::ZString,
            120 => {
                *size = 1;
                return K::Padding;
            }
            88 => return K::PaddingAlignment,
            32 => {}
            60 => {
                (*h).islittle = 1;
            }
            62 => {
                (*h).islittle = 0;
            }
            61 => {
                (*h).islittle = NATIVE_ENDIAN.little as i32;
            }
            33 => {
                let maxalign: i32 = 8;
                (*h).maxalign = getnumlimit(h, fmt, maxalign);
            }
            _ => {
                lual_error(
                    (*h).state,
                    b"invalid format option '%c'\0" as *const u8 as *const i8,
                    opt,
                );
            }
        }
        return K::NoOperator;
    }
}
pub unsafe extern "C" fn getdetails(
    h: *mut Header,
    totalsize: u64,
    fmt: *mut *const i8,
    total_size: *mut i32,
    ntoalign: *mut i32,
) -> K {
    unsafe {
        let opt: K = getoption(h, fmt, total_size);
        let mut align: i32 = *total_size;
        if opt as u32 == K::PaddingAlignment as u32 {
            if **fmt as i32 == '\0' as i32
                || getoption(h, fmt, &mut align) as u32 == K::Character as u32
                || align == 0
            {
                lual_argerror(
                    (*h).state,
                    1,
                    b"invalid next option for option 'X'\0" as *const u8 as *const i8,
                );
            }
        }
        if align <= 1 || opt as u32 == K::Character as u32 {
            *ntoalign = 0;
        } else {
            if align > (*h).maxalign {
                align = (*h).maxalign;
            }
            if ((align & align - 1 != 0) as i32 != 0) as i64 != 0 {
                lual_argerror(
                    (*h).state,
                    1,
                    b"format asks for alignment not power of 2\0" as *const u8 as *const i8,
                );
            }
            *ntoalign = align - (totalsize & (align - 1) as u64) as i32 & align - 1;
        }
        return opt;
    }
}
pub unsafe extern "C" fn packint(
    b: *mut Buffer,
    mut n: u64,
    islittle: i32,
    size: i32,
    is_negative_: i32,
) {
    unsafe {
        let buffer: *mut i8 = (*b).lual_prepbuffsize(size as u64);
        let mut i: i32;
        *buffer.offset((if islittle != 0 { 0 } else { size - 1 }) as isize) =
            (n & ((1 << 8) - 1) as u64) as i8;
        i = 1;
        while i < size {
            n >>= 8;
            *buffer.offset((if islittle != 0 { i } else { size - 1 - i }) as isize) =
                (n & ((1 << 8) - 1) as u64) as i8;
            i += 1;
        }
        if is_negative_ != 0 && size > ::core::mem::size_of::<i64>() as i32 {
            i = ::core::mem::size_of::<i64>() as i32;
            while i < size {
                *buffer.offset((if islittle != 0 { i } else { size - 1 - i }) as isize) =
                    ((1 << 8) - 1) as i8;
                i += 1;
            }
        }
        (*b).length = ((*b).length as u64).wrapping_add(size as u64) as u64;
    }
}
pub unsafe extern "C" fn copywithendian(
    mut dest: *mut i8,
    mut src: *const i8,
    mut size: i32,
    islittle: i32,
) {
    unsafe {
        if islittle == NATIVE_ENDIAN.little as i32 {
            memcpy(
                dest as *mut libc::c_void,
                src as *const libc::c_void,
                size as usize,
            );
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
pub unsafe extern "C" fn str_pack(state: *mut State) -> i32 {
    unsafe {
        let mut b = Buffer::new();
        let mut h: Header = Header {
            state: std::ptr::null_mut(),
            islittle: 0,
            maxalign: 0,
        };
        let mut fmt: *const i8 = lual_checklstring(state, 1, std::ptr::null_mut());
        let mut arg: i32 = 1;
        let mut totalsize: u64 = 0;
        initheader(state, &mut h);
        (*state).push_nil();
        b.lual_buffinit(state);
        while *fmt as i32 != '\0' as i32 {
            let mut size: i32 = 0;
            let mut ntoalign: i32 = 0;
            let opt: K = getdetails(&mut h, totalsize, &mut fmt, &mut size, &mut ntoalign);
            totalsize = (totalsize as u64).wrapping_add((ntoalign + size) as u64) as u64;
            loop {
                let fresh184 = ntoalign;
                ntoalign = ntoalign - 1;
                if !(fresh184 > 0) {
                    break;
                }
                (b.length < b.size || !(b.lual_prepbuffsize(1 as u64)).is_null()) as i32;
                let fresh185 = b.length;
                b.length = (b.length).wrapping_add(1);
                *(b.pointer).offset(fresh185 as isize) = 0 as i8;
            }
            arg += 1;
            let current_block_33: u64;
            match opt as u32 {
                0 => {
                    let n: i64 = lual_checkinteger(state, arg);
                    if size < ::core::mem::size_of::<i64>() as i32 {
                        let lim: i64 = 1 << size * 8 - 1;
                        (((-lim <= n && n < lim) as i32 != 0) as i64 != 0
                            || lual_argerror(
                                state,
                                arg,
                                b"integer overflow\0" as *const u8 as *const i8,
                            ) != 0) as i32;
                    }
                    packint(&mut b, n as u64, h.islittle, size, (n < 0) as i32);
                    current_block_33 = 3222590281903869779;
                }
                1 => {
                    let n_0: i64 = lual_checkinteger(state, arg);
                    if size < ::core::mem::size_of::<i64>() as i32 {
                        ((((n_0 as u64) < (1 as u64) << size * 8) as i32 != 0) as i64 != 0
                            || lual_argerror(
                                state,
                                arg,
                                b"unsigned overflow\0" as *const u8 as *const i8,
                            ) != 0) as i32;
                    }
                    packint(&mut b, n_0 as u64, h.islittle, size, 0);
                    current_block_33 = 3222590281903869779;
                }
                2 => {
                    let mut f: libc::c_float = lual_checknumber(state, arg) as libc::c_float;
                    let buffer: *mut i8 =
                        b.lual_prepbuffsize(::core::mem::size_of::<libc::c_float>() as u64);
                    copywithendian(
                        buffer,
                        &mut f as *mut libc::c_float as *mut i8,
                        ::core::mem::size_of::<libc::c_float>() as i32,
                        h.islittle,
                    );
                    b.length = (b.length as u64).wrapping_add(size as u64) as u64;
                    current_block_33 = 3222590281903869779;
                }
                3 => {
                    let mut f_0: f64 = lual_checknumber(state, arg);
                    let buff_0: *mut i8 = b.lual_prepbuffsize(::core::mem::size_of::<f64>() as u64);
                    copywithendian(
                        buff_0,
                        &mut f_0 as *mut f64 as *mut i8,
                        ::core::mem::size_of::<f64>() as i32,
                        h.islittle,
                    );
                    b.length = (b.length as u64).wrapping_add(size as u64) as u64;
                    current_block_33 = 3222590281903869779;
                }
                4 => {
                    let mut f_1: f64 = lual_checknumber(state, arg);
                    let buff_1: *mut i8 = b.lual_prepbuffsize(::core::mem::size_of::<f64>() as u64);
                    copywithendian(
                        buff_1,
                        &mut f_1 as *mut f64 as *mut i8,
                        ::core::mem::size_of::<f64>() as i32,
                        h.islittle,
                    );
                    b.length = (b.length as u64).wrapping_add(size as u64) as u64;
                    current_block_33 = 3222590281903869779;
                }
                5 => {
                    let mut length: u64 = 0;
                    let s: *const i8 = lual_checklstring(state, arg, &mut length);
                    (((length <= size as u64) as i32 != 0) as i64 != 0
                        || lual_argerror(
                            state,
                            arg,
                            b"string longer than given size\0" as *const u8 as *const i8,
                        ) != 0) as i32;
                    b.lual_addlstring(s, length);
                    loop {
                        let fresh186 = length;
                        length = length.wrapping_add(1);
                        if !(fresh186 < size as u64) {
                            break;
                        }
                        (b.length < b.size || !(b.lual_prepbuffsize(1 as u64)).is_null()) as i32;
                        let fresh187 = b.length;
                        b.length = (b.length).wrapping_add(1);
                        *(b.pointer).offset(fresh187 as isize) = 0 as i8;
                    }
                    current_block_33 = 3222590281903869779;
                }
                6 => {
                    let mut length_0: u64 = 0;
                    let s_0: *const i8 = lual_checklstring(state, arg, &mut length_0);
                    (((size >= ::core::mem::size_of::<u64>() as i32
                        || length_0 < (1 as u64) << size * 8) as i32
                        != 0) as i64
                        != 0
                        || lual_argerror(
                            state,
                            arg,
                            b"string length does not fit in given size\0" as *const u8 as *const i8,
                        ) != 0) as i32;
                    packint(&mut b, length_0 as u64, h.islittle, size, 0);
                    b.lual_addlstring(s_0, length_0);
                    totalsize = (totalsize as u64).wrapping_add(length_0) as u64;
                    current_block_33 = 3222590281903869779;
                }
                7 => {
                    let mut length_1: u64 = 0;
                    let s_1: *const i8 = lual_checklstring(state, arg, &mut length_1);
                    (((strlen(s_1) == length_1) as i32 != 0) as i64 != 0
                        || lual_argerror(
                            state,
                            arg,
                            b"string contains zeros\0" as *const u8 as *const i8,
                        ) != 0) as i32;
                    b.lual_addlstring(s_1, length_1);
                    (b.length < b.size || !(b.lual_prepbuffsize(1 as u64)).is_null()) as i32;
                    let fresh188 = b.length;
                    b.length = (b.length).wrapping_add(1);
                    *(b.pointer).offset(fresh188 as isize) = '\0' as i8;
                    totalsize = (totalsize as u64).wrapping_add(length_1.wrapping_add(1 as u64))
                        as u64;
                    current_block_33 = 3222590281903869779;
                }
                8 => {
                    (b.length < b.size || !(b.lual_prepbuffsize(1 as u64)).is_null()) as i32;
                    let fresh189 = b.length;
                    b.length = (b.length).wrapping_add(1);
                    *(b.pointer).offset(fresh189 as isize) = 0 as i8;
                    current_block_33 = 7383952003695197780;
                }
                9 | 10 => {
                    current_block_33 = 7383952003695197780;
                }
                _ => {
                    current_block_33 = 3222590281903869779;
                }
            }
            match current_block_33 {
                7383952003695197780 => {
                    arg -= 1;
                }
                _ => {}
            }
        }
        b.lual_pushresult();
        return 1;
    }
}
pub unsafe extern "C" fn str_packsize(state: *mut State) -> i32 {
    unsafe {
        let mut h: Header = Header {
            state: std::ptr::null_mut(),
            islittle: 0,
            maxalign: 0,
        };
        let mut fmt: *const i8 = lual_checklstring(state, 1, std::ptr::null_mut());
        let mut totalsize: u64 = 0;
        initheader(state, &mut h);
        while *fmt as i32 != '\0' as i32 {
            let mut size: i32 = 0;
            let mut ntoalign: i32 = 0;
            let opt: K = getdetails(&mut h, totalsize, &mut fmt, &mut size, &mut ntoalign);
            (((opt as u32 != K::String as u32 && opt as u32 != K::ZString as u32)
                as i32
                != 0) as i64
                != 0
                || lual_argerror(
                    state,
                    1,
                    b"variable-length format\0" as *const u8 as *const i8,
                ) != 0) as i32;
            size += ntoalign;
            (((totalsize
            <= (if (::core::mem::size_of::<u64>() as u64) < ::core::mem::size_of::<i32>() as u64 {
                !(0u64)
            } else {
                0x7FFFFFFF as u64
            })
            .wrapping_sub(size as u64)) as i32
            != 0) as i64
            != 0
            || lual_argerror(
                state,
                1,
                b"format result too large\0" as *const u8 as *const i8,
            ) != 0) as i32;
            totalsize = (totalsize as u64).wrapping_add(size as u64) as u64;
        }
        (*state).push_integer(totalsize as i64);
        return 1;
    }
}
pub unsafe extern "C" fn unpackint(
    state: *mut State,
    str: *const i8,
    islittle: i32,
    size: i32,
    issigned: i32,
) -> i64 {
    unsafe {
        let mut res: u64 = 0;
        let mut i: i32;
        let limit: i32 = if size <= ::core::mem::size_of::<i64>() as i32 {
            size
        } else {
            ::core::mem::size_of::<i64>() as i32
        };
        i = limit - 1;
        while i >= 0 {
            res <<= 8;
            res |=
                *str.offset((if islittle != 0 { i } else { size - 1 - i }) as isize) as u8 as u64;
            i -= 1;
        }
        if size < ::core::mem::size_of::<i64>() as i32 {
            if issigned != 0 {
                let mask: u64 = (1 as u64) << size * 8 - 1;
                res = (res ^ mask).wrapping_sub(mask);
            }
        } else if size > ::core::mem::size_of::<i64>() as i32 {
            let mask_0: i32 = if issigned == 0 || res as i64 >= 0 {
                0
            } else {
                (1 << 8) - 1
            };
            i = limit;
            while i < size {
                if ((*str.offset((if islittle != 0 { i } else { size - 1 - i }) as isize) as u8
                    as i32
                    != mask_0) as i32
                    != 0) as i64
                    != 0
                {
                    lual_error(
                        state,
                        b"%d-byte integer does not fit into Lua Integer\0" as *const u8
                            as *const i8,
                        size,
                    );
                }
                i += 1;
            }
        }
        return res as i64;
    }
}
pub unsafe extern "C" fn str_unpack(state: *mut State) -> i32 {
    unsafe {
        let mut h: Header = Header {
            state: std::ptr::null_mut(),
            islittle: 0,
            maxalign: 0,
        };
        let mut fmt: *const i8 = lual_checklstring(state, 1, std::ptr::null_mut());
        let mut ld: u64 = 0;
        let data: *const i8 = lual_checklstring(state, 2, &mut ld);
        let mut pos: u64 =
            (get_position_relative(lual_optinteger(state, 3, 1 as i64), ld)).wrapping_sub(1 as u64);
        let mut n: i32 = 0;
        (((pos <= ld) as i32 != 0) as i64 != 0
            || lual_argerror(
                state,
                3,
                b"initial position out of string\0" as *const u8 as *const i8,
            ) != 0) as i32;
        initheader(state, &mut h);
        while *fmt as i32 != '\0' as i32 {
            let mut size: i32 = 0;
            let mut ntoalign: i32 = 0;
            let opt: K = getdetails(&mut h, pos, &mut fmt, &mut size, &mut ntoalign);
            ((((ntoalign as u64).wrapping_add(size as u64) <= ld.wrapping_sub(pos)) as i32 != 0)
                as i64
                != 0
                || lual_argerror(
                    state,
                    2,
                    b"data string too short\0" as *const u8 as *const i8,
                ) != 0) as i32;
            pos = (pos as u64).wrapping_add(ntoalign as u64) as u64;
            lual_checkstack(state, 2, b"too many results\0" as *const u8 as *const i8);
            n += 1;
            match opt as u32 {
                0 | 1 => {
                    let res: i64 = unpackint(
                        state,
                        data.offset(pos as isize),
                        h.islittle,
                        size,
                        (opt as u32 == K::Integer as u32) as i32,
                    );
                    (*state).push_integer(res);
                }
                2 => {
                    let mut f: libc::c_float = 0.0;
                    copywithendian(
                        &mut f as *mut libc::c_float as *mut i8,
                        data.offset(pos as isize),
                        ::core::mem::size_of::<libc::c_float>() as i32,
                        h.islittle,
                    );
                    (*state).push_number(f as f64);
                }
                3 => {
                    let mut f_0: f64 = 0.0;
                    copywithendian(
                        &mut f_0 as *mut f64 as *mut i8,
                        data.offset(pos as isize),
                        ::core::mem::size_of::<f64>() as i32,
                        h.islittle,
                    );
                    (*state).push_number(f_0);
                }
                4 => {
                    let mut f_1: f64 = 0.0;
                    copywithendian(
                        &mut f_1 as *mut f64 as *mut i8,
                        data.offset(pos as isize),
                        ::core::mem::size_of::<f64>() as i32,
                        h.islittle,
                    );
                    (*state).push_number(f_1);
                }
                5 => {
                    lua_pushlstring(state, data.offset(pos as isize), size as u64);
                }
                6 => {
                    let length: u64 =
                        unpackint(state, data.offset(pos as isize), h.islittle, size, 0) as u64;
                    (((length <= ld.wrapping_sub(pos).wrapping_sub(size as u64)) as i32 != 0) as i32
                        as i64
                        != 0
                        || lual_argerror(
                            state,
                            2,
                            b"data string too short\0" as *const u8 as *const i8,
                        ) != 0) as i32;
                    lua_pushlstring(
                        state,
                        data.offset(pos as isize).offset(size as isize),
                        length,
                    );
                    pos = (pos as u64).wrapping_add(length) as u64;
                }
                7 => {
                    let length_0: u64 = strlen(data.offset(pos as isize));
                    (((pos.wrapping_add(length_0) < ld) as i32 != 0) as i64 != 0
                        || lual_argerror(
                            state,
                            2,
                            b"unfinished string for format 'zio'\0" as *const u8 as *const i8,
                        ) != 0) as i32;
                    lua_pushlstring(state, data.offset(pos as isize), length_0);
                    pos = (pos as u64).wrapping_add(length_0.wrapping_add(1 as u64)) as u64;
                }
                9 | 8 | 10 => {
                    n -= 1;
                }
                _ => {}
            }
            pos = (pos as u64).wrapping_add(size as u64) as u64;
        }
        (*state).push_integer(pos.wrapping_add(1 as u64) as i64);
        return n + 1;
    }
}
pub const STRING_FUNCTIONS: [RegisteredFunction; 18] = {
    [
        {
            RegisteredFunction {
                name: b"byte\0" as *const u8 as *const i8,
                function: Some(str_byte as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"char\0" as *const u8 as *const i8,
                function: Some(str_char as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"dump\0" as *const u8 as *const i8,
                function: Some(str_dump as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"find\0" as *const u8 as *const i8,
                function: Some(str_find as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"format\0" as *const u8 as *const i8,
                function: Some(str_format as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"gmatch\0" as *const u8 as *const i8,
                function: Some(GMatchState::gmatch as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"gsub\0" as *const u8 as *const i8,
                function: Some(str_gsub as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"len\0" as *const u8 as *const i8,
                function: Some(str_len as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"lower\0" as *const u8 as *const i8,
                function: Some(str_lower as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"match\0" as *const u8 as *const i8,
                function: Some(str_match as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"rep\0" as *const u8 as *const i8,
                function: Some(str_rep as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"reverse\0" as *const u8 as *const i8,
                function: Some(str_reverse as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"sub\0" as *const u8 as *const i8,
                function: Some(str_sub as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"upper\0" as *const u8 as *const i8,
                function: Some(str_upper as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"pack\0" as *const u8 as *const i8,
                function: Some(str_pack as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"packsize\0" as *const u8 as *const i8,
                function: Some(str_packsize as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"unpack\0" as *const u8 as *const i8,
                function: Some(str_unpack as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: std::ptr::null(),
                function: None,
            }
        },
    ]
};
pub unsafe extern "C" fn createmetatable(state: *mut State) {
    unsafe {
        (*state).lua_createtable();
        lual_setfuncs(state, STRING_METAMETHODS.as_ptr(), 0);
        lua_pushstring(state, b"\0" as *const u8 as *const i8);
        lua_pushvalue(state, -2);
        lua_setmetatable(state, -2);
        lua_settop(state, -2);
        lua_pushvalue(state, -2);
        lua_setfield(state, -2, b"__index\0" as *const u8 as *const i8);
        lua_settop(state, -2);
    }
}
pub unsafe extern "C" fn luaopen_string(state: *mut State) -> i32 {
    unsafe {
        lual_checkversion_(
            state,
            504.0,
            (::core::mem::size_of::<i64>() as u64)
                .wrapping_mul(16 as u64)
                .wrapping_add(::core::mem::size_of::<f64>() as u64),
        );
        (*state).lua_createtable();
        lual_setfuncs(state, STRING_FUNCTIONS.as_ptr(), 0);
        createmetatable(state);
        return 1;
    }
}
