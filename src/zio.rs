use crate::types::*;
use crate::functions::*;
use crate::interpreter::*;
use std::ptr::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZIO {
    m_length: usize,
    m_pointer: *const i8,
    m_reader: Reader,
    m_data: *mut libc::c_void,
    m_interpreter: *mut Interpreter,
}
impl ZIO {
    pub fn get_char(&mut self) -> CharacterUnderlyingType {
        unsafe {
            if self.m_length > 0 {
                self.m_length = self.m_length.wrapping_sub(1);
                let ret = *self.m_pointer as u8 as CharacterUnderlyingType;
                self.m_pointer = self.m_pointer.offset(1);
                return ret;
            } else {
                self.m_length = self.m_length.wrapping_sub(1);
                return self.luaz_fill();
            };
        }
    }
    pub fn new(interpreter: *mut Interpreter, reader: Reader, data: *mut libc::c_void) -> ZIO {
        return ZIO {
            m_interpreter: interpreter,
            m_reader: reader,
            m_data: data,
            m_length: 0,
            m_pointer: null(),
        };
    }
    pub unsafe fn load_byte(&mut self) -> Option<u8> {
        unsafe {
            let length = self.m_length;
            self.m_length = (self.m_length).wrapping_sub(1);
            let ret: i32 = if length > 0 {
                let ret = self.m_pointer;
                self.m_pointer = (self.m_pointer).offset(1);
                *ret as u8 as i32
            } else {
                self.luaz_fill()
            };
            if ret == -1 {
                return None;
            } else {
                return Some(ret as u8);
            }
        }
    }
    unsafe fn luaz_fill(&mut self) -> CharacterUnderlyingType {
        unsafe {
            let mut size: usize = 0;
            let buffer: *const i8 = (self.m_reader).reader_readfunction.expect("non-null function pointer")(
                self.m_interpreter, self.m_data, &mut size,
            );
            if buffer.is_null() || size == 0 {
                return -1;
            } else {
                self.m_length = size.wrapping_sub(1 as usize);
                self.m_pointer = buffer;
                let ret = *self.m_pointer as u8 as CharacterUnderlyingType;
                self.m_pointer = self.m_pointer.offset(1);
                return ret;
            }
        }
    }
    pub unsafe fn luaz_read(&mut self, mut b: *mut libc::c_void, mut n: usize) -> usize {
        unsafe {
            while n != 0 {
                if self.m_length == 0 {
                    if self.luaz_fill() == -1 {
                        return n;
                    } else {
                        self.m_length = (self.m_length).wrapping_add(1);
                        self.m_length;
                        self.m_pointer = (self.m_pointer).offset(-1);
                        self.m_pointer;
                    }
                }
                let m: usize = if n <= self.m_length { n } else { self.m_length };
                libc::memcpy(b, self.m_pointer as *const libc::c_void, m as usize);
                self.m_length = (self.m_length as usize).wrapping_sub(m) as usize;
                self.m_pointer = (self.m_pointer).offset(m as isize);
                b = (b as *mut i8).offset(m as isize) as *mut libc::c_void;
                n = (n as usize).wrapping_sub(m) as usize;
            }
            return 0;
        }
    }
}
