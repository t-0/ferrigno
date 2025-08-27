use crate::object::*;
use crate::tvalue::*;
use crate::table::*;
use crate::tag::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UpValue {
    pub next: *mut Object,
    pub tag: u8,
    pub marked: u8,
    pub v: UpValueA,
    pub u: UpValueB,
}
impl TObject for UpValue {
    fn get_tag_type(&self) -> u8 {
        get_tag_type(self.tag)
    }
    fn get_class_name(& mut self) -> String {
        "upvalue".to_string()
    }
    fn get_metatable(& mut self) -> *mut Table {
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
