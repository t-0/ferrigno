use crate::registeredfunction::*;
use crate::state::*;
use crate::buffer::*;
use crate::new::*;
pub unsafe extern "C" fn u_posrelat(pos: i64, length: u64) -> i64 {
    if pos >= 0 {
        return pos;
    } else if (0u64).wrapping_sub(pos as u64) > length {
        return 0;
    } else {
        return length as i64 + pos + 1;
    };
}
pub unsafe extern "C" fn utf8_decode(mut s: *const i8, value: *mut u32, strict: i32) -> *const i8 {
    unsafe {
        pub const LIMITS: [u32; 6] = [
            !(0u32),
            0x80 as u32,
            0x800 as u32,
            0x10000 as u32,
            0x200000 as u32,
            0x4000000 as u32,
        ];
        let mut c: u32 = *s.offset(0 as isize) as u8 as u32;
        let mut res: u32 = 0u32;
        if c < 0x80 as u32 {
            res = c;
        } else {
            let mut count: i32 = 0;
            while c & 0x40 as u32 != 0 {
                count += 1;
                let cc: u32 = *s.offset(count as isize) as u8 as u32;
                if !(cc & 0xc0 as u32 == 0x80 as u32) {
                    return std::ptr::null();
                }
                res = res << 6 | cc & 0x3f as u32;
                c <<= 1;
            }
            res |= (c & 0x7f as u32) << count * 5;
            if count > 5 || res > 0x7fffffff as u32 || res < LIMITS[count as usize] {
                return std::ptr::null();
            }
            s = s.offset(count as isize);
        }
        if strict != 0 {
            if res > 0x10ffff as u32 || 0xd800 as u32 <= res && res <= 0xdfff as u32 {
                return std::ptr::null();
            }
        }
        if !value.is_null() {
            *value = res;
        }
        return s.offset(1 as isize);
    }
}
pub unsafe extern "C" fn utflen(state: *mut State) -> i32 {
    unsafe {
        let mut n: i64 = 0;
        let mut length: u64 = 0;
        let s: *const i8 = lual_checklstring(state, 1, &mut length);
        let mut posi: i64 = u_posrelat(lual_optinteger(state, 2, 1 as i64), length);
        let mut posj: i64 = u_posrelat(lual_optinteger(state, 3, -1 as i64), length);
        let lax: i32 = lua_toboolean(state, 4);
        (((1 <= posi && {
            posi -= 1;
            posi <= length as i64
        }) as i32
            != 0) as i64
            != 0
            || lual_argerror(
                state,
                2,
                b"initial position out of bounds\0" as *const u8 as *const i8,
            ) != 0) as i32;
        posj -= 1;
        (((posj < length as i64) as i32 != 0) as i64 != 0
            || lual_argerror(
                state,
                3,
                b"final position out of bounds\0" as *const u8 as *const i8,
            ) != 0) as i32;
        while posi <= posj {
            let s1: *const i8 = utf8_decode(
                s.offset(posi as isize),
                std::ptr::null_mut(),
                (lax == 0) as i32,
            );
            if s1.is_null() {
                (*state).push_nil();
                (*state).push_integer(posi + 1);
                return 2;
            }
            posi = s1.offset_from(s) as i64;
            n += 1;
        }
        (*state).push_integer(n);
        return 1;
    }
}
pub unsafe extern "C" fn codepoint(state: *mut State) -> i32 {
    unsafe {
        let mut length: u64 = 0;
        let mut s: *const i8 = lual_checklstring(state, 1, &mut length);
        let posi: i64 = u_posrelat(lual_optinteger(state, 2, 1 as i64), length);
        let pose: i64 = u_posrelat(lual_optinteger(state, 3, posi), length);
        let lax: i32 = lua_toboolean(state, 4);
        let mut n: i32;
        let se: *const i8;
        (((posi >= 1) as i32 != 0) as i64 != 0
            || lual_argerror(state, 2, b"out of bounds\0" as *const u8 as *const i8) != 0)
            as i32;
        (((pose <= length as i64) as i32 != 0) as i64 != 0
            || lual_argerror(state, 3, b"out of bounds\0" as *const u8 as *const i8) != 0)
            as i32;
        if posi > pose {
            return 0;
        }
        if pose - posi >= 0x7FFFFFFF as i64 {
            return lual_error(state, b"string slice too long\0" as *const u8 as *const i8);
        }
        n = (pose - posi) as i32 + 1;
        lual_checkstack(
            state,
            n,
            b"string slice too long\0" as *const u8 as *const i8,
        );
        n = 0;
        se = s.offset(pose as isize);
        s = s.offset((posi - 1) as isize);
        while s < se {
            let mut code: u32 = 0;
            s = utf8_decode(s, &mut code, (lax == 0) as i32);
            if s.is_null() {
                return lual_error(state, b"invalid UTF-8 code\0" as *const u8 as *const i8);
            }
            (*state).push_integer(code as i64);
            n += 1;
        }
        return n;
    }
}
pub unsafe extern "C" fn pushutfchar(state: *mut State, arg: i32) {
    unsafe {
        let code: u64 = lual_checkinteger(state, arg) as u64;
        (((code <= 0x7fffffff as u64) as i32 != 0) as i64 != 0
            || lual_argerror(
                state,
                arg,
                b"value out of range\0" as *const u8 as *const i8,
            ) != 0) as i32;
        lua_pushfstring(state, b"%U\0" as *const u8 as *const i8, code as i64);
    }
}
pub unsafe extern "C" fn utfchar(state: *mut State) -> i32 {
    unsafe {
        let n: i32 = (*state).get_top();
        if n == 1 {
            pushutfchar(state, 1);
        } else {
            let mut i: i32;
            let mut b = Buffer::new();
            b.lual_buffinit(state);
            i = 1;
            while i <= n {
                pushutfchar(state, i);
                b.lual_addvalue();
                i += 1;
            }
            b.lual_pushresult();
        }
        return 1;
    }
}
pub unsafe extern "C" fn byteoffset(state: *mut State) -> i32 {
    unsafe {
        let mut length: u64 = 0;
        let s: *const i8 = lual_checklstring(state, 1, &mut length);
        let mut n: i64 = lual_checkinteger(state, 2);
        let mut posi: i64 = (if n >= 0 {
            1 as u64
        } else {
            length.wrapping_add(1 as u64)
        }) as i64;
        posi = u_posrelat(lual_optinteger(state, 3, posi), length);
        (((1 <= posi && {
            posi -= 1;
            posi <= length as i64
        }) as i32
            != 0) as i64
            != 0
            || lual_argerror(
                state,
                3,
                b"position out of bounds\0" as *const u8 as *const i8,
            ) != 0) as i32;
        if n == 0 {
            while posi > 0 && *s.offset(posi as isize) as i32 & 0xc0 as i32 == 0x80 as i32 {
                posi -= 1;
            }
        } else {
            if *s.offset(posi as isize) as i32 & 0xc0 as i32 == 0x80 as i32 {
                return lual_error(
                    state,
                    b"initial position is a continuation byte\0" as *const u8 as *const i8,
                );
            }
            if n < 0 {
                while n < 0 && posi > 0 {
                    loop {
                        posi -= 1;
                        if !(posi > 0
                            && *s.offset(posi as isize) as i32 & 0xc0 as i32 == 0x80 as i32)
                        {
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
                        if !(*s.offset(posi as isize) as i32 & 0xc0 as i32 == 0x80 as i32) {
                            break;
                        }
                    }
                    n -= 1;
                }
            }
        }
        if n == 0 {
            (*state).push_integer(posi + 1);
        } else {
            (*state).push_nil();
        }
        return 1;
    }
}
pub unsafe extern "C" fn iter_aux(state: *mut State, strict: i32) -> i32 {
    unsafe {
        let mut length: u64 = 0;
        let s: *const i8 = lual_checklstring(state, 1, &mut length);
        let mut n: u64 = lua_tointegerx(state, 2, std::ptr::null_mut()) as u64;
        if n < length as u64 {
            while *s.offset(n as isize) as i32 & 0xc0 as i32 == 0x80 as i32 {
                n = n.wrapping_add(1);
            }
        }
        if n >= length as u64 {
            return 0;
        } else {
            let mut code: u32 = 0;
            let next: *const i8 = utf8_decode(s.offset(n as isize), &mut code, strict);
            if next.is_null() || *next as i32 & 0xc0 as i32 == 0x80 as i32 {
                return lual_error(state, b"invalid UTF-8 code\0" as *const u8 as *const i8);
            }
            (*state).push_integer(n.wrapping_add(1 as u64) as i64);
            (*state).push_integer(code as i64);
            return 2;
        };
    }
}
pub unsafe extern "C" fn iter_auxstrict(state: *mut State) -> i32 {
    unsafe {
        return iter_aux(state, 1);
    }
}
pub unsafe extern "C" fn iter_auxlax(state: *mut State) -> i32 {
    unsafe {
        return iter_aux(state, 0);
    }
}
pub unsafe extern "C" fn iter_codes(state: *mut State) -> i32 {
    unsafe {
        let lax: i32 = lua_toboolean(state, 2);
        let s: *const i8 = lual_checklstring(state, 1, std::ptr::null_mut());
        ((!(*s as i32 & 0xc0 as i32 == 0x80 as i32) as i32 != 0) as i64 != 0
            || lual_argerror(state, 1, b"invalid UTF-8 code\0" as *const u8 as *const i8) != 0)
            as i32;
        lua_pushcclosure(
            state,
            if lax != 0 {
                Some(iter_auxlax as unsafe extern "C" fn(*mut State) -> i32)
            } else {
                Some(iter_auxstrict as unsafe extern "C" fn(*mut State) -> i32)
            },
            0,
        );
        lua_pushvalue(state, 1);
        (*state).push_integer(0);
        return 3;
    }
}
pub const UTF8_FUNCTIONS: [RegisteredFunction; 7] = {
    [
        {
            RegisteredFunction {
                name: b"offset\0" as *const u8 as *const i8,
                function: Some(byteoffset as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"codepoint\0" as *const u8 as *const i8,
                function: Some(codepoint as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"char\0" as *const u8 as *const i8,
                function: Some(utfchar as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"len\0" as *const u8 as *const i8,
                function: Some(utflen as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"codes\0" as *const u8 as *const i8,
                function: Some(iter_codes as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"charpattern\0" as *const u8 as *const i8,
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
pub unsafe extern "C" fn luaopen_utf8(state: *mut State) -> i32 {
    unsafe {
        lual_checkversion_(
            state,
            504.0,
            (::core::mem::size_of::<i64>() as u64)
                .wrapping_mul(16 as u64)
                .wrapping_add(::core::mem::size_of::<f64>() as u64),
        );
        (*state).lua_createtable();
        lual_setfuncs(state, UTF8_FUNCTIONS.as_ptr(), 0);
        lua_pushlstring(
            state,
            b"[\0-\x7F\xC2-\xFD][\x80-\xBF]*\0" as *const u8 as *const i8,
            (::core::mem::size_of::<[i8; 15]>() as u64)
                .wrapping_div(::core::mem::size_of::<i8>() as u64)
                .wrapping_sub(1 as u64),
        );
        lua_setfield(state, -2, b"charpattern\0" as *const u8 as *const i8);
        return 1;
    }
}
