use crate::functions::*;
use crate::global::*;
use crate::object::*;
use crate::state::*;
use crate::table::*;
use crate::tag::*;
use crate::tvalue::*;
use crate::utility::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct User {
    pub next: *mut Object,
    pub tag: u8,
    pub marked: u8,
    pub _dummy0: u8 = 0,
    pub _dummy1: u8 = 0,
    //PROBLEM
    pub nuvalue: i32,
    pub length: u64,
    pub metatable: *mut Table,
    pub gc_list: *mut Object,
    pub uv: [TValue; 0],
}
impl TObject for User {
    fn get_tag(&self) -> u8 {
        return self.tag;
    }
    fn set_tag(&mut self, tag: u8) {
        self.tag = tag;
    }
    fn get_marked(&self) -> u8 {
        self.marked
    }
    fn set_marked(&mut self, marked_: u8) {
        self.marked = marked_;
    }
    fn get_class_name(&mut self) -> String {
        "user".to_string()
    }
    fn get_metatable(&mut self) -> *mut Table {
        self.metatable
    }
}
impl User {
    pub fn get_size(count_bytes: usize, count_upvalues: usize) -> usize {
        return core::mem::size_of::<User>().wrapping_add(
            ::core::mem::size_of::<TValue>()
                .wrapping_mul(count_upvalues)
                .wrapping_add(count_bytes),
        );
    }
    pub fn get_offset(count_upvalues: usize) -> isize {
        (core::mem::offset_of!(User, uv)).wrapping_add(::core::mem::size_of::<TValue>().wrapping_mul(count_upvalues)) as isize
    }
    pub unsafe fn get_raw_memory(& mut self) -> *mut libc::c_void {
        unsafe {
            return (self as *mut User as *mut i8).offset(User::get_offset((*self).nuvalue as usize)) as *mut libc::c_void;
        }
    }
    pub unsafe extern "C" fn luas_newudata(
        state: *mut State,
        count_bytes: usize,
        count_upvalues: usize,
    ) -> *mut User {
        unsafe {
            if count_bytes > MAXIMUM_SIZE - User::get_size(0, count_upvalues) {
                (*state).too_big();
            }
            let object: *mut Object = luac_newobj(
                state,
                TAG_TYPE_USER,
                User::get_size(count_bytes, count_upvalues),
            );
            let ret: *mut User = &mut (*(object as *mut User));
            (*ret).length = count_bytes as u64;
            (*ret).nuvalue = count_upvalues as i32;
            (*ret).metatable = std::ptr::null_mut();
            for i in 0..count_upvalues {
                (*((*ret).uv).as_mut_ptr().offset(i as isize)).set_tag(TAG_VARIANT_NIL_NIL);
            }
            return ret;
        }
    }
    pub unsafe extern "C" fn lua_newuserdatauv(
        state: *mut State,
        size: usize,
        count_upvalues: usize,
    ) -> *mut libc::c_void {
        unsafe {
            let new_user: *mut User = User::luas_newudata(state, size, count_upvalues);
            let io: *mut TValue = &mut (*(*state).top.stkidrel_pointer).tvalue;
            (*io).value.object = &mut (*(new_user as *mut Object));
            (*io).set_tag(TAG_VARIANT_USER);
            (*io).set_collectable();
            (*state).top.stkidrel_pointer = (*state).top.stkidrel_pointer.offset(1);
            if (*(*state).global).gc_debt > 0 {
                luac_step(state);
            }
            return (*new_user).get_raw_memory();
        }
    }
    pub unsafe extern "C" fn touserdata(o: *const TValue) -> *mut libc::c_void {
        unsafe {
            match get_tag_type((*o).get_tag()) {
                TAG_VARIANT_USER => return (*((*o).value.object as *mut User)).get_raw_memory(),
                TAG_VARIANT_POINTER => return (*o).value.pointer,
                _ => return std::ptr::null_mut(),
            };
        }
    }
    pub unsafe extern "C" fn lua_topointer(state: *mut State, index: i32) -> *const libc::c_void {
        unsafe {
            let o: *const TValue = (*state).index2value(index);
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
    pub unsafe extern "C" fn free_user(&mut self, state: *mut State) {
        unsafe {
            (*state).free_memory(
                self as *mut User as *mut libc::c_void,
                (if self.nuvalue as i32 == 0 {
                    32 as u64
                } else {
                    (40 as u64).wrapping_add(
                        (::core::mem::size_of::<TValue>() as u64)
                            .wrapping_mul(self.nuvalue as u64),
                    )
                })
                .wrapping_add(self.length) as usize,
            );
        }
    }
    pub unsafe extern "C" fn traverseudata(& mut self, global: *mut Global) -> i32 {
        unsafe {
            if !self.metatable.is_null() {
                if (*self.metatable).get_marked() & (1 << 3 | 1 << 4) != 0 {
                    really_mark_object(global, &mut (*(self.metatable as *mut Object)));
                }
            }
            for i in 0..self.nuvalue {
                if ((*(self.uv).as_mut_ptr().offset(i as isize)).is_collectable())
                    && (*(*(self.uv).as_mut_ptr().offset(i as isize)).value.object).get_marked()
                        & (1 << 3 | 1 << 4)
                        != 0
                {
                    really_mark_object(
                        global,
                        (*(self.uv).as_mut_ptr().offset(i as isize)).value.object,
                    );
                }
            }
            generate_link(global, &mut (*(self as *mut User as *mut libc::c_void as *mut Object)));
            return 1 + self.nuvalue as i32;
        }
    }
}
