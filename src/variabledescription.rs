use crate::tstring::*;
use crate::tvalue::*;
use crate::value::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub union VariableDescription {
    pub content: VariableDescriptionContent,
    pub k: TValue,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VariableDescriptionContent {
    pub value: Value,
    pub tag: u8,
    pub kind: u8,
    pub ridx: u8,
    pub pidx: i16,
    pub name: *mut TString,
}
