use crate::object::*;
use crate::state::*;
use crate::tagvariant::*;
use crate::tobject::*;
use crate::tvalue::*;
type UpValueSuper = Object;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UpValue {
    upvalue_super: UpValueSuper,
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
}
impl UpValue {
    pub unsafe fn upvalue_free(&mut self, state: *mut State) {
        unsafe {
            let is_open = self.upvalue_v.upvaluea_p != std::ptr::addr_of_mut!(self.upvalue_u.upvalueb_value);
            if is_open {
                self.luaf_unlinkupval();
            }
            (*state).free_memory(
                self as *mut UpValue as *mut std::ffi::c_void,
                size_of::<UpValue>(),
            );
        }
    }
    pub unsafe fn newupval(state: *mut State, level: *mut TValue, previous: *mut *mut UpValue) -> *mut UpValue {
        unsafe {
            let o: *mut Object = luac_newobj(state, TagVariant::UpValue, size_of::<UpValue>());
            let uv: *mut UpValue = &mut *(o as *mut UpValue);
            let next: *mut UpValue = *previous;
            (*uv).upvalue_v.upvaluea_p = &mut (*level);
            (*uv).upvalue_u.upvalueb_open.upvalueba_next = next;
            (*uv).upvalue_u.upvalueb_open.upvalueba_previous = previous;
            if !next.is_null() {
                (*next).upvalue_u.upvalueb_open.upvalueba_previous = &mut (*uv).upvalue_u.upvalueb_open.upvalueba_next;
            }
            *previous = uv;
            if (*state).interpreter_twups == state {
                (*state).interpreter_twups = (*(*state).interpreter_global).global_twups;
                (*(*state).interpreter_global).global_twups = state;
            }
            uv
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
pub unsafe fn luaf_findupval(state: *mut State, level: *mut TValue) -> *mut UpValue {
    unsafe {
        let mut pp: *mut *mut UpValue = &mut (*state).interpreter_open_upvalue;
        loop {
            let p: *mut UpValue = *pp;
            if !(!p.is_null() && (*p).upvalue_v.upvaluea_p as *mut TValue >= level) {
                break;
            }
            if std::ptr::eq((*p).upvalue_v.upvaluea_p, level) {
                return p;
            }
            pp = &mut (*p).upvalue_u.upvalueb_open.upvalueba_next;
        }
        UpValue::newupval(state, level, pp)
    }
}
impl UpValue {
    pub unsafe fn luaf_unlinkupval(&mut self) {
        unsafe {
            let next = std::ptr::read_volatile(&self.upvalue_u.upvalueb_open.upvalueba_next);
            let prev = std::ptr::read_volatile(&self.upvalue_u.upvalueb_open.upvalueba_previous);
            *prev = next;
            if !next.is_null() {
                (*next).upvalue_u.upvalueb_open.upvalueba_previous = prev;
            }
        }
    }
}
