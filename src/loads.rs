#[derive(Copy, Clone)]
#[repr(C)]
pub struct LoadS {
    pub s: *const i8,
    pub size: u64,
}
