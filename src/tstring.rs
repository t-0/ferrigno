use crate::object::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TString {
    pub next: *mut Object,
    pub tag: u8,
    pub marked: u8,
    pub extra: u8,
    pub shrlen: u8,
    pub hash: u32,
    pub u: TStringLongShort,
    pub contents: [i8; 1],
}
#[derive(Copy, Clone)]
pub union TStringLongShort {
    pub lnglen: u64,
    pub hnext: *mut TString,
}
