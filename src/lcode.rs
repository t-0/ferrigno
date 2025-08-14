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
    pub type BlockCnt;
    fn ldexp(_: f64, _: i32) -> f64;
    fn abs(_: i32) -> i32;
    fn luaO_ceillog2(x: u32) -> i32;
    fn luaO_rawarith(
        L: *mut lua_State,
        op: i32,
        p1: *const TValue,
        p2: *const TValue,
        res: *mut TValue,
    ) -> i32;
    fn luaM_growaux_(
        L: *mut lua_State,
        block: *mut libc::c_void,
        nelems: i32,
        size: *mut i32,
        size_elem: i32,
        limit: i32,
        what: *const libc::c_char,
    ) -> *mut libc::c_void;
    fn luaX_syntaxerror(ls: *mut LexState, s: *const libc::c_char) -> !;
    fn luaY_nvarstack(fs: *mut FuncState) -> i32;
    static luaP_opmodes: [u8; 83];
    fn luaC_barrier_(L: *mut lua_State, o: *mut GCObject, v: *mut GCObject);
    fn luaH_get(t: *mut Table, key: *const TValue) -> *const TValue;
    fn luaH_finishset(
        L: *mut lua_State,
        t: *mut Table,
        key: *const TValue,
        slot: *const TValue,
        value: *mut TValue,
    );
    fn luaV_equalobj(L: *mut lua_State, t1: *const TValue, t2: *const TValue) -> i32;
    fn luaV_tointegerns(obj: *const TValue, p: *mut i64, mode: F2Imod) -> i32;
    fn luaV_flttointeger(n: f64, p: *mut i64, mode: F2Imod) -> i32;
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
pub type lua_Alloc = Option<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void, u64, u64) -> *mut libc::c_void,
>;

pub type lua_Reader = Option<
    unsafe extern "C" fn(*mut lua_State, *mut libc::c_void, *mut u64) -> *const libc::c_char,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZIO {
    pub n: u64,
    pub p: *const libc::c_char,
    pub reader: lua_Reader,
    pub data: *mut libc::c_void,
    pub L: *mut lua_State,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Mbuffer {
    pub buffer: *mut libc::c_char,
    pub n: u64,
    pub buffsize: u64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union SemInfo {
    pub r: f64,
    pub i: i64,
    pub ts: *mut TString,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Token {
    pub token: i32,
    pub seminfo: SemInfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LexState {
    pub current: i32,
    pub linenumber: i32,
    pub lastline: i32,
    pub t: Token,
    pub lookahead: Token,
    pub fs: *mut FuncState,
    pub L: *mut lua_State,
    pub z: *mut ZIO,
    pub buff: *mut Mbuffer,
    pub h: *mut Table,
    pub dyd: *mut Dyndata,
    pub source: *mut TString,
    pub envn: *mut TString,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dyndata {
    pub actvar: C2RustUnnamed_9,
    pub gt: Labellist,
    pub label: Labellist,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Labellist {
    pub arr: *mut Labeldesc,
    pub n: i32,
    pub size: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Labeldesc {
    pub name: *mut TString,
    pub pc: i32,
    pub line: i32,
    pub nactvar: u8,
    pub close: u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub arr: *mut Vardesc,
    pub n: i32,
    pub size: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union Vardesc {
    pub vd: C2RustUnnamed_10,
    pub k: TValue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub value_: Value,
    pub tt_: u8,
    pub kind: u8,
    pub ridx: u8,
    pub pidx: libc::c_short,
    pub name: *mut TString,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FuncState {
    pub f: *mut Proto,
    pub prev: *mut FuncState,
    pub ls: *mut LexState,
    pub bl: *mut BlockCnt,
    pub pc: i32,
    pub lasttarget: i32,
    pub previousline: i32,
    pub nk: i32,
    pub np: i32,
    pub nabslineinfo: i32,
    pub firstlocal: i32,
    pub firstlabel: i32,
    pub ndebugvars: libc::c_short,
    pub nactvar: u8,
    pub nups: u8,
    pub freereg: u8,
    pub iwthabs: u8,
    pub needclose: u8,
}
pub type OpCode = u32;
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
pub type expkind = u32;
pub const VVARARG: expkind = 19;
pub const VCALL: expkind = 18;
pub const VRELOC: expkind = 17;
pub const VJMP: expkind = 16;
pub const VINDEXSTR: expkind = 15;
pub const VINDEXI: expkind = 14;
pub const VINDEXUP: expkind = 13;
pub const VINDEXED: expkind = 12;
pub const VCONST: expkind = 11;
pub const VUPVAL: expkind = 10;
pub const VLOCAL: expkind = 9;
pub const VNONRELOC: expkind = 8;
pub const VKSTR: expkind = 7;
pub const VKINT: expkind = 6;
pub const VKFLT: expkind = 5;
pub const VK: expkind = 4;
pub const VFALSE: expkind = 3;
pub const VTRUE: expkind = 2;
pub const VNIL: expkind = 1;
pub const VVOID: expkind = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct expdesc {
    pub k: expkind,
    pub u: C2RustUnnamed_11,
    pub t: i32,
    pub f: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
    pub ival: i64,
    pub nval: f64,
    pub strval: *mut TString,
    pub info: i32,
    pub ind: C2RustUnnamed_13,
    pub var: C2RustUnnamed_12,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub ridx: u8,
    pub vidx: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub index: libc::c_short,
    pub t: u8,
}
pub type BinOpr = u32;
pub const OPR_NOBINOPR: BinOpr = 21;
pub const OPR_OR: BinOpr = 20;
pub const OPR_AND: BinOpr = 19;
pub const OPR_GE: BinOpr = 18;
pub const OPR_GT: BinOpr = 17;
pub const OPR_NE: BinOpr = 16;
pub const OPR_LE: BinOpr = 15;
pub const OPR_LT: BinOpr = 14;
pub const OPR_EQ: BinOpr = 13;
pub const OPR_CONCAT: BinOpr = 12;
pub const OPR_SHR: BinOpr = 11;
pub const OPR_SHL: BinOpr = 10;
pub const OPR_BXOR: BinOpr = 9;
pub const OPR_BOR: BinOpr = 8;
pub const OPR_BAND: BinOpr = 7;
pub const OPR_IDIV: BinOpr = 6;
pub const OPR_DIV: BinOpr = 5;
pub const OPR_POW: BinOpr = 4;
pub const OPR_MOD: BinOpr = 3;
pub const OPR_MUL: BinOpr = 2;
pub const OPR_SUB: BinOpr = 1;
pub const OPR_ADD: BinOpr = 0;
pub type UnOpr = u32;
pub const OPR_NOUNOPR: UnOpr = 4;
pub const OPR_LEN: UnOpr = 3;
pub const OPR_NOT: UnOpr = 2;
pub const OPR_BNOT: UnOpr = 1;
pub const OPR_MINUS: UnOpr = 0;
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaK_semerror(mut ls: *mut LexState, mut msg: *const libc::c_char) -> ! {
    (*ls).t.token = 0i32;
    luaX_syntaxerror(ls, msg);
}
unsafe extern "C" fn tonumeral(mut e: *const expdesc, mut v: *mut TValue) -> i32 {
    if (*e).t != (*e).f {
        return 0i32;
    }
    match (*e).k as u32 {
        6 => {
            if !v.is_null() {
                let mut io: *mut TValue = v;
                (*io).value_.i = (*e).u.ival;
                (*io).tt_ = (3i32 | (0i32) << 4i32) as u8;
            }
            return 1i32;
        }
        5 => {
            if !v.is_null() {
                let mut io_0: *mut TValue = v;
                (*io_0).value_.n = (*e).u.nval;
                (*io_0).tt_ = (3i32 | (1i32) << 4i32) as u8;
            }
            return 1i32;
        }
        _ => return 0i32,
    };
}
unsafe extern "C" fn const2val(mut fs: *mut FuncState, mut e: *const expdesc) -> *mut TValue {
    return &mut (*((*(*(*fs).ls).dyd).actvar.arr).offset((*e).u.info as isize)).k;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaK_exp2const(
    mut fs: *mut FuncState,
    mut e: *const expdesc,
    mut v: *mut TValue,
) -> i32 {
    if (*e).t != (*e).f {
        return 0i32;
    }
    match (*e).k as u32 {
        3 => {
            (*v).tt_ = (1i32 | (0i32) << 4i32) as u8;
            return 1i32;
        }
        2 => {
            (*v).tt_ = (1i32 | (1i32) << 4i32) as u8;
            return 1i32;
        }
        1 => {
            (*v).tt_ = (0i32 | (0i32) << 4i32) as u8;
            return 1i32;
        }
        7 => {
            let mut io: *mut TValue = v;
            let mut x_: *mut TString = (*e).u.strval;
            (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
            (*io).tt_ = ((*x_).tt as i32 | (1i32) << 6i32) as u8;
            return 1i32;
        }
        11 => {
            let mut io1: *mut TValue = v;
            let mut io2: *const TValue = const2val(fs, e);
            (*io1).value_ = (*io2).value_;
            (*io1).tt_ = (*io2).tt_;
            return 1i32;
        }
        _ => return tonumeral(e, v),
    };
}
unsafe extern "C" fn previousinstruction(mut fs: *mut FuncState) -> *mut Instruction {
    static mut invalidinstruction: Instruction = !(0i32 as Instruction);
    if (*fs).pc > (*fs).lasttarget {
        return &mut *((*(*fs).f).code).offset(((*fs).pc - 1i32) as isize) as *mut Instruction;
    } else {
        return &invalidinstruction as *const Instruction as *mut Instruction;
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaK_nil(mut fs: *mut FuncState, mut from: i32, mut n: i32) {
    let mut l: i32 = from + n - 1i32;
    let mut previous: *mut Instruction = previousinstruction(fs);
    if (*previous >> 0i32 & !(!(0i32 as Instruction) << 7i32) << 0i32) as OpCode as u32
        == OP_LOADNIL as i32 as u32
    {
        let mut pfrom: i32 =
            (*previous >> 0i32 + 7i32 & !(!(0i32 as Instruction) << 8i32) << 0i32) as i32;
        let mut pl: i32 = pfrom
            + (*previous >> 0i32 + 7i32 + 8i32 + 1i32 & !(!(0i32 as Instruction) << 8i32) << 0i32)
                as i32;
        if pfrom <= from && from <= pl + 1i32 || from <= pfrom && pfrom <= l + 1i32 {
            if pfrom < from {
                from = pfrom;
            }
            if pl > l {
                l = pl;
            }
            *previous = *previous & !(!(!(0i32 as Instruction) << 8i32) << 0i32 + 7i32)
                | (from as Instruction) << 0i32 + 7i32
                    & !(!(0i32 as Instruction) << 8i32) << 0i32 + 7i32;
            *previous = *previous
                & !(!(!(0i32 as Instruction) << 8i32) << 0i32 + 7i32 + 8i32 + 1i32)
                | ((l - from) as Instruction) << 0i32 + 7i32 + 8i32 + 1i32
                    & !(!(0i32 as Instruction) << 8i32) << 0i32 + 7i32 + 8i32 + 1i32;
            return;
        }
    }
    luaK_codeABCk(fs, OP_LOADNIL, from, n - 1i32, 0i32, 0i32);
}
unsafe extern "C" fn getjump(mut fs: *mut FuncState, mut pc: i32) -> i32 {
    let mut offset: i32 = (*((*(*fs).f).code).offset(pc as isize) >> 0i32 + 7i32
        & !(!(0i32 as Instruction) << 8i32 + 8i32 + 1i32 + 8i32) << 0i32)
        as i32
        - (((1i32) << 8i32 + 8i32 + 1i32 + 8i32) - 1i32 >> 1i32);
    if offset == -(1i32) {
        return -(1i32);
    } else {
        return pc + 1i32 + offset;
    };
}
unsafe extern "C" fn fixjump(mut fs: *mut FuncState, mut pc: i32, mut dest: i32) {
    let mut jmp: *mut Instruction = &mut *((*(*fs).f).code).offset(pc as isize) as *mut Instruction;
    let mut offset: i32 = dest - (pc + 1i32);
    if !(-(((1i32) << 8i32 + 8i32 + 1i32 + 8i32) - 1i32 >> 1i32) <= offset
        && offset
            <= ((1i32) << 8i32 + 8i32 + 1i32 + 8i32)
                - 1i32
                - (((1i32) << 8i32 + 8i32 + 1i32 + 8i32) - 1i32 >> 1i32))
    {
        luaX_syntaxerror(
            (*fs).ls,
            b"control structure too long\0" as *const u8 as *const libc::c_char,
        );
    }
    *jmp = *jmp & !(!(!(0i32 as Instruction) << 8i32 + 8i32 + 1i32 + 8i32) << 0i32 + 7i32)
        | ((offset + (((1i32) << 8i32 + 8i32 + 1i32 + 8i32) - 1i32 >> 1i32)) as u32) << 0i32 + 7i32
            & !(!(0i32 as Instruction) << 8i32 + 8i32 + 1i32 + 8i32) << 0i32 + 7i32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaK_concat(mut fs: *mut FuncState, mut l1: *mut i32, mut l2: i32) {
    if l2 == -(1i32) {
        return;
    } else if *l1 == -(1i32) {
        *l1 = l2;
    } else {
        let mut list: i32 = *l1;
        let mut next: i32 = 0;
        loop {
            next = getjump(fs, list);
            if !(next != -(1i32)) {
                break;
            }
            list = next;
        }
        fixjump(fs, list, l2);
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaK_jump(mut fs: *mut FuncState) -> i32 {
    return codesJ(fs, OP_JMP, -(1i32), 0i32);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaK_ret(mut fs: *mut FuncState, mut first: i32, mut nret: i32) {
    let mut op: OpCode = OP_MOVE;
    match nret {
        0 => {
            op = OP_RETURN0;
        }
        1 => {
            op = OP_RETURN1;
        }
        _ => {
            op = OP_RETURN;
        }
    }
    luaK_codeABCk(fs, op, first, nret + 1i32, 0i32, 0i32);
}
unsafe extern "C" fn condjump(
    mut fs: *mut FuncState,
    mut op: OpCode,
    mut A: i32,
    mut B: i32,
    mut C: i32,
    mut k: i32,
) -> i32 {
    luaK_codeABCk(fs, op, A, B, C, k);
    return luaK_jump(fs);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaK_getlabel(mut fs: *mut FuncState) -> i32 {
    (*fs).lasttarget = (*fs).pc;
    return (*fs).pc;
}
unsafe extern "C" fn getjumpcontrol(mut fs: *mut FuncState, mut pc: i32) -> *mut Instruction {
    let mut pi: *mut Instruction = &mut *((*(*fs).f).code).offset(pc as isize) as *mut Instruction;
    if pc >= 1i32
        && luaP_opmodes[(*pi.offset(-(1i32 as isize)) >> 0i32
            & !(!(0i32 as Instruction) << 7i32) << 0i32) as OpCode as usize] as i32
            & (1i32) << 4i32
            != 0
    {
        return pi.offset(-(1i32 as isize));
    } else {
        return pi;
    };
}
unsafe extern "C" fn patchtestreg(mut fs: *mut FuncState, mut node: i32, mut reg: i32) -> i32 {
    let mut i: *mut Instruction = getjumpcontrol(fs, node);
    if (*i >> 0i32 & !(!(0i32 as Instruction) << 7i32) << 0i32) as OpCode as u32
        != OP_TESTSET as i32 as u32
    {
        return 0i32;
    }
    if reg != ((1i32) << 8i32) - 1i32
        && reg
            != (*i >> 0i32 + 7i32 + 8i32 + 1i32 & !(!(0i32 as Instruction) << 8i32) << 0i32) as i32
    {
        *i = *i & !(!(!(0i32 as Instruction) << 8i32) << 0i32 + 7i32)
            | (reg as Instruction) << 0i32 + 7i32
                & !(!(0i32 as Instruction) << 8i32) << 0i32 + 7i32;
    } else {
        *i = (OP_TEST as i32 as Instruction) << 0i32
            | ((*i >> 0i32 + 7i32 + 8i32 + 1i32 & !(!(0i32 as Instruction) << 8i32) << 0i32) as i32
                as Instruction)
                << 0i32 + 7i32
            | (0i32 as Instruction) << 0i32 + 7i32 + 8i32 + 1i32
            | (0i32 as Instruction) << 0i32 + 7i32 + 8i32 + 1i32 + 8i32
            | ((*i >> 0i32 + 7i32 + 8i32 & !(!(0i32 as Instruction) << 1i32) << 0i32) as i32
                as Instruction)
                << 0i32 + 7i32 + 8i32;
    }
    return 1i32;
}
unsafe extern "C" fn removevalues(mut fs: *mut FuncState, mut list: i32) {
    while list != -(1i32) {
        patchtestreg(fs, list, ((1i32) << 8i32) - 1i32);
        list = getjump(fs, list);
    }
}
unsafe extern "C" fn patchlistaux(
    mut fs: *mut FuncState,
    mut list: i32,
    mut vtarget: i32,
    mut reg: i32,
    mut dtarget: i32,
) {
    while list != -(1i32) {
        let mut next: i32 = getjump(fs, list);
        if patchtestreg(fs, list, reg) != 0 {
            fixjump(fs, list, vtarget);
        } else {
            fixjump(fs, list, dtarget);
        }
        list = next;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaK_patchlist(mut fs: *mut FuncState, mut list: i32, mut target: i32) {
    patchlistaux(fs, list, target, ((1i32) << 8i32) - 1i32, target);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaK_patchtohere(mut fs: *mut FuncState, mut list: i32) {
    let mut hr: i32 = luaK_getlabel(fs);
    luaK_patchlist(fs, list, hr);
}
unsafe extern "C" fn savelineinfo(mut fs: *mut FuncState, mut f: *mut Proto, mut line: i32) {
    let mut linedif: i32 = line - (*fs).previousline;
    let mut pc: i32 = (*fs).pc - 1i32;
    if abs(linedif) >= 0x80 as i32 || {
        let fresh0 = (*fs).iwthabs;
        (*fs).iwthabs = ((*fs).iwthabs).wrapping_add(1);
        fresh0 as i32 >= 128i32
    } {
        (*f).abslineinfo = luaM_growaux_(
            (*(*fs).ls).L,
            (*f).abslineinfo as *mut libc::c_void,
            (*fs).nabslineinfo,
            &mut (*f).sizeabslineinfo,
            ::core::mem::size_of::<AbsLineInfo>() as libc::c_ulong as i32,
            (if 2147483647i32 as u64
                <= (!(0i32 as u64))
                    .wrapping_div(::core::mem::size_of::<AbsLineInfo>() as libc::c_ulong)
            {
                2147483647i32 as u32
            } else {
                (!(0i32 as u64))
                    .wrapping_div(::core::mem::size_of::<AbsLineInfo>() as libc::c_ulong)
                    as u32
            }) as i32,
            b"lines\0" as *const u8 as *const libc::c_char,
        ) as *mut AbsLineInfo;
        (*((*f).abslineinfo).offset((*fs).nabslineinfo as isize)).pc = pc;
        let fresh1 = (*fs).nabslineinfo;
        (*fs).nabslineinfo = (*fs).nabslineinfo + 1;
        (*((*f).abslineinfo).offset(fresh1 as isize)).line = line;
        linedif = -(0x80 as i32);
        (*fs).iwthabs = 1i32 as u8;
    }
    (*f).lineinfo = luaM_growaux_(
        (*(*fs).ls).L,
        (*f).lineinfo as *mut libc::c_void,
        pc,
        &mut (*f).sizelineinfo,
        ::core::mem::size_of::<ls_byte>() as libc::c_ulong as i32,
        (if 2147483647i32 as u64
            <= (!(0i32 as u64)).wrapping_div(::core::mem::size_of::<ls_byte>() as libc::c_ulong)
        {
            2147483647i32 as u32
        } else {
            (!(0i32 as u64)).wrapping_div(::core::mem::size_of::<ls_byte>() as libc::c_ulong) as u32
        }) as i32,
        b"opcodes\0" as *const u8 as *const libc::c_char,
    ) as *mut ls_byte;
    *((*f).lineinfo).offset(pc as isize) = linedif as ls_byte;
    (*fs).previousline = line;
}
unsafe extern "C" fn removelastlineinfo(mut fs: *mut FuncState) {
    let mut f: *mut Proto = (*fs).f;
    let mut pc: i32 = (*fs).pc - 1i32;
    if *((*f).lineinfo).offset(pc as isize) as i32 != -(0x80 as i32) {
        (*fs).previousline -= *((*f).lineinfo).offset(pc as isize) as i32;
        (*fs).iwthabs = ((*fs).iwthabs).wrapping_sub(1);
        (*fs).iwthabs;
    } else {
        (*fs).nabslineinfo -= 1;
        (*fs).nabslineinfo;
        (*fs).iwthabs = (128i32 + 1i32) as u8;
    };
}
unsafe extern "C" fn removelastinstruction(mut fs: *mut FuncState) {
    removelastlineinfo(fs);
    (*fs).pc -= 1;
    (*fs).pc;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaK_code(mut fs: *mut FuncState, mut i: Instruction) -> i32 {
    let mut f: *mut Proto = (*fs).f;
    (*f).code = luaM_growaux_(
        (*(*fs).ls).L,
        (*f).code as *mut libc::c_void,
        (*fs).pc,
        &mut (*f).sizecode,
        ::core::mem::size_of::<Instruction>() as libc::c_ulong as i32,
        (if 2147483647i32 as u64
            <= (!(0i32 as u64)).wrapping_div(::core::mem::size_of::<Instruction>() as libc::c_ulong)
        {
            2147483647i32 as u32
        } else {
            (!(0i32 as u64)).wrapping_div(::core::mem::size_of::<Instruction>() as libc::c_ulong)
                as u32
        }) as i32,
        b"opcodes\0" as *const u8 as *const libc::c_char,
    ) as *mut Instruction;
    let fresh2 = (*fs).pc;
    (*fs).pc = (*fs).pc + 1;
    *((*f).code).offset(fresh2 as isize) = i;
    savelineinfo(fs, f, (*(*fs).ls).lastline);
    return (*fs).pc - 1i32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaK_codeABCk(
    mut fs: *mut FuncState,
    mut o: OpCode,
    mut a: i32,
    mut b: i32,
    mut c: i32,
    mut k: i32,
) -> i32 {
    return luaK_code(
        fs,
        (o as Instruction) << 0i32
            | (a as Instruction) << 0i32 + 7i32
            | (b as Instruction) << 0i32 + 7i32 + 8i32 + 1i32
            | (c as Instruction) << 0i32 + 7i32 + 8i32 + 1i32 + 8i32
            | (k as Instruction) << 0i32 + 7i32 + 8i32,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaK_codeABx(
    mut fs: *mut FuncState,
    mut o: OpCode,
    mut a: i32,
    mut bc: u32,
) -> i32 {
    return luaK_code(
        fs,
        (o as Instruction) << 0i32 | (a as Instruction) << 0i32 + 7i32 | bc << 0i32 + 7i32 + 8i32,
    );
}
unsafe extern "C" fn codeAsBx(
    mut fs: *mut FuncState,
    mut o: OpCode,
    mut a: i32,
    mut bc: i32,
) -> i32 {
    let mut b: u32 = (bc + (((1i32) << 8i32 + 8i32 + 1i32) - 1i32 >> 1i32)) as u32;
    return luaK_code(
        fs,
        (o as Instruction) << 0i32 | (a as Instruction) << 0i32 + 7i32 | b << 0i32 + 7i32 + 8i32,
    );
}
unsafe extern "C" fn codesJ(mut fs: *mut FuncState, mut o: OpCode, mut sj: i32, mut k: i32) -> i32 {
    let mut j: u32 = (sj + (((1i32) << 8i32 + 8i32 + 1i32 + 8i32) - 1i32 >> 1i32)) as u32;
    return luaK_code(
        fs,
        (o as Instruction) << 0i32 | j << 0i32 + 7i32 | (k as Instruction) << 0i32 + 7i32 + 8i32,
    );
}
unsafe extern "C" fn codeextraarg(mut fs: *mut FuncState, mut a: i32) -> i32 {
    return luaK_code(
        fs,
        (OP_EXTRAARG as i32 as Instruction) << 0i32 | (a as Instruction) << 0i32 + 7i32,
    );
}
unsafe extern "C" fn luaK_codek(mut fs: *mut FuncState, mut reg: i32, mut k: i32) -> i32 {
    if k <= ((1i32) << 8i32 + 8i32 + 1i32) - 1i32 {
        return luaK_codeABx(fs, OP_LOADK, reg, k as u32);
    } else {
        let mut p: i32 = luaK_codeABx(fs, OP_LOADKX, reg, 0i32 as u32);
        codeextraarg(fs, k);
        return p;
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaK_checkstack(mut fs: *mut FuncState, mut n: i32) {
    let mut newstack: i32 = (*fs).freereg as i32 + n;
    if newstack > (*(*fs).f).maxstacksize as i32 {
        if newstack >= 255i32 {
            luaX_syntaxerror(
                (*fs).ls,
                b"function or expression needs too many registers\0" as *const u8
                    as *const libc::c_char,
            );
        }
        (*(*fs).f).maxstacksize = newstack as u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaK_reserveregs(mut fs: *mut FuncState, mut n: i32) {
    luaK_checkstack(fs, n);
    (*fs).freereg = ((*fs).freereg as i32 + n) as u8;
}
unsafe extern "C" fn freereg(mut fs: *mut FuncState, mut reg: i32) {
    if reg >= luaY_nvarstack(fs) {
        (*fs).freereg = ((*fs).freereg).wrapping_sub(1);
        (*fs).freereg;
    }
}
unsafe extern "C" fn freeregs(mut fs: *mut FuncState, mut r1: i32, mut r2: i32) {
    if r1 > r2 {
        freereg(fs, r1);
        freereg(fs, r2);
    } else {
        freereg(fs, r2);
        freereg(fs, r1);
    };
}
unsafe extern "C" fn freeexp(mut fs: *mut FuncState, mut e: *mut expdesc) {
    if (*e).k as u32 == VNONRELOC as i32 as u32 {
        freereg(fs, (*e).u.info);
    }
}
unsafe extern "C" fn freeexps(mut fs: *mut FuncState, mut e1: *mut expdesc, mut e2: *mut expdesc) {
    let mut r1: i32 = if (*e1).k as u32 == VNONRELOC as i32 as u32 {
        (*e1).u.info
    } else {
        -(1i32)
    };
    let mut r2: i32 = if (*e2).k as u32 == VNONRELOC as i32 as u32 {
        (*e2).u.info
    } else {
        -(1i32)
    };
    freeregs(fs, r1, r2);
}
unsafe extern "C" fn addk(mut fs: *mut FuncState, mut key: *mut TValue, mut v: *mut TValue) -> i32 {
    let mut val: TValue = TValue {
        value_: Value {
            gc: 0 as *mut GCObject,
        },
        tt_: 0,
    };
    let mut L: *mut lua_State = (*(*fs).ls).L;
    let mut f: *mut Proto = (*fs).f;
    let mut index: *const TValue = luaH_get((*(*fs).ls).h, key);
    let mut k: i32 = 0;
    let mut oldsize: i32 = 0;
    if (*index).tt_ as i32 == 3i32 | (0i32) << 4i32 {
        k = (*index).value_.i as i32;
        if k < (*fs).nk
            && (*((*f).k).offset(k as isize)).tt_ as i32 & 0x3f as i32
                == (*v).tt_ as i32 & 0x3f as i32
            && luaV_equalobj(0 as *mut lua_State, &mut *((*f).k).offset(k as isize), v) != 0
        {
            return k;
        }
    }
    oldsize = (*f).sizek;
    k = (*fs).nk;
    let mut io: *mut TValue = &mut val;
    (*io).value_.i = k as i64;
    (*io).tt_ = (3i32 | (0i32) << 4i32) as u8;
    luaH_finishset(L, (*(*fs).ls).h, key, index, &mut val);
    (*f).k = luaM_growaux_(
        L,
        (*f).k as *mut libc::c_void,
        k,
        &mut (*f).sizek,
        ::core::mem::size_of::<TValue>() as libc::c_ulong as i32,
        (if (((1i32) << 8i32 + 8i32 + 1i32 + 8i32) - 1i32) as u64
            <= (!(0i32 as u64)).wrapping_div(::core::mem::size_of::<TValue>() as libc::c_ulong)
        {
            (((1i32) << 8i32 + 8i32 + 1i32 + 8i32) - 1i32) as u32
        } else {
            (!(0i32 as u64)).wrapping_div(::core::mem::size_of::<TValue>() as libc::c_ulong) as u32
        }) as i32,
        b"constants\0" as *const u8 as *const libc::c_char,
    ) as *mut TValue;
    while oldsize < (*f).sizek {
        let fresh3 = oldsize;
        oldsize = oldsize + 1;
        (*((*f).k).offset(fresh3 as isize)).tt_ = (0i32 | (0i32) << 4i32) as u8;
    }
    let mut io1: *mut TValue = &mut *((*f).k).offset(k as isize) as *mut TValue;
    let mut io2: *const TValue = v;
    (*io1).value_ = (*io2).value_;
    (*io1).tt_ = (*io2).tt_;
    (*fs).nk += 1;
    (*fs).nk;
    if (*v).tt_ as i32 & (1i32) << 6i32 != 0 {
        if (*f).marked as i32 & (1i32) << 5i32 != 0
            && (*(*v).value_.gc).marked as i32 & ((1i32) << 3i32 | (1i32) << 4i32) != 0
        {
            luaC_barrier_(
                L,
                &mut (*(f as *mut GCUnion)).gc,
                &mut (*((*v).value_.gc as *mut GCUnion)).gc,
            );
        } else {
        };
    } else {
    };
    return k;
}
unsafe extern "C" fn stringK(mut fs: *mut FuncState, mut s: *mut TString) -> i32 {
    let mut o: TValue = TValue {
        value_: Value {
            gc: 0 as *mut GCObject,
        },
        tt_: 0,
    };
    let mut io: *mut TValue = &mut o;
    let mut x_: *mut TString = s;
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
    (*io).tt_ = ((*x_).tt as i32 | (1i32) << 6i32) as u8;
    return addk(fs, &mut o, &mut o);
}
unsafe extern "C" fn luaK_intK(mut fs: *mut FuncState, mut n: i64) -> i32 {
    let mut o: TValue = TValue {
        value_: Value {
            gc: 0 as *mut GCObject,
        },
        tt_: 0,
    };
    let mut io: *mut TValue = &mut o;
    (*io).value_.i = n;
    (*io).tt_ = (3i32 | (0i32) << 4i32) as u8;
    return addk(fs, &mut o, &mut o);
}
unsafe extern "C" fn luaK_numberK(mut fs: *mut FuncState, mut r: f64) -> i32 {
    let mut o: TValue = TValue {
        value_: Value {
            gc: 0 as *mut GCObject,
        },
        tt_: 0,
    };
    let mut ik: i64 = 0;
    let mut io: *mut TValue = &mut o;
    (*io).value_.n = r;
    (*io).tt_ = (3i32 | (1i32) << 4i32) as u8;
    if luaV_flttointeger(r, &mut ik, F2Ieq) == 0 {
        return addk(fs, &mut o, &mut o);
    } else {
        let nbm: i32 = 53i32;
        let q: f64 = ldexp(1.0f64, -nbm + 1i32);
        let k: f64 = if ik == 0i32 as i64 { q } else { r + r * q };
        let mut kv: TValue = TValue {
            value_: Value {
                gc: 0 as *mut GCObject,
            },
            tt_: 0,
        };
        let mut io_0: *mut TValue = &mut kv;
        (*io_0).value_.n = k;
        (*io_0).tt_ = (3i32 | (1i32) << 4i32) as u8;
        return addk(fs, &mut kv, &mut o);
    };
}
unsafe extern "C" fn boolF(mut fs: *mut FuncState) -> i32 {
    let mut o: TValue = TValue {
        value_: Value {
            gc: 0 as *mut GCObject,
        },
        tt_: 0,
    };
    o.tt_ = (1i32 | (0i32) << 4i32) as u8;
    return addk(fs, &mut o, &mut o);
}
unsafe extern "C" fn boolT(mut fs: *mut FuncState) -> i32 {
    let mut o: TValue = TValue {
        value_: Value {
            gc: 0 as *mut GCObject,
        },
        tt_: 0,
    };
    o.tt_ = (1i32 | (1i32) << 4i32) as u8;
    return addk(fs, &mut o, &mut o);
}
unsafe extern "C" fn nilK(mut fs: *mut FuncState) -> i32 {
    let mut k: TValue = TValue {
        value_: Value {
            gc: 0 as *mut GCObject,
        },
        tt_: 0,
    };
    let mut v: TValue = TValue {
        value_: Value {
            gc: 0 as *mut GCObject,
        },
        tt_: 0,
    };
    v.tt_ = (0i32 | (0i32) << 4i32) as u8;
    let mut io: *mut TValue = &mut k;
    let mut x_: *mut Table = (*(*fs).ls).h;
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
    (*io).tt_ = (5i32 | (0i32) << 4i32 | (1i32) << 6i32) as u8;
    return addk(fs, &mut k, &mut v);
}
unsafe extern "C" fn fitsC(mut i: i64) -> i32 {
    return ((i as u64).wrapping_add((((1i32) << 8i32) - 1i32 >> 1i32) as u64)
        <= (((1i32) << 8i32) - 1i32) as u32 as u64) as i32;
}
unsafe extern "C" fn fitsBx(mut i: i64) -> i32 {
    return (-(((1i32) << 8i32 + 8i32 + 1i32) - 1i32 >> 1i32) as i64 <= i
        && i <= (((1i32) << 8i32 + 8i32 + 1i32)
            - 1i32
            - (((1i32) << 8i32 + 8i32 + 1i32) - 1i32 >> 1i32)) as i64) as i32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaK_int(mut fs: *mut FuncState, mut reg: i32, mut i: i64) {
    if fitsBx(i) != 0 {
        codeAsBx(fs, OP_LOADI, reg, i as i32);
    } else {
        luaK_codek(fs, reg, luaK_intK(fs, i));
    };
}
unsafe extern "C" fn luaK_float(mut fs: *mut FuncState, mut reg: i32, mut f: f64) {
    let mut fi: i64 = 0;
    if luaV_flttointeger(f, &mut fi, F2Ieq) != 0 && fitsBx(fi) != 0 {
        codeAsBx(fs, OP_LOADF, reg, fi as i32);
    } else {
        luaK_codek(fs, reg, luaK_numberK(fs, f));
    };
}
unsafe extern "C" fn const2exp(mut v: *mut TValue, mut e: *mut expdesc) {
    match (*v).tt_ as i32 & 0x3f as i32 {
        3 => {
            (*e).k = VKINT;
            (*e).u.ival = (*v).value_.i;
        }
        19 => {
            (*e).k = VKFLT;
            (*e).u.nval = (*v).value_.n;
        }
        1 => {
            (*e).k = VFALSE;
        }
        17 => {
            (*e).k = VTRUE;
        }
        0 => {
            (*e).k = VNIL;
        }
        4 | 20 => {
            (*e).k = VKSTR;
            (*e).u.strval = &mut (*((*v).value_.gc as *mut GCUnion)).ts;
        }
        _ => {}
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaK_setreturns(
    mut fs: *mut FuncState,
    mut e: *mut expdesc,
    mut nresults: i32,
) {
    let mut pc: *mut Instruction =
        &mut *((*(*fs).f).code).offset((*e).u.info as isize) as *mut Instruction;
    if (*e).k as u32 == VCALL as i32 as u32 {
        *pc = *pc & !(!(!(0i32 as Instruction) << 8i32) << 0i32 + 7i32 + 8i32 + 1i32 + 8i32)
            | ((nresults + 1i32) as Instruction) << 0i32 + 7i32 + 8i32 + 1i32 + 8i32
                & !(!(0i32 as Instruction) << 8i32) << 0i32 + 7i32 + 8i32 + 1i32 + 8i32;
    } else {
        *pc = *pc & !(!(!(0i32 as Instruction) << 8i32) << 0i32 + 7i32 + 8i32 + 1i32 + 8i32)
            | ((nresults + 1i32) as Instruction) << 0i32 + 7i32 + 8i32 + 1i32 + 8i32
                & !(!(0i32 as Instruction) << 8i32) << 0i32 + 7i32 + 8i32 + 1i32 + 8i32;
        *pc = *pc & !(!(!(0i32 as Instruction) << 8i32) << 0i32 + 7i32)
            | ((*fs).freereg as Instruction) << 0i32 + 7i32
                & !(!(0i32 as Instruction) << 8i32) << 0i32 + 7i32;
        luaK_reserveregs(fs, 1i32);
    };
}
unsafe extern "C" fn str2K(mut fs: *mut FuncState, mut e: *mut expdesc) {
    (*e).u.info = stringK(fs, (*e).u.strval);
    (*e).k = VK;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaK_setoneret(mut fs: *mut FuncState, mut e: *mut expdesc) {
    if (*e).k as u32 == VCALL as i32 as u32 {
        (*e).k = VNONRELOC;
        (*e).u.info = (*((*(*fs).f).code).offset((*e).u.info as isize) >> 0i32 + 7i32
            & !(!(0i32 as Instruction) << 8i32) << 0i32) as i32;
    } else if (*e).k as u32 == VVARARG as i32 as u32 {
        *((*(*fs).f).code).offset((*e).u.info as isize) = *((*(*fs).f).code)
            .offset((*e).u.info as isize)
            & !(!(!(0i32 as Instruction) << 8i32) << 0i32 + 7i32 + 8i32 + 1i32 + 8i32)
            | (2i32 as Instruction) << 0i32 + 7i32 + 8i32 + 1i32 + 8i32
                & !(!(0i32 as Instruction) << 8i32) << 0i32 + 7i32 + 8i32 + 1i32 + 8i32;
        (*e).k = VRELOC;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaK_dischargevars(mut fs: *mut FuncState, mut e: *mut expdesc) {
    match (*e).k as u32 {
        11 => {
            const2exp(const2val(fs, e), e);
        }
        9 => {
            let mut temp: i32 = (*e).u.var.ridx as i32;
            (*e).u.info = temp;
            (*e).k = VNONRELOC;
        }
        10 => {
            (*e).u.info = luaK_codeABCk(fs, OP_GETUPVAL, 0i32, (*e).u.info, 0i32, 0i32);
            (*e).k = VRELOC;
        }
        13 => {
            (*e).u.info = luaK_codeABCk(
                fs,
                OP_GETTABUP,
                0i32,
                (*e).u.ind.t as i32,
                (*e).u.ind.index as i32,
                0i32,
            );
            (*e).k = VRELOC;
        }
        14 => {
            freereg(fs, (*e).u.ind.t as i32);
            (*e).u.info = luaK_codeABCk(
                fs,
                OP_GETI,
                0i32,
                (*e).u.ind.t as i32,
                (*e).u.ind.index as i32,
                0i32,
            );
            (*e).k = VRELOC;
        }
        15 => {
            freereg(fs, (*e).u.ind.t as i32);
            (*e).u.info = luaK_codeABCk(
                fs,
                OP_GETFIELD,
                0i32,
                (*e).u.ind.t as i32,
                (*e).u.ind.index as i32,
                0i32,
            );
            (*e).k = VRELOC;
        }
        12 => {
            freeregs(fs, (*e).u.ind.t as i32, (*e).u.ind.index as i32);
            (*e).u.info = luaK_codeABCk(
                fs,
                OP_GETTABLE,
                0i32,
                (*e).u.ind.t as i32,
                (*e).u.ind.index as i32,
                0i32,
            );
            (*e).k = VRELOC;
        }
        19 | 18 => {
            luaK_setoneret(fs, e);
        }
        _ => {}
    };
}
unsafe extern "C" fn discharge2reg(mut fs: *mut FuncState, mut e: *mut expdesc, mut reg: i32) {
    luaK_dischargevars(fs, e);
    let mut current_block_14: u64;
    match (*e).k as u32 {
        1 => {
            luaK_nil(fs, reg, 1i32);
            current_block_14 = 13242334135786603907;
        }
        3 => {
            luaK_codeABCk(fs, OP_LOADFALSE, reg, 0i32, 0i32, 0i32);
            current_block_14 = 13242334135786603907;
        }
        2 => {
            luaK_codeABCk(fs, OP_LOADTRUE, reg, 0i32, 0i32, 0i32);
            current_block_14 = 13242334135786603907;
        }
        7 => {
            str2K(fs, e);
            current_block_14 = 6937071982253665452;
        }
        4 => {
            current_block_14 = 6937071982253665452;
        }
        5 => {
            luaK_float(fs, reg, (*e).u.nval);
            current_block_14 = 13242334135786603907;
        }
        6 => {
            luaK_int(fs, reg, (*e).u.ival);
            current_block_14 = 13242334135786603907;
        }
        17 => {
            let mut pc: *mut Instruction =
                &mut *((*(*fs).f).code).offset((*e).u.info as isize) as *mut Instruction;
            *pc = *pc & !(!(!(0i32 as Instruction) << 8i32) << 0i32 + 7i32)
                | (reg as Instruction) << 0i32 + 7i32
                    & !(!(0i32 as Instruction) << 8i32) << 0i32 + 7i32;
            current_block_14 = 13242334135786603907;
        }
        8 => {
            if reg != (*e).u.info {
                luaK_codeABCk(fs, OP_MOVE, reg, (*e).u.info, 0i32, 0i32);
            }
            current_block_14 = 13242334135786603907;
        }
        _ => return,
    }
    match current_block_14 {
        6937071982253665452 => {
            luaK_codek(fs, reg, (*e).u.info);
        }
        _ => {}
    }
    (*e).u.info = reg;
    (*e).k = VNONRELOC;
}
unsafe extern "C" fn discharge2anyreg(mut fs: *mut FuncState, mut e: *mut expdesc) {
    if (*e).k as u32 != VNONRELOC as i32 as u32 {
        luaK_reserveregs(fs, 1i32);
        discharge2reg(fs, e, (*fs).freereg as i32 - 1i32);
    }
}
unsafe extern "C" fn code_loadbool(mut fs: *mut FuncState, mut A: i32, mut op: OpCode) -> i32 {
    luaK_getlabel(fs);
    return luaK_codeABCk(fs, op, A, 0i32, 0i32, 0i32);
}
unsafe extern "C" fn need_value(mut fs: *mut FuncState, mut list: i32) -> i32 {
    while list != -(1i32) {
        let mut i: Instruction = *getjumpcontrol(fs, list);
        if (i >> 0i32 & !(!(0i32 as Instruction) << 7i32) << 0i32) as OpCode as u32
            != OP_TESTSET as i32 as u32
        {
            return 1i32;
        }
        list = getjump(fs, list);
    }
    return 0i32;
}
unsafe extern "C" fn exp2reg(mut fs: *mut FuncState, mut e: *mut expdesc, mut reg: i32) {
    discharge2reg(fs, e, reg);
    if (*e).k as u32 == VJMP as i32 as u32 {
        luaK_concat(fs, &mut (*e).t, (*e).u.info);
    }
    if (*e).t != (*e).f {
        let mut final_0: i32 = 0;
        let mut p_f: i32 = -(1i32);
        let mut p_t: i32 = -(1i32);
        if need_value(fs, (*e).t) != 0 || need_value(fs, (*e).f) != 0 {
            let mut fj: i32 = if (*e).k as u32 == VJMP as i32 as u32 {
                -(1i32)
            } else {
                luaK_jump(fs)
            };
            p_f = code_loadbool(fs, reg, OP_LFALSESKIP);
            p_t = code_loadbool(fs, reg, OP_LOADTRUE);
            luaK_patchtohere(fs, fj);
        }
        final_0 = luaK_getlabel(fs);
        patchlistaux(fs, (*e).f, final_0, reg, p_f);
        patchlistaux(fs, (*e).t, final_0, reg, p_t);
    }
    (*e).t = -(1i32);
    (*e).f = (*e).t;
    (*e).u.info = reg;
    (*e).k = VNONRELOC;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaK_exp2nextreg(mut fs: *mut FuncState, mut e: *mut expdesc) {
    luaK_dischargevars(fs, e);
    freeexp(fs, e);
    luaK_reserveregs(fs, 1i32);
    exp2reg(fs, e, (*fs).freereg as i32 - 1i32);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaK_exp2anyreg(mut fs: *mut FuncState, mut e: *mut expdesc) -> i32 {
    luaK_dischargevars(fs, e);
    if (*e).k as u32 == VNONRELOC as i32 as u32 {
        if !((*e).t != (*e).f) {
            return (*e).u.info;
        }
        if (*e).u.info >= luaY_nvarstack(fs) {
            exp2reg(fs, e, (*e).u.info);
            return (*e).u.info;
        }
    }
    luaK_exp2nextreg(fs, e);
    return (*e).u.info;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaK_exp2anyregup(mut fs: *mut FuncState, mut e: *mut expdesc) {
    if (*e).k as u32 != VUPVAL as i32 as u32 || (*e).t != (*e).f {
        luaK_exp2anyreg(fs, e);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaK_exp2val(mut fs: *mut FuncState, mut e: *mut expdesc) {
    if (*e).k as u32 == VJMP as i32 as u32 || (*e).t != (*e).f {
        luaK_exp2anyreg(fs, e);
    } else {
        luaK_dischargevars(fs, e);
    };
}
unsafe extern "C" fn luaK_exp2K(mut fs: *mut FuncState, mut e: *mut expdesc) -> i32 {
    if !((*e).t != (*e).f) {
        let mut info: i32 = 0;
        match (*e).k as u32 {
            2 => {
                info = boolT(fs);
            }
            3 => {
                info = boolF(fs);
            }
            1 => {
                info = nilK(fs);
            }
            6 => {
                info = luaK_intK(fs, (*e).u.ival);
            }
            5 => {
                info = luaK_numberK(fs, (*e).u.nval);
            }
            7 => {
                info = stringK(fs, (*e).u.strval);
            }
            4 => {
                info = (*e).u.info;
            }
            _ => return 0i32,
        }
        if info <= ((1i32) << 8i32) - 1i32 {
            (*e).k = VK;
            (*e).u.info = info;
            return 1i32;
        }
    }
    return 0i32;
}
unsafe extern "C" fn exp2RK(mut fs: *mut FuncState, mut e: *mut expdesc) -> i32 {
    if luaK_exp2K(fs, e) != 0 {
        return 1i32;
    } else {
        luaK_exp2anyreg(fs, e);
        return 0i32;
    };
}
unsafe extern "C" fn codeABRK(
    mut fs: *mut FuncState,
    mut o: OpCode,
    mut a: i32,
    mut b: i32,
    mut ec: *mut expdesc,
) {
    let mut k: i32 = exp2RK(fs, ec);
    luaK_codeABCk(fs, o, a, b, (*ec).u.info, k);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaK_storevar(
    mut fs: *mut FuncState,
    mut var: *mut expdesc,
    mut ex: *mut expdesc,
) {
    match (*var).k as u32 {
        9 => {
            freeexp(fs, ex);
            exp2reg(fs, ex, (*var).u.var.ridx as i32);
            return;
        }
        10 => {
            let mut e: i32 = luaK_exp2anyreg(fs, ex);
            luaK_codeABCk(fs, OP_SETUPVAL, e, (*var).u.info, 0i32, 0i32);
        }
        13 => {
            codeABRK(
                fs,
                OP_SETTABUP,
                (*var).u.ind.t as i32,
                (*var).u.ind.index as i32,
                ex,
            );
        }
        14 => {
            codeABRK(
                fs,
                OP_SETI,
                (*var).u.ind.t as i32,
                (*var).u.ind.index as i32,
                ex,
            );
        }
        15 => {
            codeABRK(
                fs,
                OP_SETFIELD,
                (*var).u.ind.t as i32,
                (*var).u.ind.index as i32,
                ex,
            );
        }
        12 => {
            codeABRK(
                fs,
                OP_SETTABLE,
                (*var).u.ind.t as i32,
                (*var).u.ind.index as i32,
                ex,
            );
        }
        _ => {}
    }
    freeexp(fs, ex);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaK_self(
    mut fs: *mut FuncState,
    mut e: *mut expdesc,
    mut key: *mut expdesc,
) {
    let mut ereg: i32 = 0;
    luaK_exp2anyreg(fs, e);
    ereg = (*e).u.info;
    freeexp(fs, e);
    (*e).u.info = (*fs).freereg as i32;
    (*e).k = VNONRELOC;
    luaK_reserveregs(fs, 2i32);
    codeABRK(fs, OP_SELF, (*e).u.info, ereg, key);
    freeexp(fs, key);
}
unsafe extern "C" fn negatecondition(mut fs: *mut FuncState, mut e: *mut expdesc) {
    let mut pc: *mut Instruction = getjumpcontrol(fs, (*e).u.info);
    *pc = *pc & !(!(!(0i32 as Instruction) << 1i32) << 0i32 + 7i32 + 8i32)
        | (((*pc >> 0i32 + 7i32 + 8i32 & !(!(0i32 as Instruction) << 1i32) << 0i32) as i32 ^ 1i32)
            as Instruction)
            << 0i32 + 7i32 + 8i32
            & !(!(0i32 as Instruction) << 1i32) << 0i32 + 7i32 + 8i32;
}
unsafe extern "C" fn jumponcond(mut fs: *mut FuncState, mut e: *mut expdesc, mut cond: i32) -> i32 {
    if (*e).k as u32 == VRELOC as i32 as u32 {
        let mut ie: Instruction = *((*(*fs).f).code).offset((*e).u.info as isize);
        if (ie >> 0i32 & !(!(0i32 as Instruction) << 7i32) << 0i32) as OpCode as u32
            == OP_NOT as i32 as u32
        {
            removelastinstruction(fs);
            return condjump(
                fs,
                OP_TEST,
                (ie >> 0i32 + 7i32 + 8i32 + 1i32 & !(!(0i32 as Instruction) << 8i32) << 0i32)
                    as i32,
                0i32,
                0i32,
                (cond == 0) as i32,
            );
        }
    }
    discharge2anyreg(fs, e);
    freeexp(fs, e);
    return condjump(
        fs,
        OP_TESTSET,
        ((1i32) << 8i32) - 1i32,
        (*e).u.info,
        0i32,
        cond,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaK_goiftrue(mut fs: *mut FuncState, mut e: *mut expdesc) {
    let mut pc: i32 = 0;
    luaK_dischargevars(fs, e);
    match (*e).k as u32 {
        16 => {
            negatecondition(fs, e);
            pc = (*e).u.info;
        }
        4 | 5 | 6 | 7 | 2 => {
            pc = -(1i32);
        }
        _ => {
            pc = jumponcond(fs, e, 0i32);
        }
    }
    luaK_concat(fs, &mut (*e).f, pc);
    luaK_patchtohere(fs, (*e).t);
    (*e).t = -(1i32);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaK_goiffalse(mut fs: *mut FuncState, mut e: *mut expdesc) {
    let mut pc: i32 = 0;
    luaK_dischargevars(fs, e);
    match (*e).k as u32 {
        16 => {
            pc = (*e).u.info;
        }
        1 | 3 => {
            pc = -(1i32);
        }
        _ => {
            pc = jumponcond(fs, e, 1i32);
        }
    }
    luaK_concat(fs, &mut (*e).t, pc);
    luaK_patchtohere(fs, (*e).f);
    (*e).f = -(1i32);
}
unsafe extern "C" fn codenot(mut fs: *mut FuncState, mut e: *mut expdesc) {
    match (*e).k as u32 {
        1 | 3 => {
            (*e).k = VTRUE;
        }
        4 | 5 | 6 | 7 | 2 => {
            (*e).k = VFALSE;
        }
        16 => {
            negatecondition(fs, e);
        }
        17 | 8 => {
            discharge2anyreg(fs, e);
            freeexp(fs, e);
            (*e).u.info = luaK_codeABCk(fs, OP_NOT, 0i32, (*e).u.info, 0i32, 0i32);
            (*e).k = VRELOC;
        }
        _ => {}
    }
    let mut temp: i32 = (*e).f;
    (*e).f = (*e).t;
    (*e).t = temp;
    removevalues(fs, (*e).f);
    removevalues(fs, (*e).t);
}
unsafe extern "C" fn isKstr(mut fs: *mut FuncState, mut e: *mut expdesc) -> i32 {
    return ((*e).k as u32 == VK as i32 as u32
        && !((*e).t != (*e).f)
        && (*e).u.info <= ((1i32) << 8i32) - 1i32
        && (*((*(*fs).f).k).offset((*e).u.info as isize)).tt_ as i32
            == 4i32 | (0i32) << 4i32 | (1i32) << 6i32) as i32;
}
unsafe extern "C" fn isKint(mut e: *mut expdesc) -> i32 {
    return ((*e).k as u32 == VKINT as i32 as u32 && !((*e).t != (*e).f)) as i32;
}
unsafe extern "C" fn isCint(mut e: *mut expdesc) -> i32 {
    return (isKint(e) != 0 && (*e).u.ival as u64 <= (((1i32) << 8i32) - 1i32) as u64) as i32;
}
unsafe extern "C" fn isSCint(mut e: *mut expdesc) -> i32 {
    return (isKint(e) != 0 && fitsC((*e).u.ival) != 0) as i32;
}
unsafe extern "C" fn isSCnumber(
    mut e: *mut expdesc,
    mut pi: *mut i32,
    mut isfloat: *mut i32,
) -> i32 {
    let mut i: i64 = 0;
    if (*e).k as u32 == VKINT as i32 as u32 {
        i = (*e).u.ival;
    } else if (*e).k as u32 == VKFLT as i32 as u32
        && luaV_flttointeger((*e).u.nval, &mut i, F2Ieq) != 0
    {
        *isfloat = 1i32;
    } else {
        return 0i32;
    }
    if !((*e).t != (*e).f) && fitsC(i) != 0 {
        *pi = i as i32 + (((1i32) << 8i32) - 1i32 >> 1i32);
        return 1i32;
    } else {
        return 0i32;
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaK_indexed(
    mut fs: *mut FuncState,
    mut t: *mut expdesc,
    mut k: *mut expdesc,
) {
    if (*k).k as u32 == VKSTR as i32 as u32 {
        str2K(fs, k);
    }
    if (*t).k as u32 == VUPVAL as i32 as u32 && isKstr(fs, k) == 0 {
        luaK_exp2anyreg(fs, t);
    }
    if (*t).k as u32 == VUPVAL as i32 as u32 {
        let mut temp: i32 = (*t).u.info;
        (*t).u.ind.t = temp as u8;
        (*t).u.ind.index = (*k).u.info as libc::c_short;
        (*t).k = VINDEXUP;
    } else {
        (*t).u.ind.t = (if (*t).k as u32 == VLOCAL as i32 as u32 {
            (*t).u.var.ridx as i32
        } else {
            (*t).u.info
        }) as u8;
        if isKstr(fs, k) != 0 {
            (*t).u.ind.index = (*k).u.info as libc::c_short;
            (*t).k = VINDEXSTR;
        } else if isCint(k) != 0 {
            (*t).u.ind.index = (*k).u.ival as i32 as libc::c_short;
            (*t).k = VINDEXI;
        } else {
            (*t).u.ind.index = luaK_exp2anyreg(fs, k) as libc::c_short;
            (*t).k = VINDEXED;
        }
    };
}
unsafe extern "C" fn validop(mut op: i32, mut v1: *mut TValue, mut v2: *mut TValue) -> i32 {
    match op {
        7 | 8 | 9 | 10 | 11 | 13 => {
            let mut i: i64 = 0;
            return (luaV_tointegerns(v1, &mut i, F2Ieq) != 0
                && luaV_tointegerns(v2, &mut i, F2Ieq) != 0) as i32;
        }
        5 | 6 | 3 => {
            return ((if (*v2).tt_ as i32 == 3i32 | (0i32) << 4i32 {
                (*v2).value_.i as f64
            } else {
                (*v2).value_.n
            }) != 0i32 as f64) as i32;
        }
        _ => return 1i32,
    };
}
unsafe extern "C" fn constfolding(
    mut fs: *mut FuncState,
    mut op: i32,
    mut e1: *mut expdesc,
    mut e2: *const expdesc,
) -> i32 {
    let mut v1: TValue = TValue {
        value_: Value {
            gc: 0 as *mut GCObject,
        },
        tt_: 0,
    };
    let mut v2: TValue = TValue {
        value_: Value {
            gc: 0 as *mut GCObject,
        },
        tt_: 0,
    };
    let mut res: TValue = TValue {
        value_: Value {
            gc: 0 as *mut GCObject,
        },
        tt_: 0,
    };
    if tonumeral(e1, &mut v1) == 0
        || tonumeral(e2, &mut v2) == 0
        || validop(op, &mut v1, &mut v2) == 0
    {
        return 0i32;
    }
    luaO_rawarith((*(*fs).ls).L, op, &mut v1, &mut v2, &mut res);
    if res.tt_ as i32 == 3i32 | (0i32) << 4i32 {
        (*e1).k = VKINT;
        (*e1).u.ival = res.value_.i;
    } else {
        let mut n: f64 = res.value_.n;
        if !(n == n) || n == 0i32 as f64 {
            return 0i32;
        }
        (*e1).k = VKFLT;
        (*e1).u.nval = n;
    }
    return 1i32;
}
#[inline]
unsafe extern "C" fn binopr2op(mut opr: BinOpr, mut baser: BinOpr, mut base: OpCode) -> OpCode {
    return (opr as i32 - baser as i32 + base as i32) as OpCode;
}
#[inline]
unsafe extern "C" fn unopr2op(mut opr: UnOpr) -> OpCode {
    return (opr as i32 - OPR_MINUS as i32 + OP_UNM as i32) as OpCode;
}
#[inline]
unsafe extern "C" fn binopr2TM(mut opr: BinOpr) -> TMS {
    return (opr as i32 - OPR_ADD as i32 + TM_ADD as i32) as TMS;
}
unsafe extern "C" fn codeunexpval(
    mut fs: *mut FuncState,
    mut op: OpCode,
    mut e: *mut expdesc,
    mut line: i32,
) {
    let mut r: i32 = luaK_exp2anyreg(fs, e);
    freeexp(fs, e);
    (*e).u.info = luaK_codeABCk(fs, op, 0i32, r, 0i32, 0i32);
    (*e).k = VRELOC;
    luaK_fixline(fs, line);
}
unsafe extern "C" fn finishbinexpval(
    mut fs: *mut FuncState,
    mut e1: *mut expdesc,
    mut e2: *mut expdesc,
    mut op: OpCode,
    mut v2: i32,
    mut flip: i32,
    mut line: i32,
    mut mmop: OpCode,
    mut event: TMS,
) {
    let mut v1: i32 = luaK_exp2anyreg(fs, e1);
    let mut pc: i32 = luaK_codeABCk(fs, op, 0i32, v1, v2, 0i32);
    freeexps(fs, e1, e2);
    (*e1).u.info = pc;
    (*e1).k = VRELOC;
    luaK_fixline(fs, line);
    luaK_codeABCk(fs, mmop, v1, v2, event as i32, flip);
    luaK_fixline(fs, line);
}
unsafe extern "C" fn codebinexpval(
    mut fs: *mut FuncState,
    mut opr: BinOpr,
    mut e1: *mut expdesc,
    mut e2: *mut expdesc,
    mut line: i32,
) {
    let mut op: OpCode = binopr2op(opr, OPR_ADD, OP_ADD);
    let mut v2: i32 = luaK_exp2anyreg(fs, e2);
    finishbinexpval(fs, e1, e2, op, v2, 0i32, line, OP_MMBIN, binopr2TM(opr));
}
unsafe extern "C" fn codebini(
    mut fs: *mut FuncState,
    mut op: OpCode,
    mut e1: *mut expdesc,
    mut e2: *mut expdesc,
    mut flip: i32,
    mut line: i32,
    mut event: TMS,
) {
    let mut v2: i32 = (*e2).u.ival as i32 + (((1i32) << 8i32) - 1i32 >> 1i32);
    finishbinexpval(fs, e1, e2, op, v2, flip, line, OP_MMBINI, event);
}
unsafe extern "C" fn codebinK(
    mut fs: *mut FuncState,
    mut opr: BinOpr,
    mut e1: *mut expdesc,
    mut e2: *mut expdesc,
    mut flip: i32,
    mut line: i32,
) {
    let mut event: TMS = binopr2TM(opr);
    let mut v2: i32 = (*e2).u.info;
    let mut op: OpCode = binopr2op(opr, OPR_ADD, OP_ADDK);
    finishbinexpval(fs, e1, e2, op, v2, flip, line, OP_MMBINK, event);
}
unsafe extern "C" fn finishbinexpneg(
    mut fs: *mut FuncState,
    mut e1: *mut expdesc,
    mut e2: *mut expdesc,
    mut op: OpCode,
    mut line: i32,
    mut event: TMS,
) -> i32 {
    if isKint(e2) == 0 {
        return 0i32;
    } else {
        let mut i2: i64 = (*e2).u.ival;
        if !(fitsC(i2) != 0 && fitsC(-i2) != 0) {
            return 0i32;
        } else {
            let mut v2: i32 = i2 as i32;
            finishbinexpval(
                fs,
                e1,
                e2,
                op,
                -v2 + (((1i32) << 8i32) - 1i32 >> 1i32),
                0i32,
                line,
                OP_MMBINI,
                event,
            );
            *((*(*fs).f).code).offset(((*fs).pc - 1i32) as isize) = *((*(*fs).f).code)
                .offset(((*fs).pc - 1i32) as isize)
                & !(!(!(0i32 as Instruction) << 8i32) << 0i32 + 7i32 + 8i32 + 1i32)
                | ((v2 + (((1i32) << 8i32) - 1i32 >> 1i32)) as Instruction)
                    << 0i32 + 7i32 + 8i32 + 1i32
                    & !(!(0i32 as Instruction) << 8i32) << 0i32 + 7i32 + 8i32 + 1i32;
            return 1i32;
        }
    };
}
unsafe extern "C" fn swapexps(mut e1: *mut expdesc, mut e2: *mut expdesc) {
    let mut temp: expdesc = *e1;
    *e1 = *e2;
    *e2 = temp;
}
unsafe extern "C" fn codebinNoK(
    mut fs: *mut FuncState,
    mut opr: BinOpr,
    mut e1: *mut expdesc,
    mut e2: *mut expdesc,
    mut flip: i32,
    mut line: i32,
) {
    if flip != 0 {
        swapexps(e1, e2);
    }
    codebinexpval(fs, opr, e1, e2, line);
}
unsafe extern "C" fn codearith(
    mut fs: *mut FuncState,
    mut opr: BinOpr,
    mut e1: *mut expdesc,
    mut e2: *mut expdesc,
    mut flip: i32,
    mut line: i32,
) {
    if tonumeral(e2, 0 as *mut TValue) != 0 && luaK_exp2K(fs, e2) != 0 {
        codebinK(fs, opr, e1, e2, flip, line);
    } else {
        codebinNoK(fs, opr, e1, e2, flip, line);
    };
}
unsafe extern "C" fn codecommutative(
    mut fs: *mut FuncState,
    mut op: BinOpr,
    mut e1: *mut expdesc,
    mut e2: *mut expdesc,
    mut line: i32,
) {
    let mut flip: i32 = 0i32;
    if tonumeral(e1, 0 as *mut TValue) != 0 {
        swapexps(e1, e2);
        flip = 1i32;
    }
    if op as u32 == OPR_ADD as i32 as u32 && isSCint(e2) != 0 {
        codebini(fs, OP_ADDI, e1, e2, flip, line, TM_ADD);
    } else {
        codearith(fs, op, e1, e2, flip, line);
    };
}
unsafe extern "C" fn codebitwise(
    mut fs: *mut FuncState,
    mut opr: BinOpr,
    mut e1: *mut expdesc,
    mut e2: *mut expdesc,
    mut line: i32,
) {
    let mut flip: i32 = 0i32;
    if (*e1).k as u32 == VKINT as i32 as u32 {
        swapexps(e1, e2);
        flip = 1i32;
    }
    if (*e2).k as u32 == VKINT as i32 as u32 && luaK_exp2K(fs, e2) != 0 {
        codebinK(fs, opr, e1, e2, flip, line);
    } else {
        codebinNoK(fs, opr, e1, e2, flip, line);
    };
}
unsafe extern "C" fn codeorder(
    mut fs: *mut FuncState,
    mut opr: BinOpr,
    mut e1: *mut expdesc,
    mut e2: *mut expdesc,
) {
    let mut r1: i32 = 0;
    let mut r2: i32 = 0;
    let mut im: i32 = 0;
    let mut isfloat: i32 = 0i32;
    let mut op: OpCode = OP_MOVE;
    if isSCnumber(e2, &mut im, &mut isfloat) != 0 {
        r1 = luaK_exp2anyreg(fs, e1);
        r2 = im;
        op = binopr2op(opr, OPR_LT, OP_LTI);
    } else if isSCnumber(e1, &mut im, &mut isfloat) != 0 {
        r1 = luaK_exp2anyreg(fs, e2);
        r2 = im;
        op = binopr2op(opr, OPR_LT, OP_GTI);
    } else {
        r1 = luaK_exp2anyreg(fs, e1);
        r2 = luaK_exp2anyreg(fs, e2);
        op = binopr2op(opr, OPR_LT, OP_LT);
    }
    freeexps(fs, e1, e2);
    (*e1).u.info = condjump(fs, op, r1, r2, isfloat, 1i32);
    (*e1).k = VJMP;
}
unsafe extern "C" fn codeeq(
    mut fs: *mut FuncState,
    mut opr: BinOpr,
    mut e1: *mut expdesc,
    mut e2: *mut expdesc,
) {
    let mut r1: i32 = 0;
    let mut r2: i32 = 0;
    let mut im: i32 = 0;
    let mut isfloat: i32 = 0i32;
    let mut op: OpCode = OP_MOVE;
    if (*e1).k as u32 != VNONRELOC as i32 as u32 {
        swapexps(e1, e2);
    }
    r1 = luaK_exp2anyreg(fs, e1);
    if isSCnumber(e2, &mut im, &mut isfloat) != 0 {
        op = OP_EQI;
        r2 = im;
    } else if exp2RK(fs, e2) != 0 {
        op = OP_EQK;
        r2 = (*e2).u.info;
    } else {
        op = OP_EQ;
        r2 = luaK_exp2anyreg(fs, e2);
    }
    freeexps(fs, e1, e2);
    (*e1).u.info = condjump(
        fs,
        op,
        r1,
        r2,
        isfloat,
        (opr as u32 == OPR_EQ as i32 as u32) as i32,
    );
    (*e1).k = VJMP;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaK_prefix(
    mut fs: *mut FuncState,
    mut opr: UnOpr,
    mut e: *mut expdesc,
    mut line: i32,
) {
    static mut ef: expdesc = {
        let mut init = expdesc {
            k: VKINT,
            u: C2RustUnnamed_11 { ival: 0i32 as i64 },
            t: -(1i32),
            f: -(1i32),
        };
        init
    };
    luaK_dischargevars(fs, e);
    let mut current_block_3: u64;
    match opr as u32 {
        0 | 1 => {
            if constfolding(fs, (opr as u32).wrapping_add(12i32 as u32) as i32, e, &ef) != 0 {
                current_block_3 = 7815301370352969686;
            } else {
                current_block_3 = 18077247625661940693;
            }
        }
        3 => {
            current_block_3 = 18077247625661940693;
        }
        2 => {
            codenot(fs, e);
            current_block_3 = 7815301370352969686;
        }
        _ => {
            current_block_3 = 7815301370352969686;
        }
    }
    match current_block_3 {
        18077247625661940693 => {
            codeunexpval(fs, unopr2op(opr), e, line);
        }
        _ => {}
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaK_infix(mut fs: *mut FuncState, mut op: BinOpr, mut v: *mut expdesc) {
    luaK_dischargevars(fs, v);
    match op as u32 {
        19 => {
            luaK_goiftrue(fs, v);
        }
        20 => {
            luaK_goiffalse(fs, v);
        }
        12 => {
            luaK_exp2nextreg(fs, v);
        }
        0 | 1 | 2 | 5 | 6 | 3 | 4 | 7 | 8 | 9 | 10 | 11 => {
            if tonumeral(v, 0 as *mut TValue) == 0 {
                luaK_exp2anyreg(fs, v);
            }
        }
        13 | 16 => {
            if tonumeral(v, 0 as *mut TValue) == 0 {
                exp2RK(fs, v);
            }
        }
        14 | 15 | 17 | 18 => {
            let mut dummy: i32 = 0;
            let mut dummy2: i32 = 0;
            if isSCnumber(v, &mut dummy, &mut dummy2) == 0 {
                luaK_exp2anyreg(fs, v);
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn codeconcat(
    mut fs: *mut FuncState,
    mut e1: *mut expdesc,
    mut e2: *mut expdesc,
    mut line: i32,
) {
    let mut ie2: *mut Instruction = previousinstruction(fs);
    if (*ie2 >> 0i32 & !(!(0i32 as Instruction) << 7i32) << 0i32) as OpCode as u32
        == OP_CONCAT as i32 as u32
    {
        let mut n: i32 =
            (*ie2 >> 0i32 + 7i32 + 8i32 + 1i32 & !(!(0i32 as Instruction) << 8i32) << 0i32) as i32;
        freeexp(fs, e2);
        *ie2 = *ie2 & !(!(!(0i32 as Instruction) << 8i32) << 0i32 + 7i32)
            | ((*e1).u.info as Instruction) << 0i32 + 7i32
                & !(!(0i32 as Instruction) << 8i32) << 0i32 + 7i32;
        *ie2 = *ie2 & !(!(!(0i32 as Instruction) << 8i32) << 0i32 + 7i32 + 8i32 + 1i32)
            | ((n + 1i32) as Instruction) << 0i32 + 7i32 + 8i32 + 1i32
                & !(!(0i32 as Instruction) << 8i32) << 0i32 + 7i32 + 8i32 + 1i32;
    } else {
        luaK_codeABCk(fs, OP_CONCAT, (*e1).u.info, 2i32, 0i32, 0i32);
        freeexp(fs, e2);
        luaK_fixline(fs, line);
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaK_posfix(
    mut fs: *mut FuncState,
    mut opr: BinOpr,
    mut e1: *mut expdesc,
    mut e2: *mut expdesc,
    mut line: i32,
) {
    luaK_dischargevars(fs, e2);
    if opr as u32 <= OPR_SHR as i32 as u32
        && constfolding(fs, (opr as u32).wrapping_add(0i32 as u32) as i32, e1, e2) != 0
    {
        return;
    }
    let mut current_block_30: u64;
    match opr as u32 {
        19 => {
            luaK_concat(fs, &mut (*e2).f, (*e1).f);
            *e1 = *e2;
            current_block_30 = 8180496224585318153;
        }
        20 => {
            luaK_concat(fs, &mut (*e2).t, (*e1).t);
            *e1 = *e2;
            current_block_30 = 8180496224585318153;
        }
        12 => {
            luaK_exp2nextreg(fs, e2);
            codeconcat(fs, e1, e2, line);
            current_block_30 = 8180496224585318153;
        }
        0 | 2 => {
            codecommutative(fs, opr, e1, e2, line);
            current_block_30 = 8180496224585318153;
        }
        1 => {
            if finishbinexpneg(fs, e1, e2, OP_ADDI, line, TM_SUB) != 0 {
                current_block_30 = 8180496224585318153;
            } else {
                current_block_30 = 12599329904712511516;
            }
        }
        5 | 6 | 3 | 4 => {
            current_block_30 = 12599329904712511516;
        }
        7 | 8 | 9 => {
            codebitwise(fs, opr, e1, e2, line);
            current_block_30 = 8180496224585318153;
        }
        10 => {
            if isSCint(e1) != 0 {
                swapexps(e1, e2);
                codebini(fs, OP_SHLI, e1, e2, 1i32, line, TM_SHL);
            } else if !(finishbinexpneg(fs, e1, e2, OP_SHRI, line, TM_SHL) != 0) {
                codebinexpval(fs, opr, e1, e2, line);
            }
            current_block_30 = 8180496224585318153;
        }
        11 => {
            if isSCint(e2) != 0 {
                codebini(fs, OP_SHRI, e1, e2, 0i32, line, TM_SHR);
            } else {
                codebinexpval(fs, opr, e1, e2, line);
            }
            current_block_30 = 8180496224585318153;
        }
        13 | 16 => {
            codeeq(fs, opr, e1, e2);
            current_block_30 = 8180496224585318153;
        }
        17 | 18 => {
            swapexps(e1, e2);
            opr = (opr as u32)
                .wrapping_sub(OPR_GT as i32 as u32)
                .wrapping_add(OPR_LT as i32 as u32) as BinOpr;
            current_block_30 = 1118134448028020070;
        }
        14 | 15 => {
            current_block_30 = 1118134448028020070;
        }
        _ => {
            current_block_30 = 8180496224585318153;
        }
    }
    match current_block_30 {
        12599329904712511516 => {
            codearith(fs, opr, e1, e2, 0i32, line);
        }
        1118134448028020070 => {
            codeorder(fs, opr, e1, e2);
        }
        _ => {}
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaK_fixline(mut fs: *mut FuncState, mut line: i32) {
    removelastlineinfo(fs);
    savelineinfo(fs, (*fs).f, line);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaK_settablesize(
    mut fs: *mut FuncState,
    mut pc: i32,
    mut ra: i32,
    mut asize: i32,
    mut hsize: i32,
) {
    let mut inst: *mut Instruction =
        &mut *((*(*fs).f).code).offset(pc as isize) as *mut Instruction;
    let mut rb: i32 = if hsize != 0i32 {
        luaO_ceillog2(hsize as u32) + 1i32
    } else {
        0i32
    };
    let mut extra: i32 = asize / (((1i32) << 8i32) - 1i32 + 1i32);
    let mut rc: i32 = asize % (((1i32) << 8i32) - 1i32 + 1i32);
    let mut k: i32 = (extra > 0i32) as i32;
    *inst = (OP_NEWTABLE as i32 as Instruction) << 0i32
        | (ra as Instruction) << 0i32 + 7i32
        | (rb as Instruction) << 0i32 + 7i32 + 8i32 + 1i32
        | (rc as Instruction) << 0i32 + 7i32 + 8i32 + 1i32 + 8i32
        | (k as Instruction) << 0i32 + 7i32 + 8i32;
    *inst.offset(1i32 as isize) =
        (OP_EXTRAARG as i32 as Instruction) << 0i32 | (extra as Instruction) << 0i32 + 7i32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaK_setlist(
    mut fs: *mut FuncState,
    mut base: i32,
    mut nelems: i32,
    mut tostore: i32,
) {
    if tostore == -(1i32) {
        tostore = 0i32;
    }
    if nelems <= ((1i32) << 8i32) - 1i32 {
        luaK_codeABCk(fs, OP_SETLIST, base, tostore, nelems, 0i32);
    } else {
        let mut extra: i32 = nelems / (((1i32) << 8i32) - 1i32 + 1i32);
        nelems %= ((1i32) << 8i32) - 1i32 + 1i32;
        luaK_codeABCk(fs, OP_SETLIST, base, tostore, nelems, 1i32);
        codeextraarg(fs, extra);
    }
    (*fs).freereg = (base + 1i32) as u8;
}
unsafe extern "C" fn finaltarget(mut code: *mut Instruction, mut i: i32) -> i32 {
    let mut count: i32 = 0;
    count = 0i32;
    while count < 100i32 {
        let mut pc: Instruction = *code.offset(i as isize);
        if (pc >> 0i32 & !(!(0i32 as Instruction) << 7i32) << 0i32) as OpCode as u32
            != OP_JMP as i32 as u32
        {
            break;
        }
        i += (pc >> 0i32 + 7i32 & !(!(0i32 as Instruction) << 8i32 + 8i32 + 1i32 + 8i32) << 0i32)
            as i32
            - (((1i32) << 8i32 + 8i32 + 1i32 + 8i32) - 1i32 >> 1i32)
            + 1i32;
        count += 1;
    }
    return i;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaK_finish(mut fs: *mut FuncState) {
    let mut i: i32 = 0;
    let mut p: *mut Proto = (*fs).f;
    i = 0i32;
    while i < (*fs).pc {
        let mut pc: *mut Instruction = &mut *((*p).code).offset(i as isize) as *mut Instruction;
        let mut current_block_7: u64;
        match (*pc >> 0i32 & !(!(0i32 as Instruction) << 7i32) << 0i32) as OpCode as u32 {
            71 | 72 => {
                if !((*fs).needclose as i32 != 0 || (*p).is_vararg as i32 != 0) {
                    current_block_7 = 12599329904712511516;
                } else {
                    *pc = *pc & !(!(!(0i32 as Instruction) << 7i32) << 0i32)
                        | (OP_RETURN as i32 as Instruction) << 0i32
                            & !(!(0i32 as Instruction) << 7i32) << 0i32;
                    current_block_7 = 11006700562992250127;
                }
            }
            70 | 69 => {
                current_block_7 = 11006700562992250127;
            }
            56 => {
                let mut target: i32 = finaltarget((*p).code, i);
                fixjump(fs, i, target);
                current_block_7 = 12599329904712511516;
            }
            _ => {
                current_block_7 = 12599329904712511516;
            }
        }
        match current_block_7 {
            11006700562992250127 => {
                if (*fs).needclose != 0 {
                    *pc = *pc & !(!(!(0i32 as Instruction) << 1i32) << 0i32 + 7i32 + 8i32)
                        | (1i32 as Instruction) << 0i32 + 7i32 + 8i32
                            & !(!(0i32 as Instruction) << 1i32) << 0i32 + 7i32 + 8i32;
                }
                if (*p).is_vararg != 0 {
                    *pc = *pc
                        & !(!(!(0i32 as Instruction) << 8i32) << 0i32 + 7i32 + 8i32 + 1i32 + 8i32)
                        | (((*p).numparams as i32 + 1i32) as Instruction)
                            << 0i32 + 7i32 + 8i32 + 1i32 + 8i32
                            & !(!(0i32 as Instruction) << 8i32) << 0i32 + 7i32 + 8i32 + 1i32 + 8i32;
                }
            }
            _ => {}
        }
        i += 1;
    }
}
