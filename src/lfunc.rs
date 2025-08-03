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
extern "C" {
    pub type lua_longjmp;
    fn luaT_gettmbyobj(L: *mut lua_State, o: *const TValue, event: TMS) -> *const TValue;
    fn luaM_free_(L: *mut lua_State, block: *mut libc::c_void, osize: size_t);
    fn luaG_findlocal(
        L: *mut lua_State,
        ci: *mut CallInfo,
        n: libc::c_int,
        pos: *mut StkId,
    ) -> *const libc::c_char;
    fn luaG_runerror(L: *mut lua_State, fmt: *const libc::c_char, _: ...) -> !;
    fn luaD_seterrorobj(L: *mut lua_State, errcode: libc::c_int, oldtop: StkId);
    fn luaD_call(L: *mut lua_State, func: StkId, nResults: libc::c_int);
    fn luaD_callnoyield(L: *mut lua_State, func: StkId, nResults: libc::c_int);
    fn luaC_newobj(L: *mut lua_State, tt: libc::c_int, sz: size_t) -> *mut GCObject;
    fn luaC_barrier_(L: *mut lua_State, o: *mut GCObject, v: *mut GCObject);
}
pub type ptrdiff_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type __sig_atomic_t = libc::c_int;
pub type intptr_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lua_State {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub status: lu_byte,
    pub allowhook: lu_byte,
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
    pub oldpc: libc::c_int,
    pub basehookcount: libc::c_int,
    pub hookcount: libc::c_int,
    pub hookmask: sig_atomic_t,
}
pub type sig_atomic_t = __sig_atomic_t;
pub type l_uint32 = libc::c_uint;
pub type lua_Hook = Option::<unsafe extern "C" fn(*mut lua_State, *mut lua_Debug) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lua_Debug {
    pub event: libc::c_int,
    pub name: *const libc::c_char,
    pub namewhat: *const libc::c_char,
    pub what: *const libc::c_char,
    pub source: *const libc::c_char,
    pub srclen: size_t,
    pub currentline: libc::c_int,
    pub linedefined: libc::c_int,
    pub lastlinedefined: libc::c_int,
    pub nups: libc::c_uchar,
    pub nparams: libc::c_uchar,
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
    pub funcidx: libc::c_int,
    pub nyield: libc::c_int,
    pub nres: libc::c_int,
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
    unsafe extern "C" fn(*mut lua_State, libc::c_int, lua_KContext) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub savedpc: *const Instruction,
    pub trap: sig_atomic_t,
    pub nextraargs: libc::c_int,
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
    pub tt_: lu_byte,
    pub delta: libc::c_ushort,
}
pub type lu_byte = libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub union Value {
    pub gc: *mut GCObject,
    pub p: *mut libc::c_void,
    pub f: lua_CFunction,
    pub i: lua_Integer,
    pub n: lua_Number,
    pub ub: lu_byte,
}
pub type lua_Number = libc::c_double;
pub type lua_Integer = libc::c_longlong;
pub type lua_CFunction = Option::<unsafe extern "C" fn(*mut lua_State) -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GCObject {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TValue {
    pub value_: Value,
    pub tt_: lu_byte,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UpVal {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
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
    pub currentwhite: lu_byte,
    pub gcstate: lu_byte,
    pub gckind: lu_byte,
    pub gcstopem: lu_byte,
    pub genminormul: lu_byte,
    pub genmajormul: lu_byte,
    pub gcstp: lu_byte,
    pub gcemergency: lu_byte,
    pub gcpause: lu_byte,
    pub gcstepmul: lu_byte,
    pub gcstepsize: lu_byte,
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
    pub panic: lua_CFunction,
    pub mainthread: *mut lua_State,
    pub memerrmsg: *mut TString,
    pub tmname: [*mut TString; 25],
    pub mt: [*mut Table; 9],
    pub strcache: [[*mut TString; 2]; 53],
    pub warnf: lua_WarnFunction,
    pub ud_warn: *mut libc::c_void,
}
pub type lua_WarnFunction = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, libc::c_int) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TString {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub extra: lu_byte,
    pub shrlen: lu_byte,
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
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub flags: lu_byte,
    pub lsizenode: lu_byte,
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
    pub tt_: lu_byte,
    pub key_tt: lu_byte,
    pub next: libc::c_int,
    pub key_val: Value,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stringtable {
    pub hash: *mut *mut TString,
    pub nuse: libc::c_int,
    pub size: libc::c_int,
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
    pub n: lua_Number,
    pub u: libc::c_double,
    pub s: *mut libc::c_void,
    pub i: lua_Integer,
    pub l: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Udata {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
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
    pub instack: lu_byte,
    pub idx: lu_byte,
    pub kind: lu_byte,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LocVar {
    pub varname: *mut TString,
    pub startpc: libc::c_int,
    pub endpc: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AbsLineInfo {
    pub pc: libc::c_int,
    pub line: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Proto {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub numparams: lu_byte,
    pub is_vararg: lu_byte,
    pub maxstacksize: lu_byte,
    pub sizeupvalues: libc::c_int,
    pub sizek: libc::c_int,
    pub sizecode: libc::c_int,
    pub sizelineinfo: libc::c_int,
    pub sizep: libc::c_int,
    pub sizelocvars: libc::c_int,
    pub sizeabslineinfo: libc::c_int,
    pub linedefined: libc::c_int,
    pub lastlinedefined: libc::c_int,
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
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub nupvalues: lu_byte,
    pub gclist: *mut GCObject,
    pub f: lua_CFunction,
    pub upvalue: [TValue; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LClosure {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub nupvalues: lu_byte,
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
#[no_mangle]
pub unsafe extern "C" fn luaF_newCclosure(
    mut L: *mut lua_State,
    mut nupvals: libc::c_int,
) -> *mut CClosure {
    let mut o: *mut GCObject = luaC_newobj(
        L,
        6 as libc::c_int | (2 as libc::c_int) << 4 as libc::c_int,
        (32 as libc::c_ulong as libc::c_int
            + ::core::mem::size_of::<TValue>() as libc::c_ulong as libc::c_int * nupvals)
            as size_t,
    );
    let mut c: *mut CClosure = &mut (*(o as *mut GCUnion)).cl.c;
    (*c).nupvalues = nupvals as lu_byte;
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn luaF_newLclosure(
    mut L: *mut lua_State,
    mut nupvals: libc::c_int,
) -> *mut LClosure {
    let mut o: *mut GCObject = luaC_newobj(
        L,
        6 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int,
        (32 as libc::c_ulong as libc::c_int
            + ::core::mem::size_of::<*mut TValue>() as libc::c_ulong as libc::c_int
                * nupvals) as size_t,
    );
    let mut c: *mut LClosure = &mut (*(o as *mut GCUnion)).cl.l;
    (*c).p = 0 as *mut Proto;
    (*c).nupvalues = nupvals as lu_byte;
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
#[no_mangle]
pub unsafe extern "C" fn luaF_initupvals(mut L: *mut lua_State, mut cl: *mut LClosure) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*cl).nupvalues as libc::c_int {
        let mut o: *mut GCObject = luaC_newobj(
            L,
            9 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int,
            ::core::mem::size_of::<UpVal>() as libc::c_ulong,
        );
        let mut uv: *mut UpVal = &mut (*(o as *mut GCUnion)).upv;
        (*uv).v.p = &mut (*uv).u.value;
        (*(*uv).v.p)
            .tt_ = (0 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int)
            as lu_byte;
        let ref mut fresh2 = *((*cl).upvals).as_mut_ptr().offset(i as isize);
        *fresh2 = uv;
        if (*cl).marked as libc::c_int & (1 as libc::c_int) << 5 as libc::c_int != 0
            && (*uv).marked as libc::c_int
                & ((1 as libc::c_int) << 3 as libc::c_int
                    | (1 as libc::c_int) << 4 as libc::c_int) != 0
        {
            luaC_barrier_(
                L,
                &mut (*(cl as *mut GCUnion)).gc,
                &mut (*(uv as *mut GCUnion)).gc,
            );
        } else {};
        i += 1;
        i;
    }
}
unsafe extern "C" fn newupval(
    mut L: *mut lua_State,
    mut level: StkId,
    mut prev: *mut *mut UpVal,
) -> *mut UpVal {
    let mut o: *mut GCObject = luaC_newobj(
        L,
        9 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int,
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
#[no_mangle]
pub unsafe extern "C" fn luaF_findupval(
    mut L: *mut lua_State,
    mut level: StkId,
) -> *mut UpVal {
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
    mut yy: libc::c_int,
) {
    let mut top: StkId = (*L).top.p;
    let mut tm: *const TValue = luaT_gettmbyobj(L, obj, TM_CLOSE);
    let mut io1: *mut TValue = &mut (*top).val;
    let mut io2: *const TValue = tm;
    (*io1).value_ = (*io2).value_;
    (*io1).tt_ = (*io2).tt_;
    let mut io1_0: *mut TValue = &mut (*top.offset(1 as libc::c_int as isize)).val;
    let mut io2_0: *const TValue = obj;
    (*io1_0).value_ = (*io2_0).value_;
    (*io1_0).tt_ = (*io2_0).tt_;
    let mut io1_1: *mut TValue = &mut (*top.offset(2 as libc::c_int as isize)).val;
    let mut io2_1: *const TValue = err;
    (*io1_1).value_ = (*io2_1).value_;
    (*io1_1).tt_ = (*io2_1).tt_;
    (*L).top.p = top.offset(3 as libc::c_int as isize);
    if yy != 0 {
        luaD_call(L, top, 0 as libc::c_int);
    } else {
        luaD_callnoyield(L, top, 0 as libc::c_int);
    };
}
unsafe extern "C" fn checkclosemth(mut L: *mut lua_State, mut level: StkId) {
    let mut tm: *const TValue = luaT_gettmbyobj(L, &mut (*level).val, TM_CLOSE);
    if (*tm).tt_ as libc::c_int & 0xf as libc::c_int == 0 as libc::c_int {
        let mut idx: libc::c_int = level.offset_from((*(*L).ci).func.p) as libc::c_long
            as libc::c_int;
        let mut vname: *const libc::c_char = luaG_findlocal(
            L,
            (*L).ci,
            idx,
            0 as *mut StkId,
        );
        if vname.is_null() {
            vname = b"?\0" as *const u8 as *const libc::c_char;
        }
        luaG_runerror(
            L,
            b"variable '%s' got a non-closable value\0" as *const u8
                as *const libc::c_char,
            vname,
        );
    }
}
unsafe extern "C" fn prepcallclosemth(
    mut L: *mut lua_State,
    mut level: StkId,
    mut status: libc::c_int,
    mut yy: libc::c_int,
) {
    let mut uv: *mut TValue = &mut (*level).val;
    let mut errobj: *mut TValue = 0 as *mut TValue;
    if status == -(1 as libc::c_int) {
        errobj = &mut (*(*L).l_G).nilvalue;
    } else {
        errobj = &mut (*level.offset(1 as libc::c_int as isize)).val;
        luaD_seterrorobj(L, status, level.offset(1 as libc::c_int as isize));
    }
    callclosemethod(L, uv, errobj, yy);
}
#[no_mangle]
pub unsafe extern "C" fn luaF_newtbcupval(mut L: *mut lua_State, mut level: StkId) {
    if (*level).val.tt_ as libc::c_int
        == 1 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        || (*level).val.tt_ as libc::c_int & 0xf as libc::c_int == 0 as libc::c_int
    {
        return;
    }
    checkclosemth(L, level);
    while level.offset_from((*L).tbclist.p) as libc::c_long as libc::c_uint
        as libc::c_ulong
        > ((256 as libc::c_ulong)
            << (::core::mem::size_of::<libc::c_ushort>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong))
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {
        (*L)
            .tbclist
            .p = ((*L).tbclist.p)
            .offset(
                ((256 as libc::c_ulong)
                    << (::core::mem::size_of::<libc::c_ushort>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong))
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            );
        (*(*L).tbclist.p).tbclist.delta = 0 as libc::c_int as libc::c_ushort;
    }
    (*level)
        .tbclist
        .delta = level.offset_from((*L).tbclist.p) as libc::c_long as libc::c_ushort;
    (*L).tbclist.p = level;
}
#[no_mangle]
pub unsafe extern "C" fn luaF_unlinkupval(mut uv: *mut UpVal) {
    *(*uv).u.open.previous = (*uv).u.open.next;
    if !((*uv).u.open.next).is_null() {
        (*(*uv).u.open.next).u.open.previous = (*uv).u.open.previous;
    }
}
#[no_mangle]
pub unsafe extern "C" fn luaF_closeupval(mut L: *mut lua_State, mut level: StkId) {
    let mut uv: *mut UpVal = 0 as *mut UpVal;
    let mut upl: StkId = 0 as *mut StackValue;
    loop {
        uv = (*L).openupval;
        if !(!uv.is_null()
            && {
                upl = (*uv).v.p as StkId;
                upl >= level
            })
        {
            break;
        }
        let mut slot: *mut TValue = &mut (*uv).u.value;
        luaF_unlinkupval(uv);
        let mut io1: *mut TValue = slot;
        let mut io2: *const TValue = (*uv).v.p;
        (*io1).value_ = (*io2).value_;
        (*io1).tt_ = (*io2).tt_;
        (*uv).v.p = slot;
        if (*uv).marked as libc::c_int
            & ((1 as libc::c_int) << 3 as libc::c_int
                | (1 as libc::c_int) << 4 as libc::c_int) == 0
        {
            (*uv)
                .marked = ((*uv).marked as libc::c_int
                | (1 as libc::c_int) << 5 as libc::c_int) as lu_byte;
            if (*slot).tt_ as libc::c_int & (1 as libc::c_int) << 6 as libc::c_int != 0 {
                if (*uv).marked as libc::c_int & (1 as libc::c_int) << 5 as libc::c_int
                    != 0
                    && (*(*slot).value_.gc).marked as libc::c_int
                        & ((1 as libc::c_int) << 3 as libc::c_int
                            | (1 as libc::c_int) << 4 as libc::c_int) != 0
                {
                    luaC_barrier_(
                        L,
                        &mut (*(uv as *mut GCUnion)).gc,
                        &mut (*((*slot).value_.gc as *mut GCUnion)).gc,
                    );
                } else {};
            } else {};
        }
    };
}
unsafe extern "C" fn poptbclist(mut L: *mut lua_State) {
    let mut tbc: StkId = (*L).tbclist.p;
    tbc = tbc.offset(-((*tbc).tbclist.delta as libc::c_int as isize));
    while tbc > (*L).stack.p && (*tbc).tbclist.delta as libc::c_int == 0 as libc::c_int {
        tbc = tbc
            .offset(
                -(((256 as libc::c_ulong)
                    << (::core::mem::size_of::<libc::c_ushort>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong))
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize),
            );
    }
    (*L).tbclist.p = tbc;
}
#[no_mangle]
pub unsafe extern "C" fn luaF_close(
    mut L: *mut lua_State,
    mut level: StkId,
    mut status: libc::c_int,
    mut yy: libc::c_int,
) -> StkId {
    let mut levelrel: ptrdiff_t = (level as *mut libc::c_char)
        .offset_from((*L).stack.p as *mut libc::c_char) as libc::c_long;
    luaF_closeupval(L, level);
    while (*L).tbclist.p >= level {
        let mut tbc: StkId = (*L).tbclist.p;
        poptbclist(L);
        prepcallclosemth(L, tbc, status, yy);
        level = ((*L).stack.p as *mut libc::c_char).offset(levelrel as isize) as StkId;
    }
    return level;
}
#[no_mangle]
pub unsafe extern "C" fn luaF_newproto(mut L: *mut lua_State) -> *mut Proto {
    let mut o: *mut GCObject = luaC_newobj(
        L,
        9 as libc::c_int + 1 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int,
        ::core::mem::size_of::<Proto>() as libc::c_ulong,
    );
    let mut f: *mut Proto = &mut (*(o as *mut GCUnion)).p;
    (*f).k = 0 as *mut TValue;
    (*f).sizek = 0 as libc::c_int;
    (*f).p = 0 as *mut *mut Proto;
    (*f).sizep = 0 as libc::c_int;
    (*f).code = 0 as *mut Instruction;
    (*f).sizecode = 0 as libc::c_int;
    (*f).lineinfo = 0 as *mut ls_byte;
    (*f).sizelineinfo = 0 as libc::c_int;
    (*f).abslineinfo = 0 as *mut AbsLineInfo;
    (*f).sizeabslineinfo = 0 as libc::c_int;
    (*f).upvalues = 0 as *mut Upvaldesc;
    (*f).sizeupvalues = 0 as libc::c_int;
    (*f).numparams = 0 as libc::c_int as lu_byte;
    (*f).is_vararg = 0 as libc::c_int as lu_byte;
    (*f).maxstacksize = 0 as libc::c_int as lu_byte;
    (*f).locvars = 0 as *mut LocVar;
    (*f).sizelocvars = 0 as libc::c_int;
    (*f).linedefined = 0 as libc::c_int;
    (*f).lastlinedefined = 0 as libc::c_int;
    (*f).source = 0 as *mut TString;
    return f;
}
#[no_mangle]
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
#[no_mangle]
pub unsafe extern "C" fn luaF_getlocalname(
    mut f: *const Proto,
    mut local_number: libc::c_int,
    mut pc: libc::c_int,
) -> *const libc::c_char {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*f).sizelocvars && (*((*f).locvars).offset(i as isize)).startpc <= pc {
        if pc < (*((*f).locvars).offset(i as isize)).endpc {
            local_number -= 1;
            local_number;
            if local_number == 0 as libc::c_int {
                return ((*(*((*f).locvars).offset(i as isize)).varname).contents)
                    .as_mut_ptr();
            }
        }
        i += 1;
        i;
    }
    return 0 as *const libc::c_char;
}
