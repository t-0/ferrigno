#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types)]
unsafe extern "C" {
    pub type lua_State;
    fn lua_gettop(L: *mut lua_State) -> libc::c_int;
    fn lua_pushvalue(L: *mut lua_State, idx: libc::c_int);
    fn lua_tointegerx(
        L: *mut lua_State,
        idx: libc::c_int,
        isnum: *mut libc::c_int,
    ) -> lua_Integer;
    fn lua_toboolean(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    fn lua_pushnil(L: *mut lua_State);
    fn lua_pushinteger(L: *mut lua_State, n: lua_Integer);
    fn lua_pushlstring(
        L: *mut lua_State,
        s: *const libc::c_char,
        len: size_t,
    ) -> *const libc::c_char;
    fn lua_pushfstring(
        L: *mut lua_State,
        fmt: *const libc::c_char,
        _: ...
    ) -> *const libc::c_char;
    fn lua_pushcclosure(L: *mut lua_State, fn_0: lua_CFunction, n: libc::c_int);
    fn lua_createtable(L: *mut lua_State, narr: libc::c_int, nrec: libc::c_int);
    fn lua_setfield(L: *mut lua_State, idx: libc::c_int, k: *const libc::c_char);
    fn luaL_checkversion_(L: *mut lua_State, ver: lua_Number, sz: size_t);
    fn luaL_argerror(
        L: *mut lua_State,
        arg: libc::c_int,
        extramsg: *const libc::c_char,
    ) -> libc::c_int;
    fn luaL_checklstring(
        L: *mut lua_State,
        arg: libc::c_int,
        l: *mut size_t,
    ) -> *const libc::c_char;
    fn luaL_checkinteger(L: *mut lua_State, arg: libc::c_int) -> lua_Integer;
    fn luaL_optinteger(
        L: *mut lua_State,
        arg: libc::c_int,
        def: lua_Integer,
    ) -> lua_Integer;
    fn luaL_checkstack(L: *mut lua_State, sz: libc::c_int, msg: *const libc::c_char);
    fn luaL_error(L: *mut lua_State, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn luaL_setfuncs(L: *mut lua_State, l: *const luaL_Reg, nup: libc::c_int);
    fn luaL_buffinit(L: *mut lua_State, B: *mut luaL_Buffer);
    fn luaL_addvalue(B: *mut luaL_Buffer);
    fn luaL_pushresult(B: *mut luaL_Buffer);
}
pub type size_t = libc::c_ulong;
pub type lua_Number = libc::c_double;
pub type lua_Integer = libc::c_longlong;
pub type lua_Unsigned = libc::c_ulonglong;
pub type lua_CFunction = Option::<unsafe extern "C" fn(*mut lua_State) -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaL_Buffer {
    pub b: *mut libc::c_char,
    pub size: size_t,
    pub n: size_t,
    pub L: *mut lua_State,
    pub init: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub n: lua_Number,
    pub u: libc::c_double,
    pub s: *mut libc::c_void,
    pub i: lua_Integer,
    pub l: libc::c_long,
    pub b: [libc::c_char; 1024],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaL_Reg {
    pub name: *const libc::c_char,
    pub func: lua_CFunction,
}
pub type utfint = libc::c_uint;
unsafe extern "C" fn u_posrelat(mut pos: lua_Integer, mut len: size_t) -> lua_Integer {
    if pos >= 0 as libc::c_int as libc::c_longlong {
        return pos
    } else if (0 as libc::c_uint as libc::c_ulong).wrapping_sub(pos as size_t) > len {
        return 0 as libc::c_int as lua_Integer
    } else {
        return len as lua_Integer + pos + 1 as libc::c_int as libc::c_longlong
    };
}
unsafe extern "C" fn utf8_decode(
    mut s: *const libc::c_char,
    mut val: *mut utfint,
    mut strict: libc::c_int,
) -> *const libc::c_char {
    static mut limits: [utfint; 6] = [
        !(0 as libc::c_int as utfint),
        0x80 as libc::c_int as utfint,
        0x800 as libc::c_int as utfint,
        0x10000 as libc::c_uint,
        0x200000 as libc::c_uint,
        0x4000000 as libc::c_uint,
    ];
    let mut c: libc::c_uint = *s.offset(0 as libc::c_int as isize) as libc::c_uchar
        as libc::c_uint;
    let mut res: utfint = 0 as libc::c_int as utfint;
    if c < 0x80 as libc::c_int as libc::c_uint {
        res = c;
    } else {
        let mut count: libc::c_int = 0 as libc::c_int;
        while c & 0x40 as libc::c_int as libc::c_uint != 0 {
            count += 1;
            let mut cc: libc::c_uint = *s.offset(count as isize) as libc::c_uchar
                as libc::c_uint;
            if !(cc & 0xc0 as libc::c_int as libc::c_uint
                == 0x80 as libc::c_int as libc::c_uint)
            {
                return 0 as *const libc::c_char;
            }
            res = res << 6 as libc::c_int | cc & 0x3f as libc::c_int as libc::c_uint;
            c <<= 1 as libc::c_int;
        }
        res |= (c & 0x7f as libc::c_int as libc::c_uint) << count * 5 as libc::c_int;
        if count > 5 as libc::c_int || res > 0x7fffffff as libc::c_uint
            || res < limits[count as usize]
        {
            return 0 as *const libc::c_char;
        }
        s = s.offset(count as isize);
    }
    if strict != 0 {
        if res > 0x10ffff as libc::c_uint
            || 0xd800 as libc::c_uint <= res && res <= 0xdfff as libc::c_uint
        {
            return 0 as *const libc::c_char;
        }
    }
    if !val.is_null() {
        *val = res;
    }
    return s.offset(1 as libc::c_int as isize);
}
unsafe extern "C" fn utflen(mut L: *mut lua_State) -> libc::c_int {
    let mut n: lua_Integer = 0 as libc::c_int as lua_Integer;
    let mut len: size_t = 0;
    let mut s: *const libc::c_char = luaL_checklstring(L, 1 as libc::c_int, &mut len);
    let mut posi: lua_Integer = u_posrelat(
        luaL_optinteger(L, 2 as libc::c_int, 1 as libc::c_int as lua_Integer),
        len,
    );
    let mut posj: lua_Integer = u_posrelat(
        luaL_optinteger(L, 3 as libc::c_int, -(1 as libc::c_int) as lua_Integer),
        len,
    );
    let mut lax: libc::c_int = lua_toboolean(L, 4 as libc::c_int);
    (((1 as libc::c_int as libc::c_longlong <= posi
        && {
            posi -= 1;
            posi <= len as lua_Integer
        }) as libc::c_int != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
        || luaL_argerror(
            L,
            2 as libc::c_int,
            b"initial position out of bounds\0" as *const u8 as *const libc::c_char,
        ) != 0) as libc::c_int;
    posj -= 1;
    (((posj < len as lua_Integer) as libc::c_int != 0 as libc::c_int) as libc::c_int
        as libc::c_long != 0
        || luaL_argerror(
            L,
            3 as libc::c_int,
            b"final position out of bounds\0" as *const u8 as *const libc::c_char,
        ) != 0) as libc::c_int;
    while posi <= posj {
        let mut s1: *const libc::c_char = utf8_decode(
            s.offset(posi as isize),
            0 as *mut utfint,
            (lax == 0) as libc::c_int,
        );
        if s1.is_null() {
            lua_pushnil(L);
            lua_pushinteger(L, posi + 1 as libc::c_int as libc::c_longlong);
            return 2 as libc::c_int;
        }
        posi = s1.offset_from(s) as libc::c_long as lua_Integer;
        n += 1;
        n;
    }
    lua_pushinteger(L, n);
    return 1 as libc::c_int;
}
unsafe extern "C" fn codepoint(mut L: *mut lua_State) -> libc::c_int {
    let mut len: size_t = 0;
    let mut s: *const libc::c_char = luaL_checklstring(L, 1 as libc::c_int, &mut len);
    let mut posi: lua_Integer = u_posrelat(
        luaL_optinteger(L, 2 as libc::c_int, 1 as libc::c_int as lua_Integer),
        len,
    );
    let mut pose: lua_Integer = u_posrelat(
        luaL_optinteger(L, 3 as libc::c_int, posi),
        len,
    );
    let mut lax: libc::c_int = lua_toboolean(L, 4 as libc::c_int);
    let mut n: libc::c_int = 0;
    let mut se: *const libc::c_char = 0 as *const libc::c_char;
    (((posi >= 1 as libc::c_int as libc::c_longlong) as libc::c_int != 0 as libc::c_int)
        as libc::c_int as libc::c_long != 0
        || luaL_argerror(
            L,
            2 as libc::c_int,
            b"out of bounds\0" as *const u8 as *const libc::c_char,
        ) != 0) as libc::c_int;
    (((pose <= len as lua_Integer) as libc::c_int != 0 as libc::c_int) as libc::c_int
        as libc::c_long != 0
        || luaL_argerror(
            L,
            3 as libc::c_int,
            b"out of bounds\0" as *const u8 as *const libc::c_char,
        ) != 0) as libc::c_int;
    if posi > pose {
        return 0 as libc::c_int;
    }
    if pose - posi >= 2147483647 as libc::c_int as libc::c_longlong {
        return luaL_error(
            L,
            b"string slice too long\0" as *const u8 as *const libc::c_char,
        );
    }
    n = (pose - posi) as libc::c_int + 1 as libc::c_int;
    luaL_checkstack(
        L,
        n,
        b"string slice too long\0" as *const u8 as *const libc::c_char,
    );
    n = 0 as libc::c_int;
    se = s.offset(pose as isize);
    s = s.offset((posi - 1 as libc::c_int as libc::c_longlong) as isize);
    while s < se {
        let mut code: utfint = 0;
        s = utf8_decode(s, &mut code, (lax == 0) as libc::c_int);
        if s.is_null() {
            return luaL_error(
                L,
                b"invalid UTF-8 code\0" as *const u8 as *const libc::c_char,
            );
        }
        lua_pushinteger(L, code as lua_Integer);
        n += 1;
        n;
    }
    return n;
}
unsafe extern "C" fn pushutfchar(mut L: *mut lua_State, mut arg: libc::c_int) {
    let mut code: lua_Unsigned = luaL_checkinteger(L, arg) as lua_Unsigned;
    (((code <= 0x7fffffff as libc::c_uint as libc::c_ulonglong) as libc::c_int
        != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
        || luaL_argerror(
            L,
            arg,
            b"value out of range\0" as *const u8 as *const libc::c_char,
        ) != 0) as libc::c_int;
    lua_pushfstring(
        L,
        b"%U\0" as *const u8 as *const libc::c_char,
        code as libc::c_long,
    );
}
unsafe extern "C" fn utfchar(mut L: *mut lua_State) -> libc::c_int {
    let mut n: libc::c_int = lua_gettop(L);
    if n == 1 as libc::c_int {
        pushutfchar(L, 1 as libc::c_int);
    } else {
        let mut i: libc::c_int = 0;
        let mut b: luaL_Buffer = luaL_Buffer {
            b: 0 as *mut libc::c_char,
            size: 0,
            n: 0,
            L: 0 as *mut lua_State,
            init: C2RustUnnamed { n: 0. },
        };
        luaL_buffinit(L, &mut b);
        i = 1 as libc::c_int;
        while i <= n {
            pushutfchar(L, i);
            luaL_addvalue(&mut b);
            i += 1;
            i;
        }
        luaL_pushresult(&mut b);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn byteoffset(mut L: *mut lua_State) -> libc::c_int {
    let mut len: size_t = 0;
    let mut s: *const libc::c_char = luaL_checklstring(L, 1 as libc::c_int, &mut len);
    let mut n: lua_Integer = luaL_checkinteger(L, 2 as libc::c_int);
    let mut posi: lua_Integer = (if n >= 0 as libc::c_int as libc::c_longlong {
        1 as libc::c_int as libc::c_ulong
    } else {
        len.wrapping_add(1 as libc::c_int as libc::c_ulong)
    }) as lua_Integer;
    posi = u_posrelat(luaL_optinteger(L, 3 as libc::c_int, posi), len);
    (((1 as libc::c_int as libc::c_longlong <= posi
        && {
            posi -= 1;
            posi <= len as lua_Integer
        }) as libc::c_int != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
        || luaL_argerror(
            L,
            3 as libc::c_int,
            b"position out of bounds\0" as *const u8 as *const libc::c_char,
        ) != 0) as libc::c_int;
    if n == 0 as libc::c_int as libc::c_longlong {
        while posi > 0 as libc::c_int as libc::c_longlong
            && *s.offset(posi as isize) as libc::c_int & 0xc0 as libc::c_int
                == 0x80 as libc::c_int
        {
            posi -= 1;
            posi;
        }
    } else {
        if *s.offset(posi as isize) as libc::c_int & 0xc0 as libc::c_int
            == 0x80 as libc::c_int
        {
            return luaL_error(
                L,
                b"initial position is a continuation byte\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if n < 0 as libc::c_int as libc::c_longlong {
            while n < 0 as libc::c_int as libc::c_longlong
                && posi > 0 as libc::c_int as libc::c_longlong
            {
                loop {
                    posi -= 1;
                    posi;
                    if !(posi > 0 as libc::c_int as libc::c_longlong
                        && *s.offset(posi as isize) as libc::c_int & 0xc0 as libc::c_int
                            == 0x80 as libc::c_int)
                    {
                        break;
                    }
                }
                n += 1;
                n;
            }
        } else {
            n -= 1;
            n;
            while n > 0 as libc::c_int as libc::c_longlong && posi < len as lua_Integer {
                loop {
                    posi += 1;
                    posi;
                    if !(*s.offset(posi as isize) as libc::c_int & 0xc0 as libc::c_int
                        == 0x80 as libc::c_int)
                    {
                        break;
                    }
                }
                n -= 1;
                n;
            }
        }
    }
    if n == 0 as libc::c_int as libc::c_longlong {
        lua_pushinteger(L, posi + 1 as libc::c_int as libc::c_longlong);
    } else {
        lua_pushnil(L);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn iter_aux(
    mut L: *mut lua_State,
    mut strict: libc::c_int,
) -> libc::c_int {
    let mut len: size_t = 0;
    let mut s: *const libc::c_char = luaL_checklstring(L, 1 as libc::c_int, &mut len);
    let mut n: lua_Unsigned = lua_tointegerx(L, 2 as libc::c_int, 0 as *mut libc::c_int)
        as lua_Unsigned;
    if n < len as libc::c_ulonglong {
        while *s.offset(n as isize) as libc::c_int & 0xc0 as libc::c_int
            == 0x80 as libc::c_int
        {
            n = n.wrapping_add(1);
            n;
        }
    }
    if n >= len as libc::c_ulonglong {
        return 0 as libc::c_int
    } else {
        let mut code: utfint = 0;
        let mut next: *const libc::c_char = utf8_decode(
            s.offset(n as isize),
            &mut code,
            strict,
        );
        if next.is_null()
            || *next as libc::c_int & 0xc0 as libc::c_int == 0x80 as libc::c_int
        {
            return luaL_error(
                L,
                b"invalid UTF-8 code\0" as *const u8 as *const libc::c_char,
            );
        }
        lua_pushinteger(
            L,
            n.wrapping_add(1 as libc::c_int as libc::c_ulonglong) as lua_Integer,
        );
        lua_pushinteger(L, code as lua_Integer);
        return 2 as libc::c_int;
    };
}
unsafe extern "C" fn iter_auxstrict(mut L: *mut lua_State) -> libc::c_int {
    return iter_aux(L, 1 as libc::c_int);
}
unsafe extern "C" fn iter_auxlax(mut L: *mut lua_State) -> libc::c_int {
    return iter_aux(L, 0 as libc::c_int);
}
unsafe extern "C" fn iter_codes(mut L: *mut lua_State) -> libc::c_int {
    let mut lax: libc::c_int = lua_toboolean(L, 2 as libc::c_int);
    let mut s: *const libc::c_char = luaL_checklstring(
        L,
        1 as libc::c_int,
        0 as *mut size_t,
    );
    ((!(*s as libc::c_int & 0xc0 as libc::c_int == 0x80 as libc::c_int) as libc::c_int
        != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
        || luaL_argerror(
            L,
            1 as libc::c_int,
            b"invalid UTF-8 code\0" as *const u8 as *const libc::c_char,
        ) != 0) as libc::c_int;
    lua_pushcclosure(
        L,
        if lax != 0 {
            Some(iter_auxlax as unsafe extern "C" fn(*mut lua_State) -> libc::c_int)
        } else {
            Some(iter_auxstrict as unsafe extern "C" fn(*mut lua_State) -> libc::c_int)
        },
        0 as libc::c_int,
    );
    lua_pushvalue(L, 1 as libc::c_int);
    lua_pushinteger(L, 0 as libc::c_int as lua_Integer);
    return 3 as libc::c_int;
}
static mut funcs: [luaL_Reg; 7] = unsafe {
    [
        {
            let mut init = luaL_Reg {
                name: b"offset\0" as *const u8 as *const libc::c_char,
                func: Some(
                    byteoffset as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"codepoint\0" as *const u8 as *const libc::c_char,
                func: Some(
                    codepoint as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"char\0" as *const u8 as *const libc::c_char,
                func: Some(
                    utfchar as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"len\0" as *const u8 as *const libc::c_char,
                func: Some(utflen as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"codes\0" as *const u8 as *const libc::c_char,
                func: Some(
                    iter_codes as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
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
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaopen_utf8(mut L: *mut lua_State) -> libc::c_int {
    luaL_checkversion_(
        L,
        504 as libc::c_int as lua_Number,
        (::core::mem::size_of::<lua_Integer>() as libc::c_ulong)
            .wrapping_mul(16 as libc::c_int as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<lua_Number>() as libc::c_ulong),
    );
    lua_createtable(
        L,
        0 as libc::c_int,
        (::core::mem::size_of::<[luaL_Reg; 7]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<luaL_Reg>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
    );
    luaL_setfuncs(L, funcs.as_ptr(), 0 as libc::c_int);
    lua_pushlstring(
        L,
        b"[\0-\x7F\xC2-\xFD][\x80-\xBF]*\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    lua_setfield(
        L,
        -(2 as libc::c_int),
        b"charpattern\0" as *const u8 as *const libc::c_char,
    );
    return 1 as libc::c_int;
}
