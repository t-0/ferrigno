use crate::character::CHARACTER_EXCLAMATION;
use crate::state::*;
use crate::registeredfunction::*;
use crate::utility::c::*;
use crate::character::*;
use crate::tag::*;
use crate::new::*;
use crate::buffer::*;
use libc::{system,remove,rename,setlocale};
pub unsafe extern "C" fn os_execute(state: *mut State) -> i32 {
    unsafe {
        let cmd: *const i8 = lual_optlstring(state, 1, std::ptr::null(), std::ptr::null_mut());
        let stat: i32;
        *__errno_location() = 0;
        stat = system(cmd);
        if !cmd.is_null() {
            return lual_execresult(state, stat);
        } else {
            (*state).push_boolean(0 != stat);
            return 1;
        };
    }
}
pub unsafe extern "C" fn os_remove(state: *mut State) -> i32 {
    unsafe {
        let filename: *const i8 = lual_checklstring(state, 1, std::ptr::null_mut());
        *__errno_location() = 0;
        return lual_fileresult(state, (remove(filename) == 0) as i32, filename);
    }
}
pub unsafe extern "C" fn os_rename(state: *mut State) -> i32 {
    unsafe {
        let fromname: *const i8 = lual_checklstring(state, 1, std::ptr::null_mut());
        let toname: *const i8 = lual_checklstring(state, 2, std::ptr::null_mut());
        *__errno_location() = 0;
        return lual_fileresult(
            state,
            (rename(fromname, toname) == 0) as i32,
            std::ptr::null(),
        );
    }
}
pub unsafe extern "C" fn os_tmpname(state: *mut State) -> i32 {
    unsafe {
        let mut buffer: [i8; 32] = [0; 32];
        let mut err: i32;
        strcpy(
            buffer.as_mut_ptr(),
            b"/tmp/lua_XXXXXX\0" as *const u8 as *const i8,
        );
        err = mkstemp(buffer.as_mut_ptr());
        if err != -1 {
            close(err);
        }
        err = (err == -1) as i32;
        if (err != 0) as i64 != 0 {
            return lual_error(
                state,
                b"unable to generate a unique filename\0" as *const u8 as *const i8,
            );
        }
        lua_pushstring(state, buffer.as_mut_ptr());
        return 1;
    }
}
pub unsafe extern "C" fn os_getenv(state: *mut State) -> i32 {
    unsafe {
        lua_pushstring(
            state,
            getenv(lual_checklstring(state, 1, std::ptr::null_mut())),
        );
        return 1;
    }
}
pub unsafe extern "C" fn os_clock(state: *mut State) -> i32 {
    unsafe {
        (*state).push_number(clock() as f64 / 1000000 as f64);
        return 1;
    }
}
pub unsafe extern "C" fn setfield(state: *mut State, key: *const i8, value: i32, delta: i32) {
    unsafe {
        (*state).push_integer(value as i64 + delta as i64);
        lua_setfield(state, -2, key);
    }
}
pub unsafe extern "C" fn setboolfield(state: *mut State, key: *const i8, value: bool) {
    unsafe {
        (*state).push_boolean(value);
        lua_setfield(state, -2, key);
    }
}
pub unsafe extern "C" fn setallfields(state: *mut State, stm: *mut TM) {
    unsafe {
        setfield(
            state,
            b"year\0" as *const u8 as *const i8,
            (*stm).tm_year,
            1900 as i32,
        );
        setfield(
            state,
            b"month\0" as *const u8 as *const i8,
            (*stm).tm_mon,
            1,
        );
        setfield(state, b"day\0" as *const u8 as *const i8, (*stm).tm_mday, 0);
        setfield(
            state,
            b"hour\0" as *const u8 as *const i8,
            (*stm).tm_hour,
            0,
        );
        setfield(state, b"min\0" as *const u8 as *const i8, (*stm).tm_min, 0);
        setfield(state, b"sec\0" as *const u8 as *const i8, (*stm).tm_sec, 0);
        setfield(
            state,
            b"yday\0" as *const u8 as *const i8,
            (*stm).tm_yday,
            1,
        );
        setfield(
            state,
            b"wday\0" as *const u8 as *const i8,
            (*stm).tm_wday,
            1,
        );
        setboolfield(
            state,
            b"isdst\0" as *const u8 as *const i8,
            0 != (*stm).tm_isdst,
        );
    }
}
pub unsafe extern "C" fn getboolfield(state: *mut State, key: *const i8) -> i32 {
    unsafe {
        let res: i32;
        res = if lua_getfield(state, -1, key) == 0 {
            -1
        } else {
            lua_toboolean(state, -1)
        };
        lua_settop(state, -2);
        return res;
    }
}
pub unsafe extern "C" fn getfield(state: *mut State, key: *const i8, d: i32, delta: i32) -> i32 {
    unsafe {
        let mut is_number: bool = false;
        let t: i32 = lua_getfield(state, -1, key);
        let mut res: i64 = lua_tointegerx(state, -1, &mut is_number);
        if !is_number {
            if ((t != 0) as i32 != 0) as i64 != 0 {
                return lual_error(
                    state,
                    b"field '%s' is not an integer\0" as *const u8 as *const i8,
                    key,
                );
            } else if ((d < 0) as i32 != 0) as i64 != 0 {
                return lual_error(
                    state,
                    b"field '%s' missing in date table\0" as *const u8 as *const i8,
                    key,
                );
            }
            res = d as i64;
        } else {
            if if res >= 0 {
                (res - delta as i64 <= 0x7FFFFFFF as i64) as i32
            } else {
                ((-(0x7FFFFFFF as i32) - 1 + delta) as i64 <= res) as i32
            } == 0
            {
                return lual_error(
                    state,
                    b"field '%s' is out-of-bound\0" as *const u8 as *const i8,
                    key,
                );
            }
            res -= delta as i64;
        }
        lua_settop(state, -2);
        return res as i32;
    }
}
pub unsafe extern "C" fn checkoption(
    state: *mut State,
    conv: *const i8,
    convlen: i64,
    buffer: *mut i8,
) -> *const i8 {
    unsafe {
        let mut option: *const i8 =
            b"aAbBcCdDeFgGhHIjmMnprRStTuUVwWxXyYzZ%||EcECExEXEyEYOdOeOHOIOmOMOSOuOUOVOwOWOy\0"
                as *const u8 as *const i8;
        let mut oplen: i32 = 1;
        while *option as i32 != CHARACTER_NUL as i32 && oplen as i64 <= convlen {
            if *option as i32 == '|' as i32 {
                oplen += 1;
            } else if memcmp(
                conv as *const libc::c_void,
                option as *const libc::c_void,
                oplen as u64,
            ) == 0
            {
                memcpy(
                    buffer as *mut libc::c_void,
                    conv as *const libc::c_void,
                    oplen as u64,
                );
                *buffer.offset(oplen as isize) = CHARACTER_NUL as i8;
                return conv.offset(oplen as isize);
            }
            option = option.offset(oplen as isize);
        }
        lual_argerror(
            state,
            1,
            lua_pushfstring(
                state,
                b"invalid conversion specifier '%%%s'\0" as *const u8 as *const i8,
                conv,
            ),
        );
        return conv;
    }
}
pub unsafe extern "C" fn l_checktime(state: *mut State, arg: i32) -> i64 {
    unsafe {
        let t: i64 = lual_checkinteger(state, arg);
        (((t as i64 == t) as i32 != 0) as i64 != 0
            || lual_argerror(
                state,
                arg,
                b"time out-of-bounds\0" as *const u8 as *const i8,
            ) != 0) as i32;
        return t as i64;
    }
}
pub unsafe extern "C" fn os_date(state: *mut State) -> i32 {
    unsafe {
        let mut slen: u64 = 0;
        let mut s: *const i8 =
            lual_optlstring(state, 1, b"%c\0" as *const u8 as *const i8, &mut slen);
        let mut t: i64 = if is_none_or_nil(lua_type(state, 2)) {
            time(std::ptr::null_mut())
        } else {
            l_checktime(state, 2)
        };
        let se: *const i8 = s.offset(slen as isize);
        let mut tmr: TM = TM {
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
            __tm_zone: std::ptr::null(),
        };
        let stm: *mut TM;
        if *s as i32 == CHARACTER_EXCLAMATION as i32 {
            stm = gmtime_r(&mut t, &mut tmr);
            s = s.offset(1);
        } else {
            stm = localtime_r(&mut t, &mut tmr);
        }
        if stm.is_null() {
            return lual_error(
                state,
                b"date result cannot be represented in this installation\0" as *const u8
                    as *const i8,
            );
        }
        if strcmp(s, b"*t\0" as *const u8 as *const i8) == 0 {
            (*state).lua_createtable();
            setallfields(state, stm);
        } else {
            let mut cc: [i8; 4] = [0; 4];
            let mut b = Buffer::new();
            cc[0] = CHARACTER_PERCENT as i8;
            b.initialize(state);
            while s < se {
                if *s as i32 != CHARACTER_PERCENT as i32 {
                    (b.length < b.size || !(b.prepare_with_size(1)).is_null()) as i32;
                    let fresh157 = s;
                    s = s.offset(1);
                    let fresh158 = b.length;
                    b.length = (b.length).wrapping_add(1);
                    *(b.pointer).offset(fresh158 as isize) = *fresh157;
                } else {
                    let reslen: u64;
                    let buffer: *mut i8 = b.prepare_with_size(250);
                    s = s.offset(1);
                    s = checkoption(
                        state,
                        s,
                        se.offset_from(s) as i64,
                        cc.as_mut_ptr().offset(1 as isize),
                    );
                    reslen = strftime(buffer, 250 as u64, cc.as_mut_ptr(), stm);
                    b.length = b.length.wrapping_add(reslen as usize);
                }
            }
            b.push_result();
        }
        return 1;
    }
}
pub unsafe extern "C" fn os_time(state: *mut State) -> i32 {
    unsafe {
        let t: i64;
        match lua_type(state, 1) {
            None | Some(TAG_TYPE_NIL) => {
                t = time(std::ptr::null_mut());
            },
            _ => {
                let mut ts: TM = TM {
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
                    __tm_zone: std::ptr::null(),
                };
                lual_checktype(state, 1, TAG_TYPE_TABLE);
                lua_settop(state, 1);
                ts.tm_year = getfield(state, b"year\0" as *const u8 as *const i8, -1, 1900 as i32);
                ts.tm_mon = getfield(state, b"month\0" as *const u8 as *const i8, -1, 1);
                ts.tm_mday = getfield(state, b"day\0" as *const u8 as *const i8, -1, 0);
                ts.tm_hour = getfield(state, b"hour\0" as *const u8 as *const i8, 12 as i32, 0);
                ts.tm_min = getfield(state, b"min\0" as *const u8 as *const i8, 0, 0);
                ts.tm_sec = getfield(state, b"sec\0" as *const u8 as *const i8, 0, 0);
                ts.tm_isdst = getboolfield(state, b"isdst\0" as *const u8 as *const i8);
                t = mktime(&mut ts);
                setallfields(state, &mut ts);
            }
        };
        if t != t as i64 || t == -1 as i64 {
            return lual_error(
                state,
                b"time result cannot be represented in this installation\0" as *const u8
                    as *const i8,
            );
        }
        (*state).push_integer(t as i64);
        return 1;
    }
}
pub unsafe extern "C" fn os_difftime(state: *mut State) -> i32 {
    unsafe {
        let t1: i64 = l_checktime(state, 1);
        let t2: i64 = l_checktime(state, 2);
        (*state).push_number(difftime(t1, t2));
        return 1;
    }
}
pub unsafe extern "C" fn os_setlocale(state: *mut State) -> i32 {
    unsafe {
        pub const CATEGORY: [i32; 6] = [6, 3, 0, 4, 1, 2];
        pub const CATEGORY_NAMES: [*const i8; 7] = [
            b"all\0" as *const u8 as *const i8,
            b"collate\0" as *const u8 as *const i8,
            b"ctype\0" as *const u8 as *const i8,
            b"monetary\0" as *const u8 as *const i8,
            b"numeric\0" as *const u8 as *const i8,
            b"time\0" as *const u8 as *const i8,
            std::ptr::null(),
        ];
        let l: *const i8 = lual_optlstring(state, 1, std::ptr::null(), std::ptr::null_mut());
        let op: i32 = lual_checkoption(
            state,
            2,
            b"all\0" as *const u8 as *const i8,
            CATEGORY_NAMES.as_ptr(),
        );
        lua_pushstring(state, setlocale(CATEGORY[op as usize], l));
        return 1;
    }
}
pub unsafe extern "C" fn os_exit(state: *mut State) -> i32 {
    unsafe {
        let status: i32;
        if lua_type(state, 1) == Some(TAG_TYPE_BOOLEAN) {
            status = if lua_toboolean(state, 1) != 0 { 0 } else { 1 };
        } else {
            status = lual_optinteger(state, 1, 0) as i32;
        }
        if lua_toboolean(state, 2) != 0 {
            lua_close(state);
        }
        if !state.is_null() {
            exit(status);
        }
        return 0;
    }
}
pub const SYSTEM_FUNCTIONS: [RegisteredFunction; 12] = {
    [
        {
            RegisteredFunction {
                name: b"clock\0" as *const u8 as *const i8,
                function: Some(os_clock as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"date\0" as *const u8 as *const i8,
                function: Some(os_date as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"difftime\0" as *const u8 as *const i8,
                function: Some(os_difftime as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"execute\0" as *const u8 as *const i8,
                function: Some(os_execute as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"exit\0" as *const u8 as *const i8,
                function: Some(os_exit as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"getenv\0" as *const u8 as *const i8,
                function: Some(os_getenv as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"remove\0" as *const u8 as *const i8,
                function: Some(os_remove as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"rename\0" as *const u8 as *const i8,
                function: Some(os_rename as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"setlocale\0" as *const u8 as *const i8,
                function: Some(os_setlocale as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"time\0" as *const u8 as *const i8,
                function: Some(os_time as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"tmpname\0" as *const u8 as *const i8,
                function: Some(os_tmpname as unsafe extern "C" fn(*mut State) -> i32),
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
pub unsafe extern "C" fn luaopen_os(state: *mut State) -> i32 {
    unsafe {
        lual_checkversion_(
            state,
            504.0,
            (::core::mem::size_of::<i64>() as u64)
                .wrapping_mul(16 as u64)
                .wrapping_add(::core::mem::size_of::<f64>() as u64),
        );
        (*state).lua_createtable();
        lual_setfuncs(state, SYSTEM_FUNCTIONS.as_ptr(), 0);
        return 1;
    }
}
