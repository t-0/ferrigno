use crate::buffer::*;
use crate::dynamicdata::*;
use crate::zio::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SParser {
    pub zio: *mut ZIO,
    pub name: *const i8,
    pub mode: *const i8,
    pub buffer: Buffer,
    pub dynamic_data: DynamicData,
}
impl SParser {
    pub fn new(zio: *mut ZIO, name: *const i8, mode: *const i8) -> Self {
        SParser {
            zio: zio,
            name: name,
            mode: mode,
            buffer: Buffer::new(),
            dynamic_data: DynamicData::new(),
        }
    }
}
