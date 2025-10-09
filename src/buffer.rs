use crate::interpreter::*;
use crate::loads::*;
use crate::tdefaultnew::*;
use crate::userbox::*;
use libc::*;
use std::ptr::*;
pub type BufferElement = i8;
impl Buffer {
    pub const INITIAL_SIZE: usize = 1024;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Buffer {
    pub buffer_loads: LoadS<BufferElement>,
    m_interpreter: *mut Interpreter,
    m_initial_data: [BufferElement; Buffer::INITIAL_SIZE],
}
impl TDefaultNew for Buffer {
    fn new() -> Self {
        Buffer {
            buffer_loads: LoadS::<BufferElement>::new(),
            m_interpreter: null_mut(),
            m_initial_data: [0; Buffer::INITIAL_SIZE],
        }
    }
}
impl Buffer {
    pub unsafe fn initialize_with_size(&mut self, interpreter: *mut Interpreter, size: usize) -> *mut BufferElement {
        unsafe {
            self.initialize(interpreter);
            self.prepare_with_size_and_index(size, -1)
        }
    }
    pub unsafe fn push_result_with_size(&mut self, size: usize) {
        unsafe {
            self.buffer_loads.add_length(size);
            self.push_result();
        }
    }
    pub unsafe fn new_with_size(&mut self, size: usize) -> usize {
        unsafe {
            let mut newsize = 2 * self.buffer_loads.get_size();
            if (!0usize) - size < self.buffer_loads.get_length() as usize {
                lual_error(self.m_interpreter, c"buffer too large".as_ptr()) as usize
            } else {
                newsize = newsize.max(self.buffer_loads.get_length() + size as i32);
                newsize as usize
            }
        }
    }
    pub unsafe fn prepare_with_size_and_index(&mut self, size: usize, boxidx: i32) -> *mut BufferElement {
        unsafe {
            if self.buffer_loads.get_size() - self.buffer_loads.get_length() >= size as i32 {
                return self.buffer_loads.loads_pointer.offset(self.buffer_loads.get_length() as isize);
            } else {
                let interpreter = self.m_interpreter;
                let new_pointer: *mut BufferElement;
                let newsize = self.new_with_size(size);
                if self.buffer_loads.loads_pointer != (self.m_initial_data).as_mut_ptr() {
                    new_pointer = UserBox::resize_userbox(interpreter, boxidx, newsize) as *mut BufferElement;
                } else {
                    lua_rotate(interpreter, boxidx, -1);
                    lua_settop(interpreter, -2);
                    UserBox::new_userbox(interpreter);
                    lua_rotate(interpreter, boxidx, 1);
                    lua_toclose(interpreter, boxidx);
                    new_pointer = UserBox::resize_userbox(interpreter, boxidx, newsize) as *mut BufferElement;
                    libc::memcpy(
                        new_pointer as *mut c_void,
                        self.buffer_loads.loads_pointer as *const c_void,
                        (self.buffer_loads.get_length() as usize) * (size_of::<BufferElement>()),
                    );
                }
                self.buffer_loads.loads_pointer = new_pointer;
                self.buffer_loads.loads_size = newsize as i32;
                return new_pointer.offset(self.buffer_loads.get_length() as isize);
            };
        }
    }
    pub unsafe fn prepare_with_size(&mut self, size: usize) -> *mut BufferElement {
        unsafe { self.prepare_with_size_and_index(size, -1) }
    }
    pub unsafe fn add_string_with_length(&mut self, s: *const BufferElement, length: usize) {
        unsafe {
            if length > 0 {
                let raw: *mut BufferElement = self.prepare_with_size_and_index(length, -1);
                libc::memcpy(raw as *mut c_void, s as *const c_void, length * size_of::<BufferElement>());
                self.buffer_loads.add_length(length);
            }
        }
    }
    pub unsafe fn add_string(&mut self, s: *const BufferElement) {
        unsafe {
            self.add_string_with_length(s, libc::strlen(s) as usize);
        }
    }
    pub unsafe fn push_result(&mut self) {
        unsafe {
            let interpreter = self.m_interpreter;
            lua_pushlstring(
                interpreter,
                self.buffer_loads.loads_pointer,
                self.buffer_loads.get_length() as usize,
            );
            if self.buffer_loads.loads_pointer != (self.m_initial_data).as_mut_ptr() {
                lua_closeslot(interpreter, -2);
            }
            lua_rotate(interpreter, -2, -1);
            lua_settop(interpreter, -2);
        }
    }
    pub unsafe fn add_value(&mut self) {
        unsafe {
            let interpreter = self.m_interpreter;
            let mut length: usize = 0;
            let s: *const BufferElement = lua_tolstring(interpreter, -1, &mut length);
            let b: *mut BufferElement = self.prepare_with_size_and_index(length as usize, -2);
            libc::memcpy(
                b as *mut c_void,
                s as *const c_void,
                (length as usize) * (size_of::<BufferElement>()),
            );
            self.buffer_loads.add_length(length);
            lua_settop(interpreter, -2);
        }
    }
    pub unsafe fn initialize(&mut self, interpreter: *mut Interpreter) {
        unsafe {
            self.m_interpreter = interpreter;
            self.buffer_loads
                .inject(self.m_initial_data.as_mut_ptr(), Buffer::INITIAL_SIZE);
            lua_pushlightuserdata(interpreter, self as *mut Buffer as *mut c_void);
        }
    }
}
