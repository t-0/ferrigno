use crate::tstring::*;
#[repr(C)]
pub struct UpValueDescription {
    pub name: *mut TString,
    pub is_in_stack: bool,
    pub index: u8,
    pub kind: u8,
}
