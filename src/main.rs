#![feature(extern_types, c_variadic)]
mod abslineinfo;
mod c;
mod callinfo;
mod closure;
mod gcobject;
mod gcunion;
mod lg;
mod locvar;
mod lua_debug;
mod lx;
mod node;
mod onelua;
mod proto;
mod state;
mod stkidrel;
mod table;
mod tstring;
mod udata;
mod upvaldesc;
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
