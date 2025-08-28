use crate::callinfo::*;
use crate::tag::*;
use crate::functions::*;
use crate::gcunion::*;
use crate::global::*;
use crate::longjump::*;
use crate::object::*;
use crate::table::*;
use crate::stackvalue::*;
use crate::stkidrel::*;
use crate::tstring::*;
use crate::onelua::*;
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
    pub count_call_info: u16,
    pub top: StkIdRel,
    pub global: *mut Global,
    pub call_info: *mut CallInfo,
    pub stack_last: StkIdRel,
    pub stack: StkIdRel,
    pub open_upvalue: *mut UpValue,
    pub tbc_list: StkIdRel,
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
impl TObject for State {
    fn get_marked(& self) -> u8 {
        self.marked
    }
    fn set_marked(& mut self, marked_: u8) {
        self.marked = marked_;
    }
    fn set_tag(& mut self, tag: u8) {
        self.tag = tag;
    }
    fn set_collectable(& mut self) {
        self.set_tag(set_collectable(self.get_tag()));
    }
    fn is_collectable(&self) -> bool {
        return is_collectable(self.get_tag());
    }
    fn get_tag(&self) -> u8 {
        self.tag
    }
    fn get_tag_type(&self) -> u8 {
        get_tag_type(self.get_tag())
    }
    fn get_tag_variant(&self) -> u8 {
        get_tag_variant(self.get_tag())
    }
    fn get_class_name(& mut self) -> String {
        "state".to_string()
    }
    fn get_metatable(& mut self) -> *mut Table {
        std::ptr::null_mut()
    }
}
impl State {
    pub unsafe extern "C" fn set_error_object(&mut self, error_code: i32, old_top: StkId) {
        unsafe {
            match error_code {
                4 => {
                    let io: *mut TValue = &mut (*old_top).value;
                    let x_: *mut TString = (*(self.global)).memerrmsg;
                    (*io).value.gc = &mut (*(x_ as *mut GCUnion)).gc;
                    (*io).set_tag((*x_).get_tag());
                    (*io).set_collectable();
                }
                0 => {
                    (*old_top).value.set_tag(TAG_VARIANT_NIL_NIL);
                }
                _ => {
                    let io1: *mut TValue = &mut (*old_top).value;
                    let io2: *const TValue = &mut (*(self.top.p).offset(-(1i32 as isize))).value;
                    (*io1).value = (*io2).value;
                    (*io1).set_tag((*io2).get_tag());
                }
            }
            self.top.p = old_top.offset(1);
        }
    }
    pub unsafe extern "C" fn correct_stack(&mut self) {
        unsafe {
            (*self).top.p =
                ((*self).stack.p as *mut i8).offset((*self).top.offset as isize) as StkId;
            (*self).tbc_list.p =
                ((*self).stack.p as *mut i8).offset((*self).tbc_list.offset as isize) as StkId;
            let mut up: *mut UpValue = (*self).open_upvalue;
            while !up.is_null() {
                (*up).v.p = &mut (*(((*self).stack.p as *mut i8).offset((*up).v.offset as isize)
                    as StkId))
                    .value;
                up = (*up).u.open.next;
            }
            let mut call_info: *mut CallInfo = (*self).call_info;
            while !call_info.is_null() {
                (*call_info).top.p =
                    ((*self).stack.p as *mut i8).offset((*call_info).top.offset as isize) as StkId;
                (*call_info).function.p = ((*self).stack.p as *mut i8)
                    .offset((*call_info).function.offset as isize)
                    as StkId;
                if (*call_info).call_status as i32 & (1i32) << 1i32 == 0 {
                    ::core::ptr::write_volatile(&mut (*call_info).u.l.trap as *mut i32, 1i32);
                }
                call_info = (*call_info).previous;
            }
        }
    }
    pub fn is_yieldable(&mut self) -> bool {
        return self.count_c_calls & 0xffff0000u32 == 0;
    }
    pub unsafe extern "C" fn push_boolean(&mut self, x: bool) {
        unsafe {
            if x {
                (*self.top.p).value.set_tag(TAG_VARIANT_BOOLEAN_TRUE);
            } else {
                (*self.top.p).value.set_tag(TAG_VARIANT_BOOLEAN_FALSE);
            }
            self.top.p = self.top.p.offset(1);
        }
    }
    pub unsafe extern "C" fn push_integer(&mut self, x: i64) {
        unsafe {
            let t_value: *mut TValue = &mut (*self.top.p).value;
            (*t_value).value.i = x;
            (*t_value).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
            self.top.p = self.top.p.offset(1);
        }
    }
    pub unsafe extern "C" fn push_nil(&mut self) {
        unsafe {
            (*self.top.p).value.set_tag(TAG_VARIANT_NIL_NIL);
            self.top.p = self.top.p.offset(1);
        }
    }
    pub unsafe extern "C" fn push_number(&mut self, x: f64) {
        unsafe {
            let t_value: *mut TValue = &mut (*self.top.p).value;
            (*t_value).value.n = x;
            (*t_value).set_tag(TAG_VARIANT_NUMERIC_NUMBER);
            self.top.p = self.top.p.offset(1);
        }
    }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn get_top(&mut self) -> i32 {
        unsafe {
            return self
                .top
                .p
                .offset_from(((*self.call_info).function.p).offset(1 as isize))
                as i64 as i32;
        }
    }
    pub unsafe extern "C" fn find_pcall(&mut self) -> *mut CallInfo {
        unsafe {
            let mut it = self.call_info;
            return loop {
                if it.is_null() {
                    break it;
                } else if ((*it).call_status & (1 << 4)) != 0 {
                    break it;
                } else {
                    it = (*it).previous;
                }
            };
        }
    }
    pub unsafe extern "C" fn sweep_list(
        & mut self,
        mut p: *mut *mut Object,
        countin: i32,
        countout: *mut i32,
    ) -> *mut *mut Object { unsafe {
        let other_white = (*(self.global)).current_white ^ (1 << 3 | 1 << 4);
        let mut i: i32;
        let white = (*(self.global)).current_white & ((1 << 3) | (1 << 4));
        i = 0;
        while !(*p).is_null() && i < countin {
            let curr: *mut Object = *p;
            let marked = (*curr).get_marked();
            if marked & other_white != 0 {
                *p = (*curr).next;
                freeobj(self, curr);
            } else {
                (*curr).set_marked(marked & !(1 << 5 | (1 << 3 | 1 << 4) | 7)
                    | white);
                p = &mut (*curr).next;
            }
            i += 1;
        }
        if !countout.is_null() {
            *countout = i;
        }
        return if (*p).is_null() {
            std::ptr::null_mut()
        } else {
            p
        };
    }}
    pub unsafe extern "C" fn free_memory(
        & mut self,
        block: *mut libc::c_void,
        old_size: u64,
    ) { unsafe {
        raw_allocate(
            block,
            old_size,
            0u64,
        );
        (*(self.global)).gc_debt = ((*(self.global)).gc_debt as u64).wrapping_sub(old_size) as i64 as i64;
    }}
    pub unsafe extern "C" fn too_big(& mut self) -> ! { unsafe {
        luag_runerror(
            self,
            b"memory allocation error: block too big\0" as *const u8 as *const i8,
        );
    }}
}
