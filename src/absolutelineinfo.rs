#[derive(Copy, Clone)]
#[repr(C)]
pub struct AbsoluteLineInfo {
    pub program_counter: i32,
    pub line: i32,
}
