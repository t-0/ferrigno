#![allow(
    static_mut_refs,
    unpredictable_function_pointer_comparisons,
    unsafe_code
)]
use crate::buffer::*;
use crate::c::*;
use crate::debuginfo::*;
use crate::functions::*;
use crate::gmatchstate::*;
use crate::header::*;
use crate::k::*;
use crate::matchstate::*;
use crate::nativeendian::*;
use crate::new::*;
use crate::registeredfunction::*;
use crate::rn::*;
use crate::state::*;
use crate::stream::*;
use crate::streamwriter::*;
use crate::tag::*;
use crate::user::*;
use libc::{remove, rename, setlocale, system, tolower, toupper};
pub unsafe extern "C" fn getco(state: *mut State) -> *mut State {
    unsafe {
        let co: *mut State = lua_tothread(state, 1);
        ((co != std::ptr::null_mut()) as i64 != 0
            || lual_typeerror(state, 1, b"thread\0" as *const u8 as *const i8) != 0) as i32;
        return co;
    }
}
pub unsafe extern "C" fn auxresume(state: *mut State, co: *mut State, narg: i32) -> i32 {
    unsafe {
        let status: i32;
        let mut nres: i32 = 0;
        if ((lua_checkstack(co, narg) == 0) as i32 != 0) as i64 != 0 {
            lua_pushstring(
                state,
                b"too many arguments to resume\0" as *const u8 as *const i8,
            );
            return -1;
        }
        lua_xmove(state, co, narg);
        status = lua_resume(co, state, narg, &mut nres);
        if ((status == 0 || status == 1) as i32 != 0) as i64 != 0 {
            if ((lua_checkstack(state, nres + 1) == 0) as i32 != 0) as i64 != 0 {
                lua_settop(co, -nres - 1);
                lua_pushstring(
                    state,
                    b"too many results to resume\0" as *const u8 as *const i8,
                );
                return -1;
            }
            lua_xmove(co, state, nres);
            return nres;
        } else {
            lua_xmove(co, state, 1);
            return -1;
        };
    }
}
pub unsafe extern "C" fn luab_auxwrap(state: *mut State) -> i32 {
    unsafe {
        let co: *mut State = lua_tothread(state, -(1000000 as i32) - 1000 as i32 - 1);
        let r: i32 = auxresume(state, co, (*state).get_top());
        if ((r < 0) as i32 != 0) as i64 != 0 {
            let mut stat: i32 = (*co).get_status();
            if stat != 0 && stat != 1 {
                stat = lua_closethread(co, state);
                lua_xmove(co, state, 1);
            }
            if stat != 4 && lua_type(state, -1) == Some(TAG_TYPE_STRING) {
                lual_where(state, 1);
                lua_rotate(state, -2, 1);
                lua_concat(state, 2);
            }
            return lua_error(state);
        }
        return r;
    }
}

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
pub unsafe extern "C" fn tinsert(state: *mut State) -> i32 {
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
pub unsafe extern "C" fn tremove(state: *mut State) -> i32 {
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
pub unsafe extern "C" fn tmove(state: *mut State) -> i32 {
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
        (*b).lual_addvalue();
    }
}
pub unsafe extern "C" fn tconcat(state: *mut State) -> i32 {
    unsafe {
        let mut b = Buffer::new();
        checktab(state, 1, 1 | 4);
        let mut last: i64 = lual_len(state, 1);
        let mut lsep: u64 = 0;
        let sep: *const i8 = lual_optlstring(state, 2, b"\0" as *const u8 as *const i8, &mut lsep);
        let mut i: i64 = lual_optinteger(state, 3, 1);
        last = lual_optinteger(state, 4, last);
        b.lual_buffinit(state);
        while i < last {
            addfield(state, &mut b, i);
            b.lual_addlstring(sep, lsep);
            i += 1;
        }
        if i == last {
            addfield(state, &mut b, i);
        }
        b.lual_pushresult();
        return 1;
    }
}
pub unsafe extern "C" fn tpack(state: *mut State) -> i32 {
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
pub unsafe extern "C" fn tunpack(state: *mut State) -> i32 {
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
pub unsafe extern "C" fn sort(state: *mut State) -> i32 {
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
                function: Some(tconcat as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"insert\0" as *const u8 as *const i8,
                function: Some(tinsert as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"pack\0" as *const u8 as *const i8,
                function: Some(tpack as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"unpack\0" as *const u8 as *const i8,
                function: Some(tunpack as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"remove\0" as *const u8 as *const i8,
                function: Some(tremove as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"move\0" as *const u8 as *const i8,
                function: Some(tmove as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"sort\0" as *const u8 as *const i8,
                function: Some(sort as unsafe extern "C" fn(*mut State) -> i32),
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
pub unsafe extern "C" fn l_checkmode(mut mode: *const i8) -> i32 {
    unsafe {
        return (*mode as i32 != '\0' as i32
            && {
                let fresh151 = mode;
                mode = mode.offset(1);
                !(strchr(b"rwa\0" as *const u8 as *const i8, *fresh151 as i32)).is_null()
            }
            && (*mode as i32 != '+' as i32 || {
                mode = mode.offset(1);
                1 != 0
            })
            && strspn(mode, b"b\0" as *const u8 as *const i8) == strlen(mode))
            as i32;
    }
}
pub unsafe extern "C" fn io_type(state: *mut State) -> i32 {
    unsafe {
        let p: *mut Stream;
        lual_checkany(state, 1);
        p = lual_testudata(state, 1, b"FILE*\0" as *const u8 as *const i8) as *mut Stream;
        if p.is_null() {
            (*state).push_nil();
        } else if ((*p).closef).is_none() {
            lua_pushstring(state, b"closed file\0" as *const u8 as *const i8);
        } else {
            lua_pushstring(state, b"file\0" as *const u8 as *const i8);
        }
        return 1;
    }
}
pub unsafe extern "C" fn f_tostring(state: *mut State) -> i32 {
    unsafe {
        let p: *mut Stream =
            lual_checkudata(state, 1, b"FILE*\0" as *const u8 as *const i8) as *mut Stream;
        if ((*p).closef).is_none() {
            lua_pushstring(state, b"file (closed)\0" as *const u8 as *const i8);
        } else {
            lua_pushfstring(state, b"file (%p)\0" as *const u8 as *const i8, (*p).f);
        }
        return 1;
    }
}
pub unsafe extern "C" fn tofile(state: *mut State) -> *mut FILE {
    unsafe {
        let p: *mut Stream =
            lual_checkudata(state, 1, b"FILE*\0" as *const u8 as *const i8) as *mut Stream;
        if (((*p).closef).is_none() as i32 != 0) as i64 != 0 {
            lual_error(
                state,
                b"attempt to use a closed file\0" as *const u8 as *const i8,
            );
        }
        return (*p).f;
    }
}
pub unsafe extern "C" fn newprefile(state: *mut State) -> *mut Stream {
    unsafe {
        let p: *mut Stream =
            User::lua_newuserdatauv(state, ::core::mem::size_of::<Stream>() as u64, 0) as *mut Stream;
        (*p).closef = None;
        lual_setmetatable(state, b"FILE*\0" as *const u8 as *const i8);
        return p;
    }
}
pub unsafe extern "C" fn aux_close(state: *mut State) -> i32 {
    unsafe {
        let p: *mut Stream =
            lual_checkudata(state, 1, b"FILE*\0" as *const u8 as *const i8) as *mut Stream;
        let cf: CFunction = (*p).closef;
        (*p).closef = None;
        return (Some(cf.expect("non-null function pointer"))).expect("non-null function pointer")(
            state,
        );
    }
}
pub unsafe extern "C" fn f_close(state: *mut State) -> i32 {
    unsafe {
        tofile(state);
        return aux_close(state);
    }
}
pub unsafe extern "C" fn io_close(state: *mut State) -> i32 {
    unsafe {
        if lua_type(state, 1) == None {
            lua_getfield(
                state,
                -(1000000 as i32) - 1000 as i32,
                b"_IO_output\0" as *const u8 as *const i8,
            );
        }
        return f_close(state);
    }
}
pub unsafe extern "C" fn f_gc(state: *mut State) -> i32 {
    unsafe {
        let p: *mut Stream =
            lual_checkudata(state, 1, b"FILE*\0" as *const u8 as *const i8) as *mut Stream;
        if ((*p).closef).is_some() && !((*p).f).is_null() {
            aux_close(state);
        }
        return 0;
    }
}
pub unsafe extern "C" fn io_fclose(state: *mut State) -> i32 {
    unsafe {
        let p: *mut Stream =
            lual_checkudata(state, 1, b"FILE*\0" as *const u8 as *const i8) as *mut Stream;
        *__errno_location() = 0;
        return lual_fileresult(state, (fclose((*p).f) == 0) as i32, std::ptr::null());
    }
}
pub unsafe extern "C" fn newfile(state: *mut State) -> *mut Stream {
    unsafe {
        let p: *mut Stream = newprefile(state);
        (*p).f = std::ptr::null_mut();
        (*p).closef = Some(io_fclose as unsafe extern "C" fn(*mut State) -> i32);
        return p;
    }
}
pub unsafe extern "C" fn opencheck(state: *mut State, fname: *const i8, mode: *const i8) {
    unsafe {
        let p: *mut Stream = newfile(state);
        (*p).f = fopen(fname, mode);
        if (((*p).f == std::ptr::null_mut() as *mut FILE) as i32 != 0) as i64 != 0 {
            lual_error(
                state,
                b"cannot open file '%s' (%s)\0" as *const u8 as *const i8,
                fname,
                strerror(*__errno_location()),
            );
        }
    }
}
pub unsafe extern "C" fn io_open(state: *mut State) -> i32 {
    unsafe {
        let filename: *const i8 = lual_checklstring(state, 1, std::ptr::null_mut());
        let mode: *const i8 = lual_optlstring(
            state,
            2,
            b"r\0" as *const u8 as *const i8,
            std::ptr::null_mut(),
        );
        let p: *mut Stream = newfile(state);
        let md: *const i8 = mode;
        ((l_checkmode(md) != 0) as i64 != 0
            || lual_argerror(state, 2, b"invalid mode\0" as *const u8 as *const i8) != 0)
            as i32;
        *__errno_location() = 0;
        (*p).f = fopen(filename, mode);
        return if ((*p).f).is_null() {
            lual_fileresult(state, 0, filename)
        } else {
            1
        };
    }
}
pub unsafe extern "C" fn io_pclose(state: *mut State) -> i32 {
    unsafe {
        let p: *mut Stream =
            lual_checkudata(state, 1, b"FILE*\0" as *const u8 as *const i8) as *mut Stream;
        *__errno_location() = 0;
        return lual_execresult(state, pclose((*p).f));
    }
}
pub unsafe extern "C" fn io_popen(state: *mut State) -> i32 {
    unsafe {
        let filename: *const i8 = lual_checklstring(state, 1, std::ptr::null_mut());
        let mode: *const i8 = lual_optlstring(
            state,
            2,
            b"r\0" as *const u8 as *const i8,
            std::ptr::null_mut(),
        );
        let p: *mut Stream = newprefile(state);
        ((((*mode.offset(0 as isize) as i32 == 'r' as i32
            || *mode.offset(0 as isize) as i32 == 'w' as i32)
            && *mode.offset(1 as isize) as i32 == '\0' as i32) as i32
            != 0) as i64
            != 0
            || lual_argerror(state, 2, b"invalid mode\0" as *const u8 as *const i8) != 0)
            as i32;
        *__errno_location() = 0;
        fflush(std::ptr::null_mut());
        (*p).f = popen(filename, mode);
        (*p).closef = Some(io_pclose as unsafe extern "C" fn(*mut State) -> i32);
        return if ((*p).f).is_null() {
            lual_fileresult(state, 0, filename)
        } else {
            1
        };
    }
}
pub unsafe extern "C" fn io_tmpfile(state: *mut State) -> i32 {
    unsafe {
        let p: *mut Stream = newfile(state);
        *__errno_location() = 0;
        (*p).f = tmpfile();
        return if ((*p).f).is_null() {
            lual_fileresult(state, 0, std::ptr::null())
        } else {
            1
        };
    }
}
pub unsafe extern "C" fn getiofile(state: *mut State, findex: *const i8) -> *mut FILE {
    unsafe {
        let p: *mut Stream;
        lua_getfield(state, -(1000000 as i32) - 1000 as i32, findex);
        p = lua_touserdata(state, -1) as *mut Stream;
        if (((*p).closef).is_none() as i32 != 0) as i64 != 0 {
            lual_error(
                state,
                b"default %s file is closed\0" as *const u8 as *const i8,
                findex.offset(
                    (::core::mem::size_of::<[i8; 5]>() as u64)
                        .wrapping_div(::core::mem::size_of::<i8>() as u64)
                        .wrapping_sub(1 as u64) as isize,
                ),
            );
        }
        return (*p).f;
    }
}
pub unsafe extern "C" fn g_iofile(state: *mut State, f: *const i8, mode: *const i8) -> i32 {
    unsafe {
        if !(is_none_or_nil(lua_type(state, 1))) {
            let filename: *const i8 = lua_tolstring(state, 1, std::ptr::null_mut());
            if !filename.is_null() {
                opencheck(state, filename, mode);
            } else {
                tofile(state);
                lua_pushvalue(state, 1);
            }
            lua_setfield(state, -(1000000 as i32) - 1000 as i32, f);
        }
        lua_getfield(state, -(1000000 as i32) - 1000 as i32, f);
        return 1;
    }
}
pub unsafe extern "C" fn io_input(state: *mut State) -> i32 {
    unsafe {
        return g_iofile(
            state,
            b"_IO_input\0" as *const u8 as *const i8,
            b"r\0" as *const u8 as *const i8,
        );
    }
}
pub unsafe extern "C" fn io_output(state: *mut State) -> i32 {
    unsafe {
        return g_iofile(
            state,
            b"_IO_output\0" as *const u8 as *const i8,
            b"w\0" as *const u8 as *const i8,
        );
    }
}
pub unsafe extern "C" fn aux_lines(state: *mut State, to_close: bool) {
    unsafe {
        let n: i32 = (*state).get_top() - 1;
        (((n <= 250 as i32) as i32 != 0) as i64 != 0
            || lual_argerror(
                state,
                250 as i32 + 2,
                b"too many arguments\0" as *const u8 as *const i8,
            ) != 0) as i32;
        lua_pushvalue(state, 1);
        (*state).push_integer(n as i64);
        (*state).push_boolean(to_close);
        lua_rotate(state, 2, 3);
        lua_pushcclosure(
            state,
            Some(io_readline as unsafe extern "C" fn(*mut State) -> i32),
            3 + n,
        );
    }
}
pub unsafe extern "C" fn f_lines(state: *mut State) -> i32 {
    unsafe {
        tofile(state);
        aux_lines(state, false);
        return 1;
    }
}
pub unsafe extern "C" fn io_lines(state: *mut State) -> i32 {
    unsafe {
        let to_close: bool;
        if lua_type(state, 1) == None {
            (*state).push_nil();
        }
        if lua_type(state, 1) == Some(TAG_TYPE_NIL) {
            lua_getfield(
                state,
                -(1000000 as i32) - 1000 as i32,
                b"_IO_input\0" as *const u8 as *const i8,
            );
            lua_copy(state, -1, 1);
            lua_settop(state, -2);
            tofile(state);
            to_close = false;
        } else {
            let filename: *const i8 = lual_checklstring(state, 1, std::ptr::null_mut());
            opencheck(state, filename, b"r\0" as *const u8 as *const i8);
            lua_copy(state, -1, 1);
            lua_settop(state, -2);
            to_close = true;
        }
        aux_lines(state, to_close);
        if to_close {
            (*state).push_nil();
            (*state).push_nil();
            lua_pushvalue(state, 1);
            return 4;
        } else {
            return 1;
        };
    }
}
pub unsafe extern "C" fn nextc(rn: *mut RN) -> i32 {
    unsafe {
        if (((*rn).n >= 200 as i32) as i32 != 0) as i64 != 0 {
            (*rn).buffer[0] = '\0' as i8;
            return 0;
        } else {
            let fresh152 = (*rn).n;
            (*rn).n = (*rn).n + 1;
            (*rn).buffer[fresh152 as usize] = (*rn).c as i8;
            (*rn).c = getc_unlocked((*rn).f);
            return 1;
        };
    }
}
pub unsafe extern "C" fn test2(rn: *mut RN, set: *const i8) -> i32 {
    unsafe {
        if (*rn).c == *set.offset(0 as isize) as i32 || (*rn).c == *set.offset(1 as isize) as i32 {
            return nextc(rn);
        } else {
            return 0;
        };
    }
}
pub unsafe extern "C" fn readdigits(rn: *mut RN, hex: i32) -> i32 {
    unsafe {
        let mut count: i32 = 0;
        while (if hex != 0 {
            *(*__ctype_b_loc()).offset((*rn).c as isize) as i32 & _ISXDIGIT as i32
        } else {
            *(*__ctype_b_loc()).offset((*rn).c as isize) as i32 & _ISDIGIT as i32
        }) != 0
            && nextc(rn) != 0
        {
            count += 1;
        }
        return count;
    }
}
pub unsafe extern "C" fn read_number(state: *mut State, f: *mut FILE) -> i32 {
    unsafe {
        let mut rn: RN = RN {
            f: std::ptr::null_mut(),
            c: 0,
            n: 0,
            buffer: [0; 201],
        };
        let mut count: i32 = 0;
        let mut hex: i32 = 0;
        let mut decp: [i8; 2] = [0; 2];
        rn.f = f;
        rn.n = 0;
        decp[0] = '.' as i8;
        decp[1] = '.' as i8;
        flockfile(rn.f);
        loop {
            rn.c = getc_unlocked(rn.f);
            if !(*(*__ctype_b_loc()).offset(rn.c as isize) as i32 & _ISSPACE as i32
                != 0)
            {
                break;
            }
        }
        test2(&mut rn, b"-+\0" as *const u8 as *const i8);
        if test2(&mut rn, b"00\0" as *const u8 as *const i8) != 0 {
            if test2(&mut rn, b"xX\0" as *const u8 as *const i8) != 0 {
                hex = 1;
            } else {
                count = 1;
            }
        }
        count += readdigits(&mut rn, hex);
        if test2(&mut rn, decp.as_mut_ptr()) != 0 {
            count += readdigits(&mut rn, hex);
        }
        if count > 0
            && test2(
                &mut rn,
                if hex != 0 {
                    b"pP\0" as *const u8 as *const i8
                } else {
                    b"eE\0" as *const u8 as *const i8
                },
            ) != 0
        {
            test2(&mut rn, b"-+\0" as *const u8 as *const i8);
            readdigits(&mut rn, 0);
        }
        ungetc(rn.c, rn.f);
        funlockfile(rn.f);
        rn.buffer[rn.n as usize] = '\0' as i8;
        if (lua_stringtonumber(state, (rn.buffer).as_mut_ptr()) != 0u64) as i64 != 0 {
            return 1;
        } else {
            (*state).push_nil();
            return 0;
        };
    }
}
pub unsafe extern "C" fn test_eof(state: *mut State, f: *mut FILE) -> i32 {
    unsafe {
        let c: i32 = getc(f);
        ungetc(c, f);
        lua_pushstring(state, b"\0" as *const u8 as *const i8);
        return (c != -1) as i32;
    }
}
pub unsafe extern "C" fn read_line(state: *mut State, f: *mut FILE, chop: i32) -> i32 {
    unsafe {
        let mut b = Buffer::new();
        let mut c: i32 = 0;
        b.lual_buffinit(state);
        loop {
            let buffer: *mut i8 = b.lual_prepbuffsize(
                (16 as u64)
                    .wrapping_mul(::core::mem::size_of::<*mut libc::c_void>() as u64)
                    .wrapping_mul(::core::mem::size_of::<f64>() as u64) as i32
                    as u64,
            );
            let mut i: i32 = 0;
            flockfile(f);
            while i
                < (16 as u64)
                    .wrapping_mul(::core::mem::size_of::<*mut libc::c_void>() as u64)
                    .wrapping_mul(::core::mem::size_of::<f64>() as u64) as i32
                && {
                    c = getc_unlocked(f);
                    c != -1
                }
                && c != '\n' as i32
            {
                let fresh153 = i;
                i = i + 1;
                *buffer.offset(fresh153 as isize) = c as i8;
            }
            funlockfile(f);
            b.length = (b.length as u64).wrapping_add(i as u64) as u64;
            if !(c != -1 && c != '\n' as i32) {
                break;
            }
        }
        if chop == 0 && c == '\n' as i32 {
            (b.length < b.size || !(b.lual_prepbuffsize(1 as u64)).is_null()) as i32;
            let fresh154 = b.length;
            b.length = (b.length).wrapping_add(1);
            *(b.pointer).offset(fresh154 as isize) = c as i8;
        }
        b.lual_pushresult();
        return (c == '\n' as i32 || lua_rawlen(state, -1) > 0u64) as i32;
    }
}
pub unsafe extern "C" fn read_all(state: *mut State, f: *mut FILE) {
    unsafe {
        let mut nr: u64;
        let mut b = Buffer::new();
        b.lual_buffinit(state);
        loop {
            let p: *mut i8 = b.lual_prepbuffsize(
                (16 as u64)
                    .wrapping_mul(::core::mem::size_of::<*mut libc::c_void>() as u64)
                    .wrapping_mul(::core::mem::size_of::<f64>() as u64) as i32
                    as u64,
            );
            nr = fread(
                p as *mut libc::c_void,
                ::core::mem::size_of::<i8>() as u64,
                (16 as u64)
                    .wrapping_mul(::core::mem::size_of::<*mut libc::c_void>() as u64)
                    .wrapping_mul(::core::mem::size_of::<f64>() as u64) as i32
                    as u64,
                f,
            );
            b.length = (b.length as u64).wrapping_add(nr) as u64;
            if !(nr
                == (16 as u64)
                    .wrapping_mul(::core::mem::size_of::<*mut libc::c_void>() as u64)
                    .wrapping_mul(::core::mem::size_of::<f64>() as u64) as i32
                    as u64)
            {
                break;
            }
        }
        b.lual_pushresult();
    }
}
pub unsafe extern "C" fn read_chars(state: *mut State, f: *mut FILE, n: u64) -> i32 {
    unsafe {
        let nr: u64;
        let p: *mut i8;
        let mut b = Buffer::new();
        b.lual_buffinit(state);
        p = b.lual_prepbuffsize(n);
        nr = fread(
            p as *mut libc::c_void,
            ::core::mem::size_of::<i8>() as u64,
            n,
            f,
        );
        b.length = (b.length as u64).wrapping_add(nr) as u64;
        b.lual_pushresult();
        return (nr > 0u64) as i32;
    }
}
pub unsafe extern "C" fn g_read(state: *mut State, f: *mut FILE, first: i32) -> i32 {
    unsafe {
        let mut nargs: i32 = (*state).get_top() - 1;
        let mut n: i32;
        let mut success: i32;
        clearerr(f);
        *__errno_location() = 0;
        if nargs == 0 {
            success = read_line(state, f, 1);
            n = first + 1;
        } else {
            lual_checkstack(
                state,
                nargs + 20 as i32,
                b"too many arguments\0" as *const u8 as *const i8,
            );
            success = 1;
            n = first;
            loop {
                let fresh155 = nargs;
                nargs = nargs - 1;
                if !(fresh155 != 0 && success != 0) {
                    break;
                }
                if lua_type(state, n) == Some(TAG_TYPE_NUMERIC) {
                    let l: u64 = lual_checkinteger(state, n) as u64;
                    success = if l == 0u64 {
                        test_eof(state, f)
                    } else {
                        read_chars(state, f, l)
                    };
                } else {
                    let mut p: *const i8 = lual_checklstring(state, n, std::ptr::null_mut());
                    if *p as i32 == '*' as i32 {
                        p = p.offset(1);
                    }
                    match *p as i32 {
                        110 => {
                            success = read_number(state, f);
                        }
                        108 => {
                            success = read_line(state, f, 1);
                        }
                        76 => {
                            success = read_line(state, f, 0);
                        }
                        97 => {
                            read_all(state, f);
                            success = 1;
                        }
                        _ => {
                            return lual_argerror(
                                state,
                                n,
                                b"invalid format\0" as *const u8 as *const i8,
                            );
                        }
                    }
                }
                n += 1;
            }
        }
        if ferror(f) != 0 {
            return lual_fileresult(state, 0, std::ptr::null());
        }
        if success == 0 {
            lua_settop(state, -2);
            (*state).push_nil();
        }
        return n - first;
    }
}
pub unsafe extern "C" fn io_read(state: *mut State) -> i32 {
    unsafe {
        return g_read(
            state,
            getiofile(state, b"_IO_input\0" as *const u8 as *const i8),
            1,
        );
    }
}
pub unsafe extern "C" fn f_read(state: *mut State) -> i32 {
    unsafe {
        return g_read(state, tofile(state), 2);
    }
}
pub unsafe extern "C" fn io_readline(state: *mut State) -> i32 {
    unsafe {
        let p: *mut Stream =
            lua_touserdata(state, -(1000000 as i32) - 1000 as i32 - 1) as *mut Stream;
        let mut i: i32;
        let mut n: i32 = lua_tointegerx(
            state,
            -(1000000 as i32) - 1000 as i32 - 2,
            std::ptr::null_mut(),
        ) as i32;
        if ((*p).closef).is_none() {
            return lual_error(state, b"file is already closed\0" as *const u8 as *const i8);
        }
        lua_settop(state, 1);
        lual_checkstack(state, n, b"too many arguments\0" as *const u8 as *const i8);
        i = 1;
        while i <= n {
            lua_pushvalue(state, -(1000000 as i32) - 1000 as i32 - (3 + i));
            i += 1;
        }
        n = g_read(state, (*p).f, 2);
        if lua_toboolean(state, -n) != 0 {
            return n;
        } else {
            if n > 1 {
                return lual_error(
                    state,
                    b"%s\0" as *const u8 as *const i8,
                    lua_tolstring(state, -n + 1, std::ptr::null_mut()),
                );
            }
            if lua_toboolean(state, -(1000000 as i32) - 1000 as i32 - 3) != 0 {
                lua_settop(state, 0);
                lua_pushvalue(state, -(1000000 as i32) - 1000 as i32 - 1);
                aux_close(state);
            }
            return 0;
        };
    }
}
pub unsafe extern "C" fn g_write(state: *mut State, f: *mut FILE, mut arg: i32) -> i32 {
    unsafe {
        let mut nargs: i32 = (*state).get_top() - arg;
        let mut status: i32 = 1;
        *__errno_location() = 0;
        loop {
            let fresh156 = nargs;
            nargs = nargs - 1;
            if !(fresh156 != 0) {
                break;
            }
            if lua_type(state, arg) == Some(TAG_TYPE_NUMERIC) {
                let length: i32 = if lua_isinteger(state, arg) {
                    fprintf(
                        f,
                        b"%lld\0" as *const u8 as *const i8,
                        lua_tointegerx(state, arg, std::ptr::null_mut()),
                    )
                } else {
                    fprintf(
                        f,
                        b"%.14g\0" as *const u8 as *const i8,
                        lua_tonumberx(state, arg, std::ptr::null_mut()),
                    )
                };
                status = (status != 0 && length > 0) as i32;
            } else {
                let mut l: u64 = 0;
                let s: *const i8 = lual_checklstring(state, arg, &mut l);
                status = (status != 0
                    && fwrite(
                        s as *const libc::c_void,
                        ::core::mem::size_of::<i8>() as u64,
                        l,
                        f,
                    ) == l) as i32;
            }
            arg += 1;
        }
        if (status != 0) as i64 != 0 {
            return 1;
        } else {
            return lual_fileresult(state, status, std::ptr::null());
        };
    }
}
pub unsafe extern "C" fn io_write(state: *mut State) -> i32 {
    unsafe {
        return g_write(
            state,
            getiofile(state, b"_IO_output\0" as *const u8 as *const i8),
            1,
        );
    }
}
pub unsafe extern "C" fn f_write(state: *mut State) -> i32 {
    unsafe {
        let f: *mut FILE = tofile(state);
        lua_pushvalue(state, 1);
        return g_write(state, f, 2);
    }
}
pub unsafe extern "C" fn f_seek(state: *mut State) -> i32 {
    unsafe {
        pub const MODE: [i32; 3] = [0, 1, 2];
        pub const MODE_NAMES: [*const i8; 4] = [
            b"set\0" as *const u8 as *const i8,
            b"cur\0" as *const u8 as *const i8,
            b"end\0" as *const u8 as *const i8,
            std::ptr::null(),
        ];
        let f: *mut FILE = tofile(state);
        let mut op: i32 = lual_checkoption(
            state,
            2,
            b"cur\0" as *const u8 as *const i8,
            MODE_NAMES.as_ptr(),
        );
        let p3: i64 = lual_optinteger(state, 3, 0);
        let offset: i64 = p3 as i64;
        (((offset as i64 == p3) as i32 != 0) as i64 != 0
            || lual_argerror(
                state,
                3,
                b"not an integer in proper range\0" as *const u8 as *const i8,
            ) != 0) as i32;
        *__errno_location() = 0;
        op = fseeko(f, offset, MODE[op as usize]);
        if (op != 0) as i64 != 0 {
            return lual_fileresult(state, 0, std::ptr::null());
        } else {
            (*state).push_integer(ftello(f) as i64);
            return 1;
        };
    }
}
pub unsafe extern "C" fn f_setvbuf(state: *mut State) -> i32 {
    unsafe {
        pub const MODE: [i32; 3] = [2, 0, 1];
        pub const MODE_NAMES: [*const i8; 4] = [
            b"no\0" as *const u8 as *const i8,
            b"full\0" as *const u8 as *const i8,
            b"line\0" as *const u8 as *const i8,
            std::ptr::null(),
        ];
        let f: *mut FILE = tofile(state);
        let op: i32 = lual_checkoption(state, 2, std::ptr::null(), MODE_NAMES.as_ptr());
        let size: i64 = lual_optinteger(
            state,
            3,
            (16 as u64)
                .wrapping_mul(::core::mem::size_of::<*mut libc::c_void>() as u64)
                .wrapping_mul(::core::mem::size_of::<f64>() as u64) as i64,
        );
        let res: i32;
        *__errno_location() = 0;
        res = setvbuf(f, std::ptr::null_mut(), MODE[op as usize], size as u64);
        return lual_fileresult(state, (res == 0) as i32, std::ptr::null());
    }
}
pub unsafe extern "C" fn io_flush(state: *mut State) -> i32 {
    unsafe {
        let f: *mut FILE = getiofile(state, b"_IO_output\0" as *const u8 as *const i8);
        *__errno_location() = 0;
        return lual_fileresult(state, (fflush(f) == 0) as i32, std::ptr::null());
    }
}
pub unsafe extern "C" fn f_flush(state: *mut State) -> i32 {
    unsafe {
        let f: *mut FILE = tofile(state);
        *__errno_location() = 0;
        return lual_fileresult(state, (fflush(f) == 0) as i32, std::ptr::null());
    }
}
pub const IO_FUNCTIONS: [RegisteredFunction; 12] = {
    [
        {
            RegisteredFunction {
                name: b"close\0" as *const u8 as *const i8,
                function: Some(io_close as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"flush\0" as *const u8 as *const i8,
                function: Some(io_flush as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"input\0" as *const u8 as *const i8,
                function: Some(io_input as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"lines\0" as *const u8 as *const i8,
                function: Some(io_lines as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"open\0" as *const u8 as *const i8,
                function: Some(io_open as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"output\0" as *const u8 as *const i8,
                function: Some(io_output as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"popen\0" as *const u8 as *const i8,
                function: Some(io_popen as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"read\0" as *const u8 as *const i8,
                function: Some(io_read as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"tmpfile\0" as *const u8 as *const i8,
                function: Some(io_tmpfile as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"type\0" as *const u8 as *const i8,
                function: Some(io_type as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"write\0" as *const u8 as *const i8,
                function: Some(io_write as unsafe extern "C" fn(*mut State) -> i32),
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
pub const IO_METHODS: [RegisteredFunction; 8] = {
    [
        {
            RegisteredFunction {
                name: b"read\0" as *const u8 as *const i8,
                function: Some(f_read as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"write\0" as *const u8 as *const i8,
                function: Some(f_write as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"lines\0" as *const u8 as *const i8,
                function: Some(f_lines as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"flush\0" as *const u8 as *const i8,
                function: Some(f_flush as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"seek\0" as *const u8 as *const i8,
                function: Some(f_seek as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"close\0" as *const u8 as *const i8,
                function: Some(f_close as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"setvbuf\0" as *const u8 as *const i8,
                function: Some(f_setvbuf as unsafe extern "C" fn(*mut State) -> i32),
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
pub const IO_METAMETHODS: [RegisteredFunction; 5] = {
    [
        {
            RegisteredFunction {
                name: b"__index\0" as *const u8 as *const i8,
                function: None,
            }
        },
        {
            RegisteredFunction {
                name: b"__gc\0" as *const u8 as *const i8,
                function: Some(f_gc as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"__close\0" as *const u8 as *const i8,
                function: Some(f_gc as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"__tostring\0" as *const u8 as *const i8,
                function: Some(f_tostring as unsafe extern "C" fn(*mut State) -> i32),
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
pub unsafe extern "C" fn createmeta(state: *mut State) {
    unsafe {
        lual_newmetatable(state, b"FILE*\0" as *const u8 as *const i8);
        lual_setfuncs(state, IO_METAMETHODS.as_ptr(), 0);
        (*state).lua_createtable();
        lual_setfuncs(state, IO_METHODS.as_ptr(), 0);
        lua_setfield(state, -2, b"__index\0" as *const u8 as *const i8);
        lua_settop(state, -2);
    }
}
pub unsafe extern "C" fn io_noclose(state: *mut State) -> i32 {
    unsafe {
        let p: *mut Stream =
            lual_checkudata(state, 1, b"FILE*\0" as *const u8 as *const i8) as *mut Stream;
        (*p).closef = Some(io_noclose as unsafe extern "C" fn(*mut State) -> i32);
        (*state).push_nil();
        lua_pushstring(
            state,
            b"cannot close standard file\0" as *const u8 as *const i8,
        );
        return 2;
    }
}
pub unsafe extern "C" fn createstdfile(
    state: *mut State,
    f: *mut FILE,
    k: *const i8,
    fname: *const i8,
) {
    unsafe {
        let p: *mut Stream = newprefile(state);
        (*p).f = f;
        (*p).closef = Some(io_noclose as unsafe extern "C" fn(*mut State) -> i32);
        if !k.is_null() {
            lua_pushvalue(state, -1);
            lua_setfield(state, -(1000000 as i32) - 1000 as i32, k);
        }
        lua_setfield(state, -2, fname);
    }
}
pub unsafe extern "C" fn luaopen_io(state: *mut State) -> i32 {
    unsafe {
        lual_checkversion_(
            state,
            504.0,
            (::core::mem::size_of::<i64>() as u64)
                .wrapping_mul(16 as u64)
                .wrapping_add(::core::mem::size_of::<f64>() as u64),
        );
        (*state).lua_createtable();
        lual_setfuncs(state, IO_FUNCTIONS.as_ptr(), 0);
        createmeta(state);
        createstdfile(
            state,
            stdin,
            b"_IO_input\0" as *const u8 as *const i8,
            b"stdin\0" as *const u8 as *const i8,
        );
        createstdfile(
            state,
            stdout,
            b"_IO_output\0" as *const u8 as *const i8,
            b"stdout\0" as *const u8 as *const i8,
        );
        createstdfile(
            state,
            stderr,
            std::ptr::null(),
            b"stderr\0" as *const u8 as *const i8,
        );
        return 1;
    }
}
pub unsafe extern "C" fn os_execute(state: *mut State) -> i32 {
    unsafe {
        let cmd: *const i8 = lual_optlstring(state, 1, std::ptr::null(), std::ptr::null_mut());
        let stat: i32;
        *__errno_location() = 0;
        stat = system(cmd);
        if !cmd.is_null() {
            return lual_execresult(state, stat);
        } else {
            (*state).push_boolean(0 != stat);
            return 1;
        };
    }
}
pub unsafe extern "C" fn os_remove(state: *mut State) -> i32 {
    unsafe {
        let filename: *const i8 = lual_checklstring(state, 1, std::ptr::null_mut());
        *__errno_location() = 0;
        return lual_fileresult(state, (remove(filename) == 0) as i32, filename);
    }
}
pub unsafe extern "C" fn os_rename(state: *mut State) -> i32 {
    unsafe {
        let fromname: *const i8 = lual_checklstring(state, 1, std::ptr::null_mut());
        let toname: *const i8 = lual_checklstring(state, 2, std::ptr::null_mut());
        *__errno_location() = 0;
        return lual_fileresult(
            state,
            (rename(fromname, toname) == 0) as i32,
            std::ptr::null(),
        );
    }
}
pub unsafe extern "C" fn os_tmpname(state: *mut State) -> i32 {
    unsafe {
        let mut buffer: [i8; 32] = [0; 32];
        let mut err: i32;
        strcpy(
            buffer.as_mut_ptr(),
            b"/tmp/lua_XXXXXX\0" as *const u8 as *const i8,
        );
        err = mkstemp(buffer.as_mut_ptr());
        if err != -1 {
            close(err);
        }
        err = (err == -1) as i32;
        if (err != 0) as i64 != 0 {
            return lual_error(
                state,
                b"unable to generate a unique filename\0" as *const u8 as *const i8,
            );
        }
        lua_pushstring(state, buffer.as_mut_ptr());
        return 1;
    }
}
pub unsafe extern "C" fn os_getenv(state: *mut State) -> i32 {
    unsafe {
        lua_pushstring(
            state,
            getenv(lual_checklstring(state, 1, std::ptr::null_mut())),
        );
        return 1;
    }
}
pub unsafe extern "C" fn os_clock(state: *mut State) -> i32 {
    unsafe {
        (*state).push_number(clock() as f64 / 1000000 as f64);
        return 1;
    }
}
pub unsafe extern "C" fn setfield(state: *mut State, key: *const i8, value: i32, delta: i32) {
    unsafe {
        (*state).push_integer(value as i64 + delta as i64);
        lua_setfield(state, -2, key);
    }
}
pub unsafe extern "C" fn setboolfield(state: *mut State, key: *const i8, value: bool) {
    unsafe {
        (*state).push_boolean(value);
        lua_setfield(state, -2, key);
    }
}
pub unsafe extern "C" fn setallfields(state: *mut State, stm: *mut TM) {
    unsafe {
        setfield(
            state,
            b"year\0" as *const u8 as *const i8,
            (*stm).tm_year,
            1900 as i32,
        );
        setfield(
            state,
            b"month\0" as *const u8 as *const i8,
            (*stm).tm_mon,
            1,
        );
        setfield(state, b"day\0" as *const u8 as *const i8, (*stm).tm_mday, 0);
        setfield(
            state,
            b"hour\0" as *const u8 as *const i8,
            (*stm).tm_hour,
            0,
        );
        setfield(state, b"min\0" as *const u8 as *const i8, (*stm).tm_min, 0);
        setfield(state, b"sec\0" as *const u8 as *const i8, (*stm).tm_sec, 0);
        setfield(
            state,
            b"yday\0" as *const u8 as *const i8,
            (*stm).tm_yday,
            1,
        );
        setfield(
            state,
            b"wday\0" as *const u8 as *const i8,
            (*stm).tm_wday,
            1,
        );
        setboolfield(
            state,
            b"isdst\0" as *const u8 as *const i8,
            0 != (*stm).tm_isdst,
        );
    }
}
pub unsafe extern "C" fn getboolfield(state: *mut State, key: *const i8) -> i32 {
    unsafe {
        let res: i32;
        res = if lua_getfield(state, -1, key) == 0 {
            -1
        } else {
            lua_toboolean(state, -1)
        };
        lua_settop(state, -2);
        return res;
    }
}
pub unsafe extern "C" fn getfield(state: *mut State, key: *const i8, d: i32, delta: i32) -> i32 {
    unsafe {
        let mut is_number: bool = false;
        let t: i32 = lua_getfield(state, -1, key);
        let mut res: i64 = lua_tointegerx(state, -1, &mut is_number);
        if !is_number {
            if ((t != 0) as i32 != 0) as i64 != 0 {
                return lual_error(
                    state,
                    b"field '%s' is not an integer\0" as *const u8 as *const i8,
                    key,
                );
            } else if ((d < 0) as i32 != 0) as i64 != 0 {
                return lual_error(
                    state,
                    b"field '%s' missing in date table\0" as *const u8 as *const i8,
                    key,
                );
            }
            res = d as i64;
        } else {
            if if res >= 0 {
                (res - delta as i64 <= 0x7FFFFFFF as i64) as i32
            } else {
                ((-(0x7FFFFFFF as i32) - 1 + delta) as i64 <= res) as i32
            } == 0
            {
                return lual_error(
                    state,
                    b"field '%s' is out-of-bound\0" as *const u8 as *const i8,
                    key,
                );
            }
            res -= delta as i64;
        }
        lua_settop(state, -2);
        return res as i32;
    }
}
pub unsafe extern "C" fn checkoption(
    state: *mut State,
    conv: *const i8,
    convlen: i64,
    buffer: *mut i8,
) -> *const i8 {
    unsafe {
        let mut option: *const i8 =
            b"aAbBcCdDeFgGhHIjmMnprRStTuUVwWxXyYzZ%||EcECExEXEyEYOdOeOHOIOmOMOSOuOUOVOwOWOy\0"
                as *const u8 as *const i8;
        let mut oplen: i32 = 1;
        while *option as i32 != '\0' as i32 && oplen as i64 <= convlen {
            if *option as i32 == '|' as i32 {
                oplen += 1;
            } else if memcmp(
                conv as *const libc::c_void,
                option as *const libc::c_void,
                oplen as u64,
            ) == 0
            {
                memcpy(
                    buffer as *mut libc::c_void,
                    conv as *const libc::c_void,
                    oplen as u64,
                );
                *buffer.offset(oplen as isize) = '\0' as i8;
                return conv.offset(oplen as isize);
            }
            option = option.offset(oplen as isize);
        }
        lual_argerror(
            state,
            1,
            lua_pushfstring(
                state,
                b"invalid conversion specifier '%%%s'\0" as *const u8 as *const i8,
                conv,
            ),
        );
        return conv;
    }
}
pub unsafe extern "C" fn l_checktime(state: *mut State, arg: i32) -> i64 {
    unsafe {
        let t: i64 = lual_checkinteger(state, arg);
        (((t as i64 == t) as i32 != 0) as i64 != 0
            || lual_argerror(
                state,
                arg,
                b"time out-of-bounds\0" as *const u8 as *const i8,
            ) != 0) as i32;
        return t as i64;
    }
}
pub unsafe extern "C" fn os_date(state: *mut State) -> i32 {
    unsafe {
        let mut slen: u64 = 0;
        let mut s: *const i8 =
            lual_optlstring(state, 1, b"%c\0" as *const u8 as *const i8, &mut slen);
        let mut t: i64 = if is_none_or_nil(lua_type(state, 2)) {
            time(std::ptr::null_mut())
        } else {
            l_checktime(state, 2)
        };
        let se: *const i8 = s.offset(slen as isize);
        let mut tmr: TM = TM {
            tm_sec: 0,
            tm_min: 0,
            tm_hour: 0,
            tm_mday: 0,
            tm_mon: 0,
            tm_year: 0,
            tm_wday: 0,
            tm_yday: 0,
            tm_isdst: 0,
            __tm_gmtoff: 0,
            __tm_zone: std::ptr::null(),
        };
        let stm: *mut TM;
        if *s as i32 == '!' as i32 {
            stm = gmtime_r(&mut t, &mut tmr);
            s = s.offset(1);
        } else {
            stm = localtime_r(&mut t, &mut tmr);
        }
        if stm.is_null() {
            return lual_error(
                state,
                b"date result cannot be represented in this installation\0" as *const u8
                    as *const i8,
            );
        }
        if strcmp(s, b"*t\0" as *const u8 as *const i8) == 0 {
            (*state).lua_createtable();
            setallfields(state, stm);
        } else {
            let mut cc: [i8; 4] = [0; 4];
            let mut b = Buffer::new();
            cc[0] = '%' as i8;
            b.lual_buffinit(state);
            while s < se {
                if *s as i32 != '%' as i32 {
                    (b.length < b.size || !(b.lual_prepbuffsize(1 as u64)).is_null()) as i32;
                    let fresh157 = s;
                    s = s.offset(1);
                    let fresh158 = b.length;
                    b.length = (b.length).wrapping_add(1);
                    *(b.pointer).offset(fresh158 as isize) = *fresh157;
                } else {
                    let reslen: u64;
                    let buffer: *mut i8 = b.lual_prepbuffsize(250 as u64);
                    s = s.offset(1);
                    s = checkoption(
                        state,
                        s,
                        se.offset_from(s) as i64,
                        cc.as_mut_ptr().offset(1 as isize),
                    );
                    reslen = strftime(buffer, 250 as u64, cc.as_mut_ptr(), stm);
                    b.length = (b.length as u64).wrapping_add(reslen) as u64;
                }
            }
            b.lual_pushresult();
        }
        return 1;
    }
}
pub unsafe extern "C" fn os_time(state: *mut State) -> i32 {
    unsafe {
        let t: i64;
        match lua_type(state, 1) {
            None | Some(TAG_TYPE_NIL) => {
                t = time(std::ptr::null_mut());
            },
            _ => {
                let mut ts: TM = TM {
                    tm_sec: 0,
                    tm_min: 0,
                    tm_hour: 0,
                    tm_mday: 0,
                    tm_mon: 0,
                    tm_year: 0,
                    tm_wday: 0,
                    tm_yday: 0,
                    tm_isdst: 0,
                    __tm_gmtoff: 0,
                    __tm_zone: std::ptr::null(),
                };
                lual_checktype(state, 1, TAG_TYPE_TABLE);
                lua_settop(state, 1);
                ts.tm_year = getfield(state, b"year\0" as *const u8 as *const i8, -1, 1900 as i32);
                ts.tm_mon = getfield(state, b"month\0" as *const u8 as *const i8, -1, 1);
                ts.tm_mday = getfield(state, b"day\0" as *const u8 as *const i8, -1, 0);
                ts.tm_hour = getfield(state, b"hour\0" as *const u8 as *const i8, 12 as i32, 0);
                ts.tm_min = getfield(state, b"min\0" as *const u8 as *const i8, 0, 0);
                ts.tm_sec = getfield(state, b"sec\0" as *const u8 as *const i8, 0, 0);
                ts.tm_isdst = getboolfield(state, b"isdst\0" as *const u8 as *const i8);
                t = mktime(&mut ts);
                setallfields(state, &mut ts);
            }
        };
        if t != t as i64 || t == -1 as i64 {
            return lual_error(
                state,
                b"time result cannot be represented in this installation\0" as *const u8
                    as *const i8,
            );
        }
        (*state).push_integer(t as i64);
        return 1;
    }
}
pub unsafe extern "C" fn os_difftime(state: *mut State) -> i32 {
    unsafe {
        let t1: i64 = l_checktime(state, 1);
        let t2: i64 = l_checktime(state, 2);
        (*state).push_number(difftime(t1, t2));
        return 1;
    }
}
pub unsafe extern "C" fn os_setlocale(state: *mut State) -> i32 {
    unsafe {
        pub const CATEGORY: [i32; 6] = [6, 3, 0, 4, 1, 2];
        pub const CATEGORY_NAMES: [*const i8; 7] = [
            b"all\0" as *const u8 as *const i8,
            b"collate\0" as *const u8 as *const i8,
            b"ctype\0" as *const u8 as *const i8,
            b"monetary\0" as *const u8 as *const i8,
            b"numeric\0" as *const u8 as *const i8,
            b"time\0" as *const u8 as *const i8,
            std::ptr::null(),
        ];
        let l: *const i8 = lual_optlstring(state, 1, std::ptr::null(), std::ptr::null_mut());
        let op: i32 = lual_checkoption(
            state,
            2,
            b"all\0" as *const u8 as *const i8,
            CATEGORY_NAMES.as_ptr(),
        );
        lua_pushstring(state, setlocale(CATEGORY[op as usize], l));
        return 1;
    }
}
pub unsafe extern "C" fn os_exit(state: *mut State) -> i32 {
    unsafe {
        let status: i32;
        if lua_type(state, 1) == Some(TAG_TYPE_BOOLEAN) {
            status = if lua_toboolean(state, 1) != 0 { 0 } else { 1 };
        } else {
            status = lual_optinteger(state, 1, 0) as i32;
        }
        if lua_toboolean(state, 2) != 0 {
            lua_close(state);
        }
        if !state.is_null() {
            exit(status);
        }
        return 0;
    }
}
pub const SYSTEM_FUNCTIONS: [RegisteredFunction; 12] = {
    [
        {
            RegisteredFunction {
                name: b"clock\0" as *const u8 as *const i8,
                function: Some(os_clock as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"date\0" as *const u8 as *const i8,
                function: Some(os_date as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"difftime\0" as *const u8 as *const i8,
                function: Some(os_difftime as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"execute\0" as *const u8 as *const i8,
                function: Some(os_execute as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"exit\0" as *const u8 as *const i8,
                function: Some(os_exit as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"getenv\0" as *const u8 as *const i8,
                function: Some(os_getenv as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"remove\0" as *const u8 as *const i8,
                function: Some(os_remove as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"rename\0" as *const u8 as *const i8,
                function: Some(os_rename as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"setlocale\0" as *const u8 as *const i8,
                function: Some(os_setlocale as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"time\0" as *const u8 as *const i8,
                function: Some(os_time as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"tmpname\0" as *const u8 as *const i8,
                function: Some(os_tmpname as unsafe extern "C" fn(*mut State) -> i32),
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
pub unsafe extern "C" fn luaopen_os(state: *mut State) -> i32 {
    unsafe {
        lual_checkversion_(
            state,
            504.0,
            (::core::mem::size_of::<i64>() as u64)
                .wrapping_mul(16 as u64)
                .wrapping_add(::core::mem::size_of::<f64>() as u64),
        );
        (*state).lua_createtable();
        lual_setfuncs(state, SYSTEM_FUNCTIONS.as_ptr(), 0);
        return 1;
    }
}
pub unsafe extern "C" fn str_len(state: *mut State) -> i32 {
    unsafe {
        let mut l: u64 = 0;
        lual_checklstring(state, 1, &mut l);
        (*state).push_integer(l as i64);
        return 1;
    }
}
pub unsafe extern "C" fn posrelati(pos: i64, length: u64) -> u64 {
    if pos > 0 {
        return pos as u64;
    } else if pos == 0 {
        return 1 as u64;
    } else if pos < -(length as i64) {
        return 1 as u64;
    } else {
        return length.wrapping_add(pos as u64).wrapping_add(1 as u64);
    };
}
pub unsafe extern "C" fn getendpos(state: *mut State, arg: i32, def: i64, length: u64) -> u64 {
    unsafe {
        let pos: i64 = lual_optinteger(state, arg, def);
        if pos > length as i64 {
            return length;
        } else if pos >= 0 {
            return pos as u64;
        } else if pos < -(length as i64) {
            return 0u64;
        } else {
            return length.wrapping_add(pos as u64).wrapping_add(1 as u64);
        };
    }
}
pub unsafe extern "C" fn str_sub(state: *mut State) -> i32 {
    unsafe {
        let mut l: u64 = 0;
        let s: *const i8 = lual_checklstring(state, 1, &mut l);
        let start: u64 = posrelati(lual_checkinteger(state, 2), l);
        let end: u64 = getendpos(state, 3, -1 as i64, l);
        if start <= end {
            lua_pushlstring(
                state,
                s.offset(start as isize).offset(-(1 as isize)),
                end.wrapping_sub(start).wrapping_add(1 as u64),
            );
        } else {
            lua_pushstring(state, b"\0" as *const u8 as *const i8);
        }
        return 1;
    }
}
pub unsafe extern "C" fn str_reverse(state: *mut State) -> i32 {
    unsafe {
        let mut l: u64 = 0;
        let mut i: u64;
        let mut b = Buffer::new();
        let s: *const i8 = lual_checklstring(state, 1, &mut l);
        let p: *mut i8 = b.lual_buffinitsize(state, l);
        i = 0;
        while i < l {
            *p.offset(i as isize) = *s.offset(l.wrapping_sub(i).wrapping_sub(1 as u64) as isize);
            i = i.wrapping_add(1);
        }
        b.lual_pushresultsize(l);
        return 1;
    }
}
pub unsafe extern "C" fn str_lower(state: *mut State) -> i32 {
    unsafe {
        let mut l: u64 = 0;
        let mut i: u64;
        let mut b = Buffer::new();
        let s: *const i8 = lual_checklstring(state, 1, &mut l);
        let p: *mut i8 = b.lual_buffinitsize(state, l);
        i = 0;
        while i < l {
            *p.offset(i as isize) = tolower(*s.offset(i as isize) as u8 as i32) as i8;
            i = i.wrapping_add(1);
        }
        b.lual_pushresultsize(l);
        return 1;
    }
}
pub unsafe extern "C" fn str_upper(state: *mut State) -> i32 {
    unsafe {
        let mut l: u64 = 0;
        let mut i: u64;
        let mut b = Buffer::new();
        let s: *const i8 = lual_checklstring(state, 1, &mut l);
        let p: *mut i8 = b.lual_buffinitsize(state, l);
        i = 0;
        while i < l {
            *p.offset(i as isize) = toupper(*s.offset(i as isize) as u8 as i32) as i8;
            i = i.wrapping_add(1);
        }
        b.lual_pushresultsize(l);
        return 1;
    }
}
pub unsafe extern "C" fn str_rep(state: *mut State) -> i32 {
    unsafe {
        let mut l: u64 = 0;
        let mut lsep: u64 = 0;
        let s: *const i8 = lual_checklstring(state, 1, &mut l);
        let mut n: i64 = lual_checkinteger(state, 2);
        let sep: *const i8 = lual_optlstring(state, 3, b"\0" as *const u8 as *const i8, &mut lsep);
        if n <= 0 {
        lua_pushstring(state, b"\0" as *const u8 as *const i8);
    } else if ((l.wrapping_add(lsep) < l
        || l.wrapping_add(lsep) as u64
            > ((if (::core::mem::size_of::<u64>() as u64) < ::core::mem::size_of::<i32>() as u64 {
                !(0u64)
            } else {
                0x7FFFFFFF as u64
            }) as u64)
                .wrapping_div(n as u64)) as i32
        != 0) as i64
        != 0
    {
        return lual_error(
            state,
            b"resulting string too large\0" as *const u8 as *const i8,
        );
    } else {
        let totallen: u64 = (n as u64)
            .wrapping_mul(l)
            .wrapping_add(((n - 1) as u64).wrapping_mul(lsep));
        let mut b = Buffer::new();
        let mut p: *mut i8 = b.lual_buffinitsize(state, totallen);
        loop {
            let fresh159 = n;
            n = n - 1;
            if !(fresh159 > 1) {
                break;
            }
            memcpy(
                p as *mut libc::c_void,
                s as *const libc::c_void,
                l.wrapping_mul(::core::mem::size_of::<i8>() as u64),
            );
            p = p.offset(l as isize);
            if lsep > 0u64 {
                memcpy(
                    p as *mut libc::c_void,
                    sep as *const libc::c_void,
                    lsep.wrapping_mul(::core::mem::size_of::<i8>() as u64),
                );
                p = p.offset(lsep as isize);
            }
        }
        memcpy(
            p as *mut libc::c_void,
            s as *const libc::c_void,
            l.wrapping_mul(::core::mem::size_of::<i8>() as u64),
        );
        b.lual_pushresultsize(totallen);
    }
        return 1;
    }
}
pub unsafe extern "C" fn str_byte(state: *mut State) -> i32 {
    unsafe {
        let mut l: u64 = 0;
        let s: *const i8 = lual_checklstring(state, 1, &mut l);
        let pi: i64 = lual_optinteger(state, 2, 1);
        let posi: u64 = posrelati(pi, l);
        let pose: u64 = getendpos(state, 3, pi, l);
        let n: i32;
        let mut i: i32;
        if posi > pose {
            return 0;
        }
        if ((pose.wrapping_sub(posi) >= 0x7FFFFFFF as u64) as i32 != 0) as i64 != 0 {
            return lual_error(state, b"string slice too long\0" as *const u8 as *const i8);
        }
        n = pose.wrapping_sub(posi) as i32 + 1;
        lual_checkstack(
            state,
            n,
            b"string slice too long\0" as *const u8 as *const i8,
        );
        i = 0;
        while i < n {
            (*state).push_integer(
                *s.offset(posi.wrapping_add(i as u64).wrapping_sub(1 as u64) as isize) as u8 as i64,
            );
            i += 1;
        }
        return n;
    }
}
pub unsafe extern "C" fn str_char(state: *mut State) -> i32 {
    unsafe {
        let n: i32 = (*state).get_top();
        let mut i: i32;
        let mut b = Buffer::new();
        let p: *mut i8 = b.lual_buffinitsize(state, n as u64);
        i = 1;
        while i <= n {
            let c: u64 = lual_checkinteger(state, i) as u64;
            (((c <= (127 as i32 * 2 + 1) as u64) as i32 != 0) as i64 != 0
                || lual_argerror(state, i, b"value out of range\0" as *const u8 as *const i8) != 0)
                as i32;
            *p.offset((i - 1) as isize) = c as u8 as i8;
            i += 1;
        }
        b.lual_pushresultsize(n as u64);
        return 1;
    }
}
pub unsafe extern "C" fn writer(
    state: *mut State,
    b: *const libc::c_void,
    size: u64,
    arbitrary_data: *mut libc::c_void,
) -> i32 {
    unsafe {
        let stream_writer: *mut StreamWriter = arbitrary_data as *mut StreamWriter;
        if (*stream_writer).init == 0 {
            (*stream_writer).init = 1;
            (*stream_writer).buffer.lual_buffinit(state);
        }
        (*stream_writer)
            .buffer
            .lual_addlstring(b as *const i8, size);
        return 0;
    }
}
pub unsafe extern "C" fn str_dump(state: *mut State) -> i32 {
    unsafe {
        let mut stream_writer: StreamWriter = StreamWriter {
            init: 0,
            buffer: Buffer::new(),
        };
        let is_strip = 0 != lua_toboolean(state, 2);
        lual_checktype(state, 1, TAG_TYPE_CLOSURE);
        lua_settop(state, 1);
        stream_writer.init = 0;
        if ((lua_dump(
            state,
            Some(
                writer
                    as unsafe extern "C" fn(
                        *mut State,
                        *const libc::c_void,
                        u64,
                        *mut libc::c_void,
                    ) -> i32,
            ),
            &mut stream_writer as *mut StreamWriter as *mut libc::c_void,
            is_strip,
        ) != 0) as i32
            != 0) as i64
            != 0
        {
            return lual_error(
                state,
                b"unable to dump given function\0" as *const u8 as *const i8,
            );
        }
        stream_writer.buffer.lual_pushresult();
        return 1;
    }
}
pub unsafe extern "C" fn tonum(state: *mut State, arg: i32) -> i32 {
    unsafe {
        if lua_type(state, arg) == Some(TAG_TYPE_NUMERIC) {
            lua_pushvalue(state, arg);
            return 1;
        } else {
            let mut length: u64 = 0;
            let s: *const i8 = lua_tolstring(state, arg, &mut length);
            return (!s.is_null() && lua_stringtonumber(state, s) == length.wrapping_add(1 as u64))
                as i32;
        };
    }
}
pub unsafe extern "C" fn trymt(state: *mut State, mtname: *const i8) {
    unsafe {
        lua_settop(state, 2);
        if ((lua_type(state, 2) == Some(TAG_TYPE_STRING) || lual_getmetafield(state, 2, mtname) == 0) as i32 != 0)
            as i64
            != 0
        {
            lual_error(
                state,
                b"attempt to %s a '%s' with a '%s'\0" as *const u8 as *const i8,
                mtname.offset(2 as isize),
                lua_typename(state, lua_type(state, -2)),
                lua_typename(state, lua_type(state, -1)),
            );
        }
        lua_rotate(state, -3, 1);
        lua_callk(state, 2, 1, 0, None);
    }
}
pub unsafe extern "C" fn arith(state: *mut State, op: i32, mtname: *const i8) -> i32 {
    unsafe {
        if tonum(state, 1) != 0 && tonum(state, 2) != 0 {
            lua_arith(state, op);
        } else {
            trymt(state, mtname);
        }
        return 1;
    }
}
pub unsafe extern "C" fn arith_add(state: *mut State) -> i32 {
    unsafe {
        return arith(state, 0, b"__add\0" as *const u8 as *const i8);
    }
}
pub unsafe extern "C" fn arith_sub(state: *mut State) -> i32 {
    unsafe {
        return arith(state, 1, b"__sub\0" as *const u8 as *const i8);
    }
}
pub unsafe extern "C" fn arith_mul(state: *mut State) -> i32 {
    unsafe {
        return arith(state, 2, b"__mul\0" as *const u8 as *const i8);
    }
}
pub unsafe extern "C" fn arith_mod(state: *mut State) -> i32 {
    unsafe {
        return arith(state, 3, b"__mod\0" as *const u8 as *const i8);
    }
}
pub unsafe extern "C" fn arith_pow(state: *mut State) -> i32 {
    unsafe {
        return arith(state, 4, b"__pow\0" as *const u8 as *const i8);
    }
}
pub unsafe extern "C" fn arith_div(state: *mut State) -> i32 {
    unsafe {
        return arith(state, 5, b"__div\0" as *const u8 as *const i8);
    }
}
pub unsafe extern "C" fn arith_idiv(state: *mut State) -> i32 {
    unsafe {
        return arith(state, 6, b"__idiv\0" as *const u8 as *const i8);
    }
}
pub unsafe extern "C" fn arith_unm(state: *mut State) -> i32 {
    unsafe {
        return arith(state, 12 as i32, b"__unm\0" as *const u8 as *const i8);
    }
}
pub const STRING_METAMETHODS: [RegisteredFunction; 10] = {
    [
        {
            RegisteredFunction {
                name: b"__add\0" as *const u8 as *const i8,
                function: Some(arith_add as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"__sub\0" as *const u8 as *const i8,
                function: Some(arith_sub as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"__mul\0" as *const u8 as *const i8,
                function: Some(arith_mul as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"__mod\0" as *const u8 as *const i8,
                function: Some(arith_mod as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"__pow\0" as *const u8 as *const i8,
                function: Some(arith_pow as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"__div\0" as *const u8 as *const i8,
                function: Some(arith_div as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"__idiv\0" as *const u8 as *const i8,
                function: Some(arith_idiv as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"__unm\0" as *const u8 as *const i8,
                function: Some(arith_unm as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"__index\0" as *const u8 as *const i8,
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
pub unsafe extern "C" fn match_class(c: i32, cl: i32) -> i32 {
    unsafe {
        let res: i32;
        match tolower(cl) {
            97 => {
                res = *(*__ctype_b_loc()).offset(c as isize) as i32 & _ISALPHA as i32;
            }
            99 => {
                res =
                    *(*__ctype_b_loc()).offset(c as isize) as i32 & _ISCONTROL as i32;
            }
            100 => {
                res = *(*__ctype_b_loc()).offset(c as isize) as i32 & _ISDIGIT as i32;
            }
            103 => {
                res = *(*__ctype_b_loc()).offset(c as isize) as i32 & _ISGRAPH as i32;
            }
            108 => {
                res = *(*__ctype_b_loc()).offset(c as isize) as i32 & _ISLOWER as i32;
            }
            112 => {
                res = *(*__ctype_b_loc()).offset(c as isize) as i32
                    & _ISPUNCTUATION as i32;
            }
            115 => {
                res = *(*__ctype_b_loc()).offset(c as isize) as i32 & _ISSPACE as i32;
            }
            117 => {
                res = *(*__ctype_b_loc()).offset(c as isize) as i32 & _ISUPPER as i32;
            }
            119 => {
                res = *(*__ctype_b_loc()).offset(c as isize) as i32
                    & _ISALPHANUMERIC as i32;
            }
            120 => {
                res =
                    *(*__ctype_b_loc()).offset(c as isize) as i32 & _ISXDIGIT as i32;
            }
            122 => {
                res = (c == 0) as i32;
            }
            _ => return (cl == c) as i32,
        }
        return if *(*__ctype_b_loc()).offset(cl as isize) as i32 & _ISLOWER as i32
            != 0
        {
            res
        } else {
            (res == 0) as i32
        };
    }
}
pub unsafe extern "C" fn matchbracketclass(c: i32, mut p: *const i8, ec: *const i8) -> i32 {
    unsafe {
        let mut sig: i32 = 1;
        if *p.offset(1 as isize) as i32 == '^' as i32 {
            sig = 0;
            p = p.offset(1);
        }
        loop {
            p = p.offset(1);
            if !(p < ec) {
                break;
            }
            if *p as i32 == '%' as i32 {
                p = p.offset(1);
                if match_class(c, *p as u8 as i32) != 0 {
                    return sig;
                }
            } else if *p.offset(1 as isize) as i32 == '-' as i32 && p.offset(2 as isize) < ec {
                p = p.offset(2 as isize);
                if *p.offset(-(2 as isize)) as u8 as i32 <= c && c <= *p as u8 as i32 {
                    return sig;
                }
            } else if *p as u8 as i32 == c {
                return sig;
            }
        }
        return (sig == 0) as i32;
    }
}
pub unsafe extern "C" fn lmemfind(
    mut s1: *const i8,
    mut l1: u64,
    s2: *const i8,
    mut l2: u64,
) -> *const i8 {
    unsafe {
        if l2 == 0u64 {
            return s1;
        } else if l2 > l1 {
            return std::ptr::null();
        } else {
            let mut init: *const i8 = std::ptr::null();
            l2 = l2.wrapping_sub(1);
            l1 = l1.wrapping_sub(l2);
            while l1 > 0u64 && {
                init = memchr(s1 as *const libc::c_void, *s2 as i32, l1) as *const i8;
                !init.is_null()
            } {
                init = init.offset(1);
                if memcmp(
                    init as *const libc::c_void,
                    s2.offset(1 as isize) as *const libc::c_void,
                    l2,
                ) == 0
                {
                    return init.offset(-(1 as isize));
                } else {
                    l1 = (l1 as u64).wrapping_sub(init.offset_from(s1) as u64) as u64;
                    s1 = init;
                }
            }
            return std::ptr::null();
        };
    }
}
pub unsafe extern "C" fn nospecials(p: *const i8, l: u64) -> i32 {
    unsafe {
        let mut upto: u64 = 0;
        loop {
            if !(strpbrk(
                p.offset(upto as isize),
                b"^$*+?.([%-\0" as *const u8 as *const i8,
            ))
            .is_null()
            {
                return 0;
            }
            upto = (upto as u64)
                .wrapping_add((strlen(p.offset(upto as isize))).wrapping_add(1 as u64))
                as u64;
            if !(upto <= l) {
                break;
            }
        }
        return 1;
    }
}
pub unsafe extern "C" fn str_find_aux(state: *mut State, find: i32) -> i32 {
    unsafe {
        let mut lexical_state: u64 = 0;
        let mut lp: u64 = 0;
        let s: *const i8 = lual_checklstring(state, 1, &mut lexical_state);
        let mut p: *const i8 = lual_checklstring(state, 2, &mut lp);
        let init: u64 =
            (posrelati(lual_optinteger(state, 3, 1 as i64), lexical_state)).wrapping_sub(1 as u64);
        if init > lexical_state {
            (*state).push_nil();
            return 1;
        }
        if find != 0 && (lua_toboolean(state, 4) != 0 || nospecials(p, lp) != 0) {
            let s2: *const i8 = lmemfind(
                s.offset(init as isize),
                lexical_state.wrapping_sub(init),
                p,
                lp,
            );
            if !s2.is_null() {
                (*state).push_integer((s2.offset_from(s) as i64 + 1) as i64);
                (*state).push_integer((s2.offset_from(s) as u64).wrapping_add(lp) as i64);
                return 2;
            }
        } else {
            let mut match_state: MatchState = MatchState {
                src_init: std::ptr::null(),
                src_end: std::ptr::null(),
                p_end: std::ptr::null(),
                state: std::ptr::null_mut(),
                matchdepth: 0,
                level: 0,
                capture: [MatchStateCapture {
                    init: std::ptr::null(),
                    length: 0,
                }; 32],
            };
            let mut s1: *const i8 = s.offset(init as isize);
            let anchor: i32 = (*p as i32 == '^' as i32) as i32;
            if anchor != 0 {
                p = p.offset(1);
                lp = lp.wrapping_sub(1);
            }
            match_state.prepstate(state, s, lexical_state, p, lp);
            loop {
                let res: *const i8;
                match_state.reprepstate();
                res = match_state.match_0(s1, p);
                if !res.is_null() {
                    if find != 0 {
                        (*state).push_integer((s1.offset_from(s) as i64 + 1) as i64);
                        (*state).push_integer(res.offset_from(s) as i64);
                        return match_state.push_captures(std::ptr::null(), std::ptr::null()) + 2;
                    } else {
                        return match_state.push_captures(s1, res);
                    }
                }
                let fresh163 = s1;
                s1 = s1.offset(1);
                if !(fresh163 < match_state.src_end && anchor == 0) {
                    break;
                }
            }
        }
        (*state).push_nil();
        return 1;
    }
}
pub unsafe extern "C" fn str_find(state: *mut State) -> i32 {
    unsafe {
        return str_find_aux(state, 1);
    }
}
pub unsafe extern "C" fn str_match(state: *mut State) -> i32 {
    unsafe {
        return str_find_aux(state, 0);
    }
}
pub unsafe extern "C" fn str_gsub(state: *mut State) -> i32 {
    unsafe {
        let mut srcl: u64 = 0;
        let mut lp: u64 = 0;
        let mut src: *const i8 = lual_checklstring(state, 1, &mut srcl);
        let mut p: *const i8 = lual_checklstring(state, 2, &mut lp);
        let mut lastmatch: *const i8 = std::ptr::null();
        let tr = lua_type(state, 3);
        let max_s: i64 = lual_optinteger(state, 4, srcl.wrapping_add(1 as u64) as i64);
        let anchor: i32 = (*p as i32 == '^' as i32) as i32;
        let mut n: i64 = 0;
        let mut changed: i32 = 0;
        let mut match_state: MatchState = MatchState {
            src_init: std::ptr::null(),
            src_end: std::ptr::null(),
            p_end: std::ptr::null(),
            state: std::ptr::null_mut(),
            matchdepth: 0,
            level: 0,
            capture: [MatchStateCapture {
                init: std::ptr::null(),
                length: 0,
            }; 32],
        };
        let mut b = Buffer::new();
        (((tr == Some(TAG_TYPE_NUMERIC) || tr == Some(TAG_TYPE_STRING) || tr == Some(TAG_TYPE_CLOSURE) || tr == Some(TAG_TYPE_TABLE)) as i32 != 0) as i64 != 0
            || lual_typeerror(
                state,
                3,
                b"string/function/table\0" as *const u8 as *const i8,
            ) != 0) as i32;
        b.lual_buffinit(state);
        if anchor != 0 {
            p = p.offset(1);
            lp = lp.wrapping_sub(1);
        }
        match_state.prepstate(state, src, srcl, p, lp);
        while n < max_s {
            let e: *const i8;
            match_state.reprepstate();
            e = match_state.match_0(src, p);
            if !e.is_null() && e != lastmatch {
                n += 1;
                changed = match_state.add_value(&mut b, src, e, tr.unwrap()) | changed;
                lastmatch = e;
                src = lastmatch;
            } else {
                if !(src < match_state.src_end) {
                    break;
                }
                (b.length < b.size || !(b.lual_prepbuffsize(1 as u64)).is_null()) as i32;
                let fresh165 = src;
                src = src.offset(1);
                let fresh166 = b.length;
                b.length = (b.length).wrapping_add(1);
                *(b.pointer).offset(fresh166 as isize) = *fresh165;
            }
            if anchor != 0 {
                break;
            }
        }
        if changed == 0 {
            lua_pushvalue(state, 1);
        } else {
            b.lual_addlstring(src, (match_state.src_end).offset_from(src) as u64);
            b.lual_pushresult();
        }
        (*state).push_integer(n);
        return 2;
    }
}
pub unsafe extern "C" fn addquoted(b: *mut Buffer, mut s: *const i8, mut length: u64) {
    unsafe {
        ((*b).length < (*b).size || !((*b).lual_prepbuffsize(1 as u64)).is_null()) as i32;
        let fresh167 = (*b).length;
        (*b).length = ((*b).length).wrapping_add(1);
        *((*b).pointer).offset(fresh167 as isize) = '"' as i8;
        loop {
            let fresh168 = length;
            length = length.wrapping_sub(1);
            if !(fresh168 != 0) {
                break;
            }
            if *s as i32 == '"' as i32 || *s as i32 == '\\' as i32 || *s as i32 == '\n' as i32 {
                ((*b).length < (*b).size || !((*b).lual_prepbuffsize(1 as u64)).is_null()) as i32;
                let fresh169 = (*b).length;
                (*b).length = ((*b).length).wrapping_add(1);
                *((*b).pointer).offset(fresh169 as isize) = '\\' as i8;
                ((*b).length < (*b).size || !((*b).lual_prepbuffsize(1 as u64)).is_null()) as i32;
                let fresh170 = (*b).length;
                (*b).length = ((*b).length).wrapping_add(1);
                *((*b).pointer).offset(fresh170 as isize) = *s;
            } else if *(*__ctype_b_loc()).offset(*s as u8 as isize) as i32
                & _ISCONTROL as i32
                != 0
            {
                let mut buffer: [i8; 10] = [0; 10];
                if *(*__ctype_b_loc()).offset(*s.offset(1 as isize) as u8 as isize) as i32
                    & _ISDIGIT as i32
                    == 0
                {
                    snprintf(
                        buffer.as_mut_ptr(),
                        ::core::mem::size_of::<[i8; 10]>() as u64,
                        b"\\%d\0" as *const u8 as *const i8,
                        *s as u8 as i32,
                    );
                } else {
                    snprintf(
                        buffer.as_mut_ptr(),
                        ::core::mem::size_of::<[i8; 10]>() as u64,
                        b"\\%03d\0" as *const u8 as *const i8,
                        *s as u8 as i32,
                    );
                }
                (*b).lual_addstring(buffer.as_mut_ptr());
            } else {
                ((*b).length < (*b).size || !((*b).lual_prepbuffsize(1 as u64)).is_null()) as i32;
                let fresh171 = (*b).length;
                (*b).length = ((*b).length).wrapping_add(1);
                *((*b).pointer).offset(fresh171 as isize) = *s;
            }
            s = s.offset(1);
        }
        ((*b).length < (*b).size || !((*b).lual_prepbuffsize(1 as u64)).is_null()) as i32;
        let fresh172 = (*b).length;
        (*b).length = ((*b).length).wrapping_add(1);
        *((*b).pointer).offset(fresh172 as isize) = '"' as i8;
    }
}
pub unsafe extern "C" fn quotefloat(mut _state: *mut State, buffer: *mut i8, n: f64) -> i32 {
    unsafe {
        let s: *const i8;
        if n == ::core::f64::INFINITY {
            s = b"1e9999\0" as *const u8 as *const i8;
        } else if n == -::core::f64::INFINITY {
            s = b"-1e9999\0" as *const u8 as *const i8;
        } else if n != n {
            s = b"(0/0)\0" as *const u8 as *const i8;
        } else {
            let nb: i32 = snprintf(
                buffer,
                120 as u64,
                b"%a\0" as *const u8 as *const i8,
                n,
            );
            if (memchr(buffer as *const libc::c_void, '.' as i32, nb as u64)).is_null() {
                let point: i8 = '.' as i8;
                let ppoint: *mut i8 =
                    memchr(buffer as *const libc::c_void, point as i32, nb as u64) as *mut i8;
                if !ppoint.is_null() {
                    *ppoint = '.' as i8;
                }
            }
            return nb;
        }
        return snprintf(
            buffer,
            120 as u64,
            b"%s\0" as *const u8 as *const i8,
            s,
        );
    }
}
pub unsafe extern "C" fn addliteral(state: *mut State, b: *mut Buffer, arg: i32) {
    unsafe {
        match lua_type(state, arg) {
            Some(TAG_TYPE_STRING) => {
                let mut length: u64 = 0;
                let s: *const i8 = lua_tolstring(state, arg, &mut length);
                addquoted(b, s, length);
            },
            Some(TAG_TYPE_NUMERIC) => {
                let buffer: *mut i8 = (*b).lual_prepbuffsize(120 as u64);
                let nb: i32;
                if lua_isinteger(state, arg) {
                    let n: i64 = lua_tointegerx(state, arg, std::ptr::null_mut());
                    let format: *const i8 = if n == -(0x7FFFFFFFFFFFFFFF as i64) - 1 as i64 {
                        b"0x%llx\0" as *const u8 as *const i8
                    } else {
                        b"%lld\0" as *const u8 as *const i8
                    };
                    nb = snprintf(buffer, 120 as u64, format, n);
                } else {
                    nb = quotefloat(
                        state,
                        buffer,
                        lua_tonumberx(state, arg, std::ptr::null_mut()),
                    );
                }
                (*b).length = ((*b).length as u64).wrapping_add(nb as u64) as u64;
            },
            Some(TAG_TYPE_NIL) | Some(TAG_TYPE_BOOLEAN) => {
                lual_tolstring(state, arg, std::ptr::null_mut());
                (*b).lual_addvalue();
            },
            _ => {
                lual_argerror(
                    state,
                    arg,
                    b"value has no literal form\0" as *const u8 as *const i8,
                );
            }
        };
    }
}
pub unsafe extern "C" fn get2digits(mut s: *const i8) -> *const i8 {
    unsafe {
        if *(*__ctype_b_loc()).offset(*s as u8 as isize) as i32
            & _ISDIGIT as i32
            != 0
        {
            s = s.offset(1);
            if *(*__ctype_b_loc()).offset(*s as u8 as isize) as i32
                & _ISDIGIT as i32
                != 0
            {
                s = s.offset(1);
            }
        }
        return s;
    }
}
pub unsafe extern "C" fn checkformat(
    state: *mut State,
    form: *const i8,
    flags: *const i8,
    precision: i32,
) {
    unsafe {
        let mut spec: *const i8 = form.offset(1 as isize);
        spec = spec.offset(strspn(spec, flags) as isize);
        if *spec as i32 != '0' as i32 {
            spec = get2digits(spec);
            if *spec as i32 == '.' as i32 && precision != 0 {
                spec = spec.offset(1);
                spec = get2digits(spec);
            }
        }
        if *(*__ctype_b_loc()).offset(*spec as u8 as isize) as i32
            & _ISALPHA as i32
            == 0
        {
            lual_error(
                state,
                b"invalid conversion specification: '%s'\0" as *const u8 as *const i8,
                form,
            );
        }
    }
}
pub unsafe extern "C" fn getformat(
    state: *mut State,
    strfrmt: *const i8,
    mut form: *mut i8,
) -> *const i8 {
    unsafe {
        let mut length: u64 = strspn(strfrmt, b"-+#0 123456789.\0" as *const u8 as *const i8);
        length = length.wrapping_add(1);
        if length >= (32 as i32 - 10 as i32) as u64 {
            lual_error(
                state,
                b"invalid format (too long)\0" as *const u8 as *const i8,
            );
        }
        let fresh173 = form;
        form = form.offset(1);
        *fresh173 = '%' as i8;
        memcpy(
            form as *mut libc::c_void,
            strfrmt as *const libc::c_void,
            length.wrapping_mul(::core::mem::size_of::<i8>() as u64),
        );
        *form.offset(length as isize) = '\0' as i8;
        return strfrmt.offset(length as isize).offset(-(1 as isize));
    }
}
pub unsafe extern "C" fn addlenmod(form: *mut i8, lenmod: *const i8) {
    unsafe {
        let l: u64 = strlen(form);
        let lm: u64 = strlen(lenmod);
        let spec: i8 = *form.offset(l.wrapping_sub(1 as u64) as isize);
        strcpy(form.offset(l as isize).offset(-(1 as isize)), lenmod);
        *form.offset(l.wrapping_add(lm).wrapping_sub(1 as u64) as isize) = spec;
        *form.offset(l.wrapping_add(lm) as isize) = '\0' as i8;
    }
}
pub unsafe extern "C" fn str_format(state: *mut State) -> i32 {
    unsafe {
        let mut current_block: u64;
        let top: i32 = (*state).get_top();
        let mut arg: i32 = 1;
        let mut sfl: u64 = 0;
        let mut strfrmt: *const i8 = lual_checklstring(state, arg, &mut sfl);
        let strfrmt_end: *const i8 = strfrmt.offset(sfl as isize);
        let mut flags: *const i8 = std::ptr::null();
        let mut b = Buffer::new();
        b.lual_buffinit(state);
        while strfrmt < strfrmt_end {
            if *strfrmt as i32 != '%' as i32 {
                (b.length < b.size || !(b.lual_prepbuffsize(1 as u64)).is_null()) as i32;
                let fresh174 = strfrmt;
                strfrmt = strfrmt.offset(1);
                let fresh175 = b.length;
                b.length = (b.length).wrapping_add(1);
                *(b.pointer).offset(fresh175 as isize) = *fresh174;
            } else {
                strfrmt = strfrmt.offset(1);
                if *strfrmt as i32 == '%' as i32 {
                    (b.length < b.size || !(b.lual_prepbuffsize(1 as u64)).is_null()) as i32;
                    let fresh176 = strfrmt;
                    strfrmt = strfrmt.offset(1);
                    let fresh177 = b.length;
                    b.length = (b.length).wrapping_add(1);
                    *(b.pointer).offset(fresh177 as isize) = *fresh176;
                } else {
                    let mut form: [i8; 32] = [0; 32];
                    let mut maxitem: i32 = 120 as i32;
                    let mut buffer: *mut i8 = b.lual_prepbuffsize(maxitem as u64);
                    let mut nb: i32 = 0;
                    arg += 1;
                    if arg > top {
                        return lual_argerror(state, arg, b"no value\0" as *const u8 as *const i8);
                    }
                    strfrmt = getformat(state, strfrmt, form.as_mut_ptr());
                    let fresh178 = strfrmt;
                    strfrmt = strfrmt.offset(1);
                    match *fresh178 as i32 {
                        99 => {
                            checkformat(
                                state,
                                form.as_mut_ptr(),
                                b"-\0" as *const u8 as *const i8,
                                0,
                            );
                            nb = snprintf(
                                buffer,
                                maxitem as u64,
                                form.as_mut_ptr(),
                                lual_checkinteger(state, arg) as i32,
                            );
                            current_block = 11793792312832361944;
                        }
                        100 | 105 => {
                            flags = b"-+0 \0" as *const u8 as *const i8;
                            current_block = 5689001924483802034;
                        }
                        117 => {
                            flags = b"-0\0" as *const u8 as *const i8;
                            current_block = 5689001924483802034;
                        }
                        111 | 120 | 88 => {
                            flags = b"-#0\0" as *const u8 as *const i8;
                            current_block = 5689001924483802034;
                        }
                        97 | 65 => {
                            checkformat(
                                state,
                                form.as_mut_ptr(),
                                b"-+#0 \0" as *const u8 as *const i8,
                                1,
                            );
                            addlenmod(form.as_mut_ptr(), b"\0" as *const u8 as *const i8);
                            nb = snprintf(
                                buffer,
                                maxitem as u64,
                                form.as_mut_ptr(),
                                lual_checknumber(state, arg),
                            );
                            current_block = 11793792312832361944;
                        }
                        102 => {
                            maxitem = 110 as i32 + 308 as i32;
                            buffer = b.lual_prepbuffsize(maxitem as u64);
                            current_block = 6669252993407410313;
                        }
                        101 | 69 | 103 | 71 => {
                            current_block = 6669252993407410313;
                        }
                        112 => {
                            let mut p: *const libc::c_void = User::lua_topointer(state, arg);
                            checkformat(
                                state,
                                form.as_mut_ptr(),
                                b"-\0" as *const u8 as *const i8,
                                0,
                            );
                            if p.is_null() {
                                p = b"(null)\0" as *const u8 as *const i8 as *const libc::c_void;
                                form[(strlen(form.as_mut_ptr())).wrapping_sub(1 as u64) as usize] =
                                    's' as i8;
                            }
                            nb = snprintf(buffer, maxitem as u64, form.as_mut_ptr(), p);
                            current_block = 11793792312832361944;
                        }
                        113 => {
                            if form[2 as usize] as i32 != '\0' as i32 {
                                return lual_error(
                                    state,
                                    b"specifier '%%q' cannot have modifiers\0" as *const u8
                                        as *const i8,
                                );
                            }
                            addliteral(state, &mut b, arg);
                            current_block = 11793792312832361944;
                        }
                        115 => {
                            let mut l: u64 = 0;
                            let s: *const i8 = lual_tolstring(state, arg, &mut l);
                            if form[2 as usize] as i32 == '\0' as i32 {
                                b.lual_addvalue();
                            } else {
                                (((l == strlen(s)) as i32 != 0) as i64 != 0
                                    || lual_argerror(
                                        state,
                                        arg,
                                        b"string contains zeros\0" as *const u8 as *const i8,
                                    ) != 0) as i32;
                                checkformat(
                                    state,
                                    form.as_mut_ptr(),
                                    b"-\0" as *const u8 as *const i8,
                                    1,
                                );
                                if (strchr(form.as_mut_ptr(), '.' as i32)).is_null()
                                    && l >= 100 as u64
                                {
                                    b.lual_addvalue();
                                } else {
                                    nb = snprintf(buffer, maxitem as u64, form.as_mut_ptr(), s);
                                    lua_settop(state, -2);
                                }
                            }
                            current_block = 11793792312832361944;
                        }
                        _ => {
                            return lual_error(
                                state,
                                b"invalid conversion '%s' to 'format'\0" as *const u8 as *const i8,
                                form.as_mut_ptr(),
                            );
                        }
                    }
                    match current_block {
                        5689001924483802034 => {
                            let n: i64 = lual_checkinteger(state, arg);
                            checkformat(state, form.as_mut_ptr(), flags, 1);
                            addlenmod(form.as_mut_ptr(), b"ll\0" as *const u8 as *const i8);
                            nb = snprintf(buffer, maxitem as u64, form.as_mut_ptr(), n);
                        }
                        6669252993407410313 => {
                            let n_0: f64 = lual_checknumber(state, arg);
                            checkformat(
                                state,
                                form.as_mut_ptr(),
                                b"-+#0 \0" as *const u8 as *const i8,
                                1,
                            );
                            addlenmod(form.as_mut_ptr(), b"\0" as *const u8 as *const i8);
                            nb = snprintf(buffer, maxitem as u64, form.as_mut_ptr(), n_0);
                        }
                        _ => {}
                    }
                    b.length = (b.length as u64).wrapping_add(nb as u64) as u64;
                }
            }
        }
        b.lual_pushresult();
        return 1;
    }
}
pub const NATIVE_ENDIAN: NativeEndian = NativeEndian { dummy: 1 };
pub unsafe extern "C" fn digit(c: i32) -> i32 {
    return ('0' as i32 <= c && c <= '9' as i32) as i32;
}
pub unsafe extern "C" fn getnum(fmt: *mut *const i8, df: i32) -> i32 {
    unsafe {
        if digit(**fmt as i32) == 0 {
            return df;
        } else {
            let mut a: i32 = 0;
            loop {
                let fresh179 = *fmt;
                *fmt = (*fmt).offset(1);
                a = a * 10 as i32 + (*fresh179 as i32 - '0' as i32);
                if !(digit(**fmt as i32) != 0
                    && a <= ((if (::core::mem::size_of::<u64>() as u64)
                        < ::core::mem::size_of::<i32>() as u64
                    {
                        !(0u64)
                    } else {
                        0x7FFFFFFF as u64
                    }) as i32
                        - 9 as i32)
                        / 10 as i32)
                {
                    break;
                }
            }
            return a;
        };
    }
}
pub unsafe extern "C" fn getnumlimit(h: *mut Header, fmt: *mut *const i8, df: i32) -> i32 {
    unsafe {
        let size: i32 = getnum(fmt, df);
        if ((size > 16 as i32 || size <= 0) as i32 != 0) as i64 != 0 {
            return lual_error(
                (*h).state,
                b"integral size (%d) out of limits [1,%d]\0" as *const u8 as *const i8,
                size,
                16 as i32,
            );
        }
        return size;
    }
}
pub unsafe extern "C" fn initheader(state: *mut State, h: *mut Header) {
    unsafe {
        (*h).state = state;
        (*h).islittle = NATIVE_ENDIAN.little as i32;
        (*h).maxalign = 1;
    }
}
pub unsafe extern "C" fn getoption(h: *mut Header, fmt: *mut *const i8, size: *mut i32) -> K {
    unsafe {
        let fresh180 = *fmt;
        *fmt = (*fmt).offset(1);
        let opt: i32 = *fresh180 as i32;
        *size = 0;
        match opt {
            98 => {
                *size = ::core::mem::size_of::<i8>() as i32;
                return K::Integer;
            }
            66 => {
                *size = ::core::mem::size_of::<i8>() as i32;
                return K::Unsigned;
            }
            104 => {
                *size = ::core::mem::size_of::<i16>() as i32;
                return K::Integer;
            }
            72 => {
                *size = ::core::mem::size_of::<i16>() as i32;
                return K::Unsigned;
            }
            108 => {
                *size = ::core::mem::size_of::<i64>() as i32;
                return K::Integer;
            }
            76 => {
                *size = ::core::mem::size_of::<i64>() as i32;
                return K::Unsigned;
            }
            106 => {
                *size = ::core::mem::size_of::<i64>() as i32;
                return K::Integer;
            }
            74 => {
                *size = ::core::mem::size_of::<i64>() as i32;
                return K::Unsigned;
            }
            84 => {
                *size = ::core::mem::size_of::<u64>() as i32;
                return K::Unsigned;
            }
            102 => {
                *size = ::core::mem::size_of::<libc::c_float>() as i32;
                return K::Float;
            }
            110 => {
                *size = ::core::mem::size_of::<f64>() as i32;
                return K::Number;
            }
            100 => {
                *size = ::core::mem::size_of::<f64>() as i32;
                return K::Double;
            }
            105 => {
                *size = getnumlimit(h, fmt, ::core::mem::size_of::<i32>() as i32);
                return K::Integer;
            }
            73 => {
                *size = getnumlimit(h, fmt, ::core::mem::size_of::<i32>() as i32);
                return K::Unsigned;
            }
            115 => {
                *size = getnumlimit(h, fmt, ::core::mem::size_of::<u64>() as i32);
                return K::String;
            }
            99 => {
                *size = getnum(fmt, -1);
                if ((*size == -1) as i32 != 0) as i64 != 0 {
                    lual_error(
                        (*h).state,
                        b"missing size for format option 'c'\0" as *const u8 as *const i8,
                    );
                }
                return K::Character;
            }
            122 => return K::ZString,
            120 => {
                *size = 1;
                return K::Padding;
            }
            88 => return K::PaddingAlignment,
            32 => {}
            60 => {
                (*h).islittle = 1;
            }
            62 => {
                (*h).islittle = 0;
            }
            61 => {
                (*h).islittle = NATIVE_ENDIAN.little as i32;
            }
            33 => {
                let maxalign: i32 = 8;
                (*h).maxalign = getnumlimit(h, fmt, maxalign);
            }
            _ => {
                lual_error(
                    (*h).state,
                    b"invalid format option '%c'\0" as *const u8 as *const i8,
                    opt,
                );
            }
        }
        return K::NoOperator;
    }
}
pub unsafe extern "C" fn getdetails(
    h: *mut Header,
    totalsize: u64,
    fmt: *mut *const i8,
    total_size: *mut i32,
    ntoalign: *mut i32,
) -> K {
    unsafe {
        let opt: K = getoption(h, fmt, total_size);
        let mut align: i32 = *total_size;
        if opt as u32 == K::PaddingAlignment as u32 {
            if **fmt as i32 == '\0' as i32
                || getoption(h, fmt, &mut align) as u32 == K::Character as u32
                || align == 0
            {
                lual_argerror(
                    (*h).state,
                    1,
                    b"invalid next option for option 'X'\0" as *const u8 as *const i8,
                );
            }
        }
        if align <= 1 || opt as u32 == K::Character as u32 {
            *ntoalign = 0;
        } else {
            if align > (*h).maxalign {
                align = (*h).maxalign;
            }
            if ((align & align - 1 != 0) as i32 != 0) as i64 != 0 {
                lual_argerror(
                    (*h).state,
                    1,
                    b"format asks for alignment not power of 2\0" as *const u8 as *const i8,
                );
            }
            *ntoalign = align - (totalsize & (align - 1) as u64) as i32 & align - 1;
        }
        return opt;
    }
}
pub unsafe extern "C" fn packint(
    b: *mut Buffer,
    mut n: u64,
    islittle: i32,
    size: i32,
    is_negative_: i32,
) {
    unsafe {
        let buffer: *mut i8 = (*b).lual_prepbuffsize(size as u64);
        let mut i: i32;
        *buffer.offset((if islittle != 0 { 0 } else { size - 1 }) as isize) =
            (n & ((1 << 8) - 1) as u64) as i8;
        i = 1;
        while i < size {
            n >>= 8;
            *buffer.offset((if islittle != 0 { i } else { size - 1 - i }) as isize) =
                (n & ((1 << 8) - 1) as u64) as i8;
            i += 1;
        }
        if is_negative_ != 0 && size > ::core::mem::size_of::<i64>() as i32 {
            i = ::core::mem::size_of::<i64>() as i32;
            while i < size {
                *buffer.offset((if islittle != 0 { i } else { size - 1 - i }) as isize) =
                    ((1 << 8) - 1) as i8;
                i += 1;
            }
        }
        (*b).length = ((*b).length as u64).wrapping_add(size as u64) as u64;
    }
}
pub unsafe extern "C" fn copywithendian(
    mut dest: *mut i8,
    mut src: *const i8,
    mut size: i32,
    islittle: i32,
) {
    unsafe {
        if islittle == NATIVE_ENDIAN.little as i32 {
            memcpy(
                dest as *mut libc::c_void,
                src as *const libc::c_void,
                size as u64,
            );
        } else {
            dest = dest.offset((size - 1) as isize);
            loop {
                let fresh181 = size;
                size = size - 1;
                if !(fresh181 != 0) {
                    break;
                }
                let fresh182 = src;
                src = src.offset(1);
                let fresh183 = dest;
                dest = dest.offset(-1);
                *fresh183 = *fresh182;
            }
        };
    }
}
pub unsafe extern "C" fn str_pack(state: *mut State) -> i32 {
    unsafe {
        let mut b = Buffer::new();
        let mut h: Header = Header {
            state: std::ptr::null_mut(),
            islittle: 0,
            maxalign: 0,
        };
        let mut fmt: *const i8 = lual_checklstring(state, 1, std::ptr::null_mut());
        let mut arg: i32 = 1;
        let mut totalsize: u64 = 0;
        initheader(state, &mut h);
        (*state).push_nil();
        b.lual_buffinit(state);
        while *fmt as i32 != '\0' as i32 {
            let mut size: i32 = 0;
            let mut ntoalign: i32 = 0;
            let opt: K = getdetails(&mut h, totalsize, &mut fmt, &mut size, &mut ntoalign);
            totalsize = (totalsize as u64).wrapping_add((ntoalign + size) as u64) as u64;
            loop {
                let fresh184 = ntoalign;
                ntoalign = ntoalign - 1;
                if !(fresh184 > 0) {
                    break;
                }
                (b.length < b.size || !(b.lual_prepbuffsize(1 as u64)).is_null()) as i32;
                let fresh185 = b.length;
                b.length = (b.length).wrapping_add(1);
                *(b.pointer).offset(fresh185 as isize) = 0 as i8;
            }
            arg += 1;
            let current_block_33: u64;
            match opt as u32 {
                0 => {
                    let n: i64 = lual_checkinteger(state, arg);
                    if size < ::core::mem::size_of::<i64>() as i32 {
                        let lim: i64 = 1 << size * 8 - 1;
                        (((-lim <= n && n < lim) as i32 != 0) as i64 != 0
                            || lual_argerror(
                                state,
                                arg,
                                b"integer overflow\0" as *const u8 as *const i8,
                            ) != 0) as i32;
                    }
                    packint(&mut b, n as u64, h.islittle, size, (n < 0) as i32);
                    current_block_33 = 3222590281903869779;
                }
                1 => {
                    let n_0: i64 = lual_checkinteger(state, arg);
                    if size < ::core::mem::size_of::<i64>() as i32 {
                        ((((n_0 as u64) < (1 as u64) << size * 8) as i32 != 0) as i64 != 0
                            || lual_argerror(
                                state,
                                arg,
                                b"unsigned overflow\0" as *const u8 as *const i8,
                            ) != 0) as i32;
                    }
                    packint(&mut b, n_0 as u64, h.islittle, size, 0);
                    current_block_33 = 3222590281903869779;
                }
                2 => {
                    let mut f: libc::c_float = lual_checknumber(state, arg) as libc::c_float;
                    let buffer: *mut i8 =
                        b.lual_prepbuffsize(::core::mem::size_of::<libc::c_float>() as u64);
                    copywithendian(
                        buffer,
                        &mut f as *mut libc::c_float as *mut i8,
                        ::core::mem::size_of::<libc::c_float>() as i32,
                        h.islittle,
                    );
                    b.length = (b.length as u64).wrapping_add(size as u64) as u64;
                    current_block_33 = 3222590281903869779;
                }
                3 => {
                    let mut f_0: f64 = lual_checknumber(state, arg);
                    let buff_0: *mut i8 = b.lual_prepbuffsize(::core::mem::size_of::<f64>() as u64);
                    copywithendian(
                        buff_0,
                        &mut f_0 as *mut f64 as *mut i8,
                        ::core::mem::size_of::<f64>() as i32,
                        h.islittle,
                    );
                    b.length = (b.length as u64).wrapping_add(size as u64) as u64;
                    current_block_33 = 3222590281903869779;
                }
                4 => {
                    let mut f_1: f64 = lual_checknumber(state, arg);
                    let buff_1: *mut i8 = b.lual_prepbuffsize(::core::mem::size_of::<f64>() as u64);
                    copywithendian(
                        buff_1,
                        &mut f_1 as *mut f64 as *mut i8,
                        ::core::mem::size_of::<f64>() as i32,
                        h.islittle,
                    );
                    b.length = (b.length as u64).wrapping_add(size as u64) as u64;
                    current_block_33 = 3222590281903869779;
                }
                5 => {
                    let mut length: u64 = 0;
                    let s: *const i8 = lual_checklstring(state, arg, &mut length);
                    (((length <= size as u64) as i32 != 0) as i64 != 0
                        || lual_argerror(
                            state,
                            arg,
                            b"string longer than given size\0" as *const u8 as *const i8,
                        ) != 0) as i32;
                    b.lual_addlstring(s, length);
                    loop {
                        let fresh186 = length;
                        length = length.wrapping_add(1);
                        if !(fresh186 < size as u64) {
                            break;
                        }
                        (b.length < b.size || !(b.lual_prepbuffsize(1 as u64)).is_null()) as i32;
                        let fresh187 = b.length;
                        b.length = (b.length).wrapping_add(1);
                        *(b.pointer).offset(fresh187 as isize) = 0 as i8;
                    }
                    current_block_33 = 3222590281903869779;
                }
                6 => {
                    let mut length_0: u64 = 0;
                    let s_0: *const i8 = lual_checklstring(state, arg, &mut length_0);
                    (((size >= ::core::mem::size_of::<u64>() as i32
                        || length_0 < (1 as u64) << size * 8) as i32
                        != 0) as i64
                        != 0
                        || lual_argerror(
                            state,
                            arg,
                            b"string length does not fit in given size\0" as *const u8 as *const i8,
                        ) != 0) as i32;
                    packint(&mut b, length_0 as u64, h.islittle, size, 0);
                    b.lual_addlstring(s_0, length_0);
                    totalsize = (totalsize as u64).wrapping_add(length_0) as u64;
                    current_block_33 = 3222590281903869779;
                }
                7 => {
                    let mut length_1: u64 = 0;
                    let s_1: *const i8 = lual_checklstring(state, arg, &mut length_1);
                    (((strlen(s_1) == length_1) as i32 != 0) as i64 != 0
                        || lual_argerror(
                            state,
                            arg,
                            b"string contains zeros\0" as *const u8 as *const i8,
                        ) != 0) as i32;
                    b.lual_addlstring(s_1, length_1);
                    (b.length < b.size || !(b.lual_prepbuffsize(1 as u64)).is_null()) as i32;
                    let fresh188 = b.length;
                    b.length = (b.length).wrapping_add(1);
                    *(b.pointer).offset(fresh188 as isize) = '\0' as i8;
                    totalsize = (totalsize as u64).wrapping_add(length_1.wrapping_add(1 as u64))
                        as u64;
                    current_block_33 = 3222590281903869779;
                }
                8 => {
                    (b.length < b.size || !(b.lual_prepbuffsize(1 as u64)).is_null()) as i32;
                    let fresh189 = b.length;
                    b.length = (b.length).wrapping_add(1);
                    *(b.pointer).offset(fresh189 as isize) = 0 as i8;
                    current_block_33 = 7383952003695197780;
                }
                9 | 10 => {
                    current_block_33 = 7383952003695197780;
                }
                _ => {
                    current_block_33 = 3222590281903869779;
                }
            }
            match current_block_33 {
                7383952003695197780 => {
                    arg -= 1;
                }
                _ => {}
            }
        }
        b.lual_pushresult();
        return 1;
    }
}
pub unsafe extern "C" fn str_packsize(state: *mut State) -> i32 {
    unsafe {
        let mut h: Header = Header {
            state: std::ptr::null_mut(),
            islittle: 0,
            maxalign: 0,
        };
        let mut fmt: *const i8 = lual_checklstring(state, 1, std::ptr::null_mut());
        let mut totalsize: u64 = 0;
        initheader(state, &mut h);
        while *fmt as i32 != '\0' as i32 {
            let mut size: i32 = 0;
            let mut ntoalign: i32 = 0;
            let opt: K = getdetails(&mut h, totalsize, &mut fmt, &mut size, &mut ntoalign);
            (((opt as u32 != K::String as u32 && opt as u32 != K::ZString as u32)
                as i32
                != 0) as i64
                != 0
                || lual_argerror(
                    state,
                    1,
                    b"variable-length format\0" as *const u8 as *const i8,
                ) != 0) as i32;
            size += ntoalign;
            (((totalsize
            <= (if (::core::mem::size_of::<u64>() as u64) < ::core::mem::size_of::<i32>() as u64 {
                !(0u64)
            } else {
                0x7FFFFFFF as u64
            })
            .wrapping_sub(size as u64)) as i32
            != 0) as i64
            != 0
            || lual_argerror(
                state,
                1,
                b"format result too large\0" as *const u8 as *const i8,
            ) != 0) as i32;
            totalsize = (totalsize as u64).wrapping_add(size as u64) as u64;
        }
        (*state).push_integer(totalsize as i64);
        return 1;
    }
}
pub unsafe extern "C" fn unpackint(
    state: *mut State,
    str: *const i8,
    islittle: i32,
    size: i32,
    issigned: i32,
) -> i64 {
    unsafe {
        let mut res: u64 = 0;
        let mut i: i32;
        let limit: i32 = if size <= ::core::mem::size_of::<i64>() as i32 {
            size
        } else {
            ::core::mem::size_of::<i64>() as i32
        };
        i = limit - 1;
        while i >= 0 {
            res <<= 8;
            res |=
                *str.offset((if islittle != 0 { i } else { size - 1 - i }) as isize) as u8 as u64;
            i -= 1;
        }
        if size < ::core::mem::size_of::<i64>() as i32 {
            if issigned != 0 {
                let mask: u64 = (1 as u64) << size * 8 - 1;
                res = (res ^ mask).wrapping_sub(mask);
            }
        } else if size > ::core::mem::size_of::<i64>() as i32 {
            let mask_0: i32 = if issigned == 0 || res as i64 >= 0 {
                0
            } else {
                (1 << 8) - 1
            };
            i = limit;
            while i < size {
                if ((*str.offset((if islittle != 0 { i } else { size - 1 - i }) as isize) as u8
                    as i32
                    != mask_0) as i32
                    != 0) as i64
                    != 0
                {
                    lual_error(
                        state,
                        b"%d-byte integer does not fit into Lua Integer\0" as *const u8
                            as *const i8,
                        size,
                    );
                }
                i += 1;
            }
        }
        return res as i64;
    }
}
pub unsafe extern "C" fn str_unpack(state: *mut State) -> i32 {
    unsafe {
        let mut h: Header = Header {
            state: std::ptr::null_mut(),
            islittle: 0,
            maxalign: 0,
        };
        let mut fmt: *const i8 = lual_checklstring(state, 1, std::ptr::null_mut());
        let mut ld: u64 = 0;
        let data: *const i8 = lual_checklstring(state, 2, &mut ld);
        let mut pos: u64 =
            (posrelati(lual_optinteger(state, 3, 1 as i64), ld)).wrapping_sub(1 as u64);
        let mut n: i32 = 0;
        (((pos <= ld) as i32 != 0) as i64 != 0
            || lual_argerror(
                state,
                3,
                b"initial position out of string\0" as *const u8 as *const i8,
            ) != 0) as i32;
        initheader(state, &mut h);
        while *fmt as i32 != '\0' as i32 {
            let mut size: i32 = 0;
            let mut ntoalign: i32 = 0;
            let opt: K = getdetails(&mut h, pos, &mut fmt, &mut size, &mut ntoalign);
            ((((ntoalign as u64).wrapping_add(size as u64) <= ld.wrapping_sub(pos)) as i32 != 0)
                as i64
                != 0
                || lual_argerror(
                    state,
                    2,
                    b"data string too short\0" as *const u8 as *const i8,
                ) != 0) as i32;
            pos = (pos as u64).wrapping_add(ntoalign as u64) as u64;
            lual_checkstack(state, 2, b"too many results\0" as *const u8 as *const i8);
            n += 1;
            match opt as u32 {
                0 | 1 => {
                    let res: i64 = unpackint(
                        state,
                        data.offset(pos as isize),
                        h.islittle,
                        size,
                        (opt as u32 == K::Integer as u32) as i32,
                    );
                    (*state).push_integer(res);
                }
                2 => {
                    let mut f: libc::c_float = 0.0;
                    copywithendian(
                        &mut f as *mut libc::c_float as *mut i8,
                        data.offset(pos as isize),
                        ::core::mem::size_of::<libc::c_float>() as i32,
                        h.islittle,
                    );
                    (*state).push_number(f as f64);
                }
                3 => {
                    let mut f_0: f64 = 0.0;
                    copywithendian(
                        &mut f_0 as *mut f64 as *mut i8,
                        data.offset(pos as isize),
                        ::core::mem::size_of::<f64>() as i32,
                        h.islittle,
                    );
                    (*state).push_number(f_0);
                }
                4 => {
                    let mut f_1: f64 = 0.0;
                    copywithendian(
                        &mut f_1 as *mut f64 as *mut i8,
                        data.offset(pos as isize),
                        ::core::mem::size_of::<f64>() as i32,
                        h.islittle,
                    );
                    (*state).push_number(f_1);
                }
                5 => {
                    lua_pushlstring(state, data.offset(pos as isize), size as u64);
                }
                6 => {
                    let length: u64 =
                        unpackint(state, data.offset(pos as isize), h.islittle, size, 0) as u64;
                    (((length <= ld.wrapping_sub(pos).wrapping_sub(size as u64)) as i32 != 0) as i32
                        as i64
                        != 0
                        || lual_argerror(
                            state,
                            2,
                            b"data string too short\0" as *const u8 as *const i8,
                        ) != 0) as i32;
                    lua_pushlstring(
                        state,
                        data.offset(pos as isize).offset(size as isize),
                        length,
                    );
                    pos = (pos as u64).wrapping_add(length) as u64;
                }
                7 => {
                    let length_0: u64 = strlen(data.offset(pos as isize));
                    (((pos.wrapping_add(length_0) < ld) as i32 != 0) as i64 != 0
                        || lual_argerror(
                            state,
                            2,
                            b"unfinished string for format 'zio'\0" as *const u8 as *const i8,
                        ) != 0) as i32;
                    lua_pushlstring(state, data.offset(pos as isize), length_0);
                    pos = (pos as u64).wrapping_add(length_0.wrapping_add(1 as u64)) as u64;
                }
                9 | 8 | 10 => {
                    n -= 1;
                }
                _ => {}
            }
            pos = (pos as u64).wrapping_add(size as u64) as u64;
        }
        (*state).push_integer(pos.wrapping_add(1 as u64) as i64);
        return n + 1;
    }
}
pub const STRING_FUNCTIONS: [RegisteredFunction; 18] = {
    [
        {
            RegisteredFunction {
                name: b"byte\0" as *const u8 as *const i8,
                function: Some(str_byte as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"char\0" as *const u8 as *const i8,
                function: Some(str_char as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"dump\0" as *const u8 as *const i8,
                function: Some(str_dump as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"find\0" as *const u8 as *const i8,
                function: Some(str_find as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"format\0" as *const u8 as *const i8,
                function: Some(str_format as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"gmatch\0" as *const u8 as *const i8,
                function: Some(GMatchState::gmatch as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"gsub\0" as *const u8 as *const i8,
                function: Some(str_gsub as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"len\0" as *const u8 as *const i8,
                function: Some(str_len as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"lower\0" as *const u8 as *const i8,
                function: Some(str_lower as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"match\0" as *const u8 as *const i8,
                function: Some(str_match as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"rep\0" as *const u8 as *const i8,
                function: Some(str_rep as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"reverse\0" as *const u8 as *const i8,
                function: Some(str_reverse as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"sub\0" as *const u8 as *const i8,
                function: Some(str_sub as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"upper\0" as *const u8 as *const i8,
                function: Some(str_upper as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"pack\0" as *const u8 as *const i8,
                function: Some(str_pack as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"packsize\0" as *const u8 as *const i8,
                function: Some(str_packsize as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"unpack\0" as *const u8 as *const i8,
                function: Some(str_unpack as unsafe extern "C" fn(*mut State) -> i32),
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
pub unsafe extern "C" fn createmetatable(state: *mut State) {
    unsafe {
        (*state).lua_createtable();
        lual_setfuncs(state, STRING_METAMETHODS.as_ptr(), 0);
        lua_pushstring(state, b"\0" as *const u8 as *const i8);
        lua_pushvalue(state, -2);
        lua_setmetatable(state, -2);
        lua_settop(state, -2);
        lua_pushvalue(state, -2);
        lua_setfield(state, -2, b"__index\0" as *const u8 as *const i8);
        lua_settop(state, -2);
    }
}
pub unsafe extern "C" fn luaopen_string(state: *mut State) -> i32 {
    unsafe {
        lual_checkversion_(
            state,
            504.0,
            (::core::mem::size_of::<i64>() as u64)
                .wrapping_mul(16 as u64)
                .wrapping_add(::core::mem::size_of::<f64>() as u64),
        );
        (*state).lua_createtable();
        lual_setfuncs(state, STRING_FUNCTIONS.as_ptr(), 0);
        createmetatable(state);
        return 1;
    }
}
pub unsafe extern "C" fn u_posrelat(pos: i64, length: u64) -> i64 {
    if pos >= 0 {
        return pos;
    } else if (0u64).wrapping_sub(pos as u64) > length {
        return 0;
    } else {
        return length as i64 + pos + 1;
    };
}
pub unsafe extern "C" fn utf8_decode(mut s: *const i8, value: *mut u32, strict: i32) -> *const i8 {
    unsafe {
        pub const LIMITS: [u32; 6] = [
            !(0u32),
            0x80 as u32,
            0x800 as u32,
            0x10000 as u32,
            0x200000 as u32,
            0x4000000 as u32,
        ];
        let mut c: u32 = *s.offset(0 as isize) as u8 as u32;
        let mut res: u32 = 0u32;
        if c < 0x80 as u32 {
            res = c;
        } else {
            let mut count: i32 = 0;
            while c & 0x40 as u32 != 0 {
                count += 1;
                let cc: u32 = *s.offset(count as isize) as u8 as u32;
                if !(cc & 0xc0 as u32 == 0x80 as u32) {
                    return std::ptr::null();
                }
                res = res << 6 | cc & 0x3f as u32;
                c <<= 1;
            }
            res |= (c & 0x7f as u32) << count * 5;
            if count > 5 || res > 0x7fffffff as u32 || res < LIMITS[count as usize] {
                return std::ptr::null();
            }
            s = s.offset(count as isize);
        }
        if strict != 0 {
            if res > 0x10ffff as u32 || 0xd800 as u32 <= res && res <= 0xdfff as u32 {
                return std::ptr::null();
            }
        }
        if !value.is_null() {
            *value = res;
        }
        return s.offset(1 as isize);
    }
}
pub unsafe extern "C" fn utflen(state: *mut State) -> i32 {
    unsafe {
        let mut n: i64 = 0;
        let mut length: u64 = 0;
        let s: *const i8 = lual_checklstring(state, 1, &mut length);
        let mut posi: i64 = u_posrelat(lual_optinteger(state, 2, 1 as i64), length);
        let mut posj: i64 = u_posrelat(lual_optinteger(state, 3, -1 as i64), length);
        let lax: i32 = lua_toboolean(state, 4);
        (((1 <= posi && {
            posi -= 1;
            posi <= length as i64
        }) as i32
            != 0) as i64
            != 0
            || lual_argerror(
                state,
                2,
                b"initial position out of bounds\0" as *const u8 as *const i8,
            ) != 0) as i32;
        posj -= 1;
        (((posj < length as i64) as i32 != 0) as i64 != 0
            || lual_argerror(
                state,
                3,
                b"final position out of bounds\0" as *const u8 as *const i8,
            ) != 0) as i32;
        while posi <= posj {
            let s1: *const i8 = utf8_decode(
                s.offset(posi as isize),
                std::ptr::null_mut(),
                (lax == 0) as i32,
            );
            if s1.is_null() {
                (*state).push_nil();
                (*state).push_integer(posi + 1);
                return 2;
            }
            posi = s1.offset_from(s) as i64;
            n += 1;
        }
        (*state).push_integer(n);
        return 1;
    }
}
pub unsafe extern "C" fn codepoint(state: *mut State) -> i32 {
    unsafe {
        let mut length: u64 = 0;
        let mut s: *const i8 = lual_checklstring(state, 1, &mut length);
        let posi: i64 = u_posrelat(lual_optinteger(state, 2, 1 as i64), length);
        let pose: i64 = u_posrelat(lual_optinteger(state, 3, posi), length);
        let lax: i32 = lua_toboolean(state, 4);
        let mut n: i32;
        let se: *const i8;
        (((posi >= 1) as i32 != 0) as i64 != 0
            || lual_argerror(state, 2, b"out of bounds\0" as *const u8 as *const i8) != 0)
            as i32;
        (((pose <= length as i64) as i32 != 0) as i64 != 0
            || lual_argerror(state, 3, b"out of bounds\0" as *const u8 as *const i8) != 0)
            as i32;
        if posi > pose {
            return 0;
        }
        if pose - posi >= 0x7FFFFFFF as i64 {
            return lual_error(state, b"string slice too long\0" as *const u8 as *const i8);
        }
        n = (pose - posi) as i32 + 1;
        lual_checkstack(
            state,
            n,
            b"string slice too long\0" as *const u8 as *const i8,
        );
        n = 0;
        se = s.offset(pose as isize);
        s = s.offset((posi - 1) as isize);
        while s < se {
            let mut code: u32 = 0;
            s = utf8_decode(s, &mut code, (lax == 0) as i32);
            if s.is_null() {
                return lual_error(state, b"invalid UTF-8 code\0" as *const u8 as *const i8);
            }
            (*state).push_integer(code as i64);
            n += 1;
        }
        return n;
    }
}
pub unsafe extern "C" fn pushutfchar(state: *mut State, arg: i32) {
    unsafe {
        let code: u64 = lual_checkinteger(state, arg) as u64;
        (((code <= 0x7fffffff as u64) as i32 != 0) as i64 != 0
            || lual_argerror(
                state,
                arg,
                b"value out of range\0" as *const u8 as *const i8,
            ) != 0) as i32;
        lua_pushfstring(state, b"%U\0" as *const u8 as *const i8, code as i64);
    }
}
pub unsafe extern "C" fn utfchar(state: *mut State) -> i32 {
    unsafe {
        let n: i32 = (*state).get_top();
        if n == 1 {
            pushutfchar(state, 1);
        } else {
            let mut i: i32;
            let mut b = Buffer::new();
            b.lual_buffinit(state);
            i = 1;
            while i <= n {
                pushutfchar(state, i);
                b.lual_addvalue();
                i += 1;
            }
            b.lual_pushresult();
        }
        return 1;
    }
}
pub unsafe extern "C" fn byteoffset(state: *mut State) -> i32 {
    unsafe {
        let mut length: u64 = 0;
        let s: *const i8 = lual_checklstring(state, 1, &mut length);
        let mut n: i64 = lual_checkinteger(state, 2);
        let mut posi: i64 = (if n >= 0 {
            1 as u64
        } else {
            length.wrapping_add(1 as u64)
        }) as i64;
        posi = u_posrelat(lual_optinteger(state, 3, posi), length);
        (((1 <= posi && {
            posi -= 1;
            posi <= length as i64
        }) as i32
            != 0) as i64
            != 0
            || lual_argerror(
                state,
                3,
                b"position out of bounds\0" as *const u8 as *const i8,
            ) != 0) as i32;
        if n == 0 {
            while posi > 0 && *s.offset(posi as isize) as i32 & 0xc0 as i32 == 0x80 as i32 {
                posi -= 1;
            }
        } else {
            if *s.offset(posi as isize) as i32 & 0xc0 as i32 == 0x80 as i32 {
                return lual_error(
                    state,
                    b"initial position is a continuation byte\0" as *const u8 as *const i8,
                );
            }
            if n < 0 {
                while n < 0 && posi > 0 {
                    loop {
                        posi -= 1;
                        if !(posi > 0
                            && *s.offset(posi as isize) as i32 & 0xc0 as i32 == 0x80 as i32)
                        {
                            break;
                        }
                    }
                    n += 1;
                }
            } else {
                n -= 1;
                while n > 0 && posi < length as i64 {
                    loop {
                        posi += 1;
                        if !(*s.offset(posi as isize) as i32 & 0xc0 as i32 == 0x80 as i32) {
                            break;
                        }
                    }
                    n -= 1;
                }
            }
        }
        if n == 0 {
            (*state).push_integer(posi + 1);
        } else {
            (*state).push_nil();
        }
        return 1;
    }
}
pub unsafe extern "C" fn iter_aux(state: *mut State, strict: i32) -> i32 {
    unsafe {
        let mut length: u64 = 0;
        let s: *const i8 = lual_checklstring(state, 1, &mut length);
        let mut n: u64 = lua_tointegerx(state, 2, std::ptr::null_mut()) as u64;
        if n < length as u64 {
            while *s.offset(n as isize) as i32 & 0xc0 as i32 == 0x80 as i32 {
                n = n.wrapping_add(1);
            }
        }
        if n >= length as u64 {
            return 0;
        } else {
            let mut code: u32 = 0;
            let next: *const i8 = utf8_decode(s.offset(n as isize), &mut code, strict);
            if next.is_null() || *next as i32 & 0xc0 as i32 == 0x80 as i32 {
                return lual_error(state, b"invalid UTF-8 code\0" as *const u8 as *const i8);
            }
            (*state).push_integer(n.wrapping_add(1 as u64) as i64);
            (*state).push_integer(code as i64);
            return 2;
        };
    }
}
pub unsafe extern "C" fn iter_auxstrict(state: *mut State) -> i32 {
    unsafe {
        return iter_aux(state, 1);
    }
}
pub unsafe extern "C" fn iter_auxlax(state: *mut State) -> i32 {
    unsafe {
        return iter_aux(state, 0);
    }
}
pub unsafe extern "C" fn iter_codes(state: *mut State) -> i32 {
    unsafe {
        let lax: i32 = lua_toboolean(state, 2);
        let s: *const i8 = lual_checklstring(state, 1, std::ptr::null_mut());
        ((!(*s as i32 & 0xc0 as i32 == 0x80 as i32) as i32 != 0) as i64 != 0
            || lual_argerror(state, 1, b"invalid UTF-8 code\0" as *const u8 as *const i8) != 0)
            as i32;
        lua_pushcclosure(
            state,
            if lax != 0 {
                Some(iter_auxlax as unsafe extern "C" fn(*mut State) -> i32)
            } else {
                Some(iter_auxstrict as unsafe extern "C" fn(*mut State) -> i32)
            },
            0,
        );
        lua_pushvalue(state, 1);
        (*state).push_integer(0);
        return 3;
    }
}
pub const UTF8_FUNCTIONS: [RegisteredFunction; 7] = {
    [
        {
            RegisteredFunction {
                name: b"offset\0" as *const u8 as *const i8,
                function: Some(byteoffset as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"codepoint\0" as *const u8 as *const i8,
                function: Some(codepoint as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"char\0" as *const u8 as *const i8,
                function: Some(utfchar as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"len\0" as *const u8 as *const i8,
                function: Some(utflen as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"codes\0" as *const u8 as *const i8,
                function: Some(iter_codes as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"charpattern\0" as *const u8 as *const i8,
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
pub unsafe extern "C" fn luaopen_utf8(state: *mut State) -> i32 {
    unsafe {
        lual_checkversion_(
            state,
            504.0,
            (::core::mem::size_of::<i64>() as u64)
                .wrapping_mul(16 as u64)
                .wrapping_add(::core::mem::size_of::<f64>() as u64),
        );
        (*state).lua_createtable();
        lual_setfuncs(state, UTF8_FUNCTIONS.as_ptr(), 0);
        lua_pushlstring(
            state,
            b"[\0-\x7F\xC2-\xFD][\x80-\xBF]*\0" as *const u8 as *const i8,
            (::core::mem::size_of::<[i8; 15]>() as u64)
                .wrapping_div(::core::mem::size_of::<i8>() as u64)
                .wrapping_sub(1 as u64),
        );
        lua_setfield(state, -2, b"charpattern\0" as *const u8 as *const i8);
        return 1;
    }
}
pub const HOOKKEY: *const i8 = b"_HOOKKEY\0" as *const u8 as *const i8;
pub unsafe extern "C" fn checkstack(state: *mut State, other_state: *mut State, n: i32) {
    unsafe {
        if ((state != other_state && lua_checkstack(other_state, n) == 0) as i32 != 0) as i64
            != 0
        {
            lual_error(state, b"stack overflow\0" as *const u8 as *const i8);
        }
    }
}
pub unsafe extern "C" fn getthread(state: *mut State, arg: *mut i32) -> *mut State {
    unsafe {
        if lua_type(state, 1) == Some(TAG_TYPE_STATE) {
            *arg = 1;
            return lua_tothread(state, 1);
        } else {
            *arg = 0;
            return state;
        };
    }
}
pub unsafe extern "C" fn settabss(state: *mut State, k: *const i8, v: *const i8) {
    unsafe {
        lua_pushstring(state, v);
        lua_setfield(state, -2, k);
    }
}
pub unsafe extern "C" fn settabsi(state: *mut State, k: *const i8, v: i32) {
    unsafe {
        (*state).push_integer(v as i64);
        lua_setfield(state, -2, k);
    }
}
pub unsafe extern "C" fn settabsb(state: *mut State, k: *const i8, v: i32) {
    unsafe {
        (*state).push_boolean(v != 0);
        lua_setfield(state, -2, k);
    }
}
pub unsafe extern "C" fn treatstackoption(
    state: *mut State,
    other_state: *mut State,
    fname: *const i8,
) {
    unsafe {
        if state == other_state {
            lua_rotate(state, -2, 1);
        } else {
            lua_xmove(other_state, state, 1);
        }
        lua_setfield(state, -2, fname);
    }
}
pub unsafe extern "C" fn auxupvalue(state: *mut State, get: i32) -> i32 {
    unsafe {
        let n: i32 = lual_checkinteger(state, 2) as i32;
        lual_checktype(state, 1, TAG_TYPE_CLOSURE);
        let name: *const i8 = if get != 0 {
            lua_getupvalue(state, 1, n)
        } else {
            lua_setupvalue(state, 1, n)
        };
        if name.is_null() {
            return 0;
        } else {
            lua_pushstring(state, name);
            lua_rotate(state, -(get + 1), 1);
            return get + 1;
        }
    }
}
pub unsafe extern "C" fn checkupval(
    state: *mut State,
    argf: i32,
    argnup: i32,
    pnup: *mut i32,
) -> *mut libc::c_void {
    unsafe {
        let id: *mut libc::c_void;
        let nup: i32 = lual_checkinteger(state, argnup) as i32;
        lual_checktype(state, argf, TAG_TYPE_CLOSURE);
        id = lua_upvalueid(state, argf, nup);
        if !pnup.is_null() {
            (((id != std::ptr::null_mut()) as i32 != 0) as i64 != 0
                || lual_argerror(
                    state,
                    argnup,
                    b"invalid upvalue index\0" as *const u8 as *const i8,
                ) != 0) as i32;
            *pnup = nup;
        }
        return id;
    }
}
pub unsafe extern "C" fn hookf(state: *mut State, ar: *mut DebugInfo) {
    unsafe {
        pub const HOOK_NAMES: [*const i8; 5] = [
            b"call\0" as *const u8 as *const i8,
            b"return\0" as *const u8 as *const i8,
            b"line\0" as *const u8 as *const i8,
            b"count\0" as *const u8 as *const i8,
            b"tail call\0" as *const u8 as *const i8,
        ];
        lua_getfield(state, -(1000000 as i32) - 1000 as i32, HOOKKEY);
        (*state).push_state();
        if lua_rawget(state, -2) == 6 {
            lua_pushstring(state, HOOK_NAMES[(*ar).event as usize]);
            if (*ar).currentline >= 0 {
                (*state).push_integer((*ar).currentline as i64);
            } else {
                (*state).push_nil();
            }
            lua_callk(state, 2, 0, 0, None);
        }
    }
}
pub unsafe extern "C" fn makemask(smask: *const i8, count: i32) -> i32 {
    unsafe {
        let mut mask: i32 = 0;
        if !(strchr(smask, 'c' as i32)).is_null() {
            mask |= 1 << 0;
        }
        if !(strchr(smask, 'r' as i32)).is_null() {
            mask |= 1 << 1;
        }
        if !(strchr(smask, 'l' as i32)).is_null() {
            mask |= 1 << 2;
        }
        if count > 0 {
            mask |= 1 << 3;
        }
        return mask;
    }
}
pub unsafe extern "C" fn unmakemask(mask: i32, smask: *mut i8) -> *mut i8 {
    unsafe {
        let mut i: i32 = 0;
        if mask & 1 << 0 != 0 {
            let fresh190 = i;
            i = i + 1;
            *smask.offset(fresh190 as isize) = 'c' as i8;
        }
        if mask & 1 << 1 != 0 {
            let fresh191 = i;
            i = i + 1;
            *smask.offset(fresh191 as isize) = 'r' as i8;
        }
        if mask & 1 << 2 != 0 {
            let fresh192 = i;
            i = i + 1;
            *smask.offset(fresh192 as isize) = 'l' as i8;
        }
        *smask.offset(i as isize) = '\0' as i8;
        return smask;
    }
}
pub const CLIBS: *const i8 = b"_CLIBS\0" as *const u8 as *const i8;
pub unsafe extern "C" fn lsys_unloadlib(lib: *mut libc::c_void) {
    unsafe {
        dlclose(lib);
    }
}
pub unsafe extern "C" fn lsys_load(
    state: *mut State,
    path: *const i8,
    seeglb: i32,
) -> *mut libc::c_void {
    unsafe {
        let lib: *mut libc::c_void = dlopen(
            path,
            0x2 as i32 | (if seeglb != 0 { 0x100 as i32 } else { 0 }),
        );
        if ((lib == std::ptr::null_mut()) as i32 != 0) as i64 != 0 {
            lua_pushstring(state, dlerror());
        }
        return lib;
    }
}
pub unsafe extern "C" fn lsys_sym(
    state: *mut State,
    lib: *mut libc::c_void,
    sym: *const i8,
) -> CFunction {
    unsafe {
        let f: CFunction = ::core::mem::transmute::<*mut libc::c_void, CFunction>(dlsym(lib, sym));
        if (f.is_none() as i32 != 0) as i64 != 0 {
            lua_pushstring(state, dlerror());
        }
        return f;
    }
}
pub unsafe extern "C" fn noenv(state: *mut State) -> i32 {
    unsafe {
        let b: i32;
        lua_getfield(
            state,
            -(1000000 as i32) - 1000 as i32,
            b"LUA_NOENV\0" as *const u8 as *const i8,
        );
        b = lua_toboolean(state, -1);
        lua_settop(state, -2);
        return b;
    }
}
pub unsafe extern "C" fn setpath(
    state: *mut State,
    fieldname: *const i8,
    envname: *const i8,
    dft: *const i8,
) {
    unsafe {
        let dftmark: *const i8;
        let nver: *const i8 = lua_pushfstring(
            state,
            b"%s%s\0" as *const u8 as *const i8,
            envname,
            b"_5_4\0" as *const u8 as *const i8,
        );
        let mut path: *const i8 = getenv(nver);
        if path.is_null() {
            path = getenv(envname);
        }
        if path.is_null() || noenv(state) != 0 {
            lua_pushstring(state, dft);
        } else {
            dftmark = strstr(path, b";;\0" as *const u8 as *const i8);
            if dftmark.is_null() {
                lua_pushstring(state, path);
            } else {
                let length: u64 = strlen(path);
                let mut b = Buffer::new();
                b.lual_buffinit(state);
                if path < dftmark {
                    b.lual_addlstring(path, dftmark.offset_from(path) as u64);
                    (b.length < b.size || !(b.lual_prepbuffsize(1 as u64)).is_null()) as i32;
                    let fresh193 = b.length;
                    b.length = (b.length).wrapping_add(1);
                    *(b.pointer).offset(fresh193 as isize) = *(b";\0" as *const u8 as *const i8);
                }
                b.lual_addstring(dft);
                if dftmark < path.offset(length as isize).offset(-(2 as isize)) {
                    (b.length < b.size || !(b.lual_prepbuffsize(1 as u64)).is_null()) as i32;
                    let fresh194 = b.length;
                    b.length = (b.length).wrapping_add(1);
                    *(b.pointer).offset(fresh194 as isize) = *(b";\0" as *const u8 as *const i8);
                    b.lual_addlstring(
                        dftmark.offset(2 as isize),
                        path.offset(length as isize)
                            .offset(-(2 as isize))
                            .offset_from(dftmark) as u64,
                    );
                }
                b.lual_pushresult();
            }
        }
        lua_setfield(state, -3, fieldname);
        lua_settop(state, -2);
    }
}
pub unsafe extern "C" fn checkclib(state: *mut State, path: *const i8) -> *mut libc::c_void {
    unsafe {
        let plib: *mut libc::c_void;
        lua_getfield(state, -(1000000 as i32) - 1000 as i32, CLIBS);
        lua_getfield(state, -1, path);
        plib = lua_touserdata(state, -1);
        lua_settop(state, -2 - 1);
        return plib;
    }
}
pub unsafe extern "C" fn addtoclib(state: *mut State, path: *const i8, plib: *mut libc::c_void) {
    unsafe {
        lua_getfield(state, -(1000000 as i32) - 1000 as i32, CLIBS);
        lua_pushlightuserdata(state, plib);
        lua_pushvalue(state, -1);
        lua_setfield(state, -3, path);
        lua_rawseti(state, -2, lual_len(state, -2) + 1);
        lua_settop(state, -2);
    }
}
pub unsafe extern "C" fn gctm(state: *mut State) -> i32 {
    unsafe {
        let mut n: i64 = lual_len(state, 1);
        while n >= 1 {
            lua_rawgeti(state, 1, n);
            lsys_unloadlib(lua_touserdata(state, -1));
            lua_settop(state, -2);
            n -= 1;
        }
        return 0;
    }
}
pub unsafe extern "C" fn lookforfunc(state: *mut State, path: *const i8, sym: *const i8) -> i32 {
    unsafe {
        let mut reg: *mut libc::c_void = checkclib(state, path);
        if reg.is_null() {
            reg = lsys_load(state, path, (*sym as i32 == '*' as i32) as i32);
            if reg.is_null() {
                return 1;
            }
            addtoclib(state, path, reg);
        }
        if *sym as i32 == '*' as i32 {
            (*state).push_boolean(true);
            return 0;
        } else {
            let f: CFunction = lsys_sym(state, reg, sym);
            if f.is_none() {
                return 2;
            }
            lua_pushcclosure(state, f, 0);
            return 0;
        };
    }
}
pub unsafe extern "C" fn ll_loadlib(state: *mut State) -> i32 {
    unsafe {
        let path: *const i8 = lual_checklstring(state, 1, std::ptr::null_mut());
        let init: *const i8 = lual_checklstring(state, 2, std::ptr::null_mut());
        let stat: i32 = lookforfunc(state, path, init);
        if ((stat == 0) as i32 != 0) as i64 != 0 {
            return 1;
        } else {
            (*state).push_nil();
            lua_rotate(state, -2, 1);
            lua_pushstring(
                state,
                if stat == 1 {
                    b"open\0" as *const u8 as *const i8
                } else {
                    b"init\0" as *const u8 as *const i8
                },
            );
            return 3;
        };
    }
}
pub unsafe extern "C" fn readable(filename: *const i8) -> i32 {
    unsafe {
        let f: *mut FILE = fopen(filename, b"r\0" as *const u8 as *const i8);
        if f.is_null() {
            return 0;
        }
        fclose(f);
        return 1;
    }
}
pub unsafe extern "C" fn getnextfilename(path: *mut *mut i8, end: *mut i8) -> *const i8 {
    unsafe {
        let mut name: *mut i8 = *path;
        if name == end {
            return std::ptr::null();
        } else if *name as i32 == '\0' as i32 {
            *name = *(b";\0" as *const u8 as *const i8);
            name = name.offset(1);
        }
        let mut sep: *mut i8 = strchr(name, *(b";\0" as *const u8 as *const i8) as i32);
        if sep.is_null() {
            sep = end;
        }
        *sep = '\0' as i8;
        *path = sep;
        return name;
    }
}
pub unsafe extern "C" fn pusherrornotfound(state: *mut State, path: *const i8) {
    unsafe {
        let mut b = Buffer::new();
        b.lual_buffinit(state);
        b.lual_addstring(b"no file '\0" as *const u8 as *const i8);
        lual_addgsub(
            &mut b,
            path,
            b";\0" as *const u8 as *const i8,
            b"'\n\tno file '\0" as *const u8 as *const i8,
        );
        b.lual_addstring(b"'\0" as *const u8 as *const i8);
        b.lual_pushresult();
    }
}
pub unsafe extern "C" fn searchpath(
    state: *mut State,
    mut name: *const i8,
    path: *const i8,
    sep: *const i8,
    dirsep: *const i8,
) -> *const i8 {
    unsafe {
        let mut pathname;
        let endpathname;
        let mut filename;
        if *sep as i32 != '\0' as i32 && !(strchr(name, *sep as i32)).is_null() {
            name = lual_gsub(state, name, sep, dirsep);
        }
        let mut buffer = Buffer::new();
        buffer.lual_buffinit(state);
        lual_addgsub(&mut buffer, path, b"?\0" as *const u8 as *const i8, name);
        (buffer.length < buffer.size || !(buffer.lual_prepbuffsize(1 as u64)).is_null()) as i32;
        let fresh195 = buffer.length;
        buffer.length = (buffer.length).wrapping_add(1);
        *(buffer.pointer).offset(fresh195 as isize) = '\0' as i8;
        pathname = buffer.pointer;
        endpathname = pathname
            .offset(buffer.length as isize)
            .offset(-(1 as isize));
        loop {
            filename = getnextfilename(&mut pathname, endpathname);
            if filename.is_null() {
                break;
            }
            if readable(filename) != 0 {
                return lua_pushstring(state, filename);
            }
        }
        buffer.lual_pushresult();
        pusherrornotfound(state, lua_tolstring(state, -1, std::ptr::null_mut()));
        return std::ptr::null();
    }
}
pub unsafe extern "C" fn ll_searchpath(state: *mut State) -> i32 {
    unsafe {
        let f: *const i8 = searchpath(
            state,
            lual_checklstring(state, 1, std::ptr::null_mut()),
            lual_checklstring(state, 2, std::ptr::null_mut()),
            lual_optlstring(
                state,
                3,
                b".\0" as *const u8 as *const i8,
                std::ptr::null_mut(),
            ),
            lual_optlstring(
                state,
                4,
                b"/\0" as *const u8 as *const i8,
                std::ptr::null_mut(),
            ),
        );
        if !f.is_null() {
            return 1;
        } else {
            (*state).push_nil();
            lua_rotate(state, -2, 1);
            return 2;
        };
    }
}
pub unsafe extern "C" fn findfile(
    state: *mut State,
    name: *const i8,
    pname: *const i8,
    dirsep: *const i8,
) -> *const i8 {
    unsafe {
        lua_getfield(state, -(1000000 as i32) - 1000 as i32 - 1, pname);
        let path: *const i8 = lua_tolstring(state, -1, std::ptr::null_mut());
        if ((path == std::ptr::null_mut() as *const i8) as i32 != 0) as i64 != 0 {
            lual_error(
                state,
                b"'package.%s' must be a string\0" as *const u8 as *const i8,
                pname,
            );
        }
        return searchpath(state, name, path, b".\0" as *const u8 as *const i8, dirsep);
    }
}
pub unsafe extern "C" fn checkload(state: *mut State, stat: i32, filename: *const i8) -> i32 {
    unsafe {
        if (stat != 0) as i64 != 0 {
            lua_pushstring(state, filename);
            return 2;
        } else {
            return lual_error(
                state,
                b"error loading module '%s' from file '%s':\n\t%s\0" as *const u8 as *const i8,
                lua_tolstring(state, 1, std::ptr::null_mut()),
                filename,
                lua_tolstring(state, -1, std::ptr::null_mut()),
            );
        };
    }
}
pub unsafe extern "C" fn searcher_lua(state: *mut State) -> i32 {
    unsafe {
        let name: *const i8 = lual_checklstring(state, 1, std::ptr::null_mut());
        let filename: *const i8 = findfile(
            state,
            name,
            b"path\0" as *const u8 as *const i8,
            b"/\0" as *const u8 as *const i8,
        );
        if filename.is_null() {
            return 1;
        }
        return checkload(
            state,
            (lual_loadfilex(state, filename, std::ptr::null()) == 0) as i32,
            filename,
        );
    }
}
pub unsafe extern "C" fn loadfunc(
    state: *mut State,
    filename: *const i8,
    mut modname: *const i8,
) -> i32 {
    unsafe {
        modname = lual_gsub(
            state,
            modname,
            b".\0" as *const u8 as *const i8,
            b"_\0" as *const u8 as *const i8,
        );
        let mut openfunc: *const i8;
        let mark: *const i8 = strchr(modname, *(b"-\0" as *const u8 as *const i8) as i32);
        if !mark.is_null() {
            openfunc = lua_pushlstring(state, modname, mark.offset_from(modname) as u64);
            openfunc = lua_pushfstring(state, b"luaopen_%s\0" as *const u8 as *const i8, openfunc);
            let stat: i32 = lookforfunc(state, filename, openfunc);
            if stat != 2 {
                return stat;
            }
            modname = mark.offset(1 as isize);
        }
        openfunc = lua_pushfstring(state, b"luaopen_%s\0" as *const u8 as *const i8, modname);
        return lookforfunc(state, filename, openfunc);
    }
}
pub unsafe extern "C" fn searcher_c(state: *mut State) -> i32 {
    unsafe {
        let name: *const i8 = lual_checklstring(state, 1, std::ptr::null_mut());
        let filename: *const i8 = findfile(
            state,
            name,
            b"cpath\0" as *const u8 as *const i8,
            b"/\0" as *const u8 as *const i8,
        );
        if filename.is_null() {
            return 1;
        }
        return checkload(
            state,
            (loadfunc(state, filename, name) == 0) as i32,
            filename,
        );
    }
}
pub unsafe extern "C" fn searcher_croot(state: *mut State) -> i32 {
    unsafe {
        let name: *const i8 = lual_checklstring(state, 1, std::ptr::null_mut());
        let p: *const i8 = strchr(name, '.' as i32);
        if p.is_null() {
            return 0;
        }
        lua_pushlstring(state, name, p.offset_from(name) as u64);
        let filename: *const i8 = findfile(
            state,
            lua_tolstring(state, -1, std::ptr::null_mut()),
            b"cpath\0" as *const u8 as *const i8,
            b"/\0" as *const u8 as *const i8,
        );
        if filename.is_null() {
            return 1;
        }
        let stat: i32 = loadfunc(state, filename, name);
        if stat != 0 {
            if stat != 2 {
                return checkload(state, 0, filename);
            } else {
                lua_pushfstring(
                    state,
                    b"no module '%s' in file '%s'\0" as *const u8 as *const i8,
                    name,
                    filename,
                );
                return 1;
            }
        }
        lua_pushstring(state, filename);
        return 2;
    }
}
pub unsafe extern "C" fn searcher_preload(state: *mut State) -> i32 {
    unsafe {
        let name: *const i8 = lual_checklstring(state, 1, std::ptr::null_mut());
        lua_getfield(
            state,
            -(1000000 as i32) - 1000 as i32,
            b"_PRELOAD\0" as *const u8 as *const i8,
        );
        if lua_getfield(state, -1, name) == 0 {
            lua_pushfstring(
                state,
                b"no field package.preload['%s']\0" as *const u8 as *const i8,
                name,
            );
            return 1;
        } else {
            lua_pushstring(state, b":preload:\0" as *const u8 as *const i8);
            return 2;
        };
    }
}
pub unsafe extern "C" fn findloader(state: *mut State, name: *const i8) {
    unsafe {
        let mut i: i32;
        let mut message = Buffer::new();
        if ((lua_getfield(
            state,
            -(1000000 as i32) - 1000 as i32 - 1,
            b"searchers\0" as *const u8 as *const i8,
        ) != 5) as i32
            != 0) as i64
            != 0
        {
            lual_error(
                state,
                b"'package.searchers' must be a table\0" as *const u8 as *const i8,
            );
        }
        message.lual_buffinit(state);
        i = 1;
        loop {
            message.lual_addstring(b"\n\t\0" as *const u8 as *const i8);
            if ((lua_rawgeti(state, 3, i as i64) == 0) as i32 != 0) as i64 != 0 {
                lua_settop(state, -2);
                message.length = (message.length as u64).wrapping_sub(2 as u64) as u64;
                message.lual_pushresult();
                lual_error(
                    state,
                    b"module '%s' not found:%s\0" as *const u8 as *const i8,
                    name,
                    lua_tolstring(state, -1, std::ptr::null_mut()),
                );
            }
            lua_pushstring(state, name);
            lua_callk(state, 1, 2, 0, None);
            if lua_type(state, -2) == Some(TAG_TYPE_CLOSURE) {
                return;
            } else if lua_isstring(state, -2) {
                lua_settop(state, -2);
                message.lual_addvalue();
            } else {
                lua_settop(state, -2 - 1);
                message.length = (message.length as u64).wrapping_sub(2 as u64) as u64;
            }
            i += 1;
        }
    }
}
pub unsafe extern "C" fn ll_require(state: *mut State) -> i32 {
    unsafe {
        let name: *const i8 = lual_checklstring(state, 1, std::ptr::null_mut());
        lua_settop(state, 1);
        lua_getfield(
            state,
            -(1000000 as i32) - 1000 as i32,
            b"_LOADED\0" as *const u8 as *const i8,
        );
        lua_getfield(state, 2, name);
        if lua_toboolean(state, -1) != 0 {
            return 1;
        }
        lua_settop(state, -2);
        findloader(state, name);
        lua_rotate(state, -2, 1);
        lua_pushvalue(state, 1);
        lua_pushvalue(state, -3);
        lua_callk(state, 2, 1, 0, None);
        if !(lua_type(state, -1) == Some(TAG_TYPE_NIL)) {
            lua_setfield(state, 2, name);
        } else {
            lua_settop(state, -2);
        }
        if lua_getfield(state, 2, name) == 0 {
            (*state).push_boolean(true);
            lua_copy(state, -1, -2);
            lua_setfield(state, 2, name);
        }
        lua_rotate(state, -2, 1);
        return 2;
    }
}
pub const PACKAGE_FUNCTIONS: [RegisteredFunction; 8] = {
    [
        {
            RegisteredFunction {
                name: b"loadlib\0" as *const u8 as *const i8,
                function: Some(ll_loadlib as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"searchpath\0" as *const u8 as *const i8,
                function: Some(ll_searchpath as unsafe extern "C" fn(*mut State) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"preload\0" as *const u8 as *const i8,
                function: None,
            }
        },
        {
            RegisteredFunction {
                name: b"cpath\0" as *const u8 as *const i8,
                function: None,
            }
        },
        {
            RegisteredFunction {
                name: b"path\0" as *const u8 as *const i8,
                function: None,
            }
        },
        {
            RegisteredFunction {
                name: b"searchers\0" as *const u8 as *const i8,
                function: None,
            }
        },
        {
            RegisteredFunction {
                name: b"loaded\0" as *const u8 as *const i8,
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
pub const LL_FUNCTIONS: [RegisteredFunction; 2] = {
    [
        {
            RegisteredFunction {
                name: b"require\0" as *const u8 as *const i8,
                function: Some(ll_require as unsafe extern "C" fn(*mut State) -> i32),
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
pub unsafe extern "C" fn createsearcherstable(state: *mut State) {
    unsafe {
        pub const SEARCHERS: [CFunction; 5] = {
            [
                Some(searcher_preload as unsafe extern "C" fn(*mut State) -> i32),
                Some(searcher_lua as unsafe extern "C" fn(*mut State) -> i32),
                Some(searcher_c as unsafe extern "C" fn(*mut State) -> i32),
                Some(searcher_croot as unsafe extern "C" fn(*mut State) -> i32),
                None,
            ]
        };
        let mut i: i32;
        (*state).lua_createtable();
        i = 0;
        while (SEARCHERS[i as usize]).is_some() {
            lua_pushvalue(state, -2);
            lua_pushcclosure(state, SEARCHERS[i as usize], 1);
            lua_rawseti(state, -2, (i + 1) as i64);
            i += 1;
        }
        lua_setfield(state, -2, b"searchers\0" as *const u8 as *const i8);
    }
}
pub unsafe extern "C" fn createclibstable(state: *mut State) {
    unsafe {
        lual_getsubtable(state, -(1000000 as i32) - 1000 as i32, CLIBS);
        (*state).lua_createtable();
        lua_pushcclosure(
            state,
            Some(gctm as unsafe extern "C" fn(*mut State) -> i32),
            0,
        );
        lua_setfield(state, -2, b"__gc\0" as *const u8 as *const i8);
        lua_setmetatable(state, -2);
    }
}
pub unsafe extern "C" fn luaopen_package(state: *mut State) -> i32 {
    unsafe {
        createclibstable(state);
        lual_checkversion_(
            state,
            504.0,
            (::core::mem::size_of::<i64>() as u64)
                .wrapping_mul(16 as u64)
                .wrapping_add(::core::mem::size_of::<f64>() as u64),
        );
        (*state).lua_createtable();
        lual_setfuncs(state, PACKAGE_FUNCTIONS.as_ptr(), 0);
        createsearcherstable(state);
        setpath(
        state,
        b"path\0" as *const u8 as *const i8,
        b"LUA_PATH\0" as *const u8 as *const i8,
        b"/usr/local/share/lua/5.4/?.lua;/usr/local/share/lua/5.4/?/init.lua;/usr/local/lib/lua/5.4/?.lua;/usr/local/lib/lua/5.4/?/init.lua;./?.lua;./?/init.lua\0"
            as *const u8 as *const i8,
    );
        setpath(
            state,
            b"cpath\0" as *const u8 as *const i8,
            b"LUA_CPATH\0" as *const u8 as *const i8,
            b"/usr/local/lib/lua/5.4/?.so;/usr/local/lib/lua/5.4/loadall.so;./?.so\0" as *const u8
                as *const i8,
        );
        lua_pushstring(state, b"/\n;\n?\n!\n-\n\0" as *const u8 as *const i8);
        lua_setfield(state, -2, b"config\0" as *const u8 as *const i8);
        lual_getsubtable(
            state,
            -(1000000 as i32) - 1000 as i32,
            b"_LOADED\0" as *const u8 as *const i8,
        );
        lua_setfield(state, -2, b"loaded\0" as *const u8 as *const i8);
        lual_getsubtable(
            state,
            -(1000000 as i32) - 1000 as i32,
            b"_PRELOAD\0" as *const u8 as *const i8,
        );
        lua_setfield(state, -2, b"preload\0" as *const u8 as *const i8);
        lua_rawgeti(state, -(1000000 as i32) - 1000 as i32, 2 as i64);
        lua_pushvalue(state, -2);
        lual_setfuncs(state, LL_FUNCTIONS.as_ptr(), 1);
        lua_settop(state, -2);
        return 1;
    }
}
