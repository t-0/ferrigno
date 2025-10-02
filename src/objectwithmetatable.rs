use crate::objectwithgclist::*;
use crate::objectbase::*;
use crate::table::*;
use crate::tag::*;
use crate::tobject::*;
use crate::tobjectwithgclist::TObjectWithGCList;
use std::ptr::*;
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct ObjectWithMetatable {
    pub object: ObjectWithGCList,
    pub metatable: *mut Table,
}
impl TObject for ObjectWithMetatable {
    fn as_object(&self) -> &ObjectBase {
        self.object.as_object()
    }
    fn as_object_mut(&mut self) -> &mut ObjectBase {
        self.object.as_object_mut()
    }
    fn get_class_name(&mut self) -> String {
        "objectwithmetatable".to_string()
    }
    fn get_metatable(&self) -> *mut Table {
        self.metatable
    }
    fn set_metatable(&mut self, metatable:* mut Table) {
        self.metatable = metatable;
    }
}
impl ObjectWithMetatable {
    pub fn new(tagvariant: TagVariant) -> Self {
        Self { object: ObjectWithGCList::new (tagvariant), metatable: null_mut(), }
    }
}
impl TObjectWithGCList for ObjectWithMetatable {
    fn getgclist(&mut self) -> *mut *mut ObjectWithGCList {
        self.object.getgclist()
    }
}
