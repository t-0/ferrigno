use crate::new::*;
use crate::tstring::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub union SemanticInfo {
    pub number: f64,
    pub integer: i64,
    pub tstring: *mut TString,
}
impl New for SemanticInfo {
    fn new() -> Self {
        return SemanticInfo { number: 0. };
    }
}
