#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use crate::types::*;
unsafe extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type lua_State;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __errno_location() -> *mut i32;
    fn localeconv() -> *mut lconv;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> i32;
    fn tmpfile() -> *mut FILE;
    fn fflush(__stream: *mut FILE) -> i32;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn setvbuf(__stream: *mut FILE, __buf: *mut libc::c_char, __modes: i32, __n: u64) -> i32;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> i32;
    fn getc(__stream: *mut FILE) -> i32;
    fn ungetc(__c: i32, __stream: *mut FILE) -> i32;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fseeko(__stream: *mut FILE, __off: Offset, __whence: i32) -> i32;
    fn ftello(__stream: *mut FILE) -> Offset;
    fn clearerr(__stream: *mut FILE);
    fn ferror(__stream: *mut FILE) -> i32;
    fn pclose(__stream: *mut FILE) -> i32;
    fn popen(__command: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
    fn flockfile(__stream: *mut FILE);
    fn funlockfile(__stream: *mut FILE);
    fn __uflow(_: *mut FILE) -> i32;
    fn strchr(_: *const libc::c_char, _: i32) -> *mut libc::c_char;
    fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: i32) -> *mut libc::c_char;
    fn lua_gettop(L: *mut lua_State) -> i32;
    fn lua_settop(L: *mut lua_State, index: i32);
    fn lua_pushvalue(L: *mut lua_State, index: i32);
    fn lua_rotate(L: *mut lua_State, index: i32, n: i32);
    fn lua_copy(L: *mut lua_State, fromidx: i32, toidx: i32);
    fn lua_isinteger(L: *mut lua_State, index: i32) -> i32;
    fn lua_type(L: *mut lua_State, index: i32) -> i32;
    fn lua_tonumberx(L: *mut lua_State, index: i32, isnum: *mut i32) -> f64;
    fn lua_tointegerx(L: *mut lua_State, index: i32, isnum: *mut i32) -> i64;
    fn lua_toboolean(L: *mut lua_State, index: i32) -> i32;
    fn lua_tolstring(L: *mut lua_State, index: i32, len: *mut u64) -> *const libc::c_char;
    fn lua_rawlen(L: *mut lua_State, index: i32) -> u64;
    fn lua_touserdata(L: *mut lua_State, index: i32) -> *mut libc::c_void;
    fn lua_pushnil(L: *mut lua_State);
    fn lua_pushinteger(L: *mut lua_State, n: i64);
    fn lua_pushstring(L: *mut lua_State, s: *const libc::c_char) -> *const libc::c_char;
    fn lua_pushfstring(L: *mut lua_State, fmt: *const libc::c_char, _: ...) -> *const libc::c_char;
    fn lua_pushcclosure(L: *mut lua_State, fn_0: CFunction, n: i32);
    fn lua_pushboolean(L: *mut lua_State, b: i32);
    fn lua_getfield(L: *mut lua_State, index: i32, k: *const libc::c_char) -> i32;
    fn lua_createtable(L: *mut lua_State, narr: i32, nrec: i32);
    fn lua_newuserdatauv(L: *mut lua_State, sz: u64, nuvalue: i32) -> *mut libc::c_void;
    fn lua_setfield(L: *mut lua_State, index: i32, k: *const libc::c_char);
    fn lua_stringtonumber(L: *mut lua_State, s: *const libc::c_char) -> u64;
    fn luaL_checkversion_(L: *mut lua_State, ver: f64, sz: u64);
    fn luaL_argerror(L: *mut lua_State, arg: i32, extramsg: *const libc::c_char) -> i32;
    fn luaL_checklstring(L: *mut lua_State, arg: i32, l: *mut u64) -> *const libc::c_char;
    fn luaL_optlstring(
        L: *mut lua_State,
        arg: i32,
        def: *const libc::c_char,
        l: *mut u64,
    ) -> *const libc::c_char;
    fn luaL_checkinteger(L: *mut lua_State, arg: i32) -> i64;
    fn luaL_optinteger(L: *mut lua_State, arg: i32, def: i64) -> i64;
    fn luaL_checkstack(L: *mut lua_State, sz: i32, msg: *const libc::c_char);
    fn luaL_checkany(L: *mut lua_State, arg: i32);
    fn luaL_newmetatable(L: *mut lua_State, tname: *const libc::c_char) -> i32;
    fn luaL_setmetatable(L: *mut lua_State, tname: *const libc::c_char);
    fn luaL_testudata(L: *mut lua_State, ud: i32, tname: *const libc::c_char) -> *mut libc::c_void;
    fn luaL_checkudata(L: *mut lua_State, ud: i32, tname: *const libc::c_char)
        -> *mut libc::c_void;
    fn luaL_error(L: *mut lua_State, fmt: *const libc::c_char, _: ...) -> i32;
    fn luaL_checkoption(
        L: *mut lua_State,
        arg: i32,
        def: *const libc::c_char,
        lst: *const *const libc::c_char,
    ) -> i32;
    fn luaL_fileresult(L: *mut lua_State, stat: i32, fname: *const libc::c_char) -> i32;
    fn luaL_execresult(L: *mut lua_State, stat: i32) -> i32;
    fn luaL_setfuncs(L: *mut lua_State, l: *const luaL_Reg, nup: i32);
    fn luaL_buffinit(L: *mut lua_State, B: *mut luaL_Buffer);
    fn luaL_prepbuffsize(B: *mut luaL_Buffer, sz: u64) -> *mut libc::c_char;
    fn luaL_pushresult(B: *mut luaL_Buffer);
}
pub type LIOLibC2RustUnnamed = u32;
pub const _ISalnum: LIOLibC2RustUnnamed = 8;
pub const _ISpunct: LIOLibC2RustUnnamed = 4;
pub const _IScntrl: LIOLibC2RustUnnamed = 2;
pub const _ISblank: LIOLibC2RustUnnamed = 1;
pub const _ISgraph: LIOLibC2RustUnnamed = 32768;
pub const _ISprint: LIOLibC2RustUnnamed = 16384;
pub const _ISspace: LIOLibC2RustUnnamed = 8192;
pub const _ISxdigit: LIOLibC2RustUnnamed = 4096;
pub const _ISdigit: LIOLibC2RustUnnamed = 2048;
pub const _ISalpha: LIOLibC2RustUnnamed = 1024;
pub const _ISlower: LIOLibC2RustUnnamed = 512;
pub const _ISupper: LIOLibC2RustUnnamed = 256;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lconv {
    pub decimal_point: *mut libc::c_char,
    pub thousands_sep: *mut libc::c_char,
    pub grouping: *mut libc::c_char,
    pub int_curr_symbol: *mut libc::c_char,
    pub currency_symbol: *mut libc::c_char,
    pub mon_decimal_point: *mut libc::c_char,
    pub mon_thousands_sep: *mut libc::c_char,
    pub mon_grouping: *mut libc::c_char,
    pub positive_sign: *mut libc::c_char,
    pub negative_sign: *mut libc::c_char,
    pub int_frac_digits: libc::c_char,
    pub frac_digits: libc::c_char,
    pub p_cs_precedes: libc::c_char,
    pub p_sep_by_space: libc::c_char,
    pub n_cs_precedes: libc::c_char,
    pub n_sep_by_space: libc::c_char,
    pub p_sign_posn: libc::c_char,
    pub n_sign_posn: libc::c_char,
    pub int_p_cs_precedes: libc::c_char,
    pub int_p_sep_by_space: libc::c_char,
    pub int_n_cs_precedes: libc::c_char,
    pub int_n_sep_by_space: libc::c_char,
    pub int_p_sign_posn: libc::c_char,
    pub int_n_sign_posn: libc::c_char,
}
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
    pub _old_offset: Offset,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: Offset,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: u64,
    pub _mode: i32,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type off_t = Offset;

pub type CFunction = Option<unsafe extern "C" fn(*mut lua_State) -> i32>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaL_Buffer {
    pub b: *mut libc::c_char,
    pub size: u64,
    pub n: u64,
    pub L: *mut lua_State,
    pub init: LIOLibC2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union LIOLibC2RustUnnamed_0 {
    pub n: f64,
    pub u: f64,
    pub s: *mut libc::c_void,
    pub i: i64,
    pub l: i64,
    pub b: [libc::c_char; 1024],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaL_Reg {
    pub name: *const libc::c_char,
    pub func: CFunction,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaL_Stream {
    pub f: *mut FILE,
    pub closef: CFunction,
}
pub type LStream = luaL_Stream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RN {
    pub f: *mut FILE,
    pub c: i32,
    pub n: i32,
    pub buff: [libc::c_char; 201],
}
#[inline]
unsafe extern "C" fn getc_unlocked(mut __fp: *mut FILE) -> i32 {
    return if ((*__fp)._IO_read_ptr >= (*__fp)._IO_read_end) as i32 as i64 != 0 {
        __uflow(__fp)
    } else {
        let fresh0 = (*__fp)._IO_read_ptr;
        (*__fp)._IO_read_ptr = ((*__fp)._IO_read_ptr).offset(1);
        *(fresh0 as *mut u8) as i32
    };
}
unsafe extern "C" fn l_checkmode(mut mode: *const libc::c_char) -> i32 {
    return (*mode as i32 != '\0' as i32
        && {
            let fresh1 = mode;
            mode = mode.offset(1);
            !(strchr(b"rwa\0" as *const u8 as *const libc::c_char, *fresh1 as i32)).is_null()
        }
        && (*mode as i32 != '+' as i32 || {
            mode = mode.offset(1);
            1i32 != 0
        })
        && strspn(mode, b"b\0" as *const u8 as *const libc::c_char) == strlen(mode))
        as i32;
}
unsafe extern "C" fn io_type(mut L: *mut lua_State) -> i32 {
    let mut p: *mut LStream = 0 as *mut LStream;
    luaL_checkany(L, 1i32);
    p = luaL_testudata(L, 1i32, b"FILE*\0" as *const u8 as *const libc::c_char) as *mut LStream;
    if p.is_null() {
        lua_pushnil(L);
    } else if ((*p).closef).is_none() {
        lua_pushstring(L, b"closed file\0" as *const u8 as *const libc::c_char);
    } else {
        lua_pushstring(L, b"file\0" as *const u8 as *const libc::c_char);
    }
    return 1i32;
}
unsafe extern "C" fn f_tostring(mut L: *mut lua_State) -> i32 {
    let mut p: *mut LStream =
        luaL_checkudata(L, 1i32, b"FILE*\0" as *const u8 as *const libc::c_char) as *mut LStream;
    if ((*p).closef).is_none() {
        lua_pushstring(L, b"file (closed)\0" as *const u8 as *const libc::c_char);
    } else {
        lua_pushfstring(
            L,
            b"file (%p)\0" as *const u8 as *const libc::c_char,
            (*p).f,
        );
    }
    return 1i32;
}
unsafe extern "C" fn tofile(mut L: *mut lua_State) -> *mut FILE {
    let mut p: *mut LStream =
        luaL_checkudata(L, 1i32, b"FILE*\0" as *const u8 as *const libc::c_char) as *mut LStream;
    if (((*p).closef).is_none() as i32 != 0i32) as i32 as i64 != 0 {
        luaL_error(
            L,
            b"attempt to use a closed file\0" as *const u8 as *const libc::c_char,
        );
    }
    return (*p).f;
}
unsafe extern "C" fn newprefile(mut L: *mut lua_State) -> *mut LStream {
    let mut p: *mut LStream =
        lua_newuserdatauv(L, ::core::mem::size_of::<LStream>() as libc::c_ulong, 0i32)
            as *mut LStream;
    (*p).closef = None;
    luaL_setmetatable(L, b"FILE*\0" as *const u8 as *const libc::c_char);
    return p;
}
unsafe extern "C" fn aux_close(mut L: *mut lua_State) -> i32 {
    let mut p: *mut LStream =
        luaL_checkudata(L, 1i32, b"FILE*\0" as *const u8 as *const libc::c_char) as *mut LStream;
    let mut cf: CFunction = (*p).closef;
    (*p).closef = None;
    return (Some(cf.expect("non-null function pointer"))).expect("non-null function pointer")(L);
}
unsafe extern "C" fn f_close(mut L: *mut lua_State) -> i32 {
    tofile(L);
    return aux_close(L);
}
unsafe extern "C" fn io_close(mut L: *mut lua_State) -> i32 {
    if lua_type(L, 1i32) == -(1i32) {
        lua_getfield(
            L,
            -(1000000i32) - 1000i32,
            b"_IO_output\0" as *const u8 as *const libc::c_char,
        );
    }
    return f_close(L);
}
unsafe extern "C" fn f_gc(mut L: *mut lua_State) -> i32 {
    let mut p: *mut LStream =
        luaL_checkudata(L, 1i32, b"FILE*\0" as *const u8 as *const libc::c_char) as *mut LStream;
    if ((*p).closef).is_some() && !((*p).f).is_null() {
        aux_close(L);
    }
    return 0i32;
}
unsafe extern "C" fn io_fclose(mut L: *mut lua_State) -> i32 {
    let mut p: *mut LStream =
        luaL_checkudata(L, 1i32, b"FILE*\0" as *const u8 as *const libc::c_char) as *mut LStream;
    *__errno_location() = 0i32;
    return luaL_fileresult(L, (fclose((*p).f) == 0i32) as i32, 0 as *const libc::c_char);
}
unsafe extern "C" fn newfile(mut L: *mut lua_State) -> *mut LStream {
    let mut p: *mut LStream = newprefile(L);
    (*p).f = 0 as *mut FILE;
    (*p).closef = Some(io_fclose as unsafe extern "C" fn(*mut lua_State) -> i32);
    return p;
}
unsafe extern "C" fn opencheck(
    mut L: *mut lua_State,
    mut fname: *const libc::c_char,
    mut mode: *const libc::c_char,
) {
    let mut p: *mut LStream = newfile(L);
    (*p).f = fopen(fname, mode);
    if (((*p).f == 0 as *mut libc::c_void as *mut FILE) as i32 != 0i32) as i32 as i64 != 0 {
        luaL_error(
            L,
            b"cannot open file '%s' (%s)\0" as *const u8 as *const libc::c_char,
            fname,
            strerror(*__errno_location()),
        );
    }
}
unsafe extern "C" fn io_open(mut L: *mut lua_State) -> i32 {
    let mut filename: *const libc::c_char = luaL_checklstring(L, 1i32, 0 as *mut u64);
    let mut mode: *const libc::c_char = luaL_optlstring(
        L,
        2i32,
        b"r\0" as *const u8 as *const libc::c_char,
        0 as *mut u64,
    );
    let mut p: *mut LStream = newfile(L);
    let mut md: *const libc::c_char = mode;
    ((l_checkmode(md) != 0i32) as i32 as i64 != 0
        || luaL_argerror(
            L,
            2i32,
            b"invalid mode\0" as *const u8 as *const libc::c_char,
        ) != 0) as i32;
    *__errno_location() = 0i32;
    (*p).f = fopen(filename, mode);
    return if ((*p).f).is_null() {
        luaL_fileresult(L, 0i32, filename)
    } else {
        1i32
    };
}
unsafe extern "C" fn io_pclose(mut L: *mut lua_State) -> i32 {
    let mut p: *mut LStream =
        luaL_checkudata(L, 1i32, b"FILE*\0" as *const u8 as *const libc::c_char) as *mut LStream;
    *__errno_location() = 0i32;
    return luaL_execresult(L, pclose((*p).f));
}
unsafe extern "C" fn io_popen(mut L: *mut lua_State) -> i32 {
    let mut filename: *const libc::c_char = luaL_checklstring(L, 1i32, 0 as *mut u64);
    let mut mode: *const libc::c_char = luaL_optlstring(
        L,
        2i32,
        b"r\0" as *const u8 as *const libc::c_char,
        0 as *mut u64,
    );
    let mut p: *mut LStream = newprefile(L);
    ((((*mode.offset(0i32 as isize) as i32 == 'r' as i32
        || *mode.offset(0i32 as isize) as i32 == 'w' as i32)
        && *mode.offset(1i32 as isize) as i32 == '\0' as i32) as i32
        != 0i32) as i32 as i64
        != 0
        || luaL_argerror(
            L,
            2i32,
            b"invalid mode\0" as *const u8 as *const libc::c_char,
        ) != 0) as i32;
    *__errno_location() = 0i32;
    fflush(0 as *mut FILE);
    (*p).f = popen(filename, mode);
    (*p).closef = Some(io_pclose as unsafe extern "C" fn(*mut lua_State) -> i32);
    return if ((*p).f).is_null() {
        luaL_fileresult(L, 0i32, filename)
    } else {
        1i32
    };
}
unsafe extern "C" fn io_tmpfile(mut L: *mut lua_State) -> i32 {
    let mut p: *mut LStream = newfile(L);
    *__errno_location() = 0i32;
    (*p).f = tmpfile();
    return if ((*p).f).is_null() {
        luaL_fileresult(L, 0i32, 0 as *const libc::c_char)
    } else {
        1i32
    };
}
unsafe extern "C" fn getiofile(
    mut L: *mut lua_State,
    mut findex: *const libc::c_char,
) -> *mut FILE {
    let mut p: *mut LStream = 0 as *mut LStream;
    lua_getfield(L, -(1000000i32) - 1000i32, findex);
    p = lua_touserdata(L, -(1i32)) as *mut LStream;
    if (((*p).closef).is_none() as i32 != 0i32) as i32 as i64 != 0 {
        luaL_error(
            L,
            b"default %s file is closed\0" as *const u8 as *const libc::c_char,
            findex.offset(
                (::core::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    .wrapping_sub(1i32 as libc::c_ulong) as isize,
            ),
        );
    }
    return (*p).f;
}
unsafe extern "C" fn g_iofile(
    mut L: *mut lua_State,
    mut f: *const libc::c_char,
    mut mode: *const libc::c_char,
) -> i32 {
    if !(lua_type(L, 1i32) <= 0i32) {
        let mut filename: *const libc::c_char = lua_tolstring(L, 1i32, 0 as *mut u64);
        if !filename.is_null() {
            opencheck(L, filename, mode);
        } else {
            tofile(L);
            lua_pushvalue(L, 1i32);
        }
        lua_setfield(L, -(1000000i32) - 1000i32, f);
    }
    lua_getfield(L, -(1000000i32) - 1000i32, f);
    return 1i32;
}
unsafe extern "C" fn io_input(mut L: *mut lua_State) -> i32 {
    return g_iofile(
        L,
        b"_IO_input\0" as *const u8 as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn io_output(mut L: *mut lua_State) -> i32 {
    return g_iofile(
        L,
        b"_IO_output\0" as *const u8 as *const libc::c_char,
        b"w\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn aux_lines(mut L: *mut lua_State, mut toclose: i32) {
    let mut n: i32 = lua_gettop(L) - 1i32;
    (((n <= 250i32) as i32 != 0i32) as i32 as i64 != 0
        || luaL_argerror(
            L,
            250i32 + 2i32,
            b"too many arguments\0" as *const u8 as *const libc::c_char,
        ) != 0) as i32;
    lua_pushvalue(L, 1i32);
    lua_pushinteger(L, n as i64);
    lua_pushboolean(L, toclose);
    lua_rotate(L, 2i32, 3i32);
    lua_pushcclosure(
        L,
        Some(io_readline as unsafe extern "C" fn(*mut lua_State) -> i32),
        3i32 + n,
    );
}
unsafe extern "C" fn f_lines(mut L: *mut lua_State) -> i32 {
    tofile(L);
    aux_lines(L, 0i32);
    return 1i32;
}
unsafe extern "C" fn io_lines(mut L: *mut lua_State) -> i32 {
    let mut toclose: i32 = 0;
    if lua_type(L, 1i32) == -(1i32) {
        lua_pushnil(L);
    }
    if lua_type(L, 1i32) == 0i32 {
        lua_getfield(
            L,
            -(1000000i32) - 1000i32,
            b"_IO_input\0" as *const u8 as *const libc::c_char,
        );
        lua_copy(L, -(1i32), 1i32);
        lua_settop(L, -(1i32) - 1i32);
        tofile(L);
        toclose = 0i32;
    } else {
        let mut filename: *const libc::c_char = luaL_checklstring(L, 1i32, 0 as *mut u64);
        opencheck(L, filename, b"r\0" as *const u8 as *const libc::c_char);
        lua_copy(L, -(1i32), 1i32);
        lua_settop(L, -(1i32) - 1i32);
        toclose = 1i32;
    }
    aux_lines(L, toclose);
    if toclose != 0 {
        lua_pushnil(L);
        lua_pushnil(L);
        lua_pushvalue(L, 1i32);
        return 4i32;
    } else {
        return 1i32;
    };
}
unsafe extern "C" fn nextc(mut rn: *mut RN) -> i32 {
    if (((*rn).n >= 200i32) as i32 != 0i32) as i32 as i64 != 0 {
        (*rn).buff[0i32 as usize] = '\0' as i32 as libc::c_char;
        return 0i32;
    } else {
        let fresh2 = (*rn).n;
        (*rn).n = (*rn).n + 1;
        (*rn).buff[fresh2 as usize] = (*rn).c as libc::c_char;
        (*rn).c = getc_unlocked((*rn).f);
        return 1i32;
    };
}
unsafe extern "C" fn test2(mut rn: *mut RN, mut set: *const libc::c_char) -> i32 {
    if (*rn).c == *set.offset(0i32 as isize) as i32 || (*rn).c == *set.offset(1i32 as isize) as i32
    {
        return nextc(rn);
    } else {
        return 0i32;
    };
}
unsafe extern "C" fn readdigits(mut rn: *mut RN, mut hex: i32) -> i32 {
    let mut count: i32 = 0i32;
    while (if hex != 0 {
        *(*__ctype_b_loc()).offset((*rn).c as isize) as i32
            & _ISxdigit as i32 as libc::c_ushort as i32
    } else {
        *(*__ctype_b_loc()).offset((*rn).c as isize) as i32
            & _ISdigit as i32 as libc::c_ushort as i32
    }) != 0
        && nextc(rn) != 0
    {
        count += 1;
    }
    return count;
}
unsafe extern "C" fn read_number(mut L: *mut lua_State, mut f: *mut FILE) -> i32 {
    let mut rn: RN = RN {
        f: 0 as *mut FILE,
        c: 0,
        n: 0,
        buff: [0; 201],
    };
    let mut count: i32 = 0i32;
    let mut hex: i32 = 0i32;
    let mut decp: [libc::c_char; 2] = [0; 2];
    rn.f = f;
    rn.n = 0i32;
    decp[0i32 as usize] = *((*localeconv()).decimal_point).offset(0i32 as isize);
    decp[1i32 as usize] = '.' as i32 as libc::c_char;
    flockfile(rn.f);
    loop {
        rn.c = getc_unlocked(rn.f);
        if !(*(*__ctype_b_loc()).offset(rn.c as isize) as i32
            & _ISspace as i32 as libc::c_ushort as i32
            != 0)
        {
            break;
        }
    }
    test2(&mut rn, b"-+\0" as *const u8 as *const libc::c_char);
    if test2(&mut rn, b"00\0" as *const u8 as *const libc::c_char) != 0 {
        if test2(&mut rn, b"xX\0" as *const u8 as *const libc::c_char) != 0 {
            hex = 1i32;
        } else {
            count = 1i32;
        }
    }
    count += readdigits(&mut rn, hex);
    if test2(&mut rn, decp.as_mut_ptr()) != 0 {
        count += readdigits(&mut rn, hex);
    }
    if count > 0i32
        && test2(
            &mut rn,
            if hex != 0 {
                b"pP\0" as *const u8 as *const libc::c_char
            } else {
                b"eE\0" as *const u8 as *const libc::c_char
            },
        ) != 0
    {
        test2(&mut rn, b"-+\0" as *const u8 as *const libc::c_char);
        readdigits(&mut rn, 0i32);
    }
    ungetc(rn.c, rn.f);
    funlockfile(rn.f);
    rn.buff[rn.n as usize] = '\0' as i32 as libc::c_char;
    if (lua_stringtonumber(L, (rn.buff).as_mut_ptr()) != 0i32 as libc::c_ulong) as i32 as i64 != 0 {
        return 1i32;
    } else {
        lua_pushnil(L);
        return 0i32;
    };
}
unsafe extern "C" fn test_eof(mut L: *mut lua_State, mut f: *mut FILE) -> i32 {
    let mut c: i32 = getc(f);
    ungetc(c, f);
    lua_pushstring(L, b"\0" as *const u8 as *const libc::c_char);
    return (c != -(1i32)) as i32;
}
unsafe extern "C" fn read_line(mut L: *mut lua_State, mut f: *mut FILE, mut chop: i32) -> i32 {
    let mut b: luaL_Buffer = luaL_Buffer {
        b: 0 as *mut libc::c_char,
        size: 0,
        n: 0,
        L: 0 as *mut lua_State,
        init: LIOLibC2RustUnnamed_0 { n: 0. },
    };
    let mut c: i32 = 0;
    luaL_buffinit(L, &mut b);
    loop {
        let mut buff: *mut libc::c_char = luaL_prepbuffsize(
            &mut b,
            (16i32 as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<f64>() as libc::c_ulong) as i32
                as u64,
        );
        let mut i: i32 = 0i32;
        flockfile(f);
        while i
            < (16i32 as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<f64>() as libc::c_ulong) as i32
            && {
                c = getc_unlocked(f);
                c != -(1i32)
            }
            && c != '\n' as i32
        {
            let fresh3 = i;
            i = i + 1;
            *buff.offset(fresh3 as isize) = c as libc::c_char;
        }
        funlockfile(f);
        b.n = (b.n as libc::c_ulong).wrapping_add(i as libc::c_ulong) as u64 as u64;
        if !(c != -(1i32) && c != '\n' as i32) {
            break;
        }
    }
    if chop == 0 && c == '\n' as i32 {
        (b.n < b.size || !(luaL_prepbuffsize(&mut b, 1i32 as u64)).is_null()) as i32;
        let fresh4 = b.n;
        b.n = (b.n).wrapping_add(1);
        *(b.b).offset(fresh4 as isize) = c as libc::c_char;
    }
    luaL_pushresult(&mut b);
    return (c == '\n' as i32 || lua_rawlen(L, -(1i32)) > 0i32 as u64) as i32;
}
unsafe extern "C" fn read_all(mut L: *mut lua_State, mut f: *mut FILE) {
    let mut nr: u64 = 0;
    let mut b: luaL_Buffer = luaL_Buffer {
        b: 0 as *mut libc::c_char,
        size: 0,
        n: 0,
        L: 0 as *mut lua_State,
        init: LIOLibC2RustUnnamed_0 { n: 0. },
    };
    luaL_buffinit(L, &mut b);
    loop {
        let mut p: *mut libc::c_char = luaL_prepbuffsize(
            &mut b,
            (16i32 as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<f64>() as libc::c_ulong) as i32
                as u64,
        );
        nr = fread(
            p as *mut libc::c_void,
            ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
            (16i32 as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<f64>() as libc::c_ulong) as i32
                as libc::c_ulong,
            f,
        );
        b.n = (b.n as libc::c_ulong).wrapping_add(nr) as u64 as u64;
        if !(nr
            == (16i32 as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<f64>() as libc::c_ulong) as i32
                as libc::c_ulong)
        {
            break;
        }
    }
    luaL_pushresult(&mut b);
}
unsafe extern "C" fn read_chars(mut L: *mut lua_State, mut f: *mut FILE, mut n: u64) -> i32 {
    let mut nr: u64 = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut b: luaL_Buffer = luaL_Buffer {
        b: 0 as *mut libc::c_char,
        size: 0,
        n: 0,
        L: 0 as *mut lua_State,
        init: LIOLibC2RustUnnamed_0 { n: 0. },
    };
    luaL_buffinit(L, &mut b);
    p = luaL_prepbuffsize(&mut b, n);
    nr = fread(
        p as *mut libc::c_void,
        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
        n,
        f,
    );
    b.n = (b.n as libc::c_ulong).wrapping_add(nr) as u64 as u64;
    luaL_pushresult(&mut b);
    return (nr > 0i32 as libc::c_ulong) as i32;
}
unsafe extern "C" fn g_read(mut L: *mut lua_State, mut f: *mut FILE, mut first: i32) -> i32 {
    let mut nargs: i32 = lua_gettop(L) - 1i32;
    let mut n: i32 = 0;
    let mut success: i32 = 0;
    clearerr(f);
    *__errno_location() = 0i32;
    if nargs == 0i32 {
        success = read_line(L, f, 1i32);
        n = first + 1i32;
    } else {
        luaL_checkstack(
            L,
            nargs + 20i32,
            b"too many arguments\0" as *const u8 as *const libc::c_char,
        );
        success = 1i32;
        n = first;
        loop {
            let fresh5 = nargs;
            nargs = nargs - 1;
            if !(fresh5 != 0 && success != 0) {
                break;
            }
            if lua_type(L, n) == 3i32 {
                let mut l: u64 = luaL_checkinteger(L, n) as u64;
                success = if l == 0i32 as libc::c_ulong {
                    test_eof(L, f)
                } else {
                    read_chars(L, f, l)
                };
            } else {
                let mut p: *const libc::c_char = luaL_checklstring(L, n, 0 as *mut u64);
                if *p as i32 == '*' as i32 {
                    p = p.offset(1);
                }
                match *p as i32 {
                    110 => {
                        success = read_number(L, f);
                    }
                    108 => {
                        success = read_line(L, f, 1i32);
                    }
                    76 => {
                        success = read_line(L, f, 0i32);
                    }
                    97 => {
                        read_all(L, f);
                        success = 1i32;
                    }
                    _ => {
                        return luaL_argerror(
                            L,
                            n,
                            b"invalid format\0" as *const u8 as *const libc::c_char,
                        );
                    }
                }
            }
            n += 1;
        }
    }
    if ferror(f) != 0 {
        return luaL_fileresult(L, 0i32, 0 as *const libc::c_char);
    }
    if success == 0 {
        lua_settop(L, -(1i32) - 1i32);
        lua_pushnil(L);
    }
    return n - first;
}
unsafe extern "C" fn io_read(mut L: *mut lua_State) -> i32 {
    return g_read(
        L,
        getiofile(L, b"_IO_input\0" as *const u8 as *const libc::c_char),
        1i32,
    );
}
unsafe extern "C" fn f_read(mut L: *mut lua_State) -> i32 {
    return g_read(L, tofile(L), 2i32);
}
unsafe extern "C" fn io_readline(mut L: *mut lua_State) -> i32 {
    let mut p: *mut LStream = lua_touserdata(L, -(1000000i32) - 1000i32 - 1i32) as *mut LStream;
    let mut i: i32 = 0;
    let mut n: i32 = lua_tointegerx(L, -(1000000i32) - 1000i32 - 2i32, 0 as *mut i32) as i32;
    if ((*p).closef).is_none() {
        return luaL_error(
            L,
            b"file is already closed\0" as *const u8 as *const libc::c_char,
        );
    }
    lua_settop(L, 1i32);
    luaL_checkstack(
        L,
        n,
        b"too many arguments\0" as *const u8 as *const libc::c_char,
    );
    i = 1i32;
    while i <= n {
        lua_pushvalue(L, -(1000000i32) - 1000i32 - (3i32 + i));
        i += 1;
    }
    n = g_read(L, (*p).f, 2i32);
    if lua_toboolean(L, -n) != 0 {
        return n;
    } else {
        if n > 1i32 {
            return luaL_error(
                L,
                b"%s\0" as *const u8 as *const libc::c_char,
                lua_tolstring(L, -n + 1i32, 0 as *mut u64),
            );
        }
        if lua_toboolean(L, -(1000000i32) - 1000i32 - 3i32) != 0 {
            lua_settop(L, 0i32);
            lua_pushvalue(L, -(1000000i32) - 1000i32 - 1i32);
            aux_close(L);
        }
        return 0i32;
    };
}
unsafe extern "C" fn g_write(mut L: *mut lua_State, mut f: *mut FILE, mut arg: i32) -> i32 {
    let mut nargs: i32 = lua_gettop(L) - arg;
    let mut status: i32 = 1i32;
    *__errno_location() = 0i32;
    loop {
        let fresh6 = nargs;
        nargs = nargs - 1;
        if !(fresh6 != 0) {
            break;
        }
        if lua_type(L, arg) == 3i32 {
            let mut len: i32 = if lua_isinteger(L, arg) != 0 {
                fprintf(
                    f,
                    b"%lld\0" as *const u8 as *const libc::c_char,
                    lua_tointegerx(L, arg, 0 as *mut i32),
                )
            } else {
                fprintf(
                    f,
                    b"%.14g\0" as *const u8 as *const libc::c_char,
                    lua_tonumberx(L, arg, 0 as *mut i32),
                )
            };
            status = (status != 0 && len > 0i32) as i32;
        } else {
            let mut l: u64 = 0;
            let mut s: *const libc::c_char = luaL_checklstring(L, arg, &mut l);
            status = (status != 0
                && fwrite(
                    s as *const libc::c_void,
                    ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                    l,
                    f,
                ) == l) as i32;
        }
        arg += 1;
    }
    if (status != 0i32) as i32 as i64 != 0 {
        return 1i32;
    } else {
        return luaL_fileresult(L, status, 0 as *const libc::c_char);
    };
}
unsafe extern "C" fn io_write(mut L: *mut lua_State) -> i32 {
    return g_write(
        L,
        getiofile(L, b"_IO_output\0" as *const u8 as *const libc::c_char),
        1i32,
    );
}
unsafe extern "C" fn f_write(mut L: *mut lua_State) -> i32 {
    let mut f: *mut FILE = tofile(L);
    lua_pushvalue(L, 1i32);
    return g_write(L, f, 2i32);
}
unsafe extern "C" fn f_seek(mut L: *mut lua_State) -> i32 {
    static mut mode: [i32; 3] = [0i32, 1i32, 2i32];
    static mut modenames: [*const libc::c_char; 4] = [
        b"set\0" as *const u8 as *const libc::c_char,
        b"cur\0" as *const u8 as *const libc::c_char,
        b"end\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut f: *mut FILE = tofile(L);
    let mut op: i32 = luaL_checkoption(
        L,
        2i32,
        b"cur\0" as *const u8 as *const libc::c_char,
        modenames.as_ptr(),
    );
    let mut p3: i64 = luaL_optinteger(L, 3i32, 0i32 as i64);
    let mut offset: off_t = p3 as off_t;
    (((offset as i64 == p3) as i32 != 0i32) as i32 as i64 != 0
        || luaL_argerror(
            L,
            3i32,
            b"not an integer in proper range\0" as *const u8 as *const libc::c_char,
        ) != 0) as i32;
    *__errno_location() = 0i32;
    op = fseeko(f, offset, mode[op as usize]);
    if (op != 0i32) as i32 as i64 != 0 {
        return luaL_fileresult(L, 0i32, 0 as *const libc::c_char);
    } else {
        lua_pushinteger(L, ftello(f) as i64);
        return 1i32;
    };
}
unsafe extern "C" fn f_setvbuf(mut L: *mut lua_State) -> i32 {
    static mut mode: [i32; 3] = [2i32, 0i32, 1i32];
    static mut modenames: [*const libc::c_char; 4] = [
        b"no\0" as *const u8 as *const libc::c_char,
        b"full\0" as *const u8 as *const libc::c_char,
        b"line\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut f: *mut FILE = tofile(L);
    let mut op: i32 = luaL_checkoption(L, 2i32, 0 as *const libc::c_char, modenames.as_ptr());
    let mut sz: i64 = luaL_optinteger(
        L,
        3i32,
        (16i32 as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<f64>() as libc::c_ulong) as i32 as i64,
    );
    let mut res: i32 = 0;
    *__errno_location() = 0i32;
    res = setvbuf(f, 0 as *mut libc::c_char, mode[op as usize], sz as u64);
    return luaL_fileresult(L, (res == 0i32) as i32, 0 as *const libc::c_char);
}
unsafe extern "C" fn io_flush(mut L: *mut lua_State) -> i32 {
    let mut f: *mut FILE = getiofile(L, b"_IO_output\0" as *const u8 as *const libc::c_char);
    *__errno_location() = 0i32;
    return luaL_fileresult(L, (fflush(f) == 0i32) as i32, 0 as *const libc::c_char);
}
unsafe extern "C" fn f_flush(mut L: *mut lua_State) -> i32 {
    let mut f: *mut FILE = tofile(L);
    *__errno_location() = 0i32;
    return luaL_fileresult(L, (fflush(f) == 0i32) as i32, 0 as *const libc::c_char);
}
static mut iolib: [luaL_Reg; 12] = {
    [
        {
            let mut init = luaL_Reg {
                name: b"close\0" as *const u8 as *const libc::c_char,
                func: Some(io_close as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"flush\0" as *const u8 as *const libc::c_char,
                func: Some(io_flush as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"input\0" as *const u8 as *const libc::c_char,
                func: Some(io_input as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"lines\0" as *const u8 as *const libc::c_char,
                func: Some(io_lines as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"open\0" as *const u8 as *const libc::c_char,
                func: Some(io_open as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"output\0" as *const u8 as *const libc::c_char,
                func: Some(io_output as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"popen\0" as *const u8 as *const libc::c_char,
                func: Some(io_popen as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"read\0" as *const u8 as *const libc::c_char,
                func: Some(io_read as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"tmpfile\0" as *const u8 as *const libc::c_char,
                func: Some(io_tmpfile as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"type\0" as *const u8 as *const libc::c_char,
                func: Some(io_type as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"write\0" as *const u8 as *const libc::c_char,
                func: Some(io_write as unsafe extern "C" fn(*mut lua_State) -> i32),
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
static mut meth: [luaL_Reg; 8] = {
    [
        {
            let mut init = luaL_Reg {
                name: b"read\0" as *const u8 as *const libc::c_char,
                func: Some(f_read as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"write\0" as *const u8 as *const libc::c_char,
                func: Some(f_write as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"lines\0" as *const u8 as *const libc::c_char,
                func: Some(f_lines as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"flush\0" as *const u8 as *const libc::c_char,
                func: Some(f_flush as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"seek\0" as *const u8 as *const libc::c_char,
                func: Some(f_seek as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"close\0" as *const u8 as *const libc::c_char,
                func: Some(f_close as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"setvbuf\0" as *const u8 as *const libc::c_char,
                func: Some(f_setvbuf as unsafe extern "C" fn(*mut lua_State) -> i32),
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
static mut metameth: [luaL_Reg; 5] = {
    [
        {
            let mut init = luaL_Reg {
                name: b"__index\0" as *const u8 as *const libc::c_char,
                func: None,
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"__gc\0" as *const u8 as *const libc::c_char,
                func: Some(f_gc as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"__close\0" as *const u8 as *const libc::c_char,
                func: Some(f_gc as unsafe extern "C" fn(*mut lua_State) -> i32),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"__tostring\0" as *const u8 as *const libc::c_char,
                func: Some(f_tostring as unsafe extern "C" fn(*mut lua_State) -> i32),
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
unsafe extern "C" fn createmeta(mut L: *mut lua_State) {
    luaL_newmetatable(L, b"FILE*\0" as *const u8 as *const libc::c_char);
    luaL_setfuncs(L, metameth.as_ptr(), 0i32);
    lua_createtable(
        L,
        0i32,
        (::core::mem::size_of::<[luaL_Reg; 8]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<luaL_Reg>() as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong) as i32,
    );
    luaL_setfuncs(L, meth.as_ptr(), 0i32);
    lua_setfield(L, -(2i32), b"__index\0" as *const u8 as *const libc::c_char);
    lua_settop(L, -(1i32) - 1i32);
}
unsafe extern "C" fn io_noclose(mut L: *mut lua_State) -> i32 {
    let mut p: *mut LStream =
        luaL_checkudata(L, 1i32, b"FILE*\0" as *const u8 as *const libc::c_char) as *mut LStream;
    (*p).closef = Some(io_noclose as unsafe extern "C" fn(*mut lua_State) -> i32);
    lua_pushnil(L);
    lua_pushstring(
        L,
        b"cannot close standard file\0" as *const u8 as *const libc::c_char,
    );
    return 2i32;
}
unsafe extern "C" fn createstdfile(
    mut L: *mut lua_State,
    mut f: *mut FILE,
    mut k: *const libc::c_char,
    mut fname: *const libc::c_char,
) {
    let mut p: *mut LStream = newprefile(L);
    (*p).f = f;
    (*p).closef = Some(io_noclose as unsafe extern "C" fn(*mut lua_State) -> i32);
    if !k.is_null() {
        lua_pushvalue(L, -(1i32));
        lua_setfield(L, -(1000000i32) - 1000i32, k);
    }
    lua_setfield(L, -(2i32), fname);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaopen_io(mut L: *mut lua_State) -> i32 {
    luaL_checkversion_(
        L,
        504i32 as f64,
        (::core::mem::size_of::<i64>() as libc::c_ulong)
            .wrapping_mul(16i32 as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<f64>() as libc::c_ulong),
    );
    lua_createtable(
        L,
        0i32,
        (::core::mem::size_of::<[luaL_Reg; 12]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<luaL_Reg>() as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong) as i32,
    );
    luaL_setfuncs(L, iolib.as_ptr(), 0i32);
    createmeta(L);
    createstdfile(
        L,
        stdin,
        b"_IO_input\0" as *const u8 as *const libc::c_char,
        b"stdin\0" as *const u8 as *const libc::c_char,
    );
    createstdfile(
        L,
        stdout,
        b"_IO_output\0" as *const u8 as *const libc::c_char,
        b"stdout\0" as *const u8 as *const libc::c_char,
    );
    createstdfile(
        L,
        stderr,
        0 as *const libc::c_char,
        b"stderr\0" as *const u8 as *const libc::c_char,
    );
    return 1i32;
}
