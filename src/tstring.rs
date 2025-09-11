use std::ptr::*;
use crate::character::*;
use crate::global::*;
use crate::object::*;
use crate::stackvalue::*;
use crate::interpreter::*;
use crate::stringtable::*;
use crate::table::*;
use crate::tag::*;
use crate::tvalue::*;
use crate::utility::c::*;
use crate::utility::*;
pub const STRING_SHORT_MAX: usize = 40;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TString {
    pub object: Object,
    pub hash: u32,
    pub extra: u8,
    pub short_length: u8,
    pub u: TStringExtension,
    pub contents: [libc::c_char; 0],
}
impl TObject for TString {
    fn as_object(&self) -> &Object {
        &self.object
    }
    fn as_object_mut(&mut self) -> &mut Object {
        &mut self.object
    }
    fn get_class_name(&mut self) -> String {
        "string".to_string()
    }
    fn get_metatable(&mut self) -> *mut Table {
        null_mut()
    }
}
#[derive(Copy, Clone)]
pub union TStringExtension {
    pub long_length: usize,
    pub hash_next: *mut TString,
}
impl TString {
    pub unsafe fn free_tstring(&mut self, interpreter: *mut Interpreter) {
        unsafe {
            if self.get_tag_variant() == TAG_VARIANT_STRING_SHORT {
                self.remove_from_state(interpreter);
                (*interpreter).free_memory(
                    self as *mut TString as *mut libc::c_void,
                    core::mem::size_of::<TString>() + 1 + self.get_length() as usize,
                );
            } else {
                (*interpreter).free_memory(
                    self as *mut TString as *mut libc::c_void,
                    core::mem::size_of::<TString>() + 1 + self.get_length() as usize,
                );
            }
        }
    }
    pub unsafe fn remove_from_global(&mut self, global: *mut Global) {
        unsafe {
            let stringtable: *mut StringTable = &mut (*global).string_table;
            (*stringtable).remove(self);
        }
    }
    pub unsafe fn remove_from_state(&mut self, interpreter: *mut Interpreter) {
        unsafe {
            let global: *mut Global = &mut (*(*interpreter).global);
            self.remove_from_global(global);
        }
    }
    pub fn get_contents_mut(&self) -> *const libc::c_char {
        return &self.contents as *const libc::c_char;
    }
    pub fn get_contents(&mut self) -> *mut libc::c_char {
        return &mut self.contents as *mut libc::c_char;
    }
    pub fn get_length(&self) -> usize {
        if self.short_length < 0xFF {
            return self.short_length as usize;
        } else {
            unsafe {
                return self.u.long_length;
            }
        }
    }
    pub unsafe fn create_long(interpreter: *mut Interpreter, length: usize) -> *mut TString {
        unsafe {
            let ret: *mut TString = createstrobj(
                interpreter,
                length as usize,
                TAG_VARIANT_STRING_LONG,
                (*(*interpreter).global).seed,
            );
            (*ret).u.long_length = length;
            (*ret).short_length = 0xFF;
            return ret;
        }
    }
    pub unsafe fn intern(interpreter: *mut Interpreter, str: *const libc::c_char, length: usize) -> *mut TString {
        unsafe {
            let global: *mut Global = (*interpreter).global;
            let tb: *mut StringTable = &mut (*global).string_table;
            let h: u32 = luas_hash(str, length as usize, (*global).seed);
            let mut list: *mut *mut TString = &mut *((*tb).hash)
                .offset((h & ((*tb).size - 1) as u32) as isize)
                as *mut *mut TString;
            let mut ts: *mut TString = *list;
            while !ts.is_null() {
                if length as usize == (*ts).get_length() as usize
                    && memcmp(
                        str as *const libc::c_void,
                        (*ts).get_contents() as *const libc::c_void,
                        length.wrapping_mul(::core::mem::size_of::<libc::c_char>()),
                    ) == 0
                {
                    if (*ts).get_marked() & ((*global).current_white ^ (1 << 3 | 1 << 4)) != 0 {
                        (*ts).set_marked((*ts).get_marked() ^ (1 << 3 | 1 << 4));
                    }
                    return ts;
                }
                ts = (*ts).u.hash_next;
            }
            if (*tb).length >= (*tb).size {
                growstrtab(interpreter, tb);
                list = &mut *((*tb).hash).offset((h & ((*tb).size - 1) as u32) as isize)
                    as *mut *mut TString;
            }
            ts = createstrobj(interpreter, length as usize, TAG_VARIANT_STRING_SHORT, h);
            (*ts).short_length = length as u8;
            memcpy(
                (*ts).get_contents() as *mut libc::c_void,
                str as *const libc::c_void,
                length.wrapping_mul(::core::mem::size_of::<libc::c_char>()),
            );
            (*ts).u.hash_next = *list;
            *list = ts;
            (*tb).length += 1;
            (*tb).length;
            return ts;
        }
    }
}
pub unsafe extern "C" fn luas_eqlngstr(a: *mut TString, b: *mut TString) -> bool {
    unsafe {
        if a == b {
            return true;
        } else {
            let length = (*a).get_length();
            if length != (*b).get_length() {
                return false;
            } else {
                return 0
                    == memcmp(
                        ((*a).get_contents_mut()) as *const libc::c_void,
                        ((*b).get_contents_mut()) as *const libc::c_void,
                        length,
                    );
            }
        }
    }
}
pub unsafe extern "C" fn luas_hash(str: *const libc::c_char, mut l: usize, seed: u32) -> u32 {
    unsafe {
        let mut h: u32 = seed ^ l as u32;
        while l > 0 {
            h ^= (h << 5)
                .wrapping_add(h >> 2)
                .wrapping_add(*str.offset(l.wrapping_sub(1 as usize) as isize) as u8 as u32);
            l = l.wrapping_sub(1);
        }
        return h;
    }
}
pub unsafe extern "C" fn hash_string_long(ts: *mut TString) -> u32 {
    unsafe {
        if (*ts).extra == 0 {
            let length = (*ts).get_length();
            (*ts).hash = luas_hash((*ts).get_contents_mut(), length as usize, (*ts).hash);
            (*ts).extra = 1;
        }
        return (*ts).hash;
    }
}
pub unsafe extern "C" fn createstrobj(interpreter: *mut Interpreter, l: usize, tag: u8, h: u32) -> *mut TString {
    unsafe {
        let total_size = core::mem::size_of::<TString>() + 1 + l as usize;
        let o: *mut Object = luac_newobj(interpreter, tag, total_size);
        let ts: *mut TString = &mut (*(o as *mut TString));
        (*ts).hash = h;
        (*ts).extra = 0;
        *((*ts).get_contents()).offset(l as isize) = Character::Null as libc::c_char;
        return ts;
    }
}
pub unsafe extern "C" fn luas_newlstr(interpreter: *mut Interpreter, str: *const libc::c_char, length: usize) -> *mut TString {
    unsafe {
        if length <= STRING_SHORT_MAX {
            return TString::intern(interpreter, str, length);
        } else {
            if length.wrapping_mul(::core::mem::size_of::<libc::c_char>())
        >= (if (::core::mem::size_of::<usize>()) < ::core::mem::size_of::<i64>() {
                !(0usize)
            } else {
                MAXIMUM_SIZE
            })
            .wrapping_sub(::core::mem::size_of::<TString>())
        {
            (*interpreter).too_big();
        }
            let ts: *mut TString = TString::create_long(interpreter, length);
            memcpy(
                ((*ts).get_contents_mut()) as *mut libc::c_void,
                str as *const libc::c_void,
                length.wrapping_mul(::core::mem::size_of::<libc::c_char>()),
            );
            return ts;
        };
    }
}
pub unsafe extern "C" fn luas_new(interpreter: *mut Interpreter, str: *const libc::c_char) -> *mut TString {
    unsafe {
        let i: u32 = ((str as usize
            & (0x7FFFFFFF as u32)
                .wrapping_mul(2 as u32)
                .wrapping_add(1 as u32) as usize) as u32)
            .wrapping_rem(53 as u32);
        let p: *mut *mut TString = ((*(*interpreter).global).string_cache[i as usize]).as_mut_ptr();
        let mut j: i32 = 0;
        while j < 2 {
            if strcmp(str, (**p.offset(j as isize)).get_contents_mut()) == 0 {
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
        *fresh24 = luas_newlstr(interpreter, str, strlen(str) as usize);
        return *p.offset(0 as isize);
    }
}
pub unsafe extern "C" fn l_strcmp(ts1: *const TString, ts2: *const TString) -> i32 {
    unsafe {
        let mut s1: *const libc::c_char = (*ts1).get_contents_mut();
        let mut rl1 = (*ts1).get_length();
        let mut s2: *const libc::c_char = (*ts2).get_contents_mut();
        let mut rl2 = (*ts2).get_length();
        loop {
            let temp: i32 = strcoll(s1, s2);
            if temp != 0 {
                return temp;
            } else {
                let mut zl1 = strlen(s1);
                let mut zl2 = strlen(s2);
                if zl2 == rl2 {
                    return if zl1 == rl1 { 0 } else { 1 };
                } else if zl1 == rl1 {
                    return -1;
                }
                zl1 = zl1.wrapping_add(1);
                zl2 = zl2.wrapping_add(1);
                s1 = s1.offset(zl1 as isize);
                rl1 = rl1.wrapping_sub(zl1);
                s2 = s2.offset(zl2 as isize);
                rl2 = rl2.wrapping_sub(zl2);
            }
        }
    }
}
pub unsafe extern "C" fn copy2buff(top: StackValuePointer, mut n: i32, buffer: *mut libc::c_char) {
    unsafe {
        let mut tl: usize = 0;
        loop {
            let st: *mut TString =
                &mut (*((*top.offset(-(n as isize))).tvalue.value.object as *mut TString));
            let length = (*st).get_length();
            memcpy(
                buffer.offset(tl as isize) as *mut libc::c_void,
                ((*st).get_contents_mut()) as *const libc::c_void,
                length.wrapping_mul(::core::mem::size_of::<libc::c_char>()),
            );
            tl = tl.wrapping_add(length as usize);
            n -= 1;
            if !(n > 0) {
                break;
            }
        }
    }
}
pub unsafe extern "C" fn concatenate(interpreter: *mut Interpreter, mut total: i32) {
    unsafe {
        if total == 1 {
            return;
        }
        loop {
            let top: StackValuePointer = (*interpreter).top.stkidrel_pointer;
            let mut n: i32 = 2;
            if !((*top.offset(-(2 as isize))).tvalue.is_tagtype_string()
                || (*top.offset(-(2 as isize))).tvalue.is_tagtype_numeric())
                || !((*top.offset(-(1 as isize))).tvalue.is_tagtype_string()
                    || (*top.offset(-(1 as isize))).tvalue.is_tagtype_numeric()
                    && {
                            luao_tostring(interpreter, &mut (*top.offset(-(1 as isize))).tvalue);
                            1 != 0
                        })
            {
                luat_tryconcattm(interpreter);
            } else if (*top.offset(-(1 as isize))).tvalue.get_tag_variant()
                == TAG_VARIANT_STRING_SHORT
                && (*((*top.offset(-(1 as isize))).tvalue.value.object as *mut TString))
                    .get_length() as i32
                    == 0
            {
                (((*top.offset(-(2 as isize))).tvalue.is_tagtype_string())
                    || (*top.offset(-(2 as isize))).tvalue.is_tagtype_numeric()
                        && {
                            luao_tostring(interpreter, &mut (*top.offset(-(2 as isize))).tvalue);
                            1 != 0
                        }) as i32;
            } else if (*top.offset(-(2 as isize))).tvalue.get_tag_variant()
                == TAG_VARIANT_STRING_SHORT
                && (*((*top.offset(-(2 as isize))).tvalue.value.object as *mut TString))
                    .get_length() as i32
                    == 0
            {
                let io1: *mut TValue = &mut (*top.offset(-(2 as isize))).tvalue;
                let io2: *const TValue = &mut (*top.offset(-(1 as isize))).tvalue;
                (*io1).copy_from(&*io2);
            } else {
                let mut tl = (*((*top.offset(-(1 as isize))).tvalue.value.object
                    as *mut TString))
                    .get_length();
                let ts: *mut TString;
                n = 1;
                while n < total
                    && (
                        (*top.offset(-(n as isize)).offset(-(1 as isize)))
                            .tvalue
                            .is_tagtype_string()
                        ||
                            (*top.offset(-(n as isize)).offset(-(1 as isize)))
                                .tvalue
                                .is_tagtype_numeric()
                            && {
                                luao_tostring(
                                    interpreter,
                                    &mut (*top.offset(-(n as isize)).offset(-(1 as isize))).tvalue,
                                );
                                1 != 0
                            })
                {
                    let l = (*((*top.offset(-(n as isize)).offset(-(1 as isize)))
                        .tvalue
                        .value
                        .object as *mut TString))
                        .get_length();
                    if ((l
                        >= (if (::core::mem::size_of::<usize>())
                            < ::core::mem::size_of::<i64>()
                        {
                            !(0usize)
                        } else {
                            MAXIMUM_SIZE
                        })
                        .wrapping_sub(::core::mem::size_of::<TString>())
                        .wrapping_sub(tl)) as i32
                        != 0) as i64
                        != 0
                    {
                        (*interpreter).top.stkidrel_pointer = top.offset(-(total as isize));
                        luag_runerror(interpreter, b"string length overflow\0" as *const u8 as *const libc::c_char);
                    }
                    tl = tl.wrapping_add(l);
                    n += 1;
                }
                if tl <= 40 {
                    let mut buffer: [libc::c_char; 40] = [0; 40];
                    copy2buff(top, n, buffer.as_mut_ptr());
                    ts = luas_newlstr(interpreter, buffer.as_mut_ptr(), tl as usize);
                } else {
                    ts = TString::create_long(interpreter, tl);
                    copy2buff(top, n, (*ts).get_contents());
                }
                let io: *mut TValue = &mut (*top.offset(-(n as isize))).tvalue;
                (*io).value.object = &mut (*(ts as *mut Object));
                (*io).set_tag_variant((*ts).get_tag_variant());
                (*io).set_collectable(true);
            }
            total -= n - 1;
            (*interpreter).top.stkidrel_pointer =
                (*interpreter).top.stkidrel_pointer.offset(-((n - 1) as isize));
            if !(total > 1) {
                break;
            }
        }
    }
}
pub unsafe extern "C" fn get_position_relative(pos: i64, length: usize) -> usize {
    if pos > 0 {
        return pos as usize;
    } else if pos == 0 {
        return 1 as usize;
    } else if pos < -(length as i64) {
        return 1 as usize;
    } else {
        return length.wrapping_add(pos as usize).wrapping_add(1 as usize);
    };
}
pub unsafe extern "C" fn get_position_end(
    interpreter: *mut Interpreter,
    arg: i32,
    def: i64,
    length: usize,
) -> usize {
    unsafe {
        let pos: i64 = lual_optinteger(interpreter, arg, def);
        if pos > length as i64 {
            return length;
        } else if pos >= 0 {
            return pos as usize;
        } else if pos < -(length as i64) {
            return 0usize;
        } else {
            return length.wrapping_add(pos as usize).wrapping_add(1 as usize);
        };
    }
}
