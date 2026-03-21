use crate::library::json::{json_null_ptr, JsonParser};
use crate::registeredfunction::*;
use crate::state::*;
use crate::tagtype::*;
use crate::utility::*;
use std::ptr::*;

// ─── libcurl FFI ──────────────────────────────────────────────────────────────

#[link(name = "curl")]
unsafe extern "C" {
    fn curl_easy_init() -> *mut std::ffi::c_void;
    fn curl_easy_setopt(handle: *mut std::ffi::c_void, option: std::ffi::c_int, ...) -> std::ffi::c_int;
    fn curl_easy_perform(handle: *mut std::ffi::c_void) -> std::ffi::c_int;
    fn curl_easy_cleanup(handle: *mut std::ffi::c_void);
    fn curl_easy_strerror(code: std::ffi::c_int) -> *const std::ffi::c_char;
    fn curl_easy_getinfo(handle: *mut std::ffi::c_void, info: std::ffi::c_int, ...) -> std::ffi::c_int;
    fn curl_slist_append(list: *mut std::ffi::c_void, string: *const std::ffi::c_char) -> *mut std::ffi::c_void;
    fn curl_slist_free_all(list: *mut std::ffi::c_void);
}

// CURLOPT constants
const CURLOPT_URL: std::ffi::c_int = 10002;
const CURLOPT_USERAGENT: std::ffi::c_int = 10018;
const CURLOPT_USERPWD: std::ffi::c_int = 10005;
const CURLOPT_COOKIE: std::ffi::c_int = 10022;
const CURLOPT_HTTPHEADER: std::ffi::c_int = 10023;
const CURLOPT_POSTFIELDS: std::ffi::c_int = 10015;
const CURLOPT_CUSTOMREQUEST: std::ffi::c_int = 10036;
const CURLOPT_FOLLOWLOCATION: std::ffi::c_int = 52;
const CURLOPT_NOBODY: std::ffi::c_int = 44;
const CURLOPT_POST: std::ffi::c_int = 47;
const CURLOPT_POSTFIELDSIZE: std::ffi::c_int = 60;
const CURLOPT_TIMEOUT: std::ffi::c_int = 13;
const CURLOPT_CONNECTTIMEOUT: std::ffi::c_int = 78;
const CURLOPT_SSL_VERIFYPEER: std::ffi::c_int = 64;
const CURLOPT_SSL_VERIFYHOST: std::ffi::c_int = 81;
const CURLOPT_WRITEFUNCTION: std::ffi::c_int = 20011;
const CURLOPT_WRITEDATA: std::ffi::c_int = 10001;
const CURLOPT_HEADERFUNCTION: std::ffi::c_int = 20079;
const CURLOPT_HEADERDATA: std::ffi::c_int = 10029;

// CURLINFO constants
const CURLINFO_RESPONSE_CODE: std::ffi::c_int = 0x200002;
const CURLINFO_EFFECTIVE_URL: std::ffi::c_int = 0x100001;

// ─── Helpers ──────────────────────────────────────────────────────────────────

unsafe extern "C" fn write_cb(data: *const std::ffi::c_char, size: usize, nmemb: usize, user_data: *mut std::ffi::c_void) -> usize {
    unsafe {
        let buf = user_data as *mut Vec<u8>;
        let n = size * nmemb;
        (*buf).extend_from_slice(std::slice::from_raw_parts(data as *const u8, n));
        n
    }
}

unsafe extern "C" fn header_cb(
    data: *const std::ffi::c_char,
    size: usize,
    nmemb: usize,
    user_data: *mut std::ffi::c_void,
) -> usize {
    unsafe {
        let buf = user_data as *mut Vec<u8>;
        let n = size * nmemb;
        (*buf).extend_from_slice(std::slice::from_raw_parts(data as *const u8, n));
        n
    }
}

fn percent_encode_param(input: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(input.len());
    for &b in input {
        if b.is_ascii_alphanumeric() || b == b'-' || b == b'_' || b == b'.' || b == b'~' {
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
    if n < 10 {
        b'0' + n
    } else {
        b'A' + n - 10
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
    let start = s
        .iter()
        .position(|&b| b != b' ' && b != b'\t')
        .unwrap_or(s.len());
    let end = s
        .iter()
        .rposition(|&b| b != b' ' && b != b'\t')
        .map(|i| i + 1)
        .unwrap_or(0);
    if start >= end {
        &[]
    } else {
        &s[start..end]
    }
}

fn memchr_local(s: &[u8], c: u8) -> Option<usize> {
    s.iter().position(|&b| b == c)
}

/// Build URL-encoded query string from a Lua table at stack index `idx`.
unsafe fn build_query_string(state: *mut State, idx: i32) -> Vec<u8> {
    unsafe {
        let mut out: Vec<u8> = Vec::new();
        (*state).push_nil();
        let abs = if idx < 0 {
            (*state).get_top() + idx
        } else {
            idx
        };
        while lua_next(state, abs) != 0 {
            // Capture absolute positions before any lual_tolstring calls shift things
            let abs_key = (*state).get_top() - 1;
            let abs_val = (*state).get_top();
            if !out.is_empty() {
                out.push(b'&');
            }
            let mut klen = 0usize;
            let kptr = lual_tolstring(state, abs_key, &mut klen);
            let mut vlen = 0usize;
            let vptr = lual_tolstring(state, abs_val, &mut vlen);
            if !kptr.is_null() {
                out.extend_from_slice(&percent_encode_param(std::slice::from_raw_parts(
                    kptr as *const u8,
                    klen,
                )));
            }
            out.push(b'=');
            if !vptr.is_null() {
                out.extend_from_slice(&percent_encode_param(std::slice::from_raw_parts(
                    vptr as *const u8,
                    vlen,
                )));
            }
            lua_settop(state, abs_key); // restore to just key for next lua_next
        }
        out
    }
}

/// Build a curl_slist* from a Lua header table { name = value, ... }.
unsafe fn build_header_slist(state: *mut State, idx: i32) -> *mut std::ffi::c_void {
    unsafe {
        let mut list: *mut std::ffi::c_void = null_mut();
        (*state).push_nil();
        let abs = if idx < 0 {
            (*state).get_top() + idx
        } else {
            idx
        };
        while lua_next(state, abs) != 0 {
            let abs_key = (*state).get_top() - 1;
            let abs_val = (*state).get_top();
            let mut klen = 0usize;
            let mut vlen = 0usize;
            let kptr = lual_tolstring(state, abs_key, &mut klen);
            let vptr = lual_tolstring(state, abs_val, &mut vlen);
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
            lua_settop(state, abs_key); // restore to just key for next lua_next
        }
        list
    }
}

/// Build a cookie string "k=v; k2=v2" from a Lua table.
unsafe fn build_cookie_string(state: *mut State, idx: i32) -> Vec<u8> {
    unsafe {
        let mut out: Vec<u8> = Vec::new();
        (*state).push_nil();
        let abs = if idx < 0 {
            (*state).get_top() + idx
        } else {
            idx
        };
        while lua_next(state, abs) != 0 {
            let abs_key = (*state).get_top() - 1;
            let abs_val = (*state).get_top();
            if !out.is_empty() {
                out.extend_from_slice(b"; ");
            }
            let mut klen = 0usize;
            let mut vlen = 0usize;
            let kptr = lual_tolstring(state, abs_key, &mut klen);
            let vptr = lual_tolstring(state, abs_val, &mut vlen);
            if !kptr.is_null() && !vptr.is_null() {
                out.extend_from_slice(std::slice::from_raw_parts(kptr as *const u8, klen));
                out.push(b'=');
                out.extend_from_slice(std::slice::from_raw_parts(vptr as *const u8, vlen));
            }
            lua_settop(state, abs_key);
        }
        out
    }
}

/// Parse raw header bytes into a Lua table: { [lowercase_name] = value }
unsafe fn push_header_table(state: *mut State, raw: &[u8]) {
    unsafe {
        (*state).lua_createtable();
        for line in raw.split(|&b| b == b'\n') {
            let line = strip_crlf(line);
            if line.is_empty() || line.starts_with(b"HTTP/") {
                continue;
            }
            if let Some(colon) = memchr_local(line, b':') {
                let name_raw = trim_bytes(&line[..colon]);
                let val_raw = trim_bytes(&line[colon + 1..]);
                if name_raw.is_empty() {
                    continue;
                }
                let name: Vec<u8> = name_raw.iter().map(|b| b.to_ascii_lowercase()).collect();
                // If already present, comma-join
                lua_pushlstring(state, name.as_ptr() as *const i8, name.len());
                let ty = lua_rawget(state, -2);
                if ty == TagType::String {
                    let mut elen = 0usize;
                    let eptr = lua_tolstring(state, -1, &mut elen);
                    let mut combined = std::slice::from_raw_parts(eptr as *const u8, elen).to_vec();
                    combined.extend_from_slice(b", ");
                    combined.extend_from_slice(val_raw);
                    lua_settop(state, -2); // pop existing value
                    lua_pushlstring(state, name.as_ptr() as *const i8, name.len());
                    lua_pushlstring(state, combined.as_ptr() as *const i8, combined.len());
                } else {
                    lua_settop(state, -2); // pop nil
                    lua_pushlstring(state, name.as_ptr() as *const i8, name.len());
                    lua_pushlstring(state, val_raw.as_ptr() as *const i8, val_raw.len());
                }
                lua_rawset(state, -3);
            }
        }
    }
}

// ─── Core request function ────────────────────────────────────────────────────

// Extract options from a Lua table at stack index `opts_idx` (0 = no opts).
// Performs the HTTP request and pushes the response table on success.
// On error, pushes nil + error string.
// Returns number of values pushed.
unsafe fn do_request(
    state: *mut State,
    method: &[u8], // e.g. b"GET"
    url: &[u8],    // full URL
    opts_idx: i32, // stack index of opts table, or 0
) -> i32 {
    unsafe {
        let curl = curl_easy_init();
        if curl.is_null() {
            (*state).push_nil();
            lua_pushstring(state, c"curl_easy_init failed".as_ptr());
            return 2;
        }

        // ── URL + optional params ────────────────────────────────────────────

        let mut full_url = url.to_vec();

        if opts_idx != 0 && lua_type(state, opts_idx) == Some(TagType::Table) {
            // params
            lua_getfield(state, opts_idx, c"params".as_ptr());
            if lua_type(state, -1) == Some(TagType::Table) {
                let qs = build_query_string(state, -1);
                if !qs.is_empty() {
                    // Append ? or & depending on whether there's already a query
                    if full_url.contains(&b'?') {
                        full_url.push(b'&');
                    } else {
                        full_url.push(b'?');
                    }
                    full_url.extend_from_slice(&qs);
                }
            }
            lua_settop(state, -2); // pop params
        }

        full_url.push(0); // null-terminate
        curl_easy_setopt(
            curl,
            CURLOPT_URL,
            full_url.as_ptr() as *const std::ffi::c_char,
        );

        // ── User-Agent ───────────────────────────────────────────────────────

        curl_easy_setopt(curl, CURLOPT_USERAGENT, c"ferrigno-requests/1.0".as_ptr());

        // ── Write callbacks ──────────────────────────────────────────────────

        let mut body_buf: Vec<u8> = Vec::new();
        let mut hdr_buf: Vec<u8> = Vec::new();
        curl_easy_setopt(
            curl,
            CURLOPT_WRITEFUNCTION,
            write_cb as *mut std::ffi::c_void,
        );
        curl_easy_setopt(
            curl,
            CURLOPT_WRITEDATA,
            &mut body_buf as *mut Vec<u8> as *mut std::ffi::c_void,
        );
        curl_easy_setopt(
            curl,
            CURLOPT_HEADERFUNCTION,
            header_cb as *mut std::ffi::c_void,
        );
        curl_easy_setopt(
            curl,
            CURLOPT_HEADERDATA,
            &mut hdr_buf as *mut Vec<u8> as *mut std::ffi::c_void,
        );

        // ── Follow redirects (default on, opts.allow_redirects overrides) ────

        let mut follow: i64 = 1;

        // ── SSL verification (default on) ────────────────────────────────────

        let mut ssl_verify: i64 = 1;

        // ── Timeout (default: none) ───────────────────────────────────────────

        let mut timeout_secs: i64 = 0;

        // ── Options table ────────────────────────────────────────────────────

        let mut extra_headers: *mut std::ffi::c_void = null_mut();
        let mut userpwd_buf: Vec<u8> = Vec::new();
        let mut cookie_buf: Vec<u8> = Vec::new();
        let mut body_data: Option<Vec<u8>> = None;

        if opts_idx != 0 && lua_type(state, opts_idx) == Some(TagType::Table) {
            // headers
            lua_getfield(state, opts_idx, c"headers".as_ptr());
            if lua_type(state, -1) == Some(TagType::Table) {
                extra_headers = build_header_slist(state, -1);
            }
            lua_settop(state, -2);

            // data (raw string body or form-encoded table)
            lua_getfield(state, opts_idx, c"data".as_ptr());
            match lua_type(state, -1) {
                Some(TagType::String) => {
                    let mut dlen = 0usize;
                    let dptr = lua_tolstring(state, -1, &mut dlen);
                    body_data = Some(std::slice::from_raw_parts(dptr as *const u8, dlen).to_vec());
                }
                Some(TagType::Table) => {
                    // Form-encode the table
                    let qs = build_query_string(state, -1);
                    body_data = Some(qs);
                    // Add Content-Type if not already set
                    let ct = b"Content-Type: application/x-www-form-urlencoded\0";
                    extra_headers = curl_slist_append(extra_headers, ct.as_ptr() as *const i8);
                }
                _ => {}
            }
            lua_settop(state, -2);

            // timeout
            lua_getfield(state, opts_idx, c"timeout".as_ptr());
            if lua_type(state, -1) == Some(TagType::Numeric) {
                timeout_secs = lua_tointegerx(state, -1, null_mut());
                if timeout_secs == 0 {
                    // Could be a float
                    let f = lua_tonumberx(state, -1, null_mut());
                    timeout_secs = f.ceil() as i64;
                }
            }
            lua_settop(state, -2);

            // auth = {"user", "pass"}
            lua_getfield(state, opts_idx, c"auth".as_ptr());
            if lua_type(state, -1) == Some(TagType::Table) {
                lua_rawgeti(state, -1, 1); // user
                lua_rawgeti(state, -2, 2); // pass
                let mut ulen = 0usize;
                let mut plen = 0usize;
                let uptr = lua_tolstring(state, -2, &mut ulen);
                let pptr = lua_tolstring(state, -1, &mut plen);
                if !uptr.is_null() && !pptr.is_null() {
                    let u = std::slice::from_raw_parts(uptr as *const u8, ulen);
                    let p = std::slice::from_raw_parts(pptr as *const u8, plen);
                    userpwd_buf.extend_from_slice(u);
                    userpwd_buf.push(b':');
                    userpwd_buf.extend_from_slice(p);
                    userpwd_buf.push(0);
                }
                lua_settop(state, -3); // pop user and pass
            }
            lua_settop(state, -2); // pop auth

            // allow_redirects
            lua_getfield(state, opts_idx, c"allow_redirects".as_ptr());
            if lua_type(state, -1) == Some(TagType::Boolean) {
                follow = if lua_toboolean(state, -1) { 1 } else { 0 };
            }
            lua_settop(state, -2);

            // verify (SSL peer verification)
            lua_getfield(state, opts_idx, c"verify".as_ptr());
            if lua_type(state, -1) == Some(TagType::Boolean) {
                ssl_verify = if lua_toboolean(state, -1) { 1 } else { 0 };
            }
            lua_settop(state, -2);

            // cookies
            lua_getfield(state, opts_idx, c"cookies".as_ptr());
            if lua_type(state, -1) == Some(TagType::Table) {
                cookie_buf = build_cookie_string(state, -1);
                cookie_buf.push(0);
            }
            lua_settop(state, -2);
        }

        // ── Apply curl options ────────────────────────────────────────────────

        curl_easy_setopt(curl, CURLOPT_FOLLOWLOCATION, follow);
        curl_easy_setopt(curl, CURLOPT_SSL_VERIFYPEER, ssl_verify);
        curl_easy_setopt(
            curl,
            CURLOPT_SSL_VERIFYHOST,
            if ssl_verify != 0 { 2i64 } else { 0i64 },
        );

        if timeout_secs > 0 {
            curl_easy_setopt(curl, CURLOPT_TIMEOUT, timeout_secs);
            curl_easy_setopt(curl, CURLOPT_CONNECTTIMEOUT, timeout_secs);
        }

        if !userpwd_buf.is_empty() {
            curl_easy_setopt(
                curl,
                CURLOPT_USERPWD,
                userpwd_buf.as_ptr() as *const std::ffi::c_char,
            );
        }

        if !cookie_buf.is_empty() {
            curl_easy_setopt(
                curl,
                CURLOPT_COOKIE,
                cookie_buf.as_ptr() as *const std::ffi::c_char,
            );
        }

        if !extra_headers.is_null() {
            curl_easy_setopt(curl, CURLOPT_HTTPHEADER, extra_headers);
        }

        // ── Method + body ─────────────────────────────────────────────────────

        let method_upper: Vec<u8> = method.iter().map(|b| b.to_ascii_uppercase()).collect();
        match method_upper.as_slice() {
            b"GET" => {
                // Default — nothing extra needed
            }
            b"HEAD" => {
                curl_easy_setopt(curl, CURLOPT_NOBODY, 1i64);
            }
            b"POST" => {
                curl_easy_setopt(curl, CURLOPT_POST, 1i64);
                if let Some(ref data) = body_data {
                    curl_easy_setopt(
                        curl,
                        CURLOPT_POSTFIELDS,
                        data.as_ptr() as *const std::ffi::c_char,
                    );
                    curl_easy_setopt(curl, CURLOPT_POSTFIELDSIZE, data.len() as i64);
                } else {
                    curl_easy_setopt(curl, CURLOPT_POSTFIELDS, c"".as_ptr());
                    curl_easy_setopt(curl, CURLOPT_POSTFIELDSIZE, 0i64);
                }
            }
            _ => {
                // PUT, PATCH, DELETE, etc.
                let mut understudy = method_upper.clone();
                understudy.push(0);
                curl_easy_setopt(
                    curl,
                    CURLOPT_CUSTOMREQUEST,
                    understudy.as_ptr() as *const std::ffi::c_char,
                );
                if let Some(ref data) = body_data {
                    curl_easy_setopt(
                        curl,
                        CURLOPT_POSTFIELDS,
                        data.as_ptr() as *const std::ffi::c_char,
                    );
                    curl_easy_setopt(curl, CURLOPT_POSTFIELDSIZE, data.len() as i64);
                }
            }
        }

        // ── Perform ───────────────────────────────────────────────────────────

        let rc = curl_easy_perform(curl);

        if rc != 0 {
            if !extra_headers.is_null() {
                curl_slist_free_all(extra_headers);
            }
            curl_easy_cleanup(curl);
            (*state).push_nil();
            let err_ptr = curl_easy_strerror(rc);
            lua_pushstring(state, err_ptr);
            return 2;
        }

        // ── Get status code and effective URL ─────────────────────────────────

        let mut status_code: i64 = 0;
        curl_easy_getinfo(curl, CURLINFO_RESPONSE_CODE, &mut status_code as *mut i64);

        let mut effective_url_ptr: *const std::ffi::c_char = null();
        curl_easy_getinfo(
            curl,
            CURLINFO_EFFECTIVE_URL,
            &mut effective_url_ptr as *mut *const std::ffi::c_char,
        );

        // Capture effective URL before cleanup
        let effective_url: Vec<u8> = if !effective_url_ptr.is_null() {
            let len = cstr_len(effective_url_ptr);
            std::slice::from_raw_parts(effective_url_ptr as *const u8, len).to_vec()
        } else {
            url.to_vec()
        };

        if !extra_headers.is_null() {
            curl_slist_free_all(extra_headers);
        }
        curl_easy_cleanup(curl);

        // ── Build response table ───────────────────────────────────────────────

        push_response_table(state, status_code, &body_buf, &hdr_buf, &effective_url);

        1
    }
}

/// Push a response table: {status_code, ok, text, content, headers, url, json, raise_for_status}
unsafe fn push_response_table(state: *mut State, status: i64, body: &[u8], raw_headers: &[u8], url: &[u8]) {
    unsafe {
        (*state).lua_createtable();

        // status_code
        lua_pushlstring(state, b"status_code".as_ptr() as *const i8, 11);
        (*state).push_integer(status);
        lua_rawset(state, -3);

        // ok
        lua_pushlstring(state, b"ok".as_ptr() as *const i8, 2);
        (*state).push_boolean((200..400).contains(&status));
        lua_rawset(state, -3);

        // text / content (same bytes, Lua strings are binary-safe)
        lua_pushlstring(state, b"text".as_ptr() as *const i8, 4);
        lua_pushlstring(state, body.as_ptr() as *const i8, body.len());
        lua_rawset(state, -3);

        lua_pushlstring(state, b"content".as_ptr() as *const i8, 7);
        lua_pushlstring(state, body.as_ptr() as *const i8, body.len());
        lua_rawset(state, -3);

        // headers
        lua_pushlstring(state, b"headers".as_ptr() as *const i8, 7);
        push_header_table(state, raw_headers);
        lua_rawset(state, -3);

        // url
        lua_pushlstring(state, b"url".as_ptr() as *const i8, 3);
        lua_pushlstring(state, url.as_ptr() as *const i8, url.len());
        lua_rawset(state, -3);

        // json() method
        lua_pushlstring(state, b"json".as_ptr() as *const i8, 4);
        lua_pushcclosure(
            state,
            Some(response_json as unsafe fn(*mut State) -> i32),
            0,
        );
        lua_rawset(state, -3);

        // raise_for_status() method
        lua_pushlstring(state, b"raise_for_status".as_ptr() as *const i8, 16);
        lua_pushcclosure(
            state,
            Some(response_raise_for_status as unsafe fn(*mut State) -> i32),
            0,
        );
        lua_rawset(state, -3);
    }
}

// ─── Response methods ─────────────────────────────────────────────────────────

/// response:json() — decode response.text as JSON
pub unsafe fn response_json(state: *mut State) -> i32 {
    unsafe {
        lua_getfield(state, 1, c"text".as_ptr());
        if lua_type(state, -1) != Some(TagType::String) {
            lua_settop(state, -2);
            return lual_error(state, c"response:json(): no text field".as_ptr(), &[]);
        }
        let mut slen = 0usize;
        let sptr = lua_tolstring(state, -1, &mut slen);
        // Copy bytes before popping
        let input: Vec<u8> = std::slice::from_raw_parts(sptr as *const u8, slen).to_vec();
        lua_settop(state, -2); // pop text

        let saved_top = (*state).get_top();
        let mut parser = JsonParser::new(&input);

        match parser.parse_value(state) {
            Ok(()) => 1,
            Err(msg) => {
                lua_settop(state, saved_top);
                let full = format!("response:json(): {}\0", msg);
                lual_error(state, full.as_ptr() as *const i8, &[])
            }
        }
    }
}

/// response:raise_for_status() — errors if status_code is not 2xx/3xx
pub unsafe fn response_raise_for_status(state: *mut State) -> i32 {
    unsafe {
        lua_getfield(state, 1, c"status_code".as_ptr());
        let code = lua_tointegerx(state, -1, null_mut());
        lua_settop(state, -2);
        if !(200..400).contains(&code) {
            let msg = format!("HTTP Error: {}\0", code);
            lual_error(state, msg.as_ptr() as *const i8, &[])
        } else {
            lua_pushvalue(state, 1); // return self for chaining
            1
        }
    }
}

// ─── Lua-callable request functions ──────────────────────────────────────────

/// requests.request(method, url [, opts]) → response | nil, errmsg
pub unsafe fn requests_request(state: *mut State) -> i32 {
    unsafe {
        let mut mlen = 0usize;
        let mptr = lual_checklstring(state, 1, &mut mlen);
        let method = std::slice::from_raw_parts(mptr as *const u8, mlen);

        let mut ulen = 0usize;
        let uptr = lual_checklstring(state, 2, &mut ulen);
        let url = std::slice::from_raw_parts(uptr as *const u8, ulen);

        let opts_idx = if lua_type(state, 3) == Some(TagType::Table) {
            3
        } else {
            0
        };

        do_request(state, method, url, opts_idx)
    }
}

/// requests.get(url [, opts]) → response | nil, errmsg
pub unsafe fn requests_get(state: *mut State) -> i32 {
    unsafe {
        let mut ulen = 0usize;
        let uptr = lual_checklstring(state, 1, &mut ulen);
        let url = std::slice::from_raw_parts(uptr as *const u8, ulen);
        let opts_idx = if lua_type(state, 2) == Some(TagType::Table) {
            2
        } else {
            0
        };
        do_request(state, b"GET", url, opts_idx)
    }
}

/// requests.post(url [, opts]) → response | nil, errmsg
pub unsafe fn requests_post(state: *mut State) -> i32 {
    unsafe {
        let mut ulen = 0usize;
        let uptr = lual_checklstring(state, 1, &mut ulen);
        let url = std::slice::from_raw_parts(uptr as *const u8, ulen);
        let opts_idx = if lua_type(state, 2) == Some(TagType::Table) {
            2
        } else {
            0
        };
        do_request(state, b"POST", url, opts_idx)
    }
}

/// requests.put(url [, opts]) → response | nil, errmsg
pub unsafe fn requests_put(state: *mut State) -> i32 {
    unsafe {
        let mut ulen = 0usize;
        let uptr = lual_checklstring(state, 1, &mut ulen);
        let url = std::slice::from_raw_parts(uptr as *const u8, ulen);
        let opts_idx = if lua_type(state, 2) == Some(TagType::Table) {
            2
        } else {
            0
        };
        do_request(state, b"PUT", url, opts_idx)
    }
}

/// requests.delete(url [, opts]) → response | nil, errmsg
pub unsafe fn requests_delete(state: *mut State) -> i32 {
    unsafe {
        let mut ulen = 0usize;
        let uptr = lual_checklstring(state, 1, &mut ulen);
        let url = std::slice::from_raw_parts(uptr as *const u8, ulen);
        let opts_idx = if lua_type(state, 2) == Some(TagType::Table) {
            2
        } else {
            0
        };
        do_request(state, b"DELETE", url, opts_idx)
    }
}

/// requests.patch(url [, opts]) → response | nil, errmsg
pub unsafe fn requests_patch(state: *mut State) -> i32 {
    unsafe {
        let mut ulen = 0usize;
        let uptr = lual_checklstring(state, 1, &mut ulen);
        let url = std::slice::from_raw_parts(uptr as *const u8, ulen);
        let opts_idx = if lua_type(state, 2) == Some(TagType::Table) {
            2
        } else {
            0
        };
        do_request(state, b"PATCH", url, opts_idx)
    }
}

/// requests.head(url [, opts]) → response | nil, errmsg
pub unsafe fn requests_head(state: *mut State) -> i32 {
    unsafe {
        let mut ulen = 0usize;
        let uptr = lual_checklstring(state, 1, &mut ulen);
        let url = std::slice::from_raw_parts(uptr as *const u8, ulen);
        let opts_idx = if lua_type(state, 2) == Some(TagType::Table) {
            2
        } else {
            0
        };
        do_request(state, b"HEAD", url, opts_idx)
    }
}

// ─── Registration ─────────────────────────────────────────────────────────────

pub const REQUESTS_FUNCTIONS: [RegisteredFunction; 7] = [
    RegisteredFunction {
        registeredfunction_name: c"request".as_ptr(),
        registeredfunction_function: Some(requests_request as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"get".as_ptr(),
        registeredfunction_function: Some(requests_get as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"post".as_ptr(),
        registeredfunction_function: Some(requests_post as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"put".as_ptr(),
        registeredfunction_function: Some(requests_put as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"delete".as_ptr(),
        registeredfunction_function: Some(requests_delete as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"patch".as_ptr(),
        registeredfunction_function: Some(requests_patch as unsafe fn(*mut State) -> i32),
    },
    RegisteredFunction {
        registeredfunction_name: c"head".as_ptr(),
        registeredfunction_function: Some(requests_head as unsafe fn(*mut State) -> i32),
    },
];

pub unsafe fn luaopen_requests(state: *mut State) -> i32 {
    unsafe {
        // Register the json.null sentinel so response:json() can return it
        let _ = json_null_ptr(); // ensure sentinel is initialized

        (*state).lua_createtable();
        lual_setfuncs(
            state,
            REQUESTS_FUNCTIONS.as_ptr(),
            REQUESTS_FUNCTIONS.len(),
            0,
        );
        1
    }
}
