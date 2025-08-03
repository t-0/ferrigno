#![allow(
    static_mut_refs,
    unsafe_code,
    unsafe_attr_outside_unsafe,
    unsafe_op_in_unsafe_fn,
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types, c_variadic)]
mod lapi;
mod lauxlib;
mod lbaselib;
mod lcode;
mod lcorolib;
mod lctype;
mod ldblib;
mod ldebug;
mod ldo;
mod ldump;
mod lfunc;
mod lgc;
mod linit;
mod liolib;
mod llex;
mod lmathlib;
mod lmem;
mod loadlib;
mod lobject;
mod lopcodes;
mod loslib;
mod lparser;
mod lstate;
mod lstring;
mod lstrlib;
mod ltable;
mod ltablib;
mod ltm;
mod lua;
mod lundump;
mod lutf8lib;
mod lvm;
mod lzio;
mod types;
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
        ::std::process::exit(crate::lua::main_0(
            (args.len() - 1) as i32,
            args.as_mut_ptr() as *mut *mut libc::c_char,
        ) as i32)
    }
}
