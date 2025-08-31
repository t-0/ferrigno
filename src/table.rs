use crate::new::*;
use crate::node::*;
use crate::object::*;
use crate::tag::*;
use crate::tstring::*;
use crate::tm::*;
use crate::global::*;
use crate::onelua::*;
use crate::tvalue::*;
use libc::*;
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct Table {
    pub next: *mut Object,
    pub tag: u8,
    pub marked: u8,
    pub dummy0: u8 = 0,
    pub dummy1: u8 = 0,
    pub dummy2: u32 = 0,
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
    fn get_marked(&self) -> u8 {
        self.marked
    }
    fn set_marked(&mut self, marked_: u8) {
        self.marked = marked_;
    }
    fn set_tag(&mut self, tag: u8) {
        self.tag = tag;
    }
    fn set_collectable(&mut self) {
        self.tag = set_collectable(self.tag);
    }
    fn is_collectable(&self) -> bool {
        is_collectable(self.tag)
    }
    fn get_tag(&self) -> u8 {
        self.tag
    }
    fn get_tag_type(&self) -> u8 {
        get_tag_type(self.get_tag())
    }
    fn get_tag_variant(&self) -> u8 {
        get_tag_variant(self.tag)
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
            next: std::ptr::null_mut(),
            tag: TAG_VARIANT_TABLE,
            marked: 0,
            dummy0: 0,
            dummy1: 0,
            dummy2: 0,
            dummy3: 0,
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
                clearkey(node);
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
        if (*g).gcstate as i32 == 2 && hasclears != 0 {
            linkgclist_(
                &mut (*(h as *mut Object)),
                &mut (*h).gc_list,
                &mut (*g).weak,
            );
        } else {
            linkgclist_(
                &mut (*(h as *mut Object)),
                &mut (*h).gc_list,
                &mut (*g).grayagain,
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
        let mut i: u32 = 0;
        while i < asize {
            if ((*((*h).array).offset(i as isize)).is_collectable())
                && (*(*((*h).array).offset(i as isize)).value.object).get_marked()
                    & (1 << 3 | 1 << 4)
                    != 0
            {
                marked = 1;
                reallymarkobject(g, (*((*h).array).offset(i as isize)).value.object);
            }
            i = i.wrapping_add(1);
        }
        i = 0u32;
        while i < new_size {
            let node: *mut Node = if inv != 0 {
                &mut *((*h).node).offset(new_size.wrapping_sub(1 as u32).wrapping_sub(i) as isize)
                    as *mut Node
            } else {
                &mut *((*h).node).offset(i as isize) as *mut Node
            };
            if get_tag_type((*node).value.get_tag()) == TAG_TYPE_NIL {
                clearkey(node);
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
            i = i.wrapping_add(1);
        }
        if (*g).gcstate as i32 == 0 {
            linkgclist_(
                &mut (*(h as *mut Object)),
                &mut (*h).gc_list,
                &mut (*g).grayagain,
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
                &mut (*g).allweak,
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
        let mut i: u32 = 0u32;
        while i < asize {
            if ((*((*h).array).offset(i as isize)).is_collectable())
                && (*(*((*h).array).offset(i as isize)).value.object).get_marked()
                    & (1 << 3 | 1 << 4)
                    != 0
            {
                reallymarkobject(g, (*((*h).array).offset(i as isize)).value.object);
            }
            i = i.wrapping_add(1);
        }
        let mut node: *mut Node = &mut *((*h).node).offset(0 as isize) as *mut Node;
        while node < limit {
            if get_tag_type((*node).value.get_tag()) == TAG_TYPE_NIL {
                clearkey(node);
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
                (*g).tmname[TM_MODE as usize],
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
            weakkey = strchr((*smode).get_contents(), 'k' as i32);
            weakvalue = strchr((*smode).get_contents(), 'v' as i32);
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
                    &mut (*g).allweak,
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
        let mut i: i32;
        i = old_size;
        while i < new_size {
            let ref mut fresh20 = *vect.offset(i as isize);
            *fresh20 = std::ptr::null_mut();
            i += 1;
        }
        i = 0;
        while i < old_size {
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
            i += 1;
        }
    }
}
