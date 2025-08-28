use crate::tvalue::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Node {
    pub u: NodeKey,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NodeKey {
    pub value: TValue,
    pub key: TValue,
    pub next: i32,
}
