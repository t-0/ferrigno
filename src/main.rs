#![feature(extern_types, c_variadic)]
mod absolutelineinfo;
mod blockcontrol;
mod unary;
mod buffer;
mod bufffs;
mod c;
mod callinfo;
mod calls;
mod closep;
mod closure;
mod v;
mod constructorcontrol;
mod debug;
mod dumpstate;
mod dynamicdata;
mod utility;
mod expressiondescription;
mod f2i;
mod functionstate;
mod gcunion;
mod gmatchstate;
mod math;
mod header;
mod instruction;
mod labeldescription;
mod labellist;
mod lexicalstate;
mod lhsassign;
mod priority;
mod semanticinfo;
mod lg;
mod loadf;
mod loads;
mod value;
mod upvalue;
mod tvalue;
mod loadstate;
mod stackvalue;
mod localvariable;
mod variabledescription;
mod lx;
mod matchstate;
mod nativeendian;
mod new;
mod lclosure;
mod cclosure;
mod global;
mod functions;
mod node;
mod character;
mod longjump;
mod object;
mod onelua;
mod prototype;
mod randomstate;
mod rawvalue;
mod registeredfunction;
mod rn;
mod sparser;
mod state;
mod stkidrel;
mod stream;
mod streamwriter;
mod stringtable;
mod table;
mod tm;
mod token;
mod tstring;
mod ubox;
mod user;
mod tag;
mod upvaldesc;
mod uvalue;
mod zio;
mod coroutine;
mod k;
mod operator_;
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
        ::std::process::exit(crate::onelua::main_0(
            (args.len() - 1) as i32,
            args.as_mut_ptr() as *mut *mut libc::c_char,
        ) as i32)
    }
}
