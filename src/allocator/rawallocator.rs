#![allow(dead_code)]
use crate::allocator::*;
use libc::{free, malloc, realloc};
pub struct RawAllocator {}
impl RawAllocator {
    pub fn new() -> Self {
        return RawAllocator {};
    }
}
impl Allocator for RawAllocator {
    unsafe fn allocate(&mut self, new_size: usize) -> *mut libc::c_void {
        unsafe {
            return malloc(new_size);
        }
    }
    unsafe fn free(&mut self, pointer: *mut libc::c_void, _old_size: usize) {
        unsafe {
            free(pointer);
        }
    }
    unsafe fn reallocate(
        &mut self,
        pointer: *mut libc::c_void,
        _old_size: usize,
        new_size: usize,
    ) -> *mut libc::c_void {
        unsafe {
            if 0 == new_size {
                free(pointer);
                return std::ptr::null_mut();
            } else {
                return realloc(pointer, new_size);
            };
        }
    }
}
