#![allow(unpredictable_function_pointer_comparisons)]
use crate::registeredfunction::*;
use crate::state::*;
use crate::tagtype::*;

const UPVALUE1: i32 = LUA_REGISTRYINDEX - 1;
const UPVALUE2: i32 = LUA_REGISTRYINDEX - 2;

/// The closure returned by functools.partial(f, ...).
/// Upvalue 1 = f, upvalue 2 = table of bound args.
unsafe fn partial_call(state: *mut State) -> i32 {
    unsafe {
        let new_args = (*state).get_top();
        let bound_table = UPVALUE2;
        let nbound = get_length_raw(state, bound_table) as i32;
        let base = (*state).get_top();
        // push f
        lua_pushvalue(state, UPVALUE1);
        // push bound args
        for i in 1..=nbound {
            lua_rawgeti(state, bound_table, i as i64);
        }
        // push new args
        for i in 1..=new_args {
            lua_pushvalue(state, i);
        }
        // call f(bound..., new...)
        (*state).lua_callk(nbound + new_args, -1, 0, None);
        (*state).get_top() - base
    }
}

/// functools.partial(f, ...) → closure
unsafe fn ft_partial(state: *mut State) -> i32 {
    unsafe {
        (*state).lual_checktype(1, TagType::Closure);
        let nargs = (*state).get_top();
        // push f as upvalue 1
        lua_pushvalue(state, 1);
        // create table of bound args as upvalue 2
        (*state).lua_createtable();
        for i in 2..=nargs {
            lua_pushvalue(state, i);
            lua_rawseti(state, -2, (i - 1) as i64);
        }
        lua_pushcclosure(state, Some(partial_call as unsafe fn(*mut State) -> i32), 2);
        1
    }
}

/// functools.reduce(f, t [, init]) → value
unsafe fn ft_reduce(state: *mut State) -> i32 {
    unsafe {
        (*state).lual_checktype(1, TagType::Closure);
        (*state).lual_checktype(2, TagType::Table);
        let has_init = (*state).get_top() >= 3;
        let len = get_length_raw(state, 2) as i64;
        // determine initial accumulator and start index
        let start: i64;
        if has_init {
            lua_pushvalue(state, 3);
            start = 1;
        } else {
            if len == 0 {
                return lual_error(
                    state,
                    c"reduce of empty table with no initial value".as_ptr(),
                    &[],
                );
            }
            lua_rawgeti(state, 2, 1);
            start = 2;
        }
        // stack: f t [init] acc
        for i in start..=len {
            lua_pushvalue(state, 1); // push f
            lua_pushvalue(state, -2); // push acc copy
            lua_rawgeti(state, 2, i); // push t[i]
            (*state).lua_callk(2, 1, 0, None); // f(acc, t[i]) → new_acc
            lua_rotate(state, -2, 1); // swap: old_acc new_acc → new_acc old_acc
            lua_settop(state, -2); // pop old acc
        }
        1
    }
}

/// functools.map(f, t) → new table
unsafe fn ft_map(state: *mut State) -> i32 {
    unsafe {
        (*state).lual_checktype(1, TagType::Closure);
        (*state).lual_checktype(2, TagType::Table);
        let len = get_length_raw(state, 2) as i64;
        (*state).lua_createtable(); // result table at stack top
        for i in 1..=len {
            lua_pushvalue(state, 1); // push f
            lua_rawgeti(state, 2, i); // push t[i]
            (*state).lua_callk(1, 1, 0, None); // call f(t[i]) → result
            lua_rawseti(state, -2, i); // result_table[i] = result
        }
        1
    }
}

/// functools.filter(f, t) → new table
unsafe fn ft_filter(state: *mut State) -> i32 {
    unsafe {
        (*state).lual_checktype(1, TagType::Closure);
        (*state).lual_checktype(2, TagType::Table);
        let len = get_length_raw(state, 2) as i64;
        (*state).lua_createtable(); // result at stack index 3
        let result_idx = (*state).get_top();
        let mut j: i64 = 1;
        for i in 1..=len {
            lua_pushvalue(state, 1); // push f
            lua_rawgeti(state, 2, i); // push t[i]
            (*state).lua_callk(1, 1, 0, None); // f(t[i]) → bool
            if lua_toboolean(state, -1) {
                lua_settop(state, -2); // pop bool
                lua_rawgeti(state, 2, i); // re-fetch t[i]
                lua_rawseti(state, result_idx, j);
                j += 1;
            } else {
                lua_settop(state, -2); // pop bool
            }
        }
        lua_pushvalue(state, result_idx);
        1
    }
}

/// The closure returned by functools.compose(...).
/// Upvalue 1 = table of functions (rightmost first).
unsafe fn compose_call(state: *mut State) -> i32 {
    unsafe {
        let nargs = (*state).get_top();
        let fns_table = UPVALUE1;
        let nfns = get_length_raw(state, fns_table) as i64;
        if nfns == 0 {
            return nargs; // identity
        }
        // call rightmost function first with all args
        lua_rawgeti(state, fns_table, nfns); // push last fn
        for i in 1..=nargs {
            lua_pushvalue(state, i);
        }
        (*state).lua_callk(nargs, 1, 0, None);
        // then chain through the rest right-to-left
        for i in (1..nfns).rev() {
            lua_rawgeti(state, fns_table, i);
            lua_rotate(state, -2, 1); // put fn below result
            (*state).lua_callk(1, 1, 0, None);
        }
        1
    }
}

/// functools.compose(f, g, ...) → closure that applies rightmost first
/// compose(f, g, h)(x) = f(g(h(x)))
unsafe fn ft_compose(state: *mut State) -> i32 {
    unsafe {
        let nargs = (*state).get_top();
        if nargs == 0 {
            return lual_error(
                state,
                c"compose requires at least one function".as_ptr(),
                &[],
            );
        }
        (*state).lua_createtable();
        for i in 1..=nargs {
            lua_pushvalue(state, i);
            lua_rawseti(state, -2, i as i64);
        }
        lua_pushcclosure(state, Some(compose_call as unsafe fn(*mut State) -> i32), 1);
        1
    }
}

/// The closure returned by functools.memoize(f).
/// Upvalue 1 = f, upvalue 2 = cache table.
unsafe fn memoize_call(state: *mut State) -> i32 {
    unsafe {
        let nargs = (*state).get_top();
        // for single-arg memoization, use arg as key directly
        // for multi-arg, build a string key
        if nargs == 1 {
            // check cache[arg1]
            lua_pushvalue(state, 1);
            if lua_rawget(state, UPVALUE2) != TagType::Nil {
                return 1; // cached
            }
            lua_settop(state, -2); // pop nil
                                   // call f
            lua_pushvalue(state, UPVALUE1);
            lua_pushvalue(state, 1);
            (*state).lua_callk(1, 1, 0, None);
            // cache result
            lua_pushvalue(state, 1); // key
            lua_pushvalue(state, -2); // value
            lua_rawset(state, UPVALUE2);
            1
        } else {
            // multi-arg: build string key from tostring of each arg
            let mut key = String::new();
            for i in 1..=nargs {
                if i > 1 {
                    key.push('\0');
                }
                let mut len: usize = 0;
                let ptr = lual_tolstring(state, i, &mut len);
                if !ptr.is_null() {
                    let s = std::slice::from_raw_parts(ptr as *const u8, len);
                    key.extend(s.iter().map(|&b| b as char));
                }
                lua_settop(state, -2); // pop tostring result
            }
            // check cache
            lua_pushlstring(state, key.as_ptr() as *const i8, key.len());
            lua_pushvalue(state, -1); // dup key for later
            if lua_rawget(state, UPVALUE2) != TagType::Nil {
                return 1;
            }
            lua_settop(state, -2); // pop nil, key dup still on stack
                                   // call f with original args
            lua_pushvalue(state, UPVALUE1);
            for i in 1..=nargs {
                lua_pushvalue(state, i);
            }
            (*state).lua_callk(nargs, 1, 0, None);
            // cache: key is at -2, result at -1
            lua_rotate(state, -2, 1); // result key → key result
            lua_pushvalue(state, -1); // dup result
            lua_rotate(state, -3, 1); // result key result → key result result? nope
                                      // let me just: we have key at -2, result at -1
                                      // we need rawset(cache, key, result) then return result
            lua_pushvalue(state, -2); // push key → key result key
            lua_pushvalue(state, -2); // push result → key result key result
            lua_rawset(state, UPVALUE2); // cache[key] = result → key result
            lua_rotate(state, -2, 1); // result key
            lua_settop(state, -2); // pop key → result
            1
        }
    }
}

/// functools.memoize(f) → memoized closure
unsafe fn ft_memoize(state: *mut State) -> i32 {
    unsafe {
        (*state).lual_checktype(1, TagType::Closure);
        lua_pushvalue(state, 1); // upvalue 1 = f
        (*state).lua_createtable(); // upvalue 2 = cache
        lua_pushcclosure(state, Some(memoize_call as unsafe fn(*mut State) -> i32), 2);
        1
    }
}

/// functools.any(f, t) → boolean
unsafe fn ft_any(state: *mut State) -> i32 {
    unsafe {
        (*state).lual_checktype(1, TagType::Closure);
        (*state).lual_checktype(2, TagType::Table);
        let len = get_length_raw(state, 2) as i64;
        for i in 1..=len {
            lua_pushvalue(state, 1);
            lua_rawgeti(state, 2, i);
            (*state).lua_callk(1, 1, 0, None);
            if lua_toboolean(state, -1) {
                (*state).push_boolean(true);
                return 1;
            }
            lua_settop(state, -2);
        }
        (*state).push_boolean(false);
        1
    }
}

/// functools.all(f, t) → boolean
unsafe fn ft_all(state: *mut State) -> i32 {
    unsafe {
        (*state).lual_checktype(1, TagType::Closure);
        (*state).lual_checktype(2, TagType::Table);
        let len = get_length_raw(state, 2) as i64;
        for i in 1..=len {
            lua_pushvalue(state, 1);
            lua_rawgeti(state, 2, i);
            (*state).lua_callk(1, 1, 0, None);
            if !lua_toboolean(state, -1) {
                (*state).push_boolean(false);
                return 1;
            }
            lua_settop(state, -2);
        }
        (*state).push_boolean(true);
        1
    }
}

/// functools.identity(...) → ...
unsafe fn ft_identity(state: *mut State) -> i32 {
    unsafe { (*state).get_top() }
}

/// The closure returned by functools.flip(f).
unsafe fn flip_call(state: *mut State) -> i32 {
    unsafe {
        let nargs = (*state).get_top();
        let base = nargs;
        lua_pushvalue(state, UPVALUE1); // push f
        if nargs >= 2 {
            lua_pushvalue(state, 2); // second arg first
            lua_pushvalue(state, 1); // first arg second
            for i in 3..=nargs {
                lua_pushvalue(state, i);
            }
        } else {
            for i in 1..=nargs {
                lua_pushvalue(state, i);
            }
        }
        (*state).lua_callk(nargs, -1, 0, None);
        (*state).get_top() - base
    }
}

/// functools.flip(f) → closure with first two args swapped
unsafe fn ft_flip(state: *mut State) -> i32 {
    unsafe {
        (*state).lual_checktype(1, TagType::Closure);
        lua_pushvalue(state, 1);
        lua_pushcclosure(state, Some(flip_call as unsafe fn(*mut State) -> i32), 1);
        1
    }
}

pub const FUNCTOOLS_FUNCTIONS: [RegisteredFunction; 10] = [
    RegisteredFunction {
        registeredfunction_name: c"partial".as_ptr(),
        registeredfunction_function: Some(ft_partial as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"reduce".as_ptr(),
        registeredfunction_function: Some(ft_reduce as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"map".as_ptr(),
        registeredfunction_function: Some(ft_map as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"filter".as_ptr(),
        registeredfunction_function: Some(ft_filter as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"compose".as_ptr(),
        registeredfunction_function: Some(ft_compose as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"memoize".as_ptr(),
        registeredfunction_function: Some(ft_memoize as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"any".as_ptr(),
        registeredfunction_function: Some(ft_any as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"all".as_ptr(),
        registeredfunction_function: Some(ft_all as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"identity".as_ptr(),
        registeredfunction_function: Some(ft_identity as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"flip".as_ptr(),
        registeredfunction_function: Some(ft_flip as unsafe fn(*mut State) -> i32),
    },
];

pub unsafe fn luaopen_functools(state: *mut State) -> i32 {
    unsafe {
        (*state).lua_createtable();
        lual_setfuncs(
            state,
            FUNCTOOLS_FUNCTIONS.as_ptr(),
            FUNCTOOLS_FUNCTIONS.len(),
            0,
        );
        1
    }
}
