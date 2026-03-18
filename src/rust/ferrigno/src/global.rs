use crate::closeprotected::*;
use crate::closure::*;
use crate::functions::*;
use crate::functionstate::*;
use crate::gckind::*;
use crate::node::*;
use crate::object::*;
use crate::objectwithgclist::*;
use crate::prototype::*;
use crate::state::*;
use crate::status::*;
use crate::stringtable::*;
use crate::table::*;
use crate::tagtype::*;
use crate::tagvariant::*;
use crate::tm::*;
use crate::tobject::*;
use crate::tobjectwithgclist::*;
use crate::tstring::*;
use crate::tvalue::*;
use crate::upvalue::*;
use crate::user::*;
use std::ptr::*;
pub const GCP_MINORMUL: usize = 0;
pub const GCP_MAJORMINOR: usize = 1;
pub const GCP_MINORMAJOR: usize = 2;
pub const GCP_PAUSE: usize = 3;
pub const GCP_STEPMUL: usize = 4;
pub const GCP_STEPSIZE: usize = 5;
pub const GCPN: usize = 6;
pub const LUAI_GCPAUSE: u32 = 250;
pub const LUAI_GCMUL: u32 = 200;
pub const LUAI_GCSTEPSIZE: u32 = (200 * size_of::<Table>()) as u32;
pub const LUAI_GENMINORMUL: u32 = 20;
pub const LUAI_MINORMAJOR: u32 = 70;
pub const LUAI_MAJORMINOR: u32 = 50;
pub const GCSWEEPMAX: i32 = 20;
pub const GCS_PROPAGATE: i32 = 0;
pub const GCS_ENTERATOMIC: i32 = 1;
pub const GCS_ATOMIC: i32 = 2;
pub const GCS_SWPALLGC: i32 = 3;
pub const GCS_SWPFINOBJ: i32 = 4;
pub const GCS_SWPTOBEFNZ: i32 = 5;
pub const GCS_SWPEND: i32 = 6;
pub const GCS_CALLFIN: i32 = 7;
pub const GCS_PAUSE: i32 = 8;
pub const GC_STOP: i32 = 0;
pub const GC_RESTART: i32 = 1;
pub const GC_COLLECT: i32 = 2;
pub const GC_COUNT: i32 = 3;
pub const GC_COUNTB: i32 = 4;
pub const GC_STEP: i32 = 5;
pub const GC_SETPAUSE: i32 = 6;
pub const GC_SETSTEPMUL: i32 = 7;
pub const GC_ISRUNNING: i32 = 9;
pub const GC_GENERATIONAL: i32 = 10;
pub const GC_INCREMENTAL: i32 = 11;
pub const GC_PARAM: i32 = 12;
pub const MAX_LMEM: i64 = i64::MAX;
pub const STEP2PAUSE: i64 = -3;
pub const ATOMICSTEP: i64 = -2;
pub const STEP2MINOR: i64 = -1;
const GCPARAM_MANTISSA_MASK: u8 = 0x0F;
const GCPARAM_IMPLICIT_BIT: i64 = 0x10;
const GCPARAM_MAX_MANTISSA: i64 = 0x1F;
const GCPARAM_EXCESS: i32 = 7;
const GCPARAM_MANTISSA_BITS: u64 = 5;
const GCPARAM_MAX_BYTE: u8 = 0xFF;
const GCPARAM_PERCENT_BASE: u64 = 100;
const GCPARAM_SCALE: u64 = 128;
pub fn codeparam(p: u32) -> u8 {
    let limit =
        ((GCPARAM_MAX_MANTISSA as u64) << (GCPARAM_MANTISSA_MASK as u64 - GCPARAM_EXCESS as u64 - 1)) * GCPARAM_PERCENT_BASE;
    if p as u64 >= limit {
        return GCPARAM_MAX_BYTE;
    }
    let p = ((p as u64) * GCPARAM_SCALE).div_ceil(GCPARAM_PERCENT_BASE);
    if p < GCPARAM_IMPLICIT_BIT as u64 {
        p as u8
    } else {
        let log = ceillog2(p + 1) - GCPARAM_MANTISSA_BITS;
        (((p >> log) - GCPARAM_IMPLICIT_BIT as u64) | ((log + 1) << 4)) as u8
    }
}
pub fn applyparam(p: u8, x: i64) -> i64 {
    let mut m = (p & GCPARAM_MANTISSA_MASK) as i64;
    let mut e = (p >> 4) as i32;
    if e > 0 {
        e -= 1;
        m += GCPARAM_IMPLICIT_BIT;
    }
    e -= GCPARAM_EXCESS;
    if e >= 0 {
        if x < (MAX_LMEM / GCPARAM_MAX_MANTISSA) >> e { (x * m) << e } else { MAX_LMEM }
    } else {
        let e = (-e) as u32;
        if x < MAX_LMEM / GCPARAM_MAX_MANTISSA {
            (x * m) >> e
        } else if (x >> e) < MAX_LMEM / GCPARAM_MAX_MANTISSA {
            (x >> e) * m
        } else {
            MAX_LMEM
        }
    }
}
const BITS_PER_BYTE: u64 = 8;
const BYTE_VALUES: usize = 1 << BITS_PER_BYTE;
fn ceillog2(mut x: u64) -> u64 {
    let mut l: u64 = 0;
    x -= 1;
    while x >= BYTE_VALUES as u64 {
        l += BITS_PER_BYTE;
        x >>= BITS_PER_BYTE;
    }
    l + (CEILLOG2_TABLE[x as usize] as u64)
}
static CEILLOG2_TABLE: [u8; BYTE_VALUES] = {
    let mut t = [0u8; BYTE_VALUES];
    let mut i = 1usize;
    while i < BYTE_VALUES {
        t[i] = if i >= 128 {
            8
        } else if i >= 64 {
            7
        } else if i >= 32 {
            6
        } else if i >= 16 {
            5
        } else if i >= 8 {
            4
        } else if i >= 4 {
            3
        } else if i >= 2 {
            2
        } else {
            1
        };
        i += 1;
    }
    t
};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Global {
    pub global_count_total_bytes: i64,
    pub global_count_gc_debt: i64,
    pub global_stringtable: StringTable,
    pub global_lregistry: TValue,
    pub global_nonevalue: TValue,
    pub global_seed: u32,
    pub global_current_white: u8,
    pub global_gcstate: u8,
    pub global_gckind: GCKind,
    pub global_gcstopem: u8,
    pub global_gcparams: [u8; GCPN],
    pub global_gcstep: u8,
    pub global_is_emergency: bool,
    pub global_gcmarked: i64,
    pub global_gcmajorminor: i64,
    pub global_allgc: *mut Object,
    pub global_sweepgc: *mut *mut Object,
    pub global_finalizedobjects: *mut Object,
    pub global_gray: *mut ObjectWithGCList,
    pub global_grayagain: *mut ObjectWithGCList,
    pub global_weak: *mut ObjectWithGCList,
    pub global_ephemeron: *mut ObjectWithGCList,
    pub global_allweak: *mut ObjectWithGCList,
    pub global_tobefinalized: *mut Object,
    pub global_fixedgc: *mut Object,
    pub global_survival: *mut Object,
    pub global_old1: *mut Object,
    pub global_reallyold: *mut Object,
    pub global_firstold1: *mut Object,
    pub global_finobjsur: *mut Object,
    pub global_finobjold1: *mut Object,
    pub global_finobjrold: *mut Object,
    pub global_twups: *mut State,
    pub global_panic: CFunction,
    pub global_maininterpreter: *mut State,
    pub global_memoryerrormessage: *mut TString,
    pub global_tmname: [*mut TString; TM_N as usize],
    pub global_metatables: [*mut Table; LUA_NUM_TYPES],
    pub global_stringcache: [[*mut TString; GLOBAL_STRINGCACHE_M]; GLOBAL_STRINGCACHE_N],
    pub global_warnfunction: WarnFunction,
    pub global_warnuserdata: *mut std::ffi::c_void,
}
impl Global {
    pub unsafe fn allocate(&mut self, state: *mut State, newsize: usize) -> *mut std::ffi::c_void {
        unsafe {
            if newsize == 0 {
                null_mut()
            } else {
                let mut newblock: *mut std::ffi::c_void = raw_allocate(null_mut(), 0, newsize);
                if newblock.is_null() {
                    if self.global_nonevalue.get_tagvariant().to_tag_type().is_nil() && self.global_gcstopem == 0 {
                        self.luac_fullgc(state, true);
                        newblock = raw_allocate(null_mut(), 0, newsize);
                    } else {
                        newblock = null_mut();
                    };
                    if newblock.is_null() {
                        luad_throw(state, Status::MemoryError);
                    }
                }
                self.global_count_gc_debt += newsize as i64;
                newblock
            }
        }
    }
    pub unsafe fn reallocate(
        &mut self, state: *mut State, oldblock: *mut std::ffi::c_void, oldsize: usize, newsize: usize,
    ) -> *mut std::ffi::c_void {
        unsafe {
            let mut newblock = raw_allocate(oldblock, oldsize, newsize);
            if newblock.is_null() && newsize > 0 {
                if self.global_nonevalue.get_tagvariant().to_tag_type().is_nil() && self.global_gcstopem == 0 {
                    self.luac_fullgc(state, true);
                    newblock = raw_allocate(oldblock, oldsize, newsize);
                } else {
                    newblock = null_mut();
                };
                if newblock.is_null() {
                    return null_mut();
                }
            }
            self.global_count_gc_debt += newsize as i64;
            self.global_count_gc_debt -= oldsize as i64;
            newblock
        }
    }
    pub fn should_step(&self) -> bool {
        self.global_count_gc_debt > 0
    }
    pub unsafe fn free_memory(&mut self, block: *mut std::ffi::c_void, oldsize: usize) {
        unsafe {
            raw_allocate(block, oldsize, 0);
            self.global_count_gc_debt -= oldsize as i64;
        }
    }
    pub unsafe fn close(&mut self) {
        unsafe {
            let maininterpreter = self.global_maininterpreter;
            if self.global_nonevalue.get_tagvariant().to_tag_type().is_nil() {
                (*maininterpreter).interpreter_callinfo = &mut (*maininterpreter).interpreter_base_callinfo;
                (*maininterpreter).interpreter_error_function = 0;
                do_close_protected(maininterpreter, 1_i64, Status::OK);
                (*maininterpreter).interpreter_top.stkidrel_pointer =
                    ((*maininterpreter).interpreter_stack.stkidrel_pointer).add(1);
                self.luac_freeallobjects(maininterpreter);
            } else {
                self.luac_freeallobjects(maininterpreter);
            }
            (*maininterpreter).free_memory(
                (*(*maininterpreter).interpreter_global).global_stringtable.stringtable_hash as *mut std::ffi::c_void,
                (*(*maininterpreter).interpreter_global).global_stringtable.stringtable_size * size_of::<*mut TString>(),
            );
            freestack(maininterpreter);
            std::alloc::dealloc(maininterpreter as *mut u8, std::alloc::Layout::new::<State>());
            std::alloc::dealloc(self as *mut Global as *mut u8, std::alloc::Layout::new::<Global>());
        }
    }
    pub fn initialize(&mut self) {
        self.global_current_white = WHITE0BIT;
        self.global_warnfunction = None;
        self.global_warnuserdata = null_mut();
        self.global_gcstep = 2;
        self.global_stringtable.stringtable_length = 0;
        self.global_stringtable.stringtable_size = 0;
        self.global_stringtable.stringtable_hash = null_mut();
        self.global_lregistry.tvalue_set_tag_variant(TagVariant::NilNil);
        self.global_panic = None;
        self.global_gcstate = GCS_PAUSE as u8;
        self.global_gckind = GCKind::Incremental;
        self.global_gcstopem = 0;
        self.global_is_emergency = false;
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
        self.global_count_total_bytes = 0;
        self.global_count_gc_debt = 0;
        self.global_gcmarked = 0;
        self.global_gcmajorminor = 0;
        self.global_gcparams[GCP_PAUSE] = codeparam(LUAI_GCPAUSE);
        self.global_gcparams[GCP_STEPMUL] = codeparam(LUAI_GCMUL);
        self.global_gcparams[GCP_STEPSIZE] = codeparam(LUAI_GCSTEPSIZE);
        self.global_gcparams[GCP_MINORMUL] = codeparam(LUAI_GENMINORMUL);
        self.global_gcparams[GCP_MINORMAJOR] = codeparam(LUAI_MINORMAJOR);
        self.global_gcparams[GCP_MAJORMINOR] = codeparam(LUAI_MAJORMINOR);
        for i in 0..9 {
            self.global_metatables[i as usize] = null_mut();
        }
        let io: *mut TValue = &mut self.global_nonevalue;
        unsafe {
            (*io).set_integer(0);
        }
    }
    pub unsafe fn luac_runtilstate(&mut self, state: *mut State, state2: i32) {
        unsafe {
            while state2 != self.global_gcstate as i32 {
                self.singlestep(state, true);
            }
        }
    }
    pub unsafe fn minor2inc(&mut self, state: *mut State, kind: GCKind) {
        unsafe {
            self.global_gcmajorminor = self.global_gcmarked;
            self.global_gckind = kind;
            self.global_reallyold = null_mut();
            self.global_old1 = null_mut();
            self.global_survival = null_mut();
            self.global_finobjrold = null_mut();
            self.global_finobjold1 = null_mut();
            self.global_finobjsur = null_mut();
            self.entersweep(state);
            self.set_debt(-applyparam(self.global_gcparams[GCP_STEPSIZE], 100));
        }
    }
    pub fn checkminormajor(&self) -> bool {
        let limit = applyparam(self.global_gcparams[GCP_MINORMAJOR], self.global_gcmajorminor);
        if limit == 0 {
            return false;
        }
        self.global_gcmarked >= limit
    }
    pub unsafe fn checkmajorminor(&mut self, state: *mut State) -> bool {
        unsafe {
            if self.global_gckind == GCKind::GenerationalMajor {
                let numbytes = self.global_count_total_bytes + self.global_count_gc_debt;
                let addedbytes = numbytes - self.global_gcmajorminor;
                let limit = applyparam(self.global_gcparams[GCP_MAJORMINOR], addedbytes);
                let tobecollected = numbytes - self.global_gcmarked;
                if tobecollected > limit {
                    self.atomic2gen(state);
                    self.set_minor_debt();
                    return true;
                }
            }
            self.global_gcmajorminor = self.global_gcmarked;
            false
        }
    }
    pub unsafe fn singlestep(&mut self, state: *mut State, fast: bool) -> i64 {
        unsafe {
            let stepresult: i64;
            self.global_gcstopem = 1;
            match self.global_gcstate as i32 {
                | GCS_PAUSE => {
                    self.restartcollection();
                    self.global_gcstate = GCS_PROPAGATE as u8;
                    stepresult = 1;
                },
                | GCS_PROPAGATE => {
                    if fast || (self.global_gray).is_null() {
                        self.global_gcstate = GCS_ENTERATOMIC as u8;
                        stepresult = 1;
                    } else {
                        stepresult = self.propagatemark() as i64;
                    }
                },
                | GCS_ENTERATOMIC => {
                    self.atomic(state);
                    if self.checkmajorminor(state) {
                        stepresult = STEP2MINOR;
                    } else {
                        self.entersweep(state);
                        stepresult = ATOMICSTEP;
                    }
                },
                | GCS_SWPALLGC => {
                    let p = &mut self.global_finalizedobjects as *mut *mut Object;
                    self.sweepstep_unified(state, fast, GCS_SWPFINOBJ, p);
                    stepresult = GCSWEEPMAX as i64;
                },
                | GCS_SWPFINOBJ => {
                    let p = &mut self.global_tobefinalized as *mut *mut Object;
                    self.sweepstep_unified(state, fast, GCS_SWPTOBEFNZ, p);
                    stepresult = GCSWEEPMAX as i64;
                },
                | GCS_SWPTOBEFNZ => {
                    self.sweepstep_unified(state, fast, GCS_SWPEND, null_mut());
                    stepresult = GCSWEEPMAX as i64;
                },
                | GCS_SWPEND => {
                    self.check_sizes(state);
                    self.global_gcstate = GCS_CALLFIN as u8;
                    stepresult = GCSWEEPMAX as i64;
                },
                | GCS_CALLFIN => {
                    if !(self.global_tobefinalized).is_null() && !self.global_is_emergency {
                        self.global_gcstopem = 0;
                        gctm_function(state);
                        stepresult = 10;
                    } else {
                        self.global_gcstate = GCS_PAUSE as u8;
                        stepresult = STEP2PAUSE;
                    }
                },
                | _ => return 0,
            }
            self.global_gcstopem = 0;
            stepresult
        }
    }
    pub unsafe fn check_sizes(&mut self, state: *mut State) {
        unsafe {
            if !self.global_is_emergency
                && self.global_stringtable.stringtable_length < self.global_stringtable.stringtable_size / 4
            {
                luas_resize(state, self.global_stringtable.stringtable_size / 2);
            }
        }
    }
    pub unsafe fn do_gc_step(&mut self, state: *mut State) {
        unsafe {
            if self.global_gcstep != 0 {
                self.set_debt(-20000);
            } else {
                match self.global_gckind {
                    | GCKind::Incremental | GCKind::GenerationalMajor => {
                        self.incstep(state);
                    },
                    | GCKind::GenerationalMinor => {
                        self.youngcollection(state);
                        self.set_minor_debt();
                    },
                }
            }
        }
    }
    pub unsafe fn incstep(&mut self, state: *mut State) {
        unsafe {
            let stepsize: i64 = applyparam(self.global_gcparams[GCP_STEPSIZE], 100);
            let work2do: i64 = applyparam(self.global_gcparams[GCP_STEPMUL], stepsize / size_of::<*mut ()>() as i64);
            let fast = work2do == 0;
            let mut remaining = work2do;
            loop {
                let stres = self.singlestep(state, fast);
                if stres == STEP2MINOR {
                    return;
                } else if stres == STEP2PAUSE || (stres == ATOMICSTEP && !fast) {
                    break;
                } else {
                    remaining -= stres;
                }
                if !(fast || remaining > 0) {
                    break;
                }
            }
            if self.global_gcstate as i32 == GCS_PAUSE {
                self.setpause();
            } else {
                self.set_debt(-stepsize);
            };
        }
    }
    pub unsafe fn atomic(&mut self, state: *mut State) -> usize {
        unsafe {
            let mut work: usize = 0;
            let grayagain = self.global_grayagain;
            self.global_grayagain = null_mut();
            self.global_gcstate = GCS_ATOMIC as u8;
            if (*state).get_marked() & WHITEBITS != 0 {
                Object::really_mark_object(self, state as *mut Object);
            }
            if let Some(obj) = self.global_lregistry.as_object()
                && (*obj).get_marked() & WHITEBITS != 0
            {
                Object::really_mark_object(self, obj);
            }
            self.markmt();
            work = work.wrapping_add(self.propagateall());
            work = work.wrapping_add(self.remarkupvals() as usize);
            work = work.wrapping_add(self.propagateall());
            self.global_gray = grayagain;
            work = work.wrapping_add(self.propagateall());
            self.convergeephemerons();
            clearbyvalues(self, self.global_weak, null_mut());
            clearbyvalues(self, self.global_allweak, null_mut());
            let origweak = self.global_weak;
            let origall = self.global_allweak;
            self.separatetobefnz(false);
            work = work.wrapping_add(self.markbeingfnz());
            work = work.wrapping_add(self.propagateall());
            self.convergeephemerons();
            clearbykeys(self, self.global_ephemeron);
            clearbykeys(self, self.global_allweak);
            clearbyvalues(self, self.global_weak, origweak);
            clearbyvalues(self, self.global_allweak, origall);
            self.stringcache_clear();
            self.global_current_white = (self.global_current_white as i32 ^ WHITEBITS as i32) as u8;
            work
        }
    }
    pub unsafe fn sweepstep_unified(&mut self, state: *mut State, fast: bool, nextstate: i32, nextlist: *mut *mut Object) {
        unsafe {
            if !(self.global_sweepgc).is_null() {
                let mut count: i32 = 0;
                let limit = if fast { i32::MAX } else { GCSWEEPMAX };
                self.global_sweepgc = (*state).sweep_list(self.global_sweepgc, limit, &mut count);
            } else {
                self.global_gcstate = nextstate as u8;
                self.global_sweepgc = nextlist;
            };
        }
    }
    pub unsafe fn entersweep(&mut self, state: *mut State) {
        unsafe {
            self.global_gcstate = GCS_SWPALLGC as u8;
            self.global_sweepgc = sweeptolive(state, &mut self.global_allgc);
        }
    }
    pub unsafe fn luac_fullgc(&mut self, state: *mut State, is_emergency: bool) {
        unsafe {
            self.global_is_emergency = is_emergency;
            match self.global_gckind {
                | GCKind::GenerationalMinor => {
                    self.fullgen(state);
                },
                | GCKind::Incremental => {
                    self.fullinc(state);
                },
                | GCKind::GenerationalMajor => {
                    self.global_gckind = GCKind::Incremental;
                    self.fullinc(state);
                    self.global_gckind = GCKind::GenerationalMajor;
                },
            }
            self.global_is_emergency = false;
            self.setpause();
        }
    }
    pub unsafe fn fullgen(&mut self, state: *mut State) {
        unsafe {
            self.minor2inc(state, GCKind::Incremental);
            self.entergen(state);
        }
    }
    pub unsafe fn fullinc(&mut self, state: *mut State) {
        unsafe {
            if self.global_gcstate <= GCS_ATOMIC as u8 {
                self.entersweep(state);
            }
            self.luac_runtilstate(state, GCS_PAUSE);
            self.luac_runtilstate(state, GCS_CALLFIN);
            self.luac_runtilstate(state, GCS_PAUSE);
            self.setpause();
        }
    }
    pub unsafe fn luac_changemode(&mut self, state: *mut State, new_kind: GCKind) {
        unsafe {
            if self.global_gckind == GCKind::GenerationalMajor {
                self.global_gckind = GCKind::Incremental;
            }
            if new_kind != self.global_gckind {
                if new_kind == GCKind::Incremental {
                    self.minor2inc(state, GCKind::Incremental);
                } else {
                    self.entergen(state);
                }
            }
        }
    }
    pub unsafe fn entergen(&mut self, state: *mut State) {
        unsafe {
            self.luac_runtilstate(state, GCS_PAUSE);
            self.luac_runtilstate(state, GCS_PROPAGATE);
            self.atomic(state);
            self.atomic2gen(state);
            self.set_minor_debt();
        }
    }
    pub unsafe fn atomic2gen(&mut self, state: *mut State) {
        unsafe {
            self.cleargraylists();
            self.global_gcstate = GCS_SWPALLGC as u8;
            sweep2old(state, &mut self.global_allgc);
            self.global_survival = self.global_allgc;
            self.global_old1 = self.global_survival;
            self.global_reallyold = self.global_old1;
            self.global_firstold1 = null_mut();
            sweep2old(state, &mut self.global_finalizedobjects);
            self.global_finobjsur = self.global_finalizedobjects;
            self.global_finobjold1 = self.global_finobjsur;
            self.global_finobjrold = self.global_finobjold1;
            sweep2old(state, &mut self.global_tobefinalized);
            self.global_gckind = GCKind::GenerationalMinor;
            self.global_gcmajorminor = self.global_gcmarked;
            self.global_gcmarked = 0;
            self.finishgencycle(state);
        }
    }
    pub unsafe fn callallpendingfinalizers(&mut self, state: *mut State) {
        unsafe {
            let tobefnz_ptr: *const *mut Object = &raw const self.global_tobefinalized;
            while !std::ptr::read_volatile(tobefnz_ptr).is_null() {
                gctm_function(state);
            }
        }
    }
    pub unsafe fn youngcollection(&mut self, state: *mut State) {
        unsafe {
            let mut addedold1: i64 = 0;
            let marked = self.global_gcmarked;
            if !(self.global_firstold1).is_null() {
                markold(self, self.global_firstold1, self.global_reallyold);
                self.global_firstold1 = null_mut();
            }
            markold(self, self.global_finalizedobjects, self.global_finobjrold);
            markold(self, self.global_tobefinalized, null_mut());
            self.atomic(state);
            self.global_gcstate = GCS_SWPALLGC as u8;
            let mut psurvival: *mut *mut Object = sweepgen(
                state, self, &mut self.global_allgc, self.global_survival, &mut self.global_firstold1, &mut addedold1,
            );
            sweepgen(
                state, self, psurvival, self.global_old1, &mut self.global_firstold1, &mut addedold1,
            );
            self.global_reallyold = self.global_old1;
            self.global_old1 = *psurvival;
            self.global_survival = self.global_allgc;
            let mut dummy: *mut Object = null_mut();
            psurvival = sweepgen(
                state, self, &mut self.global_finalizedobjects, self.global_finobjsur, &mut dummy, &mut addedold1,
            );
            sweepgen(state, self, psurvival, self.global_finobjold1, &mut dummy, &mut addedold1);
            self.global_finobjrold = self.global_finobjold1;
            self.global_finobjold1 = *psurvival;
            self.global_finobjsur = self.global_finalizedobjects;
            sweepgen(
                state,
                self,
                &mut self.global_tobefinalized,
                null_mut(),
                &mut dummy,
                &mut addedold1,
            );
            self.global_gcmarked = marked + addedold1;
            if self.checkminormajor() {
                self.minor2inc(state, GCKind::GenerationalMajor);
                self.global_gcmarked = 0;
            } else {
                self.finishgencycle(state);
            }
        }
    }
    pub unsafe fn luac_freeallobjects(&mut self, state: *mut State) {
        unsafe {
            self.global_gcstep = 4_u8;
            self.luac_changemode(state, GCKind::Incremental);
            self.separatetobefnz(true);
            self.callallpendingfinalizers(state);
            Object::delete_list(state, self.global_allgc, self.global_maininterpreter as *mut Object);
            Object::delete_list(state, self.global_fixedgc, null_mut());
        }
    }
    pub unsafe fn finishgencycle(&mut self, state: *mut State) {
        unsafe {
            self.correctgraylists();
            self.check_sizes(state);
            self.global_gcstate = GCS_PROPAGATE as u8;
            if !self.global_is_emergency {
                self.callallpendingfinalizers(state);
            }
        }
    }
    pub unsafe fn luas_init_global(&mut self, state: *mut State) {
        unsafe {
            let stringtable: *mut StringTable = &mut self.global_stringtable;
            (*stringtable).initialize(state);
            self.global_memoryerrormessage = luas_newlstr(state, c"not enough memory".as_ptr(), "not enough memory".len());
            Object::fix_object_global(self, self.global_memoryerrormessage as *mut Object);
            for i in 0..GLOBAL_STRINGCACHE_N {
                for j in 0..GLOBAL_STRINGCACHE_M {
                    self.global_stringcache[i][j] = self.global_memoryerrormessage;
                }
            }
        }
    }
    pub unsafe fn correct_pointers(&mut self, object: *mut Object) {
        unsafe {
            Object::check_pointer(&mut self.global_survival, object);
            Object::check_pointer(&mut self.global_old1, object);
            Object::check_pointer(&mut self.global_reallyold, object);
            Object::check_pointer(&mut self.global_firstold1, object);
        }
    }
    pub unsafe fn separatetobefnz(&mut self, is_all: bool) {
        unsafe {
            let mut p: *mut *mut Object = &mut self.global_finalizedobjects;
            let mut last_next: *mut *mut Object = Object::find_last(&mut self.global_tobefinalized);
            loop {
                let head: *mut Object = *p;
                if head == self.global_finobjold1 {
                    break;
                }
                if !((*head).get_marked() & WHITEBITS != 0 || is_all) {
                    p = &mut (*head).object_next;
                } else {
                    if head == self.global_finobjsur {
                        self.global_finobjsur = (*head).object_next;
                    }
                    *p = (*head).object_next;
                    (*head).object_next = *last_next;
                    *last_next = head;
                    last_next = &mut (*head).object_next;
                }
            }
        }
    }
    pub unsafe fn setpause(&mut self) {
        unsafe {
            let threshold = applyparam(self.global_gcparams[GCP_PAUSE], self.global_gcmarked);
            let mut debt = threshold - (self.global_count_total_bytes + self.global_count_gc_debt);
            if debt < 0 {
                debt = 0;
            }
            self.set_debt(-debt);
        }
    }
    pub unsafe fn correctgraylists(&mut self) {
        unsafe {
            let mut list = ObjectWithGCList::correct_gray_list(&mut self.global_grayagain);
            *list = self.global_weak;
            self.global_weak = null_mut();
            list = ObjectWithGCList::correct_gray_list(list);
            *list = self.global_allweak;
            self.global_allweak = null_mut();
            list = ObjectWithGCList::correct_gray_list(list);
            *list = self.global_ephemeron;
            self.global_ephemeron = null_mut();
            ObjectWithGCList::correct_gray_list(list);
        }
    }
    pub unsafe fn stringcache_clear(&mut self) {
        unsafe {
            for i in 0..GLOBAL_STRINGCACHE_N {
                for j in 0..GLOBAL_STRINGCACHE_M {
                    if (*self.global_stringcache[i][j]).get_marked() & WHITEBITS != 0 {
                        self.global_stringcache[i][j] = self.global_memoryerrormessage;
                    }
                }
            }
        }
    }
    pub unsafe fn white_list(&mut self, mut p: *mut Object) {
        unsafe {
            let white = self.global_current_white & WHITEBITS;
            while !p.is_null() {
                (*p).set_marked((*p).get_marked() & !(BLACKBIT | WHITEBITS | AGEBITS) | white);
                p = (*p).object_next;
            }
        }
    }
    pub unsafe fn set_debt(&mut self, mut debt: i64) {
        let tb: i64 = self.global_count_total_bytes + self.global_count_gc_debt;
        if debt < tb - MAX_LMEM {
            debt = tb - MAX_LMEM;
        }
        self.global_count_total_bytes = tb - debt;
        self.global_count_gc_debt = debt;
    }
    pub unsafe fn set_minor_debt(&mut self) {
        unsafe {
            self.set_debt(-applyparam(self.global_gcparams[GCP_MINORMUL], self.global_gcmajorminor));
        }
    }
    pub unsafe fn propagatemark(&mut self) -> usize {
        unsafe {
            let object = self.global_gray;
            (*object).set_marked((*object).get_marked() | BLACKBIT);
            self.global_gray = *(*object).getgclist();
            match (*object).get_tagvariant() {
                | TagVariant::Table => traversetable(self, object as *mut Table),
                | TagVariant::User => (*(object as *mut User)).traverseudata(self) as usize,
                | TagVariant::ClosureL => Closure::traverselclosure(self, object as *mut Closure),
                | TagVariant::ClosureC => Closure::traversecclosure(self, object as *mut Closure),
                | TagVariant::Prototype => (&mut *(object as *mut Prototype)).prototype_traverse(self),
                | TagVariant::State => traverse_state(self, object as *mut State) as usize,
                | _ => 0,
            }
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
                if !(self.global_metatables[i as usize]).is_null()
                    && (*self.global_metatables[i as usize]).get_marked() & WHITEBITS != 0
                {
                    Object::really_mark_object(self, *(self.global_metatables).as_mut_ptr().add(i as usize) as *mut Object);
                }
            }
        }
    }
    pub unsafe fn markbeingfnz(&mut self) -> usize {
        unsafe {
            let mut count: usize = 0;
            let mut object: *mut Object = self.global_tobefinalized;
            while !object.is_null() {
                count += 1;
                if (*object).get_marked() & WHITEBITS != 0 {
                    Object::really_mark_object(self, object as *mut Object);
                }
                object = (*object).object_next;
            }
            count
        }
    }
    pub unsafe fn restartcollection(&mut self) {
        unsafe {
            self.global_gcmarked = 0;
            self.cleargraylists();
            if (*self.global_maininterpreter).get_marked() & WHITEBITS != 0 {
                Object::really_mark_object(self, self.global_maininterpreter as *mut Object);
            }
            if let Some(obj) = self.global_lregistry.as_object()
                && (*obj).get_marked() & WHITEBITS != 0
            {
                Object::really_mark_object(self, obj);
            }
            self.markmt();
            self.markbeingfnz();
        }
    }
    pub unsafe fn remarkupvals(&mut self) -> i32 {
        unsafe {
            let mut p: *mut *mut State = &mut self.global_twups;
            let mut work: i32 = 0;
            loop {
                let thread: *mut State = *p;
                if thread.is_null() {
                    break;
                }
                work += 1;
                if (*thread).get_marked() & WHITEBITS == 0 && !((*thread).interpreter_open_upvalue).is_null() {
                    p = &mut (*thread).interpreter_twups;
                } else {
                    *p = (*thread).interpreter_twups;
                    (*thread).interpreter_twups = thread;
                    let mut uv: *mut UpValue = (*thread).interpreter_open_upvalue;
                    while !uv.is_null() {
                        work += 1;
                        if (*uv).get_marked() & WHITEBITS == 0
                            && let Some(obj) = (*(*uv).upvalue_v.upvaluea_p).as_object()
                            && (*obj).get_marked() & WHITEBITS != 0
                        {
                            Object::really_mark_object(self, obj);
                        }
                        uv = (*uv).upvalue_u.upvalueb_open.upvalueba_next;
                    }
                }
            }
            work
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
            total
        }
    }
    pub unsafe fn convergeephemerons(&mut self) {
        unsafe {
            let mut is_reverse = false;
            loop {
                let mut next = self.global_ephemeron;
                self.global_ephemeron = null_mut();
                let mut changed = false;
                loop {
                    let w = next;
                    if w.is_null() {
                        break;
                    } else {
                        let table: *mut Table = w as *mut Table;
                        next = *(*table).getgclist();
                        (*table).set_marked((*table).get_marked() | BLACKBIT);
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
            let object: *mut Object = self.global_tobefinalized;
            self.global_tobefinalized = (*object).object_next;
            (*object).object_next = self.global_allgc;
            self.global_allgc = object;
            (*object).set_marked((*object).get_marked() & !FINALIZEDBIT);
            if GCS_SWPALLGC <= self.global_gcstate as i32 && self.global_gcstate as i32 <= GCS_SWPEND {
                (*object).set_marked((*object).get_marked() & !(BLACKBIT | WHITEBITS) | (self.global_current_white & WHITEBITS));
            } else if (*object).get_marked() & AGEBITS == AGE_OLD1 {
                self.global_firstold1 = object;
            }
            object
        }
    }
}
pub unsafe fn clearbykeys(global: *mut Global, mut l: *mut ObjectWithGCList) {
    unsafe {
        while !l.is_null() {
            let table: *mut Table = l as *mut Table;
            let limit: *mut Node = &mut *((*table).table_node).add(1usize << (*table).table_log_size_node as i32) as *mut Node;
            let mut node: *mut Node = &mut *((*table).table_node).add(0) as *mut Node;
            while node < limit {
                if Object::iscleared(global, (*node).node_key.as_object().unwrap_or(null_mut())) != 0 {
                    (*node).node_value.tvalue_set_tag_variant(TagVariant::NilEmpty);
                }
                if (*node).node_value.get_tagvariant().to_tag_type().is_nil() {
                    (*node).clearkey();
                }
                node = node.add(1);
            }
            l = *(*(l as *mut Table)).getgclist();
        }
    }
}
pub unsafe fn clearbyvalues(global: *mut Global, mut l: *mut ObjectWithGCList, f: *mut ObjectWithGCList) {
    unsafe {
        while l != f {
            let table: *mut Table = l as *mut Table;
            let limit: *mut Node = &mut *((*table).table_node).add(1usize << (*table).table_log_size_node as i32) as *mut Node;
            let asize: u32 = (*table).table_a_size;
            for i in 0..asize {
                let tag = *get_arr_tag(table, i);
                if (tag & 0x0F) >= TagType::String as u8 {
                    let obj = (*get_arr_val(table, i)).value_object;
                    if Object::iscleared(global, obj) != 0 {
                        *get_arr_tag(table, i) = TagVariant::NilEmpty as u8;
                    }
                }
            }
            let mut node: *mut Node = &mut *((*table).table_node).add(0) as *mut Node;
            while node < limit {
                if Object::iscleared(global, (*node).node_value.as_object().unwrap_or(null_mut())) != 0 {
                    (*node).node_value.tvalue_set_tag_variant(TagVariant::NilEmpty);
                }
                if (*node).node_value.get_tagvariant().to_tag_type().is_nil() {
                    (*node).clearkey();
                }
                node = node.add(1);
            }
            l = *(*(l as *mut Table)).getgclist();
        }
    }
}
pub unsafe fn markold(global: *mut Global, from: *mut Object, to: *mut Object) {
    unsafe {
        let mut p = from;
        while p != to {
            if (*p).get_marked() & AGEBITS == AGE_OLD1 {
                (*p).set_marked((*p).get_marked() ^ (AGE_OLD1 ^ AGE_OLD));
                if (*p).get_marked() & BLACKBIT != 0 {
                    Object::really_mark_object(global, p);
                }
            }
            p = (*p).object_next;
        }
    }
}
pub unsafe fn lua_gc(state: *mut State, what: i32, args: &[i32]) -> i32 {
    unsafe {
        let mut res: i32 = 0;
        let global: *mut Global = (*state).interpreter_global;
        if (*global).global_gcstep as i32 & 2 != 0 {
            return -1;
        }
        match what {
            | GC_STOP => {
                (*global).global_gcstep = 1;
            },
            | GC_RESTART => {
                (*global).set_debt(0);
                (*global).global_gcstep = 0;
            },
            | GC_COLLECT => {
                (*global).luac_fullgc(state, false);
            },
            | GC_COUNT => {
                res = (((*global).global_count_total_bytes + (*global).global_count_gc_debt) as usize >> 10_i32) as i32;
            },
            | GC_COUNTB => {
                res = (((*global).global_count_total_bytes + (*global).global_count_gc_debt) as usize & 0x3ff_usize) as i32;
            },
            | GC_STEP => {
                let n: i64 = args.first().copied().unwrap_or(0) as i64;
                let oldstp: u8 = (*global).global_gcstep;
                let mut work: i32 = 0;
                (*global).global_gcstep = 0;
                let n = if n <= 0 { -(*global).global_count_gc_debt } else { n };
                (*global).set_debt((*global).global_count_gc_debt + n);
                if (*global).global_count_gc_debt >= 0 {
                    (*state).do_gc_step();
                    work = 1;
                }
                if work != 0 && (*global).global_gcstate as i32 == GCS_PAUSE {
                    res = 1;
                }
                (*global).global_gcstep = oldstp;
            },
            | GC_ISRUNNING => {
                res = ((*global).global_gcstep as i32 == 0) as i32;
            },
            | GC_GENERATIONAL => {
                res = if (*global).global_gckind == GCKind::Incremental {
                    GC_INCREMENTAL
                } else {
                    GC_GENERATIONAL
                };
                (*global).luac_changemode(state, GCKind::GenerationalMinor);
            },
            | GC_INCREMENTAL => {
                res = if (*global).global_gckind == GCKind::Incremental {
                    GC_INCREMENTAL
                } else {
                    GC_GENERATIONAL
                };
                (*global).luac_changemode(state, GCKind::Incremental);
            },
            | GC_PARAM => {
                let param: i32 = args[0];
                let value: i32 = args[1];
                if param >= 0 && (param as usize) < GCPN {
                    res = applyparam((*global).global_gcparams[param as usize], 100) as i32;
                    if value >= 0 {
                        (*global).global_gcparams[param as usize] = codeparam(value as u32);
                    }
                } else {
                    res = -1;
                }
            },
            | _ => {
                res = -1;
            },
        }
        res
    }
}
