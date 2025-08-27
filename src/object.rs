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
use crate::tag::*;
pub trait TObject {
    fn get_tag_type(&self) -> u8;
    fn get_class_name(& mut self) -> String;
    fn get_metatable(& mut self) -> *mut Table;
}
#[derive(Copy, Clone)]
pub struct Object {
    pub next: *mut Object,
    pub tag: u8,
    pub marked: u8,
}
impl TObject for Object {
    fn get_tag_type(&self) -> u8 {
        return get_tag_type(self.tag);
    }
    fn get_class_name(& mut self) -> String {
        "object".to_string()
    }
    fn get_metatable(& mut self) -> *mut Table {
        std::ptr::null_mut()
    }
}
