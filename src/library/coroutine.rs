#![allow(unused)]
use crate::coroutine::*;
use crate::interpreter::*;
use crate::registeredfunction::*;
use crate::tag::*;
use rlua::*;
unsafe fn luab_cocreate(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        (*interpreter).lual_checktype(1, TagType::Closure);
        let nl: *mut Interpreter = lua_newthread(interpreter);
        lua_pushvalue(interpreter, 1);
        lua_xmove(interpreter, nl, 1);
        return 1;
    }
}
unsafe fn luab_cowrap(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        luab_cocreate(interpreter);
        lua_pushcclosure(interpreter, Some(luab_auxwrap as unsafe fn(*mut Interpreter) -> i32), 1);
        return 1;
    }
}
unsafe fn luab_coresume(interpreter: *mut Interpreter) -> i32 {
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
unsafe fn luab_corunning(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        (*interpreter).push_boolean((*interpreter).push_state());
        return 2;
    }
}
pub const COS_RUN: i32 = 0;
pub const COS_DEAD: i32 = 1;
pub const COS_YIELD: i32 = 2;
pub const COS_NORM: i32 = 3;
unsafe fn luab_close(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let co: *mut Interpreter = getco(interpreter);
        let mut couroutinestatus: i32 = auxstatus(interpreter, co);
        match couroutinestatus {
            COS_DEAD | COS_YIELD => {
                let status = lua_closethread(co, interpreter);
                if status == Status::OK {
                    (*interpreter).push_boolean(true);
                    return 1;
                } else {
                    (*interpreter).push_boolean(false);
                    lua_xmove(co, interpreter, 1);
                    return 2;
                }
            },
            _ => {
                return lual_error(interpreter, c"cannot close a %s coroutine".as_ptr(), COROUTINE_STATUS_NAMES[couroutinestatus as usize]);
            },
        };
    }
}
unsafe fn luab_costatus(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let co: *mut Interpreter = getco(interpreter);
        lua_pushstring(interpreter, COROUTINE_STATUS_NAMES[auxstatus(interpreter, co) as usize]);
        return 1;
    }
}
unsafe fn luab_yieldable(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let coroutine: *mut Interpreter = if lua_type(interpreter, 1) == None { interpreter } else { getco(interpreter) };
        (*interpreter).push_boolean((*coroutine).is_yieldable());
        return 1;
    }
}
unsafe fn luab_yield(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        return lua_yieldk(interpreter, (*interpreter).get_top(), 0, None);
    }
}
const COROUTINE_FUNCTIONS: [RegisteredFunction; 8] = {
    [
        { RegisteredFunction { name: c"create".as_ptr(), function: Some(luab_cocreate as unsafe fn(*mut Interpreter) -> i32) } },
        { RegisteredFunction { name: c"resume".as_ptr(), function: Some(luab_coresume as unsafe fn(*mut Interpreter) -> i32) } },
        { RegisteredFunction { name: c"running".as_ptr(), function: Some(luab_corunning as unsafe fn(*mut Interpreter) -> i32) } },
        { RegisteredFunction { name: c"status".as_ptr(), function: Some(luab_costatus as unsafe fn(*mut Interpreter) -> i32) } },
        { RegisteredFunction { name: c"wrap".as_ptr(), function: Some(luab_cowrap as unsafe fn(*mut Interpreter) -> i32) } },
        { RegisteredFunction { name: c"yield".as_ptr(), function: Some(luab_yield as unsafe fn(*mut Interpreter) -> i32) } },
        { RegisteredFunction { name: c"isyieldable".as_ptr(), function: Some(luab_yieldable as unsafe fn(*mut Interpreter) -> i32) } },
        { RegisteredFunction { name: c"close".as_ptr(), function: Some(luab_close as unsafe fn(*mut Interpreter) -> i32) } },
    ]
};
pub unsafe fn luaopen_coroutine(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        lual_checkversion_(interpreter, 504.0, (size_of::<i64>() as usize).wrapping_mul(16 as usize).wrapping_add(size_of::<f64>() as usize));
        (*interpreter).lua_createtable();
        lual_setfuncs(interpreter, COROUTINE_FUNCTIONS.as_ptr(), COROUTINE_FUNCTIONS.len(), 0);
        return 1;
    }
}
pub unsafe fn getco(interpreter: *mut Interpreter) -> *mut Interpreter {
    unsafe {
        let co: *mut Interpreter = lua_tothread(interpreter, 1);
        if co.is_null() {
            lual_typeerror(interpreter, 1, c"thread".as_ptr());
        }
        return co;
    }
}
pub unsafe fn auxresume(interpreter: *mut Interpreter, co: *mut Interpreter, narg: i32) -> i32 {
    unsafe {
        if lua_checkstack(co, narg) == 0 {
            lua_pushstring(interpreter, c"too many arguments to resume".as_ptr());
            return -1;
        } else {
            lua_xmove(interpreter, co, narg);
            let mut nres: i32 = 0;
            let status = lua_resume(co, interpreter, narg, &mut nres);
            if status == Status::OK || status == Status::Yield {
                if lua_checkstack(interpreter, nres + 1) == 0 {
                    lua_settop(co, -nres - 1);
                    lua_pushstring(interpreter, c"too many results to resume".as_ptr());
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
}
pub unsafe fn luab_auxwrap(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let co = lua_tothread(interpreter, -(1000000 as i32) - 1000 as i32 - 1);
        let r = auxresume(interpreter, co, (*interpreter).get_top());
        if r < 0 {
            let mut status = (*co).get_status();
            if status != Status::OK && status != Status::Yield {
                status = lua_closethread(co, interpreter);
                lua_xmove(co, interpreter, 1);
            }
            if status != Status::MemoryError && lua_type(interpreter, -1) == Some(TagType::String) {
                lual_where(interpreter, 1);
                lua_rotate(interpreter, -2, 1);
                lua_concat(interpreter, 2);
            }
            return lua_error(interpreter);
        }
        return r;
    }
}
