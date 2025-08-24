use crate::stackvalue::*;
#[derive(Copy, Clone)]
pub union StkIdRel {
    pub p: StkId,
    pub offset: i64,
}
