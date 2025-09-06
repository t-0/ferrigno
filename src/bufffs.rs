use crate::utility::c::*;
use crate::object::*;
use crate::state::*;
use crate::tstring::*;
use crate::tvalue::*;
const BUFFFS_SIZE: usize = 0x100;
#[repr(C)]
pub struct BuffFS {
    state: *mut State,
    is_pushed: bool,
    size: usize,
    block: [i8; BUFFFS_SIZE],
}
impl BuffFS {
    pub fn new(state_: *mut State) -> Self {
        return BuffFS {
            state: state_,
            is_pushed: false,
            size: 0,
            block: [0; BUFFFS_SIZE],
        };
    }
    pub unsafe extern "C" fn clear(&mut self) {
        unsafe {
            let io: *mut TValue = &mut (*(*self.state).top.stkidrel_pointer).tvalue;
            let ts: *mut TString =
                luas_newlstr(self.state, self.block.as_mut_ptr(), self.size as u64);
            (*io).value.object = &mut (*(ts as *mut Object));
            (*io).set_tag((*ts).get_tag());
            (*io).set_collectable();
            (*self.state).top.stkidrel_pointer = (*self.state).top.stkidrel_pointer.offset(1);
            if self.is_pushed {
                concatenate(self.state, 2);
            } else {
                self.is_pushed = true;
            };
            self.size = 0;
        }
    }
    pub unsafe extern "C" fn get_raw(&mut self, size: usize) -> *mut i8 {
        unsafe {
            if size > ((60 + 44 + 95) - self.size) {
                self.clear();
            }
            return self.block.as_mut_ptr().offset(self.size as isize);
        }
    }
    pub unsafe extern "C" fn add_string(&mut self, pointer: *const i8, length: u64) {
        unsafe {
            if length <= (60 + 44 + 95) {
                let bf = self.get_raw(length as usize);
                memcpy(
                    bf as *mut libc::c_void,
                    pointer as *const libc::c_void,
                    length,
                );
                self.size += length as usize;
            } else {
                self.clear();
                let io = &mut (*(*self.state).top.stkidrel_pointer).tvalue;
                let ts = luas_newlstr(self.state, pointer, length);
                (*io).value.object = &mut (*(ts as *mut Object));
                (*io).set_tag((*ts).get_tag());
                (*io).set_collectable();
                (*self.state).top.stkidrel_pointer = (*self.state).top.stkidrel_pointer.offset(1);
                if self.is_pushed {
                    concatenate(self.state, 2);
                } else {
                    self.is_pushed = true;
                };
            };
        }
    }
    pub unsafe extern "C" fn add_number(&mut self, number: *mut TValue) {
        unsafe {
            let number_buffer = self.get_raw(44);
            self.size += tostringbuff(number, number_buffer) as usize;
        }
    }
    pub fn add_length(&mut self, length: usize) {
        self.size += length;
    }
}
