#![allow(unpredictable_function_pointer_comparisons)]
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
pub const DUMMY_NODE: Node = Node {
    value: TValue {
        tag: TAG_VARIANT_NIL_EMPTY,
        value: Value {
            object: std::ptr::null_mut(),
        },
    },
    key: TValue {
        tag: TagVariant::NilNil as u8,
        value: Value {
            object: std::ptr::null_mut(),
        },
    },
    next: 0,
};
impl Node {
    pub fn clearkey(& mut self) {
        if is_collectable(self.key.tag) {
            self.key.tag = TAG_VARIANT_DEADKEY;
        }
    }
}
pub unsafe extern "C" fn equal_key(k1: *const TValue, node: *const Node, deadok: i32) -> bool {
    unsafe {
        return if (*k1).get_tag() != (*node).key.tag
            && !(deadok != 0 && (*node).key.tag == TAG_VARIANT_DEADKEY && ((*k1).is_collectable()))
        {
            false
        } else {
            match get_tag_variant((*node).key.tag) {
                TAG_VARIANT_NIL_NIL | TAG_VARIANT_BOOLEAN_FALSE | TAG_VARIANT_BOOLEAN_TRUE => true,
                TAG_VARIANT_NUMERIC_INTEGER => return (*k1).value.integer == (*node).key.value.integer,
                TAG_VARIANT_NUMERIC_NUMBER => return (*k1).value.number == (*node).key.value.number,
                TAG_VARIANT_POINTER => return (*k1).value.pointer == (*node).key.value.pointer,
                TAG_VARIANT_CLOSURE_CFUNCTION => return (*k1).value.function == (*node).key.value.function,
                TAG_VARIANT_STRING_LONG => {
                    luas_eqlngstr(
                        &mut (*((*k1).value.object as *mut TString)),
                        &mut (*((*node).key.value.object as *mut TString)),
                    )
                }
                _ => (*k1).value.object == (*node).key.value.object
            }
        }
    }
}
