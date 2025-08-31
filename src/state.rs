use crate::callinfo::*;
use crate::functions::*;
use crate::debug::*;
use crate::c::*;
use crate::dynamicdata::*;
use crate::tm::*;
use crate::global::*;
use crate::longjump::*;
use crate::object::*;
use crate::onelua::*;
use crate::prototype::*;
use crate::cclosure::*;
use crate::zio::*;
use crate::buffer::*;
use crate::sparser::*;
use crate::closep::*;
use crate::new::*;
use crate::lclosure::*;
use crate::f2i::*;
use crate::value::*;
use crate::labeldescription::*;
use crate::registeredfunction::*;
use crate::labellist::*;
use crate::stackvalue::*;
use crate::variabledescription::*;
use crate::stkidrel::*;
use crate::libraries::*;
use crate::table::*;
use crate::tag::*;
use crate::user::*;
use crate::tstring::*;
use crate::tvalue::*;
use crate::upvalue::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct State {
    pub next: *mut Object,
    pub tag: u8,
    pub marked: u8,
    pub dummy0: u8 = 0,
    pub dummy1: u8 = 0,
    pub dummy2: u32 = 0,
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
    fn get_marked(&self) -> u8 {
        self.marked
    }
    fn set_marked(&mut self, marked: u8) {
        self.marked = marked;
    }
    fn set_tag(&mut self, tag: u8) {
        self.tag = tag;
    }
    fn set_collectable(&mut self) {
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
    fn get_class_name(&mut self) -> String {
        "state".to_string()
    }
    fn get_metatable(&mut self) -> *mut Table {
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
                    (*io).value.object = &mut (*(x_ as *mut Object));
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
            let io: *mut TValue = &mut (*self.top.p).value;
            (*io).value.object = &mut (*(self as *mut State as *mut Object));
            (*io).set_tag(TAG_VARIANT_STATE);
            (*io).set_collectable();
            self.top.p = self.top.p.offset(1);
            return (*self.global).mainthread == self;
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
                    ((*up).v.p as StkId as *mut i8).offset_from(self.stack.p as *mut i8) as i64;
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
            let io: *mut TValue = &mut (*self.top.p).value;
            (*io).value.object = &mut (*(message as *mut Object));
            (*io).set_tag((*message).get_tag());
            (*io).set_collectable();
            self.top.p = self.top.p.offset(1);
            luad_throw(self, 5);
        }
    }
    pub unsafe extern "C" fn luae_checkcstack(& mut self) {
        unsafe {
            if self.count_c_calls & 0xffff as i32 as u32 == 200 as i32 as u32 {
                luag_runerror(self, b"C stack overflow\0" as *const u8 as *const i8);
            } else if self.count_c_calls & 0xffff as i32 as u32
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
            if ((self.count_c_calls & 0xffff as i32 as u32 >= 200 as i32 as u32) as i32 != 0) as i32
                as i64
                != 0
            {
                self.luae_checkcstack();
            }
        }
    }
    pub unsafe extern "C" fn stackinuse(& mut self) -> i32 {
        unsafe {
            let mut lim: StkId = self.top.p;
            let mut call_info: *mut CallInfo = self.call_info;
            while !call_info.is_null() {
                if lim < (*call_info).top.p {
                    lim = (*call_info).top.p;
                }
                call_info = (*call_info).previous;
            }
            let mut res: i32 = lim.offset_from(self.stack.p) as i64 as i32 + 1;
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
                && (self.stack_last.p).offset_from(self.stack.p) as i64 as i32 > max
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
            let io: *mut TValue = &mut (*self.top.p).value;
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
    pub unsafe extern "C" fn lua_getmetatable(& mut self, object_index: i32) -> i32 {
        unsafe {
            let object: *const TValue = index2value(self, object_index);
            let metatable: *mut Table;
            match (*object).get_tag_type() {
                TAG_TYPE_TABLE => {
                    metatable = (*((*object).value.object as *mut Table)).metatable;
                }
                TAG_TYPE_USER => {
                    metatable = (*((*object).value.object as *mut User)).metatable;
                }
                _ => {
                    metatable = (*self.global).mt[(get_tag_type((*object).get_tag())) as usize];
                }
            }
            if metatable.is_null() {
                0
            } else {
                let io: *mut TValue = &mut (*self.top.p).value;
                (*io).value.object = &mut (*(metatable as *mut Object));
                (*io).set_tag(TAG_VARIANT_TABLE);
                (*io).set_collectable();
                self.top.p = self.top.p.offset(1);
                1
            }
        }
    }
    pub unsafe extern "C" fn lua_getiuservalue(& mut self, index: i32, n: i32) -> i32 {
        unsafe {
            let t: i32;
            let o: *mut TValue = index2value(self, index);
            if n <= 0 || n > (*((*o).value.object as *mut User)).nuvalue as i32 {
                (*self.top.p).value.set_tag(TAG_VARIANT_NIL_NIL);
                t = -1;
            } else {
                let io1: *mut TValue = &mut (*self.top.p).value;
                let io2: *const TValue = &mut (*((*((*o).value.object as *mut User)).uv)
                    .as_mut_ptr()
                    .offset((n - 1) as isize))
                .uv;
                (*io1).value = (*io2).value;
                (*io1).set_tag((*io2).get_tag());
                t = (get_tag_type((*self.top.p).value.get_tag())) as i32;
            }
            self.top.p = self.top.p.offset(1);
            return t;
        }
    }
}
pub unsafe extern "C" fn pmain(state: *mut State) -> i32 {
    unsafe {
        let argc: i32 = lua_tointegerx(state, 1, std::ptr::null_mut()) as i32;
        let argv: *mut *mut i8 = lua_touserdata(state, 2) as *mut *mut i8;
        let mut script: i32 = 0;
        let args: i32 = collectargs(argv, &mut script);
        let optlim: i32 = if script > 0 { script } else { argc };
        lual_checkversion_(
            state,
            504.0,
            (::core::mem::size_of::<i64>() as u64)
                .wrapping_mul(16 as i32 as u64)
                .wrapping_add(::core::mem::size_of::<f64>() as u64),
        );
        if args == 1 {
            print_usage(*argv.offset(script as isize));
            return 0;
        }
        if args & 16 as i32 != 0 {
            (*state).push_boolean(true);
            lua_setfield(
                state,
                -(1000000 as i32) - 1000 as i32,
                b"LUA_NOENV\0" as *const u8 as *const i8,
            );
        }
        lual_openlibs(state);
        createargtable(state, argv, argc, script);
        lua_gc(state, 1);
        lua_gc(state, 10 as i32, 0, 0);
        if args & 16 as i32 == 0 {
            if handle_luainit(state) != 0 {
                return 0;
            }
        }
        if runargs(state, argv, optlim) == 0 {
            return 0;
        }
        if script > 0 {
            if handle_script(state, argv.offset(script as isize)) != 0 {
                return 0;
            }
        }
        if args & 2 != 0 {
            do_repl(state);
        } else if script < 1 && args & (8 | 4) == 0 {
            if isatty(0) != 0 {
                do_repl(state);
            } else {
                dofile(state, std::ptr::null());
            }
        }
        (*state).push_boolean(true);
        return 1;
    }
}
pub unsafe fn main_0(argc: i32, argv: *mut *mut i8) -> i32 {
    unsafe {
        let state: *mut State = lual_newstate();
        if state.is_null() {
            l_message(
                *argv.offset(0),
                b"cannot create state: not enough memory\0" as *const u8 as *const i8,
            );
            return 1;
        } else {
            lua_gc(state, 0);
            lua_pushcclosure(
                state,
                Some(pmain as unsafe extern "C" fn(*mut State) -> i32),
                0,
            );
            (*state).push_integer(argc as i64);
            lua_pushlightuserdata(state, argv as *mut libc::c_void);
            let status: i32 = lua_pcallk(state, 2, 1, 0, 0, None);
            let result: i32 = lua_toboolean(state, -1);
            report(state, status);
            lua_close(state);
            return if result != 0 && status == 0 { 0 } else { 1 };
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
pub unsafe extern "C" fn lual_openlibs(state: *mut State) {
    unsafe {
        let mut lib: *const RegisteredFunction = LOADED_FUNCTIONS.as_ptr();
        while ((*lib).function).is_some() {
            lual_requiref(state, (*lib).name, (*lib).function, 1);
            lua_settop(state, -1 - 1);
            lib = lib.offset(1);
        }
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
            if !((*(*g).mainthread).long_jump).is_null() {
                let fresh0 = (*(*g).mainthread).top.p;
                (*(*g).mainthread).top.p = ((*(*g).mainthread).top.p).offset(1);
                let io1: *mut TValue = &mut (*fresh0).value;
                let io2: *const TValue = &mut (*(*state).top.p.offset(-(1 as isize))).value;
                (*io1).value = (*io2).value;
                (*io1).set_tag((*io2).get_tag());
                luad_throw((*g).mainthread, error_code);
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
        let old_size: i32 = ((*state).stack_last.p).offset_from((*state).stack.p) as i64 as i32;
        let oldgcstop: i32 = (*(*state).global).gcstopem as i32;
        (*state).relstack();
        (*(*state).global).gcstopem = 1;
        let newstack: StkId = luam_realloc_(
            state,
            (*state).stack.p as *mut libc::c_void,
            ((old_size + 5) as u64).wrapping_mul(::core::mem::size_of::<StackValue>() as u64),
            ((new_size + 5) as u64).wrapping_mul(::core::mem::size_of::<StackValue>() as u64),
        ) as *mut StackValue;
        (*(*state).global).gcstopem = oldgcstop as u8;
        if ((newstack == std::ptr::null_mut() as StkId) as i32 != 0) as i32 as i64 != 0 {
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
                .value
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
        let size: i32 = ((*state).stack_last.p).offset_from((*state).stack.p) as i64 as i32;
        if size > 1000000 {
            if should_raise_error {
                (*state).luad_errerr();
            }
            return 0;
        } else if n < 1000000 {
            let mut new_size: i32 = 2 * size;
            let needed: i32 = ((*state).top.p).offset_from((*state).stack.p) as i64 as i32 + n;
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
            let mut ar: Debug = Debug {
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
            if ((((*state).stack_last.p).offset_from((*state).top.p) as i64 <= 20 as i32 as i64)
                as i32
                != 0) as i32 as i64
                != 0
            {
                luad_growstack(state, 20 as i32, true);
            }
            if (*call_info).top.p < (*state).top.p.offset(20 as i32 as isize) {
                (*call_info).top.p = (*state).top.p.offset(20 as i32 as isize);
            }
            (*state).allow_hook = 0;
            (*call_info).call_status = ((*call_info).call_status as i32 | mask) as u16;
            (Some(hook.expect("non-null function pointer"))).expect("non-null function pointer")(
                state, &mut ar,
            );
            (*state).allow_hook = 1;
            (*call_info).top.p = ((*state).stack.p as *mut i8).offset(ci_top as isize) as StkId;
            (*state).top.p = ((*state).stack.p as *mut i8).offset(top as isize) as StkId;
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
            let p: *mut Prototype = (*((*(*call_info).function.p).value.value.object
                as *mut LClosure)).p;
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
            let firstres: StkId = (*state).top.p.offset(-(nres as isize));
            let mut delta: i32 = 0;
            if (*call_info).call_status as i32 & 1 << 1 == 0 {
                let p: *mut Prototype = (*((*(*call_info).function.p).value.value.object
                    as *mut LClosure)).p;
                if (*p).is_variable_arguments {
                    delta =
                        (*call_info).u.l.count_extra_arguments + (*p).count_parameters as i32 + 1;
                }
            }
            (*call_info).function.p = ((*call_info).function.p).offset(delta as isize);
            let ftransfer: i32 = firstres.offset_from((*call_info).function.p) as i64 as u16 as i32;
            luad_hook(state, 1, -1, ftransfer, nres);
            (*call_info).function.p = ((*call_info).function.p).offset(-(delta as isize));
        }
        call_info = (*call_info).previous;
        if (*call_info).call_status as i32 & 1 << 1 == 0 {
            (*state).old_program_counter = ((*call_info).u.l.saved_program_counter).offset_from(
                (*(*((*(*call_info).function.p).value.value.object as *mut LClosure))
                    .p)
                    .code,
            ) as i64 as i32
                - 1;
        }
    }
}
pub unsafe extern "C" fn tryfunctm(state: *mut State, mut function: StkId) -> StkId {
    unsafe {
        let mut p: StkId;
        if ((((*state).stack_last.p).offset_from((*state).top.p) as i64 <= 1) as i32 != 0) as i32
            as i64
            != 0
        {
            let t__: i64 = (function as *mut i8).offset_from((*state).stack.p as *mut i8) as i64;
            if (*(*state).global).gc_debt > 0 {
                luac_step(state);
            }
            luad_growstack(state, 1, true);
            function = ((*state).stack.p as *mut i8).offset(t__ as isize) as StkId;
        }
        let tm: *const TValue = luat_gettmbyobj(state, &mut (*function).value, TM_CALL);
        if (*tm).get_tag_type() == TAG_TYPE_NIL {
            luag_callerror(state, &mut (*function).value);
        }
        p = (*state).top.p;
        while p > function {
            let io1: *mut TValue = &mut (*p).value;
            let io2: *const TValue = &mut (*p.offset(-(1 as isize))).value;
            (*io1).value = (*io2).value;
            (*io1).set_tag((*io2).get_tag());
            p = p.offset(-1);
        }
        (*state).top.p = (*state).top.p.offset(1);
        let io1_0: *mut TValue = &mut (*function).value;
        let io2_0: *const TValue = tm;
        (*io1_0).value = (*io2_0).value;
        (*io1_0).set_tag((*io2_0).get_tag());
        return function;
    }
}
#[inline]
pub unsafe extern "C" fn moveresults(
    state: *mut State,
    mut res: StkId,
    mut nres: i32,
    mut wanted: i32,
) {
    unsafe {
        let firstresult: StkId;
        let mut i: i32;
        match wanted {
            0 => {
                (*state).top.p = res;
                return;
            }
            1 => {
                if nres == 0 {
                    (*res).value.set_tag(TAG_VARIANT_NIL_NIL);
                } else {
                    let io1: *mut TValue = &mut (*res).value;
                    let io2: *const TValue = &mut (*(*state).top.p.offset(-(nres as isize))).value;
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
                        res = ((*state).stack.p as *mut i8).offset(savedres as isize) as StkId;
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
            let io1_0: *mut TValue = &mut (*res.offset(i as isize)).value;
            let io2_0: *const TValue = &mut (*firstresult.offset(i as isize)).value;
            (*io1_0).value = (*io2_0).value;
            (*io1_0).set_tag((*io2_0).get_tag());
            i += 1;
        }
        while i < wanted {
            (*res.offset(i as isize)).value.set_tag(TAG_VARIANT_NIL_NIL);
            i += 1;
        }
        (*state).top.p = res.offset(wanted as isize);
    }
}
pub unsafe extern "C" fn luad_poscall(state: *mut State, call_info: *mut CallInfo, nres: i32) {
    unsafe {
        let wanted: i32 = (*call_info).count_results as i32;
        if (((*state).hook_mask != 0 && !(wanted < -1)) as i32 != 0) as i32 as i64 != 0 {
            rethook(state, call_info, nres);
        }
        moveresults(state, (*call_info).function.p, nres, wanted);
        (*state).call_info = (*call_info).previous;
    }
}
#[inline]
pub unsafe extern "C" fn prepcallinfo(
    state: *mut State,
    function: StkId,
    nret: i32,
    mask: i32,
    top: StkId,
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
#[inline]
pub unsafe extern "C" fn precallc(
    state: *mut State,
    mut function: StkId,
    count_results: i32,
    f: CFunction,
) -> i32 {
    unsafe {
        if ((((*state).stack_last.p).offset_from((*state).top.p) as i64 <= 20 as i32 as i64) as i32
            != 0) as i32 as i64
            != 0
        {
            let t__: i64 = (function as *mut i8).offset_from((*state).stack.p as *mut i8) as i64;
            if (*(*state).global).gc_debt > 0 {
                luac_step(state);
            }
            luad_growstack(state, 20 as i32, true);
            function = ((*state).stack.p as *mut i8).offset(t__ as isize) as StkId;
        }
        let call_info = prepcallinfo(
            state,
            function,
            count_results,
            1 << 1,
            (*state).top.p.offset(20 as i32 as isize),
        );
        (*state).call_info = call_info;
        if ((*state).hook_mask & 1 << 0 != 0) as i32 as i64 != 0 {
            let narg: i32 = ((*state).top.p).offset_from(function) as i64 as i32 - 1;
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
    mut function: StkId,
    mut narg1: i32,
    delta: i32,
) -> i32 {
    unsafe {
        loop {
            match (*function).value.get_tag_variant() {
                TAG_VARIANT_CLOSURE_C => {
                    return precallc(
                        state,
                        function,
                        -1,
                        (*((*function).value.value.object as *mut CClosure)).f,
                    );
                }
                TAG_VARIANT_CLOSURE_CFUNCTION => {
                    return precallc(state, function, -1, (*function).value.value.f)
                }
                TAG_VARIANT_CLOSURE_L => {
                    let p: *mut Prototype =
                        (*((*function).value.value.object as *mut LClosure)).p;
                    let fsize: i32 = (*p).maximum_stack_size as i32;
                    let nfixparams: i32 = (*p).count_parameters as i32;
                    let mut i: i32;
                    if ((((*state).stack_last.p).offset_from((*state).top.p) as i64
                        <= (fsize - delta) as i64) as i32
                        != 0) as i32 as i64
                        != 0
                    {
                        let t__: i64 =
                            (function as *mut i8).offset_from((*state).stack.p as *mut i8) as i64;
                        if (*(*state).global).gc_debt > 0 {
                            luac_step(state);
                        }
                        luad_growstack(state, fsize - delta, true);
                        function = ((*state).stack.p as *mut i8).offset(t__ as isize) as StkId;
                    }
                    (*call_info).function.p = ((*call_info).function.p).offset(-(delta as isize));
                    i = 0;
                    while i < narg1 {
                        let io1: *mut TValue =
                            &mut (*((*call_info).function.p).offset(i as isize)).value;
                        let io2: *const TValue = &mut (*function.offset(i as isize)).value;
                        (*io1).value = (*io2).value;
                        (*io1).set_tag((*io2).get_tag());
                        i += 1;
                    }
                    function = (*call_info).function.p;
                    while narg1 <= nfixparams {
                        (*function.offset(narg1 as isize))
                            .value
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
    mut function: StkId,
    count_results: i32,
) -> *mut CallInfo {
    unsafe {
        loop {
            match (*function).value.get_tag_variant() {
                TAG_VARIANT_CLOSURE_C => {
                    precallc(
                        state,
                        function,
                        count_results,
                        (*((*function).value.value.object as *mut CClosure)).f,
                    );
                    return std::ptr::null_mut();
                }
                TAG_VARIANT_CLOSURE_CFUNCTION => {
                    precallc(state, function, count_results, (*function).value.value.f);
                    return std::ptr::null_mut();
                }
                TAG_VARIANT_CLOSURE_L => {
                    let call_info;
                    let p: *mut Prototype =
                        (*((*function).value.value.object as *mut LClosure)).p;
                    let mut narg: i32 = ((*state).top.p).offset_from(function) as i64 as i32 - 1;
                    let nfixparams: i32 = (*p).count_parameters as i32;
                    let fsize: i32 = (*p).maximum_stack_size as i32;
                    if ((((*state).stack_last.p).offset_from((*state).top.p) as i64 <= fsize as i64)
                        as i32
                        != 0) as i32 as i64
                        != 0
                    {
                        let t__: i64 =
                            (function as *mut i8).offset_from((*state).stack.p as *mut i8) as i64;
                        if (*(*state).global).gc_debt > 0 {
                            luac_step(state);
                        }
                        luad_growstack(state, fsize, true);
                        function = ((*state).stack.p as *mut i8).offset(t__ as isize) as StkId;
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
                        (*fresh1).value.set_tag(TAG_VARIANT_NIL_NIL);
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
#[inline]
pub unsafe extern "C" fn ccall(
    state: *mut State,
    mut function: StkId,
    count_results: i32,
    inc: u32,
) {
    unsafe {
        let call_info;
        (*state).count_c_calls = ((*state).count_c_calls as u32).wrapping_add(inc) as u32 as u32;
        if (((*state).count_c_calls & 0xffff as i32 as u32 >= 200 as i32 as u32) as i32 != 0) as i32
            as i64
            != 0
        {
            if ((((*state).stack_last.p).offset_from((*state).top.p) as i64 <= 0) as i32 != 0)
                as i32 as i64
                != 0
            {
                let t__: i64 =
                    (function as *mut i8).offset_from((*state).stack.p as *mut i8) as i64;
                luad_growstack(state, 0, true);
                function = ((*state).stack.p as *mut i8).offset(t__ as isize) as StkId;
            }
            (*state).luae_checkcstack();
        }
        call_info = luad_precall(state, function, count_results);
        if !call_info.is_null() {
            (*call_info).call_status = (1 << 2) as u16;
            luav_execute(state, call_info);
        }
        (*state).count_c_calls = ((*state).count_c_calls as u32).wrapping_sub(inc) as u32 as u32;
    }
}
pub unsafe extern "C" fn luad_callnoyield(state: *mut State, function: StkId, count_results: i32) {
    unsafe {
        ccall(state, function, count_results, (0x10000 as i32 | 1) as u32);
    }
}
pub unsafe extern "C" fn finishpcallk(state: *mut State, call_info: *mut CallInfo) -> i32 {
    unsafe {
        let mut status: i32 = (*call_info).call_status as i32 >> 10 as i32 & 7;
        if ((status == 0) as i32 != 0) as i32 as i64 != 0 {
            status = 1;
        } else {
            let mut function: StkId =
                ((*state).stack.p as *mut i8).offset((*call_info).u2.funcidx as isize) as StkId;
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
            n = (Some(((*call_info).u.c.k).expect("non-null function pointer")))
                .expect("non-null function pointer")(
                state, status, (*call_info).u.c.ctx
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
        let io: *mut TValue = &mut (*(*state).top.p).value;
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
        let first_argument: StkId = (*state).top.p.offset(-(n as isize));
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
                if ((*call_info).u.c.k).is_some() {
                    n = (Some(((*call_info).u.c.k).expect("non-null function pointer")))
                        .expect("non-null function pointer")(
                        state, 1, (*call_info).u.c.ctx
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
            (*from).count_c_calls & 0xffff as i32 as u32
        } else {
            0
        };
        if (*state).count_c_calls & 0xffff as i32 as u32 >= 200 as i32 as u32 {
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
        if !((!(status > 1) as i32 != 0) as i32 as i64 != 0) {
            (*state).status = status as u8;
            (*state).set_error_object(status, (*state).top.p);
            (*(*state).call_info).top.p = (*state).top.p;
        }
        *count_results = if status == 1 {
            (*(*state).call_info).u2.nyield
        } else {
            ((*state).top.p).offset_from(((*(*state).call_info).function.p).offset(1 as isize))
                as i64 as i32
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
        if (!((*state).count_c_calls & 0xffff0000 as u32 == 0) as i32 != 0) as i32 as i64 != 0 {
            if state != (*(*state).global).mainthread {
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
            (*call_info).u.c.k = k;
            if ((*call_info).u.c.k).is_some() {
                (*call_info).u.c.ctx = ctx;
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
            closep.level = ((*state).stack.p as *mut i8).offset(level as isize) as StkId;
            closep.status = status;
            status = luad_rawrunprotected(
                state,
                Some(closepaux as unsafe extern "C" fn(*mut State, *mut libc::c_void) -> ()),
                &mut closep as *mut CloseP as *mut libc::c_void,
            );
            if ((status == 0) as i32 != 0) as i32 as i64 != 0 {
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
        if ((status != 0) as i32 != 0) as i32 as i64 != 0 {
            (*state).call_info = old_call_info;
            (*state).allow_hook = old_allowhooks;
            status = luad_closeprotected(state, old_top, status);
            (*state).set_error_object(
                status,
                ((*state).stack.p as *mut i8).offset(old_top as isize) as StkId,
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
        let cl: *mut LClosure;
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
                    n: 0,
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
            ((*state).count_c_calls as u32).wrapping_add(0x10000 as i32 as u32) as u32 as u32;
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
            ((*state).count_c_calls as u32).wrapping_sub(0x10000 as i32 as u32) as u32 as u32;
        return status;
    }
}
pub unsafe extern "C" fn index2value(state: *mut State, mut index: i32) -> *mut TValue {
    unsafe {
        let call_info: *mut CallInfo = (*state).call_info;
        if index > 0 {
            let o: StkId = ((*call_info).function.p).offset(index as isize);
            if o >= (*state).top.p {
                return &mut (*(*state).global).nilvalue;
            } else {
                return &mut (*o).value;
            }
        } else if !(index <= -(1000000 as i32) - 1000 as i32) {
            return &mut (*(*state).top.p.offset(index as isize)).value;
        } else if index == -(1000000 as i32) - 1000 as i32 {
            return &mut (*(*state).global).l_registry;
        } else {
            index = -(1000000 as i32) - 1000 as i32 - index;
            let value = (*(*call_info).function.p).value;
            if value.is_collectable() && value.get_tag_variant() == TAG_VARIANT_CLOSURE_C {
                let function: *mut CClosure = &mut (*(value.value.object as *mut CClosure));
                return if index <= (*function).count_upvalues as i32 {
                    &mut *((*function).upvalue)
                        .as_mut_ptr()
                        .offset((index - 1) as isize) as *mut TValue
                } else {
                    &mut (*(*state).global).nilvalue
                };
            } else {
                return &mut (*(*state).global).nilvalue;
            }
        };
    }
}
#[inline]
pub unsafe extern "C" fn index2stack(state: *mut State, index: i32) -> StkId {
    unsafe {
        let call_info: *mut CallInfo = (*state).call_info;
        if index > 0 {
            let o: StkId = ((*call_info).function.p).offset(index as isize);
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
            let io1: *mut TValue = &mut (*(*to).top.p).value;
            let io2: *const TValue = &mut (*((*from).top.p).offset(i as isize)).value;
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
            ((*state).top.p).offset_from((*(*state).call_info).function.p) as i64 as i32 + index
        };
    }
}
pub unsafe extern "C" fn lua_settop(state: *mut State, index: i32) {
    unsafe {
        let call_info;
        let mut newtop;
        let mut diff;
        call_info = (*state).call_info;
        let function: StkId = (*call_info).function.p;
        if index >= 0 {
            diff = function
                .offset(1 as isize)
                .offset(index as isize)
                .offset_from((*state).top.p) as i64;
            while diff > 0 {
                let fresh4 = (*state).top.p;
                (*state).top.p = (*state).top.p.offset(1);
                (*fresh4).value.set_tag(TAG_VARIANT_NIL_NIL);
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
        (*level).value.set_tag(TAG_VARIANT_NIL_NIL);
    }
}
#[inline]
pub unsafe extern "C" fn reverse(mut _state: *mut State, mut from: StkId, mut to: StkId) {
    unsafe {
        while from < to {
            let mut temp: TValue = TValue {
                value: Value {
                    object: std::ptr::null_mut(),
                },
                tag: 0,
            };
            let io1: *mut TValue = &mut temp;
            let io2: *const TValue = &mut (*from).value;
            (*io1).value = (*io2).value;
            (*io1).set_tag((*io2).get_tag());
            let io1_0: *mut TValue = &mut (*from).value;
            let io2_0: *const TValue = &mut (*to).value;
            (*io1_0).value = (*io2_0).value;
            (*io1_0).set_tag((*io2_0).get_tag());
            let io1_1: *mut TValue = &mut (*to).value;
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
        let high: StkId = (*state).top.p.offset(-(1 as isize));
        let low: StkId = index2stack(state, index);
        let middle: StkId = if n >= 0 {
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
        let fr: *mut TValue = index2value(state, fromidx);
        let to: *mut TValue = index2value(state, toidx);
        let io1: *mut TValue = to;
        let io2: *const TValue = fr;
        (*io1).value = (*io2).value;
        (*io1).set_tag((*io2).get_tag());
        if toidx < -(1000000 as i32) - 1000 as i32 {
            if (*fr).is_collectable() {
                if (*((*(*(*state).call_info).function.p).value.value.object as *mut CClosure))
                    .get_marked()
                    & 1 << 5
                    != 0
                    && (*(*fr).value.object).get_marked() & (1 << 3 | 1 << 4) != 0
                {
                    luac_barrier_(
                        state,
                        &mut (*(&mut (*((*(*(*state).call_info).function.p).value.value.object)))),
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
        let io1: *mut TValue = &mut (*(*state).top.p).value;
        let io2: *const TValue = index2value(state, index);
        (*io1).value = (*io2).value;
        (*io1).set_tag((*io2).get_tag());
        (*state).top.p = (*state).top.p.offset(1);
    }
}
pub unsafe extern "C" fn lua_type(state: *mut State, index: i32) -> Option<u8> {
    unsafe {
        let o: *const TValue = index2value(state, index);
        return if (get_tag_type((*o).get_tag()) != TAG_TYPE_NIL)
            || o != &mut (*(*state).global).nilvalue as *mut TValue as *const TValue
        {
            return Some((*o).get_tag_type())
        } else {
            None
        };
    }
}
pub unsafe extern "C" fn lua_typename(mut _state: *mut State, t: Option<u8>) -> *const i8 {
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
        let o: *const TValue = index2value(state, index);
        match (*o).get_tag_variant() {
            TAG_VARIANT_CLOSURE_CFUNCTION => return true,
            TAG_VARIANT_CLOSURE_C => return true,
            _ => return false,
        }
    }
}
pub unsafe extern "C" fn lua_isinteger(state: *mut State, index: i32) -> bool {
    unsafe {
        return (*index2value(state, index)).get_tag() == TAG_VARIANT_NUMERIC_INTEGER;
    }
}
pub unsafe extern "C" fn lua_isnumber(state: *mut State, index: i32) -> bool {
    unsafe {
        let o: *const TValue = index2value(state, index);
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
        let o: *const TValue = index2value(state, index);
        return match get_tag_type((*o).get_tag()) {
            TAG_TYPE_NUMERIC => true,
            TAG_TYPE_STRING => true,
            _ => false,
        };
    }
}
pub unsafe extern "C" fn lua_rawequal(state: *mut State, index1: i32, index2: i32) -> bool {
    unsafe {
        let o1: *const TValue = index2value(state, index1);
        let o2: *const TValue = index2value(state, index2);
        return if (!(get_tag_type((*o1).get_tag()) == TAG_TYPE_NIL)
            || o1 != &mut (*(*state).global).nilvalue as *mut TValue as *const TValue)
            && (!(get_tag_type((*o2).get_tag()) == TAG_TYPE_NIL)
                || o2 != &mut (*(*state).global).nilvalue as *mut TValue as *const TValue)
        {
            0 != luav_equalobj(std::ptr::null_mut(), o1, o2)
        } else {
            false
        };
    }
}
pub unsafe extern "C" fn lua_arith(state: *mut State, op: i32) {
    unsafe {
        if !(op != 12 as i32 && op != 13 as i32) {
            let io1: *mut TValue = &mut (*(*state).top.p).value;
            let io2: *const TValue = &mut (*(*state).top.p.offset(-(1 as isize))).value;
            (*io1).value = (*io2).value;
            (*io1).set_tag((*io2).get_tag());
            (*state).top.p = (*state).top.p.offset(1);
        }
        luao_arith(
            state,
            op,
            &mut (*(*state).top.p.offset(-(2 as isize))).value,
            &mut (*(*state).top.p.offset(-(1 as isize))).value,
            (*state).top.p.offset(-(2 as isize)),
        );
        (*state).top.p = (*state).top.p.offset(-1);
    }
}
pub unsafe extern "C" fn lua_compare(state: *mut State, index1: i32, index2: i32, op: i32) -> i32 {
    unsafe {
        let o1: *const TValue = index2value(state, index1);
        let o2: *const TValue = index2value(state, index2);
        let mut i: i32 = 0;
        if (!(get_tag_type((*o1).get_tag()) == TAG_TYPE_NIL)
            || o1 != &mut (*(*state).global).nilvalue as *mut TValue as *const TValue)
            && (!(get_tag_type((*o2).get_tag()) == TAG_TYPE_NIL)
                || o2 != &mut (*(*state).global).nilvalue as *mut TValue as *const TValue)
        {
            match op {
                0 => {
                    i = luav_equalobj(state, o1, o2);
                }
                1 => {
                    i = luav_lessthan(state, o1, o2);
                }
                2 => {
                    i = luav_lessequal(state, o1, o2);
                }
                _ => {}
            }
        }
        return i;
    }
}
pub unsafe extern "C" fn lua_stringtonumber(state: *mut State, s: *const i8) -> u64 {
    unsafe {
        let size: u64 = luao_str2num(s, &mut (*(*state).top.p).value);
        if size != 0u64 {
            (*state).top.p = (*state).top.p.offset(1);
        }
        return size;
    }
}
pub unsafe extern "C" fn lua_tonumberx(state: *mut State, index: i32, is_number: *mut bool) -> f64 {
    unsafe {
        let mut n: f64 = 0.0;
        let o: *const TValue = index2value(state, index);
        let is_number_: bool = if (*o).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
            n = (*o).value.n;
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
        let o: *const TValue = index2value(state, index);
        let is_number_: bool =
            if (((*o).get_tag() == TAG_VARIANT_NUMERIC_INTEGER) as i32 != 0) as i32 as i64 != 0 {
                res = (*o).value.i;
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
        let o: *const TValue = index2value(state, index);
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
        let mut o: *mut TValue = index2value(state, index);
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
            o = index2value(state, index);
        }
        if !length.is_null() {
            *length = (*((*o).value.object as *mut TString)).get_length();
        }
        return (*((*o).value.object as *mut TString)).get_contents();
    }
}
pub unsafe extern "C" fn lua_rawlen(state: *mut State, index: i32) -> u64 {
    unsafe {
        let o: *const TValue = index2value(state, index);
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
        let o: *const TValue = index2value(state, index);
        return User::touserdata(o);
    }
}
pub unsafe extern "C" fn lua_tothread(state: *mut State, index: i32) -> *mut State {
    unsafe {
        let o: *const TValue = index2value(state, index);
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
        let io: *mut TValue = &mut (*(*state).top.p).value;
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
            (*(*state).top.p).value.set_tag(TAG_VARIANT_NIL_NIL);
        } else {
            let ts: *mut TString = luas_new(state, s);
            let io: *mut TValue = &mut (*(*state).top.p).value;
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
            let io: *mut TValue = &mut (*(*state).top.p).value;
            (*io).value.f = fn_0;
            (*io).set_tag(TAG_VARIANT_CLOSURE_CFUNCTION);
            (*state).top.p = (*state).top.p.offset(1);
        } else {
            let cl: *mut CClosure = luaf_newcclosure(state, n);
            (*cl).f = fn_0;
            (*state).top.p = (*state).top.p.offset(-(n as isize));
            loop {
                let fresh5 = n;
                n = n - 1;
                if !(fresh5 != 0) {
                    break;
                }
                let io1: *mut TValue =
                    &mut *((*cl).upvalue).as_mut_ptr().offset(n as isize) as *mut TValue;
                let io2: *const TValue = &mut (*(*state).top.p.offset(n as isize)).value;
                (*io1).value = (*io2).value;
                (*io1).set_tag((*io2).get_tag());
            }
            let io_0: *mut TValue = &mut (*(*state).top.p).value;
            let x_: *mut CClosure = cl;
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
        let io: *mut TValue = &mut (*(*state).top.p).value;
        (*io).value.p = p;
        (*io).set_tag(TAG_TYPE_POINTER);
        (*state).top.p = (*state).top.p.offset(1);
    }
}
#[inline]
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
            let io1: *mut TValue = &mut (*(*state).top.p).value;
            let io2: *const TValue = slot;
            (*io1).value = (*io2).value;
            (*io1).set_tag((*io2).get_tag());
            (*state).top.p = (*state).top.p.offset(1);
        } else {
            let io: *mut TValue = &mut (*(*state).top.p).value;
            let x_: *mut TString = str;
            (*io).value.object = &mut (*(x_ as *mut Object));
            (*io).set_tag((*x_).get_tag());
            (*io).set_collectable();
            (*state).top.p = (*state).top.p.offset(1);
            luav_finishget(
                state,
                t,
                &mut (*(*state).top.p.offset(-(1 as isize))).value,
                (*state).top.p.offset(-(1 as isize)),
                slot,
            );
        }
        return (get_tag_type((*(*state).top.p.offset(-(1 as isize))).value.get_tag())) as i32;
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
        let t: *mut TValue = index2value(state, index);
        if if (*t).get_tag_variant() != TAG_VARIANT_TABLE {
            slot = std::ptr::null();
            0
        } else {
            slot = luah_get(
                &mut (*((*t).value.object as *mut Table)),
                &mut (*(*state).top.p.offset(-(1 as isize))).value,
            );
            (get_tag_type((*slot).get_tag()) != TAG_TYPE_NIL) as i32
        } != 0
        {
            let io1: *mut TValue = &mut (*(*state).top.p.offset(-(1 as isize))).value;
            let io2: *const TValue = slot;
            (*io1).value = (*io2).value;
            (*io1).set_tag((*io2).get_tag());
        } else {
            luav_finishget(
                state,
                t,
                &mut (*(*state).top.p.offset(-(1 as isize))).value,
                (*state).top.p.offset(-(1 as isize)),
                slot,
            );
        }
        return (get_tag_type((*(*state).top.p.offset(-(1 as isize))).value.get_tag())) as i32;
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
