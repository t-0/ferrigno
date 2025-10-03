use crate::interpreter::*;
use crate::object::*;
use crate::tag::*;
use crate::tobject::*;
use crate::tvalue::*;
pub type UpValueSuper = Object;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UpValue {
    pub upvalue_super: UpValueSuper,
    pub upvalue_v: UpValueA,
    pub upvalue_u: UpValueB,
}
impl TObject for UpValue {
    fn as_object(&self) -> &Object {
        &self.upvalue_super
    }
    fn as_object_mut(&mut self) -> &mut Object {
        &mut self.upvalue_super
    }
    fn get_classname(&mut self) -> String {
        "upvalue".to_string()
    }
}
impl UpValue {
    pub unsafe fn upvalue_free(&mut self, interpreter: *mut Interpreter) {
        unsafe {
            if self.upvalue_v.upvaluea_p != &mut self.upvalue_u.upvalueb_value as *mut TValue {
                luaf_unlinkupval(self);
            }
            (*interpreter).free_memory(self as *mut UpValue as *mut libc::c_void, size_of::<UpValue>());
        }
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union UpValueA {
    pub upvaluea_p: *mut TValue,
    pub upvaluea_offset: i64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union UpValueB {
    pub upvalueb_open: UpValueBA,
    pub upvalueb_value: TValue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UpValueBA {
    pub upvalueba_next: *mut UpValue,
    pub upvalueba_previous: *mut *mut UpValue,
}
pub unsafe fn newupval(interpreter: *mut Interpreter, level: *mut TValue, previous: *mut *mut UpValue) -> *mut UpValue {
    unsafe {
        let o: *mut Object = luac_newobj(interpreter, TagVariant::UpValue, size_of::<UpValue>());
        let uv: *mut UpValue = &mut (*(o as *mut UpValue));
        let next: *mut UpValue = *previous;
        (*uv).upvalue_v.upvaluea_p = &mut (*level);
        (*uv).upvalue_u.upvalueb_open.upvalueba_next = next;
        (*uv).upvalue_u.upvalueb_open.upvalueba_previous = previous;
        if !next.is_null() {
            (*next).upvalue_u.upvalueb_open.upvalueba_previous = &mut (*uv).upvalue_u.upvalueb_open.upvalueba_next;
        }
        *previous = uv;
        if !((*interpreter).interpreter_twups != interpreter) {
            (*interpreter).interpreter_twups = (*(*interpreter).interpreter_global).global_twups;
            (*(*interpreter).interpreter_global).global_twups = interpreter;
        }
        return uv;
    }
}
pub unsafe fn luaf_findupval(interpreter: *mut Interpreter, level: *mut TValue) -> *mut UpValue {
    unsafe {
        let mut pp: *mut *mut UpValue = &mut (*interpreter).interpreter_openupvalue;
        loop {
            let p: *mut UpValue = *pp;
            if !(!p.is_null() && (*p).upvalue_v.upvaluea_p as *mut TValue >= level) {
                break;
            }
            if (*p).upvalue_v.upvaluea_p as *mut TValue == level {
                return p;
            }
            pp = &mut (*p).upvalue_u.upvalueb_open.upvalueba_next;
        }
        return newupval(interpreter, level, pp);
    }
}
pub unsafe fn luaf_unlinkupval(uv: *mut UpValue) {
    unsafe {
        *(*uv).upvalue_u.upvalueb_open.upvalueba_previous = (*uv).upvalue_u.upvalueb_open.upvalueba_next;
        if !((*uv).upvalue_u.upvalueb_open.upvalueba_next).is_null() {
            (*(*uv).upvalue_u.upvalueb_open.upvalueba_next)
                .upvalue_u
                .upvalueb_open
                .upvalueba_previous = (*uv).upvalue_u.upvalueb_open.upvalueba_previous;
        }
    }
}
