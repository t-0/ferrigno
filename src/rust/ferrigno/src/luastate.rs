use crate::state::{State, close_state, lual_newstate};

/// RAII owner of a Lua state.
///
/// Calls `close_state` automatically on drop, so there is no need to call
/// `(*global).close()` manually.  The state pointer is valid for the
/// lifetime of this struct.
pub struct LuaState {
    luastate_state: *mut State,
}

impl LuaState {
    pub unsafe fn new() -> Option<LuaState> {
        unsafe {
            let (_global, state) = lual_newstate();
            if state.is_null() { None } else { Some(LuaState { luastate_state: state }) }
        }
    }

    pub fn state(&self) -> *mut State {
        self.luastate_state
    }
}

impl Drop for LuaState {
    fn drop(&mut self) {
        unsafe { close_state(self.luastate_state) }
    }
}
