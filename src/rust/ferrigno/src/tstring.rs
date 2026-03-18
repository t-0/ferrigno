use crate::character::*;
use crate::dumpstate::*;
use crate::functions::*;
use crate::global::*;
use crate::lexicalstate::FIRST_RESERVED;
use crate::object::*;
use crate::state::*;
use crate::stringtable::*;
use crate::tagvariant::*;
use crate::tobject::*;
use crate::token::*;
use crate::tvalue::*;
use crate::utility::*;
type TStringSuper = Object;
pub const TSTRING_SHORT_MAX: usize = 40;
pub const LSTRREG: i8 = 0;
pub const LSTRFIX: i8 = -1;
pub const LSTRMEM: i8 = -2;
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
enum TStringExtra {
    Short { extra: u8 },
    Long { hashed: bool },
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TString {
    tstring_super: TStringSuper,
    tstring_length: usize,
    pub tstring_hash_next: *mut TString,
    pub tstring_hash: u32,
    tstring_extra: TStringExtra,
    tstring_string_kind: i8,
    tstring_external_pointer: *const i8,
    tstring_allocation_function: AllocationFunction,
    tstring_user_data: *mut std::ffi::c_void,
    tstring_contents: [i8; 0],
}
impl TObject for TString {
    fn as_object(&self) -> &Object {
        &self.tstring_super
    }
    fn as_object_mut(&mut self) -> &mut Object {
        &mut self.tstring_super
    }
}
impl TString {
    pub fn is_unhashed(&self) -> bool {
        match self.tstring_extra {
            | TStringExtra::Long { hashed } => !hashed,
            | _ => false,
        }
    }
    pub fn set_hashed(&mut self) {
        self.tstring_extra = TStringExtra::Long { hashed: true };
    }
    pub unsafe fn hash_string_long(&mut self) -> u32 {
        unsafe {
            if self.is_unhashed() {
                let length = self.get_length();
                self.tstring_hash = luas_hash_long(self.get_contents_mut(), length, self.tstring_hash);
                self.set_hashed();
            }
            self.tstring_hash
        }
    }
    pub fn set_extra(&mut self, extra: u8) {
        self.tstring_extra = TStringExtra::Short { extra };
    }
    pub unsafe fn somefunction(&self) -> i32 {
        if self.get_tagvariant() == TagVariant::StringShort {
            match self.tstring_extra {
                | TStringExtra::Short { extra: x } => {
                    if 0 == x {
                        return Token::Name as i32;
                    } else {
                        return x as i32 - 1 + FIRST_RESERVED;
                    }
                },
                | _ => {
                    return Token::Name as i32;
                },
            }
        }
        Token::Name as i32
    }
    pub unsafe fn tstring_free(&mut self, state: *mut State) {
        unsafe {
            if self.get_tagvariant() == TagVariant::StringShort {
                self.remove_from_state(state);
                (*state).free_memory(
                    self as *mut TString as *mut std::ffi::c_void,
                    core::mem::size_of::<TString>() + 1 + self.get_length(),
                );
            } else {
                if self.tstring_string_kind == LSTRMEM {
                    (self.tstring_allocation_function.unwrap())(
                        self.tstring_user_data,
                        self.tstring_external_pointer as *mut std::ffi::c_void,
                        self.tstring_length + 1,
                        0,
                    );
                }
                let alloc_size = if self.tstring_string_kind != LSTRREG {
                    core::mem::size_of::<TString>()
                } else {
                    core::mem::size_of::<TString>() + 1 + self.get_length()
                };
                (*state).free_memory(self as *mut TString as *mut std::ffi::c_void, alloc_size);
            }
        }
    }
    pub unsafe fn remove_from_global(&mut self, global: *mut Global) {
        unsafe {
            let stringtable: *mut StringTable = &mut (*global).global_stringtable;
            (*stringtable).remove(self);
        }
    }
    pub unsafe fn remove_from_state(&mut self, state: *mut State) {
        unsafe {
            let global: *mut Global = &mut (*(*state).interpreter_global);
            self.remove_from_global(global);
        }
    }
    pub fn get_contents_mut(&self) -> *const i8 {
        if self.tstring_string_kind != LSTRREG {
            return self.tstring_external_pointer;
        }
        &self.tstring_contents as *const i8
    }
    pub fn get_contents(&mut self) -> *mut i8 {
        if self.tstring_string_kind != LSTRREG {
            return self.tstring_external_pointer as *mut i8;
        }
        &mut self.tstring_contents as *mut i8
    }
    pub fn is_external(&self) -> bool {
        self.tstring_string_kind != LSTRREG
    }
    pub fn get_length(&self) -> usize {
        self.tstring_length
    }
    pub fn get_length_raw(&self) -> usize {
        self.tstring_length
    }
    pub unsafe fn create_long(state: *mut State, length: usize) -> *mut TString {
        unsafe {
            let ret: *mut TString = createstrobj(
                state,
                length,
                TagVariant::StringLong,
                (*(*state).interpreter_global).global_seed,
            );
            (*ret).tstring_length = length;
            ret
        }
    }
    pub unsafe fn create_external(
        state: *mut State, s: *const i8, length: usize, allocation_function: AllocationFunction, user_data: *mut std::ffi::c_void,
    ) -> *mut TString {
        unsafe {
            let total_size = core::mem::size_of::<TString>();
            let object: *mut Object = luac_newobj(state, TagVariant::StringLong, total_size);
            let ret: *mut TString = &mut *(object as *mut TString);
            (*ret).tstring_hash = (*(*state).interpreter_global).global_seed;
            (*ret).tstring_extra = TStringExtra::Long { hashed: false };
            (*ret).tstring_length = length;
            (*ret).tstring_external_pointer = s;
            if allocation_function.is_some() {
                (*ret).tstring_string_kind = LSTRMEM;
                (*ret).tstring_allocation_function = allocation_function;
                (*ret).tstring_user_data = user_data;
            } else {
                (*ret).tstring_string_kind = LSTRFIX;
                (*ret).tstring_allocation_function = None;
                (*ret).tstring_user_data = std::ptr::null_mut();
            }
            ret
        }
    }
    pub unsafe fn intern(state: *mut State, str: *const i8, length: usize) -> *mut TString {
        unsafe {
            let global: *mut Global = (*state).interpreter_global;
            let tb: *mut StringTable = &mut (*global).global_stringtable;
            let h: u32 = luas_hash(str, length, (*global).global_seed);
            let mut list: *mut *mut TString =
                &mut *((*tb).stringtable_hash).add((h & ((*tb).stringtable_size - 1) as u32) as usize) as *mut *mut TString;
            let mut tstring: *mut TString = *list;
            while !tstring.is_null() {
                if length == (*tstring).get_length()
                    && std::slice::from_raw_parts(str as *const u8, length)
                        == std::slice::from_raw_parts((*tstring).get_contents() as *const u8, length)
                {
                    if (*tstring).get_marked() & ((*global).global_current_white ^ WHITEBITS) != 0 {
                        (*tstring).set_marked((*tstring).get_marked() ^ WHITEBITS);
                    }
                    return tstring;
                }
                tstring = (*tstring).tstring_hash_next;
            }
            if (*tb).stringtable_length >= (*tb).stringtable_size {
                growstrtab(state, tb);
                list = &mut *((*tb).stringtable_hash).add((h & ((*tb).stringtable_size - 1) as u32) as usize) as *mut *mut TString;
            }
            tstring = createstrobj(state, length, TagVariant::StringShort, h);
            (*tstring).tstring_length = length;
            std::ptr::copy_nonoverlapping(str as *const u8, (*tstring).get_contents() as *mut u8, length);
            (*tstring).tstring_hash_next = *list;
            *list = tstring;
            (*tb).stringtable_length += 1;
            tstring
        }
    }
    pub unsafe fn dump_string(dump_state: &mut DumpState, tstring: *const TString) {
        unsafe {
            if tstring.is_null() {
                dump_state.dump_size(0);
                dump_state.dump_size(0);
            } else {
                let slot = crate::table::luah_getstr(dump_state.dumpstate_table, tstring as *mut TString);
                if (*slot).get_tagvariant() != TagVariant::NilAbsentKey {
                    dump_state.dump_size(0);
                    dump_state.dump_size((*slot).as_integer().unwrap() as usize);
                } else {
                    let size: usize = (*tstring).get_length();
                    dump_state.dump_size(size + 1);
                    let pointer: *const i8 = (*tstring).get_contents_mut();
                    dump_state.dump_block(pointer as *const std::ffi::c_void, size + 1);
                    dump_state.dumpstate_count_string += 1;
                    let mut key = TValue::new(TagVariant::NilNil);
                    let ts_mut = tstring as *mut TString;
                    key.set_object(ts_mut as *mut Object, (*tstring).get_tagvariant());
                    let mut value = TValue::new(TagVariant::NilNil);
                    value.set_integer(dump_state.dumpstate_count_string as i64);
                    crate::table::luah_set(dump_state.state(), dump_state.dumpstate_table, &key, &mut value);
                }
            };
        }
    }
}
pub unsafe fn luas_eqlngstr(a: *mut TString, b: *mut TString) -> bool {
    unsafe {
        if a == b {
            true
        } else {
            let length = (*a).get_length();
            if length != (*b).get_length() {
                false
            } else {
                std::slice::from_raw_parts((*a).get_contents_mut() as *const u8, length)
                    == std::slice::from_raw_parts((*b).get_contents_mut() as *const u8, length)
            }
        }
    }
}
pub unsafe fn luas_hash(pointer: *const i8, mut length: usize, seed: u32) -> u32 {
    unsafe {
        let mut ret: u32 = seed ^ length as u32;
        while length > 0 {
            ret ^= (ret << 5)
                .wrapping_add(ret >> 2)
                .wrapping_add(*pointer.add(length - 1) as u8 as u32);
            length -= 1;
        }
        ret
    }
}
pub unsafe fn luas_hash_long(pointer: *const i8, mut length: usize, seed: u32) -> u32 {
    unsafe {
        let mut ret: u32 = seed ^ length as u32;
        let step: usize = (length >> 5) + 1;
        while length >= step {
            ret ^= (ret << 5)
                .wrapping_add(ret >> 2)
                .wrapping_add(*pointer.add(length - 1) as u8 as u32);
            length -= step;
        }
        ret
    }
}
pub unsafe fn createstrobj(state: *mut State, l: usize, tagvariant: TagVariant, h: u32) -> *mut TString {
    unsafe {
        let total_size = core::mem::size_of::<TString>() + 1 + l;
        let object: *mut Object = luac_newobj(state, tagvariant, total_size);
        let ret: *mut TString = &mut *(object as *mut TString);
        (*ret).tstring_hash = h;
        (*ret).tstring_extra = TStringExtra::Long { hashed: false };
        (*ret).tstring_string_kind = LSTRREG;
        (*ret).tstring_external_pointer = std::ptr::null();
        (*ret).tstring_allocation_function = None;
        (*ret).tstring_user_data = std::ptr::null_mut();
        *((*ret).get_contents()).add(l) = Character::Null as i8;
        ret
    }
}
pub unsafe fn luas_newlstr(state: *mut State, str: *const i8, length: usize) -> *mut TString {
    unsafe {
        if length <= TSTRING_SHORT_MAX {
            TString::intern(state, str, length)
        } else {
            if length >= (if (size_of::<usize>()) < size_of::<i64>() { !(0usize) } else { MAXIMUM_SIZE }) - size_of::<TString>() {
                (*state).too_big();
            }
            let tstring: *mut TString = TString::create_long(state, length);
            std::ptr::copy_nonoverlapping(str as *const u8, (*tstring).get_contents_mut() as *mut u8, length);
            tstring
        }
    }
}
pub unsafe fn luas_new(state: *mut State, str: *const i8) -> *mut TString {
    unsafe {
        let i: u32 = ((str as usize & u32::MAX as usize) as u32).wrapping_rem(GLOBAL_STRINGCACHE_N as u32);
        let p: *mut *mut TString = ((*(*state).interpreter_global).global_stringcache[i as usize]).as_mut_ptr();
        let mut j: i32 = 0;
        while j < GLOBAL_STRINGCACHE_M as i32 {
            if std::ffi::CStr::from_ptr(str) == std::ffi::CStr::from_ptr((**p.add(j as usize)).get_contents_mut()) {
                return *p.add(j as usize);
            }
            j += 1;
        }
        j = GLOBAL_STRINGCACHE_M as i32 - 1;
        while j > 0 {
            *p.add(j as usize) = *p.add((j - 1) as usize);
            j -= 1;
        }
        *p.add(0) = luas_newlstr(state, str, cstr_len(str));
        *p.add(0)
    }
}
pub unsafe fn luas_normstr(state: *mut State, ts: *mut TString) -> *mut TString {
    unsafe {
        let length = (*ts).get_length();
        if length > TSTRING_SHORT_MAX {
            ts
        } else {
            let str = (*ts).get_contents_mut();
            TString::intern(state, str, length)
        }
    }
}
pub unsafe fn l_strcmp(ts1: *const TString, ts2: *const TString) -> i32 {
    unsafe {
        let mut s1: *const i8 = (*ts1).get_contents_mut();
        let mut rl1 = (*ts1).get_length();
        let mut s2: *const i8 = (*ts2).get_contents_mut();
        let mut rl2 = (*ts2).get_length();
        loop {
            unsafe extern "C" {
                fn strcoll(s1: *const i8, s2: *const i8) -> i32;
            }
            let temp: i32 = strcoll(s1, s2);
            if temp != 0 {
                return temp;
            } else {
                let mut zl1 = cstr_len(s1);
                let mut zl2 = cstr_len(s2);
                if zl2 == rl2 {
                    return if zl1 == rl1 { 0 } else { 1 };
                } else if zl1 == rl1 {
                    return -1;
                }
                zl1 += 1;
                zl2 += 1;
                s1 = s1.add(zl1);
                rl1 -= zl1;
                s2 = s2.add(zl2);
                rl2 -= zl2;
            }
        }
    }
}
pub unsafe fn copy2buff(top: *mut TValue, mut n: i32, buffer: *mut i8) {
    unsafe {
        let mut tl: usize = 0;
        loop {
            let tstring: *mut TString = (*top.sub(n as usize)).as_string().unwrap();
            let length = (*tstring).get_length();
            std::ptr::copy_nonoverlapping((*tstring).get_contents_mut() as *const u8, buffer.add(tl) as *mut u8, length);
            tl = tl.wrapping_add(length);
            n -= 1;
            if n <= 0 {
                break;
            }
        }
    }
}
pub unsafe fn concatenate(state: *mut State, mut total: i32) {
    unsafe {
        if total == 1 {
            return;
        }
        loop {
            let top: *mut TValue = (*state).interpreter_top.stkidrel_pointer;
            let mut n: i32 = 2;
            if !((*top.sub(2)).get_tagvariant().to_tag_type().is_string()
                || (*top.sub(2)).get_tagvariant().to_tag_type().is_numeric())
                || !((*top.sub(1)).get_tagvariant().to_tag_type().is_string()
                    || (*top.sub(1)).get_tagvariant().to_tag_type().is_numeric() && {
                        (*top.sub(1)).from_interpreter_to_string(state);
                        1 != 0
                    })
            {
                luat_tryconcattm(state);
            } else if (*top.sub(1)).get_tagvariant() == TagVariant::StringShort
                && (*(*top.sub(1)).as_string().unwrap()).get_length() as i32 == 0
            {
                if !((*top.sub(2)).get_tagvariant().to_tag_type().is_string())
                    && (*top.sub(2)).get_tagvariant().to_tag_type().is_numeric()
                {
                    {
                        (*top.sub(2)).from_interpreter_to_string(state);
                        false
                    };
                }
            } else if (*top.sub(2)).get_tagvariant() == TagVariant::StringShort
                && (*(*top.sub(2)).as_string().unwrap()).get_length() as i32 == 0
            {
                let io1: *mut TValue = &mut (*top.sub(2));
                let io2: *const TValue = &mut (*top.sub(1));
                (*io1).copy_from(&*io2);
            } else {
                let mut tl = (*(*top.sub(1)).as_string().unwrap()).get_length();
                let tstring: *mut TString;
                n = 1;
                while n < total
                    && ((*top.sub(n as usize).sub(1)).get_tagvariant().to_tag_type().is_string()
                        || (*top.sub(n as usize).sub(1)).get_tagvariant().to_tag_type().is_numeric() && {
                            (*top.sub(n as usize).sub(1)).from_interpreter_to_string(state);
                            1 != 0
                        })
                {
                    let l = (*(*top.sub(n as usize).sub(1)).as_string().unwrap()).get_length();
                    if l >= (if (size_of::<usize>()) < size_of::<i64>() { !(0usize) } else { MAXIMUM_SIZE })
                        - size_of::<TString>()
                        - tl
                    {
                        (*state).interpreter_top.stkidrel_pointer = top.sub(total as usize);
                        luag_runerror(state, c"string length overflow".as_ptr(), &[]);
                    }
                    tl = tl.wrapping_add(l);
                    n += 1;
                }
                if tl <= LUAI_MAXSHORTLEN {
                    let mut buffer: [i8; LUAI_MAXSHORTLEN] = [0; LUAI_MAXSHORTLEN];
                    copy2buff(top, n, buffer.as_mut_ptr());
                    tstring = luas_newlstr(state, buffer.as_mut_ptr(), tl);
                } else {
                    tstring = TString::create_long(state, tl);
                    copy2buff(top, n, (*tstring).get_contents());
                }
                let io: *mut TValue = &mut (*top.sub(n as usize));
                (*io).set_object(tstring as *mut Object, (*tstring).get_tagvariant());
            }
            total -= n - 1;
            (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.sub((n - 1) as usize);
            if total <= 1 {
                break;
            }
        }
    }
}
pub unsafe fn get_position_relative(position: i64, length: usize) -> usize {
    if position > 0 {
        position as usize
    } else if position == 0 {
        1_usize
    } else if position < -(length as i64) {
        1_usize
    } else {
        length.wrapping_add(position as usize).wrapping_add(1_usize)
    }
}
pub unsafe fn get_position_end(state: *mut State, arg: i32, def: i64, length: usize) -> usize {
    unsafe {
        let position: i64 = lual_optinteger(state, arg, def);
        if position > length as i64 {
            length
        } else if position >= 0 {
            position as usize
        } else if position < -(length as i64) {
            0
        } else {
            length.wrapping_add(position as usize).wrapping_add(1_usize)
        }
    }
}
