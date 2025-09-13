use crate::tvalue::*;
#[repr(C)]
pub struct CallS {
    pub function: *mut TValue,
    pub count_results: i32,
}
