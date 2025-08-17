#![allow(
    static_mut_refs,
    unsafe_code,
    unsafe_attr_outside_unsafe,
    unsafe_op_in_unsafe_fn,
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut,
    unpredictable_function_pointer_comparisons,
    unused_imports,
)]
use libc::{tolower, toupper, remove, rename, setlocale};
use crate::c::*;
use crate::state::*;
use crate::gcobject::*;
use crate::lua_debug::*;
use crate::tstring::*;
use crate::callinfo::*;
use crate::stkidrel::*;
use crate::node::*;
use crate::table::*;
use crate::tstring::*;
use crate::lg::*;
use crate::lx::*;
use crate::proto::*;
use crate::gcunion::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Udata {
    pub next: *mut GCObject,
    pub tt: u8,
    pub marked: u8,
    pub nuvalue: u16,
    pub len: u64,
    pub metatable: *mut Table,
    pub gclist: *mut GCObject,
    pub uv: [UValue; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union UValue {
    pub uv: TValue,
    pub n: f64,
    pub u: f64,
    pub s: *mut libc::c_void,
    pub i: i64,
    pub l: i64,
}
