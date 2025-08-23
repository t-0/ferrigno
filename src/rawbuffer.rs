use crate::new::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RawBuffer {
    pub pointer: *mut i8,
    pub length: u64,
    pub size: u64,
}
impl New for RawBuffer {
    fn new() -> Self {
        return RawBuffer {
            pointer: 0 as *mut i8,
            length: 0,
            size: 0,
        };
    }

}