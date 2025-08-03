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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> i32;
    fn luaE_incCstack(L: *mut lua_State);
    fn luaO_pushfstring(
        L: *mut lua_State,
        fmt: *const libc::c_char,
        _: ...
    ) -> *const libc::c_char;
    fn luaM_growaux_(
        L: *mut lua_State,
        block_0: *mut libc::c_void,
        nelems: i32,
        size: *mut i32,
        size_elem: i32,
        limit: i32,
        what: *const libc::c_char,
    ) -> *mut libc::c_void;
    fn luaM_shrinkvector_(
        L: *mut lua_State,
        block_0: *mut libc::c_void,
        nelem: *mut i32,
        final_n: i32,
        size_elem: i32,
    ) -> *mut libc::c_void;
    fn luaX_setinput(
        L: *mut lua_State,
        ls: *mut LexState,
        z: *mut ZIO,
        source: *mut TString,
        firstchar: i32,
    );
    fn luaX_newstring(
        ls: *mut LexState,
        str: *const libc::c_char,
        l: size_t,
    ) -> *mut TString;
    fn luaX_next(ls: *mut LexState);
    fn luaX_lookahead(ls: *mut LexState) -> i32;
    fn luaX_syntaxerror(ls: *mut LexState, s: *const libc::c_char) -> !;
    fn luaX_token2str(ls: *mut LexState, token: i32) -> *const libc::c_char;
    fn luaK_code(fs: *mut FuncState, i: Instruction) -> i32;
    fn luaK_codeABx(
        fs: *mut FuncState,
        o: OpCode,
        A: i32,
        Bx: libc::c_uint,
    ) -> i32;
    fn luaK_codeABCk(
        fs: *mut FuncState,
        o: OpCode,
        A: i32,
        B: i32,
        C: i32,
        k: i32,
    ) -> i32;
    fn luaK_exp2const(
        fs: *mut FuncState,
        e: *const expdesc,
        v: *mut TValue,
    ) -> i32;
    fn luaK_fixline(fs: *mut FuncState, line: i32);
    fn luaK_nil(fs: *mut FuncState, from: i32, n: i32);
    fn luaK_reserveregs(fs: *mut FuncState, n: i32);
    fn luaK_checkstack(fs: *mut FuncState, n: i32);
    fn luaK_int(fs: *mut FuncState, reg: i32, n: Integer);
    fn luaK_dischargevars(fs: *mut FuncState, e: *mut expdesc);
    fn luaK_exp2anyreg(fs: *mut FuncState, e: *mut expdesc) -> i32;
    fn luaK_exp2anyregup(fs: *mut FuncState, e: *mut expdesc);
    fn luaK_exp2nextreg(fs: *mut FuncState, e: *mut expdesc);
    fn luaK_exp2val(fs: *mut FuncState, e: *mut expdesc);
    fn luaK_self(fs: *mut FuncState, e: *mut expdesc, key: *mut expdesc);
    fn luaK_indexed(fs: *mut FuncState, t: *mut expdesc, k: *mut expdesc);
    fn luaK_goiftrue(fs: *mut FuncState, e: *mut expdesc);
    fn luaK_goiffalse(fs: *mut FuncState, e: *mut expdesc);
    fn luaK_storevar(fs: *mut FuncState, var: *mut expdesc, e: *mut expdesc);
    fn luaK_setreturns(fs: *mut FuncState, e: *mut expdesc, nresults: i32);
    fn luaK_setoneret(fs: *mut FuncState, e: *mut expdesc);
    fn luaK_jump(fs: *mut FuncState) -> i32;
    fn luaK_ret(fs: *mut FuncState, first: i32, nret: i32);
    fn luaK_patchlist(fs: *mut FuncState, list: i32, target: i32);
    fn luaK_patchtohere(fs: *mut FuncState, list: i32);
    fn luaK_concat(fs: *mut FuncState, l1: *mut i32, l2: i32);
    fn luaK_getlabel(fs: *mut FuncState) -> i32;
    fn luaK_prefix(fs: *mut FuncState, op: UnOpr, v: *mut expdesc, line: i32);
    fn luaK_infix(fs: *mut FuncState, op: BinOpr, v: *mut expdesc);
    fn luaK_posfix(
        fs: *mut FuncState,
        op: BinOpr,
        v1: *mut expdesc,
        v2: *mut expdesc,
        line: i32,
    );
    fn luaK_settablesize(
        fs: *mut FuncState,
        pc: i32,
        ra: i32,
        asize: i32,
        hsize: i32,
    );
    fn luaK_setlist(
        fs: *mut FuncState,
        base: i32,
        nelems: i32,
        tostore: i32,
    );
    fn luaK_finish(fs: *mut FuncState);
    fn luaK_semerror(ls: *mut LexState, msg: *const libc::c_char) -> !;
    fn luaD_inctop(L: *mut lua_State);
    fn luaF_newproto(L: *mut lua_State) -> *mut Proto;
    fn luaF_newLclosure(L: *mut lua_State, nupvals: i32) -> *mut LClosure;
    fn luaC_step(L: *mut lua_State);
    fn luaC_barrier_(L: *mut lua_State, o: *mut GCObject, v: *mut GCObject);
    fn luaS_newlstr(
        L: *mut lua_State,
        str: *const libc::c_char,
        l: size_t,
    ) -> *mut TString;
    fn luaS_new(L: *mut lua_State, str: *const libc::c_char) -> *mut TString;
    fn luaH_new(L: *mut lua_State) -> *mut Table;
}
pub type size_t = libc::c_ulong;
pub type ptrdiff_t = libc::c_long;
pub type __sig_atomic_t = i32;
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
pub type RESERVED = libc::c_uint;
pub const TK_STRING: RESERVED = 292;
pub const TK_NAME: RESERVED = 291;
pub const TK_INT: RESERVED = 290;
pub const TK_FLT: RESERVED = 289;
pub const TK_EOS: RESERVED = 288;
pub const TK_DBCOLON: RESERVED = 287;
pub const TK_SHR: RESERVED = 286;
pub const TK_SHL: RESERVED = 285;
pub const TK_NE: RESERVED = 284;
pub const TK_LE: RESERVED = 283;
pub const TK_GE: RESERVED = 282;
pub const TK_EQ: RESERVED = 281;
pub const TK_DOTS: RESERVED = 280;
pub const TK_CONCAT: RESERVED = 279;
pub const TK_IDIV: RESERVED = 278;
pub const TK_WHILE: RESERVED = 277;
pub const TK_UNTIL: RESERVED = 276;
pub const TK_TRUE: RESERVED = 275;
pub const TK_THEN: RESERVED = 274;
pub const TK_RETURN: RESERVED = 273;
pub const TK_REPEAT: RESERVED = 272;
pub const TK_OR: RESERVED = 271;
pub const TK_NOT: RESERVED = 270;
pub const TK_NIL: RESERVED = 269;
pub const TK_LOCAL: RESERVED = 268;
pub const TK_IN: RESERVED = 267;
pub const TK_IF: RESERVED = 266;
pub const TK_GOTO: RESERVED = 265;
pub const TK_FUNCTION: RESERVED = 264;
pub const TK_FOR: RESERVED = 263;
pub const TK_FALSE: RESERVED = 262;
pub const TK_END: RESERVED = 261;
pub const TK_ELSEIF: RESERVED = 260;
pub const TK_ELSE: RESERVED = 259;
pub const TK_DO: RESERVED = 258;
pub const TK_BREAK: RESERVED = 257;
pub const TK_AND: RESERVED = 256;
#[derive(Copy, Clone)]
#[repr(C)]
pub union SemInfo {
    pub r: Number,
    pub i: Integer,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BlockCnt {
    pub previous: *mut BlockCnt,
    pub firstlabel: i32,
    pub firstgoto: i32,
    pub nactvar: u8,
    pub upval: u8,
    pub isloop: u8,
    pub insidetbc: u8,
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
    pub t: i32,
    pub f: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
    pub ival: Integer,
    pub nval: Number,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LHS_assign {
    pub prev: *mut LHS_assign,
    pub v: expdesc,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub left: u8,
    pub right: u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ConsControl {
    pub v: expdesc,
    pub t: *mut expdesc,
    pub nh: i32,
    pub na: i32,
    pub tostore: i32,
}
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
pub type UnOpr = libc::c_uint;
pub const OPR_NOUNOPR: UnOpr = 4;
pub const OPR_LEN: UnOpr = 3;
pub const OPR_NOT: UnOpr = 2;
pub const OPR_BNOT: UnOpr = 1;
pub const OPR_MINUS: UnOpr = 0;
unsafe extern "C" fn error_expected(mut ls: *mut LexState, mut token: i32) -> ! {
    luaX_syntaxerror(
        ls,
        luaO_pushfstring(
            (*ls).L,
            b"%s expected\0" as *const u8 as *const libc::c_char,
            luaX_token2str(ls, token),
        ),
    );
}
unsafe extern "C" fn errorlimit(
    mut fs: *mut FuncState,
    mut limit: i32,
    mut what: *const libc::c_char,
) -> ! {
    let mut L: *mut lua_State = (*(*fs).ls).L;
    let mut msg: *const libc::c_char = 0 as *const libc::c_char;
    let mut line: i32 = (*(*fs).f).linedefined;
    let mut where_0: *const libc::c_char = if line == 0 as i32 {
        b"main function\0" as *const u8 as *const libc::c_char
    } else {
        luaO_pushfstring(
            L,
            b"function at line %d\0" as *const u8 as *const libc::c_char,
            line,
        )
    };
    msg = luaO_pushfstring(
        L,
        b"too many %s (limit is %d) in %s\0" as *const u8 as *const libc::c_char,
        what,
        limit,
        where_0,
    );
    luaX_syntaxerror((*fs).ls, msg);
}
unsafe extern "C" fn checklimit(
    mut fs: *mut FuncState,
    mut v: i32,
    mut l: i32,
    mut what: *const libc::c_char,
) {
    if v > l {
        errorlimit(fs, l, what);
    }
}
unsafe extern "C" fn testnext(mut ls: *mut LexState, mut c: i32) -> i32 {
    if (*ls).t.token == c {
        luaX_next(ls);
        return 1 as i32;
    } else {
        return 0 as i32
    };
}
unsafe extern "C" fn check(mut ls: *mut LexState, mut c: i32) {
    if (*ls).t.token != c {
        error_expected(ls, c);
    }
}
unsafe extern "C" fn checknext(mut ls: *mut LexState, mut c: i32) {
    check(ls, c);
    luaX_next(ls);
}
unsafe extern "C" fn check_match(
    mut ls: *mut LexState,
    mut what: i32,
    mut who: i32,
    mut where_0: i32,
) {
    if ((testnext(ls, what) == 0) as i32 != 0 as i32) as i32
        as libc::c_long != 0
    {
        if where_0 == (*ls).linenumber {
            error_expected(ls, what);
        } else {
            luaX_syntaxerror(
                ls,
                luaO_pushfstring(
                    (*ls).L,
                    b"%s expected (to close %s at line %d)\0" as *const u8
                        as *const libc::c_char,
                    luaX_token2str(ls, what),
                    luaX_token2str(ls, who),
                    where_0,
                ),
            );
        }
    }
}
unsafe extern "C" fn str_checkname(mut ls: *mut LexState) -> *mut TString {
    let mut ts: *mut TString = 0 as *mut TString;
    check(ls, TK_NAME as i32);
    ts = (*ls).t.seminfo.ts;
    luaX_next(ls);
    return ts;
}
unsafe extern "C" fn init_exp(mut e: *mut expdesc, mut k: expkind, mut i: i32) {
    (*e).t = -(1 as i32);
    (*e).f = (*e).t;
    (*e).k = k;
    (*e).u.info = i;
}
unsafe extern "C" fn codestring(mut e: *mut expdesc, mut s: *mut TString) {
    (*e).t = -(1 as i32);
    (*e).f = (*e).t;
    (*e).k = VKSTR;
    (*e).u.strval = s;
}
unsafe extern "C" fn codename(mut ls: *mut LexState, mut e: *mut expdesc) {
    codestring(e, str_checkname(ls));
}
unsafe extern "C" fn registerlocalvar(
    mut ls: *mut LexState,
    mut fs: *mut FuncState,
    mut varname: *mut TString,
) -> i32 {
    let mut f: *mut Proto = (*fs).f;
    let mut oldsize: i32 = (*f).sizelocvars;
    (*f)
        .locvars = luaM_growaux_(
        (*ls).L,
        (*f).locvars as *mut libc::c_void,
        (*fs).ndebugvars as i32,
        &mut (*f).sizelocvars,
        ::core::mem::size_of::<LocVar>() as libc::c_ulong as i32,
        (if 32767 as i32 as size_t
            <= (!(0 as i32 as size_t))
                .wrapping_div(::core::mem::size_of::<LocVar>() as libc::c_ulong)
        {
            32767 as i32 as libc::c_uint
        } else {
            (!(0 as i32 as size_t))
                .wrapping_div(::core::mem::size_of::<LocVar>() as libc::c_ulong)
                as libc::c_uint
        }) as i32,
        b"local variables\0" as *const u8 as *const libc::c_char,
    ) as *mut LocVar;
    while oldsize < (*f).sizelocvars {
        let fresh0 = oldsize;
        oldsize = oldsize + 1;
        let ref mut fresh1 = (*((*f).locvars).offset(fresh0 as isize)).varname;
        *fresh1 = 0 as *mut TString;
    }
    let ref mut fresh2 = (*((*f).locvars).offset((*fs).ndebugvars as isize)).varname;
    *fresh2 = varname;
    (*((*f).locvars).offset((*fs).ndebugvars as isize)).startpc = (*fs).pc;
    if (*f).marked as i32 & (1 as i32) << 5 as i32 != 0
        && (*varname).marked as i32
            & ((1 as i32) << 3 as i32
                | (1 as i32) << 4 as i32) != 0
    {
        luaC_barrier_(
            (*ls).L,
            &mut (*(f as *mut GCUnion)).gc,
            &mut (*(varname as *mut GCUnion)).gc,
        );
    } else {};
    let fresh3 = (*fs).ndebugvars;
    (*fs).ndebugvars = (*fs).ndebugvars + 1;
    return fresh3 as i32;
}
unsafe extern "C" fn new_localvar(
    mut ls: *mut LexState,
    mut name: *mut TString,
) -> i32 {
    let mut L: *mut lua_State = (*ls).L;
    let mut fs: *mut FuncState = (*ls).fs;
    let mut dyd: *mut Dyndata = (*ls).dyd;
    let mut var: *mut Vardesc = 0 as *mut Vardesc;
    checklimit(
        fs,
        (*dyd).actvar.n + 1 as i32 - (*fs).firstlocal,
        200 as i32,
        b"local variables\0" as *const u8 as *const libc::c_char,
    );
    (*dyd)
        .actvar
        .arr = luaM_growaux_(
        L,
        (*dyd).actvar.arr as *mut libc::c_void,
        (*dyd).actvar.n + 1 as i32,
        &mut (*dyd).actvar.size,
        ::core::mem::size_of::<Vardesc>() as libc::c_ulong as i32,
        (if 32767 as i32 as size_t
            <= (!(0 as i32 as size_t))
                .wrapping_div(::core::mem::size_of::<Vardesc>() as libc::c_ulong)
        {
            32767 as i32 as libc::c_uint
        } else {
            (!(0 as i32 as size_t))
                .wrapping_div(::core::mem::size_of::<Vardesc>() as libc::c_ulong)
                as libc::c_uint
        }) as i32,
        b"local variables\0" as *const u8 as *const libc::c_char,
    ) as *mut Vardesc;
    let fresh4 = (*dyd).actvar.n;
    (*dyd).actvar.n = (*dyd).actvar.n + 1;
    var = &mut *((*dyd).actvar.arr).offset(fresh4 as isize) as *mut Vardesc;
    (*var).vd.kind = 0 as i32 as u8;
    (*var).vd.name = name;
    return (*dyd).actvar.n - 1 as i32 - (*fs).firstlocal;
}
unsafe extern "C" fn getlocalvardesc(
    mut fs: *mut FuncState,
    mut vidx: i32,
) -> *mut Vardesc {
    return &mut *((*(*(*fs).ls).dyd).actvar.arr)
        .offset(((*fs).firstlocal + vidx) as isize) as *mut Vardesc;
}
unsafe extern "C" fn reglevel(
    mut fs: *mut FuncState,
    mut nvar: i32,
) -> i32 {
    loop {
        let fresh5 = nvar;
        nvar = nvar - 1;
        if !(fresh5 > 0 as i32) {
            break;
        }
        let mut vd: *mut Vardesc = getlocalvardesc(fs, nvar);
        if (*vd).vd.kind as i32 != 3 as i32 {
            return (*vd).vd.ridx as i32 + 1 as i32;
        }
    }
    return 0 as i32;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaY_nvarstack(mut fs: *mut FuncState) -> i32 {
    return reglevel(fs, (*fs).nactvar as i32);
}
unsafe extern "C" fn localdebuginfo(
    mut fs: *mut FuncState,
    mut vidx: i32,
) -> *mut LocVar {
    let mut vd: *mut Vardesc = getlocalvardesc(fs, vidx);
    if (*vd).vd.kind as i32 == 3 as i32 {
        return 0 as *mut LocVar
    } else {
        let mut index: i32 = (*vd).vd.pidx as i32;
        return &mut *((*(*fs).f).locvars).offset(index as isize) as *mut LocVar;
    };
}
unsafe extern "C" fn init_var(
    mut fs: *mut FuncState,
    mut e: *mut expdesc,
    mut vidx: i32,
) {
    (*e).t = -(1 as i32);
    (*e).f = (*e).t;
    (*e).k = VLOCAL;
    (*e).u.var.vidx = vidx as libc::c_ushort;
    (*e).u.var.ridx = (*getlocalvardesc(fs, vidx)).vd.ridx;
}
unsafe extern "C" fn check_readonly(mut ls: *mut LexState, mut e: *mut expdesc) {
    let mut fs: *mut FuncState = (*ls).fs;
    let mut varname: *mut TString = 0 as *mut TString;
    match (*e).k as libc::c_uint {
        11 => {
            varname = (*((*(*ls).dyd).actvar.arr).offset((*e).u.info as isize)).vd.name;
        }
        9 => {
            let mut vardesc: *mut Vardesc = getlocalvardesc(
                fs,
                (*e).u.var.vidx as i32,
            );
            if (*vardesc).vd.kind as i32 != 0 as i32 {
                varname = (*vardesc).vd.name;
            }
        }
        10 => {
            let mut up: *mut Upvaldesc = &mut *((*(*fs).f).upvalues)
                .offset((*e).u.info as isize) as *mut Upvaldesc;
            if (*up).kind as i32 != 0 as i32 {
                varname = (*up).name;
            }
        }
        _ => return,
    }
    if !varname.is_null() {
        let mut msg: *const libc::c_char = luaO_pushfstring(
            (*ls).L,
            b"attempt to assign to const variable '%s'\0" as *const u8
                as *const libc::c_char,
            ((*varname).contents).as_mut_ptr(),
        );
        luaK_semerror(ls, msg);
    }
}
unsafe extern "C" fn adjustlocalvars(mut ls: *mut LexState, mut nvars: i32) {
    let mut fs: *mut FuncState = (*ls).fs;
    let mut reglevel_0: i32 = luaY_nvarstack(fs);
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < nvars {
        let fresh6 = (*fs).nactvar;
        (*fs).nactvar = ((*fs).nactvar).wrapping_add(1);
        let mut vidx: i32 = fresh6 as i32;
        let mut var: *mut Vardesc = getlocalvardesc(fs, vidx);
        let fresh7 = reglevel_0;
        reglevel_0 = reglevel_0 + 1;
        (*var).vd.ridx = fresh7 as u8;
        (*var).vd.pidx = registerlocalvar(ls, fs, (*var).vd.name) as libc::c_short;
        i += 1;
        i;
    }
}
unsafe extern "C" fn removevars(mut fs: *mut FuncState, mut tolevel: i32) {
    (*(*(*fs).ls).dyd).actvar.n -= (*fs).nactvar as i32 - tolevel;
    while (*fs).nactvar as i32 > tolevel {
        (*fs).nactvar = ((*fs).nactvar).wrapping_sub(1);
        let mut var: *mut LocVar = localdebuginfo(fs, (*fs).nactvar as i32);
        if !var.is_null() {
            (*var).endpc = (*fs).pc;
        }
    }
}
unsafe extern "C" fn searchupvalue(
    mut fs: *mut FuncState,
    mut name: *mut TString,
) -> i32 {
    let mut i: i32 = 0;
    let mut up: *mut Upvaldesc = (*(*fs).f).upvalues;
    i = 0 as i32;
    while i < (*fs).nups as i32 {
        if (*up.offset(i as isize)).name == name {
            return i;
        }
        i += 1;
        i;
    }
    return -(1 as i32);
}
unsafe extern "C" fn allocupvalue(mut fs: *mut FuncState) -> *mut Upvaldesc {
    let mut f: *mut Proto = (*fs).f;
    let mut oldsize: i32 = (*f).sizeupvalues;
    checklimit(
        fs,
        (*fs).nups as i32 + 1 as i32,
        255 as i32,
        b"upvalues\0" as *const u8 as *const libc::c_char,
    );
    (*f)
        .upvalues = luaM_growaux_(
        (*(*fs).ls).L,
        (*f).upvalues as *mut libc::c_void,
        (*fs).nups as i32,
        &mut (*f).sizeupvalues,
        ::core::mem::size_of::<Upvaldesc>() as libc::c_ulong as i32,
        (if 255 as i32 as size_t
            <= (!(0 as i32 as size_t))
                .wrapping_div(::core::mem::size_of::<Upvaldesc>() as libc::c_ulong)
        {
            255 as i32 as libc::c_uint
        } else {
            (!(0 as i32 as size_t))
                .wrapping_div(::core::mem::size_of::<Upvaldesc>() as libc::c_ulong)
                as libc::c_uint
        }) as i32,
        b"upvalues\0" as *const u8 as *const libc::c_char,
    ) as *mut Upvaldesc;
    while oldsize < (*f).sizeupvalues {
        let fresh8 = oldsize;
        oldsize = oldsize + 1;
        let ref mut fresh9 = (*((*f).upvalues).offset(fresh8 as isize)).name;
        *fresh9 = 0 as *mut TString;
    }
    let fresh10 = (*fs).nups;
    (*fs).nups = ((*fs).nups).wrapping_add(1);
    return &mut *((*f).upvalues).offset(fresh10 as isize) as *mut Upvaldesc;
}
unsafe extern "C" fn newupvalue(
    mut fs: *mut FuncState,
    mut name: *mut TString,
    mut v: *mut expdesc,
) -> i32 {
    let mut up: *mut Upvaldesc = allocupvalue(fs);
    let mut prev: *mut FuncState = (*fs).prev;
    if (*v).k as libc::c_uint == VLOCAL as i32 as libc::c_uint {
        (*up).instack = 1 as i32 as u8;
        (*up).index = (*v).u.var.ridx;
        (*up).kind = (*getlocalvardesc(prev, (*v).u.var.vidx as i32)).vd.kind;
    } else {
        (*up).instack = 0 as i32 as u8;
        (*up).index = (*v).u.info as u8;
        (*up).kind = (*((*(*prev).f).upvalues).offset((*v).u.info as isize)).kind;
    }
    (*up).name = name;
    if (*(*fs).f).marked as i32 & (1 as i32) << 5 as i32 != 0
        && (*name).marked as i32
            & ((1 as i32) << 3 as i32
                | (1 as i32) << 4 as i32) != 0
    {
        luaC_barrier_(
            (*(*fs).ls).L,
            &mut (*((*fs).f as *mut GCUnion)).gc,
            &mut (*(name as *mut GCUnion)).gc,
        );
    } else {};
    return (*fs).nups as i32 - 1 as i32;
}
unsafe extern "C" fn searchvar(
    mut fs: *mut FuncState,
    mut n: *mut TString,
    mut var: *mut expdesc,
) -> i32 {
    let mut i: i32 = 0;
    i = (*fs).nactvar as i32 - 1 as i32;
    while i >= 0 as i32 {
        let mut vd: *mut Vardesc = getlocalvardesc(fs, i);
        if n == (*vd).vd.name {
            if (*vd).vd.kind as i32 == 3 as i32 {
                init_exp(var, VCONST, (*fs).firstlocal + i);
            } else {
                init_var(fs, var, i);
            }
            return (*var).k as i32;
        }
        i -= 1;
        i;
    }
    return -(1 as i32);
}
unsafe extern "C" fn markupval(mut fs: *mut FuncState, mut level: i32) {
    let mut bl: *mut BlockCnt = (*fs).bl;
    while (*bl).nactvar as i32 > level {
        bl = (*bl).previous;
    }
    (*bl).upval = 1 as i32 as u8;
    (*fs).needclose = 1 as i32 as u8;
}
unsafe extern "C" fn marktobeclosed(mut fs: *mut FuncState) {
    let mut bl: *mut BlockCnt = (*fs).bl;
    (*bl).upval = 1 as i32 as u8;
    (*bl).insidetbc = 1 as i32 as u8;
    (*fs).needclose = 1 as i32 as u8;
}
unsafe extern "C" fn singlevaraux(
    mut fs: *mut FuncState,
    mut n: *mut TString,
    mut var: *mut expdesc,
    mut base: i32,
) {
    if fs.is_null() {
        init_exp(var, VVOID, 0 as i32);
    } else {
        let mut v: i32 = searchvar(fs, n, var);
        if v >= 0 as i32 {
            if v == VLOCAL as i32 && base == 0 {
                markupval(fs, (*var).u.var.vidx as i32);
            }
        } else {
            let mut index: i32 = searchupvalue(fs, n);
            if index < 0 as i32 {
                singlevaraux((*fs).prev, n, var, 0 as i32);
                if (*var).k as libc::c_uint == VLOCAL as i32 as libc::c_uint
                    || (*var).k as libc::c_uint == VUPVAL as i32 as libc::c_uint
                {
                    index = newupvalue(fs, n, var);
                } else {
                    return
                }
            }
            init_exp(var, VUPVAL, index);
        }
    };
}
unsafe extern "C" fn singlevar(mut ls: *mut LexState, mut var: *mut expdesc) {
    let mut varname: *mut TString = str_checkname(ls);
    let mut fs: *mut FuncState = (*ls).fs;
    singlevaraux(fs, varname, var, 1 as i32);
    if (*var).k as libc::c_uint == VVOID as i32 as libc::c_uint {
        let mut key: expdesc = expdesc {
            k: VVOID,
            u: C2RustUnnamed_11 { ival: 0 },
            t: 0,
            f: 0,
        };
        singlevaraux(fs, (*ls).envn, var, 1 as i32);
        luaK_exp2anyregup(fs, var);
        codestring(&mut key, varname);
        luaK_indexed(fs, var, &mut key);
    }
}
unsafe extern "C" fn adjust_assign(
    mut ls: *mut LexState,
    mut nvars: i32,
    mut nexps: i32,
    mut e: *mut expdesc,
) {
    let mut fs: *mut FuncState = (*ls).fs;
    let mut needed: i32 = nvars - nexps;
    if (*e).k as libc::c_uint == VCALL as i32 as libc::c_uint
        || (*e).k as libc::c_uint == VVARARG as i32 as libc::c_uint
    {
        let mut extra: i32 = needed + 1 as i32;
        if extra < 0 as i32 {
            extra = 0 as i32;
        }
        luaK_setreturns(fs, e, extra);
    } else {
        if (*e).k as libc::c_uint != VVOID as i32 as libc::c_uint {
            luaK_exp2nextreg(fs, e);
        }
        if needed > 0 as i32 {
            luaK_nil(fs, (*fs).freereg as i32, needed);
        }
    }
    if needed > 0 as i32 {
        luaK_reserveregs(fs, needed);
    } else {
        (*fs).freereg = ((*fs).freereg as i32 + needed) as u8;
    };
}
unsafe extern "C" fn jumpscopeerror(mut ls: *mut LexState, mut gt: *mut Labeldesc) -> ! {
    let mut varname: *const libc::c_char = ((*(*getlocalvardesc(
        (*ls).fs,
        (*gt).nactvar as i32,
    ))
        .vd
        .name)
        .contents)
        .as_mut_ptr();
    let mut msg: *const libc::c_char = b"<goto %s> at line %d jumps into the scope of local '%s'\0"
        as *const u8 as *const libc::c_char;
    msg = luaO_pushfstring(
        (*ls).L,
        msg,
        ((*(*gt).name).contents).as_mut_ptr(),
        (*gt).line,
        varname,
    );
    luaK_semerror(ls, msg);
}
unsafe extern "C" fn solvegoto(
    mut ls: *mut LexState,
    mut g: i32,
    mut label: *mut Labeldesc,
) {
    let mut i: i32 = 0;
    let mut gl: *mut Labellist = &mut (*(*ls).dyd).gt;
    let mut gt: *mut Labeldesc = &mut *((*gl).arr).offset(g as isize) as *mut Labeldesc;
    if ((((*gt).nactvar as i32) < (*label).nactvar as i32) as i32
        != 0 as i32) as i32 as libc::c_long != 0
    {
        jumpscopeerror(ls, gt);
    }
    luaK_patchlist((*ls).fs, (*gt).pc, (*label).pc);
    i = g;
    while i < (*gl).n - 1 as i32 {
        *((*gl).arr)
            .offset(i as isize) = *((*gl).arr).offset((i + 1 as i32) as isize);
        i += 1;
        i;
    }
    (*gl).n -= 1;
    (*gl).n;
}
unsafe extern "C" fn findlabel(
    mut ls: *mut LexState,
    mut name: *mut TString,
) -> *mut Labeldesc {
    let mut i: i32 = 0;
    let mut dyd: *mut Dyndata = (*ls).dyd;
    i = (*(*ls).fs).firstlabel;
    while i < (*dyd).label.n {
        let mut lb: *mut Labeldesc = &mut *((*dyd).label.arr).offset(i as isize)
            as *mut Labeldesc;
        if (*lb).name == name {
            return lb;
        }
        i += 1;
        i;
    }
    return 0 as *mut Labeldesc;
}
unsafe extern "C" fn newlabelentry(
    mut ls: *mut LexState,
    mut l: *mut Labellist,
    mut name: *mut TString,
    mut line: i32,
    mut pc: i32,
) -> i32 {
    let mut n: i32 = (*l).n;
    (*l)
        .arr = luaM_growaux_(
        (*ls).L,
        (*l).arr as *mut libc::c_void,
        n,
        &mut (*l).size,
        ::core::mem::size_of::<Labeldesc>() as libc::c_ulong as i32,
        (if 32767 as i32 as size_t
            <= (!(0 as i32 as size_t))
                .wrapping_div(::core::mem::size_of::<Labeldesc>() as libc::c_ulong)
        {
            32767 as i32 as libc::c_uint
        } else {
            (!(0 as i32 as size_t))
                .wrapping_div(::core::mem::size_of::<Labeldesc>() as libc::c_ulong)
                as libc::c_uint
        }) as i32,
        b"labels/gotos\0" as *const u8 as *const libc::c_char,
    ) as *mut Labeldesc;
    let ref mut fresh11 = (*((*l).arr).offset(n as isize)).name;
    *fresh11 = name;
    (*((*l).arr).offset(n as isize)).line = line;
    (*((*l).arr).offset(n as isize)).nactvar = (*(*ls).fs).nactvar;
    (*((*l).arr).offset(n as isize)).close = 0 as i32 as u8;
    (*((*l).arr).offset(n as isize)).pc = pc;
    (*l).n = n + 1 as i32;
    return n;
}
unsafe extern "C" fn newgotoentry(
    mut ls: *mut LexState,
    mut name: *mut TString,
    mut line: i32,
    mut pc: i32,
) -> i32 {
    return newlabelentry(ls, &mut (*(*ls).dyd).gt, name, line, pc);
}
unsafe extern "C" fn solvegotos(
    mut ls: *mut LexState,
    mut lb: *mut Labeldesc,
) -> i32 {
    let mut gl: *mut Labellist = &mut (*(*ls).dyd).gt;
    let mut i: i32 = (*(*(*ls).fs).bl).firstgoto;
    let mut needsclose: i32 = 0 as i32;
    while i < (*gl).n {
        if (*((*gl).arr).offset(i as isize)).name == (*lb).name {
            needsclose |= (*((*gl).arr).offset(i as isize)).close as i32;
            solvegoto(ls, i, lb);
        } else {
            i += 1;
            i;
        }
    }
    return needsclose;
}
unsafe extern "C" fn createlabel(
    mut ls: *mut LexState,
    mut name: *mut TString,
    mut line: i32,
    mut last: i32,
) -> i32 {
    let mut fs: *mut FuncState = (*ls).fs;
    let mut ll: *mut Labellist = &mut (*(*ls).dyd).label;
    let mut l: i32 = newlabelentry(ls, ll, name, line, luaK_getlabel(fs));
    if last != 0 {
        (*((*ll).arr).offset(l as isize)).nactvar = (*(*fs).bl).nactvar;
    }
    if solvegotos(ls, &mut *((*ll).arr).offset(l as isize)) != 0 {
        luaK_codeABCk(
            fs,
            OP_CLOSE,
            luaY_nvarstack(fs),
            0 as i32,
            0 as i32,
            0 as i32,
        );
        return 1 as i32;
    }
    return 0 as i32;
}
unsafe extern "C" fn movegotosout(mut fs: *mut FuncState, mut bl: *mut BlockCnt) {
    let mut i: i32 = 0;
    let mut gl: *mut Labellist = &mut (*(*(*fs).ls).dyd).gt;
    i = (*bl).firstgoto;
    while i < (*gl).n {
        let mut gt: *mut Labeldesc = &mut *((*gl).arr).offset(i as isize)
            as *mut Labeldesc;
        if reglevel(fs, (*gt).nactvar as i32)
            > reglevel(fs, (*bl).nactvar as i32)
        {
            (*gt)
                .close = ((*gt).close as i32 | (*bl).upval as i32)
                as u8;
        }
        (*gt).nactvar = (*bl).nactvar;
        i += 1;
        i;
    }
}
unsafe extern "C" fn enterblock(
    mut fs: *mut FuncState,
    mut bl: *mut BlockCnt,
    mut isloop: u8,
) {
    (*bl).isloop = isloop;
    (*bl).nactvar = (*fs).nactvar;
    (*bl).firstlabel = (*(*(*fs).ls).dyd).label.n;
    (*bl).firstgoto = (*(*(*fs).ls).dyd).gt.n;
    (*bl).upval = 0 as i32 as u8;
    (*bl)
        .insidetbc = (!((*fs).bl).is_null() && (*(*fs).bl).insidetbc as i32 != 0)
        as i32 as u8;
    (*bl).previous = (*fs).bl;
    (*fs).bl = bl;
}
unsafe extern "C" fn undefgoto(mut ls: *mut LexState, mut gt: *mut Labeldesc) -> ! {
    let mut msg: *const libc::c_char = 0 as *const libc::c_char;
    if (*gt).name
        == luaS_newlstr(
            (*ls).L,
            b"break\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_sub(1 as i32 as libc::c_ulong),
        )
    {
        msg = b"break outside loop at line %d\0" as *const u8 as *const libc::c_char;
        msg = luaO_pushfstring((*ls).L, msg, (*gt).line);
    } else {
        msg = b"no visible label '%s' for <goto> at line %d\0" as *const u8
            as *const libc::c_char;
        msg = luaO_pushfstring(
            (*ls).L,
            msg,
            ((*(*gt).name).contents).as_mut_ptr(),
            (*gt).line,
        );
    }
    luaK_semerror(ls, msg);
}
unsafe extern "C" fn leaveblock(mut fs: *mut FuncState) {
    let mut bl: *mut BlockCnt = (*fs).bl;
    let mut ls: *mut LexState = (*fs).ls;
    let mut hasclose: i32 = 0 as i32;
    let mut stklevel: i32 = reglevel(fs, (*bl).nactvar as i32);
    removevars(fs, (*bl).nactvar as i32);
    if (*bl).isloop != 0 {
        hasclose = createlabel(
            ls,
            luaS_newlstr(
                (*ls).L,
                b"break\0" as *const u8 as *const libc::c_char,
                (::core::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                    )
                    .wrapping_sub(1 as i32 as libc::c_ulong),
            ),
            0 as i32,
            0 as i32,
        );
    }
    if hasclose == 0 && !((*bl).previous).is_null() && (*bl).upval as i32 != 0 {
        luaK_codeABCk(
            fs,
            OP_CLOSE,
            stklevel,
            0 as i32,
            0 as i32,
            0 as i32,
        );
    }
    (*fs).freereg = stklevel as u8;
    (*(*ls).dyd).label.n = (*bl).firstlabel;
    (*fs).bl = (*bl).previous;
    if !((*bl).previous).is_null() {
        movegotosout(fs, bl);
    } else if (*bl).firstgoto < (*(*ls).dyd).gt.n {
        undefgoto(ls, &mut *((*(*ls).dyd).gt.arr).offset((*bl).firstgoto as isize));
    }
}
unsafe extern "C" fn addprototype(mut ls: *mut LexState) -> *mut Proto {
    let mut clp: *mut Proto = 0 as *mut Proto;
    let mut L: *mut lua_State = (*ls).L;
    let mut fs: *mut FuncState = (*ls).fs;
    let mut f: *mut Proto = (*fs).f;
    if (*fs).np >= (*f).sizep {
        let mut oldsize: i32 = (*f).sizep;
        (*f)
            .p = luaM_growaux_(
            L,
            (*f).p as *mut libc::c_void,
            (*fs).np,
            &mut (*f).sizep,
            ::core::mem::size_of::<*mut Proto>() as libc::c_ulong as i32,
            (if (((1 as i32)
                << 8 as i32 + 8 as i32 + 1 as i32)
                - 1 as i32) as size_t
                <= (!(0 as i32 as size_t))
                    .wrapping_div(::core::mem::size_of::<*mut Proto>() as libc::c_ulong)
            {
                (((1 as i32)
                    << 8 as i32 + 8 as i32 + 1 as i32)
                    - 1 as i32) as libc::c_uint
            } else {
                (!(0 as i32 as size_t))
                    .wrapping_div(::core::mem::size_of::<*mut Proto>() as libc::c_ulong)
                    as libc::c_uint
            }) as i32,
            b"functions\0" as *const u8 as *const libc::c_char,
        ) as *mut *mut Proto;
        while oldsize < (*f).sizep {
            let fresh12 = oldsize;
            oldsize = oldsize + 1;
            let ref mut fresh13 = *((*f).p).offset(fresh12 as isize);
            *fresh13 = 0 as *mut Proto;
        }
    }
    clp = luaF_newproto(L);
    let fresh14 = (*fs).np;
    (*fs).np = (*fs).np + 1;
    let ref mut fresh15 = *((*f).p).offset(fresh14 as isize);
    *fresh15 = clp;
    if (*f).marked as i32 & (1 as i32) << 5 as i32 != 0
        && (*clp).marked as i32
            & ((1 as i32) << 3 as i32
                | (1 as i32) << 4 as i32) != 0
    {
        luaC_barrier_(
            L,
            &mut (*(f as *mut GCUnion)).gc,
            &mut (*(clp as *mut GCUnion)).gc,
        );
    } else {};
    return clp;
}
unsafe extern "C" fn codeclosure(mut ls: *mut LexState, mut v: *mut expdesc) {
    let mut fs: *mut FuncState = (*(*ls).fs).prev;
    init_exp(
        v,
        VRELOC,
        luaK_codeABx(
            fs,
            OP_CLOSURE,
            0 as i32,
            ((*fs).np - 1 as i32) as libc::c_uint,
        ),
    );
    luaK_exp2nextreg(fs, v);
}
unsafe extern "C" fn open_func(
    mut ls: *mut LexState,
    mut fs: *mut FuncState,
    mut bl: *mut BlockCnt,
) {
    let mut f: *mut Proto = (*fs).f;
    (*fs).prev = (*ls).fs;
    (*fs).ls = ls;
    (*ls).fs = fs;
    (*fs).pc = 0 as i32;
    (*fs).previousline = (*f).linedefined;
    (*fs).iwthabs = 0 as i32 as u8;
    (*fs).lasttarget = 0 as i32;
    (*fs).freereg = 0 as i32 as u8;
    (*fs).nk = 0 as i32;
    (*fs).nabslineinfo = 0 as i32;
    (*fs).np = 0 as i32;
    (*fs).nups = 0 as i32 as u8;
    (*fs).ndebugvars = 0 as i32 as libc::c_short;
    (*fs).nactvar = 0 as i32 as u8;
    (*fs).needclose = 0 as i32 as u8;
    (*fs).firstlocal = (*(*ls).dyd).actvar.n;
    (*fs).firstlabel = (*(*ls).dyd).label.n;
    (*fs).bl = 0 as *mut BlockCnt;
    (*f).source = (*ls).source;
    if (*f).marked as i32 & (1 as i32) << 5 as i32 != 0
        && (*(*f).source).marked as i32
            & ((1 as i32) << 3 as i32
                | (1 as i32) << 4 as i32) != 0
    {
        luaC_barrier_(
            (*ls).L,
            &mut (*(f as *mut GCUnion)).gc,
            &mut (*((*f).source as *mut GCUnion)).gc,
        );
    } else {};
    (*f).maxstacksize = 2 as i32 as u8;
    enterblock(fs, bl, 0 as i32 as u8);
}
unsafe extern "C" fn close_func(mut ls: *mut LexState) {
    let mut L: *mut lua_State = (*ls).L;
    let mut fs: *mut FuncState = (*ls).fs;
    let mut f: *mut Proto = (*fs).f;
    luaK_ret(fs, luaY_nvarstack(fs), 0 as i32);
    leaveblock(fs);
    luaK_finish(fs);
    (*f)
        .code = luaM_shrinkvector_(
        L,
        (*f).code as *mut libc::c_void,
        &mut (*f).sizecode,
        (*fs).pc,
        ::core::mem::size_of::<Instruction>() as libc::c_ulong as i32,
    ) as *mut Instruction;
    (*f)
        .lineinfo = luaM_shrinkvector_(
        L,
        (*f).lineinfo as *mut libc::c_void,
        &mut (*f).sizelineinfo,
        (*fs).pc,
        ::core::mem::size_of::<ls_byte>() as libc::c_ulong as i32,
    ) as *mut ls_byte;
    (*f)
        .abslineinfo = luaM_shrinkvector_(
        L,
        (*f).abslineinfo as *mut libc::c_void,
        &mut (*f).sizeabslineinfo,
        (*fs).nabslineinfo,
        ::core::mem::size_of::<AbsLineInfo>() as libc::c_ulong as i32,
    ) as *mut AbsLineInfo;
    (*f)
        .k = luaM_shrinkvector_(
        L,
        (*f).k as *mut libc::c_void,
        &mut (*f).sizek,
        (*fs).nk,
        ::core::mem::size_of::<TValue>() as libc::c_ulong as i32,
    ) as *mut TValue;
    (*f)
        .p = luaM_shrinkvector_(
        L,
        (*f).p as *mut libc::c_void,
        &mut (*f).sizep,
        (*fs).np,
        ::core::mem::size_of::<*mut Proto>() as libc::c_ulong as i32,
    ) as *mut *mut Proto;
    (*f)
        .locvars = luaM_shrinkvector_(
        L,
        (*f).locvars as *mut libc::c_void,
        &mut (*f).sizelocvars,
        (*fs).ndebugvars as i32,
        ::core::mem::size_of::<LocVar>() as libc::c_ulong as i32,
    ) as *mut LocVar;
    (*f)
        .upvalues = luaM_shrinkvector_(
        L,
        (*f).upvalues as *mut libc::c_void,
        &mut (*f).sizeupvalues,
        (*fs).nups as i32,
        ::core::mem::size_of::<Upvaldesc>() as libc::c_ulong as i32,
    ) as *mut Upvaldesc;
    (*ls).fs = (*fs).prev;
    if (*(*L).l_G).GCdebt > 0 as i32 as libc::c_long {
        luaC_step(L);
    }
}
unsafe extern "C" fn block_follow(
    mut ls: *mut LexState,
    mut withuntil: i32,
) -> i32 {
    match (*ls).t.token {
        259 | 260 | 261 | 288 => return 1 as i32,
        276 => return withuntil,
        _ => return 0 as i32,
    };
}
unsafe extern "C" fn statlist(mut ls: *mut LexState) {
    while block_follow(ls, 1 as i32) == 0 {
        if (*ls).t.token == TK_RETURN as i32 {
            statement(ls);
            return;
        }
        statement(ls);
    }
}
unsafe extern "C" fn fieldsel(mut ls: *mut LexState, mut v: *mut expdesc) {
    let mut fs: *mut FuncState = (*ls).fs;
    let mut key: expdesc = expdesc {
        k: VVOID,
        u: C2RustUnnamed_11 { ival: 0 },
        t: 0,
        f: 0,
    };
    luaK_exp2anyregup(fs, v);
    luaX_next(ls);
    codename(ls, &mut key);
    luaK_indexed(fs, v, &mut key);
}
unsafe extern "C" fn yindex(mut ls: *mut LexState, mut v: *mut expdesc) {
    luaX_next(ls);
    expr(ls, v);
    luaK_exp2val((*ls).fs, v);
    checknext(ls, ']' as i32);
}
unsafe extern "C" fn recfield(mut ls: *mut LexState, mut cc: *mut ConsControl) {
    let mut fs: *mut FuncState = (*ls).fs;
    let mut reg: i32 = (*(*ls).fs).freereg as i32;
    let mut tab: expdesc = expdesc {
        k: VVOID,
        u: C2RustUnnamed_11 { ival: 0 },
        t: 0,
        f: 0,
    };
    let mut key: expdesc = expdesc {
        k: VVOID,
        u: C2RustUnnamed_11 { ival: 0 },
        t: 0,
        f: 0,
    };
    let mut val: expdesc = expdesc {
        k: VVOID,
        u: C2RustUnnamed_11 { ival: 0 },
        t: 0,
        f: 0,
    };
    if (*ls).t.token == TK_NAME as i32 {
        codename(ls, &mut key);
    } else {
        yindex(ls, &mut key);
    }
    checklimit(
        fs,
        (*cc).nh,
        2147483647 as i32,
        b"items in a constructor\0" as *const u8 as *const libc::c_char,
    );
    (*cc).nh += 1;
    (*cc).nh;
    checknext(ls, '=' as i32);
    tab = *(*cc).t;
    luaK_indexed(fs, &mut tab, &mut key);
    expr(ls, &mut val);
    luaK_storevar(fs, &mut tab, &mut val);
    (*fs).freereg = reg as u8;
}
unsafe extern "C" fn closelistfield(mut fs: *mut FuncState, mut cc: *mut ConsControl) {
    if (*cc).v.k as libc::c_uint == VVOID as i32 as libc::c_uint {
        return;
    }
    luaK_exp2nextreg(fs, &mut (*cc).v);
    (*cc).v.k = VVOID;
    if (*cc).tostore == 50 as i32 {
        luaK_setlist(fs, (*(*cc).t).u.info, (*cc).na, (*cc).tostore);
        (*cc).na += (*cc).tostore;
        (*cc).tostore = 0 as i32;
    }
}
unsafe extern "C" fn lastlistfield(mut fs: *mut FuncState, mut cc: *mut ConsControl) {
    if (*cc).tostore == 0 as i32 {
        return;
    }
    if (*cc).v.k as libc::c_uint == VCALL as i32 as libc::c_uint
        || (*cc).v.k as libc::c_uint == VVARARG as i32 as libc::c_uint
    {
        luaK_setreturns(fs, &mut (*cc).v, -(1 as i32));
        luaK_setlist(fs, (*(*cc).t).u.info, (*cc).na, -(1 as i32));
        (*cc).na -= 1;
        (*cc).na;
    } else {
        if (*cc).v.k as libc::c_uint != VVOID as i32 as libc::c_uint {
            luaK_exp2nextreg(fs, &mut (*cc).v);
        }
        luaK_setlist(fs, (*(*cc).t).u.info, (*cc).na, (*cc).tostore);
    }
    (*cc).na += (*cc).tostore;
}
unsafe extern "C" fn listfield(mut ls: *mut LexState, mut cc: *mut ConsControl) {
    expr(ls, &mut (*cc).v);
    (*cc).tostore += 1;
    (*cc).tostore;
}
unsafe extern "C" fn field(mut ls: *mut LexState, mut cc: *mut ConsControl) {
    match (*ls).t.token {
        291 => {
            if luaX_lookahead(ls) != '=' as i32 {
                listfield(ls, cc);
            } else {
                recfield(ls, cc);
            }
        }
        91 => {
            recfield(ls, cc);
        }
        _ => {
            listfield(ls, cc);
        }
    };
}
unsafe extern "C" fn constructor(mut ls: *mut LexState, mut t: *mut expdesc) {
    let mut fs: *mut FuncState = (*ls).fs;
    let mut line: i32 = (*ls).linenumber;
    let mut pc: i32 = luaK_codeABCk(
        fs,
        OP_NEWTABLE,
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
    );
    let mut cc: ConsControl = ConsControl {
        v: expdesc {
            k: VVOID,
            u: C2RustUnnamed_11 { ival: 0 },
            t: 0,
            f: 0,
        },
        t: 0 as *mut expdesc,
        nh: 0,
        na: 0,
        tostore: 0,
    };
    luaK_code(fs, 0 as i32 as Instruction);
    cc.tostore = 0 as i32;
    cc.nh = cc.tostore;
    cc.na = cc.nh;
    cc.t = t;
    init_exp(t, VNONRELOC, (*fs).freereg as i32);
    luaK_reserveregs(fs, 1 as i32);
    init_exp(&mut cc.v, VVOID, 0 as i32);
    checknext(ls, '{' as i32);
    while !((*ls).t.token == '}' as i32) {
        closelistfield(fs, &mut cc);
        field(ls, &mut cc);
        if !(testnext(ls, ',' as i32) != 0 || testnext(ls, ';' as i32) != 0) {
            break;
        }
    }
    check_match(ls, '}' as i32, '{' as i32, line);
    lastlistfield(fs, &mut cc);
    luaK_settablesize(fs, pc, (*t).u.info, cc.na, cc.nh);
}
unsafe extern "C" fn setvararg(mut fs: *mut FuncState, mut nparams: i32) {
    (*(*fs).f).is_vararg = 1 as i32 as u8;
    luaK_codeABCk(
        fs,
        OP_VARARGPREP,
        nparams,
        0 as i32,
        0 as i32,
        0 as i32,
    );
}
unsafe extern "C" fn parlist(mut ls: *mut LexState) {
    let mut fs: *mut FuncState = (*ls).fs;
    let mut f: *mut Proto = (*fs).f;
    let mut nparams: i32 = 0 as i32;
    let mut isvararg: i32 = 0 as i32;
    if (*ls).t.token != ')' as i32 {
        loop {
            match (*ls).t.token {
                291 => {
                    new_localvar(ls, str_checkname(ls));
                    nparams += 1;
                    nparams;
                }
                280 => {
                    luaX_next(ls);
                    isvararg = 1 as i32;
                }
                _ => {
                    luaX_syntaxerror(
                        ls,
                        b"<name> or '...' expected\0" as *const u8 as *const libc::c_char,
                    );
                }
            }
            if !(isvararg == 0 && testnext(ls, ',' as i32) != 0) {
                break;
            }
        }
    }
    adjustlocalvars(ls, nparams);
    (*f).numparams = (*fs).nactvar;
    if isvararg != 0 {
        setvararg(fs, (*f).numparams as i32);
    }
    luaK_reserveregs(fs, (*fs).nactvar as i32);
}
unsafe extern "C" fn body(
    mut ls: *mut LexState,
    mut e: *mut expdesc,
    mut ismethod: i32,
    mut line: i32,
) {
    let mut new_fs: FuncState = FuncState {
        f: 0 as *mut Proto,
        prev: 0 as *mut FuncState,
        ls: 0 as *mut LexState,
        bl: 0 as *mut BlockCnt,
        pc: 0,
        lasttarget: 0,
        previousline: 0,
        nk: 0,
        np: 0,
        nabslineinfo: 0,
        firstlocal: 0,
        firstlabel: 0,
        ndebugvars: 0,
        nactvar: 0,
        nups: 0,
        freereg: 0,
        iwthabs: 0,
        needclose: 0,
    };
    let mut bl: BlockCnt = BlockCnt {
        previous: 0 as *mut BlockCnt,
        firstlabel: 0,
        firstgoto: 0,
        nactvar: 0,
        upval: 0,
        isloop: 0,
        insidetbc: 0,
    };
    new_fs.f = addprototype(ls);
    (*new_fs.f).linedefined = line;
    open_func(ls, &mut new_fs, &mut bl);
    checknext(ls, '(' as i32);
    if ismethod != 0 {
        new_localvar(
            ls,
            luaX_newstring(
                ls,
                b"self\0" as *const u8 as *const libc::c_char,
                (::core::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                    )
                    .wrapping_sub(1 as i32 as libc::c_ulong),
            ),
        );
        adjustlocalvars(ls, 1 as i32);
    }
    parlist(ls);
    checknext(ls, ')' as i32);
    statlist(ls);
    (*new_fs.f).lastlinedefined = (*ls).linenumber;
    check_match(ls, TK_END as i32, TK_FUNCTION as i32, line);
    codeclosure(ls, e);
    close_func(ls);
}
unsafe extern "C" fn explist(mut ls: *mut LexState, mut v: *mut expdesc) -> i32 {
    let mut n: i32 = 1 as i32;
    expr(ls, v);
    while testnext(ls, ',' as i32) != 0 {
        luaK_exp2nextreg((*ls).fs, v);
        expr(ls, v);
        n += 1;
        n;
    }
    return n;
}
unsafe extern "C" fn funcargs(mut ls: *mut LexState, mut f: *mut expdesc) {
    let mut fs: *mut FuncState = (*ls).fs;
    let mut args: expdesc = expdesc {
        k: VVOID,
        u: C2RustUnnamed_11 { ival: 0 },
        t: 0,
        f: 0,
    };
    let mut base: i32 = 0;
    let mut nparams: i32 = 0;
    let mut line: i32 = (*ls).linenumber;
    match (*ls).t.token {
        40 => {
            luaX_next(ls);
            if (*ls).t.token == ')' as i32 {
                args.k = VVOID;
            } else {
                explist(ls, &mut args);
                if args.k as libc::c_uint == VCALL as i32 as libc::c_uint
                    || args.k as libc::c_uint == VVARARG as i32 as libc::c_uint
                {
                    luaK_setreturns(fs, &mut args, -(1 as i32));
                }
            }
            check_match(ls, ')' as i32, '(' as i32, line);
        }
        123 => {
            constructor(ls, &mut args);
        }
        292 => {
            codestring(&mut args, (*ls).t.seminfo.ts);
            luaX_next(ls);
        }
        _ => {
            luaX_syntaxerror(
                ls,
                b"function arguments expected\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    base = (*f).u.info;
    if args.k as libc::c_uint == VCALL as i32 as libc::c_uint
        || args.k as libc::c_uint == VVARARG as i32 as libc::c_uint
    {
        nparams = -(1 as i32);
    } else {
        if args.k as libc::c_uint != VVOID as i32 as libc::c_uint {
            luaK_exp2nextreg(fs, &mut args);
        }
        nparams = (*fs).freereg as i32 - (base + 1 as i32);
    }
    init_exp(
        f,
        VCALL,
        luaK_codeABCk(
            fs,
            OP_CALL,
            base,
            nparams + 1 as i32,
            2 as i32,
            0 as i32,
        ),
    );
    luaK_fixline(fs, line);
    (*fs).freereg = (base + 1 as i32) as u8;
}
unsafe extern "C" fn primaryexp(mut ls: *mut LexState, mut v: *mut expdesc) {
    match (*ls).t.token {
        40 => {
            let mut line: i32 = (*ls).linenumber;
            luaX_next(ls);
            expr(ls, v);
            check_match(ls, ')' as i32, '(' as i32, line);
            luaK_dischargevars((*ls).fs, v);
            return;
        }
        291 => {
            singlevar(ls, v);
            return;
        }
        _ => {
            luaX_syntaxerror(
                ls,
                b"unexpected symbol\0" as *const u8 as *const libc::c_char,
            );
        }
    };
}
unsafe extern "C" fn suffixedexp(mut ls: *mut LexState, mut v: *mut expdesc) {
    let mut fs: *mut FuncState = (*ls).fs;
    primaryexp(ls, v);
    loop {
        match (*ls).t.token {
            46 => {
                fieldsel(ls, v);
            }
            91 => {
                let mut key: expdesc = expdesc {
                    k: VVOID,
                    u: C2RustUnnamed_11 { ival: 0 },
                    t: 0,
                    f: 0,
                };
                luaK_exp2anyregup(fs, v);
                yindex(ls, &mut key);
                luaK_indexed(fs, v, &mut key);
            }
            58 => {
                let mut key_0: expdesc = expdesc {
                    k: VVOID,
                    u: C2RustUnnamed_11 { ival: 0 },
                    t: 0,
                    f: 0,
                };
                luaX_next(ls);
                codename(ls, &mut key_0);
                luaK_self(fs, v, &mut key_0);
                funcargs(ls, v);
            }
            40 | 292 | 123 => {
                luaK_exp2nextreg(fs, v);
                funcargs(ls, v);
            }
            _ => return,
        }
    };
}
unsafe extern "C" fn simpleexp(mut ls: *mut LexState, mut v: *mut expdesc) {
    match (*ls).t.token {
        289 => {
            init_exp(v, VKFLT, 0 as i32);
            (*v).u.nval = (*ls).t.seminfo.r;
        }
        290 => {
            init_exp(v, VKINT, 0 as i32);
            (*v).u.ival = (*ls).t.seminfo.i;
        }
        292 => {
            codestring(v, (*ls).t.seminfo.ts);
        }
        269 => {
            init_exp(v, VNIL, 0 as i32);
        }
        275 => {
            init_exp(v, VTRUE, 0 as i32);
        }
        262 => {
            init_exp(v, VFALSE, 0 as i32);
        }
        280 => {
            let mut fs: *mut FuncState = (*ls).fs;
            if (*(*fs).f).is_vararg == 0 {
                luaX_syntaxerror(
                    ls,
                    b"cannot use '...' outside a vararg function\0" as *const u8
                        as *const libc::c_char,
                );
            }
            init_exp(
                v,
                VVARARG,
                luaK_codeABCk(
                    fs,
                    OP_VARARG,
                    0 as i32,
                    0 as i32,
                    1 as i32,
                    0 as i32,
                ),
            );
        }
        123 => {
            constructor(ls, v);
            return;
        }
        264 => {
            luaX_next(ls);
            body(ls, v, 0 as i32, (*ls).linenumber);
            return;
        }
        _ => {
            suffixedexp(ls, v);
            return;
        }
    }
    luaX_next(ls);
}
unsafe extern "C" fn getunopr(mut op: i32) -> UnOpr {
    match op {
        270 => return OPR_NOT,
        45 => return OPR_MINUS,
        126 => return OPR_BNOT,
        35 => return OPR_LEN,
        _ => return OPR_NOUNOPR,
    };
}
unsafe extern "C" fn getbinopr(mut op: i32) -> BinOpr {
    match op {
        43 => return OPR_ADD,
        45 => return OPR_SUB,
        42 => return OPR_MUL,
        37 => return OPR_MOD,
        94 => return OPR_POW,
        47 => return OPR_DIV,
        278 => return OPR_IDIV,
        38 => return OPR_BAND,
        124 => return OPR_BOR,
        126 => return OPR_BXOR,
        285 => return OPR_SHL,
        286 => return OPR_SHR,
        279 => return OPR_CONCAT,
        284 => return OPR_NE,
        281 => return OPR_EQ,
        60 => return OPR_LT,
        283 => return OPR_LE,
        62 => return OPR_GT,
        282 => return OPR_GE,
        256 => return OPR_AND,
        271 => return OPR_OR,
        _ => return OPR_NOBINOPR,
    };
}
static mut priority: [C2RustUnnamed_14; 21] = [
    {
        let mut init = C2RustUnnamed_14 {
            left: 10 as i32 as u8,
            right: 10 as i32 as u8,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_14 {
            left: 10 as i32 as u8,
            right: 10 as i32 as u8,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_14 {
            left: 11 as i32 as u8,
            right: 11 as i32 as u8,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_14 {
            left: 11 as i32 as u8,
            right: 11 as i32 as u8,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_14 {
            left: 14 as i32 as u8,
            right: 13 as i32 as u8,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_14 {
            left: 11 as i32 as u8,
            right: 11 as i32 as u8,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_14 {
            left: 11 as i32 as u8,
            right: 11 as i32 as u8,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_14 {
            left: 6 as i32 as u8,
            right: 6 as i32 as u8,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_14 {
            left: 4 as i32 as u8,
            right: 4 as i32 as u8,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_14 {
            left: 5 as i32 as u8,
            right: 5 as i32 as u8,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_14 {
            left: 7 as i32 as u8,
            right: 7 as i32 as u8,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_14 {
            left: 7 as i32 as u8,
            right: 7 as i32 as u8,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_14 {
            left: 9 as i32 as u8,
            right: 8 as i32 as u8,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_14 {
            left: 3 as i32 as u8,
            right: 3 as i32 as u8,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_14 {
            left: 3 as i32 as u8,
            right: 3 as i32 as u8,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_14 {
            left: 3 as i32 as u8,
            right: 3 as i32 as u8,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_14 {
            left: 3 as i32 as u8,
            right: 3 as i32 as u8,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_14 {
            left: 3 as i32 as u8,
            right: 3 as i32 as u8,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_14 {
            left: 3 as i32 as u8,
            right: 3 as i32 as u8,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_14 {
            left: 2 as i32 as u8,
            right: 2 as i32 as u8,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_14 {
            left: 1 as i32 as u8,
            right: 1 as i32 as u8,
        };
        init
    },
];
unsafe extern "C" fn subexpr(
    mut ls: *mut LexState,
    mut v: *mut expdesc,
    mut limit: i32,
) -> BinOpr {
    let mut op: BinOpr = OPR_ADD;
    let mut uop: UnOpr = OPR_MINUS;
    luaE_incCstack((*ls).L);
    uop = getunopr((*ls).t.token);
    if uop as libc::c_uint != OPR_NOUNOPR as i32 as libc::c_uint {
        let mut line: i32 = (*ls).linenumber;
        luaX_next(ls);
        subexpr(ls, v, 12 as i32);
        luaK_prefix((*ls).fs, uop, v, line);
    } else {
        simpleexp(ls, v);
    }
    op = getbinopr((*ls).t.token);
    while op as libc::c_uint != OPR_NOBINOPR as i32 as libc::c_uint
        && priority[op as usize].left as i32 > limit
    {
        let mut v2: expdesc = expdesc {
            k: VVOID,
            u: C2RustUnnamed_11 { ival: 0 },
            t: 0,
            f: 0,
        };
        let mut nextop: BinOpr = OPR_ADD;
        let mut line_0: i32 = (*ls).linenumber;
        luaX_next(ls);
        luaK_infix((*ls).fs, op, v);
        nextop = subexpr(ls, &mut v2, priority[op as usize].right as i32);
        luaK_posfix((*ls).fs, op, v, &mut v2, line_0);
        op = nextop;
    }
    (*(*ls).L).nCcalls = ((*(*ls).L).nCcalls).wrapping_sub(1);
    (*(*ls).L).nCcalls;
    return op;
}
unsafe extern "C" fn expr(mut ls: *mut LexState, mut v: *mut expdesc) {
    subexpr(ls, v, 0 as i32);
}
unsafe extern "C" fn block(mut ls: *mut LexState) {
    let mut fs: *mut FuncState = (*ls).fs;
    let mut bl: BlockCnt = BlockCnt {
        previous: 0 as *mut BlockCnt,
        firstlabel: 0,
        firstgoto: 0,
        nactvar: 0,
        upval: 0,
        isloop: 0,
        insidetbc: 0,
    };
    enterblock(fs, &mut bl, 0 as i32 as u8);
    statlist(ls);
    leaveblock(fs);
}
unsafe extern "C" fn check_conflict(
    mut ls: *mut LexState,
    mut lh: *mut LHS_assign,
    mut v: *mut expdesc,
) {
    let mut fs: *mut FuncState = (*ls).fs;
    let mut extra: i32 = (*fs).freereg as i32;
    let mut conflict: i32 = 0 as i32;
    while !lh.is_null() {
        if VINDEXED as i32 as libc::c_uint <= (*lh).v.k as libc::c_uint
            && (*lh).v.k as libc::c_uint <= VINDEXSTR as i32 as libc::c_uint
        {
            if (*lh).v.k as libc::c_uint == VINDEXUP as i32 as libc::c_uint {
                if (*v).k as libc::c_uint == VUPVAL as i32 as libc::c_uint
                    && (*lh).v.u.ind.t as i32 == (*v).u.info
                {
                    conflict = 1 as i32;
                    (*lh).v.k = VINDEXSTR;
                    (*lh).v.u.ind.t = extra as u8;
                }
            } else {
                if (*v).k as libc::c_uint == VLOCAL as i32 as libc::c_uint
                    && (*lh).v.u.ind.t as i32 == (*v).u.var.ridx as i32
                {
                    conflict = 1 as i32;
                    (*lh).v.u.ind.t = extra as u8;
                }
                if (*lh).v.k as libc::c_uint == VINDEXED as i32 as libc::c_uint
                    && (*v).k as libc::c_uint == VLOCAL as i32 as libc::c_uint
                    && (*lh).v.u.ind.index as i32 == (*v).u.var.ridx as i32
                {
                    conflict = 1 as i32;
                    (*lh).v.u.ind.index = extra as libc::c_short;
                }
            }
        }
        lh = (*lh).prev;
    }
    if conflict != 0 {
        if (*v).k as libc::c_uint == VLOCAL as i32 as libc::c_uint {
            luaK_codeABCk(
                fs,
                OP_MOVE,
                extra,
                (*v).u.var.ridx as i32,
                0 as i32,
                0 as i32,
            );
        } else {
            luaK_codeABCk(
                fs,
                OP_GETUPVAL,
                extra,
                (*v).u.info,
                0 as i32,
                0 as i32,
            );
        }
        luaK_reserveregs(fs, 1 as i32);
    }
}
unsafe extern "C" fn restassign(
    mut ls: *mut LexState,
    mut lh: *mut LHS_assign,
    mut nvars: i32,
) {
    let mut e: expdesc = expdesc {
        k: VVOID,
        u: C2RustUnnamed_11 { ival: 0 },
        t: 0,
        f: 0,
    };
    if !(VLOCAL as i32 as libc::c_uint <= (*lh).v.k as libc::c_uint
        && (*lh).v.k as libc::c_uint <= VINDEXSTR as i32 as libc::c_uint)
    {
        luaX_syntaxerror(ls, b"syntax error\0" as *const u8 as *const libc::c_char);
    }
    check_readonly(ls, &mut (*lh).v);
    if testnext(ls, ',' as i32) != 0 {
        let mut nv: LHS_assign = LHS_assign {
            prev: 0 as *mut LHS_assign,
            v: expdesc {
                k: VVOID,
                u: C2RustUnnamed_11 { ival: 0 },
                t: 0,
                f: 0,
            },
        };
        nv.prev = lh;
        suffixedexp(ls, &mut nv.v);
        if !(VINDEXED as i32 as libc::c_uint <= nv.v.k as libc::c_uint
            && nv.v.k as libc::c_uint <= VINDEXSTR as i32 as libc::c_uint)
        {
            check_conflict(ls, lh, &mut nv.v);
        }
        luaE_incCstack((*ls).L);
        restassign(ls, &mut nv, nvars + 1 as i32);
        (*(*ls).L).nCcalls = ((*(*ls).L).nCcalls).wrapping_sub(1);
        (*(*ls).L).nCcalls;
    } else {
        let mut nexps: i32 = 0;
        checknext(ls, '=' as i32);
        nexps = explist(ls, &mut e);
        if nexps != nvars {
            adjust_assign(ls, nvars, nexps, &mut e);
        } else {
            luaK_setoneret((*ls).fs, &mut e);
            luaK_storevar((*ls).fs, &mut (*lh).v, &mut e);
            return;
        }
    }
    init_exp(&mut e, VNONRELOC, (*(*ls).fs).freereg as i32 - 1 as i32);
    luaK_storevar((*ls).fs, &mut (*lh).v, &mut e);
}
unsafe extern "C" fn cond(mut ls: *mut LexState) -> i32 {
    let mut v: expdesc = expdesc {
        k: VVOID,
        u: C2RustUnnamed_11 { ival: 0 },
        t: 0,
        f: 0,
    };
    expr(ls, &mut v);
    if v.k as libc::c_uint == VNIL as i32 as libc::c_uint {
        v.k = VFALSE;
    }
    luaK_goiftrue((*ls).fs, &mut v);
    return v.f;
}
unsafe extern "C" fn gotostat(mut ls: *mut LexState) {
    let mut fs: *mut FuncState = (*ls).fs;
    let mut line: i32 = (*ls).linenumber;
    let mut name: *mut TString = str_checkname(ls);
    let mut lb: *mut Labeldesc = findlabel(ls, name);
    if lb.is_null() {
        newgotoentry(ls, name, line, luaK_jump(fs));
    } else {
        let mut lblevel: i32 = reglevel(fs, (*lb).nactvar as i32);
        if luaY_nvarstack(fs) > lblevel {
            luaK_codeABCk(
                fs,
                OP_CLOSE,
                lblevel,
                0 as i32,
                0 as i32,
                0 as i32,
            );
        }
        luaK_patchlist(fs, luaK_jump(fs), (*lb).pc);
    };
}
unsafe extern "C" fn breakstat(mut ls: *mut LexState) {
    let mut line: i32 = (*ls).linenumber;
    luaX_next(ls);
    newgotoentry(
        ls,
        luaS_newlstr(
            (*ls).L,
            b"break\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_sub(1 as i32 as libc::c_ulong),
        ),
        line,
        luaK_jump((*ls).fs),
    );
}
unsafe extern "C" fn checkrepeated(mut ls: *mut LexState, mut name: *mut TString) {
    let mut lb: *mut Labeldesc = findlabel(ls, name);
    if ((lb != 0 as *mut libc::c_void as *mut Labeldesc) as i32
        != 0 as i32) as i32 as libc::c_long != 0
    {
        let mut msg: *const libc::c_char = b"label '%s' already defined on line %d\0"
            as *const u8 as *const libc::c_char;
        msg = luaO_pushfstring(
            (*ls).L,
            msg,
            ((*name).contents).as_mut_ptr(),
            (*lb).line,
        );
        luaK_semerror(ls, msg);
    }
}
unsafe extern "C" fn labelstat(
    mut ls: *mut LexState,
    mut name: *mut TString,
    mut line: i32,
) {
    checknext(ls, TK_DBCOLON as i32);
    while (*ls).t.token == ';' as i32 || (*ls).t.token == TK_DBCOLON as i32 {
        statement(ls);
    }
    checkrepeated(ls, name);
    createlabel(ls, name, line, block_follow(ls, 0 as i32));
}
unsafe extern "C" fn whilestat(mut ls: *mut LexState, mut line: i32) {
    let mut fs: *mut FuncState = (*ls).fs;
    let mut whileinit: i32 = 0;
    let mut condexit: i32 = 0;
    let mut bl: BlockCnt = BlockCnt {
        previous: 0 as *mut BlockCnt,
        firstlabel: 0,
        firstgoto: 0,
        nactvar: 0,
        upval: 0,
        isloop: 0,
        insidetbc: 0,
    };
    luaX_next(ls);
    whileinit = luaK_getlabel(fs);
    condexit = cond(ls);
    enterblock(fs, &mut bl, 1 as i32 as u8);
    checknext(ls, TK_DO as i32);
    block(ls);
    luaK_patchlist(fs, luaK_jump(fs), whileinit);
    check_match(ls, TK_END as i32, TK_WHILE as i32, line);
    leaveblock(fs);
    luaK_patchtohere(fs, condexit);
}
unsafe extern "C" fn repeatstat(mut ls: *mut LexState, mut line: i32) {
    let mut condexit: i32 = 0;
    let mut fs: *mut FuncState = (*ls).fs;
    let mut repeat_init: i32 = luaK_getlabel(fs);
    let mut bl1: BlockCnt = BlockCnt {
        previous: 0 as *mut BlockCnt,
        firstlabel: 0,
        firstgoto: 0,
        nactvar: 0,
        upval: 0,
        isloop: 0,
        insidetbc: 0,
    };
    let mut bl2: BlockCnt = BlockCnt {
        previous: 0 as *mut BlockCnt,
        firstlabel: 0,
        firstgoto: 0,
        nactvar: 0,
        upval: 0,
        isloop: 0,
        insidetbc: 0,
    };
    enterblock(fs, &mut bl1, 1 as i32 as u8);
    enterblock(fs, &mut bl2, 0 as i32 as u8);
    luaX_next(ls);
    statlist(ls);
    check_match(ls, TK_UNTIL as i32, TK_REPEAT as i32, line);
    condexit = cond(ls);
    leaveblock(fs);
    if bl2.upval != 0 {
        let mut exit: i32 = luaK_jump(fs);
        luaK_patchtohere(fs, condexit);
        luaK_codeABCk(
            fs,
            OP_CLOSE,
            reglevel(fs, bl2.nactvar as i32),
            0 as i32,
            0 as i32,
            0 as i32,
        );
        condexit = luaK_jump(fs);
        luaK_patchtohere(fs, exit);
    }
    luaK_patchlist(fs, condexit, repeat_init);
    leaveblock(fs);
}
unsafe extern "C" fn exp1(mut ls: *mut LexState) {
    let mut e: expdesc = expdesc {
        k: VVOID,
        u: C2RustUnnamed_11 { ival: 0 },
        t: 0,
        f: 0,
    };
    expr(ls, &mut e);
    luaK_exp2nextreg((*ls).fs, &mut e);
}
unsafe extern "C" fn fixforjump(
    mut fs: *mut FuncState,
    mut pc: i32,
    mut dest: i32,
    mut back: i32,
) {
    let mut jmp: *mut Instruction = &mut *((*(*fs).f).code).offset(pc as isize)
        as *mut Instruction;
    let mut offset: i32 = dest - (pc + 1 as i32);
    if back != 0 {
        offset = -offset;
    }
    if ((offset
        > ((1 as i32) << 8 as i32 + 8 as i32 + 1 as i32)
            - 1 as i32) as i32 != 0 as i32) as i32
        as libc::c_long != 0
    {
        luaX_syntaxerror(
            (*fs).ls,
            b"control structure too long\0" as *const u8 as *const libc::c_char,
        );
    }
    *jmp = *jmp
        & !(!(!(0 as i32 as Instruction)
            << 8 as i32 + 8 as i32 + 1 as i32)
            << 0 as i32 + 7 as i32 + 8 as i32)
        | (offset as Instruction)
            << 0 as i32 + 7 as i32 + 8 as i32
            & !(!(0 as i32 as Instruction)
                << 8 as i32 + 8 as i32 + 1 as i32)
                << 0 as i32 + 7 as i32 + 8 as i32;
}
unsafe extern "C" fn forbody(
    mut ls: *mut LexState,
    mut base: i32,
    mut line: i32,
    mut nvars: i32,
    mut isgen: i32,
) {
    static mut forprep: [OpCode; 2] = [OP_FORPREP, OP_TFORPREP];
    static mut forloop: [OpCode; 2] = [OP_FORLOOP, OP_TFORLOOP];
    let mut bl: BlockCnt = BlockCnt {
        previous: 0 as *mut BlockCnt,
        firstlabel: 0,
        firstgoto: 0,
        nactvar: 0,
        upval: 0,
        isloop: 0,
        insidetbc: 0,
    };
    let mut fs: *mut FuncState = (*ls).fs;
    let mut prep: i32 = 0;
    let mut endfor: i32 = 0;
    checknext(ls, TK_DO as i32);
    prep = luaK_codeABx(
        fs,
        forprep[isgen as usize],
        base,
        0 as i32 as libc::c_uint,
    );
    enterblock(fs, &mut bl, 0 as i32 as u8);
    adjustlocalvars(ls, nvars);
    luaK_reserveregs(fs, nvars);
    block(ls);
    leaveblock(fs);
    fixforjump(fs, prep, luaK_getlabel(fs), 0 as i32);
    if isgen != 0 {
        luaK_codeABCk(fs, OP_TFORCALL, base, 0 as i32, nvars, 0 as i32);
        luaK_fixline(fs, line);
    }
    endfor = luaK_codeABx(
        fs,
        forloop[isgen as usize],
        base,
        0 as i32 as libc::c_uint,
    );
    fixforjump(fs, endfor, prep + 1 as i32, 1 as i32);
    luaK_fixline(fs, line);
}
unsafe extern "C" fn fornum(
    mut ls: *mut LexState,
    mut varname: *mut TString,
    mut line: i32,
) {
    let mut fs: *mut FuncState = (*ls).fs;
    let mut base: i32 = (*fs).freereg as i32;
    new_localvar(
        ls,
        luaX_newstring(
            ls,
            b"(for state)\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_sub(1 as i32 as libc::c_ulong),
        ),
    );
    new_localvar(
        ls,
        luaX_newstring(
            ls,
            b"(for state)\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_sub(1 as i32 as libc::c_ulong),
        ),
    );
    new_localvar(
        ls,
        luaX_newstring(
            ls,
            b"(for state)\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_sub(1 as i32 as libc::c_ulong),
        ),
    );
    new_localvar(ls, varname);
    checknext(ls, '=' as i32);
    exp1(ls);
    checknext(ls, ',' as i32);
    exp1(ls);
    if testnext(ls, ',' as i32) != 0 {
        exp1(ls);
    } else {
        luaK_int(fs, (*fs).freereg as i32, 1 as i32 as Integer);
        luaK_reserveregs(fs, 1 as i32);
    }
    adjustlocalvars(ls, 3 as i32);
    forbody(ls, base, line, 1 as i32, 0 as i32);
}
unsafe extern "C" fn forlist(mut ls: *mut LexState, mut indexname: *mut TString) {
    let mut fs: *mut FuncState = (*ls).fs;
    let mut e: expdesc = expdesc {
        k: VVOID,
        u: C2RustUnnamed_11 { ival: 0 },
        t: 0,
        f: 0,
    };
    let mut nvars: i32 = 5 as i32;
    let mut line: i32 = 0;
    let mut base: i32 = (*fs).freereg as i32;
    new_localvar(
        ls,
        luaX_newstring(
            ls,
            b"(for state)\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_sub(1 as i32 as libc::c_ulong),
        ),
    );
    new_localvar(
        ls,
        luaX_newstring(
            ls,
            b"(for state)\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_sub(1 as i32 as libc::c_ulong),
        ),
    );
    new_localvar(
        ls,
        luaX_newstring(
            ls,
            b"(for state)\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_sub(1 as i32 as libc::c_ulong),
        ),
    );
    new_localvar(
        ls,
        luaX_newstring(
            ls,
            b"(for state)\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_sub(1 as i32 as libc::c_ulong),
        ),
    );
    new_localvar(ls, indexname);
    while testnext(ls, ',' as i32) != 0 {
        new_localvar(ls, str_checkname(ls));
        nvars += 1;
        nvars;
    }
    checknext(ls, TK_IN as i32);
    line = (*ls).linenumber;
    adjust_assign(ls, 4 as i32, explist(ls, &mut e), &mut e);
    adjustlocalvars(ls, 4 as i32);
    marktobeclosed(fs);
    luaK_checkstack(fs, 3 as i32);
    forbody(ls, base, line, nvars - 4 as i32, 1 as i32);
}
unsafe extern "C" fn forstat(mut ls: *mut LexState, mut line: i32) {
    let mut fs: *mut FuncState = (*ls).fs;
    let mut varname: *mut TString = 0 as *mut TString;
    let mut bl: BlockCnt = BlockCnt {
        previous: 0 as *mut BlockCnt,
        firstlabel: 0,
        firstgoto: 0,
        nactvar: 0,
        upval: 0,
        isloop: 0,
        insidetbc: 0,
    };
    enterblock(fs, &mut bl, 1 as i32 as u8);
    luaX_next(ls);
    varname = str_checkname(ls);
    match (*ls).t.token {
        61 => {
            fornum(ls, varname, line);
        }
        44 | 267 => {
            forlist(ls, varname);
        }
        _ => {
            luaX_syntaxerror(
                ls,
                b"'=' or 'in' expected\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    check_match(ls, TK_END as i32, TK_FOR as i32, line);
    leaveblock(fs);
}
unsafe extern "C" fn test_then_block(
    mut ls: *mut LexState,
    mut escapelist: *mut i32,
) {
    let mut bl: BlockCnt = BlockCnt {
        previous: 0 as *mut BlockCnt,
        firstlabel: 0,
        firstgoto: 0,
        nactvar: 0,
        upval: 0,
        isloop: 0,
        insidetbc: 0,
    };
    let mut fs: *mut FuncState = (*ls).fs;
    let mut v: expdesc = expdesc {
        k: VVOID,
        u: C2RustUnnamed_11 { ival: 0 },
        t: 0,
        f: 0,
    };
    let mut jf: i32 = 0;
    luaX_next(ls);
    expr(ls, &mut v);
    checknext(ls, TK_THEN as i32);
    if (*ls).t.token == TK_BREAK as i32 {
        let mut line: i32 = (*ls).linenumber;
        luaK_goiffalse((*ls).fs, &mut v);
        luaX_next(ls);
        enterblock(fs, &mut bl, 0 as i32 as u8);
        newgotoentry(
            ls,
            luaS_newlstr(
                (*ls).L,
                b"break\0" as *const u8 as *const libc::c_char,
                (::core::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                    )
                    .wrapping_sub(1 as i32 as libc::c_ulong),
            ),
            line,
            v.t,
        );
        while testnext(ls, ';' as i32) != 0 {}
        if block_follow(ls, 0 as i32) != 0 {
            leaveblock(fs);
            return;
        } else {
            jf = luaK_jump(fs);
        }
    } else {
        luaK_goiftrue((*ls).fs, &mut v);
        enterblock(fs, &mut bl, 0 as i32 as u8);
        jf = v.f;
    }
    statlist(ls);
    leaveblock(fs);
    if (*ls).t.token == TK_ELSE as i32
        || (*ls).t.token == TK_ELSEIF as i32
    {
        luaK_concat(fs, escapelist, luaK_jump(fs));
    }
    luaK_patchtohere(fs, jf);
}
unsafe extern "C" fn ifstat(mut ls: *mut LexState, mut line: i32) {
    let mut fs: *mut FuncState = (*ls).fs;
    let mut escapelist: i32 = -(1 as i32);
    test_then_block(ls, &mut escapelist);
    while (*ls).t.token == TK_ELSEIF as i32 {
        test_then_block(ls, &mut escapelist);
    }
    if testnext(ls, TK_ELSE as i32) != 0 {
        block(ls);
    }
    check_match(ls, TK_END as i32, TK_IF as i32, line);
    luaK_patchtohere(fs, escapelist);
}
unsafe extern "C" fn localfunc(mut ls: *mut LexState) {
    let mut b: expdesc = expdesc {
        k: VVOID,
        u: C2RustUnnamed_11 { ival: 0 },
        t: 0,
        f: 0,
    };
    let mut fs: *mut FuncState = (*ls).fs;
    let mut fvar: i32 = (*fs).nactvar as i32;
    new_localvar(ls, str_checkname(ls));
    adjustlocalvars(ls, 1 as i32);
    body(ls, &mut b, 0 as i32, (*ls).linenumber);
    (*localdebuginfo(fs, fvar)).startpc = (*fs).pc;
}
unsafe extern "C" fn getlocalattribute(mut ls: *mut LexState) -> i32 {
    if testnext(ls, '<' as i32) != 0 {
        let mut attr: *const libc::c_char = ((*str_checkname(ls)).contents).as_mut_ptr();
        checknext(ls, '>' as i32);
        if strcmp(attr, b"const\0" as *const u8 as *const libc::c_char)
            == 0 as i32
        {
            return 1 as i32
        } else if strcmp(attr, b"close\0" as *const u8 as *const libc::c_char)
            == 0 as i32
        {
            return 2 as i32
        } else {
            luaK_semerror(
                ls,
                luaO_pushfstring(
                    (*ls).L,
                    b"unknown attribute '%s'\0" as *const u8 as *const libc::c_char,
                    attr,
                ),
            );
        }
    }
    return 0 as i32;
}
unsafe extern "C" fn checktoclose(mut fs: *mut FuncState, mut level: i32) {
    if level != -(1 as i32) {
        marktobeclosed(fs);
        luaK_codeABCk(
            fs,
            OP_TBC,
            reglevel(fs, level),
            0 as i32,
            0 as i32,
            0 as i32,
        );
    }
}
unsafe extern "C" fn localstat(mut ls: *mut LexState) {
    let mut fs: *mut FuncState = (*ls).fs;
    let mut toclose: i32 = -(1 as i32);
    let mut var: *mut Vardesc = 0 as *mut Vardesc;
    let mut vidx: i32 = 0;
    let mut kind: i32 = 0;
    let mut nvars: i32 = 0 as i32;
    let mut nexps: i32 = 0;
    let mut e: expdesc = expdesc {
        k: VVOID,
        u: C2RustUnnamed_11 { ival: 0 },
        t: 0,
        f: 0,
    };
    loop {
        vidx = new_localvar(ls, str_checkname(ls));
        kind = getlocalattribute(ls);
        (*getlocalvardesc(fs, vidx)).vd.kind = kind as u8;
        if kind == 2 as i32 {
            if toclose != -(1 as i32) {
                luaK_semerror(
                    ls,
                    b"multiple to-be-closed variables in local list\0" as *const u8
                        as *const libc::c_char,
                );
            }
            toclose = (*fs).nactvar as i32 + nvars;
        }
        nvars += 1;
        nvars;
        if !(testnext(ls, ',' as i32) != 0) {
            break;
        }
    }
    if testnext(ls, '=' as i32) != 0 {
        nexps = explist(ls, &mut e);
    } else {
        e.k = VVOID;
        nexps = 0 as i32;
    }
    var = getlocalvardesc(fs, vidx);
    if nvars == nexps && (*var).vd.kind as i32 == 1 as i32
        && luaK_exp2const(fs, &mut e, &mut (*var).k) != 0
    {
        (*var).vd.kind = 3 as i32 as u8;
        adjustlocalvars(ls, nvars - 1 as i32);
        (*fs).nactvar = ((*fs).nactvar).wrapping_add(1);
        (*fs).nactvar;
    } else {
        adjust_assign(ls, nvars, nexps, &mut e);
        adjustlocalvars(ls, nvars);
    }
    checktoclose(fs, toclose);
}
unsafe extern "C" fn funcname(
    mut ls: *mut LexState,
    mut v: *mut expdesc,
) -> i32 {
    let mut ismethod: i32 = 0 as i32;
    singlevar(ls, v);
    while (*ls).t.token == '.' as i32 {
        fieldsel(ls, v);
    }
    if (*ls).t.token == ':' as i32 {
        ismethod = 1 as i32;
        fieldsel(ls, v);
    }
    return ismethod;
}
unsafe extern "C" fn funcstat(mut ls: *mut LexState, mut line: i32) {
    let mut ismethod: i32 = 0;
    let mut v: expdesc = expdesc {
        k: VVOID,
        u: C2RustUnnamed_11 { ival: 0 },
        t: 0,
        f: 0,
    };
    let mut b: expdesc = expdesc {
        k: VVOID,
        u: C2RustUnnamed_11 { ival: 0 },
        t: 0,
        f: 0,
    };
    luaX_next(ls);
    ismethod = funcname(ls, &mut v);
    body(ls, &mut b, ismethod, line);
    check_readonly(ls, &mut v);
    luaK_storevar((*ls).fs, &mut v, &mut b);
    luaK_fixline((*ls).fs, line);
}
unsafe extern "C" fn exprstat(mut ls: *mut LexState) {
    let mut fs: *mut FuncState = (*ls).fs;
    let mut v: LHS_assign = LHS_assign {
        prev: 0 as *mut LHS_assign,
        v: expdesc {
            k: VVOID,
            u: C2RustUnnamed_11 { ival: 0 },
            t: 0,
            f: 0,
        },
    };
    suffixedexp(ls, &mut v.v);
    if (*ls).t.token == '=' as i32 || (*ls).t.token == ',' as i32 {
        v.prev = 0 as *mut LHS_assign;
        restassign(ls, &mut v, 1 as i32);
    } else {
        let mut inst: *mut Instruction = 0 as *mut Instruction;
        if !(v.v.k as libc::c_uint == VCALL as i32 as libc::c_uint) {
            luaX_syntaxerror(ls, b"syntax error\0" as *const u8 as *const libc::c_char);
        }
        inst = &mut *((*(*fs).f).code).offset(v.v.u.info as isize) as *mut Instruction;
        *inst = *inst
            & !(!(!(0 as i32 as Instruction) << 8 as i32)
                << 0 as i32 + 7 as i32 + 8 as i32
                    + 1 as i32 + 8 as i32)
            | (1 as i32 as Instruction)
                << 0 as i32 + 7 as i32 + 8 as i32
                    + 1 as i32 + 8 as i32
                & !(!(0 as i32 as Instruction) << 8 as i32)
                    << 0 as i32 + 7 as i32 + 8 as i32
                        + 1 as i32 + 8 as i32;
    };
}
unsafe extern "C" fn retstat(mut ls: *mut LexState) {
    let mut fs: *mut FuncState = (*ls).fs;
    let mut e: expdesc = expdesc {
        k: VVOID,
        u: C2RustUnnamed_11 { ival: 0 },
        t: 0,
        f: 0,
    };
    let mut nret: i32 = 0;
    let mut first: i32 = luaY_nvarstack(fs);
    if block_follow(ls, 1 as i32) != 0 || (*ls).t.token == ';' as i32 {
        nret = 0 as i32;
    } else {
        nret = explist(ls, &mut e);
        if e.k as libc::c_uint == VCALL as i32 as libc::c_uint
            || e.k as libc::c_uint == VVARARG as i32 as libc::c_uint
        {
            luaK_setreturns(fs, &mut e, -(1 as i32));
            if e.k as libc::c_uint == VCALL as i32 as libc::c_uint
                && nret == 1 as i32 && (*(*fs).bl).insidetbc == 0
            {
                *((*(*fs).f).code)
                    .offset(
                        e.u.info as isize,
                    ) = *((*(*fs).f).code).offset(e.u.info as isize)
                    & !(!(!(0 as i32 as Instruction) << 7 as i32)
                        << 0 as i32)
                    | (OP_TAILCALL as i32 as Instruction) << 0 as i32
                        & !(!(0 as i32 as Instruction) << 7 as i32)
                            << 0 as i32;
            }
            nret = -(1 as i32);
        } else if nret == 1 as i32 {
            first = luaK_exp2anyreg(fs, &mut e);
        } else {
            luaK_exp2nextreg(fs, &mut e);
        }
    }
    luaK_ret(fs, first, nret);
    testnext(ls, ';' as i32);
}
unsafe extern "C" fn statement(mut ls: *mut LexState) {
    let mut line: i32 = (*ls).linenumber;
    luaE_incCstack((*ls).L);
    match (*ls).t.token {
        59 => {
            luaX_next(ls);
        }
        266 => {
            ifstat(ls, line);
        }
        277 => {
            whilestat(ls, line);
        }
        258 => {
            luaX_next(ls);
            block(ls);
            check_match(ls, TK_END as i32, TK_DO as i32, line);
        }
        263 => {
            forstat(ls, line);
        }
        272 => {
            repeatstat(ls, line);
        }
        264 => {
            funcstat(ls, line);
        }
        268 => {
            luaX_next(ls);
            if testnext(ls, TK_FUNCTION as i32) != 0 {
                localfunc(ls);
            } else {
                localstat(ls);
            }
        }
        287 => {
            luaX_next(ls);
            labelstat(ls, str_checkname(ls), line);
        }
        273 => {
            luaX_next(ls);
            retstat(ls);
        }
        257 => {
            breakstat(ls);
        }
        265 => {
            luaX_next(ls);
            gotostat(ls);
        }
        _ => {
            exprstat(ls);
        }
    }
    (*(*ls).fs).freereg = luaY_nvarstack((*ls).fs) as u8;
    (*(*ls).L).nCcalls = ((*(*ls).L).nCcalls).wrapping_sub(1);
    (*(*ls).L).nCcalls;
}
unsafe extern "C" fn mainfunc(mut ls: *mut LexState, mut fs: *mut FuncState) {
    let mut bl: BlockCnt = BlockCnt {
        previous: 0 as *mut BlockCnt,
        firstlabel: 0,
        firstgoto: 0,
        nactvar: 0,
        upval: 0,
        isloop: 0,
        insidetbc: 0,
    };
    let mut env: *mut Upvaldesc = 0 as *mut Upvaldesc;
    open_func(ls, fs, &mut bl);
    setvararg(fs, 0 as i32);
    env = allocupvalue(fs);
    (*env).instack = 1 as i32 as u8;
    (*env).index = 0 as i32 as u8;
    (*env).kind = 0 as i32 as u8;
    (*env).name = (*ls).envn;
    if (*(*fs).f).marked as i32 & (1 as i32) << 5 as i32 != 0
        && (*(*env).name).marked as i32
            & ((1 as i32) << 3 as i32
                | (1 as i32) << 4 as i32) != 0
    {
        luaC_barrier_(
            (*ls).L,
            &mut (*((*fs).f as *mut GCUnion)).gc,
            &mut (*((*env).name as *mut GCUnion)).gc,
        );
    } else {};
    luaX_next(ls);
    statlist(ls);
    check(ls, TK_EOS as i32);
    close_func(ls);
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaY_parser(
    mut L: *mut lua_State,
    mut z: *mut ZIO,
    mut buff: *mut Mbuffer,
    mut dyd: *mut Dyndata,
    mut name: *const libc::c_char,
    mut firstchar: i32,
) -> *mut LClosure {
    let mut lexstate: LexState = LexState {
        current: 0,
        linenumber: 0,
        lastline: 0,
        t: Token {
            token: 0,
            seminfo: SemInfo { r: 0. },
        },
        lookahead: Token {
            token: 0,
            seminfo: SemInfo { r: 0. },
        },
        fs: 0 as *mut FuncState,
        L: 0 as *mut lua_State,
        z: 0 as *mut ZIO,
        buff: 0 as *mut Mbuffer,
        h: 0 as *mut Table,
        dyd: 0 as *mut Dyndata,
        source: 0 as *mut TString,
        envn: 0 as *mut TString,
    };
    let mut funcstate: FuncState = FuncState {
        f: 0 as *mut Proto,
        prev: 0 as *mut FuncState,
        ls: 0 as *mut LexState,
        bl: 0 as *mut BlockCnt,
        pc: 0,
        lasttarget: 0,
        previousline: 0,
        nk: 0,
        np: 0,
        nabslineinfo: 0,
        firstlocal: 0,
        firstlabel: 0,
        ndebugvars: 0,
        nactvar: 0,
        nups: 0,
        freereg: 0,
        iwthabs: 0,
        needclose: 0,
    };
    let mut cl: *mut LClosure = luaF_newLclosure(L, 1 as i32);
    let mut io: *mut TValue = &mut (*(*L).top.p).val;
    let mut x_: *mut LClosure = cl;
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
    (*io)
        .tt_ = (6 as i32 | (0 as i32) << 4 as i32
        | (1 as i32) << 6 as i32) as u8;
    luaD_inctop(L);
    lexstate.h = luaH_new(L);
    let mut io_0: *mut TValue = &mut (*(*L).top.p).val;
    let mut x__0: *mut Table = lexstate.h;
    (*io_0).value_.gc = &mut (*(x__0 as *mut GCUnion)).gc;
    (*io_0)
        .tt_ = (5 as i32 | (0 as i32) << 4 as i32
        | (1 as i32) << 6 as i32) as u8;
    luaD_inctop(L);
    (*cl).p = luaF_newproto(L);
    funcstate.f = (*cl).p;
    if (*cl).marked as i32 & (1 as i32) << 5 as i32 != 0
        && (*(*cl).p).marked as i32
            & ((1 as i32) << 3 as i32
                | (1 as i32) << 4 as i32) != 0
    {
        luaC_barrier_(
            L,
            &mut (*(cl as *mut GCUnion)).gc,
            &mut (*((*cl).p as *mut GCUnion)).gc,
        );
    } else {};
    (*funcstate.f).source = luaS_new(L, name);
    if (*funcstate.f).marked as i32 & (1 as i32) << 5 as i32 != 0
        && (*(*funcstate.f).source).marked as i32
            & ((1 as i32) << 3 as i32
                | (1 as i32) << 4 as i32) != 0
    {
        luaC_barrier_(
            L,
            &mut (*(funcstate.f as *mut GCUnion)).gc,
            &mut (*((*funcstate.f).source as *mut GCUnion)).gc,
        );
    } else {};
    lexstate.buff = buff;
    lexstate.dyd = dyd;
    (*dyd).label.n = 0 as i32;
    (*dyd).gt.n = (*dyd).label.n;
    (*dyd).actvar.n = (*dyd).gt.n;
    luaX_setinput(L, &mut lexstate, z, (*funcstate.f).source, firstchar);
    mainfunc(&mut lexstate, &mut funcstate);
    (*L).top.p = ((*L).top.p).offset(-1);
    (*L).top.p;
    return cl;
}
