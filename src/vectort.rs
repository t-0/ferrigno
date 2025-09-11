use std::ptr::*;
use crate::interpreter::*;
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct VectorT<T> {
    pub vectort_pointer: *mut T,
    pub vectort_length: i32,
    pub vectort_size: i32,
}
impl <T> VectorT<T> {
    pub fn new() -> VectorT::<T> {
        VectorT::<T> {
            vectort_pointer: null_mut(),
            vectort_length: 0,
            vectort_size: 0,
        }
    }
    pub fn add_length(& mut self, length: usize) {
        self.vectort_length += length as i32;
    }
    pub fn subtract_length(& mut self, length: usize) {
        self.vectort_length -= length as i32;
    }
    pub fn set_length(& mut self, length: usize) {
        self.vectort_length = length as i32;
    }
    pub fn zero_length(& mut self) {
        self.vectort_length  = 0;
    }
    pub fn initialize(&mut self) {
        self.vectort_pointer = null_mut();
        self.vectort_length = 0;
        self.vectort_size = 0;
    }
    pub unsafe fn shrink (&mut self, interpreter: &mut Interpreter, new_size: usize) { unsafe {
        let old_total = self.vectort_size as usize * size_of::<T>();
        let new_total = new_size * size_of::<T>();
        self.vectort_pointer = luam_saferealloc_(interpreter, self.vectort_pointer as *mut libc::c_void, old_total, new_total) as *mut T;
        self.vectort_length = 0;
        self.vectort_size = new_size as i32;
    } }
    pub unsafe fn at (&self, index: isize) -> *const T { unsafe {
        return self.vectort_pointer.offset(index);
    } }
    pub unsafe fn at_mut (&mut self, index: isize) -> * mut T { unsafe {
        return self.vectort_pointer.offset(index);
    } }
    pub fn get_length(&self) -> i32 {
        self.vectort_length
    }
    pub fn get_size(&self) -> i32 {
        self.vectort_size
    }
    pub unsafe fn initialize_size (&mut self, interpreter: *mut Interpreter, size: usize) { unsafe {
        self.vectort_pointer = luam_malloc_(interpreter, (size as usize) * size_of::<T>()) as *mut T;
        self.vectort_size = size as i32;
    } }
    pub unsafe fn destroy (& mut self, interpreter: *mut Interpreter) { unsafe {
        luam_saferealloc_(
            interpreter,
            self.vectort_pointer as *mut libc::c_void,
            (self.vectort_size as usize).wrapping_mul(size_of::<i8>()),
            0,
        );
        self.vectort_pointer = null_mut();
        self.vectort_size = 0;
    } }
    pub unsafe extern "C" fn grow(&mut self,
        interpreter: *mut Interpreter,
        new_length: usize,
        limit: i32,
        what: *const i8,
    ) {
        unsafe {
            let mut new_size: i32 = self.vectort_size;
            if new_length + 1 <= new_size as usize {
                return;
            }
            if new_size >= limit / 2 {
                if new_size >= limit {
                    luag_runerror(
                        interpreter,
                        b"too many %s (limit is %d)\0" as *const u8 as *const i8,
                        what,
                        limit,
                    );
                }
                new_size = limit;
            } else {
                new_size *= 2;
                if new_size < 4 {
                    new_size = 4;
                }
            }
            self.vectort_pointer = luam_saferealloc_(
                interpreter,
                self.vectort_pointer as *mut libc::c_void,
                (self.vectort_size as usize).wrapping_mul(size_of::<T> ()),
                (new_size as usize).wrapping_mul(size_of::<T> ()),
            ) as *mut T;
            self.vectort_size = new_size;
        }
    }
    pub unsafe fn resize(&mut self, interpreter: *mut Interpreter, new_size: usize) { unsafe {
        self.vectort_pointer = luam_saferealloc_(
            interpreter,
            self.vectort_pointer as *mut libc::c_void,
            (self.vectort_size as usize).wrapping_mul(size_of::<T>()),
            new_size.wrapping_mul(size_of::<T>()),
        ) as *mut T;
        self.vectort_size = new_size as i32;
    } }
}
