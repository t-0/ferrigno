use crate::functions::*;
use crate::state::*;
use crate::tstring::*;
use crate::prototype::*;
use crate::tag::*;
use crate::tvalue::*;
#[repr(C)]
struct DumpState {
    pub state: *mut State,
    pub write_function: WriteFunction,
    pub pointer: *mut libc::c_void,
    pub is_strip: bool,
    pub status: i32,
}
impl DumpState {
    fn new(
        state: *mut State,
        write_function: WriteFunction,
        pointer: *mut libc::c_void,
        is_strip: bool,
    ) -> Self {
        return DumpState {
            state: state,
            write_function,
            pointer,
            is_strip: is_strip,
            status: 0,
        };
    }
    unsafe extern "C" fn dump_block(& mut self, pointer: *const libc::c_void, size: usize) {
        unsafe {
            if self.status == 0 && size > 0 {
                self.status =
                    (Some((self.write_function).expect("non-null function pointer")))
                        .expect("non-null function pointer")(
                        self.state,
                        pointer,
                        size as u64,
                        self.pointer,
                    );
            }
        }
    }
    unsafe extern "C" fn dump_byte(& mut self, integer: u8) {
        unsafe {
            let mut x: u8 = integer;
            self.dump_block(
                &mut x as *mut u8 as *const libc::c_void,
                ::core::mem::size_of::<u8>(),
            );
        }
    }
    unsafe extern "C" fn dump_size(& mut self, mut integer: u64) {
        unsafe {
            let mut buffer: [u8; 10] = [0; 10];
            let mut n: usize = 0;
            loop {
                n += 1;
                buffer[::core::mem::size_of::<u64>()
                    .wrapping_mul(8)
                    .wrapping_add(6)
                    .wrapping_div(7)
                    .wrapping_sub(n)] = (integer & 0x7F) as u8;
                integer >>= 7;
                if !(integer != 0) {
                    break;
                }
            }
            buffer[::core::mem::size_of::<u64>()
                .wrapping_mul(8)
                .wrapping_add(6)
                .wrapping_div(7)
                .wrapping_sub(1)] = (buffer[::core::mem::size_of::<u64>()
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
                        (::core::mem::size_of::<u64>() as u64)
                            .wrapping_mul(8 as u64)
                            .wrapping_add(6 as u64)
                            .wrapping_div(7 as u64) as isize,
                    )
                    .offset(-(n as isize)) as *const libc::c_void,
                n.wrapping_mul(::core::mem::size_of::<u8>()),
            );
        }
    }
    unsafe extern "C" fn dump_int(& mut self, integer: i32) {
        unsafe {
            self.dump_size(integer as u64);
        }
    }
    unsafe extern "C" fn dump_number(& mut self, mut number: f64) {
        unsafe {
            self.dump_block(
                &mut number as *mut f64 as *const libc::c_void,
                ::core::mem::size_of::<f64>(),
            );
        }
    }
    unsafe extern "C" fn dump_integer(& mut self, mut integer: i64) {
        unsafe {
            self.dump_block(
                &mut integer as *mut i64 as *const libc::c_void,
                ::core::mem::size_of::<i64>(),
            );
        }
    }
    unsafe extern "C" fn dump_string(& mut self, tstring: *const TString) {
        unsafe {
            if tstring.is_null() {
                self.dump_size(0);
            } else {
                let size: usize = (*tstring).get_length() as usize;
                let str: *const i8 = (*tstring).get_contents_mut();
                self.dump_size(size.wrapping_add(1) as u64);
                self.dump_block(
                    str as *const libc::c_void,
                    size.wrapping_mul(::core::mem::size_of::<i8>()),
                );
            };
        }
    }
    unsafe extern "C" fn dump_code(& mut self, prototype: *const Prototype) {
        unsafe {
            self.dump_int((*prototype).size_code);
            self.dump_block(
                (*prototype).code as *const libc::c_void,
                ((*prototype).size_code as usize).wrapping_mul(::core::mem::size_of::<u32>()),
            );
        }
    }
    unsafe extern "C" fn dump_constants(& mut self, prototype: *const Prototype) {
        unsafe {
            let n: i32 = (*prototype).size_k;
            self.dump_int(n);
            for i in 0..n {
                let tvalue: *const TValue = &mut *((*prototype).k).offset(i as isize) as *mut TValue;
                let tag = (*tvalue).get_tag_variant();
                self.dump_byte(tag);
                match tag {
                    TAG_VARIANT_NUMERIC_NUMBER => {
                        self.dump_number((*tvalue).value.number);
                    }
                    TAG_VARIANT_NUMERIC_INTEGER => {
                        self.dump_integer((*tvalue).value.integer);
                    }
                    TAG_VARIANT_STRING_SHORT | TAG_VARIANT_STRING_LONG => {
                        self.dump_string(&mut (*((*tvalue).value.object as *mut TString)));
                    }
                    _ => {}
                }
            }
        }
    }
    unsafe extern "C" fn dump_prototypes(& mut self, prototype: *const Prototype) {
        unsafe {
            let n: i32 = (*prototype).size_p;
            self.dump_int(n);
            for i in 0..n {
                self.dump_function(*((*prototype).p).offset(i as isize), (*prototype).source);
            }
        }
    }
    unsafe extern "C" fn dump_upvalues(& mut self, prototype: *const Prototype) {
        unsafe {
            let n: i32 = (*prototype).size_upvalues;
            self.dump_int(n);
            for i in 0..n {
                self.dump_byte(
                    if (*((*prototype).upvalues).offset(i as isize)).is_in_stack {
                        1
                    } else {
                        0
                    },
                );
                self.dump_byte((*((*prototype).upvalues).offset(i as isize)).index);
                self.dump_byte((*((*prototype).upvalues).offset(i as isize)).kind);
            }
        }
    }
    unsafe extern "C" fn dump_debug(& mut self, prototype: *const Prototype) {
        unsafe {
            let mut n: usize = if self.is_strip {
                0
            } else {
                (*prototype).size_line_info as usize
            };
            self.dump_int(n as i32);
            self.dump_block(
                (*prototype).line_info as *const libc::c_void,
                n.wrapping_mul(::core::mem::size_of::<i8>()),
            );
            n = if self.is_strip {
                0
            } else {
                (*prototype).size_absolute_line_info as usize
            };
            self.dump_int(n as i32);
            for i in 0..n {
                self.dump_int(
                    (*((*prototype).absolute_line_info).offset(i as isize)).program_counter,
                );
                self.dump_int(
                    (*((*prototype).absolute_line_info).offset(i as isize)).line,
                );
            }
            n = if self.is_strip {
                0
            } else {
                (*prototype).size_local_variables as usize
            };
            self.dump_int(n as i32);
            for i in 0..n {
                self.dump_string(
                    (*((*prototype).local_variables).offset(i as isize)).variable_name,
                );
                self.dump_int(
                    (*((*prototype).local_variables).offset(i as isize)).start_program_counter,
                );
                self.dump_int(
                    (*((*prototype).local_variables).offset(i as isize)).end_program_counter,
                );
            }
            n = if self.is_strip {
                0
            } else {
                (*prototype).size_upvalues as usize
            };
            self.dump_int(n as i32);
            for i in 0..n {
                self.dump_string((*((*prototype).upvalues).offset(i as isize)).name);
            }
        }
    }
    unsafe extern "C" fn dump_function(
        & mut self,
        prototype: *const Prototype,
        psource: *mut TString,
    ) {
        unsafe {
            if self.is_strip || (*prototype).source == psource {
                self.dump_string(std::ptr::null());
            } else {
                self.dump_string((*prototype).source);
            }
            self.dump_int((*prototype).line_defined);
            self.dump_int((*prototype).last_line_defined);
            self.dump_byte((*prototype).count_parameters);
            self.dump_byte(if (*prototype).is_variable_arguments { 1 } else { 0 });
            self.dump_byte((*prototype).maximum_stack_size);
            self.dump_code(prototype);
            self.dump_constants(prototype);
            self.dump_upvalues(prototype);
            self.dump_prototypes(prototype);
            self.dump_debug(prototype);
        }
    }
    unsafe extern "C" fn dump_header(& mut self) {
        unsafe {
            self.dump_block(
                b"\x1BLua\0" as *const u8 as *const i8 as *const libc::c_void,
                (::core::mem::size_of::<[i8; 5]>())
                    .wrapping_sub(::core::mem::size_of::<i8>()),
            );
            self.dump_byte(5 * 16 + 4);
            self.dump_byte(0);
            self.dump_block(
                b"\x19\x93\r\n\x1A\n\0" as *const u8 as *const i8 as *const libc::c_void,
                (::core::mem::size_of::<[i8; 7]>())
                    .wrapping_sub(::core::mem::size_of::<i8>()),
            );
            self.dump_byte(::core::mem::size_of::<u32>() as u8);
            self.dump_byte(::core::mem::size_of::<i64>() as u8);
            self.dump_byte(::core::mem::size_of::<f64>() as u8);
            self.dump_integer(0x5678);
            self.dump_number(370.5);
        }
    }
}
pub unsafe extern "C" fn save_prototype(
    state: *mut State,
    prototype: *const Prototype,
    write_function: WriteFunction,
    data: *mut libc::c_void,
    is_strip: bool,
) -> i32 {
    unsafe {
        let mut dump_state = DumpState::new(state, write_function, data, is_strip);
        dump_state.dump_header();
        dump_state.dump_byte((*prototype).size_upvalues as u8);
        dump_state.dump_function(prototype, std::ptr::null_mut());
        return dump_state.status;
    }
}
