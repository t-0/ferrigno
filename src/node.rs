#![allow(unpredictable_function_pointer_comparisons)]
use crate::tag::*;
use crate::tstring::*;
use crate::tvalue::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Node {
    pub value: TValue,
    pub key: TValue,
    pub next: i32,
}
pub const DUMMY_NODE: Node = Node { value: TValue::new(TAG_VARIANT_NIL_EMPTY), key: TValue::new(TAG_VARIANT_NIL_NIL), next: 0 };
impl Node {
    pub fn clearkey(&mut self) {
        if self.key.is_collectable() {
            self.key.set_tag_variant(TAG_VARIANT_DEADKEY);
        }
    }
}
pub unsafe fn equal_key(k1: *const TValue, node: *const Node, deadok: i32) -> bool {
    unsafe {
        return if (*k1).get_tag_variant() != (*node).key.get_tag_variant() && !(deadok != 0 && (*node).key.get_tag_variant() == TAG_VARIANT_DEADKEY && ((*k1).is_collectable())) {
            false
        } else {
            match (*node).key.get_tag_variant() {
                TAG_VARIANT_NIL_NIL | TAG_VARIANT_BOOLEAN_FALSE | TAG_VARIANT_BOOLEAN_TRUE => true,
                TAG_VARIANT_NUMERIC_INTEGER => return (*k1).value.value_integer == (*node).key.value.value_integer,
                TAG_VARIANT_NUMERIC_NUMBER => return (*k1).value.value_number == (*node).key.value.value_number,
                TAG_VARIANT_POINTER => return (*k1).value.value_pointer == (*node).key.value.value_pointer,
                TAG_VARIANT_CLOSURE_CFUNCTION => return (*k1).value.value_function == (*node).key.value.value_function,
                TAG_VARIANT_STRING_LONG => luas_eqlngstr(&mut (*((*k1).value.value_object as *mut TString)), &mut (*((*node).key.value.value_object as *mut TString))),
                _ => (*k1).value.value_object == (*node).key.value.value_object,
            }
        };
    }
}
