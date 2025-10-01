#![allow(dead_code)]
use crate::global::*;
use crate::interpreter::*;
use crate::object::*;
use crate::table::*;
use crate::tag::*;
use crate::tvalue::*;
use crate::utility::*;
use std::ptr::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct User {
    pub gclist: ObjectWithGCList,
    pub metatable: *mut Table,
    pub count_bytes: usize,
    pub count_upvalues: i32,
    pub upvalues: [TValue; 0],
}
impl TObject for User {
    fn as_object(&self) -> &Object {
        self.gclist.as_object()
    }
    fn as_object_mut(&mut self) -> &mut Object {
        self.gclist.as_object_mut()
    }
    fn get_class_name(&mut self) -> String {
        "user".to_string()
    }
    fn get_metatable(&mut self) -> *mut Table {
        self.metatable
    }
    fn getgclist(& mut self) -> *mut *mut Object {
        self.gclist.getgclist()
    }
}
impl User {
    pub fn user_get_size(count_bytes: usize, count_upvalues: usize) -> usize {
        core::mem::size_of::<User>() + size_of::<TValue>() * count_upvalues + count_bytes
    }
    pub fn user_get_offset(count_upvalues: usize) -> isize {
        (core::mem::offset_of!(User, upvalues) + size_of::<TValue>() * count_upvalues) as isize
    }
    pub fn get_size(&self) -> usize {
        return User::user_get_size(self.count_bytes as usize, self.count_upvalues as usize);
    }
    pub unsafe fn get_raw_memory(&self) -> *const libc::c_void {
        unsafe {
            return (self as *const User as *mut i8).offset(User::user_get_offset((*self).count_upvalues as usize)) as *const libc::c_void;
        }
    }
    pub unsafe fn get_raw_memory_mut(&mut self) -> *mut libc::c_void {
        unsafe {
            return (self as *mut User as *mut i8).offset(User::user_get_offset((*self).count_upvalues as usize)) as *mut libc::c_void;
        }
    }
    pub unsafe fn luas_newudata(interpreter: *mut Interpreter, count_bytes: usize, count_upvalues: usize) -> *mut User {
        unsafe {
            if count_bytes > MAXIMUM_SIZE - User::user_get_size(0, count_upvalues) {
                (*interpreter).too_big();
            }
            let object: *mut Object = luac_newobj(interpreter, TagVariant::User, User::user_get_size(count_bytes, count_upvalues));
            let ret: *mut User = &mut (*(object as *mut User));
            (*ret).count_bytes = count_bytes;
            (*ret).count_upvalues = count_upvalues as i32;
            (*ret).metatable = null_mut();
            for i in 0..count_upvalues {
                (*((*ret).upvalues).as_mut_ptr().offset(i as isize)).set_tag_variant(TagVariant::NilNil);
            }
            return ret;
        }
    }
    pub unsafe fn lua_newuserdatauv(interpreter: *mut Interpreter, size: usize, count_upvalues: usize) -> *mut libc::c_void {
        unsafe {
            let new_user: *mut User = User::luas_newudata(interpreter, size, count_upvalues);
            let io: *mut TValue = &mut (*(*interpreter).top.stkidrel_pointer);
            (*io).value.value_object = &mut (*(new_user as *mut Object));
            (*io).set_tag_variant(TagVariant::User);
            (*io).set_collectable(true);
            (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
            if (*(*interpreter).global).global_gcdebt > 0 {
                (*interpreter).luac_step();
            }
            return (*new_user).get_raw_memory_mut();
        }
    }
    pub unsafe fn free_user(&mut self, interpreter: *mut Interpreter) {
        unsafe {
            (*interpreter).free_memory(self as *mut User as *mut libc::c_void, self.get_size());
        }
    }
    pub unsafe fn traverseudata(&mut self, global: *mut Global) -> i32 {
        unsafe {
            if !self.get_metatable().is_null() {
                if (*self.get_metatable()).get_marked() & (1 << 3 | 1 << 4) != 0 {
                    really_mark_object(global, &mut (*(self.get_metatable() as *mut Object)));
                }
            }
            for i in 0..self.count_upvalues {
                if ((*(self.upvalues).as_mut_ptr().offset(i as isize)).is_collectable()) && (*(*(self.upvalues).as_mut_ptr().offset(i as isize)).value.value_object).get_marked() & (1 << 3 | 1 << 4) != 0 {
                    really_mark_object(global, (*(self.upvalues).as_mut_ptr().offset(i as isize)).value.value_object);
                }
            }
            generate_link(global, &mut (*(self as *mut User as *mut libc::c_void as *mut Object)));
            return 1 + self.count_upvalues;
        }
    }
}
