use crate::tstring::*;
#[derive(Copy, Clone)]
pub union SemanticInfo {
    pub r: f64,
    pub i: i64,
    pub ts: *mut TString,
}
