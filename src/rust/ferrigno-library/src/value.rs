use crate::functions::*;
use crate::object::*;
use crate::tstring::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub union Value {
    pub value_object: *mut Object,
    pub value_pointer: *mut std::ffi::c_void,
    pub value_function: CFunction,
    pub value_integer: i64,
    pub value_number: f64,
    pub value_tstring: *mut TString,
    pub value_info: i32,
    pub value_index: ValueReference,
    pub value_variable: ValueRegister,
}
impl Value {
    pub const fn new_object(object: *mut Object) -> Self {
        Value { value_object: object }
    }
    pub const fn new_integer(integer: i64) -> Self {
        Value { value_integer: integer }
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ValueRegister {
    pub valueregister_registerindex: u8,
    pub valueregister_valueindex: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ValueReference {
    pub valuereference_tag: u8,
    pub valuereference_readonly: u8,
    pub valuereference_index: i16,
    pub valuereference_keystr: i32,
}
