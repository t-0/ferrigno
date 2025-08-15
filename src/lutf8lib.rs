#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
unsafe extern "C" {
    pub type lua_State;
    fn lua_gettop(L: *mut lua_State) -> i32;
    fn lua_pushvalue(L: *mut lua_State, index: i32);
    fn lua_tointegerx(L: *mut lua_State, index: i32, isnum: *mut i32) -> i64;
    fn lua_toboolean(L: *mut lua_State, index: i32) -> i32;
    fn lua_pushnil(L: *mut lua_State);
    fn lua_pushinteger(L: *mut lua_State, n: i64);
    fn lua_pushlstring(L: *mut lua_State, s: *const libc::c_char, len: u64) -> *const libc::c_char;
    fn lua_pushfstring(L: *mut lua_State, fmt: *const libc::c_char, _: ...) -> *const libc::c_char;
    fn lua_pushcclosure(L: *mut lua_State, fn_0: CFunction, n: i32);
    fn lua_createtable(L: *mut lua_State, narr: i32, nrec: i32);
    fn lua_setfield(L: *mut lua_State, index: i32, k: *const libc::c_char);
    fn luaL_checkversion_(L: *mut lua_State, ver: f64, sz: u64);
    fn luaL_argerror(L: *mut lua_State, arg: i32, extramsg: *const libc::c_char) -> i32;
    fn luaL_checklstring(L: *mut lua_State, arg: i32, l: *mut u64) -> *const libc::c_char;
    fn luaL_checkinteger(L: *mut lua_State, arg: i32) -> i64;
    fn luaL_optinteger(L: *mut lua_State, arg: i32, def: i64) -> i64;
    fn luaL_checkstack(L: *mut lua_State, sz: i32, msg: *const libc::c_char);
    fn luaL_error(L: *mut lua_State, fmt: *const libc::c_char, _: ...) -> i32;
    fn luaL_setfuncs(L: *mut lua_State, l: *const luaL_Reg, nup: i32);
    fn luaL_buffinit(L: *mut lua_State, B: *mut luaL_Buffer);
    fn luaL_addvalue(B: *mut luaL_Buffer);
    fn luaL_pushresult(B: *mut luaL_Buffer);
}

pub type CFunction = Option<unsafe extern "C" fn(*mut lua_State) -> i32>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaL_Buffer {
    pub b: *mut libc::c_char,
    pub size: u64,
    pub n: u64,
    pub L: *mut lua_State,
    pub init: LUTF8LibC3RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union LUTF8LibC3RustUnnamed {
    pub n: f64,
    pub u: f64,
    pub s: *mut libc::c_void,
    pub i: i64,
    pub l: i64,
    pub b: [libc::c_char; 1024],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaL_Reg {
    pub name: *const libc::c_char,
    pub func: CFunction,
}
pub type utfint = u32;
unsafe extern "C" fn u_posrelat(mut pos: i64, mut len: u64) -> i64 {
    if pos >= 0i32 as i64 {
        return pos;
    } else if (0 as u32 as libc::c_ulong).wrapping_sub(pos as u64) > len {
        return 0i32 as i64;
    } else {
        return len as i64 + pos + 1i32 as i64;
    };
}
unsafe extern "C" fn utf8_decode(
    mut s: *const libc::c_char,
    mut val: *mut utfint,
    mut strict: i32,
) -> *const libc::c_char {
    static mut limits: [utfint; 6] = [
        !(0i32 as utfint),
        0x80 as i32 as utfint,
        0x800 as i32 as utfint,
        0x10000 as u32,
        0x200000 as u32,
        0x4000000 as u32,
    ];
    let mut c: u32 = *s.offset(0i32 as isize) as u8 as u32;
    let mut res: utfint = 0i32 as utfint;
    if c < 0x80 as i32 as u32 {
        res = c;
    } else {
        let mut count: i32 = 0i32;
        while c & 0x40 as i32 as u32 != 0 {
            count += 1;
            let mut cc: u32 = *s.offset(count as isize) as u8 as u32;
            if !(cc & 0xc0 as i32 as u32 == 0x80 as i32 as u32) {
                return 0 as *const libc::c_char;
            }
            res = res << 6i32 | cc & 0x3f as i32 as u32;
            c <<= 1i32;
        }
        res |= (c & 0x7f as i32 as u32) << count * 5i32;
        if count > 5i32 || res > 0x7fffffff as u32 || res < limits[count as usize] {
            return 0 as *const libc::c_char;
        }
        s = s.offset(count as isize);
    }
    if strict != 0 {
        if res > 0x10ffff as u32 || 0xd800 as u32 <= res && res <= 0xdfff as u32 {
            return 0 as *const libc::c_char;
        }
    }
    if !val.is_null() {
        *val = res;
    }
    return s.offset(1i32 as isize);
}
unsafe extern "C" fn utflen(mut L: *mut lua_State) -> i32 {
    let mut n: i64 = 0i32 as i64;
    let mut len: u64 = 0;
    let mut s: *const libc::c_char = luaL_checklstring(L, 1i32, &mut len);
    let mut posi: i64 = u_posrelat(luaL_optinteger(L, 2i32, 1i32 as i64), len);
    let mut posj: i64 = u_posrelat(luaL_optinteger(L, 3i32, -(1i32) as i64), len);
    let mut lax: i32 = lua_toboolean(L, 4i32);
    (((1i32 as i64 <= posi && {
        posi -= 1;
        posi <= len as i64
    }) as i32
        != 0i32) as i32 as i64
        != 0
        || luaL_argerror(
            L,
            2i32,
            b"initial position out of bounds\0" as *const u8 as *const libc::c_char,
        ) != 0) as i32;
    posj -= 1;
    (((posj < len as i64) as i32 != 0i32) as i32 as i64 != 0
        || luaL_argerror(
            L,
            3i32,
            b"final position out of bounds\0" as *const u8 as *const libc::c_char,
        ) != 0) as i32;
    while posi <= posj {
        let mut s1: *const libc::c_char =
            utf8_decode(s.offset(posi as isize), 0 as *mut utfint, (lax == 0) as i32);
        if s1.is_null() {
            lua_pushnil(L);
            lua_pushinteger(L, posi + 1i32 as i64);
            return 2i32;
        }
        posi = s1.offset_from(s) as i64 as i64;
        n += 1;
    }
    lua_pushinteger(L, n);
    return 1i32;
}
unsafe extern "C" fn codepoint(mut L: *mut lua_State) -> i32 {
    let mut len: u64 = 0;
    let mut s: *const libc::c_char = luaL_checklstring(L, 1i32, &mut len);
    let mut posi: i64 = u_posrelat(luaL_optinteger(L, 2i32, 1i32 as i64), len);
    let mut pose: i64 = u_posrelat(luaL_optinteger(L, 3i32, posi), len);
    let mut lax: i32 = lua_toboolean(L, 4i32);
    let mut n: i32 = 0;
    let mut se: *const libc::c_char = 0 as *const libc::c_char;
    (((posi >= 1i32 as i64) as i32 != 0i32) as i32 as i64 != 0
        || luaL_argerror(
            L,
            2i32,
            b"out of bounds\0" as *const u8 as *const libc::c_char,
        ) != 0) as i32;
    (((pose <= len as i64) as i32 != 0i32) as i32 as i64 != 0
        || luaL_argerror(
            L,
            3i32,
            b"out of bounds\0" as *const u8 as *const libc::c_char,
        ) != 0) as i32;
    if posi > pose {
        return 0i32;
    }
    if pose - posi >= 2147483647i32 as i64 {
        return luaL_error(
            L,
            b"string slice too long\0" as *const u8 as *const libc::c_char,
        );
    }
    n = (pose - posi) as i32 + 1i32;
    luaL_checkstack(
        L,
        n,
        b"string slice too long\0" as *const u8 as *const libc::c_char,
    );
    n = 0i32;
    se = s.offset(pose as isize);
    s = s.offset((posi - 1i32 as i64) as isize);
    while s < se {
        let mut code: utfint = 0;
        s = utf8_decode(s, &mut code, (lax == 0) as i32);
        if s.is_null() {
            return luaL_error(
                L,
                b"invalid UTF-8 code\0" as *const u8 as *const libc::c_char,
            );
        }
        lua_pushinteger(L, code as i64);
        n += 1;
    }
    return n;
}
unsafe extern "C" fn pushutfchar(mut L: *mut lua_State, mut arg: i32) {
    let mut code: u64 = luaL_checkinteger(L, arg) as u64;
    (((code <= 0x7fffffff as u32 as u64) as i32 != 0i32) as i32 as i64 != 0
        || luaL_argerror(
            L,
            arg,
            b"value out of range\0" as *const u8 as *const libc::c_char,
        ) != 0) as i32;
    lua_pushfstring(L, b"%U\0" as *const u8 as *const libc::c_char, code as i64);
}
unsafe extern "C" fn utfchar(mut L: *mut lua_State) -> i32 {
    let mut n: i32 = lua_gettop(L);
    if n == 1i32 {
        pushutfchar(L, 1i32);
    } else {
        let mut i: i32 = 0;
        let mut b: luaL_Buffer = luaL_Buffer {
            b: 0 as *mut libc::c_char,
            size: 0,
            n: 0,
            L: 0 as *mut lua_State,
            init: LUTF8LibC3RustUnnamed { n: 0. },
        };
        luaL_buffinit(L, &mut b);
        i = 1i32;
        while i <= n {
            pushutfchar(L, i);
            luaL_addvalue(&mut b);
            i += 1;
        }
        luaL_pushresult(&mut b);
    }
    return 1i32;
}
unsafe extern "C" fn byteoffset(mut L: *mut lua_State) -> i32 {
    let mut len: u64 = 0;
    let mut s: *const libc::c_char = luaL_checklstring(L, 1i32, &mut len);
    let mut n: i64 = luaL_checkinteger(L, 2i32);
    let mut posi: i64 = (if n >= 0i32 as i64 {
        1i32 as libc::c_ulong
    } else {
        len.wrapping_add(1i32 as libc::c_ulong)
    }) as i64;
    posi = u_posrelat(luaL_optinteger(L, 3i32, posi), len);
    (((1i32 as i64 <= posi && {
        posi -= 1;
        posi <= len as i64
    }) as i32
        != 0i32) as i32 as i64
        != 0
        || luaL_argerror(
            L,
            3i32,
            b"position out of bounds\0" as *const u8 as *const libc::c_char,
        ) != 0) as i32;
    if n == 0i32 as i64 {
        while posi > 0i32 as i64 && *s.offset(posi as isize) as i32 & 0xc0 as i32 == 0x80 as i32 {
            posi -= 1;
        }
    } else {
        if *s.offset(posi as isize) as i32 & 0xc0 as i32 == 0x80 as i32 {
            return luaL_error(
                L,
                b"initial position is a continuation byte\0" as *const u8 as *const libc::c_char,
            );
        }
        if n < 0i32 as i64 {
            while n < 0i32 as i64 && posi > 0i32 as i64 {
                loop {
                    posi -= 1;
                    if !(posi > 0i32 as i64
                        && *s.offset(posi as isize) as i32 & 0xc0 as i32 == 0x80 as i32)
                    {
                        break;
                    }
                }
                n += 1;
            }
        } else {
            n -= 1;

            while n > 0i32 as i64 && posi < len as i64 {
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
    if n == 0i32 as i64 {
        lua_pushinteger(L, posi + 1i32 as i64);
    } else {
        lua_pushnil(L);
    }
    return 1i32;
}
unsafe extern "C" fn iter_aux(mut L: *mut lua_State, mut strict: i32) -> i32 {
    let mut len: u64 = 0;
    let mut s: *const libc::c_char = luaL_checklstring(L, 1i32, &mut len);
    let mut n: u64 = lua_tointegerx(L, 2i32, 0 as *mut i32) as u64;
    if n < len as u64 {
        while *s.offset(n as isize) as i32 & 0xc0 as i32 == 0x80 as i32 {
            n = n.wrapping_add(1);
        }
    }
    if n >= len as u64 {
        return 0i32;
    } else {
        let mut code: utfint = 0;
        let mut next: *const libc::c_char = utf8_decode(s.offset(n as isize), &mut code, strict);
        if next.is_null() || *next as i32 & 0xc0 as i32 == 0x80 as i32 {
            return luaL_error(
                L,
                b"invalid UTF-8 code\0" as *const u8 as *const libc::c_char,
            );
        }
        lua_pushinteger(L, n.wrapping_add(1i32 as u64) as i64);
        lua_pushinteger(L, code as i64);
        return 2i32;
    };
}
unsafe extern "C" fn iter_auxstrict(mut L: *mut lua_State) -> i32 {
    return iter_aux(L, 1i32);
}
unsafe extern "C" fn iter_auxlax(mut L: *mut lua_State) -> i32 {
    return iter_aux(L, 0i32);
}
unsafe extern "C" fn iter_codes(mut L: *mut lua_State) -> i32 {
    let mut lax: i32 = lua_toboolean(L, 2i32);
    let mut s: *const libc::c_char = luaL_checklstring(L, 1i32, 0 as *mut u64);
    ((!(*s as i32 & 0xc0 as i32 == 0x80 as i32) as i32 != 0i32) as i32 as i64 != 0
        || luaL_argerror(
            L,
            1i32,
            b"invalid UTF-8 code\0" as *const u8 as *const libc::c_char,
        ) != 0) as i32;
    lua_pushcclosure(
        L,
        if lax != 0 {
            Some(iter_auxlax as unsafe extern "C" fn(*mut lua_State) -> i32)
        } else {
            Some(iter_auxstrict as unsafe extern "C" fn(*mut lua_State) -> i32)
        },
        0i32,
    );
    lua_pushvalue(L, 1i32);
    lua_pushinteger(L, 0i32 as i64);
    return 3i32;
}
static mut funcs: [luaL_Reg; 7] = {
    [
        {
            let mut init = luaL_Reg {
                name: b"offset\0" as *const u8 as *const libc::c_char,
                func: Some(byteoffset as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"codepoint\0" as *const u8 as *const libc::c_char,
                func: Some(codepoint as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"char\0" as *const u8 as *const libc::c_char,
                func: Some(utfchar as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"len\0" as *const u8 as *const libc::c_char,
                func: Some(utflen as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"codes\0" as *const u8 as *const libc::c_char,
                func: Some(iter_codes as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"charpattern\0" as *const u8 as *const libc::c_char,
                func: None,
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: 0 as *const libc::c_char,
                func: None,
            };
            init
        },
    ]
};
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaopen_utf8(mut L: *mut lua_State) -> i32 {
    luaL_checkversion_(
        L,
        504i32 as f64,
        (::core::mem::size_of::<i64>() as libc::c_ulong)
            .wrapping_mul(16i32 as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<f64>() as libc::c_ulong),
    );
    lua_createtable(
        L,
        0i32,
        (::core::mem::size_of::<[luaL_Reg; 7]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<luaL_Reg>() as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong) as i32,
    );
    luaL_setfuncs(L, funcs.as_ptr(), 0i32);
    lua_pushlstring(
        L,
        b"[\0-\x7F\xC2-\xFD][\x80-\xBF]*\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong),
    );
    lua_setfield(
        L,
        -(2i32),
        b"charpattern\0" as *const u8 as *const libc::c_char,
    );
    return 1i32;
}
