use crate::buffer::*;
use crate::interpreter::*;
use crate::new::*;
use crate::tag::*;
use rlua::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StreamWriter {
    pub is_initialized: bool,
    pub buffer: Buffer,
}
impl StreamWriter {
    pub unsafe fn writer(interpreter: *mut Interpreter, b: *const libc::c_void, size: usize, arbitrary_data: *mut libc::c_void) -> i32 {
        unsafe {
            let stream_writer: *mut StreamWriter = arbitrary_data as *mut StreamWriter;
            if !(*stream_writer).is_initialized {
                (*stream_writer).is_initialized = true;
                (*stream_writer).buffer.initialize(interpreter);
            }
            (*stream_writer).buffer.add_string_with_length(b as *const i8, size as usize);
            return 0;
        }
    }
    pub unsafe fn dump(&mut self, interpreter: *mut Interpreter) -> i32 {
        unsafe {
            let is_strip = 0 != lua_toboolean(interpreter, 2);
            (*interpreter).lual_checktype(1, TagType::Closure);
            lua_settop(interpreter, 1);
            self.is_initialized = false;
            if ((lua_dump(
                interpreter,
                Some(StreamWriter::writer as unsafe fn(*mut Interpreter, *const libc::c_void, usize, *mut libc::c_void) -> i32),
                self as *mut StreamWriter as *mut libc::c_void,
                is_strip,
            ) != 0) as i32
                != 0) as i64
                != 0
            {
                return lual_error(interpreter, make_cstring!("unable to dump given function"));
            }
            self.buffer.push_result();
            return 1;
        }
    }
    pub unsafe fn str_dump(interpreter: *mut Interpreter) -> i32 {
        unsafe {
            let mut stream_writer = StreamWriter { is_initialized: false, buffer: Buffer::new() };
            return stream_writer.dump(interpreter);
        }
    }
}
