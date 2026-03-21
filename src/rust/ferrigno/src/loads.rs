#![allow(unused, dead_code)]
use crate::state::*;
use crate::tdefaultnew::*;
use std::ptr::*;
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct LoadS<T> {
    pub loads_pointer: *mut T,
    loads_length: i32,
    pub loads_size: i32,
}
impl<T> TDefaultNew for LoadS<T> {
    fn new() -> LoadS<T> {
        LoadS::<T> {
            loads_pointer: null_mut(),
            loads_length: 0,
            loads_size: 0,
        }
    }
}
impl<T> LoadS<T> {
    pub fn capitulate(&mut self) -> (*mut T, usize) {
        let ret = (self.loads_pointer, self.loads_size as usize);
        self.loads_pointer = null_mut();
        self.loads_size = 0;
        self.loads_length = 0;
        ret
    }
    pub fn inject(&mut self, pointer: *mut T, size: usize) {
        self.loads_pointer = pointer;
        self.loads_size = size as i32;
        self.loads_length = 0;
    }
    pub fn add_length(&mut self, length: usize) {
        self.loads_length += length as i32;
    }
    pub fn subtract_length(&mut self, length: usize) {
        self.loads_length -= length as i32;
    }
    pub fn set_length(&mut self, length: usize) {
        self.loads_length = length as i32;
    }
    pub fn zero_length(&mut self) {
        self.loads_length = 0;
    }
    pub fn initialize(&mut self) {
        self.loads_pointer = null_mut();
        self.loads_length = 0;
        self.loads_size = 0;
    }
    pub unsafe fn shrink(&mut self, state: &mut State, newsize: usize) {
        unsafe {
            let old_total = self.loads_size as usize * size_of::<T>();
            let new_total = newsize * size_of::<T>();
            self.loads_pointer = (*state).safereallocate(
                self.loads_pointer as *mut std::ffi::c_void,
                old_total,
                new_total,
            ) as *mut T;
            self.loads_length = 0;
            self.loads_size = newsize as i32;
        }
    }
    pub unsafe fn at(&self, index: isize) -> *const T {
        unsafe { self.loads_pointer.add(index as usize) }
    }
    pub unsafe fn at_mut(&mut self, index: isize) -> *mut T {
        unsafe { self.loads_pointer.add(index as usize) }
    }
    pub fn get_length(&self) -> i32 {
        self.loads_length
    }
    pub fn get_size(&self) -> i32 {
        self.loads_size
    }
    pub unsafe fn initialize_size(&mut self, state: *mut State, size: usize) {
        unsafe {
            self.loads_pointer = (*state).allocate(size * size_of::<T>()) as *mut T;
            self.loads_size = size as i32;
        }
    }
    pub unsafe fn destroy(&mut self, state: *mut State) {
        unsafe {
            (*state).safereallocate(
                self.loads_pointer as *mut std::ffi::c_void,
                (self.loads_size as usize),
                0,
            );
            self.loads_pointer = null_mut();
            self.loads_size = 0;
        }
    }
    pub unsafe fn grow(&mut self, state: *mut State, new_length: usize, limit: i32, what: *const i8) {
        unsafe {
            let mut newsize: i32 = self.loads_size;
            if new_length < newsize as usize {
                return;
            }
            if newsize >= limit / 2 {
                if newsize >= limit {
                    luag_runerror(
                        state,
                        c"too many %s (limit is %d)".as_ptr(),
                        &[what.into(), limit.into()],
                    );
                }
                newsize = limit;
            } else {
                newsize *= 2;
                if newsize < 4 {
                    newsize = 4;
                }
            }
            self.loads_pointer = (*state).safereallocate(
                self.loads_pointer as *mut std::ffi::c_void,
                (self.loads_size as usize) * size_of::<T>(),
                (newsize as usize) * size_of::<T>(),
            ) as *mut T;
            self.loads_size = newsize;
        }
    }
    pub unsafe fn resize(&mut self, state: *mut State, newsize: usize) {
        unsafe {
            self.loads_pointer = (*state).safereallocate(
                self.loads_pointer as *mut std::ffi::c_void,
                (self.loads_size as usize) * size_of::<T>(),
                newsize * size_of::<T>(),
            ) as *mut T;
            self.loads_size = newsize as i32;
        }
    }
}
