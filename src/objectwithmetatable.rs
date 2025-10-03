use crate::object::*;
use crate::objectwithgclist::*;
use crate::table::*;
use crate::tag::*;
use crate::tobject::*;
use crate::tobjectwithgclist::*;
use crate::tobjectwithmetatable::*;
use std::ptr::*;
pub type ObjectWithMetatableSuper = ObjectWithGCList;
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct ObjectWithMetatable {
    objectwithmetatable_super: ObjectWithMetatableSuper,
    objectwithmetatable_metatable: *mut Table,
}
impl TObject for ObjectWithMetatable {
    fn as_object(&self) -> &Object {
        self.objectwithmetatable_super.as_object()
    }
    fn as_object_mut(&mut self) -> &mut Object {
        self.objectwithmetatable_super.as_object_mut()
    }
    fn get_classname(&mut self) -> String {
        "objectwithmetatable".to_string()
    }
}
impl ObjectWithMetatable {
    pub fn new(tagvariant: TagVariant) -> Self {
        Self {
            objectwithmetatable_super: ObjectWithMetatableSuper::new(tagvariant),
            objectwithmetatable_metatable: null_mut(),
        }
    }
}
impl TObjectWithGCList for ObjectWithMetatable {
    fn getgclist(&mut self) -> *mut *mut ObjectWithGCList {
        self.objectwithmetatable_super.getgclist()
    }
}
impl TObjectWithMetatable for ObjectWithMetatable {
    fn set_metatable(&mut self, metatable: *mut Table) {
        self.objectwithmetatable_metatable = metatable;
    }
    fn get_metatable(&self) -> *mut Table {
        self.objectwithmetatable_metatable
    }
}
