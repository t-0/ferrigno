use crate::tstring::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LabelDescription {
    pub labeldescription_name: *mut TString,
    pub labeldescription_programcounter: i32,
    pub labeldescription_line: i32,
    pub labeldescription_countactivevariables: usize,
    pub labeldescription_close: u8,
}
