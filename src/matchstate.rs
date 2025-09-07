use crate::interpreter::*;
use crate::buffer::*;
use crate::tag::*;
use crate::character::*;
use crate::utility::c::*;
use libc::{tolower,};
pub const MAX_CAPTURES: usize = 32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MatchStateCapture {
    pub init: *const i8,
    pub length: i64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MatchState {
    pub src_init: *const i8,
    pub src_end: *const i8,
    pub p_end: *const i8,
    pub interpreter: *mut Interpreter,
    pub matchdepth: i32,
    pub level: usize,
    pub capture: [MatchStateCapture; MAX_CAPTURES],
}
impl MatchState {
    pub unsafe extern "C" fn add_value(
        & mut self,
        b: *mut Buffer,
        s: *const i8,
        e: *const i8,
        tr: u8,
    ) -> i32 {
        unsafe {
            let state: *mut Interpreter = self.interpreter;
            match tr {
                TAG_TYPE_CLOSURE => {
                    let n: i32;
                    lua_pushvalue(state, 3);
                    n = self.push_captures(s, e);
                    lua_callk(state, n, 1, 0, None);
                }
                TAG_TYPE_TABLE => {
                    self.push_onecapture(0, s, e);
                    lua_gettable(state, 3);
                }
                _ => {
                    self.add_s(b, s, e);
                    return 1;
                }
            }
            if lua_toboolean(state, -1) == 0 {
                lua_settop(state, -1 - 1);
                (*b).add_string_with_length(s, e.offset_from(s) as usize);
                return 0;
            } else if ((!lua_isstring(state, -1)) as i32 != 0) as i64 != 0 {
                return lual_error(
                    state,
                    b"invalid replacement value (a %s)\0" as *const u8 as *const i8,
                    lua_typename(state, lua_type(state, -1)),
                );
            } else {
                (*b).add_value();
                return 1;
            };
        }
    }
    pub unsafe extern "C" fn push_captures(& mut self, s: *const i8, e: *const i8) -> i32 {
        unsafe {
            let nlevels: i32 = if self.level as i32 == 0 && !s.is_null() {
                1
            } else {
                self.level as i32
            };
            lual_checkstack(
                self.interpreter,
                nlevels,
                b"too many captures\0" as *const u8 as *const i8,
            );
            for i in 0..nlevels {
                self.push_onecapture(i, s, e);
            }
            return nlevels;
        }
    }
    pub unsafe extern "C" fn push_onecapture(& mut self, i: i32, s: *const i8, e: *const i8) {
        unsafe {
            let mut cap: *const i8 = std::ptr::null();
            let level: i64 = self.get_onecapture(i, s, e, &mut cap) as i64;
            if level != -2 as i64 {
                lua_pushlstring(self.interpreter, cap, level as u64);
            }
        }
    }
    pub unsafe extern "C" fn get_onecapture(& mut self,
        i: i32,
        s: *const i8,
        e: *const i8,
        cap: *mut *const i8,
    ) -> u64 {
        unsafe {
            if i >= self.level as i32 {
                if ((i != 0) as i32 != 0) as i64 != 0 {
                    lual_error(
                        self.interpreter,
                        b"invalid capture index %%%d\0" as *const u8 as *const i8,
                        i + 1,
                    );
                }
                *cap = s;
                return e.offset_from(s) as u64;
            } else {
                let capl: i64 = self.capture[i as usize].length;
                *cap = self.capture[i as usize].init;
                if ((capl == -1 as i64) as i32 != 0) as i64 != 0 {
                    lual_error(
                        self.interpreter,
                        b"unfinished capture\0" as *const u8 as *const i8,
                    );
                } else if capl == -2 as i64 {
                    (*(self.interpreter)).push_integer(
                        ((self.capture[i as usize].init).offset_from(self.src_init) as i64 + 1)
                            as i64,
                    );
                }
                return capl as u64;
            };
        }
    }
    pub unsafe extern "C" fn check_capture(&mut self, mut l: i32) -> i32 {
        unsafe {
            l -= CHARACTER_1 as i32;
            if ((l < 0 || l >= self.level as i32 || self.capture[l as usize].length == -1 as i64)
                as i32
                != 0) as i64
                != 0
            {
                return lual_error(
                    self.interpreter,
                    b"invalid capture index %%%d\0" as *const u8 as *const i8,
                    l + 1,
                );
            }
            return l;
        }
    }
    pub unsafe extern "C" fn capture_to_close(& mut self) -> i32 {
        unsafe {
            let mut level: i32 = self.level as i32;
            level -= 1;
            while level >= 0 {
                if self.capture[level as usize].length == -1 as i64 {
                    return level;
                }
                level -= 1;
            }
            return lual_error(
                self.interpreter,
                b"invalid pattern capture\0" as *const u8 as *const i8,
            );
        }
    }
    pub unsafe extern "C" fn classend(& mut self, mut p: *const i8) -> *const i8 {
        unsafe {
            let fresh160 = p;
            p = p.offset(1);
            match *fresh160 as i32 {
                37 => {
                    if ((p == self.p_end) as i32 != 0) as i64 != 0 {
                        lual_error(
                            self.interpreter,
                            b"malformed pattern (ends with '%%')\0" as *const u8 as *const i8,
                        );
                    }
                    return p.offset(1 as isize);
                }
                91 => {
                    if *p as i32 == CHARACTER_CARET as i32 {
                        p = p.offset(1);
                    }
                    loop {
                        if ((p == self.p_end) as i32 != 0) as i64 != 0 {
                            lual_error(
                                self.interpreter,
                                b"malformed pattern (missing CHARACTER_BRACKET_RIGHT)\0" as *const u8 as *const i8,
                            );
                        }
                        let fresh161 = p;
                        p = p.offset(1);
                        if *fresh161 as i32 == CHARACTER_PERCENT as i32 && p < self.p_end {
                            p = p.offset(1);
                        }
                        if !(*p as i32 != CHARACTER_BRACKET_RIGHT as i32) {
                            break;
                        }
                    }
                    return p.offset(1 as isize);
                }
                _ => return p,
            };
        }
    }
    pub unsafe extern "C" fn singlematch(
        & mut self,
        s: *const i8,
        p: *const i8,
        ep: *const i8,
    ) -> i32 {
        unsafe {
            if s >= self.src_end {
                return 0;
            } else {
                let c: i32 = *s as u8 as i32;
                match *p as i32 {
                    46 => return 1,
                    37 => {
                        return match_class(c, *p.offset(1 as isize) as u8 as i32);
                    }
                    91 => return matchbracketclass(c, p, ep.offset(-(1 as isize))),
                    _ => return (*p as u8 as i32 == c) as i32,
                }
            };
        }
    }
    pub unsafe extern "C" fn matchbalance(
        & mut self,
        mut s: *const i8,
        p: *const i8,
    ) -> *const i8 {
        unsafe {
            if ((p >= (self.p_end).offset(-(1 as isize))) as i32 != 0) as i64 != 0 {
                lual_error(
                    self.interpreter,
                    b"malformed pattern (missing arguments to '%%b')\0" as *const u8 as *const i8,
                );
            }
            if *s as i32 != *p as i32 {
                return std::ptr::null();
            } else {
                let b: i32 = *p as i32;
                let e: i32 = *p.offset(1 as isize) as i32;
                let mut cont: i32 = 1;
                loop {
                    s = s.offset(1);
                    if !(s < self.src_end) {
                        break;
                    }
                    if *s as i32 == e {
                        cont -= 1;
                        if cont == 0 {
                            return s.offset(1 as isize);
                        }
                    } else if *s as i32 == b {
                        cont += 1;
                    }
                }
            }
            return std::ptr::null();
        }
    }
    pub unsafe extern "C" fn max_expand(
        & mut self,
        s: *const i8,
        p: *const i8,
        ep: *const i8,
    ) -> *const i8 {
        unsafe {
            let mut i: i64 = 0;
            while self.singlematch(s.offset(i as isize), p, ep) != 0 {
                i += 1;
            }
            while i >= 0 {
                let res: *const i8 = self.match_0(s.offset(i as isize), ep.offset(1 as isize));
                if !res.is_null() {
                    return res;
                }
                i -= 1;
            }
            return std::ptr::null();
        }
    }
    pub unsafe extern "C" fn min_expand(
        & mut self,
        mut s: *const i8,
        p: *const i8,
        ep: *const i8,
    ) -> *const i8 {
        unsafe {
            loop {
                let res: *const i8 = self.match_0(s, ep.offset(1 as isize));
                if !res.is_null() {
                    return res;
                } else if self.singlematch(s, p, ep) != 0 {
                    s = s.offset(1);
                } else {
                    return std::ptr::null();
                }
            }
        }
    }
    pub unsafe extern "C" fn start_capture(
        & mut self,
        s: *const i8,
        p: *const i8,
        what: i32,
    ) -> *const i8 {
        unsafe {
            let res: *const i8;
            let level: usize = self.level;
            if level >= MAX_CAPTURES {
                lual_error(
                    self.interpreter,
                    b"too many captures\0" as *const u8 as *const i8,
                );
            }
            self.capture[level].init = s;
            self.capture[level].length = what as i64;
            self.level = level + 1;
            res = self.match_0(s, p);
            if res.is_null() {
                self.level = (self.level).wrapping_sub(1);
                self.level;
            }
            return res;
        }
    }
    pub unsafe extern "C" fn end_capture(& mut self, s: *const i8, p: *const i8) -> *const i8 {
        unsafe {
            let l: i32 = self.capture_to_close();
            let res: *const i8;
            self.capture[l as usize].length = s.offset_from(self.capture[l as usize].init) as i64;
            res = self.match_0(s, p);
            if res.is_null() {
                self.capture[l as usize].length = -1 as i64;
            }
            return res;
        }
    }
    pub unsafe extern "C" fn match_capture(& mut self, s: *const i8, mut l: i32) -> *const i8 {
        unsafe {
            let length: u64;
            l = self.check_capture(l);
            length = self.capture[l as usize].length as u64;
            if (self.src_end).offset_from(s) as u64 >= length
                && memcmp(
                    self.capture[l as usize].init as *const libc::c_void,
                    s as *const libc::c_void,
                    length,
                ) == 0
            {
                return s.offset(length as isize);
            } else {
                return std::ptr::null();
            };
        }
    }
    pub unsafe extern "C" fn match_0(
        & mut self,
        mut s: *const i8,
        mut p: *const i8,
    ) -> *const i8 {
        unsafe {
            let mut ep_0: *const i8 = std::ptr::null();
            let mut current_block: u64;
            let fresh162 = self.matchdepth;
            self.matchdepth = self.matchdepth - 1;
            if ((fresh162 == 0) as i32 != 0) as i64 != 0 {
                lual_error(
                    self.interpreter,
                    b"pattern too complex\0" as *const u8 as *const i8,
                );
            }
            loop {
                if !(p != self.p_end) {
                    current_block = 6476622998065200121;
                    break;
                }
                match *p as i32 {
                    CHARACTER_PARENTHESIS_LEFT => {
                        if *p.offset(1 as isize) as i32 == CHARACTER_PARENTHESIS_RIGHT as i32 {
                            s = self.start_capture(s, p.offset(2 as isize), -2);
                        } else {
                            s = self.start_capture(s, p.offset(1 as isize), -1);
                        }
                        current_block = 6476622998065200121;
                        break;
                    }
                    CHARACTER_PARENTHESIS_RIGHT => {
                        s = self.end_capture(s, p.offset(1 as isize));
                        current_block = 6476622998065200121;
                        break;
                    }
                    CHARACTER_DOLLAR => {
                        if !(p.offset(1 as isize) != self.p_end) {
                            s = if s == self.src_end {
                                s
                            } else {
                                std::ptr::null()
                            };
                            current_block = 6476622998065200121;
                            break;
                        }
                    }
                    CHARACTER_PERCENT => match *p.offset(1 as isize) as i32 {
                        CHARACTER_LOWER_B => {
                            current_block = 17965632435239708295;
                            match current_block {
                                17965632435239708295 => {
                                    s = self.matchbalance(s, p.offset(2 as isize));
                                    if s.is_null() {
                                        current_block = 6476622998065200121;
                                        break;
                                    }
                                    p = p.offset(4 as isize);
                                    continue;
                                }
                                8236137900636309791 => {
                                    let ep: *const i8;
                                    let previous: i8;
                                    p = p.offset(2 as isize);
                                    if ((*p as i32 != CHARACTER_BRACKET_LEFT as i32) as i32 != 0) as i64 != 0 {
                                        lual_error(
                                            self.interpreter,
                                            b"missing CHARACTER_BRACKET_LEFT after '%%f' in pattern\0" as *const u8
                                                as *const i8,
                                        );
                                    }
                                    ep = self.classend(p);
                                    previous = (if s == self.src_init {
                                        CHARACTER_NUL as i32
                                    } else {
                                        *s.offset(-(1 as isize)) as i32
                                    }) as i8;
                                    if matchbracketclass(
                                        previous as u8 as i32,
                                        p,
                                        ep.offset(-(1 as isize)),
                                    ) == 0
                                        && matchbracketclass(
                                            *s as u8 as i32,
                                            p,
                                            ep.offset(-(1 as isize)),
                                        ) != 0
                                    {
                                        p = ep;
                                        continue;
                                    } else {
                                        s = std::ptr::null();
                                        current_block = 6476622998065200121;
                                        break;
                                    }
                                }
                                _ => {
                                    s = self.match_capture(s, *p.offset(1 as isize) as u8 as i32);
                                    if s.is_null() {
                                        current_block = 6476622998065200121;
                                        break;
                                    }
                                    p = p.offset(2 as isize);
                                    continue;
                                }
                            }
                        }
                        CHARACTER_LOWER_F => {
                            current_block = 8236137900636309791;
                            match current_block {
                                17965632435239708295 => {
                                    s = self.matchbalance(s, p.offset(2 as isize));
                                    if s.is_null() {
                                        current_block = 6476622998065200121;
                                        break;
                                    }
                                    p = p.offset(4 as isize);
                                    continue;
                                }
                                8236137900636309791 => {
                                    let ep: *const i8;
                                    let previous: i8;
                                    p = p.offset(2 as isize);
                                    if ((*p as i32 != CHARACTER_BRACKET_LEFT as i32) as i32 != 0) as i64 != 0 {
                                        lual_error(
                                            self.interpreter,
                                            b"missing CHARACTER_BRACKET_LEFT after '%%f' in pattern\0" as *const u8
                                                as *const i8,
                                        );
                                    }
                                    ep = self.classend(p);
                                    previous = (if s == self.src_init {
                                        CHARACTER_NUL as i32
                                    } else {
                                        *s.offset(-(1 as isize)) as i32
                                    }) as i8;
                                    if matchbracketclass(
                                        previous as u8 as i32,
                                        p,
                                        ep.offset(-(1 as isize)),
                                    ) == 0
                                        && matchbracketclass(
                                            *s as u8 as i32,
                                            p,
                                            ep.offset(-(1 as isize)),
                                        ) != 0
                                    {
                                        p = ep;
                                        continue;
                                    } else {
                                        s = std::ptr::null();
                                        current_block = 6476622998065200121;
                                        break;
                                    }
                                }
                                _ => {
                                    s = self.match_capture(s, *p.offset(1 as isize) as u8 as i32);
                                    if s.is_null() {
                                        current_block = 6476622998065200121;
                                        break;
                                    }
                                    p = p.offset(2 as isize);
                                    continue;
                                }
                            }
                        }
                        CHARACTER_0 | CHARACTER_1 | CHARACTER_2 | CHARACTER_3 | CHARACTER_4 | CHARACTER_5 | CHARACTER_6 | CHARACTER_7 | CHARACTER_8 | CHARACTER_9 => {
                            current_block = 14576567515993809846;
                            match current_block {
                                17965632435239708295 => {
                                    s = self.matchbalance(s, p.offset(2 as isize));
                                    if s.is_null() {
                                        current_block = 6476622998065200121;
                                        break;
                                    }
                                    p = p.offset(4 as isize);
                                    continue;
                                }
                                8236137900636309791 => {
                                    let ep: *const i8;
                                    let previous: i8;
                                    p = p.offset(2 as isize);
                                    if ((*p as i32 != CHARACTER_BRACKET_LEFT as i32) as i32 != 0) as i64 != 0 {
                                        lual_error(
                                            self.interpreter,
                                            b"missing CHARACTER_BRACKET_LEFT after '%%f' in pattern\0" as *const u8
                                                as *const i8,
                                        );
                                    }
                                    ep = self.classend(p);
                                    previous = (if s == self.src_init {
                                        CHARACTER_NUL as i32
                                    } else {
                                        *s.offset(-(1 as isize)) as i32
                                    }) as i8;
                                    if matchbracketclass(
                                        previous as u8 as i32,
                                        p,
                                        ep.offset(-(1 as isize)),
                                    ) == 0
                                        && matchbracketclass(
                                            *s as u8 as i32,
                                            p,
                                            ep.offset(-(1 as isize)),
                                        ) != 0
                                    {
                                        p = ep;
                                        continue;
                                    } else {
                                        s = std::ptr::null();
                                        current_block = 6476622998065200121;
                                        break;
                                    }
                                }
                                _ => {
                                    s = self.match_capture(s, *p.offset(1 as isize) as u8 as i32);
                                    if s.is_null() {
                                        current_block = 6476622998065200121;
                                        break;
                                    }
                                    p = p.offset(2 as isize);
                                    continue;
                                }
                            }
                        }
                        _ => {}
                    },
                    _ => {}
                }
                ep_0 = self.classend(p);
                if self.singlematch(s, p, ep_0) == 0 {
                    if *ep_0 as i32 == CHARACTER_ASTERISK as i32
                        || *ep_0 as i32 == CHARACTER_QUESTION as i32
                        || *ep_0 as i32 == CHARACTER_HYPHEN as i32
                    {
                        p = ep_0.offset(1 as isize);
                    } else {
                        s = std::ptr::null();
                        current_block = 6476622998065200121;
                        break;
                    }
                } else {
                    match *ep_0 as i32 {
                        63 => {
                            let res: *const i8;
                            res = self.match_0(s.offset(1 as isize), ep_0.offset(1 as isize));
                            if !res.is_null() {
                                s = res;
                                current_block = 6476622998065200121;
                                break;
                            } else {
                                p = ep_0.offset(1 as isize);
                            }
                        }
                        43 => {
                            s = s.offset(1);
                            current_block = 13376797365003376294;
                            break;
                        }
                        42 => {
                            current_block = 13376797365003376294;
                            break;
                        }
                        45 => {
                            s = self.min_expand(s, p, ep_0);
                            current_block = 6476622998065200121;
                            break;
                        }
                        _ => {
                            s = s.offset(1);
                            p = ep_0;
                        }
                    }
                }
            }
            match current_block {
                13376797365003376294 => {
                    s = self.max_expand(s, p, ep_0);
                }
                _ => {}
            }
            self.matchdepth += 1;
            self.matchdepth;
            return s;
        }
    }
    pub unsafe extern "C" fn prepstate(
        & mut self,
        state: *mut Interpreter,
        s: *const i8,
        lexical_state: u64,
        p: *const i8,
        lp: u64,
    ) {
        self.interpreter = state;
        self.matchdepth = 200 as i32;
        self.src_init = s;
        unsafe {
            self.src_end = s.offset(lexical_state as isize);
            self.p_end = p.offset(lp as isize);
        }
    }
    pub fn reprepstate(& mut self) {
        self.level = 0;
    }
    pub unsafe extern "C" fn add_s(& mut self, b: *mut Buffer, s: *const i8, e: *const i8) {
        unsafe {
            let mut l: u64 = 0;
            let state: *mut Interpreter = self.interpreter;
            let mut news: *const i8 = lua_tolstring(state, 3, &mut l);
            let mut p: *const i8;
            loop {
                p = memchr(news as *const libc::c_void, CHARACTER_PERCENT as i32, l) as *mut i8;
                if p.is_null() {
                    break;
                }
                (*b).add_string_with_length(news, p.offset_from(news) as usize);
                p = p.offset(1);
                if *p as i32 == CHARACTER_PERCENT as i32 {
                    ((*b).length < (*b).size || !((*b).prepare_with_size(1)).is_null()) as i32;
                    let fresh164 = (*b).length;
                    (*b).length = ((*b).length).wrapping_add(1);
                    *((*b).pointer).offset(fresh164 as isize) = *p;
                } else if *p as i32 == CHARACTER_0 as i32 {
                    (*b).add_string_with_length(s, e.offset_from(s) as usize);
                } else if *(*__ctype_b_loc()).offset(*p as u8 as isize) as i32
                    & _ISDIGIT as i32
                    != 0
                {
                    let mut cap: *const i8 = std::ptr::null();
                    let resl: i64 = self.get_onecapture(*p as i32 - CHARACTER_1 as i32, s, e, &mut cap) as i64;
                    if resl == -2 as i64 {
                        (*b).add_value();
                    } else {
                        (*b).add_string_with_length(cap, resl as usize);
                    }
                } else {
                    lual_error(
                        state,
                        b"invalid use of '%c' in replacement string\0" as *const u8 as *const i8,
                        CHARACTER_PERCENT as i32,
                    );
                }
                l = (l as u64).wrapping_sub(p.offset(1 as isize).offset_from(news) as u64) as u64
                    as u64;
                news = p.offset(1 as isize);
            }
            (*b).add_string_with_length(news, l as usize);
        }
    }
}
pub unsafe extern "C" fn match_class(c: i32, cl: i32) -> i32 {
    unsafe {
        let res: i32;
        match tolower(cl) {
            97 => {
                res = *(*__ctype_b_loc()).offset(c as isize) as i32 & _ISALPHA as i32;
            }
            99 => {
                res =
                    *(*__ctype_b_loc()).offset(c as isize) as i32 & _ISCONTROL as i32;
            }
            100 => {
                res = *(*__ctype_b_loc()).offset(c as isize) as i32 & _ISDIGIT as i32;
            }
            103 => {
                res = *(*__ctype_b_loc()).offset(c as isize) as i32 & _ISGRAPH as i32;
            }
            108 => {
                res = *(*__ctype_b_loc()).offset(c as isize) as i32 & _ISLOWER as i32;
            }
            112 => {
                res = *(*__ctype_b_loc()).offset(c as isize) as i32
                    & _ISPUNCTUATION as i32;
            }
            115 => {
                res = *(*__ctype_b_loc()).offset(c as isize) as i32 & _ISSPACE as i32;
            }
            117 => {
                res = *(*__ctype_b_loc()).offset(c as isize) as i32 & _ISUPPER as i32;
            }
            119 => {
                res = *(*__ctype_b_loc()).offset(c as isize) as i32
                    & _ISALPHANUMERIC as i32;
            }
            120 => {
                res =
                    *(*__ctype_b_loc()).offset(c as isize) as i32 & _ISXDIGIT as i32;
            }
            122 => {
                res = (c == 0) as i32;
            }
            _ => return (cl == c) as i32,
        }
        return if *(*__ctype_b_loc()).offset(cl as isize) as i32 & _ISLOWER as i32
            != 0
        {
            res
        } else {
            (res == 0) as i32
        };
    }
}
pub unsafe extern "C" fn matchbracketclass(c: i32, mut p: *const i8, ec: *const i8) -> i32 {
    unsafe {
        let mut sig: i32 = 1;
        if *p.offset(1 as isize) as i32 == CHARACTER_CARET as i32 {
            sig = 0;
            p = p.offset(1);
        }
        loop {
            p = p.offset(1);
            if !(p < ec) {
                break;
            }
            if *p as i32 == CHARACTER_PERCENT as i32 {
                p = p.offset(1);
                if match_class(c, *p as u8 as i32) != 0 {
                    return sig;
                }
            } else if *p.offset(1 as isize) as i32 == CHARACTER_HYPHEN as i32 && p.offset(2 as isize) < ec {
                p = p.offset(2 as isize);
                if *p.offset(-(2 as isize)) as u8 as i32 <= c && c <= *p as u8 as i32 {
                    return sig;
                }
            } else if *p as u8 as i32 == c {
                return sig;
            }
        }
        return (sig == 0) as i32;
    }
}
