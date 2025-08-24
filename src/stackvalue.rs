use crate::value::*;
use crate::tvalue::*;
#[derive(Copy, Clone)]
pub union StackValue {
    pub val: TValue,
    pub tbclist: StackValueExtension,
}
pub type StkId = *mut StackValue;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StackValueExtension {
    pub value: Value,
    pub tag: u8,
    pub delta: u16,
}
