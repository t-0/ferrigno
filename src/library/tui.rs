use crate::interpreter::*;
use crate::registeredfunction::*;
use crate::tagtype::*;
use std::ptr::*;

// ─── Terminal state ───────────────────────────────────────────────────────────

// Use zeroed() as const for static initialisation (all-zero is valid for
// libc::termios since it is a plain-old-data struct).
static mut SAVED_TERMIOS: libc::termios = unsafe { std::mem::zeroed() };
static mut HAS_SAVED_TERMIOS: bool = false;
static mut IN_RAW_MODE: bool = false;

// ─── Low-level I/O ───────────────────────────────────────────────────────────

unsafe fn write_stdout(data: &[u8]) {
    unsafe {
        let mut offset = 0;
        while offset < data.len() {
            let n = libc::write(
                libc::STDOUT_FILENO,
                data[offset..].as_ptr() as *const libc::c_void,
                data.len() - offset,
            );
            if n <= 0 {
                break;
            }
            offset += n as usize;
        }
    }
}

// Read one byte with a timeout in ms.  -1 = block forever, 0 = non-blocking.
unsafe fn read_byte(timeout_ms: i32) -> Option<u8> {
    unsafe {
        let mut pfd = libc::pollfd {
            fd: libc::STDIN_FILENO,
            events: libc::POLLIN,
            revents: 0,
        };
        let r = libc::poll(&mut pfd as *mut libc::pollfd, 1, timeout_ms);
        if r <= 0 {
            return None;
        }
        let mut b: u8 = 0;
        let n = libc::read(
            libc::STDIN_FILENO,
            &mut b as *mut u8 as *mut libc::c_void,
            1,
        );
        if n == 1 { Some(b) } else { None }
    }
}

// ─── Escape-sequence parsing ──────────────────────────────────────────────────

fn decode_csi(params: &[u8], final_byte: u8) -> Vec<u8> {
    // Parse optional modifier from params (e.g. "1;2" → modifier=2 = shift).
    // xterm encodes: modifier_value = (shift?1:0)|(alt?2:0)|(ctrl?4:0) + 1
    let modifier: u8 = std::str::from_utf8(params)
        .unwrap_or("")
        .split(';')
        .nth(1)
        .and_then(|m| m.parse().ok())
        .unwrap_or(1);
    let shifted = (modifier.saturating_sub(1) & 1) != 0;

    match final_byte {
        b'A' if shifted => b"shift-up".to_vec(),
        b'B' if shifted => b"shift-down".to_vec(),
        b'C' if shifted => b"shift-right".to_vec(),
        b'D' if shifted => b"shift-left".to_vec(),
        b'A' => b"up".to_vec(),
        b'B' => b"down".to_vec(),
        b'C' => b"right".to_vec(),
        b'D' => b"left".to_vec(),
        b'E' => b"down".to_vec(),    // cursor next N lines
        b'F' => b"end".to_vec(),
        b'G' => b"home".to_vec(),
        b'H' => b"home".to_vec(),
        b'P' => b"f1".to_vec(),
        b'Q' => b"f2".to_vec(),
        b'R' => b"f3".to_vec(),
        b'S' => b"f4".to_vec(),
        b'Z' => b"shift-tab".to_vec(),
        b'~' => {
            // First numeric parameter selects the key
            let n: u32 = std::str::from_utf8(params)
                .unwrap_or("")
                .split(';')
                .next()
                .unwrap_or("")
                .parse()
                .unwrap_or(0);
            match n {
                1 | 7 => b"home".to_vec(),
                2     => b"ins".to_vec(),
                3     => b"del".to_vec(),
                4 | 8 => b"end".to_vec(),
                5     => b"pgup".to_vec(),
                6     => b"pgdn".to_vec(),
                11    => b"f1".to_vec(),
                12    => b"f2".to_vec(),
                13    => b"f3".to_vec(),
                14    => b"f4".to_vec(),
                15    => b"f5".to_vec(),
                17    => b"f6".to_vec(),
                18    => b"f7".to_vec(),
                19    => b"f8".to_vec(),
                20    => b"f9".to_vec(),
                21    => b"f10".to_vec(),
                23    => b"f11".to_vec(),
                24    => b"f12".to_vec(),
                _     => {
                    let mut s = b"esc-[".to_vec();
                    s.extend_from_slice(params);
                    s.push(final_byte);
                    s
                }
            }
        }
        _ => {
            let mut s = b"esc-[".to_vec();
            s.extend_from_slice(params);
            s.push(final_byte);
            s
        }
    }
}

unsafe fn parse_csi() -> Vec<u8> {
    unsafe {
        let mut params: Vec<u8> = Vec::new();
        loop {
            match read_byte(50) {
                None => return b"esc".to_vec(),
                Some(b) if (0x40..=0x7E).contains(&b) => {
                    return decode_csi(&params, b);
                }
                Some(b) => {
                    params.push(b);
                }
            }
        }
    }
}

unsafe fn parse_ss3() -> Vec<u8> {
    unsafe {
        match read_byte(50) {
            Some(b'A') => b"up".to_vec(),
            Some(b'B') => b"down".to_vec(),
            Some(b'C') => b"right".to_vec(),
            Some(b'D') => b"left".to_vec(),
            Some(b'H') => b"home".to_vec(),
            Some(b'F') => b"end".to_vec(),
            Some(b'P') => b"f1".to_vec(),
            Some(b'Q') => b"f2".to_vec(),
            Some(b'R') => b"f3".to_vec(),
            Some(b'S') => b"f4".to_vec(),
            Some(b)    => {
                let mut s = b"alt-".to_vec();
                if (0x20..0x7F).contains(&b) {
                    s.push(b);
                }
                s
            }
            None => b"esc".to_vec(),
        }
    }
}

// Collect a partial UTF-8 sequence: we already have the leading byte,
// now read n_extra continuation bytes (best-effort).
unsafe fn read_utf8_tail(first: u8, n_extra: usize) -> Vec<u8> {
    unsafe {
        let mut buf = vec![first];
        for _ in 0..n_extra {
            match read_byte(20) {
                Some(b) => buf.push(b),
                None    => break,
            }
        }
        buf
    }
}

unsafe fn parse_key(timeout_ms: i32) -> Option<Vec<u8>> {
    unsafe {
        let b = read_byte(timeout_ms)?;
        Some(match b {
            0   => b"ctrl-space".to_vec(),
            8   => b"backspace".to_vec(),
            9   => b"tab".to_vec(),
            10  => b"enter".to_vec(),
            13  => b"enter".to_vec(),
            27  => {
                // ESC — try to parse an escape sequence
                match read_byte(50) {
                    None          => b"esc".to_vec(),
                    Some(b'[')    => parse_csi(),
                    Some(b'O')    => parse_ss3(),
                    Some(x) if (0x20..0x7F).contains(&x) => {
                        let mut s = b"alt-".to_vec();
                        s.push(x);
                        s
                    }
                    Some(_) => b"esc".to_vec(),
                }
            }
            127 => b"backspace".to_vec(),
            1..=26 => {
                // Ctrl-A (1) through Ctrl-Z (26)
                let letter = b'a' + b - 1;
                let mut s = b"ctrl-".to_vec();
                s.push(letter);
                s
            }
            28  => b"ctrl-\\".to_vec(),
            29  => b"ctrl-]".to_vec(),
            30  => b"ctrl-^".to_vec(),
            31  => b"ctrl-_".to_vec(),
            0x20..=0x7E => vec![b],
            // UTF-8 multi-byte characters
            0xC0..=0xDF => read_utf8_tail(b, 1),
            0xE0..=0xEF => read_utf8_tail(b, 2),
            0xF0..=0xF7 => read_utf8_tail(b, 3),
            _           => vec![b],
        })
    }
}

// ─── Color and attribute helpers ──────────────────────────────────────────────

// fg / bg: 0-7 = standard colours, 8-15 = bright, -1 = no change.
// attrs: bitmask — BOLD=1, DIM=2, ITALIC=4, UNDERLINE=8, BLINK=16, REVERSE=32.
fn color_escape(fg: i32, bg: i32, attrs: i32) -> Vec<u8> {
    let mut codes: Vec<u8> = Vec::new();

    let mut push_code = |s: &str| {
        if !codes.is_empty() { codes.push(b';'); }
        codes.extend_from_slice(s.as_bytes());
    };

    if attrs & 1  != 0 { push_code("1"); }   // bold
    if attrs & 2  != 0 { push_code("2"); }   // dim
    if attrs & 4  != 0 { push_code("3"); }   // italic
    if attrs & 8  != 0 { push_code("4"); }   // underline
    if attrs & 16 != 0 { push_code("5"); }   // blink
    if attrs & 32 != 0 { push_code("7"); }   // reverse

    if fg >= 0 {
        let s = if fg < 8 {
            format!("{}", 30 + fg)
        } else {
            format!("{}", 90 + (fg - 8))
        };
        push_code(&s);
    }

    if bg >= 0 {
        let s = if bg < 8 {
            format!("{}", 40 + bg)
        } else {
            format!("{}", 100 + (bg - 8))
        };
        push_code(&s);
    }

    let mut buf = b"\x1b[".to_vec();
    if codes.is_empty() {
        buf.push(b'0');
    } else {
        buf.extend_from_slice(&codes);
    }
    buf.push(b'm');
    buf
}

// ─── tui.size() → cols, rows ─────────────────────────────────────────────────

pub unsafe fn tui_size(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let mut ws: libc::winsize = std::mem::zeroed();
        if libc::ioctl(libc::STDOUT_FILENO, libc::TIOCGWINSZ, &mut ws) == 0 {
            (*interpreter).push_integer(ws.ws_col as i64);
            (*interpreter).push_integer(ws.ws_row as i64);
        } else {
            (*interpreter).push_integer(80);
            (*interpreter).push_integer(24);
        }
        2
    }
}

// ─── tui.clear() — clear screen and home cursor ───────────────────────────────

pub unsafe fn tui_clear(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        write_stdout(b"\x1b[2J\x1b[H");
        let _ = interpreter;
        0
    }
}

// ─── tui.clear_line() — clear from cursor to end of line ─────────────────────

pub unsafe fn tui_clear_line(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        write_stdout(b"\x1b[2K");
        let _ = interpreter;
        0
    }
}

// ─── tui.flush() — no-op (write() to STDOUT_FILENO is synchronous) ───────────

pub unsafe fn tui_flush(interpreter: *mut Interpreter) -> i32 {
    let _ = interpreter;
    0
}

// ─── tui.move(row, col) — position cursor (1-based) ──────────────────────────

pub unsafe fn tui_move(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let row = lual_checkinteger(interpreter, 1);
        let col = lual_checkinteger(interpreter, 2);
        let esc = format!("\x1b[{};{}H", row, col);
        write_stdout(esc.as_bytes());
        0
    }
}

// ─── tui.hide_cursor() / tui.show_cursor() ───────────────────────────────────

pub unsafe fn tui_hide_cursor(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        write_stdout(b"\x1b[?25l");
        let _ = interpreter;
        0
    }
}

pub unsafe fn tui_show_cursor(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        write_stdout(b"\x1b[?25h");
        let _ = interpreter;
        0
    }
}

// ─── tui.print(text) — write text to terminal ────────────────────────────────

pub unsafe fn tui_print(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let mut slen = 0usize;
        let sptr = lual_checklstring(interpreter, 1, &mut slen);
        write_stdout(std::slice::from_raw_parts(sptr as *const u8, slen));
        0
    }
}

// ─── tui.print_at(row, col, text [, fg [, bg [, attrs]]]) ───────────────────

pub unsafe fn tui_print_at(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let row   = lual_checkinteger(interpreter, 1);
        let col   = lual_checkinteger(interpreter, 2);
        let mut slen = 0usize;
        let sptr  = lual_checklstring(interpreter, 3, &mut slen);

        let has_fg    = lua_type(interpreter, 4) == Some(TagType::Numeric);
        let has_bg    = lua_type(interpreter, 5) == Some(TagType::Numeric);
        let has_attrs = lua_type(interpreter, 6) == Some(TagType::Numeric);
        let colorise  = has_fg || has_bg || has_attrs;

        // Move cursor
        let move_esc = format!("\x1b[{};{}H", row, col);
        write_stdout(move_esc.as_bytes());

        if colorise {
            let fg    = if has_fg    { lua_tointegerx(interpreter, 4, null_mut()) as i32 } else { -1 };
            let bg    = if has_bg    { lua_tointegerx(interpreter, 5, null_mut()) as i32 } else { -1 };
            let attrs = if has_attrs { lua_tointegerx(interpreter, 6, null_mut()) as i32 } else { 0  };
            write_stdout(&color_escape(fg, bg, attrs));
        }

        write_stdout(std::slice::from_raw_parts(sptr as *const u8, slen));

        if colorise {
            write_stdout(b"\x1b[0m");
        }

        0
    }
}

// ─── tui.color([fg [, bg [, attrs]]]) → ANSI escape string ──────────────────

pub unsafe fn tui_color(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let fg    = if lua_type(interpreter, 1) == Some(TagType::Numeric) {
            lua_tointegerx(interpreter, 1, null_mut()) as i32
        } else { -1 };
        let bg    = if lua_type(interpreter, 2) == Some(TagType::Numeric) {
            lua_tointegerx(interpreter, 2, null_mut()) as i32
        } else { -1 };
        let attrs = if lua_type(interpreter, 3) == Some(TagType::Numeric) {
            lua_tointegerx(interpreter, 3, null_mut()) as i32
        } else { 0 };

        let esc = color_escape(fg, bg, attrs);
        lua_pushlstring(interpreter, esc.as_ptr() as *const i8, esc.len());
        1
    }
}

// ─── tui.reset() → "\x1b[0m" ─────────────────────────────────────────────────

pub unsafe fn tui_reset(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        lua_pushlstring(interpreter, b"\x1b[0m".as_ptr() as *const i8, 4);
        1
    }
}

// ─── tui.enter_alt() / tui.exit_alt() — alternate screen buffer ──────────────

pub unsafe fn tui_enter_alt(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        write_stdout(b"\x1b[?1049h");
        let _ = interpreter;
        0
    }
}

pub unsafe fn tui_exit_alt(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        write_stdout(b"\x1b[?1049l");
        let _ = interpreter;
        0
    }
}

// ─── tui.raw() — enter raw (non-canonical) input mode ────────────────────────

pub unsafe fn tui_raw(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        if IN_RAW_MODE {
            return 0;
        }
        let mut t: libc::termios = std::mem::zeroed();
        if libc::tcgetattr(libc::STDIN_FILENO, &mut t) != 0 {
            return lual_error(interpreter, c"tui.raw: tcgetattr failed".as_ptr());
        }

        // Save original settings using raw-pointer write to avoid static_mut_refs
        std::ptr::write(&raw mut SAVED_TERMIOS, t);
        HAS_SAVED_TERMIOS = true;

        // Switch to raw mode:
        //   - ICANON: disable line buffering (read char-by-char)
        //   - ECHO:   disable echoing
        //   - IEXTEN: disable extended processing
        //   - IXON:   disable XON/XOFF flow control
        // ISIG (signal generation) is left enabled so Ctrl-C still works.
        t.c_lflag &= !(libc::ICANON | libc::ECHO | libc::IEXTEN);
        t.c_iflag &= !(libc::IXON);
        t.c_cc[libc::VMIN]  = 1;
        t.c_cc[libc::VTIME] = 0;

        libc::tcsetattr(libc::STDIN_FILENO, libc::TCSAFLUSH, &t);
        IN_RAW_MODE = true;
        0
    }
}

// ─── tui.cooked() — restore canonical input mode ─────────────────────────────

pub unsafe fn tui_cooked(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        if !IN_RAW_MODE {
            return 0;
        }
        if HAS_SAVED_TERMIOS {
            // Read saved termios by copy (no reference to static mut)
            let saved = std::ptr::read(&raw const SAVED_TERMIOS);
            libc::tcsetattr(libc::STDIN_FILENO, libc::TCSAFLUSH, &saved);
        }
        IN_RAW_MODE = false;
        let _ = interpreter;
        0
    }
}

// ─── tui.read_key([timeout_ms]) → string | nil ───────────────────────────────
// timeout_ms: milliseconds to wait (-1 = block, 0 = non-blocking).
// Returns a key name string or nil on timeout.
//
// Key name strings:
//   single character   — printable ASCII (e.g. "a", "A", "!")
//   "enter", "tab", "backspace", "esc"
//   "up", "down", "left", "right"
//   "home", "end", "ins", "del", "pgup", "pgdn"
//   "f1"–"f12"
//   "ctrl-a"–"ctrl-z", "ctrl-space"
//   "alt-X"            — Alt + character
//   "shift-tab"
//   multi-byte UTF-8   — returned as raw bytes

pub unsafe fn tui_read_key(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let timeout_ms: i32 = if lua_type(interpreter, 1) == Some(TagType::Numeric) {
            lua_tointegerx(interpreter, 1, null_mut()) as i32
        } else {
            -1  // block
        };

        match parse_key(timeout_ms) {
            None => {
                (*interpreter).push_nil();
                1
            }
            Some(key) => {
                lua_pushlstring(interpreter, key.as_ptr() as *const i8, key.len());
                1
            }
        }
    }
}

// ─── tui.bell() ───────────────────────────────────────────────────────────────

pub unsafe fn tui_bell(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        write_stdout(b"\x07");
        let _ = interpreter;
        0
    }
}

// ─── tui.init() — convenience: enter alt screen, hide cursor, raw mode ───────

pub unsafe fn tui_init(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        write_stdout(b"\x1b[?1049h"); // alternate screen
        write_stdout(b"\x1b[?25l");   // hide cursor
        write_stdout(b"\x1b[2J\x1b[H"); // clear
        tui_raw(interpreter);
        0
    }
}

// ─── tui.cleanup() — undo everything tui.init() did ─────────────────────────

pub unsafe fn tui_cleanup(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        tui_cooked(interpreter);
        write_stdout(b"\x1b[?25h");   // show cursor
        write_stdout(b"\x1b[?1049l"); // normal screen
        0
    }
}

// ─── tui.set_title(title) — set terminal window title ────────────────────────

pub unsafe fn tui_set_title(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let mut slen = 0usize;
        let sptr = lual_checklstring(interpreter, 1, &mut slen);
        let title = std::slice::from_raw_parts(sptr as *const u8, slen);
        let mut buf = b"\x1b]0;".to_vec();
        buf.extend_from_slice(title);
        buf.push(0x07); // BEL terminator
        write_stdout(&buf);
        0
    }
}

// ─── Registration ─────────────────────────────────────────────────────────────

pub const TUI_FUNCTIONS: [RegisteredFunction; 19] = [
    RegisteredFunction {
        registeredfunction_name: c"size".as_ptr(),
        registeredfunction_function: Some(tui_size as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"clear".as_ptr(),
        registeredfunction_function: Some(tui_clear as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"clear_line".as_ptr(),
        registeredfunction_function: Some(tui_clear_line as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"flush".as_ptr(),
        registeredfunction_function: Some(tui_flush as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"move".as_ptr(),
        registeredfunction_function: Some(tui_move as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"hide_cursor".as_ptr(),
        registeredfunction_function: Some(tui_hide_cursor as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"show_cursor".as_ptr(),
        registeredfunction_function: Some(tui_show_cursor as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"print".as_ptr(),
        registeredfunction_function: Some(tui_print as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"print_at".as_ptr(),
        registeredfunction_function: Some(tui_print_at as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"color".as_ptr(),
        registeredfunction_function: Some(tui_color as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"reset".as_ptr(),
        registeredfunction_function: Some(tui_reset as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"enter_alt".as_ptr(),
        registeredfunction_function: Some(tui_enter_alt as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"exit_alt".as_ptr(),
        registeredfunction_function: Some(tui_exit_alt as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"raw".as_ptr(),
        registeredfunction_function: Some(tui_raw as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"cooked".as_ptr(),
        registeredfunction_function: Some(tui_cooked as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"read_key".as_ptr(),
        registeredfunction_function: Some(tui_read_key as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"bell".as_ptr(),
        registeredfunction_function: Some(tui_bell as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"init".as_ptr(),
        registeredfunction_function: Some(tui_init as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"cleanup".as_ptr(),
        registeredfunction_function: Some(tui_cleanup as unsafe fn(*mut Interpreter) -> i32),
    },
];

// ─── luaopen_tui ─────────────────────────────────────────────────────────────

pub unsafe fn luaopen_tui(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        (*interpreter).lua_createtable();
        lual_setfuncs(interpreter, TUI_FUNCTIONS.as_ptr(), TUI_FUNCTIONS.len(), 0);

        // Also expose set_title (not in the const array due to count flexibility)
        lua_pushcclosure(
            interpreter,
            Some(tui_set_title as unsafe fn(*mut Interpreter) -> i32),
            0,
        );
        lua_setfield(interpreter, -2, c"set_title".as_ptr());

        // ── Colour constants ──────────────────────────────────────────────────
        macro_rules! set_int {
            ($name:expr, $val:expr) => {
                (*interpreter).push_integer($val);
                lua_setfield(interpreter, -2, $name);
            };
        }

        set_int!(c"BLACK".as_ptr(),         0);
        set_int!(c"RED".as_ptr(),           1);
        set_int!(c"GREEN".as_ptr(),         2);
        set_int!(c"YELLOW".as_ptr(),        3);
        set_int!(c"BLUE".as_ptr(),          4);
        set_int!(c"MAGENTA".as_ptr(),       5);
        set_int!(c"CYAN".as_ptr(),          6);
        set_int!(c"WHITE".as_ptr(),         7);
        set_int!(c"BRIGHT_BLACK".as_ptr(),  8);
        set_int!(c"BRIGHT_RED".as_ptr(),    9);
        set_int!(c"BRIGHT_GREEN".as_ptr(),  10);
        set_int!(c"BRIGHT_YELLOW".as_ptr(), 11);
        set_int!(c"BRIGHT_BLUE".as_ptr(),   12);
        set_int!(c"BRIGHT_MAGENTA".as_ptr(),13);
        set_int!(c"BRIGHT_CYAN".as_ptr(),   14);
        set_int!(c"BRIGHT_WHITE".as_ptr(),  15);

        // ── Attribute bitmask constants ───────────────────────────────────────
        set_int!(c"BOLD".as_ptr(),      1);
        set_int!(c"DIM".as_ptr(),       2);
        set_int!(c"ITALIC".as_ptr(),    4);
        set_int!(c"UNDERLINE".as_ptr(), 8);
        set_int!(c"BLINK".as_ptr(),     16);
        set_int!(c"REVERSE".as_ptr(),   32);

        1
    }
}
