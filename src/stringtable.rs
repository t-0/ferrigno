use crate::tstring::*;
#[derive(Copy, Clone)]
pub struct StringTable {
    pub hash: *mut *mut TString,
    pub length: i32,
    pub size: i32,
}
