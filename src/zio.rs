use crate::functions::*;
use crate::interpreter::*;
use libc::*;
use std::ptr::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZIO {
    pub length: usize,
    pub pointer: *const i8,
    pub reader: ReadFunction,
    pub data: *mut libc::c_void,
    pub zio_interpreter: *mut Interpreter,
}
impl ZIO {
    pub fn new(
        interpreter: *mut Interpreter,
        reader: ReadFunction,
        data: *mut libc::c_void,
    ) -> ZIO {
        return ZIO {
            zio_interpreter: interpreter,
            reader: reader,
            data: data,
            length: 0,
            pointer: null(),
        };
    }
}
pub unsafe fn luaz_fill(zio: *mut ZIO) -> i32 {
    unsafe {
        let mut size: usize = 0;
        let interpreter: *mut Interpreter = (*zio).zio_interpreter;
        let buffer: *const i8 = ((*zio).reader).expect("non-null function pointer")(
            interpreter,
            (*zio).data,
            &mut size,
        );
        if buffer.is_null() || size == 0 {
            return -1;
        } else {
            (*zio).length = size.wrapping_sub(1 as usize);
            (*zio).pointer = buffer;
            let fresh14 = (*zio).pointer;
            (*zio).pointer = ((*zio).pointer).offset(1);
            return *fresh14 as u8 as i32;
        }
    }
}
pub unsafe fn luaz_read(zio: *mut ZIO, mut b: *mut libc::c_void, mut n: usize) -> usize {
    unsafe {
        while n != 0 {
            if (*zio).length == 0 {
                if luaz_fill(zio) == -1 {
                    return n;
                } else {
                    (*zio).length = ((*zio).length).wrapping_add(1);
                    (*zio).length;
                    (*zio).pointer = ((*zio).pointer).offset(-1);
                    (*zio).pointer;
                }
            }
            let m: usize = if n <= (*zio).length { n } else { (*zio).length };
            memcpy(b, (*zio).pointer as *const libc::c_void, m as usize);
            (*zio).length = ((*zio).length as usize).wrapping_sub(m) as usize;
            (*zio).pointer = ((*zio).pointer).offset(m as isize);
            b = (b as *mut i8).offset(m as isize) as *mut libc::c_void;
            n = (n as usize).wrapping_sub(m) as usize;
        }
        return 0usize;
    }
}
