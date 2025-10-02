use crate::objectwithgclist::*;
pub trait TObjectWithGCList {
    fn getgclist(& mut self) -> *mut *mut ObjectWithGCList;
}
