use crate::functions::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RegisteredFunction {
    pub name: *const libc::c_char,
    pub function: CFunction,
}
