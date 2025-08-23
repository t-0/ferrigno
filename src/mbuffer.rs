#[derive(Copy, Clone)]
#[repr(C)]
pub struct Mbuffer {
    pub pointer: *mut i8,
    pub length: u64,
    pub size: u64,
}
