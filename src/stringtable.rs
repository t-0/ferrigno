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
            let mut it: *mut *mut TString = &mut *(self.stringtable_hash)
                .offset(((*tstring).tstring_hash & (self.stringtable_size - 1) as u32) as isize)
                as *mut *mut TString;
            while *it != tstring {
                it = &mut (**it).tstring_hashnext;
            }
            *it = (**it).tstring_hashnext;
            self.stringtable_length -= 1;
        }
    }
    pub unsafe fn resize(&mut self, interpreter: *mut Interpreter, newsize: usize) {
        unsafe {
            let oldsize = self.stringtable_size;
            if newsize < oldsize {
                tablerehash(self.stringtable_hash, oldsize, newsize);
            }
            let new_vector: *mut *mut TString = (*interpreter).reallocate(
                self.stringtable_hash as *mut libc::c_void,
                oldsize.wrapping_mul(size_of::<*mut TString>()),
                newsize.wrapping_mul(size_of::<*mut TString>()),
            ) as *mut *mut TString;
            if new_vector.is_null() {
                if newsize < oldsize {
                    tablerehash(self.stringtable_hash, newsize, oldsize);
                }
            } else {
                self.stringtable_hash = new_vector;
                self.stringtable_size = newsize;
                if newsize > oldsize {
                    tablerehash(new_vector, oldsize, newsize);
                }
            };
        }
    }
    pub unsafe fn initialize(&mut self, interpreter: *mut Interpreter) {
        unsafe {
            self.stringtable_hash =
                (*interpreter).allocate(STRINGTABLE_INITIAL_SIZE.wrapping_mul(size_of::<*mut TString>())) as *mut *mut TString;
            tablerehash(self.stringtable_hash, 0, STRINGTABLE_INITIAL_SIZE);
            self.stringtable_size = STRINGTABLE_INITIAL_SIZE;
        }
    }
}
pub unsafe fn luas_resize(interpreter: *mut Interpreter, newsize: usize) {
    unsafe {
        let stringtable: *mut StringTable = &mut (*(*interpreter).interpreter_global).global_stringtable;
        (*stringtable).resize(interpreter, newsize);
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
