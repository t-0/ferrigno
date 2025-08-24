use crate::tvalue::*;
use crate::value::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub union Node {
    pub u: NodeKey,
    pub i_val: TValue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NodeKey {
    pub value_: Value,
    pub tt_: u8,
    pub key_tt: u8,
    pub next: i32,
    pub key_val: Value,
}
