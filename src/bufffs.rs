use crate::interpreter::*;
use crate::object::*;
use crate::tobject::*;
use crate::tstring::*;
use crate::tvalue::*;
use libc::*;
const BUFFFS_SIZE: usize = 0x100;
#[repr(C)]
pub struct BuffFS {
    m_interpreter: *mut Interpreter,
    m_ispushed: bool,
    m_size: usize,
    m_block: [i8; BUFFFS_SIZE],
}
impl BuffFS {
    pub fn new(interpreter: *mut Interpreter) -> Self {
        return BuffFS {
            m_interpreter: interpreter,
            m_ispushed: false,
            m_size: 0,
            m_block: [0; BUFFFS_SIZE],
        };
    }
    pub unsafe fn clear(&mut self) {
        unsafe {
            let tvalue: *mut TValue = &mut (*(*self.m_interpreter).interpreter_top.stkidrel_pointer);
            let tstring: *mut TString = luas_newlstr(self.m_interpreter, self.m_block.as_mut_ptr(), self.m_size);
            (*tvalue).tvalue_value.value_object = &mut (*(tstring as *mut Object));
            (*tvalue).tvalue_set_tag_variant((*tstring).get_tagvariant());
            (*tvalue).set_collectable(true);
            (*self.m_interpreter).interpreter_top.stkidrel_pointer =
                (*self.m_interpreter).interpreter_top.stkidrel_pointer.offset(1);
            if self.m_ispushed {
                concatenate(self.m_interpreter, 2);
            } else {
                self.m_ispushed = true;
            };
            self.m_size = 0;
        }
    }
    pub unsafe fn get_raw(&mut self, size: usize) -> *mut i8 {
        unsafe {
            if size > ((60 + 44 + 95) - self.m_size) {
                self.clear();
            }
            return self.m_block.as_mut_ptr().offset(self.m_size as isize);
        }
    }
    pub unsafe fn add_string(&mut self, pointer: *const i8, length: usize) {
        unsafe {
            if length <= (60 + 44 + 95) {
                let bf = self.get_raw(length as usize);
                libc::memcpy(bf as *mut c_void, pointer as *const c_void, length as usize);
                self.m_size += length as usize;
            } else {
                self.clear();
                let io = &mut (*(*self.m_interpreter).interpreter_top.stkidrel_pointer);
                let tstring = luas_newlstr(self.m_interpreter, pointer, length as usize);
                (*io).tvalue_value.value_object = &mut (*(tstring as *mut Object));
                (*io).tvalue_set_tag_variant((*tstring).get_tagvariant());
                (*io).set_collectable(true);
                (*self.m_interpreter).interpreter_top.stkidrel_pointer =
                    (*self.m_interpreter).interpreter_top.stkidrel_pointer.offset(1);
                if self.m_ispushed {
                    concatenate(self.m_interpreter, 2);
                } else {
                    self.m_ispushed = true;
                };
            };
        }
    }
    pub unsafe fn add_number(&mut self, number: *mut TValue) {
        unsafe {
            let number_buffer = self.get_raw(44);
            self.m_size += tostringbuff(number, number_buffer) as usize;
        }
    }
    pub fn add_length(&mut self, length: usize) {
        self.m_size += length;
    }
}
