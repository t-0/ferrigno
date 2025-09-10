use crate::utility::c::*;
use crate::io::stream::*;
use crate::interpreter::*;
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
        return (*mode as i32 != Character::Null as i32
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
pub unsafe extern "C" fn io_type(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let p: *mut Stream;
        lual_checkany(interpreter, 1);
        p = lual_testudata(interpreter, 1, b"FILE*\0" as *const u8 as *const i8) as *mut Stream;
        if p.is_null() {
            (*interpreter).push_nil();
        } else if ((*p).close_function).is_none() {
            lua_pushstring(interpreter, b"closed file\0" as *const u8 as *const i8);
        } else {
            lua_pushstring(interpreter, b"file\0" as *const u8 as *const i8);
        }
        return 1;
    }
}
pub unsafe extern "C" fn f_tostring(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let p: *mut Stream =
            lual_checkudata(interpreter, 1, b"FILE*\0" as *const u8 as *const i8) as *mut Stream;
        if ((*p).close_function).is_none() {
            lua_pushstring(interpreter, b"file (closed)\0" as *const u8 as *const i8);
        } else {
            lua_pushfstring(interpreter, b"file (%p)\0" as *const u8 as *const i8, (*p).file);
        }
        return 1;
    }
}
pub unsafe extern "C" fn tofile(interpreter: *mut Interpreter) -> *mut FILE {
    unsafe {
        let p: *mut Stream =
            lual_checkudata(interpreter, 1, b"FILE*\0" as *const u8 as *const i8) as *mut Stream;
        if (((*p).close_function).is_none() as i32 != 0) as i64 != 0 {
            lual_error(
                interpreter,
                b"attempt to use a closed file\0".as_ptr(),
            );
        }
        return (*p).file;
    }
}
pub unsafe extern "C" fn newprefile(interpreter: *mut Interpreter) -> *mut Stream {
    unsafe {
        let p: *mut Stream =
            User::lua_newuserdatauv(interpreter, ::core::mem::size_of::<Stream>(), 0) as *mut Stream;
        (*p).close_function = None;
        lual_setmetatable(interpreter, b"FILE*\0" as *const u8 as *const i8);
        return p;
    }
}
pub unsafe extern "C" fn aux_close(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let p: *mut Stream =
            lual_checkudata(interpreter, 1, b"FILE*\0" as *const u8 as *const i8) as *mut Stream;
        let cf: CFunction = (*p).close_function;
        (*p).close_function = None;
        return (Some(cf.expect("non-null function pointer"))).expect("non-null function pointer")(
            interpreter,
        );
    }
}
pub unsafe extern "C" fn f_close(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        tofile(interpreter);
        return aux_close(interpreter);
    }
}
pub unsafe extern "C" fn io_close(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        if lua_type(interpreter, 1) == None {
            lua_getfield(
                interpreter,
                -(1000000 as i32) - 1000 as i32,
                b"_IO_output\0" as *const u8 as *const i8,
            );
        }
        return f_close(interpreter);
    }
}
pub unsafe extern "C" fn f_gc(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let p: *mut Stream =
            lual_checkudata(interpreter, 1, b"FILE*\0" as *const u8 as *const i8) as *mut Stream;
        if ((*p).close_function).is_some() && !((*p).file).is_null() {
            aux_close(interpreter);
        }
        return 0;
    }
}
pub unsafe extern "C" fn io_fclose(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let p: *mut Stream =
            lual_checkudata(interpreter, 1, b"FILE*\0" as *const u8 as *const i8) as *mut Stream;
        *__errno_location() = 0;
        return lual_fileresult(interpreter, (fclose((*p).file) == 0) as i32, std::ptr::null());
    }
}
pub unsafe extern "C" fn newfile(interpreter: *mut Interpreter) -> *mut Stream {
    unsafe {
        let p: *mut Stream = newprefile(interpreter);
        (*p).file = std::ptr::null_mut();
        (*p).close_function = Some(io_fclose as unsafe extern "C" fn(*mut Interpreter) -> i32);
        return p;
    }
}
pub unsafe extern "C" fn opencheck(interpreter: *mut Interpreter, fname: *const i8, mode: *const i8) {
    unsafe {
        let p: *mut Stream = newfile(interpreter);
        (*p).file = fopen(fname, mode);
        if (((*p).file == std::ptr::null_mut() as *mut FILE) as i32 != 0) as i64 != 0 {
            lual_error(
                interpreter,
                b"cannot open file '%s' (%s)\0".as_ptr(),
                fname,
                strerror(*__errno_location()),
            );
        }
    }
}
pub unsafe extern "C" fn io_open(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let filename: *const i8 = lual_checklstring(interpreter, 1, std::ptr::null_mut());
        let mode: *const i8 = lual_optlstring(
            interpreter,
            2,
            b"r\0" as *const u8 as *const i8,
            std::ptr::null_mut(),
        );
        let p: *mut Stream = newfile(interpreter);
        let md: *const i8 = mode;
        ((l_checkmode(md) != 0) as i64 != 0
            || lual_argerror(interpreter, 2, b"invalid mode\0" as *const u8 as *const i8) != 0)
            as i32;
        *__errno_location() = 0;
        (*p).file = fopen(filename, mode);
        return if ((*p).file).is_null() {
            lual_fileresult(interpreter, 0, filename)
        } else {
            1
        };
    }
}
pub unsafe extern "C" fn io_pclose(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let p: *mut Stream =
            lual_checkudata(interpreter, 1, b"FILE*\0" as *const u8 as *const i8) as *mut Stream;
        *__errno_location() = 0;
        return lual_execresult(interpreter, pclose((*p).file));
    }
}
pub unsafe extern "C" fn io_popen(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let filename: *const i8 = lual_checklstring(interpreter, 1, std::ptr::null_mut());
        let mode: *const i8 = lual_optlstring(
            interpreter,
            2,
            b"r\0" as *const u8 as *const i8,
            std::ptr::null_mut(),
        );
        let p: *mut Stream = newprefile(interpreter);
        ((((*mode.offset(0 as isize) as i32 == CHARACTER_LOWER_R as i32
            || *mode.offset(0 as isize) as i32 == CHARACTER_LOWER_W as i32)
            && *mode.offset(1 as isize) as i32 == Character::Null as i32) as i32
            != 0) as i64
            != 0
            || lual_argerror(interpreter, 2, b"invalid mode\0" as *const u8 as *const i8) != 0)
            as i32;
        *__errno_location() = 0;
        fflush(std::ptr::null_mut());
        (*p).file = popen(filename, mode);
        (*p).close_function = Some(io_pclose as unsafe extern "C" fn(*mut Interpreter) -> i32);
        return if ((*p).file).is_null() {
            lual_fileresult(interpreter, 0, filename)
        } else {
            1
        };
    }
}
pub unsafe extern "C" fn io_tmpfile(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let p: *mut Stream = newfile(interpreter);
        *__errno_location() = 0;
        (*p).file = tmpfile();
        return if ((*p).file).is_null() {
            lual_fileresult(interpreter, 0, std::ptr::null())
        } else {
            1
        };
    }
}
pub unsafe extern "C" fn getiofile(interpreter: *mut Interpreter, findex: *const i8) -> *mut FILE {
    unsafe {
        let p: *mut Stream;
        lua_getfield(interpreter, -(1000000 as i32) - 1000 as i32, findex);
        p = lua_touserdata(interpreter, -1) as *mut Stream;
        if (((*p).close_function).is_none() as i32 != 0) as i64 != 0 {
            lual_error(
                interpreter,
                b"default %s file is closed\0".as_ptr(),
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
pub unsafe extern "C" fn g_iofile(interpreter: *mut Interpreter, f: *const i8, mode: *const i8) -> i32 {
    unsafe {
        if !(is_none_or_nil(lua_type(interpreter, 1))) {
            let filename: *const i8 = lua_tolstring(interpreter, 1, std::ptr::null_mut());
            if !filename.is_null() {
                opencheck(interpreter, filename, mode);
            } else {
                tofile(interpreter);
                lua_pushvalue(interpreter, 1);
            }
            lua_setfield(interpreter, -(1000000 as i32) - 1000 as i32, f);
        }
        lua_getfield(interpreter, -(1000000 as i32) - 1000 as i32, f);
        return 1;
    }
}
pub unsafe extern "C" fn io_input(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        return g_iofile(
            interpreter,
            b"_IO_input\0" as *const u8 as *const i8,
            b"r\0" as *const u8 as *const i8,
        );
    }
}
pub unsafe extern "C" fn io_output(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        return g_iofile(
            interpreter,
            b"_IO_output\0" as *const u8 as *const i8,
            b"w\0" as *const u8 as *const i8,
        );
    }
}
pub unsafe extern "C" fn aux_lines(interpreter: *mut Interpreter, to_close: bool) {
    unsafe {
        let n: i32 = (*interpreter).get_top() - 1;
        (((n <= 250 as i32) as i32 != 0) as i64 != 0
            || lual_argerror(
                interpreter,
                250 as i32 + 2,
                b"too many arguments\0" as *const u8 as *const i8,
            ) != 0) as i32;
        lua_pushvalue(interpreter, 1);
        (*interpreter).push_integer(n as i64);
        (*interpreter).push_boolean(to_close);
        lua_rotate(interpreter, 2, 3);
        lua_pushcclosure(
            interpreter,
            Some(io_readline as unsafe extern "C" fn(*mut Interpreter) -> i32),
            3 + n,
        );
    }
}
pub unsafe extern "C" fn f_lines(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        tofile(interpreter);
        aux_lines(interpreter, false);
        return 1;
    }
}
pub unsafe extern "C" fn io_lines(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let to_close: bool;
        if lua_type(interpreter, 1) == None {
            (*interpreter).push_nil();
        }
        if lua_type(interpreter, 1) == Some(TagType::Nil) {
            lua_getfield(
                interpreter,
                -(1000000 as i32) - 1000 as i32,
                b"_IO_input\0" as *const u8 as *const i8,
            );
            lua_copy(interpreter, -1, 1);
            lua_settop(interpreter, -2);
            tofile(interpreter);
            to_close = false;
        } else {
            let filename: *const i8 = lual_checklstring(interpreter, 1, std::ptr::null_mut());
            opencheck(interpreter, filename, b"r\0" as *const u8 as *const i8);
            lua_copy(interpreter, -1, 1);
            lua_settop(interpreter, -2);
            to_close = true;
        }
        aux_lines(interpreter, to_close);
        if to_close {
            (*interpreter).push_nil();
            (*interpreter).push_nil();
            lua_pushvalue(interpreter, 1);
            return 4;
        } else {
            return 1;
        };
    }
}
pub unsafe extern "C" fn nextc(rn: *mut RN) -> i32 {
    unsafe {
        if (((*rn).n >= 200 as i32) as i32 != 0) as i64 != 0 {
            (*rn).buffer[0] = Character::Null as i8;
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
pub unsafe extern "C" fn read_number(interpreter: *mut Interpreter, file: *mut FILE) -> i32 {
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
        rn.buffer[rn.n as usize] = Character::Null as i8;
        if (lua_stringtonumber(interpreter, (rn.buffer).as_mut_ptr()) != 0) as i64 != 0 {
            return 1;
        } else {
            (*interpreter).push_nil();
            return 0;
        };
    }
}
pub unsafe extern "C" fn test_eof(interpreter: *mut Interpreter, file: *mut FILE) -> i32 {
    unsafe {
        let c: i32 = getc(file);
        ungetc(c, file);
        lua_pushstring(interpreter, b"\0" as *const u8 as *const i8);
        return (c != -1) as i32;
    }
}
pub unsafe extern "C" fn read_line(interpreter: *mut Interpreter, file: *mut FILE, chop: i32) -> i32 {
    unsafe {
        let mut b = Buffer::new();
        let mut c: i32 = 0;
        b.initialize(interpreter);
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
            b.vector.length = (b.vector.length as usize).wrapping_add(i as usize) as i32;
            if !(c != -1 && c != CHARACTER_LF as i32) {
                break;
            }
        }
        if chop == 0 && c == CHARACTER_LF as i32 {
            (b.vector.length < b.vector.size || !(b.prepare_with_size(1)).is_null()) as i32;
            let fresh154 = b.vector.length;
            b.vector.length = (b.vector.length).wrapping_add(1);
            *(b.vector.pointer).offset(fresh154 as isize) = c as i8;
        }
        b.push_result();
        return (c == CHARACTER_LF as i32 || get_length_raw(interpreter, -1) > 0) as u64 as u32 as i32;
    }
}
pub unsafe extern "C" fn read_all(interpreter: *mut Interpreter, file: *mut FILE) {
    unsafe {
        let mut nr: u64;
        let mut b = Buffer::new();
        b.initialize(interpreter);
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
            b.vector.length = (b.vector.length as usize).wrapping_add(nr as usize) as i32;
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
pub unsafe extern "C" fn read_chars(interpreter: *mut Interpreter, file: *mut FILE, n: u64) -> i32 {
    unsafe {
        let nr: u64;
        let p: *mut i8;
        let mut b = Buffer::new();
        b.initialize(interpreter);
        p = b.prepare_with_size(n as usize);
        nr = fread(
            p as *mut libc::c_void,
            ::core::mem::size_of::<i8>() as u64,
            n,
            file,
        );
        b.vector.length = (b.vector.length as usize).wrapping_add(nr as usize) as i32;
        b.push_result();
        return (nr > 0) as i32;
    }
}
pub unsafe extern "C" fn g_read(interpreter: *mut Interpreter, file: *mut FILE, first: i32) -> i32 {
    unsafe {
        let mut nargs: i32 = (*interpreter).get_top() - 1;
        let mut n: i32;
        let mut success: i32;
        clearerr(file);
        *__errno_location() = 0;
        if nargs == 0 {
            success = read_line(interpreter, file, 1);
            n = first + 1;
        } else {
            lual_checkstack(
                interpreter,
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
                if lua_type(interpreter, n) == Some(TagType::Numeric) {
                    let l: u64 = lual_checkinteger(interpreter, n) as u64;
                    success = if l == 0 {
                        test_eof(interpreter, file)
                    } else {
                        read_chars(interpreter, file, l)
                    };
                } else {
                    let mut p: *const i8 = lual_checklstring(interpreter, n, std::ptr::null_mut());
                    if *p as i32 == CHARACTER_ASTERISK as i32 {
                        p = p.offset(1);
                    }
                    match *p as i32 {
                        110 => {
                            success = read_number(interpreter, file);
                        }
                        108 => {
                            success = read_line(interpreter, file, 1);
                        }
                        76 => {
                            success = read_line(interpreter, file, 0);
                        }
                        97 => {
                            read_all(interpreter, file);
                            success = 1;
                        }
                        _ => {
                            return lual_argerror(
                                interpreter,
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
            return lual_fileresult(interpreter, 0, std::ptr::null());
        }
        if success == 0 {
            lua_settop(interpreter, -2);
            (*interpreter).push_nil();
        }
        return n - first;
    }
}
pub unsafe extern "C" fn io_read(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        return g_read(
            interpreter,
            getiofile(interpreter, b"_IO_input\0" as *const u8 as *const i8),
            1,
        );
    }
}
pub unsafe extern "C" fn f_read(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        return g_read(interpreter, tofile(interpreter), 2);
    }
}
pub unsafe extern "C" fn io_readline(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let p: *mut Stream =
            lua_touserdata(interpreter, -(1000000 as i32) - 1000 as i32 - 1) as *mut Stream;
        let mut n: i32 = lua_tointegerx(
            interpreter,
            -(1000000 as i32) - 1000 as i32 - 2,
            std::ptr::null_mut(),
        ) as i32;
        if ((*p).close_function).is_none() {
            return lual_error(interpreter, b"file is already closed\0".as_ptr());
        }
        lua_settop(interpreter, 1);
        lual_checkstack(interpreter, n, b"too many arguments\0" as *const u8 as *const i8);
        for i in 1..(1 + n) {
            lua_pushvalue(interpreter, -(1000000 as i32) - 1000 as i32 - (3 + i));
        }
        n = g_read(interpreter, (*p).file, 2);
        if lua_toboolean(interpreter, -n) != 0 {
            return n;
        } else {
            if n > 1 {
                return lual_error(
                    interpreter,
                    b"%s\0".as_ptr(),
                    lua_tolstring(interpreter, -n + 1, std::ptr::null_mut()),
                );
            }
            if lua_toboolean(interpreter, -(1000000 as i32) - 1000 as i32 - 3) != 0 {
                lua_settop(interpreter, 0);
                lua_pushvalue(interpreter, -(1000000 as i32) - 1000 as i32 - 1);
                aux_close(interpreter);
            }
            return 0;
        };
    }
}
pub unsafe extern "C" fn g_write(interpreter: *mut Interpreter, file: *mut FILE, mut arg: i32) -> i32 {
    unsafe {
        let mut nargs: i32 = (*interpreter).get_top() - arg;
        let mut status: i32 = 1;
        *__errno_location() = 0;
        loop {
            let fresh156 = nargs;
            nargs = nargs - 1;
            if !(fresh156 != 0) {
                break;
            }
            if lua_type(interpreter, arg) == Some(TagType::Numeric) {
                let length: i32 = if lua_isinteger(interpreter, arg) {
                    fprintf(
                        file,
                        b"%lld\0" as *const u8 as *const i8,
                        lua_tointegerx(interpreter, arg, std::ptr::null_mut()),
                    )
                } else {
                    fprintf(
                        file,
                        b"%.14g\0" as *const u8 as *const i8,
                        lua_tonumberx(interpreter, arg, std::ptr::null_mut()),
                    )
                };
                status = (status != 0 && length > 0) as i32;
            } else {
                let mut l: u64 = 0;
                let s: *const i8 = lual_checklstring(interpreter, arg, &mut l);
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
            return lual_fileresult(interpreter, status, std::ptr::null());
        };
    }
}
pub unsafe extern "C" fn io_write(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        return g_write(
            interpreter,
            getiofile(interpreter, b"_IO_output\0" as *const u8 as *const i8),
            1,
        );
    }
}
pub unsafe extern "C" fn f_write(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let file: *mut FILE = tofile(interpreter);
        lua_pushvalue(interpreter, 1);
        return g_write(interpreter, file, 2);
    }
}
pub unsafe extern "C" fn f_seek(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        pub const MODE: [i32; 3] = [0, 1, 2];
        pub const MODE_NAMES: [*const i8; 4] = [
            b"set\0" as *const u8 as *const i8,
            b"cur\0" as *const u8 as *const i8,
            b"end\0" as *const u8 as *const i8,
            std::ptr::null(),
        ];
        let file: *mut FILE = tofile(interpreter);
        let mut op: i32 = lual_checkoption(
            interpreter,
            2,
            b"cur\0" as *const u8 as *const i8,
            MODE_NAMES.as_ptr(),
        );
        let p3: i64 = lual_optinteger(interpreter, 3, 0);
        let offset: i64 = p3 as i64;
        (((offset as i64 == p3) as i32 != 0) as i64 != 0
            || lual_argerror(
                interpreter,
                3,
                b"not an integer in proper range\0" as *const u8 as *const i8,
            ) != 0) as i32;
        *__errno_location() = 0;
        op = fseeko(file, offset, MODE[op as usize]);
        if (op != 0) as i64 != 0 {
            return lual_fileresult(interpreter, 0, std::ptr::null());
        } else {
            (*interpreter).push_integer(ftello(file) as i64);
            return 1;
        };
    }
}
pub unsafe extern "C" fn f_setvbuf(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        pub const MODE: [i32; 3] = [2, 0, 1];
        pub const MODE_NAMES: [*const i8; 4] = [
            b"no\0" as *const u8 as *const i8,
            b"full\0" as *const u8 as *const i8,
            b"line\0" as *const u8 as *const i8,
            std::ptr::null(),
        ];
        let file: *mut FILE = tofile(interpreter);
        let op: i32 = lual_checkoption(interpreter, 2, std::ptr::null(), MODE_NAMES.as_ptr());
        let size: i64 = lual_optinteger(
            interpreter,
            3,
            (16 as u64)
                .wrapping_mul(::core::mem::size_of::<*mut libc::c_void>() as u64)
                .wrapping_mul(::core::mem::size_of::<f64>() as u64) as i64,
        );
        let res: i32;
        *__errno_location() = 0;
        res = setvbuf(file, std::ptr::null_mut(), MODE[op as usize], size as u64);
        return lual_fileresult(interpreter, (res == 0) as i32, std::ptr::null());
    }
}
pub unsafe extern "C" fn io_flush(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let file: *mut FILE = getiofile(interpreter, b"_IO_output\0" as *const u8 as *const i8);
        *__errno_location() = 0;
        return lual_fileresult(interpreter, (fflush(file) == 0) as i32, std::ptr::null());
    }
}
pub unsafe extern "C" fn f_flush(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let file: *mut FILE = tofile(interpreter);
        *__errno_location() = 0;
        return lual_fileresult(interpreter, (fflush(file) == 0) as i32, std::ptr::null());
    }
}
pub const IO_FUNCTIONS: [RegisteredFunction; 12] = {
    [
        {
            RegisteredFunction {
                name: b"close\0" as *const u8 as *const i8,
                function: Some(io_close as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"flush\0" as *const u8 as *const i8,
                function: Some(io_flush as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"input\0" as *const u8 as *const i8,
                function: Some(io_input as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"lines\0" as *const u8 as *const i8,
                function: Some(io_lines as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"open\0" as *const u8 as *const i8,
                function: Some(io_open as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"output\0" as *const u8 as *const i8,
                function: Some(io_output as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"popen\0" as *const u8 as *const i8,
                function: Some(io_popen as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"read\0" as *const u8 as *const i8,
                function: Some(io_read as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"tmpfile\0" as *const u8 as *const i8,
                function: Some(io_tmpfile as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"type\0" as *const u8 as *const i8,
                function: Some(io_type as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"write\0" as *const u8 as *const i8,
                function: Some(io_write as unsafe extern "C" fn(*mut Interpreter) -> i32),
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
                function: Some(f_read as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"write\0" as *const u8 as *const i8,
                function: Some(f_write as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"lines\0" as *const u8 as *const i8,
                function: Some(f_lines as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"flush\0" as *const u8 as *const i8,
                function: Some(f_flush as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"seek\0" as *const u8 as *const i8,
                function: Some(f_seek as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"close\0" as *const u8 as *const i8,
                function: Some(f_close as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"setvbuf\0" as *const u8 as *const i8,
                function: Some(f_setvbuf as unsafe extern "C" fn(*mut Interpreter) -> i32),
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
                function: Some(f_gc as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"__close\0" as *const u8 as *const i8,
                function: Some(f_gc as unsafe extern "C" fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: b"__tostring\0" as *const u8 as *const i8,
                function: Some(f_tostring as unsafe extern "C" fn(*mut Interpreter) -> i32),
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
pub unsafe extern "C" fn createmeta(interpreter: *mut Interpreter) {
    unsafe {
        lual_newmetatable(interpreter, b"FILE*\0" as *const u8 as *const i8);
        lual_setfuncs(interpreter, IO_METAMETHODS.as_ptr(), 0);
        (*interpreter).lua_createtable();
        lual_setfuncs(interpreter, IO_METHODS.as_ptr(), 0);
        lua_setfield(interpreter, -2, b"__index\0" as *const u8 as *const i8);
        lua_settop(interpreter, -2);
    }
}
pub unsafe extern "C" fn io_noclose(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let p: *mut Stream =
            lual_checkudata(interpreter, 1, b"FILE*\0" as *const u8 as *const i8) as *mut Stream;
        (*p).close_function = Some(io_noclose as unsafe extern "C" fn(*mut Interpreter) -> i32);
        (*interpreter).push_nil();
        lua_pushstring(
            interpreter,
            b"cannot close standard file\0" as *const u8 as *const i8,
        );
        return 2;
    }
}
pub unsafe extern "C" fn createstdfile(
    interpreter: *mut Interpreter,
    file: *mut FILE,
    k: *const i8,
    fname: *const i8,
) {
    unsafe {
        let p: *mut Stream = newprefile(interpreter);
        (*p).file = file;
        (*p).close_function = Some(io_noclose as unsafe extern "C" fn(*mut Interpreter) -> i32);
        if !k.is_null() {
            lua_pushvalue(interpreter, -1);
            lua_setfield(interpreter, -(1000000 as i32) - 1000 as i32, k);
        }
        lua_setfield(interpreter, -2, fname);
    }
}
pub unsafe extern "C" fn luaopen_io(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        lual_checkversion_(
            interpreter,
            504.0,
            (::core::mem::size_of::<i64>() as u64)
                .wrapping_mul(16 as u64)
                .wrapping_add(::core::mem::size_of::<f64>() as u64),
        );
        (*interpreter).lua_createtable();
        lual_setfuncs(interpreter, IO_FUNCTIONS.as_ptr(), 0);
        createmeta(interpreter);
        createstdfile(
            interpreter,
            stdin,
            b"_IO_input\0" as *const u8 as *const i8,
            b"stdin\0" as *const u8 as *const i8,
        );
        createstdfile(
            interpreter,
            stdout,
            b"_IO_output\0" as *const u8 as *const i8,
            b"stdout\0" as *const u8 as *const i8,
        );
        createstdfile(
            interpreter,
            stderr,
            std::ptr::null(),
            b"stderr\0" as *const u8 as *const i8,
        );
        return 1;
    }
}
