use std::ptr::*;
pub fn main() {
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        if !ferrigno::state::in_lua_protected_context() {
            default_hook(info);
        }
    }));
    let mut args: Vec<*mut i8> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(null_mut());
    unsafe { ::std::process::exit(ferrigno::repl::main_0((args.len() - 1) as i32, args.as_mut_ptr() as *mut *mut i8) as i32) }
}
