use crate::dynamicdata::*;
use crate::mbuffer::*;
use crate::zio::*;
#[derive(Copy, Clone)]
pub struct SParser {
    pub z: *mut ZIO,
    pub buff: Mbuffer,
    pub dyd: DynamicData,
    pub mode: *const i8,
    pub name: *const i8,
}
