use crate::buffer::*;
use crate::interpreter::*;
use crate::registeredfunction::*;
use crate::stream::*;
use crate::tagtype::*;
use crate::tdefaultnew::*;
use crate::user::*;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::ptr::*;

// ─── Mode string validation ───────────────────────────────────────────────────

fn check_mode(mode: &[u8]) -> bool {
    if mode.is_empty() {
        return false;
    }
    let mut i = 0;
    if !matches!(mode[i], b'r' | b'w' | b'a') {
        return false;
    }
    i += 1;
    if i < mode.len() && mode[i] == b'+' {
        i += 1;
    }
    // rest must be 'b' only
    mode[i..].iter().all(|&b| b == b'b')
}

/// Parse a mode string into an OpenOptions. Returns None for invalid modes.
fn open_opts_from_mode(mode: &[u8]) -> Option<OpenOptions> {
    if mode.is_empty() {
        return None;
    }
    let base = mode[0];
    let plus = mode.contains(&b'+');
    let mut opts = OpenOptions::new();
    match base {
        b'r' => {
            opts.read(true);
            if plus {
                opts.write(true);
            }
        }
        b'w' => {
            opts.write(true).create(true).truncate(true);
            if plus {
                opts.read(true);
            }
        }
        b'a' => {
            opts.append(true).create(true);
            if plus {
                opts.read(true);
            }
        }
        _ => return None,
    }
    Some(opts)
}

// ─── Handle-level helpers ─────────────────────────────────────────────────────

/// Write bytes to an IoHandle, returning whether successful.
unsafe fn handle_write_bytes(handle: *mut IoHandle, data: &[u8]) -> bool {
    unsafe {
        match &mut *handle {
            IoHandle::File(fh) => fh.write_buffered(data),
            IoHandle::Pipe(f) => {
                libc::fwrite(data.as_ptr() as *const _, 1, data.len(), *f) == data.len()
            }
            IoHandle::Stdout => std::io::stdout().write_all(data).is_ok(),
            IoHandle::Stderr => std::io::stderr().write_all(data).is_ok(),
            IoHandle::Stdin { .. } => false,
        }
    }
}

/// Write a formatted integer to an IoHandle.
unsafe fn handle_write_integer(handle: *mut IoHandle, i: i64) -> bool {
    unsafe {
        let mut buf = [0i8; 32];
        let n = libc::snprintf(buf.as_mut_ptr(), buf.len(), c"%lld".as_ptr(), i);
        handle_write_bytes(handle, std::slice::from_raw_parts(buf.as_ptr() as *const u8, n as usize))
    }
}

/// Write a formatted float to an IoHandle using %.14g.
unsafe fn handle_write_float(handle: *mut IoHandle, f: f64) -> bool {
    unsafe {
        let mut buf = [0i8; 64];
        let n = libc::snprintf(buf.as_mut_ptr(), buf.len(), c"%.14g".as_ptr(), f);
        handle_write_bytes(handle, std::slice::from_raw_parts(buf.as_ptr() as *const u8, n as usize))
    }
}

/// Read all remaining bytes from an IoHandle into a Vec.
unsafe fn handle_read_all(handle: *mut IoHandle) -> Vec<u8> {
    unsafe {
        match &mut *handle {
            IoHandle::File(fh) => {
                let mut buf = Vec::new();
                if let Some(b) = fh.pushback.take() {
                    buf.push(b);
                }
                let _ = fh.file.read_to_end(&mut buf);
                buf
            }
            IoHandle::Pipe(f) => {
                let mut buf = Vec::new();
                let mut chunk = [0u8; 1024];
                loop {
                    let n = libc::fread(chunk.as_mut_ptr() as *mut _, 1, chunk.len(), *f);
                    if n == 0 {
                        break;
                    }
                    buf.extend_from_slice(&chunk[..n]);
                }
                buf
            }
            IoHandle::Stdin { pushback } => {
                let mut buf = Vec::new();
                if let Some(b) = pushback.take() {
                    buf.push(b);
                }
                let _ = std::io::stdin().lock().read_to_end(&mut buf);
                buf
            }
            _ => Vec::new(),
        }
    }
}

/// Read exactly n bytes from an IoHandle. Returns how many were actually read.
unsafe fn handle_read_n(handle: *mut IoHandle, n: usize) -> Vec<u8> {
    unsafe {
        match &mut *handle {
            IoHandle::File(fh) => {
                let mut buf = Vec::with_capacity(n);
                if let Some(b) = fh.pushback.take() {
                    buf.push(b);
                }
                let remain = n - buf.len();
                if remain > 0 {
                    let start = buf.len();
                    buf.resize(n, 0);
                    let nr = fh.file.read(&mut buf[start..]).unwrap_or(0);
                    buf.truncate(start + nr);
                }
                buf
            }
            IoHandle::Pipe(f) => {
                let mut buf = vec![0u8; n];
                let nr = libc::fread(buf.as_mut_ptr() as *mut _, 1, n, *f);
                buf.truncate(nr);
                buf
            }
            IoHandle::Stdin { pushback } => {
                let mut buf = Vec::with_capacity(n);
                if let Some(b) = pushback.take() {
                    buf.push(b);
                }
                let remain = n - buf.len();
                if remain > 0 {
                    let mut tmp = vec![0u8; remain];
                    let nr = std::io::stdin().lock().read(&mut tmp).unwrap_or(0);
                    buf.extend_from_slice(&tmp[..nr]);
                }
                buf
            }
            _ => Vec::new(),
        }
    }
}

/// Flush an IoHandle's output buffer.
unsafe fn handle_flush(handle: *mut IoHandle) -> bool {
    unsafe { (*handle).flush() }
}

// ─── Stream userdata helpers ──────────────────────────────────────────────────

pub unsafe fn io_type(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        lual_checkany(interpreter, 1);
        let p = lual_testudata(interpreter, 1, c"FILE*".as_ptr()) as *mut Stream;
        if p.is_null() {
            (*interpreter).push_nil();
        } else if (*p).stream_cfunctionclose.is_none() {
            lua_pushstring(interpreter, c"closed file".as_ptr());
        } else {
            lua_pushstring(interpreter, c"file".as_ptr());
        }
        1
    }
}

pub unsafe fn f_tostring(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let p = lual_checkudata(interpreter, 1, c"FILE*".as_ptr()) as *mut Stream;
        if (*p).stream_cfunctionclose.is_none() {
            lua_pushstring(interpreter, c"file (closed)".as_ptr());
        } else {
            lua_pushfstring(interpreter, c"file (%p)".as_ptr(), (*p).stream_handle);
        }
        1
    }
}

/// Get the open handle for the file at stack index 1, or error if closed.
pub unsafe fn tofile(interpreter: *mut Interpreter) -> *mut IoHandle {
    unsafe {
        let p = lual_checkudata(interpreter, 1, c"FILE*".as_ptr()) as *mut Stream;
        if (*p).stream_cfunctionclose.is_none() {
            lual_error(interpreter, c"attempt to use a closed file".as_ptr());
        }
        (*p).stream_handle
    }
}

/// Allocate a new pre-file Stream userdata (handle and close function both null).
pub unsafe fn newprefile(interpreter: *mut Interpreter) -> *mut Stream {
    unsafe {
        let p = User::lua_newuserdatauv(interpreter, size_of::<Stream>(), 0) as *mut Stream;
        (*p).stream_handle = null_mut();
        (*p).stream_cfunctionclose = None;
        lual_setmetatable(interpreter, c"FILE*".as_ptr());
        p
    }
}

/// Allocate a new file Stream userdata, wired up for io_fclose.
pub unsafe fn newfile(interpreter: *mut Interpreter) -> *mut Stream {
    unsafe {
        let p = newprefile(interpreter);
        (*p).stream_handle = null_mut();
        (*p).stream_cfunctionclose =
            Some(io_fclose as unsafe fn(*mut Interpreter) -> i32);
        p
    }
}

pub unsafe fn aux_close(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let p = lual_checkudata(interpreter, 1, c"FILE*".as_ptr()) as *mut Stream;
        let close_fn = (*p).stream_cfunctionclose;
        (*p).stream_cfunctionclose = None;
        (close_fn.expect("non-null function pointer"))(interpreter)
    }
}

pub unsafe fn f_close(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        tofile(interpreter); // validate open
        aux_close(interpreter)
    }
}

pub unsafe fn io_close(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        if lua_type(interpreter, 1) == None {
            lua_getfield(interpreter, -(1000000i32) - 1000, c"_IO_output".as_ptr());
        }
        f_close(interpreter)
    }
}

pub unsafe fn f_gc(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let p = lual_checkudata(interpreter, 1, c"FILE*".as_ptr()) as *mut Stream;
        // Call the close function if still open
        if (*p).stream_cfunctionclose.is_some() && !(*p).stream_handle.is_null() {
            aux_close(interpreter);
        }
        // Free the handle allocation (io_fclose/io_pclose null it; io_noclose does not)
        if !(*p).stream_handle.is_null() {
            drop(Box::from_raw((*p).stream_handle));
            (*p).stream_handle = null_mut();
        }
        0
    }
}

// ─── Close functions ──────────────────────────────────────────────────────────

pub unsafe fn io_fclose(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let p = lual_checkudata(interpreter, 1, c"FILE*".as_ptr()) as *mut Stream;
        let handle = (*p).stream_handle;
        (*p).stream_handle = null_mut(); // null before drop
        *errno_location() = 0;
        // Flush write buffer before dropping (in case setvbuf buffering is active)
        if let IoHandle::File(fh) = &mut *handle {
            fh.flush_write_buf();
        }
        drop(Box::from_raw(handle)); // dropping IoHandle::File closes the File
        lual_fileresult(interpreter, 1, null())
    }
}

pub unsafe fn io_pclose(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let p = lual_checkudata(interpreter, 1, c"FILE*".as_ptr()) as *mut Stream;
        let handle = (*p).stream_handle;
        (*p).stream_handle = null_mut();
        *errno_location() = 0;
        let stat = if let IoHandle::Pipe(f) = &*handle {
            libc::pclose(*f)
        } else {
            0
        };
        drop(Box::from_raw(handle));
        lual_execresult(interpreter, stat)
    }
}

pub unsafe fn io_noclose(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let p = lual_checkudata(interpreter, 1, c"FILE*".as_ptr()) as *mut Stream;
        // Reset cfunctionclose so the file appears open (cannot be user-closed)
        (*p).stream_cfunctionclose =
            Some(io_noclose as unsafe fn(*mut Interpreter) -> i32);
        (*interpreter).push_nil();
        lua_pushstring(interpreter, c"cannot close standard file".as_ptr());
        2
    }
}

// ─── File opening ─────────────────────────────────────────────────────────────

pub unsafe fn opencheck(interpreter: *mut Interpreter, fname: *const i8, mode: *const i8) {
    unsafe {
        let p = newfile(interpreter);
        let fname_str = std::ffi::CStr::from_ptr(fname).to_string_lossy();
        let mode_bytes = std::ffi::CStr::from_ptr(mode).to_bytes();
        match open_opts_from_mode(mode_bytes) {
            Some(opts) => match opts.open(fname_str.as_ref()) {
                Ok(file) => {
                    let handle = Box::into_raw(Box::new(IoHandle::File(FileHandle::new(file))));
                    (*p).stream_handle = handle;
                }
                Err(e) => {
                    if let Some(code) = e.raw_os_error() {
                        *errno_location() = code;
                    }
                    lual_error(
                        interpreter,
                        c"cannot open file '%s' (%s)".as_ptr(),
                        fname,
                        libc::strerror(*errno_location()),
                    );
                }
            },
            None => {
                lual_error(interpreter, c"invalid mode".as_ptr());
            }
        }
    }
}

pub unsafe fn io_open(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let mut fnlen = 0usize;
        let fname = lual_checklstring(interpreter, 1, &mut fnlen);
        let mode = lual_optlstring(interpreter, 2, c"r".as_ptr(), null_mut());
        let mode_bytes = std::ffi::CStr::from_ptr(mode).to_bytes();
        if !check_mode(mode_bytes) {
            return lual_argerror(interpreter, 2, c"invalid mode".as_ptr());
        }
        let p = newfile(interpreter);
        let fname_str = std::ffi::CStr::from_ptr(fname).to_string_lossy();
        *errno_location() = 0;
        match open_opts_from_mode(mode_bytes) {
            Some(opts) => match opts.open(fname_str.as_ref()) {
                Ok(file) => {
                    let handle =
                        Box::into_raw(Box::new(IoHandle::File(FileHandle::new(file))));
                    (*p).stream_handle = handle;
                    1
                }
                Err(e) => {
                    if let Some(code) = e.raw_os_error() {
                        *errno_location() = code;
                    }
                    lual_fileresult(interpreter, 0, fname)
                }
            },
            None => lual_argerror(interpreter, 2, c"invalid mode".as_ptr()),
        }
    }
}

pub unsafe fn io_popen(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let filename = lual_checklstring(interpreter, 1, null_mut());
        let mode = lual_optlstring(interpreter, 2, c"r".as_ptr(), null_mut());
        let mode_bytes = std::ffi::CStr::from_ptr(mode).to_bytes();
        let valid = matches!(mode_bytes, b"r" | b"w");
        if !valid {
            return lual_argerror(interpreter, 2, c"invalid mode".as_ptr());
        }
        let p = newprefile(interpreter);
        *errno_location() = 0;
        std::io::stdout().flush().unwrap_or(());
        std::io::stderr().flush().unwrap_or(());
        let file = libc::popen(filename, mode);
        if file.is_null() {
            return lual_fileresult(interpreter, 0, filename);
        }
        let handle = Box::into_raw(Box::new(IoHandle::Pipe(file)));
        (*p).stream_handle = handle;
        (*p).stream_cfunctionclose =
            Some(io_pclose as unsafe fn(*mut Interpreter) -> i32);
        1
    }
}

pub unsafe fn io_tmpfile(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        // Use libc tmpfile() for a proper anonymous temp file, then wrap
        // the fd in a Rust File via dup so Rust manages its lifetime.
        *errno_location() = 0;
        let tmp = libc::tmpfile();
        if tmp.is_null() {
            return lual_fileresult(interpreter, 0, null());
        }
        use std::os::unix::io::FromRawFd;
        let fd = libc::dup(libc::fileno(tmp));
        libc::fclose(tmp); // closes original fd; dup'd fd stays open
        if fd < 0 {
            return lual_fileresult(interpreter, 0, null());
        }
        let file = File::from_raw_fd(fd);
        let handle = Box::into_raw(Box::new(IoHandle::File(FileHandle::new(file))));
        let p = newfile(interpreter);
        (*p).stream_handle = handle;
        1
    }
}

// ─── Default I/O file management ─────────────────────────────────────────────

pub unsafe fn getiofile(
    interpreter: *mut Interpreter,
    findex: *const i8,
) -> *mut IoHandle {
    unsafe {
        lua_getfield(interpreter, -(1000000i32) - 1000, findex);
        let p = (*interpreter).to_pointer(-1) as *mut Stream;
        if (*p).stream_cfunctionclose.is_none() {
            lual_error(
                interpreter,
                c"default %s file is closed".as_ptr(),
                findex.offset(4),
            );
        }
        (*p).stream_handle
    }
}

pub unsafe fn g_iofile(
    interpreter: *mut Interpreter,
    f: *const i8,
    mode: *const i8,
) -> i32 {
    unsafe {
        if !TagType::is_none_or_nil(lua_type(interpreter, 1)) {
            let filename = lua_tolstring(interpreter, 1, null_mut());
            if !filename.is_null() {
                opencheck(interpreter, filename, mode);
            } else {
                tofile(interpreter);
                lua_pushvalue(interpreter, 1);
            }
            lua_setfield(interpreter, -(1000000i32) - 1000, f);
        }
        lua_getfield(interpreter, -(1000000i32) - 1000, f);
        1
    }
}

pub unsafe fn io_input(interpreter: *mut Interpreter) -> i32 {
    unsafe { g_iofile(interpreter, c"_IO_input".as_ptr(), c"r".as_ptr()) }
}

pub unsafe fn io_output(interpreter: *mut Interpreter) -> i32 {
    unsafe { g_iofile(interpreter, c"_IO_output".as_ptr(), c"w".as_ptr()) }
}

// ─── Read functions ───────────────────────────────────────────────────────────

/// Read a Lua number from the handle. Returns 1 on success (value on stack), 0 on failure.
unsafe fn read_number(interpreter: *mut Interpreter, handle: *mut IoHandle) -> i32 {
    unsafe {
        let mut buf = [0i8; 201];
        let mut n = 0usize;

        let push = |buf: &mut [i8; 201], n: &mut usize, c: u8| -> bool {
            if *n >= 200 {
                buf[0] = 0;
                return false;
            }
            buf[*n] = c as i8;
            *n += 1;
            true
        };

        // Skip leading whitespace
        let mut c = loop {
            match (*handle).read_byte() {
                None => {
                    (*interpreter).push_nil();
                    return 0;
                }
                Some(b) if (b as char).is_ascii_whitespace() => continue,
                Some(b) => break b,
            }
        };

        // Optional sign
        if c == b'+' || c == b'-' {
            if !push(&mut buf, &mut n, c) {
                (*interpreter).push_nil();
                return 0;
            }
            c = match (*handle).read_byte() {
                Some(b) => b,
                None => {
                    buf[n] = 0;
                    if lua_stringtonumber(interpreter, buf.as_mut_ptr()) != 0 {
                        return 1;
                    }
                    (*interpreter).push_nil();
                    return 0;
                }
            };
        }

        let mut count = 0i32;
        let mut hex = false;

        // Leading zero + possible 0x/0X
        if c == b'0' {
            if !push(&mut buf, &mut n, c) {
                (*interpreter).push_nil();
                return 0;
            }
            count = 1;
            c = match (*handle).read_byte() {
                Some(b) => b,
                None => {
                    buf[n] = 0;
                    return if lua_stringtonumber(interpreter, buf.as_mut_ptr()) != 0 {
                        1
                    } else {
                        (*interpreter).push_nil();
                        0
                    };
                }
            };
            if c == b'x' || c == b'X' {
                hex = true;
                if !push(&mut buf, &mut n, c) {
                    (*interpreter).push_nil();
                    return 0;
                }
                c = (*handle).read_byte().unwrap_or(0);
            }
        }

        // Integer digits
        loop {
            let is_digit = if hex { c.is_ascii_hexdigit() } else { c.is_ascii_digit() };
            if !is_digit { break; }
            if !push(&mut buf, &mut n, c) { break; }
            count += 1;
            c = (*handle).read_byte().unwrap_or(0);
        }

        // Decimal point
        let lc = libc::localeconv();
        let dec_point = if !lc.is_null() && !(*lc).decimal_point.is_null() {
            *(*lc).decimal_point as u8
        } else {
            b'.'
        };
        if c == b'.' || c == dec_point {
            if !push(&mut buf, &mut n, b'.') {
                (*interpreter).push_nil();
                return 0;
            }
            c = (*handle).read_byte().unwrap_or(0);
            loop {
                let is_digit = if hex { c.is_ascii_hexdigit() } else { c.is_ascii_digit() };
                if !is_digit { break; }
                if !push(&mut buf, &mut n, c) { break; }
                count += 1;
                c = (*handle).read_byte().unwrap_or(0);
            }
        }

        // Exponent
        if count > 0 {
            let exp_char = if hex { b'p' } else { b'e' };
            if c == exp_char || c == exp_char.to_ascii_uppercase() {
                if !push(&mut buf, &mut n, c) {
                    (*interpreter).push_nil();
                    return 0;
                }
                c = (*handle).read_byte().unwrap_or(0);
                if c == b'+' || c == b'-' {
                    if !push(&mut buf, &mut n, c) {
                        (*interpreter).push_nil();
                        return 0;
                    }
                    c = (*handle).read_byte().unwrap_or(0);
                }
                while c.is_ascii_digit() {
                    if !push(&mut buf, &mut n, c) { break; }
                    c = (*handle).read_byte().unwrap_or(0);
                }
            }
        }

        // Push back the character that didn't fit
        if c != 0 {
            (*handle).unread_byte(c);
        }
        buf[n] = 0;

        if lua_stringtonumber(interpreter, buf.as_mut_ptr()) != 0 {
            1
        } else {
            (*interpreter).push_nil();
            0
        }
    }
}

/// Test for EOF: push "" and return 1 if not at EOF, 0 if at EOF.
unsafe fn test_eof(interpreter: *mut Interpreter, handle: *mut IoHandle) -> i32 {
    unsafe {
        match (*handle).read_byte() {
            None => {
                lua_pushstring(interpreter, c"".as_ptr());
                0
            }
            Some(b) => {
                (*handle).unread_byte(b);
                lua_pushstring(interpreter, c"".as_ptr());
                1
            }
        }
    }
}

/// Read a line from the handle. If chop==1, strip the trailing newline.
unsafe fn read_line(
    interpreter: *mut Interpreter,
    handle: *mut IoHandle,
    chop: i32,
) -> i32 {
    unsafe {
        let mut b = Buffer::new();
        b.initialize(interpreter);
        let mut last_c: Option<u8> = None;

        'outer: loop {
            let buf_ptr = b.prepare_with_size(1024);
            let buf = std::slice::from_raw_parts_mut(buf_ptr as *mut u8, 1024);
            let mut i = 0usize;
            loop {
                match (*handle).read_byte() {
                    None => {
                        b.buffer_loads.set_length(b.buffer_loads.get_length() as usize + i);
                        last_c = None;
                        break 'outer;
                    }
                    Some(b'\n') => {
                        b.buffer_loads.set_length(b.buffer_loads.get_length() as usize + i);
                        last_c = Some(b'\n');
                        break 'outer;
                    }
                    Some(byte) => {
                        buf[i] = byte;
                        i += 1;
                        if i == 1024 {
                            b.buffer_loads.set_length(b.buffer_loads.get_length() as usize + i);
                            break;
                        }
                    }
                }
            }
        }

        // If not chopping newline, add it back
        if chop == 0 {
            if let Some(b'\n') = last_c {
                let ptr = b.prepare_with_size(1);
                *ptr = b'\n' as i8;
                b.buffer_loads.set_length(b.buffer_loads.get_length() as usize + 1);
            }
        }

        b.push_result();
        (last_c == Some(b'\n') || get_length_raw(interpreter, -1) > 0) as i32
    }
}

/// Read all remaining content from the handle onto the Lua stack.
unsafe fn read_all(interpreter: *mut Interpreter, handle: *mut IoHandle) {
    unsafe {
        let data = handle_read_all(handle);
        lua_pushlstring(interpreter, data.as_ptr() as *const i8, data.len());
    }
}

/// Read n bytes from the handle. Returns 1 if any bytes read, 0 if EOF.
unsafe fn read_chars(
    interpreter: *mut Interpreter,
    handle: *mut IoHandle,
    n: usize,
) -> i32 {
    unsafe {
        let data = handle_read_n(handle, n);
        let len = data.len();
        lua_pushlstring(interpreter, data.as_ptr() as *const i8, len);
        (len > 0) as i32
    }
}

pub unsafe fn g_read(
    interpreter: *mut Interpreter,
    handle: *mut IoHandle,
    first: i32,
) -> i32 {
    unsafe {
        let mut nargs = (*interpreter).get_top() - 1;
        let mut n: i32;
        let success: i32;

        (*handle).clear_error();
        *errno_location() = 0;

        if nargs == 0 {
            success = read_line(interpreter, handle, 1);
            n = first + 1;
        } else {
            lual_checkstack(interpreter, nargs + 20, c"too many arguments".as_ptr());
            let mut ok = 1i32;
            n = first;
            loop {
                let prev = nargs;
                nargs -= 1;
                if prev == 0 || ok == 0 {
                    break;
                }
                if lua_type(interpreter, n) == Some(TagType::Numeric) {
                    let l = lual_checkinteger(interpreter, n) as usize;
                    ok = if l == 0 {
                        test_eof(interpreter, handle)
                    } else {
                        read_chars(interpreter, handle, l)
                    };
                } else {
                    let mut p = lual_checklstring(interpreter, n, null_mut());
                    if *p == b'*' as i8 {
                        p = p.offset(1);
                    }
                    match *p as u8 {
                        b'n' => ok = read_number(interpreter, handle),
                        b'l' => ok = read_line(interpreter, handle, 1),
                        b'L' => ok = read_line(interpreter, handle, 0),
                        b'a' => {
                            read_all(interpreter, handle);
                            ok = 1;
                        }
                        _ => return lual_argerror(interpreter, n, c"invalid format".as_ptr()),
                    }
                }
                n += 1;
            }
            success = ok;
        }

        if (*handle).had_error() {
            return lual_fileresult(interpreter, 0, null());
        }
        if success == 0 {
            lua_settop(interpreter, -2);
            (*interpreter).push_nil();
        }
        n - first
    }
}

pub unsafe fn io_read(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        g_read(interpreter, getiofile(interpreter, c"_IO_input".as_ptr()), 1)
    }
}

pub unsafe fn f_read(interpreter: *mut Interpreter) -> i32 {
    unsafe { g_read(interpreter, tofile(interpreter), 2) }
}

// ─── Lines iterator ───────────────────────────────────────────────────────────

pub unsafe fn aux_lines(interpreter: *mut Interpreter, to_close: bool) {
    unsafe {
        let n = (*interpreter).get_top() - 1;
        if n > 250 {
            lual_argerror(interpreter, 252, c"too many arguments".as_ptr());
        }
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
        1
    }
}

pub unsafe fn io_lines(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let to_close: bool;
        if lua_type(interpreter, 1) == None {
            (*interpreter).push_nil();
        }
        if lua_type(interpreter, 1) == Some(TagType::Nil) {
            lua_getfield(interpreter, -(1000000i32) - 1000, c"_IO_input".as_ptr());
            lua_copy(interpreter, -1, 1);
            lua_settop(interpreter, -2);
            tofile(interpreter);
            to_close = false;
        } else {
            let filename = lual_checklstring(interpreter, 1, null_mut());
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
            4
        } else {
            1
        }
    }
}

pub unsafe fn io_readline(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let p = (*interpreter).to_pointer(-(1000000i32) - 1000 - 1) as *mut Stream;
        let n = lua_tointegerx(interpreter, -(1000000i32) - 1000 - 2, null_mut()) as i32;
        if (*p).stream_cfunctionclose.is_none() {
            return lual_error(interpreter, c"file is already closed".as_ptr());
        }
        lua_settop(interpreter, 1);
        lual_checkstack(interpreter, n, c"too many arguments".as_ptr());
        for i in 1..=n {
            lua_pushvalue(interpreter, -(1000000i32) - 1000 - (3 + i));
        }
        let n_read = g_read(interpreter, (*p).stream_handle, 2);
        if lua_toboolean(interpreter, -n_read) {
            return n_read;
        }
        if n_read > 1 {
            return lual_error(
                interpreter,
                c"%s".as_ptr(),
                lua_tolstring(interpreter, -n_read + 1, null_mut()),
            );
        }
        if lua_toboolean(interpreter, -(1000000i32) - 1000 - 3) {
            lua_settop(interpreter, 0);
            lua_pushvalue(interpreter, -(1000000i32) - 1000 - 1);
            aux_close(interpreter);
        }
        0
    }
}

// ─── Write functions ──────────────────────────────────────────────────────────

pub unsafe fn g_write(
    interpreter: *mut Interpreter,
    handle: *mut IoHandle,
    mut arg: i32,
) -> i32 {
    unsafe {
        let nargs = (*interpreter).get_top() - arg;
        let mut status = true;
        *errno_location() = 0;

        for _ in 0..nargs {
            if lua_type(interpreter, arg) == Some(TagType::Numeric) {
                status = status
                    && if lua_isinteger(interpreter, arg) {
                        let i = lua_tointegerx(interpreter, arg, null_mut());
                        handle_write_integer(handle, i)
                    } else {
                        let f = lua_tonumberx(interpreter, arg, null_mut());
                        handle_write_float(handle, f)
                    };
            } else {
                let mut l = 0usize;
                let s = lual_checklstring(interpreter, arg, &mut l);
                status = status
                    && handle_write_bytes(
                        handle,
                        std::slice::from_raw_parts(s as *const u8, l),
                    );
            }
            arg += 1;
        }

        if status {
            1
        } else {
            lual_fileresult(interpreter, 0, null())
        }
    }
}

pub unsafe fn io_write(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        g_write(interpreter, getiofile(interpreter, c"_IO_output".as_ptr()), 1)
    }
}

pub unsafe fn f_write(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let handle = tofile(interpreter);
        lua_pushvalue(interpreter, 1);
        g_write(interpreter, handle, 2)
    }
}

// ─── Seek / flush / setvbuf ───────────────────────────────────────────────────

pub unsafe fn f_seek(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        const SEEK_MODES: [SeekFrom; 3] = [
            SeekFrom::Start(0),
            SeekFrom::Current(0),
            SeekFrom::End(0),
        ];
        const MODE_NAMES: [*const i8; 4] =
            [c"set".as_ptr(), c"cur".as_ptr(), c"end".as_ptr(), null()];

        let handle = tofile(interpreter);
        let op = lual_checkoption(interpreter, 2, c"cur".as_ptr(), MODE_NAMES.as_ptr()) as usize;
        let offset = lual_optinteger(interpreter, 3, 0);

        *errno_location() = 0;

        let seek_from = match SEEK_MODES[op] {
            SeekFrom::Start(_) => {
                if offset < 0 {
                    return lual_argerror(
                        interpreter,
                        3,
                        c"not an integer in proper range".as_ptr(),
                    );
                }
                SeekFrom::Start(offset as u64)
            }
            SeekFrom::Current(_) => SeekFrom::Current(offset),
            SeekFrom::End(_) => SeekFrom::End(offset),
        };

        match &mut *handle {
            IoHandle::File(fh) => match fh.file.seek(seek_from) {
                Ok(pos) => {
                    (*interpreter).push_integer(pos as i64);
                    1
                }
                Err(e) => {
                    if let Some(code) = e.raw_os_error() {
                        *errno_location() = code;
                    }
                    lual_fileresult(interpreter, 0, null())
                }
            },
            IoHandle::Pipe(_) | IoHandle::Stdin { .. } | IoHandle::Stdout | IoHandle::Stderr => {
                lual_fileresult(interpreter, 0, null())
            }
        }
    }
}

pub unsafe fn f_setvbuf(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        const MODE_NAMES: [*const i8; 4] =
            [c"no".as_ptr(), c"full".as_ptr(), c"line".as_ptr(), null()];
        let handle = tofile(interpreter);
        let opt = lual_checkoption(interpreter, 2, null(), MODE_NAMES.as_ptr());
        let size = lual_optinteger(interpreter, 3, 1024) as usize;
        if let IoHandle::File(fh) = &mut *handle {
            // Flush any pending write buffer before changing mode
            fh.flush_write_buf();
            fh.write_buf_mode = match opt {
                0 => WriteBufMode::No,
                1 => WriteBufMode::Full(size),
                _ => WriteBufMode::Line,
            };
        }
        lual_fileresult(interpreter, 1, null())
    }
}

pub unsafe fn io_flush(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let handle = getiofile(interpreter, c"_IO_output".as_ptr());
        *errno_location() = 0;
        lual_fileresult(interpreter, handle_flush(handle) as i32, null())
    }
}

pub unsafe fn f_flush(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let handle = tofile(interpreter);
        *errno_location() = 0;
        lual_fileresult(interpreter, handle_flush(handle) as i32, null())
    }
}

// ─── Library / metatable registration ────────────────────────────────────────

pub const IO_FUNCTIONS: [RegisteredFunction; 11] = [
    RegisteredFunction {
        registeredfunction_name: c"close".as_ptr(),
        registeredfunction_function: Some(io_close as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"flush".as_ptr(),
        registeredfunction_function: Some(io_flush as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"input".as_ptr(),
        registeredfunction_function: Some(io_input as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"lines".as_ptr(),
        registeredfunction_function: Some(io_lines as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"open".as_ptr(),
        registeredfunction_function: Some(io_open as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"output".as_ptr(),
        registeredfunction_function: Some(io_output as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"popen".as_ptr(),
        registeredfunction_function: Some(io_popen as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"read".as_ptr(),
        registeredfunction_function: Some(io_read as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"tmpfile".as_ptr(),
        registeredfunction_function: Some(io_tmpfile as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"type".as_ptr(),
        registeredfunction_function: Some(io_type as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"write".as_ptr(),
        registeredfunction_function: Some(io_write as unsafe fn(*mut Interpreter) -> i32),
    },
];

pub const IO_METHODS: [RegisteredFunction; 7] = [
    RegisteredFunction {
        registeredfunction_name: c"read".as_ptr(),
        registeredfunction_function: Some(f_read as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"write".as_ptr(),
        registeredfunction_function: Some(f_write as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"lines".as_ptr(),
        registeredfunction_function: Some(f_lines as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"flush".as_ptr(),
        registeredfunction_function: Some(f_flush as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"seek".as_ptr(),
        registeredfunction_function: Some(f_seek as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"close".as_ptr(),
        registeredfunction_function: Some(f_close as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"setvbuf".as_ptr(),
        registeredfunction_function: Some(f_setvbuf as unsafe fn(*mut Interpreter) -> i32),
    },
];

pub const IO_METAMETHODS: [RegisteredFunction; 3] = [
    RegisteredFunction {
        registeredfunction_name: c"__gc".as_ptr(),
        registeredfunction_function: Some(f_gc as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"__close".as_ptr(),
        registeredfunction_function: Some(f_gc as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"__tostring".as_ptr(),
        registeredfunction_function: Some(f_tostring as unsafe fn(*mut Interpreter) -> i32),
    },
];

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

pub unsafe fn createstdfile(
    interpreter: *mut Interpreter,
    handle: *mut IoHandle,
    k: *const i8,
    fname: *const i8,
) {
    unsafe {
        let p = newprefile(interpreter);
        (*p).stream_handle = handle;
        (*p).stream_cfunctionclose =
            Some(io_noclose as unsafe fn(*mut Interpreter) -> i32);
        if !k.is_null() {
            lua_pushvalue(interpreter, -1);
            lua_setfield(interpreter, -(1000000i32) - 1000, k);
        }
        lua_setfield(interpreter, -2, fname);
    }
}

pub unsafe fn luaopen_io(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        (*interpreter).lua_createtable();
        lual_setfuncs(interpreter, IO_FUNCTIONS.as_ptr(), IO_FUNCTIONS.len(), 0);
        createmeta(interpreter);
        // Allocate heap handles for the standard streams. These are intentionally
        // not freed during normal operation (io_noclose prevents user close), and
        // are freed by f_gc when the Lua state is closed.
        let stdin_h = Box::into_raw(Box::new(IoHandle::Stdin { pushback: None }));
        let stdout_h = Box::into_raw(Box::new(IoHandle::Stdout));
        let stderr_h = Box::into_raw(Box::new(IoHandle::Stderr));
        createstdfile(interpreter, stdin_h, c"_IO_input".as_ptr(), c"stdin".as_ptr());
        createstdfile(interpreter, stdout_h, c"_IO_output".as_ptr(), c"stdout".as_ptr());
        createstdfile(interpreter, stderr_h, null_mut(), c"stderr".as_ptr());
        1
    }
}
