use crate::stkidrel::*;
use crate::tstring::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub union VariableDescriptionOrValue {
    pub vd: VariableDescription,
    pub k: TValue,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VariableDescription {
    pub value_: Value,
    pub tt_: u8,
    pub kind: u8,
    pub ridx: u8,
    pub pidx: i16,
    pub name: *mut TString,
}
