use crate::tvalue::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StackValue {
    pub tvalue: TValue,
    pub delta: u16,
}
pub type StackValuePointer = *mut StackValue;
