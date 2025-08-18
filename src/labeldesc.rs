use crate::tstring::*;
use crate::stkidrel::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Labeldesc {
    pub name: *mut TString,
    pub pc: i32,
    pub line: i32,
    pub count_active_variables: u8,
    pub close: u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed21 {
    pub arr: *mut Vardesc,
    pub n: i32,
    pub size: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union Vardesc {
    pub vd: C2RustUnnamed22,
    pub k: TValue,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed22 {
    pub value_: Value,
    pub tt_: u8,
    pub kind: u8,
    pub ridx: u8,
    pub pidx: i16,
    pub name: *mut TString,
}
