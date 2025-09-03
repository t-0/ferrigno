use crate::functions::*;
use crate::state::*;
use libc::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZIO {
    pub n: u64,
    pub p: *const i8,
    pub reader: ReadFunction,
    pub data: *mut libc::c_void,
    pub state: *mut State,
}
pub unsafe extern "C" fn luaz_fill(zio: *mut ZIO) -> i32 {
    unsafe {
        let mut size: u64 = 0;
        let state: *mut State = (*zio).state;
        let buffer: *const i8 =
            ((*zio).reader).expect("non-null function pointer")(state, (*zio).data, &mut size);
        if buffer.is_null() || size == 0 {
            return -1;
        }
        (*zio).n = size.wrapping_sub(1 as u64);
        (*zio).p = buffer;
        let fresh14 = (*zio).p;
        (*zio).p = ((*zio).p).offset(1);
        return *fresh14 as u8 as i32;
    }
}
pub unsafe extern "C" fn luaz_init(
    state: *mut State,
    zio: *mut ZIO,
    reader: ReadFunction,
    data: *mut libc::c_void,
) {
    unsafe {
        (*zio).state = state;
        (*zio).reader = reader;
        (*zio).data = data;
        (*zio).n = 0;
        (*zio).p = std::ptr::null();
    }
}
pub unsafe extern "C" fn luaz_read(zio: *mut ZIO, mut b: *mut libc::c_void, mut n: u64) -> u64 {
    unsafe {
        while n != 0 {
            if (*zio).n == 0 {
                if luaz_fill(zio) == -1 {
                    return n;
                } else {
                    (*zio).n = ((*zio).n).wrapping_add(1);
                    (*zio).n;
                    (*zio).p = ((*zio).p).offset(-1);
                    (*zio).p;
                }
            }
            let m: u64 = if n <= (*zio).n { n } else { (*zio).n };
            memcpy(b, (*zio).p as *const libc::c_void, m as usize);
            (*zio).n = ((*zio).n as u64).wrapping_sub(m) as u64;
            (*zio).p = ((*zio).p).offset(m as isize);
            b = (b as *mut i8).offset(m as isize) as *mut libc::c_void;
            n = (n as u64).wrapping_sub(m) as u64;
        }
        return 0u64;
    }
}
