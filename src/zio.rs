use crate::functions::*;
use crate::interpreter::*;
use libc::*;
use std::ptr::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZIO {
    pub zio_length: usize,
    pub zio_pointer: *const i8,
    zio_reader: Reader,
    zio_data: *mut libc::c_void,
    zio_interpreter: *mut Interpreter,
}
impl ZIO {
    pub fn get_char(&mut self) -> i32 { unsafe {
        let length = self.zio_length;
        self.zio_length = self.zio_length.wrapping_sub(1);
        if length > 0 {
            let ret =  *self.zio_pointer as u8 as i32;
            self.zio_pointer = (self.zio_pointer).offset(1);
            return ret;
        } else {
            return self.luaz_fill()
        };
    } }
    pub fn new(interpreter: *mut Interpreter, reader: Reader, data: *mut libc::c_void) -> ZIO {
        return ZIO { zio_interpreter: interpreter, zio_reader: reader, zio_data: data, zio_length: 0, zio_pointer: null() };
    }
    pub unsafe fn load_byte(&mut self) -> Option<u8> {
        unsafe {
            let length = self.zio_length;
            self.zio_length = (self.zio_length).wrapping_sub(1);
            let ret: i32 = if length > 0 {
                let ret = self.zio_pointer;
                self.zio_pointer = (self.zio_pointer).offset(1);
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
    pub unsafe fn luaz_fill(& mut self) -> i32 {
        unsafe {
            let mut size: usize = 0;
            let interpreter: *mut Interpreter = self.zio_interpreter;
            let buffer: *const i8 = (self.zio_reader).reader_readfunction.expect("non-null function pointer")(interpreter, self.zio_data, &mut size);
            if buffer.is_null() || size == 0 {
                return -1;
            } else {
                self.zio_length = size.wrapping_sub(1 as usize);
                self.zio_pointer = buffer;
                let fresh14 = self.zio_pointer;
                self.zio_pointer = (self.zio_pointer).offset(1);
                return *fresh14 as u8 as i32;
            }
        }
    }
}
pub unsafe fn luaz_read(zio: *mut ZIO, mut b: *mut libc::c_void, mut n: usize) -> usize {
    unsafe {
        while n != 0 {
            if (*zio).zio_length == 0 {
                if (*zio).luaz_fill() == -1 {
                    return n;
                } else {
                    (*zio).zio_length = ((*zio).zio_length).wrapping_add(1);
                    (*zio).zio_length;
                    (*zio).zio_pointer = ((*zio).zio_pointer).offset(-1);
                    (*zio).zio_pointer;
                }
            }
            let m: usize = if n <= (*zio).zio_length { n } else { (*zio).zio_length };
            memcpy(b, (*zio).zio_pointer as *const libc::c_void, m as usize);
            (*zio).zio_length = ((*zio).zio_length as usize).wrapping_sub(m) as usize;
            (*zio).zio_pointer = ((*zio).zio_pointer).offset(m as isize);
            b = (b as *mut i8).offset(m as isize) as *mut libc::c_void;
            n = (n as usize).wrapping_sub(m) as usize;
        }
        return 0usize;
    }
}
