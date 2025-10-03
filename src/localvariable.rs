use crate::tstring::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LocalVariable {
    pub localvariable_variablename: *mut TString,
    pub localvariable_startprogramcounter: i32,
    pub localvariable_endprogramcounter: i32,
}
