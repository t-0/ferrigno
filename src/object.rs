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
pub trait TObject {
    fn get_class_name(& mut self) -> String;
}
#[derive(Copy, Clone)]
pub struct Object {
    pub next: *mut Object,
    pub tag: u8,
    pub marked: u8,
}
impl TObject for Object {
    fn get_class_name(& mut self) -> String {
        "Object".to_string()
    }
}
