use crate::tstring::*;
#[derive(Copy, Clone)]
pub union RawValue {
    pub ival: i64,
    pub nval: f64,
    pub strval: *mut TString,
    pub info: i32,
    pub ind: RawValueReference,
    pub var: RawValueRegister,
}
#[derive(Copy, Clone)]
pub struct RawValueRegister {
    pub ridx: u8,
    pub vidx: u16,
}
#[derive(Copy, Clone)]
pub struct RawValueReference {
    pub index: i16,
    pub t: u8,
}
