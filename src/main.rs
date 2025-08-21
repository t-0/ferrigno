#![feature(extern_types, c_variadic)]
mod nativeendian;
mod absolutelineinfo;
mod blockcontrol;
mod bufffs;
mod c;
mod rawvalue;
mod callinfo;
mod calls;
mod closep;
mod closure;
mod constructorcontrol;
mod dumpstate;
mod dynamicdata;
mod expressiondescription;
mod functionstate;
mod gcobject;
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
mod debug;
mod readfunction;
mod writefunction;
mod buffer;
mod registeredfunction;
mod stream;
mod lx;
mod matchstate;
mod mbuffer;
mod node;
mod onelua;
mod prototype;
mod ranstate;
mod rn;
mod sparser;
mod state;
mod stkidrel;
mod streamwriter;
mod stringtable;
mod table;
mod tm;
mod token;
mod tstring;
mod uvalue;
mod ubox;
mod udata;
mod upvaldesc;
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
