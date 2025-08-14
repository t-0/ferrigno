#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use crate::types::*;
unsafe extern "C" {
    fn luaT_gettmbyobj(L: *mut lua_State, o: *const TValue, event: TMS) -> *const TValue;
    fn luaM_free_(L: *mut lua_State, block: *mut libc::c_void, osize: u64);
    fn luaG_findlocal(
        L: *mut lua_State,
        ci: *mut CallInfo,
        n: i32,
        pos: *mut StkId,
    ) -> *const libc::c_char;
    fn luaG_runerror(L: *mut lua_State, fmt: *const libc::c_char, _: ...) -> !;
    fn luaD_seterrorobj(L: *mut lua_State, errcode: i32, oldtop: StkId);
    fn luaD_call(L: *mut lua_State, func: StkId, nResults: i32);
    fn luaD_callnoyield(L: *mut lua_State, func: StkId, nResults: i32);
    fn luaC_newobj(L: *mut lua_State, tt: i32, sz: u64) -> *mut GCObject;
    fn luaC_barrier_(L: *mut lua_State, o: *mut GCObject, v: *mut GCObject);
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct lua_State {
    pub next: *mut GCObject,
    pub tt: u8,
    pub marked: u8,
    pub status: u8,
    pub allowhook: u8,
    pub nci: libc::c_ushort,
    pub top: StkIdRel,
    pub l_G: *mut global_State,
    pub ci: *mut CallInfo,
    pub stack_last: StkIdRel,
    pub stack: StkIdRel,
    pub openupval: *mut UpVal,
    pub tbclist: StkIdRel,
    pub gclist: *mut GCObject,
    pub twups: *mut lua_State,
    pub errorJmp: *mut LongJump,
    pub base_ci: CallInfo,
    pub hook: lua_Hook,
    pub errfunc: i64,
    pub nCcalls: u32,
    pub oldpc: i32,
    pub basehookcount: i32,
    pub hookcount: i32,
    pub hookmask: sig_atomic_t,
}
pub type sig_atomic_t = i32;

pub type lua_Hook = Option<unsafe extern "C" fn(*mut lua_State, *mut lua_Debug) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lua_Debug {
    pub event: i32,
    pub name: *const libc::c_char,
    pub namewhat: *const libc::c_char,
    pub what: *const libc::c_char,
    pub source: *const libc::c_char,
    pub srclen: u64,
    pub currentline: i32,
    pub linedefined: i32,
    pub lastlinedefined: i32,
    pub nups: u8,
    pub nparams: u8,
    pub isvararg: libc::c_char,
    pub istailcall: libc::c_char,
    pub ftransfer: libc::c_ushort,
    pub ntransfer: libc::c_ushort,
    pub short_src: [libc::c_char; 60],
    pub i_ci: *mut CallInfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CallInfo {
    pub func: StkIdRel,
    pub top: StkIdRel,
    pub previous: *mut CallInfo,
    pub next: *mut CallInfo,
    pub u: C2RustUnnamed_1,
    pub u2: C2RustUnnamed,
    pub nresults: libc::c_short,
    pub callstatus: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub funcidx: i32,
    pub nyield: i32,
    pub nres: i32,
    pub transferinfo: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub ftransfer: libc::c_ushort,
    pub ntransfer: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub l: C2RustUnnamed_3,
    pub c: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub k: lua_KFunction,
    pub old_errfunc: i64,
    pub ctx: lua_KContext,
}
pub type lua_KFunction = Option<unsafe extern "C" fn(*mut lua_State, i32, lua_KContext) -> i32>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub savedpc: *const Instruction,
    pub trap: sig_atomic_t,
    pub nextraargs: i32,
}
pub type Instruction = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub union StkIdRel {
    pub p: StkId,
    pub offset: i64,
}
pub type StkId = *mut StackValue;
#[derive(Copy, Clone)]
#[repr(C)]
pub union StackValue {
    pub val: TValue,
    pub tbclist: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub value_: Value,
    pub tt_: u8,
    pub delta: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union Value {
    pub gc: *mut GCObject,
    pub p: *mut libc::c_void,
    pub f: CFunction,
    pub i: i64,
    pub n: f64,
    pub ub: u8,
}

pub type CFunction = Option<unsafe extern "C" fn(*mut lua_State) -> i32>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GCObject {
    pub next: *mut GCObject,
    pub tt: u8,
    pub marked: u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TValue {
    pub value_: Value,
    pub tt_: u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UpVal {
    pub next: *mut GCObject,
    pub tt: u8,
    pub marked: u8,
    pub v: C2RustUnnamed_7,
    pub u: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub open: C2RustUnnamed_6,
    pub value: TValue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub next: *mut UpVal,
    pub previous: *mut *mut UpVal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub p: *mut TValue,
    pub offset: i64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct global_State {
    pub frealloc: lua_Alloc,
    pub ud: *mut libc::c_void,
    pub totalbytes: l_mem,
    pub GCdebt: l_mem,
    pub GCestimate: lu_mem,
    pub lastatomic: lu_mem,
    pub strt: stringtable,
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
    pub twups: *mut lua_State,
    pub panic: CFunction,
    pub mainthread: *mut lua_State,
    pub memerrmsg: *mut TString,
    pub tmname: [*mut TString; 25],
    pub mt: [*mut Table; 9],
    pub strcache: [[*mut TString; 2]; 53],
    pub warnf: lua_WarnFunction,
    pub ud_warn: *mut libc::c_void,
}
pub type lua_WarnFunction =
    Option<unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, i32) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TString {
    pub next: *mut GCObject,
    pub tt: u8,
    pub marked: u8,
    pub extra: u8,
    pub shrlen: u8,
    pub hash: u32,
    pub u: C2RustUnnamed_8,
    pub contents: [libc::c_char; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub lnglen: u64,
    pub hnext: *mut TString,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Table {
    pub next: *mut GCObject,
    pub tt: u8,
    pub marked: u8,
    pub flags: u8,
    pub lsizenode: u8,
    pub alimit: u32,
    pub array: *mut TValue,
    pub node: *mut Node,
    pub lastfree: *mut Node,
    pub metatable: *mut Table,
    pub gclist: *mut GCObject,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union Node {
    pub u: NodeKey,
    pub i_val: TValue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NodeKey {
    pub value_: Value,
    pub tt_: u8,
    pub key_tt: u8,
    pub next: i32,
    pub key_val: Value,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stringtable {
    pub hash: *mut *mut TString,
    pub nuse: i32,
    pub size: i32,
}
pub type lu_mem = u64;
pub type l_mem = i64;
pub type lua_Alloc = Option<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void, u64, u64) -> *mut libc::c_void,
>;
pub type ls_byte = libc::c_schar;
#[derive(Copy, Clone)]
#[repr(C)]
pub union UValue {
    pub uv: TValue,
    pub n: f64,
    pub u: f64,
    pub s: *mut libc::c_void,
    pub i: i64,
    pub l: i64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Udata {
    pub next: *mut GCObject,
    pub tt: u8,
    pub marked: u8,
    pub nuvalue: libc::c_ushort,
    pub len: u64,
    pub metatable: *mut Table,
    pub gclist: *mut GCObject,
    pub uv: [UValue; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Upvaldesc {
    pub name: *mut TString,
    pub instack: u8,
    pub index: u8,
    pub kind: u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LocVar {
    pub varname: *mut TString,
    pub startpc: i32,
    pub endpc: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AbsLineInfo {
    pub pc: i32,
    pub line: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Proto {
    pub next: *mut GCObject,
    pub tt: u8,
    pub marked: u8,
    pub numparams: u8,
    pub is_vararg: u8,
    pub maxstacksize: u8,
    pub sizeupvalues: i32,
    pub sizek: i32,
    pub sizecode: i32,
    pub sizelineinfo: i32,
    pub sizep: i32,
    pub sizelocvars: i32,
    pub sizeabslineinfo: i32,
    pub linedefined: i32,
    pub lastlinedefined: i32,
    pub k: *mut TValue,
    pub code: *mut Instruction,
    pub p: *mut *mut Proto,
    pub upvalues: *mut Upvaldesc,
    pub lineinfo: *mut ls_byte,
    pub abslineinfo: *mut AbsLineInfo,
    pub locvars: *mut LocVar,
    pub source: *mut TString,
    pub gclist: *mut GCObject,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CClosure {
    pub next: *mut GCObject,
    pub tt: u8,
    pub marked: u8,
    pub nupvalues: u8,
    pub gclist: *mut GCObject,
    pub f: CFunction,
    pub upvalue: [TValue; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LClosure {
    pub next: *mut GCObject,
    pub tt: u8,
    pub marked: u8,
    pub nupvalues: u8,
    pub gclist: *mut GCObject,
    pub p: *mut Proto,
    pub upvals: [*mut UpVal; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union Closure {
    pub c: CClosure,
    pub l: LClosure,
}
pub type TMS = u32;
pub const TM_N: TMS = 25;
pub const TM_CLOSE: TMS = 24;
pub const TM_CALL: TMS = 23;
pub const TM_CONCAT: TMS = 22;
pub const TM_LE: TMS = 21;
pub const TM_LT: TMS = 20;
pub const TM_BNOT: TMS = 19;
pub const TM_UNM: TMS = 18;
pub const TM_SHR: TMS = 17;
pub const TM_SHL: TMS = 16;
pub const TM_BXOR: TMS = 15;
pub const TM_BOR: TMS = 14;
pub const TM_BAND: TMS = 13;
pub const TM_IDIV: TMS = 12;
pub const TM_DIV: TMS = 11;
pub const TM_POW: TMS = 10;
pub const TM_MOD: TMS = 9;
pub const TM_MUL: TMS = 8;
pub const TM_SUB: TMS = 7;
pub const TM_ADD: TMS = 6;
pub const TM_EQ: TMS = 5;
pub const TM_LEN: TMS = 4;
pub const TM_MODE: TMS = 3;
pub const TM_GC: TMS = 2;
pub const TM_NEWINDEX: TMS = 1;
pub const TM_INDEX: TMS = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union GCUnion {
    pub gc: GCObject,
    pub ts: TString,
    pub u: Udata,
    pub cl: Closure,
    pub h: Table,
    pub p: Proto,
    pub th: lua_State,
    pub upv: UpVal,
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaF_newCclosure(
    mut L: *mut lua_State,
    mut nupvals: i32,
) -> *mut CClosure {
    let mut o: *mut GCObject = luaC_newobj(
        L,
        6i32 | (2i32) << 4i32,
        (32 as libc::c_ulong as i32
            + ::core::mem::size_of::<TValue>() as libc::c_ulong as i32 * nupvals) as u64,
    );
    let mut c: *mut CClosure = &mut (*(o as *mut GCUnion)).cl.c;
    (*c).nupvalues = nupvals as u8;
    return c;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaF_newLclosure(
    mut L: *mut lua_State,
    mut nupvals: i32,
) -> *mut LClosure {
    let mut o: *mut GCObject = luaC_newobj(
        L,
        6i32 | (0i32) << 4i32,
        (32 as libc::c_ulong as i32
            + ::core::mem::size_of::<*mut TValue>() as libc::c_ulong as i32 * nupvals)
            as u64,
    );
    let mut c: *mut LClosure = &mut (*(o as *mut GCUnion)).cl.l;
    (*c).p = 0 as *mut Proto;
    (*c).nupvalues = nupvals as u8;
    loop {
        let fresh0 = nupvals;
        nupvals = nupvals - 1;
        if !(fresh0 != 0) {
            break;
        }
        let ref mut fresh1 = *((*c).upvals).as_mut_ptr().offset(nupvals as isize);
        *fresh1 = 0 as *mut UpVal;
    }
    return c;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaF_initupvals(mut L: *mut lua_State, mut cl: *mut LClosure) {
    let mut i: i32 = 0;
    i = 0i32;
    while i < (*cl).nupvalues as i32 {
        let mut o: *mut GCObject = luaC_newobj(
            L,
            9i32 | (0i32) << 4i32,
            ::core::mem::size_of::<UpVal>() as libc::c_ulong,
        );
        let mut uv: *mut UpVal = &mut (*(o as *mut GCUnion)).upv;
        (*uv).v.p = &mut (*uv).u.value;
        (*(*uv).v.p).tt_ = (0i32 | (0i32) << 4i32) as u8;
        let ref mut fresh2 = *((*cl).upvals).as_mut_ptr().offset(i as isize);
        *fresh2 = uv;
        if (*cl).marked as i32 & (1i32) << 5i32 != 0
            && (*uv).marked as i32 & ((1i32) << 3i32 | (1i32) << 4i32) != 0
        {
            luaC_barrier_(
                L,
                &mut (*(cl as *mut GCUnion)).gc,
                &mut (*(uv as *mut GCUnion)).gc,
            );
        } else {
        };
        i += 1;
    }
}
unsafe extern "C" fn newupval(
    mut L: *mut lua_State,
    mut level: StkId,
    mut prev: *mut *mut UpVal,
) -> *mut UpVal {
    let mut o: *mut GCObject = luaC_newobj(
        L,
        9i32 | (0i32) << 4i32,
        ::core::mem::size_of::<UpVal>() as libc::c_ulong,
    );
    let mut uv: *mut UpVal = &mut (*(o as *mut GCUnion)).upv;
    let mut next: *mut UpVal = *prev;
    (*uv).v.p = &mut (*level).val;
    (*uv).u.open.next = next;
    (*uv).u.open.previous = prev;
    if !next.is_null() {
        (*next).u.open.previous = &mut (*uv).u.open.next;
    }
    *prev = uv;
    if !((*L).twups != L) {
        (*L).twups = (*(*L).l_G).twups;
        (*(*L).l_G).twups = L;
    }
    return uv;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaF_findupval(mut L: *mut lua_State, mut level: StkId) -> *mut UpVal {
    let mut pp: *mut *mut UpVal = &mut (*L).openupval;
    let mut p: *mut UpVal = 0 as *mut UpVal;
    loop {
        p = *pp;
        if !(!p.is_null() && (*p).v.p as StkId >= level) {
            break;
        }
        if (*p).v.p as StkId == level {
            return p;
        }
        pp = &mut (*p).u.open.next;
    }
    return newupval(L, level, pp);
}
unsafe extern "C" fn callclosemethod(
    mut L: *mut lua_State,
    mut obj: *mut TValue,
    mut err: *mut TValue,
    mut yy: i32,
) {
    let mut top: StkId = (*L).top.p;
    let mut tm: *const TValue = luaT_gettmbyobj(L, obj, TM_CLOSE);
    let mut io1: *mut TValue = &mut (*top).val;
    let mut io2: *const TValue = tm;
    (*io1).value_ = (*io2).value_;
    (*io1).tt_ = (*io2).tt_;
    let mut io1_0: *mut TValue = &mut (*top.offset(1i32 as isize)).val;
    let mut io2_0: *const TValue = obj;
    (*io1_0).value_ = (*io2_0).value_;
    (*io1_0).tt_ = (*io2_0).tt_;
    let mut io1_1: *mut TValue = &mut (*top.offset(2i32 as isize)).val;
    let mut io2_1: *const TValue = err;
    (*io1_1).value_ = (*io2_1).value_;
    (*io1_1).tt_ = (*io2_1).tt_;
    (*L).top.p = top.offset(3i32 as isize);
    if yy != 0 {
        luaD_call(L, top, 0i32);
    } else {
        luaD_callnoyield(L, top, 0i32);
    };
}
unsafe extern "C" fn checkclosemth(mut L: *mut lua_State, mut level: StkId) {
    let mut tm: *const TValue = luaT_gettmbyobj(L, &mut (*level).val, TM_CLOSE);
    if (*tm).tt_ as i32 & 0xf as i32 == 0i32 {
        let mut index: i32 = level.offset_from((*(*L).ci).func.p) as i64 as i32;
        let mut vname: *const libc::c_char = luaG_findlocal(L, (*L).ci, index, 0 as *mut StkId);
        if vname.is_null() {
            vname = b"?\0" as *const u8 as *const libc::c_char;
        }
        luaG_runerror(
            L,
            b"variable '%s' got a non-closable value\0" as *const u8 as *const libc::c_char,
            vname,
        );
    }
}
unsafe extern "C" fn prepcallclosemth(
    mut L: *mut lua_State,
    mut level: StkId,
    mut status: i32,
    mut yy: i32,
) {
    let mut uv: *mut TValue = &mut (*level).val;
    let mut errobj: *mut TValue = 0 as *mut TValue;
    if status == -(1i32) {
        errobj = &mut (*(*L).l_G).nilvalue;
    } else {
        errobj = &mut (*level.offset(1i32 as isize)).val;
        luaD_seterrorobj(L, status, level.offset(1i32 as isize));
    }
    callclosemethod(L, uv, errobj, yy);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaF_newtbcupval(mut L: *mut lua_State, mut level: StkId) {
    if (*level).val.tt_ as i32 == 1i32 | (0i32) << 4i32
        || (*level).val.tt_ as i32 & 0xf as i32 == 0i32
    {
        return;
    }
    checkclosemth(L, level);
    while level.offset_from((*L).tbclist.p) as i64 as u32 as libc::c_ulong
        > ((256 as libc::c_ulong)
            << (::core::mem::size_of::<libc::c_ushort>() as libc::c_ulong)
                .wrapping_sub(1i32 as libc::c_ulong)
                .wrapping_mul(8i32 as libc::c_ulong))
        .wrapping_sub(1i32 as libc::c_ulong)
    {
        (*L).tbclist.p = ((*L).tbclist.p).offset(
            ((256 as libc::c_ulong)
                << (::core::mem::size_of::<libc::c_ushort>() as libc::c_ulong)
                    .wrapping_sub(1i32 as libc::c_ulong)
                    .wrapping_mul(8i32 as libc::c_ulong))
            .wrapping_sub(1i32 as libc::c_ulong) as isize,
        );
        (*(*L).tbclist.p).tbclist.delta = 0i32 as libc::c_ushort;
    }
    (*level).tbclist.delta = level.offset_from((*L).tbclist.p) as i64 as libc::c_ushort;
    (*L).tbclist.p = level;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaF_unlinkupval(mut uv: *mut UpVal) {
    *(*uv).u.open.previous = (*uv).u.open.next;
    if !((*uv).u.open.next).is_null() {
        (*(*uv).u.open.next).u.open.previous = (*uv).u.open.previous;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaF_closeupval(mut L: *mut lua_State, mut level: StkId) {
    let mut uv: *mut UpVal = 0 as *mut UpVal;
    let mut upl: StkId = 0 as *mut StackValue;
    loop {
        uv = (*L).openupval;
        if !(!uv.is_null() && {
            upl = (*uv).v.p as StkId;
            upl >= level
        }) {
            break;
        }
        let mut slot: *mut TValue = &mut (*uv).u.value;
        luaF_unlinkupval(uv);
        let mut io1: *mut TValue = slot;
        let mut io2: *const TValue = (*uv).v.p;
        (*io1).value_ = (*io2).value_;
        (*io1).tt_ = (*io2).tt_;
        (*uv).v.p = slot;
        if (*uv).marked as i32 & ((1i32) << 3i32 | (1i32) << 4i32) == 0 {
            (*uv).marked = ((*uv).marked as i32 | (1i32) << 5i32) as u8;
            if (*slot).tt_ as i32 & (1i32) << 6i32 != 0 {
                if (*uv).marked as i32 & (1i32) << 5i32 != 0
                    && (*(*slot).value_.gc).marked as i32 & ((1i32) << 3i32 | (1i32) << 4i32) != 0
                {
                    luaC_barrier_(
                        L,
                        &mut (*(uv as *mut GCUnion)).gc,
                        &mut (*((*slot).value_.gc as *mut GCUnion)).gc,
                    );
                } else {
                };
            } else {
            };
        }
    }
}
unsafe extern "C" fn poptbclist(mut L: *mut lua_State) {
    let mut tbc: StkId = (*L).tbclist.p;
    tbc = tbc.offset(-((*tbc).tbclist.delta as i32 as isize));
    while tbc > (*L).stack.p && (*tbc).tbclist.delta as i32 == 0i32 {
        tbc = tbc.offset(
            -(((256 as libc::c_ulong)
                << (::core::mem::size_of::<libc::c_ushort>() as libc::c_ulong)
                    .wrapping_sub(1i32 as libc::c_ulong)
                    .wrapping_mul(8i32 as libc::c_ulong))
            .wrapping_sub(1i32 as libc::c_ulong) as isize),
        );
    }
    (*L).tbclist.p = tbc;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaF_close(
    mut L: *mut lua_State,
    mut level: StkId,
    mut status: i32,
    mut yy: i32,
) -> StkId {
    let mut levelrel: i64 =
        (level as *mut libc::c_char).offset_from((*L).stack.p as *mut libc::c_char) as i64;
    luaF_closeupval(L, level);
    while (*L).tbclist.p >= level {
        let mut tbc: StkId = (*L).tbclist.p;
        poptbclist(L);
        prepcallclosemth(L, tbc, status, yy);
        level = ((*L).stack.p as *mut libc::c_char).offset(levelrel as isize) as StkId;
    }
    return level;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaF_newproto(mut L: *mut lua_State) -> *mut Proto {
    let mut o: *mut GCObject = luaC_newobj(
        L,
        9i32 + 1i32 | (0i32) << 4i32,
        ::core::mem::size_of::<Proto>() as libc::c_ulong,
    );
    let mut f: *mut Proto = &mut (*(o as *mut GCUnion)).p;
    (*f).k = 0 as *mut TValue;
    (*f).sizek = 0i32;
    (*f).p = 0 as *mut *mut Proto;
    (*f).sizep = 0i32;
    (*f).code = 0 as *mut Instruction;
    (*f).sizecode = 0i32;
    (*f).lineinfo = 0 as *mut ls_byte;
    (*f).sizelineinfo = 0i32;
    (*f).abslineinfo = 0 as *mut AbsLineInfo;
    (*f).sizeabslineinfo = 0i32;
    (*f).upvalues = 0 as *mut Upvaldesc;
    (*f).sizeupvalues = 0i32;
    (*f).numparams = 0i32 as u8;
    (*f).is_vararg = 0i32 as u8;
    (*f).maxstacksize = 0i32 as u8;
    (*f).locvars = 0 as *mut LocVar;
    (*f).sizelocvars = 0i32;
    (*f).linedefined = 0i32;
    (*f).lastlinedefined = 0i32;
    (*f).source = 0 as *mut TString;
    return f;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaF_freeproto(mut L: *mut lua_State, mut f: *mut Proto) {
    luaM_free_(
        L,
        (*f).code as *mut libc::c_void,
        ((*f).sizecode as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<Instruction>() as libc::c_ulong),
    );
    luaM_free_(
        L,
        (*f).p as *mut libc::c_void,
        ((*f).sizep as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut Proto>() as libc::c_ulong),
    );
    luaM_free_(
        L,
        (*f).k as *mut libc::c_void,
        ((*f).sizek as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<TValue>() as libc::c_ulong),
    );
    luaM_free_(
        L,
        (*f).lineinfo as *mut libc::c_void,
        ((*f).sizelineinfo as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<ls_byte>() as libc::c_ulong),
    );
    luaM_free_(
        L,
        (*f).abslineinfo as *mut libc::c_void,
        ((*f).sizeabslineinfo as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<AbsLineInfo>() as libc::c_ulong),
    );
    luaM_free_(
        L,
        (*f).locvars as *mut libc::c_void,
        ((*f).sizelocvars as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<LocVar>() as libc::c_ulong),
    );
    luaM_free_(
        L,
        (*f).upvalues as *mut libc::c_void,
        ((*f).sizeupvalues as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<Upvaldesc>() as libc::c_ulong),
    );
    luaM_free_(
        L,
        f as *mut libc::c_void,
        ::core::mem::size_of::<Proto>() as libc::c_ulong,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaF_getlocalname(
    mut f: *const Proto,
    mut local_number: i32,
    mut pc: i32,
) -> *const libc::c_char {
    let mut i: i32 = 0;
    i = 0i32;
    while i < (*f).sizelocvars && (*((*f).locvars).offset(i as isize)).startpc <= pc {
        if pc < (*((*f).locvars).offset(i as isize)).endpc {
            local_number -= 1;
            if local_number == 0i32 {
                return ((*(*((*f).locvars).offset(i as isize)).varname).contents).as_mut_ptr();
            }
        }
        i += 1;
    }
    return 0 as *const libc::c_char;
}
