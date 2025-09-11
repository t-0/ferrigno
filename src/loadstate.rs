use std::ptr::*;
use crate::interpreter::*;
use crate::zio::*;
use crate::character::*;
use crate::closure::*;
use crate::prototype::*;
use crate::tstring::*;
use crate::loadable::*;
use crate::object::*;
use crate::tag::*;
use crate::tvalue::*;
use crate::localvariable::*;
use crate::upvaluedescription::*;
use crate::debugger::absolutelineinfo::*;
use crate::utility::c::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LoadState {
    interpreter: *mut Interpreter,
    zio: *mut ZIO,
    name: *const i8,
}
impl LoadState {
    pub unsafe extern "C" fn error(& mut self, why: *const i8) -> ! {
        unsafe {
            luao_pushfstring(
                self.interpreter,
                b"%s: bad binary format (%s)\0" as *const u8 as *const i8,
                self.name,
                why,
            );
            luad_throw(self.interpreter, 3);
        }
    }
    pub unsafe extern "C" fn load_block(& mut self, b: *mut libc::c_void, size: u64) {
        unsafe {
            if luaz_read(self.zio, b, size) != 0 {
                self.error(b"truncated chunk\0" as *const u8 as *const i8);
            }
        }
    }
    pub unsafe extern "C" fn load_byte(& mut self) -> u8 {
        unsafe {
            let aa = (*self.zio).length;
            (*self.zio).length = ((*self.zio).length).wrapping_sub(1);
            let ret: i32 = if aa > 0 {
                let bb = (*self.zio).pointer;
                (*self.zio).pointer = ((*self.zio).pointer).offset(1);
                *bb as u8 as i32
            } else {
                luaz_fill(self.zio)
            };
            if ret == -1 {
                self.error(b"truncated chunk\0" as *const u8 as *const i8);
            }
            return ret as u8;
        }
    }
    pub unsafe extern "C" fn load_unsigned(& mut self, mut limit: u64) -> u64 {
        unsafe {
            let mut ret: u64 = 0;
            limit >>= 7;
            loop {
                let b: i32 = self.load_byte() as i32;
                if ret >= limit {
                    self.error(b"integer overflow\0" as *const u8 as *const i8);
                }
                ret = ret << 7 | (b & 0x7f as i32) as u64;
                if !(b & 0x80 as i32 == 0) {
                    break;
                }
            }
            return ret;
        }
    }
    pub unsafe extern "C" fn load_size(& mut self) -> u64 {
        unsafe {
            return self.load_unsigned(!(0u64));
        }
    }
    pub unsafe extern "C" fn load_int(& mut self) -> i32 {
        unsafe {
            return self.load_unsigned(0x7FFFFFFF as u64) as i32;
        }
    }
    pub unsafe extern "C" fn load_number(& mut self) -> f64 {
        unsafe {
            let mut x: f64 = 0.0;
            self.load_block(
                &mut x as *mut f64 as *mut libc::c_void,
                1u64.wrapping_mul(::core::mem::size_of::<f64>() as u64),
            );
            return x;
        }
    }
    pub unsafe extern "C" fn load_integer(& mut self) -> i64 {
        unsafe {
            let mut x: i64 = 0;
            self.load_block(
                &mut x as *mut i64 as *mut libc::c_void,
                1u64.wrapping_mul(::core::mem::size_of::<i64>() as u64),
            );
            return x;
        }
    }
    pub unsafe extern "C" fn load_string_n(
        & mut self,
        p: *mut Prototype,
    ) -> *mut TString {
        unsafe {
            let interpreter: *mut Interpreter = self.interpreter;
            let ts: *mut TString;
            let mut size: u64 = self.load_size();
            if size == 0 {
                return null_mut();
            } else {
                size = size.wrapping_sub(1);
                if size <= 40 as u64 {
                    let mut buffer: [i8; 40] = [0; 40];
                    self.load_block(
                        buffer.as_mut_ptr() as *mut libc::c_void,
                        size.wrapping_mul(::core::mem::size_of::<i8>() as u64),
                    );
                    ts = luas_newlstr(interpreter, buffer.as_mut_ptr(), size);
                } else {
                    ts = TString::create_long(interpreter, size);
                    let io: *mut TValue = &mut (*(*interpreter).top.stkidrel_pointer).tvalue;
                    (*io).value.object = &mut (*(ts as *mut Object));
                    (*io).set_tag_variant((*ts).get_tag_variant());
                    (*io).set_collectable(true);
                    (*interpreter).luad_inctop();
                    self.load_block(
                        ((*ts).get_contents_mut()) as *mut libc::c_void,
                        size.wrapping_mul(::core::mem::size_of::<i8>() as u64),
                    );
                    (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(-1);
                }
            }
            if (*p).get_marked() & 1 << 5 != 0 && (*ts).get_marked() & (1 << 3 | 1 << 4) != 0 {
                luac_barrier_(
                    interpreter,
                    &mut (*(p as *mut Object)),
                    &mut (*(ts as *mut Object)),
                );
            } else {
            };
            return ts;
        }
    }
    pub unsafe extern "C" fn load_string(
        & mut self,
        p: *mut Prototype,
    ) -> *mut TString {
        unsafe {
            let st: *mut TString = self.load_string_n(p);
            if st.is_null() {
                self.error(
                    b"bad format for constant string\0" as *const u8 as *const i8,
                );
            }
            return st;
        }
    }
    pub unsafe extern "C" fn load_code(& mut self, prototype: *mut Prototype) {
        unsafe {
            let n: i32 = self.load_int();
            if ::core::mem::size_of::<i32>() as u64 >= ::core::mem::size_of::<u64>() as u64
                && (n as u64).wrapping_add(1u64)
                    > (!(0u64)).wrapping_div(::core::mem::size_of::<u32>() as u64)
            {
                (*(self.interpreter)).too_big();
            } else {
            };
            (*prototype).prototype_code.pointer = luam_malloc_(
                self.interpreter,
                (n as usize).wrapping_mul(::core::mem::size_of::<u32>()),
            ) as *mut u32;
            (*prototype).prototype_code.size = n;
            self.load_block(
                (*prototype).prototype_code.pointer as *mut libc::c_void,
                (n as usize).wrapping_mul(::core::mem::size_of::<u32>()) as u64,
            );
        }
    }
    pub unsafe extern "C" fn load_constants(& mut self, prototype: *mut Prototype) {
        unsafe {
            let n: i32 = self.load_int();
            if ::core::mem::size_of::<i32>() as u64 >= ::core::mem::size_of::<u64>() as u64
                && (n as u64).wrapping_add(1u64)
                    > (!(0u64)).wrapping_div(::core::mem::size_of::<TValue>() as u64)
            {
                (*(self.interpreter)).too_big();
            } else {
            };
            (*prototype).prototype_constants.pointer = luam_malloc_(
                self.interpreter,
                (n as usize).wrapping_mul(::core::mem::size_of::<TValue>()),
            ) as *mut TValue;
            (*prototype).prototype_constants.size = n;
            for i in 0..n {
                (*((*prototype).prototype_constants.pointer).offset(i as isize)).set_tag_variant(TagVariant::NilNil as u8);
            }
            for i in 0..n {
                let tvalue: *mut TValue = &mut *((*prototype).prototype_constants.pointer).offset(i as isize) as *mut TValue;
                let t = self.load_byte() as u8;
                match t {
                    TAG_VARIANT_NIL_NIL => {
                        (*tvalue).set_tag_variant(TagVariant::NilNil as u8);
                    }
                    TAG_VARIANT_BOOLEAN_FALSE => {
                        (*tvalue).set_tag_variant(TAG_VARIANT_BOOLEAN_FALSE);
                    }
                    TAG_VARIANT_BOOLEAN_TRUE => {
                        (*tvalue).set_tag_variant(TAG_VARIANT_BOOLEAN_TRUE);
                    }
                    TAG_VARIANT_NUMERIC_NUMBER => {
                        let io: *mut TValue = tvalue;
                        (*io).value.number = self.load_number();
                        (*io).set_tag_variant(TAG_VARIANT_NUMERIC_NUMBER);
                    }
                    TAG_VARIANT_NUMERIC_INTEGER => {
                        let io_0: *mut TValue = tvalue;
                        (*io_0).value.integer = self.load_integer();
                        (*io_0).set_tag_variant(TAG_VARIANT_NUMERIC_INTEGER);
                    }
                    TAG_VARIANT_STRING_SHORT | TAG_VARIANT_STRING_LONG => {
                        let io_1: *mut TValue = tvalue;
                        let ts: *mut TString = self.load_string(prototype);
                        (*io_1).value.object = &mut (*(ts as *mut Object));
                        (*io_1).set_tag_variant((*ts).get_tag_variant());
                        (*io_1).set_collectable(true);
                    }
                    _ => {}
                }
            }
        }
    }
    pub unsafe extern "C" fn load_prototypes(& mut self, prototype: *mut Prototype) {
        unsafe {
            let n: i32 = self.load_int();
            if ::core::mem::size_of::<i32>() as u64 >= ::core::mem::size_of::<u64>() as u64
                && (n as u64).wrapping_add(1u64)
                    > (!(0u64)).wrapping_div(::core::mem::size_of::<*mut Prototype>() as u64)
            {
                (*(self.interpreter)).too_big();
            } else {
            };
            (*prototype).prototype_prototypes.pointer = luam_malloc_(
                self.interpreter,
                (n as usize).wrapping_mul(::core::mem::size_of::<*mut Prototype>()),
            ) as *mut *mut Prototype;
            (*prototype).prototype_prototypes.size = n;
            for i in 0..n {
                *((*prototype).prototype_prototypes.pointer).offset(i as isize) = null_mut();
            }
            for i in 0..n {
                *((*prototype).prototype_prototypes.pointer).offset(i as isize) = luaf_newproto(self.interpreter);
                if (*prototype).get_marked() & 1 << 5 != 0
                    && (**((*prototype).prototype_prototypes.pointer).offset(i as isize)).get_marked() & (1 << 3 | 1 << 4) != 0
                {
                    luac_barrier_(
                        self.interpreter,
                        &mut (*(prototype as *mut Object)),
                        &mut (*(*((*prototype).prototype_prototypes.pointer).offset(i as isize) as *mut Object)),
                    );
                } else {
                }
                self.load_function(*((*prototype).prototype_prototypes.pointer).offset(i as isize), (*prototype).prototype_source);
            }
        }
    }
    pub unsafe extern "C" fn load_upvalues(& mut self, prototype: *mut Prototype) {
        unsafe {
            let n: i32;
            n = self.load_int();
            if ::core::mem::size_of::<i32>() as u64 >= ::core::mem::size_of::<u64>() as u64
                && (n as u64).wrapping_add(1u64)
                    > (!(0u64)).wrapping_div(::core::mem::size_of::<UpValueDescription>() as u64)
            {
                (*(self.interpreter)).too_big();
            } else {
            };
            (*prototype).prototype_upvalues.pointer = luam_malloc_(
                self.interpreter,
                (n as usize).wrapping_mul(::core::mem::size_of::<UpValueDescription>()),
            ) as *mut UpValueDescription;
            (*prototype).prototype_upvalues.size = n;
            for i in 0..n {
                let ref mut fresh29 = (*((*prototype).prototype_upvalues.pointer).offset(i as isize)).name;
                *fresh29 = null_mut();
            }
            for i in 0..n {
                let upvalue_description = & mut *((*prototype).prototype_upvalues.at_mut(i as isize));
                upvalue_description.load(self);
            }
        }
    }
    pub unsafe extern "C" fn load_debug(& mut self, prototype: *mut Prototype) {
        unsafe {
            let mut n: i32;
            n = self.load_int();
            if ::core::mem::size_of::<i32>() as u64 >= ::core::mem::size_of::<u64>() as u64
                && (n as u64).wrapping_add(1u64)
                    > (!(0u64)).wrapping_div(::core::mem::size_of::<i8>() as u64)
            {
                (*(self.interpreter)).too_big();
            } else {
            };
            (*prototype).prototype_line_info.pointer = luam_malloc_(
                self.interpreter,
                (n as usize).wrapping_mul(::core::mem::size_of::<i8>()),
            ) as *mut i8;
            (*prototype).prototype_line_info.size = n;
            self.load_block(
                (*prototype).prototype_line_info.pointer as *mut libc::c_void,
                (n as u64).wrapping_mul(::core::mem::size_of::<i8>() as u64),
            );
            n = self.load_int();
            if ::core::mem::size_of::<i32>() as u64 >= ::core::mem::size_of::<u64>() as u64
                && (n as u64).wrapping_add(1u64)
                    > (!(0u64)).wrapping_div(::core::mem::size_of::<AbsoluteLineInfo>() as u64)
            {
                (*(self.interpreter)).too_big();
            } else {
            };
            (*prototype).prototype_absolute_line_info.pointer = luam_malloc_(
                self.interpreter,
                (n as usize).wrapping_mul(::core::mem::size_of::<AbsoluteLineInfo>()),
            ) as *mut AbsoluteLineInfo;
            (*prototype).prototype_absolute_line_info.size = n;
            for i in 0..n {
                (*((*prototype).prototype_absolute_line_info.pointer).offset(i as isize)).program_counter = self.load_int();
                (*((*prototype).prototype_absolute_line_info.pointer).offset(i as isize)).line = self.load_int();
            }
            n = self.load_int();
            if ::core::mem::size_of::<i32>() as u64 >= ::core::mem::size_of::<u64>() as u64
                && (n as u64).wrapping_add(1u64)
                    > (!(0u64)).wrapping_div(::core::mem::size_of::<LocalVariable>() as u64)
            {
                (*(self.interpreter)).too_big();
            } else {
            };
            (*prototype).prototype_local_variables.pointer = luam_malloc_(
                self.interpreter,
                (n as usize).wrapping_mul(::core::mem::size_of::<LocalVariable>()),
            ) as *mut LocalVariable;
            (*prototype).prototype_local_variables.size = n;
            for i in 0..n {
                let ref mut fresh30 = (*((*prototype).prototype_local_variables.at_mut(i as isize))).variable_name;
                *fresh30 = null_mut();
            }
            for i in 0..n {
                let ref mut fresh31 = (*((*prototype).prototype_local_variables.at_mut(i as isize))).variable_name;
                *fresh31 = self.load_string_n(prototype);
                (*((*prototype).prototype_local_variables.at_mut(i as isize))).start_program_counter =
                    self.load_int();
                (*((*prototype).prototype_local_variables.at_mut(i as isize))).end_program_counter = self.load_int();
            }
            n = self.load_int();
            if n != 0 {
                n = (*prototype).prototype_upvalues.size;
            }
            for i in 0..n {
                let ref mut fresh32 = (*((*prototype).prototype_upvalues.pointer).offset(i as isize)).name;
                *fresh32 = self.load_string_n(prototype);
            }
        }
    }
    pub unsafe extern "C" fn load_function(
        & mut self,
        prototype: *mut Prototype,
        psource: *mut TString,
    ) {
        unsafe {
            (*prototype).prototype_source = self.load_string_n(prototype);
            if ((*prototype).prototype_source).is_null() {
                (*prototype).prototype_source = psource;
            }
            (*prototype).prototype_line_defined = self.load_int();
            (*prototype).prototype_last_line_defined = self.load_int();
            (*prototype).prototype_count_parameters = self.load_byte();
            (*prototype).prototype_is_variable_arguments = 0 != self.load_byte();
            (*prototype).prototype_maximum_stack_size = self.load_byte();
            self.load_code(prototype);
            self.load_constants(prototype);
            self.load_upvalues(prototype);
            self.load_prototypes(prototype);
            self.load_debug(prototype);
        }
    }
    pub unsafe extern "C" fn check_literal(
        & mut self,
        s: *const i8,
        message: *const i8,
    ) {
        unsafe {
            let mut buffer: [i8; 12] = [0; 12];
            let length: u64 = strlen(s);
            self.load_block(
                buffer.as_mut_ptr() as *mut libc::c_void,
                length.wrapping_mul(::core::mem::size_of::<i8>() as u64),
            );
            if memcmp(
                s as *const libc::c_void,
                buffer.as_mut_ptr() as *const libc::c_void,
                length,
            ) != 0
            {
                self.error(message);
            }
        }
    }
    pub unsafe extern "C" fn f_check_size(& mut self, size: u64, tname: *const i8) {
        unsafe {
            if self.load_byte() as u64 != size {
                self.error(
                    luao_pushfstring(
                        self.interpreter,
                        b"%s size mismatch\0" as *const u8 as *const i8,
                        tname,
                    ),
                );
            }
        }
    }
    pub unsafe extern "C" fn check_header(& mut self) {
        unsafe {
            self.check_literal(
                &*(b"\x1BLua\0" as *const u8 as *const i8).offset(1 as isize),
                b"not a binary chunk\0" as *const u8 as *const i8,
            );
            if self.load_byte() as i32
                != 504 as i32 / 100 as i32 * 16 as i32 + 504 as i32 % 100 as i32
            {
                self.error(b"version mismatch\0" as *const u8 as *const i8);
            }
            if self.load_byte() as i32 != 0 {
                self.error(b"format mismatch\0" as *const u8 as *const i8);
            }
            self.check_literal(
                b"\x19\x93\r\n\x1A\n\0" as *const u8 as *const i8,
                b"corrupted chunk\0" as *const u8 as *const i8,
            );
            self.f_check_size(
                ::core::mem::size_of::<u32>() as u64,
                b"u32\0" as *const u8 as *const i8,
            );
            self.f_check_size(
                ::core::mem::size_of::<i64>() as u64,
                b"i64\0" as *const u8 as *const i8,
            );
            self.f_check_size(
                ::core::mem::size_of::<f64>() as u64,
                b"f64\0" as *const u8 as *const i8,
            );
            if self.load_integer() != 0x5678 as i64 {
                self.error(
                    b"integer format mismatch\0" as *const u8 as *const i8,
                );
            }
            if self.load_number() != 370.5f64 {
                self.error(
                    b"float format mismatch\0" as *const u8 as *const i8,
                );
            }
        }
    }
}
pub unsafe extern "C" fn load_closure(
    interpreter: *mut Interpreter,
    zio: *mut ZIO,
    name: *const i8,
) -> *mut Closure {
    unsafe {
        let mut load_state: LoadState = LoadState {
            interpreter: null_mut(),
            zio: null_mut(),
            name: null(),
        };
        if *name as i32 == CHARACTER_AT as i32 || *name as i32 == CHARACTER_EQUAL as i32 {
            load_state.name = name.offset(1 as isize);
        } else if *name as i32
            == (*::core::mem::transmute::<&[u8; 5], &[i8; 5]>(b"\x1BLua\0"))[0] as i32
        {
            load_state.name = b"binary string\0" as *const u8 as *const i8;
        } else {
            load_state.name = name;
        }
        load_state.interpreter = interpreter;
        load_state.zio = zio;
        load_state.check_header();
        let ret: *mut Closure = luaf_newlclosure(interpreter, load_state.load_byte() as i32);
        let io: *mut TValue = &mut (*(*interpreter).top.stkidrel_pointer).tvalue;
        (*io).value.object = &mut (*(ret as *mut Object));
        (*io).set_tag_variant(TAG_VARIANT_CLOSURE_L);
        (*io).set_collectable(true);
        (*interpreter).luad_inctop();
        (*ret).payload.l_prototype = luaf_newproto(interpreter);
        if (*ret).get_marked() & 1 << 5 != 0 && (*(*ret).payload.l_prototype).get_marked() & (1 << 3 | 1 << 4) != 0 {
            luac_barrier_(
                interpreter,
                &mut (*(ret as *mut Object)),
                &mut (*((*ret).payload.l_prototype as *mut Object)),
            );
        } else {
        };
        load_state.load_function((*ret).payload.l_prototype, null_mut());
        return ret;
    }
}
