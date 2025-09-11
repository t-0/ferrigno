#![allow(unpredictable_function_pointer_comparisons)]
use std::ptr::*;
use crate::utility::c::*;
use crate::character::*;
use crate::interpreter::*;
use crate::functions::*;
use crate::debuginfo::*;
use crate::tag::*;
use crate::registeredfunction::*;
pub unsafe extern "C" fn db_getregistry(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        lua_pushvalue(interpreter, -(1000000 as i32) - 1000 as i32);
        return 1;
    }
}
pub unsafe extern "C" fn db_getmetatable(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        lual_checkany(interpreter, 1);
        if !(*interpreter).lua_getmetatable(1) {
            (*interpreter).push_nil();
        }
        return 1;
    }
}
pub unsafe extern "C" fn db_setmetatable(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let t = lua_type(interpreter, 2);
        (((t == Some(TagType::Nil) || t == Some(TagType::Table)) as i32 != 0) as i64 != 0
            || lual_typeerror(interpreter, 2, b"nil or table\0" as *const u8 as *const i8) != 0)
            as i32;
        lua_settop(interpreter, 2);
        lua_setmetatable(interpreter, 1);
        return 1;
    }
}
pub unsafe extern "C" fn db_getuservalue(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let n: i32 = lual_optinteger(interpreter, 2, 1) as i32;
        if lua_type(interpreter, 1) != Some(TagType::User) {
            (*interpreter).push_nil();
        } else if (*interpreter).lua_getiuservalue(1, n) != None {
            (*interpreter).push_boolean(true);
            return 2;
        }
        return 1;
    }
}
pub unsafe extern "C" fn db_setuservalue(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let n: i32 = lual_optinteger(interpreter, 3, 1) as i32;
        lual_checktype(interpreter, 1, TagType::User);
        lual_checkany(interpreter, 2);
        lua_settop(interpreter, 2);
        if lua_setiuservalue(interpreter, 1, n) == 0 {
            (*interpreter).push_nil();
        }
        return 1;
    }
}
pub unsafe extern "C" fn db_getupvalue(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        return auxupvalue(interpreter, 1);
    }
}
pub unsafe extern "C" fn db_setupvalue(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        lual_checkany(interpreter, 3);
        return auxupvalue(interpreter, 0);
    }
}
pub unsafe extern "C" fn db_getinfo(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let mut ar: DebugInfo = DebugInfo {
            event: 0,
            name: null(),
            namewhat: null(),
            what: null(),
            source: null(),
            source_length: 0,
            currentline: 0,
            line_defined: 0,
            last_line_defined: 0,
            nups: 0,
            nparams: 0,
            is_variable_arguments: false,
            is_tail_call: false,
            ftransfer: 0,
            ntransfer: 0,
            short_src: [0; 60],
            i_ci: null_mut(),
        };
        let mut arg: i32 = 0;
        let other_state: *mut Interpreter = getthread(interpreter, &mut arg);
        let mut options: *const i8 = lual_optlstring(
            interpreter,
            arg + 2,
            b"flnSrtu\0" as *const u8 as *const i8,
            null_mut(),
        );
        checkstack(interpreter, other_state, 3);
        (((*options.offset(0 as isize) as i32 != CHARACTER_ANGLE_RIGHT as i32) as i32 != 0) as i64 != 0
            || lual_argerror(
                interpreter,
                arg + 2,
                b"invalid option CHARACTER_ANGLE_RIGHT\0" as *const u8 as *const i8,
            ) != 0) as i32;
        if lua_type(interpreter, arg + 1) == Some(TagType::Closure) {
            options = lua_pushfstring(interpreter, b">%s\0" as *const u8 as *const i8, options);
            lua_pushvalue(interpreter, arg + 1);
            lua_xmove(interpreter, other_state, 1);
        } else if lua_getstack(
            other_state,
            lual_checkinteger(interpreter, arg + 1) as i32,
            &mut ar,
        ) == 0
        {
            (*interpreter).push_nil();
            return 1;
        }
        if lua_getinfo(other_state, options, &mut ar) == 0 {
            return lual_argerror(
                interpreter,
                arg + 2,
                b"invalid option\0" as *const u8 as *const i8,
            );
        }
        (*interpreter).lua_createtable();
        if !(strchr(options, CHARACTER_UPPER_S as i32)).is_null() {
            lua_pushlstring(interpreter, ar.source, ar.source_length);
            lua_setfield(interpreter, -2, b"source\0" as *const u8 as *const i8);
            settabss(
                interpreter,
                b"short_src\0" as *const u8 as *const i8,
                (ar.short_src).as_mut_ptr(),
            );
            settabsi(
                interpreter,
                b"linedefined\0" as *const u8 as *const i8,
                ar.line_defined,
            );
            settabsi(
                interpreter,
                b"lastlinedefined\0" as *const u8 as *const i8,
                ar.last_line_defined,
            );
            settabss(interpreter, b"what\0" as *const u8 as *const i8, ar.what);
        }
        if !(strchr(options, CHARACTER_LOWER_L as i32)).is_null() {
            settabsi(
                interpreter,
                b"currentline\0" as *const u8 as *const i8,
                ar.currentline,
            );
        }
        if !(strchr(options, CHARACTER_LOWER_U as i32)).is_null() {
            settabsi(interpreter, b"nups\0" as *const u8 as *const i8, ar.nups as i32);
            settabsi(
                interpreter,
                b"nparams\0" as *const u8 as *const i8,
                ar.nparams as i32,
            );
            settabsb(
                interpreter,
                b"isvararg\0" as *const u8 as *const i8,
                ar.is_variable_arguments as i32,
            );
        }
        if !(strchr(options, CHARACTER_LOWER_N as i32)).is_null() {
            settabss(interpreter, b"name\0" as *const u8 as *const i8, ar.name);
            settabss(interpreter, b"namewhat\0" as *const u8 as *const i8, ar.namewhat);
        }
        if !(strchr(options, CHARACTER_LOWER_R as i32)).is_null() {
            settabsi(
                interpreter,
                b"ftransfer\0" as *const u8 as *const i8,
                ar.ftransfer as i32,
            );
            settabsi(
                interpreter,
                b"ntransfer\0" as *const u8 as *const i8,
                ar.ntransfer as i32,
            );
        }
        if !(strchr(options, CHARACTER_LOWER_T as i32)).is_null() {
            settabsb(
                interpreter,
                b"istailcall\0" as *const u8 as *const i8,
                if ar.is_tail_call { 1 } else { 0 },
            );
        }
        if !(strchr(options, CHARACTER_UPPER_L as i32)).is_null() {
            treatstackoption(
                interpreter,
                other_state,
                b"activelines\0" as *const u8 as *const i8,
            );
        }
        if !(strchr(options, CHARACTER_LOWER_F as i32)).is_null() {
            treatstackoption(interpreter, other_state, b"func\0" as *const u8 as *const i8);
        }
        return 1;
    }
}
pub unsafe extern "C" fn db_getlocal(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let mut arg: i32 = 0;
        let other_state: *mut Interpreter = getthread(interpreter, &mut arg);
        let nvar: i32 = lual_checkinteger(interpreter, arg + 2) as i32;
        if lua_type(interpreter, arg + 1) == Some(TagType::Closure) {
            lua_pushvalue(interpreter, arg + 1);
            lua_pushstring(interpreter, lua_getlocal(interpreter, null(), nvar));
            return 1;
        } else {
            let mut ar: DebugInfo = DebugInfo {
                event: 0,
                name: null(),
                namewhat: null(),
                what: null(),
                source: null(),
                source_length: 0,
                currentline: 0,
                line_defined: 0,
                last_line_defined: 0,
                nups: 0,
                nparams: 0,
                is_variable_arguments: false,
                is_tail_call: false,
                ftransfer: 0,
                ntransfer: 0,
                short_src: [0; 60],
                i_ci: null_mut(),
            };
            let level: i32 = lual_checkinteger(interpreter, arg + 1) as i32;
            if lua_getstack(other_state, level, &mut ar) == 0 {
                return lual_argerror(
                    interpreter,
                    arg + 1,
                    b"level out of range\0" as *const u8 as *const i8,
                );
            }
            checkstack(interpreter, other_state, 1);
            let name: *const i8 = lua_getlocal(other_state, &mut ar, nvar);
            if !name.is_null() {
                lua_xmove(other_state, interpreter, 1);
                lua_pushstring(interpreter, name);
                lua_rotate(interpreter, -2, 1);
                return 2;
            } else {
                (*interpreter).push_nil();
                return 1;
            }
        };
    }
}
pub unsafe extern "C" fn db_setlocal(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let mut arg: i32 = 0;
        let name: *const i8;
        let other_state: *mut Interpreter = getthread(interpreter, &mut arg);
        let mut ar: DebugInfo = DebugInfo {
            event: 0,
            name: null(),
            namewhat: null(),
            what: null(),
            source: null(),
            source_length: 0,
            currentline: 0,
            line_defined: 0,
            last_line_defined: 0,
            nups: 0,
            nparams: 0,
            is_variable_arguments: false,
            is_tail_call: false,
            ftransfer: 0,
            ntransfer: 0,
            short_src: [0; 60],
            i_ci: null_mut(),
        };
        let level: i32 = lual_checkinteger(interpreter, arg + 1) as i32;
        let nvar: i32 = lual_checkinteger(interpreter, arg + 2) as i32;
        if lua_getstack(other_state, level, &mut ar) == 0 {
            return lual_argerror(
                interpreter,
                arg + 1,
                b"level out of range\0" as *const u8 as *const i8,
            );
        }
        lual_checkany(interpreter, arg + 3);
        lua_settop(interpreter, arg + 3);
        checkstack(interpreter, other_state, 1);
        lua_xmove(interpreter, other_state, 1);
        name = lua_setlocal(other_state, &mut ar, nvar);
        if name.is_null() {
            lua_settop(other_state, -2);
        }
        lua_pushstring(interpreter, name);
        return 1;
    }
}
pub unsafe extern "C" fn db_upvalueid(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let id: *mut libc::c_void = checkupval(interpreter, 1, 2, null_mut());
        if !id.is_null() {
            lua_pushlightuserdata(interpreter, id);
        } else {
            (*interpreter).push_nil();
        }
        return 1;
    }
}
pub unsafe extern "C" fn db_upvaluejoin(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let mut n1: i32 = 0;
        let mut n2: i32 = 0;
        checkupval(interpreter, 1, 2, &mut n1);
        checkupval(interpreter, 3, 4, &mut n2);
        ((!lua_iscfunction(interpreter, 1))
            || lual_argerror(
                interpreter,
                1,
                b"Lua function expected\0" as *const u8 as *const i8,
            ) != 0) as i32;
        ((!lua_iscfunction(interpreter, 3))
            || lual_argerror(
                interpreter,
                3,
                b"Lua function expected\0" as *const u8 as *const i8,
            ) != 0) as i32;
        lua_upvaluejoin(interpreter, 1, n1, 3, n2);
        return 0;
    }
}
pub unsafe extern "C" fn db_sethook(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let mut arg: i32 = 0;
        let mask: i32;
        let count: i32;
        let function: HookFunction;
        let other_state: *mut Interpreter = getthread(interpreter, &mut arg);
        match lua_type(interpreter, arg + 1) {
            None | Some(TagType::Nil) => {
                lua_settop(interpreter, arg + 1);
                function = None;
                mask = 0;
                count = 0;
            },
            _ => {
                let smask: *const i8 = lual_checklstring(interpreter, arg + 2, null_mut());
                lual_checktype(interpreter, arg + 1, TagType::Closure);
                count = lual_optinteger(interpreter, arg + 3, 0) as i32;
                function = Some(hookf as unsafe extern "C" fn(*mut Interpreter, *mut DebugInfo) -> ());
                mask = makemask(smask, count);
            },
        };
        if lual_getsubtable(interpreter, -(1000000 as i32) - 1000 as i32, HOOKKEY) == 0 {
            lua_pushstring(interpreter, b"k\0" as *const u8 as *const i8);
            lua_setfield(interpreter, -2, b"__mode\0" as *const u8 as *const i8);
            lua_pushvalue(interpreter, -1);
            lua_setmetatable(interpreter, -2);
        }
        checkstack(interpreter, other_state, 1);
        (*other_state).push_state();
        lua_xmove(other_state, interpreter, 1);
        lua_pushvalue(interpreter, arg + 1);
        lua_rawset(interpreter, -3);
        lua_sethook(other_state, function, mask, count);
        return 0;
    }
}
pub unsafe extern "C" fn db_gethook(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let mut arg: i32 = 0;
        let other_state: *mut Interpreter = getthread(interpreter, &mut arg);
        let mut buffer: [i8; 5] = [0; 5];
        let mask: i32 = lua_gethookmask(other_state);
        let hook: HookFunction = lua_gethook(other_state);
        if hook.is_none() {
            (*interpreter).push_nil();
            return 1;
        } else if hook != Some(hookf as unsafe extern "C" fn(*mut Interpreter, *mut DebugInfo) -> ()) {
            lua_pushstring(interpreter, b"external hook\0" as *const u8 as *const i8);
        } else {
            lua_getfield(interpreter, -(1000000 as i32) - 1000 as i32, HOOKKEY);
            checkstack(interpreter, other_state, 1);
            (*other_state).push_state();
            lua_xmove(other_state, interpreter, 1);
            lua_rawget(interpreter, -2);
            lua_rotate(interpreter, -2, -1);
            lua_settop(interpreter, -2);
        }
        lua_pushstring(interpreter, unmakemask(mask, buffer.as_mut_ptr()));
        (*interpreter).push_integer(lua_gethookcount(other_state) as i64);
        return 3;
    }
}
pub unsafe extern "C" fn db_debug(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        loop {
            let mut buffer: [i8; 250] = [0; 250];
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const i8,
                b"lua_debug> \0" as *const u8 as *const i8,
            );
            fflush(stderr);
            if (fgets(
                buffer.as_mut_ptr(),
                ::core::mem::size_of::<[i8; 250]>() as i32,
                stdin,
            ))
            .is_null()
                || strcmp(buffer.as_mut_ptr(), b"cont\n\0" as *const u8 as *const i8) == 0
            {
                return 0;
            }
            if lual_loadbufferx(
                interpreter,
                buffer.as_mut_ptr(),
                strlen(buffer.as_mut_ptr()),
                b"=(debug command)\0" as *const u8 as *const i8,
                null(),
            ) != 0
                || lua_pcallk(interpreter, 0, 0, 0, 0, None) != 0
            {
                fprintf(
                    stderr,
                    b"%s\n\0" as *const u8 as *const i8,
                    lual_tolstring(interpreter, -1, null_mut()),
                );
                fflush(stderr);
            }
            lua_settop(interpreter, 0);
        }
    }
}
pub unsafe extern "C" fn db_traceback(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let mut arg: i32 = 0;
        let other_state: *mut Interpreter = getthread(interpreter, &mut arg);
        let message: *const i8 = lua_tolstring(interpreter, arg + 1, null_mut());
        if message.is_null() && !(is_none_or_nil (lua_type(interpreter, arg + 1))) {
            lua_pushvalue(interpreter, arg + 1);
        } else {
            let level: i32 = lual_optinteger(
                interpreter,
                arg + 2,
                (if interpreter == other_state { 1 } else { 0 }) as i64,
            ) as i32;
            lual_traceback(interpreter, other_state, message, level);
        }
        return 1;
    }
}
pub const DEBUG_FUNCTIONS: [RegisteredFunction; 17] = {
    [
        {
            RegisteredFunction {
                name: b"debug\0" as *const u8 as *const i8,
                function: Some(db_debug as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"getuservalue\0" as *const u8 as *const i8,
                function: Some(db_getuservalue as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"gethook\0" as *const u8 as *const i8,
                function: Some(db_gethook as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"getinfo\0" as *const u8 as *const i8,
                function: Some(db_getinfo as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"getlocal\0" as *const u8 as *const i8,
                function: Some(db_getlocal as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"getregistry\0" as *const u8 as *const i8,
                function: Some(db_getregistry as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"getmetatable\0" as *const u8 as *const i8,
                function: Some(db_getmetatable as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"getupvalue\0" as *const u8 as *const i8,
                function: Some(db_getupvalue as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"upvaluejoin\0" as *const u8 as *const i8,
                function: Some(db_upvaluejoin as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"upvalueid\0" as *const u8 as *const i8,
                function: Some(db_upvalueid as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"setuservalue\0" as *const u8 as *const i8,
                function: Some(db_setuservalue as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"sethook\0" as *const u8 as *const i8,
                function: Some(db_sethook as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"setlocal\0" as *const u8 as *const i8,
                function: Some(db_setlocal as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"setmetatable\0" as *const u8 as *const i8,
                function: Some(db_setmetatable as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"setupvalue\0" as *const u8 as *const i8,
                function: Some(db_setupvalue as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"traceback\0" as *const u8 as *const i8,
                function: Some(db_traceback as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: null(),
                function: None,
            }
        },
    ]
};
pub unsafe extern "C" fn luaopen_debug(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        lual_checkversion_(
            interpreter,
            504.0,
            (::core::mem::size_of::<i64>() as u64)
                .wrapping_mul(16 as u64)
                .wrapping_add(::core::mem::size_of::<f64>() as u64),
        );
        (*interpreter).lua_createtable();
        lual_setfuncs(interpreter, DEBUG_FUNCTIONS.as_ptr(), 0);
        return 1;
    }
}
