use crate::matchstate::*;
use crate::state::*;
use crate::tstring::*;
use crate::user::*;
use std::ptr::*;
#[derive(Clone)]
pub struct GMatchState {
    gmatchstate_source: *const i8,
    gmatchstate_pattern: *const i8,
    gmatchstate_lastmatch: *const i8,
    gmatchstate_matchstate: MatchState,
}
impl GMatchState {
    pub unsafe fn gmatch_aux(state: *mut State) -> i32 {
        unsafe {
            let gmatch_state: *mut GMatchState = (*state).to_pointer(LUA_REGISTRYINDEX - 3) as *mut GMatchState;
            (*gmatch_state).auxiliary(state)
        }
    }
    pub unsafe fn auxiliary(&mut self, state: *mut State) -> i32 {
        unsafe {
            self.gmatchstate_matchstate.matchstate_interpreter = state;
            let mut src = self.gmatchstate_source;
            while src <= self.gmatchstate_matchstate.src_end {
                self.gmatchstate_matchstate.reprepstate();
                let e = self.gmatchstate_matchstate.match_pattern(src, self.gmatchstate_pattern);
                if !e.is_null() && e != self.gmatchstate_lastmatch {
                    self.gmatchstate_lastmatch = e;
                    self.gmatchstate_source = self.gmatchstate_lastmatch;
                    return self.gmatchstate_matchstate.push_captures(src, e);
                }
                src = src.add(1);
            }
        }
        0
    }
    pub unsafe fn gmatch(state: *mut State) -> i32 {
        unsafe {
            let mut sourcelength: usize = 0;
            let mut patternlength: usize = 0;
            let source: *const i8 = lual_checklstring(state, 1, &mut sourcelength);
            let pattern: *const i8 = lual_checklstring(state, 2, &mut patternlength);
            let mut initial: usize = (get_position_relative(lual_optinteger(state, 3, 1_i64), sourcelength)) - 1;
            lua_settop(state, 2);
            if initial > sourcelength {
                initial = sourcelength.wrapping_add(1_usize);
            }
            let gm: *mut GMatchState = User::lua_newuserdatauv(state, size_of::<GMatchState>(), 0) as *mut GMatchState;
            (*gm)
                .gmatchstate_matchstate
                .prepstate(state, source, sourcelength, pattern, patternlength);
            (*gm).gmatchstate_source = source.add(initial);
            (*gm).gmatchstate_pattern = pattern;
            (*gm).gmatchstate_lastmatch = null();
            lua_pushcclosure(state, Some(GMatchState::gmatch_aux as unsafe fn(*mut State) -> i32), 3);
            1
        }
    }
}
