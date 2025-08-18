use crate::stkidrel::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed11 {
    pub funcidx: i32,
    pub nyield: i32,
    pub nres: i32,
    pub transferinfo: C2RustUnnamed12,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed15 {
    pub savedpc: *const u32,
    pub trap: i32,
    pub nextraargs: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed14 {
    pub k: ContextFunction,
    pub old_errfunc: i64,
    pub ctx: i64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed13 {
    pub l: C2RustUnnamed15,
    pub c: C2RustUnnamed14,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CallInfo {
    pub func: StkIdRel,
    pub top: StkIdRel,
    pub previous: *mut CallInfo,
    pub next: *mut CallInfo,
    pub u: C2RustUnnamed13,
    pub u2: C2RustUnnamed11,
    pub nresults: i16,
    pub callstatus: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed12 {
    pub ftransfer: u16,
    pub ntransfer: u16,
}
