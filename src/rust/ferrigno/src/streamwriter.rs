use crate::buffer::*;
use crate::dumpstate::*;
use crate::state::*;
use crate::tagtype::*;
use crate::tdefaultnew::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StreamWriter {
    pub streamwriter_is_initialized: bool,
    pub streamwriter_buffer: Buffer,
}
impl StreamWriter {
    pub unsafe fn writer(state: *mut State, b: *const std::ffi::c_void, size: usize, arbitrary_data: *mut std::ffi::c_void) -> i32 {
        unsafe {
            let stream_writer: *mut StreamWriter = arbitrary_data as *mut StreamWriter;
            if !(*stream_writer).streamwriter_is_initialized {
                (*stream_writer).streamwriter_is_initialized = true;
                (*stream_writer).streamwriter_buffer.initialize(state);
            }
            (*stream_writer)
                .streamwriter_buffer
                .add_string_with_length(b as *const i8, size);
            0
        }
    }
    pub unsafe fn dump(&mut self, state: *mut State) -> i32 {
        unsafe {
            let is_strip = lua_toboolean(state, 2);
            (*state).lual_checktype(1, TagType::Closure);
            lua_settop(state, 1);
            self.streamwriter_is_initialized = false;
            if lua_dump(
                state,
                Some(StreamWriter::writer as unsafe fn(*mut State, *const std::ffi::c_void, usize, *mut std::ffi::c_void) -> i32),
                self as *mut StreamWriter as *mut std::ffi::c_void,
                is_strip,
            ) != 0
            {
                return lual_error(state, c"unable to dump given function".as_ptr());
            }
            self.streamwriter_buffer.push_result();
            1
        }
    }
    pub unsafe fn str_dump(state: *mut State) -> i32 {
        unsafe {
            let mut stream_writer = StreamWriter { streamwriter_is_initialized: false, streamwriter_buffer: Buffer::new() };
            stream_writer.dump(state)
        }
    }
}
