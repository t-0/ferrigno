use crate::interpreter::*;
use crate::matchstate::*;
use crate::tstring::*;
use crate::user::*;
use std::ptr::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GMatchState {
    pub gmatchstate_source: *const i8,
    pub gmatchstate_pointer: *const i8,
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
                let e = self.gmatchstate_matchstate.match_0(src, self.gmatchstate_pointer);
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
            let mut lexical_state: usize = 0;
            let mut lp: usize = 0;
            let s: *const i8 = lual_checklstring(interpreter, 1, &mut lexical_state);
            let p: *const i8 = lual_checklstring(interpreter, 2, &mut lp);
            let mut initial: usize = (get_position_relative(lual_optinteger(interpreter, 3, 1 as i64), lexical_state)) - 1;
            lua_settop(interpreter, 2);
            if initial > lexical_state {
                initial = lexical_state.wrapping_add(1 as usize);
            }
            let gm: *mut GMatchState = User::lua_newuserdatauv(interpreter, size_of::<GMatchState>(), 0) as *mut GMatchState;
            (*gm).gmatchstate_matchstate.prepstate(interpreter, s, lexical_state, p, lp);
            (*gm).gmatchstate_source = s.offset(initial as isize);
            (*gm).gmatchstate_pointer = p;
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
