use std::ptr::*;
use crate::matchstate::*;
use crate::interpreter::*;
use crate::tstring::*;
use crate::user::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GMatchState {
    pub source: *const i8,
    pub pointer: *const i8,
    pub last_match: *const i8,
    pub match_state: MatchState,
}
impl GMatchState {
    pub unsafe extern "C" fn gmatch_aux(interpreter: *mut Interpreter) -> i32 {
        unsafe {
            let gmatch_state: *mut GMatchState =
                lua_touserdata(interpreter, -(1000000 as i32) - 1000 as i32 - 3) as *mut GMatchState;
            return (*gmatch_state).auxiliary(interpreter);
        }
    }

    pub unsafe fn auxiliary(& mut self, interpreter: *mut Interpreter) -> i32{
        unsafe {
            self.match_state.interpreter = interpreter;
            let mut src = self.source;
            while src <= self.match_state.src_end {
                self.match_state.reprepstate();
                let e = self.match_state.match_0(src, self.pointer);
                if !e.is_null() && e != self.last_match {
                    self.last_match = e;
                    self.source = self.last_match;
                    return self.match_state.push_captures(src, e);
                }
                src = src.offset(1);
            }
        }
        return 0;
    }
    pub unsafe extern "C" fn gmatch(interpreter: *mut Interpreter) -> i32 {
        unsafe {
            let mut lexical_state: usize = 0;
            let mut lp: usize = 0;
            let s: *const i8 = lual_checklstring(interpreter, 1, &mut lexical_state);
            let p: *const i8 = lual_checklstring(interpreter, 2, &mut lp);
            let mut init: usize =
                (get_position_relative(lual_optinteger(interpreter, 3, 1 as i64), lexical_state)).wrapping_sub(1 as usize);
            lua_settop(interpreter, 2);
            if init > lexical_state {
                init = lexical_state.wrapping_add(1 as usize);
            }
            let gm: *mut GMatchState = User::lua_newuserdatauv(interpreter, ::core::mem::size_of::<GMatchState>(), 0)
                as *mut GMatchState;
            (*gm).match_state.prepstate(interpreter, s, lexical_state, p, lp);
            (*gm).source = s.offset(init as isize);
            (*gm).pointer = p;
            (*gm).last_match = null();
            lua_pushcclosure(
                interpreter,
                Some(GMatchState::gmatch_aux as unsafe extern "C" fn(*mut Interpreter) -> i32),
                3,
            );
            return 1;
        }
    }
}
