#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
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
use crate::udata::*;
use crate::closure::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LocVar {
    pub varname: *mut TString,
    pub startpc: i32,
    pub endpc: i32,
}
