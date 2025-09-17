use crate::interpreter::*;
use crate::object::*;
use crate::tstring::*;
use crate::tvalue::*;
use crate::utility::c::*;
const BUFFFS_SIZE: usize = 0x100;
#[repr(C)]
pub struct BuffFS {
    interpreter: *mut Interpreter,
    is_pushed: bool,
    size: usize,
    block: [i8; BUFFFS_SIZE],
}
impl BuffFS {
    pub fn new(interpreter: *mut Interpreter) -> Self {
        return BuffFS { interpreter: interpreter, is_pushed: false, size: 0, block: [0; BUFFFS_SIZE] };
    }
    pub unsafe fn clear(&mut self) {
        unsafe {
            let tvalue: *mut TValue = &mut (*(*self.interpreter).top.stkidrel_pointer);
            let tstring: *mut TString = luas_newlstr(self.interpreter, self.block.as_mut_ptr(), self.size);
            (*tvalue).value.object = &mut (*(tstring as *mut Object));
            (*tvalue).set_tag_variant((*tstring).get_tag_variant());
            (*tvalue).set_collectable(true);
            (*self.interpreter).top.stkidrel_pointer = (*self.interpreter).top.stkidrel_pointer.offset(1);
            if self.is_pushed {
                concatenate(self.interpreter, 2);
            } else {
                self.is_pushed = true;
            };
            self.size = 0;
        }
    }
    pub unsafe fn get_raw(&mut self, size: usize) -> *mut i8 {
        unsafe {
            if size > ((60 + 44 + 95) - self.size) {
                self.clear();
            }
            return self.block.as_mut_ptr().offset(self.size as isize);
        }
    }
    pub unsafe fn add_string(&mut self, pointer: *const i8, length: usize) {
        unsafe {
            if length <= (60 + 44 + 95) {
                let bf = self.get_raw(length as usize);
                memcpy(bf as *mut libc::c_void, pointer as *const libc::c_void, length as usize);
                self.size += length as usize;
            } else {
                self.clear();
                let io = &mut (*(*self.interpreter).top.stkidrel_pointer);
                let ts = luas_newlstr(self.interpreter, pointer, length as usize);
                (*io).value.object = &mut (*(ts as *mut Object));
                (*io).set_tag_variant((*ts).get_tag_variant());
                (*io).set_collectable(true);
                (*self.interpreter).top.stkidrel_pointer = (*self.interpreter).top.stkidrel_pointer.offset(1);
                if self.is_pushed {
                    concatenate(self.interpreter, 2);
                } else {
                    self.is_pushed = true;
                };
            };
        }
    }
    pub unsafe fn add_number(&mut self, number: *mut TValue) {
        unsafe {
            let number_buffer = self.get_raw(44);
            self.size += tostringbuff(number, number_buffer) as usize;
        }
    }
    pub fn add_length(&mut self, length: usize) {
        self.size += length;
    }
}
