#![allow(unpredictable_function_pointer_comparisons,unsafe_code)]
use crate::f2i::*;
use crate::registeredfunction::*;
use crate::state::*;
use crate::tag::*;
pub unsafe extern "C" fn luab_tonumber(state: *mut State) -> i32 {
    unsafe {
        match lua_type(state, 2) {
            None | Some(TAG_TYPE_NIL) => {
                if lua_type(state, 1) == Some(TAG_TYPE_NUMERIC) {
                    lua_settop(state, 1);
                    return 1;
                } else {
                    let mut l: u64 = 0;
                    let s: *const i8 = lua_tolstring(state, 1, &mut l);
                    if !s.is_null() && lua_stringtonumber(state, s) == l.wrapping_add(1 as u64) {
                        return 1;
                    }
                    lual_checkany(state, 1);
                }
            },
            _ => {
                let mut l_0: u64 = 0;
                let s_0: *const i8;
                let mut n: i64 = 0;
                let base: i64 = lual_checkinteger(state, 2);
                lual_checktype(state, 1, TAG_TYPE_STRING);
                s_0 = lua_tolstring(state, 1, &mut l_0);
                (((2 as i64 <= base && base <= 36 as i64) as i32 != 0) as i64 != 0
                    || lual_argerror(state, 2, b"base out of range\0" as *const u8 as *const i8) != 0)
                    as i32;
                if b_str2int(s_0, base as i32, &mut n) == s_0.offset(l_0 as isize) {
                    (*state).push_integer(n);
                    return 1;
                }
            }
        };
        (*state).push_nil();
        return 1;
    }
}
pub unsafe extern "C" fn luab_error(state: *mut State) -> i32 {
    unsafe {
        let level: i32 = lual_optinteger(state, 2, 1) as i32;
        lua_settop(state, 1);
        if lua_type(state, 1) == Some(TAG_TYPE_STRING) && level > 0 {
            lual_where(state, level);
            lua_pushvalue(state, 1);
            lua_concat(state, 2);
        }
        return lua_error(state);
    }
}
pub unsafe extern "C" fn luab_getmetatable(state: *mut State) -> i32 {
    unsafe {
        lual_checkany(state, 1);
        if (*state).lua_getmetatable(1) {
            lual_getmetafield(state, 1, b"__metatable\0" as *const u8 as *const i8);
            return 1;
        } else {
            (*state).push_nil();
            return 1;
        }
    }
}
pub unsafe extern "C" fn luab_setmetatable(state: *mut State) -> i32 {
    unsafe {
        lual_checktype(state, 1, TAG_TYPE_TABLE);
        match lua_type(state, 2) {
            Some(TAG_TYPE_NIL) | Some(TAG_TYPE_TABLE) => {
            },
            _ => {
                lual_typeerror(state, 2, b"nil or table\0" as *const u8 as *const i8);
            },
        };
        if ((lual_getmetafield(state, 1, b"__metatable\0" as *const u8 as *const i8) != 0) as i32
            != 0) as i64
            != 0
        {
            return lual_error(
                state,
                b"cannot change a protected metatable\0" as *const u8 as *const i8,
            );
        }
        lua_settop(state, 2);
        lua_setmetatable(state, 1);
        return 1;
    }
}
pub unsafe extern "C" fn luab_rawequal(state: *mut State) -> i32 {
    unsafe {
        lual_checkany(state, 1);
        lual_checkany(state, 2);
        (*state).push_boolean(lua_rawequal(state, 1, 2));
        return 1;
    }
}
pub unsafe extern "C" fn luab_rawlen(state: *mut State) -> i32 {
    unsafe {
        match lua_type(state, 1) {
            Some(TAG_TYPE_TABLE) | Some(TAG_TYPE_STRING) => {
            },
            _ => {
                lual_typeerror(state, 1, b"table or string\0" as *const u8 as *const i8);
            },
        };
        (*state).push_integer(lua_rawlen(state, 1) as i64);
        return 1;
    }
}
pub unsafe extern "C" fn luab_rawget(state: *mut State) -> i32 {
    unsafe {
        lual_checktype(state, 1, TAG_TYPE_TABLE);
        lual_checkany(state, 2);
        lua_settop(state, 2);
        lua_rawget(state, 1);
        return 1;
    }
}
pub unsafe extern "C" fn luab_rawset(state: *mut State) -> i32 {
    unsafe {
        lual_checktype(state, 1, TAG_TYPE_TABLE);
        lual_checkany(state, 2);
        lual_checkany(state, 3);
        lua_settop(state, 3);
        lua_rawset(state, 1);
        return 1;
    }
}
pub unsafe extern "C" fn pushmode(state: *mut State, oldmode: i32) -> i32 {
    unsafe {
        if oldmode == -1 {
            (*state).push_nil();
        } else {
            lua_pushstring(
                state,
                if oldmode == 11 as i32 {
                    b"incremental\0" as *const u8 as *const i8
                } else {
                    b"generational\0" as *const u8 as *const i8
                },
            );
        }
        return 1;
    }
}
pub unsafe extern "C" fn luab_collectgarbage(state: *mut State) -> i32 {
    unsafe {
        pub const OPTS: [*const i8; 11] = [
            b"stop\0" as *const u8 as *const i8,
            b"restart\0" as *const u8 as *const i8,
            b"collect\0" as *const u8 as *const i8,
            b"count\0" as *const u8 as *const i8,
            b"step\0" as *const u8 as *const i8,
            b"setpause\0" as *const u8 as *const i8,
            b"setstepmul\0" as *const u8 as *const i8,
            b"isrunning\0" as *const u8 as *const i8,
            b"generational\0" as *const u8 as *const i8,
            b"incremental\0" as *const u8 as *const i8,
            std::ptr::null(),
        ];
        pub const OPTS_NUMBERS: [i32; 10] = [0, 1, 2, 3, 5, 6, 7, 9 as i32, 10 as i32, 11 as i32];
        let o: i32 = OPTS_NUMBERS[lual_checkoption(
            state,
            1,
            b"collect\0" as *const u8 as *const i8,
            OPTS.as_ptr(),
        ) as usize];
        match o {
            3 => {
                let k: i32 = lua_gc(state, o);
                let b: i32 = lua_gc(state, 4);
                if !(k == -1) {
                    (*state).push_number(k as f64 + b as f64 / 1024.0);
                    return 1;
                }
            }
            5 => {
                let step: i32 = lual_optinteger(state, 2, 0) as i32;
                let res: i32 = lua_gc(state, o, step);
                if !(res == -1) {
                    (*state).push_boolean(0 != res);
                    return 1;
                }
            }
            6 | 7 => {
                let p: i32 = lual_optinteger(state, 2, 0) as i32;
                let previous: i32 = lua_gc(state, o, p);
                if !(previous == -1) {
                    (*state).push_integer(previous as i64);
                    return 1;
                }
            }
            9 => {
                let res_0: i32 = lua_gc(state, o);
                if !(res_0 == -1) {
                    (*state).push_boolean(0 != res_0);
                    return 1;
                }
            }
            10 => {
                let minormul: i32 = lual_optinteger(state, 2, 0) as i32;
                let majormul: i32 = lual_optinteger(state, 3, 0) as i32;
                return pushmode(state, lua_gc(state, o, minormul, majormul));
            }
            11 => {
                let pause: i32 = lual_optinteger(state, 2, 0) as i32;
                let stepmul: i32 = lual_optinteger(state, 3, 0) as i32;
                let stepsize: i32 = lual_optinteger(state, 4, 0) as i32;
                return pushmode(state, lua_gc(state, o, pause, stepmul, stepsize));
            }
            _ => {
                let res_1: i32 = lua_gc(state, o);
                if !(res_1 == -1) {
                    (*state).push_integer(res_1 as i64);
                    return 1;
                }
            }
        }
        (*state).push_nil();
        return 1;
    }
}
pub unsafe extern "C" fn luab_type(state: *mut State) -> i32 {
    unsafe {
        let t = lua_type(state, 1);
        match t {
            None => {
                lual_argerror(state, 1, b"value expected\0" as *const u8 as *const i8);
            },
            _  => {
                lua_pushstring(state, lua_typename(state, t));
            },
        };
        return 1;
    }
}
pub unsafe extern "C" fn luab_next(state: *mut State) -> i32 {
    unsafe {
        lual_checktype(state, 1, TAG_TYPE_TABLE);
        lua_settop(state, 2);
        if lua_next(state, 1) != 0 {
            return 2;
        } else {
            (*state).push_nil();
            return 1;
        };
    }
}
pub unsafe extern "C" fn pairscont(mut _state: *mut State, mut _status: i32, mut _k: i64) -> i32 {
    return 3;
}
pub unsafe extern "C" fn luab_pairs(state: *mut State) -> i32 {
    unsafe {
        lual_checkany(state, 1);
        if lual_getmetafield(state, 1, b"__pairs\0" as *const u8 as *const i8) == 0 {
            lua_pushcclosure(
                state,
                Some(luab_next as unsafe extern "C" fn(*mut State) -> i32),
                0,
            );
            lua_pushvalue(state, 1);
            (*state).push_nil();
        } else {
            lua_pushvalue(state, 1);
            lua_callk(
                state,
                1,
                3,
                0,
                Some(pairscont as unsafe extern "C" fn(*mut State, i32, i64) -> i32),
            );
        }
        return 3;
    }
}
pub unsafe extern "C" fn ipairsaux(state: *mut State) -> i32 {
    unsafe {
        let mut i: i64 = lual_checkinteger(state, 2);
        i = (i as u64).wrapping_add(1 as u64) as i64;
        (*state).push_integer(i);
        return if lua_geti(state, 1, i) == 0 { 1 } else { 2 };
    }
}
pub unsafe extern "C" fn luab_ipairs(state: *mut State) -> i32 {
    unsafe {
        lual_checkany(state, 1);
        lua_pushcclosure(
            state,
            Some(ipairsaux as unsafe extern "C" fn(*mut State) -> i32),
            0,
        );
        lua_pushvalue(state, 1);
        (*state).push_integer(0);
        return 3;
    }
}
pub unsafe extern "C" fn load_aux(state: *mut State, status: i32, envidx: i32) -> i32 {
    unsafe {
        if ((status == 0) as i32 != 0) as i64 != 0 {
            if envidx != 0 {
                lua_pushvalue(state, envidx);
                if (lua_setupvalue(state, -2, 1)).is_null() {
                    lua_settop(state, -2);
                }
            }
            return 1;
        } else {
            (*state).push_nil();
            lua_rotate(state, -2, 1);
            return 2;
        };
    }
}
pub unsafe extern "C" fn luab_loadfile(state: *mut State) -> i32 {
    unsafe {
        let fname: *const i8 = lual_optlstring(state, 1, std::ptr::null(), std::ptr::null_mut());
        let mode: *const i8 = lual_optlstring(state, 2, std::ptr::null(), std::ptr::null_mut());
        let env: i32 = if lua_type(state, 3) == None { 0 } else { 3 };
        let status: i32 = lual_loadfilex(state, fname, mode);
        return load_aux(state, status, env);
    }
}
pub unsafe extern "C" fn generic_reader(
    state: *mut State,
    mut _ud: *mut libc::c_void,
    size: *mut u64,
) -> *const i8 {
    unsafe {
        lual_checkstack(
            state,
            2,
            b"too many nested functions\0" as *const u8 as *const i8,
        );
        lua_pushvalue(state, 1);
        lua_callk(state, 0, 1, 0, None);
        if lua_type(state, -1) == Some(TAG_TYPE_NIL) {
            lua_settop(state, -2);
            *size = 0;
            return std::ptr::null();
        } else if !lua_isstring(state, -1) {
            lual_error(
                state,
                b"reader function must return a string\0" as *const u8 as *const i8,
            );
        }
        lua_copy(state, -1, 5);
        lua_settop(state, -2);
        return lua_tolstring(state, 5, size);
    }
}
pub unsafe extern "C" fn luab_load(state: *mut State) -> i32 {
    unsafe {
        let status: i32;
        let mut l: u64 = 0;
        let s: *const i8 = lua_tolstring(state, 1, &mut l);
        let mode: *const i8 = lual_optlstring(
            state,
            3,
            b"bt\0" as *const u8 as *const i8,
            std::ptr::null_mut(),
        );
        let env: i32 = if !(lua_type(state, 4) == None) { 4 } else { 0 };
        if !s.is_null() {
            let chunkname: *const i8 = lual_optlstring(state, 2, s, std::ptr::null_mut());
            status = lual_loadbufferx(state, s, l, chunkname, mode);
        } else {
            let chunkname_0: *const i8 = lual_optlstring(
                state,
                2,
                b"=(load)\0" as *const u8 as *const i8,
                std::ptr::null_mut(),
            );
            lual_checktype(state, 1, TAG_TYPE_CLOSURE);
            lua_settop(state, 5);
            status = lua_load(
                state,
                Some(
                    generic_reader
                        as unsafe extern "C" fn(
                            *mut State,
                            *mut libc::c_void,
                            *mut u64,
                        ) -> *const i8,
                ),
                std::ptr::null_mut(),
                chunkname_0,
                mode,
            );
        }
        return load_aux(state, status, env);
    }
}
pub unsafe extern "C" fn dofilecont(state: *mut State, mut _d1: i32, mut _d2: i64) -> i32 {
    unsafe {
        return (*state).get_top() - 1;
    }
}
pub unsafe extern "C" fn luab_dofile(state: *mut State) -> i32 {
    unsafe {
        let fname: *const i8 = lual_optlstring(state, 1, std::ptr::null(), std::ptr::null_mut());
        lua_settop(state, 1);
        if ((lual_loadfilex(state, fname, std::ptr::null()) != 0) as i32 != 0) as i64 != 0 {
            return lua_error(state);
        }
        lua_callk(
            state,
            0,
            -1,
            0,
            Some(dofilecont as unsafe extern "C" fn(*mut State, i32, i64) -> i32),
        );
        return dofilecont(state, 0, 0);
    }
}
pub unsafe extern "C" fn luab_assert(state: *mut State) -> i32 {
    unsafe {
        if (lua_toboolean(state, 1) != 0) as i64 != 0 {
            return (*state).get_top();
        } else {
            lual_checkany(state, 1);
            lua_rotate(state, 1, -1);
            lua_settop(state, -2);
            lua_pushstring(state, b"assertion failed!\0" as *const u8 as *const i8);
            lua_settop(state, 1);
            return luab_error(state);
        };
    }
}
pub unsafe extern "C" fn luab_select(state: *mut State) -> i32 {
    unsafe {
        let n: i32 = (*state).get_top();
        if lua_type(state, 1) == Some(TAG_TYPE_STRING)
            && *lua_tolstring(state, 1, std::ptr::null_mut()) as i32 == '#' as i32
        {
            (*state).push_integer((n - 1) as i64);
            return 1;
        } else {
            let mut i: i64 = lual_checkinteger(state, 1);
            if i < 0 {
                i = n as i64 + i;
            } else if i > n as i64 {
                i = n as i64;
            }
            (((1 <= i) as i32 != 0) as i64 != 0
                || lual_argerror(state, 1, b"index out of range\0" as *const u8 as *const i8) != 0)
                as i32;
            return n - i as i32;
        };
    }
}
pub unsafe extern "C" fn luab_xpcall(state: *mut State) -> i32 {
    unsafe {
        let status: i32;
        let n: i32 = (*state).get_top();
        lual_checktype(state, 2, TAG_TYPE_CLOSURE);
        (*state).push_boolean(true);
        lua_pushvalue(state, 1);
        lua_rotate(state, 3, 2);
        status = lua_pcallk(
            state,
            n - 2,
            -1,
            2,
            2 as i64,
            Some(finishpcall as unsafe extern "C" fn(*mut State, i32, i64) -> i32),
        );
        return finishpcall(state, status, 2 as i64);
    }
}
pub unsafe extern "C" fn luab_tostring(state: *mut State) -> i32 {
    unsafe {
        lual_checkany(state, 1);
        lual_tolstring(state, 1, std::ptr::null_mut());
        return 1;
    }
}
pub const BASE_FUNCTIONS: [RegisteredFunction; 26] = {
    [
        {
            RegisteredFunction {
                name: b"assert\0" as *const u8 as *const i8,
                function: Some(luab_assert as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"collectgarbage\0" as *const u8 as *const i8,
                function: Some(luab_collectgarbage as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"dofile\0" as *const u8 as *const i8,
                function: Some(luab_dofile as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"error\0" as *const u8 as *const i8,
                function: Some(luab_error as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"getmetatable\0" as *const u8 as *const i8,
                function: Some(luab_getmetatable as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"ipairs\0" as *const u8 as *const i8,
                function: Some(luab_ipairs as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"loadfile\0" as *const u8 as *const i8,
                function: Some(luab_loadfile as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"load\0" as *const u8 as *const i8,
                function: Some(luab_load as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"next\0" as *const u8 as *const i8,
                function: Some(luab_next as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"pairs\0" as *const u8 as *const i8,
                function: Some(luab_pairs as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"pcall\0" as *const u8 as *const i8,
                function: Some(luab_pcall as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"print\0" as *const u8 as *const i8,
                function: Some(luab_print as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"warn\0" as *const u8 as *const i8,
                function: Some(luab_warn as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"rawequal\0" as *const u8 as *const i8,
                function: Some(luab_rawequal as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"rawlen\0" as *const u8 as *const i8,
                function: Some(luab_rawlen as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"rawget\0" as *const u8 as *const i8,
                function: Some(luab_rawget as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"rawset\0" as *const u8 as *const i8,
                function: Some(luab_rawset as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"select\0" as *const u8 as *const i8,
                function: Some(luab_select as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"setmetatable\0" as *const u8 as *const i8,
                function: Some(luab_setmetatable as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"tonumber\0" as *const u8 as *const i8,
                function: Some(luab_tonumber as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"tostring\0" as *const u8 as *const i8,
                function: Some(luab_tostring as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"type\0" as *const u8 as *const i8,
                function: Some(luab_type as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"xpcall\0" as *const u8 as *const i8,
                function: Some(luab_xpcall as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"_G\0" as *const u8 as *const i8,
                function: None,
            }
        },
        {
            RegisteredFunction {
                name: b"_VERSION\0" as *const u8 as *const i8,
                function: None,
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
pub unsafe extern "C" fn luaopen_base(state: *mut State) -> i32 {
    unsafe {
        lua_rawgeti(state, -(1000000 as i32) - 1000 as i32, 2 as i64);
        lual_setfuncs(state, BASE_FUNCTIONS.as_ptr(), 0);
        lua_pushvalue(state, -1);
        lua_setfield(state, -2, b"_G\0" as *const u8 as *const i8);
        lua_pushstring(state, b"Lua 5.4\0" as *const u8 as *const i8);
        lua_setfield(state, -2, b"_VERSION\0" as *const u8 as *const i8);
        return 1;
    }
}
