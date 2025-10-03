#![allow(unpredictable_function_pointer_comparisons)]
use crate::tag::*;
use crate::tstring::*;
use crate::tvalue::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Node {
    pub node_value: TValue,
    pub node_key: TValue,
    pub node_next: i32,
}
pub const DUMMY_NODE: Node = Node {
    node_value: TValue::new(TagVariant::NilEmpty),
    node_key: TValue::new(TagVariant::NilNil),
    node_next: 0,
};
impl Node {
    pub fn clearkey(&mut self) {
        if self.node_key.is_collectable() {
            self.node_key.tvalue_set_tag_variant(TagVariant::DeadKey);
        }
    }
}
pub unsafe fn equal_key(k1: *const TValue, node: *const Node, deadok: i32) -> bool {
    unsafe {
        return if (*k1).get_tagvariant() != (*node).node_key.get_tagvariant()
            && !(deadok != 0 && (*node).node_key.get_tagvariant() == TagVariant::DeadKey && ((*k1).is_collectable()))
        {
            false
        } else {
            match (*node).node_key.get_tagvariant() {
                | TagVariant::NilNil | TagVariant::BooleanFalse | TagVariant::BooleanTrue => true,
                | TagVariant::NumericInteger => {
                    return (*k1).tvalue_value.value_integer == (*node).node_key.tvalue_value.value_integer;
                },
                | TagVariant::NumericNumber => {
                    return (*k1).tvalue_value.value_number == (*node).node_key.tvalue_value.value_number;
                },
                | TagVariant::Pointer => return (*k1).tvalue_value.value_pointer == (*node).node_key.tvalue_value.value_pointer,
                | TagVariant::ClosureCFunction => {
                    return (*k1).tvalue_value.value_function == (*node).node_key.tvalue_value.value_function;
                },
                | TagVariant::StringLong => luas_eqlngstr(
                    &mut (*((*k1).tvalue_value.value_object as *mut TString)),
                    &mut (*((*node).node_key.tvalue_value.value_object as *mut TString)),
                ),
                | _ => (*k1).tvalue_value.value_object == (*node).node_key.tvalue_value.value_object,
            }
        };
    }
}
