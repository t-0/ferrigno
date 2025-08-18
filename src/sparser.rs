use crate::zio::*;
use crate::dyndata::*;
use crate::mbuffer::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SParser {
    pub z: *mut ZIO,
    pub buff: Mbuffer,
    pub dyd: Dyndata,
    pub mode: *const i8,
    pub name: *const i8,
}
