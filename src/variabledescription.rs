use crate::tstring::*;
use crate::tvalue::*;
use crate::value::*;
use crate::tag::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub union VariableDescription {
    pub content: VariableDescriptionContent,
    pub k: TValue,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VariableDescriptionContent {
    pub variabledescriptioncontent_value: Value,
    pub tag: TagVariant,
    pub variabledescriptioncontent_kind: u8,
    pub variabledescriptioncontent_registerindex: u8,
    pub variabledescriptioncontent_pidx: i16,
    pub variabledescriptioncontent_name: *mut TString,
}
