use crate::callinfo::*;
use crate::functions::*;
use crate::vectort::*;
use crate::vm::opcode::*;
use crate::vm::opmode::*;
use crate::debuginfo::*;
use crate::loadstate::*;
use crate::loadf::*;
use crate::token::*;
use crate::character::*;
use crate::upvaluedescription::*;
use crate::forloop::*;
use crate::bufffs::*;
use crate::utility::c::*;
use crate::calls::*;
use crate::dumpstate::*;
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
use crate::utility::*;
use crate::sparser::*;
use crate::closep::*;
use crate::new::*;
use crate::f2i::*;
use crate::labeldescription::*;
use crate::registeredfunction::*;
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
pub struct Interpreter {
    pub object: Object,
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
    pub twups: *mut Interpreter,
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
impl TObject for Interpreter {
    fn as_object(&self) -> &Object {
        &self.object
    }
    fn as_object_mut(&mut self) -> &mut Object {
        &mut self.object
    }
    fn get_class_name(&mut self) -> String {
        "interpreter".to_string()
    }
    fn get_metatable(&mut self) -> *mut Table {
        std::ptr::null_mut()
    }
}
impl Interpreter {
    pub unsafe extern "C" fn free_state(& mut self, interpreter: *mut Interpreter) {
        unsafe {
            luaf_closeupval(self, self.stack.stkidrel_pointer);
            freestack(self);
            (*interpreter).free_memory(self as *mut Interpreter as *mut libc::c_void, ::core::mem::size_of::<Interpreter>());
        }
    }
    pub fn get_status(& mut self) -> i32 {
        return self.status as i32;
    }
    pub unsafe fn set_error_object(&mut self, error_code: i32, old_top: StackValuePointer) {
        unsafe {
            match error_code {
                4 => {
                    let io: *mut TValue = &mut (*old_top).tvalue;
                    let ts: *mut TString = (*(self.global)).memory_error_message;
                    (*io).value.object = &mut (*(ts as *mut Object));
                    (*io).set_tag_variant((*ts).get_tag_variant());
                    (*io).set_collectable(true);
                }
                0 => {
                    (*old_top).tvalue.set_tag_variant(TagVariant::NilNil as u8);
                }
                _ => {
                    let io1: *mut TValue = &mut (*old_top).tvalue;
                    let io2: *const TValue = &mut (*(self.top.stkidrel_pointer).offset(-(1i32 as isize))).tvalue;
                    (*io1).copy_from(&*io2);
                }
            }
            self.top.stkidrel_pointer = old_top.offset(1);
        }
    }
    pub unsafe extern "C" fn correct_stack(&mut self) {
        unsafe {
            (*self).top.stkidrel_pointer =
                ((*self).stack.stkidrel_pointer as *mut i8).offset((*self).top.stkidrel_offset as isize) as StackValuePointer;
            (*self).tbc_list.stkidrel_pointer =
                ((*self).stack.stkidrel_pointer as *mut i8).offset((*self).tbc_list.stkidrel_offset as isize) as StackValuePointer;
            let mut up: *mut UpValue = (*self).open_upvalue;
            while !up.is_null() {
                (*up).v.p = &mut (*(((*self).stack.stkidrel_pointer as *mut i8).offset((*up).v.offset as isize)
                    as StackValuePointer))
                    .tvalue;
                up = (*up).u.open.next;
            }
            let mut call_info: *mut CallInfo = (*self).call_info;
            while !call_info.is_null() {
                (*call_info).top.stkidrel_pointer =
                    ((*self).stack.stkidrel_pointer as *mut i8).offset((*call_info).top.stkidrel_offset as isize) as StackValuePointer;
                (*call_info).function.stkidrel_pointer = ((*self).stack.stkidrel_pointer as *mut i8)
                    .offset((*call_info).function.stkidrel_offset as isize)
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
                (*self.top.stkidrel_pointer).tvalue.set_tag_variant(TAG_VARIANT_BOOLEAN_TRUE);
            } else {
                (*self.top.stkidrel_pointer).tvalue.set_tag_variant(TAG_VARIANT_BOOLEAN_FALSE);
            }
            self.top.stkidrel_pointer = self.top.stkidrel_pointer.offset(1);
        }
    }
    pub unsafe extern "C" fn push_integer(&mut self, x: i64) {
        unsafe {
            let t_value: *mut TValue = &mut (*self.top.stkidrel_pointer).tvalue;
            (*t_value).value.integer = x;
            (*t_value).set_tag_variant(TAG_VARIANT_NUMERIC_INTEGER);
            self.top.stkidrel_pointer = self.top.stkidrel_pointer.offset(1);
        }
    }
    pub unsafe extern "C" fn push_nil(&mut self) {
        unsafe {
            (*self.top.stkidrel_pointer).tvalue.set_tag_variant(TagVariant::NilNil as u8);
            self.top.stkidrel_pointer = self.top.stkidrel_pointer.offset(1);
        }
    }
    pub unsafe extern "C" fn push_number(&mut self, x: f64) {
        unsafe {
            let t_value: *mut TValue = &mut (*self.top.stkidrel_pointer).tvalue;
            (*t_value).value.number = x;
            (*t_value).set_tag_variant(TAG_VARIANT_NUMERIC_NUMBER);
            self.top.stkidrel_pointer = self.top.stkidrel_pointer.offset(1);
        }
    }
    pub unsafe extern "C" fn get_top(&mut self) -> i32 {
        unsafe {
            return self
                .top
                .stkidrel_pointer
                .offset_from(((*self.call_info).function.stkidrel_pointer).offset(1 as isize))
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
                    free_object(self, curr);
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
    pub unsafe extern "C" fn free_memory(&mut self, block: *mut libc::c_void, old_size: usize) {
        unsafe {
            raw_allocate(block, old_size, 0);
            (*(self.global)).gc_debt =
                ((*(self.global)).gc_debt as u64).wrapping_sub(old_size as u64) as i64;
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
            let io: *mut TValue = &mut (*self.top.stkidrel_pointer).tvalue;
            (*io).value.object = &mut (*(self as *mut Interpreter as *mut Object));
            (*io).set_tag_variant(TAG_VARIANT_STATE);
            (*io).set_collectable(true);
            self.top.stkidrel_pointer = self.top.stkidrel_pointer.offset(1);
            return (*self.global).main_state == self;
        }
    }
    pub unsafe extern "C" fn relstack(& mut self) {
        unsafe {
            self.top.stkidrel_offset =
                (self.top.stkidrel_pointer as *mut i8).offset_from(self.stack.stkidrel_pointer as *mut i8) as i64;
            self.tbc_list.stkidrel_offset =
                (self.tbc_list.stkidrel_pointer as *mut i8).offset_from(self.stack.stkidrel_pointer as *mut i8) as i64;
            let mut up: *mut UpValue = self.open_upvalue;
            while !up.is_null() {
                (*up).v.offset =
                    ((*up).v.p as StackValuePointer as *mut i8).offset_from(self.stack.stkidrel_pointer as *mut i8) as i64;
                up = (*up).u.open.next;
            }
            let mut call_info: *mut CallInfo = self.call_info;
            while !call_info.is_null() {
                (*call_info).top.stkidrel_offset =
                    ((*call_info).top.stkidrel_pointer as *mut i8).offset_from(self.stack.stkidrel_pointer as *mut i8) as i64;
                (*call_info).function.stkidrel_offset = ((*call_info).function.stkidrel_pointer as *mut i8)
                    .offset_from(self.stack.stkidrel_pointer as *mut i8)
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
                23,
            );
            let io: *mut TValue = &mut (*self.top.stkidrel_pointer).tvalue;
            (*io).value.object = &mut (*(message as *mut Object));
            (*io).set_tag_variant((*message).get_tag_variant());
            (*io).set_collectable(true);
            self.top.stkidrel_pointer = self.top.stkidrel_pointer.offset(1);
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
            let mut lim: StackValuePointer = self.top.stkidrel_pointer;
            let mut call_info: *mut CallInfo = self.call_info;
            while !call_info.is_null() {
                if lim < (*call_info).top.stkidrel_pointer {
                    lim = (*call_info).top.stkidrel_pointer;
                }
                call_info = (*call_info).previous;
            }
            let mut res: i32 = lim.offset_from(self.stack.stkidrel_pointer) as i32 + 1;
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
                && (self.stack_last.stkidrel_pointer).offset_from(self.stack.stkidrel_pointer) as i32 > max
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
            if (((self.stack_last.stkidrel_pointer).offset_from(self.top.stkidrel_pointer) as i64 <= 1) as i32 != 0) as i32
                as i64
                != 0
            {
                luad_growstack(self, 1, true);
            }
            self.top.stkidrel_pointer = self.top.stkidrel_pointer.offset(1);
        }
    }
    pub unsafe extern "C" fn lua_createtable(& mut self) {
        unsafe {
            let table: *mut Table = luah_new(self);
            let io: *mut TValue = &mut (*self.top.stkidrel_pointer).tvalue;
            (*io).value.object = &mut (*(table as *mut Object));
            (*io).set_tag_variant(TAG_VARIANT_TABLE);
            (*io).set_collectable(true);
            self.top.stkidrel_pointer = self.top.stkidrel_pointer.offset(1);
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
                TagType::Table => {
                    metatable = (*((*object).value.object as *mut Table)).get_metatable();
                }
                TagType::User => {
                    metatable = (*((*object).value.object as *mut User)).get_metatable();
                }
                _ => {
                    metatable = (*self.global).metatables[(*object).get_tag_type() as usize];
                }
            }
            if metatable.is_null() {
                false
            } else {
                let io: *mut TValue = &mut (*self.top.stkidrel_pointer).tvalue;
                (*io).value.object = &mut (*(metatable as *mut Object));
                (*io).set_tag_variant(TAG_VARIANT_TABLE);
                (*io).set_collectable(true);
                self.top.stkidrel_pointer = self.top.stkidrel_pointer.offset(1);
                true
            }
        }
    }
    pub unsafe extern "C" fn lua_getiuservalue(& mut self, index: i32, n: i32) -> i32 {
        unsafe {
            let t: i32;
            let tvalue: *mut TValue = self.index2value(index);
            if n <= 0 || n > (*((*tvalue).value.object as *mut User)).count_upvalues as i32 {
                (*self.top.stkidrel_pointer).tvalue.set_tag_variant(TagVariant::NilNil as u8);
                t = -1;
            } else {
                let io1: *mut TValue = &mut (*self.top.stkidrel_pointer).tvalue;
                let io2: *const TValue = &mut (*((*((*tvalue).value.object as *mut User)).upvalues)
                    .as_mut_ptr()
                    .offset((n - 1) as isize));
                (*io1).copy_from(&*io2);
                t = (*self.top.stkidrel_pointer).tvalue.get_tag_type() as i32;
            }
            self.top.stkidrel_pointer = self.top.stkidrel_pointer.offset(1);
            return t;
        }
    }
    pub unsafe extern "C" fn index2value(& mut self, mut index: i32) -> *mut TValue {
        unsafe {
            let call_info: *mut CallInfo = self.call_info;
            if index > 0 {
                let o: StackValuePointer = ((*call_info).function.stkidrel_pointer).offset(index as isize);
                if o >= self.top.stkidrel_pointer {
                    return &mut (*self.global).none_value;
                } else {
                    return &mut (*o).tvalue;
                }
            } else if !(index <= -(1000000 as i32) - 1000 as i32) {
                return &mut (*self.top.stkidrel_pointer.offset(index as isize)).tvalue;
            } else if index == -(1000000 as i32) - 1000 as i32 {
                return &mut (*self.global).l_registry;
            } else {
                index = -(1000000 as i32) - 1000 as i32 - index;
                let value = (*(*call_info).function.stkidrel_pointer).tvalue;
                if value.is_collectable() && value.get_tag_variant() == TAG_VARIANT_CLOSURE_C {
                    let function: *mut Closure = &mut (*(value.value.object as *mut Closure));
                    return if index <= (*function).count_upvalues as i32 {
                        &mut *((*function).upvalues).
                            c_tvalues.as_mut_ptr()
                            .offset((index - 1) as isize) as *mut TValue
                    } else {
                        &mut (*self.global).none_value
                    };
                } else {
                    return &mut (*self.global).none_value;
                }
            };
        }
    }
}
pub unsafe extern "C" fn do_repl(interpreter: *mut Interpreter) {
    unsafe {
        let mut status: i32;
        let oldprogname: *const i8 = PROGRAM_NAME;
        PROGRAM_NAME = std::ptr::null();
        loop {
            status = loadline(interpreter);
            if !(status != -1) {
                break;
            }
            if status == 0 {
                status = docall(interpreter, 0, -1);
            }
            if status == 0 {
                l_print(interpreter);
            } else {
                report(interpreter, status);
            }
        }
        lua_settop(interpreter, 0);
        fwrite(
            b"\n\0" as *const u8 as *const i8 as *const libc::c_void,
            ::core::mem::size_of::<i8>(),
            1,
            stdout,
        );
        fflush(stdout);
        PROGRAM_NAME = oldprogname;
    }
}
pub unsafe extern "C" fn luad_throw(interpreter: *mut Interpreter, mut error_code: i32) -> ! {
    unsafe {
        if !((*interpreter).long_jump).is_null() {
            ::core::ptr::write_volatile(&mut (*(*interpreter).long_jump).status as *mut i32, error_code);
            _longjmp(((*(*interpreter).long_jump).jbt).as_mut_ptr(), 1);
        } else {
            let global: *mut Global = (*interpreter).global;
            error_code = luae_resetthread(interpreter, error_code);
            (*interpreter).status = error_code as u8;
            if !((*(*global).main_state).long_jump).is_null() {
                let fresh0 = (*(*global).main_state).top.stkidrel_pointer;
                (*(*global).main_state).top.stkidrel_pointer = ((*(*global).main_state).top.stkidrel_pointer).offset(1);
                let io1: *mut TValue = &mut (*fresh0).tvalue;
                let io2: *const TValue = &mut (*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))).tvalue;
                (*io1).copy_from(&*io2);
                luad_throw((*global).main_state, error_code);
            } else {
                if ((*global).panic).is_some() {
                    ((*global).panic).expect("non-null function pointer")(interpreter);
                }
                abort();
            }
        };
    }
}
pub unsafe extern "C" fn luad_rawrunprotected(
    interpreter: *mut Interpreter,
    f: ProtectedFunction,
    arbitrary_data: *mut libc::c_void,
) -> i32 {
    unsafe {
        let old_count_c_calls: u32 = (*interpreter).count_c_calls;
        let mut long_jump = LongJump::new();
        ::core::ptr::write_volatile(&mut long_jump.status as *mut i32, 0);
        long_jump.previous = (*interpreter).long_jump;
        (*interpreter).long_jump = &mut long_jump;
        if _setjmp((long_jump.jbt).as_mut_ptr()) == 0 {
            (Some(f.expect("non-null function pointer"))).expect("non-null function pointer")(
                interpreter, arbitrary_data,
            );
        }
        (*interpreter).long_jump = long_jump.previous;
        (*interpreter).count_c_calls = old_count_c_calls;
        return long_jump.status;
    }
}
pub unsafe extern "C" fn luad_reallocstack(
    interpreter: *mut Interpreter,
    new_size: i32,
    should_raise_error: bool,
) -> i32 {
    unsafe {
        let old_size: i32 = ((*interpreter).stack_last.stkidrel_pointer).offset_from((*interpreter).stack.stkidrel_pointer) as i32;
        let oldgcstop: i32 = (*(*interpreter).global).gcstopem as i32;
        (*interpreter).relstack();
        (*(*interpreter).global).gcstopem = 1;
        let newstack: StackValuePointer = luam_realloc_(
            interpreter,
            (*interpreter).stack.stkidrel_pointer as *mut libc::c_void,
            ((old_size + 5) as usize).wrapping_mul(::core::mem::size_of::<StackValue>() as usize),
            ((new_size + 5) as usize).wrapping_mul(::core::mem::size_of::<StackValue>() as usize),
        ) as *mut StackValue;
        (*(*interpreter).global).gcstopem = oldgcstop as u8;
        if ((newstack == std::ptr::null_mut() as StackValuePointer) as i32 != 0) as i64 != 0 {
            (*interpreter).correct_stack();
            if should_raise_error {
                luad_throw(interpreter, 4);
            } else {
                return 0;
            }
        }
        (*interpreter).stack.stkidrel_pointer = newstack;
        (*interpreter).correct_stack();
        (*interpreter).stack_last.stkidrel_pointer = ((*interpreter).stack.stkidrel_pointer).offset(new_size as isize);
        for i in (old_size + 5)..(new_size + 5) {
            (*newstack.offset(i as isize))
                .tvalue
                .set_tag_variant(TagVariant::NilNil as u8);
        }
        return 1;
    }
}
pub unsafe extern "C" fn luad_growstack(
    interpreter: *mut Interpreter,
    n: i32,
    should_raise_error: bool,
) -> i32 {
    unsafe {
        let size: i32 = ((*interpreter).stack_last.stkidrel_pointer).offset_from((*interpreter).stack.stkidrel_pointer) as i32;
        if size > 1000000 {
            if should_raise_error {
                (*interpreter).luad_errerr();
            }
            return 0;
        } else if n < 1000000 {
            let mut new_size: i32 = 2 * size;
            let needed: i32 = ((*interpreter).top.stkidrel_pointer).offset_from((*interpreter).stack.stkidrel_pointer) as i32 + n;
            if new_size > 1000000 {
                new_size = 1000000;
            }
            if new_size < needed {
                new_size = needed;
            }
            if new_size <= 1000000 {
                return luad_reallocstack(interpreter, new_size, should_raise_error);
            }
        }
        luad_reallocstack(interpreter, 1000000 + 200, should_raise_error);
        if should_raise_error {
            luag_runerror(interpreter, b"stack overflow\0" as *const u8 as *const i8);
        }
        return 0;
    }
}
pub unsafe extern "C" fn luad_hook(
    interpreter: *mut Interpreter,
    event: i32,
    line: i32,
    ftransfer: i32,
    ntransfer: i32,
) {
    unsafe {
        let hook: HookFunction = (*interpreter).hook;
        if hook.is_some() && (*interpreter).allow_hook as i32 != 0 {
            let mut mask: i32 = 1 << 3;
            let call_info: *mut CallInfo = (*interpreter).call_info;
            let top: i64 =
                ((*interpreter).top.stkidrel_pointer as *mut i8).offset_from((*interpreter).stack.stkidrel_pointer as *mut i8) as i64;
            let ci_top: i64 =
                ((*call_info).top.stkidrel_pointer as *mut i8).offset_from((*interpreter).stack.stkidrel_pointer as *mut i8) as i64;
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
            if (*call_info).call_status as i32 & 1 << 1 == 0 && (*interpreter).top.stkidrel_pointer < (*call_info).top.stkidrel_pointer
            {
                (*interpreter).top.stkidrel_pointer = (*call_info).top.stkidrel_pointer;
            }
            if ((((*interpreter).stack_last.stkidrel_pointer).offset_from((*interpreter).top.stkidrel_pointer) as i64 <= 20 as i64)
                as i32
                != 0) as i64
                != 0
            {
                luad_growstack(interpreter, 20 as i32, true);
            }
            if (*call_info).top.stkidrel_pointer < (*interpreter).top.stkidrel_pointer.offset(20 as isize) {
                (*call_info).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(20 as isize);
            }
            (*interpreter).allow_hook = 0;
            (*call_info).call_status = ((*call_info).call_status as i32 | mask) as u16;
            (Some(hook.expect("non-null function pointer"))).expect("non-null function pointer")(
                interpreter, &mut ar,
            );
            (*interpreter).allow_hook = 1;
            (*call_info).top.stkidrel_pointer = ((*interpreter).stack.stkidrel_pointer as *mut i8).offset(ci_top as isize) as StackValuePointer;
            (*interpreter).top.stkidrel_pointer = ((*interpreter).stack.stkidrel_pointer as *mut i8).offset(top as isize) as StackValuePointer;
            (*call_info).call_status = ((*call_info).call_status as i32 & !mask) as u16;
        }
    }
}
pub unsafe extern "C" fn luad_hookcall(interpreter: *mut Interpreter, call_info: *mut CallInfo) {
    unsafe {
        (*interpreter).old_program_counter = 0;
        if (*interpreter).hook_mask & (1 << 0) != 0 {
            let event: i32 = if ((*call_info).call_status & (1 << 5)) != 0 {
                4
            } else {
                0
            };
            let p: *mut Prototype = (*((*(*call_info).function.stkidrel_pointer).tvalue.value.object
                as *mut Closure)).payload.l_prototype;
            (*call_info).u.l.saved_program_counter =
                ((*call_info).u.l.saved_program_counter).offset(1);
            (*call_info).u.l.saved_program_counter;
            luad_hook(interpreter, event, -1, 1, (*p).prototype_count_parameters as i32);
            (*call_info).u.l.saved_program_counter =
                ((*call_info).u.l.saved_program_counter).offset(-1);
            (*call_info).u.l.saved_program_counter;
        }
    }
}
pub unsafe extern "C" fn rethook(interpreter: *mut Interpreter, mut call_info: *mut CallInfo, nres: i32) {
    unsafe {
        if (*interpreter).hook_mask & 1 << 1 != 0 {
            let firstres: StackValuePointer = (*interpreter).top.stkidrel_pointer.offset(-(nres as isize));
            let mut delta: i32 = 0;
            if (*call_info).call_status as i32 & 1 << 1 == 0 {
                let p: *mut Prototype = (*((*(*call_info).function.stkidrel_pointer).tvalue.value.object
                    as *mut Closure)).payload.l_prototype;
                if (*p).prototype_is_variable_arguments {
                    delta =
                        (*call_info).u.l.count_extra_arguments + (*p).prototype_count_parameters as i32 + 1;
                }
            }
            (*call_info).function.stkidrel_pointer = ((*call_info).function.stkidrel_pointer).offset(delta as isize);
            let ftransfer: i32 = firstres.offset_from((*call_info).function.stkidrel_pointer) as i32;
            luad_hook(interpreter, 1, -1, ftransfer, nres);
            (*call_info).function.stkidrel_pointer = ((*call_info).function.stkidrel_pointer).offset(-(delta as isize));
        }
        call_info = (*call_info).previous;
        if (*call_info).call_status as i32 & 1 << 1 == 0 {
            (*interpreter).old_program_counter = ((*call_info).u.l.saved_program_counter).offset_from(
                (*(*((*(*call_info).function.stkidrel_pointer).tvalue.value.object as *mut Closure))
                    .payload.l_prototype)
                    .prototype_code.pointer,
            ) as i32
                - 1;
        }
    }
}
pub unsafe extern "C" fn tryfunctm(interpreter: *mut Interpreter, mut function: StackValuePointer) -> StackValuePointer {
    unsafe {
        let mut p: StackValuePointer;
        if ((((*interpreter).stack_last.stkidrel_pointer).offset_from((*interpreter).top.stkidrel_pointer) as i64 <= 1) as i32 != 0) as i32
            as i64
            != 0
        {
            let t__: i64 = (function as *mut i8).offset_from((*interpreter).stack.stkidrel_pointer as *mut i8) as i64;
            if (*(*interpreter).global).gc_debt > 0 {
                luac_step(interpreter);
            }
            luad_growstack(interpreter, 1, true);
            function = ((*interpreter).stack.stkidrel_pointer as *mut i8).offset(t__ as isize) as StackValuePointer;
        }
        let tm: *const TValue = luat_gettmbyobj(interpreter, &mut (*function).tvalue, TM_CALL);
        if (*tm).is_tagtype_nil() {
            luag_callerror(interpreter, &mut (*function).tvalue);
        }
        p = (*interpreter).top.stkidrel_pointer;
        while p > function {
            let io1: *mut TValue = &mut (*p).tvalue;
            let io2: *const TValue = &mut (*p.offset(-(1 as isize))).tvalue;
            (*io1).copy_from(&*io2);
            p = p.offset(-1);
        }
        (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
        let io1_0: *mut TValue = &mut (*function).tvalue;
        (*io1_0).copy_from (&*tm);
        return function;
    }
}
pub unsafe extern "C" fn moveresults(
    interpreter: *mut Interpreter,
    mut res: StackValuePointer,
    mut nres: i32,
    mut wanted: i32,
) {
    unsafe {
        let firstresult: StackValuePointer;
        match wanted {
            0 => {
                (*interpreter).top.stkidrel_pointer = res;
                return;
            }
            1 => {
                if nres == 0 {
                    (*res).tvalue.set_tag_variant(TagVariant::NilNil as u8);
                } else {
                    let io1: *mut TValue = &mut (*res).tvalue;
                    let io2: *const TValue = &mut (*(*interpreter).top.stkidrel_pointer.offset(-(nres as isize))).tvalue;
                    (*io1).copy_from(&*io2);
                }
                (*interpreter).top.stkidrel_pointer = res.offset(1 as isize);
                return;
            }
            -1 => {
                wanted = nres;
            }
            _ => {
                if wanted < -1 {
                    (*(*interpreter).call_info).call_status =
                        ((*(*interpreter).call_info).call_status as i32 | 1 << 9 as i32) as u16;
                    (*(*interpreter).call_info).u2.nres = nres;
                    res = luaf_close(interpreter, res, -1, 1);
                    (*(*interpreter).call_info).call_status =
                        ((*(*interpreter).call_info).call_status as i32 & !(1 << 9 as i32)) as u16;
                    if (*interpreter).hook_mask != 0 {
                        let savedres: i64 =
                            (res as *mut i8).offset_from((*interpreter).stack.stkidrel_pointer as *mut i8) as i64;
                        rethook(interpreter, (*interpreter).call_info, nres);
                        res = ((*interpreter).stack.stkidrel_pointer as *mut i8).offset(savedres as isize) as StackValuePointer;
                    }
                    wanted = -wanted - 3;
                    if wanted == -1 {
                        wanted = nres;
                    }
                }
            }
        }
        firstresult = (*interpreter).top.stkidrel_pointer.offset(-(nres as isize));
        if nres > wanted {
            nres = wanted;
        }
        for i in 0..nres {
            let io1_0: *mut TValue = &mut (*res.offset(i as isize)).tvalue;
            let io2_0: *const TValue = &mut (*firstresult.offset(i as isize)).tvalue;
            (*io1_0).copy_from(&*io2_0);
        }
        for i in nres..wanted {
            (*res.offset(i as isize)).tvalue.set_tag_variant(TagVariant::NilNil as u8);
        }
        (*interpreter).top.stkidrel_pointer = res.offset(wanted as isize);
    }
}
pub unsafe extern "C" fn luad_poscall(interpreter: *mut Interpreter, call_info: *mut CallInfo, nres: i32) {
    unsafe {
        let wanted: i32 = (*call_info).count_results as i32;
        if (((*interpreter).hook_mask != 0 && !(wanted < -1)) as i32 != 0) as i64 != 0 {
            rethook(interpreter, call_info, nres);
        }
        moveresults(interpreter, (*call_info).function.stkidrel_pointer, nres, wanted);
        (*interpreter).call_info = (*call_info).previous;
    }
}
pub unsafe extern "C" fn prepcallinfo(
    interpreter: *mut Interpreter,
    function: StackValuePointer,
    nret: i32,
    mask: i32,
    top: StackValuePointer,
) -> *mut CallInfo {
    unsafe {
        (*interpreter).call_info = if !((*(*interpreter).call_info).next).is_null() {
            (*(*interpreter).call_info).next
        } else {
            luae_extendci(interpreter)
        };
        let call_info: *mut CallInfo = (*interpreter).call_info;
        (*call_info).function.stkidrel_pointer = function;
        (*call_info).count_results = nret;
        (*call_info).call_status = mask as u16;
        (*call_info).top.stkidrel_pointer = top;
        return call_info;
    }
}
pub unsafe extern "C" fn precallc(
    interpreter: *mut Interpreter,
    mut function: StackValuePointer,
    count_results: i32,
    cfunction: CFunction,
) -> i32 {
    unsafe {
        if ((((*interpreter).stack_last.stkidrel_pointer).offset_from((*interpreter).top.stkidrel_pointer) as i64 <= 20 as i64) as i32
            != 0) as i64
            != 0
        {
            let t__: i64 = (function as *mut i8).offset_from((*interpreter).stack.stkidrel_pointer as *mut i8) as i64;
            if (*(*interpreter).global).gc_debt > 0 {
                luac_step(interpreter);
            }
            luad_growstack(interpreter, 20 as i32, true);
            function = ((*interpreter).stack.stkidrel_pointer as *mut i8).offset(t__ as isize) as StackValuePointer;
        }
        let call_info = prepcallinfo(
            interpreter,
            function,
            count_results,
            1 << 1,
            (*interpreter).top.stkidrel_pointer.offset(20 as isize),
        );
        (*interpreter).call_info = call_info;
        if ((*interpreter).hook_mask & 1 << 0 != 0) as i64 != 0 {
            let narg: i32 = ((*interpreter).top.stkidrel_pointer).offset_from(function) as i32 - 1;
            luad_hook(interpreter, 0, -1, 1, narg);
        }
        let n: i32 = (Some(cfunction.expect("non-null function pointer")))
            .expect("non-null function pointer")(interpreter);
        luad_poscall(interpreter, call_info, n);
        return n;
    }
}
pub unsafe extern "C" fn luad_pretailcall(
    interpreter: *mut Interpreter,
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
                        interpreter,
                        function,
                        -1,
                        (*((*function).tvalue.value.object as *mut Closure)).payload.c_cfunction,
                    );
                }
                TAG_VARIANT_CLOSURE_CFUNCTION => {
                    return precallc(interpreter, function, -1, (*function).tvalue.value.function)
                }
                TAG_VARIANT_CLOSURE_L => {
                    let p: *mut Prototype =
                        (*((*function).tvalue.value.object as *mut Closure)).payload.l_prototype;
                    let fsize: i32 = (*p).prototype_maximum_stack_size as i32;
                    let nfixparams: i32 = (*p).prototype_count_parameters as i32;
                    if ((((*interpreter).stack_last.stkidrel_pointer).offset_from((*interpreter).top.stkidrel_pointer) as i64
                        <= (fsize - delta) as i64) as i32
                        != 0) as i64
                        != 0
                    {
                        let t__: i64 =
                            (function as *mut i8).offset_from((*interpreter).stack.stkidrel_pointer as *mut i8) as i64;
                        if (*(*interpreter).global).gc_debt > 0 {
                            luac_step(interpreter);
                        }
                        luad_growstack(interpreter, fsize - delta, true);
                        function = ((*interpreter).stack.stkidrel_pointer as *mut i8).offset(t__ as isize) as StackValuePointer;
                    }
                    (*call_info).function.stkidrel_pointer = ((*call_info).function.stkidrel_pointer).offset(-(delta as isize));
                    for i in 0..narg1   {
                        let io1: *mut TValue =
                            &mut (*((*call_info).function.stkidrel_pointer).offset(i as isize)).tvalue;
                        let io2: *const TValue = &mut (*function.offset(i as isize)).tvalue;
                        (*io1).copy_from(&*io2);
                    }
                    function = (*call_info).function.stkidrel_pointer;
                    while narg1 <= nfixparams {
                        (*function.offset(narg1 as isize))
                            .tvalue
                            .set_tag_variant(TagVariant::NilNil as u8);
                        narg1 += 1;
                    }
                    (*call_info).top.stkidrel_pointer = function.offset(1 as isize).offset(fsize as isize);
                    (*call_info).u.l.saved_program_counter = (*p).prototype_code.pointer;
                    (*call_info).call_status = ((*call_info).call_status as i32 | 1 << 5) as u16;
                    (*interpreter).top.stkidrel_pointer = function.offset(narg1 as isize);
                    return -1;
                }
                _ => {
                    function = tryfunctm(interpreter, function);
                    narg1 += 1;
                }
            }
        }
    }
}
pub unsafe extern "C" fn luad_precall(
    interpreter: *mut Interpreter,
    mut function: StackValuePointer,
    count_results: i32,
) -> *mut CallInfo {
    unsafe {
        loop {
            match (*function).tvalue.get_tag_variant() {
                TAG_VARIANT_CLOSURE_C => {
                    precallc(
                        interpreter,
                        function,
                        count_results,
                        (*((*function).tvalue.value.object as *mut Closure)).payload.c_cfunction,
                    );
                    return std::ptr::null_mut();
                }
                TAG_VARIANT_CLOSURE_CFUNCTION => {
                    precallc(interpreter, function, count_results, (*function).tvalue.value.function);
                    return std::ptr::null_mut();
                }
                TAG_VARIANT_CLOSURE_L => {
                    let call_info;
                    let p: *mut Prototype =
                        (*((*function).tvalue.value.object as *mut Closure)).payload.l_prototype;
                    let mut narg: i32 = ((*interpreter).top.stkidrel_pointer).offset_from(function) as i32 - 1;
                    let nfixparams: i32 = (*p).prototype_count_parameters as i32;
                    let fsize: i32 = (*p).prototype_maximum_stack_size as i32;
                    if ((((*interpreter).stack_last.stkidrel_pointer).offset_from((*interpreter).top.stkidrel_pointer) as i64 <= fsize as i64)
                        as i32
                        != 0) as i64
                        != 0
                    {
                        let t__: i64 =
                            (function as *mut i8).offset_from((*interpreter).stack.stkidrel_pointer as *mut i8) as i64;
                        if (*(*interpreter).global).gc_debt > 0 {
                            luac_step(interpreter);
                        }
                        luad_growstack(interpreter, fsize, true);
                        function = ((*interpreter).stack.stkidrel_pointer as *mut i8).offset(t__ as isize) as StackValuePointer;
                    }
                    call_info = prepcallinfo(
                        interpreter,
                        function,
                        count_results,
                        0,
                        function.offset(1 as isize).offset(fsize as isize),
                    );
                    (*interpreter).call_info = call_info;
                    (*call_info).u.l.saved_program_counter = (*p).prototype_code.pointer;
                    while narg < nfixparams {
                        let fresh1 = (*interpreter).top.stkidrel_pointer;
                        (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
                        (*fresh1).tvalue.set_tag_variant(TagVariant::NilNil as u8);
                        narg += 1;
                    }
                    return call_info;
                }
                _ => {
                    function = tryfunctm(interpreter, function);
                }
            }
        }
    }
}
pub unsafe extern "C" fn ccall(
    interpreter: *mut Interpreter,
    mut function: StackValuePointer,
    count_results: i32,
    inc: u32,
) {
    unsafe {
        let call_info;
        (*interpreter).count_c_calls = ((*interpreter).count_c_calls as u32).wrapping_add(inc) as u32;
        if (((*interpreter).count_c_calls & 0xffff as u32 >= 200 as u32) as i32 != 0) as i32
            as i64
            != 0
        {
            if ((((*interpreter).stack_last.stkidrel_pointer).offset_from((*interpreter).top.stkidrel_pointer) as i64 <= 0) as i32 != 0)
                as i64
                != 0
            {
                let t__: i64 =
                    (function as *mut i8).offset_from((*interpreter).stack.stkidrel_pointer as *mut i8) as i64;
                luad_growstack(interpreter, 0, true);
                function = ((*interpreter).stack.stkidrel_pointer as *mut i8).offset(t__ as isize) as StackValuePointer;
            }
            (*interpreter).luae_checkcstack();
        }
        call_info = luad_precall(interpreter, function, count_results);
        if !call_info.is_null() {
            (*call_info).call_status = (1 << 2) as u16;
            luav_execute(interpreter, call_info);
        }
        (*interpreter).count_c_calls = ((*interpreter).count_c_calls as u32).wrapping_sub(inc) as u32;
    }
}
pub unsafe extern "C" fn luad_callnoyield(interpreter: *mut Interpreter, function: StackValuePointer, count_results: i32) {
    unsafe {
        ccall(interpreter, function, count_results, (0x10000 as i32 | 1) as u32);
    }
}
pub unsafe extern "C" fn finishpcallk(interpreter: *mut Interpreter, call_info: *mut CallInfo) -> i32 {
    unsafe {
        let mut status: i32 = (*call_info).call_status as i32 >> 10 as i32 & 7;
        if ((status == 0) as i32 != 0) as i64 != 0 {
            status = 1;
        } else {
            let mut function: StackValuePointer =
                ((*interpreter).stack.stkidrel_pointer as *mut i8).offset((*call_info).u2.funcidx as isize) as StackValuePointer;
            (*interpreter).allow_hook = ((*call_info).call_status as i32 & 1 << 0) as u8;
            function = luaf_close(interpreter, function, status, 1);
            (*interpreter).set_error_object(status, function);
            (*interpreter).luad_shrinkstack();
            (*call_info).call_status =
                ((*call_info).call_status as i32 & !((7) << 10 as i32) | 0 << 10 as i32) as u16;
        }
        (*call_info).call_status = ((*call_info).call_status as i32 & !(1 << 4)) as u16;
        (*interpreter).error_function = (*call_info).u.c.old_error_function;
        return status;
    }
}
pub unsafe extern "C" fn finishccall(interpreter: *mut Interpreter, call_info: *mut CallInfo) {
    unsafe {
        let n: i32;
        if (*call_info).call_status as i32 & 1 << 9 as i32 != 0 {
            n = (*call_info).u2.nres;
        } else {
            let mut status: i32 = 1;
            if (*call_info).call_status as i32 & 1 << 4 != 0 {
                status = finishpcallk(interpreter, call_info);
            }
            if -1 <= -1 && (*(*interpreter).call_info).top.stkidrel_pointer < (*interpreter).top.stkidrel_pointer {
                (*(*interpreter).call_info).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer;
            }
            n = (Some(((*call_info).u.c.context_function).expect("non-null function pointer")))
                .expect("non-null function pointer")(
                interpreter, status, (*call_info).u.c.context
            );
        }
        luad_poscall(interpreter, call_info, n);
    }
}
pub unsafe extern "C" fn unroll(interpreter: *mut Interpreter, mut _ud: *mut libc::c_void) {
    unsafe {
        let mut call_info;
        loop {
            call_info = (*interpreter).call_info;
            if !(call_info != &mut (*interpreter).base_callinfo as *mut CallInfo) {
                break;
            }
            if (*call_info).call_status as i32 & 1 << 1 != 0 {
                finishccall(interpreter, call_info);
            } else {
                luav_finishop(interpreter);
                luav_execute(interpreter, call_info);
            }
        }
    }
}
pub unsafe extern "C" fn resume_error(interpreter: *mut Interpreter, message: *const i8, narg: i32) -> i32 {
    unsafe {
        (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(-(narg as isize));
        let io: *mut TValue = &mut (*(*interpreter).top.stkidrel_pointer).tvalue;
        let ts: *mut TString = luas_new(interpreter, message);
        (*io).value.object = &mut (*(ts as *mut Object));
        (*io).set_tag_variant((*ts).get_tag_variant());
        (*io).set_collectable(true);
        (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
        return 2;
    }
}
pub unsafe extern "C" fn resume(interpreter: *mut Interpreter, arbitrary_data: *mut libc::c_void) {
    unsafe {
        let mut n: i32 = *(arbitrary_data as *mut i32);
        let first_argument: StackValuePointer = (*interpreter).top.stkidrel_pointer.offset(-(n as isize));
        let call_info: *mut CallInfo = (*interpreter).call_info;
        if (*interpreter).status as i32 == 0 {
            ccall(interpreter, first_argument.offset(-(1 as isize)), -1, 0);
        } else {
            (*interpreter).status = 0;
            if (*call_info).call_status as i32 & 1 << 1 == 0 {
                (*call_info).u.l.saved_program_counter =
                    ((*call_info).u.l.saved_program_counter).offset(-1);
                (*call_info).u.l.saved_program_counter;
                (*interpreter).top.stkidrel_pointer = first_argument;
                luav_execute(interpreter, call_info);
            } else {
                if ((*call_info).u.c.context_function).is_some() {
                    n = (Some(((*call_info).u.c.context_function).expect("non-null function pointer")))
                        .expect("non-null function pointer")(
                        interpreter, 1, (*call_info).u.c.context
                    );
                }
                luad_poscall(interpreter, call_info, n);
            }
            unroll(interpreter, std::ptr::null_mut());
        };
    }
}
pub unsafe extern "C" fn precover(interpreter: *mut Interpreter, mut status: i32) -> i32 {
    unsafe {
        let mut call_info;
        while status > 1 && {
            call_info = (*interpreter).find_pcall();
            !call_info.is_null()
        } {
            (*interpreter).call_info = call_info;
            (*call_info).call_status = ((*call_info).call_status as i32 & !((7) << 10 as i32)
                | status << 10 as i32) as u16;
            status = luad_rawrunprotected(
                interpreter,
                Some(unroll as unsafe extern "C" fn(*mut Interpreter, *mut libc::c_void) -> ()),
                std::ptr::null_mut(),
            );
        }
        return status;
    }
}
pub unsafe extern "C" fn lua_resume(
    interpreter: *mut Interpreter,
    from: *mut Interpreter,
    mut nargs: i32,
    count_results: *mut i32,
) -> i32 {
    unsafe {
        let mut status;
        if (*interpreter).status as i32 == 0 {
            if (*interpreter).call_info != &mut (*interpreter).base_callinfo as *mut CallInfo {
                return resume_error(
                    interpreter,
                    b"cannot resume non-suspended coroutine\0" as *const u8 as *const i8,
                    nargs,
                );
            } else if ((*interpreter).top.stkidrel_pointer)
                .offset_from(((*(*interpreter).call_info).function.stkidrel_pointer).offset(1 as isize))
                as i64
                == nargs as i64
            {
                return resume_error(
                    interpreter,
                    b"cannot resume dead coroutine\0" as *const u8 as *const i8,
                    nargs,
                );
            }
        } else if (*interpreter).status as i32 != 1 {
            return resume_error(
                interpreter,
                b"cannot resume dead coroutine\0" as *const u8 as *const i8,
                nargs,
            );
        }
        (*interpreter).count_c_calls = if !from.is_null() {
            (*from).count_c_calls & 0xffff as u32
        } else {
            0
        };
        if (*interpreter).count_c_calls & 0xffff as u32 >= 200 as u32 {
            return resume_error(
                interpreter,
                b"C stack overflow\0" as *const u8 as *const i8,
                nargs,
            );
        }
        (*interpreter).count_c_calls = ((*interpreter).count_c_calls).wrapping_add(1);
        (*interpreter).count_c_calls;
        status = luad_rawrunprotected(
            interpreter,
            Some(resume as unsafe extern "C" fn(*mut Interpreter, *mut libc::c_void) -> ()),
            &mut nargs as *mut i32 as *mut libc::c_void,
        );
        status = precover(interpreter, status);
        if !((!(status > 1) as i32 != 0) as i64 != 0) {
            (*interpreter).status = status as u8;
            (*interpreter).set_error_object(status, (*interpreter).top.stkidrel_pointer);
            (*(*interpreter).call_info).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer;
        }
        *count_results = if status == 1 {
            (*(*interpreter).call_info).u2.nyield
        } else {
            ((*interpreter).top.stkidrel_pointer).offset_from(((*(*interpreter).call_info).function.stkidrel_pointer).offset(1 as isize))
                as i32
        };
        return status;
    }
}
pub unsafe extern "C" fn lua_yieldk(
    interpreter: *mut Interpreter,
    count_results: i32,
    ctx: i64,
    k: ContextFunction,
) -> i32 {
    unsafe {
        let call_info;
        call_info = (*interpreter).call_info;
        if (!((*interpreter).count_c_calls & 0xffff0000 as u32 == 0) as i32 != 0) as i64 != 0 {
            if interpreter != (*(*interpreter).global).main_state {
                luag_runerror(
                    interpreter,
                    b"attempt to yield across a C-call boundary\0" as *const u8 as *const i8,
                );
            } else {
                luag_runerror(
                    interpreter,
                    b"attempt to yield from outside a coroutine\0" as *const u8 as *const i8,
                );
            }
        }
        (*interpreter).status = 1;
        (*call_info).u2.nyield = count_results;
        if (*call_info).call_status as i32 & 1 << 1 == 0 {
        } else {
            (*call_info).u.c.context_function = k;
            if ((*call_info).u.c.context_function).is_some() {
                (*call_info).u.c.context = ctx;
            }
            luad_throw(interpreter, 1);
        }
        return 0;
    }
}
pub unsafe extern "C" fn closepaux(interpreter: *mut Interpreter, arbitrary_data: *mut libc::c_void) {
    unsafe {
        let closep: *mut CloseP = arbitrary_data as *mut CloseP;
        luaf_close(interpreter, (*closep).level, (*closep).status, 0);
    }
}
pub unsafe extern "C" fn luad_closeprotected(
    interpreter: *mut Interpreter,
    level: i64,
    mut status: i32,
) -> i32 {
    unsafe {
        let old_call_info: *mut CallInfo = (*interpreter).call_info;
        let old_allowhooks: u8 = (*interpreter).allow_hook;
        loop {
            let mut closep = CloseP::new();
            closep.level = ((*interpreter).stack.stkidrel_pointer as *mut i8).offset(level as isize) as StackValuePointer;
            closep.status = status;
            status = luad_rawrunprotected(
                interpreter,
                Some(closepaux as unsafe extern "C" fn(*mut Interpreter, *mut libc::c_void) -> ()),
                &mut closep as *mut CloseP as *mut libc::c_void,
            );
            if ((status == 0) as i32 != 0) as i64 != 0 {
                return closep.status;
            } else {
                (*interpreter).call_info = old_call_info;
                (*interpreter).allow_hook = old_allowhooks;
            }
        }
    }
}
pub unsafe extern "C" fn luad_pcall(
    interpreter: *mut Interpreter,
    function: ProtectedFunction,
    u: *mut libc::c_void,
    old_top: i64,
    ef: i64,
) -> i32 {
    unsafe {
        let mut status: i32;
        let old_call_info: *mut CallInfo = (*interpreter).call_info;
        let old_allowhooks: u8 = (*interpreter).allow_hook;
        let old_error_function: i64 = (*interpreter).error_function;
        (*interpreter).error_function = ef;
        status = luad_rawrunprotected(interpreter, function, u);
        if ((status != 0) as i32 != 0) as i64 != 0 {
            (*interpreter).call_info = old_call_info;
            (*interpreter).allow_hook = old_allowhooks;
            status = luad_closeprotected(interpreter, old_top, status);
            (*interpreter).set_error_object(
                status,
                ((*interpreter).stack.stkidrel_pointer as *mut i8).offset(old_top as isize) as StackValuePointer,
            );
            (*interpreter).luad_shrinkstack();
        }
        (*interpreter).error_function = old_error_function;
        return status;
    }
}
pub unsafe extern "C" fn checkmode(interpreter: *mut Interpreter, mode: *const i8, x: *const i8) {
    unsafe {
        if !mode.is_null() && (strchr(mode, *x.offset(0) as i32)).is_null() {
            luao_pushfstring(
                interpreter,
                b"attempt to load a %s chunk (mode is '%s')\0" as *const u8 as *const i8,
                x,
                mode,
            );
            luad_throw(interpreter, 3);
        }
    }
}
pub unsafe extern "C" fn f_parser(interpreter: *mut Interpreter, arbitrary_data: *mut libc::c_void) {
    unsafe {
        let cl: *mut Closure;
        let p: *mut SParser = arbitrary_data as *mut SParser;
        let fresh2 = (*(*p).zio).length;
        (*(*p).zio).length = ((*(*p).zio).length).wrapping_sub(1);
        let c: i32 = if fresh2 > 0 {
            let fresh3 = (*(*p).zio).pointer;
            (*(*p).zio).pointer = ((*(*p).zio).pointer).offset(1);
            *fresh3 as u8 as i32
        } else {
            luaz_fill((*p).zio)
        };
        if c == (*::core::mem::transmute::<&[u8; 5], &[i8; 5]>(b"\x1BLua\0"))[0] as i32 {
            checkmode(interpreter, (*p).mode, b"binary\0" as *const u8 as *const i8);
            cl = load_closure(interpreter, (*p).zio, (*p).name);
        } else {
            checkmode(interpreter, (*p).mode, b"text\0" as *const u8 as *const i8);
            cl = luay_parser(
                interpreter,
                (*p).zio,
                &mut (*p).buffer,
                &mut (*p).dynamic_data,
                (*p).name,
                c,
            );
        }
        luaf_initupvals(interpreter, cl);
    }
}
pub unsafe extern "C" fn luad_protectedparser(
    interpreter: *mut Interpreter,
    zio: *mut ZIO,
    name: *const i8,
    mode: *const i8,
) -> i32 {
    unsafe {
        let mut p: SParser = SParser {
            zio: std::ptr::null_mut(),
            buffer: Buffer::new(),
            dynamic_data: DynamicData {
                active_variable: VectorT::<VariableDescription> {
                    pointer: std::ptr::null_mut(),
                    length: 0,
                    size: 0,
                },
                gt: VectorT::<LabelDescription> {
                    pointer: std::ptr::null_mut(),
                    length: 0,
                    size: 0,
                },
                label: VectorT::<LabelDescription> {
                    pointer: std::ptr::null_mut(),
                    length: 0,
                    size: 0,
                },
            },
            mode: std::ptr::null(),
            name: std::ptr::null(),
        };
        (*interpreter).count_c_calls =
            ((*interpreter).count_c_calls as u32).wrapping_add(0x10000 as u32) as u32;
        p.zio = zio;
        p.name = name;
        p.mode = mode;
        p.dynamic_data.active_variable.pointer = std::ptr::null_mut();
        p.dynamic_data.active_variable.size = 0;
        p.dynamic_data.gt.pointer = std::ptr::null_mut();
        p.dynamic_data.gt.size = 0;
        p.dynamic_data.label.pointer = std::ptr::null_mut();
        p.dynamic_data.label.size = 0;
        p.buffer.vector.pointer = std::ptr::null_mut();
        p.buffer.vector.size = 0;
        let status = luad_pcall(
            interpreter,
            Some(f_parser as unsafe extern "C" fn(*mut Interpreter, *mut libc::c_void) -> ()),
            &mut p as *mut SParser as *mut libc::c_void,
            ((*interpreter).top.stkidrel_pointer as *mut i8).offset_from((*interpreter).stack.stkidrel_pointer as *mut i8) as i64,
            (*interpreter).error_function,
        );
        p.buffer.vector.pointer = luam_saferealloc_(
            interpreter,
            p.buffer.vector.pointer as *mut libc::c_void,
            (p.buffer.vector.size as usize).wrapping_mul(::core::mem::size_of::<i8>()),
            0,
        ) as *mut i8;
        p.buffer.vector.size = 0;
        (*interpreter).free_memory(
            p.dynamic_data.active_variable.pointer as *mut libc::c_void,
            (p.dynamic_data.active_variable.size as u64)
                .wrapping_mul(::core::mem::size_of::<VariableDescription>() as u64) as usize,
        );
        (*interpreter).free_memory(
            p.dynamic_data.gt.pointer as *mut libc::c_void,
            (p.dynamic_data.gt.size as u64)
                .wrapping_mul(::core::mem::size_of::<LabelDescription>() as u64) as usize,
        );
        (*interpreter).free_memory(
            p.dynamic_data.label.pointer as *mut libc::c_void,
            (p.dynamic_data.label.size as u64)
                .wrapping_mul(::core::mem::size_of::<LabelDescription>() as u64) as usize,
        );
        (*interpreter).count_c_calls =
            ((*interpreter).count_c_calls as u32).wrapping_sub(0x10000 as u32) as u32;
        return status;
    }
}
pub unsafe extern "C" fn index2stack(interpreter: *mut Interpreter, index: i32) -> StackValuePointer {
    unsafe {
        let call_info: *mut CallInfo = (*interpreter).call_info;
        if index > 0 {
            let o: StackValuePointer = ((*call_info).function.stkidrel_pointer).offset(index as isize);
            return o;
        } else {
            return (*interpreter).top.stkidrel_pointer.offset(index as isize);
        };
    }
}
pub unsafe extern "C" fn lua_checkstack(interpreter: *mut Interpreter, n: i32) -> i32 {
    unsafe {
        let res: i32;
        let call_info;
        call_info = (*interpreter).call_info;
        if ((*interpreter).stack_last.stkidrel_pointer).offset_from((*interpreter).top.stkidrel_pointer) as i64 > n as i64 {
            res = 1;
        } else {
            res = luad_growstack(interpreter, n, false);
        }
        if res != 0 && (*call_info).top.stkidrel_pointer < (*interpreter).top.stkidrel_pointer.offset(n as isize) {
            (*call_info).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(n as isize);
        }
        return res;
    }
}
pub unsafe extern "C" fn lua_xmove(from: *mut Interpreter, to: *mut Interpreter, n: i32) {
    unsafe {
        if from != to {
            (*from).top.stkidrel_pointer = ((*from).top.stkidrel_pointer).offset(-(n as isize));
            for i in 0..n {
                let io1: *mut TValue = &mut (*(*to).top.stkidrel_pointer).tvalue;
                let io2: *const TValue = &mut (*((*from).top.stkidrel_pointer).offset(i as isize)).tvalue;
                (*io1).copy_from(&*io2);
                (*to).top.stkidrel_pointer = ((*to).top.stkidrel_pointer).offset(1);
                (*to).top.stkidrel_pointer;
            }
        }
    }
}
pub unsafe extern "C" fn lua_atpanic(interpreter: *mut Interpreter, panicf: CFunction) -> CFunction {
    unsafe {
        let old: CFunction = (*(*interpreter).global).panic;
        (*(*interpreter).global).panic = panicf;
        return old;
    }
}
pub unsafe extern "C" fn lua_absindex(interpreter: *mut Interpreter, index: i32) -> i32 {
    unsafe {
        return if index > 0 || index <= -(1000000 as i32) - 1000 as i32 {
            index
        } else {
            ((*interpreter).top.stkidrel_pointer).offset_from((*(*interpreter).call_info).function.stkidrel_pointer) as i32 + index
        };
    }
}
pub unsafe extern "C" fn lua_settop(interpreter: *mut Interpreter, index: i32) {
    unsafe {
        let call_info;
        let mut newtop;
        let mut diff;
        call_info = (*interpreter).call_info;
        let function: StackValuePointer = (*call_info).function.stkidrel_pointer;
        if index >= 0 {
            diff = function
                .offset(1 as isize)
                .offset(index as isize)
                .offset_from((*interpreter).top.stkidrel_pointer) as i64;
            while diff > 0 {
                let fresh4 = (*interpreter).top.stkidrel_pointer;
                (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
                (*fresh4).tvalue.set_tag_variant(TagVariant::NilNil as u8);
                diff -= 1;
            }
        } else {
            diff = (index + 1) as i64;
        }
        newtop = (*interpreter).top.stkidrel_pointer.offset(diff as isize);
        if diff < 0 && (*interpreter).tbc_list.stkidrel_pointer >= newtop {
            newtop = luaf_close(interpreter, newtop, -1, 0);
        }
        (*interpreter).top.stkidrel_pointer = newtop;
    }
}
pub unsafe extern "C" fn lua_closeslot(interpreter: *mut Interpreter, index: i32) {
    unsafe {
        let mut level = index2stack(interpreter, index);
        level = luaf_close(interpreter, level, -1, 0);
        (*level).tvalue.set_tag_variant(TagVariant::NilNil as u8);
    }
}
pub unsafe extern "C" fn reverse(mut _state: *mut Interpreter, mut from: StackValuePointer, mut to: StackValuePointer) {
    unsafe {
        while from < to {
            let mut temp: TValue = TValue::new(TAG_VARIANT_NIL_NIL);
            let io1: *mut TValue = &mut temp;
            let io2: *const TValue = &mut (*from).tvalue;
            (*io1).copy_from(&*io2);
            let io1_0: *mut TValue = &mut (*from).tvalue;
            let io2_0: *const TValue = &mut (*to).tvalue;
            (*io1_0).copy_from(&*io2_0);
            let io1_1: *mut TValue = &mut (*to).tvalue;
            let io2_1: *const TValue = &mut temp;
            (*io1_1).copy_from(&*io2_1);
            from = from.offset(1);
            to = to.offset(-1);
        }
    }
}
pub unsafe extern "C" fn lua_rotate(interpreter: *mut Interpreter, index: i32, n: i32) {
    unsafe {
        let high: StackValuePointer = (*interpreter).top.stkidrel_pointer.offset(-(1 as isize));
        let low: StackValuePointer = index2stack(interpreter, index);
        let middle: StackValuePointer = if n >= 0 {
            high.offset(-(n as isize))
        } else {
            low.offset(-(n as isize)).offset(-(1 as isize))
        };
        reverse(interpreter, low, middle);
        reverse(interpreter, middle.offset(1 as isize), high);
        reverse(interpreter, low, high);
    }
}
pub unsafe extern "C" fn lua_copy(interpreter: *mut Interpreter, fromidx: i32, toidx: i32) {
    unsafe {
        let fr: *mut TValue = (*interpreter).index2value(fromidx);
        let to: *mut TValue = (*interpreter).index2value(toidx);
        let io1: *mut TValue = to;
        let io2: *const TValue = fr;
        (*io1).copy_from(&*io2);
        if toidx < -(1000000 as i32) - 1000 as i32 {
            if (*fr).is_collectable() {
                if (*((*(*(*interpreter).call_info).function.stkidrel_pointer).tvalue.value.object as *mut Closure))
                    .get_marked()
                    & 1 << 5
                    != 0
                    && (*(*fr).value.object).get_marked() & (1 << 3 | 1 << 4) != 0
                {
                    luac_barrier_(
                        interpreter,
                        &mut (*(&mut (*((*(*(*interpreter).call_info).function.stkidrel_pointer).tvalue.value.object)))),
                        &mut (*((*fr).value.object as *mut Object)),
                    );
                } else {
                };
            } else {
            };
        }
    }
}
pub unsafe extern "C" fn lua_pushvalue(interpreter: *mut Interpreter, index: i32) {
    unsafe {
        let io1: *mut TValue = &mut (*(*interpreter).top.stkidrel_pointer).tvalue;
        let io2: *const TValue = (*interpreter).index2value(index);
        (*io1).copy_from(&*io2);
        (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
    }
}
pub unsafe fn lua_type(interpreter: *mut Interpreter, index: i32) -> Option<TagType> {
    unsafe {
        let tvalue: *const TValue = (*interpreter).index2value(index);
        return if !(*tvalue).is_tagtype_nil()
            || tvalue != &mut (*(*interpreter).global).none_value as *mut TValue as *const TValue
        {
            return Some((*tvalue).get_tag_type())
        } else {
            None
        };
    }
}
pub unsafe fn lua_typename(mut _state: *mut Interpreter, t: Option<TagType>) -> *const i8 {
    match t {
        None => b"no value\0" as *const u8 as *const i8,
        Some(TagType::Nil) => b"nil\0" as *const u8 as *const i8,
        Some(TagType::Boolean) => b"boolean\0" as *const u8 as *const i8,
        Some(TagType::Pointer) => b"userdata\0" as *const u8 as *const i8,
        Some(TagType::Numeric) => b"number\0" as *const u8 as *const i8,
        Some(TagType::String) => b"string\0" as *const u8 as *const i8,
        Some(TagType::Table) => b"table\0" as *const u8 as *const i8,
        Some(TagType::Closure) => b"function\0" as *const u8 as *const i8,
        Some(TagType::User) => b"userdata\0" as *const u8 as *const i8,
        Some(TagType::State) => b"thread\0" as *const u8 as *const i8,
        Some(TagType::UpValue) => b"upvalue\0" as *const u8 as *const i8,
        Some(TagType::Prototype) => b"proto\0" as *const u8 as *const i8,
        _ => b"unknown\0" as *const u8 as *const i8,
    }
}
pub unsafe extern "C" fn lua_iscfunction(interpreter: *mut Interpreter, index: i32) -> bool {
    unsafe {
        let o: *const TValue = (*interpreter).index2value(index);
        match (*o).get_tag_variant() {
            TAG_VARIANT_CLOSURE_CFUNCTION => return true,
            TAG_VARIANT_CLOSURE_C => return true,
            _ => return false,
        }
    }
}
pub unsafe extern "C" fn lua_isinteger(interpreter: *mut Interpreter, index: i32) -> bool {
    unsafe {
        return (*(*interpreter).index2value(index)).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER;
    }
}
pub unsafe extern "C" fn lua_isnumber(interpreter: *mut Interpreter, index: i32) -> bool {
    unsafe {
        let o: *const TValue = (*interpreter).index2value(index);
        return if (*o).get_tag_variant() == TAG_VARIANT_NUMERIC_NUMBER {
            true
        } else {
            let mut n: f64 = 0.0;
            luav_tonumber_(o, &mut n)
        };
    }
}
pub unsafe extern "C" fn lua_isstring(interpreter: *mut Interpreter, index: i32) -> bool {
    unsafe {
        let o: *const TValue = (*interpreter).index2value(index);
        return match (*o).get_tag_type() {
            TagType::Numeric => true,
            TagType::String => true,
            _ => false,
        };
    }
}
pub unsafe extern "C" fn lua_rawequal(interpreter: *mut Interpreter, index1: i32, index2: i32) -> bool {
    unsafe {
        let o1: *const TValue = (*interpreter).index2value(index1);
        let o2: *const TValue = (*interpreter).index2value(index2);
        return if (!((*o1).is_tagtype_nil())
            || o1 != &mut (*(*interpreter).global).none_value as *mut TValue as *const TValue)
            && (!((*o2).is_tagtype_nil())
                || o2 != &mut (*(*interpreter).global).none_value as *mut TValue as *const TValue)
        {
            luav_equalobj(std::ptr::null_mut(), o1, o2)
        } else {
            false
        };
    }
}
pub unsafe extern "C" fn lua_arith(interpreter: *mut Interpreter, op: i32) {
    unsafe {
        if !(op != 12 as i32 && op != 13 as i32) {
            let io1: *mut TValue = &mut (*(*interpreter).top.stkidrel_pointer).tvalue;
            let io2: *const TValue = &mut (*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))).tvalue;
            (*io1).copy_from(&*io2);
            (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
        }
        luao_arith(
            interpreter,
            op,
            &mut (*(*interpreter).top.stkidrel_pointer.offset(-(2 as isize))).tvalue,
            &mut (*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))).tvalue,
            (*interpreter).top.stkidrel_pointer.offset(-(2 as isize)),
        );
        (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(-1);
    }
}
pub unsafe extern "C" fn lua_compare(interpreter: *mut Interpreter, index1: i32, index2: i32, op: i32) -> i32 {
    unsafe {
        let o1: *const TValue = (*interpreter).index2value(index1);
        let o2: *const TValue = (*interpreter).index2value(index2);
        let mut i: i32 = 0;
        if (!((*o1).is_tagtype_nil())
            || o1 != &mut (*(*interpreter).global).none_value as *mut TValue as *const TValue)
            && (!((*o2).is_tagtype_nil())
                || o2 != &mut (*(*interpreter).global).none_value as *mut TValue as *const TValue)
        {
            match op {
                0 => {
                    i = if luav_equalobj(interpreter, o1, o2) { 1 } else { 0 };
                }
                1 => {
                    i = if luav_lessthan(interpreter, o1, o2) { 1 } else { 0 };
                }
                2 => {
                    i = if luav_lessequal(interpreter, o1, o2) { 1 } else { 0 };
                }
                _ => {}
            }
        }
        return i;
    }
}
pub unsafe extern "C" fn lua_stringtonumber(interpreter: *mut Interpreter, s: *const i8) -> u64 {
    unsafe {
        let size: u64 = luao_str2num(s, &mut (*(*interpreter).top.stkidrel_pointer).tvalue);
        if size != 0 {
            (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
        }
        return size;
    }
}
pub unsafe extern "C" fn lua_tonumberx(interpreter: *mut Interpreter, index: i32, is_number: *mut bool) -> f64 {
    unsafe {
        let mut n: f64 = 0.0;
        let o: *const TValue = (*interpreter).index2value(index);
        let is_number_: bool = if (*o).get_tag_variant() == TAG_VARIANT_NUMERIC_NUMBER {
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
    interpreter: *mut Interpreter,
    index: i32,
    is_number: *mut bool,
) -> i64 {
    unsafe {
        let mut res: i64 = 0;
        let o: *const TValue = (*interpreter).index2value(index);
        let is_number_: bool =
            if (((*o).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER) as i32 != 0) as i64 != 0 {
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
pub unsafe extern "C" fn lua_toboolean(interpreter: *mut Interpreter, index: i32) -> i32 {
    unsafe {
        let o: *const TValue = (*interpreter).index2value(index);
        return !((*o).get_tag_variant() == TAG_VARIANT_BOOLEAN_FALSE
            || (*o).is_tagtype_nil()) as i32;
    }
}
pub unsafe extern "C" fn lua_tolstring(
    interpreter: *mut Interpreter,
    index: i32,
    length: *mut u64,
) -> *const i8 {
    unsafe {
        let mut o: *mut TValue = (*interpreter).index2value(index);
        if !((*o).is_tagtype_string()) {
            if !((*o).is_tagtype_numeric()) {
                if !length.is_null() {
                    *length = 0;
                }
                return std::ptr::null();
            }
            luao_tostring(interpreter, o);
            if (*(*interpreter).global).gc_debt > 0 {
                luac_step(interpreter);
            }
            o = (*interpreter).index2value(index);
        }
        if !length.is_null() {
            *length = (*((*o).value.object as *mut TString)).get_length();
        }
        return (*((*o).value.object as *mut TString)).get_contents_mut();
    }
}
pub unsafe extern "C" fn get_length_raw(interpreter: *mut Interpreter, index: i32) -> usize {
    unsafe {
        let tvalue: *const TValue = (*interpreter).index2value(index);
        match (*tvalue).get_tag_variant() {
            TAG_VARIANT_STRING_SHORT | TAG_VARIANT_STRING_LONG => {
                return (*((*tvalue).value.object as *mut TString)).get_length() as usize;
            },
            TAG_VARIANT_USER => return (*((*tvalue).value.object as *mut User)).count_bytes,
            TAG_VARIANT_TABLE => return luah_getn(&mut (*((*tvalue).value.object as *mut Table))) as usize,
            _ => return 0,
        };
    }
}
pub unsafe extern "C" fn lua_touserdata(interpreter: *mut Interpreter, index: i32) -> *mut libc::c_void {
    unsafe {
        let o: *const TValue = (*interpreter).index2value(index);
        return User::touserdata(o);
    }
}
pub unsafe extern "C" fn lua_tothread(interpreter: *mut Interpreter, index: i32) -> *mut Interpreter {
    unsafe {
        let o: *const TValue = (*interpreter).index2value(index);
        return if !((*o).get_tag_variant() == TAG_VARIANT_STATE) {
            std::ptr::null_mut()
        } else {
            &mut (*((*o).value.object as *mut Interpreter))
        };
    }
}
pub unsafe extern "C" fn lua_pushlstring(
    interpreter: *mut Interpreter,
    s: *const i8,
    length: u64,
) -> *const i8 {
    unsafe {
        let ts: *mut TString = if length == 0 {
            luas_new(interpreter, b"\0" as *const u8 as *const i8)
        } else {
            luas_newlstr(interpreter, s, length)
        };
        let io: *mut TValue = &mut (*(*interpreter).top.stkidrel_pointer).tvalue;
        (*io).value.object = &mut (*(ts as *mut Object));
        (*io).set_tag_variant((*ts).get_tag_variant());
        (*io).set_collectable(true);
        (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
        if (*(*interpreter).global).gc_debt > 0 {
            luac_step(interpreter);
        }
        return (*ts).get_contents_mut();
    }
}
pub unsafe extern "C" fn lua_pushstring(interpreter: *mut Interpreter, mut s: *const i8) -> *const i8 {
    unsafe {
        if s.is_null() {
            (*(*interpreter).top.stkidrel_pointer).tvalue.set_tag_variant(TagVariant::NilNil as u8);
        } else {
            let ts: *mut TString = luas_new(interpreter, s);
            let io: *mut TValue = &mut (*(*interpreter).top.stkidrel_pointer).tvalue;
            (*io).value.object = &mut (*(ts as *mut Object));
            (*io).set_tag_variant((*ts).get_tag_variant());
            (*io).set_collectable(true);
            s = (*ts).get_contents_mut();
        }
        (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
        if (*(*interpreter).global).gc_debt > 0 {
            luac_step(interpreter);
        }
        return s;
    }
}
pub unsafe extern "C" fn lua_pushvfstring(
    interpreter: *mut Interpreter,
    fmt: *const i8,
    mut argp: ::core::ffi::VaList,
) -> *const i8 {
    unsafe {
        let ret: *const i8 = luao_pushvfstring(interpreter, fmt, argp.as_va_list());
        if (*(*interpreter).global).gc_debt > 0 {
            luac_step(interpreter);
        }
        return ret;
    }
}
pub unsafe extern "C" fn lua_pushfstring(
    interpreter: *mut Interpreter,
    fmt: *const i8,
    args: ...
) -> *const i8 {
    unsafe {
        let mut argp: ::core::ffi::VaListImpl;
        argp = args.clone();
        let ret: *const i8 = luao_pushvfstring(interpreter, fmt, argp.as_va_list());
        if (*(*interpreter).global).gc_debt > 0 {
            luac_step(interpreter);
        }
        return ret;
    }
}
pub unsafe extern "C" fn lua_pushcclosure(interpreter: *mut Interpreter, fn_0: CFunction, mut n: i32) {
    unsafe {
        if n == 0 {
            let io: *mut TValue = &mut (*(*interpreter).top.stkidrel_pointer).tvalue;
            (*io).value.function = fn_0;
            (*io).set_tag_variant(TAG_VARIANT_CLOSURE_CFUNCTION);
            (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
        } else {
            let cl: *mut Closure = luaf_newcclosure(interpreter, n);
            (*cl).payload.c_cfunction = fn_0;
            (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(-(n as isize));
            loop {
                let fresh5 = n;
                n = n - 1;
                if !(fresh5 != 0) {
                    break;
                }
                let io1: *mut TValue =
                    &mut *((*cl).upvalues).c_tvalues.as_mut_ptr().offset(n as isize) as *mut TValue;
                let io2: *const TValue = &mut (*(*interpreter).top.stkidrel_pointer.offset(n as isize)).tvalue;
                (*io1).copy_from(&*io2);
            }
            let io_0: *mut TValue = &mut (*(*interpreter).top.stkidrel_pointer).tvalue;
            let x_: *mut Closure = cl;
            (*io_0).value.object = &mut (*(x_ as *mut Object));
            (*io_0).set_tag_variant(TAG_VARIANT_CLOSURE_C);
            (*io_0).set_collectable(true);
            (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
            if (*(*interpreter).global).gc_debt > 0 {
                luac_step(interpreter);
            }
        };
    }
}
pub unsafe extern "C" fn lua_pushlightuserdata(interpreter: *mut Interpreter, p: *mut libc::c_void) {
    unsafe {
        let io: *mut TValue = &mut (*(*interpreter).top.stkidrel_pointer).tvalue;
        (*io).value.pointer = p;
        (*io).set_tag_variant(TAG_VARIANT_POINTER);
        (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
    }
}
pub unsafe extern "C" fn auxgetstr(interpreter: *mut Interpreter, t: *const TValue, k: *const i8) -> i32 {
    unsafe {
        let slot: *const TValue;
        let str: *mut TString = luas_new(interpreter, k);
        if if !((*t).get_tag_variant() == TAG_VARIANT_TABLE) {
            slot = std::ptr::null();
            0
        } else {
            slot = luah_getstr(&mut (*((*t).value.object as *mut Table)), str);
            !(*slot).is_tagtype_nil() as i32
        } != 0
        {
            let io1: *mut TValue = &mut (*(*interpreter).top.stkidrel_pointer).tvalue;
            let io2: *const TValue = slot;
            (*io1).copy_from(&*io2);
            (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
        } else {
            let io: *mut TValue = &mut (*(*interpreter).top.stkidrel_pointer).tvalue;
            (*io).value.object = &mut (*(str as *mut Object));
            (*io).set_tag_variant((*str).get_tag_variant());
            (*io).set_collectable(true);
            (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
            luav_finishget(
                interpreter,
                t,
                &mut (*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))).tvalue,
                (*interpreter).top.stkidrel_pointer.offset(-(1 as isize)),
                slot,
            );
        }
        return ((*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))).tvalue.get_tag_type()) as i32;
    }
}
pub unsafe extern "C" fn lua_getglobal(interpreter: *mut Interpreter, name: *const i8) -> i32 {
    unsafe {
        let global_table: *const TValue = &mut *((*((*(*interpreter).global).l_registry.value.object
            as *mut Table))
            .array)
            .offset((2 - 1) as isize) as *mut TValue;
        return auxgetstr(interpreter, global_table, name);
    }
}
pub unsafe extern "C" fn lua_gettable(interpreter: *mut Interpreter, index: i32) -> i32 {
    unsafe {
        let slot;
        let t: *mut TValue = (*interpreter).index2value(index);
        if if (*t).get_tag_variant() != TAG_VARIANT_TABLE {
            slot = std::ptr::null();
            0
        } else {
            slot = luah_get(
                &mut (*((*t).value.object as *mut Table)),
                &mut (*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))).tvalue,
            );
            !(*slot).is_tagtype_nil() as i32
        } != 0
        {
            let io1: *mut TValue = &mut (*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))).tvalue;
            let io2: *const TValue = slot;
            (*io1).copy_from(&*io2);
        } else {
            luav_finishget(
                interpreter,
                t,
                &mut (*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))).tvalue,
                (*interpreter).top.stkidrel_pointer.offset(-(1 as isize)),
                slot,
            );
        }
        return (((*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))).tvalue.get_tag_type())) as i32;
    }
}
pub unsafe extern "C" fn handle_luainit(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let mut name: *const i8 = b"=LUA_INIT_5_4\0" as *const u8 as *const i8;
        let mut init: *const i8 = getenv(name.offset(1 as isize));
        if init.is_null() {
            name = b"=LUA_INIT\0" as *const u8 as *const i8;
            init = getenv(name.offset(1 as isize));
        }
        if init.is_null() {
            return 0;
        } else if *init.offset(0 as isize) as i32 == CHARACTER_AT as i32 {
            return dofile(interpreter, init.offset(1 as isize));
        } else {
            return dostring(interpreter, init, name);
        };
    }
}
pub unsafe extern "C" fn lua_getfield(interpreter: *mut Interpreter, index: i32, k: *const i8) -> i32 {
    unsafe {
        return auxgetstr(interpreter, (*interpreter).index2value(index), k);
    }
}
pub unsafe extern "C" fn lua_geti(interpreter: *mut Interpreter, index: i32, n: i64) -> i32 {
    unsafe {
        let t: *mut TValue;
        let slot: *const TValue;
        t = (*interpreter).index2value(index);
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
            !(*slot).is_tagtype_nil() as i32
        } != 0
        {
            let io1: *mut TValue = &mut (*(*interpreter).top.stkidrel_pointer).tvalue;
            let io2: *const TValue = slot;
            (*io1).copy_from(&*io2);
        } else {
            let mut aux: TValue = TValue::new(TAG_VARIANT_NIL_NIL);
            let io: *mut TValue = &mut aux;
            (*io).value.integer = n;
            (*io).set_tag_variant(TAG_VARIANT_NUMERIC_INTEGER);
            luav_finishget(interpreter, t, &mut aux, (*interpreter).top.stkidrel_pointer, slot);
        }
        (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
        return (((*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))).tvalue.get_tag_type())) as i32;
    }
}
pub unsafe extern "C" fn finishrawget(interpreter: *mut Interpreter, value: *const TValue) -> TagType {
    unsafe {
        if (*value).is_tagtype_nil() {
            (*(*interpreter).top.stkidrel_pointer).tvalue.set_tag_variant(TagVariant::NilNil as u8);
        } else {
            let io1: *mut TValue = &mut (*(*interpreter).top.stkidrel_pointer).tvalue;
            let io2: *const TValue = value;
            (*io1).copy_from(&*io2);
        }
        (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
        return (*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))).tvalue.get_tag_type();
    }
}
pub unsafe extern "C" fn gettable(interpreter: *mut Interpreter, index: i32) -> *mut Table {
    unsafe {
        let t: *mut TValue = (*interpreter).index2value(index);
        return &mut (*((*t).value.object as *mut Table));
    }
}
pub unsafe extern "C" fn lua_rawget(interpreter: *mut Interpreter, index: i32) -> TagType {
    unsafe {
        let table: *mut Table = gettable(interpreter, index);
        let value: *const TValue =
            luah_get(table, &mut (*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))).tvalue);
        (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(-1);
        return finishrawget(interpreter, value);
    }
}
pub unsafe extern "C" fn lua_rawgeti(interpreter: *mut Interpreter, index: i32, n: i64) -> TagType {
    unsafe {
        let table: *mut Table = gettable(interpreter, index);
        return finishrawget(interpreter, luah_getint(table, n));
    }
}
pub unsafe extern "C" fn auxsetstr(interpreter: *mut Interpreter, t: *const TValue, k: *const i8) {
    unsafe {
        let slot: *const TValue;
        let str: *mut TString = luas_new(interpreter, k);
        if if !((*t).get_tag_variant() == TAG_VARIANT_TABLE) {
            slot = std::ptr::null();
            0
        } else {
            slot = luah_getstr(&mut (*((*t).value.object as *mut Table)), str);
            !(*slot).is_tagtype_nil() as i32
        } != 0
        {
            let io1: *mut TValue = slot as *mut TValue;
            let io2: *const TValue = &mut (*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))).tvalue;
            (*io1).copy_from(&*io2);
            if (*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize)))
                .tvalue
                .is_collectable()
            {
                if (*(*t).value.object).get_marked() & 1 << 5 != 0
                    && (*(*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))).tvalue.value.object).get_marked()
                        & (1 << 3 | 1 << 4)
                        != 0
                {
                    luac_barrierback_(interpreter, (*t).value.object);
                } else {
                };
            } else {
            };
            (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(-1);
        } else {
            let io: *mut TValue = &mut (*(*interpreter).top.stkidrel_pointer).tvalue;
            (*io).value.object = &mut (*(str as *mut Object));
            (*io).set_tag_variant((*str).get_tag_variant());
            (*io).set_collectable(true);
            (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
            luav_finishset(
                interpreter,
                t,
                &mut (*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))).tvalue,
                &mut (*(*interpreter).top.stkidrel_pointer.offset(-(2 as isize))).tvalue,
                slot,
            );
            (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(-(2 as isize));
        };
    }
}
pub unsafe extern "C" fn lua_setglobal(interpreter: *mut Interpreter, name: *const i8) {
    unsafe {
        let global_table: *const TValue = &mut *((*((*(*interpreter).global).l_registry.value.object
            as *mut Table))
            .array)
            .offset((2 - 1) as isize) as *mut TValue;
        auxsetstr(interpreter, global_table, name);
    }
}
pub unsafe extern "C" fn lua_setfield(interpreter: *mut Interpreter, index: i32, k: *const i8) {
    unsafe {
        auxsetstr(interpreter, (*interpreter).index2value(index), k);
    }
}
pub unsafe extern "C" fn lua_seti(interpreter: *mut Interpreter, index: i32, n: i64) {
    unsafe {
        let t: *mut TValue;
        let slot: *const TValue;
        t = (*interpreter).index2value(index);
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
            !(*slot).is_tagtype_nil() as i32
        } != 0
        {
            let io1: *mut TValue = slot as *mut TValue;
            let io2: *const TValue = &mut (*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))).tvalue;
            (*io1).copy_from(&*io2);
            if (*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize)))
                .tvalue
                .is_collectable()
            {
                if (*(*t).value.object).get_marked() & 1 << 5 != 0
                    && (*(*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))).tvalue.value.object).get_marked()
                        & (1 << 3 | 1 << 4)
                        != 0
                {
                    luac_barrierback_(interpreter, (*t).value.object);
                } else {
                };
            } else {
            };
        } else {
            let mut aux: TValue = TValue::new(TAG_VARIANT_NUMERIC_INTEGER);
            aux.value.integer = n;
            luav_finishset(
                interpreter,
                t,
                &mut aux,
                &mut (*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))).tvalue,
                slot,
            );
        }
        (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(-1);
    }
}
pub unsafe extern "C" fn aux_rawset(interpreter: *mut Interpreter, index: i32, key: *mut TValue, n: i32) {
    unsafe {
        let table: *mut Table = gettable(interpreter, index);
        luah_set(
            interpreter,
            table,
            key,
            &mut (*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))).tvalue,
        );
        (*table).flags = ((*table).flags as u32 & !!(!0 << TM_EQ as i32 + 1)) as u8;
        if (*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize)))
            .tvalue
            .is_collectable()
        {
            if (*(table as *mut Object)).get_marked() & 1 << 5 != 0
                && (*(*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))).tvalue.value.object).get_marked()
                    & (1 << 3 | 1 << 4)
                    != 0
            {
                luac_barrierback_(interpreter, &mut (*(table as *mut Object)));
            } else {
            };
        } else {
        };
        (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(-(n as isize));
    }
}
pub unsafe extern "C" fn lua_rawset(interpreter: *mut Interpreter, index: i32) {
    unsafe {
        aux_rawset(
            interpreter,
            index,
            &mut (*(*interpreter).top.stkidrel_pointer.offset(-(2 as isize))).tvalue,
            2,
        );
    }
}
pub unsafe extern "C" fn lua_rawseti(interpreter: *mut Interpreter, index: i32, n: i64) {
    unsafe {
        let table: *mut Table = gettable(interpreter, index);
        luah_setint(
            interpreter,
            table,
            n,
            &mut (*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))).tvalue,
        );
        if (*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize)))
            .tvalue
            .is_collectable()
        {
            if (*(table as *mut Object)).get_marked() & 1 << 5 != 0
                && (*(*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))).tvalue.value.object).get_marked()
                    & (1 << 3 | 1 << 4)
                    != 0
            {
                luac_barrierback_(interpreter, &mut (*(table as *mut Object)));
            } else {
            };
        } else {
        };
        (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(-1);
    }
}
pub unsafe extern "C" fn lua_setmetatable(interpreter: *mut Interpreter, index: i32) -> i32 {
    unsafe {
        let metatable: *mut Table;
        let object: *mut TValue = (*interpreter).index2value(index);
        if (*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))).tvalue.is_tagtype_nil() {
            metatable = std::ptr::null_mut();
        } else {
            metatable = &mut (*((*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))).tvalue.value.object
                as *mut Table))
        }
        match (*object).get_tag_type() {
            TagType::Table => {
                (*((*object).value.object as *mut Table)).metatable = metatable;
                if !metatable.is_null() {
                    if (*(*object).value.object).get_marked() & 1 << 5 != 0
                        && (*metatable).get_marked() & (1 << 3 | 1 << 4) != 0
                    {
                        luac_barrier_(
                            interpreter,
                            &mut (*((*object).value.object as *mut Object)),
                            &mut (*(metatable as *mut Object)),
                        );
                    } else {
                    };
                    luac_checkfinalizer(interpreter, (*object).value.object, metatable);
                }
            }
            TagType::User => {
                (*((*object).value.object as *mut User)).metatable = metatable;
                if !metatable.is_null() {
                    if (*((*object).value.object as *mut User)).get_marked() & 1 << 5 != 0
                        && (*metatable).get_marked() & (1 << 3 | 1 << 4) != 0
                    {
                        luac_barrier_(
                            interpreter,
                            &mut (*(&mut (*((*object).value.object as *mut User)) as *mut User
                                as *mut Object)),
                            &mut (*(metatable as *mut Object)),
                        );
                    } else {
                    };
                    luac_checkfinalizer(interpreter, (*object).value.object, metatable);
                }
            }
            _ => {
                (*(*interpreter).global).metatables[(((*object).get_tag_type())) as usize] = metatable;
            }
        }
        (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(-1);
        return 1;
    }
}
pub unsafe extern "C" fn lua_setiuservalue(interpreter: *mut Interpreter, index: i32, n: i32) -> i32 {
    unsafe {
        let res: i32;
        let o: *mut TValue = (*interpreter).index2value(index);
        if !((n as u32).wrapping_sub(1 as u32)
            < (*((*o).value.object as *mut User)).count_upvalues as u32)
        {
            res = 0;
        } else {
            let io1: *mut TValue = &mut (*((*((*o).value.object as *mut User)).upvalues)
                .as_mut_ptr()
                .offset((n - 1) as isize));
            let io2: *const TValue = &mut (*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))).tvalue;
            (*io1).copy_from(&*io2);
            if (*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize)))
                .tvalue
                .is_collectable()
            {
                if (*(*o).value.object).get_marked() & 1 << 5 != 0
                    && (*(*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))).tvalue.value.object).get_marked()
                        & (1 << 3 | 1 << 4)
                        != 0
                {
                    luac_barrierback_(interpreter, (*o).value.object);
                } else {
                };
            } else {
            };
            res = 1;
        }
        (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(-1);
        return res;
    }
}
pub unsafe extern "C" fn lua_callk(
    interpreter: *mut Interpreter,
    nargs: i32,
    count_results: i32,
    ctx: i64,
    k: ContextFunction,
) {
    unsafe {
        let function: StackValuePointer = (*interpreter).top.stkidrel_pointer.offset(-((nargs + 1) as isize));
        if k.is_some() && (*interpreter).count_c_calls & 0xffff0000 as u32 == 0 {
            (*(*interpreter).call_info).u.c.context_function = k;
            (*(*interpreter).call_info).u.c.context = ctx;
            ccall(interpreter, function, count_results, 1);
        } else {
            luad_callnoyield(interpreter, function, count_results);
        }
        if count_results <= -1 && (*(*interpreter).call_info).top.stkidrel_pointer < (*interpreter).top.stkidrel_pointer {
            (*(*interpreter).call_info).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer;
        }
    }
}
pub unsafe extern "C" fn f_call(interpreter: *mut Interpreter, arbitrary_data: *mut libc::c_void) {
    unsafe {
        let calls: *mut CallS = arbitrary_data as *mut CallS;
        luad_callnoyield(interpreter, (*calls).function, (*calls).count_results);
    }
}
pub unsafe extern "C" fn lua_pcallk(
    interpreter: *mut Interpreter,
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
            let o: StackValuePointer = index2stack(interpreter, error_function);
            function = (o as *mut i8).offset_from((*interpreter).stack.stkidrel_pointer as *mut i8) as i64;
        }
        calls.function = (*interpreter).top.stkidrel_pointer.offset(-((nargs + 1) as isize));
        if k.is_none() || !((*interpreter).count_c_calls & 0xffff0000 as u32 == 0) {
            calls.count_results = count_results;
            status = luad_pcall(
                interpreter,
                Some(f_call as unsafe extern "C" fn(*mut Interpreter, *mut libc::c_void) -> ()),
                &mut calls as *mut CallS as *mut libc::c_void,
                (calls.function as *mut i8).offset_from((*interpreter).stack.stkidrel_pointer as *mut i8) as i64,
                function,
            );
        } else {
            let call_info: *mut CallInfo = (*interpreter).call_info;
            (*call_info).u.c.context_function = k;
            (*call_info).u.c.context = ctx;
            (*call_info).u2.funcidx =
                (calls.function as *mut i8).offset_from((*interpreter).stack.stkidrel_pointer as *mut i8) as i32;
            (*call_info).u.c.old_error_function = (*interpreter).error_function;
            (*interpreter).error_function = function;
            (*call_info).call_status =
                ((*call_info).call_status as i32 & !(1 << 0) | (*interpreter).allow_hook as i32) as u16;
            (*call_info).call_status = ((*call_info).call_status as i32 | 1 << 4) as u16;
            ccall(interpreter, calls.function, count_results, 1);
            (*call_info).call_status = ((*call_info).call_status as i32 & !(1 << 4)) as u16;
            (*interpreter).error_function = (*call_info).u.c.old_error_function;
            status = 0;
        }
        if count_results <= -1 && (*(*interpreter).call_info).top.stkidrel_pointer < (*interpreter).top.stkidrel_pointer {
            (*(*interpreter).call_info).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer;
        }
        return status;
    }
}
pub unsafe extern "C" fn lua_load(
    interpreter: *mut Interpreter,
    reader: ReadFunction,
    data: *mut libc::c_void,
    mut chunkname: *const i8,
    mode: *const i8,
) -> i32 {
    unsafe {
        let mut zio: ZIO = ZIO::new(interpreter, reader, data);
        if chunkname.is_null() {
            chunkname = b"?\0" as *const u8 as *const i8;
        }
        let status: i32 = luad_protectedparser(interpreter, &mut zio, chunkname, mode);
        if status == 0 {
            let closure: *mut Closure =
                &mut (*((*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))).tvalue.value.object
                    as *mut Closure));
            if (*closure).count_upvalues as i32 >= 1 {
                let gt: *const TValue =
                    &mut *((*((*(*interpreter).global).l_registry.value.object as *mut Table))
                        .array)
                        .offset((2 - 1) as isize) as *mut TValue;
                let io1: *mut TValue = (**((*closure).upvalues).l_upvalues.as_mut_ptr().offset(0 as isize)).v.p;
                let io2: *const TValue = gt;
                (*io1).copy_from(&*io2);
                if (*gt).is_collectable() {
                    if (**((*closure).upvalues).l_upvalues.as_mut_ptr().offset(0 as isize)).get_marked() & 1 << 5
                        != 0
                        && (*(*gt).value.object).get_marked() & (1 << 3 | 1 << 4) != 0
                    {
                        luac_barrier_(
                            interpreter,
                            &mut (*(*((*closure).upvalues).l_upvalues.as_mut_ptr().offset(0 as isize)
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
    interpreter: *mut Interpreter,
    writer_0: WriteFunction,
    data: *mut libc::c_void,
    is_strip: bool,
) -> i32 {
    unsafe {
        let status: i32;
        let o: *mut TValue = &mut (*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))).tvalue;
        if (*o).get_tag_variant() == TAG_VARIANT_CLOSURE_L {
            status = save_prototype(
                interpreter,
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
pub unsafe extern "C" fn lua_gc(interpreter: *mut Interpreter, what: i32, args: ...) -> i32 {
    unsafe {
        let mut argp: ::core::ffi::VaListImpl;
        let mut res: i32 = 0;
        let global: *mut Global = (*interpreter).global;
        if (*global).gc_step as i32 & 2 != 0 {
            return -1;
        }
        argp = args.clone();
        match what {
            0 => {
                (*global).gc_step = 1;
            }
            1 => {
                (*global).set_debt(0);
                (*global).gc_step = 0;
            }
            2 => {
                luac_fullgc(interpreter, false);
            }
            3 => {
                res = (((*global).total_bytes + (*global).gc_debt) as u64 >> 10 as i32) as i32;
            }
            4 => {
                res = (((*global).total_bytes + (*global).gc_debt) as u64 & 0x3ff as u64) as i32;
            }
            5 => {
                let data: i32 = argp.arg::<i32>();
                let mut debt: i64 = 1;
                let oldstp: u8 = (*global).gc_step;
                (*global).gc_step = 0;
                if data == 0 {
                    (*global).set_debt(0);
                    luac_step(interpreter);
                } else {
                    debt = data as i64 * 1024 as i64 + (*global).gc_debt;
                    (*global).set_debt(debt);
                    if (*(*interpreter).global).gc_debt > 0 {
                        luac_step(interpreter);
                    }
                }
                (*global).gc_step = oldstp;
                if debt > 0 && (*global).gc_state as i32 == 8 {
                    res = 1;
                }
            }
            6 => {
                let data_0: i32 = argp.arg::<i32>();
                res = (*global).gc_pause as i32 * 4;
                (*global).gc_pause = (data_0 / 4) as u8;
            }
            7 => {
                let data_1: i32 = argp.arg::<i32>();
                res = (*global).gc_step_multiplier as i32 * 4;
                (*global).gc_step_multiplier = (data_1 / 4) as u8;
            }
            9 => {
                res = ((*global).gc_step as i32 == 0) as i32;
            }
            10 => {
                let minormul: i32 = argp.arg::<i32>();
                let majormul: i32 = argp.arg::<i32>();
                res = if (*global).gc_kind as i32 == 1 || (*global).last_atomic != 0 {
                    10 as i32
                } else {
                    11 as i32
                };
                if minormul != 0 {
                    (*global).generational_minor_multiplier = minormul as u64;
                }
                if majormul != 0 {
                    (*global).generational_major_multiplier = (majormul / 4) as u64;
                }
                luac_changemode(interpreter, 1);
            }
            11 => {
                let pause: i32 = argp.arg::<i32>();
                let stepmul: i32 = argp.arg::<i32>();
                let stepsize: i32 = argp.arg::<i32>();
                res = if (*global).gc_kind as i32 == 1 || (*global).last_atomic != 0 {
                    10 as i32
                } else {
                    11 as i32
                };
                if pause != 0 {
                    (*global).gc_pause = (pause / 4) as u8;
                }
                if stepmul != 0 {
                    (*global).gc_step_multiplier = (stepmul / 4) as u8;
                }
                if stepsize != 0 {
                    (*global).gc_step_size = stepsize as u8;
                }
                luac_changemode(interpreter, 0);
            }
            _ => {
                res = -1;
            }
        }
        return res;
    }
}
pub unsafe extern "C" fn lua_error(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let errobj: *mut TValue = &mut (*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))).tvalue;
        if (*errobj).get_tag_variant() == TAG_VARIANT_STRING_SHORT
            && &mut (*((*errobj).value.object as *mut TString)) as *mut TString
                == (*(*interpreter).global).memory_error_message
        {
            luad_throw(interpreter, 4);
        } else {
            luag_errormsg(interpreter);
        };
    }
}
pub unsafe extern "C" fn lua_next(interpreter: *mut Interpreter, index: i32) -> i32 {
    unsafe {
        let table: *mut Table = gettable(interpreter, index);
        let more: i32 = luah_next(interpreter, table, (*interpreter).top.stkidrel_pointer.offset(-(1 as isize)));
        if more != 0 {
            (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
        } else {
            (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(-(1 as isize));
        }
        return more;
    }
}
pub unsafe extern "C" fn lua_toclose(interpreter: *mut Interpreter, index: i32) {
    unsafe {
        let o: StackValuePointer = index2stack(interpreter, index);
        let count_results: i32 = (*(*interpreter).call_info).count_results as i32;
        luaf_newtbcupval(interpreter, o);
        if !(count_results < -1) {
            (*(*interpreter).call_info).count_results = -count_results - 3;
        }
    }
}
pub unsafe extern "C" fn lua_concat(interpreter: *mut Interpreter, n: i32) {
    unsafe {
        if n > 0 {
            concatenate(interpreter, n);
        } else {
            let io: *mut TValue = &mut (*(*interpreter).top.stkidrel_pointer).tvalue;
            let ts: *mut TString = luas_newlstr(interpreter, b"\0" as *const u8 as *const i8, 0u64);
            (*io).value.object = &mut (*(ts as *mut Object));
            (*io).set_tag_variant((*ts).get_tag_variant());
            (*io).set_collectable(true);
            (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
        }
        if (*(*interpreter).global).gc_debt > 0 {
            luac_step(interpreter);
        }
    }
}
pub unsafe extern "C" fn lua_len(interpreter: *mut Interpreter, index: i32) {
    unsafe {
        let t: *mut TValue = (*interpreter).index2value(index);
        luav_objlen(interpreter, (*interpreter).top.stkidrel_pointer, t);
        (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
    }
}
pub unsafe extern "C" fn lua_setwarnf(interpreter: *mut Interpreter, f: WarnFunction, arbitrary_data: *mut libc::c_void) {
    unsafe {
        (*(*interpreter).global).warn_userdata = arbitrary_data;
        (*(*interpreter).global).warn_function = f;
    }
}
pub unsafe extern "C" fn lua_warning(interpreter: *mut Interpreter, message: *const i8, tocont: i32) {
    unsafe {
        luae_warning(interpreter, message, tocont);
    }
}
pub unsafe extern "C" fn lua_getupvalue(interpreter: *mut Interpreter, funcindex: i32, n: i32) -> *const i8 {
    unsafe {
        let mut value: *mut TValue = std::ptr::null_mut();
        let name: *const i8 = aux_upvalue(
            (*interpreter).index2value(funcindex),
            n,
            &mut value,
            std::ptr::null_mut(),
        );
        if !name.is_null() {
            let io1: *mut TValue = &mut (*(*interpreter).top.stkidrel_pointer).tvalue;
            let io2: *const TValue = value;
            (*io1).copy_from(&*io2);
            (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
        }
        return name;
    }
}
pub unsafe extern "C" fn lua_setupvalue(interpreter: *mut Interpreter, funcindex: i32, n: i32) -> *const i8 {
    unsafe {
        let mut value: *mut TValue = std::ptr::null_mut();
        let mut owner: *mut Object = std::ptr::null_mut();
        let fi: *mut TValue = (*interpreter).index2value(funcindex);
        let name: *const i8 = aux_upvalue(fi, n, &mut value, &mut owner);
        if !name.is_null() {
            (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(-1);
            let io1: *mut TValue = value;
            let io2: *const TValue = &mut (*(*interpreter).top.stkidrel_pointer).tvalue;
            (*io1).copy_from(&*io2);
            if (*value).is_collectable() {
                if (*owner).get_marked() & 1 << 5 != 0
                    && (*(*value).value.object).get_marked() & (1 << 3 | 1 << 4) != 0
                {
                    luac_barrier_(
                        interpreter,
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
    interpreter: *mut Interpreter,
    fidx: i32,
    n: i32,
    pf: *mut *mut Closure,
) -> *mut *mut UpValue {
    unsafe {
        let fi: *mut TValue = (*interpreter).index2value(fidx);
        let closure: *mut Closure = &mut (*((*fi).value.object as *mut Closure));
        if !pf.is_null() {
            *pf = closure;
        }
        if 1 <= n && n <= (*(*closure).payload.l_prototype).prototype_upvalues.size {
            return &mut *((*closure).upvalues).l_upvalues.as_mut_ptr().offset((n - 1) as isize)
                as *mut *mut UpValue;
        } else {
            return &NULLUP as *const *const UpValue as *mut *mut UpValue;
        };
    }
}
pub unsafe extern "C" fn lua_upvalueid(interpreter: *mut Interpreter, fidx: i32, n: i32) -> *mut libc::c_void {
    unsafe {
        let fi: *mut TValue = (*interpreter).index2value(fidx);
        match (*fi).get_tag_variant() {
            TAG_VARIANT_CLOSURE_L => {
                return *getupvalref(interpreter, fidx, n, std::ptr::null_mut()) as *mut libc::c_void;
            }
            TAG_VARIANT_CLOSURE_C => {
                let closure: *mut Closure = &mut (*((*fi).value.object as *mut Closure));
                if 1 <= n && n <= (*closure).count_upvalues as i32 {
                    return &mut *((*closure).upvalues).c_tvalues.as_mut_ptr().offset((n - 1) as isize) as *mut TValue
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
    interpreter: *mut Interpreter,
    fidx1: i32,
    n1: i32,
    fidx2: i32,
    n2: i32,
) {
    unsafe {
        let mut f1: *mut Closure = std::ptr::null_mut();
        let up1: *mut *mut UpValue = getupvalref(interpreter, fidx1, n1, &mut f1);
        let up2: *mut *mut UpValue = getupvalref(interpreter, fidx2, n2, std::ptr::null_mut());
        *up1 = *up2;
        if (*f1).get_marked() & 1 << 5 != 0 && (**up1).get_marked() & (1 << 3 | 1 << 4) != 0 {
            luac_barrier_(
                interpreter,
                &mut (*(f1 as *mut Object)),
                &mut (*(*up1 as *mut Object)),
            );
        } else {
        };
    }
}
pub unsafe fn luai_makeseed(interpreter: *mut Interpreter) -> u32 {
    unsafe {
        let mut buffer: [i8; 24] = [0; 24];
        let mut h: u32 = time(std::ptr::null_mut()) as u32;
        let mut p: i32 = 0;
        let mut t: u64 = interpreter as u64;
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
            Option<unsafe fn (interpreter: *mut Interpreter) -> u32>,
            u64,
        >(Some(luai_makeseed as unsafe fn (interpreter: *mut Interpreter) -> u32));
        memcpy(
            buffer.as_mut_ptr().offset(p as isize) as *mut libc::c_void,
            &mut t_1 as *mut u64 as *const libc::c_void,
            ::core::mem::size_of::<u64>() as u64,
        );
        p = (p as u64).wrapping_add(::core::mem::size_of::<u64>() as u64) as i32;
        return luas_hash(buffer.as_mut_ptr(), p as u64, h);
    }
}
pub unsafe extern "C" fn luae_extendci(interpreter: *mut Interpreter) -> *mut CallInfo {
    unsafe {
        let ret = luam_malloc_(interpreter, ::core::mem::size_of::<CallInfo>()) as *mut CallInfo;
        (*(*interpreter).call_info).next = ret;
        (*ret).previous = (*interpreter).call_info;
        (*ret).next = std::ptr::null_mut();
        ::core::ptr::write_volatile(&mut (*ret).u.l.trap as *mut i32, 0);
        (*interpreter).count_call_info = ((*interpreter).count_call_info).wrapping_add(1);
        (*interpreter).count_call_info;
        return ret;
    }
}
pub unsafe extern "C" fn freeci(interpreter: *mut Interpreter) {
    unsafe {
        let mut call_info: *mut CallInfo = (*interpreter).call_info;
        let mut next: *mut CallInfo = (*call_info).next;
        (*call_info).next = std::ptr::null_mut();
        loop {
            call_info = next;
            if call_info.is_null() {
                break;
            }
            next = (*call_info).next;
            (*interpreter).free_memory(
                call_info as *mut libc::c_void,
                ::core::mem::size_of::<CallInfo>(),
            );
            (*interpreter).count_call_info = ((*interpreter).count_call_info).wrapping_sub(1);
            (*interpreter).count_call_info;
        }
    }
}
pub unsafe extern "C" fn luae_shrinkci(interpreter: *mut Interpreter) {
    unsafe {
        let mut call_info: *mut CallInfo = (*(*interpreter).call_info).next;
        if !call_info.is_null() {
            let mut next: *mut CallInfo;
            loop {
                next = (*call_info).next;
                if next.is_null() {
                    break;
                }
                let next2: *mut CallInfo = (*next).next;
                (*call_info).next = next2;
                (*interpreter).count_call_info = ((*interpreter).count_call_info).wrapping_sub(1);
                (*interpreter).count_call_info;
                (*interpreter).free_memory(
                    next as *mut libc::c_void,
                    ::core::mem::size_of::<CallInfo>(),
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
pub unsafe extern "C" fn stack_init(other_state: *mut Interpreter, interpreter: *mut Interpreter) {
    unsafe {
        (*other_state).stack.stkidrel_pointer = luam_malloc_(
            interpreter,
            ((2 * 20 as i32 + 5) as usize).wrapping_mul(::core::mem::size_of::<StackValue>()),
        ) as *mut StackValue;
        (*other_state).tbc_list.stkidrel_pointer = (*other_state).stack.stkidrel_pointer;
        for i in 0..2 * 20 as i32 + 5 {
            (*((*other_state).stack.stkidrel_pointer).offset(i as isize))
                .tvalue
                .set_tag_variant(TagVariant::NilNil as u8);
        }
        (*other_state).top.stkidrel_pointer = (*other_state).stack.stkidrel_pointer;
        (*other_state).stack_last.stkidrel_pointer = ((*other_state).stack.stkidrel_pointer).offset((2 * 20 as i32) as isize);
        let call_info = &mut (*other_state).base_callinfo;
        (*call_info).previous = std::ptr::null_mut();
        (*call_info).next = (*call_info).previous;
        (*call_info).call_status = (1 << 1) as u16;
        (*call_info).function.stkidrel_pointer = (*other_state).top.stkidrel_pointer;
        (*call_info).u.c.context_function = None;
        (*call_info).count_results = 0;
        (*(*other_state).top.stkidrel_pointer).tvalue.set_tag_variant(TagVariant::NilNil as u8);
        (*other_state).top.stkidrel_pointer = ((*other_state).top.stkidrel_pointer).offset(1);
        (*other_state).top.stkidrel_pointer;
        (*call_info).top.stkidrel_pointer = ((*other_state).top.stkidrel_pointer).offset(20 as isize);
        (*other_state).call_info = call_info;
    }
}
pub unsafe extern "C" fn freestack(interpreter: *mut Interpreter) {
    unsafe {
        if ((*interpreter).stack.stkidrel_pointer).is_null() {
            return;
        }
        (*interpreter).call_info = &mut (*interpreter).base_callinfo;
        freeci(interpreter);
        (*interpreter).free_memory(
            (*interpreter).stack.stkidrel_pointer as *mut libc::c_void,
            ((((*interpreter).stack_last.stkidrel_pointer).offset_from((*interpreter).stack.stkidrel_pointer) as i32 + 5) as u64)
                .wrapping_mul(::core::mem::size_of::<StackValue>() as u64) as usize,
        );
    }
}
pub unsafe extern "C" fn init_registry(interpreter: *mut Interpreter, global: *mut Global) {
    unsafe {
        let registry: *mut Table = luah_new(interpreter);
        let io: *mut TValue = &mut (*global).l_registry;
        let x_: *mut Table = registry;
        (*io).value.object = &mut (*(x_ as *mut Object));
        (*io).set_tag_variant(TAG_VARIANT_TABLE);
        (*io).set_collectable(true);
        luah_resize(interpreter, registry, 2, 0);
        let io_0: *mut TValue = &mut *((*registry).array).offset((1 - 1) as isize) as *mut TValue;
        let x0: *mut Interpreter = interpreter;
        (*io_0).value.object = &mut (*(x0 as *mut Object));
        (*io_0).set_tag_variant(TAG_VARIANT_STATE);
        (*io_0).set_collectable(true);
        let io_1: *mut TValue = &mut *((*registry).array).offset((2 - 1) as isize) as *mut TValue;
        let x1: *mut Table = luah_new(interpreter);
        (*io_1).value.object = &mut (*(x1 as *mut Object));
        (*io_1).set_tag_variant(TAG_VARIANT_TABLE);
        (*io_1).set_collectable(true);
    }
}
pub unsafe extern "C" fn f_luaopen(interpreter: *mut Interpreter, mut _ud: *mut libc::c_void) {
    unsafe {
        let global: *mut Global = (*interpreter).global;
        stack_init(interpreter, interpreter);
        init_registry(interpreter, global);
        luas_init_state(interpreter);
        luat_init(interpreter);
        luax_init(interpreter);
        (*global).gc_step = 0;
        (*global).none_value.set_tag_variant(TagVariant::NilNil as u8);
    }
}
pub unsafe extern "C" fn preinit_thread(interpreter: *mut Interpreter, global: *mut Global) {
    unsafe {
        (*interpreter).global = global;
        (*interpreter).stack.stkidrel_pointer = std::ptr::null_mut();
        (*interpreter).call_info = std::ptr::null_mut();
        (*interpreter).count_call_info = 0;
        (*interpreter).twups = interpreter;
        (*interpreter).count_c_calls = 0;
        (*interpreter).long_jump = std::ptr::null_mut();
        ::core::ptr::write_volatile(&mut (*interpreter).hook as *mut HookFunction, None);
        ::core::ptr::write_volatile(&mut (*interpreter).hook_mask as *mut i32, 0);
        (*interpreter).base_hook_count = 0;
        (*interpreter).allow_hook = 1;
        (*interpreter).hook_count = (*interpreter).base_hook_count;
        (*interpreter).open_upvalue = std::ptr::null_mut();
        (*interpreter).status = 0;
        (*interpreter).error_function = 0;
        (*interpreter).old_program_counter = 0;
    }
}
pub unsafe extern "C" fn close_state(interpreter: *mut Interpreter) {
    unsafe {
        let global: *mut Global = (*interpreter).global;
        if (*global).none_value.is_tagtype_nil() {
            (*interpreter).call_info = &mut (*interpreter).base_callinfo;
            (*interpreter).error_function = 0;
            luad_closeprotected(interpreter, 1 as i64, 0);
            (*interpreter).top.stkidrel_pointer = ((*interpreter).stack.stkidrel_pointer).offset(1 as isize);
            luac_freeallobjects(interpreter);
        } else {
            luac_freeallobjects(interpreter);
        }
        (*interpreter).free_memory(
            (*(*interpreter).global).string_table.hash as *mut libc::c_void,
            ((*(*interpreter).global).string_table.size as u64)
                .wrapping_mul(::core::mem::size_of::<*mut TString>() as u64) as usize,
        );
        freestack(interpreter);
        raw_allocate(
            interpreter as *mut u8 as *mut libc::c_void,
            ::core::mem::size_of::<Interpreter>(),
            0,
        );
        raw_allocate(
            global as *mut u8 as *mut libc::c_void,
            ::core::mem::size_of::<Global>(),
            0,
        );
    }
}
pub unsafe extern "C" fn lua_newthread(interpreter: *mut Interpreter) -> *mut Interpreter {
    unsafe {
        let global: *mut Global = (*interpreter).global;
        if (*(*interpreter).global).gc_debt > 0 {
            luac_step(interpreter);
        }
        let ret = luac_newobj(interpreter, TAG_VARIANT_STATE, ::core::mem::size_of::<Interpreter>()) as *mut Interpreter;
        let io: *mut TValue = &mut (*(*interpreter).top.stkidrel_pointer).tvalue;
        (*io).set_tag_variant(TAG_VARIANT_STATE);
        (*io).value.object = &mut (*(ret as *mut Object));
        (*io).set_collectable(true);
        (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
        preinit_thread(ret, global);
        ::core::ptr::write_volatile(
            &mut (*ret).hook_mask as *mut i32,
            (*interpreter).hook_mask,
        );
        (*ret).base_hook_count = (*interpreter).base_hook_count;
        ::core::ptr::write_volatile(&mut (*ret).hook as *mut HookFunction, (*interpreter).hook);
        (*ret).hook_count = (*ret).base_hook_count;
        stack_init(ret, interpreter);
        return ret;
    }
}
pub unsafe extern "C" fn luae_resetthread(interpreter: *mut Interpreter, mut status: i32) -> i32 {
    unsafe {
        (*interpreter).call_info = &mut (*interpreter).base_callinfo;
        let call_info: *mut CallInfo = (*interpreter).call_info;
        (*(*interpreter).stack.stkidrel_pointer).tvalue.set_tag_variant(TagVariant::NilNil as u8);
        (*call_info).function.stkidrel_pointer = (*interpreter).stack.stkidrel_pointer;
        (*call_info).call_status = (1 << 1) as u16;
        if status == 1 {
            status = 0;
        }
        (*interpreter).status = 0;
        (*interpreter).error_function = 0;
        status = luad_closeprotected(interpreter, 1 as i64, status);
        if status != 0 {
            (*interpreter).set_error_object(status, ((*interpreter).stack.stkidrel_pointer).offset(1 as isize));
        } else {
            (*interpreter).top.stkidrel_pointer = ((*interpreter).stack.stkidrel_pointer).offset(1 as isize);
        }
        (*call_info).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(20 as isize);
        luad_reallocstack(
            interpreter,
            ((*call_info).top.stkidrel_pointer).offset_from((*interpreter).stack.stkidrel_pointer) as i32,
            false,
        );
        return status;
    }
}
pub unsafe extern "C" fn lua_closethread(interpreter: *mut Interpreter, from: *mut Interpreter) -> i32 {
    unsafe {
        let status: i32;
        (*interpreter).count_c_calls = if !from.is_null() {
            (*from).count_c_calls & 0xffff as u32
        } else {
            0
        };
        status = luae_resetthread(interpreter, (*interpreter).status as i32);
        return status;
    }
}
pub unsafe extern "C" fn lua_close(mut interpreter: *mut Interpreter) {
    unsafe {
        interpreter = (*(*interpreter).global).main_state;
        close_state(interpreter);
    }
}
pub unsafe extern "C" fn luae_warning(interpreter: *mut Interpreter, message: *const i8, tocont: i32) {
    unsafe {
        let warn_function: WarnFunction = (*(*interpreter).global).warn_function;
        if warn_function.is_some() {
            warn_function.expect("non-null function pointer")((*(*interpreter).global).warn_userdata, message, tocont);
        }
    }
}
pub unsafe extern "C" fn luae_warnerror(interpreter: *mut Interpreter, where_0: *const i8) {
    unsafe {
        let errobj: *mut TValue = &mut (*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))).tvalue;
        let message: *const i8 = if (*errobj).is_tagtype_string() {
            ((*((*errobj).value.object as *mut TString)).get_contents_mut()) as *const i8
        } else {
            b"error object is not a string\0" as *const u8 as *const i8
        };
        luae_warning(interpreter, b"error in \0" as *const u8 as *const i8, 1);
        luae_warning(interpreter, where_0, 1);
        luae_warning(interpreter, b" (\0" as *const u8 as *const i8, 1);
        luae_warning(interpreter, message, 1);
        luae_warning(interpreter, b")\0" as *const u8 as *const i8, 0);
    }
}
pub unsafe extern "C" fn lua_sethook(
    interpreter: *mut Interpreter,
    mut function: HookFunction,
    mut mask: i32,
    count: i32,
) {
    unsafe {
        if function.is_none() || mask == 0 {
            mask = 0;
            function = None;
        }
        ::core::ptr::write_volatile(&mut (*interpreter).hook as *mut HookFunction, function);
        (*interpreter).base_hook_count = count;
        (*interpreter).hook_count = (*interpreter).base_hook_count;
        ::core::ptr::write_volatile(&mut (*interpreter).hook_mask as *mut i32, mask as u8 as i32);
        if mask != 0 {
            settraps((*interpreter).call_info);
        }
    }
}
pub unsafe extern "C" fn lua_gethook(interpreter: *mut Interpreter) -> HookFunction {
    unsafe {
        return (*interpreter).hook;
    }
}
pub unsafe extern "C" fn lua_gethookmask(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        return (*interpreter).hook_mask;
    }
}
pub unsafe extern "C" fn lua_gethookcount(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        return (*interpreter).base_hook_count;
    }
}
pub unsafe extern "C" fn lua_getstack(interpreter: *mut Interpreter, mut level: i32, ar: *mut DebugInfo) -> i32 {
    unsafe {
        let status: i32;
        let mut call_info;
        if level < 0 {
            return 0;
        }
        call_info = (*interpreter).call_info;
        while level > 0 && call_info != &mut (*interpreter).base_callinfo as *mut CallInfo {
            level -= 1;
            call_info = (*call_info).previous;
        }
        if level == 0 && call_info != &mut (*interpreter).base_callinfo as *mut CallInfo {
            status = 1;
            (*ar).i_ci = call_info;
        } else {
            status = 0;
        }
        return status;
    }
}
pub unsafe extern "C" fn formatvarinfo(
    interpreter: *mut Interpreter,
    kind: *const i8,
    name: *const i8,
) -> *const i8 {
    unsafe {
        if kind.is_null() {
            return b"\0" as *const u8 as *const i8;
        } else {
            return luao_pushfstring(interpreter, b" (%s '%s')\0" as *const u8 as *const i8, kind, name);
        };
    }
}
pub unsafe extern "C" fn varinfo(interpreter: *mut Interpreter, o: *const TValue) -> *const i8 {
    unsafe {
        let call_info: *mut CallInfo = (*interpreter).call_info;
        let mut name: *const i8 = std::ptr::null();
        let mut kind: *const i8 = std::ptr::null();
        if (*call_info).call_status as i32 & 1 << 1 == 0 {
            kind = getupvalname(call_info, o, &mut name);
            if kind.is_null() {
                let reg: i32 = in_stack(call_info, o);
                if reg >= 0 {
                    kind = getobjname(
                        (*((*(*call_info).function.stkidrel_pointer).tvalue.value.object as *mut Closure))
                            .payload.l_prototype,
                        currentpc(call_info),
                        reg,
                        &mut name,
                    );
                }
            }
        }
        return formatvarinfo(interpreter, kind, name);
    }
}
pub unsafe extern "C" fn typeerror(
    interpreter: *mut Interpreter,
    o: *const TValue,
    op: *const i8,
    extra: *const i8,
) -> ! {
    unsafe {
        let t: *const i8 = luat_objtypename(interpreter, o);
        luag_runerror(
            interpreter,
            b"attempt to %s a %s value%s\0" as *const u8 as *const i8,
            op,
            t,
            extra,
        );
    }
}
pub unsafe extern "C" fn luag_typeerror(interpreter: *mut Interpreter, o: *const TValue, op: *const i8) -> ! {
    unsafe {
        typeerror(interpreter, o, op, varinfo(interpreter, o));
    }
}
pub unsafe extern "C" fn luag_callerror(interpreter: *mut Interpreter, o: *const TValue) -> ! {
    unsafe {
        let call_info: *mut CallInfo = (*interpreter).call_info;
        let mut name: *const i8 = std::ptr::null();
        let kind: *const i8 = funcnamefromcall(interpreter, call_info, &mut name);
        let extra: *const i8 = if !kind.is_null() {
            formatvarinfo(interpreter, kind, name)
        } else {
            varinfo(interpreter, o)
        };
        typeerror(interpreter, o, b"call\0" as *const u8 as *const i8, extra);
    }
}
pub unsafe extern "C" fn luag_forerror(interpreter: *mut Interpreter, o: *const TValue, what: *const i8) -> ! {
    unsafe {
        luag_runerror(
            interpreter,
            b"bad 'for' %s (number expected, got %s)\0" as *const u8 as *const i8,
            what,
            luat_objtypename(interpreter, o),
        );
    }
}
pub unsafe extern "C" fn luag_concaterror(
    interpreter: *mut Interpreter,
    mut p1: *const TValue,
    p2: *const TValue,
) -> ! {
    unsafe {
        match (*p1).get_tag_type() {
            TagType::String | TagType::Numeric => {
                p1 = p2;
            }
            _ => {},
        }
        luag_typeerror(interpreter, p1, b"concatenate\0" as *const u8 as *const i8);
    }
}
pub unsafe extern "C" fn luag_opinterror(
    interpreter: *mut Interpreter,
    p1: *const TValue,
    mut p2: *const TValue,
    message: *const i8,
) -> ! {
    unsafe {
        if !(*p1).is_tagtype_numeric() {
            p2 = p1;
        }
        luag_typeerror(interpreter, p2, message);
    }
}
pub unsafe extern "C" fn luag_tointerror(
    interpreter: *mut Interpreter,
    p1: *const TValue,
    mut p2: *const TValue,
) -> ! {
    unsafe {
        let mut temp: i64 = 0;
        if luav_tointegerns(p1, &mut temp, F2I::Equal) == 0 {
            p2 = p1;
        }
        luag_runerror(
            interpreter,
            b"number%s has no integer representation\0" as *const u8 as *const i8,
            varinfo(interpreter, p2),
        );
    }
}
pub unsafe extern "C" fn luag_ordererror(
    interpreter: *mut Interpreter,
    p1: *const TValue,
    p2: *const TValue,
) -> ! {
    unsafe {
        let t1: *const i8 = luat_objtypename(interpreter, p1);
        let t2: *const i8 = luat_objtypename(interpreter, p2);
        if strcmp(t1, t2) == 0 {
            luag_runerror(
                interpreter,
                b"attempt to compare two %s values\0" as *const u8 as *const i8,
                t1,
            );
        } else {
            luag_runerror(
                interpreter,
                b"attempt to compare %s with %s\0" as *const u8 as *const i8,
                t1,
                t2,
            );
        };
    }
}
pub unsafe extern "C" fn luag_addinfo(
    interpreter: *mut Interpreter,
    message: *const i8,
    src: *mut TString,
    line: i32,
) -> *const i8 {
    unsafe {
        let mut buffer: [i8; 60] = [0; 60];
        if !src.is_null() {
            luao_chunkid(
                buffer.as_mut_ptr(),
                (*src).get_contents_mut(),
                (*src).get_length(),
            );
        } else {
            buffer[0] = CHARACTER_QUESTION as i8;
            buffer[1] = Character::Null as i8;
        }
        return luao_pushfstring(
            interpreter,
            b"%s:%d: %s\0" as *const u8 as *const i8,
            buffer.as_mut_ptr(),
            line,
            message,
        );
    }
}
pub unsafe extern "C" fn luag_errormsg(interpreter: *mut Interpreter) -> ! {
    unsafe {
        if (*interpreter).error_function != 0 {
            let error_function: StackValuePointer =
                ((*interpreter).stack.stkidrel_pointer as *mut i8).offset((*interpreter).error_function as isize) as StackValuePointer;
            let io1: *mut TValue = &mut (*(*interpreter).top.stkidrel_pointer).tvalue;
            let io2: *const TValue = &mut (*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))).tvalue;
            (*io1).copy_from(&*io2);
            let io1_0: *mut TValue = &mut (*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))).tvalue;
            let io2_0: *const TValue = &mut (*error_function).tvalue;
            (*io1_0).copy_from(&*io2_0);
            (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
            luad_callnoyield(interpreter, (*interpreter).top.stkidrel_pointer.offset(-(2 as isize)), 1);
        }
        luad_throw(interpreter, 2);
    }
}
pub unsafe extern "C" fn luag_runerror(interpreter: *mut Interpreter, fmt: *const i8, args: ...) -> ! {
    unsafe {
        let call_info: *mut CallInfo = (*interpreter).call_info;
        let message: *const i8;
        let mut argp: ::core::ffi::VaListImpl;
        if (*(*interpreter).global).gc_debt > 0 {
            luac_step(interpreter);
        }
        argp = args.clone();
        message = luao_pushvfstring(interpreter, fmt, argp.as_va_list());
        if (*call_info).call_status as i32 & 1 << 1 == 0 {
            luag_addinfo(
                interpreter,
                message,
                (*(*((*(*call_info).function.stkidrel_pointer).tvalue.value.object as *mut Closure))
                    .payload.l_prototype)
                    .prototype_source,
                getcurrentline(call_info),
            );
            let io1: *mut TValue = &mut (*(*interpreter).top.stkidrel_pointer.offset(-(2 as isize))).tvalue;
            let io2: *const TValue = &mut (*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))).tvalue;
            (*io1).copy_from(&*io2);
            (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(-1);
        }
        luag_errormsg(interpreter);
    }
}
pub unsafe extern "C" fn luag_tracecall(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let call_info: *mut CallInfo = (*interpreter).call_info;
        let p: *mut Prototype = (*((*(*call_info).function.stkidrel_pointer).tvalue.value.object as *mut Closure))
            .payload.l_prototype;
        ::core::ptr::write_volatile(&mut (*call_info).u.l.trap as *mut i32, 1);
        if (*call_info).u.l.saved_program_counter == (*p).prototype_code.pointer as *const u32 {
            if (*p).prototype_is_variable_arguments {
                return 0;
            } else if (*call_info).call_status as i32 & 1 << 6 == 0 {
                luad_hookcall(interpreter, call_info);
            }
        }
        return 1;
    }
}
pub unsafe extern "C" fn luag_traceexec(interpreter: *mut Interpreter, mut program_counter: *const u32) -> i32 {
    unsafe {
        let call_info: *mut CallInfo = (*interpreter).call_info;
        let mask: u8 = (*interpreter).hook_mask as u8;
        let p: *const Prototype = (*((*(*call_info).function.stkidrel_pointer).tvalue.value.object
            as *mut Closure))
            .payload.l_prototype;
        if mask as i32 & (1 << 2 | 1 << 3) == 0 {
            ::core::ptr::write_volatile(&mut (*call_info).u.l.trap as *mut i32, 0);
            return 0;
        }
        program_counter = program_counter.offset(1);
        (*call_info).u.l.saved_program_counter = program_counter;
        let counthook: i32 = (mask as i32 & 1 << 3 != 0 && {
            (*interpreter).hook_count -= 1;
            (*interpreter).hook_count == 0
        }) as i32;
        if counthook != 0 {
            (*interpreter).hook_count = (*interpreter).base_hook_count;
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
            && (*((*call_info).u.l.saved_program_counter).offset(-(1 as isize)) >> POSITION_B
                & !(!(0u32) << 8) << 0) as i32
                == 0)
        {
            (*interpreter).top.stkidrel_pointer = (*call_info).top.stkidrel_pointer;
        }
        if counthook != 0 {
            luad_hook(interpreter, 3, -1, 0, 0);
        }
        if mask as i32 & 1 << 2 != 0 {
            let old_program_counter: i32 = if (*interpreter).old_program_counter < (*p).prototype_code.size {
                (*interpreter).old_program_counter
            } else {
                0
            };
            let npci: i32 = program_counter.offset_from((*p).prototype_code.pointer) as i32 - 1;
            if npci <= old_program_counter || changedline(p, old_program_counter, npci) != 0 {
                let newline: i32 = luag_getfuncline(p, npci);
                luad_hook(interpreter, 2, newline, 0, 0);
            }
            (*interpreter).old_program_counter = npci;
        }
        if (*interpreter).status as i32 == 1 {
            if counthook != 0 {
                (*interpreter).hook_count = 1;
            }
            (*call_info).call_status = ((*call_info).call_status as i32 | 1 << 6) as u16;
            luad_throw(interpreter, 1);
        }
        return 1;
    }
}
pub unsafe extern "C" fn luam_growaux_(
    interpreter: *mut Interpreter,
    block: *mut libc::c_void,
    count_elements: usize,
    total_size: *mut i32,
    element_size: usize,
    limit: i32,
    what: *const i8,
) -> *mut libc::c_void {
    unsafe {
        let mut size: i32 = *total_size;
        if count_elements + 1 <= size as usize {
            return block;
        }
        if size >= limit / 2 {
            if ((size >= limit) as i32 != 0) as i64 != 0 {
                luag_runerror(
                    interpreter,
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
            interpreter,
            block,
            (*total_size as usize).wrapping_mul(element_size),
            (size as usize).wrapping_mul(element_size),
        );
        *total_size = size;
        return new_block;
    }
}
pub unsafe extern "C" fn tryagain(
    interpreter: *mut Interpreter,
    block: *mut libc::c_void,
    old_size: usize,
    new_size: usize,
) -> *mut libc::c_void {
    unsafe {
        let global: *mut Global = (*interpreter).global;
        if (*global).none_value.is_tagtype_nil() && (*global).gcstopem == 0 {
            luac_fullgc(interpreter, true);
            return raw_allocate(block, old_size, new_size);
        } else {
            return std::ptr::null_mut();
        };
    }
}
pub unsafe extern "C" fn luam_realloc_(
    interpreter: *mut Interpreter,
    block: *mut libc::c_void,
    old_size: usize,
    new_size: usize,
) -> *mut libc::c_void {
    unsafe {
        let global: *mut Global = (*interpreter).global;
        let mut new_block: *mut libc::c_void = raw_allocate(block, old_size, new_size);
        if new_block.is_null() && new_size > 0 {
            new_block = tryagain(interpreter, block, old_size, new_size);
            if new_block.is_null() {
                return std::ptr::null_mut();
            }
        }
        (*global).gc_debt = ((*global).gc_debt as usize)
            .wrapping_add(new_size)
            .wrapping_sub(old_size) as i64;
        return new_block;
    }
}
pub unsafe extern "C" fn luam_saferealloc_(
    interpreter: *mut Interpreter,
    block: *mut libc::c_void,
    old_size: usize,
    new_size: usize,
) -> *mut libc::c_void {
    unsafe {
        let new_block: *mut libc::c_void = luam_realloc_(interpreter, block, old_size, new_size);
        if new_block.is_null() && new_size > 0 {
            luad_throw(interpreter, 4);
        }
        return new_block;
    }
}
pub unsafe extern "C" fn luam_malloc_(interpreter: *mut Interpreter, new_size: usize) -> *mut libc::c_void {
    unsafe {
        if new_size == 0 {
            return std::ptr::null_mut();
        } else {
            let global: *mut Global = (*interpreter).global;
            let mut new_block: *mut libc::c_void = raw_allocate(std::ptr::null_mut(), 0, new_size);
            if new_block.is_null() {
                new_block = tryagain(interpreter, std::ptr::null_mut(), 0, new_size);
                if new_block.is_null() {
                    luad_throw(interpreter, 4);
                }
            }
            (*global).gc_debt = ((*global).gc_debt as usize).wrapping_add(new_size) as i64;
            return new_block;
        };
    }
}
pub unsafe extern "C" fn intarith(interpreter: *mut Interpreter, op: i32, v1: i64, v2: i64) -> i64 {
    unsafe {
        match op {
            0 => return (v1 as u64).wrapping_add(v2 as u64) as i64,
            1 => return (v1 as u64).wrapping_sub(v2 as u64) as i64,
            2 => return (v1 as u64).wrapping_mul(v2 as u64) as i64,
            3 => return luav_mod(interpreter, v1, v2),
            6 => return luav_idiv(interpreter, v1, v2),
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
pub unsafe extern "C" fn numarith(interpreter: *mut Interpreter, op: i32, v1: f64, v2: f64) -> f64 {
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
            3 => return luav_modf(interpreter, v1, v2),
            _ => return 0.0,
        };
    }
}
pub unsafe extern "C" fn luao_rawarith(
    interpreter: *mut Interpreter,
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
                if (if (((*p1).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER) as i32 != 0) as i64
                    != 0
                {
                    i1 = (*p1).value.integer;
                    1
                } else {
                    luav_tointegerns(p1, &mut i1, F2I::Equal)
                }) != 0
                    && (if (((*p2).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER) as i32 != 0) as i32
                        as i64
                        != 0
                    {
                        i2 = (*p2).value.integer;
                        1
                    } else {
                        luav_tointegerns(p2, &mut i2, F2I::Equal)
                    }) != 0
                {
                    (*res).value.integer = intarith(interpreter, op, i1, i2);
                    (*res).set_tag_variant(TAG_VARIANT_NUMERIC_INTEGER);
                    return 1;
                } else {
                    return 0;
                }
            }
            5 | 4 => {
                let mut n1: f64 = 0.0;
                let mut n2: f64 = 0.0;
                if (if (*p1).get_tag_variant() == TAG_VARIANT_NUMERIC_NUMBER {
                    n1 = (*p1).value.number;
                    1
                } else {
                    if (*p1).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER {
                        n1 = (*p1).value.integer as f64;
                        1
                    } else {
                        0
                    }
                }) != 0
                    && (if (*p2).get_tag_variant() == TAG_VARIANT_NUMERIC_NUMBER {
                        n2 = (*p2).value.number;
                        1
                    } else {
                        if (*p2).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER {
                            n2 = (*p2).value.integer as f64;
                            1
                        } else {
                            0
                        }
                    }) != 0
                {
                    (*res).value.number = numarith(interpreter, op, n1, n2);
                    (*res).set_tag_variant(TAG_VARIANT_NUMERIC_NUMBER);
                    return 1;
                } else {
                    return 0;
                }
            }
            _ => {
                let mut n1_0: f64 = 0.0;
                let mut n2_0: f64 = 0.0;
                if (*p1).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER
                    && (*p2).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER
                {
                    let io_1: *mut TValue = res;
                    (*io_1).value.integer = intarith(interpreter, op, (*p1).value.integer, (*p2).value.integer);
                    (*io_1).set_tag_variant(TAG_VARIANT_NUMERIC_INTEGER);
                    return 1;
                } else if (if (*p1).get_tag_variant() == TAG_VARIANT_NUMERIC_NUMBER {
                    n1_0 = (*p1).value.number;
                    1
                } else {
                    if (*p1).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER {
                        n1_0 = (*p1).value.integer as f64;
                        1
                    } else {
                        0
                    }
                }) != 0
                    && (if (*p2).get_tag_variant() == TAG_VARIANT_NUMERIC_NUMBER {
                        n2_0 = (*p2).value.number;
                        1
                    } else {
                        if (*p2).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER {
                            n2_0 = (*p2).value.integer as f64;
                            1
                        } else {
                            0
                        }
                    }) != 0
                {
                    let io_2: *mut TValue = res;
                    (*io_2).value.number = numarith(interpreter, op, n1_0, n2_0);
                    (*io_2).set_tag_variant(TAG_VARIANT_NUMERIC_NUMBER);
                    return 1;
                } else {
                    return 0;
                }
            }
        };
    }
}
pub unsafe extern "C" fn luao_arith(
    interpreter: *mut Interpreter,
    op: i32,
    p1: *const TValue,
    p2: *const TValue,
    res: StackValuePointer,
) {
    unsafe {
        if luao_rawarith(interpreter, op, p1, p2, &mut (*res).tvalue) == 0 {
            luat_trybintm(interpreter, p1, p2, res, (op - 0 + TM_ADD as i32) as u32);
        }
    }
}
pub unsafe extern "C" fn luao_pushvfstring(
    interpreter: *mut Interpreter,
    mut fmt: *const i8,
    mut argp: ::core::ffi::VaList,
) -> *const i8 {
    unsafe {
        let mut buff_fs = BuffFS::new(interpreter);
        let mut e: *const i8;
        loop {
            e = strchr(fmt, CHARACTER_PERCENT as i32);
            if e.is_null() {
                break;
            }
            buff_fs.add_string(fmt, e.offset_from(fmt) as u64);
            match *e.offset(1 as isize) as i32 {
                CHARACTER_LOWER_S => {
                    let mut s: *const i8 = argp.arg::<*mut i8>();
                    if s.is_null() {
                        s = b"(null)\0" as *const u8 as *const i8;
                    }
                    buff_fs.add_string(s, strlen(s));
                }
                CHARACTER_LOWER_C => {
                    let mut c: i8 = argp.arg::<i32>() as u8 as i8;
                    buff_fs.add_string(&mut c, ::core::mem::size_of::<i8>() as u64);
                }
                CHARACTER_LOWER_D => {
                    let mut tvalue: TValue = TValue::new(TAG_VARIANT_NIL_NIL);
                    tvalue.value.integer = argp.arg::<i32>() as i64;
                    tvalue.set_tag_variant(TAG_VARIANT_NUMERIC_INTEGER);
                    buff_fs.add_number(&mut tvalue);
                }
                CHARACTER_UPPER_I => {
                    let mut tvalue: TValue = TValue::new(TAG_VARIANT_NIL_NIL);
                    tvalue.value.integer = argp.arg::<i64>();
                    tvalue.set_tag_variant(TAG_VARIANT_NUMERIC_INTEGER);
                    buff_fs.add_number(&mut tvalue);
                }
                CHARACTER_LOWER_F => {
                    let mut tvalue: TValue = TValue::new(TAG_VARIANT_NIL_NIL);
                    tvalue.value.number = argp.arg::<f64>();
                    tvalue.set_tag_variant(TAG_VARIANT_NUMERIC_NUMBER);
                    buff_fs.add_number(&mut tvalue);
                }
                CHARACTER_LOWER_P => {
                    let size = (3 as usize)
                        .wrapping_mul(::core::mem::size_of::<*mut libc::c_void>())
                        .wrapping_add(8);
                    let bf: *mut i8 = buff_fs.get_raw(size);
                    let p: *mut libc::c_void = argp.arg::<*mut libc::c_void>();
                    let length =
                        snprintf(bf, size as u64, b"%p\0" as *const u8 as *const i8, p) as u64;
                    buff_fs.add_length(length as usize);
                }
                CHARACTER_UPPER_U => {
                    let mut bf_0: [i8; 8] = [0; 8];
                    let length_0: i32 = luao_utf8esc(bf_0.as_mut_ptr(), argp.arg::<i64>() as u64);
                    buff_fs.add_string(
                        bf_0.as_mut_ptr()
                            .offset(8 as isize)
                            .offset(-(length_0 as isize)),
                        length_0 as u64,
                    );
                }
                CHARACTER_PERCENT => {
                    buff_fs.add_string(b"%\0" as *const u8 as *const i8, 1 as u64);
                }
                _ => {
                    luag_runerror(
                        interpreter,
                        b"invalid option '%%%c' to 'lua_pushfstring'\0" as *const u8 as *const i8,
                        *e.offset(1 as isize) as i32,
                    );
                }
            }
            fmt = e.offset(2 as isize);
        }
        buff_fs.add_string(fmt, strlen(fmt));
        buff_fs.clear();
        return (*((*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))).tvalue.value.object as *mut TString))
            .get_contents_mut();
    }
}
pub unsafe extern "C" fn luao_pushfstring(
    interpreter: *mut Interpreter,
    fmt: *const i8,
    args: ...
) -> *const i8 {
    unsafe {
        let message: *const i8;
        let mut argp: ::core::ffi::VaListImpl;
        argp = args.clone();
        message = luao_pushvfstring(interpreter, fmt, argp.as_va_list());
        return message;
    }
}
pub unsafe extern "C" fn luat_init(interpreter: *mut Interpreter) {
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
        for i in 0..TM_N {
            (*(*interpreter).global).tm_name[i as usize] = luas_new(interpreter, EVENT_NAMES[i as usize]);
            fix_object_state(
                interpreter,
                &mut (*(*((*(*interpreter).global).tm_name).as_mut_ptr().offset(i as isize)
                    as *mut Object))
            );
        }
    }
}
pub unsafe extern "C" fn luat_gettmbyobj(
    interpreter: *mut Interpreter,
    o: *const TValue,
    event: u32,
) -> *const TValue {
    unsafe {
        let metatable: *mut Table;
        match (*o).get_tag_type() {
            TagType::Table => {
                metatable = (*((*o).value.object as *mut Table)).metatable;
            }
            TagType::User => {
                metatable = (*((*o).value.object as *mut User)).metatable;
            }
            _ => {
                metatable = (*(*interpreter).global).metatables[(*o).get_tag_type() as usize];
            }
        }
        return if metatable.is_null() {
            &mut (*(*interpreter).global).none_value as *mut TValue as *const TValue
        } else {
            luah_getshortstr(metatable, (*(*interpreter).global).tm_name[event as usize])
        };
    }
}
pub unsafe extern "C" fn luat_objtypename(interpreter: *mut Interpreter, o: *const TValue) -> *const i8 {
    unsafe {
        let mut metatable: *mut Table;
        if (*o).get_tag_variant() == TAG_VARIANT_TABLE && {
            metatable = (*((*o).value.object as *mut Table)).get_metatable();
            !metatable.is_null()
        } || (*o).get_tag_variant() == TAG_VARIANT_USER && {
            metatable = (*((*o).value.object as *mut User)).get_metatable();
            !metatable.is_null()
        } {
            let name: *const TValue =
                luah_getshortstr(metatable, luas_new(interpreter, b"__name\0" as *const u8 as *const i8));
            if (*name).is_tagtype_string() {
                return (*((*name).value.object as *mut TString)).get_contents_mut();
            }
        }
        return TYPE_NAMES[((*o).get_tag_type() as usize + 1) as usize];
    }
}
pub unsafe extern "C" fn luat_calltm(
    interpreter: *mut Interpreter,
    f: *const TValue,
    p1: *const TValue,
    p2: *const TValue,
    p3: *const TValue,
) {
    unsafe {
        let function: StackValuePointer = (*interpreter).top.stkidrel_pointer;
        let io1: *mut TValue = &mut (*function).tvalue;
        (*io1).copy_from(&*f);
        let io1_0: *mut TValue = &mut (*function.offset(1 as isize)).tvalue;
        let io2_0: *const TValue = p1;
        (*io1_0).copy_from(&*io2_0);
        let io1_1: *mut TValue = &mut (*function.offset(2 as isize)).tvalue;
        let io2_1: *const TValue = p2;
        (*io1_1).copy_from(&*io2_1);
        let io1_2: *mut TValue = &mut (*function.offset(3 as isize)).tvalue;
        let io2_2: *const TValue = p3;
        (*io1_2).copy_from(&*io2_2);
        (*interpreter).top.stkidrel_pointer = function.offset(4 as isize);
        if (*(*interpreter).call_info).call_status as i32 & (1 << 1 | 1 << 3) == 0 {
            ccall(interpreter, function, 0, 1);
        } else {
            luad_callnoyield(interpreter, function, 0);
        };
    }
}
pub unsafe extern "C" fn luat_calltmres(
    interpreter: *mut Interpreter,
    f: *const TValue,
    p1: *const TValue,
    p2: *const TValue,
    mut res: StackValuePointer,
) {
    unsafe {
        let result: i64 = (res as *mut i8).offset_from((*interpreter).stack.stkidrel_pointer as *mut i8) as i64;
        let function: StackValuePointer = (*interpreter).top.stkidrel_pointer;
        let io1: *mut TValue = &mut (*function).tvalue;
        let io2: *const TValue = f;
        (*io1).copy_from(&*io2);
        let io1_0: *mut TValue = &mut (*function.offset(1 as isize)).tvalue;
        let io2_0: *const TValue = p1;
        (*io1_0).copy_from(&(*io2_0));
        let io1_1: *mut TValue = &mut (*function.offset(2 as isize)).tvalue;
        let io2_1: *const TValue = p2;
        (*io1_1).copy_from(&(*io2_1));
        (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(3 as isize);
        if (*(*interpreter).call_info).call_status as i32 & (1 << 1 | 1 << 3) == 0 {
            ccall(interpreter, function, 1, 1);
        } else {
            luad_callnoyield(interpreter, function, 1);
        }
        res = ((*interpreter).stack.stkidrel_pointer as *mut i8).offset(result as isize) as StackValuePointer;
        let io1_2: *mut TValue = &mut (*res).tvalue;
        (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(-1);
        let io2_2: *const TValue = &mut (*(*interpreter).top.stkidrel_pointer).tvalue;
        (*io1_2).copy_from(&(*io2_2));
    }
}
pub unsafe extern "C" fn callbintm(
    interpreter: *mut Interpreter,
    p1: *const TValue,
    p2: *const TValue,
    res: StackValuePointer,
    event: u32,
) -> i32 {
    unsafe {
        let mut tm: *const TValue = luat_gettmbyobj(interpreter, p1, event);
        if (*tm).is_tagtype_nil() {
            tm = luat_gettmbyobj(interpreter, p2, event);
        }
        if (*tm).is_tagtype_nil() {
            return 0;
        }
        luat_calltmres(interpreter, tm, p1, p2, res);
        return 1;
    }
}
pub unsafe extern "C" fn luat_trybintm(
    interpreter: *mut Interpreter,
    p1: *const TValue,
    p2: *const TValue,
    res: StackValuePointer,
    event: u32,
) {
    unsafe {
        if ((callbintm(interpreter, p1, p2, res, event) == 0) as i32 != 0) as i64 != 0 {
            match event as u32 {
                TM_BAND | TM_BOR | TM_BXOR | TM_SHL | TM_SHR | TM_BNOT => {
                    if (*p1).is_tagtype_numeric() && (*p2).is_tagtype_numeric() {
                        luag_tointerror(interpreter, p1, p2);
                    } else {
                        luag_opinterror(
                            interpreter,
                            p1,
                            p2,
                            b"perform bitwise operation on\0" as *const u8 as *const i8,
                        );
                    }
                }
                _ => {
                    luag_opinterror(
                        interpreter,
                        p1,
                        p2,
                        b"perform arithmetic on\0" as *const u8 as *const i8,
                    );
                }
            }
        }
    }
}
pub unsafe extern "C" fn luat_tryconcattm(interpreter: *mut Interpreter) {
    unsafe {
        let top: StackValuePointer = (*interpreter).top.stkidrel_pointer;
        if ((callbintm(
            interpreter,
            &mut (*top.offset(-(2 as isize))).tvalue,
            &mut (*top.offset(-(1 as isize))).tvalue,
            top.offset(-(2 as isize)),
            TM_CONCAT,
        ) == 0) as i32
            != 0) as i64
            != 0
        {
            luag_concaterror(
                interpreter,
                &mut (*top.offset(-(2 as isize))).tvalue,
                &mut (*top.offset(-(1 as isize))).tvalue,
            );
        }
    }
}
pub unsafe extern "C" fn luat_trybinassoctm(
    interpreter: *mut Interpreter,
    p1: *const TValue,
    p2: *const TValue,
    flip: i32,
    res: StackValuePointer,
    event: u32,
) {
    unsafe {
        if flip != 0 {
            luat_trybintm(interpreter, p2, p1, res, event);
        } else {
            luat_trybintm(interpreter, p1, p2, res, event);
        };
    }
}
pub unsafe extern "C" fn luat_trybinitm(
    interpreter: *mut Interpreter,
    p1: *const TValue,
    i2: i64,
    flip: i32,
    res: StackValuePointer,
    event: u32,
) {
    unsafe {
        let mut aux: TValue = TValue::new(TAG_VARIANT_NIL_NIL);
        let io: *mut TValue = &mut aux;
        (*io).value.integer = i2;
        (*io).set_tag_variant(TAG_VARIANT_NUMERIC_INTEGER);
        luat_trybinassoctm(interpreter, p1, &mut aux, flip, res, event);
    }
}
pub unsafe extern "C" fn luat_callordertm(
    interpreter: *mut Interpreter,
    p1: *const TValue,
    p2: *const TValue,
    event: u32,
) -> i32 {
    unsafe {
        if callbintm(interpreter, p1, p2, (*interpreter).top.stkidrel_pointer, event) != 0 {
            return !((*(*interpreter).top.stkidrel_pointer).tvalue.get_tag_variant() == TAG_VARIANT_BOOLEAN_FALSE
                || (*(*interpreter).top.stkidrel_pointer).tvalue.is_tagtype_nil())
                as i32;
        }
        luag_ordererror(interpreter, p1, p2);
    }
}
pub unsafe extern "C" fn luat_callorderitm(
    interpreter: *mut Interpreter,
    mut p1: *const TValue,
    v2: i32,
    flip: i32,
    is_float: bool,
    event: u32,
) -> i32 {
    unsafe {
        let mut aux: TValue = TValue::new(TAG_VARIANT_NIL_NIL);
        let p2: *const TValue;
        if is_float {
            let io: *mut TValue = &mut aux;
            (*io).value.number = v2 as f64;
            (*io).set_tag_variant(TAG_VARIANT_NUMERIC_NUMBER);
        } else {
            let io_0: *mut TValue = &mut aux;
            (*io_0).value.integer = v2 as i64;
            (*io_0).set_tag_variant(TAG_VARIANT_NUMERIC_INTEGER);
        }
        if flip != 0 {
            p2 = p1;
            p1 = &mut aux;
        } else {
            p2 = &mut aux;
        }
        return luat_callordertm(interpreter, p1, p2, event);
    }
}
pub unsafe extern "C" fn luat_adjustvarargs(
    interpreter: *mut Interpreter,
    nfixparams: i32,
    call_info: *mut CallInfo,
    p: *const Prototype,
) {
    unsafe {
        let actual: i32 = ((*interpreter).top.stkidrel_pointer).offset_from((*call_info).function.stkidrel_pointer) as i32 - 1;
        let nextra: i32 = actual - nfixparams;
        (*call_info).u.l.count_extra_arguments = nextra;
        if ((((*interpreter).stack_last.stkidrel_pointer).offset_from((*interpreter).top.stkidrel_pointer) as i64
            <= ((*p).prototype_maximum_stack_size as i32 + 1) as i64) as i32
            != 0) as i64
            != 0
        {
            luad_growstack(interpreter, (*p).prototype_maximum_stack_size as i32 + 1, true);
        }
        let fresh12 = (*interpreter).top.stkidrel_pointer;
        (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
        let io1: *mut TValue = &mut (*fresh12).tvalue;
        let io2: *const TValue = &mut (*(*call_info).function.stkidrel_pointer).tvalue;
        (*io1).copy_from(&*io2);
        for i in 1..(1 + nfixparams) {
            let fresh13 = (*interpreter).top.stkidrel_pointer;
            (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
            let io1_0: *mut TValue = &mut (*fresh13).tvalue;
            let io2_0: *const TValue = &mut (*((*call_info).function.stkidrel_pointer).offset(i as isize)).tvalue;
            (*io1_0).copy_from(&*io2_0);
            (*((*call_info).function.stkidrel_pointer).offset(i as isize))
                .tvalue
                .set_tag_variant(TagVariant::NilNil as u8);
        }
        (*call_info).function.stkidrel_pointer = ((*call_info).function.stkidrel_pointer).offset((actual + 1) as isize);
        (*call_info).top.stkidrel_pointer = ((*call_info).top.stkidrel_pointer).offset((actual + 1) as isize);
    }
}
pub unsafe extern "C" fn luat_getvarargs(
    interpreter: *mut Interpreter,
    call_info: *mut CallInfo,
    mut where_0: StackValuePointer,
    mut wanted: i32,
) {
    unsafe {
        let nextra: i32 = (*call_info).u.l.count_extra_arguments;
        if wanted < 0 {
            wanted = nextra;
            if ((((*interpreter).stack_last.stkidrel_pointer).offset_from((*interpreter).top.stkidrel_pointer) as i64 <= nextra as i64)
                as i32
                != 0) as i64
                != 0
            {
                let t__: i64 = (where_0 as *mut i8).offset_from((*interpreter).stack.stkidrel_pointer as *mut i8) as i64;
                if (*(*interpreter).global).gc_debt > 0 {
                    luac_step(interpreter);
                }
                luad_growstack(interpreter, nextra, true);
                where_0 = ((*interpreter).stack.stkidrel_pointer as *mut i8).offset(t__ as isize) as StackValuePointer;
            }
            (*interpreter).top.stkidrel_pointer = where_0.offset(nextra as isize);
        }
        for i in 0..wanted.min(nextra) {
            let io1: *mut TValue = &mut (*where_0.offset(i as isize)).tvalue;
            let io2: *const TValue = &mut (*((*call_info).function.stkidrel_pointer)
                .offset(-(nextra as isize))
                .offset(i as isize))
            .tvalue;
            (*io1).copy_from(&*io2);
        }
        for i in wanted.min(nextra)..wanted {
            (*where_0.offset(i as isize))
                .tvalue
                .set_tag_variant(TagVariant::NilNil as u8);
        }
    }
}
pub unsafe extern "C" fn luac_newobj(interpreter: *mut Interpreter, tag_variant: u8, size: usize) -> *mut Object {
    unsafe {
        let global: *mut Global = (*interpreter).global;
        let ret = luam_malloc_(interpreter, size as usize)  as *mut Object;
        (*ret).set_tag_variant(tag_variant);
        (*ret).set_marked((*global).current_white & (1 << 3 | 1 << 4));
        (*ret).next = (*global).all_gc;
        (*global).all_gc = ret;
        return ret;
    }
}
pub unsafe extern "C" fn traverse_state(global: *mut Global, interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let mut o: StackValuePointer = (*interpreter).stack.stkidrel_pointer;
        if (*interpreter).get_marked() & 7 > 1 || (*global).gc_state as i32 == 0 {
            linkgclist_(
                &mut (*(interpreter as *mut Object)),
                &mut (*interpreter).gc_list,
                &mut (*global).gray_again,
            );
        }
        if o.is_null() {
            return 1;
        }
        while o < (*interpreter).top.stkidrel_pointer {
            if ((*o).tvalue.is_collectable())
                && (*(*o).tvalue.value.object).get_marked() & (1 << 3 | 1 << 4) != 0
            {
                really_mark_object(global, (*o).tvalue.value.object);
            }
            o = o.offset(1);
        }
        let mut uv: *mut UpValue = (*interpreter).open_upvalue;
        while !uv.is_null() {
            if (*uv).get_marked() & (1 << 3 | 1 << 4) != 0 {
                really_mark_object(global, &mut (*(uv as *mut Object)));
            }
            uv = (*uv).u.open.next;
        }
        if (*global).gc_state as i32 == 2 {
            if !(*global).is_emergency {
                (*interpreter).luad_shrinkstack();
            }
            o = (*interpreter).top.stkidrel_pointer;
            while o < ((*interpreter).stack_last.stkidrel_pointer).offset(5 as isize) {
                (*o).tvalue.set_tag_variant(TagVariant::NilNil as u8);
                o = o.offset(1);
            }
            if !((*interpreter).twups != interpreter) && !((*interpreter).open_upvalue).is_null() {
                (*interpreter).twups = (*global).twups;
                (*global).twups = interpreter;
            }
        }
        return 1 + ((*interpreter).stack_last.stkidrel_pointer).offset_from((*interpreter).stack.stkidrel_pointer) as i32;
    }
}
pub unsafe extern "C" fn sweeptolive(
    interpreter: *mut Interpreter,
    mut p: *mut *mut Object,
) -> *mut *mut Object {
    unsafe {
        let old: *mut *mut Object = p;
        loop {
            p = (*interpreter).sweep_list(p, 1, std::ptr::null_mut());
            if !(p == old) {
                break;
            }
        }
        return p;
    }
}
pub unsafe extern "C" fn check_sizes(interpreter: *mut Interpreter, global: *mut Global) {
    unsafe {
        if !(*global).is_emergency {
            if (*global).string_table.length < (*global).string_table.size / 4 {
                let olddebt: i64 = (*global).gc_debt;
                luas_resize(interpreter, ((*global).string_table.size / 2) as usize);
                (*global).gc_estimate = ((*global).gc_estimate as u64)
                    .wrapping_add(((*global).gc_debt - olddebt) as u64)
                    as u64;
            }
        }
    }
}
pub unsafe extern "C" fn dothecall(interpreter: *mut Interpreter, mut _ud: *mut libc::c_void) {
    unsafe {
        luad_callnoyield(interpreter, (*interpreter).top.stkidrel_pointer.offset(-(2 as isize)), 0);
    }
}
pub unsafe extern "C" fn gctm_function(interpreter: *mut Interpreter) {
    unsafe {
        let global: *mut Global = (*interpreter).global;
        let tm: *const TValue;
        let mut v: TValue = TValue::new(TAG_VARIANT_NIL_NIL);
        let io: *mut TValue = &mut v;
        let i_g: *mut Object = udata2finalize(global);
        (*io).value.object = i_g;
        (*io).set_tag_variant((*i_g).get_tag_variant());
        (*io).set_collectable(true);
        tm = luat_gettmbyobj(interpreter, &mut v, TM_GC);
        if !(*tm).is_tagtype_nil() {
            let status: i32;
            let oldah: u8 = (*interpreter).allow_hook;
            let oldgcstp: i32 = (*global).gc_step as i32;
            (*global).gc_step = ((*global).gc_step as i32 | 2) as u8;
            (*interpreter).allow_hook = 0;
            let fresh15 = (*interpreter).top.stkidrel_pointer;
            (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
            let io1: *mut TValue = &mut (*fresh15).tvalue;
            let io2: *const TValue = tm;
            (*io1).copy_from(&*io2);
            let fresh16 = (*interpreter).top.stkidrel_pointer;
            (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
            let io1_0: *mut TValue = &mut (*fresh16).tvalue;
            let io2_0: *const TValue = &mut v;
            (*io1_0).copy_from(&(*io2_0));
            (*(*interpreter).call_info).call_status =
                ((*(*interpreter).call_info).call_status as i32 | 1 << 7) as u16;
            status = luad_pcall(
                interpreter,
                Some(dothecall as unsafe extern "C" fn(*mut Interpreter, *mut libc::c_void) -> ()),
                std::ptr::null_mut(),
                ((*interpreter).top.stkidrel_pointer.offset(-(2 as isize)) as *mut i8)
                    .offset_from((*interpreter).stack.stkidrel_pointer as *mut i8) as i64,
                0,
            );
            (*(*interpreter).call_info).call_status =
                ((*(*interpreter).call_info).call_status as i32 & !(1 << 7)) as u16;
            (*interpreter).allow_hook = oldah;
            (*global).gc_step = oldgcstp as u8;
            if ((status != 0) as i32 != 0) as i64 != 0 {
                luae_warnerror(interpreter, b"__gc\0" as *const u8 as *const i8);
                (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(-1);
            }
        }
    }
}
pub unsafe extern "C" fn runafewfinalizers(interpreter: *mut Interpreter, n: i32) -> i32 {
    unsafe {
        let global: *mut Global = (*interpreter).global;
        let mut i: i32;
        i = 0;
        while i < n && !((*global).to_be_finalized).is_null() {
            gctm_function(interpreter);
            i += 1;
        }
        return i;
    }
}
pub unsafe extern "C" fn callallpendingfinalizers(interpreter: *mut Interpreter) {
    unsafe {
        let global: *mut Global = (*interpreter).global;
        while !((*global).to_be_finalized).is_null() {
            gctm_function(interpreter);
        }
    }
}
pub unsafe extern "C" fn luac_checkfinalizer(interpreter: *mut Interpreter, o: *mut Object, metatable: *mut Table) {
    unsafe {
        let global: *mut Global = (*interpreter).global;
        if (*o).get_marked() & 1 << 6 != 0
            || (if metatable.is_null() {
                std::ptr::null()
            } else {
                if (*metatable).flags as u32 & (1 as u32) << TM_GC as i32 != 0 {
                    std::ptr::null()
                } else {
                    luat_gettm(metatable, TM_GC, (*global).tm_name[TM_GC as usize])
                }
            })
            .is_null()
            || (*global).gc_step as i32 & 4 != 0
        {
            return;
        } else {
            if 3 <= (*global).gc_state as i32 && (*global).gc_state as i32 <= 6 {
                (*o).set_marked(
                    (*o).get_marked() & !(1 << 5 | (1 << 3 | 1 << 4))
                        | ((*global).current_white & (1 << 3 | 1 << 4)),
                );
                if (*global).sweep_gc == &mut (*o).next as *mut *mut Object {
                    (*global).sweep_gc = sweeptolive(interpreter, (*global).sweep_gc);
                }
            } else {
                correctpointers(global, o);
            }
            let mut p: *mut *mut Object = &mut (*global).all_gc;
            while *p != o {
                p = &mut (**p).next;
            }
            *p = (*o).next;
            (*o).next = (*global).finalized_objects;
            (*global).finalized_objects = o;
            (*o).set_marked(((*o).get_marked() | 1 << 6) as u8);
        };
    }
}
pub unsafe extern "C" fn sweep2old(interpreter: *mut Interpreter, mut p: *mut *mut Object) {
    unsafe {
        let global: *mut Global = (*interpreter).global;
        loop {
            let curr: *mut Object = *p;
            if curr.is_null() {
                break;
            }
            if (*curr).get_marked() & (1 << 3 | 1 << 4) != 0 {
                *p = (*curr).next;
                free_object(interpreter, curr);
            } else {
                (*curr).set_marked((*curr).get_marked() & !(7) | 4);
                if (*curr).get_tag_variant() == TAG_VARIANT_STATE {
                    let other_state: *mut Interpreter = &mut (*(curr as *mut Interpreter));
                    linkgclist_(
                        &mut (*(other_state as *mut Object)),
                        &mut (*other_state).gc_list,
                        &mut (*global).gray_again,
                    );
                } else if (*curr).get_tag_variant() == TAG_VARIANT_UPVALUE
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
    interpreter: *mut Interpreter,
    global: *mut Global,
    mut p: *mut *mut Object,
    limit: *mut Object,
    pfirstold1: *mut *mut Object,
) -> *mut *mut Object {
    unsafe {
        static mut NEXT_AGE: [u8; 7] = [1, 3 as u8, 3 as u8, 4 as u8, 4 as u8, 5 as u8, 6 as u8];
        let white = (*global).current_white & (1 << 3 | 1 << 4);
        loop {
            let curr: *mut Object = *p;
            if !(curr != limit) {
                break;
            }
            if (*curr).get_marked() & (1 << 3 | 1 << 4) != 0 {
                *p = (*curr).next;
                free_object(interpreter, curr);
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
pub unsafe extern "C" fn finishgencycle(interpreter: *mut Interpreter, global: *mut Global) {
    unsafe {
        correctgraylists(global);
        check_sizes(interpreter, global);
        (*global).gc_state = 0;
        if !(*global).is_emergency {
            callallpendingfinalizers(interpreter);
        }
    }
}
pub unsafe extern "C" fn youngcollection(interpreter: *mut Interpreter, global: *mut Global) {
    unsafe {
        if !((*global).first_old1).is_null() {
            markold(global, (*global).first_old1, (*global).really_old);
            (*global).first_old1 = std::ptr::null_mut();
        }
        markold(global, (*global).finalized_objects, (*global).finobjrold);
        markold(global, (*global).to_be_finalized, std::ptr::null_mut());
        atomic(interpreter);
        (*global).gc_state = 3 as u8;
        let mut psurvival: *mut *mut Object = sweepgen(
            interpreter,
            global,
            &mut (*global).all_gc,
            (*global).survival,
            &mut (*global).first_old1,
        );
        sweepgen(interpreter, global, psurvival, (*global).old1, &mut (*global).first_old1);
        (*global).really_old = (*global).old1;
        (*global).old1 = *psurvival;
        (*global).survival = (*global).all_gc;
        let mut dummy: *mut Object = std::ptr::null_mut();
        psurvival = sweepgen(interpreter, global, &mut (*global).finalized_objects, (*global).finobjsur, &mut dummy);
        sweepgen(interpreter, global, psurvival, (*global).finobjold1, &mut dummy);
        (*global).finobjrold = (*global).finobjold1;
        (*global).finobjold1 = *psurvival;
        (*global).finobjsur = (*global).finalized_objects;
        sweepgen(
            interpreter,
            global,
            &mut (*global).to_be_finalized,
            std::ptr::null_mut(),
            &mut dummy,
        );
        finishgencycle(interpreter, global);
    }
}
pub unsafe extern "C" fn atomic2gen(interpreter: *mut Interpreter, global: *mut Global) {
    unsafe {
        cleargraylists(global);
        (*global).gc_state = 3 as u8;
        sweep2old(interpreter, &mut (*global).all_gc);
        (*global).survival = (*global).all_gc;
        (*global).old1 = (*global).survival;
        (*global).really_old = (*global).old1;
        (*global).first_old1 = std::ptr::null_mut();
        sweep2old(interpreter, &mut (*global).finalized_objects);
        (*global).finobjsur = (*global).finalized_objects;
        (*global).finobjold1 = (*global).finobjsur;
        (*global).finobjrold = (*global).finobjold1;
        sweep2old(interpreter, &mut (*global).to_be_finalized);
        (*global).gc_kind = 1;
        (*global).last_atomic = 0;
        (*global).gc_estimate = ((*global).total_bytes + (*global).gc_debt) as u64;
        finishgencycle(interpreter, global);
    }
}
pub unsafe extern "C" fn entergen(interpreter: *mut Interpreter, global: *mut Global) -> u64 {
    unsafe {
        luac_runtilstate(interpreter, 1 << 8);
        luac_runtilstate(interpreter, 1 << 0);
        let numobjs: u64 = atomic(interpreter);
        atomic2gen(interpreter, global);
        (*global).set_minor_debt();
        return numobjs;
    }
}
pub unsafe extern "C" fn luac_changemode(interpreter: *mut Interpreter, newmode: i32) {
    unsafe {
        let global: *mut Global = (*interpreter).global;
        if newmode != (*global).gc_kind as i32 {
            if newmode == 1 {
                entergen(interpreter, global);
            } else {
                (*global).enter_incremental();
            }
        }
        (*global).last_atomic = 0;
    }
}
pub unsafe extern "C" fn fullgen(interpreter: *mut Interpreter, global: *mut Global) -> u64 {
    unsafe {
        (*global).enter_incremental();
        return entergen(interpreter, global);
    }
}
pub unsafe extern "C" fn stepgenfull(interpreter: *mut Interpreter, global: *mut Global) {
    unsafe {
        let lastatomic: u64 = (*global).last_atomic;
        if (*global).gc_kind as i32 == 1 {
            (*global).enter_incremental();
        }
        luac_runtilstate(interpreter, 1 << 0);
        let newatomic: u64 = atomic(interpreter);
        if newatomic < lastatomic.wrapping_add(lastatomic >> 3) {
            atomic2gen(interpreter, global);
            (*global).set_minor_debt();
        } else {
            (*global).gc_estimate = ((*global).total_bytes + (*global).gc_debt) as u64;
            entersweep(interpreter);
            luac_runtilstate(interpreter, 1 << 8);
            setpause(global);
            (*global).last_atomic = newatomic;
        };
    }
}
pub unsafe extern "C" fn genstep(interpreter: *mut Interpreter, global: *mut Global) {
    unsafe {
        if (*global).last_atomic != 0 {
            stepgenfull(interpreter, global);
        } else {
            let majorbase: u64 = (*global).gc_estimate;
            let majorinc: u64 = majorbase
                .wrapping_div(100 as u64)
                .wrapping_mul((*global).generational_major_multiplier * 4);
            if (*global).gc_debt > 0
                && ((*global).total_bytes + (*global).gc_debt) as u64 > majorbase.wrapping_add(majorinc)
            {
                let numobjs: u64 = fullgen(interpreter, global);
                if !((((*global).total_bytes + (*global).gc_debt) as u64)
                    < majorbase.wrapping_add(majorinc.wrapping_div(2 as u64)))
                {
                    (*global).last_atomic = numobjs;
                    setpause(global);
                }
            } else {
                youngcollection(interpreter, global);
                (*global).set_minor_debt();
                (*global).gc_estimate = majorbase;
            }
        };
    }
}
pub unsafe extern "C" fn entersweep(interpreter: *mut Interpreter) {
    unsafe {
        let global: *mut Global = (*interpreter).global;
        (*global).gc_state = 3 as u8;
        (*global).sweep_gc = sweeptolive(interpreter, &mut (*global).all_gc);
    }
}
pub unsafe extern "C" fn luac_freeallobjects(interpreter: *mut Interpreter) {
    unsafe {
        let global: *mut Global = (*interpreter).global;
        (*global).gc_step = 4 as u8;
        luac_changemode(interpreter, 0);
        separatetobefnz(global, 1);
        callallpendingfinalizers(interpreter);
        delete_list(
            interpreter,
            (*global).all_gc,
            &mut (*((*global).main_state as *mut Object)),
        );
        delete_list(interpreter, (*global).fixed_gc, std::ptr::null_mut());
    }
}
pub unsafe extern "C" fn atomic(interpreter: *mut Interpreter) -> u64 {
    unsafe {
        let global: *mut Global = (*interpreter).global;
        let mut work: u64 = 0;
        let grayagain: *mut Object = (*global).gray_again;
        (*global).gray_again = std::ptr::null_mut();
        (*global).gc_state = 2 as u8;
        if (*interpreter).get_marked() & (1 << 3 | 1 << 4) != 0 {
            really_mark_object(global, &mut (*(interpreter as *mut Object)));
        }
        if ((*global).l_registry.is_collectable())
            && (*(*global).l_registry.value.object).get_marked() & (1 << 3 | 1 << 4) != 0
        {
            really_mark_object(global, (*global).l_registry.value.object);
        }
        (*global).markmt();
        work = (work as u64).wrapping_add(propagateall(global)) as u64;
        work = (work as u64).wrapping_add(remarkupvals(global) as u64) as u64;
        work = (work as u64).wrapping_add(propagateall(global)) as u64;
        (*global).gray = grayagain;
        work = (work as u64).wrapping_add(propagateall(global)) as u64;
        convergeephemerons(global);
        clearbyvalues(global, (*global).weak, std::ptr::null_mut());
        clearbyvalues(global, (*global).all_weak, std::ptr::null_mut());
        let origweak: *mut Object = (*global).weak;
        let origall: *mut Object = (*global).all_weak;
        separatetobefnz(global, 0);
        work = (work as u64).wrapping_add(markbeingfnz(global)) as u64;
        work = (work as u64).wrapping_add(propagateall(global)) as u64;
        convergeephemerons(global);
        clearbykeys(global, (*global).ephemeron);
        clearbykeys(global, (*global).all_weak);
        clearbyvalues(global, (*global).weak, origweak);
        clearbyvalues(global, (*global).all_weak, origall);
        (*global).stringcache_clear();
        (*global).current_white = ((*global).current_white as i32 ^ (1 << 3 | 1 << 4)) as u8;
        return work;
    }
}
pub unsafe extern "C" fn sweepstep(
    interpreter: *mut Interpreter,
    global: *mut Global,
    nextstate: i32,
    nextlist: *mut *mut Object,
) -> i32 {
    unsafe {
        if !((*global).sweep_gc).is_null() {
            let olddebt: i64 = (*global).gc_debt;
            let mut count: i32 = 0;
            (*global).sweep_gc = (*interpreter).sweep_list((*global).sweep_gc, 100 as i32, &mut count);
            (*global).gc_estimate = ((*global).gc_estimate as u64)
                .wrapping_add(((*global).gc_debt - olddebt) as u64) as u64
                as u64;
            return count;
        } else {
            (*global).gc_state = nextstate as u8;
            (*global).sweep_gc = nextlist;
            return 0;
        };
    }
}
pub unsafe extern "C" fn singlestep(interpreter: *mut Interpreter) -> u64 {
    unsafe {
        let global: *mut Global = (*interpreter).global;
        let work: u64;
        (*global).gcstopem = 1;
        match (*global).gc_state as i32 {
            8 => {
                restartcollection(global);
                (*global).gc_state = 0;
                work = 1 as u64;
            }
            0 => {
                if ((*global).gray).is_null() {
                    (*global).gc_state = 1;
                    work = 0;
                } else {
                    work = (*global).propagatemark();
                }
            }
            1 => {
                work = atomic(interpreter);
                entersweep(interpreter);
                (*global).gc_estimate = ((*global).total_bytes + (*global).gc_debt) as u64;
            }
            3 => {
                work = sweepstep(interpreter, global, 4, &mut (*global).finalized_objects) as u64;
            }
            4 => {
                work = sweepstep(interpreter, global, 5, &mut (*global).to_be_finalized) as u64;
            }
            5 => {
                work = sweepstep(interpreter, global, 6, std::ptr::null_mut()) as u64;
            }
            6 => {
                check_sizes(interpreter, global);
                (*global).gc_state = 7 as u8;
                work = 0;
            }
            7 => {
                if !((*global).to_be_finalized).is_null() && !(*global).is_emergency {
                    (*global).gcstopem = 0;
                    work = (runafewfinalizers(interpreter, 10 as i32) * 50 as i32) as u64;
                } else {
                    (*global).gc_state = 8 as u8;
                    work = 0;
                }
            }
            _ => return 0u64,
        }
        (*global).gcstopem = 0;
        return work;
    }
}
pub unsafe extern "C" fn luac_runtilstate(interpreter: *mut Interpreter, statesmask: i32) {
    unsafe {
        let global: *mut Global = (*interpreter).global;
        while statesmask & 1 << (*global).gc_state as i32 == 0 {
            singlestep(interpreter);
        }
    }
}
pub unsafe extern "C" fn incstep(interpreter: *mut Interpreter, global: *mut Global) {
    unsafe {
        let stepmul: i32 = (*global).gc_step_multiplier as i32 * 4 | 1;
        let mut debt: i64 = ((*global).gc_debt as u64)
            .wrapping_div(::core::mem::size_of::<TValue>() as u64)
            .wrapping_mul(stepmul as u64) as i64;
        let stepsize: i64 = (if (*global).gc_step_size as u64
            <= (::core::mem::size_of::<i64>() as u64)
                .wrapping_mul(8 as u64)
                .wrapping_sub(2 as u64)
        {
            ((1 << (*global).gc_step_size as i32) as u64)
                .wrapping_div(::core::mem::size_of::<TValue>() as u64)
                .wrapping_mul(stepmul as u64)
        } else {
            (!(0u64) >> 1) as u64
        }) as i64;
        loop {
            let work: u64 = singlestep(interpreter);
            debt = (debt as u64).wrapping_sub(work) as i64;
            if !(debt > -stepsize && (*global).gc_state as i32 != 8) {
                break;
            }
        }
        if (*global).gc_state as i32 == 8 {
            setpause(global);
        } else {
            debt = ((debt / stepmul as i64) as u64)
                .wrapping_mul(::core::mem::size_of::<TValue>() as u64) as i64;
            (*global).set_debt(debt);
        };
    }
}
pub unsafe extern "C" fn luac_step(interpreter: *mut Interpreter) {
    unsafe {
        let global: *mut Global = (*interpreter).global;
        if !((*global).gc_step as i32 == 0) {
            (*global).set_debt(-(2000 as i32) as i64);
        } else if (*global).gc_kind as i32 == 1 || (*global).last_atomic != 0 {
            genstep(interpreter, global);
        } else {
            incstep(interpreter, global);
        };
    }
}
pub unsafe extern "C" fn fullinc(interpreter: *mut Interpreter, global: *mut Global) {
    unsafe {
        if (*global).gc_state as i32 <= 2 {
            entersweep(interpreter);
        }
        luac_runtilstate(interpreter, 1 << 8);
        luac_runtilstate(interpreter, 1 << 0);
        (*global).gc_state = 1;
        luac_runtilstate(interpreter, 1 << 7);
        luac_runtilstate(interpreter, 1 << 8);
        setpause(global);
    }
}
pub unsafe extern "C" fn luac_fullgc(interpreter: *mut Interpreter, is_emergency: bool) {
    unsafe {
        (*((*interpreter).global)).is_emergency = is_emergency;
        if (*((*interpreter).global)).gc_kind as i32 == 0 {
            fullinc(interpreter, (*interpreter).global);
        } else {
            fullgen(interpreter, (*interpreter).global);
        }
        (*((*interpreter).global)).is_emergency = false;
    }
}
pub unsafe extern "C" fn callclosemethod(
    interpreter: *mut Interpreter,
    obj: *mut TValue,
    err: *mut TValue,
    yy: i32,
) {
    unsafe {
        let top: StackValuePointer = (*interpreter).top.stkidrel_pointer;
        let tm: *const TValue = luat_gettmbyobj(interpreter, obj, TM_CLOSE);
        let io1: *mut TValue = &mut (*top).tvalue;
        let io2: *const TValue = tm;
        (*io1).copy_from(&*io2);
        let io1_0: *mut TValue = &mut (*top.offset(1 as isize)).tvalue;
        let io2_0: *const TValue = obj;
        (*io1_0).copy_from(&(*io2_0));
        let io1_1: *mut TValue = &mut (*top.offset(2 as isize)).tvalue;
        let io2_1: *const TValue = err;
        (*io1_1).copy_from(&(*io2_1));
        (*interpreter).top.stkidrel_pointer = top.offset(3 as isize);
        if yy != 0 {
            ccall(interpreter, top, 0, 1);
        } else {
            luad_callnoyield(interpreter, top, 0);
        };
    }
}
pub unsafe extern "C" fn checkclosemth(interpreter: *mut Interpreter, level: StackValuePointer) {
    unsafe {
        let tm: *const TValue = luat_gettmbyobj(interpreter, &mut (*level).tvalue, TM_CLOSE);
        if (*tm).is_tagtype_nil() {
            let index: i32 = level.offset_from((*(*interpreter).call_info).function.stkidrel_pointer) as i32;
            let mut vname: *const i8 =
                luag_findlocal(interpreter, (*interpreter).call_info, index, std::ptr::null_mut());
            if vname.is_null() {
                vname = b"?\0" as *const u8 as *const i8;
            }
            luag_runerror(
                interpreter,
                b"variable '%s' got a non-closable value\0" as *const u8 as *const i8,
                vname,
            );
        }
    }
}
pub unsafe extern "C" fn prepcallclosemth(interpreter: *mut Interpreter, level: StackValuePointer, status: i32, yy: i32) {
    unsafe {
        let uv: *mut TValue = &mut (*level).tvalue;
        let errobj: *mut TValue;
        if status == -1 {
            errobj = &mut (*(*interpreter).global).none_value;
        } else {
            errobj = &mut (*level.offset(1 as isize)).tvalue;
            (*interpreter).set_error_object(status, level.offset(1 as isize));
        }
        callclosemethod(interpreter, uv, errobj, yy);
    }
}
pub unsafe extern "C" fn luaf_newtbcupval(interpreter: *mut Interpreter, level: StackValuePointer) {
    unsafe {
        if (*level).tvalue.get_tag_variant() == TAG_VARIANT_BOOLEAN_FALSE
            || (*level).tvalue.is_tagtype_nil()
        {
            return;
        }
        checkclosemth(interpreter, level);
        while level.offset_from((*interpreter).tbc_list.stkidrel_pointer) as u64
            > ((256 as u64)
                << (::core::mem::size_of::<u16>() as u64)
                    .wrapping_sub(1 as u64)
                    .wrapping_mul(8 as u64))
            .wrapping_sub(1 as u64)
        {
            (*interpreter).tbc_list.stkidrel_pointer = ((*interpreter).tbc_list.stkidrel_pointer).offset(
                ((256 as u64)
                    << (::core::mem::size_of::<u16>() as u64)
                        .wrapping_sub(1 as u64)
                        .wrapping_mul(8 as u64))
                .wrapping_sub(1 as u64) as isize,
            );
            (*(*interpreter).tbc_list.stkidrel_pointer).delta = 0;
        }
        (*level).delta = level.offset_from((*interpreter).tbc_list.stkidrel_pointer) as u16;
        (*interpreter).tbc_list.stkidrel_pointer = level;
    }
}
pub unsafe extern "C" fn luaf_closeupval(interpreter: *mut Interpreter, level: StackValuePointer) {
    unsafe {
        loop {
            let uv: *mut UpValue = (*interpreter).open_upvalue;
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
            (*io1).copy_from(&*io2);
            (*uv).v.p = slot;
            if (*uv).get_marked() & (1 << 3 | 1 << 4) == 0 {
                (*uv).set_marked((*uv).get_marked() | 1 << 5);
                if (*slot).is_collectable() {
                    if (*uv).get_marked() & 1 << 5 != 0
                        && (*(*slot).value.object).get_marked() & (1 << 3 | 1 << 4) != 0
                    {
                        luac_barrier_(
                            interpreter,
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
pub unsafe extern "C" fn poptbclist(interpreter: *mut Interpreter) {
    unsafe {
        let mut tbc: StackValuePointer = (*interpreter).tbc_list.stkidrel_pointer;
        tbc = tbc.offset(-((*tbc).delta as isize));
        while tbc > (*interpreter).stack.stkidrel_pointer && (*tbc).delta == 0 {
            tbc = tbc.offset(
                -(((256 as u64)
                    << (::core::mem::size_of::<u16>() as u64)
                        .wrapping_sub(1 as u64)
                        .wrapping_mul(8 as u64))
                .wrapping_sub(1 as u64) as isize),
            );
        }
        (*interpreter).tbc_list.stkidrel_pointer = tbc;
    }
}
pub unsafe extern "C" fn luaf_close(
    interpreter: *mut Interpreter,
    mut level: StackValuePointer,
    status: i32,
    yy: i32,
) -> StackValuePointer {
    unsafe {
        let levelrel: i64 = (level as *mut i8).offset_from((*interpreter).stack.stkidrel_pointer as *mut i8) as i64;
        luaf_closeupval(interpreter, level);
        while (*interpreter).tbc_list.stkidrel_pointer >= level {
            let tbc: StackValuePointer = (*interpreter).tbc_list.stkidrel_pointer;
            poptbclist(interpreter);
            prepcallclosemth(interpreter, tbc, status, yy);
            level = ((*interpreter).stack.stkidrel_pointer as *mut i8).offset(levelrel as isize) as StackValuePointer;
        }
        return level;
    }
}
pub unsafe extern "C" fn luay_parser(
    interpreter: *mut Interpreter,
    zio: *mut ZIO,
    buffer: *mut Buffer,
    dynamic_data: *mut DynamicData,
    name: *const i8,
    firstchar: i32,
) -> *mut Closure {
    unsafe {
        let mut lexstate: LexicalState = LexicalState::new();
        let mut funcstate: FunctionState = FunctionState::new();
        let cl: *mut Closure = luaf_newlclosure(interpreter, 1);
        let io: *mut TValue = &mut (*(*interpreter).top.stkidrel_pointer).tvalue;
        let x_: *mut Closure = cl;
        (*io).value.object = &mut (*(x_ as *mut Object));
        (*io).set_tag_variant(TAG_VARIANT_CLOSURE_L);
        (*io).set_collectable(true);
        (*interpreter).luad_inctop();
        lexstate.table = luah_new(interpreter);
        let io_0: *mut TValue = &mut (*(*interpreter).top.stkidrel_pointer).tvalue;
        let x0: *mut Table = lexstate.table;
        (*io_0).value.object = &mut (*(x0 as *mut Object));
        (*io_0).set_tag_variant(TAG_VARIANT_TABLE);
        (*io_0).set_collectable(true);
        (*interpreter).luad_inctop();
        (*cl).payload.l_prototype = luaf_newproto(interpreter);
        funcstate.prototype = (*cl).payload.l_prototype;
        if (*cl).get_marked() & 1 << 5 != 0 && (*(*cl).payload.l_prototype).get_marked() & (1 << 3 | 1 << 4) != 0 {
            luac_barrier_(
                interpreter,
                &mut (*(cl as *mut Object)),
                &mut (*((*cl).payload.l_prototype as *mut Object)),
            );
        } else {
        };
        (*funcstate.prototype).prototype_source = luas_new(interpreter, name);
        if (*funcstate.prototype).get_marked() & 1 << 5 != 0
            && (*(*funcstate.prototype).prototype_source).get_marked() & (1 << 3 | 1 << 4) != 0
        {
            luac_barrier_(
                interpreter,
                &mut (*(funcstate.prototype as *mut Object)),
                &mut (*((*funcstate.prototype).prototype_source as *mut Object)),
            );
        } else {
        };
        lexstate.buffer = buffer;
        lexstate.dynamic_data = dynamic_data;
        (*dynamic_data).label.length = 0;
        (*dynamic_data).gt.length = (*dynamic_data).label.length;
        (*dynamic_data).active_variable.length = (*dynamic_data).gt.length;
        luax_setinput(interpreter, &mut lexstate, zio, (*funcstate.prototype).prototype_source, firstchar);
        mainfunc(&mut lexstate, &mut funcstate);
        (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(-1);
        return cl;
    }
}
pub unsafe extern "C" fn luax_init(interpreter: *mut Interpreter) {
    unsafe {
        let mut i: i32;
        let e: *mut TString = luas_newlstr(
            interpreter,
            b"_ENV\0" as *const u8 as *const i8,
            (::core::mem::size_of::<[i8; 5]>() as u64)
                .wrapping_div(::core::mem::size_of::<i8>() as u64)
                .wrapping_sub(1 as u64),
        );
        fix_object_state(interpreter, &mut (*(e as *mut Object)));
        i = 0;
        while i < TK_WHILE as i32 - (127 as i32 * 2 + 1 + 1) + 1 {
            let ts: *mut TString = luas_new(interpreter, TOKENS[i as usize]);
            fix_object_state(interpreter, &mut (*(ts as *mut Object)));
            (*ts).extra = (i + 1) as u8;
            i += 1;
        }
    }
}
pub unsafe extern "C" fn pushclosure(
    interpreter: *mut Interpreter,
    p: *mut Prototype,
    encup: *mut *mut UpValue,
    base: StackValuePointer,
    ra: StackValuePointer,
) {
    unsafe {
        let nup: i32 = (*p).prototype_upvalues.size;
        let uv: *mut UpValueDescription = (*p).prototype_upvalues.pointer;
        let ncl: *mut Closure = luaf_newlclosure(interpreter, nup);
        (*ncl).payload.l_prototype = p;
        let io: *mut TValue = &mut (*ra).tvalue;
        let x_: *mut Closure = ncl;
        (*io).value.object = &mut (*(x_ as *mut Object));
        (*io).set_tag_variant(TAG_VARIANT_CLOSURE_L);
        (*io).set_collectable(true);
        for i in 0..nup {
            if (*uv.offset(i as isize)).is_in_stack {
                let ref mut fresh136 = *((*ncl).upvalues).l_upvalues.as_mut_ptr().offset(i as isize);
                *fresh136 = luaf_findupval(
                    interpreter,
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
                    interpreter,
                    &mut (*(ncl as *mut Object)),
                    &mut (*(*((*ncl).upvalues).l_upvalues.as_mut_ptr().offset(i as isize) as *mut Object)),
                );
            } else {
            };
        }
    }
}
pub unsafe extern "C" fn luav_finishop(interpreter: *mut Interpreter) {
    unsafe {
        let call_info: *mut CallInfo = (*interpreter).call_info;
        let base: StackValuePointer = ((*call_info).function.stkidrel_pointer).offset(1 as isize);
        let inst: u32 = *((*call_info).u.l.saved_program_counter).offset(-(1 as isize));
        let op: u32 = (inst >> 0 & !(!(0u32) << 7) << 0) as u32;
        match op as u32 {
            46 | 47 | 48 => {
                let io1: *mut TValue = &mut (*base.offset(
                    (*((*call_info).u.l.saved_program_counter).offset(-(2 as isize)) >> POSITION_A
                        & !(!(0u32) << 8) << 0) as isize,
                ))
                .tvalue;
                (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(-1);
                let io2: *const TValue = &mut (*(*interpreter).top.stkidrel_pointer).tvalue;
                (*io1).copy_from(&*io2);
            }
            49 | 50 | 52 | 11 | 12 | 13 | 14 | 20 => {
                let io1_0: *mut TValue = &mut (*base
                    .offset((inst >> POSITION_A & !(!(0u32) << 8) << 0) as isize))
                .tvalue;
                (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(-1);
                let io2_0: *const TValue = &mut (*(*interpreter).top.stkidrel_pointer).tvalue;
                (*io1_0).copy_from(&*io2_0);
            }
            58 | 59 | 62 | 63 | 64 | 65 | 57 => {
                let res: i32 = !((*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))).tvalue.get_tag_variant()
                    == TAG_VARIANT_BOOLEAN_FALSE
                    || (*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))).tvalue.get_tag_type()
                        == TagType::Nil) as i32;
                (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(-1);
                if res != (inst >> POSITION_K & !(!(0u32) << 1) << 0) as i32 {
                    (*call_info).u.l.saved_program_counter =
                        ((*call_info).u.l.saved_program_counter).offset(1);
                    (*call_info).u.l.saved_program_counter;
                }
            }
            53 => {
                let top: StackValuePointer = (*interpreter).top.stkidrel_pointer.offset(-(1 as isize));
                let a: i32 = (inst >> POSITION_A & !(!(0u32) << 8) << 0) as i32;
                let total: i32 =
                    top.offset(-(1 as isize))
                        .offset_from(base.offset(a as isize)) as i32;
                let io1_1: *mut TValue = &mut (*top.offset(-(2 as isize))).tvalue;
                let io2_1: *const TValue = &mut (*top).tvalue;
                (*io1_1).copy_from(&(*io2_1));
                (*interpreter).top.stkidrel_pointer = top.offset(-(1 as isize));
                concatenate(interpreter, total);
            }
            54 => {
                (*call_info).u.l.saved_program_counter =
                    ((*call_info).u.l.saved_program_counter).offset(-1);
                (*call_info).u.l.saved_program_counter;
            }
            70 => {
                let ra: StackValuePointer = base.offset((inst >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                (*interpreter).top.stkidrel_pointer = ra.offset((*call_info).u2.nres as isize);
                (*call_info).u.l.saved_program_counter =
                    ((*call_info).u.l.saved_program_counter).offset(-1);
                (*call_info).u.l.saved_program_counter;
            }
            _ => {}
        };
    }
}
pub unsafe extern "C" fn luav_execute(interpreter: *mut Interpreter, mut call_info: *mut CallInfo) {
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
            trap = (*interpreter).hook_mask;
            '_returning: loop {
                cl = &mut (*((*(*call_info).function.stkidrel_pointer).tvalue.value.object as *mut Closure));
                k = (*(*cl).payload.l_prototype).prototype_constants.pointer;
                program_counter = (*call_info).u.l.saved_program_counter;
                if (trap != 0) as i64 != 0 {
                    trap = luag_tracecall(interpreter);
                }
                base = ((*call_info).function.stkidrel_pointer).offset(1 as isize);
                loop {
                    if (trap != 0) as i64 != 0 {
                        trap = luag_traceexec(interpreter, program_counter);
                        base = ((*call_info).function.stkidrel_pointer).offset(1 as isize);
                    }
                    let fresh138 = program_counter;
                    program_counter = program_counter.offset(1);
                    i = *fresh138;
                    match (i >> 0 & !(!(0u32) << 7) << 0) as u32 {
                        0 => {
                            let ra: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let io1: *mut TValue = &mut (*ra).tvalue;
                            let io2: *const TValue = &mut (*base.offset(
                                (i >> POSITION_B & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            (*io1).copy_from(&*io2);
                            continue;
                        }
                        1 => {
                            let ra_0: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let b: i64 = ((i >> POSITION_K & !(!(0u32) << 8 + 8 + 1) << 0) as i32
                                - ((1 << 8 + 8 + 1) - 1 >> 1))
                                as i64;
                            let io: *mut TValue = &mut (*ra_0).tvalue;
                            (*io).value.integer = b;
                            (*io).set_tag_variant(TAG_VARIANT_NUMERIC_INTEGER);
                            continue;
                        }
                        2 => {
                            let ra_1: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let b_0: i32 = (i >> POSITION_K & !(!(0u32) << 8 + 8 + 1) << 0) as i32
                                - ((1 << 8 + 8 + 1) - 1 >> 1);
                            let io_0: *mut TValue = &mut (*ra_1).tvalue;
                            (*io_0).value.number = b_0 as f64;
                            (*io_0).set_tag_variant(TAG_VARIANT_NUMERIC_NUMBER);
                            continue;
                        }
                        3 => {
                            let ra_2: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let rb: *mut TValue = k.offset(
                                (i >> POSITION_K & !(!(0u32) << 8 + 8 + 1) << 0) as isize,
                            );
                            let io1_0: *mut TValue = &mut (*ra_2).tvalue;
                            let io2_0: *const TValue = rb;
                            (*io1_0).copy_from(&(*io2_0));
                            continue;
                        }
                        4 => {
                            let ra_3: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let rb_0: *mut TValue = k.offset(
                                (*program_counter >> POSITION_A & !(!(0u32) << 8 + 8 + 1 + 8) << 0)
                                    as isize,
                            );
                            program_counter = program_counter.offset(1);
                            let io1_1: *mut TValue = &mut (*ra_3).tvalue;
                            let io2_1: *const TValue = rb_0;
                            (*io1_1).copy_from(&(*io2_1));
                            continue;
                        }
                        5 => {
                            let ra_4: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            (*ra_4).tvalue.set_tag_variant(TAG_VARIANT_BOOLEAN_FALSE);
                            continue;
                        }
                        6 => {
                            let ra_5: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            (*ra_5).tvalue.set_tag_variant(TAG_VARIANT_BOOLEAN_FALSE);
                            program_counter = program_counter.offset(1);
                            continue;
                        }
                        7 => {
                            let ra_6: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            (*ra_6).tvalue.set_tag_variant(TAG_VARIANT_BOOLEAN_TRUE);
                            continue;
                        }
                        8 => {
                            let mut ra_7: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let mut b_1: i32 = (i >> POSITION_B & !(!(0u32) << 8) << 0) as i32;
                            loop {
                                let fresh139 = ra_7;
                                ra_7 = ra_7.offset(1);
                                (*fresh139).tvalue.set_tag_variant(TagVariant::NilNil as u8);
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
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let b_2: i32 = (i >> POSITION_B & !(!(0u32) << 8) << 0) as i32;
                            let io1_2: *mut TValue = &mut (*ra_8).tvalue;
                            let io2_2: *const TValue =
                                (**((*cl).upvalues).l_upvalues.as_mut_ptr().offset(b_2 as isize)).v.p;
                            (*io1_2).copy_from(&(*io2_2));
                            continue;
                        }
                        10 => {
                            let ra_9: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let uv: *mut UpValue = *((*cl).upvalues).l_upvalues.as_mut_ptr().offset(
                                (i >> POSITION_B & !(!(0u32) << 8) << 0) as isize,
                            );
                            let io1_3: *mut TValue = (*uv).v.p;
                            let io2_3: *const TValue = &mut (*ra_9).tvalue;
                            (*io1_3).copy_from(&(*io2_3));
                            if (*ra_9).tvalue.is_collectable() {
                                if (*uv).get_marked() & 1 << 5 != 0
                                    && (*(*ra_9).tvalue.value.object).get_marked()
                                        & (1 << 3 | 1 << 4)
                                        != 0
                                {
                                    luac_barrier_(
                                        interpreter,
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
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let slot: *const TValue;
                            let count_upvalues: *mut TValue =
                                (**((*cl).upvalues).l_upvalues.as_mut_ptr().offset(
                                    (i >> POSITION_B & !(!(0u32) << 8) << 0) as isize,
                                ))
                                .v
                                .p;
                            let rc: *mut TValue = k.offset(
                                (i >> POSITION_C & !(!(0u32) << 8) << 0) as isize,
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
                                !(*slot).is_tagtype_nil() as i32
                            } != 0
                            {
                                let io1_4: *mut TValue = &mut (*ra_10).tvalue;
                                let io2_4: *const TValue = slot;
                                (*io1_4).copy_from(&(*io2_4));
                            } else {
                                (*call_info).u.l.saved_program_counter = program_counter;
                                (*interpreter).top.stkidrel_pointer = (*call_info).top.stkidrel_pointer;
                                luav_finishget(interpreter, count_upvalues, rc, ra_10, slot);
                                trap = (*call_info).u.l.trap;
                            }
                            continue;
                        }
                        12 => {
                            let ra_11: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let slot_0: *const TValue;
                            let rb_1: *mut TValue = &mut (*base.offset(
                                (i >> POSITION_B & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let rc_0: *mut TValue = &mut (*base.offset(
                                (i >> POSITION_C & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let n: u64;
                            if if (*rc_0).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER {
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
                                    !(*slot_0).is_tagtype_nil() as i32
                                }
                            } else if !((*rb_1).get_tag_variant() == TAG_VARIANT_TABLE) {
                                slot_0 = std::ptr::null();
                                0
                            } else {
                                slot_0 = luah_get(
                                    &mut (*((*rb_1).value.object as *mut Table)),
                                    rc_0,
                                );
                                !(*slot_0).is_tagtype_nil() as i32
                            } != 0
                            {
                                let io1_5: *mut TValue = &mut (*ra_11).tvalue;
                                let io2_5: *const TValue = slot_0;
                                (*io1_5).copy_from(&(*io2_5));
                            } else {
                                (*call_info).u.l.saved_program_counter = program_counter;
                                (*interpreter).top.stkidrel_pointer = (*call_info).top.stkidrel_pointer;
                                luav_finishget(interpreter, rb_1, rc_0, ra_11, slot_0);
                                trap = (*call_info).u.l.trap;
                            }
                            continue;
                        }
                        13 => {
                            let ra_12: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let slot_1: *const TValue;
                            let rb_2: *mut TValue = &mut (*base.offset(
                                (i >> POSITION_B & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let c: i32 = (i >> POSITION_C & !(!(0u32) << 8) << 0) as i32;
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
                                !(*slot_1).is_tagtype_nil() as i32
                            } != 0
                            {
                                let io1_6: *mut TValue = &mut (*ra_12).tvalue;
                                let io2_6: *const TValue = slot_1;
                                (*io1_6).copy_from(&(*io2_6));
                            } else {
                                let mut key_0: TValue = TValue::new(TAG_VARIANT_NIL_NIL);
                                let io_1: *mut TValue = &mut key_0;
                                (*io_1).value.integer = c as i64;
                                (*io_1).set_tag_variant(TAG_VARIANT_NUMERIC_INTEGER);
                                (*call_info).u.l.saved_program_counter = program_counter;
                                (*interpreter).top.stkidrel_pointer = (*call_info).top.stkidrel_pointer;
                                luav_finishget(interpreter, rb_2, &mut key_0, ra_12, slot_1);
                                trap = (*call_info).u.l.trap;
                            }
                            continue;
                        }
                        14 => {
                            let ra_13: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let slot_2: *const TValue;
                            let rb_3: *mut TValue = &mut (*base.offset(
                                (i >> POSITION_B & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let rc_1: *mut TValue = k.offset(
                                (i >> POSITION_C & !(!(0u32) << 8) << 0) as isize,
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
                                !(*slot_2).is_tagtype_nil() as i32
                            } != 0
                            {
                                let io1_7: *mut TValue = &mut (*ra_13).tvalue;
                                let io2_7: *const TValue = slot_2;
                                (*io1_7).copy_from(&(*io2_7));
                            } else {
                                (*call_info).u.l.saved_program_counter = program_counter;
                                (*interpreter).top.stkidrel_pointer = (*call_info).top.stkidrel_pointer;
                                luav_finishget(interpreter, rb_3, rc_1, ra_13, slot_2);
                                trap = (*call_info).u.l.trap;
                            }
                            continue;
                        }
                        15 => {
                            let slot_3: *const TValue;
                            let upval_0: *mut TValue = (**((*cl).upvalues)
                                .l_upvalues.as_mut_ptr()
                                .offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize))
                            .v
                            .p;
                            let rb_4: *mut TValue = k.offset(
                                (i >> POSITION_B & !(!(0u32) << 8) << 0) as isize,
                            );
                            let rc_2: *mut TValue = if (i & (1 as u32) << POSITION_K) as i32 != 0 {
                                k.offset(
                                    (i >> POSITION_C & !(!(0u32) << 8) << 0) as isize,
                                )
                            } else {
                                &mut (*base.offset(
                                    (i >> POSITION_C & !(!(0u32) << 8) << 0) as isize,
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
                                !((*slot_3).is_tagtype_nil()) as i32
                            } != 0
                            {
                                let io1_8: *mut TValue = slot_3 as *mut TValue;
                                let io2_8: *const TValue = rc_2;
                                (*io1_8).copy_from(&(*io2_8));
                                if (*rc_2).is_collectable() {
                                    if (*(*upval_0).value.object).get_marked() & 1 << 5 != 0
                                        && (*(*rc_2).value.object).get_marked() & (1 << 3 | 1 << 4)
                                            != 0
                                    {
                                        luac_barrierback_(interpreter, (*upval_0).value.object);
                                    } else {
                                    };
                                } else {
                                };
                            } else {
                                (*call_info).u.l.saved_program_counter = program_counter;
                                (*interpreter).top.stkidrel_pointer = (*call_info).top.stkidrel_pointer;
                                luav_finishset(interpreter, upval_0, rb_4, rc_2, slot_3);
                                trap = (*call_info).u.l.trap;
                            }
                            continue;
                        }
                        16 => {
                            let ra_14: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let slot_4: *const TValue;
                            let rb_5: *mut TValue = &mut (*base.offset(
                                (i >> POSITION_B & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let rc_3: *mut TValue = if (i & (1 as u32) << POSITION_K) as i32 != 0 {
                                k.offset(
                                    (i >> POSITION_C & !(!(0u32) << 8) << 0) as isize,
                                )
                            } else {
                                &mut (*base.offset(
                                    (i >> POSITION_C & !(!(0u32) << 8) << 0) as isize,
                                ))
                                .tvalue
                            };
                            let n_0: u64;
                            if if (*rb_5).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER {
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
                                    !(*slot_4).is_tagtype_nil() as i32
                                }
                            } else if !((*ra_14).tvalue.get_tag_variant() == TAG_VARIANT_TABLE) {
                                slot_4 = std::ptr::null();
                                0
                            } else {
                                slot_4 = luah_get(
                                    &mut (*((*ra_14).tvalue.value.object as *mut Table)),
                                    rb_5,
                                );
                                !((*slot_4).is_tagtype_nil()) as i32
                            } != 0
                            {
                                let io1_9: *mut TValue = slot_4 as *mut TValue;
                                let io2_9: *const TValue = rc_3;
                                (*io1_9).copy_from(&(*io2_9));
                                if (*rc_3).is_collectable() {
                                    if (*(*ra_14).tvalue.value.object).get_marked() & 1 << 5 != 0
                                        && (*(*rc_3).value.object).get_marked() & (1 << 3 | 1 << 4)
                                            != 0
                                    {
                                        luac_barrierback_(interpreter, (*ra_14).tvalue.value.object);
                                    } else {
                                    };
                                } else {
                                };
                            } else {
                                (*call_info).u.l.saved_program_counter = program_counter;
                                (*interpreter).top.stkidrel_pointer = (*call_info).top.stkidrel_pointer;
                                luav_finishset(interpreter, &mut (*ra_14).tvalue, rb_5, rc_3, slot_4);
                                trap = (*call_info).u.l.trap;
                            }
                            continue;
                        }
                        17 => {
                            let ra_15: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let slot_5: *const TValue;
                            let c_0: i32 = (i >> POSITION_B & !(!(0u32) << 8) << 0) as i32;
                            let rc_4: *mut TValue = if (i & (1 as u32) << POSITION_K) as i32 != 0 {
                                k.offset(
                                    (i >> POSITION_C & !(!(0u32) << 8) << 0) as isize,
                                )
                            } else {
                                &mut (*base.offset(
                                    (i >> POSITION_C & !(!(0u32) << 8) << 0) as isize,
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
                                !((*slot_5).is_tagtype_nil()) as i32
                            } != 0
                            {
                                let io1_10: *mut TValue = slot_5 as *mut TValue;
                                let io2_10: *const TValue = rc_4;
                                (*io1_10).copy_from(&(*io2_10));
                                if (*rc_4).is_collectable() {
                                    if (*(*ra_15).tvalue.value.object).get_marked() & 1 << 5 != 0
                                        && (*(*rc_4).value.object).get_marked() & (1 << 3 | 1 << 4)
                                            != 0
                                    {
                                        luac_barrierback_(interpreter, (*ra_15).tvalue.value.object);
                                    } else {
                                    };
                                } else {
                                };
                            } else {
                                let mut key_3: TValue = TValue::new(TAG_VARIANT_NIL_NIL);
                                let io_2: *mut TValue = &mut key_3;
                                (*io_2).value.integer = c_0 as i64;
                                (*io_2).set_tag_variant(TAG_VARIANT_NUMERIC_INTEGER);
                                (*call_info).u.l.saved_program_counter = program_counter;
                                (*interpreter).top.stkidrel_pointer = (*call_info).top.stkidrel_pointer;
                                luav_finishset(
                                    interpreter,
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
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let slot_6: *const TValue;
                            let rb_6: *mut TValue = k.offset(
                                (i >> POSITION_B & !(!(0u32) << 8) << 0) as isize,
                            );
                            let rc_5: *mut TValue = if (i & (1 as u32) << POSITION_K) as i32 != 0 {
                                k.offset(
                                    (i >> POSITION_C & !(!(0u32) << 8) << 0) as isize,
                                )
                            } else {
                                &mut (*base.offset(
                                    (i >> POSITION_C & !(!(0u32) << 8) << 0) as isize,
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
                                !((*slot_6).is_tagtype_nil()) as i32
                            } != 0
                            {
                                let io1_11: *mut TValue = slot_6 as *mut TValue;
                                let io2_11: *const TValue = rc_5;
                                (*io1_11).copy_from(&(*io2_11));
                                if (*rc_5).is_collectable() {
                                    if (*(*ra_16).tvalue.value.object).get_marked() & 1 << 5 != 0
                                        && (*(*rc_5).value.object).get_marked() & (1 << 3 | 1 << 4)
                                            != 0
                                    {
                                        luac_barrierback_(interpreter, (*ra_16).tvalue.value.object);
                                    } else {
                                    };
                                } else {
                                };
                            } else {
                                (*call_info).u.l.saved_program_counter = program_counter;
                                (*interpreter).top.stkidrel_pointer = (*call_info).top.stkidrel_pointer;
                                luav_finishset(interpreter, &mut (*ra_16).tvalue, rb_6, rc_5, slot_6);
                                trap = (*call_info).u.l.trap;
                            }
                            continue;
                        }
                        OP_NEWTABLE => {
                            let ra_17: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let mut new_table_size = (i >> POSITION_B & !(!(0u32) << 8) << 0) as usize;
                            let mut new_array_size: usize =
                                (i >> POSITION_C & !(!(0u32) << 8) << 0) as usize;
                            let table: *mut Table;
                            if new_table_size > 0 {
                                new_table_size = 1 << new_table_size - 1;
                            }
                            if (i & (1 as u32) << POSITION_K) as i32 != 0 {
                                new_array_size += ((*program_counter >> POSITION_A as usize
                                    & !(!(0u32) << 8 + 8 + 1 + 8) << 0)
                                    as i32
                                    * ((1 << 8) - 1 + 1)) as usize;
                            }
                            program_counter = program_counter.offset(1);
                            (*interpreter).top.stkidrel_pointer = ra_17.offset(1 as isize);
                            table = luah_new(interpreter);
                            let io_3: *mut TValue = &mut (*ra_17).tvalue;
                            let x_: *mut Table = table;
                            (*io_3).value.object = &mut (*(x_ as *mut Object));
                            (*io_3).set_tag_variant(TAG_VARIANT_TABLE);
                            (*io_3).set_collectable(true);
                            if new_table_size != 0 || new_array_size != 0 {
                                luah_resize(interpreter, table, new_array_size, new_table_size);
                            }
                            if (*(*interpreter).global).gc_debt > 0 {
                                (*call_info).u.l.saved_program_counter = program_counter;
                                (*interpreter).top.stkidrel_pointer = ra_17.offset(1 as isize);
                                luac_step(interpreter);
                                trap = (*call_info).u.l.trap;
                            }
                            continue;
                        }
                        20 => {
                            let ra_18: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let slot_7: *const TValue;
                            let rb_7: *mut TValue = &mut (*base.offset(
                                (i >> POSITION_B & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let rc_6: *mut TValue = if (i & (1 as u32) << POSITION_K) as i32 != 0 {
                                k.offset(
                                    (i >> POSITION_C & !(!(0u32) << 8) << 0) as isize,
                                )
                            } else {
                                &mut (*base.offset(
                                    (i >> POSITION_C & !(!(0u32) << 8) << 0) as isize,
                                ))
                                .tvalue
                            };
                            let key_5: *mut TString =
                                &mut (*((*rc_6).value.object as *mut TString));
                            let io1_12: *mut TValue = &mut (*ra_18.offset(1 as isize)).tvalue;
                            let io2_12: *const TValue = rb_7;
                            (*io1_12).copy_from(&(*io2_12));
                            if if !((*rb_7).get_tag_variant() == TAG_VARIANT_TABLE) {
                                slot_7 = std::ptr::null();
                                0
                            } else {
                                slot_7 = luah_getstr(
                                    &mut (*((*rb_7).value.object as *mut Table)),
                                    key_5,
                                );
                                !((*slot_7).is_tagtype_nil()) as i32
                            } != 0
                            {
                                let io1_13: *mut TValue = &mut (*ra_18).tvalue;
                                let io2_13: *const TValue = slot_7;
                                (*io1_13).copy_from(&(*io2_13));
                            } else {
                                (*call_info).u.l.saved_program_counter = program_counter;
                                (*interpreter).top.stkidrel_pointer = (*call_info).top.stkidrel_pointer;
                                luav_finishget(interpreter, rb_7, rc_6, ra_18, slot_7);
                                trap = (*call_info).u.l.trap;
                            }
                            continue;
                        }
                        21 => {
                            let ra_19: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let v1: *mut TValue = &mut (*base.offset(
                                (i >> POSITION_B & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let imm: i32 = (i >> POSITION_C & !(!(0u32) << 8) << 0) as i32
                                - ((1 << 8) - 1 >> 1);
                            if (*v1).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER {
                                let iv1: i64 = (*v1).value.integer;
                                program_counter = program_counter.offset(1);
                                let io_4: *mut TValue = &mut (*ra_19).tvalue;
                                (*io_4).value.integer = (iv1 as u64).wrapping_add(imm as u64) as i64;
                                (*io_4).set_tag_variant(TAG_VARIANT_NUMERIC_INTEGER);
                            } else if (*v1).get_tag_variant() == TAG_VARIANT_NUMERIC_NUMBER {
                                let nb: f64 = (*v1).value.number;
                                let fimm: f64 = imm as f64;
                                program_counter = program_counter.offset(1);
                                let io_5: *mut TValue = &mut (*ra_19).tvalue;
                                (*io_5).value.number = nb + fimm;
                                (*io_5).set_tag_variant(TAG_VARIANT_NUMERIC_NUMBER);
                            }
                            continue;
                        }
                        22 => {
                            let v1_0: *mut TValue = &mut (*base.offset(
                                (i >> POSITION_B & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let v2: *mut TValue = k.offset(
                                (i >> POSITION_C & !(!(0u32) << 8) << 0) as isize,
                            );
                            let ra_20: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            if (*v1_0).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER
                                && (*v2).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER
                            {
                                let i1: i64 = (*v1_0).value.integer;
                                let i2: i64 = (*v2).value.integer;
                                program_counter = program_counter.offset(1);
                                let io_6: *mut TValue = &mut (*ra_20).tvalue;
                                (*io_6).value.integer = (i1 as u64).wrapping_add(i2 as u64) as i64;
                                (*io_6).set_tag_variant(TAG_VARIANT_NUMERIC_INTEGER);
                            } else {
                                let mut n1: f64 = 0.0;
                                let mut n2: f64 = 0.0;
                                if (if (*v1_0).get_tag_variant() == TAG_VARIANT_NUMERIC_NUMBER {
                                    n1 = (*v1_0).value.number;
                                    1
                                } else {
                                    if (*v1_0).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER {
                                        n1 = (*v1_0).value.integer as f64;
                                        1
                                    } else {
                                        0
                                    }
                                }) != 0
                                    && (if (*v2).get_tag_variant() == TAG_VARIANT_NUMERIC_NUMBER {
                                        n2 = (*v2).value.number;
                                        1
                                    } else {
                                        if (*v2).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER {
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
                                    (*io_7).set_tag_variant(TAG_VARIANT_NUMERIC_NUMBER);
                                }
                            }
                            continue;
                        }
                        OP_SUBK => {
                            let v1_1: *mut TValue = &mut (*base.offset(
                                (i >> POSITION_B & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let v2_0: *mut TValue = k.offset(
                                (i >> POSITION_C & !(!(0u32) << 8) << 0) as isize,
                            );
                            let ra_21: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            if (*v1_1).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER
                                && (*v2_0).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER
                            {
                                let i1_0: i64 = (*v1_1).value.integer;
                                let i2_0: i64 = (*v2_0).value.integer;
                                program_counter = program_counter.offset(1);
                                let io_8: *mut TValue = &mut (*ra_21).tvalue;
                                (*io_8).value.integer = (i1_0 as u64).wrapping_sub(i2_0 as u64) as i64;
                                (*io_8).set_tag_variant(TAG_VARIANT_NUMERIC_INTEGER);
                            } else {
                                let mut n1_0: f64 = 0.0;
                                let mut n2_0: f64 = 0.0;
                                if (if (*v1_1).get_tag_variant() == TAG_VARIANT_NUMERIC_NUMBER {
                                    n1_0 = (*v1_1).value.number;
                                    1
                                } else {
                                    if (*v1_1).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER {
                                        n1_0 = (*v1_1).value.integer as f64;
                                        1
                                    } else {
                                        0
                                    }
                                }) != 0
                                    && (if (*v2_0).get_tag_variant() == TAG_VARIANT_NUMERIC_NUMBER {
                                        n2_0 = (*v2_0).value.number;
                                        1
                                    } else {
                                        if (*v2_0).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER {
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
                                    (*io_9).set_tag_variant(TAG_VARIANT_NUMERIC_NUMBER);
                                }
                            }
                            continue;
                        }
                        OP_MULK => {
                            let v1_2: *mut TValue = &mut (*base.offset(
                                (i >> POSITION_B & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let v2_1: *mut TValue = k.offset(
                                (i >> POSITION_C & !(!(0u32) << 8) << 0) as isize,
                            );
                            let ra_22: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            if (*v1_2).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER
                                && (*v2_1).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER
                            {
                                let i1_1: i64 = (*v1_2).value.integer;
                                let i2_1: i64 = (*v2_1).value.integer;
                                program_counter = program_counter.offset(1);
                                let io_10: *mut TValue = &mut (*ra_22).tvalue;
                                (*io_10).value.integer = (i1_1 as u64).wrapping_mul(i2_1 as u64) as i64;
                                (*io_10).set_tag_variant(TAG_VARIANT_NUMERIC_INTEGER);
                            } else {
                                let mut n1_1: f64 = 0.0;
                                let mut n2_1: f64 = 0.0;
                                if (if (*v1_2).get_tag_variant() == TAG_VARIANT_NUMERIC_NUMBER {
                                    n1_1 = (*v1_2).value.number;
                                    1
                                } else {
                                    if (*v1_2).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER {
                                        n1_1 = (*v1_2).value.integer as f64;
                                        1
                                    } else {
                                        0
                                    }
                                }) != 0
                                    && (if (*v2_1).get_tag_variant() == TAG_VARIANT_NUMERIC_NUMBER {
                                        n2_1 = (*v2_1).value.number;
                                        1
                                    } else {
                                        if (*v2_1).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER {
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
                                    (*io_11).set_tag_variant(TAG_VARIANT_NUMERIC_NUMBER);
                                }
                            }
                            continue;
                        }
                        OP_MODK => {
                            (*call_info).u.l.saved_program_counter = program_counter;
                            (*interpreter).top.stkidrel_pointer = (*call_info).top.stkidrel_pointer;
                            let v1_3: *mut TValue = &mut (*base.offset(
                                (i >> POSITION_B & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let v2_2: *mut TValue = k.offset(
                                (i >> POSITION_C & !(!(0u32) << 8) << 0) as isize,
                            );
                            let ra_23: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            if (*v1_3).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER
                                && (*v2_2).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER
                            {
                                let i1_2: i64 = (*v1_3).value.integer;
                                let i2_2: i64 = (*v2_2).value.integer;
                                program_counter = program_counter.offset(1);
                                let io_12: *mut TValue = &mut (*ra_23).tvalue;
                                (*io_12).value.integer = luav_mod(interpreter, i1_2, i2_2);
                                (*io_12).set_tag_variant(TAG_VARIANT_NUMERIC_INTEGER);
                            } else {
                                let mut n1_2: f64 = 0.0;
                                let mut n2_2: f64 = 0.0;
                                if (if (*v1_3).get_tag_variant() == TAG_VARIANT_NUMERIC_NUMBER {
                                    n1_2 = (*v1_3).value.number;
                                    1
                                } else {
                                    if (*v1_3).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER {
                                        n1_2 = (*v1_3).value.integer as f64;
                                        1
                                    } else {
                                        0
                                    }
                                }) != 0
                                    && (if (*v2_2).get_tag_variant() == TAG_VARIANT_NUMERIC_NUMBER {
                                        n2_2 = (*v2_2).value.number;
                                        1
                                    } else {
                                        if (*v2_2).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER {
                                            n2_2 = (*v2_2).value.integer as f64;
                                            1
                                        } else {
                                            0
                                        }
                                    }) != 0
                                {
                                    program_counter = program_counter.offset(1);
                                    let io_13: *mut TValue = &mut (*ra_23).tvalue;
                                    (*io_13).value.number = luav_modf(interpreter, n1_2, n2_2);
                                    (*io_13).set_tag_variant(TAG_VARIANT_NUMERIC_NUMBER);
                                }
                            }
                            continue;
                        }
                        OP_POWK => {
                            let ra_24: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let v1_4: *mut TValue = &mut (*base.offset(
                                (i >> POSITION_B & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let v2_3: *mut TValue = k.offset(
                                (i >> POSITION_C & !(!(0u32) << 8) << 0) as isize,
                            );
                            let mut n1_3: f64 = 0.0;
                            let mut n2_3: f64 = 0.0;
                            if (if (*v1_4).get_tag_variant() == TAG_VARIANT_NUMERIC_NUMBER {
                                n1_3 = (*v1_4).value.number;
                                1
                            } else {
                                if (*v1_4).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER {
                                    n1_3 = (*v1_4).value.integer as f64;
                                    1
                                } else {
                                    0
                                }
                            }) != 0
                                && (if (*v2_3).get_tag_variant() == TAG_VARIANT_NUMERIC_NUMBER {
                                    n2_3 = (*v2_3).value.number;
                                    1
                                } else {
                                    if (*v2_3).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER {
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
                                (*io_14).set_tag_variant(TAG_VARIANT_NUMERIC_NUMBER);
                            }
                            continue;
                        }
                        OP_DIVK => {
                            let ra_25: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let v1_5: *mut TValue = &mut (*base.offset(
                                (i >> POSITION_B & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let v2_4: *mut TValue = k.offset(
                                (i >> POSITION_C & !(!(0u32) << 8) << 0) as isize,
                            );
                            let mut n1_4: f64 = 0.0;
                            let mut n2_4: f64 = 0.0;
                            if (if (*v1_5).get_tag_variant() == TAG_VARIANT_NUMERIC_NUMBER {
                                n1_4 = (*v1_5).value.number;
                                1
                            } else {
                                if (*v1_5).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER {
                                    n1_4 = (*v1_5).value.integer as f64;
                                    1
                                } else {
                                    0
                                }
                            }) != 0
                                && (if (*v2_4).get_tag_variant() == TAG_VARIANT_NUMERIC_NUMBER {
                                    n2_4 = (*v2_4).value.number;
                                    1
                                } else {
                                    if (*v2_4).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER {
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
                                (*io_15).set_tag_variant(TAG_VARIANT_NUMERIC_NUMBER);
                            }
                            continue;
                        }
                        OP_IDIVK => {
                            (*call_info).u.l.saved_program_counter = program_counter;
                            (*interpreter).top.stkidrel_pointer = (*call_info).top.stkidrel_pointer;
                            let v1_6: *mut TValue = &mut (*base.offset(
                                (i >> POSITION_B & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let v2_5: *mut TValue = k.offset(
                                (i >> POSITION_C & !(!(0u32) << 8) << 0) as isize,
                            );
                            let ra_26: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            if (*v1_6).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER
                                && (*v2_5).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER
                            {
                                let i1_3: i64 = (*v1_6).value.integer;
                                let i2_3: i64 = (*v2_5).value.integer;
                                program_counter = program_counter.offset(1);
                                let io_16: *mut TValue = &mut (*ra_26).tvalue;
                                (*io_16).value.integer = luav_idiv(interpreter, i1_3, i2_3);
                                (*io_16).set_tag_variant(TAG_VARIANT_NUMERIC_INTEGER);
                            } else {
                                let mut n1_5: f64 = 0.0;
                                let mut n2_5: f64 = 0.0;
                                if (if (*v1_6).get_tag_variant() == TAG_VARIANT_NUMERIC_NUMBER {
                                    n1_5 = (*v1_6).value.number;
                                    1
                                } else {
                                    if (*v1_6).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER {
                                        n1_5 = (*v1_6).value.integer as f64;
                                        1
                                    } else {
                                        0
                                    }
                                }) != 0
                                    && (if (*v2_5).get_tag_variant() == TAG_VARIANT_NUMERIC_NUMBER {
                                        n2_5 = (*v2_5).value.number;
                                        1
                                    } else {
                                        if (*v2_5).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER {
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
                                    (*io_17).set_tag_variant(TAG_VARIANT_NUMERIC_NUMBER);
                                }
                            }
                            continue;
                        }
                        OP_BANDK => {
                            let ra_27: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let v1_7: *mut TValue = &mut (*base.offset(
                                (i >> POSITION_B & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let v2_6: *mut TValue = k.offset(
                                (i >> POSITION_C & !(!(0u32) << 8) << 0) as isize,
                            );
                            let mut i1_4: i64 = 0;
                            let i2_4: i64 = (*v2_6).value.integer;
                            if if (((*v1_7).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER) as i32 != 0)
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
                                (*io_18).set_tag_variant(TAG_VARIANT_NUMERIC_INTEGER);
                            }
                            continue;
                        }
                        OP_BORK => {
                            let ra_28: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let v1_8: *mut TValue = &mut (*base.offset(
                                (i >> POSITION_B & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let v2_7: *mut TValue = k.offset(
                                (i >> POSITION_C & !(!(0u32) << 8) << 0) as isize,
                            );
                            let mut i1_5: i64 = 0;
                            let i2_5: i64 = (*v2_7).value.integer;
                            if if (((*v1_8).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER) as i32 != 0)
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
                                (*io_19).set_tag_variant(TAG_VARIANT_NUMERIC_INTEGER);
                            }
                            continue;
                        }
                        OP_BXORK => {
                            let ra_29: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let v1_9: *mut TValue = &mut (*base.offset(
                                (i >> POSITION_B & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let v2_8: *mut TValue = k.offset(
                                (i >> POSITION_C & !(!(0u32) << 8) << 0) as isize,
                            );
                            let mut i1_6: i64 = 0;
                            let i2_6: i64 = (*v2_8).value.integer;
                            if if (((*v1_9).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER) as i32 != 0)
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
                                (*io_20).set_tag_variant(TAG_VARIANT_NUMERIC_INTEGER);
                            }
                            continue;
                        }
                        32 => {
                            let ra_30: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let rb_8: *mut TValue = &mut (*base.offset(
                                (i >> POSITION_B & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let ic: i32 = (i >> POSITION_C & !(!(0u32) << 8) << 0) as i32
                                - ((1 << 8) - 1 >> 1);
                            let mut ib: i64 = 0;
                            if if (((*rb_8).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER) as i32 != 0)
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
                                (*io_21).set_tag_variant(TAG_VARIANT_NUMERIC_INTEGER);
                            }
                            continue;
                        }
                        33 => {
                            let ra_31: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let rb_9: *mut TValue = &mut (*base.offset(
                                (i >> POSITION_B & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let ic_0: i32 = (i >> POSITION_C & !(!(0u32) << 8) << 0) as i32
                                - ((1 << 8) - 1 >> 1);
                            let mut ib_0: i64 = 0;
                            if if (((*rb_9).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER) as i32 != 0)
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
                                (*io_22).set_tag_variant(TAG_VARIANT_NUMERIC_INTEGER);
                            }
                            continue;
                        }
                        34 => {
                            let v1_10: *mut TValue = &mut (*base.offset(
                                (i >> POSITION_B & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let v2_9: *mut TValue = &mut (*base.offset(
                                (i >> POSITION_C & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let ra_32: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            if (*v1_10).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER
                                && (*v2_9).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER
                            {
                                let i1_7: i64 = (*v1_10).value.integer;
                                let i2_7: i64 = (*v2_9).value.integer;
                                program_counter = program_counter.offset(1);
                                let io_23: *mut TValue = &mut (*ra_32).tvalue;
                                (*io_23).value.integer = (i1_7 as u64).wrapping_add(i2_7 as u64) as i64;
                                (*io_23).set_tag_variant(TAG_VARIANT_NUMERIC_INTEGER);
                            } else {
                                let mut n1_6: f64 = 0.0;
                                let mut n2_6: f64 = 0.0;
                                if (if (*v1_10).get_tag_variant() == TAG_VARIANT_NUMERIC_NUMBER {
                                    n1_6 = (*v1_10).value.number;
                                    1
                                } else {
                                    if (*v1_10).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER {
                                        n1_6 = (*v1_10).value.integer as f64;
                                        1
                                    } else {
                                        0
                                    }
                                }) != 0
                                    && (if (*v2_9).get_tag_variant() == TAG_VARIANT_NUMERIC_NUMBER {
                                        n2_6 = (*v2_9).value.number;
                                        1
                                    } else {
                                        if (*v2_9).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER {
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
                                    (*io_24).set_tag_variant(TAG_VARIANT_NUMERIC_NUMBER);
                                }
                            }
                            continue;
                        }
                        OP_SUB => {
                            let v1_11: *mut TValue = &mut (*base.offset(
                                (i >> POSITION_B & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let v2_10: *mut TValue = &mut (*base.offset(
                                (i >> POSITION_C & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let ra_33: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            if (*v1_11).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER
                                && (*v2_10).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER
                            {
                                let i1_8: i64 = (*v1_11).value.integer;
                                let i2_8: i64 = (*v2_10).value.integer;
                                program_counter = program_counter.offset(1);
                                let io_25: *mut TValue = &mut (*ra_33).tvalue;
                                (*io_25).value.integer = (i1_8 as u64).wrapping_sub(i2_8 as u64) as i64;
                                (*io_25).set_tag_variant(TAG_VARIANT_NUMERIC_INTEGER);
                            } else {
                                let mut n1_7: f64 = 0.0;
                                let mut n2_7: f64 = 0.0;
                                if (if (*v1_11).get_tag_variant() == TAG_VARIANT_NUMERIC_NUMBER {
                                    n1_7 = (*v1_11).value.number;
                                    1
                                } else {
                                    if (*v1_11).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER {
                                        n1_7 = (*v1_11).value.integer as f64;
                                        1
                                    } else {
                                        0
                                    }
                                }) != 0
                                    && (if (*v2_10).get_tag_variant() == TAG_VARIANT_NUMERIC_NUMBER {
                                        n2_7 = (*v2_10).value.number;
                                        1
                                    } else {
                                        if (*v2_10).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER {
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
                                    (*io_26).set_tag_variant(TAG_VARIANT_NUMERIC_NUMBER);
                                }
                            }
                            continue;
                        }
                        OP_MUL => {
                            let v1_12: *mut TValue = &mut (*base.offset(
                                (i >> POSITION_B & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let v2_11: *mut TValue = &mut (*base.offset(
                                (i >> POSITION_C & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let ra_34: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            if (*v1_12).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER
                                && (*v2_11).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER
                            {
                                let i1_9: i64 = (*v1_12).value.integer;
                                let i2_9: i64 = (*v2_11).value.integer;
                                program_counter = program_counter.offset(1);
                                let io_27: *mut TValue = &mut (*ra_34).tvalue;
                                (*io_27).value.integer = (i1_9 as u64).wrapping_mul(i2_9 as u64) as i64;
                                (*io_27).set_tag_variant(TAG_VARIANT_NUMERIC_INTEGER);
                            } else {
                                let mut n1_8: f64 = 0.0;
                                let mut n2_8: f64 = 0.0;
                                if (if (*v1_12).get_tag_variant() == TAG_VARIANT_NUMERIC_NUMBER {
                                    n1_8 = (*v1_12).value.number;
                                    1
                                } else {
                                    if (*v1_12).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER {
                                        n1_8 = (*v1_12).value.integer as f64;
                                        1
                                    } else {
                                        0
                                    }
                                }) != 0
                                    && (if (*v2_11).get_tag_variant() == TAG_VARIANT_NUMERIC_NUMBER {
                                        n2_8 = (*v2_11).value.number;
                                        1
                                    } else {
                                        if (*v2_11).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER {
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
                                    (*io_28).set_tag_variant(TAG_VARIANT_NUMERIC_NUMBER);
                                }
                            }
                            continue;
                        }
                        OP_MOD => {
                            (*call_info).u.l.saved_program_counter = program_counter;
                            (*interpreter).top.stkidrel_pointer = (*call_info).top.stkidrel_pointer;
                            let v1_13: *mut TValue = &mut (*base.offset(
                                (i >> POSITION_B & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let v2_12: *mut TValue = &mut (*base.offset(
                                (i >> POSITION_C & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let ra_35: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            if (*v1_13).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER
                                && (*v2_12).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER
                            {
                                let i1_10: i64 = (*v1_13).value.integer;
                                let i2_10: i64 = (*v2_12).value.integer;
                                program_counter = program_counter.offset(1);
                                let io_29: *mut TValue = &mut (*ra_35).tvalue;
                                (*io_29).value.integer = luav_mod(interpreter, i1_10, i2_10);
                                (*io_29).set_tag_variant(TAG_VARIANT_NUMERIC_INTEGER);
                            } else {
                                let mut n1_9: f64 = 0.0;
                                let mut n2_9: f64 = 0.0;
                                if (if (*v1_13).get_tag_variant() == TAG_VARIANT_NUMERIC_NUMBER {
                                    n1_9 = (*v1_13).value.number;
                                    1
                                } else {
                                    if (*v1_13).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER {
                                        n1_9 = (*v1_13).value.integer as f64;
                                        1
                                    } else {
                                        0
                                    }
                                }) != 0
                                    && (if (*v2_12).get_tag_variant() == TAG_VARIANT_NUMERIC_NUMBER {
                                        n2_9 = (*v2_12).value.number;
                                        1
                                    } else {
                                        if (*v2_12).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER {
                                            n2_9 = (*v2_12).value.integer as f64;
                                            1
                                        } else {
                                            0
                                        }
                                    }) != 0
                                {
                                    program_counter = program_counter.offset(1);
                                    let io_30: *mut TValue = &mut (*ra_35).tvalue;
                                    (*io_30).value.number = luav_modf(interpreter, n1_9, n2_9);
                                    (*io_30).set_tag_variant(TAG_VARIANT_NUMERIC_NUMBER);
                                }
                            }
                            continue;
                        }
                        OP_POW => {
                            let ra_36: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let v1_14: *mut TValue = &mut (*base.offset(
                                (i >> POSITION_B & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let v2_13: *mut TValue = &mut (*base.offset(
                                (i >> POSITION_C & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let mut n1_10: f64 = 0.0;
                            let mut n2_10: f64 = 0.0;
                            if (if (*v1_14).get_tag_variant() == TAG_VARIANT_NUMERIC_NUMBER {
                                n1_10 = (*v1_14).value.number;
                                1
                            } else {
                                if (*v1_14).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER {
                                    n1_10 = (*v1_14).value.integer as f64;
                                    1
                                } else {
                                    0
                                }
                            }) != 0
                                && (if (*v2_13).get_tag_variant() == TAG_VARIANT_NUMERIC_NUMBER {
                                    n2_10 = (*v2_13).value.number;
                                    1
                                } else {
                                    if (*v2_13).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER {
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
                                (*io_31).set_tag_variant(TAG_VARIANT_NUMERIC_NUMBER);
                            }
                            continue;
                        }
                        OP_DIV => {
                            let ra_37: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let v1_15: *mut TValue = &mut (*base.offset(
                                (i >> POSITION_B & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let v2_14: *mut TValue = &mut (*base.offset(
                                (i >> POSITION_C & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let mut n1_11: f64 = 0.0;
                            let mut n2_11: f64 = 0.0;
                            if (if (*v1_15).get_tag_variant() == TAG_VARIANT_NUMERIC_NUMBER {
                                n1_11 = (*v1_15).value.number;
                                1
                            } else {
                                if (*v1_15).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER {
                                    n1_11 = (*v1_15).value.integer as f64;
                                    1
                                } else {
                                    0
                                }
                            }) != 0
                                && (if (*v2_14).get_tag_variant() == TAG_VARIANT_NUMERIC_NUMBER {
                                    n2_11 = (*v2_14).value.number;
                                    1
                                } else {
                                    if (*v2_14).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER {
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
                                (*io_32).set_tag_variant(TAG_VARIANT_NUMERIC_NUMBER);
                            }
                            continue;
                        }
                        OP_IDIV => {
                            (*call_info).u.l.saved_program_counter = program_counter;
                            (*interpreter).top.stkidrel_pointer = (*call_info).top.stkidrel_pointer;
                            let v1_16: *mut TValue = &mut (*base.offset(
                                (i >> POSITION_B & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let v2_15: *mut TValue = &mut (*base.offset(
                                (i >> POSITION_C & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let ra_38: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            if (*v1_16).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER
                                && (*v2_15).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER
                            {
                                let i1_11: i64 = (*v1_16).value.integer;
                                let i2_11: i64 = (*v2_15).value.integer;
                                program_counter = program_counter.offset(1);
                                let io_33: *mut TValue = &mut (*ra_38).tvalue;
                                (*io_33).value.integer = luav_idiv(interpreter, i1_11, i2_11);
                                (*io_33).set_tag_variant(TAG_VARIANT_NUMERIC_INTEGER);
                            } else {
                                let mut n1_12: f64 = 0.0;
                                let mut n2_12: f64 = 0.0;
                                if (if (*v1_16).get_tag_variant() == TAG_VARIANT_NUMERIC_NUMBER {
                                    n1_12 = (*v1_16).value.number;
                                    1
                                } else {
                                    if (*v1_16).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER {
                                        n1_12 = (*v1_16).value.integer as f64;
                                        1
                                    } else {
                                        0
                                    }
                                }) != 0
                                    && (if (*v2_15).get_tag_variant() == TAG_VARIANT_NUMERIC_NUMBER {
                                        n2_12 = (*v2_15).value.number;
                                        1
                                    } else {
                                        if (*v2_15).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER {
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
                                    (*io_34).set_tag_variant(TAG_VARIANT_NUMERIC_NUMBER);
                                }
                            }
                            continue;
                        }
                        OP_BAND => {
                            let ra_39: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let v1_17: *mut TValue = &mut (*base.offset(
                                (i >> POSITION_B & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let v2_16: *mut TValue = &mut (*base.offset(
                                (i >> POSITION_C & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let mut i1_12: i64 = 0;
                            let mut i2_12: i64 = 0;
                            if (if (((*v1_17).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER) as i32 != 0)
                                as i64
                                != 0
                            {
                                i1_12 = (*v1_17).value.integer;
                                1
                            } else {
                                luav_tointegerns(v1_17, &mut i1_12, F2I::Equal)
                            }) != 0
                                && (if (((*v2_16).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER) as i32
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
                                (*io_35).set_tag_variant(TAG_VARIANT_NUMERIC_INTEGER);
                            }
                            continue;
                        }
                        OP_BOR => {
                            let ra_40: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let v1_18: *mut TValue = &mut (*base.offset(
                                (i >> POSITION_B & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let v2_17: *mut TValue = &mut (*base.offset(
                                (i >> POSITION_C & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let mut i1_13: i64 = 0;
                            let mut i2_13: i64 = 0;
                            if (if (((*v1_18).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER) as i32 != 0)
                                as i64
                                != 0
                            {
                                i1_13 = (*v1_18).value.integer;
                                1
                            } else {
                                luav_tointegerns(v1_18, &mut i1_13, F2I::Equal)
                            }) != 0
                                && (if (((*v2_17).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER) as i32
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
                                (*io_36).set_tag_variant(TAG_VARIANT_NUMERIC_INTEGER);
                            }
                            continue;
                        }
                        OP_BXOR => {
                            let ra_41: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let v1_19: *mut TValue = &mut (*base.offset(
                                (i >> POSITION_B & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let v2_18: *mut TValue = &mut (*base.offset(
                                (i >> POSITION_C & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let mut i1_14: i64 = 0;
                            let mut i2_14: i64 = 0;
                            if (if (((*v1_19).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER) as i32 != 0)
                                as i64
                                != 0
                            {
                                i1_14 = (*v1_19).value.integer;
                                1
                            } else {
                                luav_tointegerns(v1_19, &mut i1_14, F2I::Equal)
                            }) != 0
                                && (if (((*v2_18).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER) as i32
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
                                (*io_37).set_tag_variant(TAG_VARIANT_NUMERIC_INTEGER);
                            }
                            continue;
                        }
                        OP_SHR => {
                            let ra_42: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let v1_20: *mut TValue = &mut (*base.offset(
                                (i >> POSITION_B & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let v2_19: *mut TValue = &mut (*base.offset(
                                (i >> POSITION_C & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let mut i1_15: i64 = 0;
                            let mut i2_15: i64 = 0;
                            if (if (((*v1_20).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER) as i32 != 0)
                                as i64
                                != 0
                            {
                                i1_15 = (*v1_20).value.integer;
                                1
                            } else {
                                luav_tointegerns(v1_20, &mut i1_15, F2I::Equal)
                            }) != 0
                                && (if (((*v2_19).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER) as i32
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
                                (*io_38).set_tag_variant(TAG_VARIANT_NUMERIC_INTEGER);
                            }
                            continue;
                        }
                        OP_SHL => {
                            let ra_43: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let v1_21: *mut TValue = &mut (*base.offset(
                                (i >> POSITION_B & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let v2_20: *mut TValue = &mut (*base.offset(
                                (i >> POSITION_C & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let mut i1_16: i64 = 0;
                            let mut i2_16: i64 = 0;
                            if (if (((*v1_21).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER) as i32 != 0)
                                as i64
                                != 0
                            {
                                i1_16 = (*v1_21).value.integer;
                                1
                            } else {
                                luav_tointegerns(v1_21, &mut i1_16, F2I::Equal)
                            }) != 0
                                && (if (((*v2_20).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER) as i32
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
                                (*io_39).set_tag_variant(TAG_VARIANT_NUMERIC_INTEGER);
                            }
                            continue;
                        }
                        46 => {
                            let ra_44: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let pi: u32 = *program_counter.offset(-(2 as isize));
                            let rb_10: *mut TValue = &mut (*base.offset(
                                (i >> POSITION_B & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let tm: u32 =
                                (i >> POSITION_C & !(!(0u32) << 8) << 0) as u32;
                            let result: StackValuePointer =
                                base.offset((pi >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            (*call_info).u.l.saved_program_counter = program_counter;
                            (*interpreter).top.stkidrel_pointer = (*call_info).top.stkidrel_pointer;
                            luat_trybintm(interpreter, &mut (*ra_44).tvalue, rb_10, result, tm);
                            trap = (*call_info).u.l.trap;
                            continue;
                        }
                        47 => {
                            let ra_45: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let pi_0: u32 = *program_counter.offset(-(2 as isize));
                            let imm_0: i32 = (i >> POSITION_B & !(!(0u32) << 8) << 0) as i32
                                - ((1 << 8) - 1 >> 1);
                            let tm_0: u32 =
                                (i >> POSITION_C & !(!(0u32) << 8) << 0) as u32;
                            let flip: i32 = (i >> POSITION_K & !(!(0u32) << 1) << 0) as i32;
                            let result_0: StackValuePointer =
                                base.offset((pi_0 >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            (*call_info).u.l.saved_program_counter = program_counter;
                            (*interpreter).top.stkidrel_pointer = (*call_info).top.stkidrel_pointer;
                            luat_trybinitm(
                                interpreter,
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
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let pi_1: u32 = *program_counter.offset(-(2 as isize));
                            let imm_1: *mut TValue = k.offset(
                                (i >> POSITION_B & !(!(0u32) << 8) << 0) as isize,
                            );
                            let tm_1: u32 =
                                (i >> POSITION_C & !(!(0u32) << 8) << 0) as u32;
                            let flip_0: i32 = (i >> POSITION_K & !(!(0u32) << 1) << 0) as i32;
                            let result_1: StackValuePointer =
                                base.offset((pi_1 >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            (*call_info).u.l.saved_program_counter = program_counter;
                            (*interpreter).top.stkidrel_pointer = (*call_info).top.stkidrel_pointer;
                            luat_trybinassoctm(
                                interpreter,
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
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let rb_11: *mut TValue = &mut (*base.offset(
                                (i >> POSITION_B & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let mut nb_0: f64 = 0.0;
                            if (*rb_11).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER {
                                let ib_1: i64 = (*rb_11).value.integer;
                                let io_40: *mut TValue = &mut (*ra_47).tvalue;
                                (*io_40).value.integer = (0u64).wrapping_sub(ib_1 as u64) as i64;
                                (*io_40).set_tag_variant(TAG_VARIANT_NUMERIC_INTEGER);
                            } else if if (*rb_11).get_tag_variant() == TAG_VARIANT_NUMERIC_NUMBER {
                                nb_0 = (*rb_11).value.number;
                                1
                            } else if (*rb_11).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER {
                                nb_0 = (*rb_11).value.integer as f64;
                                1
                            } else {
                                0
                            } != 0
                            {
                                let io_41: *mut TValue = &mut (*ra_47).tvalue;
                                (*io_41).value.number = -nb_0;
                                (*io_41).set_tag_variant(TAG_VARIANT_NUMERIC_NUMBER);
                            } else {
                                (*call_info).u.l.saved_program_counter = program_counter;
                                (*interpreter).top.stkidrel_pointer = (*call_info).top.stkidrel_pointer;
                                luat_trybintm(interpreter, rb_11, rb_11, ra_47, TM_UNM);
                                trap = (*call_info).u.l.trap;
                            }
                            continue;
                        }
                        50 => {
                            let ra_48: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let rb_12: *mut TValue = &mut (*base.offset(
                                (i >> POSITION_B & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            let mut ib_2: i64 = 0;
                            if if (((*rb_12).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER) as i32 != 0)
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
                                (*io_42).set_tag_variant(TAG_VARIANT_NUMERIC_INTEGER);
                            } else {
                                (*call_info).u.l.saved_program_counter = program_counter;
                                (*interpreter).top.stkidrel_pointer = (*call_info).top.stkidrel_pointer;
                                luat_trybintm(interpreter, rb_12, rb_12, ra_48, TM_BNOT);
                                trap = (*call_info).u.l.trap;
                            }
                            continue;
                        }
                        51 => {
                            let ra_49: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let rb_13: *mut TValue = &mut (*base.offset(
                                (i >> POSITION_B & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            if (*rb_13).get_tag_variant() == TAG_VARIANT_BOOLEAN_FALSE
                                || (*rb_13).is_tagtype_nil()
                            {
                                (*ra_49).tvalue.set_tag_variant(TAG_VARIANT_BOOLEAN_TRUE);
                            } else {
                                (*ra_49).tvalue.set_tag_variant(TAG_VARIANT_BOOLEAN_FALSE);
                            }
                            continue;
                        }
                        OP_LEN => {
                            let ra_50: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            (*call_info).u.l.saved_program_counter = program_counter;
                            (*interpreter).top.stkidrel_pointer = (*call_info).top.stkidrel_pointer;
                            luav_objlen(
                                interpreter,
                                ra_50,
                                &mut (*base.offset(
                                    (i >> POSITION_B & !(!(0u32) << 8) << 0) as isize,
                                ))
                                .tvalue,
                            );
                            trap = (*call_info).u.l.trap;
                            continue;
                        }
                        53 => {
                            let ra_51: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let n_1: i32 = (i >> POSITION_B & !(!(0u32) << 8) << 0) as i32;
                            (*interpreter).top.stkidrel_pointer = ra_51.offset(n_1 as isize);
                            (*call_info).u.l.saved_program_counter = program_counter;
                            concatenate(interpreter, n_1);
                            trap = (*call_info).u.l.trap;
                            if (*(*interpreter).global).gc_debt > 0 {
                                (*call_info).u.l.saved_program_counter = program_counter;
                                (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer;
                                luac_step(interpreter);
                                trap = (*call_info).u.l.trap;
                            }
                            continue;
                        }
                        54 => {
                            let ra_52: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            (*call_info).u.l.saved_program_counter = program_counter;
                            (*interpreter).top.stkidrel_pointer = (*call_info).top.stkidrel_pointer;
                            luaf_close(interpreter, ra_52, 0, 1);
                            trap = (*call_info).u.l.trap;
                            continue;
                        }
                        55 => {
                            let ra_53: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            (*call_info).u.l.saved_program_counter = program_counter;
                            (*interpreter).top.stkidrel_pointer = (*call_info).top.stkidrel_pointer;
                            luaf_newtbcupval(interpreter, ra_53);
                            continue;
                        }
                        56 => {
                            program_counter = program_counter.offset(
                                ((i >> POSITION_A & !(!(0u32) << 8 + 8 + 1 + 8) << 0) as i32
                                    - ((1 << 8 + 8 + 1 + 8) - 1 >> 1)
                                    + 0) as isize,
                            );
                            trap = (*call_info).u.l.trap;
                            continue;
                        }
                        57 => {
                            let ra_54: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let cond_0: i32;
                            let rb_14: *mut TValue = &mut (*base.offset(
                                (i >> POSITION_B & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            (*call_info).u.l.saved_program_counter = program_counter;
                            (*interpreter).top.stkidrel_pointer = (*call_info).top.stkidrel_pointer;
                            cond_0 = if luav_equalobj(interpreter, &mut (*ra_54).tvalue, rb_14) { 1 } else { 0 };
                            trap = (*call_info).u.l.trap;
                            if cond_0 != (i >> POSITION_K & !(!(0u32) << 1) << 0) as i32 {
                                program_counter = program_counter.offset(1);
                            } else {
                                let ni: u32 = *program_counter;
                                program_counter = program_counter.offset(
                                    ((ni >> POSITION_A & !(!(0u32) << 8 + 8 + 1 + 8) << 0) as i32
                                        - ((1 << 8 + 8 + 1 + 8) - 1 >> 1)
                                        + 1) as isize,
                                );
                                trap = (*call_info).u.l.trap;
                            }
                            continue;
                        }
                        58 => {
                            let ra_55: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let cond_1: i32;
                            let rb_15: *mut TValue = &mut (*base.offset(
                                (i >> POSITION_B & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            if (*ra_55).tvalue.get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER
                                && (*rb_15).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER
                            {
                                let ia: i64 = (*ra_55).tvalue.value.integer;
                                let ib_3: i64 = (*rb_15).value.integer;
                                cond_1 = (ia < ib_3) as i32;
                            } else if (*ra_55).tvalue.is_tagtype_numeric() && (*rb_15).is_tagtype_numeric() {
                                cond_1 = if ltnum(&mut (*ra_55).tvalue, rb_15) { 1 } else { 0 };
                            } else {
                                (*call_info).u.l.saved_program_counter = program_counter;
                                (*interpreter).top.stkidrel_pointer = (*call_info).top.stkidrel_pointer;
                                cond_1 = lessthanothers(interpreter, &mut (*ra_55).tvalue, rb_15);
                                trap = (*call_info).u.l.trap;
                            }
                            if cond_1 != (i >> POSITION_K & !(!(0u32) << 1) << 0) as i32 {
                                program_counter = program_counter.offset(1);
                            } else {
                                let ni_0: u32 = *program_counter;
                                program_counter = program_counter.offset(
                                    ((ni_0 >> POSITION_A & !(!(0u32) << 8 + 8 + 1 + 8) << 0) as i32
                                        - ((1 << 8 + 8 + 1 + 8) - 1 >> 1)
                                        + 1) as isize,
                                );
                                trap = (*call_info).u.l.trap;
                            }
                            continue;
                        }
                        OP_LE => {
                            let ra_56: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let cond_2: i32;
                            let rb_16: *mut TValue = &mut (*base.offset(
                                (i >> POSITION_B & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            if (*ra_56).tvalue.get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER
                                && (*rb_16).get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER
                            {
                                let ia_0: i64 = (*ra_56).tvalue.value.integer;
                                let ib_4: i64 = (*rb_16).value.integer;
                                cond_2 = (ia_0 <= ib_4) as i32;
                            } else if (*ra_56).tvalue.is_tagtype_numeric() && (*rb_16).is_tagtype_numeric() {
                                cond_2 = if lenum(&mut (*ra_56).tvalue, rb_16) { 1 } else { 0 };
                            } else {
                                (*call_info).u.l.saved_program_counter = program_counter;
                                (*interpreter).top.stkidrel_pointer = (*call_info).top.stkidrel_pointer;
                                cond_2 = if lessequalothers(interpreter, &mut (*ra_56).tvalue, rb_16) { 1 } else { 0 };
                                trap = (*call_info).u.l.trap;
                            }
                            if cond_2 != (i >> POSITION_K & !(!(0u32) << 1) << 0) as i32 {
                                program_counter = program_counter.offset(1);
                            } else {
                                let ni_1: u32 = *program_counter;
                                program_counter = program_counter.offset(
                                    ((ni_1 >> POSITION_A & !(!(0u32) << 8 + 8 + 1 + 8) << 0) as i32
                                        - ((1 << 8 + 8 + 1 + 8) - 1 >> 1)
                                        + 1) as isize,
                                );
                                trap = (*call_info).u.l.trap;
                            }
                            continue;
                        }
                        60 => {
                            let ra_57: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let rb_17: *mut TValue = k.offset(
                                (i >> POSITION_B & !(!(0u32) << 8) << 0) as isize,
                            );
                            let cond_3: i32 =
                                if luav_equalobj(std::ptr::null_mut(), &mut (*ra_57).tvalue, rb_17) { 1 } else { 0 };
                            if cond_3 != (i >> POSITION_K & !(!(0u32) << 1) << 0) as i32 {
                                program_counter = program_counter.offset(1);
                            } else {
                                let ni_2: u32 = *program_counter;
                                program_counter = program_counter.offset(
                                    ((ni_2 >> POSITION_A & !(!(0u32) << 8 + 8 + 1 + 8) << 0) as i32
                                        - ((1 << 8 + 8 + 1 + 8) - 1 >> 1)
                                        + 1) as isize,
                                );
                                trap = (*call_info).u.l.trap;
                            }
                            continue;
                        }
                        61 => {
                            let ra_58: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let cond_4: i32;
                            let im: i32 = (i >> POSITION_B & !(!(0u32) << 8) << 0) as i32
                                - ((1 << 8) - 1 >> 1);
                            if (*ra_58).tvalue.get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER {
                                cond_4 = ((*ra_58).tvalue.value.integer == im as i64) as i32;
                            } else if (*ra_58).tvalue.get_tag_variant() == TAG_VARIANT_NUMERIC_NUMBER {
                                cond_4 = ((*ra_58).tvalue.value.number == im as f64) as i32;
                            } else {
                                cond_4 = 0;
                            }
                            if cond_4 != (i >> POSITION_K & !(!(0u32) << 1) << 0) as i32 {
                                program_counter = program_counter.offset(1);
                            } else {
                                let ni_3: u32 = *program_counter;
                                program_counter = program_counter.offset(
                                    ((ni_3 >> POSITION_A & !(!(0u32) << 8 + 8 + 1 + 8) << 0) as i32
                                        - ((1 << 8 + 8 + 1 + 8) - 1 >> 1)
                                        + 1) as isize,
                                );
                                trap = (*call_info).u.l.trap;
                            }
                            continue;
                        }
                        62 => {
                            let ra_59: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let cond_5: i32;
                            let im_0: i32 = (i >> POSITION_B & !(!(0u32) << 8) << 0) as i32
                                - ((1 << 8) - 1 >> 1);
                            if (*ra_59).tvalue.get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER {
                                cond_5 = ((*ra_59).tvalue.value.integer < im_0 as i64) as i32;
                            } else if (*ra_59).tvalue.get_tag_variant() == TAG_VARIANT_NUMERIC_NUMBER {
                                let fa: f64 = (*ra_59).tvalue.value.number;
                                let fim: f64 = im_0 as f64;
                                cond_5 = (fa < fim) as i32;
                            } else {
                                let isf: bool =
                                    (i >> POSITION_C & !(!(0u32) << 8) << 0) != 0;
                                (*call_info).u.l.saved_program_counter = program_counter;
                                (*interpreter).top.stkidrel_pointer = (*call_info).top.stkidrel_pointer;
                                cond_5 = luat_callorderitm(
                                    interpreter,
                                    &mut (*ra_59).tvalue,
                                    im_0,
                                    0,
                                    isf,
                                    TM_LT,
                                );
                                trap = (*call_info).u.l.trap;
                            }
                            if cond_5 != (i >> POSITION_K & !(!(0u32) << 1) << 0) as i32 {
                                program_counter = program_counter.offset(1);
                            } else {
                                let ni_4: u32 = *program_counter;
                                program_counter = program_counter.offset(
                                    ((ni_4 >> POSITION_A & !(!(0u32) << 8 + 8 + 1 + 8) << 0) as i32
                                        - ((1 << 8 + 8 + 1 + 8) - 1 >> 1)
                                        + 1) as isize,
                                );
                                trap = (*call_info).u.l.trap;
                            }
                            continue;
                        }
                        OP_LEI => {
                            let ra_60: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let cond_6: i32;
                            let im_1: i32 = (i >> POSITION_B & !(!(0u32) << 8) << 0) as i32
                                - ((1 << 8) - 1 >> 1);
                            if (*ra_60).tvalue.get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER {
                                cond_6 = ((*ra_60).tvalue.value.integer <= im_1 as i64) as i32;
                            } else if (*ra_60).tvalue.get_tag_variant() == TAG_VARIANT_NUMERIC_NUMBER {
                                let fa_0: f64 = (*ra_60).tvalue.value.number;
                                let fim_0: f64 = im_1 as f64;
                                cond_6 = (fa_0 <= fim_0) as i32;
                            } else {
                                let isf_0: bool =
                                    (i >> POSITION_C & !(!(0u32) << 8) << 0) != 0;
                                (*call_info).u.l.saved_program_counter = program_counter;
                                (*interpreter).top.stkidrel_pointer = (*call_info).top.stkidrel_pointer;
                                cond_6 = luat_callorderitm(
                                    interpreter,
                                    &mut (*ra_60).tvalue,
                                    im_1,
                                    0,
                                    isf_0,
                                    TM_LE,
                                );
                                trap = (*call_info).u.l.trap;
                            }
                            if cond_6 != (i >> POSITION_K & !(!(0u32) << 1) << 0) as i32 {
                                program_counter = program_counter.offset(1);
                            } else {
                                let ni_5: u32 = *program_counter;
                                program_counter = program_counter.offset(
                                    ((ni_5 >> POSITION_A & !(!(0u32) << 8 + 8 + 1 + 8) << 0) as i32
                                        - ((1 << 8 + 8 + 1 + 8) - 1 >> 1)
                                        + 1) as isize,
                                );
                                trap = (*call_info).u.l.trap;
                            }
                            continue;
                        }
                        64 => {
                            let ra_61: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let cond_7: i32;
                            let im_2: i32 = (i >> POSITION_B & !(!(0u32) << 8) << 0) as i32
                                - ((1 << 8) - 1 >> 1);
                            if (*ra_61).tvalue.get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER {
                                cond_7 = ((*ra_61).tvalue.value.integer > im_2 as i64) as i32;
                            } else if (*ra_61).tvalue.get_tag_variant() == TAG_VARIANT_NUMERIC_NUMBER {
                                let fa_1: f64 = (*ra_61).tvalue.value.number;
                                let fim_1: f64 = im_2 as f64;
                                cond_7 = (fa_1 > fim_1) as i32;
                            } else {
                                let isf_1: bool =
                                    (i >> POSITION_C & !(!(0u32) << 8) << 0) != 0;
                                (*call_info).u.l.saved_program_counter = program_counter;
                                (*interpreter).top.stkidrel_pointer = (*call_info).top.stkidrel_pointer;
                                cond_7 = luat_callorderitm(
                                    interpreter,
                                    &mut (*ra_61).tvalue,
                                    im_2,
                                    1,
                                    isf_1,
                                    TM_LT,
                                );
                                trap = (*call_info).u.l.trap;
                            }
                            if cond_7 != (i >> POSITION_K & !(!(0u32) << 1) << 0) as i32 {
                                program_counter = program_counter.offset(1);
                            } else {
                                let ni_6: u32 = *program_counter;
                                program_counter = program_counter.offset(
                                    ((ni_6 >> POSITION_A & !(!(0u32) << 8 + 8 + 1 + 8) << 0) as i32
                                        - ((1 << 8 + 8 + 1 + 8) - 1 >> 1)
                                        + 1) as isize,
                                );
                                trap = (*call_info).u.l.trap;
                            }
                            continue;
                        }
                        OP_GEI => {
                            let ra_62: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let cond_8: i32;
                            let im_3: i32 = (i >> POSITION_B & !(!(0u32) << 8) << 0) as i32
                                - ((1 << 8) - 1 >> 1);
                            if (*ra_62).tvalue.get_tag_variant() == TAG_VARIANT_NUMERIC_INTEGER {
                                cond_8 = ((*ra_62).tvalue.value.integer >= im_3 as i64) as i32;
                            } else if (*ra_62).tvalue.get_tag_variant() == TAG_VARIANT_NUMERIC_NUMBER {
                                let fa_2: f64 = (*ra_62).tvalue.value.number;
                                let fim_2: f64 = im_3 as f64;
                                cond_8 = (fa_2 >= fim_2) as i32;
                            } else {
                                let isf_2: bool =
                                    (i >> POSITION_C & !(!(0u32) << 8) << 0) != 0;
                                (*call_info).u.l.saved_program_counter = program_counter;
                                (*interpreter).top.stkidrel_pointer = (*call_info).top.stkidrel_pointer;
                                cond_8 = luat_callorderitm(
                                    interpreter,
                                    &mut (*ra_62).tvalue,
                                    im_3,
                                    1,
                                    isf_2,
                                    TM_LE,
                                );
                                trap = (*call_info).u.l.trap;
                            }
                            if cond_8 != (i >> POSITION_K & !(!(0u32) << 1) << 0) as i32 {
                                program_counter = program_counter.offset(1);
                            } else {
                                let ni_7: u32 = *program_counter;
                                program_counter = program_counter.offset(
                                    ((ni_7 >> POSITION_A & !(!(0u32) << 8 + 8 + 1 + 8) << 0) as i32
                                        - ((1 << 8 + 8 + 1 + 8) - 1 >> 1)
                                        + 1) as isize,
                                );
                                trap = (*call_info).u.l.trap;
                            }
                            continue;
                        }
                        66 => {
                            let ra_63: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let cond_9: i32 = !((*ra_63).tvalue.get_tag_variant()
                                == TAG_VARIANT_BOOLEAN_FALSE
                                || (*ra_63).tvalue.is_tagtype_nil())
                                as i32;
                            if cond_9 != (i >> POSITION_K & !(!(0u32) << 1) << 0) as i32 {
                                program_counter = program_counter.offset(1);
                            } else {
                                let ni_8: u32 = *program_counter;
                                program_counter = program_counter.offset(
                                    ((ni_8 >> POSITION_A & !(!(0u32) << 8 + 8 + 1 + 8) << 0) as i32
                                        - ((1 << 8 + 8 + 1 + 8) - 1 >> 1)
                                        + 1) as isize,
                                );
                                trap = (*call_info).u.l.trap;
                            }
                            continue;
                        }
                        67 => {
                            let ra_64: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let rb_18: *mut TValue = &mut (*base.offset(
                                (i >> POSITION_B & !(!(0u32) << 8) << 0) as isize,
                            ))
                            .tvalue;
                            if ((*rb_18).get_tag_variant() == TAG_VARIANT_BOOLEAN_FALSE
                                || (*rb_18).is_tagtype_nil())
                                as i32
                                == (i >> POSITION_K & !(!(0u32) << 1) << 0) as i32
                            {
                                program_counter = program_counter.offset(1);
                            } else {
                                let io1_14: *mut TValue = &mut (*ra_64).tvalue;
                                let io2_14: *const TValue = rb_18;
                                (*io1_14).copy_from(&(*io2_14));
                                let ni_9: u32 = *program_counter;
                                program_counter = program_counter.offset(
                                    ((ni_9 >> POSITION_A & !(!(0u32) << 8 + 8 + 1 + 8) << 0) as i32
                                        - ((1 << 8 + 8 + 1 + 8) - 1 >> 1)
                                        + 1) as isize,
                                );
                                trap = (*call_info).u.l.trap;
                            }
                            continue;
                        }
                        68 => {
                            ra_65 =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            b_4 = (i >> POSITION_B & !(!(0u32) << 8) << 0) as i32;
                            count_results =
                                (i >> POSITION_C & !(!(0u32) << 8) << 0) as i32 - 1;
                            if b_4 != 0 {
                                (*interpreter).top.stkidrel_pointer = ra_65.offset(b_4 as isize);
                            }
                            (*call_info).u.l.saved_program_counter = program_counter;
                            new_call_info = luad_precall(interpreter, ra_65, count_results);
                            if !new_call_info.is_null() {
                                break '_returning;
                            }
                            trap = (*call_info).u.l.trap;
                            continue;
                        }
                        69 => {
                            let ra_66: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let mut b_5: i32 = (i >> POSITION_B & !(!(0u32) << 8) << 0) as i32;
                            let n_2: i32;
                            let nparams1: i32 =
                                (i >> POSITION_C & !(!(0u32) << 8) << 0) as i32;
                            let delta: i32 = if nparams1 != 0 {
                                (*call_info).u.l.count_extra_arguments + nparams1
                            } else {
                                0
                            };
                            if b_5 != 0 {
                                (*interpreter).top.stkidrel_pointer = ra_66.offset(b_5 as isize);
                            } else {
                                b_5 = ((*interpreter).top.stkidrel_pointer).offset_from(ra_66) as i32;
                            }
                            (*call_info).u.l.saved_program_counter = program_counter;
                            if (i & (1 as u32) << POSITION_K) as i32 != 0 {
                                luaf_closeupval(interpreter, base);
                            }
                            n_2 = luad_pretailcall(interpreter, call_info, ra_66, b_5, delta);
                            if n_2 < 0 {
                                continue '_startfunc;
                            }
                            (*call_info).function.stkidrel_pointer =
                                ((*call_info).function.stkidrel_pointer).offset(-(delta as isize));
                            luad_poscall(interpreter, call_info, n_2);
                            trap = (*call_info).u.l.trap;
                            break;
                        }
                        70 => {
                            let mut ra_67: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let mut n_3: i32 =
                                (i >> POSITION_B & !(!(0u32) << 8) << 0) as i32 - 1;
                            let nparams1_0: i32 =
                                (i >> POSITION_C & !(!(0u32) << 8) << 0) as i32;
                            if n_3 < 0 {
                                n_3 = ((*interpreter).top.stkidrel_pointer).offset_from(ra_67) as i32;
                            }
                            (*call_info).u.l.saved_program_counter = program_counter;
                            if (i & (1 as u32) << POSITION_K) as i32 != 0 {
                                (*call_info).u2.nres = n_3;
                                if (*interpreter).top.stkidrel_pointer < (*call_info).top.stkidrel_pointer {
                                    (*interpreter).top.stkidrel_pointer = (*call_info).top.stkidrel_pointer;
                                }
                                luaf_close(interpreter, base, -1, 1);
                                trap = (*call_info).u.l.trap;
                                if (trap != 0) as i64 != 0 {
                                    base = ((*call_info).function.stkidrel_pointer).offset(1 as isize);
                                    ra_67 = base.offset(
                                        (i >> POSITION_A & !(!(0u32) << 8) << 0) as isize,
                                    );
                                }
                            }
                            if nparams1_0 != 0 {
                                (*call_info).function.stkidrel_pointer = ((*call_info).function.stkidrel_pointer).offset(
                                    -(((*call_info).u.l.count_extra_arguments + nparams1_0)
                                        as isize),
                                );
                            }
                            (*interpreter).top.stkidrel_pointer = ra_67.offset(n_3 as isize);
                            luad_poscall(interpreter, call_info, n_3);
                            trap = (*call_info).u.l.trap;
                            break;
                        }
                        71 => {
                            if ((*interpreter).hook_mask != 0) as i64 != 0 {
                                let ra_68: StackValuePointer = base
                                    .offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                                (*interpreter).top.stkidrel_pointer = ra_68;
                                (*call_info).u.l.saved_program_counter = program_counter;
                                luad_poscall(interpreter, call_info, 0);
                                trap = 1;
                            } else {
                                let mut nres: i32;
                                (*interpreter).call_info = (*call_info).previous;
                                (*interpreter).top.stkidrel_pointer = base.offset(-(1 as isize));
                                nres = (*call_info).count_results as i32;
                                while ((nres > 0) as i32 != 0) as i64 != 0 {
                                    let fresh141 = (*interpreter).top.stkidrel_pointer;
                                    (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
                                    (*fresh141).tvalue.set_tag_variant(TagVariant::NilNil as u8);
                                    nres -= 1;
                                }
                            }
                            break;
                        }
                        72 => {
                            if ((*interpreter).hook_mask != 0) as i64 != 0 {
                                let ra_69: StackValuePointer = base
                                    .offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                                (*interpreter).top.stkidrel_pointer = ra_69.offset(1 as isize);
                                (*call_info).u.l.saved_program_counter = program_counter;
                                luad_poscall(interpreter, call_info, 1);
                                trap = 1;
                            } else {
                                let mut nres_0: i32 = (*call_info).count_results as i32;
                                (*interpreter).call_info = (*call_info).previous;
                                if nres_0 == 0 {
                                    (*interpreter).top.stkidrel_pointer = base.offset(-(1 as isize));
                                } else {
                                    let ra_70: StackValuePointer = base.offset(
                                        (i >> POSITION_A & !(!(0u32) << 8) << 0) as isize,
                                    );
                                    let io1_15: *mut TValue =
                                        &mut (*base.offset(-(1 as isize))).tvalue;
                                    let io2_15: *const TValue = &mut (*ra_70).tvalue;
                                    (*io1_15).copy_from(&(*io2_15));
                                    (*interpreter).top.stkidrel_pointer = base;
                                    while ((nres_0 > 1) as i32 != 0) as i64 != 0 {
                                        let fresh142 = (*interpreter).top.stkidrel_pointer;
                                        (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
                                        (*fresh142).tvalue.set_tag_variant(TagVariant::NilNil as u8);
                                        nres_0 -= 1;
                                    }
                                }
                            }
                            break;
                        }
                        OP_FORLOOP => {
                            let ra_71: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            if (*ra_71.offset(2 as isize)).tvalue.get_tag_variant()
                                == TAG_VARIANT_NUMERIC_INTEGER
                            {
                                let count: u64 = (*ra_71.offset(1 as isize)).tvalue.value.integer as u64;
                                if count > 0 {
                                    let step: i64 = (*ra_71.offset(2 as isize)).tvalue.value.integer;
                                    let mut index: i64 = (*ra_71).tvalue.value.integer;
                                    let io_43: *mut TValue = &mut (*ra_71.offset(1 as isize)).tvalue;
                                    (*io_43).value.integer = count.wrapping_sub(1 as u64) as i64;
                                    index = (index as u64).wrapping_add(step as u64) as i64;
                                    let io_44: *mut TValue = &mut (*ra_71).tvalue;
                                    (*io_44).value.integer = index;
                                    let io_45: *mut TValue = &mut (*ra_71.offset(3 as isize)).tvalue;
                                    (*io_45).value.integer = index;
                                    (*io_45).set_tag_variant(TAG_VARIANT_NUMERIC_INTEGER);
                                    program_counter = program_counter.offset(
                                        -((i >> POSITION_K & !(!(0u32) << 8 + 8 + 1) << 0) as i32
                                            as isize),
                                    );
                                }
                            } else if floatforloop(ra_71) != 0 {
                                program_counter = program_counter.offset(
                                    -((i >> POSITION_K & !(!(0u32) << 8 + 8 + 1) << 0) as i32
                                        as isize),
                                );
                            }
                            trap = (*call_info).u.l.trap;
                            continue;
                        }
                        74 => {
                            let ra_72: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            (*call_info).u.l.saved_program_counter = program_counter;
                            (*interpreter).top.stkidrel_pointer = (*call_info).top.stkidrel_pointer;
                            if forprep(interpreter, ra_72) != 0 {
                                program_counter = program_counter.offset(
                                    ((i >> POSITION_K & !(!(0u32) << 8 + 8 + 1) << 0) as i32 + 1)
                                        as isize,
                                );
                            }
                            continue;
                        }
                        75 => {
                            let ra_73: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            (*call_info).u.l.saved_program_counter = program_counter;
                            (*interpreter).top.stkidrel_pointer = (*call_info).top.stkidrel_pointer;
                            luaf_newtbcupval(interpreter, ra_73.offset(3 as isize));
                            program_counter = program_counter.offset(
                                (i >> POSITION_K & !(!(0u32) << 8 + 8 + 1) << 0) as isize,
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
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let mut n_4: i32 = (i >> POSITION_B & !(!(0u32) << 8) << 0) as i32;
                            let mut last: u32 =
                                (i >> POSITION_C & !(!(0u32) << 8) << 0) as u32;
                            let h: *mut Table =
                                &mut (*((*ra_76).tvalue.value.object as *mut Table));
                            if n_4 == 0 {
                                n_4 = ((*interpreter).top.stkidrel_pointer).offset_from(ra_76) as i32 - 1;
                            } else {
                                (*interpreter).top.stkidrel_pointer = (*call_info).top.stkidrel_pointer;
                            }
                            last = last.wrapping_add(n_4 as u32);
                            if (i & (1 as u32) << POSITION_K) as i32 != 0 {
                                last = last.wrapping_add(
                                    ((*program_counter >> POSITION_A & !(!(0u32) << 8 + 8 + 1 + 8) << 0)
                                        as i32
                                        * ((1 << 8) - 1 + 1))
                                        as u32,
                                );
                                program_counter = program_counter.offset(1);
                            }
                            if last > luah_realasize(h) {
                                luah_resizearray(interpreter, h, last as usize);
                            }
                            while n_4 > 0 {
                                let value: *mut TValue = &mut (*ra_76.offset(n_4 as isize)).tvalue;
                                let io1_17: *mut TValue = &mut *((*h).array)
                                    .offset(last.wrapping_sub(1 as u32) as isize)
                                    as *mut TValue;
                                let io2_17: *const TValue = value;
                                (*io1_17).copy_from(&(*io2_17));
                                last = last.wrapping_sub(1);
                                if (*value).is_collectable() {
                                    if (*(h as *mut Object)).get_marked() & 1 << 5 != 0
                                        && (*(*value).value.object).get_marked() & (1 << 3 | 1 << 4)
                                            != 0
                                    {
                                        luac_barrierback_(
                                            interpreter,
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
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let p: *mut Prototype = *((*(*cl).payload.l_prototype).prototype_prototypes.pointer).offset(
                                (i >> POSITION_K & !(!(0u32) << 8 + 8 + 1) << 0) as isize,
                            );
                            (*call_info).u.l.saved_program_counter = program_counter;
                            (*interpreter).top.stkidrel_pointer = (*call_info).top.stkidrel_pointer;
                            pushclosure(interpreter, p, ((*cl).upvalues).l_upvalues.as_mut_ptr(), base, ra_77);
                            if (*(*interpreter).global).gc_debt > 0 {
                                (*call_info).u.l.saved_program_counter = program_counter;
                                (*interpreter).top.stkidrel_pointer = ra_77.offset(1 as isize);
                                luac_step(interpreter);
                                trap = (*call_info).u.l.trap;
                            }
                            continue;
                        }
                        80 => {
                            let ra_78: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let n_5: i32 =
                                (i >> POSITION_C & !(!(0u32) << 8) << 0) as i32 - 1;
                            (*call_info).u.l.saved_program_counter = program_counter;
                            (*interpreter).top.stkidrel_pointer = (*call_info).top.stkidrel_pointer;
                            luat_getvarargs(interpreter, call_info, ra_78, n_5);
                            trap = (*call_info).u.l.trap;
                            continue;
                        }
                        81 => {
                            (*call_info).u.l.saved_program_counter = program_counter;
                            luat_adjustvarargs(
                                interpreter,
                                (i >> POSITION_A & !(!(0u32) << 8) << 0) as i32,
                                call_info,
                                (*cl).payload.l_prototype,
                            );
                            trap = (*call_info).u.l.trap;
                            if (trap != 0) as i64 != 0 {
                                luad_hookcall(interpreter, call_info);
                                (*interpreter).old_program_counter = 1;
                            }
                            base = ((*call_info).function.stkidrel_pointer).offset(1 as isize);
                            continue;
                        }
                        82 | _ => {
                            continue;
                        }
                    }
                    match current_block {
                        13973394567113199817 => {
                            let ra_74: StackValuePointer =
                                base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            memcpy(
                                ra_74.offset(4 as isize) as *mut libc::c_void,
                                ra_74 as *const libc::c_void,
                                (3 as u64)
                                    .wrapping_mul(::core::mem::size_of::<StackValue>() as u64),
                            );
                            (*interpreter).top.stkidrel_pointer = ra_74.offset(4 as isize).offset(3 as isize);
                            (*call_info).u.l.saved_program_counter = program_counter;
                            ccall(
                                interpreter,
                                ra_74.offset(4 as isize),
                                (i >> POSITION_C & !(!(0u32) << 8) << 0) as i32,
                                1,
                            );
                            trap = (*call_info).u.l.trap;
                            if (trap != 0) as i64 != 0 {
                                base = ((*call_info).function.stkidrel_pointer).offset(1 as isize);
                            }
                            let fresh144 = program_counter;
                            program_counter = program_counter.offset(1);
                            i = *fresh144;
                        }
                        _ => {}
                    }
                    let ra_75: StackValuePointer =
                        base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                    if !(*ra_75.offset(4 as isize)).tvalue.is_tagtype_nil() {
                        let io1_16: *mut TValue = &mut (*ra_75.offset(2 as isize)).tvalue;
                        let io2_16: *const TValue = &mut (*ra_75.offset(4 as isize)).tvalue;
                        (*io1_16).copy_from(&(*io2_16));
                        program_counter = program_counter.offset(
                            -((i >> POSITION_K & !(!(0u32) << 8 + 8 + 1) << 0) as isize),
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
pub unsafe extern "C" fn findfield(interpreter: *mut Interpreter, objidx: i32, level: i32) -> bool {
    unsafe {
        if level == 0 || (lua_type(interpreter, -1) != Some(TagType::Table)) {
            return false;
        }
        (*interpreter).push_nil();
        while lua_next(interpreter, -2) != 0 {
            if lua_type(interpreter, -2) == Some(TagType::String) {
                if lua_rawequal(interpreter, objidx, -1) {
                    lua_settop(interpreter, -2);
                    return true;
                } else if findfield(interpreter, objidx, level - 1) {
                    lua_pushstring(interpreter, b".\0" as *const u8 as *const i8);
                    lua_copy(interpreter, -1, -3);
                    lua_settop(interpreter, -2);
                    lua_concat(interpreter, 3);
                    return true;
                }
            }
            lua_settop(interpreter, -2);
        }
        return false;
    }
}
pub unsafe extern "C" fn pushglobalfuncname(interpreter: *mut Interpreter, ar: *mut DebugInfo) -> bool {
    unsafe {
        let top: i32 = (*interpreter).get_top();
        lua_getinfo(interpreter, b"f\0" as *const u8 as *const i8, ar);
        lua_getfield(
            interpreter,
            -(1000000 as i32) - 1000 as i32,
            b"_LOADED\0" as *const u8 as *const i8,
        );
        lual_checkstack(interpreter, 6, b"not enough stack\0" as *const u8 as *const i8);
        if findfield(interpreter, top + 1, 2) {
            let name: *const i8 = lua_tolstring(interpreter, -1, std::ptr::null_mut());
            if strncmp(name, b"_G.\0" as *const u8 as *const i8, 3 as u64) == 0 {
                lua_pushstring(interpreter, name.offset(3 as isize));
                lua_rotate(interpreter, -2, -1);
                lua_settop(interpreter, -2);
            }
            lua_copy(interpreter, -1, top + 1);
            lua_settop(interpreter, top + 1);
            return true;
        } else {
            lua_settop(interpreter, top);
            return false;
        };
    }
}
pub unsafe extern "C" fn pushfuncname(interpreter: *mut Interpreter, ar: *mut DebugInfo) {
    unsafe {
        if pushglobalfuncname(interpreter, ar) {
            lua_pushfstring(
                interpreter,
                b"function '%s'\0" as *const u8 as *const i8,
                lua_tolstring(interpreter, -1, std::ptr::null_mut()),
            );
            lua_rotate(interpreter, -2, -1);
            lua_settop(interpreter, -2);
        } else if *(*ar).namewhat as i32 != Character::Null as i32 {
            lua_pushfstring(
                interpreter,
                b"%s '%s'\0" as *const u8 as *const i8,
                (*ar).namewhat,
                (*ar).name,
            );
        } else if *(*ar).what as i32 == CHARACTER_LOWER_M as i32 {
            lua_pushstring(interpreter, b"main chunk\0" as *const u8 as *const i8);
        } else if *(*ar).what as i32 != CHARACTER_UPPER_C as i32 {
            lua_pushfstring(
                interpreter,
                b"function <%s:%d>\0" as *const u8 as *const i8,
                ((*ar).short_src).as_mut_ptr(),
                (*ar).line_defined,
            );
        } else {
            lua_pushstring(interpreter, b"?\0" as *const u8 as *const i8);
        };
    }
}
pub unsafe extern "C" fn lastlevel(interpreter: *mut Interpreter) -> i32 {
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
        while lua_getstack(interpreter, le, &mut ar) != 0 {
            li = le;
            le *= 2;
        }
        while li < le {
            let m: i32 = (li + le) / 2;
            if lua_getstack(interpreter, m, &mut ar) != 0 {
                li = m + 1;
            } else {
                le = m;
            }
        }
        return le - 1;
    }
}
pub unsafe extern "C" fn lual_traceback(
    interpreter: *mut Interpreter,
    other_state: *mut Interpreter,
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
        b.initialize(interpreter);
        if !message.is_null() {
            b.add_string(message);
            (b.vector.length < b.vector.size || !(b.prepare_with_size(1)).is_null()) as i32;
            let fresh145 = b.vector.length;
            b.vector.length = (b.vector.length).wrapping_add(1);
            *(b.vector.pointer).offset(fresh145 as isize) = CHARACTER_LF as i8;
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
                    interpreter,
                    b"\n\t...\t(skipping %d levels)\0" as *const u8 as *const i8,
                    n,
                );
                b.add_value();
                level += n;
            } else {
                lua_getinfo(other_state, b"Slnt\0" as *const u8 as *const i8, &mut ar);
                if ar.currentline <= 0 {
                    lua_pushfstring(
                        interpreter,
                        b"\n\t%s: in \0" as *const u8 as *const i8,
                        (ar.short_src).as_mut_ptr(),
                    );
                } else {
                    lua_pushfstring(
                        interpreter,
                        b"\n\t%s:%d: in \0" as *const u8 as *const i8,
                        (ar.short_src).as_mut_ptr(),
                        ar.currentline,
                    );
                }
                b.add_value();
                pushfuncname(interpreter, &mut ar);
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
    interpreter: *mut Interpreter,
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
        if lua_getstack(interpreter, 0, &mut ar) == 0 {
            return lual_error(interpreter, b"bad argument #%d (%s)\0".as_ptr(), arg, extramsg);
        }
        lua_getinfo(interpreter, b"n\0" as *const u8 as *const i8, &mut ar);
        if strcmp(ar.namewhat, b"method\0" as *const u8 as *const i8) == 0 {
            arg -= 1;
            if arg == 0 {
                return lual_error(interpreter, b"calling '%s' on bad self (%s)\0".as_ptr(), ar.name, extramsg);
            }
        }
        if ar.name.is_null() {
            ar.name = if pushglobalfuncname(interpreter, &mut ar) {
                lua_tolstring(interpreter, -1, std::ptr::null_mut())
            } else {
                b"?\0" as *const u8 as *const i8
            };
        }
        return lual_error(
            interpreter,
            b"bad argument #%d to '%s' (%s)\0".as_ptr(),
            arg,
            ar.name,
            extramsg,
        );
    }
}
pub unsafe extern "C" fn lual_typeerror(interpreter: *mut Interpreter, arg: i32, tname: *const i8) -> i32 {
    unsafe {
        let message: *const i8;
        let typearg: *const i8;
        if lual_getmetafield(interpreter, arg, b"__name\0" as *const u8 as *const i8) == TagType::String {
            typearg = lua_tolstring(interpreter, -1, std::ptr::null_mut());
        } else if lua_type(interpreter, arg) == Some(TagType::Pointer) {
            typearg = b"light userdata\0" as *const u8 as *const i8;
        } else {
            typearg = lua_typename(interpreter, lua_type(interpreter, arg));
        }
        message = lua_pushfstring(
            interpreter,
            b"%s expected, got %s\0" as *const u8 as *const i8,
            tname,
            typearg,
        );
        return lual_argerror(interpreter, arg, message);
    }
}
pub unsafe fn tag_error2(interpreter: *mut Interpreter, arg: i32, tag: Option<TagType>) {
    unsafe {
        lual_typeerror(interpreter, arg, lua_typename(interpreter, tag));
    }
}
pub unsafe extern "C" fn lual_where(interpreter: *mut Interpreter, level: i32) {
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
        if lua_getstack(interpreter, level, &mut ar) != 0 {
            lua_getinfo(interpreter, b"Sl\0" as *const u8 as *const i8, &mut ar);
            if ar.currentline > 0 {
                lua_pushfstring(
                    interpreter,
                    b"%s:%d: \0" as *const u8 as *const i8,
                    (ar.short_src).as_mut_ptr(),
                    ar.currentline,
                );
                return;
            }
        }
        lua_pushfstring(interpreter, b"\0" as *const u8 as *const i8);
    }
}
pub unsafe extern "C" fn lual_error(interpreter: *mut Interpreter, fmt: *const u8, args: ...) -> i32 {
    unsafe {
        let mut argp: ::core::ffi::VaListImpl;
        argp = args.clone();
        lual_where(interpreter, 1);
        lua_pushvfstring(interpreter, fmt as *const i8, argp.as_va_list());
        lua_concat(interpreter, 2);
        return lua_error(interpreter);
    }
}
pub unsafe extern "C" fn lual_fileresult(interpreter: *mut Interpreter, stat: i32, fname: *const i8) -> i32 {
    unsafe {
        let en: i32 = *__errno_location();
        if stat != 0 {
            (*interpreter).push_boolean(true);
            return 1;
        } else {
            let message: *const i8;
            (*interpreter).push_nil();
            message = if en != 0 {
                strerror(en) as *const i8
            } else {
                b"(no extra info)\0" as *const u8 as *const i8
            };
            if !fname.is_null() {
                lua_pushfstring(interpreter, b"%s: %s\0" as *const u8 as *const i8, fname, message);
            } else {
                lua_pushstring(interpreter, message);
            }
            (*interpreter).push_integer(en as i64);
            return 3;
        };
    }
}
pub unsafe extern "C" fn lual_execresult(interpreter: *mut Interpreter, mut stat: i32) -> i32 {
    unsafe {
        if stat != 0 && *__errno_location() != 0 {
            return lual_fileresult(interpreter, 0, std::ptr::null());
        } else {
            let mut what: *const i8 = b"exit\0" as *const u8 as *const i8;
            if stat & 0x7f as i32 == 0 {
                stat = (stat & 0xff00 as i32) >> 8;
            } else if ((stat & 0x7f as i32) + 1) as i32 >> 1 > 0 {
                stat = stat & 0x7f as i32;
                what = b"signal\0" as *const u8 as *const i8;
            }
            if *what as i32 == CHARACTER_LOWER_E as i32 && stat == 0 {
                (*interpreter).push_boolean(true);
            } else {
                (*interpreter).push_nil();
            }
            lua_pushstring(interpreter, what);
            (*interpreter).push_integer(stat as i64);
            return 3;
        };
    }
}
pub unsafe extern "C" fn lual_newmetatable(interpreter: *mut Interpreter, tname: *const i8) -> i32 {
    unsafe {
        if lua_getfield(interpreter, -1000000 - 1000, tname) != 0 {
            return 0;
        }
        lua_settop(interpreter, -2);
        (*interpreter).lua_createtable();
        lua_pushstring(interpreter, tname);
        lua_setfield(interpreter, -2, b"__name\0" as *const u8 as *const i8);
        lua_pushvalue(interpreter, -1);
        lua_setfield(interpreter, -(1000000 as i32) - 1000 as i32, tname);
        return 1;
    }
}
pub unsafe extern "C" fn lual_setmetatable(interpreter: *mut Interpreter, tname: *const i8) {
    unsafe {
        lua_getfield(interpreter, -(1000000 as i32) - 1000 as i32, tname);
        lua_setmetatable(interpreter, -2);
    }
}
pub unsafe extern "C" fn lual_testudata(
    interpreter: *mut Interpreter,
    arbitrary_data: i32,
    tname: *const i8,
) -> *mut libc::c_void {
    unsafe {
        let mut p: *mut libc::c_void = lua_touserdata(interpreter, arbitrary_data);
        if !p.is_null() {
            if (*interpreter).lua_getmetatable(arbitrary_data) {
                lua_getfield(interpreter, -(1000000 as i32) - 1000 as i32, tname);
                if !lua_rawequal(interpreter, -1, -2) {
                    p = std::ptr::null_mut();
                }
                lua_settop(interpreter, -2 - 1);
                return p;
            }
        }
        return std::ptr::null_mut();
    }
}
pub unsafe extern "C" fn lual_checkudata(
    interpreter: *mut Interpreter,
    arbitrary_data: i32,
    tname: *const i8,
) -> *mut libc::c_void {
    unsafe {
        let p: *mut libc::c_void = lual_testudata(interpreter, arbitrary_data, tname);
        (((p != std::ptr::null_mut()) as i32 != 0) as i64 != 0
            || lual_typeerror(interpreter, arbitrary_data, tname) != 0) as i32;
        return p;
    }
}
pub unsafe extern "C" fn lual_checkoption(
    interpreter: *mut Interpreter,
    arg: i32,
    def: *const i8,
    lst: *const *const i8,
) -> i32 {
    unsafe {
        let name: *const i8 = if !def.is_null() {
            lual_optlstring(interpreter, arg, def, std::ptr::null_mut())
        } else {
            lual_checklstring(interpreter, arg, std::ptr::null_mut())
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
            interpreter,
            arg,
            lua_pushfstring(
                interpreter,
                b"invalid option '%s'\0" as *const u8 as *const i8,
                name,
            ),
        );
    }
}
pub unsafe extern "C" fn lual_checkstack(interpreter: *mut Interpreter, space: i32, message: *const i8) {
    unsafe {
        if ((lua_checkstack(interpreter, space) == 0) as i32 != 0) as i64 != 0 {
            if !message.is_null() {
                lual_error(
                    interpreter,
                    b"stack overflow (%s)\0".as_ptr(),
                    message,
                );
            } else {
                lual_error(interpreter, b"stack overflow\0".as_ptr());
            }
        }
    }
}
pub unsafe extern "C" fn lual_checktype(interpreter: *mut Interpreter, arg: i32, tag: TagType) {
    unsafe {
        if lua_type(interpreter, arg) != Some(tag) {
            tag_error2(interpreter, arg, Some(tag));
        }
    }
}
pub unsafe extern "C" fn lual_checkany(interpreter: *mut Interpreter, arg: i32) {
    unsafe {
        if lua_type(interpreter, arg) == None {
            lual_argerror(interpreter, arg, b"value expected\0" as *const u8 as *const i8);
        }
    }
}
pub unsafe extern "C" fn lual_checklstring(
    interpreter: *mut Interpreter,
    arg: i32,
    length: *mut u64,
) -> *const i8 {
    unsafe {
        let s: *const i8 = lua_tolstring(interpreter, arg, length);
        if (s.is_null() as i32 != 0) as i64 != 0 {
            tag_error2(interpreter, arg, Some(TagType::String));
        }
        return s;
    }
}
pub unsafe extern "C" fn lual_optlstring(
    interpreter: *mut Interpreter,
    arg: i32,
    def: *const i8,
    length: *mut u64,
) -> *const i8 {
    unsafe {
        match lua_type(interpreter, arg) {
            None | Some(TagType::Nil) => {
                if !length.is_null() {
                    *length = if !def.is_null() { strlen(def) } else { 0u64 };
                }
                return def;
            },
            _ => {
                return lual_checklstring(interpreter, arg, length);
            },
        }
    }
}
pub unsafe extern "C" fn lual_checknumber(interpreter: *mut Interpreter, arg: i32) -> f64 {
    unsafe {
        let mut is_number: bool = false;
        let d: f64 = lua_tonumberx(interpreter, arg, &mut is_number);
        if !is_number {
            tag_error2(interpreter, arg, Some(TagType::Numeric));
        }
        return d;
    }
}
pub unsafe extern "C" fn lual_optnumber(interpreter: *mut Interpreter, arg: i32, def: f64) -> f64 {
    unsafe {
        match lua_type(interpreter, arg) {
            None | Some(TagType::Nil) => {
                def
            },
            _ => {
                lual_checknumber(interpreter, arg)
            }
        }
    }
}
pub unsafe extern "C" fn interror(interpreter: *mut Interpreter, arg: i32) {
    unsafe {
        if lua_isnumber(interpreter, arg) {
            lual_argerror(
                interpreter,
                arg,
                b"number has no integer representation\0" as *const u8 as *const i8,
            );
        } else {
            tag_error2(interpreter, arg, Some(TagType::Numeric));
        };
    }
}
pub unsafe extern "C" fn lual_checkinteger(interpreter: *mut Interpreter, arg: i32) -> i64 {
    unsafe {
        let mut is_number: bool = false;
        let ret: i64 = lua_tointegerx(interpreter, arg, &mut is_number);
        if !is_number {
            interror(interpreter, arg);
        }
        return ret;
    }
}
pub unsafe extern "C" fn lual_optinteger(interpreter: *mut Interpreter, arg: i32, def: i64) -> i64 {
    unsafe {
        return match lua_type(interpreter, arg) {
            None | Some(TagType::Nil) => {
                def
            },
            _ => {
                lual_checkinteger(interpreter, arg)
            }
        };
    }
}
pub unsafe extern "C" fn get_f(
    mut _state: *mut Interpreter,
    arbitrary_data: *mut libc::c_void,
    size: *mut u64,
) -> *const i8 {
    unsafe {
        let lf: *mut LoadF = arbitrary_data as *mut LoadF;
        if (*lf).n > 0 {
            *size = (*lf).n as u64;
            (*lf).n = 0;
        } else {
            if feof((*lf).file) != 0 {
                return std::ptr::null();
            }
            *size = fread(
                ((*lf).buffer).as_mut_ptr() as *mut libc::c_void,
                1 as u64,
                ::core::mem::size_of::<[i8; 8192]>() as u64,
                (*lf).file,
            );
        }
        return ((*lf).buffer).as_mut_ptr();
    }
}
pub unsafe extern "C" fn errfile(interpreter: *mut Interpreter, what: *const i8, fnameindex: i32) -> i32 {
    unsafe {
        let err: i32 = *__errno_location();
        let filename: *const i8 =
            (lua_tolstring(interpreter, fnameindex, std::ptr::null_mut())).offset(1 as isize);
        if err != 0 {
            lua_pushfstring(
                interpreter,
                b"cannot %s %s: %s\0" as *const u8 as *const i8,
                what,
                filename,
                strerror(err),
            );
        } else {
            lua_pushfstring(
                interpreter,
                b"cannot %s %s\0" as *const u8 as *const i8,
                what,
                filename,
            );
        }
        lua_rotate(interpreter, fnameindex, -1);
        lua_settop(interpreter, -2);
        return 5 + 1;
    }
}
pub unsafe extern "C" fn skip_bom(file: *mut FILE) -> i32 {
    unsafe {
        let c: i32 = getc(file);
        if c == 0xef as i32 && getc(file) == 0xbb as i32 && getc(file) == 0xbf as i32 {
            return getc(file);
        } else {
            return c;
        };
    }
}
pub unsafe extern "C" fn skipcomment(file: *mut FILE, cp: *mut i32) -> i32 {
    unsafe {
        *cp = skip_bom(file);
        let mut c: i32 = *cp;
        if c == CHARACTER_OCTOTHORPE as i32 {
            loop {
                c = getc(file);
                if !(c != -1 && c != CHARACTER_LF as i32) {
                    break;
                }
            }
            *cp = getc(file);
            return 1;
        } else {
            return 0;
        };
    }
}
pub unsafe extern "C" fn lual_loadfilex(
    interpreter: *mut Interpreter,
    filename: *const i8,
    mode: *const i8,
) -> i32 {
    unsafe {
        let mut lf: LoadF = LoadF {
            n: 0,
            file: std::ptr::null_mut(),
            buffer: [0; 8192],
        };
        let status: i32;
        let readstatus: i32;
        let mut c: i32 = 0;
        let fnameindex: i32 = (*interpreter).get_top() + 1;
        if filename.is_null() {
            lua_pushstring(interpreter, b"=stdin\0" as *const u8 as *const i8);
            lf.file = stdin;
        } else {
            lua_pushfstring(interpreter, b"@%s\0" as *const u8 as *const i8, filename);
            *__errno_location() = 0;
            lf.file = fopen(filename, b"r\0" as *const u8 as *const i8);
            if (lf.file).is_null() {
                return errfile(interpreter, b"open\0" as *const u8 as *const i8, fnameindex);
            }
        }
        lf.n = 0;
        if skipcomment(lf.file, &mut c) != 0 {
            let fresh148 = lf.n;
            lf.n = lf.n + 1;
            lf.buffer[fresh148 as usize] = CHARACTER_LF as i8;
        }
        if c == (*::core::mem::transmute::<&[u8; 5], &[i8; 5]>(b"\x1BLua\0"))[0] as i32 {
            lf.n = 0;
            if !filename.is_null() {
                *__errno_location() = 0;
                lf.file = freopen(filename, b"rb\0" as *const u8 as *const i8, lf.file);
                if (lf.file).is_null() {
                    return errfile(interpreter, b"reopen\0" as *const u8 as *const i8, fnameindex);
                }
                skipcomment(lf.file, &mut c);
            }
        }
        if c != -1 {
            let fresh149 = lf.n;
            lf.n = lf.n + 1;
            lf.buffer[fresh149 as usize] = c as i8;
        }
        *__errno_location() = 0;
        status = lua_load(
            interpreter,
            Some(
                get_f as unsafe extern "C" fn(*mut Interpreter, *mut libc::c_void, *mut u64) -> *const i8,
            ),
            &mut lf as *mut LoadF as *mut libc::c_void,
            lua_tolstring(interpreter, -1, std::ptr::null_mut()),
            mode,
        );
        readstatus = ferror(lf.file);
        if !filename.is_null() {
            fclose(lf.file);
        }
        if readstatus != 0 {
            lua_settop(interpreter, fnameindex);
            return errfile(interpreter, b"read\0" as *const u8 as *const i8, fnameindex);
        }
        lua_rotate(interpreter, fnameindex, -1);
        lua_settop(interpreter, -2);
        return status;
    }
}
pub unsafe extern "C" fn get_s(
    mut _state: *mut Interpreter,
    arbitrary_data: *mut libc::c_void,
    size: *mut u64,
) -> *const i8 {
    unsafe {
        let load_s: *mut VectorT::<i8> = arbitrary_data as *mut VectorT::<i8>;
        if (*load_s).size == 0 {
            return std::ptr::null();
        }
        *size = (*load_s).size as u64;
        (*load_s).size = 0;
        return (*load_s).pointer;
    }
}
pub unsafe extern "C" fn lual_loadbufferx(
    interpreter: *mut Interpreter,
    buffer: *const i8,
    size: u64,
    name: *const i8,
    mode: *const i8,
) -> i32 {
    unsafe {
        let mut load_s: VectorT::<i8> = VectorT::<i8> {
            pointer: std::ptr::null_mut(),
            size: 0,
            length: 0,
        };
        load_s.pointer = buffer as *mut i8;
        load_s.size = size as i32;
        return lua_load(
            interpreter,
            Some(
                get_s as unsafe extern "C" fn(*mut Interpreter, *mut libc::c_void, *mut u64) -> *const i8,
            ),
            &mut load_s as *mut VectorT::<i8> as *mut libc::c_void,
            name,
            mode,
        );
    }
}
pub unsafe extern "C" fn lual_getmetafield(interpreter: *mut Interpreter, obj: i32, event: *const i8) -> TagType {
    unsafe {
        if (*interpreter).lua_getmetatable(obj) {
            lua_pushstring(interpreter, event);
            let tag = lua_rawget(interpreter, -2);
            if tag == TagType::Nil {
                lua_settop(interpreter, -3);
            } else {
                lua_rotate(interpreter, -2, -1);
                lua_settop(interpreter, -2);
            }
            return tag;
        } else {
            return TagType::Nil;
        };
    }
}
pub unsafe extern "C" fn lual_callmeta(interpreter: *mut Interpreter, mut obj: i32, event: *const i8) -> bool {
    unsafe {
        obj = lua_absindex(interpreter, obj);
        if lual_getmetafield(interpreter, obj, event) == TagType::Nil {
            return false;
        }
        lua_pushvalue(interpreter, obj);
        lua_callk(interpreter, 1, 1, 0, None);
        return true;
    }
}
pub unsafe extern "C" fn lual_len(interpreter: *mut Interpreter, index: i32) -> i64 {
    unsafe {
        let l: i64;
        let mut is_number: bool = false;
        lua_len(interpreter, index);
        l = lua_tointegerx(interpreter, -1, &mut is_number);
        if !is_number {
            lual_error(
                interpreter,
                b"object length is not an integer\0".as_ptr(),
            );
        }
        lua_settop(interpreter, -2);
        return l;
    }
}
pub unsafe extern "C" fn lual_tolstring(
    interpreter: *mut Interpreter,
    mut index: i32,
    length: *mut u64,
) -> *const i8 {
    unsafe {
        index = lua_absindex(interpreter, index);
        if lual_callmeta(interpreter, index, b"__tostring\0" as *const u8 as *const i8) {
            if !lua_isstring(interpreter, -1) {
                lual_error(
                    interpreter,
                    b"'__tostring' must return a string\0".as_ptr(),
                );
            }
        } else {
            match lua_type(interpreter, index) {
                Some(TagType::Numeric) => {
                    if lua_isinteger(interpreter, index) {
                        lua_pushfstring(
                            interpreter,
                            b"%I\0" as *const u8 as *const i8,
                            lua_tointegerx(interpreter, index, std::ptr::null_mut()),
                        );
                    } else {
                        lua_pushfstring(
                            interpreter,
                            b"%f\0" as *const u8 as *const i8,
                            lua_tonumberx(interpreter, index, std::ptr::null_mut()),
                        );
                    }
                }
                Some(TagType::String) => {
                    lua_pushvalue(interpreter, index);
                }
                Some(TagType::Boolean) => {
                    lua_pushstring(
                        interpreter,
                        if lua_toboolean(interpreter, index) != 0 {
                            b"true\0" as *const u8 as *const i8
                        } else {
                            b"false\0" as *const u8 as *const i8
                        },
                    );
                }
                Some(TagType::Nil) => {
                    lua_pushstring(interpreter, b"nil\0" as *const u8 as *const i8);
                }
                _ => {
                    let tag =
                        lual_getmetafield(interpreter, index, b"__name\0" as *const u8 as *const i8);
                    let kind: *const i8 = if tag == TagType::String {
                        lua_tolstring(interpreter, -1, std::ptr::null_mut())
                    } else {
                        lua_typename(interpreter, lua_type(interpreter, index))
                    };
                    lua_pushfstring(
                        interpreter,
                        b"%s: %p\0" as *const u8 as *const i8,
                        kind,
                        User::lua_topointer(interpreter, index),
                    );
                    if tag != TagType::Nil {
                        lua_rotate(interpreter, -2, -1);
                        lua_settop(interpreter, -2);
                    }
                }
            }
        }
        return lua_tolstring(interpreter, -1, length);
    }
}
pub unsafe extern "C" fn lual_setfuncs(
    interpreter: *mut Interpreter,
    mut l: *const RegisteredFunction,
    nup: i32,
) {
    unsafe {
        lual_checkstack(interpreter, nup, b"too many upvalues\0" as *const u8 as *const i8);
        while !((*l).name).is_null() {
            if ((*l).function).is_none() {
                (*interpreter).push_boolean(false);
            } else {
                for _ in 0..nup {
                    lua_pushvalue(interpreter, -nup);
                }
                lua_pushcclosure(interpreter, (*l).function, nup);
            }
            lua_setfield(interpreter, -(nup + 2), (*l).name);
            l = l.offset(1);
        }
        lua_settop(interpreter, -nup - 1);
    }
}
pub unsafe extern "C" fn lual_getsubtable(
    interpreter: *mut Interpreter,
    mut index: i32,
    fname: *const i8,
) -> i32 {
    unsafe {
        if lua_getfield(interpreter, index, fname) == 5 {
            return 1;
        } else {
            lua_settop(interpreter, -2);
            index = lua_absindex(interpreter, index);
            (*interpreter).lua_createtable();
            lua_pushvalue(interpreter, -1);
            lua_setfield(interpreter, index, fname);
            return 0;
        };
    }
}
pub unsafe extern "C" fn lual_requiref(
    interpreter: *mut Interpreter,
    modname: *const i8,
    openf: CFunction,
    glb: i32,
) {
    unsafe {
        lual_getsubtable(
            interpreter,
            -(1000000 as i32) - 1000 as i32,
            b"_LOADED\0" as *const u8 as *const i8,
        );
        lua_getfield(interpreter, -1, modname);
        if lua_toboolean(interpreter, -1) == 0 {
            lua_settop(interpreter, -2);
            lua_pushcclosure(interpreter, openf, 0);
            lua_pushstring(interpreter, modname);
            lua_callk(interpreter, 1, 1, 0, None);
            lua_pushvalue(interpreter, -1);
            lua_setfield(interpreter, -3, modname);
        }
        lua_rotate(interpreter, -2, -1);
        lua_settop(interpreter, -2);
        if glb != 0 {
            lua_pushvalue(interpreter, -1);
            lua_setglobal(interpreter, modname);
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
            (*b).add_string_with_length(s, wild.offset_from(s) as usize);
            (*b).add_string(r);
            s = wild.offset(l as isize);
        }
        (*b).add_string(s);
    }
}
pub unsafe extern "C" fn lual_gsub(
    interpreter: *mut Interpreter,
    s: *const i8,
    p: *const i8,
    r: *const i8,
) -> *const i8 {
    unsafe {
        let mut b = Buffer::new();
        b.initialize(interpreter);
        lual_addgsub(&mut b, s, p, r);
        b.push_result();
        return lua_tolstring(interpreter, -1, std::ptr::null_mut());
    }
}
pub unsafe extern "C" fn raw_allocate(
    ptr: *mut libc::c_void,
    mut _old_size: usize,
    new_size: usize,
) -> *mut libc::c_void {
    unsafe {
        if new_size == 0 {
            free(ptr);
            return std::ptr::null_mut();
        } else {
            return realloc(ptr, new_size as u64);
        };
    }
}
pub unsafe extern "C" fn panic(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let message: *const i8 = if lua_type(interpreter, -1) == Some(TagType::String) {
            lua_tolstring(interpreter, -1, std::ptr::null_mut())
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
    interpreter: *mut Interpreter,
    mut message: *const i8,
    tocont: i32,
) -> i32 {
    unsafe {
        if tocont != 0 || {
            let fresh150 = message;
            message = message.offset(1);
            *fresh150 as i32 != CHARACTER_AT as i32
        } {
            return 0;
        } else {
            if strcmp(message, b"off\0" as *const u8 as *const i8) == 0 {
                lua_setwarnf(
                    interpreter,
                    Some(warnfoff as unsafe extern "C" fn(*mut libc::c_void, *const i8, i32) -> ()),
                    interpreter as *mut libc::c_void,
                );
            } else if strcmp(message, b"on\0" as *const u8 as *const i8) == 0 {
                lua_setwarnf(
                    interpreter,
                    Some(warnfon as unsafe extern "C" fn(*mut libc::c_void, *const i8, i32) -> ()),
                    interpreter as *mut libc::c_void,
                );
            }
            return 1;
        };
    }
}
pub unsafe extern "C" fn warnfoff(arbitrary_data: *mut libc::c_void, message: *const i8, tocont: i32) {
    unsafe {
        checkcontrol(arbitrary_data as *mut Interpreter, message, tocont);
    }
}
pub unsafe extern "C" fn warnfcont(arbitrary_data: *mut libc::c_void, message: *const i8, tocont: i32) {
    unsafe {
        let interpreter: *mut Interpreter = arbitrary_data as *mut Interpreter;
        fprintf(stderr, b"%s\0" as *const u8 as *const i8, message);
        fflush(stderr);
        if tocont != 0 {
            lua_setwarnf(
                interpreter,
                Some(warnfcont as unsafe extern "C" fn(*mut libc::c_void, *const i8, i32) -> ()),
                interpreter as *mut libc::c_void,
            );
        } else {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const i8,
                b"\n\0" as *const u8 as *const i8,
            );
            fflush(stderr);
            lua_setwarnf(
                interpreter,
                Some(warnfon as unsafe extern "C" fn(*mut libc::c_void, *const i8, i32) -> ()),
                interpreter as *mut libc::c_void,
            );
        };
    }
}
pub unsafe extern "C" fn warnfon(arbitrary_data: *mut libc::c_void, message: *const i8, tocont: i32) {
    unsafe {
        if checkcontrol(arbitrary_data as *mut Interpreter, message, tocont) != 0 {
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
pub unsafe extern "C" fn lual_newstate() -> *mut Interpreter {
    unsafe {
        let mut interpreter: *mut Interpreter = raw_allocate(
            std::ptr::null_mut(),
            0,
            ::core::mem::size_of::<Interpreter>(),
        ) as *mut Interpreter;
        if interpreter.is_null() {
            return std::ptr::null_mut();
        }
        let global: *mut Global = raw_allocate(
            std::ptr::null_mut(),
            0,
            ::core::mem::size_of::<Global>(),
        ) as *mut Global;
        if global.is_null() {
            raw_allocate(interpreter as *mut u8 as *mut libc::c_void, ::core::mem::size_of::<Interpreter>(), 0);
            return std::ptr::null_mut();
        }
        (*interpreter).set_tag_variant(TAG_VARIANT_STATE);
        (*global).current_white = (1 << 3) as u8;
        (*interpreter).set_marked((*global).current_white & (1 << 3 | 1 << 4));
        preinit_thread(interpreter, global);
        (*global).all_gc = &mut (*(interpreter as *mut Object));
        (*interpreter).object.next = std::ptr::null_mut();
        (*interpreter).count_c_calls =
            ((*interpreter).count_c_calls as u32).wrapping_add(0x10000 as u32) as u32;
        (*global).warn_function = None;
        (*global).warn_userdata = std::ptr::null_mut();
        (*global).main_state = interpreter;
        (*global).seed = luai_makeseed(interpreter);
        (*global).gc_step = 2 as u8;
        (*global).string_table.length = 0;
        (*global).string_table.size = (*global).string_table.length;
        (*global).string_table.hash = std::ptr::null_mut();
        (*global).l_registry.set_tag_variant(TagVariant::NilNil as u8);
        (*global).panic = None;
        (*global).gc_state = 8 as u8;
        (*global).gc_kind = 0;
        (*global).gcstopem = 0;
        (*global).is_emergency = false;
        (*global).fixed_gc = std::ptr::null_mut();
        (*global).to_be_finalized = (*global).fixed_gc;
        (*global).finalized_objects = (*global).to_be_finalized;
        (*global).really_old = std::ptr::null_mut();
        (*global).old1 = (*global).really_old;
        (*global).survival = (*global).old1;
        (*global).first_old1 = (*global).survival;
        (*global).finobjrold = std::ptr::null_mut();
        (*global).finobjold1 = (*global).finobjrold;
        (*global).finobjsur = (*global).finobjold1;
        (*global).sweep_gc = std::ptr::null_mut();
        (*global).gray_again = std::ptr::null_mut();
        (*global).gray = (*global).gray_again;
        (*global).all_weak = std::ptr::null_mut();
        (*global).ephemeron = (*global).all_weak;
        (*global).weak = (*global).ephemeron;
        (*global).twups = std::ptr::null_mut();
        (*global).total_bytes = 0;
        (*global).total_bytes += ::core::mem::size_of::<Interpreter>() as i64;
        (*global).total_bytes += ::core::mem::size_of::<Global>() as i64;
        (*global).gc_debt = 0;
        (*global).last_atomic = 0;
        let io: *mut TValue = &mut (*global).none_value;
        (*io).value.integer = 0;
        (*io).set_tag_variant(TAG_VARIANT_NUMERIC_INTEGER);
        (*global).gc_pause = 200 / 4;
        (*global).gc_step_multiplier = 100 / 4;
        (*global).gc_step_size = 13;
        (*global).generational_major_multiplier = 100 / 4;
        (*global).generational_minor_multiplier = 20;
        for i in 0..9 {
            (*global).metatables[i as usize] = std::ptr::null_mut();
        }
        if luad_rawrunprotected(
            interpreter,
            Some(f_luaopen as unsafe extern "C" fn(*mut Interpreter, *mut libc::c_void) -> ()),
            std::ptr::null_mut(),
        ) != 0
        {
            close_state(interpreter);
            interpreter = std::ptr::null_mut();
        }
        if (interpreter != std::ptr::null_mut()) as i64 != 0 {
            lua_atpanic(
                interpreter,
                Some(panic as unsafe extern "C" fn(*mut Interpreter) -> i32),
            );
            lua_setwarnf(
                interpreter,
                Some(warnfoff as unsafe extern "C" fn(*mut libc::c_void, *const i8, i32) -> ()),
                interpreter as *mut libc::c_void,
            );
        }
        return interpreter;
    }
}
pub unsafe extern "C" fn lual_checkversion_(interpreter: *mut Interpreter, version: f64, size: u64) {
    unsafe {
        let v: f64 = 504.0;
        if size
            != (::core::mem::size_of::<i64>() as u64)
                .wrapping_mul(16 as u64)
                .wrapping_add(::core::mem::size_of::<f64>() as u64)
        {
            lual_error(
                interpreter,
                b"core and library have incompatible numeric types\0".as_ptr(),
            );
        } else if v != version {
            lual_error(
                interpreter,
                b"version mismatch: app. needs %f, Lua core provides %f\0".as_ptr(),
                version,
                v,
            );
        }
    }
}
pub unsafe extern "C" fn luab_print(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let n: i32 = (*interpreter).get_top();
        for i in 1..(1 + n) {
            let mut l: u64 = 0;
            let s: *const i8 = lual_tolstring(interpreter, i, &mut l);
            if i > 1 {
                fwrite(
                    b"\t\0" as *const u8 as *const i8 as *const libc::c_void,
                    ::core::mem::size_of::<i8>(),
                    1,
                    stdout,
                );
            }
            fwrite(
                s as *const libc::c_void,
                ::core::mem::size_of::<i8>(),
                l as usize,
                stdout,
            );
            lua_settop(interpreter, -2);
        }
        fwrite(
            b"\n\0" as *const u8 as *const i8 as *const libc::c_void,
            ::core::mem::size_of::<i8>(),
            1,
            stdout,
        );
        fflush(stdout);
        return 0;
    }
}
pub unsafe extern "C" fn luab_warn(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let n: i32 = (*interpreter).get_top();
        lual_checklstring(interpreter, 1, std::ptr::null_mut());
        for i in 2..(1 + n) {
            lual_checklstring(interpreter, i, std::ptr::null_mut());
        }
        for i in 1..n {
            lua_warning(interpreter, lua_tolstring(interpreter, i, std::ptr::null_mut()), 1);
        }
        lua_warning(interpreter, lua_tolstring(interpreter, n, std::ptr::null_mut()), 0);
        return 0;
    }
}
pub unsafe extern "C" fn l_print(interpreter: *mut Interpreter) {
    unsafe {
        let n: i32 = (*interpreter).get_top();
        if n > 0 {
            lual_checkstack(
                interpreter,
                20 as i32,
                b"too many results to print\0" as *const u8 as *const i8,
            );
            lua_getglobal(interpreter, b"print\0" as *const u8 as *const i8);
            lua_rotate(interpreter, 1, 1);
            if lua_pcallk(interpreter, n, 0, 0, 0, None) != 0 {
                l_message(
                    PROGRAM_NAME,
                    lua_pushfstring(
                        interpreter,
                        b"error calling 'print' (%s)\0" as *const u8 as *const i8,
                        lua_tolstring(interpreter, -1, std::ptr::null_mut()),
                    ),
                );
            }
        }
    }
}
pub static mut GLOBAL_STATE: *mut Interpreter = std::ptr::null_mut();
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
pub unsafe extern "C" fn lstop(interpreter: *mut Interpreter, mut _ar: *mut DebugInfo) {
    unsafe {
        lua_sethook(interpreter, None, 0, 0);
        lual_error(interpreter, b"interrupted!\0".as_ptr());
    }
}
pub unsafe extern "C" fn laction(i: i32) {
    unsafe {
        let flag: i32 = 1 << 0 | 1 << 1 | 1 << 2 | 1 << 3;
        setsignal(i, None);
        lua_sethook(
            GLOBAL_STATE,
            Some(lstop as unsafe extern "C" fn(*mut Interpreter, *mut DebugInfo) -> ()),
            flag,
            1,
        );
    }
}
pub unsafe extern "C" fn print_usage(badoption: *const i8) {
    unsafe {
        fprintf(stderr, b"%s: \0" as *const u8 as *const i8, PROGRAM_NAME);
        fflush(stderr);
        if *badoption.offset(1 as isize) as i32 == CHARACTER_LOWER_E as i32
            || *badoption.offset(1 as isize) as i32 == CHARACTER_LOWER_L as i32
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
        b"usage: %s [options] [script [args]]\nAvailable options are:\n  -e stat   execute string 'stat'\n  -i        enter interactive mode after executing 'script'\n  -l mod    require library 'mod' into global 'mod'\n  -l global=mod  require library 'mod' into global CHARACTER_LOWER_G\n  -v        show version information\n  -E        ignore environment variables\n  -W        turn warnings on\n  --        stop handling options\n  -         stop handling options and execute stdin\n\0"
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
pub unsafe extern "C" fn report(interpreter: *mut Interpreter, status: i32) -> i32 {
    unsafe {
        if status != 0 {
            let mut message: *const i8 = lua_tolstring(interpreter, -1, std::ptr::null_mut());
            if message.is_null() {
                message = b"(error message not a string)\0" as *const u8 as *const i8;
            }
            l_message(PROGRAM_NAME, message);
            lua_settop(interpreter, -2);
        }
        return status;
    }
}
pub unsafe extern "C" fn msghandler(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let mut message: *const i8 = lua_tolstring(interpreter, 1, std::ptr::null_mut());
        if message.is_null() {
            if lual_callmeta(interpreter, 1, b"__tostring\0" as *const u8 as *const i8)
                && lua_type(interpreter, -1) == Some(TagType::String)
            {
                return 1;
            } else {
                message = lua_pushfstring(
                    interpreter,
                    b"(error object is a %s value)\0" as *const u8 as *const i8,
                    lua_typename(interpreter, lua_type(interpreter, 1)),
                );
            }
        }
        lual_traceback(interpreter, interpreter, message, 1);
        return 1;
    }
}
pub unsafe extern "C" fn docall(interpreter: *mut Interpreter, narg: i32, nres: i32) -> i32 {
    unsafe {
        let status: i32;
        let base: i32 = (*interpreter).get_top() - narg;
        lua_pushcclosure(
            interpreter,
            Some(msghandler as unsafe extern "C" fn(*mut Interpreter) -> i32),
            0,
        );
        lua_rotate(interpreter, base, 1);
        GLOBAL_STATE = interpreter;
        setsignal(2, Some(laction as unsafe extern "C" fn(i32) -> ()));
        status = lua_pcallk(interpreter, narg, nres, base, 0, None);
        setsignal(2, None);
        lua_rotate(interpreter, base, -1);
        lua_settop(interpreter, -2);
        return status;
    }
}
pub unsafe extern "C" fn createargtable(
    interpreter: *mut Interpreter,
    argv: *mut *mut i8,
    argc: i32,
    script: i32,
) {
    unsafe {
        (*interpreter).lua_createtable();
        for i in 0..argc {
            lua_pushstring(interpreter, *argv.offset(i as isize));
            lua_rawseti(interpreter, -2, (i - script) as i64);
        }
        lua_setglobal(interpreter, b"arg\0" as *const u8 as *const i8);
    }
}
pub unsafe extern "C" fn dochunk(interpreter: *mut Interpreter, mut status: i32) -> i32 {
    unsafe {
        if status == 0 {
            status = docall(interpreter, 0, 0);
        }
        return report(interpreter, status);
    }
}
pub unsafe extern "C" fn dofile(interpreter: *mut Interpreter, name: *const i8) -> i32 {
    unsafe {
        return dochunk(interpreter, lual_loadfilex(interpreter, name, std::ptr::null()));
    }
}
pub unsafe extern "C" fn dostring(interpreter: *mut Interpreter, s: *const i8, name: *const i8) -> i32 {
    unsafe {
        return dochunk(
            interpreter,
            lual_loadbufferx(interpreter, s, strlen(s), name, std::ptr::null()),
        );
    }
}
pub unsafe extern "C" fn dolibrary(interpreter: *mut Interpreter, globname: *mut i8) -> i32 {
    unsafe {
        let status: i32;
        let mut suffix: *mut i8 = std::ptr::null_mut();
        let mut modname: *mut i8 = strchr(globname, CHARACTER_EQUAL as i32);
        if modname.is_null() {
            modname = globname;
            suffix = strchr(modname, *(b"-\0" as *const u8 as *const i8) as i32);
        } else {
            *modname = Character::Null as i8;
            modname = modname.offset(1);
        }
        lua_getglobal(interpreter, b"require\0" as *const u8 as *const i8);
        lua_pushstring(interpreter, modname);
        status = docall(interpreter, 1, 1);
        if status == 0 {
            if !suffix.is_null() {
                *suffix = Character::Null as i8;
            }
            lua_setglobal(interpreter, globname);
        }
        return report(interpreter, status);
    }
}
pub unsafe extern "C" fn pushargs(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let n: i32;
        if lua_getglobal(interpreter, b"arg\0" as *const u8 as *const i8) != 5 {
            lual_error(interpreter, b"'arg' is not a table\0".as_ptr());
        }
        n = lual_len(interpreter, -1) as i32;
        lual_checkstack(
            interpreter,
            n + 3,
            b"too many arguments to script\0" as *const u8 as *const i8,
        );
        for i in 1..(1 + n) {
            lua_rawgeti(interpreter, -i, i as i64);
        }
        lua_rotate(interpreter, -(1 + n), -1);
        lua_settop(interpreter, -2);
        return n;
    }
}
pub unsafe extern "C" fn handle_script(interpreter: *mut Interpreter, argv: *mut *mut i8) -> i32 {
    unsafe {
        let mut status: i32;
        let mut fname: *const i8 = *argv.offset(0 as isize);
        if strcmp(fname, b"-\0" as *const u8 as *const i8) == 0
            && strcmp(*argv.offset(-1 as isize), b"--\0" as *const u8 as *const i8) != 0
        {
            fname = std::ptr::null();
        }
        status = lual_loadfilex(interpreter, fname, std::ptr::null());
        if status == 0 {
            let n: i32 = pushargs(interpreter);
            status = docall(interpreter, n, -1);
        }
        return report(interpreter, status);
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
            if *(*argv.offset(i as isize)).offset(0 as isize) as i32 != CHARACTER_HYPHEN as i32 {
                return args;
            }
            let current_block_31: u64;
            match *(*argv.offset(i as isize)).offset(1 as isize) as i32 {
                45 => {
                    if *(*argv.offset(i as isize)).offset(2 as isize) as i32 != Character::Null as i32 {
                        return 1;
                    }
                    *first = i + 1;
                    return args;
                }
                0 => return args,
                69 => {
                    if *(*argv.offset(i as isize)).offset(2 as isize) as i32 != Character::Null as i32 {
                        return 1;
                    }
                    args |= 16 as i32;
                    current_block_31 = 4761528863920922185;
                }
                87 => {
                    if *(*argv.offset(i as isize)).offset(2 as isize) as i32 != Character::Null as i32 {
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
                    if *(*argv.offset(i as isize)).offset(2 as isize) as i32 != Character::Null as i32 {
                        return 1;
                    }
                    args |= 4;
                }
                15172496195422792753 => {
                    if *(*argv.offset(i as isize)).offset(2 as isize) as i32 == Character::Null as i32 {
                        i += 1;
                        if (*argv.offset(i as isize)).is_null()
                            || *(*argv.offset(i as isize)).offset(0 as isize) as i32 == CHARACTER_HYPHEN as i32
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
pub unsafe extern "C" fn runargs(interpreter: *mut Interpreter, argv: *mut *mut i8, n: i32) -> i32 {
    unsafe {
        for i in 0..n {
            let option: i32 = *(*argv.offset(i as isize)).offset(1 as isize) as i32;
            match option {
                CHARACTER_LOWER_E | CHARACTER_LOWER_L => {
                    let status: i32;
                    let extra: *mut i8 = (*argv.offset(i as isize)).offset(2 as isize);
                    if *extra as i32 == Character::Null as i32 {
                        continue;
                    }
                    status = if option == CHARACTER_LOWER_E as i32 {
                        dostring(interpreter, extra, b"=(command line)\0" as *const u8 as *const i8)
                    } else {
                        dolibrary(interpreter, extra)
                    };
                    if status != 0 {
                        return 0;
                    }
                }
                CHARACTER_UPPER_W => {
                    lua_warning(interpreter, b"@on\0" as *const u8 as *const i8, 0);
                }
                _ => {}
            }
        }
        return 1;
    }
}
pub unsafe extern "C" fn get_prompt(interpreter: *mut Interpreter, firstline: i32) -> *const i8 {
    unsafe {
        if lua_getglobal(
            interpreter,
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
            let p: *const i8 = lual_tolstring(interpreter, -1, std::ptr::null_mut());
            lua_rotate(interpreter, -2, -1);
            lua_settop(interpreter, -2);
            return p;
        };
    }
}
pub unsafe extern "C" fn incomplete(interpreter: *mut Interpreter, status: i32) -> i32 {
    unsafe {
        if status == 3 {
            let mut lmsg: u64 = 0;
            let message: *const i8 = lua_tolstring(interpreter, -1, &mut lmsg);
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
pub unsafe extern "C" fn pushline(interpreter: *mut Interpreter, firstline: i32) -> i32 {
    unsafe {
        let mut buffer: [i8; 512] = [0; 512];
        let b: *mut i8 = buffer.as_mut_ptr();
        let prmt: *const i8 = get_prompt(interpreter, firstline);
        fputs(prmt, stdout);
        fflush(stdout);
        let readstatus: i32 =
            (fgets(b, 512 as i32, stdin) != std::ptr::null_mut() as *mut i8) as i32;
        lua_settop(interpreter, 0);
        if readstatus == 0 {
            return 0;
        }
        let mut l: u64 = strlen(b);
        if l > 0 && *b.offset(l.wrapping_sub(1 as u64) as isize) as i32 == CHARACTER_LF as i32 {
            l = l.wrapping_sub(1);
            *b.offset(l as isize) = Character::Null as i8;
        }
        if firstline != 0 && *b.offset(0 as isize) as i32 == CHARACTER_EQUAL as i32 {
            lua_pushfstring(
                interpreter,
                b"return %s\0" as *const u8 as *const i8,
                b.offset(1 as isize),
            );
        } else {
            lua_pushlstring(interpreter, b, l);
        }
        return 1;
    }
}
pub unsafe extern "C" fn addreturn(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let line: *const i8 = lua_tolstring(interpreter, -1, std::ptr::null_mut());
        let retline: *const i8 =
            lua_pushfstring(interpreter, b"return %s;\0" as *const u8 as *const i8, line);
        let status: i32 = lual_loadbufferx(
            interpreter,
            retline,
            strlen(retline),
            b"=stdin\0" as *const u8 as *const i8,
            std::ptr::null(),
        );
        if status == 0 {
            lua_rotate(interpreter, -2, -1);
            lua_settop(interpreter, -2);
        } else {
            lua_settop(interpreter, -2 - 1);
        }
        return status;
    }
}
pub unsafe extern "C" fn multiline(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        loop {
            let mut length: u64 = 0;
            let line: *const i8 = lua_tolstring(interpreter, 1, &mut length);
            let status: i32 = lual_loadbufferx(
                interpreter,
                line,
                length,
                b"=stdin\0" as *const u8 as *const i8,
                std::ptr::null(),
            );
            if incomplete(interpreter, status) == 0 || pushline(interpreter, 0) == 0 {
                return status;
            }
            lua_rotate(interpreter, -2, -1);
            lua_settop(interpreter, -2);
            lua_pushstring(interpreter, b"\n\0" as *const u8 as *const i8);
            lua_rotate(interpreter, -2, 1);
            lua_concat(interpreter, 3);
        }
    }
}
pub unsafe extern "C" fn loadline(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        lua_settop(interpreter, 0);
        if pushline(interpreter, 1) == 0 {
            return -1;
        }
        let mut status: i32 = addreturn(interpreter);
        if status != 0 {
            status = multiline(interpreter);
        }
        lua_rotate(interpreter, 1, -1);
        lua_settop(interpreter, -2);
        return status;
    }
}
pub unsafe extern "C" fn finishpcall(interpreter: *mut Interpreter, status: i32, extra: i64) -> i32 {
    unsafe {
        if ((status != 0 && status != 1) as i32 != 0) as i64 != 0 {
            (*interpreter).push_boolean(false);
            lua_pushvalue(interpreter, -2);
            return 2;
        } else {
            return (*interpreter).get_top() - extra as i32;
        };
    }
}
pub unsafe extern "C" fn luab_pcall(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let status: i32;
        lual_checkany(interpreter, 1);
        (*interpreter).push_boolean(true);
        lua_rotate(interpreter, 1, 1);
        status = lua_pcallk(
            interpreter,
            (*interpreter).get_top() - 2,
            -1,
            0,
            0,
            Some(finishpcall as unsafe extern "C" fn(*mut Interpreter, i32, i64) -> i32),
        );
        return finishpcall(interpreter, status, 0);
    }
}
pub unsafe extern "C" fn checkstack(interpreter: *mut Interpreter, other_state: *mut Interpreter, n: i32) {
    unsafe {
        if ((interpreter != other_state && lua_checkstack(other_state, n) == 0) as i32 != 0) as i64
            != 0
        {
            lual_error(interpreter, b"stack overflow\0".as_ptr());
        }
    }
}
pub unsafe extern "C" fn getthread(interpreter: *mut Interpreter, arg: *mut i32) -> *mut Interpreter {
    unsafe {
        if lua_type(interpreter, 1) == Some(TagType::State) {
            *arg = 1;
            return lua_tothread(interpreter, 1);
        } else {
            *arg = 0;
            return interpreter;
        };
    }
}
pub unsafe extern "C" fn settabss(interpreter: *mut Interpreter, k: *const i8, v: *const i8) {
    unsafe {
        lua_pushstring(interpreter, v);
        lua_setfield(interpreter, -2, k);
    }
}
pub unsafe extern "C" fn settabsi(interpreter: *mut Interpreter, k: *const i8, v: i32) {
    unsafe {
        (*interpreter).push_integer(v as i64);
        lua_setfield(interpreter, -2, k);
    }
}
pub unsafe extern "C" fn settabsb(interpreter: *mut Interpreter, k: *const i8, v: i32) {
    unsafe {
        (*interpreter).push_boolean(v != 0);
        lua_setfield(interpreter, -2, k);
    }
}
pub unsafe extern "C" fn treatstackoption(
    interpreter: *mut Interpreter,
    other_state: *mut Interpreter,
    fname: *const i8,
) {
    unsafe {
        if interpreter == other_state {
            lua_rotate(interpreter, -2, 1);
        } else {
            lua_xmove(other_state, interpreter, 1);
        }
        lua_setfield(interpreter, -2, fname);
    }
}
pub unsafe extern "C" fn auxupvalue(interpreter: *mut Interpreter, get: i32) -> i32 {
    unsafe {
        let n: i32 = lual_checkinteger(interpreter, 2) as i32;
        lual_checktype(interpreter, 1, TagType::Closure);
        let name: *const i8 = if get != 0 {
            lua_getupvalue(interpreter, 1, n)
        } else {
            lua_setupvalue(interpreter, 1, n)
        };
        if name.is_null() {
            return 0;
        } else {
            lua_pushstring(interpreter, name);
            lua_rotate(interpreter, -(get + 1), 1);
            return get + 1;
        }
    }
}
pub unsafe extern "C" fn checkupval(
    interpreter: *mut Interpreter,
    argf: i32,
    argnup: i32,
    pnup: *mut i32,
) -> *mut libc::c_void {
    unsafe {
        let id: *mut libc::c_void;
        let nup: i32 = lual_checkinteger(interpreter, argnup) as i32;
        lual_checktype(interpreter, argf, TagType::Closure);
        id = lua_upvalueid(interpreter, argf, nup);
        if !pnup.is_null() {
            (((id != std::ptr::null_mut()) as i32 != 0) as i64 != 0
                || lual_argerror(
                    interpreter,
                    argnup,
                    b"invalid upvalue index\0" as *const u8 as *const i8,
                ) != 0) as i32;
            *pnup = nup;
        }
        return id;
    }
}
pub unsafe extern "C" fn hookf(interpreter: *mut Interpreter, ar: *mut DebugInfo) {
    unsafe {
        pub const HOOK_NAMES: [*const i8; 5] = [
            b"call\0" as *const u8 as *const i8,
            b"return\0" as *const u8 as *const i8,
            b"line\0" as *const u8 as *const i8,
            b"count\0" as *const u8 as *const i8,
            b"tail call\0" as *const u8 as *const i8,
        ];
        lua_getfield(interpreter, -(1000000 as i32) - 1000 as i32, HOOKKEY);
        (*interpreter).push_state();
        if lua_rawget(interpreter, -2) == TagType::Closure {
            lua_pushstring(interpreter, HOOK_NAMES[(*ar).event as usize]);
            if (*ar).currentline >= 0 {
                (*interpreter).push_integer((*ar).currentline as i64);
            } else {
                (*interpreter).push_nil();
            }
            lua_callk(interpreter, 2, 0, 0, None);
        }
    }
}
pub unsafe extern "C" fn makemask(smask: *const i8, count: i32) -> i32 {
    unsafe {
        let mut mask: i32 = 0;
        if !(strchr(smask, CHARACTER_LOWER_C as i32)).is_null() {
            mask |= 1 << 0;
        }
        if !(strchr(smask, CHARACTER_LOWER_R as i32)).is_null() {
            mask |= 1 << 1;
        }
        if !(strchr(smask, CHARACTER_LOWER_L as i32)).is_null() {
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
            *smask.offset(fresh190 as isize) = CHARACTER_LOWER_C as i8;
        }
        if mask & 1 << 1 != 0 {
            let fresh191 = i;
            i = i + 1;
            *smask.offset(fresh191 as isize) = CHARACTER_LOWER_R as i8;
        }
        if mask & 1 << 2 != 0 {
            let fresh192 = i;
            i = i + 1;
            *smask.offset(fresh192 as isize) = CHARACTER_LOWER_L as i8;
        }
        *smask.offset(i as isize) = Character::Null as i8;
        return smask;
    }
}
pub const HOOKKEY: *const i8 = b"_HOOKKEY\0" as *const u8 as *const i8;
