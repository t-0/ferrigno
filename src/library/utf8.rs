use crate::buffer::*;
use crate::interpreter::*;
use crate::new::*;
use crate::registeredfunction::*;
use rlua::*;
use std::ptr::*;
pub unsafe fn u_posrelat(position: i64, length: usize) -> i64 {
    if position >= 0 {
        return position;
    } else if (0usize).wrapping_sub(position as usize) > length {
        return 0;
    } else {
        return length as i64 + position + 1;
    };
}
pub unsafe fn utf8_decode(mut s: *const i8, value: *mut u32, strict: i32) -> *const i8 {
    unsafe {
        pub const LIMITS: [u32; 6] = [!(0u32), 0x80 as u32, 0x800 as u32, 0x10000 as u32, 0x200000 as u32, 0x4000000 as u32];
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
                    return null();
                }
                res = res << 6 | cc & 0x3f as u32;
                c <<= 1;
            }
            res |= (c & 0x7f as u32) << count * 5;
            if count > 5 || res > 0x7fffffff as u32 || res < LIMITS[count as usize] {
                return null();
            }
            s = s.offset(count as isize);
        }
        if strict != 0 {
            if res > 0x10ffff as u32 || 0xd800 as u32 <= res && res <= 0xdfff as u32 {
                return null();
            }
        }
        if !value.is_null() {
            *value = res;
        }
        return s.offset(1 as isize);
    }
}
pub unsafe fn utflen(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let mut n: i64 = 0;
        let mut length: usize = 0;
        let s: *const i8 = lual_checklstring(interpreter, 1, &mut length);
        let mut posi: i64 = u_posrelat(lual_optinteger(interpreter, 2, 1 as i64), length);
        let mut posj: i64 = u_posrelat(lual_optinteger(interpreter, 3, -1 as i64), length);
        let lax: i32 = lua_toboolean(interpreter, 4);
        (((1 <= posi && {
            posi -= 1;
            posi <= length as i64
        }) as i32
            != 0) as i64
            != 0
            || lual_argerror(interpreter, 2, make_cstring!("initial position out of bounds")) != 0) as i32;
        posj -= 1;
        (((posj < length as i64) as i32 != 0) as i64 != 0 || lual_argerror(interpreter, 3, make_cstring!("final position out of bounds")) != 0) as i32;
        while posi <= posj {
            let s1: *const i8 = utf8_decode(s.offset(posi as isize), null_mut(), (lax == 0) as i32);
            if s1.is_null() {
                (*interpreter).push_nil();
                (*interpreter).push_integer(posi + 1);
                return 2;
            }
            posi = s1.offset_from(s) as i64;
            n += 1;
        }
        (*interpreter).push_integer(n);
        return 1;
    }
}
pub unsafe fn codepoint(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let mut length: usize = 0;
        let mut s: *const i8 = lual_checklstring(interpreter, 1, &mut length);
        let posi: i64 = u_posrelat(lual_optinteger(interpreter, 2, 1 as i64), length);
        let pose: i64 = u_posrelat(lual_optinteger(interpreter, 3, posi), length);
        let lax: i32 = lua_toboolean(interpreter, 4);
        let mut n: i32;
        let se: *const i8;
        (((posi >= 1) as i32 != 0) as i64 != 0 || lual_argerror(interpreter, 2, make_cstring!("out of bounds")) != 0) as i32;
        (((pose <= length as i64) as i32 != 0) as i64 != 0 || lual_argerror(interpreter, 3, make_cstring!("out of bounds")) != 0) as i32;
        if posi > pose {
            return 0;
        }
        if pose - posi >= 0x7FFFFFFF as i64 {
            return lual_error(interpreter, make_cstring!("string slice too long"));
        }
        n = (pose - posi) as i32 + 1;
        lual_checkstack(interpreter, n, make_cstring!("string slice too long"));
        n = 0;
        se = s.offset(pose as isize);
        s = s.offset((posi - 1) as isize);
        while s < se {
            let mut code: u32 = 0;
            s = utf8_decode(s, &mut code, (lax == 0) as i32);
            if s.is_null() {
                return lual_error(interpreter, make_cstring!("invalid UTF-8 code"));
            }
            (*interpreter).push_integer(code as i64);
            n += 1;
        }
        return n;
    }
}
pub unsafe fn pushutfchar(interpreter: *mut Interpreter, arg: i32) {
    unsafe {
        let code: usize = lual_checkinteger(interpreter, arg) as usize;
        (((code <= 0x7fffffff as usize) as i32 != 0) as i64 != 0 || lual_argerror(interpreter, arg, make_cstring!("value out of range")) != 0) as i32;
        lua_pushfstring(interpreter, make_cstring!("%U"), code as i64);
    }
}
pub unsafe fn utfchar(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let n: i32 = (*interpreter).get_top();
        if n == 1 {
            pushutfchar(interpreter, 1);
        } else {
            let mut b = Buffer::new();
            b.initialize(interpreter);
            for i in 1..(1 + n) {
                pushutfchar(interpreter, i);
                b.add_value();
            }
            b.push_result();
        }
        return 1;
    }
}
pub unsafe fn byteoffset(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let mut length: usize = 0;
        let s: *const i8 = lual_checklstring(interpreter, 1, &mut length);
        let mut n: i64 = lual_checkinteger(interpreter, 2);
        let mut posi: i64 = (if n >= 0 { 1 as usize } else { length.wrapping_add(1 as usize) }) as i64;
        posi = u_posrelat(lual_optinteger(interpreter, 3, posi), length);
        (((1 <= posi && {
            posi -= 1;
            posi <= length as i64
        }) as i32
            != 0) as i64
            != 0
            || lual_argerror(interpreter, 3, make_cstring!("position out of bounds")) != 0) as i32;
        if n == 0 {
            while posi > 0 && *s.offset(posi as isize) as i32 & 0xc0 as i32 == 0x80 as i32 {
                posi -= 1;
            }
        } else {
            if *s.offset(posi as isize) as i32 & 0xc0 as i32 == 0x80 as i32 {
                return lual_error(interpreter, make_cstring!("initial position is a continuation byte"));
            }
            if n < 0 {
                while n < 0 && posi > 0 {
                    loop {
                        posi -= 1;
                        if !(posi > 0 && *s.offset(posi as isize) as i32 & 0xc0 as i32 == 0x80 as i32) {
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
            (*interpreter).push_integer(posi + 1);
        } else {
            (*interpreter).push_nil();
        }
        return 1;
    }
}
pub unsafe fn iter_aux(interpreter: *mut Interpreter, strict: i32) -> i32 {
    unsafe {
        let mut length: usize = 0;
        let s: *const i8 = lual_checklstring(interpreter, 1, &mut length);
        let mut n: usize = lua_tointegerx(interpreter, 2, null_mut()) as usize;
        if n < length as usize {
            while *s.offset(n as isize) as i32 & 0xc0 as i32 == 0x80 as i32 {
                n = n.wrapping_add(1);
            }
        }
        if n >= length as usize {
            return 0;
        } else {
            let mut code: u32 = 0;
            let next: *const i8 = utf8_decode(s.offset(n as isize), &mut code, strict);
            if next.is_null() || *next as i32 & 0xc0 as i32 == 0x80 as i32 {
                return lual_error(interpreter, make_cstring!("invalid UTF-8 code"));
            }
            (*interpreter).push_integer(n.wrapping_add(1 as usize) as i64);
            (*interpreter).push_integer(code as i64);
            return 2;
        };
    }
}
pub unsafe fn iter_auxstrict(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        return iter_aux(interpreter, 1);
    }
}
pub unsafe fn iter_auxlax(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        return iter_aux(interpreter, 0);
    }
}
pub unsafe fn iter_codes(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let lax: i32 = lua_toboolean(interpreter, 2);
        let s: *const i8 = lual_checklstring(interpreter, 1, null_mut());
        ((!(*s as i32 & 0xc0 as i32 == 0x80 as i32) as i32 != 0) as i64 != 0 || lual_argerror(interpreter, 1, make_cstring!("invalid UTF-8 code")) != 0) as i32;
        lua_pushcclosure(
            interpreter,
            if lax != 0 {
                Some(iter_auxlax as unsafe fn(*mut Interpreter) -> i32)
            } else {
                Some(iter_auxstrict as unsafe fn(*mut Interpreter) -> i32)
            },
            0,
        );
        lua_pushvalue(interpreter, 1);
        (*interpreter).push_integer(0);
        return 3;
    }
}
pub const UTF8_FUNCTIONS: [RegisteredFunction; 5] = {
    [
        { RegisteredFunction { name: make_cstring!("offset"), function: Some(byteoffset as unsafe fn(*mut Interpreter) -> i32) } },
        { RegisteredFunction { name: make_cstring!("codepoint"), function: Some(codepoint as unsafe fn(*mut Interpreter) -> i32) } },
        { RegisteredFunction { name: make_cstring!("char"), function: Some(utfchar as unsafe fn(*mut Interpreter) -> i32) } },
        { RegisteredFunction { name: make_cstring!("len"), function: Some(utflen as unsafe fn(*mut Interpreter) -> i32) } },
        { RegisteredFunction { name: make_cstring!("codes"), function: Some(iter_codes as unsafe fn(*mut Interpreter) -> i32) } },
    ]
};
pub unsafe fn luaopen_utf8(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        lual_checkversion_(interpreter, 504.0, (size_of::<i64>() as usize).wrapping_mul(16 as usize).wrapping_add(size_of::<f64>() as usize));
        (*interpreter).lua_createtable();
        lual_setfuncs(interpreter, UTF8_FUNCTIONS.as_ptr(), UTF8_FUNCTIONS.len(), 0);
        lua_pushlstring(
            interpreter,
            b"[\0-\x7F\xC2-\xFD][\x80-\xBF]*".as_ptr() as *const i8,
            (size_of::<[i8; 15]>() as usize).wrapping_div(size_of::<i8>() as usize).wrapping_sub(1 as usize),
        );
        lua_setfield(interpreter, -2, make_cstring!("charpattern"));
        return 1;
    }
}
