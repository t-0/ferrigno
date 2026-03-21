use std::ffi::c_void;
use std::ptr::*;

use ferrigno::dumpstate::*;
use ferrigno::global::lua_gc;
use ferrigno::luastate::LuaState;
use ferrigno::prototype::*;
use ferrigno::state::*;
use ferrigno::status::*;

const PROGNAME: &str = "ferrignoc";

fn fatal(msg: &str) -> ! {
    eprintln!("{}: {}", PROGNAME, msg);
    std::process::exit(1);
}

fn usage() -> ! {
    eprintln!(
        "usage: {} [options] [filenames]\n\
         Available options are:\n  \
         -l       list (use -l -l for full listing)\n  \
         -o name  output to file 'name' (default is \"ferrignoc.out\")\n  \
         -p       parse only\n  \
         -s       strip debug information\n  \
         --       stop handling options\n  \
         -        stop handling options and process stdin",
        PROGNAME
    );
    std::process::exit(1);
}

struct Options {
    listing: i32,
    output: String,
    parse_only: bool,
    strip: bool,
    files: Vec<String>,
}

fn parse_args(args: &[String]) -> Options {
    let mut opts = Options {
        listing: 0,
        output: "ferrignoc.out".to_string(),
        parse_only: false,
        strip: false,
        files: Vec::new(),
    };
    let mut i = 1;
    while i < args.len() {
        let arg = &args[i];
        if !arg.starts_with('-') || arg == "-" {
            break;
        }
        if arg == "--" {
            i += 1;
            break;
        }
        match arg.as_str() {
            "-l" => opts.listing += 1,
            "-o" => {
                i += 1;
                if i >= args.len() {
                    usage();
                }
                opts.output = args[i].clone();
            }
            "-p" => opts.parse_only = true,
            "-s" => opts.strip = true,
            _ => usage(),
        }
        i += 1;
    }
    while i < args.len() {
        opts.files.push(args[i].clone());
        i += 1;
    }
    if opts.files.is_empty() {
        usage();
    }
    opts
}

/// Writer callback for lua_dump — appends to a Vec<u8>.
unsafe fn file_writer(_state: *mut State, data: *const c_void, size: usize, user_data: *mut c_void) -> i32 {
    unsafe {
        let vec = &mut *(user_data as *mut Vec<u8>);
        let slice = std::slice::from_raw_parts(data as *const u8, size);
        vec.extend_from_slice(slice);
        0
    }
}

/// Load a single source file, leaving the compiled closure on the stack.
unsafe fn load_file(state: *mut State, filename: &str) -> Status {
    unsafe {
        if filename == "-" {
            lual_loadfilex(state, null(), null())
        } else {
            let cname = std::ffi::CString::new(filename).unwrap();
            lual_loadfilex(state, cname.as_ptr(), null())
        }
    }
}

/// Get the prototype from a closure at the given stack offset (relative to top).
unsafe fn get_prototype(state: *mut State, offset: isize) -> *mut Prototype {
    unsafe {
        let ptr = if offset >= 0 {
            (*state)
                .interpreter_top
                .stkidrel_pointer
                .add(offset as usize)
        } else {
            (*state)
                .interpreter_top
                .stkidrel_pointer
                .sub((-offset) as usize)
        };
        let o = &*ptr;
        let cl = o.as_closure().unwrap();
        (*cl).closure_payload.closurepayload_lprototype
    }
}

/// Combine multiple closures on the stack into a single wrapper.
///
/// Each loaded file is a main-chunk closure whose upvalue[0] (_ENV) has
/// isinstack=true, index=0 — meaning "capture enclosing function's stack
/// slot 0".  When we embed these as sub-prototypes inside a wrapper, _ENV
/// is the wrapper's upvalue[0], not a stack local.  So we patch each
/// sub-prototype's upvalue[0] to isinstack=false, index=0 ("inherit from
/// enclosing function's upvalue[0]").
unsafe fn combine(state: *mut State, n: i32) -> *const Prototype {
    unsafe {
        if n == 1 {
            return get_prototype(state, -1);
        }

        // Build a wrapper with n dummy inner functions that reference _ENV
        // so the compiler allocates n sub-prototypes with the right upvalue
        // structure.
        let mut src = String::new();
        for i in 0..n {
            if i > 0 {
                src.push_str("; ");
            }
            src.push_str("(function() return _ENV end)()");
        }

        let csrc = std::ffi::CString::new(src).unwrap();
        let status = lual_loadbufferx(
            state,
            csrc.as_ptr(),
            csrc.as_bytes().len(),
            c"=(ferrignoc)".as_ptr(),
            null(),
        );
        if status != Status::OK {
            fatal("failed to create wrapper chunk");
        }

        let wrapper_proto = get_prototype(state, -1);

        // Replace dummy sub-prototypes with the real ones.
        // The loaded closures sit at stack positions -(n+1) .. -2,
        // the wrapper is at -1.
        for i in 0..n {
            let idx = -(n + 1) + i;
            let sub_proto = get_prototype(state, idx as isize);

            // Patch upvalue[0]: change from "capture parent's stack slot"
            // to "inherit parent's upvalue" so the sub-function gets the
            // wrapper's _ENV upvalue instead of a non-existent stack local.
            if (*sub_proto).prototype_upvalues.get_size() > 0 {
                let uv = &mut *(*sub_proto).prototype_upvalues.vectort_pointer;
                uv.upvaluedescription_isinstack = false;
                uv.upvaluedescription_index = 0;
            }

            *(*wrapper_proto)
                .prototype_prototypes
                .vectort_pointer
                .add(i as usize) = sub_proto;
        }

        wrapper_proto
    }
}

// ─── Listing (disassembly) ───────────────────────────────────────────────────

unsafe fn print_header(proto: *const Prototype) {
    unsafe {
        let source = if (*proto).prototype_source.is_null() {
            "=?".to_string()
        } else {
            let src_ptr = (*proto).prototype_source;
            let len = (*src_ptr).get_length();
            let contents = (*src_ptr).get_contents_mut();
            let bytes = std::slice::from_raw_parts(contents as *const u8, len);
            String::from_utf8_lossy(bytes).into_owned()
        };
        let is_main = (*proto).prototype_linedefined == 0;
        let kind = if is_main { "main" } else { "function" };

        let n_code = (*proto).prototype_code.get_size();
        let first_line = (*proto).prototype_linedefined;
        let last_line = (*proto).prototype_lastlinedefined;
        let n_params = (*proto).prototype_countparameters;
        let is_vararg = (*proto).prototype_isvariablearguments;
        let n_stack = (*proto).prototype_maximumstacksize;
        let n_ups = (*proto).prototype_upvalues.get_size();
        let n_locals = (*proto).prototype_localvariables.get_size();
        let n_consts = (*proto).prototype_constants.get_size();
        let n_protos = (*proto).prototype_prototypes.get_size();

        if is_main {
            eprint!("\n{} <{}:{}> ", kind, source, first_line);
        } else {
            eprint!("\n{} <{}:{},{}> ", kind, source, first_line, last_line);
        }
        eprintln!(
            "({} instruction{})",
            n_code,
            if n_code == 1 { "" } else { "s" }
        );
        eprintln!(
            "{}{} param{}, {} slot{}, {} upvalue{}, {} local{}, {} constant{}, {} function{}",
            n_params,
            if is_vararg { "+" } else { "" },
            if n_params == 1 { "" } else { "s" },
            n_stack,
            if n_stack == 1 { "" } else { "s" },
            n_ups,
            if n_ups == 1 { "" } else { "s" },
            n_locals,
            if n_locals == 1 { "" } else { "s" },
            n_consts,
            if n_consts == 1 { "" } else { "s" },
            n_protos,
            if n_protos == 1 { "" } else { "s" },
        );
    }
}

unsafe fn print_code(proto: *const Prototype) {
    unsafe {
        let code = (*proto).prototype_code.vectort_pointer;
        let n = (*proto).prototype_code.get_size();
        for i in 0..n as isize {
            let instruction = *code.add(i as usize);
            let line = get_line(proto, i as i32);
            eprintln!("\t{}  [{}]\t0x{:08X}", i + 1, line, instruction);
        }
    }
}

unsafe fn get_line(proto: *const Prototype, pc: i32) -> i32 {
    unsafe {
        if (*proto).prototype_lineinfo.get_size() == 0 {
            return 0;
        }
        let base_line = (*proto).prototype_linedefined;
        let li = (*proto).prototype_lineinfo.vectort_pointer;
        let mut line = base_line;
        let n = ((*proto).prototype_lineinfo.get_size() as i32).min(pc + 1);
        for j in 0..n as isize {
            let delta = *li.add(j as usize);
            if delta == -128 {
                line = get_abs_line(proto, j as i32);
            } else {
                line += delta as i32;
            }
        }
        line
    }
}

unsafe fn get_abs_line(proto: *const Prototype, pc: i32) -> i32 {
    unsafe {
        let n = (*proto).prototype_absolutelineinfo.get_size() as i32;
        if n == 0 {
            return (*proto).prototype_linedefined;
        }
        let abs = (*proto).prototype_absolutelineinfo.vectort_pointer;
        let mut lo = 0i32;
        let mut hi = n - 1;
        while lo < hi {
            let mid = (lo + hi + 1) / 2;
            if (*abs.add(mid as usize)).absolutelineinfo_program_counter > pc {
                hi = mid - 1;
            } else {
                lo = mid;
            }
        }
        (*abs.add(lo as usize)).absolutelineinfo_line
    }
}

unsafe fn print_listing(proto: *const Prototype, full: bool) {
    unsafe {
        print_header(proto);
        print_code(proto);
        let n = (*proto).prototype_prototypes.get_size();
        let pp = (*proto).prototype_prototypes.vectort_pointer;
        for i in 0..n as isize {
            print_listing(*pp.add(i as usize), full);
        }
    }
}

// ─── Main ────────────────────────────────────────────────────────────────────

fn main() {
    std::panic::set_hook(Box::new(|_| {}));
    let args: Vec<String> = std::env::args().collect();
    let opts = parse_args(&args);

    unsafe {
        let state = match LuaState::new() {
            None => fatal("cannot create state: not enough memory"),
            Some(s) => s,
        };
        let state = state.state();
        lua_gc(state, 0, &[]);

        // Load all input files
        for filename in &opts.files {
            let status = load_file(state, filename);
            if status != Status::OK {
                let msg = lua_tolstring(state, -1, null_mut());
                if !msg.is_null() {
                    let s = std::ffi::CStr::from_ptr(msg).to_string_lossy();
                    fatal(&s);
                } else {
                    fatal(&format!("failed to load {}", filename));
                }
            }
        }

        let n = opts.files.len() as i32;

        // Listing
        if opts.listing > 0 {
            for i in 0..n {
                let idx = -(n - i);
                let proto = get_prototype(state, idx as isize);
                print_listing(proto, opts.listing > 1);
            }
        }

        // Write output
        if !opts.parse_only {
            let proto = combine(state, n);

            let mut output_buf: Vec<u8> = Vec::new();
            let status = DumpState::save_prototype(
                state,
                proto,
                Some(file_writer as unsafe fn(*mut State, *const c_void, usize, *mut c_void) -> i32),
                &mut output_buf as *mut Vec<u8> as *mut c_void,
                opts.strip,
            );
            if status != 0 {
                fatal("failed to dump");
            }

            if let Err(e) = std::fs::write(&opts.output, &output_buf) {
                fatal(&format!("cannot write {}: {}", opts.output, e));
            }
        }
    }
}
