use crate::functions::*;
use crate::interpreter::*;
use libc::*;
use std::ptr::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZIO {
    length: usize,
    pointer: *const i8,
    reader: Reader,
    data: *mut libc::c_void,
    interpreter: *mut Interpreter,
}
impl ZIO {
    pub fn get_char(&mut self) -> i32 { unsafe {
        if self.length > 0 {
            self.length = self.length.wrapping_sub(1);
            let ret =  *self.pointer as u8 as i32;
            self.pointer = self.pointer.offset(1);
            return ret;
        } else {
            self.length = self.length.wrapping_sub(1);
            return self.luaz_fill()
        };
    } }
    pub fn new(interpreter: *mut Interpreter, reader: Reader, data: *mut libc::c_void) -> ZIO {
        return ZIO { interpreter: interpreter, reader: reader, data: data, length: 0, pointer: null() };
    }
    pub unsafe fn load_byte(&mut self) -> Option<u8> {
        unsafe {
            let length = self.length;
            self.length = (self.length).wrapping_sub(1);
            let ret: i32 = if length > 0 {
                let ret = self.pointer;
                self.pointer = (self.pointer).offset(1);
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
    unsafe fn luaz_fill(& mut self) -> i32 {
        unsafe {
            let mut size: usize = 0;
            let buffer: *const i8 = (self.reader).reader_readfunction.expect("non-null function pointer")(self.interpreter, self.data, &mut size);
            if buffer.is_null() || size == 0 {
                return -1;
            } else {
                self.length = size.wrapping_sub(1 as usize);
                self.pointer = buffer;
                let ret = *self.pointer as u8 as i32;
                self.pointer = self.pointer.offset(1);
                return ret;
            }
        }
    }
    pub unsafe fn luaz_read(& mut self, mut b: *mut libc::c_void, mut n: usize) -> usize {
        unsafe {
            while n != 0 {
                if self.length == 0 {
                    if self.luaz_fill() == -1 {
                        return n;
                    } else {
                        self.length = (self.length).wrapping_add(1);
                        self.length;
                        self.pointer = (self.pointer).offset(-1);
                        self.pointer;
                    }
                }
                let m: usize = if n <= self.length { n } else { self.length };
                memcpy(b, self.pointer as *const libc::c_void, m as usize);
                self.length = (self.length as usize).wrapping_sub(m) as usize;
                self.pointer = (self.pointer).offset(m as isize);
                b = (b as *mut i8).offset(m as isize) as *mut libc::c_void;
                n = (n as usize).wrapping_sub(m) as usize;
            }
            return 0;
        }
    }
}
