use crate::global::*;
use crate::object::*;
use crate::state::*;
use crate::tagvariant::*;
use crate::tobject::*;
use crate::tobjectwithgclist::TObjectWithGCList;
use std::ptr::*;
type ObjectWithGCListSuper = Object;
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct ObjectWithGCList {
    objectwithgclist_super: ObjectWithGCListSuper,
    objectwithgclist_gclist: *mut ObjectWithGCList,
}
impl ObjectWithGCList {
    pub fn new(tagvariant: TagVariant) -> Self {
        Self {
            objectwithgclist_super: ObjectWithGCListSuper::new(tagvariant),
            objectwithgclist_gclist: null_mut(),
            ..
        }
    }
    pub unsafe fn correct_gray_list(mut objects: *mut *mut ObjectWithGCList) -> *mut *mut ObjectWithGCList {
        unsafe {
            const KEEP_IN_GRAY: usize = 0;
            const REMOVE_FROM_GRAY: usize = 1;
            let mut gray_action: usize;
            loop {
                let head: *mut ObjectWithGCList = *objects;
                if head.is_null() {
                    break;
                }
                let next: *mut *mut ObjectWithGCList = (*head).getgclist();
                if (*head).get_marked() & WHITEBITS == 0 {
                    if (*head).get_marked() & AGEBITS == AGE_TOUCHED1 {
                        (*head).set_marked((*head).get_marked() | BLACKBIT);
                        (*head).set_marked((*head).get_marked() ^ (AGE_TOUCHED1 ^ AGE_TOUCHED2));
                        gray_action = KEEP_IN_GRAY;
                    } else if (*head).get_tagvariant() == TagVariant::State {
                        gray_action = KEEP_IN_GRAY;
                    } else {
                        if (*head).get_marked() & AGEBITS == AGE_TOUCHED2 {
                            (*head).set_marked((*head).get_marked() ^ (AGE_TOUCHED2 ^ AGE_OLD));
                        }
                        (*head).set_marked((*head).get_marked() | BLACKBIT);
                        gray_action = REMOVE_FROM_GRAY;
                    }
                    match gray_action {
                        | REMOVE_FROM_GRAY => {},
                        | _ => {
                            objects = next;
                            continue;
                        },
                    }
                }
                *objects = *next;
            }
            objects
        }
    }
    pub unsafe fn linkgclist_(object: *mut ObjectWithGCList, pnext: *mut *mut ObjectWithGCList, list: *mut *mut ObjectWithGCList) {
        unsafe {
            *pnext = *list;
            *list = object;
            (*object).set_marked((*object).get_marked() & !(BLACKBIT | WHITEBITS));
        }
    }
    pub unsafe fn luac_barrierback_(state: *mut State, object: *mut ObjectWithGCList) {
        unsafe {
            let global: *mut Global = (*state).interpreter_global;
            if (*object).get_marked() & AGEBITS == AGE_TOUCHED2 {
                (*object).set_marked((*object).get_marked() & !(BLACKBIT | WHITEBITS));
            } else {
                ObjectWithGCList::linkgclist_(
                    &mut *(object as *mut ObjectWithGCList),
                    (*object).getgclist(),
                    &mut (*global).global_grayagain,
                );
            }
            if (*object).get_marked() & AGEBITS > AGE_SURVIVAL {
                (*object).set_marked((*object).get_marked() & !AGEBITS | AGE_TOUCHED1);
            }
        }
    }
}
impl TObject for ObjectWithGCList {
    fn as_object(&self) -> &Object {
        &self.objectwithgclist_super
    }
    fn as_object_mut(&mut self) -> &mut Object {
        &mut self.objectwithgclist_super
    }
}
impl TObjectWithGCList for ObjectWithGCList {
    fn getgclist(&mut self) -> *mut *mut ObjectWithGCList {
        &mut self.objectwithgclist_gclist
    }
}
