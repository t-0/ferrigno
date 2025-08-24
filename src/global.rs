use crate::functions::*;
use crate::object::*;
use crate::state::*;
use crate::stringtable::*;
use crate::table::*;
use crate::tstring::*;
use crate::tvalue::*;
#[derive(Copy, Clone)]
pub struct Global {
    pub frealloc: AllocationFunction,
    pub ud: *mut libc::c_void,
    pub totalbytes: i64,
    pub gc_debt: i64,
    pub gc_estimate: u64,
    pub lastatomic: u64,
    pub string_table: StringTable,
    pub l_registry: TValue,
    pub nilvalue: TValue,
    pub seed: u32,
    pub currentwhite: u8,
    pub gcstate: u8,
    pub gckind: u8,
    pub gcstopem: u8,
    pub genminormul: u8,
    pub genmajormul: u8,
    pub gcstp: u8,
    pub gcemergency: u8,
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
            let mut i: i32 = 0i32;
            while i < 53 as i32 {
                let mut j: i32 = 0i32;
                while j < 2i32 {
                    if (*self.strcache[i as usize][j as usize]).marked as i32
                        & ((1i32) << 3i32 | (1i32) << 4i32)
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
    pub unsafe extern "C" fn white_list(& mut self, mut p: *mut Object) {
        unsafe {
            let white: i32 = (self.currentwhite as i32 & ((1i32) << 3i32 | (1i32) << 4i32)) as u8 as i32;
            while !p.is_null() {
                (*p).marked = ((*p).marked as i32
                    & !((1i32) << 5i32
                        | ((1i32) << 3i32 | (1i32) << 4i32)
                        | 7i32)
                    | white) as u8;
                p = (*p).next;
            }
        }
    }
    pub unsafe extern "C" fn enter_incremental(& mut self) {
        unsafe {
            self.white_list(self.allgc);
            self.survival = 0 as *mut Object;
            self.old1 = self.survival;
            self.reallyold = self.old1;
            self.white_list(self.finobj);
            self.white_list(self.tobefnz);
            self.finobjsur = 0 as *mut Object;
            self.finobjold1 = self.finobjsur;
            self.finobjrold = self.finobjold1;
            self.gcstate = 8i32 as u8;
            self.gckind = 0i32 as u8;
            self.lastatomic = 0i32 as u64;
        }
    }
    pub unsafe extern "C" fn set_debt(& mut self, mut debt: i64) {
        let tb: i64 = (self.totalbytes + self.gc_debt) as u64 as i64;
        if debt < tb - (!(0i32 as u64) >> 1i32) as i64 {
            debt = tb - (!(0i32 as u64) >> 1i32) as i64;
        }
        self.totalbytes = tb - debt;
        self.gc_debt = debt;
    }
    pub unsafe extern "C" fn set_minor_debt(& mut self) {
        unsafe {
            self.set_debt(
                -(((self.totalbytes + self.gc_debt) as u64).wrapping_div(100 as i32 as u64) as i64
                    * self.genminormul as i64),
            );
        }
    }
}
