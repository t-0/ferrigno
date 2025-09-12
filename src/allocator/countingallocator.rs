#![allow(dead_code)]
use crate::allocator::*;
use libc::{free, malloc, realloc};
use std::ptr::*;
pub struct CountingAllocator {
    allocated: usize = 0,
}
impl CountingAllocator {
    pub fn new() -> Self {
        return CountingAllocator { allocated: 0 };
    }
}
impl Allocator for CountingAllocator {
    unsafe fn allocate(&mut self, new_size: usize) -> *mut libc::c_void {
        unsafe {
            if 0 == new_size {
                return null_mut();
            } else {
                let ret = malloc(new_size);
                if !ret.is_null() {
                    self.allocated += new_size;
                }
                return ret;
            }
        }
    }
    unsafe fn free(&mut self, pointer: *mut libc::c_void, old_size: usize) {
        unsafe {
            if !pointer.is_null() {
                free(pointer);
                self.allocated -= old_size;
            }
        }
    }
    unsafe fn reallocate(
        &mut self,
        pointer: *mut libc::c_void,
        old_size: usize,
        new_size: usize,
    ) -> *mut libc::c_void {
        unsafe {
            if 0 == new_size {
                self.free(pointer, old_size);
                return null_mut();
            } else if pointer.is_null() {
                if 0 == old_size {
                    return self.allocate(new_size);
                } else {
                    return null_mut();
                }
            } else {
                let ret = realloc(pointer, new_size);
                if ret.is_null() {
                    return null_mut();
                } else {
                    self.allocated -= old_size;
                    self.allocated += new_size;
                    return ret;
                }
            }
        }
    }
}
