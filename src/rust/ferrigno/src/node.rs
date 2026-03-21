#![allow(unpredictable_function_pointer_comparisons)]
use crate::tagvariant::*;
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
    /// Follow the `node_next` relative link to the next node in the hash chain.
    pub unsafe fn next_node(self_ptr: *mut Node) -> *mut Node {
        unsafe { self_ptr.offset((*self_ptr).node_next as isize) }
    }
}
pub unsafe fn equal_key(k1: *const TValue, node: *const Node, deadok: i32) -> bool {
    unsafe {
        if (*k1).get_tagvariant() != (*node).node_key.get_tagvariant()
            && !(deadok != 0 && (*node).node_key.get_tagvariant() == TagVariant::DeadKey && ((*k1).is_collectable()))
        {
            false
        } else {
            match (*node).node_key.get_tagvariant() {
                TagVariant::NilNil | TagVariant::BooleanFalse | TagVariant::BooleanTrue => true,
                TagVariant::NumericInteger => (*k1).as_integer().unwrap() == (*node).node_key.as_integer().unwrap(),
                TagVariant::NumericNumber => (*k1).as_number().unwrap() == (*node).node_key.as_number().unwrap(),
                TagVariant::Pointer => (*k1).as_pointer().unwrap() == (*node).node_key.as_pointer().unwrap(),
                TagVariant::ClosureCFunction => (*k1).as_function().unwrap() == (*node).node_key.as_function().unwrap(),
                TagVariant::StringLong => luas_eqlngstr(
                    &mut *(*k1).as_string().unwrap(),
                    &mut *(*node).node_key.as_string().unwrap(),
                ),
                _ => (*k1).raw_object_ptr() == (*node).node_key.raw_object_ptr(),
            }
        }
    }
}
