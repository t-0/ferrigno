use crate::interpreter::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Header {
    pub header_interpreter: *mut Interpreter,
    pub header_islittleendian: i32,
    pub header_maxmimumalignment: i32,
}
