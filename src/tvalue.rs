use crate::new::*;
use crate::tag::*;
use crate::value::*;
use crate::object::*;
use crate::tstring::*;
use crate::cclosure::*;
use crate::lclosure::*;
use crate::prototype::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TValue {
    pub value: Value,
    pub tag: u8,
}
impl New for TValue {
    fn new() -> Self {
        TValue {
            value: Value::new(),
            tag: TAG_VARIANT_NIL_NIL,
        }
    }
}
impl TValue {
    pub fn set_tag(&mut self, tag: u8) {
        self.tag = tag;
    }
    pub fn set_collectable(&mut self) {
        self.set_tag(set_collectable(self.get_tag()));
    }
    pub fn is_collectable(&self) -> bool {
        return is_collectable(self.get_tag());
    }
    pub fn get_tag(&self) -> u8 {
        self.tag
    }
    pub fn get_tag_type(&self) -> u8 {
        get_tag_type(self.get_tag())
    }
    pub fn get_tag_variant(&self) -> u8 {
        get_tag_variant(self.get_tag())
    }
}
pub unsafe extern "C" fn aux_upvalue(
    fi: *mut TValue,
    n: i32,
    value: *mut *mut TValue,
    owner: *mut *mut Object,
) -> *const i8 {
    unsafe {
        match (*fi).get_tag_variant() {
            TAG_VARIANT_CLOSURE_C => {
                let f: *mut CClosure = &mut (*((*fi).value.object as *mut CClosure));
                if !((n as u32).wrapping_sub(1 as u32) < (*f).count_upvalues as u32) {
                    return std::ptr::null();
                }
                *value = &mut *((*f).upvalue).as_mut_ptr().offset((n - 1) as isize) as *mut TValue;
                if !owner.is_null() {
                    *owner = &mut (*(f as *mut Object));
                }
                return b"\0" as *const u8 as *const i8;
            }
            TAG_VARIANT_CLOSURE_L => {
                let f_0: *mut LClosure = &mut (*((*fi).value.object as *mut LClosure));
                let p: *mut Prototype = (*f_0).p;
                if !((n as u32).wrapping_sub(1 as u32) < (*p).size_upvalues as u32) {
                    return std::ptr::null();
                }
                *value = (**((*f_0).upvalues).as_mut_ptr().offset((n - 1) as isize))
                    .v
                    .p;
                if !owner.is_null() {
                    *owner = &mut (*(*((*f_0).upvalues).as_mut_ptr().offset((n - 1) as isize)
                        as *mut Object));
                }
                let name: *mut TString = (*((*p).upvalues).offset((n - 1) as isize)).name;
                return if name.is_null() {
                    b"(no name)\0" as *const u8 as *const i8
                } else {
                    ((*name).get_contents()) as *const i8
                };
            }
            _ => return std::ptr::null(),
        };
    }
}
