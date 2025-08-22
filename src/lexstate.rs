use crate::dynamicdata::*;
use crate::functionstate::*;
use crate::mbuffer::*;
use crate::state::*;
use crate::table::*;
use crate::token::*;
use crate::tstring::*;
use crate::zio::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LexState {
    pub current: i32,
    pub linenumber: i32,
    pub lastline: i32,
    pub t: Token,
    pub lookahead: Token,
    pub fs: *mut FunctionState,
    pub state: *mut State,
    pub z: *mut ZIO,
    pub buff: *mut Mbuffer,
    pub h: *mut Table,
    pub dyd: *mut DynamicData,
    pub source: *mut TString,
    pub envn: *mut TString,
}
