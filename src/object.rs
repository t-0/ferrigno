#[derive(Copy, Clone)]
pub struct Object {
    pub next: *mut Object,
    pub tag: u8,
    pub marked: u8,
}
