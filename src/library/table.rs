use crate::buffer::*;
use crate::tdefaultnew::*;
use libc::{time, c_void};
use crate::interpreter::*;
use crate::registeredfunction::*;
use crate::tag::*;
use crate::utility::c::*;
use crate::utility::*;
use std::ptr::*;
pub unsafe fn checkfield(interpreter: *mut Interpreter, key: *const i8, n: i32) -> bool {
    unsafe {
        lua_pushstring(interpreter, key);
        return lua_rawget(interpreter, -n) != TagType::Nil;
    }
}
pub unsafe fn checktab(interpreter: *mut Interpreter, arg: i32, what: i32) {
    unsafe {
        if lua_type(interpreter, arg) != Some(TagType::Table) {
            let mut n: i32 = 1;
            if (*interpreter).lua_getmetatable(arg)
                && (what & 1 == 0 || {
                    n += 1;
                    checkfield(interpreter, c"__index".as_ptr(), n)
                })
                && (what & 2 == 0 || {
                    n += 1;
                    checkfield(interpreter, c"__newindex".as_ptr(), n)
                })
                && (what & 4 == 0 || {
                    n += 1;
                    checkfield(interpreter, c"__len".as_ptr(), n)
                })
            {
                lua_settop(interpreter, -n - 1);
            } else {
                (*interpreter).lual_checktype(arg, TagType::Table);
            }
        }
    }
}
pub unsafe fn table_insert(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let position: i64;
        checktab(interpreter, 1, 1 | 2 | 4);
        let mut e: i64 = lual_len(interpreter, 1);
        e = (e as usize).wrapping_add(1 as usize) as i64;
        match (*interpreter).get_top() {
            2 => {
                position = e;
            },
            3 => {
                let mut i: i64;
                position = lual_checkinteger(interpreter, 2);
                ((((position as usize).wrapping_sub(1 as usize) < e as usize) as i32 != 0) as i32 as i64 != 0 || lual_argerror(interpreter, 2, c"position out of bounds".as_ptr()) != 0) as i32;
                i = e;
                while i > position {
                    lua_geti(interpreter, 1, i - 1);
                    lua_seti(interpreter, 1, i);
                    i -= 1;
                }
            },
            _ => {
                return lual_error(interpreter, c"wrong number of arguments to 'insert'".as_ptr());
            },
        }
        lua_seti(interpreter, 1, position);
        return 0;
    }
}
pub unsafe fn table_remove(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        checktab(interpreter, 1, 1 | 2 | 4);
        let size: i64 = lual_len(interpreter, 1);
        let mut position: i64 = lual_optinteger(interpreter, 2, size);
        if position != size {
            ((((position as usize).wrapping_sub(1 as usize) <= size as usize) as i32 != 0) as i32 as i64 != 0 || lual_argerror(interpreter, 2, c"position out of bounds".as_ptr()) != 0) as i32;
        }
        lua_geti(interpreter, 1, position);
        while position < size {
            lua_geti(interpreter, 1, position + 1);
            lua_seti(interpreter, 1, position);
            position += 1;
        }
        (*interpreter).push_nil();
        lua_seti(interpreter, 1, position);
        return 1;
    }
}
pub unsafe fn table_move(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let f: i64 = lual_checkinteger(interpreter, 2);
        let e: i64 = lual_checkinteger(interpreter, 3);
        let t: i64 = lual_checkinteger(interpreter, 4);
        let tag: i32 = match lua_type(interpreter, 5) {
            None | Some(TagType::Nil) => 1,
            _ => 5,
        };
        checktab(interpreter, 1, 1);
        checktab(interpreter, tag, 2);
        if e >= f {
            let n: i64;
            let mut i: i64;
            (((f > 0 || e < MAXIMUM_SIZE as i64 + f) as i32 != 0) as i64 != 0 || lual_argerror(interpreter, 3, c"too many elements to move".as_ptr()) != 0) as i32;
            n = e - f + 1;
            (((t <= MAXIMUM_SIZE as i64 - n + 1) as i32 != 0) as i64 != 0 || lual_argerror(interpreter, 4, c"destination wrap around".as_ptr()) != 0) as i32;
            if t > e || t <= f || tag != 1 && !lua_compare(interpreter, 1, tag, 0) {
                for i in 0..n {
                    lua_geti(interpreter, 1, f + i);
                    lua_seti(interpreter, tag, t + i);
                }
            } else {
                i = n - 1;
                while i >= 0 {
                    lua_geti(interpreter, 1, f + i);
                    lua_seti(interpreter, tag, t + i);
                    i -= 1;
                }
            }
        }
        lua_pushvalue(interpreter, tag);
        return 1;
    }
}
pub unsafe fn addfield(interpreter: *mut Interpreter, b: *mut Buffer, i: i64) {
    unsafe {
        lua_geti(interpreter, 1, i);
        if !lua_isstring(interpreter, -1) {
            lual_error(
                interpreter,
                c"invalid value (%s) at index %I in table for 'concat'".as_ptr(),
                lua_typename(interpreter, lua_type(interpreter, -1)),
                i,
            );
        }
        (*b).add_value();
    }
}
pub unsafe fn table_concat(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let mut b = Buffer::new();
        checktab(interpreter, 1, 1 | 4);
        let mut last: i64 = lual_len(interpreter, 1);
        let mut lsep: usize = 0;
        let sep: *const i8 = lual_optlstring(interpreter, 2, c"".as_ptr(), &mut lsep);
        let mut i: i64 = lual_optinteger(interpreter, 3, 1);
        last = lual_optinteger(interpreter, 4, last);
        b.initialize(interpreter);
        while i < last {
            addfield(interpreter, &mut b, i);
            b.add_string_with_length(sep, lsep as usize);
            i += 1;
        }
        if i == last {
            addfield(interpreter, &mut b, i);
        }
        b.push_result();
        return 1;
    }
}
pub unsafe fn table_pack(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let mut i: i32;
        let n: i32 = (*interpreter).get_top();
        (*interpreter).lua_createtable();
        lua_rotate(interpreter, 1, 1);
        i = n;
        while i >= 1 {
            lua_seti(interpreter, 1, i as i64);
            i -= 1;
        }
        (*interpreter).push_integer(n as i64);
        lua_setfield(interpreter, 1, c"n".as_ptr());
        return 1;
    }
}
pub unsafe fn table_unpack(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let mut n: usize;
        let mut i: i64 = lual_optinteger(interpreter, 2, 1);
        let e = match lua_type(interpreter, 3) {
            None | Some(TagType::Nil) => lual_len(interpreter, 1),
            _ => lual_checkinteger(interpreter, 3),
        };
        if i > e {
            return 0;
        }
        n = (e as usize).wrapping_sub(i as usize);
        if ((n >= 0x7FFFFFFF as usize || {
            n = n.wrapping_add(1);
            lua_checkstack(interpreter, n as i32) == 0
        }) as i32
            != 0) as i64
            != 0
        {
            return lual_error(interpreter, c"too many results to unpack".as_ptr());
        }
        while i < e {
            lua_geti(interpreter, 1, i);
            i += 1;
        }
        lua_geti(interpreter, 1, e);
        return n as i32;
    }
}
pub unsafe fn l_randomizepivot() -> u32 {
    unsafe {
        let mut c: i64 = clock();
        let mut t: i64 = time(null_mut());
        let mut buffer: [u32; 4] = [0; 4];
        let mut i: u32;
        let mut rnd: u32 = 0u32;
        memcpy(
            buffer.as_mut_ptr() as *mut c_void,
            &mut c as *mut i64 as *const c_void,
            size_of::<i64>(),
        );
        memcpy(
            buffer.as_mut_ptr().offset((size_of::<i64>() / size_of::<u32>()) as isize) as *mut c_void,
            &mut t as *mut i64 as *const c_void,
            size_of::<i64>(),
        );
        i = 0u32;
        while (i as usize) < 4 {
            rnd = rnd.wrapping_add(buffer[i as usize]);
            i = i.wrapping_add(1);
        }
        return rnd;
    }
}
pub unsafe fn set2(interpreter: *mut Interpreter, i: u32, j: u32) {
    unsafe {
        lua_seti(interpreter, 1, i as i64);
        lua_seti(interpreter, 1, j as i64);
    }
}
pub unsafe fn sort_comp(interpreter: *mut Interpreter, a: i32, b: i32) -> bool {
    unsafe {
        if lua_type(interpreter, 2) == Some(TagType::Nil) {
            return lua_compare(interpreter, a, b, 1);
        } else {
            lua_pushvalue(interpreter, 2);
            lua_pushvalue(interpreter, a - 1);
            lua_pushvalue(interpreter, b - 2);
            (*interpreter).lua_callk(2, 1, 0, None);
            let res = lua_toboolean(interpreter, -1);
            lua_settop(interpreter, -2);
            return res;
        };
    }
}
pub unsafe fn partition(interpreter: *mut Interpreter, low: u32, high: u32) -> u32 {
    unsafe {
        let mut i: u32 = low;
        let mut j: u32 = high.wrapping_sub(1 as u32);
        loop {
            loop {
                i = i.wrapping_add(1);
                lua_geti(interpreter, 1, i as i64);
                if !sort_comp(interpreter, -1, -2) {
                    break;
                }
                if i == high - 1 {
                    lual_error(interpreter, c"invalid order function for sorting".as_ptr());
                }
                lua_settop(interpreter, -2);
            }
            loop {
                j -= 1;
                lua_geti(interpreter, 1, j as i64);
                if !sort_comp(interpreter, -3, -1) {
                    break;
                }
                if j < i {
                    lual_error(interpreter, c"invalid order function for sorting".as_ptr());
                }
                lua_settop(interpreter, -2);
            }
            if j < i {
                lua_settop(interpreter, -2);
                set2(interpreter, high.wrapping_sub(1 as u32), i);
                return i;
            }
            set2(interpreter, i, j);
        }
    }
}
pub unsafe fn choose_pivot(low: u32, high: u32, rnd: u32) -> u32 {
    let r4: u32 = (high - low) / 4;
    let p: u32 = rnd.wrapping_rem(r4.wrapping_mul(2 as u32)).wrapping_add(low.wrapping_add(r4));
    return p;
}
pub unsafe fn auxsort(interpreter: *mut Interpreter, mut low: u32, mut high: u32, mut rnd: u32) {
    unsafe {
        while low < high {
            let mut p: u32;
            let n: u32;
            lua_geti(interpreter, 1, low as i64);
            lua_geti(interpreter, 1, high as i64);
            if sort_comp(interpreter, -1, -2) {
                set2(interpreter, low, high);
            } else {
                lua_settop(interpreter, -2 - 1);
            }
            if high - low == 1 {
                return;
            }
            if high - low < 100 || rnd == 0 {
                p = (low + high) / 2;
            } else {
                p = choose_pivot(low, high, rnd);
            }
            lua_geti(interpreter, 1, p as i64);
            lua_geti(interpreter, 1, low as i64);
            if sort_comp(interpreter, -2, -1) {
                set2(interpreter, p, low);
            } else {
                lua_settop(interpreter, -2);
                lua_geti(interpreter, 1, high as i64);
                if sort_comp(interpreter, -1, -2) {
                    set2(interpreter, p, high);
                } else {
                    lua_settop(interpreter, -2 - 1);
                }
            }
            if high.wrapping_sub(low) == 2 as u32 {
                return;
            }
            lua_geti(interpreter, 1, p as i64);
            lua_pushvalue(interpreter, -1);
            lua_geti(interpreter, 1, high.wrapping_sub(1 as u32) as i64);
            set2(interpreter, p, high.wrapping_sub(1 as u32));
            p = partition(interpreter, low, high);
            if p.wrapping_sub(low) < high.wrapping_sub(p) {
                auxsort(interpreter, low, p.wrapping_sub(1 as u32), rnd);
                n = p.wrapping_sub(low);
                low = p.wrapping_add(1 as u32);
            } else {
                auxsort(interpreter, p.wrapping_add(1 as u32), high, rnd);
                n = high.wrapping_sub(p);
                high = p.wrapping_sub(1 as u32);
            }
            if ((high - low) / 128) > n {
                rnd = l_randomizepivot();
            }
        }
    }
}
pub unsafe fn table_sort(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        checktab(interpreter, 1, 1 | 2 | 4);
        let n: i64 = lual_len(interpreter, 1);
        if n > 1 {
            if n >= 0x7FFFFFFF {
                lual_argerror(interpreter, 1, c"array too big".as_ptr());
            }
            match lua_type(interpreter, 2) {
                None | Some(TagType::Nil) => {},
                _ => {
                    (*interpreter).lual_checktype(2, TagType::Closure);
                },
            }
            lua_settop(interpreter, 2);
            auxsort(interpreter, 1 as u32, n as u32, 0);
        }
        return 0;
    }
}
pub const TABLE_FUNCTIONS: [RegisteredFunction; 7] = {
    [
        { RegisteredFunction { name: c"concat".as_ptr(), function: Some(table_concat as unsafe fn(*mut Interpreter) -> i32) } },
        { RegisteredFunction { name: c"insert".as_ptr(), function: Some(table_insert as unsafe fn(*mut Interpreter) -> i32) } },
        { RegisteredFunction { name: c"pack".as_ptr(), function: Some(table_pack as unsafe fn(*mut Interpreter) -> i32) } },
        { RegisteredFunction { name: c"unpack".as_ptr(), function: Some(table_unpack as unsafe fn(*mut Interpreter) -> i32) } },
        { RegisteredFunction { name: c"remove".as_ptr(), function: Some(table_remove as unsafe fn(*mut Interpreter) -> i32) } },
        { RegisteredFunction { name: c"move".as_ptr(), function: Some(table_move as unsafe fn(*mut Interpreter) -> i32) } },
        { RegisteredFunction { name: c"sort".as_ptr(), function: Some(table_sort as unsafe fn(*mut Interpreter) -> i32) } },
    ]
};
pub unsafe fn luaopen_table(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        lual_checkversion_(interpreter, 504.0, (size_of::<i64>() as usize).wrapping_mul(16 as usize).wrapping_add(size_of::<f64>() as usize));
        (*interpreter).lua_createtable();
        lual_setfuncs(interpreter, TABLE_FUNCTIONS.as_ptr(), TABLE_FUNCTIONS.len(), 0);
        return 1;
    }
}
