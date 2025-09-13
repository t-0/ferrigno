use rlua::*;
use crate::buffer::*;
use crate::interpreter::*;
use crate::new::*;
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
                    checkfield(interpreter, make_cstring!("__index"), n)
                })
                && (what & 2 == 0 || {
                    n += 1;
                    checkfield(interpreter, make_cstring!("__newindex"), n)
                })
                && (what & 4 == 0 || {
                    n += 1;
                    checkfield(interpreter, make_cstring!("__len"), n)
                })
            {
                lua_settop(interpreter, -n - 1);
            } else {
                lual_checktype(interpreter, arg, TagType::Table);
            }
        }
    }
}
pub unsafe fn table_insert(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let pos: i64;
        checktab(interpreter, 1, 1 | 2 | 4);
        let mut e: i64 = lual_len(interpreter, 1);
        e = (e as usize).wrapping_add(1 as usize) as i64;
        match (*interpreter).get_top() {
            2 => {
                pos = e;
            }
            3 => {
                let mut i: i64;
                pos = lual_checkinteger(interpreter, 2);
                ((((pos as usize).wrapping_sub(1 as usize) < e as usize) as i32 != 0) as i32 as i64
                    != 0
                    || lual_argerror(
                        interpreter,
                        2,
                        make_cstring!("position out of bounds"),
                    ) != 0) as i32;
                i = e;
                while i > pos {
                    lua_geti(interpreter, 1, i - 1);
                    lua_seti(interpreter, 1, i);
                    i -= 1;
                }
            }
            _ => {
                return lual_error(
                    interpreter,
                    make_cstring!("wrong number of arguments to 'insert'"),
                );
            }
        }
        lua_seti(interpreter, 1, pos);
        return 0;
    }
}
pub unsafe fn table_remove(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        checktab(interpreter, 1, 1 | 2 | 4);
        let size: i64 = lual_len(interpreter, 1);
        let mut pos: i64 = lual_optinteger(interpreter, 2, size);
        if pos != size {
            ((((pos as usize).wrapping_sub(1 as usize) <= size as usize) as i32 != 0) as i32 as i64
                != 0
                || lual_argerror(
                    interpreter,
                    2,
                    make_cstring!("position out of bounds"),
                ) != 0) as i32;
        }
        lua_geti(interpreter, 1, pos);
        while pos < size {
            lua_geti(interpreter, 1, pos + 1);
            lua_seti(interpreter, 1, pos);
            pos += 1;
        }
        (*interpreter).push_nil();
        lua_seti(interpreter, 1, pos);
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
            (((f > 0 || e < MAXIMUM_SIZE as i64 + f) as i32 != 0) as i64 != 0
                || lual_argerror(
                    interpreter,
                    3,
                    make_cstring!("too many elements to move"),
                ) != 0) as i32;
            n = e - f + 1;
            (((t <= MAXIMUM_SIZE as i64 - n + 1) as i32 != 0) as i64 != 0
                || lual_argerror(
                    interpreter,
                    4,
                    make_cstring!("destination wrap around"),
                ) != 0) as i32;
            if t > e || t <= f || tag != 1 && lua_compare(interpreter, 1, tag, 0) == 0 {
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
                make_cstring!("invalid value (%s) at index %I in table for 'concat'"),
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
        let sep: *const i8 =
            lual_optlstring(interpreter, 2, make_cstring!(""), &mut lsep);
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
        lua_setfield(interpreter, 1, make_cstring!("n"));
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
            return lual_error(interpreter, make_cstring!("too many results to unpack"));
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
            buffer.as_mut_ptr() as *mut libc::c_void,
            &mut c as *mut i64 as *const libc::c_void,
            (size_of::<i64>())
                .wrapping_div(size_of::<u32>())
                .wrapping_mul(size_of::<u32>()),
        );
        memcpy(
            buffer.as_mut_ptr().offset(
                (size_of::<i64>() as usize).wrapping_div(size_of::<u32>() as usize) as isize,
            ) as *mut libc::c_void,
            &mut t as *mut i64 as *const libc::c_void,
            (size_of::<i64>())
                .wrapping_div(size_of::<u32>())
                .wrapping_mul(size_of::<u32>()),
        );
        i = 0u32;
        while (i as usize)
            < (size_of::<[u32; 4]>() as usize).wrapping_div(size_of::<u32>() as usize)
        {
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
pub unsafe fn sort_comp(interpreter: *mut Interpreter, a: i32, b: i32) -> i32 {
    unsafe {
        if lua_type(interpreter, 2) == Some(TagType::Nil) {
            return lua_compare(interpreter, a, b, 1);
        } else {
            let res: i32;
            lua_pushvalue(interpreter, 2);
            lua_pushvalue(interpreter, a - 1);
            lua_pushvalue(interpreter, b - 2);
            lua_callk(interpreter, 2, 1, 0, None);
            res = lua_toboolean(interpreter, -1);
            lua_settop(interpreter, -2);
            return res;
        };
    }
}
pub unsafe fn partition(interpreter: *mut Interpreter, lo: u32, up: u32) -> u32 {
    unsafe {
        let mut i: u32 = lo;
        let mut j: u32 = up.wrapping_sub(1 as u32);
        loop {
            loop {
                i = i.wrapping_add(1);
                lua_geti(interpreter, 1, i as i64);
                if !(sort_comp(interpreter, -1, -2) != 0) {
                    break;
                }
                if i == up.wrapping_sub(1) {
                    lual_error(
                        interpreter,
                        make_cstring!("invalid order function for sorting"),
                    );
                }
                lua_settop(interpreter, -2);
            }
            loop {
                j = j.wrapping_sub(1);
                lua_geti(interpreter, 1, j as i64);
                if !(sort_comp(interpreter, -3, -1) != 0) {
                    break;
                }
                if j < i {
                    lual_error(
                        interpreter,
                        make_cstring!("invalid order function for sorting"),
                    );
                }
                lua_settop(interpreter, -2);
            }
            if j < i {
                lua_settop(interpreter, -2);
                set2(interpreter, up.wrapping_sub(1 as u32), i);
                return i;
            }
            set2(interpreter, i, j);
        }
    }
}
pub unsafe fn choose_pivot(lo: u32, up: u32, rnd: u32) -> u32 {
    let r4: u32 = up.wrapping_sub(lo).wrapping_div(4 as u32);
    let p: u32 = rnd
        .wrapping_rem(r4.wrapping_mul(2 as u32))
        .wrapping_add(lo.wrapping_add(r4));
    return p;
}
pub unsafe fn auxsort(
    interpreter: *mut Interpreter,
    mut lo: u32,
    mut up: u32,
    mut rnd: u32,
) {
    unsafe {
        while lo < up {
            let mut p: u32;
            let n: u32;
            lua_geti(interpreter, 1, lo as i64);
            lua_geti(interpreter, 1, up as i64);
            if sort_comp(interpreter, -1, -2) != 0 {
                set2(interpreter, lo, up);
            } else {
                lua_settop(interpreter, -2 - 1);
            }
            if up.wrapping_sub(lo) == 1 as u32 {
                return;
            }
            if up.wrapping_sub(lo) < 100 as u32 || rnd == 0 {
                p = lo.wrapping_add(up).wrapping_div(2 as u32);
            } else {
                p = choose_pivot(lo, up, rnd);
            }
            lua_geti(interpreter, 1, p as i64);
            lua_geti(interpreter, 1, lo as i64);
            if sort_comp(interpreter, -2, -1) != 0 {
                set2(interpreter, p, lo);
            } else {
                lua_settop(interpreter, -2);
                lua_geti(interpreter, 1, up as i64);
                if sort_comp(interpreter, -1, -2) != 0 {
                    set2(interpreter, p, up);
                } else {
                    lua_settop(interpreter, -2 - 1);
                }
            }
            if up.wrapping_sub(lo) == 2 as u32 {
                return;
            }
            lua_geti(interpreter, 1, p as i64);
            lua_pushvalue(interpreter, -1);
            lua_geti(interpreter, 1, up.wrapping_sub(1 as u32) as i64);
            set2(interpreter, p, up.wrapping_sub(1 as u32));
            p = partition(interpreter, lo, up);
            if p.wrapping_sub(lo) < up.wrapping_sub(p) {
                auxsort(interpreter, lo, p.wrapping_sub(1 as u32), rnd);
                n = p.wrapping_sub(lo);
                lo = p.wrapping_add(1 as u32);
            } else {
                auxsort(interpreter, p.wrapping_add(1 as u32), up, rnd);
                n = up.wrapping_sub(p);
                up = p.wrapping_sub(1 as u32);
            }
            if up.wrapping_sub(lo).wrapping_div(128 as u32) > n {
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
                lual_argerror(interpreter, 1, make_cstring!("array too big"));
            }
            match lua_type(interpreter, 2) {
                None | Some(TagType::Nil) => {}
                _ => {
                    lual_checktype(interpreter, 2, TagType::Closure);
                }
            }
            lua_settop(interpreter, 2);
            auxsort(interpreter, 1 as u32, n as u32, 0);
        }
        return 0;
    }
}
pub const TABLE_FUNCTIONS: [RegisteredFunction; 8] = {
    [
        {
            RegisteredFunction {
                name: make_cstring!("concat"),
                function: Some(table_concat as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: make_cstring!("insert"),
                function: Some(table_insert as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: make_cstring!("pack"),
                function: Some(table_pack as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: make_cstring!("unpack"),
                function: Some(table_unpack as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: make_cstring!("remove"),
                function: Some(table_remove as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: make_cstring!("move"),
                function: Some(table_move as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: make_cstring!("sort"),
                function: Some(table_sort as unsafe fn(*mut Interpreter) -> i32),
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
pub unsafe fn luaopen_table(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        lual_checkversion_(
            interpreter,
            504.0,
            (size_of::<i64>() as usize)
                .wrapping_mul(16 as usize)
                .wrapping_add(size_of::<f64>() as usize),
        );
        (*interpreter).lua_createtable();
        lual_setfuncs(interpreter, TABLE_FUNCTIONS.as_ptr(), 0);
        return 1;
    }
}
