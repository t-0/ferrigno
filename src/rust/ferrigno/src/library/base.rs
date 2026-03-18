#![allow(unpredictable_function_pointer_comparisons, unsafe_code)]
use crate::calls::*;
use crate::character::*;
use crate::f2i::*;
use crate::functions::*;
use crate::global::*;
use crate::registeredfunction::*;
use crate::state::*;
use crate::status::*;
use crate::tagtype::*;
use crate::utility::*;
use std::ptr::*;
pub unsafe fn luab_tonumber(state: *mut State) -> i32 {
    unsafe {
        match lua_type(state, 2) {
            | None | Some(TagType::Nil) => {
                if lua_type(state, 1) == Some(TagType::Numeric) {
                    lua_settop(state, 1);
                    return 1;
                } else {
                    let mut l: usize = 0;
                    let s: *const i8 = lua_tolstring(state, 1, &mut l);
                    if !s.is_null() && lua_stringtonumber(state, s) == l.wrapping_add(1_usize) {
                        return 1;
                    }
                    lual_checkany(state, 1);
                }
            },
            | _ => {
                let mut l: usize = 0;
                let mut n: i64 = 0;
                let base: i64 = lual_checkinteger(state, 2);
                (*state).lual_checktype(1, TagType::String);
                let s: *const i8 = lua_tolstring(state, 1, &mut l);
                if !(2_i64..=36_i64).contains(&base) {
                    lual_argerror(state, 2, c"base out of range".as_ptr());
                }
                if b_str2int(s, base as i32, &mut n) == s.add(l) {
                    (*state).push_integer(n);
                    return 1;
                }
            },
        };
        (*state).push_nil();
        1
    }
}
pub unsafe fn luab_error(state: *mut State) -> i32 {
    unsafe {
        let level: i32 = lual_optinteger(state, 2, 1) as i32;
        lua_settop(state, 1);
        if lua_type(state, 1) == Some(TagType::String) && level > 0 {
            lual_where(state, level);
            lua_pushvalue(state, 1);
            lua_concat(state, 2);
        }
        lua_error(state)
    }
}
pub unsafe fn luab_getmetatable(state: *mut State) -> i32 {
    unsafe {
        lual_checkany(state, 1);
        if (*state).lua_getmetatable(1) {
            lual_getmetafield(state, 1, c"__metatable".as_ptr());
            1
        } else {
            (*state).push_nil();
            1
        }
    }
}
pub unsafe fn luab_setmetatable(state: *mut State) -> i32 {
    unsafe {
        (*state).lual_checktype(1, TagType::Table);
        match lua_type(state, 2) {
            | Some(TagType::Nil) | Some(TagType::Table) => {},
            | _ => {
                lual_typeerror(state, 2, c"nil or table".as_ptr());
            },
        };
        if lual_getmetafield(state, 1, c"__metatable".as_ptr()) != TagType::Nil {
            return lual_error(state, c"cannot change a protected metatable".as_ptr(), &[]);
        }
        lua_settop(state, 2);
        lua_setmetatable(state, 1);
        1
    }
}
pub unsafe fn luab_rawequal(state: *mut State) -> i32 {
    unsafe {
        lual_checkany(state, 1);
        lual_checkany(state, 2);
        (*state).push_boolean(lua_rawequal(state, 1, 2));
        1
    }
}
pub unsafe fn luab_rawlen(state: *mut State) -> i32 {
    unsafe {
        match lua_type(state, 1) {
            | Some(TagType::Table) | Some(TagType::String) => {},
            | _ => {
                lual_typeerror(state, 1, c"table or string".as_ptr());
            },
        };
        (*state).push_integer(get_length_raw(state, 1) as i64);
        1
    }
}
pub unsafe fn luab_rawget(state: *mut State) -> i32 {
    unsafe {
        (*state).lual_checktype(1, TagType::Table);
        lual_checkany(state, 2);
        lua_settop(state, 2);
        lua_rawget(state, 1);
        1
    }
}
pub unsafe fn luab_rawset(state: *mut State) -> i32 {
    unsafe {
        (*state).lual_checktype(1, TagType::Table);
        lual_checkany(state, 2);
        lual_checkany(state, 3);
        lua_settop(state, 3);
        lua_rawset(state, 1);
        1
    }
}
pub unsafe fn pushmode(state: *mut State, oldmode: i32) -> i32 {
    unsafe {
        if oldmode == -1 {
            (*state).push_nil();
        } else {
            lua_pushstring(
                state,
                if oldmode == GC_INCREMENTAL {
                    c"incremental".as_ptr()
                } else {
                    c"generational".as_ptr()
                },
            );
        }
        1
    }
}
pub unsafe fn luab_collectgarbage(state: *mut State) -> i32 {
    unsafe {
        pub const OPTS: [*const i8; 12] = [
            c"stop".as_ptr(),
            c"restart".as_ptr(),
            c"collect".as_ptr(),
            c"count".as_ptr(),
            c"step".as_ptr(),
            c"setpause".as_ptr(),
            c"setstepmul".as_ptr(),
            c"isrunning".as_ptr(),
            c"generational".as_ptr(),
            c"incremental".as_ptr(),
            c"param".as_ptr(),
            null(),
        ];
        pub const OPTS_NUMBERS: [i32; 11] = [
            GC_STOP, GC_RESTART, GC_COLLECT, GC_COUNT, GC_STEP, GC_SETPAUSE, GC_SETSTEPMUL, GC_ISRUNNING, GC_GENERATIONAL,
            GC_INCREMENTAL, GC_PARAM,
        ];
        let o: i32 = OPTS_NUMBERS[lual_checkoption(state, 1, c"collect".as_ptr(), OPTS.as_ptr()) as usize];
        match o {
            | GC_COUNT => {
                let k: i32 = lua_gc(state, o, &[]);
                let b: i32 = lua_gc(state, GC_COUNTB, &[]);
                if k != -1 {
                    (*state).push_number(k as f64 + b as f64 / 1024.0);
                    return 1;
                }
            },
            | GC_STEP => {
                let step: i32 = lual_optinteger(state, 2, 0) as i32;
                let res: i32 = lua_gc(state, o, &[step]);
                if res != -1 {
                    (*state).push_boolean(0 != res);
                    return 1;
                }
            },
            | GC_SETPAUSE | GC_SETSTEPMUL => {
                let p: i32 = lual_optinteger(state, 2, 0) as i32;
                let previous: i32 = lua_gc(state, o, &[p]);
                if previous != -1 {
                    (*state).push_integer(previous as i64);
                    return 1;
                }
            },
            | GC_ISRUNNING => {
                let res: i32 = lua_gc(state, o, &[]);
                if res != -1 {
                    (*state).push_boolean(0 != res);
                    return 1;
                }
            },
            | GC_GENERATIONAL => {
                let minormul: i32 = lual_optinteger(state, 2, 0) as i32;
                let majormul: i32 = lual_optinteger(state, 3, 0) as i32;
                return pushmode(state, lua_gc(state, o, &[minormul, majormul]));
            },
            | GC_INCREMENTAL => {
                let pause: i32 = lual_optinteger(state, 2, 0) as i32;
                let stepmul: i32 = lual_optinteger(state, 3, 0) as i32;
                let stepsize: i32 = lual_optinteger(state, 4, 0) as i32;
                return pushmode(state, lua_gc(state, o, &[pause, stepmul, stepsize]));
            },
            | GC_PARAM => {
                // collectgarbage("param", name [, value])
                const PARAM_NAMES: [*const i8; 7] = [
                    c"minormul".as_ptr(),
                    c"majorminor".as_ptr(),
                    c"minormajor".as_ptr(),
                    c"pause".as_ptr(),
                    c"stepmul".as_ptr(),
                    c"stepsize".as_ptr(),
                    null(),
                ];
                let param_idx: i32 = lual_checkoption(state, 2, null(), PARAM_NAMES.as_ptr());
                let value: i32 = lual_optinteger(state, 3, -1) as i32;
                let res: i32 = lua_gc(state, GC_PARAM, &[param_idx, value]);
                if res != -1 {
                    (*state).push_integer(res as i64);
                    return 1;
                }
            },
            | _ => {
                let res: i32 = lua_gc(state, o, &[]);
                if res != -1 {
                    (*state).push_integer(res as i64);
                    return 1;
                }
            },
        }
        (*state).push_nil();
        1
    }
}
pub unsafe fn luab_type(state: *mut State) -> i32 {
    unsafe {
        let t = lua_type(state, 1);
        match t {
            | None => {
                lual_argerror(state, 1, c"value expected".as_ptr());
            },
            | _ => {
                lua_pushstring(state, lua_typename(state, t));
            },
        };
        1
    }
}
pub unsafe fn luab_next(state: *mut State) -> i32 {
    unsafe {
        (*state).lual_checktype(1, TagType::Table);
        lua_settop(state, 2);
        if lua_next(state, 1) != 0 {
            2
        } else {
            (*state).push_nil();
            1
        }
    }
}
pub unsafe fn pairscont(mut _state: *mut State, mut _status: Status, mut _k: i64) -> i32 {
    4
}
pub unsafe fn luab_pairs(state: *mut State) -> i32 {
    unsafe {
        lual_checkany(state, 1);
        if lual_getmetafield(state, 1, c"__pairs".as_ptr()) == TagType::Nil {
            lua_pushcclosure(state, Some(luab_next as unsafe fn(*mut State) -> i32), 0);
            lua_pushvalue(state, 1);
            (*state).push_nil();
            (*state).push_nil();
        } else {
            lua_pushvalue(state, 1);
            (*state).lua_callk(1, 4, 0, Some(pairscont as unsafe fn(*mut State, Status, i64) -> i32));
        }
        4
    }
}
pub unsafe fn ipairsaux(state: *mut State) -> i32 {
    unsafe {
        let mut i: i64 = lual_checkinteger(state, 2);
        i = (i as usize).wrapping_add(1_usize) as i64;
        (*state).push_integer(i);
        if lua_geti(state, 1, i) == TagType::Nil { 1 } else { 2 }
    }
}
pub unsafe fn luab_ipairs(state: *mut State) -> i32 {
    unsafe {
        lual_checkany(state, 1);
        lua_pushcclosure(state, Some(ipairsaux as unsafe fn(*mut State) -> i32), 0);
        lua_pushvalue(state, 1);
        (*state).push_integer(0);
        3
    }
}
pub unsafe fn load_aux(state: *mut State, status: Status, envidx: i32) -> i32 {
    unsafe {
        if status == Status::OK {
            if envidx != 0 {
                lua_pushvalue(state, envidx);
                if (lua_setupvalue(state, -2, 1)).is_null() {
                    lua_settop(state, -2);
                }
            }
            1
        } else {
            (*state).push_nil();
            lua_rotate(state, -2, 1);
            2
        }
    }
}
unsafe fn getmode(state: *mut State, idx: i32) -> *const i8 {
    unsafe {
        let mode: *const i8 = lual_optlstring(state, idx, c"bt".as_ptr(), null_mut());
        if !cstr_chr(mode, b'B' as i8).is_null() {
            lual_argerror(state, idx, c"invalid mode".as_ptr());
        }
        mode
    }
}
pub unsafe fn luab_loadfile(state: *mut State) -> i32 {
    unsafe {
        let fname: *const i8 = lual_optlstring(state, 1, null(), null_mut());
        let mode: *const i8 = getmode(state, 2);
        let env: i32 = if lua_type(state, 3).is_none() { 0 } else { 3 };
        let status = lual_loadfilex(state, fname, mode);
        load_aux(state, status, env)
    }
}
pub unsafe fn generic_reader(state: *mut State, mut _ud: *mut std::ffi::c_void, size: *mut usize) -> *const i8 {
    unsafe {
        lual_checkstack(state, 2, c"too many nested functions".as_ptr());
        lua_pushvalue(state, 1);
        (*state).lua_callk(0, 1, 0, None);
        if lua_type(state, -1) == Some(TagType::Nil) {
            lua_settop(state, -2);
            *size = 0;
            return null();
        } else if !lua_isstring(state, -1) {
            lual_error(state, c"reader function must return a string".as_ptr(), &[]);
        }
        lua_copy(state, -1, 5);
        lua_settop(state, -2);
        lua_tolstring(state, 5, size)
    }
}
pub unsafe fn luab_load(state: *mut State) -> i32 {
    unsafe {
        let status: Status;
        let mut l: usize = 0;
        let s: *const i8 = lua_tolstring(state, 1, &mut l);
        let mode: *const i8 = getmode(state, 3);
        let env: i32 = if lua_type(state, 4).is_some() { 4 } else { 0 };
        if !s.is_null() {
            let chunkname: *const i8 = lual_optlstring(state, 2, s, null_mut());
            status = lual_loadbufferx(state, s, l, chunkname, mode);
        } else {
            let chunkname: *const i8 = lual_optlstring(state, 2, c"=(load)".as_ptr(), null_mut());
            (*state).lual_checktype(1, TagType::Closure);
            lua_settop(state, 5);
            let reader: Reader = Reader::new(Some(
                generic_reader as unsafe fn(*mut State, *mut std::ffi::c_void, *mut usize) -> *const i8,
            ));
            status = lua_load(state, reader, null_mut(), chunkname, mode);
        }
        load_aux(state, status, env)
    }
}
pub unsafe fn dofilecont(state: *mut State, mut _d1: Status, mut _d2: i64) -> i32 {
    unsafe { (*state).get_top() - 1 }
}
pub unsafe fn luab_dofile(state: *mut State) -> i32 {
    unsafe {
        let fname: *const i8 = lual_optlstring(state, 1, null(), null_mut());
        lua_settop(state, 1);
        if lual_loadfilex(state, fname, null()) != Status::OK {
            return lua_error(state);
        }
        (*state).lua_callk(0, -1, 0, Some(dofilecont as unsafe fn(*mut State, Status, i64) -> i32));
        dofilecont(state, Status::OK, 0)
    }
}
pub unsafe fn luab_assert(state: *mut State) -> i32 {
    unsafe {
        if lua_toboolean(state, 1) {
            (*state).get_top()
        } else {
            lual_checkany(state, 1);
            lua_rotate(state, 1, -1);
            lua_settop(state, -2);
            lua_pushstring(state, c"assertion failed!".as_ptr());
            lua_settop(state, 1);
            luab_error(state)
        }
    }
}
pub unsafe fn luab_select(state: *mut State) -> i32 {
    unsafe {
        let n = (*state).get_top();
        if lua_type(state, 1) == Some(TagType::String)
            && *lua_tolstring(state, 1, null_mut()) as i32 == Character::Octothorpe as i32
        {
            (*state).push_integer((n - 1) as i64);
            1
        } else {
            let mut i = lual_checkinteger(state, 1);
            if i < 0 {
                i += n as i64;
            } else if i > n as i64 {
                i = n as i64;
            }
            if 1 > i {
                lual_argerror(state, 1, c"index out of range".as_ptr());
                0;
            }
            n - i as i32
        }
    }
}
pub unsafe fn luab_xpcall(state: *mut State) -> i32 {
    unsafe {
        let n: i32 = (*state).get_top();
        (*state).lual_checktype(2, TagType::Closure);
        (*state).push_boolean(true);
        lua_pushvalue(state, 1);
        lua_rotate(state, 3, 2);
        let status = CallS::api_call(
            state,
            n - 2,
            -1,
            2,
            2_i64,
            Some(finishpcall as unsafe fn(*mut State, Status, i64) -> i32),
        );
        finishpcall(state, status, 2_i64)
    }
}
pub unsafe fn luab_tostring(state: *mut State) -> i32 {
    unsafe {
        lual_checkany(state, 1);
        lual_tolstring(state, 1, null_mut());
        1
    }
}
pub const BASE_FUNCTIONS: [RegisteredFunction; 23] = {
    [
        {
            RegisteredFunction {
                registeredfunction_name: c"assert".as_ptr(),
                registeredfunction_function: Some(luab_assert as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"collectgarbage".as_ptr(),
                registeredfunction_function: Some(luab_collectgarbage as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"dofile".as_ptr(),
                registeredfunction_function: Some(luab_dofile as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"error".as_ptr(),
                registeredfunction_function: Some(luab_error as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"getmetatable".as_ptr(),
                registeredfunction_function: Some(luab_getmetatable as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"ipairs".as_ptr(),
                registeredfunction_function: Some(luab_ipairs as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"loadfile".as_ptr(),
                registeredfunction_function: Some(luab_loadfile as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"load".as_ptr(),
                registeredfunction_function: Some(luab_load as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"next".as_ptr(),
                registeredfunction_function: Some(luab_next as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"pairs".as_ptr(),
                registeredfunction_function: Some(luab_pairs as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"pcall".as_ptr(),
                registeredfunction_function: Some(luab_pcall as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"print".as_ptr(),
                registeredfunction_function: Some(luab_print as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"warn".as_ptr(),
                registeredfunction_function: Some(luab_warn as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"rawequal".as_ptr(),
                registeredfunction_function: Some(luab_rawequal as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"rawlen".as_ptr(),
                registeredfunction_function: Some(luab_rawlen as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"rawget".as_ptr(),
                registeredfunction_function: Some(luab_rawget as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"rawset".as_ptr(),
                registeredfunction_function: Some(luab_rawset as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"select".as_ptr(),
                registeredfunction_function: Some(luab_select as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"setmetatable".as_ptr(),
                registeredfunction_function: Some(luab_setmetatable as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"tonumber".as_ptr(),
                registeredfunction_function: Some(luab_tonumber as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"tostring".as_ptr(),
                registeredfunction_function: Some(luab_tostring as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"type".as_ptr(),
                registeredfunction_function: Some(luab_type as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"xpcall".as_ptr(),
                registeredfunction_function: Some(luab_xpcall as unsafe fn(*mut State) -> i32),
            }
        },
    ]
};
pub unsafe fn luaopen_base(state: *mut State) -> i32 {
    unsafe {
        lua_rawgeti(state, LUA_REGISTRYINDEX, 2_i64);
        lual_setfuncs(state, BASE_FUNCTIONS.as_ptr(), BASE_FUNCTIONS.len(), 0);
        lua_pushvalue(state, -1);
        lua_setfield(state, -2, c"_G".as_ptr());
        lua_pushstring(state, c"Lua 5.5".as_ptr());
        lua_setfield(state, -2, c"_VERSION".as_ptr());
        1
    }
}
