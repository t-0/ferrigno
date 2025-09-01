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
use crate::lclosure::*;
use crate::cclosure::*;
use crate::tvalue::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Global {
    pub totalbytes: i64,
    pub gc_debt: i64,
    pub gc_estimate: u64,
    pub lastatomic: u64,
    pub string_table: StringTable,
    pub l_registry: TValue,
    pub nilvalue: TValue,
    pub seed: u32,
    pub current_white: u8,
    pub gcstate: u8,
    pub gckind: u8,
    pub gcstopem: u8,
    pub genminormul: u64,
    pub genmajormul: u64,
    pub gcstp: u8,
    pub is_emergency: bool,
    pub gcpause: u8,
    pub gcstepmul: u8,
    pub gcstepsize: u8,
    pub allgc: *mut Object,
    pub sweepgc: *mut *mut Object,
    pub finobj: *mut Object,
    pub gray: *mut Object,
    pub grayagain: *mut Object,
    pub weak: *mut Object,
    pub ephemeron: *mut Object,
    pub allweak: *mut Object,
    pub tobefnz: *mut Object,
    pub fixedgc: *mut Object,
    pub survival: *mut Object,
    pub old1: *mut Object,
    pub reallyold: *mut Object,
    pub firstold1: *mut Object,
    pub finobjsur: *mut Object,
    pub finobjold1: *mut Object,
    pub finobjrold: *mut Object,
    pub twups: *mut State,
    pub panic: CFunction,
    pub mainthread: *mut State,
    pub memerrmsg: *mut TString,
    pub tmname: [*mut TString; 25],
    pub mt: [*mut Table; 9],
    pub strcache: [[*mut TString; 2]; 53],
    pub warnf: WarnFunction,
    pub ud_warn: *mut libc::c_void,
}
impl Global {
    pub unsafe extern "C" fn clear_cache(&mut self) {
        unsafe {
            let mut i: i32 = 0;
            while i < 53 as i32 {
                let mut j: i32 = 0;
                while j < 2 {
                    if (*self.strcache[i as usize][j as usize]).get_marked() & (1 << 3 | 1 << 4)
                        != 0
                    {
                        self.strcache[i as usize][j as usize] = self.memerrmsg;
                    }
                    j += 1;
                }
                i += 1;
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
            self.white_list(self.allgc);
            self.survival = std::ptr::null_mut();
            self.old1 = self.survival;
            self.reallyold = self.old1;
            self.white_list(self.finobj);
            self.white_list(self.tobefnz);
            self.finobjsur = std::ptr::null_mut();
            self.finobjold1 = self.finobjsur;
            self.finobjrold = self.finobjold1;
            self.gcstate = 8i32 as u8;
            self.gckind = 0i32 as u8;
            self.lastatomic = 0i32 as u64;
        }
    }
    pub unsafe extern "C" fn set_debt(&mut self, mut debt: i64) {
        let tb: i64 = (self.totalbytes + self.gc_debt) as i64;
        if debt < tb - (!(0i32 as u64) >> 1i32) as i64 {
            debt = tb - (!(0i32 as u64) >> 1i32) as i64;
        }
        self.totalbytes = tb - debt;
        self.gc_debt = debt;
    }
    pub unsafe extern "C" fn set_minor_debt(&mut self) {
        unsafe {
            self.set_debt(
                -((self.totalbytes + self.gc_debt).wrapping_div(100) * self.genminormul as i64),
            );
        }
    }
    pub unsafe extern "C" fn propagatemark(&mut self) -> u64 {
        unsafe {
            let o: *mut Object = self.gray;
            (*o).set_marked((*o).get_marked() | 1 << 5);
            self.gray = *getgclist(o);
            match (*o).get_tag_variant() {
                TAG_VARIANT_TABLE => return traversetable(self, &mut (*(o as *mut Table))),
                TAG_VARIANT_USER => {
                    return traverseudata(self, &mut (*(o as *mut User))) as u64
                }
                TAG_VARIANT_CLOSURE_L => {
                    return traverselclosure(self, &mut (*(o as *mut LClosure)))
                }
                TAG_VARIANT_CLOSURE_C => {
                    return traversecclosure(self, &mut (*(o as *mut CClosure)))
                }
                TAG_VARIANT_PROTOTYPE => {
                    return traverseproto(self, &mut (*(o as *mut Prototype)))
                }
                TAG_VARIANT_STATE => {
                    return traverse_state(self, &mut (*(o as *mut State))) as u64
                }
                _ => return 0,
            };
        }
    }
}
pub unsafe extern "C" fn markmt(g: *mut Global) {
    unsafe {
        let mut i: i32;
        i = 0;
        while i < 9 as i32 {
            if !((*g).mt[i as usize]).is_null() {
                if (*(*g).mt[i as usize]).get_marked() & (1 << 3 | 1 << 4) != 0 {
                    reallymarkobject(
                        g,
                        &mut (*(*((*g).mt).as_mut_ptr().offset(i as isize) as *mut Object)),
                    );
                }
            }
            i += 1;
        }
    }
}
pub unsafe extern "C" fn markbeingfnz(g: *mut Global) -> u64 {
    unsafe {
        let mut count: u64 = 0;
        let mut o: *mut Object = (*g).tobefnz;
        while !o.is_null() {
            count = count.wrapping_add(1);
            if (*o).get_marked() & (1 << 3 | 1 << 4) != 0 {
                reallymarkobject(g, &mut (*(o as *mut Object)));
            }
            o = (*o).next;
        }
        return count;
    }
}
pub unsafe extern "C" fn remarkupvals(g: *mut Global) -> i32 {
    unsafe {
        let mut p: *mut *mut State = &mut (*g).twups;
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
                            reallymarkobject(g, (*(*uv).v.p).value.object);
                        }
                    }
                    uv = (*uv).u.open.next;
                }
            }
        }
        return work;
    }
}
pub unsafe extern "C" fn cleargraylists(g: *mut Global) {
    unsafe {
        (*g).grayagain = std::ptr::null_mut();
        (*g).gray = (*g).grayagain;
        (*g).ephemeron = std::ptr::null_mut();
        (*g).allweak = (*g).ephemeron;
        (*g).weak = (*g).allweak;
    }
}
pub unsafe extern "C" fn restartcollection(g: *mut Global) {
    unsafe {
        cleargraylists(g);
        if (*(*g).mainthread).get_marked() & (1 << 3 | 1 << 4) != 0 {
            reallymarkobject(g, &mut (*((*g).mainthread as *mut Object)));
        }
        if ((*g).l_registry.is_collectable())
            && (*(*g).l_registry.value.object).get_marked() & (1 << 3 | 1 << 4) != 0
        {
            reallymarkobject(g, (*g).l_registry.value.object);
        }
        markmt(g);
        markbeingfnz(g);
    }
}
pub unsafe extern "C" fn propagateall(g: *mut Global) -> u64 {
    unsafe {
        let mut tot: u64 = 0;
        while !((*g).gray).is_null() {
            tot = (tot as u64).wrapping_add((*g).propagatemark()) as u64;
        }
        return tot;
    }
}
pub unsafe extern "C" fn convergeephemerons(g: *mut Global) {
    unsafe {
        let mut changed;
        let mut dir: i32 = 0;
        loop {
            let mut next: *mut Object = (*g).ephemeron;
            (*g).ephemeron = std::ptr::null_mut();
            changed = 0;
            loop {
                let w: *mut Object = next;
                if w.is_null() {
                    break;
                }
                let h: *mut Table = &mut (*(w as *mut Table));
                next = (*h).gc_list;
                (*h).set_marked((*h).get_marked() | 1 << 5);
                if traverseephemeron(g, h, dir) != 0 {
                    propagateall(g);
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
pub unsafe extern "C" fn clearbykeys(g: *mut Global, mut l: *mut Object) {
    unsafe {
        while !l.is_null() {
            let h: *mut Table = &mut (*(l as *mut Table));
            let limit: *mut Node = &mut *((*h).node)
                .offset((1 << (*h).log_size_node as i32) as isize)
                as *mut Node;
            let mut node: *mut Node = &mut *((*h).node).offset(0 as isize) as *mut Node;
            while node < limit {
                if iscleared(
                    g,
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
                    clearkey(node);
                }
                node = node.offset(1);
            }
            l = (*(l as *mut Table)).gc_list;
        }
    }
}
pub unsafe extern "C" fn clearbyvalues(g: *mut Global, mut l: *mut Object, f: *mut Object) {
    unsafe {
        while l != f {
            let h: *mut Table = &mut (*(l as *mut Table));
            let limit: *mut Node = &mut *((*h).node)
                .offset((1 << (*h).log_size_node as i32) as isize)
                as *mut Node;
            let mut i: u32 = 0;
            let asize: u32 = luah_realasize(h);
            while i < asize {
                let o: *mut TValue = &mut *((*h).array).offset(i as isize) as *mut TValue;
                if iscleared(
                    g,
                    if (*o).is_collectable() {
                        (*o).value.object
                    } else {
                        std::ptr::null_mut()
                    },
                ) != 0
                {
                    (*o).set_tag(TAG_VARIANT_NIL_EMPTY);
                }
                i = i.wrapping_add(1);
            }
            let mut node: *mut Node = &mut *((*h).node).offset(0 as isize) as *mut Node;
            while node < limit {
                if iscleared(
                    g,
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
                    clearkey(node);
                }
                node = node.offset(1);
            }
            l = (*(l as *mut Table)).gc_list;
        }
    }
}
pub unsafe extern "C" fn udata2finalize(g: *mut Global) -> *mut Object {
    unsafe {
        let o: *mut Object = (*g).tobefnz;
        (*g).tobefnz = (*o).next;
        (*o).next = (*g).allgc;
        (*g).allgc = o;
        (*o).set_marked((*o).get_marked() & !(1 << 6));
        if 3 <= (*g).gcstate as i32 && (*g).gcstate as i32 <= 6 {
            (*o).set_marked(
                (*o).get_marked() & !(1 << 5 | (1 << 3 | 1 << 4))
                    | ((*g).current_white & (1 << 3 | 1 << 4)),
            );
        } else if (*o).get_marked() & 7 == 3 {
            (*g).firstold1 = o;
        }
        return o;
    }
}
pub unsafe extern "C" fn separatetobefnz(g: *mut Global, all: i32) {
    unsafe {
        let mut p: *mut *mut Object = &mut (*g).finobj;
        let mut lastnext: *mut *mut Object = findlast(&mut (*g).tobefnz);
        loop {
            let curr: *mut Object = *p;
            if !(curr != (*g).finobjold1) {
                break;
            }
            if !((*curr).get_marked() & (1 << 3 | 1 << 4) != 0 || all != 0) {
                p = &mut (*curr).next;
            } else {
                if curr == (*g).finobjsur {
                    (*g).finobjsur = (*curr).next;
                }
                *p = (*curr).next;
                (*curr).next = *lastnext;
                *lastnext = curr;
                lastnext = &mut (*curr).next;
            }
        }
    }
}
pub unsafe extern "C" fn correctpointers(g: *mut Global, o: *mut Object) {
    unsafe {
        checkpointer(&mut (*g).survival, o);
        checkpointer(&mut (*g).old1, o);
        checkpointer(&mut (*g).reallyold, o);
        checkpointer(&mut (*g).firstold1, o);
    }
}
pub unsafe extern "C" fn setpause(g: *mut Global) {
    unsafe {
        let pause: i32 = (*g).gcpause as i32 * 4;
        let estimate: i64 = ((*g).gc_estimate).wrapping_div(100 as u64) as i64;
        let threshold: i64 = if (pause as i64) < (!(0u64) >> 1) as i64 / estimate {
            estimate * pause as i64
        } else {
            (!(0u64) >> 1) as i64
        };
        let mut debt: i64 =
            (((*g).totalbytes + (*g).gc_debt) as u64).wrapping_sub(threshold as u64) as i64;
        if debt > 0 {
            debt = 0;
        }
        (*g).set_debt(debt);
    }
}
pub unsafe extern "C" fn correctgraylists(g: *mut Global) {
    unsafe {
        let mut list: *mut *mut Object = correctgraylist(&mut (*g).grayagain);
        *list = (*g).weak;
        (*g).weak = std::ptr::null_mut();
        list = correctgraylist(list);
        *list = (*g).allweak;
        (*g).allweak = std::ptr::null_mut();
        list = correctgraylist(list);
        *list = (*g).ephemeron;
        (*g).ephemeron = std::ptr::null_mut();
        correctgraylist(list);
    }
}
pub unsafe extern "C" fn markold(g: *mut Global, from: *mut Object, to: *mut Object) {
    unsafe {
        let mut p: *mut Object = from;
        while p != to {
            if (*p).get_marked() & 7 == 3 {
                (*p).set_marked((*p).get_marked() ^ (3 ^ 4));
                if (*p).get_marked() & 1 << 5 != 0 {
                    reallymarkobject(g, p);
                }
            }
            p = (*p).next;
        }
    }
}
