use crate::buffer::*;
use crate::bufffs::*;
use crate::callinfo::*;
use crate::calls::*;
use crate::character::*;
use crate::closeprotected::*;
use crate::closure::*;
use crate::debuginfo::*;
use crate::dynamicdata::*;
use crate::f2i::*;
use crate::forloop::*;
use crate::functions::*;
pub use crate::functionstate::*;
use crate::global::*;
use crate::lexicalstate::*;
use crate::loadf::*;
use crate::longjump::LuaError;
use crate::longjump::*;
use crate::object::*;
use crate::objectwithgclist::*;
use crate::opcode::*;
use crate::operatorbinary::*;
use crate::opmode::*;
use crate::prototype::*;
use crate::registeredfunction::*;
use crate::signalaction::*;
use crate::sparser::*;
use crate::status::*;
use crate::stkidrel::*;
use crate::table::*;
use crate::tagtype::*;
use crate::tagvariant::*;
use crate::tdefaultnew::*;
use crate::tm::*;
use crate::tobject::*;
use crate::tobjectwithgclist::*;
use crate::tobjectwithmetatable::*;
use crate::token::*;
use crate::tstring::*;
use crate::tvalue::*;
use crate::upvalue::*;
use crate::upvaluedescription::*;
use crate::utility::*;
use crate::vectort::*;
use crate::zio::*;
use std::cell::RefCell;
use std::io::Write;
use std::ptr::*;

const TFOR_CALL: usize = 0;
const TFOR_LOOP: usize = 1;
const MAX_CCMT: u32 = 0xF;
const HOOKEVENT_CALL: i32 = 0;
const HOOKEVENT_RET: i32 = 1;
const HOOKEVENT_LINE: i32 = 2;
const HOOKEVENT_COUNT: i32 = 3;
const CALLSTATUS_RECST_SHIFT: i32 = 10;
const CALLSTATUS_RECST_MASK: i32 = 7;
const EXTRA_STACK: i32 = 5;
const BASIC_STACK_SIZE: i32 = 2 * LUA_MINSTACK;
const MAXDELTA: usize = u16::MAX as usize;

pub fn get_errno() -> i32 {
    std::io::Error::last_os_error().raw_os_error().unwrap_or(0)
}

pub fn set_errno(code: i32) {
    #[cfg(target_os = "macos")]
    {
        unsafe extern "C" {
            fn __error() -> *mut i32;
        }
        unsafe {
            *__error() = code;
        }
    }
    #[cfg(not(target_os = "macos"))]
    {
        unsafe extern "C" {
            fn __errno_location() -> *mut i32;
        }
        unsafe {
            *__errno_location() = code;
        }
    }
}

const INTERPRETER_NY: u32 = 0x10000;
const INTERPRETER_NY_MASK: u32 = 0xFFFF0000;
const INTERPRETER_C_CALLS_MASK: u32 = 0xFFFF;
type InterpreterSuper = ObjectWithGCList;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct State {
    pub interpreter_super: InterpreterSuper,
    pub interpreter_status: Status,
    pub interpreter_allow_hook: u8,
    pub interpreter_count_callinfo: u32,
    pub interpreter_top: StkIdRel,
    pub interpreter_global: *mut Global,
    pub interpreter_callinfo: *mut CallInfo,
    pub interpreter_stack_last: StkIdRel,
    pub interpreter_stack: StkIdRel,
    pub interpreter_open_upvalue: *mut UpValue,
    pub interpreter_tbclist: StkIdRel,
    pub interpreter_twups: *mut State,
    pub interpreter_longjump: *mut LongJump,
    pub interpreter_base_callinfo: CallInfo,
    pub interpreter_hook: HookFunction,
    pub interpreter_error_function: i64,
    interpreter_count_c_calls: u32,
    interpreter_count_yield: u32,
    pub interpreter_old_program_counter: i32,
    pub interpreter_base_hookcount: i32,
    pub interpreter_hook_count: i32,
    pub interpreter_hook_mask: i32,
}
impl TObject for State {
    fn as_object(&self) -> &Object {
        self.interpreter_super.as_object()
    }
    fn as_object_mut(&mut self) -> &mut Object {
        self.interpreter_super.as_object_mut()
    }
}
impl TObjectWithGCList for State {
    fn getgclist(&mut self) -> *mut *mut ObjectWithGCList {
        self.interpreter_super.getgclist()
    }
}
impl State {
    pub fn decrement_c_stack(&mut self) {
        self.interpreter_count_c_calls -= 1;
    }
    pub fn increment_noyield(&mut self) {
        self.interpreter_count_c_calls += INTERPRETER_NY;
        self.interpreter_count_yield += 1;
    }
    pub fn decrement_noyield(&mut self) {
        self.interpreter_count_c_calls -= INTERPRETER_NY;
        self.interpreter_count_yield -= 1;
    }
    pub unsafe fn safereallocate(
        &mut self,
        oldblock: *mut std::ffi::c_void,
        oldsize: usize,
        newsize: usize,
    ) -> *mut std::ffi::c_void {
        unsafe {
            let newblock = self.reallocate(oldblock, oldsize, newsize);
            if newblock.is_null() && newsize > 0 {
                luad_throw(self, Status::MemoryError);
            }
            newblock
        }
    }
    pub unsafe fn allocate(&mut self, newsize: usize) -> *mut std::ffi::c_void {
        unsafe { (*self.interpreter_global).allocate(self, newsize) }
    }
    pub unsafe fn reallocate(&mut self, oldblock: *mut std::ffi::c_void, oldsize: usize, newsize: usize) -> *mut std::ffi::c_void {
        unsafe { (*self.interpreter_global).reallocate(self, oldblock, oldsize, newsize) }
    }
    pub fn should_step(&self) -> bool {
        unsafe { (*self.interpreter_global).should_step() }
    }
    pub fn do_gc_step_if_should_step(&mut self) {
        if self.should_step() {
            unsafe {
                self.do_gc_step();
            }
        }
    }
    pub unsafe fn preinit_thread(&mut self, global: *mut Global) {
        unsafe {
            self.interpreter_global = global;
            self.interpreter_stack.stkidrel_pointer = null_mut();
            self.interpreter_callinfo = null_mut();
            self.interpreter_count_callinfo = 0;
            self.interpreter_twups = self as *mut State;
            self.interpreter_count_c_calls = 0;
            self.interpreter_count_yield = 0;
            self.interpreter_longjump = null_mut();
            write_volatile(&mut self.interpreter_hook as *mut HookFunction, None);
            write_volatile(&mut self.interpreter_hook_mask as *mut i32, 0);
            self.interpreter_base_hookcount = 0;
            self.interpreter_allow_hook = 1;
            self.interpreter_hook_count = self.interpreter_base_hookcount;
            self.interpreter_open_upvalue = null_mut();
            self.interpreter_status = Status::OK;
            self.interpreter_error_function = 0;
            self.interpreter_old_program_counter = 0;
        }
    }
    pub fn initialize(&mut self, global: &Global) {
        self.set_tagvariant(TagVariant::State);
        self.set_marked(global.global_current_white & WHITEBITS);
    }
    pub unsafe fn lua_callk(&mut self, nargs: i32, count_results: i32, ctx: i64, k: ContextFunction) {
        unsafe {
            let function: *mut TValue = self
                .interpreter_top
                .stkidrel_pointer
                .sub((nargs + 1) as usize);
            if k.is_some() && self.is_yieldable() {
                (*self.interpreter_callinfo).callinfo_u.c.context_function = k;
                (*self.interpreter_callinfo).callinfo_u.c.context = ctx;
                ccall(self, function, count_results, 1);
            } else {
                luad_callnoyield(self, function, count_results);
            }
            if count_results <= -1
                && (*self.interpreter_callinfo).callinfo_top.stkidrel_pointer < self.interpreter_top.stkidrel_pointer
            {
                (*self.interpreter_callinfo).callinfo_top.stkidrel_pointer = self.interpreter_top.stkidrel_pointer;
            }
        }
    }
    pub unsafe fn lual_checktype(&mut self, arg: i32, tagtype: TagType) {
        unsafe {
            if lua_type(self, arg) != Some(tagtype) {
                tag_error2(self, arg, Some(tagtype));
            }
        }
    }
    pub unsafe fn do_gc_step(&mut self) {
        unsafe {
            (*self.interpreter_global).do_gc_step(self);
        }
    }
    pub unsafe fn luac_fullgc(&mut self, is_emergency: bool) {
        unsafe {
            (*self.interpreter_global).luac_fullgc(self, is_emergency);
        }
    }
    pub unsafe fn luas_init_state(&mut self) {
        unsafe {
            (*self.interpreter_global).luas_init_global(self);
        }
    }
    pub unsafe fn to_pointer(&mut self, index: i32) -> *mut std::ffi::c_void {
        unsafe { self.index_to_value(index).to_pointer() }
    }
    pub unsafe fn interpreter_free(&mut self, state: *mut State) {
        unsafe {
            luaf_closeupval(self, self.interpreter_stack.stkidrel_pointer);
            freestack(self);
            (*state).free_memory(
                self as *mut State as *mut std::ffi::c_void,
                size_of::<State>(),
            );
        }
    }
    pub fn get_status(&mut self) -> Status {
        self.interpreter_status
    }
    pub unsafe fn set_error_object(&mut self, status: Status, old_top: *mut TValue) {
        unsafe {
            match status {
                Status::MemoryError => {
                    let io: *mut TValue = &mut (*old_top);
                    let tstring: *mut TString = (*(self.interpreter_global)).global_memoryerrormessage;
                    (*io).set_object(tstring as *mut Object, (*tstring).get_tagvariant());
                }
                Status::OK => {
                    (*old_top).tvalue_set_tag_variant(TagVariant::NilNil);
                }
                _ => {
                    let io1: *mut TValue = &mut (*old_top);
                    let io2: *const TValue = &mut (*(self.interpreter_top.stkidrel_pointer).sub(1));
                    (*io1).copy_from(&*io2);
                }
            }
            self.interpreter_top.stkidrel_pointer = old_top.add(1);
        }
    }
    pub unsafe fn correct_stack(&mut self) {
        unsafe {
            self.interpreter_top.stkidrel_pointer = (self.interpreter_stack.stkidrel_pointer as *mut i8)
                .add(self.interpreter_top.stkidrel_offset as usize)
                as *mut TValue;
            self.interpreter_tbclist.stkidrel_pointer = (self.interpreter_stack.stkidrel_pointer as *mut i8)
                .add(self.interpreter_tbclist.stkidrel_offset as usize)
                as *mut TValue;
            let mut upvalue: *mut UpValue = self.interpreter_open_upvalue;
            while !upvalue.is_null() {
                (*upvalue).upvalue_v.upvaluea_p = &mut *((self.interpreter_stack.stkidrel_pointer as *mut i8)
                    .add((*upvalue).upvalue_v.upvaluea_offset as usize)
                    as *mut TValue);
                upvalue = (*upvalue).upvalue_u.upvalueb_open.upvalueba_next;
            }
            let mut callinfo = self.interpreter_callinfo;
            while !callinfo.is_null() {
                (*callinfo).callinfo_top.stkidrel_pointer = (self.interpreter_stack.stkidrel_pointer as *mut i8)
                    .add((*callinfo).callinfo_top.stkidrel_offset as usize)
                    as *mut TValue;
                (*callinfo).callinfo_function.stkidrel_pointer = (self.interpreter_stack.stkidrel_pointer as *mut i8)
                    .add((*callinfo).callinfo_function.stkidrel_offset as usize)
                    as *mut TValue;
                if (*callinfo).callinfo_callstatus as i32 & CALLSTATUS_LUA == 0 {
                    write_volatile(&mut (*callinfo).callinfo_u.l.trap as *mut i32, 1i32);
                }
                callinfo = (*callinfo).callinfo_previous;
            }
        }
    }
    pub fn is_yieldable(&mut self) -> bool {
        0 == self.interpreter_count_c_calls & INTERPRETER_NY_MASK
        //0 == self.interpreter_countyield
    }
    pub unsafe fn push_boolean(&mut self, x: bool) {
        unsafe {
            if x {
                (*self.interpreter_top.stkidrel_pointer).tvalue_set_tag_variant(TagVariant::BooleanTrue);
            } else {
                (*self.interpreter_top.stkidrel_pointer).tvalue_set_tag_variant(TagVariant::BooleanFalse);
            }
            self.interpreter_top.stkidrel_pointer = self.interpreter_top.stkidrel_pointer.add(1);
        }
    }
    pub unsafe fn push_integer(&mut self, x: i64) {
        unsafe {
            let tvalue: *mut TValue = &mut (*self.interpreter_top.stkidrel_pointer);
            (*tvalue).set_integer(x);
            self.interpreter_top.stkidrel_pointer = self.interpreter_top.stkidrel_pointer.add(1);
        }
    }
    pub unsafe fn push_nil(&mut self) {
        unsafe {
            (*self.interpreter_top.stkidrel_pointer).tvalue_set_tag_variant(TagVariant::NilNil);
            self.interpreter_top.stkidrel_pointer = self.interpreter_top.stkidrel_pointer.add(1);
        }
    }
    pub unsafe fn push_number(&mut self, x: f64) {
        unsafe {
            let tvalue: *mut TValue = &mut (*self.interpreter_top.stkidrel_pointer);
            (*tvalue).set_number(x);
            self.interpreter_top.stkidrel_pointer = self.interpreter_top.stkidrel_pointer.add(1);
        }
    }
    pub unsafe fn get_top(&mut self) -> i32 {
        unsafe {
            self.interpreter_top.stkidrel_pointer.offset_from(
                ((*self.interpreter_callinfo)
                    .callinfo_function
                    .stkidrel_pointer)
                    .add(1),
            ) as i32
        }
    }
    pub unsafe fn find_pcall(&mut self) -> *mut CallInfo {
        unsafe {
            let mut it = self.interpreter_callinfo;
            loop {
                if it.is_null() {
                    break it;
                } else if ((*it).callinfo_callstatus & (CALLSTATUS_YPCALL as u16)) != 0 {
                    break it;
                } else {
                    it = (*it).callinfo_previous;
                }
            }
        }
    }
    pub unsafe fn sweep_list(&mut self, mut p: *mut *mut Object, countin: i32, countout: *mut i32) -> *mut *mut Object {
        unsafe {
            let other_white = (*(self.interpreter_global)).global_current_white ^ WHITEBITS;
            let mut i: i32;
            let white = (*(self.interpreter_global)).global_current_white & WHITEBITS;
            i = 0;
            while !(*p).is_null() && i < countin {
                let head: *mut Object = *p;
                let marked = (*head).get_marked();
                if marked & other_white != 0 {
                    *p = (*head).object_next;
                    Object::object_free(self, head);
                } else {
                    (*head).set_marked(marked & !(BLACKBIT | WHITEBITS | AGEBITS) | white);
                    p = &mut (*head).object_next;
                }
                i += 1;
            }
            if !countout.is_null() {
                *countout = i;
            }
            if (*p).is_null() {
                null_mut()
            } else {
                p
            }
        }
    }
    pub unsafe fn free_memory(&mut self, block: *mut std::ffi::c_void, oldsize: usize) {
        unsafe {
            (*self.interpreter_global).free_memory(block, oldsize);
        }
    }
    pub unsafe fn too_big(&mut self) -> ! {
        unsafe {
            luag_runerror(
                self,
                c"memory allocation error: block too big".as_ptr(),
                &[],
            );
        }
    }
    pub unsafe fn push_state(&mut self) -> bool {
        unsafe {
            let io: *mut TValue = &mut (*self.interpreter_top.stkidrel_pointer);
            (*io).set_object(self as *mut State as *mut Object, TagVariant::State);
            self.interpreter_top.stkidrel_pointer = self.interpreter_top.stkidrel_pointer.add(1);
            (*self.interpreter_global).global_maininterpreter == self
        }
    }
    pub unsafe fn relstack(&mut self) {
        unsafe {
            self.interpreter_top.stkidrel_offset = (self.interpreter_top.stkidrel_pointer as *mut i8)
                .offset_from(self.interpreter_stack.stkidrel_pointer as *mut i8)
                as i64;
            self.interpreter_tbclist.stkidrel_offset = (self.interpreter_tbclist.stkidrel_pointer as *mut i8)
                .offset_from(self.interpreter_stack.stkidrel_pointer as *mut i8)
                as i64;
            let mut upvalue: *mut UpValue = self.interpreter_open_upvalue;
            while !upvalue.is_null() {
                (*upvalue).upvalue_v.upvaluea_offset = ((*upvalue).upvalue_v.upvaluea_p as *mut TValue as *mut i8)
                    .offset_from(self.interpreter_stack.stkidrel_pointer as *mut i8)
                    as i64;
                upvalue = (*upvalue).upvalue_u.upvalueb_open.upvalueba_next;
            }
            let mut callinfo = self.interpreter_callinfo;
            while !callinfo.is_null() {
                (*callinfo).callinfo_top.stkidrel_offset = ((*callinfo).callinfo_top.stkidrel_pointer as *mut i8)
                    .offset_from(self.interpreter_stack.stkidrel_pointer as *mut i8)
                    as i64;
                (*callinfo).callinfo_function.stkidrel_offset = ((*callinfo).callinfo_function.stkidrel_pointer as *mut i8)
                    .offset_from(self.interpreter_stack.stkidrel_pointer as *mut i8)
                    as i64;
                callinfo = (*callinfo).callinfo_previous;
            }
        }
    }
    pub unsafe fn luad_errerr(&mut self) -> ! {
        unsafe {
            let message: *mut TString = luas_newlstr(self, c"error in error handling".as_ptr(), 23);
            let io: *mut TValue = &mut (*self.interpreter_top.stkidrel_pointer);
            (*io).set_object(message as *mut Object, (*message).get_tagvariant());
            self.interpreter_top.stkidrel_pointer = self.interpreter_top.stkidrel_pointer.add(1);
            luad_throw(self, Status::GenericError);
        }
    }
    pub unsafe fn luae_checkcstack(&mut self) {
        unsafe {
            if self.interpreter_count_c_calls & INTERPRETER_C_CALLS_MASK == LUAI_MAXCCALLS {
                luag_runerror(self, c"C stack overflow".as_ptr(), &[]);
            } else if self.interpreter_count_c_calls & INTERPRETER_C_CALLS_MASK >= LUAI_MAXCCALLS_ERRERR {
                self.luad_errerr();
            }
        }
    }
    pub unsafe fn increment_c_stack(&mut self) {
        unsafe {
            self.interpreter_count_c_calls += 1;
            if self.interpreter_count_c_calls & INTERPRETER_C_CALLS_MASK >= LUAI_MAXCCALLS {
                self.luae_checkcstack();
            }
        }
    }
    pub unsafe fn stackinuse(&mut self) -> i32 {
        unsafe {
            let mut lim = self.interpreter_top.stkidrel_pointer;
            let mut callinfo = self.interpreter_callinfo;
            while !callinfo.is_null() {
                if lim < (*callinfo).callinfo_top.stkidrel_pointer {
                    lim = (*callinfo).callinfo_top.stkidrel_pointer;
                }
                callinfo = (*callinfo).callinfo_previous;
            }
            let mut res: i32 = lim.offset_from(self.interpreter_stack.stkidrel_pointer) as i32 + 1;
            if res < LUA_MINSTACK {
                res = LUA_MINSTACK;
            }
            res
        }
    }
    pub unsafe fn luad_shrinkstack(&mut self) {
        unsafe {
            let inuse: i32 = self.stackinuse();
            let max: i32 = if inuse > LUAI_MAXSTACK / 3 {
                LUAI_MAXSTACK
            } else {
                inuse * 3
            };
            if inuse <= LUAI_MAXSTACK
                && (self.interpreter_stack_last.stkidrel_pointer).offset_from(self.interpreter_stack.stkidrel_pointer) as i32 > max
            {
                let newsize: i32 = if inuse > LUAI_MAXSTACK / 2 {
                    LUAI_MAXSTACK
                } else {
                    inuse * 2
                };
                luad_reallocstack(self, newsize, false);
            }
            luae_shrinkci(self);
        }
    }
    pub unsafe fn luad_inctop(&mut self) {
        unsafe {
            if (self.interpreter_stack_last.stkidrel_pointer).offset_from(self.interpreter_top.stkidrel_pointer) <= 1 {
                luad_growstack(self, 1, true);
            }
            self.interpreter_top.stkidrel_pointer = self.interpreter_top.stkidrel_pointer.add(1);
        }
    }
    pub unsafe fn lua_createtable(&mut self) {
        unsafe {
            let table: *mut Table = luah_new(self);
            let io: *mut TValue = &mut (*self.interpreter_top.stkidrel_pointer);
            (*io).set_table(table);
            self.interpreter_top.stkidrel_pointer = self.interpreter_top.stkidrel_pointer.add(1);
            self.do_gc_step_if_should_step();
        }
    }
    pub unsafe fn lua_getmetatable(&mut self, object_index: i32) -> bool {
        unsafe {
            let object: *const TValue = self.index_to_value(object_index);
            let metatable: *mut Table = match (*object).get_tagvariant().to_tag_type() {
                TagType::Table => (*(*object).as_table().unwrap()).get_metatable(),
                TagType::User => (*(*object).as_user().unwrap()).get_metatable(),
                _ => (*self.interpreter_global).global_metatables[(*object).get_tagvariant().to_tag_type() as usize],
            };
            if metatable.is_null() {
                false
            } else {
                let io: *mut TValue = &mut (*self.interpreter_top.stkidrel_pointer);
                (*io).set_table(metatable);
                self.interpreter_top.stkidrel_pointer = self.interpreter_top.stkidrel_pointer.add(1);
                true
            }
        }
    }
    pub unsafe fn lua_getiuservalue(&mut self, index: i32, n: i32) -> Option<TagType> {
        unsafe {
            let t: Option<TagType>;
            let tvalue: *mut TValue = self.index_to_value(index);
            if n <= 0 || n > (*(*tvalue).as_user().unwrap()).user_countupvalues {
                (*self.interpreter_top.stkidrel_pointer).tvalue_set_tag_variant(TagVariant::NilNil);
                t = None;
            } else {
                let io1: *mut TValue = &mut (*self.interpreter_top.stkidrel_pointer);
                let io2: *const TValue = &mut (*((*(*tvalue).as_user().unwrap()).user_upvalues)
                    .as_mut_ptr()
                    .add((n - 1) as usize));
                (*io1).copy_from(&*io2);
                t = Some(
                    (*self.interpreter_top.stkidrel_pointer)
                        .get_tagvariant()
                        .to_tag_type(),
                );
            }
            self.interpreter_top.stkidrel_pointer = self.interpreter_top.stkidrel_pointer.add(1);
            t
        }
    }
    pub unsafe fn index_to_value(&mut self, mut index: i32) -> &mut TValue {
        unsafe {
            let callinfo = self.interpreter_callinfo;
            if index > 0 {
                let o: *mut TValue = ((*callinfo).callinfo_function.stkidrel_pointer).add(index as usize);
                if o >= self.interpreter_top.stkidrel_pointer {
                    &mut (*self.interpreter_global).global_nonevalue
                } else {
                    &mut (*o)
                }
            } else if index > LUA_REGISTRYINDEX {
                &mut (*self.interpreter_top.stkidrel_pointer.sub((-index) as usize))
            } else if index == LUA_REGISTRYINDEX {
                &mut (*self.interpreter_global).global_lregistry
            } else {
                index = LUA_REGISTRYINDEX - index;
                let value = *(*callinfo).callinfo_function.stkidrel_pointer;
                if value.is_collectable() && value.get_tagvariant() == TagVariant::ClosureC {
                    let function: *mut Closure = value.as_closure().unwrap();
                    if index <= (*function).closure_count_upvalues as i32 {
                        &mut *((*function).closure_upvalues)
                            .closureupvalue_tvalues
                            .as_mut_ptr()
                            .add((index - 1) as usize) as &mut TValue
                    } else {
                        &mut (*self.interpreter_global).global_nonevalue
                    }
                } else {
                    &mut (*self.interpreter_global).global_nonevalue
                }
            }
        }
    }
}
pub unsafe fn do_repl(state: *mut State) {
    unsafe {
        let mut status: Status;
        let oldprogname: *mut i8 = PROGRAM_NAME.load(Ordering::Relaxed);
        PROGRAM_NAME.store(null_mut(), Ordering::Relaxed);
        loop {
            status = loadline(state);
            if !(status != Status::Closing) {
                break;
            }
            if status == Status::OK {
                status = docall(state, 0, -1);
            }
            if status == Status::OK {
                l_print(state);
            } else {
                report(state, status);
            }
        }
        lua_settop(state, 0);
        println!();
        std::io::stdout().flush().unwrap();
        PROGRAM_NAME.store(oldprogname, Ordering::Relaxed);
    }
}
pub unsafe fn luad_throw(state: *mut State, status: Status) -> ! {
    unsafe {
        if !((*state).interpreter_longjump).is_null() {
            std::panic::panic_any(LuaError {
                status,
                unwind_to_base: false,
            });
        } else {
            let global: *mut Global = (*state).interpreter_global;
            let outerstatus = luae_resetthread(state, status);
            (*state).interpreter_status = outerstatus;
            if !((*(*global).global_maininterpreter).interpreter_longjump).is_null() {
                let top = (*(*global).global_maininterpreter)
                    .interpreter_top
                    .stkidrel_pointer;
                (*(*global).global_maininterpreter)
                    .interpreter_top
                    .stkidrel_pointer = ((*(*global).global_maininterpreter)
                    .interpreter_top
                    .stkidrel_pointer)
                    .add(1);
                let io1: *mut TValue = &mut (*top);
                let io2: *const TValue = &mut (*(*state).interpreter_top.stkidrel_pointer.sub(1));
                (*io1).copy_from(&*io2);
                luad_throw((*global).global_maininterpreter, outerstatus);
            } else {
                if ((*global).global_panic).is_some() {
                    ((*global).global_panic).expect("non-null function pointer")(state);
                }
                std::process::abort();
            }
        };
    }
}
std::thread_local! {
    static LUA_PROTECTED_DEPTH: std::cell::Cell<u32> = const { std::cell::Cell::new(0) };
}
pub fn in_lua_protected_context() -> bool {
    LUA_PROTECTED_DEPTH.with(|d| d.get() > 0)
}
pub unsafe fn luad_rawrunprotected(state: *mut State, f: ProtectedFunction, arbitrary_data: *mut std::ffi::c_void) -> Status {
    unsafe {
        let oldcountccalls = (*state).interpreter_count_c_calls;
        let oldcountyield = (*state).interpreter_count_yield;
        let mut long_jump = LongJump::new();
        long_jump.longjump_previous = (*state).interpreter_longjump;
        (*state).interpreter_longjump = &mut long_jump;
        let f_unwrapped = f.expect("non-null function pointer");
        LUA_PROTECTED_DEPTH.with(|d| d.set(d.get() + 1));
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            f_unwrapped(state, arbitrary_data);
        }));
        LUA_PROTECTED_DEPTH.with(|d| d.set(d.get() - 1));
        (*state).interpreter_longjump = long_jump.longjump_previous;
        (*state).interpreter_count_yield = oldcountyield;
        (*state).interpreter_count_c_calls = oldcountccalls;
        match result {
            Ok(()) => Status::OK,
            Err(payload) => match payload.downcast::<LuaError>() {
                Ok(lua_error) => {
                    if lua_error.unwind_to_base && !long_jump.longjump_previous.is_null() {
                        std::panic::resume_unwind(Box::new(*lua_error));
                    }
                    lua_error.status
                }
                Err(other) => std::panic::resume_unwind(other),
            },
        }
    }
}
pub unsafe fn luad_reallocstack(state: *mut State, newsize: i32, should_raise_error: bool) -> i32 {
    unsafe {
        let oldsize: i32 =
            ((*state).interpreter_stack_last.stkidrel_pointer).offset_from((*state).interpreter_stack.stkidrel_pointer) as i32;
        let oldgcstop: i32 = (*(*state).interpreter_global).global_gcstopem as i32;
        (*state).relstack();
        (*(*state).interpreter_global).global_gcstopem = 1;
        let newstack: *mut TValue = (*state).reallocate(
            (*state).interpreter_stack.stkidrel_pointer as *mut std::ffi::c_void,
            ((oldsize + EXTRA_STACK) as usize) * size_of::<TValue>(),
            ((newsize + EXTRA_STACK) as usize) * size_of::<TValue>(),
        ) as *mut TValue;
        (*(*state).interpreter_global).global_gcstopem = oldgcstop as u8;
        if newstack.is_null() {
            (*state).correct_stack();
            if should_raise_error {
                luad_throw(state, Status::MemoryError);
            } else {
                return 0;
            }
        }
        (*state).interpreter_stack.stkidrel_pointer = newstack;
        (*state).correct_stack();
        (*state).interpreter_stack_last.stkidrel_pointer = ((*state).interpreter_stack.stkidrel_pointer).add(newsize as usize);
        for i in (oldsize + EXTRA_STACK)..(newsize + EXTRA_STACK) {
            (*newstack.add(i as usize)).tvalue_set_tag_variant(TagVariant::NilNil);
        }
        1
    }
}
pub unsafe fn luad_growstack(state: *mut State, n: i32, should_raise_error: bool) -> i32 {
    unsafe {
        let size: i32 =
            ((*state).interpreter_stack_last.stkidrel_pointer).offset_from((*state).interpreter_stack.stkidrel_pointer) as i32;
        if size > LUAI_MAXSTACK {
            if should_raise_error {
                (*state).luad_errerr();
            }
            return 0;
        } else if n < LUAI_MAXSTACK {
            let mut newsize: i32 = 2 * size;
            let needed: i32 =
                ((*state).interpreter_top.stkidrel_pointer).offset_from((*state).interpreter_stack.stkidrel_pointer) as i32 + n;
            if newsize > LUAI_MAXSTACK {
                newsize = LUAI_MAXSTACK;
            }
            if newsize < needed {
                newsize = needed;
            }
            if newsize <= LUAI_MAXSTACK {
                return luad_reallocstack(state, newsize, should_raise_error);
            }
        }
        luad_reallocstack(
            state,
            LUAI_MAXSTACK + LUAI_MAXCCALLS as i32,
            should_raise_error,
        );
        if should_raise_error {
            luag_runerror(state, c"stack overflow".as_ptr(), &[]);
        }
        0
    }
}
pub unsafe fn luad_hook(state: *mut State, event: i32, line: i32, ftransfer: i32, ntransfer: i32) {
    unsafe {
        let hook: HookFunction = (*state).interpreter_hook;
        if hook.is_some() && (*state).interpreter_allow_hook as i32 != 0 {
            let mut mask: i32 = CALLSTATUS_FRESH;
            let callinfo = (*state).interpreter_callinfo;
            let top: i64 = ((*state).interpreter_top.stkidrel_pointer as *mut i8)
                .offset_from((*state).interpreter_stack.stkidrel_pointer as *mut i8) as i64;
            let ci_top: i64 = ((*callinfo).callinfo_top.stkidrel_pointer as *mut i8)
                .offset_from((*state).interpreter_stack.stkidrel_pointer as *mut i8) as i64;
            let mut debuginfo: DebugInfo = DebugInfo::new2(event, line, callinfo);
            if ntransfer != 0 {
                mask |= CALLSTATUS_FIN;
                (*callinfo)
                    .callinfo_u2
                    .callinfoconstituentb_transferinfo
                    .callinfoconsistuentbtransferinfo_ftransfer = ftransfer as u16;
                (*callinfo)
                    .callinfo_u2
                    .callinfoconstituentb_transferinfo
                    .callinfoconsistuentbtransferinfo_ntransfer = ntransfer as u16;
            }
            if (*callinfo).callinfo_callstatus as i32 & CALLSTATUS_LUA == 0
                && (*state).interpreter_top.stkidrel_pointer < (*callinfo).callinfo_top.stkidrel_pointer
            {
                (*state).interpreter_top.stkidrel_pointer = (*callinfo).callinfo_top.stkidrel_pointer;
            }
            if ((*state).interpreter_stack_last.stkidrel_pointer).offset_from((*state).interpreter_top.stkidrel_pointer)
                <= LUA_MINSTACK as isize
            {
                luad_growstack(state, LUA_MINSTACK, true);
            }
            if (*callinfo).callinfo_top.stkidrel_pointer
                < (*state)
                    .interpreter_top
                    .stkidrel_pointer
                    .add(LUA_MINSTACK as usize)
            {
                (*callinfo).callinfo_top.stkidrel_pointer = (*state)
                    .interpreter_top
                    .stkidrel_pointer
                    .add(LUA_MINSTACK as usize);
            }
            (*state).interpreter_allow_hook = 0;
            (*callinfo).callinfo_callstatus = ((*callinfo).callinfo_callstatus as i32 | mask) as u16;
            hook.expect("non-null function pointer")(state, &mut debuginfo);
            (*state).interpreter_allow_hook = 1;
            (*callinfo).callinfo_top.stkidrel_pointer =
                ((*state).interpreter_stack.stkidrel_pointer as *mut i8).add(ci_top as usize) as *mut TValue;
            (*state).interpreter_top.stkidrel_pointer =
                ((*state).interpreter_stack.stkidrel_pointer as *mut i8).add(top as usize) as *mut TValue;
            (*callinfo).callinfo_callstatus = ((*callinfo).callinfo_callstatus as i32 & !mask) as u16;
        }
    }
}
pub unsafe fn luad_hookcall(state: *mut State, callinfo: *mut CallInfo) {
    unsafe {
        (*state).interpreter_old_program_counter = 0;
        if (*state).interpreter_hook_mask & HOOKMASK_CALL != 0 {
            let event: i32 = if ((*callinfo).callinfo_callstatus & (CALLSTATUS_TAIL as u16)) != 0 {
                4
            } else {
                0
            };
            let p: *mut Prototype = (*(*(*callinfo).callinfo_function.stkidrel_pointer)
                .as_closure()
                .unwrap())
            .closure_payload
            .closurepayload_lprototype;
            (*callinfo).callinfo_u.l.saved_program_counter = ((*callinfo).callinfo_u.l.saved_program_counter).add(1);
            (*callinfo).callinfo_u.l.saved_program_counter;
            luad_hook(state, event, -1, 1, (*p).prototype_countparameters as i32);
            (*callinfo).callinfo_u.l.saved_program_counter = ((*callinfo).callinfo_u.l.saved_program_counter).sub(1);
            (*callinfo).callinfo_u.l.saved_program_counter;
        }
    }
}
pub unsafe fn rethook(state: *mut State, mut callinfo: *mut CallInfo, nres: i32) {
    unsafe {
        if (*state).interpreter_hook_mask & HOOKMASK_RET != 0 {
            let firstres: *mut TValue = (*state).interpreter_top.stkidrel_pointer.sub(nres as usize);
            let mut delta: i32 = 0;
            if (*callinfo).callinfo_callstatus as i32 & CALLSTATUS_LUA == 0 {
                let p: *mut Prototype = (*(*(*callinfo).callinfo_function.stkidrel_pointer)
                    .as_closure()
                    .unwrap())
                .closure_payload
                .closurepayload_lprototype;
                if (*p).prototype_isvariablearguments {
                    delta = (*callinfo).callinfo_u.l.count_extra_arguments + (*p).prototype_countparameters as i32 + 1;
                }
            }
            (*callinfo).callinfo_function.stkidrel_pointer = ((*callinfo).callinfo_function.stkidrel_pointer).add(delta as usize);
            let ftransfer: i32 = firstres.offset_from((*callinfo).callinfo_function.stkidrel_pointer) as i32;
            luad_hook(state, HOOKEVENT_RET, -1, ftransfer, nres);
            (*callinfo).callinfo_function.stkidrel_pointer = ((*callinfo).callinfo_function.stkidrel_pointer).sub(delta as usize);
        }
        callinfo = (*callinfo).callinfo_previous;
        if (*callinfo).callinfo_callstatus as i32 & CALLSTATUS_LUA == 0 {
            (*state).interpreter_old_program_counter = ((*callinfo).callinfo_u.l.saved_program_counter).offset_from(
                (*(*(*(*callinfo).callinfo_function.stkidrel_pointer)
                    .as_closure()
                    .unwrap())
                .closure_payload
                .closurepayload_lprototype)
                    .prototype_code
                    .vectort_pointer,
            ) as i32
                - 1;
        }
    }
}
pub unsafe fn tryfunctm(state: *mut State, mut function: *mut TValue) -> *mut TValue {
    unsafe {
        let mut p: *mut TValue;
        if ((*state).interpreter_stack_last.stkidrel_pointer).offset_from((*state).interpreter_top.stkidrel_pointer) <= 1 {
            let t__: i64 = (function as *mut i8).offset_from((*state).interpreter_stack.stkidrel_pointer as *mut i8) as i64;
            (*state).do_gc_step_if_should_step();
            luad_growstack(state, 1, true);
            function = ((*state).interpreter_stack.stkidrel_pointer as *mut i8).add(t__ as usize) as *mut TValue;
        }
        let tm: *const TValue = luat_gettmbyobj(state, &(*function), TM_CALL);
        if (*tm).get_tagvariant().to_tag_type().is_nil() {
            luag_callerror(state, &(*function));
        }
        p = (*state).interpreter_top.stkidrel_pointer;
        while p > function {
            let io1: *mut TValue = &mut (*p);
            let io2: *const TValue = &mut (*p.sub(1));
            (*io1).copy_from(&*io2);
            p = p.sub(1);
        }
        (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.add(1);
        let io_dst: *mut TValue = &mut (*function);
        (*io_dst).copy_from(&*tm);
        function
    }
}
pub unsafe fn moveresults(state: *mut State, mut res: *mut TValue, mut nres: i32, mut wanted: i32) {
    unsafe {
        match wanted {
            0 => {
                (*state).interpreter_top.stkidrel_pointer = res;
                return;
            }
            1 => {
                if nres == 0 {
                    (*res).tvalue_set_tag_variant(TagVariant::NilNil);
                } else {
                    let io1: *mut TValue = &mut (*res);
                    let io2: *const TValue = &mut (*(*state).interpreter_top.stkidrel_pointer.sub(nres as usize));
                    (*io1).copy_from(&*io2);
                }
                (*state).interpreter_top.stkidrel_pointer = res.add(1);
                return;
            }
            -1 => {
                wanted = nres;
            }
            _ => {
                if wanted < -1 {
                    (*(*state).interpreter_callinfo).callinfo_callstatus =
                        ((*(*state).interpreter_callinfo).callinfo_callstatus as i32 | CALLSTATUS_TRAN) as u16;
                    (*(*state).interpreter_callinfo)
                        .callinfo_u2
                        .callinfoconstituentb_nres = nres;
                    res = luaf_close(state, res, Status::Closing, 1);
                    (*(*state).interpreter_callinfo).callinfo_callstatus =
                        ((*(*state).interpreter_callinfo).callinfo_callstatus as i32 & !CALLSTATUS_TRAN) as u16;
                    if (*state).interpreter_hook_mask != 0 {
                        let savedres: i64 =
                            (res as *mut i8).offset_from((*state).interpreter_stack.stkidrel_pointer as *mut i8) as i64;
                        rethook(state, (*state).interpreter_callinfo, nres);
                        res = ((*state).interpreter_stack.stkidrel_pointer as *mut i8).add(savedres as usize) as *mut TValue;
                    }
                    wanted = -wanted - 3;
                    if wanted == -1 {
                        wanted = nres;
                    }
                }
            }
        }
        let firstresult: *mut TValue = (*state).interpreter_top.stkidrel_pointer.sub(nres as usize);
        if nres > wanted {
            nres = wanted;
        }
        for i in 0..nres {
            let io_dst: *mut TValue = &mut (*res.add(i as usize));
            let io_src: *const TValue = &mut (*firstresult.add(i as usize));
            (*io_dst).copy_from(&*io_src);
        }
        for i in nres..wanted {
            (*res.add(i as usize)).tvalue_set_tag_variant(TagVariant::NilNil);
        }
        (*state).interpreter_top.stkidrel_pointer = res.add(wanted as usize);
    }
}
pub unsafe fn luad_poscall(state: *mut State, callinfo: *mut CallInfo, nres: i32) {
    unsafe {
        let wanted: i32 = (*callinfo).callinfo_count_results;
        if (*state).interpreter_hook_mask != 0 && (wanted >= -1) {
            rethook(state, callinfo, nres);
        }
        moveresults(
            state,
            (*callinfo).callinfo_function.stkidrel_pointer,
            nres,
            wanted,
        );
        (*state).interpreter_callinfo = (*callinfo).callinfo_previous;
    }
}
pub unsafe fn prepcallinfo(state: *mut State, function: *mut TValue, nret: i32, mask: i32, top: *mut TValue) -> *mut CallInfo {
    unsafe {
        (*state).interpreter_callinfo = if !((*(*state).interpreter_callinfo).callinfo_next).is_null() {
            (*(*state).interpreter_callinfo).callinfo_next
        } else {
            luae_extendci(state)
        };
        let callinfo = (*state).interpreter_callinfo;
        (*callinfo).callinfo_function.stkidrel_pointer = function;
        (*callinfo).callinfo_count_results = nret;
        (*callinfo).callinfo_callstatus = mask as u16;
        (*callinfo).callinfo_top.stkidrel_pointer = top;
        callinfo
    }
}
pub unsafe fn precallc(state: *mut State, mut function: *mut TValue, count_results: i32, cfunction: CFunction) -> i32 {
    unsafe {
        if ((*state).interpreter_stack_last.stkidrel_pointer).offset_from((*state).interpreter_top.stkidrel_pointer)
            <= LUA_MINSTACK as isize
        {
            let t__: i64 = (function as *mut i8).offset_from((*state).interpreter_stack.stkidrel_pointer as *mut i8) as i64;
            (*state).do_gc_step_if_should_step();
            luad_growstack(state, LUA_MINSTACK, true);
            function = ((*state).interpreter_stack.stkidrel_pointer as *mut i8).add(t__ as usize) as *mut TValue;
        }
        let callinfo = prepcallinfo(
            state,
            function,
            count_results,
            CALLSTATUS_LUA,
            (*state)
                .interpreter_top
                .stkidrel_pointer
                .add(LUA_MINSTACK as usize),
        );
        (*state).interpreter_callinfo = callinfo;
        if (*state).interpreter_hook_mask & HOOKMASK_CALL != 0 {
            let narg: i32 = ((*state).interpreter_top.stkidrel_pointer).offset_from(function) as i32 - 1;
            luad_hook(state, HOOKEVENT_CALL, -1, 1, narg);
        }
        let n: i32 = cfunction.expect("non-null function pointer")(state);
        luad_poscall(state, callinfo, n);
        n
    }
}
pub unsafe fn luad_pretailcall(
    state: *mut State,
    callinfo: *mut CallInfo,
    mut function: *mut TValue,
    mut narg1: i32,
    delta: i32,
) -> i32 {
    unsafe {
        let mut ccmt_count: u32 = 0;
        loop {
            match (*function).get_tagvariant() {
                TagVariant::ClosureC => {
                    return precallc(
                        state,
                        function,
                        -1,
                        (*(*function).as_closure().unwrap())
                            .closure_payload
                            .closurepayload_cfunction,
                    );
                }
                TagVariant::ClosureCFunction => {
                    return precallc(state, function, -1, (*function).as_function().unwrap());
                }
                TagVariant::ClosureL => {
                    let p: *mut Prototype = (*(*function).as_closure().unwrap())
                        .closure_payload
                        .closurepayload_lprototype;
                    let fsize: i32 = (*p).prototype_maximumstacksize as i32;
                    let nfixparams: i32 = (*p).prototype_countparameters as i32;
                    if ((*state).interpreter_stack_last.stkidrel_pointer).offset_from((*state).interpreter_top.stkidrel_pointer)
                        <= (fsize - delta) as isize
                    {
                        let t__: i64 =
                            (function as *mut i8).offset_from((*state).interpreter_stack.stkidrel_pointer as *mut i8) as i64;
                        (*state).do_gc_step_if_should_step();
                        luad_growstack(state, fsize - delta, true);
                        function = ((*state).interpreter_stack.stkidrel_pointer as *mut i8).add(t__ as usize) as *mut TValue;
                    }
                    (*callinfo).callinfo_function.stkidrel_pointer =
                        ((*callinfo).callinfo_function.stkidrel_pointer).sub(delta as usize);
                    for i in 0..narg1 {
                        let io1: *mut TValue = &mut (*((*callinfo).callinfo_function.stkidrel_pointer).add(i as usize));
                        let io2: *const TValue = &mut (*function.add(i as usize));
                        (*io1).copy_from(&*io2);
                    }
                    function = (*callinfo).callinfo_function.stkidrel_pointer;
                    while narg1 <= nfixparams {
                        (*function.add(narg1 as usize)).tvalue_set_tag_variant(TagVariant::NilNil);
                        narg1 += 1;
                    }
                    (*callinfo).callinfo_top.stkidrel_pointer = function.add(1).add(fsize as usize);
                    (*callinfo).callinfo_u.l.saved_program_counter = (*p).prototype_code.vectort_pointer;
                    (*callinfo).callinfo_callstatus = ((*callinfo).callinfo_callstatus as i32 | CALLSTATUS_TAIL) as u16;
                    (*state).interpreter_top.stkidrel_pointer = function.add(narg1 as usize);
                    return -1;
                }
                _ => {
                    if ccmt_count >= MAX_CCMT {
                        luag_runerror(state, c"'__call' chain too long".as_ptr(), &[]);
                    }
                    ccmt_count += 1;
                    function = tryfunctm(state, function);
                    narg1 += 1;
                }
            }
        }
    }
}
pub unsafe fn luad_precall(state: *mut State, mut function: *mut TValue, count_results: i32) -> *mut CallInfo {
    unsafe {
        let mut ccmt_count: u32 = 0;
        loop {
            match (*function).get_tagvariant() {
                TagVariant::ClosureC => {
                    precallc(
                        state,
                        function,
                        count_results,
                        (*(*function).as_closure().unwrap())
                            .closure_payload
                            .closurepayload_cfunction,
                    );
                    return null_mut();
                }
                TagVariant::ClosureCFunction => {
                    precallc(
                        state,
                        function,
                        count_results,
                        (*function).as_function().unwrap(),
                    );
                    return null_mut();
                }
                TagVariant::ClosureL => {
                    let p: *mut Prototype = (*(*function).as_closure().unwrap())
                        .closure_payload
                        .closurepayload_lprototype;
                    let mut narg: i32 = ((*state).interpreter_top.stkidrel_pointer).offset_from(function) as i32 - 1;
                    let nfixparams: i32 = (*p).prototype_countparameters as i32;
                    let fsize: i32 = (*p).prototype_maximumstacksize as i32;
                    if ((*state).interpreter_stack_last.stkidrel_pointer).offset_from((*state).interpreter_top.stkidrel_pointer)
                        <= fsize as isize
                    {
                        let t__: i64 =
                            (function as *mut i8).offset_from((*state).interpreter_stack.stkidrel_pointer as *mut i8) as i64;
                        (*state).do_gc_step_if_should_step();
                        luad_growstack(state, fsize, true);
                        function = ((*state).interpreter_stack.stkidrel_pointer as *mut i8).add(t__ as usize) as *mut TValue;
                    }
                    let callinfo = prepcallinfo(
                        state,
                        function,
                        count_results,
                        0,
                        function.add(1).add(fsize as usize),
                    );
                    (*state).interpreter_callinfo = callinfo;
                    (*callinfo).callinfo_u.l.saved_program_counter = (*p).prototype_code.vectort_pointer;
                    while narg < nfixparams {
                        let top = (*state).interpreter_top.stkidrel_pointer;
                        (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.add(1);
                        (*top).tvalue_set_tag_variant(TagVariant::NilNil);
                        narg += 1;
                    }
                    return callinfo;
                }
                _ => {
                    if ccmt_count >= MAX_CCMT {
                        luag_runerror(state, c"'__call' chain too long".as_ptr(), &[]);
                    }
                    ccmt_count += 1;
                    function = tryfunctm(state, function);
                }
            }
        }
    }
}
pub unsafe fn ccall(state: *mut State, mut function: *mut TValue, count_results: i32, inc: u32) {
    unsafe {
        (*state).interpreter_count_c_calls += inc;
        if (*state).interpreter_count_c_calls & INTERPRETER_C_CALLS_MASK >= LUAI_MAXCCALLS {
            if ((*state).interpreter_stack_last.stkidrel_pointer).offset_from((*state).interpreter_top.stkidrel_pointer) as i64 <= 0
            {
                let t__: i64 = (function as *mut i8).offset_from((*state).interpreter_stack.stkidrel_pointer as *mut i8) as i64;
                luad_growstack(state, 0, true);
                function = ((*state).interpreter_stack.stkidrel_pointer as *mut i8).add(t__ as usize) as *mut TValue;
            }
            (*state).luae_checkcstack();
        }
        let callinfo = luad_precall(state, function, count_results);
        if !callinfo.is_null() {
            (*callinfo).callinfo_callstatus = CALLSTATUS_HOOKED as u16;
            luav_execute(state, callinfo);
        }
        (*state).interpreter_count_c_calls -= inc;
    }
}
pub unsafe fn luad_callnoyield(state: *mut State, function: *mut TValue, count_results: i32) {
    unsafe {
        ccall(state, function, count_results, INTERPRETER_NY | 1);
    }
}
pub unsafe fn finishpcallk(state: *mut State, callinfo: *mut CallInfo) -> Status {
    unsafe {
        let mut status: Status =
            Status::from((*callinfo).callinfo_callstatus as i32 >> CALLSTATUS_RECST_SHIFT & CALLSTATUS_RECST_MASK);
        if status == Status::OK {
            status = Status::Yield;
        } else {
            let mut function: *mut TValue = ((*state).interpreter_stack.stkidrel_pointer as *mut i8)
                .add((*callinfo).callinfo_u2.callinfoconstituentb_funcidx as usize)
                as *mut TValue;
            (*state).interpreter_allow_hook = ((*callinfo).callinfo_callstatus as i32 & CALLSTATUS_ALLOWHOOK) as u8;
            function = luaf_close(state, function, status, 1);
            (*state).set_error_object(status, function);
            (*state).luad_shrinkstack();
            (*callinfo).callinfo_callstatus =
                ((*callinfo).callinfo_callstatus as i32 & !(CALLSTATUS_RECST_MASK << CALLSTATUS_RECST_SHIFT)) as u16;
        }
        (*callinfo).callinfo_callstatus = ((*callinfo).callinfo_callstatus as i32 & !CALLSTATUS_YPCALL) as u16;
        (*state).interpreter_error_function = (*callinfo).callinfo_u.c.old_error_function;
        status
    }
}
pub unsafe fn finishccall(state: *mut State, callinfo: *mut CallInfo) {
    unsafe {
        let n: i32;
        if (*callinfo).callinfo_callstatus as i32 & CALLSTATUS_TRAN != 0 {
            n = (*callinfo).callinfo_u2.callinfoconstituentb_nres;
        } else {
            let mut status = Status::Yield;
            if (*callinfo).callinfo_callstatus as i32 & CALLSTATUS_YPCALL != 0 {
                status = finishpcallk(state, callinfo);
            }
            if -1 <= -1
                && (*(*state).interpreter_callinfo)
                    .callinfo_top
                    .stkidrel_pointer
                    < (*state).interpreter_top.stkidrel_pointer
            {
                (*(*state).interpreter_callinfo)
                    .callinfo_top
                    .stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer;
            }
            n = ((*callinfo).callinfo_u.c.context_function).expect("non-null function pointer")(
                state,
                status,
                (*callinfo).callinfo_u.c.context,
            );
        }
        luad_poscall(state, callinfo, n);
    }
}
pub unsafe fn unroll(state: *mut State, mut _ud: *mut std::ffi::c_void) {
    unsafe {
        let mut callinfo;
        loop {
            callinfo = (*state).interpreter_callinfo;
            if !!std::ptr::eq(callinfo, &(*state).interpreter_base_callinfo) {
                break;
            }
            if (*callinfo).callinfo_callstatus as i32 & CALLSTATUS_LUA != 0 {
                finishccall(state, callinfo);
            } else {
                luav_finishop(state);
                luav_execute(state, callinfo);
            }
        }
    }
}
pub unsafe fn resume_error(state: *mut State, message: *const i8, narg: i32) -> Status {
    unsafe {
        (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.sub(narg as usize);
        let io: *mut TValue = &mut (*(*state).interpreter_top.stkidrel_pointer);
        let tstring: *mut TString = luas_new(state, message);
        (*io).set_object(tstring as *mut Object, (*tstring).get_tagvariant());
        (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.add(1);
        Status::RuntimeError
    }
}
pub unsafe fn resume(state: *mut State, arbitrary_data: *mut std::ffi::c_void) {
    unsafe {
        let mut n: i32 = *(arbitrary_data as *mut i32);
        let first_argument: *mut TValue = (*state).interpreter_top.stkidrel_pointer.sub(n as usize);
        let callinfo = (*state).interpreter_callinfo;
        if (*state).interpreter_status == Status::OK {
            ccall(state, first_argument.sub(1), -1, 0);
        } else {
            (*state).interpreter_status = Status::OK;
            if (*callinfo).callinfo_callstatus as i32 & CALLSTATUS_LUA == 0 {
                (*callinfo).callinfo_u.l.saved_program_counter = ((*callinfo).callinfo_u.l.saved_program_counter).sub(1);
                (*callinfo).callinfo_u.l.saved_program_counter;
                (*state).interpreter_top.stkidrel_pointer = first_argument;
                luav_execute(state, callinfo);
            } else {
                if ((*callinfo).callinfo_u.c.context_function).is_some() {
                    n = ((*callinfo).callinfo_u.c.context_function).expect("non-null function pointer")(
                        state,
                        Status::Yield,
                        (*callinfo).callinfo_u.c.context,
                    );
                }
                luad_poscall(state, callinfo, n);
            }
            unroll(state, null_mut());
        };
    }
}
pub unsafe fn precover(state: *mut State, mut status: Status) -> Status {
    unsafe {
        let mut callinfo;
        while status.is_error() && {
            callinfo = (*state).find_pcall();
            !callinfo.is_null()
        } {
            (*state).interpreter_callinfo = callinfo;
            (*callinfo).callinfo_callstatus = ((*callinfo).callinfo_callstatus as i32
                & !(CALLSTATUS_RECST_MASK << CALLSTATUS_RECST_SHIFT)
                | (status as i32) << CALLSTATUS_RECST_SHIFT) as u16;
            status = luad_rawrunprotected(
                state,
                Some(unroll as unsafe fn(*mut State, *mut std::ffi::c_void) -> ()),
                null_mut(),
            );
        }
        status
    }
}
pub unsafe fn lua_resume(state: *mut State, from: *mut State, mut nargs: i32, count_results: *mut i32) -> Status {
    unsafe {
        let mut status;
        if (*state).interpreter_status == Status::OK {
            if !std::ptr::eq(
                (*state).interpreter_callinfo,
                &(*state).interpreter_base_callinfo,
            ) {
                return resume_error(
                    state,
                    c"cannot resume non-suspended coroutine".as_ptr(),
                    nargs,
                );
            } else if ((*state).interpreter_top.stkidrel_pointer).offset_from(
                ((*(*state).interpreter_callinfo)
                    .callinfo_function
                    .stkidrel_pointer)
                    .add(1),
            ) as i64
                == nargs as i64
            {
                return resume_error(state, c"cannot resume dead coroutine".as_ptr(), nargs);
            }
        } else if (*state).interpreter_status != Status::Yield {
            return resume_error(state, c"cannot resume dead coroutine".as_ptr(), nargs);
        }
        (*state).interpreter_count_yield = 0;
        (*state).interpreter_count_c_calls = if from.is_null() {
            0
        } else {
            (*from).interpreter_count_c_calls & INTERPRETER_C_CALLS_MASK
        };
        if (*state).interpreter_count_c_calls & INTERPRETER_C_CALLS_MASK >= LUAI_MAXCCALLS {
            return resume_error(state, c"C stack overflow".as_ptr(), nargs);
        }
        (*state).interpreter_count_c_calls += 1;
        status = luad_rawrunprotected(
            state,
            Some(resume as unsafe fn(*mut State, *mut std::ffi::c_void) -> ()),
            &mut nargs as *mut i32 as *mut std::ffi::c_void,
        );
        status = precover(state, status);
        if status.is_error() {
            (*state).interpreter_status = status;
            (*state).set_error_object(status, (*state).interpreter_top.stkidrel_pointer);
            (*(*state).interpreter_callinfo)
                .callinfo_top
                .stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer;
        }
        *count_results = if status == Status::Yield {
            (*(*state).interpreter_callinfo)
                .callinfo_u2
                .callinfoconstituentb_nyield
        } else {
            ((*state).interpreter_top.stkidrel_pointer).offset_from(
                ((*(*state).interpreter_callinfo)
                    .callinfo_function
                    .stkidrel_pointer)
                    .add(1),
            ) as i32
        };
        status
    }
}
pub unsafe fn lua_yieldk(state: *mut State, count_results: i32, ctx: i64, k: ContextFunction) -> i32 {
    unsafe {
        let callinfo = (*state).interpreter_callinfo;
        if !(*state).is_yieldable() {
            if state != (*(*state).interpreter_global).global_maininterpreter {
                luag_runerror(
                    state,
                    c"attempt to yield across a C-call boundary".as_ptr(),
                    &[],
                );
            } else {
                luag_runerror(
                    state,
                    c"attempt to yield from outside a coroutine".as_ptr(),
                    &[],
                );
            }
        }
        (*state).interpreter_status = Status::Yield;
        (*callinfo).callinfo_u2.callinfoconstituentb_nyield = count_results;
        if (*callinfo).callinfo_callstatus as i32 & CALLSTATUS_LUA == 0 {
        } else {
            (*callinfo).callinfo_u.c.context_function = k;
            if ((*callinfo).callinfo_u.c.context_function).is_some() {
                (*callinfo).callinfo_u.c.context = ctx;
            }
            luad_throw(state, Status::Yield);
        }
        0
    }
}
pub unsafe fn luad_pcall(
    state: *mut State,
    function: ProtectedFunction,
    u: *mut std::ffi::c_void,
    old_top: i64,
    ef: i64,
) -> Status {
    unsafe {
        let old_call_info = (*state).interpreter_callinfo;
        let old_allowhooks: u8 = (*state).interpreter_allow_hook;
        let old_error_function: i64 = (*state).interpreter_error_function;
        (*state).interpreter_error_function = ef;
        let mut status = luad_rawrunprotected(state, function, u);
        if status != Status::OK {
            (*state).interpreter_callinfo = old_call_info;
            (*state).interpreter_allow_hook = old_allowhooks;
            status = do_close_protected(state, old_top, status);
            (*state).set_error_object(
                status,
                ((*state).interpreter_stack.stkidrel_pointer as *mut i8).add(old_top as usize) as *mut TValue,
            );
            (*state).luad_shrinkstack();
        }
        (*state).interpreter_error_function = old_error_function;
        status
    }
}
pub unsafe fn checkmode(state: *mut State, mode: *const i8, x: *const i8) {
    unsafe {
        if !mode.is_null() && (cstr_chr(mode, *x.add(0))).is_null() {
            luao_pushfstring(
                state,
                c"attempt to load a %s chunk (mode is '%s')".as_ptr(),
                &[x.into(), mode.into()],
            );
            luad_throw(state, Status::SyntaxError);
        }
    }
}
pub unsafe fn index2stack(state: *mut State, index: i32) -> *mut TValue {
    unsafe {
        let callinfo = (*state).interpreter_callinfo;
        if index > 0 {
            let o: *mut TValue = ((*callinfo).callinfo_function.stkidrel_pointer).add(index as usize);
            o
        } else {
            (*state)
                .interpreter_top
                .stkidrel_pointer
                .sub((-index) as usize)
        }
    }
}
pub unsafe fn lua_checkstack(state: *mut State, n: i32) -> i32 {
    unsafe {
        let res: i32;

        let callinfo = (*state).interpreter_callinfo;
        if ((*state).interpreter_stack_last.stkidrel_pointer).offset_from((*state).interpreter_top.stkidrel_pointer) as i64
            > n as i64
        {
            res = 1;
        } else {
            res = luad_growstack(state, n, false);
        }
        if res != 0 && (*callinfo).callinfo_top.stkidrel_pointer < (*state).interpreter_top.stkidrel_pointer.add(n as usize) {
            (*callinfo).callinfo_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.add(n as usize);
        }
        res
    }
}
pub unsafe fn lua_xmove(from: *mut State, to: *mut State, n: i32) {
    unsafe {
        if from != to {
            (*from).interpreter_top.stkidrel_pointer = ((*from).interpreter_top.stkidrel_pointer).sub(n as usize);
            for i in 0..n {
                let io1: *mut TValue = &mut (*(*to).interpreter_top.stkidrel_pointer);
                let io2: *const TValue = &mut (*((*from).interpreter_top.stkidrel_pointer).add(i as usize));
                (*io1).copy_from(&*io2);
                (*to).interpreter_top.stkidrel_pointer = ((*to).interpreter_top.stkidrel_pointer).add(1);
                (*to).interpreter_top.stkidrel_pointer;
            }
        }
    }
}
pub unsafe fn lua_atpanic(state: *mut State, panicf: CFunction) -> CFunction {
    unsafe {
        let old: CFunction = (*(*state).interpreter_global).global_panic;
        (*(*state).interpreter_global).global_panic = panicf;
        old
    }
}
pub unsafe fn lua_absindex(state: *mut State, index: i32) -> i32 {
    unsafe {
        if index > 0 || index <= LUA_REGISTRYINDEX {
            index
        } else {
            ((*state).interpreter_top.stkidrel_pointer).offset_from(
                (*(*state).interpreter_callinfo)
                    .callinfo_function
                    .stkidrel_pointer,
            ) as i32
                + index
        }
    }
}
pub unsafe fn lua_settop(state: *mut State, index: i32) {
    unsafe {
        let mut newtop;
        let mut diff;
        let callinfo = (*state).interpreter_callinfo;
        let function: *mut TValue = (*callinfo).callinfo_function.stkidrel_pointer;
        if index >= 0 {
            diff = function
                .add(1)
                .add(index as usize)
                .offset_from((*state).interpreter_top.stkidrel_pointer) as i64;
            while diff > 0 {
                let top = (*state).interpreter_top.stkidrel_pointer;
                (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.add(1);
                (*top).tvalue_set_tag_variant(TagVariant::NilNil);
                diff -= 1;
            }
        } else {
            diff = (index + 1) as i64;
        }
        newtop = (*state).interpreter_top.stkidrel_pointer.add(diff as usize);
        if diff < 0 && (*state).interpreter_tbclist.stkidrel_pointer >= newtop {
            newtop = luaf_close(state, newtop, Status::Closing, 0);
        }
        (*state).interpreter_top.stkidrel_pointer = newtop;
    }
}
pub unsafe fn lua_closeslot(state: *mut State, index: i32) {
    unsafe {
        let mut level = index2stack(state, index);
        level = luaf_close(state, level, Status::Closing, 0);
        (*level).tvalue_set_tag_variant(TagVariant::NilNil);
    }
}
pub unsafe fn reverse(mut _state: *mut State, mut from: *mut TValue, mut to: *mut TValue) {
    unsafe {
        while from < to {
            let mut temp: TValue = TValue::new(TagVariant::NilNil);
            let temp_dst: *mut TValue = &mut temp;
            let from_src: *const TValue = &mut (*from);
            (*temp_dst).copy_from(&*from_src);
            let from_dst: *mut TValue = &mut (*from);
            let to_src: *const TValue = &mut (*to);
            (*from_dst).copy_from(&*to_src);
            let to_dst: *mut TValue = &mut (*to);
            let temp_src: *const TValue = &mut temp;
            (*to_dst).copy_from(&*temp_src);
            from = from.add(1);
            to = to.sub(1);
        }
    }
}
pub unsafe fn lua_rotate(state: *mut State, index: i32, n: i32) {
    unsafe {
        let high: *mut TValue = (*state).interpreter_top.stkidrel_pointer.sub(1);
        let low: *mut TValue = index2stack(state, index);
        let middle: *mut TValue = if n >= 0 {
            high.sub(n as usize)
        } else {
            low.sub(n as usize).sub(1)
        };
        reverse(state, low, middle);
        reverse(state, middle.add(1), high);
        reverse(state, low, high);
    }
}
pub unsafe fn lua_copy(state: *mut State, fromidx: i32, toidx: i32) {
    unsafe {
        let fr: *mut TValue = (*state).index_to_value(fromidx);
        let to: *mut TValue = (*state).index_to_value(toidx);
        let io1: *mut TValue = to;
        let io2: *const TValue = fr;
        (*io1).copy_from(&*io2);
        if toidx < LUA_REGISTRYINDEX
            && (*fr).is_collectable()
            && (*(*(*(*state).interpreter_callinfo)
                .callinfo_function
                .stkidrel_pointer)
                .as_closure()
                .unwrap())
            .get_marked()
                & BLACKBIT
                != 0
            && (*(*fr).as_object().unwrap()).get_marked() & WHITEBITS != 0
        {
            Object::luac_barrier_(
                state,
                (*(*(*state).interpreter_callinfo)
                    .callinfo_function
                    .stkidrel_pointer)
                    .as_object()
                    .unwrap(),
                (*fr).as_object().unwrap(),
            );
        }
    }
}
pub unsafe fn lua_pushvalue(state: *mut State, index: i32) {
    unsafe {
        let io1: *mut TValue = &mut (*(*state).interpreter_top.stkidrel_pointer);
        let io2: *const TValue = (*state).index_to_value(index);
        (*io1).copy_from(&*io2);
        (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.add(1);
    }
}
pub unsafe fn lua_type(state: *mut State, index: i32) -> Option<TagType> {
    unsafe {
        let tvalue: *const TValue = (*state).index_to_value(index);
        if !(*tvalue).get_tagvariant().to_tag_type().is_nil()
            || !std::ptr::eq(tvalue, &(*(*state).interpreter_global).global_nonevalue)
        {
            Some((*tvalue).get_tagvariant().to_tag_type())
        } else {
            None
        }
    }
}
pub unsafe fn lua_typename(mut _state: *mut State, tagtype: Option<TagType>) -> *const i8 {
    match tagtype {
        None => c"no value".as_ptr(),
        Some(TagType::Nil) => c"nil".as_ptr(),
        Some(TagType::Boolean) => c"boolean".as_ptr(),
        Some(TagType::Pointer) => c"pointer".as_ptr(),
        Some(TagType::Numeric) => c"number".as_ptr(),
        Some(TagType::String) => c"string".as_ptr(),
        Some(TagType::Table) => c"table".as_ptr(),
        Some(TagType::Closure) => c"function".as_ptr(),
        Some(TagType::User) => c"userdata".as_ptr(),
        Some(TagType::State) => c"thread".as_ptr(),
        Some(TagType::UpValue) => c"upvalue".as_ptr(),
        Some(TagType::Prototype) => c"proto".as_ptr(),
        Some(TagType::DeadKey) => c"deadkey".as_ptr(),
    }
}
pub unsafe fn lua_iscfunction(state: *mut State, index: i32) -> bool {
    unsafe {
        let tvalue: *const TValue = (*state).index_to_value(index);
        match (*tvalue).get_tagvariant() {
            TagVariant::ClosureCFunction | TagVariant::ClosureC => true,
            _ => false,
        }
    }
}
pub unsafe fn lua_isinteger(state: *mut State, index: i32) -> bool {
    unsafe { (*(*state).index_to_value(index)).get_tagvariant() == TagVariant::NumericInteger }
}
pub unsafe fn lua_isnumber(state: *mut State, index: i32) -> bool {
    unsafe {
        let tvalue: *const TValue = (*state).index_to_value(index);
        if (*tvalue).get_tagvariant() == TagVariant::NumericNumber {
            true
        } else {
            let mut n: f64 = 0.0;
            (*tvalue).to_number(&mut n)
        }
    }
}
pub unsafe fn lua_isstring(state: *mut State, index: i32) -> bool {
    unsafe {
        let tvalue: *const TValue = (*state).index_to_value(index);
        match (*tvalue).get_tagvariant().to_tag_type() {
            TagType::Numeric => true,
            TagType::String => true,
            _ => false,
        }
    }
}
pub unsafe fn lua_rawequal(state: *mut State, index1: i32, index2: i32) -> bool {
    unsafe {
        let o1: *const TValue = (*state).index_to_value(index1);
        let o2: *const TValue = (*state).index_to_value(index2);
        if (!((*o1).get_tagvariant().to_tag_type().is_nil()) || !std::ptr::eq(o1, &(*(*state).interpreter_global).global_nonevalue))
            && (!((*o2).get_tagvariant().to_tag_type().is_nil())
                || !std::ptr::eq(o2, &(*(*state).interpreter_global).global_nonevalue))
        {
            luav_equalobj(null_mut(), o1, o2)
        } else {
            false
        }
    }
}
pub unsafe fn lua_compare(state: *mut State, index1: i32, index2: i32, op: i32) -> bool {
    unsafe {
        let o1: *const TValue = (*state).index_to_value(index1);
        let o2: *const TValue = (*state).index_to_value(index2);
        let mut ret: bool = false;
        if (!((*o1).get_tagvariant().to_tag_type().is_nil()) || !std::ptr::eq(o1, &(*(*state).interpreter_global).global_nonevalue))
            && (!((*o2).get_tagvariant().to_tag_type().is_nil())
                || !std::ptr::eq(o2, &(*(*state).interpreter_global).global_nonevalue))
        {
            match op {
                0 => {
                    ret = luav_equalobj(state, o1, o2);
                }
                1 => {
                    ret = luav_lessthan(state, o1, o2);
                }
                2 => {
                    ret = luav_lessequal(state, o1, o2);
                }
                _ => {}
            }
        }
        ret
    }
}
pub unsafe fn lua_stringtonumber(state: *mut State, s: *const i8) -> usize {
    unsafe {
        let size: usize = luao_str2num(s, &mut (*(*state).interpreter_top.stkidrel_pointer));
        if size != 0 {
            (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.add(1);
        }
        size
    }
}
pub unsafe fn lua_tonumberx(state: *mut State, index: i32, is_number: *mut bool) -> f64 {
    unsafe {
        let mut n: f64 = 0.0;
        let tvalue: *const TValue = (*state).index_to_value(index);
        let is_number_: bool = if (*tvalue).get_tagvariant() == TagVariant::NumericNumber {
            n = (*tvalue).as_number().unwrap();
            true
        } else {
            (*tvalue).to_number(&mut n)
        };
        if !is_number.is_null() {
            *is_number = is_number_;
        }
        n
    }
}
pub unsafe fn lua_tointegerx(state: *mut State, index: i32, is_number: *mut bool) -> i64 {
    unsafe {
        let mut res: i64 = 0;
        let tvalue: *const TValue = (*state).index_to_value(index);
        let is_number_: bool = if (*tvalue).get_tagvariant() == TagVariant::NumericInteger {
            res = (*tvalue).as_integer().unwrap();
            true
        } else {
            F2I::Equal.convert_tv_i64(tvalue, &mut res) != 0
        };
        if !is_number.is_null() {
            *is_number = is_number_;
        }
        res
    }
}
pub unsafe fn lua_toboolean(state: *mut State, index: i32) -> bool {
    unsafe {
        let tvalue: *const TValue = (*state).index_to_value(index);
        !((*tvalue).get_tagvariant() == TagVariant::BooleanFalse || (*tvalue).get_tagvariant().to_tag_type().is_nil())
    }
}
pub unsafe fn lua_tolstring(state: *mut State, index: i32, length: *mut usize) -> *const i8 {
    unsafe {
        let mut o: *mut TValue = (*state).index_to_value(index);
        if !((*o).get_tagvariant().to_tag_type().is_string()) {
            if !((*o).get_tagvariant().to_tag_type().is_numeric()) {
                if !length.is_null() {
                    *length = 0;
                }
                return null();
            }
            (*o).from_interpreter_to_string(state);
            (*state).do_gc_step_if_should_step();
            o = (*state).index_to_value(index);
        }
        if !length.is_null() {
            *length = (*(*o).as_string().unwrap()).get_length();
        }
        (*(*o).as_string().unwrap()).get_contents_mut()
    }
}
pub unsafe fn get_length_raw(state: *mut State, index: i32) -> usize {
    unsafe {
        let tvalue: *const TValue = (*state).index_to_value(index);
        match (*tvalue).get_tagvariant() {
            TagVariant::StringShort | TagVariant::StringLong => (*(*tvalue).as_string().unwrap()).get_length_raw(),
            TagVariant::User => (*(*tvalue).as_user().unwrap()).get_length_raw(),
            TagVariant::Table => (*(*tvalue).as_table().unwrap()).get_length_raw(state),
            _ => 0,
        }
    }
}
pub unsafe fn lua_tothread(state: *mut State, index: i32) -> *mut State {
    unsafe {
        let tvalue: *const TValue = (*state).index_to_value(index);
        if (*tvalue).get_tagvariant() == TagVariant::State {
            (*tvalue).as_object().unwrap() as *mut State
        } else {
            null_mut()
        }
    }
}
pub unsafe fn lua_pushlstring(state: *mut State, s: *const i8, length: usize) -> *const i8 {
    unsafe {
        let tstring: *mut TString = if length == 0 {
            luas_new(state, c"".as_ptr())
        } else {
            luas_newlstr(state, s, length)
        };
        let io: *mut TValue = &mut (*(*state).interpreter_top.stkidrel_pointer);
        (*io).set_object(tstring as *mut Object, (*tstring).get_tagvariant());
        (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.add(1);
        (*state).do_gc_step_if_should_step();
        (*tstring).get_contents_mut()
    }
}
pub unsafe fn lua_pushexternalstring(
    state: *mut State,
    s: *const i8,
    length: usize,
    allocation_function: AllocationFunction,
    user_data: *mut std::ffi::c_void,
) -> *const i8 {
    unsafe {
        let tstring: *mut TString = TString::create_external(state, s, length, allocation_function, user_data);
        let io: *mut TValue = &mut (*(*state).interpreter_top.stkidrel_pointer);
        (*io).set_object(tstring as *mut Object, (*tstring).get_tagvariant());
        (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.add(1);
        (*state).do_gc_step_if_should_step();
        (*tstring).get_contents_mut()
    }
}
pub unsafe fn lua_pushstring(state: *mut State, mut s: *const i8) -> *const i8 {
    unsafe {
        if s.is_null() {
            (*(*state).interpreter_top.stkidrel_pointer).tvalue_set_tag_variant(TagVariant::NilNil);
        } else {
            let tstring: *mut TString = luas_new(state, s);
            let io: *mut TValue = &mut (*(*state).interpreter_top.stkidrel_pointer);
            (*io).set_object(tstring as *mut Object, (*tstring).get_tagvariant());
            s = (*tstring).get_contents_mut();
        }
        (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.add(1);
        (*state).do_gc_step_if_should_step();
        s
    }
}
pub unsafe fn lua_pushvfstring(state: *mut State, fmt: *const i8, args: &[crate::fmtarg::FmtArg]) -> *const i8 {
    unsafe {
        let ret = luao_pushvfstring(state, fmt, args);
        (*state).do_gc_step_if_should_step();
        ret
    }
}
pub unsafe fn lua_pushfstring(state: *mut State, fmt: *const i8, args: &[crate::fmtarg::FmtArg]) -> *const i8 {
    unsafe {
        let ret: *const i8 = luao_pushvfstring(state, fmt, args);
        (*state).do_gc_step_if_should_step();
        ret
    }
}
pub unsafe fn lua_pushcclosure(state: *mut State, fn_0: CFunction, mut n: i32) {
    unsafe {
        if n == 0 {
            let io: *mut TValue = &mut (*(*state).interpreter_top.stkidrel_pointer);
            (*io).set_function(fn_0);
            (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.add(1);
        } else {
            let closure: *mut Closure = Closure::luaf_newcclosure(state, n);
            (*closure).closure_payload.closurepayload_cfunction = fn_0;
            (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.sub(n as usize);
            loop {
                let prev_n = n;
                n -= 1;
                if prev_n == 0 {
                    break;
                }
                let io1: *mut TValue = &mut *((*closure).closure_upvalues)
                    .closureupvalue_tvalues
                    .as_mut_ptr()
                    .add(n as usize) as *mut TValue;
                let io2: *const TValue = &mut (*(*state).interpreter_top.stkidrel_pointer.add(n as usize));
                (*io1).copy_from(&*io2);
            }
            let io: *mut TValue = &mut (*(*state).interpreter_top.stkidrel_pointer);
            let x_: *mut Closure = closure;
            (*io).set_object(x_ as *mut Object, TagVariant::ClosureC);
            (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.add(1);
            (*state).do_gc_step_if_should_step();
        };
    }
}
pub unsafe fn lua_pushlightuserdata(state: *mut State, pointer: *mut std::ffi::c_void) {
    unsafe {
        let tvalue: *mut TValue = &mut (*(*state).interpreter_top.stkidrel_pointer);
        (*tvalue).set_pointer(pointer);
        (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.add(1);
    }
}
pub unsafe fn auxgetstr(state: *mut State, t: *const TValue, k: *const i8) -> TagType {
    unsafe {
        let tag: TagVariant;
        let str: *mut TString = luas_new(state, k);
        if !((*t).get_tagvariant() == TagVariant::Table) {
            tag = TagVariant::NilNil;
        } else {
            let slot = luah_getstr((*t).as_table().unwrap(), str);
            if !(*slot).get_tagvariant().to_tag_type().is_nil() {
                let io1: *mut TValue = &mut (*(*state).interpreter_top.stkidrel_pointer);
                (*io1).copy_from(&*slot);
                (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.add(1);
                return (*(*state).interpreter_top.stkidrel_pointer.sub(1))
                    .get_tagvariant()
                    .to_tag_type();
            }
            tag = (*slot).get_tagvariant();
        }
        {
            let io: *mut TValue = &mut (*(*state).interpreter_top.stkidrel_pointer);
            (*io).set_object(str as *mut Object, (*str).get_tagvariant());
            (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.add(1);
            luav_finishget(
                state,
                t,
                &mut (*(*state).interpreter_top.stkidrel_pointer.sub(1)),
                (*state).interpreter_top.stkidrel_pointer.sub(1),
                tag,
            );
        }
        (*(*state).interpreter_top.stkidrel_pointer.sub(1))
            .get_tagvariant()
            .to_tag_type()
    }
}
pub unsafe fn lua_getglobal(state: *mut State, name: *const i8) -> TagType {
    unsafe {
        let registry = (*(*state).interpreter_global)
            .global_lregistry
            .as_table()
            .unwrap();
        let mut global_table: TValue = TValue::new(TagVariant::NilNil);
        arr2obj(registry, 1, &mut global_table);
        auxgetstr(state, &global_table, name)
    }
}
pub unsafe fn lua_gettable(state: *mut State, index: i32) -> i32 {
    unsafe {
        let t: *mut TValue = (*state).index_to_value(index);
        let tag;
        if (*t).get_tagvariant() != TagVariant::Table {
            tag = TagVariant::NilNil;
        } else {
            let dest = (*state).interpreter_top.stkidrel_pointer.sub(1);
            tag = luah_get((*t).as_table().unwrap(), &(*dest), dest);
        }
        if tag.to_tag_type().is_nil() {
            luav_finishget(
                state,
                t,
                &mut (*(*state).interpreter_top.stkidrel_pointer.sub(1)),
                (*state).interpreter_top.stkidrel_pointer.sub(1),
                tag,
            );
        }
        ((*(*state).interpreter_top.stkidrel_pointer.sub(1)).get_tagvariant()).to_tag_type() as i32
    }
}
pub unsafe fn handle_luainit(state: *mut State) -> Status {
    unsafe {
        let mut name: *const i8 = c"=LUA_INIT_5_5".as_ptr();
        let mut initial: *const i8 = os_getenv(name.add(1));
        if initial.is_null() {
            name = c"=LUA_INIT".as_ptr();
            initial = os_getenv(name.add(1));
        }
        if initial.is_null() {
            Status::OK
        } else if *initial.add(0) as i32 == Character::At as i32 {
            dofile(state, initial.add(1))
        } else {
            dostring(state, initial, name)
        }
    }
}
pub unsafe fn lua_getfield(state: *mut State, index: i32, k: *const i8) -> TagType {
    unsafe { auxgetstr(state, (*state).index_to_value(index), k) }
}
pub unsafe fn lua_geti(state: *mut State, index: i32, n: i64) -> TagType {
    unsafe {
        let t: *mut TValue = (*state).index_to_value(index);
        let tag;
        if (*t).get_tagvariant() != TagVariant::Table {
            tag = TagVariant::NilNil;
        } else {
            tag = luah_getint(
                (*t).as_table().unwrap(),
                n,
                (*state).interpreter_top.stkidrel_pointer,
            );
        }
        if tag.to_tag_type().is_nil() {
            let mut aux: TValue = TValue::new(TagVariant::NilNil);
            aux.set_integer(n);
            luav_finishget(
                state,
                t,
                &mut aux,
                (*state).interpreter_top.stkidrel_pointer,
                tag,
            );
        }
        (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.add(1);
        (*(*state).interpreter_top.stkidrel_pointer.sub(1))
            .get_tagvariant()
            .to_tag_type()
    }
}
pub unsafe fn finishrawget(state: *mut State, tag: TagVariant) -> TagType {
    unsafe {
        if tag.to_tag_type().is_nil() {
            (*(*state).interpreter_top.stkidrel_pointer).tvalue_set_tag_variant(TagVariant::NilNil);
        }
        (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.add(1);
        (*(*state).interpreter_top.stkidrel_pointer.sub(1))
            .get_tagvariant()
            .to_tag_type()
    }
}
pub unsafe fn gettable(state: *mut State, index: i32) -> *mut Table {
    unsafe {
        let t: *mut TValue = (*state).index_to_value(index);
        (*t).as_table().unwrap()
    }
}
pub unsafe fn lua_rawget(state: *mut State, index: i32) -> TagType {
    unsafe {
        let table: *mut Table = gettable(state, index);
        let tag = luah_get(
            table,
            &(*(*state).interpreter_top.stkidrel_pointer.sub(1)),
            (*state).interpreter_top.stkidrel_pointer.sub(1),
        );
        (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.sub(1);
        finishrawget(state, tag)
    }
}
pub unsafe fn lua_rawgeti(state: *mut State, index: i32, n: i64) -> TagType {
    unsafe {
        let table: *mut Table = gettable(state, index);
        let tag = luah_getint(table, n, (*state).interpreter_top.stkidrel_pointer);
        finishrawget(state, tag)
    }
}
pub unsafe fn auxsetstr(state: *mut State, t: *const TValue, k: *const i8) {
    unsafe {
        let hres: i32;
        let str: *mut TString = luas_new(state, k);
        if !((*t).get_tagvariant() == TagVariant::Table) {
            hres = HNOTATABLE;
        } else {
            let slot = luah_getstr((*t).as_table().unwrap(), str);
            if !(*slot).get_tagvariant().to_tag_type().is_nil() {
                let io1: *mut TValue = slot as *mut TValue;
                let io2: *const TValue = &mut (*(*state).interpreter_top.stkidrel_pointer.sub(1));
                (*io1).copy_from(&*io2);
                if (*(*state).interpreter_top.stkidrel_pointer.sub(1)).is_collectable()
                    && (*(*t).as_object().unwrap()).get_marked() & BLACKBIT != 0
                    && (*(*(*state).interpreter_top.stkidrel_pointer.sub(1))
                        .as_object()
                        .unwrap())
                    .get_marked()
                        & WHITEBITS
                        != 0
                {
                    ObjectWithGCList::luac_barrierback_(state, (*t).as_object().unwrap() as *mut ObjectWithGCList);
                }
                (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.sub(1);
                return;
            }
            hres = retpsetcode((*t).as_table().unwrap(), slot);
        }
        {
            let io: *mut TValue = &mut (*(*state).interpreter_top.stkidrel_pointer);
            (*io).set_object(str as *mut Object, (*str).get_tagvariant());
            (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.add(1);
            luav_finishset(
                state,
                t,
                &mut (*(*state).interpreter_top.stkidrel_pointer.sub(1)),
                &mut (*(*state).interpreter_top.stkidrel_pointer.sub(2)),
                hres,
            );
            (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.sub(2);
        };
    }
}
pub unsafe fn lua_setglobal(state: *mut State, name: *const i8) {
    unsafe {
        let registry = (*(*state).interpreter_global)
            .global_lregistry
            .as_table()
            .unwrap();
        let mut global_table: TValue = TValue::new(TagVariant::NilNil);
        arr2obj(registry, 1, &mut global_table);
        auxsetstr(state, &global_table, name);
    }
}
pub unsafe fn lua_setfield(state: *mut State, index: i32, k: *const i8) {
    unsafe {
        auxsetstr(state, (*state).index_to_value(index), k);
    }
}
pub unsafe fn lua_seti(state: *mut State, index: i32, n: i64) {
    unsafe {
        let t: *mut TValue = (*state).index_to_value(index);
        let hres: i32;
        if (*t).get_tagvariant() != TagVariant::Table {
            hres = HNOTATABLE;
        } else {
            let h = (*t).as_table().unwrap();
            let ik = ikeyinarray(h, n);
            if ik > 0 {
                let tag = get_arr_tag(h, ik - 1);
                if ((*h).get_metatable().is_null() || (*(*h).get_metatable()).table_flags as u32 & 1_u32 << TM_NEWINDEX as i32 != 0)
                    || !tagisempty(*tag)
                {
                    *tag = table_store_tag((*(*state).interpreter_top.stkidrel_pointer.sub(1)).get_tagvariant() as u8);
                    *get_arr_val(h, ik - 1) = (*(*state).interpreter_top.stkidrel_pointer.sub(1)).get_raw_value();
                    (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.sub(1);
                    return;
                } else {
                    hres = !(ik as i32 - 1);
                }
            } else {
                hres = luah_psetint(h, n, (*state).interpreter_top.stkidrel_pointer.sub(1));
            }
        }
        if hres != HOK {
            let mut aux: TValue = TValue::new(TagVariant::NumericInteger);
            aux.set_integer(n);
            luav_finishset(
                state,
                t,
                &mut aux,
                &mut (*(*state).interpreter_top.stkidrel_pointer.sub(1)),
                hres,
            );
        }
        (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.sub(1);
    }
}
pub unsafe fn aux_rawset(state: *mut State, index: i32, key: *mut TValue, n: i32) {
    unsafe {
        let table: *mut Table = gettable(state, index);
        luah_set(
            state,
            table,
            key,
            &mut (*(*state).interpreter_top.stkidrel_pointer.sub(1)),
        );
        (*table).table_flags = ((*table).table_flags as u32 & !!(!0 << (TM_EQ as i32 + 1))) as u8;
        if (*(*state).interpreter_top.stkidrel_pointer.sub(1)).is_collectable()
            && (*(table as *mut Object)).get_marked() & BLACKBIT != 0
            && (*(*(*state).interpreter_top.stkidrel_pointer.sub(1))
                .as_object()
                .unwrap())
            .get_marked()
                & WHITEBITS
                != 0
        {
            ObjectWithGCList::luac_barrierback_(state, &mut *(table as *mut ObjectWithGCList));
        }
        (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.sub(n as usize);
    }
}
pub unsafe fn lua_rawset(state: *mut State, index: i32) {
    unsafe {
        aux_rawset(
            state,
            index,
            &mut (*(*state).interpreter_top.stkidrel_pointer.sub(2)),
            2,
        );
    }
}
pub unsafe fn lua_rawseti(state: *mut State, index: i32, n: i64) {
    unsafe {
        let table: *mut Table = gettable(state, index);
        luah_setint(
            state,
            table,
            n,
            &mut (*(*state).interpreter_top.stkidrel_pointer.sub(1)),
        );
        if (*(*state).interpreter_top.stkidrel_pointer.sub(1)).is_collectable()
            && (*(table as *mut Object)).get_marked() & BLACKBIT != 0
            && (*(*(*state).interpreter_top.stkidrel_pointer.sub(1))
                .as_object()
                .unwrap())
            .get_marked()
                & WHITEBITS
                != 0
        {
            ObjectWithGCList::luac_barrierback_(state, &mut *(table as *mut ObjectWithGCList));
        }
        (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.sub(1);
    }
}
pub unsafe fn lua_setmetatable(state: *mut State, index: i32) -> i32 {
    unsafe {
        let metatable: *mut Table;
        let object: *mut TValue = (*state).index_to_value(index);
        if (*(*state).interpreter_top.stkidrel_pointer.sub(1))
            .get_tagvariant()
            .to_tag_type()
            .is_nil()
        {
            metatable = null_mut();
        } else {
            metatable = (*(*state).interpreter_top.stkidrel_pointer.sub(1))
                .as_table()
                .unwrap()
        }
        match (*object).get_tagvariant().to_tag_type() {
            TagType::Table => {
                (*(*object).as_table().unwrap()).set_metatable(metatable);
                if !metatable.is_null() {
                    if (*(*object).as_object().unwrap()).get_marked() & BLACKBIT != 0 && (*metatable).get_marked() & WHITEBITS != 0
                    {
                        Object::luac_barrier_(
                            state,
                            &mut (*((*object).as_object().unwrap())),
                            &mut *(metatable as *mut Object),
                        );
                    }
                    luac_checkfinalizer(state, (*object).as_object().unwrap(), metatable);
                }
            }
            TagType::User => {
                (*(*object).as_user().unwrap()).set_metatable(metatable);
                if !metatable.is_null() {
                    if (*(*object).as_user().unwrap()).get_marked() & BLACKBIT != 0 && (*metatable).get_marked() & WHITEBITS != 0 {
                        Object::luac_barrier_(
                            state,
                            &mut *((*object).as_user().unwrap() as *mut Object),
                            &mut *(metatable as *mut Object),
                        );
                    }
                    luac_checkfinalizer(state, (*object).as_object().unwrap(), metatable);
                }
            }
            _ => {
                (*(*state).interpreter_global).global_metatables[(*object).get_tagvariant().to_tag_type() as usize] = metatable;
            }
        }
        (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.sub(1);
        1
    }
}
pub unsafe fn lua_setiuservalue(state: *mut State, index: i32, n: i32) -> i32 {
    unsafe {
        let res: i32;
        let o: *mut TValue = (*state).index_to_value(index);
        if (n as u32).wrapping_sub(1_u32) >= (*(*o).as_user().unwrap()).user_countupvalues as u32 {
            res = 0;
        } else {
            let io1: *mut TValue = &mut (*((*(*o).as_user().unwrap()).user_upvalues)
                .as_mut_ptr()
                .add((n - 1) as usize));
            let io2: *const TValue = &mut (*(*state).interpreter_top.stkidrel_pointer.sub(1));
            (*io1).copy_from(&*io2);
            if (*(*state).interpreter_top.stkidrel_pointer.sub(1)).is_collectable()
                && (*(*o).as_object().unwrap()).get_marked() & BLACKBIT != 0
                && (*(*(*state).interpreter_top.stkidrel_pointer.sub(1))
                    .as_object()
                    .unwrap())
                .get_marked()
                    & WHITEBITS
                    != 0
            {
                ObjectWithGCList::luac_barrierback_(state, (*o).as_object().unwrap() as *mut ObjectWithGCList);
            }
            res = 1;
        }
        (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.sub(1);
        res
    }
}
pub unsafe fn lua_load(
    state: *mut State,
    reader: Reader,
    data: *mut std::ffi::c_void,
    chunkname: *const i8,
    mode: *const i8,
) -> Status {
    unsafe {
        let status = do_protected_parser(state, chunkname, mode, reader, data);
        if status == Status::OK {
            let closure = (*(*state).interpreter_top.stkidrel_pointer.sub(1))
                .as_closure()
                .unwrap();
            if (*closure).closure_count_upvalues as i32 >= 1 {
                let registry = (*(*state).interpreter_global)
                    .global_lregistry
                    .as_table()
                    .unwrap();
                let mut gt_val: TValue = TValue::new(TagVariant::NilNil);
                arr2obj(registry, 1, &mut gt_val);
                let gt = &mut gt_val as *mut TValue;
                let io1: *mut TValue = (**((*closure).closure_upvalues)
                    .closureupvalue_lvalues
                    .as_mut_ptr()
                    .add(0))
                .upvalue_v
                .upvaluea_p;
                (*io1).copy_from(&*gt);
                if (*gt).is_collectable()
                    && (**((*closure).closure_upvalues)
                        .closureupvalue_lvalues
                        .as_mut_ptr()
                        .add(0))
                    .get_marked()
                        & BLACKBIT
                        != 0
                    && (*(*gt).as_object().unwrap()).get_marked() & WHITEBITS != 0
                {
                    Object::luac_barrier_(
                        state,
                        &mut *(*((*closure).closure_upvalues)
                            .closureupvalue_lvalues
                            .as_mut_ptr()
                            .add(0) as *mut Object),
                        &mut (*((*gt).as_object().unwrap())),
                    );
                }
            }
        }
        status
    }
}
pub unsafe fn lua_error(state: *mut State) -> i32 {
    unsafe {
        let errobj: *mut TValue = &mut (*(*state).interpreter_top.stkidrel_pointer.sub(1));
        if (*errobj).get_tagvariant() == TagVariant::StringShort
            && (*errobj).as_string().unwrap() == (*(*state).interpreter_global).global_memoryerrormessage
        {
            luad_throw(state, Status::MemoryError);
        } else {
            luag_errormsg(state);
        };
    }
}
pub unsafe fn lua_next(state: *mut State, index: i32) -> i32 {
    unsafe {
        let table: *mut Table = gettable(state, index);
        let more: i32 = luah_next(
            state,
            table,
            (*state).interpreter_top.stkidrel_pointer.sub(1),
        );
        if more != 0 {
            (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.add(1);
        } else {
            (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.sub(1);
        }
        more
    }
}
pub unsafe fn lua_toclose(state: *mut State, index: i32) {
    unsafe {
        let o: *mut TValue = index2stack(state, index);
        let count_results: i32 = (*(*state).interpreter_callinfo).callinfo_count_results;
        luaf_newtbcupval(state, o);
        if count_results >= -1 {
            (*(*state).interpreter_callinfo).callinfo_count_results = -count_results - 3;
        }
    }
}
pub unsafe fn lua_concat(state: *mut State, n: i32) {
    unsafe {
        if n > 0 {
            concatenate(state, n);
        } else {
            let io: *mut TValue = &mut (*(*state).interpreter_top.stkidrel_pointer);
            let tstring: *mut TString = luas_newlstr(state, c"".as_ptr(), 0);
            (*io).set_object(tstring as *mut Object, (*tstring).get_tagvariant());
            (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.add(1);
        }
        (*state).do_gc_step_if_should_step();
    }
}
pub unsafe fn lua_len(state: *mut State, index: i32) {
    unsafe {
        let t: *mut TValue = (*state).index_to_value(index);
        luav_objlen(state, (*state).interpreter_top.stkidrel_pointer, t);
        (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.add(1);
    }
}
pub unsafe fn lua_setwarnf(state: *mut State, f: WarnFunction, arbitrary_data: *mut std::ffi::c_void) {
    unsafe {
        (*(*state).interpreter_global).global_warnuserdata = arbitrary_data;
        (*(*state).interpreter_global).global_warnfunction = f;
    }
}
pub unsafe fn lua_getupvalue(state: *mut State, funcindex: i32, n: i32) -> *const i8 {
    unsafe {
        let mut value: *mut TValue = null_mut();
        let name: *const i8 = aux_upvalue(
            (*state).index_to_value(funcindex),
            n,
            &mut value,
            null_mut(),
        );
        if !name.is_null() {
            let io1: *mut TValue = &mut (*(*state).interpreter_top.stkidrel_pointer);
            let io2: *const TValue = value;
            (*io1).copy_from(&*io2);
            (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.add(1);
        }
        name
    }
}
pub unsafe fn lua_setupvalue(state: *mut State, funcindex: i32, n: i32) -> *const i8 {
    unsafe {
        let mut value: *mut TValue = null_mut();
        let mut owner: *mut Object = null_mut();
        let fi: *mut TValue = (*state).index_to_value(funcindex);
        let name: *const i8 = aux_upvalue(fi, n, &mut value, &mut owner);
        if !name.is_null() {
            (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.sub(1);
            let io1: *mut TValue = value;
            let io2: *const TValue = &mut (*(*state).interpreter_top.stkidrel_pointer);
            (*io1).copy_from(&*io2);
            if (*value).is_collectable()
                && (*owner).get_marked() & BLACKBIT != 0
                && (*(*value).as_object().unwrap()).get_marked() & WHITEBITS != 0
            {
                Object::luac_barrier_(state, &mut *owner, &mut (*((*value).as_object().unwrap())));
            }
        }
        name
    }
}
pub const NULLUP: *const UpValue = null();
pub unsafe fn getupvalref(state: *mut State, fidx: i32, n: i32, pf: *mut *mut Closure) -> *mut *mut UpValue {
    unsafe {
        let fi: *mut TValue = (*state).index_to_value(fidx);
        let closure: *mut Closure = (*fi).as_closure().unwrap();
        if !pf.is_null() {
            *pf = closure;
        }
        if 1 <= n
            && n <= (*(*closure).closure_payload.closurepayload_lprototype)
                .prototype_upvalues
                .get_size() as i32
        {
            &mut *((*closure).closure_upvalues)
                .closureupvalue_lvalues
                .as_mut_ptr()
                .add((n - 1) as usize) as *mut *mut UpValue
        } else {
            &NULLUP as *const *const UpValue as *mut *mut UpValue
        }
    }
}
pub unsafe fn lua_upvalueid(state: *mut State, fidx: i32, n: i32) -> *mut std::ffi::c_void {
    unsafe {
        let fi: *mut TValue = (*state).index_to_value(fidx);
        match (*fi).get_tagvariant() {
            TagVariant::ClosureL => {
                return *getupvalref(state, fidx, n, null_mut()) as *mut std::ffi::c_void;
            }
            TagVariant::ClosureC => {
                let closure: *mut Closure = (*fi).as_closure().unwrap();
                if 1 <= n && n <= (*closure).closure_count_upvalues as i32 {
                    return &mut *((*closure).closure_upvalues)
                        .closureupvalue_tvalues
                        .as_mut_ptr()
                        .add((n - 1) as usize) as *mut TValue as *mut std::ffi::c_void;
                }
            }
            TagVariant::ClosureCFunction => {}
            _ => return null_mut(),
        }
        null_mut()
    }
}
pub unsafe fn lua_upvaluejoin(state: *mut State, fidx1: i32, n1: i32, fidx2: i32, n2: i32) {
    unsafe {
        let mut f1: *mut Closure = null_mut();
        let up1: *mut *mut UpValue = getupvalref(state, fidx1, n1, &mut f1);
        let up2: *mut *mut UpValue = getupvalref(state, fidx2, n2, null_mut());
        *up1 = *up2;
        if (*f1).get_marked() & BLACKBIT != 0 && (**up1).get_marked() & WHITEBITS != 0 {
            Object::luac_barrier_(
                state,
                &mut *(f1 as *mut Object),
                &mut *(*up1 as *mut Object),
            );
        }
    }
}
pub unsafe fn luai_makeseed(state: *mut State) -> u32 {
    unsafe {
        let mut buffer: [i8; 24] = [0; 24];
        let mut h: u32 = os_time_now() as u32;
        let mut p: i32 = 0;
        let t: usize = state as usize;
        std::ptr::copy_nonoverlapping(
            &t as *const usize as *const u8,
            buffer.as_mut_ptr().add(p as usize) as *mut u8,
            size_of::<usize>(),
        );
        p += size_of::<usize>() as i32;
        let addr_h: usize = &mut h as *mut u32 as usize;
        std::ptr::copy_nonoverlapping(
            &addr_h as *const usize as *const u8,
            buffer.as_mut_ptr().add(p as usize) as *mut u8,
            size_of::<usize>(),
        );
        p += size_of::<usize>() as i32;
        let addr_fn: usize = ::core::mem::transmute::<Option<unsafe fn(state: *mut State) -> u32>, usize>(Some(
            luai_makeseed as unsafe fn(state: *mut State) -> u32,
        ));
        std::ptr::copy_nonoverlapping(
            &addr_fn as *const usize as *const u8,
            buffer.as_mut_ptr().add(p as usize) as *mut u8,
            size_of::<usize>(),
        );
        p += size_of::<usize>() as i32;
        luas_hash(buffer.as_mut_ptr(), p as usize, h)
    }
}
pub unsafe fn luae_extendci(state: *mut State) -> *mut CallInfo {
    unsafe {
        let ret = (*state).allocate(size_of::<CallInfo>()) as *mut CallInfo;
        (*(*state).interpreter_callinfo).callinfo_next = ret;
        (*ret).callinfo_previous = (*state).interpreter_callinfo;
        (*ret).callinfo_next = null_mut();
        write_volatile(&mut (*ret).callinfo_u.l.trap as *mut i32, 0);
        (*state).interpreter_count_callinfo += 1;
        (*state).interpreter_count_callinfo;
        ret
    }
}
pub unsafe fn freeci(state: *mut State) {
    unsafe {
        let mut callinfo = (*state).interpreter_callinfo;
        let mut next_call_info = (*callinfo).callinfo_next;
        (*callinfo).callinfo_next = null_mut();
        loop {
            callinfo = next_call_info;
            if callinfo.is_null() {
                break;
            }
            next_call_info = (*callinfo).callinfo_next;
            (*state).free_memory(callinfo as *mut std::ffi::c_void, size_of::<CallInfo>());
            (*state).interpreter_count_callinfo -= 1;
            (*state).interpreter_count_callinfo;
        }
    }
}
pub unsafe fn luae_shrinkci(state: *mut State) {
    unsafe {
        let mut callinfo = (*(*state).interpreter_callinfo).callinfo_next;
        if !callinfo.is_null() {
            loop {
                let next_call_info = (*callinfo).callinfo_next;
                if next_call_info.is_null() {
                    break;
                }
                let next_next_call_info = (*next_call_info).callinfo_next;
                (*callinfo).callinfo_next = next_next_call_info;
                (*state).interpreter_count_callinfo -= 1;
                (*state).interpreter_count_callinfo;
                (*state).free_memory(
                    next_call_info as *mut std::ffi::c_void,
                    size_of::<CallInfo>(),
                );
                if next_next_call_info.is_null() {
                    break;
                }
                (*next_next_call_info).callinfo_previous = callinfo;
                callinfo = next_next_call_info;
            }
        }
    }
}
pub unsafe fn stack_init(other_state: *mut State, state: *mut State) {
    unsafe {
        (*other_state).interpreter_stack.stkidrel_pointer =
            (*state).allocate(((BASIC_STACK_SIZE + EXTRA_STACK) as usize).wrapping_mul(size_of::<TValue>())) as *mut TValue;
        (*other_state).interpreter_tbclist.stkidrel_pointer = (*other_state).interpreter_stack.stkidrel_pointer;
        for i in 0..BASIC_STACK_SIZE + EXTRA_STACK {
            (*((*other_state).interpreter_stack.stkidrel_pointer).add(i as usize)).tvalue_set_tag_variant(TagVariant::NilNil);
        }
        (*other_state).interpreter_top.stkidrel_pointer = (*other_state).interpreter_stack.stkidrel_pointer;
        (*other_state).interpreter_stack_last.stkidrel_pointer =
            ((*other_state).interpreter_stack.stkidrel_pointer).add(BASIC_STACK_SIZE as usize);
        let callinfo = &mut (*other_state).interpreter_base_callinfo;
        callinfo.callinfo_previous = null_mut();
        callinfo.callinfo_next = callinfo.callinfo_previous;
        callinfo.callinfo_callstatus = CALLSTATUS_LUA as u16;
        callinfo.callinfo_function.stkidrel_pointer = (*other_state).interpreter_top.stkidrel_pointer;
        callinfo.callinfo_u.c.context_function = None;
        callinfo.callinfo_count_results = 0;
        (*(*other_state).interpreter_top.stkidrel_pointer).tvalue_set_tag_variant(TagVariant::NilNil);
        (*other_state).interpreter_top.stkidrel_pointer = ((*other_state).interpreter_top.stkidrel_pointer).add(1);
        (*other_state).interpreter_top.stkidrel_pointer;
        callinfo.callinfo_top.stkidrel_pointer = ((*other_state).interpreter_top.stkidrel_pointer).add(LUA_MINSTACK as usize);
        (*other_state).interpreter_callinfo = callinfo;
    }
}
pub unsafe fn freestack(state: *mut State) {
    unsafe {
        if ((*state).interpreter_stack.stkidrel_pointer).is_null() {
            return;
        }
        (*state).interpreter_callinfo = &mut (*state).interpreter_base_callinfo;
        freeci(state);
        (*state).free_memory(
            (*state).interpreter_stack.stkidrel_pointer as *mut std::ffi::c_void,
            ((((*state).interpreter_stack_last.stkidrel_pointer).offset_from((*state).interpreter_stack.stkidrel_pointer) as i32
                + EXTRA_STACK) as usize)
                .wrapping_mul(size_of::<TValue>()) as usize,
        );
    }
}
pub unsafe fn init_registry(state: *mut State, global: *mut Global) {
    unsafe {
        let registry: *mut Table = luah_new(state);
        let io: *mut TValue = &mut (*global).global_lregistry;
        let x_: *mut Table = registry;
        (*io).set_table(x_);
        luah_resize(state, registry, 2, 0);
        let mut val0: TValue = TValue::new(TagVariant::NilNil);
        let x0: *mut State = state;
        val0.set_object(x0 as *mut Object, TagVariant::State);
        obj2arr(registry, 0, &val0);
        let mut val1: TValue = TValue::new(TagVariant::NilNil);
        let x1: *mut Table = luah_new(state);
        val1.set_table(x1);
        obj2arr(registry, 1, &val1);
    }
}
pub unsafe fn f_luaopen(state: *mut State, mut _ud: *mut std::ffi::c_void) {
    unsafe {
        let global: *mut Global = (*state).interpreter_global;
        stack_init(state, state);
        init_registry(state, global);
        (*state).luas_init_state();
        luat_init(state);
        luax_init(state);
        (*global).global_gcstep = 0;
        (*global)
            .global_nonevalue
            .tvalue_set_tag_variant(TagVariant::NilNil);
    }
}
pub unsafe fn close_state(state: *mut State) {
    unsafe {
        let global: *mut Global = (*state).interpreter_global;
        if (*global)
            .global_nonevalue
            .get_tagvariant()
            .to_tag_type()
            .is_nil()
        {
            (*state).interpreter_callinfo = &mut (*state).interpreter_base_callinfo;
            (*state).interpreter_error_function = 0;
            do_close_protected(state, 1_i64, Status::OK);
            (*state).interpreter_top.stkidrel_pointer = ((*state).interpreter_stack.stkidrel_pointer).add(1);
            (*global).luac_freeallobjects(state);
        } else {
            (*global).luac_freeallobjects(state);
        }
        (*state).free_memory(
            (*(*state).interpreter_global)
                .global_stringtable
                .stringtable_hash as *mut std::ffi::c_void,
            (*(*state).interpreter_global)
                .global_stringtable
                .stringtable_size
                * size_of::<*mut TString>(),
        );
        freestack(state);
        std::alloc::dealloc(state as *mut u8, std::alloc::Layout::new::<State>());
        std::alloc::dealloc(global as *mut u8, std::alloc::Layout::new::<Global>());
    }
}
pub unsafe fn lua_newthread(state: *mut State) -> *mut State {
    unsafe {
        let global: *mut Global = (*state).interpreter_global;
        (*state).do_gc_step_if_should_step();
        let ret = luac_newobj(state, TagVariant::State, size_of::<State>()) as *mut State;
        let io: *mut TValue = &mut (*(*state).interpreter_top.stkidrel_pointer);
        (*io).set_object(ret as *mut Object, TagVariant::State);
        (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.add(1);
        (*ret).preinit_thread(global);
        write_volatile(
            &mut (*ret).interpreter_hook_mask as *mut i32,
            (*state).interpreter_hook_mask,
        );
        (*ret).interpreter_base_hookcount = (*state).interpreter_base_hookcount;
        write_volatile(
            &mut (*ret).interpreter_hook as *mut HookFunction,
            (*state).interpreter_hook,
        );
        (*ret).interpreter_hook_count = (*ret).interpreter_base_hookcount;
        stack_init(ret, state);
        ret
    }
}
pub unsafe fn luae_resetthread(state: *mut State, mut status: Status) -> Status {
    unsafe {
        (*state).interpreter_callinfo = &mut (*state).interpreter_base_callinfo;
        let callinfo = (*state).interpreter_callinfo;
        (*(*state).interpreter_stack.stkidrel_pointer).tvalue_set_tag_variant(TagVariant::NilNil);
        (*callinfo).callinfo_function.stkidrel_pointer = (*state).interpreter_stack.stkidrel_pointer;
        (*callinfo).callinfo_callstatus = CALLSTATUS_LUA as u16;
        if status == Status::Yield {
            status = Status::OK;
        }
        (*state).interpreter_status = Status::OK;
        (*state).interpreter_error_function = 0;
        status = do_close_protected(state, 1_i64, status);
        if status != Status::OK {
            (*state).set_error_object(status, ((*state).interpreter_stack.stkidrel_pointer).add(1));
        } else {
            (*state).interpreter_top.stkidrel_pointer = ((*state).interpreter_stack.stkidrel_pointer).add(1);
        }
        (*callinfo).callinfo_top.stkidrel_pointer = (*state)
            .interpreter_top
            .stkidrel_pointer
            .add(LUA_MINSTACK as usize);
        luad_reallocstack(
            state,
            ((*callinfo).callinfo_top.stkidrel_pointer).offset_from((*state).interpreter_stack.stkidrel_pointer) as i32,
            false,
        );
        status
    }
}
pub unsafe fn luad_throwbaselevel(state: *mut State, status: Status) -> ! {
    unsafe {
        if !(*state).interpreter_longjump.is_null() {
            std::panic::panic_any(LuaError {
                status,
                unwind_to_base: true,
            });
        } else {
            luad_throw(state, status);
        }
    }
}
pub unsafe fn lua_closethread(state: *mut State, from: *mut State) -> Status {
    unsafe {
        (*state).interpreter_count_yield = 0;
        (*state).interpreter_count_c_calls = if from.is_null() {
            0
        } else {
            (*from).interpreter_count_c_calls & INTERPRETER_C_CALLS_MASK
        };
        let status = luae_resetthread(state, (*state).interpreter_status);
        if state == from {
            luad_throwbaselevel(state, status);
        }
        status
    }
}
pub unsafe fn lua_close(mut state: *mut State) {
    unsafe {
        state = (*(*state).interpreter_global).global_maininterpreter;
        close_state(state);
    }
}
pub unsafe fn luae_warning(state: *mut State, message: *const i8, tocont: i32) {
    unsafe {
        let warn_function: WarnFunction = (*(*state).interpreter_global).global_warnfunction;
        if warn_function.is_some() {
            warn_function.expect("non-null function pointer")(
                (*(*state).interpreter_global).global_warnuserdata,
                message,
                tocont,
            );
        }
    }
}
pub unsafe fn luae_warnerror(state: *mut State, where_0: *const i8) {
    unsafe {
        let errobj: *mut TValue = &mut (*(*state).interpreter_top.stkidrel_pointer.sub(1));
        let message: *const i8 = if (*errobj).get_tagvariant().to_tag_type().is_string() {
            (*(*errobj).as_string().unwrap()).get_contents_mut() as *const i8
        } else {
            c"error object is not a string".as_ptr()
        };
        luae_warning(state, c"error in ".as_ptr(), 1);
        luae_warning(state, where_0, 1);
        luae_warning(state, c" (".as_ptr(), 1);
        luae_warning(state, message, 1);
        luae_warning(state, c")".as_ptr(), 0);
    }
}
pub unsafe fn lua_sethook(state: *mut State, mut function: HookFunction, mut mask: i32, count: i32) {
    unsafe {
        if function.is_none() || mask == 0 {
            mask = 0;
            function = None;
        }
        write_volatile(
            &mut (*state).interpreter_hook as *mut HookFunction,
            function,
        );
        (*state).interpreter_base_hookcount = count;
        (*state).interpreter_hook_count = (*state).interpreter_base_hookcount;
        write_volatile(
            &mut (*state).interpreter_hook_mask as *mut i32,
            mask as u8 as i32,
        );
        if mask != 0 {
            CallInfo::settraps((*state).interpreter_callinfo);
        }
    }
}
pub unsafe fn lua_gethook(state: *mut State) -> HookFunction {
    unsafe { (*state).interpreter_hook }
}
pub unsafe fn lua_gethookmask(state: *mut State) -> i32 {
    unsafe { (*state).interpreter_hook_mask }
}
pub unsafe fn lua_gethookcount(state: *mut State) -> i32 {
    unsafe { (*state).interpreter_base_hookcount }
}
pub unsafe fn lua_getstack(state: *mut State, mut level: i32, debuginfo: *mut DebugInfo) -> i32 {
    unsafe {
        let status: i32;
        let mut callinfo;
        if level < 0 {
            return 0;
        }
        callinfo = (*state).interpreter_callinfo;
        while level > 0 && !std::ptr::eq(callinfo, &(*state).interpreter_base_callinfo) {
            level -= 1;
            callinfo = (*callinfo).callinfo_previous;
        }
        if level == 0 && !std::ptr::eq(callinfo, &(*state).interpreter_base_callinfo) {
            status = 1;
            (*debuginfo).debuginfo_callinfo = callinfo;
        } else {
            status = 0;
        }
        status
    }
}
pub unsafe fn formatvarinfo(state: *mut State, kind: *const i8, name: *const i8) -> *const i8 {
    unsafe {
        if kind.is_null() {
            c"".as_ptr()
        } else {
            luao_pushfstring(state, c" (%s '%s')".as_ptr(), &[kind.into(), name.into()])
        }
    }
}
pub unsafe fn varinfo(state: *mut State, tvalue: *const TValue) -> *const i8 {
    unsafe {
        let callinfo = (*state).interpreter_callinfo;
        let mut name: *const i8 = null();
        let mut kind: *const i8 = null();
        if (*callinfo).callinfo_callstatus as i32 & CALLSTATUS_LUA == 0 {
            kind = CallInfo::getupvalname(callinfo, tvalue, &mut name);
            if kind.is_null() {
                let reg: i32 = CallInfo::in_stack(callinfo, tvalue);
                if reg >= 0 {
                    kind = getobjname(
                        (*(*(*callinfo).callinfo_function.stkidrel_pointer)
                            .as_closure()
                            .unwrap())
                        .closure_payload
                        .closurepayload_lprototype,
                        CallInfo::currentpc(callinfo),
                        reg,
                        &mut name,
                    );
                }
            }
        }
        formatvarinfo(state, kind, name)
    }
}
pub unsafe fn typeerror(state: *mut State, tvalue: *const TValue, op: *const i8, extra: *const i8) -> ! {
    unsafe {
        let t: *const i8 = luat_objtypename(state, tvalue);
        luag_runerror(
            state,
            c"attempt to %s a %s value%s".as_ptr(),
            &[op.into(), t.into(), extra.into()],
        );
    }
}
pub unsafe fn luag_typeerror(state: *mut State, tvalue: *const TValue, op: *const i8) -> ! {
    unsafe {
        typeerror(state, tvalue, op, varinfo(state, tvalue));
    }
}
pub unsafe fn luag_callerror(state: *mut State, tvalue: *const TValue) -> ! {
    unsafe {
        let callinfo = (*state).interpreter_callinfo;
        let mut name: *const i8 = null();
        let kind: *const i8 = CallInfo::funcnamefromcall(state, callinfo, &mut name);
        let extra: *const i8 = if !kind.is_null() {
            formatvarinfo(state, kind, name)
        } else {
            varinfo(state, tvalue)
        };
        typeerror(state, tvalue, c"call".as_ptr(), extra);
    }
}
pub unsafe fn luag_forerror(state: *mut State, tvalue: *const TValue, what: *const i8) -> ! {
    unsafe {
        luag_runerror(
            state,
            c"bad 'for' %s (number expected, got %s)".as_ptr(),
            &[what.into(), luat_objtypename(state, tvalue).into()],
        );
    }
}
pub unsafe fn luag_concaterror(state: *mut State, mut p1: *const TValue, p2: *const TValue) -> ! {
    unsafe {
        match (*p1).get_tagvariant().to_tag_type() {
            TagType::String | TagType::Numeric => {
                p1 = p2;
            }
            _ => {}
        }
        luag_typeerror(state, p1, c"concatenate".as_ptr());
    }
}
pub unsafe fn luag_opinterror(state: *mut State, p1: *const TValue, mut p2: *const TValue, message: *const i8) -> ! {
    unsafe {
        if !(*p1).get_tagvariant().to_tag_type().is_numeric() {
            p2 = p1;
        }
        luag_typeerror(state, p2, message);
    }
}
pub unsafe fn luag_tointerror(state: *mut State, p1: *const TValue, mut p2: *const TValue) -> ! {
    unsafe {
        let mut temp: i64 = 0;
        if F2I::Equal.convert_tv_i64(p1, &mut temp) == 0 {
            p2 = p1;
        }
        luag_runerror(
            state,
            c"number%s has no integer representation".as_ptr(),
            &[varinfo(state, p2).into()],
        );
    }
}
pub unsafe fn luag_ordererror(state: *mut State, p1: *const TValue, p2: *const TValue) -> ! {
    unsafe {
        let t1: *const i8 = luat_objtypename(state, p1);
        let t2: *const i8 = luat_objtypename(state, p2);
        if std::ffi::CStr::from_ptr(t1) == std::ffi::CStr::from_ptr(t2) {
            luag_runerror(
                state,
                c"attempt to compare two %s values".as_ptr(),
                &[t1.into()],
            );
        } else {
            luag_runerror(
                state,
                c"attempt to compare %s with %s".as_ptr(),
                &[t1.into(), t2.into()],
            );
        };
    }
}
pub unsafe fn luag_addinfo(state: *mut State, message: *const i8, src: *mut TString, line: i32) -> *const i8 {
    unsafe {
        if src.is_null() {
            luao_pushfstring(state, c"?:?: %s".as_ptr(), &[message.into()])
        } else {
            let mut buffer: [i8; LUA_IDSIZE] = [0; LUA_IDSIZE];
            luao_chunkid(
                buffer.as_mut_ptr(),
                (*src).get_contents_mut(),
                (*src).get_length(),
            );
            luao_pushfstring(
                state,
                c"%s:%d: %s".as_ptr(),
                &[buffer.as_mut_ptr().into(), line.into(), message.into()],
            )
        }
    }
}
pub unsafe fn luag_errormsg(state: *mut State) -> ! {
    unsafe {
        if (*state).interpreter_error_function != 0 {
            let error_function: *mut TValue = ((*state).interpreter_stack.stkidrel_pointer as *mut i8)
                .add((*state).interpreter_error_function as usize) as *mut TValue;
            let top_dst: *mut TValue = &mut (*(*state).interpreter_top.stkidrel_pointer);
            let below_top_src: *const TValue = &mut (*(*state).interpreter_top.stkidrel_pointer.sub(1));
            (*top_dst).copy_from(&*below_top_src);
            let below_top_dst: *mut TValue = &mut (*(*state).interpreter_top.stkidrel_pointer.sub(1));
            let errfn_src: *const TValue = &mut (*error_function);
            (*below_top_dst).copy_from(&*errfn_src);
            (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.add(1);
            luad_callnoyield(state, (*state).interpreter_top.stkidrel_pointer.sub(2), 1);
        }
        if (*(*state).interpreter_top.stkidrel_pointer.sub(1))
            .get_tagvariant()
            .to_tag_type()
            .is_nil()
        {
            lua_pushlstring(state, c"<no error object>".as_ptr(), 17);
            let io1: *mut TValue = &mut (*(*state).interpreter_top.stkidrel_pointer.sub(2));
            let io2: *const TValue = &(*(*state).interpreter_top.stkidrel_pointer.sub(1));
            (*io1).copy_from(&*io2);
            (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.sub(1);
        }
        luad_throw(state, Status::RuntimeError);
    }
}
pub unsafe fn luag_runerror(state: *mut State, fmt: *const i8, args: &[crate::fmtarg::FmtArg]) -> ! {
    unsafe {
        let callinfo = (*state).interpreter_callinfo;
        (*state).do_gc_step_if_should_step();
        let message = luao_pushvfstring(state, fmt, args);
        if (*callinfo).callinfo_callstatus as i32 & CALLSTATUS_LUA == 0 {
            luag_addinfo(
                state,
                message,
                (*(*(*(*callinfo).callinfo_function.stkidrel_pointer)
                    .as_closure()
                    .unwrap())
                .closure_payload
                .closurepayload_lprototype)
                    .prototype_source,
                CallInfo::getcurrentline(callinfo),
            );
            let io1: *mut TValue = &mut (*(*state).interpreter_top.stkidrel_pointer.sub(2));
            let io2: *const TValue = &mut (*(*state).interpreter_top.stkidrel_pointer.sub(1));
            (*io1).copy_from(&*io2);
            (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.sub(1);
        }
        luag_errormsg(state);
    }
}
pub unsafe fn luag_tracecall(state: *mut State) -> i32 {
    unsafe {
        let callinfo = (*state).interpreter_callinfo;
        let p: *mut Prototype = (*(*(*callinfo).callinfo_function.stkidrel_pointer)
            .as_closure()
            .unwrap())
        .closure_payload
        .closurepayload_lprototype;
        write_volatile(&mut (*callinfo).callinfo_u.l.trap as *mut i32, 1);
        if std::ptr::eq(
            (*callinfo).callinfo_u.l.saved_program_counter,
            (*p).prototype_code.vectort_pointer,
        ) {
            if (*p).prototype_isvariablearguments {
                return 0;
            } else if (*callinfo).callinfo_callstatus as i32 & CALLSTATUS_HOOKYIELD == 0 {
                luad_hookcall(state, callinfo);
            }
        }
        1
    }
}
pub unsafe fn luag_traceexec(state: *mut State, mut program_counter: *const u32) -> i32 {
    unsafe {
        let callinfo = (*state).interpreter_callinfo;
        let mask: u8 = (*state).interpreter_hook_mask as u8;
        let p: *const Prototype = (*(*(*callinfo).callinfo_function.stkidrel_pointer)
            .as_closure()
            .unwrap())
        .closure_payload
        .closurepayload_lprototype;
        if mask as i32 & (HOOKMASK_LINE | HOOKMASK_COUNT) == 0 {
            write_volatile(&mut (*callinfo).callinfo_u.l.trap as *mut i32, 0);
            return 0;
        }
        program_counter = program_counter.add(1);
        (*callinfo).callinfo_u.l.saved_program_counter = program_counter;
        let counthook: i32 = (mask as i32 & HOOKMASK_COUNT != 0 && {
            (*state).interpreter_hook_count -= 1;
            (*state).interpreter_hook_count == 0
        }) as i32;
        if counthook != 0 {
            (*state).interpreter_hook_count = (*state).interpreter_base_hookcount;
        } else if mask as i32 & HOOKMASK_LINE == 0 {
            return 1;
        }
        if (*callinfo).callinfo_callstatus as i32 & CALLSTATUS_HOOKYIELD != 0 {
            (*callinfo).callinfo_callstatus = ((*callinfo).callinfo_callstatus as i32 & !CALLSTATUS_HOOKYIELD) as u16;
            return 1;
        }
        if !(OPMODES[(*((*callinfo).callinfo_u.l.saved_program_counter).sub(1) & MASK_OP) as usize] as i32 & OPMODE_IT != 0
            && (*((*callinfo).callinfo_u.l.saved_program_counter).sub(1) >> POSITION_B & MASK_A) as i32 == 0)
        {
            (*state).interpreter_top.stkidrel_pointer = (*callinfo).callinfo_top.stkidrel_pointer;
        }
        if counthook != 0 {
            luad_hook(state, HOOKEVENT_COUNT, -1, 0, 0);
        }
        if mask as i32 & HOOKMASK_LINE != 0 {
            let old_program_counter: i32 = if (*state).interpreter_old_program_counter < (*p).prototype_code.get_size() as i32 {
                (*state).interpreter_old_program_counter
            } else {
                0
            };
            let npci: i32 = program_counter.offset_from((*p).prototype_code.vectort_pointer) as i32 - 1;
            if npci <= old_program_counter || changedline(p, old_program_counter, npci) != 0 {
                let newline: i32 = luag_getfuncline(p, npci);
                luad_hook(state, HOOKEVENT_LINE, newline, 0, 0);
            }
            (*state).interpreter_old_program_counter = npci;
        }
        if (*state).interpreter_status == Status::Yield {
            if counthook != 0 {
                (*state).interpreter_hook_count = 1;
            }
            (*callinfo).callinfo_callstatus = ((*callinfo).callinfo_callstatus as i32 | CALLSTATUS_HOOKYIELD) as u16;
            luad_throw(state, Status::Yield);
        }
        1
    }
}
pub unsafe fn intarith(state: *mut State, op: i32, v1: i64, v2: i64) -> i64 {
    unsafe {
        const ADD: i32 = OperatorBinary::Add as i32;
        const SUB: i32 = OperatorBinary::Subtract as i32;
        const MUL: i32 = OperatorBinary::Multiply as i32;
        const MOD: i32 = OperatorBinary::Modulus as i32;
        const IDIV: i32 = OperatorBinary::IntegralDivide as i32;
        const BAND: i32 = OperatorBinary::BitwiseAnd as i32;
        const BOR: i32 = OperatorBinary::BitwiseOr as i32;
        const BXOR: i32 = OperatorBinary::BitwiseExclusiveOr as i32;
        const SHL: i32 = OperatorBinary::ShiftLeft as i32;
        const SHR: i32 = OperatorBinary::ShiftRight as i32;
        match op {
            ADD => (v1 as usize).wrapping_add(v2 as usize) as i64,
            SUB => (v1 as usize).wrapping_sub(v2 as usize) as i64,
            MUL => (v1 as usize).wrapping_mul(v2 as usize) as i64,
            MOD => luav_mod(state, v1, v2),
            IDIV => luav_idiv(state, v1, v2),
            BAND => (v1 as usize & v2 as usize) as i64,
            BOR => (v1 as usize | v2 as usize) as i64,
            BXOR => (v1 as usize ^ v2 as usize) as i64,
            SHL => luav_shiftl(v1, v2),
            SHR => luav_shiftl(v1, (0usize).wrapping_sub(v2 as usize) as i64),
            ARITH_UNM => (0usize).wrapping_sub(v1 as usize) as i64,
            ARITH_BNOT => (!(0usize) ^ v1 as usize) as i64,
            _ => 0,
        }
    }
}
pub unsafe fn numarith(state: *mut State, op: i32, v1: f64, v2: f64) -> f64 {
    unsafe {
        const ADD: i32 = OperatorBinary::Add as i32;
        const SUB: i32 = OperatorBinary::Subtract as i32;
        const MUL: i32 = OperatorBinary::Multiply as i32;
        const MOD: i32 = OperatorBinary::Modulus as i32;
        const POW: i32 = OperatorBinary::Power as i32;
        const DIV: i32 = OperatorBinary::Divide as i32;
        const IDIV: i32 = OperatorBinary::IntegralDivide as i32;
        match op {
            ADD => v1 + v2,
            SUB => v1 - v2,
            MUL => v1 * v2,
            DIV => v1 / v2,
            POW => {
                if v2 == 2.0 {
                    v1 * v1
                } else {
                    v1.powf(v2)
                }
            }
            IDIV => (v1 / v2).floor(),
            ARITH_UNM => -v1,
            MOD => luav_modf(state, v1, v2),
            _ => 0.0,
        }
    }
}
pub unsafe fn luao_rawarith(state: *mut State, op: i32, p1: *const TValue, p2: *const TValue, res: *mut TValue) -> i32 {
    unsafe {
        const BITWISE_AND: i32 = OperatorBinary::BitwiseAnd as i32;
        const BITWISE_OR: i32 = OperatorBinary::BitwiseOr as i32;
        const BITWISE_XOR: i32 = OperatorBinary::BitwiseExclusiveOr as i32;
        const SHIFT_LEFT: i32 = OperatorBinary::ShiftLeft as i32;
        const SHIFT_RIGHT: i32 = OperatorBinary::ShiftRight as i32;
        const EQUAL: i32 = OperatorBinary::Equal as i32;
        const DIVIDE: i32 = OperatorBinary::Divide as i32;
        const POWER: i32 = OperatorBinary::Power as i32;
        match op {
            BITWISE_AND | BITWISE_OR | BITWISE_XOR | SHIFT_LEFT | SHIFT_RIGHT | EQUAL => {
                let mut i1: i64 = 0;
                let mut i2: i64 = 0;
                if (if (*p1).get_tagvariant() == TagVariant::NumericInteger {
                    i1 = (*p1).as_integer().unwrap();
                    1
                } else {
                    F2I::Equal.convert_tv_i64(p1, &mut i1)
                }) != 0
                    && (if (*p2).get_tagvariant() == TagVariant::NumericInteger {
                        i2 = (*p2).as_integer().unwrap();
                        1
                    } else {
                        F2I::Equal.convert_tv_i64(p2, &mut i2)
                    }) != 0
                {
                    (*res).set_integer(intarith(state, op, i1, i2));
                    1
                } else {
                    0
                }
            }
            DIVIDE | POWER => {
                let mut n1: f64 = 0.0;
                let mut n2: f64 = 0.0;
                if (if (*p1).get_tagvariant() == TagVariant::NumericNumber {
                    n1 = (*p1).as_number().unwrap();
                    1
                } else {
                    if (*p1).get_tagvariant() == TagVariant::NumericInteger {
                        n1 = (*p1).as_integer().unwrap() as f64;
                        1
                    } else {
                        0
                    }
                }) != 0
                    && (if (*p2).get_tagvariant() == TagVariant::NumericNumber {
                        n2 = (*p2).as_number().unwrap();
                        1
                    } else {
                        if (*p2).get_tagvariant() == TagVariant::NumericInteger {
                            n2 = (*p2).as_integer().unwrap() as f64;
                            1
                        } else {
                            0
                        }
                    }) != 0
                {
                    (*res).set_number(numarith(state, op, n1, n2));
                    1
                } else {
                    0
                }
            }
            _ => {
                let mut n1: f64 = 0.0;
                let mut n2: f64 = 0.0;
                if (*p1).get_tagvariant() == TagVariant::NumericInteger && (*p2).get_tagvariant() == TagVariant::NumericInteger {
                    let io: *mut TValue = res;
                    (*io).set_integer(intarith(
                        state,
                        op,
                        (*p1).as_integer().unwrap(),
                        (*p2).as_integer().unwrap(),
                    ));
                    1
                } else if (if (*p1).get_tagvariant() == TagVariant::NumericNumber {
                    n1 = (*p1).as_number().unwrap();
                    1
                } else {
                    if (*p1).get_tagvariant() == TagVariant::NumericInteger {
                        n1 = (*p1).as_integer().unwrap() as f64;
                        1
                    } else {
                        0
                    }
                }) != 0
                    && (if (*p2).get_tagvariant() == TagVariant::NumericNumber {
                        n2 = (*p2).as_number().unwrap();
                        1
                    } else {
                        if (*p2).get_tagvariant() == TagVariant::NumericInteger {
                            n2 = (*p2).as_integer().unwrap() as f64;
                            1
                        } else {
                            0
                        }
                    }) != 0
                {
                    let io: *mut TValue = res;
                    (*io).set_number(numarith(state, op, n1, n2));
                    1
                } else {
                    0
                }
            }
        }
    }
}
pub unsafe fn luao_pushvfstring(state: *mut State, mut fmt: *const i8, args: &[crate::fmtarg::FmtArg]) -> *const i8 {
    unsafe {
        let mut buff_fs = BuffFS::new(state);
        let mut e: *const i8;
        let mut arg_idx: usize = 0;
        loop {
            e = cstr_chr(fmt, Character::Percent as i8);
            if e.is_null() {
                break;
            }
            buff_fs.add_string(fmt, e.offset_from(fmt) as usize);
            match Character::from(*e.add(1) as i32) {
                Character::LowerS => {
                    let mut s: *const i8 = args[arg_idx].as_str();
                    arg_idx += 1;
                    if s.is_null() {
                        s = c"(null)".as_ptr();
                    }
                    buff_fs.add_string(s, cstr_len(s));
                }
                Character::LowerC => {
                    let c: i8 = args[arg_idx].as_int() as u8 as i8;
                    arg_idx += 1;
                    buff_fs.add_string(&c, 1_usize);
                }
                Character::LowerD => {
                    let mut tvalue: TValue = TValue::new(TagVariant::NilNil);
                    tvalue.set_integer(args[arg_idx].as_int() as i64);
                    arg_idx += 1;
                    buff_fs.add_number(&mut tvalue);
                }
                Character::UpperI => {
                    let mut tvalue: TValue = TValue::new(TagVariant::NilNil);
                    tvalue.set_integer(args[arg_idx].as_long());
                    arg_idx += 1;
                    buff_fs.add_number(&mut tvalue);
                }
                Character::LowerF => {
                    let mut tvalue: TValue = TValue::new(TagVariant::NilNil);
                    tvalue.set_number(args[arg_idx].as_float());
                    arg_idx += 1;
                    buff_fs.add_number(&mut tvalue);
                }
                Character::LowerP => {
                    let size = 3_usize
                        .wrapping_mul(size_of::<*mut std::ffi::c_void>())
                        .wrapping_add(8);
                    let bf: *mut i8 = buff_fs.get_raw(size);
                    let p: *mut std::ffi::c_void = args[arg_idx].as_ptr();
                    arg_idx += 1;
                    let length = snprintf_pointer(bf, size, p);
                    buff_fs.add_length(length as usize);
                }
                Character::UpperU => {
                    let mut bf: [i8; 8] = [0; 8];
                    let length: i32 = luao_utf8esc(bf.as_mut_ptr(), args[arg_idx].as_long() as usize);
                    arg_idx += 1;
                    buff_fs.add_string(bf.as_mut_ptr().add(8).sub(length as usize), length as usize);
                }
                Character::Percent => {
                    buff_fs.add_string(c"%".as_ptr(), 1_usize);
                }
                _ => {
                    luag_runerror(
                        state,
                        c"invalid option '%%%c' to 'lua_pushfstring'".as_ptr(),
                        &[(*e.add(1) as i32).into()],
                    );
                }
            }
            fmt = e.add(2);
        }
        buff_fs.add_string(fmt, cstr_len(fmt));
        buff_fs.clear();
        (*(*(*state).interpreter_top.stkidrel_pointer.sub(1))
            .as_string()
            .unwrap())
        .get_contents_mut()
    }
}
pub unsafe fn luao_pushfstring(state: *mut State, fmt: *const i8, args: &[crate::fmtarg::FmtArg]) -> *const i8 {
    unsafe { luao_pushvfstring(state, fmt, args) }
}
pub unsafe fn luat_init(state: *mut State) {
    unsafe {
        const EVENT_NAMES: [*const i8; 25] = [
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
            (*(*state).interpreter_global).global_tmname[i as usize] = luas_new(state, EVENT_NAMES[i as usize]);
            fix_object_state(
                state,
                &mut (*(*((*(*state).interpreter_global).global_tmname)
                    .as_mut_ptr()
                    .add(i as usize) as *mut Object)),
            );
        }
    }
}
pub unsafe fn luat_gettmbyobj(state: *mut State, tvalue: *const TValue, event: u32) -> *const TValue {
    unsafe {
        let metatable: *mut Table;
        match (*tvalue).get_tagvariant().to_tag_type() {
            TagType::Table => {
                metatable = (*(*tvalue).as_table().unwrap()).get_metatable();
            }
            TagType::User => {
                metatable = (*(*tvalue).as_user().unwrap()).get_metatable();
            }
            _ => {
                metatable = (*(*state).interpreter_global).global_metatables[(*tvalue).get_tagvariant().to_tag_type() as usize];
            }
        }
        if metatable.is_null() {
            &mut (*(*state).interpreter_global).global_nonevalue as *mut TValue as *const TValue
        } else {
            luah_getshortstr(
                metatable,
                (*(*state).interpreter_global).global_tmname[event as usize],
            )
        }
    }
}
pub unsafe fn luat_objtypename(state: *mut State, tvalue: *const TValue) -> *const i8 {
    unsafe {
        let mut metatable: *mut Table;
        if (*tvalue).get_tagvariant() == TagVariant::Table && {
            metatable = (*(*tvalue).as_table().unwrap()).get_metatable();
            !metatable.is_null()
        } || (*tvalue).get_tagvariant() == TagVariant::User && {
            metatable = (*(*tvalue).as_user().unwrap()).get_metatable();
            !metatable.is_null()
        } {
            let name: *const TValue = luah_getshortstr(metatable, luas_new(state, c"__name".as_ptr()));
            if (*name).get_tagvariant().to_tag_type().is_string() {
                return (*(*name).as_string().unwrap()).get_contents_mut();
            }
        }
        TagType::TYPE_NAMES[(*tvalue).get_tagvariant().to_tag_type() as usize + 1]
    }
}
pub unsafe fn luat_calltm(state: *mut State, f: *const TValue, p1: *const TValue, p2: *const TValue, p3: *const TValue) {
    unsafe {
        let function: *mut TValue = (*state).interpreter_top.stkidrel_pointer;
        let slot_fn: *mut TValue = &mut (*function);
        (*slot_fn).copy_from(&*f);
        let slot_p1: *mut TValue = &mut (*function.add(1));
        (*slot_p1).copy_from(&*p1);
        let slot_p2: *mut TValue = &mut (*function.add(2));
        (*slot_p2).copy_from(&*p2);
        let slot_p3: *mut TValue = &mut (*function.add(3));
        (*slot_p3).copy_from(&*p3);
        (*state).interpreter_top.stkidrel_pointer = function.add(4);
        if (*(*state).interpreter_callinfo).callinfo_callstatus as i32 & (CALLSTATUS_LUA | CALLSTATUS_FRESH) == 0 {
            ccall(state, function, 0, 1);
        } else {
            luad_callnoyield(state, function, 0);
        };
    }
}
pub unsafe fn luat_calltmres(state: *mut State, f: *const TValue, p1: *const TValue, p2: *const TValue, mut res: *mut TValue) {
    unsafe {
        let result: i64 = (res as *mut i8).offset_from((*state).interpreter_stack.stkidrel_pointer as *mut i8) as i64;
        let function: *mut TValue = (*state).interpreter_top.stkidrel_pointer;
        let slot_fn: *mut TValue = &mut (*function);
        (*slot_fn).copy_from(&*f);
        let slot_p1: *mut TValue = &mut (*function.add(1));
        (*slot_p1).copy_from(&*p1);
        let slot_p2: *mut TValue = &mut (*function.add(2));
        (*slot_p2).copy_from(&*p2);
        (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.add(3);
        if (*(*state).interpreter_callinfo).callinfo_callstatus as i32 & (CALLSTATUS_LUA | CALLSTATUS_FRESH) == 0 {
            ccall(state, function, 1, 1);
        } else {
            luad_callnoyield(state, function, 1);
        }
        res = ((*state).interpreter_stack.stkidrel_pointer as *mut i8).add(result as usize) as *mut TValue;
        let res_dst: *mut TValue = &mut (*res);
        (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.sub(1);
        let res_src: *const TValue = &mut (*(*state).interpreter_top.stkidrel_pointer);
        (*res_dst).copy_from(&(*res_src));
    }
}
pub unsafe fn callbintm(state: *mut State, p1: *const TValue, p2: *const TValue, res: *mut TValue, event: u32) -> i32 {
    unsafe {
        let mut tm: *const TValue = luat_gettmbyobj(state, p1, event);
        if (*tm).get_tagvariant().to_tag_type().is_nil() {
            tm = luat_gettmbyobj(state, p2, event);
        }
        if (*tm).get_tagvariant().to_tag_type().is_nil() {
            0
        } else {
            luat_calltmres(state, tm, p1, p2, res);
            1
        }
    }
}
pub unsafe fn luat_trybintm(state: *mut State, p1: *const TValue, p2: *const TValue, res: *mut TValue, event: u32) {
    unsafe {
        if callbintm(state, p1, p2, res, event) == 0 {
            match event {
                TM_BAND | TM_BOR | TM_BXOR | TM_SHL | TM_SHR | TM_BNOT => {
                    if (*p1).get_tagvariant().to_tag_type().is_numeric() && (*p2).get_tagvariant().to_tag_type().is_numeric() {
                        luag_tointerror(state, p1, p2);
                    } else {
                        luag_opinterror(state, p1, p2, c"perform bitwise operation on".as_ptr());
                    }
                }
                _ => {
                    luag_opinterror(state, p1, p2, c"perform arithmetic on".as_ptr());
                }
            }
        }
    }
}
pub unsafe fn luat_tryconcattm(state: *mut State) {
    unsafe {
        let top: *mut TValue = (*state).interpreter_top.stkidrel_pointer;
        if callbintm(state, &(*top.sub(2)), &(*top.sub(1)), top.sub(2), TM_CONCAT) == 0 {
            luag_concaterror(state, &(*top.sub(2)), &(*top.sub(1)));
        }
    }
}
pub unsafe fn luat_trybinassoctm(state: *mut State, p1: *const TValue, p2: *const TValue, flip: i32, res: *mut TValue, event: u32) {
    unsafe {
        if flip != 0 {
            luat_trybintm(state, p2, p1, res, event);
        } else {
            luat_trybintm(state, p1, p2, res, event);
        };
    }
}
pub unsafe fn luat_trybinitm(state: *mut State, p1: *const TValue, i2: i64, flip: i32, res: *mut TValue, event: u32) {
    unsafe {
        let mut aux: TValue = TValue::new(TagVariant::NilNil);
        let io: *mut TValue = &mut aux;
        (*io).set_integer(i2);
        luat_trybinassoctm(state, p1, &aux, flip, res, event);
    }
}
pub unsafe fn luat_callordertm(state: *mut State, p1: *const TValue, p2: *const TValue, event: u32) -> i32 {
    unsafe {
        if callbintm(
            state,
            p1,
            p2,
            (*state).interpreter_top.stkidrel_pointer,
            event,
        ) != 0
        {
            return !((*(*state).interpreter_top.stkidrel_pointer).get_tagvariant() == TagVariant::BooleanFalse
                || (*(*state).interpreter_top.stkidrel_pointer)
                    .get_tagvariant()
                    .to_tag_type()
                    .is_nil()) as i32;
        }
        luag_ordererror(state, p1, p2);
    }
}
pub unsafe fn luat_callorderitm(state: *mut State, mut p1: *const TValue, v2: i32, flip: i32, is_float: bool, event: u32) -> i32 {
    unsafe {
        let mut aux: TValue = TValue::new(TagVariant::NilNil);
        let p2: *const TValue;
        if is_float {
            let io: *mut TValue = &mut aux;
            (*io).set_number(v2 as f64);
        } else {
            let io: *mut TValue = &mut aux;
            (*io).set_integer(v2 as i64);
        }
        if flip != 0 {
            p2 = p1;
            p1 = &mut aux;
        } else {
            p2 = &mut aux;
        }
        luat_callordertm(state, p1, p2, event)
    }
}
pub unsafe fn luat_adjustvarargs(state: *mut State, nfixparams: i32, callinfo: *mut CallInfo, p: *const Prototype) {
    unsafe {
        let actual: i32 =
            ((*state).interpreter_top.stkidrel_pointer).offset_from((*callinfo).callinfo_function.stkidrel_pointer) as i32 - 1;
        let nextra: i32 = actual - nfixparams;
        if (*p).prototype_needsvarargtable {
            // Create a real table for the named vararg parameter
            let vararg_start = (*callinfo)
                .callinfo_function
                .stkidrel_pointer
                .add((nfixparams + 1) as usize);
            let table: *mut Table = luah_new(state);
            // Push the table on the stack temporarily
            let top = (*state).interpreter_top.stkidrel_pointer;
            (*top).set_table(table);
            (*state).interpreter_top.stkidrel_pointer = top.add(1);
            // Resize and fill the table
            if nextra > 0 {
                luah_resize(state, table, nextra as usize, 1);
                // Set t.n = nextra
                let mut key_n: TValue = core::mem::zeroed();
                let n_str = luas_new(state, c"n".as_ptr());
                key_n.set_object(n_str as *mut Object, (*n_str).get_tagvariant());
                let mut val_n: TValue = core::mem::zeroed();
                val_n.set_integer(nextra as i64);
                luah_set(state, table, &key_n, &mut val_n);
                for i in 0..nextra {
                    let mut val: TValue = core::mem::zeroed();
                    let src: *const TValue = &*vararg_start.add(i as usize);
                    val.copy_from(&*src);
                    luah_setint(state, table, (i + 1) as i64, &mut val);
                }
            } else {
                luah_resize(state, table, 0, 1);
                let mut key_n: TValue = core::mem::zeroed();
                let n_str = luas_new(state, c"n".as_ptr());
                key_n.set_object(n_str as *mut Object, (*n_str).get_tagvariant());
                let mut val_n: TValue = core::mem::zeroed();
                val_n.set_integer(0);
                luah_set(state, table, &key_n, &mut val_n);
            }
            // Move the table to the vararg parameter slot (func + nfixparams + 1)
            let table_src = (*state).interpreter_top.stkidrel_pointer.sub(1);
            let dest = (*callinfo)
                .callinfo_function
                .stkidrel_pointer
                .add((nfixparams + 1) as usize);
            (*dest).copy_from(&*table_src);
            (*state).interpreter_top.stkidrel_pointer = table_src;
        } else {
            // Traditional hidden vararg handling
            (*callinfo).callinfo_u.l.count_extra_arguments = nextra;
            if ((*state).interpreter_stack_last.stkidrel_pointer).offset_from((*state).interpreter_top.stkidrel_pointer)
                <= ((*p).prototype_maximumstacksize as i32 + 1) as isize
            {
                luad_growstack(state, (*p).prototype_maximumstacksize as i32 + 1, true);
            }
            let top = (*state).interpreter_top.stkidrel_pointer;
            (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.add(1);
            let io1: *mut TValue = &mut (*top);
            let io2: *const TValue = &mut (*(*callinfo).callinfo_function.stkidrel_pointer);
            (*io1).copy_from(&*io2);
            for i in 1..(1 + nfixparams) {
                let top = (*state).interpreter_top.stkidrel_pointer;
                (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.add(1);
                let io1: *mut TValue = &mut (*top);
                let io2: *const TValue = &mut (*((*callinfo).callinfo_function.stkidrel_pointer).add(i as usize));
                (*io1).copy_from(&*io2);
                (*((*callinfo).callinfo_function.stkidrel_pointer).add(i as usize)).tvalue_set_tag_variant(TagVariant::NilNil);
            }
            (*callinfo).callinfo_function.stkidrel_pointer =
                ((*callinfo).callinfo_function.stkidrel_pointer).add((actual + 1) as usize);
            (*callinfo).callinfo_top.stkidrel_pointer = ((*callinfo).callinfo_top.stkidrel_pointer).add((actual + 1) as usize);
            // Set vararg parameter to nil
            (*((*callinfo).callinfo_function.stkidrel_pointer).add((nfixparams + 1) as usize))
                .tvalue_set_tag_variant(TagVariant::NilNil);
        }
    }
}
pub unsafe fn luat_getvarargs(state: *mut State, callinfo: *mut CallInfo, mut where_0: *mut TValue, mut wanted: i32, vatab: i32) {
    unsafe {
        let h: *mut Table = if vatab >= 0 {
            let slot = (*callinfo)
                .callinfo_function
                .stkidrel_pointer
                .add((vatab + 1) as usize);
            (*slot).as_table().unwrap()
        } else {
            null_mut()
        };
        let nextra: i32 = if h.is_null() {
            (*callinfo).callinfo_u.l.count_extra_arguments
        } else {
            // Get count from table.n, with validation matching PUC 5.5 getnumargs
            let n_str = luas_new(state, c"n".as_ptr());
            let n_val = luah_getshortstr(h, n_str);
            if (*n_val).get_tagvariant() != TagVariant::NumericInteger
                || ((*n_val).as_integer().unwrap() as u64) > (i32::MAX / 2) as u64
            {
                luag_runerror(state, c"vararg table has no proper 'n'".as_ptr(), &[]);
            }
            (*n_val).as_integer().unwrap() as i32
        };
        if wanted < 0 {
            wanted = nextra;
            if ((*state).interpreter_stack_last.stkidrel_pointer).offset_from((*state).interpreter_top.stkidrel_pointer)
                <= nextra as isize
            {
                let t__: i64 = (where_0 as *mut i8).offset_from((*state).interpreter_stack.stkidrel_pointer as *mut i8) as i64;
                (*state).do_gc_step_if_should_step();
                luad_growstack(state, nextra, true);
                where_0 = ((*state).interpreter_stack.stkidrel_pointer as *mut i8).add(t__ as usize) as *mut TValue;
            }
            (*state).interpreter_top.stkidrel_pointer = where_0.add(nextra as usize);
        }
        let touse = wanted.min(nextra);
        if h.is_null() {
            // Get vararg values from the stack (hidden args)
            for i in 0..touse {
                let io1: *mut TValue = &mut (*where_0.add(i as usize));
                let io2: *const TValue = &mut (*((*callinfo).callinfo_function.stkidrel_pointer)
                    .sub(nextra as usize)
                    .add(i as usize));
                (*io1).copy_from(&*io2);
            }
        } else {
            // Get vararg values from the vararg table
            for i in 0..touse {
                let dest: *mut TValue = &mut (*where_0.add(i as usize));
                let tag = luah_getint(h, (i + 1) as i64, dest);
                if tag.to_tag_type().is_nil() {
                    (*dest).tvalue_set_tag_variant(TagVariant::NilNil);
                }
            }
        }
        for i in touse..wanted {
            (*where_0.add(i as usize)).tvalue_set_tag_variant(TagVariant::NilNil);
        }
    }
}
pub unsafe fn luac_newobj(state: *mut State, tagvariant: TagVariant, size: usize) -> *mut Object {
    unsafe {
        let global: *mut Global = (*state).interpreter_global;
        let ret = (*state).allocate(size) as *mut Object;
        (*ret).set_tagvariant(tagvariant);
        (*ret).set_marked((*global).global_current_white & WHITEBITS);
        (*ret).object_next = (*global).global_allgc;
        (*global).global_allgc = ret;
        ret
    }
}
pub unsafe fn traverse_state(global: *mut Global, state: *mut State) -> i32 {
    unsafe {
        let mut o: *mut TValue = (*state).interpreter_stack.stkidrel_pointer;
        if (*state).get_marked() & AGEBITS > AGE_SURVIVAL || (*global).global_gcstate as i32 == GCS_PROPAGATE {
            ObjectWithGCList::linkgclist_(
                &mut *(state as *mut ObjectWithGCList),
                (*state).getgclist(),
                &mut (*global).global_grayagain,
            );
        }
        if o.is_null() {
            return 1;
        }
        while o < (*state).interpreter_top.stkidrel_pointer {
            if ((*o).is_collectable()) && (*(*o).as_object().unwrap()).get_marked() & WHITEBITS != 0 {
                Object::really_mark_object(global, (*o).as_object().unwrap());
            }
            o = o.add(1);
        }
        let mut uv: *mut UpValue = (*state).interpreter_open_upvalue;
        while !uv.is_null() {
            if (*uv).get_marked() & WHITEBITS != 0 {
                Object::really_mark_object(global, &mut *(uv as *mut Object));
            }
            uv = (*uv).upvalue_u.upvalueb_open.upvalueba_next;
        }
        if (*global).global_gcstate as i32 == GCS_ATOMIC {
            if !(*global).global_is_emergency {
                (*state).luad_shrinkstack();
            }
            o = (*state).interpreter_top.stkidrel_pointer;
            while o < ((*state).interpreter_stack_last.stkidrel_pointer).add(5) {
                (*o).tvalue_set_tag_variant(TagVariant::NilNil);
                o = o.add(1);
            }
            if ((*state).interpreter_twups == state) && !((*state).interpreter_open_upvalue).is_null() {
                (*state).interpreter_twups = (*global).global_twups;
                (*global).global_twups = state;
            }
        }
        1 + ((*state).interpreter_stack_last.stkidrel_pointer).offset_from((*state).interpreter_stack.stkidrel_pointer) as i32
    }
}
pub unsafe fn sweeptolive(state: *mut State, mut p: *mut *mut Object) -> *mut *mut Object {
    unsafe {
        let old: *mut *mut Object = p;
        loop {
            p = (*state).sweep_list(p, 1, null_mut());
            if p != old {
                break;
            }
        }
        p
    }
}
pub unsafe fn dothecall(state: *mut State, mut _ud: *mut std::ffi::c_void) {
    unsafe {
        luad_callnoyield(state, (*state).interpreter_top.stkidrel_pointer.sub(2), 0);
    }
}
pub unsafe fn gctm_function(state: *mut State) {
    unsafe {
        let global: *mut Global = (*state).interpreter_global;

        let mut v: TValue = TValue::new(TagVariant::NilNil);
        let io: *mut TValue = &mut v;
        let i_g: *mut Object = (*global).udata2finalize();
        (*io).set_object(i_g, (*i_g).get_tagvariant());
        let tm: *const TValue = luat_gettmbyobj(state, &v, TM_GC);
        if !(*tm).get_tagvariant().to_tag_type().is_nil() {
            let oldah: u8 = (*state).interpreter_allow_hook;
            let oldgcstp: i32 = (*global).global_gcstep as i32;
            (*global).global_gcstep = ((*global).global_gcstep as i32 | 2) as u8;
            (*state).interpreter_allow_hook = 0;
            let top = (*state).interpreter_top.stkidrel_pointer;
            (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.add(1);
            let tm_dst: *mut TValue = &mut (*top);
            let tm_src: *const TValue = tm;
            (*tm_dst).copy_from(&*tm_src);
            let top = (*state).interpreter_top.stkidrel_pointer;
            (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.add(1);
            let val_dst: *mut TValue = &mut (*top);
            let val_src: *const TValue = &mut v;
            (*val_dst).copy_from(&(*val_src));
            (*(*state).interpreter_callinfo).callinfo_callstatus =
                ((*(*state).interpreter_callinfo).callinfo_callstatus as i32 | CALLSTATUS_LEQ) as u16;
            let status = luad_pcall(
                state,
                Some(dothecall as unsafe fn(*mut State, *mut std::ffi::c_void) -> ()),
                null_mut(),
                ((*state).interpreter_top.stkidrel_pointer.sub(2) as *mut i8)
                    .offset_from((*state).interpreter_stack.stkidrel_pointer as *mut i8) as i64,
                0,
            );
            (*(*state).interpreter_callinfo).callinfo_callstatus =
                ((*(*state).interpreter_callinfo).callinfo_callstatus as i32 & !CALLSTATUS_LEQ) as u16;
            (*state).interpreter_allow_hook = oldah;
            (*global).global_gcstep = oldgcstp as u8;
            if status != Status::OK {
                luae_warnerror(state, c"__gc".as_ptr());
                (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.sub(1);
            }
        }
    }
}
pub unsafe fn runafewfinalizers(state: *mut State, n: i32) -> i32 {
    unsafe {
        let global: *mut Global = (*state).interpreter_global;
        let mut i: i32;
        let tobefnz_ptr: *const *mut Object = &raw const (*global).global_tobefinalized;
        i = 0;
        while i < n && !std::ptr::read_volatile(tobefnz_ptr).is_null() {
            gctm_function(state);
            i += 1;
        }
        i
    }
}
pub unsafe fn luac_checkfinalizer(state: *mut State, o: *mut Object, metatable: *mut Table) {
    unsafe {
        let global: *mut Global = (*state).interpreter_global;
        if (*o).get_marked() & FINALIZEDBIT != 0
            || (if metatable.is_null() {
                null()
            } else {
                if (*metatable).table_flags as u32 & 1_u32 << TM_GC as i32 != 0 {
                    null()
                } else {
                    luat_gettm(metatable, TM_GC, (*global).global_tmname[TM_GC as usize])
                }
            })
            .is_null()
            || (*global).global_gcstep as i32 & 4 != 0
        {
        } else {
            if GCS_SWPALLGC <= (*global).global_gcstate as i32 && (*global).global_gcstate as i32 <= GCS_SWPEND {
                (*o).set_marked((*o).get_marked() & !(BLACKBIT | WHITEBITS) | ((*global).global_current_white & WHITEBITS));
                if std::ptr::eq((*global).global_sweepgc, &(*o).object_next) {
                    (*global).global_sweepgc = sweeptolive(state, (*global).global_sweepgc);
                }
            } else {
                (*global).correct_pointers(o);
                // Make 'o' new to avoid premature aging
                (*o).set_marked(
                    (*o).get_marked() & !(BLACKBIT | WHITEBITS | AGEBITS) | ((*global).global_current_white & WHITEBITS),
                );
            }
            let mut p: *mut *mut Object = &mut (*global).global_allgc;
            while *p != o {
                p = &mut (**p).object_next;
            }
            *p = (*o).object_next;
            (*o).object_next = (*global).global_finalizedobjects;
            (*global).global_finalizedobjects = o;
            (*o).set_marked((*o).get_marked() | FINALIZEDBIT);
        }
    }
}
pub unsafe fn sweep2old(state: *mut State, mut p: *mut *mut Object) {
    unsafe {
        let global: *mut Global = (*state).interpreter_global;
        loop {
            let head: *mut Object = std::ptr::read_volatile(p);
            if head.is_null() {
                break;
            }
            if (*head).get_marked() & WHITEBITS != 0 {
                *p = (*head).object_next;
                Object::object_free(state, head);
            } else {
                (*head).set_marked((*head).get_marked() & !AGEBITS | AGE_OLD);
                if (*head).get_tagvariant() == TagVariant::State {
                    let other_state: *mut State = &mut *(head as *mut State);
                    ObjectWithGCList::linkgclist_(
                        &mut *(other_state as *mut ObjectWithGCList),
                        (*other_state).getgclist(),
                        &mut (*global).global_grayagain,
                    );
                } else if (*head).get_tagvariant() == TagVariant::UpValue
                    && (*(head as *mut UpValue)).upvalue_v.upvaluea_p
                        != std::ptr::addr_of_mut!((*(head as *mut UpValue)).upvalue_u.upvalueb_value)
                {
                    (*head).set_marked((*head).get_marked() & !(BLACKBIT | WHITEBITS));
                } else {
                    (*head).set_marked((*head).get_marked() | BLACKBIT);
                }
                p = &mut (*head).object_next;
            }
        }
    }
}
pub unsafe fn sweepgen(
    state: *mut State,
    global: *mut Global,
    mut p: *mut *mut Object,
    limit: *mut Object,
    pfirstold1: *mut *mut Object,
    paddedold: *mut i64,
) -> *mut *mut Object {
    unsafe {
        static NEXT_AGE: [u8; 7] = [1, 3, 3, 4, 4, 5, 6];
        let white = (*global).global_current_white & WHITEBITS;
        loop {
            let head: *mut Object = std::ptr::read_volatile(p);
            if head == limit {
                break;
            }
            if (*head).get_marked() & WHITEBITS != 0 {
                *p = (*head).object_next;
                Object::object_free(state, head);
            } else {
                let age = (*head).get_marked() & AGEBITS;
                if age == AGE_NEW {
                    let marked = (*head).get_marked() & !(BLACKBIT | WHITEBITS | AGEBITS);
                    (*head).set_marked(marked | AGE_SURVIVAL | white);
                } else {
                    (*head).set_marked((*head).get_marked() & !AGEBITS | NEXT_AGE[age as usize]);
                    if (*head).get_marked() & AGEBITS == AGE_OLD1 {
                        *paddedold += Object::objsize(head);
                        if (*pfirstold1).is_null() {
                            *pfirstold1 = head;
                        }
                    }
                }
                p = &mut (*head).object_next;
            }
        }
        p
    }
}
pub unsafe fn callclosemethod(state: *mut State, obj: *mut TValue, err: *mut TValue, yy: i32) {
    unsafe {
        let mut top: *mut TValue = (*state).interpreter_top.stkidrel_pointer;
        let func = top;
        let tm: *const TValue = luat_gettmbyobj(state, obj, TM_CLOSE);
        (*top).copy_from(&*tm);
        top = top.add(1);
        (*top).copy_from(&*obj);
        top = top.add(1);
        if !err.is_null() {
            (*top).copy_from(&*err);
            top = top.add(1);
        }
        (*state).interpreter_top.stkidrel_pointer = top;
        if yy != 0 {
            ccall(state, func, 0, 1);
        } else {
            luad_callnoyield(state, func, 0);
        };
    }
}
pub unsafe fn checkclosemth(state: *mut State, level: *mut TValue) {
    unsafe {
        let tm: *const TValue = luat_gettmbyobj(state, &(*level), TM_CLOSE);
        if (*tm).get_tagvariant().to_tag_type().is_nil() {
            let index: i32 = level.offset_from(
                (*(*state).interpreter_callinfo)
                    .callinfo_function
                    .stkidrel_pointer,
            ) as i32;
            let mut vname: *const i8 = CallInfo::luag_findlocal(state, (*state).interpreter_callinfo, index, null_mut());
            if vname.is_null() {
                vname = c"?".as_ptr();
            }
            luag_runerror(
                state,
                c"variable '%s' got a non-closable value".as_ptr(),
                &[vname.into()],
            );
        }
    }
}
pub unsafe fn prepcallclosemth(state: *mut State, level: *mut TValue, status: Status, yy: i32) {
    unsafe {
        let uv: *mut TValue = &mut (*level);
        let errobj: *mut TValue;
        if status == Status::OK {
            (*state).interpreter_top.stkidrel_pointer = level.add(1);
            errobj = null_mut();
        } else if status == Status::Closing {
            errobj = null_mut();
        } else {
            errobj = &mut (*level.add(1));
            (*state).set_error_object(status, level.add(1));
        }
        callclosemethod(state, uv, errobj, yy);
    }
}
pub unsafe fn luaf_newtbcupval(state: *mut State, level: *mut TValue) {
    unsafe {
        if (*level).get_tagvariant() == TagVariant::BooleanFalse || (*level).get_tagvariant().to_tag_type().is_nil() {
            return;
        }
        checkclosemth(state, level);
        while level.offset_from((*state).interpreter_tbclist.stkidrel_pointer) as usize > MAXDELTA {
            (*state).interpreter_tbclist.stkidrel_pointer = ((*state).interpreter_tbclist.stkidrel_pointer).add(MAXDELTA);
            (*(*state).interpreter_tbclist.stkidrel_pointer).tvalue_delta = 0;
        }
        (*level).tvalue_delta = level.offset_from((*state).interpreter_tbclist.stkidrel_pointer) as u16;
        (*state).interpreter_tbclist.stkidrel_pointer = level;
    }
}
pub unsafe fn luaf_closeupval(state: *mut State, level: *mut TValue) {
    unsafe {
        loop {
            let uv: *mut UpValue = std::ptr::read_volatile(&(*state).interpreter_open_upvalue);
            let upl: *mut TValue;
            if !(!uv.is_null() && {
                upl = (*uv).upvalue_v.upvaluea_p as *mut TValue;
                upl >= level
            }) {
                break;
            }
            (*uv).luaf_unlinkupval();
            let slot: *mut TValue = std::ptr::addr_of_mut!((*uv).upvalue_u.upvalueb_value);
            let io2: *const TValue = (*uv).upvalue_v.upvaluea_p;
            std::ptr::copy_nonoverlapping(io2, slot, 1);
            (*uv).upvalue_v.upvaluea_p = slot;
            if (*uv).get_marked() & WHITEBITS == 0 {
                (*uv).set_marked((*uv).get_marked() | BLACKBIT);
                if (*slot).is_collectable()
                    && (*uv).get_marked() & BLACKBIT != 0
                    && (*(*slot).as_object().unwrap()).get_marked() & WHITEBITS != 0
                {
                    Object::luac_barrier_(
                        state,
                        &mut *(uv as *mut Object),
                        &mut (*((*slot).as_object().unwrap())),
                    );
                }
            }
        }
    }
}
pub unsafe fn poptbclist(state: *mut State) {
    unsafe {
        let mut tbc: *mut TValue = (*state).interpreter_tbclist.stkidrel_pointer;
        tbc = tbc.sub((*tbc).tvalue_delta as usize);
        while tbc > (*state).interpreter_stack.stkidrel_pointer && (*tbc).tvalue_delta == 0 {
            tbc = tbc.sub(MAXDELTA);
        }
        (*state).interpreter_tbclist.stkidrel_pointer = tbc;
    }
}
pub unsafe fn luaf_close(state: *mut State, mut level: *mut TValue, status: Status, yy: i32) -> *mut TValue {
    unsafe {
        let levelrel: i64 = (level as *mut i8).offset_from((*state).interpreter_stack.stkidrel_pointer as *mut i8) as i64;
        luaf_closeupval(state, level);
        let tbclist_ptr: *const *mut TValue = &raw const (*state).interpreter_tbclist.stkidrel_pointer;
        while std::ptr::read_volatile(tbclist_ptr) >= level {
            let tbc: *mut TValue = std::ptr::read_volatile(tbclist_ptr);
            poptbclist(state);
            prepcallclosemth(state, tbc, status, yy);
            level = ((*state).interpreter_stack.stkidrel_pointer as *mut i8).add(levelrel as usize) as *mut TValue;
        }
        level
    }
}
pub unsafe fn luay_parser(
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
        let closure: *mut Closure = Closure::luaf_newlclosure(state, 1);
        let io: *mut TValue = &mut (*(*state).interpreter_top.stkidrel_pointer);
        let x_: *mut Closure = closure;
        (*io).set_object(x_ as *mut Object, TagVariant::ClosureL);
        (*state).luad_inctop();
        lexstate.lexicalstate_table = luah_new(state);
        let io_table: *mut TValue = &mut (*(*state).interpreter_top.stkidrel_pointer);
        let x0: *mut Table = lexstate.lexicalstate_table;
        (*io_table).set_table(x0);
        (*state).luad_inctop();
        (*closure).closure_payload.closurepayload_lprototype = luaf_newproto(state);
        funcstate.functionstate_prototype = (*closure).closure_payload.closurepayload_lprototype;
        if (*closure).get_marked() & BLACKBIT != 0
            && (*(*closure).closure_payload.closurepayload_lprototype).get_marked() & WHITEBITS != 0
        {
            Object::luac_barrier_(
                state,
                &mut *(closure as *mut Object),
                &mut *((*closure).closure_payload.closurepayload_lprototype as *mut Object),
            );
        }
        (*funcstate.functionstate_prototype).prototype_source = luas_new(state, name);
        if (*funcstate.functionstate_prototype).get_marked() & BLACKBIT != 0
            && (*(*funcstate.functionstate_prototype).prototype_source).get_marked() & WHITEBITS != 0
        {
            Object::luac_barrier_(
                state,
                &mut *(funcstate.functionstate_prototype as *mut Object),
                &mut *((*funcstate.functionstate_prototype).prototype_source as *mut Object),
            );
        }
        lexstate.lexicalstate_buffer = buffer;
        lexstate.lexicalstate_dynamicdata = dynamic_data;
        (*dynamic_data).dynamicdata_labels.zero_length();
        (*dynamic_data)
            .dynamicdata_goto
            .set_length((*dynamic_data).dynamicdata_labels.get_length());
        (*dynamic_data)
            .dynamicdata_active_variables
            .set_length((*dynamic_data).dynamicdata_goto.get_length());
        luax_setinput(
            state,
            &mut lexstate,
            zio,
            (*funcstate.functionstate_prototype).prototype_source,
            firstchar,
        );
        handle_main_function(state, &mut lexstate, &mut funcstate);
        (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.sub(1);
        closure
    }
}
pub unsafe fn luax_init(state: *mut State) {
    unsafe {
        let env_string: *mut TString = luas_newlstr(state, c"_ENV".as_ptr(), "_ENV".len());
        fix_object_state(state, &mut *(env_string as *mut Object));
        let mut i: i32 = 0;
        while i < Token::While as i32 - FIRST_RESERVED + 1 {
            let tstring: *mut TString = luas_new(state, TOKENS[i as usize]);
            fix_object_state(state, &mut *(tstring as *mut Object));
            (*tstring).set_extra((i + 1) as u8);
            i += 1;
        }
    }
}
pub unsafe fn pushclosure(state: *mut State, p: *mut Prototype, encup: *mut *mut UpValue, base: *mut TValue, ra: *mut TValue) {
    unsafe {
        let count_upvalues = (*p).prototype_upvalues.get_size();
        let uv: *mut UpValueDescription = (*p).prototype_upvalues.vectort_pointer;
        let ncl: *mut Closure = Closure::luaf_newlclosure(state, count_upvalues as i32);
        (*ncl).closure_payload.closurepayload_lprototype = p;
        let io: *mut TValue = &mut (*ra);
        (*io).set_object(ncl as *mut Object, TagVariant::ClosureL);
        for i in 0..count_upvalues {
            if (*uv.add(i)).upvaluedescription_isinstack {
                let upvalue_slot = &mut *((*ncl).closure_upvalues)
                    .closureupvalue_lvalues
                    .as_mut_ptr()
                    .add(i);
                *upvalue_slot = luaf_findupval(
                    state,
                    base.add((*uv.add(i)).upvaluedescription_index as usize),
                );
            } else {
                let upvalue_slot = &mut *((*ncl).closure_upvalues)
                    .closureupvalue_lvalues
                    .as_mut_ptr()
                    .add(i);
                *upvalue_slot = *encup.add((*uv.add(i)).upvaluedescription_index as usize);
            }
            if (*ncl).get_marked() & BLACKBIT != 0
                && (**((*ncl).closure_upvalues)
                    .closureupvalue_lvalues
                    .as_mut_ptr()
                    .add(i))
                .get_marked()
                    & WHITEBITS
                    != 0
            {
                Object::luac_barrier_(
                    state,
                    &mut *(ncl as *mut Object),
                    &mut *(*((*ncl).closure_upvalues)
                        .closureupvalue_lvalues
                        .as_mut_ptr()
                        .add(i) as *mut Object),
                );
            }
        }
    }
}
pub unsafe fn luav_finishop(state: *mut State) {
    unsafe {
        let callinfo = (*state).interpreter_callinfo;
        let base: *mut TValue = ((*callinfo).callinfo_function.stkidrel_pointer).add(1);
        let inst: u32 = *((*callinfo).callinfo_u.l.saved_program_counter).sub(1);
        let op: u32 = inst & MASK_OP;
        match op {
            OPCODE_MMBIN | OPCODE_MMBINI | OPCODE_MMBINK => {
                let io1: *mut TValue =
                    &mut (*base.add((*((*callinfo).callinfo_u.l.saved_program_counter).sub(2) >> POSITION_A & MASK_A) as usize));
                (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.sub(1);
                let io2: *const TValue = &mut (*(*state).interpreter_top.stkidrel_pointer);
                (*io1).copy_from(&*io2);
            }
            OPCODE_UNM
            | OPCODE_BNOT
            | OPCODE_LEN
            | OPCODE_GET_TABLE_UPVALUE
            | OPCODE_GET_TABLE
            | OPCODE_INDEX_INTEGER
            | OPCODE_GET_FIELD
            | OPCODE_SELF => {
                let io1: *mut TValue = &mut (*base.add((inst >> POSITION_A & MASK_A) as usize));
                (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.sub(1);
                let io2: *const TValue = &mut (*(*state).interpreter_top.stkidrel_pointer);
                (*io1).copy_from(&*io2);
            }
            OPCODE_LT | OPCODE_LE | OPCODE_LTI | OPCODE_LEI | OPCODE_GTI | OPCODE_GEI | OPCODE_EQ => {
                let res: i32 = !((*(*state).interpreter_top.stkidrel_pointer.sub(1)).get_tagvariant() == TagVariant::BooleanFalse
                    || (*(*state).interpreter_top.stkidrel_pointer.sub(1))
                        .get_tagvariant()
                        .to_tag_type()
                        == TagType::Nil) as i32;
                (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.sub(1);
                if res != (inst >> POSITION_K & MASK_K) as i32 {
                    (*callinfo).callinfo_u.l.saved_program_counter = ((*callinfo).callinfo_u.l.saved_program_counter).add(1);
                    (*callinfo).callinfo_u.l.saved_program_counter;
                }
            }
            OPCODE_CONCAT => {
                let top: *mut TValue = (*state).interpreter_top.stkidrel_pointer.sub(1);
                let a: i32 = (inst >> POSITION_A & MASK_A) as i32;
                let total: i32 = top.sub(1).offset_from(base.add(a as usize)) as i32;
                let io1: *mut TValue = &mut (*top.sub(2));
                let io2: *const TValue = &mut (*top);
                (*io1).copy_from(&(*io2));
                (*state).interpreter_top.stkidrel_pointer = top.sub(1);
                concatenate(state, total);
            }
            OPCODE_CLOSE => {
                (*callinfo).callinfo_u.l.saved_program_counter = ((*callinfo).callinfo_u.l.saved_program_counter).sub(1);
                (*callinfo).callinfo_u.l.saved_program_counter;
            }
            OPCODE_RETURN => {
                let ra: *mut TValue = base.add((inst >> POSITION_A & MASK_A) as usize);
                (*state).interpreter_top.stkidrel_pointer = ra.add((*callinfo).callinfo_u2.callinfoconstituentb_nres as usize);
                (*callinfo).callinfo_u.l.saved_program_counter = ((*callinfo).callinfo_u.l.saved_program_counter).sub(1);
                (*callinfo).callinfo_u.l.saved_program_counter;
            }
            _ => {}
        };
    }
}
pub unsafe fn luav_execute(state: *mut State, mut callinfo: *mut CallInfo) {
    unsafe {
        let mut i: u32;
        let mut ra_call: *mut TValue;
        let mut new_call_info: *mut CallInfo;
        let mut b_call: i32;
        let mut count_results: i32;
        let mut tfor_action: usize;
        let mut closure: *mut Closure;
        let mut k: *mut TValue;
        let mut base: *mut TValue;
        let mut program_counter: *const u32;
        let mut trap: i32;
        '_startfunc: loop {
            trap = (*state).interpreter_hook_mask;
            '_returning: loop {
                closure = (*(*callinfo).callinfo_function.stkidrel_pointer)
                    .as_closure()
                    .unwrap();
                k = (*(*closure).closure_payload.closurepayload_lprototype)
                    .prototype_constants
                    .vectort_pointer;
                program_counter = (*callinfo).callinfo_u.l.saved_program_counter;
                if trap != 0 {
                    trap = luag_tracecall(state);
                }
                base = ((*callinfo).callinfo_function.stkidrel_pointer).add(1);
                loop {
                    if trap != 0 {
                        trap = luag_traceexec(state, program_counter);
                        base = ((*callinfo).callinfo_function.stkidrel_pointer).add(1);
                    }
                    let current_pc = program_counter;
                    program_counter = program_counter.add(1);
                    i = *current_pc;
                    match i & MASK_OP {
                        OPCODE_MOVE => {
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            let io1: *mut TValue = &mut (*ra);
                            let io2: *const TValue = &mut (*base.add((i >> POSITION_B & MASK_A) as usize));
                            (*io1).copy_from(&*io2);
                            continue;
                        }
                        OPCODE_LOADI => {
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            let b: i64 = ((i >> POSITION_K & MASK_BX) as i32 - (OFFSET_SBX)) as i64;
                            let io: *mut TValue = &mut (*ra);
                            (*io).set_integer(b);
                            continue;
                        }
                        OPCODE_LOADF => {
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            let b: i32 = (i >> POSITION_K & MASK_BX) as i32 - (OFFSET_SBX);
                            let io: *mut TValue = &mut (*ra);
                            (*io).set_number(b as f64);
                            continue;
                        }
                        OPCODE_LOADK => {
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            let rb: *mut TValue = k.add((i >> POSITION_K & MASK_BX) as usize);
                            let io1: *mut TValue = &mut (*ra);
                            let io2: *const TValue = rb;
                            (*io1).copy_from(&(*io2));
                            continue;
                        }
                        OPCODE_LOADKX => {
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            let rb: *mut TValue = k.add((*program_counter >> POSITION_A & MASK_AX) as usize);
                            program_counter = program_counter.add(1);
                            let io1: *mut TValue = &mut (*ra);
                            let io2: *const TValue = rb;
                            (*io1).copy_from(&(*io2));
                            continue;
                        }
                        OPCODE_LOAD_FALSE => {
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            (*ra).tvalue_set_tag_variant(TagVariant::BooleanFalse);
                            continue;
                        }
                        OPCODE_LFALSESKIP => {
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            (*ra).tvalue_set_tag_variant(TagVariant::BooleanFalse);
                            program_counter = program_counter.add(1);
                            continue;
                        }
                        OPCODE_LOAD_TRUE => {
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            (*ra).tvalue_set_tag_variant(TagVariant::BooleanTrue);
                            continue;
                        }
                        OPCODE_LOADNIL => {
                            let mut ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            let mut b: i32 = (i >> POSITION_B & MASK_A) as i32;
                            loop {
                                let slot = ra;
                                ra = ra.add(1);
                                (*slot).tvalue_set_tag_variant(TagVariant::NilNil);
                                let prev_b = b;
                                b -= 1;
                                if prev_b == 0 {
                                    break;
                                }
                            }
                            continue;
                        }
                        OPCODE_GET_UPVALUE => {
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            let b: i32 = (i >> POSITION_B & MASK_A) as i32;
                            let io1: *mut TValue = &mut (*ra);
                            let io2: *const TValue = (**((*closure).closure_upvalues)
                                .closureupvalue_lvalues
                                .as_mut_ptr()
                                .add(b as usize))
                            .upvalue_v
                            .upvaluea_p;
                            (*io1).copy_from(&(*io2));
                            continue;
                        }
                        OPCODE_SETUPVAL => {
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            let uv: *mut UpValue = *((*closure).closure_upvalues)
                                .closureupvalue_lvalues
                                .as_mut_ptr()
                                .add((i >> POSITION_B & MASK_A) as usize);
                            let io1: *mut TValue = (*uv).upvalue_v.upvaluea_p;
                            let io2: *const TValue = &mut (*ra);
                            (*io1).copy_from(&(*io2));
                            if (*ra).is_collectable()
                                && (*uv).get_marked() & BLACKBIT != 0
                                && (*(*ra).as_object().unwrap()).get_marked() & WHITEBITS != 0
                            {
                                Object::luac_barrier_(state, &mut *(uv as *mut Object), (*ra).as_object().unwrap());
                            }
                            continue;
                        }
                        OPCODE_GET_TABLE_UPVALUE => {
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            let count_upvalues: *mut TValue = (**((*closure).closure_upvalues)
                                .closureupvalue_lvalues
                                .as_mut_ptr()
                                .add((i >> POSITION_B & MASK_A) as usize))
                            .upvalue_v
                            .upvaluea_p;
                            let rc: *mut TValue = k.add((i >> POSITION_C & MASK_A) as usize);
                            let key: *mut TString = (*rc).as_string().unwrap();
                            let tag: TagVariant;
                            if !((*count_upvalues).get_tagvariant() == TagVariant::Table) {
                                tag = TagVariant::NilNil;
                            } else {
                                let slot = luah_getshortstr(&mut (*(*count_upvalues).as_table().unwrap()), key);
                                if !(*slot).get_tagvariant().to_tag_type().is_nil() {
                                    (*ra).copy_from(&*slot);
                                    tag = (*slot).get_tagvariant();
                                } else {
                                    tag = (*slot).get_tagvariant();
                                }
                            }
                            if tag.to_tag_type().is_nil() {
                                (*callinfo).callinfo_u.l.saved_program_counter = program_counter;
                                (*state).interpreter_top.stkidrel_pointer = (*callinfo).callinfo_top.stkidrel_pointer;
                                luav_finishget(state, count_upvalues, rc, ra, tag);
                                trap = (*callinfo).callinfo_u.l.trap;
                            }
                            continue;
                        }
                        OPCODE_GET_TABLE => {
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            let rb: *mut TValue = &mut (*base.add((i >> POSITION_B & MASK_A) as usize));
                            let rc: *mut TValue = &mut (*base.add((i >> POSITION_C & MASK_A) as usize));
                            let tag: TagVariant;
                            if (*rc).get_tagvariant() == TagVariant::NumericInteger {
                                let n = (*rc).as_integer().unwrap() as usize;
                                if (*rb).get_tagvariant() != TagVariant::Table {
                                    tag = TagVariant::NilNil;
                                } else {
                                    tag = luah_getint(&mut (*(*rb).as_table().unwrap()), n as i64, ra);
                                }
                            } else if (*rb).get_tagvariant() != TagVariant::Table {
                                tag = TagVariant::NilNil;
                            } else {
                                tag = luah_get(&mut (*(*rb).as_table().unwrap()), rc, ra);
                            }
                            if tag.to_tag_type().is_nil() {
                                (*callinfo).callinfo_u.l.saved_program_counter = program_counter;
                                (*state).interpreter_top.stkidrel_pointer = (*callinfo).callinfo_top.stkidrel_pointer;
                                luav_finishget(state, rb, rc, ra, tag);
                                trap = (*callinfo).callinfo_u.l.trap;
                            }
                            continue;
                        }
                        OPCODE_INDEX_INTEGER => {
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            let rb: *mut TValue = &mut (*base.add((i >> POSITION_B & MASK_A) as usize));
                            let c: i32 = (i >> POSITION_C & MASK_A) as i32;
                            let tag: TagVariant;
                            if (*rb).get_tagvariant() != TagVariant::Table {
                                tag = TagVariant::NilNil;
                            } else {
                                tag = luah_getint(&mut (*(*rb).as_table().unwrap()), c as i64, ra);
                            }
                            if tag.to_tag_type().is_nil() {
                                let mut key: TValue = TValue::new(TagVariant::NilNil);
                                key.set_integer(c as i64);
                                (*callinfo).callinfo_u.l.saved_program_counter = program_counter;
                                (*state).interpreter_top.stkidrel_pointer = (*callinfo).callinfo_top.stkidrel_pointer;
                                luav_finishget(state, rb, &mut key, ra, tag);
                                trap = (*callinfo).callinfo_u.l.trap;
                            }
                            continue;
                        }
                        OPCODE_GET_FIELD => {
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            let rb: *mut TValue = &mut (*base.add((i >> POSITION_B & MASK_A) as usize));
                            let rc: *mut TValue = k.add((i >> POSITION_C & MASK_A) as usize);
                            let key: *mut TString = (*rc).as_string().unwrap();
                            let tag: TagVariant;
                            if (*rb).get_tagvariant() != TagVariant::Table {
                                tag = TagVariant::NilNil;
                            } else {
                                let slot = luah_getshortstr(&mut (*(*rb).as_table().unwrap()), key);
                                if !(*slot).get_tagvariant().to_tag_type().is_nil() {
                                    (*ra).copy_from(&*slot);
                                    tag = (*slot).get_tagvariant();
                                } else {
                                    tag = (*slot).get_tagvariant();
                                }
                            }
                            if tag.to_tag_type().is_nil() {
                                (*callinfo).callinfo_u.l.saved_program_counter = program_counter;
                                (*state).interpreter_top.stkidrel_pointer = (*callinfo).callinfo_top.stkidrel_pointer;
                                luav_finishget(state, rb, rc, ra, tag);
                                trap = (*callinfo).callinfo_u.l.trap;
                            }
                            continue;
                        }
                        OPCODE_SETTABUP => {
                            let hres: i32;
                            let upval: *mut TValue = (**((*closure).closure_upvalues)
                                .closureupvalue_lvalues
                                .as_mut_ptr()
                                .add((i >> POSITION_A & MASK_A) as usize))
                            .upvalue_v
                            .upvaluea_p;
                            let rb: *mut TValue = k.add((i >> POSITION_B & MASK_A) as usize);
                            let rc: *mut TValue = if (i & 1_u32 << POSITION_K) as i32 != 0 {
                                k.add((i >> POSITION_C & MASK_A) as usize)
                            } else {
                                &mut (*base.add((i >> POSITION_C & MASK_A) as usize))
                            };
                            let key: *mut TString = (*rb).as_string().unwrap();
                            if !((*upval).get_tagvariant() == TagVariant::Table) {
                                hres = HNOTATABLE;
                            } else {
                                hres = finishnodeset(
                                    (*upval).as_table().unwrap(),
                                    luah_getshortstr(&mut (*(*upval).as_table().unwrap()), key),
                                    rc,
                                );
                            }
                            if hres == HOK {
                                if (*rc).is_collectable()
                                    && (*(*upval).as_object().unwrap()).get_marked() & BLACKBIT != 0
                                    && (*(*rc).as_object().unwrap()).get_marked() & WHITEBITS != 0
                                {
                                    ObjectWithGCList::luac_barrierback_(
                                        state,
                                        (*upval).as_object().unwrap() as *mut ObjectWithGCList,
                                    );
                                }
                            } else {
                                (*callinfo).callinfo_u.l.saved_program_counter = program_counter;
                                (*state).interpreter_top.stkidrel_pointer = (*callinfo).callinfo_top.stkidrel_pointer;
                                luav_finishset(state, upval, rb, rc, hres);
                                trap = (*callinfo).callinfo_u.l.trap;
                            }
                            continue;
                        }
                        OPCODE_SETTABLE => {
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            let hres: i32;
                            let rb: *mut TValue = &mut (*base.add((i >> POSITION_B & MASK_A) as usize));
                            let rc: *mut TValue = if (i & 1_u32 << POSITION_K) as i32 != 0 {
                                k.add((i >> POSITION_C & MASK_A) as usize)
                            } else {
                                &mut (*base.add((i >> POSITION_C & MASK_A) as usize))
                            };
                            if (*rb).get_tagvariant() == TagVariant::NumericInteger {
                                if !((*ra).get_tagvariant() == TagVariant::Table) {
                                    hres = HNOTATABLE;
                                } else {
                                    let h = (*ra).as_table().unwrap();
                                    let u = ((*rb).as_integer().unwrap() as u64).wrapping_sub(1);
                                    if u < (*h).table_a_size as u64 {
                                        let u = u as u32;
                                        let tag = get_arr_tag(h, u);
                                        if ((*h).get_metatable().is_null()
                                            || (*(*h).get_metatable()).table_flags as u32 & (1u32 << TM_NEWINDEX as i32) != 0)
                                            || !tagisempty(*tag)
                                        {
                                            *tag = table_store_tag((*rc).get_tagvariant() as u8);
                                            *get_arr_val(h, u) = (*rc).get_raw_value();
                                            hres = HOK;
                                        } else {
                                            hres = !(u as i32);
                                        }
                                    } else {
                                        hres = luah_psetint(h, (*rb).as_integer().unwrap(), rc);
                                    }
                                }
                            } else if !((*ra).get_tagvariant() == TagVariant::Table) {
                                hres = HNOTATABLE;
                            } else {
                                hres = luah_pset((*ra).as_table().unwrap(), rb, rc);
                            }
                            if hres == HOK {
                                if (*rc).is_collectable()
                                    && (*(*ra).as_object().unwrap()).get_marked() & BLACKBIT != 0
                                    && (*(*rc).as_object().unwrap()).get_marked() & WHITEBITS != 0
                                {
                                    ObjectWithGCList::luac_barrierback_(state, (*ra).as_object().unwrap() as *mut ObjectWithGCList);
                                }
                            } else {
                                (*callinfo).callinfo_u.l.saved_program_counter = program_counter;
                                (*state).interpreter_top.stkidrel_pointer = (*callinfo).callinfo_top.stkidrel_pointer;
                                luav_finishset(state, &(*ra), rb, rc, hres);
                                trap = (*callinfo).callinfo_u.l.trap;
                            }
                            continue;
                        }
                        OPCODE_SETI => {
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            let hres: i32;
                            let b: i32 = (i >> POSITION_B & MASK_A) as i32;
                            let rc: *mut TValue = if (i & 1_u32 << POSITION_K) as i32 != 0 {
                                k.add((i >> POSITION_C & MASK_A) as usize)
                            } else {
                                &mut (*base.add((i >> POSITION_C & MASK_A) as usize))
                            };
                            if !((*ra).get_tagvariant() == TagVariant::Table) {
                                hres = HNOTATABLE;
                            } else {
                                let h = (*ra).as_table().unwrap();
                                let u = (b as u64).wrapping_sub(1);
                                if u < (*h).table_a_size as u64 {
                                    let u = u as u32;
                                    let tag = get_arr_tag(h, u);
                                    if ((*h).get_metatable().is_null()
                                        || (*(*h).get_metatable()).table_flags as u32 & (1u32 << TM_NEWINDEX as i32) != 0)
                                        || !tagisempty(*tag)
                                    {
                                        *tag = table_store_tag((*rc).get_tagvariant() as u8);
                                        *get_arr_val(h, u) = (*rc).get_raw_value();
                                        hres = HOK;
                                    } else {
                                        hres = !(u as i32);
                                    }
                                } else {
                                    hres = luah_psetint(h, b as i64, rc);
                                }
                            }
                            if hres == HOK {
                                if (*rc).is_collectable()
                                    && (*(*ra).as_object().unwrap()).get_marked() & BLACKBIT != 0
                                    && (*(*rc).as_object().unwrap()).get_marked() & WHITEBITS != 0
                                {
                                    ObjectWithGCList::luac_barrierback_(state, (*ra).as_object().unwrap() as *mut ObjectWithGCList);
                                }
                            } else {
                                let mut key: TValue = TValue::new(TagVariant::NilNil);
                                let io: *mut TValue = &mut key;
                                (*io).set_integer(b as i64);
                                (*callinfo).callinfo_u.l.saved_program_counter = program_counter;
                                (*state).interpreter_top.stkidrel_pointer = (*callinfo).callinfo_top.stkidrel_pointer;
                                luav_finishset(state, &(*ra), &mut key, rc, hres);
                                trap = (*callinfo).callinfo_u.l.trap;
                            }
                            continue;
                        }
                        OPCODE_SETFIELD => {
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            let hres: i32;
                            let rb: *mut TValue = k.add((i >> POSITION_B & MASK_A) as usize);
                            let rc: *mut TValue = if (i & 1_u32 << POSITION_K) as i32 != 0 {
                                k.add((i >> POSITION_C & MASK_A) as usize)
                            } else {
                                &mut (*base.add((i >> POSITION_C & MASK_A) as usize))
                            };
                            let key: *mut TString = (*rb).as_string().unwrap();
                            if !((*ra).get_tagvariant() == TagVariant::Table) {
                                hres = HNOTATABLE;
                            } else {
                                hres = finishnodeset(
                                    (*ra).as_table().unwrap(),
                                    luah_getshortstr(&mut (*(*ra).as_table().unwrap()), key),
                                    rc,
                                );
                            }
                            if hres == HOK {
                                if (*rc).is_collectable()
                                    && (*(*ra).as_object().unwrap()).get_marked() & BLACKBIT != 0
                                    && (*(*rc).as_object().unwrap()).get_marked() & WHITEBITS != 0
                                {
                                    ObjectWithGCList::luac_barrierback_(state, (*ra).as_object().unwrap() as *mut ObjectWithGCList);
                                }
                            } else {
                                (*callinfo).callinfo_u.l.saved_program_counter = program_counter;
                                (*state).interpreter_top.stkidrel_pointer = (*callinfo).callinfo_top.stkidrel_pointer;
                                luav_finishset(state, &(*ra), rb, rc, hres);
                                trap = (*callinfo).callinfo_u.l.trap;
                            }
                            continue;
                        }
                        OPCODE_NEWTABLE => {
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            let mut new_table_size = (i >> POSITION_B & MASK_A) as usize;
                            let mut new_array_size: usize = (i >> POSITION_C & MASK_A) as usize;

                            if new_table_size > 0 {
                                new_table_size = 1 << (new_table_size - 1);
                            }
                            if (i & 1_u32 << POSITION_K) as i32 != 0 {
                                new_array_size += ((*program_counter >> POSITION_A & MASK_AX) as i32 * ((1 << 8) - 1 + 1)) as usize;
                            }
                            program_counter = program_counter.add(1);
                            (*state).interpreter_top.stkidrel_pointer = ra.add(1);
                            let table: *mut Table = luah_new(state);
                            let io: *mut TValue = &mut (*ra);
                            (*io).set_table(table);
                            if new_table_size != 0 || new_array_size != 0 {
                                luah_resize(state, table, new_array_size, new_table_size);
                            }
                            if (*state).should_step() {
                                (*callinfo).callinfo_u.l.saved_program_counter = program_counter;
                                (*state).interpreter_top.stkidrel_pointer = ra.add(1);
                                (*state).do_gc_step();
                                trap = (*callinfo).callinfo_u.l.trap;
                            }
                            continue;
                        }
                        OPCODE_SELF => {
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            let rb: *mut TValue = &mut (*base.add((i >> POSITION_B & MASK_A) as usize));
                            let rc: *mut TValue = k.add((i >> POSITION_C & MASK_A) as usize);
                            let key: *mut TString = (*rc).as_string().unwrap();
                            let io1: *mut TValue = &mut (*ra.add(1));
                            let io2: *const TValue = rb;
                            (*io1).copy_from(&(*io2));
                            let tag: TagVariant;
                            if !((*rb).get_tagvariant() == TagVariant::Table) {
                                tag = TagVariant::NilNil;
                            } else {
                                let slot = luah_getstr(&mut (*(*rb).as_table().unwrap()), key);
                                if !(*slot).get_tagvariant().to_tag_type().is_nil() {
                                    (*ra).copy_from(&*slot);
                                    tag = (*slot).get_tagvariant();
                                } else {
                                    tag = (*slot).get_tagvariant();
                                }
                            }
                            if tag.to_tag_type().is_nil() {
                                (*callinfo).callinfo_u.l.saved_program_counter = program_counter;
                                (*state).interpreter_top.stkidrel_pointer = (*callinfo).callinfo_top.stkidrel_pointer;
                                luav_finishget(state, rb, rc, ra, tag);
                                trap = (*callinfo).callinfo_u.l.trap;
                            }
                            continue;
                        }
                        OPCODE_ADDI => {
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            let v1: *mut TValue = &mut (*base.add((i >> POSITION_B & MASK_A) as usize));
                            let imm: i32 = (i >> POSITION_C & MASK_A) as i32 - (((1 << 8) - 1) >> 1);
                            if (*v1).get_tagvariant() == TagVariant::NumericInteger {
                                let iv1: i64 = (*v1).as_integer().unwrap();
                                program_counter = program_counter.add(1);
                                let io: *mut TValue = &mut (*ra);
                                (*io).set_integer((iv1 as usize).wrapping_add(imm as usize) as i64);
                            } else if (*v1).get_tagvariant() == TagVariant::NumericNumber {
                                let nb: f64 = (*v1).as_number().unwrap();
                                let fimm: f64 = imm as f64;
                                program_counter = program_counter.add(1);
                                let io: *mut TValue = &mut (*ra);
                                (*io).set_number(nb + fimm);
                            }
                            continue;
                        }
                        OPCODE_ADDK => {
                            let v1: *mut TValue = &mut (*base.add((i >> POSITION_B & MASK_A) as usize));
                            let v2: *mut TValue = k.add((i >> POSITION_C & MASK_A) as usize);
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            if (*v1).get_tagvariant() == TagVariant::NumericInteger
                                && (*v2).get_tagvariant() == TagVariant::NumericInteger
                            {
                                let i1: i64 = (*v1).as_integer().unwrap();
                                let i2: i64 = (*v2).as_integer().unwrap();
                                program_counter = program_counter.add(1);
                                let io: *mut TValue = &mut (*ra);
                                (*io).set_integer((i1 as usize).wrapping_add(i2 as usize) as i64);
                            } else {
                                let mut n1: f64 = 0.0;
                                let mut n2: f64 = 0.0;
                                if (if (*v1).get_tagvariant() == TagVariant::NumericNumber {
                                    n1 = (*v1).as_number().unwrap();
                                    1
                                } else {
                                    if (*v1).get_tagvariant() == TagVariant::NumericInteger {
                                        n1 = (*v1).as_integer().unwrap() as f64;
                                        1
                                    } else {
                                        0
                                    }
                                }) != 0
                                    && (if (*v2).get_tagvariant() == TagVariant::NumericNumber {
                                        n2 = (*v2).as_number().unwrap();
                                        1
                                    } else {
                                        if (*v2).get_tagvariant() == TagVariant::NumericInteger {
                                            n2 = (*v2).as_integer().unwrap() as f64;
                                            1
                                        } else {
                                            0
                                        }
                                    }) != 0
                                {
                                    program_counter = program_counter.add(1);
                                    let io: *mut TValue = &mut (*ra);
                                    (*io).set_number(n1 + n2);
                                }
                            }
                            continue;
                        }
                        OPCODE_SUBK => {
                            let v1: *mut TValue = &mut (*base.add((i >> POSITION_B & MASK_A) as usize));
                            let v2: *mut TValue = k.add((i >> POSITION_C & MASK_A) as usize);
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            if (*v1).get_tagvariant() == TagVariant::NumericInteger
                                && (*v2).get_tagvariant() == TagVariant::NumericInteger
                            {
                                let i1: i64 = (*v1).as_integer().unwrap();
                                let i2: i64 = (*v2).as_integer().unwrap();
                                program_counter = program_counter.add(1);
                                let io: *mut TValue = &mut (*ra);
                                (*io).set_integer((i1 as usize).wrapping_sub(i2 as usize) as i64);
                            } else {
                                let mut n1: f64 = 0.0;
                                let mut n2: f64 = 0.0;
                                if (if (*v1).get_tagvariant() == TagVariant::NumericNumber {
                                    n1 = (*v1).as_number().unwrap();
                                    1
                                } else {
                                    if (*v1).get_tagvariant() == TagVariant::NumericInteger {
                                        n1 = (*v1).as_integer().unwrap() as f64;
                                        1
                                    } else {
                                        0
                                    }
                                }) != 0
                                    && (if (*v2).get_tagvariant() == TagVariant::NumericNumber {
                                        n2 = (*v2).as_number().unwrap();
                                        1
                                    } else {
                                        if (*v2).get_tagvariant() == TagVariant::NumericInteger {
                                            n2 = (*v2).as_integer().unwrap() as f64;
                                            1
                                        } else {
                                            0
                                        }
                                    }) != 0
                                {
                                    program_counter = program_counter.add(1);
                                    let io: *mut TValue = &mut (*ra);
                                    (*io).set_number(n1 - n2);
                                }
                            }
                            continue;
                        }
                        OPCODE_MULK => {
                            let v1: *mut TValue = &mut (*base.add((i >> POSITION_B & MASK_A) as usize));
                            let v2: *mut TValue = k.add((i >> POSITION_C & MASK_A) as usize);
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            if (*v1).get_tagvariant() == TagVariant::NumericInteger
                                && (*v2).get_tagvariant() == TagVariant::NumericInteger
                            {
                                let i1: i64 = (*v1).as_integer().unwrap();
                                let i2: i64 = (*v2).as_integer().unwrap();
                                program_counter = program_counter.add(1);
                                let io: *mut TValue = &mut (*ra);
                                (*io).set_integer((i1 as usize).wrapping_mul(i2 as usize) as i64);
                            } else {
                                let mut n1: f64 = 0.0;
                                let mut n2: f64 = 0.0;
                                if (if (*v1).get_tagvariant() == TagVariant::NumericNumber {
                                    n1 = (*v1).as_number().unwrap();
                                    1
                                } else {
                                    if (*v1).get_tagvariant() == TagVariant::NumericInteger {
                                        n1 = (*v1).as_integer().unwrap() as f64;
                                        1
                                    } else {
                                        0
                                    }
                                }) != 0
                                    && (if (*v2).get_tagvariant() == TagVariant::NumericNumber {
                                        n2 = (*v2).as_number().unwrap();
                                        1
                                    } else {
                                        if (*v2).get_tagvariant() == TagVariant::NumericInteger {
                                            n2 = (*v2).as_integer().unwrap() as f64;
                                            1
                                        } else {
                                            0
                                        }
                                    }) != 0
                                {
                                    program_counter = program_counter.add(1);
                                    let io: *mut TValue = &mut (*ra);
                                    (*io).set_number(n1 * n2);
                                }
                            }
                            continue;
                        }
                        OPCODE_MODK => {
                            (*callinfo).callinfo_u.l.saved_program_counter = program_counter;
                            (*state).interpreter_top.stkidrel_pointer = (*callinfo).callinfo_top.stkidrel_pointer;
                            let v1: *mut TValue = &mut (*base.add((i >> POSITION_B & MASK_A) as usize));
                            let v2: *mut TValue = k.add((i >> POSITION_C & MASK_A) as usize);
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            if (*v1).get_tagvariant() == TagVariant::NumericInteger
                                && (*v2).get_tagvariant() == TagVariant::NumericInteger
                            {
                                let i1: i64 = (*v1).as_integer().unwrap();
                                let i2: i64 = (*v2).as_integer().unwrap();
                                program_counter = program_counter.add(1);
                                let io: *mut TValue = &mut (*ra);
                                (*io).set_integer(luav_mod(state, i1, i2));
                            } else {
                                let mut n1: f64 = 0.0;
                                let mut n2: f64 = 0.0;
                                if (if (*v1).get_tagvariant() == TagVariant::NumericNumber {
                                    n1 = (*v1).as_number().unwrap();
                                    1
                                } else {
                                    if (*v1).get_tagvariant() == TagVariant::NumericInteger {
                                        n1 = (*v1).as_integer().unwrap() as f64;
                                        1
                                    } else {
                                        0
                                    }
                                }) != 0
                                    && (if (*v2).get_tagvariant() == TagVariant::NumericNumber {
                                        n2 = (*v2).as_number().unwrap();
                                        1
                                    } else {
                                        if (*v2).get_tagvariant() == TagVariant::NumericInteger {
                                            n2 = (*v2).as_integer().unwrap() as f64;
                                            1
                                        } else {
                                            0
                                        }
                                    }) != 0
                                {
                                    program_counter = program_counter.add(1);
                                    let io: *mut TValue = &mut (*ra);
                                    (*io).set_number(luav_modf(state, n1, n2));
                                }
                            }
                            continue;
                        }
                        OPCODE_POWK => {
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            let v1: *mut TValue = &mut (*base.add((i >> POSITION_B & MASK_A) as usize));
                            let v2: *mut TValue = k.add((i >> POSITION_C & MASK_A) as usize);
                            let mut n1: f64 = 0.0;
                            let mut n2: f64 = 0.0;
                            if (if (*v1).get_tagvariant() == TagVariant::NumericNumber {
                                n1 = (*v1).as_number().unwrap();
                                1
                            } else {
                                if (*v1).get_tagvariant() == TagVariant::NumericInteger {
                                    n1 = (*v1).as_integer().unwrap() as f64;
                                    1
                                } else {
                                    0
                                }
                            }) != 0
                                && (if (*v2).get_tagvariant() == TagVariant::NumericNumber {
                                    n2 = (*v2).as_number().unwrap();
                                    1
                                } else {
                                    if (*v2).get_tagvariant() == TagVariant::NumericInteger {
                                        n2 = (*v2).as_integer().unwrap() as f64;
                                        1
                                    } else {
                                        0
                                    }
                                }) != 0
                            {
                                program_counter = program_counter.add(1);
                                let io: *mut TValue = &mut (*ra);
                                (*io).set_number(if n2 == 2.0 { n1 * n1 } else { n1.powf(n2) });
                            }
                            continue;
                        }
                        OPCODE_DIVK => {
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            let v1: *mut TValue = &mut (*base.add((i >> POSITION_B & MASK_A) as usize));
                            let v2: *mut TValue = k.add((i >> POSITION_C & MASK_A) as usize);
                            let mut n1: f64 = 0.0;
                            let mut n2: f64 = 0.0;
                            if (if (*v1).get_tagvariant() == TagVariant::NumericNumber {
                                n1 = (*v1).as_number().unwrap();
                                1
                            } else {
                                if (*v1).get_tagvariant() == TagVariant::NumericInteger {
                                    n1 = (*v1).as_integer().unwrap() as f64;
                                    1
                                } else {
                                    0
                                }
                            }) != 0
                                && (if (*v2).get_tagvariant() == TagVariant::NumericNumber {
                                    n2 = (*v2).as_number().unwrap();
                                    1
                                } else {
                                    if (*v2).get_tagvariant() == TagVariant::NumericInteger {
                                        n2 = (*v2).as_integer().unwrap() as f64;
                                        1
                                    } else {
                                        0
                                    }
                                }) != 0
                            {
                                program_counter = program_counter.add(1);
                                let io: *mut TValue = &mut (*ra);
                                (*io).set_number(n1 / n2);
                            }
                            continue;
                        }
                        OPCODE_IDIVK => {
                            (*callinfo).callinfo_u.l.saved_program_counter = program_counter;
                            (*state).interpreter_top.stkidrel_pointer = (*callinfo).callinfo_top.stkidrel_pointer;
                            let v1: *mut TValue = &mut (*base.add((i >> POSITION_B & MASK_A) as usize));
                            let v2: *mut TValue = k.add((i >> POSITION_C & MASK_A) as usize);
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            if (*v1).get_tagvariant() == TagVariant::NumericInteger
                                && (*v2).get_tagvariant() == TagVariant::NumericInteger
                            {
                                let i1: i64 = (*v1).as_integer().unwrap();
                                let i2: i64 = (*v2).as_integer().unwrap();
                                program_counter = program_counter.add(1);
                                let io: *mut TValue = &mut (*ra);
                                (*io).set_integer(luav_idiv(state, i1, i2));
                            } else {
                                let mut n1: f64 = 0.0;
                                let mut n2: f64 = 0.0;
                                if (if (*v1).get_tagvariant() == TagVariant::NumericNumber {
                                    n1 = (*v1).as_number().unwrap();
                                    1
                                } else {
                                    if (*v1).get_tagvariant() == TagVariant::NumericInteger {
                                        n1 = (*v1).as_integer().unwrap() as f64;
                                        1
                                    } else {
                                        0
                                    }
                                }) != 0
                                    && (if (*v2).get_tagvariant() == TagVariant::NumericNumber {
                                        n2 = (*v2).as_number().unwrap();
                                        1
                                    } else {
                                        if (*v2).get_tagvariant() == TagVariant::NumericInteger {
                                            n2 = (*v2).as_integer().unwrap() as f64;
                                            1
                                        } else {
                                            0
                                        }
                                    }) != 0
                                {
                                    program_counter = program_counter.add(1);
                                    let io: *mut TValue = &mut (*ra);
                                    (*io).set_number((n1 / n2).floor());
                                }
                            }
                            continue;
                        }
                        OPCODE_BANDK => {
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            let v1: *mut TValue = &mut (*base.add((i >> POSITION_B & MASK_A) as usize));
                            let v2: *mut TValue = k.add((i >> POSITION_C & MASK_A) as usize);
                            let mut i1: i64 = 0;
                            let i2: i64 = (*v2).as_integer().unwrap();
                            if if (*v1).get_tagvariant() == TagVariant::NumericInteger {
                                i1 = (*v1).as_integer().unwrap();
                                1
                            } else {
                                F2I::Equal.convert_tv_i64(v1, &mut i1)
                            } != 0
                            {
                                program_counter = program_counter.add(1);
                                let io: *mut TValue = &mut (*ra);
                                (*io).set_integer((i1 as usize & i2 as usize) as i64);
                            }
                            continue;
                        }
                        OPCODE_BORK => {
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            let v1: *mut TValue = &mut (*base.add((i >> POSITION_B & MASK_A) as usize));
                            let v2: *mut TValue = k.add((i >> POSITION_C & MASK_A) as usize);
                            let mut i1: i64 = 0;
                            let i2: i64 = (*v2).as_integer().unwrap();
                            if if (*v1).get_tagvariant() == TagVariant::NumericInteger {
                                i1 = (*v1).as_integer().unwrap();
                                1
                            } else {
                                F2I::Equal.convert_tv_i64(v1, &mut i1)
                            } != 0
                            {
                                program_counter = program_counter.add(1);
                                let io: *mut TValue = &mut (*ra);
                                (*io).set_integer((i1 as usize | i2 as usize) as i64);
                            }
                            continue;
                        }
                        OPCODE_BXORK => {
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            let v1: *mut TValue = &mut (*base.add((i >> POSITION_B & MASK_A) as usize));
                            let v2: *mut TValue = k.add((i >> POSITION_C & MASK_A) as usize);
                            let mut i1: i64 = 0;
                            let i2: i64 = (*v2).as_integer().unwrap();
                            if if (*v1).get_tagvariant() == TagVariant::NumericInteger {
                                i1 = (*v1).as_integer().unwrap();
                                1
                            } else {
                                F2I::Equal.convert_tv_i64(v1, &mut i1)
                            } != 0
                            {
                                program_counter = program_counter.add(1);
                                let io: *mut TValue = &mut (*ra);
                                (*io).set_integer((i1 as usize ^ i2 as usize) as i64);
                            }
                            continue;
                        }
                        OPCODE_SHRI => {
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            let rb: *mut TValue = &mut (*base.add((i >> POSITION_B & MASK_A) as usize));
                            let ic: i32 = (i >> POSITION_C & MASK_A) as i32 - (((1 << 8) - 1) >> 1);
                            let mut ib: i64 = 0;
                            if if (*rb).get_tagvariant() == TagVariant::NumericInteger {
                                ib = (*rb).as_integer().unwrap();
                                1
                            } else {
                                F2I::Equal.convert_tv_i64(rb, &mut ib)
                            } != 0
                            {
                                program_counter = program_counter.add(1);
                                let io: *mut TValue = &mut (*ra);
                                (*io).set_integer(luav_shiftl(ib, -ic as i64));
                            }
                            continue;
                        }
                        OPCODE_SHLI => {
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            let rb: *mut TValue = &mut (*base.add((i >> POSITION_B & MASK_A) as usize));
                            let ic: i32 = (i >> POSITION_C & MASK_A) as i32 - (((1 << 8) - 1) >> 1);
                            let mut ib: i64 = 0;
                            if if (*rb).get_tagvariant() == TagVariant::NumericInteger {
                                ib = (*rb).as_integer().unwrap();
                                1
                            } else {
                                F2I::Equal.convert_tv_i64(rb, &mut ib)
                            } != 0
                            {
                                program_counter = program_counter.add(1);
                                let io: *mut TValue = &mut (*ra);
                                (*io).set_integer(luav_shiftl(ic as i64, ib));
                            }
                            continue;
                        }
                        OPCODE_ADD => {
                            let v1: *mut TValue = &mut (*base.add((i >> POSITION_B & MASK_A) as usize));
                            let v2: *mut TValue = &mut (*base.add((i >> POSITION_C & MASK_A) as usize));
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            if (*v1).get_tagvariant() == TagVariant::NumericInteger
                                && (*v2).get_tagvariant() == TagVariant::NumericInteger
                            {
                                let i1: i64 = (*v1).as_integer().unwrap();
                                let i2: i64 = (*v2).as_integer().unwrap();
                                program_counter = program_counter.add(1);
                                let io: *mut TValue = &mut (*ra);
                                (*io).set_integer((i1 as usize).wrapping_add(i2 as usize) as i64);
                            } else {
                                let mut n1: f64 = 0.0;
                                let mut n2: f64 = 0.0;
                                if (if (*v1).get_tagvariant() == TagVariant::NumericNumber {
                                    n1 = (*v1).as_number().unwrap();
                                    1
                                } else {
                                    if (*v1).get_tagvariant() == TagVariant::NumericInteger {
                                        n1 = (*v1).as_integer().unwrap() as f64;
                                        1
                                    } else {
                                        0
                                    }
                                }) != 0
                                    && (if (*v2).get_tagvariant() == TagVariant::NumericNumber {
                                        n2 = (*v2).as_number().unwrap();
                                        1
                                    } else {
                                        if (*v2).get_tagvariant() == TagVariant::NumericInteger {
                                            n2 = (*v2).as_integer().unwrap() as f64;
                                            1
                                        } else {
                                            0
                                        }
                                    }) != 0
                                {
                                    program_counter = program_counter.add(1);
                                    let io: *mut TValue = &mut (*ra);
                                    (*io).set_number(n1 + n2);
                                }
                            }
                            continue;
                        }
                        OPCODE_SUB => {
                            let v1: *mut TValue = &mut (*base.add((i >> POSITION_B & MASK_A) as usize));
                            let v2: *mut TValue = &mut (*base.add((i >> POSITION_C & MASK_A) as usize));
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            if (*v1).get_tagvariant() == TagVariant::NumericInteger
                                && (*v2).get_tagvariant() == TagVariant::NumericInteger
                            {
                                let i1: i64 = (*v1).as_integer().unwrap();
                                let i2: i64 = (*v2).as_integer().unwrap();
                                program_counter = program_counter.add(1);
                                let io: *mut TValue = &mut (*ra);
                                (*io).set_integer((i1 as usize).wrapping_sub(i2 as usize) as i64);
                            } else {
                                let mut n1: f64 = 0.0;
                                let mut n2: f64 = 0.0;
                                if (if (*v1).get_tagvariant() == TagVariant::NumericNumber {
                                    n1 = (*v1).as_number().unwrap();
                                    1
                                } else {
                                    if (*v1).get_tagvariant() == TagVariant::NumericInteger {
                                        n1 = (*v1).as_integer().unwrap() as f64;
                                        1
                                    } else {
                                        0
                                    }
                                }) != 0
                                    && (if (*v2).get_tagvariant() == TagVariant::NumericNumber {
                                        n2 = (*v2).as_number().unwrap();
                                        1
                                    } else {
                                        if (*v2).get_tagvariant() == TagVariant::NumericInteger {
                                            n2 = (*v2).as_integer().unwrap() as f64;
                                            1
                                        } else {
                                            0
                                        }
                                    }) != 0
                                {
                                    program_counter = program_counter.add(1);
                                    let io: *mut TValue = &mut (*ra);
                                    (*io).set_number(n1 - n2);
                                }
                            }
                            continue;
                        }
                        OPCODE_MUL => {
                            let v1: *mut TValue = &mut (*base.add((i >> POSITION_B & MASK_A) as usize));
                            let v2: *mut TValue = &mut (*base.add((i >> POSITION_C & MASK_A) as usize));
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            if (*v1).get_tagvariant() == TagVariant::NumericInteger
                                && (*v2).get_tagvariant() == TagVariant::NumericInteger
                            {
                                let i1: i64 = (*v1).as_integer().unwrap();
                                let i2: i64 = (*v2).as_integer().unwrap();
                                program_counter = program_counter.add(1);
                                let io: *mut TValue = &mut (*ra);
                                (*io).set_integer((i1 as usize).wrapping_mul(i2 as usize) as i64);
                            } else {
                                let mut n1: f64 = 0.0;
                                let mut n2: f64 = 0.0;
                                if (if (*v1).get_tagvariant() == TagVariant::NumericNumber {
                                    n1 = (*v1).as_number().unwrap();
                                    1
                                } else {
                                    if (*v1).get_tagvariant() == TagVariant::NumericInteger {
                                        n1 = (*v1).as_integer().unwrap() as f64;
                                        1
                                    } else {
                                        0
                                    }
                                }) != 0
                                    && (if (*v2).get_tagvariant() == TagVariant::NumericNumber {
                                        n2 = (*v2).as_number().unwrap();
                                        1
                                    } else {
                                        if (*v2).get_tagvariant() == TagVariant::NumericInteger {
                                            n2 = (*v2).as_integer().unwrap() as f64;
                                            1
                                        } else {
                                            0
                                        }
                                    }) != 0
                                {
                                    program_counter = program_counter.add(1);
                                    let io: *mut TValue = &mut (*ra);
                                    (*io).set_number(n1 * n2);
                                }
                            }
                            continue;
                        }
                        OPCODE_MOD => {
                            (*callinfo).callinfo_u.l.saved_program_counter = program_counter;
                            (*state).interpreter_top.stkidrel_pointer = (*callinfo).callinfo_top.stkidrel_pointer;
                            let v1: *mut TValue = &mut (*base.add((i >> POSITION_B & MASK_A) as usize));
                            let v2: *mut TValue = &mut (*base.add((i >> POSITION_C & MASK_A) as usize));
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            if (*v1).get_tagvariant() == TagVariant::NumericInteger
                                && (*v2).get_tagvariant() == TagVariant::NumericInteger
                            {
                                let i1: i64 = (*v1).as_integer().unwrap();
                                let i2: i64 = (*v2).as_integer().unwrap();
                                program_counter = program_counter.add(1);
                                let io: *mut TValue = &mut (*ra);
                                (*io).set_integer(luav_mod(state, i1, i2));
                            } else {
                                let mut n1: f64 = 0.0;
                                let mut n2: f64 = 0.0;
                                if (if (*v1).get_tagvariant() == TagVariant::NumericNumber {
                                    n1 = (*v1).as_number().unwrap();
                                    1
                                } else {
                                    if (*v1).get_tagvariant() == TagVariant::NumericInteger {
                                        n1 = (*v1).as_integer().unwrap() as f64;
                                        1
                                    } else {
                                        0
                                    }
                                }) != 0
                                    && (if (*v2).get_tagvariant() == TagVariant::NumericNumber {
                                        n2 = (*v2).as_number().unwrap();
                                        1
                                    } else {
                                        if (*v2).get_tagvariant() == TagVariant::NumericInteger {
                                            n2 = (*v2).as_integer().unwrap() as f64;
                                            1
                                        } else {
                                            0
                                        }
                                    }) != 0
                                {
                                    program_counter = program_counter.add(1);
                                    let io: *mut TValue = &mut (*ra);
                                    (*io).set_number(luav_modf(state, n1, n2));
                                }
                            }
                            continue;
                        }
                        OPCODE_POW => {
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            let v1: *mut TValue = &mut (*base.add((i >> POSITION_B & MASK_A) as usize));
                            let v2: *mut TValue = &mut (*base.add((i >> POSITION_C & MASK_A) as usize));
                            let mut n1: f64 = 0.0;
                            let mut n2: f64 = 0.0;
                            if (if (*v1).get_tagvariant() == TagVariant::NumericNumber {
                                n1 = (*v1).as_number().unwrap();
                                1
                            } else {
                                if (*v1).get_tagvariant() == TagVariant::NumericInteger {
                                    n1 = (*v1).as_integer().unwrap() as f64;
                                    1
                                } else {
                                    0
                                }
                            }) != 0
                                && (if (*v2).get_tagvariant() == TagVariant::NumericNumber {
                                    n2 = (*v2).as_number().unwrap();
                                    1
                                } else {
                                    if (*v2).get_tagvariant() == TagVariant::NumericInteger {
                                        n2 = (*v2).as_integer().unwrap() as f64;
                                        1
                                    } else {
                                        0
                                    }
                                }) != 0
                            {
                                program_counter = program_counter.add(1);
                                let io: *mut TValue = &mut (*ra);
                                (*io).set_number(if n2 == 2.0 { n1 * n1 } else { n1.powf(n2) });
                            }
                            continue;
                        }
                        OPCODE_DIV => {
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            let v1: *mut TValue = &mut (*base.add((i >> POSITION_B & MASK_A) as usize));
                            let v2: *mut TValue = &mut (*base.add((i >> POSITION_C & MASK_A) as usize));
                            let mut n1: f64 = 0.0;
                            let mut n2: f64 = 0.0;
                            if (if (*v1).get_tagvariant() == TagVariant::NumericNumber {
                                n1 = (*v1).as_number().unwrap();
                                1
                            } else {
                                if (*v1).get_tagvariant() == TagVariant::NumericInteger {
                                    n1 = (*v1).as_integer().unwrap() as f64;
                                    1
                                } else {
                                    0
                                }
                            }) != 0
                                && (if (*v2).get_tagvariant() == TagVariant::NumericNumber {
                                    n2 = (*v2).as_number().unwrap();
                                    1
                                } else {
                                    if (*v2).get_tagvariant() == TagVariant::NumericInteger {
                                        n2 = (*v2).as_integer().unwrap() as f64;
                                        1
                                    } else {
                                        0
                                    }
                                }) != 0
                            {
                                program_counter = program_counter.add(1);
                                let io: *mut TValue = &mut (*ra);
                                (*io).set_number(n1 / n2);
                            }
                            continue;
                        }
                        OPCODE_IDIV => {
                            (*callinfo).callinfo_u.l.saved_program_counter = program_counter;
                            (*state).interpreter_top.stkidrel_pointer = (*callinfo).callinfo_top.stkidrel_pointer;
                            let v1: *mut TValue = &mut (*base.add((i >> POSITION_B & MASK_A) as usize));
                            let v2: *mut TValue = &mut (*base.add((i >> POSITION_C & MASK_A) as usize));
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            if (*v1).get_tagvariant() == TagVariant::NumericInteger
                                && (*v2).get_tagvariant() == TagVariant::NumericInteger
                            {
                                let i1: i64 = (*v1).as_integer().unwrap();
                                let i2: i64 = (*v2).as_integer().unwrap();
                                program_counter = program_counter.add(1);
                                let io: *mut TValue = &mut (*ra);
                                (*io).set_integer(luav_idiv(state, i1, i2));
                            } else {
                                let mut n1: f64 = 0.0;
                                let mut n2: f64 = 0.0;
                                if (if (*v1).get_tagvariant() == TagVariant::NumericNumber {
                                    n1 = (*v1).as_number().unwrap();
                                    1
                                } else {
                                    if (*v1).get_tagvariant() == TagVariant::NumericInteger {
                                        n1 = (*v1).as_integer().unwrap() as f64;
                                        1
                                    } else {
                                        0
                                    }
                                }) != 0
                                    && (if (*v2).get_tagvariant() == TagVariant::NumericNumber {
                                        n2 = (*v2).as_number().unwrap();
                                        1
                                    } else {
                                        if (*v2).get_tagvariant() == TagVariant::NumericInteger {
                                            n2 = (*v2).as_integer().unwrap() as f64;
                                            1
                                        } else {
                                            0
                                        }
                                    }) != 0
                                {
                                    program_counter = program_counter.add(1);
                                    let io: *mut TValue = &mut (*ra);
                                    (*io).set_number((n1 / n2).floor());
                                }
                            }
                            continue;
                        }
                        OPCODE_BAND => {
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            let v1: *mut TValue = &mut (*base.add((i >> POSITION_B & MASK_A) as usize));
                            let v2: *mut TValue = &mut (*base.add((i >> POSITION_C & MASK_A) as usize));
                            let mut i1: i64 = 0;
                            let mut i2: i64 = 0;
                            if (if (*v1).get_tagvariant() == TagVariant::NumericInteger {
                                i1 = (*v1).as_integer().unwrap();
                                1
                            } else {
                                F2I::Equal.convert_tv_i64(v1, &mut i1)
                            }) != 0
                                && (if (*v2).get_tagvariant() == TagVariant::NumericInteger {
                                    i2 = (*v2).as_integer().unwrap();
                                    1
                                } else {
                                    F2I::Equal.convert_tv_i64(v2, &mut i2)
                                }) != 0
                            {
                                program_counter = program_counter.add(1);
                                let io: *mut TValue = &mut (*ra);
                                (*io).set_integer((i1 as usize & i2 as usize) as i64);
                            }
                            continue;
                        }
                        OPCODE_BOR => {
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            let v1: *mut TValue = &mut (*base.add((i >> POSITION_B & MASK_A) as usize));
                            let v2: *mut TValue = &mut (*base.add((i >> POSITION_C & MASK_A) as usize));
                            let mut i1: i64 = 0;
                            let mut i2: i64 = 0;
                            if (if (*v1).get_tagvariant() == TagVariant::NumericInteger {
                                i1 = (*v1).as_integer().unwrap();
                                1
                            } else {
                                F2I::Equal.convert_tv_i64(v1, &mut i1)
                            }) != 0
                                && (if (*v2).get_tagvariant() == TagVariant::NumericInteger {
                                    i2 = (*v2).as_integer().unwrap();
                                    1
                                } else {
                                    F2I::Equal.convert_tv_i64(v2, &mut i2)
                                }) != 0
                            {
                                program_counter = program_counter.add(1);
                                let io: *mut TValue = &mut (*ra);
                                (*io).set_integer((i1 as usize | i2 as usize) as i64);
                            }
                            continue;
                        }
                        OPCODE_BXOR => {
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            let v1: *mut TValue = &mut (*base.add((i >> POSITION_B & MASK_A) as usize));
                            let v2: *mut TValue = &mut (*base.add((i >> POSITION_C & MASK_A) as usize));
                            let mut i1: i64 = 0;
                            let mut i2: i64 = 0;
                            if (if (*v1).get_tagvariant() == TagVariant::NumericInteger {
                                i1 = (*v1).as_integer().unwrap();
                                1
                            } else {
                                F2I::Equal.convert_tv_i64(v1, &mut i1)
                            }) != 0
                                && (if (*v2).get_tagvariant() == TagVariant::NumericInteger {
                                    i2 = (*v2).as_integer().unwrap();
                                    1
                                } else {
                                    F2I::Equal.convert_tv_i64(v2, &mut i2)
                                }) != 0
                            {
                                program_counter = program_counter.add(1);
                                let io: *mut TValue = &mut (*ra);
                                (*io).set_integer((i1 as usize ^ i2 as usize) as i64);
                            }
                            continue;
                        }
                        OPCODE_SHR => {
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            let v1: *mut TValue = &mut (*base.add((i >> POSITION_B & MASK_A) as usize));
                            let v2: *mut TValue = &mut (*base.add((i >> POSITION_C & MASK_A) as usize));
                            let mut i1: i64 = 0;
                            let mut i2: i64 = 0;
                            if (if (*v1).get_tagvariant() == TagVariant::NumericInteger {
                                i1 = (*v1).as_integer().unwrap();
                                1
                            } else {
                                F2I::Equal.convert_tv_i64(v1, &mut i1)
                            }) != 0
                                && (if (*v2).get_tagvariant() == TagVariant::NumericInteger {
                                    i2 = (*v2).as_integer().unwrap();
                                    1
                                } else {
                                    F2I::Equal.convert_tv_i64(v2, &mut i2)
                                }) != 0
                            {
                                program_counter = program_counter.add(1);
                                (*ra).set_integer(luav_shiftl(i1, (0usize).wrapping_sub(i2 as usize) as i64));
                            }
                            continue;
                        }
                        OPCODE_SHL => {
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            let v1: *mut TValue = &mut (*base.add((i >> POSITION_B & MASK_A) as usize));
                            let v2: *mut TValue = &mut (*base.add((i >> POSITION_C & MASK_A) as usize));
                            let mut i1: i64 = 0;
                            let mut i2: i64 = 0;
                            if (if (*v1).get_tagvariant() == TagVariant::NumericInteger {
                                i1 = (*v1).as_integer().unwrap();
                                1
                            } else {
                                F2I::Equal.convert_tv_i64(v1, &mut i1)
                            }) != 0
                                && (if (*v2).get_tagvariant() == TagVariant::NumericInteger {
                                    i2 = (*v2).as_integer().unwrap();
                                    1
                                } else {
                                    F2I::Equal.convert_tv_i64(v2, &mut i2)
                                }) != 0
                            {
                                program_counter = program_counter.add(1);
                                let io: *mut TValue = &mut (*ra);
                                (*io).set_integer(luav_shiftl(i1, i2));
                            }
                            continue;
                        }
                        OPCODE_MMBIN => {
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            let pi: u32 = *program_counter.sub(2);
                            let rb: *mut TValue = &mut (*base.add((i >> POSITION_B & MASK_A) as usize));
                            let tm: u32 = i >> POSITION_C & MASK_A;
                            let result: *mut TValue = base.add((pi >> POSITION_A & MASK_A) as usize);
                            (*callinfo).callinfo_u.l.saved_program_counter = program_counter;
                            (*state).interpreter_top.stkidrel_pointer = (*callinfo).callinfo_top.stkidrel_pointer;
                            luat_trybintm(state, &(*ra), rb, result, tm);
                            trap = (*callinfo).callinfo_u.l.trap;
                            continue;
                        }
                        OPCODE_MMBINI => {
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            let pi: u32 = *program_counter.sub(2);
                            let imm: i32 = (i >> POSITION_B & MASK_A) as i32 - (((1 << 8) - 1) >> 1);
                            let tm: u32 = i >> POSITION_C & MASK_A;
                            let flip: i32 = (i >> POSITION_K & MASK_K) as i32;
                            let result: *mut TValue = base.add((pi >> POSITION_A & MASK_A) as usize);
                            (*callinfo).callinfo_u.l.saved_program_counter = program_counter;
                            (*state).interpreter_top.stkidrel_pointer = (*callinfo).callinfo_top.stkidrel_pointer;
                            luat_trybinitm(state, &(*ra), imm as i64, flip, result, tm);
                            trap = (*callinfo).callinfo_u.l.trap;
                            continue;
                        }
                        OPCODE_MMBINK => {
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            let pi: u32 = *program_counter.sub(2);
                            let imm: *mut TValue = k.add((i >> POSITION_B & MASK_A) as usize);
                            let tm: u32 = i >> POSITION_C & MASK_A;
                            let flip: i32 = (i >> POSITION_K & MASK_K) as i32;
                            let result: *mut TValue = base.add((pi >> POSITION_A & MASK_A) as usize);
                            (*callinfo).callinfo_u.l.saved_program_counter = program_counter;
                            (*state).interpreter_top.stkidrel_pointer = (*callinfo).callinfo_top.stkidrel_pointer;
                            luat_trybinassoctm(state, &(*ra), imm, flip, result, tm);
                            trap = (*callinfo).callinfo_u.l.trap;
                            continue;
                        }
                        OPCODE_UNM => {
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            let rb: *mut TValue = &mut (*base.add((i >> POSITION_B & MASK_A) as usize));
                            let mut nb: f64 = 0.0;
                            if (*rb).get_tagvariant() == TagVariant::NumericInteger {
                                let ib: i64 = (*rb).as_integer().unwrap();
                                let io0: *mut TValue = &mut (*ra);
                                (*io0).set_integer((0usize).wrapping_sub(ib as usize) as i64);
                            } else if if (*rb).get_tagvariant() == TagVariant::NumericNumber {
                                nb = (*rb).as_number().unwrap();
                                1
                            } else if (*rb).get_tagvariant() == TagVariant::NumericInteger {
                                nb = (*rb).as_integer().unwrap() as f64;
                                1
                            } else {
                                0
                            } != 0
                            {
                                let io1: *mut TValue = &mut (*ra);
                                (*io1).set_number(-nb);
                            } else {
                                (*callinfo).callinfo_u.l.saved_program_counter = program_counter;
                                (*state).interpreter_top.stkidrel_pointer = (*callinfo).callinfo_top.stkidrel_pointer;
                                luat_trybintm(state, rb, rb, ra, TM_UNM);
                                trap = (*callinfo).callinfo_u.l.trap;
                            }
                            continue;
                        }
                        OPCODE_BNOT => {
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            let rb: *mut TValue = &mut (*base.add((i >> POSITION_B & MASK_A) as usize));
                            let mut ib: i64 = 0;
                            if if (*rb).get_tagvariant() == TagVariant::NumericInteger {
                                ib = (*rb).as_integer().unwrap();
                                1
                            } else {
                                F2I::Equal.convert_tv_i64(rb, &mut ib)
                            } != 0
                            {
                                let io2: *mut TValue = &mut (*ra);
                                (*io2).set_integer((!(0usize) ^ ib as usize) as i64);
                            } else {
                                (*callinfo).callinfo_u.l.saved_program_counter = program_counter;
                                (*state).interpreter_top.stkidrel_pointer = (*callinfo).callinfo_top.stkidrel_pointer;
                                luat_trybintm(state, rb, rb, ra, TM_BNOT);
                                trap = (*callinfo).callinfo_u.l.trap;
                            }
                            continue;
                        }
                        OPCODE_NOT => {
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            let rb: *mut TValue = &mut (*base.add((i >> POSITION_B & MASK_A) as usize));
                            if (*rb).get_tagvariant() == TagVariant::BooleanFalse || (*rb).get_tagvariant().to_tag_type().is_nil() {
                                (*ra).tvalue_set_tag_variant(TagVariant::BooleanTrue);
                            } else {
                                (*ra).tvalue_set_tag_variant(TagVariant::BooleanFalse);
                            }
                            continue;
                        }
                        OPCODE_LEN => {
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            (*callinfo).callinfo_u.l.saved_program_counter = program_counter;
                            (*state).interpreter_top.stkidrel_pointer = (*callinfo).callinfo_top.stkidrel_pointer;
                            luav_objlen(state, ra, &(*base.add((i >> POSITION_B & MASK_A) as usize)));
                            trap = (*callinfo).callinfo_u.l.trap;
                            continue;
                        }
                        OPCODE_CONCAT => {
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            let n: i32 = (i >> POSITION_B & MASK_A) as i32;
                            (*state).interpreter_top.stkidrel_pointer = ra.add(n as usize);
                            (*callinfo).callinfo_u.l.saved_program_counter = program_counter;
                            concatenate(state, n);
                            trap = (*callinfo).callinfo_u.l.trap;
                            if (*state).should_step() {
                                (*callinfo).callinfo_u.l.saved_program_counter = program_counter;
                                (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer;
                                (*state).do_gc_step();
                                trap = (*callinfo).callinfo_u.l.trap;
                            }
                            continue;
                        }
                        OPCODE_CLOSE => {
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            (*callinfo).callinfo_u.l.saved_program_counter = program_counter;
                            (*state).interpreter_top.stkidrel_pointer = (*callinfo).callinfo_top.stkidrel_pointer;
                            luaf_close(state, ra, Status::OK, 1);
                            trap = (*callinfo).callinfo_u.l.trap;
                            continue;
                        }
                        OPCODE_TBC => {
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            (*callinfo).callinfo_u.l.saved_program_counter = program_counter;
                            (*state).interpreter_top.stkidrel_pointer = (*callinfo).callinfo_top.stkidrel_pointer;
                            luaf_newtbcupval(state, ra);
                            continue;
                        }
                        OPCODE_JMP => {
                            program_counter = program_counter.offset(((i >> POSITION_A & MASK_AX) as i32 - (OFFSET_SJ)) as isize);
                            trap = (*callinfo).callinfo_u.l.trap;
                            continue;
                        }
                        OPCODE_EQ => {
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);

                            let rb: *mut TValue = &mut (*base.add((i >> POSITION_B & MASK_A) as usize));
                            (*callinfo).callinfo_u.l.saved_program_counter = program_counter;
                            (*state).interpreter_top.stkidrel_pointer = (*callinfo).callinfo_top.stkidrel_pointer;
                            let cond: i32 = if luav_equalobj(state, &(*ra), rb) {
                                1
                            } else {
                                0
                            };
                            trap = (*callinfo).callinfo_u.l.trap;
                            if cond != (i >> POSITION_K & MASK_K) as i32 {
                                program_counter = program_counter.add(1);
                            } else {
                                let ni: u32 = *program_counter;
                                program_counter =
                                    program_counter.offset(((ni >> POSITION_A & MASK_AX) as i32 - (OFFSET_SJ) + 1) as isize);
                                trap = (*callinfo).callinfo_u.l.trap;
                            }
                            continue;
                        }
                        OPCODE_LT => {
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            let cond: i32;
                            let rb: *mut TValue = &mut (*base.add((i >> POSITION_B & MASK_A) as usize));
                            if (*ra).get_tagvariant() == TagVariant::NumericInteger
                                && (*rb).get_tagvariant() == TagVariant::NumericInteger
                            {
                                let ia: i64 = (*ra).as_integer().unwrap();
                                let ib: i64 = (*rb).as_integer().unwrap();
                                cond = (ia < ib) as i32;
                            } else if (*ra).get_tagvariant().to_tag_type().is_numeric()
                                && (*rb).get_tagvariant().to_tag_type().is_numeric()
                            {
                                cond = if ltnum(&(*ra), rb) { 1 } else { 0 };
                            } else {
                                (*callinfo).callinfo_u.l.saved_program_counter = program_counter;
                                (*state).interpreter_top.stkidrel_pointer = (*callinfo).callinfo_top.stkidrel_pointer;
                                cond = lessthanothers(state, &(*ra), rb);
                                trap = (*callinfo).callinfo_u.l.trap;
                            }
                            if cond != (i >> POSITION_K & MASK_K) as i32 {
                                program_counter = program_counter.add(1);
                            } else {
                                let ni: u32 = *program_counter;
                                program_counter =
                                    program_counter.offset(((ni >> POSITION_A & MASK_AX) as i32 - (OFFSET_SJ) + 1) as isize);
                                trap = (*callinfo).callinfo_u.l.trap;
                            }
                            continue;
                        }
                        OPCODE_LE => {
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            let cond: i32;
                            let rb: *mut TValue = &mut (*base.add((i >> POSITION_B & MASK_A) as usize));
                            if (*ra).get_tagvariant() == TagVariant::NumericInteger
                                && (*rb).get_tagvariant() == TagVariant::NumericInteger
                            {
                                let ia: i64 = (*ra).as_integer().unwrap();
                                let ib_call: i64 = (*rb).as_integer().unwrap();
                                cond = (ia <= ib_call) as i32;
                            } else if (*ra).get_tagvariant().to_tag_type().is_numeric()
                                && (*rb).get_tagvariant().to_tag_type().is_numeric()
                            {
                                cond = if lenum(&(*ra), rb) { 1 } else { 0 };
                            } else {
                                (*callinfo).callinfo_u.l.saved_program_counter = program_counter;
                                (*state).interpreter_top.stkidrel_pointer = (*callinfo).callinfo_top.stkidrel_pointer;
                                cond = if lessequalothers(state, &(*ra), rb) {
                                    1
                                } else {
                                    0
                                };
                                trap = (*callinfo).callinfo_u.l.trap;
                            }
                            if cond != (i >> POSITION_K & MASK_K) as i32 {
                                program_counter = program_counter.add(1);
                            } else {
                                let ni: u32 = *program_counter;
                                program_counter =
                                    program_counter.offset(((ni >> POSITION_A & MASK_AX) as i32 - (OFFSET_SJ) + 1) as isize);
                                trap = (*callinfo).callinfo_u.l.trap;
                            }
                            continue;
                        }
                        OPCODE_EQK => {
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            let rb: *mut TValue = k.add((i >> POSITION_B & MASK_A) as usize);
                            let cond: i32 = if luav_equalobj(null_mut(), &(*ra), rb) {
                                1
                            } else {
                                0
                            };
                            if cond != (i >> POSITION_K & MASK_K) as i32 {
                                program_counter = program_counter.add(1);
                            } else {
                                let ni: u32 = *program_counter;
                                program_counter =
                                    program_counter.offset(((ni >> POSITION_A & MASK_AX) as i32 - (OFFSET_SJ) + 1) as isize);
                                trap = (*callinfo).callinfo_u.l.trap;
                            }
                            continue;
                        }
                        OPCODE_EQI => {
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            let cond: i32;
                            let im: i32 = (i >> POSITION_B & MASK_A) as i32 - (((1 << 8) - 1) >> 1);
                            if (*ra).get_tagvariant() == TagVariant::NumericInteger {
                                cond = ((*ra).as_integer().unwrap() == im as i64) as i32;
                            } else if (*ra).get_tagvariant() == TagVariant::NumericNumber {
                                cond = ((*ra).as_number().unwrap() == im as f64) as i32;
                            } else {
                                cond = 0;
                            }
                            if cond != (i >> POSITION_K & MASK_K) as i32 {
                                program_counter = program_counter.add(1);
                            } else {
                                let ni: u32 = *program_counter;
                                program_counter =
                                    program_counter.offset(((ni >> POSITION_A & MASK_AX) as i32 - (OFFSET_SJ) + 1) as isize);
                                trap = (*callinfo).callinfo_u.l.trap;
                            }
                            continue;
                        }
                        OPCODE_LTI => {
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            let cond: i32;
                            let im: i32 = (i >> POSITION_B & MASK_A) as i32 - (((1 << 8) - 1) >> 1);
                            if (*ra).get_tagvariant() == TagVariant::NumericInteger {
                                cond = ((*ra).as_integer().unwrap() < im as i64) as i32;
                            } else if (*ra).get_tagvariant() == TagVariant::NumericNumber {
                                let fa: f64 = (*ra).as_number().unwrap();
                                let fim: f64 = im as f64;
                                cond = (fa < fim) as i32;
                            } else {
                                let isf: bool = (i >> POSITION_C & MASK_A) != 0;
                                (*callinfo).callinfo_u.l.saved_program_counter = program_counter;
                                (*state).interpreter_top.stkidrel_pointer = (*callinfo).callinfo_top.stkidrel_pointer;
                                cond = luat_callorderitm(state, &(*ra), im, 0, isf, TM_LT);
                                trap = (*callinfo).callinfo_u.l.trap;
                            }
                            if cond != (i >> POSITION_K & MASK_K) as i32 {
                                program_counter = program_counter.add(1);
                            } else {
                                let ni: u32 = *program_counter;
                                program_counter =
                                    program_counter.offset(((ni >> POSITION_A & MASK_AX) as i32 - (OFFSET_SJ) + 1) as isize);
                                trap = (*callinfo).callinfo_u.l.trap;
                            }
                            continue;
                        }
                        OPCODE_LEI => {
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            let cond: i32;
                            let im: i32 = (i >> POSITION_B & MASK_A) as i32 - (((1 << 8) - 1) >> 1);
                            if (*ra).get_tagvariant() == TagVariant::NumericInteger {
                                cond = ((*ra).as_integer().unwrap() <= im as i64) as i32;
                            } else if (*ra).get_tagvariant() == TagVariant::NumericNumber {
                                let fa: f64 = (*ra).as_number().unwrap();
                                let fim: f64 = im as f64;
                                cond = (fa <= fim) as i32;
                            } else {
                                let isf: bool = (i >> POSITION_C & MASK_A) != 0;
                                (*callinfo).callinfo_u.l.saved_program_counter = program_counter;
                                (*state).interpreter_top.stkidrel_pointer = (*callinfo).callinfo_top.stkidrel_pointer;
                                cond = luat_callorderitm(state, &(*ra), im, 0, isf, TM_LE);
                                trap = (*callinfo).callinfo_u.l.trap;
                            }
                            if cond != (i >> POSITION_K & MASK_K) as i32 {
                                program_counter = program_counter.add(1);
                            } else {
                                let ni: u32 = *program_counter;
                                program_counter =
                                    program_counter.offset(((ni >> POSITION_A & MASK_AX) as i32 - (OFFSET_SJ) + 1) as isize);
                                trap = (*callinfo).callinfo_u.l.trap;
                            }
                            continue;
                        }
                        OPCODE_GTI => {
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            let cond: i32;
                            let im: i32 = (i >> POSITION_B & MASK_A) as i32 - (((1 << 8) - 1) >> 1);
                            if (*ra).get_tagvariant() == TagVariant::NumericInteger {
                                cond = ((*ra).as_integer().unwrap() > im as i64) as i32;
                            } else if (*ra).get_tagvariant() == TagVariant::NumericNumber {
                                let fa: f64 = (*ra).as_number().unwrap();
                                let fim: f64 = im as f64;
                                cond = (fa > fim) as i32;
                            } else {
                                let isf: bool = (i >> POSITION_C & MASK_A) != 0;
                                (*callinfo).callinfo_u.l.saved_program_counter = program_counter;
                                (*state).interpreter_top.stkidrel_pointer = (*callinfo).callinfo_top.stkidrel_pointer;
                                cond = luat_callorderitm(state, &(*ra), im, 1, isf, TM_LT);
                                trap = (*callinfo).callinfo_u.l.trap;
                            }
                            if cond != (i >> POSITION_K & MASK_K) as i32 {
                                program_counter = program_counter.add(1);
                            } else {
                                let ni: u32 = *program_counter;
                                program_counter =
                                    program_counter.offset(((ni >> POSITION_A & MASK_AX) as i32 - (OFFSET_SJ) + 1) as isize);
                                trap = (*callinfo).callinfo_u.l.trap;
                            }
                            continue;
                        }
                        OPCODE_GEI => {
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            let cond: i32;
                            let im: i32 = (i >> POSITION_B & MASK_A) as i32 - (((1 << 8) - 1) >> 1);
                            if (*ra).get_tagvariant() == TagVariant::NumericInteger {
                                cond = ((*ra).as_integer().unwrap() >= im as i64) as i32;
                            } else if (*ra).get_tagvariant() == TagVariant::NumericNumber {
                                let fa: f64 = (*ra).as_number().unwrap();
                                let fim: f64 = im as f64;
                                cond = (fa >= fim) as i32;
                            } else {
                                let isf: bool = (i >> POSITION_C & MASK_A) != 0;
                                (*callinfo).callinfo_u.l.saved_program_counter = program_counter;
                                (*state).interpreter_top.stkidrel_pointer = (*callinfo).callinfo_top.stkidrel_pointer;
                                cond = luat_callorderitm(state, &(*ra), im, 1, isf, TM_LE);
                                trap = (*callinfo).callinfo_u.l.trap;
                            }
                            if cond != (i >> POSITION_K & MASK_K) as i32 {
                                program_counter = program_counter.add(1);
                            } else {
                                let ni: u32 = *program_counter;
                                program_counter =
                                    program_counter.offset(((ni >> POSITION_A & MASK_AX) as i32 - (OFFSET_SJ) + 1) as isize);
                                trap = (*callinfo).callinfo_u.l.trap;
                            }
                            continue;
                        }
                        OPCODE_TEST => {
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            let cond: i32 = !((*ra).get_tagvariant() == TagVariant::BooleanFalse
                                || (*ra).get_tagvariant().to_tag_type().is_nil())
                                as i32;
                            if cond != (i >> POSITION_K & MASK_K) as i32 {
                                program_counter = program_counter.add(1);
                            } else {
                                let ni: u32 = *program_counter;
                                program_counter =
                                    program_counter.offset(((ni >> POSITION_A & MASK_AX) as i32 - (OFFSET_SJ) + 1) as isize);
                                trap = (*callinfo).callinfo_u.l.trap;
                            }
                            continue;
                        }
                        OPCODE_TESTSET => {
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            let rb: *mut TValue = &mut (*base.add((i >> POSITION_B & MASK_A) as usize));
                            if ((*rb).get_tagvariant() == TagVariant::BooleanFalse || (*rb).get_tagvariant().to_tag_type().is_nil())
                                as i32
                                == (i >> POSITION_K & MASK_K) as i32
                            {
                                program_counter = program_counter.add(1);
                            } else {
                                let io1_14: *mut TValue = &mut (*ra);
                                let io2_14: *const TValue = rb;
                                (*io1_14).copy_from(&(*io2_14));
                                let ni: u32 = *program_counter;
                                program_counter =
                                    program_counter.offset(((ni >> POSITION_A & MASK_AX) as i32 - (OFFSET_SJ) + 1) as isize);
                                trap = (*callinfo).callinfo_u.l.trap;
                            }
                            continue;
                        }
                        OPCODE_CALL => {
                            ra_call = base.add((i >> POSITION_A & MASK_A) as usize);
                            b_call = (i >> POSITION_B & MASK_A) as i32;
                            count_results = (i >> POSITION_C & MASK_A) as i32 - 1;
                            if b_call != 0 {
                                (*state).interpreter_top.stkidrel_pointer = ra_call.add(b_call as usize);
                            }
                            (*callinfo).callinfo_u.l.saved_program_counter = program_counter;
                            new_call_info = luad_precall(state, ra_call, count_results);
                            if !new_call_info.is_null() {
                                break '_returning;
                            }
                            trap = (*callinfo).callinfo_u.l.trap;
                            continue;
                        }
                        OPCODE_TAILCALL => {
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            let mut b: i32 = (i >> POSITION_B & MASK_A) as i32;

                            let nparams1: i32 = (i >> POSITION_C & MASK_A) as i32;
                            let delta: i32 = if nparams1 != 0 {
                                (*callinfo).callinfo_u.l.count_extra_arguments + nparams1
                            } else {
                                0
                            };
                            if b != 0 {
                                (*state).interpreter_top.stkidrel_pointer = ra.add(b as usize);
                            } else {
                                b = ((*state).interpreter_top.stkidrel_pointer).offset_from(ra) as i32;
                            }
                            (*callinfo).callinfo_u.l.saved_program_counter = program_counter;
                            if (i & 1_u32 << POSITION_K) as i32 != 0 {
                                luaf_closeupval(state, base);
                            }
                            let n: i32 = luad_pretailcall(state, callinfo, ra, b, delta);
                            if n < 0 {
                                continue '_startfunc;
                            }
                            (*callinfo).callinfo_function.stkidrel_pointer =
                                ((*callinfo).callinfo_function.stkidrel_pointer).sub(delta as usize);
                            luad_poscall(state, callinfo, n);
                            trap = (*callinfo).callinfo_u.l.trap;
                            break;
                        }
                        OPCODE_RETURN => {
                            let mut ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            let mut n: i32 = (i >> POSITION_B & MASK_A) as i32 - 1;
                            let nparams1_0: i32 = (i >> POSITION_C & MASK_A) as i32;
                            if n < 0 {
                                n = ((*state).interpreter_top.stkidrel_pointer).offset_from(ra) as i32;
                            }
                            (*callinfo).callinfo_u.l.saved_program_counter = program_counter;
                            if (i & 1_u32 << POSITION_K) as i32 != 0 {
                                (*callinfo).callinfo_u2.callinfoconstituentb_nres = n;
                                if (*state).interpreter_top.stkidrel_pointer < (*callinfo).callinfo_top.stkidrel_pointer {
                                    (*state).interpreter_top.stkidrel_pointer = (*callinfo).callinfo_top.stkidrel_pointer;
                                }
                                luaf_close(state, base, Status::Closing, 1);
                                trap = (*callinfo).callinfo_u.l.trap;
                                if trap != 0 {
                                    base = ((*callinfo).callinfo_function.stkidrel_pointer).add(1);
                                    ra = base.add((i >> POSITION_A & MASK_A) as usize);
                                }
                            }
                            if nparams1_0 != 0 {
                                (*callinfo).callinfo_function.stkidrel_pointer = ((*callinfo).callinfo_function.stkidrel_pointer)
                                    .sub(((*callinfo).callinfo_u.l.count_extra_arguments + nparams1_0) as usize);
                            }
                            (*state).interpreter_top.stkidrel_pointer = ra.add(n as usize);
                            luad_poscall(state, callinfo, n);
                            trap = (*callinfo).callinfo_u.l.trap;
                            break;
                        }
                        OPCODE_RETURN0 => {
                            if (*state).interpreter_hook_mask != 0 {
                                let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                                (*state).interpreter_top.stkidrel_pointer = ra;
                                (*callinfo).callinfo_u.l.saved_program_counter = program_counter;
                                luad_poscall(state, callinfo, 0);
                                trap = 1;
                            } else {
                                let mut nres: i32;
                                (*state).interpreter_callinfo = (*callinfo).callinfo_previous;
                                (*state).interpreter_top.stkidrel_pointer = base.sub(1);
                                nres = (*callinfo).callinfo_count_results;
                                while nres > 0 {
                                    let top = (*state).interpreter_top.stkidrel_pointer;
                                    (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.add(1);
                                    (*top).tvalue_set_tag_variant(TagVariant::NilNil);
                                    nres -= 1;
                                }
                            }
                            break;
                        }
                        OPCODE_RETURN1 => {
                            if (*state).interpreter_hook_mask != 0 {
                                let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                                (*state).interpreter_top.stkidrel_pointer = ra.add(1);
                                (*callinfo).callinfo_u.l.saved_program_counter = program_counter;
                                luad_poscall(state, callinfo, 1);
                                trap = 1;
                            } else {
                                let mut nres: i32 = (*callinfo).callinfo_count_results;
                                (*state).interpreter_callinfo = (*callinfo).callinfo_previous;
                                if nres == 0 {
                                    (*state).interpreter_top.stkidrel_pointer = base.sub(1);
                                } else {
                                    let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                                    let io1_15: *mut TValue = &mut (*base.sub(1));
                                    let io2_15: *const TValue = &mut (*ra);
                                    (*io1_15).copy_from(&(*io2_15));
                                    (*state).interpreter_top.stkidrel_pointer = base;
                                    while nres > 1 {
                                        let top = (*state).interpreter_top.stkidrel_pointer;
                                        (*state).interpreter_top.stkidrel_pointer =
                                            (*state).interpreter_top.stkidrel_pointer.add(1);
                                        (*top).tvalue_set_tag_variant(TagVariant::NilNil);
                                        nres -= 1;
                                    }
                                }
                            }
                            break;
                        }
                        OPCODE_FORLOOP => {
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            if (*ra.add(2)).get_tagvariant() == TagVariant::NumericInteger {
                                let count: usize = (*ra.add(1)).as_integer().unwrap() as usize;
                                if count > 0 {
                                    let step: i64 = (*ra.add(2)).as_integer().unwrap();
                                    let mut index: i64 = (*ra).as_integer().unwrap();
                                    let io3: *mut TValue = &mut (*ra.add(1));
                                    (*io3).set_integer(count.wrapping_sub(1_usize) as i64);
                                    index = (index as usize).wrapping_add(step as usize) as i64;
                                    let io4: *mut TValue = &mut (*ra);
                                    (*io4).set_integer(index);
                                    let io5: *mut TValue = &mut (*ra.add(3));
                                    (*io5).set_integer(index);
                                    program_counter = program_counter.sub((i >> POSITION_K & MASK_BX) as usize);
                                }
                            } else if floatforloop(ra) != 0 {
                                program_counter = program_counter.sub((i >> POSITION_K & MASK_BX) as usize);
                            }
                            trap = (*callinfo).callinfo_u.l.trap;
                            continue;
                        }
                        OPCODE_FORPREP => {
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            (*callinfo).callinfo_u.l.saved_program_counter = program_counter;
                            (*state).interpreter_top.stkidrel_pointer = (*callinfo).callinfo_top.stkidrel_pointer;
                            if forprep(state, ra) != 0 {
                                program_counter = program_counter.add(((i >> POSITION_K & MASK_BX) as i32 + 1) as usize);
                            }
                            continue;
                        }
                        OPCODE_TFORPREP => {
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            // Swap control (ra+2) and closing (ra+3) variables
                            let mut temp_tfor: TValue = core::mem::zeroed();
                            temp_tfor.copy_from(&*ra.add(3));
                            (*ra.add(3)).copy_from(&*ra.add(2));
                            (*ra.add(2)).copy_from(&temp_tfor);
                            (*callinfo).callinfo_u.l.saved_program_counter = program_counter;
                            (*state).interpreter_top.stkidrel_pointer = (*callinfo).callinfo_top.stkidrel_pointer;
                            luaf_newtbcupval(state, ra.add(2));
                            program_counter = program_counter.add((i >> POSITION_K & MASK_BX) as usize);
                            let current_pc = program_counter;
                            program_counter = program_counter.add(1);
                            i = *current_pc;
                            tfor_action = TFOR_CALL;
                        }
                        OPCODE_TFORCALL => {
                            tfor_action = TFOR_CALL;
                        }
                        OPCODE_TFORLOOP => {
                            tfor_action = TFOR_LOOP;
                        }
                        OPCODE_SETLIST => {
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            let mut n: i32 = (i >> POSITION_B & MASK_A) as i32;
                            let mut last: u32 = i >> POSITION_C & MASK_A;
                            let h: *mut Table = (*ra).as_table().unwrap();
                            if n == 0 {
                                n = ((*state).interpreter_top.stkidrel_pointer).offset_from(ra) as i32 - 1;
                            } else {
                                (*state).interpreter_top.stkidrel_pointer = (*callinfo).callinfo_top.stkidrel_pointer;
                            }
                            last = last.wrapping_add(n as u32);
                            if (i & 1_u32 << POSITION_K) as i32 != 0 {
                                last = last
                                    .wrapping_add(((*program_counter >> POSITION_A & MASK_AX) as i32 * ((1 << 8) - 1 + 1)) as u32);
                                program_counter = program_counter.add(1);
                            }
                            if last > (*h).table_a_size {
                                luah_resizearray(state, h, last as usize);
                            }
                            while n > 0 {
                                let value: *mut TValue = &mut (*ra.add(n as usize));
                                obj2arr(h, last.wrapping_sub(1), value);
                                last = last.wrapping_sub(1);
                                if (*value).is_collectable()
                                    && (*(h as *mut Object)).get_marked() & BLACKBIT != 0
                                    && (*(*value).as_object().unwrap()).get_marked() & WHITEBITS != 0
                                {
                                    ObjectWithGCList::luac_barrierback_(state, &mut *(h as *mut ObjectWithGCList));
                                }
                                n -= 1;
                            }
                            continue;
                        }
                        OPCODE_CLOSURE => {
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            let p: *mut Prototype = *((*(*closure).closure_payload.closurepayload_lprototype)
                                .prototype_prototypes
                                .vectort_pointer)
                                .add((i >> POSITION_K & MASK_BX) as usize);
                            (*callinfo).callinfo_u.l.saved_program_counter = program_counter;
                            (*state).interpreter_top.stkidrel_pointer = (*callinfo).callinfo_top.stkidrel_pointer;
                            pushclosure(
                                state,
                                p,
                                ((*closure).closure_upvalues)
                                    .closureupvalue_lvalues
                                    .as_mut_ptr(),
                                base,
                                ra,
                            );
                            if (*state).should_step() {
                                (*callinfo).callinfo_u.l.saved_program_counter = program_counter;
                                (*state).interpreter_top.stkidrel_pointer = ra.add(1);
                                (*state).do_gc_step();
                                trap = (*callinfo).callinfo_u.l.trap;
                            }
                            continue;
                        }
                        OPCODE_VARARG => {
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            let n: i32 = (i >> POSITION_C & MASK_A) as i32 - 1;
                            let vatab: i32 = if (i & 1_u32 << POSITION_K) as i32 != 0 {
                                (i >> POSITION_B & MASK_A) as i32
                            } else {
                                -1
                            };
                            (*callinfo).callinfo_u.l.saved_program_counter = program_counter;
                            (*state).interpreter_top.stkidrel_pointer = (*callinfo).callinfo_top.stkidrel_pointer;
                            luat_getvarargs(state, callinfo, ra, n, vatab);
                            trap = (*callinfo).callinfo_u.l.trap;
                            continue;
                        }
                        OPCODE_GETVARG => {
                            // OP_GETVARG: A B C — R[A] := vararg[R[C]]
                            // Used when varargs are hidden (not in a table)
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            let rc: *mut TValue = &mut (*base.add((i >> POSITION_C & MASK_A) as usize));
                            let nextra: i32 = (*callinfo).callinfo_u.l.count_extra_arguments;
                            // Check if rc is an integer key
                            let mut handled = false;
                            if (*rc).get_tagvariant() == TagVariant::NumericInteger {
                                let n = (*rc).as_integer().unwrap();
                                if (n as u64).wrapping_sub(1) < nextra as u64 {
                                    let slot = (*callinfo)
                                        .callinfo_function
                                        .stkidrel_pointer
                                        .sub(nextra as usize)
                                        .add(n as usize - 1);
                                    (*ra).copy_from(&*slot);
                                    handled = true;
                                }
                            } else if (*rc).get_tagvariant() == TagVariant::NumericNumber {
                                let f = (*rc).as_number().unwrap();
                                let n = f as i64;
                                if n as f64 == f && (n as u64).wrapping_sub(1) < nextra as u64 {
                                    let slot = (*callinfo)
                                        .callinfo_function
                                        .stkidrel_pointer
                                        .sub(nextra as usize)
                                        .add(n as usize - 1);
                                    (*ra).copy_from(&*slot);
                                    handled = true;
                                }
                            } else if (*rc).get_tagvariant() == TagVariant::StringShort {
                                let ts = (*rc).as_string().unwrap();
                                let len = (*ts).get_length();
                                let s = (*ts).get_contents_mut();
                                if len == 1 && *s == b'n' as i8 {
                                    (*ra).set_integer(nextra as i64);
                                    handled = true;
                                }
                            }
                            if !handled {
                                (*ra).tvalue_set_tag_variant(TagVariant::NilNil);
                            }
                            continue;
                        }
                        OPCODE_ERRNNIL => {
                            // OP_ERRNNIL: A Bx — raise error if R[A] is not nil
                            let ra_errnnil: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            if (*ra_errnnil).get_tagvariant() != TagVariant::NilNil {
                                let bx = (i >> POSITION_K & MASK_BX) as i32;
                                let name: *const i8 = if bx > 0 {
                                    let tv = &*k.add((bx - 1) as usize);
                                    (*tv.as_string().unwrap()).get_contents_mut()
                                } else {
                                    c"?".as_ptr()
                                };
                                (*callinfo).callinfo_u.l.saved_program_counter = program_counter;
                                luag_runerror(
                                    state,
                                    c"global '%s' already defined".as_ptr(),
                                    &[name.into()],
                                );
                            }
                            continue;
                        }
                        OPCODE_VARARGPREP => {
                            (*callinfo).callinfo_u.l.saved_program_counter = program_counter;
                            luat_adjustvarargs(
                                state,
                                (i >> POSITION_A & MASK_A) as i32,
                                callinfo,
                                (*closure).closure_payload.closurepayload_lprototype,
                            );
                            trap = (*callinfo).callinfo_u.l.trap;
                            if trap != 0 {
                                luad_hookcall(state, callinfo);
                                (*state).interpreter_old_program_counter = 1;
                            }
                            base = ((*callinfo).callinfo_function.stkidrel_pointer).add(1);
                            continue;
                        }
                        OPCODE_EXTRAARG | _ => {
                            continue;
                        }
                    }
                    match tfor_action {
                        TFOR_CALL => {
                            let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                            // Copy control, state, function to ra+3..ra+5
                            (*ra.add(5)).copy_from(&*ra.add(3)); // control -> ra+5
                            (*ra.add(4)).copy_from(&*ra.add(1)); // state -> ra+4
                            (*ra.add(3)).copy_from(&*ra); // function -> ra+3
                            (*state).interpreter_top.stkidrel_pointer = ra.add(3).add(3);
                            (*callinfo).callinfo_u.l.saved_program_counter = program_counter;
                            ccall(state, ra.add(3), (i >> POSITION_C & MASK_A) as i32, 1);
                            trap = (*callinfo).callinfo_u.l.trap;
                            if trap != 0 {
                                base = ((*callinfo).callinfo_function.stkidrel_pointer).add(1);
                            }
                            let current_pc = program_counter;
                            program_counter = program_counter.add(1);
                            i = *current_pc;
                        }
                        _ => {}
                    }
                    let ra: *mut TValue = base.add((i >> POSITION_A & MASK_A) as usize);
                    if !(*ra.add(3)).get_tagvariant().to_tag_type().is_nil() {
                        program_counter = program_counter.sub((i >> POSITION_K & MASK_BX) as usize);
                    }
                }
                if (*callinfo).callinfo_callstatus as i32 & CALLSTATUS_HOOKED != 0 {
                    break '_startfunc;
                }
                callinfo = (*callinfo).callinfo_previous;
            }
            callinfo = new_call_info;
        }
    }
}
pub unsafe fn findfield(state: *mut State, objidx: i32, level: i32) -> bool {
    unsafe {
        if level == 0 || (lua_type(state, -1) != Some(TagType::Table)) {
            return false;
        }
        (*state).push_nil();
        while lua_next(state, -2) != 0 {
            if lua_type(state, -2) == Some(TagType::String) {
                if lua_rawequal(state, objidx, -1) {
                    lua_settop(state, -2);
                    return true;
                } else if findfield(state, objidx, level - 1) {
                    lua_pushstring(state, c".".as_ptr());
                    lua_copy(state, -1, -3);
                    lua_settop(state, -2);
                    lua_concat(state, 3);
                    return true;
                }
            }
            lua_settop(state, -2);
        }
        false
    }
}
pub unsafe fn pushglobalfuncname(state: *mut State, debuginfo: *mut DebugInfo) -> bool {
    unsafe {
        let top: i32 = (*state).get_top();
        lua_getinfo(state, c"f".as_ptr(), debuginfo);
        lua_getfield(state, LUA_REGISTRYINDEX, c"_LOADED".as_ptr());
        lual_checkstack(state, 6, c"not enough stack".as_ptr());
        if findfield(state, top + 1, 2) {
            let name: *const i8 = lua_tolstring(state, -1, null_mut());
            if std::slice::from_raw_parts(name as *const u8, 3) == b"_G." {
                lua_pushstring(state, name.add(3));
                lua_rotate(state, -2, -1);
                lua_settop(state, -2);
            }
            lua_copy(state, -1, top + 1);
            lua_settop(state, top + 1);
            true
        } else {
            lua_settop(state, top);
            false
        }
    }
}
pub unsafe fn pushfuncname(state: *mut State, debuginfo: *mut DebugInfo) {
    unsafe {
        if *(*debuginfo).debuginfo_name_what as i32 != Character::Null as i32 {
            lua_pushfstring(
                state,
                c"%s '%s'".as_ptr(),
                &[
                    (*debuginfo).debuginfo_name_what.into(),
                    (*debuginfo).debuginfo_name.into(),
                ],
            );
        } else if *(*debuginfo).debuginfo_what as i32 == Character::LowerM as i32 {
            lua_pushstring(state, c"main chunk".as_ptr());
        } else if pushglobalfuncname(state, debuginfo) {
            lua_pushfstring(
                state,
                c"function '%s'".as_ptr(),
                &[lua_tolstring(state, -1, null_mut()).into()],
            );
            lua_rotate(state, -2, -1);
            lua_settop(state, -2);
        } else if *(*debuginfo).debuginfo_what as i32 != Character::UpperC as i32 {
            lua_pushfstring(
                state,
                c"function <%s:%d>".as_ptr(),
                &[
                    ((*debuginfo).debuginfo_short_source).as_mut_ptr().into(),
                    (*debuginfo).debuginfo_line_defined.into(),
                ],
            );
        } else {
            lua_pushstring(state, c"?".as_ptr());
        };
    }
}
pub unsafe fn lastlevel(state: *mut State) -> i32 {
    unsafe {
        let mut debuginfo: DebugInfo = DebugInfo::new();
        let mut li: i32 = 1;
        let mut le: i32 = 1;
        while lua_getstack(state, le, &mut debuginfo) != 0 {
            li = le;
            le *= 2;
        }
        while li < le {
            let m: i32 = (li + le) / 2;
            if lua_getstack(state, m, &mut debuginfo) != 0 {
                li = m + 1;
            } else {
                le = m;
            }
        }
        le - 1
    }
}
const LEVELS1: i32 = 10;
const LEVELS2: i32 = 11;
pub unsafe fn lual_traceback(state: *mut State, other_state: *mut State, message: *const i8, mut level: i32) {
    unsafe {
        let mut b = Buffer::new();
        let mut debuginfo: DebugInfo = DebugInfo::new();
        let last: i32 = lastlevel(other_state);
        let mut limit2show: i32 = if last - level > LEVELS1 + LEVELS2 {
            LEVELS1
        } else {
            -1
        };
        b.initialize(state);
        if !message.is_null() {
            b.add_string(message);
            (b.buffer_loads.get_length() < b.buffer_loads.get_size() || !(b.prepare_with_size(1)).is_null()) as i32;
            let write_offset = b.buffer_loads.get_length();
            b.buffer_loads
                .set_length((b.buffer_loads.get_length() + 1) as usize);
            *(b.buffer_loads.loads_pointer).add(write_offset as usize) = Character::LineFeed as i8;
        }
        b.add_string(c"stack traceback:".as_ptr());
        loop {
            let prev_level = level;
            level += 1;
            if lua_getstack(other_state, prev_level, &mut debuginfo) == 0 {
                break;
            }
            let prev_limit = limit2show;
            limit2show -= 1;
            if prev_limit == 0 {
                let n: i32 = last - level - LEVELS2 + 1;
                lua_pushfstring(
                    state,
                    c"\n\t...\t(skipping %d levels)".as_ptr(),
                    &[n.into()],
                );
                b.add_value();
                level += n;
            } else {
                lua_getinfo(other_state, c"Slnt".as_ptr(), &mut debuginfo);
                if debuginfo.debuginfo_current_line <= 0 {
                    lua_pushfstring(
                        state,
                        c"\n\t%s: in ".as_ptr(),
                        &[(debuginfo.debuginfo_short_source).as_mut_ptr().into()],
                    );
                } else {
                    lua_pushfstring(
                        state,
                        c"\n\t%s:%d: in ".as_ptr(),
                        &[
                            (debuginfo.debuginfo_short_source).as_mut_ptr().into(),
                            debuginfo.debuginfo_current_line.into(),
                        ],
                    );
                }
                b.add_value();
                pushfuncname(state, &mut debuginfo);
                b.add_value();
                if debuginfo.debuginfo_is_tail_call {
                    b.add_string(c"\n\t(...tail calls...)".as_ptr());
                }
            }
        }
        b.push_result();
    }
}
pub unsafe fn lual_argerror(state: *mut State, mut arg: i32, extramsg: *const i8) -> i32 {
    unsafe {
        let mut debuginfo: DebugInfo = DebugInfo::new();
        if lua_getstack(state, 0, &mut debuginfo) == 0 {
            return lual_error(
                state,
                c"bad argument #%d (%s)".as_ptr(),
                &[arg.into(), extramsg.into()],
            );
        }
        lua_getinfo(state, c"n".as_ptr(), &mut debuginfo);
        if std::ffi::CStr::from_ptr(debuginfo.debuginfo_name_what) == c"method" {
            arg -= 1;
            if arg == 0 {
                return lual_error(
                    state,
                    c"calling '%s' on bad self (%s)".as_ptr(),
                    &[debuginfo.debuginfo_name.into(), extramsg.into()],
                );
            }
        }
        if debuginfo.debuginfo_name.is_null() {
            debuginfo.debuginfo_name = if pushglobalfuncname(state, &mut debuginfo) {
                lua_tolstring(state, -1, null_mut())
            } else {
                c"?".as_ptr()
            };
        }
        lual_error(
            state,
            c"bad argument #%d to '%s' (%s)".as_ptr(),
            &[arg.into(), debuginfo.debuginfo_name.into(), extramsg.into()],
        )
    }
}
pub unsafe fn lual_typeerror(state: *mut State, arg: i32, tname: *const i8) -> i32 {
    unsafe {
        let typearg: *const i8;
        if lual_getmetafield(state, arg, c"__name".as_ptr()) == TagType::String {
            typearg = lua_tolstring(state, -1, null_mut());
        } else if lua_type(state, arg) == Some(TagType::Pointer) {
            typearg = c"light userdata".as_ptr();
        } else {
            typearg = lua_typename(state, lua_type(state, arg));
        }
        let message: *const i8 = lua_pushfstring(
            state,
            c"%s expected, got %s".as_ptr(),
            &[tname.into(), typearg.into()],
        );
        lual_argerror(state, arg, message)
    }
}
pub unsafe fn tag_error2(state: *mut State, arg: i32, tagtype: Option<TagType>) {
    unsafe {
        lual_typeerror(state, arg, lua_typename(state, tagtype));
    }
}
pub unsafe fn lual_where(state: *mut State, level: i32) {
    unsafe {
        let mut debuginfo: DebugInfo = DebugInfo::new();
        if lua_getstack(state, level, &mut debuginfo) != 0 {
            lua_getinfo(state, c"Sl".as_ptr(), &mut debuginfo);
            if debuginfo.debuginfo_current_line > 0 {
                lua_pushfstring(
                    state,
                    c"%s:%d: ".as_ptr(),
                    &[
                        (debuginfo.debuginfo_short_source).as_mut_ptr().into(),
                        debuginfo.debuginfo_current_line.into(),
                    ],
                );
                return;
            }
        }
        lua_pushfstring(state, c"".as_ptr(), &[]);
    }
}
pub unsafe fn lual_error(state: *mut State, fmt: *const i8, args: &[crate::fmtarg::FmtArg]) -> i32 {
    unsafe {
        lual_where(state, 1);
        lua_pushvfstring(state, fmt, args);
        lua_concat(state, 2);
        lua_error(state)
    }
}
pub unsafe fn lual_fileresult(state: *mut State, stat: i32, fname: *const i8) -> i32 {
    unsafe {
        let en: i32 = get_errno();
        if stat != 0 {
            (*state).push_boolean(true);
            1
        } else {
            (*state).push_nil();
            let message: *const i8 = if en != 0 {
                os_strerror(en)
            } else {
                c"(no extra info)".as_ptr()
            };
            if !fname.is_null() {
                lua_pushfstring(state, c"%s: %s".as_ptr(), &[fname.into(), message.into()]);
            } else {
                lua_pushstring(state, message);
            }
            (*state).push_integer(en as i64);
            3
        }
    }
}
pub unsafe fn lual_execresult(state: *mut State, mut stat: i32) -> i32 {
    unsafe {
        if stat != 0 && get_errno() != 0 {
            lual_fileresult(state, 0, null())
        } else {
            let mut what: *const i8 = c"exit".as_ptr();
            if stat & 0x7f_i32 == 0 {
                stat = (stat & 0xff00_i32) >> 8;
            } else if ((stat & 0x7f_i32) + 1) >> 1 > 0 {
                stat &= 0x7f_i32;
                what = c"signal".as_ptr();
            }
            if *what as i32 == Character::LowerE as i32 && stat == 0 {
                (*state).push_boolean(true);
            } else {
                (*state).push_nil();
            }
            lua_pushstring(state, what);
            (*state).push_integer(stat as i64);
            3
        }
    }
}
pub unsafe fn lual_newmetatable(state: *mut State, tname: *const i8) -> i32 {
    unsafe {
        if lua_getfield(state, LUA_REGISTRYINDEX, tname) != TagType::Nil {
            0
        } else {
            lua_settop(state, -2);
            (*state).lua_createtable();
            lua_pushstring(state, tname);
            lua_setfield(state, -2, c"__name".as_ptr());
            lua_pushvalue(state, -1);
            lua_setfield(state, LUA_REGISTRYINDEX, tname);
            1
        }
    }
}
pub unsafe fn lual_setmetatable(state: *mut State, tname: *const i8) {
    unsafe {
        lua_getfield(state, LUA_REGISTRYINDEX, tname);
        lua_setmetatable(state, -2);
    }
}
pub unsafe fn lual_testudata(state: *mut State, arbitrary_data: i32, tname: *const i8) -> *mut std::ffi::c_void {
    unsafe {
        let mut p: *mut std::ffi::c_void = (*state).to_pointer(arbitrary_data);
        if !p.is_null() && (*state).lua_getmetatable(arbitrary_data) {
            lua_getfield(state, LUA_REGISTRYINDEX, tname);
            if !lua_rawequal(state, -1, -2) {
                p = null_mut();
            }
            lua_settop(state, -2 - 1);
            return p;
        }
        null_mut()
    }
}
pub unsafe fn lual_checkudata(state: *mut State, arbitrary_data: i32, tname: *const i8) -> *mut std::ffi::c_void {
    unsafe {
        let p: *mut std::ffi::c_void = lual_testudata(state, arbitrary_data, tname);
        if p.is_null() {
            lual_typeerror(state, arbitrary_data, tname);
        }
        p
    }
}
pub unsafe fn lual_checkoption(state: *mut State, arg: i32, def: *const i8, lst: *const *const i8) -> i32 {
    unsafe {
        let name: *const i8 = if !def.is_null() {
            lual_optlstring(state, arg, def, null_mut())
        } else {
            lual_checklstring(state, arg, null_mut())
        };
        let mut i: i32;
        i = 0;
        while !(*lst.add(i as usize)).is_null() {
            if std::ffi::CStr::from_ptr(*lst.add(i as usize)) == std::ffi::CStr::from_ptr(name) {
                return i;
            }
            i += 1;
        }
        lual_argerror(
            state,
            arg,
            lua_pushfstring(state, c"invalid option '%s'".as_ptr(), &[name.into()]),
        )
    }
}
pub unsafe fn lual_checkstack(state: *mut State, space: i32, message: *const i8) {
    unsafe {
        if lua_checkstack(state, space) == 0 {
            if message.is_null() {
                lual_error(state, c"stack overflow".as_ptr(), &[]);
            } else {
                lual_error(state, c"stack overflow (%s)".as_ptr(), &[message.into()]);
            }
        }
    }
}
pub unsafe fn lual_checkany(state: *mut State, arg: i32) {
    unsafe {
        if lua_type(state, arg).is_none() {
            lual_argerror(state, arg, c"value expected".as_ptr());
        }
    }
}
pub unsafe fn lual_checklstring(state: *mut State, arg: i32, length: *mut usize) -> *const i8 {
    unsafe {
        let s: *const i8 = lua_tolstring(state, arg, length);
        if s.is_null() {
            tag_error2(state, arg, Some(TagType::String));
        }
        s
    }
}
pub unsafe fn lual_optlstring(state: *mut State, arg: i32, def: *const i8, length: *mut usize) -> *const i8 {
    unsafe {
        match lua_type(state, arg) {
            None | Some(TagType::Nil) => {
                if !length.is_null() {
                    *length = if !def.is_null() {
                        cstr_len(def)
                    } else {
                        0usize
                    };
                }
                def
            }
            _ => lual_checklstring(state, arg, length),
        }
    }
}
pub unsafe fn lual_checknumber(state: *mut State, arg: i32) -> f64 {
    unsafe {
        let mut is_number = false;
        let d: f64 = lua_tonumberx(state, arg, &mut is_number);
        if !is_number {
            tag_error2(state, arg, Some(TagType::Numeric));
        }
        d
    }
}
pub unsafe fn lual_optnumber(state: *mut State, arg: i32, def: f64) -> f64 {
    unsafe {
        match lua_type(state, arg) {
            None | Some(TagType::Nil) => def,
            _ => lual_checknumber(state, arg),
        }
    }
}
pub unsafe fn interror(state: *mut State, arg: i32) {
    unsafe {
        if lua_isnumber(state, arg) {
            lual_argerror(state, arg, c"number has no integer representation".as_ptr());
        } else {
            tag_error2(state, arg, Some(TagType::Numeric));
        };
    }
}
pub unsafe fn lual_checkinteger(state: *mut State, arg: i32) -> i64 {
    unsafe {
        let mut is_number = false;
        let ret: i64 = lua_tointegerx(state, arg, &mut is_number);
        if !is_number {
            interror(state, arg);
        }
        ret
    }
}
pub unsafe fn lual_optinteger(state: *mut State, arg: i32, def: i64) -> i64 {
    unsafe {
        match lua_type(state, arg) {
            None | Some(TagType::Nil) => def,
            _ => lual_checkinteger(state, arg),
        }
    }
}
pub unsafe fn get_f(mut _state: *mut State, arbitrary_data: *mut std::ffi::c_void, size: *mut usize) -> *const i8 {
    unsafe {
        let lf: *mut LoadF = arbitrary_data as *mut LoadF;
        if (*lf).loadf_n > 0 {
            *size = (*lf).loadf_n as usize;
            (*lf).loadf_n = 0;
        } else {
            match (*lf).loadf_source.as_mut() {
                None => return null(),
                Some(source) => {
                    let buf = std::slice::from_raw_parts_mut((*lf).loadf_buffer.as_mut_ptr() as *mut u8, LUAL_BUFFERSIZE);
                    match source.read_chunk(buf) {
                        Ok(0) => return null(),
                        Ok(n) => *size = n,
                        Err(_) => {
                            (*lf).loadf_had_error = true;
                            return null();
                        }
                    }
                }
            }
        }
        ((*lf).loadf_buffer).as_mut_ptr()
    }
}
pub unsafe fn errfile(state: *mut State, what: *const i8, fnameindex: i32) -> Status {
    unsafe {
        let err: i32 = get_errno();
        let filename: *const i8 = (lua_tolstring(state, fnameindex, null_mut())).add(1);
        if err != 0 {
            lua_pushfstring(
                state,
                c"cannot %s %s: %s".as_ptr(),
                &[what.into(), filename.into(), os_strerror(err).into()],
            );
        } else {
            lua_pushfstring(
                state,
                c"cannot %s %s".as_ptr(),
                &[what.into(), filename.into()],
            );
        }
        lua_rotate(state, fnameindex, -1);
        lua_settop(state, -2);
        Status::FileError
    }
}
fn skip_bom(source: &mut LoadSource) -> i32 {
    match source.read_byte() {
        None => -1,
        Some(c) => {
            if c == 0xef && source.read_byte() == Some(0xbb) && source.read_byte() == Some(0xbf) {
                source.read_byte().map_or(-1, |b| b as i32)
            } else {
                c as i32
            }
        }
    }
}
fn skipcomment(source: &mut LoadSource, pointer: *mut i32) -> i32 {
    unsafe {
        *pointer = skip_bom(source);
        let mut c: i32 = *pointer;
        if c == Character::Octothorpe as i32 {
            loop {
                c = source.read_byte().map_or(-1, |b| b as i32);
                if c == -1 || c == Character::LineFeed as i32 {
                    break;
                }
            }
            *pointer = source.read_byte().map_or(-1, |b| b as i32);
            1
        } else {
            0
        }
    }
}
thread_local! {
    static EMBEDDED_BASE_DIR: RefCell<String> = const { RefCell::new(String::new()) };
}
fn set_embedded_base_dir(path: &str) {
    let base = match path.rfind('/') {
        Some(pos) => &path[..pos + 1],
        None => "",
    };
    EMBEDDED_BASE_DIR.with(|d| *d.borrow_mut() = base.to_string());
}
pub(crate) fn resolve_embedded(name: &str) -> Option<&'static [u8]> {
    EMBEDDED_BASE_DIR.with(|d| {
        let base = d.borrow();
        if !base.is_empty() {
            let full = format!("{}{}", base, name);
            if let Some(content) = crate::embedded_resources::lookup(&full) {
                return Some(content);
            }
        }
        crate::embedded_resources::lookup(name)
    })
}
pub(crate) fn skip_shebang(content: &[u8]) -> &[u8] {
    if content.starts_with(b"#") {
        match content.iter().position(|&b| b == b'\n') {
            Some(pos) => &content[pos..],
            None => &[],
        }
    } else {
        content
    }
}
pub unsafe fn lual_loadfilex(state: *mut State, filename: *const i8, mode: *const i8) -> Status {
    unsafe {
        let mut lf: LoadF = LoadF {
            loadf_n: 0,
            loadf_source: None,
            loadf_had_error: false,
            loadf_buffer: [0; LUAL_BUFFERSIZE],
        };
        let mut c: i32 = 0;
        let fnameindex: i32 = (*state).get_top() + 1;
        if !filename.is_null() {
            let fname_cstr = std::ffi::CStr::from_ptr(filename);
            if let Ok(fname_str) = fname_cstr.to_str() {
                // Explicit embedded resource: @path/to/file.lua
                if let Some(stripped) = fname_str.strip_prefix('@')
                    && let Some(raw) = crate::embedded_resources::lookup(stripped)
                {
                    set_embedded_base_dir(stripped);
                    let content = skip_shebang(raw);
                    lua_pushfstring(state, c"@%s".as_ptr(), &[filename.into()]);
                    let name_ptr = lua_tolstring(state, -1, null_mut());
                    let status = lual_loadbufferx(
                        state,
                        content.as_ptr() as *const i8,
                        content.len(),
                        name_ptr,
                        mode,
                    );
                    lua_rotate(state, fnameindex, -1);
                    lua_settop(state, -2);
                    return status;
                }
            }
        }
        if filename.is_null() {
            lua_pushstring(state, c"=stdin".as_ptr());
            lf.loadf_source = Some(LoadSource::Stdin);
        } else {
            lua_pushfstring(state, c"@%s".as_ptr(), &[filename.into()]);
            let fname_cstr = std::ffi::CStr::from_ptr(filename);
            match fname_cstr
                .to_str()
                .ok()
                .and_then(|s| std::fs::File::open(s).ok())
            {
                Some(file) => {
                    lf.loadf_source = Some(LoadSource::File(file));
                }
                None => {
                    // Fallback: try embedded resources relative to current embedded base dir
                    if let Ok(fname_str) = fname_cstr.to_str()
                        && let Some(raw) = resolve_embedded(fname_str)
                    {
                        let content = skip_shebang(raw);
                        let name_ptr = lua_tolstring(state, -1, null_mut());
                        let status = lual_loadbufferx(
                            state,
                            content.as_ptr() as *const i8,
                            content.len(),
                            name_ptr,
                            mode,
                        );
                        lua_rotate(state, fnameindex, -1);
                        lua_settop(state, -2);
                        return status;
                    }
                    return errfile(state, c"open".as_ptr(), fnameindex);
                }
            }
        }
        lf.loadf_n = 0;
        if skipcomment(lf.loadf_source.as_mut().unwrap(), &mut c) != 0 {
            let write_index = lf.loadf_n;
            lf.loadf_n += 1;
            lf.loadf_buffer[write_index as usize] = Character::LineFeed as i8;
        }
        if c == 0x1B {
            lf.loadf_n = 0;
            if !filename.is_null() {
                // On Unix, "r" and "rb" are identical; just reopen the file
                let fname_cstr = std::ffi::CStr::from_ptr(filename);
                match fname_cstr
                    .to_str()
                    .ok()
                    .and_then(|s| std::fs::File::open(s).ok())
                {
                    Some(file) => {
                        lf.loadf_source = Some(LoadSource::File(file));
                    }
                    None => {
                        return errfile(state, c"reopen".as_ptr(), fnameindex);
                    }
                }
                skipcomment(lf.loadf_source.as_mut().unwrap(), &mut c);
            }
        }
        if c != -1 {
            let write_index = lf.loadf_n;
            lf.loadf_n += 1;
            lf.loadf_buffer[write_index as usize] = c as i8;
        }
        let reader = Reader::new(Some(
            get_f as unsafe fn(*mut State, *mut std::ffi::c_void, *mut usize) -> *const i8,
        ));
        let status = lua_load(
            state,
            reader,
            &mut lf as *mut LoadF as *mut std::ffi::c_void,
            lua_tolstring(state, -1, null_mut()),
            mode,
        );
        if !filename.is_null() {
            lf.loadf_source = None; // close the file
        }
        if lf.loadf_had_error {
            lua_settop(state, fnameindex);
            return errfile(state, c"read".as_ptr(), fnameindex);
        }
        lua_rotate(state, fnameindex, -1);
        lua_settop(state, -2);
        status
    }
}
pub unsafe fn get_s(mut _state: *mut State, arbitrary_data: *mut std::ffi::c_void, size: *mut usize) -> *const i8 {
    unsafe {
        let load_s: *mut VectorT<i8> = arbitrary_data as *mut VectorT<i8>;
        if (*load_s).get_size() == 0 {
            null()
        } else {
            let (capitulated_pointer, capitulated_size) = (*load_s).capitulate();
            *size = capitulated_size;
            capitulated_pointer
        }
    }
}
pub unsafe fn lual_loadbufferx(state: *mut State, buffer: *const i8, size: usize, name: *const i8, mode: *const i8) -> Status {
    unsafe {
        let mut load_s: VectorT<i8> = VectorT::<i8>::new();
        load_s.inject(buffer as *mut i8, size);
        let reader: Reader = Reader::new(Some(
            get_s as unsafe fn(*mut State, *mut std::ffi::c_void, *mut usize) -> *const i8,
        ));
        lua_load(
            state,
            reader,
            &mut load_s as *mut VectorT<i8> as *mut std::ffi::c_void,
            name,
            mode,
        )
    }
}
pub unsafe fn lual_getmetafield(state: *mut State, obj: i32, event: *const i8) -> TagType {
    unsafe {
        if (*state).lua_getmetatable(obj) {
            lua_pushstring(state, event);
            let tagtype = lua_rawget(state, -2);
            if tagtype == TagType::Nil {
                lua_settop(state, -3);
            } else {
                lua_rotate(state, -2, -1);
                lua_settop(state, -2);
            }
            tagtype
        } else {
            TagType::Nil
        }
    }
}
pub unsafe fn lual_callmeta(state: *mut State, mut obj: i32, event: *const i8) -> bool {
    unsafe {
        obj = lua_absindex(state, obj);
        if lual_getmetafield(state, obj, event) == TagType::Nil {
            return false;
        }
        lua_pushvalue(state, obj);
        (*state).lua_callk(1, 1, 0, None);
        true
    }
}
pub unsafe fn lual_len(state: *mut State, index: i32) -> i64 {
    unsafe {
        let mut is_number = false;
        lua_len(state, index);
        let l: i64 = lua_tointegerx(state, -1, &mut is_number);
        if !is_number {
            lual_error(state, c"object length is not an integer".as_ptr(), &[]);
        }
        lua_settop(state, -2);
        l
    }
}
pub unsafe fn lual_tolstring(state: *mut State, mut index: i32, length: *mut usize) -> *const i8 {
    unsafe {
        index = lua_absindex(state, index);
        if lual_callmeta(state, index, c"__tostring".as_ptr()) {
            if !lua_isstring(state, -1) {
                lual_error(state, c"'__tostring' must return a string".as_ptr(), &[]);
            }
        } else {
            match lua_type(state, index) {
                Some(TagType::Numeric) => {
                    if lua_isinteger(state, index) {
                        lua_pushfstring(
                            state,
                            c"%I".as_ptr(),
                            &[lua_tointegerx(state, index, null_mut()).into()],
                        );
                    } else {
                        lua_pushfstring(
                            state,
                            c"%f".as_ptr(),
                            &[lua_tonumberx(state, index, null_mut()).into()],
                        );
                    }
                }
                Some(TagType::String) => {
                    lua_pushvalue(state, index);
                }
                Some(TagType::Boolean) => {
                    lua_pushstring(
                        state,
                        if lua_toboolean(state, index) {
                            c"true".as_ptr()
                        } else {
                            c"false".as_ptr()
                        },
                    );
                }
                Some(TagType::Nil) => {
                    lua_pushstring(state, c"nil".as_ptr());
                }
                _ => {
                    let tagtype = lual_getmetafield(state, index, c"__name".as_ptr());
                    let kind: *const i8 = if tagtype == TagType::String {
                        lua_tolstring(state, -1, null_mut())
                    } else {
                        lua_typename(state, lua_type(state, index))
                    };
                    lua_pushfstring(
                        state,
                        c"%s: %p".as_ptr(),
                        &[kind.into(), (*state).to_pointer(index).into()],
                    );
                    if tagtype != TagType::Nil {
                        lua_rotate(state, -2, -1);
                        lua_settop(state, -2);
                    }
                }
            }
        }
        lua_tolstring(state, -1, length)
    }
}
pub unsafe fn lual_setfuncs(
    state: *mut State,
    registered_functions: *const RegisteredFunction,
    count_registered_functions: usize,
    count_upvalues: i32,
) {
    unsafe {
        lual_checkstack(state, count_upvalues, c"too many upvalues".as_ptr());
        for it in 0..count_registered_functions {
            if (*registered_functions.add(it))
                .registeredfunction_function
                .is_none()
            {
                (*state).push_boolean(false);
            } else {
                for _ in 0..count_upvalues {
                    lua_pushvalue(state, -count_upvalues);
                }
                lua_pushcclosure(
                    state,
                    (*registered_functions.add(it)).registeredfunction_function,
                    count_upvalues,
                );
            }
            lua_setfield(
                state,
                -(count_upvalues + 2),
                (*registered_functions.add(it)).registeredfunction_name,
            );
        }
        lua_settop(state, -count_upvalues - 1);
    }
}
pub unsafe fn lual_getsubtable(state: *mut State, mut index: i32, fname: *const i8) -> i32 {
    unsafe {
        if lua_getfield(state, index, fname) == TagType::Table {
            1
        } else {
            lua_settop(state, -2);
            index = lua_absindex(state, index);
            (*state).lua_createtable();
            lua_pushvalue(state, -1);
            lua_setfield(state, index, fname);
            0
        }
    }
}
pub unsafe fn lual_requiref(state: *mut State, modname: *const i8, openf: CFunction, glb: i32) {
    unsafe {
        lual_getsubtable(state, LUA_REGISTRYINDEX, c"_LOADED".as_ptr());
        lua_getfield(state, -1, modname);
        if !lua_toboolean(state, -1) {
            lua_settop(state, -2);
            lua_pushcclosure(state, openf, 0);
            lua_pushstring(state, modname);
            (*state).lua_callk(1, 1, 0, None);
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
pub unsafe fn lual_addgsub(b: *mut Buffer, mut s: *const i8, p: *const i8, r: *const i8) {
    unsafe {
        let mut wild: *const i8;
        let l = cstr_len(p);
        loop {
            wild = cstr_str(s, p);
            if wild.is_null() {
                break;
            }
            (*b).add_string_with_length(s, wild.offset_from(s) as usize);
            (*b).add_string(r);
            s = wild.add(l);
        }
        (*b).add_string(s);
    }
}
pub unsafe fn lual_gsub(state: *mut State, s: *const i8, p: *const i8, r: *const i8) -> *const i8 {
    unsafe {
        let mut b = Buffer::new();
        b.initialize(state);
        lual_addgsub(&mut b, s, p, r);
        b.push_result();
        lua_tolstring(state, -1, null_mut())
    }
}
pub unsafe fn raw_allocate(ptr: *mut std::ffi::c_void, old_size: usize, newsize: usize) -> *mut std::ffi::c_void {
    unsafe {
        if newsize == 0 {
            if !ptr.is_null() && old_size > 0 {
                let layout = std::alloc::Layout::from_size_align(old_size, std::mem::align_of::<usize>()).unwrap();
                std::alloc::dealloc(ptr as *mut u8, layout);
            }
            null_mut()
        } else if ptr.is_null() || old_size == 0 {
            let layout = std::alloc::Layout::from_size_align(newsize, std::mem::align_of::<usize>()).unwrap();
            std::alloc::alloc(layout) as *mut std::ffi::c_void
        } else {
            let layout = std::alloc::Layout::from_size_align(old_size, std::mem::align_of::<usize>()).unwrap();
            std::alloc::realloc(ptr as *mut u8, layout, newsize) as *mut std::ffi::c_void
        }
    }
}
pub unsafe fn panic(state: *mut State) -> i32 {
    unsafe {
        let message: *const i8 = if lua_type(state, -1) == Some(TagType::String) {
            lua_tolstring(state, -1, null_mut())
        } else {
            c"error object is not a string".as_ptr()
        };
        eprintln!(
            "PANIC: unprotected error in call to Lua API ({})",
            std::ffi::CStr::from_ptr(message).to_string_lossy()
        );
        0
    }
}
pub unsafe fn checkcontrol(state: *mut State, mut message: *const i8, tocont: i32) -> i32 {
    unsafe {
        if tocont != 0 || {
            let current_char = message;
            message = message.add(1);
            *current_char as i32 != Character::At as i32
        } {
            0
        } else {
            if std::ffi::CStr::from_ptr(message) == c"off" {
                lua_setwarnf(
                    state,
                    Some(warnfoff as unsafe fn(*mut std::ffi::c_void, *const i8, i32) -> ()),
                    state as *mut std::ffi::c_void,
                );
            } else if std::ffi::CStr::from_ptr(message) == c"on" {
                lua_setwarnf(
                    state,
                    Some(warnfon as unsafe fn(*mut std::ffi::c_void, *const i8, i32) -> ()),
                    state as *mut std::ffi::c_void,
                );
            }
            1
        }
    }
}
pub unsafe fn warnfoff(arbitrary_data: *mut std::ffi::c_void, message: *const i8, tocont: i32) {
    unsafe {
        checkcontrol(arbitrary_data as *mut State, message, tocont);
    }
}
pub unsafe fn warnfcont(arbitrary_data: *mut std::ffi::c_void, message: *const i8, tocont: i32) {
    unsafe {
        let state: *mut State = arbitrary_data as *mut State;
        eprint!("{}", std::ffi::CStr::from_ptr(message).to_string_lossy());
        if tocont != 0 {
            lua_setwarnf(
                state,
                Some(warnfcont as unsafe fn(*mut std::ffi::c_void, *const i8, i32) -> ()),
                state as *mut std::ffi::c_void,
            );
        } else {
            eprintln!();
            lua_setwarnf(
                state,
                Some(warnfon as unsafe fn(*mut std::ffi::c_void, *const i8, i32) -> ()),
                state as *mut std::ffi::c_void,
            );
        };
    }
}
pub unsafe fn warnfon(arbitrary_data: *mut std::ffi::c_void, message: *const i8, tocont: i32) {
    unsafe {
        if checkcontrol(arbitrary_data as *mut State, message, tocont) == 0 {
            eprint!("Lua warning: ");
            warnfcont(arbitrary_data, message, tocont);
        }
    }
}
pub unsafe fn lual_newstate() -> (*mut Global, *mut State) {
    use std::alloc::{alloc_zeroed, dealloc, Layout};
    unsafe {
        let global_layout = Layout::new::<Global>();
        let global = alloc_zeroed(global_layout) as *mut Global;
        if !global.is_null() {
            let interp_layout = Layout::new::<State>();
            let mut state = alloc_zeroed(interp_layout) as *mut State;
            if state.is_null() {
                dealloc(global as *mut u8, global_layout);
            } else {
                (*global).initialize();
                (*state).initialize(&*global);
                (*global).global_count_total_bytes += size_of::<State>() as i64;
                (*global).global_count_total_bytes += size_of::<Global>() as i64;
                (*state).preinit_thread(global);
                (*global).global_allgc = &mut *(state as *mut Object);
                (*state).as_object_mut().object_next = null_mut();
                (*state).increment_noyield();
                (*global).global_maininterpreter = state;
                (*global).global_seed = luai_makeseed(state);
                if luad_rawrunprotected(
                    state,
                    Some(f_luaopen as unsafe fn(*mut State, *mut std::ffi::c_void) -> ()),
                    null_mut(),
                ) != Status::OK
                {
                    close_state(state);
                    state = null_mut();
                }
                if !state.is_null() {
                    lua_atpanic(state, Some(panic as unsafe fn(*mut State) -> i32));
                    lua_setwarnf(
                        state,
                        Some(warnfoff as unsafe fn(*mut std::ffi::c_void, *const i8, i32) -> ()),
                        state as *mut std::ffi::c_void,
                    );
                }
                return (global, state);
            }
        }
        (null_mut(), null_mut())
    }
}
pub unsafe fn luab_print(state: *mut State) -> i32 {
    unsafe {
        let n: i32 = (*state).get_top();
        let mut out = std::io::stdout().lock();
        for i in 1..(1 + n) {
            let mut l: usize = 0;
            let s: *const i8 = lual_tolstring(state, i, &mut l);
            if i > 1 {
                out.write_all(b"\t").ok();
            }
            out.write_all(std::slice::from_raw_parts(s as *const u8, l))
                .ok();
            lua_settop(state, -2);
        }
        out.write_all(b"\n").ok();
        out.flush().ok();
        0
    }
}
pub unsafe fn luab_warn(state: *mut State) -> i32 {
    unsafe {
        let n: i32 = (*state).get_top();
        lual_checklstring(state, 1, null_mut());
        for i in 2..(1 + n) {
            lual_checklstring(state, i, null_mut());
        }
        for i in 1..n {
            luae_warning(state, lua_tolstring(state, i, null_mut()), 1);
        }
        luae_warning(state, lua_tolstring(state, n, null_mut()), 0);
        0
    }
}
pub unsafe fn l_print(state: *mut State) {
    unsafe {
        let n: i32 = (*state).get_top();
        if n > 0 {
            lual_checkstack(state, LUA_MINSTACK, c"too many results to print".as_ptr());
            lua_getglobal(state, c"print".as_ptr());
            lua_rotate(state, 1, 1);
            if CallS::api_call(state, n, 0, 0, 0, None) != Status::OK {
                l_message(
                    PROGRAM_NAME.load(Ordering::Relaxed),
                    lua_pushfstring(
                        state,
                        c"error calling 'print' (%s)".as_ptr(),
                        &[lua_tolstring(state, -1, null_mut()).into()],
                    ),
                );
            }
        }
    }
}
use std::sync::atomic::{AtomicPtr, Ordering};
pub static GLOBAL_STATE: AtomicPtr<State> = AtomicPtr::new(null_mut());
pub static PROGRAM_NAME: AtomicPtr<i8> = AtomicPtr::new(c"lua".as_ptr() as *mut i8);
pub unsafe fn lstop(state: *mut State, mut _ar: *mut DebugInfo) {
    unsafe {
        lua_sethook(state, None, 0, 0);
        lual_error(state, c"interrupted!".as_ptr(), &[]);
    }
}
pub unsafe extern "C" fn laction(i: i32) {
    unsafe {
        let flag: i32 = HOOKMASK_CALL | HOOKMASK_RET | HOOKMASK_LINE | HOOKMASK_COUNT;
        SignalAction::setsignal(i, None);
        lua_sethook(
            GLOBAL_STATE.load(Ordering::Relaxed),
            Some(lstop as unsafe fn(*mut State, *mut DebugInfo) -> ()),
            flag,
            1,
        );
    }
}
pub unsafe fn print_version() {
    print!(
        "ferrigno {} — Lua 5.5 state\nlibraries: midi, tui, sqlite, json, toml, requests, urllib, sh\nembedded lua: {} files\n",
        env!("CARGO_PKG_VERSION"),
        crate::embedded_resources::list().len(),
    );
    std::io::stdout().flush().ok();
}
pub unsafe fn print_usage(badoption: *const i8) {
    unsafe {
        eprint!(
            "{}: ",
            std::ffi::CStr::from_ptr(PROGRAM_NAME.load(Ordering::Relaxed)).to_string_lossy()
        );
        if *badoption.add(1) as i32 == Character::LowerE as i32 || *badoption.add(1) as i32 == Character::LowerL as i32 {
            eprintln!(
                "'{}' needs argument",
                std::ffi::CStr::from_ptr(badoption).to_string_lossy()
            );
        } else {
            eprintln!(
                "unrecognized option '{}'",
                std::ffi::CStr::from_ptr(badoption).to_string_lossy()
            );
        }
        eprint!(
            "usage: {} [options] [script [args]]\nAvailable options are:\n  -e stat   execute string 'stat'\n  -i        enter interactive mode after executing 'script'\n  -l mod    require library 'mod' into global 'mod'\n  -l g=mod  require library 'mod' into global 'g'\n  -v        show version information\n  -E        ignore environment variables\n  -W        turn warnings on\n  --        stop handling options\n  --bare    standard Lua state (skip embedded app)\n  -         stop handling options and execute stdin\n",
            std::ffi::CStr::from_ptr(PROGRAM_NAME.load(Ordering::Relaxed)).to_string_lossy(),
        );
    }
}
pub unsafe fn l_message(pname: *const i8, message: *const i8) {
    unsafe {
        if !pname.is_null() {
            eprint!("{}: ", std::ffi::CStr::from_ptr(pname).to_string_lossy());
        }
        eprintln!("{}", std::ffi::CStr::from_ptr(message).to_string_lossy());
    }
}
pub unsafe fn report(state: *mut State, status: Status) -> Status {
    unsafe {
        if status != Status::OK {
            let mut message: *const i8 = lua_tolstring(state, -1, null_mut());
            if message.is_null() {
                message = c"(error message not a string)".as_ptr();
            }
            l_message(PROGRAM_NAME.load(Ordering::Relaxed), message);
            lua_settop(state, -2);
        }
        status
    }
}
pub unsafe fn msghandler(state: *mut State) -> i32 {
    unsafe {
        let mut message: *const i8 = lua_tolstring(state, 1, null_mut());
        if message.is_null() {
            if lual_callmeta(state, 1, c"__tostring".as_ptr()) && lua_type(state, -1) == Some(TagType::String) {
                return 1;
            } else {
                message = lua_pushfstring(
                    state,
                    c"(error object is a %s value)".as_ptr(),
                    &[lua_typename(state, lua_type(state, 1)).into()],
                );
            }
        }
        lual_traceback(state, state, message, 1);
        1
    }
}
pub unsafe fn docall(state: *mut State, narg: i32, nres: i32) -> Status {
    unsafe {
        let base: i32 = (*state).get_top() - narg;
        lua_pushcclosure(state, Some(msghandler as unsafe fn(*mut State) -> i32), 0);
        lua_rotate(state, base, 1);
        GLOBAL_STATE.store(state, Ordering::Relaxed);
        SignalAction::setsignal(2, Some(laction as unsafe extern "C" fn(i32) -> ()));
        let status = CallS::api_call(state, narg, nres, base, 0, None);
        SignalAction::setsignal(2, None);
        lua_rotate(state, base, -1);
        lua_settop(state, -2);
        status
    }
}
pub unsafe fn createargtable(state: *mut State, argv: *mut *mut i8, argc: i32, script: i32) {
    unsafe {
        (*state).lua_createtable();
        for i in 0..argc {
            lua_pushstring(state, *argv.add(i as usize));
            lua_rawseti(state, -2, (i - script) as i64);
        }
        lua_setglobal(state, c"arg".as_ptr());
    }
}
pub unsafe fn dochunk(state: *mut State, mut status: Status) -> Status {
    unsafe {
        if status == Status::OK {
            status = docall(state, 0, 0);
        }
        report(state, status)
    }
}
pub unsafe fn dofile(state: *mut State, name: *const i8) -> Status {
    unsafe { dochunk(state, lual_loadfilex(state, name, null())) }
}
pub unsafe fn dostring(state: *mut State, s: *const i8, name: *const i8) -> Status {
    unsafe { dochunk(state, lual_loadbufferx(state, s, cstr_len(s), name, null())) }
}
pub unsafe fn dolibrary(state: *mut State, globname: *mut i8) -> Status {
    unsafe {
        let mut suffix: *mut i8 = null_mut();
        let mut modname: *mut i8 = cstr_chr(globname, Character::Equal as i8) as *mut i8;
        if modname.is_null() {
            modname = globname;
            suffix = cstr_chr(modname, *(c"-".as_ptr())) as *mut i8;
        } else {
            *modname = Character::Null as i8;
            modname = modname.add(1);
        }
        lua_getglobal(state, c"require".as_ptr());
        lua_pushstring(state, modname);
        let status = docall(state, 1, 1);
        if status == Status::OK {
            if !suffix.is_null() {
                *suffix = Character::Null as i8;
            }
            lua_setglobal(state, globname);
        }
        report(state, status)
    }
}
pub unsafe fn pushargs(state: *mut State) -> i32 {
    unsafe {
        if lua_getglobal(state, c"arg".as_ptr()) != TagType::Table {
            lual_error(state, c"'arg' is not a table".as_ptr(), &[]);
        }
        let n: i32 = lual_len(state, -1) as i32;
        lual_checkstack(state, n + 3, c"too many arguments to script".as_ptr());
        for i in 1..(1 + n) {
            lua_rawgeti(state, -i, i as i64);
        }
        lua_rotate(state, -(1 + n), -1);
        lua_settop(state, -2);
        n
    }
}
pub unsafe fn handle_script(state: *mut State, argv: *mut *mut i8) -> Status {
    unsafe {
        let mut fname: *const i8 = *argv.add(0);
        if std::ffi::CStr::from_ptr(fname) == c"-" && std::ffi::CStr::from_ptr(*argv.sub(1)) != c"--" {
            fname = null();
        }
        let mut status = lual_loadfilex(state, fname, null());
        if status == Status::OK {
            let n: i32 = pushargs(state);
            status = docall(state, n, -1);
        }
        report(state, status)
    }
}
pub unsafe fn collectargs(argv: *mut *mut i8, first: *mut i32) -> i32 {
    unsafe {
        let mut args: i32 = 0;
        let mut i: i32;
        if !(*argv.add(0)).is_null() {
            if *(*argv.add(0)).add(0) != 0 {
                PROGRAM_NAME.store(*argv.add(0), Ordering::Relaxed);
            }
        } else {
            *first = -1;
            return 0;
        }
        i = 1;
        while !(*argv.add(i as usize)).is_null() {
            *first = i;
            if *(*argv.add(i as usize)).add(0) as i32 != Character::Hyphen as i32 {
                return args;
            }
            const ARG_NEXT: usize = 0;
            const ARG_VERSION_INFO: usize = 1;
            const ARG_EXECUTE_LIBRARY: usize = 2;
            let arg_action: usize;
            match Character::from_negative(*(*argv.add(i as usize)).add(1) as i32) {
                Some(Character::Hyphen) => {
                    if *(*argv.add(i as usize)).add(2) as i32 == Character::Null as i32 {
                        *first = if !(*argv.add((i + 1) as usize)).is_null() {
                            i + 1
                        } else {
                            0
                        };
                        return args;
                    }
                    let rest = std::ffi::CStr::from_ptr((*argv.add(i as usize)).add(2));
                    if rest == c"bare" {
                        args |= ARGS_BARE;
                        i += 1;
                        continue;
                    } else {
                        return 1;
                    }
                }
                Some(Character::Null) => {
                    return args;
                }
                Some(Character::UpperE) => {
                    if *(*argv.add(i as usize)).add(2) as i32 != Character::Null as i32 {
                        return 1;
                    }
                    args |= ARGS_NOENV;
                    arg_action = ARG_NEXT;
                }
                Some(Character::UpperW) => {
                    if *(*argv.add(i as usize)).add(2) as i32 != Character::Null as i32 {
                        return 1;
                    }
                    arg_action = ARG_NEXT;
                }
                Some(Character::LowerI) => {
                    args |= ARGS_INTERACTIVE;
                    arg_action = ARG_VERSION_INFO;
                }
                Some(Character::LowerV) => {
                    arg_action = ARG_VERSION_INFO;
                }
                Some(Character::LowerE) => {
                    args |= ARGS_EXECUTE;
                    arg_action = ARG_EXECUTE_LIBRARY;
                }
                Some(Character::LowerL) => {
                    arg_action = ARG_EXECUTE_LIBRARY;
                }
                _ => return 1,
            }
            match arg_action {
                ARG_VERSION_INFO => {
                    if *(*argv.add(i as usize)).add(2) as i32 != Character::Null as i32 {
                        return 1;
                    }
                    args |= ARGS_VERSION;
                }
                ARG_EXECUTE_LIBRARY => {
                    if *(*argv.add(i as usize)).add(2) as i32 == Character::Null as i32 {
                        i += 1;
                        if (*argv.add(i as usize)).is_null() || *(*argv.add(i as usize)).add(0) as i32 == Character::Hyphen as i32 {
                            return 1;
                        }
                    }
                }
                _ => {}
            }
            i += 1;
        }
        *first = 0;
        args
    }
}
pub unsafe fn runargs(state: *mut State, argv: *mut *mut i8, n: i32) -> i32 {
    unsafe {
        let mut i = 1i32;
        while i < n {
            let option: Character = Character::from(*(*argv.add(i as usize)).add(1) as i32);
            match option {
                Character::LowerE | Character::LowerL => {
                    let mut extra: *mut i8 = (*argv.add(i as usize)).add(2);
                    if *extra as i32 == Character::Null as i32 {
                        i += 1;
                        extra = *argv.add(i as usize);
                    }
                    let status = if option == Character::LowerE {
                        dostring(state, extra, c"=(command line)".as_ptr())
                    } else {
                        dolibrary(state, extra)
                    };
                    if status != Status::OK {
                        return 0;
                    }
                }
                Character::UpperW => {
                    luae_warning(state, c"@on".as_ptr(), 0);
                }
                _ => {}
            }
            i += 1;
        }
        1
    }
}
pub unsafe fn get_prompt(state: *mut State, firstline: i32) -> *const i8 {
    unsafe {
        if lua_getglobal(
            state,
            if firstline != 0 {
                c"_PROMPT".as_ptr()
            } else {
                c"_PROMPT2".as_ptr()
            },
        ) == TagType::Nil
        {
            if firstline != 0 {
                c"> ".as_ptr()
            } else {
                c">> ".as_ptr()
            }
        } else {
            let p: *const i8 = lual_tolstring(state, -1, null_mut());
            lua_rotate(state, -2, -1);
            lua_settop(state, -2);
            p
        }
    }
}
pub unsafe fn incomplete(state: *mut State, status: Status) -> i32 {
    unsafe {
        if status == Status::SyntaxError {
            let mut lmsg: usize = 0;
            let message: *const i8 = lua_tolstring(state, -1, &mut lmsg);
            if lmsg >= size_of::<[i8; 6]>() - 1
                && std::ffi::CStr::from_ptr(message.add(lmsg).sub(size_of::<[i8; 6]>() - 1)) == c"<eof>"
            {
                return 1;
            }
        }
        0
    }
}
const LUA_MAXINPUT: usize = 512;
pub unsafe fn pushline(state: *mut State, firstline: i32) -> bool {
    unsafe {
        let mut buffer: [i8; LUA_MAXINPUT] = [0; LUA_MAXINPUT];
        let b: *mut i8 = buffer.as_mut_ptr();
        let prmt: *const i8 = get_prompt(state, firstline);
        {
            let prmt_bytes = std::ffi::CStr::from_ptr(prmt).to_bytes();
            let mut out = std::io::stdout().lock();
            out.write_all(prmt_bytes).ok();
            out.flush().ok();
        }
        let mut line = String::new();
        let readstatus = std::io::BufRead::read_line(&mut std::io::stdin().lock(), &mut line).unwrap_or(0) > 0;
        lua_settop(state, -2);
        if !readstatus {
            false
        } else {
            let bytes = line.as_bytes();
            let copy_len = bytes.len().min(LUA_MAXINPUT - 1);
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), b as *mut u8, copy_len);
            *b.add(copy_len) = 0;
            let mut l: usize = copy_len;
            if l > 0 && *b.add(l - 1) as i32 == Character::LineFeed as i32 {
                l -= 1;
                *b.add(l) = Character::Null as i8;
            }
            if firstline != 0 && *b.add(0) as i32 == Character::Equal as i32 {
                lua_pushfstring(state, c"return %s".as_ptr(), &[b.add(1).into()]);
            } else {
                lua_pushlstring(state, b, l);
            }
            true
        }
    }
}
pub unsafe fn addreturn(state: *mut State) -> Status {
    unsafe {
        let line: *const i8 = lua_tolstring(state, -1, null_mut());
        let retline: *const i8 = lua_pushfstring(state, c"return %s;".as_ptr(), &[line.into()]);
        let status = lual_loadbufferx(
            state,
            retline,
            cstr_len(retline),
            c"=stdin".as_ptr(),
            null(),
        );
        if status == Status::OK {
            lua_rotate(state, -2, -1);
            lua_settop(state, -2);
        } else {
            lua_settop(state, -2 - 1);
        }
        status
    }
}
unsafe fn checklocal(line: *const i8) {
    unsafe {
        let space = c" \t".as_ptr();
        let p = line.add(cstr_span(line, space));
        if std::slice::from_raw_parts(p as *const u8, 5) == b"local" && !cstr_chr(space, *p.add(5)).is_null() {
            eprintln!("warning: locals do not survive across lines in interactive mode");
        }
    }
}
pub unsafe fn multiline(state: *mut State) -> Status {
    unsafe {
        let mut length: usize = 0;
        let line: *const i8 = lua_tolstring(state, 1, &mut length);
        checklocal(line);
        loop {
            let mut length: usize = 0;
            let line: *const i8 = lua_tolstring(state, 1, &mut length);
            let status = lual_loadbufferx(state, line, length, c"=stdin".as_ptr(), null());
            if incomplete(state, status) == 0 || !pushline(state, 0) {
                return status;
            }
            lua_rotate(state, -2, -1);
            lua_settop(state, -2);
            lua_pushstring(state, c"\n".as_ptr());
            lua_rotate(state, -2, 1);
            lua_concat(state, 3);
        }
    }
}
pub unsafe fn loadline(state: *mut State) -> Status {
    unsafe {
        lua_settop(state, 0);
        if !pushline(state, 1) {
            return Status::Closing;
        }
        let mut status = addreturn(state);
        if status != Status::OK {
            status = multiline(state);
        }
        lua_rotate(state, 1, -1);
        lua_settop(state, -2);
        status
    }
}
pub unsafe fn finishpcall(state: *mut State, status: Status, extra: i64) -> i32 {
    unsafe {
        match status {
            Status::OK | Status::Yield => (*state).get_top() - extra as i32,
            _ => {
                (*state).push_boolean(false);
                lua_pushvalue(state, -2);
                2
            }
        }
    }
}
pub unsafe fn luab_pcall(state: *mut State) -> i32 {
    unsafe {
        lual_checkany(state, 1);
        (*state).push_boolean(true);
        lua_rotate(state, 1, 1);
        let status = CallS::api_call(
            state,
            (*state).get_top() - 2,
            -1,
            0,
            0,
            Some(finishpcall as unsafe fn(*mut State, Status, i64) -> i32),
        );
        finishpcall(state, status, 0)
    }
}
pub unsafe fn checkstack(state: *mut State, other_state: *mut State, n: i32) {
    unsafe {
        if state != other_state && lua_checkstack(other_state, n) == 0 {
            lual_error(state, c"stack overflow".as_ptr(), &[]);
        }
    }
}
pub unsafe fn getthread(state: *mut State, arg: *mut i32) -> *mut State {
    unsafe {
        if lua_type(state, 1) == Some(TagType::State) {
            *arg = 1;
            lua_tothread(state, 1)
        } else {
            *arg = 0;
            state
        }
    }
}
pub unsafe fn settabss(state: *mut State, k: *const i8, v: *const i8) {
    unsafe {
        lua_pushstring(state, v);
        lua_setfield(state, -2, k);
    }
}
pub unsafe fn settabsi(state: *mut State, k: *const i8, v: i32) {
    unsafe {
        (*state).push_integer(v as i64);
        lua_setfield(state, -2, k);
    }
}
pub unsafe fn settabsb(state: *mut State, k: *const i8, v: i32) {
    unsafe {
        (*state).push_boolean(v != 0);
        lua_setfield(state, -2, k);
    }
}
pub unsafe fn treatstackoption(state: *mut State, other_state: *mut State, fname: *const i8) {
    unsafe {
        if state == other_state {
            lua_rotate(state, -2, 1);
        } else {
            lua_xmove(other_state, state, 1);
        }
        lua_setfield(state, -2, fname);
    }
}
pub unsafe fn auxupvalue(state: *mut State, get: i32) -> i32 {
    unsafe {
        let n: i32 = lual_checkinteger(state, 2) as i32;
        (*state).lual_checktype(1, TagType::Closure);
        let name: *const i8 = if get != 0 {
            lua_getupvalue(state, 1, n)
        } else {
            lua_setupvalue(state, 1, n)
        };
        if name.is_null() {
            0
        } else {
            lua_pushstring(state, name);
            lua_rotate(state, -(get + 1), 1);
            get + 1
        }
    }
}
pub unsafe fn checkupval(state: *mut State, argf: i32, argnup: i32, pnup: *mut i32) -> *mut std::ffi::c_void {
    unsafe {
        let count_upvalues: i32 = lual_checkinteger(state, argnup) as i32;
        (*state).lual_checktype(argf, TagType::Closure);
        let id: *mut std::ffi::c_void = lua_upvalueid(state, argf, count_upvalues);
        if !pnup.is_null() {
            if id.is_null() {
                lual_argerror(state, argnup, c"invalid upvalue index".as_ptr());
            }
            *pnup = count_upvalues;
        }
        id
    }
}
pub unsafe fn makemask(smask: *const i8, count: i32) -> i32 {
    unsafe {
        let mut mask: i32 = 0;
        if !(cstr_chr(smask, Character::LowerC as i8)).is_null() {
            mask |= HOOKMASK_CALL;
        }
        if !(cstr_chr(smask, Character::LowerR as i8)).is_null() {
            mask |= HOOKMASK_RET;
        }
        if !(cstr_chr(smask, Character::LowerL as i8)).is_null() {
            mask |= HOOKMASK_LINE;
        }
        if count > 0 {
            mask |= HOOKMASK_COUNT;
        }
        mask
    }
}
pub unsafe fn unmakemask(mask: i32, smask: *mut i8) -> *mut i8 {
    unsafe {
        let mut i: i32 = 0;
        if mask & HOOKMASK_CALL != 0 {
            let old = i;
            i += 1;
            *smask.add(old as usize) = Character::LowerC as i8;
        }
        if mask & HOOKMASK_RET != 0 {
            let old = i;
            i += 1;
            *smask.add(old as usize) = Character::LowerR as i8;
        }
        if mask & HOOKMASK_LINE != 0 {
            let old = i;
            i += 1;
            *smask.add(old as usize) = Character::LowerL as i8;
        }
        *smask.add(i as usize) = Character::Null as i8;
        smask
    }
}
pub unsafe fn fix_object_state(state: *mut State, object: *mut Object) {
    unsafe {
        let global: *mut Global = (*state).interpreter_global;
        Object::fix_object_global(global, object);
    }
}
