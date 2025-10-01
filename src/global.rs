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
    pub global_totalbytes: i64,
    pub global_gcdebt: i64,
    pub global_gcestimate: usize,
    pub global_lastatomic: usize,
    pub global_stringtable: StringTable,
    pub global_lregistry: TValue,
    pub global_nonevalue: TValue,
    pub global_seed: u32,
    pub global_currentwhite: u8,
    pub global_gcstate: u8,
    pub global_gckind: u8,
    pub global_gcstopem: u8,
    pub global_generationalminormultiplier: usize,
    pub global_generationalmajormultiplier: usize,
    pub global_gcstep: u8,
    pub global_isemergency: bool,
    pub global_gcpause: u8,
    pub global_gcstepmultiplier: u8,
    pub global_gcstepsize: u8,
    pub global_allgc: *mut Object,
    pub global_sweepgc: *mut *mut Object,
    pub global_finalizedobjects: *mut Object,
    pub global_gray: *mut Object,
    pub global_grayagain: *mut Object,
    pub global_weak: *mut Object,
    pub global_ephemeron: *mut Object,
    pub global_allweak: *mut Object,
    pub global_tobefinalized: *mut Object,
    pub global_fixedgc: *mut Object,
    pub global_survival: *mut Object,
    pub global_old1: *mut Object,
    pub global_reallyold: *mut Object,
    pub global_firstold1: *mut Object,
    pub global_finobjsur: *mut Object,
    pub global_finobjold1: *mut Object,
    pub global_finobjrold: *mut Object,
    pub global_twups: *mut Interpreter,
    pub global_panic: CFunction,
    pub global_mainstate: *mut Interpreter,
    pub global_memoryerrormessage: *mut TString,
    pub global_tmname: [*mut TString; 25],
    pub global_metatables: [*mut Table; 9],
    pub global_stringcache: [[*mut TString; 2]; 53],
    pub global_warnfunction: WarnFunction,
    pub global_warnuserdata: *mut c_void,
}
impl Global {
    pub fn init (&mut self) {
        self.global_currentwhite = (1 << 3) as u8;
        self.global_warnfunction = None;
        self.global_warnuserdata = null_mut();
        self.global_gcstep = 2;
        self.global_stringtable.stringtable_length = 0;
        self.global_stringtable.stringtable_size = 0;
        self.global_stringtable.stringtable_hash = null_mut();
        self.global_lregistry.set_tag_variant(TagVariant::NilNil);
        self.global_panic = None;
        self.global_gcstate = 8;
        self.global_gckind = 0;
        self.global_gcstopem = 0;
        self.global_isemergency = false;
        self.global_fixedgc = null_mut();
        self.global_tobefinalized = self.global_fixedgc;
        self.global_finalizedobjects = self.global_tobefinalized;
        self.global_reallyold = null_mut();
        self.global_old1 = null_mut();
        self.global_survival = null_mut();
        self.global_firstold1 = null_mut();
        self.global_finobjrold = null_mut();
        self.global_finobjold1 = null_mut();
        self.global_finobjsur = null_mut();
        self.global_sweepgc = null_mut();
        self.global_grayagain = null_mut();
        self.global_gray = null_mut();
        self.global_allweak = null_mut();
        self.global_ephemeron = null_mut();
        self.global_weak = null_mut();
        self.global_twups = null_mut();
        self.global_totalbytes = 0;
        self.global_gcdebt = 0;
        self.global_lastatomic = 0;
        self.global_gcpause = 200 / 4;
        self.global_gcstepmultiplier = 100 / 4;
        self.global_gcstepsize = 13;
        self.global_generationalmajormultiplier = 100 / 4;
        self.global_generationalminormultiplier = 20;
        for i in 0..9 {
            self.global_metatables[i as usize] = null_mut();
        }
        let io: *mut TValue = &mut self.global_nonevalue;
        unsafe {
            (*io).value.value_integer = 0;
            (*io).set_tag_variant(TagVariant::NumericInteger);
        }
    }
    pub unsafe fn luac_runtilstate(&mut self, interpreter: *mut Interpreter, statesmask: i32) {
        unsafe {
            while statesmask & 1 << self.global_gcstate as i32 == 0 {
                self.singlestep(interpreter);
            }
        }
    }
    pub unsafe fn singlestep(&mut self, interpreter: *mut Interpreter) -> usize {
        unsafe {
            let work: usize;
            self.global_gcstopem = 1;
            match self.global_gcstate as i32 {
                8 => {
                    self.restartcollection();
                    self.global_gcstate = 0;
                    work = 1 as usize;
                },
                0 => {
                    if (self.global_gray).is_null() {
                        self.global_gcstate = 1;
                        work = 0;
                    } else {
                        work = self.propagatemark();
                    }
                },
                1 => {
                    work = self.atomic(interpreter);
                    self.entersweep(interpreter);
                    self.global_gcestimate = (self.global_totalbytes + self.global_gcdebt) as usize;
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
                    self.global_gcstate = 7;
                    work = 0;
                },
                7 => {
                    if !(self.global_tobefinalized).is_null() && !self.global_isemergency {
                        self.global_gcstopem = 0;
                        work = (runafewfinalizers(interpreter, 10 as i32) * 50) as usize;
                    } else {
                        self.global_gcstate = 8;
                        work = 0;
                    }
                },
                _ => return 0,
            }
            self.global_gcstopem = 0;
            return work;
        }
    }
    pub unsafe fn check_sizes(&mut self, interpreter: *mut Interpreter) {
        unsafe {
            if !self.global_isemergency {
                if self.global_stringtable.stringtable_length < self.global_stringtable.stringtable_size / 4 {
                    let olddebt: i64 = self.global_gcdebt;
                    luas_resize(interpreter, self.global_stringtable.stringtable_size / 2);
                    self.global_gcestimate = (self.global_gcestimate as usize).wrapping_add((self.global_gcdebt - olddebt) as usize) as usize;
                }
            }
        }
    }
    pub unsafe fn luac_step(&mut self, interpreter: *mut Interpreter) {
        unsafe {
            if self.global_gcstep as i32 != 0 {
                self.set_debt(-2000);
            } else if self.global_gckind as i32 == 1 || self.global_lastatomic != 0 {
                self.genstep(interpreter);
            } else {
                self.incstep(interpreter);
            }
        }
    }
    pub unsafe fn incstep(&mut self, interpreter: *mut Interpreter) {
        unsafe {
            let stepmul: i32 = self.global_gcstepmultiplier as i32 * 4 | 1;
            let mut debt: i64 = (((self.global_gcdebt as usize) / size_of::<TValue>()) * (stepmul as usize)) as i64;
            let stepsize: i64 = (if self.global_gcstepsize as usize <= size_of::<i64>() * 8 - 2 {
                ((1 << self.global_gcstepsize as i32) as usize) / (size_of::<TValue>()) * (stepmul as usize)
            } else {
                (!(0usize) >> 1) as usize
            }) as i64;
            loop {
                let work: usize = self.singlestep(interpreter);
                debt = debt - (work as i64);
                if !(debt > -stepsize && self.global_gcstate as i32 != 8) {
                    break;
                }
            }
            if self.global_gcstate as i32 == 8 {
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
            let grayagain: *mut Object = self.global_grayagain;
            self.global_grayagain = null_mut();
            self.global_gcstate = 2 as u8;
            if (*interpreter).get_marked() & (1 << 3 | 1 << 4) != 0 {
                really_mark_object(self, &mut (*(interpreter as *mut Object)));
            }
            if (self.global_lregistry.is_collectable()) && (*self.global_lregistry.value.value_object).get_marked() & (1 << 3 | 1 << 4) != 0 {
                really_mark_object(self, self.global_lregistry.value.value_object);
            }
            self.markmt();
            work = (work as usize).wrapping_add(self.propagateall()) as usize;
            work = (work as usize).wrapping_add(self.remarkupvals() as usize) as usize;
            work = (work as usize).wrapping_add(self.propagateall()) as usize;
            self.global_gray = grayagain;
            work = (work as usize).wrapping_add(self.propagateall()) as usize;
            self.convergeephemerons();
            clearbyvalues(self, self.global_weak, null_mut());
            clearbyvalues(self, self.global_allweak, null_mut());
            let origweak: *mut Object = self.global_weak;
            let origall: *mut Object = self.global_allweak;
            self.separatetobefnz(false);
            work = (work as usize).wrapping_add(self.markbeingfnz()) as usize;
            work = (work as usize).wrapping_add(self.propagateall()) as usize;
            self.convergeephemerons();
            clearbykeys(self, self.global_ephemeron);
            clearbykeys(self, self.global_allweak);
            clearbyvalues(self, self.global_weak, origweak);
            clearbyvalues(self, self.global_allweak, origall);
            self.stringcache_clear();
            self.global_currentwhite = (self.global_currentwhite as i32 ^ (1 << 3 | 1 << 4)) as u8;
            return work;
        }
    }
    pub unsafe fn sweepstep(&mut self, interpreter: *mut Interpreter, nextstate: i32) -> i32 {
        unsafe {
            if !(self.global_sweepgc).is_null() {
                let olddebt: i64 = self.global_gcdebt;
                let mut count: i32 = 0;
                self.global_sweepgc = (*interpreter).sweep_list(self.global_sweepgc, 100 as i32, &mut count);
                self.global_gcestimate = (self.global_gcestimate as usize).wrapping_add((self.global_gcdebt - olddebt) as usize) as usize as usize;
                return count;
            } else {
                self.global_gcstate = nextstate as u8;
                self.global_sweepgc = null_mut();
                return 0;
            };
        }
    }
    pub unsafe fn sweepstep_finalized(&mut self, interpreter: *mut Interpreter, nextstate: i32) -> i32 {
        unsafe {
            if !(self.global_sweepgc).is_null() {
                let olddebt: i64 = self.global_gcdebt;
                let mut count: i32 = 0;
                self.global_sweepgc = (*interpreter).sweep_list(self.global_sweepgc, 100 as i32, &mut count);
                self.global_gcestimate = (self.global_gcestimate as usize).wrapping_add((self.global_gcdebt - olddebt) as usize) as usize as usize;
                return count;
            } else {
                self.global_gcstate = nextstate as u8;
                self.global_sweepgc = &mut self.global_finalizedobjects;
                return 0;
            };
        }
    }
    pub unsafe fn sweepstep_to_be_finalized(&mut self, interpreter: *mut Interpreter, nextstate: i32) -> i32 {
        unsafe {
            if !(self.global_sweepgc).is_null() {
                let olddebt: i64 = self.global_gcdebt;
                let mut count: i32 = 0;
                self.global_sweepgc = (*interpreter).sweep_list(self.global_sweepgc, 100 as i32, &mut count);
                self.global_gcestimate = (self.global_gcestimate as usize).wrapping_add((self.global_gcdebt - olddebt) as usize) as usize as usize;
                return count;
            } else {
                self.global_gcstate = nextstate as u8;
                self.global_sweepgc = &mut self.global_tobefinalized;
                return 0;
            };
        }
    }
    pub unsafe fn genstep(&mut self, interpreter: *mut Interpreter) {
        unsafe {
            if self.global_lastatomic != 0 {
                self.stepgenfull(interpreter);
            } else {
                let majorbase: usize = self.global_gcestimate;
                let majorinc: usize = (majorbase / 100) * (self.global_generationalmajormultiplier * 4);
                if self.global_gcdebt > 0 && (self.global_totalbytes + self.global_gcdebt) as usize > majorbase.wrapping_add(majorinc) {
                    let numobjs: usize = self.fullgen(interpreter);
                    if !(((self.global_totalbytes + self.global_gcdebt) as usize) < majorbase + (majorinc / 2)) {
                        self.global_lastatomic = numobjs;
                        self.setpause();
                    }
                } else {
                    self.youngcollection(interpreter);
                    self.set_minor_debt();
                    self.global_gcestimate = majorbase;
                }
            };
        }
    }
    pub unsafe fn entersweep(&mut self, interpreter: *mut Interpreter) {
        unsafe {
            self.global_gcstate = 3 as u8;
            self.global_sweepgc = sweeptolive(interpreter, &mut self.global_allgc);
        }
    }
    pub unsafe fn luac_fullgc(&mut self, interpreter: *mut Interpreter, is_emergency: bool) {
        unsafe {
            self.global_isemergency = is_emergency;
            if self.global_gckind as i32 == 0 {
                if self.global_gcstate as i32 <= 2 {
                    self.entersweep(interpreter);
                }
                self.luac_runtilstate(interpreter, 1 << 8);
                self.luac_runtilstate(interpreter, 1 << 0);
                self.global_gcstate = 1;
                self.luac_runtilstate(interpreter, 1 << 7);
                self.luac_runtilstate(interpreter, 1 << 8);
                self.setpause();
            } else {
                self.fullgen(interpreter);
            }
            self.global_isemergency = false;
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
            if new_mode != self.global_gckind as i32 {
                if new_mode == 1 {
                    self.entergen(interpreter);
                } else {
                    self.enter_incremental();
                }
            }
            self.global_lastatomic = 0;
        }
    }
    pub unsafe fn stepgenfull(&mut self, interpreter: *mut Interpreter) {
        unsafe {
            let lastatomic: usize = self.global_lastatomic;
            if self.global_gckind as i32 == 1 {
                self.enter_incremental();
            }
            self.luac_runtilstate(interpreter, 1 << 0);
            let newatomic: usize = self.atomic(interpreter);
            if newatomic < lastatomic.wrapping_add(lastatomic >> 3) {
                self.atomic2gen(interpreter);
                self.set_minor_debt();
            } else {
                self.global_gcestimate = (self.global_totalbytes + self.global_gcdebt) as usize;
                self.entersweep(interpreter);
                self.luac_runtilstate(interpreter, 1 << 8);
                self.setpause();
                self.global_lastatomic = newatomic;
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
            self.global_gcstate = 3 as u8;
            sweep2old(interpreter, &mut self.global_allgc);
            self.global_survival = self.global_allgc;
            self.global_old1 = self.global_survival;
            self.global_reallyold = self.global_old1;
            self.global_firstold1 = null_mut();
            sweep2old(interpreter, &mut self.global_finalizedobjects);
            self.global_finobjsur = self.global_finalizedobjects;
            self.global_finobjold1 = self.global_finobjsur;
            self.global_finobjrold = self.global_finobjold1;
            sweep2old(interpreter, &mut self.global_tobefinalized);
            self.global_gckind = 1;
            self.global_lastatomic = 0;
            self.global_gcestimate = (self.global_totalbytes + self.global_gcdebt) as usize;
            self.finishgencycle(interpreter);
        }
    }
    pub unsafe fn callallpendingfinalizers(&mut self, interpreter: *mut Interpreter) {
        unsafe {
            while !(self.global_tobefinalized).is_null() {
                gctm_function(interpreter);
            }
        }
    }
    pub unsafe fn youngcollection(&mut self, interpreter: *mut Interpreter) {
        unsafe {
            if !(self.global_firstold1).is_null() {
                markold(self, self.global_firstold1, self.global_reallyold);
                self.global_firstold1 = null_mut();
            }
            markold(self, self.global_finalizedobjects, self.global_finobjrold);
            markold(self, self.global_tobefinalized, null_mut());
            self.atomic(interpreter);
            self.global_gcstate = 3 as u8;
            let mut psurvival: *mut *mut Object = sweepgen(interpreter, self, &mut self.global_allgc, self.global_survival, &mut self.global_firstold1);
            sweepgen(interpreter, self, psurvival, self.global_old1, &mut self.global_firstold1);
            self.global_reallyold = self.global_old1;
            self.global_old1 = *psurvival;
            self.global_survival = self.global_allgc;
            let mut dummy: *mut Object = null_mut();
            psurvival = sweepgen(interpreter, self, &mut self.global_finalizedobjects, self.global_finobjsur, &mut dummy);
            sweepgen(interpreter, self, psurvival, self.global_finobjold1, &mut dummy);
            self.global_finobjrold = self.global_finobjold1;
            self.global_finobjold1 = *psurvival;
            self.global_finobjsur = self.global_finalizedobjects;
            sweepgen(interpreter, self, &mut self.global_tobefinalized, null_mut(), &mut dummy);
            self.finishgencycle(interpreter);
        }
    }
    pub unsafe fn luac_freeallobjects(&mut self, interpreter: *mut Interpreter) {
        unsafe {
            self.global_gcstep = 4 as u8;
            self.luac_changemode(interpreter, 0);
            self.separatetobefnz(true);
            self.callallpendingfinalizers(interpreter);
            delete_list(interpreter, self.global_allgc, &mut (*(self.global_mainstate as *mut Object)));
            delete_list(interpreter, self.global_fixedgc, null_mut());
        }
    }
    pub unsafe fn finishgencycle(&mut self, interpreter: *mut Interpreter) {
        unsafe {
            self.correctgraylists();
            self.check_sizes(interpreter);
            self.global_gcstate = 0;
            if !self.global_isemergency {
                self.callallpendingfinalizers(interpreter);
            }
        }
    }
    pub unsafe fn luas_init_global(&mut self, interpreter: *mut Interpreter) {
        unsafe {
            let stringtable: *mut StringTable = &mut self.global_stringtable;
            (*stringtable).initialize(interpreter);
            self.global_memoryerrormessage = luas_newlstr(interpreter, c"not enough memory".as_ptr(), "not enough memory".len());
            fix_object_global(self, self.global_memoryerrormessage as *mut Object);
            for i in 0..GLOBAL_STRINGCACHE_N {
                for j in 0..GLOBAL_STRINGCACHE_M {
                    self.global_stringcache[i][j] = self.global_memoryerrormessage;
                }
            }
        }
    }
    pub unsafe fn correct_pointers(&mut self, object: *mut Object) {
        unsafe {
            check_pointer(&mut self.global_survival, object);
            check_pointer(&mut self.global_old1, object);
            check_pointer(&mut self.global_reallyold, object);
            check_pointer(&mut self.global_firstold1, object);
        }
    }
    pub unsafe fn separatetobefnz(&mut self, is_all: bool) {
        unsafe {
            let mut p: *mut *mut Object = &mut (*self).global_finalizedobjects;
            let mut last_next: *mut *mut Object = find_last(&mut (*self).global_tobefinalized);
            loop {
                let current: *mut Object = *p;
                if current == (*self).global_finobjold1 {
                    break;
                }
                if !((*current).get_marked() & (1 << 3 | 1 << 4) != 0 || is_all) {
                    p = &mut (*current).next;
                } else {
                    if current == (*self).global_finobjsur {
                        (*self).global_finobjsur = (*current).next;
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
            let pause: i32 = (*self).global_gcpause as i32 * 4;
            let estimate: i64 = ((*self).global_gcestimate / 100) as i64;
            let threshold: i64 = if (pause as i64) < (!(0usize) >> 1) as i64 / estimate { estimate * pause as i64 } else { (!(0usize) >> 1) as i64 };
            let mut debt: i64 = (*self).global_totalbytes + (*self).global_gcdebt - threshold;
            if debt > 0 {
                debt = 0;
            }
            (*self).set_debt(debt);
        }
    }
    pub unsafe fn correctgraylists(&mut self) {
        unsafe {
            let mut list: *mut *mut Object = correct_gray_list(&mut (*self).global_grayagain);
            *list = (*self).global_weak;
            (*self).global_weak = null_mut();
            list = correct_gray_list(list);
            *list = (*self).global_allweak;
            (*self).global_allweak = null_mut();
            list = correct_gray_list(list);
            *list = (*self).global_ephemeron;
            (*self).global_ephemeron = null_mut();
            correct_gray_list(list);
        }
    }
    pub unsafe fn stringcache_clear(&mut self) {
        unsafe {
            for i in 0..GLOBAL_STRINGCACHE_N {
                for j in 0..GLOBAL_STRINGCACHE_M {
                    if (*self.global_stringcache[i as usize][j as usize]).get_marked() & (1 << 3 | 1 << 4) != 0 {
                        self.global_stringcache[i as usize][j as usize] = self.global_memoryerrormessage;
                    }
                }
            }
        }
    }
    pub unsafe fn fix_memory_error_message_global(&mut self) {
        unsafe {
            fix_object_global(self, self.global_memoryerrormessage as *mut Object);
        }
    }
    pub unsafe fn white_list(&mut self, mut p: *mut Object) {
        unsafe {
            let white = self.global_currentwhite & ((1 << 3) | (1 << 4));
            while !p.is_null() {
                (*p).set_marked((*p).get_marked() & !((1 << 5) | ((1 << 3) | (1 << 4)) | 7) | white);
                p = (*p).next;
            }
        }
    }
    pub unsafe fn enter_incremental(&mut self) {
        unsafe {
            self.white_list(self.global_allgc);
            self.global_survival = null_mut();
            self.global_old1 = self.global_survival;
            self.global_reallyold = self.global_old1;
            self.white_list(self.global_finalizedobjects);
            self.white_list(self.global_tobefinalized);
            self.global_finobjsur = null_mut();
            self.global_finobjold1 = self.global_finobjsur;
            self.global_finobjrold = self.global_finobjold1;
            self.global_gcstate = 8i32 as u8;
            self.global_gckind = 0i32 as u8;
            self.global_lastatomic = 0i32 as usize;
        }
    }
    pub unsafe fn set_debt(&mut self, mut debt: i64) {
        let tb: i64 = (self.global_totalbytes + self.global_gcdebt) as i64;
        if debt < tb - (!(0i32 as usize) >> 1i32) as i64 {
            debt = tb - (!(0i32 as usize) >> 1i32) as i64;
        }
        self.global_totalbytes = tb - debt;
        self.global_gcdebt = debt;
    }
    pub unsafe fn set_minor_debt(&mut self) {
        unsafe {
            self.set_debt(-(((self.global_totalbytes + self.global_gcdebt) / 100) * self.global_generationalminormultiplier as i64));
        }
    }
    pub unsafe fn propagatemark(&mut self) -> usize {
        unsafe {
            let object: *mut Object = self.global_gray;
            (*object).set_marked((*object).get_marked() | 1 << 5);
            self.global_gray = *(*object).getgclist();
            match (*object).get_tag_variant() {
                TagVariant::Table => return traversetable(self, &mut (*(object as *mut Table))),
                TagVariant::User => return (*(object as *mut User)).traverseudata(self) as usize,
                TagVariant::ClosureL => return Closure::traverselclosure(self, &mut (*(object as *mut Closure))),
                TagVariant::ClosureC => return Closure::traversecclosure(self, &mut (*(object as *mut Closure))),
                TagVariant::Prototype => return (&mut (*(object as *mut Prototype))).prototype_traverse(self),
                TagVariant::State => return traverse_state(self, &mut (*(object as *mut Interpreter))) as usize,
                _ => return 0,
            };
        }
    }
    pub unsafe fn markmt(&mut self) {
        unsafe {
            const TAGTYPE_SIMPLE: [TagType; 9] = [
                TagType::Nil,
                TagType::Boolean,
                TagType::Pointer,
                TagType::Numeric,
                TagType::String,
                TagType::Table,
                TagType::Closure,
                TagType::User,
                TagType::State,
            ];
            for i in TAGTYPE_SIMPLE {
                if !(self.global_metatables[i as usize]).is_null() {
                    if (*self.global_metatables[i as usize]).get_marked() & (1 << 3 | 1 << 4) != 0 {
                        really_mark_object(self, &mut (*(*(self.global_metatables).as_mut_ptr().offset(i as isize) as *mut Object)));
                    }
                }
            }
        }
    }
    pub unsafe fn markbeingfnz(&mut self) -> usize {
        unsafe {
            let mut count: usize = 0;
            let mut object: *mut Object = self.global_tobefinalized;
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
            if (*self.global_mainstate).get_marked() & (1 << 3 | 1 << 4) != 0 {
                really_mark_object(self, &mut (*(self.global_mainstate as *mut Object)));
            }
            if (self.global_lregistry.is_collectable()) && (*self.global_lregistry.value.value_object).get_marked() & (1 << 3 | 1 << 4) != 0 {
                really_mark_object(self, self.global_lregistry.value.value_object);
            }
            self.markmt();
            self.markbeingfnz();
        }
    }
    pub unsafe fn remarkupvals(&mut self) -> i32 {
        unsafe {
            let mut p: *mut *mut Interpreter = &mut self.global_twups;
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
        self.global_grayagain = null_mut();
        self.global_gray = null_mut();
        self.global_ephemeron = null_mut();
        self.global_allweak = null_mut();
        self.global_weak = null_mut();
    }
    pub unsafe fn propagateall(&mut self) -> usize {
        unsafe {
            let mut total: usize = 0;
            while !self.global_gray.is_null() {
                total += self.propagatemark();
            }
            return total;
        }
    }
    pub unsafe fn convergeephemerons(&mut self) {
        unsafe {
            let mut is_reverse = false;
            loop {
                let mut next: *mut Object = (*self).global_ephemeron;
                (*self).global_ephemeron = null_mut();
                let mut changed = false;
                loop {
                    let w: *mut Object = next;
                    if w.is_null() {
                        break;
                    } else {
                        let table: *mut Table = &mut (*(w as *mut Table));
                        next = *(*table).getgclist();
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
            let object: *mut Object = (*self).global_tobefinalized;
            (*self).global_tobefinalized = (*object).next;
            (*object).next = (*self).global_allgc;
            (*self).global_allgc = object;
            (*object).set_marked((*object).get_marked() & !(1 << 6));
            if 3 <= (*self).global_gcstate as i32 && (*self).global_gcstate as i32 <= 6 {
                (*object).set_marked((*object).get_marked() & !(1 << 5 | (1 << 3 | 1 << 4)) | ((*self).global_currentwhite & (1 << 3 | 1 << 4)));
            } else if (*object).get_marked() & 7 == 3 {
                (*self).global_firstold1 = object;
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
                    (*node).value.set_tag_variant(TagVariant::NilEmpty);
                }
                if (*node).value.is_tagtype_nil() {
                    (*node).clearkey();
                }
                node = node.offset(1);
            }
            l = *(*(l as *mut Table)).getgclist();
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
                    (*tvalue).set_tag_variant(TagVariant::NilEmpty);
                }
            }
            let mut node: *mut Node = &mut *((*table).node).offset(0 as isize) as *mut Node;
            while node < limit {
                if iscleared(global, if (*node).value.is_collectable() { (*node).value.value.value_object } else { null_mut() }) != 0 {
                    (*node).value.set_tag_variant(TagVariant::NilEmpty);
                }
                if (*node).value.is_tagtype_nil() {
                    (*node).clearkey();
                }
                node = node.offset(1);
            }
            l = *(*(l as *mut Table)).getgclist();
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
        if (*global).global_gcstep as i32 & 2 != 0 {
            return -1;
        }
        argp = args.clone();
        match what {
            0 => {
                (*global).global_gcstep = 1;
            },
            1 => {
                (*global).set_debt(0);
                (*global).global_gcstep = 0;
            },
            2 => {
                (*global).luac_fullgc(interpreter, false);
            },
            3 => {
                res = (((*global).global_totalbytes + (*global).global_gcdebt) as usize >> 10 as i32) as i32;
            },
            4 => {
                res = (((*global).global_totalbytes + (*global).global_gcdebt) as usize & 0x3ff as usize) as i32;
            },
            5 => {
                let data: i32 = argp.arg::<i32>();
                let mut debt: i64 = 1;
                let oldstp: u8 = (*global).global_gcstep;
                (*global).global_gcstep = 0;
                if data == 0 {
                    (*global).set_debt(0);
                    (*interpreter).luac_step();
                } else {
                    debt = data as i64 * 1024 as i64 + (*global).global_gcdebt;
                    (*global).set_debt(debt);
                    if (*(*interpreter).global).global_gcdebt > 0 {
                        (*interpreter).luac_step();
                    }
                }
                (*global).global_gcstep = oldstp;
                if debt > 0 && (*global).global_gcstate as i32 == 8 {
                    res = 1;
                }
            },
            6 => {
                let data_0: i32 = argp.arg::<i32>();
                res = (*global).global_gcpause as i32 * 4;
                (*global).global_gcpause = (data_0 / 4) as u8;
            },
            7 => {
                let data_1: i32 = argp.arg::<i32>();
                res = (*global).global_gcstepmultiplier as i32 * 4;
                (*global).global_gcstepmultiplier = (data_1 / 4) as u8;
            },
            9 => {
                res = ((*global).global_gcstep as i32 == 0) as i32;
            },
            10 => {
                let minormul: i32 = argp.arg::<i32>();
                let majormul: i32 = argp.arg::<i32>();
                res = if (*global).global_gckind as i32 == 1 || (*global).global_lastatomic != 0 { 10 as i32 } else { 11 as i32 };
                if minormul != 0 {
                    (*global).global_generationalminormultiplier = minormul as usize;
                }
                if majormul != 0 {
                    (*global).global_generationalmajormultiplier = (majormul / 4) as usize;
                }
                (*global).luac_changemode(interpreter, 1);
            },
            11 => {
                let pause: i32 = argp.arg::<i32>();
                let stepmul: i32 = argp.arg::<i32>();
                let stepsize: i32 = argp.arg::<i32>();
                res = if (*global).global_gckind as i32 == 1 || (*global).global_lastatomic != 0 { 10 as i32 } else { 11 as i32 };
                if pause != 0 {
                    (*global).global_gcpause = (pause / 4) as u8;
                }
                if stepmul != 0 {
                    (*global).global_gcstepmultiplier = (stepmul / 4) as u8;
                }
                if stepsize != 0 {
                    (*global).global_gcstepsize = stepsize as u8;
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
