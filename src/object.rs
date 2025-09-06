#![allow(dead_code, unused)]
// #[macro_export]
// macro_rules! ObjectBase {
//     (#[derive($($derive:meta),*)] $pub:vis struct $name:ident { $($fpub:vis $field:ident : $type:ty,)* }) => {
//         #[derive($($derive),*)]
//         #[repr(C)]
//         $pub struct $name {
//             pub next: *mut Object,
//             pub tag: u8,
//             pub marked: u8,
//             $($fpub $field : $type,)*
//         }
//     }
// }
use crate::global::*;
use crate::table::*;
use crate::tag::*;
use crate::tstring::*;
use crate::closure::*;
use crate::tvalue::*;
use crate::upvalue::*;
use crate::closure::*;
use crate::state::*;
use crate::prototype::*;
use crate::user::*;
pub trait TObject {
    fn get_tag(&self) -> u8;
    fn set_tag(&mut self, tag: u8);
    fn get_marked(&self) -> u8;
    fn set_marked(&mut self, marked_: u8);
    fn is_collectable(&self) -> bool {
        is_collectable(self.get_tag())
    }
    fn set_collectable(&mut self) {
        self.set_tag(set_collectable(self.get_tag()));
    }
    fn get_tag_type(&self) -> u8 {
        get_tag_type(self.get_tag())
    }
    fn get_tag_variant(&self) -> u8 {
        get_tag_variant(self.get_tag())
    }
    fn get_class_name(&mut self) -> String;
    fn get_metatable(&mut self) -> *mut Table;
}
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Object {
    pub next: *mut Object = std::ptr::null_mut(),
    pub tag: u8 = TAG_VARIANT_NIL_NIL,
    pub marked: u8 = 0,
    pub _dummy0: u16 = 0,
    pub _dummy1: u32 = 0,
}
impl TObject for Object {
    fn get_marked(&self) -> u8 {
        self.marked
    }
    fn set_marked(&mut self, marked_: u8) {
        self.marked = marked_;
    }
    fn get_tag(&self) -> u8 {
        return self.tag;
    }
    fn set_tag(&mut self, tag: u8) {
        self.tag = tag;
    }
    fn get_class_name(&mut self) -> String {
        "object".to_string()
    }
    fn get_metatable(&mut self) -> *mut Table {
        std::ptr::null_mut()
    }
}
pub unsafe extern "C" fn getgclist(o: *mut Object) -> *mut *mut Object {
    unsafe {
        match (*o).get_tag() {
            TAG_VARIANT_TABLE => return &mut (*(o as *mut Table)).gc_list,
            TAG_VARIANT_CLOSURE_L => return &mut (*(o as *mut Closure)).gc_list,
            TAG_VARIANT_CLOSURE_C => return &mut (*(o as *mut Closure)).gc_list,
            TAG_VARIANT_STATE => return &mut (*(o as *mut State)).gc_list,
            TAG_VARIANT_PROTOTYPE => return &mut (*(o as *mut Prototype)).gc_list,
            TAG_VARIANT_USER => return &mut (*(o as *mut User)).gc_list,
            _ => return std::ptr::null_mut(),
        };
    }
}
pub unsafe extern "C" fn linkgclist_(
    o: *mut Object,
    pnext: *mut *mut Object,
    list: *mut *mut Object,
) {
    unsafe {
        *pnext = *list;
        *list = o;
        (*o).set_marked((*o).get_marked() & !(1 << 5 | (1 << 3 | 1 << 4)));
    }
}
pub unsafe extern "C" fn iscleared(global: *mut Global, o: *const Object) -> i32 {
    unsafe {
        if o.is_null() {
            return 0;
        } else if get_tag_type((*o).get_tag()) == TAG_TYPE_STRING {
            if (*o).get_marked() & (1 << 3 | 1 << 4) != 0 {
                reallymarkobject(global, &mut (*(o as *mut Object)));
            }
            return 0;
        } else {
            return ((*o).get_marked() & (1 << 3 | 1 << 4)) as i32;
        };
    }
}
pub unsafe extern "C" fn luac_barrier_(state: *mut State, o: *mut Object, v: *mut Object) {
    unsafe {
        let global: *mut Global = (*state).global;
        if (*global).gc_state as i32 <= 2 {
            reallymarkobject(global, v);
            if (*o).get_marked() & 7 > 1 {
                (*v).set_marked((*v).get_marked() & !(7) | 2);
            }
        } else if (*global).gc_kind as i32 == 0 {
            (*o).set_marked(
                (*o).get_marked() & !(1 << 5 | (1 << 3 | 1 << 4))
                    | ((*global).current_white & (1 << 3 | 1 << 4)),
            );
        }
    }
}
pub unsafe extern "C" fn luac_barrierback_(state: *mut State, o: *mut Object) {
    unsafe {
        let global: *mut Global = (*state).global;
        if (*o).get_marked() & 7 == 6 {
            (*o).set_marked((*o).get_marked() & !(1 << 5 | (1 << 3 | 1 << 4)));
        } else {
            linkgclist_(
                &mut (*(o as *mut Object)),
                getgclist(o),
                &mut (*global).gray_again,
            );
        }
        if (*o).get_marked() & 7 > 1 {
            (*o).set_marked((*o).get_marked() & !7 | 5);
        }
    }
}
pub unsafe extern "C" fn luac_fix(state: *mut State, o: *mut Object) {
    unsafe {
        let global: *mut Global = (*state).global;
        (*o).set_marked((*o).get_marked() & !(1 << 5 | (1 << 3 | 1 << 4)));
        (*o).set_marked((*o).get_marked() & !(7) | 4);
        (*global).all_gc = (*o).next;
        (*o).next = (*global).fixed_gc;
        (*global).fixed_gc = o;
    }
}
pub unsafe extern "C" fn reallymarkobject(global: *mut Global, o: *mut Object) {
    unsafe {
        let current_block_18: u64;
        match (*o).get_tag() {
            TAG_VARIANT_STRING_SHORT | TAG_VARIANT_STRING_LONG => {
                (*o).set_marked((*o).get_marked() & !(1 << 3 | 1 << 4) | 1 << 5);
                current_block_18 = 18317007320854588510;
            }
            TAG_VARIANT_UPVALUE => {
                let uv: *mut UpValue = &mut (*(o as *mut UpValue));
                if (*uv).v.p != &mut (*uv).u.value as *mut TValue {
                    (*uv).set_marked((*uv).get_marked() & !(1 << 5 | (1 << 3 | 1 << 4)));
                } else {
                    (*uv).set_marked(((*uv).get_marked() & !(1 << 3 | 1 << 4) | 1 << 5) as u8);
                }
                if ((*(*uv).v.p).is_collectable())
                    && (*(*(*uv).v.p).value.object).get_marked() & (1 << 3 | 1 << 4) != 0
                {
                    reallymarkobject(global, (*(*uv).v.p).value.object);
                }
                current_block_18 = 18317007320854588510;
            }
            TAG_VARIANT_USER => {
                let u: *mut User = &mut (*(o as *mut User));
                if (*u).nuvalue as i32 == 0 {
                    if !((*u).metatable).is_null() {
                        if (*(*u).metatable).get_marked() & (1 << 3 | 1 << 4) != 0 {
                            reallymarkobject(global, &mut (*((*u).metatable as *mut Object)));
                        }
                    }
                    (*u).set_marked((*u).get_marked() & !(1 << 3 | 1 << 4) | 1 << 5);
                    current_block_18 = 18317007320854588510;
                } else {
                    current_block_18 = 15904375183555213903;
                }
            }
            TAG_VARIANT_CLOSURE_L
            | TAG_VARIANT_CLOSURE_C
            | TAG_VARIANT_TABLE
            | TAG_VARIANT_STATE
            | TAG_VARIANT_PROTOTYPE => {
                current_block_18 = 15904375183555213903;
            }
            _ => {
                current_block_18 = 18317007320854588510;
            }
        }
        match current_block_18 {
            15904375183555213903 => {
                linkgclist_(
                    &mut (*(o as *mut Object)),
                    getgclist(o),
                    &mut (*global).gray,
                );
            }
            _ => {}
        };
    }
}
pub unsafe extern "C" fn genlink(global: *mut Global, o: *mut Object) {
    unsafe {
        if (*o).get_marked() & 7 == 5 {
            linkgclist_(
                &mut (*(o as *mut Object)),
                getgclist(o),
                &mut (*global).gray_again,
            );
        } else if (*o).get_marked() & 7 == 6 {
            (*o).set_marked(((*o).get_marked() ^ (6 ^ 4)) as u8);
        }
    }
}
pub unsafe extern "C" fn freeobj(state: *mut State, o: *mut Object) {
    unsafe {
        match (*o).get_tag() {
            TAG_VARIANT_PROTOTYPE => {
                (*(&mut (*(o as *mut Prototype)))).free_prototype(state);
            }
            TAG_VARIANT_UPVALUE => {
                freeupval(state, &mut (*(o as *mut UpValue)));
            }
            TAG_VARIANT_CLOSURE_L => {
                let cl: *mut Closure = &mut (*(o as *mut Closure));
                (*state).free_memory(
                    cl as *mut libc::c_void,
                    (32 as i32
                        + ::core::mem::size_of::<*mut TValue>() as i32
                            * (*cl).count_upvalues as i32) as usize,
                );
            }
            TAG_VARIANT_CLOSURE_C => {
                let cl_0: *mut Closure = &mut (*(o as *mut Closure));
                (*state).free_memory(
                    cl_0 as *mut libc::c_void,
                    (32 as i32
                        + ::core::mem::size_of::<TValue>() as i32
                            * (*cl_0).count_upvalues as i32) as usize,
                );
            }
            TAG_VARIANT_TABLE => {
                luah_free(state, &mut (*(o as *mut Table)));
            }
            TAG_VARIANT_STATE => {
                luae_freethread(state, &mut (*(o as *mut State)));
            }
            TAG_VARIANT_USER => {
                let u: *mut User = &mut (*(o as *mut User));
                (*state).free_memory(
                    o as *mut libc::c_void,
                    (if (*u).nuvalue as i32 == 0 {
                        32 as u64
                    } else {
                        (40 as u64).wrapping_add(
                            (::core::mem::size_of::<TValue>() as u64)
                                .wrapping_mul((*u).nuvalue as u64),
                        )
                    })
                    .wrapping_add((*u).length) as usize,
                );
            }
            TAG_VARIANT_STRING_SHORT => {
                let ts: *mut TString = &mut (*(o as *mut TString));
                (*ts).remove_from_state(state);
                (*state).free_memory(
                    ts as *mut libc::c_void,
                    (24 as u64).wrapping_add(
                        (((*ts).get_length() as i32 + 1) as u64)
                            .wrapping_mul(::core::mem::size_of::<i8>() as u64),
                    ) as usize,
                );
            }
            TAG_VARIANT_STRING_LONG => {
                let ts_0: *mut TString = &mut (*(o as *mut TString));
                (*state).free_memory(
                    ts_0 as *mut libc::c_void,
                    (24 as usize).wrapping_add(
                        ((*ts_0).get_length() as usize)
                            .wrapping_add(1 as usize)
                            .wrapping_mul(::core::mem::size_of::<i8>() as usize),
                    ),
                );
            }
            _ => {}
        };
    }
}
pub unsafe extern "C" fn findlast(mut p: *mut *mut Object) -> *mut *mut Object {
    unsafe {
        while !(*p).is_null() {
            p = &mut (**p).next;
        }
        return p;
    }
}
pub unsafe extern "C" fn checkpointer(p: *mut *mut Object, o: *mut Object) {
    unsafe {
        if o == *p {
            *p = (*o).next;
        }
    }
}
pub unsafe extern "C" fn correctgraylist(mut p: *mut *mut Object) -> *mut *mut Object {
    unsafe {
        let mut current_block: u64;
        loop {
            let curr: *mut Object = *p;
            if curr.is_null() {
                break;
            }
            let next: *mut *mut Object = getgclist(curr);
            if !((*curr).get_marked() & (1 << 3 | 1 << 4) != 0) {
                if (*curr).get_marked() & 7 == 5 {
                    (*curr).set_marked(((*curr).get_marked() | 1 << 5) as u8);
                    (*curr).set_marked(((*curr).get_marked() ^ (5 ^ 6)) as u8);
                    current_block = 11248371660297272285;
                } else if (*curr).get_tag() == TAG_TYPE_STATE {
                    current_block = 11248371660297272285;
                } else {
                    if (*curr).get_marked() & 7 == 6 {
                        (*curr).set_marked(((*curr).get_marked() ^ (6 ^ 4)) as u8);
                    }
                    (*curr).set_marked(((*curr).get_marked() | 1 << 5) as u8);
                    current_block = 6316553219439668466;
                }
                match current_block {
                    6316553219439668466 => {}
                    _ => {
                        p = next;
                        continue;
                    }
                }
            }
            *p = *next;
        }
        return p;
    }
}
pub unsafe extern "C" fn deletelist(state: *mut State, mut p: *mut Object, limit: *mut Object) {
    unsafe {
        while p != limit {
            let next: *mut Object = (*p).next;
            freeobj(state, p);
            p = next;
        }
    }
}
