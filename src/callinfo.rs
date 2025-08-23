use crate::stkidrel::*;
#[derive(Copy, Clone)]
pub union CallInfoSomethingElse {
    pub funcidx: i32,
    pub nyield: i32,
    pub nres: i32,
    pub transferinfo: TransferInfo,
}
#[derive(Copy, Clone)]
pub struct CallInfoSomethingL {
    pub saved_program_counter: *const u32,
    pub trap: i32,
    pub count_extra_arguments: i32,
}
#[derive(Copy, Clone)]
pub struct CallInfoSomethingC {
    pub k: ContextFunction,
    pub old_error_function: i64,
    pub ctx: i64,
}
#[derive(Copy, Clone)]
pub union CallInfoSomething {
    pub l: CallInfoSomethingL,
    pub c: CallInfoSomethingC,
}
#[derive(Copy, Clone)]
pub struct CallInfo {
    pub function: StkIdRel,
    pub top: StkIdRel,
    pub previous: *mut CallInfo,
    pub next: *mut CallInfo,
    pub u: CallInfoSomething,
    pub u2: CallInfoSomethingElse,
    pub count_results: i16,
    pub call_status: u16,
}
#[derive(Copy, Clone)]
pub struct TransferInfo {
    pub ftransfer: u16,
    pub ntransfer: u16,
}
