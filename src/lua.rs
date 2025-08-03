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
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type lua_State;
    pub type CallInfo;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> i32;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> i32;
    fn fgets(
        __s: *mut libc::c_char,
        __n: i32,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> i32;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> i32;
    fn strchr(_: *const libc::c_char, _: i32) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn sigemptyset(__set: *mut sigset_t) -> i32;
    fn sigaction(
        __sig: i32,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> i32;
    fn lua_close(L: *mut lua_State);
    fn lua_gettop(L: *mut lua_State) -> i32;
    fn lua_settop(L: *mut lua_State, index: i32);
    fn lua_rotate(L: *mut lua_State, index: i32, n: i32);
    fn lua_type(L: *mut lua_State, index: i32) -> i32;
    fn lua_typename(L: *mut lua_State, tp: i32) -> *const libc::c_char;
    fn lua_tointegerx(
        L: *mut lua_State,
        index: i32,
        isnum: *mut i32,
    ) -> Integer;
    fn lua_toboolean(L: *mut lua_State, index: i32) -> i32;
    fn lua_tolstring(
        L: *mut lua_State,
        index: i32,
        len: *mut size_t,
    ) -> *const libc::c_char;
    fn lua_touserdata(L: *mut lua_State, index: i32) -> *mut libc::c_void;
    fn lua_pushinteger(L: *mut lua_State, n: Integer);
    fn lua_pushlstring(
        L: *mut lua_State,
        s: *const libc::c_char,
        len: size_t,
    ) -> *const libc::c_char;
    fn lua_pushstring(L: *mut lua_State, s: *const libc::c_char) -> *const libc::c_char;
    fn lua_pushfstring(
        L: *mut lua_State,
        fmt: *const libc::c_char,
        _: ...
    ) -> *const libc::c_char;
    fn lua_pushcclosure(L: *mut lua_State, fn_0: CFunction, n: i32);
    fn lua_pushboolean(L: *mut lua_State, b: i32);
    fn lua_pushlightuserdata(L: *mut lua_State, p: *mut libc::c_void);
    fn lua_getglobal(L: *mut lua_State, name: *const libc::c_char) -> i32;
    fn lua_rawgeti(L: *mut lua_State, index: i32, n: Integer) -> i32;
    fn lua_createtable(L: *mut lua_State, narr: i32, nrec: i32);
    fn lua_setglobal(L: *mut lua_State, name: *const libc::c_char);
    fn lua_setfield(L: *mut lua_State, index: i32, k: *const libc::c_char);
    fn lua_rawseti(L: *mut lua_State, index: i32, n: Integer);
    fn lua_pcallk(
        L: *mut lua_State,
        nargs: i32,
        nresults: i32,
        errfunc: i32,
        ctx: lua_KContext,
        k: lua_KFunction,
    ) -> i32;
    fn lua_warning(L: *mut lua_State, msg: *const libc::c_char, tocont: i32);
    fn lua_gc(L: *mut lua_State, what: i32, _: ...) -> i32;
    fn lua_concat(L: *mut lua_State, n: i32);
    fn lua_sethook(
        L: *mut lua_State,
        func: lua_Hook,
        mask: i32,
        count: i32,
    );
    fn luaL_checkversion_(L: *mut lua_State, ver: Number, sz: size_t);
    fn luaL_callmeta(
        L: *mut lua_State,
        obj: i32,
        e: *const libc::c_char,
    ) -> i32;
    fn luaL_tolstring(
        L: *mut lua_State,
        index: i32,
        len: *mut size_t,
    ) -> *const libc::c_char;
    fn luaL_checkstack(L: *mut lua_State, sz: i32, msg: *const libc::c_char);
    fn luaL_error(L: *mut lua_State, fmt: *const libc::c_char, _: ...) -> i32;
    fn luaL_loadfilex(
        L: *mut lua_State,
        filename: *const libc::c_char,
        mode: *const libc::c_char,
    ) -> i32;
    fn luaL_loadbufferx(
        L: *mut lua_State,
        buff: *const libc::c_char,
        sz: size_t,
        name: *const libc::c_char,
        mode: *const libc::c_char,
    ) -> i32;
    fn luaL_newstate() -> *mut lua_State;
    fn luaL_len(L: *mut lua_State, index: i32) -> Integer;
    fn luaL_traceback(
        L: *mut lua_State,
        L1: *mut lua_State,
        msg: *const libc::c_char,
        level: i32,
    );
    fn luaL_openlibs(L: *mut lua_State);
    fn isatty(__fd: i32) -> i32;
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type __uid_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = i32;
pub type __clock_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: i32,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: i32,
    pub _flags2: i32,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: i32,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: i32,
    pub sival_ptr: *mut libc::c_void,
}
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: i32,
    pub si_errno: i32,
    pub si_code: i32,
    pub __pad0: i32,
    pub _sifields: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub _pad: [i32; 28],
    pub _kill: C2RustUnnamed_8,
    pub _timer: C2RustUnnamed_7,
    pub _rt: C2RustUnnamed_6,
    pub _sigchld: C2RustUnnamed_5,
    pub _sigfault: C2RustUnnamed_2,
    pub _sigpoll: C2RustUnnamed_1,
    pub _sigsys: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: i32,
    pub _arch: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub si_band: libc::c_long,
    pub si_fd: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub _addr_bnd: C2RustUnnamed_4,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: i32,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub si_tid: i32,
    pub si_overrun: i32,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option::<unsafe extern "C" fn(i32) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_9,
    pub sa_mask: __sigset_t,
    pub sa_flags: i32,
    pub sa_restorer: Option::<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option::<
        unsafe extern "C" fn(i32, *mut siginfo_t, *mut libc::c_void) -> (),
    >,
}
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
pub type lua_Hook = Option::<unsafe extern "C" fn(*mut lua_State, *mut lua_Debug) -> ()>;
static mut globalL: *mut lua_State = 0 as *const lua_State as *mut lua_State;
static mut progname: *const libc::c_char = b"lua\0" as *const u8 as *const libc::c_char;
unsafe extern "C" fn setsignal(
    mut sig: i32,
    mut handler: Option::<unsafe extern "C" fn(i32) -> ()>,
) {
    let mut sa: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_9 {
            sa_handler: None,
        },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    sa.__sigaction_handler.sa_handler = handler;
    sa.sa_flags = 0i32;
    sigemptyset(&mut sa.sa_mask);
    sigaction(sig, &mut sa, 0 as *mut sigaction);
}
unsafe extern "C" fn lstop(mut L: *mut lua_State, mut _ar: *mut lua_Debug) {
    lua_sethook(L, None, 0i32, 0i32);
    luaL_error(L, b"interrupted!\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn laction(mut i: i32) {
    let mut flag: i32 = (1i32) << 0i32
        | (1i32) << 1i32 | (1i32) << 2i32
        | (1i32) << 3i32;
    setsignal(i, None);
    lua_sethook(
        globalL,
        Some(lstop as unsafe extern "C" fn(*mut lua_State, *mut lua_Debug) -> ()),
        flag,
        1i32,
    );
}
unsafe extern "C" fn print_usage(mut badoption: *const libc::c_char) {
    fprintf(stderr, b"%s: \0" as *const u8 as *const libc::c_char, progname);
    fflush(stderr);
    if *badoption.offset(1i32 as isize) as i32 == 'e' as i32
        || *badoption.offset(1i32 as isize) as i32 == 'l' as i32
    {
        fprintf(
            stderr,
            b"'%s' needs argument\n\0" as *const u8 as *const libc::c_char,
            badoption,
        );
        fflush(stderr);
    } else {
        fprintf(
            stderr,
            b"unrecognized option '%s'\n\0" as *const u8 as *const libc::c_char,
            badoption,
        );
        fflush(stderr);
    }
    fprintf(
        stderr,
        b"usage: %s [options] [script [args]]\nAvailable options are:\n  -e stat   execute string 'stat'\n  -i        enter interactive mode after executing 'script'\n  -l mod    require library 'mod' into global 'mod'\n  -l g=mod  require library 'mod' into global 'g'\n  -v        show version information\n  -E        ignore environment variables\n  -W        turn warnings on\n  --        stop handling options\n  -         stop handling options and execute stdin\n\0"
            as *const u8 as *const libc::c_char,
        progname,
    );
    fflush(stderr);
}
unsafe extern "C" fn l_message(
    mut pname: *const libc::c_char,
    mut msg: *const libc::c_char,
) {
    if !pname.is_null() {
        fprintf(stderr, b"%s: \0" as *const u8 as *const libc::c_char, pname);
        fflush(stderr);
    }
    fprintf(stderr, b"%s\n\0" as *const u8 as *const libc::c_char, msg);
    fflush(stderr);
}
unsafe extern "C" fn report(
    mut L: *mut lua_State,
    mut status: i32,
) -> i32 {
    if status != 0i32 {
        let mut msg: *const libc::c_char = lua_tolstring(
            L,
            -(1i32),
            0 as *mut size_t,
        );
        if msg.is_null() {
            msg = b"(error message not a string)\0" as *const u8 as *const libc::c_char;
        }
        l_message(progname, msg);
        lua_settop(L, -(1i32) - 1i32);
    }
    return status;
}
unsafe extern "C" fn msghandler(mut L: *mut lua_State) -> i32 {
    let mut msg: *const libc::c_char = lua_tolstring(
        L,
        1i32,
        0 as *mut size_t,
    );
    if msg.is_null() {
        if luaL_callmeta(
            L,
            1i32,
            b"__tostring\0" as *const u8 as *const libc::c_char,
        ) != 0 && lua_type(L, -(1i32)) == 4i32
        {
            return 1i32
        } else {
            msg = lua_pushfstring(
                L,
                b"(error object is a %s value)\0" as *const u8 as *const libc::c_char,
                lua_typename(L, lua_type(L, 1i32)),
            );
        }
    }
    luaL_traceback(L, L, msg, 1i32);
    return 1i32;
}
unsafe extern "C" fn docall(
    mut L: *mut lua_State,
    mut narg: i32,
    mut nres: i32,
) -> i32 {
    let mut status: i32 = 0;
    let mut base: i32 = lua_gettop(L) - narg;
    lua_pushcclosure(
        L,
        Some(msghandler as unsafe extern "C" fn(*mut lua_State) -> i32),
        0i32,
    );
    lua_rotate(L, base, 1i32);
    globalL = L;
    setsignal(
        2i32,
        Some(laction as unsafe extern "C" fn(i32) -> ()),
    );
    status = lua_pcallk(L, narg, nres, base, 0i32 as lua_KContext, None);
    setsignal(2i32, None);
    lua_rotate(L, base, -(1i32));
    lua_settop(L, -(1i32) - 1i32);
    return status;
}
unsafe extern "C" fn print_version() {
    fwrite(
        b"Lua 5.4.8  Copyright (C) 1994-2025 Lua.org, PUC-Rio\0" as *const u8
            as *const libc::c_char as *const libc::c_void,
        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
        strlen(
            b"Lua 5.4.8  Copyright (C) 1994-2025 Lua.org, PUC-Rio\0" as *const u8
                as *const libc::c_char,
        ),
        stdout,
    );
    fwrite(
        b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
        1i32 as libc::c_ulong,
        stdout,
    );
    fflush(stdout);
}
unsafe extern "C" fn createargtable(
    mut L: *mut lua_State,
    mut argv: *mut *mut libc::c_char,
    mut argc: i32,
    mut script: i32,
) {
    let mut i: i32 = 0;
    let mut narg: i32 = 0;
    narg = argc - (script + 1i32);
    lua_createtable(L, narg, script + 1i32);
    i = 0i32;
    while i < argc {
        lua_pushstring(L, *argv.offset(i as isize));
        lua_rawseti(L, -(2i32), (i - script) as Integer);
        i += 1;

    }
    lua_setglobal(L, b"arg\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn dochunk(
    mut L: *mut lua_State,
    mut status: i32,
) -> i32 {
    if status == 0i32 {
        status = docall(L, 0i32, 0i32);
    }
    return report(L, status);
}
unsafe extern "C" fn dofile(
    mut L: *mut lua_State,
    mut name: *const libc::c_char,
) -> i32 {
    return dochunk(L, luaL_loadfilex(L, name, 0 as *const libc::c_char));
}
unsafe extern "C" fn dostring(
    mut L: *mut lua_State,
    mut s: *const libc::c_char,
    mut name: *const libc::c_char,
) -> i32 {
    return dochunk(L, luaL_loadbufferx(L, s, strlen(s), name, 0 as *const libc::c_char));
}
unsafe extern "C" fn dolibrary(
    mut L: *mut lua_State,
    mut globname: *mut libc::c_char,
) -> i32 {
    let mut status: i32 = 0;
    let mut suffix: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut modname: *mut libc::c_char = strchr(globname, '=' as i32);
    if modname.is_null() {
        modname = globname;
        suffix = strchr(
            modname,
            *(b"-\0" as *const u8 as *const libc::c_char) as i32,
        );
    } else {
        *modname = '\0' as i32 as libc::c_char;
        modname = modname.offset(1);
    }
    lua_getglobal(L, b"require\0" as *const u8 as *const libc::c_char);
    lua_pushstring(L, modname);
    status = docall(L, 1i32, 1i32);
    if status == 0i32 {
        if !suffix.is_null() {
            *suffix = '\0' as i32 as libc::c_char;
        }
        lua_setglobal(L, globname);
    }
    return report(L, status);
}
unsafe extern "C" fn pushargs(mut L: *mut lua_State) -> i32 {
    let mut i: i32 = 0;
    let mut n: i32 = 0;
    if lua_getglobal(L, b"arg\0" as *const u8 as *const libc::c_char) != 5i32
    {
        luaL_error(L, b"'arg' is not a table\0" as *const u8 as *const libc::c_char);
    }
    n = luaL_len(L, -(1i32)) as i32;
    luaL_checkstack(
        L,
        n + 3i32,
        b"too many arguments to script\0" as *const u8 as *const libc::c_char,
    );
    i = 1i32;
    while i <= n {
        lua_rawgeti(L, -i, i as Integer);
        i += 1;

    }
    lua_rotate(L, -i, -(1i32));
    lua_settop(L, -(1i32) - 1i32);
    return n;
}
unsafe extern "C" fn handle_script(
    mut L: *mut lua_State,
    mut argv: *mut *mut libc::c_char,
) -> i32 {
    let mut status: i32 = 0;
    let mut fname: *const libc::c_char = *argv.offset(0i32 as isize);
    if strcmp(fname, b"-\0" as *const u8 as *const libc::c_char) == 0i32
        && strcmp(
            *argv.offset(-(1i32) as isize),
            b"--\0" as *const u8 as *const libc::c_char,
        ) != 0i32
    {
        fname = 0 as *const libc::c_char;
    }
    status = luaL_loadfilex(L, fname, 0 as *const libc::c_char);
    if status == 0i32 {
        let mut n: i32 = pushargs(L);
        status = docall(L, n, -(1i32));
    }
    return report(L, status);
}
unsafe extern "C" fn collectargs(
    mut argv: *mut *mut libc::c_char,
    mut first: *mut i32,
) -> i32 {
    let mut args: i32 = 0i32;
    let mut i: i32 = 0;
    if !(*argv.offset(0i32 as isize)).is_null() {
        if *(*argv.offset(0i32 as isize)).offset(0i32 as isize)
            != 0
        {
            progname = *argv.offset(0i32 as isize);
        }
    } else {
        *first = -(1i32);
        return 0i32;
    }
    i = 1i32;
    while !(*argv.offset(i as isize)).is_null() {
        *first = i;
        if *(*argv.offset(i as isize)).offset(0i32 as isize) as i32
            != '-' as i32
        {
            return args;
        }
        let mut current_block_31: u64;
        match *(*argv.offset(i as isize)).offset(1i32 as isize)
            as i32
        {
            45 => {
                if *(*argv.offset(i as isize)).offset(2i32 as isize)
                    as i32 != '\0' as i32
                {
                    return 1i32;
                }
                *first = i + 1i32;
                return args;
            }
            0 => return args,
            69 => {
                if *(*argv.offset(i as isize)).offset(2i32 as isize)
                    as i32 != '\0' as i32
                {
                    return 1i32;
                }
                args |= 16i32;
                current_block_31 = 4761528863920922185;
            }
            87 => {
                if *(*argv.offset(i as isize)).offset(2i32 as isize)
                    as i32 != '\0' as i32
                {
                    return 1i32;
                }
                current_block_31 = 4761528863920922185;
            }
            105 => {
                args |= 2i32;
                current_block_31 = 1195002048942084387;
            }
            118 => {
                current_block_31 = 1195002048942084387;
            }
            101 => {
                args |= 8i32;
                current_block_31 = 8308901264692237116;
            }
            108 => {
                current_block_31 = 8308901264692237116;
            }
            _ => return 1i32,
        }
        match current_block_31 {
            1195002048942084387 => {
                if *(*argv.offset(i as isize)).offset(2i32 as isize)
                    as i32 != '\0' as i32
                {
                    return 1i32;
                }
                args |= 4i32;
            }
            8308901264692237116 => {
                if *(*argv.offset(i as isize)).offset(2i32 as isize)
                    as i32 == '\0' as i32
                {
                    i += 1;

                    if (*argv.offset(i as isize)).is_null()
                        || *(*argv.offset(i as isize)).offset(0i32 as isize)
                            as i32 == '-' as i32
                    {
                        return 1i32;
                    }
                }
            }
            _ => {}
        }
        i += 1;

    }
    *first = 0i32;
    return args;
}
unsafe extern "C" fn runargs(
    mut L: *mut lua_State,
    mut argv: *mut *mut libc::c_char,
    mut n: i32,
) -> i32 {
    let mut i: i32 = 0;
    i = 1i32;
    while i < n {
        let mut option: i32 = *(*argv.offset(i as isize))
            .offset(1i32 as isize) as i32;
        match option {
            101 | 108 => {
                let mut status: i32 = 0;
                let mut extra: *mut libc::c_char = (*argv.offset(i as isize))
                    .offset(2i32 as isize);
                if *extra as i32 == '\0' as i32 {
                    i += 1;
                    extra = *argv.offset(i as isize);
                }
                status = if option == 'e' as i32 {
                    dostring(
                        L,
                        extra,
                        b"=(command line)\0" as *const u8 as *const libc::c_char,
                    )
                } else {
                    dolibrary(L, extra)
                };
                if status != 0i32 {
                    return 0i32;
                }
            }
            87 => {
                lua_warning(
                    L,
                    b"@on\0" as *const u8 as *const libc::c_char,
                    0i32,
                );
            }
            _ => {}
        }
        i += 1;

    }
    return 1i32;
}
unsafe extern "C" fn handle_luainit(mut L: *mut lua_State) -> i32 {
    let mut name: *const libc::c_char = b"=LUA_INIT_5_4\0" as *const u8
        as *const libc::c_char;
    let mut init: *const libc::c_char = getenv(name.offset(1i32 as isize));
    if init.is_null() {
        name = b"=LUA_INIT\0" as *const u8 as *const libc::c_char;
        init = getenv(name.offset(1i32 as isize));
    }
    if init.is_null() {
        return 0i32
    } else if *init.offset(0i32 as isize) as i32 == '@' as i32 {
        return dofile(L, init.offset(1i32 as isize))
    } else {
        return dostring(L, init, name)
    };
}
unsafe extern "C" fn get_prompt(
    mut L: *mut lua_State,
    mut firstline: i32,
) -> *const libc::c_char {
    if lua_getglobal(
        L,
        if firstline != 0 {
            b"_PROMPT\0" as *const u8 as *const libc::c_char
        } else {
            b"_PROMPT2\0" as *const u8 as *const libc::c_char
        },
    ) == 0i32
    {
        return if firstline != 0 {
            b"> \0" as *const u8 as *const libc::c_char
        } else {
            b">> \0" as *const u8 as *const libc::c_char
        }
    } else {
        let mut p: *const libc::c_char = luaL_tolstring(
            L,
            -(1i32),
            0 as *mut size_t,
        );
        lua_rotate(L, -(2i32), -(1i32));
        lua_settop(L, -(1i32) - 1i32);
        return p;
    };
}
unsafe extern "C" fn incomplete(
    mut L: *mut lua_State,
    mut status: i32,
) -> i32 {
    if status == 3i32 {
        let mut lmsg: size_t = 0;
        let mut msg: *const libc::c_char = lua_tolstring(
            L,
            -(1i32),
            &mut lmsg,
        );
        if lmsg
            >= (::core::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_sub(1i32 as libc::c_ulong)
            && strcmp(
                msg
                    .offset(lmsg as isize)
                    .offset(
                        -((::core::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                            )
                            .wrapping_sub(1i32 as libc::c_ulong) as isize),
                    ),
                b"<eof>\0" as *const u8 as *const libc::c_char,
            ) == 0i32
        {
            return 1i32;
        }
    }
    return 0i32;
}
unsafe extern "C" fn pushline(
    mut L: *mut lua_State,
    mut firstline: i32,
) -> i32 {
    let mut buffer: [libc::c_char; 512] = [0; 512];
    let mut b: *mut libc::c_char = buffer.as_mut_ptr();
    let mut l: size_t = 0;
    let mut prmt: *const libc::c_char = get_prompt(L, firstline);
    fputs(prmt, stdout);
    fflush(stdout);
    let mut readstatus: i32 = (fgets(b, 512i32, stdin)
        != 0 as *mut libc::c_void as *mut libc::c_char) as i32;
    lua_settop(L, -(1i32) - 1i32);
    if readstatus == 0i32 {
        return 0i32;
    }
    l = strlen(b);
    if l > 0i32 as libc::c_ulong
        && *b.offset(l.wrapping_sub(1i32 as libc::c_ulong) as isize)
            as i32 == '\n' as i32
    {
        l = l.wrapping_sub(1);
        *b.offset(l as isize) = '\0' as i32 as libc::c_char;
    }
    if firstline != 0
        && *b.offset(0i32 as isize) as i32 == '=' as i32
    {
        lua_pushfstring(
            L,
            b"return %s\0" as *const u8 as *const libc::c_char,
            b.offset(1i32 as isize),
        );
    } else {
        lua_pushlstring(L, b, l);
    }
    return 1i32;
}
unsafe extern "C" fn addreturn(mut L: *mut lua_State) -> i32 {
    let mut line: *const libc::c_char = lua_tolstring(
        L,
        -(1i32),
        0 as *mut size_t,
    );
    let mut retline: *const libc::c_char = lua_pushfstring(
        L,
        b"return %s;\0" as *const u8 as *const libc::c_char,
        line,
    );
    let mut status: i32 = luaL_loadbufferx(
        L,
        retline,
        strlen(retline),
        b"=stdin\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    );
    if status == 0i32 {
        lua_rotate(L, -(2i32), -(1i32));
        lua_settop(L, -(1i32) - 1i32);
        *line.offset(0i32 as isize) as i32 != '\0' as i32;
    } else {
        lua_settop(L, -(2i32) - 1i32);
    }
    return status;
}
unsafe extern "C" fn multiline(mut L: *mut lua_State) -> i32 {
    loop {
        let mut len: size_t = 0;
        let mut line: *const libc::c_char = lua_tolstring(L, 1i32, &mut len);
        let mut status: i32 = luaL_loadbufferx(
            L,
            line,
            len,
            b"=stdin\0" as *const u8 as *const libc::c_char,
            0 as *const libc::c_char,
        );
        if incomplete(L, status) == 0 || pushline(L, 0i32) == 0 {
            return status;
        }
        lua_rotate(L, -(2i32), -(1i32));
        lua_settop(L, -(1i32) - 1i32);
        lua_pushstring(L, b"\n\0" as *const u8 as *const libc::c_char);
        lua_rotate(L, -(2i32), 1i32);
        lua_concat(L, 3i32);
    };
}
unsafe extern "C" fn loadline(mut L: *mut lua_State) -> i32 {
    let mut status: i32 = 0;
    lua_settop(L, 0i32);
    if pushline(L, 1i32) == 0 {
        return -(1i32);
    }
    status = addreturn(L);
    if status != 0i32 {
        status = multiline(L);
    }
    lua_rotate(L, 1i32, -(1i32));
    lua_settop(L, -(1i32) - 1i32);
    return status;
}
unsafe extern "C" fn l_print(mut L: *mut lua_State) {
    let mut n: i32 = lua_gettop(L);
    if n > 0i32 {
        luaL_checkstack(
            L,
            20i32,
            b"too many results to print\0" as *const u8 as *const libc::c_char,
        );
        lua_getglobal(L, b"print\0" as *const u8 as *const libc::c_char);
        lua_rotate(L, 1i32, 1i32);
        if lua_pcallk(
            L,
            n,
            0i32,
            0i32,
            0i32 as lua_KContext,
            None,
        ) != 0i32
        {
            l_message(
                progname,
                lua_pushfstring(
                    L,
                    b"error calling 'print' (%s)\0" as *const u8 as *const libc::c_char,
                    lua_tolstring(L, -(1i32), 0 as *mut size_t),
                ),
            );
        }
    }
}
unsafe extern "C" fn doREPL(mut L: *mut lua_State) {
    let mut status: i32 = 0;
    let mut oldprogname: *const libc::c_char = progname;
    progname = 0 as *const libc::c_char;
    loop {
        status = loadline(L);
        if !(status != -(1i32)) {
            break;
        }
        if status == 0i32 {
            status = docall(L, 0i32, -(1i32));
        }
        if status == 0i32 {
            l_print(L);
        } else {
            report(L, status);
        }
    }
    lua_settop(L, 0i32);
    fwrite(
        b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
        1i32 as libc::c_ulong,
        stdout,
    );
    fflush(stdout);
    progname = oldprogname;
}
unsafe extern "C" fn pmain(mut L: *mut lua_State) -> i32 {
    let mut argc: i32 = lua_tointegerx(
        L,
        1i32,
        0 as *mut i32,
    ) as i32;
    let mut argv: *mut *mut libc::c_char = lua_touserdata(L, 2i32)
        as *mut *mut libc::c_char;
    let mut script: i32 = 0;
    let mut args: i32 = collectargs(argv, &mut script);
    let mut optlim: i32 = if script > 0i32 { script } else { argc };
    luaL_checkversion_(
        L,
        504i32 as Number,
        (::core::mem::size_of::<Integer>() as libc::c_ulong)
            .wrapping_mul(16i32 as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<Number>() as libc::c_ulong),
    );
    if args == 1i32 {
        print_usage(*argv.offset(script as isize));
        return 0i32;
    }
    if args & 4i32 != 0 {
        print_version();
    }
    if args & 16i32 != 0 {
        lua_pushboolean(L, 1i32);
        lua_setfield(
            L,
            -(1000000i32) - 1000i32,
            b"LUA_NOENV\0" as *const u8 as *const libc::c_char,
        );
    }
    luaL_openlibs(L);
    createargtable(L, argv, argc, script);
    lua_gc(L, 1i32);
    lua_gc(L, 10i32, 0i32, 0i32);
    if args & 16i32 == 0 {
        if handle_luainit(L) != 0i32 {
            return 0i32;
        }
    }
    if runargs(L, argv, optlim) == 0 {
        return 0i32;
    }
    if script > 0i32 {
        if handle_script(L, argv.offset(script as isize)) != 0i32 {
            return 0i32;
        }
    }
    if args & 2i32 != 0 {
        doREPL(L);
    } else if script < 1i32
        && args & (8i32 | 4i32) == 0
    {
        if isatty(0i32) != 0 {
            print_version();
            doREPL(L);
        } else {
            dofile(L, 0 as *const libc::c_char);
        }
    }
    lua_pushboolean(L, 1i32);
    return 1i32;
}
pub unsafe fn main_0(
    mut argc: i32,
    mut argv: *mut *mut libc::c_char,
) -> i32 {
    let mut status: i32 = 0;
    let mut result: i32 = 0;
    let mut L: *mut lua_State = luaL_newstate();
    if L.is_null() {
        l_message(
            *argv.offset(0i32 as isize),
            b"cannot create state: not enough memory\0" as *const u8
                as *const libc::c_char,
        );
        return 1i32;
    }
    lua_gc(L, 0i32);
    lua_pushcclosure(
        L,
        Some(pmain as unsafe extern "C" fn(*mut lua_State) -> i32),
        0i32,
    );
    lua_pushinteger(L, argc as Integer);
    lua_pushlightuserdata(L, argv as *mut libc::c_void);
    status = lua_pcallk(
        L,
        2i32,
        1i32,
        0i32,
        0i32 as lua_KContext,
        None,
    );
    result = lua_toboolean(L, -(1i32));
    report(L, status);
    lua_close(L);
    return if result != 0 && status == 0i32 {
        0i32
    } else {
        1i32
    };
}
