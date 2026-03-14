use crate::loads::*;
use crate::state::{self, *};
use crate::tdefaultnew::*;
use crate::userbox::*;
use std::ptr::*;
type BufferElement = i8;
impl Buffer {
    const INITIAL_SIZE: usize = 1024;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Buffer {
    pub buffer_loads: LoadS<BufferElement>,
    buffer_interpreter: *mut State,
    buffer_initial_data: [BufferElement; Buffer::INITIAL_SIZE],
    buffer_stack_index: i32,
}
impl TDefaultNew for Buffer {
    fn new() -> Self {
        Buffer {
            buffer_loads: LoadS::<BufferElement>::new(),
            buffer_interpreter: null_mut(),
            buffer_initial_data: [0; Buffer::INITIAL_SIZE],
            buffer_stack_index: 0,
        }
    }
}
impl Buffer {
    pub unsafe fn initialize_with_size(&mut self, state: *mut State, size: usize) -> *mut BufferElement {
        unsafe {
            self.initialize(state);
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
            let mut newsize = self.buffer_loads.get_size().wrapping_mul(2);
            if (!0usize) - size < self.buffer_loads.get_length() as usize {
                lual_error(self.buffer_interpreter, c"buffer too large".as_ptr()) as usize
            } else {
                newsize = newsize.max(self.buffer_loads.get_length() + size as i32);
                newsize as usize
            }
        }
    }
    pub unsafe fn prepare_with_size_and_index(&mut self, size: usize, boxidx: i32) -> *mut BufferElement {
        unsafe {
            if self.buffer_loads.get_size() - self.buffer_loads.get_length() >= size as i32 {
                self.buffer_loads.loads_pointer.add(self.buffer_loads.get_length() as usize)
            } else {
                let state = self.buffer_interpreter;
                let new_pointer: *mut BufferElement;
                let newsize = self.new_with_size(size);
                if self.buffer_loads.loads_pointer != (self.buffer_initial_data).as_mut_ptr() {
                    // Buffer already on stack as userbox — resize it
                    let idx = if self.buffer_stack_index != 0 { self.buffer_stack_index } else { boxidx };
                    new_pointer = UserBox::resize_userbox(state, idx, newsize) as *mut BufferElement;
                } else {
                    // First overflow: create userbox on stack
                    UserBox::new_userbox(state);
                    if boxidx < -1 {
                        // Caller has items above the buffer position (e.g. add_value
                        // has a value string at -1). Insert userbox below them so
                        // the caller's lua_settop(-2) pops the value, not the box.
                        lua_rotate(state, boxidx, 1);
                    }
                    let ub_idx = if boxidx < -1 { boxidx } else { -1 };
                    self.buffer_stack_index = state::lua_absindex(state, ub_idx);
                    lua_toclose(state, self.buffer_stack_index);
                    new_pointer = UserBox::resize_userbox(state, self.buffer_stack_index, newsize) as *mut BufferElement;
                    std::ptr::copy_nonoverlapping(
                        self.buffer_loads.loads_pointer as *const u8,
                        new_pointer as *mut u8,
                        (self.buffer_loads.get_length() as usize) * size_of::<BufferElement>(),
                    );
                }
                self.buffer_loads.loads_pointer = new_pointer;
                self.buffer_loads.loads_size = newsize as i32;
                new_pointer.add(self.buffer_loads.get_length() as usize)
            }
        }
    }
    pub unsafe fn prepare_with_size(&mut self, size: usize) -> *mut BufferElement {
        unsafe { self.prepare_with_size_and_index(size, -1) }
    }
    pub unsafe fn add_string_with_length(&mut self, s: *const BufferElement, length: usize) {
        unsafe {
            if length > 0 {
                let idx = if self.buffer_stack_index != 0 { self.buffer_stack_index } else { -1 };
                let raw: *mut BufferElement = self.prepare_with_size_and_index(length, idx);
                std::ptr::copy_nonoverlapping(s as *const u8, raw as *mut u8, length * size_of::<BufferElement>());
                self.buffer_loads.add_length(length);
            }
        }
    }
    pub unsafe fn add_string(&mut self, s: *const BufferElement) {
        unsafe {
            self.add_string_with_length(s, crate::utility::cstr_len(s));
        }
    }
    pub unsafe fn push_result(&mut self) {
        unsafe {
            let state = self.buffer_interpreter;
            lua_pushlstring(state, self.buffer_loads.loads_pointer, self.buffer_loads.get_length() as usize);
            if self.buffer_loads.loads_pointer != (self.buffer_initial_data).as_mut_ptr() && self.buffer_stack_index != 0 {
                // Buffer is on stack as userbox — close it and remove
                let idx = self.buffer_stack_index;
                // Check if the userbox is actually still on the stack at this position
                let top = state::lua_absindex(state, -1);
                if idx <= top {
                    lua_closeslot(state, idx);
                    lua_rotate(state, idx, -1);
                    lua_settop(state, idx);
                }
            }
        }
    }
    pub unsafe fn add_value(&mut self) {
        unsafe {
            let state = self.buffer_interpreter;
            let mut length: usize = 0;
            let s: *const BufferElement = lua_tolstring(state, -1, &mut length);
            let b: *mut BufferElement = self.prepare_with_size_and_index(length, -2);
            std::ptr::copy_nonoverlapping(s as *const u8, b as *mut u8, length * size_of::<BufferElement>());
            self.buffer_loads.add_length(length);
            lua_settop(state, -2);
        }
    }
    pub unsafe fn initialize(&mut self, state: *mut State) {
        self.buffer_interpreter = state;
        self.buffer_loads
            .inject(self.buffer_initial_data.as_mut_ptr(), Buffer::INITIAL_SIZE);
        self.buffer_stack_index = 0;
    }
}
