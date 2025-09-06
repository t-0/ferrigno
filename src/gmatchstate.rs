use crate::matchstate::*;
use crate::state::*;
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
    pub unsafe extern "C" fn gmatch_aux(state: *mut State) -> i32 {
        unsafe {
            let gmatch_state: *mut GMatchState =
                lua_touserdata(state, -(1000000 as i32) - 1000 as i32 - 3) as *mut GMatchState;
            return (*gmatch_state).auxiliary(state);
        }
    }

    pub unsafe fn auxiliary(& mut self, state: *mut State) -> i32{
        unsafe {
            self.match_state.state = state;
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
    pub unsafe extern "C" fn gmatch(state: *mut State) -> i32 {
        unsafe {
            let mut lexical_state: u64 = 0;
            let mut lp: u64 = 0;
            let s: *const i8 = lual_checklstring(state, 1, &mut lexical_state);
            let p: *const i8 = lual_checklstring(state, 2, &mut lp);
            let mut init: u64 =
                (get_position_relative(lual_optinteger(state, 3, 1 as i64), lexical_state)).wrapping_sub(1 as u64);
            lua_settop(state, 2);
            if init > lexical_state {
                init = lexical_state.wrapping_add(1 as u64);
            }
            let gm: *mut GMatchState = User::lua_newuserdatauv(state, ::core::mem::size_of::<GMatchState>(), 0)
                as *mut GMatchState;
            (*gm).match_state.prepstate(state, s, lexical_state, p, lp);
            (*gm).source = s.offset(init as isize);
            (*gm).pointer = p;
            (*gm).last_match = std::ptr::null();
            lua_pushcclosure(
                state,
                Some(GMatchState::gmatch_aux as unsafe extern "C" fn(*mut State) -> i32),
                3,
            );
            return 1;
        }
    }
}
