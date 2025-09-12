use crate::interpreter::*;
use crate::object::*;
use crate::stackvalue::*;
use crate::table::*;
use crate::tag::*;
use crate::tvalue::*;
use std::ptr::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UpValue {
    pub object: Object,
    pub v: UpValueA,
    pub u: UpValueB,
}
impl TObject for UpValue {
    fn as_object(&self) -> &Object {
        &self.object
    }
    fn as_object_mut(&mut self) -> &mut Object {
        &mut self.object
    }
    fn get_class_name(&mut self) -> String {
        "upvalue".to_string()
    }
    fn get_metatable(&mut self) -> *mut Table {
        null_mut()
    }
}
impl UpValue {
    pub unsafe extern "C" fn free_upvalue(&mut self, interpreter: *mut Interpreter) {
        unsafe {
            if self.v.p != &mut self.u.value as *mut TValue {
                luaf_unlinkupval(self);
            }
            (*interpreter).free_memory(
                self as *mut UpValue as *mut libc::c_void,
                size_of::<UpValue>(),
            );
        }
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union UpValueA {
    pub p: *mut TValue,
    pub offset: i64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union UpValueB {
    pub open: UpValueBA,
    pub value: TValue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UpValueBA {
    pub next: *mut UpValue,
    pub previous: *mut *mut UpValue,
}
pub unsafe extern "C" fn newupval(
    interpreter: *mut Interpreter,
    level: StackValuePointer,
    previous: *mut *mut UpValue,
) -> *mut UpValue {
    unsafe {
        let o: *mut Object = luac_newobj(interpreter, TAG_VARIANT_UPVALUE, size_of::<UpValue>());
        let uv: *mut UpValue = &mut (*(o as *mut UpValue));
        let next: *mut UpValue = *previous;
        (*uv).v.p = &mut (*level).tvalue;
        (*uv).u.open.next = next;
        (*uv).u.open.previous = previous;
        if !next.is_null() {
            (*next).u.open.previous = &mut (*uv).u.open.next;
        }
        *previous = uv;
        if !((*interpreter).twups != interpreter) {
            (*interpreter).twups = (*(*interpreter).global).twups;
            (*(*interpreter).global).twups = interpreter;
        }
        return uv;
    }
}
pub unsafe extern "C" fn luaf_findupval(
    interpreter: *mut Interpreter,
    level: StackValuePointer,
) -> *mut UpValue {
    unsafe {
        let mut pp: *mut *mut UpValue = &mut (*interpreter).open_upvalue;
        loop {
            let p: *mut UpValue = *pp;
            if !(!p.is_null() && (*p).v.p as StackValuePointer >= level) {
                break;
            }
            if (*p).v.p as StackValuePointer == level {
                return p;
            }
            pp = &mut (*p).u.open.next;
        }
        return newupval(interpreter, level, pp);
    }
}
pub unsafe extern "C" fn luaf_unlinkupval(uv: *mut UpValue) {
    unsafe {
        *(*uv).u.open.previous = (*uv).u.open.next;
        if !((*uv).u.open.next).is_null() {
            (*(*uv).u.open.next).u.open.previous = (*uv).u.open.previous;
        }
    }
}
