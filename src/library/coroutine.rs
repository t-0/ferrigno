use crate::coroutine::*;
use crate::tag::*;
use crate::registeredfunction::*;
use crate::interpreter::*;
unsafe extern "C" fn luab_cocreate(state: *mut Interpreter) -> i32 {
    unsafe {
        lual_checktype(state, 1, TAG_TYPE_CLOSURE);
        let nl: *mut Interpreter = lua_newthread(state);
        lua_pushvalue(state, 1);
        lua_xmove(state, nl, 1);
        return 1;
    }
}
unsafe extern "C" fn luab_cowrap(state: *mut Interpreter) -> i32 {
    unsafe {
        luab_cocreate(state);
        lua_pushcclosure(
            state,
            Some(luab_auxwrap as unsafe extern "C" fn(*mut Interpreter) -> i32),
            1,
        );
        return 1;
    }
}
unsafe extern "C" fn luab_coresume(state: *mut Interpreter) -> i32 {
    unsafe {
        let co: *mut Interpreter = getco(state);
        let r: i32 = auxresume(state, co, (*state).get_top() - 1);
        if ((r < 0) as i32 != 0) as i64 != 0 {
            (*state).push_boolean(false);
            lua_rotate(state, -2, 1);
            return 2;
        } else {
            (*state).push_boolean(true);
            lua_rotate(state, -(r + 1), 1);
            return r + 1;
        };
    }
}
unsafe extern "C" fn luab_corunning(state: *mut Interpreter) -> i32 {
    unsafe {
        (*state).push_boolean((*state).push_state());
        return 2;
    }
}
unsafe extern "C" fn luab_close(state: *mut Interpreter) -> i32 {
    unsafe {
        let co: *mut Interpreter = getco(state);
        let mut status: i32 = auxstatus(state, co);
        match status {
            1 | 2 => {
                status = lua_closethread(co, state);
                if status == 0 {
                    (*state).push_boolean(true);
                    return 1;
                } else {
                    (*state).push_boolean(false);
                    lua_xmove(co, state, 1);
                    return 2;
                }
            }
            _ => {
                return lual_error(
                    state,
                    b"cannot close a %s coroutine\0" as *const u8 as *const i8,
                    COROUTINE_STATUS_NAMES[status as usize],
                );
            }
        };
    }
}
unsafe extern "C" fn luab_costatus(state: *mut Interpreter) -> i32 {
    unsafe {
        let co: *mut Interpreter = getco(state);
        lua_pushstring(state, COROUTINE_STATUS_NAMES[auxstatus(state, co) as usize]);
        return 1;
    }
}
unsafe extern "C" fn luab_yieldable(state: *mut Interpreter) -> i32 {
    unsafe {
        let coroutine: *mut Interpreter = if lua_type(state, 1) == None {
            state
        } else {
            getco(state)
        };
        (*state).push_boolean((*coroutine).is_yieldable());
        return 1;
    }
}
unsafe extern "C" fn luab_yield(state: *mut Interpreter) -> i32 {
    unsafe {
        return lua_yieldk(state, (*state).get_top(), 0, None);
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
pub unsafe extern "C" fn luaopen_coroutine(state: *mut Interpreter) -> i32 {
    unsafe {
        lual_checkversion_(
            state,
            504.0,
            (::core::mem::size_of::<i64>() as u64)
                .wrapping_mul(16 as u64)
                .wrapping_add(::core::mem::size_of::<f64>() as u64),
        );
        (*state).lua_createtable();
        lual_setfuncs(state, COROUTINE_FUNCTIONS.as_ptr(), 0);
        return 1;
    }
}
pub unsafe extern "C" fn getco(state: *mut Interpreter) -> *mut Interpreter {
    unsafe {
        let co: *mut Interpreter = lua_tothread(state, 1);
        ((co != std::ptr::null_mut()) as i64 != 0
            || lual_typeerror(state, 1, b"thread\0" as *const u8 as *const i8) != 0) as i32;
        return co;
    }
}
pub unsafe extern "C" fn auxresume(state: *mut Interpreter, co: *mut Interpreter, narg: i32) -> i32 {
    unsafe {
        let status: i32;
        let mut nres: i32 = 0;
        if ((lua_checkstack(co, narg) == 0) as i32 != 0) as i64 != 0 {
            lua_pushstring(
                state,
                b"too many arguments to resume\0" as *const u8 as *const i8,
            );
            return -1;
        }
        lua_xmove(state, co, narg);
        status = lua_resume(co, state, narg, &mut nres);
        if ((status == 0 || status == 1) as i32 != 0) as i64 != 0 {
            if ((lua_checkstack(state, nres + 1) == 0) as i32 != 0) as i64 != 0 {
                lua_settop(co, -nres - 1);
                lua_pushstring(
                    state,
                    b"too many results to resume\0" as *const u8 as *const i8,
                );
                return -1;
            }
            lua_xmove(co, state, nres);
            return nres;
        } else {
            lua_xmove(co, state, 1);
            return -1;
        };
    }
}
pub unsafe extern "C" fn luab_auxwrap(state: *mut Interpreter) -> i32 {
    unsafe {
        let co: *mut Interpreter = lua_tothread(state, -(1000000 as i32) - 1000 as i32 - 1);
        let r: i32 = auxresume(state, co, (*state).get_top());
        if ((r < 0) as i32 != 0) as i64 != 0 {
            let mut stat: i32 = (*co).get_status();
            if stat != 0 && stat != 1 {
                stat = lua_closethread(co, state);
                lua_xmove(co, state, 1);
            }
            if stat != 4 && lua_type(state, -1) == Some(TAG_TYPE_STRING) {
                lual_where(state, 1);
                lua_rotate(state, -2, 1);
                lua_concat(state, 2);
            }
            return lua_error(state);
        }
        return r;
    }
}
