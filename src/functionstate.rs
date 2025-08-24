use crate::blockcontrol::*;
use crate::lexstate::*;
use crate::prototype::*;
use crate::new::*;
#[derive(Copy, Clone)]
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
impl New for FunctionState {
    fn new() -> Self {
        return FunctionState {
            f: std::ptr::null_mut(),
            prev: std::ptr::null_mut(),
            ls: std::ptr::null_mut(),
            blockcontrol: std::ptr::null_mut(),
            program_counter: 0,
            lasttarget: 0,
            previousline: 0,
            nk: 0,
            np: 0,
            nabslineinfo: 0,
            firstlocal: 0,
            first_label: 0,
            ndebugvars: 0,
            count_active_variables: 0,
            nups: 0,
            freereg: 0,
            iwthabs: 0,
            needclose: 0,
        };
    }
}
