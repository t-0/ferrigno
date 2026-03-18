#[derive(Copy, Clone)]
#[repr(C)]
pub struct RandomState {
    pub randomstate_data: [usize; 4],
}
