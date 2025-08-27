use crate::state::*;
use crate::onelua::*;
use crate::tvalue::*;
use crate::c::*;
use crate::tstring::*;
use crate::tag::*;
use crate::gcunion::*;
const BUFFFS_SIZE: usize = 0x100;
pub struct BuffFS {
    state: *mut State,
    is_pushed: bool,
    size: i32,
    block: [i8; BUFFFS_SIZE],
}
impl BuffFS {
    pub fn new(state_: * mut State) -> Self {
        return BuffFS {
            state: state_,
            is_pushed: false,
            size: 0,
            block: [0; BUFFFS_SIZE],
        }
    }
pub unsafe extern "C" fn clear(& mut self) { unsafe {
    let io: *mut TValue = &mut (*(*self.state).top.p).val;
    let ts: *mut TString = luas_newlstr(self.state, self.block.as_mut_ptr(), self.size as u64);
    (*io).value.gc = &mut (*(ts as *mut GCUnion)).gc;
    (*io).tag = ((*ts).tag | TAG_COLLECTABLE) as u8;
    (*self.state).top.p = (*self.state).top.p.offset(1);
    if self.is_pushed {
        luav_concat(self.state, 2);
    } else {
        self.is_pushed = true;
    };
    self.size = 0;
}}
pub unsafe extern "C" fn get_raw(& mut self, size: i32) -> *mut i8 { unsafe {
    if size > 60 as i32 + 44 as i32 + 95 as i32 - self.size {
        self.clear();
    }
    return (self.block)
        .as_mut_ptr()
        .offset(self.size as isize);
}}
pub unsafe extern "C" fn add_string(& mut self, pointer: *const i8, length: u64) { unsafe {
    if length <= (60 as i32 + 44 as i32 + 95 as i32) as u64 {
        let bf: *mut i8 = self.get_raw(length as i32);
        memcpy(bf as *mut libc::c_void, pointer as *const libc::c_void, length);
        self.size += length as i32;
    } else {
        self.clear();
        let io: *mut TValue = &mut (*(*self.state).top.p).val;
        let ts: *mut TString = luas_newlstr(self.state, pointer, length);
        (*io).value.gc = &mut (*(ts as *mut GCUnion)).gc;
        (*io).tag = ((*ts).tag | TAG_COLLECTABLE) as u8;
        (*self.state).top.p = (*self.state).top.p.offset(1);
        if self.is_pushed {
            luav_concat(self.state, 2);
        } else {
            self.is_pushed = true;
        };
    };
}}
pub unsafe extern "C" fn add_number(& mut self, number: *mut TValue) { unsafe {
    let number_buffer: *mut i8 = self.get_raw(44 as i32);
    let length: i32 = tostringbuff(number, number_buffer);
    self.size += length;
}}
pub fn add_length(& mut self, length: i32) {
    self.size += length;
}
}
