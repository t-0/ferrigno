use crate::stkidrel::*;
use crate::functions::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CallInfo {
    pub function: StkIdRel,
    pub top: StkIdRel,
    pub previous: *mut CallInfo,
    pub next: *mut CallInfo,
    pub u: CallInfoConstituentA,
    pub u2: CallInfoConsistuentB,
    pub count_results: i16,
    pub call_status: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union CallInfoConstituentA {
    pub l: CallInfoConstituentAL,
    pub c: CallInfoConstituentAC,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CallInfoConstituentAL {
    pub saved_program_counter: *const u32,
    pub trap: i32,
    pub count_extra_arguments: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CallInfoConstituentAC {
    pub k: ContextFunction,
    pub old_error_function: i64,
    pub ctx: i64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union CallInfoConsistuentB {
    pub funcidx: i32,
    pub nyield: i32,
    pub nres: i32,
    pub transferinfo: CallInfoConsistuentBTransferInfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CallInfoConsistuentBTransferInfo {
    pub ftransfer: u16,
    pub ntransfer: u16,
}
