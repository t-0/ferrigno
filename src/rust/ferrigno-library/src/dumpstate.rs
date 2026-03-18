use std::ptr::*;
//
use crate::functions::*;
use crate::prototype::*;
use crate::state::*;
use crate::table::*;
use crate::tagvariant::*;
use crate::tvalue::*;
//
#[repr(C)]
pub struct DumpState {
    dumpstate_interpreter: *mut State,
    dumpstate_write_function: WriteFunction,
    dumpstate_pointer: *mut std::ffi::c_void,
    dumpstate_is_strip: bool,
    dumpstate_status: i32,
    pub dumpstate_table: *mut Table,
    pub dumpstate_count_string: u64,
}
const VARINT_DATA_MASK: u8 = 0x7F;
const VARINT_END_BIT: u8 = 0x80;
const MAX_VARINT_BYTES: usize = (usize::BITS as usize).div_ceil(7);
impl DumpState {
    pub const LUA_SIGNATURE: *const i8 = c"\x1BLua".as_ptr();
    pub const LUA_VERSION_MAJOR: u8 = 5;
    pub const LUA_VERSION_MINOR: u8 = 5;
    pub const LUAC_VERSION: u8 = DumpState::LUA_VERSION_MAJOR * 16 + DumpState::LUA_VERSION_MINOR;
    pub const LUAC_FORMAT: u8 = 0;
    pub const LUAC_DATA: *const i8 = c"\x19\x93\r\n\x1A\n".as_ptr();
    pub const LUAC_INT: i32 = -0x5678;
    pub const LUAC_INST: u32 = 0x12345678;
    pub const LUAC_INTEGER: i64 = -0x5678;
    pub const LUAC_NUM: f64 = -370.5;
    pub fn new(state: *mut State, write_function: WriteFunction, pointer: *mut std::ffi::c_void, is_strip: bool) -> Self {
        DumpState {
            dumpstate_interpreter: state,
            dumpstate_write_function: write_function,
            dumpstate_pointer: pointer,
            dumpstate_is_strip: is_strip,
            dumpstate_status: 0,
            dumpstate_table: null_mut(),
            dumpstate_count_string: 0,
        }
    }
    pub fn is_strip(&self) -> bool {
        self.dumpstate_is_strip
    }
    pub unsafe fn dump_block(&mut self, pointer: *const std::ffi::c_void, size: usize) {
        unsafe {
            if self.dumpstate_status == 0 && size > 0 {
                self.dumpstate_status = (self.dumpstate_write_function).expect("non-null function pointer")(
                    self.dumpstate_interpreter, pointer, size, self.dumpstate_pointer,
                );
            }
        }
    }
    pub unsafe fn dump_byte(&mut self, integer: u8) {
        unsafe {
            let x: u8 = integer;
            self.dump_block(&x as *const u8 as *const std::ffi::c_void, 1);
        }
    }
    pub unsafe fn dump_size(&mut self, mut integer: usize) {
        unsafe {
            let mut buffer: [u8; MAX_VARINT_BYTES] = [0; MAX_VARINT_BYTES];
            let mut n: usize = 0;
            loop {
                n += 1;
                buffer[MAX_VARINT_BYTES - n] = (integer as u8) & VARINT_DATA_MASK;
                integer >>= 7;
                if integer == 0 {
                    break;
                }
            }
            buffer[MAX_VARINT_BYTES - 1] |= VARINT_END_BIT;
            self.dump_block(buffer.as_mut_ptr().add(MAX_VARINT_BYTES).sub(n) as *const std::ffi::c_void, n);
        }
    }
    pub unsafe fn dump_int(&mut self, integer: i32) {
        unsafe {
            self.dump_size(integer as usize);
        }
    }
    pub unsafe fn dump_number(&mut self, number: f64) {
        unsafe {
            self.dump_block(&number as *const f64 as *const std::ffi::c_void, size_of::<f64>());
        }
    }
    pub unsafe fn dump_integer(&mut self, integer: i64) {
        unsafe {
            self.dump_block(&integer as *const i64 as *const std::ffi::c_void, size_of::<i64>());
        }
    }
    pub unsafe fn dump_header(&mut self) {
        unsafe {
            self.dump_block(DumpState::LUA_SIGNATURE as *const std::ffi::c_void, (size_of::<[i8; 5]>()) - 1);
            self.dump_byte(DumpState::LUAC_VERSION);
            self.dump_byte(DumpState::LUAC_FORMAT);
            self.dump_block(DumpState::LUAC_DATA as *const std::ffi::c_void, (size_of::<[i8; 7]>()) - 1);
            let int_val: i32 = DumpState::LUAC_INT;
            self.dump_byte(size_of::<i32>() as u8);
            self.dump_block(&int_val as *const i32 as *const std::ffi::c_void, size_of::<i32>());
            let inst_val: u32 = DumpState::LUAC_INST;
            self.dump_byte(size_of::<u32>() as u8);
            self.dump_block(&inst_val as *const u32 as *const std::ffi::c_void, size_of::<u32>());
            let integer_val: i64 = DumpState::LUAC_INTEGER;
            self.dump_byte(size_of::<i64>() as u8);
            self.dump_block(&integer_val as *const i64 as *const std::ffi::c_void, size_of::<i64>());
            let number_val: f64 = DumpState::LUAC_NUM;
            self.dump_byte(size_of::<f64>() as u8);
            self.dump_block(&number_val as *const f64 as *const std::ffi::c_void, size_of::<f64>());
        }
    }
    pub fn state(&self) -> *mut State {
        self.dumpstate_interpreter
    }
    pub unsafe fn save_prototype(
        state: *mut State, prototype: *const Prototype, write_function: WriteFunction, pointer: *mut std::ffi::c_void,
        is_strip: bool,
    ) -> i32 {
        unsafe {
            let mut dump_state = DumpState::new(state, write_function, pointer, is_strip);
            dump_state.dumpstate_table = luah_new(state);
            // anchor the table on the stack
            let io: *mut TValue = &mut (*(*state).interpreter_top.stkidrel_pointer);
            (*io).set_table(dump_state.dumpstate_table);
            (*state).luad_inctop();
            dump_state.dump_header();
            dump_state.dump_byte((*prototype).prototype_upvalues.get_size() as u8);
            (*prototype).dump_function(&mut dump_state, null_mut());
            // Don't restore the stack here: the writer callback may have pushed
            // a to-be-closed userbox above the dedup table. Forcibly lowering the
            // stack top would orphan the tbc entry, causing a crash when
            // moveresults calls luaf_close on function return. The extra table
            // slot is harmless — moveresults cleans it up.
            dump_state.dumpstate_status
        }
    }
}
pub unsafe fn lua_dump(state: *mut State, writer: WriteFunction, data: *mut std::ffi::c_void, is_strip: bool) -> i32 {
    unsafe {
        let status: i32;
        let o: *mut TValue = &mut (*(*state).interpreter_top.stkidrel_pointer.sub(1));
        if (*o).get_tagvariant() == TagVariant::ClosureL {
            status = DumpState::save_prototype(
                state,
                (*(*o).as_closure().unwrap()).closure_payload.closurepayload_lprototype,
                writer,
                data,
                is_strip,
            );
        } else {
            status = 1;
        }
        status
    }
}
