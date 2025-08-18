#![allow(
    non_snake_case,
)]
use crate::state::*;
use crate::tstring::*;
use crate::table::*;
use crate::zio::*;
use crate::dynamicdata::*;
use crate::mbuffer::*;
use crate::functionstate::*;
use crate::token::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LexState {
    pub current: i32,
    pub linenumber: i32,
    pub lastline: i32,
    pub t: Token,
    pub lookahead: Token,
    pub fs: *mut FunctionState,
    pub L: *mut State,
    pub z: *mut ZIO,
    pub buff: *mut Mbuffer,
    pub h: *mut Table,
    pub dyd: *mut DynamicData,
    pub source: *mut TString,
    pub envn: *mut TString,
}
