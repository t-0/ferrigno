use crate::interpreter::*;
use crate::table::*;
use crate::tstring::*;
pub const STRINGTABLE_INITIAL_SIZE: usize = 128;
pub const GLOBAL_STRINGCACHE_N: usize = 53;
pub const GLOBAL_STRINGCACHE_M: usize = 2;
pub const STRINGTABLE_LENGTH_MAX: usize = 0x7FFFFFFF;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StringTable {
    pub hash: *mut *mut TString,
    pub length: i32,
    pub size: i32,
}
impl StringTable {
    pub unsafe fn remove(&mut self, tstring: *mut TString) {
        unsafe {
            let mut p: *mut *mut TString = &mut *(self.hash)
                .offset(((*tstring).hash & (self.size - 1) as u32) as isize)
                as *mut *mut TString;
            while *p != tstring {
                p = &mut (**p).u.hash_next;
            }
            *p = (**p).u.hash_next;
            self.length -= 1;
        }
    }
    pub unsafe extern "C" fn resize(&mut self, interpreter: *mut Interpreter, new_size: usize) {
        unsafe {
            let old_size = self.size as usize;
            if new_size < old_size {
                tablerehash(self.hash, old_size, new_size);
            }
            let newvect: *mut *mut TString = luam_realloc_(
                interpreter,
                self.hash as *mut libc::c_void,
                old_size.wrapping_mul(size_of::<*mut TString>()),
                new_size.wrapping_mul(size_of::<*mut TString>()),
            ) as *mut *mut TString;
            if newvect.is_null() {
                if new_size < old_size {
                    tablerehash(self.hash, new_size, old_size);
                }
            } else {
                self.hash = newvect;
                self.size = new_size as i32;
                if new_size > old_size {
                    tablerehash(newvect, old_size, new_size);
                }
            };
        }
    }
    pub unsafe extern "C" fn initialize(&mut self, interpreter: *mut Interpreter) {
        unsafe {
            self.hash = luam_malloc_(
                interpreter,
                STRINGTABLE_INITIAL_SIZE.wrapping_mul(size_of::<*mut TString>()),
            ) as *mut *mut TString;
            tablerehash(self.hash, 0, STRINGTABLE_INITIAL_SIZE);
            self.size = STRINGTABLE_INITIAL_SIZE as i32;
        }
    }
}
pub unsafe extern "C" fn luas_resize(interpreter: *mut Interpreter, new_size: usize) {
    unsafe {
        let tb: *mut StringTable = &mut (*(*interpreter).global).string_table;
        (*tb).resize(interpreter, new_size);
    }
}
pub unsafe extern "C" fn growstrtab(interpreter: *mut Interpreter, tb: *mut StringTable) {
    unsafe {
        if (*tb).length as usize == STRINGTABLE_LENGTH_MAX {
            (*interpreter).luac_fullgc(true);
            if (*tb).length as usize == STRINGTABLE_LENGTH_MAX {
                luad_throw(interpreter, 4);
            }
        }
        if (*tb).size
            <= (if STRINGTABLE_LENGTH_MAX <= (!(0usize)).wrapping_div(size_of::<*mut TString>()) {
                STRINGTABLE_LENGTH_MAX
            } else {
                (!(0usize)).wrapping_div(size_of::<*mut TString>())
            }) as i32
                / 2
        {
            luas_resize(interpreter, ((*tb).size * 2) as usize);
        }
    }
}
