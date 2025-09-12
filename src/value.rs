use crate::functions::*;
use crate::object::*;
use crate::tstring::*;
use std::ptr::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub union Value {
    pub object: *mut Object,
    pub pointer: *mut libc::c_void,
    pub function: CFunction,
    pub integer: i64,
    pub number: f64,
    pub tstring: *mut TString,
    pub info: i32,
    pub index: ValueReference,
    pub variable: ValueRegister,
}
impl Value {
    pub const fn new() -> Self {
        Value { object: null_mut() }
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ValueRegister {
    pub register_index: u8,
    pub value_index: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ValueReference {
    pub reference_tag: u8,
    pub reference_dummy0: u8,
    pub reference_index: i16,
}
