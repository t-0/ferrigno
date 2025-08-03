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
    pub type BlockCnt;
    fn ldexp(_: f64, _: libc::c_int) -> f64;
    fn abs(_: libc::c_int) -> libc::c_int;
    fn luaO_ceillog2(x: libc::c_uint) -> libc::c_int;
    fn luaO_rawarith(
        L: *mut lua_State,
        op: libc::c_int,
        p1: *const TValue,
        p2: *const TValue,
        res: *mut TValue,
    ) -> libc::c_int;
    fn luaM_growaux_(
        L: *mut lua_State,
        block: *mut libc::c_void,
        nelems: libc::c_int,
        size: *mut libc::c_int,
        size_elem: libc::c_int,
        limit: libc::c_int,
        what: *const libc::c_char,
    ) -> *mut libc::c_void;
    fn luaX_syntaxerror(ls: *mut LexState, s: *const libc::c_char) -> !;
    fn luaY_nvarstack(fs: *mut FuncState) -> libc::c_int;
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
    fn luaV_equalobj(
        L: *mut lua_State,
        t1: *const TValue,
        t2: *const TValue,
    ) -> libc::c_int;
    fn luaV_tointegerns(
        obj: *const TValue,
        p: *mut lua_Integer,
        mode: F2Imod,
    ) -> libc::c_int;
    fn luaV_flttointeger(
        n: lua_Number,
        p: *mut lua_Integer,
        mode: F2Imod,
    ) -> libc::c_int;
}
pub type __sig_atomic_t = libc::c_int;
pub type size_t = libc::c_ulong;
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
    pub tt_: u8,
    pub delta: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union Value {
    pub gc: *mut GCObject,
    pub p: *mut libc::c_void,
    pub f: lua_CFunction,
    pub i: lua_Integer,
    pub n: lua_Number,
    pub ub: u8,
}
pub type lua_Number = f64;
pub type lua_Integer = i64;
pub type lua_CFunction = Option::<unsafe extern "C" fn(*mut lua_State) -> libc::c_int>;
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
pub type lua_Reader = Option::<
    unsafe extern "C" fn(
        *mut lua_State,
        *mut libc::c_void,
        *mut size_t,
    ) -> *const libc::c_char,
>;
pub type ls_byte = libc::c_schar;
#[derive(Copy, Clone)]
#[repr(C)]
pub union UValue {
    pub uv: TValue,
    pub n: lua_Number,
    pub u: f64,
    pub s: *mut libc::c_void,
    pub i: lua_Integer,
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
    pub tt: u8,
    pub marked: u8,
    pub numparams: u8,
    pub is_vararg: u8,
    pub maxstacksize: u8,
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
    pub tt: u8,
    pub marked: u8,
    pub nupvalues: u8,
    pub gclist: *mut GCObject,
    pub f: lua_CFunction,
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
pub struct Zio {
    pub n: size_t,
    pub p: *const libc::c_char,
    pub reader: lua_Reader,
    pub data: *mut libc::c_void,
    pub L: *mut lua_State,
}
pub type ZIO = Zio;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Mbuffer {
    pub buffer: *mut libc::c_char,
    pub n: size_t,
    pub buffsize: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union SemInfo {
    pub r: lua_Number,
    pub i: lua_Integer,
    pub ts: *mut TString,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Token {
    pub token: libc::c_int,
    pub seminfo: SemInfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LexState {
    pub current: libc::c_int,
    pub linenumber: libc::c_int,
    pub lastline: libc::c_int,
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
    pub n: libc::c_int,
    pub size: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Labeldesc {
    pub name: *mut TString,
    pub pc: libc::c_int,
    pub line: libc::c_int,
    pub nactvar: u8,
    pub close: u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub arr: *mut Vardesc,
    pub n: libc::c_int,
    pub size: libc::c_int,
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
    pub pc: libc::c_int,
    pub lasttarget: libc::c_int,
    pub previousline: libc::c_int,
    pub nk: libc::c_int,
    pub np: libc::c_int,
    pub nabslineinfo: libc::c_int,
    pub firstlocal: libc::c_int,
    pub firstlabel: libc::c_int,
    pub ndebugvars: libc::c_short,
    pub nactvar: u8,
    pub nups: u8,
    pub freereg: u8,
    pub iwthabs: u8,
    pub needclose: u8,
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
pub type expkind = libc::c_uint;
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
    pub t: libc::c_int,
    pub f: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
    pub ival: lua_Integer,
    pub nval: lua_Number,
    pub strval: *mut TString,
    pub info: libc::c_int,
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
pub type BinOpr = libc::c_uint;
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
pub type UnOpr = libc::c_uint;
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
pub type F2Imod = libc::c_uint;
pub const F2Iceil: F2Imod = 2;
pub const F2Ifloor: F2Imod = 1;
pub const F2Ieq: F2Imod = 0;
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
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaK_semerror(
    mut ls: *mut LexState,
    mut msg: *const libc::c_char,
) -> ! {
    (*ls).t.token = 0 as libc::c_int;
    luaX_syntaxerror(ls, msg);
}
unsafe extern "C" fn tonumeral(
    mut e: *const expdesc,
    mut v: *mut TValue,
) -> libc::c_int {
    if (*e).t != (*e).f {
        return 0 as libc::c_int;
    }
    match (*e).k as libc::c_uint {
        6 => {
            if !v.is_null() {
                let mut io: *mut TValue = v;
                (*io).value_.i = (*e).u.ival;
                (*io)
                    .tt_ = (3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int)
                    as u8;
            }
            return 1 as libc::c_int;
        }
        5 => {
            if !v.is_null() {
                let mut io_0: *mut TValue = v;
                (*io_0).value_.n = (*e).u.nval;
                (*io_0)
                    .tt_ = (3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int)
                    as u8;
            }
            return 1 as libc::c_int;
        }
        _ => return 0 as libc::c_int,
    };
}
unsafe extern "C" fn const2val(
    mut fs: *mut FuncState,
    mut e: *const expdesc,
) -> *mut TValue {
    return &mut (*((*(*(*fs).ls).dyd).actvar.arr).offset((*e).u.info as isize)).k;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaK_exp2const(
    mut fs: *mut FuncState,
    mut e: *const expdesc,
    mut v: *mut TValue,
) -> libc::c_int {
    if (*e).t != (*e).f {
        return 0 as libc::c_int;
    }
    match (*e).k as libc::c_uint {
        3 => {
            (*v)
                .tt_ = (1 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int)
                as u8;
            return 1 as libc::c_int;
        }
        2 => {
            (*v)
                .tt_ = (1 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int)
                as u8;
            return 1 as libc::c_int;
        }
        1 => {
            (*v)
                .tt_ = (0 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int)
                as u8;
            return 1 as libc::c_int;
        }
        7 => {
            let mut io: *mut TValue = v;
            let mut x_: *mut TString = (*e).u.strval;
            (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
            (*io)
                .tt_ = ((*x_).tt as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int)
                as u8;
            return 1 as libc::c_int;
        }
        11 => {
            let mut io1: *mut TValue = v;
            let mut io2: *const TValue = const2val(fs, e);
            (*io1).value_ = (*io2).value_;
            (*io1).tt_ = (*io2).tt_;
            return 1 as libc::c_int;
        }
        _ => return tonumeral(e, v),
    };
}
unsafe extern "C" fn previousinstruction(mut fs: *mut FuncState) -> *mut Instruction {
    static mut invalidinstruction: Instruction = !(0 as libc::c_int as Instruction);
    if (*fs).pc > (*fs).lasttarget {
        return &mut *((*(*fs).f).code).offset(((*fs).pc - 1 as libc::c_int) as isize)
            as *mut Instruction
    } else {
        return &invalidinstruction as *const Instruction as *mut Instruction
    };
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaK_nil(
    mut fs: *mut FuncState,
    mut from: libc::c_int,
    mut n: libc::c_int,
) {
    let mut l: libc::c_int = from + n - 1 as libc::c_int;
    let mut previous: *mut Instruction = previousinstruction(fs);
    if (*previous >> 0 as libc::c_int
        & !(!(0 as libc::c_int as Instruction) << 7 as libc::c_int) << 0 as libc::c_int)
        as OpCode as libc::c_uint == OP_LOADNIL as libc::c_int as libc::c_uint
    {
        let mut pfrom: libc::c_int = (*previous >> 0 as libc::c_int + 7 as libc::c_int
            & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                << 0 as libc::c_int) as libc::c_int;
        let mut pl: libc::c_int = pfrom
            + (*previous
                >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                    + 1 as libc::c_int
                & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                    << 0 as libc::c_int) as libc::c_int;
        if pfrom <= from && from <= pl + 1 as libc::c_int
            || from <= pfrom && pfrom <= l + 1 as libc::c_int
        {
            if pfrom < from {
                from = pfrom;
            }
            if pl > l {
                l = pl;
            }
            *previous = *previous
                & !(!(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                    << 0 as libc::c_int + 7 as libc::c_int)
                | (from as Instruction) << 0 as libc::c_int + 7 as libc::c_int
                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                        << 0 as libc::c_int + 7 as libc::c_int;
            *previous = *previous
                & !(!(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                    << 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                        + 1 as libc::c_int)
                | ((l - from) as Instruction)
                    << 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                        + 1 as libc::c_int
                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                        << 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                            + 1 as libc::c_int;
            return;
        }
    }
    luaK_codeABCk(
        fs,
        OP_LOADNIL,
        from,
        n - 1 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    );
}
unsafe extern "C" fn getjump(
    mut fs: *mut FuncState,
    mut pc: libc::c_int,
) -> libc::c_int {
    let mut offset: libc::c_int = (*((*(*fs).f).code).offset(pc as isize)
        >> 0 as libc::c_int + 7 as libc::c_int
        & !(!(0 as libc::c_int as Instruction)
            << 8 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int + 8 as libc::c_int)
            << 0 as libc::c_int) as libc::c_int
        - (((1 as libc::c_int)
            << 8 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int + 8 as libc::c_int)
            - 1 as libc::c_int >> 1 as libc::c_int);
    if offset == -(1 as libc::c_int) {
        return -(1 as libc::c_int)
    } else {
        return pc + 1 as libc::c_int + offset
    };
}
unsafe extern "C" fn fixjump(
    mut fs: *mut FuncState,
    mut pc: libc::c_int,
    mut dest: libc::c_int,
) {
    let mut jmp: *mut Instruction = &mut *((*(*fs).f).code).offset(pc as isize)
        as *mut Instruction;
    let mut offset: libc::c_int = dest - (pc + 1 as libc::c_int);
    if !(-(((1 as libc::c_int)
        << 8 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int + 8 as libc::c_int)
        - 1 as libc::c_int >> 1 as libc::c_int) <= offset
        && offset
            <= ((1 as libc::c_int)
                << 8 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int
                    + 8 as libc::c_int) - 1 as libc::c_int
                - (((1 as libc::c_int)
                    << 8 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int
                        + 8 as libc::c_int) - 1 as libc::c_int >> 1 as libc::c_int))
    {
        luaX_syntaxerror(
            (*fs).ls,
            b"control structure too long\0" as *const u8 as *const libc::c_char,
        );
    }
    *jmp = *jmp
        & !(!(!(0 as libc::c_int as Instruction)
            << 8 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int + 8 as libc::c_int)
            << 0 as libc::c_int + 7 as libc::c_int)
        | ((offset
            + (((1 as libc::c_int)
                << 8 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int
                    + 8 as libc::c_int) - 1 as libc::c_int >> 1 as libc::c_int))
            as libc::c_uint) << 0 as libc::c_int + 7 as libc::c_int
            & !(!(0 as libc::c_int as Instruction)
                << 8 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int
                    + 8 as libc::c_int) << 0 as libc::c_int + 7 as libc::c_int;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaK_concat(
    mut fs: *mut FuncState,
    mut l1: *mut libc::c_int,
    mut l2: libc::c_int,
) {
    if l2 == -(1 as libc::c_int) {
        return
    } else if *l1 == -(1 as libc::c_int) {
        *l1 = l2;
    } else {
        let mut list: libc::c_int = *l1;
        let mut next: libc::c_int = 0;
        loop {
            next = getjump(fs, list);
            if !(next != -(1 as libc::c_int)) {
                break;
            }
            list = next;
        }
        fixjump(fs, list, l2);
    };
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaK_jump(mut fs: *mut FuncState) -> libc::c_int {
    return codesJ(fs, OP_JMP, -(1 as libc::c_int), 0 as libc::c_int);
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaK_ret(
    mut fs: *mut FuncState,
    mut first: libc::c_int,
    mut nret: libc::c_int,
) {
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
    luaK_codeABCk(
        fs,
        op,
        first,
        nret + 1 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    );
}
unsafe extern "C" fn condjump(
    mut fs: *mut FuncState,
    mut op: OpCode,
    mut A: libc::c_int,
    mut B: libc::c_int,
    mut C: libc::c_int,
    mut k: libc::c_int,
) -> libc::c_int {
    luaK_codeABCk(fs, op, A, B, C, k);
    return luaK_jump(fs);
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaK_getlabel(mut fs: *mut FuncState) -> libc::c_int {
    (*fs).lasttarget = (*fs).pc;
    return (*fs).pc;
}
unsafe extern "C" fn getjumpcontrol(
    mut fs: *mut FuncState,
    mut pc: libc::c_int,
) -> *mut Instruction {
    let mut pi: *mut Instruction = &mut *((*(*fs).f).code).offset(pc as isize)
        as *mut Instruction;
    if pc >= 1 as libc::c_int
        && luaP_opmodes[(*pi.offset(-(1 as libc::c_int as isize)) >> 0 as libc::c_int
            & !(!(0 as libc::c_int as Instruction) << 7 as libc::c_int)
                << 0 as libc::c_int) as OpCode as usize] as libc::c_int
            & (1 as libc::c_int) << 4 as libc::c_int != 0
    {
        return pi.offset(-(1 as libc::c_int as isize))
    } else {
        return pi
    };
}
unsafe extern "C" fn patchtestreg(
    mut fs: *mut FuncState,
    mut node: libc::c_int,
    mut reg: libc::c_int,
) -> libc::c_int {
    let mut i: *mut Instruction = getjumpcontrol(fs, node);
    if (*i >> 0 as libc::c_int
        & !(!(0 as libc::c_int as Instruction) << 7 as libc::c_int) << 0 as libc::c_int)
        as OpCode as libc::c_uint != OP_TESTSET as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    if reg != ((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int
        && reg
            != (*i
                >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                    + 1 as libc::c_int
                & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                    << 0 as libc::c_int) as libc::c_int
    {
        *i = *i
            & !(!(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                << 0 as libc::c_int + 7 as libc::c_int)
            | (reg as Instruction) << 0 as libc::c_int + 7 as libc::c_int
                & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                    << 0 as libc::c_int + 7 as libc::c_int;
    } else {
        *i = (OP_TEST as libc::c_int as Instruction) << 0 as libc::c_int
            | ((*i
                >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                    + 1 as libc::c_int
                & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                    << 0 as libc::c_int) as libc::c_int as Instruction)
                << 0 as libc::c_int + 7 as libc::c_int
            | (0 as libc::c_int as Instruction)
                << 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                    + 1 as libc::c_int
            | (0 as libc::c_int as Instruction)
                << 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                    + 1 as libc::c_int + 8 as libc::c_int
            | ((*i >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                & !(!(0 as libc::c_int as Instruction) << 1 as libc::c_int)
                    << 0 as libc::c_int) as libc::c_int as Instruction)
                << 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn removevalues(mut fs: *mut FuncState, mut list: libc::c_int) {
    while list != -(1 as libc::c_int) {
        patchtestreg(
            fs,
            list,
            ((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int,
        );
        list = getjump(fs, list);
    }
}
unsafe extern "C" fn patchlistaux(
    mut fs: *mut FuncState,
    mut list: libc::c_int,
    mut vtarget: libc::c_int,
    mut reg: libc::c_int,
    mut dtarget: libc::c_int,
) {
    while list != -(1 as libc::c_int) {
        let mut next: libc::c_int = getjump(fs, list);
        if patchtestreg(fs, list, reg) != 0 {
            fixjump(fs, list, vtarget);
        } else {
            fixjump(fs, list, dtarget);
        }
        list = next;
    }
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaK_patchlist(
    mut fs: *mut FuncState,
    mut list: libc::c_int,
    mut target: libc::c_int,
) {
    patchlistaux(
        fs,
        list,
        target,
        ((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int,
        target,
    );
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaK_patchtohere(
    mut fs: *mut FuncState,
    mut list: libc::c_int,
) {
    let mut hr: libc::c_int = luaK_getlabel(fs);
    luaK_patchlist(fs, list, hr);
}
unsafe extern "C" fn savelineinfo(
    mut fs: *mut FuncState,
    mut f: *mut Proto,
    mut line: libc::c_int,
) {
    let mut linedif: libc::c_int = line - (*fs).previousline;
    let mut pc: libc::c_int = (*fs).pc - 1 as libc::c_int;
    if abs(linedif) >= 0x80 as libc::c_int
        || {
            let fresh0 = (*fs).iwthabs;
            (*fs).iwthabs = ((*fs).iwthabs).wrapping_add(1);
            fresh0 as libc::c_int >= 128 as libc::c_int
        }
    {
        (*f)
            .abslineinfo = luaM_growaux_(
            (*(*fs).ls).L,
            (*f).abslineinfo as *mut libc::c_void,
            (*fs).nabslineinfo,
            &mut (*f).sizeabslineinfo,
            ::core::mem::size_of::<AbsLineInfo>() as libc::c_ulong as libc::c_int,
            (if 2147483647 as libc::c_int as size_t
                <= (!(0 as libc::c_int as size_t))
                    .wrapping_div(::core::mem::size_of::<AbsLineInfo>() as libc::c_ulong)
            {
                2147483647 as libc::c_int as libc::c_uint
            } else {
                (!(0 as libc::c_int as size_t))
                    .wrapping_div(::core::mem::size_of::<AbsLineInfo>() as libc::c_ulong)
                    as libc::c_uint
            }) as libc::c_int,
            b"lines\0" as *const u8 as *const libc::c_char,
        ) as *mut AbsLineInfo;
        (*((*f).abslineinfo).offset((*fs).nabslineinfo as isize)).pc = pc;
        let fresh1 = (*fs).nabslineinfo;
        (*fs).nabslineinfo = (*fs).nabslineinfo + 1;
        (*((*f).abslineinfo).offset(fresh1 as isize)).line = line;
        linedif = -(0x80 as libc::c_int);
        (*fs).iwthabs = 1 as libc::c_int as u8;
    }
    (*f)
        .lineinfo = luaM_growaux_(
        (*(*fs).ls).L,
        (*f).lineinfo as *mut libc::c_void,
        pc,
        &mut (*f).sizelineinfo,
        ::core::mem::size_of::<ls_byte>() as libc::c_ulong as libc::c_int,
        (if 2147483647 as libc::c_int as size_t
            <= (!(0 as libc::c_int as size_t))
                .wrapping_div(::core::mem::size_of::<ls_byte>() as libc::c_ulong)
        {
            2147483647 as libc::c_int as libc::c_uint
        } else {
            (!(0 as libc::c_int as size_t))
                .wrapping_div(::core::mem::size_of::<ls_byte>() as libc::c_ulong)
                as libc::c_uint
        }) as libc::c_int,
        b"opcodes\0" as *const u8 as *const libc::c_char,
    ) as *mut ls_byte;
    *((*f).lineinfo).offset(pc as isize) = linedif as ls_byte;
    (*fs).previousline = line;
}
unsafe extern "C" fn removelastlineinfo(mut fs: *mut FuncState) {
    let mut f: *mut Proto = (*fs).f;
    let mut pc: libc::c_int = (*fs).pc - 1 as libc::c_int;
    if *((*f).lineinfo).offset(pc as isize) as libc::c_int != -(0x80 as libc::c_int) {
        (*fs).previousline -= *((*f).lineinfo).offset(pc as isize) as libc::c_int;
        (*fs).iwthabs = ((*fs).iwthabs).wrapping_sub(1);
        (*fs).iwthabs;
    } else {
        (*fs).nabslineinfo -= 1;
        (*fs).nabslineinfo;
        (*fs).iwthabs = (128 as libc::c_int + 1 as libc::c_int) as u8;
    };
}
unsafe extern "C" fn removelastinstruction(mut fs: *mut FuncState) {
    removelastlineinfo(fs);
    (*fs).pc -= 1;
    (*fs).pc;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaK_code(
    mut fs: *mut FuncState,
    mut i: Instruction,
) -> libc::c_int {
    let mut f: *mut Proto = (*fs).f;
    (*f)
        .code = luaM_growaux_(
        (*(*fs).ls).L,
        (*f).code as *mut libc::c_void,
        (*fs).pc,
        &mut (*f).sizecode,
        ::core::mem::size_of::<Instruction>() as libc::c_ulong as libc::c_int,
        (if 2147483647 as libc::c_int as size_t
            <= (!(0 as libc::c_int as size_t))
                .wrapping_div(::core::mem::size_of::<Instruction>() as libc::c_ulong)
        {
            2147483647 as libc::c_int as libc::c_uint
        } else {
            (!(0 as libc::c_int as size_t))
                .wrapping_div(::core::mem::size_of::<Instruction>() as libc::c_ulong)
                as libc::c_uint
        }) as libc::c_int,
        b"opcodes\0" as *const u8 as *const libc::c_char,
    ) as *mut Instruction;
    let fresh2 = (*fs).pc;
    (*fs).pc = (*fs).pc + 1;
    *((*f).code).offset(fresh2 as isize) = i;
    savelineinfo(fs, f, (*(*fs).ls).lastline);
    return (*fs).pc - 1 as libc::c_int;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaK_codeABCk(
    mut fs: *mut FuncState,
    mut o: OpCode,
    mut a: libc::c_int,
    mut b: libc::c_int,
    mut c: libc::c_int,
    mut k: libc::c_int,
) -> libc::c_int {
    return luaK_code(
        fs,
        (o as Instruction) << 0 as libc::c_int
            | (a as Instruction) << 0 as libc::c_int + 7 as libc::c_int
            | (b as Instruction)
                << 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                    + 1 as libc::c_int
            | (c as Instruction)
                << 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                    + 1 as libc::c_int + 8 as libc::c_int
            | (k as Instruction)
                << 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int,
    );
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaK_codeABx(
    mut fs: *mut FuncState,
    mut o: OpCode,
    mut a: libc::c_int,
    mut bc: libc::c_uint,
) -> libc::c_int {
    return luaK_code(
        fs,
        (o as Instruction) << 0 as libc::c_int
            | (a as Instruction) << 0 as libc::c_int + 7 as libc::c_int
            | bc << 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int,
    );
}
unsafe extern "C" fn codeAsBx(
    mut fs: *mut FuncState,
    mut o: OpCode,
    mut a: libc::c_int,
    mut bc: libc::c_int,
) -> libc::c_int {
    let mut b: libc::c_uint = (bc
        + (((1 as libc::c_int) << 8 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int)
            - 1 as libc::c_int >> 1 as libc::c_int)) as libc::c_uint;
    return luaK_code(
        fs,
        (o as Instruction) << 0 as libc::c_int
            | (a as Instruction) << 0 as libc::c_int + 7 as libc::c_int
            | b << 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int,
    );
}
unsafe extern "C" fn codesJ(
    mut fs: *mut FuncState,
    mut o: OpCode,
    mut sj: libc::c_int,
    mut k: libc::c_int,
) -> libc::c_int {
    let mut j: libc::c_uint = (sj
        + (((1 as libc::c_int)
            << 8 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int + 8 as libc::c_int)
            - 1 as libc::c_int >> 1 as libc::c_int)) as libc::c_uint;
    return luaK_code(
        fs,
        (o as Instruction) << 0 as libc::c_int | j << 0 as libc::c_int + 7 as libc::c_int
            | (k as Instruction)
                << 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int,
    );
}
unsafe extern "C" fn codeextraarg(
    mut fs: *mut FuncState,
    mut a: libc::c_int,
) -> libc::c_int {
    return luaK_code(
        fs,
        (OP_EXTRAARG as libc::c_int as Instruction) << 0 as libc::c_int
            | (a as Instruction) << 0 as libc::c_int + 7 as libc::c_int,
    );
}
unsafe extern "C" fn luaK_codek(
    mut fs: *mut FuncState,
    mut reg: libc::c_int,
    mut k: libc::c_int,
) -> libc::c_int {
    if k
        <= ((1 as libc::c_int) << 8 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int)
            - 1 as libc::c_int
    {
        return luaK_codeABx(fs, OP_LOADK, reg, k as libc::c_uint)
    } else {
        let mut p: libc::c_int = luaK_codeABx(
            fs,
            OP_LOADKX,
            reg,
            0 as libc::c_int as libc::c_uint,
        );
        codeextraarg(fs, k);
        return p;
    };
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaK_checkstack(mut fs: *mut FuncState, mut n: libc::c_int) {
    let mut newstack: libc::c_int = (*fs).freereg as libc::c_int + n;
    if newstack > (*(*fs).f).maxstacksize as libc::c_int {
        if newstack >= 255 as libc::c_int {
            luaX_syntaxerror(
                (*fs).ls,
                b"function or expression needs too many registers\0" as *const u8
                    as *const libc::c_char,
            );
        }
        (*(*fs).f).maxstacksize = newstack as u8;
    }
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaK_reserveregs(mut fs: *mut FuncState, mut n: libc::c_int) {
    luaK_checkstack(fs, n);
    (*fs).freereg = ((*fs).freereg as libc::c_int + n) as u8;
}
unsafe extern "C" fn freereg(mut fs: *mut FuncState, mut reg: libc::c_int) {
    if reg >= luaY_nvarstack(fs) {
        (*fs).freereg = ((*fs).freereg).wrapping_sub(1);
        (*fs).freereg;
    }
}
unsafe extern "C" fn freeregs(
    mut fs: *mut FuncState,
    mut r1: libc::c_int,
    mut r2: libc::c_int,
) {
    if r1 > r2 {
        freereg(fs, r1);
        freereg(fs, r2);
    } else {
        freereg(fs, r2);
        freereg(fs, r1);
    };
}
unsafe extern "C" fn freeexp(mut fs: *mut FuncState, mut e: *mut expdesc) {
    if (*e).k as libc::c_uint == VNONRELOC as libc::c_int as libc::c_uint {
        freereg(fs, (*e).u.info);
    }
}
unsafe extern "C" fn freeexps(
    mut fs: *mut FuncState,
    mut e1: *mut expdesc,
    mut e2: *mut expdesc,
) {
    let mut r1: libc::c_int = if (*e1).k as libc::c_uint
        == VNONRELOC as libc::c_int as libc::c_uint
    {
        (*e1).u.info
    } else {
        -(1 as libc::c_int)
    };
    let mut r2: libc::c_int = if (*e2).k as libc::c_uint
        == VNONRELOC as libc::c_int as libc::c_uint
    {
        (*e2).u.info
    } else {
        -(1 as libc::c_int)
    };
    freeregs(fs, r1, r2);
}
unsafe extern "C" fn addk(
    mut fs: *mut FuncState,
    mut key: *mut TValue,
    mut v: *mut TValue,
) -> libc::c_int {
    let mut val: TValue = TValue {
        value_: Value { gc: 0 as *mut GCObject },
        tt_: 0,
    };
    let mut L: *mut lua_State = (*(*fs).ls).L;
    let mut f: *mut Proto = (*fs).f;
    let mut index: *const TValue = luaH_get((*(*fs).ls).h, key);
    let mut k: libc::c_int = 0;
    let mut oldsize: libc::c_int = 0;
    if (*index).tt_ as libc::c_int
        == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
    {
        k = (*index).value_.i as libc::c_int;
        if k < (*fs).nk
            && (*((*f).k).offset(k as isize)).tt_ as libc::c_int & 0x3f as libc::c_int
                == (*v).tt_ as libc::c_int & 0x3f as libc::c_int
            && luaV_equalobj(0 as *mut lua_State, &mut *((*f).k).offset(k as isize), v)
                != 0
        {
            return k;
        }
    }
    oldsize = (*f).sizek;
    k = (*fs).nk;
    let mut io: *mut TValue = &mut val;
    (*io).value_.i = k as lua_Integer;
    (*io).tt_ = (3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int) as u8;
    luaH_finishset(L, (*(*fs).ls).h, key, index, &mut val);
    (*f)
        .k = luaM_growaux_(
        L,
        (*f).k as *mut libc::c_void,
        k,
        &mut (*f).sizek,
        ::core::mem::size_of::<TValue>() as libc::c_ulong as libc::c_int,
        (if (((1 as libc::c_int)
            << 8 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int + 8 as libc::c_int)
            - 1 as libc::c_int) as size_t
            <= (!(0 as libc::c_int as size_t))
                .wrapping_div(::core::mem::size_of::<TValue>() as libc::c_ulong)
        {
            (((1 as libc::c_int)
                << 8 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int
                    + 8 as libc::c_int) - 1 as libc::c_int) as libc::c_uint
        } else {
            (!(0 as libc::c_int as size_t))
                .wrapping_div(::core::mem::size_of::<TValue>() as libc::c_ulong)
                as libc::c_uint
        }) as libc::c_int,
        b"constants\0" as *const u8 as *const libc::c_char,
    ) as *mut TValue;
    while oldsize < (*f).sizek {
        let fresh3 = oldsize;
        oldsize = oldsize + 1;
        (*((*f).k).offset(fresh3 as isize))
            .tt_ = (0 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int)
            as u8;
    }
    let mut io1: *mut TValue = &mut *((*f).k).offset(k as isize) as *mut TValue;
    let mut io2: *const TValue = v;
    (*io1).value_ = (*io2).value_;
    (*io1).tt_ = (*io2).tt_;
    (*fs).nk += 1;
    (*fs).nk;
    if (*v).tt_ as libc::c_int & (1 as libc::c_int) << 6 as libc::c_int != 0 {
        if (*f).marked as libc::c_int & (1 as libc::c_int) << 5 as libc::c_int != 0
            && (*(*v).value_.gc).marked as libc::c_int
                & ((1 as libc::c_int) << 3 as libc::c_int
                    | (1 as libc::c_int) << 4 as libc::c_int) != 0
        {
            luaC_barrier_(
                L,
                &mut (*(f as *mut GCUnion)).gc,
                &mut (*((*v).value_.gc as *mut GCUnion)).gc,
            );
        } else {};
    } else {};
    return k;
}
unsafe extern "C" fn stringK(
    mut fs: *mut FuncState,
    mut s: *mut TString,
) -> libc::c_int {
    let mut o: TValue = TValue {
        value_: Value { gc: 0 as *mut GCObject },
        tt_: 0,
    };
    let mut io: *mut TValue = &mut o;
    let mut x_: *mut TString = s;
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
    (*io)
        .tt_ = ((*x_).tt as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int)
        as u8;
    return addk(fs, &mut o, &mut o);
}
unsafe extern "C" fn luaK_intK(
    mut fs: *mut FuncState,
    mut n: lua_Integer,
) -> libc::c_int {
    let mut o: TValue = TValue {
        value_: Value { gc: 0 as *mut GCObject },
        tt_: 0,
    };
    let mut io: *mut TValue = &mut o;
    (*io).value_.i = n;
    (*io).tt_ = (3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int) as u8;
    return addk(fs, &mut o, &mut o);
}
unsafe extern "C" fn luaK_numberK(
    mut fs: *mut FuncState,
    mut r: lua_Number,
) -> libc::c_int {
    let mut o: TValue = TValue {
        value_: Value { gc: 0 as *mut GCObject },
        tt_: 0,
    };
    let mut ik: lua_Integer = 0;
    let mut io: *mut TValue = &mut o;
    (*io).value_.n = r;
    (*io).tt_ = (3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int) as u8;
    if luaV_flttointeger(r, &mut ik, F2Ieq) == 0 {
        return addk(fs, &mut o, &mut o)
    } else {
        let nbm: libc::c_int = 53 as libc::c_int;
        let q: lua_Number = ldexp(1.0f64, -nbm + 1 as libc::c_int);
        let k: lua_Number = if ik == 0 as libc::c_int as i64 {
            q
        } else {
            r + r * q
        };
        let mut kv: TValue = TValue {
            value_: Value { gc: 0 as *mut GCObject },
            tt_: 0,
        };
        let mut io_0: *mut TValue = &mut kv;
        (*io_0).value_.n = k;
        (*io_0)
            .tt_ = (3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int)
            as u8;
        return addk(fs, &mut kv, &mut o);
    };
}
unsafe extern "C" fn boolF(mut fs: *mut FuncState) -> libc::c_int {
    let mut o: TValue = TValue {
        value_: Value { gc: 0 as *mut GCObject },
        tt_: 0,
    };
    o.tt_ = (1 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int) as u8;
    return addk(fs, &mut o, &mut o);
}
unsafe extern "C" fn boolT(mut fs: *mut FuncState) -> libc::c_int {
    let mut o: TValue = TValue {
        value_: Value { gc: 0 as *mut GCObject },
        tt_: 0,
    };
    o.tt_ = (1 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int) as u8;
    return addk(fs, &mut o, &mut o);
}
unsafe extern "C" fn nilK(mut fs: *mut FuncState) -> libc::c_int {
    let mut k: TValue = TValue {
        value_: Value { gc: 0 as *mut GCObject },
        tt_: 0,
    };
    let mut v: TValue = TValue {
        value_: Value { gc: 0 as *mut GCObject },
        tt_: 0,
    };
    v.tt_ = (0 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int) as u8;
    let mut io: *mut TValue = &mut k;
    let mut x_: *mut Table = (*(*fs).ls).h;
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
    (*io)
        .tt_ = (5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 6 as libc::c_int) as u8;
    return addk(fs, &mut k, &mut v);
}
unsafe extern "C" fn fitsC(mut i: lua_Integer) -> libc::c_int {
    return ((i as lua_Unsigned)
        .wrapping_add(
            (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int
                >> 1 as libc::c_int) as libc::c_ulonglong,
        )
        <= (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int) as libc::c_uint
            as libc::c_ulonglong) as libc::c_int;
}
unsafe extern "C" fn fitsBx(mut i: lua_Integer) -> libc::c_int {
    return (-(((1 as libc::c_int)
        << 8 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int) - 1 as libc::c_int
        >> 1 as libc::c_int) as i64 <= i
        && i
            <= (((1 as libc::c_int)
                << 8 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int)
                - 1 as libc::c_int
                - (((1 as libc::c_int)
                    << 8 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int)
                    - 1 as libc::c_int >> 1 as libc::c_int)) as i64)
        as libc::c_int;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaK_int(
    mut fs: *mut FuncState,
    mut reg: libc::c_int,
    mut i: lua_Integer,
) {
    if fitsBx(i) != 0 {
        codeAsBx(fs, OP_LOADI, reg, i as libc::c_int);
    } else {
        luaK_codek(fs, reg, luaK_intK(fs, i));
    };
}
unsafe extern "C" fn luaK_float(
    mut fs: *mut FuncState,
    mut reg: libc::c_int,
    mut f: lua_Number,
) {
    let mut fi: lua_Integer = 0;
    if luaV_flttointeger(f, &mut fi, F2Ieq) != 0 && fitsBx(fi) != 0 {
        codeAsBx(fs, OP_LOADF, reg, fi as libc::c_int);
    } else {
        luaK_codek(fs, reg, luaK_numberK(fs, f));
    };
}
unsafe extern "C" fn const2exp(mut v: *mut TValue, mut e: *mut expdesc) {
    match (*v).tt_ as libc::c_int & 0x3f as libc::c_int {
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
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaK_setreturns(
    mut fs: *mut FuncState,
    mut e: *mut expdesc,
    mut nresults: libc::c_int,
) {
    let mut pc: *mut Instruction = &mut *((*(*fs).f).code).offset((*e).u.info as isize)
        as *mut Instruction;
    if (*e).k as libc::c_uint == VCALL as libc::c_int as libc::c_uint {
        *pc = *pc
            & !(!(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                << 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                    + 1 as libc::c_int + 8 as libc::c_int)
            | ((nresults + 1 as libc::c_int) as Instruction)
                << 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                    + 1 as libc::c_int + 8 as libc::c_int
                & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                    << 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                        + 1 as libc::c_int + 8 as libc::c_int;
    } else {
        *pc = *pc
            & !(!(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                << 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                    + 1 as libc::c_int + 8 as libc::c_int)
            | ((nresults + 1 as libc::c_int) as Instruction)
                << 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                    + 1 as libc::c_int + 8 as libc::c_int
                & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                    << 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                        + 1 as libc::c_int + 8 as libc::c_int;
        *pc = *pc
            & !(!(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                << 0 as libc::c_int + 7 as libc::c_int)
            | ((*fs).freereg as Instruction) << 0 as libc::c_int + 7 as libc::c_int
                & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                    << 0 as libc::c_int + 7 as libc::c_int;
        luaK_reserveregs(fs, 1 as libc::c_int);
    };
}
unsafe extern "C" fn str2K(mut fs: *mut FuncState, mut e: *mut expdesc) {
    (*e).u.info = stringK(fs, (*e).u.strval);
    (*e).k = VK;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaK_setoneret(mut fs: *mut FuncState, mut e: *mut expdesc) {
    if (*e).k as libc::c_uint == VCALL as libc::c_int as libc::c_uint {
        (*e).k = VNONRELOC;
        (*e)
            .u
            .info = (*((*(*fs).f).code).offset((*e).u.info as isize)
            >> 0 as libc::c_int + 7 as libc::c_int
            & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                << 0 as libc::c_int) as libc::c_int;
    } else if (*e).k as libc::c_uint == VVARARG as libc::c_int as libc::c_uint {
        *((*(*fs).f).code)
            .offset(
                (*e).u.info as isize,
            ) = *((*(*fs).f).code).offset((*e).u.info as isize)
            & !(!(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                << 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                    + 1 as libc::c_int + 8 as libc::c_int)
            | (2 as libc::c_int as Instruction)
                << 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                    + 1 as libc::c_int + 8 as libc::c_int
                & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                    << 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                        + 1 as libc::c_int + 8 as libc::c_int;
        (*e).k = VRELOC;
    }
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaK_dischargevars(
    mut fs: *mut FuncState,
    mut e: *mut expdesc,
) {
    match (*e).k as libc::c_uint {
        11 => {
            const2exp(const2val(fs, e), e);
        }
        9 => {
            let mut temp: libc::c_int = (*e).u.var.ridx as libc::c_int;
            (*e).u.info = temp;
            (*e).k = VNONRELOC;
        }
        10 => {
            (*e)
                .u
                .info = luaK_codeABCk(
                fs,
                OP_GETUPVAL,
                0 as libc::c_int,
                (*e).u.info,
                0 as libc::c_int,
                0 as libc::c_int,
            );
            (*e).k = VRELOC;
        }
        13 => {
            (*e)
                .u
                .info = luaK_codeABCk(
                fs,
                OP_GETTABUP,
                0 as libc::c_int,
                (*e).u.ind.t as libc::c_int,
                (*e).u.ind.index as libc::c_int,
                0 as libc::c_int,
            );
            (*e).k = VRELOC;
        }
        14 => {
            freereg(fs, (*e).u.ind.t as libc::c_int);
            (*e)
                .u
                .info = luaK_codeABCk(
                fs,
                OP_GETI,
                0 as libc::c_int,
                (*e).u.ind.t as libc::c_int,
                (*e).u.ind.index as libc::c_int,
                0 as libc::c_int,
            );
            (*e).k = VRELOC;
        }
        15 => {
            freereg(fs, (*e).u.ind.t as libc::c_int);
            (*e)
                .u
                .info = luaK_codeABCk(
                fs,
                OP_GETFIELD,
                0 as libc::c_int,
                (*e).u.ind.t as libc::c_int,
                (*e).u.ind.index as libc::c_int,
                0 as libc::c_int,
            );
            (*e).k = VRELOC;
        }
        12 => {
            freeregs(fs, (*e).u.ind.t as libc::c_int, (*e).u.ind.index as libc::c_int);
            (*e)
                .u
                .info = luaK_codeABCk(
                fs,
                OP_GETTABLE,
                0 as libc::c_int,
                (*e).u.ind.t as libc::c_int,
                (*e).u.ind.index as libc::c_int,
                0 as libc::c_int,
            );
            (*e).k = VRELOC;
        }
        19 | 18 => {
            luaK_setoneret(fs, e);
        }
        _ => {}
    };
}
unsafe extern "C" fn discharge2reg(
    mut fs: *mut FuncState,
    mut e: *mut expdesc,
    mut reg: libc::c_int,
) {
    luaK_dischargevars(fs, e);
    let mut current_block_14: u64;
    match (*e).k as libc::c_uint {
        1 => {
            luaK_nil(fs, reg, 1 as libc::c_int);
            current_block_14 = 13242334135786603907;
        }
        3 => {
            luaK_codeABCk(
                fs,
                OP_LOADFALSE,
                reg,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
            );
            current_block_14 = 13242334135786603907;
        }
        2 => {
            luaK_codeABCk(
                fs,
                OP_LOADTRUE,
                reg,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
            );
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
            let mut pc: *mut Instruction = &mut *((*(*fs).f).code)
                .offset((*e).u.info as isize) as *mut Instruction;
            *pc = *pc
                & !(!(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                    << 0 as libc::c_int + 7 as libc::c_int)
                | (reg as Instruction) << 0 as libc::c_int + 7 as libc::c_int
                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                        << 0 as libc::c_int + 7 as libc::c_int;
            current_block_14 = 13242334135786603907;
        }
        8 => {
            if reg != (*e).u.info {
                luaK_codeABCk(
                    fs,
                    OP_MOVE,
                    reg,
                    (*e).u.info,
                    0 as libc::c_int,
                    0 as libc::c_int,
                );
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
    if (*e).k as libc::c_uint != VNONRELOC as libc::c_int as libc::c_uint {
        luaK_reserveregs(fs, 1 as libc::c_int);
        discharge2reg(fs, e, (*fs).freereg as libc::c_int - 1 as libc::c_int);
    }
}
unsafe extern "C" fn code_loadbool(
    mut fs: *mut FuncState,
    mut A: libc::c_int,
    mut op: OpCode,
) -> libc::c_int {
    luaK_getlabel(fs);
    return luaK_codeABCk(
        fs,
        op,
        A,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    );
}
unsafe extern "C" fn need_value(
    mut fs: *mut FuncState,
    mut list: libc::c_int,
) -> libc::c_int {
    while list != -(1 as libc::c_int) {
        let mut i: Instruction = *getjumpcontrol(fs, list);
        if (i >> 0 as libc::c_int
            & !(!(0 as libc::c_int as Instruction) << 7 as libc::c_int)
                << 0 as libc::c_int) as OpCode as libc::c_uint
            != OP_TESTSET as libc::c_int as libc::c_uint
        {
            return 1 as libc::c_int;
        }
        list = getjump(fs, list);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn exp2reg(
    mut fs: *mut FuncState,
    mut e: *mut expdesc,
    mut reg: libc::c_int,
) {
    discharge2reg(fs, e, reg);
    if (*e).k as libc::c_uint == VJMP as libc::c_int as libc::c_uint {
        luaK_concat(fs, &mut (*e).t, (*e).u.info);
    }
    if (*e).t != (*e).f {
        let mut final_0: libc::c_int = 0;
        let mut p_f: libc::c_int = -(1 as libc::c_int);
        let mut p_t: libc::c_int = -(1 as libc::c_int);
        if need_value(fs, (*e).t) != 0 || need_value(fs, (*e).f) != 0 {
            let mut fj: libc::c_int = if (*e).k as libc::c_uint
                == VJMP as libc::c_int as libc::c_uint
            {
                -(1 as libc::c_int)
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
    (*e).t = -(1 as libc::c_int);
    (*e).f = (*e).t;
    (*e).u.info = reg;
    (*e).k = VNONRELOC;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaK_exp2nextreg(mut fs: *mut FuncState, mut e: *mut expdesc) {
    luaK_dischargevars(fs, e);
    freeexp(fs, e);
    luaK_reserveregs(fs, 1 as libc::c_int);
    exp2reg(fs, e, (*fs).freereg as libc::c_int - 1 as libc::c_int);
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaK_exp2anyreg(
    mut fs: *mut FuncState,
    mut e: *mut expdesc,
) -> libc::c_int {
    luaK_dischargevars(fs, e);
    if (*e).k as libc::c_uint == VNONRELOC as libc::c_int as libc::c_uint {
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
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaK_exp2anyregup(mut fs: *mut FuncState, mut e: *mut expdesc) {
    if (*e).k as libc::c_uint != VUPVAL as libc::c_int as libc::c_uint
        || (*e).t != (*e).f
    {
        luaK_exp2anyreg(fs, e);
    }
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaK_exp2val(mut fs: *mut FuncState, mut e: *mut expdesc) {
    if (*e).k as libc::c_uint == VJMP as libc::c_int as libc::c_uint || (*e).t != (*e).f
    {
        luaK_exp2anyreg(fs, e);
    } else {
        luaK_dischargevars(fs, e);
    };
}
unsafe extern "C" fn luaK_exp2K(
    mut fs: *mut FuncState,
    mut e: *mut expdesc,
) -> libc::c_int {
    if !((*e).t != (*e).f) {
        let mut info: libc::c_int = 0;
        match (*e).k as libc::c_uint {
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
            _ => return 0 as libc::c_int,
        }
        if info <= ((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int {
            (*e).k = VK;
            (*e).u.info = info;
            return 1 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn exp2RK(mut fs: *mut FuncState, mut e: *mut expdesc) -> libc::c_int {
    if luaK_exp2K(fs, e) != 0 {
        return 1 as libc::c_int
    } else {
        luaK_exp2anyreg(fs, e);
        return 0 as libc::c_int;
    };
}
unsafe extern "C" fn codeABRK(
    mut fs: *mut FuncState,
    mut o: OpCode,
    mut a: libc::c_int,
    mut b: libc::c_int,
    mut ec: *mut expdesc,
) {
    let mut k: libc::c_int = exp2RK(fs, ec);
    luaK_codeABCk(fs, o, a, b, (*ec).u.info, k);
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaK_storevar(
    mut fs: *mut FuncState,
    mut var: *mut expdesc,
    mut ex: *mut expdesc,
) {
    match (*var).k as libc::c_uint {
        9 => {
            freeexp(fs, ex);
            exp2reg(fs, ex, (*var).u.var.ridx as libc::c_int);
            return;
        }
        10 => {
            let mut e: libc::c_int = luaK_exp2anyreg(fs, ex);
            luaK_codeABCk(
                fs,
                OP_SETUPVAL,
                e,
                (*var).u.info,
                0 as libc::c_int,
                0 as libc::c_int,
            );
        }
        13 => {
            codeABRK(
                fs,
                OP_SETTABUP,
                (*var).u.ind.t as libc::c_int,
                (*var).u.ind.index as libc::c_int,
                ex,
            );
        }
        14 => {
            codeABRK(
                fs,
                OP_SETI,
                (*var).u.ind.t as libc::c_int,
                (*var).u.ind.index as libc::c_int,
                ex,
            );
        }
        15 => {
            codeABRK(
                fs,
                OP_SETFIELD,
                (*var).u.ind.t as libc::c_int,
                (*var).u.ind.index as libc::c_int,
                ex,
            );
        }
        12 => {
            codeABRK(
                fs,
                OP_SETTABLE,
                (*var).u.ind.t as libc::c_int,
                (*var).u.ind.index as libc::c_int,
                ex,
            );
        }
        _ => {}
    }
    freeexp(fs, ex);
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaK_self(
    mut fs: *mut FuncState,
    mut e: *mut expdesc,
    mut key: *mut expdesc,
) {
    let mut ereg: libc::c_int = 0;
    luaK_exp2anyreg(fs, e);
    ereg = (*e).u.info;
    freeexp(fs, e);
    (*e).u.info = (*fs).freereg as libc::c_int;
    (*e).k = VNONRELOC;
    luaK_reserveregs(fs, 2 as libc::c_int);
    codeABRK(fs, OP_SELF, (*e).u.info, ereg, key);
    freeexp(fs, key);
}
unsafe extern "C" fn negatecondition(mut fs: *mut FuncState, mut e: *mut expdesc) {
    let mut pc: *mut Instruction = getjumpcontrol(fs, (*e).u.info);
    *pc = *pc
        & !(!(!(0 as libc::c_int as Instruction) << 1 as libc::c_int)
            << 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int)
        | (((*pc >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
            & !(!(0 as libc::c_int as Instruction) << 1 as libc::c_int)
                << 0 as libc::c_int) as libc::c_int ^ 1 as libc::c_int) as Instruction)
            << 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
            & !(!(0 as libc::c_int as Instruction) << 1 as libc::c_int)
                << 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int;
}
unsafe extern "C" fn jumponcond(
    mut fs: *mut FuncState,
    mut e: *mut expdesc,
    mut cond: libc::c_int,
) -> libc::c_int {
    if (*e).k as libc::c_uint == VRELOC as libc::c_int as libc::c_uint {
        let mut ie: Instruction = *((*(*fs).f).code).offset((*e).u.info as isize);
        if (ie >> 0 as libc::c_int
            & !(!(0 as libc::c_int as Instruction) << 7 as libc::c_int)
                << 0 as libc::c_int) as OpCode as libc::c_uint
            == OP_NOT as libc::c_int as libc::c_uint
        {
            removelastinstruction(fs);
            return condjump(
                fs,
                OP_TEST,
                (ie
                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                        + 1 as libc::c_int
                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                        << 0 as libc::c_int) as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                (cond == 0) as libc::c_int,
            );
        }
    }
    discharge2anyreg(fs, e);
    freeexp(fs, e);
    return condjump(
        fs,
        OP_TESTSET,
        ((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int,
        (*e).u.info,
        0 as libc::c_int,
        cond,
    );
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaK_goiftrue(mut fs: *mut FuncState, mut e: *mut expdesc) {
    let mut pc: libc::c_int = 0;
    luaK_dischargevars(fs, e);
    match (*e).k as libc::c_uint {
        16 => {
            negatecondition(fs, e);
            pc = (*e).u.info;
        }
        4 | 5 | 6 | 7 | 2 => {
            pc = -(1 as libc::c_int);
        }
        _ => {
            pc = jumponcond(fs, e, 0 as libc::c_int);
        }
    }
    luaK_concat(fs, &mut (*e).f, pc);
    luaK_patchtohere(fs, (*e).t);
    (*e).t = -(1 as libc::c_int);
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaK_goiffalse(mut fs: *mut FuncState, mut e: *mut expdesc) {
    let mut pc: libc::c_int = 0;
    luaK_dischargevars(fs, e);
    match (*e).k as libc::c_uint {
        16 => {
            pc = (*e).u.info;
        }
        1 | 3 => {
            pc = -(1 as libc::c_int);
        }
        _ => {
            pc = jumponcond(fs, e, 1 as libc::c_int);
        }
    }
    luaK_concat(fs, &mut (*e).t, pc);
    luaK_patchtohere(fs, (*e).f);
    (*e).f = -(1 as libc::c_int);
}
unsafe extern "C" fn codenot(mut fs: *mut FuncState, mut e: *mut expdesc) {
    match (*e).k as libc::c_uint {
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
            (*e)
                .u
                .info = luaK_codeABCk(
                fs,
                OP_NOT,
                0 as libc::c_int,
                (*e).u.info,
                0 as libc::c_int,
                0 as libc::c_int,
            );
            (*e).k = VRELOC;
        }
        _ => {}
    }
    let mut temp: libc::c_int = (*e).f;
    (*e).f = (*e).t;
    (*e).t = temp;
    removevalues(fs, (*e).f);
    removevalues(fs, (*e).t);
}
unsafe extern "C" fn isKstr(mut fs: *mut FuncState, mut e: *mut expdesc) -> libc::c_int {
    return ((*e).k as libc::c_uint == VK as libc::c_int as libc::c_uint
        && !((*e).t != (*e).f)
        && (*e).u.info <= ((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int
        && (*((*(*fs).f).k).offset((*e).u.info as isize)).tt_ as libc::c_int
            == 4 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                | (1 as libc::c_int) << 6 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn isKint(mut e: *mut expdesc) -> libc::c_int {
    return ((*e).k as libc::c_uint == VKINT as libc::c_int as libc::c_uint
        && !((*e).t != (*e).f)) as libc::c_int;
}
unsafe extern "C" fn isCint(mut e: *mut expdesc) -> libc::c_int {
    return (isKint(e) != 0
        && (*e).u.ival as lua_Unsigned
            <= (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                as lua_Unsigned) as libc::c_int;
}
unsafe extern "C" fn isSCint(mut e: *mut expdesc) -> libc::c_int {
    return (isKint(e) != 0 && fitsC((*e).u.ival) != 0) as libc::c_int;
}
unsafe extern "C" fn isSCnumber(
    mut e: *mut expdesc,
    mut pi: *mut libc::c_int,
    mut isfloat: *mut libc::c_int,
) -> libc::c_int {
    let mut i: lua_Integer = 0;
    if (*e).k as libc::c_uint == VKINT as libc::c_int as libc::c_uint {
        i = (*e).u.ival;
    } else if (*e).k as libc::c_uint == VKFLT as libc::c_int as libc::c_uint
        && luaV_flttointeger((*e).u.nval, &mut i, F2Ieq) != 0
    {
        *isfloat = 1 as libc::c_int;
    } else {
        return 0 as libc::c_int
    }
    if !((*e).t != (*e).f) && fitsC(i) != 0 {
        *pi = i as libc::c_int
            + (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int
                >> 1 as libc::c_int);
        return 1 as libc::c_int;
    } else {
        return 0 as libc::c_int
    };
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaK_indexed(
    mut fs: *mut FuncState,
    mut t: *mut expdesc,
    mut k: *mut expdesc,
) {
    if (*k).k as libc::c_uint == VKSTR as libc::c_int as libc::c_uint {
        str2K(fs, k);
    }
    if (*t).k as libc::c_uint == VUPVAL as libc::c_int as libc::c_uint
        && isKstr(fs, k) == 0
    {
        luaK_exp2anyreg(fs, t);
    }
    if (*t).k as libc::c_uint == VUPVAL as libc::c_int as libc::c_uint {
        let mut temp: libc::c_int = (*t).u.info;
        (*t).u.ind.t = temp as u8;
        (*t).u.ind.index = (*k).u.info as libc::c_short;
        (*t).k = VINDEXUP;
    } else {
        (*t)
            .u
            .ind
            .t = (if (*t).k as libc::c_uint == VLOCAL as libc::c_int as libc::c_uint {
            (*t).u.var.ridx as libc::c_int
        } else {
            (*t).u.info
        }) as u8;
        if isKstr(fs, k) != 0 {
            (*t).u.ind.index = (*k).u.info as libc::c_short;
            (*t).k = VINDEXSTR;
        } else if isCint(k) != 0 {
            (*t).u.ind.index = (*k).u.ival as libc::c_int as libc::c_short;
            (*t).k = VINDEXI;
        } else {
            (*t).u.ind.index = luaK_exp2anyreg(fs, k) as libc::c_short;
            (*t).k = VINDEXED;
        }
    };
}
unsafe extern "C" fn validop(
    mut op: libc::c_int,
    mut v1: *mut TValue,
    mut v2: *mut TValue,
) -> libc::c_int {
    match op {
        7 | 8 | 9 | 10 | 11 | 13 => {
            let mut i: lua_Integer = 0;
            return (luaV_tointegerns(v1, &mut i, F2Ieq) != 0
                && luaV_tointegerns(v2, &mut i, F2Ieq) != 0) as libc::c_int;
        }
        5 | 6 | 3 => {
            return ((if (*v2).tt_ as libc::c_int
                == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
            {
                (*v2).value_.i as lua_Number
            } else {
                (*v2).value_.n
            }) != 0 as libc::c_int as f64) as libc::c_int;
        }
        _ => return 1 as libc::c_int,
    };
}
unsafe extern "C" fn constfolding(
    mut fs: *mut FuncState,
    mut op: libc::c_int,
    mut e1: *mut expdesc,
    mut e2: *const expdesc,
) -> libc::c_int {
    let mut v1: TValue = TValue {
        value_: Value { gc: 0 as *mut GCObject },
        tt_: 0,
    };
    let mut v2: TValue = TValue {
        value_: Value { gc: 0 as *mut GCObject },
        tt_: 0,
    };
    let mut res: TValue = TValue {
        value_: Value { gc: 0 as *mut GCObject },
        tt_: 0,
    };
    if tonumeral(e1, &mut v1) == 0 || tonumeral(e2, &mut v2) == 0
        || validop(op, &mut v1, &mut v2) == 0
    {
        return 0 as libc::c_int;
    }
    luaO_rawarith((*(*fs).ls).L, op, &mut v1, &mut v2, &mut res);
    if res.tt_ as libc::c_int
        == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
    {
        (*e1).k = VKINT;
        (*e1).u.ival = res.value_.i;
    } else {
        let mut n: lua_Number = res.value_.n;
        if !(n == n) || n == 0 as libc::c_int as f64 {
            return 0 as libc::c_int;
        }
        (*e1).k = VKFLT;
        (*e1).u.nval = n;
    }
    return 1 as libc::c_int;
}
#[inline]
unsafe extern "C" fn binopr2op(
    mut opr: BinOpr,
    mut baser: BinOpr,
    mut base: OpCode,
) -> OpCode {
    return (opr as libc::c_int - baser as libc::c_int + base as libc::c_int) as OpCode;
}
#[inline]
unsafe extern "C" fn unopr2op(mut opr: UnOpr) -> OpCode {
    return (opr as libc::c_int - OPR_MINUS as libc::c_int + OP_UNM as libc::c_int)
        as OpCode;
}
#[inline]
unsafe extern "C" fn binopr2TM(mut opr: BinOpr) -> TMS {
    return (opr as libc::c_int - OPR_ADD as libc::c_int + TM_ADD as libc::c_int) as TMS;
}
unsafe extern "C" fn codeunexpval(
    mut fs: *mut FuncState,
    mut op: OpCode,
    mut e: *mut expdesc,
    mut line: libc::c_int,
) {
    let mut r: libc::c_int = luaK_exp2anyreg(fs, e);
    freeexp(fs, e);
    (*e)
        .u
        .info = luaK_codeABCk(
        fs,
        op,
        0 as libc::c_int,
        r,
        0 as libc::c_int,
        0 as libc::c_int,
    );
    (*e).k = VRELOC;
    luaK_fixline(fs, line);
}
unsafe extern "C" fn finishbinexpval(
    mut fs: *mut FuncState,
    mut e1: *mut expdesc,
    mut e2: *mut expdesc,
    mut op: OpCode,
    mut v2: libc::c_int,
    mut flip: libc::c_int,
    mut line: libc::c_int,
    mut mmop: OpCode,
    mut event: TMS,
) {
    let mut v1: libc::c_int = luaK_exp2anyreg(fs, e1);
    let mut pc: libc::c_int = luaK_codeABCk(
        fs,
        op,
        0 as libc::c_int,
        v1,
        v2,
        0 as libc::c_int,
    );
    freeexps(fs, e1, e2);
    (*e1).u.info = pc;
    (*e1).k = VRELOC;
    luaK_fixline(fs, line);
    luaK_codeABCk(fs, mmop, v1, v2, event as libc::c_int, flip);
    luaK_fixline(fs, line);
}
unsafe extern "C" fn codebinexpval(
    mut fs: *mut FuncState,
    mut opr: BinOpr,
    mut e1: *mut expdesc,
    mut e2: *mut expdesc,
    mut line: libc::c_int,
) {
    let mut op: OpCode = binopr2op(opr, OPR_ADD, OP_ADD);
    let mut v2: libc::c_int = luaK_exp2anyreg(fs, e2);
    finishbinexpval(
        fs,
        e1,
        e2,
        op,
        v2,
        0 as libc::c_int,
        line,
        OP_MMBIN,
        binopr2TM(opr),
    );
}
unsafe extern "C" fn codebini(
    mut fs: *mut FuncState,
    mut op: OpCode,
    mut e1: *mut expdesc,
    mut e2: *mut expdesc,
    mut flip: libc::c_int,
    mut line: libc::c_int,
    mut event: TMS,
) {
    let mut v2: libc::c_int = (*e2).u.ival as libc::c_int
        + (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int
            >> 1 as libc::c_int);
    finishbinexpval(fs, e1, e2, op, v2, flip, line, OP_MMBINI, event);
}
unsafe extern "C" fn codebinK(
    mut fs: *mut FuncState,
    mut opr: BinOpr,
    mut e1: *mut expdesc,
    mut e2: *mut expdesc,
    mut flip: libc::c_int,
    mut line: libc::c_int,
) {
    let mut event: TMS = binopr2TM(opr);
    let mut v2: libc::c_int = (*e2).u.info;
    let mut op: OpCode = binopr2op(opr, OPR_ADD, OP_ADDK);
    finishbinexpval(fs, e1, e2, op, v2, flip, line, OP_MMBINK, event);
}
unsafe extern "C" fn finishbinexpneg(
    mut fs: *mut FuncState,
    mut e1: *mut expdesc,
    mut e2: *mut expdesc,
    mut op: OpCode,
    mut line: libc::c_int,
    mut event: TMS,
) -> libc::c_int {
    if isKint(e2) == 0 {
        return 0 as libc::c_int
    } else {
        let mut i2: lua_Integer = (*e2).u.ival;
        if !(fitsC(i2) != 0 && fitsC(-i2) != 0) {
            return 0 as libc::c_int
        } else {
            let mut v2: libc::c_int = i2 as libc::c_int;
            finishbinexpval(
                fs,
                e1,
                e2,
                op,
                -v2
                    + (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int
                        >> 1 as libc::c_int),
                0 as libc::c_int,
                line,
                OP_MMBINI,
                event,
            );
            *((*(*fs).f).code)
                .offset(
                    ((*fs).pc - 1 as libc::c_int) as isize,
                ) = *((*(*fs).f).code).offset(((*fs).pc - 1 as libc::c_int) as isize)
                & !(!(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                    << 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                        + 1 as libc::c_int)
                | ((v2
                    + (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int
                        >> 1 as libc::c_int)) as Instruction)
                    << 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                        + 1 as libc::c_int
                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                        << 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                            + 1 as libc::c_int;
            return 1 as libc::c_int;
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
    mut flip: libc::c_int,
    mut line: libc::c_int,
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
    mut flip: libc::c_int,
    mut line: libc::c_int,
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
    mut line: libc::c_int,
) {
    let mut flip: libc::c_int = 0 as libc::c_int;
    if tonumeral(e1, 0 as *mut TValue) != 0 {
        swapexps(e1, e2);
        flip = 1 as libc::c_int;
    }
    if op as libc::c_uint == OPR_ADD as libc::c_int as libc::c_uint && isSCint(e2) != 0 {
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
    mut line: libc::c_int,
) {
    let mut flip: libc::c_int = 0 as libc::c_int;
    if (*e1).k as libc::c_uint == VKINT as libc::c_int as libc::c_uint {
        swapexps(e1, e2);
        flip = 1 as libc::c_int;
    }
    if (*e2).k as libc::c_uint == VKINT as libc::c_int as libc::c_uint
        && luaK_exp2K(fs, e2) != 0
    {
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
    let mut r1: libc::c_int = 0;
    let mut r2: libc::c_int = 0;
    let mut im: libc::c_int = 0;
    let mut isfloat: libc::c_int = 0 as libc::c_int;
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
    (*e1).u.info = condjump(fs, op, r1, r2, isfloat, 1 as libc::c_int);
    (*e1).k = VJMP;
}
unsafe extern "C" fn codeeq(
    mut fs: *mut FuncState,
    mut opr: BinOpr,
    mut e1: *mut expdesc,
    mut e2: *mut expdesc,
) {
    let mut r1: libc::c_int = 0;
    let mut r2: libc::c_int = 0;
    let mut im: libc::c_int = 0;
    let mut isfloat: libc::c_int = 0 as libc::c_int;
    let mut op: OpCode = OP_MOVE;
    if (*e1).k as libc::c_uint != VNONRELOC as libc::c_int as libc::c_uint {
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
    (*e1)
        .u
        .info = condjump(
        fs,
        op,
        r1,
        r2,
        isfloat,
        (opr as libc::c_uint == OPR_EQ as libc::c_int as libc::c_uint) as libc::c_int,
    );
    (*e1).k = VJMP;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaK_prefix(
    mut fs: *mut FuncState,
    mut opr: UnOpr,
    mut e: *mut expdesc,
    mut line: libc::c_int,
) {
    static mut ef: expdesc = {
        let mut init = expdesc {
            k: VKINT,
            u: C2RustUnnamed_11 {
                ival: 0 as libc::c_int as lua_Integer,
            },
            t: -(1 as libc::c_int),
            f: -(1 as libc::c_int),
        };
        init
    };
    luaK_dischargevars(fs, e);
    let mut current_block_3: u64;
    match opr as libc::c_uint {
        0 | 1 => {
            if constfolding(
                fs,
                (opr as libc::c_uint).wrapping_add(12 as libc::c_int as libc::c_uint)
                    as libc::c_int,
                e,
                &ef,
            ) != 0
            {
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
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaK_infix(
    mut fs: *mut FuncState,
    mut op: BinOpr,
    mut v: *mut expdesc,
) {
    luaK_dischargevars(fs, v);
    match op as libc::c_uint {
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
            let mut dummy: libc::c_int = 0;
            let mut dummy2: libc::c_int = 0;
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
    mut line: libc::c_int,
) {
    let mut ie2: *mut Instruction = previousinstruction(fs);
    if (*ie2 >> 0 as libc::c_int
        & !(!(0 as libc::c_int as Instruction) << 7 as libc::c_int) << 0 as libc::c_int)
        as OpCode as libc::c_uint == OP_CONCAT as libc::c_int as libc::c_uint
    {
        let mut n: libc::c_int = (*ie2
            >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int
            & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                << 0 as libc::c_int) as libc::c_int;
        freeexp(fs, e2);
        *ie2 = *ie2
            & !(!(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                << 0 as libc::c_int + 7 as libc::c_int)
            | ((*e1).u.info as Instruction) << 0 as libc::c_int + 7 as libc::c_int
                & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                    << 0 as libc::c_int + 7 as libc::c_int;
        *ie2 = *ie2
            & !(!(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                << 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                    + 1 as libc::c_int)
            | ((n + 1 as libc::c_int) as Instruction)
                << 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                    + 1 as libc::c_int
                & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                    << 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                        + 1 as libc::c_int;
    } else {
        luaK_codeABCk(
            fs,
            OP_CONCAT,
            (*e1).u.info,
            2 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
        );
        freeexp(fs, e2);
        luaK_fixline(fs, line);
    };
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaK_posfix(
    mut fs: *mut FuncState,
    mut opr: BinOpr,
    mut e1: *mut expdesc,
    mut e2: *mut expdesc,
    mut line: libc::c_int,
) {
    luaK_dischargevars(fs, e2);
    if opr as libc::c_uint <= OPR_SHR as libc::c_int as libc::c_uint
        && constfolding(
            fs,
            (opr as libc::c_uint).wrapping_add(0 as libc::c_int as libc::c_uint)
                as libc::c_int,
            e1,
            e2,
        ) != 0
    {
        return;
    }
    let mut current_block_30: u64;
    match opr as libc::c_uint {
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
                codebini(fs, OP_SHLI, e1, e2, 1 as libc::c_int, line, TM_SHL);
            } else if !(finishbinexpneg(fs, e1, e2, OP_SHRI, line, TM_SHL) != 0) {
                codebinexpval(fs, opr, e1, e2, line);
            }
            current_block_30 = 8180496224585318153;
        }
        11 => {
            if isSCint(e2) != 0 {
                codebini(fs, OP_SHRI, e1, e2, 0 as libc::c_int, line, TM_SHR);
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
            opr = (opr as libc::c_uint)
                .wrapping_sub(OPR_GT as libc::c_int as libc::c_uint)
                .wrapping_add(OPR_LT as libc::c_int as libc::c_uint) as BinOpr;
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
            codearith(fs, opr, e1, e2, 0 as libc::c_int, line);
        }
        1118134448028020070 => {
            codeorder(fs, opr, e1, e2);
        }
        _ => {}
    };
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaK_fixline(mut fs: *mut FuncState, mut line: libc::c_int) {
    removelastlineinfo(fs);
    savelineinfo(fs, (*fs).f, line);
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaK_settablesize(
    mut fs: *mut FuncState,
    mut pc: libc::c_int,
    mut ra: libc::c_int,
    mut asize: libc::c_int,
    mut hsize: libc::c_int,
) {
    let mut inst: *mut Instruction = &mut *((*(*fs).f).code).offset(pc as isize)
        as *mut Instruction;
    let mut rb: libc::c_int = if hsize != 0 as libc::c_int {
        luaO_ceillog2(hsize as libc::c_uint) + 1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    let mut extra: libc::c_int = asize
        / (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int
            + 1 as libc::c_int);
    let mut rc: libc::c_int = asize
        % (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int
            + 1 as libc::c_int);
    let mut k: libc::c_int = (extra > 0 as libc::c_int) as libc::c_int;
    *inst = (OP_NEWTABLE as libc::c_int as Instruction) << 0 as libc::c_int
        | (ra as Instruction) << 0 as libc::c_int + 7 as libc::c_int
        | (rb as Instruction)
            << 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int
        | (rc as Instruction)
            << 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int
                + 8 as libc::c_int
        | (k as Instruction) << 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int;
    *inst
        .offset(
            1 as libc::c_int as isize,
        ) = (OP_EXTRAARG as libc::c_int as Instruction) << 0 as libc::c_int
        | (extra as Instruction) << 0 as libc::c_int + 7 as libc::c_int;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaK_setlist(
    mut fs: *mut FuncState,
    mut base: libc::c_int,
    mut nelems: libc::c_int,
    mut tostore: libc::c_int,
) {
    if tostore == -(1 as libc::c_int) {
        tostore = 0 as libc::c_int;
    }
    if nelems <= ((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int {
        luaK_codeABCk(fs, OP_SETLIST, base, tostore, nelems, 0 as libc::c_int);
    } else {
        let mut extra: libc::c_int = nelems
            / (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int
                + 1 as libc::c_int);
        nelems
            %= ((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int
                + 1 as libc::c_int;
        luaK_codeABCk(fs, OP_SETLIST, base, tostore, nelems, 1 as libc::c_int);
        codeextraarg(fs, extra);
    }
    (*fs).freereg = (base + 1 as libc::c_int) as u8;
}
unsafe extern "C" fn finaltarget(
    mut code: *mut Instruction,
    mut i: libc::c_int,
) -> libc::c_int {
    let mut count: libc::c_int = 0;
    count = 0 as libc::c_int;
    while count < 100 as libc::c_int {
        let mut pc: Instruction = *code.offset(i as isize);
        if (pc >> 0 as libc::c_int
            & !(!(0 as libc::c_int as Instruction) << 7 as libc::c_int)
                << 0 as libc::c_int) as OpCode as libc::c_uint
            != OP_JMP as libc::c_int as libc::c_uint
        {
            break;
        }
        i
            += (pc >> 0 as libc::c_int + 7 as libc::c_int
                & !(!(0 as libc::c_int as Instruction)
                    << 8 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int
                        + 8 as libc::c_int) << 0 as libc::c_int) as libc::c_int
                - (((1 as libc::c_int)
                    << 8 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int
                        + 8 as libc::c_int) - 1 as libc::c_int >> 1 as libc::c_int)
                + 1 as libc::c_int;
        count += 1;
        count;
    }
    return i;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaK_finish(mut fs: *mut FuncState) {
    let mut i: libc::c_int = 0;
    let mut p: *mut Proto = (*fs).f;
    i = 0 as libc::c_int;
    while i < (*fs).pc {
        let mut pc: *mut Instruction = &mut *((*p).code).offset(i as isize)
            as *mut Instruction;
        let mut current_block_7: u64;
        match (*pc >> 0 as libc::c_int
            & !(!(0 as libc::c_int as Instruction) << 7 as libc::c_int)
                << 0 as libc::c_int) as OpCode as libc::c_uint
        {
            71 | 72 => {
                if !((*fs).needclose as libc::c_int != 0
                    || (*p).is_vararg as libc::c_int != 0)
                {
                    current_block_7 = 12599329904712511516;
                } else {
                    *pc = *pc
                        & !(!(!(0 as libc::c_int as Instruction) << 7 as libc::c_int)
                            << 0 as libc::c_int)
                        | (OP_RETURN as libc::c_int as Instruction) << 0 as libc::c_int
                            & !(!(0 as libc::c_int as Instruction) << 7 as libc::c_int)
                                << 0 as libc::c_int;
                    current_block_7 = 11006700562992250127;
                }
            }
            70 | 69 => {
                current_block_7 = 11006700562992250127;
            }
            56 => {
                let mut target: libc::c_int = finaltarget((*p).code, i);
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
                    *pc = *pc
                        & !(!(!(0 as libc::c_int as Instruction) << 1 as libc::c_int)
                            << 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int)
                        | (1 as libc::c_int as Instruction)
                            << 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                            & !(!(0 as libc::c_int as Instruction) << 1 as libc::c_int)
                                << 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int;
                }
                if (*p).is_vararg != 0 {
                    *pc = *pc
                        & !(!(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                            << 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                + 1 as libc::c_int + 8 as libc::c_int)
                        | (((*p).numparams as libc::c_int + 1 as libc::c_int)
                            as Instruction)
                            << 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                + 1 as libc::c_int + 8 as libc::c_int
                            & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                                << 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                                    + 1 as libc::c_int + 8 as libc::c_int;
                }
            }
            _ => {}
        }
        i += 1;
        i;
    }
}
