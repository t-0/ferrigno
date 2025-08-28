use crate::buffer::*;
use crate::dynamicdata::*;
use crate::zio::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SParser {
    pub zio: *mut ZIO,
    pub buffer: Buffer,
    pub dynamic_data: DynamicData,
    pub mode: *const i8,
    pub name: *const i8,
}
