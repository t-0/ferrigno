use crate::tvalue::*;
use crate::tstring::*;
use crate::value::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub union VariableDescription {
    pub vd: VariableDescriptionA,
    pub k: TValue,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VariableDescriptionA {
    pub value: Value,
    pub tag: u8,
    pub kind: u8,
    pub ridx: u8,
    pub pidx: i16,
    pub name: *mut TString,
}
