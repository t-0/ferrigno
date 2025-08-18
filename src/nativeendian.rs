#[derive(Copy, Clone)]
#[repr(C)]
pub union NativeEndian {
    pub dummy: i32,
    pub little: i8,
}
