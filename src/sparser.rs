use crate::dynamicdata::*;
use crate::buffer::*;
use crate::zio::*;
#[derive(Copy, Clone)]
pub struct SParser {
    pub z: *mut ZIO,
    pub buff: Buffer,
    pub dyd: DynamicData,
    pub mode: *const i8,
    pub name: *const i8,
}
