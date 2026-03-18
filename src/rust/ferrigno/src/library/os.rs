use crate::buffer::*;
use crate::character::*;
use crate::registeredfunction::*;
use crate::state::*;
use crate::tagtype::*;
use crate::tdefaultnew::*;
use crate::utility::*;
use std::ptr::*;
#[repr(C)]
pub struct Tm {
    pub tm_sec: i32,
    pub tm_min: i32,
    pub tm_hour: i32,
    pub tm_mday: i32,
    pub tm_mon: i32,
    pub tm_year: i32,
    pub tm_wday: i32,
    pub tm_yday: i32,
    pub tm_isdst: i32,
    pub tm_gmtoff: i64,
    pub tm_zone: *mut i8,
}
impl Tm {
    pub fn zeroed() -> Self {
        Tm {
            tm_sec: 0,
            tm_min: 0,
            tm_hour: 0,
            tm_mday: 0,
            tm_mon: 0,
            tm_year: 0,
            tm_wday: 0,
            tm_yday: 0,
            tm_isdst: 0,
            tm_gmtoff: 0,
            tm_zone: null_mut(),
        }
    }
}
unsafe extern "C" {
    fn gmtime_r(timep: *const i64, result: *mut Tm) -> *mut Tm;
    fn localtime_r(timep: *const i64, result: *mut Tm) -> *mut Tm;
    fn strftime(s: *mut i8, max: usize, format: *const i8, tm: *const Tm) -> usize;
    fn mktime(tm: *mut Tm) -> i64;
}
pub unsafe fn os_execute(state: *mut State) -> i32 {
    unsafe {
        let cmd: *const i8 = lual_optlstring(state, 1, null(), null_mut());
        if cmd.is_null() {
            // Check if shell is available
            (*state).push_boolean(true);
            return 1;
        }
        set_errno(0);
        let cmd_str = std::ffi::CStr::from_ptr(cmd).to_str().unwrap_or("");
        match std::process::Command::new("/bin/sh").arg("-c").arg(cmd_str).status() {
            | Ok(status) => {
                #[cfg(unix)]
                {
                    use std::os::unix::process::ExitStatusExt;
                    lual_execresult(state, status.into_raw())
                }
                #[cfg(not(unix))]
                {
                    let code = status.code().unwrap_or(-1);
                    return lual_execresult(state, code << 8);
                }
            },
            | Err(e) => {
                set_errno(e.raw_os_error().unwrap_or(1));
                lual_execresult(state, -1)
            },
        }
    }
}
pub unsafe fn os_remove(state: *mut State) -> i32 {
    unsafe {
        let filename: *const i8 = lual_checklstring(state, 1, null_mut());
        set_errno(0);
        let path = std::ffi::CStr::from_ptr(filename).to_str().unwrap_or("");
        let result = std::fs::remove_file(path).or_else(|_| std::fs::remove_dir(path));
        if let Err(e) = &result {
            set_errno(e.raw_os_error().unwrap_or(1));
        }
        lual_fileresult(state, result.is_ok() as i32, filename)
    }
}
pub unsafe fn os_rename(state: *mut State) -> i32 {
    unsafe {
        let fromname: *const i8 = lual_checklstring(state, 1, null_mut());
        let toname: *const i8 = lual_checklstring(state, 2, null_mut());
        set_errno(0);
        let from = std::ffi::CStr::from_ptr(fromname).to_str().unwrap_or("");
        let to = std::ffi::CStr::from_ptr(toname).to_str().unwrap_or("");
        let result = std::fs::rename(from, to);
        if let Err(e) = &result {
            set_errno(e.raw_os_error().unwrap_or(1));
        }
        lual_fileresult(state, result.is_ok() as i32, null())
    }
}
pub unsafe fn os_tmpname(state: *mut State) -> i32 {
    unsafe {
        let dur = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default();
        let unique = dur.as_nanos() ^ (std::process::id() as u128);
        let mut buffer: [u8; 64] = [0; 64];
        let path = format!("/tmp/lua_{:x}", unique);
        let len = path.len().min(63);
        buffer[..len].copy_from_slice(&path.as_bytes()[..len]);
        buffer[len] = 0;
        // Create the file to reserve the name
        match std::fs::File::create(&path) {
            | Ok(_) => {},
            | Err(_) => {
                return lual_error(state, c"unable to generate a unique filename".as_ptr(), &[]);
            },
        }
        lua_pushstring(state, buffer.as_ptr() as *const i8);
        1
    }
}
pub unsafe fn os_getenv(state: *mut State) -> i32 {
    unsafe {
        lua_pushstring(state, crate::utility::os_getenv(lual_checklstring(state, 1, null_mut())));
        1
    }
}
pub unsafe fn os_clock(state: *mut State) -> i32 {
    unsafe {
        #[repr(C)]
        struct Timespec {
            tv_sec: i64,
            tv_nsec: i64,
        }
        unsafe extern "C" {
            fn clock_gettime(clk_id: i32, tp: *mut Timespec) -> i32;
        }
        #[cfg(target_os = "macos")]
        const CLOCK_PROCESS_CPUTIME_ID: i32 = 12;
        #[cfg(not(target_os = "macos"))]
        const CLOCK_PROCESS_CPUTIME_ID: i32 = 2;
        let mut ts = Timespec { tv_sec: 0, tv_nsec: 0 };
        clock_gettime(CLOCK_PROCESS_CPUTIME_ID, &mut ts);
        (*state).push_number(ts.tv_sec as f64 + ts.tv_nsec as f64 / 1_000_000_000.0);
        1
    }
}
// os.monotime() — wall-clock seconds (CLOCK_MONOTONIC), high resolution.
// Unlike os.clock() this advances even while the process is blocked in I/O.
pub unsafe fn os_monotime(state: *mut State) -> i32 {
    unsafe {
        use std::sync::OnceLock;
        static START: OnceLock<std::time::Instant> = OnceLock::new();
        let start = START.get_or_init(std::time::Instant::now);
        (*state).push_number(start.elapsed().as_secs_f64());
        1
    }
}
pub unsafe fn setfield(state: *mut State, key: *const i8, value: i32, delta: i32) {
    unsafe {
        (*state).push_integer(value as i64 + delta as i64);
        lua_setfield(state, -2, key);
    }
}
pub unsafe fn setboolfield(state: *mut State, key: *const i8, value: bool) {
    unsafe {
        (*state).push_boolean(value);
        lua_setfield(state, -2, key);
    }
}
pub unsafe fn setallfields(state: *mut State, stm: *mut Tm) {
    unsafe {
        setfield(state, c"year".as_ptr(), (*stm).tm_year, 1900_i32);
        setfield(state, c"month".as_ptr(), (*stm).tm_mon, 1);
        setfield(state, c"day".as_ptr(), (*stm).tm_mday, 0);
        setfield(state, c"hour".as_ptr(), (*stm).tm_hour, 0);
        setfield(state, c"min".as_ptr(), (*stm).tm_min, 0);
        setfield(state, c"sec".as_ptr(), (*stm).tm_sec, 0);
        setfield(state, c"yday".as_ptr(), (*stm).tm_yday, 1);
        setfield(state, c"wday".as_ptr(), (*stm).tm_wday, 1);
        setboolfield(state, c"isdst".as_ptr(), 0 != (*stm).tm_isdst);
    }
}
pub unsafe fn getboolfield(state: *mut State, key: *const i8) -> i32 {
    unsafe {
        let res = if lua_getfield(state, -1, key) == TagType::Nil {
            -1
        } else {
            lua_toboolean(state, -1) as i32
        };
        lua_settop(state, -2);
        res
    }
}
pub unsafe fn getfield(state: *mut State, key: *const i8, d: i32, delta: i32) -> i32 {
    unsafe {
        let mut is_number = false;
        let t = lua_getfield(state, -1, key);
        let mut res: i64 = lua_tointegerx(state, -1, &mut is_number);
        if !is_number {
            if t != TagType::Nil {
                return lual_error(state, c"field '%s' is not an integer".as_ptr(), &[key.into()]);
            } else if d < 0 {
                return lual_error(state, c"field '%s' missing in date table".as_ptr(), &[key.into()]);
            }
            res = d as i64;
        } else {
            if if res >= 0 {
                (res - delta as i64 <= i32::MAX as i64) as i32
            } else {
                ((i32::MIN + delta) as i64 <= res) as i32
            } == 0
            {
                return lual_error(state, c"field '%s' is out-of-bound".as_ptr(), &[key.into()]);
            }
            res -= delta as i64;
        }
        lua_settop(state, -2);
        res as i32
    }
}
pub unsafe fn checkoption(state: *mut State, conv: *const i8, convlen: i64, buffer: *mut i8) -> *const i8 {
    unsafe {
        let mut option: *const i8 = c"aAbBcCdDeFgGhHIjmMnprRStTuUVwWxXyYzZ%||EcECExEXEyEYOdOeOHOIOmOMOSOuOUOVOwOWOy".as_ptr();
        let mut oplen: i32 = 1;
        while *option as i32 != Character::Null as i32 && oplen as i64 <= convlen {
            if *option as i32 == '|' as i32 {
                oplen += 1;
            } else if std::slice::from_raw_parts(conv as *const u8, oplen as usize)
                == std::slice::from_raw_parts(option as *const u8, oplen as usize)
            {
                std::ptr::copy_nonoverlapping(conv as *const u8, buffer as *mut u8, oplen as usize);
                *buffer.add(oplen as usize) = Character::Null as i8;
                return conv.add(oplen as usize);
            }
            option = option.add(oplen as usize);
        }
        lual_argerror(
            state,
            1,
            lua_pushfstring(state, c"invalid conversion specifier '%%%s'".as_ptr(), &[conv.into()]),
        );
        conv
    }
}
pub unsafe fn l_checktime(state: *mut State, arg: i32) -> i64 {
    unsafe {
        let t: i64 = lual_checkinteger(state, arg);
        if t != t {
            lual_argerror(state, arg, c"time out-of-bounds".as_ptr());
        }
        t
    }
}
pub unsafe fn os_date(state: *mut State) -> i32 {
    unsafe {
        let mut slen: usize = 0;
        let mut stringpointer: *const i8 = lual_optlstring(state, 1, c"%c".as_ptr(), &mut slen);
        let t: i64 = if TagType::is_none_or_nil(lua_type(state, 2)) {
            os_time_now()
        } else {
            l_checktime(state, 2)
        };
        let stringend: *const i8 = stringpointer.add(slen);
        let mut tmr = Tm::zeroed();
        let stm: *mut Tm;
        if *stringpointer as i32 == Character::Exclamation as i32 {
            stm = gmtime_r(&t, &mut tmr);
            stringpointer = stringpointer.add(1);
        } else {
            stm = localtime_r(&t, &mut tmr);
        }
        if stm.is_null() {
            return lual_error(state, c"date result cannot be represented in this installation".as_ptr(), &[]);
        }
        if std::ffi::CStr::from_ptr(stringpointer) == c"*t" {
            (*state).lua_createtable();
            setallfields(state, stm);
        } else {
            let mut cc: [i8; 4] = [0; 4];
            let mut b = Buffer::new();
            cc[0] = Character::Percent as i8;
            b.initialize(state);
            while stringpointer < stringend {
                if *stringpointer as i32 != Character::Percent as i32 {
                    (b.buffer_loads.get_length() < b.buffer_loads.get_size() || !(b.prepare_with_size(1)).is_null()) as i32;
                    let current_char = stringpointer;
                    stringpointer = stringpointer.add(1);
                    let write_offset = b.buffer_loads.get_length();
                    b.buffer_loads.set_length((b.buffer_loads.get_length() + 1) as usize);
                    *(b.buffer_loads.loads_pointer).add(write_offset as usize) = *current_char;
                } else {
                    let buffer: *mut i8 = b.prepare_with_size(250);
                    stringpointer = stringpointer.add(1);
                    stringpointer = checkoption(
                        state,
                        stringpointer,
                        stringend.offset_from(stringpointer) as i64,
                        cc.as_mut_ptr().add(1),
                    );
                    let reslen: usize = strftime(buffer, 250, cc.as_mut_ptr(), stm);
                    b.buffer_loads.set_length(b.buffer_loads.get_length() as usize + reslen);
                }
            }
            b.push_result();
        }
        1
    }
}
pub unsafe fn os_time(state: *mut State) -> i32 {
    unsafe {
        let sometime: i64;
        match lua_type(state, 1) {
            | None | Some(TagType::Nil) => {
                sometime = os_time_now();
            },
            | _ => {
                let mut timestruct = Tm::zeroed();
                (*state).lual_checktype(1, TagType::Table);
                lua_settop(state, 1);
                timestruct.tm_year = getfield(state, c"year".as_ptr(), -1, 1900_i32);
                timestruct.tm_mon = getfield(state, c"month".as_ptr(), -1, 1);
                timestruct.tm_mday = getfield(state, c"day".as_ptr(), -1, 0);
                timestruct.tm_hour = getfield(state, c"hour".as_ptr(), 12_i32, 0);
                timestruct.tm_min = getfield(state, c"min".as_ptr(), 0, 0);
                timestruct.tm_sec = getfield(state, c"sec".as_ptr(), 0, 0);
                timestruct.tm_isdst = getboolfield(state, c"isdst".as_ptr());
                sometime = mktime(&mut timestruct);
                setallfields(state, &mut timestruct);
            },
        };
        if sometime != sometime || sometime == -1_i64 {
            return lual_error(state, c"time result cannot be represented in this installation".as_ptr(), &[]);
        }
        (*state).push_integer(sometime);
        1
    }
}
pub unsafe fn os_difftime(state: *mut State) -> i32 {
    unsafe {
        let sometime1: i64 = l_checktime(state, 1);
        let sometime2: i64 = l_checktime(state, 2);
        (*state).push_number((sometime1 - sometime2) as f64);
        1
    }
}
pub unsafe fn os_setlocale(state: *mut State) -> i32 {
    unsafe extern "C" {
        fn setlocale(category: i32, locale: *const i8) -> *mut i8;
    }
    const LC_ALL: i32 = 0;
    const LC_COLLATE: i32 = 1;
    const LC_CTYPE: i32 = 2;
    const LC_MONETARY: i32 = 3;
    const LC_NUMERIC: i32 = 4;
    const LC_TIME: i32 = 5;
    unsafe {
        pub const CATEGORY: [i32; 6] = [LC_ALL, LC_COLLATE, LC_CTYPE, LC_MONETARY, LC_NUMERIC, LC_TIME];
        pub const CATEGORY_NAMES: [*const i8; 7] = [
            c"all".as_ptr(),
            c"collate".as_ptr(),
            c"ctype".as_ptr(),
            c"monetary".as_ptr(),
            c"numeric".as_ptr(),
            c"time".as_ptr(),
            null(),
        ];
        let text: *const i8 = lual_optlstring(state, 1, null(), null_mut());
        let op: i32 = lual_checkoption(state, 2, c"all".as_ptr(), CATEGORY_NAMES.as_ptr());
        lua_pushstring(state, setlocale(CATEGORY[op as usize], text));
        1
    }
}
pub unsafe fn os_exit(state: *mut State) -> i32 {
    unsafe {
        let status: i32 = if lua_type(state, 1) == Some(TagType::Boolean) {
            if lua_toboolean(state, 1) { 0 } else { 1 }
        } else {
            lual_optinteger(state, 1, 0) as i32
        };
        if lua_toboolean(state, 2) {
            lua_close(state);
        }
        if state.is_null() {
            0
        } else {
            std::process::exit(status);
        }
    }
}
pub const SYSTEM_FUNCTIONS: [RegisteredFunction; 12] = [
    RegisteredFunction {
        registeredfunction_name: c"clock".as_ptr(),
        registeredfunction_function: Some(os_clock as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"monotime".as_ptr(),
        registeredfunction_function: Some(os_monotime as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"date".as_ptr(),
        registeredfunction_function: Some(os_date as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"difftime".as_ptr(),
        registeredfunction_function: Some(os_difftime as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"execute".as_ptr(),
        registeredfunction_function: Some(os_execute as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"exit".as_ptr(),
        registeredfunction_function: Some(os_exit as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"getenv".as_ptr(),
        registeredfunction_function: Some(os_getenv as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"remove".as_ptr(),
        registeredfunction_function: Some(os_remove as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"rename".as_ptr(),
        registeredfunction_function: Some(os_rename as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"setlocale".as_ptr(),
        registeredfunction_function: Some(os_setlocale as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"time".as_ptr(),
        registeredfunction_function: Some(os_time as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"tmpname".as_ptr(),
        registeredfunction_function: Some(os_tmpname as unsafe fn(*mut State) -> i32),
    },
];
pub unsafe fn luaopen_os(state: *mut State) -> i32 {
    unsafe {
        (*state).lua_createtable();
        lual_setfuncs(state, SYSTEM_FUNCTIONS.as_ptr(), SYSTEM_FUNCTIONS.len(), 0);
        1
    }
}
