use crate::utility::c::*;
use crate::io::stream::*;
use crate::state::*;
use crate::character::*;
use crate::functions::*;
use crate::tag::*;
use crate::io::rn::*;
use crate::user::*;
use crate::registeredfunction::*;
use crate::new::*;
use crate::buffer::*;
pub unsafe extern "C" fn l_checkmode(mut mode: *const i8) -> i32 {
    unsafe {
        return (*mode as i32 != CHARACTER_NUL as i32
            && {
                let fresh151 = mode;
                mode = mode.offset(1);
                !(strchr(b"rwa\0" as *const u8 as *const i8, *fresh151 as i32)).is_null()
            }
            && (*mode as i32 != CHARACTER_PLUS as i32 || {
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
        } else if ((*p).close_function).is_none() {
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
        if ((*p).close_function).is_none() {
            lua_pushstring(state, b"file (closed)\0" as *const u8 as *const i8);
        } else {
            lua_pushfstring(state, b"file (%p)\0" as *const u8 as *const i8, (*p).file);
        }
        return 1;
    }
}
pub unsafe extern "C" fn tofile(state: *mut State) -> *mut FILE {
    unsafe {
        let p: *mut Stream =
            lual_checkudata(state, 1, b"FILE*\0" as *const u8 as *const i8) as *mut Stream;
        if (((*p).close_function).is_none() as i32 != 0) as i64 != 0 {
            lual_error(
                state,
                b"attempt to use a closed file\0" as *const u8 as *const i8,
            );
        }
        return (*p).file;
    }
}
pub unsafe extern "C" fn newprefile(state: *mut State) -> *mut Stream {
    unsafe {
        let p: *mut Stream =
            User::lua_newuserdatauv(state, ::core::mem::size_of::<Stream>(), 0) as *mut Stream;
        (*p).close_function = None;
        lual_setmetatable(state, b"FILE*\0" as *const u8 as *const i8);
        return p;
    }
}
pub unsafe extern "C" fn aux_close(state: *mut State) -> i32 {
    unsafe {
        let p: *mut Stream =
            lual_checkudata(state, 1, b"FILE*\0" as *const u8 as *const i8) as *mut Stream;
        let cf: CFunction = (*p).close_function;
        (*p).close_function = None;
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
        if ((*p).close_function).is_some() && !((*p).file).is_null() {
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
        return lual_fileresult(state, (fclose((*p).file) == 0) as i32, std::ptr::null());
    }
}
pub unsafe extern "C" fn newfile(state: *mut State) -> *mut Stream {
    unsafe {
        let p: *mut Stream = newprefile(state);
        (*p).file = std::ptr::null_mut();
        (*p).close_function = Some(io_fclose as unsafe extern "C" fn(*mut State) -> i32);
        return p;
    }
}
pub unsafe extern "C" fn opencheck(state: *mut State, fname: *const i8, mode: *const i8) {
    unsafe {
        let p: *mut Stream = newfile(state);
        (*p).file = fopen(fname, mode);
        if (((*p).file == std::ptr::null_mut() as *mut FILE) as i32 != 0) as i64 != 0 {
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
        (*p).file = fopen(filename, mode);
        return if ((*p).file).is_null() {
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
        return lual_execresult(state, pclose((*p).file));
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
        ((((*mode.offset(0 as isize) as i32 == CHARACTER_LOWER_R as i32
            || *mode.offset(0 as isize) as i32 == CHARACTER_LOWER_W as i32)
            && *mode.offset(1 as isize) as i32 == CHARACTER_NUL as i32) as i32
            != 0) as i64
            != 0
            || lual_argerror(state, 2, b"invalid mode\0" as *const u8 as *const i8) != 0)
            as i32;
        *__errno_location() = 0;
        fflush(std::ptr::null_mut());
        (*p).file = popen(filename, mode);
        (*p).close_function = Some(io_pclose as unsafe extern "C" fn(*mut State) -> i32);
        return if ((*p).file).is_null() {
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
        (*p).file = tmpfile();
        return if ((*p).file).is_null() {
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
        if (((*p).close_function).is_none() as i32 != 0) as i64 != 0 {
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
        return (*p).file;
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
            (*rn).buffer[0] = CHARACTER_NUL as i8;
            return 0;
        } else {
            let fresh152 = (*rn).n;
            (*rn).n = (*rn).n + 1;
            (*rn).buffer[fresh152 as usize] = (*rn).c as i8;
            (*rn).c = getc_unlocked((*rn).file);
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
pub unsafe extern "C" fn read_number(state: *mut State, file: *mut FILE) -> i32 {
    unsafe {
        let mut rn: RN = RN {
            file: std::ptr::null_mut(),
            c: 0,
            n: 0,
            buffer: [0; 201],
        };
        let mut count: i32 = 0;
        let mut hex: i32 = 0;
        let mut decp: [i8; 2] = [0; 2];
        rn.file = file;
        rn.n = 0;
        decp[0] = CHARACTER_PERIOD as i8;
        decp[1] = CHARACTER_PERIOD as i8;
        flockfile(rn.file);
        loop {
            rn.c = getc_unlocked(rn.file);
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
        ungetc(rn.c, rn.file);
        funlockfile(rn.file);
        rn.buffer[rn.n as usize] = CHARACTER_NUL as i8;
        if (lua_stringtonumber(state, (rn.buffer).as_mut_ptr()) != 0) as i64 != 0 {
            return 1;
        } else {
            (*state).push_nil();
            return 0;
        };
    }
}
pub unsafe extern "C" fn test_eof(state: *mut State, file: *mut FILE) -> i32 {
    unsafe {
        let c: i32 = getc(file);
        ungetc(c, file);
        lua_pushstring(state, b"\0" as *const u8 as *const i8);
        return (c != -1) as i32;
    }
}
pub unsafe extern "C" fn read_line(state: *mut State, file: *mut FILE, chop: i32) -> i32 {
    unsafe {
        let mut b = Buffer::new();
        let mut c: i32 = 0;
        b.initialize(state);
        loop {
            let buffer: *mut i8 = b.prepare_with_size(
                (16 as usize)
                    .wrapping_mul(::core::mem::size_of::<*mut libc::c_void>())
                    .wrapping_mul(::core::mem::size_of::<f64>()),
            );
            let mut i: i32 = 0;
            flockfile(file);
            while i
                < (16 as u64)
                    .wrapping_mul(::core::mem::size_of::<*mut libc::c_void>() as u64)
                    .wrapping_mul(::core::mem::size_of::<f64>() as u64) as i32
                && {
                    c = getc_unlocked(file);
                    c != -1
                }
                && c != CHARACTER_LF as i32
            {
                let fresh153 = i;
                i = i + 1;
                *buffer.offset(fresh153 as isize) = c as i8;
            }
            funlockfile(file);
            b.length = b.length.wrapping_add(i as usize);
            if !(c != -1 && c != CHARACTER_LF as i32) {
                break;
            }
        }
        if chop == 0 && c == CHARACTER_LF as i32 {
            (b.length < b.size || !(b.prepare_with_size(1)).is_null()) as i32;
            let fresh154 = b.length;
            b.length = (b.length).wrapping_add(1);
            *(b.pointer).offset(fresh154 as isize) = c as i8;
        }
        b.push_result();
        return (c == CHARACTER_LF as i32 || get_length_raw(state, -1) > 0) as u64 as u32 as i32;
    }
}
pub unsafe extern "C" fn read_all(state: *mut State, file: *mut FILE) {
    unsafe {
        let mut nr: u64;
        let mut b = Buffer::new();
        b.initialize(state);
        loop {
            let p: *mut i8 = b.prepare_with_size(
                (16 as usize)
                    .wrapping_mul(::core::mem::size_of::<*mut libc::c_void>())
                    .wrapping_mul(::core::mem::size_of::<f64>()),
            );
            nr = fread(
                p as *mut libc::c_void,
                ::core::mem::size_of::<i8>() as u64,
                (16 as u64)
                    .wrapping_mul(::core::mem::size_of::<*mut libc::c_void>() as u64)
                    .wrapping_mul(::core::mem::size_of::<f64>() as u64) as i32
                    as u64,
                file,
            );
            b.length = b.length.wrapping_add(nr as usize);
            if !(nr
                == (16 as u64)
                    .wrapping_mul(::core::mem::size_of::<*mut libc::c_void>() as u64)
                    .wrapping_mul(::core::mem::size_of::<f64>() as u64) as i32
                    as u64)
            {
                break;
            }
        }
        b.push_result();
    }
}
pub unsafe extern "C" fn read_chars(state: *mut State, file: *mut FILE, n: u64) -> i32 {
    unsafe {
        let nr: u64;
        let p: *mut i8;
        let mut b = Buffer::new();
        b.initialize(state);
        p = b.prepare_with_size(n as usize);
        nr = fread(
            p as *mut libc::c_void,
            ::core::mem::size_of::<i8>() as u64,
            n,
            file,
        );
        b.length = b.length.wrapping_add(nr as usize);
        b.push_result();
        return (nr > 0) as i32;
    }
}
pub unsafe extern "C" fn g_read(state: *mut State, file: *mut FILE, first: i32) -> i32 {
    unsafe {
        let mut nargs: i32 = (*state).get_top() - 1;
        let mut n: i32;
        let mut success: i32;
        clearerr(file);
        *__errno_location() = 0;
        if nargs == 0 {
            success = read_line(state, file, 1);
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
                    success = if l == 0 {
                        test_eof(state, file)
                    } else {
                        read_chars(state, file, l)
                    };
                } else {
                    let mut p: *const i8 = lual_checklstring(state, n, std::ptr::null_mut());
                    if *p as i32 == CHARACTER_ASTERISK as i32 {
                        p = p.offset(1);
                    }
                    match *p as i32 {
                        110 => {
                            success = read_number(state, file);
                        }
                        108 => {
                            success = read_line(state, file, 1);
                        }
                        76 => {
                            success = read_line(state, file, 0);
                        }
                        97 => {
                            read_all(state, file);
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
        if ferror(file) != 0 {
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
        let mut n: i32 = lua_tointegerx(
            state,
            -(1000000 as i32) - 1000 as i32 - 2,
            std::ptr::null_mut(),
        ) as i32;
        if ((*p).close_function).is_none() {
            return lual_error(state, b"file is already closed\0" as *const u8 as *const i8);
        }
        lua_settop(state, 1);
        lual_checkstack(state, n, b"too many arguments\0" as *const u8 as *const i8);
        for i in 1..(1 + n) {
            lua_pushvalue(state, -(1000000 as i32) - 1000 as i32 - (3 + i));
        }
        n = g_read(state, (*p).file, 2);
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
pub unsafe extern "C" fn g_write(state: *mut State, file: *mut FILE, mut arg: i32) -> i32 {
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
                        file,
                        b"%lld\0" as *const u8 as *const i8,
                        lua_tointegerx(state, arg, std::ptr::null_mut()),
                    )
                } else {
                    fprintf(
                        file,
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
                        ::core::mem::size_of::<i8>(),
                        l as usize,
                        file,
                    ) == l as usize) as i32;
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
        let file: *mut FILE = tofile(state);
        lua_pushvalue(state, 1);
        return g_write(state, file, 2);
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
        let file: *mut FILE = tofile(state);
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
        op = fseeko(file, offset, MODE[op as usize]);
        if (op != 0) as i64 != 0 {
            return lual_fileresult(state, 0, std::ptr::null());
        } else {
            (*state).push_integer(ftello(file) as i64);
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
        let file: *mut FILE = tofile(state);
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
        res = setvbuf(file, std::ptr::null_mut(), MODE[op as usize], size as u64);
        return lual_fileresult(state, (res == 0) as i32, std::ptr::null());
    }
}
pub unsafe extern "C" fn io_flush(state: *mut State) -> i32 {
    unsafe {
        let file: *mut FILE = getiofile(state, b"_IO_output\0" as *const u8 as *const i8);
        *__errno_location() = 0;
        return lual_fileresult(state, (fflush(file) == 0) as i32, std::ptr::null());
    }
}
pub unsafe extern "C" fn f_flush(state: *mut State) -> i32 {
    unsafe {
        let file: *mut FILE = tofile(state);
        *__errno_location() = 0;
        return lual_fileresult(state, (fflush(file) == 0) as i32, std::ptr::null());
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
        (*p).close_function = Some(io_noclose as unsafe extern "C" fn(*mut State) -> i32);
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
    file: *mut FILE,
    k: *const i8,
    fname: *const i8,
) {
    unsafe {
        let p: *mut Stream = newprefile(state);
        (*p).file = file;
        (*p).close_function = Some(io_noclose as unsafe extern "C" fn(*mut State) -> i32);
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
