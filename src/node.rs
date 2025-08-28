use crate::tvalue::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Node {
    pub value: TValue,
    pub key: TValue,
    pub next: i32,
}
