use crate::object::*;
use crate::table::*;
use crate::tag::*;
use crate::functions::*;
use crate::uvalue::*;
use crate::tvalue::*;
use crate::state::*;
use crate::onelua::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct User {
    pub next: *mut Object,
    pub tag: u8,
    pub marked: u8,
    pub dummy0: u8 = 0,
    pub dummy1: u8 = 0,
    pub nuvalue: i32,
    pub length: u64,
    pub metatable: *mut Table,
    pub gc_list: *mut Object,
    pub uv: [UValue; 1],
}
impl TObject for User {
    fn get_marked(&self) -> u8 {
        self.marked
    }
    fn set_marked(&mut self, marked_: u8) {
        self.marked = marked_;
    }
    fn set_collectable(&mut self) {
        self.tag = set_collectable(self.tag);
    }
    fn set_tag(&mut self, tag: u8) {
        self.tag = tag;
    }
    fn is_collectable(&self) -> bool {
        return is_collectable(self.tag);
    }
    fn get_tag(&self) -> u8 {
        return self.tag;
    }
    fn get_tag_type(&self) -> u8 {
        return get_tag_type(self.get_tag());
    }
    fn get_tag_variant(&self) -> u8 {
        get_tag_variant(self.get_tag())
    }
    fn get_class_name(&mut self) -> String {
        "user".to_string()
    }
    fn get_metatable(&mut self) -> *mut Table {
        self.metatable
    }
}
impl User {
    pub fn get_size(s: u64, nuvalue: u64) -> u64 {
        return (core::mem::size_of::<User>().wrapping_add(
            ::core::mem::size_of::<UValue>().wrapping_mul(nuvalue as usize).wrapping_add(s as usize))) as u64;
    }
    pub unsafe extern "C" fn luas_newudata(state: *mut State, s: u64, nuvalue: i32) -> *mut User {
        unsafe {
            let mut i: i32;
            if ((s
                > (if (::core::mem::size_of::<u64>() as u64) < ::core::mem::size_of::<i64>() as u64 {
                    !(0u64)
                } else {
                    0x7FFFFFFFFFFFFFFF as i64 as u64
                })
                .wrapping_sub(if nuvalue == 0 {
                    32 as u64
                } else {
                    (40 as u64).wrapping_add(
                        (::core::mem::size_of::<UValue>() as u64).wrapping_mul(nuvalue as u64),
                    )
                })) as i32
                != 0) as i32 as i64
                != 0
            {
                (*state).too_big();
            }
            let o: *mut Object = luac_newobj(
                state,
                TAG_TYPE_USER,
                User::get_size(s, nuvalue as u64),
            );
            let u: *mut User = &mut (*(o as *mut User));
            (*u).length = s;
            (*u).nuvalue = nuvalue;
            (*u).metatable = std::ptr::null_mut();
            i = 0;
            while i < nuvalue {
                (*((*u).uv).as_mut_ptr().offset(i as isize))
                    .uv
                    .set_tag(TAG_VARIANT_NIL_NIL);
                i += 1;
            }
            return u;
        }
    }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn lua_newuserdatauv(
        state: *mut State,
        size: u64,
        nuvalue: i32,
    ) -> *mut libc::c_void {
        unsafe {
            let u: *mut User = User::luas_newudata(state, size, nuvalue);
            let io: *mut TValue = &mut (*(*state).top.p).value;
            let x_: *mut User = u;
            (*io).value.object = &mut (*(x_ as *mut Object));
            (*io).set_tag(TAG_VARIANT_USER);
            (*io).set_collectable();
            (*state).top.p = (*state).top.p.offset(1);
            if (*(*state).global).gc_debt > 0 {
                luac_step(state);
            }
            return (u as *mut i8).offset(
                (if (*u).nuvalue as i32 == 0 {
                    32 as u64
                } else {
                    (40 as u64).wrapping_add(
                        (::core::mem::size_of::<UValue>() as u64).wrapping_mul((*u).nuvalue as u64),
                    )
                }) as isize,
            ) as *mut libc::c_void;
        }
    }
    pub unsafe extern "C" fn touserdata(o: *const TValue) -> *mut libc::c_void {
        unsafe {
            match get_tag_type((*o).get_tag()) {
                TAG_VARIANT_USER => {
                    return (&mut (*((*o).value.object as *mut User)) as *mut User as *mut i8)
                        .offset(
                            (if (*((*o).value.object as *mut User)).nuvalue as i32 == 0 {
                                32 as u64
                            } else {
                                (40 as u64).wrapping_add(
                                    (::core::mem::size_of::<UValue>() as u64).wrapping_mul(
                                        (*((*o).value.object as *mut User)).nuvalue as u64,
                                    ),
                                )
                            }) as isize,
                        ) as *mut libc::c_void;
                }
                TAG_VARIANT_POINTER => return (*o).value.p,
                _ => return std::ptr::null_mut(),
            };
        }
    }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn lua_topointer(state: *mut State, index: i32) -> *const libc::c_void {
        unsafe {
            let o: *const TValue = index2value(state, index);
            match (*o).get_tag_variant() {
                TAG_VARIANT_CLOSURE_CFUNCTION => {
                    return ::core::mem::transmute::<CFunction, u64>((*o).value.f) as *mut libc::c_void;
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
}
