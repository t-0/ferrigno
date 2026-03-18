use crate::character::*;
const MAXABITS: i32 = (i32::BITS as usize - 1) as i32;
use crate::f2i::*;
use crate::functions::*;
use crate::global::*;
use crate::node::*;
use crate::object::*;
use crate::objectwithgclist::*;
use crate::objectwithmetatable::*;
use crate::state::*;
use crate::status::*;
use crate::tagtype::*;
use crate::tagvariant::*;
use crate::tdefaultnew::*;
use crate::tm::*;
use crate::tobject::*;
use crate::tobjectwithgclist::TObjectWithGCList;
use crate::tobjectwithmetatable::TObjectWithMetatable;
use crate::tstring::*;
use crate::tvalue::*;
use crate::utility::*;
use crate::value::*;
use std::ptr::*;
type TableSuper = ObjectWithMetatable;
pub const HOK: i32 = 0;
pub const HNOTFOUND: i32 = 1;
pub const HNOTATABLE: i32 = 2;
pub const HFIRSTNODE: i32 = 3;
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct Table {
    pub table_super: TableSuper,
    pub table_flags: u8,
    pub table_log_size_node: u8,
    pub table_a_size: u32,
    pub table_array: *mut Value,
    pub table_node: *mut Node,
    pub table_last_free: *mut Node,
}
impl TObject for Table {
    fn as_object(&self) -> &Object {
        self.table_super.as_object()
    }
    fn as_object_mut(&mut self) -> &mut Object {
        self.table_super.as_object_mut()
    }
}
impl TObjectWithGCList for Table {
    fn getgclist(&mut self) -> *mut *mut ObjectWithGCList {
        self.table_super.getgclist()
    }
}
impl TObjectWithMetatable for Table {
    fn get_metatable(&self) -> *mut Table {
        self.table_super.get_metatable()
    }
    fn set_metatable(&mut self, metatable: *mut Table) {
        self.table_super.set_metatable(metatable);
    }
}
impl TDefaultNew for Table {
    fn new() -> Self {
        Table {
            table_super: TableSuper::new(TagVariant::Table),
            table_flags: 0,
            table_log_size_node: 0,
            table_a_size: 0,
            table_array: null_mut(),
            table_node: null_mut(),
            table_last_free: null_mut(),
        }
    }
}
impl Table {
    pub unsafe fn get_length_raw(&mut self, state: *mut State) -> usize {
        unsafe { luah_getn(state, self) }
    }
    pub unsafe fn table_free(&mut self, state: *mut State) {
        unsafe {
            freehash(state, self);
            if self.table_a_size > 0 {
                let base = self.table_array.sub(self.table_a_size as usize) as *mut std::ffi::c_void;
                (*state).free_memory(base, concretesize(self.table_a_size));
            }
            (*state).free_memory(self as *mut Table as *mut std::ffi::c_void, size_of::<Table>());
        }
    }
    pub unsafe fn exchange_hash_part(t1: *mut Table, t2: *mut Table) {
        unsafe {
            std::mem::swap(&mut (*t1).table_log_size_node, &mut (*t2).table_log_size_node);
            std::mem::swap(&mut (*t1).table_node, &mut (*t2).table_node);
            std::mem::swap(&mut (*t1).table_last_free, &mut (*t2).table_last_free);
        }
    }
    pub unsafe fn get_free_position(&mut self) -> *mut Node {
        unsafe {
            if !self.table_last_free.is_null() {
                while self.table_last_free > self.table_node {
                    self.table_last_free = self.table_last_free.sub(1);
                    self.table_last_free;
                    if (*self.table_last_free).node_key.get_tagvariant() == TagVariant::NilNil {
                        return self.table_last_free;
                    }
                }
            }
            null_mut()
        }
    }
}
pub fn concretesize(size: u32) -> usize {
    if size == 0 {
        0
    } else {
        size as usize * (size_of::<Value>() + 1) + size_of::<u32>()
    }
}
pub unsafe fn get_arr_tag(t: *const Table, k: u32) -> *mut u8 {
    unsafe { ((*t).table_array as *mut u8).add(size_of::<u32>()).add(k as usize) }
}
pub unsafe fn get_arr_val(t: *const Table, k: u32) -> *mut Value {
    unsafe { (*t).table_array.sub(1 + k as usize) }
}
pub unsafe fn lenhint(t: *const Table) -> *mut u32 {
    unsafe { (*t).table_array as *mut u32 }
}
pub fn tagisempty(tag: u8) -> bool {
    tag & 0x0F == TagType::Nil as u8
}
/// Map NilNil → NilEmpty for tags stored in table slots.
/// NilNil (0x00) in a table slot would be misinterpreted as "not a table"
/// by luav_finishget; NilEmpty (0x10) correctly means "empty slot".
#[inline(always)]
pub fn table_store_tag(tag: u8) -> u8 {
    if tag == TagVariant::NilNil as u8 { TagVariant::NilEmpty as u8 } else { tag }
}
pub unsafe fn arraykeyisempty(t: *const Table, key: u32) -> bool {
    unsafe { tagisempty(*get_arr_tag(t, key - 1)) }
}
pub unsafe fn arr2obj(t: *const Table, k: u32, val: *mut TValue) {
    unsafe {
        let tag = *get_arr_tag(t, k);
        (*val).tvalue_set_tag_variant(TagVariant::from(tag));
        (*val).set_raw_value(*get_arr_val(t, k));
        (*val).set_collectable((tag & 0x0F) >= TagType::String as u8);
    }
}
pub unsafe fn obj2arr(t: *const Table, k: u32, val: *const TValue) {
    unsafe {
        *get_arr_tag(t, k) = table_store_tag((*val).get_tagvariant() as u8);
        *get_arr_val(t, k) = (*val).get_raw_value();
    }
}
pub unsafe fn ikeyinarray(t: *const Table, k: i64) -> u32 {
    unsafe {
        if (k as usize).wrapping_sub(1) < (*t).table_a_size as usize {
            k as u32
        } else {
            0
        }
    }
}
pub unsafe fn luat_gettm(events: *mut Table, event: u32, ename: *mut TString) -> *const TValue {
    unsafe {
        let tm: *const TValue = luah_getshortstr(events, ename);
        if (*tm).get_tagvariant().to_tag_type().is_nil() {
            (*events).table_flags = ((*events).table_flags as i32 | (1_u32 << event) as u8 as i32) as u8;
            null()
        } else {
            tm
        }
    }
}
pub unsafe fn traverseweakvalue(global: *mut Global, h: *mut Table) {
    unsafe {
        let limit: *mut Node = &mut *((*h).table_node).add(1usize << (*h).table_log_size_node as i32) as *mut Node;
        let mut hasclears: i32 = ((*h).table_a_size > 0) as i32;
        let mut node: *mut Node = &mut *((*h).table_node).add(0) as *mut Node;
        while node < limit {
            if (*node).node_value.get_tagvariant().to_tag_type().is_nil() {
                (*node).clearkey();
            } else {
                if (*node).node_key.is_collectable() && (*(*node).node_key.as_object().unwrap()).get_marked() & WHITEBITS != 0 {
                    Object::really_mark_object(global, (*node).node_key.as_object().unwrap());
                }
                if hasclears == 0
                    && Object::iscleared(
                        global,
                        if (*node).node_value.is_collectable() {
                            (*node).node_value.as_object().unwrap()
                        } else {
                            null_mut()
                        },
                    ) != 0
                {
                    hasclears = 1;
                }
            }
            node = node.add(1);
        }
        if (*global).global_gcstate as i32 == 0 {
            ObjectWithGCList::linkgclist_(h as *mut ObjectWithGCList, (*h).getgclist(), &mut (*global).global_grayagain);
        } else if hasclears != 0 {
            ObjectWithGCList::linkgclist_(h as *mut ObjectWithGCList, (*h).getgclist(), &mut (*global).global_weak);
        }
    }
}
pub unsafe fn traverseephemeron(global: *mut Global, h: *mut Table, is_reverse: bool) -> i32 {
    unsafe {
        let mut marked: i32 = 0;
        let mut hasclears: i32 = 0;
        let mut hasww: i32 = 0;
        let asize: u32 = (*h).table_a_size;
        let newsize: u32 = (1 << (*h).table_log_size_node as i32) as u32;
        for i in 0..asize {
            let tag = *get_arr_tag(h, i);
            if (tag & 0x0F) >= TagType::String as u8 {
                let obj = (*get_arr_val(h, i)).value_object;
                if (*obj).get_marked() & WHITEBITS != 0 {
                    marked = 1;
                    Object::really_mark_object(global, obj);
                }
            }
        }
        for i in 0..newsize {
            let node: *mut Node = if is_reverse {
                &mut *((*h).table_node).add((newsize - 1 - i) as usize) as *mut Node
            } else {
                &mut *((*h).table_node).add(i as usize) as *mut Node
            };
            if (*node).node_value.get_tagvariant().to_tag_type().is_nil() {
                (*node).clearkey();
            } else if Object::iscleared(
                global,
                if (*node).node_key.is_collectable() {
                    (*node).node_key.as_object().unwrap()
                } else {
                    null_mut()
                },
            ) != 0
            {
                hasclears = 1;
                if ((*node).node_value.is_collectable()) && (*(*node).node_value.as_object().unwrap()).get_marked() & WHITEBITS != 0
                {
                    hasww = 1;
                }
            } else if ((*node).node_value.is_collectable())
                && (*(*node).node_value.as_object().unwrap()).get_marked() & WHITEBITS != 0
            {
                marked = 1;
                Object::really_mark_object(global, (*node).node_value.as_object().unwrap());
            }
        }
        if (*global).global_gcstate as i32 == 0 {
            ObjectWithGCList::linkgclist_(h as *mut ObjectWithGCList, (*h).getgclist(), &mut (*global).global_grayagain);
        } else if hasww != 0 {
            ObjectWithGCList::linkgclist_(h as *mut ObjectWithGCList, (*h).getgclist(), &mut (*global).global_ephemeron);
        } else if hasclears != 0 {
            ObjectWithGCList::linkgclist_(h as *mut ObjectWithGCList, (*h).getgclist(), &mut (*global).global_allweak);
        } else {
            Object::generate_link(global, h as *mut Object);
        }
        marked
    }
}
pub unsafe fn traversestrongtable(global: *mut Global, h: *mut Table) {
    unsafe {
        let limit: *mut Node = &mut *((*h).table_node).add(1usize << (*h).table_log_size_node as i32) as *mut Node;
        let asize: u32 = (*h).table_a_size;
        for i in 0..asize {
            let tag = *get_arr_tag(h, i);
            if (tag & 0x0F) >= TagType::String as u8 {
                let obj = (*get_arr_val(h, i)).value_object;
                if (*obj).get_marked() & WHITEBITS != 0 {
                    Object::really_mark_object(global, obj);
                }
            }
        }
        let mut node: *mut Node = &mut *((*h).table_node).add(0) as *mut Node;
        while node < limit {
            if (*node).node_value.get_tagvariant().to_tag_type().is_nil() {
                (*node).clearkey();
            } else {
                if (*node).node_key.is_collectable() && (*(*node).node_key.as_object().unwrap()).get_marked() & WHITEBITS != 0 {
                    Object::really_mark_object(global, (*node).node_key.as_object().unwrap());
                }
                if (*node).node_value.is_collectable() && (*(*node).node_value.as_object().unwrap()).get_marked() & WHITEBITS != 0 {
                    Object::really_mark_object(global, (*node).node_value.as_object().unwrap());
                }
            }
            node = node.add(1);
        }
        Object::generate_link(global, h as *mut Object);
    }
}
pub unsafe fn traversetable(global: *mut Global, h: *mut Table) -> usize {
    unsafe {
        let mode: *const TValue = if ((*h).get_metatable()).is_null() {
            null()
        } else if (*(*h).get_metatable()).table_flags as u32 & 1_u32 << TM_MODE as i32 != 0 {
            null()
        } else {
            luat_gettm((*h).get_metatable(), TM_MODE, (*global).global_tmname[TM_MODE as usize])
        };
        let smode: *mut TString;
        if !((*h).get_metatable()).is_null() && (*(*h).get_metatable()).get_marked() & WHITEBITS != 0 {
            Object::really_mark_object(global, (*h).get_metatable() as *mut Object);
        }
        let weakkey: *const i8;
        let weakvalue: *const i8;
        if !mode.is_null() && (*mode).get_tagvariant() == TagVariant::StringShort && {
            smode = (*mode).as_string().unwrap();
            weakkey = cstr_chr((*smode).get_contents_mut(), Character::LowerK as i8);
            weakvalue = cstr_chr((*smode).get_contents_mut(), Character::LowerV as i8);
            !weakkey.is_null() || !weakvalue.is_null()
        } {
            if weakkey.is_null() {
                traverseweakvalue(global, h);
            } else if weakvalue.is_null() {
                traverseephemeron(global, h, false);
            } else {
                ObjectWithGCList::linkgclist_(h as *mut ObjectWithGCList, (*h).getgclist(), &mut (*global).global_allweak);
            }
        } else {
            traversestrongtable(global, h);
        }
        1_u32.wrapping_add((*h).table_a_size).wrapping_add(
            (2 * (if ((*h).table_last_free).is_null() {
                0
            } else {
                1 << (*h).table_log_size_node as i32
            })) as u32,
        ) as usize
    }
}
pub unsafe fn tablerehash(vect: *mut *mut TString, oldsize: usize, newsize: usize) {
    unsafe {
        for i in oldsize..newsize {
            let fresh = &mut *vect.add(i);
            *fresh = null_mut();
        }
        for i in 0..oldsize {
            let mut p: *mut TString = *vect.add(i);
            *vect.add(i) = null_mut();
            while !p.is_null() {
                let hash_next: *mut TString = (*p).tstring_hash_next;
                let h: u32 = (*p).tstring_hash & (newsize - 1) as u32;
                (*p).tstring_hash_next = *vect.add(h as usize);
                *vect.add(h as usize) = p;
                p = hash_next;
            }
        }
    }
}
pub unsafe fn hashint(t: *const Table, i: i64) -> *mut Node {
    unsafe {
        let ui: usize = i as usize;
        if ui <= MAX_INT {
            &mut *((*t).table_node).add((ui as i32 % (((1 << (*t).table_log_size_node as i32) - 1) | 1)) as usize) as *mut Node
        } else {
            &mut *((*t).table_node).add(ui.wrapping_rem((((1 << (*t).table_log_size_node as i32) - 1) | 1) as usize)) as *mut Node
        }
    }
}
pub unsafe fn mainpositiontv(t: *const Table, key: *const TValue) -> *mut Node {
    unsafe {
        match (*key).get_tagvariant() {
            | TagVariant::NumericInteger => {
                let i: i64 = (*key).as_integer().unwrap();
                hashint(t, i)
            },
            | TagVariant::NumericNumber => {
                let n: f64 = (*key).as_number().unwrap();
                &mut *((*t).table_node)
                    .add(((l_hashfloat as unsafe fn(f64) -> i32)(n) % (((1 << (*t).table_log_size_node as i32) - 1) | 1)) as usize)
                    as *mut Node
            },
            | TagVariant::StringShort => {
                let tstring: *mut TString = (*key).as_string().unwrap();
                &mut *((*t).table_node)
                    .add(((*tstring).tstring_hash & ((1 << (*t).table_log_size_node as i32) - 1) as u32) as usize)
                    as *mut Node
            },
            | TagVariant::StringLong => {
                let long_str: *mut TString = (*key).as_string().unwrap();
                &mut *((*t).table_node)
                    .offset(((*long_str).hash_string_long() & ((1 << (*t).table_log_size_node as i32) - 1) as u32) as i32 as isize)
                    as *mut Node
            },
            | TagVariant::BooleanFalse => {
                &mut *((*t).table_node).add((0 & ((1 << (*t).table_log_size_node as i32) - 1)) as usize) as *mut Node
            },
            | TagVariant::BooleanTrue => {
                &mut *((*t).table_node).add((1 & ((1 << (*t).table_log_size_node as i32) - 1)) as usize) as *mut Node
            },
            | TagVariant::Pointer => {
                let p: *mut std::ffi::c_void = (*key).as_pointer().unwrap();
                &mut *((*t).table_node).offset(
                    ((p as usize & u32::MAX as usize) as u32)
                        .wrapping_rem((((1 << (*t).table_log_size_node as i32) - 1) | 1) as u32) as isize,
                ) as *mut Node
            },
            | TagVariant::ClosureCFunction => {
                let cfunction: CFunction = (*key).as_function().unwrap();
                &mut *((*t).table_node).offset(
                    ((::core::mem::transmute::<CFunction, usize>(cfunction) & u32::MAX as usize) as u32)
                        .wrapping_rem((((1 << (*t).table_log_size_node as i32) - 1) | 1) as u32) as isize,
                ) as *mut Node
            },
            | _ => {
                let o: *mut Object = (*key).as_object().unwrap();
                &mut *((*t).table_node).offset(
                    ((o as usize & u32::MAX as usize) as u32)
                        .wrapping_rem((((1 << (*t).table_log_size_node as i32) - 1) | 1) as u32) as isize,
                ) as *mut Node
            },
        }
    }
}
pub unsafe fn mainpositionfromnode(t: *const Table, nd: *mut Node) -> *mut Node {
    unsafe {
        let mut key: TValue = TValue::new(TagVariant::NilNil);
        let io_: *mut TValue = &mut key;
        let node: *const Node = nd;
        (*io_).copy_from(&((*node).node_key));
        mainpositiontv(t, &key)
    }
}
pub unsafe fn getintfromhash(table: *mut Table, key: i64) -> *const TValue {
    unsafe {
        let mut node: *mut Node = hashint(table, key);
        loop {
            if (*node).node_key.get_tagvariant() == TagVariant::NumericInteger && (*node).node_key.as_integer().unwrap() == key {
                return &mut (*node).node_value;
            } else {
                let nx: i32 = (*node).node_next;
                if nx == 0 {
                    return &ABSENT_KEY;
                }
                node = Node::next_node(node);
            }
        }
    }
}
pub unsafe fn hashkeyisempty(table: *mut Table, key: usize) -> bool {
    unsafe {
        let val = getintfromhash(table, key as i64);
        (*val).get_tagvariant().to_tag_type().is_nil()
    }
}
pub unsafe fn finishnodeget(val: *const TValue, res: *mut TValue) -> TagVariant {
    unsafe {
        if !(*val).get_tagvariant().to_tag_type().is_nil() {
            (*res).copy_from(&*val);
        }
        (*val).get_tagvariant()
    }
}
pub unsafe fn finishnodeset(table: *mut Table, slot: *const TValue, val: *mut TValue) -> i32 {
    unsafe {
        if !(*slot).get_tagvariant().to_tag_type().is_nil() {
            let io1: *mut TValue = slot as *mut TValue;
            (*io1).copy_from(&*val);
            // NilNil in a table slot means "not a table" to luav_finishget;
            // use NilEmpty for "empty slot" instead.
            if (*io1).get_tagvariant() == TagVariant::NilNil {
                (*io1).tvalue_set_tag_variant(TagVariant::NilEmpty);
            }
            HOK
        } else {
            retpsetcode(table, slot)
        }
    }
}
pub unsafe fn rawfinishnodeset(slot: *const TValue, val: *mut TValue) -> bool {
    unsafe {
        if (*slot).get_tagvariant() == TagVariant::NilAbsentKey {
            false
        } else {
            let io1: *mut TValue = slot as *mut TValue;
            (*io1).copy_from(&*val);
            if (*io1).get_tagvariant() == TagVariant::NilNil {
                (*io1).tvalue_set_tag_variant(TagVariant::NilEmpty);
            }
            true
        }
    }
}
pub unsafe fn retpsetcode(table: *mut Table, slot: *const TValue) -> i32 {
    unsafe {
        if (*slot).get_tagvariant() == TagVariant::NilAbsentKey {
            HNOTFOUND
        } else {
            (slot as *mut Node).offset_from((*table).table_node) as i32 + HFIRSTNODE
        }
    }
}
pub unsafe fn getgeneric(table: *mut Table, key: *const TValue, deadok: i32) -> *const TValue {
    unsafe {
        let mut node: *mut Node = mainpositiontv(table, key);
        loop {
            if equal_key(key, node, deadok) {
                return &mut (*node).node_value;
            } else {
                let nx: i32 = (*node).node_next;
                if nx == 0 {
                    return &ABSENT_KEY;
                }
                node = Node::next_node(node);
            }
        }
    }
}
pub unsafe fn findindex(state: *mut State, table: *mut Table, key: *mut TValue, asize: u32) -> u32 {
    unsafe {
        let mut i: u32;
        if (*key).get_tagvariant().to_tag_type().is_nil() {
            return 0;
        }
        i = if (*key).get_tagvariant() == TagVariant::NumericInteger {
            ikeyinarray(table, (*key).as_integer().unwrap())
        } else {
            0
        };
        if i != 0 {
            i
        } else {
            let n_value: *const TValue = getgeneric(table, key, 1);
            if (*n_value).get_tagvariant() == TagVariant::NilAbsentKey {
                luag_runerror(state, c"invalid key to 'next'".as_ptr(), &[]);
            }
            i = (n_value as *mut Node).offset_from(&mut *((*table).table_node).add(0) as *mut Node) as u32;
            i.wrapping_add(1_u32).wrapping_add(asize)
        }
    }
}
pub unsafe fn luah_next(state: *mut State, table: *mut Table, key: *mut TValue) -> i32 {
    unsafe {
        let asize: u32 = (*table).table_a_size;
        let mut i: u32 = findindex(state, table, key, asize);
        while i < asize {
            let tag = *get_arr_tag(table, i);
            if !tagisempty(tag) {
                let io: *mut TValue = &mut (*key);
                (*io).set_integer((i + 1) as i64);
                arr2obj(table, i, &mut (*key.add(1)));
                return 1;
            }
            i += 1;
        }
        i -= asize;
        while (i as i32) < 1 << (*table).table_log_size_node as i32 {
            if !(*((*table).table_node).add(i as usize))
                .node_value
                .get_tagvariant()
                .to_tag_type()
                .is_nil()
            {
                let node: *mut Node = &mut *((*table).table_node).add(i as usize) as *mut Node;
                let io_: *mut TValue = &mut (*key);
                (*io_).copy_from(&((*node).node_key));
                let dest_val: *mut TValue = &mut (*key.add(1));
                let src_val: *const TValue = &mut (*node).node_value;
                (*dest_val).copy_from(&*src_val);
                return 1;
            }
            i += 1;
        }
        0
    }
}
pub unsafe fn freehash(state: *mut State, table: *mut Table) {
    unsafe {
        if !((*table).table_last_free).is_null() {
            (*state).free_memory(
                (*table).table_node as *mut std::ffi::c_void,
                ((1 << (*table).table_log_size_node as i32) as usize).wrapping_mul(size_of::<Node>()),
            );
        }
    }
}
pub unsafe fn computesizes(nums: *mut u32, pna: *mut u32) -> u32 {
    unsafe {
        let mut i: i32;
        let mut twotoi: u32;
        let mut a: u32 = 0;
        let mut count_array: u32 = 0;
        let mut optimal: u32 = 0;
        i = 0;
        twotoi = 1_u32;
        while twotoi > 0 && *pna > twotoi / 2 {
            a = a.wrapping_add(*nums.add(i as usize));
            if a > twotoi / 2 {
                optimal = twotoi;
                count_array = a;
            }
            i += 1;
            twotoi = twotoi.wrapping_mul(2_u32);
        }
        *pna = count_array;
        optimal
    }
}
pub unsafe fn countint(key: i64, nums: *mut u32) -> i32 {
    unsafe {
        let k: u32 = arrayindex(key);
        if k == 0 {
            0
        } else {
            let fresh = &mut *nums.add(ceiling_log2(k as usize));
            *fresh += 1;
            1
        }
    }
}
pub unsafe fn numusearray(t: *const Table, nums: *mut u32) -> u32 {
    unsafe {
        let mut lg: i32;
        let mut ttlg: u32;
        let mut ause: u32 = 0;
        let mut i: u32 = 1_u32;
        let asize: u32 = (*t).table_a_size;
        lg = 0;
        ttlg = 1_u32;
        while lg <= MAXABITS {
            let mut lc: u32 = 0;
            let mut lim: u32 = ttlg;
            if lim > asize {
                lim = asize;
                if i > lim {
                    break;
                }
            }
            while i <= lim {
                if !arraykeyisempty(t, i) {
                    lc += 1;
                }
                i += 1;
            }
            let count = &mut *nums.add(lg as usize);
            *count = (*count).wrapping_add(lc);
            ause = ause.wrapping_add(lc);
            lg += 1;
            ttlg = ttlg.wrapping_mul(2_u32);
        }
        ause
    }
}
pub unsafe fn numusehash(t: *const Table, nums: *mut u32, pna: *mut u32) -> i32 {
    unsafe {
        let mut totaluse: i32 = 0;
        let mut ause: i32 = 0;
        let mut i: i32 = 1 << (*t).table_log_size_node as i32;
        loop {
            let prev_i = i;
            i -= 1;
            if prev_i == 0 {
                break;
            }
            let node: *mut Node = &mut *((*t).table_node).add(i as usize) as *mut Node;
            if !(*node).node_value.get_tagvariant().to_tag_type().is_nil() {
                if (*node).node_key.get_tagvariant() == TagVariant::NumericInteger {
                    ause += countint((*node).node_key.as_integer().unwrap(), nums);
                }
                totaluse += 1;
            }
        }
        *pna = (*pna).wrapping_add(ause as u32);
        totaluse
    }
}
pub unsafe fn setnodevector(state: *mut State, table: *mut Table, mut size: u32) {
    unsafe {
        if size == 0 {
            (*table).table_node = &DUMMY_NODE as *const Node as *mut Node;
            (*table).table_log_size_node = 0;
            (*table).table_last_free = null_mut();
        } else {
            let lsize: i32 = ceiling_log2(size as usize) as i32;
            if lsize > MAXABITS - 1
                || 1_u32 << lsize
                    > (if (1_u32 << (MAXABITS - 1)) as usize <= ((!0usize) / size_of::<Node>()) {
                        1_u32 << (MAXABITS - 1)
                    } else {
                        ((!0usize) / size_of::<Node>()) as u32
                    })
            {
                luag_runerror(state, c"table overflow".as_ptr(), &[]);
            }
            size = (1 << lsize) as u32;
            (*table).table_node = (*state).allocate((size as usize).wrapping_mul(size_of::<Node>())) as *mut Node;
            for i in 0..size {
                let node: *mut Node = &mut *((*table).table_node).add(i as usize) as *mut Node;
                (*node).node_next = 0;
                (*node).node_key.tvalue_set_tag_variant(TagVariant::NilNil);
                (*node).node_value.tvalue_set_tag_variant(TagVariant::NilEmpty);
            }
            (*table).table_log_size_node = lsize as u8;
            (*table).table_last_free = &mut *((*table).table_node).add(size as usize) as *mut Node;
        };
    }
}
pub unsafe fn reinsert(_state: *mut State, ot: *mut Table, table: *mut Table) {
    unsafe {
        let mut j: i32;
        let size: i32 = 1 << (*ot).table_log_size_node as i32;
        j = 0;
        while j < size {
            let old: *mut Node = &mut *((*ot).table_node).add(j as usize) as *mut Node;
            if !(*old).node_value.get_tagvariant().to_tag_type().is_nil() {
                let mut k: TValue = TValue::new(TagVariant::NilNil);
                let io_: *mut TValue = &mut k;
                let node: *const Node = old;
                (*io_).copy_from(&(*node).node_key);
                newcheckedkey(table, &k, &mut (*old).node_value);
            }
            j += 1;
        }
    }
}
pub unsafe fn newcheckedkey(table: *mut Table, key: *const TValue, value: *mut TValue) {
    unsafe {
        let i = if (*key).get_tagvariant() == TagVariant::NumericInteger {
            ikeyinarray(table, (*key).as_integer().unwrap())
        } else {
            0
        };
        if i > 0 {
            obj2arr(table, i - 1, value);
        } else {
            let mut mp = mainpositiontv(table, key);
            if !(*mp).node_value.get_tagvariant().to_tag_type().is_nil() || (*table).table_last_free.is_null() {
                let mut other_node: *mut Node;
                let free_node: *mut Node = (*table).get_free_position();
                if free_node.is_null() {
                    return;
                }
                other_node = mainpositionfromnode(table, mp);
                if other_node != mp {
                    while Node::next_node(other_node) != mp {
                        other_node = Node::next_node(other_node);
                    }
                    (*other_node).node_next = free_node.offset_from(other_node) as i32;
                    *free_node = *mp;
                    if (*mp).node_next != 0 {
                        (*free_node).node_next += mp.offset_from(free_node) as i32;
                        (*mp).node_next = 0;
                    }
                    (*mp).node_value.tvalue_set_tag_variant(TagVariant::NilEmpty);
                } else {
                    if (*mp).node_next != 0 {
                        (*free_node).node_next = Node::next_node(mp).offset_from(free_node) as i32;
                    }
                    (*mp).node_next = free_node.offset_from(mp) as i32;
                    mp = free_node;
                }
            }
            let node: *mut Node = mp;
            (*node).node_key.copy_from(&*key);
            let io1: *mut TValue = &mut (*mp).node_value;
            (*io1).copy_from(&*value);
        }
    }
}
pub unsafe fn resizearray(state: *mut State, table: *mut Table, oldasize: u32, newasize: u32) -> *mut Value {
    unsafe {
        if oldasize == newasize {
            (*table).table_array
        } else if newasize == 0 {
            if oldasize > 0 {
                let base = (*table).table_array.sub(oldasize as usize) as *mut std::ffi::c_void;
                (*state).free_memory(base, concretesize(oldasize));
            }
            null_mut()
        } else {
            let newasizeb = concretesize(newasize);
            let np_raw = (*state).reallocate(null_mut(), 0, newasizeb) as *mut u8;
            if np_raw.is_null() {
                return null_mut();
            }
            let np = (np_raw as *mut Value).add(newasize as usize);
            if oldasize > 0 {
                let oldasizeb = concretesize(oldasize);
                let op = (*table).table_array;
                let tomove = if oldasize < newasize { oldasize } else { newasize };
                let tomoveb = if oldasize < newasize { oldasizeb } else { newasizeb };
                std::ptr::copy_nonoverlapping(
                    op.sub(tomove as usize) as *const u8,
                    np.sub(tomove as usize) as *mut u8,
                    tomoveb,
                );
                let old_base = op.sub(oldasize as usize) as *mut std::ffi::c_void;
                (*state).free_memory(old_base, oldasizeb);
            }
            np
        }
    }
}
pub unsafe fn reinsertoldslice(table: *mut Table, oldasize: u32, newasize: u32) {
    unsafe {
        for i in newasize..oldasize {
            let tag = *get_arr_tag(table, i);
            if !tagisempty(tag) {
                let mut key: TValue = TValue::new(TagVariant::NilNil);
                key.set_integer((i as i64) + 1);
                let mut aux: TValue = TValue::new(TagVariant::NilNil);
                arr2obj(table, i, &mut aux);
                newcheckedkey(table, &key, &mut aux);
            }
        }
    }
}
pub unsafe fn clearnewslice(table: *mut Table, mut oldasize: u32, newasize: u32) {
    unsafe {
        while oldasize < newasize {
            *get_arr_tag(table, oldasize) = TagVariant::NilEmpty as u8;
            oldasize += 1;
        }
    }
}
pub unsafe fn luah_resize(state: *mut State, table: *mut Table, new_array_size: usize, new_table_size: usize) {
    unsafe {
        let mut new_table: Table = Table::new();
        let old_array_size = (*table).table_a_size;
        let newasize = new_array_size as u32;
        setnodevector(state, &mut new_table, new_table_size as u32);
        if newasize < old_array_size {
            // Temporarily set asize to new (smaller) value so that
            // ikeyinarray inside newcheckedkey routes high-index keys
            // to the hash part instead of back into the (shrinking) array.
            (*table).table_a_size = newasize;
            Table::exchange_hash_part(table, &mut new_table);
            reinsertoldslice(table, old_array_size, newasize);
            Table::exchange_hash_part(table, &mut new_table);
            (*table).table_a_size = old_array_size;
        }
        let new_array = resizearray(state, table, old_array_size, newasize);
        if new_array.is_null() && newasize > 0 {
            freehash(state, &mut new_table);
            luad_throw(state, Status::MemoryError);
        }
        Table::exchange_hash_part(table, &mut new_table);
        (*table).table_array = new_array;
        (*table).table_a_size = newasize;
        if !new_array.is_null() {
            *lenhint(table) = newasize / 2;
        }
        clearnewslice(table, old_array_size, newasize);
        reinsert(state, &mut new_table, table);
        freehash(state, &mut new_table);
    }
}
pub unsafe fn luah_resizearray(state: *mut State, table: *mut Table, new_array_size: usize) {
    unsafe {
        let new_table_size = if (*table).table_last_free.is_null() && (*table).table_log_size_node == 0 {
            0
        } else {
            1 << (*table).table_log_size_node
        };
        luah_resize(state, table, new_array_size, new_table_size);
    }
}
pub unsafe fn rehash(state: *mut State, table: *mut Table, ek: *const TValue) {
    unsafe {
        let mut nums: [u32; MAXABITS as usize + 1] = [0; MAXABITS as usize + 1];
        let mut i: i32 = 0;
        while i <= MAXABITS {
            nums[i as usize] = 0;
            i += 1;
        }
        let mut count_array: u32 = numusearray(table, nums.as_mut_ptr());
        let mut totaluse = count_array as i32;
        totaluse += numusehash(table, nums.as_mut_ptr(), &mut count_array);
        if (*ek).get_tagvariant() == TagVariant::NumericInteger {
            count_array = count_array.wrapping_add(countint((*ek).as_integer().unwrap(), nums.as_mut_ptr()) as u32);
        }
        totaluse += 1;
        let asize: u32 = computesizes(nums.as_mut_ptr(), &mut count_array);
        luah_resize(
            state,
            table,
            asize as usize,
            (totaluse as usize).wrapping_sub(count_array as usize),
        );
    }
}
pub unsafe fn luah_new(state: *mut State) -> *mut Table {
    unsafe {
        let object: *mut Object = luac_newobj(state, TagVariant::Table, size_of::<Table>());
        let new_table: *mut Table = &mut *(object as *mut Table);
        (*new_table).set_metatable(null_mut());
        (*new_table).table_flags = !(!0 << (TM_EQ as i32 + 1)) as u8;
        (*new_table).table_array = null_mut();
        (*new_table).table_a_size = 0;
        setnodevector(state, new_table, 0);
        new_table
    }
}
pub unsafe fn luah_newkey(state: *mut State, table: *mut Table, key: *const TValue, value: *mut TValue) {
    unsafe {
        let mut mp;
        if (*value).get_tagvariant().to_tag_type().is_nil() {
            return;
        }
        mp = mainpositiontv(table, key);
        if !(*mp).node_value.get_tagvariant().to_tag_type().is_nil() || (*table).table_last_free.is_null() {
            let mut other_node: *mut Node;
            let free_node: *mut Node = (*table).get_free_position();
            if free_node.is_null() {
                rehash(state, table, key);
                luah_set(state, table, key, value);
                return;
            }
            other_node = mainpositionfromnode(table, mp);
            if other_node != mp {
                while Node::next_node(other_node) != mp {
                    other_node = Node::next_node(other_node);
                }
                (*other_node).node_next = free_node.offset_from(other_node) as i32;
                *free_node = *mp;
                if (*mp).node_next != 0 {
                    (*free_node).node_next += mp.offset_from(free_node) as i32;
                    (*mp).node_next = 0;
                }
                (*mp).node_value.tvalue_set_tag_variant(TagVariant::NilEmpty);
            } else {
                if (*mp).node_next != 0 {
                    (*free_node).node_next = Node::next_node(mp).offset_from(free_node) as i32;
                }
                (*mp).node_next = free_node.offset_from(mp) as i32;
                mp = free_node;
            }
        }
        let node: *mut Node = mp;
        let io_: *const TValue = key;
        (*node).node_key.copy_from(&*io_);
        if (*key).is_collectable()
            && (*(table as *mut Object)).get_marked() & BLACKBIT != 0
            && (*(*key).as_object().unwrap()).get_marked() & WHITEBITS != 0
        {
            ObjectWithGCList::luac_barrierback_(state, &mut *(table as *mut ObjectWithGCList));
        };
        let io1: *mut TValue = &mut (*mp).node_value;
        let io2: *const TValue = value;
        (*io1).copy_from(&*io2);
    }
}
pub unsafe fn luah_getint(table: *mut Table, key: i64, res: *mut TValue) -> TagVariant {
    unsafe {
        let k = ikeyinarray(table, key);
        if k > 0 {
            let tag = *get_arr_tag(table, k - 1);
            if !tagisempty(tag) {
                (*res).tvalue_set_tag_variant(TagVariant::from(tag));
                (*res).set_raw_value(*get_arr_val(table, k - 1));
                (*res).set_collectable((tag & 0x0F) >= TagType::String as u8);
            }
            TagVariant::from(tag)
        } else {
            finishnodeget(getintfromhash(table, key), res)
        }
    }
}
pub unsafe fn luah_getshortstr(table: *mut Table, key: *mut TString) -> *const TValue {
    unsafe {
        let mut node: *mut Node = &mut *((*table).table_node)
            .add(((*key).tstring_hash & ((1 << (*table).table_log_size_node as i32) - 1) as u32) as usize)
            as *mut Node;
        loop {
            if (*node).node_key.get_tagvariant() == TagVariant::StringShort && (*node).node_key.as_string().unwrap() == key {
                return &mut (*node).node_value;
            } else {
                let nx: i32 = (*node).node_next;
                if nx == 0 {
                    return &ABSENT_KEY;
                }
                node = Node::next_node(node);
            }
        }
    }
}
pub unsafe fn luah_getstr(table: *mut Table, key: *mut TString) -> *const TValue {
    unsafe {
        if (*key).get_tagvariant() == TagVariant::StringShort {
            luah_getshortstr(table, key)
        } else {
            let mut ko: TValue = TValue::new(TagVariant::NilNil);
            let io: *mut TValue = &mut ko;
            let tstring: *mut TString = key;
            (*io).set_object(tstring as *mut Object, (*tstring).get_tagvariant());
            getgeneric(table, &ko, 0)
        }
    }
}
pub unsafe fn luah_get(table: *mut Table, key: *const TValue, res: *mut TValue) -> TagVariant {
    unsafe {
        match (*key).get_tagvariant() {
            | TagVariant::StringShort => {
                return finishnodeget(luah_getshortstr(table, (*key).as_string().unwrap()), res);
            },
            | TagVariant::NumericInteger => return luah_getint(table, (*key).as_integer().unwrap(), res),
            | TagVariant::NilNil => return TagVariant::NilAbsentKey,
            | TagVariant::NumericNumber => {
                let mut k: i64 = 0;
                if F2I::Equal.convert_f64_i64((*key).as_number().unwrap(), &mut k) {
                    return luah_getint(table, k, res);
                }
            },
            | _ => {},
        }
        finishnodeget(getgeneric(table, key, 0), res)
    }
}
pub unsafe fn luah_pset(table: *mut Table, key: *const TValue, val: *mut TValue) -> i32 {
    unsafe {
        match (*key).get_tagvariant() {
            | TagVariant::StringShort => {
                return finishnodeset(table, luah_getshortstr(table, (*key).as_string().unwrap()), val);
            },
            | TagVariant::NumericInteger => {
                let k = ikeyinarray(table, (*key).as_integer().unwrap());
                if k > 0 {
                    let tag = get_arr_tag(table, k - 1);
                    if !tagisempty(*tag) {
                        *tag = table_store_tag((*val).get_tagvariant() as u8);
                        *get_arr_val(table, k - 1) = (*val).get_raw_value();
                        return HOK;
                    } else {
                        return !(k as i32 - 1);
                    }
                } else {
                    return finishnodeset(table, getintfromhash(table, (*key).as_integer().unwrap()), val);
                }
            },
            | TagVariant::NilNil => return HNOTFOUND,
            | TagVariant::NumericNumber => {
                let mut k: i64 = 0;
                if F2I::Equal.convert_f64_i64((*key).as_number().unwrap(), &mut k) {
                    let ik = ikeyinarray(table, k);
                    if ik > 0 {
                        let tag = get_arr_tag(table, ik - 1);
                        if !tagisempty(*tag) {
                            *tag = table_store_tag((*val).get_tagvariant() as u8);
                            *get_arr_val(table, ik - 1) = (*val).get_raw_value();
                            return HOK;
                        } else {
                            return !(ik as i32 - 1);
                        }
                    } else {
                        return finishnodeset(table, getintfromhash(table, k), val);
                    }
                }
            },
            | _ => {},
        }
        finishnodeset(table, getgeneric(table, key, 0), val)
    }
}
pub unsafe fn luah_finishset(state: *mut State, table: *mut Table, key: *const TValue, value: *mut TValue, hres: i32) {
    unsafe {
        if hres == HNOTFOUND {
            let mut aux: TValue = TValue::new(TagVariant::NilNil);
            let mut key = key;
            if (*key).get_tagvariant().to_tag_type().is_nil() {
                luag_runerror(state, c"table index is nil".as_ptr(), &[]);
            } else if (*key).get_tagvariant() == TagVariant::NumericNumber {
                let number = (*key).as_number().unwrap();
                let mut k: i64 = 0;
                if F2I::Equal.convert_f64_i64(number, &mut k) {
                    aux.set_integer(k);
                    key = &aux;
                } else if number != number {
                    luag_runerror(state, c"table index is NaN".as_ptr(), &[]);
                }
            } else if (*key).get_tagvariant() == TagVariant::StringLong {
                let ts = (*key).as_string().unwrap();
                if (*ts).is_external() {
                    let norm = luas_normstr(state, ts);
                    let io: *mut TValue = &mut (*(*state).interpreter_top.stkidrel_pointer);
                    (*io).set_object(norm as *mut Object, (*norm).get_tagvariant());
                    (*state).luad_inctop();
                    luah_newkey(state, table, &(*(*state).interpreter_top.stkidrel_pointer.sub(1)), value);
                    (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.sub(1);
                    return;
                }
            }
            luah_newkey(state, table, key, value);
        } else if hres > 0 {
            let io1: *mut TValue = &mut (*((*table).table_node).add((hres - HFIRSTNODE) as usize)).node_value;
            (*io1).copy_from(&*value);
        } else {
            let idx = !hres as u32;
            obj2arr(table, idx, value);
        };
    }
}
pub unsafe fn luah_set(state: *mut State, table: *mut Table, key: *const TValue, value: *mut TValue) {
    unsafe {
        let hres = luah_pset(table, key, value);
        if hres != HOK {
            luah_finishset(state, table, key, value, hres);
        }
    }
}
pub unsafe fn luah_psetint(table: *mut Table, key: i64, val: *mut TValue) -> i32 {
    unsafe { finishnodeset(table, getintfromhash(table, key), val) }
}
pub unsafe fn luah_setint(state: *mut State, table: *mut Table, key: i64, value: *mut TValue) {
    unsafe {
        let ik = ikeyinarray(table, key);
        if ik > 0 {
            obj2arr(table, ik - 1, value);
        } else {
            let ok = rawfinishnodeset(getintfromhash(table, key), value);
            if !ok {
                let mut k: TValue = TValue::new(TagVariant::NilNil);
                k.set_integer(key);
                luah_newkey(state, table, &k, value);
            }
        }
    }
}
pub unsafe fn hash_search(state: *mut State, table: *mut Table, asize: u32) -> usize {
    unsafe {
        let mut i: usize = asize as usize + 1;
        let mut rnd: u32 = (*(*state).interpreter_global).global_seed;
        let n: i32 = if asize > 0 { ceiling_log2(asize as usize) as i32 } else { 0 };
        let mask: u32 = (1u32 << n).wrapping_sub(1);
        let incr: u32 = (rnd & mask).wrapping_add(1);
        let mut j: usize = if (incr as usize) <= MAXIMUM_SIZE - i { i + incr as usize } else { i + 1 };
        rnd >>= n;
        while !hashkeyisempty(table, j) {
            i = j;
            if j < MAXIMUM_SIZE / 2 {
                j = j * 2 + (rnd & 1) as usize;
                rnd >>= 1;
            } else {
                j = MAXIMUM_SIZE;
                if hashkeyisempty(table, j) {
                    break;
                }
                return j;
            }
        }
        while j - i > 1 {
            let m = (i + j) / 2;
            if hashkeyisempty(table, m) {
                j = m;
            } else {
                i = m;
            }
        }
        i
    }
}
unsafe fn newhint(table: *mut Table, hint: u32) -> usize {
    unsafe {
        *lenhint(table) = hint;
        hint as usize
    }
}
unsafe fn table_binsearch(table: *const Table, mut i: u32, mut j: u32) -> u32 {
    unsafe {
        while j.wrapping_sub(i) > 1 {
            let m = i.wrapping_add(j) / 2;
            if arraykeyisempty(table, m) {
                j = m;
            } else {
                i = m;
            }
        }
        i
    }
}
pub unsafe fn luah_getn(state: *mut State, table: *mut Table) -> usize {
    unsafe {
        let asize: u32 = (*table).table_a_size;
        if asize > 0 {
            let maxvicinity: u32 = 4;
            let mut limit: u32 = *lenhint(table);
            if limit == 0 {
                limit = 1;
            }
            if arraykeyisempty(table, limit) {
                let mut i: u32 = 0;
                while i < maxvicinity && limit > 1 {
                    limit -= 1;
                    if !arraykeyisempty(table, limit) {
                        return newhint(table, limit);
                    }
                    i += 1;
                }
                return newhint(table, table_binsearch(table, 0, limit));
            } else {
                let mut i: u32 = 0;
                while i < maxvicinity && limit < asize {
                    limit += 1;
                    if arraykeyisempty(table, limit) {
                        return newhint(table, limit - 1);
                    }
                    i += 1;
                }
                if arraykeyisempty(table, asize) {
                    return newhint(table, table_binsearch(table, limit, asize));
                }
            }
            *lenhint(table) = asize;
        }
        if ((*table).table_last_free.is_null() && (*table).table_log_size_node == 0) || hashkeyisempty(table, asize as usize + 1) {
            asize as usize
        } else {
            hash_search(state, table, asize)
        }
    }
}
pub unsafe fn luav_finishget(state: *mut State, mut t: *const TValue, key: *mut TValue, value: *mut TValue, mut _tag: TagVariant) {
    unsafe {
        let mut loop_count: i32 = 0;
        let mut tm: *const TValue;
        while loop_count < MAXTAGLOOP {
            if (*t).get_tagvariant() != TagVariant::Table {
                // t is not a table — try __index metamethod on the non-table value
                tm = luat_gettmbyobj(state, t, TM_INDEX);
                if (*tm).get_tagvariant().to_tag_type().is_nil() {
                    luag_typeerror(state, t, c"index".as_ptr());
                }
            } else {
                let t_table: *mut Table = (*t).as_table().unwrap();
                tm = if ((*t_table).get_metatable()).is_null() {
                    null()
                } else if (*(*t_table).get_metatable()).table_flags as u32 & 1_u32 << TM_INDEX as i32 != 0 {
                    null()
                } else {
                    luat_gettm(
                        (*t_table).get_metatable(),
                        TM_INDEX,
                        (*(*state).interpreter_global).global_tmname[TM_INDEX as usize],
                    )
                };
                if tm.is_null() {
                    (*value).tvalue_set_tag_variant(TagVariant::NilNil);
                    return;
                }
            }
            if (*tm).get_tagvariant().to_tag_type().is_closure() {
                luat_calltmres(state, tm, t, key, value);
                return;
            }
            t = tm;
            if (*t).get_tagvariant() == TagVariant::Table {
                _tag = luah_get((*t).as_table().unwrap(), key, value);
                if !_tag.to_tag_type().is_nil() {
                    return;
                }
            } else {
                _tag = TagVariant::NilNil;
            }
            loop_count += 1;
        }
        luag_runerror(state, c"'__index' chain too long; possible loop".as_ptr(), &[]);
    }
}
pub unsafe fn luav_finishset(state: *mut State, mut t: *const TValue, key: *mut TValue, value: *mut TValue, mut hres: i32) {
    unsafe {
        let mut loop_count: i32 = 0;
        while loop_count < MAXTAGLOOP {
            let tm: *const TValue;
            if hres != HNOTATABLE {
                let h: *mut Table = (*t).as_table().unwrap();
                tm = if ((*h).get_metatable()).is_null() {
                    null()
                } else if (*(*h).get_metatable()).table_flags as u32 & 1_u32 << TM_NEWINDEX as i32 != 0 {
                    null()
                } else {
                    luat_gettm(
                        (*h).get_metatable(),
                        TM_NEWINDEX,
                        (*(*state).interpreter_global).global_tmname[TM_NEWINDEX as usize],
                    )
                };
                if tm.is_null() {
                    let io: *mut TValue = &mut (*(*state).interpreter_top.stkidrel_pointer);
                    (*io).set_table(h);
                    (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.add(1);
                    luah_finishset(state, h, key, value, hres);
                    (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.sub(1);
                    (*h).table_flags = ((*h).table_flags as u32 & !!(!0 << (TM_EQ as i32 + 1))) as u8;
                    if (*value).is_collectable()
                        && (*(h as *mut Object)).get_marked() & BLACKBIT != 0
                        && (*(*value).as_object().unwrap()).get_marked() & WHITEBITS != 0
                    {
                        ObjectWithGCList::luac_barrierback_(state, &mut *(h as *mut ObjectWithGCList));
                    }
                    return;
                }
            } else {
                tm = luat_gettmbyobj(state, t, TM_NEWINDEX);
                if (*tm).get_tagvariant().to_tag_type().is_nil() {
                    luag_typeerror(state, t, c"index".as_ptr());
                }
            }
            if (*tm).get_tagvariant().to_tag_type().is_closure() {
                luat_calltm(state, tm, t, key, value);
                return;
            }
            t = tm;
            if (*t).get_tagvariant() == TagVariant::Table {
                hres = luah_pset((*t).as_table().unwrap(), key, value);
                if hres == HOK {
                    return;
                }
            } else {
                hres = HNOTATABLE;
            }
            loop_count += 1;
        }
        luag_runerror(state, c"'__newindex' chain too long; possible loop".as_ptr(), &[]);
    }
}
