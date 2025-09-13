use rlua::*;
use crate::buffer::*;
use crate::character::*;
use crate::functions::*;
use crate::interpreter::*;
use crate::io::rn::*;
use crate::io::stream::*;
use crate::new::*;
use crate::registeredfunction::*;
use crate::tag::*;
use crate::user::*;
use crate::utility::c::*;
use std::ptr::*;
pub unsafe fn l_checkmode(mut mode: *const i8) -> i32 {
    unsafe {
        return (*mode as i32 != Character::Null as i32
            && {
                let fresh151 = mode;
                mode = mode.offset(1);
                !(strchr(make_cstring!("rwa"), *fresh151 as i32)).is_null()
            }
            && (*mode as i32 != CHARACTER_PLUS as i32 || {
                mode = mode.offset(1);
                1 != 0
            })
            && strspn(mode, make_cstring!("b")) == strlen(mode))
            as i32;
    }
}
pub unsafe fn io_type(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let p: *mut Stream;
        lual_checkany(interpreter, 1);
        p = lual_testudata(interpreter, 1, make_cstring!("FILE*")) as *mut Stream;
        if p.is_null() {
            (*interpreter).push_nil();
        } else if ((*p).close_function).is_none() {
            lua_pushstring(interpreter, make_cstring!("closed file"));
        } else {
            lua_pushstring(interpreter, make_cstring!("file"));
        }
        return 1;
    }
}
pub unsafe fn f_tostring(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let p: *mut Stream =
            lual_checkudata(interpreter, 1, make_cstring!("FILE*")) as *mut Stream;
        if ((*p).close_function).is_none() {
            lua_pushstring(interpreter, make_cstring!("file (closed)"));
        } else {
            lua_pushfstring(
                interpreter,
                make_cstring!("file (%p)"),
                (*p).file,
            );
        }
        return 1;
    }
}
pub unsafe fn tofile(interpreter: *mut Interpreter) -> *mut FILE {
    unsafe {
        let p: *mut Stream =
            lual_checkudata(interpreter, 1, make_cstring!("FILE*")) as *mut Stream;
        if (*p).close_function.is_none() {
            lual_error(interpreter, make_cstring!("attempt to use a closed file"));
        }
        return (*p).file;
    }
}
pub unsafe fn newprefile(interpreter: *mut Interpreter) -> *mut Stream {
    unsafe {
        let p: *mut Stream =
            User::lua_newuserdatauv(interpreter, size_of::<Stream>(), 0) as *mut Stream;
        (*p).close_function = None;
        lual_setmetatable(interpreter, make_cstring!("FILE*"));
        return p;
    }
}
pub unsafe fn aux_close(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let p: *mut Stream =
            lual_checkudata(interpreter, 1, make_cstring!("FILE*")) as *mut Stream;
        let cf: CFunction = (*p).close_function;
        (*p).close_function = None;
        return (Some(cf.expect("non-null function pointer"))).expect("non-null function pointer")(
            interpreter,
        );
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
            lua_getfield(
                interpreter,
                -(1000000 as i32) - 1000 as i32,
                make_cstring!("_IO_output"),
            );
        }
        return f_close(interpreter);
    }
}
pub unsafe fn f_gc(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let p: *mut Stream =
            lual_checkudata(interpreter, 1, make_cstring!("FILE*")) as *mut Stream;
        if ((*p).close_function).is_some() && !((*p).file).is_null() {
            aux_close(interpreter);
        }
        return 0;
    }
}
pub unsafe fn io_fclose(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let p: *mut Stream =
            lual_checkudata(interpreter, 1, make_cstring!("FILE*")) as *mut Stream;
        *__errno_location() = 0;
        return lual_fileresult(interpreter, (fclose((*p).file) == 0) as i32, null());
    }
}
pub unsafe fn newfile(interpreter: *mut Interpreter) -> *mut Stream {
    unsafe {
        let p: *mut Stream = newprefile(interpreter);
        (*p).file = null_mut();
        (*p).close_function = Some(io_fclose as unsafe fn(*mut Interpreter) -> i32);
        return p;
    }
}
pub unsafe fn opencheck(
    interpreter: *mut Interpreter,
    fname: *const i8,
    mode: *const i8,
) {
    unsafe {
        let p: *mut Stream = newfile(interpreter);
        (*p).file = fopen(fname, mode);
        if (*p).file.is_null() {
            lual_error(
                interpreter,
                make_cstring!("cannot open file '%s' (%s)"),
                fname,
                strerror(*__errno_location()),
            );
        }
    }
}
pub unsafe fn io_open(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let filename: *const i8 = lual_checklstring(interpreter, 1, null_mut());
        let mode: *const i8 =
            lual_optlstring(interpreter, 2, make_cstring!("r"), null_mut());
        let p: *mut Stream = newfile(interpreter);
        let md: *const i8 = mode;
        ((l_checkmode(md) != 0) as i64 != 0
            || lual_argerror(interpreter, 2, make_cstring!("invalid mode")) != 0)
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
pub unsafe fn io_pclose(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let p: *mut Stream =
            lual_checkudata(interpreter, 1, make_cstring!("FILE*")) as *mut Stream;
        *__errno_location() = 0;
        return lual_execresult(interpreter, pclose((*p).file));
    }
}
pub unsafe fn io_popen(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let filename: *const i8 = lual_checklstring(interpreter, 1, null_mut());
        let mode: *const i8 =
            lual_optlstring(interpreter, 2, make_cstring!("r"), null_mut());
        let p: *mut Stream = newprefile(interpreter);
        ((((*mode.offset(0 as isize) as i32 == CHARACTER_LOWER_R as i32
            || *mode.offset(0 as isize) as i32 == CHARACTER_LOWER_W as i32)
            && *mode.offset(1 as isize) as i32 == Character::Null as i32) as i32
            != 0) as i64
            != 0
            || lual_argerror(interpreter, 2, make_cstring!("invalid mode")) != 0)
            as i32;
        *__errno_location() = 0;
        fflush(null_mut());
        (*p).file = popen(filename, mode);
        (*p).close_function = Some(io_pclose as unsafe fn(*mut Interpreter) -> i32);
        return if ((*p).file).is_null() {
            lual_fileresult(interpreter, 0, filename)
        } else {
            1
        };
    }
}
pub unsafe fn io_tmpfile(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let p: *mut Stream = newfile(interpreter);
        *__errno_location() = 0;
        (*p).file = tmpfile();
        return if ((*p).file).is_null() {
            lual_fileresult(interpreter, 0, null())
        } else {
            1
        };
    }
}
pub unsafe fn getiofile(interpreter: *mut Interpreter, findex: *const i8) -> *mut FILE {
    unsafe {
        let p: *mut Stream;
        lua_getfield(interpreter, -(1000000 as i32) - 1000 as i32, findex);
        p = (*interpreter).to_pointer(-1) as *mut Stream;
        if (*p).close_function.is_none() {
            lual_error(
                interpreter,
                make_cstring!("default %s file is closed"),
                findex.offset(
                    (size_of::<[i8; 5]>() as usize)
                        .wrapping_div(size_of::<i8>() as usize)
                        .wrapping_sub(1 as usize) as isize,
                ),
            );
        }
        return (*p).file;
    }
}
pub unsafe fn g_iofile(
    interpreter: *mut Interpreter,
    f: *const i8,
    mode: *const i8,
) -> i32 {
    unsafe {
        if !(is_none_or_nil(lua_type(interpreter, 1))) {
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
        return g_iofile(
            interpreter,
            make_cstring!("_IO_input"),
            make_cstring!("r"),
        );
    }
}
pub unsafe fn io_output(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        return g_iofile(
            interpreter,
            make_cstring!("_IO_output"),
            make_cstring!("w"),
        );
    }
}
pub unsafe fn aux_lines(interpreter: *mut Interpreter, to_close: bool) {
    unsafe {
        let n: i32 = (*interpreter).get_top() - 1;
        (((n <= 250 as i32) as i32 != 0) as i64 != 0
            || lual_argerror(
                interpreter,
                250 as i32 + 2,
                make_cstring!("too many arguments"),
            ) != 0) as i32;
        lua_pushvalue(interpreter, 1);
        (*interpreter).push_integer(n as i64);
        (*interpreter).push_boolean(to_close);
        lua_rotate(interpreter, 2, 3);
        lua_pushcclosure(
            interpreter,
            Some(io_readline as unsafe fn(*mut Interpreter) -> i32),
            3 + n,
        );
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
            lua_getfield(
                interpreter,
                -(1000000 as i32) - 1000 as i32,
                make_cstring!("_IO_input"),
            );
            lua_copy(interpreter, -1, 1);
            lua_settop(interpreter, -2);
            tofile(interpreter);
            to_close = false;
        } else {
            let filename: *const i8 = lual_checklstring(interpreter, 1, null_mut());
            opencheck(interpreter, filename, make_cstring!("r"));
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
        if (*rn).n >= 200 {
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
pub unsafe fn test2(rn: *mut RN, set: *const i8) -> i32 {
    unsafe {
        if (*rn).c == *set.offset(0 as isize) as i32 || (*rn).c == *set.offset(1 as isize) as i32 {
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
pub unsafe fn read_number(interpreter: *mut Interpreter, file: *mut FILE) -> i32 {
    unsafe {
        let mut rn: RN = RN {
            file: null_mut(),
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
            if !(*(*__ctype_b_loc()).offset(rn.c as isize) as i32 & _ISSPACE as i32 != 0) {
                break;
            }
        }
        test2(&mut rn, make_cstring!("-+"));
        if test2(&mut rn, make_cstring!("00")) != 0 {
            if test2(&mut rn, make_cstring!("xX")) != 0 {
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
                    make_cstring!("pP")
                } else {
                    make_cstring!("eE")
                },
            ) != 0
        {
            test2(&mut rn, make_cstring!("-+"));
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
pub unsafe fn test_eof(interpreter: *mut Interpreter, file: *mut FILE) -> i32 {
    unsafe {
        let c: i32 = getc(file);
        ungetc(c, file);
        lua_pushstring(interpreter, make_cstring!(""));
        return (c != -1) as i32;
    }
}
pub unsafe fn read_line(
    interpreter: *mut Interpreter,
    file: *mut FILE,
    chop: i32,
) -> i32 {
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
            flockfile(file);
            while i
                < (16 as usize)
                    .wrapping_mul(size_of::<*mut libc::c_void>() as usize)
                    .wrapping_mul(size_of::<f64>() as usize) as i32
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
            b.loads.set_length(
                ((b.loads.get_length() as usize).wrapping_add(i as usize) as i32) as usize,
            );
            if !(c != -1 && c != CHARACTER_LF as i32) {
                break;
            }
        }
        if chop == 0 && c == CHARACTER_LF as i32 {
            (b.loads.get_length() < b.loads.get_size() || !(b.prepare_with_size(1)).is_null())
                as i32;
            let fresh154 = b.loads.get_length();
            b.loads
                .set_length(((b.loads.get_length()).wrapping_add(1)) as usize);
            *(b.loads.loads_pointer).offset(fresh154 as isize) = c as i8;
        }
        b.push_result();
        return (c == CHARACTER_LF as i32 || get_length_raw(interpreter, -1) > 0) as usize as u32
            as i32;
    }
}
pub unsafe fn read_all(interpreter: *mut Interpreter, file: *mut FILE) {
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
            nr = fread(
                p as *mut libc::c_void,
                size_of::<i8>(),
                (16 as usize)
                    .wrapping_mul(size_of::<*mut libc::c_void>())
                    .wrapping_mul(size_of::<f64>()),
                file,
            ) as usize;
            b.loads.set_length(
                ((b.loads.get_length() as usize).wrapping_add(nr as usize) as i32) as usize,
            );
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
pub unsafe fn read_chars(
    interpreter: *mut Interpreter,
    file: *mut FILE,
    n: usize,
) -> i32 {
    unsafe {
        let nr: usize;
        let p: *mut i8;
        let mut b = Buffer::new();
        b.initialize(interpreter);
        p = b.prepare_with_size(n as usize);
        nr = fread(p as *mut libc::c_void, size_of::<i8>(), n as usize, file) as usize;
        b.loads.set_length(
            ((b.loads.get_length() as usize).wrapping_add(nr as usize) as i32) as usize,
        );
        b.push_result();
        return (nr > 0) as i32;
    }
}
pub unsafe fn g_read(interpreter: *mut Interpreter, file: *mut FILE, first: i32) -> i32 {
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
                make_cstring!("too many arguments"),
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
                    let l: usize = lual_checkinteger(interpreter, n) as usize;
                    success = if l == 0 {
                        test_eof(interpreter, file)
                    } else {
                        read_chars(interpreter, file, l)
                    };
                } else {
                    let mut p: *const i8 = lual_checklstring(interpreter, n, null_mut());
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
                                make_cstring!("invalid format"),
                            );
                        }
                    }
                }
                n += 1;
            }
        }
        if ferror(file) != 0 {
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
        return g_read(
            interpreter,
            getiofile(interpreter, make_cstring!("_IO_input")),
            1,
        );
    }
}
pub unsafe fn f_read(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        return g_read(interpreter, tofile(interpreter), 2);
    }
}
pub unsafe fn io_readline(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let p: *mut Stream =
            (*interpreter).to_pointer(-(1000000 as i32) - 1000 as i32 - 1) as *mut Stream;
        let mut n: i32 =
            lua_tointegerx(interpreter, -(1000000 as i32) - 1000 as i32 - 2, null_mut()) as i32;
        if ((*p).close_function).is_none() {
            return lual_error(interpreter, make_cstring!("file is already closed"));
        }
        lua_settop(interpreter, 1);
        lual_checkstack(
            interpreter,
            n,
            make_cstring!("too many arguments"),
        );
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
                    make_cstring!("%s"),
                    lua_tolstring(interpreter, -n + 1, null_mut()),
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
pub unsafe fn g_write(
    interpreter: *mut Interpreter,
    file: *mut FILE,
    mut arg: i32,
) -> i32 {
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
                        make_cstring!("%lld"),
                        lua_tointegerx(interpreter, arg, null_mut()),
                    )
                } else {
                    fprintf(
                        file,
                        make_cstring!("%.14g"),
                        lua_tonumberx(interpreter, arg, null_mut()),
                    )
                };
                status = (status != 0 && length > 0) as i32;
            } else {
                let mut l: usize = 0;
                let s: *const i8 = lual_checklstring(interpreter, arg, &mut l);
                status = (status != 0
                    && fwrite(s as *const libc::c_void, size_of::<i8>(), l as usize, file)
                        == l as usize) as i32;
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
        return g_write(
            interpreter,
            getiofile(interpreter, make_cstring!("_IO_output")),
            1,
        );
    }
}
pub unsafe fn f_write(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let file: *mut FILE = tofile(interpreter);
        lua_pushvalue(interpreter, 1);
        return g_write(interpreter, file, 2);
    }
}
pub unsafe fn f_seek(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        pub const MODE: [i32; 3] = [0, 1, 2];
        pub const MODE_NAMES: [*const i8; 4] = [
            make_cstring!("set"),
            make_cstring!("cur"),
            make_cstring!("end"),
            null(),
        ];
        let file: *mut FILE = tofile(interpreter);
        let mut op: i32 = lual_checkoption(
            interpreter,
            2,
            make_cstring!("cur"),
            MODE_NAMES.as_ptr(),
        );
        let p3: i64 = lual_optinteger(interpreter, 3, 0);
        let offset: i64 = p3 as i64;
        (((offset as i64 == p3) as i32 != 0) as i64 != 0
            || lual_argerror(
                interpreter,
                3,
                make_cstring!("not an integer in proper range"),
            ) != 0) as i32;
        *__errno_location() = 0;
        op = fseeko(file, offset, MODE[op as usize]);
        if (op != 0) as i64 != 0 {
            return lual_fileresult(interpreter, 0, null());
        } else {
            (*interpreter).push_integer(ftello(file) as i64);
            return 1;
        };
    }
}
pub unsafe fn f_setvbuf(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        pub const MODE: [i32; 3] = [2, 0, 1];
        pub const MODE_NAMES: [*const i8; 4] = [
            make_cstring!("no"),
            make_cstring!("full"),
            make_cstring!("line"),
            null(),
        ];
        let file: *mut FILE = tofile(interpreter);
        let op: i32 = lual_checkoption(interpreter, 2, null(), MODE_NAMES.as_ptr());
        let size: i64 = lual_optinteger(
            interpreter,
            3,
            (16 as usize)
                .wrapping_mul(size_of::<*mut libc::c_void>() as usize)
                .wrapping_mul(size_of::<f64>() as usize) as i64,
        );
        let res: i32;
        *__errno_location() = 0;
        res = setvbuf(file, null_mut(), MODE[op as usize], size as usize);
        return lual_fileresult(interpreter, (res == 0) as i32, null());
    }
}
pub unsafe fn io_flush(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let file: *mut FILE = getiofile(interpreter, make_cstring!("_IO_output"));
        *__errno_location() = 0;
        return lual_fileresult(interpreter, (fflush(file) == 0) as i32, null());
    }
}
pub unsafe fn f_flush(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let file: *mut FILE = tofile(interpreter);
        *__errno_location() = 0;
        return lual_fileresult(interpreter, (fflush(file) == 0) as i32, null());
    }
}
pub const IO_FUNCTIONS: [RegisteredFunction; 12] = {
    [
        {
            RegisteredFunction {
                name: make_cstring!("close"),
                function: Some(io_close as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: make_cstring!("flush"),
                function: Some(io_flush as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: make_cstring!("input"),
                function: Some(io_input as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: make_cstring!("lines"),
                function: Some(io_lines as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: make_cstring!("open"),
                function: Some(io_open as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: make_cstring!("output"),
                function: Some(io_output as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: make_cstring!("popen"),
                function: Some(io_popen as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: make_cstring!("read"),
                function: Some(io_read as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: make_cstring!("tmpfile"),
                function: Some(io_tmpfile as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: make_cstring!("type"),
                function: Some(io_type as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: make_cstring!("write"),
                function: Some(io_write as unsafe fn(*mut Interpreter) -> i32),
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
pub const IO_METHODS: [RegisteredFunction; 8] = {
    [
        {
            RegisteredFunction {
                name: make_cstring!("read"),
                function: Some(f_read as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: make_cstring!("write"),
                function: Some(f_write as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: make_cstring!("lines"),
                function: Some(f_lines as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: make_cstring!("flush"),
                function: Some(f_flush as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: make_cstring!("seek"),
                function: Some(f_seek as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: make_cstring!("close"),
                function: Some(f_close as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: make_cstring!("setvbuf"),
                function: Some(f_setvbuf as unsafe fn(*mut Interpreter) -> i32),
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
pub const IO_METAMETHODS: [RegisteredFunction; 5] = {
    [
        {
            RegisteredFunction {
                name: make_cstring!("__index"),
                function: None,
            }
        },
        {
            RegisteredFunction {
                name: make_cstring!("__gc"),
                function: Some(f_gc as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: make_cstring!("__close"),
                function: Some(f_gc as unsafe fn(*mut Interpreter) -> i32),
            }
        },
        {
            RegisteredFunction {
                name: make_cstring!("__tostring"),
                function: Some(f_tostring as unsafe fn(*mut Interpreter) -> i32),
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
pub unsafe fn createmeta(interpreter: *mut Interpreter) {
    unsafe {
        lual_newmetatable(interpreter, make_cstring!("FILE*"));
        lual_setfuncs(interpreter, IO_METAMETHODS.as_ptr(), 0);
        (*interpreter).lua_createtable();
        lual_setfuncs(interpreter, IO_METHODS.as_ptr(), 0);
        lua_setfield(interpreter, -2, make_cstring!("__index"));
        lua_settop(interpreter, -2);
    }
}
pub unsafe fn io_noclose(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let p: *mut Stream =
            lual_checkudata(interpreter, 1, make_cstring!("FILE*")) as *mut Stream;
        (*p).close_function = Some(io_noclose as unsafe fn(*mut Interpreter) -> i32);
        (*interpreter).push_nil();
        lua_pushstring(
            interpreter,
            make_cstring!("cannot close standard file"),
        );
        return 2;
    }
}
pub unsafe fn createstdfile(
    interpreter: *mut Interpreter,
    file: *mut FILE,
    k: *const i8,
    fname: *const i8,
) {
    unsafe {
        let p: *mut Stream = newprefile(interpreter);
        (*p).file = file;
        (*p).close_function = Some(io_noclose as unsafe fn(*mut Interpreter) -> i32);
        if !k.is_null() {
            lua_pushvalue(interpreter, -1);
            lua_setfield(interpreter, -(1000000 as i32) - 1000 as i32, k);
        }
        lua_setfield(interpreter, -2, fname);
    }
}
pub unsafe fn luaopen_io(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        lual_checkversion_(
            interpreter,
            504.0,
            (size_of::<i64>() as usize)
                .wrapping_mul(16 as usize)
                .wrapping_add(size_of::<f64>() as usize),
        );
        (*interpreter).lua_createtable();
        lual_setfuncs(interpreter, IO_FUNCTIONS.as_ptr(), 0);
        createmeta(interpreter);
        createstdfile(
            interpreter,
            stdin,
            make_cstring!("_IO_input"),
            make_cstring!("stdin"),
        );
        createstdfile(
            interpreter,
            stdout,
            make_cstring!("_IO_output"),
            make_cstring!("stdout"),
        );
        createstdfile(
            interpreter,
            stderr,
            null(),
            make_cstring!("stderr"),
        );
        return 1;
    }
}
