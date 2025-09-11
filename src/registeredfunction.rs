use crate::functions::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RegisteredFunction {
    pub name: *const i8,
    pub function: CFunction,
}
