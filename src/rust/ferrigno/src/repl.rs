use crate::calls::*;
use crate::global::*;
use crate::library::*;
use crate::luastate::LuaState;
use crate::state::*;
use crate::status::*;
use std::io::IsTerminal;
use std::ptr::*;

use crate::functionstate::{ARGS_BARE, ARGS_EXECUTE, ARGS_INTERACTIVE, ARGS_NOENV, ARGS_VERSION};

pub unsafe fn pmain(state: *mut State) -> i32 {
    unsafe {
        let argc = lua_tointegerx(state, 1, null_mut()) as i32;
        let argv: *mut *mut i8 = (*state).to_pointer(2) as *mut *mut i8;
        let mut script: i32 = 0;
        let args = collectargs(argv, &mut script);
        let optlim = if script > 0 { script } else { argc };
        if args == 1 {
            print_usage(*argv.add(script as usize));
            return 0;
        }
        if args & ARGS_NOENV != 0 {
            (*state).push_boolean(true);
            lua_setfield(state, LUA_REGISTRYINDEX, c"LUA_NOENV".as_ptr());
        }
        if args & ARGS_VERSION != 0 {
            print_version();
        }
        lual_openlibs(state);
        createargtable(state, argv, argc, script);
        lua_gc(state, GC_RESTART);
        lua_gc(state, GC_GENERATIONAL, 0, 0);
        if args & ARGS_NOENV == 0 && handle_luainit(state) != Status::OK {
            return 0;
        }
        if runargs(state, argv, optlim) == 0 {
            return 0;
        }
        if script > 0 {
            if handle_script(state, argv.add(script as usize)) != Status::OK {
                return 0;
            }
        } else if args & (ARGS_INTERACTIVE | ARGS_VERSION | ARGS_EXECUTE | ARGS_BARE) == 0 {
            if std::io::stdin().is_terminal() {
                // Run init.lua then drop into the REPL
                if let Some(init_src) = crate::embedded_resources::lookup("init.lua") {
                    let status = lual_loadbufferx(
                        state,
                        init_src.as_ptr() as *const i8,
                        init_src.len(),
                        c"@init.lua".as_ptr(),
                        null(),
                    );
                    if status == Status::OK {
                        let run_status = docall(state, 0, -1);
                        if run_status != Status::OK {
                            report(state, run_status);
                            return 0;
                        }
                    } else {
                        report(state, status);
                        return 0;
                    }
                }
                do_repl(state);
            } else {
                dofile(state, null());
            }
        }
        if args & ARGS_INTERACTIVE != 0 {
            do_repl(state);
        } else if script < 1 && args & (ARGS_EXECUTE | ARGS_VERSION) == 0 && args & ARGS_BARE != 0 {
            if std::io::stdin().is_terminal() {
                do_repl(state);
            } else {
                dofile(state, null());
            }
        }
        (*state).push_boolean(true);
        1
    }
}
pub unsafe fn main_0(argc: i32, argv: *mut *mut i8) -> i32 {
    unsafe {
        crate::lexicalstate::ferrigno_extensions_init();
        let state = match LuaState::new() {
            | None => {
                l_message(*argv.add(0), c"cannot create state: not enough memory".as_ptr());
                return 1;
            },
            | Some(s) => s,
        };
        let state = state.state();
        lua_gc(state, GC_STOP);
        lua_pushcclosure(state, Some(pmain as unsafe fn(*mut State) -> i32), 0);
        (*state).push_integer(argc as i64);
        lua_pushlightuserdata(state, argv as *mut std::ffi::c_void);
        let status = CallS::api_call(state, 2, 1, 0, 0, None);
        let result = lua_toboolean(state, -1);
        report(state, status);
        // state drops here, calling close_state automatically
        if result && status == Status::OK { 0 } else { 1 }
    }
}
