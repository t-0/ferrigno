use crate::registeredfunction::*;
use crate::state::*;
use crate::tag::*;
use crate::buffer::*;
use crate::new::*;
use crate::utility::c::*;
pub unsafe extern "C" fn checkfield(state: *mut State, key: *const i8, n: i32) -> i32 {
    unsafe {
        lua_pushstring(state, key);
        return (lua_rawget(state, -n) != 0) as i32;
    }
}
pub unsafe extern "C" fn checktab(state: *mut State, arg: i32, what: i32) {
    unsafe {
        if lua_type(state, arg) != Some(TAG_TYPE_TABLE) {
            let mut n: i32 = 1;
            if (*state).lua_getmetatable(arg)
                && (what & 1 == 0 || {
                    n += 1;
                    checkfield(state, b"__index\0" as *const u8 as *const i8, n) != 0
                })
                && (what & 2 == 0 || {
                    n += 1;
                    checkfield(state, b"__newindex\0" as *const u8 as *const i8, n) != 0
                })
                && (what & 4 == 0 || {
                    n += 1;
                    checkfield(state, b"__len\0" as *const u8 as *const i8, n) != 0
                })
            {
                lua_settop(state, -n - 1);
            } else {
                lual_checktype(state, arg, TAG_TYPE_TABLE);
            }
        }
    }
}
pub unsafe extern "C" fn table_insert(state: *mut State) -> i32 {
    unsafe {
        let pos: i64;
        checktab(state, 1, 1 | 2 | 4);
        let mut e: i64 = lual_len(state, 1);
        e = (e as u64).wrapping_add(1 as u64) as i64;
        match (*state).get_top() {
            2 => {
                pos = e;
            }
            3 => {
                let mut i: i64;
                pos = lual_checkinteger(state, 2);
                ((((pos as u64).wrapping_sub(1 as u64) < e as u64) as i32 != 0) as i32
                    as i64
                    != 0
                    || lual_argerror(
                        state,
                        2,
                        b"position out of bounds\0" as *const u8 as *const i8,
                    ) != 0) as i32;
                i = e;
                while i > pos {
                    lua_geti(state, 1, i - 1);
                    lua_seti(state, 1, i);
                    i -= 1;
                }
            }
            _ => {
                return lual_error(
                    state,
                    b"wrong number of arguments to 'insert'\0" as *const u8 as *const i8,
                );
            }
        }
        lua_seti(state, 1, pos);
        return 0;
    }
}
pub unsafe extern "C" fn table_remove(state: *mut State) -> i32 {
    unsafe {
        checktab(state, 1, 1 | 2 | 4);
        let size: i64 = lual_len(state, 1);
        let mut pos: i64 = lual_optinteger(state, 2, size);
        if pos != size {
            ((((pos as u64).wrapping_sub(1 as u64) <= size as u64) as i32 != 0) as i32
                as i64
                != 0
                || lual_argerror(
                    state,
                    2,
                    b"position out of bounds\0" as *const u8 as *const i8,
                ) != 0) as i32;
        }
        lua_geti(state, 1, pos);
        while pos < size {
            lua_geti(state, 1, pos + 1);
            lua_seti(state, 1, pos);
            pos += 1;
        }
        (*state).push_nil();
        lua_seti(state, 1, pos);
        return 1;
    }
}
pub unsafe extern "C" fn table_move(state: *mut State) -> i32 {
    unsafe {
        let f: i64 = lual_checkinteger(state, 2);
        let e: i64 = lual_checkinteger(state, 3);
        let t: i64 = lual_checkinteger(state, 4);
        let tag: i32 = match lua_type(state, 5) {
            None | Some(TAG_TYPE_NIL) => 1,
            _ =>  5,
        };
        checktab(state, 1, 1);
        checktab(state, tag, 2);
        if e >= f {
            let n: i64;
            let mut i: i64;
            (((f > 0 || e < 0x7FFFFFFFFFFFFFFF as i64 + f) as i32 != 0) as i64 != 0
                || lual_argerror(
                    state,
                    3,
                    b"too many elements to move\0" as *const u8 as *const i8,
                ) != 0) as i32;
            n = e - f + 1;
            (((t <= 0x7FFFFFFFFFFFFFFF as i64 - n + 1) as i32 != 0) as i64 != 0
                || lual_argerror(
                    state,
                    4,
                    b"destination wrap around\0" as *const u8 as *const i8,
                ) != 0) as i32;
            if t > e || t <= f || tag != 1 && lua_compare(state, 1, tag, 0) == 0 {
                i = 0;
                while i < n {
                    lua_geti(state, 1, f + i);
                    lua_seti(state, tag, t + i);
                    i += 1;
                }
            } else {
                i = n - 1;
                while i >= 0 {
                    lua_geti(state, 1, f + i);
                    lua_seti(state, tag, t + i);
                    i -= 1;
                }
            }
        }
        lua_pushvalue(state, tag);
        return 1;
    }
}
pub unsafe extern "C" fn addfield(state: *mut State, b: *mut Buffer, i: i64) {
    unsafe {
        lua_geti(state, 1, i);
        if !lua_isstring(state, -1) {
            lual_error(
                state,
                b"invalid value (%s) at index %I in table for 'concat'\0" as *const u8 as *const i8,
                lua_typename(state, lua_type(state, -1)),
                i,
            );
        }
        (*b).add_value();
    }
}
pub unsafe extern "C" fn table_concat(state: *mut State) -> i32 {
    unsafe {
        let mut b = Buffer::new();
        checktab(state, 1, 1 | 4);
        let mut last: i64 = lual_len(state, 1);
        let mut lsep: u64 = 0;
        let sep: *const i8 = lual_optlstring(state, 2, b"\0" as *const u8 as *const i8, &mut lsep);
        let mut i: i64 = lual_optinteger(state, 3, 1);
        last = lual_optinteger(state, 4, last);
        b.initialize(state);
        while i < last {
            addfield(state, &mut b, i);
            b.add_string_with_length(sep, lsep);
            i += 1;
        }
        if i == last {
            addfield(state, &mut b, i);
        }
        b.push_result();
        return 1;
    }
}
pub unsafe extern "C" fn table_pack(state: *mut State) -> i32 {
    unsafe {
        let mut i: i32;
        let n: i32 = (*state).get_top();
        (*state).lua_createtable();
        lua_rotate(state, 1, 1);
        i = n;
        while i >= 1 {
            lua_seti(state, 1, i as i64);
            i -= 1;
        }
        (*state).push_integer(n as i64);
        lua_setfield(state, 1, b"n\0" as *const u8 as *const i8);
        return 1;
    }
}
pub unsafe extern "C" fn table_unpack(state: *mut State) -> i32 {
    unsafe {
        let mut n: u64;
        let mut i: i64 = lual_optinteger(state, 2, 1);
        let e = match lua_type(state, 3) {
            None | Some(TAG_TYPE_NIL) => {
                lual_len(state, 1)
            },
            _ => {
                lual_checkinteger(state, 3)
            },
        };
        if i > e {
            return 0;
        }
        n = (e as u64).wrapping_sub(i as u64);
        if ((n >= 0x7FFFFFFF as u64 || {
            n = n.wrapping_add(1);
            lua_checkstack(state, n as i32) == 0
        }) as i32
            != 0) as i64
            != 0
        {
            return lual_error(
                state,
                b"too many results to unpack\0" as *const u8 as *const i8,
            );
        }
        while i < e {
            lua_geti(state, 1, i);
            i += 1;
        }
        lua_geti(state, 1, e);
        return n as i32;
    }
}
pub unsafe extern "C" fn l_randomizepivot() -> u32 {
    unsafe {
        let mut c: i64 = clock();
        let mut t: i64 = time(std::ptr::null_mut());
        let mut buffer: [u32; 4] = [0; 4];
        let mut i: u32;
        let mut rnd: u32 = 0u32;
        memcpy(
            buffer.as_mut_ptr() as *mut libc::c_void,
            &mut c as *mut i64 as *const libc::c_void,
            (::core::mem::size_of::<i64>() as u64)
                .wrapping_div(::core::mem::size_of::<u32>() as u64)
                .wrapping_mul(::core::mem::size_of::<u32>() as u64),
        );
        memcpy(
            buffer.as_mut_ptr().offset(
                (::core::mem::size_of::<i64>() as u64)
                    .wrapping_div(::core::mem::size_of::<u32>() as u64) as isize,
            ) as *mut libc::c_void,
            &mut t as *mut i64 as *const libc::c_void,
            (::core::mem::size_of::<i64>() as u64)
                .wrapping_div(::core::mem::size_of::<u32>() as u64)
                .wrapping_mul(::core::mem::size_of::<u32>() as u64),
        );
        i = 0u32;
        while (i as u64)
            < (::core::mem::size_of::<[u32; 4]>() as u64)
                .wrapping_div(::core::mem::size_of::<u32>() as u64)
        {
            rnd = rnd.wrapping_add(buffer[i as usize]);
            i = i.wrapping_add(1);
        }
        return rnd;
    }
}
pub unsafe extern "C" fn set2(state: *mut State, i: u32, j: u32) {
    unsafe {
        lua_seti(state, 1, i as i64);
        lua_seti(state, 1, j as i64);
    }
}
pub unsafe extern "C" fn sort_comp(state: *mut State, a: i32, b: i32) -> i32 {
    unsafe {
        if lua_type(state, 2) == Some(TAG_TYPE_NIL) {
            return lua_compare(state, a, b, 1);
        } else {
            let res: i32;
            lua_pushvalue(state, 2);
            lua_pushvalue(state, a - 1);
            lua_pushvalue(state, b - 2);
            lua_callk(state, 2, 1, 0, None);
            res = lua_toboolean(state, -1);
            lua_settop(state, -2);
            return res;
        };
    }
}
pub unsafe extern "C" fn partition(state: *mut State, lo: u32, up: u32) -> u32 {
    unsafe {
        let mut i: u32 = lo;
        let mut j: u32 = up.wrapping_sub(1 as u32);
        loop {
            loop {
                i = i.wrapping_add(1);
                lua_geti(state, 1, i as i64);
                if !(sort_comp(state, -1, -2) != 0) {
                    break;
                }
                if ((i == up.wrapping_sub(1 as u32)) as i32 != 0) as i64 != 0 {
                    lual_error(
                        state,
                        b"invalid order function for sorting\0" as *const u8 as *const i8,
                    );
                }
                lua_settop(state, -2);
            }
            loop {
                j = j.wrapping_sub(1);
                lua_geti(state, 1, j as i64);
                if !(sort_comp(state, -3, -1) != 0) {
                    break;
                }
                if ((j < i) as i32 != 0) as i64 != 0 {
                    lual_error(
                        state,
                        b"invalid order function for sorting\0" as *const u8 as *const i8,
                    );
                }
                lua_settop(state, -2);
            }
            if j < i {
                lua_settop(state, -2);
                set2(state, up.wrapping_sub(1 as u32), i);
                return i;
            }
            set2(state, i, j);
        }
    }
}
pub unsafe extern "C" fn choose_pivot(lo: u32, up: u32, rnd: u32) -> u32 {
    let r4: u32 = up.wrapping_sub(lo).wrapping_div(4 as u32);
    let p: u32 = rnd
        .wrapping_rem(r4.wrapping_mul(2 as u32))
        .wrapping_add(lo.wrapping_add(r4));
    return p;
}
pub unsafe extern "C" fn auxsort(state: *mut State, mut lo: u32, mut up: u32, mut rnd: u32) {
    unsafe {
        while lo < up {
            let mut p: u32;
            let n: u32;
            lua_geti(state, 1, lo as i64);
            lua_geti(state, 1, up as i64);
            if sort_comp(state, -1, -2) != 0 {
                set2(state, lo, up);
            } else {
                lua_settop(state, -2 - 1);
            }
            if up.wrapping_sub(lo) == 1 as u32 {
                return;
            }
            if up.wrapping_sub(lo) < 100 as u32 || rnd == 0u32 {
                p = lo.wrapping_add(up).wrapping_div(2 as u32);
            } else {
                p = choose_pivot(lo, up, rnd);
            }
            lua_geti(state, 1, p as i64);
            lua_geti(state, 1, lo as i64);
            if sort_comp(state, -2, -1) != 0 {
                set2(state, p, lo);
            } else {
                lua_settop(state, -2);
                lua_geti(state, 1, up as i64);
                if sort_comp(state, -1, -2) != 0 {
                    set2(state, p, up);
                } else {
                    lua_settop(state, -2 - 1);
                }
            }
            if up.wrapping_sub(lo) == 2 as u32 {
                return;
            }
            lua_geti(state, 1, p as i64);
            lua_pushvalue(state, -1);
            lua_geti(state, 1, up.wrapping_sub(1 as u32) as i64);
            set2(state, p, up.wrapping_sub(1 as u32));
            p = partition(state, lo, up);
            if p.wrapping_sub(lo) < up.wrapping_sub(p) {
                auxsort(state, lo, p.wrapping_sub(1 as u32), rnd);
                n = p.wrapping_sub(lo);
                lo = p.wrapping_add(1 as u32);
            } else {
                auxsort(state, p.wrapping_add(1 as u32), up, rnd);
                n = up.wrapping_sub(p);
                up = p.wrapping_sub(1 as u32);
            }
            if up.wrapping_sub(lo).wrapping_div(128 as u32) > n {
                rnd = l_randomizepivot();
            }
        }
    }
}
pub unsafe extern "C" fn table_sort(state: *mut State) -> i32 {
    unsafe {
        checktab(state, 1, 1 | 2 | 4);
        let n: i64 = lual_len(state, 1);
        if n > 1 {
            if n >= 0x7FFFFFFF {
                lual_argerror(state, 1, b"array too big\0" as *const u8 as *const i8);
            }
            match lua_type(state, 2) {
                None | Some(TAG_TYPE_NIL) => {
                },
                _ => {
                    lual_checktype(state, 2, TAG_TYPE_CLOSURE);
                },
            }
            lua_settop(state, 2);
            auxsort(state, 1 as u32, n as u32, 0);
        }
        return 0;
    }
}
pub const TABLE_FUNCTIONS: [RegisteredFunction; 8] = {
    [
        {
            RegisteredFunction {
                name: b"concat\0" as *const u8 as *const i8,
                function: Some(table_concat as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"insert\0" as *const u8 as *const i8,
                function: Some(table_insert as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"pack\0" as *const u8 as *const i8,
                function: Some(table_pack as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"unpack\0" as *const u8 as *const i8,
                function: Some(table_unpack as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"remove\0" as *const u8 as *const i8,
                function: Some(table_remove as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"move\0" as *const u8 as *const i8,
                function: Some(table_move as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"sort\0" as *const u8 as *const i8,
                function: Some(table_sort as unsafe extern "C" fn(*mut State) -> i32),
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
pub unsafe extern "C" fn luaopen_table(state: *mut State) -> i32 {
    unsafe {
        lual_checkversion_(
            state,
            504.0,
            (::core::mem::size_of::<i64>() as u64)
                .wrapping_mul(16 as u64)
                .wrapping_add(::core::mem::size_of::<f64>() as u64),
        );
        (*state).lua_createtable();
        lual_setfuncs(state, TABLE_FUNCTIONS.as_ptr(), 0);
        return 1;
    }
}
