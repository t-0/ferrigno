use crate::buffer::*;
use crate::filehandle::*;
use crate::iohandle::*;
use crate::registeredfunction::*;
use crate::state::*;
use crate::stream::*;
use crate::tagtype::*;
use crate::tdefaultnew::*;
use crate::user::*;
use crate::utility::*;
use crate::writebuffermode::*;
use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom, Write};
use std::ptr::*;

const IO_BUFFERSIZE: usize = 1024;

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
fn mode_has_write(mode: &[u8]) -> bool {
    if mode.is_empty() {
        return false;
    }
    matches!(mode[0], b'w' | b'a') || (mode[0] == b'r' && mode.contains(&b'+'))
}

fn open_opts_from_mode(mode: &[u8]) -> Option<OpenOptions> {
    if mode.is_empty() {
        return None;
    }
    let base = mode[0];
    let plus = mode.contains(&b'+');
    let mut opts = OpenOptions::new();
    match base {
        | b'r' => {
            opts.read(true);
            if plus {
                opts.write(true);
            }
        },
        | b'w' => {
            opts.write(true).create(true).truncate(true);
            if plus {
                opts.read(true);
            }
        },
        | b'a' => {
            opts.append(true).create(true);
            if plus {
                opts.read(true);
            }
        },
        | _ => return None,
    }
    Some(opts)
}

// ─── Handle-level helpers ─────────────────────────────────────────────────────

/// Write bytes to an IoHandle, returning whether successful.
unsafe fn handle_write_bytes(handle: *mut IoHandle, data: &[u8]) -> bool {
    unsafe {
        match &mut *handle {
            | IoHandle::File(fh) => fh.write_buffered(data),
            | IoHandle::Pipe(ph) => {
                if let Some(ref mut writer) = ph.pipe_writer {
                    writer.write_all(data).is_ok()
                } else {
                    false
                }
            },
            | IoHandle::Stdout => std::io::stdout().write_all(data).is_ok(),
            | IoHandle::Stderr => std::io::stderr().write_all(data).is_ok(),
            | IoHandle::Stdin { .. } => false,
        }
    }
}

/// Write a formatted integer to an IoHandle.
unsafe fn handle_write_integer(handle: *mut IoHandle, i: i64) -> bool {
    unsafe {
        let mut buf = [0u8; 32];
        let n = {
            let mut cursor = std::io::Cursor::new(&mut buf[..]);
            write!(cursor, "{}", i).ok();
            cursor.position() as usize
        };
        handle_write_bytes(handle, &buf[..n])
    }
}

/// Write a formatted float to an IoHandle using %.15g with %.17g fallback.
unsafe fn handle_write_float(handle: *mut IoHandle, f: f64) -> bool {
    unsafe {
        let mut buf = [0u8; 64];
        let n = crate::utility::format_float_roundtrip(f, &mut buf);
        handle_write_bytes(handle, &buf[..n])
    }
}

/// Read all remaining bytes from an IoHandle into a Vec.
unsafe fn handle_read_all(handle: *mut IoHandle) -> Vec<u8> {
    unsafe {
        match &mut *handle {
            | IoHandle::File(fh) => {
                let mut buf = Vec::new();
                if let Some(b) = fh.filehandle_pushback.take() {
                    buf.push(b);
                }
                let _ = fh.filehandle_file.read_to_end(&mut buf);
                buf
            },
            | IoHandle::Pipe(ph) => {
                let mut buf = Vec::new();
                if let Some(ref mut reader) = ph.pipe_reader {
                    let mut chunk = [0u8; IO_BUFFERSIZE];
                    loop {
                        match reader.read(&mut chunk) {
                            | Ok(0) => break,
                            | Ok(n) => buf.extend_from_slice(&chunk[..n]),
                            | Err(_) => {
                                ph.pipe_had_error = true;
                                break;
                            },
                        }
                    }
                }
                buf
            },
            | IoHandle::Stdin { filehandle_pushback } => {
                let mut buf = Vec::new();
                if let Some(b) = filehandle_pushback.take() {
                    buf.push(b);
                }
                let _ = std::io::stdin().lock().read_to_end(&mut buf);
                buf
            },
            | _ => Vec::new(),
        }
    }
}

/// Read exactly n bytes from an IoHandle. Returns how many were actually read.
unsafe fn handle_read_n(handle: *mut IoHandle, n: usize) -> Vec<u8> {
    unsafe {
        match &mut *handle {
            | IoHandle::File(fh) => {
                let mut buf = Vec::with_capacity(n);
                if let Some(b) = fh.filehandle_pushback.take() {
                    buf.push(b);
                }
                let remain = n - buf.len();
                if remain > 0 {
                    let start = buf.len();
                    buf.resize(n, 0);
                    let nr = fh.filehandle_file.read(&mut buf[start..]).unwrap_or(0);
                    buf.truncate(start + nr);
                }
                buf
            },
            | IoHandle::Pipe(ph) => {
                let mut buf = vec![0u8; n];
                let nr = if let Some(ref mut reader) = ph.pipe_reader {
                    reader.read(&mut buf).unwrap_or(0)
                } else {
                    0
                };
                buf.truncate(nr);
                buf
            },
            | IoHandle::Stdin { filehandle_pushback } => {
                let mut buf = Vec::with_capacity(n);
                if let Some(b) = filehandle_pushback.take() {
                    buf.push(b);
                }
                let remain = n - buf.len();
                if remain > 0 {
                    let mut tmp = vec![0u8; remain];
                    let nr = std::io::stdin().lock().read(&mut tmp).unwrap_or(0);
                    buf.extend_from_slice(&tmp[..nr]);
                }
                buf
            },
            | _ => Vec::new(),
        }
    }
}

/// Flush an IoHandle's output buffer.
unsafe fn handle_flush(handle: *mut IoHandle) -> bool {
    unsafe { (*handle).flush() }
}

// ─── Stream userdata helpers ──────────────────────────────────────────────────

pub unsafe fn io_type(state: *mut State) -> i32 {
    unsafe {
        lual_checkany(state, 1);
        let p = lual_testudata(state, 1, c"FILE*".as_ptr()) as *mut Stream;
        if p.is_null() {
            (*state).push_nil();
        } else if (*p).stream_cfunctionclose.is_none() {
            lua_pushstring(state, c"closed file".as_ptr());
        } else {
            lua_pushstring(state, c"file".as_ptr());
        }
        1
    }
}

pub unsafe fn f_tostring(state: *mut State) -> i32 {
    unsafe {
        let p = lual_checkudata(state, 1, c"FILE*".as_ptr()) as *mut Stream;
        if (*p).stream_cfunctionclose.is_none() {
            lua_pushstring(state, c"file (closed)".as_ptr());
        } else {
            lua_pushfstring(state, c"file (%p)".as_ptr(), (*p).stream_handle);
        }
        1
    }
}

/// Get the open handle for the file at stack index 1, or error if closed.
pub unsafe fn tofile(state: *mut State) -> *mut IoHandle {
    unsafe {
        let p = lual_checkudata(state, 1, c"FILE*".as_ptr()) as *mut Stream;
        if (*p).stream_cfunctionclose.is_none() {
            lual_error(state, c"attempt to use a closed file".as_ptr());
        }
        (*p).stream_handle
    }
}

/// Allocate a new pre-file Stream userdata (handle and close function both null).
pub unsafe fn newprefile(state: *mut State) -> *mut Stream {
    unsafe {
        let p = User::lua_newuserdatauv(state, size_of::<Stream>(), 0) as *mut Stream;
        (*p).stream_handle = null_mut();
        (*p).stream_cfunctionclose = None;
        lual_setmetatable(state, c"FILE*".as_ptr());
        p
    }
}

/// Allocate a new file Stream userdata, wired up for io_fclose.
pub unsafe fn newfile(state: *mut State) -> *mut Stream {
    unsafe {
        let p = newprefile(state);
        (*p).stream_handle = null_mut();
        (*p).stream_cfunctionclose = Some(io_fclose as unsafe fn(*mut State) -> i32);
        p
    }
}

pub unsafe fn aux_close(state: *mut State) -> i32 {
    unsafe {
        let p = lual_checkudata(state, 1, c"FILE*".as_ptr()) as *mut Stream;
        let close_fn = (*p).stream_cfunctionclose;
        (*p).stream_cfunctionclose = None;
        (close_fn.expect("non-null function pointer"))(state)
    }
}

pub unsafe fn f_close(state: *mut State) -> i32 {
    unsafe {
        tofile(state); // validate open
        aux_close(state)
    }
}

pub unsafe fn io_close(state: *mut State) -> i32 {
    unsafe {
        if lua_type(state, 1).is_none() {
            lua_getfield(state, LUA_REGISTRYINDEX, c"_IO_output".as_ptr());
        }
        f_close(state)
    }
}

pub unsafe fn f_gc(state: *mut State) -> i32 {
    unsafe {
        let p = lual_checkudata(state, 1, c"FILE*".as_ptr()) as *mut Stream;
        // Call the close function if still open
        if (*p).stream_cfunctionclose.is_some() && !(*p).stream_handle.is_null() {
            aux_close(state);
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

pub unsafe fn io_fclose(state: *mut State) -> i32 {
    unsafe {
        let p = lual_checkudata(state, 1, c"FILE*".as_ptr()) as *mut Stream;
        let handle = (*p).stream_handle;
        (*p).stream_handle = null_mut(); // null before drop
        set_errno(0);
        // Flush write buffer before dropping (in case setvbuf buffering is active)
        if let IoHandle::File(fh) = &mut *handle {
            fh.flush_write_buf();
        }
        drop(Box::from_raw(handle)); // dropping IoHandle::File closes the File
        lual_fileresult(state, 1, null())
    }
}

pub unsafe fn io_pclose(state: *mut State) -> i32 {
    unsafe {
        let p = lual_checkudata(state, 1, c"FILE*".as_ptr()) as *mut Stream;
        let handle = (*p).stream_handle;
        (*p).stream_handle = null_mut();
        set_errno(0);
        let stat = if let IoHandle::Pipe(ph) = &mut *handle {
            // Drop pipe ends to signal EOF, then wait for child
            ph.pipe_reader = None;
            ph.pipe_writer = None;
            use std::os::unix::process::ExitStatusExt;
            match ph.pipe_child.wait() {
                | Ok(status) => status.into_raw(),
                | Err(_) => -1,
            }
        } else {
            0
        };
        drop(Box::from_raw(handle));
        lual_execresult(state, stat)
    }
}

pub unsafe fn io_noclose(state: *mut State) -> i32 {
    unsafe {
        let p = lual_checkudata(state, 1, c"FILE*".as_ptr()) as *mut Stream;
        // Reset cfunctionclose so the file appears open (cannot be user-closed)
        (*p).stream_cfunctionclose = Some(io_noclose as unsafe fn(*mut State) -> i32);
        (*state).push_nil();
        lua_pushstring(state, c"cannot close standard file".as_ptr());
        2
    }
}

// ─── File opening ─────────────────────────────────────────────────────────────

pub unsafe fn opencheck(state: *mut State, fname: *const i8, mode: *const i8) {
    unsafe {
        let p = newfile(state);
        let fname_str = std::ffi::CStr::from_ptr(fname).to_string_lossy();
        let mode_bytes = std::ffi::CStr::from_ptr(mode).to_bytes();
        match open_opts_from_mode(mode_bytes) {
            | Some(opts) => match opts.open(fname_str.as_ref()) {
                | Ok(file) => {
                    let fh = if mode_has_write(mode_bytes) {
                        FileHandle::new_buffered(file)
                    } else {
                        FileHandle::new(file)
                    };
                    let handle = Box::into_raw(Box::new(IoHandle::File(fh)));
                    (*p).stream_handle = handle;
                },
                | Err(e) => {
                    if let Some(code) = e.raw_os_error() {
                        set_errno(code);
                    }
                    lual_error(state, c"cannot open file '%s' (%s)".as_ptr(), fname, os_strerror(get_errno()));
                },
            },
            | None => {
                lual_error(state, c"invalid mode".as_ptr());
            },
        }
    }
}

pub unsafe fn io_open(state: *mut State) -> i32 {
    unsafe {
        let mut fnlen = 0usize;
        let fname = lual_checklstring(state, 1, &mut fnlen);
        let mode = lual_optlstring(state, 2, c"r".as_ptr(), null_mut());
        let mode_bytes = std::ffi::CStr::from_ptr(mode).to_bytes();
        if !check_mode(mode_bytes) {
            return lual_argerror(state, 2, c"invalid mode".as_ptr());
        }
        let p = newfile(state);
        let fname_str = std::ffi::CStr::from_ptr(fname).to_string_lossy();
        set_errno(0);
        match open_opts_from_mode(mode_bytes) {
            | Some(opts) => match opts.open(fname_str.as_ref()) {
                | Ok(file) => {
                    let fh = if mode_has_write(mode_bytes) {
                        FileHandle::new_buffered(file)
                    } else {
                        FileHandle::new(file)
                    };
                    let handle = Box::into_raw(Box::new(IoHandle::File(fh)));
                    (*p).stream_handle = handle;
                    1
                },
                | Err(e) => {
                    if let Some(code) = e.raw_os_error() {
                        set_errno(code);
                    }
                    lual_fileresult(state, 0, fname)
                },
            },
            | None => lual_argerror(state, 2, c"invalid mode".as_ptr()),
        }
    }
}

pub unsafe fn io_popen(state: *mut State) -> i32 {
    unsafe {
        let filename = lual_checklstring(state, 1, null_mut());
        let mode = lual_optlstring(state, 2, c"r".as_ptr(), null_mut());
        let mode_bytes = std::ffi::CStr::from_ptr(mode).to_bytes();
        let valid = matches!(mode_bytes, b"r" | b"w");
        if !valid {
            return lual_argerror(state, 2, c"invalid mode".as_ptr());
        }
        let p = newprefile(state);
        set_errno(0);
        std::io::stdout().flush().unwrap_or(());
        std::io::stderr().flush().unwrap_or(());
        let cmd_str = std::ffi::CStr::from_ptr(filename).to_string_lossy();
        let is_read = mode_bytes == b"r";
        let mut cmd = std::process::Command::new("sh");
        cmd.arg("-c").arg(cmd_str.as_ref());
        if is_read {
            cmd.stdout(std::process::Stdio::piped());
        } else {
            cmd.stdin(std::process::Stdio::piped());
        }
        match cmd.spawn() {
            | Ok(child) => {
                let ph = if is_read { PipeHandle::new_read(child) } else { PipeHandle::new_write(child) };
                let handle = Box::into_raw(Box::new(IoHandle::Pipe(ph)));
                (*p).stream_handle = handle;
                (*p).stream_cfunctionclose = Some(io_pclose as unsafe fn(*mut State) -> i32);
                1
            },
            | Err(e) => {
                if let Some(code) = e.raw_os_error() {
                    set_errno(code);
                }
                lual_fileresult(state, 0, filename)
            },
        }
    }
}

pub unsafe fn io_tmpfile(state: *mut State) -> i32 {
    unsafe {
        let p = newfile(state);
        set_errno(0);
        let dir = std::env::temp_dir();
        for i in 0u32..1000 {
            let path = dir.join(format!("lua_tmp_{}_{}", std::process::id(), i));
            match OpenOptions::new().read(true).write(true).create_new(true).open(&path) {
                | Ok(file) => {
                    std::fs::remove_file(&path).ok(); // unlink immediately — anonymous temp file
                    let handle = Box::into_raw(Box::new(IoHandle::File(FileHandle::new_buffered(file))));
                    (*p).stream_handle = handle;
                    return 1;
                },
                | Err(ref e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
                | Err(e) => {
                    if let Some(code) = e.raw_os_error() {
                        set_errno(code);
                    }
                    return lual_fileresult(state, 0, null());
                },
            }
        }
        set_errno(17); // EEXIST
        lual_fileresult(state, 0, null())
    }
}

// ─── Default I/O file management ─────────────────────────────────────────────

pub unsafe fn getiofile(state: *mut State, findex: *const i8) -> *mut IoHandle {
    unsafe {
        lua_getfield(state, LUA_REGISTRYINDEX, findex);
        let p = (*state).to_pointer(-1) as *mut Stream;
        if (*p).stream_cfunctionclose.is_none() {
            lual_error(state, c"default %s file is closed".as_ptr(), findex.add(4));
        }
        (*p).stream_handle
    }
}

pub unsafe fn g_iofile(state: *mut State, f: *const i8, mode: *const i8) -> i32 {
    unsafe {
        if !TagType::is_none_or_nil(lua_type(state, 1)) {
            let filename = lua_tolstring(state, 1, null_mut());
            if !filename.is_null() {
                opencheck(state, filename, mode);
            } else {
                tofile(state);
                lua_pushvalue(state, 1);
            }
            lua_setfield(state, LUA_REGISTRYINDEX, f);
        }
        lua_getfield(state, LUA_REGISTRYINDEX, f);
        1
    }
}

pub unsafe fn io_input(state: *mut State) -> i32 {
    unsafe { g_iofile(state, c"_IO_input".as_ptr(), c"r".as_ptr()) }
}

pub unsafe fn io_output(state: *mut State) -> i32 {
    unsafe { g_iofile(state, c"_IO_output".as_ptr(), c"w".as_ptr()) }
}

// ─── Read functions ───────────────────────────────────────────────────────────

/// Read a Lua number from the handle. Returns 1 on success (value on stack), 0 on failure.
unsafe fn read_number(state: *mut State, handle: *mut IoHandle) -> i32 {
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
                | None => {
                    (*state).push_nil();
                    return 0;
                },
                | Some(b) if (b as char).is_ascii_whitespace() => continue,
                | Some(b) => break b,
            }
        };

        // Optional sign
        if c == b'+' || c == b'-' {
            if !push(&mut buf, &mut n, c) {
                (*state).push_nil();
                return 0;
            }
            c = match (*handle).read_byte() {
                | Some(b) => b,
                | None => {
                    buf[n] = 0;
                    if lua_stringtonumber(state, buf.as_mut_ptr()) != 0 {
                        return 1;
                    }
                    (*state).push_nil();
                    return 0;
                },
            };
        }

        let mut count = 0i32;
        let mut hex = false;

        // Leading zero + possible 0x/0X
        if c == b'0' {
            if !push(&mut buf, &mut n, c) {
                (*state).push_nil();
                return 0;
            }
            count = 1;
            c = match (*handle).read_byte() {
                | Some(b) => b,
                | None => {
                    buf[n] = 0;
                    return if lua_stringtonumber(state, buf.as_mut_ptr()) != 0 {
                        1
                    } else {
                        (*state).push_nil();
                        0
                    };
                },
            };
            if c == b'x' || c == b'X' {
                hex = true;
                if !push(&mut buf, &mut n, c) {
                    (*state).push_nil();
                    return 0;
                }
                c = (*handle).read_byte().unwrap_or(0);
            }
        }

        // Integer digits
        loop {
            let is_digit = if hex { c.is_ascii_hexdigit() } else { c.is_ascii_digit() };
            if !is_digit {
                break;
            }
            if !push(&mut buf, &mut n, c) {
                break;
            }
            count += 1;
            c = (*handle).read_byte().unwrap_or(0);
        }

        // Decimal point
        let lc = localeconv();
        let dec_point = if !lc.is_null() && !(*lc).decimal_point.is_null() {
            *(*lc).decimal_point as u8
        } else {
            b'.'
        };
        if c == b'.' || c == dec_point {
            if !push(&mut buf, &mut n, b'.') {
                (*state).push_nil();
                return 0;
            }
            c = (*handle).read_byte().unwrap_or(0);
            loop {
                let is_digit = if hex { c.is_ascii_hexdigit() } else { c.is_ascii_digit() };
                if !is_digit {
                    break;
                }
                if !push(&mut buf, &mut n, c) {
                    break;
                }
                count += 1;
                c = (*handle).read_byte().unwrap_or(0);
            }
        }

        // Exponent
        if count > 0 {
            let exp_char = if hex { b'p' } else { b'e' };
            if c == exp_char || c == exp_char.to_ascii_uppercase() {
                if !push(&mut buf, &mut n, c) {
                    (*state).push_nil();
                    return 0;
                }
                c = (*handle).read_byte().unwrap_or(0);
                if c == b'+' || c == b'-' {
                    if !push(&mut buf, &mut n, c) {
                        (*state).push_nil();
                        return 0;
                    }
                    c = (*handle).read_byte().unwrap_or(0);
                }
                while c.is_ascii_digit() {
                    if !push(&mut buf, &mut n, c) {
                        break;
                    }
                    c = (*handle).read_byte().unwrap_or(0);
                }
            }
        }

        // Push back the character that didn't fit
        if c != 0 {
            (*handle).unread_byte(c);
        }
        buf[n] = 0;

        if lua_stringtonumber(state, buf.as_mut_ptr()) != 0 {
            1
        } else {
            (*state).push_nil();
            0
        }
    }
}

/// Test for EOF: push "" and return 1 if not at EOF, 0 if at EOF.
unsafe fn test_eof(state: *mut State, handle: *mut IoHandle) -> i32 {
    unsafe {
        match (*handle).read_byte() {
            | None => {
                lua_pushstring(state, c"".as_ptr());
                0
            },
            | Some(b) => {
                (*handle).unread_byte(b);
                lua_pushstring(state, c"".as_ptr());
                1
            },
        }
    }
}

/// Read a line from the handle. If chop==1, strip the trailing newline.
unsafe fn read_line(state: *mut State, handle: *mut IoHandle, chop: i32) -> i32 {
    unsafe {
        let mut b = Buffer::new();
        b.initialize(state);
        let last_c: Option<u8>;

        'outer: loop {
            let buf_ptr = b.prepare_with_size(IO_BUFFERSIZE);
            let buf = std::slice::from_raw_parts_mut(buf_ptr as *mut u8, IO_BUFFERSIZE);
            let mut i = 0usize;
            loop {
                match (*handle).read_byte() {
                    | None => {
                        b.buffer_loads.set_length(b.buffer_loads.get_length() as usize + i);
                        last_c = None;
                        break 'outer;
                    },
                    | Some(b'\n') => {
                        b.buffer_loads.set_length(b.buffer_loads.get_length() as usize + i);
                        last_c = Some(b'\n');
                        break 'outer;
                    },
                    | Some(byte) => {
                        buf[i] = byte;
                        i += 1;
                        if i == IO_BUFFERSIZE {
                            b.buffer_loads.set_length(b.buffer_loads.get_length() as usize + i);
                            break;
                        }
                    },
                }
            }
        }

        // If not chopping newline, add it back
        if chop == 0
            && let Some(b'\n') = last_c
        {
            let ptr = b.prepare_with_size(1);
            *ptr = b'\n' as i8;
            b.buffer_loads.set_length(b.buffer_loads.get_length() as usize + 1);
        }

        b.push_result();
        (last_c == Some(b'\n') || get_length_raw(state, -1) > 0) as i32
    }
}

/// Read all remaining content from the handle onto the Lua stack.
unsafe fn read_all(state: *mut State, handle: *mut IoHandle) {
    unsafe {
        let data = handle_read_all(handle);
        lua_pushlstring(state, data.as_ptr() as *const i8, data.len());
    }
}

/// Read n bytes from the handle. Returns 1 if any bytes read, 0 if EOF.
unsafe fn read_chars(state: *mut State, handle: *mut IoHandle, n: usize) -> i32 {
    unsafe {
        let data = handle_read_n(handle, n);
        let len = data.len();
        lua_pushlstring(state, data.as_ptr() as *const i8, len);
        (len > 0) as i32
    }
}

pub unsafe fn g_read(state: *mut State, handle: *mut IoHandle, first: i32) -> i32 {
    unsafe {
        let mut nargs = (*state).get_top() - 1;
        let mut n: i32;
        let success: i32;

        (*handle).clear_error();
        set_errno(0);
        // Flush write buffer before reading to ensure correct file position
        if let IoHandle::File(fh) = &mut *handle {
            fh.flush_write_buf();
        }

        if nargs == 0 {
            success = read_line(state, handle, 1);
            n = first + 1;
        } else {
            lual_checkstack(state, nargs + 20, c"too many arguments".as_ptr());
            let mut ok = 1i32;
            n = first;
            loop {
                let prev = nargs;
                nargs -= 1;
                if prev == 0 || ok == 0 {
                    break;
                }
                if lua_type(state, n) == Some(TagType::Numeric) {
                    let l = lual_checkinteger(state, n) as usize;
                    ok = if l == 0 { test_eof(state, handle) } else { read_chars(state, handle, l) };
                } else {
                    let mut p = lual_checklstring(state, n, null_mut());
                    if *p == b'*' as i8 {
                        p = p.add(1);
                    }
                    match *p as u8 {
                        | b'n' => ok = read_number(state, handle),
                        | b'l' => ok = read_line(state, handle, 1),
                        | b'L' => ok = read_line(state, handle, 0),
                        | b'a' => {
                            read_all(state, handle);
                            ok = 1;
                        },
                        | _ => return lual_argerror(state, n, c"invalid format".as_ptr()),
                    }
                }
                n += 1;
            }
            success = ok;
        }

        if (*handle).filehandle_had_error() {
            return lual_fileresult(state, 0, null());
        }
        if success == 0 {
            lua_settop(state, -2);
            (*state).push_nil();
        }
        n - first
    }
}

pub unsafe fn io_read(state: *mut State) -> i32 {
    unsafe { g_read(state, getiofile(state, c"_IO_input".as_ptr()), 1) }
}

pub unsafe fn f_read(state: *mut State) -> i32 {
    unsafe { g_read(state, tofile(state), 2) }
}

// ─── Lines iterator ───────────────────────────────────────────────────────────

pub unsafe fn aux_lines(state: *mut State, to_close: bool) {
    unsafe {
        let n = (*state).get_top() - 1;
        if n > 250 {
            lual_argerror(state, 252, c"too many arguments".as_ptr());
        }
        lua_pushvalue(state, 1);
        (*state).push_integer(n as i64);
        (*state).push_boolean(to_close);
        lua_rotate(state, 2, 3);
        lua_pushcclosure(state, Some(io_readline as unsafe fn(*mut State) -> i32), 3 + n);
    }
}

pub unsafe fn f_lines(state: *mut State) -> i32 {
    unsafe {
        tofile(state);
        aux_lines(state, false);
        1
    }
}

pub unsafe fn io_lines(state: *mut State) -> i32 {
    unsafe {
        let to_close: bool;
        if lua_type(state, 1).is_none() {
            (*state).push_nil();
        }
        if lua_type(state, 1) == Some(TagType::Nil) {
            lua_getfield(state, LUA_REGISTRYINDEX, c"_IO_input".as_ptr());
            lua_copy(state, -1, 1);
            lua_settop(state, -2);
            tofile(state);
            to_close = false;
        } else {
            let filename = lual_checklstring(state, 1, null_mut());
            opencheck(state, filename, c"r".as_ptr());
            lua_copy(state, -1, 1);
            lua_settop(state, -2);
            to_close = true;
        }
        aux_lines(state, to_close);
        if to_close {
            (*state).push_nil();
            (*state).push_nil();
            lua_pushvalue(state, 1);
            4
        } else {
            1
        }
    }
}

pub unsafe fn io_readline(state: *mut State) -> i32 {
    unsafe {
        let p = (*state).to_pointer(LUA_REGISTRYINDEX - 1) as *mut Stream;
        let n = lua_tointegerx(state, LUA_REGISTRYINDEX - 2, null_mut()) as i32;
        if (*p).stream_cfunctionclose.is_none() {
            return lual_error(state, c"file is already closed".as_ptr());
        }
        lua_settop(state, 1);
        lual_checkstack(state, n, c"too many arguments".as_ptr());
        for i in 1..=n {
            lua_pushvalue(state, LUA_REGISTRYINDEX - (3 + i));
        }
        let n_read = g_read(state, (*p).stream_handle, 2);
        if lua_toboolean(state, -n_read) {
            return n_read;
        }
        if n_read > 1 {
            return lual_error(state, c"%s".as_ptr(), lua_tolstring(state, -n_read + 1, null_mut()));
        }
        if lua_toboolean(state, LUA_REGISTRYINDEX - 3) {
            lua_settop(state, 0);
            lua_pushvalue(state, LUA_REGISTRYINDEX - 1);
            aux_close(state);
        }
        0
    }
}

// ─── Write functions ──────────────────────────────────────────────────────────

pub unsafe fn g_write(state: *mut State, handle: *mut IoHandle, mut arg: i32) -> i32 {
    unsafe {
        let nargs = (*state).get_top() - arg;
        let mut status = true;
        set_errno(0);

        for _ in 0..nargs {
            if lua_type(state, arg) == Some(TagType::Numeric) {
                status = status
                    && if lua_isinteger(state, arg) {
                        let i = lua_tointegerx(state, arg, null_mut());
                        handle_write_integer(handle, i)
                    } else {
                        let f = lua_tonumberx(state, arg, null_mut());
                        handle_write_float(handle, f)
                    };
            } else {
                let mut l = 0usize;
                let s = lual_checklstring(state, arg, &mut l);
                status = status && handle_write_bytes(handle, std::slice::from_raw_parts(s as *const u8, l));
            }
            arg += 1;
        }

        if status { 1 } else { lual_fileresult(state, 0, null()) }
    }
}

pub unsafe fn io_write(state: *mut State) -> i32 {
    unsafe { g_write(state, getiofile(state, c"_IO_output".as_ptr()), 1) }
}

pub unsafe fn f_write(state: *mut State) -> i32 {
    unsafe {
        let handle = tofile(state);
        lua_pushvalue(state, 1);
        g_write(state, handle, 2)
    }
}

// ─── Seek / flush / setvbuf ───────────────────────────────────────────────────

pub unsafe fn f_seek(state: *mut State) -> i32 {
    unsafe {
        const SEEK_MODES: [SeekFrom; 3] = [SeekFrom::Start(0), SeekFrom::Current(0), SeekFrom::End(0)];
        const MODE_NAMES: [*const i8; 4] = [c"set".as_ptr(), c"cur".as_ptr(), c"end".as_ptr(), null()];

        let handle = tofile(state);
        let op = lual_checkoption(state, 2, c"cur".as_ptr(), MODE_NAMES.as_ptr()) as usize;
        let offset = lual_optinteger(state, 3, 0);

        set_errno(0);

        let seek_from = match SEEK_MODES[op] {
            | SeekFrom::Start(_) => {
                if offset < 0 {
                    return lual_argerror(state, 3, c"not an integer in proper range".as_ptr());
                }
                SeekFrom::Start(offset as u64)
            },
            | SeekFrom::Current(_) => SeekFrom::Current(offset),
            | SeekFrom::End(_) => SeekFrom::End(offset),
        };

        match &mut *handle {
            | IoHandle::File(fh) => {
                fh.flush_write_buf();
                match fh.filehandle_file.seek(seek_from) {
                    | Ok(pos) => {
                        (*state).push_integer(pos as i64);
                        1
                    },
                    | Err(e) => {
                        if let Some(code) = e.raw_os_error() {
                            set_errno(code);
                        }
                        lual_fileresult(state, 0, null())
                    },
                }
            },
            | IoHandle::Pipe(_) | IoHandle::Stdin { .. } | IoHandle::Stdout | IoHandle::Stderr => lual_fileresult(state, 0, null()),
        }
    }
}

pub unsafe fn f_setvbuf(state: *mut State) -> i32 {
    unsafe {
        const MODE_NAMES: [*const i8; 4] = [c"no".as_ptr(), c"full".as_ptr(), c"line".as_ptr(), null()];
        let handle = tofile(state);
        let opt = lual_checkoption(state, 2, null(), MODE_NAMES.as_ptr());
        let size = lual_optinteger(state, 3, IO_BUFFERSIZE as i64) as usize;
        if let IoHandle::File(fh) = &mut *handle {
            // Flush any pending write buffer before changing mode
            fh.flush_write_buf();
            fh.filehandle_write_buffer_mode = match opt {
                | 0 => WriteBufferMode::No,
                | 1 => WriteBufferMode::Full(size),
                | _ => WriteBufferMode::Line,
            };
        }
        lual_fileresult(state, 1, null())
    }
}

pub unsafe fn io_flush(state: *mut State) -> i32 {
    unsafe {
        let handle = getiofile(state, c"_IO_output".as_ptr());
        set_errno(0);
        lual_fileresult(state, handle_flush(handle) as i32, null())
    }
}

pub unsafe fn f_flush(state: *mut State) -> i32 {
    unsafe {
        let handle = tofile(state);
        set_errno(0);
        lual_fileresult(state, handle_flush(handle) as i32, null())
    }
}

// ─── Library / metatable registration ────────────────────────────────────────

pub const IO_FUNCTIONS: [RegisteredFunction; 11] = [
    RegisteredFunction {
        registeredfunction_name: c"close".as_ptr(),
        registeredfunction_function: Some(io_close as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"flush".as_ptr(),
        registeredfunction_function: Some(io_flush as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"input".as_ptr(),
        registeredfunction_function: Some(io_input as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"lines".as_ptr(),
        registeredfunction_function: Some(io_lines as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"open".as_ptr(),
        registeredfunction_function: Some(io_open as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"output".as_ptr(),
        registeredfunction_function: Some(io_output as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"popen".as_ptr(),
        registeredfunction_function: Some(io_popen as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"read".as_ptr(),
        registeredfunction_function: Some(io_read as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"tmpfile".as_ptr(),
        registeredfunction_function: Some(io_tmpfile as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"type".as_ptr(),
        registeredfunction_function: Some(io_type as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"write".as_ptr(),
        registeredfunction_function: Some(io_write as unsafe fn(*mut State) -> i32),
    },
];

pub const IO_METHODS: [RegisteredFunction; 7] = [
    RegisteredFunction {
        registeredfunction_name: c"read".as_ptr(),
        registeredfunction_function: Some(f_read as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"write".as_ptr(),
        registeredfunction_function: Some(f_write as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"lines".as_ptr(),
        registeredfunction_function: Some(f_lines as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"flush".as_ptr(),
        registeredfunction_function: Some(f_flush as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"seek".as_ptr(),
        registeredfunction_function: Some(f_seek as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"close".as_ptr(),
        registeredfunction_function: Some(f_close as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"setvbuf".as_ptr(),
        registeredfunction_function: Some(f_setvbuf as unsafe fn(*mut State) -> i32),
    },
];

pub const IO_METAMETHODS: [RegisteredFunction; 3] = [
    RegisteredFunction {
        registeredfunction_name: c"__gc".as_ptr(),
        registeredfunction_function: Some(f_gc as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"__close".as_ptr(),
        registeredfunction_function: Some(f_gc as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"__tostring".as_ptr(),
        registeredfunction_function: Some(f_tostring as unsafe fn(*mut State) -> i32),
    },
];

pub unsafe fn createmeta(state: *mut State) {
    unsafe {
        lual_newmetatable(state, c"FILE*".as_ptr());
        lual_setfuncs(state, IO_METAMETHODS.as_ptr(), IO_METAMETHODS.len(), 0);
        (*state).lua_createtable();
        lual_setfuncs(state, IO_METHODS.as_ptr(), IO_METHODS.len(), 0);
        lua_setfield(state, -2, c"__index".as_ptr());
        lua_settop(state, -2);
    }
}

pub unsafe fn createstdfile(state: *mut State, handle: *mut IoHandle, k: *const i8, fname: *const i8) {
    unsafe {
        let p = newprefile(state);
        (*p).stream_handle = handle;
        (*p).stream_cfunctionclose = Some(io_noclose as unsafe fn(*mut State) -> i32);
        if !k.is_null() {
            lua_pushvalue(state, -1);
            lua_setfield(state, LUA_REGISTRYINDEX, k);
        }
        lua_setfield(state, -2, fname);
    }
}

pub unsafe fn luaopen_io(state: *mut State) -> i32 {
    unsafe {
        (*state).lua_createtable();
        lual_setfuncs(state, IO_FUNCTIONS.as_ptr(), IO_FUNCTIONS.len(), 0);
        createmeta(state);
        // Allocate heap handles for the standard streams. These are intentionally
        // not freed during normal operation (io_noclose prevents user close), and
        // are freed by f_gc when the Lua state is closed.
        let stdin_h = Box::into_raw(Box::new(IoHandle::Stdin { filehandle_pushback: None }));
        let stdout_h = Box::into_raw(Box::new(IoHandle::Stdout));
        let stderr_h = Box::into_raw(Box::new(IoHandle::Stderr));
        createstdfile(state, stdin_h, c"_IO_input".as_ptr(), c"stdin".as_ptr());
        createstdfile(state, stdout_h, c"_IO_output".as_ptr(), c"stdout".as_ptr());
        createstdfile(state, stderr_h, null_mut(), c"stderr".as_ptr());
        1
    }
}
