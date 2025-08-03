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
    fn __errno_location() -> *mut libc::c_int;
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn mkstemp(__template: *mut libc::c_char) -> libc::c_int;
    fn system(__command: *const libc::c_char) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
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
    fn lua_settop(L: *mut lua_State, index: libc::c_int);
    fn lua_type(L: *mut lua_State, index: libc::c_int) -> libc::c_int;
    fn lua_tointegerx(
        L: *mut lua_State,
        index: libc::c_int,
        isnum: *mut libc::c_int,
    ) -> lua_Integer;
    fn lua_toboolean(L: *mut lua_State, index: libc::c_int) -> libc::c_int;
    fn lua_pushnumber(L: *mut lua_State, n: lua_Number);
    fn lua_pushinteger(L: *mut lua_State, n: lua_Integer);
    fn lua_pushstring(L: *mut lua_State, s: *const libc::c_char) -> *const libc::c_char;
    fn lua_pushfstring(
        L: *mut lua_State,
        fmt: *const libc::c_char,
        _: ...
    ) -> *const libc::c_char;
    fn lua_pushboolean(L: *mut lua_State, b: libc::c_int);
    fn lua_getfield(
        L: *mut lua_State,
        index: libc::c_int,
        k: *const libc::c_char,
    ) -> libc::c_int;
    fn lua_createtable(L: *mut lua_State, narr: libc::c_int, nrec: libc::c_int);
    fn lua_setfield(L: *mut lua_State, index: libc::c_int, k: *const libc::c_char);
    fn remove(__filename: *const libc::c_char) -> libc::c_int;
    fn rename(__old: *const libc::c_char, __new: *const libc::c_char) -> libc::c_int;
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
    fn luaL_optlstring(
        L: *mut lua_State,
        arg: libc::c_int,
        def: *const libc::c_char,
        l: *mut size_t,
    ) -> *const libc::c_char;
    fn luaL_checkinteger(L: *mut lua_State, arg: libc::c_int) -> lua_Integer;
    fn luaL_optinteger(
        L: *mut lua_State,
        arg: libc::c_int,
        def: lua_Integer,
    ) -> lua_Integer;
    fn luaL_checktype(L: *mut lua_State, arg: libc::c_int, t: libc::c_int);
    fn luaL_error(L: *mut lua_State, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn luaL_checkoption(
        L: *mut lua_State,
        arg: libc::c_int,
        def: *const libc::c_char,
        lst: *const *const libc::c_char,
    ) -> libc::c_int;
    fn luaL_fileresult(
        L: *mut lua_State,
        stat: libc::c_int,
        fname: *const libc::c_char,
    ) -> libc::c_int;
    fn luaL_execresult(L: *mut lua_State, stat: libc::c_int) -> libc::c_int;
    fn luaL_setfuncs(L: *mut lua_State, l: *const luaL_Reg, nup: libc::c_int);
    fn luaL_buffinit(L: *mut lua_State, B: *mut luaL_Buffer);
    fn luaL_prepbuffsize(B: *mut luaL_Buffer, sz: size_t) -> *mut libc::c_char;
    fn luaL_pushresult(B: *mut luaL_Buffer);
    fn close(__fd: libc::c_int) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __clock_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type clock_t = __clock_t;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub __tm_gmtoff: libc::c_long,
    pub __tm_zone: *const libc::c_char,
}
pub type ptrdiff_t = libc::c_long;
pub type lua_Number = f64;
pub type lua_Integer = i64;
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
    pub u: f64,
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
unsafe extern "C" fn os_execute(mut L: *mut lua_State) -> libc::c_int {
    let mut cmd: *const libc::c_char = luaL_optlstring(
        L,
        1 as libc::c_int,
        0 as *const libc::c_char,
        0 as *mut size_t,
    );
    let mut stat: libc::c_int = 0;
    *__errno_location() = 0 as libc::c_int;
    stat = system(cmd);
    if !cmd.is_null() {
        return luaL_execresult(L, stat)
    } else {
        lua_pushboolean(L, stat);
        return 1 as libc::c_int;
    };
}
unsafe extern "C" fn os_remove(mut L: *mut lua_State) -> libc::c_int {
    let mut filename: *const libc::c_char = luaL_checklstring(
        L,
        1 as libc::c_int,
        0 as *mut size_t,
    );
    *__errno_location() = 0 as libc::c_int;
    return luaL_fileresult(
        L,
        (remove(filename) == 0 as libc::c_int) as libc::c_int,
        filename,
    );
}
unsafe extern "C" fn os_rename(mut L: *mut lua_State) -> libc::c_int {
    let mut fromname: *const libc::c_char = luaL_checklstring(
        L,
        1 as libc::c_int,
        0 as *mut size_t,
    );
    let mut toname: *const libc::c_char = luaL_checklstring(
        L,
        2 as libc::c_int,
        0 as *mut size_t,
    );
    *__errno_location() = 0 as libc::c_int;
    return luaL_fileresult(
        L,
        (rename(fromname, toname) == 0 as libc::c_int) as libc::c_int,
        0 as *const libc::c_char,
    );
}
unsafe extern "C" fn os_tmpname(mut L: *mut lua_State) -> libc::c_int {
    let mut buff: [libc::c_char; 32] = [0; 32];
    let mut err: libc::c_int = 0;
    strcpy(buff.as_mut_ptr(), b"/tmp/lua_XXXXXX\0" as *const u8 as *const libc::c_char);
    err = mkstemp(buff.as_mut_ptr());
    if err != -(1 as libc::c_int) {
        close(err);
    }
    err = (err == -(1 as libc::c_int)) as libc::c_int;
    if (err != 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        return luaL_error(
            L,
            b"unable to generate a unique filename\0" as *const u8 as *const libc::c_char,
        );
    }
    lua_pushstring(L, buff.as_mut_ptr());
    return 1 as libc::c_int;
}
unsafe extern "C" fn os_getenv(mut L: *mut lua_State) -> libc::c_int {
    lua_pushstring(L, getenv(luaL_checklstring(L, 1 as libc::c_int, 0 as *mut size_t)));
    return 1 as libc::c_int;
}
unsafe extern "C" fn os_clock(mut L: *mut lua_State) -> libc::c_int {
    lua_pushnumber(
        L,
        clock() as lua_Number / 1000000 as libc::c_int as __clock_t as lua_Number,
    );
    return 1 as libc::c_int;
}
unsafe extern "C" fn setfield(
    mut L: *mut lua_State,
    mut key: *const libc::c_char,
    mut value: libc::c_int,
    mut delta: libc::c_int,
) {
    lua_pushinteger(L, value as lua_Integer + delta as i64);
    lua_setfield(L, -(2 as libc::c_int), key);
}
unsafe extern "C" fn setboolfield(
    mut L: *mut lua_State,
    mut key: *const libc::c_char,
    mut value: libc::c_int,
) {
    if value < 0 as libc::c_int {
        return;
    }
    lua_pushboolean(L, value);
    lua_setfield(L, -(2 as libc::c_int), key);
}
unsafe extern "C" fn setallfields(mut L: *mut lua_State, mut stm: *mut tm) {
    setfield(
        L,
        b"year\0" as *const u8 as *const libc::c_char,
        (*stm).tm_year,
        1900 as libc::c_int,
    );
    setfield(
        L,
        b"month\0" as *const u8 as *const libc::c_char,
        (*stm).tm_mon,
        1 as libc::c_int,
    );
    setfield(
        L,
        b"day\0" as *const u8 as *const libc::c_char,
        (*stm).tm_mday,
        0 as libc::c_int,
    );
    setfield(
        L,
        b"hour\0" as *const u8 as *const libc::c_char,
        (*stm).tm_hour,
        0 as libc::c_int,
    );
    setfield(
        L,
        b"min\0" as *const u8 as *const libc::c_char,
        (*stm).tm_min,
        0 as libc::c_int,
    );
    setfield(
        L,
        b"sec\0" as *const u8 as *const libc::c_char,
        (*stm).tm_sec,
        0 as libc::c_int,
    );
    setfield(
        L,
        b"yday\0" as *const u8 as *const libc::c_char,
        (*stm).tm_yday,
        1 as libc::c_int,
    );
    setfield(
        L,
        b"wday\0" as *const u8 as *const libc::c_char,
        (*stm).tm_wday,
        1 as libc::c_int,
    );
    setboolfield(L, b"isdst\0" as *const u8 as *const libc::c_char, (*stm).tm_isdst);
}
unsafe extern "C" fn getboolfield(
    mut L: *mut lua_State,
    mut key: *const libc::c_char,
) -> libc::c_int {
    let mut res: libc::c_int = 0;
    res = if lua_getfield(L, -(1 as libc::c_int), key) == 0 as libc::c_int {
        -(1 as libc::c_int)
    } else {
        lua_toboolean(L, -(1 as libc::c_int))
    };
    lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
    return res;
}
unsafe extern "C" fn getfield(
    mut L: *mut lua_State,
    mut key: *const libc::c_char,
    mut d: libc::c_int,
    mut delta: libc::c_int,
) -> libc::c_int {
    let mut isnum: libc::c_int = 0;
    let mut t: libc::c_int = lua_getfield(L, -(1 as libc::c_int), key);
    let mut res: lua_Integer = lua_tointegerx(L, -(1 as libc::c_int), &mut isnum);
    if isnum == 0 {
        if ((t != 0 as libc::c_int) as libc::c_int != 0 as libc::c_int) as libc::c_int
            as libc::c_long != 0
        {
            return luaL_error(
                L,
                b"field '%s' is not an integer\0" as *const u8 as *const libc::c_char,
                key,
            )
        } else if ((d < 0 as libc::c_int) as libc::c_int != 0 as libc::c_int)
            as libc::c_int as libc::c_long != 0
        {
            return luaL_error(
                L,
                b"field '%s' missing in date table\0" as *const u8
                    as *const libc::c_char,
                key,
            )
        }
        res = d as lua_Integer;
    } else {
        if if res >= 0 as libc::c_int as i64 {
            (res - delta as i64
                <= 2147483647 as libc::c_int as i64) as libc::c_int
        } else {
            ((-(2147483647 as libc::c_int) - 1 as libc::c_int + delta)
                as i64 <= res) as libc::c_int
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
    lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
    return res as libc::c_int;
}
unsafe extern "C" fn checkoption(
    mut L: *mut lua_State,
    mut conv: *const libc::c_char,
    mut convlen: ptrdiff_t,
    mut buff: *mut libc::c_char,
) -> *const libc::c_char {
    let mut option: *const libc::c_char = b"aAbBcCdDeFgGhHIjmMnprRStTuUVwWxXyYzZ%||EcECExEXEyEYOdOeOHOIOmOMOSOuOUOVOwOWOy\0"
        as *const u8 as *const libc::c_char;
    let mut oplen: libc::c_int = 1 as libc::c_int;
    while *option as libc::c_int != '\0' as i32 && oplen as libc::c_long <= convlen {
        if *option as libc::c_int == '|' as i32 {
            oplen += 1;
            oplen;
        } else if memcmp(
            conv as *const libc::c_void,
            option as *const libc::c_void,
            oplen as libc::c_ulong,
        ) == 0 as libc::c_int
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
        1 as libc::c_int,
        lua_pushfstring(
            L,
            b"invalid conversion specifier '%%%s'\0" as *const u8 as *const libc::c_char,
            conv,
        ),
    );
    return conv;
}
unsafe extern "C" fn l_checktime(mut L: *mut lua_State, mut arg: libc::c_int) -> time_t {
    let mut t: lua_Integer = luaL_checkinteger(L, arg);
    (((t as time_t as i64 == t) as libc::c_int != 0 as libc::c_int)
        as libc::c_int as libc::c_long != 0
        || luaL_argerror(
            L,
            arg,
            b"time out-of-bounds\0" as *const u8 as *const libc::c_char,
        ) != 0) as libc::c_int;
    return t as time_t;
}
unsafe extern "C" fn os_date(mut L: *mut lua_State) -> libc::c_int {
    let mut slen: size_t = 0;
    let mut s: *const libc::c_char = luaL_optlstring(
        L,
        1 as libc::c_int,
        b"%c\0" as *const u8 as *const libc::c_char,
        &mut slen,
    );
    let mut t: time_t = if lua_type(L, 2 as libc::c_int) <= 0 as libc::c_int {
        time(0 as *mut time_t)
    } else {
        l_checktime(L, 2 as libc::c_int)
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
    if *s as libc::c_int == '!' as i32 {
        stm = gmtime_r(&mut t, &mut tmr);
        s = s.offset(1);
        s;
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
    if strcmp(s, b"*t\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        lua_createtable(L, 0 as libc::c_int, 9 as libc::c_int);
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
        cc[0 as libc::c_int as usize] = '%' as i32 as libc::c_char;
        luaL_buffinit(L, &mut b);
        while s < se {
            if *s as libc::c_int != '%' as i32 {
                (b.n < b.size
                    || !(luaL_prepbuffsize(&mut b, 1 as libc::c_int as size_t))
                        .is_null()) as libc::c_int;
                let fresh0 = s;
                s = s.offset(1);
                let fresh1 = b.n;
                b.n = (b.n).wrapping_add(1);
                *(b.b).offset(fresh1 as isize) = *fresh0;
            } else {
                let mut reslen: size_t = 0;
                let mut buff: *mut libc::c_char = luaL_prepbuffsize(
                    &mut b,
                    250 as libc::c_int as size_t,
                );
                s = s.offset(1);
                s;
                s = checkoption(
                    L,
                    s,
                    se.offset_from(s) as libc::c_long,
                    cc.as_mut_ptr().offset(1 as libc::c_int as isize),
                );
                reslen = strftime(
                    buff,
                    250 as libc::c_int as size_t,
                    cc.as_mut_ptr(),
                    stm,
                );
                b.n = (b.n as libc::c_ulong).wrapping_add(reslen) as size_t as size_t;
            }
        }
        luaL_pushresult(&mut b);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn os_time(mut L: *mut lua_State) -> libc::c_int {
    let mut t: time_t = 0;
    if lua_type(L, 1 as libc::c_int) <= 0 as libc::c_int {
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
        luaL_checktype(L, 1 as libc::c_int, 5 as libc::c_int);
        lua_settop(L, 1 as libc::c_int);
        ts
            .tm_year = getfield(
            L,
            b"year\0" as *const u8 as *const libc::c_char,
            -(1 as libc::c_int),
            1900 as libc::c_int,
        );
        ts
            .tm_mon = getfield(
            L,
            b"month\0" as *const u8 as *const libc::c_char,
            -(1 as libc::c_int),
            1 as libc::c_int,
        );
        ts
            .tm_mday = getfield(
            L,
            b"day\0" as *const u8 as *const libc::c_char,
            -(1 as libc::c_int),
            0 as libc::c_int,
        );
        ts
            .tm_hour = getfield(
            L,
            b"hour\0" as *const u8 as *const libc::c_char,
            12 as libc::c_int,
            0 as libc::c_int,
        );
        ts
            .tm_min = getfield(
            L,
            b"min\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            0 as libc::c_int,
        );
        ts
            .tm_sec = getfield(
            L,
            b"sec\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            0 as libc::c_int,
        );
        ts.tm_isdst = getboolfield(L, b"isdst\0" as *const u8 as *const libc::c_char);
        t = mktime(&mut ts);
        setallfields(L, &mut ts);
    }
    if t != t as lua_Integer as time_t || t == -(1 as libc::c_int) as time_t {
        return luaL_error(
            L,
            b"time result cannot be represented in this installation\0" as *const u8
                as *const libc::c_char,
        );
    }
    lua_pushinteger(L, t as lua_Integer);
    return 1 as libc::c_int;
}
unsafe extern "C" fn os_difftime(mut L: *mut lua_State) -> libc::c_int {
    let mut t1: time_t = l_checktime(L, 1 as libc::c_int);
    let mut t2: time_t = l_checktime(L, 2 as libc::c_int);
    lua_pushnumber(L, difftime(t1, t2));
    return 1 as libc::c_int;
}
unsafe extern "C" fn os_setlocale(mut L: *mut lua_State) -> libc::c_int {
    static mut cat: [libc::c_int; 6] = [
        6 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
        4 as libc::c_int,
        1 as libc::c_int,
        2 as libc::c_int,
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
        1 as libc::c_int,
        0 as *const libc::c_char,
        0 as *mut size_t,
    );
    let mut op: libc::c_int = luaL_checkoption(
        L,
        2 as libc::c_int,
        b"all\0" as *const u8 as *const libc::c_char,
        catnames.as_ptr(),
    );
    lua_pushstring(L, setlocale(cat[op as usize], l));
    return 1 as libc::c_int;
}
unsafe extern "C" fn os_exit(mut L: *mut lua_State) -> libc::c_int {
    let mut status: libc::c_int = 0;
    if lua_type(L, 1 as libc::c_int) == 1 as libc::c_int {
        status = if lua_toboolean(L, 1 as libc::c_int) != 0 {
            0 as libc::c_int
        } else {
            1 as libc::c_int
        };
    } else {
        status = luaL_optinteger(L, 1 as libc::c_int, 0 as libc::c_int as lua_Integer)
            as libc::c_int;
    }
    if lua_toboolean(L, 2 as libc::c_int) != 0 {
        lua_close(L);
    }
    if !L.is_null() {
        exit(status);
    }
    return 0 as libc::c_int;
}
static mut syslib: [luaL_Reg; 12] = unsafe {
    [
        {
            let mut init = luaL_Reg {
                name: b"clock\0" as *const u8 as *const libc::c_char,
                func: Some(
                    os_clock as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"date\0" as *const u8 as *const libc::c_char,
                func: Some(
                    os_date as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"difftime\0" as *const u8 as *const libc::c_char,
                func: Some(
                    os_difftime as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"execute\0" as *const u8 as *const libc::c_char,
                func: Some(
                    os_execute as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"exit\0" as *const u8 as *const libc::c_char,
                func: Some(
                    os_exit as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"getenv\0" as *const u8 as *const libc::c_char,
                func: Some(
                    os_getenv as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"remove\0" as *const u8 as *const libc::c_char,
                func: Some(
                    os_remove as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"rename\0" as *const u8 as *const libc::c_char,
                func: Some(
                    os_rename as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"setlocale\0" as *const u8 as *const libc::c_char,
                func: Some(
                    os_setlocale as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"time\0" as *const u8 as *const libc::c_char,
                func: Some(
                    os_time as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"tmpname\0" as *const u8 as *const libc::c_char,
                func: Some(
                    os_tmpname as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
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
pub unsafe extern "C" fn luaopen_os(mut L: *mut lua_State) -> libc::c_int {
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
        (::core::mem::size_of::<[luaL_Reg; 12]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<luaL_Reg>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
    );
    luaL_setfuncs(L, syslib.as_ptr(), 0 as libc::c_int);
    return 1 as libc::c_int;
}
