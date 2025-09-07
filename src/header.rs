use crate::interpreter::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Header {
    pub interpreter: *mut Interpreter,
    pub is_little_endian: i32,
    pub maxmimum_alignment: i32,
}
