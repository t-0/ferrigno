use crate::object::*;
use crate::stkidrel::*;
use crate::functions::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CClosure {
    pub next: *mut Object,
    pub tag: u8,
    pub marked: u8,
    pub count_upvalues: u8,
    pub gc_list: *mut Object,
    pub f: CFunction,
    pub upvalue: [TValue; 1],
}
