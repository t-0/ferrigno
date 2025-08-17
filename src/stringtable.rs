use crate::tstring::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StringTable {
    pub hash: *mut *mut TString,
    pub nuse: i32,
    pub size: i32,
}
