use crate::object::*;
use crate::state::*;
pub type ContextFunction = Option<unsafe extern "C" fn(*mut State, i32, i64) -> i32>;
#[derive(Copy, Clone)]
pub union StkIdRel {
    pub p: StkId,
    pub offset: i64,
}
pub type StkId = *mut StackValue;
#[derive(Copy, Clone)]
pub union StackValue {
    pub val: TValue,
    pub tbclist: StackValueExtension,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StackValueExtension {
    pub value_: Value,
    pub tt_: u8,
    pub delta: u16,
}
#[derive(Copy, Clone)]
pub union Value {
    pub gc: *mut Object,
    pub p: *mut libc::c_void,
    pub f: CFunction,
    pub i: i64,
    pub n: f64,
}
pub type CFunction = Option<unsafe extern "C" fn(*mut State) -> i32>;
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
