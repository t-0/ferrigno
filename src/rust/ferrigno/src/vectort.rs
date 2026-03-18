#![allow(unused)]
use crate::state::*;
use crate::tdefaultnew::*;
use std::{mem::*, ptr::*};
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct VectorT<T> {
    pub vectort_pointer: *mut T,
    vectort_length: usize,
    vectort_size: usize,
}
impl<T> TDefaultNew for VectorT<T> {
    fn new() -> VectorT<T> {
        VectorT::<T> { vectort_pointer: null_mut(), vectort_length: 0, vectort_size: 0 }
    }
}
impl<T> VectorT<T> {
    pub fn capitulate(&mut self) -> (*mut T, usize) {
        let ret = (self.vectort_pointer, self.vectort_size);
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
    pub unsafe fn shrink(&mut self, state: &mut State, newsize: usize) {
        unsafe {
            let old_total = self.vectort_size * size_of::<T>();
            let new_total = newsize * size_of::<T>();
            self.vectort_pointer =
                (*state).safereallocate(self.vectort_pointer as *mut std::ffi::c_void, old_total, new_total) as *mut T;
            self.vectort_length = 0;
            self.vectort_size = newsize;
        }
    }
    pub unsafe fn at(&self, index: isize) -> *const T {
        unsafe { self.vectort_pointer.add(index as usize) }
    }
    pub unsafe fn at_mut(&mut self, index: isize) -> *mut T {
        unsafe { self.vectort_pointer.add(index as usize) }
    }
    pub fn get_length(&self) -> usize {
        self.vectort_length
    }
    pub fn get_size(&self) -> usize {
        self.vectort_size
    }
    pub fn set_size(&mut self, size: usize) {
        self.vectort_size = size;
    }
    pub unsafe fn initialize_size(&mut self, state: *mut State, size: usize) {
        unsafe {
            self.vectort_pointer = (*state).allocate(size * size_of::<T>()) as *mut T;
            self.vectort_size = size;
        }
    }
    pub unsafe fn destroy(&mut self, state: *mut State) {
        unsafe {
            (*state).safereallocate(self.vectort_pointer as *mut std::ffi::c_void, self.vectort_size, 0);
            self.vectort_pointer = null_mut();
            self.vectort_size = 0;
        }
    }
    pub unsafe fn grow(&mut self, state: *mut State, new_length: usize, limit: usize, what: *const i8) {
        unsafe {
            let mut newsize = self.vectort_size;
            if new_length < newsize {
                return;
            }
            if newsize >= limit / 2 {
                if newsize >= limit {
                    luag_runerror(state, c"too many %s (limit is %d)".as_ptr(), &[what.into(), limit.into()]);
                }
                newsize = limit;
            } else {
                newsize *= 2;
                if newsize < 4 {
                    newsize = 4;
                }
            }
            self.vectort_pointer = (*state).safereallocate(
                self.vectort_pointer as *mut std::ffi::c_void,
                self.vectort_size.wrapping_mul(size_of::<T>()),
                newsize.wrapping_mul(size_of::<T>()),
            ) as *mut T;
            self.vectort_size = newsize;
        }
    }
    pub unsafe fn resize(&mut self, state: *mut State, newsize: usize) {
        unsafe {
            self.vectort_pointer = (*state).safereallocate(
                self.vectort_pointer as *mut std::ffi::c_void,
                self.vectort_size.wrapping_mul(size_of::<T>()),
                newsize.wrapping_mul(size_of::<T>()),
            ) as *mut T;
            self.vectort_size = newsize;
        }
    }
}
