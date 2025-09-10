use crate::coroutine::*;
use crate::tag::*;
use crate::registeredfunction::*;
use crate::interpreter::*;
unsafe extern "C" fn luab_cocreate(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        lual_checktype(interpreter, 1, TagType::Closure);
        let nl: *mut Interpreter = lua_newthread(interpreter);
        lua_pushvalue(interpreter, 1);
        lua_xmove(interpreter, nl, 1);
        return 1;
    }
}
unsafe extern "C" fn luab_cowrap(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        luab_cocreate(interpreter);
        lua_pushcclosure(
            interpreter,
            Some(luab_auxwrap as unsafe extern "C" fn(*mut Interpreter) -> i32),
            1,
        );
        return 1;
    }
}
unsafe extern "C" fn luab_coresume(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let co: *mut Interpreter = getco(interpreter);
        let r: i32 = auxresume(interpreter, co, (*interpreter).get_top() - 1);
        if r < 0 {
            (*interpreter).push_boolean(false);
            lua_rotate(interpreter, -2, 1);
            return 2;
        } else {
            (*interpreter).push_boolean(true);
            lua_rotate(interpreter, -(r + 1), 1);
            return r + 1;
        };
    }
}
unsafe extern "C" fn luab_corunning(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        (*interpreter).push_boolean((*interpreter).push_state());
        return 2;
    }
}
unsafe extern "C" fn luab_close(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let co: *mut Interpreter = getco(interpreter);
        let mut status: i32 = auxstatus(interpreter, co);
        match status {
            1 | 2 => {
                status = lua_closethread(co, interpreter);
                if status == 0 {
                    (*interpreter).push_boolean(true);
                    return 1;
                } else {
                    (*interpreter).push_boolean(false);
                    lua_xmove(co, interpreter, 1);
                    return 2;
                }
            }
            _ => {
                return lual_error(
                    interpreter,
                    b"cannot close a %s coroutine\0".as_ptr(),
                    COROUTINE_STATUS_NAMES[status as usize],
                );
            }
        };
    }
}
unsafe extern "C" fn luab_costatus(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let co: *mut Interpreter = getco(interpreter);
        lua_pushstring(interpreter, COROUTINE_STATUS_NAMES[auxstatus(interpreter, co) as usize]);
        return 1;
    }
}
unsafe extern "C" fn luab_yieldable(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let coroutine: *mut Interpreter = if lua_type(interpreter, 1) == None {
            interpreter
        } else {
            getco(interpreter)
        };
        (*interpreter).push_boolean((*coroutine).is_yieldable());
        return 1;
    }
}
unsafe extern "C" fn luab_yield(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        return lua_yieldk(interpreter, (*interpreter).get_top(), 0, None);
    }
}
const COROUTINE_FUNCTIONS: [RegisteredFunction; 9] = {
    [
        {
            RegisteredFunction {
                name: b"create\0" as *const u8 as *const i8,
                function: Some(luab_cocreate as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"resume\0" as *const u8 as *const i8,
                function: Some(luab_coresume as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"running\0" as *const u8 as *const i8,
                function: Some(luab_corunning as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"status\0" as *const u8 as *const i8,
                function: Some(luab_costatus as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"wrap\0" as *const u8 as *const i8,
                function: Some(luab_cowrap as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"yield\0" as *const u8 as *const i8,
                function: Some(luab_yield as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"isyieldable\0" as *const u8 as *const i8,
                function: Some(luab_yieldable as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"close\0" as *const u8 as *const i8,
                function: Some(luab_close as unsafe extern "C" fn(*mut Interpreter) -> i32),
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
pub unsafe extern "C" fn luaopen_coroutine(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        lual_checkversion_(
            interpreter,
            504.0,
            (::core::mem::size_of::<i64>() as u64)
                .wrapping_mul(16 as u64)
                .wrapping_add(::core::mem::size_of::<f64>() as u64),
        );
        (*interpreter).lua_createtable();
        lual_setfuncs(interpreter, COROUTINE_FUNCTIONS.as_ptr(), 0);
        return 1;
    }
}
pub unsafe extern "C" fn getco(interpreter: *mut Interpreter) -> *mut Interpreter {
    unsafe {
        let co: *mut Interpreter = lua_tothread(interpreter, 1);
        ((co != std::ptr::null_mut()) as i64 != 0
            || lual_typeerror(interpreter, 1, b"thread\0" as *const u8 as *const i8) != 0) as i32;
        return co;
    }
}
pub unsafe extern "C" fn auxresume(interpreter: *mut Interpreter, co: *mut Interpreter, narg: i32) -> i32 {
    unsafe {
        let status: i32;
        let mut nres: i32 = 0;
        if lua_checkstack(co, narg) == 0 {
            lua_pushstring(
                interpreter,
                b"too many arguments to resume\0" as *const u8 as *const i8,
            );
            return -1;
        }
        lua_xmove(interpreter, co, narg);
        status = lua_resume(co, interpreter, narg, &mut nres);
        if status == 0 || status == 1 {
            if lua_checkstack(interpreter, nres + 1) == 0 {
                lua_settop(co, -nres - 1);
                lua_pushstring(
                    interpreter,
                    b"too many results to resume\0" as *const u8 as *const i8,
                );
                return -1;
            }
            lua_xmove(co, interpreter, nres);
            return nres;
        } else {
            lua_xmove(co, interpreter, 1);
            return -1;
        };
    }
}
pub unsafe extern "C" fn luab_auxwrap(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let co: *mut Interpreter = lua_tothread(interpreter, -(1000000 as i32) - 1000 as i32 - 1);
        let r: i32 = auxresume(interpreter, co, (*interpreter).get_top());
        if r < 0 {
            let mut stat: i32 = (*co).get_status();
            if stat != 0 && stat != 1 {
                stat = lua_closethread(co, interpreter);
                lua_xmove(co, interpreter, 1);
            }
            if stat != 4 && lua_type(interpreter, -1) == Some(TagType::String) {
                lual_where(interpreter, 1);
                lua_rotate(interpreter, -2, 1);
                lua_concat(interpreter, 2);
            }
            return lua_error(interpreter);
        }
        return r;
    }
}
