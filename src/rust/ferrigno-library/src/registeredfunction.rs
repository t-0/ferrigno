use crate::functions::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RegisteredFunction {
    pub registeredfunction_name: *const i8,
    pub registeredfunction_function: CFunction,
}
