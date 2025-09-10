#![feature(default_field_values,extern_types, c_variadic)]
mod allocator;
mod buffer;
mod bufffs;
mod callinfo;
mod calls;
mod character;
mod closep;
mod closure;
mod coroutine;
mod debugger;
mod debuginfo;
mod dumpstate;
mod dynamicdata;
mod expressiondescription;
mod expressionkind;
mod f2i;
mod forloop;
mod functions;
mod functionstate;
mod global;
mod gmatchstate;
mod header;
mod io;
mod k;
mod labeldescription;
mod lexical;
mod library;
mod loadf;
mod loadstate;
mod localvariable;
mod longjump;
mod matchstate;
mod nativeendian;
mod new;
mod node;
mod object;
mod operator_;
mod prototype;
mod randomstate;
mod registeredfunction;
mod repl;
mod sparser;
mod stackvalue;
mod interpreter;
mod stkidrel;
mod streamwriter;
mod stringtable;
mod vectort;
mod table;
mod tag;
mod tm;
mod token;
mod tstring;
mod tvalue;
mod upvalue;
mod loadable;
mod upvaluedescription;
mod user;
mod utility;
mod value;
mod variabledescription;
mod vm;
mod zio;
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(crate::repl::main_0(
            (args.len() - 1) as i32,
            args.as_mut_ptr() as *mut *mut libc::c_char,
        ) as i32)
    }
}
