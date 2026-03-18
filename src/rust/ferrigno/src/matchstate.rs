use crate::buffer::*;
use crate::character::*;
use crate::functionstate::LUAI_MAXCCALLS;
use crate::state::*;
use crate::tagtype::*;
use crate::utility::*;
use std::ptr::*;
pub const MAX_CAPTURES: usize = 32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MatchStateCapture {
    pub matchstatecapture_initial: *const i8,
    pub matchstatecapture_length: i64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MatchState {
    pub src_init: *const i8,
    pub src_end: *const i8,
    pub p_end: *const i8,
    pub matchstate_interpreter: *mut State,
    pub matchdepth: i32,
    pub level: usize,
    pub capture: [MatchStateCapture; MAX_CAPTURES],
}
impl MatchState {
    pub unsafe fn add_value(&mut self, b: *mut Buffer, s: *const i8, e: *const i8, tr: TagType) -> i32 {
        unsafe {
            let state: *mut State = self.matchstate_interpreter;
            match tr {
                | TagType::Closure => {
                    lua_pushvalue(state, 3);
                    let n: i32 = self.push_captures(s, e);
                    (*state).lua_callk(n, 1, 0, None);
                },
                | TagType::Table => {
                    self.push_onecapture(0, s, e);
                    lua_gettable(state, 3);
                },
                | _ => {
                    self.add_s(b, s, e);
                    return 1;
                },
            }
            if !lua_toboolean(state, -1) {
                lua_settop(state, -2);
                (*b).add_string_with_length(s, e.offset_from(s) as usize);
                0
            } else if !lua_isstring(state, -1) {
                lual_error(
                    state,
                    c"invalid replacement value (a %s)".as_ptr(),
                    &[lua_typename(state, lua_type(state, -1)).into()],
                )
            } else {
                (*b).add_value();
                1
            }
        }
    }
    pub unsafe fn push_captures(&mut self, s: *const i8, e: *const i8) -> i32 {
        unsafe {
            let nlevels: i32 = if self.level as i32 == 0 && !s.is_null() { 1 } else { self.level as i32 };
            lual_checkstack(self.matchstate_interpreter, nlevels, c"too many captures".as_ptr());
            for i in 0..nlevels {
                self.push_onecapture(i, s, e);
            }
            nlevels
        }
    }
    pub unsafe fn push_onecapture(&mut self, i: i32, s: *const i8, e: *const i8) {
        unsafe {
            let mut cap: *const i8 = null();
            let level: i64 = self.get_onecapture(i, s, e, &mut cap) as i64;
            if level != -2_i64 {
                lua_pushlstring(self.matchstate_interpreter, cap, level as usize);
            }
        }
    }
    pub unsafe fn get_onecapture(&mut self, i: i32, s: *const i8, e: *const i8, cap: *mut *const i8) -> usize {
        unsafe {
            if i >= self.level as i32 {
                if i != 0 {
                    lual_error(self.matchstate_interpreter, c"invalid capture index %%%d".as_ptr(), &[(i + 1).into()]);
                }
                *cap = s;
                e.offset_from(s) as usize
            } else {
                let capl: i64 = self.capture[i as usize].matchstatecapture_length;
                *cap = self.capture[i as usize].matchstatecapture_initial;
                if capl == -1 {
                    lual_error(self.matchstate_interpreter, c"unfinished capture".as_ptr(), &[]);
                } else if capl == -2_i64 {
                    (*(self.matchstate_interpreter))
                        .push_integer((self.capture[i as usize].matchstatecapture_initial).offset_from(self.src_init) as i64 + 1);
                }
                capl as usize
            }
        }
    }
    pub unsafe fn check_capture(&mut self, mut l: i32) -> i32 {
        unsafe {
            l -= Character::Digit1 as i32;
            if l < 0 || l >= self.level as i32 || self.capture[l as usize].matchstatecapture_length == -1_i64 {
                return lual_error(self.matchstate_interpreter, c"invalid capture index %%%d".as_ptr(), &[(l + 1).into()]);
            }
            l
        }
    }
    pub unsafe fn capture_to_close(&mut self) -> i32 {
        unsafe {
            let mut level: i32 = self.level as i32;
            level -= 1;
            while level >= 0 {
                if self.capture[level as usize].matchstatecapture_length == -1_i64 {
                    return level;
                }
                level -= 1;
            }
            lual_error(self.matchstate_interpreter, c"invalid pattern capture".as_ptr(), &[])
        }
    }
    pub unsafe fn classend(&mut self, mut p: *const i8) -> *const i8 {
        unsafe {
            let current_p = p;
            p = p.add(1);
            const PERCENT: i32 = Character::Percent as i32;
            const BRACKET_LEFT: i32 = Character::BracketLeft as i32;
            match *current_p as i32 {
                | PERCENT => {
                    if p == self.p_end {
                        lual_error(self.matchstate_interpreter, c"malformed pattern (ends with '%%')".as_ptr(), &[]);
                    }
                    p.add(1)
                },
                | BRACKET_LEFT => {
                    if *p as i32 == Character::Caret as i32 {
                        p = p.add(1);
                    }
                    loop {
                        if p == self.p_end {
                            lual_error(
                                self.matchstate_interpreter,
                                c"malformed pattern (missing Character::BracketRight)".as_ptr(),
                                &[],
                            );
                        }
                        let current_p = p;
                        p = p.add(1);
                        if *current_p as i32 == Character::Percent as i32 && p < self.p_end {
                            p = p.add(1);
                        }
                        if *p as i32 == Character::BracketRight as i32 {
                            break;
                        }
                    }
                    p.add(1)
                },
                | _ => p,
            }
        }
    }
    pub unsafe fn singlematch(&mut self, s: *const i8, p: *const i8, ep: *const i8) -> bool {
        unsafe {
            if s >= self.src_end {
                false
            } else {
                let c: i32 = *s as u8 as i32;
                const PERIOD: i32 = Character::Period as i32;
                const PERCENT: i32 = Character::Percent as i32;
                const BRACKET_LEFT: i32 = Character::BracketLeft as i32;
                match *p as i32 {
                    | PERIOD => true,
                    | PERCENT => match_class(c, *p.add(1) as u8 as i32),
                    | BRACKET_LEFT => matchbracketclass(c, p, ep.sub(1)),
                    | _ => *p as u8 as i32 == c,
                }
            }
        }
    }
    pub unsafe fn matchbalance(&mut self, mut s: *const i8, p: *const i8) -> *const i8 {
        unsafe {
            if p >= self.p_end.sub(1) {
                lual_error(
                    self.matchstate_interpreter,
                    c"malformed pattern (missing arguments to '%%b')".as_ptr(),
                    &[],
                );
            }
            if *s as i32 != *p as i32 {
                return null();
            } else {
                let b: i32 = *p as i32;
                let e: i32 = *p.add(1) as i32;
                let mut cont: i32 = 1;
                loop {
                    s = s.add(1);
                    if s >= self.src_end {
                        break;
                    }
                    if *s as i32 == e {
                        cont -= 1;
                        if cont == 0 {
                            return s.add(1);
                        }
                    } else if *s as i32 == b {
                        cont += 1;
                    }
                }
            }
            null()
        }
    }
    pub unsafe fn max_expand(&mut self, s: *const i8, p: *const i8, ep: *const i8) -> *const i8 {
        unsafe {
            let mut i: i64 = 0;
            while self.singlematch(s.add(i as usize), p, ep) {
                i += 1;
            }
            while i >= 0 {
                let res: *const i8 = self.match_pattern(s.add(i as usize), ep.add(1));
                if !res.is_null() {
                    return res;
                }
                i -= 1;
            }
            null()
        }
    }
    pub unsafe fn min_expand(&mut self, mut s: *const i8, p: *const i8, ep: *const i8) -> *const i8 {
        unsafe {
            loop {
                let res: *const i8 = self.match_pattern(s, ep.add(1));
                if !res.is_null() {
                    return res;
                } else if self.singlematch(s, p, ep) {
                    s = s.add(1);
                } else {
                    return null();
                }
            }
        }
    }
    pub unsafe fn start_capture(&mut self, s: *const i8, p: *const i8, what: i32) -> *const i8 {
        unsafe {
            let level: usize = self.level;
            if level >= MAX_CAPTURES {
                lual_error(self.matchstate_interpreter, c"too many captures".as_ptr(), &[]);
            }
            self.capture[level].matchstatecapture_initial = s;
            self.capture[level].matchstatecapture_length = what as i64;
            self.level = level + 1;
            let res: *const i8 = self.match_pattern(s, p);
            if res.is_null() {
                self.level -= 1;
                self.level;
            }
            res
        }
    }
    pub unsafe fn end_capture(&mut self, s: *const i8, p: *const i8) -> *const i8 {
        unsafe {
            let l: i32 = self.capture_to_close();

            self.capture[l as usize].matchstatecapture_length =
                s.offset_from(self.capture[l as usize].matchstatecapture_initial) as i64;
            let res: *const i8 = self.match_pattern(s, p);
            if res.is_null() {
                self.capture[l as usize].matchstatecapture_length = -1_i64;
            }
            res
        }
    }
    pub unsafe fn match_capture(&mut self, s: *const i8, mut l: i32) -> *const i8 {
        unsafe {
            l = self.check_capture(l);
            let length: usize = self.capture[l as usize].matchstatecapture_length as usize;
            if (self.src_end).offset_from(s) as usize >= length
                && std::slice::from_raw_parts(self.capture[l as usize].matchstatecapture_initial as *const u8, length)
                    == std::slice::from_raw_parts(s as *const u8, length)
            {
                s.add(length)
            } else {
                null()
            }
        }
    }
    pub unsafe fn match_pattern(&mut self, mut s: *const i8, mut p: *const i8) -> *const i8 {
        unsafe {
            let mut ep: *const i8 = null();
            const MATCH_RETURN: usize = 0;
            const MATCH_MAX_EXPAND: usize = 1;
            const MATCH_BALANCE: usize = 2;
            const MATCH_FRONTIER: usize = 3;
            const MATCH_CAPTURE: usize = 4;
            let mut match_action: usize;
            let prev_matchdepth = self.matchdepth;
            self.matchdepth -= 1;
            if prev_matchdepth == 0 {
                lual_error(self.matchstate_interpreter, c"pattern too complex".as_ptr(), &[]);
            }
            loop {
                if p == self.p_end {
                    match_action = MATCH_RETURN;
                    break;
                }
                match Character::from(*p as i32) {
                    | Character::ParenthesisLeft => {
                        if *p.add(1) as i32 == Character::ParenthesisRight as i32 {
                            s = self.start_capture(s, p.add(2), -2);
                        } else {
                            s = self.start_capture(s, p.add(1), -1);
                        }
                        match_action = MATCH_RETURN;
                        break;
                    },
                    | Character::ParenthesisRight => {
                        s = self.end_capture(s, p.add(1));
                        match_action = MATCH_RETURN;
                        break;
                    },
                    | Character::Dollar => {
                        if p.add(1) == self.p_end {
                            s = if s == self.src_end { s } else { null() };
                            match_action = MATCH_RETURN;
                            break;
                        }
                    },
                    | Character::Percent => match Character::from(*p.add(1) as i32) {
                        | Character::LowerB => {
                            match_action = MATCH_BALANCE;
                            match match_action {
                                | MATCH_BALANCE => {
                                    s = self.matchbalance(s, p.add(2));
                                    if s.is_null() {
                                        match_action = MATCH_RETURN;
                                        break;
                                    }
                                    p = p.add(4);
                                    continue;
                                },
                                | MATCH_FRONTIER => {
                                    p = p.add(2);
                                    if *p as i32 != Character::BracketLeft as i32 {
                                        lual_error(
                                            self.matchstate_interpreter,
                                            c"missing Character::BracketLeft after '%%f' in pattern".as_ptr(),
                                            &[],
                                        );
                                    }
                                    let ep: *const i8 = self.classend(p);
                                    let previous: i8 =
                                        (if s == self.src_init { Character::Null as i32 } else { *s.sub(1) as i32 }) as i8;
                                    if !matchbracketclass(previous as u8 as i32, p, ep.sub(1))
                                        && matchbracketclass(*s as u8 as i32, p, ep.sub(1))
                                    {
                                        p = ep;
                                        continue;
                                    } else {
                                        s = null();
                                        match_action = MATCH_RETURN;
                                        break;
                                    }
                                },
                                | _ => {
                                    s = self.match_capture(s, *p.add(1) as u8 as i32);
                                    if s.is_null() {
                                        match_action = MATCH_RETURN;
                                        break;
                                    }
                                    p = p.add(2);
                                    continue;
                                },
                            }
                        },
                        | Character::LowerF => {
                            match_action = MATCH_FRONTIER;
                            match match_action {
                                | MATCH_BALANCE => {
                                    s = self.matchbalance(s, p.add(2));
                                    if s.is_null() {
                                        match_action = MATCH_RETURN;
                                        break;
                                    }
                                    p = p.add(4);
                                    continue;
                                },
                                | MATCH_FRONTIER => {
                                    p = p.add(2);
                                    if *p as i32 != Character::BracketLeft as i32 {
                                        lual_error(
                                            self.matchstate_interpreter,
                                            c"missing Character::BracketLeft after '%%f' in pattern".as_ptr(),
                                            &[],
                                        );
                                    }
                                    let ep: *const i8 = self.classend(p);
                                    let previous: i8 =
                                        (if s == self.src_init { Character::Null as i32 } else { *s.sub(1) as i32 }) as i8;
                                    if !matchbracketclass(previous as u8 as i32, p, ep.sub(1))
                                        && matchbracketclass(*s as u8 as i32, p, ep.sub(1))
                                    {
                                        p = ep;
                                        continue;
                                    } else {
                                        s = null();
                                        match_action = MATCH_RETURN;
                                        break;
                                    }
                                },
                                | _ => {
                                    s = self.match_capture(s, *p.add(1) as u8 as i32);
                                    if s.is_null() {
                                        match_action = MATCH_RETURN;
                                        break;
                                    }
                                    p = p.add(2);
                                    continue;
                                },
                            }
                        },
                        | Character::Digit0
                        | Character::Digit1
                        | Character::Digit2
                        | Character::Digit3
                        | Character::Digit4
                        | Character::Digit5
                        | Character::Digit6
                        | Character::Digit7
                        | Character::Digit8
                        | Character::Digit9 => {
                            match_action = MATCH_CAPTURE;
                            match match_action {
                                | MATCH_BALANCE => {
                                    s = self.matchbalance(s, p.add(2));
                                    if s.is_null() {
                                        match_action = MATCH_RETURN;
                                        break;
                                    }
                                    p = p.add(4);
                                    continue;
                                },
                                | MATCH_FRONTIER => {
                                    p = p.add(2);
                                    if *p as i32 != Character::BracketLeft as i32 {
                                        lual_error(
                                            self.matchstate_interpreter,
                                            c"missing Character::BracketLeft after '%%f' in pattern".as_ptr(),
                                            &[],
                                        );
                                    }
                                    let ep: *const i8 = self.classend(p);
                                    let previous: i8 =
                                        (if s == self.src_init { Character::Null as i32 } else { *s.sub(1) as i32 }) as i8;
                                    if !matchbracketclass(previous as u8 as i32, p, ep.sub(1))
                                        && matchbracketclass(*s as u8 as i32, p, ep.sub(1))
                                    {
                                        p = ep;
                                        continue;
                                    } else {
                                        s = null();
                                        match_action = MATCH_RETURN;
                                        break;
                                    }
                                },
                                | _ => {
                                    s = self.match_capture(s, *p.add(1) as u8 as i32);
                                    if s.is_null() {
                                        match_action = MATCH_RETURN;
                                        break;
                                    }
                                    p = p.add(2);
                                    continue;
                                },
                            }
                        },
                        | _ => {},
                    },
                    | _ => {},
                }
                ep = self.classend(p);
                if !self.singlematch(s, p, ep) {
                    if *ep as i32 == Character::Asterisk as i32
                        || *ep as i32 == Character::Question as i32
                        || *ep as i32 == Character::Hyphen as i32
                    {
                        p = ep.add(1);
                    } else {
                        s = null();
                        match_action = MATCH_RETURN;
                        break;
                    }
                } else {
                    const QUESTION: i32 = Character::Question as i32;
                    const PLUS: i32 = Character::Plus as i32;
                    const ASTERISK: i32 = Character::Asterisk as i32;
                    const HYPHEN: i32 = Character::Hyphen as i32;
                    match *ep as i32 {
                        | QUESTION => {
                            let res: *const i8 = self.match_pattern(s.add(1), ep.add(1));
                            if !res.is_null() {
                                s = res;
                                match_action = MATCH_RETURN;
                                break;
                            } else {
                                p = ep.add(1);
                            }
                        },
                        | PLUS => {
                            s = s.add(1);
                            match_action = MATCH_MAX_EXPAND;
                            break;
                        },
                        | ASTERISK => {
                            match_action = MATCH_MAX_EXPAND;
                            break;
                        },
                        | HYPHEN => {
                            s = self.min_expand(s, p, ep);
                            match_action = MATCH_RETURN;
                            break;
                        },
                        | _ => {
                            s = s.add(1);
                            p = ep;
                        },
                    }
                }
            }
            match match_action {
                | MATCH_MAX_EXPAND => {
                    s = self.max_expand(s, p, ep);
                },
                | _ => {},
            }
            self.matchdepth += 1;
            self.matchdepth;
            s
        }
    }
    pub unsafe fn prepstate(&mut self, state: *mut State, s: *const i8, lexical_state: usize, p: *const i8, lp: usize) {
        self.matchstate_interpreter = state;
        self.matchdepth = LUAI_MAXCCALLS as i32;
        self.src_init = s;
        unsafe {
            self.src_end = s.add(lexical_state);
            self.p_end = p.add(lp);
        }
    }
    pub fn reprepstate(&mut self) {
        self.level = 0;
    }
    pub unsafe fn add_s(&mut self, b: *mut Buffer, s: *const i8, e: *const i8) {
        unsafe {
            let mut l: usize = 0;
            let state: *mut State = self.matchstate_interpreter;
            let mut news: *const i8 = lua_tolstring(state, 3, &mut l);
            let mut p: *const i8;
            loop {
                p = mem_chr(news as *const u8, Character::Percent as u8, l) as *mut i8;
                if p.is_null() {
                    break;
                }
                (*b).add_string_with_length(news, p.offset_from(news) as usize);
                p = p.add(1);
                if *p as i32 == Character::Percent as i32 {
                    if (*b).buffer_loads.get_length() >= (*b).buffer_loads.get_size() {
                        ((*b).prepare_with_size(1)).is_null();
                    }
                    let write_offset = (*b).buffer_loads.get_length();
                    (*b).buffer_loads.set_length(((*b).buffer_loads.get_length() + 1) as usize);
                    *((*b).buffer_loads.at_mut(write_offset as isize)) = *p;
                } else if *p as i32 == Character::Digit0 as i32 {
                    (*b).add_string_with_length(s, e.offset_from(s) as usize);
                } else if Character::from(*p as u8 as i32).is_digit_decimal() {
                    let mut cap: *const i8 = null();
                    let resl: i64 = self.get_onecapture(*p as i32 - Character::Digit1 as i32, s, e, &mut cap) as i64;
                    if resl == -2_i64 {
                        (*b).add_value();
                    } else {
                        (*b).add_string_with_length(cap, resl as usize);
                    }
                } else {
                    lual_error(
                        state,
                        c"invalid use of '%c' in replacement string".as_ptr(),
                        &[(Character::Percent as i32).into()],
                    );
                }
                l -= p.add(1).offset_from(news) as usize;
                news = p.add(1);
            }
            (*b).add_string_with_length(news, l);
        }
    }
}
pub unsafe fn match_class(c: i32, class_: i32) -> bool {
    let res: bool;
    match Character::from((class_ as u8).to_ascii_lowercase() as i32) {
        | Character::LowerA => {
            res = Character::from(c).is_alpha();
        },
        | Character::LowerC => {
            res = Character::from(c).is_control();
        },
        | Character::LowerD => {
            res = Character::from(c).is_digit_decimal();
        },
        | Character::LowerG => {
            res = Character::from(c).is_printable();
        },
        | Character::LowerL => {
            res = Character::from(c).is_lower();
        },
        | Character::LowerP => {
            res = Character::from(c).is_punctuation();
        },
        | Character::LowerS => {
            res = Character::from(c).is_whitespace();
        },
        | Character::LowerU => {
            res = Character::from(c).is_upper();
        },
        | Character::LowerW => {
            res = Character::from(c).is_alphanumeric();
        },
        | Character::LowerX => {
            res = Character::from(c).is_digit_hexadecimal();
        },
        | Character::LowerZ => {
            res = Character::from_negative(c) == Some(Character::Null);
        },
        | _ => return class_ == c,
    }
    if Character::from(class_).is_lower() { res } else { !res }
}
pub unsafe fn matchbracketclass(c: i32, mut p: *const i8, ec: *const i8) -> bool {
    unsafe {
        let mut sig: i32 = 1;
        if *p.add(1) as i32 == Character::Caret as i32 {
            sig = 0;
            p = p.add(1);
        }
        loop {
            p = p.add(1);
            if p >= ec {
                break;
            }
            if *p as i32 == Character::Percent as i32 {
                p = p.add(1);
                if match_class(c, *p as u8 as i32) {
                    return sig != 0;
                }
            } else if *p.add(1) as i32 == Character::Hyphen as i32 && p.add(2) < ec {
                p = p.add(2);
                if *p.sub(2) as u8 as i32 <= c && c <= *p as u8 as i32 {
                    return sig != 0;
                }
            } else if *p as u8 as i32 == c {
                return sig != 0;
            }
        }
        sig == 0
    }
}
