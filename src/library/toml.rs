use crate::interpreter::*;
use crate::registeredfunction::*;
use crate::tagtype::*;

// ─── Pure-Rust TOML parser ────────────────────────────────────────────────────

struct Parser<'a> {
    src: &'a [u8],
    pos: usize,
    line: usize,
}

#[derive(Debug)]
enum TomlValue {
    String(Vec<u8>),
    Integer(i64),
    Float(f64),
    Bool(bool),
    Array(Vec<TomlValue>),
    Table(Vec<(Vec<u8>, TomlValue)>),
}

impl<'a> Parser<'a> {
    fn new(src: &'a [u8]) -> Self {
        Parser { src, pos: 0, line: 1 }
    }

    fn peek(&self) -> Option<u8> {
        self.src.get(self.pos).copied()
    }

    fn peek2(&self) -> Option<u8> {
        self.src.get(self.pos + 1).copied()
    }

    fn advance(&mut self) -> Option<u8> {
        let b = self.src.get(self.pos).copied()?;
        self.pos += 1;
        if b == b'\n' {
            self.line += 1;
        }
        Some(b)
    }

    fn skip_whitespace(&mut self) {
        while let Some(b) = self.peek() {
            if b == b' ' || b == b'\t' || b == b'\r' {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn skip_whitespace_and_newlines(&mut self) {
        while let Some(b) = self.peek() {
            if b == b' ' || b == b'\t' || b == b'\r' || b == b'\n' {
                self.advance();
            } else if b == b'#' {
                self.skip_comment();
            } else {
                break;
            }
        }
    }

    fn skip_comment(&mut self) {
        // skip everything to end of line
        while let Some(b) = self.peek() {
            self.advance();
            if b == b'\n' {
                break;
            }
        }
    }

    fn expect(&mut self, expected: u8) -> Result<(), String> {
        match self.advance() {
            Some(b) if b == expected => Ok(()),
            Some(b) => Err(format!("line {}: expected '{}' got '{}'", self.line, expected as char, b as char)),
            None => Err(format!("line {}: expected '{}' got EOF", self.line, expected as char)),
        }
    }

    // Parse a bare key or quoted key segment
    fn parse_key_segment(&mut self) -> Result<Vec<u8>, String> {
        match self.peek() {
            Some(b'"') => self.parse_basic_string(),
            Some(b'\'') => self.parse_literal_string(),
            Some(b) if is_bare_key_char(b) => {
                let mut key = Vec::new();
                while let Some(b) = self.peek() {
                    if is_bare_key_char(b) {
                        key.push(b);
                        self.advance();
                    } else {
                        break;
                    }
                }
                Ok(key)
            }
            Some(b) => Err(format!("line {}: invalid key character '{}'", self.line, b as char)),
            None => Err(format!("line {}: unexpected EOF in key", self.line)),
        }
    }

    // Parse a (possibly dotted) key: returns Vec of segments
    fn parse_key(&mut self) -> Result<Vec<Vec<u8>>, String> {
        let mut parts = vec![self.parse_key_segment()?];
        self.skip_whitespace();
        while self.peek() == Some(b'.') {
            self.advance(); // consume '.'
            self.skip_whitespace();
            parts.push(self.parse_key_segment()?);
            self.skip_whitespace();
        }
        Ok(parts)
    }

    fn parse_basic_string(&mut self) -> Result<Vec<u8>, String> {
        self.expect(b'"')?;
        // Check for multi-line """
        if self.peek() == Some(b'"') && self.peek2() == Some(b'"') {
            self.advance(); self.advance();
            // skip immediate newline after """
            if self.peek() == Some(b'\n') { self.advance(); }
            else if self.peek() == Some(b'\r') && self.peek2() == Some(b'\n') { self.advance(); self.advance(); }
            return self.parse_ml_basic_string();
        }
        let mut out = Vec::new();
        loop {
            match self.advance() {
                None => return Err(format!("line {}: unterminated string", self.line)),
                Some(b'"') => break,
                Some(b'\\') => out.extend(self.parse_escape()?),
                Some(b'\n') | Some(b'\r') => return Err(format!("line {}: newline in basic string", self.line)),
                Some(b) => out.push(b),
            }
        }
        Ok(out)
    }

    fn parse_ml_basic_string(&mut self) -> Result<Vec<u8>, String> {
        let mut out = Vec::new();
        loop {
            if self.peek() == Some(b'"') && self.src.get(self.pos+1).copied() == Some(b'"') && self.src.get(self.pos+2).copied() == Some(b'"') {
                self.advance(); self.advance(); self.advance();
                // up to 2 extra quotes
                while self.peek() == Some(b'"') { out.push(b'"'); self.advance(); if out.last() == Some(&b'"') && out.len() >= 2 { break; } }
                break;
            }
            match self.advance() {
                None => return Err(format!("line {}: unterminated multi-line string", self.line)),
                Some(b'\\') => {
                    // line-ending backslash: skip whitespace/newlines
                    if self.peek() == Some(b'\n') || self.peek() == Some(b'\r') || self.peek() == Some(b' ') || self.peek() == Some(b'\t') {
                        while matches!(self.peek(), Some(b' ') | Some(b'\t') | Some(b'\n') | Some(b'\r')) {
                            self.advance();
                        }
                    } else {
                        out.extend(self.parse_escape()?);
                    }
                }
                Some(b) => out.push(b),
            }
        }
        Ok(out)
    }

    fn parse_escape(&mut self) -> Result<Vec<u8>, String> {
        match self.advance() {
            Some(b'b') => Ok(vec![0x08]),
            Some(b't') => Ok(vec![b'\t']),
            Some(b'n') => Ok(vec![b'\n']),
            Some(b'f') => Ok(vec![0x0C]),
            Some(b'r') => Ok(vec![b'\r']),
            Some(b'"') => Ok(vec![b'"']),
            Some(b'\\') => Ok(vec![b'\\']),
            Some(b'u') => {
                let cp = self.parse_hex4()?;
                Ok(encode_utf8(cp as u32))
            }
            Some(b'U') => {
                let hi = self.parse_hex4()? as u32;
                let lo = self.parse_hex4()? as u32;
                Ok(encode_utf8((hi << 16) | lo))
            }
            Some(b) => Err(format!("line {}: invalid escape '\\{}'", self.line, b as char)),
            None => Err(format!("line {}: EOF in escape", self.line)),
        }
    }

    fn parse_hex4(&mut self) -> Result<u16, String> {
        let mut val: u32 = 0;
        for _ in 0..4 {
            match self.advance() {
                Some(b) if b.is_ascii_hexdigit() => {
                    val = val * 16 + hex_digit(b) as u32;
                }
                Some(b) => return Err(format!("line {}: invalid hex digit '{}'", self.line, b as char)),
                None => return Err(format!("line {}: EOF in unicode escape", self.line)),
            }
        }
        Ok(val as u16)
    }

    fn parse_literal_string(&mut self) -> Result<Vec<u8>, String> {
        self.expect(b'\'')?;
        // Check for multi-line '''
        if self.peek() == Some(b'\'') && self.peek2() == Some(b'\'') {
            self.advance(); self.advance();
            if self.peek() == Some(b'\n') { self.advance(); }
            return self.parse_ml_literal_string();
        }
        let mut out = Vec::new();
        loop {
            match self.advance() {
                None => return Err(format!("line {}: unterminated literal string", self.line)),
                Some(b'\'') => break,
                Some(b'\n') | Some(b'\r') => return Err(format!("line {}: newline in literal string", self.line)),
                Some(b) => out.push(b),
            }
        }
        Ok(out)
    }

    fn parse_ml_literal_string(&mut self) -> Result<Vec<u8>, String> {
        let mut out = Vec::new();
        loop {
            if self.peek() == Some(b'\'') && self.src.get(self.pos+1).copied() == Some(b'\'') && self.src.get(self.pos+2).copied() == Some(b'\'') {
                self.advance(); self.advance(); self.advance();
                break;
            }
            match self.advance() {
                None => return Err(format!("line {}: unterminated multi-line literal string", self.line)),
                Some(b) => out.push(b),
            }
        }
        Ok(out)
    }

    fn parse_number_or_date(&mut self) -> Result<TomlValue, String> {
        // Collect the number token
        let start = self.pos;
        // sign
        if matches!(self.peek(), Some(b'+') | Some(b'-')) { self.advance(); }
        // special float: inf, nan
        if self.peek() == Some(b'i') || self.peek() == Some(b'n') {
            while let Some(b) = self.peek() {
                if b.is_ascii_alphabetic() { self.advance(); } else { break; }
            }
            let tok = &self.src[start..self.pos];
            return match tok {
                b"inf" | b"+inf" => Ok(TomlValue::Float(f64::INFINITY)),
                b"-inf" => Ok(TomlValue::Float(f64::NEG_INFINITY)),
                b"nan" | b"+nan" | b"-nan" => Ok(TomlValue::Float(f64::NAN)),
                _ => Err(format!("line {}: invalid token '{}'", self.line, String::from_utf8_lossy(tok))),
            };
        }
        // hex/octal/binary integer
        if self.peek() == Some(b'0') {
            match self.peek2() {
                Some(b'x') => {
                    self.advance(); self.advance();
                    return self.parse_int_base(16);
                }
                Some(b'o') => {
                    self.advance(); self.advance();
                    return self.parse_int_base(8);
                }
                Some(b'b') => {
                    self.advance(); self.advance();
                    return self.parse_int_base(2);
                }
                _ => {}
            }
        }
        // collect digits, '.', 'e', 'E', '_', '-', ':', 'T', 'Z'
        while let Some(b) = self.peek() {
            if b.is_ascii_digit() || b == b'.' || b == b'e' || b == b'E'
                || b == b'_' || b == b'-' || b == b'+' || b == b':'
                || b == b'T' || b == b'Z' || b == b't' || b == b'z'
            {
                self.advance();
            } else {
                break;
            }
        }
        let tok = &self.src[start..self.pos];
        // Remove underscores for parsing
        let clean: Vec<u8> = tok.iter().copied().filter(|&b| b != b'_').collect();
        let s = String::from_utf8_lossy(&clean);
        // Try integer first
        if !clean.contains(&b'.') && !clean.contains(&b'e') && !clean.contains(&b'E') {
            if let Ok(i) = s.parse::<i64>() {
                return Ok(TomlValue::Integer(i));
            }
        }
        // Try float
        if let Ok(f) = s.parse::<f64>() {
            return Ok(TomlValue::Float(f));
        }
        // Datetime: just return as string
        Ok(TomlValue::String(tok.to_vec()))
    }

    fn parse_int_base(&mut self, base: u64) -> Result<TomlValue, String> {
        let mut val: i64 = 0;
        let mut had_digit = false;
        while let Some(b) = self.peek() {
            if b == b'_' { self.advance(); continue; }
            let digit = match base {
                16 if b.is_ascii_hexdigit() => hex_digit(b) as u64,
                8 if matches!(b, b'0'..=b'7') => (b - b'0') as u64,
                2 if matches!(b, b'0' | b'1') => (b - b'0') as u64,
                _ => break,
            };
            val = val * base as i64 + digit as i64;
            had_digit = true;
            self.advance();
        }
        if !had_digit {
            return Err(format!("line {}: empty integer literal", self.line));
        }
        Ok(TomlValue::Integer(val))
    }

    fn parse_value(&mut self) -> Result<TomlValue, String> {
        self.skip_whitespace();
        match self.peek() {
            Some(b'"') => Ok(TomlValue::String(self.parse_basic_string()?)),
            Some(b'\'') => Ok(TomlValue::String(self.parse_literal_string()?)),
            Some(b't') => {
                self.expect_bytes(b"true")?;
                Ok(TomlValue::Bool(true))
            }
            Some(b'f') => {
                self.expect_bytes(b"false")?;
                Ok(TomlValue::Bool(false))
            }
            Some(b'[') => self.parse_array(),
            Some(b'{') => self.parse_inline_table(),
            Some(b) if b.is_ascii_digit() || b == b'+' || b == b'-' || b == b'i' || b == b'n' => {
                self.parse_number_or_date()
            }
            Some(b) => Err(format!("line {}: unexpected character '{}' in value", self.line, b as char)),
            None => Err(format!("line {}: unexpected EOF in value", self.line)),
        }
    }

    fn expect_bytes(&mut self, expected: &[u8]) -> Result<(), String> {
        for &e in expected {
            self.expect(e)?;
        }
        Ok(())
    }

    fn parse_array(&mut self) -> Result<TomlValue, String> {
        self.expect(b'[')?;
        let mut items = Vec::new();
        loop {
            self.skip_whitespace_and_newlines();
            if self.peek() == Some(b']') {
                self.advance();
                break;
            }
            items.push(self.parse_value()?);
            self.skip_whitespace_and_newlines();
            match self.peek() {
                Some(b',') => { self.advance(); }
                Some(b']') => { self.advance(); break; }
                Some(b) => return Err(format!("line {}: expected ',' or ']' in array, got '{}'", self.line, b as char)),
                None => return Err(format!("line {}: EOF in array", self.line)),
            }
        }
        Ok(TomlValue::Array(items))
    }

    fn parse_inline_table(&mut self) -> Result<TomlValue, String> {
        self.expect(b'{')?;
        let mut pairs = Vec::new();
        self.skip_whitespace();
        if self.peek() == Some(b'}') {
            self.advance();
            return Ok(TomlValue::Table(pairs));
        }
        loop {
            self.skip_whitespace();
            let key_parts = self.parse_key()?;
            self.skip_whitespace();
            self.expect(b'=')?;
            self.skip_whitespace();
            let val = self.parse_value()?;
            // Flatten dotted key into nested tables
            let pair = build_dotted_value(key_parts, val);
            pairs.push(pair);
            self.skip_whitespace();
            match self.peek() {
                Some(b',') => { self.advance(); }
                Some(b'}') => { self.advance(); break; }
                Some(b) => return Err(format!("line {}: expected ',' or '}}' in inline table, got '{}'", self.line, b as char)),
                None => return Err(format!("line {}: EOF in inline table", self.line)),
            }
        }
        Ok(TomlValue::Table(pairs))
    }
}

fn build_dotted_value(mut parts: Vec<Vec<u8>>, val: TomlValue) -> (Vec<u8>, TomlValue) {
    if parts.len() == 1 {
        return (parts.remove(0), val);
    }
    let first = parts.remove(0);
    let inner = build_dotted_value(parts, val);
    (first, TomlValue::Table(vec![inner]))
}

fn is_bare_key_char(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'-' || b == b'_'
}

fn hex_digit(b: u8) -> u8 {
    match b {
        b'0'..=b'9' => b - b'0',
        b'a'..=b'f' => b - b'a' + 10,
        b'A'..=b'F' => b - b'A' + 10,
        _ => 0,
    }
}

fn encode_utf8(cp: u32) -> Vec<u8> {
    let mut buf = Vec::new();
    if cp < 0x80 {
        buf.push(cp as u8);
    } else if cp < 0x800 {
        buf.push(0xC0 | (cp >> 6) as u8);
        buf.push(0x80 | (cp & 0x3F) as u8);
    } else if cp < 0x10000 {
        buf.push(0xE0 | (cp >> 12) as u8);
        buf.push(0x80 | ((cp >> 6) & 0x3F) as u8);
        buf.push(0x80 | (cp & 0x3F) as u8);
    } else {
        buf.push(0xF0 | (cp >> 18) as u8);
        buf.push(0x80 | ((cp >> 12) & 0x3F) as u8);
        buf.push(0x80 | ((cp >> 6) & 0x3F) as u8);
        buf.push(0x80 | (cp & 0x3F) as u8);
    }
    buf
}

// ─── Document-level parser ────────────────────────────────────────────────────

struct Document {
    root: Vec<(Vec<u8>, TomlValue)>,
}

impl Document {
    fn parse(src: &[u8]) -> Result<Self, String> {
        let mut p = Parser::new(src);
        let mut root: Vec<(Vec<u8>, TomlValue)> = Vec::new();
        // current table path (for [header] sections)
        let mut current_path: Vec<Vec<u8>> = Vec::new();
        let mut in_array_table = false;

        p.skip_whitespace_and_newlines();
        while p.pos < p.src.len() {
            p.skip_whitespace();
            match p.peek() {
                Some(b'#') => { p.skip_comment(); }
                Some(b'\n') | Some(b'\r') => { p.advance(); }
                Some(b'[') => {
                    p.advance();
                    if p.peek() == Some(b'[') {
                        // Array of tables [[header]]
                        p.advance();
                        let key = p.parse_key()?;
                        p.skip_whitespace();
                        p.expect(b']')?;
                        p.expect(b']')?;
                        current_path = key;
                        in_array_table = true;
                        // Insert an empty table into the array at that path
                        insert_aot(&mut root, &current_path)?;
                    } else {
                        // Standard table [header]
                        let key = p.parse_key()?;
                        p.skip_whitespace();
                        p.expect(b']')?;
                        current_path = key;
                        in_array_table = false;
                        // Ensure path exists as a table
                        ensure_table(&mut root, &current_path)?;
                    }
                    p.skip_whitespace();
                    if p.peek() == Some(b'#') { p.skip_comment(); }
                    if let Some(b'\n') | Some(b'\r') = p.peek() { p.advance(); }
                }
                Some(_) => {
                    // key = value
                    let key_parts = p.parse_key()?;
                    p.skip_whitespace();
                    p.expect(b'=')?;
                    p.skip_whitespace();
                    let val = p.parse_value()?;
                    p.skip_whitespace();
                    if p.peek() == Some(b'#') { p.skip_comment(); }
                    if let Some(b'\n') | Some(b'\r') = p.peek() { p.advance(); }
                    // Build full path
                    let mut full_path = current_path.clone();
                    full_path.extend_from_slice(&key_parts);
                    if in_array_table {
                        // Insert into the last element of the AOT
                        insert_into_last_aot(&mut root, &current_path, key_parts, val)?;
                    } else {
                        insert_at(&mut root, &full_path, val)?;
                    }
                }
                None => break,
            }
        }
        Ok(Document { root })
    }
}

fn get_or_create_table<'a>(
    pairs: &'a mut Vec<(Vec<u8>, TomlValue)>,
    key: &[u8],
) -> &'a mut Vec<(Vec<u8>, TomlValue)> {
    // find existing
    for i in 0..pairs.len() {
        if pairs[i].0 == key {
            if let TomlValue::Table(_) = &pairs[i].1 {
                if let TomlValue::Table(ref mut t) = pairs[i].1 {
                    return t;
                }
            }
            // key exists but is not a table — will fail later
        }
    }
    pairs.push((key.to_vec(), TomlValue::Table(Vec::new())));
    let last = pairs.len() - 1;
    if let TomlValue::Table(ref mut t) = pairs[last].1 { t } else { unreachable!() }
}

fn ensure_table(root: &mut Vec<(Vec<u8>, TomlValue)>, path: &[Vec<u8>]) -> Result<(), String> {
    let mut cur = root;
    for seg in path {
        cur = get_or_create_table(cur, seg);
    }
    Ok(())
}

fn insert_aot(root: &mut Vec<(Vec<u8>, TomlValue)>, path: &[Vec<u8>]) -> Result<(), String> {
    let mut cur = root;
    for seg in &path[..path.len()-1] {
        cur = get_or_create_table(cur, seg);
    }
    let last = &path[path.len()-1];
    // Find existing array
    for pair in cur.iter_mut() {
        if &pair.0 == last {
            if let TomlValue::Array(ref mut arr) = pair.1 {
                arr.push(TomlValue::Table(Vec::new()));
                return Ok(());
            }
            return Err(format!("key '{}' is not an array of tables", String::from_utf8_lossy(last)));
        }
    }
    cur.push((last.clone(), TomlValue::Array(vec![TomlValue::Table(Vec::new())])));
    Ok(())
}

fn insert_into_last_aot(
    root: &mut Vec<(Vec<u8>, TomlValue)>,
    aot_path: &[Vec<u8>],
    key_parts: Vec<Vec<u8>>,
    val: TomlValue,
) -> Result<(), String> {
    // Navigate to the aot array
    let mut cur = root;
    for seg in &aot_path[..aot_path.len()-1] {
        cur = get_or_create_table(cur, seg);
    }
    let last_seg = &aot_path[aot_path.len()-1];
    for pair in cur.iter_mut() {
        if &pair.0 == last_seg {
            if let TomlValue::Array(ref mut arr) = pair.1 {
                if let Some(TomlValue::Table(ref mut t)) = arr.last_mut() {
                    let mut full = key_parts;
                    insert_at(t, &full, val)?;
                    return Ok(());
                }
            }
        }
    }
    Err(format!("array-of-tables '{}' not found", String::from_utf8_lossy(last_seg)))
}

fn insert_at(
    root: &mut Vec<(Vec<u8>, TomlValue)>,
    path: &[Vec<u8>],
    val: TomlValue,
) -> Result<(), String> {
    if path.is_empty() {
        return Err("empty key path".to_string());
    }
    if path.len() == 1 {
        let key = &path[0];
        for pair in root.iter() {
            if &pair.0 == key {
                return Err(format!("duplicate key '{}'", String::from_utf8_lossy(key)));
            }
        }
        root.push((key.clone(), val));
        return Ok(());
    }
    let cur = get_or_create_table(root, &path[0]);
    insert_at(cur, &path[1..], val)
}

// ─── Push a TomlValue onto the Lua stack ─────────────────────────────────────

unsafe fn push_toml_value(interpreter: *mut Interpreter, val: &TomlValue) {
    unsafe {
        match val {
            TomlValue::String(s) => {
                lua_pushlstring(interpreter, s.as_ptr() as *const i8, s.len());
            }
            TomlValue::Integer(i) => {
                (*interpreter).push_integer(*i);
            }
            TomlValue::Float(f) => {
                (*interpreter).push_number(*f);
            }
            TomlValue::Bool(b) => {
                (*interpreter).push_boolean(*b);
            }
            TomlValue::Array(items) => {
                (*interpreter).lua_createtable();
                for (i, item) in items.iter().enumerate() {
                    push_toml_value(interpreter, item);
                    lua_rawseti(interpreter, -2, (i + 1) as i64);
                }
            }
            TomlValue::Table(pairs) => {
                (*interpreter).lua_createtable();
                for (k, v) in pairs {
                    push_toml_value(interpreter, v);
                    lua_setfield(interpreter, -2, k.as_ptr() as *const i8);
                    // Note: lua_setfield expects a null-terminated key.
                    // Since bare keys are ASCII alphanumeric we add the null below.
                }
            }
        }
    }
}

// lua_setfield with a counted key (not necessarily null-terminated)
unsafe fn setfield_bytes(interpreter: *mut Interpreter, table_idx: i32, key: &[u8]) {
    unsafe {
        // Push key as Lua string, then rawset
        lua_pushlstring(interpreter, key.as_ptr() as *const i8, key.len());
        // stack: ..., table, value, key  → rotate so table is at -3
        lua_rotate(interpreter, -2, 1); // stack: ..., table, key, value
        lua_rawset(interpreter, table_idx - 1); // adjusting index since we rotated
    }
}

unsafe fn push_toml_value_safe(interpreter: *mut Interpreter, val: &TomlValue) {
    unsafe {
        match val {
            TomlValue::String(s) => {
                lua_pushlstring(interpreter, s.as_ptr() as *const i8, s.len());
            }
            TomlValue::Integer(i) => {
                (*interpreter).push_integer(*i);
            }
            TomlValue::Float(f) => {
                (*interpreter).push_number(*f);
            }
            TomlValue::Bool(b) => {
                (*interpreter).push_boolean(*b);
            }
            TomlValue::Array(items) => {
                (*interpreter).lua_createtable();
                for (i, item) in items.iter().enumerate() {
                    push_toml_value_safe(interpreter, item);
                    lua_rawseti(interpreter, -2, (i + 1) as i64);
                }
            }
            TomlValue::Table(pairs) => {
                (*interpreter).lua_createtable();
                let tbl_idx = lua_absindex(interpreter, -1);
                for (k, v) in pairs {
                    // push key
                    lua_pushlstring(interpreter, k.as_ptr() as *const i8, k.len());
                    // push value
                    push_toml_value_safe(interpreter, v);
                    // rawset table[key] = value
                    lua_rawset(interpreter, tbl_idx);
                }
            }
        }
    }
}

// ─── toml.parse ──────────────────────────────────────────────────────────────

pub unsafe fn toml_parse(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let mut len: usize = 0;
        let ptr = lua_tolstring(interpreter, 1, &mut len);
        if ptr.is_null() {
            return lual_error(interpreter, c"toml.parse: expected string argument".as_ptr());
        }
        let src = std::slice::from_raw_parts(ptr as *const u8, len);
        match Document::parse(src) {
            Err(e) => {
                (*interpreter).push_nil();
                lua_pushlstring(interpreter, e.as_ptr() as *const i8, e.len());
                2
            }
            Ok(doc) => {
                push_toml_value_safe(interpreter, &TomlValue::Table(doc.root));
                1
            }
        }
    }
}

// ─── TOML serializer ─────────────────────────────────────────────────────────

unsafe fn lua_to_toml(
    interpreter: *mut Interpreter,
    idx: i32,
    out: &mut Vec<u8>,
    path: &str,
    depth: usize,
) -> Result<(), String> {
    unsafe {
        let idx = lua_absindex(interpreter, idx);
        let tt = lua_type(interpreter, idx);
        match tt {
            Some(TagType::Boolean) => {
                if lua_toboolean(interpreter, idx) {
                    out.extend_from_slice(b"true");
                } else {
                    out.extend_from_slice(b"false");
                }
                Ok(())
            }
            Some(TagType::Numeric) => {
                if lua_isinteger(interpreter, idx) {
                    let i = lua_tointegerx(interpreter, idx, std::ptr::null_mut());
                    let s = i.to_string();
                    out.extend_from_slice(s.as_bytes());
                } else {
                    let f = lua_tonumberx(interpreter, idx, std::ptr::null_mut());
                    if f.is_nan() {
                        out.extend_from_slice(b"nan");
                    } else if f.is_infinite() {
                        if f > 0.0 { out.extend_from_slice(b"inf"); } else { out.extend_from_slice(b"-inf"); }
                    } else {
                        // Use Rust's float formatter; ensure there's a decimal point
                        let s = format!("{}", f);
                        out.extend_from_slice(s.as_bytes());
                        if !s.contains('.') && !s.contains('e') {
                            out.extend_from_slice(b".0");
                        }
                    }
                }
                Ok(())
            }
            Some(TagType::String) => {
                let mut slen: usize = 0;
                let sptr = lua_tolstring(interpreter, idx, &mut slen);
                let s = std::slice::from_raw_parts(sptr as *const u8, slen);
                out.push(b'"');
                for &b in s {
                    match b {
                        b'"' => out.extend_from_slice(b"\\\""),
                        b'\\' => out.extend_from_slice(b"\\\\"),
                        b'\n' => out.extend_from_slice(b"\\n"),
                        b'\r' => out.extend_from_slice(b"\\r"),
                        b'\t' => out.extend_from_slice(b"\\t"),
                        0x00..=0x1F | 0x7F => {
                            let esc = format!("\\u{:04X}", b);
                            out.extend_from_slice(esc.as_bytes());
                        }
                        _ => out.push(b),
                    }
                }
                out.push(b'"');
                Ok(())
            }
            Some(TagType::Table) => {
                // Determine if it's an array or a map
                if is_lua_array(interpreter, idx) {
                    out.push(b'[');
                    let n = get_length_raw(interpreter, idx);
                    for i in 1..=n {
                        if i > 1 { out.extend_from_slice(b", "); }
                        lua_rawgeti(interpreter, idx, i as i64);
                        lua_to_toml(interpreter, -1, out, path, depth + 1)?;
                        lua_settop(interpreter, -2);
                    }
                    out.push(b']');
                } else {
                    // Inline table for nested/inline contexts — use inline at depth>0
                    if depth > 0 {
                        out.push(b'{');
                        let mut first = true;
                        lua_pushnil(interpreter);
                        while lua_next(interpreter, idx) != 0 {
                            if !first { out.extend_from_slice(b", "); }
                            first = false;
                            // key
                            let mut klen: usize = 0;
                            let kptr = lua_tolstring(interpreter, -2, &mut klen);
                            if kptr.is_null() {
                                lua_settop(interpreter, -3);
                                return Err("toml.stringify: table keys must be strings".to_string());
                            }
                            let kslice = std::slice::from_raw_parts(kptr as *const u8, klen);
                            write_toml_key(out, kslice);
                            out.extend_from_slice(b" = ");
                            lua_to_toml(interpreter, -1, out, path, depth + 1)?;
                            lua_settop(interpreter, -2); // pop value, keep key for next
                        }
                        out.push(b'}');
                    } else {
                        // top-level: emit [section] style — handled by emit_table
                        emit_table(interpreter, idx, out, path)?;
                    }
                }
                Ok(())
            }
            _ => Err(format!("toml.stringify: cannot serialize value of type {:?} at '{}'", tt.map(|t| t as u8), path)),
        }
    }
}

unsafe fn is_lua_array(interpreter: *mut Interpreter, idx: i32) -> bool {
    unsafe {
        let n = get_length_raw(interpreter, idx);
        if n == 0 {
            // Check if there are any non-integer keys
            lua_pushnil(interpreter);
            if lua_next(interpreter, idx) != 0 {
                lua_settop(interpreter, -3); // pop key and value
                return false;
            }
            return true; // empty table — treat as array
        }
        // Check all keys are 1..n integers
        lua_pushnil(interpreter);
        let mut count = 0usize;
        while lua_next(interpreter, idx) != 0 {
            count += 1;
            let mut is_int = false;
            let ki = lua_tointegerx(interpreter, -2, &mut is_int);
            if !is_int || ki < 1 || ki as usize > n {
                lua_settop(interpreter, -3);
                return false;
            }
            lua_settop(interpreter, -2);
        }
        count == n
    }
}

fn write_toml_key(out: &mut Vec<u8>, key: &[u8]) {
    // bare key if all alphanumeric/dash/underscore
    if key.iter().all(|&b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_') && !key.is_empty() {
        out.extend_from_slice(key);
    } else {
        out.push(b'"');
        for &b in key {
            match b {
                b'"' => out.extend_from_slice(b"\\\""),
                b'\\' => out.extend_from_slice(b"\\\\"),
                _ => out.push(b),
            }
        }
        out.push(b'"');
    }
}

unsafe fn lua_pushnil(interpreter: *mut Interpreter) {
    unsafe { (*interpreter).push_nil(); }
}

unsafe fn emit_table(
    interpreter: *mut Interpreter,
    idx: i32,
    out: &mut Vec<u8>,
    prefix: &str,
) -> Result<(), String> {
    unsafe {
        // Two passes: first scalar values, then sub-tables/arrays
        let idx = lua_absindex(interpreter, idx);

        // Pass 1: scalars
        lua_pushnil(interpreter);
        while lua_next(interpreter, idx) != 0 {
            let vtype = lua_type(interpreter, -1);
            let is_table = matches!(vtype, Some(TagType::Table));
            let is_array_of_tables = is_table && {
                let sub_idx = lua_absindex(interpreter, -1);
                let is_arr = is_lua_array(interpreter, sub_idx);
                if is_arr && get_length_raw(interpreter, sub_idx) > 0 {
                    lua_rawgeti(interpreter, sub_idx, 1);
                    let first_is_table = matches!(lua_type(interpreter, -1), Some(TagType::Table));
                    lua_settop(interpreter, -2);
                    first_is_table
                } else {
                    false
                }
            };

            if !is_table || is_array_of_tables {
                if !is_array_of_tables {
                    // scalar or plain array: emit key = value
                    let mut klen: usize = 0;
                    let kptr = lua_tolstring(interpreter, -2, &mut klen);
                    if kptr.is_null() {
                        lua_settop(interpreter, -3);
                        return Err("toml.stringify: table keys must be strings".to_string());
                    }
                    let kslice = std::slice::from_raw_parts(kptr as *const u8, klen);
                    write_toml_key(out, kslice);
                    out.extend_from_slice(b" = ");
                    lua_to_toml(interpreter, -1, out, prefix, 1)?;
                    out.push(b'\n');
                }
            }
            lua_settop(interpreter, -2);
        }

        // Pass 2: sub-tables and arrays-of-tables
        lua_pushnil(interpreter);
        while lua_next(interpreter, idx) != 0 {
            let vtype = lua_type(interpreter, -1);
            let sub_idx = lua_absindex(interpreter, -1);

            if matches!(vtype, Some(TagType::Table)) {
                let mut klen: usize = 0;
                let kptr = lua_tolstring(interpreter, -2, &mut klen);
                if kptr.is_null() {
                    lua_settop(interpreter, -3);
                    return Err("toml.stringify: table keys must be strings".to_string());
                }
                let kslice = std::slice::from_raw_parts(kptr as *const u8, klen);
                let key_str = String::from_utf8_lossy(kslice).into_owned();

                let child_path = if prefix.is_empty() {
                    key_str.clone()
                } else {
                    format!("{}.{}", prefix, key_str)
                };

                let is_arr = is_lua_array(interpreter, sub_idx);
                if is_arr && get_length_raw(interpreter, sub_idx) > 0 {
                    lua_rawgeti(interpreter, sub_idx, 1);
                    let first_is_table = matches!(lua_type(interpreter, -1), Some(TagType::Table));
                    lua_settop(interpreter, -2);
                    if first_is_table {
                        // Array of tables
                        let n = get_length_raw(interpreter, sub_idx);
                        for i in 1..=n {
                            out.extend_from_slice(b"\n[[");
                            out.extend_from_slice(child_path.as_bytes());
                            out.extend_from_slice(b"]]\n");
                            lua_rawgeti(interpreter, sub_idx, i as i64);
                            let elem_idx = lua_absindex(interpreter, -1);
                            emit_table(interpreter, elem_idx, out, &child_path)?;
                            lua_settop(interpreter, -2);
                        }
                        lua_settop(interpreter, -2);
                        continue;
                    }
                }

                // Regular sub-table
                out.extend_from_slice(b"\n[");
                out.extend_from_slice(child_path.as_bytes());
                out.extend_from_slice(b"]\n");
                emit_table(interpreter, sub_idx, out, &child_path)?;
            }
            lua_settop(interpreter, -2);
        }
        Ok(())
    }
}

pub unsafe fn toml_stringify(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        if !matches!(lua_type(interpreter, 1), Some(TagType::Table)) {
            return lual_error(interpreter, c"toml.stringify: expected table argument".as_ptr());
        }
        let mut out: Vec<u8> = Vec::new();
        match emit_table(interpreter, 1, &mut out, "") {
            Err(e) => {
                (*interpreter).push_nil();
                lua_pushlstring(interpreter, e.as_ptr() as *const i8, e.len());
                2
            }
            Ok(()) => {
                lua_pushlstring(interpreter, out.as_ptr() as *const i8, out.len());
                1
            }
        }
    }
}

// ─── Library registration ─────────────────────────────────────────────────────

pub const TOML_FUNCTIONS: [RegisteredFunction; 2] = [
    RegisteredFunction {
        registeredfunction_name: c"parse".as_ptr(),
        registeredfunction_function: Some(toml_parse as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"stringify".as_ptr(),
        registeredfunction_function: Some(toml_stringify as unsafe fn(*mut Interpreter) -> i32),
    },
];

pub unsafe fn luaopen_toml(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        (*interpreter).lua_createtable();
        lual_setfuncs(interpreter, TOML_FUNCTIONS.as_ptr(), TOML_FUNCTIONS.len(), 0);
        1
    }
}
