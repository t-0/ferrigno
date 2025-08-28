#[derive(Copy, Clone)]
#[repr(C)]
pub struct RandomState {
    pub s: [u64; 4],
}
