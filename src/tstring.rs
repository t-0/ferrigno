use crate::character::*;
use crate::dumpstate::*;
use crate::global::*;
use crate::interpreter::*;
use crate::object::*;
use crate::stringtable::*;
use crate::table::*;
use crate::tag::*;
use crate::tvalue::*;
use crate::utility::c::*;
use crate::utility::*;
use libc::memcmp;
use std::ptr::*;
pub const STRING_SHORT_MAX: usize = 40;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TString {
    pub object: Object,
    pub long_length: usize,
    pub hash_next: *mut TString,
    pub hash: u32,
    pub extra: u8,
    pub contents: [i8; 0],
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
impl TString {
    pub unsafe fn free_tstring(&mut self, interpreter: *mut Interpreter) {
        unsafe {
            if self.get_tag_variant() == TagVariant::StringShort as u8 {
                self.remove_from_state(interpreter);
                (*interpreter).free_memory(self as *mut TString as *mut libc::c_void, core::mem::size_of::<TString>() + 1 + self.get_length() as usize);
            } else {
                (*interpreter).free_memory(self as *mut TString as *mut libc::c_void, core::mem::size_of::<TString>() + 1 + self.get_length() as usize);
            }
        }
    }
    pub unsafe fn remove_from_global(&mut self, global: *mut Global) {
        unsafe {
            let stringtable: *mut StringTable = &mut (*global).stringtable;
            (*stringtable).remove(self);
        }
    }
    pub unsafe fn remove_from_state(&mut self, interpreter: *mut Interpreter) {
        unsafe {
            let global: *mut Global = &mut (*(*interpreter).global);
            self.remove_from_global(global);
        }
    }
    pub fn get_contents_mut(&self) -> *const i8 {
        return &self.contents as *const i8;
    }
    pub fn get_contents(&mut self) -> *mut i8 {
        return &mut self.contents as *mut i8;
    }
    pub fn get_length(&self) -> usize {
        return self.long_length;
    }
    pub unsafe fn create_long(interpreter: *mut Interpreter, length: usize) -> *mut TString {
        unsafe {
            let ret: *mut TString = createstrobj(interpreter, length as usize, TagVariant::StringLong as u8, (*(*interpreter).global).seed);
            (*ret).long_length = length;
            return ret;
        }
    }
    pub unsafe fn intern(interpreter: *mut Interpreter, str: *const i8, length: usize) -> *mut TString {
        unsafe {
            let global: *mut Global = (*interpreter).global;
            let tb: *mut StringTable = &mut (*global).stringtable;
            let h: u32 = luas_hash(str, length as usize, (*global).seed);
            let mut list: *mut *mut TString = &mut *((*tb).stringtable_hash).offset((h & ((*tb).stringtable_size - 1) as u32) as isize) as *mut *mut TString;
            let mut tstring: *mut TString = *list;
            while !tstring.is_null() {
                if length as usize == (*tstring).get_length() as usize && memcmp(str as *const libc::c_void, (*tstring).get_contents() as *const libc::c_void, length) == 0 {
                    if (*tstring).get_marked() & ((*global).current_white ^ (1 << 3 | 1 << 4)) != 0 {
                        (*tstring).set_marked((*tstring).get_marked() ^ (1 << 3 | 1 << 4));
                    }
                    return tstring;
                }
                tstring = (*tstring).hash_next;
            }
            if (*tb).stringtable_length >= (*tb).stringtable_size {
                growstrtab(interpreter, tb);
                list = &mut *((*tb).stringtable_hash).offset((h & ((*tb).stringtable_size - 1) as u32) as isize) as *mut *mut TString;
            }
            tstring = createstrobj(interpreter, length as usize, TagVariant::StringShort as u8, h);
            (*tstring).long_length = length;
            memcpy((*tstring).get_contents() as *mut libc::c_void, str as *const libc::c_void, length);
            (*tstring).hash_next = *list;
            *list = tstring;
            (*tb).stringtable_length += 1;
            return tstring;
        }
    }
    pub unsafe fn dump_string(dump_state: &mut DumpState, tstring: *const TString) {
        unsafe {
            if tstring.is_null() {
                dump_state.dump_size(0);
            } else {
                let size: usize = (*tstring).get_length() as usize;
                dump_state.dump_size(size.wrapping_add(1) as usize);
                let pointer: *const i8 = (*tstring).get_contents_mut();
                dump_state.dump_block(pointer as *const libc::c_void, size);
            };
        }
    }
}
pub unsafe fn luas_eqlngstr(a: *mut TString, b: *mut TString) -> bool {
    unsafe {
        if a == b {
            return true;
        } else {
            let length = (*a).get_length();
            if length != (*b).get_length() {
                return false;
            } else {
                return 0 == memcmp(((*a).get_contents_mut()) as *const libc::c_void, ((*b).get_contents_mut()) as *const libc::c_void, length);
            }
        }
    }
}
pub unsafe fn luas_hash(pointer: *const i8, mut length: usize, seed: u32) -> u32 {
    unsafe {
        let mut ret: u32 = seed ^ length as u32;
        while length > 0 {
            ret ^= (ret << 5).wrapping_add(ret >> 2).wrapping_add(*pointer.offset((length - 1) as isize) as u8 as u32);
            length -= 1;
        }
        return ret;
    }
}
pub unsafe fn hash_string_long(tstring: *mut TString) -> u32 {
    unsafe {
        if (*tstring).extra == 0 {
            let length = (*tstring).get_length();
            (*tstring).hash = luas_hash((*tstring).get_contents_mut(), length, (*tstring).hash);
            (*tstring).extra = 1;
        }
        return (*tstring).hash;
    }
}
pub unsafe fn createstrobj(interpreter: *mut Interpreter, l: usize, tag: u8, h: u32) -> *mut TString {
    unsafe {
        let total_size = core::mem::size_of::<TString>() + 1 + l as usize;
        let object: *mut Object = luac_newobj(interpreter, tag, total_size);
        let ret: *mut TString = &mut (*(object as *mut TString));
        (*ret).hash = h;
        (*ret).extra = 0;
        *((*ret).get_contents()).offset(l as isize) = Character::Null as i8;
        return ret;
    }
}
pub unsafe fn luas_newlstr(interpreter: *mut Interpreter, str: *const i8, length: usize) -> *mut TString {
    unsafe {
        if length <= STRING_SHORT_MAX {
            return TString::intern(interpreter, str, length);
        } else {
            if length >= (if (size_of::<usize>()) < size_of::<i64>() { !(0usize) } else { MAXIMUM_SIZE }) - size_of::<TString>() {
                (*interpreter).too_big();
            }
            let tstring: *mut TString = TString::create_long(interpreter, length);
            memcpy(((*tstring).get_contents_mut()) as *mut libc::c_void, str as *const libc::c_void, length);
            return tstring;
        };
    }
}
pub unsafe fn luas_new(interpreter: *mut Interpreter, str: *const i8) -> *mut TString {
    unsafe {
        let i: u32 = ((str as usize & (0x7FFFFFFF as u32).wrapping_mul(2 as u32).wrapping_add(1 as u32) as usize) as u32).wrapping_rem(53 as u32);
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
pub unsafe fn l_strcmp(ts1: *const TString, ts2: *const TString) -> i32 {
    unsafe {
        let mut s1: *const i8 = (*ts1).get_contents_mut();
        let mut rl1 = (*ts1).get_length();
        let mut s2: *const i8 = (*ts2).get_contents_mut();
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
                zl1 += 1;
                zl2 += 1;
                s1 = s1.offset(zl1 as isize);
                rl1 -= zl1;
                s2 = s2.offset(zl2 as isize);
                rl2 -= zl2;
            }
        }
    }
}
pub unsafe fn copy2buff(top: *mut TValue, mut n: i32, buffer: *mut i8) {
    unsafe {
        let mut tl: usize = 0;
        loop {
            let tstring: *mut TString = &mut (*((*top.offset(-(n as isize))).value.value_object as *mut TString));
            let length = (*tstring).get_length();
            memcpy(buffer.offset(tl as isize) as *mut libc::c_void, ((*tstring).get_contents_mut()) as *const libc::c_void, length);
            tl = tl.wrapping_add(length as usize);
            n -= 1;
            if !(n > 0) {
                break;
            }
        }
    }
}
pub unsafe fn concatenate(interpreter: *mut Interpreter, mut total: i32) {
    unsafe {
        if total == 1 {
            return;
        }
        loop {
            let top: *mut TValue = (*interpreter).top.stkidrel_pointer;
            let mut n: i32 = 2;
            if !((*top.offset(-(2 as isize))).is_tagtype_string() || (*top.offset(-(2 as isize))).is_tagtype_numeric())
                || !((*top.offset(-(1 as isize))).is_tagtype_string()
                    || (*top.offset(-(1 as isize))).is_tagtype_numeric() && {
                        (*top.offset(-(1 as isize))).from_interpreter_to_string(interpreter);
                        1 != 0
                    })
            {
                luat_tryconcattm(interpreter);
            } else if (*top.offset(-(1 as isize))).get_tag_variant() == TagVariant::StringShort as u8 && (*((*top.offset(-(1 as isize))).value.value_object as *mut TString)).get_length() as i32 == 0 {
                (((*top.offset(-(2 as isize))).is_tagtype_string())
                    || (*top.offset(-(2 as isize))).is_tagtype_numeric() && {
                        (*top.offset(-(2 as isize))).from_interpreter_to_string(interpreter);
                        1 != 0
                    }) as i32;
            } else if (*top.offset(-(2 as isize))).get_tag_variant() == TagVariant::StringShort as u8 && (*((*top.offset(-(2 as isize))).value.value_object as *mut TString)).get_length() as i32 == 0 {
                let io1: *mut TValue = &mut (*top.offset(-(2 as isize)));
                let io2: *const TValue = &mut (*top.offset(-(1 as isize)));
                (*io1).copy_from(&*io2);
            } else {
                let mut tl = (*((*top.offset(-(1 as isize))).value.value_object as *mut TString)).get_length();
                let tstring: *mut TString;
                n = 1;
                while n < total
                    && ((*top.offset(-(n as isize)).offset(-(1 as isize))).is_tagtype_string()
                        || (*top.offset(-(n as isize)).offset(-(1 as isize))).is_tagtype_numeric() && {
                            (*top.offset(-(n as isize)).offset(-(1 as isize))).from_interpreter_to_string(interpreter);
                            1 != 0
                        })
                {
                    let l = (*((*top.offset(-(n as isize)).offset(-(1 as isize))).value.value_object as *mut TString)).get_length();
                    if ((l >= (if (size_of::<usize>()) < size_of::<i64>() { !(0usize) } else { MAXIMUM_SIZE }) - size_of::<TString>() - tl) as i32 != 0) as i64 != 0 {
                        (*interpreter).top.stkidrel_pointer = top.offset(-(total as isize));
                        luag_runerror(interpreter, c"string length overflow".as_ptr());
                    }
                    tl = tl.wrapping_add(l);
                    n += 1;
                }
                if tl <= 40 {
                    let mut buffer: [i8; 40] = [0; 40];
                    copy2buff(top, n, buffer.as_mut_ptr());
                    tstring = luas_newlstr(interpreter, buffer.as_mut_ptr(), tl as usize);
                } else {
                    tstring = TString::create_long(interpreter, tl);
                    copy2buff(top, n, (*tstring).get_contents());
                }
                let io: *mut TValue = &mut (*top.offset(-(n as isize)));
                (*io).value.value_object = &mut (*(tstring as *mut Object));
                (*io).set_tag_variant((*tstring).get_tag_variant());
                (*io).set_collectable(true);
            }
            total -= n - 1;
            (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(-((n - 1) as isize));
            if !(total > 1) {
                break;
            }
        }
    }
}
pub unsafe fn get_position_relative(position: i64, length: usize) -> usize {
    if position > 0 {
        return position as usize;
    } else if position == 0 {
        return 1 as usize;
    } else if position < -(length as i64) {
        return 1 as usize;
    } else {
        return length.wrapping_add(position as usize).wrapping_add(1 as usize);
    };
}
pub unsafe fn get_position_end(interpreter: *mut Interpreter, arg: i32, def: i64, length: usize) -> usize {
    unsafe {
        let position: i64 = lual_optinteger(interpreter, arg, def);
        if position > length as i64 {
            return length;
        } else if position >= 0 {
            return position as usize;
        } else if position < -(length as i64) {
            return 0;
        } else {
            return length.wrapping_add(position as usize).wrapping_add(1 as usize);
        };
    }
}
