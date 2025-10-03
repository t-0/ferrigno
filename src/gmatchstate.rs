use crate::interpreter::*;
use crate::matchstate::*;
use crate::tstring::*;
use crate::user::*;
use std::ptr::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GMatchState {
    pub gmatchstate_source: *const i8,
    pub gmatchstate_pattern: *const i8,
    pub gmatchstate_lastmatch: *const i8,
    pub gmatchstate_matchstate: MatchState,
}
impl GMatchState {
    pub unsafe fn gmatch_aux(interpreter: *mut Interpreter) -> i32 {
        unsafe {
            let gmatch_state: *mut GMatchState = (*interpreter).to_pointer(-1000000 - 1000 - 3) as *mut GMatchState;
            return (*gmatch_state).auxiliary(interpreter);
        }
    }
    pub unsafe fn auxiliary(&mut self, interpreter: *mut Interpreter) -> i32 {
        unsafe {
            self.gmatchstate_matchstate.matchstate_interpreter = interpreter;
            let mut src = self.gmatchstate_source;
            while src <= self.gmatchstate_matchstate.src_end {
                self.gmatchstate_matchstate.reprepstate();
                let e = self.gmatchstate_matchstate.match_0(src, self.gmatchstate_pattern);
                if !e.is_null() && e != self.gmatchstate_lastmatch {
                    self.gmatchstate_lastmatch = e;
                    self.gmatchstate_source = self.gmatchstate_lastmatch;
                    return self.gmatchstate_matchstate.push_captures(src, e);
                }
                src = src.offset(1);
            }
        }
        return 0;
    }
    pub unsafe fn gmatch(interpreter: *mut Interpreter) -> i32 {
        unsafe {
            let mut sourcelength: usize = 0;
            let mut patternlength: usize = 0;
            let source: *const i8 = lual_checklstring(interpreter, 1, &mut sourcelength);
            let pattern: *const i8 = lual_checklstring(interpreter, 2, &mut patternlength);
            let mut initial: usize = (get_position_relative(lual_optinteger(interpreter, 3, 1 as i64), sourcelength)) - 1;
            lua_settop(interpreter, 2);
            if initial > sourcelength {
                initial = sourcelength.wrapping_add(1 as usize);
            }
            let gm: *mut GMatchState = User::lua_newuserdatauv(interpreter, size_of::<GMatchState>(), 0) as *mut GMatchState;
            (*gm).gmatchstate_matchstate.prepstate(interpreter, source, sourcelength, pattern, patternlength);
            (*gm).gmatchstate_source = source.offset(initial as isize);
            (*gm).gmatchstate_pattern = pattern;
            (*gm).gmatchstate_lastmatch = null();
            lua_pushcclosure(
                interpreter,
                Some(GMatchState::gmatch_aux as unsafe fn(*mut Interpreter) -> i32),
                3,
            );
            return 1;
        }
    }
}
