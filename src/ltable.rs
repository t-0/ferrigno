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
    fn frexp(_: f64, _: *mut i32) -> f64;
    fn luaO_ceillog2(x: u32) -> i32;
    fn luaM_realloc_(
        L: *mut lua_State,
        block: *mut libc::c_void,
        oldsize: u64,
        size: u64,
    ) -> *mut libc::c_void;
    fn luaM_free_(L: *mut lua_State, block: *mut libc::c_void, osize: u64);
    fn luaM_malloc_(L: *mut lua_State, size: u64, tag: i32) -> *mut libc::c_void;
    fn luaG_runerror(L: *mut lua_State, fmt: *const libc::c_char, _: ...) -> !;
    fn luaD_throw(L: *mut lua_State, errcode: i32) -> !;
    fn luaC_newobj(L: *mut lua_State, tt: i32, sz: u64) -> *mut GCObject;
    fn luaC_barrierback_(L: *mut lua_State, o: *mut GCObject);
    fn luaS_hashlongstr(ts: *mut TString) -> u32;
    fn luaS_eqlngstr(a: *mut TString, b: *mut TString) -> i32;
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
    pub u: LTableC2RustUnnamed_1,
    pub u2: LTableC2RustUnnamed,
    pub nresults: libc::c_short,
    pub callstatus: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union LTableC2RustUnnamed {
    pub funcidx: i32,
    pub nyield: i32,
    pub nres: i32,
    pub transferinfo: LTableC2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LTableC2RustUnnamed_0 {
    pub ftransfer: libc::c_ushort,
    pub ntransfer: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union LTableC2RustUnnamed_1 {
    pub l: LTableC2RustUnnamed_3,
    pub c: LTableC2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LTableC2RustUnnamed_2 {
    pub k: lua_KFunction,
    pub old_errfunc: i64,
    pub ctx: lua_KContext,
}
pub type lua_KFunction = Option<unsafe extern "C" fn(*mut lua_State, i32, lua_KContext) -> i32>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LTableC2RustUnnamed_3 {
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
    pub tbclist: LTableC2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LTableC2RustUnnamed_4 {
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
    pub v: LTableC2RustUnnamed_7,
    pub u: LTableC2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union LTableC2RustUnnamed_5 {
    pub open: LTableC2RustUnnamed_6,
    pub value: TValue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LTableC2RustUnnamed_6 {
    pub next: *mut UpVal,
    pub previous: *mut *mut UpVal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union LTableC2RustUnnamed_7 {
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
    pub u: LTableC2RustUnnamed_8,
    pub contents: [libc::c_char; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union LTableC2RustUnnamed_8 {
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
pub type LTableC2RustUnnamed_9 = u32;
pub const TM_N: LTableC2RustUnnamed_9 = 25;
pub const TM_CLOSE: LTableC2RustUnnamed_9 = 24;
pub const TM_CALL: LTableC2RustUnnamed_9 = 23;
pub const TM_CONCAT: LTableC2RustUnnamed_9 = 22;
pub const TM_LE: LTableC2RustUnnamed_9 = 21;
pub const TM_LT: LTableC2RustUnnamed_9 = 20;
pub const TM_BNOT: LTableC2RustUnnamed_9 = 19;
pub const TM_UNM: LTableC2RustUnnamed_9 = 18;
pub const TM_SHR: LTableC2RustUnnamed_9 = 17;
pub const TM_SHL: LTableC2RustUnnamed_9 = 16;
pub const TM_BXOR: LTableC2RustUnnamed_9 = 15;
pub const TM_BOR: LTableC2RustUnnamed_9 = 14;
pub const TM_BAND: LTableC2RustUnnamed_9 = 13;
pub const TM_IDIV: LTableC2RustUnnamed_9 = 12;
pub const TM_DIV: LTableC2RustUnnamed_9 = 11;
pub const TM_POW: LTableC2RustUnnamed_9 = 10;
pub const TM_MOD: LTableC2RustUnnamed_9 = 9;
pub const TM_MUL: LTableC2RustUnnamed_9 = 8;
pub const TM_SUB: LTableC2RustUnnamed_9 = 7;
pub const TM_ADD: LTableC2RustUnnamed_9 = 6;
pub const TM_EQ: LTableC2RustUnnamed_9 = 5;
pub const TM_LEN: LTableC2RustUnnamed_9 = 4;
pub const TM_MODE: LTableC2RustUnnamed_9 = 3;
pub const TM_GC: LTableC2RustUnnamed_9 = 2;
pub const TM_NEWINDEX: LTableC2RustUnnamed_9 = 1;
pub const TM_INDEX: LTableC2RustUnnamed_9 = 0;
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
static mut dummynode_: Node = Node {
    u: {
        let mut init = NodeKey {
            value_: Value {
                gc: 0 as *const GCObject as *mut GCObject,
            },
            tt_: (0i32 | (1i32) << 4i32) as u8,
            key_tt: (0i32 | (0i32) << 4i32) as u8,
            next: 0i32,
            key_val: Value {
                gc: 0 as *const GCObject as *mut GCObject,
            },
        };
        init
    },
};
static mut absentkey: TValue = {
    let mut init = TValue {
        value_: Value {
            gc: 0 as *const GCObject as *mut GCObject,
        },
        tt_: (0i32 | (2i32) << 4i32) as u8,
    };
    init
};
unsafe extern "C" fn hashint(mut t: *const Table, mut i: i64) -> *mut Node {
    let mut ui: u64 = i as u64;
    if ui <= 2147483647i32 as u32 as u64 {
        return &mut *((*t).node)
            .offset((ui as i32 % (((1i32) << (*t).lsizenode as i32) - 1i32 | 1i32)) as isize)
            as *mut Node;
    } else {
        return &mut *((*t).node).offset(
            ui.wrapping_rem((((1i32) << (*t).lsizenode as i32) - 1i32 | 1i32) as u64) as isize,
        ) as *mut Node;
    };
}
unsafe extern "C" fn l_hashfloat(mut n: f64) -> i32 {
    let mut i: i32 = 0;
    let mut ni: i64 = 0;
    n = frexp(n, &mut i) * -((-(2147483647i32) - 1i32) as f64);
    if !(n >= (-(9223372036854775807i64) - 1i64) as f64
        && n < -((-(9223372036854775807i64) - 1i64) as f64)
        && {
            ni = n as i64;
            1i32 != 0
        })
    {
        return 0i32;
    } else {
        let mut u: u32 = (i as u32).wrapping_add(ni as u32);
        return (if u <= 2147483647i32 as u32 { u } else { !u }) as i32;
    };
}
unsafe extern "C" fn mainpositionTV(mut t: *const Table, mut key: *const TValue) -> *mut Node {
    match (*key).tt_ as i32 & 0x3f as i32 {
        3 => {
            let mut i: i64 = (*key).value_.i;
            return hashint(t, i);
        }
        19 => {
            let mut n: f64 = (*key).value_.n;
            return &mut *((*t).node).offset(
                ((l_hashfloat as unsafe extern "C" fn(f64) -> i32)(n)
                    % (((1i32) << (*t).lsizenode as i32) - 1i32 | 1i32)) as isize,
            ) as *mut Node;
        }
        4 => {
            let mut ts: *mut TString = &mut (*((*key).value_.gc as *mut GCUnion)).ts;
            return &mut *((*t).node).offset(
                ((*ts).hash & (((1i32) << (*t).lsizenode as i32) - 1i32) as u32) as i32 as isize,
            ) as *mut Node;
        }
        20 => {
            let mut ts_0: *mut TString = &mut (*((*key).value_.gc as *mut GCUnion)).ts;
            return &mut *((*t).node).offset(
                ((luaS_hashlongstr as unsafe extern "C" fn(*mut TString) -> u32)(ts_0)
                    & (((1i32) << (*t).lsizenode as i32) - 1i32) as u32) as i32
                    as isize,
            ) as *mut Node;
        }
        1 => {
            return &mut *((*t).node)
                .offset((0i32 & ((1i32) << (*t).lsizenode as i32) - 1i32) as isize)
                as *mut Node;
        }
        17 => {
            return &mut *((*t).node)
                .offset((1i32 & ((1i32) << (*t).lsizenode as i32) - 1i32) as isize)
                as *mut Node;
        }
        2 => {
            let mut p: *mut libc::c_void = (*key).value_.p;
            return &mut *((*t).node).offset(
                ((p as u64
                    & (2147483647i32 as u32)
                        .wrapping_mul(2 as u32)
                        .wrapping_add(1 as u32) as libc::c_ulong) as u32)
                    .wrapping_rem((((1i32) << (*t).lsizenode as i32) - 1i32 | 1i32) as u32)
                    as isize,
            ) as *mut Node;
        }
        22 => {
            let mut f: CFunction = (*key).value_.f;
            return &mut *((*t).node).offset(
                ((::core::mem::transmute::<CFunction, u64>(f)
                    & (2147483647i32 as u32)
                        .wrapping_mul(2 as u32)
                        .wrapping_add(1 as u32) as libc::c_ulong) as u32)
                    .wrapping_rem((((1i32) << (*t).lsizenode as i32) - 1i32 | 1i32) as u32)
                    as isize,
            ) as *mut Node;
        }
        _ => {
            let mut o: *mut GCObject = (*key).value_.gc;
            return &mut *((*t).node).offset(
                ((o as u64
                    & (2147483647i32 as u32)
                        .wrapping_mul(2 as u32)
                        .wrapping_add(1 as u32) as libc::c_ulong) as u32)
                    .wrapping_rem((((1i32) << (*t).lsizenode as i32) - 1i32 | 1i32) as u32)
                    as isize,
            ) as *mut Node;
        }
    };
}
#[inline]
unsafe extern "C" fn mainpositionfromnode(mut t: *const Table, mut nd: *mut Node) -> *mut Node {
    let mut key: TValue = TValue {
        value_: Value {
            gc: 0 as *mut GCObject,
        },
        tt_: 0,
    };
    let mut io_: *mut TValue = &mut key;
    let mut n_: *const Node = nd;
    (*io_).value_ = (*n_).u.key_val;
    (*io_).tt_ = (*n_).u.key_tt;
    return mainpositionTV(t, &mut key);
}
unsafe extern "C" fn equalkey(mut k1: *const TValue, mut n2: *const Node, mut deadok: i32) -> i32 {
    if (*k1).tt_ as i32 != (*n2).u.key_tt as i32
        && !(deadok != 0
            && (*n2).u.key_tt as i32 == 9i32 + 2i32
            && (*k1).tt_ as i32 & (1i32) << 6i32 != 0)
    {
        return 0i32;
    }
    match (*n2).u.key_tt as i32 {
        0 | 1 | 17 => return 1i32,
        3 => return ((*k1).value_.i == (*n2).u.key_val.i) as i32,
        19 => return ((*k1).value_.n == (*n2).u.key_val.n) as i32,
        2 => return ((*k1).value_.p == (*n2).u.key_val.p) as i32,
        22 => return ((*k1).value_.f == (*n2).u.key_val.f) as i32,
        84 => {
            return luaS_eqlngstr(
                &mut (*((*k1).value_.gc as *mut GCUnion)).ts,
                &mut (*((*n2).u.key_val.gc as *mut GCUnion)).ts,
            );
        }
        _ => return ((*k1).value_.gc == (*n2).u.key_val.gc) as i32,
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaH_realasize(mut t: *const Table) -> u32 {
    if (*t).flags as i32 & (1i32) << 7i32 == 0
        || (*t).alimit & ((*t).alimit).wrapping_sub(1i32 as u32) == 0i32 as u32
    {
        return (*t).alimit;
    } else {
        let mut size: u32 = (*t).alimit;
        size |= size >> 1i32;
        size |= size >> 2i32;
        size |= size >> 4i32;
        size |= size >> 8i32;
        size |= size >> 16i32;
        size = size.wrapping_add(1);
        return size;
    };
}
unsafe extern "C" fn ispow2realasize(mut t: *const Table) -> i32 {
    return ((*t).flags as i32 & (1i32) << 7i32 != 0
        || (*t).alimit & ((*t).alimit).wrapping_sub(1i32 as u32) == 0i32 as u32) as i32;
}
unsafe extern "C" fn setlimittosize(mut t: *mut Table) -> u32 {
    (*t).alimit = luaH_realasize(t);
    (*t).flags = ((*t).flags as i32 & !((1i32) << 7i32) as u8 as i32) as u8;
    return (*t).alimit;
}
unsafe extern "C" fn getgeneric(
    mut t: *mut Table,
    mut key: *const TValue,
    mut deadok: i32,
) -> *const TValue {
    let mut n: *mut Node = mainpositionTV(t, key);
    loop {
        if equalkey(key, n, deadok) != 0 {
            return &mut (*n).i_val;
        } else {
            let mut nx: i32 = (*n).u.next;
            if nx == 0i32 {
                return &absentkey;
            }
            n = n.offset(nx as isize);
        }
    }
}
unsafe extern "C" fn arrayindex(mut k: i64) -> u32 {
    if (k as u64).wrapping_sub(1 as u32 as u64)
        < (if ((1 as u32)
            << (::core::mem::size_of::<i32>() as libc::c_ulong)
                .wrapping_mul(8i32 as libc::c_ulong)
                .wrapping_sub(1i32 as libc::c_ulong) as i32) as u64
            <= (!(0i32 as u64)).wrapping_div(::core::mem::size_of::<TValue>() as libc::c_ulong)
        {
            (1 as u32)
                << (::core::mem::size_of::<i32>() as libc::c_ulong)
                    .wrapping_mul(8i32 as libc::c_ulong)
                    .wrapping_sub(1i32 as libc::c_ulong) as i32
        } else {
            (!(0i32 as u64)).wrapping_div(::core::mem::size_of::<TValue>() as libc::c_ulong) as u32
        }) as u64
    {
        return k as u32;
    } else {
        return 0i32 as u32;
    };
}
unsafe extern "C" fn findindex(
    mut L: *mut lua_State,
    mut t: *mut Table,
    mut key: *mut TValue,
    mut asize: u32,
) -> u32 {
    let mut i: u32 = 0;
    if (*key).tt_ as i32 & 0xf as i32 == 0i32 {
        return 0i32 as u32;
    }
    i = if (*key).tt_ as i32 == 3i32 | (0i32) << 4i32 {
        arrayindex((*key).value_.i)
    } else {
        0i32 as u32
    };
    if i.wrapping_sub(1 as u32) < asize {
        return i;
    } else {
        let mut n: *const TValue = getgeneric(t, key, 1i32);
        if (((*n).tt_ as i32 == 0i32 | (2i32) << 4i32) as i32 != 0i32) as i32 as i64 != 0 {
            luaG_runerror(
                L,
                b"invalid key to 'next'\0" as *const u8 as *const libc::c_char,
            );
        }
        i = (n as *mut Node).offset_from(&mut *((*t).node).offset(0i32 as isize) as *mut Node)
            as i64 as i32 as u32;
        return i.wrapping_add(1i32 as u32).wrapping_add(asize);
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaH_next(
    mut L: *mut lua_State,
    mut t: *mut Table,
    mut key: StkId,
) -> i32 {
    let mut asize: u32 = luaH_realasize(t);
    let mut i: u32 = findindex(L, t, &mut (*key).val, asize);
    while i < asize {
        if !((*((*t).array).offset(i as isize)).tt_ as i32 & 0xf as i32 == 0i32) {
            let mut io: *mut TValue = &mut (*key).val;
            (*io).value_.i = i.wrapping_add(1i32 as u32) as i64;
            (*io).tt_ = (3i32 | (0i32) << 4i32) as u8;
            let mut io1: *mut TValue = &mut (*key.offset(1i32 as isize)).val;
            let mut io2: *const TValue = &mut *((*t).array).offset(i as isize) as *mut TValue;
            (*io1).value_ = (*io2).value_;
            (*io1).tt_ = (*io2).tt_;
            return 1i32;
        }
        i = i.wrapping_add(1);
    }
    i = i.wrapping_sub(asize);
    while (i as i32) < (1i32) << (*t).lsizenode as i32 {
        if !((*((*t).node).offset(i as isize)).i_val.tt_ as i32 & 0xf as i32 == 0i32) {
            let mut n: *mut Node = &mut *((*t).node).offset(i as isize) as *mut Node;
            let mut io_: *mut TValue = &mut (*key).val;
            let mut n_: *const Node = n;
            (*io_).value_ = (*n_).u.key_val;
            (*io_).tt_ = (*n_).u.key_tt;
            let mut io1_0: *mut TValue = &mut (*key.offset(1i32 as isize)).val;
            let mut io2_0: *const TValue = &mut (*n).i_val;
            (*io1_0).value_ = (*io2_0).value_;
            (*io1_0).tt_ = (*io2_0).tt_;
            return 1i32;
        }
        i = i.wrapping_add(1);
    }
    return 0i32;
}
unsafe extern "C" fn freehash(mut L: *mut lua_State, mut t: *mut Table) {
    if !((*t).lastfree).is_null() {
        luaM_free_(
            L,
            (*t).node as *mut libc::c_void,
            (((1i32) << (*t).lsizenode as i32) as u64)
                .wrapping_mul(::core::mem::size_of::<Node>() as libc::c_ulong),
        );
    }
}
unsafe extern "C" fn computesizes(mut nums: *mut u32, mut pna: *mut u32) -> u32 {
    let mut i: i32 = 0;
    let mut twotoi: u32 = 0;
    let mut a: u32 = 0i32 as u32;
    let mut na: u32 = 0i32 as u32;
    let mut optimal: u32 = 0i32 as u32;
    i = 0i32;
    twotoi = 1i32 as u32;
    while twotoi > 0i32 as u32 && *pna > twotoi.wrapping_div(2i32 as u32) {
        a = a.wrapping_add(*nums.offset(i as isize));
        if a > twotoi.wrapping_div(2i32 as u32) {
            optimal = twotoi;
            na = a;
        }
        i += 1;
        twotoi = twotoi.wrapping_mul(2i32 as u32);
    }
    *pna = na;
    return optimal;
}
unsafe extern "C" fn countint(mut key: i64, mut nums: *mut u32) -> i32 {
    let mut k: u32 = arrayindex(key);
    if k != 0i32 as u32 {
        let ref mut fresh0 = *nums.offset(luaO_ceillog2(k) as isize);
        *fresh0 = (*fresh0).wrapping_add(1);
        return 1i32;
    } else {
        return 0i32;
    };
}
unsafe extern "C" fn numusearray(mut t: *const Table, mut nums: *mut u32) -> u32 {
    let mut lg: i32 = 0;
    let mut ttlg: u32 = 0;
    let mut ause: u32 = 0i32 as u32;
    let mut i: u32 = 1i32 as u32;
    let mut asize: u32 = (*t).alimit;
    lg = 0i32;
    ttlg = 1i32 as u32;
    while lg
        <= (::core::mem::size_of::<i32>() as libc::c_ulong)
            .wrapping_mul(8i32 as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong) as i32
    {
        let mut lc: u32 = 0i32 as u32;
        let mut lim: u32 = ttlg;
        if lim > asize {
            lim = asize;
            if i > lim {
                break;
            }
        }
        while i <= lim {
            if !((*((*t).array).offset(i.wrapping_sub(1i32 as u32) as isize)).tt_ as i32
                & 0xf as i32
                == 0i32)
            {
                lc = lc.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
        let ref mut fresh1 = *nums.offset(lg as isize);
        *fresh1 = (*fresh1).wrapping_add(lc);
        ause = ause.wrapping_add(lc);
        lg += 1;

        ttlg = ttlg.wrapping_mul(2i32 as u32);
    }
    return ause;
}
unsafe extern "C" fn numusehash(mut t: *const Table, mut nums: *mut u32, mut pna: *mut u32) -> i32 {
    let mut totaluse: i32 = 0i32;
    let mut ause: i32 = 0i32;
    let mut i: i32 = (1i32) << (*t).lsizenode as i32;
    loop {
        let fresh2 = i;
        i = i - 1;
        if !(fresh2 != 0) {
            break;
        }
        let mut n: *mut Node = &mut *((*t).node).offset(i as isize) as *mut Node;
        if !((*n).i_val.tt_ as i32 & 0xf as i32 == 0i32) {
            if (*n).u.key_tt as i32 == 3i32 | (0i32) << 4i32 {
                ause += countint((*n).u.key_val.i, nums);
            }
            totaluse += 1;
        }
    }
    *pna = (*pna).wrapping_add(ause as u32);
    return totaluse;
}
unsafe extern "C" fn setnodevector(mut L: *mut lua_State, mut t: *mut Table, mut size: u32) {
    if size == 0i32 as u32 {
        (*t).node = &dummynode_ as *const Node as *mut Node;
        (*t).lsizenode = 0i32 as u8;
        (*t).lastfree = 0 as *mut Node;
    } else {
        let mut i: i32 = 0;
        let mut lsize: i32 = luaO_ceillog2(size);
        if lsize
            > (::core::mem::size_of::<i32>() as libc::c_ulong)
                .wrapping_mul(8i32 as libc::c_ulong)
                .wrapping_sub(1i32 as libc::c_ulong) as i32
                - 1i32
            || (1 as u32) << lsize
                > (if ((1 as u32)
                    << (::core::mem::size_of::<i32>() as libc::c_ulong)
                        .wrapping_mul(8i32 as libc::c_ulong)
                        .wrapping_sub(1i32 as libc::c_ulong) as i32
                        - 1i32) as u64
                    <= (!(0i32 as u64))
                        .wrapping_div(::core::mem::size_of::<Node>() as libc::c_ulong)
                {
                    (1 as u32)
                        << (::core::mem::size_of::<i32>() as libc::c_ulong)
                            .wrapping_mul(8i32 as libc::c_ulong)
                            .wrapping_sub(1i32 as libc::c_ulong) as i32
                            - 1i32
                } else {
                    (!(0i32 as u64)).wrapping_div(::core::mem::size_of::<Node>() as libc::c_ulong)
                        as u32
                })
        {
            luaG_runerror(L, b"table overflow\0" as *const u8 as *const libc::c_char);
        }
        size = ((1i32) << lsize) as u32;
        (*t).node = luaM_malloc_(
            L,
            (size as libc::c_ulong).wrapping_mul(::core::mem::size_of::<Node>() as libc::c_ulong),
            0i32,
        ) as *mut Node;
        i = 0i32;
        while i < size as i32 {
            let mut n: *mut Node = &mut *((*t).node).offset(i as isize) as *mut Node;
            (*n).u.next = 0i32;
            (*n).u.key_tt = 0i32 as u8;
            (*n).i_val.tt_ = (0i32 | (1i32) << 4i32) as u8;
            i += 1;
        }
        (*t).lsizenode = lsize as u8;
        (*t).lastfree = &mut *((*t).node).offset(size as isize) as *mut Node;
    };
}
unsafe extern "C" fn reinsert(mut L: *mut lua_State, mut ot: *mut Table, mut t: *mut Table) {
    let mut j: i32 = 0;
    let mut size: i32 = (1i32) << (*ot).lsizenode as i32;
    j = 0i32;
    while j < size {
        let mut old: *mut Node = &mut *((*ot).node).offset(j as isize) as *mut Node;
        if !((*old).i_val.tt_ as i32 & 0xf as i32 == 0i32) {
            let mut k: TValue = TValue {
                value_: Value {
                    gc: 0 as *mut GCObject,
                },
                tt_: 0,
            };
            let mut io_: *mut TValue = &mut k;
            let mut n_: *const Node = old;
            (*io_).value_ = (*n_).u.key_val;
            (*io_).tt_ = (*n_).u.key_tt;
            luaH_set(L, t, &mut k, &mut (*old).i_val);
        }
        j += 1;
    }
}
unsafe extern "C" fn exchangehashpart(mut t1: *mut Table, mut t2: *mut Table) {
    let mut lsizenode: u8 = (*t1).lsizenode;
    let mut node: *mut Node = (*t1).node;
    let mut lastfree: *mut Node = (*t1).lastfree;
    (*t1).lsizenode = (*t2).lsizenode;
    (*t1).node = (*t2).node;
    (*t1).lastfree = (*t2).lastfree;
    (*t2).lsizenode = lsizenode;
    (*t2).node = node;
    (*t2).lastfree = lastfree;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaH_resize(
    mut L: *mut lua_State,
    mut t: *mut Table,
    mut newasize: u32,
    mut nhsize: u32,
) {
    let mut i: u32 = 0;
    let mut newt: Table = Table {
        next: 0 as *mut GCObject,
        tt: 0,
        marked: 0,
        flags: 0,
        lsizenode: 0,
        alimit: 0,
        array: 0 as *mut TValue,
        node: 0 as *mut Node,
        lastfree: 0 as *mut Node,
        metatable: 0 as *mut Table,
        gclist: 0 as *mut GCObject,
    };
    let mut oldasize: u32 = setlimittosize(t);
    let mut newarray: *mut TValue = 0 as *mut TValue;
    setnodevector(L, &mut newt, nhsize);
    if newasize < oldasize {
        (*t).alimit = newasize;
        exchangehashpart(t, &mut newt);
        i = newasize;
        while i < oldasize {
            if !((*((*t).array).offset(i as isize)).tt_ as i32 & 0xf as i32 == 0i32) {
                luaH_setint(
                    L,
                    t,
                    i.wrapping_add(1i32 as u32) as i64,
                    &mut *((*t).array).offset(i as isize),
                );
            }
            i = i.wrapping_add(1);
        }
        (*t).alimit = oldasize;
        exchangehashpart(t, &mut newt);
    }
    newarray = luaM_realloc_(
        L,
        (*t).array as *mut libc::c_void,
        (oldasize as u64).wrapping_mul(::core::mem::size_of::<TValue>() as libc::c_ulong),
        (newasize as u64).wrapping_mul(::core::mem::size_of::<TValue>() as libc::c_ulong),
    ) as *mut TValue;
    if ((newarray.is_null() && newasize > 0i32 as u32) as i32 != 0i32) as i32 as i64 != 0 {
        freehash(L, &mut newt);
        luaD_throw(L, 4i32);
    }
    exchangehashpart(t, &mut newt);
    (*t).array = newarray;
    (*t).alimit = newasize;
    i = oldasize;
    while i < newasize {
        (*((*t).array).offset(i as isize)).tt_ = (0i32 | (1i32) << 4i32) as u8;
        i = i.wrapping_add(1);
    }
    reinsert(L, &mut newt, t);
    freehash(L, &mut newt);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaH_resizearray(
    mut L: *mut lua_State,
    mut t: *mut Table,
    mut nasize: u32,
) {
    let mut nsize: i32 = if ((*t).lastfree).is_null() {
        0i32
    } else {
        (1i32) << (*t).lsizenode as i32
    };
    luaH_resize(L, t, nasize, nsize as u32);
}
unsafe extern "C" fn rehash(mut L: *mut lua_State, mut t: *mut Table, mut ek: *const TValue) {
    let mut asize: u32 = 0;
    let mut na: u32 = 0;
    let mut nums: [u32; 32] = [0; 32];
    let mut i: i32 = 0;
    let mut totaluse: i32 = 0;
    i = 0i32;
    while i
        <= (::core::mem::size_of::<i32>() as libc::c_ulong)
            .wrapping_mul(8i32 as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong) as i32
    {
        nums[i as usize] = 0i32 as u32;
        i += 1;
    }
    setlimittosize(t);
    na = numusearray(t, nums.as_mut_ptr());
    totaluse = na as i32;
    totaluse += numusehash(t, nums.as_mut_ptr(), &mut na);
    if (*ek).tt_ as i32 == 3i32 | (0i32) << 4i32 {
        na = na.wrapping_add(countint((*ek).value_.i, nums.as_mut_ptr()) as u32);
    }
    totaluse += 1;
    asize = computesizes(nums.as_mut_ptr(), &mut na);
    luaH_resize(L, t, asize, (totaluse as u32).wrapping_sub(na));
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaH_new(mut L: *mut lua_State) -> *mut Table {
    let mut o: *mut GCObject = luaC_newobj(
        L,
        5i32 | (0i32) << 4i32,
        ::core::mem::size_of::<Table>() as libc::c_ulong,
    );
    let mut t: *mut Table = &mut (*(o as *mut GCUnion)).h;
    (*t).metatable = 0 as *mut Table;
    (*t).flags = !(!(0 as u32) << TM_EQ as i32 + 1i32) as u8;
    (*t).array = 0 as *mut TValue;
    (*t).alimit = 0i32 as u32;
    setnodevector(L, t, 0i32 as u32);
    return t;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaH_free(mut L: *mut lua_State, mut t: *mut Table) {
    freehash(L, t);
    luaM_free_(
        L,
        (*t).array as *mut libc::c_void,
        (luaH_realasize(t) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<TValue>() as libc::c_ulong),
    );
    luaM_free_(
        L,
        t as *mut libc::c_void,
        ::core::mem::size_of::<Table>() as libc::c_ulong,
    );
}
unsafe extern "C" fn getfreepos(mut t: *mut Table) -> *mut Node {
    if !((*t).lastfree).is_null() {
        while (*t).lastfree > (*t).node {
            (*t).lastfree = ((*t).lastfree).offset(-1);
            (*t).lastfree;
            if (*(*t).lastfree).u.key_tt as i32 == 0i32 {
                return (*t).lastfree;
            }
        }
    }
    return 0 as *mut Node;
}
unsafe extern "C" fn luaH_newkey(
    mut L: *mut lua_State,
    mut t: *mut Table,
    mut key: *const TValue,
    mut value: *mut TValue,
) {
    let mut mp: *mut Node = 0 as *mut Node;
    let mut aux: TValue = TValue {
        value_: Value {
            gc: 0 as *mut GCObject,
        },
        tt_: 0,
    };
    if (((*key).tt_ as i32 & 0xf as i32 == 0i32) as i32 != 0i32) as i32 as i64 != 0 {
        luaG_runerror(
            L,
            b"table index is nil\0" as *const u8 as *const libc::c_char,
        );
    } else if (*key).tt_ as i32 == 3i32 | (1i32) << 4i32 {
        let mut f: f64 = (*key).value_.n;
        let mut k: i64 = 0;
        if luaV_flttointeger(f, &mut k, F2Ieq) != 0 {
            let mut io: *mut TValue = &mut aux;
            (*io).value_.i = k;
            (*io).tt_ = (3i32 | (0i32) << 4i32) as u8;
            key = &mut aux;
        } else if (!(f == f) as i32 != 0i32) as i32 as i64 != 0 {
            luaG_runerror(
                L,
                b"table index is NaN\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    if (*value).tt_ as i32 & 0xf as i32 == 0i32 {
        return;
    }
    mp = mainpositionTV(t, key);
    if !((*mp).i_val.tt_ as i32 & 0xf as i32 == 0i32) || ((*t).lastfree).is_null() {
        let mut othern: *mut Node = 0 as *mut Node;
        let mut f_0: *mut Node = getfreepos(t);
        if f_0.is_null() {
            rehash(L, t, key);
            luaH_set(L, t, key, value);
            return;
        }
        othern = mainpositionfromnode(t, mp);
        if othern != mp {
            while othern.offset((*othern).u.next as isize) != mp {
                othern = othern.offset((*othern).u.next as isize);
            }
            (*othern).u.next = f_0.offset_from(othern) as i64 as i32;
            *f_0 = *mp;
            if (*mp).u.next != 0i32 {
                (*f_0).u.next += mp.offset_from(f_0) as i64 as i32;
                (*mp).u.next = 0i32;
            }
            (*mp).i_val.tt_ = (0i32 | (1i32) << 4i32) as u8;
        } else {
            if (*mp).u.next != 0i32 {
                (*f_0).u.next = mp.offset((*mp).u.next as isize).offset_from(f_0) as i64 as i32;
            }
            (*mp).u.next = f_0.offset_from(mp) as i64 as i32;
            mp = f_0;
        }
    }
    let mut n_: *mut Node = mp;
    let mut io_: *const TValue = key;
    (*n_).u.key_val = (*io_).value_;
    (*n_).u.key_tt = (*io_).tt_;
    if (*key).tt_ as i32 & (1i32) << 6i32 != 0 {
        if (*(t as *mut GCUnion)).gc.marked as i32 & (1i32) << 5i32 != 0
            && (*(*key).value_.gc).marked as i32 & ((1i32) << 3i32 | (1i32) << 4i32) != 0
        {
            luaC_barrierback_(L, &mut (*(t as *mut GCUnion)).gc);
        } else {
        };
    } else {
    };
    let mut io1: *mut TValue = &mut (*mp).i_val;
    let mut io2: *const TValue = value;
    (*io1).value_ = (*io2).value_;
    (*io1).tt_ = (*io2).tt_;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaH_getint(mut t: *mut Table, mut key: i64) -> *const TValue {
    let mut alimit: u64 = (*t).alimit as u64;
    if (key as u64).wrapping_sub(1 as u32 as u64) < alimit {
        return &mut *((*t).array).offset((key - 1i32 as i64) as isize) as *mut TValue;
    } else if (*t).flags as i32 & (1i32) << 7i32 != 0
        && (key as u64).wrapping_sub(1 as u32 as u64) & !alimit.wrapping_sub(1 as u32 as u64)
            < alimit
    {
        (*t).alimit = key as u32;
        return &mut *((*t).array).offset((key - 1i32 as i64) as isize) as *mut TValue;
    } else {
        let mut n: *mut Node = hashint(t, key);
        loop {
            if (*n).u.key_tt as i32 == 3i32 | (0i32) << 4i32 && (*n).u.key_val.i == key {
                return &mut (*n).i_val;
            } else {
                let mut nx: i32 = (*n).u.next;
                if nx == 0i32 {
                    break;
                }
                n = n.offset(nx as isize);
            }
        }
        return &absentkey;
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaH_getshortstr(
    mut t: *mut Table,
    mut key: *mut TString,
) -> *const TValue {
    let mut n: *mut Node = &mut *((*t).node)
        .offset(((*key).hash & (((1i32) << (*t).lsizenode as i32) - 1i32) as u32) as i32 as isize)
        as *mut Node;
    loop {
        if (*n).u.key_tt as i32 == 4i32 | (0i32) << 4i32 | (1i32) << 6i32
            && &mut (*((*n).u.key_val.gc as *mut GCUnion)).ts as *mut TString == key
        {
            return &mut (*n).i_val;
        } else {
            let mut nx: i32 = (*n).u.next;
            if nx == 0i32 {
                return &absentkey;
            }
            n = n.offset(nx as isize);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaH_getstr(mut t: *mut Table, mut key: *mut TString) -> *const TValue {
    if (*key).tt as i32 == 4i32 | (0i32) << 4i32 {
        return luaH_getshortstr(t, key);
    } else {
        let mut ko: TValue = TValue {
            value_: Value {
                gc: 0 as *mut GCObject,
            },
            tt_: 0,
        };
        let mut io: *mut TValue = &mut ko;
        let mut x_: *mut TString = key;
        (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
        (*io).tt_ = ((*x_).tt as i32 | (1i32) << 6i32) as u8;
        return getgeneric(t, &mut ko, 0i32);
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaH_get(mut t: *mut Table, mut key: *const TValue) -> *const TValue {
    match (*key).tt_ as i32 & 0x3f as i32 {
        4 => return luaH_getshortstr(t, &mut (*((*key).value_.gc as *mut GCUnion)).ts),
        3 => return luaH_getint(t, (*key).value_.i),
        0 => return &absentkey,
        19 => {
            let mut k: i64 = 0;
            if luaV_flttointeger((*key).value_.n, &mut k, F2Ieq) != 0 {
                return luaH_getint(t, k);
            }
        }
        _ => {}
    }
    return getgeneric(t, key, 0i32);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaH_finishset(
    mut L: *mut lua_State,
    mut t: *mut Table,
    mut key: *const TValue,
    mut slot: *const TValue,
    mut value: *mut TValue,
) {
    if (*slot).tt_ as i32 == 0i32 | (2i32) << 4i32 {
        luaH_newkey(L, t, key, value);
    } else {
        let mut io1: *mut TValue = slot as *mut TValue;
        let mut io2: *const TValue = value;
        (*io1).value_ = (*io2).value_;
        (*io1).tt_ = (*io2).tt_;
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaH_set(
    mut L: *mut lua_State,
    mut t: *mut Table,
    mut key: *const TValue,
    mut value: *mut TValue,
) {
    let mut slot: *const TValue = luaH_get(t, key);
    luaH_finishset(L, t, key, slot, value);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaH_setint(
    mut L: *mut lua_State,
    mut t: *mut Table,
    mut key: i64,
    mut value: *mut TValue,
) {
    let mut p: *const TValue = luaH_getint(t, key);
    if (*p).tt_ as i32 == 0i32 | (2i32) << 4i32 {
        let mut k: TValue = TValue {
            value_: Value {
                gc: 0 as *mut GCObject,
            },
            tt_: 0,
        };
        let mut io: *mut TValue = &mut k;
        (*io).value_.i = key;
        (*io).tt_ = (3i32 | (0i32) << 4i32) as u8;
        luaH_newkey(L, t, &mut k, value);
    } else {
        let mut io1: *mut TValue = p as *mut TValue;
        let mut io2: *const TValue = value;
        (*io1).value_ = (*io2).value_;
        (*io1).tt_ = (*io2).tt_;
    };
}
unsafe extern "C" fn hash_search(mut t: *mut Table, mut j: u64) -> u64 {
    let mut i: u64 = 0;
    if j == 0i32 as u64 {
        j = j.wrapping_add(1);
    }
    loop {
        i = j;
        if j <= (9223372036854775807i64 as u64).wrapping_div(2i32 as u64) {
            j = (j as u64).wrapping_mul(2i32 as u64) as u64 as u64;
            if (*luaH_getint(t, j as i64)).tt_ as i32 & 0xf as i32 == 0i32 {
                break;
            }
        } else {
            j = 9223372036854775807i64 as u64;
            if (*luaH_getint(t, j as i64)).tt_ as i32 & 0xf as i32 == 0i32 {
                break;
            }
            return j;
        }
    }
    while j.wrapping_sub(i) > 1 as u32 as u64 {
        let mut m: u64 = i.wrapping_add(j).wrapping_div(2i32 as u64);
        if (*luaH_getint(t, m as i64)).tt_ as i32 & 0xf as i32 == 0i32 {
            j = m;
        } else {
            i = m;
        }
    }
    return i;
}
unsafe extern "C" fn binsearch(mut array: *const TValue, mut i: u32, mut j: u32) -> u32 {
    while j.wrapping_sub(i) > 1 as u32 {
        let mut m: u32 = i.wrapping_add(j).wrapping_div(2i32 as u32);
        if (*array.offset(m.wrapping_sub(1i32 as u32) as isize)).tt_ as i32 & 0xf as i32 == 0i32 {
            j = m;
        } else {
            i = m;
        }
    }
    return i;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaH_getn(mut t: *mut Table) -> u64 {
    let mut limit: u32 = (*t).alimit;
    if limit > 0i32 as u32
        && (*((*t).array).offset(limit.wrapping_sub(1i32 as u32) as isize)).tt_ as i32 & 0xf as i32
            == 0i32
    {
        if limit >= 2i32 as u32
            && !((*((*t).array).offset(limit.wrapping_sub(2i32 as u32) as isize)).tt_ as i32
                & 0xf as i32
                == 0i32)
        {
            if ispow2realasize(t) != 0
                && !(limit.wrapping_sub(1i32 as u32)
                    & limit.wrapping_sub(1i32 as u32).wrapping_sub(1i32 as u32)
                    == 0i32 as u32)
            {
                (*t).alimit = limit.wrapping_sub(1i32 as u32);
                (*t).flags = ((*t).flags as i32 | (1i32) << 7i32) as u8;
            }
            return limit.wrapping_sub(1i32 as u32) as u64;
        } else {
            let mut boundary: u32 = binsearch((*t).array, 0i32 as u32, limit);
            if ispow2realasize(t) != 0 && boundary > (luaH_realasize(t)).wrapping_div(2i32 as u32) {
                (*t).alimit = boundary;
                (*t).flags = ((*t).flags as i32 | (1i32) << 7i32) as u8;
            }
            return boundary as u64;
        }
    }
    if !((*t).flags as i32 & (1i32) << 7i32 == 0
        || (*t).alimit & ((*t).alimit).wrapping_sub(1i32 as u32) == 0i32 as u32)
    {
        if (*((*t).array).offset(limit as isize)).tt_ as i32 & 0xf as i32 == 0i32 {
            return limit as u64;
        }
        limit = luaH_realasize(t);
        if (*((*t).array).offset(limit.wrapping_sub(1i32 as u32) as isize)).tt_ as i32 & 0xf as i32
            == 0i32
        {
            let mut boundary_0: u32 = binsearch((*t).array, (*t).alimit, limit);
            (*t).alimit = boundary_0;
            return boundary_0 as u64;
        }
    }
    if ((*t).lastfree).is_null()
        || (*luaH_getint(t, limit.wrapping_add(1i32 as u32) as i64)).tt_ as i32 & 0xf as i32 == 0i32
    {
        return limit as u64;
    } else {
        return hash_search(t, limit as u64);
    };
}
