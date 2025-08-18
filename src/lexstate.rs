#![allow(
    non_snake_case,
)]
use crate::state::*;
use crate::tstring::*;
use crate::table::*;
use crate::zio::*;
use crate::dyndata::*;
use crate::mbuffer::*;
use crate::funcstate::*;
use crate::token::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LexState {
    pub current: i32,
    pub linenumber: i32,
    pub lastline: i32,
    pub t: Token,
    pub lookahead: Token,
    pub fs: *mut FuncState,
    pub L: *mut State,
    pub z: *mut ZIO,
    pub buff: *mut Mbuffer,
    pub h: *mut Table,
    pub dyd: *mut Dyndata,
    pub source: *mut TString,
    pub envn: *mut TString,
}
