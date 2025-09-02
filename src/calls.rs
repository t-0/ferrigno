use crate::stackvalue::*;
#[repr(C)]
pub struct CallS {
    pub function: StackValuePointer,
    pub count_results: i32,
}
