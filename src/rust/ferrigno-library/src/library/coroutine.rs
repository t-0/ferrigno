use crate::coroutinestatus::*;
use crate::registeredfunction::*;
use crate::state::*;
use crate::status::*;
use crate::tagtype::*;
unsafe fn luab_cocreate(state: *mut State) -> i32 {
    unsafe {
        (*state).lual_checktype(1, TagType::Closure);
        let nl: *mut State = lua_newthread(state);
        lua_pushvalue(state, 1);
        lua_xmove(state, nl, 1);
    }
    1
}
unsafe fn luab_cowrap(state: *mut State) -> i32 {
    unsafe {
        luab_cocreate(state);
        lua_pushcclosure(state, Some(luab_auxwrap as unsafe fn(*mut State) -> i32), 1);
    }
    1
}
unsafe fn luab_coresume(state: *mut State) -> i32 {
    unsafe {
        let coroutine: *mut State = get_coroutine(state);
        let r: i32 = auxresume(state, coroutine, (*state).get_top() - 1);
        if r < 0 {
            (*state).push_boolean(false);
            lua_rotate(state, -2, 1);
            2
        } else {
            (*state).push_boolean(true);
            lua_rotate(state, -(r + 1), 1);
            r + 1
        }
    }
}
unsafe fn luab_corunning(state: *mut State) -> i32 {
    unsafe {
        (*state).push_boolean((*state).push_state());
    }
    2
}
unsafe fn luab_close(state: *mut State) -> i32 {
    unsafe {
        let coroutine: *mut State = if lua_type(state, 1).is_none() { state } else { get_coroutine(state) };
        match CoroutineStatus::auxiliary_status(state, coroutine) {
            | CoroutineStatus::Dead | CoroutineStatus::Yield => match lua_closethread(coroutine, state) {
                | Status::OK => {
                    (*state).push_boolean(true);
                    1
                },
                | _ => {
                    (*state).push_boolean(false);
                    lua_xmove(coroutine, state, 1);
                    2
                },
            },
            | CoroutineStatus::Running => {
                if coroutine == (*(*state).interpreter_global).global_maininterpreter {
                    lual_error(state, c"cannot close main thread".as_ptr())
                } else {
                    lua_closethread(coroutine, state);
                    0 // does not return
                }
            },
            | x => lual_error(state, c"cannot close a %s coroutine".as_ptr(), x.get_name()),
        }
    }
}
unsafe fn luab_costatus(state: *mut State) -> i32 {
    unsafe {
        lua_pushstring(state, CoroutineStatus::auxiliary_status(state, get_coroutine(state)).get_name());
    }
    1
}
unsafe fn luab_yieldable(state: *mut State) -> i32 {
    unsafe {
        let coroutine: *mut State = if lua_type(state, 1).is_none() { state } else { get_coroutine(state) };
        (*state).push_boolean((*coroutine).is_yieldable());
    }
    1
}
unsafe fn luab_yield(state: *mut State) -> i32 {
    unsafe { lua_yieldk(state, (*state).get_top(), 0, None) }
}
const COROUTINE_FUNCTIONS: [RegisteredFunction; 8] = {
    [
        {
            RegisteredFunction {
                registeredfunction_name: c"create".as_ptr(),
                registeredfunction_function: Some(luab_cocreate as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"resume".as_ptr(),
                registeredfunction_function: Some(luab_coresume as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"running".as_ptr(),
                registeredfunction_function: Some(luab_corunning as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"status".as_ptr(),
                registeredfunction_function: Some(luab_costatus as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"wrap".as_ptr(),
                registeredfunction_function: Some(luab_cowrap as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"yield".as_ptr(),
                registeredfunction_function: Some(luab_yield as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"isyieldable".as_ptr(),
                registeredfunction_function: Some(luab_yieldable as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"close".as_ptr(),
                registeredfunction_function: Some(luab_close as unsafe fn(*mut State) -> i32),
            }
        },
    ]
};
pub unsafe fn luaopen_coroutine(state: *mut State) -> i32 {
    unsafe {
        (*state).lua_createtable();
        lual_setfuncs(state, COROUTINE_FUNCTIONS.as_ptr(), COROUTINE_FUNCTIONS.len(), 0);
    }
    1
}
pub unsafe fn get_coroutine(state: *mut State) -> *mut State {
    unsafe {
        let coroutine: *mut State = lua_tothread(state, 1);
        if coroutine.is_null() {
            lual_typeerror(state, 1, c"thread".as_ptr());
        }
        coroutine
    }
}
pub unsafe fn auxresume(state: *mut State, coroutine: *mut State, narg: i32) -> i32 {
    unsafe {
        if lua_checkstack(coroutine, narg) == 0 {
            lua_pushstring(state, c"too many arguments to resume".as_ptr());
            -1
        } else {
            lua_xmove(state, coroutine, narg);
            let mut nres: i32 = 0;
            let status = lua_resume(coroutine, state, narg, &mut nres);
            if status == Status::OK || status == Status::Yield {
                if lua_checkstack(state, nres + 1) == 0 {
                    lua_settop(coroutine, -nres - 1);
                    lua_pushstring(state, c"too many results to resume".as_ptr());
                    return -1;
                }
                lua_xmove(coroutine, state, nres);
                nres
            } else {
                lua_xmove(coroutine, state, 1);
                -1
            }
        }
    }
}
pub unsafe fn luab_auxwrap(state: *mut State) -> i32 {
    unsafe {
        let coroutine = lua_tothread(state, LUA_REGISTRYINDEX - 1);
        let r = auxresume(state, coroutine, (*state).get_top());
        if r < 0 {
            let mut status = (*coroutine).get_status();
            if status != Status::OK && status != Status::Yield {
                status = lua_closethread(coroutine, state);
                lua_xmove(coroutine, state, 1);
            }
            if status != Status::MemoryError && lua_type(state, -1) == Some(TagType::String) {
                lual_where(state, 1);
                lua_rotate(state, -2, 1);
                lua_concat(state, 2);
            }
            return lua_error(state);
        }
        r
    }
}
