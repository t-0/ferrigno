use crate::tag::*;
use crate::tobject::*;
use crate::objectbase::*;
use std::ptr::*;
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct ObjectWithGCList {
    pub object: ObjectBase,
    pub gclist: *mut ObjectBase,
}
impl ObjectWithGCList {
    pub fn new(tagvariant: TagVariant) -> Self {
        Self { object: ObjectBase::new(tagvariant), gclist: null_mut(), .. }
    }
}
impl TObject for ObjectWithGCList {
    fn as_object(&self) -> &ObjectBase {
        &self.object
    }
    fn as_object_mut(&mut self) -> &mut ObjectBase {
        &mut self.object
    }
    fn get_class_name(&mut self) -> String {
        "gclist".to_string()
    }
    fn getgclist(& mut self) -> *mut *mut ObjectBase {
        &mut self.gclist
    }
}
