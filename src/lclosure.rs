use crate::object::*;
use crate::prototype::*;
use crate::upvalue::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LClosure {
    pub next: *mut Object,
    pub tag: u8,
    pub marked: u8,
    pub count_upvalues: u8,
    pub gc_list: *mut Object,
    pub p: *mut Prototype,
    pub upvalues: [*mut UpValue; 1],
}
impl TObject for LClosure {
    fn get_class_name() -> String {
        "LClosure".to_string()
    }
}
