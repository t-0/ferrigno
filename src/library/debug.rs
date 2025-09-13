#![allow(unpredictable_function_pointer_comparisons)]
use rlua::*;
use crate::character::*;
use crate::debuginfo::*;
use crate::functions::*;
use crate::interpreter::*;
use crate::registeredfunction::*;
use crate::tag::*;
use crate::utility::c::*;
use std::ptr::*;
pub unsafe fn db_getregistry(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        lua_pushvalue(interpreter, -(1000000 as i32) - 1000 as i32);
        return 1;
    }
}
pub unsafe fn db_getmetatable(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        lual_checkany(interpreter, 1);
        if !(*interpreter).lua_getmetatable(1) {
            (*interpreter).push_nil();
        }
        return 1;
    }
}
pub unsafe fn db_setmetatable(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let t = lua_type(interpreter, 2);
        (((t == Some(TagType::Nil) || t == Some(TagType::Table)) as i32 != 0) as i64 != 0
            || lual_typeerror(interpreter, 2, make_cstring!("nil or table")) != 0)
            as i32;
        lua_settop(interpreter, 2);
        lua_setmetatable(interpreter, 1);
        return 1;
    }
}
pub unsafe fn db_getuservalue(interpreter: *mut Interpreter) -> i32 {
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
pub unsafe fn db_setuservalue(interpreter: *mut Interpreter) -> i32 {
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
pub unsafe fn db_getupvalue(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        return auxupvalue(interpreter, 1);
    }
}
pub unsafe fn db_setupvalue(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        lual_checkany(interpreter, 3);
        return auxupvalue(interpreter, 0);
    }
}
pub unsafe fn db_getinfo(interpreter: *mut Interpreter) -> i32 {
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
            make_cstring!("flnSrtu"),
            null_mut(),
        );
        checkstack(interpreter, other_state, 3);
        (((*options.offset(0 as isize) as i32 != CHARACTER_ANGLE_RIGHT as i32) as i32 != 0) as i64
            != 0
            || lual_argerror(
                interpreter,
                arg + 2,
                make_cstring!("invalid option CHARACTER_ANGLE_RIGHT"),
            ) != 0) as i32;
        if lua_type(interpreter, arg + 1) == Some(TagType::Closure) {
            options = lua_pushfstring(interpreter, make_cstring!(">%s"), options);
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
                make_cstring!("invalid option"),
            );
        }
        (*interpreter).lua_createtable();
        if !(strchr(options, CHARACTER_UPPER_S as i32)).is_null() {
            lua_pushlstring(interpreter, ar.source, ar.source_length);
            lua_setfield(interpreter, -2, make_cstring!("source"));
            settabss(
                interpreter,
                make_cstring!("short_src"),
                (ar.short_src).as_mut_ptr(),
            );
            settabsi(
                interpreter,
                make_cstring!("linedefined"),
                ar.line_defined,
            );
            settabsi(
                interpreter,
                make_cstring!("lastlinedefined"),
                ar.last_line_defined,
            );
            settabss(interpreter, make_cstring!("what"), ar.what);
        }
        if !(strchr(options, CHARACTER_LOWER_L as i32)).is_null() {
            settabsi(
                interpreter,
                make_cstring!("currentline"),
                ar.currentline,
            );
        }
        if !(strchr(options, CHARACTER_LOWER_U as i32)).is_null() {
            settabsi(
                interpreter,
                make_cstring!("nups"),
                ar.nups as i32,
            );
            settabsi(
                interpreter,
                make_cstring!("nparams"),
                ar.nparams as i32,
            );
            settabsb(
                interpreter,
                make_cstring!("isvararg"),
                ar.is_variable_arguments as i32,
            );
        }
        if !(strchr(options, CHARACTER_LOWER_N as i32)).is_null() {
            settabss(interpreter, make_cstring!("name"), ar.name);
            settabss(
                interpreter,
                make_cstring!("namewhat"),
                ar.namewhat,
            );
        }
        if !(strchr(options, CHARACTER_LOWER_R as i32)).is_null() {
            settabsi(
                interpreter,
                make_cstring!("ftransfer"),
                ar.ftransfer as i32,
            );
            settabsi(
                interpreter,
                make_cstring!("ntransfer"),
                ar.ntransfer as i32,
            );
        }
        if !(strchr(options, CHARACTER_LOWER_T as i32)).is_null() {
            settabsb(
                interpreter,
                make_cstring!("istailcall"),
                if ar.is_tail_call { 1 } else { 0 },
            );
        }
        if !(strchr(options, CHARACTER_UPPER_L as i32)).is_null() {
            treatstackoption(
                interpreter,
                other_state,
                make_cstring!("activelines"),
            );
        }
        if !(strchr(options, CHARACTER_LOWER_F as i32)).is_null() {
            treatstackoption(
                interpreter,
                other_state,
                make_cstring!("func"),
            );
        }
        return 1;
    }
}
pub unsafe fn db_getlocal(interpreter: *mut Interpreter) -> i32 {
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
                    make_cstring!("level out of range"),
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
pub unsafe fn db_setlocal(interpreter: *mut Interpreter) -> i32 {
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
                make_cstring!("level out of range"),
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
pub unsafe fn db_upvalueid(interpreter: *mut Interpreter) -> i32 {
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
pub unsafe fn db_upvaluejoin(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let mut n1: i32 = 0;
        let mut n2: i32 = 0;
        checkupval(interpreter, 1, 2, &mut n1);
        checkupval(interpreter, 3, 4, &mut n2);
        ((!lua_iscfunction(interpreter, 1))
            || lual_argerror(
                interpreter,
                1,
                make_cstring!("Lua function expected"),
            ) != 0) as i32;
        ((!lua_iscfunction(interpreter, 3))
            || lual_argerror(
                interpreter,
                3,
                make_cstring!("Lua function expected"),
            ) != 0) as i32;
        lua_upvaluejoin(interpreter, 1, n1, 3, n2);
        return 0;
    }
}
pub unsafe fn db_sethook(interpreter: *mut Interpreter) -> i32 {
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
            }
            _ => {
                let smask: *const i8 = lual_checklstring(interpreter, arg + 2, null_mut());
                lual_checktype(interpreter, arg + 1, TagType::Closure);
                count = lual_optinteger(interpreter, arg + 3, 0) as i32;
                function =
                    Some(hookf as unsafe fn(*mut Interpreter, *mut DebugInfo) -> ());
                mask = makemask(smask, count);
            }
        };
        if lual_getsubtable(interpreter, -(1000000 as i32) - 1000 as i32, HOOKKEY) == 0 {
            lua_pushstring(interpreter, make_cstring!("k"));
            lua_setfield(interpreter, -2, make_cstring!("__mode"));
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
pub unsafe fn db_gethook(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let mut arg: i32 = 0;
        let other_state: *mut Interpreter = getthread(interpreter, &mut arg);
        let mut buffer: [i8; 5] = [0; 5];
        let mask: i32 = lua_gethookmask(other_state);
        let hook: HookFunction = lua_gethook(other_state);
        if hook.is_none() {
            (*interpreter).push_nil();
            return 1;
        } else if hook
            != Some(hookf as unsafe fn(*mut Interpreter, *mut DebugInfo) -> ())
        {
            lua_pushstring(interpreter, make_cstring!("external hook"));
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
pub unsafe fn db_debug(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        loop {
            let mut buffer: [i8; 250] = [0; 250];
            fprintf(
                stderr,
                make_cstring!("%s"),
                make_cstring!("lua_debug> "),
            );
            fflush(stderr);
            if (fgets(buffer.as_mut_ptr(), size_of::<[i8; 250]>() as i32, stdin)).is_null()
                || strcmp(buffer.as_mut_ptr(), make_cstring!("cont\n")) == 0
            {
                return 0;
            }
            if lual_loadbufferx(
                interpreter,
                buffer.as_mut_ptr(),
                strlen(buffer.as_mut_ptr()) as usize,
                make_cstring!("=(debug command)"),
                null(),
            ) != 0
                || lua_pcallk(interpreter, 0, 0, 0, 0, None) != 0
            {
                fprintf(
                    stderr,
                    make_cstring!("%s\n"),
                    lual_tolstring(interpreter, -1, null_mut()),
                );
                fflush(stderr);
            }
            lua_settop(interpreter, 0);
        }
    }
}
pub unsafe fn db_traceback(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let mut arg: i32 = 0;
        let other_state: *mut Interpreter = getthread(interpreter, &mut arg);
        let message: *const i8 = lua_tolstring(interpreter, arg + 1, null_mut());
        if message.is_null() && !(is_none_or_nil(lua_type(interpreter, arg + 1))) {
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
                name: make_cstring!("debug"),
                function: Some(db_debug as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: make_cstring!("getuservalue"),
                function: Some(db_getuservalue as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: make_cstring!("gethook"),
                function: Some(db_gethook as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: make_cstring!("getinfo"),
                function: Some(db_getinfo as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: make_cstring!("getlocal"),
                function: Some(db_getlocal as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: make_cstring!("getregistry"),
                function: Some(db_getregistry as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: make_cstring!("getmetatable"),
                function: Some(db_getmetatable as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: make_cstring!("getupvalue"),
                function: Some(db_getupvalue as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: make_cstring!("upvaluejoin"),
                function: Some(db_upvaluejoin as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: make_cstring!("upvalueid"),
                function: Some(db_upvalueid as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: make_cstring!("setuservalue"),
                function: Some(db_setuservalue as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: make_cstring!("sethook"),
                function: Some(db_sethook as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: make_cstring!("setlocal"),
                function: Some(db_setlocal as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: make_cstring!("setmetatable"),
                function: Some(db_setmetatable as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: make_cstring!("setupvalue"),
                function: Some(db_setupvalue as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: make_cstring!("traceback"),
                function: Some(db_traceback as unsafe fn(*mut Interpreter) -> i32),
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
pub unsafe fn luaopen_debug(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        lual_checkversion_(
            interpreter,
            504.0,
            (size_of::<i64>() as usize)
                .wrapping_mul(16 as usize)
                .wrapping_add(size_of::<f64>() as usize),
        );
        (*interpreter).lua_createtable();
        lual_setfuncs(interpreter, DEBUG_FUNCTIONS.as_ptr(), 0);
        return 1;
    }
}
