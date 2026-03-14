use crate::functions::*;
use crate::state::*;
use crate::types::*;
use std::ptr::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZIO {
    zio_length: usize,
    zio_pointer: *const i8,
    zio_reader: Reader,
    zio_data: *mut std::ffi::c_void,
    zio_interpreter: *mut State,
}
impl ZIO {
    pub fn get_char(&mut self) -> CharacterUnderlyingType {
        unsafe {
            if self.zio_length > 0 {
                self.zio_length -= 1;
                let ret = *self.zio_pointer as u8 as CharacterUnderlyingType;
                self.zio_pointer = self.zio_pointer.add(1);
                ret
            } else {
                self.luaz_fill()
            }
        }
    }
    pub fn peek_char(&self) -> CharacterUnderlyingType {
        if self.zio_length > 0 {
            unsafe { *self.zio_pointer as u8 as CharacterUnderlyingType }
        } else {
            -1
        }
    }
    pub fn new(state: *mut State, reader: Reader, data: *mut std::ffi::c_void) -> ZIO {
        ZIO {
            zio_interpreter: state,
            zio_reader: reader,
            zio_data: data,
            zio_length: 0,
            zio_pointer: null(),
        }
    }
    pub unsafe fn load_byte(&mut self) -> Option<u8> {
        unsafe {
            let ret: i32 = if self.zio_length > 0 {
                self.zio_length -= 1;
                let ret = self.zio_pointer;
                self.zio_pointer = (self.zio_pointer).add(1);
                *ret as u8 as i32
            } else {
                self.luaz_fill()
            };
            if ret == -1 { None } else { Some(ret as u8) }
        }
    }
    unsafe fn luaz_fill(&mut self) -> CharacterUnderlyingType {
        unsafe {
            let mut size: usize = 0;
            let buffer: *const i8 = (self.zio_reader).reader_read_function.expect("non-null function pointer")(
                self.zio_interpreter, self.zio_data, &mut size,
            );
            if buffer.is_null() || size == 0 {
                -1
            } else {
                self.zio_length = size - 1;
                self.zio_pointer = buffer;
                let ret = *self.zio_pointer as u8 as CharacterUnderlyingType;
                self.zio_pointer = self.zio_pointer.add(1);
                ret
            }
        }
    }
    /// Returns a direct pointer into the buffer if enough contiguous bytes are available,
    /// advancing the read position. Returns null if the buffer doesn't have n contiguous bytes.
    pub unsafe fn luaz_getaddr(&mut self, n: usize) -> *const std::ffi::c_void {
        if self.zio_length == 0 {
            return null();
        }
        if self.zio_length < n {
            return null();
        }
        let res = self.zio_pointer as *const std::ffi::c_void;
        self.zio_length -= n;
        unsafe {
            self.zio_pointer = self.zio_pointer.add(n);
        }
        res
    }
    pub unsafe fn luaz_read(&mut self, mut b: *mut std::ffi::c_void, mut n: usize) -> usize {
        unsafe {
            while n != 0 {
                if self.zio_length == 0 {
                    if self.luaz_fill() == -1 {
                        return n;
                    } else {
                        self.zio_length += 1;
                        self.zio_length;
                        self.zio_pointer = (self.zio_pointer).sub(1);
                        self.zio_pointer;
                    }
                }
                let m: usize = if n <= self.zio_length { n } else { self.zio_length };
                std::ptr::copy_nonoverlapping(self.zio_pointer as *const u8, b as *mut u8, m);
                self.zio_length -= m;
                self.zio_pointer = (self.zio_pointer).add(m);
                b = (b as *mut i8).add(m) as *mut std::ffi::c_void;
                n -= m;
            }
            0
        }
    }
}
