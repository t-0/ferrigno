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
use crate::interpreter::*;
use crate::prototype::*;
use crate::table::*;
use crate::tag::*;
use crate::tstring::*;
use crate::tvalue::*;
use crate::upvalue::*;
use crate::user::*;
use std::ptr::*;
pub trait TObject {
    fn as_object(&self) -> &Object;
    fn as_object_mut(&mut self) -> &mut Object;
    fn get_class_name(&mut self) -> String;
    fn get_metatable(&mut self) -> *mut Table {
        null_mut()
    }
    fn getgclist(& mut self) -> *mut *mut Object {
        null_mut()
    }
    fn get_tag(&self) -> u8 {
        self.as_object().get_tag()
    }
    fn set_tag(&mut self, tag: u8) {
        self.as_object_mut().set_tag(tag);
    }
    fn set_tag_variant(&mut self, tag: u8) {
        self.as_object_mut().set_tag_variant(tag);
    }
    fn get_marked(&self) -> u8 {
        self.as_object().get_marked()
    }
    fn set_marked(&mut self, marked: u8) {
        self.as_object_mut().set_marked(marked);
    }
    fn is_collectable(&self) -> bool {
        self.as_object().is_collectable()
    }
    fn set_collectable(&mut self, value: bool) {
        self.as_object_mut().set_collectable(value);
    }
    fn get_tag_type(&self) -> TagType {
        get_tag_type(self.get_tag())
    }
    fn get_tag_variant(&self) -> u8 {
        get_tag_variant(self.get_tag())
    }
    fn is_tagtype_nil(&self) -> bool {
        self.get_tag_type() == TagType::Nil
    }
    fn is_tagtype_boolean(&self) -> bool {
        self.get_tag_type() == TagType::Boolean
    }
    fn is_tagtype_string(&self) -> bool {
        self.get_tag_type() == TagType::String
    }
    fn is_tagtype_numeric(&self) -> bool {
        self.get_tag_type() == TagType::Numeric
    }
    fn is_tagtype_closure(&self) -> bool {
        self.get_tag_type() == TagType::Closure
    }
}
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Object {
    pub next: *mut Object = null_mut(),
    pub tag: u8 = TagVariant::NilNil as u8,
    pub marked: u8 = 0,
}
impl TObject for Object {
    fn as_object(&self) -> &Object {
        self
    }
    fn as_object_mut(&mut self) -> &mut Object {
        self
    }
    fn get_marked(&self) -> u8 {
        self.marked
    }
    fn set_marked(&mut self, marked: u8) {
        self.marked = marked;
    }
    fn get_tag(&self) -> u8 {
        return self.tag;
    }
    fn get_tag_type(&self) -> TagType {
        get_tag_type(self.get_tag())
    }
    fn set_tag(&mut self, tag: u8) {
        self.tag = tag;
    }
    fn set_tag_variant(&mut self, tag: u8) {
        self.tag = tag;
    }
    fn get_class_name(&mut self) -> String {
        "object".to_string()
    }
    fn getgclist(& mut self) -> *mut *mut Object {
        unsafe {
            match TagVariant::from(self.get_tag_variant()) {
                TagVariant::Table => return (*(self as *mut Object as *mut Table)).getgclist(),
                TagVariant::ClosureL | TagVariant::ClosureC => return (*(self as *mut Object as *mut Closure)).getgclist(),
                TagVariant::State => return (*(self as *mut Object as *mut Interpreter)).getgclist(),
                TagVariant::Prototype => return (*(self as *mut Object as *mut Prototype)).getgclist(),
                TagVariant::User => return (*(self as *mut Object as *mut User)).getgclist(),
                _ => return null_mut(),
            };
        }
    }
}
impl Object {
    pub fn new(tag: u8) -> Object {
        Object { next: null_mut(), tag: tag, marked: 0, .. }
    }
}
pub unsafe fn linkgclist_(object: *mut Object, pnext: *mut *mut Object, list: *mut *mut Object) {
    unsafe {
        *pnext = *list;
        *list = object;
        (*object).set_marked((*object).get_marked() & !(1 << 5 | (1 << 3 | 1 << 4)));
    }
}
pub unsafe fn iscleared(global: *mut Global, object: *const Object) -> i32 {
    unsafe {
        if object.is_null() {
            return 0;
        } else if (*object).is_tagtype_string() {
            if (*object).get_marked() & (1 << 3 | 1 << 4) != 0 {
                really_mark_object(global, &mut (*(object as *mut Object)));
            }
            return 0;
        } else {
            return ((*object).get_marked() & (1 << 3 | 1 << 4)) as i32;
        };
    }
}
pub unsafe fn luac_barrier_(interpreter: *mut Interpreter, object: *mut Object, v: *mut Object) {
    unsafe {
        let global: *mut Global = (*interpreter).global;
        if (*global).gc_state as i32 <= 2 {
            really_mark_object(global, v);
            if (*object).get_marked() & 7 > 1 {
                (*v).set_marked((*v).get_marked() & !(7) | 2);
            }
        } else if (*global).gc_kind as i32 == 0 {
            (*object).set_marked((*object).get_marked() & !(1 << 5 | (1 << 3 | 1 << 4)) | ((*global).current_white & (1 << 3 | 1 << 4)));
        }
    }
}
pub unsafe fn luac_barrierback_(interpreter: *mut Interpreter, object: *mut Object) {
    unsafe {
        let global: *mut Global = (*interpreter).global;
        if (*object).get_marked() & 7 == 6 {
            (*object).set_marked((*object).get_marked() & !(1 << 5 | (1 << 3 | 1 << 4)));
        } else {
            linkgclist_(&mut (*(object as *mut Object)), (*object).getgclist(), &mut (*global).gray_again);
        }
        if (*object).get_marked() & 7 > 1 {
            (*object).set_marked((*object).get_marked() & !7 | 5);
        }
    }
}

pub unsafe fn fix_memory_error_message_state(interpreter: *mut Interpreter) {
    unsafe {
        let global: *mut Global = (*interpreter).global;
        (*global).fix_memory_error_message_global();
    }
}
pub unsafe fn fix_object_state(interpreter: *mut Interpreter, object: *mut Object) {
    unsafe {
        let global: *mut Global = (*interpreter).global;
        fix_object_global(global, object);
    }
}
pub unsafe fn fix_object_global(global: *mut Global, object: *mut Object) {
    unsafe {
        (*object).set_marked((*object).get_marked() & !(1 << 5 | (1 << 3 | 1 << 4)));
        (*object).set_marked((*object).get_marked() & !(7) | 4);
        (*global).all_gc = (*object).next;
        (*object).next = (*global).fixed_gc;
        (*global).fixed_gc = object;
    }
}
pub unsafe fn really_mark_object(global: *mut Global, object: *mut Object) {
    unsafe {
        let current_block_18: usize;
        const TAG_VARIANT_STRING_SHORT: u8 = TagVariant::StringShort as u8;
        const TAG_VARIANT_STRING_LONG: u8 = TagVariant::StringLong as u8;
        const TAG_VARIANT_TABLE: u8 = TagVariant::Table as u8;
        const TAG_VARIANT_PROTOTYPE: u8 = TagVariant::Prototype as u8;
        const TAG_VARIANT_UPVALUE: u8 = TagVariant::UpValue as u8;
        const TAG_VARIANT_STATE: u8 = TagVariant::State as u8;
        const TAG_VARIANT_USER: u8 = TagVariant::User as u8;
        const TAG_VARIANT_CLOSURE_C: u8 = TagVariant::ClosureC as u8;
        const TAG_VARIANT_CLOSURE_L: u8 = TagVariant::ClosureL as u8;
        match (*object).get_tag_variant() {
            TAG_VARIANT_STRING_SHORT | TAG_VARIANT_STRING_LONG => {
                (*object).set_marked((*object).get_marked() & !(1 << 3 | 1 << 4) | 1 << 5);
                current_block_18 = 18317007320854588510;
            },
            TAG_VARIANT_UPVALUE => {
                let uv: *mut UpValue = &mut (*(object as *mut UpValue));
                if (*uv).v.p != &mut (*uv).u.value as *mut TValue {
                    (*uv).set_marked((*uv).get_marked() & !(1 << 5 | (1 << 3 | 1 << 4)));
                } else {
                    (*uv).set_marked(((*uv).get_marked() & !(1 << 3 | 1 << 4) | 1 << 5) as u8);
                }
                if ((*(*uv).v.p).is_collectable()) && (*(*(*uv).v.p).value.value_object).get_marked() & (1 << 3 | 1 << 4) != 0 {
                    really_mark_object(global, (*(*uv).v.p).value.value_object);
                }
                current_block_18 = 18317007320854588510;
            },
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
            },
            TAG_VARIANT_CLOSURE_L | TAG_VARIANT_CLOSURE_C | TAG_VARIANT_TABLE | TAG_VARIANT_STATE | TAG_VARIANT_PROTOTYPE => {
                current_block_18 = 15904375183555213903;
            },
            _ => {
                current_block_18 = 18317007320854588510;
            },
        }
        match current_block_18 {
            15904375183555213903 => {
                linkgclist_(&mut (*(object as *mut Object)), (*object).getgclist(), &mut (*global).gray);
            },
            _ => {},
        };
    }
}
pub unsafe fn generate_link(global: *mut Global, object: *mut Object) {
    unsafe {
        if (*object).get_marked() & 7 == 5 {
            linkgclist_(&mut (*(object as *mut Object)), (*object).getgclist(), &mut (*global).gray_again);
        } else if (*object).get_marked() & 7 == 6 {
            (*object).set_marked(((*object).get_marked() ^ (6 ^ 4)) as u8);
        }
    }
}
pub unsafe fn free_object(interpreter: *mut Interpreter, object: *mut Object) {
    unsafe {
        const TAG_VARIANT_STRING_SHORT: u8 = TagVariant::StringShort as u8;
        const TAG_VARIANT_STRING_LONG: u8 = TagVariant::StringLong as u8;
        const TAG_VARIANT_TABLE: u8 = TagVariant::Table as u8;
        const TAG_VARIANT_PROTOTYPE: u8 = TagVariant::Prototype as u8;
        const TAG_VARIANT_UPVALUE: u8 = TagVariant::UpValue as u8;
        const TAG_VARIANT_STATE: u8 = TagVariant::State as u8;
        const TAG_VARIANT_USER: u8 = TagVariant::User as u8;
        const TAG_VARIANT_CLOSURE_C: u8 = TagVariant::ClosureC as u8;
        const TAG_VARIANT_CLOSURE_L: u8 = TagVariant::ClosureL as u8;
        match (*object).get_tag_variant() {
            TAG_VARIANT_PROTOTYPE => {
                let prototype: *mut Prototype = &mut (*(object as *mut Prototype));
                (*prototype).prototype_free(interpreter);
            },
            TAG_VARIANT_UPVALUE => {
                let upvalue: *mut UpValue = &mut (*(object as *mut UpValue));
                (*upvalue).free_upvalue(interpreter);
            },
            TAG_VARIANT_CLOSURE_L | TAG_VARIANT_CLOSURE_C => {
                let closure: *mut Closure = &mut (*(object as *mut Closure));
                (*closure).free_closure(interpreter);
            },
            TAG_VARIANT_TABLE => {
                let table: *mut Table = &mut (*(object as *mut Table));
                (*table).free_table(interpreter);
            },
            TAG_VARIANT_STATE => {
                let other: *mut Interpreter = &mut (*(object as *mut Interpreter));
                (*other).free_interpreter(interpreter);
            },
            TAG_VARIANT_USER => {
                let user: *mut User = &mut (*(object as *mut User));
                (*user).free_user(interpreter);
            },
            TAG_VARIANT_STRING_SHORT | TAG_VARIANT_STRING_LONG => {
                let tstring: *mut TString = &mut (*(object as *mut TString));
                (*tstring).free_tstring(interpreter);
            },
            _ => {},
        };
    }
}
pub unsafe fn find_last(mut objects: *mut *mut Object) -> *mut *mut Object {
    unsafe {
        while !(*objects).is_null() {
            objects = &mut (**objects).next;
        }
        return objects;
    }
}
pub unsafe fn check_pointer(objects: *mut *mut Object, object: *mut Object) {
    unsafe {
        if object == *objects {
            *objects = (*object).next;
        }
    }
}
pub unsafe fn correct_gray_list(mut objects: *mut *mut Object) -> *mut *mut Object {
    unsafe {
        let mut current_block: usize;
        loop {
            let curr: *mut Object = *objects;
            if curr.is_null() {
                break;
            }
            let next: *mut *mut Object = (*curr).getgclist();
            if !((*curr).get_marked() & (1 << 3 | 1 << 4) != 0) {
                if (*curr).get_marked() & 7 == 5 {
                    (*curr).set_marked(((*curr).get_marked() | 1 << 5) as u8);
                    (*curr).set_marked(((*curr).get_marked() ^ (5 ^ 6)) as u8);
                    current_block = 11248371660297272285;
                } else if (*curr).get_tag_variant() == TagVariant::State as u8 {
                    current_block = 11248371660297272285;
                } else {
                    if (*curr).get_marked() & 7 == 6 {
                        (*curr).set_marked(((*curr).get_marked() ^ (6 ^ 4)) as u8);
                    }
                    (*curr).set_marked(((*curr).get_marked() | 1 << 5) as u8);
                    current_block = 6316553219439668466;
                }
                match current_block {
                    6316553219439668466 => {},
                    _ => {
                        objects = next;
                        continue;
                    },
                }
            }
            *objects = *next;
        }
        return objects;
    }
}
pub unsafe fn delete_list(interpreter: *mut Interpreter, mut object: *mut Object, limit: *mut Object) {
    unsafe {
        while object != limit {
            let next: *mut Object = (*object).next;
            free_object(interpreter, object);
            object = next;
        }
    }
}
