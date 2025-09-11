#![allow(unpredictable_function_pointer_comparisons,unsafe_code)]
use std::ptr::*;
use crate::f2i::*;
use crate::registeredfunction::*;
use crate::interpreter::*;
use crate::character::*;
use crate::tag::*;
pub unsafe extern "C" fn luab_tonumber(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        match lua_type(interpreter, 2) {
            None | Some(TagType::Nil) => {
                if lua_type(interpreter, 1) == Some(TagType::Numeric) {
                    lua_settop(interpreter, 1);
                    return 1;
                } else {
                    let mut l: u64 = 0;
                    let s: *const i8 = lua_tolstring(interpreter, 1, &mut l);
                    if !s.is_null() && lua_stringtonumber(interpreter, s) == l.wrapping_add(1 as u64) {
                        return 1;
                    }
                    lual_checkany(interpreter, 1);
                }
            },
            _ => {
                let mut l_0: u64 = 0;
                let s_0: *const i8;
                let mut n: i64 = 0;
                let base: i64 = lual_checkinteger(interpreter, 2);
                lual_checktype(interpreter, 1, TagType::String);
                s_0 = lua_tolstring(interpreter, 1, &mut l_0);
                (((2 as i64 <= base && base <= 36 as i64) as i32 != 0) as i64 != 0
                    || lual_argerror(interpreter, 2, b"base out of range\0" as *const u8 as *const i8) != 0)
                    as i32;
                if b_str2int(s_0, base as i32, &mut n) == s_0.offset(l_0 as isize) {
                    (*interpreter).push_integer(n);
                    return 1;
                }
            }
        };
        (*interpreter).push_nil();
        return 1;
    }
}
pub unsafe extern "C" fn luab_error(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let level: i32 = lual_optinteger(interpreter, 2, 1) as i32;
        lua_settop(interpreter, 1);
        if lua_type(interpreter, 1) == Some(TagType::String) && level > 0 {
            lual_where(interpreter, level);
            lua_pushvalue(interpreter, 1);
            lua_concat(interpreter, 2);
        }
        return lua_error(interpreter);
    }
}
pub unsafe extern "C" fn luab_getmetatable(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        lual_checkany(interpreter, 1);
        if (*interpreter).lua_getmetatable(1) {
            lual_getmetafield(interpreter, 1, b"__metatable\0" as *const u8 as *const i8);
            return 1;
        } else {
            (*interpreter).push_nil();
            return 1;
        }
    }
}
pub unsafe extern "C" fn luab_setmetatable(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        lual_checktype(interpreter, 1, TagType::Table);
        match lua_type(interpreter, 2) {
            Some(TagType::Nil) | Some(TagType::Table) => {
            },
            _ => {
                lual_typeerror(interpreter, 2, b"nil or table\0" as *const u8 as *const i8);
            },
        };
        if lual_getmetafield(interpreter, 1, b"__metatable\0" as *const u8 as *const i8) != TagType::Nil {
            return lual_error(
                interpreter,
                b"cannot change a protected metatable\0".as_ptr()
            );
        }
        lua_settop(interpreter, 2);
        lua_setmetatable(interpreter, 1);
        return 1;
    }
}
pub unsafe extern "C" fn luab_rawequal(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        lual_checkany(interpreter, 1);
        lual_checkany(interpreter, 2);
        (*interpreter).push_boolean(lua_rawequal(interpreter, 1, 2));
        return 1;
    }
}
pub unsafe extern "C" fn luab_rawlen(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        match lua_type(interpreter, 1) {
            Some(TagType::Table) | Some(TagType::String) => {
            },
            _ => {
                lual_typeerror(interpreter, 1, b"table or string\0" as *const u8 as *const i8);
            },
        };
        (*interpreter).push_integer(get_length_raw(interpreter, 1) as u64 as i64);
        return 1;
    }
}
pub unsafe extern "C" fn luab_rawget(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        lual_checktype(interpreter, 1, TagType::Table);
        lual_checkany(interpreter, 2);
        lua_settop(interpreter, 2);
        lua_rawget(interpreter, 1);
        return 1;
    }
}
pub unsafe extern "C" fn luab_rawset(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        lual_checktype(interpreter, 1, TagType::Table);
        lual_checkany(interpreter, 2);
        lual_checkany(interpreter, 3);
        lua_settop(interpreter, 3);
        lua_rawset(interpreter, 1);
        return 1;
    }
}
pub unsafe extern "C" fn pushmode(interpreter: *mut Interpreter, oldmode: i32) -> i32 {
    unsafe {
        if oldmode == -1 {
            (*interpreter).push_nil();
        } else {
            lua_pushstring(
                interpreter,
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
pub unsafe extern "C" fn luab_collectgarbage(interpreter: *mut Interpreter) -> i32 {
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
            null(),
        ];
        pub const OPTS_NUMBERS: [i32; 10] = [0, 1, 2, 3, 5, 6, 7, 9 as i32, 10 as i32, 11 as i32];
        let o: i32 = OPTS_NUMBERS[lual_checkoption(
            interpreter,
            1,
            b"collect\0" as *const u8 as *const i8,
            OPTS.as_ptr(),
        ) as usize];
        match o {
            3 => {
                let k: i32 = lua_gc(interpreter, o);
                let b: i32 = lua_gc(interpreter, 4);
                if !(k == -1) {
                    (*interpreter).push_number(k as f64 + b as f64 / 1024.0);
                    return 1;
                }
            }
            5 => {
                let step: i32 = lual_optinteger(interpreter, 2, 0) as i32;
                let res: i32 = lua_gc(interpreter, o, step);
                if !(res == -1) {
                    (*interpreter).push_boolean(0 != res);
                    return 1;
                }
            }
            6 | 7 => {
                let p: i32 = lual_optinteger(interpreter, 2, 0) as i32;
                let previous: i32 = lua_gc(interpreter, o, p);
                if !(previous == -1) {
                    (*interpreter).push_integer(previous as i64);
                    return 1;
                }
            }
            9 => {
                let res_0: i32 = lua_gc(interpreter, o);
                if !(res_0 == -1) {
                    (*interpreter).push_boolean(0 != res_0);
                    return 1;
                }
            }
            10 => {
                let minormul: i32 = lual_optinteger(interpreter, 2, 0) as i32;
                let majormul: i32 = lual_optinteger(interpreter, 3, 0) as i32;
                return pushmode(interpreter, lua_gc(interpreter, o, minormul, majormul));
            }
            11 => {
                let pause: i32 = lual_optinteger(interpreter, 2, 0) as i32;
                let stepmul: i32 = lual_optinteger(interpreter, 3, 0) as i32;
                let stepsize: i32 = lual_optinteger(interpreter, 4, 0) as i32;
                return pushmode(interpreter, lua_gc(interpreter, o, pause, stepmul, stepsize));
            }
            _ => {
                let res_1: i32 = lua_gc(interpreter, o);
                if !(res_1 == -1) {
                    (*interpreter).push_integer(res_1 as i64);
                    return 1;
                }
            }
        }
        (*interpreter).push_nil();
        return 1;
    }
}
pub unsafe extern "C" fn luab_type(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let t = lua_type(interpreter, 1);
        match t {
            None => {
                lual_argerror(interpreter, 1, b"value expected\0" as *const u8 as *const i8);
            },
            _  => {
                lua_pushstring(interpreter, lua_typename(interpreter, t));
            },
        };
        return 1;
    }
}
pub unsafe extern "C" fn luab_next(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        lual_checktype(interpreter, 1, TagType::Table);
        lua_settop(interpreter, 2);
        if lua_next(interpreter, 1) != 0 {
            return 2;
        } else {
            (*interpreter).push_nil();
            return 1;
        };
    }
}
pub unsafe extern "C" fn pairscont(mut _state: *mut Interpreter, mut _status: i32, mut _k: i64) -> i32 {
    return 3;
}
pub unsafe extern "C" fn luab_pairs(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        lual_checkany(interpreter, 1);
        if lual_getmetafield(interpreter, 1, b"__pairs\0" as *const u8 as *const i8) == TagType::Nil {
            lua_pushcclosure(
                interpreter,
                Some(luab_next as unsafe extern "C" fn(*mut Interpreter) -> i32),
                0,
            );
            lua_pushvalue(interpreter, 1);
            (*interpreter).push_nil();
        } else {
            lua_pushvalue(interpreter, 1);
            lua_callk(
                interpreter,
                1,
                3,
                0,
                Some(pairscont as unsafe extern "C" fn(*mut Interpreter, i32, i64) -> i32),
            );
        }
        return 3;
    }
}
pub unsafe extern "C" fn ipairsaux(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let mut i: i64 = lual_checkinteger(interpreter, 2);
        i = (i as u64).wrapping_add(1 as u64) as i64;
        (*interpreter).push_integer(i);
        return if lua_geti(interpreter, 1, i) == TagType::Nil { 1 } else { 2 };
    }
}
pub unsafe extern "C" fn luab_ipairs(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        lual_checkany(interpreter, 1);
        lua_pushcclosure(
            interpreter,
            Some(ipairsaux as unsafe extern "C" fn(*mut Interpreter) -> i32),
            0,
        );
        lua_pushvalue(interpreter, 1);
        (*interpreter).push_integer(0);
        return 3;
    }
}
pub unsafe extern "C" fn load_aux(interpreter: *mut Interpreter, status: i32, envidx: i32) -> i32 {
    unsafe {
        if status == 0 {
            if envidx != 0 {
                lua_pushvalue(interpreter, envidx);
                if (lua_setupvalue(interpreter, -2, 1)).is_null() {
                    lua_settop(interpreter, -2);
                }
            }
            return 1;
        } else {
            (*interpreter).push_nil();
            lua_rotate(interpreter, -2, 1);
            return 2;
        };
    }
}
pub unsafe extern "C" fn luab_loadfile(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let fname: *const i8 = lual_optlstring(interpreter, 1, null(), null_mut());
        let mode: *const i8 = lual_optlstring(interpreter, 2, null(), null_mut());
        let env: i32 = if lua_type(interpreter, 3) == None { 0 } else { 3 };
        let status: i32 = lual_loadfilex(interpreter, fname, mode);
        return load_aux(interpreter, status, env);
    }
}
pub unsafe extern "C" fn generic_reader(
    interpreter: *mut Interpreter,
    mut _ud: *mut libc::c_void,
    size: *mut u64,
) -> *const i8 {
    unsafe {
        lual_checkstack(
            interpreter,
            2,
            b"too many nested functions\0" as *const u8 as *const i8,
        );
        lua_pushvalue(interpreter, 1);
        lua_callk(interpreter, 0, 1, 0, None);
        if lua_type(interpreter, -1) == Some(TagType::Nil) {
            lua_settop(interpreter, -2);
            *size = 0;
            return null();
        } else if !lua_isstring(interpreter, -1) {
            lual_error(
                interpreter,
                b"reader function must return a string\0".as_ptr(),
            );
        }
        lua_copy(interpreter, -1, 5);
        lua_settop(interpreter, -2);
        return lua_tolstring(interpreter, 5, size);
    }
}
pub unsafe extern "C" fn luab_load(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let status: i32;
        let mut l: u64 = 0;
        let s: *const i8 = lua_tolstring(interpreter, 1, &mut l);
        let mode: *const i8 = lual_optlstring(
            interpreter,
            3,
            b"bt\0" as *const u8 as *const i8,
            null_mut(),
        );
        let env: i32 = if !(lua_type(interpreter, 4) == None) { 4 } else { 0 };
        if !s.is_null() {
            let chunkname: *const i8 = lual_optlstring(interpreter, 2, s, null_mut());
            status = lual_loadbufferx(interpreter, s, l, chunkname, mode);
        } else {
            let chunkname_0: *const i8 = lual_optlstring(
                interpreter,
                2,
                b"=(load)\0" as *const u8 as *const i8,
                null_mut(),
            );
            lual_checktype(interpreter, 1, TagType::Closure);
            lua_settop(interpreter, 5);
            status = lua_load(
                interpreter,
                Some(
                    generic_reader
                        as unsafe extern "C" fn(
                            *mut Interpreter,
                            *mut libc::c_void,
                            *mut u64,
                        ) -> *const i8,
                ),
                null_mut(),
                chunkname_0,
                mode,
            );
        }
        return load_aux(interpreter, status, env);
    }
}
pub unsafe extern "C" fn dofilecont(interpreter: *mut Interpreter, mut _d1: i32, mut _d2: i64) -> i32 {
    unsafe {
        return (*interpreter).get_top() - 1;
    }
}
pub unsafe extern "C" fn luab_dofile(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let fname: *const i8 = lual_optlstring(interpreter, 1, null(), null_mut());
        lua_settop(interpreter, 1);
        if lual_loadfilex(interpreter, fname, null()) != 0 {
            return lua_error(interpreter);
        }
        lua_callk(
            interpreter,
            0,
            -1,
            0,
            Some(dofilecont as unsafe extern "C" fn(*mut Interpreter, i32, i64) -> i32),
        );
        return dofilecont(interpreter, 0, 0);
    }
}
pub unsafe extern "C" fn luab_assert(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        if (lua_toboolean(interpreter, 1) != 0) as i64 != 0 {
            return (*interpreter).get_top();
        } else {
            lual_checkany(interpreter, 1);
            lua_rotate(interpreter, 1, -1);
            lua_settop(interpreter, -2);
            lua_pushstring(interpreter, b"assertion failed!\0" as *const u8 as *const i8);
            lua_settop(interpreter, 1);
            return luab_error(interpreter);
        };
    }
}
pub unsafe extern "C" fn luab_select(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let n = (*interpreter).get_top();
        if lua_type(interpreter, 1) == Some(TagType::String)
            && *lua_tolstring(interpreter, 1, null_mut()) as i32 == CHARACTER_OCTOTHORPE as i32
        {
            (*interpreter).push_integer((n - 1) as i64);
            return 1;
        } else {
            let mut i = lual_checkinteger(interpreter, 1);
            if i < 0 {
                i = n as i64 + i;
            } else if i > n as i64 {
                i = n as i64;
            }
            (((1 <= i) as i32 != 0) as i64 != 0
                || lual_argerror(interpreter, 1, b"index out of range\0" as *const u8 as *const i8) != 0)
                as i32;
            return n - i as i32;
        };
    }
}
pub unsafe extern "C" fn luab_xpcall(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let status: i32;
        let n: i32 = (*interpreter).get_top();
        lual_checktype(interpreter, 2, TagType::Closure);
        (*interpreter).push_boolean(true);
        lua_pushvalue(interpreter, 1);
        lua_rotate(interpreter, 3, 2);
        status = lua_pcallk(
            interpreter,
            n - 2,
            -1,
            2,
            2 as i64,
            Some(finishpcall as unsafe extern "C" fn(*mut Interpreter, i32, i64) -> i32),
        );
        return finishpcall(interpreter, status, 2 as i64);
    }
}
pub unsafe extern "C" fn luab_tostring(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        lual_checkany(interpreter, 1);
        lual_tolstring(interpreter, 1, null_mut());
        return 1;
    }
}
pub const BASE_FUNCTIONS: [RegisteredFunction; 26] = {
    [
        {
            RegisteredFunction {
                name: b"assert\0" as *const u8 as *const i8,
                function: Some(luab_assert as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"collectgarbage\0" as *const u8 as *const i8,
                function: Some(luab_collectgarbage as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"dofile\0" as *const u8 as *const i8,
                function: Some(luab_dofile as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"error\0" as *const u8 as *const i8,
                function: Some(luab_error as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"getmetatable\0" as *const u8 as *const i8,
                function: Some(luab_getmetatable as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"ipairs\0" as *const u8 as *const i8,
                function: Some(luab_ipairs as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"loadfile\0" as *const u8 as *const i8,
                function: Some(luab_loadfile as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"load\0" as *const u8 as *const i8,
                function: Some(luab_load as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"next\0" as *const u8 as *const i8,
                function: Some(luab_next as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"pairs\0" as *const u8 as *const i8,
                function: Some(luab_pairs as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"pcall\0" as *const u8 as *const i8,
                function: Some(luab_pcall as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"print\0" as *const u8 as *const i8,
                function: Some(luab_print as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"warn\0" as *const u8 as *const i8,
                function: Some(luab_warn as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"rawequal\0" as *const u8 as *const i8,
                function: Some(luab_rawequal as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"rawlen\0" as *const u8 as *const i8,
                function: Some(luab_rawlen as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"rawget\0" as *const u8 as *const i8,
                function: Some(luab_rawget as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"rawset\0" as *const u8 as *const i8,
                function: Some(luab_rawset as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"select\0" as *const u8 as *const i8,
                function: Some(luab_select as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"setmetatable\0" as *const u8 as *const i8,
                function: Some(luab_setmetatable as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"tonumber\0" as *const u8 as *const i8,
                function: Some(luab_tonumber as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"tostring\0" as *const u8 as *const i8,
                function: Some(luab_tostring as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"type\0" as *const u8 as *const i8,
                function: Some(luab_type as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"xpcall\0" as *const u8 as *const i8,
                function: Some(luab_xpcall as unsafe extern "C" fn(*mut Interpreter) -> i32),
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
                name: null(),
                function: None,
            }
        },
    ]
};
pub unsafe extern "C" fn luaopen_base(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        lua_rawgeti(interpreter, -(1000000 as i32) - 1000 as i32, 2 as i64);
        lual_setfuncs(interpreter, BASE_FUNCTIONS.as_ptr(), 0);
        lua_pushvalue(interpreter, -1);
        lua_setfield(interpreter, -2, b"_G\0" as *const u8 as *const i8);
        lua_pushstring(interpreter, b"Lua 5.4\0" as *const u8 as *const i8);
        lua_setfield(interpreter, -2, b"_VERSION\0" as *const u8 as *const i8);
        return 1;
    }
}
