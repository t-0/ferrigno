use crate::functions::*;
use crate::object::*;
use crate::tag::*;
use crate::state::*;
use crate::gcunion::*;
use crate::stringtable::*;
use crate::onelua::*;
use crate::table::*;
use crate::tstring::*;
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
            let mut i: i32 = 0;
            while i < 53 as i32 {
                let mut j: i32 = 0;
                while j < 2 {
                    if (*self.strcache[i as usize][j as usize]).get_marked()
                        & (1 << 3 | 1 << 4)
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
                (*p).set_marked((*p).get_marked()
                    & !((1 << 5) | ((1 << 3) | (1 << 4)) | 7) | white);
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
        let tb: i64 = (self.totalbytes + self.gc_debt) as u64 as i64;
        if debt < tb - (!(0i32 as u64) >> 1i32) as i64 {
            debt = tb - (!(0i32 as u64) >> 1i32) as i64;
        }
        self.totalbytes = tb - debt;
        self.gc_debt = debt;
    }
    pub unsafe extern "C" fn set_minor_debt(&mut self) {
        unsafe {
            self.set_debt(-((self.totalbytes + self.gc_debt).wrapping_div(100) * self.genminormul as i64));
        }
    }
    pub unsafe extern "C" fn propagatemark(& mut self) -> u64 { unsafe {
        let o: *mut Object = self.gray;
        (*o).set_marked((*o).get_marked() | 1 << 5);
        self.gray = *getgclist(o);
        match (*o).get_tag_variant() {
            TAG_VARIANT_TABLE => return traversetable(self, &mut (*(o as *mut GCUnion)).h),
            TAG_VARIANT_USER => return traverseudata(self, &mut (*(o as *mut GCUnion)).u) as u64,
            TAG_VARIANT_CLOSURE_L => return traverselclosure(self, &mut (*(o as *mut GCUnion)).lcl) as u64,
            TAG_VARIANT_CLOSURE_C => return traversecclosure(self, &mut (*(o as *mut GCUnion)).ccl) as u64,
            TAG_VARIANT_PROTOTYPE => return traverseproto(self, &mut (*(o as *mut GCUnion)).p) as u64,
            TAG_VARIANT_STATE => return traversethread(self, &mut (*(o as *mut GCUnion)).th) as u64,
            _ => return 0,
        };
    }}
}
