use crate::closure::*;
use crate::functions::*;
use libc::*;
use crate::interpreter::*;
use crate::node::*;
use crate::object::*;
use crate::prototype::*;
use crate::stringtable::*;
use crate::table::*;
use crate::tag::*;
use crate::tstring::*;
use crate::tvalue::*;
use crate::upvalue::*;
use crate::user::*;
use std::ptr::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Global {
    pub total_bytes: i64,
    pub gc_debt: i64,
    pub gc_estimate: usize,
    pub last_atomic: usize,
    pub string_table: StringTable,
    pub l_registry: TValue,
    pub none_value: TValue,
    pub seed: u32,
    pub current_white: u8,
    pub gc_state: u8,
    pub gc_kind: u8,
    pub gcstopem: u8,
    pub generational_minor_multiplier: usize,
    pub generational_major_multiplier: usize,
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
    pub twups: *mut Interpreter,
    pub panic: CFunction,
    pub main_state: *mut Interpreter,
    pub memory_error_message: *mut TString,
    pub tm_name: [*mut TString; 25],
    pub metatables: [*mut Table; 9],
    pub string_cache: [[*mut TString; 2]; 53],
    pub warn_function: WarnFunction,
    pub warn_userdata: *mut c_void,
}
impl Global {
    pub unsafe fn luac_runtilstate(&mut self, interpreter: *mut Interpreter, statesmask: i32) {
        unsafe {
            while statesmask & 1 << self.gc_state as i32 == 0 {
                self.singlestep(interpreter);
            }
        }
    }
    pub unsafe fn singlestep(&mut self, interpreter: *mut Interpreter) -> usize {
        unsafe {
            let work: usize;
            self.gcstopem = 1;
            match self.gc_state as i32 {
                8 => {
                    self.restartcollection();
                    self.gc_state = 0;
                    work = 1 as usize;
                },
                0 => {
                    if (self.gray).is_null() {
                        self.gc_state = 1;
                        work = 0;
                    } else {
                        work = self.propagatemark();
                    }
                },
                1 => {
                    work = self.atomic(interpreter);
                    self.entersweep(interpreter);
                    self.gc_estimate = (self.total_bytes + self.gc_debt) as usize;
                },
                3 => {
                    work = self.sweepstep_finalized(interpreter, 4) as usize;
                },
                4 => {
                    work = self.sweepstep_to_be_finalized(interpreter, 5) as usize;
                },
                5 => {
                    work = self.sweepstep(interpreter, 6) as usize;
                },
                6 => {
                    self.check_sizes(interpreter);
                    self.gc_state = 7 as u8;
                    work = 0;
                },
                7 => {
                    if !(self.to_be_finalized).is_null() && !self.is_emergency {
                        self.gcstopem = 0;
                        work = (runafewfinalizers(interpreter, 10 as i32) * 50 as i32) as usize;
                    } else {
                        self.gc_state = 8 as u8;
                        work = 0;
                    }
                },
                _ => return 0usize,
            }
            self.gcstopem = 0;
            return work;
        }
    }
    pub unsafe fn check_sizes(&mut self, interpreter: *mut Interpreter) {
        unsafe {
            if !self.is_emergency {
                if self.string_table.length < self.string_table.size / 4 {
                    let olddebt: i64 = self.gc_debt;
                    luas_resize(interpreter, (self.string_table.size / 2) as usize);
                    self.gc_estimate = (self.gc_estimate as usize).wrapping_add((self.gc_debt - olddebt) as usize) as usize;
                }
            }
        }
    }
    pub unsafe fn luac_step(&mut self, interpreter: *mut Interpreter) {
        unsafe {
            if self.gc_step as i32 != 0 {
                self.set_debt(-2000);
            } else if self.gc_kind as i32 == 1 || self.last_atomic != 0 {
                self.genstep(interpreter);
            } else {
                self.incstep(interpreter);
            }
        }
    }
    pub unsafe fn incstep(&mut self, interpreter: *mut Interpreter) {
        unsafe {
            let stepmul: i32 = self.gc_step_multiplier as i32 * 4 | 1;
            let mut debt: i64 = (self.gc_debt as usize).wrapping_div(size_of::<TValue>() as usize).wrapping_mul(stepmul as usize) as i64;
            let stepsize: i64 = (if self.gc_step_size as usize <= (size_of::<i64>() as usize).wrapping_mul(8 as usize).wrapping_sub(2 as usize) {
                ((1 << self.gc_step_size as i32) as usize).wrapping_div(size_of::<TValue>() as usize).wrapping_mul(stepmul as usize)
            } else {
                (!(0usize) >> 1) as usize
            }) as i64;
            loop {
                let work: usize = self.singlestep(interpreter);
                debt = (debt as usize).wrapping_sub(work) as i64;
                if !(debt > -stepsize && self.gc_state as i32 != 8) {
                    break;
                }
            }
            if self.gc_state as i32 == 8 {
                self.setpause();
            } else {
                debt = ((debt / stepmul as i64) as usize).wrapping_mul(size_of::<TValue>() as usize) as i64;
                self.set_debt(debt);
            };
        }
    }
    pub unsafe fn atomic(&mut self, interpreter: *mut Interpreter) -> usize {
        unsafe {
            let mut work: usize = 0;
            let grayagain: *mut Object = self.gray_again;
            self.gray_again = null_mut();
            self.gc_state = 2 as u8;
            if (*interpreter).get_marked() & (1 << 3 | 1 << 4) != 0 {
                really_mark_object(self, &mut (*(interpreter as *mut Object)));
            }
            if (self.l_registry.is_collectable()) && (*self.l_registry.value.value_object).get_marked() & (1 << 3 | 1 << 4) != 0 {
                really_mark_object(self, self.l_registry.value.value_object);
            }
            self.markmt();
            work = (work as usize).wrapping_add(self.propagateall()) as usize;
            work = (work as usize).wrapping_add(self.remarkupvals() as usize) as usize;
            work = (work as usize).wrapping_add(self.propagateall()) as usize;
            self.gray = grayagain;
            work = (work as usize).wrapping_add(self.propagateall()) as usize;
            self.convergeephemerons();
            clearbyvalues(self, self.weak, null_mut());
            clearbyvalues(self, self.all_weak, null_mut());
            let origweak: *mut Object = self.weak;
            let origall: *mut Object = self.all_weak;
            self.separatetobefnz(false);
            work = (work as usize).wrapping_add(self.markbeingfnz()) as usize;
            work = (work as usize).wrapping_add(self.propagateall()) as usize;
            self.convergeephemerons();
            clearbykeys(self, self.ephemeron);
            clearbykeys(self, self.all_weak);
            clearbyvalues(self, self.weak, origweak);
            clearbyvalues(self, self.all_weak, origall);
            self.stringcache_clear();
            self.current_white = (self.current_white as i32 ^ (1 << 3 | 1 << 4)) as u8;
            return work;
        }
    }
    pub unsafe fn sweepstep(&mut self, interpreter: *mut Interpreter, nextstate: i32) -> i32 {
        unsafe {
            if !(self.sweep_gc).is_null() {
                let olddebt: i64 = self.gc_debt;
                let mut count: i32 = 0;
                self.sweep_gc = (*interpreter).sweep_list(self.sweep_gc, 100 as i32, &mut count);
                self.gc_estimate = (self.gc_estimate as usize).wrapping_add((self.gc_debt - olddebt) as usize) as usize as usize;
                return count;
            } else {
                self.gc_state = nextstate as u8;
                self.sweep_gc = null_mut();
                return 0;
            };
        }
    }
    pub unsafe fn sweepstep_finalized(&mut self, interpreter: *mut Interpreter, nextstate: i32) -> i32 {
        unsafe {
            if !(self.sweep_gc).is_null() {
                let olddebt: i64 = self.gc_debt;
                let mut count: i32 = 0;
                self.sweep_gc = (*interpreter).sweep_list(self.sweep_gc, 100 as i32, &mut count);
                self.gc_estimate = (self.gc_estimate as usize).wrapping_add((self.gc_debt - olddebt) as usize) as usize as usize;
                return count;
            } else {
                self.gc_state = nextstate as u8;
                self.sweep_gc = &mut self.finalized_objects;
                return 0;
            };
        }
    }
    pub unsafe fn sweepstep_to_be_finalized(&mut self, interpreter: *mut Interpreter, nextstate: i32) -> i32 {
        unsafe {
            if !(self.sweep_gc).is_null() {
                let olddebt: i64 = self.gc_debt;
                let mut count: i32 = 0;
                self.sweep_gc = (*interpreter).sweep_list(self.sweep_gc, 100 as i32, &mut count);
                self.gc_estimate = (self.gc_estimate as usize).wrapping_add((self.gc_debt - olddebt) as usize) as usize as usize;
                return count;
            } else {
                self.gc_state = nextstate as u8;
                self.sweep_gc = &mut self.to_be_finalized;
                return 0;
            };
        }
    }
    pub unsafe fn genstep(&mut self, interpreter: *mut Interpreter) {
        unsafe {
            if self.last_atomic != 0 {
                self.stepgenfull(interpreter);
            } else {
                let majorbase: usize = self.gc_estimate;
                let majorinc: usize = majorbase.wrapping_div(100 as usize).wrapping_mul(self.generational_major_multiplier * 4);
                if self.gc_debt > 0 && (self.total_bytes + self.gc_debt) as usize > majorbase.wrapping_add(majorinc) {
                    let numobjs: usize = self.fullgen(interpreter);
                    if !(((self.total_bytes + self.gc_debt) as usize) < majorbase.wrapping_add(majorinc.wrapping_div(2 as usize))) {
                        self.last_atomic = numobjs;
                        self.setpause();
                    }
                } else {
                    self.youngcollection(interpreter);
                    self.set_minor_debt();
                    self.gc_estimate = majorbase;
                }
            };
        }
    }
    pub unsafe fn entersweep(&mut self, interpreter: *mut Interpreter) {
        unsafe {
            self.gc_state = 3 as u8;
            self.sweep_gc = sweeptolive(interpreter, &mut self.all_gc);
        }
    }
    pub unsafe fn luac_fullgc(&mut self, interpreter: *mut Interpreter, is_emergency: bool) {
        unsafe {
            self.is_emergency = is_emergency;
            if self.gc_kind as i32 == 0 {
                if self.gc_state as i32 <= 2 {
                    self.entersweep(interpreter);
                }
                self.luac_runtilstate(interpreter, 1 << 8);
                self.luac_runtilstate(interpreter, 1 << 0);
                self.gc_state = 1;
                self.luac_runtilstate(interpreter, 1 << 7);
                self.luac_runtilstate(interpreter, 1 << 8);
                self.setpause();
            } else {
                self.fullgen(interpreter);
            }
            self.is_emergency = false;
        }
    }
    pub unsafe fn fullgen(&mut self, interpreter: *mut Interpreter) -> usize {
        unsafe {
            self.enter_incremental();
            return self.entergen(interpreter);
        }
    }
    pub unsafe fn luac_changemode(&mut self, interpreter: *mut Interpreter, new_mode: i32) {
        unsafe {
            if new_mode != self.gc_kind as i32 {
                if new_mode == 1 {
                    self.entergen(interpreter);
                } else {
                    self.enter_incremental();
                }
            }
            self.last_atomic = 0;
        }
    }
    pub unsafe fn stepgenfull(&mut self, interpreter: *mut Interpreter) {
        unsafe {
            let lastatomic: usize = self.last_atomic;
            if self.gc_kind as i32 == 1 {
                self.enter_incremental();
            }
            self.luac_runtilstate(interpreter, 1 << 0);
            let newatomic: usize = self.atomic(interpreter);
            if newatomic < lastatomic.wrapping_add(lastatomic >> 3) {
                self.atomic2gen(interpreter);
                self.set_minor_debt();
            } else {
                self.gc_estimate = (self.total_bytes + self.gc_debt) as usize;
                self.entersweep(interpreter);
                self.luac_runtilstate(interpreter, 1 << 8);
                self.setpause();
                self.last_atomic = newatomic;
            };
        }
    }
    pub unsafe fn entergen(&mut self, interpreter: *mut Interpreter) -> usize {
        unsafe {
            self.luac_runtilstate(interpreter, 1 << 8);
            self.luac_runtilstate(interpreter, 1 << 0);
            let numobjs: usize = self.atomic(interpreter);
            self.atomic2gen(interpreter);
            self.set_minor_debt();
            return numobjs;
        }
    }
    pub unsafe fn atomic2gen(&mut self, interpreter: *mut Interpreter) {
        unsafe {
            self.cleargraylists();
            self.gc_state = 3 as u8;
            sweep2old(interpreter, &mut self.all_gc);
            self.survival = self.all_gc;
            self.old1 = self.survival;
            self.really_old = self.old1;
            self.first_old1 = null_mut();
            sweep2old(interpreter, &mut self.finalized_objects);
            self.finobjsur = self.finalized_objects;
            self.finobjold1 = self.finobjsur;
            self.finobjrold = self.finobjold1;
            sweep2old(interpreter, &mut self.to_be_finalized);
            self.gc_kind = 1;
            self.last_atomic = 0;
            self.gc_estimate = (self.total_bytes + self.gc_debt) as usize;
            self.finishgencycle(interpreter);
        }
    }
    pub unsafe fn callallpendingfinalizers(&mut self, interpreter: *mut Interpreter) {
        unsafe {
            while !(self.to_be_finalized).is_null() {
                gctm_function(interpreter);
            }
        }
    }
    pub unsafe fn youngcollection(&mut self, interpreter: *mut Interpreter) {
        unsafe {
            if !(self.first_old1).is_null() {
                markold(self, self.first_old1, self.really_old);
                self.first_old1 = null_mut();
            }
            markold(self, self.finalized_objects, self.finobjrold);
            markold(self, self.to_be_finalized, null_mut());
            self.atomic(interpreter);
            self.gc_state = 3 as u8;
            let mut psurvival: *mut *mut Object = sweepgen(interpreter, self, &mut self.all_gc, self.survival, &mut self.first_old1);
            sweepgen(interpreter, self, psurvival, self.old1, &mut self.first_old1);
            self.really_old = self.old1;
            self.old1 = *psurvival;
            self.survival = self.all_gc;
            let mut dummy: *mut Object = null_mut();
            psurvival = sweepgen(interpreter, self, &mut self.finalized_objects, self.finobjsur, &mut dummy);
            sweepgen(interpreter, self, psurvival, self.finobjold1, &mut dummy);
            self.finobjrold = self.finobjold1;
            self.finobjold1 = *psurvival;
            self.finobjsur = self.finalized_objects;
            sweepgen(interpreter, self, &mut self.to_be_finalized, null_mut(), &mut dummy);
            self.finishgencycle(interpreter);
        }
    }
    pub unsafe fn luac_freeallobjects(&mut self, interpreter: *mut Interpreter) {
        unsafe {
            self.gc_step = 4 as u8;
            self.luac_changemode(interpreter, 0);
            self.separatetobefnz(true);
            self.callallpendingfinalizers(interpreter);
            delete_list(interpreter, self.all_gc, &mut (*(self.main_state as *mut Object)));
            delete_list(interpreter, self.fixed_gc, null_mut());
        }
    }
    pub unsafe fn finishgencycle(&mut self, interpreter: *mut Interpreter) {
        unsafe {
            self.correctgraylists();
            self.check_sizes(interpreter);
            self.gc_state = 0;
            if !self.is_emergency {
                self.callallpendingfinalizers(interpreter);
            }
        }
    }
    pub unsafe fn luas_init_global(&mut self, interpreter: *mut Interpreter) {
        unsafe {
            let string_table: *mut StringTable = &mut self.string_table;
            (*string_table).initialize(interpreter);
            self.memory_error_message = luas_newlstr(interpreter, c"not enough memory".as_ptr(), (size_of::<[i8; 18]>()).wrapping_div(size_of::<i8>()).wrapping_sub(1));
            fix_object_global(self, self.memory_error_message as *mut Object);
            for i in 0..GLOBAL_STRINGCACHE_N {
                for j in 0..GLOBAL_STRINGCACHE_M {
                    self.string_cache[i][j] = self.memory_error_message;
                }
            }
        }
    }
    pub unsafe fn correct_pointers(&mut self, object: *mut Object) {
        unsafe {
            check_pointer(&mut self.survival, object);
            check_pointer(&mut self.old1, object);
            check_pointer(&mut self.really_old, object);
            check_pointer(&mut self.first_old1, object);
        }
    }
    pub unsafe fn separatetobefnz(&mut self, is_all: bool) {
        unsafe {
            let mut p: *mut *mut Object = &mut (*self).finalized_objects;
            let mut last_next: *mut *mut Object = find_last(&mut (*self).to_be_finalized);
            loop {
                let current: *mut Object = *p;
                if current == (*self).finobjold1 {
                    break;
                }
                if !((*current).get_marked() & (1 << 3 | 1 << 4) != 0 || is_all) {
                    p = &mut (*current).next;
                } else {
                    if current == (*self).finobjsur {
                        (*self).finobjsur = (*current).next;
                    }
                    *p = (*current).next;
                    (*current).next = *last_next;
                    *last_next = current;
                    last_next = &mut (*current).next;
                }
            }
        }
    }
    pub unsafe fn setpause(&mut self) {
        unsafe {
            let pause: i32 = (*self).gc_pause as i32 * 4;
            let estimate: i64 = ((*self).gc_estimate).wrapping_div(100 as usize) as i64;
            let threshold: i64 = if (pause as i64) < (!(0usize) >> 1) as i64 / estimate { estimate * pause as i64 } else { (!(0usize) >> 1) as i64 };
            let mut debt: i64 = (((*self).total_bytes + (*self).gc_debt) as usize).wrapping_sub(threshold as usize) as i64;
            if debt > 0 {
                debt = 0;
            }
            (*self).set_debt(debt);
        }
    }
    pub unsafe fn correctgraylists(&mut self) {
        unsafe {
            let mut list: *mut *mut Object = correct_gray_list(&mut (*self).gray_again);
            *list = (*self).weak;
            (*self).weak = null_mut();
            list = correct_gray_list(list);
            *list = (*self).all_weak;
            (*self).all_weak = null_mut();
            list = correct_gray_list(list);
            *list = (*self).ephemeron;
            (*self).ephemeron = null_mut();
            correct_gray_list(list);
        }
    }
    pub unsafe fn stringcache_clear(&mut self) {
        unsafe {
            for i in 0..GLOBAL_STRINGCACHE_N {
                for j in 0..GLOBAL_STRINGCACHE_M {
                    if (*self.string_cache[i as usize][j as usize]).get_marked() & (1 << 3 | 1 << 4) != 0 {
                        self.string_cache[i as usize][j as usize] = self.memory_error_message;
                    }
                }
            }
        }
    }
    pub unsafe fn fix_memory_error_message_global(&mut self) {
        unsafe {
            fix_object_global(self, self.memory_error_message as *mut Object);
        }
    }
    pub unsafe fn white_list(&mut self, mut p: *mut Object) {
        unsafe {
            let white = self.current_white & ((1 << 3) | (1 << 4));
            while !p.is_null() {
                (*p).set_marked((*p).get_marked() & !((1 << 5) | ((1 << 3) | (1 << 4)) | 7) | white);
                p = (*p).next;
            }
        }
    }
    pub unsafe fn enter_incremental(&mut self) {
        unsafe {
            self.white_list(self.all_gc);
            self.survival = null_mut();
            self.old1 = self.survival;
            self.really_old = self.old1;
            self.white_list(self.finalized_objects);
            self.white_list(self.to_be_finalized);
            self.finobjsur = null_mut();
            self.finobjold1 = self.finobjsur;
            self.finobjrold = self.finobjold1;
            self.gc_state = 8i32 as u8;
            self.gc_kind = 0i32 as u8;
            self.last_atomic = 0i32 as usize;
        }
    }
    pub unsafe fn set_debt(&mut self, mut debt: i64) {
        let tb: i64 = (self.total_bytes + self.gc_debt) as i64;
        if debt < tb - (!(0i32 as usize) >> 1i32) as i64 {
            debt = tb - (!(0i32 as usize) >> 1i32) as i64;
        }
        self.total_bytes = tb - debt;
        self.gc_debt = debt;
    }
    pub unsafe fn set_minor_debt(&mut self) {
        unsafe {
            self.set_debt(-((self.total_bytes + self.gc_debt).wrapping_div(100) * self.generational_minor_multiplier as i64));
        }
    }
    pub unsafe fn propagatemark(&mut self) -> usize {
        unsafe {
            let object: *mut Object = self.gray;
            (*object).set_marked((*object).get_marked() | 1 << 5);
            self.gray = *getgclist(object);
            match (*object).get_tag_variant() {
                TAG_VARIANT_TABLE => return traversetable(self, &mut (*(object as *mut Table))),
                TAG_VARIANT_USER => return (*(object as *mut User)).traverseudata(self) as usize,
                TAG_VARIANT_CLOSURE_L => return Closure::traverselclosure(self, &mut (*(object as *mut Closure))),
                TAG_VARIANT_CLOSURE_C => return Closure::traversecclosure(self, &mut (*(object as *mut Closure))),
                TAG_VARIANT_PROTOTYPE => return (&mut (*(object as *mut Prototype))).prototype_traverse(self),
                TAG_VARIANT_STATE => return traverse_state(self, &mut (*(object as *mut Interpreter))) as usize,
                _ => return 0,
            };
        }
    }
    pub unsafe fn markmt(&mut self) {
        unsafe {
            for i in TAGTYPE_SIMPLE_ {
                if !(self.metatables[i as usize]).is_null() {
                    if (*self.metatables[i as usize]).get_marked() & (1 << 3 | 1 << 4) != 0 {
                        really_mark_object(self, &mut (*(*(self.metatables).as_mut_ptr().offset(i as isize) as *mut Object)));
                    }
                }
            }
        }
    }
    pub unsafe fn markbeingfnz(&mut self) -> usize {
        unsafe {
            let mut count: usize = 0;
            let mut object: *mut Object = self.to_be_finalized;
            while !object.is_null() {
                count = count.wrapping_add(1);
                if (*object).get_marked() & (1 << 3 | 1 << 4) != 0 {
                    really_mark_object(self, &mut (*(object as *mut Object)));
                }
                object = (*object).next;
            }
            return count;
        }
    }
    pub unsafe fn restartcollection(&mut self) {
        unsafe {
            self.cleargraylists();
            if (*self.main_state).get_marked() & (1 << 3 | 1 << 4) != 0 {
                really_mark_object(self, &mut (*(self.main_state as *mut Object)));
            }
            if (self.l_registry.is_collectable()) && (*self.l_registry.value.value_object).get_marked() & (1 << 3 | 1 << 4) != 0 {
                really_mark_object(self, self.l_registry.value.value_object);
            }
            self.markmt();
            self.markbeingfnz();
        }
    }
    pub unsafe fn remarkupvals(&mut self) -> i32 {
        unsafe {
            let mut p: *mut *mut Interpreter = &mut self.twups;
            let mut work: i32 = 0;
            loop {
                let thread: *mut Interpreter = *p;
                if thread.is_null() {
                    break;
                }
                work += 1;
                if (*thread).get_marked() & (1 << 3 | 1 << 4) == 0 && !((*thread).open_upvalue).is_null() {
                    p = &mut (*thread).twups;
                } else {
                    *p = (*thread).twups;
                    (*thread).twups = thread;
                    let mut uv: *mut UpValue = (*thread).open_upvalue;
                    while !uv.is_null() {
                        work += 1;
                        if (*uv).get_marked() & (1 << 3 | 1 << 4) == 0 {
                            if ((*(*uv).v.p).is_collectable()) && (*(*(*uv).v.p).value.value_object).get_marked() & (1 << 3 | 1 << 4) != 0 {
                                really_mark_object(self, (*(*uv).v.p).value.value_object);
                            }
                        }
                        uv = (*uv).u.open.next;
                    }
                }
            }
            return work;
        }
    }
    pub fn cleargraylists(&mut self) {
        self.gray_again = null_mut();
        self.gray = null_mut();
        self.ephemeron = null_mut();
        self.all_weak = null_mut();
        self.weak = null_mut();
    }
    pub unsafe fn propagateall(&mut self) -> usize {
        unsafe {
            let mut total: usize = 0;
            while !self.gray.is_null() {
                total += self.propagatemark();
            }
            return total;
        }
    }
    pub unsafe fn convergeephemerons(&mut self) {
        unsafe {
            let mut is_reverse = false;
            loop {
                let mut next: *mut Object = (*self).ephemeron;
                (*self).ephemeron = null_mut();
                let mut changed = false;
                loop {
                    let w: *mut Object = next;
                    if w.is_null() {
                        break;
                    } else {
                        let table: *mut Table = &mut (*(w as *mut Table));
                        next = (*table).gc_list;
                        (*table).set_marked((*table).get_marked() | 1 << 5);
                        if traverseephemeron(self, table, is_reverse) != 0 {
                            (*self).propagateall();
                            changed = true;
                        }
                    }
                }
                if !changed {
                    break;
                } else {
                    is_reverse = !is_reverse;
                }
            }
        }
    }
    pub unsafe fn udata2finalize(&mut self) -> *mut Object {
        unsafe {
            let object: *mut Object = (*self).to_be_finalized;
            (*self).to_be_finalized = (*object).next;
            (*object).next = (*self).all_gc;
            (*self).all_gc = object;
            (*object).set_marked((*object).get_marked() & !(1 << 6));
            if 3 <= (*self).gc_state as i32 && (*self).gc_state as i32 <= 6 {
                (*object).set_marked((*object).get_marked() & !(1 << 5 | (1 << 3 | 1 << 4)) | ((*self).current_white & (1 << 3 | 1 << 4)));
            } else if (*object).get_marked() & 7 == 3 {
                (*self).first_old1 = object;
            }
            return object;
        }
    }
}
pub unsafe fn clearbykeys(global: *mut Global, mut l: *mut Object) {
    unsafe {
        while !l.is_null() {
            let table: *mut Table = &mut (*(l as *mut Table));
            let limit: *mut Node = &mut *((*table).node).offset((1 << (*table).log_size_node as i32) as isize) as *mut Node;
            let mut node: *mut Node = &mut *((*table).node).offset(0 as isize) as *mut Node;
            while node < limit {
                if iscleared(global, if (*node).key.is_collectable() { (*node).key.value.value_object } else { null_mut() }) != 0 {
                    (*node).value.set_tag_variant(TAG_VARIANT_NIL_EMPTY);
                }
                if (*node).value.is_tagtype_nil() {
                    (*node).clearkey();
                }
                node = node.offset(1);
            }
            l = (*(l as *mut Table)).gc_list;
        }
    }
}
pub unsafe fn clearbyvalues(global: *mut Global, mut l: *mut Object, f: *mut Object) {
    unsafe {
        while l != f {
            let table: *mut Table = &mut (*(l as *mut Table));
            let limit: *mut Node = &mut *((*table).node).offset((1 << (*table).log_size_node as i32) as isize) as *mut Node;
            let asize: u32 = luah_realasize(table);
            for i in 0..asize {
                let tvalue: *mut TValue = &mut *((*table).array).offset(i as isize) as *mut TValue;
                if iscleared(global, if (*tvalue).is_collectable() { (*tvalue).value.value_object } else { null_mut() }) != 0 {
                    (*tvalue).set_tag_variant(TAG_VARIANT_NIL_EMPTY);
                }
            }
            let mut node: *mut Node = &mut *((*table).node).offset(0 as isize) as *mut Node;
            while node < limit {
                if iscleared(global, if (*node).value.is_collectable() { (*node).value.value.value_object } else { null_mut() }) != 0 {
                    (*node).value.set_tag_variant(TAG_VARIANT_NIL_EMPTY);
                }
                if (*node).value.is_tagtype_nil() {
                    (*node).clearkey();
                }
                node = node.offset(1);
            }
            l = (*(l as *mut Table)).gc_list;
        }
    }
}
pub unsafe fn markold(global: *mut Global, from: *mut Object, to: *mut Object) {
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
pub unsafe extern "C" fn lua_gc(interpreter: *mut Interpreter, what: i32, args: ...) -> i32 {
    unsafe {
        let mut argp: ::core::ffi::VaListImpl;
        let mut res: i32 = 0;
        let global: *mut Global = (*interpreter).global;
        if (*global).gc_step as i32 & 2 != 0 {
            return -1;
        }
        argp = args.clone();
        match what {
            0 => {
                (*global).gc_step = 1;
            },
            1 => {
                (*global).set_debt(0);
                (*global).gc_step = 0;
            },
            2 => {
                (*global).luac_fullgc(interpreter, false);
            },
            3 => {
                res = (((*global).total_bytes + (*global).gc_debt) as usize >> 10 as i32) as i32;
            },
            4 => {
                res = (((*global).total_bytes + (*global).gc_debt) as usize & 0x3ff as usize) as i32;
            },
            5 => {
                let data: i32 = argp.arg::<i32>();
                let mut debt: i64 = 1;
                let oldstp: u8 = (*global).gc_step;
                (*global).gc_step = 0;
                if data == 0 {
                    (*global).set_debt(0);
                    (*interpreter).luac_step();
                } else {
                    debt = data as i64 * 1024 as i64 + (*global).gc_debt;
                    (*global).set_debt(debt);
                    if (*(*interpreter).global).gc_debt > 0 {
                        (*interpreter).luac_step();
                    }
                }
                (*global).gc_step = oldstp;
                if debt > 0 && (*global).gc_state as i32 == 8 {
                    res = 1;
                }
            },
            6 => {
                let data_0: i32 = argp.arg::<i32>();
                res = (*global).gc_pause as i32 * 4;
                (*global).gc_pause = (data_0 / 4) as u8;
            },
            7 => {
                let data_1: i32 = argp.arg::<i32>();
                res = (*global).gc_step_multiplier as i32 * 4;
                (*global).gc_step_multiplier = (data_1 / 4) as u8;
            },
            9 => {
                res = ((*global).gc_step as i32 == 0) as i32;
            },
            10 => {
                let minormul: i32 = argp.arg::<i32>();
                let majormul: i32 = argp.arg::<i32>();
                res = if (*global).gc_kind as i32 == 1 || (*global).last_atomic != 0 { 10 as i32 } else { 11 as i32 };
                if minormul != 0 {
                    (*global).generational_minor_multiplier = minormul as usize;
                }
                if majormul != 0 {
                    (*global).generational_major_multiplier = (majormul / 4) as usize;
                }
                (*global).luac_changemode(interpreter, 1);
            },
            11 => {
                let pause: i32 = argp.arg::<i32>();
                let stepmul: i32 = argp.arg::<i32>();
                let stepsize: i32 = argp.arg::<i32>();
                res = if (*global).gc_kind as i32 == 1 || (*global).last_atomic != 0 { 10 as i32 } else { 11 as i32 };
                if pause != 0 {
                    (*global).gc_pause = (pause / 4) as u8;
                }
                if stepmul != 0 {
                    (*global).gc_step_multiplier = (stepmul / 4) as u8;
                }
                if stepsize != 0 {
                    (*global).gc_step_size = stepsize as u8;
                }
                (*global).luac_changemode(interpreter, 0);
            },
            _ => {
                res = -1;
            },
        }
        return res;
    }
}
