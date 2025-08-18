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
use crate::debug::*;
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
pub union Closure {
    pub c: CClosure,
    pub l: LClosure,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LClosure {
    pub next: *mut GCObject,
    pub tt: u8,
    pub marked: u8,
    pub nupvalues: u8,
    pub gclist: *mut GCObject,
    pub p: *mut Proto,
    pub upvals: [*mut UpVal; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CClosure {
    pub next: *mut GCObject,
    pub tt: u8,
    pub marked: u8,
    pub nupvalues: u8,
    pub gclist: *mut GCObject,
    pub f: CFunction,
    pub upvalue: [TValue; 1],
}
