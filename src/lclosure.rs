use crate::object::*;
use crate::prototype::*;
use crate::stkidrel::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LClosure {
    pub next: *mut Object,
    pub tag: u8,
    pub marked: u8,
    pub count_upvalues: u8,
    pub gc_list: *mut Object,
    pub p: *mut Prototype,
    pub upvalues: [*mut UpVal; 1],
}
