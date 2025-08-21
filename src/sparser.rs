use crate::zio::*;
use crate::dynamicdata::*;
use crate::mbuffer::*;
#[derive(Copy, Clone)]
pub struct SParser {
    pub z: *mut ZIO,
    pub buff: Mbuffer,
    pub dyd: DynamicData,
    pub mode: *const i8,
    pub name: *const i8,
}
