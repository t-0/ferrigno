use crate::functionstate::{BUFVFS, LUA_N2SBUFFSZ};
use crate::object::*;
use crate::state::*;
use crate::tobject::*;
use crate::tstring::*;
use crate::tvalue::*;
impl BuffFS {
    const INITIAL_SIZE: usize = 0x100;
}
#[repr(C)]
pub struct BuffFS {
    bufffs_interpreter: *mut State,
    bufffs_is_pushed: bool,
    bufffs_size: usize,
    bufffs_block: [i8; BuffFS::INITIAL_SIZE],
}
impl BuffFS {
    pub fn new(state: *mut State) -> Self {
        BuffFS {
            bufffs_interpreter: state,
            bufffs_is_pushed: false,
            bufffs_size: 0,
            bufffs_block: [0; BuffFS::INITIAL_SIZE],
        }
    }
    pub unsafe fn clear(&mut self) {
        unsafe {
            let tvalue: *mut TValue = &mut (*(*self.bufffs_interpreter).interpreter_top.stkidrel_pointer);
            let tstring: *mut TString = luas_newlstr(
                self.bufffs_interpreter,
                self.bufffs_block.as_mut_ptr(),
                self.bufffs_size,
            );
            (*tvalue).set_object(tstring as *mut Object, (*tstring).get_tagvariant());
            (*self.bufffs_interpreter).interpreter_top.stkidrel_pointer = (*self.bufffs_interpreter)
                .interpreter_top
                .stkidrel_pointer
                .add(1);
            if self.bufffs_is_pushed {
                concatenate(self.bufffs_interpreter, 2);
            } else {
                self.bufffs_is_pushed = true;
            };
            self.bufffs_size = 0;
        }
    }
    pub unsafe fn get_raw(&mut self, size: usize) -> *mut i8 {
        unsafe {
            if size > (BUFVFS - self.bufffs_size) {
                self.clear();
            }
            self.bufffs_block.as_mut_ptr().add(self.bufffs_size)
        }
    }
    pub unsafe fn add_string(&mut self, pointer: *const i8, length: usize) {
        unsafe {
            if length <= BUFVFS {
                let bf = self.get_raw(length);
                std::ptr::copy_nonoverlapping(pointer as *const u8, bf as *mut u8, length);
                self.bufffs_size += length;
            } else {
                self.clear();
                let io = &mut (*(*self.bufffs_interpreter).interpreter_top.stkidrel_pointer);
                let tstring = luas_newlstr(self.bufffs_interpreter, pointer, length);
                (*io).set_object(tstring as *mut Object, (*tstring).get_tagvariant());
                (*self.bufffs_interpreter).interpreter_top.stkidrel_pointer = (*self.bufffs_interpreter)
                    .interpreter_top
                    .stkidrel_pointer
                    .add(1);
                if self.bufffs_is_pushed {
                    concatenate(self.bufffs_interpreter, 2);
                } else {
                    self.bufffs_is_pushed = true;
                };
            };
        }
    }
    pub unsafe fn add_number(&mut self, number: *mut TValue) {
        unsafe {
            let number_buffer = self.get_raw(LUA_N2SBUFFSZ);
            self.bufffs_size += tostringbuff(number, number_buffer);
        }
    }
    pub fn add_length(&mut self, length: usize) {
        self.bufffs_size += length;
    }
}
