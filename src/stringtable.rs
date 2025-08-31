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
            (old_size as u64).wrapping_mul(::core::mem::size_of::<*mut TString>() as u64),
            (new_size as u64).wrapping_mul(::core::mem::size_of::<*mut TString>() as u64),
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
pub const STRINGTABLE_INITIAL_SIZE: u64 = 128;
pub unsafe extern "C" fn luas_init(state: *mut State) {
    unsafe {
        let g: *mut Global = (*state).global;
        let tb: *mut StringTable = &mut (*(*state).global).string_table;
        (*tb).hash = luam_malloc_(
            state,
            STRINGTABLE_INITIAL_SIZE.wrapping_mul(::core::mem::size_of::<*mut TString>() as u64),
        ) as *mut *mut TString;
        tablerehash((*tb).hash, 0, STRINGTABLE_INITIAL_SIZE as i32);
        (*tb).size = STRINGTABLE_INITIAL_SIZE as i32;
        (*g).memerrmsg = luas_newlstr(
            state,
            b"not enough memory\0" as *const u8 as *const i8,
            (::core::mem::size_of::<[i8; 18]>() as u64)
                .wrapping_div(::core::mem::size_of::<i8>() as u64)
                .wrapping_sub(1 as u64),
        );
        luac_fix(state, &mut (*((*g).memerrmsg as *mut Object)));
        let mut i: i32 = 0;
        while i < 53 as i32 {
            let mut j: i32 = 0;
            while j < 2 {
                (*g).strcache[i as usize][j as usize] = (*g).memerrmsg;
                j += 1;
            }
            i += 1;
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
