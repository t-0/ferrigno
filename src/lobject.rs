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
    fn localeconv() -> *mut lconv;
    fn pow(_: f64, _: f64) -> f64;
    fn snprintf(_: *mut libc::c_char, _: libc::c_ulong, _: *const libc::c_char, _: ...) -> i32;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> f64;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: i32) -> *mut libc::c_char;
    fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strpbrk(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn luaT_trybinTM(
        L: *mut lua_State,
        p1: *const TValue,
        p2: *const TValue,
        res: StkId,
        event: TMS,
    );
    static luai_ctype_: [u8; 257];
    fn luaG_runerror(L: *mut lua_State, fmt: *const libc::c_char, _: ...) -> !;
    fn luaS_newlstr(L: *mut lua_State, str: *const libc::c_char, l: u64) -> *mut TString;
    fn luaV_tointegerns(obj: *const TValue, p: *mut i64, mode: F2Imod) -> i32;
    fn luaV_concat(L: *mut lua_State, total: i32);
    fn luaV_idiv(L: *mut lua_State, x: i64, y: i64) -> i64;
    fn luaV_mod(L: *mut lua_State, x: i64, y: i64) -> i64;
    fn luaV_modf(L: *mut lua_State, x: f64, y: f64) -> f64;
    fn luaV_shiftl(x: i64, y: i64) -> i64;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: u32,
    pub fp_offset: u32,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lconv {
    pub decimal_point: *mut libc::c_char,
    pub thousands_sep: *mut libc::c_char,
    pub grouping: *mut libc::c_char,
    pub int_curr_symbol: *mut libc::c_char,
    pub currency_symbol: *mut libc::c_char,
    pub mon_decimal_point: *mut libc::c_char,
    pub mon_thousands_sep: *mut libc::c_char,
    pub mon_grouping: *mut libc::c_char,
    pub positive_sign: *mut libc::c_char,
    pub negative_sign: *mut libc::c_char,
    pub int_frac_digits: libc::c_char,
    pub frac_digits: libc::c_char,
    pub p_cs_precedes: libc::c_char,
    pub p_sep_by_space: libc::c_char,
    pub n_cs_precedes: libc::c_char,
    pub n_sep_by_space: libc::c_char,
    pub p_sign_posn: libc::c_char,
    pub n_sign_posn: libc::c_char,
    pub int_p_cs_precedes: libc::c_char,
    pub int_p_sep_by_space: libc::c_char,
    pub int_n_cs_precedes: libc::c_char,
    pub int_n_sep_by_space: libc::c_char,
    pub int_p_sign_posn: libc::c_char,
    pub int_n_sign_posn: libc::c_char,
}

pub type va_list = __builtin_va_list;

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
    pub u: LObjectC2RustUnnamed_1,
    pub u2: LObjectC2RustUnnamed,
    pub nresults: libc::c_short,
    pub callstatus: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union LObjectC2RustUnnamed {
    pub funcidx: i32,
    pub nyield: i32,
    pub nres: i32,
    pub transferinfo: LObjectC2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LObjectC2RustUnnamed_0 {
    pub ftransfer: libc::c_ushort,
    pub ntransfer: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union LObjectC2RustUnnamed_1 {
    pub l: LObjectC2RustUnnamed_3,
    pub c: LObjectC2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LObjectC2RustUnnamed_2 {
    pub k: lua_KFunction,
    pub old_errfunc: i64,
    pub ctx: lua_KContext,
}
pub type lua_KFunction = Option<unsafe extern "C" fn(*mut lua_State, i32, lua_KContext) -> i32>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LObjectC2RustUnnamed_3 {
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
    pub tbclist: LObjectC2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LObjectC2RustUnnamed_4 {
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
    pub v: LObjectC2RustUnnamed_7,
    pub u: LObjectC2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union LObjectC2RustUnnamed_5 {
    pub open: LObjectC2RustUnnamed_6,
    pub value: TValue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LObjectC2RustUnnamed_6 {
    pub next: *mut UpVal,
    pub previous: *mut *mut UpVal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union LObjectC2RustUnnamed_7 {
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
    pub u: LObjectC2RustUnnamed_8,
    pub contents: [libc::c_char; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union LObjectC2RustUnnamed_8 {
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

pub type ls_byte = libc::c_schar;
pub type l_uacNumber = f64;
pub type l_uacInt = i64;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BuffFS {
    pub L: *mut lua_State,
    pub pushed: i32,
    pub blen: i32,
    pub space: [libc::c_char; 199],
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaO_ceillog2(mut x: u32) -> i32 {
    static mut log_2: [u8; 256] = [
        0i32 as u8, 1i32 as u8, 2i32 as u8, 2i32 as u8, 3i32 as u8, 3i32 as u8, 3i32 as u8,
        3i32 as u8, 4i32 as u8, 4i32 as u8, 4i32 as u8, 4i32 as u8, 4i32 as u8, 4i32 as u8,
        4i32 as u8, 4i32 as u8, 5i32 as u8, 5i32 as u8, 5i32 as u8, 5i32 as u8, 5i32 as u8,
        5i32 as u8, 5i32 as u8, 5i32 as u8, 5i32 as u8, 5i32 as u8, 5i32 as u8, 5i32 as u8,
        5i32 as u8, 5i32 as u8, 5i32 as u8, 5i32 as u8, 6i32 as u8, 6i32 as u8, 6i32 as u8,
        6i32 as u8, 6i32 as u8, 6i32 as u8, 6i32 as u8, 6i32 as u8, 6i32 as u8, 6i32 as u8,
        6i32 as u8, 6i32 as u8, 6i32 as u8, 6i32 as u8, 6i32 as u8, 6i32 as u8, 6i32 as u8,
        6i32 as u8, 6i32 as u8, 6i32 as u8, 6i32 as u8, 6i32 as u8, 6i32 as u8, 6i32 as u8,
        6i32 as u8, 6i32 as u8, 6i32 as u8, 6i32 as u8, 6i32 as u8, 6i32 as u8, 6i32 as u8,
        6i32 as u8, 7i32 as u8, 7i32 as u8, 7i32 as u8, 7i32 as u8, 7i32 as u8, 7i32 as u8,
        7i32 as u8, 7i32 as u8, 7i32 as u8, 7i32 as u8, 7i32 as u8, 7i32 as u8, 7i32 as u8,
        7i32 as u8, 7i32 as u8, 7i32 as u8, 7i32 as u8, 7i32 as u8, 7i32 as u8, 7i32 as u8,
        7i32 as u8, 7i32 as u8, 7i32 as u8, 7i32 as u8, 7i32 as u8, 7i32 as u8, 7i32 as u8,
        7i32 as u8, 7i32 as u8, 7i32 as u8, 7i32 as u8, 7i32 as u8, 7i32 as u8, 7i32 as u8,
        7i32 as u8, 7i32 as u8, 7i32 as u8, 7i32 as u8, 7i32 as u8, 7i32 as u8, 7i32 as u8,
        7i32 as u8, 7i32 as u8, 7i32 as u8, 7i32 as u8, 7i32 as u8, 7i32 as u8, 7i32 as u8,
        7i32 as u8, 7i32 as u8, 7i32 as u8, 7i32 as u8, 7i32 as u8, 7i32 as u8, 7i32 as u8,
        7i32 as u8, 7i32 as u8, 7i32 as u8, 7i32 as u8, 7i32 as u8, 7i32 as u8, 7i32 as u8,
        7i32 as u8, 7i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8,
        8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8,
        8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8,
        8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8,
        8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8,
        8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8,
        8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8,
        8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8,
        8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8,
        8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8,
        8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8,
        8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8,
        8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8,
        8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8,
        8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8,
        8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8,
        8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8,
        8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8,
        8i32 as u8, 8i32 as u8, 8i32 as u8, 8i32 as u8,
    ];
    let mut l: i32 = 0i32;
    x = x.wrapping_sub(1);
    while x >= 256i32 as u32 {
        l += 8i32;
        x >>= 8i32;
    }
    return l + log_2[x as usize] as i32;
}
unsafe extern "C" fn intarith(mut L: *mut lua_State, mut op: i32, mut v1: i64, mut v2: i64) -> i64 {
    match op {
        0 => return (v1 as u64).wrapping_add(v2 as u64) as i64,
        1 => return (v1 as u64).wrapping_sub(v2 as u64) as i64,
        2 => return (v1 as u64).wrapping_mul(v2 as u64) as i64,
        3 => return luaV_mod(L, v1, v2),
        6 => return luaV_idiv(L, v1, v2),
        7 => return (v1 as u64 & v2 as u64) as i64,
        8 => return (v1 as u64 | v2 as u64) as i64,
        9 => return (v1 as u64 ^ v2 as u64) as i64,
        10 => return luaV_shiftl(v1, v2),
        11 => {
            return luaV_shiftl(v1, (0i32 as u64).wrapping_sub(v2 as u64) as i64);
        }
        12 => {
            return (0i32 as u64).wrapping_sub(v1 as u64) as i64;
        }
        13 => {
            return (!(0i32 as u64) ^ v1 as u64) as i64;
        }
        _ => return 0i32 as i64,
    };
}
unsafe extern "C" fn numarith(mut L: *mut lua_State, mut op: i32, mut v1: f64, mut v2: f64) -> f64 {
    match op {
        0 => return v1 + v2,
        1 => return v1 - v2,
        2 => return v1 * v2,
        5 => return v1 / v2,
        4 => {
            return if v2 == 2i32 as f64 {
                v1 * v1
            } else {
                pow(v1, v2)
            };
        }
        6 => return (v1 / v2).floor(),
        12 => return -v1,
        3 => return luaV_modf(L, v1, v2),
        _ => return 0i32 as f64,
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaO_rawarith(
    mut L: *mut lua_State,
    mut op: i32,
    mut p1: *const TValue,
    mut p2: *const TValue,
    mut res: *mut TValue,
) -> i32 {
    match op {
        7 | 8 | 9 | 10 | 11 | 13 => {
            let mut i1: i64 = 0;
            let mut i2: i64 = 0;
            if (if (((*p1).tt_ as i32 == 3i32 | (0i32) << 4i32) as i32 != 0i32) as i32 as i64 != 0 {
                i1 = (*p1).value_.i;
                1i32
            } else {
                luaV_tointegerns(p1, &mut i1, F2Ieq)
            }) != 0
                && (if (((*p2).tt_ as i32 == 3i32 | (0i32) << 4i32) as i32 != 0i32) as i32 as i64
                    != 0
                {
                    i2 = (*p2).value_.i;
                    1i32
                } else {
                    luaV_tointegerns(p2, &mut i2, F2Ieq)
                }) != 0
            {
                let mut io: *mut TValue = res;
                (*io).value_.i = intarith(L, op, i1, i2);
                (*io).tt_ = (3i32 | (0i32) << 4i32) as u8;
                return 1i32;
            } else {
                return 0i32;
            }
        }
        5 | 4 => {
            let mut n1: f64 = 0.;
            let mut n2: f64 = 0.;
            if (if (*p1).tt_ as i32 == 3i32 | (1i32) << 4i32 {
                n1 = (*p1).value_.n;
                1i32
            } else {
                if (*p1).tt_ as i32 == 3i32 | (0i32) << 4i32 {
                    n1 = (*p1).value_.i as f64;
                    1i32
                } else {
                    0i32
                }
            }) != 0
                && (if (*p2).tt_ as i32 == 3i32 | (1i32) << 4i32 {
                    n2 = (*p2).value_.n;
                    1i32
                } else {
                    if (*p2).tt_ as i32 == 3i32 | (0i32) << 4i32 {
                        n2 = (*p2).value_.i as f64;
                        1i32
                    } else {
                        0i32
                    }
                }) != 0
            {
                let mut io_0: *mut TValue = res;
                (*io_0).value_.n = numarith(L, op, n1, n2);
                (*io_0).tt_ = (3i32 | (1i32) << 4i32) as u8;
                return 1i32;
            } else {
                return 0i32;
            }
        }
        _ => {
            let mut n1_0: f64 = 0.;
            let mut n2_0: f64 = 0.;
            if (*p1).tt_ as i32 == 3i32 | (0i32) << 4i32
                && (*p2).tt_ as i32 == 3i32 | (0i32) << 4i32
            {
                let mut io_1: *mut TValue = res;
                (*io_1).value_.i = intarith(L, op, (*p1).value_.i, (*p2).value_.i);
                (*io_1).tt_ = (3i32 | (0i32) << 4i32) as u8;
                return 1i32;
            } else if (if (*p1).tt_ as i32 == 3i32 | (1i32) << 4i32 {
                n1_0 = (*p1).value_.n;
                1i32
            } else {
                if (*p1).tt_ as i32 == 3i32 | (0i32) << 4i32 {
                    n1_0 = (*p1).value_.i as f64;
                    1i32
                } else {
                    0i32
                }
            }) != 0
                && (if (*p2).tt_ as i32 == 3i32 | (1i32) << 4i32 {
                    n2_0 = (*p2).value_.n;
                    1i32
                } else {
                    if (*p2).tt_ as i32 == 3i32 | (0i32) << 4i32 {
                        n2_0 = (*p2).value_.i as f64;
                        1i32
                    } else {
                        0i32
                    }
                }) != 0
            {
                let mut io_2: *mut TValue = res;
                (*io_2).value_.n = numarith(L, op, n1_0, n2_0);
                (*io_2).tt_ = (3i32 | (1i32) << 4i32) as u8;
                return 1i32;
            } else {
                return 0i32;
            }
        }
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaO_arith(
    mut L: *mut lua_State,
    mut op: i32,
    mut p1: *const TValue,
    mut p2: *const TValue,
    mut res: StkId,
) {
    if luaO_rawarith(L, op, p1, p2, &mut (*res).val) == 0 {
        luaT_trybinTM(L, p1, p2, res, (op - 0i32 + TM_ADD as i32) as TMS);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaO_hexavalue(mut c: i32) -> i32 {
    if luai_ctype_[(c + 1i32) as usize] as i32 & (1i32) << 1i32 != 0 {
        return c - '0' as i32;
    } else {
        return (c | 'A' as i32 ^ 'a' as i32) - 'a' as i32 + 10i32;
    };
}
unsafe extern "C" fn isneg(mut s: *mut *const libc::c_char) -> i32 {
    if **s as i32 == '-' as i32 {
        *s = (*s).offset(1);
        return 1i32;
    } else if **s as i32 == '+' as i32 {
        *s = (*s).offset(1);
    }
    return 0i32;
}
unsafe extern "C" fn l_str2dloc(
    mut s: *const libc::c_char,
    mut result: *mut f64,
    mut mode: i32,
) -> *const libc::c_char {
    let mut endptr: *mut libc::c_char = 0 as *mut libc::c_char;
    *result = if mode == 'x' as i32 {
        strtod(s, &mut endptr)
    } else {
        strtod(s, &mut endptr)
    };
    if endptr == s as *mut libc::c_char {
        return 0 as *const libc::c_char;
    }
    while luai_ctype_[(*endptr as u8 as i32 + 1i32) as usize] as i32 & (1i32) << 3i32 != 0 {
        endptr = endptr.offset(1);
    }
    return if *endptr as i32 == '\0' as i32 {
        endptr
    } else {
        0 as *mut libc::c_char
    };
}
unsafe extern "C" fn l_str2d(
    mut s: *const libc::c_char,
    mut result: *mut f64,
) -> *const libc::c_char {
    let mut endptr: *const libc::c_char = 0 as *const libc::c_char;
    let mut pmode: *const libc::c_char = strpbrk(s, b".xXnN\0" as *const u8 as *const libc::c_char);
    let mut mode: i32 = if !pmode.is_null() {
        *pmode as u8 as i32 | 'A' as i32 ^ 'a' as i32
    } else {
        0i32
    };
    if mode == 'n' as i32 {
        return 0 as *const libc::c_char;
    }
    endptr = l_str2dloc(s, result, mode);
    if endptr.is_null() {
        let mut buff: [libc::c_char; 201] = [0; 201];
        let mut pdot: *const libc::c_char = strchr(s, '.' as i32);
        if pdot.is_null() || strlen(s) > 200i32 as libc::c_ulong {
            return 0 as *const libc::c_char;
        }
        strcpy(buff.as_mut_ptr(), s);
        buff[pdot.offset_from(s) as i64 as usize] =
            *((*localeconv()).decimal_point).offset(0i32 as isize);
        endptr = l_str2dloc(buff.as_mut_ptr(), result, mode);
        if !endptr.is_null() {
            endptr = s.offset(endptr.offset_from(buff.as_mut_ptr()) as i64 as isize);
        }
    }
    return endptr;
}
unsafe extern "C" fn l_str2int(
    mut s: *const libc::c_char,
    mut result: *mut i64,
) -> *const libc::c_char {
    let mut a: u64 = 0i32 as u64;
    let mut empty: i32 = 1i32;
    let mut neg: i32 = 0;
    while luai_ctype_[(*s as u8 as i32 + 1i32) as usize] as i32 & (1i32) << 3i32 != 0 {
        s = s.offset(1);
    }
    neg = isneg(&mut s);
    if *s.offset(0i32 as isize) as i32 == '0' as i32
        && (*s.offset(1i32 as isize) as i32 == 'x' as i32
            || *s.offset(1i32 as isize) as i32 == 'X' as i32)
    {
        s = s.offset(2i32 as isize);
        while luai_ctype_[(*s as u8 as i32 + 1i32) as usize] as i32 & (1i32) << 4i32 != 0 {
            a = a
                .wrapping_mul(16i32 as u64)
                .wrapping_add(luaO_hexavalue(*s as i32) as u64);
            empty = 0i32;
            s = s.offset(1);
        }
    } else {
        while luai_ctype_[(*s as u8 as i32 + 1i32) as usize] as i32 & (1i32) << 1i32 != 0 {
            let mut d: i32 = *s as i32 - '0' as i32;
            if a >= (9223372036854775807i64 / 10i32 as i64) as u64
                && (a > (9223372036854775807i64 / 10i32 as i64) as u64
                    || d > (9223372036854775807i64 % 10i32 as i64) as i32 + neg)
            {
                return 0 as *const libc::c_char;
            }
            a = a.wrapping_mul(10i32 as u64).wrapping_add(d as u64);
            empty = 0i32;
            s = s.offset(1);
        }
    }
    while luai_ctype_[(*s as u8 as i32 + 1i32) as usize] as i32 & (1i32) << 3i32 != 0 {
        s = s.offset(1);
    }
    if empty != 0 || *s as i32 != '\0' as i32 {
        return 0 as *const libc::c_char;
    } else {
        *result = (if neg != 0 {
            (0 as u32 as u64).wrapping_sub(a)
        } else {
            a
        }) as i64;
        return s;
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaO_str2num(mut s: *const libc::c_char, mut o: *mut TValue) -> u64 {
    let mut i: i64 = 0;
    let mut n: f64 = 0.;
    let mut e: *const libc::c_char = 0 as *const libc::c_char;
    e = l_str2int(s, &mut i);
    if !e.is_null() {
        let mut io: *mut TValue = o;
        (*io).value_.i = i;
        (*io).tt_ = (3i32 | (0i32) << 4i32) as u8;
    } else {
        e = l_str2d(s, &mut n);
        if !e.is_null() {
            let mut io_0: *mut TValue = o;
            (*io_0).value_.n = n;
            (*io_0).tt_ = (3i32 | (1i32) << 4i32) as u8;
        } else {
            return 0i32 as u64;
        }
    }
    return (e.offset_from(s) as i64 + 1i32 as i64) as u64;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaO_utf8esc(mut buff: *mut libc::c_char, mut x: libc::c_ulong) -> i32 {
    let mut n: i32 = 1i32;
    if x < 0x80 as i32 as libc::c_ulong {
        *buff.offset((8i32 - 1i32) as isize) = x as libc::c_char;
    } else {
        let mut mfb: u32 = 0x3f as i32 as u32;
        loop {
            let fresh0 = n;
            n = n + 1;
            *buff.offset((8i32 - fresh0) as isize) =
                (0x80 as i32 as libc::c_ulong | x & 0x3f as i32 as libc::c_ulong) as libc::c_char;
            x >>= 6i32;
            mfb >>= 1i32;
            if !(x > mfb as libc::c_ulong) {
                break;
            }
        }
        *buff.offset((8i32 - n) as isize) = ((!mfb << 1i32) as libc::c_ulong | x) as libc::c_char;
    }
    return n;
}
unsafe extern "C" fn tostringbuff(mut obj: *mut TValue, mut buff: *mut libc::c_char) -> i32 {
    let mut len: i32 = 0;
    if (*obj).tt_ as i32 == 3i32 | (0i32) << 4i32 {
        len = snprintf(
            buff,
            44i32 as libc::c_ulong,
            b"%lld\0" as *const u8 as *const libc::c_char,
            (*obj).value_.i,
        );
    } else {
        len = snprintf(
            buff,
            44i32 as libc::c_ulong,
            b"%.14g\0" as *const u8 as *const libc::c_char,
            (*obj).value_.n,
        );
        if *buff.offset(strspn(buff, b"-0123456789\0" as *const u8 as *const libc::c_char) as isize)
            as i32
            == '\0' as i32
        {
            let fresh1 = len;
            len = len + 1;
            *buff.offset(fresh1 as isize) = *((*localeconv()).decimal_point).offset(0i32 as isize);
            let fresh2 = len;
            len = len + 1;
            *buff.offset(fresh2 as isize) = '0' as i32 as libc::c_char;
        }
    }
    return len;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaO_tostring(mut L: *mut lua_State, mut obj: *mut TValue) {
    let mut buff: [libc::c_char; 44] = [0; 44];
    let mut len: i32 = tostringbuff(obj, buff.as_mut_ptr());
    let mut io: *mut TValue = obj;
    let mut x_: *mut TString = luaS_newlstr(L, buff.as_mut_ptr(), len as u64);
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
    (*io).tt_ = ((*x_).tt as i32 | (1i32) << 6i32) as u8;
}
unsafe extern "C" fn pushstr(mut buff: *mut BuffFS, mut str: *const libc::c_char, mut lstr: u64) {
    let mut L: *mut lua_State = (*buff).L;
    let mut io: *mut TValue = &mut (*(*L).top.p).val;
    let mut x_: *mut TString = luaS_newlstr(L, str, lstr);
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
    (*io).tt_ = ((*x_).tt as i32 | (1i32) << 6i32) as u8;
    (*L).top.p = ((*L).top.p).offset(1);
    (*L).top.p;
    if (*buff).pushed == 0 {
        (*buff).pushed = 1i32;
    } else {
        luaV_concat(L, 2i32);
    };
}
unsafe extern "C" fn clearbuff(mut buff: *mut BuffFS) {
    pushstr(buff, ((*buff).space).as_mut_ptr(), (*buff).blen as u64);
    (*buff).blen = 0i32;
}
unsafe extern "C" fn getbuff(mut buff: *mut BuffFS, mut sz: i32) -> *mut libc::c_char {
    if sz > 60i32 + 44i32 + 95i32 - (*buff).blen {
        clearbuff(buff);
    }
    return ((*buff).space).as_mut_ptr().offset((*buff).blen as isize);
}
unsafe extern "C" fn addstr2buff(
    mut buff: *mut BuffFS,
    mut str: *const libc::c_char,
    mut slen: u64,
) {
    if slen <= (60i32 + 44i32 + 95i32) as libc::c_ulong {
        let mut bf: *mut libc::c_char = getbuff(buff, slen as i32);
        memcpy(bf as *mut libc::c_void, str as *const libc::c_void, slen);
        (*buff).blen += slen as i32;
    } else {
        clearbuff(buff);
        pushstr(buff, str, slen);
    };
}
unsafe extern "C" fn addnum2buff(mut buff: *mut BuffFS, mut num: *mut TValue) {
    let mut numbuff: *mut libc::c_char = getbuff(buff, 44i32);
    let mut len: i32 = tostringbuff(num, numbuff);
    (*buff).blen += len;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaO_pushvfstring(
    mut L: *mut lua_State,
    mut fmt: *const libc::c_char,
    mut argp: ::core::ffi::VaList,
) -> *const libc::c_char {
    let mut buff: BuffFS = BuffFS {
        L: 0 as *mut lua_State,
        pushed: 0,
        blen: 0,
        space: [0; 199],
    };
    let mut e: *const libc::c_char = 0 as *const libc::c_char;
    buff.blen = 0i32;
    buff.pushed = buff.blen;
    buff.L = L;
    loop {
        e = strchr(fmt, '%' as i32);
        if e.is_null() {
            break;
        }
        addstr2buff(&mut buff, fmt, e.offset_from(fmt) as i64 as u64);
        match *e.offset(1i32 as isize) as i32 {
            115 => {
                let mut s: *const libc::c_char = argp.arg::<*mut libc::c_char>();
                if s.is_null() {
                    s = b"(null)\0" as *const u8 as *const libc::c_char;
                }
                addstr2buff(&mut buff, s, strlen(s));
            }
            99 => {
                let mut c: libc::c_char = argp.arg::<i32>() as u8 as libc::c_char;
                addstr2buff(
                    &mut buff,
                    &mut c,
                    ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                );
            }
            100 => {
                let mut num: TValue = TValue {
                    value_: Value {
                        gc: 0 as *mut GCObject,
                    },
                    tt_: 0,
                };
                let mut io: *mut TValue = &mut num;
                (*io).value_.i = argp.arg::<i32>() as i64;
                (*io).tt_ = (3i32 | (0i32) << 4i32) as u8;
                addnum2buff(&mut buff, &mut num);
            }
            73 => {
                let mut num_0: TValue = TValue {
                    value_: Value {
                        gc: 0 as *mut GCObject,
                    },
                    tt_: 0,
                };
                let mut io_0: *mut TValue = &mut num_0;
                (*io_0).value_.i = argp.arg::<l_uacInt>();
                (*io_0).tt_ = (3i32 | (0i32) << 4i32) as u8;
                addnum2buff(&mut buff, &mut num_0);
            }
            102 => {
                let mut num_1: TValue = TValue {
                    value_: Value {
                        gc: 0 as *mut GCObject,
                    },
                    tt_: 0,
                };
                let mut io_1: *mut TValue = &mut num_1;
                (*io_1).value_.n = argp.arg::<l_uacNumber>();
                (*io_1).tt_ = (3i32 | (1i32) << 4i32) as u8;
                addnum2buff(&mut buff, &mut num_1);
            }
            112 => {
                let sz: i32 = (3i32 as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                    .wrapping_add(8i32 as libc::c_ulong) as i32;
                let mut bf: *mut libc::c_char = getbuff(&mut buff, sz);
                let mut p: *mut libc::c_void = argp.arg::<*mut libc::c_void>();
                let mut len: i32 = snprintf(
                    bf,
                    sz as libc::c_ulong,
                    b"%p\0" as *const u8 as *const libc::c_char,
                    p,
                );
                buff.blen += len;
            }
            85 => {
                let mut bf_0: [libc::c_char; 8] = [0; 8];
                let mut len_0: i32 =
                    luaO_utf8esc(bf_0.as_mut_ptr(), argp.arg::<i64>() as libc::c_ulong);
                addstr2buff(
                    &mut buff,
                    bf_0.as_mut_ptr()
                        .offset(8i32 as isize)
                        .offset(-(len_0 as isize)),
                    len_0 as u64,
                );
            }
            37 => {
                addstr2buff(
                    &mut buff,
                    b"%\0" as *const u8 as *const libc::c_char,
                    1i32 as u64,
                );
            }
            _ => {
                luaG_runerror(
                    L,
                    b"invalid option '%%%c' to 'lua_pushfstring'\0" as *const u8
                        as *const libc::c_char,
                    *e.offset(1i32 as isize) as i32,
                );
            }
        }
        fmt = e.offset(2i32 as isize);
    }
    addstr2buff(&mut buff, fmt, strlen(fmt));
    clearbuff(&mut buff);
    return ((*((*((*L).top.p).offset(-(1i32 as isize))).val.value_.gc as *mut GCUnion))
        .ts
        .contents)
        .as_mut_ptr();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaO_pushfstring(
    mut L: *mut lua_State,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> *const libc::c_char {
    let mut msg: *const libc::c_char = 0 as *const libc::c_char;
    let mut argp: ::core::ffi::VaListImpl;
    argp = args.clone();
    msg = luaO_pushvfstring(L, fmt, argp.as_va_list());
    return msg;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaO_chunkid(
    mut out: *mut libc::c_char,
    mut source: *const libc::c_char,
    mut srclen: u64,
) {
    let mut bufflen: u64 = 60i32 as u64;
    if *source as i32 == '=' as i32 {
        if srclen <= bufflen {
            memcpy(
                out as *mut libc::c_void,
                source.offset(1i32 as isize) as *const libc::c_void,
                srclen.wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
            );
        } else {
            memcpy(
                out as *mut libc::c_void,
                source.offset(1i32 as isize) as *const libc::c_void,
                bufflen
                    .wrapping_sub(1i32 as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
            );
            out = out.offset(bufflen.wrapping_sub(1i32 as libc::c_ulong) as isize);
            *out = '\0' as i32 as libc::c_char;
        }
    } else if *source as i32 == '@' as i32 {
        if srclen <= bufflen {
            memcpy(
                out as *mut libc::c_void,
                source.offset(1i32 as isize) as *const libc::c_void,
                srclen.wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
            );
        } else {
            memcpy(
                out as *mut libc::c_void,
                b"...\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                (::core::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    .wrapping_sub(1i32 as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
            );
            out = out.offset(
                (::core::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    .wrapping_sub(1i32 as libc::c_ulong) as isize,
            );
            bufflen = (bufflen as libc::c_ulong).wrapping_sub(
                (::core::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    .wrapping_sub(1i32 as libc::c_ulong),
            ) as u64 as u64;
            memcpy(
                out as *mut libc::c_void,
                source
                    .offset(1i32 as isize)
                    .offset(srclen as isize)
                    .offset(-(bufflen as isize)) as *const libc::c_void,
                bufflen.wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
            );
        }
    } else {
        let mut nl: *const libc::c_char = strchr(source, '\n' as i32);
        memcpy(
            out as *mut libc::c_void,
            b"[string \"\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            (::core::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_sub(1i32 as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
        );
        out = out.offset(
            (::core::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_sub(1i32 as libc::c_ulong) as isize,
        );
        bufflen = (bufflen as libc::c_ulong).wrapping_sub(
            (::core::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_sub(1i32 as libc::c_ulong)
                .wrapping_add(1i32 as libc::c_ulong),
        ) as u64 as u64;
        if srclen < bufflen && nl.is_null() {
            memcpy(
                out as *mut libc::c_void,
                source as *const libc::c_void,
                srclen.wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
            );
            out = out.offset(srclen as isize);
        } else {
            if !nl.is_null() {
                srclen = nl.offset_from(source) as i64 as u64;
            }
            if srclen > bufflen {
                srclen = bufflen;
            }
            memcpy(
                out as *mut libc::c_void,
                source as *const libc::c_void,
                srclen.wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
            );
            out = out.offset(srclen as isize);
            memcpy(
                out as *mut libc::c_void,
                b"...\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                (::core::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    .wrapping_sub(1i32 as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
            );
            out = out.offset(
                (::core::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    .wrapping_sub(1i32 as libc::c_ulong) as isize,
            );
        }
        memcpy(
            out as *mut libc::c_void,
            b"\"]\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            (::core::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_sub(1i32 as libc::c_ulong)
                .wrapping_add(1i32 as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
        );
    };
}
