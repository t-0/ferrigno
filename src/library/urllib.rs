use crate::interpreter::*;
use crate::registeredfunction::*;
use crate::tagtype::*;
use std::ptr::*;

// ─── libcurl FFI ──────────────────────────────────────────────────────────────

#[link(name = "curl")]
unsafe extern "C" {
    fn curl_easy_init() -> *mut libc::c_void;
    fn curl_easy_setopt(handle: *mut libc::c_void, option: libc::c_int, ...) -> libc::c_int;
    fn curl_easy_perform(handle: *mut libc::c_void) -> libc::c_int;
    fn curl_easy_cleanup(handle: *mut libc::c_void);
    fn curl_easy_strerror(code: libc::c_int) -> *const libc::c_char;
    fn curl_slist_append(list: *mut libc::c_void, string: *const libc::c_char) -> *mut libc::c_void;
    fn curl_slist_free_all(list: *mut libc::c_void);

    fn curl_easy_getinfo(handle: *mut libc::c_void, info: libc::c_int, ...) -> libc::c_int;
}

// CURLOPT constants (from curl.h)
const CURLOPT_URL: libc::c_int = 10002;
const CURLOPT_WRITEFUNCTION: libc::c_int = 20011;
const CURLOPT_WRITEDATA: libc::c_int = 10001;
const CURLOPT_HEADERFUNCTION: libc::c_int = 20079;
const CURLOPT_HEADERDATA: libc::c_int = 10029;
const CURLOPT_HTTPHEADER: libc::c_int = 10023;
const CURLOPT_POST: libc::c_int = 47;
const CURLOPT_POSTFIELDS: libc::c_int = 10015;
const CURLOPT_POSTFIELDSIZE: libc::c_int = 60;
const CURLOPT_CUSTOMREQUEST: libc::c_int = 10036;
const CURLOPT_FOLLOWLOCATION: libc::c_int = 52;
const CURLOPT_USERAGENT: libc::c_int = 10018;

// CURLINFO constants
const CURLINFO_RESPONSE_CODE: libc::c_int = 0x200002;

// Write callback: appends received data into a Vec<u8>.
unsafe extern "C" fn write_callback(
    data: *const libc::c_char,
    size: libc::size_t,
    nmemb: libc::size_t,
    userdata: *mut libc::c_void,
) -> libc::size_t {
    unsafe {
        let buf = userdata as *mut Vec<u8>;
        let n = size * nmemb;
        let slice = std::slice::from_raw_parts(data as *const u8, n);
        (*buf).extend_from_slice(slice);
        n
    }
}

// Header callback: appends raw header lines into a Vec<u8>.
unsafe extern "C" fn header_callback(
    data: *const libc::c_char,
    size: libc::size_t,
    nmemb: libc::size_t,
    userdata: *mut libc::c_void,
) -> libc::size_t {
    unsafe {
        let buf = userdata as *mut Vec<u8>;
        let n = size * nmemb;
        let slice = std::slice::from_raw_parts(data as *const u8, n);
        (*buf).extend_from_slice(slice);
        n
    }
}

// ─── URL encoding / decoding ──────────────────────────────────────────────────

/// Percent-encodes a byte, returning true if it is an unreserved character.
fn is_unreserved(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'-' || b == b'_' || b == b'.' || b == b'~'
}

/// Percent-encode a byte slice (RFC 3986 unreserved characters are left as-is).
fn percent_encode(input: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(input.len());
    for &b in input {
        if is_unreserved(b) {
            out.push(b);
        } else {
            out.push(b'%');
            out.push(hex_nibble(b >> 4));
            out.push(hex_nibble(b & 0xf));
        }
    }
    out
}

fn hex_nibble(n: u8) -> u8 {
    if n < 10 { b'0' + n } else { b'A' + n - 10 }
}

fn from_hex(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

/// Percent-decode a byte slice.  `+` is decoded as a literal `+` (not space).
fn percent_decode(input: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(input.len());
    let mut i = 0;
    while i < input.len() {
        if input[i] == b'%' && i + 2 < input.len() {
            if let (Some(hi), Some(lo)) = (from_hex(input[i + 1]), from_hex(input[i + 2])) {
                out.push(hi << 4 | lo);
                i += 3;
                continue;
            }
        }
        out.push(input[i]);
        i += 1;
    }
    out
}

// ─── URL parsing ─────────────────────────────────────────────────────────────

struct ParsedUrl {
    scheme:   Vec<u8>,
    user:     Vec<u8>,
    password: Vec<u8>,
    host:     Vec<u8>,
    port:     Option<u16>,
    path:     Vec<u8>,
    query:    Vec<u8>,
    fragment: Vec<u8>,
}

fn parse_url(url: &[u8]) -> ParsedUrl {
    let mut rest = url;

    // fragment
    let (main, fragment) = split_off(rest, b'#');
    rest = main;
    let fragment = fragment.unwrap_or(&[]).to_vec();

    // query
    let (main, query) = split_off(rest, b'?');
    rest = main;
    let query = query.unwrap_or(&[]).to_vec();

    // scheme
    let scheme;
    if let Some(pos) = find_bytes(rest, b"://") {
        scheme = rest[..pos].to_vec();
        rest = &rest[pos + 3..];
    } else {
        scheme = Vec::new();
    }

    // path
    let (authority, path) = split_at_slash(rest);
    let path = path.to_vec();
    rest = authority;

    // user:password@host
    let (userinfo, hostpart) = if let Some(at) = memrchr(rest, b'@') {
        (&rest[..at], &rest[at + 1..])
    } else {
        (&[][..], rest)
    };
    let (user, password) = if userinfo.is_empty() {
        (Vec::new(), Vec::new())
    } else {
        let (u, p) = split_off(userinfo, b':');
        (u.to_vec(), p.unwrap_or(&[]).to_vec())
    };

    // host:port  (handle IPv6 brackets)
    let (host, port) = if hostpart.starts_with(b"[") {
        // IPv6
        if let Some(close) = memchr(hostpart, b']') {
            let h = hostpart[1..close].to_vec();
            let after = &hostpart[close + 1..];
            let port = if after.starts_with(b":") {
                parse_port(&after[1..])
            } else {
                None
            };
            (h, port)
        } else {
            (hostpart.to_vec(), None)
        }
    } else {
        let (h, p) = split_off(hostpart, b':');
        let port = p.and_then(parse_port);
        (h.to_vec(), port)
    };

    ParsedUrl { scheme, user, password, host, port, path, query, fragment }
}

fn split_off<'a>(s: &'a [u8], sep: u8) -> (&'a [u8], Option<&'a [u8]>) {
    if let Some(pos) = memchr(s, sep) {
        (&s[..pos], Some(&s[pos + 1..]))
    } else {
        (s, None)
    }
}

fn split_at_slash(s: &[u8]) -> (&[u8], &[u8]) {
    if let Some(pos) = memchr(s, b'/') {
        (&s[..pos], &s[pos..])
    } else {
        (s, b"/")
    }
}

fn find_bytes(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|w| w == needle)
}

fn memchr(s: &[u8], c: u8) -> Option<usize> {
    s.iter().position(|&b| b == c)
}

fn memrchr(s: &[u8], c: u8) -> Option<usize> {
    s.iter().rposition(|&b| b == c)
}

fn parse_port(s: &[u8]) -> Option<u16> {
    let n: u32 = s.iter().try_fold(0u32, |acc, &b| {
        if b.is_ascii_digit() { Some(acc * 10 + (b - b'0') as u32) } else { None }
    })?;
    if n <= 65535 { Some(n as u16) } else { None }
}

// ─── HTTP request helper ──────────────────────────────────────────────────────

/// Perform an HTTP(S) request.
/// Returns (body_bytes, status_code, raw_headers_bytes) or an error string.
unsafe fn do_request(
    method: &[u8],
    url: &[u8],
    body: Option<&[u8]>,
    headers: *mut libc::c_void, // curl_slist* or null
) -> Result<(Vec<u8>, i64, Vec<u8>), Vec<u8>> {
    unsafe {
        let curl = curl_easy_init();
        if curl.is_null() {
            return Err(b"curl_easy_init failed".to_vec());
        }

        // Null-terminate the URL
        let mut url_buf = url.to_vec();
        url_buf.push(0);
        curl_easy_setopt(curl, CURLOPT_URL, url_buf.as_ptr() as *const libc::c_char);

        // Follow redirects
        curl_easy_setopt(curl, CURLOPT_FOLLOWLOCATION, 1i64);

        // User-Agent
        curl_easy_setopt(curl, CURLOPT_USERAGENT, c"rlua-urllib/1.0".as_ptr());

        // Write callback for body
        let mut body_buf: Vec<u8> = Vec::new();
        curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, write_callback as *mut libc::c_void);
        curl_easy_setopt(curl, CURLOPT_WRITEDATA, &mut body_buf as *mut Vec<u8> as *mut libc::c_void);

        // Header callback
        let mut header_buf: Vec<u8> = Vec::new();
        curl_easy_setopt(curl, CURLOPT_HEADERFUNCTION, header_callback as *mut libc::c_void);
        curl_easy_setopt(curl, CURLOPT_HEADERDATA, &mut header_buf as *mut Vec<u8> as *mut libc::c_void);

        // Custom headers
        if !headers.is_null() {
            curl_easy_setopt(curl, CURLOPT_HTTPHEADER, headers);
        }

        // Method / body
        match method {
            b"GET" | b"get" => {}
            b"POST" | b"post" => {
                curl_easy_setopt(curl, CURLOPT_POST, 1i64);
                if let Some(data) = body {
                    curl_easy_setopt(curl, CURLOPT_POSTFIELDS, data.as_ptr() as *const libc::c_char);
                    curl_easy_setopt(curl, CURLOPT_POSTFIELDSIZE, data.len() as i64);
                } else {
                    curl_easy_setopt(curl, CURLOPT_POSTFIELDS, c"".as_ptr());
                    curl_easy_setopt(curl, CURLOPT_POSTFIELDSIZE, 0i64);
                }
            }
            m => {
                let mut m_buf = m.to_vec();
                for b in &mut m_buf {
                    *b = b.to_ascii_uppercase();
                }
                m_buf.push(0);
                curl_easy_setopt(curl, CURLOPT_CUSTOMREQUEST, m_buf.as_ptr() as *const libc::c_char);
                if let Some(data) = body {
                    curl_easy_setopt(curl, CURLOPT_POSTFIELDS, data.as_ptr() as *const libc::c_char);
                    curl_easy_setopt(curl, CURLOPT_POSTFIELDSIZE, data.len() as i64);
                }
            }
        }

        let rc = curl_easy_perform(curl);

        if rc != 0 {
            let err_ptr = curl_easy_strerror(rc);
            let err = std::ffi::CStr::from_ptr(err_ptr).to_bytes().to_vec();
            curl_easy_cleanup(curl);
            return Err(err);
        }

        let mut status_code: libc::c_long = 0;
        curl_easy_getinfo(curl, CURLINFO_RESPONSE_CODE, &mut status_code as *mut libc::c_long);
        curl_easy_cleanup(curl);

        Ok((body_buf, status_code as i64, header_buf))
    }
}

/// Parse raw header bytes into a Lua table: { [name] = value, ... }
/// Skips the status line. Duplicate header names produce a comma-joined value.
unsafe fn push_header_table(interpreter: *mut Interpreter, raw: &[u8]) {
    unsafe {
        (*interpreter).lua_createtable();
        for line in raw.split(|&b| b == b'\n') {
            let line = strip_crlf(line);
            if line.is_empty() { continue; }
            // Skip HTTP/1.x status line
            if line.starts_with(b"HTTP/") { continue; }
            if let Some(colon) = memchr(line, b':') {
                let name_raw = trim_bytes(&line[..colon]);
                let value_raw = trim_bytes(&line[colon + 1..]);
                if name_raw.is_empty() { continue; }
                // Lowercase the header name for consistent keys
                let name: Vec<u8> = name_raw.iter().map(|b| b.to_ascii_lowercase()).collect();
                // If already present, append with comma
                lua_pushlstring(interpreter, name.as_ptr() as *const i8, name.len());
                let ty = lua_rawget(interpreter, -2);
                if ty == TagType::String {
                    // Concatenate existing value with new value
                    let mut existing_len = 0usize;
                    let existing_ptr = lua_tolstring(interpreter, -1, &mut existing_len);
                    let mut combined = std::slice::from_raw_parts(existing_ptr as *const u8, existing_len).to_vec();
                    combined.extend_from_slice(b", ");
                    combined.extend_from_slice(value_raw);
                    lua_settop(interpreter, -2); // pop old value
                    // Push key again
                    lua_pushlstring(interpreter, name.as_ptr() as *const i8, name.len());
                    lua_pushlstring(interpreter, combined.as_ptr() as *const i8, combined.len());
                } else {
                    lua_settop(interpreter, -2); // pop nil/other
                    // Push key+value fresh
                    lua_pushlstring(interpreter, name.as_ptr() as *const i8, name.len());
                    lua_pushlstring(interpreter, value_raw.as_ptr() as *const i8, value_raw.len());
                }
                lua_rawset(interpreter, -3);
            }
        }
    }
}

fn strip_crlf(s: &[u8]) -> &[u8] {
    let mut end = s.len();
    while end > 0 && (s[end - 1] == b'\r' || s[end - 1] == b'\n') {
        end -= 1;
    }
    &s[..end]
}

fn trim_bytes(s: &[u8]) -> &[u8] {
    let start = s.iter().position(|&b| b != b' ' && b != b'\t').unwrap_or(s.len());
    let end = s.iter().rposition(|&b| b != b' ' && b != b'\t').map(|i| i + 1).unwrap_or(0);
    if start >= end { &[] } else { &s[start..end] }
}

/// Build a curl_slist from a Lua table at stack index `idx`.
/// The caller must free the returned list with curl_slist_free_all.
unsafe fn build_header_slist(interpreter: *mut Interpreter, idx: i32) -> *mut libc::c_void {
    unsafe {
        let mut list: *mut libc::c_void = null_mut();
        (*interpreter).push_nil();
        let abs_idx = if idx < 0 { (*interpreter).get_top() + idx } else { idx };
        while lua_next(interpreter, abs_idx) != 0 {
            // Capture absolute positions before lual_tolstring shifts the stack
            let abs_key = (*interpreter).get_top() - 1;
            let abs_val = (*interpreter).get_top();
            let mut klen = 0usize;
            let mut vlen = 0usize;
            let kptr = lual_tolstring(interpreter, abs_key, &mut klen);
            let vptr = lual_tolstring(interpreter, abs_val, &mut vlen);
            if !kptr.is_null() && !vptr.is_null() {
                let k = std::slice::from_raw_parts(kptr as *const u8, klen);
                let v = std::slice::from_raw_parts(vptr as *const u8, vlen);
                let mut hdr: Vec<u8> = Vec::with_capacity(klen + 2 + vlen + 1);
                hdr.extend_from_slice(k);
                hdr.extend_from_slice(b": ");
                hdr.extend_from_slice(v);
                hdr.push(0);
                list = curl_slist_append(list, hdr.as_ptr() as *const i8);
            }
            lua_settop(interpreter, abs_key); // restore to just key for next lua_next
        }
        list
    }
}

// ─── Lua-callable functions ───────────────────────────────────────────────────

/// urllib.encode(str) → percent-encoded string
pub unsafe fn urllib_encode(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let mut len = 0usize;
        let ptr = lual_checklstring(interpreter, 1, &mut len);
        let input = std::slice::from_raw_parts(ptr as *const u8, len);
        let encoded = percent_encode(input);
        lua_pushlstring(interpreter, encoded.as_ptr() as *const i8, encoded.len());
        1
    }
}

/// urllib.decode(str) → percent-decoded string
pub unsafe fn urllib_decode(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let mut len = 0usize;
        let ptr = lual_checklstring(interpreter, 1, &mut len);
        let input = std::slice::from_raw_parts(ptr as *const u8, len);
        let decoded = percent_decode(input);
        lua_pushlstring(interpreter, decoded.as_ptr() as *const i8, decoded.len());
        1
    }
}

/// urllib.parse(url) → table
pub unsafe fn urllib_parse(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let mut len = 0usize;
        let ptr = lual_checklstring(interpreter, 1, &mut len);
        let url = std::slice::from_raw_parts(ptr as *const u8, len);
        let p = parse_url(url);

        (*interpreter).lua_createtable();

        let push_field = |interp: *mut Interpreter, key: &[u8], val: &[u8]| {
            lua_pushlstring(interp, key.as_ptr() as *const i8, key.len());
            lua_pushlstring(interp, val.as_ptr() as *const i8, val.len());
            lua_rawset(interp, -3);
        };

        push_field(interpreter, b"scheme",   &p.scheme);
        push_field(interpreter, b"user",     &p.user);
        push_field(interpreter, b"password", &p.password);
        push_field(interpreter, b"host",     &p.host);
        push_field(interpreter, b"path",     &p.path);
        push_field(interpreter, b"query",    &p.query);
        push_field(interpreter, b"fragment", &p.fragment);

        lua_pushlstring(interpreter, b"port".as_ptr() as *const i8, 4);
        if let Some(port) = p.port {
            (*interpreter).push_integer(port as i64);
        } else {
            (*interpreter).push_nil();
        }
        lua_rawset(interpreter, -3);

        1
    }
}

/// urllib.request(method, url [, body [, headers]]) → body, status, headers  |  nil, errmsg
pub unsafe fn urllib_request(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let mut mlen = 0usize;
        let mptr = lual_checklstring(interpreter, 1, &mut mlen);
        let method = std::slice::from_raw_parts(mptr as *const u8, mlen);

        let mut ulen = 0usize;
        let uptr = lual_checklstring(interpreter, 2, &mut ulen);
        let url = std::slice::from_raw_parts(uptr as *const u8, ulen);

        // Optional body (arg 3)
        let body_data: Option<Vec<u8>> = if lua_type(interpreter, 3) == Some(TagType::String) {
            let mut blen = 0usize;
            let bptr = lua_tolstring(interpreter, 3, &mut blen);
            Some(std::slice::from_raw_parts(bptr as *const u8, blen).to_vec())
        } else {
            None
        };

        // Optional headers table (arg 4)
        let mut slist: *mut libc::c_void = null_mut();
        if lua_type(interpreter, 4) == Some(TagType::Table) {
            slist = build_header_slist(interpreter, 4);
        }

        let result = do_request(
            method,
            url,
            body_data.as_deref(),
            slist,
        );

        if !slist.is_null() {
            curl_slist_free_all(slist);
        }

        match result {
            Ok((body, status, raw_headers)) => {
                lua_pushlstring(interpreter, body.as_ptr() as *const i8, body.len());
                (*interpreter).push_integer(status);
                push_header_table(interpreter, &raw_headers);
                3
            }
            Err(msg) => {
                (*interpreter).push_nil();
                lua_pushlstring(interpreter, msg.as_ptr() as *const i8, msg.len());
                2
            }
        }
    }
}

/// urllib.get(url [, headers]) → body, status, headers  |  nil, errmsg
pub unsafe fn urllib_get(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let mut ulen = 0usize;
        let uptr = lual_checklstring(interpreter, 1, &mut ulen);
        let url = std::slice::from_raw_parts(uptr as *const u8, ulen);

        let mut slist: *mut libc::c_void = null_mut();
        if lua_type(interpreter, 2) == Some(TagType::Table) {
            slist = build_header_slist(interpreter, 2);
        }

        let result = do_request(b"GET", url, None, slist);

        if !slist.is_null() {
            curl_slist_free_all(slist);
        }

        match result {
            Ok((body, status, raw_headers)) => {
                lua_pushlstring(interpreter, body.as_ptr() as *const i8, body.len());
                (*interpreter).push_integer(status);
                push_header_table(interpreter, &raw_headers);
                3
            }
            Err(msg) => {
                (*interpreter).push_nil();
                lua_pushlstring(interpreter, msg.as_ptr() as *const i8, msg.len());
                2
            }
        }
    }
}

/// urllib.post(url, body [, headers]) → body, status, headers  |  nil, errmsg
pub unsafe fn urllib_post(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let mut ulen = 0usize;
        let uptr = lual_checklstring(interpreter, 1, &mut ulen);
        let url = std::slice::from_raw_parts(uptr as *const u8, ulen);

        let mut blen = 0usize;
        let bptr = lual_checklstring(interpreter, 2, &mut blen);
        let body = std::slice::from_raw_parts(bptr as *const u8, blen).to_vec();

        let mut slist: *mut libc::c_void = null_mut();
        if lua_type(interpreter, 3) == Some(TagType::Table) {
            slist = build_header_slist(interpreter, 3);
        }

        let result = do_request(b"POST", url, Some(&body), slist);

        if !slist.is_null() {
            curl_slist_free_all(slist);
        }

        match result {
            Ok((body, status, raw_headers)) => {
                lua_pushlstring(interpreter, body.as_ptr() as *const i8, body.len());
                (*interpreter).push_integer(status);
                push_header_table(interpreter, &raw_headers);
                3
            }
            Err(msg) => {
                (*interpreter).push_nil();
                lua_pushlstring(interpreter, msg.as_ptr() as *const i8, msg.len());
                2
            }
        }
    }
}

// ─── Library registration ─────────────────────────────────────────────────────

pub const URLLIB_FUNCTIONS: [RegisteredFunction; 6] = [
    RegisteredFunction {
        registeredfunction_name: c"encode".as_ptr(),
        registeredfunction_function: Some(urllib_encode as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"decode".as_ptr(),
        registeredfunction_function: Some(urllib_decode as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"parse".as_ptr(),
        registeredfunction_function: Some(urllib_parse as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"get".as_ptr(),
        registeredfunction_function: Some(urllib_get as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"post".as_ptr(),
        registeredfunction_function: Some(urllib_post as unsafe fn(*mut Interpreter) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"request".as_ptr(),
        registeredfunction_function: Some(urllib_request as unsafe fn(*mut Interpreter) -> i32),
    },
];

pub unsafe fn luaopen_urllib(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        (*interpreter).lua_createtable();
        lual_setfuncs(interpreter, URLLIB_FUNCTIONS.as_ptr(), URLLIB_FUNCTIONS.len(), 0);
        1
    }
}
