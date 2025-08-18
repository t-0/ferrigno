#[derive(Copy, Clone)]
#[repr(C)]
pub struct RanState {
    pub s: [u64; 4],
}
