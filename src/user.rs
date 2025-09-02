use crate::object::*;
use crate::table::*;
use crate::tag::*;
use crate::functions::*;
use crate::tvalue::*;
use crate::state::*;
use crate::global::*;
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
    pub uv: [TValue; 1],
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
    pub fn get_size(s: u64, nuvalue: u64) -> u64 {
        return (core::mem::size_of::<User>().wrapping_add(
            ::core::mem::size_of::<TValue>().wrapping_mul(nuvalue as usize).wrapping_add(s as usize))) as u64;
    }
    pub unsafe extern "C" fn luas_newudata(state: *mut State, s: u64, nuvalue: i32) -> *mut User {
        unsafe {
            let mut i: i32;
            if ((s
                > (if (::core::mem::size_of::<u64>() as u64) < ::core::mem::size_of::<i64>() as u64 {
                    !(0u64)
                } else {
                    0x7FFFFFFFFFFFFFFF as u64
                })
                .wrapping_sub(if nuvalue == 0 {
                    32 as u64
                } else {
                    (40 as u64).wrapping_add(
                        (::core::mem::size_of::<TValue>() as u64).wrapping_mul(nuvalue as u64),
                    )
                })) as i32
                != 0) as i64
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
                    .set_tag(TAG_VARIANT_NIL_NIL);
                i += 1;
            }
            return u;
        }
    }
    pub unsafe extern "C" fn lua_newuserdatauv(
        state: *mut State,
        size: u64,
        nuvalue: i32,
    ) -> *mut libc::c_void {
        unsafe {
            let u: *mut User = User::luas_newudata(state, size, nuvalue);
            let io: *mut TValue = &mut (*(*state).top.p).tvalue;
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
                        (::core::mem::size_of::<TValue>() as u64).wrapping_mul((*u).nuvalue as u64),
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
                                    (::core::mem::size_of::<TValue>() as u64).wrapping_mul(
                                        (*((*o).value.object as *mut User)).nuvalue as u64,
                                    ),
                                )
                            }) as isize,
                        ) as *mut libc::c_void;
                }
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
                    return ::core::mem::transmute::<CFunction, u64>((*o).value.function) as *mut libc::c_void;
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
pub unsafe extern "C" fn traverseudata(g: *mut Global, u: *mut User) -> i32 {
    unsafe {
        let mut i: i32;
        if !((*u).metatable).is_null() {
            if (*(*u).metatable).get_marked() & (1 << 3 | 1 << 4) != 0 {
                reallymarkobject(g, &mut (*((*u).metatable as *mut Object)));
            }
        }
        i = 0;
        while i < (*u).nuvalue as i32 {
            if ((*((*u).uv).as_mut_ptr().offset(i as isize))
                .is_collectable())
                && (*(*((*u).uv).as_mut_ptr().offset(i as isize)).value.object).get_marked()
                    & (1 << 3 | 1 << 4)
                    != 0
            {
                reallymarkobject(
                    g,
                    (*((*u).uv).as_mut_ptr().offset(i as isize)).value.object,
                );
            }
            i += 1;
        }
        genlink(g, &mut (*(u as *mut Object)));
        return 1 + (*u).nuvalue as i32;
    }
}
