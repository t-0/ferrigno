use crate::functions::*;
use crate::state::*;
use crate::tstring::*;
use crate::prototype::*;
use crate::tag::*;
use crate::tvalue::*;
#[repr(C)]
pub struct DumpState {
    pub state: *mut State,
    pub write_function: WriteFunction,
    pub data: *mut libc::c_void,
    pub is_strip: bool,
    pub status: i32,
}
impl DumpState {
    fn new(
        state: *mut State,
        w: WriteFunction,
        data: *mut libc::c_void,
        is_strip: bool,
    ) -> Self {
        return DumpState {
            state: state,
            write_function: w,
            data: data,
            is_strip: is_strip,
            status: 0,
        };
    }
    pub unsafe extern "C" fn dump_block(& mut self, b: *const libc::c_void, size: u64) {
        unsafe {
            if self.status == 0 && size > 0 {
                self.status =
                    (Some((self.write_function).expect("non-null function pointer")))
                        .expect("non-null function pointer")(
                        self.state,
                        b,
                        size,
                        self.data,
                    );
            }
        }
    }
    pub unsafe extern "C" fn dump_byte(& mut self, y: u8) {
        unsafe {
            let mut x: u8 = y;
            self.dump_block(
                &mut x as *mut u8 as *const libc::c_void,
                (1 as u64).wrapping_mul(::core::mem::size_of::<u8>() as u64),
            );
        }
    }
    pub unsafe extern "C" fn dump_size(& mut self, mut x: u64) {
        unsafe {
            let mut buffer: [u8; 10] = [0; 10];
            let mut n: i32 = 0;
            loop {
                n += 1;
                buffer[(::core::mem::size_of::<u64>() as u64)
                    .wrapping_mul(8 as u64)
                    .wrapping_add(6 as u64)
                    .wrapping_div(7 as u64)
                    .wrapping_sub(n as u64) as usize] = (x & 0x7f as u64) as u8;
                x >>= 7;
                if !(x != 0u64) {
                    break;
                }
            }
            buffer[(::core::mem::size_of::<u64>() as u64)
                .wrapping_mul(8 as u64)
                .wrapping_add(6 as u64)
                .wrapping_div(7 as u64)
                .wrapping_sub(1 as u64) as usize] = (buffer[(::core::mem::size_of::<u64>() as u64)
                .wrapping_mul(8 as u64)
                .wrapping_add(6 as u64)
                .wrapping_div(7 as u64)
                .wrapping_sub(1 as u64)
                as usize] as i32
                | 0x80 as i32) as u8;
            self.dump_block(
                buffer
                    .as_mut_ptr()
                    .offset(
                        (::core::mem::size_of::<u64>() as u64)
                            .wrapping_mul(8 as u64)
                            .wrapping_add(6 as u64)
                            .wrapping_div(7 as u64) as isize,
                    )
                    .offset(-(n as isize)) as *const libc::c_void,
                (n as u64).wrapping_mul(::core::mem::size_of::<u8>() as u64),
            );
        }
    }
    pub unsafe extern "C" fn dump_int(& mut self, x: i32) {
        unsafe {
            self.dump_size(x as u64);
        }
    }
    pub unsafe extern "C" fn dump_number(& mut self, mut x: f64) {
        unsafe {
            self.dump_block(
                &mut x as *mut f64 as *const libc::c_void,
                (1 as u64).wrapping_mul(::core::mem::size_of::<f64>() as u64),
            );
        }
    }
    pub unsafe extern "C" fn dump_integer(& mut self, mut x: i64) {
        unsafe {
            self.dump_block(
                &mut x as *mut i64 as *const libc::c_void,
                (1 as u64).wrapping_mul(::core::mem::size_of::<i64>() as u64),
            );
        }
    }
    pub unsafe extern "C" fn dump_string(& mut self, s: *const TString) {
        unsafe {
            if s.is_null() {
                self.dump_size(0u64);
            } else {
                let size: u64 = (*s).get_length();
                let str: *const i8 = (*s).get_contents();
                self.dump_size(size.wrapping_add(1 as u64));
                self.dump_block(
                    str as *const libc::c_void,
                    size.wrapping_mul(::core::mem::size_of::<i8>() as u64),
                );
            };
        }
    }
    pub unsafe extern "C" fn dump_code(& mut self, f: *const Prototype) {
        unsafe {
            self.dump_int((*f).size_code);
            self.dump_block(
                (*f).code as *const libc::c_void,
                ((*f).size_code as u64).wrapping_mul(::core::mem::size_of::<u32>() as u64),
            );
        }
    }
    pub unsafe extern "C" fn dump_constants(& mut self, f: *const Prototype) {
        unsafe {
            let mut i: i32;
            let n: i32 = (*f).size_k;
            self.dump_int(n);
            i = 0;
            while i < n {
                let o: *const TValue = &mut *((*f).k).offset(i as isize) as *mut TValue;
                let tag = (*o).get_tag_variant();
                self.dump_byte(tag);
                match tag {
                    TAG_VARIANT_NUMERIC_NUMBER => {
                        self.dump_number((*o).value.n);
                    }
                    TAG_VARIANT_NUMERIC_INTEGER => {
                        self.dump_integer((*o).value.i);
                    }
                    TAG_VARIANT_STRING_SHORT | TAG_VARIANT_STRING_LONG => {
                        self.dump_string(&mut (*((*o).value.object as *mut TString)));
                    }
                    _ => {}
                }
                i += 1;
            }
        }
    }
    pub unsafe extern "C" fn dump_prototypes(& mut self, f: *const Prototype) {
        unsafe {
            let n: i32 = (*f).size_p;
            self.dump_int(n);
            let mut i: i32 = 0;
            while i < n {
                self.dump_function(*((*f).p).offset(i as isize), (*f).source);
                i += 1;
            }
        }
    }
    pub unsafe extern "C" fn dump_upvalues(& mut self, f: *const Prototype) {
        unsafe {
            let n: i32 = (*f).size_upvalues;
            self.dump_int(n);
            let mut i: i32 = 0;
            while i < n {
                self.dump_byte(
                    if (*((*f).upvalues).offset(i as isize)).is_in_stack {
                        1
                    } else {
                        0
                    },
                );
                self.dump_byte((*((*f).upvalues).offset(i as isize)).index);
                self.dump_byte((*((*f).upvalues).offset(i as isize)).kind);
                i += 1;
            }
        }
    }
    pub unsafe extern "C" fn dump_debug(& mut self, f: *const Prototype) {
        unsafe {
            let mut n: i32 = if self.is_strip {
                0
            } else {
                (*f).size_line_info
            };
            self.dump_int(n);
            let mut i: i32;
            self.dump_block(
                (*f).line_info as *const libc::c_void,
                (n as u64).wrapping_mul(::core::mem::size_of::<i8>() as u64),
            );
            n = if self.is_strip {
                0
            } else {
                (*f).size_absolute_line_info
            };
            self.dump_int(n);
            i = 0;
            while i < n {
                self.dump_int(
                    (*((*f).absolute_line_info).offset(i as isize)).program_counter,
                );
                self.dump_int(
                    (*((*f).absolute_line_info).offset(i as isize)).line,
                );
                i += 1;
            }
            n = if self.is_strip {
                0
            } else {
                (*f).size_local_variables
            };
            self.dump_int(n);
            i = 0;
            while i < n {
                self.dump_string(
                    (*((*f).local_variables).offset(i as isize)).variable_name,
                );
                self.dump_int(
                    (*((*f).local_variables).offset(i as isize)).start_program_counter,
                );
                self.dump_int(
                    (*((*f).local_variables).offset(i as isize)).end_program_counter,
                );
                i += 1;
            }
            n = if self.is_strip {
                0
            } else {
                (*f).size_upvalues
            };
            self.dump_int(n);
            i = 0;
            while i < n {
                self.dump_string((*((*f).upvalues).offset(i as isize)).name);
                i += 1;
            }
        }
    }
    pub unsafe extern "C" fn dump_function(
        & mut self,
        f: *const Prototype,
        psource: *mut TString,
    ) {
        unsafe {
            if self.is_strip || (*f).source == psource {
                self.dump_string(std::ptr::null());
            } else {
                self.dump_string((*f).source);
            }
            self.dump_int((*f).line_defined);
            self.dump_int((*f).last_line_defined);
            self.dump_byte((*f).count_parameters);
            self.dump_byte(if (*f).is_variable_arguments { 1 } else { 0 });
            self.dump_byte((*f).maximum_stack_size);
            self.dump_code(f);
            self.dump_constants(f);
            self.dump_upvalues(f);
            self.dump_prototypes(f);
            self.dump_debug(f);
        }
    }
    pub unsafe extern "C" fn dump_header(& mut self) {
        unsafe {
            self.dump_block(
                b"\x1BLua\0" as *const u8 as *const i8 as *const libc::c_void,
                (::core::mem::size_of::<[i8; 5]>() as u64)
                    .wrapping_sub(::core::mem::size_of::<i8>() as u64),
            );
            self.dump_byte(5 * 16 + 4);
            self.dump_byte(0);
            self.dump_block(
                b"\x19\x93\r\n\x1A\n\0" as *const u8 as *const i8 as *const libc::c_void,
                (::core::mem::size_of::<[i8; 7]>() as u64)
                    .wrapping_sub(::core::mem::size_of::<i8>() as u64),
            );
            self.dump_byte(::core::mem::size_of::<u32>() as u8);
            self.dump_byte(::core::mem::size_of::<i64>() as u8);
            self.dump_byte(::core::mem::size_of::<f64>() as u8);
            self.dump_integer(0x5678 as i64);
            self.dump_number(370.5f64);
        }
    }
    pub unsafe extern "C" fn dump(
        state: *mut State,
        f: *const Prototype,
        w: WriteFunction,
        data: *mut libc::c_void,
        is_strip: bool,
    ) -> i32 {
        unsafe {
            let mut dump_state = DumpState::new(state, w, data, is_strip);
            dump_state.dump_header();
            dump_state.dump_byte((*f).size_upvalues as u8);
            dump_state.dump_function(f, std::ptr::null_mut());
            return dump_state.status;
        }
    }
}
