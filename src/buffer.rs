pub mod userbox;
use crate::utility::c::*;
use crate::new::*;
use std::ptr::*;
use crate::buffer::userbox::*;
use crate::interpreter::*;
use crate::vectort::*;
pub type BufferElement = i8;
impl Buffer {
    pub const INITIAL_SIZE: usize = 1024;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Buffer {
    pub vector: VectorT<BufferElement>,
    pub interpreter: *mut Interpreter,
    pub initial_data: [BufferElement; Buffer::INITIAL_SIZE],
}
impl New for Buffer {
    fn new() -> Self {
        return Buffer {
            vector: VectorT::<BufferElement> {
                pointer: null_mut(),
                size: 0,
                length: 0,
            },
            interpreter: null_mut(),
            initial_data: [0; Buffer::INITIAL_SIZE],
        };
    }
}
impl Buffer {
    pub unsafe fn initialize_with_size(&mut self, interpreter: *mut Interpreter, size: usize) -> *mut BufferElement {
        unsafe {
            self.initialize(interpreter);
            return self.prepare_with_size_and_index(size, -1);
        }
    }
    pub unsafe fn push_result_with_size(&mut self, size: usize) {
        unsafe {
            self.vector.length += size as i32;
            self.push_result();
        }
    }
    pub unsafe fn new_with_size(&mut self, size: usize) -> usize {
        unsafe {
            let mut new_size = 2 * self.vector.size;
            if (!0usize).wrapping_sub(size) < self.vector.length as usize{
                return lual_error(self.interpreter, b"buffer too large\0".as_ptr()) as usize;
            }
            new_size = new_size.max(self.vector.length + size as i32);
            return new_size as usize;
        }
    }
    pub unsafe fn prepare_with_size_and_index(&mut self, size: usize, boxidx: i32) -> *mut BufferElement {
        unsafe {
            if self.vector.size - self.vector.length >= size as i32 {
                return self.vector.pointer.offset(self.vector.length as isize);
            } else {
                let interpreter: *mut Interpreter = self.interpreter;
                let new_pointer: *mut BufferElement;
                let new_size = self.new_with_size(size);
                if self.vector.pointer != (self.initial_data).as_mut_ptr() {
                    new_pointer = UserBox::resize_userbox(interpreter, boxidx, new_size) as *mut BufferElement;
                } else {
                    lua_rotate(interpreter, boxidx, -1);
                    lua_settop(interpreter, -1 - 1);
                    UserBox::new_userbox(interpreter);
                    lua_rotate(interpreter, boxidx, 1);
                    lua_toclose(interpreter, boxidx);
                    new_pointer = UserBox::resize_userbox(interpreter, boxidx, new_size) as *mut BufferElement;
                    memcpy(
                        new_pointer as *mut libc::c_void,
                        self.vector.pointer as *const libc::c_void,
                        (self.vector.length as usize).wrapping_mul(::core::mem::size_of::<BufferElement>()),
                    );
                }
                self.vector.pointer = new_pointer;
                self.vector.size = new_size as i32;
                return new_pointer.offset(self.vector.length as isize);
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
                    length.wrapping_mul(::core::mem::size_of::<BufferElement>()),
                );
                self.vector.length += length as i32;
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
            lua_pushlstring(interpreter, self.vector.pointer, self.vector.length as usize);
            if self.vector.pointer != (self.initial_data).as_mut_ptr() {
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
                (length as usize).wrapping_mul(::core::mem::size_of::<BufferElement>()),
            );
            self.vector.length += length as i32;
            lua_settop(interpreter, -1 - 1);
        }
    }
    pub unsafe fn initialize(&mut self, interpreter: *mut Interpreter) {
        unsafe {
            self.interpreter = interpreter;
            self.vector.pointer = self.initial_data.as_mut_ptr();
            self.vector.length = 0;
            self.vector.size = 16usize
                .wrapping_mul(::core::mem::size_of::<*mut libc::c_void>())
                .wrapping_mul(::core::mem::size_of::<f64>()) as i32;
            lua_pushlightuserdata(interpreter, self as *mut Buffer as *mut libc::c_void);
        }
    }
}
