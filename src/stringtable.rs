use crate::interpreter::*;
use crate::status::*;
use crate::table::*;
use crate::tstring::*;
pub const STRINGTABLE_INITIAL_SIZE: usize = 128;
pub const GLOBAL_STRINGCACHE_N: usize = 53;
pub const GLOBAL_STRINGCACHE_M: usize = 2;
pub const STRINGTABLE_MAX_SIZE: usize = (!0usize) / size_of::<*mut TString>();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StringTable {
    pub stringtable_hash: *mut *mut TString,
    pub stringtable_length: usize,
    pub stringtable_size: usize,
}
impl StringTable {
    pub unsafe fn remove(&mut self, tstring: *mut TString) {
        unsafe {
            let mut it: *mut *mut TString = &mut *(self.stringtable_hash).offset(((*tstring).hash & (self.stringtable_size - 1) as u32) as isize) as *mut *mut TString;
            while *it != tstring {
                it = &mut (**it).hash_next;
            }
            *it = (**it).hash_next;
            self.stringtable_length -= 1;
        }
    }
    pub unsafe fn resize(&mut self, interpreter: *mut Interpreter, new_size: usize) {
        unsafe {
            let old_size = self.stringtable_size;
            if new_size < old_size {
                tablerehash(self.stringtable_hash, old_size, new_size);
            }
            let new_vector: *mut *mut TString = luam_realloc_(
                interpreter,
                self.stringtable_hash as *mut libc::c_void,
                old_size.wrapping_mul(size_of::<*mut TString>()),
                new_size.wrapping_mul(size_of::<*mut TString>()),
            ) as *mut *mut TString;
            if new_vector.is_null() {
                if new_size < old_size {
                    tablerehash(self.stringtable_hash, new_size, old_size);
                }
            } else {
                self.stringtable_hash = new_vector;
                self.stringtable_size = new_size;
                if new_size > old_size {
                    tablerehash(new_vector, old_size, new_size);
                }
            };
        }
    }
    pub unsafe fn initialize(&mut self, interpreter: *mut Interpreter) {
        unsafe {
            self.stringtable_hash = luam_malloc_(interpreter, STRINGTABLE_INITIAL_SIZE.wrapping_mul(size_of::<*mut TString>())) as *mut *mut TString;
            tablerehash(self.stringtable_hash, 0, STRINGTABLE_INITIAL_SIZE);
            self.stringtable_size = STRINGTABLE_INITIAL_SIZE;
        }
    }
}
pub unsafe fn luas_resize(interpreter: *mut Interpreter, new_size: usize) {
    unsafe {
        let stringtable: *mut StringTable = &mut (*(*interpreter).global).stringtable;
        (*stringtable).resize(interpreter, new_size);
    }
}
pub unsafe fn growstrtab(interpreter: *mut Interpreter, tb: *mut StringTable) {
    unsafe {
        if (*tb).stringtable_length as usize == STRINGTABLE_MAX_SIZE {
            (*interpreter).luac_fullgc(true);
            if (*tb).stringtable_length as usize == STRINGTABLE_MAX_SIZE {
                luad_throw(interpreter, Status::MemoryError);
            }
        }
        if (*tb).stringtable_size <= STRINGTABLE_MAX_SIZE / 2 {
            luas_resize(interpreter, (*tb).stringtable_size * 2);
        }
    }
}
