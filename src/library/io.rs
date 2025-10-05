use crate::buffer::*;
use crate::c::*;
use crate::character::*;
use crate::functions::*;
use crate::interpreter::*;
use crate::registeredfunction::*;
use std::io::Write;
use crate::rn::*;
use crate::stream::*;
use crate::tag::*;
use crate::tdefaultnew::*;
use crate::user::*;
use std::ptr::*;
pub unsafe fn l_checkmode(mut mode: *const i8) -> i32 {
    unsafe {
        return (*mode as i32 != Character::Null as i32
            && {
                let fresh151 = mode;
                mode = mode.offset(1);
                !(libc::strchr(c"rwa".as_ptr(), *fresh151 as i32)).is_null()
            }
            && (*mode as i32 != Character::Plus as i32 || {
                mode = mode.offset(1);
                1 != 0
            })
            && libc::strspn(mode, c"b".as_ptr()) == libc::strlen(mode)) as i32;
    }
}
pub unsafe fn io_type(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let p: *mut Stream;
        lual_checkany(interpreter, 1);
        p = lual_testudata(interpreter, 1, c"FILE*".as_ptr()) as *mut Stream;
        if p.is_null() {
            (*interpreter).push_nil();
        } else if ((*p).stream_cfunctionclose).is_none() {
            lua_pushstring(interpreter, c"closed file".as_ptr());
        } else {
            lua_pushstring(interpreter, c"file".as_ptr());
        }
        return 1;
    }
}
pub unsafe fn f_tostring(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let p: *mut Stream = lual_checkudata(interpreter, 1, c"FILE*".as_ptr()) as *mut Stream;
        if ((*p).stream_cfunctionclose).is_none() {
            lua_pushstring(interpreter, c"file (closed)".as_ptr());
        } else {
            lua_pushfstring(interpreter, c"file (%p)".as_ptr(), (*p).stream_file);
        }
        return 1;
    }
}
pub unsafe fn tofile(interpreter: *mut Interpreter) -> *mut libc::FILE {
    unsafe {
        let p: *mut Stream = lual_checkudata(interpreter, 1, c"FILE*".as_ptr()) as *mut Stream;
        if (*p).stream_cfunctionclose.is_none() {
            lual_error(interpreter, c"attempt to use a closed file".as_ptr());
        }
        return (*p).stream_file;
    }
}
pub unsafe fn newprefile(interpreter: *mut Interpreter) -> *mut Stream {
    unsafe {
        let p: *mut Stream = User::lua_newuserdatauv(interpreter, size_of::<Stream>(), 0) as *mut Stream;
        (*p).stream_cfunctionclose = None;
        lual_setmetatable(interpreter, c"FILE*".as_ptr());
        return p;
    }
}
pub unsafe fn aux_close(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let p: *mut Stream = lual_checkudata(interpreter, 1, c"FILE*".as_ptr()) as *mut Stream;
        let cfunction_close: CFunction = (*p).stream_cfunctionclose;
        (*p).stream_cfunctionclose = None;
        return (Some(cfunction_close.expect("non-null function pointer"))).expect("non-null function pointer")(interpreter);
    }
}
pub unsafe fn f_close(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        tofile(interpreter);
        return aux_close(interpreter);
    }
}
pub unsafe fn io_close(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        if lua_type(interpreter, 1) == None {
            lua_getfield(interpreter, -(1000000 as i32) - 1000 as i32, c"_IO_output".as_ptr());
        }
        return f_close(interpreter);
    }
}
pub unsafe fn f_gc(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let p: *mut Stream = lual_checkudata(interpreter, 1, c"FILE*".as_ptr()) as *mut Stream;
        if ((*p).stream_cfunctionclose).is_some() && !((*p).stream_file).is_null() {
            aux_close(interpreter);
        }
        return 0;
    }
}
pub unsafe fn io_fclose(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let p: *mut Stream = lual_checkudata(interpreter, 1, c"FILE*".as_ptr()) as *mut Stream;
        *libc::__errno_location() = 0;
        return lual_fileresult(interpreter, (libc::fclose((*p).stream_file) == 0) as i32, null());
    }
}
pub unsafe fn newfile(interpreter: *mut Interpreter) -> *mut Stream {
    unsafe {
        let p: *mut Stream = newprefile(interpreter);
        (*p).stream_file = null_mut();
        (*p).stream_cfunctionclose = Some(io_fclose as unsafe fn(*mut Interpreter) -> i32);
        return p;
    }
}
pub unsafe fn opencheck(interpreter: *mut Interpreter, fname: *const i8, mode: *const i8) {
    unsafe {
        let p: *mut Stream = newfile(interpreter);
        (*p).stream_file = libc::fopen(fname, mode) as *mut libc::FILE;
        if (*p).stream_file.is_null() {
            lual_error(
                interpreter,
                c"cannot open file '%s' (%s)".as_ptr(),
                fname,
                libc::strerror(*libc::__errno_location()),
            );
        }
    }
}
pub unsafe fn io_open(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let filename: *const i8 = lual_checklstring(interpreter, 1, null_mut());
        let mode: *const i8 = lual_optlstring(interpreter, 2, c"r".as_ptr(), null_mut());
        let p: *mut Stream = newfile(interpreter);
        let md: *const i8 = mode;
        ((l_checkmode(md) != 0) as i64 != 0 || lual_argerror(interpreter, 2, c"invalid mode".as_ptr()) != 0) as i32;
        *libc::__errno_location() = 0;
        (*p).stream_file = libc::fopen(filename, mode) as *mut libc::FILE;
        return if ((*p).stream_file).is_null() {
            lual_fileresult(interpreter, 0, filename)
        } else {
            1
        };
    }
}
pub unsafe fn io_pclose(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let p: *mut Stream = lual_checkudata(interpreter, 1, c"FILE*".as_ptr()) as *mut Stream;
        *libc::__errno_location() = 0;
        return lual_execresult(interpreter, libc::pclose((*p).stream_file));
    }
}
pub unsafe fn io_popen(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let filename: *const i8 = lual_checklstring(interpreter, 1, null_mut());
        let mode: *const i8 = lual_optlstring(interpreter, 2, c"r".as_ptr(), null_mut());
        let p: *mut Stream = newprefile(interpreter);
        ((((*mode.offset(0 as isize) as i32 == Character::LowerR as i32
            || *mode.offset(0 as isize) as i32 == Character::LowerW as i32)
            && *mode.offset(1 as isize) as i32 == Character::Null as i32) as i32
            != 0) as i64
            != 0
            || lual_argerror(interpreter, 2, c"invalid mode".as_ptr()) != 0) as i32;
        *libc::__errno_location() = 0;
        std::io::stdout().flush().unwrap();
        std::io::stderr().flush().unwrap();
        (*p).stream_file = libc::popen(filename, mode) as *mut libc::FILE;
        (*p).stream_cfunctionclose = Some(io_pclose as unsafe fn(*mut Interpreter) -> i32);
        return if ((*p).stream_file).is_null() {
            lual_fileresult(interpreter, 0, filename)
        } else {
            1
        };
    }
}
pub unsafe fn io_tmpfile(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let p: *mut Stream = newfile(interpreter);
        *libc::__errno_location() = 0;
        (*p).stream_file = libc::tmpfile();
        return if ((*p).stream_file).is_null() {
            lual_fileresult(interpreter, 0, null())
        } else {
            1
        };
    }
}
pub unsafe fn getiofile(interpreter: *mut Interpreter, findex: *const i8) -> *mut libc::FILE {
    unsafe {
        let p: *mut Stream;
        lua_getfield(interpreter, -(1000000 as i32) - 1000 as i32, findex);
        p = (*interpreter).to_pointer(-1) as *mut Stream;
        if (*p).stream_cfunctionclose.is_none() {
            lual_error(interpreter, c"default %s file is closed".as_ptr(), findex.offset(4));
        }
        return (*p).stream_file;
    }
}
pub unsafe fn g_iofile(interpreter: *mut Interpreter, f: *const i8, mode: *const i8) -> i32 {
    unsafe {
        if !(TagType::is_none_or_nil(lua_type(interpreter, 1))) {
            let filename: *const i8 = lua_tolstring(interpreter, 1, null_mut());
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
pub unsafe fn io_input(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        return g_iofile(interpreter, c"_IO_input".as_ptr(), c"r".as_ptr());
    }
}
pub unsafe fn io_output(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        return g_iofile(interpreter, c"_IO_output".as_ptr(), c"w".as_ptr());
    }
}
pub unsafe fn aux_lines(interpreter: *mut Interpreter, to_close: bool) {
    unsafe {
        let n: i32 = (*interpreter).get_top() - 1;
        (((n <= 250 as i32) as i32 != 0) as i64 != 0
            || lual_argerror(interpreter, 250 as i32 + 2, c"too many arguments".as_ptr()) != 0) as i32;
        lua_pushvalue(interpreter, 1);
        (*interpreter).push_integer(n as i64);
        (*interpreter).push_boolean(to_close);
        lua_rotate(interpreter, 2, 3);
        lua_pushcclosure(interpreter, Some(io_readline as unsafe fn(*mut Interpreter) -> i32), 3 + n);
    }
}
pub unsafe fn f_lines(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        tofile(interpreter);
        aux_lines(interpreter, false);
        return 1;
    }
}
pub unsafe fn io_lines(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let to_close: bool;
        if lua_type(interpreter, 1) == None {
            (*interpreter).push_nil();
        }
        if lua_type(interpreter, 1) == Some(TagType::Nil) {
            lua_getfield(interpreter, -(1000000 as i32) - 1000 as i32, c"_IO_input".as_ptr());
            lua_copy(interpreter, -1, 1);
            lua_settop(interpreter, -2);
            tofile(interpreter);
            to_close = false;
        } else {
            let filename: *const i8 = lual_checklstring(interpreter, 1, null_mut());
            opencheck(interpreter, filename, c"r".as_ptr());
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
pub unsafe fn nextc(rn: *mut RN) -> i32 {
    unsafe {
        if (*rn).rn_n >= 200 {
            (*rn).rn_buffer[0] = Character::Null as i8;
            return 0;
        } else {
            let fresh152 = (*rn).rn_n;
            (*rn).rn_n = (*rn).rn_n + 1;
            (*rn).rn_buffer[fresh152 as usize] = (*rn).rn_c as i8;
            (*rn).rn_c = libc::fgetc((*rn).rn_file);
            return 1;
        };
    }
}
pub unsafe fn test2(rn: *mut RN, set: *const i8) -> i32 {
    unsafe {
        if (*rn).rn_c == *set.offset(0 as isize) as i32 || (*rn).rn_c == *set.offset(1 as isize) as i32 {
            return nextc(rn);
        } else {
            return 0;
        };
    }
}
pub unsafe fn readdigits(rn: *mut RN, hex: i32) -> i32 {
    unsafe {
        let mut count: i32 = 0;
        while (if hex != 0 {
            Character::from((*rn).rn_c as i32).is_digit_hexadecimal()
        } else {
            Character::from((*rn).rn_c as i32).is_digit_decimal()
        })
            && nextc(rn) != 0
        {
            count += 1;
        }
        return count;
    }
}
pub unsafe fn read_number(interpreter: *mut Interpreter, file: *mut libc::FILE) -> i32 {
    unsafe {
        let mut rn: RN = RN { rn_file: null_mut(), rn_c: 0, rn_n: 0, rn_buffer: [0; 201] };
        let mut count: i32 = 0;
        let mut hex: i32 = 0;
        let mut decp: [i8; 2] = [0; 2];
        rn.rn_file = file;
        rn.rn_n = 0;
        decp[0] = Character::Period as i8;
        decp[1] = Character::Period as i8;
        loop {
            rn.rn_c = libc::fgetc(rn.rn_file);
            if !Character::from(rn.rn_c as i32).is_whitespace() {
                break;
            }
        }
        test2(&mut rn, c"-+".as_ptr());
        if test2(&mut rn, c"00".as_ptr()) != 0 {
            if test2(&mut rn, c"xX".as_ptr()) != 0 {
                hex = 1;
            } else {
                count = 1;
            }
        }
        count += readdigits(&mut rn, hex);
        if test2(&mut rn, decp.as_mut_ptr()) != 0 {
            count += readdigits(&mut rn, hex);
        }
        if count > 0 && test2(&mut rn, if hex != 0 { c"pP".as_ptr() } else { c"eE".as_ptr() }) != 0 {
            test2(&mut rn, c"-+".as_ptr());
            readdigits(&mut rn, 0);
        }
        libc::ungetc(rn.rn_c, rn.rn_file);
        rn.rn_buffer[rn.rn_n as usize] = Character::Null as i8;
        if (lua_stringtonumber(interpreter, (rn.rn_buffer).as_mut_ptr()) != 0) as i64 != 0 {
            return 1;
        } else {
            (*interpreter).push_nil();
            return 0;
        };
    }
}
pub unsafe fn test_eof(interpreter: *mut Interpreter, file: *mut libc::FILE) -> i32 {
    unsafe {
        let c: i32 = libc::fgetc(file);
        libc::ungetc(c, file);
        lua_pushstring(interpreter, c"".as_ptr());
        return (c != -1) as i32;
    }
}
pub unsafe fn read_line(interpreter: *mut Interpreter, file: *mut libc::FILE, chop: i32) -> i32 {
    unsafe {
        let mut b = Buffer::new();
        let mut c: i32 = 0;
        b.initialize(interpreter);
        loop {
            let buffer: *mut i8 = b.prepare_with_size(
                (16 as usize)
                    .wrapping_mul(size_of::<*mut libc::c_void>())
                    .wrapping_mul(size_of::<f64>()),
            );
            let mut i: i32 = 0;
            while i
                < (16 as usize)
                    .wrapping_mul(size_of::<*mut libc::c_void>() as usize)
                    .wrapping_mul(size_of::<f64>() as usize) as i32
                && {
                    c = libc::fgetc(file);
                    c != -1
                }
                && c != Character::LineFeed as i32
            {
                let fresh153 = i;
                i = i + 1;
                *buffer.offset(fresh153 as isize) = c as i8;
            }
            b.buffer_loads
                .set_length(((b.buffer_loads.get_length() as usize).wrapping_add(i as usize) as i32) as usize);
            if !(c != -1 && c != Character::LineFeed as i32) {
                break;
            }
        }
        if chop == 0 && c == Character::LineFeed as i32 {
            (b.buffer_loads.get_length() < b.buffer_loads.get_size() || !(b.prepare_with_size(1)).is_null()) as i32;
            let fresh154 = b.buffer_loads.get_length();
            b.buffer_loads
                .set_length(((b.buffer_loads.get_length()).wrapping_add(1)) as usize);
            *(b.buffer_loads.loads_pointer).offset(fresh154 as isize) = c as i8;
        }
        b.push_result();
        return (c == Character::LineFeed as i32 || get_length_raw(interpreter, -1) > 0) as usize as u32 as i32;
    }
}
pub unsafe fn read_all(interpreter: *mut Interpreter, file: *mut libc::FILE) {
    unsafe {
        let mut nr: usize;
        let mut b = Buffer::new();
        b.initialize(interpreter);
        loop {
            let p: *mut i8 = b.prepare_with_size(
                (16 as usize)
                    .wrapping_mul(size_of::<*mut libc::c_void>())
                    .wrapping_mul(size_of::<f64>()),
            );
            nr = libc::fread(
                p as *mut libc::c_void,
                1,
                (16 as usize)
                    .wrapping_mul(size_of::<*mut libc::c_void>())
                    .wrapping_mul(size_of::<f64>()),
                file,
            ) as usize;
            b.buffer_loads
                .set_length(((b.buffer_loads.get_length() as usize).wrapping_add(nr as usize) as i32) as usize);
            if !(nr
                == (16 as usize)
                    .wrapping_mul(size_of::<*mut libc::c_void>() as usize)
                    .wrapping_mul(size_of::<f64>() as usize) as i32 as usize)
            {
                break;
            }
        }
        b.push_result();
    }
}
pub unsafe fn read_chars(interpreter: *mut Interpreter, file: *mut libc::FILE, n: usize) -> i32 {
    unsafe {
        let nr: usize;
        let p: *mut i8;
        let mut b = Buffer::new();
        b.initialize(interpreter);
        p = b.prepare_with_size(n as usize);
        nr = libc::fread(p as *mut libc::c_void, 1, n as usize, file) as usize;
        b.buffer_loads
            .set_length(((b.buffer_loads.get_length() as usize).wrapping_add(nr as usize) as i32) as usize);
        b.push_result();
        return (nr > 0) as i32;
    }
}
pub unsafe fn g_read(interpreter: *mut Interpreter, file: *mut libc::FILE, first: i32) -> i32 {
    unsafe {
        let mut nargs: i32 = (*interpreter).get_top() - 1;
        let mut n: i32;
        let mut success: i32;
        libc::clearerr(file);
        *libc::__errno_location() = 0;
        if nargs == 0 {
            success = read_line(interpreter, file, 1);
            n = first + 1;
        } else {
            lual_checkstack(interpreter, nargs + 20 as i32, c"too many arguments".as_ptr());
            success = 1;
            n = first;
            loop {
                let fresh155 = nargs;
                nargs = nargs - 1;
                if !(fresh155 != 0 && success != 0) {
                    break;
                }
                if lua_type(interpreter, n) == Some(TagType::Numeric) {
                    let l: usize = lual_checkinteger(interpreter, n) as usize;
                    success = if l == 0 {
                        test_eof(interpreter, file)
                    } else {
                        read_chars(interpreter, file, l)
                    };
                } else {
                    let mut p: *const i8 = lual_checklstring(interpreter, n, null_mut());
                    if *p as i32 == Character::Asterisk as i32 {
                        p = p.offset(1);
                    }
                    match Character::from(*p as i32) {
                        | Character::LowerN => {
                            success = read_number(interpreter, file);
                        },
                        | Character::LowerL => {
                            success = read_line(interpreter, file, 1);
                        },
                        | Character::UpperL => {
                            success = read_line(interpreter, file, 0);
                        },
                        | Character::LowerA => {
                            read_all(interpreter, file);
                            success = 1;
                        },
                        | _ => {
                            return lual_argerror(interpreter, n, c"invalid format".as_ptr());
                        },
                    }
                }
                n += 1;
            }
        }
        if libc::ferror(file) != 0 {
            return lual_fileresult(interpreter, 0, null());
        }
        if success == 0 {
            lua_settop(interpreter, -2);
            (*interpreter).push_nil();
        }
        return n - first;
    }
}
pub unsafe fn io_read(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        return g_read(interpreter, getiofile(interpreter, c"_IO_input".as_ptr()), 1);
    }
}
pub unsafe fn f_read(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        return g_read(interpreter, tofile(interpreter), 2);
    }
}
pub unsafe fn io_readline(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let p: *mut Stream = (*interpreter).to_pointer(-(1000000 as i32) - 1000 as i32 - 1) as *mut Stream;
        let mut n: i32 = lua_tointegerx(interpreter, -(1000000 as i32) - 1000 as i32 - 2, null_mut()) as i32;
        if ((*p).stream_cfunctionclose).is_none() {
            return lual_error(interpreter, c"file is already closed".as_ptr());
        }
        lua_settop(interpreter, 1);
        lual_checkstack(interpreter, n, c"too many arguments".as_ptr());
        for i in 1..(1 + n) {
            lua_pushvalue(interpreter, -(1000000 as i32) - 1000 as i32 - (3 + i));
        }
        n = g_read(interpreter, (*p).stream_file, 2);
        if lua_toboolean(interpreter, -n) {
            return n;
        } else {
            if n > 1 {
                return lual_error(interpreter, c"%s".as_ptr(), lua_tolstring(interpreter, -n + 1, null_mut()));
            }
            if lua_toboolean(interpreter, -(1000000 as i32) - 1000 as i32 - 3) {
                lua_settop(interpreter, 0);
                lua_pushvalue(interpreter, -(1000000 as i32) - 1000 as i32 - 1);
                aux_close(interpreter);
            }
            return 0;
        };
    }
}
pub unsafe fn g_write(interpreter: *mut Interpreter, file: *mut libc::FILE, mut arg: i32) -> i32 {
    unsafe {
        let mut nargs: i32 = (*interpreter).get_top() - arg;
        let mut status: i32 = 1;
        *libc::__errno_location() = 0;
        loop {
            let fresh156 = nargs;
            nargs = nargs - 1;
            if !(fresh156 != 0) {
                break;
            }
            if lua_type(interpreter, arg) == Some(TagType::Numeric) {
                let length: i32 = if lua_isinteger(interpreter, arg) {
                    libc::fprintf(file, c"%lld".as_ptr(), lua_tointegerx(interpreter, arg, null_mut()))
                } else {
                    libc::fprintf(file, c"%.14g".as_ptr(), lua_tonumberx(interpreter, arg, null_mut()))
                };
                status = (status != 0 && length > 0) as i32;
            } else {
                let mut l: usize = 0;
                let s: *const i8 = lual_checklstring(interpreter, arg, &mut l);
                status = (status != 0 && libc::fwrite(s as *const libc::c_void, 1, l as usize, file) == l as usize) as i32;
            }
            arg += 1;
        }
        if (status != 0) as i64 != 0 {
            return 1;
        } else {
            return lual_fileresult(interpreter, status, null());
        };
    }
}
pub unsafe fn io_write(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        return g_write(interpreter, getiofile(interpreter, c"_IO_output".as_ptr()), 1);
    }
}
pub unsafe fn f_write(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let file: *mut libc::FILE = tofile(interpreter);
        lua_pushvalue(interpreter, 1);
        return g_write(interpreter, file, 2);
    }
}
pub unsafe fn f_seek(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        pub const MODE: [i32; 3] = [0, 1, 2];
        pub const MODE_NAMES: [*const i8; 4] = [c"set".as_ptr(), c"cur".as_ptr(), c"end".as_ptr(), null()];
        let file: *mut libc::FILE = tofile(interpreter);
        let mut op: i32 = lual_checkoption(interpreter, 2, c"cur".as_ptr(), MODE_NAMES.as_ptr());
        let p3: i64 = lual_optinteger(interpreter, 3, 0);
        let offset: i64 = p3 as i64;
        (((offset as i64 == p3) as i32 != 0) as i64 != 0
            || lual_argerror(interpreter, 3, c"not an integer in proper range".as_ptr()) != 0) as i32;
        *libc::__errno_location() = 0;
        op = libc::fseeko(file, offset, MODE[op as usize]);
        if (op != 0) as i64 != 0 {
            return lual_fileresult(interpreter, 0, null());
        } else {
            (*interpreter).push_integer(libc::ftello(file) as i64);
            return 1;
        };
    }
}
pub unsafe fn f_setvbuf(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        pub const MODE: [i32; 3] = [2, 0, 1];
        pub const MODE_NAMES: [*const i8; 4] = [c"no".as_ptr(), c"full".as_ptr(), c"line".as_ptr(), null()];
        let file: *mut libc::FILE = tofile(interpreter);
        let op: i32 = lual_checkoption(interpreter, 2, null(), MODE_NAMES.as_ptr());
        let size: i64 = lual_optinteger(
            interpreter,
            3,
            (16 as usize)
                .wrapping_mul(size_of::<*mut libc::c_void>() as usize)
                .wrapping_mul(size_of::<f64>() as usize) as i64,
        );
        let res: i32;
        *libc::__errno_location() = 0;
        res = libc::setvbuf(file, null_mut(), MODE[op as usize], size as usize);
        return lual_fileresult(interpreter, (res == 0) as i32, null());
    }
}
pub unsafe fn io_flush(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let file: *mut libc::FILE = getiofile(interpreter, c"_IO_output".as_ptr());
        *libc::__errno_location() = 0;
        return lual_fileresult(interpreter, (libc::fflush(file) == 0) as i32, null());
    }
}
pub unsafe fn f_flush(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let file: *mut libc::FILE = tofile(interpreter);
        *libc::__errno_location() = 0;
        return lual_fileresult(interpreter, (libc::fflush(file) == 0) as i32, null());
    }
}
pub const IO_FUNCTIONS: [RegisteredFunction; 11] = {
    [
        {
            RegisteredFunction {
                registeredfunction_name: c"close".as_ptr(),
                registeredfunction_function: Some(io_close as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"flush".as_ptr(),
                registeredfunction_function: Some(io_flush as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"input".as_ptr(),
                registeredfunction_function: Some(io_input as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"lines".as_ptr(),
                registeredfunction_function: Some(io_lines as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"open".as_ptr(),
                registeredfunction_function: Some(io_open as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"output".as_ptr(),
                registeredfunction_function: Some(io_output as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"popen".as_ptr(),
                registeredfunction_function: Some(io_popen as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"read".as_ptr(),
                registeredfunction_function: Some(io_read as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"tmpfile".as_ptr(),
                registeredfunction_function: Some(io_tmpfile as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"type".as_ptr(),
                registeredfunction_function: Some(io_type as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"write".as_ptr(),
                registeredfunction_function: Some(io_write as unsafe fn(*mut Interpreter) -> i32),
            }
        },
    ]
};
pub const IO_METHODS: [RegisteredFunction; 7] = {
    [
        {
            RegisteredFunction {
                registeredfunction_name: c"read".as_ptr(),
                registeredfunction_function: Some(f_read as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"write".as_ptr(),
                registeredfunction_function: Some(f_write as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"lines".as_ptr(),
                registeredfunction_function: Some(f_lines as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"flush".as_ptr(),
                registeredfunction_function: Some(f_flush as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"seek".as_ptr(),
                registeredfunction_function: Some(f_seek as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"close".as_ptr(),
                registeredfunction_function: Some(f_close as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"setvbuf".as_ptr(),
                registeredfunction_function: Some(f_setvbuf as unsafe fn(*mut Interpreter) -> i32),
            }
        },
    ]
};
pub const IO_METAMETHODS: [RegisteredFunction; 3] = {
    [
        {
            RegisteredFunction {
                registeredfunction_name: c"__gc".as_ptr(),
                registeredfunction_function: Some(f_gc as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"__close".as_ptr(),
                registeredfunction_function: Some(f_gc as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                registeredfunction_name: c"__tostring".as_ptr(),
                registeredfunction_function: Some(f_tostring as unsafe fn(*mut Interpreter) -> i32),
            }
        },
    ]
};
pub unsafe fn createmeta(interpreter: *mut Interpreter) {
    unsafe {
        lual_newmetatable(interpreter, c"FILE*".as_ptr());
        lual_setfuncs(interpreter, IO_METAMETHODS.as_ptr(), IO_METAMETHODS.len(), 0);
        (*interpreter).lua_createtable();
        lual_setfuncs(interpreter, IO_METHODS.as_ptr(), IO_METHODS.len(), 0);
        lua_setfield(interpreter, -2, c"__index".as_ptr());
        lua_settop(interpreter, -2);
    }
}
pub unsafe fn io_noclose(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let p: *mut Stream = lual_checkudata(interpreter, 1, c"FILE*".as_ptr()) as *mut Stream;
        (*p).stream_cfunctionclose = Some(io_noclose as unsafe fn(*mut Interpreter) -> i32);
        (*interpreter).push_nil();
        lua_pushstring(interpreter, c"cannot close standard file".as_ptr());
        return 2;
    }
}
pub unsafe fn createstdfile(interpreter: *mut Interpreter, file: *mut libc::FILE, k: *const i8, fname: *const i8) {
    unsafe {
        let p: *mut Stream = newprefile(interpreter);
        (*p).stream_file = file;
        (*p).stream_cfunctionclose = Some(io_noclose as unsafe fn(*mut Interpreter) -> i32);
        if !k.is_null() {
            lua_pushvalue(interpreter, -1);
            lua_setfield(interpreter, -(1000000 as i32) - 1000 as i32, k);
        }
        lua_setfield(interpreter, -2, fname);
    }
}
pub unsafe fn luaopen_io(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        (*interpreter).lua_createtable();
        lual_setfuncs(interpreter, IO_FUNCTIONS.as_ptr(), IO_FUNCTIONS.len(), 0);
        createmeta(interpreter);
        createstdfile(interpreter, stdin, c"_IO_input".as_ptr(), c"stdin".as_ptr());
        createstdfile(interpreter, stdout, c"_IO_output".as_ptr(), c"stdout".as_ptr());
        createstdfile(interpreter, stderr, null(), c"stderr".as_ptr());
        return 1;
    }
}
