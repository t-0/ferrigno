use crate::node::*;
use crate::object::*;
use crate::stkidrel::*;
use crate::ObjectBase;

ObjectBase! {
#[derive(Debug, Copy, Clone)]
pub struct Table {
    pub flags: u8,
    pub lsizenode: u8,
    pub alimit: u32,
    pub array: *mut TValue,
    pub node: *mut Node,
    pub lastfree: *mut Node,
    pub metatable: *mut Table,
    pub gc_list: *mut Object,
}
}
