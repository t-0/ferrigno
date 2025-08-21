use crate::prototype::*;
use crate::lexstate::*;
use crate::blockcontrol::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FunctionState {
    pub f: *mut Prototype,
    pub prev: *mut FunctionState,
    pub ls: *mut LexState,
    pub blockcontrol: *mut BlockControl,
    pub program_counter: i32,
    pub lasttarget: i32,
    pub previousline: i32,
    pub nk: i32,
    pub np: i32,
    pub nabslineinfo: i32,
    pub firstlocal: i32,
    pub first_label: i32,
    pub ndebugvars: i16,
    pub count_active_variables: u8,
    pub nups: u8,
    pub freereg: u8,
    pub iwthabs: u8,
    pub needclose: u8,
}
