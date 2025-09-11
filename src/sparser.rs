use crate::buffer::*;
use crate::dynamicdata::*;
use crate::zio::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SParser {
    pub zio: *mut ZIO,
    pub buffer: Buffer,
    pub dynamic_data: DynamicData,
    pub mode: *const libc::c_char,
    pub name: *const libc::c_char,
}
