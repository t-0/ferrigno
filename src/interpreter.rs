#![allow(unused)]
use std::ptr::*;
use libc::time;
use crate::objectbase::*;
use crate::buffer::*;
use crate::closeprotected::*;
use crate::bufffs::*;
use crate::callinfo::*;
use crate::calls::*;
use crate::character::*;
use crate::closeprotected::*;
use crate::closure::*;
use crate::debuginfo::*;
use crate::dumpstate::*;
use crate::dynamicdata::*;
use crate::f2i::*;
use crate::forloop::*;
use crate::functions::*;
use crate::functionstate::*;
use crate::global::*;
use crate::labeldescription::*;
use crate::lexical::lexicalstate::*;
use crate::loadf::*;
use crate::loadstate::*;
use crate::longjump::*;
use crate::object::*;
use crate::prototype::*;
use crate::registeredfunction::*;
use crate::sparser::*;
use crate::stkidrel::*;
use crate::table::*;
use crate::tag::*;
use crate::tm::*;
use crate::tobjectwithgclist::*;
use crate::token::*;
use crate::tstring::*;
use crate::tvalue::*;
use crate::upvalue::*;
use crate::upvaluedescription::*;
use crate::user::*;
use crate::utility::c::*;
use crate::utility::*;
use crate::variabledescription::*;
use crate::vectort::*;
use crate::vm::opcode::*;
use crate::vm::opmode::*;
use crate::objectwithgclist::*;
use crate::tobject::*;
use crate::zio::*;
use crate::status::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Interpreter {
    pub object: ObjectWithGCList,
    pub status: Status,
    pub allow_hook: u8,
    pub count_call_info: u16,
    pub top: StkIdRel,
    pub global: *mut Global,
    pub callinfo: *mut CallInfo,
    pub stack_last: StkIdRel,
    pub stack: StkIdRel,
    pub open_upvalue: *mut UpValue,
    pub tbc_list: StkIdRel,
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
    fn as_object(&self) -> &ObjectBase {
        self.object.as_object()
    }
    fn as_object_mut(&mut self) -> &mut ObjectBase {
        self.object.as_object_mut()
    }
    fn get_class_name(&mut self) -> String {
        "interpreter".to_string()
    }
}
impl TObjectWithGCList for Interpreter {
    fn getgclist(&mut self) -> *mut *mut ObjectWithGCList {
        self.object.getgclist()
    }
}
impl Interpreter {
    pub unsafe fn preinit_thread(& mut self, global: *mut Global) {
        unsafe {
            self.global = global;
            self.stack.stkidrel_pointer = null_mut();
            self.callinfo = null_mut();
            self.count_call_info = 0;
            self.twups = self as *mut Interpreter;
            self.count_c_calls = 0;
            self.long_jump = null_mut();
            write_volatile(&mut self.hook as *mut HookFunction, None);
            write_volatile(&mut self.hook_mask as *mut i32, 0);
            self.base_hook_count = 0;
            self.allow_hook = 1;
            self.hook_count = self.base_hook_count;
            self.open_upvalue = null_mut();
            self.status = Status::OK;
            self.error_function = 0;
            self.old_program_counter = 0;
        }
    }
    pub fn init(&mut self, global: &Global) {
        self.set_tag_variant(TagVariant::Interpreter);
        self.set_marked(global.global_currentwhite & (1 << 3 | 1 << 4));
    }
    pub unsafe fn lua_callk(& mut self, nargs: i32, count_results: i32, ctx: i64, k: ContextFunction) {
        unsafe {
            let function: *mut TValue = self.top.stkidrel_pointer.offset(-((nargs + 1) as isize));
            if k.is_some() && self.count_c_calls & 0xffff0000 as u32 == 0 {
                (*self.callinfo).call_info_u.c.context_function = k;
                (*self.callinfo).call_info_u.c.context = ctx;
                ccall(self, function, count_results, 1);
            } else {
                luad_callnoyield(self, function, count_results);
            }
            if count_results <= -1 && (*self.callinfo).call_info_top.stkidrel_pointer < self.top.stkidrel_pointer {
                (*self.callinfo).call_info_top.stkidrel_pointer = self.top.stkidrel_pointer;
            }
        }
    }
    pub unsafe fn lual_checktype(& mut self, arg: i32, tagtype: TagType) {
        unsafe {
            if lua_type(self, arg) != Some(tagtype) {
                tag_error2(self, arg, Some(tagtype));
            }
        }
    }
    pub unsafe fn luac_step(&mut self) {
        unsafe {
            (*self.global).luac_step(self);
        }
    }
    pub unsafe fn luac_fullgc(&mut self, is_emergency: bool) {
        unsafe {
            (*self.global).luac_fullgc(self, is_emergency);
        }
    }
    pub unsafe fn luas_init_state(&mut self) {
        unsafe {
            (*self.global).luas_init_global(self);
        }
    }
    pub unsafe fn to_pointer(&mut self, index: i32) -> *mut libc::c_void {
        unsafe { self.index_to_value(index).to_pointer() }
    }
    pub unsafe fn free_interpreter(&mut self, interpreter: *mut Interpreter) {
        unsafe {
            luaf_closeupval(self, self.stack.stkidrel_pointer);
            freestack(self);
            (*interpreter).free_memory(self as *mut Interpreter as *mut libc::c_void, size_of::<Interpreter>());
        }
    }
    pub fn get_status(&mut self) -> Status {
        return self.status;
    }
    pub unsafe fn set_error_object(&mut self, status: Status, old_top: *mut TValue) {
        unsafe {
            match status {
                Status::MemoryError => {
                    let io: *mut TValue = &mut (*old_top);
                    let tstring: *mut TString = (*(self.global)).global_memoryerrormessage;
                    (*io).value.value_object = &mut (*(tstring as *mut ObjectBase));
                    (*io).set_tag_variant((*tstring).get_tag_variant());
                    (*io).set_collectable(true);
                },
                Status::OK => {
                    (*old_top).set_tag_variant(TagVariant::NilNil);
                },
                _ => {
                    let io1: *mut TValue = &mut (*old_top);
                    let io2: *const TValue = &mut (*(self.top.stkidrel_pointer).offset(-(1i32 as isize)));
                    (*io1).copy_from(&*io2);
                },
            }
            self.top.stkidrel_pointer = old_top.offset(1);
        }
    }
    pub unsafe fn correct_stack(&mut self) {
        unsafe {
            (*self).top.stkidrel_pointer = ((*self).stack.stkidrel_pointer as *mut i8).offset((*self).top.stkidrel_offset as isize) as *mut TValue;
            (*self).tbc_list.stkidrel_pointer = ((*self).stack.stkidrel_pointer as *mut i8).offset((*self).tbc_list.stkidrel_offset as isize) as *mut TValue;
            let mut upvalue: *mut UpValue = (*self).open_upvalue;
            while !upvalue.is_null() {
                (*upvalue).v.p = &mut (*(((*self).stack.stkidrel_pointer as *mut i8).offset((*upvalue).v.offset as isize) as *mut TValue));
                upvalue = (*upvalue).u.open.next;
            }
            let mut callinfo = (*self).callinfo;
            while !callinfo.is_null() {
                (*callinfo).call_info_top.stkidrel_pointer = ((*self).stack.stkidrel_pointer as *mut i8).offset((*callinfo).call_info_top.stkidrel_offset as isize) as *mut TValue;
                (*callinfo).call_info_function.stkidrel_pointer = ((*self).stack.stkidrel_pointer as *mut i8).offset((*callinfo).call_info_function.stkidrel_offset as isize) as *mut TValue;
                if (*callinfo).call_info_call_status as i32 & (1i32) << 1i32 == 0 {
                    write_volatile(&mut (*callinfo).call_info_u.l.trap as *mut i32, 1i32);
                }
                callinfo = (*callinfo).call_info_previous;
            }
        }
    }
    pub fn is_yieldable(&mut self) -> bool {
        return self.count_c_calls & 0xffff0000u32 == 0;
    }
    pub unsafe fn push_boolean(&mut self, x: bool) {
        unsafe {
            if x {
                (*self.top.stkidrel_pointer).set_tag_variant(TagVariant::BooleanTrue);
            } else {
                (*self.top.stkidrel_pointer).set_tag_variant(TagVariant::BooleanFalse);
            }
            self.top.stkidrel_pointer = self.top.stkidrel_pointer.offset(1);
        }
    }
    pub unsafe fn push_integer(&mut self, x: i64) {
        unsafe {
            let t_value: *mut TValue = &mut (*self.top.stkidrel_pointer);
            (*t_value).value.value_integer = x;
            (*t_value).set_tag_variant(TagVariant::NumericInteger);
            self.top.stkidrel_pointer = self.top.stkidrel_pointer.offset(1);
        }
    }
    pub unsafe fn push_nil(&mut self) {
        unsafe {
            (*self.top.stkidrel_pointer).set_tag_variant(TagVariant::NilNil);
            self.top.stkidrel_pointer = self.top.stkidrel_pointer.offset(1);
        }
    }
    pub unsafe fn push_number(&mut self, x: f64) {
        unsafe {
            let t_value: *mut TValue = &mut (*self.top.stkidrel_pointer);
            (*t_value).value.value_number = x;
            (*t_value).set_tag_variant(TagVariant::NumericNumber);
            self.top.stkidrel_pointer = self.top.stkidrel_pointer.offset(1);
        }
    }
    pub unsafe fn get_top(&mut self) -> i32 {
        unsafe {
            return self.top.stkidrel_pointer.offset_from(((*self.callinfo).call_info_function.stkidrel_pointer).offset(1 as isize)) as i32;
        }
    }
    pub unsafe fn find_pcall(&mut self) -> *mut CallInfo {
        unsafe {
            let mut it = self.callinfo;
            return loop {
                if it.is_null() {
                    break it;
                } else if ((*it).call_info_call_status & (1 << 4)) != 0 {
                    break it;
                } else {
                    it = (*it).call_info_previous;
                }
            };
        }
    }
    pub unsafe fn sweep_list(&mut self, mut p: *mut *mut ObjectBase, countin: i32, countout: *mut i32) -> *mut *mut ObjectBase {
        unsafe {
            let other_white = (*(self.global)).global_currentwhite ^ (1 << 3 | 1 << 4);
            let mut i: i32;
            let white = (*(self.global)).global_currentwhite & ((1 << 3) | (1 << 4));
            i = 0;
            while !(*p).is_null() && i < countin {
                let curr: *mut ObjectBase = *p;
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
            return if (*p).is_null() { null_mut() } else { p };
        }
    }
    pub unsafe fn free_memory(&mut self, block: *mut libc::c_void, old_size: usize) {
        unsafe {
            raw_allocate(block, old_size, 0);
            (*self.global).global_gcdebt -= old_size as i64;
        }
    }
    pub unsafe fn too_big(&mut self) -> ! {
        unsafe {
            luag_runerror(self, c"memory allocation error: block too big".as_ptr());
        }
    }
    pub unsafe fn push_state(&mut self) -> bool {
        unsafe {
            let io: *mut TValue = &mut (*self.top.stkidrel_pointer);
            (*io).value.value_object = &mut (*(self as *mut Interpreter as *mut ObjectBase));
            (*io).set_tag_variant(TagVariant::Interpreter);
            (*io).set_collectable(true);
            self.top.stkidrel_pointer = self.top.stkidrel_pointer.offset(1);
            return (*self.global).global_maininterpreter == self;
        }
    }
    pub unsafe fn relstack(&mut self) {
        unsafe {
            self.top.stkidrel_offset = (self.top.stkidrel_pointer as *mut i8).offset_from(self.stack.stkidrel_pointer as *mut i8) as i64;
            self.tbc_list.stkidrel_offset = (self.tbc_list.stkidrel_pointer as *mut i8).offset_from(self.stack.stkidrel_pointer as *mut i8) as i64;
            let mut upvalue: *mut UpValue = self.open_upvalue;
            while !upvalue.is_null() {
                (*upvalue).v.offset = ((*upvalue).v.p as *mut TValue as *mut i8).offset_from(self.stack.stkidrel_pointer as *mut i8) as i64;
                upvalue = (*upvalue).u.open.next;
            }
            let mut callinfo = self.callinfo;
            while !callinfo.is_null() {
                (*callinfo).call_info_top.stkidrel_offset = ((*callinfo).call_info_top.stkidrel_pointer as *mut i8).offset_from(self.stack.stkidrel_pointer as *mut i8) as i64;
                (*callinfo).call_info_function.stkidrel_offset = ((*callinfo).call_info_function.stkidrel_pointer as *mut i8).offset_from(self.stack.stkidrel_pointer as *mut i8) as i64;
                callinfo = (*callinfo).call_info_previous;
            }
        }
    }
    pub unsafe fn luad_errerr(&mut self) -> ! {
        unsafe {
            let message: *mut TString = luas_newlstr(self, c"error in error handling".as_ptr(), 23);
            let io: *mut TValue = &mut (*self.top.stkidrel_pointer);
            (*io).value.value_object = &mut (*(message as *mut ObjectBase));
            (*io).set_tag_variant((*message).get_tag_variant());
            (*io).set_collectable(true);
            self.top.stkidrel_pointer = self.top.stkidrel_pointer.offset(1);
            luad_throw(self, Status::GenericError);
        }
    }
    pub unsafe fn luae_checkcstack(&mut self) {
        unsafe {
            if self.count_c_calls & 0xffff as u32 == 200 as u32 {
                luag_runerror(self, c"C stack overflow".as_ptr());
            } else if self.count_c_calls & 0xffff as u32 >= (200 as i32 / 10 as i32 * 11 as i32) as u32 {
                self.luad_errerr();
            }
        }
    }
    pub unsafe fn luae_inccstack(&mut self) {
        unsafe {
            self.count_c_calls = (self.count_c_calls).wrapping_add(1);
            self.count_c_calls;
            if ((self.count_c_calls & 0xffff as u32 >= 200 as u32) as i32 != 0) as i32 as i64 != 0 {
                self.luae_checkcstack();
            }
        }
    }
    pub unsafe fn stackinuse(&mut self) -> i32 {
        unsafe {
            let mut lim = self.top.stkidrel_pointer;
            let mut callinfo = self.callinfo;
            while !callinfo.is_null() {
                if lim < (*callinfo).call_info_top.stkidrel_pointer {
                    lim = (*callinfo).call_info_top.stkidrel_pointer;
                }
                callinfo = (*callinfo).call_info_previous;
            }
            let mut res: i32 = lim.offset_from(self.stack.stkidrel_pointer) as i32 + 1;
            if res < 20 as i32 {
                res = 20 as i32;
            }
            return res;
        }
    }
    pub unsafe fn luad_shrinkstack(&mut self) {
        unsafe {
            let inuse: i32 = self.stackinuse();
            let max: i32 = if inuse > 1000000 / 3 { 1000000 } else { inuse * 3 };
            if inuse <= 1000000 && (self.stack_last.stkidrel_pointer).offset_from(self.stack.stkidrel_pointer) as i32 > max {
                let new_size: i32 = if inuse > 1000000 / 2 { 1000000 } else { inuse * 2 };
                luad_reallocstack(self, new_size, false);
            }
            luae_shrinkci(self);
        }
    }
    pub unsafe fn luad_inctop(&mut self) {
        unsafe {
            if (((self.stack_last.stkidrel_pointer).offset_from(self.top.stkidrel_pointer) as i64 <= 1) as i32 != 0) as i32 as i64 != 0 {
                luad_growstack(self, 1, true);
            }
            self.top.stkidrel_pointer = self.top.stkidrel_pointer.offset(1);
        }
    }
    pub unsafe fn lua_createtable(&mut self) {
        unsafe {
            let table: *mut Table = luah_new(self);
            let io: *mut TValue = &mut (*self.top.stkidrel_pointer);
            (*io).value.value_object = &mut (*(table as *mut ObjectBase));
            (*io).set_tag_variant(TagVariant::Table);
            (*io).set_collectable(true);
            self.top.stkidrel_pointer = self.top.stkidrel_pointer.offset(1);
            if (*self.global).global_gcdebt > 0 {
                (*self.global).luac_step(self);
            }
        }
    }
    pub unsafe fn lua_getmetatable(&mut self, object_index: i32) -> bool {
        unsafe {
            let object: *const TValue = self.index_to_value(object_index);
            let metatable: *mut Table = match (*object).get_tag_variant().to_tag_type() {
                TagType::Table => (*((*object).value.value_object as *mut Table)).get_metatable(),
                TagType::User => (*((*object).value.value_object as *mut User)).get_metatable(),
                _ => (*self.global).global_metatables[(*object).get_tag_variant().to_tag_type() as usize],
            };
            if metatable.is_null() {
                false
            } else {
                let io: *mut TValue = &mut (*self.top.stkidrel_pointer);
                (*io).value.value_object = &mut (*(metatable as *mut ObjectBase));
                (*io).set_tag_variant(TagVariant::Table);
                (*io).set_collectable(true);
                self.top.stkidrel_pointer = self.top.stkidrel_pointer.offset(1);
                true
            }
        }
    }
    pub unsafe fn lua_getiuservalue(&mut self, index: i32, n: i32) -> Option<TagType> {
        unsafe {
            let t: Option<TagType>;
            let tvalue: *mut TValue = self.index_to_value(index);
            if n <= 0 || n > (*((*tvalue).value.value_object as *mut User)).count_upvalues as i32 {
                (*self.top.stkidrel_pointer).set_tag_variant(TagVariant::NilNil);
                t = None;
            } else {
                let io1: *mut TValue = &mut (*self.top.stkidrel_pointer);
                let io2: *const TValue = &mut (*((*((*tvalue).value.value_object as *mut User)).upvalues).as_mut_ptr().offset((n - 1) as isize));
                (*io1).copy_from(&*io2);
                t = Some((*self.top.stkidrel_pointer).get_tag_variant().to_tag_type());
            }
            self.top.stkidrel_pointer = self.top.stkidrel_pointer.offset(1);
            return t;
        }
    }
    pub unsafe fn index_to_value(&mut self, mut index: i32) -> &mut TValue {
        unsafe {
            let callinfo = self.callinfo;
            if index > 0 {
                let o: *mut TValue = ((*callinfo).call_info_function.stkidrel_pointer).offset(index as isize);
                if o >= self.top.stkidrel_pointer {
                    return &mut (*self.global).global_nonevalue;
                } else {
                    return &mut (*o);
                }
            } else if !(index <= -(1000000 as i32) - 1000 as i32) {
                return &mut (*self.top.stkidrel_pointer.offset(index as isize));
            } else if index == -(1000000 as i32) - 1000 as i32 {
                return &mut (*self.global).global_lregistry;
            } else {
                index = -(1000000 as i32) - 1000 as i32 - index;
                let value = *(*callinfo).call_info_function.stkidrel_pointer;
                if value.is_collectable() && value.get_tag_variant() == TagVariant::ClosureC {
                    let function: *mut Closure = &mut (*(value.value.value_object as *mut Closure));
                    return if index <= (*function).count_upvalues as i32 {
                        &mut *((*function).upvalues).c_tvalues.as_mut_ptr().offset((index - 1) as isize) as &mut TValue
                    } else {
                        &mut (*self.global).global_nonevalue
                    };
                } else {
                    return &mut (*self.global).global_nonevalue;
                }
            };
        }
    }
}
pub unsafe fn do_repl(interpreter: *mut Interpreter) {
    unsafe {
        let mut status: Status;
        let oldprogname: *const i8 = PROGRAM_NAME;
        PROGRAM_NAME = null();
        loop {
            status = loadline(interpreter);
            if !(status != Status::Closing) {
                break;
            }
            if status == Status::OK {
                status = docall(interpreter, 0, -1);
            }
            if status == Status::OK {
                l_print(interpreter);
            } else {
                report(interpreter, status);
            }
        }
        lua_settop(interpreter, 0);
        fwrite(c"\n".as_ptr() as *const libc::c_void, 1, 1, stdout);
        fflush(stdout);
        PROGRAM_NAME = oldprogname;
    }
}
pub unsafe fn luad_throw(interpreter: *mut Interpreter, mut status: Status) -> ! {
    unsafe {
        if !((*interpreter).long_jump).is_null() {
            (*(*interpreter).long_jump).status = status;
            _longjmp(((*(*interpreter).long_jump).jbt).as_mut_ptr(), 1);
        } else {
            let global: *mut Global = (*interpreter).global;
            let outerstatus = luae_resetthread(interpreter, status);
            (*interpreter).status = outerstatus;
            if !((*(*global).global_maininterpreter).long_jump).is_null() {
                let fresh0 = (*(*global).global_maininterpreter).top.stkidrel_pointer;
                (*(*global).global_maininterpreter).top.stkidrel_pointer = ((*(*global).global_maininterpreter).top.stkidrel_pointer).offset(1);
                let io1: *mut TValue = &mut (*fresh0);
                let io2: *const TValue = &mut (*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize)));
                (*io1).copy_from(&*io2);
                luad_throw((*global).global_maininterpreter, outerstatus);
            } else {
                if ((*global).global_panic).is_some() {
                    ((*global).global_panic).expect("non-null function pointer")(interpreter);
                }
                abort();
            }
        };
    }
}
pub unsafe fn luad_rawrunprotected(interpreter: *mut Interpreter, f: ProtectedFunction, arbitrary_data: *mut libc::c_void) -> Status {
    unsafe {
        let old_count_c_calls: u32 = (*interpreter).count_c_calls;
        let mut long_jump = LongJump::new();
        write_volatile(&mut long_jump.status as *mut Status as *mut i32, 0);
        long_jump.previous = (*interpreter).long_jump;
        (*interpreter).long_jump = &mut long_jump;
        if _setjmp((long_jump.jbt).as_mut_ptr()) == 0 {
            (Some(f.expect("non-null function pointer"))).expect("non-null function pointer")(interpreter, arbitrary_data);
        }
        (*interpreter).long_jump = long_jump.previous;
        (*interpreter).count_c_calls = old_count_c_calls;
        return long_jump.status;
    }
}
pub unsafe fn luad_reallocstack(interpreter: *mut Interpreter, new_size: i32, should_raise_error: bool) -> i32 {
    unsafe {
        let old_size: i32 = ((*interpreter).stack_last.stkidrel_pointer).offset_from((*interpreter).stack.stkidrel_pointer) as i32;
        let oldgcstop: i32 = (*(*interpreter).global).global_gcstopem as i32;
        (*interpreter).relstack();
        (*(*interpreter).global).global_gcstopem = 1;
        let newstack: *mut TValue = luam_realloc_(
            interpreter,
            (*interpreter).stack.stkidrel_pointer as *mut libc::c_void,
            ((old_size + 5) as usize) * size_of::<TValue>(),
            ((new_size + 5) as usize) * size_of::<TValue>(),
        ) as *mut TValue;
        (*(*interpreter).global).global_gcstopem = oldgcstop as u8;
        if newstack.is_null() {
            (*interpreter).correct_stack();
            if should_raise_error {
                luad_throw(interpreter, Status::MemoryError);
            } else {
                return 0;
            }
        }
        (*interpreter).stack.stkidrel_pointer = newstack;
        (*interpreter).correct_stack();
        (*interpreter).stack_last.stkidrel_pointer = ((*interpreter).stack.stkidrel_pointer).offset(new_size as isize);
        for i in (old_size + 5)..(new_size + 5) {
            (*newstack.offset(i as isize)).set_tag_variant(TagVariant::NilNil);
        }
        return 1;
    }
}
pub unsafe fn luad_growstack(interpreter: *mut Interpreter, n: i32, should_raise_error: bool) -> i32 {
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
            luag_runerror(interpreter, c"stack overflow".as_ptr());
        }
        return 0;
    }
}
pub unsafe fn luad_hook(interpreter: *mut Interpreter, event: i32, line: i32, ftransfer: i32, ntransfer: i32) {
    unsafe {
        let hook: HookFunction = (*interpreter).hook;
        if hook.is_some() && (*interpreter).allow_hook as i32 != 0 {
            let mut mask: i32 = 1 << 3;
            let callinfo = (*interpreter).callinfo;
            let top: i64 = ((*interpreter).top.stkidrel_pointer as *mut i8).offset_from((*interpreter).stack.stkidrel_pointer as *mut i8) as i64;
            let ci_top: i64 = ((*callinfo).call_info_top.stkidrel_pointer as *mut i8).offset_from((*interpreter).stack.stkidrel_pointer as *mut i8) as i64;
            let mut debuginfo: DebugInfo = DebugInfo::new2(event, line, callinfo);
            if ntransfer != 0 {
                mask |= 1 << 8;
                (*callinfo).call_info_u2.transferinfo.ftransfer = ftransfer as u16;
                (*callinfo).call_info_u2.transferinfo.ntransfer = ntransfer as u16;
            }
            if (*callinfo).call_info_call_status as i32 & 1 << 1 == 0 && (*interpreter).top.stkidrel_pointer < (*callinfo).call_info_top.stkidrel_pointer {
                (*interpreter).top.stkidrel_pointer = (*callinfo).call_info_top.stkidrel_pointer;
            }
            if ((((*interpreter).stack_last.stkidrel_pointer).offset_from((*interpreter).top.stkidrel_pointer) as i64 <= 20 as i64) as i32 != 0) as i64 != 0 {
                luad_growstack(interpreter, 20 as i32, true);
            }
            if (*callinfo).call_info_top.stkidrel_pointer < (*interpreter).top.stkidrel_pointer.offset(20 as isize) {
                (*callinfo).call_info_top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(20 as isize);
            }
            (*interpreter).allow_hook = 0;
            (*callinfo).call_info_call_status = ((*callinfo).call_info_call_status as i32 | mask) as u16;
            (Some(hook.expect("non-null function pointer"))).expect("non-null function pointer")(interpreter, &mut debuginfo);
            (*interpreter).allow_hook = 1;
            (*callinfo).call_info_top.stkidrel_pointer = ((*interpreter).stack.stkidrel_pointer as *mut i8).offset(ci_top as isize) as *mut TValue;
            (*interpreter).top.stkidrel_pointer = ((*interpreter).stack.stkidrel_pointer as *mut i8).offset(top as isize) as *mut TValue;
            (*callinfo).call_info_call_status = ((*callinfo).call_info_call_status as i32 & !mask) as u16;
        }
    }
}
pub unsafe fn luad_hookcall(interpreter: *mut Interpreter, callinfo: *mut CallInfo) {
    unsafe {
        (*interpreter).old_program_counter = 0;
        if (*interpreter).hook_mask & (1 << 0) != 0 {
            let event: i32 = if ((*callinfo).call_info_call_status & (1 << 5)) != 0 { 4 } else { 0 };
            let p: *mut Prototype = (*((*(*callinfo).call_info_function.stkidrel_pointer).value.value_object as *mut Closure)).payload.l_prototype;
            (*callinfo).call_info_u.l.saved_program_counter = ((*callinfo).call_info_u.l.saved_program_counter).offset(1);
            (*callinfo).call_info_u.l.saved_program_counter;
            luad_hook(interpreter, event, -1, 1, (*p).prototype_count_parameters as i32);
            (*callinfo).call_info_u.l.saved_program_counter = ((*callinfo).call_info_u.l.saved_program_counter).offset(-1);
            (*callinfo).call_info_u.l.saved_program_counter;
        }
    }
}
pub unsafe fn rethook(interpreter: *mut Interpreter, mut callinfo: *mut CallInfo, nres: i32) {
    unsafe {
        if (*interpreter).hook_mask & 1 << 1 != 0 {
            let firstres: *mut TValue = (*interpreter).top.stkidrel_pointer.offset(-(nres as isize));
            let mut delta: i32 = 0;
            if (*callinfo).call_info_call_status as i32 & 1 << 1 == 0 {
                let p: *mut Prototype = (*((*(*callinfo).call_info_function.stkidrel_pointer).value.value_object as *mut Closure)).payload.l_prototype;
                if (*p).prototype_is_variable_arguments {
                    delta = (*callinfo).call_info_u.l.count_extra_arguments + (*p).prototype_count_parameters as i32 + 1;
                }
            }
            (*callinfo).call_info_function.stkidrel_pointer = ((*callinfo).call_info_function.stkidrel_pointer).offset(delta as isize);
            let ftransfer: i32 = firstres.offset_from((*callinfo).call_info_function.stkidrel_pointer) as i32;
            luad_hook(interpreter, 1, -1, ftransfer, nres);
            (*callinfo).call_info_function.stkidrel_pointer = ((*callinfo).call_info_function.stkidrel_pointer).offset(-(delta as isize));
        }
        callinfo = (*callinfo).call_info_previous;
        if (*callinfo).call_info_call_status as i32 & 1 << 1 == 0 {
            (*interpreter).old_program_counter = ((*callinfo).call_info_u.l.saved_program_counter).offset_from((*(*((*(*callinfo).call_info_function.stkidrel_pointer).value.value_object as *mut Closure)).payload.l_prototype).prototype_code.vectort_pointer) as i32 - 1;
        }
    }
}
pub unsafe fn tryfunctm(interpreter: *mut Interpreter, mut function: *mut TValue) -> *mut TValue {
    unsafe {
        let mut p: *mut TValue;
        if ((((*interpreter).stack_last.stkidrel_pointer).offset_from((*interpreter).top.stkidrel_pointer) as i64 <= 1) as i32 != 0) as i32 as i64 != 0 {
            let t__: i64 = (function as *mut i8).offset_from((*interpreter).stack.stkidrel_pointer as *mut i8) as i64;
            if (*(*interpreter).global).global_gcdebt > 0 {
                (*interpreter).luac_step();
            }
            luad_growstack(interpreter, 1, true);
            function = ((*interpreter).stack.stkidrel_pointer as *mut i8).offset(t__ as isize) as *mut TValue;
        }
        let tm: *const TValue = luat_gettmbyobj(interpreter, &mut (*function), TM_CALL);
        if (*tm).is_tagtype_nil() {
            luag_callerror(interpreter, &mut (*function));
        }
        p = (*interpreter).top.stkidrel_pointer;
        while p > function {
            let io1: *mut TValue = &mut (*p);
            let io2: *const TValue = &mut (*p.offset(-(1 as isize)));
            (*io1).copy_from(&*io2);
            p = p.offset(-1);
        }
        (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
        let io1_0: *mut TValue = &mut (*function);
        (*io1_0).copy_from(&*tm);
        return function;
    }
}
pub unsafe fn moveresults(interpreter: *mut Interpreter, mut res: *mut TValue, mut nres: i32, mut wanted: i32) {
    unsafe {
        let firstresult: *mut TValue;
        match wanted {
            0 => {
                (*interpreter).top.stkidrel_pointer = res;
                return;
            },
            1 => {
                if nres == 0 {
                    (*res).set_tag_variant(TagVariant::NilNil);
                } else {
                    let io1: *mut TValue = &mut (*res);
                    let io2: *const TValue = &mut (*(*interpreter).top.stkidrel_pointer.offset(-(nres as isize)));
                    (*io1).copy_from(&*io2);
                }
                (*interpreter).top.stkidrel_pointer = res.offset(1 as isize);
                return;
            },
            -1 => {
                wanted = nres;
            },
            _ => {
                if wanted < -1 {
                    (*(*interpreter).callinfo).call_info_call_status = ((*(*interpreter).callinfo).call_info_call_status as i32 | 1 << 9 as i32) as u16;
                    (*(*interpreter).callinfo).call_info_u2.nres = nres;
                    res = luaf_close(interpreter, res, Status::Closing, 1);
                    (*(*interpreter).callinfo).call_info_call_status = ((*(*interpreter).callinfo).call_info_call_status as i32 & !(1 << 9 as i32)) as u16;
                    if (*interpreter).hook_mask != 0 {
                        let savedres: i64 = (res as *mut i8).offset_from((*interpreter).stack.stkidrel_pointer as *mut i8) as i64;
                        rethook(interpreter, (*interpreter).callinfo, nres);
                        res = ((*interpreter).stack.stkidrel_pointer as *mut i8).offset(savedres as isize) as *mut TValue;
                    }
                    wanted = -wanted - 3;
                    if wanted == -1 {
                        wanted = nres;
                    }
                }
            },
        }
        firstresult = (*interpreter).top.stkidrel_pointer.offset(-(nres as isize));
        if nres > wanted {
            nres = wanted;
        }
        for i in 0..nres {
            let io1_0: *mut TValue = &mut (*res.offset(i as isize));
            let io2_0: *const TValue = &mut (*firstresult.offset(i as isize));
            (*io1_0).copy_from(&*io2_0);
        }
        for i in nres..wanted {
            (*res.offset(i as isize)).set_tag_variant(TagVariant::NilNil);
        }
        (*interpreter).top.stkidrel_pointer = res.offset(wanted as isize);
    }
}
pub unsafe fn luad_poscall(interpreter: *mut Interpreter, callinfo: *mut CallInfo, nres: i32) {
    unsafe {
        let wanted: i32 = (*callinfo).call_info_count_results as i32;
        if (*interpreter).hook_mask != 0 && (wanted >= -1) {
            rethook(interpreter, callinfo, nres);
        }
        moveresults(interpreter, (*callinfo).call_info_function.stkidrel_pointer, nres, wanted);
        (*interpreter).callinfo = (*callinfo).call_info_previous;
    }
}
pub unsafe fn prepcallinfo(interpreter: *mut Interpreter, function: *mut TValue, nret: i32, mask: i32, top: *mut TValue) -> *mut CallInfo {
    unsafe {
        (*interpreter).callinfo = if !((*(*interpreter).callinfo).call_info_next).is_null() { (*(*interpreter).callinfo).call_info_next } else { luae_extendci(interpreter) };
        let callinfo = (*interpreter).callinfo;
        (*callinfo).call_info_function.stkidrel_pointer = function;
        (*callinfo).call_info_count_results = nret;
        (*callinfo).call_info_call_status = mask as u16;
        (*callinfo).call_info_top.stkidrel_pointer = top;
        return callinfo;
    }
}
pub unsafe fn precallc(interpreter: *mut Interpreter, mut function: *mut TValue, count_results: i32, cfunction: CFunction) -> i32 {
    unsafe {
        if ((((*interpreter).stack_last.stkidrel_pointer).offset_from((*interpreter).top.stkidrel_pointer) as i64 <= 20 as i64) as i32 != 0) as i64 != 0 {
            let t__: i64 = (function as *mut i8).offset_from((*interpreter).stack.stkidrel_pointer as *mut i8) as i64;
            if (*(*interpreter).global).global_gcdebt > 0 {
                (*interpreter).luac_step();
            }
            luad_growstack(interpreter, 20 as i32, true);
            function = ((*interpreter).stack.stkidrel_pointer as *mut i8).offset(t__ as isize) as *mut TValue;
        }
        let callinfo = prepcallinfo(interpreter, function, count_results, 1 << 1, (*interpreter).top.stkidrel_pointer.offset(20 as isize));
        (*interpreter).callinfo = callinfo;
        if ((*interpreter).hook_mask & 1 << 0 != 0) as i64 != 0 {
            let narg: i32 = ((*interpreter).top.stkidrel_pointer).offset_from(function) as i32 - 1;
            luad_hook(interpreter, 0, -1, 1, narg);
        }
        let n: i32 = (Some(cfunction.expect("non-null function pointer"))).expect("non-null function pointer")(interpreter);
        luad_poscall(interpreter, callinfo, n);
        return n;
    }
}
pub unsafe fn luad_pretailcall(interpreter: *mut Interpreter, callinfo: *mut CallInfo, mut function: *mut TValue, mut narg1: i32, delta: i32) -> i32 {
    unsafe {
        loop {
            match (*function).get_tag_variant() {
                TagVariant::ClosureC => {
                    return precallc(interpreter, function, -1, (*((*function).value.value_object as *mut Closure)).payload.c_cfunction);
                },
                TagVariant::ClosureCFunction => {
                    return precallc(interpreter, function, -1, (*function).value.value_function);
                },
                TagVariant::ClosureL => {
                    let p: *mut Prototype = (*((*function).value.value_object as *mut Closure)).payload.l_prototype;
                    let fsize: i32 = (*p).prototype_maximum_stack_size as i32;
                    let nfixparams: i32 = (*p).prototype_count_parameters as i32;
                    if ((((*interpreter).stack_last.stkidrel_pointer).offset_from((*interpreter).top.stkidrel_pointer) as i64 <= (fsize - delta) as i64) as i32 != 0) as i64 != 0 {
                        let t__: i64 = (function as *mut i8).offset_from((*interpreter).stack.stkidrel_pointer as *mut i8) as i64;
                        if (*(*interpreter).global).global_gcdebt > 0 {
                            (*interpreter).luac_step();
                        }
                        luad_growstack(interpreter, fsize - delta, true);
                        function = ((*interpreter).stack.stkidrel_pointer as *mut i8).offset(t__ as isize) as *mut TValue;
                    }
                    (*callinfo).call_info_function.stkidrel_pointer = ((*callinfo).call_info_function.stkidrel_pointer).offset(-(delta as isize));
                    for i in 0..narg1 {
                        let io1: *mut TValue = &mut (*((*callinfo).call_info_function.stkidrel_pointer).offset(i as isize));
                        let io2: *const TValue = &mut (*function.offset(i as isize));
                        (*io1).copy_from(&*io2);
                    }
                    function = (*callinfo).call_info_function.stkidrel_pointer;
                    while narg1 <= nfixparams {
                        (*function.offset(narg1 as isize)).set_tag_variant(TagVariant::NilNil);
                        narg1 += 1;
                    }
                    (*callinfo).call_info_top.stkidrel_pointer = function.offset(1 as isize).offset(fsize as isize);
                    (*callinfo).call_info_u.l.saved_program_counter = (*p).prototype_code.vectort_pointer;
                    (*callinfo).call_info_call_status = ((*callinfo).call_info_call_status as i32 | 1 << 5) as u16;
                    (*interpreter).top.stkidrel_pointer = function.offset(narg1 as isize);
                    return -1;
                },
                _ => {
                    function = tryfunctm(interpreter, function);
                    narg1 += 1;
                },
            }
        }
    }
}
pub unsafe fn luad_precall(interpreter: *mut Interpreter, mut function: *mut TValue, count_results: i32) -> *mut CallInfo {
    unsafe {
        loop {
            match (*function).get_tag_variant() {
                TagVariant::ClosureC => {
                    precallc(interpreter, function, count_results, (*((*function).value.value_object as *mut Closure)).payload.c_cfunction);
                    return null_mut();
                },
                TagVariant::ClosureCFunction => {
                    precallc(interpreter, function, count_results, (*function).value.value_function);
                    return null_mut();
                },
                TagVariant::ClosureL => {
                    let callinfo;
                    let p: *mut Prototype = (*((*function).value.value_object as *mut Closure)).payload.l_prototype;
                    let mut narg: i32 = ((*interpreter).top.stkidrel_pointer).offset_from(function) as i32 - 1;
                    let nfixparams: i32 = (*p).prototype_count_parameters as i32;
                    let fsize: i32 = (*p).prototype_maximum_stack_size as i32;
                    if ((((*interpreter).stack_last.stkidrel_pointer).offset_from((*interpreter).top.stkidrel_pointer) as i64 <= fsize as i64) as i32 != 0) as i64 != 0 {
                        let t__: i64 = (function as *mut i8).offset_from((*interpreter).stack.stkidrel_pointer as *mut i8) as i64;
                        if (*(*interpreter).global).global_gcdebt > 0 {
                            (*interpreter).luac_step();
                        }
                        luad_growstack(interpreter, fsize, true);
                        function = ((*interpreter).stack.stkidrel_pointer as *mut i8).offset(t__ as isize) as *mut TValue;
                    }
                    callinfo = prepcallinfo(interpreter, function, count_results, 0, function.offset(1 as isize).offset(fsize as isize));
                    (*interpreter).callinfo = callinfo;
                    (*callinfo).call_info_u.l.saved_program_counter = (*p).prototype_code.vectort_pointer;
                    while narg < nfixparams {
                        let fresh1 = (*interpreter).top.stkidrel_pointer;
                        (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
                        (*fresh1).set_tag_variant(TagVariant::NilNil);
                        narg += 1;
                    }
                    return callinfo;
                },
                _ => {
                    function = tryfunctm(interpreter, function);
                },
            }
        }
    }
}
pub unsafe fn ccall(interpreter: *mut Interpreter, mut function: *mut TValue, count_results: i32, inc: u32) {
    unsafe {
        let callinfo;
        (*interpreter).count_c_calls = ((*interpreter).count_c_calls as u32).wrapping_add(inc) as u32;
        if (((*interpreter).count_c_calls & 0xffff as u32 >= 200 as u32) as i32 != 0) as i32 as i64 != 0 {
            if ((((*interpreter).stack_last.stkidrel_pointer).offset_from((*interpreter).top.stkidrel_pointer) as i64 <= 0) as i32 != 0) as i64 != 0 {
                let t__: i64 = (function as *mut i8).offset_from((*interpreter).stack.stkidrel_pointer as *mut i8) as i64;
                luad_growstack(interpreter, 0, true);
                function = ((*interpreter).stack.stkidrel_pointer as *mut i8).offset(t__ as isize) as *mut TValue;
            }
            (*interpreter).luae_checkcstack();
        }
        callinfo = luad_precall(interpreter, function, count_results);
        if !callinfo.is_null() {
            (*callinfo).call_info_call_status = (1 << 2) as u16;
            luav_execute(interpreter, callinfo);
        }
        (*interpreter).count_c_calls -= inc;
    }
}
pub unsafe fn luad_callnoyield(interpreter: *mut Interpreter, function: *mut TValue, count_results: i32) {
    unsafe {
        ccall(interpreter, function, count_results, (0x10000 as i32 | 1) as u32);
    }
}
pub unsafe fn finishpcallk(interpreter: *mut Interpreter, callinfo: *mut CallInfo) -> Status {
    unsafe {
        let mut status: Status = Status::from((*callinfo).call_info_call_status as i32 >> 10 as i32 & 7);
        if status == Status::OK {
            status = Status::Yield;
        } else {
            let mut function: *mut TValue = ((*interpreter).stack.stkidrel_pointer as *mut i8).offset((*callinfo).call_info_u2.funcidx as isize) as *mut TValue;
            (*interpreter).allow_hook = ((*callinfo).call_info_call_status as i32 & 1 << 0) as u8;
            function = luaf_close(interpreter, function, status, 1);
            (*interpreter).set_error_object(status, function);
            (*interpreter).luad_shrinkstack();
            (*callinfo).call_info_call_status = ((*callinfo).call_info_call_status as i32 & !((7) << 10 as i32) | 0 << 10 as i32) as u16;
        }
        (*callinfo).call_info_call_status = ((*callinfo).call_info_call_status as i32 & !(1 << 4)) as u16;
        (*interpreter).error_function = (*callinfo).call_info_u.c.old_error_function;
        return status;
    }
}
pub unsafe fn finishccall(interpreter: *mut Interpreter, callinfo: *mut CallInfo) {
    unsafe {
        let n: i32;
        if (*callinfo).call_info_call_status as i32 & 1 << 9 as i32 != 0 {
            n = (*callinfo).call_info_u2.nres;
        } else {
            let mut status = Status::Yield;
            if (*callinfo).call_info_call_status as i32 & 1 << 4 != 0 {
                status = finishpcallk(interpreter, callinfo);
            }
            if -1 <= -1 && (*(*interpreter).callinfo).call_info_top.stkidrel_pointer < (*interpreter).top.stkidrel_pointer {
                (*(*interpreter).callinfo).call_info_top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer;
            }
            n = (Some(((*callinfo).call_info_u.c.context_function).expect("non-null function pointer"))).expect("non-null function pointer")(interpreter, status, (*callinfo).call_info_u.c.context);
        }
        luad_poscall(interpreter, callinfo, n);
    }
}
pub unsafe fn unroll(interpreter: *mut Interpreter, mut _ud: *mut libc::c_void) {
    unsafe {
        let mut callinfo;
        loop {
            callinfo = (*interpreter).callinfo;
            if !(callinfo != &mut (*interpreter).base_callinfo as *mut CallInfo) {
                break;
            }
            if (*callinfo).call_info_call_status as i32 & 1 << 1 != 0 {
                finishccall(interpreter, callinfo);
            } else {
                luav_finishop(interpreter);
                luav_execute(interpreter, callinfo);
            }
        }
    }
}
pub unsafe fn resume_error(interpreter: *mut Interpreter, message: *const i8, narg: i32) -> Status {
    unsafe {
        (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(-(narg as isize));
        let io: *mut TValue = &mut (*(*interpreter).top.stkidrel_pointer);
        let tstring: *mut TString = luas_new(interpreter, message);
        (*io).value.value_object = &mut (*(tstring as *mut ObjectBase));
        (*io).set_tag_variant((*tstring).get_tag_variant());
        (*io).set_collectable(true);
        (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
        return Status::RuntimeError;
    }
}
pub unsafe fn resume(interpreter: *mut Interpreter, arbitrary_data: *mut libc::c_void) {
    unsafe {
        let mut n: i32 = *(arbitrary_data as *mut i32);
        let first_argument: *mut TValue = (*interpreter).top.stkidrel_pointer.offset(-(n as isize));
        let callinfo = (*interpreter).callinfo;
        if (*interpreter).status == Status::OK {
            ccall(interpreter, first_argument.offset(-(1 as isize)), -1, 0);
        } else {
            (*interpreter).status = Status::OK;
            if (*callinfo).call_info_call_status as i32 & 1 << 1 == 0 {
                (*callinfo).call_info_u.l.saved_program_counter = ((*callinfo).call_info_u.l.saved_program_counter).offset(-1);
                (*callinfo).call_info_u.l.saved_program_counter;
                (*interpreter).top.stkidrel_pointer = first_argument;
                luav_execute(interpreter, callinfo);
            } else {
                if ((*callinfo).call_info_u.c.context_function).is_some() {
                    n = (Some(((*callinfo).call_info_u.c.context_function).expect("non-null function pointer"))).expect("non-null function pointer")(interpreter, Status::Yield, (*callinfo).call_info_u.c.context);
                }
                luad_poscall(interpreter, callinfo, n);
            }
            unroll(interpreter, null_mut());
        };
    }
}
pub unsafe fn precover(interpreter: *mut Interpreter, mut status: Status) -> Status {
    unsafe {
        let mut callinfo;
        while status.is_error() && {
            callinfo = (*interpreter).find_pcall();
            !callinfo.is_null()
        } {
            (*interpreter).callinfo = callinfo;
            (*callinfo).call_info_call_status = ((*callinfo).call_info_call_status as i32 & !((7) << 10 as i32) | (status as i32) << 10 as i32) as u16;
            status = luad_rawrunprotected(interpreter, Some(unroll as unsafe fn(*mut Interpreter, *mut libc::c_void) -> ()), null_mut());
        }
        return status;
    }
}
pub unsafe fn lua_resume(interpreter: *mut Interpreter, from: *mut Interpreter, mut nargs: i32, count_results: *mut i32) -> Status {
    unsafe {
        let mut status;
        if (*interpreter).status == Status::OK {
            if (*interpreter).callinfo != &mut (*interpreter).base_callinfo as *mut CallInfo {
                return resume_error(interpreter, c"cannot resume non-suspended coroutine".as_ptr(), nargs);
            } else if ((*interpreter).top.stkidrel_pointer).offset_from(((*(*interpreter).callinfo).call_info_function.stkidrel_pointer).offset(1 as isize)) as i64 == nargs as i64 {
                return resume_error(interpreter, c"cannot resume dead coroutine".as_ptr(), nargs);
            }
        } else if (*interpreter).status != Status::Yield {
            return resume_error(interpreter, c"cannot resume dead coroutine".as_ptr(), nargs);
        }
        (*interpreter).count_c_calls = if !from.is_null() { (*from).count_c_calls & 0xffff as u32 } else { 0 };
        if (*interpreter).count_c_calls & 0xffff as u32 >= 200 as u32 {
            return resume_error(interpreter, c"C stack overflow".as_ptr(), nargs);
        }
        (*interpreter).count_c_calls = ((*interpreter).count_c_calls).wrapping_add(1);
        (*interpreter).count_c_calls;
        status = luad_rawrunprotected(interpreter, Some(resume as unsafe fn(*mut Interpreter, *mut libc::c_void) -> ()), &mut nargs as *mut i32 as *mut libc::c_void);
        status = precover(interpreter, status);
        if status.is_error() {
            (*interpreter).status = status;
            (*interpreter).set_error_object(status, (*interpreter).top.stkidrel_pointer);
            (*(*interpreter).callinfo).call_info_top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer;
        }
        *count_results = if status == Status::Yield {
            (*(*interpreter).callinfo).call_info_u2.nyield
        } else {
            ((*interpreter).top.stkidrel_pointer).offset_from(((*(*interpreter).callinfo).call_info_function.stkidrel_pointer).offset(1 as isize)) as i32
        };
        return status;
    }
}
pub unsafe fn lua_yieldk(interpreter: *mut Interpreter, count_results: i32, ctx: i64, k: ContextFunction) -> i32 {
    unsafe {
        let callinfo;
        callinfo = (*interpreter).callinfo;
        if (*interpreter).count_c_calls & 0xffff0000 as u32 != 0 {
            if interpreter != (*(*interpreter).global).global_maininterpreter {
                luag_runerror(interpreter, c"attempt to yield across a C-call boundary".as_ptr());
            } else {
                luag_runerror(interpreter, c"attempt to yield from outside a coroutine".as_ptr());
            }
        }
        (*interpreter).status = Status::Yield;
        (*callinfo).call_info_u2.nyield = count_results;
        if (*callinfo).call_info_call_status as i32 & 1 << 1 == 0 {
        } else {
            (*callinfo).call_info_u.c.context_function = k;
            if ((*callinfo).call_info_u.c.context_function).is_some() {
                (*callinfo).call_info_u.c.context = ctx;
            }
            luad_throw(interpreter, Status::Yield);
        }
        return 0;
    }
}
pub unsafe fn luad_pcall(interpreter: *mut Interpreter, function: ProtectedFunction, u: *mut libc::c_void, old_top: i64, ef: i64) -> Status {
    unsafe {
        let old_call_info = (*interpreter).callinfo;
        let old_allowhooks: u8 = (*interpreter).allow_hook;
        let old_error_function: i64 = (*interpreter).error_function;
        (*interpreter).error_function = ef;
        let mut status = luad_rawrunprotected(interpreter, function, u);
        if status != Status::OK {
            (*interpreter).callinfo = old_call_info;
            (*interpreter).allow_hook = old_allowhooks;
            status = do_close_protected(interpreter, old_top, status);
            (*interpreter).set_error_object(status, ((*interpreter).stack.stkidrel_pointer as *mut i8).offset(old_top as isize) as *mut TValue);
            (*interpreter).luad_shrinkstack();
        }
        (*interpreter).error_function = old_error_function;
        return status;
    }
}
pub unsafe fn checkmode(interpreter: *mut Interpreter, mode: *const i8, x: *const i8) {
    unsafe {
        if !mode.is_null() && (strchr(mode, *x.offset(0) as i32)).is_null() {
            luao_pushfstring(interpreter, c"attempt to load a %s chunk (mode is '%s')".as_ptr(), x, mode);
            luad_throw(interpreter, Status::SyntaxError);
        }
    }
}
pub unsafe fn f_parser(interpreter: *mut Interpreter, arbitrary_data: *mut libc::c_void) {
    unsafe {
        let sparser = arbitrary_data as *mut SParser;
        let ch: i32 = (*(*sparser).zio).get_char();
        let closure = if ch == (*::core::mem::transmute::<&[u8; 5], &[i8; 5]>(b"\x1BLua\0"))[0] as i32 {
            checkmode(interpreter, (*sparser).mode, c"binary".as_ptr());
            load_closure(interpreter, (*sparser).zio, (*sparser).name)
        } else {
            checkmode(interpreter, (*sparser).mode, c"text".as_ptr());
            luay_parser(interpreter, (*sparser).zio, &mut (*sparser).buffer, &mut (*sparser).dynamic_data, (*sparser).name, ch)
        };
        luaf_initupvals(interpreter, closure);
    }
}
pub unsafe fn luad_protectedparser(interpreter: *mut Interpreter, zio: *mut ZIO, name: *const i8, mode: *const i8) -> Status {
    unsafe {
        let mut sparser = SParser::new(zio, name, mode);
        (*interpreter).count_c_calls = ((*interpreter).count_c_calls as u32).wrapping_add(0x10000 as u32) as u32;
        sparser.dynamic_data.active_variables.initialize();
        sparser.dynamic_data.goto_.initialize();
        sparser.dynamic_data.labels.initialize();
        sparser.buffer.loads.initialize();
        let status = luad_pcall(
            interpreter,
            Some(f_parser as unsafe fn(*mut Interpreter, *mut libc::c_void) -> ()),
            &mut sparser as *mut SParser as *mut libc::c_void,
            ((*interpreter).top.stkidrel_pointer as *mut i8).offset_from((*interpreter).stack.stkidrel_pointer as *mut i8) as i64,
            (*interpreter).error_function,
        );
        sparser.buffer.loads.destroy(interpreter);
        (*interpreter).free_memory(
            sparser.dynamic_data.active_variables.vectort_pointer as *mut libc::c_void,
            (sparser.dynamic_data.active_variables.get_size() as usize).wrapping_mul(size_of::<VariableDescription>() as usize) as usize,
        );
        (*interpreter).free_memory(
            sparser.dynamic_data.goto_.vectort_pointer as *mut libc::c_void,
            (sparser.dynamic_data.goto_.get_size() as usize).wrapping_mul(size_of::<LabelDescription>() as usize) as usize,
        );
        (*interpreter).free_memory(
            sparser.dynamic_data.labels.vectort_pointer as *mut libc::c_void,
            (sparser.dynamic_data.labels.get_size() as usize).wrapping_mul(size_of::<LabelDescription>() as usize) as usize,
        );
        (*interpreter).count_c_calls = ((*interpreter).count_c_calls as u32).wrapping_sub(0x10000 as u32) as u32;
        return status;
    }
}
pub unsafe fn index2stack(interpreter: *mut Interpreter, index: i32) -> *mut TValue {
    unsafe {
        let callinfo = (*interpreter).callinfo;
        if index > 0 {
            let o: *mut TValue = ((*callinfo).call_info_function.stkidrel_pointer).offset(index as isize);
            return o;
        } else {
            return (*interpreter).top.stkidrel_pointer.offset(index as isize);
        };
    }
}
pub unsafe fn lua_checkstack(interpreter: *mut Interpreter, n: i32) -> i32 {
    unsafe {
        let res: i32;
        let callinfo;
        callinfo = (*interpreter).callinfo;
        if ((*interpreter).stack_last.stkidrel_pointer).offset_from((*interpreter).top.stkidrel_pointer) as i64 > n as i64 {
            res = 1;
        } else {
            res = luad_growstack(interpreter, n, false);
        }
        if res != 0 && (*callinfo).call_info_top.stkidrel_pointer < (*interpreter).top.stkidrel_pointer.offset(n as isize) {
            (*callinfo).call_info_top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(n as isize);
        }
        return res;
    }
}
pub unsafe fn lua_xmove(from: *mut Interpreter, to: *mut Interpreter, n: i32) {
    unsafe {
        if from != to {
            (*from).top.stkidrel_pointer = ((*from).top.stkidrel_pointer).offset(-(n as isize));
            for i in 0..n {
                let io1: *mut TValue = &mut (*(*to).top.stkidrel_pointer);
                let io2: *const TValue = &mut (*((*from).top.stkidrel_pointer).offset(i as isize));
                (*io1).copy_from(&*io2);
                (*to).top.stkidrel_pointer = ((*to).top.stkidrel_pointer).offset(1);
                (*to).top.stkidrel_pointer;
            }
        }
    }
}
pub unsafe fn lua_atpanic(interpreter: *mut Interpreter, panicf: CFunction) -> CFunction {
    unsafe {
        let old: CFunction = (*(*interpreter).global).global_panic;
        (*(*interpreter).global).global_panic = panicf;
        return old;
    }
}
pub unsafe fn lua_absindex(interpreter: *mut Interpreter, index: i32) -> i32 {
    unsafe {
        return if index > 0 || index <= -(1000000 as i32) - 1000 as i32 {
            index
        } else {
            ((*interpreter).top.stkidrel_pointer).offset_from((*(*interpreter).callinfo).call_info_function.stkidrel_pointer) as i32 + index
        };
    }
}
pub unsafe fn lua_settop(interpreter: *mut Interpreter, index: i32) {
    unsafe {
        let callinfo;
        let mut newtop;
        let mut diff;
        callinfo = (*interpreter).callinfo;
        let function: *mut TValue = (*callinfo).call_info_function.stkidrel_pointer;
        if index >= 0 {
            diff = function.offset(1 as isize).offset(index as isize).offset_from((*interpreter).top.stkidrel_pointer) as i64;
            while diff > 0 {
                let fresh4 = (*interpreter).top.stkidrel_pointer;
                (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
                (*fresh4).set_tag_variant(TagVariant::NilNil);
                diff -= 1;
            }
        } else {
            diff = (index + 1) as i64;
        }
        newtop = (*interpreter).top.stkidrel_pointer.offset(diff as isize);
        if diff < 0 && (*interpreter).tbc_list.stkidrel_pointer >= newtop {
            newtop = luaf_close(interpreter, newtop, Status::Closing, 0);
        }
        (*interpreter).top.stkidrel_pointer = newtop;
    }
}
pub unsafe fn lua_closeslot(interpreter: *mut Interpreter, index: i32) {
    unsafe {
        let mut level = index2stack(interpreter, index);
        level = luaf_close(interpreter, level, Status::Closing, 0);
        (*level).set_tag_variant(TagVariant::NilNil);
    }
}
pub unsafe fn reverse(mut _state: *mut Interpreter, mut from: *mut TValue, mut to: *mut TValue) {
    unsafe {
        while from < to {
            let mut temp: TValue = TValue::new(TagVariant::NilNil);
            let io1: *mut TValue = &mut temp;
            let io2: *const TValue = &mut (*from);
            (*io1).copy_from(&*io2);
            let io1_0: *mut TValue = &mut (*from);
            let io2_0: *const TValue = &mut (*to);
            (*io1_0).copy_from(&*io2_0);
            let io1_1: *mut TValue = &mut (*to);
            let io2_1: *const TValue = &mut temp;
            (*io1_1).copy_from(&*io2_1);
            from = from.offset(1);
            to = to.offset(-1);
        }
    }
}
pub unsafe fn lua_rotate(interpreter: *mut Interpreter, index: i32, n: i32) {
    unsafe {
        let high: *mut TValue = (*interpreter).top.stkidrel_pointer.offset(-(1 as isize));
        let low: *mut TValue = index2stack(interpreter, index);
        let middle: *mut TValue = if n >= 0 { high.offset(-(n as isize)) } else { low.offset(-(n as isize)).offset(-(1 as isize)) };
        reverse(interpreter, low, middle);
        reverse(interpreter, middle.offset(1 as isize), high);
        reverse(interpreter, low, high);
    }
}
pub unsafe fn lua_copy(interpreter: *mut Interpreter, fromidx: i32, toidx: i32) {
    unsafe {
        let fr: *mut TValue = (*interpreter).index_to_value(fromidx);
        let to: *mut TValue = (*interpreter).index_to_value(toidx);
        let io1: *mut TValue = to;
        let io2: *const TValue = fr;
        (*io1).copy_from(&*io2);
        if toidx < -(1000000 as i32) - 1000 as i32 {
            if (*fr).is_collectable() {
                if (*((*(*(*interpreter).callinfo).call_info_function.stkidrel_pointer).value.value_object as *mut Closure)).get_marked() & 1 << 5 != 0 && (*(*fr).value.value_object).get_marked() & (1 << 3 | 1 << 4) != 0 {
                    luac_barrier_(
                        interpreter,
                        &mut (*(&mut (*((*(*(*interpreter).callinfo).call_info_function.stkidrel_pointer).value.value_object)))),
                        &mut (*((*fr).value.value_object as *mut ObjectBase)),
                    );
                } else {
                };
            } else {
            };
        }
    }
}
pub unsafe fn lua_pushvalue(interpreter: *mut Interpreter, index: i32) {
    unsafe {
        let io1: *mut TValue = &mut (*(*interpreter).top.stkidrel_pointer);
        let io2: *const TValue = (*interpreter).index_to_value(index);
        (*io1).copy_from(&*io2);
        (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
    }
}
pub unsafe fn lua_type(interpreter: *mut Interpreter, index: i32) -> Option<TagType> {
    unsafe {
        let tvalue: *const TValue = (*interpreter).index_to_value(index);
        return if !(*tvalue).is_tagtype_nil() || tvalue != &mut (*(*interpreter).global).global_nonevalue as *mut TValue as *const TValue {
            return Some((*tvalue).get_tag_variant().to_tag_type());
        } else {
            None
        };
    }
}
pub unsafe fn lua_typename(mut _state: *mut Interpreter, t: Option<TagType>) -> *const i8 {
    match t {
        None => c"no value".as_ptr(),
        Some(TagType::Nil) => c"nil".as_ptr(),
        Some(TagType::Boolean) => c"boolean".as_ptr(),
        Some(TagType::Pointer) => c"userdata".as_ptr(),
        Some(TagType::Numeric) => c"number".as_ptr(),
        Some(TagType::String) => c"string".as_ptr(),
        Some(TagType::Table) => c"table".as_ptr(),
        Some(TagType::Closure) => c"function".as_ptr(),
        Some(TagType::User) => c"userdata".as_ptr(),
        Some(TagType::Interpreter) => c"thread".as_ptr(),
        Some(TagType::UpValue) => c"upvalue".as_ptr(),
        Some(TagType::Prototype) => c"proto".as_ptr(),
        _ => c"unknown".as_ptr(),
    }
}
pub unsafe fn lua_iscfunction(interpreter: *mut Interpreter, index: i32) -> bool {
    unsafe {
        let o: *const TValue = (*interpreter).index_to_value(index);
        match (*o).get_tag_variant() {
            TagVariant::ClosureCFunction => return true,
            TagVariant::ClosureC => return true,
            _ => return false,
        }
    }
}
pub unsafe fn lua_isinteger(interpreter: *mut Interpreter, index: i32) -> bool {
    unsafe {
        return (*(*interpreter).index_to_value(index)).get_tag_variant() == TagVariant::NumericInteger;
    }
}
pub unsafe fn lua_isnumber(interpreter: *mut Interpreter, index: i32) -> bool {
    unsafe {
        let o: *const TValue = (*interpreter).index_to_value(index);
        return if (*o).get_tag_variant() == TagVariant::NumericNumber {
            true
        } else {
            let mut n: f64 = 0.0;
            (*o).to_number(&mut n)
        };
    }
}
pub unsafe fn lua_isstring(interpreter: *mut Interpreter, index: i32) -> bool {
    unsafe {
        let tvalue: *const TValue = (*interpreter).index_to_value(index);
        match (*tvalue).get_tag_variant().to_tag_type() {
            TagType::Numeric => true,
            TagType::String => true,
            _ => false,
        }
    }
}
pub unsafe fn lua_rawequal(interpreter: *mut Interpreter, index1: i32, index2: i32) -> bool {
    unsafe {
        let o1: *const TValue = (*interpreter).index_to_value(index1);
        let o2: *const TValue = (*interpreter).index_to_value(index2);
        return if (!((*o1).is_tagtype_nil()) || o1 != &mut (*(*interpreter).global).global_nonevalue as *mut TValue as *const TValue) && (!((*o2).is_tagtype_nil()) || o2 != &mut (*(*interpreter).global).global_nonevalue as *mut TValue as *const TValue) {
            luav_equalobj(null_mut(), o1, o2)
        } else {
            false
        };
    }
}
pub unsafe fn lua_compare(interpreter: *mut Interpreter, index1: i32, index2: i32, op: i32) -> bool {
    unsafe {
        let o1: *const TValue = (*interpreter).index_to_value(index1);
        let o2: *const TValue = (*interpreter).index_to_value(index2);
        let mut ret: bool = false;
        if (!((*o1).is_tagtype_nil()) || o1 != &mut (*(*interpreter).global).global_nonevalue as *mut TValue as *const TValue) && (!((*o2).is_tagtype_nil()) || o2 != &mut (*(*interpreter).global).global_nonevalue as *mut TValue as *const TValue) {
            match op {
                0 => {
                    ret = luav_equalobj(interpreter, o1, o2);
                },
                1 => {
                    ret = luav_lessthan(interpreter, o1, o2);
                },
                2 => {
                    ret = luav_lessequal(interpreter, o1, o2);
                },
                _ => {},
            }
        }
        return ret;
    }
}
pub unsafe fn lua_stringtonumber(interpreter: *mut Interpreter, s: *const i8) -> usize {
    unsafe {
        let size: usize = luao_str2num(s, &mut (*(*interpreter).top.stkidrel_pointer));
        if size != 0 {
            (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
        }
        return size;
    }
}
pub unsafe fn lua_tonumberx(interpreter: *mut Interpreter, index: i32, is_number: *mut bool) -> f64 {
    unsafe {
        let mut n: f64 = 0.0;
        let o: *const TValue = (*interpreter).index_to_value(index);
        let is_number_: bool = if (*o).get_tag_variant() == TagVariant::NumericNumber {
            n = (*o).value.value_number;
            true
        } else {
            (*o).to_number(&mut n)
        };
        if !is_number.is_null() {
            *is_number = is_number_;
        }
        return n;
    }
}
pub unsafe fn lua_tointegerx(interpreter: *mut Interpreter, index: i32, is_number: *mut bool) -> i64 {
    unsafe {
        let mut res: i64 = 0;
        let o: *const TValue = (*interpreter).index_to_value(index);
        let is_number_: bool = if (*o).get_tag_variant() == TagVariant::NumericInteger {
            res = (*o).value.value_integer;
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
pub unsafe fn lua_toboolean(interpreter: *mut Interpreter, index: i32) -> bool {
    unsafe {
        let o: *const TValue = (*interpreter).index_to_value(index);
        return !((*o).get_tag_variant() == TagVariant::BooleanFalse || (*o).is_tagtype_nil());
    }
}
pub unsafe fn lua_tolstring(interpreter: *mut Interpreter, index: i32, length: *mut usize) -> *const i8 {
    unsafe {
        let mut o: *mut TValue = (*interpreter).index_to_value(index);
        if !((*o).is_tagtype_string()) {
            if !((*o).is_tagtype_numeric()) {
                if !length.is_null() {
                    *length = 0;
                }
                return null();
            }
            (*o).from_interpreter_to_string(interpreter);
            if (*(*interpreter).global).global_gcdebt > 0 {
                (*interpreter).luac_step();
            }
            o = (*interpreter).index_to_value(index);
        }
        if !length.is_null() {
            *length = (*((*o).value.value_object as *mut TString)).get_length() as usize;
        }
        return (*((*o).value.value_object as *mut TString)).get_contents_mut();
    }
}
pub unsafe fn get_length_raw(interpreter: *mut Interpreter, index: i32) -> usize {
    unsafe {
        let tvalue: *const TValue = (*interpreter).index_to_value(index);
        match (*tvalue).get_tag_variant() {
            TagVariant::StringShort | TagVariant::StringLong => {
                return (*((*tvalue).value.value_object as *mut TString)).get_length() as usize;
            },
            TagVariant::User => return (*((*tvalue).value.value_object as *mut User)).count_bytes,
            TagVariant::Table => {
                return luah_getn(&mut (*((*tvalue).value.value_object as *mut Table))) as usize;
            },
            _ => return 0,
        };
    }
}
pub unsafe fn lua_tothread(interpreter: *mut Interpreter, index: i32) -> *mut Interpreter {
    unsafe {
        let o: *const TValue = (*interpreter).index_to_value(index);
        return if !((*o).get_tag_variant() == TagVariant::Interpreter) { null_mut() } else { &mut (*((*o).value.value_object as *mut Interpreter)) };
    }
}
pub unsafe fn lua_pushlstring(interpreter: *mut Interpreter, s: *const i8, length: usize) -> *const i8 {
    unsafe {
        let tstring: *mut TString = if length == 0 { luas_new(interpreter, c"".as_ptr()) } else { luas_newlstr(interpreter, s, length as usize) };
        let io: *mut TValue = &mut (*(*interpreter).top.stkidrel_pointer);
        (*io).value.value_object = &mut (*(tstring as *mut ObjectBase));
        (*io).set_tag_variant((*tstring).get_tag_variant());
        (*io).set_collectable(true);
        (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
        if (*(*interpreter).global).global_gcdebt > 0 {
            (*interpreter).luac_step();
        }
        return (*tstring).get_contents_mut();
    }
}
pub unsafe fn lua_pushstring(interpreter: *mut Interpreter, mut s: *const i8) -> *const i8 {
    unsafe {
        if s.is_null() {
            (*(*interpreter).top.stkidrel_pointer).set_tag_variant(TagVariant::NilNil);
        } else {
            let tstring: *mut TString = luas_new(interpreter, s);
            let io: *mut TValue = &mut (*(*interpreter).top.stkidrel_pointer);
            (*io).value.value_object = &mut (*(tstring as *mut ObjectBase));
            (*io).set_tag_variant((*tstring).get_tag_variant());
            (*io).set_collectable(true);
            s = (*tstring).get_contents_mut();
        }
        (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
        if (*(*interpreter).global).global_gcdebt > 0 {
            (*interpreter).luac_step();
        }
        return s;
    }
}
pub unsafe fn lua_pushvfstring(interpreter: *mut Interpreter, fmt: *const i8, mut argp: ::core::ffi::VaList) -> *const i8 {
    unsafe {
        let ret: *const i8 = luao_pushvfstring(interpreter, fmt, argp.as_va_list());
        if (*(*interpreter).global).global_gcdebt > 0 {
            (*interpreter).luac_step();
        }
        return ret;
    }
}
pub unsafe extern "C" fn lua_pushfstring(interpreter: *mut Interpreter, fmt: *const i8, args: ...) -> *const i8 {
    unsafe {
        let mut argp: ::core::ffi::VaListImpl;
        argp = args.clone();
        let ret: *const i8 = luao_pushvfstring(interpreter, fmt, argp.as_va_list());
        if (*(*interpreter).global).global_gcdebt > 0 {
            (*interpreter).luac_step();
        }
        return ret;
    }
}
pub unsafe fn lua_pushcclosure(interpreter: *mut Interpreter, fn_0: CFunction, mut n: i32) {
    unsafe {
        if n == 0 {
            let io: *mut TValue = &mut (*(*interpreter).top.stkidrel_pointer);
            (*io).value.value_function = fn_0;
            (*io).set_tag_variant(TagVariant::ClosureCFunction);
            (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
        } else {
            let closure: *mut Closure = luaf_newcclosure(interpreter, n);
            (*closure).payload.c_cfunction = fn_0;
            (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(-(n as isize));
            loop {
                let fresh5 = n;
                n = n - 1;
                if !(fresh5 != 0) {
                    break;
                }
                let io1: *mut TValue = &mut *((*closure).upvalues).c_tvalues.as_mut_ptr().offset(n as isize) as *mut TValue;
                let io2: *const TValue = &mut (*(*interpreter).top.stkidrel_pointer.offset(n as isize));
                (*io1).copy_from(&*io2);
            }
            let io_0: *mut TValue = &mut (*(*interpreter).top.stkidrel_pointer);
            let x_: *mut Closure = closure;
            (*io_0).value.value_object = &mut (*(x_ as *mut ObjectBase));
            (*io_0).set_tag_variant(TagVariant::ClosureC);
            (*io_0).set_collectable(true);
            (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
            if (*(*interpreter).global).global_gcdebt > 0 {
                (*interpreter).luac_step();
            }
        };
    }
}
pub unsafe fn lua_pushlightuserdata(interpreter: *mut Interpreter, p: *mut libc::c_void) {
    unsafe {
        let io: *mut TValue = &mut (*(*interpreter).top.stkidrel_pointer);
        (*io).value.value_pointer = p;
        (*io).set_tag_variant(TagVariant::Pointer);
        (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
    }
}
pub unsafe fn auxgetstr(interpreter: *mut Interpreter, t: *const TValue, k: *const i8) -> TagType {
    unsafe {
        let slot: *const TValue;
        let str: *mut TString = luas_new(interpreter, k);
        if if !((*t).get_tag_variant() == TagVariant::Table) {
            slot = null();
            0
        } else {
            slot = luah_getstr(&mut (*((*t).value.value_object as *mut Table)), str);
            !(*slot).is_tagtype_nil() as i32
        } != 0
        {
            let io1: *mut TValue = &mut (*(*interpreter).top.stkidrel_pointer);
            let io2: *const TValue = slot;
            (*io1).copy_from(&*io2);
            (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
        } else {
            let io: *mut TValue = &mut (*(*interpreter).top.stkidrel_pointer);
            (*io).value.value_object = &mut (*(str as *mut ObjectBase));
            (*io).set_tag_variant((*str).get_tag_variant());
            (*io).set_collectable(true);
            (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
            luav_finishget(
                interpreter,
                t,
                &mut (*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))),
                (*interpreter).top.stkidrel_pointer.offset(-(1 as isize)),
                slot,
            );
        }
        return (*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))).get_tag_variant().to_tag_type();
    }
}
pub unsafe fn lua_getglobal(interpreter: *mut Interpreter, name: *const i8) -> TagType {
    unsafe {
        let global_table: *const TValue = &mut *((*((*(*interpreter).global).global_lregistry.value.value_object as *mut Table)).array).offset((2 - 1) as isize) as *mut TValue;
        return auxgetstr(interpreter, global_table, name);
    }
}
pub unsafe fn lua_gettable(interpreter: *mut Interpreter, index: i32) -> i32 {
    unsafe {
        let slot;
        let t: *mut TValue = (*interpreter).index_to_value(index);
        if if (*t).get_tag_variant() != TagVariant::Table {
            slot = null();
            0
        } else {
            slot = luah_get(&mut (*((*t).value.value_object as *mut Table)), &mut (*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))));
            !(*slot).is_tagtype_nil() as i32
        } != 0
        {
            let io1: *mut TValue = &mut (*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize)));
            let io2: *const TValue = slot;
            (*io1).copy_from(&*io2);
        } else {
            luav_finishget(
                interpreter,
                t,
                &mut (*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))),
                (*interpreter).top.stkidrel_pointer.offset(-(1 as isize)),
                slot,
            );
        }
        return ((*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))).get_tag_variant()).to_tag_type() as i32;
    }
}
pub unsafe fn handle_luainit(interpreter: *mut Interpreter) -> Status {
    unsafe {
        let mut name: *const i8 = c"=LUA_INIT_5_4".as_ptr();
        let mut initial: *const i8 = getenv(name.offset(1 as isize));
        if initial.is_null() {
            name = c"=LUA_INIT".as_ptr();
            initial = getenv(name.offset(1 as isize));
        }
        if initial.is_null() {
            return Status::OK;
        } else if *initial.offset(0 as isize) as i32 == Character::At as i32 {
            return dofile(interpreter, initial.offset(1 as isize));
        } else {
            return dostring(interpreter, initial, name);
        };
    }
}
pub unsafe fn lua_getfield(interpreter: *mut Interpreter, index: i32, k: *const i8) -> TagType {
    unsafe {
        return auxgetstr(interpreter, (*interpreter).index_to_value(index), k);
    }
}
pub unsafe fn lua_geti(interpreter: *mut Interpreter, index: i32, n: i64) -> TagType {
    unsafe {
        let t: *mut TValue;
        let slot: *const TValue;
        t = (*interpreter).index_to_value(index);
        if if (*t).get_tag_variant() != TagVariant::Table {
            slot = null();
            0
        } else {
            slot = if (n as usize).wrapping_sub(1 as usize) < (*((*t).value.value_object as *mut Table)).array_limit as usize {
                &mut *((*((*t).value.value_object as *mut Table)).array).offset((n - 1) as isize) as *mut TValue as *const TValue
            } else {
                luah_getint(&mut (*((*t).value.value_object as *mut Table)), n)
            };
            !(*slot).is_tagtype_nil() as i32
        } != 0
        {
            let io1: *mut TValue = &mut (*(*interpreter).top.stkidrel_pointer);
            let io2: *const TValue = slot;
            (*io1).copy_from(&*io2);
        } else {
            let mut aux: TValue = TValue::new(TagVariant::NilNil);
            let io: *mut TValue = &mut aux;
            (*io).value.value_integer = n;
            (*io).set_tag_variant(TagVariant::NumericInteger);
            luav_finishget(interpreter, t, &mut aux, (*interpreter).top.stkidrel_pointer, slot);
        }
        (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
        return (*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))).get_tag_variant().to_tag_type();
    }
}
pub unsafe fn finishrawget(interpreter: *mut Interpreter, value: *const TValue) -> TagType {
    unsafe {
        if (*value).is_tagtype_nil() {
            (*(*interpreter).top.stkidrel_pointer).set_tag_variant(TagVariant::NilNil);
        } else {
            let io1: *mut TValue = &mut (*(*interpreter).top.stkidrel_pointer);
            let io2: *const TValue = value;
            (*io1).copy_from(&*io2);
        }
        (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
        return (*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))).get_tag_variant().to_tag_type();
    }
}
pub unsafe fn gettable(interpreter: *mut Interpreter, index: i32) -> *mut Table {
    unsafe {
        let t: *mut TValue = (*interpreter).index_to_value(index);
        return &mut (*((*t).value.value_object as *mut Table));
    }
}
pub unsafe fn lua_rawget(interpreter: *mut Interpreter, index: i32) -> TagType {
    unsafe {
        let table: *mut Table = gettable(interpreter, index);
        let value: *const TValue = luah_get(table, &mut (*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))));
        (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(-1);
        return finishrawget(interpreter, value);
    }
}
pub unsafe fn lua_rawgeti(interpreter: *mut Interpreter, index: i32, n: i64) -> TagType {
    unsafe {
        let table: *mut Table = gettable(interpreter, index);
        return finishrawget(interpreter, luah_getint(table, n));
    }
}
pub unsafe fn auxsetstr(interpreter: *mut Interpreter, t: *const TValue, k: *const i8) {
    unsafe {
        let slot: *const TValue;
        let str: *mut TString = luas_new(interpreter, k);
        if if !((*t).get_tag_variant() == TagVariant::Table) {
            slot = null();
            0
        } else {
            slot = luah_getstr(&mut (*((*t).value.value_object as *mut Table)), str);
            !(*slot).is_tagtype_nil() as i32
        } != 0
        {
            let io1: *mut TValue = slot as *mut TValue;
            let io2: *const TValue = &mut (*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize)));
            (*io1).copy_from(&*io2);
            if (*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))).is_collectable() {
                if (*(*t).value.value_object).get_marked() & 1 << 5 != 0 && (*(*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))).value.value_object).get_marked() & (1 << 3 | 1 << 4) != 0 {
                    luac_barrierback_(interpreter, (*t).value.value_object as *mut ObjectWithGCList);
                } else {
                };
            } else {
            };
            (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(-1);
        } else {
            let io: *mut TValue = &mut (*(*interpreter).top.stkidrel_pointer);
            (*io).value.value_object = &mut (*(str as *mut ObjectBase));
            (*io).set_tag_variant((*str).get_tag_variant());
            (*io).set_collectable(true);
            (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
            luav_finishset(
                interpreter,
                t,
                &mut (*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))),
                &mut (*(*interpreter).top.stkidrel_pointer.offset(-(2 as isize))),
                slot,
            );
            (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(-(2 as isize));
        };
    }
}
pub unsafe fn lua_setglobal(interpreter: *mut Interpreter, name: *const i8) {
    unsafe {
        let global_table: *const TValue = &mut *((*((*(*interpreter).global).global_lregistry.value.value_object as *mut Table)).array).offset((2 - 1) as isize) as *mut TValue;
        auxsetstr(interpreter, global_table, name);
    }
}
pub unsafe fn lua_setfield(interpreter: *mut Interpreter, index: i32, k: *const i8) {
    unsafe {
        auxsetstr(interpreter, (*interpreter).index_to_value(index), k);
    }
}
pub unsafe fn lua_seti(interpreter: *mut Interpreter, index: i32, n: i64) {
    unsafe {
        let t: *mut TValue;
        let slot: *const TValue;
        t = (*interpreter).index_to_value(index);
        if if !((*t).get_tag_variant() == TagVariant::Table) {
            slot = null();
            0
        } else {
            slot = if (n as usize).wrapping_sub(1 as usize) < (*((*t).value.value_object as *mut Table)).array_limit as usize {
                &mut *((*((*t).value.value_object as *mut Table)).array).offset((n - 1) as isize) as *mut TValue as *const TValue
            } else {
                luah_getint(&mut (*((*t).value.value_object as *mut Table)), n)
            };
            !(*slot).is_tagtype_nil() as i32
        } != 0
        {
            let io1: *mut TValue = slot as *mut TValue;
            let io2: *const TValue = &mut (*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize)));
            (*io1).copy_from(&*io2);
            if (*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))).is_collectable() {
                if (*(*t).value.value_object).get_marked() & 1 << 5 != 0 && (*(*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))).value.value_object).get_marked() & (1 << 3 | 1 << 4) != 0 {
                    luac_barrierback_(interpreter, (*t).value.value_object as *mut ObjectWithGCList);
                } else {
                };
            } else {
            };
        } else {
            let mut aux: TValue = TValue::new(TagVariant::NumericInteger);
            aux.value.value_integer = n;
            luav_finishset(interpreter, t, &mut aux, &mut (*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))), slot);
        }
        (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(-1);
    }
}
pub unsafe fn aux_rawset(interpreter: *mut Interpreter, index: i32, key: *mut TValue, n: i32) {
    unsafe {
        let table: *mut Table = gettable(interpreter, index);
        luah_set(interpreter, table, key, &mut (*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))));
        (*table).flags = ((*table).flags as u32 & !!(!0 << TM_EQ as i32 + 1)) as u8;
        if (*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))).is_collectable() {
            if (*(table as *mut ObjectBase)).get_marked() & 1 << 5 != 0 && (*(*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))).value.value_object).get_marked() & (1 << 3 | 1 << 4) != 0 {
                luac_barrierback_(interpreter, &mut (*(table as *mut ObjectWithGCList)));
            } else {
            };
        } else {
        };
        (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(-(n as isize));
    }
}
pub unsafe fn lua_rawset(interpreter: *mut Interpreter, index: i32) {
    unsafe {
        aux_rawset(interpreter, index, &mut (*(*interpreter).top.stkidrel_pointer.offset(-(2 as isize))), 2);
    }
}
pub unsafe fn lua_rawseti(interpreter: *mut Interpreter, index: i32, n: i64) {
    unsafe {
        let table: *mut Table = gettable(interpreter, index);
        luah_setint(interpreter, table, n, &mut (*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))));
        if (*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))).is_collectable() {
            if (*(table as *mut ObjectBase)).get_marked() & 1 << 5 != 0 && (*(*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))).value.value_object).get_marked() & (1 << 3 | 1 << 4) != 0 {
                luac_barrierback_(interpreter, &mut (*(table as *mut ObjectWithGCList)));
            } else {
            };
        } else {
        };
        (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(-1);
    }
}
pub unsafe fn lua_setmetatable(interpreter: *mut Interpreter, index: i32) -> i32 {
    unsafe {
        let metatable: *mut Table;
        let object: *mut TValue = (*interpreter).index_to_value(index);
        if (*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))).is_tagtype_nil() {
            metatable = null_mut();
        } else {
            metatable = &mut (*((*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))).value.value_object as *mut Table))
        }
        match (*object).get_tag_variant().to_tag_type() {
            TagType::Table => {
                (*((*object).value.value_object as *mut Table)).set_metatable(metatable);
                if !metatable.is_null() {
                    if (*(*object).value.value_object).get_marked() & 1 << 5 != 0 && (*metatable).get_marked() & (1 << 3 | 1 << 4) != 0 {
                        luac_barrier_(interpreter, &mut (*((*object).value.value_object as *mut ObjectBase)), &mut (*(metatable as *mut ObjectBase)));
                    } else {
                    };
                    luac_checkfinalizer(interpreter, (*object).value.value_object, metatable);
                }
            },
            TagType::User => {
                (*((*object).value.value_object as *mut User)).set_metatable(metatable);
                if !metatable.is_null() {
                    if (*((*object).value.value_object as *mut User)).get_marked() & 1 << 5 != 0 && (*metatable).get_marked() & (1 << 3 | 1 << 4) != 0 {
                        luac_barrier_(interpreter, &mut (*(&mut (*((*object).value.value_object as *mut User)) as *mut User as *mut ObjectBase)), &mut (*(metatable as *mut ObjectBase)));
                    } else {
                    };
                    luac_checkfinalizer(interpreter, (*object).value.value_object, metatable);
                }
            },
            _ => {
                (*(*interpreter).global).global_metatables[(*object).get_tag_variant().to_tag_type() as usize] = metatable;
            },
        }
        (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(-1);
        return 1;
    }
}
pub unsafe fn lua_setiuservalue(interpreter: *mut Interpreter, index: i32, n: i32) -> i32 {
    unsafe {
        let res: i32;
        let o: *mut TValue = (*interpreter).index_to_value(index);
        if !((n as u32).wrapping_sub(1 as u32) < (*((*o).value.value_object as *mut User)).count_upvalues as u32) {
            res = 0;
        } else {
            let io1: *mut TValue = &mut (*((*((*o).value.value_object as *mut User)).upvalues).as_mut_ptr().offset((n - 1) as isize));
            let io2: *const TValue = &mut (*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize)));
            (*io1).copy_from(&*io2);
            if (*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))).is_collectable() {
                if (*(*o).value.value_object).get_marked() & 1 << 5 != 0 && (*(*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))).value.value_object).get_marked() & (1 << 3 | 1 << 4) != 0 {
                    luac_barrierback_(interpreter, (*o).value.value_object as *mut ObjectWithGCList);
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
pub unsafe fn lua_load(interpreter: *mut Interpreter, reader: Reader, data: *mut libc::c_void, mut chunkname: *const i8, mode: *const i8) -> Status {
    unsafe {
        let mut zio: ZIO = ZIO::new(interpreter, reader, data);
        if chunkname.is_null() {
            chunkname = c"?".as_ptr();
        }
        let status = luad_protectedparser(interpreter, &mut zio, chunkname, mode);
        if status == Status::OK {
            let closure: *mut Closure = &mut (*((*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))).value.value_object as *mut Closure));
            if (*closure).count_upvalues as i32 >= 1 {
                let gt: *const TValue = &mut *((*((*(*interpreter).global).global_lregistry.value.value_object as *mut Table)).array).offset((2 - 1) as isize) as *mut TValue;
                let io1: *mut TValue = (**((*closure).upvalues).l_upvalues.as_mut_ptr().offset(0 as isize)).v.p;
                (*io1).copy_from(&*gt);
                if (*gt).is_collectable() {
                    if (**((*closure).upvalues).l_upvalues.as_mut_ptr().offset(0 as isize)).get_marked() & 1 << 5 != 0 && (*(*gt).value.value_object).get_marked() & (1 << 3 | 1 << 4) != 0 {
                        luac_barrier_(
                            interpreter,
                            &mut (*(*((*closure).upvalues).l_upvalues.as_mut_ptr().offset(0 as isize) as *mut ObjectBase)),
                            &mut (*((*gt).value.value_object as *mut ObjectBase)),
                        );
                    } else {
                    }
                } else {
                }
            }
        }
        return status;
    }
}
pub unsafe fn lua_dump(interpreter: *mut Interpreter, writer_0: WriteFunction, data: *mut libc::c_void, is_strip: bool) -> i32 {
    unsafe {
        let status: i32;
        let o: *mut TValue = &mut (*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize)));
        if (*o).get_tag_variant() == TagVariant::ClosureL {
            status = save_prototype(interpreter, (*((*o).value.value_object as *mut Closure)).payload.l_prototype, writer_0, data, is_strip);
        } else {
            status = 1;
        }
        return status;
    }
}
pub unsafe fn lua_error(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let errobj: *mut TValue = &mut (*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize)));
        if (*errobj).get_tag_variant() == TagVariant::StringShort && &mut (*((*errobj).value.value_object as *mut TString)) as *mut TString == (*(*interpreter).global).global_memoryerrormessage {
            luad_throw(interpreter, Status::MemoryError);
        } else {
            luag_errormsg(interpreter);
        };
    }
}
pub unsafe fn lua_next(interpreter: *mut Interpreter, index: i32) -> i32 {
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
pub unsafe fn lua_toclose(interpreter: *mut Interpreter, index: i32) {
    unsafe {
        let o: *mut TValue = index2stack(interpreter, index);
        let count_results: i32 = (*(*interpreter).callinfo).call_info_count_results as i32;
        luaf_newtbcupval(interpreter, o);
        if !(count_results < -1) {
            (*(*interpreter).callinfo).call_info_count_results = -count_results - 3;
        }
    }
}
pub unsafe fn lua_concat(interpreter: *mut Interpreter, n: i32) {
    unsafe {
        if n > 0 {
            concatenate(interpreter, n);
        } else {
            let io: *mut TValue = &mut (*(*interpreter).top.stkidrel_pointer);
            let tstring: *mut TString = luas_newlstr(interpreter, c"".as_ptr(), 0);
            (*io).value.value_object = &mut (*(tstring as *mut ObjectBase));
            (*io).set_tag_variant((*tstring).get_tag_variant());
            (*io).set_collectable(true);
            (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
        }
        if (*(*interpreter).global).global_gcdebt > 0 {
            (*interpreter).luac_step();
        }
    }
}
pub unsafe fn lua_len(interpreter: *mut Interpreter, index: i32) {
    unsafe {
        let t: *mut TValue = (*interpreter).index_to_value(index);
        luav_objlen(interpreter, (*interpreter).top.stkidrel_pointer, t);
        (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
    }
}
pub unsafe fn lua_setwarnf(interpreter: *mut Interpreter, f: WarnFunction, arbitrary_data: *mut libc::c_void) {
    unsafe {
        (*(*interpreter).global).global_warnuserdata = arbitrary_data;
        (*(*interpreter).global).global_warnfunction = f;
    }
}
pub unsafe fn lua_warning(interpreter: *mut Interpreter, message: *const i8, tocont: i32) {
    unsafe {
        luae_warning(interpreter, message, tocont);
    }
}
pub unsafe fn lua_getupvalue(interpreter: *mut Interpreter, funcindex: i32, n: i32) -> *const i8 {
    unsafe {
        let mut value: *mut TValue = null_mut();
        let name: *const i8 = aux_upvalue((*interpreter).index_to_value(funcindex), n, &mut value, null_mut());
        if !name.is_null() {
            let io1: *mut TValue = &mut (*(*interpreter).top.stkidrel_pointer);
            let io2: *const TValue = value;
            (*io1).copy_from(&*io2);
            (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
        }
        return name;
    }
}
pub unsafe fn lua_setupvalue(interpreter: *mut Interpreter, funcindex: i32, n: i32) -> *const i8 {
    unsafe {
        let mut value: *mut TValue = null_mut();
        let mut owner: *mut ObjectBase = null_mut();
        let fi: *mut TValue = (*interpreter).index_to_value(funcindex);
        let name: *const i8 = aux_upvalue(fi, n, &mut value, &mut owner);
        if !name.is_null() {
            (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(-1);
            let io1: *mut TValue = value;
            let io2: *const TValue = &mut (*(*interpreter).top.stkidrel_pointer);
            (*io1).copy_from(&*io2);
            if (*value).is_collectable() {
                if (*owner).get_marked() & 1 << 5 != 0 && (*(*value).value.value_object).get_marked() & (1 << 3 | 1 << 4) != 0 {
                    luac_barrier_(interpreter, &mut (*(owner as *mut ObjectBase)), &mut (*((*value).value.value_object as *mut ObjectBase)));
                } else {
                };
            } else {
            };
        }
        return name;
    }
}
pub const NULLUP: *const UpValue = null();
pub unsafe fn getupvalref(interpreter: *mut Interpreter, fidx: i32, n: i32, pf: *mut *mut Closure) -> *mut *mut UpValue {
    unsafe {
        let fi: *mut TValue = (*interpreter).index_to_value(fidx);
        let closure: *mut Closure = &mut (*((*fi).value.value_object as *mut Closure));
        if !pf.is_null() {
            *pf = closure;
        }
        if 1 <= n && n <= (*(*closure).payload.l_prototype).prototype_upvalues.get_size() as i32 {
            return &mut *((*closure).upvalues).l_upvalues.as_mut_ptr().offset((n - 1) as isize) as *mut *mut UpValue;
        } else {
            return &NULLUP as *const *const UpValue as *mut *mut UpValue;
        };
    }
}
pub unsafe fn lua_upvalueid(interpreter: *mut Interpreter, fidx: i32, n: i32) -> *mut libc::c_void {
    unsafe {
        let fi: *mut TValue = (*interpreter).index_to_value(fidx);
        match (*fi).get_tag_variant() {
            TagVariant::ClosureL => {
                return *getupvalref(interpreter, fidx, n, null_mut()) as *mut libc::c_void;
            },
            TagVariant::ClosureC => {
                let closure: *mut Closure = &mut (*((*fi).value.value_object as *mut Closure));
                if 1 <= n && n <= (*closure).count_upvalues as i32 {
                    return &mut *((*closure).upvalues).c_tvalues.as_mut_ptr().offset((n - 1) as isize) as *mut TValue as *mut libc::c_void;
                }
            },
            TagVariant::ClosureCFunction => {},
            _ => return null_mut(),
        }
        return null_mut();
    }
}
pub unsafe fn lua_upvaluejoin(interpreter: *mut Interpreter, fidx1: i32, n1: i32, fidx2: i32, n2: i32) {
    unsafe {
        let mut f1: *mut Closure = null_mut();
        let up1: *mut *mut UpValue = getupvalref(interpreter, fidx1, n1, &mut f1);
        let up2: *mut *mut UpValue = getupvalref(interpreter, fidx2, n2, null_mut());
        *up1 = *up2;
        if (*f1).get_marked() & 1 << 5 != 0 && (**up1).get_marked() & (1 << 3 | 1 << 4) != 0 {
            luac_barrier_(interpreter, &mut (*(f1 as *mut ObjectBase)), &mut (*(*up1 as *mut ObjectBase)));
        } else {
        };
    }
}
pub unsafe fn luai_makeseed(interpreter: *mut Interpreter) -> u32 {
    unsafe {
        let mut buffer: [i8; 24] = [0; 24];
        let mut h: u32 = time(null_mut()) as u32;
        let mut p: i32 = 0;
        let mut t: usize = interpreter as usize;
        memcpy(buffer.as_mut_ptr().offset(p as isize) as *mut libc::c_void, &mut t as *mut usize as *const libc::c_void, size_of::<usize>());
        p = (p as usize).wrapping_add(size_of::<usize>() as usize) as i32;
        let mut t_0: usize = &mut h as *mut u32 as usize;
        memcpy(buffer.as_mut_ptr().offset(p as isize) as *mut libc::c_void, &mut t_0 as *mut usize as *const libc::c_void, size_of::<usize>());
        p = (p as usize).wrapping_add(size_of::<usize>() as usize) as i32;
        let mut t_1: usize = ::core::mem::transmute::<Option<unsafe fn(interpreter: *mut Interpreter) -> u32>, usize>(Some(luai_makeseed as unsafe fn(interpreter: *mut Interpreter) -> u32));
        memcpy(buffer.as_mut_ptr().offset(p as isize) as *mut libc::c_void, &mut t_1 as *mut usize as *const libc::c_void, size_of::<usize>());
        p = (p as usize).wrapping_add(size_of::<usize>() as usize) as i32;
        return luas_hash(buffer.as_mut_ptr(), p as usize, h);
    }
}
pub unsafe fn luae_extendci(interpreter: *mut Interpreter) -> *mut CallInfo {
    unsafe {
        let ret = luam_malloc_(interpreter, size_of::<CallInfo>()) as *mut CallInfo;
        (*(*interpreter).callinfo).call_info_next = ret;
        (*ret).call_info_previous = (*interpreter).callinfo;
        (*ret).call_info_next = null_mut();
        write_volatile(&mut (*ret).call_info_u.l.trap as *mut i32, 0);
        (*interpreter).count_call_info = ((*interpreter).count_call_info).wrapping_add(1);
        (*interpreter).count_call_info;
        return ret;
    }
}
pub unsafe fn freeci(interpreter: *mut Interpreter) {
    unsafe {
        let mut callinfo = (*interpreter).callinfo;
        let mut next_call_info = (*callinfo).call_info_next;
        (*callinfo).call_info_next = null_mut();
        loop {
            callinfo = next_call_info;
            if callinfo.is_null() {
                break;
            }
            next_call_info = (*callinfo).call_info_next;
            (*interpreter).free_memory(callinfo as *mut libc::c_void, size_of::<CallInfo>());
            (*interpreter).count_call_info = ((*interpreter).count_call_info).wrapping_sub(1);
            (*interpreter).count_call_info;
        }
    }
}
pub unsafe fn luae_shrinkci(interpreter: *mut Interpreter) {
    unsafe {
        let mut callinfo = (*(*interpreter).callinfo).call_info_next;
        if !callinfo.is_null() {
            loop {
                let next_call_info = (*callinfo).call_info_next;
                if next_call_info.is_null() {
                    break;
                }
                let next_next_call_info = (*next_call_info).call_info_next;
                (*callinfo).call_info_next = next_next_call_info;
                (*interpreter).count_call_info = ((*interpreter).count_call_info).wrapping_sub(1);
                (*interpreter).count_call_info;
                (*interpreter).free_memory(next_call_info as *mut libc::c_void, size_of::<CallInfo>());
                if next_next_call_info.is_null() {
                    break;
                }
                (*next_next_call_info).call_info_previous = callinfo;
                callinfo = next_next_call_info;
            }
        }
    }
}
pub unsafe fn stack_init(other_state: *mut Interpreter, interpreter: *mut Interpreter) {
    unsafe {
        (*other_state).stack.stkidrel_pointer = luam_malloc_(interpreter, ((2 * 20 as i32 + 5) as usize).wrapping_mul(size_of::<TValue>())) as *mut TValue;
        (*other_state).tbc_list.stkidrel_pointer = (*other_state).stack.stkidrel_pointer;
        for i in 0..2 * 20 as i32 + 5 {
            (*((*other_state).stack.stkidrel_pointer).offset(i as isize)).set_tag_variant(TagVariant::NilNil);
        }
        (*other_state).top.stkidrel_pointer = (*other_state).stack.stkidrel_pointer;
        (*other_state).stack_last.stkidrel_pointer = ((*other_state).stack.stkidrel_pointer).offset((2 * 20 as i32) as isize);
        let callinfo = &mut (*other_state).base_callinfo;
        (*callinfo).call_info_previous = null_mut();
        (*callinfo).call_info_next = (*callinfo).call_info_previous;
        (*callinfo).call_info_call_status = (1 << 1) as u16;
        (*callinfo).call_info_function.stkidrel_pointer = (*other_state).top.stkidrel_pointer;
        (*callinfo).call_info_u.c.context_function = None;
        (*callinfo).call_info_count_results = 0;
        (*(*other_state).top.stkidrel_pointer).set_tag_variant(TagVariant::NilNil);
        (*other_state).top.stkidrel_pointer = ((*other_state).top.stkidrel_pointer).offset(1);
        (*other_state).top.stkidrel_pointer;
        (*callinfo).call_info_top.stkidrel_pointer = ((*other_state).top.stkidrel_pointer).offset(20 as isize);
        (*other_state).callinfo = callinfo;
    }
}
pub unsafe fn freestack(interpreter: *mut Interpreter) {
    unsafe {
        if ((*interpreter).stack.stkidrel_pointer).is_null() {
            return;
        }
        (*interpreter).callinfo = &mut (*interpreter).base_callinfo;
        freeci(interpreter);
        (*interpreter).free_memory(
            (*interpreter).stack.stkidrel_pointer as *mut libc::c_void,
            ((((*interpreter).stack_last.stkidrel_pointer).offset_from((*interpreter).stack.stkidrel_pointer) as i32 + 5) as usize).wrapping_mul(size_of::<TValue>() as usize) as usize,
        );
    }
}
pub unsafe fn init_registry(interpreter: *mut Interpreter, global: *mut Global) {
    unsafe {
        let registry: *mut Table = luah_new(interpreter);
        let io: *mut TValue = &mut (*global).global_lregistry;
        let x_: *mut Table = registry;
        (*io).value.value_object = &mut (*(x_ as *mut ObjectBase));
        (*io).set_tag_variant(TagVariant::Table);
        (*io).set_collectable(true);
        luah_resize(interpreter, registry, 2, 0);
        let io_0: *mut TValue = &mut *((*registry).array).offset((1 - 1) as isize) as *mut TValue;
        let x0: *mut Interpreter = interpreter;
        (*io_0).value.value_object = &mut (*(x0 as *mut ObjectBase));
        (*io_0).set_tag_variant(TagVariant::Interpreter);
        (*io_0).set_collectable(true);
        let io_1: *mut TValue = &mut *((*registry).array).offset((2 - 1) as isize) as *mut TValue;
        let x1: *mut Table = luah_new(interpreter);
        (*io_1).value.value_object = &mut (*(x1 as *mut ObjectBase));
        (*io_1).set_tag_variant(TagVariant::Table);
        (*io_1).set_collectable(true);
    }
}
pub unsafe fn f_luaopen(interpreter: *mut Interpreter, mut _ud: *mut libc::c_void) {
    unsafe {
        let global: *mut Global = (*interpreter).global;
        stack_init(interpreter, interpreter);
        init_registry(interpreter, global);
        (*interpreter).luas_init_state();
        luat_init(interpreter);
        luax_init(interpreter);
        (*global).global_gcstep = 0;
        (*global).global_nonevalue.set_tag_variant(TagVariant::NilNil);
    }
}
pub unsafe fn close_state(interpreter: *mut Interpreter) {
    unsafe {
        let global: *mut Global = (*interpreter).global;
        if (*global).global_nonevalue.is_tagtype_nil() {
            (*interpreter).callinfo = &mut (*interpreter).base_callinfo;
            (*interpreter).error_function = 0;
            do_close_protected(interpreter, 1 as i64, Status::OK);
            (*interpreter).top.stkidrel_pointer = ((*interpreter).stack.stkidrel_pointer).offset(1 as isize);
            (*global).luac_freeallobjects(interpreter);
        } else {
            (*global).luac_freeallobjects(interpreter);
        }
        (*interpreter).free_memory(
            (*(*interpreter).global).global_stringtable.stringtable_hash as *mut libc::c_void,
            (*(*interpreter).global).global_stringtable.stringtable_size * size_of::<*mut TString>(),
        );
        freestack(interpreter);
        raw_allocate(interpreter as *mut u8 as *mut libc::c_void, size_of::<Interpreter>(), 0);
        raw_allocate(global as *mut u8 as *mut libc::c_void, size_of::<Global>(), 0);
    }
}
pub unsafe fn lua_newthread(interpreter: *mut Interpreter) -> *mut Interpreter {
    unsafe {
        let global: *mut Global = (*interpreter).global;
        if (*(*interpreter).global).global_gcdebt > 0 {
            (*interpreter).luac_step();
        }
        let ret = luac_newobj(interpreter, TagVariant::Interpreter, size_of::<Interpreter>()) as *mut Interpreter;
        let io: *mut TValue = &mut (*(*interpreter).top.stkidrel_pointer);
        (*io).set_tag_variant(TagVariant::Interpreter);
        (*io).value.value_object = &mut (*(ret as *mut ObjectBase));
        (*io).set_collectable(true);
        (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
        (*ret).preinit_thread(global);
        write_volatile(&mut (*ret).hook_mask as *mut i32, (*interpreter).hook_mask);
        (*ret).base_hook_count = (*interpreter).base_hook_count;
        write_volatile(&mut (*ret).hook as *mut HookFunction, (*interpreter).hook);
        (*ret).hook_count = (*ret).base_hook_count;
        stack_init(ret, interpreter);
        return ret;
    }
}
pub unsafe fn luae_resetthread(interpreter: *mut Interpreter, mut status: Status) -> Status {
    unsafe {
        (*interpreter).callinfo = &mut (*interpreter).base_callinfo;
        let callinfo = (*interpreter).callinfo;
        (*(*interpreter).stack.stkidrel_pointer).set_tag_variant(TagVariant::NilNil);
        (*callinfo).call_info_function.stkidrel_pointer = (*interpreter).stack.stkidrel_pointer;
        (*callinfo).call_info_call_status = (1 << 1) as u16;
        if status == Status::Yield {
            status = Status::OK;
        }
        (*interpreter).status = Status::OK;
        (*interpreter).error_function = 0;
        status = do_close_protected(interpreter, 1 as i64, status);
        if status != Status::OK {
            (*interpreter).set_error_object(status, ((*interpreter).stack.stkidrel_pointer).offset(1 as isize));
        } else {
            (*interpreter).top.stkidrel_pointer = ((*interpreter).stack.stkidrel_pointer).offset(1 as isize);
        }
        (*callinfo).call_info_top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(20 as isize);
        luad_reallocstack(interpreter, ((*callinfo).call_info_top.stkidrel_pointer).offset_from((*interpreter).stack.stkidrel_pointer) as i32, false);
        return status;
    }
}
pub unsafe fn lua_closethread(interpreter: *mut Interpreter, from: *mut Interpreter) -> Status {
    unsafe {
        (*interpreter).count_c_calls = if !from.is_null() { (*from).count_c_calls & 0xffff as u32 } else { 0 };
        let status = luae_resetthread(interpreter, (*interpreter).status);
        return status;
    }
}
pub unsafe fn lua_close(mut interpreter: *mut Interpreter) {
    unsafe {
        interpreter = (*(*interpreter).global).global_maininterpreter;
        close_state(interpreter);
    }
}
pub unsafe fn luae_warning(interpreter: *mut Interpreter, message: *const i8, tocont: i32) {
    unsafe {
        let warn_function: WarnFunction = (*(*interpreter).global).global_warnfunction;
        if warn_function.is_some() {
            warn_function.expect("non-null function pointer")((*(*interpreter).global).global_warnuserdata, message, tocont);
        }
    }
}
pub unsafe fn luae_warnerror(interpreter: *mut Interpreter, where_0: *const i8) {
    unsafe {
        let errobj: *mut TValue = &mut (*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize)));
        let message: *const i8 = if (*errobj).is_tagtype_string() {
            ((*((*errobj).value.value_object as *mut TString)).get_contents_mut()) as *const i8
        } else {
            c"error object is not a string".as_ptr()
        };
        luae_warning(interpreter, c"error in ".as_ptr(), 1);
        luae_warning(interpreter, where_0, 1);
        luae_warning(interpreter, c" (".as_ptr(), 1);
        luae_warning(interpreter, message, 1);
        luae_warning(interpreter, c")".as_ptr(), 0);
    }
}
pub unsafe fn lua_sethook(interpreter: *mut Interpreter, mut function: HookFunction, mut mask: i32, count: i32) {
    unsafe {
        if function.is_none() || mask == 0 {
            mask = 0;
            function = None;
        }
        write_volatile(&mut (*interpreter).hook as *mut HookFunction, function);
        (*interpreter).base_hook_count = count;
        (*interpreter).hook_count = (*interpreter).base_hook_count;
        write_volatile(&mut (*interpreter).hook_mask as *mut i32, mask as u8 as i32);
        if mask != 0 {
            settraps((*interpreter).callinfo);
        }
    }
}
pub unsafe fn lua_gethook(interpreter: *mut Interpreter) -> HookFunction {
    unsafe {
        return (*interpreter).hook;
    }
}
pub unsafe fn lua_gethookmask(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        return (*interpreter).hook_mask;
    }
}
pub unsafe fn lua_gethookcount(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        return (*interpreter).base_hook_count;
    }
}
pub unsafe fn lua_getstack(interpreter: *mut Interpreter, mut level: i32, debuginfo: *mut DebugInfo) -> i32 {
    unsafe {
        let status: i32;
        let mut callinfo;
        if level < 0 {
            return 0;
        }
        callinfo = (*interpreter).callinfo;
        while level > 0 && callinfo != &mut (*interpreter).base_callinfo as *mut CallInfo {
            level -= 1;
            callinfo = (*callinfo).call_info_previous;
        }
        if level == 0 && callinfo != &mut (*interpreter).base_callinfo as *mut CallInfo {
            status = 1;
            (*debuginfo).debuginfo_callinfo = callinfo;
        } else {
            status = 0;
        }
        return status;
    }
}
pub unsafe fn formatvarinfo(interpreter: *mut Interpreter, kind: *const i8, name: *const i8) -> *const i8 {
    unsafe {
        if kind.is_null() {
            return c"".as_ptr();
        } else {
            return luao_pushfstring(interpreter, c" (%s '%s')".as_ptr(), kind, name);
        };
    }
}
pub unsafe fn varinfo(interpreter: *mut Interpreter, o: *const TValue) -> *const i8 {
    unsafe {
        let callinfo = (*interpreter).callinfo;
        let mut name: *const i8 = null();
        let mut kind: *const i8 = null();
        if (*callinfo).call_info_call_status as i32 & 1 << 1 == 0 {
            kind = getupvalname(callinfo, o, &mut name);
            if kind.is_null() {
                let reg: i32 = in_stack(callinfo, o);
                if reg >= 0 {
                    kind = getobjname((*((*(*callinfo).call_info_function.stkidrel_pointer).value.value_object as *mut Closure)).payload.l_prototype, currentpc(callinfo), reg, &mut name);
                }
            }
        }
        return formatvarinfo(interpreter, kind, name);
    }
}
pub unsafe fn typeerror(interpreter: *mut Interpreter, o: *const TValue, op: *const i8, extra: *const i8) -> ! {
    unsafe {
        let t: *const i8 = luat_objtypename(interpreter, o);
        luag_runerror(interpreter, c"attempt to %s a %s value%s".as_ptr(), op, t, extra);
    }
}
pub unsafe fn luag_typeerror(interpreter: *mut Interpreter, o: *const TValue, op: *const i8) -> ! {
    unsafe {
        typeerror(interpreter, o, op, varinfo(interpreter, o));
    }
}
pub unsafe fn luag_callerror(interpreter: *mut Interpreter, o: *const TValue) -> ! {
    unsafe {
        let callinfo = (*interpreter).callinfo;
        let mut name: *const i8 = null();
        let kind: *const i8 = funcnamefromcall(interpreter, callinfo, &mut name);
        let extra: *const i8 = if !kind.is_null() { formatvarinfo(interpreter, kind, name) } else { varinfo(interpreter, o) };
        typeerror(interpreter, o, c"call".as_ptr(), extra);
    }
}
pub unsafe fn luag_forerror(interpreter: *mut Interpreter, o: *const TValue, what: *const i8) -> ! {
    unsafe {
        luag_runerror(interpreter, c"bad 'for' %s (number expected, got %s)".as_ptr(), what, luat_objtypename(interpreter, o));
    }
}
pub unsafe fn luag_concaterror(interpreter: *mut Interpreter, mut p1: *const TValue, p2: *const TValue) -> ! {
    unsafe {
        match (*p1).get_tag_variant().to_tag_type() {
            TagType::String | TagType::Numeric => {
                p1 = p2;
            },
            _ => {},
        }
        luag_typeerror(interpreter, p1, c"concatenate".as_ptr());
    }
}
pub unsafe fn luag_opinterror(interpreter: *mut Interpreter, p1: *const TValue, mut p2: *const TValue, message: *const i8) -> ! {
    unsafe {
        if !(*p1).is_tagtype_numeric() {
            p2 = p1;
        }
        luag_typeerror(interpreter, p2, message);
    }
}
pub unsafe fn luag_tointerror(interpreter: *mut Interpreter, p1: *const TValue, mut p2: *const TValue) -> ! {
    unsafe {
        let mut temp: i64 = 0;
        if luav_tointegerns(p1, &mut temp, F2I::Equal) == 0 {
            p2 = p1;
        }
        luag_runerror(interpreter, c"number%s has no integer representation".as_ptr(), varinfo(interpreter, p2));
    }
}
pub unsafe fn luag_ordererror(interpreter: *mut Interpreter, p1: *const TValue, p2: *const TValue) -> ! {
    unsafe {
        let t1: *const i8 = luat_objtypename(interpreter, p1);
        let t2: *const i8 = luat_objtypename(interpreter, p2);
        if strcmp(t1, t2) == 0 {
            luag_runerror(interpreter, c"attempt to compare two %s values".as_ptr(), t1);
        } else {
            luag_runerror(interpreter, c"attempt to compare %s with %s".as_ptr(), t1, t2);
        };
    }
}
pub unsafe fn luag_addinfo(interpreter: *mut Interpreter, message: *const i8, src: *mut TString, line: i32) -> *const i8 {
    unsafe {
        let mut buffer: [i8; 60] = [0; 60];
        if !src.is_null() {
            luao_chunkid(buffer.as_mut_ptr(), (*src).get_contents_mut(), (*src).get_length() as usize);
        } else {
            buffer[0] = Character::Question as i8;
            buffer[1] = Character::Null as i8;
        }
        return luao_pushfstring(interpreter, c"%s:%d: %s".as_ptr(), buffer.as_mut_ptr(), line, message);
    }
}
pub unsafe fn luag_errormsg(interpreter: *mut Interpreter) -> ! {
    unsafe {
        if (*interpreter).error_function != 0 {
            let error_function: *mut TValue = ((*interpreter).stack.stkidrel_pointer as *mut i8).offset((*interpreter).error_function as isize) as *mut TValue;
            let io1: *mut TValue = &mut (*(*interpreter).top.stkidrel_pointer);
            let io2: *const TValue = &mut (*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize)));
            (*io1).copy_from(&*io2);
            let io1_0: *mut TValue = &mut (*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize)));
            let io2_0: *const TValue = &mut (*error_function);
            (*io1_0).copy_from(&*io2_0);
            (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
            luad_callnoyield(interpreter, (*interpreter).top.stkidrel_pointer.offset(-(2 as isize)), 1);
        }
        luad_throw(interpreter, Status::RuntimeError);
    }
}
pub unsafe extern "C" fn luag_runerror(interpreter: *mut Interpreter, fmt: *const i8, args: ...) -> ! {
    unsafe {
        let callinfo = (*interpreter).callinfo;
        let message: *const i8;
        let mut argp: ::core::ffi::VaListImpl;
        if (*(*interpreter).global).global_gcdebt > 0 {
            (*interpreter).luac_step();
        }
        argp = args.clone();
        message = luao_pushvfstring(interpreter, fmt, argp.as_va_list());
        if (*callinfo).call_info_call_status as i32 & 1 << 1 == 0 {
            luag_addinfo(
                interpreter,
                message,
                (*(*((*(*callinfo).call_info_function.stkidrel_pointer).value.value_object as *mut Closure)).payload.l_prototype).prototype_source,
                getcurrentline(callinfo),
            );
            let io1: *mut TValue = &mut (*(*interpreter).top.stkidrel_pointer.offset(-(2 as isize)));
            let io2: *const TValue = &mut (*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize)));
            (*io1).copy_from(&*io2);
            (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(-1);
        }
        luag_errormsg(interpreter);
    }
}
pub unsafe fn luag_tracecall(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let callinfo = (*interpreter).callinfo;
        let p: *mut Prototype = (*((*(*callinfo).call_info_function.stkidrel_pointer).value.value_object as *mut Closure)).payload.l_prototype;
        write_volatile(&mut (*callinfo).call_info_u.l.trap as *mut i32, 1);
        if (*callinfo).call_info_u.l.saved_program_counter == (*p).prototype_code.vectort_pointer as *const u32 {
            if (*p).prototype_is_variable_arguments {
                return 0;
            } else if (*callinfo).call_info_call_status as i32 & 1 << 6 == 0 {
                luad_hookcall(interpreter, callinfo);
            }
        }
        return 1;
    }
}
pub unsafe fn luag_traceexec(interpreter: *mut Interpreter, mut program_counter: *const u32) -> i32 {
    unsafe {
        let callinfo = (*interpreter).callinfo;
        let mask: u8 = (*interpreter).hook_mask as u8;
        let p: *const Prototype = (*((*(*callinfo).call_info_function.stkidrel_pointer).value.value_object as *mut Closure)).payload.l_prototype;
        if mask as i32 & (1 << 2 | 1 << 3) == 0 {
            write_volatile(&mut (*callinfo).call_info_u.l.trap as *mut i32, 0);
            return 0;
        }
        program_counter = program_counter.offset(1);
        (*callinfo).call_info_u.l.saved_program_counter = program_counter;
        let counthook: i32 = (mask as i32 & 1 << 3 != 0 && {
            (*interpreter).hook_count -= 1;
            (*interpreter).hook_count == 0
        }) as i32;
        if counthook != 0 {
            (*interpreter).hook_count = (*interpreter).base_hook_count;
        } else if mask as i32 & 1 << 2 == 0 {
            return 1;
        }
        if (*callinfo).call_info_call_status as i32 & 1 << 6 != 0 {
            (*callinfo).call_info_call_status = ((*callinfo).call_info_call_status as i32 & !(1 << 6)) as u16;
            return 1;
        }
        if !(OPMODES[(*((*callinfo).call_info_u.l.saved_program_counter).offset(-(1 as isize)) >> 0 & !(!(0u32) << 7) << 0) as usize] as i32 & 1 << 5 != 0
            && (*((*callinfo).call_info_u.l.saved_program_counter).offset(-(1 as isize)) >> POSITION_B & !(!(0u32) << 8) << 0) as i32 == 0)
        {
            (*interpreter).top.stkidrel_pointer = (*callinfo).call_info_top.stkidrel_pointer;
        }
        if counthook != 0 {
            luad_hook(interpreter, 3, -1, 0, 0);
        }
        if mask as i32 & 1 << 2 != 0 {
            let old_program_counter: i32 = if (*interpreter).old_program_counter < (*p).prototype_code.get_size() as i32 { (*interpreter).old_program_counter } else { 0 };
            let npci: i32 = program_counter.offset_from((*p).prototype_code.vectort_pointer) as i32 - 1;
            if npci <= old_program_counter || changedline(p, old_program_counter, npci) != 0 {
                let newline: i32 = luag_getfuncline(p, npci);
                luad_hook(interpreter, 2, newline, 0, 0);
            }
            (*interpreter).old_program_counter = npci;
        }
        if (*interpreter).status == Status::Yield {
            if counthook != 0 {
                (*interpreter).hook_count = 1;
            }
            (*callinfo).call_info_call_status = ((*callinfo).call_info_call_status as i32 | 1 << 6) as u16;
            luad_throw(interpreter, Status::Yield);
        }
        return 1;
    }
}
pub unsafe fn tryagain(interpreter: *mut Interpreter, block: *mut libc::c_void, old_size: usize, new_size: usize) -> *mut libc::c_void {
    unsafe {
        let global: *mut Global = (*interpreter).global;
        if (*global).global_nonevalue.is_tagtype_nil() && (*global).global_gcstopem == 0 {
            (*global).luac_fullgc(interpreter, true);
            return raw_allocate(block, old_size, new_size);
        } else {
            return null_mut();
        };
    }
}
pub unsafe fn luam_realloc_(interpreter: *mut Interpreter, block: *mut libc::c_void, old_size: usize, new_size: usize) -> *mut libc::c_void {
    unsafe {
        let global: *mut Global = (*interpreter).global;
        let mut new_block: *mut libc::c_void = raw_allocate(block, old_size, new_size);
        if new_block.is_null() && new_size > 0 {
            new_block = tryagain(interpreter, block, old_size, new_size);
            if new_block.is_null() {
                return null_mut();
            }
        }
        (*global).global_gcdebt = ((*global).global_gcdebt as usize).wrapping_add(new_size).wrapping_sub(old_size) as i64;
        return new_block;
    }
}
pub unsafe fn luam_saferealloc_(interpreter: *mut Interpreter, block: *mut libc::c_void, old_size: usize, new_size: usize) -> *mut libc::c_void {
    unsafe {
        let new_block: *mut libc::c_void = luam_realloc_(interpreter, block, old_size, new_size);
        if new_block.is_null() && new_size > 0 {
            luad_throw(interpreter, Status::MemoryError);
        }
        return new_block;
    }
}
pub unsafe fn luam_malloc_(interpreter: *mut Interpreter, new_size: usize) -> *mut libc::c_void {
    unsafe {
        if new_size == 0 {
            return null_mut();
        } else {
            let global: *mut Global = (*interpreter).global;
            let mut new_block: *mut libc::c_void = raw_allocate(null_mut(), 0, new_size);
            if new_block.is_null() {
                new_block = tryagain(interpreter, null_mut(), 0, new_size);
                if new_block.is_null() {
                    luad_throw(interpreter, Status::MemoryError);
                }
            }
            (*global).global_gcdebt = ((*global).global_gcdebt as usize).wrapping_add(new_size) as i64;
            return new_block;
        };
    }
}
pub unsafe fn intarith(interpreter: *mut Interpreter, op: i32, v1: i64, v2: i64) -> i64 {
    unsafe {
        match op {
            0 => return (v1 as usize).wrapping_add(v2 as usize) as i64,
            1 => return (v1 as usize).wrapping_sub(v2 as usize) as i64,
            2 => return (v1 as usize).wrapping_mul(v2 as usize) as i64,
            3 => return luav_mod(interpreter, v1, v2),
            6 => return luav_idiv(interpreter, v1, v2),
            7 => return (v1 as usize & v2 as usize) as i64,
            8 => return (v1 as usize | v2 as usize) as i64,
            9 => return (v1 as usize ^ v2 as usize) as i64,
            10 => return luav_shiftl(v1, v2),
            11 => {
                return luav_shiftl(v1, (0usize).wrapping_sub(v2 as usize) as i64);
            },
            12 => {
                return (0usize).wrapping_sub(v1 as usize) as i64;
            },
            13 => {
                return (!(0usize) ^ v1 as usize) as i64;
            },
            _ => return 0,
        };
    }
}
pub unsafe fn numarith(interpreter: *mut Interpreter, op: i32, v1: f64, v2: f64) -> f64 {
    unsafe {
        match op {
            0 => return v1 + v2,
            1 => return v1 - v2,
            2 => return v1 * v2,
            5 => return v1 / v2,
            4 => {
                return if v2 == 2.0 { v1 * v1 } else { v1.powf(v2) };
            },
            6 => return (v1 / v2).floor(),
            12 => return -v1,
            3 => return luav_modf(interpreter, v1, v2),
            _ => return 0.0,
        };
    }
}
pub unsafe fn luao_rawarith(interpreter: *mut Interpreter, op: i32, p1: *const TValue, p2: *const TValue, res: *mut TValue) -> i32 {
    unsafe {
        match op {
            7 | 8 | 9 | 10 | 11 | 13 => {
                let mut i1: i64 = 0;
                let mut i2: i64 = 0;
                if (if (((*p1).get_tag_variant() == TagVariant::NumericInteger) as i32 != 0) as i64 != 0 {
                    i1 = (*p1).value.value_integer;
                    1
                } else {
                    luav_tointegerns(p1, &mut i1, F2I::Equal)
                }) != 0
                    && (if (((*p2).get_tag_variant() == TagVariant::NumericInteger) as i32 != 0) as i32 as i64 != 0 {
                        i2 = (*p2).value.value_integer;
                        1
                    } else {
                        luav_tointegerns(p2, &mut i2, F2I::Equal)
                    }) != 0
                {
                    (*res).value.value_integer = intarith(interpreter, op, i1, i2);
                    (*res).set_tag_variant(TagVariant::NumericInteger);
                    return 1;
                } else {
                    return 0;
                }
            },
            5 | 4 => {
                let mut n1: f64 = 0.0;
                let mut n2: f64 = 0.0;
                if (if (*p1).get_tag_variant() == TagVariant::NumericNumber {
                    n1 = (*p1).value.value_number;
                    1
                } else {
                    if (*p1).get_tag_variant() == TagVariant::NumericInteger {
                        n1 = (*p1).value.value_integer as f64;
                        1
                    } else {
                        0
                    }
                }) != 0
                    && (if (*p2).get_tag_variant() == TagVariant::NumericNumber {
                        n2 = (*p2).value.value_number;
                        1
                    } else {
                        if (*p2).get_tag_variant() == TagVariant::NumericInteger {
                            n2 = (*p2).value.value_integer as f64;
                            1
                        } else {
                            0
                        }
                    }) != 0
                {
                    (*res).value.value_number = numarith(interpreter, op, n1, n2);
                    (*res).set_tag_variant(TagVariant::NumericNumber);
                    return 1;
                } else {
                    return 0;
                }
            },
            _ => {
                let mut n1_0: f64 = 0.0;
                let mut n2_0: f64 = 0.0;
                if (*p1).get_tag_variant() == TagVariant::NumericInteger && (*p2).get_tag_variant() == TagVariant::NumericInteger {
                    let io_1: *mut TValue = res;
                    (*io_1).value.value_integer = intarith(interpreter, op, (*p1).value.value_integer, (*p2).value.value_integer);
                    (*io_1).set_tag_variant(TagVariant::NumericInteger);
                    return 1;
                } else if (if (*p1).get_tag_variant() == TagVariant::NumericNumber {
                    n1_0 = (*p1).value.value_number;
                    1
                } else {
                    if (*p1).get_tag_variant() == TagVariant::NumericInteger {
                        n1_0 = (*p1).value.value_integer as f64;
                        1
                    } else {
                        0
                    }
                }) != 0
                    && (if (*p2).get_tag_variant() == TagVariant::NumericNumber {
                        n2_0 = (*p2).value.value_number;
                        1
                    } else {
                        if (*p2).get_tag_variant() == TagVariant::NumericInteger {
                            n2_0 = (*p2).value.value_integer as f64;
                            1
                        } else {
                            0
                        }
                    }) != 0
                {
                    let io_2: *mut TValue = res;
                    (*io_2).value.value_number = numarith(interpreter, op, n1_0, n2_0);
                    (*io_2).set_tag_variant(TagVariant::NumericNumber);
                    return 1;
                } else {
                    return 0;
                }
            },
        };
    }
}
pub unsafe fn luao_pushvfstring(interpreter: *mut Interpreter, mut fmt: *const i8, mut argp: ::core::ffi::VaList) -> *const i8 {
    unsafe {
        let mut buff_fs = BuffFS::new(interpreter);
        let mut e: *const i8;
        loop {
            e = strchr(fmt, Character::Percent as i32);
            if e.is_null() {
                break;
            }
            buff_fs.add_string(fmt, e.offset_from(fmt) as usize);
            match Character::from(*e.offset(1 as isize) as i32) {
                Character::LowerS => {
                    let mut s: *const i8 = argp.arg::<*mut i8>();
                    if s.is_null() {
                        s = c"(null)".as_ptr();
                    }
                    buff_fs.add_string(s, strlen(s) as usize);
                },
                Character::LowerC => {
                    let mut c: i8 = argp.arg::<i32>() as u8 as i8;
                    buff_fs.add_string(&mut c, 1 as usize);
                },
                Character::LowerD => {
                    let mut tvalue: TValue = TValue::new(TagVariant::NilNil);
                    tvalue.value.value_integer = argp.arg::<i32>() as i64;
                    tvalue.set_tag_variant(TagVariant::NumericInteger);
                    buff_fs.add_number(&mut tvalue);
                },
                Character::UpperI => {
                    let mut tvalue: TValue = TValue::new(TagVariant::NilNil);
                    tvalue.value.value_integer = argp.arg::<i64>();
                    tvalue.set_tag_variant(TagVariant::NumericInteger);
                    buff_fs.add_number(&mut tvalue);
                },
                Character::LowerF => {
                    let mut tvalue: TValue = TValue::new(TagVariant::NilNil);
                    tvalue.value.value_number = argp.arg::<f64>();
                    tvalue.set_tag_variant(TagVariant::NumericNumber);
                    buff_fs.add_number(&mut tvalue);
                },
                Character::LowerP => {
                    let size = (3 as usize).wrapping_mul(size_of::<*mut libc::c_void>()).wrapping_add(8);
                    let bf: *mut i8 = buff_fs.get_raw(size);
                    let p: *mut libc::c_void = argp.arg::<*mut libc::c_void>();
                    let length = snprintf(bf, size, c"%p".as_ptr(), p);
                    buff_fs.add_length(length as usize);
                },
                Character::UpperU => {
                    let mut bf_0: [i8; 8] = [0; 8];
                    let length_0: i32 = luao_utf8esc(bf_0.as_mut_ptr(), argp.arg::<i64>() as usize);
                    buff_fs.add_string(bf_0.as_mut_ptr().offset(8 as isize).offset(-(length_0 as isize)), length_0 as usize);
                },
                Character::Percent => {
                    buff_fs.add_string(c"%".as_ptr(), 1 as usize);
                },
                _ => {
                    luag_runerror(interpreter, c"invalid option '%%%c' to 'lua_pushfstring'".as_ptr(), *e.offset(1 as isize) as i32);
                },
            }
            fmt = e.offset(2 as isize);
        }
        buff_fs.add_string(fmt, strlen(fmt) as usize);
        buff_fs.clear();
        return (*((*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))).value.value_object as *mut TString)).get_contents_mut();
    }
}
pub unsafe extern "C" fn luao_pushfstring(interpreter: *mut Interpreter, fmt: *const i8, args: ...) -> *const i8 {
    unsafe {
        let message: *const i8;
        let mut argp: ::core::ffi::VaListImpl;
        argp = args.clone();
        message = luao_pushvfstring(interpreter, fmt, argp.as_va_list());
        return message;
    }
}
pub unsafe fn luat_init(interpreter: *mut Interpreter) {
    unsafe {
        static mut EVENT_NAMES: [*const i8; 25] = [
            c"__index".as_ptr(),
            c"__newindex".as_ptr(),
            c"__gc".as_ptr(),
            c"__mode".as_ptr(),
            c"__len".as_ptr(),
            c"__eq".as_ptr(),
            c"__add".as_ptr(),
            c"__sub".as_ptr(),
            c"__mul".as_ptr(),
            c"__mod".as_ptr(),
            c"__pow".as_ptr(),
            c"__div".as_ptr(),
            c"__idiv".as_ptr(),
            c"__band".as_ptr(),
            c"__bor".as_ptr(),
            c"__bxor".as_ptr(),
            c"__shl".as_ptr(),
            c"__shr".as_ptr(),
            c"__unm".as_ptr(),
            c"__bnot".as_ptr(),
            c"__lt".as_ptr(),
            c"__le".as_ptr(),
            c"__concat".as_ptr(),
            c"__call".as_ptr(),
            c"__close".as_ptr(),
        ];
        for i in 0..TM_N {
            (*(*interpreter).global).global_tmname[i as usize] = luas_new(interpreter, EVENT_NAMES[i as usize]);
            fix_object_state(interpreter, &mut (*(*((*(*interpreter).global).global_tmname).as_mut_ptr().offset(i as isize) as *mut ObjectBase)));
        }
    }
}
pub unsafe fn luat_gettmbyobj(interpreter: *mut Interpreter, o: *const TValue, event: u32) -> *const TValue {
    unsafe {
        let metatable: *mut Table;
        match (*o).get_tag_variant().to_tag_type() {
            TagType::Table => {
                metatable = (*((*o).value.value_object as *mut Table)).get_metatable();
            },
            TagType::User => {
                metatable = (*((*o).value.value_object as *mut User)).get_metatable();
            },
            _ => {
                metatable = (*(*interpreter).global).global_metatables[(*o).get_tag_variant().to_tag_type() as usize];
            },
        }
        return if metatable.is_null() {
            &mut (*(*interpreter).global).global_nonevalue as *mut TValue as *const TValue
        } else {
            luah_getshortstr(metatable, (*(*interpreter).global).global_tmname[event as usize])
        };
    }
}
pub unsafe fn luat_objtypename(interpreter: *mut Interpreter, o: *const TValue) -> *const i8 {
    unsafe {
        let mut metatable: *mut Table;
        if (*o).get_tag_variant() == TagVariant::Table && {
            metatable = (*((*o).value.value_object as *mut Table)).get_metatable();
            !metatable.is_null()
        } || (*o).get_tag_variant() == TagVariant::User && {
            metatable = (*((*o).value.value_object as *mut User)).get_metatable();
            !metatable.is_null()
        } {
            let name: *const TValue = luah_getshortstr(metatable, luas_new(interpreter, c"__name".as_ptr()));
            if (*name).is_tagtype_string() {
                return (*((*name).value.value_object as *mut TString)).get_contents_mut();
            }
        }
        return TYPE_NAMES[((*o).get_tag_variant().to_tag_type() as usize + 1) as usize];
    }
}
pub unsafe fn luat_calltm(interpreter: *mut Interpreter, f: *const TValue, p1: *const TValue, p2: *const TValue, p3: *const TValue) {
    unsafe {
        let function: *mut TValue = (*interpreter).top.stkidrel_pointer;
        let io1: *mut TValue = &mut (*function);
        (*io1).copy_from(&*f);
        let io1_0: *mut TValue = &mut (*function.offset(1 as isize));
        let io2_0: *const TValue = p1;
        (*io1_0).copy_from(&*io2_0);
        let io1_1: *mut TValue = &mut (*function.offset(2 as isize));
        let io2_1: *const TValue = p2;
        (*io1_1).copy_from(&*io2_1);
        let io1_2: *mut TValue = &mut (*function.offset(3 as isize));
        let io2_2: *const TValue = p3;
        (*io1_2).copy_from(&*io2_2);
        (*interpreter).top.stkidrel_pointer = function.offset(4 as isize);
        if (*(*interpreter).callinfo).call_info_call_status as i32 & (1 << 1 | 1 << 3) == 0 {
            ccall(interpreter, function, 0, 1);
        } else {
            luad_callnoyield(interpreter, function, 0);
        };
    }
}
pub unsafe fn luat_calltmres(interpreter: *mut Interpreter, f: *const TValue, p1: *const TValue, p2: *const TValue, mut res: *mut TValue) {
    unsafe {
        let result: i64 = (res as *mut i8).offset_from((*interpreter).stack.stkidrel_pointer as *mut i8) as i64;
        let function: *mut TValue = (*interpreter).top.stkidrel_pointer;
        let io1: *mut TValue = &mut (*function);
        let io2: *const TValue = f;
        (*io1).copy_from(&*io2);
        let io1_0: *mut TValue = &mut (*function.offset(1 as isize));
        let io2_0: *const TValue = p1;
        (*io1_0).copy_from(&(*io2_0));
        let io1_1: *mut TValue = &mut (*function.offset(2 as isize));
        let io2_1: *const TValue = p2;
        (*io1_1).copy_from(&(*io2_1));
        (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(3 as isize);
        if (*(*interpreter).callinfo).call_info_call_status as i32 & (1 << 1 | 1 << 3) == 0 {
            ccall(interpreter, function, 1, 1);
        } else {
            luad_callnoyield(interpreter, function, 1);
        }
        res = ((*interpreter).stack.stkidrel_pointer as *mut i8).offset(result as isize) as *mut TValue;
        let io1_2: *mut TValue = &mut (*res);
        (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(-1);
        let io2_2: *const TValue = &mut (*(*interpreter).top.stkidrel_pointer);
        (*io1_2).copy_from(&(*io2_2));
    }
}
pub unsafe fn callbintm(interpreter: *mut Interpreter, p1: *const TValue, p2: *const TValue, res: *mut TValue, event: u32) -> i32 {
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
pub unsafe fn luat_trybintm(interpreter: *mut Interpreter, p1: *const TValue, p2: *const TValue, res: *mut TValue, event: u32) {
    unsafe {
        if callbintm(interpreter, p1, p2, res, event) == 0 {
            match event as u32 {
                TM_BAND | TM_BOR | TM_BXOR | TM_SHL | TM_SHR | TM_BNOT => {
                    if (*p1).is_tagtype_numeric() && (*p2).is_tagtype_numeric() {
                        luag_tointerror(interpreter, p1, p2);
                    } else {
                        luag_opinterror(interpreter, p1, p2, c"perform bitwise operation on".as_ptr());
                    }
                },
                _ => {
                    luag_opinterror(interpreter, p1, p2, c"perform arithmetic on".as_ptr());
                },
            }
        }
    }
}
pub unsafe fn luat_tryconcattm(interpreter: *mut Interpreter) {
    unsafe {
        let top: *mut TValue = (*interpreter).top.stkidrel_pointer;
        if ((callbintm(interpreter, &mut (*top.offset(-(2 as isize))), &mut (*top.offset(-(1 as isize))), top.offset(-(2 as isize)), TM_CONCAT) == 0) as i32 != 0) as i64 != 0 {
            luag_concaterror(interpreter, &mut (*top.offset(-(2 as isize))), &mut (*top.offset(-(1 as isize))));
        }
    }
}
pub unsafe fn luat_trybinassoctm(interpreter: *mut Interpreter, p1: *const TValue, p2: *const TValue, flip: i32, res: *mut TValue, event: u32) {
    unsafe {
        if flip != 0 {
            luat_trybintm(interpreter, p2, p1, res, event);
        } else {
            luat_trybintm(interpreter, p1, p2, res, event);
        };
    }
}
pub unsafe fn luat_trybinitm(interpreter: *mut Interpreter, p1: *const TValue, i2: i64, flip: i32, res: *mut TValue, event: u32) {
    unsafe {
        let mut aux: TValue = TValue::new(TagVariant::NilNil);
        let io: *mut TValue = &mut aux;
        (*io).value.value_integer = i2;
        (*io).set_tag_variant(TagVariant::NumericInteger);
        luat_trybinassoctm(interpreter, p1, &mut aux, flip, res, event);
    }
}
pub unsafe fn luat_callordertm(interpreter: *mut Interpreter, p1: *const TValue, p2: *const TValue, event: u32) -> i32 {
    unsafe {
        if callbintm(interpreter, p1, p2, (*interpreter).top.stkidrel_pointer, event) != 0 {
            return !((*(*interpreter).top.stkidrel_pointer).get_tag_variant() == TagVariant::BooleanFalse || (*(*interpreter).top.stkidrel_pointer).is_tagtype_nil()) as i32;
        }
        luag_ordererror(interpreter, p1, p2);
    }
}
pub unsafe fn luat_callorderitm(interpreter: *mut Interpreter, mut p1: *const TValue, v2: i32, flip: i32, is_float: bool, event: u32) -> i32 {
    unsafe {
        let mut aux: TValue = TValue::new(TagVariant::NilNil);
        let p2: *const TValue;
        if is_float {
            let io: *mut TValue = &mut aux;
            (*io).value.value_number = v2 as f64;
            (*io).set_tag_variant(TagVariant::NumericNumber);
        } else {
            let io_0: *mut TValue = &mut aux;
            (*io_0).value.value_integer = v2 as i64;
            (*io_0).set_tag_variant(TagVariant::NumericInteger);
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
pub unsafe fn luat_adjustvarargs(interpreter: *mut Interpreter, nfixparams: i32, callinfo: *mut CallInfo, p: *const Prototype) {
    unsafe {
        let actual: i32 = ((*interpreter).top.stkidrel_pointer).offset_from((*callinfo).call_info_function.stkidrel_pointer) as i32 - 1;
        let nextra: i32 = actual - nfixparams;
        (*callinfo).call_info_u.l.count_extra_arguments = nextra;
        if ((((*interpreter).stack_last.stkidrel_pointer).offset_from((*interpreter).top.stkidrel_pointer) as i64 <= ((*p).prototype_maximum_stack_size as i32 + 1) as i64) as i32 != 0) as i64 != 0 {
            luad_growstack(interpreter, (*p).prototype_maximum_stack_size as i32 + 1, true);
        }
        let fresh12 = (*interpreter).top.stkidrel_pointer;
        (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
        let io1: *mut TValue = &mut (*fresh12);
        let io2: *const TValue = &mut (*(*callinfo).call_info_function.stkidrel_pointer);
        (*io1).copy_from(&*io2);
        for i in 1..(1 + nfixparams) {
            let fresh13 = (*interpreter).top.stkidrel_pointer;
            (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
            let io1_0: *mut TValue = &mut (*fresh13);
            let io2_0: *const TValue = &mut (*((*callinfo).call_info_function.stkidrel_pointer).offset(i as isize));
            (*io1_0).copy_from(&*io2_0);
            (*((*callinfo).call_info_function.stkidrel_pointer).offset(i as isize)).set_tag_variant(TagVariant::NilNil);
        }
        (*callinfo).call_info_function.stkidrel_pointer = ((*callinfo).call_info_function.stkidrel_pointer).offset((actual + 1) as isize);
        (*callinfo).call_info_top.stkidrel_pointer = ((*callinfo).call_info_top.stkidrel_pointer).offset((actual + 1) as isize);
    }
}
pub unsafe fn luat_getvarargs(interpreter: *mut Interpreter, callinfo: *mut CallInfo, mut where_0: *mut TValue, mut wanted: i32) {
    unsafe {
        let nextra: i32 = (*callinfo).call_info_u.l.count_extra_arguments;
        if wanted < 0 {
            wanted = nextra;
            if ((((*interpreter).stack_last.stkidrel_pointer).offset_from((*interpreter).top.stkidrel_pointer) as i64 <= nextra as i64) as i32 != 0) as i64 != 0 {
                let t__: i64 = (where_0 as *mut i8).offset_from((*interpreter).stack.stkidrel_pointer as *mut i8) as i64;
                if (*(*interpreter).global).global_gcdebt > 0 {
                    (*interpreter).luac_step();
                }
                luad_growstack(interpreter, nextra, true);
                where_0 = ((*interpreter).stack.stkidrel_pointer as *mut i8).offset(t__ as isize) as *mut TValue;
            }
            (*interpreter).top.stkidrel_pointer = where_0.offset(nextra as isize);
        }
        for i in 0..wanted.min(nextra) {
            let io1: *mut TValue = &mut (*where_0.offset(i as isize));
            let io2: *const TValue = &mut (*((*callinfo).call_info_function.stkidrel_pointer).offset(-(nextra as isize)).offset(i as isize));
            (*io1).copy_from(&*io2);
        }
        for i in wanted.min(nextra)..wanted {
            (*where_0.offset(i as isize)).set_tag_variant(TagVariant::NilNil);
        }
    }
}
pub unsafe fn luac_newobj(interpreter: *mut Interpreter, tagvariant: TagVariant, size: usize) -> *mut ObjectBase {
    unsafe {
        let global: *mut Global = (*interpreter).global;
        let ret = luam_malloc_(interpreter, size as usize) as *mut ObjectBase;
        (*ret).set_tag_variant(tagvariant);
        (*ret).set_marked((*global).global_currentwhite & (1 << 3 | 1 << 4));
        (*ret).next = (*global).global_allgc;
        (*global).global_allgc = ret;
        return ret;
    }
}
pub unsafe fn traverse_state(global: *mut Global, interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let mut o: *mut TValue = (*interpreter).stack.stkidrel_pointer;
        if (*interpreter).get_marked() & 7 > 1 || (*global).global_gcstate as i32 == 0 {
            linkgclist_(&mut (*(interpreter as *mut ObjectWithGCList)), (*interpreter).getgclist(), &mut (*global).global_grayagain);
        }
        if o.is_null() {
            return 1;
        }
        while o < (*interpreter).top.stkidrel_pointer {
            if ((*o).is_collectable()) && (*(*o).value.value_object).get_marked() & (1 << 3 | 1 << 4) != 0 {
                really_mark_object(global, (*o).value.value_object);
            }
            o = o.offset(1);
        }
        let mut uv: *mut UpValue = (*interpreter).open_upvalue;
        while !uv.is_null() {
            if (*uv).get_marked() & (1 << 3 | 1 << 4) != 0 {
                really_mark_object(global, &mut (*(uv as *mut ObjectBase)));
            }
            uv = (*uv).u.open.next;
        }
        if (*global).global_gcstate as i32 == 2 {
            if !(*global).global_isemergency {
                (*interpreter).luad_shrinkstack();
            }
            o = (*interpreter).top.stkidrel_pointer;
            while o < ((*interpreter).stack_last.stkidrel_pointer).offset(5 as isize) {
                (*o).set_tag_variant(TagVariant::NilNil);
                o = o.offset(1);
            }
            if !((*interpreter).twups != interpreter) && !((*interpreter).open_upvalue).is_null() {
                (*interpreter).twups = (*global).global_twups;
                (*global).global_twups = interpreter;
            }
        }
        return 1 + ((*interpreter).stack_last.stkidrel_pointer).offset_from((*interpreter).stack.stkidrel_pointer) as i32;
    }
}
pub unsafe fn sweeptolive(interpreter: *mut Interpreter, mut p: *mut *mut ObjectBase) -> *mut *mut ObjectBase {
    unsafe {
        let old: *mut *mut ObjectBase = p;
        loop {
            p = (*interpreter).sweep_list(p, 1, null_mut());
            if !(p == old) {
                break;
            }
        }
        return p;
    }
}
pub unsafe fn dothecall(interpreter: *mut Interpreter, mut _ud: *mut libc::c_void) {
    unsafe {
        luad_callnoyield(interpreter, (*interpreter).top.stkidrel_pointer.offset(-(2 as isize)), 0);
    }
}
pub unsafe fn gctm_function(interpreter: *mut Interpreter) {
    unsafe {
        let global: *mut Global = (*interpreter).global;
        let tm: *const TValue;
        let mut v: TValue = TValue::new(TagVariant::NilNil);
        let io: *mut TValue = &mut v;
        let i_g: *mut ObjectBase = (*global).udata2finalize();
        (*io).value.value_object = i_g;
        (*io).set_tag_variant((*i_g).get_tag_variant());
        (*io).set_collectable(true);
        tm = luat_gettmbyobj(interpreter, &mut v, TM_GC);
        if !(*tm).is_tagtype_nil() {
            let oldah: u8 = (*interpreter).allow_hook;
            let oldgcstp: i32 = (*global).global_gcstep as i32;
            (*global).global_gcstep = ((*global).global_gcstep as i32 | 2) as u8;
            (*interpreter).allow_hook = 0;
            let fresh15 = (*interpreter).top.stkidrel_pointer;
            (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
            let io1: *mut TValue = &mut (*fresh15);
            let io2: *const TValue = tm;
            (*io1).copy_from(&*io2);
            let fresh16 = (*interpreter).top.stkidrel_pointer;
            (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
            let io1_0: *mut TValue = &mut (*fresh16);
            let io2_0: *const TValue = &mut v;
            (*io1_0).copy_from(&(*io2_0));
            (*(*interpreter).callinfo).call_info_call_status = ((*(*interpreter).callinfo).call_info_call_status as i32 | 1 << 7) as u16;
            let status = luad_pcall(
                interpreter,
                Some(dothecall as unsafe fn(*mut Interpreter, *mut libc::c_void) -> ()),
                null_mut(),
                ((*interpreter).top.stkidrel_pointer.offset(-(2 as isize)) as *mut i8).offset_from((*interpreter).stack.stkidrel_pointer as *mut i8) as i64,
                0,
            );
            (*(*interpreter).callinfo).call_info_call_status = ((*(*interpreter).callinfo).call_info_call_status as i32 & !(1 << 7)) as u16;
            (*interpreter).allow_hook = oldah;
            (*global).global_gcstep = oldgcstp as u8;
            if status != Status::OK {
                luae_warnerror(interpreter, c"__gc".as_ptr());
                (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(-1);
            }
        }
    }
}
pub unsafe fn runafewfinalizers(interpreter: *mut Interpreter, n: i32) -> i32 {
    unsafe {
        let global: *mut Global = (*interpreter).global;
        let mut i: i32;
        i = 0;
        while i < n && !((*global).global_tobefinalized).is_null() {
            gctm_function(interpreter);
            i += 1;
        }
        return i;
    }
}
pub unsafe fn luac_checkfinalizer(interpreter: *mut Interpreter, o: *mut ObjectBase, metatable: *mut Table) {
    unsafe {
        let global: *mut Global = (*interpreter).global;
        if (*o).get_marked() & 1 << 6 != 0
            || (if metatable.is_null() {
                null()
            } else {
                if (*metatable).flags as u32 & (1 as u32) << TM_GC as i32 != 0 {
                    null()
                } else {
                    luat_gettm(metatable, TM_GC, (*global).global_tmname[TM_GC as usize])
                }
            })
            .is_null()
            || (*global).global_gcstep as i32 & 4 != 0
        {
            return;
        } else {
            if 3 <= (*global).global_gcstate as i32 && (*global).global_gcstate as i32 <= 6 {
                (*o).set_marked((*o).get_marked() & !(1 << 5 | (1 << 3 | 1 << 4)) | ((*global).global_currentwhite & (1 << 3 | 1 << 4)));
                if (*global).global_sweepgc == &mut (*o).next as *mut *mut ObjectBase {
                    (*global).global_sweepgc = sweeptolive(interpreter, (*global).global_sweepgc);
                }
            } else {
                (*global).correct_pointers(o);
            }
            let mut p: *mut *mut ObjectBase = &mut (*global).global_allgc;
            while *p != o {
                p = &mut (**p).next;
            }
            *p = (*o).next;
            (*o).next = (*global).global_finalizedobjects;
            (*global).global_finalizedobjects = o;
            (*o).set_marked(((*o).get_marked() | 1 << 6) as u8);
        };
    }
}
pub unsafe fn sweep2old(interpreter: *mut Interpreter, mut p: *mut *mut ObjectBase) {
    unsafe {
        let global: *mut Global = (*interpreter).global;
        loop {
            let curr: *mut ObjectBase = *p;
            if curr.is_null() {
                break;
            }
            if (*curr).get_marked() & (1 << 3 | 1 << 4) != 0 {
                *p = (*curr).next;
                free_object(interpreter, curr);
            } else {
                (*curr).set_marked((*curr).get_marked() & !(7) | 4);
                if (*curr).get_tag_variant() == TagVariant::Interpreter {
                    let other_state: *mut Interpreter = &mut (*(curr as *mut Interpreter));
                    linkgclist_(&mut (*(other_state as *mut ObjectWithGCList)),  (*other_state).getgclist(), &mut (*global).global_grayagain);
                } else if (*curr).get_tag_variant() == TagVariant::UpValue && (*(curr as *mut UpValue)).v.p != &mut (*(curr as *mut UpValue)).u.value as *mut TValue {
                    (*curr).set_marked((*curr).get_marked() & !(1 << 5 | (1 << 3 | 1 << 4)));
                } else {
                    (*curr).set_marked((*curr).get_marked() | 1 << 5);
                }
                p = &mut (*curr).next;
            }
        }
    }
}
pub unsafe fn sweepgen(interpreter: *mut Interpreter, global: *mut Global, mut p: *mut *mut ObjectBase, limit: *mut ObjectBase, pfirstold1: *mut *mut ObjectBase) -> *mut *mut ObjectBase {
    unsafe {
        static mut NEXT_AGE: [u8; 7] = [1, 3 as u8, 3 as u8, 4 as u8, 4 as u8, 5 as u8, 6 as u8];
        let white = (*global).global_currentwhite & (1 << 3 | 1 << 4);
        loop {
            let curr: *mut ObjectBase = *p;
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
                    (*curr).set_marked((*curr).get_marked() & !(7) | NEXT_AGE[((*curr).get_marked() & 7) as usize]);
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
pub unsafe fn callclosemethod(interpreter: *mut Interpreter, obj: *mut TValue, err: *mut TValue, yy: i32) {
    unsafe {
        let top: *mut TValue = (*interpreter).top.stkidrel_pointer;
        let tm: *const TValue = luat_gettmbyobj(interpreter, obj, TM_CLOSE);
        let io1: *mut TValue = &mut (*top);
        let io2: *const TValue = tm;
        (*io1).copy_from(&*io2);
        let io1_0: *mut TValue = &mut (*top.offset(1 as isize));
        let io2_0: *const TValue = obj;
        (*io1_0).copy_from(&(*io2_0));
        let io1_1: *mut TValue = &mut (*top.offset(2 as isize));
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
pub unsafe fn checkclosemth(interpreter: *mut Interpreter, level: *mut TValue) {
    unsafe {
        let tm: *const TValue = luat_gettmbyobj(interpreter, &mut (*level), TM_CLOSE);
        if (*tm).is_tagtype_nil() {
            let index: i32 = level.offset_from((*(*interpreter).callinfo).call_info_function.stkidrel_pointer) as i32;
            let mut vname: *const i8 = luag_findlocal(interpreter, (*interpreter).callinfo, index, null_mut());
            if vname.is_null() {
                vname = c"?".as_ptr();
            }
            luag_runerror(interpreter, c"variable '%s' got a non-closable value".as_ptr(), vname);
        }
    }
}
pub unsafe fn prepcallclosemth(interpreter: *mut Interpreter, level: *mut TValue, status: Status, yy: i32) {
    unsafe {
        let uv: *mut TValue = &mut (*level);
        let errobj: *mut TValue;
        if status == Status::Closing {
            errobj = &mut (*(*interpreter).global).global_nonevalue;
        } else {
            errobj = &mut (*level.offset(1 as isize));
            (*interpreter).set_error_object(status, level.offset(1 as isize));
        }
        callclosemethod(interpreter, uv, errobj, yy);
    }
}
pub unsafe fn luaf_newtbcupval(interpreter: *mut Interpreter, level: *mut TValue) {
    unsafe {
        if (*level).get_tag_variant() == TagVariant::BooleanFalse || (*level).is_tagtype_nil() {
            return;
        }
        checkclosemth(interpreter, level);
        while level.offset_from((*interpreter).tbc_list.stkidrel_pointer) as usize > ((256 as usize) << (size_of::<u16>() as usize).wrapping_sub(1 as usize).wrapping_mul(8 as usize)).wrapping_sub(1 as usize) {
            (*interpreter).tbc_list.stkidrel_pointer = ((*interpreter).tbc_list.stkidrel_pointer).offset(((256 as usize) << (size_of::<u16>() as usize).wrapping_sub(1 as usize).wrapping_mul(8 as usize)).wrapping_sub(1 as usize) as isize);
            (*(*interpreter).tbc_list.stkidrel_pointer).delta = 0;
        }
        (*level).delta = level.offset_from((*interpreter).tbc_list.stkidrel_pointer) as u16;
        (*interpreter).tbc_list.stkidrel_pointer = level;
    }
}
pub unsafe fn luaf_closeupval(interpreter: *mut Interpreter, level: *mut TValue) {
    unsafe {
        loop {
            let uv: *mut UpValue = (*interpreter).open_upvalue;
            let upl: *mut TValue;
            if !(!uv.is_null() && {
                upl = (*uv).v.p as *mut TValue;
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
                    if (*uv).get_marked() & 1 << 5 != 0 && (*(*slot).value.value_object).get_marked() & (1 << 3 | 1 << 4) != 0 {
                        luac_barrier_(interpreter, &mut (*(uv as *mut ObjectBase)), &mut (*((*slot).value.value_object as *mut ObjectBase)));
                    } else {
                    };
                } else {
                };
            }
        }
    }
}
pub unsafe fn poptbclist(interpreter: *mut Interpreter) {
    unsafe {
        let mut tbc: *mut TValue = (*interpreter).tbc_list.stkidrel_pointer;
        tbc = tbc.offset(-((*tbc).delta as isize));
        while tbc > (*interpreter).stack.stkidrel_pointer && (*tbc).delta == 0 {
            tbc = tbc.offset(-(((256 as usize) << (size_of::<u16>() as usize).wrapping_sub(1 as usize).wrapping_mul(8 as usize)).wrapping_sub(1 as usize) as isize));
        }
        (*interpreter).tbc_list.stkidrel_pointer = tbc;
    }
}
pub unsafe fn luaf_close(interpreter: *mut Interpreter, mut level: *mut TValue, status: Status, yy: i32) -> *mut TValue {
    unsafe {
        let levelrel: i64 = (level as *mut i8).offset_from((*interpreter).stack.stkidrel_pointer as *mut i8) as i64;
        luaf_closeupval(interpreter, level);
        while (*interpreter).tbc_list.stkidrel_pointer >= level {
            let tbc: *mut TValue = (*interpreter).tbc_list.stkidrel_pointer;
            poptbclist(interpreter);
            prepcallclosemth(interpreter, tbc, status, yy);
            level = ((*interpreter).stack.stkidrel_pointer as *mut i8).offset(levelrel as isize) as *mut TValue;
        }
        return level;
    }
}
pub unsafe fn luay_parser(interpreter: *mut Interpreter, zio: *mut ZIO, buffer: *mut Buffer, dynamic_data: *mut DynamicData, name: *const i8, firstchar: i32) -> *mut Closure {
    unsafe {
        let mut lexstate: LexicalState = LexicalState::new();
        let mut funcstate: FunctionState = FunctionState::new();
        let closure: *mut Closure = luaf_newlclosure(interpreter, 1);
        let io: *mut TValue = &mut (*(*interpreter).top.stkidrel_pointer);
        let x_: *mut Closure = closure;
        (*io).value.value_object = &mut (*(x_ as *mut ObjectBase));
        (*io).set_tag_variant(TagVariant::ClosureL);
        (*io).set_collectable(true);
        (*interpreter).luad_inctop();
        lexstate.table = luah_new(interpreter);
        let io_0: *mut TValue = &mut (*(*interpreter).top.stkidrel_pointer);
        let x0: *mut Table = lexstate.table;
        (*io_0).value.value_object = &mut (*(x0 as *mut ObjectBase));
        (*io_0).set_tag_variant(TagVariant::Table);
        (*io_0).set_collectable(true);
        (*interpreter).luad_inctop();
        (*closure).payload.l_prototype = luaf_newproto(interpreter);
        funcstate.prototype = (*closure).payload.l_prototype;
        if (*closure).get_marked() & 1 << 5 != 0 && (*(*closure).payload.l_prototype).get_marked() & (1 << 3 | 1 << 4) != 0 {
            luac_barrier_(interpreter, &mut (*(closure as *mut ObjectBase)), &mut (*((*closure).payload.l_prototype as *mut ObjectBase)));
        } else {
        };
        (*funcstate.prototype).prototype_source = luas_new(interpreter, name);
        if (*funcstate.prototype).get_marked() & 1 << 5 != 0 && (*(*funcstate.prototype).prototype_source).get_marked() & (1 << 3 | 1 << 4) != 0 {
            luac_barrier_(interpreter, &mut (*(funcstate.prototype as *mut ObjectBase)), &mut (*((*funcstate.prototype).prototype_source as *mut ObjectBase)));
        } else {
        };
        lexstate.buffer = buffer;
        lexstate.dynamic_data = dynamic_data;
        (*dynamic_data).labels.zero_length();
        (*dynamic_data).goto_.set_length((*dynamic_data).labels.get_length() as usize);
        (*dynamic_data).active_variables.set_length((*dynamic_data).goto_.get_length() as usize);
        luax_setinput(interpreter, &mut lexstate, zio, (*funcstate.prototype).prototype_source, firstchar);
        handle_main_function(interpreter, &mut lexstate, &mut funcstate);
        (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(-1);
        return closure;
    }
}
pub unsafe fn luax_init(interpreter: *mut Interpreter) {
    unsafe {
        let env_string: *mut TString = luas_newlstr(interpreter, c"_ENV".as_ptr(), "_ENV".len());
        fix_object_state(interpreter, &mut (*(env_string as *mut ObjectBase)));
        let mut i: i32 = 0;
        while i < Token::While as i32 - 255 {
            let tstring: *mut TString = luas_new(interpreter, TOKENS[i as usize]);
            fix_object_state(interpreter, &mut (*(tstring as *mut ObjectBase)));
            (*tstring).extra = (i + 1) as u8;
            i += 1;
        }
    }
}
pub unsafe fn pushclosure(interpreter: *mut Interpreter, p: *mut Prototype, encup: *mut *mut UpValue, base: *mut TValue, ra: *mut TValue) {
    unsafe {
        let count_upvalues = (*p).prototype_upvalues.get_size();
        let uv: *mut UpValueDescription = (*p).prototype_upvalues.vectort_pointer;
        let ncl: *mut Closure = luaf_newlclosure(interpreter, count_upvalues as i32);
        (*ncl).payload.l_prototype = p;
        let io: *mut TValue = &mut (*ra);
        let x_: *mut Closure = ncl;
        (*io).value.value_object = &mut (*(x_ as *mut ObjectBase));
        (*io).set_tag_variant(TagVariant::ClosureL);
        (*io).set_collectable(true);
        for i in 0..count_upvalues {
            if (*uv.offset(i as isize)).upvaluedescription_isinstack {
                let ref mut fresh136 = *((*ncl).upvalues).l_upvalues.as_mut_ptr().offset(i as isize);
                *fresh136 = luaf_findupval(interpreter, base.offset((*uv.offset(i as isize)).upvaluedescription_index as isize));
            } else {
                let ref mut fresh137 = *((*ncl).upvalues).l_upvalues.as_mut_ptr().offset(i as isize);
                *fresh137 = *encup.offset((*uv.offset(i as isize)).upvaluedescription_index as isize);
            }
            if (*ncl).get_marked() & 1 << 5 != 0 && (**((*ncl).upvalues).l_upvalues.as_mut_ptr().offset(i as isize)).get_marked() & (1 << 3 | 1 << 4) != 0 {
                luac_barrier_(interpreter, &mut (*(ncl as *mut ObjectBase)), &mut (*(*((*ncl).upvalues).l_upvalues.as_mut_ptr().offset(i as isize) as *mut ObjectBase)));
            } else {
            };
        }
    }
}
pub unsafe fn luav_finishop(interpreter: *mut Interpreter) {
    unsafe {
        let callinfo = (*interpreter).callinfo;
        let base: *mut TValue = ((*callinfo).call_info_function.stkidrel_pointer).offset(1 as isize);
        let inst: u32 = *((*callinfo).call_info_u.l.saved_program_counter).offset(-(1 as isize));
        let op: u32 = (inst >> 0 & !(!(0u32) << 7) << 0) as u32;
        match op as u32 {
            46 | 47 | 48 => {
                let io1: *mut TValue = &mut (*base.offset((*((*callinfo).call_info_u.l.saved_program_counter).offset(-(2 as isize)) >> POSITION_A & !(!(0u32) << 8) << 0) as isize));
                (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(-1);
                let io2: *const TValue = &mut (*(*interpreter).top.stkidrel_pointer);
                (*io1).copy_from(&*io2);
            },
            49 | 50 | 52 | 11 | 12 | 13 | 14 | 20 => {
                let io1_0: *mut TValue = &mut (*base.offset((inst >> POSITION_A & !(!(0u32) << 8) << 0) as isize));
                (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(-1);
                let io2_0: *const TValue = &mut (*(*interpreter).top.stkidrel_pointer);
                (*io1_0).copy_from(&*io2_0);
            },
            58 | 59 | 62 | 63 | 64 | 65 | 57 => {
                let res: i32 = !((*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))).get_tag_variant() == TagVariant::BooleanFalse || (*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))).get_tag_variant().to_tag_type() == TagType::Nil) as i32;
                (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(-1);
                if res != (inst >> POSITION_K & !(!(0u32) << 1) << 0) as i32 {
                    (*callinfo).call_info_u.l.saved_program_counter = ((*callinfo).call_info_u.l.saved_program_counter).offset(1);
                    (*callinfo).call_info_u.l.saved_program_counter;
                }
            },
            53 => {
                let top: *mut TValue = (*interpreter).top.stkidrel_pointer.offset(-(1 as isize));
                let a: i32 = (inst >> POSITION_A & !(!(0u32) << 8) << 0) as i32;
                let total: i32 = top.offset(-(1 as isize)).offset_from(base.offset(a as isize)) as i32;
                let io1_1: *mut TValue = &mut (*top.offset(-(2 as isize)));
                let io2_1: *const TValue = &mut (*top);
                (*io1_1).copy_from(&(*io2_1));
                (*interpreter).top.stkidrel_pointer = top.offset(-(1 as isize));
                concatenate(interpreter, total);
            },
            54 => {
                (*callinfo).call_info_u.l.saved_program_counter = ((*callinfo).call_info_u.l.saved_program_counter).offset(-1);
                (*callinfo).call_info_u.l.saved_program_counter;
            },
            70 => {
                let ra: *mut TValue = base.offset((inst >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                (*interpreter).top.stkidrel_pointer = ra.offset((*callinfo).call_info_u2.nres as isize);
                (*callinfo).call_info_u.l.saved_program_counter = ((*callinfo).call_info_u.l.saved_program_counter).offset(-1);
                (*callinfo).call_info_u.l.saved_program_counter;
            },
            _ => {},
        };
    }
}
pub unsafe fn luav_execute(interpreter: *mut Interpreter, mut callinfo: *mut CallInfo) {
    unsafe {
        let mut i: u32;
        let mut ra_65: *mut TValue;
        let mut new_call_info: *mut CallInfo;
        let mut b_4: i32;
        let mut count_results: i32;
        let mut current_block: usize;
        let mut closure: *mut Closure;
        let mut k: *mut TValue;
        let mut base: *mut TValue;
        let mut program_counter: *const u32;
        let mut trap: i32;
        '_startfunc: loop {
            trap = (*interpreter).hook_mask;
            '_returning: loop {
                closure = &mut (*((*(*callinfo).call_info_function.stkidrel_pointer).value.value_object as *mut Closure));
                k = (*(*closure).payload.l_prototype).prototype_constants.vectort_pointer;
                program_counter = (*callinfo).call_info_u.l.saved_program_counter;
                if (trap != 0) as i64 != 0 {
                    trap = luag_tracecall(interpreter);
                }
                base = ((*callinfo).call_info_function.stkidrel_pointer).offset(1 as isize);
                loop {
                    if (trap != 0) as i64 != 0 {
                        trap = luag_traceexec(interpreter, program_counter);
                        base = ((*callinfo).call_info_function.stkidrel_pointer).offset(1 as isize);
                    }
                    let fresh138 = program_counter;
                    program_counter = program_counter.offset(1);
                    i = *fresh138;
                    match (i >> 0 & !(!(0u32) << 7) << 0) as u32 {
                        0 => {
                            let ra: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let io1: *mut TValue = &mut (*ra);
                            let io2: *const TValue = &mut (*base.offset((i >> POSITION_B & !(!(0u32) << 8) << 0) as isize));
                            (*io1).copy_from(&*io2);
                            continue;
                        },
                        1 => {
                            let ra_0: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let b: i64 = ((i >> POSITION_K & !(!(0u32) << 8 + 8 + 1) << 0) as i32 - ((1 << 8 + 8 + 1) - 1 >> 1)) as i64;
                            let io: *mut TValue = &mut (*ra_0);
                            (*io).value.value_integer = b;
                            (*io).set_tag_variant(TagVariant::NumericInteger);
                            continue;
                        },
                        2 => {
                            let ra_1: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let b_0: i32 = (i >> POSITION_K & !(!(0u32) << 8 + 8 + 1) << 0) as i32 - ((1 << 8 + 8 + 1) - 1 >> 1);
                            let io_0: *mut TValue = &mut (*ra_1);
                            (*io_0).value.value_number = b_0 as f64;
                            (*io_0).set_tag_variant(TagVariant::NumericNumber);
                            continue;
                        },
                        3 => {
                            let ra_2: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let rb: *mut TValue = k.offset((i >> POSITION_K & !(!(0u32) << 8 + 8 + 1) << 0) as isize);
                            let io1_0: *mut TValue = &mut (*ra_2);
                            let io2_0: *const TValue = rb;
                            (*io1_0).copy_from(&(*io2_0));
                            continue;
                        },
                        4 => {
                            let ra_3: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let rb_0: *mut TValue = k.offset((*program_counter >> POSITION_A & !(!(0u32) << 8 + 8 + 1 + 8) << 0) as isize);
                            program_counter = program_counter.offset(1);
                            let io1_1: *mut TValue = &mut (*ra_3);
                            let io2_1: *const TValue = rb_0;
                            (*io1_1).copy_from(&(*io2_1));
                            continue;
                        },
                        5 => {
                            let ra_4: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            (*ra_4).set_tag_variant(TagVariant::BooleanFalse);
                            continue;
                        },
                        6 => {
                            let ra_5: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            (*ra_5).set_tag_variant(TagVariant::BooleanFalse);
                            program_counter = program_counter.offset(1);
                            continue;
                        },
                        7 => {
                            let ra_6: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            (*ra_6).set_tag_variant(TagVariant::BooleanTrue);
                            continue;
                        },
                        8 => {
                            let mut ra_7: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let mut b_1: i32 = (i >> POSITION_B & !(!(0u32) << 8) << 0) as i32;
                            loop {
                                let fresh139 = ra_7;
                                ra_7 = ra_7.offset(1);
                                (*fresh139).set_tag_variant(TagVariant::NilNil);
                                let fresh140 = b_1;
                                b_1 = b_1 - 1;
                                if !(fresh140 != 0) {
                                    break;
                                }
                            }
                            continue;
                        },
                        9 => {
                            let ra_8: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let b_2: i32 = (i >> POSITION_B & !(!(0u32) << 8) << 0) as i32;
                            let io1_2: *mut TValue = &mut (*ra_8);
                            let io2_2: *const TValue = (**((*closure).upvalues).l_upvalues.as_mut_ptr().offset(b_2 as isize)).v.p;
                            (*io1_2).copy_from(&(*io2_2));
                            continue;
                        },
                        10 => {
                            let ra_9: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let uv: *mut UpValue = *((*closure).upvalues).l_upvalues.as_mut_ptr().offset((i >> POSITION_B & !(!(0u32) << 8) << 0) as isize);
                            let io1_3: *mut TValue = (*uv).v.p;
                            let io2_3: *const TValue = &mut (*ra_9);
                            (*io1_3).copy_from(&(*io2_3));
                            if (*ra_9).is_collectable() {
                                if (*uv).get_marked() & 1 << 5 != 0 && (*(*ra_9).value.value_object).get_marked() & (1 << 3 | 1 << 4) != 0 {
                                    luac_barrier_(interpreter, &mut (*(uv as *mut ObjectBase)), &mut (*((*ra_9).value.value_object as *mut ObjectBase)));
                                } else {
                                };
                            } else {
                            };
                            continue;
                        },
                        11 => {
                            let ra_10: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let slot: *const TValue;
                            let count_upvalues: *mut TValue = (**((*closure).upvalues).l_upvalues.as_mut_ptr().offset((i >> POSITION_B & !(!(0u32) << 8) << 0) as isize)).v.p;
                            let rc: *mut TValue = k.offset((i >> POSITION_C & !(!(0u32) << 8) << 0) as isize);
                            let key: *mut TString = &mut (*((*rc).value.value_object as *mut TString));
                            if if !((*count_upvalues).get_tag_variant() == TagVariant::Table) {
                                slot = null();
                                0
                            } else {
                                slot = luah_getshortstr(&mut (*((*count_upvalues).value.value_object as *mut Table)), key);
                                !(*slot).is_tagtype_nil() as i32
                            } != 0
                            {
                                let io1_4: *mut TValue = &mut (*ra_10);
                                let io2_4: *const TValue = slot;
                                (*io1_4).copy_from(&(*io2_4));
                            } else {
                                (*callinfo).call_info_u.l.saved_program_counter = program_counter;
                                (*interpreter).top.stkidrel_pointer = (*callinfo).call_info_top.stkidrel_pointer;
                                luav_finishget(interpreter, count_upvalues, rc, ra_10, slot);
                                trap = (*callinfo).call_info_u.l.trap;
                            }
                            continue;
                        },
                        12 => {
                            let ra_11: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let slot_0: *const TValue;
                            let rb_1: *mut TValue = &mut (*base.offset((i >> POSITION_B & !(!(0u32) << 8) << 0) as isize));
                            let rc_0: *mut TValue = &mut (*base.offset((i >> POSITION_C & !(!(0u32) << 8) << 0) as isize));
                            let n: usize;
                            if if (*rc_0).get_tag_variant() == TagVariant::NumericInteger {
                                n = (*rc_0).value.value_integer as usize;
                                if !((*rb_1).get_tag_variant() == TagVariant::Table) {
                                    slot_0 = null();
                                    0
                                } else {
                                    slot_0 = if n.wrapping_sub(1 as usize) < (*((*rb_1).value.value_object as *mut Table)).array_limit as usize {
                                        &mut *((*((*rb_1).value.value_object as *mut Table)).array).offset(n.wrapping_sub(1 as usize) as isize) as *mut TValue as *const TValue
                                    } else {
                                        luah_getint(&mut (*((*rb_1).value.value_object as *mut Table)), n as i64)
                                    };
                                    !(*slot_0).is_tagtype_nil() as i32
                                }
                            } else if !((*rb_1).get_tag_variant() == TagVariant::Table) {
                                slot_0 = null();
                                0
                            } else {
                                slot_0 = luah_get(&mut (*((*rb_1).value.value_object as *mut Table)), rc_0);
                                !(*slot_0).is_tagtype_nil() as i32
                            } != 0
                            {
                                let io1_5: *mut TValue = &mut (*ra_11);
                                let io2_5: *const TValue = slot_0;
                                (*io1_5).copy_from(&(*io2_5));
                            } else {
                                (*callinfo).call_info_u.l.saved_program_counter = program_counter;
                                (*interpreter).top.stkidrel_pointer = (*callinfo).call_info_top.stkidrel_pointer;
                                luav_finishget(interpreter, rb_1, rc_0, ra_11, slot_0);
                                trap = (*callinfo).call_info_u.l.trap;
                            }
                            continue;
                        },
                        13 => {
                            let ra_12: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let slot_1: *const TValue;
                            let rb_2: *mut TValue = &mut (*base.offset((i >> POSITION_B & !(!(0u32) << 8) << 0) as isize));
                            let c: i32 = (i >> POSITION_C & !(!(0u32) << 8) << 0) as i32;
                            if if !((*rb_2).get_tag_variant() == TagVariant::Table) {
                                slot_1 = null();
                                0
                            } else {
                                slot_1 = if (c as usize).wrapping_sub(1 as usize) < (*((*rb_2).value.value_object as *mut Table)).array_limit as usize {
                                    &mut *((*((*rb_2).value.value_object as *mut Table)).array).offset((c - 1) as isize) as *mut TValue as *const TValue
                                } else {
                                    luah_getint(&mut (*((*rb_2).value.value_object as *mut Table)), c as i64)
                                };
                                !(*slot_1).is_tagtype_nil() as i32
                            } != 0
                            {
                                let io1_6: *mut TValue = &mut (*ra_12);
                                let io2_6: *const TValue = slot_1;
                                (*io1_6).copy_from(&(*io2_6));
                            } else {
                                let mut key_0: TValue = TValue::new(TagVariant::NilNil);
                                let io_1: *mut TValue = &mut key_0;
                                (*io_1).value.value_integer = c as i64;
                                (*io_1).set_tag_variant(TagVariant::NumericInteger);
                                (*callinfo).call_info_u.l.saved_program_counter = program_counter;
                                (*interpreter).top.stkidrel_pointer = (*callinfo).call_info_top.stkidrel_pointer;
                                luav_finishget(interpreter, rb_2, &mut key_0, ra_12, slot_1);
                                trap = (*callinfo).call_info_u.l.trap;
                            }
                            continue;
                        },
                        14 => {
                            let ra_13: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let slot_2: *const TValue;
                            let rb_3: *mut TValue = &mut (*base.offset((i >> POSITION_B & !(!(0u32) << 8) << 0) as isize));
                            let rc_1: *mut TValue = k.offset((i >> POSITION_C & !(!(0u32) << 8) << 0) as isize);
                            let key_1: *mut TString = &mut (*((*rc_1).value.value_object as *mut TString));
                            if if !((*rb_3).get_tag_variant() == TagVariant::Table) {
                                slot_2 = null();
                                0
                            } else {
                                slot_2 = luah_getshortstr(&mut (*((*rb_3).value.value_object as *mut Table)), key_1);
                                !(*slot_2).is_tagtype_nil() as i32
                            } != 0
                            {
                                let io1_7: *mut TValue = &mut (*ra_13);
                                let io2_7: *const TValue = slot_2;
                                (*io1_7).copy_from(&(*io2_7));
                            } else {
                                (*callinfo).call_info_u.l.saved_program_counter = program_counter;
                                (*interpreter).top.stkidrel_pointer = (*callinfo).call_info_top.stkidrel_pointer;
                                luav_finishget(interpreter, rb_3, rc_1, ra_13, slot_2);
                                trap = (*callinfo).call_info_u.l.trap;
                            }
                            continue;
                        },
                        15 => {
                            let slot_3: *const TValue;
                            let upval_0: *mut TValue = (**((*closure).upvalues).l_upvalues.as_mut_ptr().offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize)).v.p;
                            let rb_4: *mut TValue = k.offset((i >> POSITION_B & !(!(0u32) << 8) << 0) as isize);
                            let rc_2: *mut TValue = if (i & (1 as u32) << POSITION_K) as i32 != 0 {
                                k.offset((i >> POSITION_C & !(!(0u32) << 8) << 0) as isize)
                            } else {
                                &mut (*base.offset((i >> POSITION_C & !(!(0u32) << 8) << 0) as isize))
                            };
                            let key_2: *mut TString = &mut (*((*rb_4).value.value_object as *mut TString));
                            if if !((*upval_0).get_tag_variant() == TagVariant::Table) {
                                slot_3 = null();
                                0
                            } else {
                                slot_3 = luah_getshortstr(&mut (*((*upval_0).value.value_object as *mut Table)), key_2);
                                !((*slot_3).is_tagtype_nil()) as i32
                            } != 0
                            {
                                let io1_8: *mut TValue = slot_3 as *mut TValue;
                                let io2_8: *const TValue = rc_2;
                                (*io1_8).copy_from(&(*io2_8));
                                if (*rc_2).is_collectable() {
                                    if (*(*upval_0).value.value_object).get_marked() & 1 << 5 != 0 && (*(*rc_2).value.value_object).get_marked() & (1 << 3 | 1 << 4) != 0 {
                                        luac_barrierback_(interpreter, (*upval_0).value.value_object as *mut ObjectWithGCList);
                                    } else {
                                    };
                                } else {
                                };
                            } else {
                                (*callinfo).call_info_u.l.saved_program_counter = program_counter;
                                (*interpreter).top.stkidrel_pointer = (*callinfo).call_info_top.stkidrel_pointer;
                                luav_finishset(interpreter, upval_0, rb_4, rc_2, slot_3);
                                trap = (*callinfo).call_info_u.l.trap;
                            }
                            continue;
                        },
                        16 => {
                            let ra_14: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let slot_4: *const TValue;
                            let rb_5: *mut TValue = &mut (*base.offset((i >> POSITION_B & !(!(0u32) << 8) << 0) as isize));
                            let rc_3: *mut TValue = if (i & (1 as u32) << POSITION_K) as i32 != 0 {
                                k.offset((i >> POSITION_C & !(!(0u32) << 8) << 0) as isize)
                            } else {
                                &mut (*base.offset((i >> POSITION_C & !(!(0u32) << 8) << 0) as isize))
                            };
                            let n_0: usize;
                            if if (*rb_5).get_tag_variant() == TagVariant::NumericInteger {
                                n_0 = (*rb_5).value.value_integer as usize;
                                if !((*ra_14).get_tag_variant() == TagVariant::Table) {
                                    slot_4 = null();
                                    0
                                } else {
                                    slot_4 = if n_0.wrapping_sub(1 as usize) < (*((*ra_14).value.value_object as *mut Table)).array_limit as usize {
                                        &mut *((*((*ra_14).value.value_object as *mut Table)).array).offset(n_0.wrapping_sub(1 as usize) as isize) as *mut TValue as *const TValue
                                    } else {
                                        luah_getint(&mut (*((*ra_14).value.value_object as *mut Table)), n_0 as i64)
                                    };
                                    !(*slot_4).is_tagtype_nil() as i32
                                }
                            } else if !((*ra_14).get_tag_variant() == TagVariant::Table) {
                                slot_4 = null();
                                0
                            } else {
                                slot_4 = luah_get(&mut (*((*ra_14).value.value_object as *mut Table)), rb_5);
                                !((*slot_4).is_tagtype_nil()) as i32
                            } != 0
                            {
                                let io1_9: *mut TValue = slot_4 as *mut TValue;
                                let io2_9: *const TValue = rc_3;
                                (*io1_9).copy_from(&(*io2_9));
                                if (*rc_3).is_collectable() {
                                    if (*(*ra_14).value.value_object).get_marked() & 1 << 5 != 0 && (*(*rc_3).value.value_object).get_marked() & (1 << 3 | 1 << 4) != 0 {
                                        luac_barrierback_(interpreter, (*ra_14).value.value_object as *mut ObjectWithGCList);
                                    } else {
                                    };
                                } else {
                                };
                            } else {
                                (*callinfo).call_info_u.l.saved_program_counter = program_counter;
                                (*interpreter).top.stkidrel_pointer = (*callinfo).call_info_top.stkidrel_pointer;
                                luav_finishset(interpreter, &mut (*ra_14), rb_5, rc_3, slot_4);
                                trap = (*callinfo).call_info_u.l.trap;
                            }
                            continue;
                        },
                        17 => {
                            let ra_15: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let slot_5: *const TValue;
                            let c_0: i32 = (i >> POSITION_B & !(!(0u32) << 8) << 0) as i32;
                            let rc_4: *mut TValue = if (i & (1 as u32) << POSITION_K) as i32 != 0 {
                                k.offset((i >> POSITION_C & !(!(0u32) << 8) << 0) as isize)
                            } else {
                                &mut (*base.offset((i >> POSITION_C & !(!(0u32) << 8) << 0) as isize))
                            };
                            if if !((*ra_15).get_tag_variant() == TagVariant::Table) {
                                slot_5 = null();
                                0
                            } else {
                                slot_5 = if (c_0 as usize).wrapping_sub(1 as usize) < (*((*ra_15).value.value_object as *mut Table)).array_limit as usize {
                                    &mut *((*((*ra_15).value.value_object as *mut Table)).array).offset((c_0 - 1) as isize) as *mut TValue as *const TValue
                                } else {
                                    luah_getint(&mut (*((*ra_15).value.value_object as *mut Table)), c_0 as i64)
                                };
                                !((*slot_5).is_tagtype_nil()) as i32
                            } != 0
                            {
                                let io1_10: *mut TValue = slot_5 as *mut TValue;
                                let io2_10: *const TValue = rc_4;
                                (*io1_10).copy_from(&(*io2_10));
                                if (*rc_4).is_collectable() {
                                    if (*(*ra_15).value.value_object).get_marked() & 1 << 5 != 0 && (*(*rc_4).value.value_object).get_marked() & (1 << 3 | 1 << 4) != 0 {
                                        luac_barrierback_(interpreter, (*ra_15).value.value_object as *mut ObjectWithGCList);
                                    } else {
                                    };
                                } else {
                                };
                            } else {
                                let mut key_3: TValue = TValue::new(TagVariant::NilNil);
                                let io_2: *mut TValue = &mut key_3;
                                (*io_2).value.value_integer = c_0 as i64;
                                (*io_2).set_tag_variant(TagVariant::NumericInteger);
                                (*callinfo).call_info_u.l.saved_program_counter = program_counter;
                                (*interpreter).top.stkidrel_pointer = (*callinfo).call_info_top.stkidrel_pointer;
                                luav_finishset(interpreter, &mut (*ra_15), &mut key_3, rc_4, slot_5);
                                trap = (*callinfo).call_info_u.l.trap;
                            }
                            continue;
                        },
                        18 => {
                            let ra_16: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let slot_6: *const TValue;
                            let rb_6: *mut TValue = k.offset((i >> POSITION_B & !(!(0u32) << 8) << 0) as isize);
                            let rc_5: *mut TValue = if (i & (1 as u32) << POSITION_K) as i32 != 0 {
                                k.offset((i >> POSITION_C & !(!(0u32) << 8) << 0) as isize)
                            } else {
                                &mut (*base.offset((i >> POSITION_C & !(!(0u32) << 8) << 0) as isize))
                            };
                            let key_4: *mut TString = &mut (*((*rb_6).value.value_object as *mut TString));
                            if if !((*ra_16).get_tag_variant() == TagVariant::Table) {
                                slot_6 = null();
                                0
                            } else {
                                slot_6 = luah_getshortstr(&mut (*((*ra_16).value.value_object as *mut Table)), key_4);
                                !((*slot_6).is_tagtype_nil()) as i32
                            } != 0
                            {
                                let io1_11: *mut TValue = slot_6 as *mut TValue;
                                let io2_11: *const TValue = rc_5;
                                (*io1_11).copy_from(&(*io2_11));
                                if (*rc_5).is_collectable() {
                                    if (*(*ra_16).value.value_object).get_marked() & 1 << 5 != 0 && (*(*rc_5).value.value_object).get_marked() & (1 << 3 | 1 << 4) != 0 {
                                        luac_barrierback_(interpreter, (*ra_16).value.value_object as *mut ObjectWithGCList);
                                    } else {
                                    };
                                } else {
                                };
                            } else {
                                (*callinfo).call_info_u.l.saved_program_counter = program_counter;
                                (*interpreter).top.stkidrel_pointer = (*callinfo).call_info_top.stkidrel_pointer;
                                luav_finishset(interpreter, &mut (*ra_16), rb_6, rc_5, slot_6);
                                trap = (*callinfo).call_info_u.l.trap;
                            }
                            continue;
                        },
                        OPCODE_NEWTABLE => {
                            let ra_17: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let mut new_table_size = (i >> POSITION_B & !(!(0u32) << 8) << 0) as usize;
                            let mut new_array_size: usize = (i >> POSITION_C & !(!(0u32) << 8) << 0) as usize;
                            let table: *mut Table;
                            if new_table_size > 0 {
                                new_table_size = 1 << new_table_size - 1;
                            }
                            if (i & (1 as u32) << POSITION_K) as i32 != 0 {
                                new_array_size += ((*program_counter >> POSITION_A as usize & !(!(0u32) << 8 + 8 + 1 + 8) << 0) as i32 * ((1 << 8) - 1 + 1)) as usize;
                            }
                            program_counter = program_counter.offset(1);
                            (*interpreter).top.stkidrel_pointer = ra_17.offset(1 as isize);
                            table = luah_new(interpreter);
                            let io_3: *mut TValue = &mut (*ra_17);
                            let x_: *mut Table = table;
                            (*io_3).value.value_object = &mut (*(x_ as *mut ObjectBase));
                            (*io_3).set_tag_variant(TagVariant::Table);
                            (*io_3).set_collectable(true);
                            if new_table_size != 0 || new_array_size != 0 {
                                luah_resize(interpreter, table, new_array_size, new_table_size);
                            }
                            if (*(*interpreter).global).global_gcdebt > 0 {
                                (*callinfo).call_info_u.l.saved_program_counter = program_counter;
                                (*interpreter).top.stkidrel_pointer = ra_17.offset(1 as isize);
                                (*interpreter).luac_step();
                                trap = (*callinfo).call_info_u.l.trap;
                            }
                            continue;
                        },
                        20 => {
                            let ra_18: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let slot_7: *const TValue;
                            let rb_7: *mut TValue = &mut (*base.offset((i >> POSITION_B & !(!(0u32) << 8) << 0) as isize));
                            let rc_6: *mut TValue = if (i & (1 as u32) << POSITION_K) as i32 != 0 {
                                k.offset((i >> POSITION_C & !(!(0u32) << 8) << 0) as isize)
                            } else {
                                &mut (*base.offset((i >> POSITION_C & !(!(0u32) << 8) << 0) as isize))
                            };
                            let key_5: *mut TString = &mut (*((*rc_6).value.value_object as *mut TString));
                            let io1_12: *mut TValue = &mut (*ra_18.offset(1 as isize));
                            let io2_12: *const TValue = rb_7;
                            (*io1_12).copy_from(&(*io2_12));
                            if if !((*rb_7).get_tag_variant() == TagVariant::Table) {
                                slot_7 = null();
                                0
                            } else {
                                slot_7 = luah_getstr(&mut (*((*rb_7).value.value_object as *mut Table)), key_5);
                                !((*slot_7).is_tagtype_nil()) as i32
                            } != 0
                            {
                                let io1_13: *mut TValue = &mut (*ra_18);
                                let io2_13: *const TValue = slot_7;
                                (*io1_13).copy_from(&(*io2_13));
                            } else {
                                (*callinfo).call_info_u.l.saved_program_counter = program_counter;
                                (*interpreter).top.stkidrel_pointer = (*callinfo).call_info_top.stkidrel_pointer;
                                luav_finishget(interpreter, rb_7, rc_6, ra_18, slot_7);
                                trap = (*callinfo).call_info_u.l.trap;
                            }
                            continue;
                        },
                        21 => {
                            let ra_19: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let v1: *mut TValue = &mut (*base.offset((i >> POSITION_B & !(!(0u32) << 8) << 0) as isize));
                            let imm: i32 = (i >> POSITION_C & !(!(0u32) << 8) << 0) as i32 - ((1 << 8) - 1 >> 1);
                            if (*v1).get_tag_variant() == TagVariant::NumericInteger {
                                let iv1: i64 = (*v1).value.value_integer;
                                program_counter = program_counter.offset(1);
                                let io_4: *mut TValue = &mut (*ra_19);
                                (*io_4).value.value_integer = (iv1 as usize).wrapping_add(imm as usize) as i64;
                                (*io_4).set_tag_variant(TagVariant::NumericInteger);
                            } else if (*v1).get_tag_variant() == TagVariant::NumericNumber {
                                let nb: f64 = (*v1).value.value_number;
                                let fimm: f64 = imm as f64;
                                program_counter = program_counter.offset(1);
                                let io_5: *mut TValue = &mut (*ra_19);
                                (*io_5).value.value_number = nb + fimm;
                                (*io_5).set_tag_variant(TagVariant::NumericNumber);
                            }
                            continue;
                        },
                        22 => {
                            let v1_0: *mut TValue = &mut (*base.offset((i >> POSITION_B & !(!(0u32) << 8) << 0) as isize));
                            let v2: *mut TValue = k.offset((i >> POSITION_C & !(!(0u32) << 8) << 0) as isize);
                            let ra_20: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            if (*v1_0).get_tag_variant() == TagVariant::NumericInteger && (*v2).get_tag_variant() == TagVariant::NumericInteger {
                                let i1: i64 = (*v1_0).value.value_integer;
                                let i2: i64 = (*v2).value.value_integer;
                                program_counter = program_counter.offset(1);
                                let io_6: *mut TValue = &mut (*ra_20);
                                (*io_6).value.value_integer = (i1 as usize).wrapping_add(i2 as usize) as i64;
                                (*io_6).set_tag_variant(TagVariant::NumericInteger);
                            } else {
                                let mut n1: f64 = 0.0;
                                let mut n2: f64 = 0.0;
                                if (if (*v1_0).get_tag_variant() == TagVariant::NumericNumber {
                                    n1 = (*v1_0).value.value_number;
                                    1
                                } else {
                                    if (*v1_0).get_tag_variant() == TagVariant::NumericInteger {
                                        n1 = (*v1_0).value.value_integer as f64;
                                        1
                                    } else {
                                        0
                                    }
                                }) != 0
                                    && (if (*v2).get_tag_variant() == TagVariant::NumericNumber {
                                        n2 = (*v2).value.value_number;
                                        1
                                    } else {
                                        if (*v2).get_tag_variant() == TagVariant::NumericInteger {
                                            n2 = (*v2).value.value_integer as f64;
                                            1
                                        } else {
                                            0
                                        }
                                    }) != 0
                                {
                                    program_counter = program_counter.offset(1);
                                    let io_7: *mut TValue = &mut (*ra_20);
                                    (*io_7).value.value_number = n1 + n2;
                                    (*io_7).set_tag_variant(TagVariant::NumericNumber);
                                }
                            }
                            continue;
                        },
                        OPCODE_SUBK => {
                            let v1_1: *mut TValue = &mut (*base.offset((i >> POSITION_B & !(!(0u32) << 8) << 0) as isize));
                            let v2_0: *mut TValue = k.offset((i >> POSITION_C & !(!(0u32) << 8) << 0) as isize);
                            let ra_21: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            if (*v1_1).get_tag_variant() == TagVariant::NumericInteger && (*v2_0).get_tag_variant() == TagVariant::NumericInteger {
                                let i1_0: i64 = (*v1_1).value.value_integer;
                                let i2_0: i64 = (*v2_0).value.value_integer;
                                program_counter = program_counter.offset(1);
                                let io_8: *mut TValue = &mut (*ra_21);
                                (*io_8).value.value_integer = (i1_0 as usize).wrapping_sub(i2_0 as usize) as i64;
                                (*io_8).set_tag_variant(TagVariant::NumericInteger);
                            } else {
                                let mut n1_0: f64 = 0.0;
                                let mut n2_0: f64 = 0.0;
                                if (if (*v1_1).get_tag_variant() == TagVariant::NumericNumber {
                                    n1_0 = (*v1_1).value.value_number;
                                    1
                                } else {
                                    if (*v1_1).get_tag_variant() == TagVariant::NumericInteger {
                                        n1_0 = (*v1_1).value.value_integer as f64;
                                        1
                                    } else {
                                        0
                                    }
                                }) != 0
                                    && (if (*v2_0).get_tag_variant() == TagVariant::NumericNumber {
                                        n2_0 = (*v2_0).value.value_number;
                                        1
                                    } else {
                                        if (*v2_0).get_tag_variant() == TagVariant::NumericInteger {
                                            n2_0 = (*v2_0).value.value_integer as f64;
                                            1
                                        } else {
                                            0
                                        }
                                    }) != 0
                                {
                                    program_counter = program_counter.offset(1);
                                    let io_9: *mut TValue = &mut (*ra_21);
                                    (*io_9).value.value_number = n1_0 - n2_0;
                                    (*io_9).set_tag_variant(TagVariant::NumericNumber);
                                }
                            }
                            continue;
                        },
                        OPCODE_MULK => {
                            let v1_2: *mut TValue = &mut (*base.offset((i >> POSITION_B & !(!(0u32) << 8) << 0) as isize));
                            let v2_1: *mut TValue = k.offset((i >> POSITION_C & !(!(0u32) << 8) << 0) as isize);
                            let ra_22: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            if (*v1_2).get_tag_variant() == TagVariant::NumericInteger && (*v2_1).get_tag_variant() == TagVariant::NumericInteger {
                                let i1_1: i64 = (*v1_2).value.value_integer;
                                let i2_1: i64 = (*v2_1).value.value_integer;
                                program_counter = program_counter.offset(1);
                                let io_10: *mut TValue = &mut (*ra_22);
                                (*io_10).value.value_integer = (i1_1 as usize).wrapping_mul(i2_1 as usize) as i64;
                                (*io_10).set_tag_variant(TagVariant::NumericInteger);
                            } else {
                                let mut n1_1: f64 = 0.0;
                                let mut n2_1: f64 = 0.0;
                                if (if (*v1_2).get_tag_variant() == TagVariant::NumericNumber {
                                    n1_1 = (*v1_2).value.value_number;
                                    1
                                } else {
                                    if (*v1_2).get_tag_variant() == TagVariant::NumericInteger {
                                        n1_1 = (*v1_2).value.value_integer as f64;
                                        1
                                    } else {
                                        0
                                    }
                                }) != 0
                                    && (if (*v2_1).get_tag_variant() == TagVariant::NumericNumber {
                                        n2_1 = (*v2_1).value.value_number;
                                        1
                                    } else {
                                        if (*v2_1).get_tag_variant() == TagVariant::NumericInteger {
                                            n2_1 = (*v2_1).value.value_integer as f64;
                                            1
                                        } else {
                                            0
                                        }
                                    }) != 0
                                {
                                    program_counter = program_counter.offset(1);
                                    let io_11: *mut TValue = &mut (*ra_22);
                                    (*io_11).value.value_number = n1_1 * n2_1;
                                    (*io_11).set_tag_variant(TagVariant::NumericNumber);
                                }
                            }
                            continue;
                        },
                        OPCODE_MODK => {
                            (*callinfo).call_info_u.l.saved_program_counter = program_counter;
                            (*interpreter).top.stkidrel_pointer = (*callinfo).call_info_top.stkidrel_pointer;
                            let v1_3: *mut TValue = &mut (*base.offset((i >> POSITION_B & !(!(0u32) << 8) << 0) as isize));
                            let v2_2: *mut TValue = k.offset((i >> POSITION_C & !(!(0u32) << 8) << 0) as isize);
                            let ra_23: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            if (*v1_3).get_tag_variant() == TagVariant::NumericInteger && (*v2_2).get_tag_variant() == TagVariant::NumericInteger {
                                let i1_2: i64 = (*v1_3).value.value_integer;
                                let i2_2: i64 = (*v2_2).value.value_integer;
                                program_counter = program_counter.offset(1);
                                let io_12: *mut TValue = &mut (*ra_23);
                                (*io_12).value.value_integer = luav_mod(interpreter, i1_2, i2_2);
                                (*io_12).set_tag_variant(TagVariant::NumericInteger);
                            } else {
                                let mut n1_2: f64 = 0.0;
                                let mut n2_2: f64 = 0.0;
                                if (if (*v1_3).get_tag_variant() == TagVariant::NumericNumber {
                                    n1_2 = (*v1_3).value.value_number;
                                    1
                                } else {
                                    if (*v1_3).get_tag_variant() == TagVariant::NumericInteger {
                                        n1_2 = (*v1_3).value.value_integer as f64;
                                        1
                                    } else {
                                        0
                                    }
                                }) != 0
                                    && (if (*v2_2).get_tag_variant() == TagVariant::NumericNumber {
                                        n2_2 = (*v2_2).value.value_number;
                                        1
                                    } else {
                                        if (*v2_2).get_tag_variant() == TagVariant::NumericInteger {
                                            n2_2 = (*v2_2).value.value_integer as f64;
                                            1
                                        } else {
                                            0
                                        }
                                    }) != 0
                                {
                                    program_counter = program_counter.offset(1);
                                    let io_13: *mut TValue = &mut (*ra_23);
                                    (*io_13).value.value_number = luav_modf(interpreter, n1_2, n2_2);
                                    (*io_13).set_tag_variant(TagVariant::NumericNumber);
                                }
                            }
                            continue;
                        },
                        OPCODE_POWK => {
                            let ra_24: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let v1_4: *mut TValue = &mut (*base.offset((i >> POSITION_B & !(!(0u32) << 8) << 0) as isize));
                            let v2_3: *mut TValue = k.offset((i >> POSITION_C & !(!(0u32) << 8) << 0) as isize);
                            let mut n1_3: f64 = 0.0;
                            let mut n2_3: f64 = 0.0;
                            if (if (*v1_4).get_tag_variant() == TagVariant::NumericNumber {
                                n1_3 = (*v1_4).value.value_number;
                                1
                            } else {
                                if (*v1_4).get_tag_variant() == TagVariant::NumericInteger {
                                    n1_3 = (*v1_4).value.value_integer as f64;
                                    1
                                } else {
                                    0
                                }
                            }) != 0
                                && (if (*v2_3).get_tag_variant() == TagVariant::NumericNumber {
                                    n2_3 = (*v2_3).value.value_number;
                                    1
                                } else {
                                    if (*v2_3).get_tag_variant() == TagVariant::NumericInteger {
                                        n2_3 = (*v2_3).value.value_integer as f64;
                                        1
                                    } else {
                                        0
                                    }
                                }) != 0
                            {
                                program_counter = program_counter.offset(1);
                                let io_14: *mut TValue = &mut (*ra_24);
                                (*io_14).value.value_number = if n2_3 == 2.0 { n1_3 * n1_3 } else { n1_3.powf(n2_3) };
                                (*io_14).set_tag_variant(TagVariant::NumericNumber);
                            }
                            continue;
                        },
                        OPCODE_DIVK => {
                            let ra_25: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let v1_5: *mut TValue = &mut (*base.offset((i >> POSITION_B & !(!(0u32) << 8) << 0) as isize));
                            let v2_4: *mut TValue = k.offset((i >> POSITION_C & !(!(0u32) << 8) << 0) as isize);
                            let mut n1_4: f64 = 0.0;
                            let mut n2_4: f64 = 0.0;
                            if (if (*v1_5).get_tag_variant() == TagVariant::NumericNumber {
                                n1_4 = (*v1_5).value.value_number;
                                1
                            } else {
                                if (*v1_5).get_tag_variant() == TagVariant::NumericInteger {
                                    n1_4 = (*v1_5).value.value_integer as f64;
                                    1
                                } else {
                                    0
                                }
                            }) != 0
                                && (if (*v2_4).get_tag_variant() == TagVariant::NumericNumber {
                                    n2_4 = (*v2_4).value.value_number;
                                    1
                                } else {
                                    if (*v2_4).get_tag_variant() == TagVariant::NumericInteger {
                                        n2_4 = (*v2_4).value.value_integer as f64;
                                        1
                                    } else {
                                        0
                                    }
                                }) != 0
                            {
                                program_counter = program_counter.offset(1);
                                let io_15: *mut TValue = &mut (*ra_25);
                                (*io_15).value.value_number = n1_4 / n2_4;
                                (*io_15).set_tag_variant(TagVariant::NumericNumber);
                            }
                            continue;
                        },
                        OPCODE_IDIVK => {
                            (*callinfo).call_info_u.l.saved_program_counter = program_counter;
                            (*interpreter).top.stkidrel_pointer = (*callinfo).call_info_top.stkidrel_pointer;
                            let v1_6: *mut TValue = &mut (*base.offset((i >> POSITION_B & !(!(0u32) << 8) << 0) as isize));
                            let v2_5: *mut TValue = k.offset((i >> POSITION_C & !(!(0u32) << 8) << 0) as isize);
                            let ra_26: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            if (*v1_6).get_tag_variant() == TagVariant::NumericInteger && (*v2_5).get_tag_variant() == TagVariant::NumericInteger {
                                let i1_3: i64 = (*v1_6).value.value_integer;
                                let i2_3: i64 = (*v2_5).value.value_integer;
                                program_counter = program_counter.offset(1);
                                let io_16: *mut TValue = &mut (*ra_26);
                                (*io_16).value.value_integer = luav_idiv(interpreter, i1_3, i2_3);
                                (*io_16).set_tag_variant(TagVariant::NumericInteger);
                            } else {
                                let mut n1_5: f64 = 0.0;
                                let mut n2_5: f64 = 0.0;
                                if (if (*v1_6).get_tag_variant() == TagVariant::NumericNumber {
                                    n1_5 = (*v1_6).value.value_number;
                                    1
                                } else {
                                    if (*v1_6).get_tag_variant() == TagVariant::NumericInteger {
                                        n1_5 = (*v1_6).value.value_integer as f64;
                                        1
                                    } else {
                                        0
                                    }
                                }) != 0
                                    && (if (*v2_5).get_tag_variant() == TagVariant::NumericNumber {
                                        n2_5 = (*v2_5).value.value_number;
                                        1
                                    } else {
                                        if (*v2_5).get_tag_variant() == TagVariant::NumericInteger {
                                            n2_5 = (*v2_5).value.value_integer as f64;
                                            1
                                        } else {
                                            0
                                        }
                                    }) != 0
                                {
                                    program_counter = program_counter.offset(1);
                                    let io_17: *mut TValue = &mut (*ra_26);
                                    (*io_17).value.value_number = (n1_5 / n2_5).floor();
                                    (*io_17).set_tag_variant(TagVariant::NumericNumber);
                                }
                            }
                            continue;
                        },
                        OPCODE_BANDK => {
                            let ra_27: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let v1_7: *mut TValue = &mut (*base.offset((i >> POSITION_B & !(!(0u32) << 8) << 0) as isize));
                            let v2_6: *mut TValue = k.offset((i >> POSITION_C & !(!(0u32) << 8) << 0) as isize);
                            let mut i1_4: i64 = 0;
                            let i2_4: i64 = (*v2_6).value.value_integer;
                            if if (((*v1_7).get_tag_variant() == TagVariant::NumericInteger) as i32 != 0) as i64 != 0 {
                                i1_4 = (*v1_7).value.value_integer;
                                1
                            } else {
                                luav_tointegerns(v1_7, &mut i1_4, F2I::Equal)
                            } != 0
                            {
                                program_counter = program_counter.offset(1);
                                let io_18: *mut TValue = &mut (*ra_27);
                                (*io_18).value.value_integer = (i1_4 as usize & i2_4 as usize) as i64;
                                (*io_18).set_tag_variant(TagVariant::NumericInteger);
                            }
                            continue;
                        },
                        OPCODE_BORK => {
                            let ra_28: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let v1_8: *mut TValue = &mut (*base.offset((i >> POSITION_B & !(!(0u32) << 8) << 0) as isize));
                            let v2_7: *mut TValue = k.offset((i >> POSITION_C & !(!(0u32) << 8) << 0) as isize);
                            let mut i1_5: i64 = 0;
                            let i2_5: i64 = (*v2_7).value.value_integer;
                            if if (((*v1_8).get_tag_variant() == TagVariant::NumericInteger) as i32 != 0) as i64 != 0 {
                                i1_5 = (*v1_8).value.value_integer;
                                1
                            } else {
                                luav_tointegerns(v1_8, &mut i1_5, F2I::Equal)
                            } != 0
                            {
                                program_counter = program_counter.offset(1);
                                let io_19: *mut TValue = &mut (*ra_28);
                                (*io_19).value.value_integer = (i1_5 as usize | i2_5 as usize) as i64;
                                (*io_19).set_tag_variant(TagVariant::NumericInteger);
                            }
                            continue;
                        },
                        OPCODE_BXORK => {
                            let ra_29: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let v1_9: *mut TValue = &mut (*base.offset((i >> POSITION_B & !(!(0u32) << 8) << 0) as isize));
                            let v2_8: *mut TValue = k.offset((i >> POSITION_C & !(!(0u32) << 8) << 0) as isize);
                            let mut i1_6: i64 = 0;
                            let i2_6: i64 = (*v2_8).value.value_integer;
                            if if (((*v1_9).get_tag_variant() == TagVariant::NumericInteger) as i32 != 0) as i64 != 0 {
                                i1_6 = (*v1_9).value.value_integer;
                                1
                            } else {
                                luav_tointegerns(v1_9, &mut i1_6, F2I::Equal)
                            } != 0
                            {
                                program_counter = program_counter.offset(1);
                                let io_20: *mut TValue = &mut (*ra_29);
                                (*io_20).value.value_integer = (i1_6 as usize ^ i2_6 as usize) as i64;
                                (*io_20).set_tag_variant(TagVariant::NumericInteger);
                            }
                            continue;
                        },
                        32 => {
                            let ra_30: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let rb_8: *mut TValue = &mut (*base.offset((i >> POSITION_B & !(!(0u32) << 8) << 0) as isize));
                            let ic: i32 = (i >> POSITION_C & !(!(0u32) << 8) << 0) as i32 - ((1 << 8) - 1 >> 1);
                            let mut ib: i64 = 0;
                            if if (((*rb_8).get_tag_variant() == TagVariant::NumericInteger) as i32 != 0) as i64 != 0 {
                                ib = (*rb_8).value.value_integer;
                                1
                            } else {
                                luav_tointegerns(rb_8, &mut ib, F2I::Equal)
                            } != 0
                            {
                                program_counter = program_counter.offset(1);
                                let io_21: *mut TValue = &mut (*ra_30);
                                (*io_21).value.value_integer = luav_shiftl(ib, -ic as i64);
                                (*io_21).set_tag_variant(TagVariant::NumericInteger);
                            }
                            continue;
                        },
                        33 => {
                            let ra_31: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let rb_9: *mut TValue = &mut (*base.offset((i >> POSITION_B & !(!(0u32) << 8) << 0) as isize));
                            let ic_0: i32 = (i >> POSITION_C & !(!(0u32) << 8) << 0) as i32 - ((1 << 8) - 1 >> 1);
                            let mut ib_0: i64 = 0;
                            if if (((*rb_9).get_tag_variant() == TagVariant::NumericInteger) as i32 != 0) as i64 != 0 {
                                ib_0 = (*rb_9).value.value_integer;
                                1
                            } else {
                                luav_tointegerns(rb_9, &mut ib_0, F2I::Equal)
                            } != 0
                            {
                                program_counter = program_counter.offset(1);
                                let io_22: *mut TValue = &mut (*ra_31);
                                (*io_22).value.value_integer = luav_shiftl(ic_0 as i64, ib_0);
                                (*io_22).set_tag_variant(TagVariant::NumericInteger);
                            }
                            continue;
                        },
                        34 => {
                            let v1_10: *mut TValue = &mut (*base.offset((i >> POSITION_B & !(!(0u32) << 8) << 0) as isize));
                            let v2_9: *mut TValue = &mut (*base.offset((i >> POSITION_C & !(!(0u32) << 8) << 0) as isize));
                            let ra_32: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            if (*v1_10).get_tag_variant() == TagVariant::NumericInteger && (*v2_9).get_tag_variant() == TagVariant::NumericInteger {
                                let i1_7: i64 = (*v1_10).value.value_integer;
                                let i2_7: i64 = (*v2_9).value.value_integer;
                                program_counter = program_counter.offset(1);
                                let io_23: *mut TValue = &mut (*ra_32);
                                (*io_23).value.value_integer = (i1_7 as usize).wrapping_add(i2_7 as usize) as i64;
                                (*io_23).set_tag_variant(TagVariant::NumericInteger);
                            } else {
                                let mut n1_6: f64 = 0.0;
                                let mut n2_6: f64 = 0.0;
                                if (if (*v1_10).get_tag_variant() == TagVariant::NumericNumber {
                                    n1_6 = (*v1_10).value.value_number;
                                    1
                                } else {
                                    if (*v1_10).get_tag_variant() == TagVariant::NumericInteger {
                                        n1_6 = (*v1_10).value.value_integer as f64;
                                        1
                                    } else {
                                        0
                                    }
                                }) != 0
                                    && (if (*v2_9).get_tag_variant() == TagVariant::NumericNumber {
                                        n2_6 = (*v2_9).value.value_number;
                                        1
                                    } else {
                                        if (*v2_9).get_tag_variant() == TagVariant::NumericInteger {
                                            n2_6 = (*v2_9).value.value_integer as f64;
                                            1
                                        } else {
                                            0
                                        }
                                    }) != 0
                                {
                                    program_counter = program_counter.offset(1);
                                    let io_24: *mut TValue = &mut (*ra_32);
                                    (*io_24).value.value_number = n1_6 + n2_6;
                                    (*io_24).set_tag_variant(TagVariant::NumericNumber);
                                }
                            }
                            continue;
                        },
                        OPCODE_SUB => {
                            let v1_11: *mut TValue = &mut (*base.offset((i >> POSITION_B & !(!(0u32) << 8) << 0) as isize));
                            let v2_10: *mut TValue = &mut (*base.offset((i >> POSITION_C & !(!(0u32) << 8) << 0) as isize));
                            let ra_33: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            if (*v1_11).get_tag_variant() == TagVariant::NumericInteger && (*v2_10).get_tag_variant() == TagVariant::NumericInteger {
                                let i1_8: i64 = (*v1_11).value.value_integer;
                                let i2_8: i64 = (*v2_10).value.value_integer;
                                program_counter = program_counter.offset(1);
                                let io_25: *mut TValue = &mut (*ra_33);
                                (*io_25).value.value_integer = (i1_8 as usize).wrapping_sub(i2_8 as usize) as i64;
                                (*io_25).set_tag_variant(TagVariant::NumericInteger);
                            } else {
                                let mut n1_7: f64 = 0.0;
                                let mut n2_7: f64 = 0.0;
                                if (if (*v1_11).get_tag_variant() == TagVariant::NumericNumber {
                                    n1_7 = (*v1_11).value.value_number;
                                    1
                                } else {
                                    if (*v1_11).get_tag_variant() == TagVariant::NumericInteger {
                                        n1_7 = (*v1_11).value.value_integer as f64;
                                        1
                                    } else {
                                        0
                                    }
                                }) != 0
                                    && (if (*v2_10).get_tag_variant() == TagVariant::NumericNumber {
                                        n2_7 = (*v2_10).value.value_number;
                                        1
                                    } else {
                                        if (*v2_10).get_tag_variant() == TagVariant::NumericInteger {
                                            n2_7 = (*v2_10).value.value_integer as f64;
                                            1
                                        } else {
                                            0
                                        }
                                    }) != 0
                                {
                                    program_counter = program_counter.offset(1);
                                    let io_26: *mut TValue = &mut (*ra_33);
                                    (*io_26).value.value_number = n1_7 - n2_7;
                                    (*io_26).set_tag_variant(TagVariant::NumericNumber);
                                }
                            }
                            continue;
                        },
                        OPCODE_MUL => {
                            let v1_12: *mut TValue = &mut (*base.offset((i >> POSITION_B & !(!(0u32) << 8) << 0) as isize));
                            let v2_11: *mut TValue = &mut (*base.offset((i >> POSITION_C & !(!(0u32) << 8) << 0) as isize));
                            let ra_34: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            if (*v1_12).get_tag_variant() == TagVariant::NumericInteger && (*v2_11).get_tag_variant() == TagVariant::NumericInteger {
                                let i1_9: i64 = (*v1_12).value.value_integer;
                                let i2_9: i64 = (*v2_11).value.value_integer;
                                program_counter = program_counter.offset(1);
                                let io_27: *mut TValue = &mut (*ra_34);
                                (*io_27).value.value_integer = (i1_9 as usize).wrapping_mul(i2_9 as usize) as i64;
                                (*io_27).set_tag_variant(TagVariant::NumericInteger);
                            } else {
                                let mut n1_8: f64 = 0.0;
                                let mut n2_8: f64 = 0.0;
                                if (if (*v1_12).get_tag_variant() == TagVariant::NumericNumber {
                                    n1_8 = (*v1_12).value.value_number;
                                    1
                                } else {
                                    if (*v1_12).get_tag_variant() == TagVariant::NumericInteger {
                                        n1_8 = (*v1_12).value.value_integer as f64;
                                        1
                                    } else {
                                        0
                                    }
                                }) != 0
                                    && (if (*v2_11).get_tag_variant() == TagVariant::NumericNumber {
                                        n2_8 = (*v2_11).value.value_number;
                                        1
                                    } else {
                                        if (*v2_11).get_tag_variant() == TagVariant::NumericInteger {
                                            n2_8 = (*v2_11).value.value_integer as f64;
                                            1
                                        } else {
                                            0
                                        }
                                    }) != 0
                                {
                                    program_counter = program_counter.offset(1);
                                    let io_28: *mut TValue = &mut (*ra_34);
                                    (*io_28).value.value_number = n1_8 * n2_8;
                                    (*io_28).set_tag_variant(TagVariant::NumericNumber);
                                }
                            }
                            continue;
                        },
                        OPCODE_MOD => {
                            (*callinfo).call_info_u.l.saved_program_counter = program_counter;
                            (*interpreter).top.stkidrel_pointer = (*callinfo).call_info_top.stkidrel_pointer;
                            let v1_13: *mut TValue = &mut (*base.offset((i >> POSITION_B & !(!(0u32) << 8) << 0) as isize));
                            let v2_12: *mut TValue = &mut (*base.offset((i >> POSITION_C & !(!(0u32) << 8) << 0) as isize));
                            let ra_35: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            if (*v1_13).get_tag_variant() == TagVariant::NumericInteger && (*v2_12).get_tag_variant() == TagVariant::NumericInteger {
                                let i1_10: i64 = (*v1_13).value.value_integer;
                                let i2_10: i64 = (*v2_12).value.value_integer;
                                program_counter = program_counter.offset(1);
                                let io_29: *mut TValue = &mut (*ra_35);
                                (*io_29).value.value_integer = luav_mod(interpreter, i1_10, i2_10);
                                (*io_29).set_tag_variant(TagVariant::NumericInteger);
                            } else {
                                let mut n1_9: f64 = 0.0;
                                let mut n2_9: f64 = 0.0;
                                if (if (*v1_13).get_tag_variant() == TagVariant::NumericNumber {
                                    n1_9 = (*v1_13).value.value_number;
                                    1
                                } else {
                                    if (*v1_13).get_tag_variant() == TagVariant::NumericInteger {
                                        n1_9 = (*v1_13).value.value_integer as f64;
                                        1
                                    } else {
                                        0
                                    }
                                }) != 0
                                    && (if (*v2_12).get_tag_variant() == TagVariant::NumericNumber {
                                        n2_9 = (*v2_12).value.value_number;
                                        1
                                    } else {
                                        if (*v2_12).get_tag_variant() == TagVariant::NumericInteger {
                                            n2_9 = (*v2_12).value.value_integer as f64;
                                            1
                                        } else {
                                            0
                                        }
                                    }) != 0
                                {
                                    program_counter = program_counter.offset(1);
                                    let io_30: *mut TValue = &mut (*ra_35);
                                    (*io_30).value.value_number = luav_modf(interpreter, n1_9, n2_9);
                                    (*io_30).set_tag_variant(TagVariant::NumericNumber);
                                }
                            }
                            continue;
                        },
                        OPCODE_POW => {
                            let ra_36: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let v1_14: *mut TValue = &mut (*base.offset((i >> POSITION_B & !(!(0u32) << 8) << 0) as isize));
                            let v2_13: *mut TValue = &mut (*base.offset((i >> POSITION_C & !(!(0u32) << 8) << 0) as isize));
                            let mut n1_10: f64 = 0.0;
                            let mut n2_10: f64 = 0.0;
                            if (if (*v1_14).get_tag_variant() == TagVariant::NumericNumber {
                                n1_10 = (*v1_14).value.value_number;
                                1
                            } else {
                                if (*v1_14).get_tag_variant() == TagVariant::NumericInteger {
                                    n1_10 = (*v1_14).value.value_integer as f64;
                                    1
                                } else {
                                    0
                                }
                            }) != 0
                                && (if (*v2_13).get_tag_variant() == TagVariant::NumericNumber {
                                    n2_10 = (*v2_13).value.value_number;
                                    1
                                } else {
                                    if (*v2_13).get_tag_variant() == TagVariant::NumericInteger {
                                        n2_10 = (*v2_13).value.value_integer as f64;
                                        1
                                    } else {
                                        0
                                    }
                                }) != 0
                            {
                                program_counter = program_counter.offset(1);
                                let io_31: *mut TValue = &mut (*ra_36);
                                (*io_31).value.value_number = if n2_10 == 2.0 { n1_10 * n1_10 } else { n1_10.powf(n2_10) };
                                (*io_31).set_tag_variant(TagVariant::NumericNumber);
                            }
                            continue;
                        },
                        OPCODE_DIV => {
                            let ra_37: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let v1_15: *mut TValue = &mut (*base.offset((i >> POSITION_B & !(!(0u32) << 8) << 0) as isize));
                            let v2_14: *mut TValue = &mut (*base.offset((i >> POSITION_C & !(!(0u32) << 8) << 0) as isize));
                            let mut n1_11: f64 = 0.0;
                            let mut n2_11: f64 = 0.0;
                            if (if (*v1_15).get_tag_variant() == TagVariant::NumericNumber {
                                n1_11 = (*v1_15).value.value_number;
                                1
                            } else {
                                if (*v1_15).get_tag_variant() == TagVariant::NumericInteger {
                                    n1_11 = (*v1_15).value.value_integer as f64;
                                    1
                                } else {
                                    0
                                }
                            }) != 0
                                && (if (*v2_14).get_tag_variant() == TagVariant::NumericNumber {
                                    n2_11 = (*v2_14).value.value_number;
                                    1
                                } else {
                                    if (*v2_14).get_tag_variant() == TagVariant::NumericInteger {
                                        n2_11 = (*v2_14).value.value_integer as f64;
                                        1
                                    } else {
                                        0
                                    }
                                }) != 0
                            {
                                program_counter = program_counter.offset(1);
                                let io_32: *mut TValue = &mut (*ra_37);
                                (*io_32).value.value_number = n1_11 / n2_11;
                                (*io_32).set_tag_variant(TagVariant::NumericNumber);
                            }
                            continue;
                        },
                        OPCODE_IDIV => {
                            (*callinfo).call_info_u.l.saved_program_counter = program_counter;
                            (*interpreter).top.stkidrel_pointer = (*callinfo).call_info_top.stkidrel_pointer;
                            let v1_16: *mut TValue = &mut (*base.offset((i >> POSITION_B & !(!(0u32) << 8) << 0) as isize));
                            let v2_15: *mut TValue = &mut (*base.offset((i >> POSITION_C & !(!(0u32) << 8) << 0) as isize));
                            let ra_38: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            if (*v1_16).get_tag_variant() == TagVariant::NumericInteger && (*v2_15).get_tag_variant() == TagVariant::NumericInteger {
                                let i1_11: i64 = (*v1_16).value.value_integer;
                                let i2_11: i64 = (*v2_15).value.value_integer;
                                program_counter = program_counter.offset(1);
                                let io_33: *mut TValue = &mut (*ra_38);
                                (*io_33).value.value_integer = luav_idiv(interpreter, i1_11, i2_11);
                                (*io_33).set_tag_variant(TagVariant::NumericInteger);
                            } else {
                                let mut n1_12: f64 = 0.0;
                                let mut n2_12: f64 = 0.0;
                                if (if (*v1_16).get_tag_variant() == TagVariant::NumericNumber {
                                    n1_12 = (*v1_16).value.value_number;
                                    1
                                } else {
                                    if (*v1_16).get_tag_variant() == TagVariant::NumericInteger {
                                        n1_12 = (*v1_16).value.value_integer as f64;
                                        1
                                    } else {
                                        0
                                    }
                                }) != 0
                                    && (if (*v2_15).get_tag_variant() == TagVariant::NumericNumber {
                                        n2_12 = (*v2_15).value.value_number;
                                        1
                                    } else {
                                        if (*v2_15).get_tag_variant() == TagVariant::NumericInteger {
                                            n2_12 = (*v2_15).value.value_integer as f64;
                                            1
                                        } else {
                                            0
                                        }
                                    }) != 0
                                {
                                    program_counter = program_counter.offset(1);
                                    let io_34: *mut TValue = &mut (*ra_38);
                                    (*io_34).value.value_number = (n1_12 / n2_12).floor();
                                    (*io_34).set_tag_variant(TagVariant::NumericNumber);
                                }
                            }
                            continue;
                        },
                        OPCODE_BAND => {
                            let ra_39: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let v1_17: *mut TValue = &mut (*base.offset((i >> POSITION_B & !(!(0u32) << 8) << 0) as isize));
                            let v2_16: *mut TValue = &mut (*base.offset((i >> POSITION_C & !(!(0u32) << 8) << 0) as isize));
                            let mut i1_12: i64 = 0;
                            let mut i2_12: i64 = 0;
                            if (if (((*v1_17).get_tag_variant() == TagVariant::NumericInteger) as i32 != 0) as i64 != 0 {
                                i1_12 = (*v1_17).value.value_integer;
                                1
                            } else {
                                luav_tointegerns(v1_17, &mut i1_12, F2I::Equal)
                            }) != 0
                                && (if (((*v2_16).get_tag_variant() == TagVariant::NumericInteger) as i32 != 0) as i64 != 0 {
                                    i2_12 = (*v2_16).value.value_integer;
                                    1
                                } else {
                                    luav_tointegerns(v2_16, &mut i2_12, F2I::Equal)
                                }) != 0
                            {
                                program_counter = program_counter.offset(1);
                                let io_35: *mut TValue = &mut (*ra_39);
                                (*io_35).value.value_integer = (i1_12 as usize & i2_12 as usize) as i64;
                                (*io_35).set_tag_variant(TagVariant::NumericInteger);
                            }
                            continue;
                        },
                        OPCODE_BOR => {
                            let ra_40: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let v1_18: *mut TValue = &mut (*base.offset((i >> POSITION_B & !(!(0u32) << 8) << 0) as isize));
                            let v2_17: *mut TValue = &mut (*base.offset((i >> POSITION_C & !(!(0u32) << 8) << 0) as isize));
                            let mut i1_13: i64 = 0;
                            let mut i2_13: i64 = 0;
                            if (if (((*v1_18).get_tag_variant() == TagVariant::NumericInteger) as i32 != 0) as i64 != 0 {
                                i1_13 = (*v1_18).value.value_integer;
                                1
                            } else {
                                luav_tointegerns(v1_18, &mut i1_13, F2I::Equal)
                            }) != 0
                                && (if (((*v2_17).get_tag_variant() == TagVariant::NumericInteger) as i32 != 0) as i64 != 0 {
                                    i2_13 = (*v2_17).value.value_integer;
                                    1
                                } else {
                                    luav_tointegerns(v2_17, &mut i2_13, F2I::Equal)
                                }) != 0
                            {
                                program_counter = program_counter.offset(1);
                                let io_36: *mut TValue = &mut (*ra_40);
                                (*io_36).value.value_integer = (i1_13 as usize | i2_13 as usize) as i64;
                                (*io_36).set_tag_variant(TagVariant::NumericInteger);
                            }
                            continue;
                        },
                        OPCODE_BXOR => {
                            let ra_41: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let v1_19: *mut TValue = &mut (*base.offset((i >> POSITION_B & !(!(0u32) << 8) << 0) as isize));
                            let v2_18: *mut TValue = &mut (*base.offset((i >> POSITION_C & !(!(0u32) << 8) << 0) as isize));
                            let mut i1_14: i64 = 0;
                            let mut i2_14: i64 = 0;
                            if (if (((*v1_19).get_tag_variant() == TagVariant::NumericInteger) as i32 != 0) as i64 != 0 {
                                i1_14 = (*v1_19).value.value_integer;
                                1
                            } else {
                                luav_tointegerns(v1_19, &mut i1_14, F2I::Equal)
                            }) != 0
                                && (if (((*v2_18).get_tag_variant() == TagVariant::NumericInteger) as i32 != 0) as i64 != 0 {
                                    i2_14 = (*v2_18).value.value_integer;
                                    1
                                } else {
                                    luav_tointegerns(v2_18, &mut i2_14, F2I::Equal)
                                }) != 0
                            {
                                program_counter = program_counter.offset(1);
                                let io_37: *mut TValue = &mut (*ra_41);
                                (*io_37).value.value_integer = (i1_14 as usize ^ i2_14 as usize) as i64;
                                (*io_37).set_tag_variant(TagVariant::NumericInteger);
                            }
                            continue;
                        },
                        OPCODE_SHR => {
                            let ra_42: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let v1_20: *mut TValue = &mut (*base.offset((i >> POSITION_B & !(!(0u32) << 8) << 0) as isize));
                            let v2_19: *mut TValue = &mut (*base.offset((i >> POSITION_C & !(!(0u32) << 8) << 0) as isize));
                            let mut i1_15: i64 = 0;
                            let mut i2_15: i64 = 0;
                            if (if (((*v1_20).get_tag_variant() == TagVariant::NumericInteger) as i32 != 0) as i64 != 0 {
                                i1_15 = (*v1_20).value.value_integer;
                                1
                            } else {
                                luav_tointegerns(v1_20, &mut i1_15, F2I::Equal)
                            }) != 0
                                && (if (((*v2_19).get_tag_variant() == TagVariant::NumericInteger) as i32 != 0) as i64 != 0 {
                                    i2_15 = (*v2_19).value.value_integer;
                                    1
                                } else {
                                    luav_tointegerns(v2_19, &mut i2_15, F2I::Equal)
                                }) != 0
                            {
                                program_counter = program_counter.offset(1);
                                let io_38: *mut TValue = &mut (*ra_42);
                                (*io_38).value.value_integer = luav_shiftl(i1_15, (0usize).wrapping_sub(i2_15 as usize) as i64);
                                (*io_38).set_tag_variant(TagVariant::NumericInteger);
                            }
                            continue;
                        },
                        OPCODE_SHL => {
                            let ra_43: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let v1_21: *mut TValue = &mut (*base.offset((i >> POSITION_B & !(!(0u32) << 8) << 0) as isize));
                            let v2_20: *mut TValue = &mut (*base.offset((i >> POSITION_C & !(!(0u32) << 8) << 0) as isize));
                            let mut i1_16: i64 = 0;
                            let mut i2_16: i64 = 0;
                            if (if (((*v1_21).get_tag_variant() == TagVariant::NumericInteger) as i32 != 0) as i64 != 0 {
                                i1_16 = (*v1_21).value.value_integer;
                                1
                            } else {
                                luav_tointegerns(v1_21, &mut i1_16, F2I::Equal)
                            }) != 0
                                && (if (((*v2_20).get_tag_variant() == TagVariant::NumericInteger) as i32 != 0) as i64 != 0 {
                                    i2_16 = (*v2_20).value.value_integer;
                                    1
                                } else {
                                    luav_tointegerns(v2_20, &mut i2_16, F2I::Equal)
                                }) != 0
                            {
                                program_counter = program_counter.offset(1);
                                let io_39: *mut TValue = &mut (*ra_43);
                                (*io_39).value.value_integer = luav_shiftl(i1_16, i2_16);
                                (*io_39).set_tag_variant(TagVariant::NumericInteger);
                            }
                            continue;
                        },
                        46 => {
                            let ra_44: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let pi: u32 = *program_counter.offset(-(2 as isize));
                            let rb_10: *mut TValue = &mut (*base.offset((i >> POSITION_B & !(!(0u32) << 8) << 0) as isize));
                            let tm: u32 = (i >> POSITION_C & !(!(0u32) << 8) << 0) as u32;
                            let result: *mut TValue = base.offset((pi >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            (*callinfo).call_info_u.l.saved_program_counter = program_counter;
                            (*interpreter).top.stkidrel_pointer = (*callinfo).call_info_top.stkidrel_pointer;
                            luat_trybintm(interpreter, &mut (*ra_44), rb_10, result, tm);
                            trap = (*callinfo).call_info_u.l.trap;
                            continue;
                        },
                        47 => {
                            let ra_45: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let pi_0: u32 = *program_counter.offset(-(2 as isize));
                            let imm_0: i32 = (i >> POSITION_B & !(!(0u32) << 8) << 0) as i32 - ((1 << 8) - 1 >> 1);
                            let tm_0: u32 = (i >> POSITION_C & !(!(0u32) << 8) << 0) as u32;
                            let flip: i32 = (i >> POSITION_K & !(!(0u32) << 1) << 0) as i32;
                            let result_0: *mut TValue = base.offset((pi_0 >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            (*callinfo).call_info_u.l.saved_program_counter = program_counter;
                            (*interpreter).top.stkidrel_pointer = (*callinfo).call_info_top.stkidrel_pointer;
                            luat_trybinitm(interpreter, &mut (*ra_45), imm_0 as i64, flip, result_0, tm_0);
                            trap = (*callinfo).call_info_u.l.trap;
                            continue;
                        },
                        48 => {
                            let ra_46: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let pi_1: u32 = *program_counter.offset(-(2 as isize));
                            let imm_1: *mut TValue = k.offset((i >> POSITION_B & !(!(0u32) << 8) << 0) as isize);
                            let tm_1: u32 = (i >> POSITION_C & !(!(0u32) << 8) << 0) as u32;
                            let flip_0: i32 = (i >> POSITION_K & !(!(0u32) << 1) << 0) as i32;
                            let result_1: *mut TValue = base.offset((pi_1 >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            (*callinfo).call_info_u.l.saved_program_counter = program_counter;
                            (*interpreter).top.stkidrel_pointer = (*callinfo).call_info_top.stkidrel_pointer;
                            luat_trybinassoctm(interpreter, &mut (*ra_46), imm_1, flip_0, result_1, tm_1);
                            trap = (*callinfo).call_info_u.l.trap;
                            continue;
                        },
                        49 => {
                            let ra_47: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let rb_11: *mut TValue = &mut (*base.offset((i >> POSITION_B & !(!(0u32) << 8) << 0) as isize));
                            let mut nb_0: f64 = 0.0;
                            if (*rb_11).get_tag_variant() == TagVariant::NumericInteger {
                                let ib_1: i64 = (*rb_11).value.value_integer;
                                let io_40: *mut TValue = &mut (*ra_47);
                                (*io_40).value.value_integer = (0usize).wrapping_sub(ib_1 as usize) as i64;
                                (*io_40).set_tag_variant(TagVariant::NumericInteger);
                            } else if if (*rb_11).get_tag_variant() == TagVariant::NumericNumber {
                                nb_0 = (*rb_11).value.value_number;
                                1
                            } else if (*rb_11).get_tag_variant() == TagVariant::NumericInteger {
                                nb_0 = (*rb_11).value.value_integer as f64;
                                1
                            } else {
                                0
                            } != 0
                            {
                                let io_41: *mut TValue = &mut (*ra_47);
                                (*io_41).value.value_number = -nb_0;
                                (*io_41).set_tag_variant(TagVariant::NumericNumber);
                            } else {
                                (*callinfo).call_info_u.l.saved_program_counter = program_counter;
                                (*interpreter).top.stkidrel_pointer = (*callinfo).call_info_top.stkidrel_pointer;
                                luat_trybintm(interpreter, rb_11, rb_11, ra_47, TM_UNM);
                                trap = (*callinfo).call_info_u.l.trap;
                            }
                            continue;
                        },
                        50 => {
                            let ra_48: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let rb_12: *mut TValue = &mut (*base.offset((i >> POSITION_B & !(!(0u32) << 8) << 0) as isize));
                            let mut ib_2: i64 = 0;
                            if if (((*rb_12).get_tag_variant() == TagVariant::NumericInteger) as i32 != 0) as i64 != 0 {
                                ib_2 = (*rb_12).value.value_integer;
                                1
                            } else {
                                luav_tointegerns(rb_12, &mut ib_2, F2I::Equal)
                            } != 0
                            {
                                let io_42: *mut TValue = &mut (*ra_48);
                                (*io_42).value.value_integer = (!(0usize) ^ ib_2 as usize) as i64;
                                (*io_42).set_tag_variant(TagVariant::NumericInteger);
                            } else {
                                (*callinfo).call_info_u.l.saved_program_counter = program_counter;
                                (*interpreter).top.stkidrel_pointer = (*callinfo).call_info_top.stkidrel_pointer;
                                luat_trybintm(interpreter, rb_12, rb_12, ra_48, TM_BNOT);
                                trap = (*callinfo).call_info_u.l.trap;
                            }
                            continue;
                        },
                        51 => {
                            let ra_49: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let rb_13: *mut TValue = &mut (*base.offset((i >> POSITION_B & !(!(0u32) << 8) << 0) as isize));
                            if (*rb_13).get_tag_variant() == TagVariant::BooleanFalse || (*rb_13).is_tagtype_nil() {
                                (*ra_49).set_tag_variant(TagVariant::BooleanTrue);
                            } else {
                                (*ra_49).set_tag_variant(TagVariant::BooleanFalse);
                            }
                            continue;
                        },
                        OPCODE_LEN => {
                            let ra_50: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            (*callinfo).call_info_u.l.saved_program_counter = program_counter;
                            (*interpreter).top.stkidrel_pointer = (*callinfo).call_info_top.stkidrel_pointer;
                            luav_objlen(interpreter, ra_50, &mut (*base.offset((i >> POSITION_B & !(!(0u32) << 8) << 0) as isize)));
                            trap = (*callinfo).call_info_u.l.trap;
                            continue;
                        },
                        53 => {
                            let ra_51: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let n_1: i32 = (i >> POSITION_B & !(!(0u32) << 8) << 0) as i32;
                            (*interpreter).top.stkidrel_pointer = ra_51.offset(n_1 as isize);
                            (*callinfo).call_info_u.l.saved_program_counter = program_counter;
                            concatenate(interpreter, n_1);
                            trap = (*callinfo).call_info_u.l.trap;
                            if (*(*interpreter).global).global_gcdebt > 0 {
                                (*callinfo).call_info_u.l.saved_program_counter = program_counter;
                                (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer;
                                (*interpreter).luac_step();
                                trap = (*callinfo).call_info_u.l.trap;
                            }
                            continue;
                        },
                        54 => {
                            let ra_52: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            (*callinfo).call_info_u.l.saved_program_counter = program_counter;
                            (*interpreter).top.stkidrel_pointer = (*callinfo).call_info_top.stkidrel_pointer;
                            luaf_close(interpreter, ra_52, Status::OK, 1);
                            trap = (*callinfo).call_info_u.l.trap;
                            continue;
                        },
                        55 => {
                            let ra_53: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            (*callinfo).call_info_u.l.saved_program_counter = program_counter;
                            (*interpreter).top.stkidrel_pointer = (*callinfo).call_info_top.stkidrel_pointer;
                            luaf_newtbcupval(interpreter, ra_53);
                            continue;
                        },
                        56 => {
                            program_counter = program_counter.offset(((i >> POSITION_A & !(!(0u32) << 8 + 8 + 1 + 8) << 0) as i32 - ((1 << 8 + 8 + 1 + 8) - 1 >> 1) + 0) as isize);
                            trap = (*callinfo).call_info_u.l.trap;
                            continue;
                        },
                        57 => {
                            let ra_54: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let cond_0: i32;
                            let rb_14: *mut TValue = &mut (*base.offset((i >> POSITION_B & !(!(0u32) << 8) << 0) as isize));
                            (*callinfo).call_info_u.l.saved_program_counter = program_counter;
                            (*interpreter).top.stkidrel_pointer = (*callinfo).call_info_top.stkidrel_pointer;
                            cond_0 = if luav_equalobj(interpreter, &mut (*ra_54), rb_14) { 1 } else { 0 };
                            trap = (*callinfo).call_info_u.l.trap;
                            if cond_0 != (i >> POSITION_K & !(!(0u32) << 1) << 0) as i32 {
                                program_counter = program_counter.offset(1);
                            } else {
                                let ni: u32 = *program_counter;
                                program_counter = program_counter.offset(((ni >> POSITION_A & !(!(0u32) << 8 + 8 + 1 + 8) << 0) as i32 - ((1 << 8 + 8 + 1 + 8) - 1 >> 1) + 1) as isize);
                                trap = (*callinfo).call_info_u.l.trap;
                            }
                            continue;
                        },
                        58 => {
                            let ra_55: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let cond_1: i32;
                            let rb_15: *mut TValue = &mut (*base.offset((i >> POSITION_B & !(!(0u32) << 8) << 0) as isize));
                            if (*ra_55).get_tag_variant() == TagVariant::NumericInteger && (*rb_15).get_tag_variant() == TagVariant::NumericInteger {
                                let ia: i64 = (*ra_55).value.value_integer;
                                let ib_3: i64 = (*rb_15).value.value_integer;
                                cond_1 = (ia < ib_3) as i32;
                            } else if (*ra_55).is_tagtype_numeric() && (*rb_15).is_tagtype_numeric() {
                                cond_1 = if ltnum(&mut (*ra_55), rb_15) { 1 } else { 0 };
                            } else {
                                (*callinfo).call_info_u.l.saved_program_counter = program_counter;
                                (*interpreter).top.stkidrel_pointer = (*callinfo).call_info_top.stkidrel_pointer;
                                cond_1 = lessthanothers(interpreter, &mut (*ra_55), rb_15);
                                trap = (*callinfo).call_info_u.l.trap;
                            }
                            if cond_1 != (i >> POSITION_K & !(!(0u32) << 1) << 0) as i32 {
                                program_counter = program_counter.offset(1);
                            } else {
                                let ni_0: u32 = *program_counter;
                                program_counter = program_counter.offset(((ni_0 >> POSITION_A & !(!(0u32) << 8 + 8 + 1 + 8) << 0) as i32 - ((1 << 8 + 8 + 1 + 8) - 1 >> 1) + 1) as isize);
                                trap = (*callinfo).call_info_u.l.trap;
                            }
                            continue;
                        },
                        OPCODE_LE => {
                            let ra_56: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let cond_2: i32;
                            let rb_16: *mut TValue = &mut (*base.offset((i >> POSITION_B & !(!(0u32) << 8) << 0) as isize));
                            if (*ra_56).get_tag_variant() == TagVariant::NumericInteger && (*rb_16).get_tag_variant() == TagVariant::NumericInteger {
                                let ia_0: i64 = (*ra_56).value.value_integer;
                                let ib_4: i64 = (*rb_16).value.value_integer;
                                cond_2 = (ia_0 <= ib_4) as i32;
                            } else if (*ra_56).is_tagtype_numeric() && (*rb_16).is_tagtype_numeric() {
                                cond_2 = if lenum(&mut (*ra_56), rb_16) { 1 } else { 0 };
                            } else {
                                (*callinfo).call_info_u.l.saved_program_counter = program_counter;
                                (*interpreter).top.stkidrel_pointer = (*callinfo).call_info_top.stkidrel_pointer;
                                cond_2 = if lessequalothers(interpreter, &mut (*ra_56), rb_16) { 1 } else { 0 };
                                trap = (*callinfo).call_info_u.l.trap;
                            }
                            if cond_2 != (i >> POSITION_K & !(!(0u32) << 1) << 0) as i32 {
                                program_counter = program_counter.offset(1);
                            } else {
                                let ni_1: u32 = *program_counter;
                                program_counter = program_counter.offset(((ni_1 >> POSITION_A & !(!(0u32) << 8 + 8 + 1 + 8) << 0) as i32 - ((1 << 8 + 8 + 1 + 8) - 1 >> 1) + 1) as isize);
                                trap = (*callinfo).call_info_u.l.trap;
                            }
                            continue;
                        },
                        60 => {
                            let ra_57: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let rb_17: *mut TValue = k.offset((i >> POSITION_B & !(!(0u32) << 8) << 0) as isize);
                            let cond_3: i32 = if luav_equalobj(null_mut(), &mut (*ra_57), rb_17) { 1 } else { 0 };
                            if cond_3 != (i >> POSITION_K & !(!(0u32) << 1) << 0) as i32 {
                                program_counter = program_counter.offset(1);
                            } else {
                                let ni_2: u32 = *program_counter;
                                program_counter = program_counter.offset(((ni_2 >> POSITION_A & !(!(0u32) << 8 + 8 + 1 + 8) << 0) as i32 - ((1 << 8 + 8 + 1 + 8) - 1 >> 1) + 1) as isize);
                                trap = (*callinfo).call_info_u.l.trap;
                            }
                            continue;
                        },
                        61 => {
                            let ra_58: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let cond_4: i32;
                            let im: i32 = (i >> POSITION_B & !(!(0u32) << 8) << 0) as i32 - ((1 << 8) - 1 >> 1);
                            if (*ra_58).get_tag_variant() == TagVariant::NumericInteger {
                                cond_4 = ((*ra_58).value.value_integer == im as i64) as i32;
                            } else if (*ra_58).get_tag_variant() == TagVariant::NumericNumber {
                                cond_4 = ((*ra_58).value.value_number == im as f64) as i32;
                            } else {
                                cond_4 = 0;
                            }
                            if cond_4 != (i >> POSITION_K & !(!(0u32) << 1) << 0) as i32 {
                                program_counter = program_counter.offset(1);
                            } else {
                                let ni_3: u32 = *program_counter;
                                program_counter = program_counter.offset(((ni_3 >> POSITION_A & !(!(0u32) << 8 + 8 + 1 + 8) << 0) as i32 - ((1 << 8 + 8 + 1 + 8) - 1 >> 1) + 1) as isize);
                                trap = (*callinfo).call_info_u.l.trap;
                            }
                            continue;
                        },
                        62 => {
                            let ra_59: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let cond_5: i32;
                            let im_0: i32 = (i >> POSITION_B & !(!(0u32) << 8) << 0) as i32 - ((1 << 8) - 1 >> 1);
                            if (*ra_59).get_tag_variant() == TagVariant::NumericInteger {
                                cond_5 = ((*ra_59).value.value_integer < im_0 as i64) as i32;
                            } else if (*ra_59).get_tag_variant() == TagVariant::NumericNumber {
                                let fa: f64 = (*ra_59).value.value_number;
                                let fim: f64 = im_0 as f64;
                                cond_5 = (fa < fim) as i32;
                            } else {
                                let isf: bool = (i >> POSITION_C & !(!(0u32) << 8) << 0) != 0;
                                (*callinfo).call_info_u.l.saved_program_counter = program_counter;
                                (*interpreter).top.stkidrel_pointer = (*callinfo).call_info_top.stkidrel_pointer;
                                cond_5 = luat_callorderitm(interpreter, &mut (*ra_59), im_0, 0, isf, TM_LT);
                                trap = (*callinfo).call_info_u.l.trap;
                            }
                            if cond_5 != (i >> POSITION_K & !(!(0u32) << 1) << 0) as i32 {
                                program_counter = program_counter.offset(1);
                            } else {
                                let ni_4: u32 = *program_counter;
                                program_counter = program_counter.offset(((ni_4 >> POSITION_A & !(!(0u32) << 8 + 8 + 1 + 8) << 0) as i32 - ((1 << 8 + 8 + 1 + 8) - 1 >> 1) + 1) as isize);
                                trap = (*callinfo).call_info_u.l.trap;
                            }
                            continue;
                        },
                        OPCODE_LEI => {
                            let ra_60: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let cond_6: i32;
                            let im_1: i32 = (i >> POSITION_B & !(!(0u32) << 8) << 0) as i32 - ((1 << 8) - 1 >> 1);
                            if (*ra_60).get_tag_variant() == TagVariant::NumericInteger {
                                cond_6 = ((*ra_60).value.value_integer <= im_1 as i64) as i32;
                            } else if (*ra_60).get_tag_variant() == TagVariant::NumericNumber {
                                let fa_0: f64 = (*ra_60).value.value_number;
                                let fim_0: f64 = im_1 as f64;
                                cond_6 = (fa_0 <= fim_0) as i32;
                            } else {
                                let isf_0: bool = (i >> POSITION_C & !(!(0u32) << 8) << 0) != 0;
                                (*callinfo).call_info_u.l.saved_program_counter = program_counter;
                                (*interpreter).top.stkidrel_pointer = (*callinfo).call_info_top.stkidrel_pointer;
                                cond_6 = luat_callorderitm(interpreter, &mut (*ra_60), im_1, 0, isf_0, TM_LE);
                                trap = (*callinfo).call_info_u.l.trap;
                            }
                            if cond_6 != (i >> POSITION_K & !(!(0u32) << 1) << 0) as i32 {
                                program_counter = program_counter.offset(1);
                            } else {
                                let ni_5: u32 = *program_counter;
                                program_counter = program_counter.offset(((ni_5 >> POSITION_A & !(!(0u32) << 8 + 8 + 1 + 8) << 0) as i32 - ((1 << 8 + 8 + 1 + 8) - 1 >> 1) + 1) as isize);
                                trap = (*callinfo).call_info_u.l.trap;
                            }
                            continue;
                        },
                        64 => {
                            let ra_61: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let cond_7: i32;
                            let im_2: i32 = (i >> POSITION_B & !(!(0u32) << 8) << 0) as i32 - ((1 << 8) - 1 >> 1);
                            if (*ra_61).get_tag_variant() == TagVariant::NumericInteger {
                                cond_7 = ((*ra_61).value.value_integer > im_2 as i64) as i32;
                            } else if (*ra_61).get_tag_variant() == TagVariant::NumericNumber {
                                let fa_1: f64 = (*ra_61).value.value_number;
                                let fim_1: f64 = im_2 as f64;
                                cond_7 = (fa_1 > fim_1) as i32;
                            } else {
                                let isf_1: bool = (i >> POSITION_C & !(!(0u32) << 8) << 0) != 0;
                                (*callinfo).call_info_u.l.saved_program_counter = program_counter;
                                (*interpreter).top.stkidrel_pointer = (*callinfo).call_info_top.stkidrel_pointer;
                                cond_7 = luat_callorderitm(interpreter, &mut (*ra_61), im_2, 1, isf_1, TM_LT);
                                trap = (*callinfo).call_info_u.l.trap;
                            }
                            if cond_7 != (i >> POSITION_K & !(!(0u32) << 1) << 0) as i32 {
                                program_counter = program_counter.offset(1);
                            } else {
                                let ni_6: u32 = *program_counter;
                                program_counter = program_counter.offset(((ni_6 >> POSITION_A & !(!(0u32) << 8 + 8 + 1 + 8) << 0) as i32 - ((1 << 8 + 8 + 1 + 8) - 1 >> 1) + 1) as isize);
                                trap = (*callinfo).call_info_u.l.trap;
                            }
                            continue;
                        },
                        OPCODE_GEI => {
                            let ra_62: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let cond_8: i32;
                            let im_3: i32 = (i >> POSITION_B & !(!(0u32) << 8) << 0) as i32 - ((1 << 8) - 1 >> 1);
                            if (*ra_62).get_tag_variant() == TagVariant::NumericInteger {
                                cond_8 = ((*ra_62).value.value_integer >= im_3 as i64) as i32;
                            } else if (*ra_62).get_tag_variant() == TagVariant::NumericNumber {
                                let fa_2: f64 = (*ra_62).value.value_number;
                                let fim_2: f64 = im_3 as f64;
                                cond_8 = (fa_2 >= fim_2) as i32;
                            } else {
                                let isf_2: bool = (i >> POSITION_C & !(!(0u32) << 8) << 0) != 0;
                                (*callinfo).call_info_u.l.saved_program_counter = program_counter;
                                (*interpreter).top.stkidrel_pointer = (*callinfo).call_info_top.stkidrel_pointer;
                                cond_8 = luat_callorderitm(interpreter, &mut (*ra_62), im_3, 1, isf_2, TM_LE);
                                trap = (*callinfo).call_info_u.l.trap;
                            }
                            if cond_8 != (i >> POSITION_K & !(!(0u32) << 1) << 0) as i32 {
                                program_counter = program_counter.offset(1);
                            } else {
                                let ni_7: u32 = *program_counter;
                                program_counter = program_counter.offset(((ni_7 >> POSITION_A & !(!(0u32) << 8 + 8 + 1 + 8) << 0) as i32 - ((1 << 8 + 8 + 1 + 8) - 1 >> 1) + 1) as isize);
                                trap = (*callinfo).call_info_u.l.trap;
                            }
                            continue;
                        },
                        66 => {
                            let ra_63: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let cond_9: i32 = !((*ra_63).get_tag_variant() == TagVariant::BooleanFalse || (*ra_63).is_tagtype_nil()) as i32;
                            if cond_9 != (i >> POSITION_K & !(!(0u32) << 1) << 0) as i32 {
                                program_counter = program_counter.offset(1);
                            } else {
                                let ni_8: u32 = *program_counter;
                                program_counter = program_counter.offset(((ni_8 >> POSITION_A & !(!(0u32) << 8 + 8 + 1 + 8) << 0) as i32 - ((1 << 8 + 8 + 1 + 8) - 1 >> 1) + 1) as isize);
                                trap = (*callinfo).call_info_u.l.trap;
                            }
                            continue;
                        },
                        67 => {
                            let ra_64: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let rb_18: *mut TValue = &mut (*base.offset((i >> POSITION_B & !(!(0u32) << 8) << 0) as isize));
                            if ((*rb_18).get_tag_variant() == TagVariant::BooleanFalse || (*rb_18).is_tagtype_nil()) as i32 == (i >> POSITION_K & !(!(0u32) << 1) << 0) as i32 {
                                program_counter = program_counter.offset(1);
                            } else {
                                let io1_14: *mut TValue = &mut (*ra_64);
                                let io2_14: *const TValue = rb_18;
                                (*io1_14).copy_from(&(*io2_14));
                                let ni_9: u32 = *program_counter;
                                program_counter = program_counter.offset(((ni_9 >> POSITION_A & !(!(0u32) << 8 + 8 + 1 + 8) << 0) as i32 - ((1 << 8 + 8 + 1 + 8) - 1 >> 1) + 1) as isize);
                                trap = (*callinfo).call_info_u.l.trap;
                            }
                            continue;
                        },
                        68 => {
                            ra_65 = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            b_4 = (i >> POSITION_B & !(!(0u32) << 8) << 0) as i32;
                            count_results = (i >> POSITION_C & !(!(0u32) << 8) << 0) as i32 - 1;
                            if b_4 != 0 {
                                (*interpreter).top.stkidrel_pointer = ra_65.offset(b_4 as isize);
                            }
                            (*callinfo).call_info_u.l.saved_program_counter = program_counter;
                            new_call_info = luad_precall(interpreter, ra_65, count_results);
                            if !new_call_info.is_null() {
                                break '_returning;
                            }
                            trap = (*callinfo).call_info_u.l.trap;
                            continue;
                        },
                        69 => {
                            let ra_66: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let mut b_5: i32 = (i >> POSITION_B & !(!(0u32) << 8) << 0) as i32;
                            let n_2: i32;
                            let nparams1: i32 = (i >> POSITION_C & !(!(0u32) << 8) << 0) as i32;
                            let delta: i32 = if nparams1 != 0 { (*callinfo).call_info_u.l.count_extra_arguments + nparams1 } else { 0 };
                            if b_5 != 0 {
                                (*interpreter).top.stkidrel_pointer = ra_66.offset(b_5 as isize);
                            } else {
                                b_5 = ((*interpreter).top.stkidrel_pointer).offset_from(ra_66) as i32;
                            }
                            (*callinfo).call_info_u.l.saved_program_counter = program_counter;
                            if (i & (1 as u32) << POSITION_K) as i32 != 0 {
                                luaf_closeupval(interpreter, base);
                            }
                            n_2 = luad_pretailcall(interpreter, callinfo, ra_66, b_5, delta);
                            if n_2 < 0 {
                                continue '_startfunc;
                            }
                            (*callinfo).call_info_function.stkidrel_pointer = ((*callinfo).call_info_function.stkidrel_pointer).offset(-(delta as isize));
                            luad_poscall(interpreter, callinfo, n_2);
                            trap = (*callinfo).call_info_u.l.trap;
                            break;
                        },
                        70 => {
                            let mut ra_67: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let mut n_3: i32 = (i >> POSITION_B & !(!(0u32) << 8) << 0) as i32 - 1;
                            let nparams1_0: i32 = (i >> POSITION_C & !(!(0u32) << 8) << 0) as i32;
                            if n_3 < 0 {
                                n_3 = ((*interpreter).top.stkidrel_pointer).offset_from(ra_67) as i32;
                            }
                            (*callinfo).call_info_u.l.saved_program_counter = program_counter;
                            if (i & (1 as u32) << POSITION_K) as i32 != 0 {
                                (*callinfo).call_info_u2.nres = n_3;
                                if (*interpreter).top.stkidrel_pointer < (*callinfo).call_info_top.stkidrel_pointer {
                                    (*interpreter).top.stkidrel_pointer = (*callinfo).call_info_top.stkidrel_pointer;
                                }
                                luaf_close(interpreter, base, Status::Closing, 1);
                                trap = (*callinfo).call_info_u.l.trap;
                                if (trap != 0) as i64 != 0 {
                                    base = ((*callinfo).call_info_function.stkidrel_pointer).offset(1 as isize);
                                    ra_67 = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                                }
                            }
                            if nparams1_0 != 0 {
                                (*callinfo).call_info_function.stkidrel_pointer = ((*callinfo).call_info_function.stkidrel_pointer).offset(-(((*callinfo).call_info_u.l.count_extra_arguments + nparams1_0) as isize));
                            }
                            (*interpreter).top.stkidrel_pointer = ra_67.offset(n_3 as isize);
                            luad_poscall(interpreter, callinfo, n_3);
                            trap = (*callinfo).call_info_u.l.trap;
                            break;
                        },
                        71 => {
                            if ((*interpreter).hook_mask != 0) as i64 != 0 {
                                let ra_68: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                                (*interpreter).top.stkidrel_pointer = ra_68;
                                (*callinfo).call_info_u.l.saved_program_counter = program_counter;
                                luad_poscall(interpreter, callinfo, 0);
                                trap = 1;
                            } else {
                                let mut nres: i32;
                                (*interpreter).callinfo = (*callinfo).call_info_previous;
                                (*interpreter).top.stkidrel_pointer = base.offset(-(1 as isize));
                                nres = (*callinfo).call_info_count_results as i32;
                                while ((nres > 0) as i32 != 0) as i64 != 0 {
                                    let fresh141 = (*interpreter).top.stkidrel_pointer;
                                    (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
                                    (*fresh141).set_tag_variant(TagVariant::NilNil);
                                    nres -= 1;
                                }
                            }
                            break;
                        },
                        72 => {
                            if ((*interpreter).hook_mask != 0) as i64 != 0 {
                                let ra_69: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                                (*interpreter).top.stkidrel_pointer = ra_69.offset(1 as isize);
                                (*callinfo).call_info_u.l.saved_program_counter = program_counter;
                                luad_poscall(interpreter, callinfo, 1);
                                trap = 1;
                            } else {
                                let mut nres_0: i32 = (*callinfo).call_info_count_results as i32;
                                (*interpreter).callinfo = (*callinfo).call_info_previous;
                                if nres_0 == 0 {
                                    (*interpreter).top.stkidrel_pointer = base.offset(-(1 as isize));
                                } else {
                                    let ra_70: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                                    let io1_15: *mut TValue = &mut (*base.offset(-(1 as isize)));
                                    let io2_15: *const TValue = &mut (*ra_70);
                                    (*io1_15).copy_from(&(*io2_15));
                                    (*interpreter).top.stkidrel_pointer = base;
                                    while ((nres_0 > 1) as i32 != 0) as i64 != 0 {
                                        let fresh142 = (*interpreter).top.stkidrel_pointer;
                                        (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
                                        (*fresh142).set_tag_variant(TagVariant::NilNil);
                                        nres_0 -= 1;
                                    }
                                }
                            }
                            break;
                        },
                        OPCODE_FORLOOP => {
                            let ra_71: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            if (*ra_71.offset(2 as isize)).get_tag_variant() == TagVariant::NumericInteger {
                                let count: usize = (*ra_71.offset(1 as isize)).value.value_integer as usize;
                                if count > 0 {
                                    let step: i64 = (*ra_71.offset(2 as isize)).value.value_integer;
                                    let mut index: i64 = (*ra_71).value.value_integer;
                                    let io_43: *mut TValue = &mut (*ra_71.offset(1 as isize));
                                    (*io_43).value.value_integer = count.wrapping_sub(1 as usize) as i64;
                                    index = (index as usize).wrapping_add(step as usize) as i64;
                                    let io_44: *mut TValue = &mut (*ra_71);
                                    (*io_44).value.value_integer = index;
                                    let io_45: *mut TValue = &mut (*ra_71.offset(3 as isize));
                                    (*io_45).value.value_integer = index;
                                    (*io_45).set_tag_variant(TagVariant::NumericInteger);
                                    program_counter = program_counter.offset(-((i >> POSITION_K & !(!(0u32) << 8 + 8 + 1) << 0) as i32 as isize));
                                }
                            } else if floatforloop(ra_71) != 0 {
                                program_counter = program_counter.offset(-((i >> POSITION_K & !(!(0u32) << 8 + 8 + 1) << 0) as i32 as isize));
                            }
                            trap = (*callinfo).call_info_u.l.trap;
                            continue;
                        },
                        74 => {
                            let ra_72: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            (*callinfo).call_info_u.l.saved_program_counter = program_counter;
                            (*interpreter).top.stkidrel_pointer = (*callinfo).call_info_top.stkidrel_pointer;
                            if forprep(interpreter, ra_72) != 0 {
                                program_counter = program_counter.offset(((i >> POSITION_K & !(!(0u32) << 8 + 8 + 1) << 0) as i32 + 1) as isize);
                            }
                            continue;
                        },
                        75 => {
                            let ra_73: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            (*callinfo).call_info_u.l.saved_program_counter = program_counter;
                            (*interpreter).top.stkidrel_pointer = (*callinfo).call_info_top.stkidrel_pointer;
                            luaf_newtbcupval(interpreter, ra_73.offset(3 as isize));
                            program_counter = program_counter.offset((i >> POSITION_K & !(!(0u32) << 8 + 8 + 1) << 0) as isize);
                            let fresh143 = program_counter;
                            program_counter = program_counter.offset(1);
                            i = *fresh143;
                            current_block = 13973394567113199817;
                        },
                        76 => {
                            current_block = 13973394567113199817;
                        },
                        77 => {
                            current_block = 15611964311717037170;
                        },
                        78 => {
                            let ra_76: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let mut n_4: i32 = (i >> POSITION_B & !(!(0u32) << 8) << 0) as i32;
                            let mut last: u32 = (i >> POSITION_C & !(!(0u32) << 8) << 0) as u32;
                            let h: *mut Table = &mut (*((*ra_76).value.value_object as *mut Table));
                            if n_4 == 0 {
                                n_4 = ((*interpreter).top.stkidrel_pointer).offset_from(ra_76) as i32 - 1;
                            } else {
                                (*interpreter).top.stkidrel_pointer = (*callinfo).call_info_top.stkidrel_pointer;
                            }
                            last = last.wrapping_add(n_4 as u32);
                            if (i & (1 as u32) << POSITION_K) as i32 != 0 {
                                last = last.wrapping_add(((*program_counter >> POSITION_A & !(!(0u32) << 8 + 8 + 1 + 8) << 0) as i32 * ((1 << 8) - 1 + 1)) as u32);
                                program_counter = program_counter.offset(1);
                            }
                            if last > luah_realasize(h) {
                                luah_resizearray(interpreter, h, last as usize);
                            }
                            while n_4 > 0 {
                                let value: *mut TValue = &mut (*ra_76.offset(n_4 as isize));
                                let io1_17: *mut TValue = &mut *((*h).array).offset(last.wrapping_sub(1 as u32) as isize) as *mut TValue;
                                let io2_17: *const TValue = value;
                                (*io1_17).copy_from(&(*io2_17));
                                last = last.wrapping_sub(1);
                                if (*value).is_collectable() {
                                    if (*(h as *mut ObjectBase)).get_marked() & 1 << 5 != 0 && (*(*value).value.value_object).get_marked() & (1 << 3 | 1 << 4) != 0 {
                                        luac_barrierback_(interpreter, &mut (*(h as *mut ObjectWithGCList)));
                                    } else {
                                    };
                                } else {
                                };
                                n_4 -= 1;
                            }
                            continue;
                        },
                        79 => {
                            let ra_77: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let p: *mut Prototype = *((*(*closure).payload.l_prototype).prototype_prototypes.vectort_pointer).offset((i >> POSITION_K & !(!(0u32) << 8 + 8 + 1) << 0) as isize);
                            (*callinfo).call_info_u.l.saved_program_counter = program_counter;
                            (*interpreter).top.stkidrel_pointer = (*callinfo).call_info_top.stkidrel_pointer;
                            pushclosure(interpreter, p, ((*closure).upvalues).l_upvalues.as_mut_ptr(), base, ra_77);
                            if (*(*interpreter).global).global_gcdebt > 0 {
                                (*callinfo).call_info_u.l.saved_program_counter = program_counter;
                                (*interpreter).top.stkidrel_pointer = ra_77.offset(1 as isize);
                                (*interpreter).luac_step();
                                trap = (*callinfo).call_info_u.l.trap;
                            }
                            continue;
                        },
                        80 => {
                            let ra_78: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            let n_5: i32 = (i >> POSITION_C & !(!(0u32) << 8) << 0) as i32 - 1;
                            (*callinfo).call_info_u.l.saved_program_counter = program_counter;
                            (*interpreter).top.stkidrel_pointer = (*callinfo).call_info_top.stkidrel_pointer;
                            luat_getvarargs(interpreter, callinfo, ra_78, n_5);
                            trap = (*callinfo).call_info_u.l.trap;
                            continue;
                        },
                        81 => {
                            (*callinfo).call_info_u.l.saved_program_counter = program_counter;
                            luat_adjustvarargs(interpreter, (i >> POSITION_A & !(!(0u32) << 8) << 0) as i32, callinfo, (*closure).payload.l_prototype);
                            trap = (*callinfo).call_info_u.l.trap;
                            if (trap != 0) as i64 != 0 {
                                luad_hookcall(interpreter, callinfo);
                                (*interpreter).old_program_counter = 1;
                            }
                            base = ((*callinfo).call_info_function.stkidrel_pointer).offset(1 as isize);
                            continue;
                        },
                        82 | _ => {
                            continue;
                        },
                    }
                    match current_block {
                        13973394567113199817 => {
                            let ra_74: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                            memcpy(ra_74.offset(4 as isize) as *mut libc::c_void, ra_74 as *const libc::c_void, (3 as usize).wrapping_mul(size_of::<TValue>()));
                            (*interpreter).top.stkidrel_pointer = ra_74.offset(4 as isize).offset(3 as isize);
                            (*callinfo).call_info_u.l.saved_program_counter = program_counter;
                            ccall(interpreter, ra_74.offset(4 as isize), (i >> POSITION_C & !(!(0u32) << 8) << 0) as i32, 1);
                            trap = (*callinfo).call_info_u.l.trap;
                            if (trap != 0) as i64 != 0 {
                                base = ((*callinfo).call_info_function.stkidrel_pointer).offset(1 as isize);
                            }
                            let fresh144 = program_counter;
                            program_counter = program_counter.offset(1);
                            i = *fresh144;
                        },
                        _ => {},
                    }
                    let ra_75: *mut TValue = base.offset((i >> POSITION_A & !(!(0u32) << 8) << 0) as isize);
                    if !(*ra_75.offset(4 as isize)).is_tagtype_nil() {
                        let io1_16: *mut TValue = &mut (*ra_75.offset(2 as isize));
                        let io2_16: *const TValue = &mut (*ra_75.offset(4 as isize));
                        (*io1_16).copy_from(&(*io2_16));
                        program_counter = program_counter.offset(-((i >> POSITION_K & !(!(0u32) << 8 + 8 + 1) << 0) as isize));
                    }
                }
                if (*callinfo).call_info_call_status as i32 & 1 << 2 != 0 {
                    break '_startfunc;
                }
                callinfo = (*callinfo).call_info_previous;
            }
            callinfo = new_call_info;
        }
    }
}
pub unsafe fn findfield(interpreter: *mut Interpreter, objidx: i32, level: i32) -> bool {
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
                    lua_pushstring(interpreter, c".".as_ptr());
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
pub unsafe fn pushglobalfuncname(interpreter: *mut Interpreter, debuginfo: *mut DebugInfo) -> bool {
    unsafe {
        let top: i32 = (*interpreter).get_top();
        lua_getinfo(interpreter, c"f".as_ptr(), debuginfo);
        lua_getfield(interpreter, -(1000000 as i32) - 1000 as i32, c"_LOADED".as_ptr());
        lual_checkstack(interpreter, 6, c"not enough stack".as_ptr());
        if findfield(interpreter, top + 1, 2) {
            let name: *const i8 = lua_tolstring(interpreter, -1, null_mut());
            if strncmp(name, c"_G.".as_ptr(), 3) == 0 {
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
pub unsafe fn pushfuncname(interpreter: *mut Interpreter, debuginfo: *mut DebugInfo) {
    unsafe {
        if pushglobalfuncname(interpreter, debuginfo) {
            lua_pushfstring(interpreter, c"function '%s'".as_ptr(), lua_tolstring(interpreter, -1, null_mut()));
            lua_rotate(interpreter, -2, -1);
            lua_settop(interpreter, -2);
        } else if *(*debuginfo).debuginfo_namewhat as i32 != Character::Null as i32 {
            lua_pushfstring(interpreter, c"%s '%s'".as_ptr(), (*debuginfo).debuginfo_namewhat, (*debuginfo).debuginfo_name);
        } else if *(*debuginfo).debuginfo_what as i32 == Character::LowerM as i32 {
            lua_pushstring(interpreter, c"main chunk".as_ptr());
        } else if *(*debuginfo).debuginfo_what as i32 != Character::UpperC as i32 {
            lua_pushfstring(interpreter, c"function <%s:%d>".as_ptr(), ((*debuginfo).debuginfo_shortsrc).as_mut_ptr(), (*debuginfo).debuginfo_linedefined);
        } else {
            lua_pushstring(interpreter, c"?".as_ptr());
        };
    }
}
pub unsafe fn lastlevel(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let mut debuginfo: DebugInfo = DebugInfo::new();
        let mut li: i32 = 1;
        let mut le: i32 = 1;
        while lua_getstack(interpreter, le, &mut debuginfo) != 0 {
            li = le;
            le *= 2;
        }
        while li < le {
            let m: i32 = (li + le) / 2;
            if lua_getstack(interpreter, m, &mut debuginfo) != 0 {
                li = m + 1;
            } else {
                le = m;
            }
        }
        return le - 1;
    }
}
pub unsafe fn lual_traceback(interpreter: *mut Interpreter, other_state: *mut Interpreter, message: *const i8, mut level: i32) {
    unsafe {
        let mut b = Buffer::new();
        let mut debuginfo: DebugInfo = DebugInfo::new();
        let last: i32 = lastlevel(other_state);
        let mut limit2show: i32 = if last - level > 10 as i32 + 11 as i32 { 10 as i32 } else { -1 };
        b.initialize(interpreter);
        if !message.is_null() {
            b.add_string(message);
            (b.loads.get_length() < b.loads.get_size() || !(b.prepare_with_size(1)).is_null()) as i32;
            let fresh145 = b.loads.get_length();
            b.loads.set_length((b.loads.get_length()).wrapping_add(1) as usize);
            *(b.loads.loads_pointer).offset(fresh145 as isize) = Character::LineFeed as i8;
        }
        b.add_string(c"stack traceback:".as_ptr());
        loop {
            let fresh146 = level;
            level = level + 1;
            if !(lua_getstack(other_state, fresh146, &mut debuginfo) != 0) {
                break;
            }
            let fresh147 = limit2show;
            limit2show = limit2show - 1;
            if fresh147 == 0 {
                let n: i32 = last - level - 11 as i32 + 1;
                lua_pushfstring(interpreter, c"\n\t...\t(skipping %d levels)".as_ptr(), n);
                b.add_value();
                level += n;
            } else {
                lua_getinfo(other_state, c"Slnt".as_ptr(), &mut debuginfo);
                if debuginfo.debuginfo_currentline <= 0 {
                    lua_pushfstring(interpreter, c"\n\t%s: in ".as_ptr(), (debuginfo.debuginfo_shortsrc).as_mut_ptr());
                } else {
                    lua_pushfstring(interpreter, c"\n\t%s:%d: in ".as_ptr(), (debuginfo.debuginfo_shortsrc).as_mut_ptr(), debuginfo.debuginfo_currentline);
                }
                b.add_value();
                pushfuncname(interpreter, &mut debuginfo);
                b.add_value();
                if debuginfo.debuginfo_istailcall {
                    b.add_string(c"\n\t(...tail calls...)".as_ptr());
                }
            }
        }
        b.push_result();
    }
}
pub unsafe fn lual_argerror(interpreter: *mut Interpreter, mut arg: i32, extramsg: *const i8) -> i32 {
    unsafe {
        let mut debuginfo: DebugInfo = DebugInfo::new();
        if lua_getstack(interpreter, 0, &mut debuginfo) == 0 {
            return lual_error(interpreter, c"bad argument #%d (%s)".as_ptr(), arg, extramsg);
        }
        lua_getinfo(interpreter, c"n".as_ptr(), &mut debuginfo);
        if strcmp(debuginfo.debuginfo_namewhat, c"method".as_ptr()) == 0 {
            arg -= 1;
            if arg == 0 {
                return lual_error(interpreter, c"calling '%s' on bad self (%s)".as_ptr(), debuginfo.debuginfo_name, extramsg);
            }
        }
        if debuginfo.debuginfo_name.is_null() {
            debuginfo.debuginfo_name = if pushglobalfuncname(interpreter, &mut debuginfo) { lua_tolstring(interpreter, -1, null_mut()) } else { c"?".as_ptr() };
        }
        return lual_error(interpreter, c"bad argument #%d to '%s' (%s)".as_ptr(), arg, debuginfo.debuginfo_name, extramsg);
    }
}
pub unsafe fn lual_typeerror(interpreter: *mut Interpreter, arg: i32, tname: *const i8) -> i32 {
    unsafe {
        let message: *const i8;
        let typearg: *const i8;
        if lual_getmetafield(interpreter, arg, c"__name".as_ptr()) == TagType::String {
            typearg = lua_tolstring(interpreter, -1, null_mut());
        } else if lua_type(interpreter, arg) == Some(TagType::Pointer) {
            typearg = c"light userdata".as_ptr();
        } else {
            typearg = lua_typename(interpreter, lua_type(interpreter, arg));
        }
        message = lua_pushfstring(interpreter, c"%s expected, got %s".as_ptr(), tname, typearg);
        return lual_argerror(interpreter, arg, message);
    }
}
pub unsafe fn tag_error2(interpreter: *mut Interpreter, arg: i32, tagtype: Option<TagType>) {
    unsafe {
        lual_typeerror(interpreter, arg, lua_typename(interpreter, tagtype));
    }
}
pub unsafe fn lual_where(interpreter: *mut Interpreter, level: i32) {
    unsafe {
        let mut debuginfo: DebugInfo = DebugInfo::new();
        if lua_getstack(interpreter, level, &mut debuginfo) != 0 {
            lua_getinfo(interpreter, c"Sl".as_ptr(), &mut debuginfo);
            if debuginfo.debuginfo_currentline > 0 {
                lua_pushfstring(interpreter, c"%s:%d: ".as_ptr(), (debuginfo.debuginfo_shortsrc).as_mut_ptr(), debuginfo.debuginfo_currentline);
                return;
            }
        }
        lua_pushfstring(interpreter, c"".as_ptr());
    }
}
pub unsafe extern "C" fn lual_error(interpreter: *mut Interpreter, fmt: *const i8, args: ...) -> i32 {
    unsafe {
        let mut argp: ::core::ffi::VaListImpl = args.clone();
        lual_where(interpreter, 1);
        lua_pushvfstring(interpreter, fmt, argp.as_va_list());
        lua_concat(interpreter, 2);
        return lua_error(interpreter);
    }
}
pub unsafe fn lual_fileresult(interpreter: *mut Interpreter, stat: i32, fname: *const i8) -> i32 {
    unsafe {
        let en: i32 = *__errno_location();
        if stat != 0 {
            (*interpreter).push_boolean(true);
            return 1;
        } else {
            (*interpreter).push_nil();
            let message: *const i8 = if en != 0 { strerror(en) as *const i8 } else { c"(no extra info)".as_ptr() };
            if !fname.is_null() {
                lua_pushfstring(interpreter, c"%s: %s".as_ptr(), fname, message);
            } else {
                lua_pushstring(interpreter, message);
            }
            (*interpreter).push_integer(en as i64);
            return 3;
        };
    }
}
pub unsafe fn lual_execresult(interpreter: *mut Interpreter, mut stat: i32) -> i32 {
    unsafe {
        if stat != 0 && *__errno_location() != 0 {
            return lual_fileresult(interpreter, 0, null());
        } else {
            let mut what: *const i8 = c"exit".as_ptr();
            if stat & 0x7f as i32 == 0 {
                stat = (stat & 0xff00 as i32) >> 8;
            } else if ((stat & 0x7f as i32) + 1) as i32 >> 1 > 0 {
                stat = stat & 0x7f as i32;
                what = c"signal".as_ptr();
            }
            if *what as i32 == Character::LowerE as i32 && stat == 0 {
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
pub unsafe fn lual_newmetatable(interpreter: *mut Interpreter, tname: *const i8) -> i32 {
    unsafe {
        if lua_getfield(interpreter, -1000000 - 1000, tname) != TagType::Nil {
            return 0;
        } else {
            lua_settop(interpreter, -2);
            (*interpreter).lua_createtable();
            lua_pushstring(interpreter, tname);
            lua_setfield(interpreter, -2, c"__name".as_ptr());
            lua_pushvalue(interpreter, -1);
            lua_setfield(interpreter, -(1000000 as i32) - 1000 as i32, tname);
            return 1;
        }
    }
}
pub unsafe fn lual_setmetatable(interpreter: *mut Interpreter, tname: *const i8) {
    unsafe {
        lua_getfield(interpreter, -(1000000 as i32) - 1000 as i32, tname);
        lua_setmetatable(interpreter, -2);
    }
}
pub unsafe fn lual_testudata(interpreter: *mut Interpreter, arbitrary_data: i32, tname: *const i8) -> *mut libc::c_void {
    unsafe {
        let mut p: *mut libc::c_void = (*interpreter).to_pointer(arbitrary_data);
        if !p.is_null() {
            if (*interpreter).lua_getmetatable(arbitrary_data) {
                lua_getfield(interpreter, -(1000000 as i32) - 1000 as i32, tname);
                if !lua_rawequal(interpreter, -1, -2) {
                    p = null_mut();
                }
                lua_settop(interpreter, -2 - 1);
                return p;
            }
        }
        return null_mut();
    }
}
pub unsafe fn lual_checkudata(interpreter: *mut Interpreter, arbitrary_data: i32, tname: *const i8) -> *mut libc::c_void {
    unsafe {
        let p: *mut libc::c_void = lual_testudata(interpreter, arbitrary_data, tname);
        if p.is_null() {
            lual_typeerror(interpreter, arbitrary_data, tname);
        }
        return p;
    }
}
pub unsafe fn lual_checkoption(interpreter: *mut Interpreter, arg: i32, def: *const i8, lst: *const *const i8) -> i32 {
    unsafe {
        let name: *const i8 = if !def.is_null() { lual_optlstring(interpreter, arg, def, null_mut()) } else { lual_checklstring(interpreter, arg, null_mut()) };
        let mut i: i32;
        i = 0;
        while !(*lst.offset(i as isize)).is_null() {
            if strcmp(*lst.offset(i as isize), name) == 0 {
                return i;
            }
            i += 1;
        }
        return lual_argerror(interpreter, arg, lua_pushfstring(interpreter, c"invalid option '%s'".as_ptr(), name));
    }
}
pub unsafe fn lual_checkstack(interpreter: *mut Interpreter, space: i32, message: *const i8) {
    unsafe {
        if lua_checkstack(interpreter, space) == 0 {
            if message.is_null() {
                lual_error(interpreter, c"stack overflow".as_ptr());
            } else {
                lual_error(interpreter, c"stack overflow (%s)".as_ptr(), message);
            }
        }
    }
}
pub unsafe fn lual_checkany(interpreter: *mut Interpreter, arg: i32) {
    unsafe {
        if lua_type(interpreter, arg) == None {
            lual_argerror(interpreter, arg, c"value expected".as_ptr());
        }
    }
}
pub unsafe fn lual_checklstring(interpreter: *mut Interpreter, arg: i32, length: *mut usize) -> *const i8 {
    unsafe {
        let s: *const i8 = lua_tolstring(interpreter, arg, length);
        if s.is_null() {
            tag_error2(interpreter, arg, Some(TagType::String));
        }
        return s;
    }
}
pub unsafe fn lual_optlstring(interpreter: *mut Interpreter, arg: i32, def: *const i8, length: *mut usize) -> *const i8 {
    unsafe {
        match lua_type(interpreter, arg) {
            None | Some(TagType::Nil) => {
                if !length.is_null() {
                    *length = if !def.is_null() { strlen(def) as usize } else { 0usize };
                }
                return def;
            },
            _ => {
                return lual_checklstring(interpreter, arg, length);
            },
        }
    }
}
pub unsafe fn lual_checknumber(interpreter: *mut Interpreter, arg: i32) -> f64 {
    unsafe {
        let mut is_number = false;
        let d: f64 = lua_tonumberx(interpreter, arg, &mut is_number);
        if !is_number {
            tag_error2(interpreter, arg, Some(TagType::Numeric));
        }
        return d;
    }
}
pub unsafe fn lual_optnumber(interpreter: *mut Interpreter, arg: i32, def: f64) -> f64 {
    unsafe {
        match lua_type(interpreter, arg) {
            None | Some(TagType::Nil) => def,
            _ => lual_checknumber(interpreter, arg),
        }
    }
}
pub unsafe fn interror(interpreter: *mut Interpreter, arg: i32) {
    unsafe {
        if lua_isnumber(interpreter, arg) {
            lual_argerror(interpreter, arg, c"number has no integer representation".as_ptr());
        } else {
            tag_error2(interpreter, arg, Some(TagType::Numeric));
        };
    }
}
pub unsafe fn lual_checkinteger(interpreter: *mut Interpreter, arg: i32) -> i64 {
    unsafe {
        let mut is_number = false;
        let ret: i64 = lua_tointegerx(interpreter, arg, &mut is_number);
        if !is_number {
            interror(interpreter, arg);
        }
        return ret;
    }
}
pub unsafe fn lual_optinteger(interpreter: *mut Interpreter, arg: i32, def: i64) -> i64 {
    unsafe {
        return match lua_type(interpreter, arg) {
            None | Some(TagType::Nil) => def,
            _ => lual_checkinteger(interpreter, arg),
        };
    }
}
pub unsafe fn get_f(mut _state: *mut Interpreter, arbitrary_data: *mut libc::c_void, size: *mut usize) -> *const i8 {
    unsafe {
        let lf: *mut LoadF = arbitrary_data as *mut LoadF;
        if (*lf).n > 0 {
            *size = (*lf).n as usize;
            (*lf).n = 0;
        } else {
            if feof((*lf).file) != 0 {
                return null();
            }
            *size = fread(((*lf).buffer).as_mut_ptr() as *mut libc::c_void, 1, size_of::<[i8; 8192]>(), (*lf).file) as usize;
        }
        return ((*lf).buffer).as_mut_ptr();
    }
}
pub unsafe fn errfile(interpreter: *mut Interpreter, what: *const i8, fnameindex: i32) -> Status {
    unsafe {
        let err: i32 = *__errno_location();
        let filename: *const i8 = (lua_tolstring(interpreter, fnameindex, null_mut())).offset(1 as isize);
        if err != 0 {
            lua_pushfstring(interpreter, c"cannot %s %s: %s".as_ptr(), what, filename, strerror(err));
        } else {
            lua_pushfstring(interpreter, c"cannot %s %s".as_ptr(), what, filename);
        }
        lua_rotate(interpreter, fnameindex, -1);
        lua_settop(interpreter, -2);
        return Status::FileError;
    }
}
pub unsafe fn skip_bom(file: *mut FILE) -> i32 {
    unsafe {
        let c: i32 = getc(file);
        if c == 0xef as i32 && getc(file) == 0xbb as i32 && getc(file) == 0xbf as i32 {
            return getc(file);
        } else {
            return c;
        };
    }
}
pub unsafe fn skipcomment(file: *mut FILE, pointer: *mut i32) -> i32 {
    unsafe {
        *pointer = skip_bom(file);
        let mut c: i32 = *pointer;
        if c == Character::Octothorpe as i32 {
            loop {
                c = getc(file);
                if !(c != -1 && c != Character::LineFeed as i32) {
                    break;
                }
            }
            *pointer = getc(file);
            return 1;
        } else {
            return 0;
        };
    }
}
pub unsafe fn lual_loadfilex(interpreter: *mut Interpreter, filename: *const i8, mode: *const i8) -> Status {
    unsafe {
        let mut lf: LoadF = LoadF { n: 0, file: null_mut(), buffer: [0; 8192] };
        let readstatus: i32;
        let mut c: i32 = 0;
        let fnameindex: i32 = (*interpreter).get_top() + 1;
        if filename.is_null() {
            lua_pushstring(interpreter, c"=stdin".as_ptr());
            lf.file = stdin;
        } else {
            lua_pushfstring(interpreter, c"@%s".as_ptr(), filename);
            *__errno_location() = 0;
            lf.file = fopen(filename, c"r".as_ptr());
            if (lf.file).is_null() {
                return errfile(interpreter, c"open".as_ptr(), fnameindex);
            }
        }
        lf.n = 0;
        if skipcomment(lf.file, &mut c) != 0 {
            let fresh148 = lf.n;
            lf.n = lf.n + 1;
            lf.buffer[fresh148 as usize] = Character::LineFeed as i8;
        }
        if c == (*::core::mem::transmute::<&[u8; 5], &[i8; 5]>(b"\x1BLua\0"))[0] as i32 {
            lf.n = 0;
            if !filename.is_null() {
                *__errno_location() = 0;
                lf.file = freopen(filename, c"rb".as_ptr(), lf.file);
                if (lf.file).is_null() {
                    return errfile(interpreter, c"reopen".as_ptr(), fnameindex);
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
        let reader = Reader::new(Some(get_f as unsafe fn(*mut Interpreter, *mut libc::c_void, *mut usize) -> *const i8));
        let status = lua_load(
            interpreter,
            reader,
            &mut lf as *mut LoadF as *mut libc::c_void,
            lua_tolstring(interpreter, -1, null_mut()),
            mode,
        );
        readstatus = ferror(lf.file);
        if !filename.is_null() {
            fclose(lf.file);
        }
        if readstatus != 0 {
            lua_settop(interpreter, fnameindex);
            return errfile(interpreter, c"read".as_ptr(), fnameindex);
        }
        lua_rotate(interpreter, fnameindex, -1);
        lua_settop(interpreter, -2);
        return status;
    }
}
pub unsafe fn get_s(mut _state: *mut Interpreter, arbitrary_data: *mut libc::c_void, size: *mut usize) -> *const i8 {
    unsafe {
        let load_s: *mut VectorT<i8> = arbitrary_data as *mut VectorT<i8>;
        if (*load_s).get_size() == 0 {
            return null();
        } else {
            let (capitulated_pointer, capitulated_size) = (*load_s).capitulate();
            *size = capitulated_size;
            return capitulated_pointer;
        }
    }
}
pub unsafe fn lual_loadbufferx(interpreter: *mut Interpreter, buffer: *const i8, size: usize, name: *const i8, mode: *const i8) -> Status {
    unsafe {
        let mut load_s: VectorT<i8> = VectorT::<i8>::new();
        load_s.inject(buffer as *mut i8, size);
        let reader: Reader = Reader::new(Some(get_s as unsafe fn(*mut Interpreter, *mut libc::c_void, *mut usize) -> *const i8));
        return lua_load(
            interpreter,
            reader,
            &mut load_s as *mut VectorT<i8> as *mut libc::c_void,
            name,
            mode,
        );
    }
}
pub unsafe fn lual_getmetafield(interpreter: *mut Interpreter, obj: i32, event: *const i8) -> TagType {
    unsafe {
        if (*interpreter).lua_getmetatable(obj) {
            lua_pushstring(interpreter, event);
            let tagtype = lua_rawget(interpreter, -2);
            if tagtype == TagType::Nil {
                lua_settop(interpreter, -3);
            } else {
                lua_rotate(interpreter, -2, -1);
                lua_settop(interpreter, -2);
            }
            return tagtype;
        } else {
            return TagType::Nil;
        };
    }
}
pub unsafe fn lual_callmeta(interpreter: *mut Interpreter, mut obj: i32, event: *const i8) -> bool {
    unsafe {
        obj = lua_absindex(interpreter, obj);
        if lual_getmetafield(interpreter, obj, event) == TagType::Nil {
            return false;
        }
        lua_pushvalue(interpreter, obj);
        (*interpreter).lua_callk(1, 1, 0, None);
        return true;
    }
}
pub unsafe fn lual_len(interpreter: *mut Interpreter, index: i32) -> i64 {
    unsafe {
        let l: i64;
        let mut is_number = false;
        lua_len(interpreter, index);
        l = lua_tointegerx(interpreter, -1, &mut is_number);
        if !is_number {
            lual_error(interpreter, c"object length is not an integer".as_ptr());
        }
        lua_settop(interpreter, -2);
        return l;
    }
}
pub unsafe fn lual_tolstring(interpreter: *mut Interpreter, mut index: i32, length: *mut usize) -> *const i8 {
    unsafe {
        index = lua_absindex(interpreter, index);
        if lual_callmeta(interpreter, index, c"__tostring".as_ptr()) {
            if !lua_isstring(interpreter, -1) {
                lual_error(interpreter, c"'__tostring' must return a string".as_ptr());
            }
        } else {
            match lua_type(interpreter, index) {
                Some(TagType::Numeric) => {
                    if lua_isinteger(interpreter, index) {
                        lua_pushfstring(interpreter, c"%I".as_ptr(), lua_tointegerx(interpreter, index, null_mut()));
                    } else {
                        lua_pushfstring(interpreter, c"%f".as_ptr(), lua_tonumberx(interpreter, index, null_mut()));
                    }
                },
                Some(TagType::String) => {
                    lua_pushvalue(interpreter, index);
                },
                Some(TagType::Boolean) => {
                    lua_pushstring(interpreter, if lua_toboolean(interpreter, index) { c"true".as_ptr() } else { c"false".as_ptr() });
                },
                Some(TagType::Nil) => {
                    lua_pushstring(interpreter, c"nil".as_ptr());
                },
                _ => {
                    let tagtype = lual_getmetafield(interpreter, index, c"__name".as_ptr());
                    let kind: *const i8 = if tagtype == TagType::String {
                        lua_tolstring(interpreter, -1, null_mut())
                    } else {
                        lua_typename(interpreter, lua_type(interpreter, index))
                    };
                    lua_pushfstring(interpreter, c"%s: %p".as_ptr(), kind, (*interpreter).to_pointer(index));
                    if tagtype != TagType::Nil {
                        lua_rotate(interpreter, -2, -1);
                        lua_settop(interpreter, -2);
                    }
                },
            }
        }
        return lua_tolstring(interpreter, -1, length);
    }
}
pub unsafe fn lual_setfuncs(interpreter: *mut Interpreter, registered_functions: *const RegisteredFunction, count_registered_functions: usize, count_upvalues: i32) {
    unsafe {
        lual_checkstack(interpreter, count_upvalues, c"too many upvalues".as_ptr());
        for it in 0..count_registered_functions {
            if (*registered_functions.offset(it as isize)).function.is_none() {
                (*interpreter).push_boolean(false);
            } else {
                for _ in 0..count_upvalues {
                    lua_pushvalue(interpreter, -count_upvalues);
                }
                lua_pushcclosure(interpreter, (*registered_functions.offset(it as isize)).function, count_upvalues);
            }
            lua_setfield(interpreter, -(count_upvalues + 2), (*registered_functions.offset(it as isize)).name);
        }
        lua_settop(interpreter, -count_upvalues - 1);
    }
}
pub unsafe fn lual_getsubtable(interpreter: *mut Interpreter, mut index: i32, fname: *const i8) -> i32 {
    unsafe {
        if lua_getfield(interpreter, index, fname) == TagType::Table {
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
pub unsafe fn lual_requiref(interpreter: *mut Interpreter, modname: *const i8, openf: CFunction, glb: i32) {
    unsafe {
        lual_getsubtable(interpreter, -(1000000 as i32) - 1000 as i32, c"_LOADED".as_ptr());
        lua_getfield(interpreter, -1, modname);
        if !lua_toboolean(interpreter, -1) {
            lua_settop(interpreter, -2);
            lua_pushcclosure(interpreter, openf, 0);
            lua_pushstring(interpreter, modname);
            (*interpreter).lua_callk(1, 1, 0, None);
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
pub unsafe fn lual_addgsub(b: *mut Buffer, mut s: *const i8, p: *const i8, r: *const i8) {
    unsafe {
        let mut wild: *const i8;
        let l = strlen(p);
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
pub unsafe fn lual_gsub(interpreter: *mut Interpreter, s: *const i8, p: *const i8, r: *const i8) -> *const i8 {
    unsafe {
        let mut b = Buffer::new();
        b.initialize(interpreter);
        lual_addgsub(&mut b, s, p, r);
        b.push_result();
        return lua_tolstring(interpreter, -1, null_mut());
    }
}
pub unsafe fn raw_allocate(ptr: *mut libc::c_void, mut _old_size: usize, new_size: usize) -> *mut libc::c_void {
    unsafe {
        if new_size == 0 {
            free(ptr);
            return null_mut();
        } else {
            return realloc(ptr, new_size);
        };
    }
}
pub unsafe fn panic(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let message: *const i8 = if lua_type(interpreter, -1) == Some(TagType::String) {
            lua_tolstring(interpreter, -1, null_mut())
        } else {
            c"error object is not a string".as_ptr()
        };
        fprintf(stderr, c"PANIC: unprotected error in call to Lua API (%s)\n".as_ptr(), message);
        fflush(stderr);
        return 0;
    }
}
pub unsafe fn checkcontrol(interpreter: *mut Interpreter, mut message: *const i8, tocont: i32) -> i32 {
    unsafe {
        if tocont != 0 || {
            let fresh150 = message;
            message = message.offset(1);
            *fresh150 as i32 != Character::At as i32
        } {
            return 0;
        } else {
            if strcmp(message, c"off".as_ptr()) == 0 {
                lua_setwarnf(interpreter, Some(warnfoff as unsafe fn(*mut libc::c_void, *const i8, i32) -> ()), interpreter as *mut libc::c_void);
            } else if strcmp(message, c"on".as_ptr()) == 0 {
                lua_setwarnf(interpreter, Some(warnfon as unsafe fn(*mut libc::c_void, *const i8, i32) -> ()), interpreter as *mut libc::c_void);
            }
            return 1;
        };
    }
}
pub unsafe fn warnfoff(arbitrary_data: *mut libc::c_void, message: *const i8, tocont: i32) {
    unsafe {
        checkcontrol(arbitrary_data as *mut Interpreter, message, tocont);
    }
}
pub unsafe fn warnfcont(arbitrary_data: *mut libc::c_void, message: *const i8, tocont: i32) {
    unsafe {
        let interpreter: *mut Interpreter = arbitrary_data as *mut Interpreter;
        fprintf(stderr, c"%s".as_ptr(), message);
        fflush(stderr);
        if tocont != 0 {
            lua_setwarnf(interpreter, Some(warnfcont as unsafe fn(*mut libc::c_void, *const i8, i32) -> ()), interpreter as *mut libc::c_void);
        } else {
            fprintf(stderr, c"%s".as_ptr(), c"\n".as_ptr());
            fflush(stderr);
            lua_setwarnf(interpreter, Some(warnfon as unsafe fn(*mut libc::c_void, *const i8, i32) -> ()), interpreter as *mut libc::c_void);
        };
    }
}
pub unsafe fn warnfon(arbitrary_data: *mut libc::c_void, message: *const i8, tocont: i32) {
    unsafe {
        if checkcontrol(arbitrary_data as *mut Interpreter, message, tocont) != 0 {
            return;
        }
        fprintf(stderr, c"%s".as_ptr(), c"Lua warning: ".as_ptr());
        fflush(stderr);
        warnfcont(arbitrary_data, message, tocont);
    }
}
pub unsafe fn lual_newstate() -> (*mut Global, *mut Interpreter) {
    unsafe {
        let global: *mut Global = raw_allocate(null_mut(), 0, size_of::<Global>()) as *mut Global;
        if !global.is_null() {
            let mut interpreter: *mut Interpreter = raw_allocate(null_mut(), 0, size_of::<Interpreter>()) as *mut Interpreter;
            if interpreter.is_null() {
                raw_allocate(global as *mut u8 as *mut libc::c_void, size_of::<Global>(), 0);
            } else {
                (*global).init();
                (*interpreter).init(&*global);
                (*global).global_totalbytes += size_of::<Interpreter>() as i64;
                (*global).global_totalbytes += size_of::<Global>() as i64;
                (*interpreter).preinit_thread(global);
                (*global).global_allgc = &mut (*(interpreter as *mut ObjectBase));
                (*interpreter).as_object_mut().next = null_mut();
                (*interpreter).count_c_calls = ((*interpreter).count_c_calls as u32).wrapping_add(0x10000 as u32) as u32;
                (*global).global_maininterpreter = interpreter;
                (*global).global_seed = luai_makeseed(interpreter);
                if luad_rawrunprotected(interpreter, Some(f_luaopen as unsafe fn(*mut Interpreter, *mut libc::c_void) -> ()), null_mut()) != Status::OK {
                    close_state(interpreter);
                    interpreter = null_mut();
                }
                if !interpreter.is_null() {
                    lua_atpanic(interpreter, Some(panic as unsafe fn(*mut Interpreter) -> i32));
                    lua_setwarnf(interpreter, Some(warnfoff as unsafe fn(*mut libc::c_void, *const i8, i32) -> ()), interpreter as *mut libc::c_void);
                }
                return (global, interpreter);
            }
        }
        return (null_mut(), null_mut());
    }
}
pub unsafe fn lual_checkversion_(interpreter: *mut Interpreter, version: f64, size: usize) {
    unsafe {
        let v: f64 = 504.0;
        if size != (size_of::<i64>() as usize).wrapping_mul(16 as usize).wrapping_add(size_of::<f64>() as usize) {
            lual_error(interpreter, c"core and library have incompatible numeric types".as_ptr());
        } else if v != version {
            lual_error(interpreter, c"version mismatch: app. needs %f, Lua core provides %f".as_ptr(), version, v);
        }
    }
}
pub unsafe fn luab_print(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let n: i32 = (*interpreter).get_top();
        for i in 1..(1 + n) {
            let mut l: usize = 0;
            let s: *const i8 = lual_tolstring(interpreter, i, &mut l);
            if i > 1 {
                fwrite(c"\t".as_ptr() as *const libc::c_void, 1, 1, stdout);
            }
            fwrite(s as *const libc::c_void, 1, l as usize, stdout);
            lua_settop(interpreter, -2);
        }
        fwrite(c"\n".as_ptr() as *const libc::c_void, 1, 1, stdout);
        fflush(stdout);
        return 0;
    }
}
pub unsafe fn luab_warn(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let n: i32 = (*interpreter).get_top();
        lual_checklstring(interpreter, 1, null_mut());
        for i in 2..(1 + n) {
            lual_checklstring(interpreter, i, null_mut());
        }
        for i in 1..n {
            lua_warning(interpreter, lua_tolstring(interpreter, i, null_mut()), 1);
        }
        lua_warning(interpreter, lua_tolstring(interpreter, n, null_mut()), 0);
        return 0;
    }
}
pub unsafe fn l_print(interpreter: *mut Interpreter) {
    unsafe {
        let n: i32 = (*interpreter).get_top();
        if n > 0 {
            lual_checkstack(interpreter, 20 as i32, c"too many results to print".as_ptr());
            lua_getglobal(interpreter, c"print".as_ptr());
            lua_rotate(interpreter, 1, 1);
            if CallS::api_call(interpreter, n, 0, 0, 0, None) != Status::OK {
                l_message(PROGRAM_NAME, lua_pushfstring(interpreter, c"error calling 'print' (%s)".as_ptr(), lua_tolstring(interpreter, -1, null_mut())));
            }
        }
    }
}
pub static mut GLOBAL_STATE: *mut Interpreter = null_mut();
pub static mut PROGRAM_NAME: *const i8 = c"lua".as_ptr();
pub unsafe fn setsignal(sig: i32, handler: Option<unsafe fn(i32) -> ()>) {
    unsafe {
        let mut signalaction: SignalAction = SignalAction { __sigaction_handler: SigActionA { sa_handler: None }, sa_mask: SIgnalSet { __val: [0; 16] }, sa_flags: 0, sa_restorer: None };
        signalaction.__sigaction_handler.sa_handler = handler;
        signalaction.sa_flags = 0;
        sigemptyset(&mut signalaction.sa_mask);
        sigaction(sig, &mut signalaction, null_mut());
    }
}
pub unsafe fn lstop(interpreter: *mut Interpreter, mut _ar: *mut DebugInfo) {
    unsafe {
        lua_sethook(interpreter, None, 0, 0);
        lual_error(interpreter, c"interrupted!".as_ptr());
    }
}
pub unsafe fn laction(i: i32) {
    unsafe {
        let flag: i32 = 1 << 0 | 1 << 1 | 1 << 2 | 1 << 3;
        setsignal(i, None);
        lua_sethook(GLOBAL_STATE, Some(lstop as unsafe fn(*mut Interpreter, *mut DebugInfo) -> ()), flag, 1);
    }
}
pub unsafe fn print_usage(badoption: *const i8) {
    unsafe {
        fprintf(stderr, c"%s: ".as_ptr(), PROGRAM_NAME);
        fflush(stderr);
        if *badoption.offset(1 as isize) as i32 == Character::LowerE as i32 || *badoption.offset(1 as isize) as i32 == Character::LowerL as i32 {
            fprintf(stderr, c"'%s' needs argument\n".as_ptr(), badoption);
            fflush(stderr);
        } else {
            fprintf(stderr, c"unrecognized option '%s'\n".as_ptr(), badoption);
            fflush(stderr);
        }
        fprintf(
            stderr,
            c"usage: %s [options] [script [args]]\nAvailable options are:\n  -e stat   execute string 'stat'\n  -i        enter interactive mode after executing 'script'\n  -l mod    require library 'mod' into global 'mod'\n  -l global=mod  require library 'mod' into global Character::LowerG\n  -v        show version information\n  -E        ignore environment variables\n  -W        turn warnings on\n  --        stop handling options\n  -         stop handling options and execute stdin\n".as_ptr(),
            PROGRAM_NAME,
        );
        fflush(stderr);
    }
}
pub unsafe fn l_message(pname: *const i8, message: *const i8) {
    unsafe {
        if !pname.is_null() {
            fprintf(stderr, c"%s: ".as_ptr(), pname);
            fflush(stderr);
        }
        fprintf(stderr, c"%s\n".as_ptr(), message);
        fflush(stderr);
    }
}
pub unsafe fn report(interpreter: *mut Interpreter, status: Status) -> Status {
    unsafe {
        if status != Status::OK {
            let mut message: *const i8 = lua_tolstring(interpreter, -1, null_mut());
            if message.is_null() {
                message = c"(error message not a string)".as_ptr();
            }
            l_message(PROGRAM_NAME, message);
            lua_settop(interpreter, -2);
        }
        return status;
    }
}
pub unsafe fn msghandler(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let mut message: *const i8 = lua_tolstring(interpreter, 1, null_mut());
        if message.is_null() {
            if lual_callmeta(interpreter, 1, c"__tostring".as_ptr()) && lua_type(interpreter, -1) == Some(TagType::String) {
                return 1;
            } else {
                message = lua_pushfstring(interpreter, c"(error object is a %s value)".as_ptr(), lua_typename(interpreter, lua_type(interpreter, 1)));
            }
        }
        lual_traceback(interpreter, interpreter, message, 1);
        return 1;
    }
}
pub unsafe fn docall(interpreter: *mut Interpreter, narg: i32, nres: i32) -> Status {
    unsafe {
        let base: i32 = (*interpreter).get_top() - narg;
        lua_pushcclosure(interpreter, Some(msghandler as unsafe fn(*mut Interpreter) -> i32), 0);
        lua_rotate(interpreter, base, 1);
        GLOBAL_STATE = interpreter;
        setsignal(2, Some(laction as unsafe fn(i32) -> ()));
        let status = CallS::api_call(interpreter, narg, nres, base, 0, None);
        setsignal(2, None);
        lua_rotate(interpreter, base, -1);
        lua_settop(interpreter, -2);
        return status;
    }
}
pub unsafe fn createargtable(interpreter: *mut Interpreter, argv: *mut *mut i8, argc: i32, script: i32) {
    unsafe {
        (*interpreter).lua_createtable();
        for i in 0..argc {
            lua_pushstring(interpreter, *argv.offset(i as isize));
            lua_rawseti(interpreter, -2, (i - script) as i64);
        }
        lua_setglobal(interpreter, c"arg".as_ptr());
    }
}
pub unsafe fn dochunk(interpreter: *mut Interpreter, mut status: Status) -> Status {
    unsafe {
        if status == Status::OK {
            status = docall(interpreter, 0, 0);
        }
        return report(interpreter, status);
    }
}
pub unsafe fn dofile(interpreter: *mut Interpreter, name: *const i8) -> Status {
    unsafe {
        return dochunk(interpreter, lual_loadfilex(interpreter, name, null()));
    }
}
pub unsafe fn dostring(interpreter: *mut Interpreter, s: *const i8, name: *const i8) -> Status {
    unsafe {
        return dochunk(interpreter, lual_loadbufferx(interpreter, s, strlen(s) as usize, name, null()));
    }
}
pub unsafe fn dolibrary(interpreter: *mut Interpreter, globname: *mut i8) -> Status {
    unsafe {
        let mut suffix: *mut i8 = null_mut();
        let mut modname: *mut i8 = strchr(globname, Character::Equal as i32);
        if modname.is_null() {
            modname = globname;
            suffix = strchr(modname, *(c"-".as_ptr()) as i32);
        } else {
            *modname = Character::Null as i8;
            modname = modname.offset(1);
        }
        lua_getglobal(interpreter, c"require".as_ptr());
        lua_pushstring(interpreter, modname);
        let status = docall(interpreter, 1, 1);
        if status == Status::OK {
            if !suffix.is_null() {
                *suffix = Character::Null as i8;
            }
            lua_setglobal(interpreter, globname);
        }
        return report(interpreter, status);
    }
}
pub unsafe fn pushargs(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        let n: i32;
        if lua_getglobal(interpreter, c"arg".as_ptr()) != TagType::Table {
            lual_error(interpreter, c"'arg' is not a table".as_ptr());
        }
        n = lual_len(interpreter, -1) as i32;
        lual_checkstack(interpreter, n + 3, c"too many arguments to script".as_ptr());
        for i in 1..(1 + n) {
            lua_rawgeti(interpreter, -i, i as i64);
        }
        lua_rotate(interpreter, -(1 + n), -1);
        lua_settop(interpreter, -2);
        return n;
    }
}
pub unsafe fn handle_script(interpreter: *mut Interpreter, argv: *mut *mut i8) -> Status {
    unsafe {
        let mut fname: *const i8 = *argv.offset(0 as isize);
        if strcmp(fname, c"-".as_ptr()) == 0 && strcmp(*argv.offset(-1 as isize), c"--".as_ptr()) != 0 {
            fname = null();
        }
        let mut status = lual_loadfilex(interpreter, fname, null());
        if status == Status::OK {
            let n: i32 = pushargs(interpreter);
            status = docall(interpreter, n, -1);
        }
        return report(interpreter, status);
    }
}
pub unsafe fn collectargs(argv: *mut *mut i8, first: *mut i32) -> i32 {
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
            if *(*argv.offset(i as isize)).offset(0 as isize) as i32 != Character::Hyphen as i32 {
                return args;
            }
            let current_block_31: usize;
            match Character::from2(*(*argv.offset(i as isize)).offset(1 as isize) as i32) {
                Some(Character::Hyphen) => {
                    if *(*argv.offset(i as isize)).offset(2 as isize) as i32 != Character::Null as i32 {
                        return 1;
                    }
                    *first = i + 1;
                    return args;
                },
                None => return args,
                Some(Character::UpperE) => {
                    if *(*argv.offset(i as isize)).offset(2 as isize) as i32 != Character::Null as i32 {
                        return 1;
                    }
                    args |= 16 as i32;
                    current_block_31 = 4761528863920922185;
                },
                Some(Character::UpperW) => {
                    if *(*argv.offset(i as isize)).offset(2 as isize) as i32 != Character::Null as i32 {
                        return 1;
                    }
                    current_block_31 = 4761528863920922185;
                },
                Some(Character::LowerI) => {
                    args |= 2;
                    current_block_31 = 6636775023221328366;
                },
                Some(Character::LowerV) => {
                    current_block_31 = 6636775023221328366;
                },
                Some(Character::LowerE) => {
                    args |= 8;
                    current_block_31 = 15172496195422792753;
                },
                Some(Character::LowerL) => {
                    current_block_31 = 15172496195422792753;
                },
                _ => return 1,
            }
            match current_block_31 {
                6636775023221328366 => {
                    if *(*argv.offset(i as isize)).offset(2 as isize) as i32 != Character::Null as i32 {
                        return 1;
                    }
                    args |= 4;
                },
                15172496195422792753 => {
                    if *(*argv.offset(i as isize)).offset(2 as isize) as i32 == Character::Null as i32 {
                        i += 1;
                        if (*argv.offset(i as isize)).is_null() || *(*argv.offset(i as isize)).offset(0 as isize) as i32 == Character::Hyphen as i32 {
                            return 1;
                        }
                    }
                },
                _ => {},
            }
            i += 1;
        }
        *first = 0;
        return args;
    }
}
pub unsafe fn runargs(interpreter: *mut Interpreter, argv: *mut *mut i8, n: i32) -> i32 {
    unsafe {
        for i in 0..n {
            let option: Character = Character::from(*(*argv.offset(i as isize)).offset(1 as isize) as i32);
            match option {
                Character::LowerE | Character::LowerL => {
                    let extra: *mut i8 = (*argv.offset(i as isize)).offset(2 as isize);
                    if *extra as i32 == Character::Null as i32 {
                        continue;
                    }
                    let status = if option == Character::LowerE {
                        dostring(interpreter, extra, c"=(command line)".as_ptr())
                    } else {
                        dolibrary(interpreter, extra)
                    };
                    if status != Status::OK {
                        return 0;
                    }
                },
                Character::UpperW => {
                    lua_warning(interpreter, c"@on".as_ptr(), 0);
                },
                _ => {},
            }
        }
        return 1;
    }
}
pub unsafe fn get_prompt(interpreter: *mut Interpreter, firstline: i32) -> *const i8 {
    unsafe {
        if lua_getglobal(interpreter, if firstline != 0 { c"_PROMPT".as_ptr() } else { c"_PROMPT2".as_ptr() }) == TagType::Nil {
            return if firstline != 0 { c"> ".as_ptr() } else { c">> ".as_ptr() };
        } else {
            let p: *const i8 = lual_tolstring(interpreter, -1, null_mut());
            lua_rotate(interpreter, -2, -1);
            lua_settop(interpreter, -2);
            return p;
        };
    }
}
pub unsafe fn incomplete(interpreter: *mut Interpreter, status: Status) -> i32 {
    unsafe {
        if status == Status::SyntaxError {
            let mut lmsg: usize = 0;
            let message: *const i8 = lua_tolstring(interpreter, -1, &mut lmsg);
            if lmsg >= (size_of::<[i8; 6]>() as usize).wrapping_sub(1 as usize) && strcmp(message.offset(lmsg as isize).offset(-((size_of::<[i8; 6]>() as usize).wrapping_sub(1 as usize) as isize)), c"<eof>".as_ptr()) == 0 {
                return 1;
            }
        }
        return 0;
    }
}
pub unsafe fn pushline(interpreter: *mut Interpreter, firstline: i32) -> bool {
    unsafe {
        let mut buffer: [i8; 512] = [0; 512];
        let b: *mut i8 = buffer.as_mut_ptr();
        let prmt: *const i8 = get_prompt(interpreter, firstline);
        fputs(prmt, stdout);
        fflush(stdout);
        let readstatus = !fgets(b, 512 as i32, stdin).is_null();
        lua_settop(interpreter, 0);
        if !readstatus {
            return false;
        } else {
            let mut l: usize = strlen(b) as usize;
            if l > 0 && *b.offset(l.wrapping_sub(1 as usize) as isize) as i32 == Character::LineFeed as i32 {
                l = l.wrapping_sub(1);
                *b.offset(l as isize) = Character::Null as i8;
            }
            if firstline != 0 && *b.offset(0 as isize) as i32 == Character::Equal as i32 {
                lua_pushfstring(interpreter, c"return %s".as_ptr(), b.offset(1 as isize));
            } else {
                lua_pushlstring(interpreter, b, l);
            }
            return true;
        }
    }
}
pub unsafe fn addreturn(interpreter: *mut Interpreter) -> Status {
    unsafe {
        let line: *const i8 = lua_tolstring(interpreter, -1, null_mut());
        let retline: *const i8 = lua_pushfstring(interpreter, c"return %s;".as_ptr(), line);
        let status = lual_loadbufferx(interpreter, retline, strlen(retline) as usize, c"=stdin".as_ptr(), null());
        if status == Status::OK {
            lua_rotate(interpreter, -2, -1);
            lua_settop(interpreter, -2);
        } else {
            lua_settop(interpreter, -2 - 1);
        }
        return status;
    }
}
pub unsafe fn multiline(interpreter: *mut Interpreter) -> Status {
    unsafe {
        loop {
            let mut length: usize = 0;
            let line: *const i8 = lua_tolstring(interpreter, 1, &mut length);
            let status = lual_loadbufferx(interpreter, line, length, c"=stdin".as_ptr(), null());
            if incomplete(interpreter, status) == 0 || !pushline(interpreter, 0) {
                return status;
            }
            lua_rotate(interpreter, -2, -1);
            lua_settop(interpreter, -2);
            lua_pushstring(interpreter, c"\n".as_ptr());
            lua_rotate(interpreter, -2, 1);
            lua_concat(interpreter, 3);
        }
    }
}
pub unsafe fn loadline(interpreter: *mut Interpreter) -> Status {
    unsafe {
        lua_settop(interpreter, 0);
        if !pushline(interpreter, 1) {
            return Status::Closing;
        }
        let mut status = addreturn(interpreter);
        if status != Status::OK {
            status = multiline(interpreter);
        }
        lua_rotate(interpreter, 1, -1);
        lua_settop(interpreter, -2);
        return status;
    }
}
pub unsafe fn finishpcall(interpreter: *mut Interpreter, status: Status, extra: i64) -> i32 {
    unsafe {
        match status {
            Status::OK | Status::Yield => (*interpreter).get_top() - extra as i32,
            _ => {
                (*interpreter).push_boolean(false);
                lua_pushvalue(interpreter, -2);
                2
            },
        }
    }
}
pub unsafe fn luab_pcall(interpreter: *mut Interpreter) -> i32 {
    unsafe {
        lual_checkany(interpreter, 1);
        (*interpreter).push_boolean(true);
        lua_rotate(interpreter, 1, 1);
        let status = CallS::api_call(interpreter, (*interpreter).get_top() - 2, -1, 0, 0, Some(finishpcall as unsafe fn(*mut Interpreter, Status, i64) -> i32));
        return finishpcall(interpreter, status, 0);
    }
}
pub unsafe fn checkstack(interpreter: *mut Interpreter, other_state: *mut Interpreter, n: i32) {
    unsafe {
        if ((interpreter != other_state && lua_checkstack(other_state, n) == 0) as i32 != 0) as i64 != 0 {
            lual_error(interpreter, c"stack overflow".as_ptr());
        }
    }
}
pub unsafe fn getthread(interpreter: *mut Interpreter, arg: *mut i32) -> *mut Interpreter {
    unsafe {
        if lua_type(interpreter, 1) == Some(TagType::Interpreter) {
            *arg = 1;
            return lua_tothread(interpreter, 1);
        } else {
            *arg = 0;
            return interpreter;
        };
    }
}
pub unsafe fn settabss(interpreter: *mut Interpreter, k: *const i8, v: *const i8) {
    unsafe {
        lua_pushstring(interpreter, v);
        lua_setfield(interpreter, -2, k);
    }
}
pub unsafe fn settabsi(interpreter: *mut Interpreter, k: *const i8, v: i32) {
    unsafe {
        (*interpreter).push_integer(v as i64);
        lua_setfield(interpreter, -2, k);
    }
}
pub unsafe fn settabsb(interpreter: *mut Interpreter, k: *const i8, v: i32) {
    unsafe {
        (*interpreter).push_boolean(v != 0);
        lua_setfield(interpreter, -2, k);
    }
}
pub unsafe fn treatstackoption(interpreter: *mut Interpreter, other_state: *mut Interpreter, fname: *const i8) {
    unsafe {
        if interpreter == other_state {
            lua_rotate(interpreter, -2, 1);
        } else {
            lua_xmove(other_state, interpreter, 1);
        }
        lua_setfield(interpreter, -2, fname);
    }
}
pub unsafe fn auxupvalue(interpreter: *mut Interpreter, get: i32) -> i32 {
    unsafe {
        let n: i32 = lual_checkinteger(interpreter, 2) as i32;
        (*interpreter).lual_checktype(1, TagType::Closure);
        let name: *const i8 = if get != 0 { lua_getupvalue(interpreter, 1, n) } else { lua_setupvalue(interpreter, 1, n) };
        if name.is_null() {
            return 0;
        } else {
            lua_pushstring(interpreter, name);
            lua_rotate(interpreter, -(get + 1), 1);
            return get + 1;
        }
    }
}
pub unsafe fn checkupval(interpreter: *mut Interpreter, argf: i32, argnup: i32, pnup: *mut i32) -> *mut libc::c_void {
    unsafe {
        let id: *mut libc::c_void;
        let count_upvalues: i32 = lual_checkinteger(interpreter, argnup) as i32;
        (*interpreter).lual_checktype(argf, TagType::Closure);
        id = lua_upvalueid(interpreter, argf, count_upvalues);
        if !pnup.is_null() {
            if id.is_null() {
                lual_argerror(interpreter, argnup, c"invalid upvalue index".as_ptr());
            }
            *pnup = count_upvalues;
        }
        return id;
    }
}
pub unsafe fn makemask(smask: *const i8, count: i32) -> i32 {
    unsafe {
        let mut mask: i32 = 0;
        if !(strchr(smask, Character::LowerC as i32)).is_null() {
            mask |= 1 << 0;
        }
        if !(strchr(smask, Character::LowerR as i32)).is_null() {
            mask |= 1 << 1;
        }
        if !(strchr(smask, Character::LowerL as i32)).is_null() {
            mask |= 1 << 2;
        }
        if count > 0 {
            mask |= 1 << 3;
        }
        return mask;
    }
}
pub unsafe fn unmakemask(mask: i32, smask: *mut i8) -> *mut i8 {
    unsafe {
        let mut i: i32 = 0;
        if mask & 1 << 0 != 0 {
            let fresh190 = i;
            i = i + 1;
            *smask.offset(fresh190 as isize) = Character::LowerC as i8;
        }
        if mask & 1 << 1 != 0 {
            let fresh191 = i;
            i = i + 1;
            *smask.offset(fresh191 as isize) = Character::LowerR as i8;
        }
        if mask & 1 << 2 != 0 {
            let fresh192 = i;
            i = i + 1;
            *smask.offset(fresh192 as isize) = Character::LowerL as i8;
        }
        *smask.offset(i as isize) = Character::Null as i8;
        return smask;
    }
}
pub const HOOKKEY: *const i8 = c"_HOOKKEY".as_ptr();
