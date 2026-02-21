use crate::interpreter::{close_state, lual_newstate, Interpreter};

/// RAII owner of a Lua interpreter.
///
/// Calls `close_state` automatically on drop, so there is no need to call
/// `(*global).close()` manually.  The interpreter pointer is valid for the
/// lifetime of this struct.
pub struct LuaState {
    interpreter: *mut Interpreter,
}

impl LuaState {
    pub unsafe fn new() -> Option<LuaState> {
        unsafe {
            let (_global, interpreter) = lual_newstate();
            if interpreter.is_null() {
                None
            } else {
                Some(LuaState { interpreter })
            }
        }
    }

    pub fn interpreter(&self) -> *mut Interpreter {
        self.interpreter
    }
}

impl Drop for LuaState {
    fn drop(&mut self) {
        unsafe { close_state(self.interpreter) }
    }
}
