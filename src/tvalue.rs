use crate::new::*;
use crate::tag::*;
use crate::value::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TValue {
    pub value: Value,
    pub tag: u8,
}
impl New for TValue {
    fn new() -> Self {
        TValue {
            value: Value::new(),
            tag: TAG_VARIANT_NIL_NIL,
        }
    }
}
impl TValue {
    pub fn set_tag(&mut self, tag: u8) {
        self.tag = tag;
    }
    pub fn set_collectable(&mut self) {
        self.set_tag(set_collectable(self.get_tag()));
    }
    pub fn is_collectable(&self) -> bool {
        return is_collectable(self.get_tag());
    }
    pub fn get_tag(&self) -> u8 {
        self.tag
    }
    pub fn get_tag_type(&self) -> u8 {
        get_tag_type(self.get_tag())
    }
    pub fn get_tag_variant(&self) -> u8 {
        get_tag_variant(self.get_tag())
    }
}
