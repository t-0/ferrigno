pub mod userbox;
use crate::utility::c::*;
use crate::new::*;
use crate::buffer::userbox::*;
use crate::interpreter::*;
impl Buffer {
    pub const INITIAL_SIZE: usize = 1024;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Buffer {
    pub pointer: *mut i8,
    pub size: usize,
    pub length: usize,
    pub interpreter: *mut Interpreter,
    pub initial_data: [i8; Buffer::INITIAL_SIZE],
}
impl New for Buffer {
    fn new() -> Self {
        return Buffer {
            pointer: std::ptr::null_mut(),
            size: 0,
            length: 0,
            interpreter: std::ptr::null_mut(),
            initial_data: [0; Buffer::INITIAL_SIZE],
        };
    }
}
impl Buffer {
    pub unsafe fn initialize_with_size(&mut self, interpreter: *mut Interpreter, size: usize) -> *mut i8 {
        unsafe {
            self.initialize(interpreter);
            return self.prepare_with_size_and_index(size, -1);
        }
    }
    pub unsafe fn push_result_with_size(&mut self, size: usize) {
        unsafe {
            self.length += size;
            self.push_result();
        }
    }
    pub unsafe fn new_with_size(&mut self, size: usize) -> usize {
        unsafe {
            let mut new_size = 3 * (self.size / 2);
            if (!0usize).wrapping_sub(size) < self.length {
                return lual_error(self.interpreter, b"buffer too large\0" as *const u8 as *const i8) as usize;
            }
            new_size = new_size.max(self.length + size);
            return new_size;
        }
    }
    pub unsafe fn prepare_with_size_and_index(&mut self, size: usize, boxidx: i32) -> *mut i8 {
        unsafe {
            if self.size - self.length >= size {
                return self.pointer.offset(self.length as isize);
            } else {
                let interpreter: *mut Interpreter = self.interpreter;
                let new_pointer: *mut i8;
                let new_size = self.new_with_size(size);
                if self.pointer != (self.initial_data).as_mut_ptr() {
                    new_pointer = UserBox::resize_userbox(interpreter, boxidx, new_size) as *mut i8;
                } else {
                    lua_rotate(interpreter, boxidx, -1);
                    lua_settop(interpreter, -1 - 1);
                    UserBox::new_userbox(interpreter);
                    lua_rotate(interpreter, boxidx, 1);
                    lua_toclose(interpreter, boxidx);
                    new_pointer = UserBox::resize_userbox(interpreter, boxidx, new_size) as *mut i8;
                    memcpy(
                        new_pointer as *mut libc::c_void,
                        self.pointer as *const libc::c_void,
                        self.length.wrapping_mul(::core::mem::size_of::<i8>()) as u64,
                    );
                }
                self.pointer = new_pointer;
                self.size = new_size;
                return new_pointer.offset(self.length as isize);
            };
        }
    }
    pub unsafe fn prepare_with_size(&mut self, size: usize) -> *mut i8 {
        unsafe {
            return self.prepare_with_size_and_index(size, -1);
        }
    }
    pub unsafe fn add_string_with_length(&mut self, s: *const i8, length: usize) {
        unsafe {
            if length > 0 {
                let raw: *mut i8 = self.prepare_with_size_and_index(length, -1);
                memcpy(
                    raw as *mut libc::c_void,
                    s as *const libc::c_void,
                    length.wrapping_mul(::core::mem::size_of::<i8>()) as u64,
                );
                self.length = self.length.wrapping_add(length as usize);
            }
        }
    }
    pub unsafe fn add_string(&mut self, s: *const i8) {
        unsafe {
            self.add_string_with_length(s, strlen(s) as usize);
        }
    }
    pub unsafe fn push_result(&mut self) {
        unsafe {
            let interpreter: *mut Interpreter = self.interpreter;
            lua_pushlstring(interpreter, self.pointer, self.length as u64);
            if self.pointer != (self.initial_data).as_mut_ptr() {
                lua_closeslot(interpreter, -2);
            }
            lua_rotate(interpreter, -2, -1);
            lua_settop(interpreter, -1 - 1);
        }
    }
    pub unsafe fn add_value(&mut self) {
        unsafe {
            let interpreter: *mut Interpreter = self.interpreter;
            let mut length: u64 = 0;
            let s: *const i8 = lua_tolstring(interpreter, -1, &mut length);
            let b: *mut i8 = self.prepare_with_size_and_index(length as usize, -2);
            memcpy(
                b as *mut libc::c_void,
                s as *const libc::c_void,
                length.wrapping_mul(::core::mem::size_of::<i8>() as u64),
            );
            self.length = self.length.wrapping_add(length as usize);
            lua_settop(interpreter, -1 - 1);
        }
    }
    pub unsafe fn initialize(&mut self, interpreter: *mut Interpreter) {
        unsafe {
            self.interpreter = interpreter;
            self.pointer = self.initial_data.as_mut_ptr();
            self.length = 0;
            self.size = 16usize
                .wrapping_mul(::core::mem::size_of::<*mut libc::c_void>())
                .wrapping_mul(::core::mem::size_of::<f64>());
            lua_pushlightuserdata(interpreter, self as *mut Buffer as *mut libc::c_void);
        }
    }
}
