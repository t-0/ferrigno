#![allow(unused, dead_code)]
use crate::object::*;
use crate::object::*;
use crate::table::*;
use crate::tag::*;
use std::ptr::*;
pub trait TObject {
    fn as_object(&self) -> &Object;
    fn as_object_mut(&mut self) -> &mut Object;
    fn get_classname(&mut self) -> String;
    fn set_tagvariant(&mut self, tagvariant: TagVariant) {
        self.as_object_mut().set_tagvariant(tagvariant);
    }
    fn get_marked(&self) -> u8 {
        self.as_object().get_marked()
    }
    fn set_marked(&mut self, marked: u8) {
        self.as_object_mut().set_marked(marked);
    }
    fn is_collectable(&self) -> bool {
        self.as_object().is_collectable()
    }
    fn set_collectable(&mut self, value: bool) {
        self.as_object_mut().set_collectable(value);
    }
    fn get_tagvariant(&self) -> TagVariant {
        self.as_object().get_tagvariant()
    }
    fn is_tagtype_nil(&self) -> bool {
        self.get_tagvariant().to_tag_type().is_nil()
    }
    fn is_tagtype_boolean(&self) -> bool {
        self.get_tagvariant().to_tag_type() == TagType::Boolean
    }
    fn is_tagtype_string(&self) -> bool {
        self.get_tagvariant().to_tag_type() == TagType::String
    }
    fn is_tagtype_numeric(&self) -> bool {
        self.get_tagvariant().to_tag_type() == TagType::Numeric
    }
    fn is_tagtype_closure(&self) -> bool {
        self.get_tagvariant().to_tag_type() == TagType::Closure
    }
}
