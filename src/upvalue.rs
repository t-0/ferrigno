use crate::object::*;
use crate::table::*;
use crate::tag::*;
use crate::tvalue::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UpValue {
    pub object: Object,
    pub v: UpValueA,
    pub u: UpValueB,
}
impl TObject for UpValue {
    fn get_marked(&self) -> u8 {
        self.object.get_marked()
    }
    fn set_marked(&mut self, marked_: u8) {
        self.object.set_marked(marked_);
    }
    fn set_tag(&mut self, tag: u8) {
        self.object.set_tag(tag);
    }
    fn set_collectable(&mut self) {
        self.object.set_collectable();
    }
    fn is_collectable(&self) -> bool {
        return self.object.is_collectable();
    }
    fn get_tag(&self) -> u8 {
        return self.object.get_tag();
    }
    fn get_tag_type(&self) -> u8 {
        return get_tag_type(self.get_tag());
    }
    fn get_tag_variant(&self) -> u8 {
        get_tag_variant(self.get_tag())
    }
    fn get_class_name(&mut self) -> String {
        "upvalue".to_string()
    }
    fn get_metatable(&mut self) -> *mut Table {
        std::ptr::null_mut()
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union UpValueA {
    pub p: *mut TValue,
    pub offset: i64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union UpValueB {
    pub open: UpValueBA,
    pub value: TValue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UpValueBA {
    pub next: *mut UpValue,
    pub previous: *mut *mut UpValue,
}
