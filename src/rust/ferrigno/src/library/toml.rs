use crate::registeredfunction::*;
use crate::state::*;
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
        Parser {
            src,
            pos: 0,
            line: 1,
        }
    }

    fn peek(&self) -> Option<u8> {
        self.src.get(self.pos).copied()
    }

    fn peek2(&self) -> Option<u8> {
        self.src.get(self.pos + 1).copied()
    }

    fn peek3(&self) -> Option<u8> {
        self.src.get(self.pos + 2).copied()
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
        loop {
            match self.peek() {
                Some(b' ') | Some(b'\t') | Some(b'\r') | Some(b'\n') => {
                    self.advance();
                }
                Some(b'#') => self.skip_comment(),
                _ => break,
            }
        }
    }

    fn skip_comment(&mut self) {
        while let Some(b) = self.advance() {
            if b == b'\n' {
                break;
            }
        }
    }

    fn expect(&mut self, expected: u8) -> Result<(), String> {
        match self.advance() {
            Some(b) if b == expected => Ok(()),
            Some(b) => Err(format!(
                "line {}: expected '{}' got '{}'",
                self.line, expected as char, b as char
            )),
            None => Err(format!(
                "line {}: expected '{}' got EOF",
                self.line, expected as char
            )),
        }
    }

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
            Some(b) => Err(format!(
                "line {}: invalid key character '{}'",
                self.line, b as char
            )),
            None => Err(format!("line {}: unexpected EOF in key", self.line)),
        }
    }

    fn parse_key(&mut self) -> Result<Vec<Vec<u8>>, String> {
        let mut parts = vec![self.parse_key_segment()?];
        self.skip_whitespace();
        while self.peek() == Some(b'.') {
            self.advance();
            self.skip_whitespace();
            parts.push(self.parse_key_segment()?);
            self.skip_whitespace();
        }
        Ok(parts)
    }

    fn parse_basic_string(&mut self) -> Result<Vec<u8>, String> {
        self.expect(b'"')?;
        if self.peek() == Some(b'"') && self.peek2() == Some(b'"') {
            self.advance();
            self.advance();
            if self.peek() == Some(b'\n') {
                self.advance();
            } else if self.peek() == Some(b'\r') && self.peek2() == Some(b'\n') {
                self.advance();
                self.advance();
            }
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
            if self.peek() == Some(b'"') && self.peek2() == Some(b'"') && self.peek3() == Some(b'"') {
                self.advance();
                self.advance();
                self.advance();
                // allow up to 2 extra quotes inside the closing delimiter
                while self.peek() == Some(b'"') && out.last() != Some(&b'"') {
                    out.push(b'"');
                    self.advance();
                }
                break;
            }
            match self.advance() {
                None => {
                    return Err(format!(
                        "line {}: unterminated multi-line string",
                        self.line
                    ))
                }
                Some(b'\\') => {
                    if matches!(
                        self.peek(),
                        Some(b' ') | Some(b'\t') | Some(b'\n') | Some(b'\r')
                    ) {
                        while matches!(
                            self.peek(),
                            Some(b' ') | Some(b'\t') | Some(b'\n') | Some(b'\r')
                        ) {
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
                let cp = self.parse_hex_n(4)?;
                Ok(encode_utf8(cp))
            }
            Some(b'U') => {
                let cp = self.parse_hex_n(8)?;
                Ok(encode_utf8(cp))
            }
            Some(b) => Err(format!(
                "line {}: invalid escape '\\{}'",
                self.line, b as char
            )),
            None => Err(format!("line {}: EOF in escape", self.line)),
        }
    }

    fn parse_hex_n(&mut self, n: usize) -> Result<u32, String> {
        let mut val: u32 = 0;
        for _ in 0..n {
            match self.advance() {
                Some(b) if b.is_ascii_hexdigit() => val = val * 16 + hex_digit(b) as u32,
                Some(b) => {
                    return Err(format!(
                        "line {}: invalid hex digit '{}'",
                        self.line, b as char
                    ))
                }
                None => return Err(format!("line {}: EOF in unicode escape", self.line)),
            }
        }
        Ok(val)
    }

    fn parse_literal_string(&mut self) -> Result<Vec<u8>, String> {
        self.expect(b'\'')?;
        if self.peek() == Some(b'\'') && self.peek2() == Some(b'\'') {
            self.advance();
            self.advance();
            if self.peek() == Some(b'\n') {
                self.advance();
            }
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
            if self.peek() == Some(b'\'') && self.peek2() == Some(b'\'') && self.peek3() == Some(b'\'') {
                self.advance();
                self.advance();
                self.advance();
                break;
            }
            match self.advance() {
                None => {
                    return Err(format!(
                        "line {}: unterminated multi-line literal string",
                        self.line
                    ))
                }
                Some(b) => out.push(b),
            }
        }
        Ok(out)
    }

    fn parse_number_or_date(&mut self) -> Result<TomlValue, String> {
        let start = self.pos;
        if matches!(self.peek(), Some(b'+') | Some(b'-')) {
            self.advance();
        }
        // special float: inf, nan
        if matches!(self.peek(), Some(b'i') | Some(b'n')) {
            while matches!(self.peek(), Some(b) if b.is_ascii_alphabetic()) {
                self.advance();
            }
            return match &self.src[start..self.pos] {
                b"inf" | b"+inf" => Ok(TomlValue::Float(f64::INFINITY)),
                b"-inf" => Ok(TomlValue::Float(f64::NEG_INFINITY)),
                b"nan" | b"+nan" | b"-nan" => Ok(TomlValue::Float(f64::NAN)),
                tok => Err(format!(
                    "line {}: invalid token '{}'",
                    self.line,
                    String::from_utf8_lossy(tok)
                )),
            };
        }
        // 0x / 0o / 0b
        if self.peek() == Some(b'0') {
            match self.peek2() {
                Some(b'x') => {
                    self.advance();
                    self.advance();
                    return self.parse_int_base(16);
                }
                Some(b'o') => {
                    self.advance();
                    self.advance();
                    return self.parse_int_base(8);
                }
                Some(b'b') => {
                    self.advance();
                    self.advance();
                    return self.parse_int_base(2);
                }
                _ => {}
            }
        }
        // collect the rest
        while let Some(b) = self.peek() {
            if b.is_ascii_digit()
                || b == b'.'
                || b == b'e'
                || b == b'E'
                || b == b'_'
                || b == b'-'
                || b == b'+'
                || b == b':'
                || b == b'T'
                || b == b'Z'
                || b == b't'
                || b == b'z'
            {
                self.advance();
            } else {
                break;
            }
        }
        let tok = &self.src[start..self.pos];
        let clean: Vec<u8> = tok.iter().copied().filter(|&b| b != b'_').collect();
        let s = String::from_utf8_lossy(&clean);
        if !clean.contains(&b'.')
            && !clean.contains(&b'e')
            && !clean.contains(&b'E')
            && let Ok(i) = s.parse::<i64>()
        {
            return Ok(TomlValue::Integer(i));
        }
        if let Ok(f) = s.parse::<f64>() {
            return Ok(TomlValue::Float(f));
        }
        // datetime fallback
        Ok(TomlValue::String(tok.to_vec()))
    }

    fn parse_int_base(&mut self, base: i64) -> Result<TomlValue, String> {
        let mut val: i64 = 0;
        let mut had = false;
        while let Some(b) = self.peek() {
            if b == b'_' {
                self.advance();
                continue;
            }
            let digit = match base {
                16 if b.is_ascii_hexdigit() => hex_digit(b) as i64,
                8 if matches!(b, b'0'..=b'7') => (b - b'0') as i64,
                2 if matches!(b, b'0' | b'1') => (b - b'0') as i64,
                _ => break,
            };
            val = val * base + digit;
            had = true;
            self.advance();
        }
        if !had {
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
            Some(b) if b.is_ascii_digit() || matches!(b, b'+' | b'-' | b'i' | b'n') => self.parse_number_or_date(),
            Some(b) => Err(format!(
                "line {}: unexpected character '{}' in value",
                self.line, b as char
            )),
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
                Some(b',') => {
                    self.advance();
                }
                Some(b']') => {
                    self.advance();
                    break;
                }
                Some(b) => {
                    return Err(format!(
                        "line {}: expected ',' or ']' got '{}'",
                        self.line, b as char
                    ))
                }
                None => return Err(format!("line {}: EOF in array", self.line)),
            }
        }
        Ok(TomlValue::Array(items))
    }

    fn parse_inline_table(&mut self) -> Result<TomlValue, String> {
        self.expect(b'{')?;
        let mut pairs: Vec<(Vec<u8>, TomlValue)> = Vec::new();
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
            let pair = build_dotted_value(key_parts, val);
            pairs.push(pair);
            self.skip_whitespace();
            match self.peek() {
                Some(b',') => {
                    self.advance();
                }
                Some(b'}') => {
                    self.advance();
                    break;
                }
                Some(b) => {
                    return Err(format!(
                        "line {}: expected ',' or '}}' got '{}'",
                        self.line, b as char
                    ))
                }
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

fn is_table_key(pairs: &[(Vec<u8>, TomlValue)], key: &[u8]) -> bool {
    pairs
        .iter()
        .any(|(k, v)| k.as_slice() == key && matches!(v, TomlValue::Table(_)))
}

fn get_or_create_table<'a>(pairs: &'a mut Vec<(Vec<u8>, TomlValue)>, key: &[u8]) -> &'a mut Vec<(Vec<u8>, TomlValue)> {
    // Pass 1: check existence — shared borrow ends before push.
    if !is_table_key(pairs, key) {
        pairs.push((key.to_vec(), TomlValue::Table(Vec::new())));
    }
    // Pass 2: find index — shared borrow ends before mutable index borrow.
    let idx = pairs
        .iter()
        .position(|(k, v)| k.as_slice() == key && matches!(v, TomlValue::Table(_)))
        .unwrap();
    // Pass 3: only mutable borrow in scope; its lifetime is 'a.
    if let TomlValue::Table(t) = &mut pairs[idx].1 {
        t
    } else {
        unreachable!()
    }
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
    for seg in &path[..path.len() - 1] {
        cur = get_or_create_table(cur, seg);
    }
    let last = &path[path.len() - 1];
    if let Some(pair) = cur
        .iter_mut()
        .find(|(k, _)| k.as_slice() == last.as_slice())
    {
        if let TomlValue::Array(arr) = &mut pair.1 {
            arr.push(TomlValue::Table(Vec::new()));
            return Ok(());
        }
        return Err(format!(
            "key '{}' is not an array of tables",
            String::from_utf8_lossy(last)
        ));
    }
    cur.push((
        last.clone(),
        TomlValue::Array(vec![TomlValue::Table(Vec::new())]),
    ));
    Ok(())
}

fn insert_into_last_aot(
    root: &mut Vec<(Vec<u8>, TomlValue)>,
    aot_path: &[Vec<u8>],
    key_parts: Vec<Vec<u8>>,
    val: TomlValue,
) -> Result<(), String> {
    let mut cur = root;
    for seg in &aot_path[..aot_path.len() - 1] {
        cur = get_or_create_table(cur, seg);
    }
    let last_seg = &aot_path[aot_path.len() - 1];
    for pair in cur.iter_mut() {
        if pair.0.as_slice() == last_seg.as_slice()
            && let TomlValue::Array(arr) = &mut pair.1
            && let Some(TomlValue::Table(t)) = arr.last_mut()
        {
            return insert_at(t, &key_parts, val);
        }
    }
    Err(format!(
        "array-of-tables '{}' not found",
        String::from_utf8_lossy(last_seg)
    ))
}

fn insert_at(root: &mut Vec<(Vec<u8>, TomlValue)>, path: &[Vec<u8>], val: TomlValue) -> Result<(), String> {
    if path.is_empty() {
        return Err("empty key path".to_string());
    }
    if path.len() == 1 {
        let key = &path[0];
        if root.iter().any(|(k, _)| k.as_slice() == key.as_slice()) {
            return Err(format!("duplicate key '{}'", String::from_utf8_lossy(key)));
        }
        root.push((key.clone(), val));
        return Ok(());
    }
    let cur = get_or_create_table(root, &path[0]);
    insert_at(cur, &path[1..], val)
}

fn parse_document(src: &[u8]) -> Result<Vec<(Vec<u8>, TomlValue)>, String> {
    let mut p = Parser::new(src);
    let mut root: Vec<(Vec<u8>, TomlValue)> = Vec::new();
    let mut current_path: Vec<Vec<u8>> = Vec::new();
    let mut in_array_table = false;

    p.skip_whitespace_and_newlines();
    while p.pos < p.src.len() {
        p.skip_whitespace();
        match p.peek() {
            Some(b'#') => {
                p.skip_comment();
            }
            Some(b'\n') | Some(b'\r') => {
                p.advance();
            }
            Some(b'[') => {
                p.advance();
                if p.peek() == Some(b'[') {
                    p.advance();
                    let key = p.parse_key()?;
                    p.skip_whitespace();
                    p.expect(b']')?;
                    p.expect(b']')?;
                    current_path = key;
                    in_array_table = true;
                    insert_aot(&mut root, &current_path)?;
                } else {
                    let key = p.parse_key()?;
                    p.skip_whitespace();
                    p.expect(b']')?;
                    current_path = key;
                    in_array_table = false;
                    ensure_table(&mut root, &current_path)?;
                }
                p.skip_whitespace();
                if p.peek() == Some(b'#') {
                    p.skip_comment();
                }
                if matches!(p.peek(), Some(b'\n') | Some(b'\r')) {
                    p.advance();
                }
            }
            Some(_) => {
                let key_parts = p.parse_key()?;
                p.skip_whitespace();
                p.expect(b'=')?;
                p.skip_whitespace();
                let val = p.parse_value()?;
                p.skip_whitespace();
                if p.peek() == Some(b'#') {
                    p.skip_comment();
                }
                if matches!(p.peek(), Some(b'\n') | Some(b'\r')) {
                    p.advance();
                }
                if in_array_table {
                    insert_into_last_aot(&mut root, &current_path, key_parts, val)?;
                } else {
                    let mut full_path = current_path.clone();
                    full_path.extend_from_slice(&key_parts);
                    insert_at(&mut root, &full_path, val)?;
                }
            }
            None => break,
        }
    }
    Ok(root)
}

// ─── Push a TomlValue onto the Lua stack ─────────────────────────────────────

unsafe fn push_toml_value(state: *mut State, val: &TomlValue) {
    unsafe {
        match val {
            TomlValue::String(s) => {
                lua_pushlstring(state, s.as_ptr() as *const i8, s.len());
            }
            TomlValue::Integer(i) => {
                (*state).push_integer(*i);
            }
            TomlValue::Float(f) => {
                (*state).push_number(*f);
            }
            TomlValue::Bool(b) => {
                (*state).push_boolean(*b);
            }
            TomlValue::Array(items) => {
                (*state).lua_createtable();
                let tbl = lua_absindex(state, -1);
                for (i, item) in items.iter().enumerate() {
                    push_toml_value(state, item);
                    lua_rawseti(state, tbl, (i + 1) as i64);
                }
            }
            TomlValue::Table(pairs) => {
                (*state).lua_createtable();
                let tbl = lua_absindex(state, -1);
                for (k, v) in pairs {
                    lua_pushlstring(state, k.as_ptr() as *const i8, k.len());
                    push_toml_value(state, v);
                    lua_rawset(state, tbl);
                }
            }
        }
    }
}

// ─── toml.parse ──────────────────────────────────────────────────────────────

pub unsafe fn toml_parse(state: *mut State) -> i32 {
    unsafe {
        let mut len: usize = 0;
        let ptr = lua_tolstring(state, 1, &mut len);
        if ptr.is_null() {
            return lual_error(state, c"toml.parse: expected string argument".as_ptr(), &[]);
        }
        let src = std::slice::from_raw_parts(ptr as *const u8, len);
        match parse_document(src) {
            Err(e) => {
                (*state).push_nil();
                lua_pushlstring(state, e.as_ptr() as *const i8, e.len());
                2
            }
            Ok(pairs) => {
                push_toml_value(state, &TomlValue::Table(pairs));
                1
            }
        }
    }
}

// ─── TOML serializer ─────────────────────────────────────────────────────────

unsafe fn is_lua_array(state: *mut State, idx: i32) -> bool {
    unsafe {
        let n = get_length_raw(state, idx);
        // Check that all keys are integers 1..n
        (*state).push_nil();
        let mut count: usize = 0;
        while lua_next(state, idx) != 0 {
            count += 1;
            let mut is_int = false;
            let ki = lua_tointegerx(state, -2, &mut is_int);
            if !is_int || ki < 1 || ki as usize > n {
                lua_settop(state, -3); // pop key and value
                return false;
            }
            lua_settop(state, -2); // pop value
        }
        count == n
    }
}

fn write_toml_key(out: &mut Vec<u8>, key: &[u8]) {
    if !key.is_empty()
        && key
            .iter()
            .all(|&b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
    {
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

/// Serialize a scalar (non-table) or inline array/inline-table value.
unsafe fn serialize_value(state: *mut State, idx: i32, out: &mut Vec<u8>) -> Result<(), String> {
    unsafe {
        let idx = lua_absindex(state, idx);
        match lua_type(state, idx) {
            Some(TagType::Boolean) => {
                if lua_toboolean(state, idx) {
                    out.extend_from_slice(b"true");
                } else {
                    out.extend_from_slice(b"false");
                }
            }
            Some(TagType::Numeric) => {
                if lua_isinteger(state, idx) {
                    let i = lua_tointegerx(state, idx, std::ptr::null_mut());
                    out.extend_from_slice(i.to_string().as_bytes());
                } else {
                    let f = lua_tonumberx(state, idx, std::ptr::null_mut());
                    if f.is_nan() {
                        out.extend_from_slice(b"nan");
                    } else if f.is_infinite() {
                        if f > 0.0 {
                            out.extend_from_slice(b"inf");
                        } else {
                            out.extend_from_slice(b"-inf");
                        }
                    } else {
                        let s = format!("{}", f);
                        out.extend_from_slice(s.as_bytes());
                        if !s.contains('.') && !s.contains('e') {
                            out.extend_from_slice(b".0");
                        }
                    }
                }
            }
            Some(TagType::String) => {
                let mut slen: usize = 0;
                let sptr = lua_tolstring(state, idx, &mut slen);
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
            }
            Some(TagType::Table) => {
                if is_lua_array(state, idx) {
                    out.push(b'[');
                    let n = get_length_raw(state, idx);
                    for i in 1..=n {
                        if i > 1 {
                            out.extend_from_slice(b", ");
                        }
                        lua_rawgeti(state, idx, i as i64);
                        serialize_value(state, -1, out)?;
                        lua_settop(state, -2);
                    }
                    out.push(b']');
                } else {
                    // Inline table
                    out.push(b'{');
                    let mut first = true;
                    (*state).push_nil();
                    while lua_next(state, idx) != 0 {
                        if !first {
                            out.extend_from_slice(b", ");
                        }
                        first = false;
                        let mut klen: usize = 0;
                        let kptr = lua_tolstring(state, -2, &mut klen);
                        if kptr.is_null() {
                            lua_settop(state, -3);
                            return Err("toml.stringify: table keys must be strings".to_string());
                        }
                        let kslice = std::slice::from_raw_parts(kptr as *const u8, klen);
                        write_toml_key(out, kslice);
                        out.extend_from_slice(b" = ");
                        serialize_value(state, -1, out)?;
                        lua_settop(state, -2);
                    }
                    out.push(b'}');
                }
            }
            tt => {
                return Err(format!(
                    "toml.stringify: cannot serialize type {:?}",
                    tt.map(|t| t as u8)
                ));
            }
        }
        Ok(())
    }
}

/// Emit a table as a TOML section (top-level or under [header]).
/// `path` is the dotted header name used for sub-sections.
unsafe fn emit_table_section(state: *mut State, idx: i32, out: &mut Vec<u8>, path: &str) -> Result<(), String> {
    unsafe {
        let idx = lua_absindex(state, idx);

        // Pass 1: scalars and plain arrays (not arrays-of-tables)
        (*state).push_nil();
        while lua_next(state, idx) != 0 {
            let v_idx = lua_absindex(state, -1);
            let is_subtable = matches!(lua_type(state, v_idx), Some(TagType::Table)) && !is_inline_value(state, v_idx);
            if !is_subtable {
                // key
                let mut klen: usize = 0;
                let kptr = lua_tolstring(state, -2, &mut klen);
                if kptr.is_null() {
                    lua_settop(state, -3);
                    return Err("toml.stringify: table keys must be strings".to_string());
                }
                let kslice = std::slice::from_raw_parts(kptr as *const u8, klen);
                write_toml_key(out, kslice);
                out.extend_from_slice(b" = ");
                serialize_value(state, v_idx, out)?;
                out.push(b'\n');
            }
            lua_settop(state, -2);
        }

        // Pass 2: sub-tables and arrays-of-tables
        (*state).push_nil();
        while lua_next(state, idx) != 0 {
            let v_idx = lua_absindex(state, -1);
            if matches!(lua_type(state, v_idx), Some(TagType::Table)) && !is_inline_value(state, v_idx) {
                let mut klen: usize = 0;
                let kptr = lua_tolstring(state, -2, &mut klen);
                if kptr.is_null() {
                    lua_settop(state, -3);
                    return Err("toml.stringify: table keys must be strings".to_string());
                }
                let kslice = std::slice::from_raw_parts(kptr as *const u8, klen);
                let key_str = String::from_utf8_lossy(kslice);
                let child_path = if path.is_empty() {
                    key_str.into_owned()
                } else {
                    format!("{}.{}", path, key_str)
                };

                if is_array_of_tables(state, v_idx) {
                    let n = get_length_raw(state, v_idx);
                    for i in 1..=n {
                        out.extend_from_slice(b"\n[[");
                        out.extend_from_slice(child_path.as_bytes());
                        out.extend_from_slice(b"]]\n");
                        lua_rawgeti(state, v_idx, i as i64);
                        let elem = lua_absindex(state, -1);
                        emit_table_section(state, elem, out, &child_path)?;
                        lua_settop(state, -2);
                    }
                } else {
                    out.extend_from_slice(b"\n[");
                    out.extend_from_slice(child_path.as_bytes());
                    out.extend_from_slice(b"]\n");
                    emit_table_section(state, v_idx, out, &child_path)?;
                }
            }
            lua_settop(state, -2);
        }
        Ok(())
    }
}

/// Returns true if this table should be emitted inline (i.e. it's an empty table,
/// or all values are scalars/plain-arrays — no nested table headers needed).
/// We use a simple heuristic: arrays-of-tables and sub-tables go as headers,
/// everything else inline. A plain array of scalars stays inline.
unsafe fn is_inline_value(state: *mut State, idx: i32) -> bool {
    unsafe {
        if is_array_of_tables(state, idx) {
            return false;
        }
        if !is_lua_array(state, idx) {
            // A regular table: check if it has any sub-table children
            (*state).push_nil();
            while lua_next(state, idx) != 0 {
                let v = lua_absindex(state, -1);
                if matches!(lua_type(state, v), Some(TagType::Table)) {
                    lua_settop(state, -3);
                    return false; // has a sub-table → needs header
                }
                lua_settop(state, -2);
            }
        }
        true
    }
}

/// Returns true if `idx` is a Lua array where every element is a table.
unsafe fn is_array_of_tables(state: *mut State, idx: i32) -> bool {
    unsafe {
        if !is_lua_array(state, idx) {
            return false;
        }
        let n = get_length_raw(state, idx);
        if n == 0 {
            return false;
        }
        lua_rawgeti(state, idx, 1);
        let first_is_table = matches!(lua_type(state, -1), Some(TagType::Table));
        lua_settop(state, -2);
        first_is_table
    }
}

pub unsafe fn toml_stringify(state: *mut State) -> i32 {
    unsafe {
        if !matches!(lua_type(state, 1), Some(TagType::Table)) {
            return lual_error(
                state,
                c"toml.stringify: expected table argument".as_ptr(),
                &[],
            );
        }
        let mut out: Vec<u8> = Vec::new();
        match emit_table_section(state, 1, &mut out, "") {
            Err(e) => {
                (*state).push_nil();
                lua_pushlstring(state, e.as_ptr() as *const i8, e.len());
                2
            }
            Ok(()) => {
                lua_pushlstring(state, out.as_ptr() as *const i8, out.len());
                1
            }
        }
    }
}

// ─── Library registration ─────────────────────────────────────────────────────

pub const TOML_FUNCTIONS: [RegisteredFunction; 2] = [
    RegisteredFunction {
        registeredfunction_name: c"parse".as_ptr(),
        registeredfunction_function: Some(toml_parse as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"stringify".as_ptr(),
        registeredfunction_function: Some(toml_stringify as unsafe fn(*mut State) -> i32),
    },
];

pub unsafe fn luaopen_toml(state: *mut State) -> i32 {
    unsafe {
        (*state).lua_createtable();
        lual_setfuncs(state, TOML_FUNCTIONS.as_ptr(), TOML_FUNCTIONS.len(), 0);
        1
    }
}
