pub mod userbox;
use crate::buffer::userbox::*;
use crate::interpreter::*;
use crate::loads::*;
use crate::new::*;
use crate::utility::c::*;
use std::ptr::*;
pub type BufferElement = i8;
impl Buffer {
    pub const INITIAL_SIZE: usize = 1024;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Buffer {
    pub loads: LoadS<BufferElement>,
    pub interpreter: *mut Interpreter,
    pub initial_data: [BufferElement; Buffer::INITIAL_SIZE],
}
impl New for Buffer {
    fn new() -> Self {
        return Buffer {
            loads: LoadS::<BufferElement>::new(),
            interpreter: null_mut(),
            initial_data: [0; Buffer::INITIAL_SIZE],
        };
    }
}
impl Buffer {
    pub unsafe fn initialize_with_size(
        &mut self,
        interpreter: *mut Interpreter,
        size: usize,
    ) -> *mut BufferElement {
        unsafe {
            self.initialize(interpreter);
            return self.prepare_with_size_and_index(size, -1);
        }
    }
    pub unsafe fn push_result_with_size(&mut self, size: usize) {
        unsafe {
            self.loads.add_length(size);
            self.push_result();
        }
    }
    pub unsafe fn new_with_size(&mut self, size: usize) -> usize {
        unsafe {
            let mut new_size = 2 * self.loads.get_size();
            if (!0usize).wrapping_sub(size) < self.loads.get_length() as usize {
                return lual_error(self.interpreter, b"buffer too large\0".as_ptr()) as usize;
            }
            new_size = new_size.max(self.loads.get_length() + size as i32);
            return new_size as usize;
        }
    }
    pub unsafe fn prepare_with_size_and_index(
        &mut self,
        size: usize,
        boxidx: i32,
    ) -> *mut BufferElement {
        unsafe {
            if self.loads.get_size() - self.loads.get_length() >= size as i32 {
                return self
                    .loads
                    .loads_pointer
                    .offset(self.loads.get_length() as isize);
            } else {
                let interpreter: *mut Interpreter = self.interpreter;
                let new_pointer: *mut BufferElement;
                let new_size = self.new_with_size(size);
                if self.loads.loads_pointer != (self.initial_data).as_mut_ptr() {
                    new_pointer = UserBox::resize_userbox(interpreter, boxidx, new_size)
                        as *mut BufferElement;
                } else {
                    lua_rotate(interpreter, boxidx, -1);
                    lua_settop(interpreter, -1 - 1);
                    UserBox::new_userbox(interpreter);
                    lua_rotate(interpreter, boxidx, 1);
                    lua_toclose(interpreter, boxidx);
                    new_pointer = UserBox::resize_userbox(interpreter, boxidx, new_size)
                        as *mut BufferElement;
                    memcpy(
                        new_pointer as *mut libc::c_void,
                        self.loads.loads_pointer as *const libc::c_void,
                        (self.loads.get_length() as usize).wrapping_mul(size_of::<BufferElement>()),
                    );
                }
                self.loads.loads_pointer = new_pointer;
                self.loads.loads_size = new_size as i32;
                return new_pointer.offset(self.loads.get_length() as isize);
            };
        }
    }
    pub unsafe fn prepare_with_size(&mut self, size: usize) -> *mut BufferElement {
        unsafe {
            return self.prepare_with_size_and_index(size, -1);
        }
    }
    pub unsafe fn add_string_with_length(&mut self, s: *const BufferElement, length: usize) {
        unsafe {
            if length > 0 {
                let raw: *mut BufferElement = self.prepare_with_size_and_index(length, -1);
                memcpy(
                    raw as *mut libc::c_void,
                    s as *const libc::c_void,
                    length.wrapping_mul(size_of::<BufferElement>()),
                );
                self.loads.add_length(length);
            }
        }
    }
    pub unsafe fn add_string(&mut self, s: *const BufferElement) {
        unsafe {
            self.add_string_with_length(s, strlen(s) as usize);
        }
    }
    pub unsafe fn push_result(&mut self) {
        unsafe {
            let interpreter: *mut Interpreter = self.interpreter;
            lua_pushlstring(
                interpreter,
                self.loads.loads_pointer,
                self.loads.get_length() as usize,
            );
            if self.loads.loads_pointer != (self.initial_data).as_mut_ptr() {
                lua_closeslot(interpreter, -2);
            }
            lua_rotate(interpreter, -2, -1);
            lua_settop(interpreter, -1 - 1);
        }
    }
    pub unsafe fn add_value(&mut self) {
        unsafe {
            let interpreter: *mut Interpreter = self.interpreter;
            let mut length: usize = 0;
            let s: *const BufferElement = lua_tolstring(interpreter, -1, &mut length);
            let b: *mut BufferElement = self.prepare_with_size_and_index(length as usize, -2);
            memcpy(
                b as *mut libc::c_void,
                s as *const libc::c_void,
                (length as usize).wrapping_mul(size_of::<BufferElement>()),
            );
            self.loads.add_length(length);
            lua_settop(interpreter, -1 - 1);
        }
    }
    pub unsafe fn initialize(&mut self, interpreter: *mut Interpreter) {
        unsafe {
            self.interpreter = interpreter;
            self.loads
                .inject(self.initial_data.as_mut_ptr(), Buffer::INITIAL_SIZE);
            lua_pushlightuserdata(interpreter, self as *mut Buffer as *mut libc::c_void);
        }
    }
}
