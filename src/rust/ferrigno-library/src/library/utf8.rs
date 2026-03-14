use crate::buffer::*;
use crate::functionstate::MAX_INT;
use crate::registeredfunction::*;
use crate::state::*;
use crate::tdefaultnew::*;
use std::ptr::*;
pub fn u_posrelat(position: i64, length: usize) -> i64 {
    if position >= 0 {
        position
    } else if -position as usize > length {
        0
    } else {
        length as i64 + position + 1
    }
}
pub unsafe fn utf8_decode(mut s: *const i8, value: *mut u32, strict: bool) -> *const i8 {
    unsafe {
        pub const LIMITS: [u32; 6] = [0xFFFFFFFFu32, 0x80u32, 0x800u32, 0x10000u32, 0x200000u32, 0x4000000u32];
        let mut c: u32 = *s.add(0) as u8 as u32;
        let mut res: u32 = 0;
        if c < 0x80 {
            res = c;
        } else {
            let mut count: i32 = 0;
            while c & 0x40_u32 != 0 {
                count += 1;
                let cc: u32 = *s.add(count as usize) as u8 as u32;
                if cc & 0xc0_u32 != 0x80_u32 {
                    return null();
                }
                res = res << 6 | cc & 0x3f_u32;
                c <<= 1;
            }
            res |= (c & 0x7f_u32) << (count * 5);
            if count > 5 || res > MAX_INT as u32 || res < LIMITS[count as usize] {
                return null();
            }
            s = s.add(count as usize);
        }
        if strict && (res > 0x10ffff_u32 || (0xd800_u32..=0xdfff_u32).contains(&res)) {
            return null();
        }
        if !value.is_null() {
            *value = res;
        }
        s.add(1)
    }
}
pub unsafe fn utflen(state: *mut State) -> i32 {
    unsafe {
        let mut n: i64 = 0;
        let mut length: usize = 0;
        let s: *const i8 = lual_checklstring(state, 1, &mut length);
        let mut posi: i64 = u_posrelat(lual_optinteger(state, 2, 1_i64), length);
        let mut posj: i64 = u_posrelat(lual_optinteger(state, 3, -1_i64), length);
        let lax = lua_toboolean(state, 4);
        if !(1 <= posi && {
            posi -= 1;
            posi <= length as i64
        }) {
            lual_argerror(state, 2, c"initial position out of bounds".as_ptr());
            0;
        }
        posj -= 1;
        if posj >= length as i64 {
            lual_argerror(state, 3, c"final position out of bounds".as_ptr());
            0;
        }
        while posi <= posj {
            let s1: *const i8 = utf8_decode(s.add(posi as usize), null_mut(), !lax);
            if s1.is_null() {
                (*state).push_nil();
                (*state).push_integer(posi + 1);
                return 2;
            }
            posi = s1.offset_from(s) as i64;
            n += 1;
        }
        (*state).push_integer(n);
        1
    }
}
pub unsafe fn codepoint(state: *mut State) -> i32 {
    unsafe {
        let mut length: usize = 0;
        let mut stringpointer: *const i8 = lual_checklstring(state, 1, &mut length);
        let posi: i64 = u_posrelat(lual_optinteger(state, 2, 1_i64), length);
        let pose: i64 = u_posrelat(lual_optinteger(state, 3, posi), length);
        let lax = lua_toboolean(state, 4);
        let mut n: i32;
        if posi < 1 {
            lual_argerror(state, 2, c"out of bounds".as_ptr());
            0;
        }
        if pose > length as i64 {
            lual_argerror(state, 3, c"out of bounds".as_ptr());
            0;
        }
        if posi > pose {
            return 0;
        }
        if pose - posi >= MAX_INT as i64 {
            return lual_error(state, c"string slice too long".as_ptr());
        }
        n = (pose - posi) as i32 + 1;
        lual_checkstack(state, n, c"string slice too long".as_ptr());
        n = 0;
        let stringend: *const i8 = stringpointer.add(pose as usize);
        stringpointer = stringpointer.add((posi - 1) as usize);
        while stringpointer < stringend {
            let mut code: u32 = 0;
            stringpointer = utf8_decode(stringpointer, &mut code, !lax);
            if stringpointer.is_null() {
                return lual_error(state, c"invalid UTF-8 code".as_ptr());
            }
            (*state).push_integer(code as i64);
            n += 1;
        }
        n
    }
}
pub unsafe fn pushutfchar(state: *mut State, arg: i32) {
    unsafe {
        let code: usize = lual_checkinteger(state, arg) as usize;
        if code > MAX_INT {
            lual_argerror(state, arg, c"value out of range".as_ptr());
            0;
        }
        lua_pushfstring(state, c"%U".as_ptr(), code as i64);
    }
}
pub unsafe fn utfchar(state: *mut State) -> i32 {
    unsafe {
        let n: i32 = (*state).get_top();
        if n == 1 {
            pushutfchar(state, 1);
        } else {
            let mut b = Buffer::new();
            b.initialize(state);
            for i in 1..(1 + n) {
                pushutfchar(state, i);
                b.add_value();
            }
            b.push_result();
        }
        1
    }
}
pub unsafe fn byteoffset(state: *mut State) -> i32 {
    unsafe {
        let mut length: usize = 0;
        let s: *const i8 = lual_checklstring(state, 1, &mut length);
        let mut n: i64 = lual_checkinteger(state, 2);
        let mut posi: i64 = (if n >= 0 { 1_usize } else { length + 1 }) as i64;
        posi = u_posrelat(lual_optinteger(state, 3, posi), length);
        if !(1 <= posi && {
            posi -= 1;
            posi <= length as i64
        }) {
            lual_argerror(state, 3, c"position out of bounds".as_ptr());
            0;
        }
        if n == 0 {
            while posi > 0 && *s.add(posi as usize) as i32 & 0xc0_i32 == 0x80_i32 {
                posi -= 1;
            }
        } else {
            if *s.add(posi as usize) as i32 & 0xc0_i32 == 0x80_i32 {
                return lual_error(state, c"initial position is a continuation byte".as_ptr());
            }
            if n < 0 {
                while n < 0 && posi > 0 {
                    loop {
                        posi -= 1;
                        if !(posi > 0 && *s.add(posi as usize) as i32 & 0xc0_i32 == 0x80_i32) {
                            break;
                        }
                    }
                    n += 1;
                }
            } else {
                n -= 1;
                while n > 0 && posi < length as i64 {
                    loop {
                        posi += 1;
                        if *s.add(posi as usize) as i32 & 0xc0_i32 != 0x80_i32 {
                            break;
                        }
                    }
                    n -= 1;
                }
            }
        }
        if n != 0 {
            (*state).push_nil();
            return 1;
        }
        (*state).push_integer(posi + 1);
        if *s.add(posi as usize) as i32 & 0x80_i32 != 0 {
            if *s.add(posi as usize) as i32 & 0xc0_i32 == 0x80_i32 {
                return lual_error(state, c"initial position is a continuation byte".as_ptr());
            }
            while posi + 1 < length as i64 && *s.add((posi + 1) as usize) as i32 & 0xc0_i32 == 0x80_i32 {
                posi += 1;
            }
        }
        (*state).push_integer(posi + 1);
        2
    }
}
pub unsafe fn iter_aux(state: *mut State, strict: bool) -> i32 {
    unsafe {
        let mut length: usize = 0;
        let s: *const i8 = lual_checklstring(state, 1, &mut length);
        let mut n: usize = lua_tointegerx(state, 2, null_mut()) as usize;
        if n < length {
            while *s.add(n as usize) as i32 & 0xc0_i32 == 0x80_i32 {
                n += 1;
            }
        }
        if n >= length {
            0
        } else {
            let mut code: u32 = 0;
            let next: *const i8 = utf8_decode(s.add(n as usize), &mut code, strict);
            if next.is_null() || *next as i32 & 0xc0_i32 == 0x80_i32 {
                return lual_error(state, c"invalid UTF-8 code".as_ptr());
            }
            (*state).push_integer((n + 1) as i64);
            (*state).push_integer(code as i64);
            2
        }
    }
}
pub unsafe fn iter_auxstrict(state: *mut State) -> i32 {
    unsafe { iter_aux(state, true) }
}
pub unsafe fn iter_auxlax(state: *mut State) -> i32 {
    unsafe { iter_aux(state, false) }
}
pub unsafe fn iter_codes(state: *mut State) -> i32 {
    unsafe {
        let lax = lua_toboolean(state, 2);
        let s: *const i8 = lual_checklstring(state, 1, null_mut());
        if *s as i32 & 0xc0_i32 == 0x80_i32 {
            lual_argerror(state, 1, c"invalid UTF-8 code".as_ptr());
            0;
        }
        lua_pushcclosure(
            state,
            if lax {
                Some(iter_auxlax as unsafe fn(*mut State) -> i32)
            } else {
                Some(iter_auxstrict as unsafe fn(*mut State) -> i32)
            },
            0,
        );
        lua_pushvalue(state, 1);
        (*state).push_integer(0);
        3
    }
}
pub const UTF8_FUNCTIONS: [RegisteredFunction; 5] = {
    [
        {
            RegisteredFunction {
                registeredfunction_name: c"offset".as_ptr(),
                registeredfunction_function: Some(byteoffset as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"codepoint".as_ptr(),
                registeredfunction_function: Some(codepoint as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"char".as_ptr(),
                registeredfunction_function: Some(utfchar as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"len".as_ptr(),
                registeredfunction_function: Some(utflen as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"codes".as_ptr(),
                registeredfunction_function: Some(iter_codes as unsafe fn(*mut State) -> i32),
            }
        },
    ]
};
pub unsafe fn luaopen_utf8(state: *mut State) -> i32 {
    unsafe {
        (*state).lua_createtable();
        lual_setfuncs(state, UTF8_FUNCTIONS.as_ptr(), UTF8_FUNCTIONS.len(), 0);
        lua_pushlstring(
            state,
            b"[\0-\x7F\xC2-\xFD][\x80-\xBF]*".as_ptr() as *const i8,
            size_of::<[i8; 15]>() - 1,
        );
        lua_setfield(state, -2, c"charpattern".as_ptr());
        1
    }
}
