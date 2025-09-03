pub mod userbox;
use crate::utility::c::*;
use crate::new::*;
use crate::buffer::userbox::*;
use crate::state::*;
impl Buffer {
    pub const INITIAL_SIZE: usize = 1024;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Buffer {
    pub pointer: *mut i8,
    pub size: u64,
    pub length: u64,
    pub state: *mut State,
    pub initial_data: [i8; Buffer::INITIAL_SIZE],
}
impl New for Buffer {
    fn new() -> Self {
        return Buffer {
            pointer: std::ptr::null_mut(),
            size: 0,
            length: 0,
            state: std::ptr::null_mut(),
            initial_data: [0; Buffer::INITIAL_SIZE],
        };
    }
}
impl Buffer {
    pub unsafe fn initialize_with_size(&mut self, state: *mut State, size: u64) -> *mut i8 {
        unsafe {
            self.initialize(state);
            return self.prepare_with_size_and_index(size, -1);
        }
    }
    pub unsafe fn push_result_with_size(&mut self, size: u64) {
        unsafe {
            self.length += size;
            self.push_result();
        }
    }
    pub unsafe fn new_with_size(&mut self, size: u64) -> u64 {
        unsafe {
            let mut new_size = 3 * (self.size / 2);
            if (!0u64).wrapping_sub(size) < self.length {
                return lual_error(self.state, b"buffer too large\0" as *const u8 as *const i8)
                    as u64;
            }
            new_size = new_size.max(self.length + size);
            return new_size;
        }
    }
    pub unsafe fn prepare_with_size_and_index(&mut self, size: u64, boxidx: i32) -> *mut i8 {
        unsafe {
            if (self.size).wrapping_sub(self.length) >= size {
                return (self.pointer).offset(self.length as isize);
            } else {
                let state: *mut State = self.state;
                let newbuff: *mut i8;
                let new_size: u64 = self.new_with_size(size);
                if self.pointer != (self.initial_data).as_mut_ptr() {
                    newbuff = UserBox::resize_userbox(state, boxidx, new_size) as *mut i8;
                } else {
                    lua_rotate(state, boxidx, -1);
                    lua_settop(state, -1 - 1);
                    UserBox::new_userbox(state);
                    lua_rotate(state, boxidx, 1);
                    lua_toclose(state, boxidx);
                    newbuff = UserBox::resize_userbox(state, boxidx, new_size) as *mut i8;
                    memcpy(
                        newbuff as *mut libc::c_void,
                        self.pointer as *const libc::c_void,
                        self.length.wrapping_mul(::core::mem::size_of::<i8>() as u64),
                    );
                }
                self.pointer = newbuff;
                self.size = new_size;
                return newbuff.offset(self.length as isize);
            };
        }
    }
    pub unsafe fn prepare_with_size(&mut self, size: u64) -> *mut i8 {
        unsafe {
            return self.prepare_with_size_and_index(size, -1);
        }
    }
    pub unsafe fn add_string_with_length(&mut self, s: *const i8, length: u64) {
        unsafe {
            if length > 0 {
                let raw: *mut i8 = self.prepare_with_size_and_index(length, -1);
                memcpy(
                    raw as *mut libc::c_void,
                    s as *const libc::c_void,
                    length.wrapping_mul(::core::mem::size_of::<i8>() as u64),
                );
                self.length = (self.length as u64).wrapping_add(length) as u64;
            }
        }
    }
    pub unsafe fn add_string(&mut self, s: *const i8) {
        unsafe {
            self.add_string_with_length(s, strlen(s));
        }
    }
    pub unsafe fn push_result(&mut self) {
        unsafe {
            let state: *mut State = self.state;
            lua_pushlstring(state, self.pointer, self.length);
            if self.pointer != (self.initial_data).as_mut_ptr() {
                lua_closeslot(state, -2);
            }
            lua_rotate(state, -2, -1);
            lua_settop(state, -1 - 1);
        }
    }
    pub unsafe fn add_value(&mut self) {
        unsafe {
            let state: *mut State = self.state;
            let mut length: u64 = 0;
            let s: *const i8 = lua_tolstring(state, -1, &mut length);
            let b: *mut i8 = self.prepare_with_size_and_index(length, -2);
            memcpy(
                b as *mut libc::c_void,
                s as *const libc::c_void,
                length.wrapping_mul(::core::mem::size_of::<i8>() as u64),
            );
            self.length = (self.length as u64).wrapping_add(length) as u64;
            lua_settop(state, -1 - 1);
        }
    }
    pub unsafe fn initialize(&mut self, state: *mut State) {
        unsafe {
            self.state = state;
            self.pointer = self.initial_data.as_mut_ptr();
            self.length = 0;
            self.size = 16usize
                .wrapping_mul(::core::mem::size_of::<*mut libc::c_void>())
                .wrapping_mul(::core::mem::size_of::<f64>())
                as u64;
            lua_pushlightuserdata(state, self as *mut Buffer as *mut libc::c_void);
        }
    }
}
