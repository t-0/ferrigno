#[derive(Copy, Clone)]
#[repr(C)]
pub struct LoadS {
    pub pointer: *const i8,
    pub size: u64,
}
