use crate::callinfo::*;
use crate::functions::*;
use crate::gcunion::*;
use crate::global::*;
use crate::longjump::*;
use crate::object::*;
use crate::stackvalue::*;
use crate::stkidrel::*;
use crate::tstring::*;
use crate::tvalue::*;
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
    pub unsafe extern "C" fn set_error_object(&mut self, error_code: i32, old_top: StkId) {
        unsafe {
            match error_code {
                4 => {
                    let io: *mut TValue = &mut (*old_top).val;
                    let x_: *mut TString = (*(self.global)).memerrmsg;
                    (*io).value.gc = &mut (*(x_ as *mut GCUnion)).gc;
                    (*io).tag = ((*x_).tag as i32 | (1i32) << 6i32) as u8;
                }
                0 => {
                    (*old_top).val.tag = 0;
                }
                _ => {
                    let io1: *mut TValue = &mut (*old_top).val;
                    let io2: *const TValue = &mut (*(self.top.p).offset(-(1i32 as isize))).val;
                    (*io1).value = (*io2).value;
                    (*io1).tag = (*io2).tag;
                }
            }
            self.top.p = old_top.offset(1);
        }
    }
    pub unsafe extern "C" fn correct_stack(&mut self) {
        unsafe {
            (*self).top.p =
                ((*self).stack.p as *mut i8).offset((*self).top.offset as isize) as StkId;
            (*self).tbclist.p =
                ((*self).stack.p as *mut i8).offset((*self).tbclist.offset as isize) as StkId;
            let mut up: *mut UpValue = (*self).openupval;
            while !up.is_null() {
                (*up).v.p = &mut (*(((*self).stack.p as *mut i8).offset((*up).v.offset as isize)
                    as StkId))
                    .val;
                up = (*up).u.open.next;
            }
            let mut ci: *mut CallInfo = (*self).ci;
            while !ci.is_null() {
                (*ci).top.p =
                    ((*self).stack.p as *mut i8).offset((*ci).top.offset as isize) as StkId;
                (*ci).function.p =
                    ((*self).stack.p as *mut i8).offset((*ci).function.offset as isize) as StkId;
                if (*ci).call_status as i32 & (1i32) << 1i32 == 0 {
                    ::core::ptr::write_volatile(&mut (*ci).u.l.trap as *mut i32, 1i32);
                }
                ci = (*ci).previous;
            }
        }
    }
    pub fn is_yieldable(&mut self) -> bool {
        return self.count_c_calls & 0xffff0000u32 == 0;
    }
}
