use crate::object::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TString {
    pub next: *mut Object,
    pub tag: u8,
    pub marked: u8,
    pub extra: u8,
    pub short_length: u8,
    pub hash: u32,
    pub u: TStringExtension,
    pub contents: [i8; 1],
}
impl TObject for TString {
    fn get_class_name(& mut self) -> String {
        "TString".to_string()
    }
}
#[derive(Copy, Clone)]
pub union TStringExtension {
    pub long_length: u64,
    pub hash_next: *mut TString,
}
