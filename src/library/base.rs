#![allow(unpredictable_function_pointer_comparisons, unsafe_code)]
use crate::status::*;
use crate::calls::*;
use crate::character::*;
use crate::f2i::*;
use crate::functions::*;
use crate::global::*;
use crate::interpreter::*;
use crate::registeredfunction::*;
use crate::tag::*;
use std::ptr::*;
pub unsafe fn luab_tonumber(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        match lua_type(interpreter, 2) {
            None | Some(TagType::Nil) => {
                if lua_type(interpreter, 1) == Some(TagType::Numeric) {
                    lua_settop(interpreter, 1);
                    return 1;
                } else {
                    let mut l: usize = 0;
                    let s: *const i8 = lua_tolstring(interpreter, 1, &mut l);
                    if !s.is_null() && lua_stringtonumber(interpreter, s) == l.wrapping_add(1 as usize) {
                        return 1;
                    }
                    lual_checkany(interpreter, 1);
                }
            },
            _ => {
                let mut l_0: usize = 0;
                let s_0: *const i8;
                let mut n: i64 = 0;
                let base: i64 = lual_checkinteger(interpreter, 2);
                (*interpreter).lual_checktype(1, TagType::String);
                s_0 = lua_tolstring(interpreter, 1, &mut l_0);
                (((2 as i64 <= base && base <= 36 as i64) as i32 != 0) as i64 != 0 || lual_argerror(interpreter, 2, c"base out of range".as_ptr()) != 0) as i32;
                if b_str2int(s_0, base as i32, &mut n) == s_0.offset(l_0 as isize) {
                    (*interpreter).push_integer(n);
                    return 1;
                }
            },
        };
        (*interpreter).push_nil();
        return 1;
    }
}
pub unsafe fn luab_error(interpreter: *mut Interpreter) -> i32 {
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
pub unsafe fn luab_getmetatable(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        lual_checkany(interpreter, 1);
        if (*interpreter).lua_getmetatable(1) {
            lual_getmetafield(interpreter, 1, c"__metatable".as_ptr());
            return 1;
        } else {
            (*interpreter).push_nil();
            return 1;
        }
    }
}
pub unsafe fn luab_setmetatable(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        (*interpreter).lual_checktype(1, TagType::Table);
        match lua_type(interpreter, 2) {
            Some(TagType::Nil) | Some(TagType::Table) => {},
            _ => {
                lual_typeerror(interpreter, 2, c"nil or table".as_ptr());
            },
        };
        if lual_getmetafield(interpreter, 1, c"__metatable".as_ptr()) != TagType::Nil {
            return lual_error(interpreter, c"cannot change a protected metatable".as_ptr());
        }
        lua_settop(interpreter, 2);
        lua_setmetatable(interpreter, 1);
        return 1;
    }
}
pub unsafe fn luab_rawequal(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        lual_checkany(interpreter, 1);
        lual_checkany(interpreter, 2);
        (*interpreter).push_boolean(lua_rawequal(interpreter, 1, 2));
        return 1;
    }
}
pub unsafe fn luab_rawlen(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        match lua_type(interpreter, 1) {
            Some(TagType::Table) | Some(TagType::String) => {},
            _ => {
                lual_typeerror(interpreter, 1, c"table or string".as_ptr());
            },
        };
        (*interpreter).push_integer(get_length_raw(interpreter, 1) as usize as i64);
        return 1;
    }
}
pub unsafe fn luab_rawget(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        (*interpreter).lual_checktype(1, TagType::Table);
        lual_checkany(interpreter, 2);
        lua_settop(interpreter, 2);
        lua_rawget(interpreter, 1);
        return 1;
    }
}
pub unsafe fn luab_rawset(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        (*interpreter).lual_checktype(1, TagType::Table);
        lual_checkany(interpreter, 2);
        lual_checkany(interpreter, 3);
        lua_settop(interpreter, 3);
        lua_rawset(interpreter, 1);
        return 1;
    }
}
pub unsafe fn pushmode(interpreter: *mut Interpreter, oldmode: i32) -> i32 {
    unsafe {
        if oldmode == -1 {
            (*interpreter).push_nil();
        } else {
            lua_pushstring(interpreter, if oldmode == 11 as i32 { c"incremental".as_ptr() } else { c"generational".as_ptr() });
        }
        return 1;
    }
}
pub unsafe fn luab_collectgarbage(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        pub const OPTS: [*const i8; 11] = [
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
            null(),
        ];
        pub const OPTS_NUMBERS: [i32; 10] = [0, 1, 2, 3, 5, 6, 7, 9 as i32, 10 as i32, 11 as i32];
        let o: i32 = OPTS_NUMBERS[lual_checkoption(interpreter, 1, c"collect".as_ptr(), OPTS.as_ptr()) as usize];
        match o {
            3 => {
                let k: i32 = lua_gc(interpreter, o);
                let b: i32 = lua_gc(interpreter, 4);
                if !(k == -1) {
                    (*interpreter).push_number(k as f64 + b as f64 / 1024.0);
                    return 1;
                }
            },
            5 => {
                let step: i32 = lual_optinteger(interpreter, 2, 0) as i32;
                let res: i32 = lua_gc(interpreter, o, step);
                if !(res == -1) {
                    (*interpreter).push_boolean(0 != res);
                    return 1;
                }
            },
            6 | 7 => {
                let p: i32 = lual_optinteger(interpreter, 2, 0) as i32;
                let previous: i32 = lua_gc(interpreter, o, p);
                if !(previous == -1) {
                    (*interpreter).push_integer(previous as i64);
                    return 1;
                }
            },
            9 => {
                let res_0: i32 = lua_gc(interpreter, o);
                if !(res_0 == -1) {
                    (*interpreter).push_boolean(0 != res_0);
                    return 1;
                }
            },
            10 => {
                let minormul: i32 = lual_optinteger(interpreter, 2, 0) as i32;
                let majormul: i32 = lual_optinteger(interpreter, 3, 0) as i32;
                return pushmode(interpreter, lua_gc(interpreter, o, minormul, majormul));
            },
            11 => {
                let pause: i32 = lual_optinteger(interpreter, 2, 0) as i32;
                let stepmul: i32 = lual_optinteger(interpreter, 3, 0) as i32;
                let stepsize: i32 = lual_optinteger(interpreter, 4, 0) as i32;
                return pushmode(interpreter, lua_gc(interpreter, o, pause, stepmul, stepsize));
            },
            _ => {
                let res_1: i32 = lua_gc(interpreter, o);
                if !(res_1 == -1) {
                    (*interpreter).push_integer(res_1 as i64);
                    return 1;
                }
            },
        }
        (*interpreter).push_nil();
        return 1;
    }
}
pub unsafe fn luab_type(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let t = lua_type(interpreter, 1);
        match t {
            None => {
                lual_argerror(interpreter, 1, c"value expected".as_ptr());
            },
            _ => {
                lua_pushstring(interpreter, lua_typename(interpreter, t));
            },
        };
        return 1;
    }
}
pub unsafe fn luab_next(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        (*interpreter).lual_checktype(1, TagType::Table);
        lua_settop(interpreter, 2);
        if lua_next(interpreter, 1) != 0 {
            return 2;
        } else {
            (*interpreter).push_nil();
            return 1;
        };
    }
}
pub unsafe fn pairscont(mut _state: *mut Interpreter, mut _status: Status, mut _k: i64) -> i32 {
    return 3;
}
pub unsafe fn luab_pairs(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        lual_checkany(interpreter, 1);
        if lual_getmetafield(interpreter, 1, c"__pairs".as_ptr()) == TagType::Nil {
            lua_pushcclosure(interpreter, Some(luab_next as unsafe fn(*mut Interpreter) -> i32), 0);
            lua_pushvalue(interpreter, 1);
            (*interpreter).push_nil();
        } else {
            lua_pushvalue(interpreter, 1);
            (*interpreter).lua_callk(1, 3, 0, Some(pairscont as unsafe fn(*mut Interpreter, Status, i64) -> i32));
        }
        return 3;
    }
}
pub unsafe fn ipairsaux(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let mut i: i64 = lual_checkinteger(interpreter, 2);
        i = (i as usize).wrapping_add(1 as usize) as i64;
        (*interpreter).push_integer(i);
        return if lua_geti(interpreter, 1, i) == TagType::Nil { 1 } else { 2 };
    }
}
pub unsafe fn luab_ipairs(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        lual_checkany(interpreter, 1);
        lua_pushcclosure(interpreter, Some(ipairsaux as unsafe fn(*mut Interpreter) -> i32), 0);
        lua_pushvalue(interpreter, 1);
        (*interpreter).push_integer(0);
        return 3;
    }
}
pub unsafe fn load_aux(interpreter: *mut Interpreter, status: Status, envidx: i32) -> i32 {
    unsafe {
        if status == Status::OK {
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
pub unsafe fn luab_loadfile(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let fname: *const i8 = lual_optlstring(interpreter, 1, null(), null_mut());
        let mode: *const i8 = lual_optlstring(interpreter, 2, null(), null_mut());
        let env: i32 = if lua_type(interpreter, 3) == None { 0 } else { 3 };
        let status = lual_loadfilex(interpreter, fname, mode);
        return load_aux(interpreter, status, env);
    }
}
pub unsafe fn generic_reader(interpreter: *mut Interpreter, mut _ud: *mut libc::c_void, size: *mut usize) -> *const i8 {
    unsafe {
        lual_checkstack(interpreter, 2, c"too many nested functions".as_ptr());
        lua_pushvalue(interpreter, 1);
        (*interpreter).lua_callk(0, 1, 0, None);
        if lua_type(interpreter, -1) == Some(TagType::Nil) {
            lua_settop(interpreter, -2);
            *size = 0;
            return null();
        } else if !lua_isstring(interpreter, -1) {
            lual_error(interpreter, c"reader function must return a string".as_ptr());
        }
        lua_copy(interpreter, -1, 5);
        lua_settop(interpreter, -2);
        return lua_tolstring(interpreter, 5, size);
    }
}
pub unsafe fn luab_load(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let status: Status;
        let mut l: usize = 0;
        let s: *const i8 = lua_tolstring(interpreter, 1, &mut l);
        let mode: *const i8 = lual_optlstring(interpreter, 3, c"bt".as_ptr(), null_mut());
        let env: i32 = if !(lua_type(interpreter, 4) == None) { 4 } else { 0 };
        if !s.is_null() {
            let chunkname: *const i8 = lual_optlstring(interpreter, 2, s, null_mut());
            status = lual_loadbufferx(interpreter, s, l, chunkname, mode);
        } else {
            let chunkname_0: *const i8 = lual_optlstring(interpreter, 2, c"=(load)".as_ptr(), null_mut());
            (*interpreter).lual_checktype(1, TagType::Closure);
            lua_settop(interpreter, 5);
            let reader: Reader = Reader::new(Some(generic_reader as unsafe fn(*mut Interpreter, *mut libc::c_void, *mut usize) -> *const i8));
            status = lua_load(
                interpreter,
                reader,
                null_mut(),
                chunkname_0,
                mode,
            );
        }
        return load_aux(interpreter, status, env);
    }
}
pub unsafe fn dofilecont(interpreter: *mut Interpreter, mut _d1: Status, mut _d2: i64) -> i32 {
    unsafe {
        return (*interpreter).get_top() - 1;
    }
}
pub unsafe fn luab_dofile(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let fname: *const i8 = lual_optlstring(interpreter, 1, null(), null_mut());
        lua_settop(interpreter, 1);
        if lual_loadfilex(interpreter, fname, null()) != Status::OK {
            return lua_error(interpreter);
        }
        (*interpreter).lua_callk(0, -1, 0, Some(dofilecont as unsafe fn(*mut Interpreter, Status, i64) -> i32));
        return dofilecont(interpreter, Status::OK, 0);
    }
}
pub unsafe fn luab_assert(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        if (lua_toboolean(interpreter, 1) != 0) as i64 != 0 {
            return (*interpreter).get_top();
        } else {
            lual_checkany(interpreter, 1);
            lua_rotate(interpreter, 1, -1);
            lua_settop(interpreter, -2);
            lua_pushstring(interpreter, c"assertion failed!".as_ptr());
            lua_settop(interpreter, 1);
            return luab_error(interpreter);
        };
    }
}
pub unsafe fn luab_select(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let n = (*interpreter).get_top();
        if lua_type(interpreter, 1) == Some(TagType::String) && *lua_tolstring(interpreter, 1, null_mut()) as i32 == Character::Octothorpe as i32 {
            (*interpreter).push_integer((n - 1) as i64);
            return 1;
        } else {
            let mut i = lual_checkinteger(interpreter, 1);
            if i < 0 {
                i = n as i64 + i;
            } else if i > n as i64 {
                i = n as i64;
            }
            (((1 <= i) as i32 != 0) as i64 != 0 || lual_argerror(interpreter, 1, c"index out of range".as_ptr()) != 0) as i32;
            return n - i as i32;
        };
    }
}
pub unsafe fn luab_xpcall(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let n: i32 = (*interpreter).get_top();
        (*interpreter).lual_checktype(2, TagType::Closure);
        (*interpreter).push_boolean(true);
        lua_pushvalue(interpreter, 1);
        lua_rotate(interpreter, 3, 2);
        let status = CallS::api_call(interpreter, n - 2, -1, 2, 2 as i64, Some(finishpcall as unsafe fn(*mut Interpreter, Status, i64) -> i32));
        return finishpcall(interpreter, status, 2 as i64);
    }
}
pub unsafe fn luab_tostring(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        lual_checkany(interpreter, 1);
        lual_tolstring(interpreter, 1, null_mut());
        return 1;
    }
}
pub const BASE_FUNCTIONS: [RegisteredFunction; 23] = {
    [
        { RegisteredFunction { name: c"assert".as_ptr(), function: Some(luab_assert as unsafe fn(*mut Interpreter) -> i32) } },
        { RegisteredFunction { name: c"collectgarbage".as_ptr(), function: Some(luab_collectgarbage as unsafe fn(*mut Interpreter) -> i32) } },
        { RegisteredFunction { name: c"dofile".as_ptr(), function: Some(luab_dofile as unsafe fn(*mut Interpreter) -> i32) } },
        { RegisteredFunction { name: c"error".as_ptr(), function: Some(luab_error as unsafe fn(*mut Interpreter) -> i32) } },
        { RegisteredFunction { name: c"getmetatable".as_ptr(), function: Some(luab_getmetatable as unsafe fn(*mut Interpreter) -> i32) } },
        { RegisteredFunction { name: c"ipairs".as_ptr(), function: Some(luab_ipairs as unsafe fn(*mut Interpreter) -> i32) } },
        { RegisteredFunction { name: c"loadfile".as_ptr(), function: Some(luab_loadfile as unsafe fn(*mut Interpreter) -> i32) } },
        { RegisteredFunction { name: c"load".as_ptr(), function: Some(luab_load as unsafe fn(*mut Interpreter) -> i32) } },
        { RegisteredFunction { name: c"next".as_ptr(), function: Some(luab_next as unsafe fn(*mut Interpreter) -> i32) } },
        { RegisteredFunction { name: c"pairs".as_ptr(), function: Some(luab_pairs as unsafe fn(*mut Interpreter) -> i32) } },
        { RegisteredFunction { name: c"pcall".as_ptr(), function: Some(luab_pcall as unsafe fn(*mut Interpreter) -> i32) } },
        { RegisteredFunction { name: c"print".as_ptr(), function: Some(luab_print as unsafe fn(*mut Interpreter) -> i32) } },
        { RegisteredFunction { name: c"warn".as_ptr(), function: Some(luab_warn as unsafe fn(*mut Interpreter) -> i32) } },
        { RegisteredFunction { name: c"rawequal".as_ptr(), function: Some(luab_rawequal as unsafe fn(*mut Interpreter) -> i32) } },
        { RegisteredFunction { name: c"rawlen".as_ptr(), function: Some(luab_rawlen as unsafe fn(*mut Interpreter) -> i32) } },
        { RegisteredFunction { name: c"rawget".as_ptr(), function: Some(luab_rawget as unsafe fn(*mut Interpreter) -> i32) } },
        { RegisteredFunction { name: c"rawset".as_ptr(), function: Some(luab_rawset as unsafe fn(*mut Interpreter) -> i32) } },
        { RegisteredFunction { name: c"select".as_ptr(), function: Some(luab_select as unsafe fn(*mut Interpreter) -> i32) } },
        { RegisteredFunction { name: c"setmetatable".as_ptr(), function: Some(luab_setmetatable as unsafe fn(*mut Interpreter) -> i32) } },
        { RegisteredFunction { name: c"tonumber".as_ptr(), function: Some(luab_tonumber as unsafe fn(*mut Interpreter) -> i32) } },
        { RegisteredFunction { name: c"tostring".as_ptr(), function: Some(luab_tostring as unsafe fn(*mut Interpreter) -> i32) } },
        { RegisteredFunction { name: c"type".as_ptr(), function: Some(luab_type as unsafe fn(*mut Interpreter) -> i32) } },
        { RegisteredFunction { name: c"xpcall".as_ptr(), function: Some(luab_xpcall as unsafe fn(*mut Interpreter) -> i32) } },
    ]
};
pub unsafe fn luaopen_base(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        lua_rawgeti(interpreter, -(1000000 as i32) - 1000 as i32, 2 as i64);
        lual_setfuncs(interpreter, BASE_FUNCTIONS.as_ptr(), BASE_FUNCTIONS.len(), 0);
        lua_pushvalue(interpreter, -1);
        lua_setfield(interpreter, -2, c"_G".as_ptr());
        lua_pushstring(interpreter, c"Lua 5.4".as_ptr());
        lua_setfield(interpreter, -2, c"_VERSION".as_ptr());
        return 1;
    }
}
