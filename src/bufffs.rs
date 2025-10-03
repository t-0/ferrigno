use crate::interpreter::*;
use crate::object::*;
use crate::tobject::*;
use crate::tstring::*;
use crate::tvalue::*;
use libc::*;
const BUFFFS_SIZE: usize = 0x100;
#[repr(C)]
pub struct BuffFS {
    bufffs_interpreter: *mut Interpreter,
    bufffs_ispushed: bool,
    bufffs_size: usize,
    bufffs_block: [i8; BUFFFS_SIZE],
}
impl BuffFS {
    pub fn new(interpreter: *mut Interpreter) -> Self {
        return BuffFS {
            bufffs_interpreter: interpreter,
            bufffs_ispushed: false,
            bufffs_size: 0,
            bufffs_block: [0; BUFFFS_SIZE],
        };
    }
    pub unsafe fn clear(&mut self) {
        unsafe {
            let tvalue: *mut TValue = &mut (*(*self.bufffs_interpreter).interpreter_top.stkidrel_pointer);
            let tstring: *mut TString = luas_newlstr(self.bufffs_interpreter, self.bufffs_block.as_mut_ptr(), self.bufffs_size);
            (*tvalue).tvalue_value.value_object = &mut (*(tstring as *mut Object));
            (*tvalue).tvalue_set_tag_variant((*tstring).get_tagvariant());
            (*tvalue).set_collectable(true);
            (*self.bufffs_interpreter).interpreter_top.stkidrel_pointer =
                (*self.bufffs_interpreter).interpreter_top.stkidrel_pointer.offset(1);
            if self.bufffs_ispushed {
                concatenate(self.bufffs_interpreter, 2);
            } else {
                self.bufffs_ispushed = true;
            };
            self.bufffs_size = 0;
        }
    }
    pub unsafe fn get_raw(&mut self, size: usize) -> *mut i8 {
        unsafe {
            if size > ((60 + 44 + 95) - self.bufffs_size) {
                self.clear();
            }
            return self.bufffs_block.as_mut_ptr().offset(self.bufffs_size as isize);
        }
    }
    pub unsafe fn add_string(&mut self, pointer: *const i8, length: usize) {
        unsafe {
            if length <= (60 + 44 + 95) {
                let bf = self.get_raw(length as usize);
                memcpy(bf as *mut c_void, pointer as *const c_void, length as usize);
                self.bufffs_size += length as usize;
            } else {
                self.clear();
                let io = &mut (*(*self.bufffs_interpreter).interpreter_top.stkidrel_pointer);
                let tstring = luas_newlstr(self.bufffs_interpreter, pointer, length as usize);
                (*io).tvalue_value.value_object = &mut (*(tstring as *mut Object));
                (*io).tvalue_set_tag_variant((*tstring).get_tagvariant());
                (*io).set_collectable(true);
                (*self.bufffs_interpreter).interpreter_top.stkidrel_pointer =
                    (*self.bufffs_interpreter).interpreter_top.stkidrel_pointer.offset(1);
                if self.bufffs_ispushed {
                    concatenate(self.bufffs_interpreter, 2);
                } else {
                    self.bufffs_ispushed = true;
                };
            };
        }
    }
    pub unsafe fn add_number(&mut self, number: *mut TValue) {
        unsafe {
            let number_buffer = self.get_raw(44);
            self.bufffs_size += tostringbuff(number, number_buffer) as usize;
        }
    }
    pub fn add_length(&mut self, length: usize) {
        self.bufffs_size += length;
    }
}
