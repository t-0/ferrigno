use crate::cclosure::*;
use crate::lclosure::*;
use crate::object::*;
#[derive(Copy, Clone)]
pub union UClosure {
    pub object: Object,
    pub c: CClosure,
    pub l: LClosure,
}
