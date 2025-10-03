use crate::tag::*;
use crate::tstring::*;
use crate::tvalue::*;
use crate::value::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub union VariableDescription {
    pub variabledescription_content: VariableDescriptionContent,
    pub variabledescription_k: TValue,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VariableDescriptionContent {
    pub variabledescriptioncontent_value: Value,
    pub variabledescriptioncontent_tag: TagVariant,
    pub variabledescriptioncontent_kind: u8,
    pub variabledescriptioncontent_registerindex: u8,
    pub variabledescriptioncontent_pidx: i16,
    pub variabledescriptioncontent_name: *mut TString,
}
