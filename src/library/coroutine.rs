use crate::coroutine::*;
use crate::interpreter::*;
use crate::registeredfunction::*;
use crate::status::*;
use crate::tag::*;
unsafe fn luab_cocreate(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        (*interpreter).lual_checktype(1, TagType::Closure);
        let nl: *mut Interpreter = lua_newthread(interpreter);
        lua_pushvalue(interpreter, 1);
        lua_xmove(interpreter, nl, 1);
    }
    1
}
unsafe fn luab_cowrap(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        luab_cocreate(interpreter);
        lua_pushcclosure(interpreter, Some(luab_auxwrap as unsafe fn(*mut Interpreter) -> i32), 1);
    }
    1
}
unsafe fn luab_coresume(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let coroutine: *mut Interpreter = get_coroutine(interpreter);
        let r: i32 = auxresume(interpreter, coroutine, (*interpreter).get_top() - 1);
        if r < 0 {
            (*interpreter).push_boolean(false);
            lua_rotate(interpreter, -2, 1);
            2
        } else {
            (*interpreter).push_boolean(true);
            lua_rotate(interpreter, -(r + 1), 1);
            r + 1
        }
    }
}
unsafe fn luab_corunning(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        (*interpreter).push_boolean((*interpreter).push_state());
    }
    2
}
unsafe fn luab_close(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let coroutine: *mut Interpreter = get_coroutine(interpreter);
        match CoroutineStatus::auxiliary_status(interpreter, coroutine) {
            | CoroutineStatus::Dead | CoroutineStatus::Yield => match lua_closethread(coroutine, interpreter) {
                | Status::OK => {
                    (*interpreter).push_boolean(true);
                    1
                },
                | _ => {
                    (*interpreter).push_boolean(false);
                    lua_xmove(coroutine, interpreter, 1);
                    2
                },
            },
            | x => lual_error(interpreter, c"cannot close a %s coroutine".as_ptr(), x.get_name()),
        }
    }
}
unsafe fn luab_costatus(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        lua_pushstring(
            interpreter,
            CoroutineStatus::auxiliary_status(interpreter, get_coroutine(interpreter)).get_name(),
        );
    }
    1
}
unsafe fn luab_yieldable(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let coroutine: *mut Interpreter = if lua_type(interpreter, 1) == None {
            interpreter
        } else {
            get_coroutine(interpreter)
        };
        (*interpreter).push_boolean((*coroutine).is_yieldable());
    }
    1
}
unsafe fn luab_yield(interpreter: *mut Interpreter) -> i32 {
    unsafe { lua_yieldk(interpreter, (*interpreter).get_top(), 0, None) }
}
const COROUTINE_FUNCTIONS: [RegisteredFunction; 8] = {
    [
        {
            RegisteredFunction {
                registeredfunction_name: c"create".as_ptr(),
                registeredfunction_function: Some(luab_cocreate as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"resume".as_ptr(),
                registeredfunction_function: Some(luab_coresume as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"running".as_ptr(),
                registeredfunction_function: Some(luab_corunning as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"status".as_ptr(),
                registeredfunction_function: Some(luab_costatus as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"wrap".as_ptr(),
                registeredfunction_function: Some(luab_cowrap as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"yield".as_ptr(),
                registeredfunction_function: Some(luab_yield as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"isyieldable".as_ptr(),
                registeredfunction_function: Some(luab_yieldable as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"close".as_ptr(),
                registeredfunction_function: Some(luab_close as unsafe fn(*mut Interpreter) -> i32),
            }
        },
    ]
};
pub unsafe fn luaopen_coroutine(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        (*interpreter).lua_createtable();
        lual_setfuncs(interpreter, COROUTINE_FUNCTIONS.as_ptr(), COROUTINE_FUNCTIONS.len(), 0);
    }
    1
}
pub unsafe fn get_coroutine(interpreter: *mut Interpreter) -> *mut Interpreter {
    unsafe {
        let coroutine: *mut Interpreter = lua_tothread(interpreter, 1);
        if coroutine.is_null() {
            lual_typeerror(interpreter, 1, c"thread".as_ptr());
        }
        coroutine
    }
}
pub unsafe fn auxresume(interpreter: *mut Interpreter, coroutine: *mut Interpreter, narg: i32) -> i32 {
    unsafe {
        if lua_checkstack(coroutine, narg) == 0 {
            lua_pushstring(interpreter, c"too many arguments to resume".as_ptr());
            return -1;
        } else {
            lua_xmove(interpreter, coroutine, narg);
            let mut nres: i32 = 0;
            let status = lua_resume(coroutine, interpreter, narg, &mut nres);
            if status == Status::OK || status == Status::Yield {
                if lua_checkstack(interpreter, nres + 1) == 0 {
                    lua_settop(coroutine, -nres - 1);
                    lua_pushstring(interpreter, c"too many results to resume".as_ptr());
                    return -1;
                }
                lua_xmove(coroutine, interpreter, nres);
                return nres;
            } else {
                lua_xmove(coroutine, interpreter, 1);
                return -1;
            };
        }
    }
}
pub unsafe fn luab_auxwrap(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let coroutine = lua_tothread(interpreter, -(1000000 as i32) - 1000 as i32 - 1);
        let r = auxresume(interpreter, coroutine, (*interpreter).get_top());
        if r < 0 {
            let mut status = (*coroutine).get_status();
            if status != Status::OK && status != Status::Yield {
                status = lua_closethread(coroutine, interpreter);
                lua_xmove(coroutine, interpreter, 1);
            }
            if status != Status::MemoryError && lua_type(interpreter, -1) == Some(TagType::String) {
                lual_where(interpreter, 1);
                lua_rotate(interpreter, -2, 1);
                lua_concat(interpreter, 2);
            }
            return lua_error(interpreter);
        }
        r
    }
}
