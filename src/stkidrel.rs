use crate::object::*;
use crate::functions::*;
use crate::stackvalue::*;
#[derive(Copy, Clone)]
pub union StkIdRel {
    pub p: StkId,
    pub offset: i64,
}
#[derive(Copy, Clone)]
pub union Value {
    pub gc: *mut Object,
    pub p: *mut libc::c_void,
    pub f: CFunction,
    pub i: i64,
    pub n: f64,
}
#[derive(Copy, Clone)]
pub struct TValue {
    pub value_: Value,
    pub tt_: u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UpVal {
    pub next: *mut Object,
    pub tag: u8,
    pub marked: u8,
    pub v: C2RustUnnamed_19,
    pub u: C2RustUnnamed17,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed17 {
    pub open: C2RustUnnamed18,
    pub value: TValue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed18 {
    pub next: *mut UpVal,
    pub previous: *mut *mut UpVal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_19 {
    pub p: *mut TValue,
    pub offset: i64,
}
