use crate::stackvalue::*;
#[derive(Copy, Clone)]
pub union StkIdRel {
    pub stkidrel_pointer: StackValuePointer,
    pub stkidrel_offset: i64,
}
