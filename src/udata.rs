use crate::object::*;
use crate::table::*;
use crate::uvalue::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Udata {
    pub next: *mut Object,
    pub tag: u8,
    pub marked: u8,
    pub nuvalue: u16,
    pub length: u64,
    pub metatable: *mut Table,
    pub gc_list: *mut Object,
    pub uv: [UValue; 1],
}
