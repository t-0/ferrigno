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
    fn __errno_location() -> *mut i32;
    fn setlocale(
        __category: i32,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn exit(_: i32) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn mkstemp(__template: *mut libc::c_char) -> i32;
    fn system(__command: *const libc::c_char) -> i32;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> i32;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> i32;
    fn clock() -> clock_t;
    fn time(__timer: *mut time_t) -> time_t;
    fn difftime(__time1: time_t, __time0: time_t) -> f64;
    fn mktime(__tp: *mut tm) -> time_t;
    fn strftime(
        __s: *mut libc::c_char,
        __maxsize: size_t,
        __format: *const libc::c_char,
        __tp: *const tm,
    ) -> size_t;
    fn gmtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    fn lua_close(L: *mut lua_State);
    fn lua_settop(L: *mut lua_State, index: i32);
    fn lua_type(L: *mut lua_State, index: i32) -> i32;
    fn lua_tointegerx(
        L: *mut lua_State,
        index: i32,
        isnum: *mut i32,
    ) -> Integer;
    fn lua_toboolean(L: *mut lua_State, index: i32) -> i32;
    fn lua_pushnumber(L: *mut lua_State, n: Number);
    fn lua_pushinteger(L: *mut lua_State, n: Integer);
    fn lua_pushstring(L: *mut lua_State, s: *const libc::c_char) -> *const libc::c_char;
    fn lua_pushfstring(
        L: *mut lua_State,
        fmt: *const libc::c_char,
        _: ...
    ) -> *const libc::c_char;
    fn lua_pushboolean(L: *mut lua_State, b: i32);
    fn lua_getfield(
        L: *mut lua_State,
        index: i32,
        k: *const libc::c_char,
    ) -> i32;
    fn lua_createtable(L: *mut lua_State, narr: i32, nrec: i32);
    fn lua_setfield(L: *mut lua_State, index: i32, k: *const libc::c_char);
    fn remove(__filename: *const libc::c_char) -> i32;
    fn rename(__old: *const libc::c_char, __new: *const libc::c_char) -> i32;
    fn luaL_checkversion_(L: *mut lua_State, ver: Number, sz: size_t);
    fn luaL_argerror(
        L: *mut lua_State,
        arg: i32,
        extramsg: *const libc::c_char,
    ) -> i32;
    fn luaL_checklstring(
        L: *mut lua_State,
        arg: i32,
        l: *mut size_t,
    ) -> *const libc::c_char;
    fn luaL_optlstring(
        L: *mut lua_State,
        arg: i32,
        def: *const libc::c_char,
        l: *mut size_t,
    ) -> *const libc::c_char;
    fn luaL_checkinteger(L: *mut lua_State, arg: i32) -> Integer;
    fn luaL_optinteger(
        L: *mut lua_State,
        arg: i32,
        def: Integer,
    ) -> Integer;
    fn luaL_checktype(L: *mut lua_State, arg: i32, t: i32);
    fn luaL_error(L: *mut lua_State, fmt: *const libc::c_char, _: ...) -> i32;
    fn luaL_checkoption(
        L: *mut lua_State,
        arg: i32,
        def: *const libc::c_char,
        lst: *const *const libc::c_char,
    ) -> i32;
    fn luaL_fileresult(
        L: *mut lua_State,
        stat: i32,
        fname: *const libc::c_char,
    ) -> i32;
    fn luaL_execresult(L: *mut lua_State, stat: i32) -> i32;
    fn luaL_setfuncs(L: *mut lua_State, l: *const luaL_Reg, nup: i32);
    fn luaL_buffinit(L: *mut lua_State, B: *mut luaL_Buffer);
    fn luaL_prepbuffsize(B: *mut luaL_Buffer, sz: size_t) -> *mut libc::c_char;
    fn luaL_pushresult(B: *mut luaL_Buffer);
    fn close(__fd: i32) -> i32;
}
pub type size_t = libc::c_ulong;
pub type __clock_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type clock_t = __clock_t;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: i32,
    pub tm_min: i32,
    pub tm_hour: i32,
    pub tm_mday: i32,
    pub tm_mon: i32,
    pub tm_year: i32,
    pub tm_wday: i32,
    pub tm_yday: i32,
    pub tm_isdst: i32,
    pub __tm_gmtoff: libc::c_long,
    pub __tm_zone: *const libc::c_char,
}
pub type ptrdiff_t = libc::c_long;
pub type Number = f64;
pub type Integer = i64;
pub type CFunction = Option::<unsafe extern "C" fn(*mut lua_State) -> i32>;
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
    pub n: Number,
    pub u: f64,
    pub s: *mut libc::c_void,
    pub i: Integer,
    pub l: libc::c_long,
    pub b: [libc::c_char; 1024],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaL_Reg {
    pub name: *const libc::c_char,
    pub func: CFunction,
}
unsafe extern "C" fn os_execute(mut L: *mut lua_State) -> i32 {
    let mut cmd: *const libc::c_char = luaL_optlstring(
        L,
        1 as i32,
        0 as *const libc::c_char,
        0 as *mut size_t,
    );
    let mut stat: i32 = 0;
    *__errno_location() = 0 as i32;
    stat = system(cmd);
    if !cmd.is_null() {
        return luaL_execresult(L, stat)
    } else {
        lua_pushboolean(L, stat);
        return 1 as i32;
    };
}
unsafe extern "C" fn os_remove(mut L: *mut lua_State) -> i32 {
    let mut filename: *const libc::c_char = luaL_checklstring(
        L,
        1 as i32,
        0 as *mut size_t,
    );
    *__errno_location() = 0 as i32;
    return luaL_fileresult(
        L,
        (remove(filename) == 0 as i32) as i32,
        filename,
    );
}
unsafe extern "C" fn os_rename(mut L: *mut lua_State) -> i32 {
    let mut fromname: *const libc::c_char = luaL_checklstring(
        L,
        1 as i32,
        0 as *mut size_t,
    );
    let mut toname: *const libc::c_char = luaL_checklstring(
        L,
        2 as i32,
        0 as *mut size_t,
    );
    *__errno_location() = 0 as i32;
    return luaL_fileresult(
        L,
        (rename(fromname, toname) == 0 as i32) as i32,
        0 as *const libc::c_char,
    );
}
unsafe extern "C" fn os_tmpname(mut L: *mut lua_State) -> i32 {
    let mut buff: [libc::c_char; 32] = [0; 32];
    let mut err: i32 = 0;
    strcpy(buff.as_mut_ptr(), b"/tmp/lua_XXXXXX\0" as *const u8 as *const libc::c_char);
    err = mkstemp(buff.as_mut_ptr());
    if err != -(1 as i32) {
        close(err);
    }
    err = (err == -(1 as i32)) as i32;
    if (err != 0 as i32) as i32 as libc::c_long != 0 {
        return luaL_error(
            L,
            b"unable to generate a unique filename\0" as *const u8 as *const libc::c_char,
        );
    }
    lua_pushstring(L, buff.as_mut_ptr());
    return 1 as i32;
}
unsafe extern "C" fn os_getenv(mut L: *mut lua_State) -> i32 {
    lua_pushstring(L, getenv(luaL_checklstring(L, 1 as i32, 0 as *mut size_t)));
    return 1 as i32;
}
unsafe extern "C" fn os_clock(mut L: *mut lua_State) -> i32 {
    lua_pushnumber(
        L,
        clock() as Number / 1000000 as i32 as __clock_t as Number,
    );
    return 1 as i32;
}
unsafe extern "C" fn setfield(
    mut L: *mut lua_State,
    mut key: *const libc::c_char,
    mut value: i32,
    mut delta: i32,
) {
    lua_pushinteger(L, value as Integer + delta as i64);
    lua_setfield(L, -(2 as i32), key);
}
unsafe extern "C" fn setboolfield(
    mut L: *mut lua_State,
    mut key: *const libc::c_char,
    mut value: i32,
) {
    if value < 0 as i32 {
        return;
    }
    lua_pushboolean(L, value);
    lua_setfield(L, -(2 as i32), key);
}
unsafe extern "C" fn setallfields(mut L: *mut lua_State, mut stm: *mut tm) {
    setfield(
        L,
        b"year\0" as *const u8 as *const libc::c_char,
        (*stm).tm_year,
        1900 as i32,
    );
    setfield(
        L,
        b"month\0" as *const u8 as *const libc::c_char,
        (*stm).tm_mon,
        1 as i32,
    );
    setfield(
        L,
        b"day\0" as *const u8 as *const libc::c_char,
        (*stm).tm_mday,
        0 as i32,
    );
    setfield(
        L,
        b"hour\0" as *const u8 as *const libc::c_char,
        (*stm).tm_hour,
        0 as i32,
    );
    setfield(
        L,
        b"min\0" as *const u8 as *const libc::c_char,
        (*stm).tm_min,
        0 as i32,
    );
    setfield(
        L,
        b"sec\0" as *const u8 as *const libc::c_char,
        (*stm).tm_sec,
        0 as i32,
    );
    setfield(
        L,
        b"yday\0" as *const u8 as *const libc::c_char,
        (*stm).tm_yday,
        1 as i32,
    );
    setfield(
        L,
        b"wday\0" as *const u8 as *const libc::c_char,
        (*stm).tm_wday,
        1 as i32,
    );
    setboolfield(L, b"isdst\0" as *const u8 as *const libc::c_char, (*stm).tm_isdst);
}
unsafe extern "C" fn getboolfield(
    mut L: *mut lua_State,
    mut key: *const libc::c_char,
) -> i32 {
    let mut res: i32 = 0;
    res = if lua_getfield(L, -(1 as i32), key) == 0 as i32 {
        -(1 as i32)
    } else {
        lua_toboolean(L, -(1 as i32))
    };
    lua_settop(L, -(1 as i32) - 1 as i32);
    return res;
}
unsafe extern "C" fn getfield(
    mut L: *mut lua_State,
    mut key: *const libc::c_char,
    mut d: i32,
    mut delta: i32,
) -> i32 {
    let mut isnum: i32 = 0;
    let mut t: i32 = lua_getfield(L, -(1 as i32), key);
    let mut res: Integer = lua_tointegerx(L, -(1 as i32), &mut isnum);
    if isnum == 0 {
        if ((t != 0 as i32) as i32 != 0 as i32) as i32
            as libc::c_long != 0
        {
            return luaL_error(
                L,
                b"field '%s' is not an integer\0" as *const u8 as *const libc::c_char,
                key,
            )
        } else if ((d < 0 as i32) as i32 != 0 as i32)
            as i32 as libc::c_long != 0
        {
            return luaL_error(
                L,
                b"field '%s' missing in date table\0" as *const u8
                    as *const libc::c_char,
                key,
            )
        }
        res = d as Integer;
    } else {
        if if res >= 0 as i32 as i64 {
            (res - delta as i64
                <= 2147483647 as i32 as i64) as i32
        } else {
            ((-(2147483647 as i32) - 1 as i32 + delta)
                as i64 <= res) as i32
        } == 0
        {
            return luaL_error(
                L,
                b"field '%s' is out-of-bound\0" as *const u8 as *const libc::c_char,
                key,
            );
        }
        res -= delta as i64;
    }
    lua_settop(L, -(1 as i32) - 1 as i32);
    return res as i32;
}
unsafe extern "C" fn checkoption(
    mut L: *mut lua_State,
    mut conv: *const libc::c_char,
    mut convlen: ptrdiff_t,
    mut buff: *mut libc::c_char,
) -> *const libc::c_char {
    let mut option: *const libc::c_char = b"aAbBcCdDeFgGhHIjmMnprRStTuUVwWxXyYzZ%||EcECExEXEyEYOdOeOHOIOmOMOSOuOUOVOwOWOy\0"
        as *const u8 as *const libc::c_char;
    let mut oplen: i32 = 1 as i32;
    while *option as i32 != '\0' as i32 && oplen as libc::c_long <= convlen {
        if *option as i32 == '|' as i32 {
            oplen += 1;
        } else if memcmp(
            conv as *const libc::c_void,
            option as *const libc::c_void,
            oplen as libc::c_ulong,
        ) == 0 as i32
        {
            memcpy(
                buff as *mut libc::c_void,
                conv as *const libc::c_void,
                oplen as libc::c_ulong,
            );
            *buff.offset(oplen as isize) = '\0' as i32 as libc::c_char;
            return conv.offset(oplen as isize);
        }
        option = option.offset(oplen as isize);
    }
    luaL_argerror(
        L,
        1 as i32,
        lua_pushfstring(
            L,
            b"invalid conversion specifier '%%%s'\0" as *const u8 as *const libc::c_char,
            conv,
        ),
    );
    return conv;
}
unsafe extern "C" fn l_checktime(mut L: *mut lua_State, mut arg: i32) -> time_t {
    let mut t: Integer = luaL_checkinteger(L, arg);
    (((t as time_t as i64 == t) as i32 != 0 as i32)
        as i32 as libc::c_long != 0
        || luaL_argerror(
            L,
            arg,
            b"time out-of-bounds\0" as *const u8 as *const libc::c_char,
        ) != 0) as i32;
    return t as time_t;
}
unsafe extern "C" fn os_date(mut L: *mut lua_State) -> i32 {
    let mut slen: size_t = 0;
    let mut s: *const libc::c_char = luaL_optlstring(
        L,
        1 as i32,
        b"%c\0" as *const u8 as *const libc::c_char,
        &mut slen,
    );
    let mut t: time_t = if lua_type(L, 2 as i32) <= 0 as i32 {
        time(0 as *mut time_t)
    } else {
        l_checktime(L, 2 as i32)
    };
    let mut se: *const libc::c_char = s.offset(slen as isize);
    let mut tmr: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        __tm_gmtoff: 0,
        __tm_zone: 0 as *const libc::c_char,
    };
    let mut stm: *mut tm = 0 as *mut tm;
    if *s as i32 == '!' as i32 {
        stm = gmtime_r(&mut t, &mut tmr);
        s = s.offset(1);
    } else {
        stm = localtime_r(&mut t, &mut tmr);
    }
    if stm.is_null() {
        return luaL_error(
            L,
            b"date result cannot be represented in this installation\0" as *const u8
                as *const libc::c_char,
        );
    }
    if strcmp(s, b"*t\0" as *const u8 as *const libc::c_char) == 0 as i32 {
        lua_createtable(L, 0 as i32, 9 as i32);
        setallfields(L, stm);
    } else {
        let mut cc: [libc::c_char; 4] = [0; 4];
        let mut b: luaL_Buffer = luaL_Buffer {
            b: 0 as *mut libc::c_char,
            size: 0,
            n: 0,
            L: 0 as *mut lua_State,
            init: C2RustUnnamed { n: 0. },
        };
        cc[0 as i32 as usize] = '%' as i32 as libc::c_char;
        luaL_buffinit(L, &mut b);
        while s < se {
            if *s as i32 != '%' as i32 {
                (b.n < b.size
                    || !(luaL_prepbuffsize(&mut b, 1 as i32 as size_t))
                        .is_null()) as i32;
                let fresh0 = s;
                s = s.offset(1);
                let fresh1 = b.n;
                b.n = (b.n).wrapping_add(1);
                *(b.b).offset(fresh1 as isize) = *fresh0;
            } else {
                let mut reslen: size_t = 0;
                let mut buff: *mut libc::c_char = luaL_prepbuffsize(
                    &mut b,
                    250 as i32 as size_t,
                );
                s = s.offset(1);
                s = checkoption(
                    L,
                    s,
                    se.offset_from(s) as libc::c_long,
                    cc.as_mut_ptr().offset(1 as i32 as isize),
                );
                reslen = strftime(
                    buff,
                    250 as i32 as size_t,
                    cc.as_mut_ptr(),
                    stm,
                );
                b.n = (b.n as libc::c_ulong).wrapping_add(reslen) as size_t as size_t;
            }
        }
        luaL_pushresult(&mut b);
    }
    return 1 as i32;
}
unsafe extern "C" fn os_time(mut L: *mut lua_State) -> i32 {
    let mut t: time_t = 0;
    if lua_type(L, 1 as i32) <= 0 as i32 {
        t = time(0 as *mut time_t);
    } else {
        let mut ts: tm = tm {
            tm_sec: 0,
            tm_min: 0,
            tm_hour: 0,
            tm_mday: 0,
            tm_mon: 0,
            tm_year: 0,
            tm_wday: 0,
            tm_yday: 0,
            tm_isdst: 0,
            __tm_gmtoff: 0,
            __tm_zone: 0 as *const libc::c_char,
        };
        luaL_checktype(L, 1 as i32, 5 as i32);
        lua_settop(L, 1 as i32);
        ts
            .tm_year = getfield(
            L,
            b"year\0" as *const u8 as *const libc::c_char,
            -(1 as i32),
            1900 as i32,
        );
        ts
            .tm_mon = getfield(
            L,
            b"month\0" as *const u8 as *const libc::c_char,
            -(1 as i32),
            1 as i32,
        );
        ts
            .tm_mday = getfield(
            L,
            b"day\0" as *const u8 as *const libc::c_char,
            -(1 as i32),
            0 as i32,
        );
        ts
            .tm_hour = getfield(
            L,
            b"hour\0" as *const u8 as *const libc::c_char,
            12 as i32,
            0 as i32,
        );
        ts
            .tm_min = getfield(
            L,
            b"min\0" as *const u8 as *const libc::c_char,
            0 as i32,
            0 as i32,
        );
        ts
            .tm_sec = getfield(
            L,
            b"sec\0" as *const u8 as *const libc::c_char,
            0 as i32,
            0 as i32,
        );
        ts.tm_isdst = getboolfield(L, b"isdst\0" as *const u8 as *const libc::c_char);
        t = mktime(&mut ts);
        setallfields(L, &mut ts);
    }
    if t != t as Integer as time_t || t == -(1 as i32) as time_t {
        return luaL_error(
            L,
            b"time result cannot be represented in this installation\0" as *const u8
                as *const libc::c_char,
        );
    }
    lua_pushinteger(L, t as Integer);
    return 1 as i32;
}
unsafe extern "C" fn os_difftime(mut L: *mut lua_State) -> i32 {
    let mut t1: time_t = l_checktime(L, 1 as i32);
    let mut t2: time_t = l_checktime(L, 2 as i32);
    lua_pushnumber(L, difftime(t1, t2));
    return 1 as i32;
}
unsafe extern "C" fn os_setlocale(mut L: *mut lua_State) -> i32 {
    static mut cat: [i32; 6] = [
        6 as i32,
        3 as i32,
        0 as i32,
        4 as i32,
        1 as i32,
        2 as i32,
    ];
    static mut catnames: [*const libc::c_char; 7] = [
        b"all\0" as *const u8 as *const libc::c_char,
        b"collate\0" as *const u8 as *const libc::c_char,
        b"ctype\0" as *const u8 as *const libc::c_char,
        b"monetary\0" as *const u8 as *const libc::c_char,
        b"numeric\0" as *const u8 as *const libc::c_char,
        b"time\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut l: *const libc::c_char = luaL_optlstring(
        L,
        1 as i32,
        0 as *const libc::c_char,
        0 as *mut size_t,
    );
    let mut op: i32 = luaL_checkoption(
        L,
        2 as i32,
        b"all\0" as *const u8 as *const libc::c_char,
        catnames.as_ptr(),
    );
    lua_pushstring(L, setlocale(cat[op as usize], l));
    return 1 as i32;
}
unsafe extern "C" fn os_exit(mut L: *mut lua_State) -> i32 {
    let mut status: i32 = 0;
    if lua_type(L, 1 as i32) == 1 as i32 {
        status = if lua_toboolean(L, 1 as i32) != 0 {
            0 as i32
        } else {
            1 as i32
        };
    } else {
        status = luaL_optinteger(L, 1 as i32, 0 as i32 as Integer)
            as i32;
    }
    if lua_toboolean(L, 2 as i32) != 0 {
        lua_close(L);
    }
    if !L.is_null() {
        exit(status);
    }
    return 0 as i32;
}
static mut syslib: [luaL_Reg; 12] = unsafe {
    [
        {
            let mut init = luaL_Reg {
                name: b"clock\0" as *const u8 as *const libc::c_char,
                func: Some(
                    os_clock as unsafe extern "C" fn(*mut lua_State) -> i32,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"date\0" as *const u8 as *const libc::c_char,
                func: Some(
                    os_date as unsafe extern "C" fn(*mut lua_State) -> i32,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"difftime\0" as *const u8 as *const libc::c_char,
                func: Some(
                    os_difftime as unsafe extern "C" fn(*mut lua_State) -> i32,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"execute\0" as *const u8 as *const libc::c_char,
                func: Some(
                    os_execute as unsafe extern "C" fn(*mut lua_State) -> i32,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"exit\0" as *const u8 as *const libc::c_char,
                func: Some(
                    os_exit as unsafe extern "C" fn(*mut lua_State) -> i32,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"getenv\0" as *const u8 as *const libc::c_char,
                func: Some(
                    os_getenv as unsafe extern "C" fn(*mut lua_State) -> i32,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"remove\0" as *const u8 as *const libc::c_char,
                func: Some(
                    os_remove as unsafe extern "C" fn(*mut lua_State) -> i32,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"rename\0" as *const u8 as *const libc::c_char,
                func: Some(
                    os_rename as unsafe extern "C" fn(*mut lua_State) -> i32,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"setlocale\0" as *const u8 as *const libc::c_char,
                func: Some(
                    os_setlocale as unsafe extern "C" fn(*mut lua_State) -> i32,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"time\0" as *const u8 as *const libc::c_char,
                func: Some(
                    os_time as unsafe extern "C" fn(*mut lua_State) -> i32,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"tmpname\0" as *const u8 as *const libc::c_char,
                func: Some(
                    os_tmpname as unsafe extern "C" fn(*mut lua_State) -> i32,
                ),
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
pub unsafe extern "C" fn luaopen_os(mut L: *mut lua_State) -> i32 {
    luaL_checkversion_(
        L,
        504 as i32 as Number,
        (::core::mem::size_of::<Integer>() as libc::c_ulong)
            .wrapping_mul(16 as i32 as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<Number>() as libc::c_ulong),
    );
    lua_createtable(
        L,
        0 as i32,
        (::core::mem::size_of::<[luaL_Reg; 12]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<luaL_Reg>() as libc::c_ulong)
            .wrapping_sub(1 as i32 as libc::c_ulong) as i32,
    );
    luaL_setfuncs(L, syslib.as_ptr(), 0 as i32);
    return 1 as i32;
}
