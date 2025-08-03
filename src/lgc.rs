#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types)]
unsafe extern "C" {
    pub type lua_longjmp;
    fn strchr(_: *const libc::c_char, _: i32) -> *mut libc::c_char;
    fn luaE_setdebt(g: *mut global_State, debt: l_mem);
    fn luaE_freethread(L: *mut lua_State, L1: *mut lua_State);
    fn luaE_warnerror(L: *mut lua_State, where_0: *const libc::c_char);
    fn luaT_gettm(events: *mut Table, event: TMS, ename: *mut TString) -> *const TValue;
    fn luaT_gettmbyobj(L: *mut lua_State, o: *const TValue, event: TMS) -> *const TValue;
    fn luaM_free_(L: *mut lua_State, block: *mut libc::c_void, osize: size_t);
    fn luaM_malloc_(
        L: *mut lua_State,
        size: size_t,
        tag: i32,
    ) -> *mut libc::c_void;
    fn luaD_callnoyield(L: *mut lua_State, func: StkId, nResults: i32);
    fn luaD_pcall(
        L: *mut lua_State,
        func: Pfunc,
        u: *mut libc::c_void,
        oldtop: ptrdiff_t,
        ef: ptrdiff_t,
    ) -> i32;
    fn luaD_shrinkstack(L: *mut lua_State);
    fn luaF_unlinkupval(uv: *mut UpVal);
    fn luaF_freeproto(L: *mut lua_State, f: *mut Proto);
    fn luaS_resize(L: *mut lua_State, newsize: i32);
    fn luaS_clearcache(g: *mut global_State);
    fn luaS_remove(L: *mut lua_State, ts: *mut TString);
    fn luaH_free(L: *mut lua_State, t: *mut Table);
    fn luaH_realasize(t: *const Table) -> libc::c_uint;
}
pub type size_t = libc::c_ulong;
pub type __sig_atomic_t = i32;
pub type ptrdiff_t = libc::c_long;
pub type intptr_t = libc::c_long;
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
    pub errorJmp: *mut lua_longjmp,
    pub base_ci: CallInfo,
    pub hook: lua_Hook,
    pub errfunc: ptrdiff_t,
    pub nCcalls: l_uint32,
    pub oldpc: i32,
    pub basehookcount: i32,
    pub hookcount: i32,
    pub hookmask: sig_atomic_t,
}
pub type sig_atomic_t = __sig_atomic_t;
pub type l_uint32 = libc::c_uint;
pub type lua_Hook = Option::<unsafe extern "C" fn(*mut lua_State, *mut lua_Debug) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lua_Debug {
    pub event: i32,
    pub name: *const libc::c_char,
    pub namewhat: *const libc::c_char,
    pub what: *const libc::c_char,
    pub source: *const libc::c_char,
    pub srclen: size_t,
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
    pub old_errfunc: ptrdiff_t,
    pub ctx: lua_KContext,
}
pub type lua_KContext = intptr_t;
pub type lua_KFunction = Option::<
    unsafe extern "C" fn(*mut lua_State, i32, lua_KContext) -> i32,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub savedpc: *const Instruction,
    pub trap: sig_atomic_t,
    pub nextraargs: i32,
}
pub type Instruction = l_uint32;
#[derive(Copy, Clone)]
#[repr(C)]
pub union StkIdRel {
    pub p: StkId,
    pub offset: ptrdiff_t,
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
    pub i: Integer,
    pub n: Number,
    pub ub: u8,
}
pub type Number = f64;
pub type Integer = i64;
pub type CFunction = Option::<unsafe extern "C" fn(*mut lua_State) -> i32>;
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
    pub offset: ptrdiff_t,
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
    pub seed: libc::c_uint,
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
pub type lua_WarnFunction = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, i32) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TString {
    pub next: *mut GCObject,
    pub tt: u8,
    pub marked: u8,
    pub extra: u8,
    pub shrlen: u8,
    pub hash: libc::c_uint,
    pub u: C2RustUnnamed_8,
    pub contents: [libc::c_char; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub lnglen: size_t,
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
    pub alimit: libc::c_uint,
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
pub type lu_mem = size_t;
pub type l_mem = ptrdiff_t;
pub type lua_Alloc = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *mut libc::c_void,
        size_t,
        size_t,
    ) -> *mut libc::c_void,
>;
pub type ls_byte = libc::c_schar;
#[derive(Copy, Clone)]
#[repr(C)]
pub union UValue {
    pub uv: TValue,
    pub n: Number,
    pub u: f64,
    pub s: *mut libc::c_void,
    pub i: Integer,
    pub l: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Udata {
    pub next: *mut GCObject,
    pub tt: u8,
    pub marked: u8,
    pub nuvalue: libc::c_ushort,
    pub len: size_t,
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
pub type TMS = libc::c_uint;
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
pub type Pfunc = Option::<unsafe extern "C" fn(*mut lua_State, *mut libc::c_void) -> ()>;
unsafe extern "C" fn getgclist(mut o: *mut GCObject) -> *mut *mut GCObject {
    match (*o).tt as i32 {
        5 => return &mut (*(o as *mut GCUnion)).h.gclist,
        6 => return &mut (*(o as *mut GCUnion)).cl.l.gclist,
        38 => return &mut (*(o as *mut GCUnion)).cl.c.gclist,
        8 => return &mut (*(o as *mut GCUnion)).th.gclist,
        10 => return &mut (*(o as *mut GCUnion)).p.gclist,
        7 => {
            let mut u: *mut Udata = &mut (*(o as *mut GCUnion)).u;
            return &mut (*u).gclist;
        }
        _ => return 0 as *mut *mut GCObject,
    };
}
unsafe extern "C" fn linkgclist_(
    mut o: *mut GCObject,
    mut pnext: *mut *mut GCObject,
    mut list: *mut *mut GCObject,
) {
    *pnext = *list;
    *list = o;
    (*o)
        .marked = ((*o).marked as i32
        & !((1 as i32) << 5 as i32
            | ((1 as i32) << 3 as i32
                | (1 as i32) << 4 as i32)) as u8 as i32)
        as u8;
}
unsafe extern "C" fn clearkey(mut n: *mut Node) {
    if (*n).u.key_tt as i32 & (1 as i32) << 6 as i32 != 0 {
        (*n).u.key_tt = (9 as i32 + 2 as i32) as u8;
    }
}
unsafe extern "C" fn iscleared(
    mut g: *mut global_State,
    mut o: *const GCObject,
) -> i32 {
    if o.is_null() {
        return 0 as i32
    } else if (*o).tt as i32 & 0xf as i32 == 4 as i32 {
        if (*o).marked as i32
            & ((1 as i32) << 3 as i32
                | (1 as i32) << 4 as i32) != 0
        {
            reallymarkobject(g, &mut (*(o as *mut GCUnion)).gc);
        }
        return 0 as i32;
    } else {
        return (*o).marked as i32
            & ((1 as i32) << 3 as i32
                | (1 as i32) << 4 as i32)
    };
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaC_barrier_(
    mut L: *mut lua_State,
    mut o: *mut GCObject,
    mut v: *mut GCObject,
) {
    let mut g: *mut global_State = (*L).l_G;
    if (*g).gcstate as i32 <= 2 as i32 {
        reallymarkobject(g, v);
        if (*o).marked as i32 & 7 as i32 > 1 as i32 {
            (*v)
                .marked = ((*v).marked as i32 & !(7 as i32)
                | 2 as i32) as u8;
        }
    } else if (*g).gckind as i32 == 0 as i32 {
        (*o)
            .marked = ((*o).marked as i32
            & !((1 as i32) << 5 as i32
                | ((1 as i32) << 3 as i32
                    | (1 as i32) << 4 as i32))
            | ((*g).currentwhite as i32
                & ((1 as i32) << 3 as i32
                    | (1 as i32) << 4 as i32)) as u8 as i32)
            as u8;
    }
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaC_barrierback_(mut L: *mut lua_State, mut o: *mut GCObject) {
    let mut g: *mut global_State = (*L).l_G;
    if (*o).marked as i32 & 7 as i32 == 6 as i32 {
        (*o)
            .marked = ((*o).marked as i32
            & !((1 as i32) << 5 as i32
                | ((1 as i32) << 3 as i32
                    | (1 as i32) << 4 as i32)) as u8 as i32)
            as u8;
    } else {
        linkgclist_(&mut (*(o as *mut GCUnion)).gc, getgclist(o), &mut (*g).grayagain);
    }
    if (*o).marked as i32 & 7 as i32 > 1 as i32 {
        (*o)
            .marked = ((*o).marked as i32 & !(7 as i32)
            | 5 as i32) as u8;
    }
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaC_fix(mut L: *mut lua_State, mut o: *mut GCObject) {
    let mut g: *mut global_State = (*L).l_G;
    (*o)
        .marked = ((*o).marked as i32
        & !((1 as i32) << 5 as i32
            | ((1 as i32) << 3 as i32
                | (1 as i32) << 4 as i32)) as u8 as i32)
        as u8;
    (*o)
        .marked = ((*o).marked as i32 & !(7 as i32) | 4 as i32)
        as u8;
    (*g).allgc = (*o).next;
    (*o).next = (*g).fixedgc;
    (*g).fixedgc = o;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaC_newobjdt(
    mut L: *mut lua_State,
    mut tt: i32,
    mut sz: size_t,
    mut offset: size_t,
) -> *mut GCObject {
    let mut g: *mut global_State = (*L).l_G;
    let mut p: *mut libc::c_char = luaM_malloc_(L, sz, tt & 0xf as i32)
        as *mut libc::c_char;
    let mut o: *mut GCObject = p.offset(offset as isize) as *mut GCObject;
    (*o)
        .marked = ((*g).currentwhite as i32
        & ((1 as i32) << 3 as i32
            | (1 as i32) << 4 as i32)) as u8;
    (*o).tt = tt as u8;
    (*o).next = (*g).allgc;
    (*g).allgc = o;
    return o;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaC_newobj(
    mut L: *mut lua_State,
    mut tt: i32,
    mut sz: size_t,
) -> *mut GCObject {
    return luaC_newobjdt(L, tt, sz, 0 as i32 as size_t);
}
unsafe extern "C" fn reallymarkobject(mut g: *mut global_State, mut o: *mut GCObject) {
    let mut current_block_18: u64;
    match (*o).tt as i32 {
        4 | 20 => {
            (*o)
                .marked = ((*o).marked as i32
                & !((1 as i32) << 3 as i32
                    | (1 as i32) << 4 as i32)
                | (1 as i32) << 5 as i32) as u8;
            current_block_18 = 18317007320854588510;
        }
        9 => {
            let mut uv: *mut UpVal = &mut (*(o as *mut GCUnion)).upv;
            if (*uv).v.p != &mut (*uv).u.value as *mut TValue {
                (*uv)
                    .marked = ((*uv).marked as i32
                    & !((1 as i32) << 5 as i32
                        | ((1 as i32) << 3 as i32
                            | (1 as i32) << 4 as i32)) as u8
                        as i32) as u8;
            } else {
                (*uv)
                    .marked = ((*uv).marked as i32
                    & !((1 as i32) << 3 as i32
                        | (1 as i32) << 4 as i32)
                    | (1 as i32) << 5 as i32) as u8;
            }
            if (*(*uv).v.p).tt_ as i32 & (1 as i32) << 6 as i32
                != 0
                && (*(*(*uv).v.p).value_.gc).marked as i32
                    & ((1 as i32) << 3 as i32
                        | (1 as i32) << 4 as i32) != 0
            {
                reallymarkobject(g, (*(*uv).v.p).value_.gc);
            }
            current_block_18 = 18317007320854588510;
        }
        7 => {
            let mut u: *mut Udata = &mut (*(o as *mut GCUnion)).u;
            if (*u).nuvalue as i32 == 0 as i32 {
                if !((*u).metatable).is_null() {
                    if (*(*u).metatable).marked as i32
                        & ((1 as i32) << 3 as i32
                            | (1 as i32) << 4 as i32) != 0
                    {
                        reallymarkobject(g, &mut (*((*u).metatable as *mut GCUnion)).gc);
                    }
                }
                (*u)
                    .marked = ((*u).marked as i32
                    & !((1 as i32) << 3 as i32
                        | (1 as i32) << 4 as i32)
                    | (1 as i32) << 5 as i32) as u8;
                current_block_18 = 18317007320854588510;
            } else {
                current_block_18 = 15904375183555213903;
            }
        }
        6 | 38 | 5 | 8 | 10 => {
            current_block_18 = 15904375183555213903;
        }
        _ => {
            current_block_18 = 18317007320854588510;
        }
    }
    match current_block_18 {
        15904375183555213903 => {
            linkgclist_(&mut (*(o as *mut GCUnion)).gc, getgclist(o), &mut (*g).gray);
        }
        _ => {}
    };
}
unsafe extern "C" fn markmt(mut g: *mut global_State) {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < 9 as i32 {
        if !((*g).mt[i as usize]).is_null() {
            if (*(*g).mt[i as usize]).marked as i32
                & ((1 as i32) << 3 as i32
                    | (1 as i32) << 4 as i32) != 0
            {
                reallymarkobject(
                    g,
                    &mut (*(*((*g).mt).as_mut_ptr().offset(i as isize) as *mut GCUnion))
                        .gc,
                );
            }
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn markbeingfnz(mut g: *mut global_State) -> lu_mem {
    let mut o: *mut GCObject = 0 as *mut GCObject;
    let mut count: lu_mem = 0 as i32 as lu_mem;
    o = (*g).tobefnz;
    while !o.is_null() {
        count = count.wrapping_add(1);
        count;
        if (*o).marked as i32
            & ((1 as i32) << 3 as i32
                | (1 as i32) << 4 as i32) != 0
        {
            reallymarkobject(g, &mut (*(o as *mut GCUnion)).gc);
        }
        o = (*o).next;
    }
    return count;
}
unsafe extern "C" fn remarkupvals(mut g: *mut global_State) -> i32 {
    let mut thread: *mut lua_State = 0 as *mut lua_State;
    let mut p: *mut *mut lua_State = &mut (*g).twups;
    let mut work: i32 = 0 as i32;
    loop {
        thread = *p;
        if thread.is_null() {
            break;
        }
        work += 1;
        work;
        if (*thread).marked as i32
            & ((1 as i32) << 3 as i32
                | (1 as i32) << 4 as i32) == 0
            && !((*thread).openupval).is_null()
        {
            p = &mut (*thread).twups;
        } else {
            let mut uv: *mut UpVal = 0 as *mut UpVal;
            *p = (*thread).twups;
            (*thread).twups = thread;
            uv = (*thread).openupval;
            while !uv.is_null() {
                work += 1;
                work;
                if (*uv).marked as i32
                    & ((1 as i32) << 3 as i32
                        | (1 as i32) << 4 as i32) == 0
                {
                    if (*(*uv).v.p).tt_ as i32
                        & (1 as i32) << 6 as i32 != 0
                        && (*(*(*uv).v.p).value_.gc).marked as i32
                            & ((1 as i32) << 3 as i32
                                | (1 as i32) << 4 as i32) != 0
                    {
                        reallymarkobject(g, (*(*uv).v.p).value_.gc);
                    }
                }
                uv = (*uv).u.open.next;
            }
        }
    }
    return work;
}
unsafe extern "C" fn cleargraylists(mut g: *mut global_State) {
    (*g).grayagain = 0 as *mut GCObject;
    (*g).gray = (*g).grayagain;
    (*g).ephemeron = 0 as *mut GCObject;
    (*g).allweak = (*g).ephemeron;
    (*g).weak = (*g).allweak;
}
unsafe extern "C" fn restartcollection(mut g: *mut global_State) {
    cleargraylists(g);
    if (*(*g).mainthread).marked as i32
        & ((1 as i32) << 3 as i32
            | (1 as i32) << 4 as i32) != 0
    {
        reallymarkobject(g, &mut (*((*g).mainthread as *mut GCUnion)).gc);
    }
    if (*g).l_registry.tt_ as i32 & (1 as i32) << 6 as i32 != 0
        && (*(*g).l_registry.value_.gc).marked as i32
            & ((1 as i32) << 3 as i32
                | (1 as i32) << 4 as i32) != 0
    {
        reallymarkobject(g, (*g).l_registry.value_.gc);
    }
    markmt(g);
    markbeingfnz(g);
}
unsafe extern "C" fn genlink(mut g: *mut global_State, mut o: *mut GCObject) {
    if (*o).marked as i32 & 7 as i32 == 5 as i32 {
        linkgclist_(&mut (*(o as *mut GCUnion)).gc, getgclist(o), &mut (*g).grayagain);
    } else if (*o).marked as i32 & 7 as i32 == 6 as i32 {
        (*o)
            .marked = ((*o).marked as i32
            ^ (6 as i32 ^ 4 as i32)) as u8;
    }
}
unsafe extern "C" fn traverseweakvalue(mut g: *mut global_State, mut h: *mut Table) {
    let mut n: *mut Node = 0 as *mut Node;
    let mut limit: *mut Node = &mut *((*h).node)
        .offset(((1 as i32) << (*h).lsizenode as i32) as size_t as isize)
        as *mut Node;
    let mut hasclears: i32 = ((*h).alimit > 0 as i32 as libc::c_uint)
        as i32;
    n = &mut *((*h).node).offset(0 as i32 as isize) as *mut Node;
    while n < limit {
        if (*n).i_val.tt_ as i32 & 0xf as i32 == 0 as i32 {
            clearkey(n);
        } else {
            if (*n).u.key_tt as i32 & (1 as i32) << 6 as i32 != 0
                && (*(*n).u.key_val.gc).marked as i32
                    & ((1 as i32) << 3 as i32
                        | (1 as i32) << 4 as i32) != 0
            {
                reallymarkobject(g, (*n).u.key_val.gc);
            }
            if hasclears == 0
                && iscleared(
                    g,
                    (if (*n).i_val.tt_ as i32
                        & (1 as i32) << 6 as i32 != 0
                    {
                        (*n).i_val.value_.gc
                    } else {
                        0 as *mut GCObject
                    }),
                ) != 0
            {
                hasclears = 1 as i32;
            }
        }
        n = n.offset(1);
        n;
    }
    if (*g).gcstate as i32 == 2 as i32 && hasclears != 0 {
        linkgclist_(&mut (*(h as *mut GCUnion)).gc, &mut (*h).gclist, &mut (*g).weak);
    } else {
        linkgclist_(
            &mut (*(h as *mut GCUnion)).gc,
            &mut (*h).gclist,
            &mut (*g).grayagain,
        );
    };
}
unsafe extern "C" fn traverseephemeron(
    mut g: *mut global_State,
    mut h: *mut Table,
    mut inv: i32,
) -> i32 {
    let mut marked: i32 = 0 as i32;
    let mut hasclears: i32 = 0 as i32;
    let mut hasww: i32 = 0 as i32;
    let mut i: libc::c_uint = 0;
    let mut asize: libc::c_uint = luaH_realasize(h);
    let mut nsize: libc::c_uint = ((1 as i32) << (*h).lsizenode as i32)
        as libc::c_uint;
    i = 0 as i32 as libc::c_uint;
    while i < asize {
        if (*((*h).array).offset(i as isize)).tt_ as i32
            & (1 as i32) << 6 as i32 != 0
            && (*(*((*h).array).offset(i as isize)).value_.gc).marked as i32
                & ((1 as i32) << 3 as i32
                    | (1 as i32) << 4 as i32) != 0
        {
            marked = 1 as i32;
            reallymarkobject(g, (*((*h).array).offset(i as isize)).value_.gc);
        }
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as i32 as libc::c_uint;
    while i < nsize {
        let mut n: *mut Node = if inv != 0 {
            &mut *((*h).node)
                .offset(
                    nsize.wrapping_sub(1 as i32 as libc::c_uint).wrapping_sub(i)
                        as isize,
                ) as *mut Node
        } else {
            &mut *((*h).node).offset(i as isize) as *mut Node
        };
        if (*n).i_val.tt_ as i32 & 0xf as i32 == 0 as i32 {
            clearkey(n);
        } else if iscleared(
            g,
            if (*n).u.key_tt as i32 & (1 as i32) << 6 as i32 != 0
            {
                (*n).u.key_val.gc
            } else {
                0 as *mut GCObject
            },
        ) != 0
        {
            hasclears = 1 as i32;
            if (*n).i_val.tt_ as i32 & (1 as i32) << 6 as i32
                != 0
                && (*(*n).i_val.value_.gc).marked as i32
                    & ((1 as i32) << 3 as i32
                        | (1 as i32) << 4 as i32) != 0
            {
                hasww = 1 as i32;
            }
        } else if (*n).i_val.tt_ as i32 & (1 as i32) << 6 as i32
            != 0
            && (*(*n).i_val.value_.gc).marked as i32
                & ((1 as i32) << 3 as i32
                    | (1 as i32) << 4 as i32) != 0
        {
            marked = 1 as i32;
            reallymarkobject(g, (*n).i_val.value_.gc);
        }
        i = i.wrapping_add(1);
        i;
    }
    if (*g).gcstate as i32 == 0 as i32 {
        linkgclist_(
            &mut (*(h as *mut GCUnion)).gc,
            &mut (*h).gclist,
            &mut (*g).grayagain,
        );
    } else if hasww != 0 {
        linkgclist_(
            &mut (*(h as *mut GCUnion)).gc,
            &mut (*h).gclist,
            &mut (*g).ephemeron,
        );
    } else if hasclears != 0 {
        linkgclist_(&mut (*(h as *mut GCUnion)).gc, &mut (*h).gclist, &mut (*g).allweak);
    } else {
        genlink(g, &mut (*(h as *mut GCUnion)).gc);
    }
    return marked;
}
unsafe extern "C" fn traversestrongtable(mut g: *mut global_State, mut h: *mut Table) {
    let mut n: *mut Node = 0 as *mut Node;
    let mut limit: *mut Node = &mut *((*h).node)
        .offset(((1 as i32) << (*h).lsizenode as i32) as size_t as isize)
        as *mut Node;
    let mut i: libc::c_uint = 0;
    let mut asize: libc::c_uint = luaH_realasize(h);
    i = 0 as i32 as libc::c_uint;
    while i < asize {
        if (*((*h).array).offset(i as isize)).tt_ as i32
            & (1 as i32) << 6 as i32 != 0
            && (*(*((*h).array).offset(i as isize)).value_.gc).marked as i32
                & ((1 as i32) << 3 as i32
                    | (1 as i32) << 4 as i32) != 0
        {
            reallymarkobject(g, (*((*h).array).offset(i as isize)).value_.gc);
        }
        i = i.wrapping_add(1);
        i;
    }
    n = &mut *((*h).node).offset(0 as i32 as isize) as *mut Node;
    while n < limit {
        if (*n).i_val.tt_ as i32 & 0xf as i32 == 0 as i32 {
            clearkey(n);
        } else {
            if (*n).u.key_tt as i32 & (1 as i32) << 6 as i32 != 0
                && (*(*n).u.key_val.gc).marked as i32
                    & ((1 as i32) << 3 as i32
                        | (1 as i32) << 4 as i32) != 0
            {
                reallymarkobject(g, (*n).u.key_val.gc);
            }
            if (*n).i_val.tt_ as i32 & (1 as i32) << 6 as i32
                != 0
                && (*(*n).i_val.value_.gc).marked as i32
                    & ((1 as i32) << 3 as i32
                        | (1 as i32) << 4 as i32) != 0
            {
                reallymarkobject(g, (*n).i_val.value_.gc);
            }
        }
        n = n.offset(1);
        n;
    }
    genlink(g, &mut (*(h as *mut GCUnion)).gc);
}
unsafe extern "C" fn traversetable(
    mut g: *mut global_State,
    mut h: *mut Table,
) -> lu_mem {
    let mut weakkey: *const libc::c_char = 0 as *const libc::c_char;
    let mut weakvalue: *const libc::c_char = 0 as *const libc::c_char;
    let mut mode: *const TValue = if ((*h).metatable).is_null() {
        0 as *const TValue
    } else if (*(*h).metatable).flags as libc::c_uint
        & (1 as libc::c_uint) << TM_MODE as i32 != 0
    {
        0 as *const TValue
    } else {
        luaT_gettm((*h).metatable, TM_MODE, (*g).tmname[TM_MODE as i32 as usize])
    };
    let mut smode: *mut TString = 0 as *mut TString;
    if !((*h).metatable).is_null() {
        if (*(*h).metatable).marked as i32
            & ((1 as i32) << 3 as i32
                | (1 as i32) << 4 as i32) != 0
        {
            reallymarkobject(g, &mut (*((*h).metatable as *mut GCUnion)).gc);
        }
    }
    if !mode.is_null()
        && (*mode).tt_ as i32
            == 4 as i32 | (0 as i32) << 4 as i32
                | (1 as i32) << 6 as i32
        && {
            smode = &mut (*((*mode).value_.gc as *mut GCUnion)).ts as *mut TString;
            weakkey = strchr(((*smode).contents).as_mut_ptr(), 'k' as i32);
            weakvalue = strchr(((*smode).contents).as_mut_ptr(), 'v' as i32);
            !weakkey.is_null() || !weakvalue.is_null()
        }
    {
        if weakkey.is_null() {
            traverseweakvalue(g, h);
        } else if weakvalue.is_null() {
            traverseephemeron(g, h, 0 as i32);
        } else {
            linkgclist_(
                &mut (*(h as *mut GCUnion)).gc,
                &mut (*h).gclist,
                &mut (*g).allweak,
            );
        }
    } else {
        traversestrongtable(g, h);
    }
    return (1 as i32 as libc::c_uint)
        .wrapping_add((*h).alimit)
        .wrapping_add(
            (2 as i32
                * (if ((*h).lastfree).is_null() {
                    0 as i32
                } else {
                    (1 as i32) << (*h).lsizenode as i32
                })) as libc::c_uint,
        ) as lu_mem;
}
unsafe extern "C" fn traverseudata(
    mut g: *mut global_State,
    mut u: *mut Udata,
) -> i32 {
    let mut i: i32 = 0;
    if !((*u).metatable).is_null() {
        if (*(*u).metatable).marked as i32
            & ((1 as i32) << 3 as i32
                | (1 as i32) << 4 as i32) != 0
        {
            reallymarkobject(g, &mut (*((*u).metatable as *mut GCUnion)).gc);
        }
    }
    i = 0 as i32;
    while i < (*u).nuvalue as i32 {
        if (*((*u).uv).as_mut_ptr().offset(i as isize)).uv.tt_ as i32
            & (1 as i32) << 6 as i32 != 0
            && (*(*((*u).uv).as_mut_ptr().offset(i as isize)).uv.value_.gc).marked
                as i32
                & ((1 as i32) << 3 as i32
                    | (1 as i32) << 4 as i32) != 0
        {
            reallymarkobject(
                g,
                (*((*u).uv).as_mut_ptr().offset(i as isize)).uv.value_.gc,
            );
        }
        i += 1;
        i;
    }
    genlink(g, &mut (*(u as *mut GCUnion)).gc);
    return 1 as i32 + (*u).nuvalue as i32;
}
unsafe extern "C" fn traverseproto(
    mut g: *mut global_State,
    mut f: *mut Proto,
) -> i32 {
    let mut i: i32 = 0;
    if !((*f).source).is_null() {
        if (*(*f).source).marked as i32
            & ((1 as i32) << 3 as i32
                | (1 as i32) << 4 as i32) != 0
        {
            reallymarkobject(g, &mut (*((*f).source as *mut GCUnion)).gc);
        }
    }
    i = 0 as i32;
    while i < (*f).sizek {
        if (*((*f).k).offset(i as isize)).tt_ as i32
            & (1 as i32) << 6 as i32 != 0
            && (*(*((*f).k).offset(i as isize)).value_.gc).marked as i32
                & ((1 as i32) << 3 as i32
                    | (1 as i32) << 4 as i32) != 0
        {
            reallymarkobject(g, (*((*f).k).offset(i as isize)).value_.gc);
        }
        i += 1;
        i;
    }
    i = 0 as i32;
    while i < (*f).sizeupvalues {
        if !((*((*f).upvalues).offset(i as isize)).name).is_null() {
            if (*(*((*f).upvalues).offset(i as isize)).name).marked as i32
                & ((1 as i32) << 3 as i32
                    | (1 as i32) << 4 as i32) != 0
            {
                reallymarkobject(
                    g,
                    &mut (*((*((*f).upvalues).offset(i as isize)).name as *mut GCUnion))
                        .gc,
                );
            }
        }
        i += 1;
        i;
    }
    i = 0 as i32;
    while i < (*f).sizep {
        if !(*((*f).p).offset(i as isize)).is_null() {
            if (**((*f).p).offset(i as isize)).marked as i32
                & ((1 as i32) << 3 as i32
                    | (1 as i32) << 4 as i32) != 0
            {
                reallymarkobject(
                    g,
                    &mut (*(*((*f).p).offset(i as isize) as *mut GCUnion)).gc,
                );
            }
        }
        i += 1;
        i;
    }
    i = 0 as i32;
    while i < (*f).sizelocvars {
        if !((*((*f).locvars).offset(i as isize)).varname).is_null() {
            if (*(*((*f).locvars).offset(i as isize)).varname).marked as i32
                & ((1 as i32) << 3 as i32
                    | (1 as i32) << 4 as i32) != 0
            {
                reallymarkobject(
                    g,
                    &mut (*((*((*f).locvars).offset(i as isize)).varname
                        as *mut GCUnion))
                        .gc,
                );
            }
        }
        i += 1;
        i;
    }
    return 1 as i32 + (*f).sizek + (*f).sizeupvalues + (*f).sizep
        + (*f).sizelocvars;
}
unsafe extern "C" fn traverseCclosure(
    mut g: *mut global_State,
    mut cl: *mut CClosure,
) -> i32 {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < (*cl).nupvalues as i32 {
        if (*((*cl).upvalue).as_mut_ptr().offset(i as isize)).tt_ as i32
            & (1 as i32) << 6 as i32 != 0
            && (*(*((*cl).upvalue).as_mut_ptr().offset(i as isize)).value_.gc).marked
                as i32
                & ((1 as i32) << 3 as i32
                    | (1 as i32) << 4 as i32) != 0
        {
            reallymarkobject(
                g,
                (*((*cl).upvalue).as_mut_ptr().offset(i as isize)).value_.gc,
            );
        }
        i += 1;
        i;
    }
    return 1 as i32 + (*cl).nupvalues as i32;
}
unsafe extern "C" fn traverseLclosure(
    mut g: *mut global_State,
    mut cl: *mut LClosure,
) -> i32 {
    let mut i: i32 = 0;
    if !((*cl).p).is_null() {
        if (*(*cl).p).marked as i32
            & ((1 as i32) << 3 as i32
                | (1 as i32) << 4 as i32) != 0
        {
            reallymarkobject(g, &mut (*((*cl).p as *mut GCUnion)).gc);
        }
    }
    i = 0 as i32;
    while i < (*cl).nupvalues as i32 {
        let mut uv: *mut UpVal = *((*cl).upvals).as_mut_ptr().offset(i as isize);
        if !uv.is_null() {
            if (*uv).marked as i32
                & ((1 as i32) << 3 as i32
                    | (1 as i32) << 4 as i32) != 0
            {
                reallymarkobject(g, &mut (*(uv as *mut GCUnion)).gc);
            }
        }
        i += 1;
        i;
    }
    return 1 as i32 + (*cl).nupvalues as i32;
}
unsafe extern "C" fn traversethread(
    mut g: *mut global_State,
    mut th: *mut lua_State,
) -> i32 {
    let mut uv: *mut UpVal = 0 as *mut UpVal;
    let mut o: StkId = (*th).stack.p;
    if (*th).marked as i32 & 7 as i32 > 1 as i32
        || (*g).gcstate as i32 == 0 as i32
    {
        linkgclist_(
            &mut (*(th as *mut GCUnion)).gc,
            &mut (*th).gclist,
            &mut (*g).grayagain,
        );
    }
    if o.is_null() {
        return 1 as i32;
    }
    while o < (*th).top.p {
        if (*o).val.tt_ as i32 & (1 as i32) << 6 as i32 != 0
            && (*(*o).val.value_.gc).marked as i32
                & ((1 as i32) << 3 as i32
                    | (1 as i32) << 4 as i32) != 0
        {
            reallymarkobject(g, (*o).val.value_.gc);
        }
        o = o.offset(1);
        o;
    }
    uv = (*th).openupval;
    while !uv.is_null() {
        if (*uv).marked as i32
            & ((1 as i32) << 3 as i32
                | (1 as i32) << 4 as i32) != 0
        {
            reallymarkobject(g, &mut (*(uv as *mut GCUnion)).gc);
        }
        uv = (*uv).u.open.next;
    }
    if (*g).gcstate as i32 == 2 as i32 {
        if (*g).gcemergency == 0 {
            luaD_shrinkstack(th);
        }
        o = (*th).top.p;
        while o < ((*th).stack_last.p).offset(5 as i32 as isize) {
            (*o)
                .val
                .tt_ = (0 as i32 | (0 as i32) << 4 as i32)
                as u8;
            o = o.offset(1);
            o;
        }
        if !((*th).twups != th) && !((*th).openupval).is_null() {
            (*th).twups = (*g).twups;
            (*g).twups = th;
        }
    }
    return 1 as i32
        + ((*th).stack_last.p).offset_from((*th).stack.p) as libc::c_long as i32;
}
unsafe extern "C" fn propagatemark(mut g: *mut global_State) -> lu_mem {
    let mut o: *mut GCObject = (*g).gray;
    (*o)
        .marked = ((*o).marked as i32 | (1 as i32) << 5 as i32)
        as u8;
    (*g).gray = *getgclist(o);
    match (*o).tt as i32 {
        5 => return traversetable(g, &mut (*(o as *mut GCUnion)).h),
        7 => return traverseudata(g, &mut (*(o as *mut GCUnion)).u) as lu_mem,
        6 => return traverseLclosure(g, &mut (*(o as *mut GCUnion)).cl.l) as lu_mem,
        38 => return traverseCclosure(g, &mut (*(o as *mut GCUnion)).cl.c) as lu_mem,
        10 => return traverseproto(g, &mut (*(o as *mut GCUnion)).p) as lu_mem,
        8 => return traversethread(g, &mut (*(o as *mut GCUnion)).th) as lu_mem,
        _ => return 0 as i32 as lu_mem,
    };
}
unsafe extern "C" fn propagateall(mut g: *mut global_State) -> lu_mem {
    let mut tot: lu_mem = 0 as i32 as lu_mem;
    while !((*g).gray).is_null() {
        tot = (tot as libc::c_ulong).wrapping_add(propagatemark(g)) as lu_mem as lu_mem;
    }
    return tot;
}
unsafe extern "C" fn convergeephemerons(mut g: *mut global_State) {
    let mut changed: i32 = 0;
    let mut dir: i32 = 0 as i32;
    loop {
        let mut w: *mut GCObject = 0 as *mut GCObject;
        let mut next: *mut GCObject = (*g).ephemeron;
        (*g).ephemeron = 0 as *mut GCObject;
        changed = 0 as i32;
        loop {
            w = next;
            if w.is_null() {
                break;
            }
            let mut h: *mut Table = &mut (*(w as *mut GCUnion)).h;
            next = (*h).gclist;
            (*h)
                .marked = ((*h).marked as i32
                | (1 as i32) << 5 as i32) as u8;
            if traverseephemeron(g, h, dir) != 0 {
                propagateall(g);
                changed = 1 as i32;
            }
        }
        dir = (dir == 0) as i32;
        if !(changed != 0) {
            break;
        }
    };
}
unsafe extern "C" fn clearbykeys(mut g: *mut global_State, mut l: *mut GCObject) {
    while !l.is_null() {
        let mut h: *mut Table = &mut (*(l as *mut GCUnion)).h;
        let mut limit: *mut Node = &mut *((*h).node)
            .offset(
                ((1 as i32) << (*h).lsizenode as i32) as size_t as isize,
            ) as *mut Node;
        let mut n: *mut Node = 0 as *mut Node;
        n = &mut *((*h).node).offset(0 as i32 as isize) as *mut Node;
        while n < limit {
            if iscleared(
                g,
                if (*n).u.key_tt as i32 & (1 as i32) << 6 as i32
                    != 0
                {
                    (*n).u.key_val.gc
                } else {
                    0 as *mut GCObject
                },
            ) != 0
            {
                (*n)
                    .i_val
                    .tt_ = (0 as i32 | (1 as i32) << 4 as i32)
                    as u8;
            }
            if (*n).i_val.tt_ as i32 & 0xf as i32 == 0 as i32 {
                clearkey(n);
            }
            n = n.offset(1);
            n;
        }
        l = (*(l as *mut GCUnion)).h.gclist;
    }
}
unsafe extern "C" fn clearbyvalues(
    mut g: *mut global_State,
    mut l: *mut GCObject,
    mut f: *mut GCObject,
) {
    while l != f {
        let mut h: *mut Table = &mut (*(l as *mut GCUnion)).h;
        let mut n: *mut Node = 0 as *mut Node;
        let mut limit: *mut Node = &mut *((*h).node)
            .offset(
                ((1 as i32) << (*h).lsizenode as i32) as size_t as isize,
            ) as *mut Node;
        let mut i: libc::c_uint = 0;
        let mut asize: libc::c_uint = luaH_realasize(h);
        i = 0 as i32 as libc::c_uint;
        while i < asize {
            let mut o: *mut TValue = &mut *((*h).array).offset(i as isize)
                as *mut TValue;
            if iscleared(
                g,
                if (*o).tt_ as i32 & (1 as i32) << 6 as i32 != 0
                {
                    (*o).value_.gc
                } else {
                    0 as *mut GCObject
                },
            ) != 0
            {
                (*o)
                    .tt_ = (0 as i32 | (1 as i32) << 4 as i32)
                    as u8;
            }
            i = i.wrapping_add(1);
            i;
        }
        n = &mut *((*h).node).offset(0 as i32 as isize) as *mut Node;
        while n < limit {
            if iscleared(
                g,
                if (*n).i_val.tt_ as i32 & (1 as i32) << 6 as i32
                    != 0
                {
                    (*n).i_val.value_.gc
                } else {
                    0 as *mut GCObject
                },
            ) != 0
            {
                (*n)
                    .i_val
                    .tt_ = (0 as i32 | (1 as i32) << 4 as i32)
                    as u8;
            }
            if (*n).i_val.tt_ as i32 & 0xf as i32 == 0 as i32 {
                clearkey(n);
            }
            n = n.offset(1);
            n;
        }
        l = (*(l as *mut GCUnion)).h.gclist;
    }
}
unsafe extern "C" fn freeupval(mut L: *mut lua_State, mut uv: *mut UpVal) {
    if (*uv).v.p != &mut (*uv).u.value as *mut TValue {
        luaF_unlinkupval(uv);
    }
    luaM_free_(
        L,
        uv as *mut libc::c_void,
        ::core::mem::size_of::<UpVal>() as libc::c_ulong,
    );
}
unsafe extern "C" fn freeobj(mut L: *mut lua_State, mut o: *mut GCObject) {
    match (*o).tt as i32 {
        10 => {
            luaF_freeproto(L, &mut (*(o as *mut GCUnion)).p);
        }
        9 => {
            freeupval(L, &mut (*(o as *mut GCUnion)).upv);
        }
        6 => {
            let mut cl: *mut LClosure = &mut (*(o as *mut GCUnion)).cl.l;
            luaM_free_(
                L,
                cl as *mut libc::c_void,
                (32 as libc::c_ulong as i32
                    + ::core::mem::size_of::<*mut TValue>() as libc::c_ulong
                        as i32 * (*cl).nupvalues as i32) as size_t,
            );
        }
        38 => {
            let mut cl_0: *mut CClosure = &mut (*(o as *mut GCUnion)).cl.c;
            luaM_free_(
                L,
                cl_0 as *mut libc::c_void,
                (32 as libc::c_ulong as i32
                    + ::core::mem::size_of::<TValue>() as libc::c_ulong as i32
                        * (*cl_0).nupvalues as i32) as size_t,
            );
        }
        5 => {
            luaH_free(L, &mut (*(o as *mut GCUnion)).h);
        }
        8 => {
            luaE_freethread(L, &mut (*(o as *mut GCUnion)).th);
        }
        7 => {
            let mut u: *mut Udata = &mut (*(o as *mut GCUnion)).u;
            luaM_free_(
                L,
                o as *mut libc::c_void,
                (if (*u).nuvalue as i32 == 0 as i32 {
                    32 as libc::c_ulong
                } else {
                    (40 as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<UValue>() as libc::c_ulong)
                                .wrapping_mul((*u).nuvalue as libc::c_ulong),
                        )
                })
                    .wrapping_add((*u).len),
            );
        }
        4 => {
            let mut ts: *mut TString = &mut (*(o as *mut GCUnion)).ts;
            luaS_remove(L, ts);
            luaM_free_(
                L,
                ts as *mut libc::c_void,
                (24 as libc::c_ulong)
                    .wrapping_add(
                        (((*ts).shrlen as i32 + 1 as i32)
                            as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                            ),
                    ),
            );
        }
        20 => {
            let mut ts_0: *mut TString = &mut (*(o as *mut GCUnion)).ts;
            luaM_free_(
                L,
                ts_0 as *mut libc::c_void,
                (24 as libc::c_ulong)
                    .wrapping_add(
                        ((*ts_0).u.lnglen)
                            .wrapping_add(1 as i32 as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                            ),
                    ),
            );
        }
        _ => {}
    };
}
unsafe extern "C" fn sweeplist(
    mut L: *mut lua_State,
    mut p: *mut *mut GCObject,
    mut countin: i32,
    mut countout: *mut i32,
) -> *mut *mut GCObject {
    let mut g: *mut global_State = (*L).l_G;
    let mut ow: i32 = (*g).currentwhite as i32
        ^ ((1 as i32) << 3 as i32
            | (1 as i32) << 4 as i32);
    let mut i: i32 = 0;
    let mut white: i32 = ((*g).currentwhite as i32
        & ((1 as i32) << 3 as i32
            | (1 as i32) << 4 as i32)) as u8 as i32;
    i = 0 as i32;
    while !(*p).is_null() && i < countin {
        let mut curr: *mut GCObject = *p;
        let mut marked: i32 = (*curr).marked as i32;
        if marked & ow != 0 {
            *p = (*curr).next;
            freeobj(L, curr);
        } else {
            (*curr)
                .marked = (marked
                & !((1 as i32) << 5 as i32
                    | ((1 as i32) << 3 as i32
                        | (1 as i32) << 4 as i32) | 7 as i32)
                | white) as u8;
            p = &mut (*curr).next;
        }
        i += 1;
        i;
    }
    if !countout.is_null() {
        *countout = i;
    }
    return if (*p).is_null() { 0 as *mut *mut GCObject } else { p };
}
unsafe extern "C" fn sweeptolive(
    mut L: *mut lua_State,
    mut p: *mut *mut GCObject,
) -> *mut *mut GCObject {
    let mut old: *mut *mut GCObject = p;
    loop {
        p = sweeplist(L, p, 1 as i32, 0 as *mut i32);
        if !(p == old) {
            break;
        }
    }
    return p;
}
unsafe extern "C" fn checkSizes(mut L: *mut lua_State, mut g: *mut global_State) {
    if (*g).gcemergency == 0 {
        if (*g).strt.nuse < (*g).strt.size / 4 as i32 {
            let mut olddebt: l_mem = (*g).GCdebt;
            luaS_resize(L, (*g).strt.size / 2 as i32);
            (*g)
                .GCestimate = ((*g).GCestimate as libc::c_ulong)
                .wrapping_add(((*g).GCdebt - olddebt) as libc::c_ulong) as lu_mem
                as lu_mem;
        }
    }
}
unsafe extern "C" fn udata2finalize(mut g: *mut global_State) -> *mut GCObject {
    let mut o: *mut GCObject = (*g).tobefnz;
    (*g).tobefnz = (*o).next;
    (*o).next = (*g).allgc;
    (*g).allgc = o;
    (*o)
        .marked = ((*o).marked as i32
        & !((1 as i32) << 6 as i32) as u8 as i32)
        as u8;
    if 3 as i32 <= (*g).gcstate as i32
        && (*g).gcstate as i32 <= 6 as i32
    {
        (*o)
            .marked = ((*o).marked as i32
            & !((1 as i32) << 5 as i32
                | ((1 as i32) << 3 as i32
                    | (1 as i32) << 4 as i32))
            | ((*g).currentwhite as i32
                & ((1 as i32) << 3 as i32
                    | (1 as i32) << 4 as i32)) as u8 as i32)
            as u8;
    } else if (*o).marked as i32 & 7 as i32 == 3 as i32 {
        (*g).firstold1 = o;
    }
    return o;
}
unsafe extern "C" fn dothecall(mut L: *mut lua_State, mut ud: *mut libc::c_void) {
    luaD_callnoyield(
        L,
        ((*L).top.p).offset(-(2 as i32 as isize)),
        0 as i32,
    );
}
unsafe extern "C" fn GCTM(mut L: *mut lua_State) {
    let mut g: *mut global_State = (*L).l_G;
    let mut tm: *const TValue = 0 as *const TValue;
    let mut v: TValue = TValue {
        value_: Value { gc: 0 as *mut GCObject },
        tt_: 0,
    };
    let mut io: *mut TValue = &mut v;
    let mut i_g: *mut GCObject = udata2finalize(g);
    (*io).value_.gc = i_g;
    (*io)
        .tt_ = ((*i_g).tt as i32 | (1 as i32) << 6 as i32)
        as u8;
    tm = luaT_gettmbyobj(L, &mut v, TM_GC);
    if !((*tm).tt_ as i32 & 0xf as i32 == 0 as i32) {
        let mut status: i32 = 0;
        let mut oldah: u8 = (*L).allowhook;
        let mut oldgcstp: i32 = (*g).gcstp as i32;
        (*g).gcstp = ((*g).gcstp as i32 | 2 as i32) as u8;
        (*L).allowhook = 0 as i32 as u8;
        let fresh0 = (*L).top.p;
        (*L).top.p = ((*L).top.p).offset(1);
        let mut io1: *mut TValue = &mut (*fresh0).val;
        let mut io2: *const TValue = tm;
        (*io1).value_ = (*io2).value_;
        (*io1).tt_ = (*io2).tt_;
        let fresh1 = (*L).top.p;
        (*L).top.p = ((*L).top.p).offset(1);
        let mut io1_0: *mut TValue = &mut (*fresh1).val;
        let mut io2_0: *const TValue = &mut v;
        (*io1_0).value_ = (*io2_0).value_;
        (*io1_0).tt_ = (*io2_0).tt_;
        (*(*L).ci)
            .callstatus = ((*(*L).ci).callstatus as i32
            | (1 as i32) << 7 as i32) as libc::c_ushort;
        status = luaD_pcall(
            L,
            Some(
                dothecall
                    as unsafe extern "C" fn(*mut lua_State, *mut libc::c_void) -> (),
            ),
            0 as *mut libc::c_void,
            (((*L).top.p).offset(-(2 as i32 as isize)) as *mut libc::c_char)
                .offset_from((*L).stack.p as *mut libc::c_char) as libc::c_long,
            0 as i32 as ptrdiff_t,
        );
        (*(*L).ci)
            .callstatus = ((*(*L).ci).callstatus as i32
            & !((1 as i32) << 7 as i32)) as libc::c_ushort;
        (*L).allowhook = oldah;
        (*g).gcstp = oldgcstp as u8;
        if ((status != 0 as i32) as i32 != 0 as i32)
            as i32 as libc::c_long != 0
        {
            luaE_warnerror(L, b"__gc\0" as *const u8 as *const libc::c_char);
            (*L).top.p = ((*L).top.p).offset(-1);
            (*L).top.p;
        }
    }
}
unsafe extern "C" fn runafewfinalizers(
    mut L: *mut lua_State,
    mut n: i32,
) -> i32 {
    let mut g: *mut global_State = (*L).l_G;
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < n && !((*g).tobefnz).is_null() {
        GCTM(L);
        i += 1;
        i;
    }
    return i;
}
unsafe extern "C" fn callallpendingfinalizers(mut L: *mut lua_State) {
    let mut g: *mut global_State = (*L).l_G;
    while !((*g).tobefnz).is_null() {
        GCTM(L);
    }
}
unsafe extern "C" fn findlast(mut p: *mut *mut GCObject) -> *mut *mut GCObject {
    while !(*p).is_null() {
        p = &mut (**p).next;
    }
    return p;
}
unsafe extern "C" fn separatetobefnz(mut g: *mut global_State, mut all: i32) {
    let mut curr: *mut GCObject = 0 as *mut GCObject;
    let mut p: *mut *mut GCObject = &mut (*g).finobj;
    let mut lastnext: *mut *mut GCObject = findlast(&mut (*g).tobefnz);
    loop {
        curr = *p;
        if !(curr != (*g).finobjold1) {
            break;
        }
        if !((*curr).marked as i32
            & ((1 as i32) << 3 as i32
                | (1 as i32) << 4 as i32) != 0 || all != 0)
        {
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
    };
}
unsafe extern "C" fn checkpointer(mut p: *mut *mut GCObject, mut o: *mut GCObject) {
    if o == *p {
        *p = (*o).next;
    }
}
unsafe extern "C" fn correctpointers(mut g: *mut global_State, mut o: *mut GCObject) {
    checkpointer(&mut (*g).survival, o);
    checkpointer(&mut (*g).old1, o);
    checkpointer(&mut (*g).reallyold, o);
    checkpointer(&mut (*g).firstold1, o);
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaC_checkfinalizer(
    mut L: *mut lua_State,
    mut o: *mut GCObject,
    mut mt: *mut Table,
) {
    let mut g: *mut global_State = (*L).l_G;
    if (*o).marked as i32 & (1 as i32) << 6 as i32 != 0
        || (if mt.is_null() {
            0 as *const TValue
        } else {
            (if (*mt).flags as libc::c_uint & (1 as libc::c_uint) << TM_GC as i32
                != 0
            {
                0 as *const TValue
            } else {
                luaT_gettm(mt, TM_GC, (*g).tmname[TM_GC as i32 as usize])
            })
        })
            .is_null() || (*g).gcstp as i32 & 4 as i32 != 0
    {
        return
    } else {
        let mut p: *mut *mut GCObject = 0 as *mut *mut GCObject;
        if 3 as i32 <= (*g).gcstate as i32
            && (*g).gcstate as i32 <= 6 as i32
        {
            (*o)
                .marked = ((*o).marked as i32
                & !((1 as i32) << 5 as i32
                    | ((1 as i32) << 3 as i32
                        | (1 as i32) << 4 as i32))
                | ((*g).currentwhite as i32
                    & ((1 as i32) << 3 as i32
                        | (1 as i32) << 4 as i32)) as u8
                    as i32) as u8;
            if (*g).sweepgc == &mut (*o).next as *mut *mut GCObject {
                (*g).sweepgc = sweeptolive(L, (*g).sweepgc);
            }
        } else {
            correctpointers(g, o);
        }
        p = &mut (*g).allgc;
        while *p != o {
            p = &mut (**p).next;
        }
        *p = (*o).next;
        (*o).next = (*g).finobj;
        (*g).finobj = o;
        (*o)
            .marked = ((*o).marked as i32
            | (1 as i32) << 6 as i32) as u8;
    };
}
unsafe extern "C" fn setpause(mut g: *mut global_State) {
    let mut threshold: l_mem = 0;
    let mut debt: l_mem = 0;
    let mut pause: i32 = (*g).gcpause as i32 * 4 as i32;
    let mut estimate: l_mem = ((*g).GCestimate)
        .wrapping_div(100 as i32 as libc::c_ulong) as l_mem;
    threshold = if (pause as libc::c_long)
        < (!(0 as i32 as lu_mem) >> 1 as i32) as l_mem / estimate
    {
        estimate * pause as libc::c_long
    } else {
        (!(0 as i32 as lu_mem) >> 1 as i32) as l_mem
    };
    debt = (((*g).totalbytes + (*g).GCdebt) as lu_mem)
        .wrapping_sub(threshold as libc::c_ulong) as l_mem;
    if debt > 0 as i32 as libc::c_long {
        debt = 0 as i32 as l_mem;
    }
    luaE_setdebt(g, debt);
}
unsafe extern "C" fn sweep2old(mut L: *mut lua_State, mut p: *mut *mut GCObject) {
    let mut curr: *mut GCObject = 0 as *mut GCObject;
    let mut g: *mut global_State = (*L).l_G;
    loop {
        curr = *p;
        if curr.is_null() {
            break;
        }
        if (*curr).marked as i32
            & ((1 as i32) << 3 as i32
                | (1 as i32) << 4 as i32) != 0
        {
            *p = (*curr).next;
            freeobj(L, curr);
        } else {
            (*curr)
                .marked = ((*curr).marked as i32 & !(7 as i32)
                | 4 as i32) as u8;
            if (*curr).tt as i32
                == 8 as i32 | (0 as i32) << 4 as i32
            {
                let mut th: *mut lua_State = &mut (*(curr as *mut GCUnion)).th;
                linkgclist_(
                    &mut (*(th as *mut GCUnion)).gc,
                    &mut (*th).gclist,
                    &mut (*g).grayagain,
                );
            } else if (*curr).tt as i32
                == 9 as i32 | (0 as i32) << 4 as i32
                && (*(curr as *mut GCUnion)).upv.v.p
                    != &mut (*(curr as *mut GCUnion)).upv.u.value as *mut TValue
            {
                (*curr)
                    .marked = ((*curr).marked as i32
                    & !((1 as i32) << 5 as i32
                        | ((1 as i32) << 3 as i32
                            | (1 as i32) << 4 as i32)) as u8
                        as i32) as u8;
            } else {
                (*curr)
                    .marked = ((*curr).marked as i32
                    | (1 as i32) << 5 as i32) as u8;
            }
            p = &mut (*curr).next;
        }
    };
}
unsafe extern "C" fn sweepgen(
    mut L: *mut lua_State,
    mut g: *mut global_State,
    mut p: *mut *mut GCObject,
    mut limit: *mut GCObject,
    mut pfirstold1: *mut *mut GCObject,
) -> *mut *mut GCObject {
    static mut nextage: [u8; 7] = [
        1 as i32 as u8,
        3 as i32 as u8,
        3 as i32 as u8,
        4 as i32 as u8,
        4 as i32 as u8,
        5 as i32 as u8,
        6 as i32 as u8,
    ];
    let mut white: i32 = ((*g).currentwhite as i32
        & ((1 as i32) << 3 as i32
            | (1 as i32) << 4 as i32)) as u8 as i32;
    let mut curr: *mut GCObject = 0 as *mut GCObject;
    loop {
        curr = *p;
        if !(curr != limit) {
            break;
        }
        if (*curr).marked as i32
            & ((1 as i32) << 3 as i32
                | (1 as i32) << 4 as i32) != 0
        {
            *p = (*curr).next;
            freeobj(L, curr);
        } else {
            if (*curr).marked as i32 & 7 as i32 == 0 as i32 {
                let mut marked: i32 = (*curr).marked as i32
                    & !((1 as i32) << 5 as i32
                        | ((1 as i32) << 3 as i32
                            | (1 as i32) << 4 as i32)
                        | 7 as i32);
                (*curr).marked = (marked | 1 as i32 | white) as u8;
            } else {
                (*curr)
                    .marked = ((*curr).marked as i32 & !(7 as i32)
                    | nextage[((*curr).marked as i32 & 7 as i32)
                        as usize] as i32) as u8;
                if (*curr).marked as i32 & 7 as i32 == 3 as i32
                    && (*pfirstold1).is_null()
                {
                    *pfirstold1 = curr;
                }
            }
            p = &mut (*curr).next;
        }
    }
    return p;
}
unsafe extern "C" fn whitelist(mut g: *mut global_State, mut p: *mut GCObject) {
    let mut white: i32 = ((*g).currentwhite as i32
        & ((1 as i32) << 3 as i32
            | (1 as i32) << 4 as i32)) as u8 as i32;
    while !p.is_null() {
        (*p)
            .marked = ((*p).marked as i32
            & !((1 as i32) << 5 as i32
                | ((1 as i32) << 3 as i32
                    | (1 as i32) << 4 as i32) | 7 as i32)
            | white) as u8;
        p = (*p).next;
    }
}
unsafe extern "C" fn correctgraylist(mut p: *mut *mut GCObject) -> *mut *mut GCObject {
    let mut current_block: u64;
    let mut curr: *mut GCObject = 0 as *mut GCObject;
    loop {
        curr = *p;
        if curr.is_null() {
            break;
        }
        let mut next: *mut *mut GCObject = getgclist(curr);
        if !((*curr).marked as i32
            & ((1 as i32) << 3 as i32
                | (1 as i32) << 4 as i32) != 0)
        {
            if (*curr).marked as i32 & 7 as i32 == 5 as i32 {
                (*curr)
                    .marked = ((*curr).marked as i32
                    | (1 as i32) << 5 as i32) as u8;
                (*curr)
                    .marked = ((*curr).marked as i32
                    ^ (5 as i32 ^ 6 as i32)) as u8;
                current_block = 11386734783587865205;
            } else if (*curr).tt as i32
                == 8 as i32 | (0 as i32) << 4 as i32
            {
                current_block = 11386734783587865205;
            } else {
                if (*curr).marked as i32 & 7 as i32 == 6 as i32 {
                    (*curr)
                        .marked = ((*curr).marked as i32
                        ^ (6 as i32 ^ 4 as i32)) as u8;
                }
                (*curr)
                    .marked = ((*curr).marked as i32
                    | (1 as i32) << 5 as i32) as u8;
                current_block = 11052768930202291551;
            }
            match current_block {
                11052768930202291551 => {}
                _ => {
                    p = next;
                    continue;
                }
            }
        }
        *p = *next;
    }
    return p;
}
unsafe extern "C" fn correctgraylists(mut g: *mut global_State) {
    let mut list: *mut *mut GCObject = correctgraylist(&mut (*g).grayagain);
    *list = (*g).weak;
    (*g).weak = 0 as *mut GCObject;
    list = correctgraylist(list);
    *list = (*g).allweak;
    (*g).allweak = 0 as *mut GCObject;
    list = correctgraylist(list);
    *list = (*g).ephemeron;
    (*g).ephemeron = 0 as *mut GCObject;
    correctgraylist(list);
}
unsafe extern "C" fn markold(
    mut g: *mut global_State,
    mut from: *mut GCObject,
    mut to: *mut GCObject,
) {
    let mut p: *mut GCObject = 0 as *mut GCObject;
    p = from;
    while p != to {
        if (*p).marked as i32 & 7 as i32 == 3 as i32 {
            (*p)
                .marked = ((*p).marked as i32
                ^ (3 as i32 ^ 4 as i32)) as u8;
            if (*p).marked as i32 & (1 as i32) << 5 as i32 != 0 {
                reallymarkobject(g, p);
            }
        }
        p = (*p).next;
    }
}
unsafe extern "C" fn finishgencycle(mut L: *mut lua_State, mut g: *mut global_State) {
    correctgraylists(g);
    checkSizes(L, g);
    (*g).gcstate = 0 as i32 as u8;
    if (*g).gcemergency == 0 {
        callallpendingfinalizers(L);
    }
}
unsafe extern "C" fn youngcollection(mut L: *mut lua_State, mut g: *mut global_State) {
    let mut psurvival: *mut *mut GCObject = 0 as *mut *mut GCObject;
    let mut dummy: *mut GCObject = 0 as *mut GCObject;
    if !((*g).firstold1).is_null() {
        markold(g, (*g).firstold1, (*g).reallyold);
        (*g).firstold1 = 0 as *mut GCObject;
    }
    markold(g, (*g).finobj, (*g).finobjrold);
    markold(g, (*g).tobefnz, 0 as *mut GCObject);
    atomic(L);
    (*g).gcstate = 3 as i32 as u8;
    psurvival = sweepgen(L, g, &mut (*g).allgc, (*g).survival, &mut (*g).firstold1);
    sweepgen(L, g, psurvival, (*g).old1, &mut (*g).firstold1);
    (*g).reallyold = (*g).old1;
    (*g).old1 = *psurvival;
    (*g).survival = (*g).allgc;
    dummy = 0 as *mut GCObject;
    psurvival = sweepgen(L, g, &mut (*g).finobj, (*g).finobjsur, &mut dummy);
    sweepgen(L, g, psurvival, (*g).finobjold1, &mut dummy);
    (*g).finobjrold = (*g).finobjold1;
    (*g).finobjold1 = *psurvival;
    (*g).finobjsur = (*g).finobj;
    sweepgen(L, g, &mut (*g).tobefnz, 0 as *mut GCObject, &mut dummy);
    finishgencycle(L, g);
}
unsafe extern "C" fn atomic2gen(mut L: *mut lua_State, mut g: *mut global_State) {
    cleargraylists(g);
    (*g).gcstate = 3 as i32 as u8;
    sweep2old(L, &mut (*g).allgc);
    (*g).survival = (*g).allgc;
    (*g).old1 = (*g).survival;
    (*g).reallyold = (*g).old1;
    (*g).firstold1 = 0 as *mut GCObject;
    sweep2old(L, &mut (*g).finobj);
    (*g).finobjsur = (*g).finobj;
    (*g).finobjold1 = (*g).finobjsur;
    (*g).finobjrold = (*g).finobjold1;
    sweep2old(L, &mut (*g).tobefnz);
    (*g).gckind = 1 as i32 as u8;
    (*g).lastatomic = 0 as i32 as lu_mem;
    (*g).GCestimate = ((*g).totalbytes + (*g).GCdebt) as lu_mem;
    finishgencycle(L, g);
}
unsafe extern "C" fn setminordebt(mut g: *mut global_State) {
    luaE_setdebt(
        g,
        -((((*g).totalbytes + (*g).GCdebt) as lu_mem)
            .wrapping_div(100 as i32 as libc::c_ulong) as l_mem
            * (*g).genminormul as libc::c_long),
    );
}
unsafe extern "C" fn entergen(
    mut L: *mut lua_State,
    mut g: *mut global_State,
) -> lu_mem {
    let mut numobjs: lu_mem = 0;
    luaC_runtilstate(L, (1 as i32) << 8 as i32);
    luaC_runtilstate(L, (1 as i32) << 0 as i32);
    numobjs = atomic(L);
    atomic2gen(L, g);
    setminordebt(g);
    return numobjs;
}
unsafe extern "C" fn enterinc(mut g: *mut global_State) {
    whitelist(g, (*g).allgc);
    (*g).survival = 0 as *mut GCObject;
    (*g).old1 = (*g).survival;
    (*g).reallyold = (*g).old1;
    whitelist(g, (*g).finobj);
    whitelist(g, (*g).tobefnz);
    (*g).finobjsur = 0 as *mut GCObject;
    (*g).finobjold1 = (*g).finobjsur;
    (*g).finobjrold = (*g).finobjold1;
    (*g).gcstate = 8 as i32 as u8;
    (*g).gckind = 0 as i32 as u8;
    (*g).lastatomic = 0 as i32 as lu_mem;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaC_changemode(
    mut L: *mut lua_State,
    mut newmode: i32,
) {
    let mut g: *mut global_State = (*L).l_G;
    if newmode != (*g).gckind as i32 {
        if newmode == 1 as i32 {
            entergen(L, g);
        } else {
            enterinc(g);
        }
    }
    (*g).lastatomic = 0 as i32 as lu_mem;
}
unsafe extern "C" fn fullgen(mut L: *mut lua_State, mut g: *mut global_State) -> lu_mem {
    enterinc(g);
    return entergen(L, g);
}
unsafe extern "C" fn stepgenfull(mut L: *mut lua_State, mut g: *mut global_State) {
    let mut newatomic: lu_mem = 0;
    let mut lastatomic: lu_mem = (*g).lastatomic;
    if (*g).gckind as i32 == 1 as i32 {
        enterinc(g);
    }
    luaC_runtilstate(L, (1 as i32) << 0 as i32);
    newatomic = atomic(L);
    if newatomic < lastatomic.wrapping_add(lastatomic >> 3 as i32) {
        atomic2gen(L, g);
        setminordebt(g);
    } else {
        (*g).GCestimate = ((*g).totalbytes + (*g).GCdebt) as lu_mem;
        entersweep(L);
        luaC_runtilstate(L, (1 as i32) << 8 as i32);
        setpause(g);
        (*g).lastatomic = newatomic;
    };
}
unsafe extern "C" fn genstep(mut L: *mut lua_State, mut g: *mut global_State) {
    if (*g).lastatomic != 0 as i32 as libc::c_ulong {
        stepgenfull(L, g);
    } else {
        let mut majorbase: lu_mem = (*g).GCestimate;
        let mut majorinc: lu_mem = majorbase
            .wrapping_div(100 as i32 as libc::c_ulong)
            .wrapping_mul(
                ((*g).genmajormul as i32 * 4 as i32) as libc::c_ulong,
            );
        if (*g).GCdebt > 0 as i32 as libc::c_long
            && ((*g).totalbytes + (*g).GCdebt) as lu_mem
                > majorbase.wrapping_add(majorinc)
        {
            let mut numobjs: lu_mem = fullgen(L, g);
            if !((((*g).totalbytes + (*g).GCdebt) as lu_mem)
                < majorbase
                    .wrapping_add(
                        majorinc.wrapping_div(2 as i32 as libc::c_ulong),
                    ))
            {
                (*g).lastatomic = numobjs;
                setpause(g);
            }
        } else {
            youngcollection(L, g);
            setminordebt(g);
            (*g).GCestimate = majorbase;
        }
    };
}
unsafe extern "C" fn entersweep(mut L: *mut lua_State) {
    let mut g: *mut global_State = (*L).l_G;
    (*g).gcstate = 3 as i32 as u8;
    (*g).sweepgc = sweeptolive(L, &mut (*g).allgc);
}
unsafe extern "C" fn deletelist(
    mut L: *mut lua_State,
    mut p: *mut GCObject,
    mut limit: *mut GCObject,
) {
    while p != limit {
        let mut next: *mut GCObject = (*p).next;
        freeobj(L, p);
        p = next;
    }
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaC_freeallobjects(mut L: *mut lua_State) {
    let mut g: *mut global_State = (*L).l_G;
    (*g).gcstp = 4 as i32 as u8;
    luaC_changemode(L, 0 as i32);
    separatetobefnz(g, 1 as i32);
    callallpendingfinalizers(L);
    deletelist(L, (*g).allgc, &mut (*((*g).mainthread as *mut GCUnion)).gc);
    deletelist(L, (*g).fixedgc, 0 as *mut GCObject);
}
unsafe extern "C" fn atomic(mut L: *mut lua_State) -> lu_mem {
    let mut g: *mut global_State = (*L).l_G;
    let mut work: lu_mem = 0 as i32 as lu_mem;
    let mut origweak: *mut GCObject = 0 as *mut GCObject;
    let mut origall: *mut GCObject = 0 as *mut GCObject;
    let mut grayagain: *mut GCObject = (*g).grayagain;
    (*g).grayagain = 0 as *mut GCObject;
    (*g).gcstate = 2 as i32 as u8;
    if (*L).marked as i32
        & ((1 as i32) << 3 as i32
            | (1 as i32) << 4 as i32) != 0
    {
        reallymarkobject(g, &mut (*(L as *mut GCUnion)).gc);
    }
    if (*g).l_registry.tt_ as i32 & (1 as i32) << 6 as i32 != 0
        && (*(*g).l_registry.value_.gc).marked as i32
            & ((1 as i32) << 3 as i32
                | (1 as i32) << 4 as i32) != 0
    {
        reallymarkobject(g, (*g).l_registry.value_.gc);
    }
    markmt(g);
    work = (work as libc::c_ulong).wrapping_add(propagateall(g)) as lu_mem as lu_mem;
    work = (work as libc::c_ulong).wrapping_add(remarkupvals(g) as libc::c_ulong)
        as lu_mem as lu_mem;
    work = (work as libc::c_ulong).wrapping_add(propagateall(g)) as lu_mem as lu_mem;
    (*g).gray = grayagain;
    work = (work as libc::c_ulong).wrapping_add(propagateall(g)) as lu_mem as lu_mem;
    convergeephemerons(g);
    clearbyvalues(g, (*g).weak, 0 as *mut GCObject);
    clearbyvalues(g, (*g).allweak, 0 as *mut GCObject);
    origweak = (*g).weak;
    origall = (*g).allweak;
    separatetobefnz(g, 0 as i32);
    work = (work as libc::c_ulong).wrapping_add(markbeingfnz(g)) as lu_mem as lu_mem;
    work = (work as libc::c_ulong).wrapping_add(propagateall(g)) as lu_mem as lu_mem;
    convergeephemerons(g);
    clearbykeys(g, (*g).ephemeron);
    clearbykeys(g, (*g).allweak);
    clearbyvalues(g, (*g).weak, origweak);
    clearbyvalues(g, (*g).allweak, origall);
    luaS_clearcache(g);
    (*g)
        .currentwhite = ((*g).currentwhite as i32
        ^ ((1 as i32) << 3 as i32
            | (1 as i32) << 4 as i32)) as u8;
    return work;
}
unsafe extern "C" fn sweepstep(
    mut L: *mut lua_State,
    mut g: *mut global_State,
    mut nextstate: i32,
    mut nextlist: *mut *mut GCObject,
) -> i32 {
    if !((*g).sweepgc).is_null() {
        let mut olddebt: l_mem = (*g).GCdebt;
        let mut count: i32 = 0;
        (*g).sweepgc = sweeplist(L, (*g).sweepgc, 100 as i32, &mut count);
        (*g)
            .GCestimate = ((*g).GCestimate as libc::c_ulong)
            .wrapping_add(((*g).GCdebt - olddebt) as libc::c_ulong) as lu_mem as lu_mem;
        return count;
    } else {
        (*g).gcstate = nextstate as u8;
        (*g).sweepgc = nextlist;
        return 0 as i32;
    };
}
unsafe extern "C" fn singlestep(mut L: *mut lua_State) -> lu_mem {
    let mut g: *mut global_State = (*L).l_G;
    let mut work: lu_mem = 0;
    (*g).gcstopem = 1 as i32 as u8;
    match (*g).gcstate as i32 {
        8 => {
            restartcollection(g);
            (*g).gcstate = 0 as i32 as u8;
            work = 1 as i32 as lu_mem;
        }
        0 => {
            if ((*g).gray).is_null() {
                (*g).gcstate = 1 as i32 as u8;
                work = 0 as i32 as lu_mem;
            } else {
                work = propagatemark(g);
            }
        }
        1 => {
            work = atomic(L);
            entersweep(L);
            (*g).GCestimate = ((*g).totalbytes + (*g).GCdebt) as lu_mem;
        }
        3 => {
            work = sweepstep(L, g, 4 as i32, &mut (*g).finobj) as lu_mem;
        }
        4 => {
            work = sweepstep(L, g, 5 as i32, &mut (*g).tobefnz) as lu_mem;
        }
        5 => {
            work = sweepstep(L, g, 6 as i32, 0 as *mut *mut GCObject) as lu_mem;
        }
        6 => {
            checkSizes(L, g);
            (*g).gcstate = 7 as i32 as u8;
            work = 0 as i32 as lu_mem;
        }
        7 => {
            if !((*g).tobefnz).is_null() && (*g).gcemergency == 0 {
                (*g).gcstopem = 0 as i32 as u8;
                work = (runafewfinalizers(L, 10 as i32) * 50 as i32)
                    as lu_mem;
            } else {
                (*g).gcstate = 8 as i32 as u8;
                work = 0 as i32 as lu_mem;
            }
        }
        _ => return 0 as i32 as lu_mem,
    }
    (*g).gcstopem = 0 as i32 as u8;
    return work;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaC_runtilstate(
    mut L: *mut lua_State,
    mut statesmask: i32,
) {
    let mut g: *mut global_State = (*L).l_G;
    while statesmask & (1 as i32) << (*g).gcstate as i32 == 0 {
        singlestep(L);
    }
}
unsafe extern "C" fn incstep(mut L: *mut lua_State, mut g: *mut global_State) {
    let mut stepmul: i32 = (*g).gcstepmul as i32 * 4 as i32
        | 1 as i32;
    let mut debt: l_mem = ((*g).GCdebt as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<TValue>() as libc::c_ulong)
        .wrapping_mul(stepmul as libc::c_ulong) as l_mem;
    let mut stepsize: l_mem = (if (*g).gcstepsize as libc::c_ulong
        <= (::core::mem::size_of::<l_mem>() as libc::c_ulong)
            .wrapping_mul(8 as i32 as libc::c_ulong)
            .wrapping_sub(2 as i32 as libc::c_ulong)
    {
        (((1 as i32 as l_mem) << (*g).gcstepsize as i32)
            as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<TValue>() as libc::c_ulong)
            .wrapping_mul(stepmul as libc::c_ulong)
    } else {
        (!(0 as i32 as lu_mem) >> 1 as i32) as l_mem as libc::c_ulong
    }) as l_mem;
    loop {
        let mut work: lu_mem = singlestep(L);
        debt = (debt as libc::c_ulong).wrapping_sub(work) as l_mem as l_mem;
        if !(debt > -stepsize && (*g).gcstate as i32 != 8 as i32) {
            break;
        }
    }
    if (*g).gcstate as i32 == 8 as i32 {
        setpause(g);
    } else {
        debt = ((debt / stepmul as libc::c_long) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<TValue>() as libc::c_ulong) as l_mem;
        luaE_setdebt(g, debt);
    };
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaC_step(mut L: *mut lua_State) {
    let mut g: *mut global_State = (*L).l_G;
    if !((*g).gcstp as i32 == 0 as i32) {
        luaE_setdebt(g, -(2000 as i32) as l_mem);
    } else if (*g).gckind as i32 == 1 as i32
        || (*g).lastatomic != 0 as i32 as libc::c_ulong
    {
        genstep(L, g);
    } else {
        incstep(L, g);
    };
}
unsafe extern "C" fn fullinc(mut L: *mut lua_State, mut g: *mut global_State) {
    if (*g).gcstate as i32 <= 2 as i32 {
        entersweep(L);
    }
    luaC_runtilstate(L, (1 as i32) << 8 as i32);
    luaC_runtilstate(L, (1 as i32) << 0 as i32);
    (*g).gcstate = 1 as i32 as u8;
    luaC_runtilstate(L, (1 as i32) << 7 as i32);
    luaC_runtilstate(L, (1 as i32) << 8 as i32);
    setpause(g);
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaC_fullgc(
    mut L: *mut lua_State,
    mut isemergency: i32,
) {
    let mut g: *mut global_State = (*L).l_G;
    (*g).gcemergency = isemergency as u8;
    if (*g).gckind as i32 == 0 as i32 {
        fullinc(L, g);
    } else {
        fullgen(L, g);
    }
    (*g).gcemergency = 0 as i32 as u8;
}
