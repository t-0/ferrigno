use libc::memcmp;
use crate::character::*;
use crate::closure::*;
use crate::debugger::absolutelineinfo::*;
use crate::dumpstate::*;
use crate::interpreter::*;
use crate::loadable::*;
use crate::localvariable::*;
use crate::object::*;
use crate::prototype::*;
use crate::tag::*;
use crate::tstring::*;
use crate::tvalue::*;
use crate::upvaluedescription::*;
use crate::utility::c::*;
use crate::zio::*;
use core::mem::*;
use std::ptr::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LoadState {
    interpreter: *mut Interpreter,
    zio: *mut ZIO,
    name: *const i8,
}
impl LoadState {
    pub unsafe fn error(&mut self, why: *const i8) -> ! {
        unsafe {
            luao_pushfstring(self.interpreter, c"%s: bad binary format (%s)".as_ptr(), self.name, why);
            luad_throw(self.interpreter, 3);
        }
    }
    pub unsafe fn load_block(&mut self, b: *mut libc::c_void, size: usize) {
        unsafe {
            if (*self.zio).luaz_read(b, size) != 0 {
                self.error(c"truncated chunk".as_ptr());
            }
        }
    }
    pub unsafe fn load_byte(&mut self) -> u8 {
        unsafe {
            match (*(self.zio)).load_byte() {
                None => {
                    self.error(c"truncated chunk".as_ptr());
                },
                Some(x) => {
                    return x;
                },
            }
        }
    }
    pub unsafe fn load_unsigned(&mut self, mut limit: usize) -> usize {
        unsafe {
            let mut ret: usize = 0;
            limit >>= 7;
            loop {
                let b: i32 = self.load_byte() as i32;
                if ret >= limit {
                    self.error(c"integer overflow".as_ptr());
                }
                ret = ret << 7 | (b & 0x7f as i32) as usize;
                if !(b & 0x80 as i32 == 0) {
                    break;
                }
            }
            return ret;
        }
    }
    pub unsafe fn load_size(&mut self) -> usize {
        unsafe {
            return self.load_unsigned(!0usize);
        }
    }
    pub unsafe fn load_int(&mut self) -> i32 {
        unsafe {
            return self.load_unsigned(0x7FFFFFFF as usize) as i32;
        }
    }
    pub unsafe fn load_number(&mut self) -> f64 {
        unsafe {
            let mut x: f64 = 0.0;
            self.load_block(&mut x as *mut f64 as *mut libc::c_void, 1usize.wrapping_mul(size_of::<f64>() as usize));
            return x;
        }
    }
    pub unsafe fn load_integer(&mut self) -> i64 {
        unsafe {
            let mut x: i64 = 0;
            self.load_block(&mut x as *mut i64 as *mut libc::c_void, 1usize.wrapping_mul(size_of::<i64>() as usize));
            return x;
        }
    }
    pub unsafe fn load_string_n(&mut self, p: *mut Prototype) -> *mut TString {
        unsafe {
            let interpreter: *mut Interpreter = self.interpreter;
            let ts: *mut TString;
            let mut size: usize = self.load_size();
            if size == 0 {
                return null_mut();
            } else {
                size = size.wrapping_sub(1);
                if size <= 40 as usize {
                    let mut buffer: [i8; 40] = [0; 40];
                    self.load_block(buffer.as_mut_ptr() as *mut libc::c_void, size.wrapping_mul(1 as usize));
                    ts = luas_newlstr(interpreter, buffer.as_mut_ptr(), size as usize);
                } else {
                    ts = TString::create_long(interpreter, size as usize);
                    let io: *mut TValue = &mut (*(*interpreter).top.stkidrel_pointer);
                    (*io).value.value_object = &mut (*(ts as *mut Object));
                    (*io).set_tag_variant((*ts).get_tag_variant());
                    (*io).set_collectable(true);
                    (*interpreter).luad_inctop();
                    self.load_block(((*ts).get_contents_mut()) as *mut libc::c_void, size.wrapping_mul(1 as usize));
                    (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(-1);
                }
            }
            if (*p).get_marked() & 1 << 5 != 0 && (*ts).get_marked() & (1 << 3 | 1 << 4) != 0 {
                luac_barrier_(interpreter, &mut (*(p as *mut Object)), &mut (*(ts as *mut Object)));
            } else {
            };
            return ts;
        }
    }
    pub unsafe fn load_string(&mut self, p: *mut Prototype) -> *mut TString {
        unsafe {
            let tstring = self.load_string_n(p);
            if tstring.is_null() {
                self.error(c"bad format for code_constant string".as_ptr());
            }
            return tstring;
        }
    }
    pub unsafe fn load_code(&mut self, prototype: *mut Prototype) {
        unsafe {
            let n = self.load_int();
            if size_of::<i32>() as usize >= size_of::<usize>() as usize && (n as usize) + 1 > ((!0usize) / size_of::<u32>()) {
                (*(self.interpreter)).too_big();
            }
            (*prototype).prototype_code.initialize_size(self.interpreter, (n as usize).wrapping_mul(size_of::<u32>()));
            self.load_block((*prototype).prototype_code.vectort_pointer as *mut libc::c_void, (n as usize).wrapping_mul(size_of::<u32>()) as usize);
        }
    }
    pub unsafe fn load_constants(&mut self, prototype: *mut Prototype) {
        unsafe {
            let n = self.load_int();
            if size_of::<i32>() as usize >= size_of::<usize>() as usize && (n as usize) + 1 > ((!0usize) / size_of::<TValue>()) {
                (*(self.interpreter)).too_big();
            }
            (*prototype).prototype_constants.initialize_size(self.interpreter, n as usize);
            for i in 0..n {
                (*((*prototype).prototype_constants.vectort_pointer).offset(i as isize)).set_tag_variant(TagVariant::NilNil as u8);
            }
            for i in 0..n {
                let tvalue: *mut TValue = &mut *((*prototype).prototype_constants.vectort_pointer).offset(i as isize) as *mut TValue;
                let t = self.load_byte() as u8;
                match t {
                    TAG_VARIANT_NIL_NIL => {
                        (*tvalue).set_tag_variant(TagVariant::NilNil as u8);
                    },
                    TAG_VARIANT_BOOLEAN_FALSE => {
                        (*tvalue).set_tag_variant(TAG_VARIANT_BOOLEAN_FALSE);
                    },
                    TAG_VARIANT_BOOLEAN_TRUE => {
                        (*tvalue).set_tag_variant(TAG_VARIANT_BOOLEAN_TRUE);
                    },
                    TAG_VARIANT_NUMERIC_NUMBER => {
                        let io: *mut TValue = tvalue;
                        (*io).value.value_number = self.load_number();
                        (*io).set_tag_variant(TAG_VARIANT_NUMERIC_NUMBER);
                    },
                    TAG_VARIANT_NUMERIC_INTEGER => {
                        let io_0: *mut TValue = tvalue;
                        (*io_0).value.value_integer = self.load_integer();
                        (*io_0).set_tag_variant(TAG_VARIANT_NUMERIC_INTEGER);
                    },
                    TAG_VARIANT_STRING_SHORT | TAG_VARIANT_STRING_LONG => {
                        let io_1: *mut TValue = tvalue;
                        let ts: *mut TString = self.load_string(prototype);
                        (*io_1).value.value_object = &mut (*(ts as *mut Object));
                        (*io_1).set_tag_variant((*ts).get_tag_variant());
                        (*io_1).set_collectable(true);
                    },
                    _ => {},
                }
            }
        }
    }
    pub unsafe fn load_prototypes(&mut self, prototype: *mut Prototype) {
        unsafe {
            let n = self.load_int();
            if size_of::<i32>() as usize >= size_of::<usize>() as usize && (n as usize) + 1 > ((!0usize) / size_of::<*mut Prototype>()) {
                (*(self.interpreter)).too_big();
            }
            (*prototype).prototype_prototypes.initialize_size(self.interpreter, n as usize);
            for i in 0..n {
                *((*prototype).prototype_prototypes.vectort_pointer).offset(i as isize) = null_mut();
            }
            for i in 0..n {
                *((*prototype).prototype_prototypes.vectort_pointer).offset(i as isize) = luaf_newproto(self.interpreter);
                if (*prototype).get_marked() & 1 << 5 != 0 && (**((*prototype).prototype_prototypes.vectort_pointer).offset(i as isize)).get_marked() & (1 << 3 | 1 << 4) != 0 {
                    luac_barrier_(
                        self.interpreter,
                        &mut (*(prototype as *mut Object)),
                        &mut (*(*((*prototype).prototype_prototypes.vectort_pointer).offset(i as isize) as *mut Object)),
                    );
                } else {
                }
                self.load_function(*((*prototype).prototype_prototypes.vectort_pointer).offset(i as isize), (*prototype).prototype_source);
            }
        }
    }
    pub unsafe fn load_upvalues(&mut self, prototype: *mut Prototype) {
        unsafe {
            let n = self.load_int();
            if size_of::<i32>() as usize >= size_of::<usize>() as usize && (n as usize) + 1 > ((!0usize) / size_of::<UpValueDescription>()) {
                (*(self.interpreter)).too_big();
            }
            (*prototype).prototype_upvalues.initialize_size(self.interpreter, n as usize);
            for i in 0..n {
                let ref mut fresh29 = (*((*prototype).prototype_upvalues.vectort_pointer).offset(i as isize)).upvaluedescription_name;
                *fresh29 = null_mut();
            }
            for i in 0..n {
                let upvalue_description = &mut *((*prototype).prototype_upvalues.at_mut(i as isize));
                upvalue_description.load(self);
            }
        }
    }
    pub unsafe fn load_debug(&mut self, prototype: *mut Prototype) {
        unsafe {
            let mut n = self.load_int();
            if size_of::<i32>() as usize >= size_of::<usize>() as usize && (n as usize) + 1 > (!0usize) {
                (*(self.interpreter)).too_big();
            }
            (*prototype).prototype_line_info.initialize_size(self.interpreter, n as usize);
            self.load_block((*prototype).prototype_line_info.vectort_pointer as *mut libc::c_void, (n as usize).wrapping_mul(1 as usize));
            n = self.load_int();
            if size_of::<i32>() as usize >= size_of::<usize>() as usize && (n as usize) + 1 > ((!0usize) / size_of::<AbsoluteLineInfo>()) {
                (*(self.interpreter)).too_big();
            } else {
            };
            (*prototype).prototype_absolute_line_info.initialize_size(self.interpreter, n as usize);
            for i in 0..n {
                (*((*prototype).prototype_absolute_line_info.vectort_pointer).offset(i as isize)).program_counter = self.load_int();
                (*((*prototype).prototype_absolute_line_info.vectort_pointer).offset(i as isize)).line = self.load_int();
            }
            n = self.load_int();
            if size_of::<i32>() as usize >= size_of::<usize>() as usize && (n as usize) + 1 > ((!0usize) / size_of::<LocalVariable>()) {
                (*(self.interpreter)).too_big();
            } else {
            };
            (*prototype).prototype_local_variables.initialize_size(self.interpreter, n as usize);
            for i in 0..n {
                let ref mut fresh30 = (*((*prototype).prototype_local_variables.at_mut(i as isize))).variable_name;
                *fresh30 = null_mut();
            }
            for i in 0..n {
                let ref mut fresh31 = (*((*prototype).prototype_local_variables.at_mut(i as isize))).variable_name;
                *fresh31 = self.load_string_n(prototype);
                (*((*prototype).prototype_local_variables.at_mut(i as isize))).start_program_counter = self.load_int();
                (*((*prototype).prototype_local_variables.at_mut(i as isize))).end_program_counter = self.load_int();
            }
            n = self.load_int();
            if n != 0 {
                n = (*prototype).prototype_upvalues.get_size() as i32;
            }
            for i in 0..n {
                let ref mut fresh32 = (*((*prototype).prototype_upvalues.vectort_pointer).offset(i as isize)).upvaluedescription_name;
                *fresh32 = self.load_string_n(prototype);
            }
        }
    }
    pub unsafe fn load_function(&mut self, prototype: *mut Prototype, psource: *mut TString) {
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
    pub unsafe fn check_literal(&mut self, s: *const i8, message: *const i8) {
        unsafe {
            let mut buffer: [i8; 12] = [0; 12];
            let length: usize = strlen(s) as usize;
            self.load_block(buffer.as_mut_ptr() as *mut libc::c_void, length.wrapping_mul(1 as usize));
            if memcmp(s as *const libc::c_void, buffer.as_mut_ptr() as *const libc::c_void, length as usize) != 0 {
                self.error(message);
            }
        }
    }
    pub unsafe fn check_header(&mut self) {
        unsafe {
            self.check_literal(&*(LUA_SIGNATURE).offset(1 as isize), c"not a binary chunk".as_ptr());
            if self.load_byte() as i32 != 504 as i32 / 100 as i32 * 16 as i32 + 504 as i32 % 100 as i32 {
                self.error(c"version mismatch".as_ptr());
            }
            if self.load_byte() as i32 != 0 {
                self.error(c"format mismatch".as_ptr());
            }
            self.check_literal(c"\x19\x7F\r\n\x1A\n".as_ptr(), c"corrupted chunk".as_ptr());
            if self.load_integer() != 0x5678 as i64 {
                self.error(c"integer format mismatch".as_ptr());
            }
            if self.load_number() != 370.5f64 {
                self.error(c"float format mismatch".as_ptr());
            }
        }
    }
}
pub unsafe fn load_closure(interpreter: *mut Interpreter, zio: *mut ZIO, name: *const i8) -> *mut Closure {
    unsafe {
        let mut load_state = LoadState { interpreter: null_mut(), zio: null_mut(), name: null() };
        if *name as i32 == Character::At as i32 || *name as i32 == Character::Equal as i32 {
            load_state.name = name.offset(1 as isize);
        } else if *name as i32 == (*::core::mem::transmute::<&[u8; 5], &[i8; 5]>(b"\x1BLua\0"))[0] as i32 {
            load_state.name = c"binary string".as_ptr();
        } else {
            load_state.name = name;
        }
        load_state.interpreter = interpreter;
        load_state.zio = zio;
        load_state.check_header();
        let ret: *mut Closure = luaf_newlclosure(interpreter, load_state.load_byte() as i32);
        let io: *mut TValue = &mut (*(*interpreter).top.stkidrel_pointer);
        (*io).value.value_object = &mut (*(ret as *mut Object));
        (*io).set_tag_variant(TAG_VARIANT_CLOSURE_L);
        (*io).set_collectable(true);
        (*interpreter).luad_inctop();
        (*ret).payload.l_prototype = luaf_newproto(interpreter);
        if (*ret).get_marked() & 1 << 5 != 0 && (*(*ret).payload.l_prototype).get_marked() & (1 << 3 | 1 << 4) != 0 {
            luac_barrier_(interpreter, &mut (*(ret as *mut Object)), &mut (*((*ret).payload.l_prototype as *mut Object)));
        } else {
        };
        load_state.load_function((*ret).payload.l_prototype, null_mut());
        return ret;
    }
}
