#[derive(Copy, Clone)]
#[repr(C)]
pub struct RandomState {
    pub data: [usize; 4],
}
