use crate::tvalue::*;
use crate::tag::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Node {
    pub value: TValue,
    pub key: TValue,
    pub next: i32,
}
pub unsafe extern "C" fn clearkey(node: *mut Node) {
    unsafe {
        if is_collectable((*node).key.tag) {
            (*node).key.tag = (9 as i32 + 2) as u8;
        }
    }
}
