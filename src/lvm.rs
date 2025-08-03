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
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn fmod(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcoll(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn luaO_str2num(s: *const libc::c_char, o: *mut TValue) -> size_t;
    fn luaO_tostring(L: *mut lua_State, obj: *mut TValue);
    fn luaT_gettm(events: *mut Table, event: TMS, ename: *mut TString) -> *const TValue;
    fn luaT_gettmbyobj(L: *mut lua_State, o: *const TValue, event: TMS) -> *const TValue;
    fn luaT_callTM(
        L: *mut lua_State,
        f: *const TValue,
        p1: *const TValue,
        p2: *const TValue,
        p3: *const TValue,
    );
    fn luaT_callTMres(
        L: *mut lua_State,
        f: *const TValue,
        p1: *const TValue,
        p2: *const TValue,
        p3: StkId,
    );
    fn luaT_trybinTM(
        L: *mut lua_State,
        p1: *const TValue,
        p2: *const TValue,
        res: StkId,
        event: TMS,
    );
    fn luaT_tryconcatTM(L: *mut lua_State);
    fn luaT_trybinassocTM(
        L: *mut lua_State,
        p1: *const TValue,
        p2: *const TValue,
        inv: libc::c_int,
        res: StkId,
        event: TMS,
    );
    fn luaT_trybiniTM(
        L: *mut lua_State,
        p1: *const TValue,
        i2: lua_Integer,
        inv: libc::c_int,
        res: StkId,
        event: TMS,
    );
    fn luaT_callorderTM(
        L: *mut lua_State,
        p1: *const TValue,
        p2: *const TValue,
        event: TMS,
    ) -> libc::c_int;
    fn luaT_callorderiTM(
        L: *mut lua_State,
        p1: *const TValue,
        v2: libc::c_int,
        inv: libc::c_int,
        isfloat: libc::c_int,
        event: TMS,
    ) -> libc::c_int;
    fn luaT_adjustvarargs(
        L: *mut lua_State,
        nfixparams: libc::c_int,
        ci: *mut CallInfo,
        p: *const Proto,
    );
    fn luaT_getvarargs(
        L: *mut lua_State,
        ci: *mut CallInfo,
        where_0: StkId,
        wanted: libc::c_int,
    );
    fn luaG_typeerror(
        L: *mut lua_State,
        o: *const TValue,
        opname: *const libc::c_char,
    ) -> !;
    fn luaG_forerror(
        L: *mut lua_State,
        o: *const TValue,
        what: *const libc::c_char,
    ) -> !;
    fn luaG_runerror(L: *mut lua_State, fmt: *const libc::c_char, _: ...) -> !;
    fn luaG_traceexec(L: *mut lua_State, pc: *const Instruction) -> libc::c_int;
    fn luaG_tracecall(L: *mut lua_State) -> libc::c_int;
    fn luaD_hookcall(L: *mut lua_State, ci: *mut CallInfo);
    fn luaD_pretailcall(
        L: *mut lua_State,
        ci: *mut CallInfo,
        func: StkId,
        narg1: libc::c_int,
        delta: libc::c_int,
    ) -> libc::c_int;
    fn luaD_precall(
        L: *mut lua_State,
        func: StkId,
        nResults: libc::c_int,
    ) -> *mut CallInfo;
    fn luaD_call(L: *mut lua_State, func: StkId, nResults: libc::c_int);
    fn luaD_poscall(L: *mut lua_State, ci: *mut CallInfo, nres: libc::c_int);
    fn luaF_newLclosure(L: *mut lua_State, nupvals: libc::c_int) -> *mut LClosure;
    fn luaF_findupval(L: *mut lua_State, level: StkId) -> *mut UpVal;
    fn luaF_newtbcupval(L: *mut lua_State, level: StkId);
    fn luaF_closeupval(L: *mut lua_State, level: StkId);
    fn luaF_close(
        L: *mut lua_State,
        level: StkId,
        status: libc::c_int,
        yy: libc::c_int,
    ) -> StkId;
    fn luaC_step(L: *mut lua_State);
    fn luaC_barrier_(L: *mut lua_State, o: *mut GCObject, v: *mut GCObject);
    fn luaC_barrierback_(L: *mut lua_State, o: *mut GCObject);
    fn luaS_eqlngstr(a: *mut TString, b: *mut TString) -> libc::c_int;
    fn luaS_newlstr(
        L: *mut lua_State,
        str: *const libc::c_char,
        l: size_t,
    ) -> *mut TString;
    fn luaS_createlngstrobj(L: *mut lua_State, l: size_t) -> *mut TString;
    fn luaH_getint(t: *mut Table, key: lua_Integer) -> *const TValue;
    fn luaH_getshortstr(t: *mut Table, key: *mut TString) -> *const TValue;
    fn luaH_getstr(t: *mut Table, key: *mut TString) -> *const TValue;
    fn luaH_get(t: *mut Table, key: *const TValue) -> *const TValue;
    fn luaH_finishset(
        L: *mut lua_State,
        t: *mut Table,
        key: *const TValue,
        slot: *const TValue,
        value: *mut TValue,
    );
    fn luaH_new(L: *mut lua_State) -> *mut Table;
    fn luaH_resize(
        L: *mut lua_State,
        t: *mut Table,
        nasize: libc::c_uint,
        nhsize: libc::c_uint,
    );
    fn luaH_resizearray(L: *mut lua_State, t: *mut Table, nasize: libc::c_uint);
    fn luaH_getn(t: *mut Table) -> lua_Unsigned;
    fn luaH_realasize(t: *const Table) -> libc::c_uint;
}
pub type __sig_atomic_t = libc::c_int;
pub type size_t = libc::c_ulong;
pub type ptrdiff_t = libc::c_long;
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
pub type lua_Unsigned = libc::c_ulonglong;
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
pub type OpCode = libc::c_uint;
pub const OP_EXTRAARG: OpCode = 82;
pub const OP_VARARGPREP: OpCode = 81;
pub const OP_VARARG: OpCode = 80;
pub const OP_CLOSURE: OpCode = 79;
pub const OP_SETLIST: OpCode = 78;
pub const OP_TFORLOOP: OpCode = 77;
pub const OP_TFORCALL: OpCode = 76;
pub const OP_TFORPREP: OpCode = 75;
pub const OP_FORPREP: OpCode = 74;
pub const OP_FORLOOP: OpCode = 73;
pub const OP_RETURN1: OpCode = 72;
pub const OP_RETURN0: OpCode = 71;
pub const OP_RETURN: OpCode = 70;
pub const OP_TAILCALL: OpCode = 69;
pub const OP_CALL: OpCode = 68;
pub const OP_TESTSET: OpCode = 67;
pub const OP_TEST: OpCode = 66;
pub const OP_GEI: OpCode = 65;
pub const OP_GTI: OpCode = 64;
pub const OP_LEI: OpCode = 63;
pub const OP_LTI: OpCode = 62;
pub const OP_EQI: OpCode = 61;
pub const OP_EQK: OpCode = 60;
pub const OP_LE: OpCode = 59;
pub const OP_LT: OpCode = 58;
pub const OP_EQ: OpCode = 57;
pub const OP_JMP: OpCode = 56;
pub const OP_TBC: OpCode = 55;
pub const OP_CLOSE: OpCode = 54;
pub const OP_CONCAT: OpCode = 53;
pub const OP_LEN: OpCode = 52;
pub const OP_NOT: OpCode = 51;
pub const OP_BNOT: OpCode = 50;
pub const OP_UNM: OpCode = 49;
pub const OP_MMBINK: OpCode = 48;
pub const OP_MMBINI: OpCode = 47;
pub const OP_MMBIN: OpCode = 46;
pub const OP_SHR: OpCode = 45;
pub const OP_SHL: OpCode = 44;
pub const OP_BXOR: OpCode = 43;
pub const OP_BOR: OpCode = 42;
pub const OP_BAND: OpCode = 41;
pub const OP_IDIV: OpCode = 40;
pub const OP_DIV: OpCode = 39;
pub const OP_POW: OpCode = 38;
pub const OP_MOD: OpCode = 37;
pub const OP_MUL: OpCode = 36;
pub const OP_SUB: OpCode = 35;
pub const OP_ADD: OpCode = 34;
pub const OP_SHLI: OpCode = 33;
pub const OP_SHRI: OpCode = 32;
pub const OP_BXORK: OpCode = 31;
pub const OP_BORK: OpCode = 30;
pub const OP_BANDK: OpCode = 29;
pub const OP_IDIVK: OpCode = 28;
pub const OP_DIVK: OpCode = 27;
pub const OP_POWK: OpCode = 26;
pub const OP_MODK: OpCode = 25;
pub const OP_MULK: OpCode = 24;
pub const OP_SUBK: OpCode = 23;
pub const OP_ADDK: OpCode = 22;
pub const OP_ADDI: OpCode = 21;
pub const OP_SELF: OpCode = 20;
pub const OP_NEWTABLE: OpCode = 19;
pub const OP_SETFIELD: OpCode = 18;
pub const OP_SETI: OpCode = 17;
pub const OP_SETTABLE: OpCode = 16;
pub const OP_SETTABUP: OpCode = 15;
pub const OP_GETFIELD: OpCode = 14;
pub const OP_GETI: OpCode = 13;
pub const OP_GETTABLE: OpCode = 12;
pub const OP_GETTABUP: OpCode = 11;
pub const OP_SETUPVAL: OpCode = 10;
pub const OP_GETUPVAL: OpCode = 9;
pub const OP_LOADNIL: OpCode = 8;
pub const OP_LOADTRUE: OpCode = 7;
pub const OP_LFALSESKIP: OpCode = 6;
pub const OP_LOADFALSE: OpCode = 5;
pub const OP_LOADKX: OpCode = 4;
pub const OP_LOADK: OpCode = 3;
pub const OP_LOADF: OpCode = 2;
pub const OP_LOADI: OpCode = 1;
pub const OP_MOVE: OpCode = 0;
pub type F2Imod = libc::c_uint;
pub const F2Iceil: F2Imod = 2;
pub const F2Ifloor: F2Imod = 1;
pub const F2Ieq: F2Imod = 0;
unsafe extern "C" fn l_strton(
    mut obj: *const TValue,
    mut result: *mut TValue,
) -> libc::c_int {
    if !((*obj).tt_ as libc::c_int & 0xf as libc::c_int == 4 as libc::c_int) {
        return 0 as libc::c_int
    } else {
        let mut st: *mut TString = &mut (*((*obj).value_.gc as *mut GCUnion)).ts;
        return (luaO_str2num(((*st).contents).as_mut_ptr(), result)
            == (if (*st).shrlen as libc::c_int != 0xff as libc::c_int {
                (*st).shrlen as libc::c_ulong
            } else {
                (*st).u.lnglen
            })
                .wrapping_add(1 as libc::c_int as libc::c_ulong)) as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaV_tonumber_(
    mut obj: *const TValue,
    mut n: *mut lua_Number,
) -> libc::c_int {
    let mut v: TValue = TValue {
        value_: Value { gc: 0 as *mut GCObject },
        tt_: 0,
    };
    if (*obj).tt_ as libc::c_int
        == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
    {
        *n = (*obj).value_.i as lua_Number;
        return 1 as libc::c_int;
    } else if l_strton(obj, &mut v) != 0 {
        *n = if v.tt_ as libc::c_int
            == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        {
            v.value_.i as lua_Number
        } else {
            v.value_.n
        };
        return 1 as libc::c_int;
    } else {
        return 0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaV_flttointeger(
    mut n: lua_Number,
    mut p: *mut lua_Integer,
    mut mode: F2Imod,
) -> libc::c_int {
    let mut f: lua_Number = floor(n);
    if n != f {
        if mode as libc::c_uint == F2Ieq as libc::c_int as libc::c_uint {
            return 0 as libc::c_int
        } else if mode as libc::c_uint == F2Iceil as libc::c_int as libc::c_uint {
            f += 1 as libc::c_int as libc::c_double;
        }
    }
    return (f
        >= (-(9223372036854775807 as libc::c_longlong) - 1 as libc::c_longlong)
            as libc::c_double
        && f
            < -((-(9223372036854775807 as libc::c_longlong) - 1 as libc::c_longlong)
                as libc::c_double)
        && {
            *p = f as libc::c_longlong;
            1 as libc::c_int != 0
        }) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn luaV_tointegerns(
    mut obj: *const TValue,
    mut p: *mut lua_Integer,
    mut mode: F2Imod,
) -> libc::c_int {
    if (*obj).tt_ as libc::c_int
        == 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
    {
        return luaV_flttointeger((*obj).value_.n, p, mode)
    } else if (*obj).tt_ as libc::c_int
        == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
    {
        *p = (*obj).value_.i;
        return 1 as libc::c_int;
    } else {
        return 0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaV_tointeger(
    mut obj: *const TValue,
    mut p: *mut lua_Integer,
    mut mode: F2Imod,
) -> libc::c_int {
    let mut v: TValue = TValue {
        value_: Value { gc: 0 as *mut GCObject },
        tt_: 0,
    };
    if l_strton(obj, &mut v) != 0 {
        obj = &mut v;
    }
    return luaV_tointegerns(obj, p, mode);
}
unsafe extern "C" fn forlimit(
    mut L: *mut lua_State,
    mut init: lua_Integer,
    mut lim: *const TValue,
    mut p: *mut lua_Integer,
    mut step: lua_Integer,
) -> libc::c_int {
    if luaV_tointeger(
        lim,
        p,
        (if step < 0 as libc::c_int as libc::c_longlong {
            F2Iceil as libc::c_int
        } else {
            F2Ifloor as libc::c_int
        }) as F2Imod,
    ) == 0
    {
        let mut flim: lua_Number = 0.;
        if if (*lim).tt_ as libc::c_int
            == 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
        {
            flim = (*lim).value_.n;
            1 as libc::c_int
        } else {
            luaV_tonumber_(lim, &mut flim)
        } == 0
        {
            luaG_forerror(L, lim, b"limit\0" as *const u8 as *const libc::c_char);
        }
        if (0 as libc::c_int as libc::c_double) < flim {
            if step < 0 as libc::c_int as libc::c_longlong {
                return 1 as libc::c_int;
            }
            *p = 9223372036854775807 as libc::c_longlong;
        } else {
            if step > 0 as libc::c_int as libc::c_longlong {
                return 1 as libc::c_int;
            }
            *p = -(9223372036854775807 as libc::c_longlong) - 1 as libc::c_longlong;
        }
    }
    return if step > 0 as libc::c_int as libc::c_longlong {
        (init > *p) as libc::c_int
    } else {
        (init < *p) as libc::c_int
    };
}
unsafe extern "C" fn forprep(mut L: *mut lua_State, mut ra: StkId) -> libc::c_int {
    let mut pinit: *mut TValue = &mut (*ra).val;
    let mut plimit: *mut TValue = &mut (*ra.offset(1 as libc::c_int as isize)).val;
    let mut pstep: *mut TValue = &mut (*ra.offset(2 as libc::c_int as isize)).val;
    if (*pinit).tt_ as libc::c_int
        == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        && (*pstep).tt_ as libc::c_int
            == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
    {
        let mut init: lua_Integer = (*pinit).value_.i;
        let mut step: lua_Integer = (*pstep).value_.i;
        let mut limit: lua_Integer = 0;
        if step == 0 as libc::c_int as libc::c_longlong {
            luaG_runerror(
                L,
                b"'for' step is zero\0" as *const u8 as *const libc::c_char,
            );
        }
        let mut io: *mut TValue = &mut (*ra.offset(3 as libc::c_int as isize)).val;
        (*io).value_.i = init;
        (*io)
            .tt_ = (3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int)
            as lu_byte;
        if forlimit(L, init, plimit, &mut limit, step) != 0 {
            return 1 as libc::c_int
        } else {
            let mut count: lua_Unsigned = 0;
            if step > 0 as libc::c_int as libc::c_longlong {
                count = (limit as lua_Unsigned).wrapping_sub(init as lua_Unsigned);
                if step != 1 as libc::c_int as libc::c_longlong {
                    count = (count as libc::c_ulonglong)
                        .wrapping_div(step as lua_Unsigned) as lua_Unsigned
                        as lua_Unsigned;
                }
            } else {
                count = (init as lua_Unsigned).wrapping_sub(limit as lua_Unsigned);
                count = (count as libc::c_ulonglong)
                    .wrapping_div(
                        (-(step + 1 as libc::c_int as libc::c_longlong) as lua_Unsigned)
                            .wrapping_add(1 as libc::c_uint as libc::c_ulonglong),
                    ) as lua_Unsigned as lua_Unsigned;
            }
            let mut io_0: *mut TValue = plimit;
            (*io_0).value_.i = count as lua_Integer;
            (*io_0)
                .tt_ = (3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int)
                as lu_byte;
        }
    } else {
        let mut init_0: lua_Number = 0.;
        let mut limit_0: lua_Number = 0.;
        let mut step_0: lua_Number = 0.;
        if (((if (*plimit).tt_ as libc::c_int
            == 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
        {
            limit_0 = (*plimit).value_.n;
            1 as libc::c_int
        } else {
            luaV_tonumber_(plimit, &mut limit_0)
        }) == 0) as libc::c_int != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
        {
            luaG_forerror(L, plimit, b"limit\0" as *const u8 as *const libc::c_char);
        }
        if (((if (*pstep).tt_ as libc::c_int
            == 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
        {
            step_0 = (*pstep).value_.n;
            1 as libc::c_int
        } else {
            luaV_tonumber_(pstep, &mut step_0)
        }) == 0) as libc::c_int != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
        {
            luaG_forerror(L, pstep, b"step\0" as *const u8 as *const libc::c_char);
        }
        if (((if (*pinit).tt_ as libc::c_int
            == 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
        {
            init_0 = (*pinit).value_.n;
            1 as libc::c_int
        } else {
            luaV_tonumber_(pinit, &mut init_0)
        }) == 0) as libc::c_int != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
        {
            luaG_forerror(
                L,
                pinit,
                b"initial value\0" as *const u8 as *const libc::c_char,
            );
        }
        if step_0 == 0 as libc::c_int as libc::c_double {
            luaG_runerror(
                L,
                b"'for' step is zero\0" as *const u8 as *const libc::c_char,
            );
        }
        if if (0 as libc::c_int as libc::c_double) < step_0 {
            (limit_0 < init_0) as libc::c_int
        } else {
            (init_0 < limit_0) as libc::c_int
        } != 0
        {
            return 1 as libc::c_int
        } else {
            let mut io_1: *mut TValue = plimit;
            (*io_1).value_.n = limit_0;
            (*io_1)
                .tt_ = (3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int)
                as lu_byte;
            let mut io_2: *mut TValue = pstep;
            (*io_2).value_.n = step_0;
            (*io_2)
                .tt_ = (3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int)
                as lu_byte;
            let mut io_3: *mut TValue = &mut (*ra).val;
            (*io_3).value_.n = init_0;
            (*io_3)
                .tt_ = (3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int)
                as lu_byte;
            let mut io_4: *mut TValue = &mut (*ra.offset(3 as libc::c_int as isize)).val;
            (*io_4).value_.n = init_0;
            (*io_4)
                .tt_ = (3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int)
                as lu_byte;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn floatforloop(mut ra: StkId) -> libc::c_int {
    let mut step: lua_Number = (*ra.offset(2 as libc::c_int as isize)).val.value_.n;
    let mut limit: lua_Number = (*ra.offset(1 as libc::c_int as isize)).val.value_.n;
    let mut idx: lua_Number = (*ra).val.value_.n;
    idx = idx + step;
    if if (0 as libc::c_int as libc::c_double) < step {
        (idx <= limit) as libc::c_int
    } else {
        (limit <= idx) as libc::c_int
    } != 0
    {
        let mut io: *mut TValue = &mut (*ra).val;
        (*io).value_.n = idx;
        let mut io_0: *mut TValue = &mut (*ra.offset(3 as libc::c_int as isize)).val;
        (*io_0).value_.n = idx;
        (*io_0)
            .tt_ = (3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int)
            as lu_byte;
        return 1 as libc::c_int;
    } else {
        return 0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaV_finishget(
    mut L: *mut lua_State,
    mut t: *const TValue,
    mut key: *mut TValue,
    mut val: StkId,
    mut slot: *const TValue,
) {
    let mut loop_0: libc::c_int = 0;
    let mut tm: *const TValue = 0 as *const TValue;
    loop_0 = 0 as libc::c_int;
    while loop_0 < 2000 as libc::c_int {
        if slot.is_null() {
            tm = luaT_gettmbyobj(L, t, TM_INDEX);
            if (((*tm).tt_ as libc::c_int & 0xf as libc::c_int == 0 as libc::c_int)
                as libc::c_int != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
            {
                luaG_typeerror(L, t, b"index\0" as *const u8 as *const libc::c_char);
            }
        } else {
            tm = if ((*((*t).value_.gc as *mut GCUnion)).h.metatable).is_null() {
                0 as *const TValue
            } else if (*(*((*t).value_.gc as *mut GCUnion)).h.metatable).flags
                as libc::c_uint & (1 as libc::c_uint) << TM_INDEX as libc::c_int != 0
            {
                0 as *const TValue
            } else {
                luaT_gettm(
                    (*((*t).value_.gc as *mut GCUnion)).h.metatable,
                    TM_INDEX,
                    (*(*L).l_G).tmname[TM_INDEX as libc::c_int as usize],
                )
            };
            if tm.is_null() {
                (*val)
                    .val
                    .tt_ = (0 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int)
                    as lu_byte;
                return;
            }
        }
        if (*tm).tt_ as libc::c_int & 0xf as libc::c_int == 6 as libc::c_int {
            luaT_callTMres(L, tm, t, key, val);
            return;
        }
        t = tm;
        if if !((*t).tt_ as libc::c_int
            == 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                | (1 as libc::c_int) << 6 as libc::c_int)
        {
            slot = 0 as *const TValue;
            0 as libc::c_int
        } else {
            slot = luaH_get(&mut (*((*t).value_.gc as *mut GCUnion)).h, key);
            !((*slot).tt_ as libc::c_int & 0xf as libc::c_int == 0 as libc::c_int)
                as libc::c_int
        } != 0
        {
            let mut io1: *mut TValue = &mut (*val).val;
            let mut io2: *const TValue = slot;
            (*io1).value_ = (*io2).value_;
            (*io1).tt_ = (*io2).tt_;
            return;
        }
        loop_0 += 1;
        loop_0;
    }
    luaG_runerror(
        L,
        b"'__index' chain too long; possible loop\0" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn luaV_finishset(
    mut L: *mut lua_State,
    mut t: *const TValue,
    mut key: *mut TValue,
    mut val: *mut TValue,
    mut slot: *const TValue,
) {
    let mut loop_0: libc::c_int = 0;
    loop_0 = 0 as libc::c_int;
    while loop_0 < 2000 as libc::c_int {
        let mut tm: *const TValue = 0 as *const TValue;
        if !slot.is_null() {
            let mut h: *mut Table = &mut (*((*t).value_.gc as *mut GCUnion)).h;
            tm = if ((*h).metatable).is_null() {
                0 as *const TValue
            } else if (*(*h).metatable).flags as libc::c_uint
                & (1 as libc::c_uint) << TM_NEWINDEX as libc::c_int != 0
            {
                0 as *const TValue
            } else {
                luaT_gettm(
                    (*h).metatable,
                    TM_NEWINDEX,
                    (*(*L).l_G).tmname[TM_NEWINDEX as libc::c_int as usize],
                )
            };
            if tm.is_null() {
                let mut io: *mut TValue = &mut (*(*L).top.p).val;
                let mut x_: *mut Table = h;
                (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
                (*io)
                    .tt_ = (5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                    | (1 as libc::c_int) << 6 as libc::c_int) as lu_byte;
                (*L).top.p = ((*L).top.p).offset(1);
                (*L).top.p;
                luaH_finishset(L, h, key, slot, val);
                (*L).top.p = ((*L).top.p).offset(-1);
                (*L).top.p;
                (*h)
                    .flags = ((*h).flags as libc::c_uint
                    & !!(!(0 as libc::c_uint)
                        << TM_EQ as libc::c_int + 1 as libc::c_int)) as lu_byte;
                if (*val).tt_ as libc::c_int & (1 as libc::c_int) << 6 as libc::c_int
                    != 0
                {
                    if (*(h as *mut GCUnion)).gc.marked as libc::c_int
                        & (1 as libc::c_int) << 5 as libc::c_int != 0
                        && (*(*val).value_.gc).marked as libc::c_int
                            & ((1 as libc::c_int) << 3 as libc::c_int
                                | (1 as libc::c_int) << 4 as libc::c_int) != 0
                    {
                        luaC_barrierback_(L, &mut (*(h as *mut GCUnion)).gc);
                    } else {};
                } else {};
                return;
            }
        } else {
            tm = luaT_gettmbyobj(L, t, TM_NEWINDEX);
            if (((*tm).tt_ as libc::c_int & 0xf as libc::c_int == 0 as libc::c_int)
                as libc::c_int != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
            {
                luaG_typeerror(L, t, b"index\0" as *const u8 as *const libc::c_char);
            }
        }
        if (*tm).tt_ as libc::c_int & 0xf as libc::c_int == 6 as libc::c_int {
            luaT_callTM(L, tm, t, key, val);
            return;
        }
        t = tm;
        if if !((*t).tt_ as libc::c_int
            == 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                | (1 as libc::c_int) << 6 as libc::c_int)
        {
            slot = 0 as *const TValue;
            0 as libc::c_int
        } else {
            slot = luaH_get(&mut (*((*t).value_.gc as *mut GCUnion)).h, key);
            !((*slot).tt_ as libc::c_int & 0xf as libc::c_int == 0 as libc::c_int)
                as libc::c_int
        } != 0
        {
            let mut io1: *mut TValue = slot as *mut TValue;
            let mut io2: *const TValue = val;
            (*io1).value_ = (*io2).value_;
            (*io1).tt_ = (*io2).tt_;
            if (*val).tt_ as libc::c_int & (1 as libc::c_int) << 6 as libc::c_int != 0 {
                if (*(*t).value_.gc).marked as libc::c_int
                    & (1 as libc::c_int) << 5 as libc::c_int != 0
                    && (*(*val).value_.gc).marked as libc::c_int
                        & ((1 as libc::c_int) << 3 as libc::c_int
                            | (1 as libc::c_int) << 4 as libc::c_int) != 0
                {
                    luaC_barrierback_(L, (*t).value_.gc);
                } else {};
            } else {};
            return;
        }
        loop_0 += 1;
        loop_0;
    }
    luaG_runerror(
        L,
        b"'__newindex' chain too long; possible loop\0" as *const u8
            as *const libc::c_char,
    );
}
unsafe extern "C" fn l_strcmp(
    mut ts1: *const TString,
    mut ts2: *const TString,
) -> libc::c_int {
    let mut s1: *const libc::c_char = ((*ts1).contents).as_ptr();
    let mut rl1: size_t = if (*ts1).shrlen as libc::c_int != 0xff as libc::c_int {
        (*ts1).shrlen as libc::c_ulong
    } else {
        (*ts1).u.lnglen
    };
    let mut s2: *const libc::c_char = ((*ts2).contents).as_ptr();
    let mut rl2: size_t = if (*ts2).shrlen as libc::c_int != 0xff as libc::c_int {
        (*ts2).shrlen as libc::c_ulong
    } else {
        (*ts2).u.lnglen
    };
    loop {
        let mut temp: libc::c_int = strcoll(s1, s2);
        if temp != 0 as libc::c_int {
            return temp
        } else {
            let mut zl1: size_t = strlen(s1);
            let mut zl2: size_t = strlen(s2);
            if zl2 == rl2 {
                return if zl1 == rl1 { 0 as libc::c_int } else { 1 as libc::c_int }
            } else if zl1 == rl1 {
                return -(1 as libc::c_int)
            }
            zl1 = zl1.wrapping_add(1);
            zl1;
            zl2 = zl2.wrapping_add(1);
            zl2;
            s1 = s1.offset(zl1 as isize);
            rl1 = (rl1 as libc::c_ulong).wrapping_sub(zl1) as size_t as size_t;
            s2 = s2.offset(zl2 as isize);
            rl2 = (rl2 as libc::c_ulong).wrapping_sub(zl2) as size_t as size_t;
        }
    };
}
#[inline]
unsafe extern "C" fn LTintfloat(mut i: lua_Integer, mut f: lua_Number) -> libc::c_int {
    if ((1 as libc::c_int as lua_Unsigned) << 53 as libc::c_int)
        .wrapping_add(i as lua_Unsigned)
        <= (2 as libc::c_int as libc::c_ulonglong)
            .wrapping_mul((1 as libc::c_int as lua_Unsigned) << 53 as libc::c_int)
    {
        return ((i as lua_Number) < f) as libc::c_int
    } else {
        let mut fi: lua_Integer = 0;
        if luaV_flttointeger(f, &mut fi, F2Iceil) != 0 {
            return (i < fi) as libc::c_int
        } else {
            return (f > 0 as libc::c_int as libc::c_double) as libc::c_int
        }
    };
}
#[inline]
unsafe extern "C" fn LEintfloat(mut i: lua_Integer, mut f: lua_Number) -> libc::c_int {
    if ((1 as libc::c_int as lua_Unsigned) << 53 as libc::c_int)
        .wrapping_add(i as lua_Unsigned)
        <= (2 as libc::c_int as libc::c_ulonglong)
            .wrapping_mul((1 as libc::c_int as lua_Unsigned) << 53 as libc::c_int)
    {
        return (i as lua_Number <= f) as libc::c_int
    } else {
        let mut fi: lua_Integer = 0;
        if luaV_flttointeger(f, &mut fi, F2Ifloor) != 0 {
            return (i <= fi) as libc::c_int
        } else {
            return (f > 0 as libc::c_int as libc::c_double) as libc::c_int
        }
    };
}
#[inline]
unsafe extern "C" fn LTfloatint(mut f: lua_Number, mut i: lua_Integer) -> libc::c_int {
    if ((1 as libc::c_int as lua_Unsigned) << 53 as libc::c_int)
        .wrapping_add(i as lua_Unsigned)
        <= (2 as libc::c_int as libc::c_ulonglong)
            .wrapping_mul((1 as libc::c_int as lua_Unsigned) << 53 as libc::c_int)
    {
        return (f < i as lua_Number) as libc::c_int
    } else {
        let mut fi: lua_Integer = 0;
        if luaV_flttointeger(f, &mut fi, F2Ifloor) != 0 {
            return (fi < i) as libc::c_int
        } else {
            return (f < 0 as libc::c_int as libc::c_double) as libc::c_int
        }
    };
}
#[inline]
unsafe extern "C" fn LEfloatint(mut f: lua_Number, mut i: lua_Integer) -> libc::c_int {
    if ((1 as libc::c_int as lua_Unsigned) << 53 as libc::c_int)
        .wrapping_add(i as lua_Unsigned)
        <= (2 as libc::c_int as libc::c_ulonglong)
            .wrapping_mul((1 as libc::c_int as lua_Unsigned) << 53 as libc::c_int)
    {
        return (f <= i as lua_Number) as libc::c_int
    } else {
        let mut fi: lua_Integer = 0;
        if luaV_flttointeger(f, &mut fi, F2Iceil) != 0 {
            return (fi <= i) as libc::c_int
        } else {
            return (f < 0 as libc::c_int as libc::c_double) as libc::c_int
        }
    };
}
#[inline]
unsafe extern "C" fn LTnum(mut l: *const TValue, mut r: *const TValue) -> libc::c_int {
    if (*l).tt_ as libc::c_int
        == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
    {
        let mut li: lua_Integer = (*l).value_.i;
        if (*r).tt_ as libc::c_int
            == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        {
            return (li < (*r).value_.i) as libc::c_int
        } else {
            return LTintfloat(li, (*r).value_.n)
        }
    } else {
        let mut lf: lua_Number = (*l).value_.n;
        if (*r).tt_ as libc::c_int
            == 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
        {
            return (lf < (*r).value_.n) as libc::c_int
        } else {
            return LTfloatint(lf, (*r).value_.i)
        }
    };
}
#[inline]
unsafe extern "C" fn LEnum(mut l: *const TValue, mut r: *const TValue) -> libc::c_int {
    if (*l).tt_ as libc::c_int
        == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
    {
        let mut li: lua_Integer = (*l).value_.i;
        if (*r).tt_ as libc::c_int
            == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        {
            return (li <= (*r).value_.i) as libc::c_int
        } else {
            return LEintfloat(li, (*r).value_.n)
        }
    } else {
        let mut lf: lua_Number = (*l).value_.n;
        if (*r).tt_ as libc::c_int
            == 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
        {
            return (lf <= (*r).value_.n) as libc::c_int
        } else {
            return LEfloatint(lf, (*r).value_.i)
        }
    };
}
unsafe extern "C" fn lessthanothers(
    mut L: *mut lua_State,
    mut l: *const TValue,
    mut r: *const TValue,
) -> libc::c_int {
    if (*l).tt_ as libc::c_int & 0xf as libc::c_int == 4 as libc::c_int
        && (*r).tt_ as libc::c_int & 0xf as libc::c_int == 4 as libc::c_int
    {
        return (l_strcmp(
            &mut (*((*l).value_.gc as *mut GCUnion)).ts,
            &mut (*((*r).value_.gc as *mut GCUnion)).ts,
        ) < 0 as libc::c_int) as libc::c_int
    } else {
        return luaT_callorderTM(L, l, r, TM_LT)
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaV_lessthan(
    mut L: *mut lua_State,
    mut l: *const TValue,
    mut r: *const TValue,
) -> libc::c_int {
    if (*l).tt_ as libc::c_int & 0xf as libc::c_int == 3 as libc::c_int
        && (*r).tt_ as libc::c_int & 0xf as libc::c_int == 3 as libc::c_int
    {
        return LTnum(l, r)
    } else {
        return lessthanothers(L, l, r)
    };
}
unsafe extern "C" fn lessequalothers(
    mut L: *mut lua_State,
    mut l: *const TValue,
    mut r: *const TValue,
) -> libc::c_int {
    if (*l).tt_ as libc::c_int & 0xf as libc::c_int == 4 as libc::c_int
        && (*r).tt_ as libc::c_int & 0xf as libc::c_int == 4 as libc::c_int
    {
        return (l_strcmp(
            &mut (*((*l).value_.gc as *mut GCUnion)).ts,
            &mut (*((*r).value_.gc as *mut GCUnion)).ts,
        ) <= 0 as libc::c_int) as libc::c_int
    } else {
        return luaT_callorderTM(L, l, r, TM_LE)
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaV_lessequal(
    mut L: *mut lua_State,
    mut l: *const TValue,
    mut r: *const TValue,
) -> libc::c_int {
    if (*l).tt_ as libc::c_int & 0xf as libc::c_int == 3 as libc::c_int
        && (*r).tt_ as libc::c_int & 0xf as libc::c_int == 3 as libc::c_int
    {
        return LEnum(l, r)
    } else {
        return lessequalothers(L, l, r)
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaV_equalobj(
    mut L: *mut lua_State,
    mut t1: *const TValue,
    mut t2: *const TValue,
) -> libc::c_int {
    let mut tm: *const TValue = 0 as *const TValue;
    if (*t1).tt_ as libc::c_int & 0x3f as libc::c_int
        != (*t2).tt_ as libc::c_int & 0x3f as libc::c_int
    {
        if (*t1).tt_ as libc::c_int & 0xf as libc::c_int
            != (*t2).tt_ as libc::c_int & 0xf as libc::c_int
            || (*t1).tt_ as libc::c_int & 0xf as libc::c_int != 3 as libc::c_int
        {
            return 0 as libc::c_int
        } else {
            let mut i1: lua_Integer = 0;
            let mut i2: lua_Integer = 0;
            return (luaV_tointegerns(t1, &mut i1, F2Ieq) != 0
                && luaV_tointegerns(t2, &mut i2, F2Ieq) != 0 && i1 == i2) as libc::c_int;
        }
    }
    match (*t1).tt_ as libc::c_int & 0x3f as libc::c_int {
        0 | 1 | 17 => return 1 as libc::c_int,
        3 => return ((*t1).value_.i == (*t2).value_.i) as libc::c_int,
        19 => return ((*t1).value_.n == (*t2).value_.n) as libc::c_int,
        2 => return ((*t1).value_.p == (*t2).value_.p) as libc::c_int,
        22 => return ((*t1).value_.f == (*t2).value_.f) as libc::c_int,
        4 => {
            return (&mut (*((*t1).value_.gc as *mut GCUnion)).ts as *mut TString
                == &mut (*((*t2).value_.gc as *mut GCUnion)).ts as *mut TString)
                as libc::c_int;
        }
        20 => {
            return luaS_eqlngstr(
                &mut (*((*t1).value_.gc as *mut GCUnion)).ts,
                &mut (*((*t2).value_.gc as *mut GCUnion)).ts,
            );
        }
        7 => {
            if &mut (*((*t1).value_.gc as *mut GCUnion)).u as *mut Udata
                == &mut (*((*t2).value_.gc as *mut GCUnion)).u as *mut Udata
            {
                return 1 as libc::c_int
            } else if L.is_null() {
                return 0 as libc::c_int
            }
            tm = if ((*((*t1).value_.gc as *mut GCUnion)).u.metatable).is_null() {
                0 as *const TValue
            } else if (*(*((*t1).value_.gc as *mut GCUnion)).u.metatable).flags
                as libc::c_uint & (1 as libc::c_uint) << TM_EQ as libc::c_int != 0
            {
                0 as *const TValue
            } else {
                luaT_gettm(
                    (*((*t1).value_.gc as *mut GCUnion)).u.metatable,
                    TM_EQ,
                    (*(*L).l_G).tmname[TM_EQ as libc::c_int as usize],
                )
            };
            if tm.is_null() {
                tm = if ((*((*t2).value_.gc as *mut GCUnion)).u.metatable).is_null() {
                    0 as *const TValue
                } else if (*(*((*t2).value_.gc as *mut GCUnion)).u.metatable).flags
                    as libc::c_uint & (1 as libc::c_uint) << TM_EQ as libc::c_int != 0
                {
                    0 as *const TValue
                } else {
                    luaT_gettm(
                        (*((*t2).value_.gc as *mut GCUnion)).u.metatable,
                        TM_EQ,
                        (*(*L).l_G).tmname[TM_EQ as libc::c_int as usize],
                    )
                };
            }
        }
        5 => {
            if &mut (*((*t1).value_.gc as *mut GCUnion)).h as *mut Table
                == &mut (*((*t2).value_.gc as *mut GCUnion)).h as *mut Table
            {
                return 1 as libc::c_int
            } else if L.is_null() {
                return 0 as libc::c_int
            }
            tm = if ((*((*t1).value_.gc as *mut GCUnion)).h.metatable).is_null() {
                0 as *const TValue
            } else if (*(*((*t1).value_.gc as *mut GCUnion)).h.metatable).flags
                as libc::c_uint & (1 as libc::c_uint) << TM_EQ as libc::c_int != 0
            {
                0 as *const TValue
            } else {
                luaT_gettm(
                    (*((*t1).value_.gc as *mut GCUnion)).h.metatable,
                    TM_EQ,
                    (*(*L).l_G).tmname[TM_EQ as libc::c_int as usize],
                )
            };
            if tm.is_null() {
                tm = if ((*((*t2).value_.gc as *mut GCUnion)).h.metatable).is_null() {
                    0 as *const TValue
                } else if (*(*((*t2).value_.gc as *mut GCUnion)).h.metatable).flags
                    as libc::c_uint & (1 as libc::c_uint) << TM_EQ as libc::c_int != 0
                {
                    0 as *const TValue
                } else {
                    luaT_gettm(
                        (*((*t2).value_.gc as *mut GCUnion)).h.metatable,
                        TM_EQ,
                        (*(*L).l_G).tmname[TM_EQ as libc::c_int as usize],
                    )
                };
            }
        }
        _ => return ((*t1).value_.gc == (*t2).value_.gc) as libc::c_int,
    }
    if tm.is_null() {
        return 0 as libc::c_int
    } else {
        luaT_callTMres(L, tm, t1, t2, (*L).top.p);
        return !((*(*L).top.p).val.tt_ as libc::c_int
            == 1 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
            || (*(*L).top.p).val.tt_ as libc::c_int & 0xf as libc::c_int
                == 0 as libc::c_int) as libc::c_int;
    };
}
unsafe extern "C" fn copy2buff(
    mut top: StkId,
    mut n: libc::c_int,
    mut buff: *mut libc::c_char,
) {
    let mut tl: size_t = 0 as libc::c_int as size_t;
    loop {
        let mut st: *mut TString = &mut (*((*top.offset(-(n as isize))).val.value_.gc
            as *mut GCUnion))
            .ts;
        let mut l: size_t = if (*st).shrlen as libc::c_int != 0xff as libc::c_int {
            (*st).shrlen as libc::c_ulong
        } else {
            (*st).u.lnglen
        };
        memcpy(
            buff.offset(tl as isize) as *mut libc::c_void,
            ((*st).contents).as_mut_ptr() as *const libc::c_void,
            l.wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
        );
        tl = (tl as libc::c_ulong).wrapping_add(l) as size_t as size_t;
        n -= 1;
        if !(n > 0 as libc::c_int) {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaV_concat(mut L: *mut lua_State, mut total: libc::c_int) {
    if total == 1 as libc::c_int {
        return;
    }
    loop {
        let mut top: StkId = (*L).top.p;
        let mut n: libc::c_int = 2 as libc::c_int;
        if !((*top.offset(-(2 as libc::c_int as isize))).val.tt_ as libc::c_int
            & 0xf as libc::c_int == 4 as libc::c_int
            || (*top.offset(-(2 as libc::c_int as isize))).val.tt_ as libc::c_int
                & 0xf as libc::c_int == 3 as libc::c_int)
            || !((*top.offset(-(1 as libc::c_int as isize))).val.tt_ as libc::c_int
                & 0xf as libc::c_int == 4 as libc::c_int
                || (*top.offset(-(1 as libc::c_int as isize))).val.tt_ as libc::c_int
                    & 0xf as libc::c_int == 3 as libc::c_int
                    && {
                        luaO_tostring(
                            L,
                            &mut (*top.offset(-(1 as libc::c_int as isize))).val,
                        );
                        1 as libc::c_int != 0
                    })
        {
            luaT_tryconcatTM(L);
        } else if (*top.offset(-(1 as libc::c_int as isize))).val.tt_ as libc::c_int
            == 4 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                | (1 as libc::c_int) << 6 as libc::c_int
            && (*((*top.offset(-(1 as libc::c_int as isize))).val.value_.gc
                as *mut GCUnion))
                .ts
                .shrlen as libc::c_int == 0 as libc::c_int
        {
            ((*top.offset(-(2 as libc::c_int as isize))).val.tt_ as libc::c_int
                & 0xf as libc::c_int == 4 as libc::c_int
                || (*top.offset(-(2 as libc::c_int as isize))).val.tt_ as libc::c_int
                    & 0xf as libc::c_int == 3 as libc::c_int
                    && {
                        luaO_tostring(
                            L,
                            &mut (*top.offset(-(2 as libc::c_int as isize))).val,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
        } else if (*top.offset(-(2 as libc::c_int as isize))).val.tt_ as libc::c_int
            == 4 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                | (1 as libc::c_int) << 6 as libc::c_int
            && (*((*top.offset(-(2 as libc::c_int as isize))).val.value_.gc
                as *mut GCUnion))
                .ts
                .shrlen as libc::c_int == 0 as libc::c_int
        {
            let mut io1: *mut TValue = &mut (*top.offset(-(2 as libc::c_int as isize)))
                .val;
            let mut io2: *const TValue = &mut (*top.offset(-(1 as libc::c_int as isize)))
                .val;
            (*io1).value_ = (*io2).value_;
            (*io1).tt_ = (*io2).tt_;
        } else {
            let mut tl: size_t = if (*((*top.offset(-(1 as libc::c_int as isize)))
                .val
                .value_
                .gc as *mut GCUnion))
                .ts
                .shrlen as libc::c_int != 0xff as libc::c_int
            {
                (*((*top.offset(-(1 as libc::c_int as isize))).val.value_.gc
                    as *mut GCUnion))
                    .ts
                    .shrlen as libc::c_ulong
            } else {
                (*((*top.offset(-(1 as libc::c_int as isize))).val.value_.gc
                    as *mut GCUnion))
                    .ts
                    .u
                    .lnglen
            };
            let mut ts: *mut TString = 0 as *mut TString;
            n = 1 as libc::c_int;
            while n < total
                && ((*top.offset(-(n as isize)).offset(-(1 as libc::c_int as isize)))
                    .val
                    .tt_ as libc::c_int & 0xf as libc::c_int == 4 as libc::c_int
                    || (*top.offset(-(n as isize)).offset(-(1 as libc::c_int as isize)))
                        .val
                        .tt_ as libc::c_int & 0xf as libc::c_int == 3 as libc::c_int
                        && {
                            luaO_tostring(
                                L,
                                &mut (*top
                                    .offset(-(n as isize))
                                    .offset(-(1 as libc::c_int as isize)))
                                    .val,
                            );
                            1 as libc::c_int != 0
                        })
            {
                let mut l: size_t = if (*((*top
                    .offset(-(n as isize))
                    .offset(-(1 as libc::c_int as isize)))
                    .val
                    .value_
                    .gc as *mut GCUnion))
                    .ts
                    .shrlen as libc::c_int != 0xff as libc::c_int
                {
                    (*((*top.offset(-(n as isize)).offset(-(1 as libc::c_int as isize)))
                        .val
                        .value_
                        .gc as *mut GCUnion))
                        .ts
                        .shrlen as libc::c_ulong
                } else {
                    (*((*top.offset(-(n as isize)).offset(-(1 as libc::c_int as isize)))
                        .val
                        .value_
                        .gc as *mut GCUnion))
                        .ts
                        .u
                        .lnglen
                };
                if ((l
                    >= (if (::core::mem::size_of::<size_t>() as libc::c_ulong)
                        < ::core::mem::size_of::<lua_Integer>() as libc::c_ulong
                    {
                        !(0 as libc::c_int as size_t)
                    } else {
                        9223372036854775807 as libc::c_longlong as size_t
                    })
                        .wrapping_sub(::core::mem::size_of::<TString>() as libc::c_ulong)
                        .wrapping_sub(tl)) as libc::c_int != 0 as libc::c_int)
                    as libc::c_int as libc::c_long != 0
                {
                    (*L).top.p = top.offset(-(total as isize));
                    luaG_runerror(
                        L,
                        b"string length overflow\0" as *const u8 as *const libc::c_char,
                    );
                }
                tl = (tl as libc::c_ulong).wrapping_add(l) as size_t as size_t;
                n += 1;
                n;
            }
            if tl <= 40 as libc::c_int as libc::c_ulong {
                let mut buff: [libc::c_char; 40] = [0; 40];
                copy2buff(top, n, buff.as_mut_ptr());
                ts = luaS_newlstr(L, buff.as_mut_ptr(), tl);
            } else {
                ts = luaS_createlngstrobj(L, tl);
                copy2buff(top, n, ((*ts).contents).as_mut_ptr());
            }
            let mut io: *mut TValue = &mut (*top.offset(-(n as isize))).val;
            let mut x_: *mut TString = ts;
            (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
            (*io)
                .tt_ = ((*x_).tt as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int)
                as lu_byte;
        }
        total -= n - 1 as libc::c_int;
        (*L).top.p = ((*L).top.p).offset(-((n - 1 as libc::c_int) as isize));
        if !(total > 1 as libc::c_int) {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaV_objlen(
    mut L: *mut lua_State,
    mut ra: StkId,
    mut rb: *const TValue,
) {
    let mut tm: *const TValue = 0 as *const TValue;
    match (*rb).tt_ as libc::c_int & 0x3f as libc::c_int {
        5 => {
            let mut h: *mut Table = &mut (*((*rb).value_.gc as *mut GCUnion)).h;
            tm = if ((*h).metatable).is_null() {
                0 as *const TValue
            } else if (*(*h).metatable).flags as libc::c_uint
                & (1 as libc::c_uint) << TM_LEN as libc::c_int != 0
            {
                0 as *const TValue
            } else {
                luaT_gettm(
                    (*h).metatable,
                    TM_LEN,
                    (*(*L).l_G).tmname[TM_LEN as libc::c_int as usize],
                )
            };
            if tm.is_null() {
                let mut io: *mut TValue = &mut (*ra).val;
                (*io).value_.i = luaH_getn(h) as lua_Integer;
                (*io)
                    .tt_ = (3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int)
                    as lu_byte;
                return;
            }
        }
        4 => {
            let mut io_0: *mut TValue = &mut (*ra).val;
            (*io_0)
                .value_
                .i = (*((*rb).value_.gc as *mut GCUnion)).ts.shrlen as lua_Integer;
            (*io_0)
                .tt_ = (3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int)
                as lu_byte;
            return;
        }
        20 => {
            let mut io_1: *mut TValue = &mut (*ra).val;
            (*io_1)
                .value_
                .i = (*((*rb).value_.gc as *mut GCUnion)).ts.u.lnglen as lua_Integer;
            (*io_1)
                .tt_ = (3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int)
                as lu_byte;
            return;
        }
        _ => {
            tm = luaT_gettmbyobj(L, rb, TM_LEN);
            if (((*tm).tt_ as libc::c_int & 0xf as libc::c_int == 0 as libc::c_int)
                as libc::c_int != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
            {
                luaG_typeerror(
                    L,
                    rb,
                    b"get length of\0" as *const u8 as *const libc::c_char,
                );
            }
        }
    }
    luaT_callTMres(L, tm, rb, rb, ra);
}
#[no_mangle]
pub unsafe extern "C" fn luaV_idiv(
    mut L: *mut lua_State,
    mut m: lua_Integer,
    mut n: lua_Integer,
) -> lua_Integer {
    if (((n as lua_Unsigned).wrapping_add(1 as libc::c_uint as libc::c_ulonglong)
        <= 1 as libc::c_uint as libc::c_ulonglong) as libc::c_int != 0 as libc::c_int)
        as libc::c_int as libc::c_long != 0
    {
        if n == 0 as libc::c_int as libc::c_longlong {
            luaG_runerror(
                L,
                b"attempt to divide by zero\0" as *const u8 as *const libc::c_char,
            );
        }
        return (0 as libc::c_int as lua_Unsigned).wrapping_sub(m as lua_Unsigned)
            as lua_Integer;
    } else {
        let mut q: lua_Integer = m / n;
        if m ^ n < 0 as libc::c_int as libc::c_longlong
            && m % n != 0 as libc::c_int as libc::c_longlong
        {
            q -= 1 as libc::c_int as libc::c_longlong;
        }
        return q;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaV_mod(
    mut L: *mut lua_State,
    mut m: lua_Integer,
    mut n: lua_Integer,
) -> lua_Integer {
    if (((n as lua_Unsigned).wrapping_add(1 as libc::c_uint as libc::c_ulonglong)
        <= 1 as libc::c_uint as libc::c_ulonglong) as libc::c_int != 0 as libc::c_int)
        as libc::c_int as libc::c_long != 0
    {
        if n == 0 as libc::c_int as libc::c_longlong {
            luaG_runerror(
                L,
                b"attempt to perform 'n%%0'\0" as *const u8 as *const libc::c_char,
            );
        }
        return 0 as libc::c_int as lua_Integer;
    } else {
        let mut r: lua_Integer = m % n;
        if r != 0 as libc::c_int as libc::c_longlong
            && r ^ n < 0 as libc::c_int as libc::c_longlong
        {
            r += n;
        }
        return r;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaV_modf(
    mut L: *mut lua_State,
    mut m: lua_Number,
    mut n: lua_Number,
) -> lua_Number {
    let mut r: lua_Number = 0.;
    r = fmod(m, n);
    if if r > 0 as libc::c_int as libc::c_double {
        (n < 0 as libc::c_int as libc::c_double) as libc::c_int
    } else {
        (r < 0 as libc::c_int as libc::c_double
            && n > 0 as libc::c_int as libc::c_double) as libc::c_int
    } != 0
    {
        r += n;
    }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn luaV_shiftl(
    mut x: lua_Integer,
    mut y: lua_Integer,
) -> lua_Integer {
    if y < 0 as libc::c_int as libc::c_longlong {
        if y
            <= -((::core::mem::size_of::<lua_Integer>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong) as libc::c_int)
                as libc::c_longlong
        {
            return 0 as libc::c_int as lua_Integer
        } else {
            return (x as lua_Unsigned >> -y as lua_Unsigned) as lua_Integer
        }
    } else if y
        >= (::core::mem::size_of::<lua_Integer>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong) as libc::c_int
            as libc::c_longlong
    {
        return 0 as libc::c_int as lua_Integer
    } else {
        return ((x as lua_Unsigned) << y as lua_Unsigned) as lua_Integer
    };
}
unsafe extern "C" fn pushclosure(
    mut L: *mut lua_State,
    mut p: *mut Proto,
    mut encup: *mut *mut UpVal,
    mut base: StkId,
    mut ra: StkId,
) {
    let mut nup: libc::c_int = (*p).sizeupvalues;
    let mut uv: *mut Upvaldesc = (*p).upvalues;
    let mut i: libc::c_int = 0;
    let mut ncl: *mut LClosure = luaF_newLclosure(L, nup);
    (*ncl).p = p;
    let mut io: *mut TValue = &mut (*ra).val;
    let mut x_: *mut LClosure = ncl;
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
    (*io)
        .tt_ = (6 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 6 as libc::c_int) as lu_byte;
    i = 0 as libc::c_int;
    while i < nup {
        if (*uv.offset(i as isize)).instack != 0 {
            let ref mut fresh0 = *((*ncl).upvals).as_mut_ptr().offset(i as isize);
            *fresh0 = luaF_findupval(
                L,
                base.offset((*uv.offset(i as isize)).idx as libc::c_int as isize),
            );
        } else {
            let ref mut fresh1 = *((*ncl).upvals).as_mut_ptr().offset(i as isize);
            *fresh1 = *encup.offset((*uv.offset(i as isize)).idx as isize);
        }
        if (*ncl).marked as libc::c_int & (1 as libc::c_int) << 5 as libc::c_int != 0
            && (**((*ncl).upvals).as_mut_ptr().offset(i as isize)).marked as libc::c_int
                & ((1 as libc::c_int) << 3 as libc::c_int
                    | (1 as libc::c_int) << 4 as libc::c_int) != 0
        {
            luaC_barrier_(
                L,
                &mut (*(ncl as *mut GCUnion)).gc,
                &mut (*(*((*ncl).upvals).as_mut_ptr().offset(i as isize)
                    as *mut GCUnion))
                    .gc,
            );
        } else {};
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn luaV_finishOp(mut L: *mut lua_State) {
    let mut ci: *mut CallInfo = (*L).ci;
    let mut base: StkId = ((*ci).func.p).offset(1 as libc::c_int as isize);
    let mut inst: Instruction = *((*ci).u.l.savedpc)
        .offset(-(1 as libc::c_int as isize));
    let mut op: OpCode = (inst >> 0 as libc::c_int
        & !(!(0 as libc::c_int as Instruction) << 7 as libc::c_int) << 0 as libc::c_int)
        as OpCode;
    match op as libc::c_uint {
        46 | 47 | 48 => {
            let mut io1: *mut TValue = &mut (*base
                .offset(
                    (*((*ci).u.l.savedpc).offset(-(2 as libc::c_int as isize))
                        >> 0 as libc::c_int + 7 as libc::c_int
                        & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                            << 0 as libc::c_int) as libc::c_int as isize,
                ))
                .val;
            (*L).top.p = ((*L).top.p).offset(-1);
            let mut io2: *const TValue = &mut (*(*L).top.p).val;
            (*io1).value_ = (*io2).value_;
            (*io1).tt_ = (*io2).tt_;
        }
        49 | 50 | 52 | 11 | 12 | 13 | 14 | 20 => {
            let mut io1_0: *mut TValue = &mut (*base
                .offset(
                    (inst >> 0 as libc::c_int + 7 as libc::c_int
                        & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                            << 0 as libc::c_int) as libc::c_int as isize,
                ))
                .val;
            (*L).top.p = ((*L).top.p).offset(-1);
            let mut io2_0: *const TValue = &mut (*(*L).top.p).val;
            (*io1_0).value_ = (*io2_0).value_;
            (*io1_0).tt_ = (*io2_0).tt_;
        }
        58 | 59 | 62 | 63 | 64 | 65 | 57 => {
            let mut res: libc::c_int = !((*((*L).top.p)
                .offset(-(1 as libc::c_int as isize)))
                .val
                .tt_ as libc::c_int
                == 1 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                || (*((*L).top.p).offset(-(1 as libc::c_int as isize))).val.tt_
                    as libc::c_int & 0xf as libc::c_int == 0 as libc::c_int)
                as libc::c_int;
            (*L).top.p = ((*L).top.p).offset(-1);
            (*L).top.p;
            if res
                != (inst >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                    & !(!(0 as libc::c_int as Instruction) << 1 as libc::c_int)
                        << 0 as libc::c_int) as libc::c_int
            {
                (*ci).u.l.savedpc = ((*ci).u.l.savedpc).offset(1);
                (*ci).u.l.savedpc;
            }
        }
        53 => {
            let mut top: StkId = ((*L).top.p).offset(-(1 as libc::c_int as isize));
            let mut a: libc::c_int = (inst >> 0 as libc::c_int + 7 as libc::c_int
                & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                    << 0 as libc::c_int) as libc::c_int;
            let mut total: libc::c_int = top
                .offset(-(1 as libc::c_int as isize))
                .offset_from(base.offset(a as isize)) as libc::c_long as libc::c_int;
            let mut io1_1: *mut TValue = &mut (*top.offset(-(2 as libc::c_int as isize)))
                .val;
            let mut io2_1: *const TValue = &mut (*top).val;
            (*io1_1).value_ = (*io2_1).value_;
            (*io1_1).tt_ = (*io2_1).tt_;
            (*L).top.p = top.offset(-(1 as libc::c_int as isize));
            luaV_concat(L, total);
        }
        54 => {
            (*ci).u.l.savedpc = ((*ci).u.l.savedpc).offset(-1);
            (*ci).u.l.savedpc;
        }
        70 => {
            let mut ra: StkId = base
                .offset(
                    (inst >> 0 as libc::c_int + 7 as libc::c_int
                        & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                            << 0 as libc::c_int) as libc::c_int as isize,
                );
            (*L).top.p = ra.offset((*ci).u2.nres as isize);
            (*ci).u.l.savedpc = ((*ci).u.l.savedpc).offset(-1);
            (*ci).u.l.savedpc;
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaV_execute(mut L: *mut lua_State, mut ci: *mut CallInfo) {
    let mut i: Instruction = 0;
    let mut ra_65: StkId = 0 as *mut StackValue;
    let mut newci: *mut CallInfo = 0 as *mut CallInfo;
    let mut b_4: libc::c_int = 0;
    let mut nresults: libc::c_int = 0;
    let mut current_block: u64;
    let mut cl: *mut LClosure = 0 as *mut LClosure;
    let mut k: *mut TValue = 0 as *mut TValue;
    let mut base: StkId = 0 as *mut StackValue;
    let mut pc: *const Instruction = 0 as *const Instruction;
    let mut trap: libc::c_int = 0;
    '_startfunc: loop {
        trap = (*L).hookmask;
        '_returning: loop {
            cl = &mut (*((*(*ci).func.p).val.value_.gc as *mut GCUnion)).cl.l;
            k = (*(*cl).p).k;
            pc = (*ci).u.l.savedpc;
            if (trap != 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
                trap = luaG_tracecall(L);
            }
            base = ((*ci).func.p).offset(1 as libc::c_int as isize);
            loop {
                i = 0;
                if (trap != 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
                    trap = luaG_traceexec(L, pc);
                    base = ((*ci).func.p).offset(1 as libc::c_int as isize);
                }
                let fresh2 = pc;
                pc = pc.offset(1);
                i = *fresh2;
                match (i >> 0 as libc::c_int
                    & !(!(0 as libc::c_int as Instruction) << 7 as libc::c_int)
                        << 0 as libc::c_int) as OpCode as libc::c_uint
                {
                    0 => {
                        let mut ra: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut io1: *mut TValue = &mut (*ra).val;
                        let mut io2: *const TValue = &mut (*base
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            ))
                            .val;
                        (*io1).value_ = (*io2).value_;
                        (*io1).tt_ = (*io2).tt_;
                        continue;
                    }
                    1 => {
                        let mut ra_0: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut b: lua_Integer = ((i
                            >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                            & !(!(0 as libc::c_int as Instruction)
                                << 8 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int)
                                << 0 as libc::c_int) as libc::c_int
                            - (((1 as libc::c_int)
                                << 8 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int)
                                - 1 as libc::c_int >> 1 as libc::c_int)) as lua_Integer;
                        let mut io: *mut TValue = &mut (*ra_0).val;
                        (*io).value_.i = b;
                        (*io)
                            .tt_ = (3 as libc::c_int
                            | (0 as libc::c_int) << 4 as libc::c_int) as lu_byte;
                        continue;
                    }
                    2 => {
                        let mut ra_1: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut b_0: libc::c_int = (i
                            >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                            & !(!(0 as libc::c_int as Instruction)
                                << 8 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int)
                                << 0 as libc::c_int) as libc::c_int
                            - (((1 as libc::c_int)
                                << 8 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int)
                                - 1 as libc::c_int >> 1 as libc::c_int);
                        let mut io_0: *mut TValue = &mut (*ra_1).val;
                        (*io_0).value_.n = b_0 as lua_Number;
                        (*io_0)
                            .tt_ = (3 as libc::c_int
                            | (1 as libc::c_int) << 4 as libc::c_int) as lu_byte;
                        continue;
                    }
                    3 => {
                        let mut ra_2: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut rb: *mut TValue = k
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction)
                                        << 8 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut io1_0: *mut TValue = &mut (*ra_2).val;
                        let mut io2_0: *const TValue = rb;
                        (*io1_0).value_ = (*io2_0).value_;
                        (*io1_0).tt_ = (*io2_0).tt_;
                        continue;
                    }
                    4 => {
                        let mut ra_3: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut rb_0: *mut TValue = 0 as *mut TValue;
                        rb_0 = k
                            .offset(
                                (*pc >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction)
                                        << 8 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int
                                            + 8 as libc::c_int) << 0 as libc::c_int) as libc::c_int
                                    as isize,
                            );
                        pc = pc.offset(1);
                        pc;
                        let mut io1_1: *mut TValue = &mut (*ra_3).val;
                        let mut io2_1: *const TValue = rb_0;
                        (*io1_1).value_ = (*io2_1).value_;
                        (*io1_1).tt_ = (*io2_1).tt_;
                        continue;
                    }
                    5 => {
                        let mut ra_4: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        (*ra_4)
                            .val
                            .tt_ = (1 as libc::c_int
                            | (0 as libc::c_int) << 4 as libc::c_int) as lu_byte;
                        continue;
                    }
                    6 => {
                        let mut ra_5: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        (*ra_5)
                            .val
                            .tt_ = (1 as libc::c_int
                            | (0 as libc::c_int) << 4 as libc::c_int) as lu_byte;
                        pc = pc.offset(1);
                        pc;
                        continue;
                    }
                    7 => {
                        let mut ra_6: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        (*ra_6)
                            .val
                            .tt_ = (1 as libc::c_int
                            | (1 as libc::c_int) << 4 as libc::c_int) as lu_byte;
                        continue;
                    }
                    8 => {
                        let mut ra_7: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut b_1: libc::c_int = (i
                            >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                + 1 as libc::c_int
                            & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                << 0 as libc::c_int) as libc::c_int;
                        loop {
                            let fresh3 = ra_7;
                            ra_7 = ra_7.offset(1);
                            (*fresh3)
                                .val
                                .tt_ = (0 as libc::c_int
                                | (0 as libc::c_int) << 4 as libc::c_int) as lu_byte;
                            let fresh4 = b_1;
                            b_1 = b_1 - 1;
                            if !(fresh4 != 0) {
                                break;
                            }
                        }
                        continue;
                    }
                    9 => {
                        let mut ra_8: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut b_2: libc::c_int = (i
                            >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                + 1 as libc::c_int
                            & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                << 0 as libc::c_int) as libc::c_int;
                        let mut io1_2: *mut TValue = &mut (*ra_8).val;
                        let mut io2_2: *const TValue = (**((*cl).upvals)
                            .as_mut_ptr()
                            .offset(b_2 as isize))
                            .v
                            .p;
                        (*io1_2).value_ = (*io2_2).value_;
                        (*io1_2).tt_ = (*io2_2).tt_;
                        continue;
                    }
                    10 => {
                        let mut ra_9: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut uv: *mut UpVal = *((*cl).upvals)
                            .as_mut_ptr()
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut io1_3: *mut TValue = (*uv).v.p;
                        let mut io2_3: *const TValue = &mut (*ra_9).val;
                        (*io1_3).value_ = (*io2_3).value_;
                        (*io1_3).tt_ = (*io2_3).tt_;
                        if (*ra_9).val.tt_ as libc::c_int
                            & (1 as libc::c_int) << 6 as libc::c_int != 0
                        {
                            if (*uv).marked as libc::c_int
                                & (1 as libc::c_int) << 5 as libc::c_int != 0
                                && (*(*ra_9).val.value_.gc).marked as libc::c_int
                                    & ((1 as libc::c_int) << 3 as libc::c_int
                                        | (1 as libc::c_int) << 4 as libc::c_int) != 0
                            {
                                luaC_barrier_(
                                    L,
                                    &mut (*(uv as *mut GCUnion)).gc,
                                    &mut (*((*ra_9).val.value_.gc as *mut GCUnion)).gc,
                                );
                            } else {};
                        } else {};
                        continue;
                    }
                    11 => {
                        let mut ra_10: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut slot: *const TValue = 0 as *const TValue;
                        let mut upval: *mut TValue = (**((*cl).upvals)
                            .as_mut_ptr()
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            ))
                            .v
                            .p;
                        let mut rc: *mut TValue = k
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int + 8 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut key: *mut TString = &mut (*((*rc).value_.gc
                            as *mut GCUnion))
                            .ts;
                        if if !((*upval).tt_ as libc::c_int
                            == 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                                | (1 as libc::c_int) << 6 as libc::c_int)
                        {
                            slot = 0 as *const TValue;
                            0 as libc::c_int
                        } else {
                            slot = luaH_getshortstr(
                                &mut (*((*upval).value_.gc as *mut GCUnion)).h,
                                key,
                            );
                            !((*slot).tt_ as libc::c_int & 0xf as libc::c_int
                                == 0 as libc::c_int) as libc::c_int
                        } != 0
                        {
                            let mut io1_4: *mut TValue = &mut (*ra_10).val;
                            let mut io2_4: *const TValue = slot;
                            (*io1_4).value_ = (*io2_4).value_;
                            (*io1_4).tt_ = (*io2_4).tt_;
                        } else {
                            (*ci).u.l.savedpc = pc;
                            (*L).top.p = (*ci).top.p;
                            luaV_finishget(L, upval, rc, ra_10, slot);
                            trap = (*ci).u.l.trap;
                        }
                        continue;
                    }
                    12 => {
                        let mut ra_11: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut slot_0: *const TValue = 0 as *const TValue;
                        let mut rb_1: *mut TValue = &mut (*base
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            ))
                            .val;
                        let mut rc_0: *mut TValue = &mut (*base
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int + 8 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            ))
                            .val;
                        let mut n: lua_Unsigned = 0;
                        if if (*rc_0).tt_ as libc::c_int
                            == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                        {
                            n = (*rc_0).value_.i as lua_Unsigned;
                            (if !((*rb_1).tt_ as libc::c_int
                                == 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                                    | (1 as libc::c_int) << 6 as libc::c_int)
                            {
                                slot_0 = 0 as *const TValue;
                                0 as libc::c_int
                            } else {
                                slot_0 = (if n
                                    .wrapping_sub(1 as libc::c_uint as libc::c_ulonglong)
                                    < (*((*rb_1).value_.gc as *mut GCUnion)).h.alimit
                                        as libc::c_ulonglong
                                {
                                    &mut *((*((*rb_1).value_.gc as *mut GCUnion)).h.array)
                                        .offset(
                                            n.wrapping_sub(1 as libc::c_int as libc::c_ulonglong)
                                                as isize,
                                        ) as *mut TValue as *const TValue
                                } else {
                                    luaH_getint(
                                        &mut (*((*rb_1).value_.gc as *mut GCUnion)).h,
                                        n as lua_Integer,
                                    )
                                });
                                !((*slot_0).tt_ as libc::c_int & 0xf as libc::c_int
                                    == 0 as libc::c_int) as libc::c_int
                            })
                        } else if !((*rb_1).tt_ as libc::c_int
                            == 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                                | (1 as libc::c_int) << 6 as libc::c_int)
                        {
                            slot_0 = 0 as *const TValue;
                            0 as libc::c_int
                        } else {
                            slot_0 = luaH_get(
                                &mut (*((*rb_1).value_.gc as *mut GCUnion)).h,
                                rc_0,
                            );
                            !((*slot_0).tt_ as libc::c_int & 0xf as libc::c_int
                                == 0 as libc::c_int) as libc::c_int
                        } != 0
                        {
                            let mut io1_5: *mut TValue = &mut (*ra_11).val;
                            let mut io2_5: *const TValue = slot_0;
                            (*io1_5).value_ = (*io2_5).value_;
                            (*io1_5).tt_ = (*io2_5).tt_;
                        } else {
                            (*ci).u.l.savedpc = pc;
                            (*L).top.p = (*ci).top.p;
                            luaV_finishget(L, rb_1, rc_0, ra_11, slot_0);
                            trap = (*ci).u.l.trap;
                        }
                        continue;
                    }
                    13 => {
                        let mut ra_12: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut slot_1: *const TValue = 0 as *const TValue;
                        let mut rb_2: *mut TValue = &mut (*base
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            ))
                            .val;
                        let mut c: libc::c_int = (i
                            >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                + 1 as libc::c_int + 8 as libc::c_int
                            & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                << 0 as libc::c_int) as libc::c_int;
                        if if !((*rb_2).tt_ as libc::c_int
                            == 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                                | (1 as libc::c_int) << 6 as libc::c_int)
                        {
                            slot_1 = 0 as *const TValue;
                            0 as libc::c_int
                        } else {
                            slot_1 = (if (c as lua_Unsigned)
                                .wrapping_sub(1 as libc::c_uint as libc::c_ulonglong)
                                < (*((*rb_2).value_.gc as *mut GCUnion)).h.alimit
                                    as libc::c_ulonglong
                            {
                                &mut *((*((*rb_2).value_.gc as *mut GCUnion)).h.array)
                                    .offset((c - 1 as libc::c_int) as isize) as *mut TValue
                                    as *const TValue
                            } else {
                                luaH_getint(
                                    &mut (*((*rb_2).value_.gc as *mut GCUnion)).h,
                                    c as lua_Integer,
                                )
                            });
                            !((*slot_1).tt_ as libc::c_int & 0xf as libc::c_int
                                == 0 as libc::c_int) as libc::c_int
                        } != 0
                        {
                            let mut io1_6: *mut TValue = &mut (*ra_12).val;
                            let mut io2_6: *const TValue = slot_1;
                            (*io1_6).value_ = (*io2_6).value_;
                            (*io1_6).tt_ = (*io2_6).tt_;
                        } else {
                            let mut key_0: TValue = TValue {
                                value_: Value { gc: 0 as *mut GCObject },
                                tt_: 0,
                            };
                            let mut io_1: *mut TValue = &mut key_0;
                            (*io_1).value_.i = c as lua_Integer;
                            (*io_1)
                                .tt_ = (3 as libc::c_int
                                | (0 as libc::c_int) << 4 as libc::c_int) as lu_byte;
                            (*ci).u.l.savedpc = pc;
                            (*L).top.p = (*ci).top.p;
                            luaV_finishget(L, rb_2, &mut key_0, ra_12, slot_1);
                            trap = (*ci).u.l.trap;
                        }
                        continue;
                    }
                    14 => {
                        let mut ra_13: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut slot_2: *const TValue = 0 as *const TValue;
                        let mut rb_3: *mut TValue = &mut (*base
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            ))
                            .val;
                        let mut rc_1: *mut TValue = k
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int + 8 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut key_1: *mut TString = &mut (*((*rc_1).value_.gc
                            as *mut GCUnion))
                            .ts;
                        if if !((*rb_3).tt_ as libc::c_int
                            == 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                                | (1 as libc::c_int) << 6 as libc::c_int)
                        {
                            slot_2 = 0 as *const TValue;
                            0 as libc::c_int
                        } else {
                            slot_2 = luaH_getshortstr(
                                &mut (*((*rb_3).value_.gc as *mut GCUnion)).h,
                                key_1,
                            );
                            !((*slot_2).tt_ as libc::c_int & 0xf as libc::c_int
                                == 0 as libc::c_int) as libc::c_int
                        } != 0
                        {
                            let mut io1_7: *mut TValue = &mut (*ra_13).val;
                            let mut io2_7: *const TValue = slot_2;
                            (*io1_7).value_ = (*io2_7).value_;
                            (*io1_7).tt_ = (*io2_7).tt_;
                        } else {
                            (*ci).u.l.savedpc = pc;
                            (*L).top.p = (*ci).top.p;
                            luaV_finishget(L, rb_3, rc_1, ra_13, slot_2);
                            trap = (*ci).u.l.trap;
                        }
                        continue;
                    }
                    15 => {
                        let mut slot_3: *const TValue = 0 as *const TValue;
                        let mut upval_0: *mut TValue = (**((*cl).upvals)
                            .as_mut_ptr()
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            ))
                            .v
                            .p;
                        let mut rb_4: *mut TValue = k
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut rc_2: *mut TValue = if (i
                            & (1 as libc::c_uint)
                                << 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int)
                            as libc::c_int != 0
                        {
                            k.offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int + 8 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            )
                        } else {
                            &mut (*base
                                .offset(
                                    (i
                                        >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                            + 1 as libc::c_int + 8 as libc::c_int
                                        & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                            << 0 as libc::c_int) as libc::c_int as isize,
                                ))
                                .val
                        };
                        let mut key_2: *mut TString = &mut (*((*rb_4).value_.gc
                            as *mut GCUnion))
                            .ts;
                        if if !((*upval_0).tt_ as libc::c_int
                            == 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                                | (1 as libc::c_int) << 6 as libc::c_int)
                        {
                            slot_3 = 0 as *const TValue;
                            0 as libc::c_int
                        } else {
                            slot_3 = luaH_getshortstr(
                                &mut (*((*upval_0).value_.gc as *mut GCUnion)).h,
                                key_2,
                            );
                            !((*slot_3).tt_ as libc::c_int & 0xf as libc::c_int
                                == 0 as libc::c_int) as libc::c_int
                        } != 0
                        {
                            let mut io1_8: *mut TValue = slot_3 as *mut TValue;
                            let mut io2_8: *const TValue = rc_2;
                            (*io1_8).value_ = (*io2_8).value_;
                            (*io1_8).tt_ = (*io2_8).tt_;
                            if (*rc_2).tt_ as libc::c_int
                                & (1 as libc::c_int) << 6 as libc::c_int != 0
                            {
                                if (*(*upval_0).value_.gc).marked as libc::c_int
                                    & (1 as libc::c_int) << 5 as libc::c_int != 0
                                    && (*(*rc_2).value_.gc).marked as libc::c_int
                                        & ((1 as libc::c_int) << 3 as libc::c_int
                                            | (1 as libc::c_int) << 4 as libc::c_int) != 0
                                {
                                    luaC_barrierback_(L, (*upval_0).value_.gc);
                                } else {};
                            } else {};
                        } else {
                            (*ci).u.l.savedpc = pc;
                            (*L).top.p = (*ci).top.p;
                            luaV_finishset(L, upval_0, rb_4, rc_2, slot_3);
                            trap = (*ci).u.l.trap;
                        }
                        continue;
                    }
                    16 => {
                        let mut ra_14: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut slot_4: *const TValue = 0 as *const TValue;
                        let mut rb_5: *mut TValue = &mut (*base
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            ))
                            .val;
                        let mut rc_3: *mut TValue = if (i
                            & (1 as libc::c_uint)
                                << 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int)
                            as libc::c_int != 0
                        {
                            k.offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int + 8 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            )
                        } else {
                            &mut (*base
                                .offset(
                                    (i
                                        >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                            + 1 as libc::c_int + 8 as libc::c_int
                                        & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                            << 0 as libc::c_int) as libc::c_int as isize,
                                ))
                                .val
                        };
                        let mut n_0: lua_Unsigned = 0;
                        if if (*rb_5).tt_ as libc::c_int
                            == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                        {
                            n_0 = (*rb_5).value_.i as lua_Unsigned;
                            (if !((*ra_14).val.tt_ as libc::c_int
                                == 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                                    | (1 as libc::c_int) << 6 as libc::c_int)
                            {
                                slot_4 = 0 as *const TValue;
                                0 as libc::c_int
                            } else {
                                slot_4 = (if n_0
                                    .wrapping_sub(1 as libc::c_uint as libc::c_ulonglong)
                                    < (*((*ra_14).val.value_.gc as *mut GCUnion)).h.alimit
                                        as libc::c_ulonglong
                                {
                                    &mut *((*((*ra_14).val.value_.gc as *mut GCUnion)).h.array)
                                        .offset(
                                            n_0.wrapping_sub(1 as libc::c_int as libc::c_ulonglong)
                                                as isize,
                                        ) as *mut TValue as *const TValue
                                } else {
                                    luaH_getint(
                                        &mut (*((*ra_14).val.value_.gc as *mut GCUnion)).h,
                                        n_0 as lua_Integer,
                                    )
                                });
                                !((*slot_4).tt_ as libc::c_int & 0xf as libc::c_int
                                    == 0 as libc::c_int) as libc::c_int
                            })
                        } else if !((*ra_14).val.tt_ as libc::c_int
                            == 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                                | (1 as libc::c_int) << 6 as libc::c_int)
                        {
                            slot_4 = 0 as *const TValue;
                            0 as libc::c_int
                        } else {
                            slot_4 = luaH_get(
                                &mut (*((*ra_14).val.value_.gc as *mut GCUnion)).h,
                                rb_5,
                            );
                            !((*slot_4).tt_ as libc::c_int & 0xf as libc::c_int
                                == 0 as libc::c_int) as libc::c_int
                        } != 0
                        {
                            let mut io1_9: *mut TValue = slot_4 as *mut TValue;
                            let mut io2_9: *const TValue = rc_3;
                            (*io1_9).value_ = (*io2_9).value_;
                            (*io1_9).tt_ = (*io2_9).tt_;
                            if (*rc_3).tt_ as libc::c_int
                                & (1 as libc::c_int) << 6 as libc::c_int != 0
                            {
                                if (*(*ra_14).val.value_.gc).marked as libc::c_int
                                    & (1 as libc::c_int) << 5 as libc::c_int != 0
                                    && (*(*rc_3).value_.gc).marked as libc::c_int
                                        & ((1 as libc::c_int) << 3 as libc::c_int
                                            | (1 as libc::c_int) << 4 as libc::c_int) != 0
                                {
                                    luaC_barrierback_(L, (*ra_14).val.value_.gc);
                                } else {};
                            } else {};
                        } else {
                            (*ci).u.l.savedpc = pc;
                            (*L).top.p = (*ci).top.p;
                            luaV_finishset(L, &mut (*ra_14).val, rb_5, rc_3, slot_4);
                            trap = (*ci).u.l.trap;
                        }
                        continue;
                    }
                    17 => {
                        let mut ra_15: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut slot_5: *const TValue = 0 as *const TValue;
                        let mut c_0: libc::c_int = (i
                            >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                + 1 as libc::c_int
                            & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                << 0 as libc::c_int) as libc::c_int;
                        let mut rc_4: *mut TValue = if (i
                            & (1 as libc::c_uint)
                                << 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int)
                            as libc::c_int != 0
                        {
                            k.offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int + 8 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            )
                        } else {
                            &mut (*base
                                .offset(
                                    (i
                                        >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                            + 1 as libc::c_int + 8 as libc::c_int
                                        & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                            << 0 as libc::c_int) as libc::c_int as isize,
                                ))
                                .val
                        };
                        if if !((*ra_15).val.tt_ as libc::c_int
                            == 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                                | (1 as libc::c_int) << 6 as libc::c_int)
                        {
                            slot_5 = 0 as *const TValue;
                            0 as libc::c_int
                        } else {
                            slot_5 = (if (c_0 as lua_Unsigned)
                                .wrapping_sub(1 as libc::c_uint as libc::c_ulonglong)
                                < (*((*ra_15).val.value_.gc as *mut GCUnion)).h.alimit
                                    as libc::c_ulonglong
                            {
                                &mut *((*((*ra_15).val.value_.gc as *mut GCUnion)).h.array)
                                    .offset((c_0 - 1 as libc::c_int) as isize) as *mut TValue
                                    as *const TValue
                            } else {
                                luaH_getint(
                                    &mut (*((*ra_15).val.value_.gc as *mut GCUnion)).h,
                                    c_0 as lua_Integer,
                                )
                            });
                            !((*slot_5).tt_ as libc::c_int & 0xf as libc::c_int
                                == 0 as libc::c_int) as libc::c_int
                        } != 0
                        {
                            let mut io1_10: *mut TValue = slot_5 as *mut TValue;
                            let mut io2_10: *const TValue = rc_4;
                            (*io1_10).value_ = (*io2_10).value_;
                            (*io1_10).tt_ = (*io2_10).tt_;
                            if (*rc_4).tt_ as libc::c_int
                                & (1 as libc::c_int) << 6 as libc::c_int != 0
                            {
                                if (*(*ra_15).val.value_.gc).marked as libc::c_int
                                    & (1 as libc::c_int) << 5 as libc::c_int != 0
                                    && (*(*rc_4).value_.gc).marked as libc::c_int
                                        & ((1 as libc::c_int) << 3 as libc::c_int
                                            | (1 as libc::c_int) << 4 as libc::c_int) != 0
                                {
                                    luaC_barrierback_(L, (*ra_15).val.value_.gc);
                                } else {};
                            } else {};
                        } else {
                            let mut key_3: TValue = TValue {
                                value_: Value { gc: 0 as *mut GCObject },
                                tt_: 0,
                            };
                            let mut io_2: *mut TValue = &mut key_3;
                            (*io_2).value_.i = c_0 as lua_Integer;
                            (*io_2)
                                .tt_ = (3 as libc::c_int
                                | (0 as libc::c_int) << 4 as libc::c_int) as lu_byte;
                            (*ci).u.l.savedpc = pc;
                            (*L).top.p = (*ci).top.p;
                            luaV_finishset(
                                L,
                                &mut (*ra_15).val,
                                &mut key_3,
                                rc_4,
                                slot_5,
                            );
                            trap = (*ci).u.l.trap;
                        }
                        continue;
                    }
                    18 => {
                        let mut ra_16: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut slot_6: *const TValue = 0 as *const TValue;
                        let mut rb_6: *mut TValue = k
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut rc_5: *mut TValue = if (i
                            & (1 as libc::c_uint)
                                << 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int)
                            as libc::c_int != 0
                        {
                            k.offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int + 8 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            )
                        } else {
                            &mut (*base
                                .offset(
                                    (i
                                        >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                            + 1 as libc::c_int + 8 as libc::c_int
                                        & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                            << 0 as libc::c_int) as libc::c_int as isize,
                                ))
                                .val
                        };
                        let mut key_4: *mut TString = &mut (*((*rb_6).value_.gc
                            as *mut GCUnion))
                            .ts;
                        if if !((*ra_16).val.tt_ as libc::c_int
                            == 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                                | (1 as libc::c_int) << 6 as libc::c_int)
                        {
                            slot_6 = 0 as *const TValue;
                            0 as libc::c_int
                        } else {
                            slot_6 = luaH_getshortstr(
                                &mut (*((*ra_16).val.value_.gc as *mut GCUnion)).h,
                                key_4,
                            );
                            !((*slot_6).tt_ as libc::c_int & 0xf as libc::c_int
                                == 0 as libc::c_int) as libc::c_int
                        } != 0
                        {
                            let mut io1_11: *mut TValue = slot_6 as *mut TValue;
                            let mut io2_11: *const TValue = rc_5;
                            (*io1_11).value_ = (*io2_11).value_;
                            (*io1_11).tt_ = (*io2_11).tt_;
                            if (*rc_5).tt_ as libc::c_int
                                & (1 as libc::c_int) << 6 as libc::c_int != 0
                            {
                                if (*(*ra_16).val.value_.gc).marked as libc::c_int
                                    & (1 as libc::c_int) << 5 as libc::c_int != 0
                                    && (*(*rc_5).value_.gc).marked as libc::c_int
                                        & ((1 as libc::c_int) << 3 as libc::c_int
                                            | (1 as libc::c_int) << 4 as libc::c_int) != 0
                                {
                                    luaC_barrierback_(L, (*ra_16).val.value_.gc);
                                } else {};
                            } else {};
                        } else {
                            (*ci).u.l.savedpc = pc;
                            (*L).top.p = (*ci).top.p;
                            luaV_finishset(L, &mut (*ra_16).val, rb_6, rc_5, slot_6);
                            trap = (*ci).u.l.trap;
                        }
                        continue;
                    }
                    19 => {
                        let mut ra_17: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut b_3: libc::c_int = (i
                            >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                + 1 as libc::c_int
                            & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                << 0 as libc::c_int) as libc::c_int;
                        let mut c_1: libc::c_int = (i
                            >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                + 1 as libc::c_int + 8 as libc::c_int
                            & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                << 0 as libc::c_int) as libc::c_int;
                        let mut t: *mut Table = 0 as *mut Table;
                        if b_3 > 0 as libc::c_int {
                            b_3 = (1 as libc::c_int) << b_3 - 1 as libc::c_int;
                        }
                        if (i
                            & (1 as libc::c_uint)
                                << 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int)
                            as libc::c_int != 0
                        {
                            c_1
                                += (*pc >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction)
                                        << 8 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int
                                            + 8 as libc::c_int) << 0 as libc::c_int) as libc::c_int
                                    * (((1 as libc::c_int) << 8 as libc::c_int)
                                        - 1 as libc::c_int + 1 as libc::c_int);
                        }
                        pc = pc.offset(1);
                        pc;
                        (*L).top.p = ra_17.offset(1 as libc::c_int as isize);
                        t = luaH_new(L);
                        let mut io_3: *mut TValue = &mut (*ra_17).val;
                        let mut x_: *mut Table = t;
                        (*io_3).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
                        (*io_3)
                            .tt_ = (5 as libc::c_int
                            | (0 as libc::c_int) << 4 as libc::c_int
                            | (1 as libc::c_int) << 6 as libc::c_int) as lu_byte;
                        if b_3 != 0 as libc::c_int || c_1 != 0 as libc::c_int {
                            luaH_resize(L, t, c_1 as libc::c_uint, b_3 as libc::c_uint);
                        }
                        if (*(*L).l_G).GCdebt > 0 as libc::c_int as libc::c_long {
                            (*ci).u.l.savedpc = pc;
                            (*L).top.p = ra_17.offset(1 as libc::c_int as isize);
                            luaC_step(L);
                            trap = (*ci).u.l.trap;
                        }
                        continue;
                    }
                    20 => {
                        let mut ra_18: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut slot_7: *const TValue = 0 as *const TValue;
                        let mut rb_7: *mut TValue = &mut (*base
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            ))
                            .val;
                        let mut rc_6: *mut TValue = if (i
                            & (1 as libc::c_uint)
                                << 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int)
                            as libc::c_int != 0
                        {
                            k.offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int + 8 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            )
                        } else {
                            &mut (*base
                                .offset(
                                    (i
                                        >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                            + 1 as libc::c_int + 8 as libc::c_int
                                        & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                            << 0 as libc::c_int) as libc::c_int as isize,
                                ))
                                .val
                        };
                        let mut key_5: *mut TString = &mut (*((*rc_6).value_.gc
                            as *mut GCUnion))
                            .ts;
                        let mut io1_12: *mut TValue = &mut (*ra_18
                            .offset(1 as libc::c_int as isize))
                            .val;
                        let mut io2_12: *const TValue = rb_7;
                        (*io1_12).value_ = (*io2_12).value_;
                        (*io1_12).tt_ = (*io2_12).tt_;
                        if if !((*rb_7).tt_ as libc::c_int
                            == 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                                | (1 as libc::c_int) << 6 as libc::c_int)
                        {
                            slot_7 = 0 as *const TValue;
                            0 as libc::c_int
                        } else {
                            slot_7 = luaH_getstr(
                                &mut (*((*rb_7).value_.gc as *mut GCUnion)).h,
                                key_5,
                            );
                            !((*slot_7).tt_ as libc::c_int & 0xf as libc::c_int
                                == 0 as libc::c_int) as libc::c_int
                        } != 0
                        {
                            let mut io1_13: *mut TValue = &mut (*ra_18).val;
                            let mut io2_13: *const TValue = slot_7;
                            (*io1_13).value_ = (*io2_13).value_;
                            (*io1_13).tt_ = (*io2_13).tt_;
                        } else {
                            (*ci).u.l.savedpc = pc;
                            (*L).top.p = (*ci).top.p;
                            luaV_finishget(L, rb_7, rc_6, ra_18, slot_7);
                            trap = (*ci).u.l.trap;
                        }
                        continue;
                    }
                    21 => {
                        let mut ra_19: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut v1: *mut TValue = &mut (*base
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            ))
                            .val;
                        let mut imm: libc::c_int = (i
                            >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                + 1 as libc::c_int + 8 as libc::c_int
                            & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                << 0 as libc::c_int) as libc::c_int
                            - (((1 as libc::c_int) << 8 as libc::c_int)
                                - 1 as libc::c_int >> 1 as libc::c_int);
                        if (*v1).tt_ as libc::c_int
                            == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                        {
                            let mut iv1: lua_Integer = (*v1).value_.i;
                            pc = pc.offset(1);
                            pc;
                            let mut io_4: *mut TValue = &mut (*ra_19).val;
                            (*io_4)
                                .value_
                                .i = (iv1 as lua_Unsigned).wrapping_add(imm as lua_Unsigned)
                                as lua_Integer;
                            (*io_4)
                                .tt_ = (3 as libc::c_int
                                | (0 as libc::c_int) << 4 as libc::c_int) as lu_byte;
                        } else if (*v1).tt_ as libc::c_int
                            == 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
                        {
                            let mut nb: lua_Number = (*v1).value_.n;
                            let mut fimm: lua_Number = imm as lua_Number;
                            pc = pc.offset(1);
                            pc;
                            let mut io_5: *mut TValue = &mut (*ra_19).val;
                            (*io_5).value_.n = nb + fimm;
                            (*io_5)
                                .tt_ = (3 as libc::c_int
                                | (1 as libc::c_int) << 4 as libc::c_int) as lu_byte;
                        }
                        continue;
                    }
                    22 => {
                        let mut v1_0: *mut TValue = &mut (*base
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            ))
                            .val;
                        let mut v2: *mut TValue = k
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int + 8 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut ra_20: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        if (*v1_0).tt_ as libc::c_int
                            == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                            && (*v2).tt_ as libc::c_int
                                == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                        {
                            let mut i1: lua_Integer = (*v1_0).value_.i;
                            let mut i2: lua_Integer = (*v2).value_.i;
                            pc = pc.offset(1);
                            pc;
                            let mut io_6: *mut TValue = &mut (*ra_20).val;
                            (*io_6)
                                .value_
                                .i = (i1 as lua_Unsigned).wrapping_add(i2 as lua_Unsigned)
                                as lua_Integer;
                            (*io_6)
                                .tt_ = (3 as libc::c_int
                                | (0 as libc::c_int) << 4 as libc::c_int) as lu_byte;
                        } else {
                            let mut n1: lua_Number = 0.;
                            let mut n2: lua_Number = 0.;
                            if (if (*v1_0).tt_ as libc::c_int
                                == 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
                            {
                                n1 = (*v1_0).value_.n;
                                1 as libc::c_int
                            } else {
                                (if (*v1_0).tt_ as libc::c_int
                                    == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                                {
                                    n1 = (*v1_0).value_.i as lua_Number;
                                    1 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                })
                            }) != 0
                                && (if (*v2).tt_ as libc::c_int
                                    == 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
                                {
                                    n2 = (*v2).value_.n;
                                    1 as libc::c_int
                                } else {
                                    (if (*v2).tt_ as libc::c_int
                                        == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                                    {
                                        n2 = (*v2).value_.i as lua_Number;
                                        1 as libc::c_int
                                    } else {
                                        0 as libc::c_int
                                    })
                                }) != 0
                            {
                                pc = pc.offset(1);
                                pc;
                                let mut io_7: *mut TValue = &mut (*ra_20).val;
                                (*io_7).value_.n = n1 + n2;
                                (*io_7)
                                    .tt_ = (3 as libc::c_int
                                    | (1 as libc::c_int) << 4 as libc::c_int) as lu_byte;
                            }
                        }
                        continue;
                    }
                    23 => {
                        let mut v1_1: *mut TValue = &mut (*base
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            ))
                            .val;
                        let mut v2_0: *mut TValue = k
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int + 8 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut ra_21: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        if (*v1_1).tt_ as libc::c_int
                            == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                            && (*v2_0).tt_ as libc::c_int
                                == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                        {
                            let mut i1_0: lua_Integer = (*v1_1).value_.i;
                            let mut i2_0: lua_Integer = (*v2_0).value_.i;
                            pc = pc.offset(1);
                            pc;
                            let mut io_8: *mut TValue = &mut (*ra_21).val;
                            (*io_8)
                                .value_
                                .i = (i1_0 as lua_Unsigned)
                                .wrapping_sub(i2_0 as lua_Unsigned) as lua_Integer;
                            (*io_8)
                                .tt_ = (3 as libc::c_int
                                | (0 as libc::c_int) << 4 as libc::c_int) as lu_byte;
                        } else {
                            let mut n1_0: lua_Number = 0.;
                            let mut n2_0: lua_Number = 0.;
                            if (if (*v1_1).tt_ as libc::c_int
                                == 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
                            {
                                n1_0 = (*v1_1).value_.n;
                                1 as libc::c_int
                            } else {
                                (if (*v1_1).tt_ as libc::c_int
                                    == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                                {
                                    n1_0 = (*v1_1).value_.i as lua_Number;
                                    1 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                })
                            }) != 0
                                && (if (*v2_0).tt_ as libc::c_int
                                    == 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
                                {
                                    n2_0 = (*v2_0).value_.n;
                                    1 as libc::c_int
                                } else {
                                    (if (*v2_0).tt_ as libc::c_int
                                        == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                                    {
                                        n2_0 = (*v2_0).value_.i as lua_Number;
                                        1 as libc::c_int
                                    } else {
                                        0 as libc::c_int
                                    })
                                }) != 0
                            {
                                pc = pc.offset(1);
                                pc;
                                let mut io_9: *mut TValue = &mut (*ra_21).val;
                                (*io_9).value_.n = n1_0 - n2_0;
                                (*io_9)
                                    .tt_ = (3 as libc::c_int
                                    | (1 as libc::c_int) << 4 as libc::c_int) as lu_byte;
                            }
                        }
                        continue;
                    }
                    24 => {
                        let mut v1_2: *mut TValue = &mut (*base
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            ))
                            .val;
                        let mut v2_1: *mut TValue = k
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int + 8 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut ra_22: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        if (*v1_2).tt_ as libc::c_int
                            == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                            && (*v2_1).tt_ as libc::c_int
                                == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                        {
                            let mut i1_1: lua_Integer = (*v1_2).value_.i;
                            let mut i2_1: lua_Integer = (*v2_1).value_.i;
                            pc = pc.offset(1);
                            pc;
                            let mut io_10: *mut TValue = &mut (*ra_22).val;
                            (*io_10)
                                .value_
                                .i = (i1_1 as lua_Unsigned)
                                .wrapping_mul(i2_1 as lua_Unsigned) as lua_Integer;
                            (*io_10)
                                .tt_ = (3 as libc::c_int
                                | (0 as libc::c_int) << 4 as libc::c_int) as lu_byte;
                        } else {
                            let mut n1_1: lua_Number = 0.;
                            let mut n2_1: lua_Number = 0.;
                            if (if (*v1_2).tt_ as libc::c_int
                                == 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
                            {
                                n1_1 = (*v1_2).value_.n;
                                1 as libc::c_int
                            } else {
                                (if (*v1_2).tt_ as libc::c_int
                                    == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                                {
                                    n1_1 = (*v1_2).value_.i as lua_Number;
                                    1 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                })
                            }) != 0
                                && (if (*v2_1).tt_ as libc::c_int
                                    == 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
                                {
                                    n2_1 = (*v2_1).value_.n;
                                    1 as libc::c_int
                                } else {
                                    (if (*v2_1).tt_ as libc::c_int
                                        == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                                    {
                                        n2_1 = (*v2_1).value_.i as lua_Number;
                                        1 as libc::c_int
                                    } else {
                                        0 as libc::c_int
                                    })
                                }) != 0
                            {
                                pc = pc.offset(1);
                                pc;
                                let mut io_11: *mut TValue = &mut (*ra_22).val;
                                (*io_11).value_.n = n1_1 * n2_1;
                                (*io_11)
                                    .tt_ = (3 as libc::c_int
                                    | (1 as libc::c_int) << 4 as libc::c_int) as lu_byte;
                            }
                        }
                        continue;
                    }
                    25 => {
                        (*ci).u.l.savedpc = pc;
                        (*L).top.p = (*ci).top.p;
                        let mut v1_3: *mut TValue = &mut (*base
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            ))
                            .val;
                        let mut v2_2: *mut TValue = k
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int + 8 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut ra_23: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        if (*v1_3).tt_ as libc::c_int
                            == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                            && (*v2_2).tt_ as libc::c_int
                                == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                        {
                            let mut i1_2: lua_Integer = (*v1_3).value_.i;
                            let mut i2_2: lua_Integer = (*v2_2).value_.i;
                            pc = pc.offset(1);
                            pc;
                            let mut io_12: *mut TValue = &mut (*ra_23).val;
                            (*io_12).value_.i = luaV_mod(L, i1_2, i2_2);
                            (*io_12)
                                .tt_ = (3 as libc::c_int
                                | (0 as libc::c_int) << 4 as libc::c_int) as lu_byte;
                        } else {
                            let mut n1_2: lua_Number = 0.;
                            let mut n2_2: lua_Number = 0.;
                            if (if (*v1_3).tt_ as libc::c_int
                                == 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
                            {
                                n1_2 = (*v1_3).value_.n;
                                1 as libc::c_int
                            } else {
                                (if (*v1_3).tt_ as libc::c_int
                                    == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                                {
                                    n1_2 = (*v1_3).value_.i as lua_Number;
                                    1 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                })
                            }) != 0
                                && (if (*v2_2).tt_ as libc::c_int
                                    == 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
                                {
                                    n2_2 = (*v2_2).value_.n;
                                    1 as libc::c_int
                                } else {
                                    (if (*v2_2).tt_ as libc::c_int
                                        == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                                    {
                                        n2_2 = (*v2_2).value_.i as lua_Number;
                                        1 as libc::c_int
                                    } else {
                                        0 as libc::c_int
                                    })
                                }) != 0
                            {
                                pc = pc.offset(1);
                                pc;
                                let mut io_13: *mut TValue = &mut (*ra_23).val;
                                (*io_13).value_.n = luaV_modf(L, n1_2, n2_2);
                                (*io_13)
                                    .tt_ = (3 as libc::c_int
                                    | (1 as libc::c_int) << 4 as libc::c_int) as lu_byte;
                            }
                        }
                        continue;
                    }
                    26 => {
                        let mut ra_24: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut v1_4: *mut TValue = &mut (*base
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            ))
                            .val;
                        let mut v2_3: *mut TValue = k
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int + 8 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut n1_3: lua_Number = 0.;
                        let mut n2_3: lua_Number = 0.;
                        if (if (*v1_4).tt_ as libc::c_int
                            == 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
                        {
                            n1_3 = (*v1_4).value_.n;
                            1 as libc::c_int
                        } else {
                            (if (*v1_4).tt_ as libc::c_int
                                == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                            {
                                n1_3 = (*v1_4).value_.i as lua_Number;
                                1 as libc::c_int
                            } else {
                                0 as libc::c_int
                            })
                        }) != 0
                            && (if (*v2_3).tt_ as libc::c_int
                                == 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
                            {
                                n2_3 = (*v2_3).value_.n;
                                1 as libc::c_int
                            } else {
                                (if (*v2_3).tt_ as libc::c_int
                                    == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                                {
                                    n2_3 = (*v2_3).value_.i as lua_Number;
                                    1 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                })
                            }) != 0
                        {
                            pc = pc.offset(1);
                            pc;
                            let mut io_14: *mut TValue = &mut (*ra_24).val;
                            (*io_14)
                                .value_
                                .n = (if n2_3 == 2 as libc::c_int as libc::c_double {
                                n1_3 * n1_3
                            } else {
                                pow(n1_3, n2_3)
                            });
                            (*io_14)
                                .tt_ = (3 as libc::c_int
                                | (1 as libc::c_int) << 4 as libc::c_int) as lu_byte;
                        }
                        continue;
                    }
                    27 => {
                        let mut ra_25: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut v1_5: *mut TValue = &mut (*base
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            ))
                            .val;
                        let mut v2_4: *mut TValue = k
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int + 8 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut n1_4: lua_Number = 0.;
                        let mut n2_4: lua_Number = 0.;
                        if (if (*v1_5).tt_ as libc::c_int
                            == 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
                        {
                            n1_4 = (*v1_5).value_.n;
                            1 as libc::c_int
                        } else {
                            (if (*v1_5).tt_ as libc::c_int
                                == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                            {
                                n1_4 = (*v1_5).value_.i as lua_Number;
                                1 as libc::c_int
                            } else {
                                0 as libc::c_int
                            })
                        }) != 0
                            && (if (*v2_4).tt_ as libc::c_int
                                == 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
                            {
                                n2_4 = (*v2_4).value_.n;
                                1 as libc::c_int
                            } else {
                                (if (*v2_4).tt_ as libc::c_int
                                    == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                                {
                                    n2_4 = (*v2_4).value_.i as lua_Number;
                                    1 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                })
                            }) != 0
                        {
                            pc = pc.offset(1);
                            pc;
                            let mut io_15: *mut TValue = &mut (*ra_25).val;
                            (*io_15).value_.n = n1_4 / n2_4;
                            (*io_15)
                                .tt_ = (3 as libc::c_int
                                | (1 as libc::c_int) << 4 as libc::c_int) as lu_byte;
                        }
                        continue;
                    }
                    28 => {
                        (*ci).u.l.savedpc = pc;
                        (*L).top.p = (*ci).top.p;
                        let mut v1_6: *mut TValue = &mut (*base
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            ))
                            .val;
                        let mut v2_5: *mut TValue = k
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int + 8 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut ra_26: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        if (*v1_6).tt_ as libc::c_int
                            == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                            && (*v2_5).tt_ as libc::c_int
                                == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                        {
                            let mut i1_3: lua_Integer = (*v1_6).value_.i;
                            let mut i2_3: lua_Integer = (*v2_5).value_.i;
                            pc = pc.offset(1);
                            pc;
                            let mut io_16: *mut TValue = &mut (*ra_26).val;
                            (*io_16).value_.i = luaV_idiv(L, i1_3, i2_3);
                            (*io_16)
                                .tt_ = (3 as libc::c_int
                                | (0 as libc::c_int) << 4 as libc::c_int) as lu_byte;
                        } else {
                            let mut n1_5: lua_Number = 0.;
                            let mut n2_5: lua_Number = 0.;
                            if (if (*v1_6).tt_ as libc::c_int
                                == 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
                            {
                                n1_5 = (*v1_6).value_.n;
                                1 as libc::c_int
                            } else {
                                (if (*v1_6).tt_ as libc::c_int
                                    == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                                {
                                    n1_5 = (*v1_6).value_.i as lua_Number;
                                    1 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                })
                            }) != 0
                                && (if (*v2_5).tt_ as libc::c_int
                                    == 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
                                {
                                    n2_5 = (*v2_5).value_.n;
                                    1 as libc::c_int
                                } else {
                                    (if (*v2_5).tt_ as libc::c_int
                                        == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                                    {
                                        n2_5 = (*v2_5).value_.i as lua_Number;
                                        1 as libc::c_int
                                    } else {
                                        0 as libc::c_int
                                    })
                                }) != 0
                            {
                                pc = pc.offset(1);
                                pc;
                                let mut io_17: *mut TValue = &mut (*ra_26).val;
                                (*io_17).value_.n = floor(n1_5 / n2_5);
                                (*io_17)
                                    .tt_ = (3 as libc::c_int
                                    | (1 as libc::c_int) << 4 as libc::c_int) as lu_byte;
                            }
                        }
                        continue;
                    }
                    29 => {
                        let mut ra_27: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut v1_7: *mut TValue = &mut (*base
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            ))
                            .val;
                        let mut v2_6: *mut TValue = k
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int + 8 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut i1_4: lua_Integer = 0;
                        let mut i2_4: lua_Integer = (*v2_6).value_.i;
                        if if (((*v1_7).tt_ as libc::c_int
                            == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int)
                            as libc::c_int != 0 as libc::c_int) as libc::c_int
                            as libc::c_long != 0
                        {
                            i1_4 = (*v1_7).value_.i;
                            1 as libc::c_int
                        } else {
                            luaV_tointegerns(v1_7, &mut i1_4, F2Ieq)
                        } != 0
                        {
                            pc = pc.offset(1);
                            pc;
                            let mut io_18: *mut TValue = &mut (*ra_27).val;
                            (*io_18)
                                .value_
                                .i = (i1_4 as lua_Unsigned & i2_4 as lua_Unsigned)
                                as lua_Integer;
                            (*io_18)
                                .tt_ = (3 as libc::c_int
                                | (0 as libc::c_int) << 4 as libc::c_int) as lu_byte;
                        }
                        continue;
                    }
                    30 => {
                        let mut ra_28: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut v1_8: *mut TValue = &mut (*base
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            ))
                            .val;
                        let mut v2_7: *mut TValue = k
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int + 8 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut i1_5: lua_Integer = 0;
                        let mut i2_5: lua_Integer = (*v2_7).value_.i;
                        if if (((*v1_8).tt_ as libc::c_int
                            == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int)
                            as libc::c_int != 0 as libc::c_int) as libc::c_int
                            as libc::c_long != 0
                        {
                            i1_5 = (*v1_8).value_.i;
                            1 as libc::c_int
                        } else {
                            luaV_tointegerns(v1_8, &mut i1_5, F2Ieq)
                        } != 0
                        {
                            pc = pc.offset(1);
                            pc;
                            let mut io_19: *mut TValue = &mut (*ra_28).val;
                            (*io_19)
                                .value_
                                .i = (i1_5 as lua_Unsigned | i2_5 as lua_Unsigned)
                                as lua_Integer;
                            (*io_19)
                                .tt_ = (3 as libc::c_int
                                | (0 as libc::c_int) << 4 as libc::c_int) as lu_byte;
                        }
                        continue;
                    }
                    31 => {
                        let mut ra_29: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut v1_9: *mut TValue = &mut (*base
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            ))
                            .val;
                        let mut v2_8: *mut TValue = k
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int + 8 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut i1_6: lua_Integer = 0;
                        let mut i2_6: lua_Integer = (*v2_8).value_.i;
                        if if (((*v1_9).tt_ as libc::c_int
                            == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int)
                            as libc::c_int != 0 as libc::c_int) as libc::c_int
                            as libc::c_long != 0
                        {
                            i1_6 = (*v1_9).value_.i;
                            1 as libc::c_int
                        } else {
                            luaV_tointegerns(v1_9, &mut i1_6, F2Ieq)
                        } != 0
                        {
                            pc = pc.offset(1);
                            pc;
                            let mut io_20: *mut TValue = &mut (*ra_29).val;
                            (*io_20)
                                .value_
                                .i = (i1_6 as lua_Unsigned ^ i2_6 as lua_Unsigned)
                                as lua_Integer;
                            (*io_20)
                                .tt_ = (3 as libc::c_int
                                | (0 as libc::c_int) << 4 as libc::c_int) as lu_byte;
                        }
                        continue;
                    }
                    32 => {
                        let mut ra_30: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut rb_8: *mut TValue = &mut (*base
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            ))
                            .val;
                        let mut ic: libc::c_int = (i
                            >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                + 1 as libc::c_int + 8 as libc::c_int
                            & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                << 0 as libc::c_int) as libc::c_int
                            - (((1 as libc::c_int) << 8 as libc::c_int)
                                - 1 as libc::c_int >> 1 as libc::c_int);
                        let mut ib: lua_Integer = 0;
                        if if (((*rb_8).tt_ as libc::c_int
                            == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int)
                            as libc::c_int != 0 as libc::c_int) as libc::c_int
                            as libc::c_long != 0
                        {
                            ib = (*rb_8).value_.i;
                            1 as libc::c_int
                        } else {
                            luaV_tointegerns(rb_8, &mut ib, F2Ieq)
                        } != 0
                        {
                            pc = pc.offset(1);
                            pc;
                            let mut io_21: *mut TValue = &mut (*ra_30).val;
                            (*io_21).value_.i = luaV_shiftl(ib, -ic as lua_Integer);
                            (*io_21)
                                .tt_ = (3 as libc::c_int
                                | (0 as libc::c_int) << 4 as libc::c_int) as lu_byte;
                        }
                        continue;
                    }
                    33 => {
                        let mut ra_31: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut rb_9: *mut TValue = &mut (*base
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            ))
                            .val;
                        let mut ic_0: libc::c_int = (i
                            >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                + 1 as libc::c_int + 8 as libc::c_int
                            & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                << 0 as libc::c_int) as libc::c_int
                            - (((1 as libc::c_int) << 8 as libc::c_int)
                                - 1 as libc::c_int >> 1 as libc::c_int);
                        let mut ib_0: lua_Integer = 0;
                        if if (((*rb_9).tt_ as libc::c_int
                            == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int)
                            as libc::c_int != 0 as libc::c_int) as libc::c_int
                            as libc::c_long != 0
                        {
                            ib_0 = (*rb_9).value_.i;
                            1 as libc::c_int
                        } else {
                            luaV_tointegerns(rb_9, &mut ib_0, F2Ieq)
                        } != 0
                        {
                            pc = pc.offset(1);
                            pc;
                            let mut io_22: *mut TValue = &mut (*ra_31).val;
                            (*io_22).value_.i = luaV_shiftl(ic_0 as lua_Integer, ib_0);
                            (*io_22)
                                .tt_ = (3 as libc::c_int
                                | (0 as libc::c_int) << 4 as libc::c_int) as lu_byte;
                        }
                        continue;
                    }
                    34 => {
                        let mut v1_10: *mut TValue = &mut (*base
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            ))
                            .val;
                        let mut v2_9: *mut TValue = &mut (*base
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int + 8 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            ))
                            .val;
                        let mut ra_32: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        if (*v1_10).tt_ as libc::c_int
                            == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                            && (*v2_9).tt_ as libc::c_int
                                == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                        {
                            let mut i1_7: lua_Integer = (*v1_10).value_.i;
                            let mut i2_7: lua_Integer = (*v2_9).value_.i;
                            pc = pc.offset(1);
                            pc;
                            let mut io_23: *mut TValue = &mut (*ra_32).val;
                            (*io_23)
                                .value_
                                .i = (i1_7 as lua_Unsigned)
                                .wrapping_add(i2_7 as lua_Unsigned) as lua_Integer;
                            (*io_23)
                                .tt_ = (3 as libc::c_int
                                | (0 as libc::c_int) << 4 as libc::c_int) as lu_byte;
                        } else {
                            let mut n1_6: lua_Number = 0.;
                            let mut n2_6: lua_Number = 0.;
                            if (if (*v1_10).tt_ as libc::c_int
                                == 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
                            {
                                n1_6 = (*v1_10).value_.n;
                                1 as libc::c_int
                            } else {
                                (if (*v1_10).tt_ as libc::c_int
                                    == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                                {
                                    n1_6 = (*v1_10).value_.i as lua_Number;
                                    1 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                })
                            }) != 0
                                && (if (*v2_9).tt_ as libc::c_int
                                    == 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
                                {
                                    n2_6 = (*v2_9).value_.n;
                                    1 as libc::c_int
                                } else {
                                    (if (*v2_9).tt_ as libc::c_int
                                        == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                                    {
                                        n2_6 = (*v2_9).value_.i as lua_Number;
                                        1 as libc::c_int
                                    } else {
                                        0 as libc::c_int
                                    })
                                }) != 0
                            {
                                pc = pc.offset(1);
                                pc;
                                let mut io_24: *mut TValue = &mut (*ra_32).val;
                                (*io_24).value_.n = n1_6 + n2_6;
                                (*io_24)
                                    .tt_ = (3 as libc::c_int
                                    | (1 as libc::c_int) << 4 as libc::c_int) as lu_byte;
                            }
                        }
                        continue;
                    }
                    35 => {
                        let mut v1_11: *mut TValue = &mut (*base
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            ))
                            .val;
                        let mut v2_10: *mut TValue = &mut (*base
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int + 8 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            ))
                            .val;
                        let mut ra_33: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        if (*v1_11).tt_ as libc::c_int
                            == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                            && (*v2_10).tt_ as libc::c_int
                                == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                        {
                            let mut i1_8: lua_Integer = (*v1_11).value_.i;
                            let mut i2_8: lua_Integer = (*v2_10).value_.i;
                            pc = pc.offset(1);
                            pc;
                            let mut io_25: *mut TValue = &mut (*ra_33).val;
                            (*io_25)
                                .value_
                                .i = (i1_8 as lua_Unsigned)
                                .wrapping_sub(i2_8 as lua_Unsigned) as lua_Integer;
                            (*io_25)
                                .tt_ = (3 as libc::c_int
                                | (0 as libc::c_int) << 4 as libc::c_int) as lu_byte;
                        } else {
                            let mut n1_7: lua_Number = 0.;
                            let mut n2_7: lua_Number = 0.;
                            if (if (*v1_11).tt_ as libc::c_int
                                == 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
                            {
                                n1_7 = (*v1_11).value_.n;
                                1 as libc::c_int
                            } else {
                                (if (*v1_11).tt_ as libc::c_int
                                    == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                                {
                                    n1_7 = (*v1_11).value_.i as lua_Number;
                                    1 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                })
                            }) != 0
                                && (if (*v2_10).tt_ as libc::c_int
                                    == 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
                                {
                                    n2_7 = (*v2_10).value_.n;
                                    1 as libc::c_int
                                } else {
                                    (if (*v2_10).tt_ as libc::c_int
                                        == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                                    {
                                        n2_7 = (*v2_10).value_.i as lua_Number;
                                        1 as libc::c_int
                                    } else {
                                        0 as libc::c_int
                                    })
                                }) != 0
                            {
                                pc = pc.offset(1);
                                pc;
                                let mut io_26: *mut TValue = &mut (*ra_33).val;
                                (*io_26).value_.n = n1_7 - n2_7;
                                (*io_26)
                                    .tt_ = (3 as libc::c_int
                                    | (1 as libc::c_int) << 4 as libc::c_int) as lu_byte;
                            }
                        }
                        continue;
                    }
                    36 => {
                        let mut v1_12: *mut TValue = &mut (*base
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            ))
                            .val;
                        let mut v2_11: *mut TValue = &mut (*base
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int + 8 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            ))
                            .val;
                        let mut ra_34: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        if (*v1_12).tt_ as libc::c_int
                            == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                            && (*v2_11).tt_ as libc::c_int
                                == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                        {
                            let mut i1_9: lua_Integer = (*v1_12).value_.i;
                            let mut i2_9: lua_Integer = (*v2_11).value_.i;
                            pc = pc.offset(1);
                            pc;
                            let mut io_27: *mut TValue = &mut (*ra_34).val;
                            (*io_27)
                                .value_
                                .i = (i1_9 as lua_Unsigned)
                                .wrapping_mul(i2_9 as lua_Unsigned) as lua_Integer;
                            (*io_27)
                                .tt_ = (3 as libc::c_int
                                | (0 as libc::c_int) << 4 as libc::c_int) as lu_byte;
                        } else {
                            let mut n1_8: lua_Number = 0.;
                            let mut n2_8: lua_Number = 0.;
                            if (if (*v1_12).tt_ as libc::c_int
                                == 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
                            {
                                n1_8 = (*v1_12).value_.n;
                                1 as libc::c_int
                            } else {
                                (if (*v1_12).tt_ as libc::c_int
                                    == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                                {
                                    n1_8 = (*v1_12).value_.i as lua_Number;
                                    1 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                })
                            }) != 0
                                && (if (*v2_11).tt_ as libc::c_int
                                    == 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
                                {
                                    n2_8 = (*v2_11).value_.n;
                                    1 as libc::c_int
                                } else {
                                    (if (*v2_11).tt_ as libc::c_int
                                        == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                                    {
                                        n2_8 = (*v2_11).value_.i as lua_Number;
                                        1 as libc::c_int
                                    } else {
                                        0 as libc::c_int
                                    })
                                }) != 0
                            {
                                pc = pc.offset(1);
                                pc;
                                let mut io_28: *mut TValue = &mut (*ra_34).val;
                                (*io_28).value_.n = n1_8 * n2_8;
                                (*io_28)
                                    .tt_ = (3 as libc::c_int
                                    | (1 as libc::c_int) << 4 as libc::c_int) as lu_byte;
                            }
                        }
                        continue;
                    }
                    37 => {
                        (*ci).u.l.savedpc = pc;
                        (*L).top.p = (*ci).top.p;
                        let mut v1_13: *mut TValue = &mut (*base
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            ))
                            .val;
                        let mut v2_12: *mut TValue = &mut (*base
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int + 8 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            ))
                            .val;
                        let mut ra_35: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        if (*v1_13).tt_ as libc::c_int
                            == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                            && (*v2_12).tt_ as libc::c_int
                                == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                        {
                            let mut i1_10: lua_Integer = (*v1_13).value_.i;
                            let mut i2_10: lua_Integer = (*v2_12).value_.i;
                            pc = pc.offset(1);
                            pc;
                            let mut io_29: *mut TValue = &mut (*ra_35).val;
                            (*io_29).value_.i = luaV_mod(L, i1_10, i2_10);
                            (*io_29)
                                .tt_ = (3 as libc::c_int
                                | (0 as libc::c_int) << 4 as libc::c_int) as lu_byte;
                        } else {
                            let mut n1_9: lua_Number = 0.;
                            let mut n2_9: lua_Number = 0.;
                            if (if (*v1_13).tt_ as libc::c_int
                                == 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
                            {
                                n1_9 = (*v1_13).value_.n;
                                1 as libc::c_int
                            } else {
                                (if (*v1_13).tt_ as libc::c_int
                                    == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                                {
                                    n1_9 = (*v1_13).value_.i as lua_Number;
                                    1 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                })
                            }) != 0
                                && (if (*v2_12).tt_ as libc::c_int
                                    == 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
                                {
                                    n2_9 = (*v2_12).value_.n;
                                    1 as libc::c_int
                                } else {
                                    (if (*v2_12).tt_ as libc::c_int
                                        == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                                    {
                                        n2_9 = (*v2_12).value_.i as lua_Number;
                                        1 as libc::c_int
                                    } else {
                                        0 as libc::c_int
                                    })
                                }) != 0
                            {
                                pc = pc.offset(1);
                                pc;
                                let mut io_30: *mut TValue = &mut (*ra_35).val;
                                (*io_30).value_.n = luaV_modf(L, n1_9, n2_9);
                                (*io_30)
                                    .tt_ = (3 as libc::c_int
                                    | (1 as libc::c_int) << 4 as libc::c_int) as lu_byte;
                            }
                        }
                        continue;
                    }
                    38 => {
                        let mut ra_36: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut v1_14: *mut TValue = &mut (*base
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            ))
                            .val;
                        let mut v2_13: *mut TValue = &mut (*base
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int + 8 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            ))
                            .val;
                        let mut n1_10: lua_Number = 0.;
                        let mut n2_10: lua_Number = 0.;
                        if (if (*v1_14).tt_ as libc::c_int
                            == 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
                        {
                            n1_10 = (*v1_14).value_.n;
                            1 as libc::c_int
                        } else {
                            (if (*v1_14).tt_ as libc::c_int
                                == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                            {
                                n1_10 = (*v1_14).value_.i as lua_Number;
                                1 as libc::c_int
                            } else {
                                0 as libc::c_int
                            })
                        }) != 0
                            && (if (*v2_13).tt_ as libc::c_int
                                == 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
                            {
                                n2_10 = (*v2_13).value_.n;
                                1 as libc::c_int
                            } else {
                                (if (*v2_13).tt_ as libc::c_int
                                    == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                                {
                                    n2_10 = (*v2_13).value_.i as lua_Number;
                                    1 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                })
                            }) != 0
                        {
                            pc = pc.offset(1);
                            pc;
                            let mut io_31: *mut TValue = &mut (*ra_36).val;
                            (*io_31)
                                .value_
                                .n = (if n2_10 == 2 as libc::c_int as libc::c_double {
                                n1_10 * n1_10
                            } else {
                                pow(n1_10, n2_10)
                            });
                            (*io_31)
                                .tt_ = (3 as libc::c_int
                                | (1 as libc::c_int) << 4 as libc::c_int) as lu_byte;
                        }
                        continue;
                    }
                    39 => {
                        let mut ra_37: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut v1_15: *mut TValue = &mut (*base
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            ))
                            .val;
                        let mut v2_14: *mut TValue = &mut (*base
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int + 8 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            ))
                            .val;
                        let mut n1_11: lua_Number = 0.;
                        let mut n2_11: lua_Number = 0.;
                        if (if (*v1_15).tt_ as libc::c_int
                            == 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
                        {
                            n1_11 = (*v1_15).value_.n;
                            1 as libc::c_int
                        } else {
                            (if (*v1_15).tt_ as libc::c_int
                                == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                            {
                                n1_11 = (*v1_15).value_.i as lua_Number;
                                1 as libc::c_int
                            } else {
                                0 as libc::c_int
                            })
                        }) != 0
                            && (if (*v2_14).tt_ as libc::c_int
                                == 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
                            {
                                n2_11 = (*v2_14).value_.n;
                                1 as libc::c_int
                            } else {
                                (if (*v2_14).tt_ as libc::c_int
                                    == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                                {
                                    n2_11 = (*v2_14).value_.i as lua_Number;
                                    1 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                })
                            }) != 0
                        {
                            pc = pc.offset(1);
                            pc;
                            let mut io_32: *mut TValue = &mut (*ra_37).val;
                            (*io_32).value_.n = n1_11 / n2_11;
                            (*io_32)
                                .tt_ = (3 as libc::c_int
                                | (1 as libc::c_int) << 4 as libc::c_int) as lu_byte;
                        }
                        continue;
                    }
                    40 => {
                        (*ci).u.l.savedpc = pc;
                        (*L).top.p = (*ci).top.p;
                        let mut v1_16: *mut TValue = &mut (*base
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            ))
                            .val;
                        let mut v2_15: *mut TValue = &mut (*base
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int + 8 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            ))
                            .val;
                        let mut ra_38: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        if (*v1_16).tt_ as libc::c_int
                            == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                            && (*v2_15).tt_ as libc::c_int
                                == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                        {
                            let mut i1_11: lua_Integer = (*v1_16).value_.i;
                            let mut i2_11: lua_Integer = (*v2_15).value_.i;
                            pc = pc.offset(1);
                            pc;
                            let mut io_33: *mut TValue = &mut (*ra_38).val;
                            (*io_33).value_.i = luaV_idiv(L, i1_11, i2_11);
                            (*io_33)
                                .tt_ = (3 as libc::c_int
                                | (0 as libc::c_int) << 4 as libc::c_int) as lu_byte;
                        } else {
                            let mut n1_12: lua_Number = 0.;
                            let mut n2_12: lua_Number = 0.;
                            if (if (*v1_16).tt_ as libc::c_int
                                == 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
                            {
                                n1_12 = (*v1_16).value_.n;
                                1 as libc::c_int
                            } else {
                                (if (*v1_16).tt_ as libc::c_int
                                    == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                                {
                                    n1_12 = (*v1_16).value_.i as lua_Number;
                                    1 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                })
                            }) != 0
                                && (if (*v2_15).tt_ as libc::c_int
                                    == 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
                                {
                                    n2_12 = (*v2_15).value_.n;
                                    1 as libc::c_int
                                } else {
                                    (if (*v2_15).tt_ as libc::c_int
                                        == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                                    {
                                        n2_12 = (*v2_15).value_.i as lua_Number;
                                        1 as libc::c_int
                                    } else {
                                        0 as libc::c_int
                                    })
                                }) != 0
                            {
                                pc = pc.offset(1);
                                pc;
                                let mut io_34: *mut TValue = &mut (*ra_38).val;
                                (*io_34).value_.n = floor(n1_12 / n2_12);
                                (*io_34)
                                    .tt_ = (3 as libc::c_int
                                    | (1 as libc::c_int) << 4 as libc::c_int) as lu_byte;
                            }
                        }
                        continue;
                    }
                    41 => {
                        let mut ra_39: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut v1_17: *mut TValue = &mut (*base
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            ))
                            .val;
                        let mut v2_16: *mut TValue = &mut (*base
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int + 8 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            ))
                            .val;
                        let mut i1_12: lua_Integer = 0;
                        let mut i2_12: lua_Integer = 0;
                        if (if (((*v1_17).tt_ as libc::c_int
                            == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int)
                            as libc::c_int != 0 as libc::c_int) as libc::c_int
                            as libc::c_long != 0
                        {
                            i1_12 = (*v1_17).value_.i;
                            1 as libc::c_int
                        } else {
                            luaV_tointegerns(v1_17, &mut i1_12, F2Ieq)
                        }) != 0
                            && (if (((*v2_16).tt_ as libc::c_int
                                == 3 as libc::c_int
                                    | (0 as libc::c_int) << 4 as libc::c_int) as libc::c_int
                                != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
                            {
                                i2_12 = (*v2_16).value_.i;
                                1 as libc::c_int
                            } else {
                                luaV_tointegerns(v2_16, &mut i2_12, F2Ieq)
                            }) != 0
                        {
                            pc = pc.offset(1);
                            pc;
                            let mut io_35: *mut TValue = &mut (*ra_39).val;
                            (*io_35)
                                .value_
                                .i = (i1_12 as lua_Unsigned & i2_12 as lua_Unsigned)
                                as lua_Integer;
                            (*io_35)
                                .tt_ = (3 as libc::c_int
                                | (0 as libc::c_int) << 4 as libc::c_int) as lu_byte;
                        }
                        continue;
                    }
                    42 => {
                        let mut ra_40: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut v1_18: *mut TValue = &mut (*base
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            ))
                            .val;
                        let mut v2_17: *mut TValue = &mut (*base
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int + 8 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            ))
                            .val;
                        let mut i1_13: lua_Integer = 0;
                        let mut i2_13: lua_Integer = 0;
                        if (if (((*v1_18).tt_ as libc::c_int
                            == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int)
                            as libc::c_int != 0 as libc::c_int) as libc::c_int
                            as libc::c_long != 0
                        {
                            i1_13 = (*v1_18).value_.i;
                            1 as libc::c_int
                        } else {
                            luaV_tointegerns(v1_18, &mut i1_13, F2Ieq)
                        }) != 0
                            && (if (((*v2_17).tt_ as libc::c_int
                                == 3 as libc::c_int
                                    | (0 as libc::c_int) << 4 as libc::c_int) as libc::c_int
                                != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
                            {
                                i2_13 = (*v2_17).value_.i;
                                1 as libc::c_int
                            } else {
                                luaV_tointegerns(v2_17, &mut i2_13, F2Ieq)
                            }) != 0
                        {
                            pc = pc.offset(1);
                            pc;
                            let mut io_36: *mut TValue = &mut (*ra_40).val;
                            (*io_36)
                                .value_
                                .i = (i1_13 as lua_Unsigned | i2_13 as lua_Unsigned)
                                as lua_Integer;
                            (*io_36)
                                .tt_ = (3 as libc::c_int
                                | (0 as libc::c_int) << 4 as libc::c_int) as lu_byte;
                        }
                        continue;
                    }
                    43 => {
                        let mut ra_41: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut v1_19: *mut TValue = &mut (*base
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            ))
                            .val;
                        let mut v2_18: *mut TValue = &mut (*base
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int + 8 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            ))
                            .val;
                        let mut i1_14: lua_Integer = 0;
                        let mut i2_14: lua_Integer = 0;
                        if (if (((*v1_19).tt_ as libc::c_int
                            == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int)
                            as libc::c_int != 0 as libc::c_int) as libc::c_int
                            as libc::c_long != 0
                        {
                            i1_14 = (*v1_19).value_.i;
                            1 as libc::c_int
                        } else {
                            luaV_tointegerns(v1_19, &mut i1_14, F2Ieq)
                        }) != 0
                            && (if (((*v2_18).tt_ as libc::c_int
                                == 3 as libc::c_int
                                    | (0 as libc::c_int) << 4 as libc::c_int) as libc::c_int
                                != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
                            {
                                i2_14 = (*v2_18).value_.i;
                                1 as libc::c_int
                            } else {
                                luaV_tointegerns(v2_18, &mut i2_14, F2Ieq)
                            }) != 0
                        {
                            pc = pc.offset(1);
                            pc;
                            let mut io_37: *mut TValue = &mut (*ra_41).val;
                            (*io_37)
                                .value_
                                .i = (i1_14 as lua_Unsigned ^ i2_14 as lua_Unsigned)
                                as lua_Integer;
                            (*io_37)
                                .tt_ = (3 as libc::c_int
                                | (0 as libc::c_int) << 4 as libc::c_int) as lu_byte;
                        }
                        continue;
                    }
                    45 => {
                        let mut ra_42: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut v1_20: *mut TValue = &mut (*base
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            ))
                            .val;
                        let mut v2_19: *mut TValue = &mut (*base
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int + 8 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            ))
                            .val;
                        let mut i1_15: lua_Integer = 0;
                        let mut i2_15: lua_Integer = 0;
                        if (if (((*v1_20).tt_ as libc::c_int
                            == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int)
                            as libc::c_int != 0 as libc::c_int) as libc::c_int
                            as libc::c_long != 0
                        {
                            i1_15 = (*v1_20).value_.i;
                            1 as libc::c_int
                        } else {
                            luaV_tointegerns(v1_20, &mut i1_15, F2Ieq)
                        }) != 0
                            && (if (((*v2_19).tt_ as libc::c_int
                                == 3 as libc::c_int
                                    | (0 as libc::c_int) << 4 as libc::c_int) as libc::c_int
                                != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
                            {
                                i2_15 = (*v2_19).value_.i;
                                1 as libc::c_int
                            } else {
                                luaV_tointegerns(v2_19, &mut i2_15, F2Ieq)
                            }) != 0
                        {
                            pc = pc.offset(1);
                            pc;
                            let mut io_38: *mut TValue = &mut (*ra_42).val;
                            (*io_38)
                                .value_
                                .i = luaV_shiftl(
                                i1_15,
                                (0 as libc::c_int as lua_Unsigned)
                                    .wrapping_sub(i2_15 as lua_Unsigned) as lua_Integer,
                            );
                            (*io_38)
                                .tt_ = (3 as libc::c_int
                                | (0 as libc::c_int) << 4 as libc::c_int) as lu_byte;
                        }
                        continue;
                    }
                    44 => {
                        let mut ra_43: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut v1_21: *mut TValue = &mut (*base
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            ))
                            .val;
                        let mut v2_20: *mut TValue = &mut (*base
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int + 8 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            ))
                            .val;
                        let mut i1_16: lua_Integer = 0;
                        let mut i2_16: lua_Integer = 0;
                        if (if (((*v1_21).tt_ as libc::c_int
                            == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int)
                            as libc::c_int != 0 as libc::c_int) as libc::c_int
                            as libc::c_long != 0
                        {
                            i1_16 = (*v1_21).value_.i;
                            1 as libc::c_int
                        } else {
                            luaV_tointegerns(v1_21, &mut i1_16, F2Ieq)
                        }) != 0
                            && (if (((*v2_20).tt_ as libc::c_int
                                == 3 as libc::c_int
                                    | (0 as libc::c_int) << 4 as libc::c_int) as libc::c_int
                                != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
                            {
                                i2_16 = (*v2_20).value_.i;
                                1 as libc::c_int
                            } else {
                                luaV_tointegerns(v2_20, &mut i2_16, F2Ieq)
                            }) != 0
                        {
                            pc = pc.offset(1);
                            pc;
                            let mut io_39: *mut TValue = &mut (*ra_43).val;
                            (*io_39).value_.i = luaV_shiftl(i1_16, i2_16);
                            (*io_39)
                                .tt_ = (3 as libc::c_int
                                | (0 as libc::c_int) << 4 as libc::c_int) as lu_byte;
                        }
                        continue;
                    }
                    46 => {
                        let mut ra_44: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut pi: Instruction = *pc
                            .offset(-(2 as libc::c_int as isize));
                        let mut rb_10: *mut TValue = &mut (*base
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            ))
                            .val;
                        let mut tm: TMS = (i
                            >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                + 1 as libc::c_int + 8 as libc::c_int
                            & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                << 0 as libc::c_int) as libc::c_int as TMS;
                        let mut result: StkId = base
                            .offset(
                                (pi >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        (*ci).u.l.savedpc = pc;
                        (*L).top.p = (*ci).top.p;
                        luaT_trybinTM(L, &mut (*ra_44).val, rb_10, result, tm);
                        trap = (*ci).u.l.trap;
                        continue;
                    }
                    47 => {
                        let mut ra_45: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut pi_0: Instruction = *pc
                            .offset(-(2 as libc::c_int as isize));
                        let mut imm_0: libc::c_int = (i
                            >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                + 1 as libc::c_int
                            & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                << 0 as libc::c_int) as libc::c_int
                            - (((1 as libc::c_int) << 8 as libc::c_int)
                                - 1 as libc::c_int >> 1 as libc::c_int);
                        let mut tm_0: TMS = (i
                            >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                + 1 as libc::c_int + 8 as libc::c_int
                            & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                << 0 as libc::c_int) as libc::c_int as TMS;
                        let mut flip: libc::c_int = (i
                            >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                            & !(!(0 as libc::c_int as Instruction) << 1 as libc::c_int)
                                << 0 as libc::c_int) as libc::c_int;
                        let mut result_0: StkId = base
                            .offset(
                                (pi_0 >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        (*ci).u.l.savedpc = pc;
                        (*L).top.p = (*ci).top.p;
                        luaT_trybiniTM(
                            L,
                            &mut (*ra_45).val,
                            imm_0 as lua_Integer,
                            flip,
                            result_0,
                            tm_0,
                        );
                        trap = (*ci).u.l.trap;
                        continue;
                    }
                    48 => {
                        let mut ra_46: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut pi_1: Instruction = *pc
                            .offset(-(2 as libc::c_int as isize));
                        let mut imm_1: *mut TValue = k
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut tm_1: TMS = (i
                            >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                + 1 as libc::c_int + 8 as libc::c_int
                            & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                << 0 as libc::c_int) as libc::c_int as TMS;
                        let mut flip_0: libc::c_int = (i
                            >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                            & !(!(0 as libc::c_int as Instruction) << 1 as libc::c_int)
                                << 0 as libc::c_int) as libc::c_int;
                        let mut result_1: StkId = base
                            .offset(
                                (pi_1 >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        (*ci).u.l.savedpc = pc;
                        (*L).top.p = (*ci).top.p;
                        luaT_trybinassocTM(
                            L,
                            &mut (*ra_46).val,
                            imm_1,
                            flip_0,
                            result_1,
                            tm_1,
                        );
                        trap = (*ci).u.l.trap;
                        continue;
                    }
                    49 => {
                        let mut ra_47: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut rb_11: *mut TValue = &mut (*base
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            ))
                            .val;
                        let mut nb_0: lua_Number = 0.;
                        if (*rb_11).tt_ as libc::c_int
                            == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                        {
                            let mut ib_1: lua_Integer = (*rb_11).value_.i;
                            let mut io_40: *mut TValue = &mut (*ra_47).val;
                            (*io_40)
                                .value_
                                .i = (0 as libc::c_int as lua_Unsigned)
                                .wrapping_sub(ib_1 as lua_Unsigned) as lua_Integer;
                            (*io_40)
                                .tt_ = (3 as libc::c_int
                                | (0 as libc::c_int) << 4 as libc::c_int) as lu_byte;
                        } else if if (*rb_11).tt_ as libc::c_int
                            == 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
                        {
                            nb_0 = (*rb_11).value_.n;
                            1 as libc::c_int
                        } else if (*rb_11).tt_ as libc::c_int
                            == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                        {
                            nb_0 = (*rb_11).value_.i as lua_Number;
                            1 as libc::c_int
                        } else {
                            0 as libc::c_int
                        } != 0
                        {
                            let mut io_41: *mut TValue = &mut (*ra_47).val;
                            (*io_41).value_.n = -nb_0;
                            (*io_41)
                                .tt_ = (3 as libc::c_int
                                | (1 as libc::c_int) << 4 as libc::c_int) as lu_byte;
                        } else {
                            (*ci).u.l.savedpc = pc;
                            (*L).top.p = (*ci).top.p;
                            luaT_trybinTM(L, rb_11, rb_11, ra_47, TM_UNM);
                            trap = (*ci).u.l.trap;
                        }
                        continue;
                    }
                    50 => {
                        let mut ra_48: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut rb_12: *mut TValue = &mut (*base
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            ))
                            .val;
                        let mut ib_2: lua_Integer = 0;
                        if if (((*rb_12).tt_ as libc::c_int
                            == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int)
                            as libc::c_int != 0 as libc::c_int) as libc::c_int
                            as libc::c_long != 0
                        {
                            ib_2 = (*rb_12).value_.i;
                            1 as libc::c_int
                        } else {
                            luaV_tointegerns(rb_12, &mut ib_2, F2Ieq)
                        } != 0
                        {
                            let mut io_42: *mut TValue = &mut (*ra_48).val;
                            (*io_42)
                                .value_
                                .i = (!(0 as libc::c_int as lua_Unsigned)
                                ^ ib_2 as lua_Unsigned) as lua_Integer;
                            (*io_42)
                                .tt_ = (3 as libc::c_int
                                | (0 as libc::c_int) << 4 as libc::c_int) as lu_byte;
                        } else {
                            (*ci).u.l.savedpc = pc;
                            (*L).top.p = (*ci).top.p;
                            luaT_trybinTM(L, rb_12, rb_12, ra_48, TM_BNOT);
                            trap = (*ci).u.l.trap;
                        }
                        continue;
                    }
                    51 => {
                        let mut ra_49: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut rb_13: *mut TValue = &mut (*base
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            ))
                            .val;
                        if (*rb_13).tt_ as libc::c_int
                            == 1 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                            || (*rb_13).tt_ as libc::c_int & 0xf as libc::c_int
                                == 0 as libc::c_int
                        {
                            (*ra_49)
                                .val
                                .tt_ = (1 as libc::c_int
                                | (1 as libc::c_int) << 4 as libc::c_int) as lu_byte;
                        } else {
                            (*ra_49)
                                .val
                                .tt_ = (1 as libc::c_int
                                | (0 as libc::c_int) << 4 as libc::c_int) as lu_byte;
                        }
                        continue;
                    }
                    52 => {
                        let mut ra_50: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        (*ci).u.l.savedpc = pc;
                        (*L).top.p = (*ci).top.p;
                        luaV_objlen(
                            L,
                            ra_50,
                            &mut (*base
                                .offset(
                                    (i
                                        >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                            + 1 as libc::c_int
                                        & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                            << 0 as libc::c_int) as libc::c_int as isize,
                                ))
                                .val,
                        );
                        trap = (*ci).u.l.trap;
                        continue;
                    }
                    53 => {
                        let mut ra_51: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut n_1: libc::c_int = (i
                            >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                + 1 as libc::c_int
                            & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                << 0 as libc::c_int) as libc::c_int;
                        (*L).top.p = ra_51.offset(n_1 as isize);
                        (*ci).u.l.savedpc = pc;
                        luaV_concat(L, n_1);
                        trap = (*ci).u.l.trap;
                        if (*(*L).l_G).GCdebt > 0 as libc::c_int as libc::c_long {
                            (*ci).u.l.savedpc = pc;
                            (*L).top.p = (*L).top.p;
                            luaC_step(L);
                            trap = (*ci).u.l.trap;
                        }
                        continue;
                    }
                    54 => {
                        let mut ra_52: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        (*ci).u.l.savedpc = pc;
                        (*L).top.p = (*ci).top.p;
                        luaF_close(L, ra_52, 0 as libc::c_int, 1 as libc::c_int);
                        trap = (*ci).u.l.trap;
                        continue;
                    }
                    55 => {
                        let mut ra_53: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        (*ci).u.l.savedpc = pc;
                        (*L).top.p = (*ci).top.p;
                        luaF_newtbcupval(L, ra_53);
                        continue;
                    }
                    56 => {
                        pc = pc
                            .offset(
                                ((i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction)
                                        << 8 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int
                                            + 8 as libc::c_int) << 0 as libc::c_int) as libc::c_int
                                    - (((1 as libc::c_int)
                                        << 8 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int
                                            + 8 as libc::c_int) - 1 as libc::c_int >> 1 as libc::c_int)
                                    + 0 as libc::c_int) as isize,
                            );
                        trap = (*ci).u.l.trap;
                        continue;
                    }
                    57 => {
                        let mut ra_54: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut cond: libc::c_int = 0;
                        let mut rb_14: *mut TValue = &mut (*base
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            ))
                            .val;
                        (*ci).u.l.savedpc = pc;
                        (*L).top.p = (*ci).top.p;
                        cond = luaV_equalobj(L, &mut (*ra_54).val, rb_14);
                        trap = (*ci).u.l.trap;
                        if cond
                            != (i
                                >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                & !(!(0 as libc::c_int as Instruction) << 1 as libc::c_int)
                                    << 0 as libc::c_int) as libc::c_int
                        {
                            pc = pc.offset(1);
                            pc;
                        } else {
                            let mut ni: Instruction = *pc;
                            pc = pc
                                .offset(
                                    ((ni >> 0 as libc::c_int + 7 as libc::c_int
                                        & !(!(0 as libc::c_int as Instruction)
                                            << 8 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int
                                                + 8 as libc::c_int) << 0 as libc::c_int) as libc::c_int
                                        - (((1 as libc::c_int)
                                            << 8 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int
                                                + 8 as libc::c_int) - 1 as libc::c_int >> 1 as libc::c_int)
                                        + 1 as libc::c_int) as isize,
                                );
                            trap = (*ci).u.l.trap;
                        }
                        continue;
                    }
                    58 => {
                        let mut ra_55: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut cond_0: libc::c_int = 0;
                        let mut rb_15: *mut TValue = &mut (*base
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            ))
                            .val;
                        if (*ra_55).val.tt_ as libc::c_int
                            == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                            && (*rb_15).tt_ as libc::c_int
                                == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                        {
                            let mut ia: lua_Integer = (*ra_55).val.value_.i;
                            let mut ib_3: lua_Integer = (*rb_15).value_.i;
                            cond_0 = (ia < ib_3) as libc::c_int;
                        } else if (*ra_55).val.tt_ as libc::c_int & 0xf as libc::c_int
                            == 3 as libc::c_int
                            && (*rb_15).tt_ as libc::c_int & 0xf as libc::c_int
                                == 3 as libc::c_int
                        {
                            cond_0 = LTnum(&mut (*ra_55).val, rb_15);
                        } else {
                            (*ci).u.l.savedpc = pc;
                            (*L).top.p = (*ci).top.p;
                            cond_0 = lessthanothers(L, &mut (*ra_55).val, rb_15);
                            trap = (*ci).u.l.trap;
                        }
                        if cond_0
                            != (i
                                >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                & !(!(0 as libc::c_int as Instruction) << 1 as libc::c_int)
                                    << 0 as libc::c_int) as libc::c_int
                        {
                            pc = pc.offset(1);
                            pc;
                        } else {
                            let mut ni_0: Instruction = *pc;
                            pc = pc
                                .offset(
                                    ((ni_0 >> 0 as libc::c_int + 7 as libc::c_int
                                        & !(!(0 as libc::c_int as Instruction)
                                            << 8 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int
                                                + 8 as libc::c_int) << 0 as libc::c_int) as libc::c_int
                                        - (((1 as libc::c_int)
                                            << 8 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int
                                                + 8 as libc::c_int) - 1 as libc::c_int >> 1 as libc::c_int)
                                        + 1 as libc::c_int) as isize,
                                );
                            trap = (*ci).u.l.trap;
                        }
                        continue;
                    }
                    59 => {
                        let mut ra_56: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut cond_1: libc::c_int = 0;
                        let mut rb_16: *mut TValue = &mut (*base
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            ))
                            .val;
                        if (*ra_56).val.tt_ as libc::c_int
                            == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                            && (*rb_16).tt_ as libc::c_int
                                == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                        {
                            let mut ia_0: lua_Integer = (*ra_56).val.value_.i;
                            let mut ib_4: lua_Integer = (*rb_16).value_.i;
                            cond_1 = (ia_0 <= ib_4) as libc::c_int;
                        } else if (*ra_56).val.tt_ as libc::c_int & 0xf as libc::c_int
                            == 3 as libc::c_int
                            && (*rb_16).tt_ as libc::c_int & 0xf as libc::c_int
                                == 3 as libc::c_int
                        {
                            cond_1 = LEnum(&mut (*ra_56).val, rb_16);
                        } else {
                            (*ci).u.l.savedpc = pc;
                            (*L).top.p = (*ci).top.p;
                            cond_1 = lessequalothers(L, &mut (*ra_56).val, rb_16);
                            trap = (*ci).u.l.trap;
                        }
                        if cond_1
                            != (i
                                >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                & !(!(0 as libc::c_int as Instruction) << 1 as libc::c_int)
                                    << 0 as libc::c_int) as libc::c_int
                        {
                            pc = pc.offset(1);
                            pc;
                        } else {
                            let mut ni_1: Instruction = *pc;
                            pc = pc
                                .offset(
                                    ((ni_1 >> 0 as libc::c_int + 7 as libc::c_int
                                        & !(!(0 as libc::c_int as Instruction)
                                            << 8 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int
                                                + 8 as libc::c_int) << 0 as libc::c_int) as libc::c_int
                                        - (((1 as libc::c_int)
                                            << 8 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int
                                                + 8 as libc::c_int) - 1 as libc::c_int >> 1 as libc::c_int)
                                        + 1 as libc::c_int) as isize,
                                );
                            trap = (*ci).u.l.trap;
                        }
                        continue;
                    }
                    60 => {
                        let mut ra_57: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut rb_17: *mut TValue = k
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut cond_2: libc::c_int = luaV_equalobj(
                            0 as *mut lua_State,
                            &mut (*ra_57).val,
                            rb_17,
                        );
                        if cond_2
                            != (i
                                >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                & !(!(0 as libc::c_int as Instruction) << 1 as libc::c_int)
                                    << 0 as libc::c_int) as libc::c_int
                        {
                            pc = pc.offset(1);
                            pc;
                        } else {
                            let mut ni_2: Instruction = *pc;
                            pc = pc
                                .offset(
                                    ((ni_2 >> 0 as libc::c_int + 7 as libc::c_int
                                        & !(!(0 as libc::c_int as Instruction)
                                            << 8 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int
                                                + 8 as libc::c_int) << 0 as libc::c_int) as libc::c_int
                                        - (((1 as libc::c_int)
                                            << 8 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int
                                                + 8 as libc::c_int) - 1 as libc::c_int >> 1 as libc::c_int)
                                        + 1 as libc::c_int) as isize,
                                );
                            trap = (*ci).u.l.trap;
                        }
                        continue;
                    }
                    61 => {
                        let mut ra_58: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut cond_3: libc::c_int = 0;
                        let mut im: libc::c_int = (i
                            >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                + 1 as libc::c_int
                            & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                << 0 as libc::c_int) as libc::c_int
                            - (((1 as libc::c_int) << 8 as libc::c_int)
                                - 1 as libc::c_int >> 1 as libc::c_int);
                        if (*ra_58).val.tt_ as libc::c_int
                            == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                        {
                            cond_3 = ((*ra_58).val.value_.i == im as libc::c_longlong)
                                as libc::c_int;
                        } else if (*ra_58).val.tt_ as libc::c_int
                            == 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
                        {
                            cond_3 = ((*ra_58).val.value_.n == im as lua_Number)
                                as libc::c_int;
                        } else {
                            cond_3 = 0 as libc::c_int;
                        }
                        if cond_3
                            != (i
                                >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                & !(!(0 as libc::c_int as Instruction) << 1 as libc::c_int)
                                    << 0 as libc::c_int) as libc::c_int
                        {
                            pc = pc.offset(1);
                            pc;
                        } else {
                            let mut ni_3: Instruction = *pc;
                            pc = pc
                                .offset(
                                    ((ni_3 >> 0 as libc::c_int + 7 as libc::c_int
                                        & !(!(0 as libc::c_int as Instruction)
                                            << 8 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int
                                                + 8 as libc::c_int) << 0 as libc::c_int) as libc::c_int
                                        - (((1 as libc::c_int)
                                            << 8 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int
                                                + 8 as libc::c_int) - 1 as libc::c_int >> 1 as libc::c_int)
                                        + 1 as libc::c_int) as isize,
                                );
                            trap = (*ci).u.l.trap;
                        }
                        continue;
                    }
                    62 => {
                        let mut ra_59: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut cond_4: libc::c_int = 0;
                        let mut im_0: libc::c_int = (i
                            >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                + 1 as libc::c_int
                            & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                << 0 as libc::c_int) as libc::c_int
                            - (((1 as libc::c_int) << 8 as libc::c_int)
                                - 1 as libc::c_int >> 1 as libc::c_int);
                        if (*ra_59).val.tt_ as libc::c_int
                            == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                        {
                            cond_4 = ((*ra_59).val.value_.i < im_0 as libc::c_longlong)
                                as libc::c_int;
                        } else if (*ra_59).val.tt_ as libc::c_int
                            == 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
                        {
                            let mut fa: lua_Number = (*ra_59).val.value_.n;
                            let mut fim: lua_Number = im_0 as lua_Number;
                            cond_4 = (fa < fim) as libc::c_int;
                        } else {
                            let mut isf: libc::c_int = (i
                                >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                    + 1 as libc::c_int + 8 as libc::c_int
                                & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                    << 0 as libc::c_int) as libc::c_int;
                            (*ci).u.l.savedpc = pc;
                            (*L).top.p = (*ci).top.p;
                            cond_4 = luaT_callorderiTM(
                                L,
                                &mut (*ra_59).val,
                                im_0,
                                0 as libc::c_int,
                                isf,
                                TM_LT,
                            );
                            trap = (*ci).u.l.trap;
                        }
                        if cond_4
                            != (i
                                >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                & !(!(0 as libc::c_int as Instruction) << 1 as libc::c_int)
                                    << 0 as libc::c_int) as libc::c_int
                        {
                            pc = pc.offset(1);
                            pc;
                        } else {
                            let mut ni_4: Instruction = *pc;
                            pc = pc
                                .offset(
                                    ((ni_4 >> 0 as libc::c_int + 7 as libc::c_int
                                        & !(!(0 as libc::c_int as Instruction)
                                            << 8 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int
                                                + 8 as libc::c_int) << 0 as libc::c_int) as libc::c_int
                                        - (((1 as libc::c_int)
                                            << 8 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int
                                                + 8 as libc::c_int) - 1 as libc::c_int >> 1 as libc::c_int)
                                        + 1 as libc::c_int) as isize,
                                );
                            trap = (*ci).u.l.trap;
                        }
                        continue;
                    }
                    63 => {
                        let mut ra_60: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut cond_5: libc::c_int = 0;
                        let mut im_1: libc::c_int = (i
                            >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                + 1 as libc::c_int
                            & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                << 0 as libc::c_int) as libc::c_int
                            - (((1 as libc::c_int) << 8 as libc::c_int)
                                - 1 as libc::c_int >> 1 as libc::c_int);
                        if (*ra_60).val.tt_ as libc::c_int
                            == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                        {
                            cond_5 = ((*ra_60).val.value_.i <= im_1 as libc::c_longlong)
                                as libc::c_int;
                        } else if (*ra_60).val.tt_ as libc::c_int
                            == 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
                        {
                            let mut fa_0: lua_Number = (*ra_60).val.value_.n;
                            let mut fim_0: lua_Number = im_1 as lua_Number;
                            cond_5 = (fa_0 <= fim_0) as libc::c_int;
                        } else {
                            let mut isf_0: libc::c_int = (i
                                >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                    + 1 as libc::c_int + 8 as libc::c_int
                                & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                    << 0 as libc::c_int) as libc::c_int;
                            (*ci).u.l.savedpc = pc;
                            (*L).top.p = (*ci).top.p;
                            cond_5 = luaT_callorderiTM(
                                L,
                                &mut (*ra_60).val,
                                im_1,
                                0 as libc::c_int,
                                isf_0,
                                TM_LE,
                            );
                            trap = (*ci).u.l.trap;
                        }
                        if cond_5
                            != (i
                                >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                & !(!(0 as libc::c_int as Instruction) << 1 as libc::c_int)
                                    << 0 as libc::c_int) as libc::c_int
                        {
                            pc = pc.offset(1);
                            pc;
                        } else {
                            let mut ni_5: Instruction = *pc;
                            pc = pc
                                .offset(
                                    ((ni_5 >> 0 as libc::c_int + 7 as libc::c_int
                                        & !(!(0 as libc::c_int as Instruction)
                                            << 8 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int
                                                + 8 as libc::c_int) << 0 as libc::c_int) as libc::c_int
                                        - (((1 as libc::c_int)
                                            << 8 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int
                                                + 8 as libc::c_int) - 1 as libc::c_int >> 1 as libc::c_int)
                                        + 1 as libc::c_int) as isize,
                                );
                            trap = (*ci).u.l.trap;
                        }
                        continue;
                    }
                    64 => {
                        let mut ra_61: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut cond_6: libc::c_int = 0;
                        let mut im_2: libc::c_int = (i
                            >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                + 1 as libc::c_int
                            & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                << 0 as libc::c_int) as libc::c_int
                            - (((1 as libc::c_int) << 8 as libc::c_int)
                                - 1 as libc::c_int >> 1 as libc::c_int);
                        if (*ra_61).val.tt_ as libc::c_int
                            == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                        {
                            cond_6 = ((*ra_61).val.value_.i > im_2 as libc::c_longlong)
                                as libc::c_int;
                        } else if (*ra_61).val.tt_ as libc::c_int
                            == 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
                        {
                            let mut fa_1: lua_Number = (*ra_61).val.value_.n;
                            let mut fim_1: lua_Number = im_2 as lua_Number;
                            cond_6 = (fa_1 > fim_1) as libc::c_int;
                        } else {
                            let mut isf_1: libc::c_int = (i
                                >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                    + 1 as libc::c_int + 8 as libc::c_int
                                & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                    << 0 as libc::c_int) as libc::c_int;
                            (*ci).u.l.savedpc = pc;
                            (*L).top.p = (*ci).top.p;
                            cond_6 = luaT_callorderiTM(
                                L,
                                &mut (*ra_61).val,
                                im_2,
                                1 as libc::c_int,
                                isf_1,
                                TM_LT,
                            );
                            trap = (*ci).u.l.trap;
                        }
                        if cond_6
                            != (i
                                >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                & !(!(0 as libc::c_int as Instruction) << 1 as libc::c_int)
                                    << 0 as libc::c_int) as libc::c_int
                        {
                            pc = pc.offset(1);
                            pc;
                        } else {
                            let mut ni_6: Instruction = *pc;
                            pc = pc
                                .offset(
                                    ((ni_6 >> 0 as libc::c_int + 7 as libc::c_int
                                        & !(!(0 as libc::c_int as Instruction)
                                            << 8 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int
                                                + 8 as libc::c_int) << 0 as libc::c_int) as libc::c_int
                                        - (((1 as libc::c_int)
                                            << 8 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int
                                                + 8 as libc::c_int) - 1 as libc::c_int >> 1 as libc::c_int)
                                        + 1 as libc::c_int) as isize,
                                );
                            trap = (*ci).u.l.trap;
                        }
                        continue;
                    }
                    65 => {
                        let mut ra_62: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut cond_7: libc::c_int = 0;
                        let mut im_3: libc::c_int = (i
                            >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                + 1 as libc::c_int
                            & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                << 0 as libc::c_int) as libc::c_int
                            - (((1 as libc::c_int) << 8 as libc::c_int)
                                - 1 as libc::c_int >> 1 as libc::c_int);
                        if (*ra_62).val.tt_ as libc::c_int
                            == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                        {
                            cond_7 = ((*ra_62).val.value_.i >= im_3 as libc::c_longlong)
                                as libc::c_int;
                        } else if (*ra_62).val.tt_ as libc::c_int
                            == 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
                        {
                            let mut fa_2: lua_Number = (*ra_62).val.value_.n;
                            let mut fim_2: lua_Number = im_3 as lua_Number;
                            cond_7 = (fa_2 >= fim_2) as libc::c_int;
                        } else {
                            let mut isf_2: libc::c_int = (i
                                >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                    + 1 as libc::c_int + 8 as libc::c_int
                                & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                    << 0 as libc::c_int) as libc::c_int;
                            (*ci).u.l.savedpc = pc;
                            (*L).top.p = (*ci).top.p;
                            cond_7 = luaT_callorderiTM(
                                L,
                                &mut (*ra_62).val,
                                im_3,
                                1 as libc::c_int,
                                isf_2,
                                TM_LE,
                            );
                            trap = (*ci).u.l.trap;
                        }
                        if cond_7
                            != (i
                                >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                & !(!(0 as libc::c_int as Instruction) << 1 as libc::c_int)
                                    << 0 as libc::c_int) as libc::c_int
                        {
                            pc = pc.offset(1);
                            pc;
                        } else {
                            let mut ni_7: Instruction = *pc;
                            pc = pc
                                .offset(
                                    ((ni_7 >> 0 as libc::c_int + 7 as libc::c_int
                                        & !(!(0 as libc::c_int as Instruction)
                                            << 8 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int
                                                + 8 as libc::c_int) << 0 as libc::c_int) as libc::c_int
                                        - (((1 as libc::c_int)
                                            << 8 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int
                                                + 8 as libc::c_int) - 1 as libc::c_int >> 1 as libc::c_int)
                                        + 1 as libc::c_int) as isize,
                                );
                            trap = (*ci).u.l.trap;
                        }
                        continue;
                    }
                    66 => {
                        let mut ra_63: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut cond_8: libc::c_int = !((*ra_63).val.tt_ as libc::c_int
                            == 1 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                            || (*ra_63).val.tt_ as libc::c_int & 0xf as libc::c_int
                                == 0 as libc::c_int) as libc::c_int;
                        if cond_8
                            != (i
                                >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                & !(!(0 as libc::c_int as Instruction) << 1 as libc::c_int)
                                    << 0 as libc::c_int) as libc::c_int
                        {
                            pc = pc.offset(1);
                            pc;
                        } else {
                            let mut ni_8: Instruction = *pc;
                            pc = pc
                                .offset(
                                    ((ni_8 >> 0 as libc::c_int + 7 as libc::c_int
                                        & !(!(0 as libc::c_int as Instruction)
                                            << 8 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int
                                                + 8 as libc::c_int) << 0 as libc::c_int) as libc::c_int
                                        - (((1 as libc::c_int)
                                            << 8 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int
                                                + 8 as libc::c_int) - 1 as libc::c_int >> 1 as libc::c_int)
                                        + 1 as libc::c_int) as isize,
                                );
                            trap = (*ci).u.l.trap;
                        }
                        continue;
                    }
                    67 => {
                        let mut ra_64: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut rb_18: *mut TValue = &mut (*base
                            .offset(
                                (i
                                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        + 1 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            ))
                            .val;
                        if ((*rb_18).tt_ as libc::c_int
                            == 1 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                            || (*rb_18).tt_ as libc::c_int & 0xf as libc::c_int
                                == 0 as libc::c_int) as libc::c_int
                            == (i
                                >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                & !(!(0 as libc::c_int as Instruction) << 1 as libc::c_int)
                                    << 0 as libc::c_int) as libc::c_int
                        {
                            pc = pc.offset(1);
                            pc;
                        } else {
                            let mut io1_14: *mut TValue = &mut (*ra_64).val;
                            let mut io2_14: *const TValue = rb_18;
                            (*io1_14).value_ = (*io2_14).value_;
                            (*io1_14).tt_ = (*io2_14).tt_;
                            let mut ni_9: Instruction = *pc;
                            pc = pc
                                .offset(
                                    ((ni_9 >> 0 as libc::c_int + 7 as libc::c_int
                                        & !(!(0 as libc::c_int as Instruction)
                                            << 8 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int
                                                + 8 as libc::c_int) << 0 as libc::c_int) as libc::c_int
                                        - (((1 as libc::c_int)
                                            << 8 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int
                                                + 8 as libc::c_int) - 1 as libc::c_int >> 1 as libc::c_int)
                                        + 1 as libc::c_int) as isize,
                                );
                            trap = (*ci).u.l.trap;
                        }
                        continue;
                    }
                    68 => {
                        ra_65 = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        newci = 0 as *mut CallInfo;
                        b_4 = (i
                            >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                + 1 as libc::c_int
                            & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                << 0 as libc::c_int) as libc::c_int;
                        nresults = (i
                            >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                + 1 as libc::c_int + 8 as libc::c_int
                            & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                << 0 as libc::c_int) as libc::c_int - 1 as libc::c_int;
                        if b_4 != 0 as libc::c_int {
                            (*L).top.p = ra_65.offset(b_4 as isize);
                        }
                        (*ci).u.l.savedpc = pc;
                        newci = luaD_precall(L, ra_65, nresults);
                        if !newci.is_null() {
                            break '_returning;
                        }
                        trap = (*ci).u.l.trap;
                        continue;
                    }
                    69 => {
                        let mut ra_66: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut b_5: libc::c_int = (i
                            >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                + 1 as libc::c_int
                            & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                << 0 as libc::c_int) as libc::c_int;
                        let mut n_2: libc::c_int = 0;
                        let mut nparams1: libc::c_int = (i
                            >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                + 1 as libc::c_int + 8 as libc::c_int
                            & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                << 0 as libc::c_int) as libc::c_int;
                        let mut delta: libc::c_int = if nparams1 != 0 {
                            (*ci).u.l.nextraargs + nparams1
                        } else {
                            0 as libc::c_int
                        };
                        if b_5 != 0 as libc::c_int {
                            (*L).top.p = ra_66.offset(b_5 as isize);
                        } else {
                            b_5 = ((*L).top.p).offset_from(ra_66) as libc::c_long
                                as libc::c_int;
                        }
                        (*ci).u.l.savedpc = pc;
                        if (i
                            & (1 as libc::c_uint)
                                << 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int)
                            as libc::c_int != 0
                        {
                            luaF_closeupval(L, base);
                        }
                        n_2 = luaD_pretailcall(L, ci, ra_66, b_5, delta);
                        if n_2 < 0 as libc::c_int {
                            continue '_startfunc;
                        }
                        (*ci).func.p = ((*ci).func.p).offset(-(delta as isize));
                        luaD_poscall(L, ci, n_2);
                        trap = (*ci).u.l.trap;
                        break;
                    }
                    70 => {
                        let mut ra_67: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut n_3: libc::c_int = (i
                            >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                + 1 as libc::c_int
                            & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                << 0 as libc::c_int) as libc::c_int - 1 as libc::c_int;
                        let mut nparams1_0: libc::c_int = (i
                            >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                + 1 as libc::c_int + 8 as libc::c_int
                            & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                << 0 as libc::c_int) as libc::c_int;
                        if n_3 < 0 as libc::c_int {
                            n_3 = ((*L).top.p).offset_from(ra_67) as libc::c_long
                                as libc::c_int;
                        }
                        (*ci).u.l.savedpc = pc;
                        if (i
                            & (1 as libc::c_uint)
                                << 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int)
                            as libc::c_int != 0
                        {
                            (*ci).u2.nres = n_3;
                            if (*L).top.p < (*ci).top.p {
                                (*L).top.p = (*ci).top.p;
                            }
                            luaF_close(L, base, -(1 as libc::c_int), 1 as libc::c_int);
                            trap = (*ci).u.l.trap;
                            if (trap != 0 as libc::c_int) as libc::c_int as libc::c_long
                                != 0
                            {
                                base = ((*ci).func.p).offset(1 as libc::c_int as isize);
                                ra_67 = base
                                    .offset(
                                        (i >> 0 as libc::c_int + 7 as libc::c_int
                                            & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                                << 0 as libc::c_int) as libc::c_int as isize,
                                    );
                            }
                        }
                        if nparams1_0 != 0 {
                            (*ci)
                                .func
                                .p = ((*ci).func.p)
                                .offset(-(((*ci).u.l.nextraargs + nparams1_0) as isize));
                        }
                        (*L).top.p = ra_67.offset(n_3 as isize);
                        luaD_poscall(L, ci, n_3);
                        trap = (*ci).u.l.trap;
                        break;
                    }
                    71 => {
                        if ((*L).hookmask != 0 as libc::c_int) as libc::c_int
                            as libc::c_long != 0
                        {
                            let mut ra_68: StkId = base
                                .offset(
                                    (i >> 0 as libc::c_int + 7 as libc::c_int
                                        & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                            << 0 as libc::c_int) as libc::c_int as isize,
                                );
                            (*L).top.p = ra_68;
                            (*ci).u.l.savedpc = pc;
                            luaD_poscall(L, ci, 0 as libc::c_int);
                            trap = 1 as libc::c_int;
                        } else {
                            let mut nres: libc::c_int = 0;
                            (*L).ci = (*ci).previous;
                            (*L).top.p = base.offset(-(1 as libc::c_int as isize));
                            nres = (*ci).nresults as libc::c_int;
                            while ((nres > 0 as libc::c_int) as libc::c_int
                                != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
                            {
                                let fresh5 = (*L).top.p;
                                (*L).top.p = ((*L).top.p).offset(1);
                                (*fresh5)
                                    .val
                                    .tt_ = (0 as libc::c_int
                                    | (0 as libc::c_int) << 4 as libc::c_int) as lu_byte;
                                nres -= 1;
                                nres;
                            }
                        }
                        break;
                    }
                    72 => {
                        if ((*L).hookmask != 0 as libc::c_int) as libc::c_int
                            as libc::c_long != 0
                        {
                            let mut ra_69: StkId = base
                                .offset(
                                    (i >> 0 as libc::c_int + 7 as libc::c_int
                                        & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                            << 0 as libc::c_int) as libc::c_int as isize,
                                );
                            (*L).top.p = ra_69.offset(1 as libc::c_int as isize);
                            (*ci).u.l.savedpc = pc;
                            luaD_poscall(L, ci, 1 as libc::c_int);
                            trap = 1 as libc::c_int;
                        } else {
                            let mut nres_0: libc::c_int = (*ci).nresults as libc::c_int;
                            (*L).ci = (*ci).previous;
                            if nres_0 == 0 as libc::c_int {
                                (*L).top.p = base.offset(-(1 as libc::c_int as isize));
                            } else {
                                let mut ra_70: StkId = base
                                    .offset(
                                        (i >> 0 as libc::c_int + 7 as libc::c_int
                                            & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                                << 0 as libc::c_int) as libc::c_int as isize,
                                    );
                                let mut io1_15: *mut TValue = &mut (*base
                                    .offset(-(1 as libc::c_int as isize)))
                                    .val;
                                let mut io2_15: *const TValue = &mut (*ra_70).val;
                                (*io1_15).value_ = (*io2_15).value_;
                                (*io1_15).tt_ = (*io2_15).tt_;
                                (*L).top.p = base;
                                while ((nres_0 > 1 as libc::c_int) as libc::c_int
                                    != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
                                {
                                    let fresh6 = (*L).top.p;
                                    (*L).top.p = ((*L).top.p).offset(1);
                                    (*fresh6)
                                        .val
                                        .tt_ = (0 as libc::c_int
                                        | (0 as libc::c_int) << 4 as libc::c_int) as lu_byte;
                                    nres_0 -= 1;
                                    nres_0;
                                }
                            }
                        }
                        break;
                    }
                    73 => {
                        let mut ra_71: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        if (*ra_71.offset(2 as libc::c_int as isize)).val.tt_
                            as libc::c_int
                            == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                        {
                            let mut count: lua_Unsigned = (*ra_71
                                .offset(1 as libc::c_int as isize))
                                .val
                                .value_
                                .i as lua_Unsigned;
                            if count > 0 as libc::c_int as libc::c_ulonglong {
                                let mut step: lua_Integer = (*ra_71
                                    .offset(2 as libc::c_int as isize))
                                    .val
                                    .value_
                                    .i;
                                let mut idx: lua_Integer = (*ra_71).val.value_.i;
                                let mut io_43: *mut TValue = &mut (*ra_71
                                    .offset(1 as libc::c_int as isize))
                                    .val;
                                (*io_43)
                                    .value_
                                    .i = count
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulonglong)
                                    as lua_Integer;
                                idx = (idx as lua_Unsigned)
                                    .wrapping_add(step as lua_Unsigned) as lua_Integer;
                                let mut io_44: *mut TValue = &mut (*ra_71).val;
                                (*io_44).value_.i = idx;
                                let mut io_45: *mut TValue = &mut (*ra_71
                                    .offset(3 as libc::c_int as isize))
                                    .val;
                                (*io_45).value_.i = idx;
                                (*io_45)
                                    .tt_ = (3 as libc::c_int
                                    | (0 as libc::c_int) << 4 as libc::c_int) as lu_byte;
                                pc = pc
                                    .offset(
                                        -((i
                                            >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                            & !(!(0 as libc::c_int as Instruction)
                                                << 8 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int)
                                                << 0 as libc::c_int) as libc::c_int as isize),
                                    );
                            }
                        } else if floatforloop(ra_71) != 0 {
                            pc = pc
                                .offset(
                                    -((i
                                        >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        & !(!(0 as libc::c_int as Instruction)
                                            << 8 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int)
                                            << 0 as libc::c_int) as libc::c_int as isize),
                                );
                        }
                        trap = (*ci).u.l.trap;
                        continue;
                    }
                    74 => {
                        let mut ra_72: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        (*ci).u.l.savedpc = pc;
                        (*L).top.p = (*ci).top.p;
                        if forprep(L, ra_72) != 0 {
                            pc = pc
                                .offset(
                                    ((i
                                        >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                        & !(!(0 as libc::c_int as Instruction)
                                            << 8 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int)
                                            << 0 as libc::c_int) as libc::c_int + 1 as libc::c_int)
                                        as isize,
                                );
                        }
                        continue;
                    }
                    75 => {
                        let mut ra_73: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        (*ci).u.l.savedpc = pc;
                        (*L).top.p = (*ci).top.p;
                        luaF_newtbcupval(L, ra_73.offset(3 as libc::c_int as isize));
                        pc = pc
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction)
                                        << 8 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let fresh7 = pc;
                        pc = pc.offset(1);
                        i = *fresh7;
                        current_block = 13973394567113199817;
                    }
                    76 => {
                        current_block = 13973394567113199817;
                    }
                    77 => {
                        current_block = 15611964311717037170;
                    }
                    78 => {
                        let mut ra_76: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut n_4: libc::c_int = (i
                            >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                + 1 as libc::c_int
                            & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                << 0 as libc::c_int) as libc::c_int;
                        let mut last: libc::c_uint = (i
                            >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                + 1 as libc::c_int + 8 as libc::c_int
                            & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                << 0 as libc::c_int) as libc::c_int as libc::c_uint;
                        let mut h: *mut Table = &mut (*((*ra_76).val.value_.gc
                            as *mut GCUnion))
                            .h;
                        if n_4 == 0 as libc::c_int {
                            n_4 = ((*L).top.p).offset_from(ra_76) as libc::c_long
                                as libc::c_int - 1 as libc::c_int;
                        } else {
                            (*L).top.p = (*ci).top.p;
                        }
                        last = last.wrapping_add(n_4 as libc::c_uint);
                        if (i
                            & (1 as libc::c_uint)
                                << 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int)
                            as libc::c_int != 0
                        {
                            last = last
                                .wrapping_add(
                                    ((*pc >> 0 as libc::c_int + 7 as libc::c_int
                                        & !(!(0 as libc::c_int as Instruction)
                                            << 8 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int
                                                + 8 as libc::c_int) << 0 as libc::c_int) as libc::c_int
                                        * (((1 as libc::c_int) << 8 as libc::c_int)
                                            - 1 as libc::c_int + 1 as libc::c_int)) as libc::c_uint,
                                );
                            pc = pc.offset(1);
                            pc;
                        }
                        if last > luaH_realasize(h) {
                            luaH_resizearray(L, h, last);
                        }
                        while n_4 > 0 as libc::c_int {
                            let mut val: *mut TValue = &mut (*ra_76.offset(n_4 as isize))
                                .val;
                            let mut io1_17: *mut TValue = &mut *((*h).array)
                                .offset(
                                    last.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                                ) as *mut TValue;
                            let mut io2_17: *const TValue = val;
                            (*io1_17).value_ = (*io2_17).value_;
                            (*io1_17).tt_ = (*io2_17).tt_;
                            last = last.wrapping_sub(1);
                            last;
                            if (*val).tt_ as libc::c_int
                                & (1 as libc::c_int) << 6 as libc::c_int != 0
                            {
                                if (*(h as *mut GCUnion)).gc.marked as libc::c_int
                                    & (1 as libc::c_int) << 5 as libc::c_int != 0
                                    && (*(*val).value_.gc).marked as libc::c_int
                                        & ((1 as libc::c_int) << 3 as libc::c_int
                                            | (1 as libc::c_int) << 4 as libc::c_int) != 0
                                {
                                    luaC_barrierback_(L, &mut (*(h as *mut GCUnion)).gc);
                                } else {};
                            } else {};
                            n_4 -= 1;
                            n_4;
                        }
                        continue;
                    }
                    79 => {
                        let mut ra_77: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut p: *mut Proto = *((*(*cl).p).p)
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction)
                                        << 8 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        (*ci).u.l.savedpc = pc;
                        (*L).top.p = (*ci).top.p;
                        pushclosure(L, p, ((*cl).upvals).as_mut_ptr(), base, ra_77);
                        if (*(*L).l_G).GCdebt > 0 as libc::c_int as libc::c_long {
                            (*ci).u.l.savedpc = pc;
                            (*L).top.p = ra_77.offset(1 as libc::c_int as isize);
                            luaC_step(L);
                            trap = (*ci).u.l.trap;
                        }
                        continue;
                    }
                    80 => {
                        let mut ra_78: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        let mut n_5: libc::c_int = (i
                            >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                + 1 as libc::c_int + 8 as libc::c_int
                            & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                << 0 as libc::c_int) as libc::c_int - 1 as libc::c_int;
                        (*ci).u.l.savedpc = pc;
                        (*L).top.p = (*ci).top.p;
                        luaT_getvarargs(L, ci, ra_78, n_5);
                        trap = (*ci).u.l.trap;
                        continue;
                    }
                    81 => {
                        (*ci).u.l.savedpc = pc;
                        luaT_adjustvarargs(
                            L,
                            (i >> 0 as libc::c_int + 7 as libc::c_int
                                & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                    << 0 as libc::c_int) as libc::c_int,
                            ci,
                            (*cl).p,
                        );
                        trap = (*ci).u.l.trap;
                        if (trap != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
                        {
                            luaD_hookcall(L, ci);
                            (*L).oldpc = 1 as libc::c_int;
                        }
                        base = ((*ci).func.p).offset(1 as libc::c_int as isize);
                        continue;
                    }
                    82 | _ => {
                        continue;
                    }
                }
                match current_block {
                    13973394567113199817 => {
                        let mut ra_74: StkId = base
                            .offset(
                                (i >> 0 as libc::c_int + 7 as libc::c_int
                                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                        << 0 as libc::c_int) as libc::c_int as isize,
                            );
                        memcpy(
                            ra_74.offset(4 as libc::c_int as isize) as *mut libc::c_void,
                            ra_74 as *const libc::c_void,
                            (3 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(
                                    ::core::mem::size_of::<StackValue>() as libc::c_ulong,
                                ),
                        );
                        (*L)
                            .top
                            .p = ra_74
                            .offset(4 as libc::c_int as isize)
                            .offset(3 as libc::c_int as isize);
                        (*ci).u.l.savedpc = pc;
                        luaD_call(
                            L,
                            ra_74.offset(4 as libc::c_int as isize),
                            (i
                                >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                    + 1 as libc::c_int + 8 as libc::c_int
                                & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                    << 0 as libc::c_int) as libc::c_int,
                        );
                        trap = (*ci).u.l.trap;
                        if (trap != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
                        {
                            base = ((*ci).func.p).offset(1 as libc::c_int as isize);
                            ra_74 = base
                                .offset(
                                    (i >> 0 as libc::c_int + 7 as libc::c_int
                                        & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                            << 0 as libc::c_int) as libc::c_int as isize,
                                );
                        }
                        let fresh8 = pc;
                        pc = pc.offset(1);
                        i = *fresh8;
                    }
                    _ => {}
                }
                let mut ra_75: StkId = base
                    .offset(
                        (i >> 0 as libc::c_int + 7 as libc::c_int
                            & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                << 0 as libc::c_int) as libc::c_int as isize,
                    );
                if !((*ra_75.offset(4 as libc::c_int as isize)).val.tt_ as libc::c_int
                    & 0xf as libc::c_int == 0 as libc::c_int)
                {
                    let mut io1_16: *mut TValue = &mut (*ra_75
                        .offset(2 as libc::c_int as isize))
                        .val;
                    let mut io2_16: *const TValue = &mut (*ra_75
                        .offset(4 as libc::c_int as isize))
                        .val;
                    (*io1_16).value_ = (*io2_16).value_;
                    (*io1_16).tt_ = (*io2_16).tt_;
                    pc = pc
                        .offset(
                            -((i
                                >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                & !(!(0 as libc::c_int as Instruction)
                                    << 8 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int)
                                    << 0 as libc::c_int) as libc::c_int as isize),
                        );
                }
            }
            if (*ci).callstatus as libc::c_int & (1 as libc::c_int) << 2 as libc::c_int
                != 0
            {
                break '_startfunc;
            }
            ci = (*ci).previous;
        }
        ci = newci;
    };
}
