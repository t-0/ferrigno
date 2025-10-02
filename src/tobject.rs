#![allow(unused,dead_code)]
use crate::objectbase::*;
use crate::table::*;
use crate::tag::*;
use crate::object::*;
use std::ptr::*;
pub trait TObject {
    fn as_object(&self) -> &ObjectBase;
    fn as_object_mut(&mut self) -> &mut ObjectBase;
    fn get_class_name(&mut self) -> String;
    fn set_tag_variant(&mut self, tagvariant: TagVariant) {
        self.as_object_mut().set_tag_variant(tagvariant);
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
    fn get_tag_variant(&self) -> TagVariant {
        self.as_object().get_tag_variant()
    }
    fn is_tagtype_nil(&self) -> bool {
        self.get_tag_variant().to_tag_type() == TagType::Nil
    }
    fn is_tagtype_boolean(&self) -> bool {
        self.get_tag_variant().to_tag_type() == TagType::Boolean
    }
    fn is_tagtype_string(&self) -> bool {
        self.get_tag_variant().to_tag_type() == TagType::String
    }
    fn is_tagtype_numeric(&self) -> bool {
        self.get_tag_variant().to_tag_type() == TagType::Numeric
    }
    fn is_tagtype_closure(&self) -> bool {
        self.get_tag_variant().to_tag_type() == TagType::Closure
    }
}
