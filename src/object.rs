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
use crate::closure::*;
use crate::closure::*;
use crate::global::*;
use crate::prototype::*;
use crate::state::*;
use crate::table::*;
use crate::tag::*;
use crate::tstring::*;
use crate::tvalue::*;
use crate::upvalue::*;
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
impl Object {
    pub fn new (tag: u8) -> Object {
        Object {
            next: std::ptr::null_mut(),
            tag: tag,
            marked: 0,
            _dummy0: 0,
            _dummy1: 0,
            ..
        }
    }
}
pub unsafe extern "C" fn getgclist(object: *mut Object) -> *mut *mut Object {
    unsafe {
        match (*object).get_tag() {
            TAG_VARIANT_TABLE => return &mut (*(object as *mut Table)).gc_list,
            TAG_VARIANT_CLOSURE_L | TAG_VARIANT_CLOSURE_C => return &mut (*(object as *mut Closure)).gc_list,
            TAG_VARIANT_STATE => return &mut (*(object as *mut State)).gc_list,
            TAG_VARIANT_PROTOTYPE => return &mut (*(object as *mut Prototype)).gc_list,
            TAG_VARIANT_USER => return &mut (*(object as *mut User)).gc_list,
            _ => return std::ptr::null_mut(),
        };
    }
}
pub unsafe extern "C" fn linkgclist_(
    object: *mut Object,
    pnext: *mut *mut Object,
    list: *mut *mut Object,
) {
    unsafe {
        *pnext = *list;
        *list = object;
        (*object).set_marked((*object).get_marked() & !(1 << 5 | (1 << 3 | 1 << 4)));
    }
}
pub unsafe extern "C" fn iscleared(global: *mut Global, object: *const Object) -> i32 {
    unsafe {
        if object.is_null() {
            return 0;
        } else if get_tag_type((*object).get_tag()) == TAG_TYPE_STRING {
            if (*object).get_marked() & (1 << 3 | 1 << 4) != 0 {
                really_mark_object(global, &mut (*(object as *mut Object)));
            }
            return 0;
        } else {
            return ((*object).get_marked() & (1 << 3 | 1 << 4)) as i32;
        };
    }
}
pub unsafe extern "C" fn luac_barrier_(state: *mut State, object: *mut Object, v: *mut Object) {
    unsafe {
        let global: *mut Global = (*state).global;
        if (*global).gc_state as i32 <= 2 {
            really_mark_object(global, v);
            if (*object).get_marked() & 7 > 1 {
                (*v).set_marked((*v).get_marked() & !(7) | 2);
            }
        } else if (*global).gc_kind as i32 == 0 {
            (*object).set_marked(
                (*object).get_marked() & !(1 << 5 | (1 << 3 | 1 << 4))
                    | ((*global).current_white & (1 << 3 | 1 << 4)),
            );
        }
    }
}
pub unsafe extern "C" fn luac_barrierback_(state: *mut State, object: *mut Object) {
    unsafe {
        let global: *mut Global = (*state).global;
        if (*object).get_marked() & 7 == 6 {
            (*object).set_marked((*object).get_marked() & !(1 << 5 | (1 << 3 | 1 << 4)));
        } else {
            linkgclist_(
                &mut (*(object as *mut Object)),
                getgclist(object),
                &mut (*global).gray_again,
            );
        }
        if (*object).get_marked() & 7 > 1 {
            (*object).set_marked((*object).get_marked() & !7 | 5);
        }
    }
}

pub unsafe extern "C" fn fix_memory_error_message_state(state: *mut State) {
    unsafe {
        let global: *mut Global = (*state).global;
        (*global).fix_memory_error_message_global();
    }
}
pub unsafe extern "C" fn fix_object_state(state: *mut State, object: *mut Object) {
    unsafe {
        let global: *mut Global = (*state).global;
        fix_object_global(global, object);
    }
}
pub unsafe extern "C" fn fix_object_global(global: *mut Global, object: *mut Object) {
    unsafe {
        (*object).set_marked((*object).get_marked() & !(1 << 5 | (1 << 3 | 1 << 4)));
        (*object).set_marked((*object).get_marked() & !(7) | 4);
        (*global).all_gc = (*object).next;
        (*object).next = (*global).fixed_gc;
        (*global).fixed_gc = object;
    }
}
pub unsafe extern "C" fn really_mark_object(global: *mut Global, object: *mut Object) {
    unsafe {
        let current_block_18: u64;
        match (*object).get_tag() {
            TAG_VARIANT_STRING_SHORT | TAG_VARIANT_STRING_LONG => {
                (*object).set_marked((*object).get_marked() & !(1 << 3 | 1 << 4) | 1 << 5);
                current_block_18 = 18317007320854588510;
            }
            TAG_VARIANT_UPVALUE => {
                let uv: *mut UpValue = &mut (*(object as *mut UpValue));
                if (*uv).v.p != &mut (*uv).u.value as *mut TValue {
                    (*uv).set_marked((*uv).get_marked() & !(1 << 5 | (1 << 3 | 1 << 4)));
                } else {
                    (*uv).set_marked(((*uv).get_marked() & !(1 << 3 | 1 << 4) | 1 << 5) as u8);
                }
                if ((*(*uv).v.p).is_collectable())
                    && (*(*(*uv).v.p).value.object).get_marked() & (1 << 3 | 1 << 4) != 0
                {
                    really_mark_object(global, (*(*uv).v.p).value.object);
                }
                current_block_18 = 18317007320854588510;
            }
            TAG_VARIANT_USER => {
                let u: *mut User = &mut (*(object as *mut User));
                if (*u).count_upvalues as i32 == 0 {
                    if !((*u).get_metatable()).is_null() {
                        if (*(*u).get_metatable()).get_marked() & (1 << 3 | 1 << 4) != 0 {
                            really_mark_object(global, &mut (*((*u).get_metatable() as *mut Object)));
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
                    &mut (*(object as *mut Object)),
                    getgclist(object),
                    &mut (*global).gray,
                );
            }
            _ => {}
        };
    }
}
pub unsafe extern "C" fn generate_link(global: *mut Global, object: *mut Object) {
    unsafe {
        if (*object).get_marked() & 7 == 5 {
            linkgclist_(
                &mut (*(object as *mut Object)),
                getgclist(object),
                &mut (*global).gray_again,
            );
        } else if (*object).get_marked() & 7 == 6 {
            (*object).set_marked(((*object).get_marked() ^ (6 ^ 4)) as u8);
        }
    }
}
pub unsafe extern "C" fn free_object(state: *mut State, object: *mut Object) {
    unsafe {
        match (*object).get_tag() {
            TAG_VARIANT_PROTOTYPE => {
                let prototype: *mut Prototype = &mut (*(object as *mut Prototype));
                (*prototype).free_prototype(state);
            }
            TAG_VARIANT_UPVALUE => {
                let upvalue: *mut UpValue = &mut (*(object as *mut UpValue));
                (*upvalue).free_upvalue(state);
            }
            TAG_VARIANT_CLOSURE_L | TAG_VARIANT_CLOSURE_C => {
                let closure: *mut Closure = &mut (*(object as *mut Closure));
                (*closure).free_closure(state);
            }
            TAG_VARIANT_TABLE => {
                let table: *mut Table = &mut (*(object as *mut Table));
                (*table).free_table(state);
            }
            TAG_VARIANT_STATE => {
                let other: *mut State = &mut (*(object as *mut State));
                (*other).free_state(state);
            }
            TAG_VARIANT_USER => {
                let user: *mut User = &mut (*(object as *mut User));
                (*user).free_user(state);
            }
            TAG_VARIANT_STRING_SHORT | TAG_VARIANT_STRING_LONG => {
                let tstring: *mut TString = &mut (*(object as *mut TString));
                (*tstring).free_tstring(state);
            },
            _ => {}
        };
    }
}
pub unsafe extern "C" fn find_last(mut objects: *mut *mut Object) -> *mut *mut Object {
    unsafe {
        while !(*objects).is_null() {
            objects = &mut (**objects).next;
        }
        return objects;
    }
}
pub unsafe extern "C" fn check_pointer(objects: *mut *mut Object, object: *mut Object) {
    unsafe {
        if object == *objects {
            *objects = (*object).next;
        }
    }
}
pub unsafe extern "C" fn correct_gray_list(mut objects: *mut *mut Object) -> *mut *mut Object {
    unsafe {
        let mut current_block: u64;
        loop {
            let curr: *mut Object = *objects;
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
                        objects = next;
                        continue;
                    }
                }
            }
            *objects = *next;
        }
        return objects;
    }
}
pub unsafe extern "C" fn delete_list(state: *mut State, mut object: *mut Object, limit: *mut Object) {
    unsafe {
        while object != limit {
            let next: *mut Object = (*object).next;
            free_object(state, object);
            object = next;
        }
    }
}
