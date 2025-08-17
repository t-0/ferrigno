use crate::tstring::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Upvaldesc {
    pub name: *mut TString,
    pub instack: u8,
    pub idx: u8,
    pub kind: u8,
}
