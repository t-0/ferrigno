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
    fn luaO_utf8esc(buff: *mut libc::c_char, x: libc::c_ulong) -> i32;
    fn luaO_str2num(s: *const libc::c_char, o: *mut TValue) -> u64;
    fn luaO_hexavalue(c: i32) -> i32;
    fn luaO_pushfstring(L: *mut lua_State, fmt: *const libc::c_char, _: ...)
        -> *const libc::c_char;
    static luai_ctype_: [u8; 257];
    fn luaM_saferealloc_(
        L: *mut lua_State,
        block: *mut libc::c_void,
        oldsize: u64,
        size: u64,
    ) -> *mut libc::c_void;
    fn luaZ_fill(z: *mut ZIO) -> i32;
    fn luaG_addinfo(
        L: *mut lua_State,
        msg: *const libc::c_char,
        src: *mut TString,
        line: i32,
    ) -> *const libc::c_char;
    fn luaD_throw(L: *mut lua_State, errcode: i32) -> !;
    fn luaC_fix(L: *mut lua_State, o: *mut GCObject);
    fn luaC_step(L: *mut lua_State);
    fn luaS_newlstr(L: *mut lua_State, str: *const libc::c_char, l: u64) -> *mut TString;
    fn luaS_new(L: *mut lua_State, str: *const libc::c_char) -> *mut TString;
    fn luaH_getstr(t: *mut Table, key: *mut TString) -> *const TValue;
    fn luaH_finishset(
        L: *mut lua_State,
        t: *mut Table,
        key: *const TValue,
        slot: *const TValue,
        value: *mut TValue,
    );
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
pub type lua_KContext = i64;
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
pub struct Zio {
    pub n: u64,
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
    pub n: u64,
    pub buffsize: u64,
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
pub type RESERVED = u32;
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
static mut luaX_tokens: [*const libc::c_char; 37] = [
    b"and\0" as *const u8 as *const libc::c_char,
    b"break\0" as *const u8 as *const libc::c_char,
    b"do\0" as *const u8 as *const libc::c_char,
    b"else\0" as *const u8 as *const libc::c_char,
    b"elseif\0" as *const u8 as *const libc::c_char,
    b"end\0" as *const u8 as *const libc::c_char,
    b"false\0" as *const u8 as *const libc::c_char,
    b"for\0" as *const u8 as *const libc::c_char,
    b"function\0" as *const u8 as *const libc::c_char,
    b"goto\0" as *const u8 as *const libc::c_char,
    b"if\0" as *const u8 as *const libc::c_char,
    b"in\0" as *const u8 as *const libc::c_char,
    b"local\0" as *const u8 as *const libc::c_char,
    b"nil\0" as *const u8 as *const libc::c_char,
    b"not\0" as *const u8 as *const libc::c_char,
    b"or\0" as *const u8 as *const libc::c_char,
    b"repeat\0" as *const u8 as *const libc::c_char,
    b"return\0" as *const u8 as *const libc::c_char,
    b"then\0" as *const u8 as *const libc::c_char,
    b"true\0" as *const u8 as *const libc::c_char,
    b"until\0" as *const u8 as *const libc::c_char,
    b"while\0" as *const u8 as *const libc::c_char,
    b"//\0" as *const u8 as *const libc::c_char,
    b"..\0" as *const u8 as *const libc::c_char,
    b"...\0" as *const u8 as *const libc::c_char,
    b"==\0" as *const u8 as *const libc::c_char,
    b">=\0" as *const u8 as *const libc::c_char,
    b"<=\0" as *const u8 as *const libc::c_char,
    b"~=\0" as *const u8 as *const libc::c_char,
    b"<<\0" as *const u8 as *const libc::c_char,
    b">>\0" as *const u8 as *const libc::c_char,
    b"::\0" as *const u8 as *const libc::c_char,
    b"<eof>\0" as *const u8 as *const libc::c_char,
    b"<number>\0" as *const u8 as *const libc::c_char,
    b"<integer>\0" as *const u8 as *const libc::c_char,
    b"<name>\0" as *const u8 as *const libc::c_char,
    b"<string>\0" as *const u8 as *const libc::c_char,
];
unsafe extern "C" fn save(mut ls: *mut LexState, mut c: i32) {
    let mut b: *mut Mbuffer = (*ls).buff;
    if ((*b).n).wrapping_add(1i32 as libc::c_ulong) > (*b).buffsize {
        let mut newsize: u64 = 0;
        if (*b).buffsize
            >= (if (::core::mem::size_of::<u64>() as libc::c_ulong)
                < ::core::mem::size_of::<i64>() as libc::c_ulong
            {
                !(0i32 as u64)
            } else {
                9223372036854775807i64 as u64
            })
            .wrapping_div(2i32 as libc::c_ulong)
        {
            lexerror(
                ls,
                b"lexical element too long\0" as *const u8 as *const libc::c_char,
                0i32,
            );
        }
        newsize = ((*b).buffsize).wrapping_mul(2i32 as libc::c_ulong);
        (*b).buffer = luaM_saferealloc_(
            (*ls).L,
            (*b).buffer as *mut libc::c_void,
            ((*b).buffsize).wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
            newsize.wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
        ) as *mut libc::c_char;
        (*b).buffsize = newsize;
    }
    let fresh0 = (*b).n;
    (*b).n = ((*b).n).wrapping_add(1);
    *((*b).buffer).offset(fresh0 as isize) = c as libc::c_char;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaX_init(mut L: *mut lua_State) {
    let mut i: i32 = 0;
    let mut e: *mut TString = luaS_newlstr(
        L,
        b"_ENV\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong),
    );
    luaC_fix(L, &mut (*(e as *mut GCUnion)).gc);
    i = 0i32;
    while i < TK_WHILE as i32 - (127i32 * 2i32 + 1i32 + 1i32) + 1i32 {
        let mut ts: *mut TString = luaS_new(L, luaX_tokens[i as usize]);
        luaC_fix(L, &mut (*(ts as *mut GCUnion)).gc);
        (*ts).extra = (i + 1i32) as u8;
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaX_token2str(
    mut ls: *mut LexState,
    mut token: i32,
) -> *const libc::c_char {
    if token < 127i32 * 2i32 + 1i32 + 1i32 {
        if luai_ctype_[(token + 1i32) as usize] as i32 & (1i32) << 2i32 != 0 {
            return luaO_pushfstring(
                (*ls).L,
                b"'%c'\0" as *const u8 as *const libc::c_char,
                token,
            );
        } else {
            return luaO_pushfstring(
                (*ls).L,
                b"'<\\%d>'\0" as *const u8 as *const libc::c_char,
                token,
            );
        }
    } else {
        let mut s: *const libc::c_char =
            luaX_tokens[(token - (127i32 * 2i32 + 1i32 + 1i32)) as usize];
        if token < TK_EOS as i32 {
            return luaO_pushfstring((*ls).L, b"'%s'\0" as *const u8 as *const libc::c_char, s);
        } else {
            return s;
        }
    };
}
unsafe extern "C" fn txtToken(mut ls: *mut LexState, mut token: i32) -> *const libc::c_char {
    match token {
        291 | 292 | 289 | 290 => {
            save(ls, '\0' as i32);
            return luaO_pushfstring(
                (*ls).L,
                b"'%s'\0" as *const u8 as *const libc::c_char,
                (*(*ls).buff).buffer,
            );
        }
        _ => return luaX_token2str(ls, token),
    };
}
unsafe extern "C" fn lexerror(
    mut ls: *mut LexState,
    mut msg: *const libc::c_char,
    mut token: i32,
) -> ! {
    msg = luaG_addinfo((*ls).L, msg, (*ls).source, (*ls).linenumber);
    if token != 0 {
        luaO_pushfstring(
            (*ls).L,
            b"%s near %s\0" as *const u8 as *const libc::c_char,
            msg,
            txtToken(ls, token),
        );
    }
    luaD_throw((*ls).L, 3i32);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaX_syntaxerror(
    mut ls: *mut LexState,
    mut msg: *const libc::c_char,
) -> ! {
    lexerror(ls, msg, (*ls).t.token);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaX_newstring(
    mut ls: *mut LexState,
    mut str: *const libc::c_char,
    mut l: u64,
) -> *mut TString {
    let mut L: *mut lua_State = (*ls).L;
    let mut ts: *mut TString = luaS_newlstr(L, str, l);
    let mut o: *const TValue = luaH_getstr((*ls).h, ts);
    if !((*o).tt_ as i32 & 0xf as i32 == 0i32) {
        ts = &mut (*((*(o as *mut Node)).u.key_val.gc as *mut GCUnion)).ts;
    } else {
        let fresh1 = (*L).top.p;
        (*L).top.p = ((*L).top.p).offset(1);
        let mut stv: *mut TValue = &mut (*fresh1).val;
        let mut io: *mut TValue = stv;
        let mut x_: *mut TString = ts;
        (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
        (*io).tt_ = ((*x_).tt as i32 | (1i32) << 6i32) as u8;
        luaH_finishset(L, (*ls).h, stv, o, stv);
        if (*(*L).l_G).GCdebt > 0i32 as i64 {
            luaC_step(L);
        }
        (*L).top.p = ((*L).top.p).offset(-1);
        (*L).top.p;
    }
    return ts;
}
unsafe extern "C" fn inclinenumber(mut ls: *mut LexState) {
    let mut old: i32 = (*ls).current;
    let fresh2 = (*(*ls).z).n;
    (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
    (*ls).current = if fresh2 > 0i32 as libc::c_ulong {
        let fresh3 = (*(*ls).z).p;
        (*(*ls).z).p = ((*(*ls).z).p).offset(1);
        *fresh3 as u8 as i32
    } else {
        luaZ_fill((*ls).z)
    };
    if ((*ls).current == '\n' as i32 || (*ls).current == '\r' as i32) && (*ls).current != old {
        let fresh4 = (*(*ls).z).n;
        (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
        (*ls).current = if fresh4 > 0i32 as libc::c_ulong {
            let fresh5 = (*(*ls).z).p;
            (*(*ls).z).p = ((*(*ls).z).p).offset(1);
            *fresh5 as u8 as i32
        } else {
            luaZ_fill((*ls).z)
        };
    }
    (*ls).linenumber += 1;
    if (*ls).linenumber >= 2147483647i32 {
        lexerror(
            ls,
            b"chunk has too many lines\0" as *const u8 as *const libc::c_char,
            0i32,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaX_setinput(
    mut L: *mut lua_State,
    mut ls: *mut LexState,
    mut z: *mut ZIO,
    mut source: *mut TString,
    mut firstchar: i32,
) {
    (*ls).t.token = 0i32;
    (*ls).L = L;
    (*ls).current = firstchar;
    (*ls).lookahead.token = TK_EOS as i32;
    (*ls).z = z;
    (*ls).fs = 0 as *mut FuncState;
    (*ls).linenumber = 1i32;
    (*ls).lastline = 1i32;
    (*ls).source = source;
    (*ls).envn = luaS_newlstr(
        L,
        b"_ENV\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong),
    );
    (*(*ls).buff).buffer = luaM_saferealloc_(
        (*ls).L,
        (*(*ls).buff).buffer as *mut libc::c_void,
        ((*(*ls).buff).buffsize)
            .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
        (32i32 as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    (*(*ls).buff).buffsize = 32i32 as u64;
}
unsafe extern "C" fn check_next1(mut ls: *mut LexState, mut c: i32) -> i32 {
    if (*ls).current == c {
        let fresh6 = (*(*ls).z).n;
        (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
        (*ls).current = if fresh6 > 0i32 as libc::c_ulong {
            let fresh7 = (*(*ls).z).p;
            (*(*ls).z).p = ((*(*ls).z).p).offset(1);
            *fresh7 as u8 as i32
        } else {
            luaZ_fill((*ls).z)
        };
        return 1i32;
    } else {
        return 0i32;
    };
}
unsafe extern "C" fn check_next2(mut ls: *mut LexState, mut set: *const libc::c_char) -> i32 {
    if (*ls).current == *set.offset(0i32 as isize) as i32
        || (*ls).current == *set.offset(1i32 as isize) as i32
    {
        save(ls, (*ls).current);
        let fresh8 = (*(*ls).z).n;
        (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
        (*ls).current = if fresh8 > 0i32 as libc::c_ulong {
            let fresh9 = (*(*ls).z).p;
            (*(*ls).z).p = ((*(*ls).z).p).offset(1);
            *fresh9 as u8 as i32
        } else {
            luaZ_fill((*ls).z)
        };
        return 1i32;
    } else {
        return 0i32;
    };
}
unsafe extern "C" fn read_numeral(mut ls: *mut LexState, mut seminfo: *mut SemInfo) -> i32 {
    let mut obj: TValue = TValue {
        value_: Value {
            gc: 0 as *mut GCObject,
        },
        tt_: 0,
    };
    let mut expo: *const libc::c_char = b"Ee\0" as *const u8 as *const libc::c_char;
    let mut first: i32 = (*ls).current;
    save(ls, (*ls).current);
    let fresh10 = (*(*ls).z).n;
    (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
    (*ls).current = if fresh10 > 0i32 as libc::c_ulong {
        let fresh11 = (*(*ls).z).p;
        (*(*ls).z).p = ((*(*ls).z).p).offset(1);
        *fresh11 as u8 as i32
    } else {
        luaZ_fill((*ls).z)
    };
    if first == '0' as i32 && check_next2(ls, b"xX\0" as *const u8 as *const libc::c_char) != 0 {
        expo = b"Pp\0" as *const u8 as *const libc::c_char;
    }
    loop {
        if check_next2(ls, expo) != 0 {
            check_next2(ls, b"-+\0" as *const u8 as *const libc::c_char);
        } else {
            if !(luai_ctype_[((*ls).current + 1i32) as usize] as i32 & (1i32) << 4i32 != 0
                || (*ls).current == '.' as i32)
            {
                break;
            }
            save(ls, (*ls).current);
            let fresh12 = (*(*ls).z).n;
            (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
            (*ls).current = if fresh12 > 0i32 as libc::c_ulong {
                let fresh13 = (*(*ls).z).p;
                (*(*ls).z).p = ((*(*ls).z).p).offset(1);
                *fresh13 as u8 as i32
            } else {
                luaZ_fill((*ls).z)
            };
        }
    }
    if luai_ctype_[((*ls).current + 1i32) as usize] as i32 & (1i32) << 0i32 != 0 {
        save(ls, (*ls).current);
        let fresh14 = (*(*ls).z).n;
        (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
        (*ls).current = if fresh14 > 0i32 as libc::c_ulong {
            let fresh15 = (*(*ls).z).p;
            (*(*ls).z).p = ((*(*ls).z).p).offset(1);
            *fresh15 as u8 as i32
        } else {
            luaZ_fill((*ls).z)
        };
    }
    save(ls, '\0' as i32);
    if luaO_str2num((*(*ls).buff).buffer, &mut obj) == 0i32 as libc::c_ulong {
        lexerror(
            ls,
            b"malformed number\0" as *const u8 as *const libc::c_char,
            TK_FLT as i32,
        );
    }
    if obj.tt_ as i32 == 3i32 | (0i32) << 4i32 {
        (*seminfo).i = obj.value_.i;
        return TK_INT as i32;
    } else {
        (*seminfo).r = obj.value_.n;
        return TK_FLT as i32;
    };
}
unsafe extern "C" fn skip_sep(mut ls: *mut LexState) -> u64 {
    let mut count: u64 = 0i32 as u64;
    let mut s: i32 = (*ls).current;
    save(ls, (*ls).current);
    let fresh16 = (*(*ls).z).n;
    (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
    (*ls).current = if fresh16 > 0i32 as libc::c_ulong {
        let fresh17 = (*(*ls).z).p;
        (*(*ls).z).p = ((*(*ls).z).p).offset(1);
        *fresh17 as u8 as i32
    } else {
        luaZ_fill((*ls).z)
    };
    while (*ls).current == '=' as i32 {
        save(ls, (*ls).current);
        let fresh18 = (*(*ls).z).n;
        (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
        (*ls).current = if fresh18 > 0i32 as libc::c_ulong {
            let fresh19 = (*(*ls).z).p;
            (*(*ls).z).p = ((*(*ls).z).p).offset(1);
            *fresh19 as u8 as i32
        } else {
            luaZ_fill((*ls).z)
        };
        count = count.wrapping_add(1);
    }
    return if (*ls).current == s {
        count.wrapping_add(2i32 as libc::c_ulong)
    } else {
        (if count == 0i32 as libc::c_ulong {
            1i32
        } else {
            0i32
        }) as libc::c_ulong
    };
}
unsafe extern "C" fn read_long_string(
    mut ls: *mut LexState,
    mut seminfo: *mut SemInfo,
    mut sep: u64,
) {
    let mut line: i32 = (*ls).linenumber;
    save(ls, (*ls).current);
    let fresh20 = (*(*ls).z).n;
    (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
    (*ls).current = if fresh20 > 0i32 as libc::c_ulong {
        let fresh21 = (*(*ls).z).p;
        (*(*ls).z).p = ((*(*ls).z).p).offset(1);
        *fresh21 as u8 as i32
    } else {
        luaZ_fill((*ls).z)
    };
    if (*ls).current == '\n' as i32 || (*ls).current == '\r' as i32 {
        inclinenumber(ls);
    }
    loop {
        match (*ls).current {
            -1 => {
                let mut what: *const libc::c_char = if !seminfo.is_null() {
                    b"string\0" as *const u8 as *const libc::c_char
                } else {
                    b"comment\0" as *const u8 as *const libc::c_char
                };
                let mut msg: *const libc::c_char = luaO_pushfstring(
                    (*ls).L,
                    b"unfinished long %s (starting at line %d)\0" as *const u8
                        as *const libc::c_char,
                    what,
                    line,
                );
                lexerror(ls, msg, TK_EOS as i32);
            }
            93 => {
                if !(skip_sep(ls) == sep) {
                    continue;
                }
                save(ls, (*ls).current);
                let fresh22 = (*(*ls).z).n;
                (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
                (*ls).current = if fresh22 > 0i32 as libc::c_ulong {
                    let fresh23 = (*(*ls).z).p;
                    (*(*ls).z).p = ((*(*ls).z).p).offset(1);
                    *fresh23 as u8 as i32
                } else {
                    luaZ_fill((*ls).z)
                };
                break;
            }
            10 | 13 => {
                save(ls, '\n' as i32);
                inclinenumber(ls);
                if seminfo.is_null() {
                    (*(*ls).buff).n = 0i32 as u64;
                }
            }
            _ => {
                if !seminfo.is_null() {
                    save(ls, (*ls).current);
                    let fresh24 = (*(*ls).z).n;
                    (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
                    (*ls).current = if fresh24 > 0i32 as libc::c_ulong {
                        let fresh25 = (*(*ls).z).p;
                        (*(*ls).z).p = ((*(*ls).z).p).offset(1);
                        *fresh25 as u8 as i32
                    } else {
                        luaZ_fill((*ls).z)
                    };
                } else {
                    let fresh26 = (*(*ls).z).n;
                    (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
                    (*ls).current = if fresh26 > 0i32 as libc::c_ulong {
                        let fresh27 = (*(*ls).z).p;
                        (*(*ls).z).p = ((*(*ls).z).p).offset(1);
                        *fresh27 as u8 as i32
                    } else {
                        luaZ_fill((*ls).z)
                    };
                }
            }
        }
    }
    if !seminfo.is_null() {
        (*seminfo).ts = luaX_newstring(
            ls,
            ((*(*ls).buff).buffer).offset(sep as isize),
            ((*(*ls).buff).n).wrapping_sub((2i32 as libc::c_ulong).wrapping_mul(sep)),
        );
    }
}
unsafe extern "C" fn esccheck(mut ls: *mut LexState, mut c: i32, mut msg: *const libc::c_char) {
    if c == 0 {
        if (*ls).current != -(1i32) {
            save(ls, (*ls).current);
            let fresh28 = (*(*ls).z).n;
            (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
            (*ls).current = if fresh28 > 0i32 as libc::c_ulong {
                let fresh29 = (*(*ls).z).p;
                (*(*ls).z).p = ((*(*ls).z).p).offset(1);
                *fresh29 as u8 as i32
            } else {
                luaZ_fill((*ls).z)
            };
        }
        lexerror(ls, msg, TK_STRING as i32);
    }
}
unsafe extern "C" fn gethexa(mut ls: *mut LexState) -> i32 {
    save(ls, (*ls).current);
    let fresh30 = (*(*ls).z).n;
    (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
    (*ls).current = if fresh30 > 0i32 as libc::c_ulong {
        let fresh31 = (*(*ls).z).p;
        (*(*ls).z).p = ((*(*ls).z).p).offset(1);
        *fresh31 as u8 as i32
    } else {
        luaZ_fill((*ls).z)
    };
    esccheck(
        ls,
        luai_ctype_[((*ls).current + 1i32) as usize] as i32 & (1i32) << 4i32,
        b"hexadecimal digit expected\0" as *const u8 as *const libc::c_char,
    );
    return luaO_hexavalue((*ls).current);
}
unsafe extern "C" fn readhexaesc(mut ls: *mut LexState) -> i32 {
    let mut r: i32 = gethexa(ls);
    r = (r << 4i32) + gethexa(ls);
    (*(*ls).buff).n =
        ((*(*ls).buff).n as libc::c_ulong).wrapping_sub(2i32 as libc::c_ulong) as u64 as u64;
    return r;
}
unsafe extern "C" fn readutf8esc(mut ls: *mut LexState) -> libc::c_ulong {
    let mut r: libc::c_ulong = 0;
    let mut i: i32 = 4i32;
    save(ls, (*ls).current);
    let fresh32 = (*(*ls).z).n;
    (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
    (*ls).current = if fresh32 > 0i32 as libc::c_ulong {
        let fresh33 = (*(*ls).z).p;
        (*(*ls).z).p = ((*(*ls).z).p).offset(1);
        *fresh33 as u8 as i32
    } else {
        luaZ_fill((*ls).z)
    };
    esccheck(
        ls,
        ((*ls).current == '{' as i32) as i32,
        b"missing '{'\0" as *const u8 as *const libc::c_char,
    );
    r = gethexa(ls) as libc::c_ulong;
    loop {
        save(ls, (*ls).current);
        let fresh34 = (*(*ls).z).n;
        (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
        (*ls).current = if fresh34 > 0i32 as libc::c_ulong {
            let fresh35 = (*(*ls).z).p;
            (*(*ls).z).p = ((*(*ls).z).p).offset(1);
            *fresh35 as u8 as i32
        } else {
            luaZ_fill((*ls).z)
        };
        if !(luai_ctype_[((*ls).current + 1i32) as usize] as i32 & (1i32) << 4i32 != 0) {
            break;
        }
        i += 1;
        esccheck(
            ls,
            (r <= (0x7fffffff as u32 >> 4i32) as libc::c_ulong) as i32,
            b"UTF-8 value too large\0" as *const u8 as *const libc::c_char,
        );
        r = (r << 4i32).wrapping_add(luaO_hexavalue((*ls).current) as libc::c_ulong);
    }
    esccheck(
        ls,
        ((*ls).current == '}' as i32) as i32,
        b"missing '}'\0" as *const u8 as *const libc::c_char,
    );
    let fresh36 = (*(*ls).z).n;
    (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
    (*ls).current = if fresh36 > 0i32 as libc::c_ulong {
        let fresh37 = (*(*ls).z).p;
        (*(*ls).z).p = ((*(*ls).z).p).offset(1);
        *fresh37 as u8 as i32
    } else {
        luaZ_fill((*ls).z)
    };
    (*(*ls).buff).n =
        ((*(*ls).buff).n as libc::c_ulong).wrapping_sub(i as libc::c_ulong) as u64 as u64;
    return r;
}
unsafe extern "C" fn utf8esc(mut ls: *mut LexState) {
    let mut buff: [libc::c_char; 8] = [0; 8];
    let mut n: i32 = luaO_utf8esc(buff.as_mut_ptr(), readutf8esc(ls));
    while n > 0i32 {
        save(ls, buff[(8i32 - n) as usize] as i32);
        n -= 1;
    }
}
unsafe extern "C" fn readdecesc(mut ls: *mut LexState) -> i32 {
    let mut i: i32 = 0;
    let mut r: i32 = 0i32;
    i = 0i32;
    while i < 3i32 && luai_ctype_[((*ls).current + 1i32) as usize] as i32 & (1i32) << 1i32 != 0 {
        r = 10i32 * r + (*ls).current - '0' as i32;
        save(ls, (*ls).current);
        let fresh38 = (*(*ls).z).n;
        (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
        (*ls).current = if fresh38 > 0i32 as libc::c_ulong {
            let fresh39 = (*(*ls).z).p;
            (*(*ls).z).p = ((*(*ls).z).p).offset(1);
            *fresh39 as u8 as i32
        } else {
            luaZ_fill((*ls).z)
        };
        i += 1;
    }
    esccheck(
        ls,
        (r <= 127i32 * 2i32 + 1i32) as i32,
        b"decimal escape too large\0" as *const u8 as *const libc::c_char,
    );
    (*(*ls).buff).n =
        ((*(*ls).buff).n as libc::c_ulong).wrapping_sub(i as libc::c_ulong) as u64 as u64;
    return r;
}
unsafe extern "C" fn read_string(mut ls: *mut LexState, mut del: i32, mut seminfo: *mut SemInfo) {
    let mut current_block: u64;
    save(ls, (*ls).current);
    let fresh40 = (*(*ls).z).n;
    (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
    (*ls).current = if fresh40 > 0i32 as libc::c_ulong {
        let fresh41 = (*(*ls).z).p;
        (*(*ls).z).p = ((*(*ls).z).p).offset(1);
        *fresh41 as u8 as i32
    } else {
        luaZ_fill((*ls).z)
    };
    while (*ls).current != del {
        match (*ls).current {
            -1 => {
                lexerror(
                    ls,
                    b"unfinished string\0" as *const u8 as *const libc::c_char,
                    TK_EOS as i32,
                );
            }
            10 | 13 => {
                lexerror(
                    ls,
                    b"unfinished string\0" as *const u8 as *const libc::c_char,
                    TK_STRING as i32,
                );
            }
            92 => {
                let mut c: i32 = 0;
                save(ls, (*ls).current);
                let fresh42 = (*(*ls).z).n;
                (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
                (*ls).current = if fresh42 > 0i32 as libc::c_ulong {
                    let fresh43 = (*(*ls).z).p;
                    (*(*ls).z).p = ((*(*ls).z).p).offset(1);
                    *fresh43 as u8 as i32
                } else {
                    luaZ_fill((*ls).z)
                };
                match (*ls).current {
                    97 => {
                        c = '\u{7}' as i32;
                        current_block = 18180014530125241895;
                    }
                    98 => {
                        c = '\u{8}' as i32;
                        current_block = 18180014530125241895;
                    }
                    102 => {
                        c = '\u{c}' as i32;
                        current_block = 18180014530125241895;
                    }
                    110 => {
                        c = '\n' as i32;
                        current_block = 18180014530125241895;
                    }
                    114 => {
                        c = '\r' as i32;
                        current_block = 18180014530125241895;
                    }
                    116 => {
                        c = '\t' as i32;
                        current_block = 18180014530125241895;
                    }
                    118 => {
                        c = '\u{b}' as i32;
                        current_block = 18180014530125241895;
                    }
                    120 => {
                        c = readhexaesc(ls);
                        current_block = 18180014530125241895;
                    }
                    117 => {
                        utf8esc(ls);
                        continue;
                    }
                    10 | 13 => {
                        inclinenumber(ls);
                        c = '\n' as i32;
                        current_block = 10644833639285892640;
                    }
                    92 | 34 | 39 => {
                        c = (*ls).current;
                        current_block = 18180014530125241895;
                    }
                    -1 => {
                        continue;
                    }
                    122 => {
                        (*(*ls).buff).n = ((*(*ls).buff).n as libc::c_ulong)
                            .wrapping_sub(1i32 as libc::c_ulong)
                            as u64 as u64;
                        let fresh44 = (*(*ls).z).n;
                        (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
                        (*ls).current = if fresh44 > 0i32 as libc::c_ulong {
                            let fresh45 = (*(*ls).z).p;
                            (*(*ls).z).p = ((*(*ls).z).p).offset(1);
                            *fresh45 as u8 as i32
                        } else {
                            luaZ_fill((*ls).z)
                        };
                        while luai_ctype_[((*ls).current + 1i32) as usize] as i32 & (1i32) << 3i32
                            != 0
                        {
                            if (*ls).current == '\n' as i32 || (*ls).current == '\r' as i32 {
                                inclinenumber(ls);
                            } else {
                                let fresh46 = (*(*ls).z).n;
                                (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
                                (*ls).current = if fresh46 > 0i32 as libc::c_ulong {
                                    let fresh47 = (*(*ls).z).p;
                                    (*(*ls).z).p = ((*(*ls).z).p).offset(1);
                                    *fresh47 as u8 as i32
                                } else {
                                    luaZ_fill((*ls).z)
                                };
                            }
                        }
                        continue;
                    }
                    _ => {
                        esccheck(
                            ls,
                            luai_ctype_[((*ls).current + 1i32) as usize] as i32 & (1i32) << 1i32,
                            b"invalid escape sequence\0" as *const u8 as *const libc::c_char,
                        );
                        c = readdecesc(ls);
                        current_block = 10644833639285892640;
                    }
                }
                match current_block {
                    18180014530125241895 => {
                        let fresh48 = (*(*ls).z).n;
                        (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
                        (*ls).current = if fresh48 > 0i32 as libc::c_ulong {
                            let fresh49 = (*(*ls).z).p;
                            (*(*ls).z).p = ((*(*ls).z).p).offset(1);
                            *fresh49 as u8 as i32
                        } else {
                            luaZ_fill((*ls).z)
                        };
                    }
                    _ => {}
                }
                (*(*ls).buff).n = ((*(*ls).buff).n as libc::c_ulong)
                    .wrapping_sub(1i32 as libc::c_ulong) as u64
                    as u64;
                save(ls, c);
            }
            _ => {
                save(ls, (*ls).current);
                let fresh50 = (*(*ls).z).n;
                (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
                (*ls).current = if fresh50 > 0i32 as libc::c_ulong {
                    let fresh51 = (*(*ls).z).p;
                    (*(*ls).z).p = ((*(*ls).z).p).offset(1);
                    *fresh51 as u8 as i32
                } else {
                    luaZ_fill((*ls).z)
                };
            }
        }
    }
    save(ls, (*ls).current);
    let fresh52 = (*(*ls).z).n;
    (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
    (*ls).current = if fresh52 > 0i32 as libc::c_ulong {
        let fresh53 = (*(*ls).z).p;
        (*(*ls).z).p = ((*(*ls).z).p).offset(1);
        *fresh53 as u8 as i32
    } else {
        luaZ_fill((*ls).z)
    };
    (*seminfo).ts = luaX_newstring(
        ls,
        ((*(*ls).buff).buffer).offset(1i32 as isize),
        ((*(*ls).buff).n).wrapping_sub(2i32 as libc::c_ulong),
    );
}
unsafe extern "C" fn llex(mut ls: *mut LexState, mut seminfo: *mut SemInfo) -> i32 {
    (*(*ls).buff).n = 0i32 as u64;
    loop {
        let mut current_block_85: u64;
        match (*ls).current {
            10 | 13 => {
                inclinenumber(ls);
            }
            32 | 12 | 9 | 11 => {
                let fresh54 = (*(*ls).z).n;
                (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
                (*ls).current = if fresh54 > 0i32 as libc::c_ulong {
                    let fresh55 = (*(*ls).z).p;
                    (*(*ls).z).p = ((*(*ls).z).p).offset(1);
                    *fresh55 as u8 as i32
                } else {
                    luaZ_fill((*ls).z)
                };
            }
            45 => {
                let fresh56 = (*(*ls).z).n;
                (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
                (*ls).current = if fresh56 > 0i32 as libc::c_ulong {
                    let fresh57 = (*(*ls).z).p;
                    (*(*ls).z).p = ((*(*ls).z).p).offset(1);
                    *fresh57 as u8 as i32
                } else {
                    luaZ_fill((*ls).z)
                };
                if (*ls).current != '-' as i32 {
                    return '-' as i32;
                }
                let fresh58 = (*(*ls).z).n;
                (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
                (*ls).current = if fresh58 > 0i32 as libc::c_ulong {
                    let fresh59 = (*(*ls).z).p;
                    (*(*ls).z).p = ((*(*ls).z).p).offset(1);
                    *fresh59 as u8 as i32
                } else {
                    luaZ_fill((*ls).z)
                };
                if (*ls).current == '[' as i32 {
                    let mut sep: u64 = skip_sep(ls);
                    (*(*ls).buff).n = 0i32 as u64;
                    if sep >= 2i32 as libc::c_ulong {
                        read_long_string(ls, 0 as *mut SemInfo, sep);
                        (*(*ls).buff).n = 0i32 as u64;
                        current_block_85 = 10512632378975961025;
                    } else {
                        current_block_85 = 3512920355445576850;
                    }
                } else {
                    current_block_85 = 3512920355445576850;
                }
                match current_block_85 {
                    10512632378975961025 => {}
                    _ => {
                        while !((*ls).current == '\n' as i32 || (*ls).current == '\r' as i32)
                            && (*ls).current != -(1i32)
                        {
                            let fresh60 = (*(*ls).z).n;
                            (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
                            (*ls).current = if fresh60 > 0i32 as libc::c_ulong {
                                let fresh61 = (*(*ls).z).p;
                                (*(*ls).z).p = ((*(*ls).z).p).offset(1);
                                *fresh61 as u8 as i32
                            } else {
                                luaZ_fill((*ls).z)
                            };
                        }
                    }
                }
            }
            91 => {
                let mut sep_0: u64 = skip_sep(ls);
                if sep_0 >= 2i32 as libc::c_ulong {
                    read_long_string(ls, seminfo, sep_0);
                    return TK_STRING as i32;
                } else if sep_0 == 0i32 as libc::c_ulong {
                    lexerror(
                        ls,
                        b"invalid long string delimiter\0" as *const u8 as *const libc::c_char,
                        TK_STRING as i32,
                    );
                }
                return '[' as i32;
            }
            61 => {
                let fresh62 = (*(*ls).z).n;
                (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
                (*ls).current = if fresh62 > 0i32 as libc::c_ulong {
                    let fresh63 = (*(*ls).z).p;
                    (*(*ls).z).p = ((*(*ls).z).p).offset(1);
                    *fresh63 as u8 as i32
                } else {
                    luaZ_fill((*ls).z)
                };
                if check_next1(ls, '=' as i32) != 0 {
                    return TK_EQ as i32;
                } else {
                    return '=' as i32;
                }
            }
            60 => {
                let fresh64 = (*(*ls).z).n;
                (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
                (*ls).current = if fresh64 > 0i32 as libc::c_ulong {
                    let fresh65 = (*(*ls).z).p;
                    (*(*ls).z).p = ((*(*ls).z).p).offset(1);
                    *fresh65 as u8 as i32
                } else {
                    luaZ_fill((*ls).z)
                };
                if check_next1(ls, '=' as i32) != 0 {
                    return TK_LE as i32;
                } else if check_next1(ls, '<' as i32) != 0 {
                    return TK_SHL as i32;
                } else {
                    return '<' as i32;
                }
            }
            62 => {
                let fresh66 = (*(*ls).z).n;
                (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
                (*ls).current = if fresh66 > 0i32 as libc::c_ulong {
                    let fresh67 = (*(*ls).z).p;
                    (*(*ls).z).p = ((*(*ls).z).p).offset(1);
                    *fresh67 as u8 as i32
                } else {
                    luaZ_fill((*ls).z)
                };
                if check_next1(ls, '=' as i32) != 0 {
                    return TK_GE as i32;
                } else if check_next1(ls, '>' as i32) != 0 {
                    return TK_SHR as i32;
                } else {
                    return '>' as i32;
                }
            }
            47 => {
                let fresh68 = (*(*ls).z).n;
                (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
                (*ls).current = if fresh68 > 0i32 as libc::c_ulong {
                    let fresh69 = (*(*ls).z).p;
                    (*(*ls).z).p = ((*(*ls).z).p).offset(1);
                    *fresh69 as u8 as i32
                } else {
                    luaZ_fill((*ls).z)
                };
                if check_next1(ls, '/' as i32) != 0 {
                    return TK_IDIV as i32;
                } else {
                    return '/' as i32;
                }
            }
            126 => {
                let fresh70 = (*(*ls).z).n;
                (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
                (*ls).current = if fresh70 > 0i32 as libc::c_ulong {
                    let fresh71 = (*(*ls).z).p;
                    (*(*ls).z).p = ((*(*ls).z).p).offset(1);
                    *fresh71 as u8 as i32
                } else {
                    luaZ_fill((*ls).z)
                };
                if check_next1(ls, '=' as i32) != 0 {
                    return TK_NE as i32;
                } else {
                    return '~' as i32;
                }
            }
            58 => {
                let fresh72 = (*(*ls).z).n;
                (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
                (*ls).current = if fresh72 > 0i32 as libc::c_ulong {
                    let fresh73 = (*(*ls).z).p;
                    (*(*ls).z).p = ((*(*ls).z).p).offset(1);
                    *fresh73 as u8 as i32
                } else {
                    luaZ_fill((*ls).z)
                };
                if check_next1(ls, ':' as i32) != 0 {
                    return TK_DBCOLON as i32;
                } else {
                    return ':' as i32;
                }
            }
            34 | 39 => {
                read_string(ls, (*ls).current, seminfo);
                return TK_STRING as i32;
            }
            46 => {
                save(ls, (*ls).current);
                let fresh74 = (*(*ls).z).n;
                (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
                (*ls).current = if fresh74 > 0i32 as libc::c_ulong {
                    let fresh75 = (*(*ls).z).p;
                    (*(*ls).z).p = ((*(*ls).z).p).offset(1);
                    *fresh75 as u8 as i32
                } else {
                    luaZ_fill((*ls).z)
                };
                if check_next1(ls, '.' as i32) != 0 {
                    if check_next1(ls, '.' as i32) != 0 {
                        return TK_DOTS as i32;
                    } else {
                        return TK_CONCAT as i32;
                    }
                } else if luai_ctype_[((*ls).current + 1i32) as usize] as i32 & (1i32) << 1i32 == 0
                {
                    return '.' as i32;
                } else {
                    return read_numeral(ls, seminfo);
                }
            }
            48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                return read_numeral(ls, seminfo);
            }
            -1 => return TK_EOS as i32,
            _ => {
                if luai_ctype_[((*ls).current + 1i32) as usize] as i32 & (1i32) << 0i32 != 0 {
                    let mut ts: *mut TString = 0 as *mut TString;
                    loop {
                        save(ls, (*ls).current);
                        let fresh76 = (*(*ls).z).n;
                        (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
                        (*ls).current = if fresh76 > 0i32 as libc::c_ulong {
                            let fresh77 = (*(*ls).z).p;
                            (*(*ls).z).p = ((*(*ls).z).p).offset(1);
                            *fresh77 as u8 as i32
                        } else {
                            luaZ_fill((*ls).z)
                        };
                        if !(luai_ctype_[((*ls).current + 1i32) as usize] as i32
                            & ((1i32) << 0i32 | (1i32) << 1i32)
                            != 0)
                        {
                            break;
                        }
                    }
                    ts = luaX_newstring(ls, (*(*ls).buff).buffer, (*(*ls).buff).n);
                    (*seminfo).ts = ts;
                    if (*ts).tt as i32 == 4i32 | (0i32) << 4i32 && (*ts).extra as i32 > 0i32 {
                        return (*ts).extra as i32 - 1i32 + (127i32 * 2i32 + 1i32 + 1i32);
                    } else {
                        return TK_NAME as i32;
                    }
                } else {
                    let mut c: i32 = (*ls).current;
                    let fresh78 = (*(*ls).z).n;
                    (*(*ls).z).n = ((*(*ls).z).n).wrapping_sub(1);
                    (*ls).current = if fresh78 > 0i32 as libc::c_ulong {
                        let fresh79 = (*(*ls).z).p;
                        (*(*ls).z).p = ((*(*ls).z).p).offset(1);
                        *fresh79 as u8 as i32
                    } else {
                        luaZ_fill((*ls).z)
                    };
                    return c;
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaX_next(mut ls: *mut LexState) {
    (*ls).lastline = (*ls).linenumber;
    if (*ls).lookahead.token != TK_EOS as i32 {
        (*ls).t = (*ls).lookahead;
        (*ls).lookahead.token = TK_EOS as i32;
    } else {
        (*ls).t.token = llex(ls, &mut (*ls).t.seminfo);
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaX_lookahead(mut ls: *mut LexState) -> i32 {
    (*ls).lookahead.token = llex(ls, &mut (*ls).lookahead.seminfo);
    return (*ls).lookahead.token;
}
