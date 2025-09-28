use crate::calls::*;
use crate::status::*;
use crate::global::*;
use crate::interpreter::*;
use crate::library::*;
use libc::isatty;
use std::ptr::*;
pub unsafe fn pmain(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let argc = lua_tointegerx(interpreter, 1, null_mut()) as i32;
        let argv: *mut *mut i8 = (*interpreter).to_pointer(2) as *mut *mut i8;
        let mut script: i32 = 0;
        let args = collectargs(argv, &mut script);
        let optlim = if script > 0 { script } else { argc };
        lual_checkversion_(interpreter, 504.0, (size_of::<i64>() as usize).wrapping_mul(16 as usize).wrapping_add(size_of::<f64>() as usize));
        if args == 1 {
            print_usage(*argv.offset(script as isize));
            return 0;
        }
        if args & 16 as i32 != 0 {
            (*interpreter).push_boolean(true);
            lua_setfield(interpreter, -(1000000 as i32) - 1000 as i32, c"LUA_NOENV".as_ptr());
        }
        lual_openlibs(interpreter);
        createargtable(interpreter, argv, argc, script);
        lua_gc(interpreter, 1);
        lua_gc(interpreter, 10 as i32, 0, 0);
        if args & 16 as i32 == 0 {
            if handle_luainit(interpreter) != Status::OK {
                return 0;
            }
        }
        if runargs(interpreter, argv, optlim) == 0 {
            return 0;
        }
        if script > 0 {
            if handle_script(interpreter, argv.offset(script as isize)) != Status::OK {
                return 0;
            }
        }
        if args & 2 != 0 {
            do_repl(interpreter);
        } else if script < 1 && args & (8 | 4) == 0 {
            if isatty(0) != 0 {
                do_repl(interpreter);
            } else {
                dofile(interpreter, null());
            }
        }
        (*interpreter).push_boolean(true);
        return 1;
    }
}
pub unsafe fn main_0(argc: i32, argv: *mut *mut i8) -> i32 {
    unsafe {
        let interpreter: *mut Interpreter = lual_newstate();
        if interpreter.is_null() {
            l_message(*argv.offset(0), c"cannot create interpreter: not enough memory".as_ptr());
            return 1;
        } else {
            lua_gc(interpreter, 0);
            lua_pushcclosure(interpreter, Some(pmain as unsafe fn(*mut Interpreter) -> i32), 0);
            (*interpreter).push_integer(argc as i64);
            lua_pushlightuserdata(interpreter, argv as *mut libc::c_void);
            let status = CallS::api_call(interpreter, 2, 1, 0, 0, None);
            let result: i32 = lua_toboolean(interpreter, -1);
            report(interpreter, status);
            lua_close(interpreter);
            return if result != 0 && status == Status::OK { 0 } else { 1 };
        }
    }
}
