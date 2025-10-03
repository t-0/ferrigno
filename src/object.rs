#![allow(dead_code, unused)]
// #[macro_export]
// macro_rules! Object {
//     (#[derive($($derive:meta),*)] $pub:vis struct $name:ident { $($fpub:vis $field:ident : $type:ty,)* }) => {
//         #[derive($($derive),*)]
//         #[repr(C)]
//         $pub struct $name {
//             pub next: *mut Object,
//             pub tag: u8,
//             pub marked: u8,
//             $($fpub $field : $type,)*
//         }
//     }
// }
use crate::closure::*;
use crate::closure::*;
use crate::closure::*;
use crate::closure::*;
use crate::global::*;
use crate::global::*;
use crate::interpreter::*;
use crate::interpreter::*;
use crate::objectwithgclist::*;
use crate::objectwithgclist::*;
use crate::objectwithmetatable::ObjectWithMetatable;
use crate::objectwithmetatable::*;
use crate::objectwithmetatable::*;
use crate::prototype::*;
use crate::prototype::*;
use crate::table::*;
use crate::table::*;
use crate::tag::*;
use crate::tag::*;
use crate::tobject::*;
use crate::tobject::*;
use crate::tobject::*;
use crate::tobjectwithgclist::*;
use crate::tobjectwithmetatable::*;
use crate::tstring::*;
use crate::tstring::*;
use crate::tvalue::*;
use crate::tvalue::*;
use crate::upvalue::*;
use crate::upvalue::*;
use crate::user::*;
use crate::user::*;
use std::ptr::*;
use std::ptr::*;
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Object {
    pub object_next: *mut Object = null_mut(),
    pub object_tagvariant: TagVariant = TagVariant::NilNil,
    pub object_marked: u8 = 0,
}
impl TObject for Object {
    fn as_object(&self) -> &Object {
        self
    }
    fn as_object_mut(&mut self) -> &mut Object {
        self
    }
    fn get_marked(&self) -> u8 {
        self.object_marked
    }
    fn set_marked(&mut self, marked: u8) {
        self.object_marked = marked;
    }
    fn get_tagvariant(&self) -> TagVariant {
        self.object_tagvariant
    }
    fn set_tagvariant(&mut self, tagvariant: TagVariant) {
        self.object_tagvariant = tagvariant;
    }
}
impl Object {
    pub fn new(tagvariant: TagVariant) -> Self {
        Self { object_next: null_mut(), object_tagvariant: tagvariant, object_marked: 0, .. }
    }
    pub unsafe fn iscleared(global: *mut Global, object: *const Object) -> i32 {
        unsafe {
            if object.is_null() {
                return 0;
            } else if (*object).get_tagvariant().to_tag_type().is_string() {
                if (*object).get_marked() & (1 << 3 | 1 << 4) != 0 {
                    Object::really_mark_object(global, &mut (*(object as *mut Object)));
                }
                return 0;
            } else {
                return ((*object).get_marked() & (1 << 3 | 1 << 4)) as i32;
            };
        }
    }
    pub unsafe fn luac_barrier_(interpreter: *mut Interpreter, object: *mut Object, v: *mut Object) {
        unsafe {
            let global: *mut Global = (*interpreter).interpreter_global;
            if (*global).global_gcstate as i32 <= 2 {
                Object::really_mark_object(global, v);
                if (*object).get_marked() & 7 > 1 {
                    (*v).set_marked((*v).get_marked() & !(7) | 2);
                }
            } else if (*global).global_gckind as i32 == 0 {
                (*object).set_marked(
                    (*object).get_marked() & !(1 << 5 | (1 << 3 | 1 << 4)) | ((*global).global_currentwhite & (1 << 3 | 1 << 4)),
                );
            }
        }
    }
    pub unsafe fn fix_object_global(global: *mut Global, object: *mut Object) {
        unsafe {
            (*object).set_marked((*object).get_marked() & !(1 << 5 | (1 << 3 | 1 << 4)));
            (*object).set_marked((*object).get_marked() & !(7) | 4);
            (*global).global_allgc = (*object).object_next;
            (*object).object_next = (*global).global_fixedgc;
            (*global).global_fixedgc = object;
        }
    }
    pub unsafe fn really_mark_object(global: *mut Global, object: *mut Object) {
        unsafe {
            let current_block_18: usize;
            match (*object).get_tagvariant() {
                | TagVariant::StringShort | TagVariant::StringLong => {
                    (*object).set_marked((*object).get_marked() & !(1 << 3 | 1 << 4) | 1 << 5);
                    current_block_18 = 18317007320854588510;
                },
                | TagVariant::UpValue => {
                    let uv: *mut UpValue = &mut (*(object as *mut UpValue));
                    if (*uv).upvalue_v.upvaluea_p != &mut (*uv).upvalue_u.upvalueb_value as *mut TValue {
                        (*uv).set_marked((*uv).get_marked() & !(1 << 5 | (1 << 3 | 1 << 4)));
                    } else {
                        (*uv).set_marked(((*uv).get_marked() & !(1 << 3 | 1 << 4) | 1 << 5) as u8);
                    }
                    if ((*(*uv).upvalue_v.upvaluea_p).is_collectable())
                        && (*(*(*uv).upvalue_v.upvaluea_p).tvalue_value.value_object).get_marked() & (1 << 3 | 1 << 4) != 0
                    {
                        Object::really_mark_object(global, (*(*uv).upvalue_v.upvaluea_p).tvalue_value.value_object);
                    }
                    current_block_18 = 18317007320854588510;
                },
                | TagVariant::User => {
                    let user: *mut User = &mut (*(object as *mut User));
                    if (*user).user_countupvalues as i32 == 0 {
                        if !((*user).get_metatable()).is_null() {
                            if (*(*user).get_metatable()).get_marked() & (1 << 3 | 1 << 4) != 0 {
                                Object::really_mark_object(global, &mut (*((*user).get_metatable() as *mut Object)));
                            }
                        }
                        (*user).set_marked((*user).get_marked() & !(1 << 3 | 1 << 4) | 1 << 5);
                        current_block_18 = 18317007320854588510;
                    } else {
                        current_block_18 = 15904375183555213903;
                    }
                },
                | TagVariant::ClosureL
                | TagVariant::ClosureC
                | TagVariant::Table
                | TagVariant::Interpreter
                | TagVariant::Prototype => {
                    current_block_18 = 15904375183555213903;
                },
                | _ => {
                    current_block_18 = 18317007320854588510;
                },
            }
            match current_block_18 {
                | 15904375183555213903 => {
                    ObjectWithGCList::linkgclist_(
                        &mut (*(object as *mut ObjectWithGCList)),
                        (*(object as *mut ObjectWithGCList)).getgclist(),
                        &mut (*global).global_gray,
                    );
                },
                | _ => {},
            };
        }
    }
    pub unsafe fn generate_link(global: *mut Global, object: *mut Object) {
        unsafe {
            if (*object).get_marked() & 7 == 5 {
                ObjectWithGCList::linkgclist_(
                    &mut (*(object as *mut ObjectWithGCList)),
                    (*(object as *mut ObjectWithGCList)).getgclist(),
                    &mut (*global).global_grayagain,
                );
            } else if (*object).get_marked() & 7 == 6 {
                (*object).set_marked(((*object).get_marked() ^ (6 ^ 4)) as u8);
            }
        }
    }
    pub unsafe fn object_free(interpreter: *mut Interpreter, object: *mut Object) {
        unsafe {
            match (*object).get_tagvariant() {
                | TagVariant::Prototype => {
                    let prototype: *mut Prototype = &mut (*(object as *mut Prototype));
                    (*prototype).prototype_free(interpreter);
                },
                | TagVariant::UpValue => {
                    let upvalue: *mut UpValue = &mut (*(object as *mut UpValue));
                    (*upvalue).upvalue_free(interpreter);
                },
                | TagVariant::ClosureL | TagVariant::ClosureC => {
                    let closure: *mut Closure = &mut (*(object as *mut Closure));
                    (*closure).closure_free(interpreter);
                },
                | TagVariant::Table => {
                    let table: *mut Table = &mut (*(object as *mut Table));
                    (*table).table_free(interpreter);
                },
                | TagVariant::Interpreter => {
                    let other: *mut Interpreter = &mut (*(object as *mut Interpreter));
                    (*other).interpreter_free(interpreter);
                },
                | TagVariant::User => {
                    let user: *mut User = &mut (*(object as *mut User));
                    (*user).user_free(interpreter);
                },
                | TagVariant::StringLong | TagVariant::StringShort => {
                    let tstring: *mut TString = &mut (*(object as *mut TString));
                    (*tstring).tstring_free(interpreter);
                },
                | _ => {},
            };
        }
    }
    pub unsafe fn find_last(mut objects: *mut *mut Object) -> *mut *mut Object {
        unsafe {
            while !(*objects).is_null() {
                objects = &mut (**objects).object_next;
            }
            return objects;
        }
    }
    pub unsafe fn check_pointer(objects: *mut *mut Object, object: *mut Object) {
        unsafe {
            if object == *objects {
                *objects = (*object).object_next;
            }
        }
    }
    pub unsafe fn delete_list(interpreter: *mut Interpreter, mut head: *mut Object, stop: *mut Object) {
        unsafe {
            while head != stop {
                let next = (*head).object_next;
                Object::object_free(interpreter, head);
                head = next;
            }
        }
    }
}
