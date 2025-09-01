#![allow(
    unpredictable_function_pointer_comparisons,
)]
use crate::tvalue::*;
use crate::tag::*;
use crate::value::*;
use crate::tstring::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Node {
    pub value: TValue,
    pub key: TValue,
    pub next: i32,
}
pub unsafe extern "C" fn clearkey(node: *mut Node) {
    unsafe {
        if is_collectable((*node).key.tag) {
            (*node).key.tag = (9 as i32 + 2) as u8;
        }
    }
}
pub const DUMMY_NODE: Node = Node {
    value: TValue {
        tag: TAG_VARIANT_NIL_EMPTY,
        value: Value {
            object: std::ptr::null_mut(),
        },
    },
    key: TValue {
        tag: TAG_VARIANT_NIL_NIL,
        value: Value {
            object: std::ptr::null_mut(),
        },
    },
    next: 0,
};
pub unsafe extern "C" fn equalkey(k1: *const TValue, node: *const Node, deadok: i32) -> i32 {
    unsafe {
        if (*k1).get_tag() != (*node).key.tag
            && !(deadok != 0 && (*node).key.tag == 9 + 2 && ((*k1).is_collectable()))
        {
            return 0;
        }
        match get_tag_variant((*node).key.tag) {
            TAG_VARIANT_NIL_NIL | TAG_VARIANT_BOOLEAN_FALSE | TAG_VARIANT_BOOLEAN_TRUE => return 1,
            TAG_VARIANT_NUMERIC_INTEGER => return ((*k1).value.i == (*node).key.value.i) as i32,
            TAG_VARIANT_NUMERIC_NUMBER => return ((*k1).value.n == (*node).key.value.n) as i32,
            TAG_VARIANT_POINTER => return ((*k1).value.p == (*node).key.value.p) as i32,
            TAG_VARIANT_CLOSURE_CFUNCTION => return ((*k1).value.f == (*node).key.value.f) as i32,
            TAG_VARIANT_STRING_LONG => {
                return luas_eqlngstr(
                    &mut (*((*k1).value.object as *mut TString)),
                    &mut (*((*node).key.value.object as *mut TString)),
                );
            }
            _ => return ((*k1).value.object == (*node).key.value.object) as i32,
        };
    }
}
