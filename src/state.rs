use crate::longjump::*;
use crate::callinfo::*;
use crate::global::*;
use crate::object::*;
use crate::functions::*;
use crate::stkidrel::*;
use crate::stackvalue::*;
use crate::tvalue::*;
use crate::tstring::*;
use crate::gcunion::*;
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
    pub base_callinfo: CallInfo,
    pub hook: HookFunction,
    pub error_function: i64,
    pub count_c_calls: u32,
    pub old_program_counter: i32,
    pub base_hook_count: i32,
    pub hook_count: i32,
    pub hook_mask: i32,
}
impl State {
    pub unsafe extern "C" fn set_error_object(
        &mut self,
        error_code: i32,
        old_top: StkId,
    ) {
        unsafe {
            match error_code {
                4 => {
                    let io: *mut TValue = &mut (*old_top).val;
                    let x_: *mut TString = (*(self.global)).memerrmsg;
                    (*io).value.gc = &mut (*(x_ as *mut GCUnion)).gc;
                    (*io).tag = ((*x_).tag as i32 | (1i32) << 6i32) as u8;
                }
                0 => {
                    (*old_top).val.tag = (0i32 | (0i32) << 4i32) as u8;
                }
                _ => {
                    let io1: *mut TValue = &mut (*old_top).val;
                    let io2: *const TValue = &mut (*(self.top.p).offset(-(1i32 as isize))).val;
                    (*io1).value = (*io2).value;
                    (*io1).tag = (*io2).tag;
                }
            }
            self.top.p = old_top.offset(1i32 as isize);
        }
    }
}
