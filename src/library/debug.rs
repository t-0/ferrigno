#![allow(
    unpredictable_function_pointer_comparisons,
)]
use crate::utility::c::*;
use crate::state::*;
use crate::functions::*;
use crate::debuginfo::*;
use crate::tag::*;
use crate::registeredfunction::*;
pub unsafe extern "C" fn db_getregistry(state: *mut State) -> i32 {
    unsafe {
        lua_pushvalue(state, -(1000000 as i32) - 1000 as i32);
        return 1;
    }
}
pub unsafe extern "C" fn db_getmetatable(state: *mut State) -> i32 {
    unsafe {
        lual_checkany(state, 1);
        if !(*state).lua_getmetatable(1) {
            (*state).push_nil();
        }
        return 1;
    }
}
pub unsafe extern "C" fn db_setmetatable(state: *mut State) -> i32 {
    unsafe {
        let t = lua_type(state, 2);
        (((t == Some(TAG_TYPE_NIL) || t == Some(TAG_TYPE_TABLE)) as i32 != 0) as i64 != 0
            || lual_typeerror(state, 2, b"nil or table\0" as *const u8 as *const i8) != 0)
            as i32;
        lua_settop(state, 2);
        lua_setmetatable(state, 1);
        return 1;
    }
}
pub unsafe extern "C" fn db_getuservalue(state: *mut State) -> i32 {
    unsafe {
        let n: i32 = lual_optinteger(state, 2, 1) as i32;
        if lua_type(state, 1) != Some(TAG_TYPE_USER) {
            (*state).push_nil();
        } else if (*state).lua_getiuservalue(1, n) != -1 {
            (*state).push_boolean(true);
            return 2;
        }
        return 1;
    }
}
pub unsafe extern "C" fn db_setuservalue(state: *mut State) -> i32 {
    unsafe {
        let n: i32 = lual_optinteger(state, 3, 1) as i32;
        lual_checktype(state, 1, TAG_TYPE_USER);
        lual_checkany(state, 2);
        lua_settop(state, 2);
        if lua_setiuservalue(state, 1, n) == 0 {
            (*state).push_nil();
        }
        return 1;
    }
}
pub unsafe extern "C" fn db_getupvalue(state: *mut State) -> i32 {
    unsafe {
        return auxupvalue(state, 1);
    }
}
pub unsafe extern "C" fn db_setupvalue(state: *mut State) -> i32 {
    unsafe {
        lual_checkany(state, 3);
        return auxupvalue(state, 0);
    }
}
pub unsafe extern "C" fn db_getinfo(state: *mut State) -> i32 {
    unsafe {
        let mut ar: DebugInfo = DebugInfo {
            event: 0,
            name: std::ptr::null(),
            namewhat: std::ptr::null(),
            what: std::ptr::null(),
            source: std::ptr::null(),
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
            i_ci: std::ptr::null_mut(),
        };
        let mut arg: i32 = 0;
        let other_state: *mut State = getthread(state, &mut arg);
        let mut options: *const i8 = lual_optlstring(
            state,
            arg + 2,
            b"flnSrtu\0" as *const u8 as *const i8,
            std::ptr::null_mut(),
        );
        checkstack(state, other_state, 3);
        (((*options.offset(0 as isize) as i32 != '>' as i32) as i32 != 0) as i64 != 0
            || lual_argerror(
                state,
                arg + 2,
                b"invalid option '>'\0" as *const u8 as *const i8,
            ) != 0) as i32;
        if lua_type(state, arg + 1) == Some(TAG_TYPE_CLOSURE) {
            options = lua_pushfstring(state, b">%s\0" as *const u8 as *const i8, options);
            lua_pushvalue(state, arg + 1);
            lua_xmove(state, other_state, 1);
        } else if lua_getstack(
            other_state,
            lual_checkinteger(state, arg + 1) as i32,
            &mut ar,
        ) == 0
        {
            (*state).push_nil();
            return 1;
        }
        if lua_getinfo(other_state, options, &mut ar) == 0 {
            return lual_argerror(
                state,
                arg + 2,
                b"invalid option\0" as *const u8 as *const i8,
            );
        }
        (*state).lua_createtable();
        if !(strchr(options, 'S' as i32)).is_null() {
            lua_pushlstring(state, ar.source, ar.source_length);
            lua_setfield(state, -2, b"source\0" as *const u8 as *const i8);
            settabss(
                state,
                b"short_src\0" as *const u8 as *const i8,
                (ar.short_src).as_mut_ptr(),
            );
            settabsi(
                state,
                b"linedefined\0" as *const u8 as *const i8,
                ar.line_defined,
            );
            settabsi(
                state,
                b"lastlinedefined\0" as *const u8 as *const i8,
                ar.last_line_defined,
            );
            settabss(state, b"what\0" as *const u8 as *const i8, ar.what);
        }
        if !(strchr(options, 'l' as i32)).is_null() {
            settabsi(
                state,
                b"currentline\0" as *const u8 as *const i8,
                ar.currentline,
            );
        }
        if !(strchr(options, 'u' as i32)).is_null() {
            settabsi(state, b"nups\0" as *const u8 as *const i8, ar.nups as i32);
            settabsi(
                state,
                b"nparams\0" as *const u8 as *const i8,
                ar.nparams as i32,
            );
            settabsb(
                state,
                b"isvararg\0" as *const u8 as *const i8,
                ar.is_variable_arguments as i32,
            );
        }
        if !(strchr(options, 'n' as i32)).is_null() {
            settabss(state, b"name\0" as *const u8 as *const i8, ar.name);
            settabss(state, b"namewhat\0" as *const u8 as *const i8, ar.namewhat);
        }
        if !(strchr(options, 'r' as i32)).is_null() {
            settabsi(
                state,
                b"ftransfer\0" as *const u8 as *const i8,
                ar.ftransfer as i32,
            );
            settabsi(
                state,
                b"ntransfer\0" as *const u8 as *const i8,
                ar.ntransfer as i32,
            );
        }
        if !(strchr(options, 't' as i32)).is_null() {
            settabsb(
                state,
                b"istailcall\0" as *const u8 as *const i8,
                if ar.is_tail_call { 1 } else { 0 },
            );
        }
        if !(strchr(options, 'L' as i32)).is_null() {
            treatstackoption(
                state,
                other_state,
                b"activelines\0" as *const u8 as *const i8,
            );
        }
        if !(strchr(options, 'f' as i32)).is_null() {
            treatstackoption(state, other_state, b"func\0" as *const u8 as *const i8);
        }
        return 1;
    }
}
pub unsafe extern "C" fn db_getlocal(state: *mut State) -> i32 {
    unsafe {
        let mut arg: i32 = 0;
        let other_state: *mut State = getthread(state, &mut arg);
        let nvar: i32 = lual_checkinteger(state, arg + 2) as i32;
        if lua_type(state, arg + 1) == Some(TAG_TYPE_CLOSURE) {
            lua_pushvalue(state, arg + 1);
            lua_pushstring(state, lua_getlocal(state, std::ptr::null(), nvar));
            return 1;
        } else {
            let mut ar: DebugInfo = DebugInfo {
                event: 0,
                name: std::ptr::null(),
                namewhat: std::ptr::null(),
                what: std::ptr::null(),
                source: std::ptr::null(),
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
                i_ci: std::ptr::null_mut(),
            };
            let level: i32 = lual_checkinteger(state, arg + 1) as i32;
            if ((lua_getstack(other_state, level, &mut ar) == 0) as i32 != 0) as i64 != 0 {
                return lual_argerror(
                    state,
                    arg + 1,
                    b"level out of range\0" as *const u8 as *const i8,
                );
            }
            checkstack(state, other_state, 1);
            let name: *const i8 = lua_getlocal(other_state, &mut ar, nvar);
            if !name.is_null() {
                lua_xmove(other_state, state, 1);
                lua_pushstring(state, name);
                lua_rotate(state, -2, 1);
                return 2;
            } else {
                (*state).push_nil();
                return 1;
            }
        };
    }
}
pub unsafe extern "C" fn db_setlocal(state: *mut State) -> i32 {
    unsafe {
        let mut arg: i32 = 0;
        let name: *const i8;
        let other_state: *mut State = getthread(state, &mut arg);
        let mut ar: DebugInfo = DebugInfo {
            event: 0,
            name: std::ptr::null(),
            namewhat: std::ptr::null(),
            what: std::ptr::null(),
            source: std::ptr::null(),
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
            i_ci: std::ptr::null_mut(),
        };
        let level: i32 = lual_checkinteger(state, arg + 1) as i32;
        let nvar: i32 = lual_checkinteger(state, arg + 2) as i32;
        if ((lua_getstack(other_state, level, &mut ar) == 0) as i32 != 0) as i64 != 0 {
            return lual_argerror(
                state,
                arg + 1,
                b"level out of range\0" as *const u8 as *const i8,
            );
        }
        lual_checkany(state, arg + 3);
        lua_settop(state, arg + 3);
        checkstack(state, other_state, 1);
        lua_xmove(state, other_state, 1);
        name = lua_setlocal(other_state, &mut ar, nvar);
        if name.is_null() {
            lua_settop(other_state, -2);
        }
        lua_pushstring(state, name);
        return 1;
    }
}
pub unsafe extern "C" fn db_upvalueid(state: *mut State) -> i32 {
    unsafe {
        let id: *mut libc::c_void = checkupval(state, 1, 2, std::ptr::null_mut());
        if !id.is_null() {
            lua_pushlightuserdata(state, id);
        } else {
            (*state).push_nil();
        }
        return 1;
    }
}
pub unsafe extern "C" fn db_upvaluejoin(state: *mut State) -> i32 {
    unsafe {
        let mut n1: i32 = 0;
        let mut n2: i32 = 0;
        checkupval(state, 1, 2, &mut n1);
        checkupval(state, 3, 4, &mut n2);
        ((!lua_iscfunction(state, 1))
            || lual_argerror(
                state,
                1,
                b"Lua function expected\0" as *const u8 as *const i8,
            ) != 0) as i32;
        ((!lua_iscfunction(state, 3))
            || lual_argerror(
                state,
                3,
                b"Lua function expected\0" as *const u8 as *const i8,
            ) != 0) as i32;
        lua_upvaluejoin(state, 1, n1, 3, n2);
        return 0;
    }
}
pub unsafe extern "C" fn db_sethook(state: *mut State) -> i32 {
    unsafe {
        let mut arg: i32 = 0;
        let mask: i32;
        let count: i32;
        let function: HookFunction;
        let other_state: *mut State = getthread(state, &mut arg);
        match lua_type(state, arg + 1) {
            None | Some(TAG_TYPE_NIL) => {
                lua_settop(state, arg + 1);
                function = None;
                mask = 0;
                count = 0;
            },
            _ => {
                let smask: *const i8 = lual_checklstring(state, arg + 2, std::ptr::null_mut());
                lual_checktype(state, arg + 1, TAG_TYPE_CLOSURE);
                count = lual_optinteger(state, arg + 3, 0) as i32;
                function = Some(hookf as unsafe extern "C" fn(*mut State, *mut DebugInfo) -> ());
                mask = makemask(smask, count);
            },
        };
        if lual_getsubtable(state, -(1000000 as i32) - 1000 as i32, HOOKKEY) == 0 {
            lua_pushstring(state, b"k\0" as *const u8 as *const i8);
            lua_setfield(state, -2, b"__mode\0" as *const u8 as *const i8);
            lua_pushvalue(state, -1);
            lua_setmetatable(state, -2);
        }
        checkstack(state, other_state, 1);
        (*other_state).push_state();
        lua_xmove(other_state, state, 1);
        lua_pushvalue(state, arg + 1);
        lua_rawset(state, -3);
        lua_sethook(other_state, function, mask, count);
        return 0;
    }
}
pub unsafe extern "C" fn db_gethook(state: *mut State) -> i32 {
    unsafe {
        let mut arg: i32 = 0;
        let other_state: *mut State = getthread(state, &mut arg);
        let mut buffer: [i8; 5] = [0; 5];
        let mask: i32 = lua_gethookmask(other_state);
        let hook: HookFunction = lua_gethook(other_state);
        if hook.is_none() {
            (*state).push_nil();
            return 1;
        } else if hook != Some(hookf as unsafe extern "C" fn(*mut State, *mut DebugInfo) -> ()) {
            lua_pushstring(state, b"external hook\0" as *const u8 as *const i8);
        } else {
            lua_getfield(state, -(1000000 as i32) - 1000 as i32, HOOKKEY);
            checkstack(state, other_state, 1);
            (*other_state).push_state();
            lua_xmove(other_state, state, 1);
            lua_rawget(state, -2);
            lua_rotate(state, -2, -1);
            lua_settop(state, -2);
        }
        lua_pushstring(state, unmakemask(mask, buffer.as_mut_ptr()));
        (*state).push_integer(lua_gethookcount(other_state) as i64);
        return 3;
    }
}
pub unsafe extern "C" fn db_debug(state: *mut State) -> i32 {
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
                state,
                buffer.as_mut_ptr(),
                strlen(buffer.as_mut_ptr()),
                b"=(debug command)\0" as *const u8 as *const i8,
                std::ptr::null(),
            ) != 0
                || lua_pcallk(state, 0, 0, 0, 0, None) != 0
            {
                fprintf(
                    stderr,
                    b"%s\n\0" as *const u8 as *const i8,
                    lual_tolstring(state, -1, std::ptr::null_mut()),
                );
                fflush(stderr);
            }
            lua_settop(state, 0);
        }
    }
}
pub unsafe extern "C" fn db_traceback(state: *mut State) -> i32 {
    unsafe {
        let mut arg: i32 = 0;
        let other_state: *mut State = getthread(state, &mut arg);
        let message: *const i8 = lua_tolstring(state, arg + 1, std::ptr::null_mut());
        if message.is_null() && !(is_none_or_nil (lua_type(state, arg + 1))) {
            lua_pushvalue(state, arg + 1);
        } else {
            let level: i32 = lual_optinteger(
                state,
                arg + 2,
                (if state == other_state { 1 } else { 0 }) as i64,
            ) as i32;
            lual_traceback(state, other_state, message, level);
        }
        return 1;
    }
}
pub const DEBUG_FUNCTIONS: [RegisteredFunction; 17] = {
    [
        {
            RegisteredFunction {
                name: b"debug\0" as *const u8 as *const i8,
                function: Some(db_debug as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"getuservalue\0" as *const u8 as *const i8,
                function: Some(db_getuservalue as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"gethook\0" as *const u8 as *const i8,
                function: Some(db_gethook as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"getinfo\0" as *const u8 as *const i8,
                function: Some(db_getinfo as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"getlocal\0" as *const u8 as *const i8,
                function: Some(db_getlocal as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"getregistry\0" as *const u8 as *const i8,
                function: Some(db_getregistry as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"getmetatable\0" as *const u8 as *const i8,
                function: Some(db_getmetatable as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"getupvalue\0" as *const u8 as *const i8,
                function: Some(db_getupvalue as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"upvaluejoin\0" as *const u8 as *const i8,
                function: Some(db_upvaluejoin as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"upvalueid\0" as *const u8 as *const i8,
                function: Some(db_upvalueid as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"setuservalue\0" as *const u8 as *const i8,
                function: Some(db_setuservalue as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"sethook\0" as *const u8 as *const i8,
                function: Some(db_sethook as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"setlocal\0" as *const u8 as *const i8,
                function: Some(db_setlocal as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"setmetatable\0" as *const u8 as *const i8,
                function: Some(db_setmetatable as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"setupvalue\0" as *const u8 as *const i8,
                function: Some(db_setupvalue as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"traceback\0" as *const u8 as *const i8,
                function: Some(db_traceback as unsafe extern "C" fn(*mut State) -> i32),
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
pub unsafe extern "C" fn luaopen_debug(state: *mut State) -> i32 {
    unsafe {
        lual_checkversion_(
            state,
            504.0,
            (::core::mem::size_of::<i64>() as u64)
                .wrapping_mul(16 as u64)
                .wrapping_add(::core::mem::size_of::<f64>() as u64),
        );
        (*state).lua_createtable();
        lual_setfuncs(state, DEBUG_FUNCTIONS.as_ptr(), 0);
        return 1;
    }
}
