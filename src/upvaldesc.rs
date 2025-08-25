use crate::tstring::*;
pub struct Upvaldesc {
    pub name: *mut TString,
    pub is_in_stack: bool,
    pub index: u8,
    pub kind: u8,
}
