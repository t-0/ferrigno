use crate::tstring::*;
use crate::new::*;
#[derive(Copy, Clone)]
pub union SemanticInfo {
    pub r: f64,
    pub i: i64,
    pub ts: *mut TString,
}
impl New for SemanticInfo {
    fn new() -> Self {
        return SemanticInfo { r: 0. };
    }
}
