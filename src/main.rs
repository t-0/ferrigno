#![feature(default_field_values,extern_types, c_variadic)]
mod lexical;
mod debugger;
mod buffer;
mod operatorbinary;
mod bufffs;
mod utility;
mod callinfo;
mod calls;
mod cclosure;
mod character;
mod closep;
mod closure;
mod coroutine;
mod debuginfo;
mod dumpstate;
mod dynamicdata;
mod expressiondescription;
mod f2i;
mod functions;
mod library;
mod functionstate;
mod global;
mod gmatchstate;
mod header;
mod instruction;
mod k;
mod labeldescription;
mod labellist;
mod lclosure;
mod lexicalstate;
mod lg;
mod lhsassign;
mod loadf;
mod loads;
mod loadstate;
mod localvariable;
mod longjump;
mod lx;
mod matchstate;
mod nativeendian;
mod new;
mod repl;
mod node;
mod object;
mod operator_;
mod priority;
mod prototype;
mod randomstate;
mod rawvalue;
mod registeredfunction;
mod rn;
mod semanticinfo;
mod sparser;
mod stackvalue;
mod state;
mod stkidrel;
mod stream;
mod streamwriter;
mod stringtable;
mod table;
mod tag;
mod math;
mod tm;
mod forloop;
mod token;
mod tstring;
mod tvalue;
mod ubox;
mod operatorunary;
mod upvaluedescription;
mod upvalue;
mod user;
mod uvalue;
mod v;
mod value;
mod variabledescription;
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
