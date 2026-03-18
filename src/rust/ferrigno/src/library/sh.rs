use crate::state::*;
use crate::tagtype::*;
use std::io::Write;

// lua_upvalueindex(1): the upvalue index for a C closure's first upvalue
const UPVALUE1: i32 = LUA_REGISTRYINDEX - 1;

// Shell-escape a byte slice using single-quote wrapping.
// Each ' inside the argument becomes '\''.
fn shell_quote(arg: &[u8], out: &mut Vec<u8>) {
    out.push(b'\'');
    for &b in arg {
        if b == b'\'' {
            out.extend_from_slice(b"'\\''");
        } else {
            out.push(b);
        }
    }
    out.push(b'\'');
}

// Called when sh.command(arg1, arg2, ...) is invoked.
// Upvalue 1 holds the command name as a Lua string.
pub unsafe fn sh_cmd(state: *mut State) -> i32 {
    unsafe {
        let nargs = (*state).get_top();

        // Build the full command string: "cmd 'arg1' 'arg2' ..."
        let mut cmd: Vec<u8> = Vec::new();

        let mut cmd_len: usize = 0;
        let cmd_ptr = lua_tolstring(state, UPVALUE1, &mut cmd_len);
        if cmd_ptr.is_null() {
            return lual_error(state, c"sh: invalid command name".as_ptr(), &[]);
        }
        // Command name is unquoted (it's a simple identifier from Lua)
        cmd.extend_from_slice(std::slice::from_raw_parts(cmd_ptr as *const u8, cmd_len));

        for i in 1..=nargs {
            cmd.push(b' ');
            let mut arg_len: usize = 0;
            // lual_tolstring coerces any Lua value to string and pushes it
            let arg_ptr = lual_tolstring(state, i, &mut arg_len);
            if arg_ptr.is_null() {
                return lual_error(state, c"sh: cannot convert argument to string".as_ptr(), &[]);
            }
            shell_quote(std::slice::from_raw_parts(arg_ptr as *const u8, arg_len), &mut cmd);
            lua_settop(state, -2); // pop the string pushed by lual_tolstring
        }
        cmd.push(0); // null-terminate for popen

        // Flush before forking
        std::io::stdout().flush().ok();
        std::io::stderr().flush().ok();

        // cmd is already null-terminated; strip the null for the Rust string
        let cmd_str = std::str::from_utf8(&cmd[..cmd.len() - 1]).unwrap_or("");
        let output = match std::process::Command::new("sh").arg("-c").arg(cmd_str).output() {
            | Ok(o) => o,
            | Err(e) => {
                (*state).push_nil();
                let msg = std::ffi::CString::new(e.to_string()).unwrap_or_default();
                lua_pushstring(state, msg.as_ptr());
                return 2;
            },
        };

        if output.status.success() {
            lua_pushlstring(state, output.stdout.as_ptr() as *const i8, output.stdout.len());
            1
        } else {
            let exit_code = output.status.code().unwrap_or(1);
            (*state).push_nil();
            let msg = std::ffi::CString::new(format!("exited with code {}", exit_code)).unwrap_or_default();
            lua_pushstring(state, msg.as_ptr());
            (*state).push_integer(exit_code as i64);
            3
        }
    }
}

/// Execute a pre-built command string via sh -c.
/// Returns the same values as sh.command: stdout on success, nil+msg+code on failure.
/// Used internally by backtick syntax.
pub unsafe fn sh_exec(state: *mut State) -> i32 {
    unsafe {
        (*state).lual_checktype(1, TagType::String);
        let mut cmd_len: usize = 0;
        let cmd_ptr = lua_tolstring(state, 1, &mut cmd_len);
        let cmd_str = std::str::from_utf8_unchecked(std::slice::from_raw_parts(cmd_ptr as *const u8, cmd_len));

        std::io::stdout().flush().ok();
        std::io::stderr().flush().ok();

        let output = match std::process::Command::new("sh").arg("-c").arg(cmd_str).output() {
            | Ok(o) => o,
            | Err(e) => {
                (*state).push_nil();
                let msg = std::ffi::CString::new(e.to_string()).unwrap_or_default();
                lua_pushstring(state, msg.as_ptr());
                return 2;
            },
        };

        if output.status.success() {
            lua_pushlstring(state, output.stdout.as_ptr() as *const i8, output.stdout.len());
            1
        } else {
            let exit_code = output.status.code().unwrap_or(1);
            (*state).push_nil();
            let msg = std::ffi::CString::new(format!("exited with code {}", exit_code)).unwrap_or_default();
            lua_pushstring(state, msg.as_ptr());
            (*state).push_integer(exit_code as i64);
            3
        }
    }
}

// __index metamethod: sh.command returns a closure that runs "command"
pub unsafe fn sh_index(state: *mut State) -> i32 {
    unsafe {
        if lua_type(state, 2) != Some(TagType::String) {
            (*state).push_nil();
            return 1;
        }
        // Push the command name (arg 2) as upvalue 1, return sh_cmd closure
        lua_pushvalue(state, 2);
        lua_pushcclosure(state, Some(sh_cmd as unsafe fn(*mut State) -> i32), 1);
        1
    }
}

pub unsafe fn luaopen_sh(state: *mut State) -> i32 {
    unsafe {
        (*state).lua_createtable(); // the sh table

        (*state).lua_createtable(); // its metatable
        lua_pushcclosure(state, Some(sh_index as unsafe fn(*mut State) -> i32), 0);
        lua_setfield(state, -2, c"__index".as_ptr());
        lua_setmetatable(state, -2);

        // register sh_exec as a global for backtick syntax
        lua_pushcclosure(state, Some(sh_exec as unsafe fn(*mut State) -> i32), 0);
        lua_setglobal(state, c"__ferrigno_backtick".as_ptr());

        1
    }
}
