use crate::object::*;
use crate::table::*;
use crate::tag::*;
use crate::c::*;
use crate::onelua::*;
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
