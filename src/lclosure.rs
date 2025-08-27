use crate::object::*;
use crate::prototype::*;
use crate::upvalue::*;
use crate::table::*;
use crate::tag::*;
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
    fn get_tag_type(&self) -> u8 {
        return get_tag_type(self.tag);
    }
    fn get_class_name(& mut self) -> String {
        "lclosure".to_string()
    }
    fn get_metatable(& mut self) -> *mut Table {
        std::ptr::null_mut()
    }
}
