use crate::lclosure::*;
use crate::cclosure::*;
#[derive(Copy, Clone)]
pub union UClosure {
    pub c: CClosure,
    pub l: LClosure,
}
