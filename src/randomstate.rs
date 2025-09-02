#[derive(Copy, Clone)]
#[repr(C)]
pub struct RandomState {
    pub data: [u64; 4],
}
