#![allow(dead_code)]
mod countingallocator;
mod rawallocator;
use libc::*;
pub trait Allocator {
    unsafe fn allocate(&mut self, new_size: usize) -> *mut c_void;
    unsafe fn free(&mut self, pointer: *mut c_void, _old_size: usize);
    unsafe fn reallocate(&mut self, pointer: *mut c_void, old_size: usize, new_size: usize) -> *mut c_void;
}
