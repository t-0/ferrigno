use crate::object::*;
use crate::table::*;
use crate::tag::*;
use crate::tvalue::*;
use crate::state::*;
use crate::stackvalue::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UpValue {
    pub object: Object,
    pub v: UpValueA,
    pub u: UpValueB,
}
impl TObject for UpValue {
    fn get_tag(&self) -> u8 {
        self.object.tag
    }
    fn set_tag(&mut self, tag: u8) {
        self.object.tag = tag;
    }
    fn get_marked(&self) -> u8 {
        self.object.marked
    }
    fn set_marked(&mut self, marked: u8) {
        self.object.marked = marked;
    }
    fn get_class_name(&mut self) -> String {
        "upvalue".to_string()
    }
    fn get_metatable(&mut self) -> *mut Table {
        std::ptr::null_mut()
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
pub unsafe extern "C" fn freeupval(state: *mut State, uv: *mut UpValue) {
    unsafe {
        if (*uv).v.p != &mut (*uv).u.value as *mut TValue {
            luaf_unlinkupval(uv);
        }
        (*state).free_memory(
            uv as *mut libc::c_void,
            ::core::mem::size_of::<UpValue>() as u64,
        );
    }
}
pub unsafe extern "C" fn newupval(
    state: *mut State,
    level: StackValuePointer,
    previous: *mut *mut UpValue,
) -> *mut UpValue {
    unsafe {
        let o: *mut Object = luac_newobj(
            state,
            TAG_TYPE_UPVALUE,
            ::core::mem::size_of::<UpValue>() as u64,
        );
        let uv: *mut UpValue = &mut (*(o as *mut UpValue));
        let next: *mut UpValue = *previous;
        (*uv).v.p = &mut (*level).tvalue;
        (*uv).u.open.next = next;
        (*uv).u.open.previous = previous;
        if !next.is_null() {
            (*next).u.open.previous = &mut (*uv).u.open.next;
        }
        *previous = uv;
        if !((*state).twups != state) {
            (*state).twups = (*(*state).global).twups;
            (*(*state).global).twups = state;
        }
        return uv;
    }
}
pub unsafe extern "C" fn luaf_findupval(state: *mut State, level: StackValuePointer) -> *mut UpValue {
    unsafe {
        let mut pp: *mut *mut UpValue = &mut (*state).open_upvalue;
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
        return newupval(state, level, pp);
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
