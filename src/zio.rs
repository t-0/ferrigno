use crate::functions::*;
use crate::state::*;
use libc::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZIO {
    pub length: u64,
    pub pointer: *const i8,
    pub reader: ReadFunction,
    pub data: *mut libc::c_void,
    pub state: *mut State,
}
impl ZIO {
    pub fn new(
        state: *mut State,
        reader: ReadFunction,
        data: *mut libc::c_void,
    ) -> ZIO {
        return ZIO {
            state: state,
            reader: reader,
            data: data,
            length: 0,
            pointer: std::ptr::null(),
        }
    }
}
pub unsafe extern "C" fn luaz_fill(zio: *mut ZIO) -> i32 {
    unsafe {
        let mut size: u64 = 0;
        let state: *mut State = (*zio).state;
        let buffer: *const i8 =
            ((*zio).reader).expect("non-null function pointer")(state, (*zio).data, &mut size);
        if buffer.is_null() || size == 0 {
            return -1;
        } else {
            (*zio).length = size.wrapping_sub(1 as u64);
            (*zio).pointer = buffer;
            let fresh14 = (*zio).pointer;
            (*zio).pointer = ((*zio).pointer).offset(1);
            return *fresh14 as u8 as i32;
        }
    }
}
pub unsafe extern "C" fn luaz_read(zio: *mut ZIO, mut b: *mut libc::c_void, mut n: u64) -> u64 {
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
            let m: u64 = if n <= (*zio).length { n } else { (*zio).length };
            memcpy(b, (*zio).pointer as *const libc::c_void, m as usize);
            (*zio).length = ((*zio).length as u64).wrapping_sub(m) as u64;
            (*zio).pointer = ((*zio).pointer).offset(m as isize);
            b = (b as *mut i8).offset(m as isize) as *mut libc::c_void;
            n = (n as u64).wrapping_sub(m) as u64;
        }
        return 0u64;
    }
}
