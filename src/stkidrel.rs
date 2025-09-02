use crate::stackvalue::*;
#[derive(Copy, Clone)]
pub union StkIdRel {
    pub p: StackValuePointer,
    pub offset: i64,
}
