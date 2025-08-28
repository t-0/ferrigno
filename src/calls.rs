use crate::stackvalue::*;
#[repr(C)]
pub struct CallS {
    pub function: StkId,
    pub count_results: i32,
}
