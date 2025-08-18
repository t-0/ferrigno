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
use crate::lexstate::*;
use crate::sparser::*;
use crate::mbuffer::*;
use crate::sparser::*;
use crate::blockcontrol::*;
use crate::token::*;
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
use crate::localvariable::*;
use crate::abslineinfo::*;
use crate::calls::*;
use crate::zio::*;
use crate::upvaldesc::*;
use crate::readfunction::*;
use crate::writefunction::*;
use crate::bufffs::*;
use crate::closep::*;
use crate::instruction::*;
use crate::dynamicdata::*;
use crate::labellist::*;
use crate::labeldesc::*;
use crate::functionstate::*;
use crate::lexstate::*;
use crate::mbuffer::*;
use crate::c2rustunnamed_25::*;
use crate::c2rustunnamed_24::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_23 {
    pub ival: i64,
    pub nval: f64,
    pub strval: *mut TString,
    pub info: i32,
    pub ind: C2RustUnnamed_25,
    pub var: C2RustUnnamed_24,
}
