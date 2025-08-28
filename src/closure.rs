use crate::cclosure::*;
use crate::lclosure::*;
#[derive(Copy, Clone)]
pub union UClosure {
    pub c: CClosure,
    pub l: LClosure,
}
