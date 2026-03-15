#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AbsoluteLineInfo {
    pub absolutelineinfo_program_counter: i32,
    pub absolutelineinfo_line: i32,
}
