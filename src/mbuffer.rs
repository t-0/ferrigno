#[derive(Copy, Clone)]
#[repr(C)]
pub struct Mbuffer {
    pub buffer: *mut i8,
    pub n: u64,
    pub buffsize: u64,
}
