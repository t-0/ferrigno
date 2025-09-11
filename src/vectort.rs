use std::ptr::*;
use crate::interpreter::*;
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct VectorT<T> {
    pub pointer: *mut T,
    pub length: i32,
    pub size: i32,
}
impl <T> VectorT<T> {
    pub fn initialize(&mut self) {
        self.pointer = null_mut();
        self.size = 0;
    }
    pub unsafe fn shrink (&mut self, interpreter: &mut Interpreter, length: usize) { unsafe {
        let old_total = self.size as usize * ::core::mem::size_of::<T>();
        let new_total = length * ::core::mem::size_of::<T>();
        self.pointer = luam_saferealloc_(interpreter, self.pointer as *mut libc::c_void, old_total, new_total) as *mut T;
        self.size = length as i32;
    } }
    pub unsafe fn at (&self, index: isize) -> *const T { unsafe {
        return self.pointer.offset(index);
    } }
    pub unsafe fn at_mut (&mut self, index: isize) -> * mut T { unsafe {
        return self.pointer.offset(index);
    } }
}
