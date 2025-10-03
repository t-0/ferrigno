use crate::buffer::*;
use crate::c::*;
use crate::character::*;
use crate::interpreter::*;
use crate::registeredfunction::*;
use crate::tag::*;
use crate::tdefaultnew::*;
use libc::time;
use libc::{remove, rename, setlocale, system};
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
        libc::strcpy(buffer.as_mut_ptr(), c"/tmp/lua_XXXXXX".as_ptr());
        err = mkstemp(buffer.as_mut_ptr());
        if err != -1 {
            libc::close(err);
        }
        err = (err == -1) as i32;
        if (err != 0) as i64 != 0 {
            return lual_error(interpreter, c"unable to generate a unique filename".as_ptr());
        }
        lua_pushstring(interpreter, buffer.as_mut_ptr());
        return 1;
    }
}
pub unsafe fn os_getenv(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        lua_pushstring(interpreter, libc::getenv(lual_checklstring(interpreter, 1, null_mut())));
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
        setfield(interpreter, c"year".as_ptr(), (*stm).tm_year, 1900 as i32);
        setfield(interpreter, c"month".as_ptr(), (*stm).tm_mon, 1);
        setfield(interpreter, c"day".as_ptr(), (*stm).tm_mday, 0);
        setfield(interpreter, c"hour".as_ptr(), (*stm).tm_hour, 0);
        setfield(interpreter, c"min".as_ptr(), (*stm).tm_min, 0);
        setfield(interpreter, c"sec".as_ptr(), (*stm).tm_sec, 0);
        setfield(interpreter, c"yday".as_ptr(), (*stm).tm_yday, 1);
        setfield(interpreter, c"wday".as_ptr(), (*stm).tm_wday, 1);
        setboolfield(interpreter, c"isdst".as_ptr(), 0 != (*stm).tm_isdst);
    }
}
pub unsafe fn getboolfield(interpreter: *mut Interpreter, key: *const i8) -> i32 {
    unsafe {
        let res = if lua_getfield(interpreter, -1, key) == TagType::Nil {
            -1
        } else {
            lua_toboolean(interpreter, -1) as i32
        };
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
                return lual_error(interpreter, c"field '%s' is not an integer".as_ptr(), key);
            } else if d < 0 {
                return lual_error(interpreter, c"field '%s' missing in date table".as_ptr(), key);
            }
            res = d as i64;
        } else {
            if if res >= 0 {
                (res - delta as i64 <= 0x7FFFFFFF as i64) as i32
            } else {
                ((-(0x7FFFFFFF as i32) - 1 + delta) as i64 <= res) as i32
            } == 0
            {
                return lual_error(interpreter, c"field '%s' is out-of-bound".as_ptr(), key);
            }
            res -= delta as i64;
        }
        lua_settop(interpreter, -2);
        return res as i32;
    }
}
pub unsafe fn checkoption(interpreter: *mut Interpreter, conv: *const i8, convlen: i64, buffer: *mut i8) -> *const i8 {
    unsafe {
        let mut option: *const i8 = c"aAbBcCdDeFgGhHIjmMnprRStTuUVwWxXyYzZ%||EcECExEXEyEYOdOeOHOIOmOMOSOuOUOVOwOWOy".as_ptr();
        let mut oplen: i32 = 1;
        while *option as i32 != Character::Null as i32 && oplen as i64 <= convlen {
            if *option as i32 == '|' as i32 {
                oplen += 1;
            } else if libc::memcmp(conv as *const libc::c_void, option as *const libc::c_void, oplen as usize) == 0 {
                libc::memcpy(buffer as *mut libc::c_void, conv as *const libc::c_void, oplen as usize);
                *buffer.offset(oplen as isize) = Character::Null as i8;
                return conv.offset(oplen as isize);
            }
            option = option.offset(oplen as isize);
        }
        lual_argerror(
            interpreter,
            1,
            lua_pushfstring(interpreter, c"invalid conversion specifier '%%%s'".as_ptr(), conv),
        );
        return conv;
    }
}
pub unsafe fn l_checktime(interpreter: *mut Interpreter, arg: i32) -> i64 {
    unsafe {
        let t: i64 = lual_checkinteger(interpreter, arg);
        if t as i64 != t {
            lual_argerror(interpreter, arg, c"time out-of-bounds".as_ptr());
        }
        return t as i64;
    }
}
pub unsafe fn os_date(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let mut slen: usize = 0;
        let mut stringpointer: *const i8 = lual_optlstring(interpreter, 1, c"%c".as_ptr(), &mut slen);
        let mut t: i64 = if TagType::is_none_or_nil(lua_type(interpreter, 2)) {
            time(null_mut())
        } else {
            l_checktime(interpreter, 2)
        };
        let stringend: *const i8 = stringpointer.offset(slen as isize);
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
        if *stringpointer as i32 == Character::Exclamation as i32 {
            stm = gmtime_r(&mut t, &mut tmr);
            stringpointer = stringpointer.offset(1);
        } else {
            stm = localtime_r(&mut t, &mut tmr);
        }
        if stm.is_null() {
            return lual_error(interpreter, c"date result cannot be represented in this installation".as_ptr());
        }
        if libc::strcmp(stringpointer, c"*t".as_ptr()) == 0 {
            (*interpreter).lua_createtable();
            setallfields(interpreter, stm);
        } else {
            let mut cc: [i8; 4] = [0; 4];
            let mut b = Buffer::new();
            cc[0] = Character::Percent as i8;
            b.initialize(interpreter);
            while stringpointer < stringend {
                if *stringpointer as i32 != Character::Percent as i32 {
                    (b.buffer_loads.get_length() < b.buffer_loads.get_size() || !(b.prepare_with_size(1)).is_null()) as i32;
                    let fresh157 = stringpointer;
                    stringpointer = stringpointer.offset(1);
                    let fresh158 = b.buffer_loads.get_length();
                    b.buffer_loads
                        .set_length((b.buffer_loads.get_length()).wrapping_add(1) as usize);
                    *(b.buffer_loads.loads_pointer).offset(fresh158 as isize) = *fresh157;
                } else {
                    let reslen: usize;
                    let buffer: *mut i8 = b.prepare_with_size(250);
                    stringpointer = stringpointer.offset(1);
                    stringpointer = checkoption(
                        interpreter,
                        stringpointer,
                        stringend.offset_from(stringpointer) as i64,
                        cc.as_mut_ptr().offset(1 as isize),
                    );
                    reslen = strftime(buffer, 250, cc.as_mut_ptr(), stm);
                    b.buffer_loads
                        .set_length((b.buffer_loads.get_length() as usize).wrapping_add(reslen as usize));
                }
            }
            b.push_result();
        }
        return 1;
    }
}
pub unsafe fn os_time(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let sometime: i64;
        match lua_type(interpreter, 1) {
            | None | Some(TagType::Nil) => {
                sometime = time(null_mut());
            },
            | _ => {
                let mut timestruct: TM = TM {
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
                (*interpreter).lual_checktype(1, TagType::Table);
                lua_settop(interpreter, 1);
                timestruct.tm_year = getfield(interpreter, c"year".as_ptr(), -1, 1900 as i32);
                timestruct.tm_mon = getfield(interpreter, c"month".as_ptr(), -1, 1);
                timestruct.tm_mday = getfield(interpreter, c"day".as_ptr(), -1, 0);
                timestruct.tm_hour = getfield(interpreter, c"hour".as_ptr(), 12 as i32, 0);
                timestruct.tm_min = getfield(interpreter, c"min".as_ptr(), 0, 0);
                timestruct.tm_sec = getfield(interpreter, c"sec".as_ptr(), 0, 0);
                timestruct.tm_isdst = getboolfield(interpreter, c"isdst".as_ptr());
                sometime = mktime(&mut timestruct);
                setallfields(interpreter, &mut timestruct);
            },
        };
        if sometime != sometime as i64 || sometime == -1 as i64 {
            return lual_error(interpreter, c"time result cannot be represented in this installation".as_ptr());
        }
        (*interpreter).push_integer(sometime as i64);
        return 1;
    }
}
pub unsafe fn os_difftime(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let sometime1: i64 = l_checktime(interpreter, 1);
        let sometime2: i64 = l_checktime(interpreter, 2);
        (*interpreter).push_number(libc::difftime(sometime1, sometime2));
        return 1;
    }
}
pub unsafe fn os_setlocale(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        pub const CATEGORY: [i32; 6] = [6, 3, 0, 4, 1, 2];
        pub const CATEGORY_NAMES: [*const i8; 7] = [
            c"all".as_ptr(),
            c"collate".as_ptr(),
            c"ctype".as_ptr(),
            c"monetary".as_ptr(),
            c"numeric".as_ptr(),
            c"time".as_ptr(),
            null(),
        ];
        let text: *const i8 = lual_optlstring(interpreter, 1, null(), null_mut());
        let op: i32 = lual_checkoption(interpreter, 2, c"all".as_ptr(), CATEGORY_NAMES.as_ptr());
        lua_pushstring(interpreter, setlocale(CATEGORY[op as usize], text));
        return 1;
    }
}
pub unsafe fn os_exit(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let status: i32 = if lua_type(interpreter, 1) == Some(TagType::Boolean) {
            if lua_toboolean(interpreter, 1) { 0 } else { 1 }
        } else {
            lual_optinteger(interpreter, 1, 0) as i32
        };
        if lua_toboolean(interpreter, 2) {
            lua_close(interpreter);
        }
        if interpreter.is_null() {
            return 0;
        } else {
            std::process::exit(status);
        }
    }
}
pub const SYSTEM_FUNCTIONS: [RegisteredFunction; 11] = [
    RegisteredFunction {
        registeredfunction_name: c"clock".as_ptr(),
        registeredfunction_function: Some(os_clock as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"date".as_ptr(),
        registeredfunction_function: Some(os_date as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"difftime".as_ptr(),
        registeredfunction_function: Some(os_difftime as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"execute".as_ptr(),
        registeredfunction_function: Some(os_execute as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"exit".as_ptr(),
        registeredfunction_function: Some(os_exit as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"getenv".as_ptr(),
        registeredfunction_function: Some(os_getenv as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"remove".as_ptr(),
        registeredfunction_function: Some(os_remove as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"rename".as_ptr(),
        registeredfunction_function: Some(os_rename as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"setlocale".as_ptr(),
        registeredfunction_function: Some(os_setlocale as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"time".as_ptr(),
        registeredfunction_function: Some(os_time as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"tmpname".as_ptr(),
        registeredfunction_function: Some(os_tmpname as unsafe fn(*mut Interpreter) -> i32),
    },
];
pub unsafe fn luaopen_os(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        (*interpreter).lua_createtable();
        lual_setfuncs(interpreter, SYSTEM_FUNCTIONS.as_ptr(), SYSTEM_FUNCTIONS.len(), 0);
        return 1;
    }
}
