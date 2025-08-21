use crate::stkidrel::*;
#[derive(Copy, Clone)]
pub struct RegisteredFunction {
    pub name: *const i8,
    pub func: CFunction,
}
