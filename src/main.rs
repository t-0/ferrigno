#![feature(default_field_values,extern_types, c_variadic)]
mod absolutelineinfo;
mod blockcontrol;
mod buffer;
mod operatorbinary;
mod bufffs;
mod c;
mod callinfo;
mod calls;
mod cclosure;
mod character;
mod closep;
mod closure;
mod constructorcontrol;
mod coroutine;
mod debuginfo;
mod dumpstate;
mod dynamicdata;
mod expressiondescription;
mod f2i;
mod functions;
mod libraries;
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
mod librarymath;
mod nativeendian;
mod new;
mod node;
mod object;
mod onelua;
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
mod librarybase;
mod librarycoroutine;
mod librarydebug;
mod tstring;
mod tvalue;
mod ubox;
mod operatorunary;
mod upvaluedescription;
mod upvalue;
mod user;
mod utility;
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
        ::std::process::exit(crate::state::main_0(
            (args.len() - 1) as i32,
            args.as_mut_ptr() as *mut *mut libc::c_char,
        ) as i32)
    }
}
