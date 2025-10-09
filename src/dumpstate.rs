use libc::*;
use std::ptr::*;
//
use crate::functions::*;
use crate::interpreter::*;
use crate::prototype::*;
use crate::tvalue::*;
use crate::closure::*;
use crate::tagvariant::*;
//
pub const LUA_SIGNATURE: *const i8 = c"\x1BLua".as_ptr();
#[repr(C)]
pub struct DumpState {
    m_interpreter: *mut Interpreter,
    m_write_function: WriteFunction,
    m_pointer: *mut c_void,
    m_is_strip: bool,
    m_status: i32,
}
impl DumpState {
    pub fn new(interpreter: *mut Interpreter, write_function: WriteFunction, pointer: *mut c_void, is_strip: bool) -> Self {
        return DumpState {
            m_interpreter: interpreter,
            m_write_function: write_function,
            m_pointer: pointer,
            m_is_strip: is_strip,
            m_status: 0,
        };
    }
    pub fn is_strip(&self) -> bool {
        self.m_is_strip
    }
    pub unsafe fn dump_block(&mut self, pointer: *const c_void, size: usize) {
        unsafe {
            if self.m_status == 0 && size > 0 {
                self.m_status = (Some((self.m_write_function).expect("non-null function pointer")))
                    .expect("non-null function pointer")(
                    self.m_interpreter, pointer, size as usize, self.m_pointer
                );
            }
        }
    }
    pub unsafe fn dump_byte(&mut self, integer: u8) {
        unsafe {
            let x: u8 = integer;
            self.dump_block(&x as *const u8 as *const c_void, 1);
        }
    }
    pub unsafe fn dump_size(&mut self, mut integer: usize) {
        unsafe {
            let mut buffer: [u8; 10] = [0; 10];
            let mut n: usize = 0;
            loop {
                n += 1;
                buffer[(size_of::<usize>() * 8 + 6) / 7 - n] = (integer & 0x7F) as u8;
                integer >>= 7;
                if !(integer != 0) {
                    break;
                }
            }
            buffer[(size_of::<usize>() * 8 + 6) / 7 - 1] =
                (buffer[(size_of::<usize>() * 8 + 6) / 7 - 1] as i32 | 0x80 as i32) as u8;
            self.dump_block(
                buffer
                    .as_mut_ptr()
                    .offset((((size_of::<usize>()) * 8 + 6) / 7) as isize)
                    .offset(-(n as isize)) as *const c_void,
                n,
            );
        }
    }
    pub unsafe fn dump_int(&mut self, integer: i32) {
        unsafe {
            self.dump_size(integer as usize);
        }
    }
    pub unsafe fn dump_number(&mut self, number: f64) {
        unsafe {
            self.dump_block(&number as *const f64 as *const c_void, size_of::<f64>());
        }
    }
    pub unsafe fn dump_integer(&mut self, integer: i64) {
        unsafe {
            self.dump_block(&integer as *const i64 as *const c_void, size_of::<i64>());
        }
    }
    pub unsafe fn dump_header(&mut self) {
        unsafe {
            self.dump_block(LUA_SIGNATURE as *const c_void, (size_of::<[i8; 5]>()) - 1);
            self.dump_byte(5 * 16 + 4);
            self.dump_byte(0);
            self.dump_block(c"\x19\x7F\r\n\x1A\n".as_ptr() as *const c_void, (size_of::<[i8; 7]>()) - 1);
            self.dump_integer(0x5678);
            self.dump_number(370.5);
        }
    }
    pub unsafe fn save_prototype(
        interpreter: *mut Interpreter, prototype: *const Prototype, write_function: WriteFunction, pointer: *mut c_void, is_strip: bool,
    ) -> i32 {
        unsafe {
            let mut dump_state = DumpState::new(interpreter, write_function, pointer, is_strip);
            dump_state.dump_header();
            dump_state.dump_byte((*prototype).prototype_upvalues.get_size() as u8);
            (*prototype).dump_function(&mut dump_state, null_mut());
            dump_state.m_status
        }
    }
}
pub unsafe fn lua_dump(interpreter: *mut Interpreter, writer_0: WriteFunction, data: *mut libc::c_void, is_strip: bool) -> i32 {
    unsafe {
        let status: i32;
        let o: *mut TValue = &mut (*(*interpreter).interpreter_top.stkidrel_pointer.offset(-(1 as isize)));
        if (*o).get_tagvariant() == TagVariant::ClosureL {
            status = DumpState::save_prototype(
                interpreter,
                (*((*o).tvalue_value.value_object as *mut Closure)).payload.l_prototype,
                writer_0,
                data,
                is_strip,
            );
        } else {
            status = 1;
        }
        return status;
    }
}
