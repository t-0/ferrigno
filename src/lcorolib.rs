#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use crate::types::{Integer,Number};
unsafe extern "C" {
    pub type lua_State;
    pub type CallInfo;
    fn lua_newthread(L: *mut lua_State) -> *mut lua_State;
    fn lua_closethread(L: *mut lua_State, from: *mut lua_State) -> i32;
    fn lua_gettop(L: *mut lua_State) -> i32;
    fn lua_settop(L: *mut lua_State, index: i32);
    fn lua_pushvalue(L: *mut lua_State, index: i32);
    fn lua_rotate(L: *mut lua_State, index: i32, n: i32);
    fn lua_checkstack(L: *mut lua_State, n: i32) -> i32;
    fn lua_xmove(from: *mut lua_State, to: *mut lua_State, n: i32);
    fn lua_type(L: *mut lua_State, index: i32) -> i32;
    fn lua_tothread(L: *mut lua_State, index: i32) -> *mut lua_State;
    fn lua_pushstring(L: *mut lua_State, s: *const libc::c_char) -> *const libc::c_char;
    fn lua_pushcclosure(L: *mut lua_State, fn_0: CFunction, n: i32);
    fn lua_pushboolean(L: *mut lua_State, b: i32);
    fn lua_pushthread(L: *mut lua_State) -> i32;
    fn lua_createtable(L: *mut lua_State, narr: i32, nrec: i32);
    fn lua_yieldk(
        L: *mut lua_State,
        nresults: i32,
        ctx: lua_KContext,
        k: lua_KFunction,
    ) -> i32;
    fn lua_resume(
        L: *mut lua_State,
        from: *mut lua_State,
        narg: i32,
        nres: *mut i32,
    ) -> i32;
    fn lua_status(L: *mut lua_State) -> i32;
    fn lua_isyieldable(L: *mut lua_State) -> i32;
    fn lua_error(L: *mut lua_State) -> i32;
    fn lua_concat(L: *mut lua_State, n: i32);
    fn lua_getstack(
        L: *mut lua_State,
        level: i32,
        ar: *mut lua_Debug,
    ) -> i32;
    fn luaL_checkversion_(L: *mut lua_State, ver: Number, sz: size_t);
    fn luaL_typeerror(
        L: *mut lua_State,
        arg: i32,
        tname: *const libc::c_char,
    ) -> i32;
    fn luaL_checktype(L: *mut lua_State, arg: i32, t: i32);
    fn luaL_where(L: *mut lua_State, lvl: i32);
    fn luaL_error(L: *mut lua_State, fmt: *const libc::c_char, _: ...) -> i32;
    fn luaL_setfuncs(L: *mut lua_State, l: *const luaL_Reg, nup: i32);
}
pub type size_t = libc::c_ulong;
pub type intptr_t = libc::c_long;

pub type lua_KContext = intptr_t;
pub type CFunction = Option::<unsafe extern "C" fn(*mut lua_State) -> i32>;
pub type lua_KFunction = Option::<
    unsafe extern "C" fn(*mut lua_State, i32, lua_KContext) -> i32,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lua_Debug {
    pub event: i32,
    pub name: *const libc::c_char,
    pub namewhat: *const libc::c_char,
    pub what: *const libc::c_char,
    pub source: *const libc::c_char,
    pub srclen: size_t,
    pub currentline: i32,
    pub linedefined: i32,
    pub lastlinedefined: i32,
    pub nups: u8,
    pub nparams: u8,
    pub isvararg: libc::c_char,
    pub istailcall: libc::c_char,
    pub ftransfer: libc::c_ushort,
    pub ntransfer: libc::c_ushort,
    pub short_src: [libc::c_char; 60],
    pub i_ci: *mut CallInfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaL_Reg {
    pub name: *const libc::c_char,
    pub func: CFunction,
}
unsafe extern "C" fn getco(mut L: *mut lua_State) -> *mut lua_State {
    let mut co: *mut lua_State = lua_tothread(L, 1i32);
    ((co != 0 as *mut lua_State) as i32 as libc::c_long != 0
        || luaL_typeerror(
            L,
            1i32,
            b"thread\0" as *const u8 as *const libc::c_char,
        ) != 0) as i32;
    return co;
}
unsafe extern "C" fn auxresume(
    mut L: *mut lua_State,
    mut co: *mut lua_State,
    mut narg: i32,
) -> i32 {
    let mut status: i32 = 0;
    let mut nres: i32 = 0;
    if ((lua_checkstack(co, narg) == 0) as i32 != 0i32)
        as i32 as libc::c_long != 0
    {
        lua_pushstring(
            L,
            b"too many arguments to resume\0" as *const u8 as *const libc::c_char,
        );
        return -(1i32);
    }
    lua_xmove(L, co, narg);
    status = lua_resume(co, L, narg, &mut nres);
    if ((status == 0i32 || status == 1i32) as i32
        != 0i32) as i32 as libc::c_long != 0
    {
        if ((lua_checkstack(L, nres + 1i32) == 0) as i32
            != 0i32) as i32 as libc::c_long != 0
        {
            lua_settop(co, -nres - 1i32);
            lua_pushstring(
                L,
                b"too many results to resume\0" as *const u8 as *const libc::c_char,
            );
            return -(1i32);
        }
        lua_xmove(co, L, nres);
        return nres;
    } else {
        lua_xmove(co, L, 1i32);
        return -(1i32);
    };
}
unsafe extern "C" fn luaB_coresume(mut L: *mut lua_State) -> i32 {
    let mut co: *mut lua_State = getco(L);
    let mut r: i32 = 0;
    r = auxresume(L, co, lua_gettop(L) - 1i32);
    if ((r < 0i32) as i32 != 0i32) as i32
        as libc::c_long != 0
    {
        lua_pushboolean(L, 0i32);
        lua_rotate(L, -(2i32), 1i32);
        return 2i32;
    } else {
        lua_pushboolean(L, 1i32);
        lua_rotate(L, -(r + 1i32), 1i32);
        return r + 1i32;
    };
}
unsafe extern "C" fn luaB_auxwrap(mut L: *mut lua_State) -> i32 {
    let mut co: *mut lua_State = lua_tothread(
        L,
        -(1000000i32) - 1000i32 - 1i32,
    );
    let mut r: i32 = auxresume(L, co, lua_gettop(L));
    if ((r < 0i32) as i32 != 0i32) as i32
        as libc::c_long != 0
    {
        let mut stat: i32 = lua_status(co);
        if stat != 0i32 && stat != 1i32 {
            stat = lua_closethread(co, L);
            lua_xmove(co, L, 1i32);
        }
        if stat != 4i32
            && lua_type(L, -(1i32)) == 4i32
        {
            luaL_where(L, 1i32);
            lua_rotate(L, -(2i32), 1i32);
            lua_concat(L, 2i32);
        }
        return lua_error(L);
    }
    return r;
}
unsafe extern "C" fn luaB_cocreate(mut L: *mut lua_State) -> i32 {
    let mut NL: *mut lua_State = 0 as *mut lua_State;
    luaL_checktype(L, 1i32, 6i32);
    NL = lua_newthread(L);
    lua_pushvalue(L, 1i32);
    lua_xmove(L, NL, 1i32);
    return 1i32;
}
unsafe extern "C" fn luaB_cowrap(mut L: *mut lua_State) -> i32 {
    luaB_cocreate(L);
    lua_pushcclosure(
        L,
        Some(luaB_auxwrap as unsafe extern "C" fn(*mut lua_State) -> i32),
        1i32,
    );
    return 1i32;
}
unsafe extern "C" fn luaB_yield(mut L: *mut lua_State) -> i32 {
    return lua_yieldk(L, lua_gettop(L), 0i32 as lua_KContext, None);
}
static mut statname: [*const libc::c_char; 4] = [
    b"running\0" as *const u8 as *const libc::c_char,
    b"dead\0" as *const u8 as *const libc::c_char,
    b"suspended\0" as *const u8 as *const libc::c_char,
    b"normal\0" as *const u8 as *const libc::c_char,
];
unsafe extern "C" fn auxstatus(
    mut L: *mut lua_State,
    mut co: *mut lua_State,
) -> i32 {
    if L == co {
        return 0i32
    } else {
        match lua_status(co) {
            1 => return 2i32,
            0 => {
                let mut ar: lua_Debug = lua_Debug {
                    event: 0,
                    name: 0 as *const libc::c_char,
                    namewhat: 0 as *const libc::c_char,
                    what: 0 as *const libc::c_char,
                    source: 0 as *const libc::c_char,
                    srclen: 0,
                    currentline: 0,
                    linedefined: 0,
                    lastlinedefined: 0,
                    nups: 0,
                    nparams: 0,
                    isvararg: 0,
                    istailcall: 0,
                    ftransfer: 0,
                    ntransfer: 0,
                    short_src: [0; 60],
                    i_ci: 0 as *mut CallInfo,
                };
                if lua_getstack(co, 0i32, &mut ar) != 0 {
                    return 3i32
                } else if lua_gettop(co) == 0i32 {
                    return 1i32
                } else {
                    return 2i32
                }
            }
            _ => return 1i32,
        }
    };
}
unsafe extern "C" fn luaB_costatus(mut L: *mut lua_State) -> i32 {
    let mut co: *mut lua_State = getco(L);
    lua_pushstring(L, statname[auxstatus(L, co) as usize]);
    return 1i32;
}
unsafe extern "C" fn luaB_yieldable(mut L: *mut lua_State) -> i32 {
    let mut co: *mut lua_State = if lua_type(L, 1i32) == -(1i32)
    {
        L
    } else {
        getco(L)
    };
    lua_pushboolean(L, lua_isyieldable(co));
    return 1i32;
}
unsafe extern "C" fn luaB_corunning(mut L: *mut lua_State) -> i32 {
    let mut ismain: i32 = lua_pushthread(L);
    lua_pushboolean(L, ismain);
    return 2i32;
}
unsafe extern "C" fn luaB_close(mut L: *mut lua_State) -> i32 {
    let mut co: *mut lua_State = getco(L);
    let mut status: i32 = auxstatus(L, co);
    match status {
        1 | 2 => {
            status = lua_closethread(co, L);
            if status == 0i32 {
                lua_pushboolean(L, 1i32);
                return 1i32;
            } else {
                lua_pushboolean(L, 0i32);
                lua_xmove(co, L, 1i32);
                return 2i32;
            }
        }
        _ => {
            return luaL_error(
                L,
                b"cannot close a %s coroutine\0" as *const u8 as *const libc::c_char,
                statname[status as usize],
            );
        }
    };
}
static mut co_funcs: [luaL_Reg; 9] = {
    [
        {
            let mut init = luaL_Reg {
                name: b"create\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_cocreate as unsafe extern "C" fn(*mut lua_State) -> i32,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"resume\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_coresume as unsafe extern "C" fn(*mut lua_State) -> i32,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"running\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_corunning as unsafe extern "C" fn(*mut lua_State) -> i32,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"status\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_costatus as unsafe extern "C" fn(*mut lua_State) -> i32,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"wrap\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_cowrap as unsafe extern "C" fn(*mut lua_State) -> i32,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"yield\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_yield as unsafe extern "C" fn(*mut lua_State) -> i32,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"isyieldable\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_yieldable as unsafe extern "C" fn(*mut lua_State) -> i32,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"close\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_close as unsafe extern "C" fn(*mut lua_State) -> i32,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: 0 as *const libc::c_char,
                func: None,
            };
            init
        },
    ]
};
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaopen_coroutine(mut L: *mut lua_State) -> i32 {
    luaL_checkversion_(
        L,
        504i32 as Number,
        (::core::mem::size_of::<Integer>() as libc::c_ulong)
            .wrapping_mul(16i32 as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<Number>() as libc::c_ulong),
    );
    lua_createtable(
        L,
        0i32,
        (::core::mem::size_of::<[luaL_Reg; 9]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<luaL_Reg>() as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong) as i32,
    );
    luaL_setfuncs(L, co_funcs.as_ptr(), 0i32);
    return 1i32;
}
