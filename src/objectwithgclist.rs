use crate::tag::*;
use crate::tobject::*;
use crate::object::*;
use crate::tobjectwithgclist::TObjectWithGCList;
use std::ptr::*;
pub type ObjectWithGCListSuper = Object;
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct ObjectWithGCList {
    pub super_: ObjectWithGCListSuper,
    pub gclist: *mut ObjectWithGCList,
}
impl ObjectWithGCList {
    pub fn new(tagvariant: TagVariant) -> Self {
        Self { super_: ObjectWithGCListSuper::new(tagvariant), gclist: null_mut(), .. }
    }
}
impl TObject for ObjectWithGCList {
    fn as_object(&self) -> &Object {
        &self.super_
    }
    fn as_object_mut(&mut self) -> &mut Object {
        &mut self.super_
    }
    fn get_class_name(&mut self) -> String {
        "gclist".to_string()
    }
}
impl TObjectWithGCList for ObjectWithGCList {
    fn getgclist(& mut self) -> *mut *mut ObjectWithGCList {
        &mut self.gclist
    }
}
