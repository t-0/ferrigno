use crate::functions::*;
use crate::object::*;
use crate::state::*;
use crate::stringtable::*;
use crate::table::*;
use crate::tag::*;
use crate::tstring::*;
use crate::node::*;
use crate::upvalue::*;
use crate::user::*;
use crate::prototype::*;
use crate::closure::*;
use crate::tvalue::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Global {
    pub total_bytes: i64,
    pub gc_debt: i64,
    pub gc_estimate: u64,
    pub last_atomic: u64,
    pub string_table: StringTable,
    pub l_registry: TValue,
    pub nil_value: TValue,
    pub seed: u32,
    pub current_white: u8,
    pub gc_state: u8,
    pub gc_kind: u8,
    pub gcstopem: u8,
    pub generational_minor_multiplier: u64,
    pub generational_major_multiplier: u64,
    pub gc_step: u8,
    pub is_emergency: bool,
    pub gc_pause: u8,
    pub gc_step_multiplier: u8,
    pub gc_step_size: u8,
    pub all_gc: *mut Object,
    pub sweep_gc: *mut *mut Object,
    pub finalized_objects: *mut Object,
    pub gray: *mut Object,
    pub gray_again: *mut Object,
    pub weak: *mut Object,
    pub ephemeron: *mut Object,
    pub all_weak: *mut Object,
    pub to_be_finalized: *mut Object,
    pub fixed_gc: *mut Object,
    pub survival: *mut Object,
    pub old1: *mut Object,
    pub really_old: *mut Object,
    pub first_old1: *mut Object,
    pub finobjsur: *mut Object,
    pub finobjold1: *mut Object,
    pub finobjrold: *mut Object,
    pub twups: *mut State,
    pub panic: CFunction,
    pub main_state: *mut State,
    pub memory_error_message: *mut TString,
    pub tm_name: [*mut TString; 25],
    pub metatables: [*mut Table; 9],
    pub string_cache: [[*mut TString; 2]; 53],
    pub warn_function: WarnFunction,
    pub warn_userdata: *mut libc::c_void,
}
impl Global {
    pub unsafe extern "C" fn stringcache_clear(&mut self) {
        unsafe {
            for i in 0..GLOBAL_STRINGCACHE_N {
                for j in 0..GLOBAL_STRINGCACHE_M {
                    if (*self.string_cache[i as usize][j as usize]).get_marked() & (1 << 3 | 1 << 4)
                        != 0
                    {
                        self.string_cache[i as usize][j as usize] = self.memory_error_message;
                    }
                }
            }
        }
    }
    pub unsafe extern "C" fn fix_memory_error_message_global(& mut self) {
        unsafe {
            fix_object_global(self, self.memory_error_message as *mut Object);
        }
    }
    pub fn stringcache_set_error(&mut self) {
        for i in 0..GLOBAL_STRINGCACHE_N {
            for j in 0..GLOBAL_STRINGCACHE_M {
                self.string_cache[i][j] = self.memory_error_message;
            }
        }
    }
    pub unsafe extern "C" fn white_list(&mut self, mut p: *mut Object) {
        unsafe {
            let white = self.current_white & ((1 << 3) | (1 << 4));
            while !p.is_null() {
                (*p).set_marked(
                    (*p).get_marked() & !((1 << 5) | ((1 << 3) | (1 << 4)) | 7) | white,
                );
                p = (*p).next;
            }
        }
    }
    pub unsafe extern "C" fn enter_incremental(&mut self) {
        unsafe {
            self.white_list(self.all_gc);
            self.survival = std::ptr::null_mut();
            self.old1 = self.survival;
            self.really_old = self.old1;
            self.white_list(self.finalized_objects);
            self.white_list(self.to_be_finalized);
            self.finobjsur = std::ptr::null_mut();
            self.finobjold1 = self.finobjsur;
            self.finobjrold = self.finobjold1;
            self.gc_state = 8i32 as u8;
            self.gc_kind = 0i32 as u8;
            self.last_atomic = 0i32 as u64;
        }
    }
    pub unsafe extern "C" fn set_debt(&mut self, mut debt: i64) {
        let tb: i64 = (self.total_bytes + self.gc_debt) as i64;
        if debt < tb - (!(0i32 as u64) >> 1i32) as i64 {
            debt = tb - (!(0i32 as u64) >> 1i32) as i64;
        }
        self.total_bytes = tb - debt;
        self.gc_debt = debt;
    }
    pub unsafe extern "C" fn set_minor_debt(&mut self) {
        unsafe {
            self.set_debt(
                -((self.total_bytes + self.gc_debt).wrapping_div(100) * self.generational_minor_multiplier as i64),
            );
        }
    }
    pub unsafe extern "C" fn propagatemark(&mut self) -> u64 {
        unsafe {
            let object: *mut Object = self.gray;
            (*object).set_marked((*object).get_marked() | 1 << 5);
            self.gray = *getgclist(object);
            match (*object).get_tag_variant() {
                TAG_VARIANT_TABLE => return traversetable(self, &mut (*(object as *mut Table))),
                TAG_VARIANT_USER => return (*(object as *mut User)).traverseudata(self) as u64,
                TAG_VARIANT_CLOSURE_L => return Closure::traverselclosure(self, &mut (*(object as *mut Closure))),
                TAG_VARIANT_CLOSURE_C => return Closure::traversecclosure(self, &mut (*(object as *mut Closure))),
                TAG_VARIANT_PROTOTYPE => return Prototype::traverseproto(self, &mut (*(object as *mut Prototype))),
                TAG_VARIANT_STATE => return traverse_state(self, &mut (*(object as *mut State))) as u64,
                _ => return 0,
            };
        }
    }
    pub unsafe extern "C" fn markmt(& mut self) {
        unsafe {
            for i in TAG_SIMPLE_ {
                if !(self.metatables[i as usize]).is_null() {
                    if (*self.metatables[i as usize]).get_marked() & (1 << 3 | 1 << 4) != 0 {
                        really_mark_object(
                            self,
                            &mut (*(*(self.metatables).as_mut_ptr().offset(i as isize) as *mut Object)),
                        );
                    }
                }
            }
        }
    }
}
pub unsafe extern "C" fn markbeingfnz(global: *mut Global) -> u64 {
    unsafe {
        let mut count: u64 = 0;
        let mut object: *mut Object = (*global).to_be_finalized;
        while !object.is_null() {
            count = count.wrapping_add(1);
            if (*object).get_marked() & (1 << 3 | 1 << 4) != 0 {
                really_mark_object(global, &mut (*(object as *mut Object)));
            }
            object = (*object).next;
        }
        return count;
    }
}
pub unsafe extern "C" fn remarkupvals(global: *mut Global) -> i32 {
    unsafe {
        let mut p: *mut *mut State = &mut (*global).twups;
        let mut work: i32 = 0;
        loop {
            let thread: *mut State = *p;
            if thread.is_null() {
                break;
            }
            work += 1;
            if (*thread).get_marked() & (1 << 3 | 1 << 4) == 0
                && !((*thread).open_upvalue).is_null()
            {
                p = &mut (*thread).twups;
            } else {
                *p = (*thread).twups;
                (*thread).twups = thread;
                let mut uv: *mut UpValue = (*thread).open_upvalue;
                while !uv.is_null() {
                    work += 1;
                    if (*uv).get_marked() & (1 << 3 | 1 << 4) == 0 {
                        if ((*(*uv).v.p).is_collectable())
                            && (*(*(*uv).v.p).value.object).get_marked() & (1 << 3 | 1 << 4) != 0
                        {
                            really_mark_object(global, (*(*uv).v.p).value.object);
                        }
                    }
                    uv = (*uv).u.open.next;
                }
            }
        }
        return work;
    }
}
pub unsafe extern "C" fn cleargraylists(global: *mut Global) {
    unsafe {
        (*global).gray_again = std::ptr::null_mut();
        (*global).gray = (*global).gray_again;
        (*global).ephemeron = std::ptr::null_mut();
        (*global).all_weak = (*global).ephemeron;
        (*global).weak = (*global).all_weak;
    }
}
pub unsafe extern "C" fn restartcollection(global: *mut Global) {
    unsafe {
        cleargraylists(global);
        if (*(*global).main_state).get_marked() & (1 << 3 | 1 << 4) != 0 {
            really_mark_object(global, &mut (*((*global).main_state as *mut Object)));
        }
        if ((*global).l_registry.is_collectable())
            && (*(*global).l_registry.value.object).get_marked() & (1 << 3 | 1 << 4) != 0
        {
            really_mark_object(global, (*global).l_registry.value.object);
        }
        (*global).markmt();
        markbeingfnz(global);
    }
}
pub unsafe extern "C" fn propagateall(global: *mut Global) -> u64 {
    unsafe {
        let mut tot: u64 = 0;
        while !((*global).gray).is_null() {
            tot = (tot as u64).wrapping_add((*global).propagatemark()) as u64;
        }
        return tot;
    }
}
pub unsafe extern "C" fn convergeephemerons(global: *mut Global) {
    unsafe {
        let mut changed;
        let mut dir: i32 = 0;
        loop {
            let mut next: *mut Object = (*global).ephemeron;
            (*global).ephemeron = std::ptr::null_mut();
            changed = 0;
            loop {
                let w: *mut Object = next;
                if w.is_null() {
                    break;
                }
                let h: *mut Table = &mut (*(w as *mut Table));
                next = (*h).gc_list;
                (*h).set_marked((*h).get_marked() | 1 << 5);
                if traverseephemeron(global, h, dir) != 0 {
                    propagateall(global);
                    changed = 1;
                }
            }
            dir = (dir == 0) as i32;
            if !(changed != 0) {
                break;
            }
        }
    }
}
pub unsafe extern "C" fn clearbykeys(global: *mut Global, mut l: *mut Object) {
    unsafe {
        while !l.is_null() {
            let h: *mut Table = &mut (*(l as *mut Table));
            let limit: *mut Node = &mut *((*h).node)
                .offset((1 << (*h).log_size_node as i32) as isize)
                as *mut Node;
            let mut node: *mut Node = &mut *((*h).node).offset(0 as isize) as *mut Node;
            while node < limit {
                if iscleared(
                    global,
                    if is_collectable((*node).key.tag) {
                        (*node).key.value.object
                    } else {
                        std::ptr::null_mut()
                    },
                ) != 0
                {
                    (*node).value.set_tag(TAG_VARIANT_NIL_EMPTY);
                }
                if get_tag_type((*node).value.get_tag()) == TAG_TYPE_NIL {
                    (*node).clearkey();
                }
                node = node.offset(1);
            }
            l = (*(l as *mut Table)).gc_list;
        }
    }
}
pub unsafe extern "C" fn clearbyvalues(global: *mut Global, mut l: *mut Object, f: *mut Object) {
    unsafe {
        while l != f {
            let h: *mut Table = &mut (*(l as *mut Table));
            let limit: *mut Node = &mut *((*h).node)
                .offset((1 << (*h).log_size_node as i32) as isize)
                as *mut Node;
            let asize: u32 = luah_realasize(h);
            for i in 0..asize {
                let tvalue: *mut TValue = &mut *((*h).array).offset(i as isize) as *mut TValue;
                if iscleared(
                    global,
                    if (*tvalue).is_collectable() {
                        (*tvalue).value.object
                    } else {
                        std::ptr::null_mut()
                    },
                ) != 0
                {
                    (*tvalue).set_tag(TAG_VARIANT_NIL_EMPTY);
                }
            }
            let mut node: *mut Node = &mut *((*h).node).offset(0 as isize) as *mut Node;
            while node < limit {
                if iscleared(
                    global,
                    if (*node).value.is_collectable() {
                        (*node).value.value.object
                    } else {
                        std::ptr::null_mut()
                    },
                ) != 0
                {
                    (*node).value.set_tag(TAG_VARIANT_NIL_EMPTY);
                }
                if get_tag_type((*node).value.get_tag()) == TAG_TYPE_NIL {
                    (*node).clearkey();
                }
                node = node.offset(1);
            }
            l = (*(l as *mut Table)).gc_list;
        }
    }
}
pub unsafe extern "C" fn udata2finalize(global: *mut Global) -> *mut Object {
    unsafe {
        let object: *mut Object = (*global).to_be_finalized;
        (*global).to_be_finalized = (*object).next;
        (*object).next = (*global).all_gc;
        (*global).all_gc = object;
        (*object).set_marked((*object).get_marked() & !(1 << 6));
        if 3 <= (*global).gc_state as i32 && (*global).gc_state as i32 <= 6 {
            (*object).set_marked(
                (*object).get_marked() & !(1 << 5 | (1 << 3 | 1 << 4))
                    | ((*global).current_white & (1 << 3 | 1 << 4)),
            );
        } else if (*object).get_marked() & 7 == 3 {
            (*global).first_old1 = object;
        }
        return object;
    }
}
pub unsafe extern "C" fn separatetobefnz(global: *mut Global, all: i32) {
    unsafe {
        let mut p: *mut *mut Object = &mut (*global).finalized_objects;
        let mut last_next: *mut *mut Object = find_last(&mut (*global).to_be_finalized);
        loop {
            let current: *mut Object = *p;
            if current == (*global).finobjold1 {
                break;
            }
            if !((*current).get_marked() & (1 << 3 | 1 << 4) != 0 || all != 0) {
                p = &mut (*current).next;
            } else {
                if current == (*global).finobjsur {
                    (*global).finobjsur = (*current).next;
                }
                *p = (*current).next;
                (*current).next = *last_next;
                *last_next = current;
                last_next = &mut (*current).next;
            }
        }
    }
}
pub unsafe extern "C" fn correctpointers(global: *mut Global, object: *mut Object) {
    unsafe {
        check_pointer(&mut (*global).survival, object);
        check_pointer(&mut (*global).old1, object);
        check_pointer(&mut (*global).really_old, object);
        check_pointer(&mut (*global).first_old1, object);
    }
}
pub unsafe extern "C" fn setpause(global: *mut Global) {
    unsafe {
        let pause: i32 = (*global).gc_pause as i32 * 4;
        let estimate: i64 = ((*global).gc_estimate).wrapping_div(100 as u64) as i64;
        let threshold: i64 = if (pause as i64) < (!(0u64) >> 1) as i64 / estimate {
            estimate * pause as i64
        } else {
            (!(0u64) >> 1) as i64
        };
        let mut debt: i64 =
            (((*global).total_bytes + (*global).gc_debt) as u64).wrapping_sub(threshold as u64) as i64;
        if debt > 0 {
            debt = 0;
        }
        (*global).set_debt(debt);
    }
}
pub unsafe extern "C" fn correctgraylists(global: *mut Global) {
    unsafe {
        let mut list: *mut *mut Object = correct_gray_list(&mut (*global).gray_again);
        *list = (*global).weak;
        (*global).weak = std::ptr::null_mut();
        list = correct_gray_list(list);
        *list = (*global).all_weak;
        (*global).all_weak = std::ptr::null_mut();
        list = correct_gray_list(list);
        *list = (*global).ephemeron;
        (*global).ephemeron = std::ptr::null_mut();
        correct_gray_list(list);
    }
}
pub unsafe extern "C" fn markold(global: *mut Global, from: *mut Object, to: *mut Object) {
    unsafe {
        let mut p: *mut Object = from;
        while p != to {
            if (*p).get_marked() & 7 == 3 {
                (*p).set_marked((*p).get_marked() ^ (3 ^ 4));
                if (*p).get_marked() & 1 << 5 != 0 {
                    really_mark_object(global, p);
                }
            }
            p = (*p).next;
        }
    }
}
