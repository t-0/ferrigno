use crate::object::*;
use crate::table::*;
use crate::uvalue::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Udata {
    pub next: *mut Object,
    pub tt: u8,
    pub marked: u8,
    pub nuvalue: u16,
    pub len: u64,
    pub metatable: *mut Table,
    pub gclist: *mut Object,
    pub uv: [UValue; 1],
}
