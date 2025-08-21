#[derive(Copy, Clone)]
#[repr(C)]
pub struct Object {
    pub next: *mut Object,
    pub tt: u8,
    pub marked: u8,
}
