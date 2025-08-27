use crate::value::*;
use crate::tag::*;
#[derive(Copy, Clone)]
pub struct TValue {
    pub value: Value,
    pub tag: u8,
}
impl TValue {
    pub fn get_tag(&self) -> u8 {
        self.tag
    }
    pub fn get_tag_type(&self) -> u8 {
        get_tag_type(self.get_tag())
    }
}
