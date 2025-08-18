#[derive(Copy, Clone)]
#[repr(C)]
pub struct AbsoluteLineInfo {
    pub pc: i32,
    pub line: i32,
}
