use crate::value::*;
use crate::tvalue::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub union StackValue {
    pub val: TValue,
    pub tbc_list: StackValueExtension,
}
pub type StkId = *mut StackValue;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StackValueExtension {
    pub value: Value,
    pub tag: u8,
    pub delta: u16,
}
