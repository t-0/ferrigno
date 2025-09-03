use crate::new::*;
use crate::character::*;
use crate::node::*;
use crate::object::*;
use crate::tag::*;
use crate::tstring::*;
use crate::tm::*;
use crate::global::*;
use crate::state::*;
use crate::value::*;
use crate::f2i::*;
use crate::functions::*;
use crate::stackvalue::*;
use crate::utility::*;
use crate::tvalue::*;
use libc::*;
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct Table {
    pub object: Object,
    pub flags: u8,
    pub log_size_node: u8,
    pub dummy3: u8 = 0,
    pub array_limit: u32,
    pub array: *mut TValue,
    pub node: *mut Node,
    pub last_free: *mut Node,
    pub metatable: *mut Table,
    pub gc_list: *mut Object,
}
impl TObject for Table {
    fn get_tag(&self) -> u8 {
        self.object.tag
    }
    fn set_tag(&mut self, tag: u8) {
        self.object.tag = tag;
    }
    fn get_marked(&self) -> u8 {
        self.object.marked
    }
    fn set_marked(&mut self, marked_: u8) {
        self.object.marked = marked_;
    }
    fn get_class_name(&mut self) -> String {
        "table".to_string()
    }
    fn get_metatable(&mut self) -> *mut Table {
        self.metatable
    }
}
impl New for Table {
    fn new() -> Self {
        Table {
            object: Object {
                next: std::ptr::null_mut(),
                tag: TAG_VARIANT_TABLE,
                marked: 0,
                ..
            },
            flags: 0,
            log_size_node: 0,
            array_limit: 0,
            array: std::ptr::null_mut(),
            node: std::ptr::null_mut(),
            last_free: std::ptr::null_mut(),
            metatable: std::ptr::null_mut(),
            gc_list: std::ptr::null_mut(),
            ..
        }
    }
}
impl Table {
    pub unsafe extern "C" fn exchange_hash_part(t1: *mut Table, t2: *mut Table) {
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
    pub unsafe extern "C" fn get_free_position(&mut self) -> *mut Node {
        unsafe {
            if !self.last_free.is_null() {
                while self.last_free > self.node {
                    self.last_free = self.last_free.offset(-1);
                    self.last_free;
                    if (*self.last_free).key.tag == TAG_VARIANT_NIL_NIL {
                        return self.last_free;
                    }
                }
            }
            return std::ptr::null_mut();
        }
    }
}
pub unsafe extern "C" fn luat_gettm(
    events: *mut Table,
    event: u32,
    ename: *mut TString,
) -> *const TValue {
    unsafe {
        let tm: *const TValue = luah_getshortstr(events, ename);
        if get_tag_type((*tm).get_tag()) == TAG_TYPE_NIL {
            (*events).flags =
                ((*events).flags as i32 | ((1 as u32) << event as u32) as u8 as i32) as u8;
            return std::ptr::null();
        } else {
            return tm;
        };
    }
}
pub unsafe extern "C" fn traverseweakvalue(g: *mut Global, h: *mut Table) {
    unsafe {
        let limit: *mut Node =
            &mut *((*h).node).offset((1 << (*h).log_size_node as i32) as isize) as *mut Node;
        let mut hasclears: i32 = ((*h).array_limit > 0u32) as i32;
        let mut node: *mut Node = &mut *((*h).node).offset(0 as isize) as *mut Node;
        while node < limit {
            if get_tag_type((*node).value.get_tag()) == TAG_TYPE_NIL {
                (*node).clearkey();
            } else {
                if is_collectable((*node).key.tag)
                    && (*(*node).key.value.object).get_marked() & (1 << 3 | 1 << 4) != 0
                {
                    reallymarkobject(g, (*node).key.value.object);
                }
                if hasclears == 0
                    && iscleared(
                        g,
                        if (*node).value.is_collectable() {
                            (*node).value.value.object
                        } else {
                            std::ptr::null_mut()
                        },
                    ) != 0
                {
                    hasclears = 1;
                }
            }
            node = node.offset(1);
        }
        if (*g).gc_state as i32 == 2 && hasclears != 0 {
            linkgclist_(
                &mut (*(h as *mut Object)),
                &mut (*h).gc_list,
                &mut (*g).weak,
            );
        } else {
            linkgclist_(
                &mut (*(h as *mut Object)),
                &mut (*h).gc_list,
                &mut (*g).gray_again,
            );
        };
    }
}
pub unsafe extern "C" fn traverseephemeron(g: *mut Global, h: *mut Table, inv: i32) -> i32 {
    unsafe {
        let mut marked: i32 = 0;
        let mut hasclears: i32 = 0;
        let mut hasww: i32 = 0;
        let asize: u32 = luah_realasize(h);
        let new_size: u32 = (1 << (*h).log_size_node as i32) as u32;
        for i in 0..asize {
            if ((*((*h).array).offset(i as isize)).is_collectable())
                && (*(*((*h).array).offset(i as isize)).value.object).get_marked()
                    & (1 << 3 | 1 << 4)
                    != 0
            {
                marked = 1;
                reallymarkobject(g, (*((*h).array).offset(i as isize)).value.object);
            }
        }
        for i in 0..new_size {
            let node: *mut Node = if inv != 0 {
                &mut *((*h).node).offset(new_size.wrapping_sub(1 as u32).wrapping_sub(i) as isize)
                    as *mut Node
            } else {
                &mut *((*h).node).offset(i as isize) as *mut Node
            };
            if get_tag_type((*node).value.get_tag()) == TAG_TYPE_NIL {
                (*node).clearkey();
            } else if iscleared(
                g,
                if is_collectable((*node).key.tag) {
                    (*node).key.value.object
                } else {
                    std::ptr::null_mut()
                },
            ) != 0
            {
                hasclears = 1;
                if ((*node).value.is_collectable())
                    && (*(*node).value.value.object).get_marked() & (1 << 3 | 1 << 4) != 0
                {
                    hasww = 1;
                }
            } else if ((*node).value.is_collectable())
                && (*(*node).value.value.object).get_marked() & (1 << 3 | 1 << 4) != 0
            {
                marked = 1;
                reallymarkobject(g, (*node).value.value.object);
            }
        }
        if (*g).gc_state as i32 == 0 {
            linkgclist_(
                &mut (*(h as *mut Object)),
                &mut (*h).gc_list,
                &mut (*g).gray_again,
            );
        } else if hasww != 0 {
            linkgclist_(
                &mut (*(h as *mut Object)),
                &mut (*h).gc_list,
                &mut (*g).ephemeron,
            );
        } else if hasclears != 0 {
            linkgclist_(
                &mut (*(h as *mut Object)),
                &mut (*h).gc_list,
                &mut (*g).all_weak,
            );
        } else {
            genlink(g, &mut (*(h as *mut Object)));
        }
        return marked;
    }
}
pub unsafe extern "C" fn traversestrongtable(g: *mut Global, h: *mut Table) {
    unsafe {
        let limit: *mut Node =
            &mut *((*h).node).offset((1 << (*h).log_size_node as i32) as isize) as *mut Node;
        let asize: u32 = luah_realasize(h);
        for i in 0..asize {
            if ((*((*h).array).offset(i as isize)).is_collectable())
                && (*(*((*h).array).offset(i as isize)).value.object).get_marked()
                    & (1 << 3 | 1 << 4)
                    != 0
            {
                reallymarkobject(g, (*((*h).array).offset(i as isize)).value.object);
            }
        }
        let mut node: *mut Node = &mut *((*h).node).offset(0 as isize) as *mut Node;
        while node < limit {
            if get_tag_type((*node).value.get_tag()) == TAG_TYPE_NIL {
                (*node).clearkey();
            } else {
                if is_collectable((*node).key.tag)
                    && (*(*node).key.value.object).get_marked() & (1 << 3 | 1 << 4) != 0
                {
                    reallymarkobject(g, (*node).key.value.object);
                }
                if ((*node).value.is_collectable())
                    && (*(*node).value.value.object).get_marked() & (1 << 3 | 1 << 4) != 0
                {
                    reallymarkobject(g, (*node).value.value.object);
                }
            }
            node = node.offset(1);
        }
        genlink(g, &mut (*(h as *mut Object)));
    }
}
pub unsafe extern "C" fn traversetable(g: *mut Global, h: *mut Table) -> u64 {
    unsafe {
        let mut weakkey: *const i8 = std::ptr::null();
        let mut weakvalue: *const i8 = std::ptr::null();
        let mode: *const TValue = if ((*h).metatable).is_null() {
            std::ptr::null()
        } else if (*(*h).metatable).flags as u32 & (1 as u32) << TM_MODE as i32 != 0 {
            std::ptr::null()
        } else {
            luat_gettm(
                (*h).metatable,
                TM_MODE,
                (*g).tm_name[TM_MODE as usize],
            )
        };
        let smode: *mut TString;
        if !((*h).metatable).is_null() {
            if (*(*h).metatable).get_marked() & (1 << 3 | 1 << 4) != 0 {
                reallymarkobject(g, &mut (*((*h).metatable as *mut Object)));
            }
        }
        if !mode.is_null() && (*mode).get_tag_variant() == TAG_VARIANT_STRING_SHORT && {
            smode = &mut (*((*mode).value.object as *mut TString)) as *mut TString;
            weakkey = strchr((*smode).get_contents(), CHARACTER_LOWER_K as i32);
            weakvalue = strchr((*smode).get_contents(), CHARACTER_LOWER_V as i32);
            !weakkey.is_null() || !weakvalue.is_null()
        } {
            if weakkey.is_null() {
                traverseweakvalue(g, h);
            } else if weakvalue.is_null() {
                traverseephemeron(g, h, 0);
            } else {
                linkgclist_(
                    &mut (*(h as *mut Object)),
                    &mut (*h).gc_list,
                    &mut (*g).all_weak,
                );
            }
        } else {
            traversestrongtable(g, h);
        }
        return (1 as u32).wrapping_add((*h).array_limit).wrapping_add(
            (2 * (if ((*h).last_free).is_null() {
                0
            } else {
                1 << (*h).log_size_node as i32
            })) as u32,
        ) as u64;
    }
}
pub unsafe extern "C" fn tablerehash(vect: *mut *mut TString, old_size: i32, new_size: i32) {
    unsafe {
        for i in old_size..new_size {
            let ref mut fresh20 = *vect.offset(i as isize);
            *fresh20 = std::ptr::null_mut();
        }
        for i in 0..old_size {
            let mut p: *mut TString = *vect.offset(i as isize);
            let ref mut fresh21 = *vect.offset(i as isize);
            *fresh21 = std::ptr::null_mut();
            while !p.is_null() {
                let hash_next: *mut TString = (*p).u.hash_next;
                let h: u32 = ((*p).hash & (new_size - 1) as u32) as u32;
                (*p).u.hash_next = *vect.offset(h as isize);
                let ref mut fresh22 = *vect.offset(h as isize);
                *fresh22 = p;
                p = hash_next;
            }
        }
    }
}
pub unsafe extern "C" fn hashint(t: *const Table, i: i64) -> *mut Node {
    unsafe {
        let ui: u64 = i as u64;
        if ui <= 0x7FFFFFFF as u64 {
            return &mut *((*t).node)
                .offset((ui as i32 % ((1 << (*t).log_size_node as i32) - 1 | 1)) as isize)
                as *mut Node;
        } else {
            return &mut *((*t).node)
                .offset(ui.wrapping_rem(((1 << (*t).log_size_node as i32) - 1 | 1) as u64) as isize)
                as *mut Node;
        };
    }
}
pub unsafe extern "C" fn mainpositiontv(t: *const Table, key: *const TValue) -> *mut Node {
    unsafe {
        match (*key).get_tag_variant() {
            TAG_VARIANT_NUMERIC_INTEGER => {
                let i: i64 = (*key).value.integer;
                return hashint(t, i);
            }
            TAG_VARIANT_NUMERIC_NUMBER => {
                let n: f64 = (*key).value.number;
                return &mut *((*t).node).offset(
                    ((l_hashfloat as unsafe extern "C" fn(f64) -> i32)(n)
                        % ((1 << (*t).log_size_node as i32) - 1 | 1)) as isize,
                ) as *mut Node;
            }
            TAG_VARIANT_STRING_SHORT => {
                let ts: *mut TString = &mut (*((*key).value.object as *mut TString));
                return &mut *((*t).node).offset(
                    ((*ts).hash & ((1 << (*t).log_size_node as i32) - 1) as u32) as isize,
                ) as *mut Node;
            }
            TAG_VARIANT_STRING_LONG => {
                let ts_0: *mut TString = &mut (*((*key).value.object as *mut TString));
                return &mut *((*t).node).offset(
                    ((hash_string_long as unsafe extern "C" fn(*mut TString) -> u32)(ts_0)
                        & ((1 << (*t).log_size_node as i32) - 1) as u32) as i32
                        as isize,
                ) as *mut Node;
            }
            TAG_VARIANT_BOOLEAN_FALSE => {
                return &mut *((*t).node).offset((0 & (1 << (*t).log_size_node as i32) - 1) as isize)
                    as *mut Node;
            }
            TAG_VARIANT_BOOLEAN_TRUE => {
                return &mut *((*t).node).offset((1 & (1 << (*t).log_size_node as i32) - 1) as isize)
                    as *mut Node;
            }
            TAG_VARIANT_POINTER => {
                let p: *mut libc::c_void = (*key).value.pointer;
                return &mut *((*t).node).offset(
                    ((p as u64
                        & (0x7FFFFFFF as u32)
                            .wrapping_mul(2 as u32)
                            .wrapping_add(1 as u32) as u64) as u32)
                        .wrapping_rem(((1 << (*t).log_size_node as i32) - 1 | 1) as u32)
                        as isize,
                ) as *mut Node;
            }
            TAG_VARIANT_CLOSURE_CFUNCTION => {
                let cfunction: CFunction = (*key).value.function;
                return &mut *((*t).node).offset(
                    ((::core::mem::transmute::<CFunction, u64>(cfunction)
                        & (0x7FFFFFFF as u32)
                            .wrapping_mul(2 as u32)
                            .wrapping_add(1 as u32) as u64) as u32)
                        .wrapping_rem(((1 << (*t).log_size_node as i32) - 1 | 1) as u32)
                        as isize,
                ) as *mut Node;
            }
            _ => {
                let o: *mut Object = (*key).value.object;
                return &mut *((*t).node).offset(
                    ((o as u64
                        & (0x7FFFFFFF as u32)
                            .wrapping_mul(2 as u32)
                            .wrapping_add(1 as u32) as u64) as u32)
                        .wrapping_rem(((1 << (*t).log_size_node as i32) - 1 | 1) as u32)
                        as isize,
                ) as *mut Node;
            }
        };
    }
}
pub unsafe extern "C" fn mainpositionfromnode(t: *const Table, nd: *mut Node) -> *mut Node {
    unsafe {
        let mut key: TValue = TValue {
            value: Value {
                object: std::ptr::null_mut(),
            },
            tag: 0,
        };
        let io_: *mut TValue = &mut key;
        let node: *const Node = nd;
        (*io_).value = (*node).key.value;
        (*io_).set_tag((*node).key.tag);
        return mainpositiontv(t, &mut key);
    }
}
pub unsafe extern "C" fn luah_realasize(t: *const Table) -> u32 {
    unsafe {
        if (*t).flags as i32 & 1 << 7 == 0
            || (*t).array_limit & ((*t).array_limit).wrapping_sub(1 as u32) == 0u32
        {
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
pub unsafe extern "C" fn ispow2realasize(t: *const Table) -> i32 {
    unsafe {
        return ((*t).flags as i32 & 1 << 7 != 0
            || (*t).array_limit & ((*t).array_limit).wrapping_sub(1 as u32) == 0u32)
            as i32;
    }
}
pub unsafe extern "C" fn setlimittosize(table: *mut Table) -> u32 {
    unsafe {
        (*table).array_limit = luah_realasize(table);
        (*table).flags = ((*table).flags as i32 & !(1 << 7) as u8 as i32) as u8;
        return (*table).array_limit;
    }
}
pub unsafe extern "C" fn getgeneric(
    table: *mut Table,
    key: *const TValue,
    deadok: i32,
) -> *const TValue {
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
pub unsafe extern "C" fn findindex(
    state: *mut State,
    table: *mut Table,
    key: *mut TValue,
    asize: u32,
) -> u32 {
    unsafe {
        let mut i: u32;
        if get_tag_type((*key).get_tag()) == TAG_TYPE_NIL {
            return 0u32;
        }
        i = if (*key).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
            arrayindex((*key).value.integer)
        } else {
            0u32
        };
        if i.wrapping_sub(1 as u32) < asize {
            return i;
        } else {
            let n_value: *const TValue = getgeneric(table, key, 1);
            if (((*n_value).get_tag() == TAG_VARIANT_NIL_ABSENTKEY) as i32 != 0) as i64 != 0
            {
                luag_runerror(state, b"invalid key to 'next'\0" as *const u8 as *const i8);
            }
            i = (n_value as *mut Node)
                .offset_from(&mut *((*table).node).offset(0 as isize) as *mut Node)
                as u32;
            return i.wrapping_add(1 as u32).wrapping_add(asize);
        };
    }
}
pub unsafe extern "C" fn luah_next(state: *mut State, table: *mut Table, key: StackValuePointer) -> i32 {
    unsafe {
        let asize: u32 = luah_realasize(table);
        let mut i: u32 = findindex(state, table, &mut (*key).tvalue, asize);
        while i < asize {
            if get_tag_type((*((*table).array).offset(i as isize)).get_tag()) != TAG_TYPE_NIL {
                let io: *mut TValue = &mut (*key).tvalue;
                (*io).value.integer = i.wrapping_add(1 as u32) as i64;
                (*io).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                let io1: *mut TValue = &mut (*key.offset(1 as isize)).tvalue;
                let io2: *const TValue = &mut *((*table).array).offset(i as isize) as *mut TValue;
                (*io1).value = (*io2).value;
                (*io1).set_tag((*io2).get_tag());
                return 1;
            }
            i = i.wrapping_add(1);
        }
        i = i.wrapping_sub(asize);
        while (i as i32) < 1 << (*table).log_size_node as i32 {
            if !(get_tag_type((*((*table).node).offset(i as isize)).value.get_tag())
                == TAG_TYPE_NIL)
            {
                let node: *mut Node = &mut *((*table).node).offset(i as isize) as *mut Node;
                let io_: *mut TValue = &mut (*key).tvalue;
                (*io_).value = (*node).key.value;
                (*io_).set_tag((*node).key.tag);
                let io1_0: *mut TValue = &mut (*key.offset(1 as isize)).tvalue;
                let io2_0: *const TValue = &mut (*node).value;
                (*io1_0).value = (*io2_0).value;
                (*io1_0).set_tag((*io2_0).get_tag());
                return 1;
            }
            i = i.wrapping_add(1);
        }
        return 0;
    }
}
pub unsafe extern "C" fn freehash(state: *mut State, table: *mut Table) {
    unsafe {
        if !((*table).last_free).is_null() {
            (*state).free_memory(
                (*table).node as *mut libc::c_void,
                ((1 << (*table).log_size_node as i32) as u64)
                    .wrapping_mul(::core::mem::size_of::<Node>() as u64),
            );
        }
    }
}
pub unsafe extern "C" fn computesizes(nums: *mut u32, pna: *mut u32) -> u32 {
    unsafe {
        let mut i: i32;
        let mut twotoi: u32;
        let mut a: u32 = 0u32;
        let mut na: u32 = 0u32;
        let mut optimal: u32 = 0u32;
        i = 0;
        twotoi = 1 as u32;
        while twotoi > 0u32 && *pna > twotoi.wrapping_div(2 as u32) {
            a = a.wrapping_add(*nums.offset(i as isize));
            if a > twotoi.wrapping_div(2 as u32) {
                optimal = twotoi;
                na = a;
            }
            i += 1;
            twotoi = twotoi.wrapping_mul(2 as u32);
        }
        *pna = na;
        return optimal;
    }
}
pub unsafe extern "C" fn countint(key: i64, nums: *mut u32) -> i32 {
    unsafe {
        let k: u32 = arrayindex(key);
        if k != 0u32 {
            let ref mut fresh129 = *nums.offset(ceiling_log2(k as u64) as isize);
            *fresh129 = (*fresh129).wrapping_add(1);
            return 1;
        } else {
            return 0;
        };
    }
}
pub unsafe extern "C" fn numusearray(t: *const Table, nums: *mut u32) -> u32 {
    unsafe {
        let mut lg: i32;
        let mut ttlg: u32;
        let mut ause: u32 = 0u32;
        let mut i: u32 = 1 as u32;
        let asize: u32 = (*t).array_limit;
        lg = 0;
        ttlg = 1 as u32;
        while lg
            <= (::core::mem::size_of::<i32>() as u64)
                .wrapping_mul(8 as u64)
                .wrapping_sub(1 as u64) as i32
        {
            let mut lc: u32 = 0u32;
            let mut lim: u32 = ttlg;
            if lim > asize {
                lim = asize;
                if i > lim {
                    break;
                }
            }
            while i <= lim {
                if get_tag_type((*((*t).array).offset(i.wrapping_sub(1 as u32) as isize)).get_tag())
                    != 0
                {
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
pub unsafe extern "C" fn numusehash(t: *const Table, nums: *mut u32, pna: *mut u32) -> i32 {
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
            if !(get_tag_type((*node).value.get_tag()) == TAG_TYPE_NIL) {
                if (*node).key.tag == TAG_VARIANT_NUMERIC_INTEGER {
                    ause += countint((*node).key.value.integer, nums);
                }
                totaluse += 1;
            }
        }
        *pna = (*pna).wrapping_add(ause as u32);
        return totaluse;
    }
}
pub unsafe extern "C" fn setnodevector(state: *mut State, table: *mut Table, mut size: u32) {
    unsafe {
        if size == 0u32 {
            (*table).node = &DUMMY_NODE as *const Node as *mut Node;
            (*table).log_size_node = 0;
            (*table).last_free = std::ptr::null_mut();
        } else {
            let lsize: i32 = ceiling_log2(size as u64) as i32;
            if lsize
                > (::core::mem::size_of::<i32>() as u64)
                    .wrapping_mul(8 as u64)
                    .wrapping_sub(1 as u64) as i32
                    - 1
                || (1 as u32) << lsize
                    > (if ((1 as u32)
                        << (::core::mem::size_of::<i32>() as u64)
                            .wrapping_mul(8 as u64)
                            .wrapping_sub(1 as u64) as i32
                            - 1) as u64
                        <= (!(0u64)).wrapping_div(::core::mem::size_of::<Node>() as u64)
                    {
                        (1 as u32)
                            << (::core::mem::size_of::<i32>() as u64)
                                .wrapping_mul(8 as u64)
                                .wrapping_sub(1 as u64) as i32
                                - 1
                    } else {
                        (!(0u64)).wrapping_div(::core::mem::size_of::<Node>() as u64) as u32
                    })
            {
                luag_runerror(state, b"table overflow\0" as *const u8 as *const i8);
            }
            size = (1 << lsize) as u32;
            (*table).node = luam_malloc_(
                state,
                (size as u64).wrapping_mul(::core::mem::size_of::<Node>() as u64),
            ) as *mut Node;
            for i in 0..size {
                let node: *mut Node = &mut *((*table).node).offset(i as isize) as *mut Node;
                (*node).next = 0;
                (*node).key.tag = 0;
                (*node).value.set_tag(TAG_VARIANT_NIL_EMPTY);
            }
            (*table).log_size_node = lsize as u8;
            (*table).last_free = &mut *((*table).node).offset(size as isize) as *mut Node;
        };
    }
}
pub unsafe extern "C" fn reinsert(state: *mut State, ot: *mut Table, table: *mut Table) {
    unsafe {
        let mut j: i32;
        let size: i32 = 1 << (*ot).log_size_node as i32;
        j = 0;
        while j < size {
            let old: *mut Node = &mut *((*ot).node).offset(j as isize) as *mut Node;
            if !(get_tag_type((*old).value.get_tag()) == TAG_TYPE_NIL) {
                let mut k: TValue = TValue {
                    value: Value {
                        object: std::ptr::null_mut(),
                    },
                    tag: 0,
                };
                let io_: *mut TValue = &mut k;
                let node: *const Node = old;
                (*io_).value = (*node).key.value;
                (*io_).set_tag((*node).key.tag);
                luah_set(state, table, &mut k, &mut (*old).value);
            }
            j += 1;
        }
    }
}
pub unsafe extern "C" fn luah_resize(
    state: *mut State,
    table: *mut Table,
    new_array_size: u32,
    nhsize: u32,
) {
    unsafe {
        let mut new_table: Table = Table::new();
        let old_array_size: u32 = setlimittosize(table);
        let new_array: *mut TValue;
        setnodevector(state, &mut new_table, nhsize);
        if new_array_size < old_array_size {
            (*table).array_limit = new_array_size;
            Table::exchange_hash_part(table, &mut new_table);
            for i in new_array_size..old_array_size {
                if get_tag_type((*((*table).array).offset(i as isize)).get_tag()) != TAG_TYPE_NIL {
                    luah_setint(
                        state,
                        table,
                        i.wrapping_add(1 as u32) as i64,
                        &mut *((*table).array).offset(i as isize),
                    );
                }
            }
            (*table).array_limit = old_array_size;
            Table::exchange_hash_part(table, &mut new_table);
        }
        new_array = luam_realloc_(
            state,
            (*table).array as *mut libc::c_void,
            (old_array_size as u64).wrapping_mul(::core::mem::size_of::<TValue>() as u64),
            (new_array_size as u64).wrapping_mul(::core::mem::size_of::<TValue>() as u64),
        ) as *mut TValue;
        if ((new_array.is_null() && new_array_size > 0u32) as i32 != 0) as i64 != 0 {
            freehash(state, &mut new_table);
            luad_throw(state, 4);
        }
        Table::exchange_hash_part(table, &mut new_table);
        (*table).array = new_array;
        (*table).array_limit = new_array_size;
        for i in old_array_size..new_array_size {
            (*((*table).array).offset(i as isize)).set_tag(TAG_VARIANT_NIL_EMPTY);
        }
        reinsert(state, &mut new_table, table);
        freehash(state, &mut new_table);
    }
}
pub unsafe extern "C" fn luah_resizearray(
    state: *mut State,
    table: *mut Table,
    new_array_size: u32,
) {
    unsafe {
        let new_size: i32 = if ((*table).last_free).is_null() {
            0
        } else {
            1 << (*table).log_size_node as i32
        };
        luah_resize(state, table, new_array_size, new_size as u32);
    }
}
pub unsafe extern "C" fn rehash(state: *mut State, table: *mut Table, ek: *const TValue) {
    unsafe {
        let asize: u32;
        let mut na: u32;
        let mut nums: [u32; 32] = [0; 32];
        let mut i: i32;
        let mut totaluse: i32;
        i = 0;
        while i
            <= (::core::mem::size_of::<i32>() as u64)
                .wrapping_mul(8 as u64)
                .wrapping_sub(1 as u64) as i32
        {
            nums[i as usize] = 0u32;
            i += 1;
        }
        setlimittosize(table);
        na = numusearray(table, nums.as_mut_ptr());
        totaluse = na as i32;
        totaluse += numusehash(table, nums.as_mut_ptr(), &mut na);
        if (*ek).get_tag() == TAG_VARIANT_NUMERIC_INTEGER {
            na = na.wrapping_add(countint((*ek).value.integer, nums.as_mut_ptr()) as u32);
        }
        totaluse += 1;
        asize = computesizes(nums.as_mut_ptr(), &mut na);
        luah_resize(state, table, asize, (totaluse as u32).wrapping_sub(na));
    }
}
pub unsafe extern "C" fn luah_new(state: *mut State) -> *mut Table {
    unsafe {
        let o: *mut Object = luac_newobj(
            state,
            TAG_TYPE_TABLE,
            ::core::mem::size_of::<Table>() as u64,
        );
        let new_table: *mut Table = &mut (*(o as *mut Table));
        (*new_table).metatable = std::ptr::null_mut();
        (*new_table).flags = !(!0 << TM_EQ as i32 + 1) as u8;
        (*new_table).array = std::ptr::null_mut();
        (*new_table).array_limit = 0u32;
        setnodevector(state, new_table, 0u32);
        return new_table;
    }
}
pub unsafe extern "C" fn luah_free(state: *mut State, table: *mut Table) {
    unsafe {
        freehash(state, table);
        (*state).free_memory(
            (*table).array as *mut libc::c_void,
            (luah_realasize(table) as u64).wrapping_mul(::core::mem::size_of::<TValue>() as u64),
        );
        (*state).free_memory(
            table as *mut libc::c_void,
            ::core::mem::size_of::<Table>() as u64,
        );
    }
}
pub unsafe extern "C" fn luah_newkey(
    state: *mut State,
    table: *mut Table,
    mut key: *const TValue,
    value: *mut TValue,
) {
    unsafe {
        let mut mp;
        let mut aux: TValue = TValue {
            value: Value {
                object: std::ptr::null_mut(),
            },
            tag: 0,
        };
        if ((get_tag_type((*key).get_tag()) == TAG_TYPE_NIL) as i32 != 0) as i64 != 0 {
            luag_runerror(state, b"table index is nil\0" as *const u8 as *const i8);
        } else if (*key).get_tag() == TAG_VARIANT_NUMERIC_NUMBER {
            let number = (*key).value.number;
            let mut k: i64 = 0;
            if luav_flttointeger(number, &mut k, F2I::Equal) {
                aux.value.integer = k;
                aux.set_tag(TAG_VARIANT_NUMERIC_INTEGER);
                key = &mut aux;
            } else if number != number {
                luag_runerror(state, b"table index is NaN\0" as *const u8 as *const i8);
            }
        }
        if get_tag_type((*value).get_tag()) == TAG_TYPE_NIL {
            return;
        }
        mp = mainpositiontv(table, key);
        if (get_tag_type((*mp).value.get_tag()) != TAG_TYPE_NIL) || ((*table).last_free).is_null() {
            let mut other_node: *mut Node;
            let f_0: *mut Node = (*table).get_free_position();
            if f_0.is_null() {
                rehash(state, table, key);
                luah_set(state, table, key, value);
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
                (*mp).value.set_tag(TAG_VARIANT_NIL_EMPTY);
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
        (*node).key.value = (*io_).value;
        (*node).key.tag = (*io_).get_tag();
        if (*key).is_collectable() {
            if (*(table as *mut Object)).get_marked() & 1 << 5 != 0
                && (*(*key).value.object).get_marked() & (1 << 3 | 1 << 4) != 0
            {
                luac_barrierback_(state, &mut (*(table as *mut Object)));
            } else {
            };
        } else {
        };
        let io1: *mut TValue = &mut (*mp).value;
        let io2: *const TValue = value;
        (*io1).value = (*io2).value;
        (*io1).set_tag((*io2).get_tag());
    }
}
pub unsafe extern "C" fn luah_getint(table: *mut Table, key: i64) -> *const TValue {
    unsafe {
        let array_limit: u64 = (*table).array_limit as u64;
        if (key as u64).wrapping_sub(1 as u64) < array_limit {
            return &mut *((*table).array).offset((key - 1) as isize) as *mut TValue;
        } else if (*table).flags as i32 & 1 << 7 != 0
            && (key as u64).wrapping_sub(1 as u64)
                & !array_limit.wrapping_sub(1 as u64)
                < array_limit
        {
            (*table).array_limit = key as u32;
            return &mut *((*table).array).offset((key - 1) as isize) as *mut TValue;
        } else {
            let mut node: *mut Node = hashint(table, key);
            loop {
                if (*node).key.tag == TAG_VARIANT_NUMERIC_INTEGER && (*node).key.value.integer == key {
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
pub unsafe extern "C" fn luah_getshortstr(table: *mut Table, key: *mut TString) -> *const TValue {
    unsafe {
        let mut node: *mut Node = &mut *((*table).node).offset(
            ((*key).hash & ((1 << (*table).log_size_node as i32) - 1) as u32) as isize,
        ) as *mut Node;
        loop {
            if get_tag_variant((*node).key.tag) == TAG_VARIANT_STRING_SHORT
                && &mut (*((*node).key.value.object as *mut TString)) as *mut TString == key
            {
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
pub unsafe extern "C" fn luah_getstr(table: *mut Table, key: *mut TString) -> *const TValue {
    unsafe {
        if (*key).get_tag() == TAG_VARIANT_STRING_SHORT {
            return luah_getshortstr(table, key);
        } else {
            let mut ko: TValue = TValue {
                value: Value {
                    object: std::ptr::null_mut(),
                },
                tag: 0,
            };
            let io: *mut TValue = &mut ko;
            let x_: *mut TString = key;
            (*io).value.object = &mut (*(x_ as *mut Object));
            (*io).set_tag((*x_).get_tag());
            (*io).set_collectable();
            return getgeneric(table, &mut ko, 0);
        };
    }
}
pub unsafe extern "C" fn luah_get(table: *mut Table, key: *const TValue) -> *const TValue {
    unsafe {
        match (*key).get_tag_variant() {
            4 => return luah_getshortstr(table, &mut (*((*key).value.object as *mut TString))),
            3 => return luah_getint(table, (*key).value.integer),
            0 => return &ABSENT_KEY,
            19 => {
                let mut k: i64 = 0;
                if luav_flttointeger((*key).value.number, &mut k, F2I::Equal) {
                    return luah_getint(table, k);
                }
            }
            _ => {}
        }
        return getgeneric(table, key, 0);
    }
}
pub unsafe extern "C" fn luah_finishset(
    state: *mut State,
    table: *mut Table,
    key: *const TValue,
    slot: *const TValue,
    value: *mut TValue,
) {
    unsafe {
        if (*slot).get_tag() == TAG_VARIANT_NIL_ABSENTKEY {
            luah_newkey(state, table, key, value);
        } else {
            let io1: *mut TValue = slot as *mut TValue;
            let io2: *const TValue = value;
            (*io1).value = (*io2).value;
            (*io1).set_tag((*io2).get_tag());
        };
    }
}
pub unsafe extern "C" fn luah_set(
    state: *mut State,
    table: *mut Table,
    key: *const TValue,
    value: *mut TValue,
) {
    unsafe {
        let slot: *const TValue = luah_get(table, key);
        luah_finishset(state, table, key, slot, value);
    }
}
pub unsafe extern "C" fn luah_setint(
    state: *mut State,
    table: *mut Table,
    key: i64,
    value: *mut TValue,
) {
    unsafe {
        let p: *const TValue = luah_getint(table, key);
        if (*p).get_tag() == TAG_VARIANT_NIL_ABSENTKEY {
            let mut k: TValue = TValue {
                value: Value {
                    object: std::ptr::null_mut(),
                },
                tag: 0,
            };
            let io: *mut TValue = &mut k;
            (*io).value.integer = key;
            (*io).set_tag(TAG_VARIANT_NUMERIC_INTEGER);
            luah_newkey(state, table, &mut k, value);
        } else {
            let io1: *mut TValue = p as *mut TValue;
            let io2: *const TValue = value;
            (*io1).value = (*io2).value;
            (*io1).set_tag((*io2).get_tag());
        };
    }
}
pub unsafe extern "C" fn hash_search(table: *mut Table, mut j: u64) -> u64 {
    unsafe {
        let mut i: u64;
        if j == 0u64 {
            j = j.wrapping_add(1);
        }
        loop {
            i = j;
            if j <= (0x7FFFFFFFFFFFFFFF as u64).wrapping_div(2 as u64) {
                j = (j as u64).wrapping_mul(2 as u64) as u64;
                if get_tag_type((*luah_getint(table, j as i64)).get_tag()) == TAG_TYPE_NIL {
                    break;
                }
            } else {
                j = 0x7FFFFFFFFFFFFFFF as u64;
                if get_tag_type((*luah_getint(table, j as i64)).get_tag()) == TAG_TYPE_NIL {
                    break;
                }
                return j;
            }
        }
        while j.wrapping_sub(i) > 1 as u64 {
            let m: u64 = i.wrapping_add(j).wrapping_div(2 as u64);
            if get_tag_type((*luah_getint(table, m as i64)).get_tag()) == TAG_TYPE_NIL {
                j = m;
            } else {
                i = m;
            }
        }
        return i;
    }
}
pub unsafe extern "C" fn luah_getn(table: *mut Table) -> u64 {
    unsafe {
        let mut limit: u32 = (*table).array_limit;
        if limit > 0u32
            && get_tag_type(
                (*((*table).array).offset(limit.wrapping_sub(1 as u32) as isize)).get_tag(),
            ) == TAG_TYPE_NIL
        {
            if limit >= 2 as u32
                && !get_tag_type(
                    (*((*table).array).offset(limit.wrapping_sub(2 as u32) as isize)).get_tag(),
                ) == TAG_TYPE_NIL
            {
                if ispow2realasize(table) != 0
                    && !(limit.wrapping_sub(1 as u32)
                        & limit.wrapping_sub(1 as u32).wrapping_sub(1 as u32)
                        == 0u32)
                {
                    (*table).array_limit = limit.wrapping_sub(1 as u32);
                    (*table).flags = ((*table).flags as i32 | 1 << 7) as u8;
                }
                return limit.wrapping_sub(1 as u32) as u64;
            } else {
                let boundary: u32 = binsearch((*table).array, 0u32, limit);
                if ispow2realasize(table) != 0
                    && boundary > (luah_realasize(table)).wrapping_div(2 as u32)
                {
                    (*table).array_limit = boundary;
                    (*table).flags = ((*table).flags as i32 | 1 << 7) as u8;
                }
                return boundary as u64;
            }
        }
        if !((*table).flags as i32 & 1 << 7 == 0
            || (*table).array_limit & ((*table).array_limit).wrapping_sub(1 as u32) == 0u32)
        {
            if get_tag_type((*((*table).array).offset(limit as isize)).get_tag()) == TAG_TYPE_NIL {
                return limit as u64;
            }
            limit = luah_realasize(table);
            if get_tag_type(
                (*((*table).array).offset(limit.wrapping_sub(1 as u32) as isize)).get_tag(),
            ) == TAG_TYPE_NIL
            {
                let boundary_0: u32 = binsearch((*table).array, (*table).array_limit, limit);
                (*table).array_limit = boundary_0;
                return boundary_0 as u64;
            }
        }
        if ((*table).last_free).is_null()
            || get_tag_type((*luah_getint(table, limit.wrapping_add(1 as u32) as i64)).get_tag())
                == TAG_TYPE_NIL
        {
            return limit as u64;
        } else {
            return hash_search(table, limit as u64);
        };
    }
}
pub unsafe extern "C" fn luav_finishget(
    state: *mut State,
    mut t: *const TValue,
    key: *mut TValue,
    value: StackValuePointer,
    mut slot: *const TValue,
) {
    unsafe {
        let mut loop_0: i32 = 0;
        let mut tm: *const TValue;
        while loop_0 < 2000 as i32 {
            if slot.is_null() {
                tm = luat_gettmbyobj(state, t, TM_INDEX);
                if ((get_tag_type((*tm).get_tag()) == TAG_TYPE_NIL) as i32 != 0) as i64 != 0
                {
                    luag_typeerror(state, t, b"index\0" as *const u8 as *const i8);
                }
            } else {
                tm = if ((*((*t).value.object as *mut Table)).metatable).is_null() {
                    std::ptr::null()
                } else if (*(*((*t).value.object as *mut Table)).metatable).flags as u32
                    & (1 as u32) << TM_INDEX as i32
                    != 0
                {
                    std::ptr::null()
                } else {
                    luat_gettm(
                        (*((*t).value.object as *mut Table)).metatable,
                        TM_INDEX,
                        (*(*state).global).tm_name[TM_INDEX as usize],
                    )
                };
                if tm.is_null() {
                    (*value).tvalue.set_tag(TAG_VARIANT_NIL_NIL);
                    return;
                }
            }
            if get_tag_type((*tm).get_tag()) == TAG_TYPE_CLOSURE {
                luat_calltmres(state, tm, t, key, value);
                return;
            }
            t = tm;
            if if !((*t).get_tag_variant() == TAG_VARIANT_TABLE) {
                slot = std::ptr::null();
                0
            } else {
                slot = luah_get(&mut (*((*t).value.object as *mut Table)), key);
                (get_tag_type((*slot).get_tag()) != TAG_TYPE_NIL) as i32
            } != 0
            {
                let io1: *mut TValue = &mut (*value).tvalue;
                let io2: *const TValue = slot;
                (*io1).value = (*io2).value;
                (*io1).set_tag((*io2).get_tag());
                return;
            }
            loop_0 += 1;
        }
        luag_runerror(
            state,
            b"'__index' chain too long; possible loop\0" as *const u8 as *const i8,
        );
    }
}
pub unsafe extern "C" fn luav_finishset(
    state: *mut State,
    mut t: *const TValue,
    key: *mut TValue,
    value: *mut TValue,
    mut slot: *const TValue,
) {
    unsafe {
        let mut loop_0: i32 = 0;
        while loop_0 < 2000 as i32 {
            let tm: *const TValue;
            if !slot.is_null() {
                let h: *mut Table = &mut (*((*t).value.object as *mut Table));
                tm = if ((*h).metatable).is_null() {
                    std::ptr::null()
                } else if (*(*h).metatable).flags as u32 & (1 as u32) << TM_NEWINDEX as i32 != 0 {
                    std::ptr::null()
                } else {
                    luat_gettm(
                        (*h).metatable,
                        TM_NEWINDEX,
                        (*(*state).global).tm_name[TM_NEWINDEX as usize],
                    )
                };
                if tm.is_null() {
                    let io: *mut TValue = &mut (*(*state).top.p).tvalue;
                    let x_: *mut Table = h;
                    (*io).value.object = &mut (*(x_ as *mut Object));
                    (*io).set_tag(TAG_VARIANT_TABLE);
                    (*io).set_collectable();
                    (*state).top.p = (*state).top.p.offset(1);
                    luah_finishset(state, h, key, slot, value);
                    (*state).top.p = (*state).top.p.offset(-1);
                    (*h).flags = ((*h).flags as u32 & !!(!0 << TM_EQ as i32 + 1)) as u8;
                    if (*value).is_collectable() {
                        if (*(h as *mut Object)).get_marked() & 1 << 5 != 0
                            && (*(*value).value.object).get_marked() & (1 << 3 | 1 << 4) != 0
                        {
                            luac_barrierback_(state, &mut (*(h as *mut Object)));
                        } else {
                        };
                    } else {
                    };
                    return;
                }
            } else {
                tm = luat_gettmbyobj(state, t, TM_NEWINDEX);
                if ((get_tag_type((*tm).get_tag()) == TAG_TYPE_NIL) as i32 != 0) as i64 != 0
                {
                    luag_typeerror(state, t, b"index\0" as *const u8 as *const i8);
                }
            }
            if get_tag_type((*tm).get_tag()) == TAG_TYPE_CLOSURE {
                luat_calltm(state, tm, t, key, value);
                return;
            }
            t = tm;
            if if !((*t).get_tag_variant() == TAG_VARIANT_TABLE) {
                slot = std::ptr::null();
                0
            } else {
                slot = luah_get(&mut (*((*t).value.object as *mut Table)), key);
                (get_tag_type((*slot).get_tag()) != TAG_TYPE_NIL) as i32
            } != 0
            {
                let io1: *mut TValue = slot as *mut TValue;
                let io2: *const TValue = value;
                (*io1).value = (*io2).value;
                (*io1).set_tag((*io2).get_tag());
                if (*value).is_collectable() {
                    if (*(*t).value.object).get_marked() & 1 << 5 != 0
                        && (*(*value).value.object).get_marked() & (1 << 3 | 1 << 4) != 0
                    {
                        luac_barrierback_(state, (*t).value.object);
                    } else {
                    };
                } else {
                };
                return;
            }
            loop_0 += 1;
        }
        luag_runerror(
            state,
            b"'__newindex' chain too long; possible loop\0" as *const u8 as *const i8,
        );
    }
}
