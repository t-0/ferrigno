use crate::longjump::*;
use crate::callinfo::*;
use crate::global::*;
use crate::object::*;
use crate::functions::*;
use crate::stkidrel::*;
use crate::upvalue::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct State {
    pub next: *mut Object,
    pub tag: u8,
    pub marked: u8,
    pub status: u8,
    pub allow_hook: u8,
    pub nci: u16,
    pub top: StkIdRel,
    pub global: *mut Global,
    pub ci: *mut CallInfo,
    pub stack_last: StkIdRel,
    pub stack: StkIdRel,
    pub openupval: *mut UpValue,
    pub tbclist: StkIdRel,
    pub gc_list: *mut Object,
    pub twups: *mut State,
    pub error_jump: *mut LongJump,
    pub base_ci: CallInfo,
    pub hook: HookFunction,
    pub errfunc: i64,
    pub count_c_calls: u32,
    pub oldpc: i32,
    pub basehookcount: i32,
    pub hookcount: i32,
    pub hookmask: i32,
}
