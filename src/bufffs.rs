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
use crate::tm::*;
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
use crate::udata::*;
use crate::closure::*;
use crate::locvar::*;
use crate::abslineinfo::*;
use crate::upvaldesc::*;
use crate::readfunction::*;
use crate::writefunction::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BuffFS {
    pub L: *mut State,
    pub pushed: i32,
    pub blen: i32,
    pub space: [i8; 199],
}
