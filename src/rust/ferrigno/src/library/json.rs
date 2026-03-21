use crate::registeredfunction::*;
use crate::state::*;
use crate::tagtype::*;
use std::ptr::*;

// ─── json.null sentinel ───────────────────────────────────────────────────────

// A static whose address serves as the unique json.null light-userdata value.
static JSON_NULL_SENTINEL: u8 = 0;

pub fn json_null_ptr() -> *mut std::ffi::c_void {
    &JSON_NULL_SENTINEL as *const u8 as *mut std::ffi::c_void
}

// ─── JSON decoder ─────────────────────────────────────────────────────────────

pub struct JsonParser<'a> {
    pub input: &'a [u8],
    pub pos: usize,
}

impl<'a> JsonParser<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        JsonParser { input, pos: 0 }
    }

    fn peek(&self) -> Option<u8> {
        self.input.get(self.pos).copied()
    }

    fn advance(&mut self) -> Option<u8> {
        let b = self.input.get(self.pos).copied();
        if b.is_some() {
            self.pos += 1;
        }
        b
    }

    pub fn skip_ws(&mut self) {
        while matches!(self.peek(), Some(b' ' | b'\t' | b'\n' | b'\r')) {
            self.pos += 1;
        }
    }

    fn expect_byte(&mut self, expected: u8) -> Result<(), String> {
        match self.advance() {
            Some(b) if b == expected => Ok(()),
            Some(b) => Err(format!(
                "expected '{}', got '{}'",
                expected as char, b as char
            )),
            None => Err(format!("expected '{}', got EOF", expected as char)),
        }
    }

    fn expect_literal(&mut self, lit: &[u8]) -> Result<(), String> {
        for &e in lit {
            match self.advance() {
                Some(b) if b == e => {}
                Some(b) => return Err(format!("expected '{}', got '{}'", e as char, b as char)),
                None => return Err("unexpected EOF".to_string()),
            }
        }
        Ok(())
    }

    // Parse a JSON value and push it onto the Lua stack.
    pub unsafe fn parse_value(&mut self, state: *mut State) -> Result<(), String> {
        unsafe {
            self.skip_ws();
            match self.peek() {
                Some(b'n') => {
                    self.expect_literal(b"null")?;
                    lua_pushlightuserdata(state, json_null_ptr());
                    Ok(())
                }
                Some(b't') => {
                    self.expect_literal(b"true")?;
                    (*state).push_boolean(true);
                    Ok(())
                }
                Some(b'f') => {
                    self.expect_literal(b"false")?;
                    (*state).push_boolean(false);
                    Ok(())
                }
                Some(b'"') => {
                    let s = self.parse_string()?;
                    lua_pushlstring(state, s.as_ptr() as *const i8, s.len());
                    Ok(())
                }
                Some(b'[') => self.parse_array(state),
                Some(b'{') => self.parse_object(state),
                Some(b'-') | Some(b'0'..=b'9') => self.parse_number(state),
                Some(b) => Err(format!("unexpected character '{}'", b as char)),
                None => Err("unexpected end of input".to_string()),
            }
        }
    }

    fn parse_string(&mut self) -> Result<Vec<u8>, String> {
        self.expect_byte(b'"')?;
        let mut out = Vec::new();
        loop {
            match self.advance() {
                None => return Err("unterminated string".to_string()),
                Some(b'"') => break,
                Some(b'\\') => match self.advance() {
                    Some(b'"') => out.push(b'"'),
                    Some(b'\\') => out.push(b'\\'),
                    Some(b'/') => out.push(b'/'),
                    Some(b'n') => out.push(b'\n'),
                    Some(b'r') => out.push(b'\r'),
                    Some(b't') => out.push(b'\t'),
                    Some(b'b') => out.push(0x08),
                    Some(b'f') => out.push(0x0C),
                    Some(b'u') => {
                        let cp = self.parse_hex4()?;
                        // Handle UTF-16 surrogate pairs
                        let codepoint = if (0xD800..=0xDBFF).contains(&cp) {
                            if self.peek() == Some(b'\\') {
                                self.advance();
                                if self.advance() != Some(b'u') {
                                    return Err("expected \\u for low surrogate".to_string());
                                }
                                let low = self.parse_hex4()?;
                                if !(0xDC00..=0xDFFF).contains(&low) {
                                    return Err("invalid low surrogate".to_string());
                                }
                                0x10000u32 + ((cp as u32 - 0xD800) << 10) + (low as u32 - 0xDC00)
                            } else {
                                return Err("expected low surrogate".to_string());
                            }
                        } else {
                            cp as u32
                        };
                        push_utf8(codepoint, &mut out);
                    }
                    Some(b) => return Err(format!("invalid escape '\\{}'", b as char)),
                    None => return Err("EOF in string escape".to_string()),
                },
                Some(b) => out.push(b),
            }
        }
        Ok(out)
    }

    fn parse_hex4(&mut self) -> Result<u16, String> {
        let mut n: u16 = 0;
        for _ in 0..4 {
            let b = self.advance().ok_or("EOF in \\uXXXX escape")?;
            let digit = match b {
                b'0'..=b'9' => b - b'0',
                b'a'..=b'f' => b - b'a' + 10,
                b'A'..=b'F' => b - b'A' + 10,
                _ => return Err(format!("invalid hex digit '{}'", b as char)),
            };
            n = n * 16 + digit as u16;
        }
        Ok(n)
    }

    unsafe fn parse_number(&mut self, state: *mut State) -> Result<(), String> {
        unsafe {
            let start = self.pos;
            let mut is_float = false;

            if self.peek() == Some(b'-') {
                self.pos += 1;
            }

            match self.peek() {
                Some(b'0') => {
                    self.pos += 1;
                }
                Some(b'1'..=b'9') => {
                    while matches!(self.peek(), Some(b'0'..=b'9')) {
                        self.pos += 1;
                    }
                }
                _ => return Err("expected digit".to_string()),
            }

            if self.peek() == Some(b'.') {
                is_float = true;
                self.pos += 1;
                if !matches!(self.peek(), Some(b'0'..=b'9')) {
                    return Err("expected digit after '.'".to_string());
                }
                while matches!(self.peek(), Some(b'0'..=b'9')) {
                    self.pos += 1;
                }
            }

            if matches!(self.peek(), Some(b'e' | b'E')) {
                is_float = true;
                self.pos += 1;
                if matches!(self.peek(), Some(b'+' | b'-')) {
                    self.pos += 1;
                }
                if !matches!(self.peek(), Some(b'0'..=b'9')) {
                    return Err("expected digit in exponent".to_string());
                }
                while matches!(self.peek(), Some(b'0'..=b'9')) {
                    self.pos += 1;
                }
            }

            let s = std::str::from_utf8(&self.input[start..self.pos]).map_err(|_| "invalid UTF-8 in number".to_string())?;

            if is_float {
                let v: f64 = s.parse().map_err(|_| format!("invalid number: {}", s))?;
                (*state).push_number(v);
            } else if let Ok(v) = s.parse::<i64>() {
                (*state).push_integer(v);
            } else {
                // Too large for i64 — fall back to float
                let v: f64 = s.parse().map_err(|_| format!("invalid number: {}", s))?;
                (*state).push_number(v);
            }

            Ok(())
        }
    }

    unsafe fn parse_array(&mut self, state: *mut State) -> Result<(), String> {
        unsafe {
            self.expect_byte(b'[')?;
            (*state).lua_createtable();
            let mut idx = 1i64;

            self.skip_ws();
            if self.peek() == Some(b']') {
                self.pos += 1;
                return Ok(());
            }

            loop {
                self.skip_ws();
                (*state).push_integer(idx); // key
                self.parse_value(state)?; // value
                lua_rawset(state, -3);
                idx += 1;

                self.skip_ws();
                match self.advance() {
                    Some(b',') => {}
                    Some(b']') => break,
                    Some(b) => return Err(format!("expected ',' or ']', got '{}'", b as char)),
                    None => return Err("unexpected EOF in array".to_string()),
                }
            }
            Ok(())
        }
    }

    unsafe fn parse_object(&mut self, state: *mut State) -> Result<(), String> {
        unsafe {
            self.expect_byte(b'{')?;
            (*state).lua_createtable();

            self.skip_ws();
            if self.peek() == Some(b'}') {
                self.pos += 1;
                return Ok(());
            }

            loop {
                self.skip_ws();
                if self.peek() != Some(b'"') {
                    return Err("expected string key in object".to_string());
                }
                let key = self.parse_string()?;
                lua_pushlstring(state, key.as_ptr() as *const i8, key.len());

                self.skip_ws();
                self.expect_byte(b':')?;
                self.skip_ws();
                self.parse_value(state)?;

                lua_rawset(state, -3);

                self.skip_ws();
                match self.advance() {
                    Some(b',') => {}
                    Some(b'}') => break,
                    Some(b) => return Err(format!("expected ',' or '}}', got '{}'", b as char)),
                    None => return Err("unexpected EOF in object".to_string()),
                }
            }
            Ok(())
        }
    }
}

fn push_utf8(cp: u32, out: &mut Vec<u8>) {
    if cp <= 0x7F {
        out.push(cp as u8);
    } else if cp <= 0x7FF {
        out.push(0xC0 | (cp >> 6) as u8);
        out.push(0x80 | (cp & 0x3F) as u8);
    } else if cp <= 0xFFFF {
        out.push(0xE0 | (cp >> 12) as u8);
        out.push(0x80 | ((cp >> 6) & 0x3F) as u8);
        out.push(0x80 | (cp & 0x3F) as u8);
    } else {
        out.push(0xF0 | (cp >> 18) as u8);
        out.push(0x80 | ((cp >> 12) & 0x3F) as u8);
        out.push(0x80 | ((cp >> 6) & 0x3F) as u8);
        out.push(0x80 | (cp & 0x3F) as u8);
    }
}

// ─── JSON encoder ─────────────────────────────────────────────────────────────

fn encode_json_string(s: &[u8], buf: &mut Vec<u8>) {
    buf.push(b'"');
    for &b in s {
        match b {
            b'"' => buf.extend_from_slice(b"\\\""),
            b'\\' => buf.extend_from_slice(b"\\\\"),
            b'\n' => buf.extend_from_slice(b"\\n"),
            b'\r' => buf.extend_from_slice(b"\\r"),
            b'\t' => buf.extend_from_slice(b"\\t"),
            0x08 => buf.extend_from_slice(b"\\b"),
            0x0C => buf.extend_from_slice(b"\\f"),
            0x00..=0x1F => {
                let s = format!("\\u{:04x}", b);
                buf.extend_from_slice(s.as_bytes());
            }
            _ => buf.push(b),
        }
    }
    buf.push(b'"');
}

unsafe fn encode_value(state: *mut State, idx: i32, buf: &mut Vec<u8>, depth: usize) -> Result<(), String> {
    unsafe {
        if depth > 200 {
            return Err("maximum nesting depth exceeded".to_string());
        }

        // Normalize negative index to absolute before any stack changes
        let abs = if idx < 0 {
            (*state).get_top() + idx + 1
        } else {
            idx
        };

        match lua_type(state, abs) {
            None | Some(TagType::Nil) => {
                buf.extend_from_slice(b"null");
            }
            Some(TagType::Pointer) => {
                // json.null light-userdata
                buf.extend_from_slice(b"null");
            }
            Some(TagType::Boolean) => {
                if lua_toboolean(state, abs) {
                    buf.extend_from_slice(b"true");
                } else {
                    buf.extend_from_slice(b"false");
                }
            }
            Some(TagType::Numeric) => {
                if lua_isinteger(state, abs) {
                    let v = lua_tointegerx(state, abs, null_mut());
                    let s = format!("{}", v);
                    buf.extend_from_slice(s.as_bytes());
                } else {
                    let v = lua_tonumberx(state, abs, null_mut());
                    if !v.is_finite() {
                        return Err("cannot encode Inf or NaN as JSON".to_string());
                    }
                    // Produce a JSON-safe float representation
                    let s = format!("{}", v);
                    buf.extend_from_slice(s.as_bytes());
                }
            }
            Some(TagType::String) => {
                let mut slen = 0usize;
                let sptr = lua_tolstring(state, abs, &mut slen);
                let s = std::slice::from_raw_parts(sptr as *const u8, slen);
                encode_json_string(s, buf);
            }
            Some(TagType::Table) => {
                encode_table(state, abs, buf, depth)?;
            }
            Some(other) => {
                return Err(format!("cannot encode {:?} as JSON", other as u8));
            }
        }
        Ok(())
    }
}

unsafe fn encode_table(state: *mut State, abs: i32, buf: &mut Vec<u8>, depth: usize) -> Result<(), String> {
    unsafe {
        // Use #t to decide array vs object.
        lua_len(state, abs); // pushes length
        let len = lua_tointegerx(state, -1, null_mut());
        lua_settop(state, -2); // pop length

        if len > 0 {
            // Encode as JSON array using elements t[1]..t[len]
            buf.push(b'[');
            for i in 1..=len {
                if i > 1 {
                    buf.push(b',');
                }
                lua_rawgeti(state, abs, i);
                let result = encode_value(state, -1, buf, depth + 1);
                lua_settop(state, -2); // pop element
                result?;
            }
            buf.push(b']');
        } else {
            // Encode as JSON object
            buf.push(b'{');
            let mut first = true;
            (*state).push_nil(); // initial key for lua_next
            while lua_next(state, abs) != 0 {
                // key at -2, value at -1
                if !first {
                    buf.push(b',');
                }
                first = false;

                // Encode key as a JSON string
                let key_ty = lua_type(state, -2);
                match key_ty {
                    Some(TagType::String) => {
                        let mut klen = 0usize;
                        let kptr = lua_tolstring(state, -2, &mut klen);
                        let k = std::slice::from_raw_parts(kptr as *const u8, klen);
                        encode_json_string(k, buf);
                    }
                    Some(TagType::Numeric) => {
                        if lua_isinteger(state, -2) {
                            let v = lua_tointegerx(state, -2, null_mut());
                            let s = format!("\"{}\"", v);
                            buf.extend_from_slice(s.as_bytes());
                        } else {
                            let v = lua_tonumberx(state, -2, null_mut());
                            let s = format!("\"{}\"", v);
                            buf.extend_from_slice(s.as_bytes());
                        }
                    }
                    _ => {
                        // Skip non-string/number keys
                        lua_settop(state, -2); // pop value
                        continue;
                    }
                }

                buf.push(b':');

                // Encode value
                let result = encode_value(state, -1, buf, depth + 1);
                lua_settop(state, -2); // pop value, keep key for lua_next
                result?;
            }
            buf.push(b'}');
        }
        Ok(())
    }
}

// ─── Lua-callable functions ───────────────────────────────────────────────────

/// json.decode(str) → Lua value
pub unsafe fn json_decode(state: *mut State) -> i32 {
    unsafe {
        let mut slen = 0usize;
        let sptr = lual_checklstring(state, 1, &mut slen);
        let input = std::slice::from_raw_parts(sptr as *const u8, slen);

        let saved_top = (*state).get_top();
        let mut parser = JsonParser::new(input);

        match parser.parse_value(state) {
            Ok(()) => {
                parser.skip_ws();
                if parser.pos < parser.input.len() {
                    lua_settop(state, saved_top);
                    return lual_error(
                        state,
                        c"json.decode: trailing garbage after JSON value".as_ptr(),
                        &[],
                    );
                }
                1
            }
            Err(msg) => {
                lua_settop(state, saved_top);
                let full = format!("json.decode: {}\0", msg);
                lual_error(state, full.as_ptr() as *const i8, &[])
            }
        }
    }
}

/// json.encode(value) → string
pub unsafe fn json_encode(state: *mut State) -> i32 {
    unsafe {
        let mut buf = Vec::new();
        match encode_value(state, 1, &mut buf, 0) {
            Ok(()) => {
                lua_pushlstring(state, buf.as_ptr() as *const i8, buf.len());
                1
            }
            Err(msg) => {
                let full = format!("json.encode: {}\0", msg);
                lual_error(state, full.as_ptr() as *const i8, &[])
            }
        }
    }
}

// ─── Registration ─────────────────────────────────────────────────────────────

pub const JSON_FUNCTIONS: [RegisteredFunction; 2] = [
    RegisteredFunction {
        registeredfunction_name: c"decode".as_ptr(),
        registeredfunction_function: Some(json_decode as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"encode".as_ptr(),
        registeredfunction_function: Some(json_encode as unsafe fn(*mut State) -> i32),
    },
];

pub unsafe fn luaopen_json(state: *mut State) -> i32 {
    unsafe {
        (*state).lua_createtable();
        lual_setfuncs(state, JSON_FUNCTIONS.as_ptr(), JSON_FUNCTIONS.len(), 0);

        // json.null — a unique light userdata sentinel
        lua_pushlightuserdata(state, json_null_ptr());
        lua_setfield(state, -2, c"null".as_ptr());

        1
    }
}
