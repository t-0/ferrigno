use crate::tvalue::*;
#[derive(Copy, Clone)]
pub struct StkIdRel {
    pub stkidrel_pointer: *mut TValue,
    pub stkidrel_offset: i64,
}
