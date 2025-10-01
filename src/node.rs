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
pub const DUMMY_NODE: Node = Node { value: TValue::new(TagVariant::NilEmpty as u8), key: TValue::new(TagVariant::NilNil as u8), next: 0 };
impl Node {
    pub fn clearkey(&mut self) {
        if self.key.is_collectable() {
            self.key.set_tag_variant(TagVariant::DeadKey);
        }
    }
}
pub unsafe fn equal_key(k1: *const TValue, node: *const Node, deadok: i32) -> bool {
    unsafe {
        return if (*k1).get_tag_variant2() != (*node).key.get_tag_variant2() && !(deadok != 0 && (*node).key.get_tag_variant() == TagVariant::DeadKey as u8 && ((*k1).is_collectable())) {
            false
        } else {
            match (*node).key.get_tag_variant2() {
                TagVariant::NilNil | TagVariant::BooleanFalse | TagVariant::BooleanTrue => true,
                TagVariant::NumericInteger => return (*k1).value.value_integer == (*node).key.value.value_integer,
                TagVariant::NumericNumber => return (*k1).value.value_number == (*node).key.value.value_number,
                TagVariant::Pointer => return (*k1).value.value_pointer == (*node).key.value.value_pointer,
                TagVariant::ClosureCFunction => return (*k1).value.value_function == (*node).key.value.value_function,
                TagVariant::StringLong => luas_eqlngstr(&mut (*((*k1).value.value_object as *mut TString)), &mut (*((*node).key.value.value_object as *mut TString))),
                _ => (*k1).value.value_object == (*node).key.value.value_object,
            }
        };
    }
}
