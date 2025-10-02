#![allow(dead_code, unused)]
// #[macro_export]
// macro_rules! ObjectBase {
//     (#[derive($($derive:meta),*)] $pub:vis struct $name:ident { $($fpub:vis $field:ident : $type:ty,)* }) => {
//         #[derive($($derive),*)]
//         #[repr(C)]
//         $pub struct $name {
//             pub next: *mut ObjectBase,
//             pub tag: u8,
//             pub marked: u8,
//             $($fpub $field : $type,)*
//         }
//     }
// }
use crate::closure::*;
use crate::closure::*;
use crate::global::*;
use crate::objectwithgclist::*;
use crate::objectwithmetatable::*;
use crate::interpreter::*;
use crate::objectwithmetatable::ObjectWithMetatable;
use crate::prototype::*;
use crate::table::*;
use crate::tag::*;
use crate::tstring::*;
use crate::tvalue::*;
use crate::upvalue::*;
use crate::tobject::*;
use crate::user::*;
use std::ptr::*;
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct ObjectBase {
    pub next: *mut ObjectBase = null_mut(),
    pub tagvariant: TagVariant = TagVariant::NilNil,
    pub marked: u8 = 0,
}
impl TObject for ObjectBase {
    fn as_object(&self) -> &ObjectBase {
        self
    }
    fn as_object_mut(&mut self) -> &mut ObjectBase {
        self
    }
    fn get_marked(&self) -> u8 {
        self.marked
    }
    fn set_marked(&mut self, marked: u8) {
        self.marked = marked;
    }
    fn get_tag_variant(&self) -> TagVariant {
        self.tagvariant
    }
    fn set_tag_variant(&mut self, tagvariant: TagVariant) {
        self.tagvariant = tagvariant;
    }
    fn get_class_name(&mut self) -> String {
        "object".to_string()
    }
    fn get_metatable(& self) -> *mut Table {
        unsafe {
            match self.get_tag_variant() {
                TagVariant::Table => return (*(self as *const ObjectBase as *const Table)).get_metatable(),
                TagVariant::User  => return (*(self as *const ObjectBase as *const ObjectWithMetatable as *const User)).get_metatable(),
                _ => return null_mut(),
            };
        }
    }
    fn getgclist(& mut self) -> *mut *mut ObjectBase {
        unsafe {
            match self.get_tag_variant() {
                TagVariant::Table | TagVariant::ClosureL | TagVariant::ClosureC | TagVariant::Interpreter | TagVariant::Prototype | TagVariant::User => return (*(self as *mut ObjectBase as *mut ObjectWithGCList)).getgclist(),
                _ => return null_mut(),
            };
        }
    }
}
impl ObjectBase {
    pub fn new(tagvariant: TagVariant) -> Self {
        Self { next: null_mut(), tagvariant: tagvariant, marked: 0, .. }
    }
}
