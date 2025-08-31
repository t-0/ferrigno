#![allow(static_mut_refs, unsafe_code)]
use crate::debug::*;
use crate::tag::*;
use crate::onelua::*;
use crate::registeredfunction::*;
use crate::state::*;
unsafe extern "C" fn luab_cocreate(state: *mut State) -> i32 {
    unsafe {
        lual_checktype(state, 1, TAG_TYPE_CLOSURE);
        let nl: *mut State = lua_newthread(state);
        lua_pushvalue(state, 1);
        lua_xmove(state, nl, 1);
        return 1;
    }
}
unsafe extern "C" fn luab_cowrap(state: *mut State) -> i32 {
    unsafe {
        luab_cocreate(state);
        lua_pushcclosure(
            state,
            Some(luab_auxwrap as unsafe extern "C" fn(*mut State) -> i32),
            1,
        );
        return 1;
    }
}
unsafe extern "C" fn luab_coresume(state: *mut State) -> i32 {
    unsafe {
        let co: *mut State = getco(state);
        let r: i32 = auxresume(state, co, (*state).get_top() - 1);
        if ((r < 0) as i32 != 0) as i64 != 0 {
            (*state).push_boolean(false);
            lua_rotate(state, -(2), 1);
            return 2;
        } else {
            (*state).push_boolean(true);
            lua_rotate(state, -(r + 1), 1);
            return r + 1;
        };
    }
}
unsafe extern "C" fn luab_corunning(state: *mut State) -> i32 {
    unsafe {
        (*state).push_boolean((*state).push_state());
        return 2;
    }
}
unsafe extern "C" fn luab_close(state: *mut State) -> i32 {
    unsafe {
        let co: *mut State = getco(state);
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
unsafe extern "C" fn luab_costatus(state: *mut State) -> i32 {
    unsafe {
        let co: *mut State = getco(state);
        lua_pushstring(state, COROUTINE_STATUS_NAMES[auxstatus(state, co) as usize]);
        return 1;
    }
}
unsafe extern "C" fn luab_yieldable(state: *mut State) -> i32 {
    unsafe {
        let coroutine: *mut State = if lua_type(state, 1) == None {
            state
        } else {
            getco(state)
        };
        (*state).push_boolean((*coroutine).is_yieldable());
        return 1;
    }
}
unsafe extern "C" fn luab_yield(state: *mut State) -> i32 {
    unsafe {
        return lua_yieldk(state, (*state).get_top(), 0, None);
    }
}
const COROUTINE_STATUS_NAMES: [*const i8; 4] = [
    b"running\0" as *const u8 as *const i8,
    b"dead\0" as *const u8 as *const i8,
    b"suspended\0" as *const u8 as *const i8,
    b"normal\0" as *const u8 as *const i8,
];
unsafe extern "C" fn auxstatus(state: *mut State, co: *mut State) -> i32 {
    unsafe {
        if state == co {
            return 0;
        } else {
            match (*co).get_status() {
                1 => return 2,
                0 => {
                    let mut ar: Debug = Debug {
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
                    if lua_getstack(co, 0, &mut ar) != 0 {
                        return 3;
                    } else if (*co).get_top() == 0 {
                        return 1;
                    } else {
                        return 2;
                    }
                }
                _ => return 1,
            }
        };
    }
}
static mut COROUTINE_FUNCTIONS: [RegisteredFunction; 9] = {
    [
        {
            RegisteredFunction {
                name: b"create\0" as *const u8 as *const i8,
                function: Some(luab_cocreate as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"resume\0" as *const u8 as *const i8,
                function: Some(luab_coresume as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"running\0" as *const u8 as *const i8,
                function: Some(luab_corunning as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"status\0" as *const u8 as *const i8,
                function: Some(luab_costatus as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"wrap\0" as *const u8 as *const i8,
                function: Some(luab_cowrap as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"yield\0" as *const u8 as *const i8,
                function: Some(luab_yield as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"isyieldable\0" as *const u8 as *const i8,
                function: Some(luab_yieldable as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"close\0" as *const u8 as *const i8,
                function: Some(luab_close as unsafe extern "C" fn(*mut State) -> i32),
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
pub unsafe extern "C" fn luaopen_coroutine(state: *mut State) -> i32 {
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
