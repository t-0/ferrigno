#![allow(dead_code)]
use crate::global::*;
use crate::object::*;
use crate::objectwithgclist::*;
use crate::objectwithmetatable::*;
use crate::state::*;
use crate::table::*;
use crate::tagvariant::*;
use crate::tobject::*;
use crate::tobjectwithgclist::*;
use crate::tobjectwithmetatable::*;
use crate::tvalue::*;
use crate::utility::*;
use std::ptr::*;
type UserSuper = ObjectWithMetatable;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct User {
    user_super: UserSuper,
    pub user_countbytes: usize,
    pub user_countupvalues: i32,
    pub user_upvalues: [TValue; 0],
}
impl TObject for User {
    fn as_object(&self) -> &Object {
        self.user_super.as_object()
    }
    fn as_object_mut(&mut self) -> &mut Object {
        self.user_super.as_object_mut()
    }
}
impl TObjectWithGCList for User {
    fn getgclist(&mut self) -> *mut *mut ObjectWithGCList {
        self.user_super.getgclist()
    }
}
impl TObjectWithMetatable for User {
    fn get_metatable(&self) -> *mut Table {
        self.user_super.get_metatable()
    }
    fn set_metatable(&mut self, metatable: *mut Table) {
        self.user_super.set_metatable(metatable);
    }
}
impl User {
    pub fn get_length_raw(&self) -> usize {
        self.user_countupvalues as usize
    }
    pub fn user_get_size(count_bytes: usize, count_upvalues: usize) -> usize {
        core::mem::size_of::<User>() + size_of::<TValue>() * count_upvalues + count_bytes
    }
    pub fn user_get_offset(count_upvalues: usize) -> isize {
        (core::mem::offset_of!(User, user_upvalues) + size_of::<TValue>() * count_upvalues) as isize
    }
    pub fn get_size(&self) -> usize {
        User::user_get_size(self.user_countbytes, self.user_countupvalues as usize)
    }
    pub unsafe fn get_raw_memory(&self) -> *const std::ffi::c_void {
        unsafe {
            (self as *const User as *mut i8).add(User::user_get_offset(self.user_countupvalues as usize) as usize)
                as *const std::ffi::c_void
        }
    }
    pub unsafe fn get_raw_memory_mut(&mut self) -> *mut std::ffi::c_void {
        unsafe {
            (self as *mut User as *mut i8).add(User::user_get_offset(self.user_countupvalues as usize) as usize)
                as *mut std::ffi::c_void
        }
    }
    pub unsafe fn luas_newudata(state: *mut State, count_bytes: usize, count_upvalues: usize) -> *mut User {
        unsafe {
            if count_bytes > MAXIMUM_SIZE - User::user_get_size(0, count_upvalues) {
                (*state).too_big();
            }
            let object: *mut Object = luac_newobj(
                state,
                TagVariant::User,
                User::user_get_size(count_bytes, count_upvalues),
            );
            let ret: *mut User = &mut *(object as *mut User);
            (*ret).user_countbytes = count_bytes;
            (*ret).user_countupvalues = count_upvalues as i32;
            (*ret).set_metatable(null_mut());
            for i in 0..count_upvalues {
                (*((*ret).user_upvalues).as_mut_ptr().add(i)).tvalue_set_tag_variant(TagVariant::NilNil);
            }
            ret
        }
    }
    pub unsafe fn lua_newuserdatauv(state: *mut State, size: usize, count_upvalues: usize) -> *mut std::ffi::c_void {
        unsafe {
            let new_user: *mut User = User::luas_newudata(state, size, count_upvalues);
            let io: *mut TValue = &mut (*(*state).interpreter_top.stkidrel_pointer);
            (*io).set_object(new_user as *mut Object, TagVariant::User);
            (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.add(1);
            (*state).do_gc_step_if_should_step();
            (*new_user).get_raw_memory_mut()
        }
    }
    pub unsafe fn user_free(&mut self, state: *mut State) {
        unsafe {
            (*state).free_memory(self as *mut User as *mut std::ffi::c_void, self.get_size());
        }
    }
    pub unsafe fn traverseudata(&mut self, global: *mut Global) -> i32 {
        unsafe {
            if !self.get_metatable().is_null() && (*self.get_metatable()).get_marked() & WHITEBITS != 0 {
                Object::really_mark_object(global, &mut *(self.get_metatable() as *mut Object));
            }
            for i in 0..self.user_countupvalues {
                let uv = &*(self.user_upvalues).as_mut_ptr().add(i as usize);
                if let Some(obj) = uv.as_object()
                    && (*obj).get_marked() & WHITEBITS != 0
                {
                    Object::really_mark_object(global, obj);
                }
            }
            Object::generate_link(
                global,
                &mut *(self as *mut User as *mut std::ffi::c_void as *mut Object),
            );
            1 + self.user_countupvalues
        }
    }
}
