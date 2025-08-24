use crate::dynamicdata::*;
use crate::functionstate::*;
use crate::buffer::*;
use crate::state::*;
use crate::table::*;
use crate::token::*;
use crate::tstring::*;
use crate::new::*;
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
    pub zio: *mut ZIO,
    pub buffer: *mut Buffer,
    pub h: *mut Table,
    pub dynamic_data: *mut DynamicData,
    pub source: *mut TString,
    pub envn: *mut TString,
}
impl New for LexState {
    fn new() -> Self {
        return LexState {
            current: 0,
            linenumber: 0,
            lastline: 0,
            t: Token::new(),
            lookahead: Token::new(),
            fs: std::ptr::null_mut(),
            state: std::ptr::null_mut(),
            zio: std::ptr::null_mut(),
            buffer: std::ptr::null_mut(),
            h: std::ptr::null_mut(),
            dynamic_data: std::ptr::null_mut(),
            source: std::ptr::null_mut(),
            envn: std::ptr::null_mut(),
        };
    }
}
