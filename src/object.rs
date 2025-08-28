// #[macro_export]
// macro_rules! ObjectBase {
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
use crate::table::*;
use crate::new::*;
use crate::tag::*;
pub trait TObject {
    fn get_marked(& self) -> u8;
    fn set_marked(& mut self, marked_: u8);
    fn set_tag(& mut self, tag: u8);
    fn is_collectable(&self) -> bool;
    fn get_tag(&self) -> u8;
    fn get_tag_type(&self) -> u8;
    fn get_tag_variant(&self) -> u8;
    fn get_class_name(& mut self) -> String;
    fn get_metatable(& mut self) -> *mut Table;
}
#[derive(Copy, Clone, Debug)]
pub struct Object {
    pub next: *mut Object,
    pub tag: u8,
    pub marked: u8,
}
impl New for Object {
    fn new() -> Self {
        Object {
            next: std::ptr::null_mut(),
            tag: TAG_VARIANT_NIL_NIL,
            marked: 0,
        }
    }
}
impl TObject for Object {
    fn get_marked(& self) -> u8 {
        self.marked
    }
    fn set_marked(& mut self, marked_: u8) {
        self.marked = marked_;
    }
    fn set_tag(& mut self, tag: u8) {
        self.tag = tag;
    }
    fn is_collectable(&self) -> bool {
        return is_collectable(self.tag);
    }
    fn get_tag(&self) -> u8 {
        return self.tag;
    }
    fn get_tag_type(&self) -> u8 {
        return get_tag_type(self.get_tag());
    }
    fn get_tag_variant(&self) -> u8 {
        get_tag_variant(self.get_tag())
    }
    fn get_class_name(& mut self) -> String {
        "object".to_string()
    }
    fn get_metatable(& mut self) -> *mut Table {
        std::ptr::null_mut()
    }
}
