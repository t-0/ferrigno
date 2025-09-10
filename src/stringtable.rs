use crate::tstring::*;
use crate::interpreter::*;
use crate::global::*;
use crate::table::*;
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
    pub unsafe fn remove(& mut self, tstring: *mut TString) {
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
}
pub unsafe extern "C" fn luas_resize(interpreter: *mut Interpreter, new_size: usize) {
    unsafe {
        let tb: *mut StringTable = &mut (*(*interpreter).global).string_table;
        let old_size= (*tb).size as usize;
        if new_size < old_size {
            tablerehash((*tb).hash, old_size, new_size);
        }
        let newvect: *mut *mut TString = luam_realloc_(
            interpreter,
            (*tb).hash as *mut libc::c_void,
            old_size.wrapping_mul(::core::mem::size_of::<*mut TString>()),
            new_size.wrapping_mul(::core::mem::size_of::<*mut TString>()),
        ) as *mut *mut TString;
        if newvect.is_null() {
            if new_size < old_size {
                tablerehash((*tb).hash, new_size, old_size);
            }
        } else {
            (*tb).hash = newvect;
            (*tb).size = new_size as i32;
            if new_size > old_size {
                tablerehash(newvect, old_size, new_size);
            }
        };
    }
}
pub unsafe extern "C" fn luas_init_state(interpreter: *mut Interpreter) {
    unsafe {
        let global: *mut Global = (*interpreter).global;
        luas_init_global(global, interpreter);
    }
}
pub unsafe extern "C" fn luas_init_global(global: *mut Global, interpreter: *mut Interpreter) {
    unsafe {
        let tb: *mut StringTable = &mut (*global).string_table;
        (*tb).hash = luam_malloc_(
            interpreter,
            STRINGTABLE_INITIAL_SIZE.wrapping_mul(::core::mem::size_of::<*mut TString>()),
        ) as *mut *mut TString;
        tablerehash((*tb).hash, 0, STRINGTABLE_INITIAL_SIZE);
        (*tb).size = STRINGTABLE_INITIAL_SIZE as i32;
        (*global).memory_error_message = luas_newlstr(
            interpreter,
            b"not enough memory\0" as *const u8 as *const i8,
            (::core::mem::size_of::<[i8; 18]>() as u64)
                .wrapping_div(::core::mem::size_of::<i8>() as u64)
                .wrapping_sub(1 as u64),
        );
        (*global).fix_memory_error_message_global();
        (*global).stringcache_set_error();
    }
}
pub unsafe extern "C" fn growstrtab(interpreter: *mut Interpreter, tb: *mut StringTable) {
    unsafe {
        if (*tb).length as usize == STRINGTABLE_LENGTH_MAX {
            luac_fullgc(interpreter, true);
            if (*tb).length as usize == STRINGTABLE_LENGTH_MAX {
                luad_throw(interpreter, 4);
            }
        }
        if (*tb).size
            <= (if STRINGTABLE_LENGTH_MAX
                <= (!(0usize)).wrapping_div(::core::mem::size_of::<*mut TString>())
            {
                STRINGTABLE_LENGTH_MAX
            } else {
                (!(0usize)).wrapping_div(::core::mem::size_of::<*mut TString>())
            }) as i32
                / 2
        {
            luas_resize(interpreter, ((*tb).size * 2) as usize);
        }
    }
}
