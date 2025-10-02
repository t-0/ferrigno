use crate::objectwithgclist::*;
use crate::object::*;
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
    pub super_: ObjectWithMetatableSuper,
    pub metatable: *mut Table,
}
impl TObject for ObjectWithMetatable {
    fn as_object(&self) -> &Object {
        self.super_.as_object()
    }
    fn as_object_mut(&mut self) -> &mut Object {
        self.super_.as_object_mut()
    }
    fn get_class_name(&mut self) -> String {
        "objectwithmetatable".to_string()
    }
}
impl ObjectWithMetatable {
    pub fn new(tagvariant: TagVariant) -> Self {
        Self { super_: ObjectWithMetatableSuper::new (tagvariant), metatable: null_mut(), }
    }
}
impl TObjectWithGCList for ObjectWithMetatable {
    fn getgclist(&mut self) -> *mut *mut ObjectWithGCList {
        self.super_.getgclist()
    }
}
impl TObjectWithMetatable for ObjectWithMetatable {
    fn set_metatable (&mut self, metatable: *mut Table) {
        self.metatable = metatable;
    }
    fn get_metatable(&self) -> *mut Table {
        self.metatable
    }
}
