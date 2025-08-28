use crate::tstring::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LocalVariable {
    pub variable_name: *mut TString,
    pub start_program_counter: i32,
    pub end_program_counter: i32,
}
