use std::ptr::*;
use crate::functions::*;
use crate::interpreter::*;
use crate::tstring::*;
use crate::prototype::*;
#[repr(C)]
pub struct DumpState {
    pub interpreter: *mut Interpreter,
    pub write_function: WriteFunction,
    pub pointer: *mut libc::c_void,
    pub is_strip: bool,
    pub status: i32,
}
impl DumpState {
    pub fn new(
        interpreter: *mut Interpreter,
        write_function: WriteFunction,
        pointer: *mut libc::c_void,
        is_strip: bool,
    ) -> Self {
        return DumpState {
            interpreter: interpreter,
            write_function,
            pointer,
            is_strip: is_strip,
            status: 0,
        };
    }
    pub unsafe extern "C" fn dump_block(& mut self, pointer: *const libc::c_void, size: usize) {
        unsafe {
            if self.status == 0 && size > 0 {
                self.status =
                    (Some((self.write_function).expect("non-null function pointer")))
                        .expect("non-null function pointer")(
                        self.interpreter,
                        pointer,
                        size as usize,
                        self.pointer,
                    );
            }
        }
    }
    pub unsafe extern "C" fn dump_byte(& mut self, integer: u8) {
        unsafe {
            let mut x: u8 = integer;
            self.dump_block(
                &mut x as *mut u8 as *const libc::c_void,
                size_of::<u8>(),
            );
        }
    }
    pub unsafe extern "C" fn dump_size(& mut self, mut integer: usize) {
        unsafe {
            let mut buffer: [u8; 10] = [0; 10];
            let mut n: usize = 0;
            loop {
                n += 1;
                buffer[size_of::<usize>()
                    .wrapping_mul(8)
                    .wrapping_add(6)
                    .wrapping_div(7)
                    .wrapping_sub(n)] = (integer & 0x7F) as u8;
                integer >>= 7;
                if !(integer != 0) {
                    break;
                }
            }
            buffer[size_of::<usize>()
                .wrapping_mul(8)
                .wrapping_add(6)
                .wrapping_div(7)
                .wrapping_sub(1)] = (buffer[size_of::<usize>()
                .wrapping_mul(8)
                .wrapping_add(6)
                .wrapping_div(7)
                .wrapping_sub(1)
                ] as i32
                | 0x80 as i32) as u8;
            self.dump_block(
                buffer
                    .as_mut_ptr()
                    .offset(
                        (size_of::<usize>() as usize)
                            .wrapping_mul(8 as usize)
                            .wrapping_add(6 as usize)
                            .wrapping_div(7 as usize) as isize,
                    )
                    .offset(-(n as isize)) as *const libc::c_void,
                n.wrapping_mul(size_of::<u8>()),
            );
        }
    }
    pub unsafe extern "C" fn dump_int(& mut self, integer: i32) {
        unsafe {
            self.dump_size(integer as usize);
        }
    }
    pub unsafe extern "C" fn dump_number(& mut self, mut number: f64) {
        unsafe {
            self.dump_block(
                &mut number as *mut f64 as *const libc::c_void,
                size_of::<f64>(),
            );
        }
    }
    pub unsafe extern "C" fn dump_integer(& mut self, mut integer: i64) {
        unsafe {
            self.dump_block(
                &mut integer as *mut i64 as *const libc::c_void,
                size_of::<i64>(),
            );
        }
    }
    pub unsafe extern "C" fn dump_string(& mut self, tstring: *const TString) {
        unsafe {
            if tstring.is_null() {
                self.dump_size(0);
            } else {
                let size: usize = (*tstring).get_length() as usize;
                let str: *const i8 = (*tstring).get_contents_mut();
                self.dump_size(size.wrapping_add(1) as usize);
                self.dump_block(
                    str as *const libc::c_void,
                    size.wrapping_mul(size_of::<i8>()),
                );
            };
        }
    }
    pub unsafe extern "C" fn dump_header(& mut self) {
        unsafe {
            self.dump_block(
                b"\x1BLua\0" as *const u8 as *const i8 as *const libc::c_void,
                (size_of::<[i8; 5]>())
                    .wrapping_sub(size_of::<i8>()),
            );
            self.dump_byte(5 * 16 + 4);
            self.dump_byte(0);
            self.dump_block(
                b"\x19\x93\r\n\x1A\n\0" as *const u8 as *const i8 as *const libc::c_void,
                (size_of::<[i8; 7]>())
                    .wrapping_sub(size_of::<i8>()),
            );
            self.dump_byte(size_of::<u32>() as u8);
            self.dump_byte(size_of::<i64>() as u8);
            self.dump_byte(size_of::<f64>() as u8);
            self.dump_integer(0x5678);
            self.dump_number(370.5);
        }
    }
}
pub unsafe extern "C" fn save_prototype(
    interpreter: *mut Interpreter,
    prototype: *const Prototype,
    write_function: WriteFunction,
    data: *mut libc::c_void,
    is_strip: bool,
) -> i32 {
    unsafe {
        let mut dump_state = DumpState::new(interpreter, write_function, data, is_strip);
        dump_state.dump_header();
        dump_state.dump_byte((*prototype).prototype_upvalues.get_size() as u8);
        (*prototype).dump_function(&mut dump_state, null_mut());
        return dump_state.status;
    }
}
