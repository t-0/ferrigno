use crate::object::*;
use crate::table::*;
use crate::tag::*;
use crate::c::*;
use crate::global::*;
use crate::stringtable::*;
use crate::state::*;
pub const STRING_SHORT_MAX: u64 = 40;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TString {
    pub next: *mut Object,
    pub tag: u8,
    pub marked: u8,
    pub extra: u8,
    pub short_length: u8,
    pub hash: u32,
    pub u: TStringExtension,
    pub contents: [i8; 1],
}
impl TObject for TString {
    fn get_marked(&self) -> u8 {
        self.marked
    }
    fn set_marked(&mut self, marked_: u8) {
        self.marked = marked_;
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
        return self.tag;
    }
    fn get_tag_type(&self) -> u8 {
        return get_tag_type(self.get_tag());
    }
    fn get_tag_variant(&self) -> u8 {
        get_tag_variant(self.get_tag())
    }
    fn get_class_name(&mut self) -> String {
        "string".to_string()
    }
    fn get_metatable(&mut self) -> *mut Table {
        std::ptr::null_mut()
    }
}
#[derive(Copy, Clone)]
pub union TStringExtension {
    pub long_length: u64,
    pub hash_next: *mut TString,
}
impl TString {
    pub fn get_contents(&self) -> *const i8 {
        return &self.contents as *const i8;
    }
    pub fn get_contents2(&mut self) -> *mut i8 {
        return & mut self.contents as *mut i8;
    }
    pub fn get_length(&self) -> u64 {
        if self.short_length < 0xFF {
            return self.short_length as u64;
        } else {
            unsafe { return self.u.long_length as u64; }
        }
    }
    pub unsafe extern "C" fn create_long(state: *mut State, length: u64) -> *mut TString {
        unsafe {
            let ret: *mut TString = createstrobj(
                state,
                length,
                TAG_VARIANT_STRING_LONG,
                (*(*state).global).seed,
            );
            (*ret).u.long_length = length;
            (*ret).short_length = 0xFF;
            return ret;
        }
    }
    pub unsafe extern "C" fn intern(state: *mut State, str: *const i8, l: u64) -> *mut TString {
        unsafe {
            let g: *mut Global = (*state).global;
            let tb: *mut StringTable = &mut (*g).string_table;
            let h: u32 = luas_hash(str, l, (*g).seed);
            let mut list: *mut *mut TString = &mut *((*tb).hash)
                .offset((h & ((*tb).size - 1) as u32) as isize)
                as *mut *mut TString;
            let mut ts: *mut TString = *list;
            while !ts.is_null() {
                if l == (*ts).get_length() as u64
                    && memcmp(
                        str as *const libc::c_void,
                        (*ts).get_contents2() as *const libc::c_void,
                        l.wrapping_mul(::core::mem::size_of::<i8>() as u64),
                    ) == 0
                {
                    if (*ts).get_marked() & ((*g).current_white ^ (1 << 3 | 1 << 4)) != 0 {
                        (*ts).set_marked((*ts).get_marked() ^ (1 << 3 | 1 << 4));
                    }
                    return ts;
                }
                ts = (*ts).u.hash_next;
            }
            if (*tb).length >= (*tb).size {
                growstrtab(state, tb);
                list = &mut *((*tb).hash).offset((h & ((*tb).size - 1) as u32) as isize)
                    as *mut *mut TString;
            }
            ts = createstrobj(state, l, TAG_VARIANT_STRING_SHORT, h);
            (*ts).short_length = l as u8;
            memcpy(
                (*ts).get_contents2() as *mut libc::c_void,
                str as *const libc::c_void,
                l.wrapping_mul(::core::mem::size_of::<i8>() as u64),
            );
            (*ts).u.hash_next = *list;
            *list = ts;
            (*tb).length += 1;
            (*tb).length;
            return ts;
        }
    }
}
pub unsafe extern "C" fn luas_eqlngstr(a: *mut TString, b: *mut TString) -> i32 {
    unsafe {
        let length: u64 = (*a).get_length();
        return (a == b
            || length == (*b).get_length()
                && memcmp(
                    ((*a).get_contents()) as *const libc::c_void,
                    ((*b).get_contents()) as *const libc::c_void,
                    length,
                ) == 0) as i32;
    }
}
pub unsafe extern "C" fn luas_hash(str: *const i8, mut l: u64, seed: u32) -> u32 {
    unsafe {
        let mut h: u32 = seed ^ l as u32;
        while l > 0u64 {
            h ^= (h << 5)
                .wrapping_add(h >> 2)
                .wrapping_add(*str.offset(l.wrapping_sub(1 as u64) as isize) as u8 as u32);
            l = l.wrapping_sub(1);
        }
        return h;
    }
}
pub unsafe extern "C" fn luas_hashlongstr(ts: *mut TString) -> u32 {
    unsafe {
        if (*ts).extra as i32 == 0 {
            let length: u64 = (*ts).get_length();
            (*ts).hash = luas_hash((*ts).get_contents(), length, (*ts).hash);
            (*ts).extra = 1;
        }
        return (*ts).hash;
    }
}
pub unsafe extern "C" fn createstrobj(state: *mut State, l: u64, tag: u8, h: u32) -> *mut TString {
    unsafe {
        let totalsize: u64 = (24 as u64).wrapping_add(
            l.wrapping_add(1 as u64)
                .wrapping_mul(::core::mem::size_of::<i8>() as u64),
        );
        let o: *mut Object = luac_newobj(state, tag, totalsize);
        let ts: *mut TString = &mut (*(o as *mut TString));
        (*ts).hash = h;
        (*ts).extra = 0;
        *((*ts).get_contents2()).offset(l as isize) = '\0' as i8;
        return ts;
    }
}
pub unsafe extern "C" fn luas_remove(state: *mut State, ts: *mut TString) {
    unsafe {
        let tb: *mut StringTable = &mut (*(*state).global).string_table;
        let mut p: *mut *mut TString = &mut *((*tb).hash)
            .offset(((*ts).hash & ((*tb).size - 1) as u32) as isize)
            as *mut *mut TString;
        while *p != ts {
            p = &mut (**p).u.hash_next;
        }
        *p = (**p).u.hash_next;
        (*tb).length -= 1;
        (*tb).length;
    }
}
pub unsafe extern "C" fn luas_newlstr(state: *mut State, str: *const i8, l: u64) -> *mut TString {
    unsafe {
        if l <= STRING_SHORT_MAX as u64 {
            return TString::intern(state, str, l);
        } else {
            if ((l.wrapping_mul(::core::mem::size_of::<i8>() as u64)
        >= (if (::core::mem::size_of::<u64>() as u64) < ::core::mem::size_of::<i64>() as u64 {
                !(0u64)
            } else {
                0x7FFFFFFFFFFFFFFF as u64
            })
            .wrapping_sub(::core::mem::size_of::<TString>() as u64)) as i32
            != 0) as i64
            != 0
        {
            (*state).too_big();
        }
            let ts: *mut TString = TString::create_long(state, l);
            memcpy(
                ((*ts).get_contents()) as *mut libc::c_void,
                str as *const libc::c_void,
                l.wrapping_mul(::core::mem::size_of::<i8>() as u64),
            );
            return ts;
        };
    }
}
pub unsafe extern "C" fn luas_new(state: *mut State, str: *const i8) -> *mut TString {
    unsafe {
        let i: u32 = ((str as u64
            & (0x7FFFFFFF as u32)
                .wrapping_mul(2 as u32)
                .wrapping_add(1 as u32) as u64) as u32)
            .wrapping_rem(53 as u32);
        let p: *mut *mut TString = ((*(*state).global).strcache[i as usize]).as_mut_ptr();
        let mut j: i32 = 0;
        while j < 2 {
            if strcmp(str, (**p.offset(j as isize)).get_contents()) == 0 {
                return *p.offset(j as isize);
            }
            j += 1;
        }
        j = 2 - 1;
        while j > 0 {
            let ref mut fresh23 = *p.offset(j as isize);
            *fresh23 = *p.offset((j - 1) as isize);
            j -= 1;
        }
        let ref mut fresh24 = *p.offset(0 as isize);
        *fresh24 = luas_newlstr(state, str, strlen(str));
        return *p.offset(0 as isize);
    }
}
