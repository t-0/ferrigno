use crate::buffer::*;
use crate::dynamicdata::*;
use crate::tdefaultnew::*;
use crate::zio::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SParser {
    pub sparser_zio: *mut ZIO,
    pub sparser_name: *const i8,
    pub sparser_mode: *const i8,
    pub sparser_buffer: Buffer,
    pub sparser_dynamicdata: DynamicData,
}
impl SParser {
    pub fn new(zio: *mut ZIO, name: *const i8, mode: *const i8) -> Self {
        SParser {
            sparser_zio: zio,
            sparser_name: name,
            sparser_mode: mode,
            sparser_buffer: Buffer::new(),
            sparser_dynamicdata: DynamicData::new(),
        }
    }
}
