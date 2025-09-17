use crate::buffer::*;
use crate::character::CHARACTER_EXCLAMATION;
use crate::character::*;
use crate::interpreter::*;
use crate::new::*;
use crate::registeredfunction::*;
use crate::tag::*;
use crate::utility::c::*;
use libc::{remove, rename, setlocale, system};
use rlua::*;
use std::ptr::*;
pub unsafe fn os_execute(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let cmd: *const i8 = lual_optlstring(interpreter, 1, null(), null_mut());
        let stat: i32;
        *__errno_location() = 0;
        stat = system(cmd);
        if !cmd.is_null() {
            return lual_execresult(interpreter, stat);
        } else {
            (*interpreter).push_boolean(0 != stat);
            return 1;
        };
    }
}
pub unsafe fn os_remove(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let filename: *const i8 = lual_checklstring(interpreter, 1, null_mut());
        *__errno_location() = 0;
        return lual_fileresult(interpreter, (remove(filename) == 0) as i32, filename);
    }
}
pub unsafe fn os_rename(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let fromname: *const i8 = lual_checklstring(interpreter, 1, null_mut());
        let toname: *const i8 = lual_checklstring(interpreter, 2, null_mut());
        *__errno_location() = 0;
        return lual_fileresult(interpreter, (rename(fromname, toname) == 0) as i32, null());
    }
}
pub unsafe fn os_tmpname(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let mut buffer: [i8; 32] = [0; 32];
        let mut err: i32;
        strcpy(buffer.as_mut_ptr(), make_cstring!("/tmp/lua_XXXXXX"));
        err = mkstemp(buffer.as_mut_ptr());
        if err != -1 {
            close(err);
        }
        err = (err == -1) as i32;
        if (err != 0) as i64 != 0 {
            return lual_error(interpreter, make_cstring!("unable to generate a unique filename"));
        }
        lua_pushstring(interpreter, buffer.as_mut_ptr());
        return 1;
    }
}
pub unsafe fn os_getenv(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        lua_pushstring(interpreter, getenv(lual_checklstring(interpreter, 1, null_mut())));
        return 1;
    }
}
pub unsafe fn os_clock(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        (*interpreter).push_number(clock() as f64 / 1000000 as f64);
        return 1;
    }
}
pub unsafe fn setfield(interpreter: *mut Interpreter, key: *const i8, value: i32, delta: i32) {
    unsafe {
        (*interpreter).push_integer(value as i64 + delta as i64);
        lua_setfield(interpreter, -2, key);
    }
}
pub unsafe fn setboolfield(interpreter: *mut Interpreter, key: *const i8, value: bool) {
    unsafe {
        (*interpreter).push_boolean(value);
        lua_setfield(interpreter, -2, key);
    }
}
pub unsafe fn setallfields(interpreter: *mut Interpreter, stm: *mut TM) {
    unsafe {
        setfield(interpreter, make_cstring!("year"), (*stm).tm_year, 1900 as i32);
        setfield(interpreter, make_cstring!("month"), (*stm).tm_mon, 1);
        setfield(interpreter, make_cstring!("day"), (*stm).tm_mday, 0);
        setfield(interpreter, make_cstring!("hour"), (*stm).tm_hour, 0);
        setfield(interpreter, make_cstring!("min"), (*stm).tm_min, 0);
        setfield(interpreter, make_cstring!("sec"), (*stm).tm_sec, 0);
        setfield(interpreter, make_cstring!("yday"), (*stm).tm_yday, 1);
        setfield(interpreter, make_cstring!("wday"), (*stm).tm_wday, 1);
        setboolfield(interpreter, make_cstring!("isdst"), 0 != (*stm).tm_isdst);
    }
}
pub unsafe fn getboolfield(interpreter: *mut Interpreter, key: *const i8) -> i32 {
    unsafe {
        let res: i32;
        res = if lua_getfield(interpreter, -1, key) == TagType::Nil { -1 } else { lua_toboolean(interpreter, -1) };
        lua_settop(interpreter, -2);
        return res;
    }
}
pub unsafe fn getfield(interpreter: *mut Interpreter, key: *const i8, d: i32, delta: i32) -> i32 {
    unsafe {
        let mut is_number = false;
        let t = lua_getfield(interpreter, -1, key);
        let mut res: i64 = lua_tointegerx(interpreter, -1, &mut is_number);
        if !is_number {
            if t != TagType::Nil {
                return lual_error(interpreter, make_cstring!("field '%s' is not an integer"), key);
            } else if d < 0 {
                return lual_error(interpreter, make_cstring!("field '%s' missing in date table"), key);
            }
            res = d as i64;
        } else {
            if if res >= 0 {
                (res - delta as i64 <= 0x7FFFFFFF as i64) as i32
            } else {
                ((-(0x7FFFFFFF as i32) - 1 + delta) as i64 <= res) as i32
            } == 0
            {
                return lual_error(interpreter, make_cstring!("field '%s' is out-of-bound"), key);
            }
            res -= delta as i64;
        }
        lua_settop(interpreter, -2);
        return res as i32;
    }
}
pub unsafe fn checkoption(interpreter: *mut Interpreter, conv: *const i8, convlen: i64, buffer: *mut i8) -> *const i8 {
    unsafe {
        let mut option: *const i8 = make_cstring!("aAbBcCdDeFgGhHIjmMnprRStTuUVwWxXyYzZ%||EcECExEXEyEYOdOeOHOIOmOMOSOuOUOVOwOWOy");
        let mut oplen: i32 = 1;
        while *option as i32 != Character::Null as i32 && oplen as i64 <= convlen {
            if *option as i32 == '|' as i32 {
                oplen += 1;
            } else if memcmp(conv as *const libc::c_void, option as *const libc::c_void, oplen as usize) == 0 {
                memcpy(buffer as *mut libc::c_void, conv as *const libc::c_void, oplen as usize);
                *buffer.offset(oplen as isize) = Character::Null as i8;
                return conv.offset(oplen as isize);
            }
            option = option.offset(oplen as isize);
        }
        lual_argerror(
            interpreter,
            1,
            lua_pushfstring(interpreter, make_cstring!("invalid conversion specifier '%%%s'"), conv),
        );
        return conv;
    }
}
pub unsafe fn l_checktime(interpreter: *mut Interpreter, arg: i32) -> i64 {
    unsafe {
        let t: i64 = lual_checkinteger(interpreter, arg);
        if t as i64 != t {
            lual_argerror(interpreter, arg, make_cstring!("time out-of-bounds"));
        }
        return t as i64;
    }
}
pub unsafe fn os_date(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let mut slen: usize = 0;
        let mut s: *const i8 = lual_optlstring(interpreter, 1, make_cstring!("%c"), &mut slen);
        let mut t: i64 = if is_none_or_nil(lua_type(interpreter, 2)) {
            time(null_mut())
        } else {
            l_checktime(interpreter, 2)
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
            __tm_zone: null(),
        };
        let stm: *mut TM;
        if *s as i32 == CHARACTER_EXCLAMATION as i32 {
            stm = gmtime_r(&mut t, &mut tmr);
            s = s.offset(1);
        } else {
            stm = localtime_r(&mut t, &mut tmr);
        }
        if stm.is_null() {
            return lual_error(interpreter, make_cstring!("date result cannot be represented in this installation"));
        }
        if strcmp(s, make_cstring!("*t")) == 0 {
            (*interpreter).lua_createtable();
            setallfields(interpreter, stm);
        } else {
            let mut cc: [i8; 4] = [0; 4];
            let mut b = Buffer::new();
            cc[0] = CHARACTER_PERCENT as i8;
            b.initialize(interpreter);
            while s < se {
                if *s as i32 != CHARACTER_PERCENT as i32 {
                    (b.loads.get_length() < b.loads.get_size() || !(b.prepare_with_size(1)).is_null()) as i32;
                    let fresh157 = s;
                    s = s.offset(1);
                    let fresh158 = b.loads.get_length();
                    b.loads.set_length((b.loads.get_length()).wrapping_add(1) as usize);
                    *(b.loads.loads_pointer).offset(fresh158 as isize) = *fresh157;
                } else {
                    let reslen: usize;
                    let buffer: *mut i8 = b.prepare_with_size(250);
                    s = s.offset(1);
                    s = checkoption(interpreter, s, se.offset_from(s) as i64, cc.as_mut_ptr().offset(1 as isize));
                    reslen = strftime(buffer, 250, cc.as_mut_ptr(), stm);
                    b.loads.set_length((b.loads.get_length() as usize).wrapping_add(reslen as usize));
                }
            }
            b.push_result();
        }
        return 1;
    }
}
pub unsafe fn os_time(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let t: i64;
        match lua_type(interpreter, 1) {
            None | Some(TagType::Nil) => {
                t = time(null_mut());
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
                    __tm_zone: null(),
                };
                lual_checktype(interpreter, 1, TagType::Table);
                lua_settop(interpreter, 1);
                ts.tm_year = getfield(interpreter, make_cstring!("year"), -1, 1900 as i32);
                ts.tm_mon = getfield(interpreter, make_cstring!("month"), -1, 1);
                ts.tm_mday = getfield(interpreter, make_cstring!("day"), -1, 0);
                ts.tm_hour = getfield(interpreter, make_cstring!("hour"), 12 as i32, 0);
                ts.tm_min = getfield(interpreter, make_cstring!("min"), 0, 0);
                ts.tm_sec = getfield(interpreter, make_cstring!("sec"), 0, 0);
                ts.tm_isdst = getboolfield(interpreter, make_cstring!("isdst"));
                t = mktime(&mut ts);
                setallfields(interpreter, &mut ts);
            },
        };
        if t != t as i64 || t == -1 as i64 {
            return lual_error(interpreter, make_cstring!("time result cannot be represented in this installation"));
        }
        (*interpreter).push_integer(t as i64);
        return 1;
    }
}
pub unsafe fn os_difftime(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let t1: i64 = l_checktime(interpreter, 1);
        let t2: i64 = l_checktime(interpreter, 2);
        (*interpreter).push_number(difftime(t1, t2));
        return 1;
    }
}
pub unsafe fn os_setlocale(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        pub const CATEGORY: [i32; 6] = [6, 3, 0, 4, 1, 2];
        pub const CATEGORY_NAMES: [*const i8; 7] = [
            make_cstring!("all"),
            make_cstring!("collate"),
            make_cstring!("ctype"),
            make_cstring!("monetary"),
            make_cstring!("numeric"),
            make_cstring!("time"),
            null(),
        ];
        let l: *const i8 = lual_optlstring(interpreter, 1, null(), null_mut());
        let op: i32 = lual_checkoption(interpreter, 2, make_cstring!("all"), CATEGORY_NAMES.as_ptr());
        lua_pushstring(interpreter, setlocale(CATEGORY[op as usize], l));
        return 1;
    }
}
pub unsafe fn os_exit(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let status: i32;
        if lua_type(interpreter, 1) == Some(TagType::Boolean) {
            status = if lua_toboolean(interpreter, 1) != 0 { 0 } else { 1 };
        } else {
            status = lual_optinteger(interpreter, 1, 0) as i32;
        }
        if lua_toboolean(interpreter, 2) != 0 {
            lua_close(interpreter);
        }
        if !interpreter.is_null() {
            exit(status);
        }
        return 0;
    }
}
pub const SYSTEM_FUNCTIONS: [RegisteredFunction; 11] = [
    RegisteredFunction { name: make_cstring!("clock"), function: Some(os_clock as unsafe fn(*mut Interpreter) -> i32) },
    RegisteredFunction { name: make_cstring!("date"), function: Some(os_date as unsafe fn(*mut Interpreter) -> i32) },
    RegisteredFunction { name: make_cstring!("difftime"), function: Some(os_difftime as unsafe fn(*mut Interpreter) -> i32) },
    RegisteredFunction { name: make_cstring!("execute"), function: Some(os_execute as unsafe fn(*mut Interpreter) -> i32) },
    RegisteredFunction { name: make_cstring!("exit"), function: Some(os_exit as unsafe fn(*mut Interpreter) -> i32) },
    RegisteredFunction { name: make_cstring!("getenv"), function: Some(os_getenv as unsafe fn(*mut Interpreter) -> i32) },
    RegisteredFunction { name: make_cstring!("remove"), function: Some(os_remove as unsafe fn(*mut Interpreter) -> i32) },
    RegisteredFunction { name: make_cstring!("rename"), function: Some(os_rename as unsafe fn(*mut Interpreter) -> i32) },
    RegisteredFunction { name: make_cstring!("setlocale"), function: Some(os_setlocale as unsafe fn(*mut Interpreter) -> i32) },
    RegisteredFunction { name: make_cstring!("time"), function: Some(os_time as unsafe fn(*mut Interpreter) -> i32) },
    RegisteredFunction { name: make_cstring!("tmpname"), function: Some(os_tmpname as unsafe fn(*mut Interpreter) -> i32) },
];
pub unsafe fn luaopen_os(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        lual_checkversion_(
            interpreter,
            504.0,
            (size_of::<i64>() as usize).wrapping_mul(16 as usize).wrapping_add(size_of::<f64>() as usize),
        );
        (*interpreter).lua_createtable();
        lual_setfuncs(interpreter, SYSTEM_FUNCTIONS.as_ptr(), SYSTEM_FUNCTIONS.len(), 0);
        return 1;
    }
}
