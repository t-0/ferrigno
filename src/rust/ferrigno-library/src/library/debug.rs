#![allow(unpredictable_function_pointer_comparisons)]
use crate::calls::*;
use crate::character::*;
use crate::debuginfo::*;
use crate::functions::*;
use crate::registeredfunction::*;
use crate::state::*;
use crate::status::*;
use crate::strings::*;
use crate::tagtype::*;
use crate::utility::*;
use std::ptr::*;
pub unsafe fn db_getregistry(state: *mut State) -> i32 {
    unsafe {
        lua_pushvalue(state, LUA_REGISTRYINDEX);
        1
    }
}
pub unsafe fn db_getmetatable(state: *mut State) -> i32 {
    unsafe {
        lual_checkany(state, 1);
        if !(*state).lua_getmetatable(1) {
            (*state).push_nil();
        }
        1
    }
}
pub unsafe fn db_setmetatable(state: *mut State) -> i32 {
    unsafe {
        let t = lua_type(state, 2);
        if !(t == Some(TagType::Nil) || t == Some(TagType::Table)) {
            lual_typeerror(state, 2, c"nil or table".as_ptr());
            0;
        }
        lua_settop(state, 2);
        lua_setmetatable(state, 1);
        1
    }
}
pub unsafe fn db_getuservalue(state: *mut State) -> i32 {
    unsafe {
        let n: i32 = lual_optinteger(state, 2, 1) as i32;
        if lua_type(state, 1) != Some(TagType::User) {
            (*state).push_nil();
        } else if (*state).lua_getiuservalue(1, n).is_some() {
            (*state).push_boolean(true);
            return 2;
        }
        1
    }
}
pub unsafe fn db_setuservalue(state: *mut State) -> i32 {
    unsafe {
        let n: i32 = lual_optinteger(state, 3, 1) as i32;
        (*state).lual_checktype(1, TagType::User);
        lual_checkany(state, 2);
        lua_settop(state, 2);
        if lua_setiuservalue(state, 1, n) == 0 {
            (*state).push_nil();
        }
        1
    }
}
pub unsafe fn db_getupvalue(state: *mut State) -> i32 {
    unsafe { auxupvalue(state, 1) }
}
pub unsafe fn db_setupvalue(state: *mut State) -> i32 {
    unsafe {
        lual_checkany(state, 3);
        auxupvalue(state, 0)
    }
}
pub unsafe fn db_getinfo(state: *mut State) -> i32 {
    unsafe {
        let mut debuginfo: DebugInfo = DebugInfo::new();
        let mut arg: i32 = 0;
        let other_state: *mut State = getthread(state, &mut arg);
        let mut options: *const i8 = lual_optlstring(state, arg + 2, c"flnSrtu".as_ptr(), null_mut());
        checkstack(state, other_state, 3);
        if *options.add(0) as i32 == Character::AngleRight as i32 {
            lual_argerror(state, arg + 2, c"invalid option Character::AngleRight".as_ptr());
            0;
        }
        if lua_type(state, arg + 1) == Some(TagType::Closure) {
            options = lua_pushfstring(state, c">%s".as_ptr(), options);
            lua_pushvalue(state, arg + 1);
            lua_xmove(state, other_state, 1);
        } else if lua_getstack(other_state, lual_checkinteger(state, arg + 1) as i32, &mut debuginfo) == 0 {
            (*state).push_nil();
            return 1;
        }
        if lua_getinfo(other_state, options, &mut debuginfo) == 0 {
            return lual_argerror(state, arg + 2, c"invalid option".as_ptr());
        }
        (*state).lua_createtable();
        if !cstr_chr(options, Character::UpperS as i8).is_null() {
            lua_pushlstring(state, debuginfo.debuginfo_source, debuginfo.debuginfo_source_length);
            lua_setfield(state, -2, c"source".as_ptr());
            settabss(state, c"short_src".as_ptr(), (debuginfo.debuginfo_short_source).as_mut_ptr());
            settabsi(state, c"linedefined".as_ptr(), debuginfo.debuginfo_line_defined);
            settabsi(state, c"lastlinedefined".as_ptr(), debuginfo.debuginfo_last_line_defined);
            settabss(state, c"what".as_ptr(), debuginfo.debuginfo_what);
        }
        if !cstr_chr(options, Character::LowerL as i8).is_null() {
            settabsi(state, c"currentline".as_ptr(), debuginfo.debuginfo_current_line);
        }
        if !cstr_chr(options, Character::LowerU as i8).is_null() {
            settabsi(state, c"nups".as_ptr(), debuginfo.debuginfo_count_upvalues as i32);
            settabsi(state, c"nparams".as_ptr(), debuginfo.debuginfo_count_parameters as i32);
            settabsb(state, c"isvararg".as_ptr(), debuginfo.debuginfo_is_variable_arguments as i32);
        }
        if !cstr_chr(options, Character::LowerN as i8).is_null() {
            settabss(state, c"name".as_ptr(), debuginfo.debuginfo_name);
            settabss(state, c"namewhat".as_ptr(), debuginfo.debuginfo_name_what);
        }
        if !cstr_chr(options, Character::LowerR as i8).is_null() {
            settabsi(state, c"ftransfer".as_ptr(), debuginfo.debuginfo_transfer_function as i32);
            settabsi(state, c"ntransfer".as_ptr(), debuginfo.debuginfo_count_transfer as i32);
        }
        if !cstr_chr(options, Character::LowerT as i8).is_null() {
            settabsb(
                state,
                c"istailcall".as_ptr(),
                if debuginfo.debuginfo_is_tail_call { 1 } else { 0 },
            );
            settabsi(state, c"extraargs".as_ptr(), debuginfo.debuginfo_extra_args);
        }
        if !cstr_chr(options, Character::UpperL as i8).is_null() {
            treatstackoption(state, other_state, c"activelines".as_ptr());
        }
        if !cstr_chr(options, Character::LowerF as i8).is_null() {
            treatstackoption(state, other_state, c"func".as_ptr());
        }
        1
    }
}
pub unsafe fn db_getlocal(state: *mut State) -> i32 {
    unsafe {
        let mut arg: i32 = 0;
        let other_state: *mut State = getthread(state, &mut arg);
        let nvar: i32 = lual_checkinteger(state, arg + 2) as i32;
        if lua_type(state, arg + 1) == Some(TagType::Closure) {
            lua_pushvalue(state, arg + 1);
            lua_pushstring(state, lua_getlocal(state, null(), nvar));
            1
        } else {
            let mut debuginfo: DebugInfo = DebugInfo::new();
            let level: i32 = lual_checkinteger(state, arg + 1) as i32;
            if lua_getstack(other_state, level, &mut debuginfo) == 0 {
                return lual_argerror(state, arg + 1, c"level out of range".as_ptr());
            }
            checkstack(state, other_state, 1);
            let name: *const i8 = lua_getlocal(other_state, &debuginfo, nvar);
            if !name.is_null() {
                lua_xmove(other_state, state, 1);
                lua_pushstring(state, name);
                lua_rotate(state, -2, 1);
                2
            } else {
                (*state).push_nil();
                1
            }
        }
    }
}
pub unsafe fn db_setlocal(state: *mut State) -> i32 {
    unsafe {
        let mut arg: i32 = 0;
        let other_state: *mut State = getthread(state, &mut arg);
        let mut debuginfo: DebugInfo = DebugInfo::new();
        let level: i32 = lual_checkinteger(state, arg + 1) as i32;
        let nvar: i32 = lual_checkinteger(state, arg + 2) as i32;
        if lua_getstack(other_state, level, &mut debuginfo) == 0 {
            lual_argerror(state, arg + 1, c"level out of range".as_ptr())
        } else {
            lual_checkany(state, arg + 3);
            lua_settop(state, arg + 3);
            checkstack(state, other_state, 1);
            lua_xmove(state, other_state, 1);
            let name: *const i8 = lua_setlocal(other_state, &debuginfo, nvar);
            if name.is_null() {
                lua_settop(other_state, -2);
            }
            lua_pushstring(state, name);
            1
        }
    }
}
pub unsafe fn db_upvalueid(state: *mut State) -> i32 {
    unsafe {
        let id: *mut std::ffi::c_void = checkupval(state, 1, 2, null_mut());
        if !id.is_null() {
            lua_pushlightuserdata(state, id);
        } else {
            (*state).push_nil();
        }
        1
    }
}
pub unsafe fn db_upvaluejoin(state: *mut State) -> i32 {
    unsafe {
        let mut n1: i32 = 0;
        let mut n2: i32 = 0;
        checkupval(state, 1, 2, &mut n1);
        checkupval(state, 3, 4, &mut n2);
        if lua_iscfunction(state, 1) {
            lual_argerror(state, 1, c"Lua function expected".as_ptr());
            0;
        }
        if lua_iscfunction(state, 3) {
            lual_argerror(state, 3, c"Lua function expected".as_ptr());
            0;
        }
        lua_upvaluejoin(state, 1, n1, 3, n2);
        0
    }
}
pub unsafe fn db_sethook(state: *mut State) -> i32 {
    unsafe {
        let mut arg: i32 = 0;
        let mask: i32;
        let count: i32;
        let function: HookFunction;
        let other_state: *mut State = getthread(state, &mut arg);
        match lua_type(state, arg + 1) {
            | None | Some(TagType::Nil) => {
                lua_settop(state, arg + 1);
                function = None;
                mask = 0;
                count = 0;
            },
            | _ => {
                let smask: *const i8 = lual_checklstring(state, arg + 2, null_mut());
                (*state).lual_checktype(arg + 1, TagType::Closure);
                count = lual_optinteger(state, arg + 3, 0) as i32;
                function = Some(DebugInfo::hookf as unsafe fn(*mut State, *mut DebugInfo) -> ());
                mask = makemask(smask, count);
            },
        };
        if lual_getsubtable(state, LUA_REGISTRYINDEX, Strings::STRING_HOOKKEY) == 0 {
            lua_pushstring(state, c"k".as_ptr());
            lua_setfield(state, -2, c"__mode".as_ptr());
            lua_pushvalue(state, -1);
            lua_setmetatable(state, -2);
        }
        checkstack(state, other_state, 1);
        (*other_state).push_state();
        lua_xmove(other_state, state, 1);
        lua_pushvalue(state, arg + 1);
        lua_rawset(state, -3);
        lua_sethook(other_state, function, mask, count);
        0
    }
}
pub unsafe fn db_gethook(state: *mut State) -> i32 {
    unsafe {
        let mut arg: i32 = 0;
        let other_state: *mut State = getthread(state, &mut arg);
        let mut buffer: [i8; 5] = [0; 5];
        let mask: i32 = lua_gethookmask(other_state);
        let hook: HookFunction = lua_gethook(other_state);
        if hook.is_none() {
            (*state).push_nil();
            return 1;
        } else if hook != Some(DebugInfo::hookf as unsafe fn(*mut State, *mut DebugInfo) -> ()) {
            lua_pushstring(state, c"external hook".as_ptr());
        } else {
            lua_getfield(state, LUA_REGISTRYINDEX, Strings::STRING_HOOKKEY);
            checkstack(state, other_state, 1);
            (*other_state).push_state();
            lua_xmove(other_state, state, 1);
            lua_rawget(state, -2);
            lua_rotate(state, -2, -1);
            lua_settop(state, -2);
        }
        lua_pushstring(state, unmakemask(mask, buffer.as_mut_ptr()));
        (*state).push_integer(lua_gethookcount(other_state) as i64);
        3
    }
}
pub unsafe fn db_debug(state: *mut State) -> i32 {
    unsafe {
        loop {
            let mut line = String::new();
            eprint!("lua_debug> ");
            std::io::Write::flush(&mut std::io::stderr()).ok();
            if std::io::BufRead::read_line(&mut std::io::stdin().lock(), &mut line).unwrap_or(0) == 0 || line == "cont\n" {
                return 0;
            }
            if lual_loadbufferx(
                state,
                line.as_ptr() as *const i8,
                line.len(),
                c"=(debug command)".as_ptr(),
                null(),
            ) != Status::OK
                || CallS::api_call(state, 0, 0, 0, 0, None) != Status::OK
            {
                eprintln!("{}", std::ffi::CStr::from_ptr(lual_tolstring(state, -1, null_mut())).display());
            }
            lua_settop(state, 0);
        }
    }
}
pub unsafe fn db_traceback(state: *mut State) -> i32 {
    unsafe {
        let mut arg: i32 = 0;
        let other_state: *mut State = getthread(state, &mut arg);
        let message: *const i8 = lua_tolstring(state, arg + 1, null_mut());
        if message.is_null() && !(TagType::is_none_or_nil(lua_type(state, arg + 1))) {
            lua_pushvalue(state, arg + 1);
        } else {
            let level: i32 = lual_optinteger(state, arg + 2, (if state == other_state { 1 } else { 0 }) as i64) as i32;
            lual_traceback(state, other_state, message, level);
        }
        1
    }
}
pub const DEBUG_FUNCTIONS: [RegisteredFunction; 16] = {
    [
        {
            RegisteredFunction {
                registeredfunction_name: c"debug".as_ptr(),
                registeredfunction_function: Some(db_debug as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"getuservalue".as_ptr(),
                registeredfunction_function: Some(db_getuservalue as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"gethook".as_ptr(),
                registeredfunction_function: Some(db_gethook as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"getinfo".as_ptr(),
                registeredfunction_function: Some(db_getinfo as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"getlocal".as_ptr(),
                registeredfunction_function: Some(db_getlocal as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"getregistry".as_ptr(),
                registeredfunction_function: Some(db_getregistry as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"getmetatable".as_ptr(),
                registeredfunction_function: Some(db_getmetatable as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"getupvalue".as_ptr(),
                registeredfunction_function: Some(db_getupvalue as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"upvaluejoin".as_ptr(),
                registeredfunction_function: Some(db_upvaluejoin as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"upvalueid".as_ptr(),
                registeredfunction_function: Some(db_upvalueid as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"setuservalue".as_ptr(),
                registeredfunction_function: Some(db_setuservalue as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"sethook".as_ptr(),
                registeredfunction_function: Some(db_sethook as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"setlocal".as_ptr(),
                registeredfunction_function: Some(db_setlocal as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"setmetatable".as_ptr(),
                registeredfunction_function: Some(db_setmetatable as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"setupvalue".as_ptr(),
                registeredfunction_function: Some(db_setupvalue as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"traceback".as_ptr(),
                registeredfunction_function: Some(db_traceback as unsafe fn(*mut State) -> i32),
            }
        },
    ]
};
pub unsafe fn luaopen_debug(state: *mut State) -> i32 {
    unsafe {
        (*state).lua_createtable();
        lual_setfuncs(state, DEBUG_FUNCTIONS.as_ptr(), DEBUG_FUNCTIONS.len(), 0);
        1
    }
}
