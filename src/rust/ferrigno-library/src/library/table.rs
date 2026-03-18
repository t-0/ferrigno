use crate::buffer::*;
use crate::registeredfunction::*;
use crate::state::*;
use crate::table::*;
use crate::tagtype::*;
use crate::tdefaultnew::*;
use crate::utility::*;
pub unsafe fn checkfield(state: *mut State, key: *const i8, n: i32) -> bool {
    unsafe {
        lua_pushstring(state, key);
        lua_rawget(state, -n) != TagType::Nil
    }
}
pub unsafe fn checktab(state: *mut State, arg: i32, what: i32) {
    unsafe {
        if lua_type(state, arg) != Some(TagType::Table) {
            let mut n: i32 = 1;
            if (*state).lua_getmetatable(arg)
                && (what & 1 == 0 || {
                    n += 1;
                    checkfield(state, c"__index".as_ptr(), n)
                })
                && (what & 2 == 0 || {
                    n += 1;
                    checkfield(state, c"__newindex".as_ptr(), n)
                })
                && (what & 4 == 0 || {
                    n += 1;
                    checkfield(state, c"__len".as_ptr(), n)
                })
            {
                lua_settop(state, -n - 1);
            } else {
                (*state).lual_checktype(arg, TagType::Table);
            }
        }
    }
}
pub unsafe fn table_insert(state: *mut State) -> i32 {
    unsafe {
        let position: i64;
        checktab(state, 1, 1 | 2 | 4);
        let mut e: i64 = lual_len(state, 1);
        e = (e as usize + 1) as i64;
        match (*state).get_top() {
            | 2 => {
                position = e;
            },
            | 3 => {
                let mut i: i64;
                position = lual_checkinteger(state, 2);
                if position < 1 || position > e {
                    lual_argerror(state, 2, c"position out of bounds".as_ptr());
                    0;
                }
                i = e;
                while i > position {
                    lua_geti(state, 1, i - 1);
                    lua_seti(state, 1, i);
                    i -= 1;
                }
            },
            | _ => {
                return lual_error(state, c"wrong number of arguments to 'insert'".as_ptr());
            },
        }
        lua_seti(state, 1, position);
        0
    }
}
pub unsafe fn table_remove(state: *mut State) -> i32 {
    unsafe {
        checktab(state, 1, 1 | 2 | 4);
        let size: i64 = lual_len(state, 1);
        let mut position: i64 = lual_optinteger(state, 2, size);
        if position != size && (position < 1 || position > size + 1) {
            lual_argerror(state, 2, c"position out of bounds".as_ptr());
            0;
        }
        lua_geti(state, 1, position);
        while position < size {
            lua_geti(state, 1, position + 1);
            lua_seti(state, 1, position);
            position += 1;
        }
        (*state).push_nil();
        lua_seti(state, 1, position);
        1
    }
}
pub unsafe fn table_move(state: *mut State) -> i32 {
    unsafe {
        let f: i64 = lual_checkinteger(state, 2);
        let e: i64 = lual_checkinteger(state, 3);
        let t: i64 = lual_checkinteger(state, 4);
        let tag: i32 = match lua_type(state, 5) {
            | None | Some(TagType::Nil) => 1,
            | _ => 5,
        };
        checktab(state, 1, 1);
        checktab(state, tag, 2);
        if e >= f {
            let mut i: i64;
            if !(f > 0 || e < MAXIMUM_SIZE as i64 + f) {
                lual_argerror(state, 3, c"too many elements to move".as_ptr());
                0;
            }
            let n: i64 = e - f + 1;
            if t > MAXIMUM_SIZE as i64 - n + 1 {
                lual_argerror(state, 4, c"destination wrap around".as_ptr());
                0;
            }
            if t > e || t <= f || tag != 1 && !lua_compare(state, 1, tag, 0) {
                for i in 0..n {
                    lua_geti(state, 1, f + i);
                    lua_seti(state, tag, t + i);
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
        1
    }
}
pub unsafe fn addfield(state: *mut State, b: *mut Buffer, i: i64) {
    unsafe {
        lua_geti(state, 1, i);
        if !lua_isstring(state, -1) {
            lual_error(
                state,
                c"invalid value (%s) at index %I in table for 'concat'".as_ptr(),
                lua_typename(state, lua_type(state, -1)),
                i,
            );
        }
        (*b).add_value();
    }
}
pub unsafe fn table_concat(state: *mut State) -> i32 {
    unsafe {
        let mut b = Buffer::new();
        checktab(state, 1, 1 | 4);
        let mut last: i64 = lual_len(state, 1);
        let mut lsep: usize = 0;
        let sep: *const i8 = lual_optlstring(state, 2, c"".as_ptr(), &mut lsep);
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
        1
    }
}
pub unsafe fn table_pack(state: *mut State) -> i32 {
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
        lua_setfield(state, 1, c"n".as_ptr());
        1
    }
}
pub unsafe fn table_unpack(state: *mut State) -> i32 {
    unsafe {
        let mut n: usize;
        let mut i: i64 = lual_optinteger(state, 2, 1);
        let e = match lua_type(state, 3) {
            | None | Some(TagType::Nil) => lual_len(state, 1),
            | _ => lual_checkinteger(state, 3),
        };
        if i > e {
            return 0;
        }
        n = (e as usize).wrapping_sub(i as usize);
        if n >= MAX_INT || {
            n += 1;
            lua_checkstack(state, n as i32) == 0
        } {
            return lual_error(state, c"too many results to unpack".as_ptr());
        }
        while i < e {
            lua_geti(state, 1, i);
            i += 1;
        }
        lua_geti(state, 1, e);
        n as i32
    }
}
pub unsafe fn l_randomizepivot() -> u32 {
    unsafe {
        let dur = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default();
        let c: i64 = dur.subsec_nanos() as i64;
        let t: i64 = dur.as_secs() as i64;
        let mut buffer: [u32; 4] = [0; 4];
        let mut i: u32;
        let mut rnd: u32 = 0;
        std::ptr::copy_nonoverlapping(&c as *const i64 as *const u8, buffer.as_mut_ptr() as *mut u8, size_of::<i64>());
        std::ptr::copy_nonoverlapping(
            &t as *const i64 as *const u8,
            buffer.as_mut_ptr().add(size_of::<i64>() / size_of::<u32>()) as *mut u8,
            size_of::<i64>(),
        );
        i = 0;
        while (i as usize) < 4 {
            rnd = rnd.wrapping_add(buffer[i as usize]);
            i += 1;
        }
        rnd
    }
}
pub unsafe fn set2(state: *mut State, i: u32, j: u32) {
    unsafe {
        lua_seti(state, 1, i as i64);
        lua_seti(state, 1, j as i64);
    }
}
pub unsafe fn sort_comp(state: *mut State, a: i32, b: i32) -> bool {
    unsafe {
        if lua_type(state, 2) == Some(TagType::Nil) {
            lua_compare(state, a, b, 1)
        } else {
            lua_pushvalue(state, 2);
            lua_pushvalue(state, a - 1);
            lua_pushvalue(state, b - 2);
            (*state).lua_callk(2, 1, 0, None);
            let res = lua_toboolean(state, -1);
            lua_settop(state, -2);
            res
        }
    }
}
pub unsafe fn partition(state: *mut State, low: u32, high: u32) -> u32 {
    unsafe {
        let mut i: u32 = low;
        let mut j: u32 = high.wrapping_sub(1_u32);
        loop {
            loop {
                i += 1;
                lua_geti(state, 1, i as i64);
                if !sort_comp(state, -1, -2) {
                    break;
                }
                if i == high - 1 {
                    lual_error(state, c"invalid order function for sorting".as_ptr());
                }
                lua_settop(state, -2);
            }
            loop {
                j -= 1;
                lua_geti(state, 1, j as i64);
                if !sort_comp(state, -3, -1) {
                    break;
                }
                if j < i {
                    lual_error(state, c"invalid order function for sorting".as_ptr());
                }
                lua_settop(state, -2);
            }
            if j < i {
                lua_settop(state, -2);
                set2(state, high.wrapping_sub(1_u32), i);
                return i;
            }
            set2(state, i, j);
        }
    }
}
pub fn choose_pivot(low: u32, high: u32, rnd: u32) -> u32 {
    let r4: u32 = (high - low) / 4;
    let p: u32 = rnd.wrapping_rem(r4.wrapping_mul(2_u32)).wrapping_add(low.wrapping_add(r4));
    p
}
pub unsafe fn auxsort(state: *mut State, mut low: u32, mut high: u32, mut rnd: u32) {
    unsafe {
        while low < high {
            let mut p: u32;
            let n: u32;
            lua_geti(state, 1, low as i64);
            lua_geti(state, 1, high as i64);
            if sort_comp(state, -1, -2) {
                set2(state, low, high);
            } else {
                lua_settop(state, -2 - 1);
            }
            if high - low == 1 {
                return;
            }
            if high - low < 100 || rnd == 0 {
                p = (low + high) / 2;
            } else {
                p = choose_pivot(low, high, rnd);
            }
            lua_geti(state, 1, p as i64);
            lua_geti(state, 1, low as i64);
            if sort_comp(state, -2, -1) {
                set2(state, p, low);
            } else {
                lua_settop(state, -2);
                lua_geti(state, 1, high as i64);
                if sort_comp(state, -1, -2) {
                    set2(state, p, high);
                } else {
                    lua_settop(state, -2 - 1);
                }
            }
            if high.wrapping_sub(low) == 2_u32 {
                return;
            }
            lua_geti(state, 1, p as i64);
            lua_pushvalue(state, -1);
            lua_geti(state, 1, high.wrapping_sub(1_u32) as i64);
            set2(state, p, high.wrapping_sub(1_u32));
            p = partition(state, low, high);
            if p.wrapping_sub(low) < high.wrapping_sub(p) {
                auxsort(state, low, p.wrapping_sub(1_u32), rnd);
                n = p.wrapping_sub(low);
                low = p.wrapping_add(1_u32);
            } else {
                auxsort(state, p.wrapping_add(1_u32), high, rnd);
                n = high.wrapping_sub(p);
                high = p.wrapping_sub(1_u32);
            }
            if ((high - low) / 128) > n {
                rnd = l_randomizepivot();
            }
        }
    }
}
pub unsafe fn table_sort(state: *mut State) -> i32 {
    unsafe {
        checktab(state, 1, 1 | 2 | 4);
        let n: i64 = lual_len(state, 1);
        if n > 1 {
            if n >= MAX_INT as i64 {
                lual_argerror(state, 1, c"array too big".as_ptr());
            }
            match lua_type(state, 2) {
                | None | Some(TagType::Nil) => {},
                | _ => {
                    (*state).lual_checktype(2, TagType::Closure);
                },
            }
            lua_settop(state, 2);
            auxsort(state, 1_u32, n as u32, 0);
        }
        0
    }
}
pub unsafe fn table_create(state: *mut State) -> i32 {
    unsafe {
        let sizeseq: i64 = lual_checkinteger(state, 1);
        let sizerest: i64 = lual_optinteger(state, 2, 0);
        if !(sizeseq >= 0 && sizeseq <= i32::MAX as i64) {
            lual_argerror(state, 1, c"out of range".as_ptr());
            0;
        }
        if !(sizerest >= 0 && sizerest <= i32::MAX as i64) {
            lual_argerror(state, 2, c"out of range".as_ptr());
            0;
        }
        (*state).lua_createtable();
        let table = (*(*state).interpreter_top.stkidrel_pointer.sub(1)).as_table().unwrap();
        if sizeseq > 0 || sizerest > 0 {
            luah_resize(state, table, sizeseq as usize, sizerest as usize);
        }
        1
    }
}
pub const TABLE_FUNCTIONS: [RegisteredFunction; 8] = {
    [
        {
            RegisteredFunction {
                registeredfunction_name: c"concat".as_ptr(),
                registeredfunction_function: Some(table_concat as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"create".as_ptr(),
                registeredfunction_function: Some(table_create as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"insert".as_ptr(),
                registeredfunction_function: Some(table_insert as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"pack".as_ptr(),
                registeredfunction_function: Some(table_pack as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"unpack".as_ptr(),
                registeredfunction_function: Some(table_unpack as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"remove".as_ptr(),
                registeredfunction_function: Some(table_remove as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"move".as_ptr(),
                registeredfunction_function: Some(table_move as unsafe fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"sort".as_ptr(),
                registeredfunction_function: Some(table_sort as unsafe fn(*mut State) -> i32),
            }
        },
    ]
};
pub unsafe fn luaopen_table(state: *mut State) -> i32 {
    unsafe {
        (*state).lua_createtable();
        lual_setfuncs(state, TABLE_FUNCTIONS.as_ptr(), TABLE_FUNCTIONS.len(), 0);
        1
    }
}
