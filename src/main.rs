#![feature(extern_types, c_variadic)]
mod absolutelineinfo;
mod blockcontrol;
mod buffer;
mod bufffs;
mod c;
mod callinfo;
mod calls;
mod closep;
mod closure;
mod constructorcontrol;
mod debug;
mod dumpstate;
mod dynamicdata;
mod expressiondescription;
mod functionstate;
mod gcunion;
mod gmatchstate;
mod header;
mod instruction;
mod labeldesc;
mod labellist;
mod lexstate;
mod lg;
mod lhsassign;
mod loadf;
mod loads;
mod loadstate;
mod localvariable;
mod lx;
mod matchstate;
mod nativeendian;
mod new;
mod global;
mod functions;
mod node;
mod longjump;
mod object;
mod onelua;
mod prototype;
mod ranstate;
mod rawvalue;
mod readfunction;
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
mod udata;
mod upvaldesc;
mod uvalue;
mod writefunction;
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
        ::std::process::exit(crate::onelua::main_0(
            (args.len() - 1) as i32,
            args.as_mut_ptr() as *mut *mut libc::c_char,
        ) as i32)
    }
}
