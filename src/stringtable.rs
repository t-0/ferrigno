use crate::tstring::*;
use crate::state::*;
use crate::global::*;
use crate::object::*;
use crate::table::*;
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
pub unsafe extern "C" fn luas_resize(state: *mut State, new_size: i32) {
    unsafe {
        let tb: *mut StringTable = &mut (*(*state).global).string_table;
        let old_size: i32 = (*tb).size;
        if new_size < old_size {
            tablerehash((*tb).hash, old_size, new_size);
        }
        let newvect: *mut *mut TString = luam_realloc_(
            state,
            (*tb).hash as *mut libc::c_void,
            (old_size as usize).wrapping_mul(::core::mem::size_of::<*mut TString>()),
            (new_size as usize).wrapping_mul(::core::mem::size_of::<*mut TString>()),
        ) as *mut *mut TString;
        if ((newvect == std::ptr::null_mut() as *mut *mut TString) as i32 != 0) as i64 != 0 {
            if new_size < old_size {
                tablerehash((*tb).hash, new_size, old_size);
            }
        } else {
            (*tb).hash = newvect;
            (*tb).size = new_size;
            if new_size > old_size {
                tablerehash(newvect, old_size, new_size);
            }
        };
    }
}
pub const STRINGTABLE_INITIAL_SIZE: usize = 128;
pub const STRCACHE_N: usize = 53;
pub const STRCACHE_M: usize = 2;
pub unsafe extern "C" fn luas_init(state: *mut State) {
    unsafe {
        let global: *mut Global = (*state).global;
        let tb: *mut StringTable = &mut (*(*state).global).string_table;
        (*tb).hash = luam_malloc_(
            state,
            STRINGTABLE_INITIAL_SIZE.wrapping_mul(::core::mem::size_of::<*mut TString>()),
        ) as *mut *mut TString;
        tablerehash((*tb).hash, 0, STRINGTABLE_INITIAL_SIZE as i32);
        (*tb).size = STRINGTABLE_INITIAL_SIZE as i32;
        (*global).memory_error_message = luas_newlstr(
            state,
            b"not enough memory\0" as *const u8 as *const i8,
            (::core::mem::size_of::<[i8; 18]>() as u64)
                .wrapping_div(::core::mem::size_of::<i8>() as u64)
                .wrapping_sub(1 as u64),
        );
        luac_fix(state, &mut (*((*global).memory_error_message as *mut Object)));
        for i in 0..STRCACHE_N {
            for j in 0..STRCACHE_M {
                (*global).string_cache[i][j] = (*global).memory_error_message;
            }
        }
    }
}
pub unsafe extern "C" fn growstrtab(state: *mut State, tb: *mut StringTable) {
    unsafe {
        if (*tb).length == 0x7FFFFFF {
            luac_fullgc(state, true);
            if (*tb).length == 0x7FFFFFF {
                luad_throw(state, 4);
            }
        }
        if (*tb).size
            <= (if 0x7FFFFFF
                <= (!(0u64)).wrapping_div(::core::mem::size_of::<*mut TString>() as u64)
            {
                0x7FFFFFF
            } else {
                (!(0u64)).wrapping_div(::core::mem::size_of::<*mut TString>() as u64) as u32
            }) as i32
                / 2
        {
            luas_resize(state, (*tb).size * 2);
        }
    }
}
