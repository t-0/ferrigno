use crate::tstring::*;
#[derive(Copy, Clone)]
pub struct LabelDescription {
    pub name: *mut TString,
    pub program_counter: i32,
    pub line: i32,
    pub count_active_variables: u8,
    pub close: u8,
}
