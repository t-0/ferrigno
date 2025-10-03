use crate::global::*;
use crate::interpreter::*;
use crate::object::*;
use crate::tag::*;
use crate::tobject::*;
use crate::tobjectwithgclist::TObjectWithGCList;
use std::ptr::*;
pub type ObjectWithGCListSuper = Object;
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
            let mut current_block: usize;
            loop {
                let curr: *mut ObjectWithGCList = *objects;
                if curr.is_null() {
                    break;
                }
                let next: *mut *mut ObjectWithGCList = (*curr).getgclist();
                if !((*curr).get_marked() & (1 << 3 | 1 << 4) != 0) {
                    if (*curr).get_marked() & 7 == 5 {
                        (*curr).set_marked(((*curr).get_marked() | 1 << 5) as u8);
                        (*curr).set_marked(((*curr).get_marked() ^ (5 ^ 6)) as u8);
                        current_block = 11248371660297272285;
                    } else if (*curr).get_tagvariant() == TagVariant::Interpreter {
                        current_block = 11248371660297272285;
                    } else {
                        if (*curr).get_marked() & 7 == 6 {
                            (*curr).set_marked(((*curr).get_marked() ^ (6 ^ 4)) as u8);
                        }
                        (*curr).set_marked(((*curr).get_marked() | 1 << 5) as u8);
                        current_block = 6316553219439668466;
                    }
                    match current_block {
                        | 6316553219439668466 => {},
                        | _ => {
                            objects = next;
                            continue;
                        },
                    }
                }
                *objects = *next;
            }
            return objects;
        }
    }
    pub unsafe fn linkgclist_(object: *mut ObjectWithGCList, pnext: *mut *mut ObjectWithGCList, list: *mut *mut ObjectWithGCList) {
        unsafe {
            *pnext = *list;
            *list = object;
            (*object).set_marked((*object).get_marked() & !(1 << 5 | (1 << 3 | 1 << 4)));
        }
    }
    pub unsafe fn luac_barrierback_(interpreter: *mut Interpreter, object: *mut ObjectWithGCList) {
        unsafe {
            let global: *mut Global = (*interpreter).interpreter_global;
            if (*object).get_marked() & 7 == 6 {
                (*object).set_marked((*object).get_marked() & !(1 << 5 | (1 << 3 | 1 << 4)));
            } else {
                ObjectWithGCList::linkgclist_(
                    &mut (*(object as *mut ObjectWithGCList)),
                    (*object).getgclist(),
                    &mut (*global).global_grayagain,
                );
            }
            if (*object).get_marked() & 7 > 1 {
                (*object).set_marked((*object).get_marked() & !7 | 5);
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
    fn get_classname(&mut self) -> String {
        "gclist".to_string()
    }
}
impl TObjectWithGCList for ObjectWithGCList {
    fn getgclist(&mut self) -> *mut *mut ObjectWithGCList {
        &mut self.objectwithgclist_gclist
    }
}
