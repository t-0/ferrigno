use crate::object::*;
use crate::ObjectBase;
ObjectBase! {
#[derive(Copy, Clone)]
pub struct TString {
    pub extra: u8,
    pub shrlen: u8,
    pub hash: u32,
    pub u: TStringLongShort,
    pub contents: [i8; 1],
}
}
#[derive(Copy, Clone)]
pub union TStringLongShort {
    pub lnglen: u64,
    pub hnext: *mut TString,
}
