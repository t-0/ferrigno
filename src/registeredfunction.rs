use crate::functions::*;
#[derive(Copy, Clone)]
pub struct RegisteredFunction {
    pub name: *const i8,
    pub function: CFunction,
}
