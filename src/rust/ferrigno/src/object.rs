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
use crate::gckind::*;
use crate::global::*;
use crate::global::*;
use crate::node::*;
use crate::objectwithgclist::*;
use crate::objectwithgclist::*;
use crate::objectwithmetatable::ObjectWithMetatable;
use crate::objectwithmetatable::*;
use crate::objectwithmetatable::*;
use crate::prototype::*;
use crate::prototype::*;
use crate::state::*;
use crate::state::*;
use crate::table::*;
use crate::table::*;
use crate::tagtype::*;
use crate::tagtype::*;
use crate::tagvariant::*;
use crate::tagvariant::*;
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
                0
            } else if (*object).get_tagvariant().to_tag_type().is_string() {
                if (*object).get_marked() & WHITEBITS != 0 {
                    Object::really_mark_object(global, object as *mut Object);
                }
                0
            } else {
                ((*object).get_marked() & WHITEBITS) as i32
            }
        }
    }
    pub unsafe fn luac_barrier_(state: *mut State, object: *mut Object, v: *mut Object) {
        unsafe {
            let global: *mut Global = (*state).interpreter_global;
            if (*global).global_gcstate as i32 <= GCS_ATOMIC {
                Object::really_mark_object(global, v);
                if (*object).get_marked() & AGEBITS > AGE_SURVIVAL {
                    (*v).set_marked((*v).get_marked() & !AGEBITS | AGE_OLD0);
                }
            } else {
                if (*global).global_gckind != GCKind::GenerationalMinor {
                    (*object).set_marked(
                        (*object).get_marked() & !(BLACKBIT | WHITEBITS) | ((*global).global_current_white & WHITEBITS),
                    );
                }
            }
        }
    }
    pub unsafe fn fix_object_global(global: *mut Global, object: *mut Object) {
        unsafe {
            (*object).set_marked((*object).get_marked() & !(BLACKBIT | WHITEBITS));
            (*object).set_marked((*object).get_marked() & !AGEBITS | AGE_OLD);
            (*global).global_allgc = (*object).object_next;
            (*object).object_next = (*global).global_fixedgc;
            (*global).global_fixedgc = object;
        }
    }
    pub unsafe fn really_mark_object(global: *mut Global, object: *mut Object) {
        unsafe {
            (*global).global_gcmarked += Object::objsize(object);
            const MARK_DONE: usize = 0;
            const LINK_TO_GRAY: usize = 1;
            let mark_action: usize;
            match (*object).get_tagvariant() {
                | TagVariant::StringShort | TagVariant::StringLong => {
                    (*object).set_marked((*object).get_marked() & !WHITEBITS | BLACKBIT);
                    mark_action = MARK_DONE;
                },
                | TagVariant::UpValue => {
                    let uv: *mut UpValue = object as *mut UpValue;
                    if (*uv).upvalue_v.upvaluea_p != std::ptr::addr_of_mut!((*uv).upvalue_u.upvalueb_value) {
                        (*uv).set_marked((*uv).get_marked() & !(BLACKBIT | WHITEBITS));
                    } else {
                        (*uv).set_marked((*uv).get_marked() & !WHITEBITS | BLACKBIT);
                    }
                    if let Some(obj) = (*(*uv).upvalue_v.upvaluea_p).as_object()
                        && (*obj).get_marked() & WHITEBITS != 0
                    {
                        Object::really_mark_object(global, obj);
                    }
                    mark_action = MARK_DONE;
                },
                | TagVariant::User => {
                    let user: *mut User = object as *mut User;
                    if (*user).user_countupvalues == 0 {
                        if !((*user).get_metatable()).is_null() && (*(*user).get_metatable()).get_marked() & WHITEBITS != 0 {
                            Object::really_mark_object(global, (*user).get_metatable() as *mut Object);
                        }
                        (*user).set_marked((*user).get_marked() & !WHITEBITS | BLACKBIT);
                        mark_action = MARK_DONE;
                    } else {
                        mark_action = LINK_TO_GRAY;
                    }
                },
                | TagVariant::ClosureL | TagVariant::ClosureC | TagVariant::Table | TagVariant::State | TagVariant::Prototype => {
                    mark_action = LINK_TO_GRAY;
                },
                | _ => {
                    mark_action = MARK_DONE;
                },
            }
            match mark_action {
                | LINK_TO_GRAY => {
                    ObjectWithGCList::linkgclist_(
                        object as *mut ObjectWithGCList,
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
            if (*object).get_marked() & AGEBITS == AGE_TOUCHED1 {
                ObjectWithGCList::linkgclist_(
                    object as *mut ObjectWithGCList,
                    (*(object as *mut ObjectWithGCList)).getgclist(),
                    &mut (*global).global_grayagain,
                );
            } else if (*object).get_marked() & AGEBITS == AGE_TOUCHED2 {
                (*object).set_marked((*object).get_marked() ^ (AGE_TOUCHED2 ^ AGE_OLD));
            }
        }
    }
    pub unsafe fn objsize(object: *mut Object) -> i64 {
        unsafe {
            match (*object).get_tagvariant() {
                | TagVariant::Table => {
                    let t = object as *mut Table;
                    let mut s = size_of::<Table>() as i64;
                    s += concretesize((*t).table_a_size) as i64;
                    if (*t).table_log_size_node > 0 {
                        s += ((1i64 << (*t).table_log_size_node as i32) * size_of::<Node>() as i64) as i64;
                    }
                    s
                },
                | TagVariant::ClosureL => {
                    let cl = object as *mut Closure;
                    (size_of::<Closure>() + (*cl).closure_count_upvalues as usize * size_of::<*mut UpValue>()) as i64
                },
                | TagVariant::ClosureC => {
                    let cl = object as *mut Closure;
                    (size_of::<Closure>() + (*cl).closure_count_upvalues as usize * size_of::<TValue>()) as i64
                },
                | TagVariant::User => {
                    let u = object as *mut User;
                    (size_of::<User>() + (*u).user_countbytes) as i64
                },
                | TagVariant::State => size_of::<State>() as i64,
                | TagVariant::Prototype => size_of::<Prototype>() as i64,
                | TagVariant::StringShort | TagVariant::StringLong => {
                    let ts = object as *mut TString;
                    (size_of::<TString>() + (*ts).get_length() + 1) as i64
                },
                | TagVariant::UpValue => size_of::<UpValue>() as i64,
                | _ => size_of::<Object>() as i64,
            }
        }
    }
    pub unsafe fn object_free(state: *mut State, object: *mut Object) {
        unsafe {
            match (*object).get_tagvariant() {
                | TagVariant::Prototype => {
                    let prototype: *mut Prototype = object as *mut Prototype;
                    (*prototype).prototype_free(state);
                },
                | TagVariant::UpValue => {
                    let upvalue: *mut UpValue = object as *mut UpValue;
                    (*upvalue).upvalue_free(state);
                },
                | TagVariant::ClosureL | TagVariant::ClosureC => {
                    let closure: *mut Closure = object as *mut Closure;
                    (*closure).closure_free(state);
                },
                | TagVariant::Table => {
                    let table: *mut Table = object as *mut Table;
                    (*table).table_free(state);
                },
                | TagVariant::State => {
                    let other: *mut State = object as *mut State;
                    (*other).interpreter_free(state);
                },
                | TagVariant::User => {
                    let user: *mut User = object as *mut User;
                    (*user).user_free(state);
                },
                | TagVariant::StringLong | TagVariant::StringShort => {
                    let tstring: *mut TString = object as *mut TString;
                    (*tstring).tstring_free(state);
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
            objects
        }
    }
    pub unsafe fn check_pointer(objects: *mut *mut Object, object: *mut Object) {
        unsafe {
            if object == *objects {
                *objects = (*object).object_next;
            }
        }
    }
    pub unsafe fn delete_list(state: *mut State, mut head: *mut Object, stop: *mut Object) {
        unsafe {
            while head != stop {
                let next = (*head).object_next;
                Object::object_free(state, head);
                head = next;
            }
        }
    }
}
