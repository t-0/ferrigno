use crate::callinfo::*;
use crate::functions::*;
use crate::debuginfo::*;
use crate::stateextra::*;
use crate::interpreter::*;
use crate::loadstate::*;
use crate::loadf::*;
use crate::loads::*;
use crate::token::*;
use crate::upvaluedescription::*;
use crate::forloop::*;
use crate::bufffs::*;
use crate::utility::c::*;
use crate::calls::*;
use crate::dumpstate::*;
use crate::vm::instruction::*;
use crate::stringtable::*;
use crate::dynamicdata::*;
use crate::functionstate::*;
use crate::tm::*;
use crate::global::*;
use crate::longjump::*;
use crate::object::*;
use crate::prototype::*;
use crate::closure::*;
use crate::zio::*;
use crate::tag::*;
use crate::buffer::*;
use crate::character::*;
use crate::utility::*;
use crate::sparser::*;
use crate::closep::*;
use crate::new::*;
use crate::f2i::*;
use crate::value::*;
use crate::labeldescription::*;
use crate::registeredfunction::*;
use crate::labellist::*;
use crate::stackvalue::*;
use crate::variabledescription::*;
use crate::stkidrel::*;
use crate::table::*;
use crate::user::*;
use crate::tstring::*;
use crate::tvalue::*;
use crate::upvalue::*;
use crate::lexical::lexicalstate::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct State {
    pub object: Object,
    pub status: u8,
    pub allow_hook: u8,
    pub count_call_info: u16,
    pub dummy3: u32 = 0,
    pub top: StkIdRel,
    pub global: *mut Global,
    pub call_info: *mut CallInfo,
    pub stack_last: StkIdRel,
    pub stack: StkIdRel,
    pub open_upvalue: *mut UpValue,
    pub tbc_list: StkIdRel,
    pub gc_list: *mut Object,
    pub twups: *mut State,
    pub long_jump: *mut LongJump,
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
    fn get_tag(&self) -> u8 {
        self.object.tag
    }
    fn set_tag(&mut self, tag: u8) {
        self.object.tag = tag;
    }
    fn get_marked(&self) -> u8 {
        self.object.marked
    }
    fn set_marked(&mut self, marked: u8) {
        self.object.marked = marked;
    }
    fn get_class_name(&mut self) -> String {
        "state".to_string()
    }
    fn get_metatable(&mut self) -> *mut Table {
        std::ptr::null_mut()
    }
}
impl State {
    pub fn get_status(& mut self) -> i32 {
        return self.status as i32;
    }
    pub unsafe fn set_error_object(&mut self, error_code: i32, old_top: StackValuePointer) {
        unsafe {
            match error_code {
                4 => {
                    let io: *mut TValue = &mut (*old_top).tvalue;
                    let x_: *mut TString = (*(self.global)).memory_error_message;
                    (*io).value.object = &mut (*(x_ as *mut Object));
                    (*io).set_tag((*x_).get_tag());
                    (*io).set_collectable();
                }
                0 => {
                    (*old_top).tvalue.set_tag(TAG_VARIANT_NIL_NIL);
                }
                _ => {
                    let io1: *mut TValue = &mut (*old_top).tvalue;
                    let io2: *const TValue = &mut (*(self.top.p).offset(-(1i32 as isize))).tvalue;
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
                ((*self).stack.p as *mut i8).offset((*self).top.offset as isize) as StackValuePointer;
            (*self).tbc_list.p =
                ((*self).stack.p as *mut i8).offset((*self).tbc_list.offset as isize) as StackValuePointer;
            let mut up: *mut UpValue = (*self).open_upvalue;
            while !up.is_null() {
                (*up).v.p = &mut (*(((*self).stack.p as *mut i8).offset((*up).v.offset as isize)
                    as StackValuePointer))
                    .tvalue;
                up = (*up).u.open.next;
            }
            let mut call_info: *mut CallInfo = (*self).call_info;
            while !call_info.is_null() {
                (*call_info).top.p =
                    ((*self).stack.p as *mut i8).offset((*call_info).top.offset as isize) as StackValuePointer;
                (*call_info).function.p = ((*self).stack.p as *mut i8)
                    .offset((*call_info).function.offset as isize)
                    as StackValuePointer;
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
                (*self.top.p).tvalue.set_tag(TAG_VARIANT_BOOLEAN_TRUE);
            } else {
                (*self.top.p).tvalue.set_tag(TAG_VARIANT_BOOLEAN_FALSE);
            }
            self.top.p = self.top.p.offset(1);
        }
    }
    pub unsafe extern "C" fn push_integer(&mut self, x: i64) {
        unsafe {
            let t_value: *mut TValue = &mut (*self.top.p).tvalue;
            (*t_value).value.integer = x;
            (*t_value).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
            self.top.p = self.top.p.offset(1);
        }
    }
    pub unsafe extern "C" fn push_nil(&mut self) {
        unsafe {
            (*self.top.p).tvalue.set_tag(TAG_VARIANT_NIL_NIL);
            self.top.p = self.top.p.offset(1);
        }
    }
    pub unsafe extern "C" fn push_number(&mut self, x: f64) {
        unsafe {
            let t_value: *mut TValue = &mut (*self.top.p).tvalue;
            (*t_value).value.number = x;
            (*t_value).set_tag(TAG_VARIANT_NUMERIC_NUMBER);
            self.top.p = self.top.p.offset(1);
        }
    }
    pub unsafe extern "C" fn get_top(&mut self) -> i32 {
        unsafe {
            return self
                .top
                .p
                .offset_from(((*self.call_info).function.p).offset(1 as isize))
                as i32;
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
        &mut self,
        mut p: *mut *mut Object,
        countin: i32,
        countout: *mut i32,
    ) -> *mut *mut Object {
        unsafe {
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
                    (*curr).set_marked(marked & !(1 << 5 | (1 << 3 | 1 << 4) | 7) | white);
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
        }
    }
    pub unsafe extern "C" fn free_memory(&mut self, block: *mut libc::c_void, old_size: u64) {
        unsafe {
            raw_allocate(block, old_size, 0u64);
            (*(self.global)).gc_debt =
                ((*(self.global)).gc_debt as u64).wrapping_sub(old_size) as i64;
        }
    }
    pub unsafe extern "C" fn too_big(&mut self) -> ! {
        unsafe {
            luag_runerror(
                self,
                b"memory allocation error: block too big\0" as *const u8 as *const i8,
            );
        }
    }
    pub unsafe extern "C" fn push_state(&mut self) -> bool {
        unsafe {
            let io: *mut TValue = &mut (*self.top.p).tvalue;
            (*io).value.object = &mut (*(self as *mut State as *mut Object));
            (*io).set_tag(TAG_VARIANT_STATE);
            (*io).set_collectable();
            self.top.p = self.top.p.offset(1);
            return (*self.global).main_state == self;
        }
    }
    pub unsafe extern "C" fn relstack(& mut self) {
        unsafe {
            self.top.offset =
                (self.top.p as *mut i8).offset_from(self.stack.p as *mut i8) as i64;
            self.tbc_list.offset =
                (self.tbc_list.p as *mut i8).offset_from(self.stack.p as *mut i8) as i64;
            let mut up: *mut UpValue = self.open_upvalue;
            while !up.is_null() {
                (*up).v.offset =
                    ((*up).v.p as StackValuePointer as *mut i8).offset_from(self.stack.p as *mut i8) as i64;
                up = (*up).u.open.next;
            }
            let mut call_info: *mut CallInfo = self.call_info;
            while !call_info.is_null() {
                (*call_info).top.offset =
                    ((*call_info).top.p as *mut i8).offset_from(self.stack.p as *mut i8) as i64;
                (*call_info).function.offset = ((*call_info).function.p as *mut i8)
                    .offset_from(self.stack.p as *mut i8)
                    as i64;
                call_info = (*call_info).previous;
            }
        }
    }
    pub unsafe extern "C" fn luad_errerr(& mut self) -> ! {
        unsafe {
            let message: *mut TString = luas_newlstr(
                self,
                b"error in error handling\0" as *const u8 as *const i8,
                (::core::mem::size_of::<[i8; 24]>() as u64)
                    .wrapping_div(::core::mem::size_of::<i8>() as u64)
                    .wrapping_sub(1 as u64),
            );
            let io: *mut TValue = &mut (*self.top.p).tvalue;
            (*io).value.object = &mut (*(message as *mut Object));
            (*io).set_tag((*message).get_tag());
            (*io).set_collectable();
            self.top.p = self.top.p.offset(1);
            luad_throw(self, 5);
        }
    }
    pub unsafe extern "C" fn luae_checkcstack(& mut self) {
        unsafe {
            if self.count_c_calls & 0xffff as u32 == 200 as u32 {
                luag_runerror(self, b"C stack overflow\0" as *const u8 as *const i8);
            } else if self.count_c_calls & 0xffff as u32
                >= (200 as i32 / 10 as i32 * 11 as i32) as u32
            {
                self.luad_errerr();
            }
        }
    }
    pub unsafe extern "C" fn luae_inccstack(& mut self) {
        unsafe {
            self.count_c_calls = (self.count_c_calls).wrapping_add(1);
            self.count_c_calls;
            if ((self.count_c_calls & 0xffff as u32 >= 200 as u32) as i32 != 0) as i32
                as i64
                != 0
            {
                self.luae_checkcstack();
            }
        }
    }
    pub unsafe extern "C" fn stackinuse(& mut self) -> i32 {
        unsafe {
            let mut lim: StackValuePointer = self.top.p;
            let mut call_info: *mut CallInfo = self.call_info;
            while !call_info.is_null() {
                if lim < (*call_info).top.p {
                    lim = (*call_info).top.p;
                }
                call_info = (*call_info).previous;
            }
            let mut res: i32 = lim.offset_from(self.stack.p) as i32 + 1;
            if res < 20 as i32 {
                res = 20 as i32;
            }
            return res;
        }
    }
    pub unsafe extern "C" fn luad_shrinkstack(& mut self) {
        unsafe {
            let inuse: i32 = self.stackinuse();
            let max: i32 = if inuse > 1000000 / 3 {
                1000000
            } else {
                inuse * 3
            };
            if inuse <= 1000000
                && (self.stack_last.p).offset_from(self.stack.p) as i32 > max
            {
                let new_size: i32 = if inuse > 1000000 / 2 {
                    1000000
                } else {
                    inuse * 2
                };
                luad_reallocstack(self, new_size, false);
            }
            luae_shrinkci(self);
        }
    }
    pub unsafe extern "C" fn luad_inctop(& mut self) {
        unsafe {
            if (((self.stack_last.p).offset_from(self.top.p) as i64 <= 1) as i32 != 0) as i32
                as i64
                != 0
            {
                luad_growstack(self, 1, true);
            }
            self.top.p = self.top.p.offset(1);
        }
    }
    pub unsafe extern "C" fn lua_createtable(& mut self) {
        unsafe {
            let table: *mut Table;
            table = luah_new(self);
            let io: *mut TValue = &mut (*self.top.p).tvalue;
            let x_: *mut Table = table;
            (*io).value.object = &mut (*(x_ as *mut Object));
            (*io).set_tag(TAG_VARIANT_TABLE);
            (*io).set_collectable();
            self.top.p = self.top.p.offset(1);
            if (*self.global).gc_debt > 0 {
                luac_step(self);
            }
        }
    }
    pub unsafe extern "C" fn lua_getmetatable(& mut self, object_index: i32) -> bool {
        unsafe {
            let object: *const TValue = self.index2value(object_index);
            let metatable: *mut Table;
            match (*object).get_tag_type() {
                TAG_TYPE_TABLE => {
                    metatable = (*((*object).value.object as *mut Table)).metatable;
                }
                TAG_TYPE_USER => {
                    metatable = (*((*object).value.object as *mut User)).metatable;
                }
                _ => {
                    metatable = (*self.global).metatable[(get_tag_type((*object).get_tag())) as usize];
                }
            }
            if metatable.is_null() {
                false
            } else {
                let io: *mut TValue = &mut (*self.top.p).tvalue;
                (*io).value.object = &mut (*(metatable as *mut Object));
                (*io).set_tag(TAG_VARIANT_TABLE);
                (*io).set_collectable();
                self.top.p = self.top.p.offset(1);
                true
            }
        }
    }
    pub unsafe extern "C" fn lua_getiuservalue(& mut self, index: i32, n: i32) -> i32 {
        unsafe {
            let t: i32;
            let o: *mut TValue = self.index2value(index);
            if n <= 0 || n > (*((*o).value.object as *mut User)).nuvalue as i32 {
                (*self.top.p).tvalue.set_tag(TAG_VARIANT_NIL_NIL);
                t = -1;
            } else {
                let io1: *mut TValue = &mut (*self.top.p).tvalue;
                let io2: *const TValue = &mut (*((*((*o).value.object as *mut User)).uv)
                    .as_mut_ptr()
                    .offset((n - 1) as isize));
                (*io1).value = (*io2).value;
                (*io1).set_tag((*io2).get_tag());
                t = (get_tag_type((*self.top.p).tvalue.get_tag())) as i32;
            }
            self.top.p = self.top.p.offset(1);
            return t;
        }
    }
    pub unsafe extern "C" fn index2value(& mut self, mut index: i32) -> *mut TValue {
        unsafe {
            let call_info: *mut CallInfo = self.call_info;
            if index > 0 {
                let o: StackValuePointer = ((*call_info).function.p).offset(index as isize);
                if o >= self.top.p {
                    return &mut (*self.global).nil_value;
                } else {
                    return &mut (*o).tvalue;
                }
            } else if !(index <= -(1000000 as i32) - 1000 as i32) {
                return &mut (*self.top.p.offset(index as isize)).tvalue;
            } else if index == -(1000000 as i32) - 1000 as i32 {
                return &mut (*self.global).l_registry;
            } else {
                index = -(1000000 as i32) - 1000 as i32 - index;
                let value = (*(*call_info).function.p).tvalue;
                if value.is_collectable() && value.get_tag_variant() == TAG_VARIANT_CLOSURE_C {
                    let function: *mut Closure = &mut (*(value.value.object as *mut Closure));
                    return if index <= (*function).count_upvalues as i32 {
                        &mut *((*function).upvalues).
                            c_tvalues.as_mut_ptr()
                            .offset((index - 1) as isize) as *mut TValue
                    } else {
                        &mut (*self.global).nil_value
                    };
                } else {
                    return &mut (*self.global).nil_value;
                }
            };
        }
    }
}
pub unsafe extern "C" fn do_repl(state: *mut State) {
    unsafe {
        let mut status: i32;
        let oldprogname: *const i8 = PROGRAM_NAME;
        PROGRAM_NAME = std::ptr::null();
        loop {
            status = loadline(state);
            if !(status != -1) {
                break;
            }
            if status == 0 {
                status = docall(state, 0, -1);
            }
            if status == 0 {
                l_print(state);
            } else {
                report(state, status);
            }
        }
        lua_settop(state, 0);
        fwrite(
            b"\n\0" as *const u8 as *const i8 as *const libc::c_void,
            ::core::mem::size_of::<i8>() as u64,
            1 as u64,
            stdout,
        );
        fflush(stdout);
        PROGRAM_NAME = oldprogname;
    }
}
pub unsafe extern "C" fn luad_throw(state: *mut State, mut error_code: i32) -> ! {
    unsafe {
        if !((*state).long_jump).is_null() {
            ::core::ptr::write_volatile(&mut (*(*state).long_jump).status as *mut i32, error_code);
            _longjmp(((*(*state).long_jump).jbt).as_mut_ptr(), 1);
        } else {
            let g: *mut Global = (*state).global;
            error_code = luae_resetthread(state, error_code);
            (*state).status = error_code as u8;
            if !((*(*g).main_state).long_jump).is_null() {
                let fresh0 = (*(*g).main_state).top.p;
                (*(*g).main_state).top.p = ((*(*g).main_state).top.p).offset(1);
                let io1: *mut TValue = &mut (*fresh0).tvalue;
                let io2: *const TValue = &mut (*(*state).top.p.offset(-(1 as isize))).tvalue;
                (*io1).value = (*io2).value;
                (*io1).set_tag((*io2).get_tag());
                luad_throw((*g).main_state, error_code);
            } else {
                if ((*g).panic).is_some() {
                    ((*g).panic).expect("non-null function pointer")(state);
                }
                abort();
            }
        };
    }
}
pub unsafe extern "C" fn luad_rawrunprotected(
    state: *mut State,
    f: ProtectedFunction,
    arbitrary_data: *mut libc::c_void,
) -> i32 {
    unsafe {
        let old_count_c_calls: u32 = (*state).count_c_calls;
        let mut long_jump = LongJump::new();
        ::core::ptr::write_volatile(&mut long_jump.status as *mut i32, 0);
        long_jump.previous = (*state).long_jump;
        (*state).long_jump = &mut long_jump;
        if _setjmp((long_jump.jbt).as_mut_ptr()) == 0 {
            (Some(f.expect("non-null function pointer"))).expect("non-null function pointer")(
                state, arbitrary_data,
            );
        }
        (*state).long_jump = long_jump.previous;
        (*state).count_c_calls = old_count_c_calls;
        return long_jump.status;
    }
}
pub unsafe extern "C" fn luad_reallocstack(
    state: *mut State,
    new_size: i32,
    should_raise_error: bool,
) -> i32 {
    unsafe {
        let old_size: i32 = ((*state).stack_last.p).offset_from((*state).stack.p) as i32;
        let oldgcstop: i32 = (*(*state).global).gcstopem as i32;
        (*state).relstack();
        (*(*state).global).gcstopem = 1;
        let newstack: StackValuePointer = luam_realloc_(
            state,
            (*state).stack.p as *mut libc::c_void,
            ((old_size + 5) as u64).wrapping_mul(::core::mem::size_of::<StackValue>() as u64),
            ((new_size + 5) as u64).wrapping_mul(::core::mem::size_of::<StackValue>() as u64),
        ) as *mut StackValue;
        (*(*state).global).gcstopem = oldgcstop as u8;
        if ((newstack == std::ptr::null_mut() as StackValuePointer) as i32 != 0) as i64 != 0 {
            (*state).correct_stack();
            if should_raise_error {
                luad_throw(state, 4);
            } else {
                return 0;
            }
        }
        (*state).stack.p = newstack;
        (*state).correct_stack();
        (*state).stack_last.p = ((*state).stack.p).offset(new_size as isize);
        let mut i: i32 = old_size + 5;
        while i < new_size + 5 {
            (*newstack.offset(i as isize))
                .tvalue
                .set_tag(TAG_VARIANT_NIL_NIL);
            i += 1;
        }
        return 1;
    }
}
pub unsafe extern "C" fn luad_growstack(
    state: *mut State,
    n: i32,
    should_raise_error: bool,
) -> i32 {
    unsafe {
        let size: i32 = ((*state).stack_last.p).offset_from((*state).stack.p) as i32;
        if size > 1000000 {
            if should_raise_error {
                (*state).luad_errerr();
            }
            return 0;
        } else if n < 1000000 {
            let mut new_size: i32 = 2 * size;
            let needed: i32 = ((*state).top.p).offset_from((*state).stack.p) as i32 + n;
            if new_size > 1000000 {
                new_size = 1000000;
            }
            if new_size < needed {
                new_size = needed;
            }
            if new_size <= 1000000 {
                return luad_reallocstack(state, new_size, should_raise_error);
            }
        }
        luad_reallocstack(state, 1000000 + 200, should_raise_error);
        if should_raise_error {
            luag_runerror(state, b"stack overflow\0" as *const u8 as *const i8);
        }
        return 0;
    }
}
pub unsafe extern "C" fn luad_hook(
    state: *mut State,
    event: i32,
    line: i32,
    ftransfer: i32,
    ntransfer: i32,
) {
    unsafe {
        let hook: HookFunction = (*state).hook;
        if hook.is_some() && (*state).allow_hook as i32 != 0 {
            let mut mask: i32 = 1 << 3;
            let call_info: *mut CallInfo = (*state).call_info;
            let top: i64 =
                ((*state).top.p as *mut i8).offset_from((*state).stack.p as *mut i8) as i64;
            let ci_top: i64 =
                ((*call_info).top.p as *mut i8).offset_from((*state).stack.p as *mut i8) as i64;
            let mut ar: DebugInfo = DebugInfo {
                event: 0,
                name: std::ptr::null(),
                namewhat: std::ptr::null(),
                what: std::ptr::null(),
                source: std::ptr::null(),
                source_length: 0,
                currentline: 0,
                line_defined: 0,
                last_line_defined: 0,
                nups: 0,
                nparams: 0,
                is_variable_arguments: false,
                is_tail_call: false,
                ftransfer: 0,
                ntransfer: 0,
                short_src: [0; 60],
                i_ci: std::ptr::null_mut(),
            };
            ar.event = event;
            ar.currentline = line;
            ar.i_ci = call_info;
            if ntransfer != 0 {
                mask |= 1 << 8;
                (*call_info).u2.transferinfo.ftransfer = ftransfer as u16;
                (*call_info).u2.transferinfo.ntransfer = ntransfer as u16;
            }
            if (*call_info).call_status as i32 & 1 << 1 == 0 && (*state).top.p < (*call_info).top.p
            {
                (*state).top.p = (*call_info).top.p;
            }
            if ((((*state).stack_last.p).offset_from((*state).top.p) as i64 <= 20 as i64)
                as i32
                != 0) as i64
                != 0
            {
                luad_growstack(state, 20 as i32, true);
            }
            if (*call_info).top.p < (*state).top.p.offset(20 as isize) {
                (*call_info).top.p = (*state).top.p.offset(20 as isize);
            }
            (*state).allow_hook = 0;
            (*call_info).call_status = ((*call_info).call_status as i32 | mask) as u16;
            (Some(hook.expect("non-null function pointer"))).expect("non-null function pointer")(
                state, &mut ar,
            );
            (*state).allow_hook = 1;
            (*call_info).top.p = ((*state).stack.p as *mut i8).offset(ci_top as isize) as StackValuePointer;
            (*state).top.p = ((*state).stack.p as *mut i8).offset(top as isize) as StackValuePointer;
            (*call_info).call_status = ((*call_info).call_status as i32 & !mask) as u16;
        }
    }
}
pub unsafe extern "C" fn luad_hookcall(state: *mut State, call_info: *mut CallInfo) {
    unsafe {
        (*state).old_program_counter = 0;
        if (*state).hook_mask & (1 << 0) != 0 {
            let event: i32 = if ((*call_info).call_status & (1 << 5)) != 0 {
                4
            } else {
                0
            };
            let p: *mut Prototype = (*((*(*call_info).function.p).tvalue.value.object
                as *mut Closure)).payload.l_prototype;
            (*call_info).u.l.saved_program_counter =
                ((*call_info).u.l.saved_program_counter).offset(1);
            (*call_info).u.l.saved_program_counter;
            luad_hook(state, event, -1, 1, (*p).count_parameters as i32);
            (*call_info).u.l.saved_program_counter =
                ((*call_info).u.l.saved_program_counter).offset(-1);
            (*call_info).u.l.saved_program_counter;
        }
    }
}
pub unsafe extern "C" fn rethook(state: *mut State, mut call_info: *mut CallInfo, nres: i32) {
    unsafe {
        if (*state).hook_mask & 1 << 1 != 0 {
            let firstres: StackValuePointer = (*state).top.p.offset(-(nres as isize));
            let mut delta: i32 = 0;
            if (*call_info).call_status as i32 & 1 << 1 == 0 {
                let p: *mut Prototype = (*((*(*call_info).function.p).tvalue.value.object
                    as *mut Closure)).payload.l_prototype;
                if (*p).is_variable_arguments {
                    delta =
                        (*call_info).u.l.count_extra_arguments + (*p).count_parameters as i32 + 1;
                }
            }
            (*call_info).function.p = ((*call_info).function.p).offset(delta as isize);
            let ftransfer: i32 = firstres.offset_from((*call_info).function.p) as i32;
            luad_hook(state, 1, -1, ftransfer, nres);
            (*call_info).function.p = ((*call_info).function.p).offset(-(delta as isize));
        }
        call_info = (*call_info).previous;
        if (*call_info).call_status as i32 & 1 << 1 == 0 {
            (*state).old_program_counter = ((*call_info).u.l.saved_program_counter).offset_from(
                (*(*((*(*call_info).function.p).tvalue.value.object as *mut Closure))
                    .payload.l_prototype)
                    .code,
            ) as i32
                - 1;
        }
    }
}
pub unsafe extern "C" fn tryfunctm(state: *mut State, mut function: StackValuePointer) -> StackValuePointer {
    unsafe {
        let mut p: StackValuePointer;
        if ((((*state).stack_last.p).offset_from((*state).top.p) as i64 <= 1) as i32 != 0) as i32
            as i64
            != 0
        {
            let t__: i64 = (function as *mut i8).offset_from((*state).stack.p as *mut i8) as i64;
            if (*(*state).global).gc_debt > 0 {
                luac_step(state);
            }
            luad_growstack(state, 1, true);
            function = ((*state).stack.p as *mut i8).offset(t__ as isize) as StackValuePointer;
        }
        let tm: *const TValue = luat_gettmbyobj(state, &mut (*function).tvalue, TM_CALL);
        if (*tm).get_tag_type() == TAG_TYPE_NIL {
            luag_callerror(state, &mut (*function).tvalue);
        }
        p = (*state).top.p;
        while p > function {
            let io1: *mut TValue = &mut (*p).tvalue;
            let io2: *const TValue = &mut (*p.offset(-(1 as isize))).tvalue;
            (*io1).value = (*io2).value;
            (*io1).set_tag((*io2).get_tag());
            p = p.offset(-1);
        }
        (*state).top.p = (*state).top.p.offset(1);
        let io1_0: *mut TValue = &mut (*function).tvalue;
        let io2_0: *const TValue = tm;
        (*io1_0).value = (*io2_0).value;
        (*io1_0).set_tag((*io2_0).get_tag());
        return function;
    }
}
pub unsafe extern "C" fn moveresults(
    state: *mut State,
    mut res: StackValuePointer,
    mut nres: i32,
    mut wanted: i32,
) {
    unsafe {
        let firstresult: StackValuePointer;
        let mut i: i32;
        match wanted {
            0 => {
                (*state).top.p = res;
                return;
            }
            1 => {
                if nres == 0 {
                    (*res).tvalue.set_tag(TAG_VARIANT_NIL_NIL);
                } else {
                    let io1: *mut TValue = &mut (*res).tvalue;
                    let io2: *const TValue = &mut (*(*state).top.p.offset(-(nres as isize))).tvalue;
                    (*io1).value = (*io2).value;
                    (*io1).set_tag((*io2).get_tag());
                }
                (*state).top.p = res.offset(1 as isize);
                return;
            }
            -1 => {
                wanted = nres;
            }
            _ => {
                if wanted < -1 {
                    (*(*state).call_info).call_status =
                        ((*(*state).call_info).call_status as i32 | 1 << 9 as i32) as u16;
                    (*(*state).call_info).u2.nres = nres;
                    res = luaf_close(state, res, -1, 1);
                    (*(*state).call_info).call_status =
                        ((*(*state).call_info).call_status as i32 & !(1 << 9 as i32)) as u16;
                    if (*state).hook_mask != 0 {
                        let savedres: i64 =
                            (res as *mut i8).offset_from((*state).stack.p as *mut i8) as i64;
                        rethook(state, (*state).call_info, nres);
                        res = ((*state).stack.p as *mut i8).offset(savedres as isize) as StackValuePointer;
                    }
                    wanted = -wanted - 3;
                    if wanted == -1 {
                        wanted = nres;
                    }
                }
            }
        }
        firstresult = (*state).top.p.offset(-(nres as isize));
        if nres > wanted {
            nres = wanted;
        }
        i = 0;
        while i < nres {
            let io1_0: *mut TValue = &mut (*res.offset(i as isize)).tvalue;
            let io2_0: *const TValue = &mut (*firstresult.offset(i as isize)).tvalue;
            (*io1_0).value = (*io2_0).value;
            (*io1_0).set_tag((*io2_0).get_tag());
            i += 1;
        }
        while i < wanted {
            (*res.offset(i as isize)).tvalue.set_tag(TAG_VARIANT_NIL_NIL);
            i += 1;
        }
        (*state).top.p = res.offset(wanted as isize);
    }
}
pub unsafe extern "C" fn luad_poscall(state: *mut State, call_info: *mut CallInfo, nres: i32) {
    unsafe {
        let wanted: i32 = (*call_info).count_results as i32;
        if (((*state).hook_mask != 0 && !(wanted < -1)) as i32 != 0) as i64 != 0 {
            rethook(state, call_info, nres);
        }
        moveresults(state, (*call_info).function.p, nres, wanted);
        (*state).call_info = (*call_info).previous;
    }
}
pub unsafe extern "C" fn prepcallinfo(
    state: *mut State,
    function: StackValuePointer,
    nret: i32,
    mask: i32,
    top: StackValuePointer,
) -> *mut CallInfo {
    unsafe {
        (*state).call_info = if !((*(*state).call_info).next).is_null() {
            (*(*state).call_info).next
        } else {
            luae_extendci(state)
        };
        let call_info: *mut CallInfo = (*state).call_info;
        (*call_info).function.p = function;
        (*call_info).count_results = nret as i16;
        (*call_info).call_status = mask as u16;
        (*call_info).top.p = top;
        return call_info;
    }
}
pub unsafe extern "C" fn precallc(
    state: *mut State,
    mut function: StackValuePointer,
    count_results: i32,
    f: CFunction,
) -> i32 {
    unsafe {
        if ((((*state).stack_last.p).offset_from((*state).top.p) as i64 <= 20 as i64) as i32
            != 0) as i64
            != 0
        {
            let t__: i64 = (function as *mut i8).offset_from((*state).stack.p as *mut i8) as i64;
            if (*(*state).global).gc_debt > 0 {
                luac_step(state);
            }
            luad_growstack(state, 20 as i32, true);
            function = ((*state).stack.p as *mut i8).offset(t__ as isize) as StackValuePointer;
        }
        let call_info = prepcallinfo(
            state,
            function,
            count_results,
            1 << 1,
            (*state).top.p.offset(20 as isize),
        );
        (*state).call_info = call_info;
        if ((*state).hook_mask & 1 << 0 != 0) as i64 != 0 {
            let narg: i32 = ((*state).top.p).offset_from(function) as i32 - 1;
            luad_hook(state, 0, -1, 1, narg);
        }
        let n: i32 = (Some(f.expect("non-null function pointer")))
            .expect("non-null function pointer")(state);
        luad_poscall(state, call_info, n);
        return n;
    }
}
pub unsafe extern "C" fn luad_pretailcall(
    state: *mut State,
    call_info: *mut CallInfo,
    mut function: StackValuePointer,
    mut narg1: i32,
    delta: i32,
) -> i32 {
    unsafe {
        loop {
            match (*function).tvalue.get_tag_variant() {
                TAG_VARIANT_CLOSURE_C => {
                    return precallc(
                        state,
                        function,
                        -1,
                        (*((*function).tvalue.value.object as *mut Closure)).payload.c_cfunction,
                    );
                }
                TAG_VARIANT_CLOSURE_CFUNCTION => {
                    return precallc(state, function, -1, (*function).tvalue.value.function)
                }
                TAG_VARIANT_CLOSURE_L => {
                    let p: *mut Prototype =
                        (*((*function).tvalue.value.object as *mut Closure)).payload.l_prototype;
                    let fsize: i32 = (*p).maximum_stack_size as i32;
                    let nfixparams: i32 = (*p).count_parameters as i32;
                    let mut i: i32;
                    if ((((*state).stack_last.p).offset_from((*state).top.p) as i64
                        <= (fsize - delta) as i64) as i32
                        != 0) as i64
                        != 0
                    {
                        let t__: i64 =
                            (function as *mut i8).offset_from((*state).stack.p as *mut i8) as i64;
                        if (*(*state).global).gc_debt > 0 {
                            luac_step(state);
                        }
                        luad_growstack(state, fsize - delta, true);
                        function = ((*state).stack.p as *mut i8).offset(t__ as isize) as StackValuePointer;
                    }
                    (*call_info).function.p = ((*call_info).function.p).offset(-(delta as isize));
                    i = 0;
                    while i < narg1 {
                        let io1: *mut TValue =
                            &mut (*((*call_info).function.p).offset(i as isize)).tvalue;
                        let io2: *const TValue = &mut (*function.offset(i as isize)).tvalue;
                        (*io1).value = (*io2).value;
                        (*io1).set_tag((*io2).get_tag());
                        i += 1;
                    }
                    function = (*call_info).function.p;
                    while narg1 <= nfixparams {
                        (*function.offset(narg1 as isize))
                            .tvalue
                            .set_tag(TAG_VARIANT_NIL_NIL);
                        narg1 += 1;
                    }
                    (*call_info).top.p = function.offset(1 as isize).offset(fsize as isize);
                    (*call_info).u.l.saved_program_counter = (*p).code;
                    (*call_info).call_status = ((*call_info).call_status as i32 | 1 << 5) as u16;
                    (*state).top.p = function.offset(narg1 as isize);
                    return -1;
                }
                _ => {
                    function = tryfunctm(state, function);
                    narg1 += 1;
                }
            }
        }
    }
}
pub unsafe extern "C" fn luad_precall(
    state: *mut State,
    mut function: StackValuePointer,
    count_results: i32,
) -> *mut CallInfo {
    unsafe {
        loop {
            match (*function).tvalue.get_tag_variant() {
                TAG_VARIANT_CLOSURE_C => {
                    precallc(
                        state,
                        function,
                        count_results,
                        (*((*function).tvalue.value.object as *mut Closure)).payload.c_cfunction,
                    );
                    return std::ptr::null_mut();
                }
                TAG_VARIANT_CLOSURE_CFUNCTION => {
                    precallc(state, function, count_results, (*function).tvalue.value.function);
                    return std::ptr::null_mut();
                }
                TAG_VARIANT_CLOSURE_L => {
                    let call_info;
                    let p: *mut Prototype =
                        (*((*function).tvalue.value.object as *mut Closure)).payload.l_prototype;
                    let mut narg: i32 = ((*state).top.p).offset_from(function) as i32 - 1;
                    let nfixparams: i32 = (*p).count_parameters as i32;
                    let fsize: i32 = (*p).maximum_stack_size as i32;
                    if ((((*state).stack_last.p).offset_from((*state).top.p) as i64 <= fsize as i64)
                        as i32
                        != 0) as i64
                        != 0
                    {
                        let t__: i64 =
                            (function as *mut i8).offset_from((*state).stack.p as *mut i8) as i64;
                        if (*(*state).global).gc_debt > 0 {
                            luac_step(state);
                        }
                        luad_growstack(state, fsize, true);
                        function = ((*state).stack.p as *mut i8).offset(t__ as isize) as StackValuePointer;
                    }
                    call_info = prepcallinfo(
                        state,
                        function,
                        count_results,
                        0,
                        function.offset(1 as isize).offset(fsize as isize),
                    );
                    (*state).call_info = call_info;
                    (*call_info).u.l.saved_program_counter = (*p).code;
                    while narg < nfixparams {
                        let fresh1 = (*state).top.p;
                        (*state).top.p = (*state).top.p.offset(1);
                        (*fresh1).tvalue.set_tag(TAG_VARIANT_NIL_NIL);
                        narg += 1;
                    }
                    return call_info;
                }
                _ => {
                    function = tryfunctm(state, function);
                }
            }
        }
    }
}
pub unsafe extern "C" fn ccall(
    state: *mut State,
    mut function: StackValuePointer,
    count_results: i32,
    inc: u32,
) {
    unsafe {
        let call_info;
        (*state).count_c_calls = ((*state).count_c_calls as u32).wrapping_add(inc) as u32;
        if (((*state).count_c_calls & 0xffff as u32 >= 200 as u32) as i32 != 0) as i32
            as i64
            != 0
        {
            if ((((*state).stack_last.p).offset_from((*state).top.p) as i64 <= 0) as i32 != 0)
                as i64
                != 0
            {
                let t__: i64 =
                    (function as *mut i8).offset_from((*state).stack.p as *mut i8) as i64;
                luad_growstack(state, 0, true);
                function = ((*state).stack.p as *mut i8).offset(t__ as isize) as StackValuePointer;
            }
            (*state).luae_checkcstack();
        }
        call_info = luad_precall(state, function, count_results);
        if !call_info.is_null() {
            (*call_info).call_status = (1 << 2) as u16;
            luav_execute(state, call_info);
        }
        (*state).count_c_calls = ((*state).count_c_calls as u32).wrapping_sub(inc) as u32;
    }
}
pub unsafe extern "C" fn luad_callnoyield(state: *mut State, function: StackValuePointer, count_results: i32) {
    unsafe {
        ccall(state, function, count_results, (0x10000 as i32 | 1) as u32);
    }
}
pub unsafe extern "C" fn finishpcallk(state: *mut State, call_info: *mut CallInfo) -> i32 {
    unsafe {
        let mut status: i32 = (*call_info).call_status as i32 >> 10 as i32 & 7;
        if ((status == 0) as i32 != 0) as i64 != 0 {
            status = 1;
        } else {
            let mut function: StackValuePointer =
                ((*state).stack.p as *mut i8).offset((*call_info).u2.funcidx as isize) as StackValuePointer;
            (*state).allow_hook = ((*call_info).call_status as i32 & 1 << 0) as u8;
            function = luaf_close(state, function, status, 1);
            (*state).set_error_object(status, function);
            (*state).luad_shrinkstack();
            (*call_info).call_status =
                ((*call_info).call_status as i32 & !((7) << 10 as i32) | 0 << 10 as i32) as u16;
        }
        (*call_info).call_status = ((*call_info).call_status as i32 & !(1 << 4)) as u16;
        (*state).error_function = (*call_info).u.c.old_error_function;
        return status;
    }
}
pub unsafe extern "C" fn finishccall(state: *mut State, call_info: *mut CallInfo) {
    unsafe {
        let n: i32;
        if (*call_info).call_status as i32 & 1 << 9 as i32 != 0 {
            n = (*call_info).u2.nres;
        } else {
            let mut status: i32 = 1;
            if (*call_info).call_status as i32 & 1 << 4 != 0 {
                status = finishpcallk(state, call_info);
            }
            if -1 <= -1 && (*(*state).call_info).top.p < (*state).top.p {
                (*(*state).call_info).top.p = (*state).top.p;
            }
            n = (Some(((*call_info).u.c.context_function).expect("non-null function pointer")))
                .expect("non-null function pointer")(
                state, status, (*call_info).u.c.context
            );
        }
        luad_poscall(state, call_info, n);
    }
}
pub unsafe extern "C" fn unroll(state: *mut State, mut _ud: *mut libc::c_void) {
    unsafe {
        let mut call_info;
        loop {
            call_info = (*state).call_info;
            if !(call_info != &mut (*state).base_callinfo as *mut CallInfo) {
                break;
            }
            if (*call_info).call_status as i32 & 1 << 1 != 0 {
                finishccall(state, call_info);
            } else {
                luav_finishop(state);
                luav_execute(state, call_info);
            }
        }
    }
}
pub unsafe extern "C" fn resume_error(state: *mut State, message: *const i8, narg: i32) -> i32 {
    unsafe {
        (*state).top.p = (*state).top.p.offset(-(narg as isize));
        let io: *mut TValue = &mut (*(*state).top.p).tvalue;
        let x_: *mut TString = luas_new(state, message);
        (*io).value.object = &mut (*(x_ as *mut Object));
        (*io).set_tag((*x_).get_tag());
        (*io).set_collectable();
        (*state).top.p = (*state).top.p.offset(1);
        return 2;
    }
}
pub unsafe extern "C" fn resume(state: *mut State, arbitrary_data: *mut libc::c_void) {
    unsafe {
        let mut n: i32 = *(arbitrary_data as *mut i32);
        let first_argument: StackValuePointer = (*state).top.p.offset(-(n as isize));
        let call_info: *mut CallInfo = (*state).call_info;
        if (*state).status as i32 == 0 {
            ccall(state, first_argument.offset(-(1 as isize)), -1, 0);
        } else {
            (*state).status = 0;
            if (*call_info).call_status as i32 & 1 << 1 == 0 {
                (*call_info).u.l.saved_program_counter =
                    ((*call_info).u.l.saved_program_counter).offset(-1);
                (*call_info).u.l.saved_program_counter;
                (*state).top.p = first_argument;
                luav_execute(state, call_info);
            } else {
                if ((*call_info).u.c.context_function).is_some() {
                    n = (Some(((*call_info).u.c.context_function).expect("non-null function pointer")))
                        .expect("non-null function pointer")(
                        state, 1, (*call_info).u.c.context
                    );
                }
                luad_poscall(state, call_info, n);
            }
            unroll(state, std::ptr::null_mut());
        };
    }
}
pub unsafe extern "C" fn precover(state: *mut State, mut status: i32) -> i32 {
    unsafe {
        let mut call_info;
        while status > 1 && {
            call_info = (*state).find_pcall();
            !call_info.is_null()
        } {
            (*state).call_info = call_info;
            (*call_info).call_status = ((*call_info).call_status as i32 & !((7) << 10 as i32)
                | status << 10 as i32) as u16;
            status = luad_rawrunprotected(
                state,
                Some(unroll as unsafe extern "C" fn(*mut State, *mut libc::c_void) -> ()),
                std::ptr::null_mut(),
            );
        }
        return status;
    }
}
pub unsafe extern "C" fn lua_resume(
    state: *mut State,
    from: *mut State,
    mut nargs: i32,
    count_results: *mut i32,
) -> i32 {
    unsafe {
        let mut status;
        if (*state).status as i32 == 0 {
            if (*state).call_info != &mut (*state).base_callinfo as *mut CallInfo {
                return resume_error(
                    state,
                    b"cannot resume non-suspended coroutine\0" as *const u8 as *const i8,
                    nargs,
                );
            } else if ((*state).top.p)
                .offset_from(((*(*state).call_info).function.p).offset(1 as isize))
                as i64
                == nargs as i64
            {
                return resume_error(
                    state,
                    b"cannot resume dead coroutine\0" as *const u8 as *const i8,
                    nargs,
                );
            }
        } else if (*state).status as i32 != 1 {
            return resume_error(
                state,
                b"cannot resume dead coroutine\0" as *const u8 as *const i8,
                nargs,
            );
        }
        (*state).count_c_calls = if !from.is_null() {
            (*from).count_c_calls & 0xffff as u32
        } else {
            0
        };
        if (*state).count_c_calls & 0xffff as u32 >= 200 as u32 {
            return resume_error(
                state,
                b"C stack overflow\0" as *const u8 as *const i8,
                nargs,
            );
        }
        (*state).count_c_calls = ((*state).count_c_calls).wrapping_add(1);
        (*state).count_c_calls;
        status = luad_rawrunprotected(
            state,
            Some(resume as unsafe extern "C" fn(*mut State, *mut libc::c_void) -> ()),
            &mut nargs as *mut i32 as *mut libc::c_void,
        );
        status = precover(state, status);
        if !((!(status > 1) as i32 != 0) as i64 != 0) {
            (*state).status = status as u8;
            (*state).set_error_object(status, (*state).top.p);
            (*(*state).call_info).top.p = (*state).top.p;
        }
        *count_results = if status == 1 {
            (*(*state).call_info).u2.nyield
        } else {
            ((*state).top.p).offset_from(((*(*state).call_info).function.p).offset(1 as isize))
                as i32
        };
        return status;
    }
}
pub unsafe extern "C" fn lua_yieldk(
    state: *mut State,
    count_results: i32,
    ctx: i64,
    k: ContextFunction,
) -> i32 {
    unsafe {
        let call_info;
        call_info = (*state).call_info;
        if (!((*state).count_c_calls & 0xffff0000 as u32 == 0) as i32 != 0) as i64 != 0 {
            if state != (*(*state).global).main_state {
                luag_runerror(
                    state,
                    b"attempt to yield across a C-call boundary\0" as *const u8 as *const i8,
                );
            } else {
                luag_runerror(
                    state,
                    b"attempt to yield from outside a coroutine\0" as *const u8 as *const i8,
                );
            }
        }
        (*state).status = 1;
        (*call_info).u2.nyield = count_results;
        if (*call_info).call_status as i32 & 1 << 1 == 0 {
        } else {
            (*call_info).u.c.context_function = k;
            if ((*call_info).u.c.context_function).is_some() {
                (*call_info).u.c.context = ctx;
            }
            luad_throw(state, 1);
        }
        return 0;
    }
}
pub unsafe extern "C" fn closepaux(state: *mut State, arbitrary_data: *mut libc::c_void) {
    unsafe {
        let closep: *mut CloseP = arbitrary_data as *mut CloseP;
        luaf_close(state, (*closep).level, (*closep).status, 0);
    }
}
pub unsafe extern "C" fn luad_closeprotected(
    state: *mut State,
    level: i64,
    mut status: i32,
) -> i32 {
    unsafe {
        let old_call_info: *mut CallInfo = (*state).call_info;
        let old_allowhooks: u8 = (*state).allow_hook;
        loop {
            let mut closep = CloseP::new();
            closep.level = ((*state).stack.p as *mut i8).offset(level as isize) as StackValuePointer;
            closep.status = status;
            status = luad_rawrunprotected(
                state,
                Some(closepaux as unsafe extern "C" fn(*mut State, *mut libc::c_void) -> ()),
                &mut closep as *mut CloseP as *mut libc::c_void,
            );
            if ((status == 0) as i32 != 0) as i64 != 0 {
                return closep.status;
            } else {
                (*state).call_info = old_call_info;
                (*state).allow_hook = old_allowhooks;
            }
        }
    }
}
pub unsafe extern "C" fn luad_pcall(
    state: *mut State,
    function: ProtectedFunction,
    u: *mut libc::c_void,
    old_top: i64,
    ef: i64,
) -> i32 {
    unsafe {
        let mut status: i32;
        let old_call_info: *mut CallInfo = (*state).call_info;
        let old_allowhooks: u8 = (*state).allow_hook;
        let old_error_function: i64 = (*state).error_function;
        (*state).error_function = ef;
        status = luad_rawrunprotected(state, function, u);
        if ((status != 0) as i32 != 0) as i64 != 0 {
            (*state).call_info = old_call_info;
            (*state).allow_hook = old_allowhooks;
            status = luad_closeprotected(state, old_top, status);
            (*state).set_error_object(
                status,
                ((*state).stack.p as *mut i8).offset(old_top as isize) as StackValuePointer,
            );
            (*state).luad_shrinkstack();
        }
        (*state).error_function = old_error_function;
        return status;
    }
}
pub unsafe extern "C" fn checkmode(state: *mut State, mode: *const i8, x: *const i8) {
    unsafe {
        if !mode.is_null() && (strchr(mode, *x.offset(0) as i32)).is_null() {
            luao_pushfstring(
                state,
                b"attempt to load a %s chunk (mode is '%s')\0" as *const u8 as *const i8,
                x,
                mode,
            );
            luad_throw(state, 3);
        }
    }
}
pub unsafe extern "C" fn f_parser(state: *mut State, arbitrary_data: *mut libc::c_void) {
    unsafe {
        let cl: *mut Closure;
        let p: *mut SParser = arbitrary_data as *mut SParser;
        let fresh2 = (*(*p).zio).n;
        (*(*p).zio).n = ((*(*p).zio).n).wrapping_sub(1);
        let c: i32 = if fresh2 > 0u64 {
            let fresh3 = (*(*p).zio).p;
            (*(*p).zio).p = ((*(*p).zio).p).offset(1);
            *fresh3 as u8 as i32
        } else {
            luaz_fill((*p).zio)
        };
        if c == (*::core::mem::transmute::<&[u8; 5], &[i8; 5]>(b"\x1BLua\0"))[0] as i32 {
            checkmode(state, (*p).mode, b"binary\0" as *const u8 as *const i8);
            cl = luau_undump(state, (*p).zio, (*p).name);
        } else {
            checkmode(state, (*p).mode, b"text\0" as *const u8 as *const i8);
            cl = luay_parser(
                state,
                (*p).zio,
                &mut (*p).buffer,
                &mut (*p).dynamic_data,
                (*p).name,
                c,
            );
        }
        luaf_initupvals(state, cl);
    }
}
pub unsafe extern "C" fn luad_protectedparser(
    state: *mut State,
    zio: *mut ZIO,
    name: *const i8,
    mode: *const i8,
) -> i32 {
    unsafe {
        let mut p: SParser = SParser {
            zio: std::ptr::null_mut(),
            buffer: Buffer::new(),
            dynamic_data: DynamicData {
                active_variable: DynamicDataActiveVariable {
                    pointer: std::ptr::null_mut(),
                    length: 0,
                    size: 0,
                },
                gt: LabelList {
                    pointer: std::ptr::null_mut(),
                    n: 0,
                    size: 0,
                },
                label: LabelList {
                    pointer: std::ptr::null_mut(),
                    n: 0,
                    size: 0,
                },
            },
            mode: std::ptr::null(),
            name: std::ptr::null(),
        };
        (*state).count_c_calls =
            ((*state).count_c_calls as u32).wrapping_add(0x10000 as u32) as u32;
        p.zio = zio;
        p.name = name;
        p.mode = mode;
        p.dynamic_data.active_variable.pointer = std::ptr::null_mut();
        p.dynamic_data.active_variable.size = 0;
        p.dynamic_data.gt.pointer = std::ptr::null_mut();
        p.dynamic_data.gt.size = 0;
        p.dynamic_data.label.pointer = std::ptr::null_mut();
        p.dynamic_data.label.size = 0;
        p.buffer.pointer = std::ptr::null_mut();
        p.buffer.size = 0;
        let status = luad_pcall(
            state,
            Some(f_parser as unsafe extern "C" fn(*mut State, *mut libc::c_void) -> ()),
            &mut p as *mut SParser as *mut libc::c_void,
            ((*state).top.p as *mut i8).offset_from((*state).stack.p as *mut i8) as i64,
            (*state).error_function,
        );
        p.buffer.pointer = luam_saferealloc_(
            state,
            p.buffer.pointer as *mut libc::c_void,
            (p.buffer.size).wrapping_mul(::core::mem::size_of::<i8>() as u64),
            (0u64).wrapping_mul(::core::mem::size_of::<i8>() as u64),
        ) as *mut i8;
        p.buffer.size = 0;
        (*state).free_memory(
            p.dynamic_data.active_variable.pointer as *mut libc::c_void,
            (p.dynamic_data.active_variable.size as u64)
                .wrapping_mul(::core::mem::size_of::<VariableDescription>() as u64),
        );
        (*state).free_memory(
            p.dynamic_data.gt.pointer as *mut libc::c_void,
            (p.dynamic_data.gt.size as u64)
                .wrapping_mul(::core::mem::size_of::<LabelDescription>() as u64),
        );
        (*state).free_memory(
            p.dynamic_data.label.pointer as *mut libc::c_void,
            (p.dynamic_data.label.size as u64)
                .wrapping_mul(::core::mem::size_of::<LabelDescription>() as u64),
        );
        (*state).count_c_calls =
            ((*state).count_c_calls as u32).wrapping_sub(0x10000 as u32) as u32;
        return status;
    }
}
pub unsafe extern "C" fn index2stack(state: *mut State, index: i32) -> StackValuePointer {
    unsafe {
        let call_info: *mut CallInfo = (*state).call_info;
        if index > 0 {
            let o: StackValuePointer = ((*call_info).function.p).offset(index as isize);
            return o;
        } else {
            return (*state).top.p.offset(index as isize);
        };
    }
}
pub unsafe extern "C" fn lua_checkstack(state: *mut State, n: i32) -> i32 {
    unsafe {
        let res: i32;
        let call_info;
        call_info = (*state).call_info;
        if ((*state).stack_last.p).offset_from((*state).top.p) as i64 > n as i64 {
            res = 1;
        } else {
            res = luad_growstack(state, n, false);
        }
        if res != 0 && (*call_info).top.p < (*state).top.p.offset(n as isize) {
            (*call_info).top.p = (*state).top.p.offset(n as isize);
        }
        return res;
    }
}
pub unsafe extern "C" fn lua_xmove(from: *mut State, to: *mut State, n: i32) {
    unsafe {
        let mut i: i32;
        if from == to {
            return;
        }
        (*from).top.p = ((*from).top.p).offset(-(n as isize));
        i = 0;
        while i < n {
            let io1: *mut TValue = &mut (*(*to).top.p).tvalue;
            let io2: *const TValue = &mut (*((*from).top.p).offset(i as isize)).tvalue;
            (*io1).value = (*io2).value;
            (*io1).set_tag((*io2).get_tag());
            (*to).top.p = ((*to).top.p).offset(1);
            (*to).top.p;
            i += 1;
        }
    }
}
pub unsafe extern "C" fn lua_atpanic(state: *mut State, panicf: CFunction) -> CFunction {
    unsafe {
        let old: CFunction = (*(*state).global).panic;
        (*(*state).global).panic = panicf;
        return old;
    }
}
pub unsafe extern "C" fn lua_absindex(state: *mut State, index: i32) -> i32 {
    unsafe {
        return if index > 0 || index <= -(1000000 as i32) - 1000 as i32 {
            index
        } else {
            ((*state).top.p).offset_from((*(*state).call_info).function.p) as i32 + index
        };
    }
}
pub unsafe extern "C" fn lua_settop(state: *mut State, index: i32) {
    unsafe {
        let call_info;
        let mut newtop;
        let mut diff;
        call_info = (*state).call_info;
        let function: StackValuePointer = (*call_info).function.p;
        if index >= 0 {
            diff = function
                .offset(1 as isize)
                .offset(index as isize)
                .offset_from((*state).top.p) as i64;
            while diff > 0 {
                let fresh4 = (*state).top.p;
                (*state).top.p = (*state).top.p.offset(1);
                (*fresh4).tvalue.set_tag(TAG_VARIANT_NIL_NIL);
                diff -= 1;
            }
        } else {
            diff = (index + 1) as i64;
        }
        newtop = (*state).top.p.offset(diff as isize);
        if diff < 0 && (*state).tbc_list.p >= newtop {
            newtop = luaf_close(state, newtop, -1, 0);
        }
        (*state).top.p = newtop;
    }
}
pub unsafe extern "C" fn lua_closeslot(state: *mut State, index: i32) {
    unsafe {
        let mut level = index2stack(state, index);
        level = luaf_close(state, level, -1, 0);
        (*level).tvalue.set_tag(TAG_VARIANT_NIL_NIL);
    }
}
pub unsafe extern "C" fn reverse(mut _state: *mut State, mut from: StackValuePointer, mut to: StackValuePointer) {
    unsafe {
        while from < to {
            let mut temp: TValue = TValue {
                value: Value {
                    object: std::ptr::null_mut(),
                },
                tag: 0,
            };
            let io1: *mut TValue = &mut temp;
            let io2: *const TValue = &mut (*from).tvalue;
            (*io1).value = (*io2).value;
            (*io1).set_tag((*io2).get_tag());
            let io1_0: *mut TValue = &mut (*from).tvalue;
            let io2_0: *const TValue = &mut (*to).tvalue;
            (*io1_0).value = (*io2_0).value;
            (*io1_0).set_tag((*io2_0).get_tag());
            let io1_1: *mut TValue = &mut (*to).tvalue;
            let io2_1: *const TValue = &mut temp;
            (*io1_1).value = (*io2_1).value;
            (*io1_1).set_tag((*io2_1).get_tag());
            from = from.offset(1);
            to = to.offset(-1);
        }
    }
}
pub unsafe extern "C" fn lua_rotate(state: *mut State, index: i32, n: i32) {
    unsafe {
        let high: StackValuePointer = (*state).top.p.offset(-(1 as isize));
        let low: StackValuePointer = index2stack(state, index);
        let middle: StackValuePointer = if n >= 0 {
            high.offset(-(n as isize))
        } else {
            low.offset(-(n as isize)).offset(-(1 as isize))
        };
        reverse(state, low, middle);
        reverse(state, middle.offset(1 as isize), high);
        reverse(state, low, high);
    }
}
pub unsafe extern "C" fn lua_copy(state: *mut State, fromidx: i32, toidx: i32) {
    unsafe {
        let fr: *mut TValue = (*state).index2value(fromidx);
        let to: *mut TValue = (*state).index2value(toidx);
        let io1: *mut TValue = to;
        let io2: *const TValue = fr;
        (*io1).value = (*io2).value;
        (*io1).set_tag((*io2).get_tag());
        if toidx < -(1000000 as i32) - 1000 as i32 {
            if (*fr).is_collectable() {
                if (*((*(*(*state).call_info).function.p).tvalue.value.object as *mut Closure))
                    .get_marked()
                    & 1 << 5
                    != 0
                    && (*(*fr).value.object).get_marked() & (1 << 3 | 1 << 4) != 0
                {
                    luac_barrier_(
                        state,
                        &mut (*(&mut (*((*(*(*state).call_info).function.p).tvalue.value.object)))),
                        &mut (*((*fr).value.object as *mut Object)),
                    );
                } else {
                };
            } else {
            };
        }
    }
}
pub unsafe extern "C" fn lua_pushvalue(state: *mut State, index: i32) {
    unsafe {
        let io1: *mut TValue = &mut (*(*state).top.p).tvalue;
        let io2: *const TValue = (*state).index2value(index);
        (*io1).value = (*io2).value;
        (*io1).set_tag((*io2).get_tag());
        (*state).top.p = (*state).top.p.offset(1);
    }
}
pub unsafe fn lua_type(state: *mut State, index: i32) -> Option<u8> {
    unsafe {
        let o: *const TValue = (*state).index2value(index);
        return if (get_tag_type((*o).get_tag()) != TAG_TYPE_NIL)
            || o != &mut (*(*state).global).nil_value as *mut TValue as *const TValue
        {
            return Some((*o).get_tag_type())
        } else {
            None
        };
    }
}
pub unsafe fn lua_typename(mut _state: *mut State, t: Option<u8>) -> *const i8 {
    match t {
        None => b"no value\0" as *const u8 as *const i8,
        Some(TAG_TYPE_NIL) => b"nil\0" as *const u8 as *const i8,
        Some(TAG_TYPE_BOOLEAN) => b"boolean\0" as *const u8 as *const i8,
        Some(TAG_TYPE_POINTER) => b"userdata\0" as *const u8 as *const i8,
        Some(TAG_TYPE_NUMERIC) => b"number\0" as *const u8 as *const i8,
        Some(TAG_TYPE_STRING) => b"string\0" as *const u8 as *const i8,
        Some(TAG_TYPE_TABLE) => b"table\0" as *const u8 as *const i8,
        Some(TAG_TYPE_CLOSURE) => b"function\0" as *const u8 as *const i8,
        Some(TAG_TYPE_USER) => b"userdata\0" as *const u8 as *const i8,
        Some(TAG_TYPE_STATE) => b"thread\0" as *const u8 as *const i8,
        Some(TAG_TYPE_UPVALUE) => b"upvalue\0" as *const u8 as *const i8,
        Some(TAG_TYPE_PROTOTYPE) => b"proto\0" as *const u8 as *const i8,
        _ => b"unknown\0" as *const u8 as *const i8,
    }
}
pub unsafe extern "C" fn lua_iscfunction(state: *mut State, index: i32) -> bool {
    unsafe {
        let o: *const TValue = (*state).index2value(index);
        match (*o).get_tag_variant() {
            TAG_VARIANT_CLOSURE_CFUNCTION => return true,
            TAG_VARIANT_CLOSURE_C => return true,
            _ => return false,
        }
    }
}
pub unsafe extern "C" fn lua_isinteger(state: *mut State, index: i32) -> bool {
    unsafe {
        return (*(*state).index2value(index)).get_tag() == TAG_VARIANT_NUMERIC_INTEGER;
    }
}
pub unsafe extern "C" fn lua_isnumber(state: *mut State, index: i32) -> bool {
    unsafe {
        let o: *const TValue = (*state).index2value(index);
        return if (*o).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
            true
        } else {
            let mut n: f64 = 0.0;
            luav_tonumber_(o, &mut n)
        };
    }
}
pub unsafe extern "C" fn lua_isstring(state: *mut State, index: i32) -> bool {
    unsafe {
        let o: *const TValue = (*state).index2value(index);
        return match get_tag_type((*o).get_tag()) {
            TAG_TYPE_NUMERIC => true,
            TAG_TYPE_STRING => true,
            _ => false,
        };
    }
}
pub unsafe extern "C" fn lua_rawequal(state: *mut State, index1: i32, index2: i32) -> bool {
    unsafe {
        let o1: *const TValue = (*state).index2value(index1);
        let o2: *const TValue = (*state).index2value(index2);
        return if (!(get_tag_type((*o1).get_tag()) == TAG_TYPE_NIL)
            || o1 != &mut (*(*state).global).nil_value as *mut TValue as *const TValue)
            && (!(get_tag_type((*o2).get_tag()) == TAG_TYPE_NIL)
                || o2 != &mut (*(*state).global).nil_value as *mut TValue as *const TValue)
        {
            luav_equalobj(std::ptr::null_mut(), o1, o2)
        } else {
            false
        };
    }
}
pub unsafe extern "C" fn lua_arith(state: *mut State, op: i32) {
    unsafe {
        if !(op != 12 as i32 && op != 13 as i32) {
            let io1: *mut TValue = &mut (*(*state).top.p).tvalue;
            let io2: *const TValue = &mut (*(*state).top.p.offset(-(1 as isize))).tvalue;
            (*io1).value = (*io2).value;
            (*io1).set_tag((*io2).get_tag());
            (*state).top.p = (*state).top.p.offset(1);
        }
        luao_arith(
            state,
            op,
            &mut (*(*state).top.p.offset(-(2 as isize))).tvalue,
            &mut (*(*state).top.p.offset(-(1 as isize))).tvalue,
            (*state).top.p.offset(-(2 as isize)),
        );
        (*state).top.p = (*state).top.p.offset(-1);
    }
}
pub unsafe extern "C" fn lua_compare(state: *mut State, index1: i32, index2: i32, op: i32) -> i32 {
    unsafe {
        let o1: *const TValue = (*state).index2value(index1);
        let o2: *const TValue = (*state).index2value(index2);
        let mut i: i32 = 0;
        if (!(get_tag_type((*o1).get_tag()) == TAG_TYPE_NIL)
            || o1 != &mut (*(*state).global).nil_value as *mut TValue as *const TValue)
            && (!(get_tag_type((*o2).get_tag()) == TAG_TYPE_NIL)
                || o2 != &mut (*(*state).global).nil_value as *mut TValue as *const TValue)
        {
            match op {
                0 => {
                    i = if luav_equalobj(state, o1, o2) { 1 } else { 0 };
                }
                1 => {
                    i = if luav_lessthan(state, o1, o2) { 1 } else { 0 };
                }
                2 => {
                    i = if luav_lessequal(state, o1, o2) { 1 } else { 0 };
                }
                _ => {}
            }
        }
        return i;
    }
}
pub unsafe extern "C" fn lua_stringtonumber(state: *mut State, s: *const i8) -> u64 {
    unsafe {
        let size: u64 = luao_str2num(s, &mut (*(*state).top.p).tvalue);
        if size != 0u64 {
            (*state).top.p = (*state).top.p.offset(1);
        }
        return size;
    }
}
pub unsafe extern "C" fn lua_tonumberx(state: *mut State, index: i32, is_number: *mut bool) -> f64 {
    unsafe {
        let mut n: f64 = 0.0;
        let o: *const TValue = (*state).index2value(index);
        let is_number_: bool = if (*o).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
            n = (*o).value.number;
            true
        } else {
            luav_tonumber_(o, &mut n)
        };
        if !is_number.is_null() {
            *is_number = is_number_;
        }
        return n;
    }
}
pub unsafe extern "C" fn lua_tointegerx(
    state: *mut State,
    index: i32,
    is_number: *mut bool,
) -> i64 {
    unsafe {
        let mut res: i64 = 0;
        let o: *const TValue = (*state).index2value(index);
        let is_number_: bool =
            if (((*o).get_tag() == TAG_VARIANT_NUMERIC_INTEGER) as i32 != 0) as i64 != 0 {
                res = (*o).value.integer;
                true
            } else {
                luav_tointeger(o, &mut res, F2I::Equal) != 0
            };
        if !is_number.is_null() {
            *is_number = is_number_;
        }
        return res;
    }
}
pub unsafe extern "C" fn lua_toboolean(state: *mut State, index: i32) -> i32 {
    unsafe {
        let o: *const TValue = (*state).index2value(index);
        return !((*o).get_tag() == TAG_VARIANT_BOOLEAN_FALSE
            || get_tag_type((*o).get_tag()) == TAG_TYPE_NIL) as i32;
    }
}
pub unsafe extern "C" fn lua_tolstring(
    state: *mut State,
    index: i32,
    length: *mut u64,
) -> *const i8 {
    unsafe {
        let mut o: *mut TValue = (*state).index2value(index);
        if !(get_tag_type((*o).get_tag()) == TAG_TYPE_STRING) {
            if !(get_tag_type((*o).get_tag()) == TAG_TYPE_NUMERIC) {
                if !length.is_null() {
                    *length = 0;
                }
                return std::ptr::null();
            }
            luao_tostring(state, o);
            if (*(*state).global).gc_debt > 0 {
                luac_step(state);
            }
            o = (*state).index2value(index);
        }
        if !length.is_null() {
            *length = (*((*o).value.object as *mut TString)).get_length();
        }
        return (*((*o).value.object as *mut TString)).get_contents();
    }
}
pub unsafe extern "C" fn lua_rawlen(state: *mut State, index: i32) -> u64 {
    unsafe {
        let o: *const TValue = (*state).index2value(index);
        match (*o).get_tag_variant() {
            TAG_VARIANT_STRING_SHORT | TAG_VARIANT_STRING_LONG => {
                return (*((*o).value.object as *mut TString)).get_length();
            },
            TAG_VARIANT_USER => return (*((*o).value.object as *mut User)).length as u64,
            TAG_VARIANT_TABLE => return luah_getn(&mut (*((*o).value.object as *mut Table))),
            _ => return 0,
        };
    }
}
pub unsafe extern "C" fn lua_touserdata(state: *mut State, index: i32) -> *mut libc::c_void {
    unsafe {
        let o: *const TValue = (*state).index2value(index);
        return User::touserdata(o);
    }
}
pub unsafe extern "C" fn lua_tothread(state: *mut State, index: i32) -> *mut State {
    unsafe {
        let o: *const TValue = (*state).index2value(index);
        return if !((*o).get_tag_variant() == TAG_VARIANT_STATE) {
            std::ptr::null_mut()
        } else {
            &mut (*((*o).value.object as *mut State))
        };
    }
}
pub unsafe extern "C" fn lua_pushlstring(
    state: *mut State,
    s: *const i8,
    length: u64,
) -> *const i8 {
    unsafe {
        let ts: *mut TString = if length == 0u64 {
            luas_new(state, b"\0" as *const u8 as *const i8)
        } else {
            luas_newlstr(state, s, length)
        };
        let io: *mut TValue = &mut (*(*state).top.p).tvalue;
        let x_: *mut TString = ts;
        (*io).value.object = &mut (*(x_ as *mut Object));
        (*io).set_tag((*x_).get_tag());
        (*io).set_collectable();
        (*state).top.p = (*state).top.p.offset(1);
        if (*(*state).global).gc_debt > 0 {
            luac_step(state);
        }
        return (*ts).get_contents();
    }
}
pub unsafe extern "C" fn lua_pushstring(state: *mut State, mut s: *const i8) -> *const i8 {
    unsafe {
        if s.is_null() {
            (*(*state).top.p).tvalue.set_tag(TAG_VARIANT_NIL_NIL);
        } else {
            let ts: *mut TString = luas_new(state, s);
            let io: *mut TValue = &mut (*(*state).top.p).tvalue;
            let x_: *mut TString = ts;
            (*io).value.object = &mut (*(x_ as *mut Object));
            (*io).set_tag((*x_).get_tag());
            (*io).set_collectable();
            s = (*ts).get_contents();
        }
        (*state).top.p = (*state).top.p.offset(1);
        if (*(*state).global).gc_debt > 0 {
            luac_step(state);
        }
        return s;
    }
}
pub unsafe extern "C" fn lua_pushvfstring(
    state: *mut State,
    fmt: *const i8,
    mut argp: ::core::ffi::VaList,
) -> *const i8 {
    unsafe {
        let ret: *const i8 = luao_pushvfstring(state, fmt, argp.as_va_list());
        if (*(*state).global).gc_debt > 0 {
            luac_step(state);
        }
        return ret;
    }
}
pub unsafe extern "C" fn lua_pushfstring(
    state: *mut State,
    fmt: *const i8,
    args: ...
) -> *const i8 {
    unsafe {
        let mut argp: ::core::ffi::VaListImpl;
        argp = args.clone();
        let ret: *const i8 = luao_pushvfstring(state, fmt, argp.as_va_list());
        if (*(*state).global).gc_debt > 0 {
            luac_step(state);
        }
        return ret;
    }
}
pub unsafe extern "C" fn lua_pushcclosure(state: *mut State, fn_0: CFunction, mut n: i32) {
    unsafe {
        if n == 0 {
            let io: *mut TValue = &mut (*(*state).top.p).tvalue;
            (*io).value.function = fn_0;
            (*io).set_tag(TAG_VARIANT_CLOSURE_CFUNCTION);
            (*state).top.p = (*state).top.p.offset(1);
        } else {
            let cl: *mut Closure = luaf_newcclosure(state, n);
            (*cl).payload.c_cfunction = fn_0;
            (*state).top.p = (*state).top.p.offset(-(n as isize));
            loop {
                let fresh5 = n;
                n = n - 1;
                if !(fresh5 != 0) {
                    break;
                }
                let io1: *mut TValue =
                    &mut *((*cl).upvalues).c_tvalues.as_mut_ptr().offset(n as isize) as *mut TValue;
                let io2: *const TValue = &mut (*(*state).top.p.offset(n as isize)).tvalue;
                (*io1).value = (*io2).value;
                (*io1).set_tag((*io2).get_tag());
            }
            let io_0: *mut TValue = &mut (*(*state).top.p).tvalue;
            let x_: *mut Closure = cl;
            (*io_0).value.object = &mut (*(x_ as *mut Object));
            (*io_0).set_tag(TAG_VARIANT_CLOSURE_C);
            (*io_0).set_collectable();
            (*state).top.p = (*state).top.p.offset(1);
            if (*(*state).global).gc_debt > 0 {
                luac_step(state);
            }
        };
    }
}
pub unsafe extern "C" fn lua_pushlightuserdata(state: *mut State, p: *mut libc::c_void) {
    unsafe {
        let io: *mut TValue = &mut (*(*state).top.p).tvalue;
        (*io).value.pointer = p;
        (*io).set_tag(TAG_TYPE_POINTER);
        (*state).top.p = (*state).top.p.offset(1);
    }
}
pub unsafe extern "C" fn auxgetstr(state: *mut State, t: *const TValue, k: *const i8) -> i32 {
    unsafe {
        let slot: *const TValue;
        let str: *mut TString = luas_new(state, k);
        if if !((*t).get_tag_variant() == TAG_VARIANT_TABLE) {
            slot = std::ptr::null();
            0
        } else {
            slot = luah_getstr(&mut (*((*t).value.object as *mut Table)), str);
            (get_tag_type((*slot).get_tag()) != TAG_TYPE_NIL) as i32
        } != 0
        {
            let io1: *mut TValue = &mut (*(*state).top.p).tvalue;
            let io2: *const TValue = slot;
            (*io1).value = (*io2).value;
            (*io1).set_tag((*io2).get_tag());
            (*state).top.p = (*state).top.p.offset(1);
        } else {
            let io: *mut TValue = &mut (*(*state).top.p).tvalue;
            let x_: *mut TString = str;
            (*io).value.object = &mut (*(x_ as *mut Object));
            (*io).set_tag((*x_).get_tag());
            (*io).set_collectable();
            (*state).top.p = (*state).top.p.offset(1);
            luav_finishget(
                state,
                t,
                &mut (*(*state).top.p.offset(-(1 as isize))).tvalue,
                (*state).top.p.offset(-(1 as isize)),
                slot,
            );
        }
        return (get_tag_type((*(*state).top.p.offset(-(1 as isize))).tvalue.get_tag())) as i32;
    }
}
pub unsafe extern "C" fn lua_getglobal(state: *mut State, name: *const i8) -> i32 {
    unsafe {
        let global_table: *const TValue = &mut *((*((*(*state).global).l_registry.value.object
            as *mut Table))
            .array)
            .offset((2 - 1) as isize) as *mut TValue;
        return auxgetstr(state, global_table, name);
    }
}
pub unsafe extern "C" fn lua_gettable(state: *mut State, index: i32) -> i32 {
    unsafe {
        let slot;
        let t: *mut TValue = (*state).index2value(index);
        if if (*t).get_tag_variant() != TAG_VARIANT_TABLE {
            slot = std::ptr::null();
            0
        } else {
            slot = luah_get(
                &mut (*((*t).value.object as *mut Table)),
                &mut (*(*state).top.p.offset(-(1 as isize))).tvalue,
            );
            (get_tag_type((*slot).get_tag()) != TAG_TYPE_NIL) as i32
        } != 0
        {
            let io1: *mut TValue = &mut (*(*state).top.p.offset(-(1 as isize))).tvalue;
            let io2: *const TValue = slot;
            (*io1).value = (*io2).value;
            (*io1).set_tag((*io2).get_tag());
        } else {
            luav_finishget(
                state,
                t,
                &mut (*(*state).top.p.offset(-(1 as isize))).tvalue,
                (*state).top.p.offset(-(1 as isize)),
                slot,
            );
        }
        return (get_tag_type((*(*state).top.p.offset(-(1 as isize))).tvalue.get_tag())) as i32;
    }
}
pub unsafe extern "C" fn handle_luainit(state: *mut State) -> i32 {
    unsafe {
        let mut name: *const i8 = b"=LUA_INIT_5_4\0" as *const u8 as *const i8;
        let mut init: *const i8 = getenv(name.offset(1 as isize));
        if init.is_null() {
            name = b"=LUA_INIT\0" as *const u8 as *const i8;
            init = getenv(name.offset(1 as isize));
        }
        if init.is_null() {
            return 0;
        } else if *init.offset(0 as isize) as i32 == '@' as i32 {
            return dofile(state, init.offset(1 as isize));
        } else {
            return dostring(state, init, name);
        };
    }
}
pub unsafe extern "C" fn lua_getfield(state: *mut State, index: i32, k: *const i8) -> i32 {
    unsafe {
        return auxgetstr(state, (*state).index2value(index), k);
    }
}
pub unsafe extern "C" fn lua_geti(state: *mut State, index: i32, n: i64) -> i32 {
    unsafe {
        let t: *mut TValue;
        let slot: *const TValue;
        t = (*state).index2value(index);
        if if (*t).get_tag_variant() != TAG_VARIANT_TABLE {
            slot = std::ptr::null();
            0
        } else {
            slot = if (n as u64).wrapping_sub(1 as u64)
                < (*((*t).value.object as *mut Table)).array_limit as u64
            {
                &mut *((*((*t).value.object as *mut Table)).array).offset((n - 1) as isize)
                    as *mut TValue as *const TValue
            } else {
                luah_getint(&mut (*((*t).value.object as *mut Table)), n)
            };
            (get_tag_type((*slot).get_tag()) != TAG_TYPE_NIL) as i32
        } != 0
        {
            let io1: *mut TValue = &mut (*(*state).top.p).tvalue;
            let io2: *const TValue = slot;
            (*io1).value = (*io2).value;
            (*io1).set_tag((*io2).get_tag());
        } else {
            let mut aux: TValue = TValue {
                value: Value {
                    object: std::ptr::null_mut(),
                },
                tag: 0,
            };
            let io: *mut TValue = &mut aux;
            (*io).value.integer = n;
            (*io).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
            luav_finishget(state, t, &mut aux, (*state).top.p, slot);
        }
        (*state).top.p = (*state).top.p.offset(1);
        return (get_tag_type((*(*state).top.p.offset(-(1 as isize))).tvalue.get_tag())) as i32;
    }
}
pub unsafe extern "C" fn finishrawget(state: *mut State, value: *const TValue) -> i32 {
    unsafe {
        if get_tag_type((*value).get_tag()) == TAG_TYPE_NIL {
            (*(*state).top.p).tvalue.set_tag(TAG_VARIANT_NIL_NIL);
        } else {
            let io1: *mut TValue = &mut (*(*state).top.p).tvalue;
            let io2: *const TValue = value;
            (*io1).value = (*io2).value;
            (*io1).set_tag((*io2).get_tag());
        }
        (*state).top.p = (*state).top.p.offset(1);
        return (get_tag_type((*(*state).top.p.offset(-(1 as isize))).tvalue.get_tag())) as i32;
    }
}
pub unsafe extern "C" fn gettable(state: *mut State, index: i32) -> *mut Table {
    unsafe {
        let t: *mut TValue = (*state).index2value(index);
        return &mut (*((*t).value.object as *mut Table));
    }
}
pub unsafe extern "C" fn lua_rawget(state: *mut State, index: i32) -> i32 {
    unsafe {
        let table: *mut Table = gettable(state, index);
        let value: *const TValue =
            luah_get(table, &mut (*(*state).top.p.offset(-(1 as isize))).tvalue);
        (*state).top.p = (*state).top.p.offset(-1);
        return finishrawget(state, value);
    }
}
pub unsafe extern "C" fn lua_rawgeti(state: *mut State, index: i32, n: i64) -> i32 {
    unsafe {
        let table: *mut Table = gettable(state, index);
        return finishrawget(state, luah_getint(table, n));
    }
}
pub unsafe extern "C" fn auxsetstr(state: *mut State, t: *const TValue, k: *const i8) {
    unsafe {
        let slot: *const TValue;
        let str: *mut TString = luas_new(state, k);
        if if !((*t).get_tag_variant() == TAG_VARIANT_TABLE) {
            slot = std::ptr::null();
            0
        } else {
            slot = luah_getstr(&mut (*((*t).value.object as *mut Table)), str);
            (get_tag_type((*slot).get_tag()) != TAG_TYPE_NIL) as i32
        } != 0
        {
            let io1: *mut TValue = slot as *mut TValue;
            let io2: *const TValue = &mut (*(*state).top.p.offset(-(1 as isize))).tvalue;
            (*io1).value = (*io2).value;
            (*io1).set_tag((*io2).get_tag());
            if (*(*state).top.p.offset(-(1 as isize)))
                .tvalue
                .is_collectable()
            {
                if (*(*t).value.object).get_marked() & 1 << 5 != 0
                    && (*(*(*state).top.p.offset(-(1 as isize))).tvalue.value.object).get_marked()
                        & (1 << 3 | 1 << 4)
                        != 0
                {
                    luac_barrierback_(state, (*t).value.object);
                } else {
                };
            } else {
            };
            (*state).top.p = (*state).top.p.offset(-1);
        } else {
            let io: *mut TValue = &mut (*(*state).top.p).tvalue;
            let x_: *mut TString = str;
            (*io).value.object = &mut (*(x_ as *mut Object));
            (*io).set_tag((*x_).get_tag());
            (*io).set_collectable();
            (*state).top.p = (*state).top.p.offset(1);
            luav_finishset(
                state,
                t,
                &mut (*(*state).top.p.offset(-(1 as isize))).tvalue,
                &mut (*(*state).top.p.offset(-(2 as isize))).tvalue,
                slot,
            );
            (*state).top.p = (*state).top.p.offset(-(2 as isize));
        };
    }
}
pub unsafe extern "C" fn lua_setglobal(state: *mut State, name: *const i8) {
    unsafe {
        let global_table: *const TValue = &mut *((*((*(*state).global).l_registry.value.object
            as *mut Table))
            .array)
            .offset((2 - 1) as isize) as *mut TValue;
        auxsetstr(state, global_table, name);
    }
}
pub unsafe extern "C" fn lua_setfield(state: *mut State, index: i32, k: *const i8) {
    unsafe {
        auxsetstr(state, (*state).index2value(index), k);
    }
}
pub unsafe extern "C" fn lua_seti(state: *mut State, index: i32, n: i64) {
    unsafe {
        let t: *mut TValue;
        let slot: *const TValue;
        t = (*state).index2value(index);
        if if !((*t).get_tag_variant() == TAG_VARIANT_TABLE) {
            slot = std::ptr::null();
            0
        } else {
            slot = if (n as u64).wrapping_sub(1 as u64)
                < (*((*t).value.object as *mut Table)).array_limit as u64
            {
                &mut *((*((*t).value.object as *mut Table)).array).offset((n - 1) as isize)
                    as *mut TValue as *const TValue
            } else {
                luah_getint(&mut (*((*t).value.object as *mut Table)), n)
            };
            (get_tag_type((*slot).get_tag()) != TAG_TYPE_NIL) as i32
        } != 0
        {
            let io1: *mut TValue = slot as *mut TValue;
            let io2: *const TValue = &mut (*(*state).top.p.offset(-(1 as isize))).tvalue;
            (*io1).value = (*io2).value;
            (*io1).set_tag((*io2).get_tag());
            if (*(*state).top.p.offset(-(1 as isize)))
                .tvalue
                .is_collectable()
            {
                if (*(*t).value.object).get_marked() & 1 << 5 != 0
                    && (*(*(*state).top.p.offset(-(1 as isize))).tvalue.value.object).get_marked()
                        & (1 << 3 | 1 << 4)
                        != 0
                {
                    luac_barrierback_(state, (*t).value.object);
                } else {
                };
            } else {
            };
        } else {
            let mut aux: TValue = TValue::new();
            aux.value.integer = n;
            aux.set_tag(TAG_VARIANT_NUMERIC_INTEGER);
            luav_finishset(
                state,
                t,
                &mut aux,
                &mut (*(*state).top.p.offset(-(1 as isize))).tvalue,
                slot,
            );
        }
        (*state).top.p = (*state).top.p.offset(-1);
    }
}
pub unsafe extern "C" fn aux_rawset(state: *mut State, index: i32, key: *mut TValue, n: i32) {
    unsafe {
        let table: *mut Table = gettable(state, index);
        luah_set(
            state,
            table,
            key,
            &mut (*(*state).top.p.offset(-(1 as isize))).tvalue,
        );
        (*table).flags = ((*table).flags as u32 & !!(!0 << TM_EQ as i32 + 1)) as u8;
        if (*(*state).top.p.offset(-(1 as isize)))
            .tvalue
            .is_collectable()
        {
            if (*(table as *mut Object)).get_marked() & 1 << 5 != 0
                && (*(*(*state).top.p.offset(-(1 as isize))).tvalue.value.object).get_marked()
                    & (1 << 3 | 1 << 4)
                    != 0
            {
                luac_barrierback_(state, &mut (*(table as *mut Object)));
            } else {
            };
        } else {
        };
        (*state).top.p = (*state).top.p.offset(-(n as isize));
    }
}
pub unsafe extern "C" fn lua_rawset(state: *mut State, index: i32) {
    unsafe {
        aux_rawset(
            state,
            index,
            &mut (*(*state).top.p.offset(-(2 as isize))).tvalue,
            2,
        );
    }
}
pub unsafe extern "C" fn lua_rawseti(state: *mut State, index: i32, n: i64) {
    unsafe {
        let table: *mut Table = gettable(state, index);
        luah_setint(
            state,
            table,
            n,
            &mut (*(*state).top.p.offset(-(1 as isize))).tvalue,
        );
        if (*(*state).top.p.offset(-(1 as isize)))
            .tvalue
            .is_collectable()
        {
            if (*(table as *mut Object)).get_marked() & 1 << 5 != 0
                && (*(*(*state).top.p.offset(-(1 as isize))).tvalue.value.object).get_marked()
                    & (1 << 3 | 1 << 4)
                    != 0
            {
                luac_barrierback_(state, &mut (*(table as *mut Object)));
            } else {
            };
        } else {
        };
        (*state).top.p = (*state).top.p.offset(-1);
    }
}
pub unsafe extern "C" fn lua_setmetatable(state: *mut State, objindex: i32) -> i32 {
    unsafe {
        let mt: *mut Table;
        let obj: *mut TValue = (*state).index2value(objindex);
        if get_tag_type((*(*state).top.p.offset(-(1 as isize))).tvalue.get_tag()) == TAG_TYPE_NIL {
            mt = std::ptr::null_mut();
        } else {
            mt = &mut (*((*(*state).top.p.offset(-(1 as isize))).tvalue.value.object
                as *mut Table))
        }
        match get_tag_type((*obj).get_tag()) {
            5 => {
                let ref mut fresh6 = (*((*obj).value.object as *mut Table)).metatable;
                *fresh6 = mt;
                if !mt.is_null() {
                    if (*(*obj).value.object).get_marked() & 1 << 5 != 0
                        && (*mt).get_marked() & (1 << 3 | 1 << 4) != 0
                    {
                        luac_barrier_(
                            state,
                            &mut (*((*obj).value.object as *mut Object)),
                            &mut (*(mt as *mut Object)),
                        );
                    } else {
                    };
                    luac_checkfinalizer(state, (*obj).value.object, mt);
                }
            }
            7 => {
                let ref mut fresh7 = (*((*obj).value.object as *mut User)).metatable;
                *fresh7 = mt;
                if !mt.is_null() {
                    if (*((*obj).value.object as *mut User)).get_marked() & 1 << 5 != 0
                        && (*mt).get_marked() & (1 << 3 | 1 << 4) != 0
                    {
                        luac_barrier_(
                            state,
                            &mut (*(&mut (*((*obj).value.object as *mut User)) as *mut User
                                as *mut Object)),
                            &mut (*(mt as *mut Object)),
                        );
                    } else {
                    };
                    luac_checkfinalizer(state, (*obj).value.object, mt);
                }
            }
            _ => {
                (*(*state).global).metatable[(get_tag_type((*obj).get_tag())) as usize] = mt;
            }
        }
        (*state).top.p = (*state).top.p.offset(-1);
        return 1;
    }
}
pub unsafe extern "C" fn lua_setiuservalue(state: *mut State, index: i32, n: i32) -> i32 {
    unsafe {
        let res: i32;
        let o: *mut TValue = (*state).index2value(index);
        if !((n as u32).wrapping_sub(1 as u32)
            < (*((*o).value.object as *mut User)).nuvalue as u32)
        {
            res = 0;
        } else {
            let io1: *mut TValue = &mut (*((*((*o).value.object as *mut User)).uv)
                .as_mut_ptr()
                .offset((n - 1) as isize));
            let io2: *const TValue = &mut (*(*state).top.p.offset(-(1 as isize))).tvalue;
            (*io1).value = (*io2).value;
            (*io1).set_tag((*io2).get_tag());
            if (*(*state).top.p.offset(-(1 as isize)))
                .tvalue
                .is_collectable()
            {
                if (*(*o).value.object).get_marked() & 1 << 5 != 0
                    && (*(*(*state).top.p.offset(-(1 as isize))).tvalue.value.object).get_marked()
                        & (1 << 3 | 1 << 4)
                        != 0
                {
                    luac_barrierback_(state, (*o).value.object);
                } else {
                };
            } else {
            };
            res = 1;
        }
        (*state).top.p = (*state).top.p.offset(-1);
        return res;
    }
}
pub unsafe extern "C" fn lua_callk(
    state: *mut State,
    nargs: i32,
    count_results: i32,
    ctx: i64,
    k: ContextFunction,
) {
    unsafe {
        let function: StackValuePointer = (*state).top.p.offset(-((nargs + 1) as isize));
        if k.is_some() && (*state).count_c_calls & 0xffff0000 as u32 == 0 {
            (*(*state).call_info).u.c.context_function = k;
            (*(*state).call_info).u.c.context = ctx;
            ccall(state, function, count_results, 1);
        } else {
            luad_callnoyield(state, function, count_results);
        }
        if count_results <= -1 && (*(*state).call_info).top.p < (*state).top.p {
            (*(*state).call_info).top.p = (*state).top.p;
        }
    }
}
pub unsafe extern "C" fn f_call(state: *mut State, arbitrary_data: *mut libc::c_void) {
    unsafe {
        let calls: *mut CallS = arbitrary_data as *mut CallS;
        luad_callnoyield(state, (*calls).function, (*calls).count_results);
    }
}
pub unsafe extern "C" fn lua_pcallk(
    state: *mut State,
    nargs: i32,
    count_results: i32,
    error_function: i32,
    ctx: i64,
    k: ContextFunction,
) -> i32 {
    unsafe {
        let mut calls: CallS = CallS {
            function: std::ptr::null_mut(),
            count_results: 0,
        };
        let status: i32;
        let function: i64;
        if error_function == 0 {
            function = 0;
        } else {
            let o: StackValuePointer = index2stack(state, error_function);
            function = (o as *mut i8).offset_from((*state).stack.p as *mut i8) as i64;
        }
        calls.function = (*state).top.p.offset(-((nargs + 1) as isize));
        if k.is_none() || !((*state).count_c_calls & 0xffff0000 as u32 == 0) {
            calls.count_results = count_results;
            status = luad_pcall(
                state,
                Some(f_call as unsafe extern "C" fn(*mut State, *mut libc::c_void) -> ()),
                &mut calls as *mut CallS as *mut libc::c_void,
                (calls.function as *mut i8).offset_from((*state).stack.p as *mut i8) as i64,
                function,
            );
        } else {
            let call_info: *mut CallInfo = (*state).call_info;
            (*call_info).u.c.context_function = k;
            (*call_info).u.c.context = ctx;
            (*call_info).u2.funcidx =
                (calls.function as *mut i8).offset_from((*state).stack.p as *mut i8) as i32;
            (*call_info).u.c.old_error_function = (*state).error_function;
            (*state).error_function = function;
            (*call_info).call_status =
                ((*call_info).call_status as i32 & !(1 << 0) | (*state).allow_hook as i32) as u16;
            (*call_info).call_status = ((*call_info).call_status as i32 | 1 << 4) as u16;
            ccall(state, calls.function, count_results, 1);
            (*call_info).call_status = ((*call_info).call_status as i32 & !(1 << 4)) as u16;
            (*state).error_function = (*call_info).u.c.old_error_function;
            status = 0;
        }
        if count_results <= -1 && (*(*state).call_info).top.p < (*state).top.p {
            (*(*state).call_info).top.p = (*state).top.p;
        }
        return status;
    }
}
pub unsafe extern "C" fn lua_load(
    state: *mut State,
    reader: ReadFunction,
    data: *mut libc::c_void,
    mut chunkname: *const i8,
    mode: *const i8,
) -> i32 {
    unsafe {
        let mut zio: ZIO = ZIO {
            n: 0,
            p: std::ptr::null(),
            reader: None,
            data: std::ptr::null_mut(),
            state: std::ptr::null_mut(),
        };
        let status: i32;
        if chunkname.is_null() {
            chunkname = b"?\0" as *const u8 as *const i8;
        }
        luaz_init(state, &mut zio, reader, data);
        status = luad_protectedparser(state, &mut zio, chunkname, mode);
        if status == 0 {
            let f: *mut Closure =
                &mut (*((*(*state).top.p.offset(-(1 as isize))).tvalue.value.object
                    as *mut Closure));
            if (*f).count_upvalues as i32 >= 1 {
                let gt: *const TValue =
                    &mut *((*((*(*state).global).l_registry.value.object as *mut Table))
                        .array)
                        .offset((2 - 1) as isize) as *mut TValue;
                let io1: *mut TValue = (**((*f).upvalues).l_upvalues.as_mut_ptr().offset(0 as isize)).v.p;
                let io2: *const TValue = gt;
                (*io1).value = (*io2).value;
                (*io1).set_tag((*io2).get_tag());
                if (*gt).is_collectable() {
                    if (**((*f).upvalues).l_upvalues.as_mut_ptr().offset(0 as isize)).get_marked() & 1 << 5
                        != 0
                        && (*(*gt).value.object).get_marked() & (1 << 3 | 1 << 4) != 0
                    {
                        luac_barrier_(
                            state,
                            &mut (*(*((*f).upvalues).l_upvalues.as_mut_ptr().offset(0 as isize)
                                as *mut Object)),
                            &mut (*((*gt).value.object as *mut Object)),
                        );
                    } else {
                    };
                } else {
                };
            }
        }
        return status;
    }
}
pub unsafe extern "C" fn lua_dump(
    state: *mut State,
    writer_0: WriteFunction,
    data: *mut libc::c_void,
    is_strip: bool,
) -> i32 {
    unsafe {
        let status: i32;
        let o: *mut TValue = &mut (*(*state).top.p.offset(-(1 as isize))).tvalue;
        if (*o).get_tag_variant() == TAG_VARIANT_CLOSURE_L {
            status = DumpState::dump(
                state,
                (*((*o).value.object as *mut Closure)).payload.l_prototype,
                writer_0,
                data,
                is_strip,
            );
        } else {
            status = 1;
        }
        return status;
    }
}
pub unsafe extern "C" fn lua_gc(state: *mut State, what: i32, args: ...) -> i32 {
    unsafe {
        let mut argp: ::core::ffi::VaListImpl;
        let mut res: i32 = 0;
        let g: *mut Global = (*state).global;
        if (*g).gc_step as i32 & 2 != 0 {
            return -1;
        }
        argp = args.clone();
        match what {
            0 => {
                (*g).gc_step = 1;
            }
            1 => {
                (*g).set_debt(0);
                (*g).gc_step = 0;
            }
            2 => {
                luac_fullgc(state, false);
            }
            3 => {
                res = (((*g).total_bytes + (*g).gc_debt) as u64 >> 10 as i32) as i32;
            }
            4 => {
                res = (((*g).total_bytes + (*g).gc_debt) as u64 & 0x3ff as u64) as i32;
            }
            5 => {
                let data: i32 = argp.arg::<i32>();
                let mut debt: i64 = 1;
                let oldstp: u8 = (*g).gc_step;
                (*g).gc_step = 0;
                if data == 0 {
                    (*g).set_debt(0);
                    luac_step(state);
                } else {
                    debt = data as i64 * 1024 as i64 + (*g).gc_debt;
                    (*g).set_debt(debt);
                    if (*(*state).global).gc_debt > 0 {
                        luac_step(state);
                    }
                }
                (*g).gc_step = oldstp;
                if debt > 0 && (*g).gc_state as i32 == 8 {
                    res = 1;
                }
            }
            6 => {
                let data_0: i32 = argp.arg::<i32>();
                res = (*g).gc_pause as i32 * 4;
                (*g).gc_pause = (data_0 / 4) as u8;
            }
            7 => {
                let data_1: i32 = argp.arg::<i32>();
                res = (*g).gc_step_multiplier as i32 * 4;
                (*g).gc_step_multiplier = (data_1 / 4) as u8;
            }
            9 => {
                res = ((*g).gc_step as i32 == 0) as i32;
            }
            10 => {
                let minormul: i32 = argp.arg::<i32>();
                let majormul: i32 = argp.arg::<i32>();
                res = if (*g).gc_kind as i32 == 1 || (*g).last_atomic != 0u64 {
                    10 as i32
                } else {
                    11 as i32
                };
                if minormul != 0 {
                    (*g).generational_minor_multiplier = minormul as u64;
                }
                if majormul != 0 {
                    (*g).generational_major_multiplier = (majormul / 4) as u64;
                }
                luac_changemode(state, 1);
            }
            11 => {
                let pause: i32 = argp.arg::<i32>();
                let stepmul: i32 = argp.arg::<i32>();
                let stepsize: i32 = argp.arg::<i32>();
                res = if (*g).gc_kind as i32 == 1 || (*g).last_atomic != 0u64 {
                    10 as i32
                } else {
                    11 as i32
                };
                if pause != 0 {
                    (*g).gc_pause = (pause / 4) as u8;
                }
                if stepmul != 0 {
                    (*g).gc_step_multiplier = (stepmul / 4) as u8;
                }
                if stepsize != 0 {
                    (*g).gc_step_size = stepsize as u8;
                }
                luac_changemode(state, 0);
            }
            _ => {
                res = -1;
            }
        }
        return res;
    }
}
pub unsafe extern "C" fn lua_error(state: *mut State) -> i32 {
    unsafe {
        let errobj: *mut TValue = &mut (*(*state).top.p.offset(-(1 as isize))).tvalue;
        if (*errobj).get_tag_variant() == TAG_VARIANT_STRING_SHORT
            && &mut (*((*errobj).value.object as *mut TString)) as *mut TString
                == (*(*state).global).memory_error_message
        {
            luad_throw(state, 4);
        } else {
            luag_errormsg(state);
        };
    }
}
pub unsafe extern "C" fn lua_next(state: *mut State, index: i32) -> i32 {
    unsafe {
        let table: *mut Table = gettable(state, index);
        let more: i32 = luah_next(state, table, (*state).top.p.offset(-(1 as isize)));
        if more != 0 {
            (*state).top.p = (*state).top.p.offset(1);
        } else {
            (*state).top.p = (*state).top.p.offset(-(1 as isize));
        }
        return more;
    }
}
pub unsafe extern "C" fn lua_toclose(state: *mut State, index: i32) {
    unsafe {
        let o: StackValuePointer = index2stack(state, index);
        let count_results: i32 = (*(*state).call_info).count_results as i32;
        luaf_newtbcupval(state, o);
        if !(count_results < -1) {
            (*(*state).call_info).count_results = (-count_results - 3) as i16;
        }
    }
}
pub unsafe extern "C" fn lua_concat(state: *mut State, n: i32) {
    unsafe {
        if n > 0 {
            concatenate(state, n);
        } else {
            let io: *mut TValue = &mut (*(*state).top.p).tvalue;
            let x_: *mut TString = luas_newlstr(state, b"\0" as *const u8 as *const i8, 0u64);
            (*io).value.object = &mut (*(x_ as *mut Object));
            (*io).set_tag((*x_).get_tag());
            (*io).set_collectable();
            (*state).top.p = (*state).top.p.offset(1);
        }
        if (*(*state).global).gc_debt > 0 {
            luac_step(state);
        }
    }
}
pub unsafe extern "C" fn lua_len(state: *mut State, index: i32) {
    unsafe {
        let t: *mut TValue = (*state).index2value(index);
        luav_objlen(state, (*state).top.p, t);
        (*state).top.p = (*state).top.p.offset(1);
    }
}
pub unsafe extern "C" fn lua_setwarnf(state: *mut State, f: WarnFunction, arbitrary_data: *mut libc::c_void) {
    unsafe {
        (*(*state).global).warn_userdata = arbitrary_data;
        (*(*state).global).warn_function = f;
    }
}
pub unsafe extern "C" fn lua_warning(state: *mut State, message: *const i8, tocont: i32) {
    unsafe {
        luae_warning(state, message, tocont);
    }
}
pub unsafe extern "C" fn lua_getupvalue(state: *mut State, funcindex: i32, n: i32) -> *const i8 {
    unsafe {
        let mut value: *mut TValue = std::ptr::null_mut();
        let name: *const i8 = aux_upvalue(
            (*state).index2value(funcindex),
            n,
            &mut value,
            std::ptr::null_mut(),
        );
        if !name.is_null() {
            let io1: *mut TValue = &mut (*(*state).top.p).tvalue;
            let io2: *const TValue = value;
            (*io1).value = (*io2).value;
            (*io1).set_tag((*io2).get_tag());
            (*state).top.p = (*state).top.p.offset(1);
        }
        return name;
    }
}
pub unsafe extern "C" fn lua_setupvalue(state: *mut State, funcindex: i32, n: i32) -> *const i8 {
    unsafe {
        let mut value: *mut TValue = std::ptr::null_mut();
        let mut owner: *mut Object = std::ptr::null_mut();
        let fi: *mut TValue = (*state).index2value(funcindex);
        let name: *const i8 = aux_upvalue(fi, n, &mut value, &mut owner);
        if !name.is_null() {
            (*state).top.p = (*state).top.p.offset(-1);
            let io1: *mut TValue = value;
            let io2: *const TValue = &mut (*(*state).top.p).tvalue;
            (*io1).value = (*io2).value;
            (*io1).set_tag((*io2).get_tag());
            if (*value).is_collectable() {
                if (*owner).get_marked() & 1 << 5 != 0
                    && (*(*value).value.object).get_marked() & (1 << 3 | 1 << 4) != 0
                {
                    luac_barrier_(
                        state,
                        &mut (*(owner as *mut Object)),
                        &mut (*((*value).value.object as *mut Object)),
                    );
                } else {
                };
            } else {
            };
        }
        return name;
    }
}
pub const NULLUP: *const UpValue = std::ptr::null();
pub unsafe extern "C" fn getupvalref(
    state: *mut State,
    fidx: i32,
    n: i32,
    pf: *mut *mut Closure,
) -> *mut *mut UpValue {
    unsafe {
        let fi: *mut TValue = (*state).index2value(fidx);
        let f: *mut Closure = &mut (*((*fi).value.object as *mut Closure));
        if !pf.is_null() {
            *pf = f;
        }
        if 1 <= n && n <= (*(*f).payload.l_prototype).size_upvalues {
            return &mut *((*f).upvalues).l_upvalues.as_mut_ptr().offset((n - 1) as isize)
                as *mut *mut UpValue;
        } else {
            return &NULLUP as *const *const UpValue as *mut *mut UpValue;
        };
    }
}
pub unsafe extern "C" fn lua_upvalueid(state: *mut State, fidx: i32, n: i32) -> *mut libc::c_void {
    unsafe {
        let fi: *mut TValue = (*state).index2value(fidx);
        match (*fi).get_tag_variant() {
            TAG_VARIANT_CLOSURE_L => {
                return *getupvalref(state, fidx, n, std::ptr::null_mut()) as *mut libc::c_void;
            }
            TAG_VARIANT_CLOSURE_C => {
                let f: *mut Closure = &mut (*((*fi).value.object as *mut Closure));
                if 1 <= n && n <= (*f).count_upvalues as i32 {
                    return &mut *((*f).upvalues).c_tvalues.as_mut_ptr().offset((n - 1) as isize) as *mut TValue
                        as *mut libc::c_void;
                }
            }
            TAG_VARIANT_CLOSURE_CFUNCTION => {}
            _ => return std::ptr::null_mut(),
        }
        return std::ptr::null_mut();
    }
}
pub unsafe extern "C" fn lua_upvaluejoin(
    state: *mut State,
    fidx1: i32,
    n1: i32,
    fidx2: i32,
    n2: i32,
) {
    unsafe {
        let mut f1: *mut Closure = std::ptr::null_mut();
        let up1: *mut *mut UpValue = getupvalref(state, fidx1, n1, &mut f1);
        let up2: *mut *mut UpValue = getupvalref(state, fidx2, n2, std::ptr::null_mut());
        *up1 = *up2;
        if (*f1).get_marked() & 1 << 5 != 0 && (**up1).get_marked() & (1 << 3 | 1 << 4) != 0 {
            luac_barrier_(
                state,
                &mut (*(f1 as *mut Object)),
                &mut (*(*up1 as *mut Object)),
            );
        } else {
        };
    }
}
pub unsafe extern "C" fn luai_makeseed(state: *mut State) -> u32 {
    unsafe {
        let mut buffer: [i8; 24] = [0; 24];
        let mut h: u32 = time(std::ptr::null_mut()) as u32;
        let mut p: i32 = 0;
        let mut t: u64 = state as u64;
        memcpy(
            buffer.as_mut_ptr().offset(p as isize) as *mut libc::c_void,
            &mut t as *mut u64 as *const libc::c_void,
            ::core::mem::size_of::<u64>() as u64,
        );
        p = (p as u64).wrapping_add(::core::mem::size_of::<u64>() as u64) as i32;
        let mut t_0: u64 = &mut h as *mut u32 as u64;
        memcpy(
            buffer.as_mut_ptr().offset(p as isize) as *mut libc::c_void,
            &mut t_0 as *mut u64 as *const libc::c_void,
            ::core::mem::size_of::<u64>() as u64,
        );
        p = (p as u64).wrapping_add(::core::mem::size_of::<u64>() as u64) as i32;
        let mut t_1: u64 = ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> *mut State>,
            u64,
        >(Some(lua_newstate as unsafe extern "C" fn() -> *mut State));
        memcpy(
            buffer.as_mut_ptr().offset(p as isize) as *mut libc::c_void,
            &mut t_1 as *mut u64 as *const libc::c_void,
            ::core::mem::size_of::<u64>() as u64,
        );
        p = (p as u64).wrapping_add(::core::mem::size_of::<u64>() as u64) as i32;
        return luas_hash(buffer.as_mut_ptr(), p as u64, h);
    }
}
pub unsafe extern "C" fn luae_extendci(state: *mut State) -> *mut CallInfo {
    unsafe {
        let ret = luam_malloc_(state, ::core::mem::size_of::<CallInfo>() as u64) as *mut CallInfo;
        (*(*state).call_info).next = ret;
        (*ret).previous = (*state).call_info;
        (*ret).next = std::ptr::null_mut();
        ::core::ptr::write_volatile(&mut (*ret).u.l.trap as *mut i32, 0);
        (*state).count_call_info = ((*state).count_call_info).wrapping_add(1);
        (*state).count_call_info;
        return ret;
    }
}
pub unsafe extern "C" fn freeci(state: *mut State) {
    unsafe {
        let mut call_info: *mut CallInfo = (*state).call_info;
        let mut next: *mut CallInfo = (*call_info).next;
        (*call_info).next = std::ptr::null_mut();
        loop {
            call_info = next;
            if call_info.is_null() {
                break;
            }
            next = (*call_info).next;
            (*state).free_memory(
                call_info as *mut libc::c_void,
                ::core::mem::size_of::<CallInfo>() as u64,
            );
            (*state).count_call_info = ((*state).count_call_info).wrapping_sub(1);
            (*state).count_call_info;
        }
    }
}
pub unsafe extern "C" fn luae_shrinkci(state: *mut State) {
    unsafe {
        let mut call_info: *mut CallInfo = (*(*state).call_info).next;
        if !call_info.is_null() {
            let mut next: *mut CallInfo;
            loop {
                next = (*call_info).next;
                if next.is_null() {
                    break;
                }
                let next2: *mut CallInfo = (*next).next;
                (*call_info).next = next2;
                (*state).count_call_info = ((*state).count_call_info).wrapping_sub(1);
                (*state).count_call_info;
                (*state).free_memory(
                    next as *mut libc::c_void,
                    ::core::mem::size_of::<CallInfo>() as u64,
                );
                if next2.is_null() {
                    break;
                }
                (*next2).previous = call_info;
                call_info = next2;
            }
        }
    }
}
pub unsafe extern "C" fn stack_init(other_state: *mut State, state: *mut State) {
    unsafe {
        let mut i: i32;
        let call_info;
        (*other_state).stack.p = luam_malloc_(
            state,
            ((2 * 20 as i32 + 5) as u64).wrapping_mul(::core::mem::size_of::<StackValue>() as u64),
        ) as *mut StackValue;
        (*other_state).tbc_list.p = (*other_state).stack.p;
        i = 0;
        while i < 2 * 20 as i32 + 5 {
            (*((*other_state).stack.p).offset(i as isize))
                .tvalue
                .set_tag(TAG_VARIANT_NIL_NIL);
            i += 1;
        }
        (*other_state).top.p = (*other_state).stack.p;
        (*other_state).stack_last.p = ((*other_state).stack.p).offset((2 * 20 as i32) as isize);
        call_info = &mut (*other_state).base_callinfo;
        (*call_info).previous = std::ptr::null_mut();
        (*call_info).next = (*call_info).previous;
        (*call_info).call_status = (1 << 1) as u16;
        (*call_info).function.p = (*other_state).top.p;
        (*call_info).u.c.context_function = None;
        (*call_info).count_results = 0 as i16;
        (*(*other_state).top.p).tvalue.set_tag(TAG_VARIANT_NIL_NIL);
        (*other_state).top.p = ((*other_state).top.p).offset(1);
        (*other_state).top.p;
        (*call_info).top.p = ((*other_state).top.p).offset(20 as isize);
        (*other_state).call_info = call_info;
    }
}
pub unsafe extern "C" fn freestack(state: *mut State) {
    unsafe {
        if ((*state).stack.p).is_null() {
            return;
        }
        (*state).call_info = &mut (*state).base_callinfo;
        freeci(state);
        (*state).free_memory(
            (*state).stack.p as *mut libc::c_void,
            ((((*state).stack_last.p).offset_from((*state).stack.p) as i32 + 5) as u64)
                .wrapping_mul(::core::mem::size_of::<StackValue>() as u64),
        );
    }
}
pub unsafe extern "C" fn init_registry(state: *mut State, g: *mut Global) {
    unsafe {
        let registry: *mut Table = luah_new(state);
        let io: *mut TValue = &mut (*g).l_registry;
        let x_: *mut Table = registry;
        (*io).value.object = &mut (*(x_ as *mut Object));
        (*io).set_tag(TAG_VARIANT_TABLE);
        (*io).set_collectable();
        luah_resize(state, registry, 2 as u32, 0);
        let io_0: *mut TValue = &mut *((*registry).array).offset((1 - 1) as isize) as *mut TValue;
        let x0: *mut State = state;
        (*io_0).value.object = &mut (*(x0 as *mut Object));
        (*io_0).set_tag(TAG_VARIANT_STATE);
        (*io_0).set_collectable();
        let io_1: *mut TValue = &mut *((*registry).array).offset((2 - 1) as isize) as *mut TValue;
        let x1: *mut Table = luah_new(state);
        (*io_1).value.object = &mut (*(x1 as *mut Object));
        (*io_1).set_tag(TAG_VARIANT_TABLE);
        (*io_1).set_collectable();
    }
}
pub unsafe extern "C" fn f_luaopen(state: *mut State, mut _ud: *mut libc::c_void) {
    unsafe {
        let g: *mut Global = (*state).global;
        stack_init(state, state);
        init_registry(state, g);
        luas_init(state);
        luat_init(state);
        luax_init(state);
        (*g).gc_step = 0;
        (*g).nil_value.set_tag(TAG_VARIANT_NIL_NIL);
    }
}
pub unsafe extern "C" fn preinit_thread(state: *mut State, g: *mut Global) {
    unsafe {
        (*state).global = g;
        (*state).stack.p = std::ptr::null_mut();
        (*state).call_info = std::ptr::null_mut();
        (*state).count_call_info = 0;
        (*state).twups = state;
        (*state).count_c_calls = 0;
        (*state).long_jump = std::ptr::null_mut();
        ::core::ptr::write_volatile(&mut (*state).hook as *mut HookFunction, None);
        ::core::ptr::write_volatile(&mut (*state).hook_mask as *mut i32, 0);
        (*state).base_hook_count = 0;
        (*state).allow_hook = 1;
        (*state).hook_count = (*state).base_hook_count;
        (*state).open_upvalue = std::ptr::null_mut();
        (*state).status = 0;
        (*state).error_function = 0;
        (*state).old_program_counter = 0;
    }
}
pub unsafe extern "C" fn close_state(state: *mut State) {
    unsafe {
        let g: *mut Global = (*state).global;
        if !(get_tag_type((*g).nil_value.get_tag()) == TAG_TYPE_NIL) {
            luac_freeallobjects(state);
        } else {
            (*state).call_info = &mut (*state).base_callinfo;
            (*state).error_function = 0;
            luad_closeprotected(state, 1 as i64, 0);
            (*state).top.p = ((*state).stack.p).offset(1 as isize);
            luac_freeallobjects(state);
        }
        (*state).free_memory(
            (*(*state).global).string_table.hash as *mut libc::c_void,
            ((*(*state).global).string_table.size as u64)
                .wrapping_mul(::core::mem::size_of::<*mut TString>() as u64),
        );
        freestack(state);
        raw_allocate(
            (state as *mut u8).offset(-(8 as isize)) as *mut StateExtra as *mut libc::c_void,
            ::core::mem::size_of::<Interpreter>() as u64,
            0u64,
        );
    }
}
pub unsafe extern "C" fn lua_newthread(state: *mut State) -> *mut State {
    unsafe {
        let g: *mut Global = (*state).global;
        if (*(*state).global).gc_debt > 0 {
            luac_step(state);
        }
        let o: *mut Object = luac_newobjdt(
            state,
            TAG_TYPE_STATE,
            ::core::mem::size_of::<StateExtra>() as u64,
            8 as u64,
        );
        let other_state: *mut State = &mut (*(o as *mut State));
        let io: *mut TValue = &mut (*(*state).top.p).tvalue;
        let x_: *mut State = other_state;
        (*io).value.object = &mut (*(x_ as *mut Object));
        (*io).set_tag(TAG_VARIANT_STATE);
        (*io).set_collectable();
        (*state).top.p = (*state).top.p.offset(1);
        preinit_thread(other_state, g);
        ::core::ptr::write_volatile(
            &mut (*other_state).hook_mask as *mut i32,
            (*state).hook_mask,
        );
        (*other_state).base_hook_count = (*state).base_hook_count;
        ::core::ptr::write_volatile(&mut (*other_state).hook as *mut HookFunction, (*state).hook);
        (*other_state).hook_count = (*other_state).base_hook_count;
        memcpy(
            (other_state as *mut i8)
                .offset(-(::core::mem::size_of::<*mut libc::c_void>() as isize))
                as *mut libc::c_void,
            ((*g).main_state as *mut i8)
                .offset(-(::core::mem::size_of::<*mut libc::c_void>() as isize))
                as *mut libc::c_void,
            ::core::mem::size_of::<*mut libc::c_void>() as u64,
        );
        stack_init(other_state, state);
        return other_state;
    }
}
pub unsafe extern "C" fn luae_freethread(state: *mut State, other_state: *mut State) {
    unsafe {
        let l: *mut StateExtra = (other_state as *mut u8).offset(-(8 as isize)) as *mut StateExtra;
        luaf_closeupval(other_state, (*other_state).stack.p);
        freestack(other_state);
        (*state).free_memory(l as *mut libc::c_void, ::core::mem::size_of::<StateExtra>() as u64);
    }
}
pub unsafe extern "C" fn luae_resetthread(state: *mut State, mut status: i32) -> i32 {
    unsafe {
        (*state).call_info = &mut (*state).base_callinfo;
        let call_info: *mut CallInfo = (*state).call_info;
        (*(*state).stack.p).tvalue.set_tag(TAG_VARIANT_NIL_NIL);
        (*call_info).function.p = (*state).stack.p;
        (*call_info).call_status = (1 << 1) as u16;
        if status == 1 {
            status = 0;
        }
        (*state).status = 0;
        (*state).error_function = 0;
        status = luad_closeprotected(state, 1 as i64, status);
        if status != 0 {
            (*state).set_error_object(status, ((*state).stack.p).offset(1 as isize));
        } else {
            (*state).top.p = ((*state).stack.p).offset(1 as isize);
        }
        (*call_info).top.p = (*state).top.p.offset(20 as isize);
        luad_reallocstack(
            state,
            ((*call_info).top.p).offset_from((*state).stack.p) as i32,
            false,
        );
        return status;
    }
}
pub unsafe extern "C" fn lua_closethread(state: *mut State, from: *mut State) -> i32 {
    unsafe {
        let status: i32;
        (*state).count_c_calls = if !from.is_null() {
            (*from).count_c_calls & 0xffff as u32
        } else {
            0
        };
        status = luae_resetthread(state, (*state).status as i32);
        return status;
    }
}
pub unsafe extern "C" fn lua_newstate() -> *mut State {
    unsafe {
        let mut i: i32;
        let l: *mut Interpreter = raw_allocate(
            std::ptr::null_mut(),
            8 as u64,
            ::core::mem::size_of::<Interpreter>() as u64,
        ) as *mut Interpreter;
        if l.is_null() {
            return std::ptr::null_mut();
        }
        let mut state: *mut State = &mut (*l).state_extra.state;
        let g: *mut Global = &mut (*l).global;
        (*state).set_tag(TAG_TYPE_STATE);
        (*g).current_white = (1 << 3) as u8;
        (*state).set_marked((*g).current_white & (1 << 3 | 1 << 4));
        preinit_thread(state, g);
        (*g).all_gc = &mut (*(state as *mut Object));
        (*state).object.next = std::ptr::null_mut();
        (*state).count_c_calls =
            ((*state).count_c_calls as u32).wrapping_add(0x10000 as u32) as u32;
        (*g).warn_function = None;
        (*g).warn_userdata = std::ptr::null_mut();
        (*g).main_state = state;
        (*g).seed = luai_makeseed(state);
        (*g).gc_step = 2 as u8;
        (*g).string_table.length = 0;
        (*g).string_table.size = (*g).string_table.length;
        (*g).string_table.hash = std::ptr::null_mut();
        (*g).l_registry.set_tag(TAG_VARIANT_NIL_NIL);
        (*g).panic = None;
        (*g).gc_state = 8 as u8;
        (*g).gc_kind = 0;
        (*g).gcstopem = 0;
        (*g).is_emergency = false;
        (*g).fixed_gc = std::ptr::null_mut();
        (*g).to_be_finalized = (*g).fixed_gc;
        (*g).finalized_objects = (*g).to_be_finalized;
        (*g).really_old = std::ptr::null_mut();
        (*g).old1 = (*g).really_old;
        (*g).survival = (*g).old1;
        (*g).first_old1 = (*g).survival;
        (*g).finobjrold = std::ptr::null_mut();
        (*g).finobjold1 = (*g).finobjrold;
        (*g).finobjsur = (*g).finobjold1;
        (*g).sweep_gc = std::ptr::null_mut();
        (*g).gray_again = std::ptr::null_mut();
        (*g).gray = (*g).gray_again;
        (*g).all_weak = std::ptr::null_mut();
        (*g).ephemeron = (*g).all_weak;
        (*g).weak = (*g).ephemeron;
        (*g).twups = std::ptr::null_mut();
        (*g).total_bytes = ::core::mem::size_of::<Interpreter>() as i64;
        (*g).gc_debt = 0;
        (*g).last_atomic = 0;
        let io: *mut TValue = &mut (*g).nil_value;
        (*io).value.integer = 0;
        (*io).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
        (*g).gc_pause = 200 / 4;
        (*g).gc_step_multiplier = 100 / 4;
        (*g).gc_step_size = 13;
        (*g).generational_major_multiplier = 100 / 4;
        (*g).generational_minor_multiplier = 20;
        i = 0;
        while i < 9 as i32 {
            (*g).metatable[i as usize] = std::ptr::null_mut();
            i += 1;
        }
        if luad_rawrunprotected(
            state,
            Some(f_luaopen as unsafe extern "C" fn(*mut State, *mut libc::c_void) -> ()),
            std::ptr::null_mut(),
        ) != 0
        {
            close_state(state);
            state = std::ptr::null_mut();
        }
        return state;
    }
}
pub unsafe extern "C" fn lua_close(mut state: *mut State) {
    unsafe {
        state = (*(*state).global).main_state;
        close_state(state);
    }
}
pub unsafe extern "C" fn luae_warning(state: *mut State, message: *const i8, tocont: i32) {
    unsafe {
        let wf: WarnFunction = (*(*state).global).warn_function;
        if wf.is_some() {
            wf.expect("non-null function pointer")((*(*state).global).warn_userdata, message, tocont);
        }
    }
}
pub unsafe extern "C" fn luae_warnerror(state: *mut State, where_0: *const i8) {
    unsafe {
        let errobj: *mut TValue = &mut (*(*state).top.p.offset(-(1 as isize))).tvalue;
        let message: *const i8 = if get_tag_type((*errobj).get_tag()) == TAG_TYPE_STRING {
            ((*((*errobj).value.object as *mut TString)).get_contents()) as *const i8
        } else {
            b"error object is not a string\0" as *const u8 as *const i8
        };
        luae_warning(state, b"error in \0" as *const u8 as *const i8, 1);
        luae_warning(state, where_0, 1);
        luae_warning(state, b" (\0" as *const u8 as *const i8, 1);
        luae_warning(state, message, 1);
        luae_warning(state, b")\0" as *const u8 as *const i8, 0);
    }
}
pub unsafe extern "C" fn lua_sethook(
    state: *mut State,
    mut function: HookFunction,
    mut mask: i32,
    count: i32,
) {
    unsafe {
        if function.is_none() || mask == 0 {
            mask = 0;
            function = None;
        }
        ::core::ptr::write_volatile(&mut (*state).hook as *mut HookFunction, function);
        (*state).base_hook_count = count;
        (*state).hook_count = (*state).base_hook_count;
        ::core::ptr::write_volatile(&mut (*state).hook_mask as *mut i32, mask as u8 as i32);
        if mask != 0 {
            settraps((*state).call_info);
        }
    }
}
pub unsafe extern "C" fn lua_gethook(state: *mut State) -> HookFunction {
    unsafe {
        return (*state).hook;
    }
}
pub unsafe extern "C" fn lua_gethookmask(state: *mut State) -> i32 {
    unsafe {
        return (*state).hook_mask;
    }
}
pub unsafe extern "C" fn lua_gethookcount(state: *mut State) -> i32 {
    unsafe {
        return (*state).base_hook_count;
    }
}
pub unsafe extern "C" fn lua_getstack(state: *mut State, mut level: i32, ar: *mut DebugInfo) -> i32 {
    unsafe {
        let status: i32;
        let mut call_info;
        if level < 0 {
            return 0;
        }
        call_info = (*state).call_info;
        while level > 0 && call_info != &mut (*state).base_callinfo as *mut CallInfo {
            level -= 1;
            call_info = (*call_info).previous;
        }
        if level == 0 && call_info != &mut (*state).base_callinfo as *mut CallInfo {
            status = 1;
            (*ar).i_ci = call_info;
        } else {
            status = 0;
        }
        return status;
    }
}
pub unsafe extern "C" fn formatvarinfo(
    state: *mut State,
    kind: *const i8,
    name: *const i8,
) -> *const i8 {
    unsafe {
        if kind.is_null() {
            return b"\0" as *const u8 as *const i8;
        } else {
            return luao_pushfstring(state, b" (%s '%s')\0" as *const u8 as *const i8, kind, name);
        };
    }
}
pub unsafe extern "C" fn varinfo(state: *mut State, o: *const TValue) -> *const i8 {
    unsafe {
        let call_info: *mut CallInfo = (*state).call_info;
        let mut name: *const i8 = std::ptr::null();
        let mut kind: *const i8 = std::ptr::null();
        if (*call_info).call_status as i32 & 1 << 1 == 0 {
            kind = getupvalname(call_info, o, &mut name);
            if kind.is_null() {
                let reg: i32 = in_stack(call_info, o);
                if reg >= 0 {
                    kind = getobjname(
                        (*((*(*call_info).function.p).tvalue.value.object as *mut Closure))
                            .payload.l_prototype,
                        currentpc(call_info),
                        reg,
                        &mut name,
                    );
                }
            }
        }
        return formatvarinfo(state, kind, name);
    }
}
pub unsafe extern "C" fn typeerror(
    state: *mut State,
    o: *const TValue,
    op: *const i8,
    extra: *const i8,
) -> ! {
    unsafe {
        let t: *const i8 = luat_objtypename(state, o);
        luag_runerror(
            state,
            b"attempt to %s a %s value%s\0" as *const u8 as *const i8,
            op,
            t,
            extra,
        );
    }
}
pub unsafe extern "C" fn luag_typeerror(state: *mut State, o: *const TValue, op: *const i8) -> ! {
    unsafe {
        typeerror(state, o, op, varinfo(state, o));
    }
}
pub unsafe extern "C" fn luag_callerror(state: *mut State, o: *const TValue) -> ! {
    unsafe {
        let call_info: *mut CallInfo = (*state).call_info;
        let mut name: *const i8 = std::ptr::null();
        let kind: *const i8 = funcnamefromcall(state, call_info, &mut name);
        let extra: *const i8 = if !kind.is_null() {
            formatvarinfo(state, kind, name)
        } else {
            varinfo(state, o)
        };
        typeerror(state, o, b"call\0" as *const u8 as *const i8, extra);
    }
}
pub unsafe extern "C" fn luag_forerror(state: *mut State, o: *const TValue, what: *const i8) -> ! {
    unsafe {
        luag_runerror(
            state,
            b"bad 'for' %s (number expected, got %s)\0" as *const u8 as *const i8,
            what,
            luat_objtypename(state, o),
        );
    }
}
pub unsafe extern "C" fn luag_concaterror(
    state: *mut State,
    mut p1: *const TValue,
    p2: *const TValue,
) -> ! {
    unsafe {
        if get_tag_type((*p1).get_tag()) == TAG_TYPE_STRING
            || get_tag_type((*p1).get_tag()) == TAG_TYPE_NUMERIC
        {
            p1 = p2;
        }
        luag_typeerror(state, p1, b"concatenate\0" as *const u8 as *const i8);
    }
}
pub unsafe extern "C" fn luag_opinterror(
    state: *mut State,
    p1: *const TValue,
    mut p2: *const TValue,
    message: *const i8,
) -> ! {
    unsafe {
        if get_tag_type((*p1).get_tag()) != 3 {
            p2 = p1;
        }
        luag_typeerror(state, p2, message);
    }
}
pub unsafe extern "C" fn luag_tointerror(
    state: *mut State,
    p1: *const TValue,
    mut p2: *const TValue,
) -> ! {
    unsafe {
        let mut temp: i64 = 0;
        if luav_tointegerns(p1, &mut temp, F2I::Equal) == 0 {
            p2 = p1;
        }
        luag_runerror(
            state,
            b"number%s has no integer representation\0" as *const u8 as *const i8,
            varinfo(state, p2),
        );
    }
}
pub unsafe extern "C" fn luag_ordererror(
    state: *mut State,
    p1: *const TValue,
    p2: *const TValue,
) -> ! {
    unsafe {
        let t1: *const i8 = luat_objtypename(state, p1);
        let t2: *const i8 = luat_objtypename(state, p2);
        if strcmp(t1, t2) == 0 {
            luag_runerror(
                state,
                b"attempt to compare two %s values\0" as *const u8 as *const i8,
                t1,
            );
        } else {
            luag_runerror(
                state,
                b"attempt to compare %s with %s\0" as *const u8 as *const i8,
                t1,
                t2,
            );
        };
    }
}
pub unsafe extern "C" fn luag_addinfo(
    state: *mut State,
    message: *const i8,
    src: *mut TString,
    line: i32,
) -> *const i8 {
    unsafe {
        let mut buffer: [i8; 60] = [0; 60];
        if !src.is_null() {
            luao_chunkid(
                buffer.as_mut_ptr(),
                (*src).get_contents(),
                (*src).get_length(),
            );
        } else {
            buffer[0] = '?' as i8;
            buffer[1] = '\0' as i8;
        }
        return luao_pushfstring(
            state,
            b"%s:%d: %s\0" as *const u8 as *const i8,
            buffer.as_mut_ptr(),
            line,
            message,
        );
    }
}
pub unsafe extern "C" fn luag_errormsg(state: *mut State) -> ! {
    unsafe {
        if (*state).error_function != 0 {
            let error_function: StackValuePointer =
                ((*state).stack.p as *mut i8).offset((*state).error_function as isize) as StackValuePointer;
            let io1: *mut TValue = &mut (*(*state).top.p).tvalue;
            let io2: *const TValue = &mut (*(*state).top.p.offset(-(1 as isize))).tvalue;
            (*io1).value = (*io2).value;
            (*io1).set_tag((*io2).get_tag());
            let io1_0: *mut TValue = &mut (*(*state).top.p.offset(-(1 as isize))).tvalue;
            let io2_0: *const TValue = &mut (*error_function).tvalue;
            (*io1_0).value = (*io2_0).value;
            (*io1_0).set_tag((*io2_0).get_tag());
            (*state).top.p = (*state).top.p.offset(1);
            luad_callnoyield(state, (*state).top.p.offset(-(2 as isize)), 1);
        }
        luad_throw(state, 2);
    }
}
pub unsafe extern "C" fn luag_runerror(state: *mut State, fmt: *const i8, args: ...) -> ! {
    unsafe {
        let call_info: *mut CallInfo = (*state).call_info;
        let message: *const i8;
        let mut argp: ::core::ffi::VaListImpl;
        if (*(*state).global).gc_debt > 0 {
            luac_step(state);
        }
        argp = args.clone();
        message = luao_pushvfstring(state, fmt, argp.as_va_list());
        if (*call_info).call_status as i32 & 1 << 1 == 0 {
            luag_addinfo(
                state,
                message,
                (*(*((*(*call_info).function.p).tvalue.value.object as *mut Closure))
                    .payload.l_prototype)
                    .source,
                getcurrentline(call_info),
            );
            let io1: *mut TValue = &mut (*(*state).top.p.offset(-(2 as isize))).tvalue;
            let io2: *const TValue = &mut (*(*state).top.p.offset(-(1 as isize))).tvalue;
            (*io1).value = (*io2).value;
            (*io1).set_tag((*io2).get_tag());
            (*state).top.p = (*state).top.p.offset(-1);
        }
        luag_errormsg(state);
    }
}
pub unsafe extern "C" fn luag_tracecall(state: *mut State) -> i32 {
    unsafe {
        let call_info: *mut CallInfo = (*state).call_info;
        let p: *mut Prototype = (*((*(*call_info).function.p).tvalue.value.object as *mut Closure))
            .payload.l_prototype;
        ::core::ptr::write_volatile(&mut (*call_info).u.l.trap as *mut i32, 1);
        if (*call_info).u.l.saved_program_counter == (*p).code as *const u32 {
            if (*p).is_variable_arguments {
                return 0;
            } else if (*call_info).call_status as i32 & 1 << 6 == 0 {
                luad_hookcall(state, call_info);
            }
        }
        return 1;
    }
}
pub unsafe extern "C" fn luag_traceexec(state: *mut State, mut program_counter: *const u32) -> i32 {
    unsafe {
        let call_info: *mut CallInfo = (*state).call_info;
        let mask: u8 = (*state).hook_mask as u8;
        let p: *const Prototype = (*((*(*call_info).function.p).tvalue.value.object
            as *mut Closure))
            .payload.l_prototype;
        if mask as i32 & (1 << 2 | 1 << 3) == 0 {
            ::core::ptr::write_volatile(&mut (*call_info).u.l.trap as *mut i32, 0);
            return 0;
        }
        program_counter = program_counter.offset(1);
        (*call_info).u.l.saved_program_counter = program_counter;
        let counthook: i32 = (mask as i32 & 1 << 3 != 0 && {
            (*state).hook_count -= 1;
            (*state).hook_count == 0
        }) as i32;
        if counthook != 0 {
            (*state).hook_count = (*state).base_hook_count;
        } else if mask as i32 & 1 << 2 == 0 {
            return 1;
        }
        if (*call_info).call_status as i32 & 1 << 6 != 0 {
            (*call_info).call_status = ((*call_info).call_status as i32 & !(1 << 6)) as u16;
            return 1;
        }
        if !(OPMODES[(*((*call_info).u.l.saved_program_counter).offset(-(1 as isize)) >> 0
            & !(!(0u32) << 7) << 0) as usize] as i32
            & 1 << 5
            != 0
            && (*((*call_info).u.l.saved_program_counter).offset(-(1 as isize)) >> 0 + 7 + 8 + 1
                & !(!(0u32) << 8) << 0) as i32
                == 0)
        {
            (*state).top.p = (*call_info).top.p;
        }
        if counthook != 0 {
            luad_hook(state, 3, -1, 0, 0);
        }
        if mask as i32 & 1 << 2 != 0 {
            let old_program_counter: i32 = if (*state).old_program_counter < (*p).size_code {
                (*state).old_program_counter
            } else {
                0
            };
            let npci: i32 = program_counter.offset_from((*p).code) as i32 - 1;
            if npci <= old_program_counter || changedline(p, old_program_counter, npci) != 0 {
                let newline: i32 = luag_getfuncline(p, npci);
                luad_hook(state, 2, newline, 0, 0);
            }
            (*state).old_program_counter = npci;
        }
        if (*state).status as i32 == 1 {
            if counthook != 0 {
                (*state).hook_count = 1;
            }
            (*call_info).call_status = ((*call_info).call_status as i32 | 1 << 6) as u16;
            luad_throw(state, 1);
        }
        return 1;
    }
}
pub unsafe extern "C" fn luam_growaux_(
    state: *mut State,
    block: *mut libc::c_void,
    count_elements: i32,
    total_size: *mut i32,
    element_size: i32,
    limit: i32,
    what: *const i8,
) -> *mut libc::c_void {
    unsafe {
        let mut size: i32 = *total_size;
        if count_elements + 1 <= size {
            return block;
        }
        if size >= limit / 2 {
            if ((size >= limit) as i32 != 0) as i64 != 0 {
                luag_runerror(
                    state,
                    b"too many %s (limit is %d)\0" as *const u8 as *const i8,
                    what,
                    limit,
                );
            }
            size = limit;
        } else {
            size *= 2;
            if size < 4 {
                size = 4;
            }
        }
        let new_block: *mut libc::c_void = luam_saferealloc_(
            state,
            block,
            (*total_size as u64).wrapping_mul(element_size as u64),
            (size as u64).wrapping_mul(element_size as u64),
        );
        *total_size = size;
        return new_block;
    }
}
pub unsafe extern "C" fn luam_shrinkvector_(
    state: *mut State,
    block: *mut libc::c_void,
    size: *mut i32,
    count_elements: i32,
    element_size: i32,
) -> *mut libc::c_void {
    unsafe {
        let old_size: u64 = (*size * element_size) as u64;
        let new_size: u64 = (count_elements * element_size) as u64;
        let new_block: *mut libc::c_void = luam_saferealloc_(state, block, old_size, new_size);
        *size = count_elements;
        return new_block;
    }
}
pub unsafe extern "C" fn tryagain(
    state: *mut State,
    block: *mut libc::c_void,
    old_size: u64,
    new_size: u64,
) -> *mut libc::c_void {
    unsafe {
        let g: *mut Global = (*state).global;
        if get_tag_type((*g).nil_value.get_tag()) == TAG_TYPE_NIL && (*g).gcstopem == 0 {
            luac_fullgc(state, true);
            return raw_allocate(block, old_size, new_size);
        } else {
            return std::ptr::null_mut();
        };
    }
}
pub unsafe extern "C" fn luam_realloc_(
    state: *mut State,
    block: *mut libc::c_void,
    old_size: u64,
    new_size: u64,
) -> *mut libc::c_void {
    unsafe {
        let g: *mut Global = (*state).global;
        let mut new_block: *mut libc::c_void = raw_allocate(block, old_size, new_size);
        if ((new_block.is_null() && new_size > 0u64) as i32 != 0) as i64 != 0 {
            new_block = tryagain(state, block, old_size, new_size);
            if new_block.is_null() {
                return std::ptr::null_mut();
            }
        }
        (*g).gc_debt = ((*g).gc_debt as u64)
            .wrapping_add(new_size)
            .wrapping_sub(old_size) as i64;
        return new_block;
    }
}
pub unsafe extern "C" fn luam_saferealloc_(
    state: *mut State,
    block: *mut libc::c_void,
    old_size: u64,
    new_size: u64,
) -> *mut libc::c_void {
    unsafe {
        let new_block: *mut libc::c_void = luam_realloc_(state, block, old_size, new_size);
        if ((new_block.is_null() && new_size > 0u64) as i32 != 0) as i64 != 0 {
            luad_throw(state, 4);
        }
        return new_block;
    }
}
pub unsafe extern "C" fn luam_malloc_(state: *mut State, size: u64) -> *mut libc::c_void {
    unsafe {
        if size == 0 {
            return std::ptr::null_mut();
        } else {
            let g: *mut Global = (*state).global;
            let mut new_block: *mut libc::c_void = raw_allocate(std::ptr::null_mut(), 0, size);
            if new_block.is_null() {
                new_block = tryagain(state, std::ptr::null_mut(), 0, size);
                if new_block.is_null() {
                    luad_throw(state, 4);
                }
            }
            (*g).gc_debt = ((*g).gc_debt as u64).wrapping_add(size) as i64;
            return new_block;
        };
    }
}
pub unsafe extern "C" fn intarith(state: *mut State, op: i32, v1: i64, v2: i64) -> i64 {
    unsafe {
        match op {
            0 => return (v1 as u64).wrapping_add(v2 as u64) as i64,
            1 => return (v1 as u64).wrapping_sub(v2 as u64) as i64,
            2 => return (v1 as u64).wrapping_mul(v2 as u64) as i64,
            3 => return luav_mod(state, v1, v2),
            6 => return luav_idiv(state, v1, v2),
            7 => return (v1 as u64 & v2 as u64) as i64,
            8 => return (v1 as u64 | v2 as u64) as i64,
            9 => return (v1 as u64 ^ v2 as u64) as i64,
            10 => return luav_shiftl(v1, v2),
            11 => {
                return luav_shiftl(v1, (0u64).wrapping_sub(v2 as u64) as i64);
            }
            12 => {
                return (0u64).wrapping_sub(v1 as u64) as i64;
            }
            13 => {
                return (!(0u64) ^ v1 as u64) as i64;
            }
            _ => return 0,
        };
    }
}
pub unsafe extern "C" fn numarith(state: *mut State, op: i32, v1: f64, v2: f64) -> f64 {
    unsafe {
        match op {
            0 => return v1 + v2,
            1 => return v1 - v2,
            2 => return v1 * v2,
            5 => return v1 / v2,
            4 => {
                return if v2 == 2.0 { v1 * v1 } else { v1.powf(v2) };
            }
            6 => return (v1 / v2).floor(),
            12 => return -v1,
            3 => return luav_modf(state, v1, v2),
            _ => return 0.0,
        };
    }
}
pub unsafe extern "C" fn luao_rawarith(
    state: *mut State,
    op: i32,
    p1: *const TValue,
    p2: *const TValue,
    res: *mut TValue,
) -> i32 {
    unsafe {
        match op {
            7 | 8 | 9 | 10 | 11 | 13 => {
                let mut i1: i64 = 0;
                let mut i2: i64 = 0;
                if (if (((*p1).get_tag() == TAG_VARIANT_NUMERIC_INTEGER) as i32 != 0) as i64
                    != 0
                {
                    i1 = (*p1).value.integer;
                    1
                } else {
                    luav_tointegerns(p1, &mut i1, F2I::Equal)
                }) != 0
                    && (if (((*p2).get_tag() == TAG_VARIANT_NUMERIC_INTEGER) as i32 != 0) as i32
                        as i64
                        != 0
                    {
                        i2 = (*p2).value.integer;
                        1
                    } else {
                        luav_tointegerns(p2, &mut i2, F2I::Equal)
                    }) != 0
                {
                    (*res).value.integer = intarith(state, op, i1, i2);
                    (*res).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                    return 1;
                } else {
                    return 0;
                }
            }
            5 | 4 => {
                let mut n1: f64 = 0.0;
                let mut n2: f64 = 0.0;
                if (if (*p1).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                    n1 = (*p1).value.number;
                    1
                } else {
                    if (*p1).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                        n1 = (*p1).value.integer as f64;
                        1
                    } else {
                        0
                    }
                }) != 0
                    && (if (*p2).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                        n2 = (*p2).value.number;
                        1
                    } else {
                        if (*p2).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                            n2 = (*p2).value.integer as f64;
                            1
                        } else {
                            0
                        }
                    }) != 0
                {
                    (*res).value.number = numarith(state, op, n1, n2);
                    (*res).set_tag(TAG_VARIANT_NUMERIC_NUMBER);
                    return 1;
                } else {
                    return 0;
                }
            }
            _ => {
                let mut n1_0: f64 = 0.0;
                let mut n2_0: f64 = 0.0;
                if (*p1).get_tag() == TAG_VARIANT_NUMERIC_INTEGER
                    && (*p2).get_tag() == TAG_VARIANT_NUMERIC_INTEGER
                {
                    let io_1: *mut TValue = res;
                    (*io_1).value.integer = intarith(state, op, (*p1).value.integer, (*p2).value.integer);
                    (*io_1).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                    return 1;
                } else if (if (*p1).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                    n1_0 = (*p1).value.number;
                    1
                } else {
                    if (*p1).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                        n1_0 = (*p1).value.integer as f64;
                        1
                    } else {
                        0
                    }
                }) != 0
                    && (if (*p2).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                        n2_0 = (*p2).value.number;
                        1
                    } else {
                        if (*p2).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                            n2_0 = (*p2).value.integer as f64;
                            1
                        } else {
                            0
                        }
                    }) != 0
                {
                    let io_2: *mut TValue = res;
                    (*io_2).value.number = numarith(state, op, n1_0, n2_0);
                    (*io_2).set_tag(TAG_VARIANT_NUMERIC_NUMBER);
                    return 1;
                } else {
                    return 0;
                }
            }
        };
    }
}
pub unsafe extern "C" fn luao_arith(
    state: *mut State,
    op: i32,
    p1: *const TValue,
    p2: *const TValue,
    res: StackValuePointer,
) {
    unsafe {
        if luao_rawarith(state, op, p1, p2, &mut (*res).tvalue) == 0 {
            luat_trybintm(state, p1, p2, res, (op - 0 + TM_ADD as i32) as u32);
        }
    }
}
pub unsafe extern "C" fn luao_pushvfstring(
    state: *mut State,
    mut fmt: *const i8,
    mut argp: ::core::ffi::VaList,
) -> *const i8 {
    unsafe {
        let mut buff_fs = BuffFS::new(state);
        let mut e: *const i8;
        loop {
            e = strchr(fmt, '%' as i32);
            if e.is_null() {
                break;
            }
            buff_fs.add_string(fmt, e.offset_from(fmt) as u64);
            match *e.offset(1 as isize) as i32 {
                115 => {
                    let mut s: *const i8 = argp.arg::<*mut i8>();
                    if s.is_null() {
                        s = b"(null)\0" as *const u8 as *const i8;
                    }
                    buff_fs.add_string(s, strlen(s));
                }
                99 => {
                    let mut c: i8 = argp.arg::<i32>() as u8 as i8;
                    buff_fs.add_string(&mut c, ::core::mem::size_of::<i8>() as u64);
                }
                100 => {
                    let mut num: TValue = TValue {
                        value: Value {
                            object: std::ptr::null_mut(),
                        },
                        tag: 0,
                    };
                    let io: *mut TValue = &mut num;
                    (*io).value.integer = argp.arg::<i32>() as i64;
                    (*io).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                    buff_fs.add_number(&mut num);
                }
                73 => {
                    let mut num_0: TValue = TValue {
                        value: Value {
                            object: std::ptr::null_mut(),
                        },
                        tag: 0,
                    };
                    let io_0: *mut TValue = &mut num_0;
                    (*io_0).value.integer = argp.arg::<i64>();
                    (*io_0).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                    buff_fs.add_number(&mut num_0);
                }
                102 => {
                    let mut num_1: TValue = TValue {
                        value: Value {
                            object: std::ptr::null_mut(),
                        },
                        tag: 0,
                    };
                    let io_1: *mut TValue = &mut num_1;
                    (*io_1).value.number = argp.arg::<f64>();
                    (*io_1).set_tag(TAG_VARIANT_NUMERIC_NUMBER);
                    buff_fs.add_number(&mut num_1);
                }
                112 => {
                    let size = (3 as u64)
                        .wrapping_mul(::core::mem::size_of::<*mut libc::c_void>() as u64)
                        .wrapping_add(8 as u64);
                    let bf: *mut i8 = buff_fs.get_raw(size);
                    let p: *mut libc::c_void = argp.arg::<*mut libc::c_void>();
                    let length =
                        snprintf(bf, size as u64, b"%p\0" as *const u8 as *const i8, p) as u64;
                    buff_fs.add_length(length);
                }
                85 => {
                    let mut bf_0: [i8; 8] = [0; 8];
                    let length_0: i32 = luao_utf8esc(bf_0.as_mut_ptr(), argp.arg::<i64>() as u64);
                    buff_fs.add_string(
                        bf_0.as_mut_ptr()
                            .offset(8 as isize)
                            .offset(-(length_0 as isize)),
                        length_0 as u64,
                    );
                }
                37 => {
                    buff_fs.add_string(b"%\0" as *const u8 as *const i8, 1 as u64);
                }
                _ => {
                    luag_runerror(
                        state,
                        b"invalid option '%%%c' to 'lua_pushfstring'\0" as *const u8 as *const i8,
                        *e.offset(1 as isize) as i32,
                    );
                }
            }
            fmt = e.offset(2 as isize);
        }
        buff_fs.add_string(fmt, strlen(fmt));
        buff_fs.clear();
        return (*((*(*state).top.p.offset(-(1 as isize))).tvalue.value.object as *mut TString))
            .get_contents();
    }
}
pub unsafe extern "C" fn luao_pushfstring(
    state: *mut State,
    fmt: *const i8,
    args: ...
) -> *const i8 {
    unsafe {
        let message: *const i8;
        let mut argp: ::core::ffi::VaListImpl;
        argp = args.clone();
        message = luao_pushvfstring(state, fmt, argp.as_va_list());
        return message;
    }
}
pub unsafe extern "C" fn luat_init(state: *mut State) {
    unsafe {
        static mut EVENT_NAMES: [*const i8; 25] = [
            b"__index\0" as *const u8 as *const i8,
            b"__newindex\0" as *const u8 as *const i8,
            b"__gc\0" as *const u8 as *const i8,
            b"__mode\0" as *const u8 as *const i8,
            b"__len\0" as *const u8 as *const i8,
            b"__eq\0" as *const u8 as *const i8,
            b"__add\0" as *const u8 as *const i8,
            b"__sub\0" as *const u8 as *const i8,
            b"__mul\0" as *const u8 as *const i8,
            b"__mod\0" as *const u8 as *const i8,
            b"__pow\0" as *const u8 as *const i8,
            b"__div\0" as *const u8 as *const i8,
            b"__idiv\0" as *const u8 as *const i8,
            b"__band\0" as *const u8 as *const i8,
            b"__bor\0" as *const u8 as *const i8,
            b"__bxor\0" as *const u8 as *const i8,
            b"__shl\0" as *const u8 as *const i8,
            b"__shr\0" as *const u8 as *const i8,
            b"__unm\0" as *const u8 as *const i8,
            b"__bnot\0" as *const u8 as *const i8,
            b"__lt\0" as *const u8 as *const i8,
            b"__le\0" as *const u8 as *const i8,
            b"__concat\0" as *const u8 as *const i8,
            b"__call\0" as *const u8 as *const i8,
            b"__close\0" as *const u8 as *const i8,
        ];
        let mut i: i32;
        i = 0;
        while i < TM_N as i32 {
            (*(*state).global).tm_name[i as usize] = luas_new(state, EVENT_NAMES[i as usize]);
            luac_fix(
                state,
                &mut (*(*((*(*state).global).tm_name).as_mut_ptr().offset(i as isize)
                    as *mut Object))
            );
            i += 1;
        }
    }
}
pub unsafe extern "C" fn luat_gettmbyobj(
    state: *mut State,
    o: *const TValue,
    event: u32,
) -> *const TValue {
    unsafe {
        let mt: *mut Table;
        match get_tag_type((*o).get_tag()) {
            5 => {
                mt = (*((*o).value.object as *mut Table)).metatable;
            }
            7 => {
                mt = (*((*o).value.object as *mut User)).metatable;
            }
            _ => {
                mt = (*(*state).global).metatable[(get_tag_type((*o).get_tag())) as usize];
            }
        }
        return if mt.is_null() {
            &mut (*(*state).global).nil_value as *mut TValue as *const TValue
        } else {
            luah_getshortstr(mt, (*(*state).global).tm_name[event as usize])
        };
    }
}
pub unsafe extern "C" fn luat_objtypename(state: *mut State, o: *const TValue) -> *const i8 {
    unsafe {
        let mut mt: *mut Table;
        if (*o).get_tag_variant() == TAG_VARIANT_TABLE && {
            mt = (*((*o).value.object as *mut Table)).metatable;
            !mt.is_null()
        } || (*o).get_tag_variant() == TAG_VARIANT_USER && {
            mt = (*((*o).value.object as *mut User)).metatable;
            !mt.is_null()
        } {
            let name: *const TValue =
                luah_getshortstr(mt, luas_new(state, b"__name\0" as *const u8 as *const i8));
            if get_tag_type((*name).get_tag()) == TAG_TYPE_STRING {
                return (*((*name).value.object as *mut TString)).get_contents();
            }
        }
        return TYPE_NAMES[(((*o).get_tag_type()) + 1) as usize];
    }
}
pub unsafe extern "C" fn luat_calltm(
    state: *mut State,
    f: *const TValue,
    p1: *const TValue,
    p2: *const TValue,
    p3: *const TValue,
) {
    unsafe {
        let function: StackValuePointer = (*state).top.p;
        let io1: *mut TValue = &mut (*function).tvalue;
        let io2: *const TValue = f;
        (*io1).value = (*io2).value;
        (*io1).set_tag((*io2).get_tag());
        let io1_0: *mut TValue = &mut (*function.offset(1 as isize)).tvalue;
        let io2_0: *const TValue = p1;
        (*io1_0).value = (*io2_0).value;
        (*io1_0).set_tag((*io2_0).get_tag());
        let io1_1: *mut TValue = &mut (*function.offset(2 as isize)).tvalue;
        let io2_1: *const TValue = p2;
        (*io1_1).value = (*io2_1).value;
        (*io1_1).set_tag((*io2_1).get_tag());
        let io1_2: *mut TValue = &mut (*function.offset(3 as isize)).tvalue;
        let io2_2: *const TValue = p3;
        (*io1_2).value = (*io2_2).value;
        (*io1_2).set_tag((*io2_2).get_tag());
        (*state).top.p = function.offset(4 as isize);
        if (*(*state).call_info).call_status as i32 & (1 << 1 | 1 << 3) == 0 {
            ccall(state, function, 0, 1);
        } else {
            luad_callnoyield(state, function, 0);
        };
    }
}
pub unsafe extern "C" fn luat_calltmres(
    state: *mut State,
    f: *const TValue,
    p1: *const TValue,
    p2: *const TValue,
    mut res: StackValuePointer,
) {
    unsafe {
        let result: i64 = (res as *mut i8).offset_from((*state).stack.p as *mut i8) as i64;
        let function: StackValuePointer = (*state).top.p;
        let io1: *mut TValue = &mut (*function).tvalue;
        let io2: *const TValue = f;
        (*io1).value = (*io2).value;
        (*io1).set_tag((*io2).get_tag());
        let io1_0: *mut TValue = &mut (*function.offset(1 as isize)).tvalue;
        let io2_0: *const TValue = p1;
        (*io1_0).value = (*io2_0).value;
        (*io1_0).set_tag((*io2_0).get_tag());
        let io1_1: *mut TValue = &mut (*function.offset(2 as isize)).tvalue;
        let io2_1: *const TValue = p2;
        (*io1_1).value = (*io2_1).value;
        (*io1_1).set_tag((*io2_1).get_tag());
        (*state).top.p = (*state).top.p.offset(3 as isize);
        if (*(*state).call_info).call_status as i32 & (1 << 1 | 1 << 3) == 0 {
            ccall(state, function, 1, 1);
        } else {
            luad_callnoyield(state, function, 1);
        }
        res = ((*state).stack.p as *mut i8).offset(result as isize) as StackValuePointer;
        let io1_2: *mut TValue = &mut (*res).tvalue;
        (*state).top.p = (*state).top.p.offset(-1);
        let io2_2: *const TValue = &mut (*(*state).top.p).tvalue;
        (*io1_2).value = (*io2_2).value;
        (*io1_2).set_tag((*io2_2).get_tag());
    }
}
pub unsafe extern "C" fn callbintm(
    state: *mut State,
    p1: *const TValue,
    p2: *const TValue,
    res: StackValuePointer,
    event: u32,
) -> i32 {
    unsafe {
        let mut tm: *const TValue = luat_gettmbyobj(state, p1, event);
        if get_tag_type((*tm).get_tag()) == TAG_TYPE_NIL {
            tm = luat_gettmbyobj(state, p2, event);
        }
        if get_tag_type((*tm).get_tag()) == TAG_TYPE_NIL {
            return 0;
        }
        luat_calltmres(state, tm, p1, p2, res);
        return 1;
    }
}
pub unsafe extern "C" fn luat_trybintm(
    state: *mut State,
    p1: *const TValue,
    p2: *const TValue,
    res: StackValuePointer,
    event: u32,
) {
    unsafe {
        if ((callbintm(state, p1, p2, res, event) == 0) as i32 != 0) as i64 != 0 {
            match event as u32 {
                13 | 14 | 15 | 16 | 17 | 19 => {
                    if get_tag_type((*p1).get_tag()) == TAG_TYPE_NUMERIC
                        && get_tag_type((*p2).get_tag()) == TAG_TYPE_NUMERIC
                    {
                        luag_tointerror(state, p1, p2);
                    } else {
                        luag_opinterror(
                            state,
                            p1,
                            p2,
                            b"perform bitwise operation on\0" as *const u8 as *const i8,
                        );
                    }
                }
                _ => {
                    luag_opinterror(
                        state,
                        p1,
                        p2,
                        b"perform arithmetic on\0" as *const u8 as *const i8,
                    );
                }
            }
        }
    }
}
pub unsafe extern "C" fn luat_tryconcattm(state: *mut State) {
    unsafe {
        let top: StackValuePointer = (*state).top.p;
        if ((callbintm(
            state,
            &mut (*top.offset(-(2 as isize))).tvalue,
            &mut (*top.offset(-(1 as isize))).tvalue,
            top.offset(-(2 as isize)),
            TM_CONCAT,
        ) == 0) as i32
            != 0) as i64
            != 0
        {
            luag_concaterror(
                state,
                &mut (*top.offset(-(2 as isize))).tvalue,
                &mut (*top.offset(-(1 as isize))).tvalue,
            );
        }
    }
}
pub unsafe extern "C" fn luat_trybinassoctm(
    state: *mut State,
    p1: *const TValue,
    p2: *const TValue,
    flip: i32,
    res: StackValuePointer,
    event: u32,
) {
    unsafe {
        if flip != 0 {
            luat_trybintm(state, p2, p1, res, event);
        } else {
            luat_trybintm(state, p1, p2, res, event);
        };
    }
}
pub unsafe extern "C" fn luat_trybinitm(
    state: *mut State,
    p1: *const TValue,
    i2: i64,
    flip: i32,
    res: StackValuePointer,
    event: u32,
) {
    unsafe {
        let mut aux: TValue = TValue {
            value: Value {
                object: std::ptr::null_mut(),
            },
            tag: 0,
        };
        let io: *mut TValue = &mut aux;
        (*io).value.integer = i2;
        (*io).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
        luat_trybinassoctm(state, p1, &mut aux, flip, res, event);
    }
}
pub unsafe extern "C" fn luat_callordertm(
    state: *mut State,
    p1: *const TValue,
    p2: *const TValue,
    event: u32,
) -> i32 {
    unsafe {
        if callbintm(state, p1, p2, (*state).top.p, event) != 0 {
            return !((*(*state).top.p).tvalue.get_tag() == TAG_VARIANT_BOOLEAN_FALSE
                || get_tag_type((*(*state).top.p).tvalue.get_tag()) == TAG_TYPE_NIL)
                as i32;
        }
        luag_ordererror(state, p1, p2);
    }
}
pub unsafe extern "C" fn luat_callorderitm(
    state: *mut State,
    mut p1: *const TValue,
    v2: i32,
    flip: i32,
    is_float: bool,
    event: u32,
) -> i32 {
    unsafe {
        let mut aux: TValue = TValue {
            value: Value {
                object: std::ptr::null_mut(),
            },
            tag: 0,
        };
        let p2: *const TValue;
        if is_float {
            let io: *mut TValue = &mut aux;
            (*io).value.number = v2 as f64;
            (*io).set_tag(TAG_VARIANT_NUMERIC_NUMBER);
        } else {
            let io_0: *mut TValue = &mut aux;
            (*io_0).value.integer = v2 as i64;
            (*io_0).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
        }
        if flip != 0 {
            p2 = p1;
            p1 = &mut aux;
        } else {
            p2 = &mut aux;
        }
        return luat_callordertm(state, p1, p2, event);
    }
}
pub unsafe extern "C" fn luat_adjustvarargs(
    state: *mut State,
    nfixparams: i32,
    call_info: *mut CallInfo,
    p: *const Prototype,
) {
    unsafe {
        let mut i: i32;
        let actual: i32 = ((*state).top.p).offset_from((*call_info).function.p) as i32 - 1;
        let nextra: i32 = actual - nfixparams;
        (*call_info).u.l.count_extra_arguments = nextra;
        if ((((*state).stack_last.p).offset_from((*state).top.p) as i64
            <= ((*p).maximum_stack_size as i32 + 1) as i64) as i32
            != 0) as i64
            != 0
        {
            luad_growstack(state, (*p).maximum_stack_size as i32 + 1, true);
        }
        let fresh12 = (*state).top.p;
        (*state).top.p = (*state).top.p.offset(1);
        let io1: *mut TValue = &mut (*fresh12).tvalue;
        let io2: *const TValue = &mut (*(*call_info).function.p).tvalue;
        (*io1).value = (*io2).value;
        (*io1).set_tag((*io2).get_tag());
        i = 1;
        while i <= nfixparams {
            let fresh13 = (*state).top.p;
            (*state).top.p = (*state).top.p.offset(1);
            let io1_0: *mut TValue = &mut (*fresh13).tvalue;
            let io2_0: *const TValue = &mut (*((*call_info).function.p).offset(i as isize)).tvalue;
            (*io1_0).value = (*io2_0).value;
            (*io1_0).set_tag((*io2_0).get_tag());
            (*((*call_info).function.p).offset(i as isize))
                .tvalue
                .set_tag(TAG_VARIANT_NIL_NIL);
            i += 1;
        }
        (*call_info).function.p = ((*call_info).function.p).offset((actual + 1) as isize);
        (*call_info).top.p = ((*call_info).top.p).offset((actual + 1) as isize);
    }
}
pub unsafe extern "C" fn luat_getvarargs(
    state: *mut State,
    call_info: *mut CallInfo,
    mut where_0: StackValuePointer,
    mut wanted: i32,
) {
    unsafe {
        let mut i: i32;
        let nextra: i32 = (*call_info).u.l.count_extra_arguments;
        if wanted < 0 {
            wanted = nextra;
            if ((((*state).stack_last.p).offset_from((*state).top.p) as i64 <= nextra as i64)
                as i32
                != 0) as i64
                != 0
            {
                let t__: i64 = (where_0 as *mut i8).offset_from((*state).stack.p as *mut i8) as i64;
                if (*(*state).global).gc_debt > 0 {
                    luac_step(state);
                }
                luad_growstack(state, nextra, true);
                where_0 = ((*state).stack.p as *mut i8).offset(t__ as isize) as StackValuePointer;
            }
            (*state).top.p = where_0.offset(nextra as isize);
        }
        i = 0;
        while i < wanted && i < nextra {
            let io1: *mut TValue = &mut (*where_0.offset(i as isize)).tvalue;
            let io2: *const TValue = &mut (*((*call_info).function.p)
                .offset(-(nextra as isize))
                .offset(i as isize))
            .tvalue;
            (*io1).value = (*io2).value;
            (*io1).set_tag((*io2).get_tag());
            i += 1;
        }
        while i < wanted {
            (*where_0.offset(i as isize))
                .tvalue
                .set_tag(TAG_VARIANT_NIL_NIL);
            i += 1;
        }
    }
}
pub unsafe extern "C" fn luac_newobjdt(
    state: *mut State,
    tag: u8,
    size: u64,
    offset: u64,
) -> *mut Object {
    unsafe {
        let g: *mut Global = (*state).global;
        let p: *mut i8 = luam_malloc_(state, size) as *mut i8;
        let o: *mut Object = p.offset(offset as isize) as *mut Object;
        (*o).set_marked((*g).current_white & (1 << 3 | 1 << 4));
        (*o).set_tag(tag);
        (*o).next = (*g).all_gc;
        (*g).all_gc = o;
        return o;
    }
}
pub unsafe extern "C" fn luac_newobj(state: *mut State, tag: u8, size: u64) -> *mut Object {
    unsafe {
        return luac_newobjdt(state, tag, size, 0u64);
    }
}
pub unsafe extern "C" fn traverse_state(g: *mut Global, state: *mut State) -> i32 {
    unsafe {
        let mut o: StackValuePointer = (*state).stack.p;
        if (*state).get_marked() & 7 > 1 || (*g).gc_state as i32 == 0 {
            linkgclist_(
                &mut (*(state as *mut Object)),
                &mut (*state).gc_list,
                &mut (*g).gray_again,
            );
        }
        if o.is_null() {
            return 1;
        }
        while o < (*state).top.p {
            if ((*o).tvalue.is_collectable())
                && (*(*o).tvalue.value.object).get_marked() & (1 << 3 | 1 << 4) != 0
            {
                reallymarkobject(g, (*o).tvalue.value.object);
            }
            o = o.offset(1);
        }
        let mut uv: *mut UpValue = (*state).open_upvalue;
        while !uv.is_null() {
            if (*uv).get_marked() & (1 << 3 | 1 << 4) != 0 {
                reallymarkobject(g, &mut (*(uv as *mut Object)));
            }
            uv = (*uv).u.open.next;
        }
        if (*g).gc_state as i32 == 2 {
            if !(*g).is_emergency {
                (*state).luad_shrinkstack();
            }
            o = (*state).top.p;
            while o < ((*state).stack_last.p).offset(5 as isize) {
                (*o).tvalue.set_tag(TAG_VARIANT_NIL_NIL);
                o = o.offset(1);
            }
            if !((*state).twups != state) && !((*state).open_upvalue).is_null() {
                (*state).twups = (*g).twups;
                (*g).twups = state;
            }
        }
        return 1 + ((*state).stack_last.p).offset_from((*state).stack.p) as i32;
    }
}
pub unsafe extern "C" fn sweeptolive(
    state: *mut State,
    mut p: *mut *mut Object,
) -> *mut *mut Object {
    unsafe {
        let old: *mut *mut Object = p;
        loop {
            p = (*state).sweep_list(p, 1, std::ptr::null_mut());
            if !(p == old) {
                break;
            }
        }
        return p;
    }
}
pub unsafe extern "C" fn check_sizes(state: *mut State, g: *mut Global) {
    unsafe {
        if !(*g).is_emergency {
            if (*g).string_table.length < (*g).string_table.size / 4 {
                let olddebt: i64 = (*g).gc_debt;
                luas_resize(state, (*g).string_table.size / 2);
                (*g).gc_estimate = ((*g).gc_estimate as u64)
                    .wrapping_add(((*g).gc_debt - olddebt) as u64)
                    as u64;
            }
        }
    }
}
pub unsafe extern "C" fn dothecall(state: *mut State, mut _ud: *mut libc::c_void) {
    unsafe {
        luad_callnoyield(state, (*state).top.p.offset(-(2 as isize)), 0);
    }
}
pub unsafe extern "C" fn gctm_function(state: *mut State) {
    unsafe {
        let g: *mut Global = (*state).global;
        let tm: *const TValue;
        let mut v: TValue = TValue {
            value: Value {
                object: std::ptr::null_mut(),
            },
            tag: 0,
        };
        let io: *mut TValue = &mut v;
        let i_g: *mut Object = udata2finalize(g);
        (*io).value.object = i_g;
        (*io).set_tag((*i_g).get_tag());
        (*io).set_collectable();
        tm = luat_gettmbyobj(state, &mut v, TM_GC);
        if !(get_tag_type((*tm).get_tag()) == TAG_TYPE_NIL) {
            let status: i32;
            let oldah: u8 = (*state).allow_hook;
            let oldgcstp: i32 = (*g).gc_step as i32;
            (*g).gc_step = ((*g).gc_step as i32 | 2) as u8;
            (*state).allow_hook = 0;
            let fresh15 = (*state).top.p;
            (*state).top.p = (*state).top.p.offset(1);
            let io1: *mut TValue = &mut (*fresh15).tvalue;
            let io2: *const TValue = tm;
            (*io1).value = (*io2).value;
            (*io1).set_tag((*io2).get_tag());
            let fresh16 = (*state).top.p;
            (*state).top.p = (*state).top.p.offset(1);
            let io1_0: *mut TValue = &mut (*fresh16).tvalue;
            let io2_0: *const TValue = &mut v;
            (*io1_0).value = (*io2_0).value;
            (*io1_0).set_tag((*io2_0).get_tag());
            (*(*state).call_info).call_status =
                ((*(*state).call_info).call_status as i32 | 1 << 7) as u16;
            status = luad_pcall(
                state,
                Some(dothecall as unsafe extern "C" fn(*mut State, *mut libc::c_void) -> ()),
                std::ptr::null_mut(),
                ((*state).top.p.offset(-(2 as isize)) as *mut i8)
                    .offset_from((*state).stack.p as *mut i8) as i64,
                0,
            );
            (*(*state).call_info).call_status =
                ((*(*state).call_info).call_status as i32 & !(1 << 7)) as u16;
            (*state).allow_hook = oldah;
            (*g).gc_step = oldgcstp as u8;
            if ((status != 0) as i32 != 0) as i64 != 0 {
                luae_warnerror(state, b"__gc\0" as *const u8 as *const i8);
                (*state).top.p = (*state).top.p.offset(-1);
            }
        }
    }
}
pub unsafe extern "C" fn runafewfinalizers(state: *mut State, n: i32) -> i32 {
    unsafe {
        let g: *mut Global = (*state).global;
        let mut i: i32;
        i = 0;
        while i < n && !((*g).to_be_finalized).is_null() {
            gctm_function(state);
            i += 1;
        }
        return i;
    }
}
pub unsafe extern "C" fn callallpendingfinalizers(state: *mut State) {
    unsafe {
        let g: *mut Global = (*state).global;
        while !((*g).to_be_finalized).is_null() {
            gctm_function(state);
        }
    }
}
pub unsafe extern "C" fn luac_checkfinalizer(state: *mut State, o: *mut Object, mt: *mut Table) {
    unsafe {
        let g: *mut Global = (*state).global;
        if (*o).get_marked() & 1 << 6 != 0
            || (if mt.is_null() {
                std::ptr::null()
            } else {
                if (*mt).flags as u32 & (1 as u32) << TM_GC as i32 != 0 {
                    std::ptr::null()
                } else {
                    luat_gettm(mt, TM_GC, (*g).tm_name[TM_GC as usize])
                }
            })
            .is_null()
            || (*g).gc_step as i32 & 4 != 0
        {
            return;
        } else {
            if 3 <= (*g).gc_state as i32 && (*g).gc_state as i32 <= 6 {
                (*o).set_marked(
                    (*o).get_marked() & !(1 << 5 | (1 << 3 | 1 << 4))
                        | ((*g).current_white & (1 << 3 | 1 << 4)),
                );
                if (*g).sweep_gc == &mut (*o).next as *mut *mut Object {
                    (*g).sweep_gc = sweeptolive(state, (*g).sweep_gc);
                }
            } else {
                correctpointers(g, o);
            }
            let mut p: *mut *mut Object = &mut (*g).all_gc;
            while *p != o {
                p = &mut (**p).next;
            }
            *p = (*o).next;
            (*o).next = (*g).finalized_objects;
            (*g).finalized_objects = o;
            (*o).set_marked(((*o).get_marked() | 1 << 6) as u8);
        };
    }
}
pub unsafe extern "C" fn sweep2old(state: *mut State, mut p: *mut *mut Object) {
    unsafe {
        let g: *mut Global = (*state).global;
        loop {
            let curr: *mut Object = *p;
            if curr.is_null() {
                break;
            }
            if (*curr).get_marked() & (1 << 3 | 1 << 4) != 0 {
                *p = (*curr).next;
                freeobj(state, curr);
            } else {
                (*curr).set_marked((*curr).get_marked() & !(7) | 4);
                if (*curr).get_tag() == TAG_TYPE_STATE {
                    let other_state: *mut State = &mut (*(curr as *mut State));
                    linkgclist_(
                        &mut (*(other_state as *mut Object)),
                        &mut (*other_state).gc_list,
                        &mut (*g).gray_again,
                    );
                } else if (*curr).get_tag() == TAG_TYPE_UPVALUE
                    && (*(curr as *mut UpValue)).v.p
                        != &mut (*(curr as *mut UpValue)).u.value as *mut TValue
                {
                    (*curr).set_marked((*curr).get_marked() & !(1 << 5 | (1 << 3 | 1 << 4)));
                } else {
                    (*curr).set_marked((*curr).get_marked() | 1 << 5);
                }
                p = &mut (*curr).next;
            }
        }
    }
}
pub unsafe extern "C" fn sweepgen(
    state: *mut State,
    g: *mut Global,
    mut p: *mut *mut Object,
    limit: *mut Object,
    pfirstold1: *mut *mut Object,
) -> *mut *mut Object {
    unsafe {
        static mut NEXT_AGE: [u8; 7] = [1, 3 as u8, 3 as u8, 4 as u8, 4 as u8, 5 as u8, 6 as u8];
        let white = (*g).current_white & (1 << 3 | 1 << 4);
        loop {
            let curr: *mut Object = *p;
            if !(curr != limit) {
                break;
            }
            if (*curr).get_marked() & (1 << 3 | 1 << 4) != 0 {
                *p = (*curr).next;
                freeobj(state, curr);
            } else {
                if (*curr).get_marked() & 7 == 0 {
                    let marked = (*curr).get_marked() & !(1 << 5 | (1 << 3 | 1 << 4) | 7);
                    (*curr).set_marked(marked | 1 | white);
                } else {
                    (*curr).set_marked(
                        (*curr).get_marked() & !(7) | NEXT_AGE[((*curr).get_marked() & 7) as usize],
                    );
                    if (*curr).get_marked() & 7 == 3 && (*pfirstold1).is_null() {
                        *pfirstold1 = curr;
                    }
                }
                p = &mut (*curr).next;
            }
        }
        return p;
    }
}
pub unsafe extern "C" fn finishgencycle(state: *mut State, g: *mut Global) {
    unsafe {
        correctgraylists(g);
        check_sizes(state, g);
        (*g).gc_state = 0;
        if !(*g).is_emergency {
            callallpendingfinalizers(state);
        }
    }
}
pub unsafe extern "C" fn youngcollection(state: *mut State, g: *mut Global) {
    unsafe {
        if !((*g).first_old1).is_null() {
            markold(g, (*g).first_old1, (*g).really_old);
            (*g).first_old1 = std::ptr::null_mut();
        }
        markold(g, (*g).finalized_objects, (*g).finobjrold);
        markold(g, (*g).to_be_finalized, std::ptr::null_mut());
        atomic(state);
        (*g).gc_state = 3 as u8;
        let mut psurvival: *mut *mut Object = sweepgen(
            state,
            g,
            &mut (*g).all_gc,
            (*g).survival,
            &mut (*g).first_old1,
        );
        sweepgen(state, g, psurvival, (*g).old1, &mut (*g).first_old1);
        (*g).really_old = (*g).old1;
        (*g).old1 = *psurvival;
        (*g).survival = (*g).all_gc;
        let mut dummy: *mut Object = std::ptr::null_mut();
        psurvival = sweepgen(state, g, &mut (*g).finalized_objects, (*g).finobjsur, &mut dummy);
        sweepgen(state, g, psurvival, (*g).finobjold1, &mut dummy);
        (*g).finobjrold = (*g).finobjold1;
        (*g).finobjold1 = *psurvival;
        (*g).finobjsur = (*g).finalized_objects;
        sweepgen(
            state,
            g,
            &mut (*g).to_be_finalized,
            std::ptr::null_mut(),
            &mut dummy,
        );
        finishgencycle(state, g);
    }
}
pub unsafe extern "C" fn atomic2gen(state: *mut State, g: *mut Global) {
    unsafe {
        cleargraylists(g);
        (*g).gc_state = 3 as u8;
        sweep2old(state, &mut (*g).all_gc);
        (*g).survival = (*g).all_gc;
        (*g).old1 = (*g).survival;
        (*g).really_old = (*g).old1;
        (*g).first_old1 = std::ptr::null_mut();
        sweep2old(state, &mut (*g).finalized_objects);
        (*g).finobjsur = (*g).finalized_objects;
        (*g).finobjold1 = (*g).finobjsur;
        (*g).finobjrold = (*g).finobjold1;
        sweep2old(state, &mut (*g).to_be_finalized);
        (*g).gc_kind = 1;
        (*g).last_atomic = 0;
        (*g).gc_estimate = ((*g).total_bytes + (*g).gc_debt) as u64;
        finishgencycle(state, g);
    }
}
pub unsafe extern "C" fn entergen(state: *mut State, g: *mut Global) -> u64 {
    unsafe {
        luac_runtilstate(state, 1 << 8);
        luac_runtilstate(state, 1 << 0);
        let numobjs: u64 = atomic(state);
        atomic2gen(state, g);
        (*g).set_minor_debt();
        return numobjs;
    }
}
pub unsafe extern "C" fn luac_changemode(state: *mut State, newmode: i32) {
    unsafe {
        let g: *mut Global = (*state).global;
        if newmode != (*g).gc_kind as i32 {
            if newmode == 1 {
                entergen(state, g);
            } else {
                (*g).enter_incremental();
            }
        }
        (*g).last_atomic = 0;
    }
}
pub unsafe extern "C" fn fullgen(state: *mut State, g: *mut Global) -> u64 {
    unsafe {
        (*g).enter_incremental();
        return entergen(state, g);
    }
}
pub unsafe extern "C" fn stepgenfull(state: *mut State, g: *mut Global) {
    unsafe {
        let lastatomic: u64 = (*g).last_atomic;
        if (*g).gc_kind as i32 == 1 {
            (*g).enter_incremental();
        }
        luac_runtilstate(state, 1 << 0);
        let newatomic: u64 = atomic(state);
        if newatomic < lastatomic.wrapping_add(lastatomic >> 3) {
            atomic2gen(state, g);
            (*g).set_minor_debt();
        } else {
            (*g).gc_estimate = ((*g).total_bytes + (*g).gc_debt) as u64;
            entersweep(state);
            luac_runtilstate(state, 1 << 8);
            setpause(g);
            (*g).last_atomic = newatomic;
        };
    }
}
pub unsafe extern "C" fn genstep(state: *mut State, g: *mut Global) {
    unsafe {
        if (*g).last_atomic != 0u64 {
            stepgenfull(state, g);
        } else {
            let majorbase: u64 = (*g).gc_estimate;
            let majorinc: u64 = majorbase
                .wrapping_div(100 as u64)
                .wrapping_mul((*g).generational_major_multiplier * 4);
            if (*g).gc_debt > 0
                && ((*g).total_bytes + (*g).gc_debt) as u64 > majorbase.wrapping_add(majorinc)
            {
                let numobjs: u64 = fullgen(state, g);
                if !((((*g).total_bytes + (*g).gc_debt) as u64)
                    < majorbase.wrapping_add(majorinc.wrapping_div(2 as u64)))
                {
                    (*g).last_atomic = numobjs;
                    setpause(g);
                }
            } else {
                youngcollection(state, g);
                (*g).set_minor_debt();
                (*g).gc_estimate = majorbase;
            }
        };
    }
}
pub unsafe extern "C" fn entersweep(state: *mut State) {
    unsafe {
        let g: *mut Global = (*state).global;
        (*g).gc_state = 3 as u8;
        (*g).sweep_gc = sweeptolive(state, &mut (*g).all_gc);
    }
}
pub unsafe extern "C" fn luac_freeallobjects(state: *mut State) {
    unsafe {
        let g: *mut Global = (*state).global;
        (*g).gc_step = 4 as u8;
        luac_changemode(state, 0);
        separatetobefnz(g, 1);
        callallpendingfinalizers(state);
        deletelist(
            state,
            (*g).all_gc,
            &mut (*((*g).main_state as *mut Object)),
        );
        deletelist(state, (*g).fixed_gc, std::ptr::null_mut());
    }
}
pub unsafe extern "C" fn atomic(state: *mut State) -> u64 {
    unsafe {
        let g: *mut Global = (*state).global;
        let mut work: u64 = 0;
        let grayagain: *mut Object = (*g).gray_again;
        (*g).gray_again = std::ptr::null_mut();
        (*g).gc_state = 2 as u8;
        if (*state).get_marked() & (1 << 3 | 1 << 4) != 0 {
            reallymarkobject(g, &mut (*(state as *mut Object)));
        }
        if ((*g).l_registry.is_collectable())
            && (*(*g).l_registry.value.object).get_marked() & (1 << 3 | 1 << 4) != 0
        {
            reallymarkobject(g, (*g).l_registry.value.object);
        }
        markmt(g);
        work = (work as u64).wrapping_add(propagateall(g)) as u64;
        work = (work as u64).wrapping_add(remarkupvals(g) as u64) as u64;
        work = (work as u64).wrapping_add(propagateall(g)) as u64;
        (*g).gray = grayagain;
        work = (work as u64).wrapping_add(propagateall(g)) as u64;
        convergeephemerons(g);
        clearbyvalues(g, (*g).weak, std::ptr::null_mut());
        clearbyvalues(g, (*g).all_weak, std::ptr::null_mut());
        let origweak: *mut Object = (*g).weak;
        let origall: *mut Object = (*g).all_weak;
        separatetobefnz(g, 0);
        work = (work as u64).wrapping_add(markbeingfnz(g)) as u64;
        work = (work as u64).wrapping_add(propagateall(g)) as u64;
        convergeephemerons(g);
        clearbykeys(g, (*g).ephemeron);
        clearbykeys(g, (*g).all_weak);
        clearbyvalues(g, (*g).weak, origweak);
        clearbyvalues(g, (*g).all_weak, origall);
        (*g).clear_cache();
        (*g).current_white = ((*g).current_white as i32 ^ (1 << 3 | 1 << 4)) as u8;
        return work;
    }
}
pub unsafe extern "C" fn sweepstep(
    state: *mut State,
    g: *mut Global,
    nextstate: i32,
    nextlist: *mut *mut Object,
) -> i32 {
    unsafe {
        if !((*g).sweep_gc).is_null() {
            let olddebt: i64 = (*g).gc_debt;
            let mut count: i32 = 0;
            (*g).sweep_gc = (*state).sweep_list((*g).sweep_gc, 100 as i32, &mut count);
            (*g).gc_estimate = ((*g).gc_estimate as u64)
                .wrapping_add(((*g).gc_debt - olddebt) as u64) as u64
                as u64;
            return count;
        } else {
            (*g).gc_state = nextstate as u8;
            (*g).sweep_gc = nextlist;
            return 0;
        };
    }
}
pub unsafe extern "C" fn singlestep(state: *mut State) -> u64 {
    unsafe {
        let g: *mut Global = (*state).global;
        let work: u64;
        (*g).gcstopem = 1;
        match (*g).gc_state as i32 {
            8 => {
                restartcollection(g);
                (*g).gc_state = 0;
                work = 1 as u64;
            }
            0 => {
                if ((*g).gray).is_null() {
                    (*g).gc_state = 1;
                    work = 0;
                } else {
                    work = (*g).propagatemark();
                }
            }
            1 => {
                work = atomic(state);
                entersweep(state);
                (*g).gc_estimate = ((*g).total_bytes + (*g).gc_debt) as u64;
            }
            3 => {
                work = sweepstep(state, g, 4, &mut (*g).finalized_objects) as u64;
            }
            4 => {
                work = sweepstep(state, g, 5, &mut (*g).to_be_finalized) as u64;
            }
            5 => {
                work = sweepstep(state, g, 6, std::ptr::null_mut()) as u64;
            }
            6 => {
                check_sizes(state, g);
                (*g).gc_state = 7 as u8;
                work = 0;
            }
            7 => {
                if !((*g).to_be_finalized).is_null() && !(*g).is_emergency {
                    (*g).gcstopem = 0;
                    work = (runafewfinalizers(state, 10 as i32) * 50 as i32) as u64;
                } else {
                    (*g).gc_state = 8 as u8;
                    work = 0;
                }
            }
            _ => return 0u64,
        }
        (*g).gcstopem = 0;
        return work;
    }
}
pub unsafe extern "C" fn luac_runtilstate(state: *mut State, statesmask: i32) {
    unsafe {
        let g: *mut Global = (*state).global;
        while statesmask & 1 << (*g).gc_state as i32 == 0 {
            singlestep(state);
        }
    }
}
pub unsafe extern "C" fn incstep(state: *mut State, g: *mut Global) {
    unsafe {
        let stepmul: i32 = (*g).gc_step_multiplier as i32 * 4 | 1;
        let mut debt: i64 = ((*g).gc_debt as u64)
            .wrapping_div(::core::mem::size_of::<TValue>() as u64)
            .wrapping_mul(stepmul as u64) as i64;
        let stepsize: i64 = (if (*g).gc_step_size as u64
            <= (::core::mem::size_of::<i64>() as u64)
                .wrapping_mul(8 as u64)
                .wrapping_sub(2 as u64)
        {
            ((1 << (*g).gc_step_size as i32) as u64)
                .wrapping_div(::core::mem::size_of::<TValue>() as u64)
                .wrapping_mul(stepmul as u64)
        } else {
            (!(0u64) >> 1) as u64
        }) as i64;
        loop {
            let work: u64 = singlestep(state);
            debt = (debt as u64).wrapping_sub(work) as i64;
            if !(debt > -stepsize && (*g).gc_state as i32 != 8) {
                break;
            }
        }
        if (*g).gc_state as i32 == 8 {
            setpause(g);
        } else {
            debt = ((debt / stepmul as i64) as u64)
                .wrapping_mul(::core::mem::size_of::<TValue>() as u64) as i64;
            (*g).set_debt(debt);
        };
    }
}
pub unsafe extern "C" fn luac_step(state: *mut State) {
    unsafe {
        let g: *mut Global = (*state).global;
        if !((*g).gc_step as i32 == 0) {
            (*g).set_debt(-(2000 as i32) as i64);
        } else if (*g).gc_kind as i32 == 1 || (*g).last_atomic != 0u64 {
            genstep(state, g);
        } else {
            incstep(state, g);
        };
    }
}
pub unsafe extern "C" fn fullinc(state: *mut State, g: *mut Global) {
    unsafe {
        if (*g).gc_state as i32 <= 2 {
            entersweep(state);
        }
        luac_runtilstate(state, 1 << 8);
        luac_runtilstate(state, 1 << 0);
        (*g).gc_state = 1;
        luac_runtilstate(state, 1 << 7);
        luac_runtilstate(state, 1 << 8);
        setpause(g);
    }
}
pub unsafe extern "C" fn luac_fullgc(state: *mut State, is_emergency: bool) {
    unsafe {
        (*((*state).global)).is_emergency = is_emergency;
        if (*((*state).global)).gc_kind as i32 == 0 {
            fullinc(state, (*state).global);
        } else {
            fullgen(state, (*state).global);
        }
        (*((*state).global)).is_emergency = false;
    }
}
pub unsafe extern "C" fn callclosemethod(
    state: *mut State,
    obj: *mut TValue,
    err: *mut TValue,
    yy: i32,
) {
    unsafe {
        let top: StackValuePointer = (*state).top.p;
        let tm: *const TValue = luat_gettmbyobj(state, obj, TM_CLOSE);
        let io1: *mut TValue = &mut (*top).tvalue;
        let io2: *const TValue = tm;
        (*io1).value = (*io2).value;
        (*io1).set_tag((*io2).get_tag());
        let io1_0: *mut TValue = &mut (*top.offset(1 as isize)).tvalue;
        let io2_0: *const TValue = obj;
        (*io1_0).value = (*io2_0).value;
        (*io1_0).set_tag((*io2_0).get_tag());
        let io1_1: *mut TValue = &mut (*top.offset(2 as isize)).tvalue;
        let io2_1: *const TValue = err;
        (*io1_1).value = (*io2_1).value;
        (*io1_1).set_tag((*io2_1).get_tag());
        (*state).top.p = top.offset(3 as isize);
        if yy != 0 {
            ccall(state, top, 0, 1);
        } else {
            luad_callnoyield(state, top, 0);
        };
    }
}
pub unsafe extern "C" fn checkclosemth(state: *mut State, level: StackValuePointer) {
    unsafe {
        let tm: *const TValue = luat_gettmbyobj(state, &mut (*level).tvalue, TM_CLOSE);
        if get_tag_type((*tm).get_tag()) == TAG_TYPE_NIL {
            let index: i32 = level.offset_from((*(*state).call_info).function.p) as i32;
            let mut vname: *const i8 =
                luag_findlocal(state, (*state).call_info, index, std::ptr::null_mut());
            if vname.is_null() {
                vname = b"?\0" as *const u8 as *const i8;
            }
            luag_runerror(
                state,
                b"variable '%s' got a non-closable value\0" as *const u8 as *const i8,
                vname,
            );
        }
    }
}
pub unsafe extern "C" fn prepcallclosemth(state: *mut State, level: StackValuePointer, status: i32, yy: i32) {
    unsafe {
        let uv: *mut TValue = &mut (*level).tvalue;
        let errobj: *mut TValue;
        if status == -1 {
            errobj = &mut (*(*state).global).nil_value;
        } else {
            errobj = &mut (*level.offset(1 as isize)).tvalue;
            (*state).set_error_object(status, level.offset(1 as isize));
        }
        callclosemethod(state, uv, errobj, yy);
    }
}
pub unsafe extern "C" fn luaf_newtbcupval(state: *mut State, level: StackValuePointer) {
    unsafe {
        if (*level).tvalue.get_tag() == TAG_VARIANT_BOOLEAN_FALSE
            || get_tag_type((*level).tvalue.get_tag()) == TAG_TYPE_NIL
        {
            return;
        }
        checkclosemth(state, level);
        while level.offset_from((*state).tbc_list.p) as u64
            > ((256 as u64)
                << (::core::mem::size_of::<u16>() as u64)
                    .wrapping_sub(1 as u64)
                    .wrapping_mul(8 as u64))
            .wrapping_sub(1 as u64)
        {
            (*state).tbc_list.p = ((*state).tbc_list.p).offset(
                ((256 as u64)
                    << (::core::mem::size_of::<u16>() as u64)
                        .wrapping_sub(1 as u64)
                        .wrapping_mul(8 as u64))
                .wrapping_sub(1 as u64) as isize,
            );
            (*(*state).tbc_list.p).delta = 0;
        }
        (*level).delta = level.offset_from((*state).tbc_list.p) as u16;
        (*state).tbc_list.p = level;
    }
}
pub unsafe extern "C" fn luaf_closeupval(state: *mut State, level: StackValuePointer) {
    unsafe {
        loop {
            let uv: *mut UpValue = (*state).open_upvalue;
            let upl: StackValuePointer;
            if !(!uv.is_null() && {
                upl = (*uv).v.p as StackValuePointer;
                upl >= level
            }) {
                break;
            }
            let slot: *mut TValue = &mut (*uv).u.value;
            luaf_unlinkupval(uv);
            let io1: *mut TValue = slot;
            let io2: *const TValue = (*uv).v.p;
            (*io1).value = (*io2).value;
            (*io1).set_tag((*io2).get_tag());
            (*uv).v.p = slot;
            if (*uv).get_marked() & (1 << 3 | 1 << 4) == 0 {
                (*uv).set_marked((*uv).get_marked() | 1 << 5);
                if (*slot).is_collectable() {
                    if (*uv).get_marked() & 1 << 5 != 0
                        && (*(*slot).value.object).get_marked() & (1 << 3 | 1 << 4) != 0
                    {
                        luac_barrier_(
                            state,
                            &mut (*(uv as *mut Object)),
                            &mut (*((*slot).value.object as *mut Object)),
                        );
                    } else {
                    };
                } else {
                };
            }
        }
    }
}
pub unsafe extern "C" fn poptbclist(state: *mut State) {
    unsafe {
        let mut tbc: StackValuePointer = (*state).tbc_list.p;
        tbc = tbc.offset(-((*tbc).delta as isize));
        while tbc > (*state).stack.p && (*tbc).delta == 0 {
            tbc = tbc.offset(
                -(((256 as u64)
                    << (::core::mem::size_of::<u16>() as u64)
                        .wrapping_sub(1 as u64)
                        .wrapping_mul(8 as u64))
                .wrapping_sub(1 as u64) as isize),
            );
        }
        (*state).tbc_list.p = tbc;
    }
}
pub unsafe extern "C" fn luaf_close(
    state: *mut State,
    mut level: StackValuePointer,
    status: i32,
    yy: i32,
) -> StackValuePointer {
    unsafe {
        let levelrel: i64 = (level as *mut i8).offset_from((*state).stack.p as *mut i8) as i64;
        luaf_closeupval(state, level);
        while (*state).tbc_list.p >= level {
            let tbc: StackValuePointer = (*state).tbc_list.p;
            poptbclist(state);
            prepcallclosemth(state, tbc, status, yy);
            level = ((*state).stack.p as *mut i8).offset(levelrel as isize) as StackValuePointer;
        }
        return level;
    }
}
pub unsafe extern "C" fn luay_parser(
    state: *mut State,
    zio: *mut ZIO,
    buffer: *mut Buffer,
    dynamic_data: *mut DynamicData,
    name: *const i8,
    firstchar: i32,
) -> *mut Closure {
    unsafe {
        let mut lexstate: LexicalState = LexicalState::new();
        let mut funcstate: FunctionState = FunctionState::new();
        let cl: *mut Closure = luaf_newlclosure(state, 1);
        let io: *mut TValue = &mut (*(*state).top.p).tvalue;
        let x_: *mut Closure = cl;
        (*io).value.object = &mut (*(x_ as *mut Object));
        (*io).set_tag(TAG_VARIANT_CLOSURE_L);
        (*io).set_collectable();
        (*state).luad_inctop();
        lexstate.table = luah_new(state);
        let io_0: *mut TValue = &mut (*(*state).top.p).tvalue;
        let x0: *mut Table = lexstate.table;
        (*io_0).value.object = &mut (*(x0 as *mut Object));
        (*io_0).set_tag(TAG_VARIANT_TABLE);
        (*io_0).set_collectable();
        (*state).luad_inctop();
        (*cl).payload.l_prototype = luaf_newproto(state);
        funcstate.prototype = (*cl).payload.l_prototype;
        if (*cl).get_marked() & 1 << 5 != 0 && (*(*cl).payload.l_prototype).get_marked() & (1 << 3 | 1 << 4) != 0 {
            luac_barrier_(
                state,
                &mut (*(cl as *mut Object)),
                &mut (*((*cl).payload.l_prototype as *mut Object)),
            );
        } else {
        };
        (*funcstate.prototype).source = luas_new(state, name);
        if (*funcstate.prototype).get_marked() & 1 << 5 != 0
            && (*(*funcstate.prototype).source).get_marked() & (1 << 3 | 1 << 4) != 0
        {
            luac_barrier_(
                state,
                &mut (*(funcstate.prototype as *mut Object)),
                &mut (*((*funcstate.prototype).source as *mut Object)),
            );
        } else {
        };
        lexstate.buffer = buffer;
        lexstate.dynamic_data = dynamic_data;
        (*dynamic_data).label.n = 0;
        (*dynamic_data).gt.n = (*dynamic_data).label.n;
        (*dynamic_data).active_variable.length = (*dynamic_data).gt.n;
        luax_setinput(state, &mut lexstate, zio, (*funcstate.prototype).source, firstchar);
        mainfunc(&mut lexstate, &mut funcstate);
        (*state).top.p = (*state).top.p.offset(-1);
        return cl;
    }
}
pub unsafe extern "C" fn luax_init(state: *mut State) {
    unsafe {
        let mut i: i32;
        let e: *mut TString = luas_newlstr(
            state,
            b"_ENV\0" as *const u8 as *const i8,
            (::core::mem::size_of::<[i8; 5]>() as u64)
                .wrapping_div(::core::mem::size_of::<i8>() as u64)
                .wrapping_sub(1 as u64),
        );
        luac_fix(state, &mut (*(e as *mut Object)));
        i = 0;
        while i < TK_WHILE as i32 - (127 as i32 * 2 + 1 + 1) + 1 {
            let ts: *mut TString = luas_new(state, TOKENS[i as usize]);
            luac_fix(state, &mut (*(ts as *mut Object)));
            (*ts).extra = (i + 1) as u8;
            i += 1;
        }
    }
}
pub unsafe extern "C" fn pushclosure(
    state: *mut State,
    p: *mut Prototype,
    encup: *mut *mut UpValue,
    base: StackValuePointer,
    ra: StackValuePointer,
) {
    unsafe {
        let nup: i32 = (*p).size_upvalues;
        let uv: *mut UpValueDescription = (*p).upvalues;
        let mut i: i32;
        let ncl: *mut Closure = luaf_newlclosure(state, nup);
        (*ncl).payload.l_prototype = p;
        let io: *mut TValue = &mut (*ra).tvalue;
        let x_: *mut Closure = ncl;
        (*io).value.object = &mut (*(x_ as *mut Object));
        (*io).set_tag(TAG_VARIANT_CLOSURE_L);
        (*io).set_collectable();
        i = 0;
        while i < nup {
            if (*uv.offset(i as isize)).is_in_stack {
                let ref mut fresh136 = *((*ncl).upvalues).l_upvalues.as_mut_ptr().offset(i as isize);
                *fresh136 = luaf_findupval(
                    state,
                    base.offset((*uv.offset(i as isize)).index as isize),
                );
            } else {
                let ref mut fresh137 = *((*ncl).upvalues).l_upvalues.as_mut_ptr().offset(i as isize);
                *fresh137 = *encup.offset((*uv.offset(i as isize)).index as isize);
            }
            if (*ncl).get_marked() & 1 << 5 != 0
                && (**((*ncl).upvalues).l_upvalues.as_mut_ptr().offset(i as isize)).get_marked()
                    & (1 << 3 | 1 << 4)
                    != 0
            {
                luac_barrier_(
                    state,
                    &mut (*(ncl as *mut Object)),
                    &mut (*(*((*ncl).upvalues).l_upvalues.as_mut_ptr().offset(i as isize) as *mut Object)),
                );
            } else {
            };
            i += 1;
        }
    }
}
pub unsafe extern "C" fn luav_finishop(state: *mut State) {
    unsafe {
        let call_info: *mut CallInfo = (*state).call_info;
        let base: StackValuePointer = ((*call_info).function.p).offset(1 as isize);
        let inst: u32 = *((*call_info).u.l.saved_program_counter).offset(-(1 as isize));
        let op: u32 = (inst >> 0 & !(!(0u32) << 7) << 0) as u32;
        match op as u32 {
            46 | 47 | 48 => {
                let io1: *mut TValue = &mut (*base.offset(
                    (*((*call_info).u.l.saved_program_counter).offset(-(2 as isize)) >> 0 + 7
                        & !(!(0u32) << 8) << 0) as isize,
                ))
                .tvalue;
                (*state).top.p = (*state).top.p.offset(-1);
                let io2: *const TValue = &mut (*(*state).top.p).tvalue;
                (*io1).value = (*io2).value;
                (*io1).set_tag((*io2).get_tag());
            }
            49 | 50 | 52 | 11 | 12 | 13 | 14 | 20 => {
                let io1_0: *mut TValue = &mut (*base
                    .offset((inst >> 0 + 7 & !(!(0u32) << 8) << 0) as isize))
                .tvalue;
                (*state).top.p = (*state).top.p.offset(-1);
                let io2_0: *const TValue = &mut (*(*state).top.p).tvalue;
                (*io1_0).value = (*io2_0).value;
                (*io1_0).set_tag((*io2_0).get_tag());
            }
            58 | 59 | 62 | 63 | 64 | 65 | 57 => {
                let res: i32 = !((*(*state).top.p.offset(-(1 as isize))).tvalue.get_tag()
                    == TAG_VARIANT_BOOLEAN_FALSE
                    || get_tag_type((*(*state).top.p.offset(-(1 as isize))).tvalue.get_tag())
                        == TAG_TYPE_NIL) as i32;
                (*state).top.p = (*state).top.p.offset(-1);
                if res != (inst >> 0 + 7 + 8 & !(!(0u32) << 1) << 0) as i32 {
                    (*call_info).u.l.saved_program_counter =
                        ((*call_info).u.l.saved_program_counter).offset(1);
                    (*call_info).u.l.saved_program_counter;
                }
            }
            53 => {
                let top: StackValuePointer = (*state).top.p.offset(-(1 as isize));
                let a: i32 = (inst >> 0 + 7 & !(!(0u32) << 8) << 0) as i32;
                let total: i32 =
                    top.offset(-(1 as isize))
                        .offset_from(base.offset(a as isize)) as i32;
                let io1_1: *mut TValue = &mut (*top.offset(-(2 as isize))).tvalue;
                let io2_1: *const TValue = &mut (*top).tvalue;
                (*io1_1).value = (*io2_1).value;
                (*io1_1).set_tag((*io2_1).get_tag());
                (*state).top.p = top.offset(-(1 as isize));
                concatenate(state, total);
            }
            54 => {
                (*call_info).u.l.saved_program_counter =
                    ((*call_info).u.l.saved_program_counter).offset(-1);
                (*call_info).u.l.saved_program_counter;
            }
            70 => {
                let ra: StackValuePointer = base.offset((inst >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                (*state).top.p = ra.offset((*call_info).u2.nres as isize);
                (*call_info).u.l.saved_program_counter =
                    ((*call_info).u.l.saved_program_counter).offset(-1);
                (*call_info).u.l.saved_program_counter;
            }
            _ => {}
        };
    }
}
pub unsafe extern "C" fn luav_execute(state: *mut State, mut call_info: *mut CallInfo) {
    unsafe {
        let mut i: u32;
        let mut ra_65: StackValuePointer;
        let mut new_call_info: *mut CallInfo;
        let mut b_4: i32;
        let mut count_results: i32;
        let mut current_block: u64;
        let mut cl: *mut Closure;
        let mut k: *mut TValue;
        let mut base: StackValuePointer;
        let mut program_counter: *const u32;
        let mut trap: i32;
        '_startfunc: loop {
            trap = (*state).hook_mask;
            '_returning: loop {
                cl = &mut (*((*(*call_info).function.p).tvalue.value.object as *mut Closure));
                k = (*(*cl).payload.l_prototype).k;
                program_counter = (*call_info).u.l.saved_program_counter;
                if (trap != 0) as i64 != 0 {
                    trap = luag_tracecall(state);
                }
                base = ((*call_info).function.p).offset(1 as isize);
                loop {
                    if (trap != 0) as i64 != 0 {
                        trap = luag_traceexec(state, program_counter);
                        base = ((*call_info).function.p).offset(1 as isize);
                    }
                    let fresh138 = program_counter;
                    program_counter = program_counter.offset(1);
                    i = *fresh138;
                    match (i >> 0 & !(!(0u32) << 7) << 0) as u32 {
                        0 => {
                            let ra: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            let io1: *mut TValue = &mut (*ra).tvalue;
                            let io2: *const TValue = &mut (*base.offset(
                                (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            (*io1).value = (*io2).value;
                            (*io1).set_tag((*io2).get_tag());
                            continue;
                        }
                        1 => {
                            let ra_0: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            let b: i64 = ((i >> 0 + 7 + 8 & !(!(0u32) << 8 + 8 + 1) << 0) as i32
                                - ((1 << 8 + 8 + 1) - 1 >> 1))
                                as i64;
                            let io: *mut TValue = &mut (*ra_0).tvalue;
                            (*io).value.integer = b;
                            (*io).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                            continue;
                        }
                        2 => {
                            let ra_1: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            let b_0: i32 = (i >> 0 + 7 + 8 & !(!(0u32) << 8 + 8 + 1) << 0) as i32
                                - ((1 << 8 + 8 + 1) - 1 >> 1);
                            let io_0: *mut TValue = &mut (*ra_1).tvalue;
                            (*io_0).value.number = b_0 as f64;
                            (*io_0).set_tag(TAG_VARIANT_NUMERIC_NUMBER);
                            continue;
                        }
                        3 => {
                            let ra_2: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            let rb: *mut TValue = k.offset(
                                (i >> 0 + 7 + 8 & !(!(0u32) << 8 + 8 + 1) << 0) as isize,
                            );
                            let io1_0: *mut TValue = &mut (*ra_2).tvalue;
                            let io2_0: *const TValue = rb;
                            (*io1_0).value = (*io2_0).value;
                            (*io1_0).set_tag((*io2_0).get_tag());
                            continue;
                        }
                        4 => {
                            let ra_3: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            let rb_0: *mut TValue = k.offset(
                                (*program_counter >> 0 + 7 & !(!(0u32) << 8 + 8 + 1 + 8) << 0)
                                    as isize,
                            );
                            program_counter = program_counter.offset(1);
                            let io1_1: *mut TValue = &mut (*ra_3).tvalue;
                            let io2_1: *const TValue = rb_0;
                            (*io1_1).value = (*io2_1).value;
                            (*io1_1).set_tag((*io2_1).get_tag());
                            continue;
                        }
                        5 => {
                            let ra_4: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            (*ra_4).tvalue.set_tag(TAG_VARIANT_BOOLEAN_FALSE);
                            continue;
                        }
                        6 => {
                            let ra_5: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            (*ra_5).tvalue.set_tag(TAG_VARIANT_BOOLEAN_FALSE);
                            program_counter = program_counter.offset(1);
                            continue;
                        }
                        7 => {
                            let ra_6: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            (*ra_6).tvalue.set_tag(TAG_VARIANT_BOOLEAN_TRUE);
                            continue;
                        }
                        8 => {
                            let mut ra_7: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            let mut b_1: i32 = (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32;
                            loop {
                                let fresh139 = ra_7;
                                ra_7 = ra_7.offset(1);
                                (*fresh139).tvalue.set_tag(TAG_VARIANT_NIL_NIL);
                                let fresh140 = b_1;
                                b_1 = b_1 - 1;
                                if !(fresh140 != 0) {
                                    break;
                                }
                            }
                            continue;
                        }
                        9 => {
                            let ra_8: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            let b_2: i32 = (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32;
                            let io1_2: *mut TValue = &mut (*ra_8).tvalue;
                            let io2_2: *const TValue =
                                (**((*cl).upvalues).l_upvalues.as_mut_ptr().offset(b_2 as isize)).v.p;
                            (*io1_2).value = (*io2_2).value;
                            (*io1_2).set_tag((*io2_2).get_tag());
                            continue;
                        }
                        10 => {
                            let ra_9: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            let uv: *mut UpValue = *((*cl).upvalues).l_upvalues.as_mut_ptr().offset(
                                (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as isize,
                            );
                            let io1_3: *mut TValue = (*uv).v.p;
                            let io2_3: *const TValue = &mut (*ra_9).tvalue;
                            (*io1_3).value = (*io2_3).value;
                            (*io1_3).set_tag((*io2_3).get_tag());
                            if (*ra_9).tvalue.is_collectable() {
                                if (*uv).get_marked() & 1 << 5 != 0
                                    && (*(*ra_9).tvalue.value.object).get_marked()
                                        & (1 << 3 | 1 << 4)
                                        != 0
                                {
                                    luac_barrier_(
                                        state,
                                        &mut (*(uv as *mut Object)),
                                        &mut (*((*ra_9).tvalue.value.object as *mut Object)),
                                    );
                                } else {
                                };
                            } else {
                            };
                            continue;
                        }
                        11 => {
                            let ra_10: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            let slot: *const TValue;
                            let count_upvalues: *mut TValue =
                                (**((*cl).upvalues).l_upvalues.as_mut_ptr().offset(
                                    (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as isize,
                                ))
                                .v
                                .p;
                            let rc: *mut TValue = k.offset(
                                (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0) as isize,
                            );
                            let key: *mut TString = &mut (*((*rc).value.object as *mut TString));
                            if if !((*count_upvalues).get_tag_variant() == TAG_VARIANT_TABLE) {
                                slot = std::ptr::null();
                                0
                            } else {
                                slot = luah_getshortstr(
                                    &mut (*((*count_upvalues).value.object as *mut Table)),
                                    key,
                                );
                                (get_tag_type((*slot).get_tag()) != TAG_TYPE_NIL) as i32
                            } != 0
                            {
                                let io1_4: *mut TValue = &mut (*ra_10).tvalue;
                                let io2_4: *const TValue = slot;
                                (*io1_4).value = (*io2_4).value;
                                (*io1_4).set_tag((*io2_4).get_tag());
                            } else {
                                (*call_info).u.l.saved_program_counter = program_counter;
                                (*state).top.p = (*call_info).top.p;
                                luav_finishget(state, count_upvalues, rc, ra_10, slot);
                                trap = (*call_info).u.l.trap;
                            }
                            continue;
                        }
                        12 => {
                            let ra_11: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            let slot_0: *const TValue;
                            let rb_1: *mut TValue = &mut (*base.offset(
                                (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let rc_0: *mut TValue = &mut (*base.offset(
                                (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let n: u64;
                            if if (*rc_0).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                n = (*rc_0).value.integer as u64;
                                if !((*rb_1).get_tag_variant() == TAG_VARIANT_TABLE) {
                                    slot_0 = std::ptr::null();
                                    0
                                } else {
                                    slot_0 = if n.wrapping_sub(1 as u64)
                                        < (*((*rb_1).value.object as *mut Table)).array_limit
                                            as u64
                                    {
                                        &mut *((*((*rb_1).value.object as *mut Table)).array)
                                            .offset(n.wrapping_sub(1 as u64) as isize)
                                            as *mut TValue
                                            as *const TValue
                                    } else {
                                        luah_getint(
                                            &mut (*((*rb_1).value.object as *mut Table)),
                                            n as i64,
                                        )
                                    };
                                    !(get_tag_type((*slot_0).get_tag()) == TAG_TYPE_NIL) as i32
                                }
                            } else if !((*rb_1).get_tag_variant() == TAG_VARIANT_TABLE) {
                                slot_0 = std::ptr::null();
                                0
                            } else {
                                slot_0 = luah_get(
                                    &mut (*((*rb_1).value.object as *mut Table)),
                                    rc_0,
                                );
                                !(get_tag_type((*slot_0).get_tag()) == TAG_TYPE_NIL) as i32
                            } != 0
                            {
                                let io1_5: *mut TValue = &mut (*ra_11).tvalue;
                                let io2_5: *const TValue = slot_0;
                                (*io1_5).value = (*io2_5).value;
                                (*io1_5).set_tag((*io2_5).get_tag());
                            } else {
                                (*call_info).u.l.saved_program_counter = program_counter;
                                (*state).top.p = (*call_info).top.p;
                                luav_finishget(state, rb_1, rc_0, ra_11, slot_0);
                                trap = (*call_info).u.l.trap;
                            }
                            continue;
                        }
                        13 => {
                            let ra_12: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            let slot_1: *const TValue;
                            let rb_2: *mut TValue = &mut (*base.offset(
                                (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let c: i32 = (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0) as i32;
                            if if !((*rb_2).get_tag_variant() == TAG_VARIANT_TABLE) {
                                slot_1 = std::ptr::null();
                                0
                            } else {
                                slot_1 = if (c as u64).wrapping_sub(1 as u64)
                                    < (*((*rb_2).value.object as *mut Table)).array_limit as u64
                                {
                                    &mut *((*((*rb_2).value.object as *mut Table)).array)
                                        .offset((c - 1) as isize)
                                        as *mut TValue
                                        as *const TValue
                                } else {
                                    luah_getint(
                                        &mut (*((*rb_2).value.object as *mut Table)),
                                        c as i64,
                                    )
                                };
                                !(get_tag_type((*slot_1).get_tag()) == TAG_TYPE_NIL) as i32
                            } != 0
                            {
                                let io1_6: *mut TValue = &mut (*ra_12).tvalue;
                                let io2_6: *const TValue = slot_1;
                                (*io1_6).value = (*io2_6).value;
                                (*io1_6).set_tag((*io2_6).get_tag());
                            } else {
                                let mut key_0: TValue = TValue {
                                    value: Value {
                                        object: std::ptr::null_mut(),
                                    },
                                    tag: 0,
                                };
                                let io_1: *mut TValue = &mut key_0;
                                (*io_1).value.integer = c as i64;
                                (*io_1).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                                (*call_info).u.l.saved_program_counter = program_counter;
                                (*state).top.p = (*call_info).top.p;
                                luav_finishget(state, rb_2, &mut key_0, ra_12, slot_1);
                                trap = (*call_info).u.l.trap;
                            }
                            continue;
                        }
                        14 => {
                            let ra_13: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            let slot_2: *const TValue;
                            let rb_3: *mut TValue = &mut (*base.offset(
                                (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let rc_1: *mut TValue = k.offset(
                                (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0) as isize,
                            );
                            let key_1: *mut TString =
                                &mut (*((*rc_1).value.object as *mut TString));
                            if if !((*rb_3).get_tag_variant() == TAG_VARIANT_TABLE) {
                                slot_2 = std::ptr::null();
                                0
                            } else {
                                slot_2 = luah_getshortstr(
                                    &mut (*((*rb_3).value.object as *mut Table)),
                                    key_1,
                                );
                                !(get_tag_type((*slot_2).get_tag()) == TAG_TYPE_NIL) as i32
                            } != 0
                            {
                                let io1_7: *mut TValue = &mut (*ra_13).tvalue;
                                let io2_7: *const TValue = slot_2;
                                (*io1_7).value = (*io2_7).value;
                                (*io1_7).set_tag((*io2_7).get_tag());
                            } else {
                                (*call_info).u.l.saved_program_counter = program_counter;
                                (*state).top.p = (*call_info).top.p;
                                luav_finishget(state, rb_3, rc_1, ra_13, slot_2);
                                trap = (*call_info).u.l.trap;
                            }
                            continue;
                        }
                        15 => {
                            let slot_3: *const TValue;
                            let upval_0: *mut TValue = (**((*cl).upvalues)
                                .l_upvalues.as_mut_ptr()
                                .offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize))
                            .v
                            .p;
                            let rb_4: *mut TValue = k.offset(
                                (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as isize,
                            );
                            let rc_2: *mut TValue = if (i & (1 as u32) << 0 + 7 + 8) as i32 != 0 {
                                k.offset(
                                    (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0) as isize,
                                )
                            } else {
                                &mut (*base.offset(
                                    (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0) as isize,
                                ))
                                .tvalue
                            };
                            let key_2: *mut TString =
                                &mut (*((*rb_4).value.object as *mut TString));
                            if if !((*upval_0).get_tag_variant() == TAG_VARIANT_TABLE) {
                                slot_3 = std::ptr::null();
                                0
                            } else {
                                slot_3 = luah_getshortstr(
                                    &mut (*((*upval_0).value.object as *mut Table)),
                                    key_2,
                                );
                                !(get_tag_type((*slot_3).get_tag()) == TAG_TYPE_NIL) as i32
                            } != 0
                            {
                                let io1_8: *mut TValue = slot_3 as *mut TValue;
                                let io2_8: *const TValue = rc_2;
                                (*io1_8).value = (*io2_8).value;
                                (*io1_8).set_tag((*io2_8).get_tag());
                                if (*rc_2).is_collectable() {
                                    if (*(*upval_0).value.object).get_marked() & 1 << 5 != 0
                                        && (*(*rc_2).value.object).get_marked() & (1 << 3 | 1 << 4)
                                            != 0
                                    {
                                        luac_barrierback_(state, (*upval_0).value.object);
                                    } else {
                                    };
                                } else {
                                };
                            } else {
                                (*call_info).u.l.saved_program_counter = program_counter;
                                (*state).top.p = (*call_info).top.p;
                                luav_finishset(state, upval_0, rb_4, rc_2, slot_3);
                                trap = (*call_info).u.l.trap;
                            }
                            continue;
                        }
                        16 => {
                            let ra_14: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            let slot_4: *const TValue;
                            let rb_5: *mut TValue = &mut (*base.offset(
                                (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let rc_3: *mut TValue = if (i & (1 as u32) << 0 + 7 + 8) as i32 != 0 {
                                k.offset(
                                    (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0) as isize,
                                )
                            } else {
                                &mut (*base.offset(
                                    (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0) as isize,
                                ))
                                .tvalue
                            };
                            let n_0: u64;
                            if if (*rb_5).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                n_0 = (*rb_5).value.integer as u64;
                                if !((*ra_14).tvalue.get_tag_variant() == TAG_VARIANT_TABLE) {
                                    slot_4 = std::ptr::null();
                                    0
                                } else {
                                    slot_4 = if n_0.wrapping_sub(1 as u64)
                                        < (*((*ra_14).tvalue.value.object as *mut Table))
                                            .array_limit
                                            as u64
                                    {
                                        &mut *((*((*ra_14).tvalue.value.object as *mut Table))
                                            .array)
                                            .offset(n_0.wrapping_sub(1 as u64) as isize)
                                            as *mut TValue
                                            as *const TValue
                                    } else {
                                        luah_getint(
                                            &mut (*((*ra_14).tvalue.value.object as *mut Table)),
                                            n_0 as i64,
                                        )
                                    };
                                    (get_tag_type((*slot_4).get_tag()) != TAG_TYPE_NIL) as i32
                                }
                            } else if !((*ra_14).tvalue.get_tag_variant() == TAG_VARIANT_TABLE) {
                                slot_4 = std::ptr::null();
                                0
                            } else {
                                slot_4 = luah_get(
                                    &mut (*((*ra_14).tvalue.value.object as *mut Table)),
                                    rb_5,
                                );
                                !(get_tag_type((*slot_4).get_tag()) == TAG_TYPE_NIL) as i32
                            } != 0
                            {
                                let io1_9: *mut TValue = slot_4 as *mut TValue;
                                let io2_9: *const TValue = rc_3;
                                (*io1_9).value = (*io2_9).value;
                                (*io1_9).set_tag((*io2_9).get_tag());
                                if (*rc_3).is_collectable() {
                                    if (*(*ra_14).tvalue.value.object).get_marked() & 1 << 5 != 0
                                        && (*(*rc_3).value.object).get_marked() & (1 << 3 | 1 << 4)
                                            != 0
                                    {
                                        luac_barrierback_(state, (*ra_14).tvalue.value.object);
                                    } else {
                                    };
                                } else {
                                };
                            } else {
                                (*call_info).u.l.saved_program_counter = program_counter;
                                (*state).top.p = (*call_info).top.p;
                                luav_finishset(state, &mut (*ra_14).tvalue, rb_5, rc_3, slot_4);
                                trap = (*call_info).u.l.trap;
                            }
                            continue;
                        }
                        17 => {
                            let ra_15: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            let slot_5: *const TValue;
                            let c_0: i32 = (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32;
                            let rc_4: *mut TValue = if (i & (1 as u32) << 0 + 7 + 8) as i32 != 0 {
                                k.offset(
                                    (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0) as isize,
                                )
                            } else {
                                &mut (*base.offset(
                                    (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0) as isize,
                                ))
                                .tvalue
                            };
                            if if !((*ra_15).tvalue.get_tag_variant() == TAG_VARIANT_TABLE) {
                                slot_5 = std::ptr::null();
                                0
                            } else {
                                slot_5 = if (c_0 as u64).wrapping_sub(1 as u64)
                                    < (*((*ra_15).tvalue.value.object as *mut Table))
                                        .array_limit as u64
                                {
                                    &mut *((*((*ra_15).tvalue.value.object as *mut Table)).array)
                                        .offset((c_0 - 1) as isize)
                                        as *mut TValue
                                        as *const TValue
                                } else {
                                    luah_getint(
                                        &mut (*((*ra_15).tvalue.value.object as *mut Table)),
                                        c_0 as i64,
                                    )
                                };
                                !(get_tag_type((*slot_5).get_tag()) == TAG_TYPE_NIL) as i32
                            } != 0
                            {
                                let io1_10: *mut TValue = slot_5 as *mut TValue;
                                let io2_10: *const TValue = rc_4;
                                (*io1_10).value = (*io2_10).value;
                                (*io1_10).set_tag((*io2_10).get_tag());
                                if (*rc_4).is_collectable() {
                                    if (*(*ra_15).tvalue.value.object).get_marked() & 1 << 5 != 0
                                        && (*(*rc_4).value.object).get_marked() & (1 << 3 | 1 << 4)
                                            != 0
                                    {
                                        luac_barrierback_(state, (*ra_15).tvalue.value.object);
                                    } else {
                                    };
                                } else {
                                };
                            } else {
                                let mut key_3: TValue = TValue {
                                    value: Value {
                                        object: std::ptr::null_mut(),
                                    },
                                    tag: 0,
                                };
                                let io_2: *mut TValue = &mut key_3;
                                (*io_2).value.integer = c_0 as i64;
                                (*io_2).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                                (*call_info).u.l.saved_program_counter = program_counter;
                                (*state).top.p = (*call_info).top.p;
                                luav_finishset(
                                    state,
                                    &mut (*ra_15).tvalue,
                                    &mut key_3,
                                    rc_4,
                                    slot_5,
                                );
                                trap = (*call_info).u.l.trap;
                            }
                            continue;
                        }
                        18 => {
                            let ra_16: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            let slot_6: *const TValue;
                            let rb_6: *mut TValue = k.offset(
                                (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as isize,
                            );
                            let rc_5: *mut TValue = if (i & (1 as u32) << 0 + 7 + 8) as i32 != 0 {
                                k.offset(
                                    (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0) as isize,
                                )
                            } else {
                                &mut (*base.offset(
                                    (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0) as isize,
                                ))
                                .tvalue
                            };
                            let key_4: *mut TString =
                                &mut (*((*rb_6).value.object as *mut TString));
                            if if !((*ra_16).tvalue.get_tag_variant() == TAG_VARIANT_TABLE) {
                                slot_6 = std::ptr::null();
                                0
                            } else {
                                slot_6 = luah_getshortstr(
                                    &mut (*((*ra_16).tvalue.value.object as *mut Table)),
                                    key_4,
                                );
                                !(get_tag_type((*slot_6).get_tag()) == TAG_TYPE_NIL) as i32
                            } != 0
                            {
                                let io1_11: *mut TValue = slot_6 as *mut TValue;
                                let io2_11: *const TValue = rc_5;
                                (*io1_11).value = (*io2_11).value;
                                (*io1_11).set_tag((*io2_11).get_tag());
                                if (*rc_5).is_collectable() {
                                    if (*(*ra_16).tvalue.value.object).get_marked() & 1 << 5 != 0
                                        && (*(*rc_5).value.object).get_marked() & (1 << 3 | 1 << 4)
                                            != 0
                                    {
                                        luac_barrierback_(state, (*ra_16).tvalue.value.object);
                                    } else {
                                    };
                                } else {
                                };
                            } else {
                                (*call_info).u.l.saved_program_counter = program_counter;
                                (*state).top.p = (*call_info).top.p;
                                luav_finishset(state, &mut (*ra_16).tvalue, rb_6, rc_5, slot_6);
                                trap = (*call_info).u.l.trap;
                            }
                            continue;
                        }
                        19 => {
                            let ra_17: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            let mut b_3: i32 = (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32;
                            let mut c_1: i32 =
                                (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0) as i32;
                            let table: *mut Table;
                            if b_3 > 0 {
                                b_3 = 1 << b_3 - 1;
                            }
                            if (i & (1 as u32) << 0 + 7 + 8) as i32 != 0 {
                                c_1 += (*program_counter >> 0 + 7
                                    & !(!(0u32) << 8 + 8 + 1 + 8) << 0)
                                    as i32
                                    * ((1 << 8) - 1 + 1);
                            }
                            program_counter = program_counter.offset(1);
                            (*state).top.p = ra_17.offset(1 as isize);
                            table = luah_new(state);
                            let io_3: *mut TValue = &mut (*ra_17).tvalue;
                            let x_: *mut Table = table;
                            (*io_3).value.object = &mut (*(x_ as *mut Object));
                            (*io_3).set_tag(TAG_VARIANT_TABLE);
                            (*io_3).set_collectable();
                            if b_3 != 0 || c_1 != 0 {
                                luah_resize(state, table, c_1 as u32, b_3 as u32);
                            }
                            if (*(*state).global).gc_debt > 0 {
                                (*call_info).u.l.saved_program_counter = program_counter;
                                (*state).top.p = ra_17.offset(1 as isize);
                                luac_step(state);
                                trap = (*call_info).u.l.trap;
                            }
                            continue;
                        }
                        20 => {
                            let ra_18: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            let slot_7: *const TValue;
                            let rb_7: *mut TValue = &mut (*base.offset(
                                (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let rc_6: *mut TValue = if (i & (1 as u32) << 0 + 7 + 8) as i32 != 0 {
                                k.offset(
                                    (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0) as isize,
                                )
                            } else {
                                &mut (*base.offset(
                                    (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0) as isize,
                                ))
                                .tvalue
                            };
                            let key_5: *mut TString =
                                &mut (*((*rc_6).value.object as *mut TString));
                            let io1_12: *mut TValue = &mut (*ra_18.offset(1 as isize)).tvalue;
                            let io2_12: *const TValue = rb_7;
                            (*io1_12).value = (*io2_12).value;
                            (*io1_12).set_tag((*io2_12).get_tag());
                            if if !((*rb_7).get_tag_variant() == TAG_VARIANT_TABLE) {
                                slot_7 = std::ptr::null();
                                0
                            } else {
                                slot_7 = luah_getstr(
                                    &mut (*((*rb_7).value.object as *mut Table)),
                                    key_5,
                                );
                                !(get_tag_type((*slot_7).get_tag()) == TAG_TYPE_NIL) as i32
                            } != 0
                            {
                                let io1_13: *mut TValue = &mut (*ra_18).tvalue;
                                let io2_13: *const TValue = slot_7;
                                (*io1_13).value = (*io2_13).value;
                                (*io1_13).set_tag((*io2_13).get_tag());
                            } else {
                                (*call_info).u.l.saved_program_counter = program_counter;
                                (*state).top.p = (*call_info).top.p;
                                luav_finishget(state, rb_7, rc_6, ra_18, slot_7);
                                trap = (*call_info).u.l.trap;
                            }
                            continue;
                        }
                        21 => {
                            let ra_19: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            let v1: *mut TValue = &mut (*base.offset(
                                (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let imm: i32 = (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0) as i32
                                - ((1 << 8) - 1 >> 1);
                            if (*v1).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                let iv1: i64 = (*v1).value.integer;
                                program_counter = program_counter.offset(1);
                                let io_4: *mut TValue = &mut (*ra_19).tvalue;
                                (*io_4).value.integer = (iv1 as u64).wrapping_add(imm as u64) as i64;
                                (*io_4).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                            } else if (*v1).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                                let nb: f64 = (*v1).value.number;
                                let fimm: f64 = imm as f64;
                                program_counter = program_counter.offset(1);
                                let io_5: *mut TValue = &mut (*ra_19).tvalue;
                                (*io_5).value.number = nb + fimm;
                                (*io_5).set_tag(TAG_VARIANT_NUMERIC_NUMBER);
                            }
                            continue;
                        }
                        22 => {
                            let v1_0: *mut TValue = &mut (*base.offset(
                                (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let v2: *mut TValue = k.offset(
                                (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0) as isize,
                            );
                            let ra_20: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            if (*v1_0).get_tag() == TAG_VARIANT_NUMERIC_INTEGER
                                && (*v2).get_tag() == TAG_VARIANT_NUMERIC_INTEGER
                            {
                                let i1: i64 = (*v1_0).value.integer;
                                let i2: i64 = (*v2).value.integer;
                                program_counter = program_counter.offset(1);
                                let io_6: *mut TValue = &mut (*ra_20).tvalue;
                                (*io_6).value.integer = (i1 as u64).wrapping_add(i2 as u64) as i64;
                                (*io_6).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                            } else {
                                let mut n1: f64 = 0.0;
                                let mut n2: f64 = 0.0;
                                if (if (*v1_0).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                                    n1 = (*v1_0).value.number;
                                    1
                                } else {
                                    if (*v1_0).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                        n1 = (*v1_0).value.integer as f64;
                                        1
                                    } else {
                                        0
                                    }
                                }) != 0
                                    && (if (*v2).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                                        n2 = (*v2).value.number;
                                        1
                                    } else {
                                        if (*v2).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                            n2 = (*v2).value.integer as f64;
                                            1
                                        } else {
                                            0
                                        }
                                    }) != 0
                                {
                                    program_counter = program_counter.offset(1);
                                    let io_7: *mut TValue = &mut (*ra_20).tvalue;
                                    (*io_7).value.number = n1 + n2;
                                    (*io_7).set_tag(TAG_VARIANT_NUMERIC_NUMBER);
                                }
                            }
                            continue;
                        }
                        OP_SUBK => {
                            let v1_1: *mut TValue = &mut (*base.offset(
                                (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let v2_0: *mut TValue = k.offset(
                                (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0) as isize,
                            );
                            let ra_21: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            if (*v1_1).get_tag() == TAG_VARIANT_NUMERIC_INTEGER
                                && (*v2_0).get_tag() == TAG_VARIANT_NUMERIC_INTEGER
                            {
                                let i1_0: i64 = (*v1_1).value.integer;
                                let i2_0: i64 = (*v2_0).value.integer;
                                program_counter = program_counter.offset(1);
                                let io_8: *mut TValue = &mut (*ra_21).tvalue;
                                (*io_8).value.integer = (i1_0 as u64).wrapping_sub(i2_0 as u64) as i64;
                                (*io_8).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                            } else {
                                let mut n1_0: f64 = 0.0;
                                let mut n2_0: f64 = 0.0;
                                if (if (*v1_1).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                                    n1_0 = (*v1_1).value.number;
                                    1
                                } else {
                                    if (*v1_1).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                        n1_0 = (*v1_1).value.integer as f64;
                                        1
                                    } else {
                                        0
                                    }
                                }) != 0
                                    && (if (*v2_0).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                                        n2_0 = (*v2_0).value.number;
                                        1
                                    } else {
                                        if (*v2_0).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                            n2_0 = (*v2_0).value.integer as f64;
                                            1
                                        } else {
                                            0
                                        }
                                    }) != 0
                                {
                                    program_counter = program_counter.offset(1);
                                    let io_9: *mut TValue = &mut (*ra_21).tvalue;
                                    (*io_9).value.number = n1_0 - n2_0;
                                    (*io_9).set_tag(TAG_VARIANT_NUMERIC_NUMBER);
                                }
                            }
                            continue;
                        }
                        OP_MULK => {
                            let v1_2: *mut TValue = &mut (*base.offset(
                                (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let v2_1: *mut TValue = k.offset(
                                (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0) as isize,
                            );
                            let ra_22: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            if (*v1_2).get_tag() == TAG_VARIANT_NUMERIC_INTEGER
                                && (*v2_1).get_tag() == TAG_VARIANT_NUMERIC_INTEGER
                            {
                                let i1_1: i64 = (*v1_2).value.integer;
                                let i2_1: i64 = (*v2_1).value.integer;
                                program_counter = program_counter.offset(1);
                                let io_10: *mut TValue = &mut (*ra_22).tvalue;
                                (*io_10).value.integer = (i1_1 as u64).wrapping_mul(i2_1 as u64) as i64;
                                (*io_10).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                            } else {
                                let mut n1_1: f64 = 0.0;
                                let mut n2_1: f64 = 0.0;
                                if (if (*v1_2).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                                    n1_1 = (*v1_2).value.number;
                                    1
                                } else {
                                    if (*v1_2).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                        n1_1 = (*v1_2).value.integer as f64;
                                        1
                                    } else {
                                        0
                                    }
                                }) != 0
                                    && (if (*v2_1).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                                        n2_1 = (*v2_1).value.number;
                                        1
                                    } else {
                                        if (*v2_1).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                            n2_1 = (*v2_1).value.integer as f64;
                                            1
                                        } else {
                                            0
                                        }
                                    }) != 0
                                {
                                    program_counter = program_counter.offset(1);
                                    let io_11: *mut TValue = &mut (*ra_22).tvalue;
                                    (*io_11).value.number = n1_1 * n2_1;
                                    (*io_11).set_tag(TAG_VARIANT_NUMERIC_NUMBER);
                                }
                            }
                            continue;
                        }
                        OP_MODK => {
                            (*call_info).u.l.saved_program_counter = program_counter;
                            (*state).top.p = (*call_info).top.p;
                            let v1_3: *mut TValue = &mut (*base.offset(
                                (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let v2_2: *mut TValue = k.offset(
                                (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0) as isize,
                            );
                            let ra_23: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            if (*v1_3).get_tag() == TAG_VARIANT_NUMERIC_INTEGER
                                && (*v2_2).get_tag() == TAG_VARIANT_NUMERIC_INTEGER
                            {
                                let i1_2: i64 = (*v1_3).value.integer;
                                let i2_2: i64 = (*v2_2).value.integer;
                                program_counter = program_counter.offset(1);
                                let io_12: *mut TValue = &mut (*ra_23).tvalue;
                                (*io_12).value.integer = luav_mod(state, i1_2, i2_2);
                                (*io_12).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                            } else {
                                let mut n1_2: f64 = 0.0;
                                let mut n2_2: f64 = 0.0;
                                if (if (*v1_3).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                                    n1_2 = (*v1_3).value.number;
                                    1
                                } else {
                                    if (*v1_3).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                        n1_2 = (*v1_3).value.integer as f64;
                                        1
                                    } else {
                                        0
                                    }
                                }) != 0
                                    && (if (*v2_2).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                                        n2_2 = (*v2_2).value.number;
                                        1
                                    } else {
                                        if (*v2_2).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                            n2_2 = (*v2_2).value.integer as f64;
                                            1
                                        } else {
                                            0
                                        }
                                    }) != 0
                                {
                                    program_counter = program_counter.offset(1);
                                    let io_13: *mut TValue = &mut (*ra_23).tvalue;
                                    (*io_13).value.number = luav_modf(state, n1_2, n2_2);
                                    (*io_13).set_tag(TAG_VARIANT_NUMERIC_NUMBER);
                                }
                            }
                            continue;
                        }
                        OP_POWK => {
                            let ra_24: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            let v1_4: *mut TValue = &mut (*base.offset(
                                (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let v2_3: *mut TValue = k.offset(
                                (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0) as isize,
                            );
                            let mut n1_3: f64 = 0.0;
                            let mut n2_3: f64 = 0.0;
                            if (if (*v1_4).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                                n1_3 = (*v1_4).value.number;
                                1
                            } else {
                                if (*v1_4).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                    n1_3 = (*v1_4).value.integer as f64;
                                    1
                                } else {
                                    0
                                }
                            }) != 0
                                && (if (*v2_3).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                                    n2_3 = (*v2_3).value.number;
                                    1
                                } else {
                                    if (*v2_3).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                        n2_3 = (*v2_3).value.integer as f64;
                                        1
                                    } else {
                                        0
                                    }
                                }) != 0
                            {
                                program_counter = program_counter.offset(1);
                                let io_14: *mut TValue = &mut (*ra_24).tvalue;
                                (*io_14).value.number = if n2_3 == 2.0 {
                                    n1_3 * n1_3
                                } else {
                                    n1_3.powf(n2_3)
                                };
                                (*io_14).set_tag(TAG_VARIANT_NUMERIC_NUMBER);
                            }
                            continue;
                        }
                        OP_DIVK => {
                            let ra_25: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            let v1_5: *mut TValue = &mut (*base.offset(
                                (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let v2_4: *mut TValue = k.offset(
                                (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0) as isize,
                            );
                            let mut n1_4: f64 = 0.0;
                            let mut n2_4: f64 = 0.0;
                            if (if (*v1_5).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                                n1_4 = (*v1_5).value.number;
                                1
                            } else {
                                if (*v1_5).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                    n1_4 = (*v1_5).value.integer as f64;
                                    1
                                } else {
                                    0
                                }
                            }) != 0
                                && (if (*v2_4).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                                    n2_4 = (*v2_4).value.number;
                                    1
                                } else {
                                    if (*v2_4).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                        n2_4 = (*v2_4).value.integer as f64;
                                        1
                                    } else {
                                        0
                                    }
                                }) != 0
                            {
                                program_counter = program_counter.offset(1);
                                let io_15: *mut TValue = &mut (*ra_25).tvalue;
                                (*io_15).value.number = n1_4 / n2_4;
                                (*io_15).set_tag(TAG_VARIANT_NUMERIC_NUMBER);
                            }
                            continue;
                        }
                        OP_IDIVK => {
                            (*call_info).u.l.saved_program_counter = program_counter;
                            (*state).top.p = (*call_info).top.p;
                            let v1_6: *mut TValue = &mut (*base.offset(
                                (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let v2_5: *mut TValue = k.offset(
                                (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0) as isize,
                            );
                            let ra_26: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            if (*v1_6).get_tag() == TAG_VARIANT_NUMERIC_INTEGER
                                && (*v2_5).get_tag() == TAG_VARIANT_NUMERIC_INTEGER
                            {
                                let i1_3: i64 = (*v1_6).value.integer;
                                let i2_3: i64 = (*v2_5).value.integer;
                                program_counter = program_counter.offset(1);
                                let io_16: *mut TValue = &mut (*ra_26).tvalue;
                                (*io_16).value.integer = luav_idiv(state, i1_3, i2_3);
                                (*io_16).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                            } else {
                                let mut n1_5: f64 = 0.0;
                                let mut n2_5: f64 = 0.0;
                                if (if (*v1_6).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                                    n1_5 = (*v1_6).value.number;
                                    1
                                } else {
                                    if (*v1_6).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                        n1_5 = (*v1_6).value.integer as f64;
                                        1
                                    } else {
                                        0
                                    }
                                }) != 0
                                    && (if (*v2_5).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                                        n2_5 = (*v2_5).value.number;
                                        1
                                    } else {
                                        if (*v2_5).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                            n2_5 = (*v2_5).value.integer as f64;
                                            1
                                        } else {
                                            0
                                        }
                                    }) != 0
                                {
                                    program_counter = program_counter.offset(1);
                                    let io_17: *mut TValue = &mut (*ra_26).tvalue;
                                    (*io_17).value.number = (n1_5 / n2_5).floor();
                                    (*io_17).set_tag(TAG_VARIANT_NUMERIC_NUMBER);
                                }
                            }
                            continue;
                        }
                        OP_BANDK => {
                            let ra_27: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            let v1_7: *mut TValue = &mut (*base.offset(
                                (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let v2_6: *mut TValue = k.offset(
                                (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0) as isize,
                            );
                            let mut i1_4: i64 = 0;
                            let i2_4: i64 = (*v2_6).value.integer;
                            if if (((*v1_7).get_tag() == TAG_VARIANT_NUMERIC_INTEGER) as i32 != 0)
                                as i64
                                != 0
                            {
                                i1_4 = (*v1_7).value.integer;
                                1
                            } else {
                                luav_tointegerns(v1_7, &mut i1_4, F2I::Equal)
                            } != 0
                            {
                                program_counter = program_counter.offset(1);
                                let io_18: *mut TValue = &mut (*ra_27).tvalue;
                                (*io_18).value.integer = (i1_4 as u64 & i2_4 as u64) as i64;
                                (*io_18).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                            }
                            continue;
                        }
                        OP_BORK => {
                            let ra_28: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            let v1_8: *mut TValue = &mut (*base.offset(
                                (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let v2_7: *mut TValue = k.offset(
                                (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0) as isize,
                            );
                            let mut i1_5: i64 = 0;
                            let i2_5: i64 = (*v2_7).value.integer;
                            if if (((*v1_8).get_tag() == TAG_VARIANT_NUMERIC_INTEGER) as i32 != 0)
                                as i64
                                != 0
                            {
                                i1_5 = (*v1_8).value.integer;
                                1
                            } else {
                                luav_tointegerns(v1_8, &mut i1_5, F2I::Equal)
                            } != 0
                            {
                                program_counter = program_counter.offset(1);
                                let io_19: *mut TValue = &mut (*ra_28).tvalue;
                                (*io_19).value.integer = (i1_5 as u64 | i2_5 as u64) as i64;
                                (*io_19).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                            }
                            continue;
                        }
                        OP_BXORK => {
                            let ra_29: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            let v1_9: *mut TValue = &mut (*base.offset(
                                (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let v2_8: *mut TValue = k.offset(
                                (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0) as isize,
                            );
                            let mut i1_6: i64 = 0;
                            let i2_6: i64 = (*v2_8).value.integer;
                            if if (((*v1_9).get_tag() == TAG_VARIANT_NUMERIC_INTEGER) as i32 != 0)
                                as i64
                                != 0
                            {
                                i1_6 = (*v1_9).value.integer;
                                1
                            } else {
                                luav_tointegerns(v1_9, &mut i1_6, F2I::Equal)
                            } != 0
                            {
                                program_counter = program_counter.offset(1);
                                let io_20: *mut TValue = &mut (*ra_29).tvalue;
                                (*io_20).value.integer = (i1_6 as u64 ^ i2_6 as u64) as i64;
                                (*io_20).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                            }
                            continue;
                        }
                        32 => {
                            let ra_30: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            let rb_8: *mut TValue = &mut (*base.offset(
                                (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let ic: i32 = (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0) as i32
                                - ((1 << 8) - 1 >> 1);
                            let mut ib: i64 = 0;
                            if if (((*rb_8).get_tag() == TAG_VARIANT_NUMERIC_INTEGER) as i32 != 0)
                                as i64
                                != 0
                            {
                                ib = (*rb_8).value.integer;
                                1
                            } else {
                                luav_tointegerns(rb_8, &mut ib, F2I::Equal)
                            } != 0
                            {
                                program_counter = program_counter.offset(1);
                                let io_21: *mut TValue = &mut (*ra_30).tvalue;
                                (*io_21).value.integer = luav_shiftl(ib, -ic as i64);
                                (*io_21).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                            }
                            continue;
                        }
                        33 => {
                            let ra_31: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            let rb_9: *mut TValue = &mut (*base.offset(
                                (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let ic_0: i32 = (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0) as i32
                                - ((1 << 8) - 1 >> 1);
                            let mut ib_0: i64 = 0;
                            if if (((*rb_9).get_tag() == TAG_VARIANT_NUMERIC_INTEGER) as i32 != 0)
                                as i64
                                != 0
                            {
                                ib_0 = (*rb_9).value.integer;
                                1
                            } else {
                                luav_tointegerns(rb_9, &mut ib_0, F2I::Equal)
                            } != 0
                            {
                                program_counter = program_counter.offset(1);
                                let io_22: *mut TValue = &mut (*ra_31).tvalue;
                                (*io_22).value.integer = luav_shiftl(ic_0 as i64, ib_0);
                                (*io_22).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                            }
                            continue;
                        }
                        34 => {
                            let v1_10: *mut TValue = &mut (*base.offset(
                                (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let v2_9: *mut TValue = &mut (*base.offset(
                                (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let ra_32: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            if (*v1_10).get_tag() == TAG_VARIANT_NUMERIC_INTEGER
                                && (*v2_9).get_tag() == TAG_VARIANT_NUMERIC_INTEGER
                            {
                                let i1_7: i64 = (*v1_10).value.integer;
                                let i2_7: i64 = (*v2_9).value.integer;
                                program_counter = program_counter.offset(1);
                                let io_23: *mut TValue = &mut (*ra_32).tvalue;
                                (*io_23).value.integer = (i1_7 as u64).wrapping_add(i2_7 as u64) as i64;
                                (*io_23).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                            } else {
                                let mut n1_6: f64 = 0.0;
                                let mut n2_6: f64 = 0.0;
                                if (if (*v1_10).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                                    n1_6 = (*v1_10).value.number;
                                    1
                                } else {
                                    if (*v1_10).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                        n1_6 = (*v1_10).value.integer as f64;
                                        1
                                    } else {
                                        0
                                    }
                                }) != 0
                                    && (if (*v2_9).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                                        n2_6 = (*v2_9).value.number;
                                        1
                                    } else {
                                        if (*v2_9).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                            n2_6 = (*v2_9).value.integer as f64;
                                            1
                                        } else {
                                            0
                                        }
                                    }) != 0
                                {
                                    program_counter = program_counter.offset(1);
                                    let io_24: *mut TValue = &mut (*ra_32).tvalue;
                                    (*io_24).value.number = n1_6 + n2_6;
                                    (*io_24).set_tag(TAG_VARIANT_NUMERIC_NUMBER);
                                }
                            }
                            continue;
                        }
                        OP_SUB => {
                            let v1_11: *mut TValue = &mut (*base.offset(
                                (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let v2_10: *mut TValue = &mut (*base.offset(
                                (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let ra_33: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            if (*v1_11).get_tag() == TAG_VARIANT_NUMERIC_INTEGER
                                && (*v2_10).get_tag() == TAG_VARIANT_NUMERIC_INTEGER
                            {
                                let i1_8: i64 = (*v1_11).value.integer;
                                let i2_8: i64 = (*v2_10).value.integer;
                                program_counter = program_counter.offset(1);
                                let io_25: *mut TValue = &mut (*ra_33).tvalue;
                                (*io_25).value.integer = (i1_8 as u64).wrapping_sub(i2_8 as u64) as i64;
                                (*io_25).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                            } else {
                                let mut n1_7: f64 = 0.0;
                                let mut n2_7: f64 = 0.0;
                                if (if (*v1_11).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                                    n1_7 = (*v1_11).value.number;
                                    1
                                } else {
                                    if (*v1_11).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                        n1_7 = (*v1_11).value.integer as f64;
                                        1
                                    } else {
                                        0
                                    }
                                }) != 0
                                    && (if (*v2_10).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                                        n2_7 = (*v2_10).value.number;
                                        1
                                    } else {
                                        if (*v2_10).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                            n2_7 = (*v2_10).value.integer as f64;
                                            1
                                        } else {
                                            0
                                        }
                                    }) != 0
                                {
                                    program_counter = program_counter.offset(1);
                                    let io_26: *mut TValue = &mut (*ra_33).tvalue;
                                    (*io_26).value.number = n1_7 - n2_7;
                                    (*io_26).set_tag(TAG_VARIANT_NUMERIC_NUMBER);
                                }
                            }
                            continue;
                        }
                        OP_MUL => {
                            let v1_12: *mut TValue = &mut (*base.offset(
                                (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let v2_11: *mut TValue = &mut (*base.offset(
                                (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let ra_34: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            if (*v1_12).get_tag() == TAG_VARIANT_NUMERIC_INTEGER
                                && (*v2_11).get_tag() == TAG_VARIANT_NUMERIC_INTEGER
                            {
                                let i1_9: i64 = (*v1_12).value.integer;
                                let i2_9: i64 = (*v2_11).value.integer;
                                program_counter = program_counter.offset(1);
                                let io_27: *mut TValue = &mut (*ra_34).tvalue;
                                (*io_27).value.integer = (i1_9 as u64).wrapping_mul(i2_9 as u64) as i64;
                                (*io_27).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                            } else {
                                let mut n1_8: f64 = 0.0;
                                let mut n2_8: f64 = 0.0;
                                if (if (*v1_12).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                                    n1_8 = (*v1_12).value.number;
                                    1
                                } else {
                                    if (*v1_12).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                        n1_8 = (*v1_12).value.integer as f64;
                                        1
                                    } else {
                                        0
                                    }
                                }) != 0
                                    && (if (*v2_11).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                                        n2_8 = (*v2_11).value.number;
                                        1
                                    } else {
                                        if (*v2_11).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                            n2_8 = (*v2_11).value.integer as f64;
                                            1
                                        } else {
                                            0
                                        }
                                    }) != 0
                                {
                                    program_counter = program_counter.offset(1);
                                    let io_28: *mut TValue = &mut (*ra_34).tvalue;
                                    (*io_28).value.number = n1_8 * n2_8;
                                    (*io_28).set_tag(TAG_VARIANT_NUMERIC_NUMBER);
                                }
                            }
                            continue;
                        }
                        OP_MOD => {
                            (*call_info).u.l.saved_program_counter = program_counter;
                            (*state).top.p = (*call_info).top.p;
                            let v1_13: *mut TValue = &mut (*base.offset(
                                (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let v2_12: *mut TValue = &mut (*base.offset(
                                (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let ra_35: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            if (*v1_13).get_tag() == TAG_VARIANT_NUMERIC_INTEGER
                                && (*v2_12).get_tag() == TAG_VARIANT_NUMERIC_INTEGER
                            {
                                let i1_10: i64 = (*v1_13).value.integer;
                                let i2_10: i64 = (*v2_12).value.integer;
                                program_counter = program_counter.offset(1);
                                let io_29: *mut TValue = &mut (*ra_35).tvalue;
                                (*io_29).value.integer = luav_mod(state, i1_10, i2_10);
                                (*io_29).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                            } else {
                                let mut n1_9: f64 = 0.0;
                                let mut n2_9: f64 = 0.0;
                                if (if (*v1_13).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                                    n1_9 = (*v1_13).value.number;
                                    1
                                } else {
                                    if (*v1_13).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                        n1_9 = (*v1_13).value.integer as f64;
                                        1
                                    } else {
                                        0
                                    }
                                }) != 0
                                    && (if (*v2_12).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                                        n2_9 = (*v2_12).value.number;
                                        1
                                    } else {
                                        if (*v2_12).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                            n2_9 = (*v2_12).value.integer as f64;
                                            1
                                        } else {
                                            0
                                        }
                                    }) != 0
                                {
                                    program_counter = program_counter.offset(1);
                                    let io_30: *mut TValue = &mut (*ra_35).tvalue;
                                    (*io_30).value.number = luav_modf(state, n1_9, n2_9);
                                    (*io_30).set_tag(TAG_VARIANT_NUMERIC_NUMBER);
                                }
                            }
                            continue;
                        }
                        OP_POW => {
                            let ra_36: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            let v1_14: *mut TValue = &mut (*base.offset(
                                (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let v2_13: *mut TValue = &mut (*base.offset(
                                (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let mut n1_10: f64 = 0.0;
                            let mut n2_10: f64 = 0.0;
                            if (if (*v1_14).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                                n1_10 = (*v1_14).value.number;
                                1
                            } else {
                                if (*v1_14).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                    n1_10 = (*v1_14).value.integer as f64;
                                    1
                                } else {
                                    0
                                }
                            }) != 0
                                && (if (*v2_13).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                                    n2_10 = (*v2_13).value.number;
                                    1
                                } else {
                                    if (*v2_13).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                        n2_10 = (*v2_13).value.integer as f64;
                                        1
                                    } else {
                                        0
                                    }
                                }) != 0
                            {
                                program_counter = program_counter.offset(1);
                                let io_31: *mut TValue = &mut (*ra_36).tvalue;
                                (*io_31).value.number = if n2_10 == 2.0 {
                                    n1_10 * n1_10
                                } else {
                                    n1_10.powf(n2_10)
                                };
                                (*io_31).set_tag(TAG_VARIANT_NUMERIC_NUMBER);
                            }
                            continue;
                        }
                        OP_DIV => {
                            let ra_37: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            let v1_15: *mut TValue = &mut (*base.offset(
                                (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let v2_14: *mut TValue = &mut (*base.offset(
                                (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let mut n1_11: f64 = 0.0;
                            let mut n2_11: f64 = 0.0;
                            if (if (*v1_15).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                                n1_11 = (*v1_15).value.number;
                                1
                            } else {
                                if (*v1_15).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                    n1_11 = (*v1_15).value.integer as f64;
                                    1
                                } else {
                                    0
                                }
                            }) != 0
                                && (if (*v2_14).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                                    n2_11 = (*v2_14).value.number;
                                    1
                                } else {
                                    if (*v2_14).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                        n2_11 = (*v2_14).value.integer as f64;
                                        1
                                    } else {
                                        0
                                    }
                                }) != 0
                            {
                                program_counter = program_counter.offset(1);
                                let io_32: *mut TValue = &mut (*ra_37).tvalue;
                                (*io_32).value.number = n1_11 / n2_11;
                                (*io_32).set_tag(TAG_VARIANT_NUMERIC_NUMBER);
                            }
                            continue;
                        }
                        OP_IDIV => {
                            (*call_info).u.l.saved_program_counter = program_counter;
                            (*state).top.p = (*call_info).top.p;
                            let v1_16: *mut TValue = &mut (*base.offset(
                                (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let v2_15: *mut TValue = &mut (*base.offset(
                                (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let ra_38: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            if (*v1_16).get_tag() == TAG_VARIANT_NUMERIC_INTEGER
                                && (*v2_15).get_tag() == TAG_VARIANT_NUMERIC_INTEGER
                            {
                                let i1_11: i64 = (*v1_16).value.integer;
                                let i2_11: i64 = (*v2_15).value.integer;
                                program_counter = program_counter.offset(1);
                                let io_33: *mut TValue = &mut (*ra_38).tvalue;
                                (*io_33).value.integer = luav_idiv(state, i1_11, i2_11);
                                (*io_33).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                            } else {
                                let mut n1_12: f64 = 0.0;
                                let mut n2_12: f64 = 0.0;
                                if (if (*v1_16).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                                    n1_12 = (*v1_16).value.number;
                                    1
                                } else {
                                    if (*v1_16).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                        n1_12 = (*v1_16).value.integer as f64;
                                        1
                                    } else {
                                        0
                                    }
                                }) != 0
                                    && (if (*v2_15).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                                        n2_12 = (*v2_15).value.number;
                                        1
                                    } else {
                                        if (*v2_15).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                            n2_12 = (*v2_15).value.integer as f64;
                                            1
                                        } else {
                                            0
                                        }
                                    }) != 0
                                {
                                    program_counter = program_counter.offset(1);
                                    let io_34: *mut TValue = &mut (*ra_38).tvalue;
                                    (*io_34).value.number = (n1_12 / n2_12).floor();
                                    (*io_34).set_tag(TAG_VARIANT_NUMERIC_NUMBER);
                                }
                            }
                            continue;
                        }
                        OP_BAND => {
                            let ra_39: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            let v1_17: *mut TValue = &mut (*base.offset(
                                (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let v2_16: *mut TValue = &mut (*base.offset(
                                (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let mut i1_12: i64 = 0;
                            let mut i2_12: i64 = 0;
                            if (if (((*v1_17).get_tag() == TAG_VARIANT_NUMERIC_INTEGER) as i32 != 0)
                                as i64
                                != 0
                            {
                                i1_12 = (*v1_17).value.integer;
                                1
                            } else {
                                luav_tointegerns(v1_17, &mut i1_12, F2I::Equal)
                            }) != 0
                                && (if (((*v2_16).get_tag() == TAG_VARIANT_NUMERIC_INTEGER) as i32
                                    != 0) as i64
                                    != 0
                                {
                                    i2_12 = (*v2_16).value.integer;
                                    1
                                } else {
                                    luav_tointegerns(v2_16, &mut i2_12, F2I::Equal)
                                }) != 0
                            {
                                program_counter = program_counter.offset(1);
                                let io_35: *mut TValue = &mut (*ra_39).tvalue;
                                (*io_35).value.integer = (i1_12 as u64 & i2_12 as u64) as i64;
                                (*io_35).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                            }
                            continue;
                        }
                        OP_BOR => {
                            let ra_40: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            let v1_18: *mut TValue = &mut (*base.offset(
                                (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let v2_17: *mut TValue = &mut (*base.offset(
                                (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let mut i1_13: i64 = 0;
                            let mut i2_13: i64 = 0;
                            if (if (((*v1_18).get_tag() == TAG_VARIANT_NUMERIC_INTEGER) as i32 != 0)
                                as i64
                                != 0
                            {
                                i1_13 = (*v1_18).value.integer;
                                1
                            } else {
                                luav_tointegerns(v1_18, &mut i1_13, F2I::Equal)
                            }) != 0
                                && (if (((*v2_17).get_tag() == TAG_VARIANT_NUMERIC_INTEGER) as i32
                                    != 0) as i64
                                    != 0
                                {
                                    i2_13 = (*v2_17).value.integer;
                                    1
                                } else {
                                    luav_tointegerns(v2_17, &mut i2_13, F2I::Equal)
                                }) != 0
                            {
                                program_counter = program_counter.offset(1);
                                let io_36: *mut TValue = &mut (*ra_40).tvalue;
                                (*io_36).value.integer = (i1_13 as u64 | i2_13 as u64) as i64;
                                (*io_36).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                            }
                            continue;
                        }
                        OP_BXOR => {
                            let ra_41: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            let v1_19: *mut TValue = &mut (*base.offset(
                                (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let v2_18: *mut TValue = &mut (*base.offset(
                                (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let mut i1_14: i64 = 0;
                            let mut i2_14: i64 = 0;
                            if (if (((*v1_19).get_tag() == TAG_VARIANT_NUMERIC_INTEGER) as i32 != 0)
                                as i64
                                != 0
                            {
                                i1_14 = (*v1_19).value.integer;
                                1
                            } else {
                                luav_tointegerns(v1_19, &mut i1_14, F2I::Equal)
                            }) != 0
                                && (if (((*v2_18).get_tag() == TAG_VARIANT_NUMERIC_INTEGER) as i32
                                    != 0) as i64
                                    != 0
                                {
                                    i2_14 = (*v2_18).value.integer;
                                    1
                                } else {
                                    luav_tointegerns(v2_18, &mut i2_14, F2I::Equal)
                                }) != 0
                            {
                                program_counter = program_counter.offset(1);
                                let io_37: *mut TValue = &mut (*ra_41).tvalue;
                                (*io_37).value.integer = (i1_14 as u64 ^ i2_14 as u64) as i64;
                                (*io_37).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                            }
                            continue;
                        }
                        OP_SHR => {
                            let ra_42: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            let v1_20: *mut TValue = &mut (*base.offset(
                                (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let v2_19: *mut TValue = &mut (*base.offset(
                                (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let mut i1_15: i64 = 0;
                            let mut i2_15: i64 = 0;
                            if (if (((*v1_20).get_tag() == TAG_VARIANT_NUMERIC_INTEGER) as i32 != 0)
                                as i64
                                != 0
                            {
                                i1_15 = (*v1_20).value.integer;
                                1
                            } else {
                                luav_tointegerns(v1_20, &mut i1_15, F2I::Equal)
                            }) != 0
                                && (if (((*v2_19).get_tag() == TAG_VARIANT_NUMERIC_INTEGER) as i32
                                    != 0) as i64
                                    != 0
                                {
                                    i2_15 = (*v2_19).value.integer;
                                    1
                                } else {
                                    luav_tointegerns(v2_19, &mut i2_15, F2I::Equal)
                                }) != 0
                            {
                                program_counter = program_counter.offset(1);
                                let io_38: *mut TValue = &mut (*ra_42).tvalue;
                                (*io_38).value.integer =
                                    luav_shiftl(i1_15, (0u64).wrapping_sub(i2_15 as u64) as i64);
                                (*io_38).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                            }
                            continue;
                        }
                        OP_SHL => {
                            let ra_43: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            let v1_21: *mut TValue = &mut (*base.offset(
                                (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let v2_20: *mut TValue = &mut (*base.offset(
                                (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let mut i1_16: i64 = 0;
                            let mut i2_16: i64 = 0;
                            if (if (((*v1_21).get_tag() == TAG_VARIANT_NUMERIC_INTEGER) as i32 != 0)
                                as i64
                                != 0
                            {
                                i1_16 = (*v1_21).value.integer;
                                1
                            } else {
                                luav_tointegerns(v1_21, &mut i1_16, F2I::Equal)
                            }) != 0
                                && (if (((*v2_20).get_tag() == TAG_VARIANT_NUMERIC_INTEGER) as i32
                                    != 0) as i64
                                    != 0
                                {
                                    i2_16 = (*v2_20).value.integer;
                                    1
                                } else {
                                    luav_tointegerns(v2_20, &mut i2_16, F2I::Equal)
                                }) != 0
                            {
                                program_counter = program_counter.offset(1);
                                let io_39: *mut TValue = &mut (*ra_43).tvalue;
                                (*io_39).value.integer = luav_shiftl(i1_16, i2_16);
                                (*io_39).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                            }
                            continue;
                        }
                        46 => {
                            let ra_44: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            let pi: u32 = *program_counter.offset(-(2 as isize));
                            let rb_10: *mut TValue = &mut (*base.offset(
                                (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let tm: u32 =
                                (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0) as u32;
                            let result: StackValuePointer =
                                base.offset((pi >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            (*call_info).u.l.saved_program_counter = program_counter;
                            (*state).top.p = (*call_info).top.p;
                            luat_trybintm(state, &mut (*ra_44).tvalue, rb_10, result, tm);
                            trap = (*call_info).u.l.trap;
                            continue;
                        }
                        47 => {
                            let ra_45: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            let pi_0: u32 = *program_counter.offset(-(2 as isize));
                            let imm_0: i32 = (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32
                                - ((1 << 8) - 1 >> 1);
                            let tm_0: u32 =
                                (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0) as u32;
                            let flip: i32 = (i >> 0 + 7 + 8 & !(!(0u32) << 1) << 0) as i32;
                            let result_0: StackValuePointer =
                                base.offset((pi_0 >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            (*call_info).u.l.saved_program_counter = program_counter;
                            (*state).top.p = (*call_info).top.p;
                            luat_trybinitm(
                                state,
                                &mut (*ra_45).tvalue,
                                imm_0 as i64,
                                flip,
                                result_0,
                                tm_0,
                            );
                            trap = (*call_info).u.l.trap;
                            continue;
                        }
                        48 => {
                            let ra_46: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            let pi_1: u32 = *program_counter.offset(-(2 as isize));
                            let imm_1: *mut TValue = k.offset(
                                (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as isize,
                            );
                            let tm_1: u32 =
                                (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0) as u32;
                            let flip_0: i32 = (i >> 0 + 7 + 8 & !(!(0u32) << 1) << 0) as i32;
                            let result_1: StackValuePointer =
                                base.offset((pi_1 >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            (*call_info).u.l.saved_program_counter = program_counter;
                            (*state).top.p = (*call_info).top.p;
                            luat_trybinassoctm(
                                state,
                                &mut (*ra_46).tvalue,
                                imm_1,
                                flip_0,
                                result_1,
                                tm_1,
                            );
                            trap = (*call_info).u.l.trap;
                            continue;
                        }
                        49 => {
                            let ra_47: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            let rb_11: *mut TValue = &mut (*base.offset(
                                (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let mut nb_0: f64 = 0.0;
                            if (*rb_11).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                let ib_1: i64 = (*rb_11).value.integer;
                                let io_40: *mut TValue = &mut (*ra_47).tvalue;
                                (*io_40).value.integer = (0u64).wrapping_sub(ib_1 as u64) as i64;
                                (*io_40).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                            } else if if (*rb_11).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                                nb_0 = (*rb_11).value.number;
                                1
                            } else if (*rb_11).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                nb_0 = (*rb_11).value.integer as f64;
                                1
                            } else {
                                0
                            } != 0
                            {
                                let io_41: *mut TValue = &mut (*ra_47).tvalue;
                                (*io_41).value.number = -nb_0;
                                (*io_41).set_tag(TAG_VARIANT_NUMERIC_NUMBER);
                            } else {
                                (*call_info).u.l.saved_program_counter = program_counter;
                                (*state).top.p = (*call_info).top.p;
                                luat_trybintm(state, rb_11, rb_11, ra_47, TM_UNM);
                                trap = (*call_info).u.l.trap;
                            }
                            continue;
                        }
                        50 => {
                            let ra_48: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            let rb_12: *mut TValue = &mut (*base.offset(
                                (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let mut ib_2: i64 = 0;
                            if if (((*rb_12).get_tag() == TAG_VARIANT_NUMERIC_INTEGER) as i32 != 0)
                                as i64
                                != 0
                            {
                                ib_2 = (*rb_12).value.integer;
                                1
                            } else {
                                luav_tointegerns(rb_12, &mut ib_2, F2I::Equal)
                            } != 0
                            {
                                let io_42: *mut TValue = &mut (*ra_48).tvalue;
                                (*io_42).value.integer = (!(0u64) ^ ib_2 as u64) as i64;
                                (*io_42).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                            } else {
                                (*call_info).u.l.saved_program_counter = program_counter;
                                (*state).top.p = (*call_info).top.p;
                                luat_trybintm(state, rb_12, rb_12, ra_48, TM_BNOT);
                                trap = (*call_info).u.l.trap;
                            }
                            continue;
                        }
                        51 => {
                            let ra_49: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            let rb_13: *mut TValue = &mut (*base.offset(
                                (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            if (*rb_13).get_tag() == TAG_VARIANT_BOOLEAN_FALSE
                                || get_tag_type((*rb_13).get_tag()) == TAG_TYPE_NIL
                            {
                                (*ra_49).tvalue.set_tag(TAG_VARIANT_BOOLEAN_TRUE);
                            } else {
                                (*ra_49).tvalue.set_tag(TAG_VARIANT_BOOLEAN_FALSE);
                            }
                            continue;
                        }
                        OP_LEN => {
                            let ra_50: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            (*call_info).u.l.saved_program_counter = program_counter;
                            (*state).top.p = (*call_info).top.p;
                            luav_objlen(
                                state,
                                ra_50,
                                &mut (*base.offset(
                                    (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as isize,
                                ))
                                .tvalue,
                            );
                            trap = (*call_info).u.l.trap;
                            continue;
                        }
                        53 => {
                            let ra_51: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            let n_1: i32 = (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32;
                            (*state).top.p = ra_51.offset(n_1 as isize);
                            (*call_info).u.l.saved_program_counter = program_counter;
                            concatenate(state, n_1);
                            trap = (*call_info).u.l.trap;
                            if (*(*state).global).gc_debt > 0 {
                                (*call_info).u.l.saved_program_counter = program_counter;
                                (*state).top.p = (*state).top.p;
                                luac_step(state);
                                trap = (*call_info).u.l.trap;
                            }
                            continue;
                        }
                        54 => {
                            let ra_52: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            (*call_info).u.l.saved_program_counter = program_counter;
                            (*state).top.p = (*call_info).top.p;
                            luaf_close(state, ra_52, 0, 1);
                            trap = (*call_info).u.l.trap;
                            continue;
                        }
                        55 => {
                            let ra_53: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            (*call_info).u.l.saved_program_counter = program_counter;
                            (*state).top.p = (*call_info).top.p;
                            luaf_newtbcupval(state, ra_53);
                            continue;
                        }
                        56 => {
                            program_counter = program_counter.offset(
                                ((i >> 0 + 7 & !(!(0u32) << 8 + 8 + 1 + 8) << 0) as i32
                                    - ((1 << 8 + 8 + 1 + 8) - 1 >> 1)
                                    + 0) as isize,
                            );
                            trap = (*call_info).u.l.trap;
                            continue;
                        }
                        57 => {
                            let ra_54: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            let cond_0: i32;
                            let rb_14: *mut TValue = &mut (*base.offset(
                                (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            (*call_info).u.l.saved_program_counter = program_counter;
                            (*state).top.p = (*call_info).top.p;
                            cond_0 = if luav_equalobj(state, &mut (*ra_54).tvalue, rb_14) { 1 } else { 0 };
                            trap = (*call_info).u.l.trap;
                            if cond_0 != (i >> 0 + 7 + 8 & !(!(0u32) << 1) << 0) as i32 {
                                program_counter = program_counter.offset(1);
                            } else {
                                let ni: u32 = *program_counter;
                                program_counter = program_counter.offset(
                                    ((ni >> 0 + 7 & !(!(0u32) << 8 + 8 + 1 + 8) << 0) as i32
                                        - ((1 << 8 + 8 + 1 + 8) - 1 >> 1)
                                        + 1) as isize,
                                );
                                trap = (*call_info).u.l.trap;
                            }
                            continue;
                        }
                        58 => {
                            let ra_55: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            let cond_1: i32;
                            let rb_15: *mut TValue = &mut (*base.offset(
                                (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            if (*ra_55).tvalue.get_tag() == TAG_VARIANT_NUMERIC_INTEGER
                                && (*rb_15).get_tag() == TAG_VARIANT_NUMERIC_INTEGER
                            {
                                let ia: i64 = (*ra_55).tvalue.value.integer;
                                let ib_3: i64 = (*rb_15).value.integer;
                                cond_1 = (ia < ib_3) as i32;
                            } else if get_tag_type((*ra_55).tvalue.get_tag()) == TAG_TYPE_NUMERIC
                                && get_tag_type((*rb_15).get_tag()) == TAG_TYPE_NUMERIC
                            {
                                cond_1 = if ltnum(&mut (*ra_55).tvalue, rb_15) { 1 } else { 0 };
                            } else {
                                (*call_info).u.l.saved_program_counter = program_counter;
                                (*state).top.p = (*call_info).top.p;
                                cond_1 = lessthanothers(state, &mut (*ra_55).tvalue, rb_15);
                                trap = (*call_info).u.l.trap;
                            }
                            if cond_1 != (i >> 0 + 7 + 8 & !(!(0u32) << 1) << 0) as i32 {
                                program_counter = program_counter.offset(1);
                            } else {
                                let ni_0: u32 = *program_counter;
                                program_counter = program_counter.offset(
                                    ((ni_0 >> 0 + 7 & !(!(0u32) << 8 + 8 + 1 + 8) << 0) as i32
                                        - ((1 << 8 + 8 + 1 + 8) - 1 >> 1)
                                        + 1) as isize,
                                );
                                trap = (*call_info).u.l.trap;
                            }
                            continue;
                        }
                        OP_LE => {
                            let ra_56: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            let cond_2: i32;
                            let rb_16: *mut TValue = &mut (*base.offset(
                                (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            if (*ra_56).tvalue.get_tag() == TAG_VARIANT_NUMERIC_INTEGER
                                && (*rb_16).get_tag() == TAG_VARIANT_NUMERIC_INTEGER
                            {
                                let ia_0: i64 = (*ra_56).tvalue.value.integer;
                                let ib_4: i64 = (*rb_16).value.integer;
                                cond_2 = (ia_0 <= ib_4) as i32;
                            } else if get_tag_type((*ra_56).tvalue.get_tag()) == TAG_TYPE_NUMERIC
                                && get_tag_type((*rb_16).get_tag()) == TAG_TYPE_NUMERIC
                            {
                                cond_2 = if lenum(&mut (*ra_56).tvalue, rb_16) { 1 } else { 0 };
                            } else {
                                (*call_info).u.l.saved_program_counter = program_counter;
                                (*state).top.p = (*call_info).top.p;
                                cond_2 = if lessequalothers(state, &mut (*ra_56).tvalue, rb_16) { 1 } else { 0 };
                                trap = (*call_info).u.l.trap;
                            }
                            if cond_2 != (i >> 0 + 7 + 8 & !(!(0u32) << 1) << 0) as i32 {
                                program_counter = program_counter.offset(1);
                            } else {
                                let ni_1: u32 = *program_counter;
                                program_counter = program_counter.offset(
                                    ((ni_1 >> 0 + 7 & !(!(0u32) << 8 + 8 + 1 + 8) << 0) as i32
                                        - ((1 << 8 + 8 + 1 + 8) - 1 >> 1)
                                        + 1) as isize,
                                );
                                trap = (*call_info).u.l.trap;
                            }
                            continue;
                        }
                        60 => {
                            let ra_57: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            let rb_17: *mut TValue = k.offset(
                                (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as isize,
                            );
                            let cond_3: i32 =
                                if luav_equalobj(std::ptr::null_mut(), &mut (*ra_57).tvalue, rb_17) { 1 } else { 0 };
                            if cond_3 != (i >> 0 + 7 + 8 & !(!(0u32) << 1) << 0) as i32 {
                                program_counter = program_counter.offset(1);
                            } else {
                                let ni_2: u32 = *program_counter;
                                program_counter = program_counter.offset(
                                    ((ni_2 >> 0 + 7 & !(!(0u32) << 8 + 8 + 1 + 8) << 0) as i32
                                        - ((1 << 8 + 8 + 1 + 8) - 1 >> 1)
                                        + 1) as isize,
                                );
                                trap = (*call_info).u.l.trap;
                            }
                            continue;
                        }
                        61 => {
                            let ra_58: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            let cond_4: i32;
                            let im: i32 = (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32
                                - ((1 << 8) - 1 >> 1);
                            if (*ra_58).tvalue.get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                cond_4 = ((*ra_58).tvalue.value.integer == im as i64) as i32;
                            } else if (*ra_58).tvalue.get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                                cond_4 = ((*ra_58).tvalue.value.number == im as f64) as i32;
                            } else {
                                cond_4 = 0;
                            }
                            if cond_4 != (i >> 0 + 7 + 8 & !(!(0u32) << 1) << 0) as i32 {
                                program_counter = program_counter.offset(1);
                            } else {
                                let ni_3: u32 = *program_counter;
                                program_counter = program_counter.offset(
                                    ((ni_3 >> 0 + 7 & !(!(0u32) << 8 + 8 + 1 + 8) << 0) as i32
                                        - ((1 << 8 + 8 + 1 + 8) - 1 >> 1)
                                        + 1) as isize,
                                );
                                trap = (*call_info).u.l.trap;
                            }
                            continue;
                        }
                        62 => {
                            let ra_59: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            let cond_5: i32;
                            let im_0: i32 = (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32
                                - ((1 << 8) - 1 >> 1);
                            if (*ra_59).tvalue.get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                cond_5 = ((*ra_59).tvalue.value.integer < im_0 as i64) as i32;
                            } else if (*ra_59).tvalue.get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                                let fa: f64 = (*ra_59).tvalue.value.number;
                                let fim: f64 = im_0 as f64;
                                cond_5 = (fa < fim) as i32;
                            } else {
                                let isf: bool =
                                    (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0) != 0;
                                (*call_info).u.l.saved_program_counter = program_counter;
                                (*state).top.p = (*call_info).top.p;
                                cond_5 = luat_callorderitm(
                                    state,
                                    &mut (*ra_59).tvalue,
                                    im_0,
                                    0,
                                    isf,
                                    TM_LT,
                                );
                                trap = (*call_info).u.l.trap;
                            }
                            if cond_5 != (i >> 0 + 7 + 8 & !(!(0u32) << 1) << 0) as i32 {
                                program_counter = program_counter.offset(1);
                            } else {
                                let ni_4: u32 = *program_counter;
                                program_counter = program_counter.offset(
                                    ((ni_4 >> 0 + 7 & !(!(0u32) << 8 + 8 + 1 + 8) << 0) as i32
                                        - ((1 << 8 + 8 + 1 + 8) - 1 >> 1)
                                        + 1) as isize,
                                );
                                trap = (*call_info).u.l.trap;
                            }
                            continue;
                        }
                        OP_LEI => {
                            let ra_60: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            let cond_6: i32;
                            let im_1: i32 = (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32
                                - ((1 << 8) - 1 >> 1);
                            if (*ra_60).tvalue.get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                cond_6 = ((*ra_60).tvalue.value.integer <= im_1 as i64) as i32;
                            } else if (*ra_60).tvalue.get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                                let fa_0: f64 = (*ra_60).tvalue.value.number;
                                let fim_0: f64 = im_1 as f64;
                                cond_6 = (fa_0 <= fim_0) as i32;
                            } else {
                                let isf_0: bool =
                                    (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0) != 0;
                                (*call_info).u.l.saved_program_counter = program_counter;
                                (*state).top.p = (*call_info).top.p;
                                cond_6 = luat_callorderitm(
                                    state,
                                    &mut (*ra_60).tvalue,
                                    im_1,
                                    0,
                                    isf_0,
                                    TM_LE,
                                );
                                trap = (*call_info).u.l.trap;
                            }
                            if cond_6 != (i >> 0 + 7 + 8 & !(!(0u32) << 1) << 0) as i32 {
                                program_counter = program_counter.offset(1);
                            } else {
                                let ni_5: u32 = *program_counter;
                                program_counter = program_counter.offset(
                                    ((ni_5 >> 0 + 7 & !(!(0u32) << 8 + 8 + 1 + 8) << 0) as i32
                                        - ((1 << 8 + 8 + 1 + 8) - 1 >> 1)
                                        + 1) as isize,
                                );
                                trap = (*call_info).u.l.trap;
                            }
                            continue;
                        }
                        64 => {
                            let ra_61: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            let cond_7: i32;
                            let im_2: i32 = (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32
                                - ((1 << 8) - 1 >> 1);
                            if (*ra_61).tvalue.get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                cond_7 = ((*ra_61).tvalue.value.integer > im_2 as i64) as i32;
                            } else if (*ra_61).tvalue.get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                                let fa_1: f64 = (*ra_61).tvalue.value.number;
                                let fim_1: f64 = im_2 as f64;
                                cond_7 = (fa_1 > fim_1) as i32;
                            } else {
                                let isf_1: bool =
                                    (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0) != 0;
                                (*call_info).u.l.saved_program_counter = program_counter;
                                (*state).top.p = (*call_info).top.p;
                                cond_7 = luat_callorderitm(
                                    state,
                                    &mut (*ra_61).tvalue,
                                    im_2,
                                    1,
                                    isf_1,
                                    TM_LT,
                                );
                                trap = (*call_info).u.l.trap;
                            }
                            if cond_7 != (i >> 0 + 7 + 8 & !(!(0u32) << 1) << 0) as i32 {
                                program_counter = program_counter.offset(1);
                            } else {
                                let ni_6: u32 = *program_counter;
                                program_counter = program_counter.offset(
                                    ((ni_6 >> 0 + 7 & !(!(0u32) << 8 + 8 + 1 + 8) << 0) as i32
                                        - ((1 << 8 + 8 + 1 + 8) - 1 >> 1)
                                        + 1) as isize,
                                );
                                trap = (*call_info).u.l.trap;
                            }
                            continue;
                        }
                        OP_GEI => {
                            let ra_62: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            let cond_8: i32;
                            let im_3: i32 = (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32
                                - ((1 << 8) - 1 >> 1);
                            if (*ra_62).tvalue.get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
                                cond_8 = ((*ra_62).tvalue.value.integer >= im_3 as i64) as i32;
                            } else if (*ra_62).tvalue.get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
                                let fa_2: f64 = (*ra_62).tvalue.value.number;
                                let fim_2: f64 = im_3 as f64;
                                cond_8 = (fa_2 >= fim_2) as i32;
                            } else {
                                let isf_2: bool =
                                    (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0) != 0;
                                (*call_info).u.l.saved_program_counter = program_counter;
                                (*state).top.p = (*call_info).top.p;
                                cond_8 = luat_callorderitm(
                                    state,
                                    &mut (*ra_62).tvalue,
                                    im_3,
                                    1,
                                    isf_2,
                                    TM_LE,
                                );
                                trap = (*call_info).u.l.trap;
                            }
                            if cond_8 != (i >> 0 + 7 + 8 & !(!(0u32) << 1) << 0) as i32 {
                                program_counter = program_counter.offset(1);
                            } else {
                                let ni_7: u32 = *program_counter;
                                program_counter = program_counter.offset(
                                    ((ni_7 >> 0 + 7 & !(!(0u32) << 8 + 8 + 1 + 8) << 0) as i32
                                        - ((1 << 8 + 8 + 1 + 8) - 1 >> 1)
                                        + 1) as isize,
                                );
                                trap = (*call_info).u.l.trap;
                            }
                            continue;
                        }
                        66 => {
                            let ra_63: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            let cond_9: i32 = !((*ra_63).tvalue.get_tag()
                                == TAG_VARIANT_BOOLEAN_FALSE
                                || get_tag_type((*ra_63).tvalue.get_tag()) == TAG_TYPE_NIL)
                                as i32;
                            if cond_9 != (i >> 0 + 7 + 8 & !(!(0u32) << 1) << 0) as i32 {
                                program_counter = program_counter.offset(1);
                            } else {
                                let ni_8: u32 = *program_counter;
                                program_counter = program_counter.offset(
                                    ((ni_8 >> 0 + 7 & !(!(0u32) << 8 + 8 + 1 + 8) << 0) as i32
                                        - ((1 << 8 + 8 + 1 + 8) - 1 >> 1)
                                        + 1) as isize,
                                );
                                trap = (*call_info).u.l.trap;
                            }
                            continue;
                        }
                        67 => {
                            let ra_64: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            let rb_18: *mut TValue = &mut (*base.offset(
                                (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            if ((*rb_18).get_tag() == TAG_VARIANT_BOOLEAN_FALSE
                                || get_tag_type((*rb_18).get_tag()) == TAG_TYPE_NIL)
                                as i32
                                == (i >> 0 + 7 + 8 & !(!(0u32) << 1) << 0) as i32
                            {
                                program_counter = program_counter.offset(1);
                            } else {
                                let io1_14: *mut TValue = &mut (*ra_64).tvalue;
                                let io2_14: *const TValue = rb_18;
                                (*io1_14).value = (*io2_14).value;
                                (*io1_14).set_tag((*io2_14).get_tag());
                                let ni_9: u32 = *program_counter;
                                program_counter = program_counter.offset(
                                    ((ni_9 >> 0 + 7 & !(!(0u32) << 8 + 8 + 1 + 8) << 0) as i32
                                        - ((1 << 8 + 8 + 1 + 8) - 1 >> 1)
                                        + 1) as isize,
                                );
                                trap = (*call_info).u.l.trap;
                            }
                            continue;
                        }
                        68 => {
                            ra_65 =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            b_4 = (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32;
                            count_results =
                                (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0) as i32 - 1;
                            if b_4 != 0 {
                                (*state).top.p = ra_65.offset(b_4 as isize);
                            }
                            (*call_info).u.l.saved_program_counter = program_counter;
                            new_call_info = luad_precall(state, ra_65, count_results);
                            if !new_call_info.is_null() {
                                break '_returning;
                            }
                            trap = (*call_info).u.l.trap;
                            continue;
                        }
                        69 => {
                            let ra_66: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            let mut b_5: i32 = (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32;
                            let n_2: i32;
                            let nparams1: i32 =
                                (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0) as i32;
                            let delta: i32 = if nparams1 != 0 {
                                (*call_info).u.l.count_extra_arguments + nparams1
                            } else {
                                0
                            };
                            if b_5 != 0 {
                                (*state).top.p = ra_66.offset(b_5 as isize);
                            } else {
                                b_5 = ((*state).top.p).offset_from(ra_66) as i32;
                            }
                            (*call_info).u.l.saved_program_counter = program_counter;
                            if (i & (1 as u32) << 0 + 7 + 8) as i32 != 0 {
                                luaf_closeupval(state, base);
                            }
                            n_2 = luad_pretailcall(state, call_info, ra_66, b_5, delta);
                            if n_2 < 0 {
                                continue '_startfunc;
                            }
                            (*call_info).function.p =
                                ((*call_info).function.p).offset(-(delta as isize));
                            luad_poscall(state, call_info, n_2);
                            trap = (*call_info).u.l.trap;
                            break;
                        }
                        70 => {
                            let mut ra_67: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            let mut n_3: i32 =
                                (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32 - 1;
                            let nparams1_0: i32 =
                                (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0) as i32;
                            if n_3 < 0 {
                                n_3 = ((*state).top.p).offset_from(ra_67) as i32;
                            }
                            (*call_info).u.l.saved_program_counter = program_counter;
                            if (i & (1 as u32) << 0 + 7 + 8) as i32 != 0 {
                                (*call_info).u2.nres = n_3;
                                if (*state).top.p < (*call_info).top.p {
                                    (*state).top.p = (*call_info).top.p;
                                }
                                luaf_close(state, base, -1, 1);
                                trap = (*call_info).u.l.trap;
                                if (trap != 0) as i64 != 0 {
                                    base = ((*call_info).function.p).offset(1 as isize);
                                    ra_67 = base.offset(
                                        (i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize,
                                    );
                                }
                            }
                            if nparams1_0 != 0 {
                                (*call_info).function.p = ((*call_info).function.p).offset(
                                    -(((*call_info).u.l.count_extra_arguments + nparams1_0)
                                        as isize),
                                );
                            }
                            (*state).top.p = ra_67.offset(n_3 as isize);
                            luad_poscall(state, call_info, n_3);
                            trap = (*call_info).u.l.trap;
                            break;
                        }
                        71 => {
                            if ((*state).hook_mask != 0) as i64 != 0 {
                                let ra_68: StackValuePointer = base
                                    .offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                                (*state).top.p = ra_68;
                                (*call_info).u.l.saved_program_counter = program_counter;
                                luad_poscall(state, call_info, 0);
                                trap = 1;
                            } else {
                                let mut nres: i32;
                                (*state).call_info = (*call_info).previous;
                                (*state).top.p = base.offset(-(1 as isize));
                                nres = (*call_info).count_results as i32;
                                while ((nres > 0) as i32 != 0) as i64 != 0 {
                                    let fresh141 = (*state).top.p;
                                    (*state).top.p = (*state).top.p.offset(1);
                                    (*fresh141).tvalue.set_tag(TAG_VARIANT_NIL_NIL);
                                    nres -= 1;
                                }
                            }
                            break;
                        }
                        72 => {
                            if ((*state).hook_mask != 0) as i64 != 0 {
                                let ra_69: StackValuePointer = base
                                    .offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                                (*state).top.p = ra_69.offset(1 as isize);
                                (*call_info).u.l.saved_program_counter = program_counter;
                                luad_poscall(state, call_info, 1);
                                trap = 1;
                            } else {
                                let mut nres_0: i32 = (*call_info).count_results as i32;
                                (*state).call_info = (*call_info).previous;
                                if nres_0 == 0 {
                                    (*state).top.p = base.offset(-(1 as isize));
                                } else {
                                    let ra_70: StackValuePointer = base.offset(
                                        (i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize,
                                    );
                                    let io1_15: *mut TValue =
                                        &mut (*base.offset(-(1 as isize))).tvalue;
                                    let io2_15: *const TValue = &mut (*ra_70).tvalue;
                                    (*io1_15).value = (*io2_15).value;
                                    (*io1_15).set_tag((*io2_15).get_tag());
                                    (*state).top.p = base;
                                    while ((nres_0 > 1) as i32 != 0) as i64 != 0 {
                                        let fresh142 = (*state).top.p;
                                        (*state).top.p = (*state).top.p.offset(1);
                                        (*fresh142).tvalue.set_tag(TAG_VARIANT_NIL_NIL);
                                        nres_0 -= 1;
                                    }
                                }
                            }
                            break;
                        }
                        73 => {
                            let ra_71: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            if (*ra_71.offset(2 as isize)).tvalue.get_tag()
                                == TAG_VARIANT_NUMERIC_INTEGER
                            {
                                let count: u64 = (*ra_71.offset(1 as isize)).tvalue.value.integer as u64;
                                if count > 0u64 {
                                    let step: i64 = (*ra_71.offset(2 as isize)).tvalue.value.integer;
                                    let mut index: i64 = (*ra_71).tvalue.value.integer;
                                    let io_43: *mut TValue = &mut (*ra_71.offset(1 as isize)).tvalue;
                                    (*io_43).value.integer = count.wrapping_sub(1 as u64) as i64;
                                    index = (index as u64).wrapping_add(step as u64) as i64;
                                    let io_44: *mut TValue = &mut (*ra_71).tvalue;
                                    (*io_44).value.integer = index;
                                    let io_45: *mut TValue = &mut (*ra_71.offset(3 as isize)).tvalue;
                                    (*io_45).value.integer = index;
                                    (*io_45).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                                    program_counter = program_counter.offset(
                                        -((i >> 0 + 7 + 8 & !(!(0u32) << 8 + 8 + 1) << 0) as i32
                                            as isize),
                                    );
                                }
                            } else if floatforloop(ra_71) != 0 {
                                program_counter = program_counter.offset(
                                    -((i >> 0 + 7 + 8 & !(!(0u32) << 8 + 8 + 1) << 0) as i32
                                        as isize),
                                );
                            }
                            trap = (*call_info).u.l.trap;
                            continue;
                        }
                        74 => {
                            let ra_72: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            (*call_info).u.l.saved_program_counter = program_counter;
                            (*state).top.p = (*call_info).top.p;
                            if forprep(state, ra_72) != 0 {
                                program_counter = program_counter.offset(
                                    ((i >> 0 + 7 + 8 & !(!(0u32) << 8 + 8 + 1) << 0) as i32 + 1)
                                        as isize,
                                );
                            }
                            continue;
                        }
                        75 => {
                            let ra_73: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            (*call_info).u.l.saved_program_counter = program_counter;
                            (*state).top.p = (*call_info).top.p;
                            luaf_newtbcupval(state, ra_73.offset(3 as isize));
                            program_counter = program_counter.offset(
                                (i >> 0 + 7 + 8 & !(!(0u32) << 8 + 8 + 1) << 0) as isize,
                            );
                            let fresh143 = program_counter;
                            program_counter = program_counter.offset(1);
                            i = *fresh143;
                            current_block = 13973394567113199817;
                        }
                        76 => {
                            current_block = 13973394567113199817;
                        }
                        77 => {
                            current_block = 15611964311717037170;
                        }
                        78 => {
                            let ra_76: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            let mut n_4: i32 = (i >> 0 + 7 + 8 + 1 & !(!(0u32) << 8) << 0) as i32;
                            let mut last: u32 =
                                (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0) as u32;
                            let h: *mut Table =
                                &mut (*((*ra_76).tvalue.value.object as *mut Table));
                            if n_4 == 0 {
                                n_4 = ((*state).top.p).offset_from(ra_76) as i32 - 1;
                            } else {
                                (*state).top.p = (*call_info).top.p;
                            }
                            last = last.wrapping_add(n_4 as u32);
                            if (i & (1 as u32) << 0 + 7 + 8) as i32 != 0 {
                                last = last.wrapping_add(
                                    ((*program_counter >> 0 + 7 & !(!(0u32) << 8 + 8 + 1 + 8) << 0)
                                        as i32
                                        * ((1 << 8) - 1 + 1))
                                        as u32,
                                );
                                program_counter = program_counter.offset(1);
                            }
                            if last > luah_realasize(h) {
                                luah_resizearray(state, h, last);
                            }
                            while n_4 > 0 {
                                let value: *mut TValue = &mut (*ra_76.offset(n_4 as isize)).tvalue;
                                let io1_17: *mut TValue = &mut *((*h).array)
                                    .offset(last.wrapping_sub(1 as u32) as isize)
                                    as *mut TValue;
                                let io2_17: *const TValue = value;
                                (*io1_17).value = (*io2_17).value;
                                (*io1_17).set_tag((*io2_17).get_tag());
                                last = last.wrapping_sub(1);
                                if (*value).is_collectable() {
                                    if (*(h as *mut Object)).get_marked() & 1 << 5 != 0
                                        && (*(*value).value.object).get_marked() & (1 << 3 | 1 << 4)
                                            != 0
                                    {
                                        luac_barrierback_(
                                            state,
                                            &mut (*(h as *mut Object)),
                                        );
                                    } else {
                                    };
                                } else {
                                };
                                n_4 -= 1;
                            }
                            continue;
                        }
                        79 => {
                            let ra_77: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            let p: *mut Prototype = *((*(*cl).payload.l_prototype).p).offset(
                                (i >> 0 + 7 + 8 & !(!(0u32) << 8 + 8 + 1) << 0) as isize,
                            );
                            (*call_info).u.l.saved_program_counter = program_counter;
                            (*state).top.p = (*call_info).top.p;
                            pushclosure(state, p, ((*cl).upvalues).l_upvalues.as_mut_ptr(), base, ra_77);
                            if (*(*state).global).gc_debt > 0 {
                                (*call_info).u.l.saved_program_counter = program_counter;
                                (*state).top.p = ra_77.offset(1 as isize);
                                luac_step(state);
                                trap = (*call_info).u.l.trap;
                            }
                            continue;
                        }
                        80 => {
                            let ra_78: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            let n_5: i32 =
                                (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0) as i32 - 1;
                            (*call_info).u.l.saved_program_counter = program_counter;
                            (*state).top.p = (*call_info).top.p;
                            luat_getvarargs(state, call_info, ra_78, n_5);
                            trap = (*call_info).u.l.trap;
                            continue;
                        }
                        81 => {
                            (*call_info).u.l.saved_program_counter = program_counter;
                            luat_adjustvarargs(
                                state,
                                (i >> 0 + 7 & !(!(0u32) << 8) << 0) as i32,
                                call_info,
                                (*cl).payload.l_prototype,
                            );
                            trap = (*call_info).u.l.trap;
                            if (trap != 0) as i64 != 0 {
                                luad_hookcall(state, call_info);
                                (*state).old_program_counter = 1;
                            }
                            base = ((*call_info).function.p).offset(1 as isize);
                            continue;
                        }
                        82 | _ => {
                            continue;
                        }
                    }
                    match current_block {
                        13973394567113199817 => {
                            let ra_74: StackValuePointer =
                                base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                            memcpy(
                                ra_74.offset(4 as isize) as *mut libc::c_void,
                                ra_74 as *const libc::c_void,
                                (3 as u64)
                                    .wrapping_mul(::core::mem::size_of::<StackValue>() as u64),
                            );
                            (*state).top.p = ra_74.offset(4 as isize).offset(3 as isize);
                            (*call_info).u.l.saved_program_counter = program_counter;
                            ccall(
                                state,
                                ra_74.offset(4 as isize),
                                (i >> 0 + 7 + 8 + 1 + 8 & !(!(0u32) << 8) << 0) as i32,
                                1,
                            );
                            trap = (*call_info).u.l.trap;
                            if (trap != 0) as i64 != 0 {
                                base = ((*call_info).function.p).offset(1 as isize);
                            }
                            let fresh144 = program_counter;
                            program_counter = program_counter.offset(1);
                            i = *fresh144;
                        }
                        _ => {}
                    }
                    let ra_75: StackValuePointer =
                        base.offset((i >> 0 + 7 & !(!(0u32) << 8) << 0) as isize);
                    if get_tag_type((*ra_75.offset(4 as isize)).tvalue.get_tag()) != TAG_TYPE_NIL {
                        let io1_16: *mut TValue = &mut (*ra_75.offset(2 as isize)).tvalue;
                        let io2_16: *const TValue = &mut (*ra_75.offset(4 as isize)).tvalue;
                        (*io1_16).value = (*io2_16).value;
                        (*io1_16).set_tag((*io2_16).get_tag());
                        program_counter = program_counter.offset(
                            -((i >> 0 + 7 + 8 & !(!(0u32) << 8 + 8 + 1) << 0) as isize),
                        );
                    }
                }
                if (*call_info).call_status as i32 & 1 << 2 != 0 {
                    break '_startfunc;
                }
                call_info = (*call_info).previous;
            }
            call_info = new_call_info;
        }
    }
}
pub unsafe extern "C" fn findfield(state: *mut State, objidx: i32, level: i32) -> bool {
    unsafe {
        if level == 0 || (lua_type(state, -1) != Some(TAG_TYPE_TABLE)) {
            return false;
        }
        (*state).push_nil();
        while lua_next(state, -2) != 0 {
            if lua_type(state, -2) == Some(TAG_TYPE_STRING) {
                if lua_rawequal(state, objidx, -1) {
                    lua_settop(state, -2);
                    return true;
                } else if findfield(state, objidx, level - 1) {
                    lua_pushstring(state, b".\0" as *const u8 as *const i8);
                    lua_copy(state, -1, -3);
                    lua_settop(state, -2);
                    lua_concat(state, 3);
                    return true;
                }
            }
            lua_settop(state, -2);
        }
        return false;
    }
}
pub unsafe extern "C" fn pushglobalfuncname(state: *mut State, ar: *mut DebugInfo) -> bool {
    unsafe {
        let top: i32 = (*state).get_top();
        lua_getinfo(state, b"f\0" as *const u8 as *const i8, ar);
        lua_getfield(
            state,
            -(1000000 as i32) - 1000 as i32,
            b"_LOADED\0" as *const u8 as *const i8,
        );
        lual_checkstack(state, 6, b"not enough stack\0" as *const u8 as *const i8);
        if findfield(state, top + 1, 2) {
            let name: *const i8 = lua_tolstring(state, -1, std::ptr::null_mut());
            if strncmp(name, b"_G.\0" as *const u8 as *const i8, 3 as u64) == 0 {
                lua_pushstring(state, name.offset(3 as isize));
                lua_rotate(state, -2, -1);
                lua_settop(state, -2);
            }
            lua_copy(state, -1, top + 1);
            lua_settop(state, top + 1);
            return true;
        } else {
            lua_settop(state, top);
            return false;
        };
    }
}
pub unsafe extern "C" fn pushfuncname(state: *mut State, ar: *mut DebugInfo) {
    unsafe {
        if pushglobalfuncname(state, ar) {
            lua_pushfstring(
                state,
                b"function '%s'\0" as *const u8 as *const i8,
                lua_tolstring(state, -1, std::ptr::null_mut()),
            );
            lua_rotate(state, -2, -1);
            lua_settop(state, -2);
        } else if *(*ar).namewhat as i32 != '\0' as i32 {
            lua_pushfstring(
                state,
                b"%s '%s'\0" as *const u8 as *const i8,
                (*ar).namewhat,
                (*ar).name,
            );
        } else if *(*ar).what as i32 == 'm' as i32 {
            lua_pushstring(state, b"main chunk\0" as *const u8 as *const i8);
        } else if *(*ar).what as i32 != 'C' as i32 {
            lua_pushfstring(
                state,
                b"function <%s:%d>\0" as *const u8 as *const i8,
                ((*ar).short_src).as_mut_ptr(),
                (*ar).line_defined,
            );
        } else {
            lua_pushstring(state, b"?\0" as *const u8 as *const i8);
        };
    }
}
pub unsafe extern "C" fn lastlevel(state: *mut State) -> i32 {
    unsafe {
        let mut ar: DebugInfo = DebugInfo {
            event: 0,
            name: std::ptr::null(),
            namewhat: std::ptr::null(),
            what: std::ptr::null(),
            source: std::ptr::null(),
            source_length: 0,
            currentline: 0,
            line_defined: 0,
            last_line_defined: 0,
            nups: 0,
            nparams: 0,
            is_variable_arguments: false,
            is_tail_call: false,
            ftransfer: 0,
            ntransfer: 0,
            short_src: [0; 60],
            i_ci: std::ptr::null_mut(),
        };
        let mut li: i32 = 1;
        let mut le: i32 = 1;
        while lua_getstack(state, le, &mut ar) != 0 {
            li = le;
            le *= 2;
        }
        while li < le {
            let m: i32 = (li + le) / 2;
            if lua_getstack(state, m, &mut ar) != 0 {
                li = m + 1;
            } else {
                le = m;
            }
        }
        return le - 1;
    }
}
pub unsafe extern "C" fn lual_traceback(
    state: *mut State,
    other_state: *mut State,
    message: *const i8,
    mut level: i32,
) {
    unsafe {
        let mut b = Buffer::new();
        let mut ar: DebugInfo = DebugInfo {
            event: 0,
            name: std::ptr::null(),
            namewhat: std::ptr::null(),
            what: std::ptr::null(),
            source: std::ptr::null(),
            source_length: 0,
            currentline: 0,
            line_defined: 0,
            last_line_defined: 0,
            nups: 0,
            nparams: 0,
            is_variable_arguments: false,
            is_tail_call: false,
            ftransfer: 0,
            ntransfer: 0,
            short_src: [0; 60],
            i_ci: std::ptr::null_mut(),
        };
        let last: i32 = lastlevel(other_state);
        let mut limit2show: i32 = if last - level > 10 as i32 + 11 as i32 {
            10 as i32
        } else {
            -1
        };
        b.initialize(state);
        if !message.is_null() {
            b.add_string(message);
            (b.length < b.size || !(b.prepare_with_size(1 as u64)).is_null()) as i32;
            let fresh145 = b.length;
            b.length = (b.length).wrapping_add(1);
            *(b.pointer).offset(fresh145 as isize) = '\n' as i8;
        }
        b.add_string(b"stack traceback:\0" as *const u8 as *const i8);
        loop {
            let fresh146 = level;
            level = level + 1;
            if !(lua_getstack(other_state, fresh146, &mut ar) != 0) {
                break;
            }
            let fresh147 = limit2show;
            limit2show = limit2show - 1;
            if fresh147 == 0 {
                let n: i32 = last - level - 11 as i32 + 1;
                lua_pushfstring(
                    state,
                    b"\n\t...\t(skipping %d levels)\0" as *const u8 as *const i8,
                    n,
                );
                b.add_value();
                level += n;
            } else {
                lua_getinfo(other_state, b"Slnt\0" as *const u8 as *const i8, &mut ar);
                if ar.currentline <= 0 {
                    lua_pushfstring(
                        state,
                        b"\n\t%s: in \0" as *const u8 as *const i8,
                        (ar.short_src).as_mut_ptr(),
                    );
                } else {
                    lua_pushfstring(
                        state,
                        b"\n\t%s:%d: in \0" as *const u8 as *const i8,
                        (ar.short_src).as_mut_ptr(),
                        ar.currentline,
                    );
                }
                b.add_value();
                pushfuncname(state, &mut ar);
                b.add_value();
                if ar.is_tail_call {
                    b.add_string(b"\n\t(...tail calls...)\0" as *const u8 as *const i8);
                }
            }
        }
        b.push_result();
    }
}
pub unsafe extern "C" fn lual_argerror(
    state: *mut State,
    mut arg: i32,
    extramsg: *const i8,
) -> i32 {
    unsafe {
        let mut ar: DebugInfo = DebugInfo {
            event: 0,
            name: std::ptr::null(),
            namewhat: std::ptr::null(),
            what: std::ptr::null(),
            source: std::ptr::null(),
            source_length: 0,
            currentline: 0,
            line_defined: 0,
            last_line_defined: 0,
            nups: 0,
            nparams: 0,
            is_variable_arguments: false,
            is_tail_call: false,
            ftransfer: 0,
            ntransfer: 0,
            short_src: [0; 60],
            i_ci: std::ptr::null_mut(),
        };
        if lua_getstack(state, 0, &mut ar) == 0 {
            return lual_error(
                state,
                b"bad argument #%d (%s)\0" as *const u8 as *const i8,
                arg,
                extramsg,
            );
        }
        lua_getinfo(state, b"n\0" as *const u8 as *const i8, &mut ar);
        if strcmp(ar.namewhat, b"method\0" as *const u8 as *const i8) == 0 {
            arg -= 1;
            if arg == 0 {
                return lual_error(
                    state,
                    b"calling '%s' on bad self (%s)\0" as *const u8 as *const i8,
                    ar.name,
                    extramsg,
                );
            }
        }
        if ar.name.is_null() {
            ar.name = if pushglobalfuncname(state, &mut ar) {
                lua_tolstring(state, -1, std::ptr::null_mut())
            } else {
                b"?\0" as *const u8 as *const i8
            };
        }
        return lual_error(
            state,
            b"bad argument #%d to '%s' (%s)\0" as *const u8 as *const i8,
            arg,
            ar.name,
            extramsg,
        );
    }
}
pub unsafe extern "C" fn lual_typeerror(state: *mut State, arg: i32, tname: *const i8) -> i32 {
    unsafe {
        let message: *const i8;
        let typearg: *const i8;
        if lual_getmetafield(state, arg, b"__name\0" as *const u8 as *const i8) == 4 {
            typearg = lua_tolstring(state, -1, std::ptr::null_mut());
        } else if lua_type(state, arg) == Some(TAG_TYPE_POINTER) {
            typearg = b"light userdata\0" as *const u8 as *const i8;
        } else {
            typearg = lua_typename(state, lua_type(state, arg));
        }
        message = lua_pushfstring(
            state,
            b"%s expected, got %s\0" as *const u8 as *const i8,
            tname,
            typearg,
        );
        return lual_argerror(state, arg, message);
    }
}
pub unsafe fn tag_error(state: *mut State, arg: i32, tag: Option<u8>) {
    unsafe {
        lual_typeerror(state, arg, lua_typename(state, tag));
    }
}
pub unsafe extern "C" fn lual_where(state: *mut State, level: i32) {
    unsafe {
        let mut ar: DebugInfo = DebugInfo {
            event: 0,
            name: std::ptr::null(),
            namewhat: std::ptr::null(),
            what: std::ptr::null(),
            source: std::ptr::null(),
            source_length: 0,
            currentline: 0,
            line_defined: 0,
            last_line_defined: 0,
            nups: 0,
            nparams: 0,
            is_variable_arguments: false,
            is_tail_call: false,
            ftransfer: 0,
            ntransfer: 0,
            short_src: [0; 60],
            i_ci: std::ptr::null_mut(),
        };
        if lua_getstack(state, level, &mut ar) != 0 {
            lua_getinfo(state, b"Sl\0" as *const u8 as *const i8, &mut ar);
            if ar.currentline > 0 {
                lua_pushfstring(
                    state,
                    b"%s:%d: \0" as *const u8 as *const i8,
                    (ar.short_src).as_mut_ptr(),
                    ar.currentline,
                );
                return;
            }
        }
        lua_pushfstring(state, b"\0" as *const u8 as *const i8);
    }
}
pub unsafe extern "C" fn lual_error(state: *mut State, fmt: *const i8, args: ...) -> i32 {
    unsafe {
        let mut argp: ::core::ffi::VaListImpl;
        argp = args.clone();
        lual_where(state, 1);
        lua_pushvfstring(state, fmt, argp.as_va_list());
        lua_concat(state, 2);
        return lua_error(state);
    }
}
pub unsafe extern "C" fn lual_fileresult(state: *mut State, stat: i32, fname: *const i8) -> i32 {
    unsafe {
        let en: i32 = *__errno_location();
        if stat != 0 {
            (*state).push_boolean(true);
            return 1;
        } else {
            let message: *const i8;
            (*state).push_nil();
            message = if en != 0 {
                strerror(en) as *const i8
            } else {
                b"(no extra info)\0" as *const u8 as *const i8
            };
            if !fname.is_null() {
                lua_pushfstring(state, b"%s: %s\0" as *const u8 as *const i8, fname, message);
            } else {
                lua_pushstring(state, message);
            }
            (*state).push_integer(en as i64);
            return 3;
        };
    }
}
pub unsafe extern "C" fn lual_execresult(state: *mut State, mut stat: i32) -> i32 {
    unsafe {
        if stat != 0 && *__errno_location() != 0 {
            return lual_fileresult(state, 0, std::ptr::null());
        } else {
            let mut what: *const i8 = b"exit\0" as *const u8 as *const i8;
            if stat & 0x7f as i32 == 0 {
                stat = (stat & 0xff00 as i32) >> 8;
            } else if ((stat & 0x7f as i32) + 1) as i32 >> 1 > 0 {
                stat = stat & 0x7f as i32;
                what = b"signal\0" as *const u8 as *const i8;
            }
            if *what as i32 == 'e' as i32 && stat == 0 {
                (*state).push_boolean(true);
            } else {
                (*state).push_nil();
            }
            lua_pushstring(state, what);
            (*state).push_integer(stat as i64);
            return 3;
        };
    }
}
pub unsafe extern "C" fn lual_newmetatable(state: *mut State, tname: *const i8) -> i32 {
    unsafe {
        if lua_getfield(state, -1000000 - 1000, tname) != 0 {
            return 0;
        }
        lua_settop(state, -2);
        (*state).lua_createtable();
        lua_pushstring(state, tname);
        lua_setfield(state, -2, b"__name\0" as *const u8 as *const i8);
        lua_pushvalue(state, -1);
        lua_setfield(state, -(1000000 as i32) - 1000 as i32, tname);
        return 1;
    }
}
pub unsafe extern "C" fn lual_setmetatable(state: *mut State, tname: *const i8) {
    unsafe {
        lua_getfield(state, -(1000000 as i32) - 1000 as i32, tname);
        lua_setmetatable(state, -2);
    }
}
pub unsafe extern "C" fn lual_testudata(
    state: *mut State,
    arbitrary_data: i32,
    tname: *const i8,
) -> *mut libc::c_void {
    unsafe {
        let mut p: *mut libc::c_void = lua_touserdata(state, arbitrary_data);
        if !p.is_null() {
            if (*state).lua_getmetatable(arbitrary_data) {
                lua_getfield(state, -(1000000 as i32) - 1000 as i32, tname);
                if !lua_rawequal(state, -1, -2) {
                    p = std::ptr::null_mut();
                }
                lua_settop(state, -2 - 1);
                return p;
            }
        }
        return std::ptr::null_mut();
    }
}
pub unsafe extern "C" fn lual_checkudata(
    state: *mut State,
    arbitrary_data: i32,
    tname: *const i8,
) -> *mut libc::c_void {
    unsafe {
        let p: *mut libc::c_void = lual_testudata(state, arbitrary_data, tname);
        (((p != std::ptr::null_mut()) as i32 != 0) as i64 != 0
            || lual_typeerror(state, arbitrary_data, tname) != 0) as i32;
        return p;
    }
}
pub unsafe extern "C" fn lual_checkoption(
    state: *mut State,
    arg: i32,
    def: *const i8,
    lst: *const *const i8,
) -> i32 {
    unsafe {
        let name: *const i8 = if !def.is_null() {
            lual_optlstring(state, arg, def, std::ptr::null_mut())
        } else {
            lual_checklstring(state, arg, std::ptr::null_mut())
        };
        let mut i: i32;
        i = 0;
        while !(*lst.offset(i as isize)).is_null() {
            if strcmp(*lst.offset(i as isize), name) == 0 {
                return i;
            }
            i += 1;
        }
        return lual_argerror(
            state,
            arg,
            lua_pushfstring(
                state,
                b"invalid option '%s'\0" as *const u8 as *const i8,
                name,
            ),
        );
    }
}
pub unsafe extern "C" fn lual_checkstack(state: *mut State, space: i32, message: *const i8) {
    unsafe {
        if ((lua_checkstack(state, space) == 0) as i32 != 0) as i64 != 0 {
            if !message.is_null() {
                lual_error(
                    state,
                    b"stack overflow (%s)\0" as *const u8 as *const i8,
                    message,
                );
            } else {
                lual_error(state, b"stack overflow\0" as *const u8 as *const i8);
            }
        }
    }
}
pub unsafe extern "C" fn lual_checktype(state: *mut State, arg: i32, tag: u8) {
    unsafe {
        if lua_type(state, arg) != Some(tag) {
            tag_error(state, arg, Some(tag));
        }
    }
}
pub unsafe extern "C" fn lual_checkany(state: *mut State, arg: i32) {
    unsafe {
        if lua_type(state, arg) == None {
            lual_argerror(state, arg, b"value expected\0" as *const u8 as *const i8);
        }
    }
}
pub unsafe extern "C" fn lual_checklstring(
    state: *mut State,
    arg: i32,
    length: *mut u64,
) -> *const i8 {
    unsafe {
        let s: *const i8 = lua_tolstring(state, arg, length);
        if (s.is_null() as i32 != 0) as i64 != 0 {
            tag_error(state, arg, Some(TAG_TYPE_STRING));
        }
        return s;
    }
}
pub unsafe extern "C" fn lual_optlstring(
    state: *mut State,
    arg: i32,
    def: *const i8,
    length: *mut u64,
) -> *const i8 {
    unsafe {
        match lua_type(state, arg) {
            None | Some(TAG_TYPE_NIL) => {
                if !length.is_null() {
                    *length = if !def.is_null() { strlen(def) } else { 0u64 };
                }
                return def;
            },
            _ => {
                return lual_checklstring(state, arg, length);
            },
        }
    }
}
pub unsafe extern "C" fn lual_checknumber(state: *mut State, arg: i32) -> f64 {
    unsafe {
        let mut is_number: bool = false;
        let d: f64 = lua_tonumberx(state, arg, &mut is_number);
        if !is_number {
            tag_error(state, arg, Some(TAG_TYPE_NUMERIC));
        }
        return d;
    }
}
pub unsafe extern "C" fn lual_optnumber(state: *mut State, arg: i32, def: f64) -> f64 {
    unsafe {
        match lua_type(state, arg) {
            None | Some(TAG_TYPE_NIL) => {
                def
            },
            _ => {
                lual_checknumber(state, arg)
            }
        }
    }
}
pub unsafe extern "C" fn interror(state: *mut State, arg: i32) {
    unsafe {
        if lua_isnumber(state, arg) {
            lual_argerror(
                state,
                arg,
                b"number has no integer representation\0" as *const u8 as *const i8,
            );
        } else {
            tag_error(state, arg, Some(TAG_TYPE_NUMERIC));
        };
    }
}
pub unsafe extern "C" fn lual_checkinteger(state: *mut State, arg: i32) -> i64 {
    unsafe {
        let mut is_number: bool = false;
        let ret: i64 = lua_tointegerx(state, arg, &mut is_number);
        if !is_number {
            interror(state, arg);
        }
        return ret;
    }
}
pub unsafe extern "C" fn lual_optinteger(state: *mut State, arg: i32, def: i64) -> i64 {
    unsafe {
        return match lua_type(state, arg) {
            None | Some(TAG_TYPE_NIL) => {
                def
            },
            _ => {
                lual_checkinteger(state, arg)
            }
        };
    }
}
pub unsafe extern "C" fn get_f(
    mut _state: *mut State,
    arbitrary_data: *mut libc::c_void,
    size: *mut u64,
) -> *const i8 {
    unsafe {
        let lf: *mut LoadF = arbitrary_data as *mut LoadF;
        if (*lf).n > 0 {
            *size = (*lf).n as u64;
            (*lf).n = 0;
        } else {
            if feof((*lf).f) != 0 {
                return std::ptr::null();
            }
            *size = fread(
                ((*lf).buffer).as_mut_ptr() as *mut libc::c_void,
                1 as u64,
                ::core::mem::size_of::<[i8; 8192]>() as u64,
                (*lf).f,
            );
        }
        return ((*lf).buffer).as_mut_ptr();
    }
}
pub unsafe extern "C" fn errfile(state: *mut State, what: *const i8, fnameindex: i32) -> i32 {
    unsafe {
        let err: i32 = *__errno_location();
        let filename: *const i8 =
            (lua_tolstring(state, fnameindex, std::ptr::null_mut())).offset(1 as isize);
        if err != 0 {
            lua_pushfstring(
                state,
                b"cannot %s %s: %s\0" as *const u8 as *const i8,
                what,
                filename,
                strerror(err),
            );
        } else {
            lua_pushfstring(
                state,
                b"cannot %s %s\0" as *const u8 as *const i8,
                what,
                filename,
            );
        }
        lua_rotate(state, fnameindex, -1);
        lua_settop(state, -2);
        return 5 + 1;
    }
}
pub unsafe extern "C" fn skip_bom(f: *mut FILE) -> i32 {
    unsafe {
        let c: i32 = getc(f);
        if c == 0xef as i32 && getc(f) == 0xbb as i32 && getc(f) == 0xbf as i32 {
            return getc(f);
        } else {
            return c;
        };
    }
}
pub unsafe extern "C" fn skipcomment(f: *mut FILE, cp: *mut i32) -> i32 {
    unsafe {
        *cp = skip_bom(f);
        let mut c: i32 = *cp;
        if c == '#' as i32 {
            loop {
                c = getc(f);
                if !(c != -1 && c != '\n' as i32) {
                    break;
                }
            }
            *cp = getc(f);
            return 1;
        } else {
            return 0;
        };
    }
}
pub unsafe extern "C" fn lual_loadfilex(
    state: *mut State,
    filename: *const i8,
    mode: *const i8,
) -> i32 {
    unsafe {
        let mut lf: LoadF = LoadF {
            n: 0,
            f: std::ptr::null_mut(),
            buffer: [0; 8192],
        };
        let status: i32;
        let readstatus: i32;
        let mut c: i32 = 0;
        let fnameindex: i32 = (*state).get_top() + 1;
        if filename.is_null() {
            lua_pushstring(state, b"=stdin\0" as *const u8 as *const i8);
            lf.f = stdin;
        } else {
            lua_pushfstring(state, b"@%s\0" as *const u8 as *const i8, filename);
            *__errno_location() = 0;
            lf.f = fopen(filename, b"r\0" as *const u8 as *const i8);
            if (lf.f).is_null() {
                return errfile(state, b"open\0" as *const u8 as *const i8, fnameindex);
            }
        }
        lf.n = 0;
        if skipcomment(lf.f, &mut c) != 0 {
            let fresh148 = lf.n;
            lf.n = lf.n + 1;
            lf.buffer[fresh148 as usize] = '\n' as i8;
        }
        if c == (*::core::mem::transmute::<&[u8; 5], &[i8; 5]>(b"\x1BLua\0"))[0] as i32 {
            lf.n = 0;
            if !filename.is_null() {
                *__errno_location() = 0;
                lf.f = freopen(filename, b"rb\0" as *const u8 as *const i8, lf.f);
                if (lf.f).is_null() {
                    return errfile(state, b"reopen\0" as *const u8 as *const i8, fnameindex);
                }
                skipcomment(lf.f, &mut c);
            }
        }
        if c != -1 {
            let fresh149 = lf.n;
            lf.n = lf.n + 1;
            lf.buffer[fresh149 as usize] = c as i8;
        }
        *__errno_location() = 0;
        status = lua_load(
            state,
            Some(
                get_f as unsafe extern "C" fn(*mut State, *mut libc::c_void, *mut u64) -> *const i8,
            ),
            &mut lf as *mut LoadF as *mut libc::c_void,
            lua_tolstring(state, -1, std::ptr::null_mut()),
            mode,
        );
        readstatus = ferror(lf.f);
        if !filename.is_null() {
            fclose(lf.f);
        }
        if readstatus != 0 {
            lua_settop(state, fnameindex);
            return errfile(state, b"read\0" as *const u8 as *const i8, fnameindex);
        }
        lua_rotate(state, fnameindex, -1);
        lua_settop(state, -2);
        return status;
    }
}
pub unsafe extern "C" fn get_s(
    mut _state: *mut State,
    arbitrary_data: *mut libc::c_void,
    size: *mut u64,
) -> *const i8 {
    unsafe {
        let lexical_state: *mut LoadS = arbitrary_data as *mut LoadS;
        if (*lexical_state).size == 0u64 {
            return std::ptr::null();
        }
        *size = (*lexical_state).size;
        (*lexical_state).size = 0;
        return (*lexical_state).s;
    }
}
pub unsafe extern "C" fn lual_loadbufferx(
    state: *mut State,
    buffer: *const i8,
    size: u64,
    name: *const i8,
    mode: *const i8,
) -> i32 {
    unsafe {
        let mut lexical_state: LoadS = LoadS {
            s: std::ptr::null(),
            size: 0,
        };
        lexical_state.s = buffer;
        lexical_state.size = size;
        return lua_load(
            state,
            Some(
                get_s as unsafe extern "C" fn(*mut State, *mut libc::c_void, *mut u64) -> *const i8,
            ),
            &mut lexical_state as *mut LoadS as *mut libc::c_void,
            name,
            mode,
        );
    }
}
pub unsafe extern "C" fn lual_getmetafield(state: *mut State, obj: i32, event: *const i8) -> i32 {
    unsafe {
        if (*state).lua_getmetatable(obj) {
            let tag: i32;
            lua_pushstring(state, event);
            tag = lua_rawget(state, -2);
            if tag == 0 {
                lua_settop(state, -3);
            } else {
                lua_rotate(state, -2, -1);
                lua_settop(state, -2);
            }
            return tag;
        } else {
            return 0;
        };
    }
}
pub unsafe extern "C" fn lual_callmeta(state: *mut State, mut obj: i32, event: *const i8) -> i32 {
    unsafe {
        obj = lua_absindex(state, obj);
        if lual_getmetafield(state, obj, event) == 0 {
            return 0;
        }
        lua_pushvalue(state, obj);
        lua_callk(state, 1, 1, 0, None);
        return 1;
    }
}
pub unsafe extern "C" fn lual_len(state: *mut State, index: i32) -> i64 {
    unsafe {
        let l: i64;
        let mut is_number: bool = false;
        lua_len(state, index);
        l = lua_tointegerx(state, -1, &mut is_number);
        if !is_number {
            lual_error(
                state,
                b"object length is not an integer\0" as *const u8 as *const i8,
            );
        }
        lua_settop(state, -2);
        return l;
    }
}
pub unsafe extern "C" fn lual_tolstring(
    state: *mut State,
    mut index: i32,
    length: *mut u64,
) -> *const i8 {
    unsafe {
        index = lua_absindex(state, index);
        if lual_callmeta(state, index, b"__tostring\0" as *const u8 as *const i8) != 0 {
            if !lua_isstring(state, -1) {
                lual_error(
                    state,
                    b"'__tostring' must return a string\0" as *const u8 as *const i8,
                );
            }
        } else {
            match lua_type(state, index) {
                Some(TAG_TYPE_NUMERIC) => {
                    if lua_isinteger(state, index) {
                        lua_pushfstring(
                            state,
                            b"%I\0" as *const u8 as *const i8,
                            lua_tointegerx(state, index, std::ptr::null_mut()),
                        );
                    } else {
                        lua_pushfstring(
                            state,
                            b"%f\0" as *const u8 as *const i8,
                            lua_tonumberx(state, index, std::ptr::null_mut()),
                        );
                    }
                }
                Some(TAG_TYPE_STRING) => {
                    lua_pushvalue(state, index);
                }
                Some(TAG_TYPE_BOOLEAN) => {
                    lua_pushstring(
                        state,
                        if lua_toboolean(state, index) != 0 {
                            b"true\0" as *const u8 as *const i8
                        } else {
                            b"false\0" as *const u8 as *const i8
                        },
                    );
                }
                Some(TAG_TYPE_NIL) => {
                    lua_pushstring(state, b"nil\0" as *const u8 as *const i8);
                }
                _ => {
                    let tag: i32 =
                        lual_getmetafield(state, index, b"__name\0" as *const u8 as *const i8);
                    let kind: *const i8 = if tag == 4 {
                        lua_tolstring(state, -1, std::ptr::null_mut())
                    } else {
                        lua_typename(state, lua_type(state, index))
                    };
                    lua_pushfstring(
                        state,
                        b"%s: %p\0" as *const u8 as *const i8,
                        kind,
                        User::lua_topointer(state, index),
                    );
                    if tag != 0 {
                        lua_rotate(state, -2, -1);
                        lua_settop(state, -2);
                    }
                }
            }
        }
        return lua_tolstring(state, -1, length);
    }
}
pub unsafe extern "C" fn lual_setfuncs(
    state: *mut State,
    mut l: *const RegisteredFunction,
    nup: i32,
) {
    unsafe {
        lual_checkstack(state, nup, b"too many upvalues\0" as *const u8 as *const i8);
        while !((*l).name).is_null() {
            if ((*l).function).is_none() {
                (*state).push_boolean(false);
            } else {
                let mut i: i32;
                i = 0;
                while i < nup {
                    lua_pushvalue(state, -nup);
                    i += 1;
                }
                lua_pushcclosure(state, (*l).function, nup);
            }
            lua_setfield(state, -(nup + 2), (*l).name);
            l = l.offset(1);
        }
        lua_settop(state, -nup - 1);
    }
}
pub unsafe extern "C" fn lual_getsubtable(
    state: *mut State,
    mut index: i32,
    fname: *const i8,
) -> i32 {
    unsafe {
        if lua_getfield(state, index, fname) == 5 {
            return 1;
        } else {
            lua_settop(state, -2);
            index = lua_absindex(state, index);
            (*state).lua_createtable();
            lua_pushvalue(state, -1);
            lua_setfield(state, index, fname);
            return 0;
        };
    }
}
pub unsafe extern "C" fn lual_requiref(
    state: *mut State,
    modname: *const i8,
    openf: CFunction,
    glb: i32,
) {
    unsafe {
        lual_getsubtable(
            state,
            -(1000000 as i32) - 1000 as i32,
            b"_LOADED\0" as *const u8 as *const i8,
        );
        lua_getfield(state, -1, modname);
        if lua_toboolean(state, -1) == 0 {
            lua_settop(state, -2);
            lua_pushcclosure(state, openf, 0);
            lua_pushstring(state, modname);
            lua_callk(state, 1, 1, 0, None);
            lua_pushvalue(state, -1);
            lua_setfield(state, -3, modname);
        }
        lua_rotate(state, -2, -1);
        lua_settop(state, -2);
        if glb != 0 {
            lua_pushvalue(state, -1);
            lua_setglobal(state, modname);
        }
    }
}
pub unsafe extern "C" fn lual_addgsub(
    b: *mut Buffer,
    mut s: *const i8,
    p: *const i8,
    r: *const i8,
) {
    unsafe {
        let mut wild: *const i8;
        let l: u64 = strlen(p);
        loop {
            wild = strstr(s, p);
            if wild.is_null() {
                break;
            }
            (*b).add_string_with_length(s, wild.offset_from(s) as u64);
            (*b).add_string(r);
            s = wild.offset(l as isize);
        }
        (*b).add_string(s);
    }
}
pub unsafe extern "C" fn lual_gsub(
    state: *mut State,
    s: *const i8,
    p: *const i8,
    r: *const i8,
) -> *const i8 {
    unsafe {
        let mut b = Buffer::new();
        b.initialize(state);
        lual_addgsub(&mut b, s, p, r);
        b.push_result();
        return lua_tolstring(state, -1, std::ptr::null_mut());
    }
}
pub unsafe extern "C" fn raw_allocate(
    ptr: *mut libc::c_void,
    mut _osize: u64,
    new_size: u64,
) -> *mut libc::c_void {
    unsafe {
        if new_size == 0u64 {
            free(ptr);
            return std::ptr::null_mut();
        } else {
            return realloc(ptr, new_size);
        };
    }
}
pub unsafe extern "C" fn panic(state: *mut State) -> i32 {
    unsafe {
        let message: *const i8 = if lua_type(state, -1) == Some(TAG_TYPE_STRING) {
            lua_tolstring(state, -1, std::ptr::null_mut())
        } else {
            b"error object is not a string\0" as *const u8 as *const i8
        };
        fprintf(
            stderr,
            b"PANIC: unprotected error in call to Lua API (%s)\n\0" as *const u8 as *const i8,
            message,
        );
        fflush(stderr);
        return 0;
    }
}
pub unsafe extern "C" fn checkcontrol(
    state: *mut State,
    mut message: *const i8,
    tocont: i32,
) -> i32 {
    unsafe {
        if tocont != 0 || {
            let fresh150 = message;
            message = message.offset(1);
            *fresh150 as i32 != '@' as i32
        } {
            return 0;
        } else {
            if strcmp(message, b"off\0" as *const u8 as *const i8) == 0 {
                lua_setwarnf(
                    state,
                    Some(warnfoff as unsafe extern "C" fn(*mut libc::c_void, *const i8, i32) -> ()),
                    state as *mut libc::c_void,
                );
            } else if strcmp(message, b"on\0" as *const u8 as *const i8) == 0 {
                lua_setwarnf(
                    state,
                    Some(warnfon as unsafe extern "C" fn(*mut libc::c_void, *const i8, i32) -> ()),
                    state as *mut libc::c_void,
                );
            }
            return 1;
        };
    }
}
pub unsafe extern "C" fn warnfoff(arbitrary_data: *mut libc::c_void, message: *const i8, tocont: i32) {
    unsafe {
        checkcontrol(arbitrary_data as *mut State, message, tocont);
    }
}
pub unsafe extern "C" fn warnfcont(arbitrary_data: *mut libc::c_void, message: *const i8, tocont: i32) {
    unsafe {
        let state: *mut State = arbitrary_data as *mut State;
        fprintf(stderr, b"%s\0" as *const u8 as *const i8, message);
        fflush(stderr);
        if tocont != 0 {
            lua_setwarnf(
                state,
                Some(warnfcont as unsafe extern "C" fn(*mut libc::c_void, *const i8, i32) -> ()),
                state as *mut libc::c_void,
            );
        } else {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const i8,
                b"\n\0" as *const u8 as *const i8,
            );
            fflush(stderr);
            lua_setwarnf(
                state,
                Some(warnfon as unsafe extern "C" fn(*mut libc::c_void, *const i8, i32) -> ()),
                state as *mut libc::c_void,
            );
        };
    }
}
pub unsafe extern "C" fn warnfon(arbitrary_data: *mut libc::c_void, message: *const i8, tocont: i32) {
    unsafe {
        if checkcontrol(arbitrary_data as *mut State, message, tocont) != 0 {
            return;
        }
        fprintf(
            stderr,
            b"%s\0" as *const u8 as *const i8,
            b"Lua warning: \0" as *const u8 as *const i8,
        );
        fflush(stderr);
        warnfcont(arbitrary_data, message, tocont);
    }
}
pub unsafe extern "C" fn lual_newstate() -> *mut State {
    unsafe {
        let state: *mut State = lua_newstate();
        if (state != std::ptr::null_mut()) as i64 != 0 {
            lua_atpanic(
                state,
                Some(panic as unsafe extern "C" fn(*mut State) -> i32),
            );
            lua_setwarnf(
                state,
                Some(warnfoff as unsafe extern "C" fn(*mut libc::c_void, *const i8, i32) -> ()),
                state as *mut libc::c_void,
            );
        }
        return state;
    }
}
pub unsafe extern "C" fn lual_checkversion_(state: *mut State, version: f64, size: u64) {
    unsafe {
        let v: f64 = 504.0;
        if size
            != (::core::mem::size_of::<i64>() as u64)
                .wrapping_mul(16 as u64)
                .wrapping_add(::core::mem::size_of::<f64>() as u64)
        {
            lual_error(
                state,
                b"core and library have incompatible numeric types\0" as *const u8 as *const i8,
            );
        } else if v != version {
            lual_error(
                state,
                b"version mismatch: app. needs %f, Lua core provides %f\0" as *const u8
                    as *const i8,
                version,
                v,
            );
        }
    }
}
pub unsafe extern "C" fn luab_print(state: *mut State) -> i32 {
    unsafe {
        let n: i32 = (*state).get_top();
        let mut i: i32;
        i = 1;
        while i <= n {
            let mut l: u64 = 0;
            let s: *const i8 = lual_tolstring(state, i, &mut l);
            if i > 1 {
                fwrite(
                    b"\t\0" as *const u8 as *const i8 as *const libc::c_void,
                    ::core::mem::size_of::<i8>() as u64,
                    1 as u64,
                    stdout,
                );
            }
            fwrite(
                s as *const libc::c_void,
                ::core::mem::size_of::<i8>() as u64,
                l,
                stdout,
            );
            lua_settop(state, -2);
            i += 1;
        }
        fwrite(
            b"\n\0" as *const u8 as *const i8 as *const libc::c_void,
            ::core::mem::size_of::<i8>() as u64,
            1 as u64,
            stdout,
        );
        fflush(stdout);
        return 0;
    }
}
pub unsafe extern "C" fn luab_warn(state: *mut State) -> i32 {
    unsafe {
        let n: i32 = (*state).get_top();
        let mut i: i32;
        lual_checklstring(state, 1, std::ptr::null_mut());
        i = 2;
        while i <= n {
            lual_checklstring(state, i, std::ptr::null_mut());
            i += 1;
        }
        i = 1;
        while i < n {
            lua_warning(state, lua_tolstring(state, i, std::ptr::null_mut()), 1);
            i += 1;
        }
        lua_warning(state, lua_tolstring(state, n, std::ptr::null_mut()), 0);
        return 0;
    }
}
pub unsafe extern "C" fn l_print(state: *mut State) {
    unsafe {
        let n: i32 = (*state).get_top();
        if n > 0 {
            lual_checkstack(
                state,
                20 as i32,
                b"too many results to print\0" as *const u8 as *const i8,
            );
            lua_getglobal(state, b"print\0" as *const u8 as *const i8);
            lua_rotate(state, 1, 1);
            if lua_pcallk(state, n, 0, 0, 0, None) != 0 {
                l_message(
                    PROGRAM_NAME,
                    lua_pushfstring(
                        state,
                        b"error calling 'print' (%s)\0" as *const u8 as *const i8,
                        lua_tolstring(state, -1, std::ptr::null_mut()),
                    ),
                );
            }
        }
    }
}
pub static mut GLOBAL_STATE: *mut State = std::ptr::null_mut();
pub static mut PROGRAM_NAME: *const i8 = b"lua\0" as *const u8 as *const i8;
pub unsafe extern "C" fn setsignal(sig: i32, handler: Option<unsafe extern "C" fn(i32) -> ()>) {
    unsafe {
        let mut sa: SignalAction = SignalAction {
            __sigaction_handler: SigActionA { sa_handler: None },
            sa_mask: SIgnalSet { __val: [0; 16] },
            sa_flags: 0,
            sa_restorer: None,
        };
        sa.__sigaction_handler.sa_handler = handler;
        sa.sa_flags = 0;
        sigemptyset(&mut sa.sa_mask);
        sigaction(sig, &mut sa, std::ptr::null_mut());
    }
}
pub unsafe extern "C" fn lstop(state: *mut State, mut _ar: *mut DebugInfo) {
    unsafe {
        lua_sethook(state, None, 0, 0);
        lual_error(state, b"interrupted!\0" as *const u8 as *const i8);
    }
}
pub unsafe extern "C" fn laction(i: i32) {
    unsafe {
        let flag: i32 = 1 << 0 | 1 << 1 | 1 << 2 | 1 << 3;
        setsignal(i, None);
        lua_sethook(
            GLOBAL_STATE,
            Some(lstop as unsafe extern "C" fn(*mut State, *mut DebugInfo) -> ()),
            flag,
            1,
        );
    }
}
pub unsafe extern "C" fn print_usage(badoption: *const i8) {
    unsafe {
        fprintf(stderr, b"%s: \0" as *const u8 as *const i8, PROGRAM_NAME);
        fflush(stderr);
        if *badoption.offset(1 as isize) as i32 == 'e' as i32
            || *badoption.offset(1 as isize) as i32 == 'l' as i32
        {
            fprintf(
                stderr,
                b"'%s' needs argument\n\0" as *const u8 as *const i8,
                badoption,
            );
            fflush(stderr);
        } else {
            fprintf(
                stderr,
                b"unrecognized option '%s'\n\0" as *const u8 as *const i8,
                badoption,
            );
            fflush(stderr);
        }
        fprintf(
        stderr,
        b"usage: %s [options] [script [args]]\nAvailable options are:\n  -e stat   execute string 'stat'\n  -i        enter interactive mode after executing 'script'\n  -l mod    require library 'mod' into global 'mod'\n  -l g=mod  require library 'mod' into global 'g'\n  -v        show version information\n  -E        ignore environment variables\n  -W        turn warnings on\n  --        stop handling options\n  -         stop handling options and execute stdin\n\0"
            as *const u8 as *const i8,
        PROGRAM_NAME,
    );
        fflush(stderr);
    }
}
pub unsafe extern "C" fn l_message(pname: *const i8, message: *const i8) {
    unsafe {
        if !pname.is_null() {
            fprintf(stderr, b"%s: \0" as *const u8 as *const i8, pname);
            fflush(stderr);
        }
        fprintf(stderr, b"%s\n\0" as *const u8 as *const i8, message);
        fflush(stderr);
    }
}
pub unsafe extern "C" fn report(state: *mut State, status: i32) -> i32 {
    unsafe {
        if status != 0 {
            let mut message: *const i8 = lua_tolstring(state, -1, std::ptr::null_mut());
            if message.is_null() {
                message = b"(error message not a string)\0" as *const u8 as *const i8;
            }
            l_message(PROGRAM_NAME, message);
            lua_settop(state, -2);
        }
        return status;
    }
}
pub unsafe extern "C" fn msghandler(state: *mut State) -> i32 {
    unsafe {
        let mut message: *const i8 = lua_tolstring(state, 1, std::ptr::null_mut());
        if message.is_null() {
            if lual_callmeta(state, 1, b"__tostring\0" as *const u8 as *const i8) != 0
                && lua_type(state, -1) == Some(TAG_TYPE_STRING)
            {
                return 1;
            } else {
                message = lua_pushfstring(
                    state,
                    b"(error object is a %s value)\0" as *const u8 as *const i8,
                    lua_typename(state, lua_type(state, 1)),
                );
            }
        }
        lual_traceback(state, state, message, 1);
        return 1;
    }
}
pub unsafe extern "C" fn docall(state: *mut State, narg: i32, nres: i32) -> i32 {
    unsafe {
        let status: i32;
        let base: i32 = (*state).get_top() - narg;
        lua_pushcclosure(
            state,
            Some(msghandler as unsafe extern "C" fn(*mut State) -> i32),
            0,
        );
        lua_rotate(state, base, 1);
        GLOBAL_STATE = state;
        setsignal(2, Some(laction as unsafe extern "C" fn(i32) -> ()));
        status = lua_pcallk(state, narg, nres, base, 0, None);
        setsignal(2, None);
        lua_rotate(state, base, -1);
        lua_settop(state, -2);
        return status;
    }
}
pub unsafe extern "C" fn createargtable(
    state: *mut State,
    argv: *mut *mut i8,
    argc: i32,
    script: i32,
) {
    unsafe {
        (*state).lua_createtable();
        let mut i: i32 = 0;
        while i < argc {
            lua_pushstring(state, *argv.offset(i as isize));
            lua_rawseti(state, -2, (i - script) as i64);
            i += 1;
        }
        lua_setglobal(state, b"arg\0" as *const u8 as *const i8);
    }
}
pub unsafe extern "C" fn dochunk(state: *mut State, mut status: i32) -> i32 {
    unsafe {
        if status == 0 {
            status = docall(state, 0, 0);
        }
        return report(state, status);
    }
}
pub unsafe extern "C" fn dofile(state: *mut State, name: *const i8) -> i32 {
    unsafe {
        return dochunk(state, lual_loadfilex(state, name, std::ptr::null()));
    }
}
pub unsafe extern "C" fn dostring(state: *mut State, s: *const i8, name: *const i8) -> i32 {
    unsafe {
        return dochunk(
            state,
            lual_loadbufferx(state, s, strlen(s), name, std::ptr::null()),
        );
    }
}
pub unsafe extern "C" fn dolibrary(state: *mut State, globname: *mut i8) -> i32 {
    unsafe {
        let status: i32;
        let mut suffix: *mut i8 = std::ptr::null_mut();
        let mut modname: *mut i8 = strchr(globname, '=' as i32);
        if modname.is_null() {
            modname = globname;
            suffix = strchr(modname, *(b"-\0" as *const u8 as *const i8) as i32);
        } else {
            *modname = '\0' as i8;
            modname = modname.offset(1);
        }
        lua_getglobal(state, b"require\0" as *const u8 as *const i8);
        lua_pushstring(state, modname);
        status = docall(state, 1, 1);
        if status == 0 {
            if !suffix.is_null() {
                *suffix = '\0' as i8;
            }
            lua_setglobal(state, globname);
        }
        return report(state, status);
    }
}
pub unsafe extern "C" fn pushargs(state: *mut State) -> i32 {
    unsafe {
        let mut i: i32;
        let n: i32;
        if lua_getglobal(state, b"arg\0" as *const u8 as *const i8) != 5 {
            lual_error(state, b"'arg' is not a table\0" as *const u8 as *const i8);
        }
        n = lual_len(state, -1) as i32;
        lual_checkstack(
            state,
            n + 3,
            b"too many arguments to script\0" as *const u8 as *const i8,
        );
        i = 1;
        while i <= n {
            lua_rawgeti(state, -i, i as i64);
            i += 1;
        }
        lua_rotate(state, -i, -1);
        lua_settop(state, -2);
        return n;
    }
}
pub unsafe extern "C" fn handle_script(state: *mut State, argv: *mut *mut i8) -> i32 {
    unsafe {
        let mut status: i32;
        let mut fname: *const i8 = *argv.offset(0 as isize);
        if strcmp(fname, b"-\0" as *const u8 as *const i8) == 0
            && strcmp(*argv.offset(-1 as isize), b"--\0" as *const u8 as *const i8) != 0
        {
            fname = std::ptr::null();
        }
        status = lual_loadfilex(state, fname, std::ptr::null());
        if status == 0 {
            let n: i32 = pushargs(state);
            status = docall(state, n, -1);
        }
        return report(state, status);
    }
}
pub unsafe extern "C" fn collectargs(argv: *mut *mut i8, first: *mut i32) -> i32 {
    unsafe {
        let mut args: i32 = 0;
        let mut i: i32;
        if !(*argv.offset(0 as isize)).is_null() {
            if *(*argv.offset(0 as isize)).offset(0 as isize) != 0 {
                PROGRAM_NAME = *argv.offset(0 as isize);
            }
        } else {
            *first = -1;
            return 0;
        }
        i = 1;
        while !(*argv.offset(i as isize)).is_null() {
            *first = i;
            if *(*argv.offset(i as isize)).offset(0 as isize) as i32 != '-' as i32 {
                return args;
            }
            let current_block_31: u64;
            match *(*argv.offset(i as isize)).offset(1 as isize) as i32 {
                45 => {
                    if *(*argv.offset(i as isize)).offset(2 as isize) as i32 != '\0' as i32 {
                        return 1;
                    }
                    *first = i + 1;
                    return args;
                }
                0 => return args,
                69 => {
                    if *(*argv.offset(i as isize)).offset(2 as isize) as i32 != '\0' as i32 {
                        return 1;
                    }
                    args |= 16 as i32;
                    current_block_31 = 4761528863920922185;
                }
                87 => {
                    if *(*argv.offset(i as isize)).offset(2 as isize) as i32 != '\0' as i32 {
                        return 1;
                    }
                    current_block_31 = 4761528863920922185;
                }
                105 => {
                    args |= 2;
                    current_block_31 = 6636775023221328366;
                }
                118 => {
                    current_block_31 = 6636775023221328366;
                }
                101 => {
                    args |= 8;
                    current_block_31 = 15172496195422792753;
                }
                108 => {
                    current_block_31 = 15172496195422792753;
                }
                _ => return 1,
            }
            match current_block_31 {
                6636775023221328366 => {
                    if *(*argv.offset(i as isize)).offset(2 as isize) as i32 != '\0' as i32 {
                        return 1;
                    }
                    args |= 4;
                }
                15172496195422792753 => {
                    if *(*argv.offset(i as isize)).offset(2 as isize) as i32 == '\0' as i32 {
                        i += 1;
                        if (*argv.offset(i as isize)).is_null()
                            || *(*argv.offset(i as isize)).offset(0 as isize) as i32 == '-' as i32
                        {
                            return 1;
                        }
                    }
                }
                _ => {}
            }
            i += 1;
        }
        *first = 0;
        return args;
    }
}
pub unsafe extern "C" fn runargs(state: *mut State, argv: *mut *mut i8, n: i32) -> i32 {
    unsafe {
        let mut i: i32;
        i = 1;
        while i < n {
            let option: i32 = *(*argv.offset(i as isize)).offset(1 as isize) as i32;
            match option {
                101 | 108 => {
                    let status: i32;
                    let mut extra: *mut i8 = (*argv.offset(i as isize)).offset(2 as isize);
                    if *extra as i32 == '\0' as i32 {
                        i += 1;
                        extra = *argv.offset(i as isize);
                    }
                    status = if option == 'e' as i32 {
                        dostring(state, extra, b"=(command line)\0" as *const u8 as *const i8)
                    } else {
                        dolibrary(state, extra)
                    };
                    if status != 0 {
                        return 0;
                    }
                }
                87 => {
                    lua_warning(state, b"@on\0" as *const u8 as *const i8, 0);
                }
                _ => {}
            }
            i += 1;
        }
        return 1;
    }
}
pub unsafe extern "C" fn get_prompt(state: *mut State, firstline: i32) -> *const i8 {
    unsafe {
        if lua_getglobal(
            state,
            if firstline != 0 {
                b"_PROMPT\0" as *const u8 as *const i8
            } else {
                b"_PROMPT2\0" as *const u8 as *const i8
            },
        ) == 0
        {
            return if firstline != 0 {
                b"> \0" as *const u8 as *const i8
            } else {
                b">> \0" as *const u8 as *const i8
            };
        } else {
            let p: *const i8 = lual_tolstring(state, -1, std::ptr::null_mut());
            lua_rotate(state, -2, -1);
            lua_settop(state, -2);
            return p;
        };
    }
}
pub unsafe extern "C" fn incomplete(state: *mut State, status: i32) -> i32 {
    unsafe {
        if status == 3 {
            let mut lmsg: u64 = 0;
            let message: *const i8 = lua_tolstring(state, -1, &mut lmsg);
            if lmsg
                >= (::core::mem::size_of::<[i8; 6]>() as u64)
                    .wrapping_div(::core::mem::size_of::<i8>() as u64)
                    .wrapping_sub(1 as u64)
                && strcmp(
                    message.offset(lmsg as isize).offset(
                        -((::core::mem::size_of::<[i8; 6]>() as u64)
                            .wrapping_div(::core::mem::size_of::<i8>() as u64)
                            .wrapping_sub(1 as u64) as isize),
                    ),
                    b"<eof>\0" as *const u8 as *const i8,
                ) == 0
            {
                return 1;
            }
        }
        return 0;
    }
}
pub unsafe extern "C" fn pushline(state: *mut State, firstline: i32) -> i32 {
    unsafe {
        let mut buffer: [i8; 512] = [0; 512];
        let b: *mut i8 = buffer.as_mut_ptr();
        let prmt: *const i8 = get_prompt(state, firstline);
        fputs(prmt, stdout);
        fflush(stdout);
        let readstatus: i32 =
            (fgets(b, 512 as i32, stdin) != std::ptr::null_mut() as *mut i8) as i32;
        lua_settop(state, 0);
        if readstatus == 0 {
            return 0;
        }
        let mut l: u64 = strlen(b);
        if l > 0 && *b.offset(l.wrapping_sub(1 as u64) as isize) as i32 == '\n' as i32 {
            l = l.wrapping_sub(1);
            *b.offset(l as isize) = '\0' as i8;
        }
        if firstline != 0 && *b.offset(0 as isize) as i32 == '=' as i32 {
            lua_pushfstring(
                state,
                b"return %s\0" as *const u8 as *const i8,
                b.offset(1 as isize),
            );
        } else {
            lua_pushlstring(state, b, l);
        }
        return 1;
    }
}
pub unsafe extern "C" fn addreturn(state: *mut State) -> i32 {
    unsafe {
        let line: *const i8 = lua_tolstring(state, -1, std::ptr::null_mut());
        let retline: *const i8 =
            lua_pushfstring(state, b"return %s;\0" as *const u8 as *const i8, line);
        let status: i32 = lual_loadbufferx(
            state,
            retline,
            strlen(retline),
            b"=stdin\0" as *const u8 as *const i8,
            std::ptr::null(),
        );
        if status == 0 {
            lua_rotate(state, -2, -1);
            lua_settop(state, -2);
        } else {
            lua_settop(state, -2 - 1);
        }
        return status;
    }
}
pub unsafe extern "C" fn multiline(state: *mut State) -> i32 {
    unsafe {
        loop {
            let mut length: u64 = 0;
            let line: *const i8 = lua_tolstring(state, 1, &mut length);
            let status: i32 = lual_loadbufferx(
                state,
                line,
                length,
                b"=stdin\0" as *const u8 as *const i8,
                std::ptr::null(),
            );
            if incomplete(state, status) == 0 || pushline(state, 0) == 0 {
                return status;
            }
            lua_rotate(state, -2, -1);
            lua_settop(state, -2);
            lua_pushstring(state, b"\n\0" as *const u8 as *const i8);
            lua_rotate(state, -2, 1);
            lua_concat(state, 3);
        }
    }
}
pub unsafe extern "C" fn loadline(state: *mut State) -> i32 {
    unsafe {
        lua_settop(state, 0);
        if pushline(state, 1) == 0 {
            return -1;
        }
        let mut status: i32 = addreturn(state);
        if status != 0 {
            status = multiline(state);
        }
        lua_rotate(state, 1, -1);
        lua_settop(state, -2);
        return status;
    }
}
pub unsafe extern "C" fn finishpcall(state: *mut State, status: i32, extra: i64) -> i32 {
    unsafe {
        if ((status != 0 && status != 1) as i32 != 0) as i64 != 0 {
            (*state).push_boolean(false);
            lua_pushvalue(state, -2);
            return 2;
        } else {
            return (*state).get_top() - extra as i32;
        };
    }
}
pub unsafe extern "C" fn luab_pcall(state: *mut State) -> i32 {
    unsafe {
        let status: i32;
        lual_checkany(state, 1);
        (*state).push_boolean(true);
        lua_rotate(state, 1, 1);
        status = lua_pcallk(
            state,
            (*state).get_top() - 2,
            -1,
            0,
            0,
            Some(finishpcall as unsafe extern "C" fn(*mut State, i32, i64) -> i32),
        );
        return finishpcall(state, status, 0);
    }
}
pub unsafe extern "C" fn checkstack(state: *mut State, other_state: *mut State, n: i32) {
    unsafe {
        if ((state != other_state && lua_checkstack(other_state, n) == 0) as i32 != 0) as i64
            != 0
        {
            lual_error(state, b"stack overflow\0" as *const u8 as *const i8);
        }
    }
}
pub unsafe extern "C" fn getthread(state: *mut State, arg: *mut i32) -> *mut State {
    unsafe {
        if lua_type(state, 1) == Some(TAG_TYPE_STATE) {
            *arg = 1;
            return lua_tothread(state, 1);
        } else {
            *arg = 0;
            return state;
        };
    }
}
pub unsafe extern "C" fn settabss(state: *mut State, k: *const i8, v: *const i8) {
    unsafe {
        lua_pushstring(state, v);
        lua_setfield(state, -2, k);
    }
}
pub unsafe extern "C" fn settabsi(state: *mut State, k: *const i8, v: i32) {
    unsafe {
        (*state).push_integer(v as i64);
        lua_setfield(state, -2, k);
    }
}
pub unsafe extern "C" fn settabsb(state: *mut State, k: *const i8, v: i32) {
    unsafe {
        (*state).push_boolean(v != 0);
        lua_setfield(state, -2, k);
    }
}
pub unsafe extern "C" fn treatstackoption(
    state: *mut State,
    other_state: *mut State,
    fname: *const i8,
) {
    unsafe {
        if state == other_state {
            lua_rotate(state, -2, 1);
        } else {
            lua_xmove(other_state, state, 1);
        }
        lua_setfield(state, -2, fname);
    }
}
pub unsafe extern "C" fn auxupvalue(state: *mut State, get: i32) -> i32 {
    unsafe {
        let n: i32 = lual_checkinteger(state, 2) as i32;
        lual_checktype(state, 1, TAG_TYPE_CLOSURE);
        let name: *const i8 = if get != 0 {
            lua_getupvalue(state, 1, n)
        } else {
            lua_setupvalue(state, 1, n)
        };
        if name.is_null() {
            return 0;
        } else {
            lua_pushstring(state, name);
            lua_rotate(state, -(get + 1), 1);
            return get + 1;
        }
    }
}
pub unsafe extern "C" fn checkupval(
    state: *mut State,
    argf: i32,
    argnup: i32,
    pnup: *mut i32,
) -> *mut libc::c_void {
    unsafe {
        let id: *mut libc::c_void;
        let nup: i32 = lual_checkinteger(state, argnup) as i32;
        lual_checktype(state, argf, TAG_TYPE_CLOSURE);
        id = lua_upvalueid(state, argf, nup);
        if !pnup.is_null() {
            (((id != std::ptr::null_mut()) as i32 != 0) as i64 != 0
                || lual_argerror(
                    state,
                    argnup,
                    b"invalid upvalue index\0" as *const u8 as *const i8,
                ) != 0) as i32;
            *pnup = nup;
        }
        return id;
    }
}
pub unsafe extern "C" fn hookf(state: *mut State, ar: *mut DebugInfo) {
    unsafe {
        pub const HOOK_NAMES: [*const i8; 5] = [
            b"call\0" as *const u8 as *const i8,
            b"return\0" as *const u8 as *const i8,
            b"line\0" as *const u8 as *const i8,
            b"count\0" as *const u8 as *const i8,
            b"tail call\0" as *const u8 as *const i8,
        ];
        lua_getfield(state, -(1000000 as i32) - 1000 as i32, HOOKKEY);
        (*state).push_state();
        if lua_rawget(state, -2) == 6 {
            lua_pushstring(state, HOOK_NAMES[(*ar).event as usize]);
            if (*ar).currentline >= 0 {
                (*state).push_integer((*ar).currentline as i64);
            } else {
                (*state).push_nil();
            }
            lua_callk(state, 2, 0, 0, None);
        }
    }
}
pub unsafe extern "C" fn makemask(smask: *const i8, count: i32) -> i32 {
    unsafe {
        let mut mask: i32 = 0;
        if !(strchr(smask, 'c' as i32)).is_null() {
            mask |= 1 << 0;
        }
        if !(strchr(smask, 'r' as i32)).is_null() {
            mask |= 1 << 1;
        }
        if !(strchr(smask, 'l' as i32)).is_null() {
            mask |= 1 << 2;
        }
        if count > 0 {
            mask |= 1 << 3;
        }
        return mask;
    }
}
pub unsafe extern "C" fn unmakemask(mask: i32, smask: *mut i8) -> *mut i8 {
    unsafe {
        let mut i: i32 = 0;
        if mask & 1 << 0 != 0 {
            let fresh190 = i;
            i = i + 1;
            *smask.offset(fresh190 as isize) = 'c' as i8;
        }
        if mask & 1 << 1 != 0 {
            let fresh191 = i;
            i = i + 1;
            *smask.offset(fresh191 as isize) = 'r' as i8;
        }
        if mask & 1 << 2 != 0 {
            let fresh192 = i;
            i = i + 1;
            *smask.offset(fresh192 as isize) = 'l' as i8;
        }
        *smask.offset(i as isize) = '\0' as i8;
        return smask;
    }
}
pub const HOOKKEY: *const i8 = b"_HOOKKEY\0" as *const u8 as *const i8;
