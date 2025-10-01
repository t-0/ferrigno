use crate::character::*;
use crate::status::*;
use crate::f2i::*;
use crate::functions::*;
use crate::global::*;
use crate::interpreter::*;
use crate::new::*;
use crate::node::*;
use crate::object::*;
use crate::tag::*;
use crate::tm::*;
use crate::tstring::*;
use crate::tvalue::*;
use crate::utility::*;
use libc::*;
use std::ptr::*;
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct Table {
    pub gclist: ObjectWithGCList,
    pub flags: u8,
    pub log_size_node: u8,
    pub array_limit: u32,
    pub array: *mut TValue,
    pub node: *mut Node,
    pub last_free: *mut Node,
    pub metatable: *mut Table,
}
impl TObject for Table {
    fn as_object(&self) -> &Object {
        self.gclist.as_object()
    }
    fn as_object_mut(&mut self) -> &mut Object {
        self.gclist.as_object_mut()
    }
    fn get_class_name(&mut self) -> String {
        "table".to_string()
    }
    fn get_metatable(&mut self) -> *mut Table {
        self.metatable
    }
    fn getgclist(& mut self) -> *mut *mut Object {
        self.gclist.getgclist()
    }
}
impl New for Table {
    fn new() -> Self {
        Table {
            gclist: ObjectWithGCList::new(TagVariant::Table),
            flags: 0,
            log_size_node: 0,
            array_limit: 0,
            array: null_mut(),
            node: null_mut(),
            last_free: null_mut(),
            metatable: null_mut(),
            ..
        }
    }
}
impl Table {
    pub unsafe fn free_table(&mut self, interpreter: *mut Interpreter) {
        unsafe {
            freehash(interpreter, self);
            (*interpreter).free_memory(self.array as *mut libc::c_void, (luah_realasize(self) as usize).wrapping_mul(size_of::<TValue>() as usize) as usize);
            (*interpreter).free_memory(self as *mut Table as *mut libc::c_void, size_of::<Table>());
        }
    }
    pub unsafe fn exchange_hash_part(t1: *mut Table, t2: *mut Table) {
        unsafe {
            let temporary_size_node: u8 = (*t1).log_size_node;
            (*t1).log_size_node = (*t2).log_size_node;
            (*t2).log_size_node = temporary_size_node;
            let temporary_node: *mut Node = (*t1).node;
            (*t1).node = (*t2).node;
            (*t2).node = temporary_node;
            let temporary_last_free: *mut Node = (*t1).last_free;
            (*t1).last_free = (*t2).last_free;
            (*t2).last_free = temporary_last_free;
        }
    }
    pub unsafe fn get_free_position(&mut self) -> *mut Node {
        unsafe {
            if !self.last_free.is_null() {
                while self.last_free > self.node {
                    self.last_free = self.last_free.offset(-1);
                    self.last_free;
                    if (*self.last_free).key.get_tag_variant() == TagVariant::NilNil {
                        return self.last_free;
                    }
                }
            }
            return null_mut();
        }
    }
}
pub unsafe fn luat_gettm(events: *mut Table, event: u32, ename: *mut TString) -> *const TValue {
    unsafe {
        let tm: *const TValue = luah_getshortstr(events, ename);
        if (*tm).is_tagtype_nil() {
            (*events).flags = ((*events).flags as i32 | ((1 as u32) << event as u32) as u8 as i32) as u8;
            return null();
        } else {
            return tm;
        };
    }
}
pub unsafe fn traverseweakvalue(global: *mut Global, h: *mut Table) {
    unsafe {
        let limit: *mut Node = &mut *((*h).node).offset((1 << (*h).log_size_node as i32) as isize) as *mut Node;
        let mut hasclears: i32 = ((*h).array_limit > 0u32) as i32;
        let mut node: *mut Node = &mut *((*h).node).offset(0 as isize) as *mut Node;
        while node < limit {
            if (*node).value.is_tagtype_nil() {
                (*node).clearkey();
            } else {
                if (*node).key.is_collectable() && (*(*node).key.value.value_object).get_marked() & (1 << 3 | 1 << 4) != 0 {
                    really_mark_object(global, (*node).key.value.value_object);
                }
                if hasclears == 0 && iscleared(global, if (*node).value.is_collectable() { (*node).value.value.value_object } else { null_mut() }) != 0 {
                    hasclears = 1;
                }
            }
            node = node.offset(1);
        }
        if (*global).global_gcstate as i32 == 2 && hasclears != 0 {
            linkgclist_(&mut (*(h as *mut Object)), (*h).getgclist(), &mut (*global).global_weak);
        } else {
            linkgclist_(&mut (*(h as *mut Object)), (*h).getgclist(), &mut (*global).global_grayagain);
        };
    }
}
pub unsafe fn traverseephemeron(global: *mut Global, h: *mut Table, is_reverse: bool) -> i32 {
    unsafe {
        let mut marked: i32 = 0;
        let mut hasclears: i32 = 0;
        let mut hasww: i32 = 0;
        let asize: u32 = luah_realasize(h);
        let new_size: u32 = (1 << (*h).log_size_node as i32) as u32;
        for i in 0..asize {
            if ((*((*h).array).offset(i as isize)).is_collectable()) && (*(*((*h).array).offset(i as isize)).value.value_object).get_marked() & (1 << 3 | 1 << 4) != 0 {
                marked = 1;
                really_mark_object(global, (*((*h).array).offset(i as isize)).value.value_object);
            }
        }
        for i in 0..new_size {
            let node: *mut Node = if is_reverse {
                &mut *((*h).node).offset(new_size.wrapping_sub(1 as u32).wrapping_sub(i) as isize) as *mut Node
            } else {
                &mut *((*h).node).offset(i as isize) as *mut Node
            };
            if (*node).value.is_tagtype_nil() {
                (*node).clearkey();
            } else if iscleared(global, if (*node).key.is_collectable() { (*node).key.value.value_object } else { null_mut() }) != 0 {
                hasclears = 1;
                if ((*node).value.is_collectable()) && (*(*node).value.value.value_object).get_marked() & (1 << 3 | 1 << 4) != 0 {
                    hasww = 1;
                }
            } else if ((*node).value.is_collectable()) && (*(*node).value.value.value_object).get_marked() & (1 << 3 | 1 << 4) != 0 {
                marked = 1;
                really_mark_object(global, (*node).value.value.value_object);
            }
        }
        if (*global).global_gcstate as i32 == 0 {
            linkgclist_(&mut (*(h as *mut Object)), (*h).getgclist(), &mut (*global).global_grayagain);
        } else if hasww != 0 {
            linkgclist_(&mut (*(h as *mut Object)), (*h).getgclist(), &mut (*global).global_ephemeron);
        } else if hasclears != 0 {
            linkgclist_(&mut (*(h as *mut Object)), (*h).getgclist(), &mut (*global).global_allweak);
        } else {
            generate_link(global, &mut (*(h as *mut Object)));
        }
        return marked;
    }
}
pub unsafe fn traversestrongtable(global: *mut Global, h: *mut Table) {
    unsafe {
        let limit: *mut Node = &mut *((*h).node).offset((1 << (*h).log_size_node as i32) as isize) as *mut Node;
        let asize: u32 = luah_realasize(h);
        for i in 0..asize {
            if ((*((*h).array).offset(i as isize)).is_collectable()) && (*(*((*h).array).offset(i as isize)).value.value_object).get_marked() & (1 << 3 | 1 << 4) != 0 {
                really_mark_object(global, (*((*h).array).offset(i as isize)).value.value_object);
            }
        }
        let mut node: *mut Node = &mut *((*h).node).offset(0 as isize) as *mut Node;
        while node < limit {
            if (*node).value.is_tagtype_nil() {
                (*node).clearkey();
            } else {
                if (*node).key.is_collectable() && (*(*node).key.value.value_object).get_marked() & (1 << 3 | 1 << 4) != 0 {
                    really_mark_object(global, (*node).key.value.value_object);
                }
                if (*node).value.is_collectable() && (*(*node).value.value.value_object).get_marked() & (1 << 3 | 1 << 4) != 0 {
                    really_mark_object(global, (*node).value.value.value_object);
                }
            }
            node = node.offset(1);
        }
        generate_link(global, &mut (*(h as *mut Object)));
    }
}
pub unsafe fn traversetable(global: *mut Global, h: *mut Table) -> usize {
    unsafe {
        let mut weakkey: *const i8 = null();
        let mut weakvalue: *const i8 = null();
        let mode: *const TValue = if ((*h).get_metatable()).is_null() {
            null()
        } else if (*(*h).get_metatable()).flags as u32 & (1 as u32) << TM_MODE as i32 != 0 {
            null()
        } else {
            luat_gettm((*h).get_metatable(), TM_MODE, (*global).global_tmname[TM_MODE as usize])
        };
        let smode: *mut TString;
        if !((*h).get_metatable()).is_null() {
            if (*(*h).get_metatable()).get_marked() & (1 << 3 | 1 << 4) != 0 {
                really_mark_object(global, &mut (*((*h).get_metatable() as *mut Object)));
            }
        }
        if !mode.is_null() && (*mode).get_tag_variant() == TagVariant::StringShort && {
            smode = &mut (*((*mode).value.value_object as *mut TString)) as *mut TString;
            weakkey = strchr((*smode).get_contents_mut(), Character::LowerK as i32);
            weakvalue = strchr((*smode).get_contents_mut(), Character::LowerV as i32);
            !weakkey.is_null() || !weakvalue.is_null()
        } {
            if weakkey.is_null() {
                traverseweakvalue(global, h);
            } else if weakvalue.is_null() {
                traverseephemeron(global, h, false);
            } else {
                linkgclist_(&mut (*(h as *mut Object)), (*h).getgclist(), &mut (*global).global_allweak);
            }
        } else {
            traversestrongtable(global, h);
        }
        return (1 as u32)
            .wrapping_add((*h).array_limit)
            .wrapping_add((2 * (if ((*h).last_free).is_null() { 0 } else { 1 << (*h).log_size_node as i32 })) as u32) as usize;
    }
}
pub unsafe fn tablerehash(vect: *mut *mut TString, old_size: usize, new_size: usize) {
    unsafe {
        for i in old_size..new_size {
            let ref mut fresh = *vect.offset(i as isize);
            *fresh = null_mut();
        }
        for i in 0..old_size {
            let mut p: *mut TString = *vect.offset(i as isize);
            let ref mut fresh21 = *vect.offset(i as isize);
            *fresh21 = null_mut();
            while !p.is_null() {
                let hash_next: *mut TString = (*p).hash_next;
                let h: u32 = ((*p).hash & (new_size - 1) as u32) as u32;
                (*p).hash_next = *vect.offset(h as isize);
                let ref mut fresh22 = *vect.offset(h as isize);
                *fresh22 = p;
                p = hash_next;
            }
        }
    }
}
pub unsafe fn hashint(t: *const Table, i: i64) -> *mut Node {
    unsafe {
        let ui: usize = i as usize;
        if ui <= 0x7FFFFFFF as usize {
            return &mut *((*t).node).offset((ui as i32 % ((1 << (*t).log_size_node as i32) - 1 | 1)) as isize) as *mut Node;
        } else {
            return &mut *((*t).node).offset(ui.wrapping_rem(((1 << (*t).log_size_node as i32) - 1 | 1) as usize) as isize) as *mut Node;
        };
    }
}
pub unsafe fn mainpositiontv(t: *const Table, key: *const TValue) -> *mut Node {
    unsafe {
        match (*key).get_tag_variant() {
            TagVariant::NumericInteger => {
                let i: i64 = (*key).value.value_integer;
                return hashint(t, i);
            },
            TagVariant::NumericNumber => {
                let n: f64 = (*key).value.value_number;
                return &mut *((*t).node).offset(((l_hashfloat as unsafe fn(f64) -> i32)(n) % ((1 << (*t).log_size_node as i32) - 1 | 1)) as isize) as *mut Node;
            },
            TagVariant::StringShort => {
                let tstring: *mut TString = &mut (*((*key).value.value_object as *mut TString));
                return &mut *((*t).node).offset(((*tstring).hash & ((1 << (*t).log_size_node as i32) - 1) as u32) as isize) as *mut Node;
            },
            TagVariant::StringLong => {
                let ts_0: *mut TString = &mut (*((*key).value.value_object as *mut TString));
                return &mut *((*t).node).offset(((hash_string_long as unsafe fn(*mut TString) -> u32)(ts_0) & ((1 << (*t).log_size_node as i32) - 1) as u32) as i32 as isize) as *mut Node;
            },
            TagVariant::BooleanFalse => {
                return &mut *((*t).node).offset((0 & (1 << (*t).log_size_node as i32) - 1) as isize) as *mut Node;
            },
            TagVariant::BooleanTrue => {
                return &mut *((*t).node).offset((1 & (1 << (*t).log_size_node as i32) - 1) as isize) as *mut Node;
            },
            TagVariant::Pointer => {
                let p: *mut libc::c_void = (*key).value.value_pointer;
                return &mut *((*t).node).offset(((p as usize & (0x7FFFFFFF as u32).wrapping_mul(2 as u32).wrapping_add(1 as u32) as usize) as u32).wrapping_rem(((1 << (*t).log_size_node as i32) - 1 | 1) as u32) as isize) as *mut Node;
            },
            TagVariant::ClosureCFunction => {
                let cfunction: CFunction = (*key).value.value_function;
                return &mut *((*t).node)
                    .offset(((::core::mem::transmute::<CFunction, usize>(cfunction) & (0x7FFFFFFF as u32).wrapping_mul(2 as u32).wrapping_add(1 as u32) as usize) as u32).wrapping_rem(((1 << (*t).log_size_node as i32) - 1 | 1) as u32) as isize)
                    as *mut Node;
            },
            _ => {
                let o: *mut Object = (*key).value.value_object;
                return &mut *((*t).node).offset(((o as usize & (0x7FFFFFFF as u32).wrapping_mul(2 as u32).wrapping_add(1 as u32) as usize) as u32).wrapping_rem(((1 << (*t).log_size_node as i32) - 1 | 1) as u32) as isize) as *mut Node;
            },
        };
    }
}
pub unsafe fn mainpositionfromnode(t: *const Table, nd: *mut Node) -> *mut Node {
    unsafe {
        let mut key: TValue = TValue::new(TagVariant::NilNil);
        let io_: *mut TValue = &mut key;
        let node: *const Node = nd;
        (*io_).copy_from(&((*node).key));
        return mainpositiontv(t, &mut key);
    }
}
pub unsafe fn luah_realasize(t: *const Table) -> u32 {
    unsafe {
        if (*t).flags as i32 & 1 << 7 == 0 || (*t).array_limit & ((*t).array_limit).wrapping_sub(1 as u32) == 0 {
            return (*t).array_limit;
        } else {
            let mut size: u32 = (*t).array_limit;
            size |= size >> 1;
            size |= size >> 2;
            size |= size >> 4;
            size |= size >> 8;
            size |= size >> 16 as i32;
            size = size.wrapping_add(1);
            return size;
        };
    }
}
pub unsafe fn ispow2realasize(t: *const Table) -> i32 {
    unsafe {
        return ((*t).flags as i32 & 1 << 7 != 0 || (*t).array_limit & ((*t).array_limit).wrapping_sub(1 as u32) == 0) as i32;
    }
}
pub unsafe fn setlimittosize(table: *mut Table) -> u32 {
    unsafe {
        (*table).array_limit = luah_realasize(table);
        (*table).flags = ((*table).flags as i32 & !(1 << 7) as u8 as i32) as u8;
        return (*table).array_limit;
    }
}
pub unsafe fn getgeneric(table: *mut Table, key: *const TValue, deadok: i32) -> *const TValue {
    unsafe {
        let mut node: *mut Node = mainpositiontv(table, key);
        loop {
            if equal_key(key, node, deadok) {
                return &mut (*node).value;
            } else {
                let nx: i32 = (*node).next;
                if nx == 0 {
                    return &ABSENT_KEY;
                }
                node = node.offset(nx as isize);
            }
        }
    }
}
pub unsafe fn findindex(interpreter: *mut Interpreter, table: *mut Table, key: *mut TValue, asize: u32) -> u32 {
    unsafe {
        let mut i: u32;
        if (*key).is_tagtype_nil() {
            return 0u32;
        }
        i = if (*key).get_tag_variant() == TagVariant::NumericInteger { arrayindex((*key).value.value_integer) } else { 0u32 };
        if i.wrapping_sub(1 as u32) < asize {
            return i;
        } else {
            let n_value: *const TValue = getgeneric(table, key, 1);
            if (*n_value).get_tag_variant() == TagVariant::NilAbsentKey {
                luag_runerror(interpreter, c"invalid key to 'next'".as_ptr());
            }
            i = (n_value as *mut Node).offset_from(&mut *((*table).node).offset(0 as isize) as *mut Node) as u32;
            return i.wrapping_add(1 as u32).wrapping_add(asize);
        };
    }
}
pub unsafe fn luah_next(interpreter: *mut Interpreter, table: *mut Table, key: *mut TValue) -> i32 {
    unsafe {
        let asize: u32 = luah_realasize(table);
        let mut i: u32 = findindex(interpreter, table, &mut (*key), asize);
        while i < asize {
            if !(*((*table).array).offset(i as isize)).is_tagtype_nil() {
                let io: *mut TValue = &mut (*key);
                (*io).value.value_integer = i.wrapping_add(1 as u32) as i64;
                (*io).set_tag_variant(TagVariant::NumericInteger);
                let io1: *mut TValue = &mut (*key.offset(1 as isize));
                let io2: *const TValue = &mut *((*table).array).offset(i as isize) as *mut TValue;
                (*io1).copy_from(&*io2);
                return 1;
            }
            i = i.wrapping_add(1);
        }
        i = i.wrapping_sub(asize);
        while (i as i32) < 1 << (*table).log_size_node as i32 {
            if !(*((*table).node).offset(i as isize)).value.is_tagtype_nil() {
                let node: *mut Node = &mut *((*table).node).offset(i as isize) as *mut Node;
                let io_: *mut TValue = &mut (*key);
                (*io_).copy_from(&((*node).key));
                let io1_0: *mut TValue = &mut (*key.offset(1 as isize));
                let io2_0: *const TValue = &mut (*node).value;
                (*io1_0).copy_from(&*io2_0);
                return 1;
            }
            i = i.wrapping_add(1);
        }
        return 0;
    }
}
pub unsafe fn freehash(interpreter: *mut Interpreter, table: *mut Table) {
    unsafe {
        if !((*table).last_free).is_null() {
            (*interpreter).free_memory(
                (*table).node as *mut libc::c_void,
                ((1 << (*table).log_size_node as i32) as usize).wrapping_mul(size_of::<Node>() as usize) as usize,
            );
        }
    }
}
pub unsafe fn computesizes(nums: *mut u32, pna: *mut u32) -> u32 {
    unsafe {
        let mut i: i32;
        let mut twotoi: u32;
        let mut a: u32 = 0u32;
        let mut count_array: u32 = 0u32;
        let mut optimal: u32 = 0u32;
        i = 0;
        twotoi = 1 as u32;
        while twotoi > 0u32 && *pna > twotoi / 2 {
            a = a.wrapping_add(*nums.offset(i as isize));
            if a > twotoi / 2 {
                optimal = twotoi;
                count_array = a;
            }
            i += 1;
            twotoi = twotoi.wrapping_mul(2 as u32);
        }
        *pna = count_array;
        return optimal;
    }
}
pub unsafe fn countint(key: i64, nums: *mut u32) -> i32 {
    unsafe {
        let k: u32 = arrayindex(key);
        if k == 0 {
            return 0;
        } else {
            let ref mut fresh = *nums.offset(ceiling_log2(k as usize) as isize);
            *fresh += 1;
            return 1;
        };
    }
}
pub unsafe fn numusearray(t: *const Table, nums: *mut u32) -> u32 {
    unsafe {
        let mut lg: i32;
        let mut ttlg: u32;
        let mut ause: u32 = 0u32;
        let mut i: u32 = 1 as u32;
        let asize: u32 = (*t).array_limit;
        lg = 0;
        ttlg = 1 as u32;
        while lg <= (size_of::<i32>() as usize).wrapping_mul(8 as usize).wrapping_sub(1 as usize) as i32 {
            let mut lc: u32 = 0u32;
            let mut lim: u32 = ttlg;
            if lim > asize {
                lim = asize;
                if i > lim {
                    break;
                }
            }
            while i <= lim {
                if (*((*t).array).offset(i.wrapping_sub(1 as u32) as isize)).get_tag_type() != TagType::Nil {
                    lc = lc.wrapping_add(1);
                }
                i = i.wrapping_add(1);
            }
            let ref mut fresh130 = *nums.offset(lg as isize);
            *fresh130 = (*fresh130).wrapping_add(lc);
            ause = ause.wrapping_add(lc);
            lg += 1;
            ttlg = ttlg.wrapping_mul(2 as u32);
        }
        return ause;
    }
}
pub unsafe fn numusehash(t: *const Table, nums: *mut u32, pna: *mut u32) -> i32 {
    unsafe {
        let mut totaluse: i32 = 0;
        let mut ause: i32 = 0;
        let mut i: i32 = 1 << (*t).log_size_node as i32;
        loop {
            let fresh131 = i;
            i = i - 1;
            if !(fresh131 != 0) {
                break;
            }
            let node: *mut Node = &mut *((*t).node).offset(i as isize) as *mut Node;
            match (*node).value.get_tag_variant() {
                TagVariant::NilNil | TagVariant::NilAbsentKey | TagVariant::NilEmpty => {},
                TagVariant::NumericInteger => {
                    ause += countint((*node).key.value.value_integer, nums);
                    totaluse += 1;
                },
                _ => totaluse += 1,
            }
        }
        *pna = (*pna).wrapping_add(ause as u32);
        return totaluse;
    }
}
pub unsafe fn setnodevector(interpreter: *mut Interpreter, table: *mut Table, mut size: u32) {
    unsafe {
        if size == 0 {
            (*table).node = &DUMMY_NODE as *const Node as *mut Node;
            (*table).log_size_node = 0;
            (*table).last_free = null_mut();
        } else {
            let lsize: i32 = ceiling_log2(size as usize) as i32;
            if lsize > (size_of::<i32>() as usize).wrapping_mul(8 as usize).wrapping_sub(1 as usize) as i32 - 1
                || (1 as u32) << lsize
                    > (if ((1 as u32) << (size_of::<i32>() as usize).wrapping_mul(8 as usize).wrapping_sub(1 as usize) as i32 - 1) as usize <= ((!0usize) / size_of::<Node>()) {
                        (1 as u32) << (size_of::<i32>() as usize).wrapping_mul(8 as usize).wrapping_sub(1 as usize) as i32 - 1
                    } else {
                        ((!0usize) / size_of::<Node>()) as u32
                    })
            {
                luag_runerror(interpreter, c"table overflow".as_ptr());
            }
            size = (1 << lsize) as u32;
            (*table).node = luam_malloc_(interpreter, (size as usize).wrapping_mul(size_of::<Node>())) as *mut Node;
            for i in 0..size {
                let node: *mut Node = &mut *((*table).node).offset(i as isize) as *mut Node;
                (*node).next = 0;
                (*node).key.set_tag_variant(TagVariant::NilNil);
                (*node).value.set_tag_variant(TagVariant::NilEmpty);
            }
            (*table).log_size_node = lsize as u8;
            (*table).last_free = &mut *((*table).node).offset(size as isize) as *mut Node;
        };
    }
}
pub unsafe fn reinsert(interpreter: *mut Interpreter, ot: *mut Table, table: *mut Table) {
    unsafe {
        let mut j: i32;
        let size: i32 = 1 << (*ot).log_size_node as i32;
        j = 0;
        while j < size {
            let old: *mut Node = &mut *((*ot).node).offset(j as isize) as *mut Node;
            if !(*old).value.is_tagtype_nil() {
                let mut k: TValue = TValue::new(TagVariant::NilNil);
                let io_: *mut TValue = &mut k;
                let node: *const Node = old;
                (*io_).copy_from(&(*node).key);
                luah_set(interpreter, table, &mut k, &mut (*old).value);
            }
            j += 1;
        }
    }
}
pub unsafe fn luah_resize(interpreter: *mut Interpreter, table: *mut Table, new_array_size: usize, new_table_size: usize) {
    unsafe {
        let mut new_table: Table = Table::new();
        let old_array_size = setlimittosize(table) as usize;
        let new_array: *mut TValue;
        setnodevector(interpreter, &mut new_table, new_table_size as u32);
        if new_array_size < old_array_size {
            (*table).array_limit = new_array_size as u32;
            Table::exchange_hash_part(table, &mut new_table);
            for i in new_array_size..old_array_size {
                if !(*((*table).array).offset(i as isize)).is_tagtype_nil() {
                    luah_setint(interpreter, table, i.wrapping_add(1) as i64, &mut *((*table).array).offset(i as isize));
                }
            }
            (*table).array_limit = old_array_size as u32;
            Table::exchange_hash_part(table, &mut new_table);
        }
        new_array = luam_realloc_(
            interpreter,
            (*table).array as *mut libc::c_void,
            (old_array_size as usize).wrapping_mul(size_of::<TValue>()),
            (new_array_size as usize).wrapping_mul(size_of::<TValue>()),
        ) as *mut TValue;
        if new_array.is_null() && new_array_size > 0 {
            freehash(interpreter, &mut new_table);
            luad_throw(interpreter, Status::MemoryError);
        }
        Table::exchange_hash_part(table, &mut new_table);
        (*table).array = new_array;
        (*table).array_limit = new_array_size as u32;
        for i in old_array_size..new_array_size {
            (*((*table).array).offset(i as isize)).set_tag_variant(TagVariant::NilEmpty);
        }
        reinsert(interpreter, &mut new_table, table);
        freehash(interpreter, &mut new_table);
    }
}
pub unsafe fn luah_resizearray(interpreter: *mut Interpreter, table: *mut Table, new_array_size: usize) {
    unsafe {
        let new_table_size = if ((*table).last_free).is_null() { 0 } else { 1 << (*table).log_size_node };
        luah_resize(interpreter, table, new_array_size, new_table_size);
    }
}
pub unsafe fn rehash(interpreter: *mut Interpreter, table: *mut Table, ek: *const TValue) {
    unsafe {
        let mut nums: [u32; 32] = [0; 32];
        let mut i: i32 = 0;
        while i <= (size_of::<i32>() as usize).wrapping_mul(8 as usize).wrapping_sub(1 as usize) as i32 {
            nums[i as usize] = 0u32;
            i += 1;
        }
        setlimittosize(table);
        let mut count_array: u32 = numusearray(table, nums.as_mut_ptr());
        let mut totaluse = count_array as i32;
        totaluse += numusehash(table, nums.as_mut_ptr(), &mut count_array);
        if (*ek).get_tag_variant() == TagVariant::NumericInteger {
            count_array = count_array.wrapping_add(countint((*ek).value.value_integer, nums.as_mut_ptr()) as u32);
        }
        totaluse += 1;
        let asize: u32 = computesizes(nums.as_mut_ptr(), &mut count_array);
        luah_resize(interpreter, table, asize as usize, (totaluse as usize).wrapping_sub(count_array as usize));
    }
}
pub unsafe fn luah_new(interpreter: *mut Interpreter) -> *mut Table {
    unsafe {
        let object: *mut Object = luac_newobj(interpreter, TagVariant::Table, size_of::<Table>());
        let new_table: *mut Table = &mut (*(object as *mut Table));
        (*new_table).metatable = null_mut();
        (*new_table).flags = !(!0 << TM_EQ as i32 + 1) as u8;
        (*new_table).array = null_mut();
        (*new_table).array_limit = 0u32;
        setnodevector(interpreter, new_table, 0u32);
        return new_table;
    }
}
pub unsafe fn luah_newkey(interpreter: *mut Interpreter, table: *mut Table, mut key: *const TValue, value: *mut TValue) {
    unsafe {
        let mut mp;
        let mut aux: TValue = TValue::new(TagVariant::NilNil);
        if (*key).is_tagtype_nil() {
            luag_runerror(interpreter, c"table index is nil".as_ptr());
        } else if (*key).get_tag_variant() == TagVariant::NumericNumber {
            let number = (*key).value.value_number;
            let mut k: i64 = 0;
            if luav_flttointeger(number, &mut k, F2I::Equal) {
                aux.value.value_integer = k;
                aux.set_tag_variant(TagVariant::NumericInteger);
                key = &mut aux;
            } else if number != number {
                luag_runerror(interpreter, c"table index is NaN".as_ptr());
            }
        }
        if (*value).is_tagtype_nil() {
            return;
        }
        mp = mainpositiontv(table, key);
        if !(*mp).value.is_tagtype_nil() || (*table).last_free.is_null() {
            let mut other_node: *mut Node;
            let f_0: *mut Node = (*table).get_free_position();
            if f_0.is_null() {
                rehash(interpreter, table, key);
                luah_set(interpreter, table, key, value);
                return;
            }
            other_node = mainpositionfromnode(table, mp);
            if other_node != mp {
                while other_node.offset((*other_node).next as isize) != mp {
                    other_node = other_node.offset((*other_node).next as isize);
                }
                (*other_node).next = f_0.offset_from(other_node) as i32;
                *f_0 = *mp;
                if (*mp).next != 0 {
                    (*f_0).next += mp.offset_from(f_0) as i32;
                    (*mp).next = 0;
                }
                (*mp).value.set_tag_variant(TagVariant::NilEmpty);
            } else {
                if (*mp).next != 0 {
                    (*f_0).next = mp.offset((*mp).next as isize).offset_from(f_0) as i32;
                }
                (*mp).next = f_0.offset_from(mp) as i32;
                mp = f_0;
            }
        }
        let node: *mut Node = mp;
        let io_: *const TValue = key;
        (*node).key.copy_from(&*io_);
        if (*key).is_collectable() {
            if (*(table as *mut Object)).get_marked() & 1 << 5 != 0 && (*(*key).value.value_object).get_marked() & (1 << 3 | 1 << 4) != 0 {
                luac_barrierback_(interpreter, &mut (*(table as *mut Object)));
            } else {
            };
        } else {
        };
        let io1: *mut TValue = &mut (*mp).value;
        let io2: *const TValue = value;
        (*io1).copy_from(&*io2);
    }
}
pub unsafe fn luah_getint(table: *mut Table, key: i64) -> *const TValue {
    unsafe {
        let array_limit: usize = (*table).array_limit as usize;
        if (key as usize).wrapping_sub(1 as usize) < array_limit {
            return &mut *((*table).array).offset((key - 1) as isize) as *mut TValue;
        } else if (*table).flags as i32 & 1 << 7 != 0 && (key as usize).wrapping_sub(1 as usize) & !array_limit.wrapping_sub(1 as usize) < array_limit {
            (*table).array_limit = key as u32;
            return &mut *((*table).array).offset((key - 1) as isize) as *mut TValue;
        } else {
            let mut node: *mut Node = hashint(table, key);
            loop {
                if (*node).key.get_tag_variant() == TagVariant::NumericInteger && (*node).key.value.value_integer == key {
                    return &mut (*node).value;
                } else {
                    let nx: i32 = (*node).next;
                    if nx == 0 {
                        break;
                    }
                    node = node.offset(nx as isize);
                }
            }
            return &ABSENT_KEY;
        };
    }
}
pub unsafe fn luah_getshortstr(table: *mut Table, key: *mut TString) -> *const TValue {
    unsafe {
        let mut node: *mut Node = &mut *((*table).node).offset(((*key).hash & ((1 << (*table).log_size_node as i32) - 1) as u32) as isize) as *mut Node;
        loop {
            if (*node).key.get_tag_variant() == TagVariant::StringShort && &mut (*((*node).key.value.value_object as *mut TString)) as *mut TString == key {
                return &mut (*node).value;
            } else {
                let nx: i32 = (*node).next;
                if nx == 0 {
                    return &ABSENT_KEY;
                }
                node = node.offset(nx as isize);
            }
        }
    }
}
pub unsafe fn luah_getstr(table: *mut Table, key: *mut TString) -> *const TValue {
    unsafe {
        if (*key).get_tag_variant() == TagVariant::StringShort {
            return luah_getshortstr(table, key);
        } else {
            let mut ko: TValue = TValue::new(TagVariant::NilNil);
            let io: *mut TValue = &mut ko;
            let tstring: *mut TString = key;
            (*io).value.value_object = &mut (*(tstring as *mut Object));
            (*io).set_tag_variant((*tstring).get_tag_variant());
            (*io).set_collectable(true);
            return getgeneric(table, &mut ko, 0);
        };
    }
}
pub unsafe fn luah_get(table: *mut Table, key: *const TValue) -> *const TValue {
    unsafe {
        match (*key).get_tag_variant() {
            TagVariant::StringShort => {
                return luah_getshortstr(table, &mut (*((*key).value.value_object as *mut TString)));
            },
            TagVariant::NumericInteger => return luah_getint(table, (*key).value.value_integer),
            TagVariant::NilNil => return &ABSENT_KEY,
            TagVariant::NumericNumber => {
                let mut k: i64 = 0;
                if luav_flttointeger((*key).value.value_number, &mut k, F2I::Equal) {
                    return luah_getint(table, k);
                }
            },
            _ => {},
        }
        return getgeneric(table, key, 0);
    }
}
pub unsafe fn luah_finishset(interpreter: *mut Interpreter, table: *mut Table, key: *const TValue, slot: *const TValue, value: *mut TValue) {
    unsafe {
        if (*slot).get_tag_variant() == TagVariant::NilAbsentKey {
            luah_newkey(interpreter, table, key, value);
        } else {
            let io1: *mut TValue = slot as *mut TValue;
            let io2: *const TValue = value;
            (*io1).copy_from(&*io2);
        };
    }
}
pub unsafe fn luah_set(interpreter: *mut Interpreter, table: *mut Table, key: *const TValue, value: *mut TValue) {
    unsafe {
        let slot: *const TValue = luah_get(table, key);
        luah_finishset(interpreter, table, key, slot, value);
    }
}
pub unsafe fn luah_setint(interpreter: *mut Interpreter, table: *mut Table, key: i64, value: *mut TValue) {
    unsafe {
        let p: *const TValue = luah_getint(table, key);
        if (*p).get_tag_variant() == TagVariant::NilAbsentKey {
            let mut k: TValue = TValue::new(TagVariant::NilNil);
            let io: *mut TValue = &mut k;
            (*io).value.value_integer = key;
            (*io).set_tag_variant(TagVariant::NumericInteger);
            luah_newkey(interpreter, table, &mut k, value);
        } else {
            let io1: *mut TValue = p as *mut TValue;
            let io2: *const TValue = value;
            (*io1).copy_from(&*io2);
        };
    }
}
pub unsafe fn hash_search(table: *mut Table, mut j: usize) -> usize {
    unsafe {
        let mut i: usize;
        if j == 0 {
            j = j.wrapping_add(1);
        }
        loop {
            i = j;
            if j <= MAXIMUM_SIZE / 2 {
                j = (j as usize).wrapping_mul(2 as usize) as usize;
                if (*luah_getint(table, j as i64)).is_tagtype_nil() {
                    break;
                }
            } else {
                j = MAXIMUM_SIZE;
                if (*luah_getint(table, j as i64)).is_tagtype_nil() {
                    break;
                }
                return j;
            }
        }
        while j.wrapping_sub(i) > 1 as usize {
            let m: usize = i.wrapping_add(j) / 2;
            if (*luah_getint(table, m as i64)).is_tagtype_nil() {
                j = m;
            } else {
                i = m;
            }
        }
        return i;
    }
}
pub unsafe fn luah_getn(table: *mut Table) -> usize {
    unsafe {
        let mut limit: u32 = (*table).array_limit;
        if limit > 0u32 && ((*((*table).array).offset(limit.wrapping_sub(1 as u32) as isize)).get_tag_type()) == TagType::Nil {
            if limit >= 2 as u32 && (*((*table).array).offset(limit.wrapping_sub(2 as u32) as isize)).get_tag_type() != TagType::Nil {
                if ispow2realasize(table) != 0 && !(limit.wrapping_sub(1 as u32) & limit.wrapping_sub(1 as u32).wrapping_sub(1 as u32) == 0) {
                    (*table).array_limit = limit.wrapping_sub(1 as u32);
                    (*table).flags = ((*table).flags as i32 | 1 << 7) as u8;
                }
                return limit.wrapping_sub(1 as u32) as usize;
            } else {
                let boundary: u32 = binsearch((*table).array, 0u32, limit);
                if ispow2realasize(table) != 0 && boundary > ((luah_realasize(table)) / 2) {
                    (*table).array_limit = boundary;
                    (*table).flags = ((*table).flags as i32 | 1 << 7) as u8;
                }
                return boundary as usize;
            }
        }
        if !((*table).flags as i32 & 1 << 7 == 0 || (*table).array_limit & ((*table).array_limit).wrapping_sub(1 as u32) == 0) {
            if (*((*table).array).offset(limit as isize)).is_tagtype_nil() {
                return limit as usize;
            }
            limit = luah_realasize(table);
            if (*((*table).array).offset(limit.wrapping_sub(1 as u32) as isize)).is_tagtype_nil() {
                let boundary_0: u32 = binsearch((*table).array, (*table).array_limit, limit);
                (*table).array_limit = boundary_0;
                return boundary_0 as usize;
            }
        }
        if ((*table).last_free).is_null() || ((*luah_getint(table, limit.wrapping_add(1 as u32) as i64)).get_tag_type()) == TagType::Nil {
            return limit as usize;
        } else {
            return hash_search(table, limit as usize);
        };
    }
}
pub unsafe fn luav_finishget(interpreter: *mut Interpreter, mut t: *const TValue, key: *mut TValue, value: *mut TValue, mut slot: *const TValue) {
    unsafe {
        let mut loop_0: i32 = 0;
        let mut tm: *const TValue;
        while loop_0 < 2000 as i32 {
            if slot.is_null() {
                tm = luat_gettmbyobj(interpreter, t, TM_INDEX);
                if (*tm).is_tagtype_nil() {
                    luag_typeerror(interpreter, t, c"index".as_ptr());
                }
            } else {
                tm = if ((*((*t).value.value_object as *mut Table)).get_metatable()).is_null() {
                    null()
                } else if (*(*((*t).value.value_object as *mut Table)).get_metatable()).flags as u32 & (1 as u32) << TM_INDEX as i32 != 0 {
                    null()
                } else {
                    luat_gettm((*((*t).value.value_object as *mut Table)).get_metatable(), TM_INDEX, (*(*interpreter).global).global_tmname[TM_INDEX as usize])
                };
                if tm.is_null() {
                    (*value).set_tag_variant(TagVariant::NilNil);
                    return;
                }
            }
            if (*tm).is_tagtype_closure() {
                luat_calltmres(interpreter, tm, t, key, value);
                return;
            }
            t = tm;
            if if !((*t).get_tag_variant() == TagVariant::Table) {
                slot = null();
                0
            } else {
                slot = luah_get(&mut (*((*t).value.value_object as *mut Table)), key);
                !(*slot).is_tagtype_nil() as i32
            } != 0
            {
                let io1: *mut TValue = &mut (*value);
                let io2: *const TValue = slot;
                (*io1).copy_from(&*io2);
                return;
            }
            loop_0 += 1;
        }
        luag_runerror(interpreter, c"'__index' chain too long; possible loop".as_ptr());
    }
}
pub unsafe fn luav_finishset(interpreter: *mut Interpreter, mut t: *const TValue, key: *mut TValue, value: *mut TValue, mut slot: *const TValue) {
    unsafe {
        let mut loop_0: i32 = 0;
        while loop_0 < 2000 as i32 {
            let tm: *const TValue;
            if !slot.is_null() {
                let h: *mut Table = &mut (*((*t).value.value_object as *mut Table));
                tm = if ((*h).get_metatable()).is_null() {
                    null()
                } else if (*(*h).get_metatable()).flags as u32 & (1 as u32) << TM_NEWINDEX as i32 != 0 {
                    null()
                } else {
                    luat_gettm((*h).get_metatable(), TM_NEWINDEX, (*(*interpreter).global).global_tmname[TM_NEWINDEX as usize])
                };
                if tm.is_null() {
                    let io: *mut TValue = &mut (*(*interpreter).top.stkidrel_pointer);
                    let x_: *mut Table = h;
                    (*io).value.value_object = &mut (*(x_ as *mut Object));
                    (*io).set_tag_variant(TagVariant::Table);
                    (*io).set_collectable(true);
                    (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
                    luah_finishset(interpreter, h, key, slot, value);
                    (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(-1);
                    (*h).flags = ((*h).flags as u32 & !!(!0 << TM_EQ as i32 + 1)) as u8;
                    if (*value).is_collectable() {
                        if (*(h as *mut Object)).get_marked() & 1 << 5 != 0 && (*(*value).value.value_object).get_marked() & (1 << 3 | 1 << 4) != 0 {
                            luac_barrierback_(interpreter, &mut (*(h as *mut Object)));
                        } else {
                        };
                    } else {
                    };
                    return;
                }
            } else {
                tm = luat_gettmbyobj(interpreter, t, TM_NEWINDEX);
                if (*tm).is_tagtype_nil() {
                    luag_typeerror(interpreter, t, c"index".as_ptr());
                }
            }
            if (*tm).is_tagtype_closure() {
                luat_calltm(interpreter, tm, t, key, value);
                return;
            }
            t = tm;
            if if (*t).get_tag_variant() != TagVariant::Table {
                slot = null();
                0
            } else {
                slot = luah_get(&mut (*((*t).value.value_object as *mut Table)), key);
                !(*slot).is_tagtype_nil() as i32
            } != 0
            {
                let io1: *mut TValue = slot as *mut TValue;
                let io2: *const TValue = value;
                (*io1).copy_from(&*io2);
                if (*value).is_collectable() {
                    if (*(*t).value.value_object).get_marked() & 1 << 5 != 0 && (*(*value).value.value_object).get_marked() & (1 << 3 | 1 << 4) != 0 {
                        luac_barrierback_(interpreter, (*t).value.value_object);
                    } else {
                    };
                } else {
                };
                return;
            }
            loop_0 += 1;
        }
        luag_runerror(interpreter, c"'__newindex' chain too long; possible loop".as_ptr());
    }
}
