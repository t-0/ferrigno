#![allow(dead_code)]
use crate::functions::*;
use crate::global::*;
use crate::object::*;
use crate::interpreter::*;
use crate::table::*;
use crate::tag::*;
use crate::tvalue::*;
use crate::utility::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct User {
    pub object: Object,
    pub gc_list: *mut Object,
    pub metatable: *mut Table,
    pub count_bytes: usize,
    pub count_upvalues: i32,
    pub upvalues: [TValue; 0],
}
impl TObject for User {
    fn as_object(&self) -> &Object {
        &self.object
    }
    fn as_object_mut(&mut self) -> &mut Object {
        &mut self.object
    }
    fn get_class_name(&mut self) -> String {
        "user".to_string()
    }
    fn get_metatable(&mut self) -> *mut Table {
        self.metatable
    }
}
impl User {
    pub fn user_get_size(count_bytes: usize, count_upvalues: usize) -> usize {
        core::mem::size_of::<User>()
            + ::core::mem::size_of::<TValue>() * count_upvalues
            + count_bytes
    }
    pub fn user_get_offset(count_upvalues: usize) -> isize {
        (core::mem::offset_of!(User, upvalues) + ::core::mem::size_of::<TValue>() * count_upvalues)
            as isize
    }
    pub fn get_size(&self) -> usize {
        return User::user_get_size(self.count_bytes as usize, self.count_upvalues as usize);
    }
    pub unsafe fn get_raw_memory(&self) -> *const libc::c_void {
        unsafe {
            return (self as *const User as *mut i8)
                .offset(User::user_get_offset((*self).count_upvalues as usize))
                as *const libc::c_void;
        }
    }
    pub unsafe fn get_raw_memory_mut(&mut self) -> *mut libc::c_void {
        unsafe {
            return (self as *mut User as *mut i8)
                .offset(User::user_get_offset((*self).count_upvalues as usize))
                as *mut libc::c_void;
        }
    }
    pub unsafe extern "C" fn luas_newudata(
        interpreter: *mut Interpreter,
        count_bytes: usize,
        count_upvalues: usize,
    ) -> *mut User {
        unsafe {
            if count_bytes > MAXIMUM_SIZE - User::user_get_size(0, count_upvalues) {
                (*interpreter).too_big();
            }
            let object: *mut Object = luac_newobj(
                interpreter,
                TAG_VARIANT_USER,
                User::user_get_size(count_bytes, count_upvalues),
            );
            let ret: *mut User = &mut (*(object as *mut User));
            (*ret).count_bytes = count_bytes;
            (*ret).count_upvalues = count_upvalues as i32;
            (*ret).metatable = std::ptr::null_mut();
            for i in 0..count_upvalues {
                (*((*ret).upvalues).as_mut_ptr().offset(i as isize)).set_tag(TagVariant::NilNil as u8);
            }
            return ret;
        }
    }
    pub unsafe extern "C" fn lua_newuserdatauv(
        interpreter: *mut Interpreter,
        size: usize,
        count_upvalues: usize,
    ) -> *mut libc::c_void {
        unsafe {
            let new_user: *mut User = User::luas_newudata(interpreter, size, count_upvalues);
            let io: *mut TValue = &mut (*(*interpreter).top.stkidrel_pointer).tvalue;
            (*io).value.object = &mut (*(new_user as *mut Object));
            (*io).set_tag(TAG_VARIANT_USER);
            (*io).set_collectable(true);
            (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
            if (*(*interpreter).global).gc_debt > 0 {
                luac_step(interpreter);
            }
            return (*new_user).get_raw_memory_mut();
        }
    }
    pub unsafe extern "C" fn touserdata(o: *const TValue) -> *mut libc::c_void {
        unsafe {
            match get_tag_type((*o).get_tag2()) {
                TagType::User => {
                    return (*((*o).value.object as *mut User)).get_raw_memory_mut();
                }
                TagType::Pointer => return (*o).value.pointer,
                _ => return std::ptr::null_mut(),
            };
        }
    }
    pub unsafe extern "C" fn lua_topointer(interpreter: *mut Interpreter, index: i32) -> *const libc::c_void {
        unsafe {
            let o: *const TValue = (*interpreter).index2value(index);
            match (*o).get_tag_variant() {
                TAG_VARIANT_CLOSURE_CFUNCTION => {
                    return ::core::mem::transmute::<CFunction, u64>((*o).value.function)
                        as *mut libc::c_void;
                }
                TAG_VARIANT_USER | TAG_VARIANT_POINTER => return User::touserdata(o),
                _ => {
                    if (*o).is_collectable() {
                        return (*o).value.object as *const libc::c_void;
                    } else {
                        return std::ptr::null();
                    }
                }
            };
        }
    }
    pub unsafe extern "C" fn free_user(&mut self, interpreter: *mut Interpreter) {
        unsafe {
            (*interpreter).free_memory(self as *mut User as *mut libc::c_void, self.get_size());
        }
    }
    pub unsafe extern "C" fn traverseudata(&mut self, global: *mut Global) -> i32 {
        unsafe {
            if !self.get_metatable().is_null() {
                if (*self.get_metatable()).get_marked() & (1 << 3 | 1 << 4) != 0 {
                    really_mark_object(global, &mut (*(self.get_metatable() as *mut Object)));
                }
            }
            for i in 0..self.count_upvalues {
                if ((*(self.upvalues).as_mut_ptr().offset(i as isize)).is_collectable())
                    && (*(*(self.upvalues).as_mut_ptr().offset(i as isize)).value.object).get_marked()
                        & (1 << 3 | 1 << 4)
                        != 0
                {
                    really_mark_object(
                        global,
                        (*(self.upvalues).as_mut_ptr().offset(i as isize)).value.object,
                    );
                }
            }
            generate_link(
                global,
                &mut (*(self as *mut User as *mut libc::c_void as *mut Object)),
            );
            return 1 + self.count_upvalues as i32;
        }
    }
}
