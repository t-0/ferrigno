use crate::node::*;
use crate::object::*;
use crate::stkidrel::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Table {
    pub next: *mut Object,
    pub tag: u8,
    pub marked: u8,
    pub flags: u8,
    pub lsizenode: u8,
    pub alimit: u32,
    pub array: *mut TValue,
    pub node: *mut Node,
    pub lastfree: *mut Node,
    pub metatable: *mut Table,
    pub gc_list: *mut Object,
}
