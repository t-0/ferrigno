use crate::object::*;
use crate::table::*;
use crate::uvalue::*;
use crate::tag::*;
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
impl TObject for Udata {
    fn set_tag(& mut self, tag: u8) {
        self.tag = tag;
    }
    fn is_collectable(&self) -> bool {
        return is_collectable(self.tag);
    }
    fn get_tag(&self) -> u8 {
        return self.tag;
    }
    fn get_tag_type(&self) -> u8 {
        return get_tag_type(self.get_tag());
    }
    fn get_tag_variant(&self) -> u8 {
        get_tag_variant(self.get_tag())
    }
    fn get_class_name(& mut self) -> String {
        "user".to_string()
    }
    fn get_metatable(& mut self) -> *mut Table {
        self.metatable
    }
}
