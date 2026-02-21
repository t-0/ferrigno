use crate::buffer::*;
use crate::interpreter::*;
use crate::tdefaultnew::*;
use crate::tagtype::*;
use std::io::Write;

// lua_upvalueindex(1): the upvalue index for a C closure's first upvalue
const UPVALUE1: i32 = -(1000000 + 1000) - 1;

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
pub unsafe fn sh_cmd(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let nargs = (*interpreter).get_top();

        // Build the full command string: "cmd 'arg1' 'arg2' ..."
        let mut cmd: Vec<u8> = Vec::new();

        let mut cmd_len: usize = 0;
        let cmd_ptr = lua_tolstring(interpreter, UPVALUE1, &mut cmd_len);
        if cmd_ptr.is_null() {
            return lual_error(interpreter, c"sh: invalid command name".as_ptr());
        }
        // Command name is unquoted (it's a simple identifier from Lua)
        cmd.extend_from_slice(std::slice::from_raw_parts(cmd_ptr as *const u8, cmd_len));

        for i in 1..=nargs {
            cmd.push(b' ');
            let mut arg_len: usize = 0;
            // lual_tolstring coerces any Lua value to string and pushes it
            let arg_ptr = lual_tolstring(interpreter, i, &mut arg_len);
            if arg_ptr.is_null() {
                return lual_error(interpreter, c"sh: cannot convert argument to string".as_ptr());
            }
            shell_quote(std::slice::from_raw_parts(arg_ptr as *const u8, arg_len), &mut cmd);
            lua_settop(interpreter, -2); // pop the string pushed by lual_tolstring
        }
        cmd.push(0); // null-terminate for popen

        // Flush before forking
        std::io::stdout().flush().ok();
        std::io::stderr().flush().ok();
        *errno_location() = 0;

        let fp = libc::popen(cmd.as_ptr() as *const i8, c"r".as_ptr());
        if fp.is_null() {
            (*interpreter).push_nil();
            lua_pushstring(interpreter, libc::strerror(*errno_location()));
            return 2;
        }

        // Read all stdout into a Lua Buffer
        let mut b = Buffer::new();
        b.initialize(interpreter);
        const READ_SIZE: usize = 4096;
        loop {
            let dst = b.prepare_with_size(READ_SIZE);
            let n = libc::fread(dst as *mut libc::c_void, 1, READ_SIZE, fp);
            b.buffer_loads.add_length(n);
            if n < READ_SIZE {
                break;
            }
        }

        // Collect exit status
        let stat = libc::pclose(fp);
        let exit_code: i32 = if stat & 0x7f == 0 { (stat & 0xff00) >> 8 } else { stat & 0x7f };

        // Push output string (pops Buffer's stack slot, replaces with string)
        b.push_result();

        if exit_code == 0 {
            return 1; // return output string
        } else {
            // Return nil, errmsg, exit_code
            lua_settop(interpreter, -2); // pop output
            (*interpreter).push_nil();
            let mut errmsg: [i8; 64] = [0; 64];
            libc::snprintf(errmsg.as_mut_ptr(), 64, c"exited with code %d".as_ptr(), exit_code);
            lua_pushstring(interpreter, errmsg.as_ptr());
            (*interpreter).push_integer(exit_code as i64);
            return 3;
        }
    }
}

// __index metamethod: sh.command returns a closure that runs "command"
pub unsafe fn sh_index(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        if lua_type(interpreter, 2) != Some(TagType::String) {
            (*interpreter).push_nil();
            return 1;
        }
        // Push the command name (arg 2) as upvalue 1, return sh_cmd closure
        lua_pushvalue(interpreter, 2);
        lua_pushcclosure(interpreter, Some(sh_cmd as unsafe fn(*mut Interpreter) -> i32), 1);
        return 1;
    }
}

pub unsafe fn luaopen_sh(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        (*interpreter).lua_createtable(); // the sh table

        (*interpreter).lua_createtable(); // its metatable
        lua_pushcclosure(interpreter, Some(sh_index as unsafe fn(*mut Interpreter) -> i32), 0);
        lua_setfield(interpreter, -2, c"__index".as_ptr());
        lua_setmetatable(interpreter, -2);

        return 1;
    }
}
