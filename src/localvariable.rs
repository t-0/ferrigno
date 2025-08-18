use crate::tstring::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LocalVariable {
    pub varname: *mut TString,
    pub startpc: i32,
    pub endpc: i32,
}
