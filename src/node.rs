use crate::tvalue::*;
use crate::value::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub union Node {
    pub u: NodeKey,
    pub i_value: TValue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NodeKey {
    pub tag: u8,
    pub value: Value,
    pub key_tag: u8,
    pub key_value: Value,
    pub next: i32,
}
