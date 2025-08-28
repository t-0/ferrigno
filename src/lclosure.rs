use crate::object::*;
use crate::prototype::*;
use crate::table::*;
use crate::tag::*;
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
    fn get_marked(&self) -> u8 {
        self.marked
    }
    fn set_marked(&mut self, marked_: u8) {
        self.marked = marked_;
    }
    fn set_tag(&mut self, tag: u8) {
        self.tag = tag;
    }
    fn is_collectable(&self) -> bool {
        return is_collectable(self.get_tag());
    }
    fn set_collectable(&mut self) {
        self.set_tag(set_collectable(self.tag));
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
    fn get_class_name(&mut self) -> String {
        "lclosure".to_string()
    }
    fn get_metatable(&mut self) -> *mut Table {
        std::ptr::null_mut()
    }
}
