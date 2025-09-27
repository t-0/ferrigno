#![allow(unused)]
use crate::interpreter::*;
use rlua::*;
use std::{mem::*, ptr::*};
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct VectorT<T> {
    pub vectort_pointer: *mut T,
    vectort_length: usize,
    vectort_size: usize,
}
impl<T> VectorT<T> {
    pub fn capitulate(&mut self) -> (*mut T, usize) {
        let ret = (self.vectort_pointer, self.vectort_size as usize);
        self.vectort_pointer = null_mut();
        self.vectort_size = 0;
        self.vectort_length = 0;
        ret
    }
    pub fn inject(&mut self, pointer: *mut T, size: usize) {
        self.vectort_pointer = pointer;
        self.vectort_size = size;
        self.vectort_length = 0;
    }
    pub fn new() -> VectorT<T> {
        VectorT::<T> { vectort_pointer: null_mut(), vectort_length: 0, vectort_size: 0 }
    }
    pub fn add_length(&mut self, length: usize) {
        self.vectort_length += length;
    }
    pub fn subtract_length(&mut self, length: usize) {
        self.vectort_length -= length;
    }
    pub fn set_length(&mut self, length: usize) {
        self.vectort_length = length;
    }
    pub fn zero_length(&mut self) {
        self.vectort_length = 0;
    }
    pub fn initialize(&mut self) {
        self.vectort_pointer = null_mut();
        self.vectort_length = 0;
        self.vectort_size = 0;
    }
    pub unsafe fn shrink(&mut self, interpreter: &mut Interpreter, new_size: usize) {
        unsafe {
            let old_total = self.vectort_size as usize * size_of::<T>();
            let new_total = new_size * size_of::<T>();
            self.vectort_pointer = luam_saferealloc_(interpreter, self.vectort_pointer as *mut libc::c_void, old_total, new_total) as *mut T;
            self.vectort_length = 0;
            self.vectort_size = new_size;
        }
    }
    pub unsafe fn at(&self, index: isize) -> *const T {
        unsafe {
            return self.vectort_pointer.offset(index);
        }
    }
    pub unsafe fn at_mut(&mut self, index: isize) -> *mut T {
        unsafe {
            return self.vectort_pointer.offset(index);
        }
    }
    pub fn get_length(&self) -> usize {
        self.vectort_length
    }
    pub fn get_size(&self) -> usize {
        self.vectort_size as usize
    }
    pub unsafe fn initialize_size(&mut self, interpreter: *mut Interpreter, size: usize) {
        unsafe {
            self.vectort_pointer = luam_malloc_(interpreter, (size as usize) * size_of::<T>()) as *mut T;
            self.vectort_size = size;
        }
    }
    pub unsafe fn destroy(&mut self, interpreter: *mut Interpreter) {
        unsafe {
            luam_saferealloc_(interpreter, self.vectort_pointer as *mut libc::c_void, (self.vectort_size as usize).wrapping_mul(size_of::<i8>()), 0);
            self.vectort_pointer = null_mut();
            self.vectort_size = 0;
        }
    }
    pub unsafe fn grow(&mut self, interpreter: *mut Interpreter, new_length: usize, limit: usize, what: *const i8) {
        unsafe {
            let mut new_size = self.vectort_size;
            if new_length + 1 <= new_size {
                return;
            }
            if new_size >= limit / 2 {
                if new_size >= limit {
                    luag_runerror(interpreter, c"too many %s (limit is %d)".as_ptr(), what, limit);
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
                (self.vectort_size as usize).wrapping_mul(size_of::<T>()),
                (new_size as usize).wrapping_mul(size_of::<T>()),
            ) as *mut T;
            self.vectort_size = new_size;
        }
    }
    pub unsafe fn resize(&mut self, interpreter: *mut Interpreter, new_size: usize) {
        unsafe {
            self.vectort_pointer = luam_saferealloc_(
                interpreter,
                self.vectort_pointer as *mut libc::c_void,
                (self.vectort_size as usize).wrapping_mul(size_of::<T>()),
                new_size.wrapping_mul(size_of::<T>()),
            ) as *mut T;
            self.vectort_size = new_size;
        }
    }
}
