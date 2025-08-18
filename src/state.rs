use crate::c::*;
use crate::callinfo::*;
use crate::debug::*;
use crate::gcobject::*;
use crate::stkidrel::*;
use crate::tstring::*;
use crate::table::*;
use crate::stringtable::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LongJump {
    pub previous: *mut LongJump,
    pub b: [__jmp_buf_tag; 1],
    pub status: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct State {
    pub next: *mut GCObject,
    pub tt: u8,
    pub marked: u8,
    pub status: u8,
    pub allowhook: u8,
    pub nci: u16,
    pub top: StkIdRel,
    pub myglobal: *mut Global,
    pub ci: *mut CallInfo,
    pub stack_last: StkIdRel,
    pub stack: StkIdRel,
    pub openupval: *mut UpVal,
    pub tbclist: StkIdRel,
    pub gclist: *mut GCObject,
    pub twups: *mut State,
    pub error_jump: *mut LongJump,
    pub base_ci: CallInfo,
    pub hook: HookFunction,
    pub errfunc: i64,
    pub count_c_calls: u32,
    pub oldpc: i32,
    pub basehookcount: i32,
    pub hookcount: i32,
    pub hookmask: i32,
}
pub type HookFunction = Option::<unsafe extern "C" fn(*mut State, *mut Debug) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Global {
    pub frealloc: AllocationFunction,
    pub ud: *mut libc::c_void,
    pub totalbytes: i64,
    pub gc_debt: i64,
    pub gc_estimate: u64,
    pub lastatomic: u64,
    pub strt: StringTable,
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
    pub allgc: *mut GCObject,
    pub sweepgc: *mut *mut GCObject,
    pub finobj: *mut GCObject,
    pub gray: *mut GCObject,
    pub grayagain: *mut GCObject,
    pub weak: *mut GCObject,
    pub ephemeron: *mut GCObject,
    pub allweak: *mut GCObject,
    pub tobefnz: *mut GCObject,
    pub fixedgc: *mut GCObject,
    pub survival: *mut GCObject,
    pub old1: *mut GCObject,
    pub reallyold: *mut GCObject,
    pub firstold1: *mut GCObject,
    pub finobjsur: *mut GCObject,
    pub finobjold1: *mut GCObject,
    pub finobjrold: *mut GCObject,
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
pub type WarnFunction = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const i8, i32) -> (),
>;
pub type AllocationFunction = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *mut libc::c_void,
        u64,
        u64,
    ) -> *mut libc::c_void,
>;
