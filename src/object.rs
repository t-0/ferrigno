#[macro_export]
macro_rules! ObjectBase {
    (#[derive($($derive:meta),*)] $pub:vis struct $name:ident { $($fpub:vis $field:ident : $type:ty,)* }) => {
        #[derive($($derive),*)]
        #[repr(C)]
        $pub struct $name {
            pub next: *mut Object,
            pub tag: u8,
            pub marked: u8,
            $($fpub $field : $type,)*
        }
        impl $name {
            $pub fn new(next: *mut Object, tag: u8, marked: u8, $($field:$type,)*) -> Self{
                Self{
                    next,
                    tag,
                    marked,
                    $($field,)*
                }
            }
        }
    }
}
#[derive(Copy, Clone)]
pub struct Object {
    pub next: *mut Object,
    pub tag: u8,
    pub marked: u8,
}
