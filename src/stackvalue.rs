use crate::tvalue::*;
use crate::value::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub union StackValue {
    pub value: TValue,
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
