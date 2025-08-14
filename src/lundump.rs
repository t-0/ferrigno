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
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> i32;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn luaO_pushfstring(L: *mut lua_State, fmt: *const libc::c_char, _: ...)
        -> *const libc::c_char;
    fn luaM_toobig(L: *mut lua_State) -> !;
    fn luaM_malloc_(L: *mut lua_State, size: u64, tag: i32) -> *mut libc::c_void;
    fn luaZ_read(z: *mut ZIO, b: *mut libc::c_void, n: u64) -> u64;
    fn luaZ_fill(z: *mut ZIO) -> i32;
    fn luaD_inctop(L: *mut lua_State);
    fn luaD_throw(L: *mut lua_State, errcode: i32) -> !;
    fn luaF_newproto(L: *mut lua_State) -> *mut Proto;
    fn luaF_newLclosure(L: *mut lua_State, nupvals: i32) -> *mut LClosure;
    fn luaC_barrier_(L: *mut lua_State, o: *mut GCObject, v: *mut GCObject);
    fn luaS_newlstr(L: *mut lua_State, str: *const libc::c_char, l: u64) -> *mut TString;
    fn luaS_createlngstrobj(L: *mut lua_State, l: u64) -> *mut TString;
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
pub struct LoadState {
    pub L: *mut lua_State,
    pub Z: *mut ZIO,
    pub name: *const libc::c_char,
}
unsafe extern "C" fn error(mut S: *mut LoadState, mut why: *const libc::c_char) -> ! {
    luaO_pushfstring(
        (*S).L,
        b"%s: bad binary format (%s)\0" as *const u8 as *const libc::c_char,
        (*S).name,
        why,
    );
    luaD_throw((*S).L, 3i32);
}
unsafe extern "C" fn loadBlock(mut S: *mut LoadState, mut b: *mut libc::c_void, mut size: u64) {
    if luaZ_read((*S).Z, b, size) != 0i32 as libc::c_ulong {
        error(S, b"truncated chunk\0" as *const u8 as *const libc::c_char);
    }
}
unsafe extern "C" fn loadByte(mut S: *mut LoadState) -> u8 {
    let fresh0 = (*(*S).Z).n;
    (*(*S).Z).n = ((*(*S).Z).n).wrapping_sub(1);
    let mut b: i32 = if fresh0 > 0i32 as libc::c_ulong {
        let fresh1 = (*(*S).Z).p;
        (*(*S).Z).p = ((*(*S).Z).p).offset(1);
        *fresh1 as u8 as i32
    } else {
        luaZ_fill((*S).Z)
    };
    if b == -(1i32) {
        error(S, b"truncated chunk\0" as *const u8 as *const libc::c_char);
    }
    return b as u8;
}
unsafe extern "C" fn loadUnsigned(mut S: *mut LoadState, mut limit: u64) -> u64 {
    let mut x: u64 = 0i32 as u64;
    let mut b: i32 = 0;
    limit >>= 7i32;
    loop {
        b = loadByte(S) as i32;
        if x >= limit {
            error(S, b"integer overflow\0" as *const u8 as *const libc::c_char);
        }
        x = x << 7i32 | (b & 0x7f as i32) as libc::c_ulong;
        if !(b & 0x80 as i32 == 0i32) {
            break;
        }
    }
    return x;
}
unsafe extern "C" fn loadSize(mut S: *mut LoadState) -> u64 {
    return loadUnsigned(S, !(0i32 as u64));
}
unsafe extern "C" fn loadInt(mut S: *mut LoadState) -> i32 {
    return loadUnsigned(S, 2147483647i32 as u64) as i32;
}
unsafe extern "C" fn loadNumber(mut S: *mut LoadState) -> f64 {
    let mut x: f64 = 0.;
    loadBlock(
        S,
        &mut x as *mut f64 as *mut libc::c_void,
        (1i32 as libc::c_ulong).wrapping_mul(::core::mem::size_of::<f64>() as libc::c_ulong),
    );
    return x;
}
unsafe extern "C" fn loadInteger(mut S: *mut LoadState) -> i64 {
    let mut x: i64 = 0;
    loadBlock(
        S,
        &mut x as *mut i64 as *mut libc::c_void,
        (1i32 as libc::c_ulong).wrapping_mul(::core::mem::size_of::<i64>() as libc::c_ulong),
    );
    return x;
}
unsafe extern "C" fn loadStringN(mut S: *mut LoadState, mut p: *mut Proto) -> *mut TString {
    let mut L: *mut lua_State = (*S).L;
    let mut ts: *mut TString = 0 as *mut TString;
    let mut size: u64 = loadSize(S);
    if size == 0i32 as libc::c_ulong {
        return 0 as *mut TString;
    } else {
        size = size.wrapping_sub(1);
        if size <= 40i32 as libc::c_ulong {
            let mut buff: [libc::c_char; 40] = [0; 40];
            loadBlock(
                S,
                buff.as_mut_ptr() as *mut libc::c_void,
                size.wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
            );
            ts = luaS_newlstr(L, buff.as_mut_ptr(), size);
        } else {
            ts = luaS_createlngstrobj(L, size);
            let mut io: *mut TValue = &mut (*(*L).top.p).val;
            let mut x_: *mut TString = ts;
            (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
            (*io).tt_ = ((*x_).tt as i32 | (1i32) << 6i32) as u8;
            luaD_inctop(L);
            loadBlock(
                S,
                ((*ts).contents).as_mut_ptr() as *mut libc::c_void,
                size.wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
            );
            (*L).top.p = ((*L).top.p).offset(-1);
            (*L).top.p;
        }
    }
    if (*p).marked as i32 & (1i32) << 5i32 != 0
        && (*ts).marked as i32 & ((1i32) << 3i32 | (1i32) << 4i32) != 0
    {
        luaC_barrier_(
            L,
            &mut (*(p as *mut GCUnion)).gc,
            &mut (*(ts as *mut GCUnion)).gc,
        );
    } else {
    };
    return ts;
}
unsafe extern "C" fn loadString(mut S: *mut LoadState, mut p: *mut Proto) -> *mut TString {
    let mut st: *mut TString = loadStringN(S, p);
    if st.is_null() {
        error(
            S,
            b"bad format for constant string\0" as *const u8 as *const libc::c_char,
        );
    }
    return st;
}
unsafe extern "C" fn loadCode(mut S: *mut LoadState, mut f: *mut Proto) {
    let mut n: i32 = loadInt(S);
    if ::core::mem::size_of::<i32>() as libc::c_ulong
        >= ::core::mem::size_of::<u64>() as libc::c_ulong
        && (n as u64).wrapping_add(1i32 as libc::c_ulong)
            > (!(0i32 as u64)).wrapping_div(::core::mem::size_of::<Instruction>() as libc::c_ulong)
    {
        luaM_toobig((*S).L);
    } else {
    };
    (*f).code = luaM_malloc_(
        (*S).L,
        (n as libc::c_ulong).wrapping_mul(::core::mem::size_of::<Instruction>() as libc::c_ulong),
        0i32,
    ) as *mut Instruction;
    (*f).sizecode = n;
    loadBlock(
        S,
        (*f).code as *mut libc::c_void,
        (n as libc::c_ulong).wrapping_mul(::core::mem::size_of::<Instruction>() as libc::c_ulong),
    );
}
unsafe extern "C" fn loadConstants(mut S: *mut LoadState, mut f: *mut Proto) {
    let mut i: i32 = 0;
    let mut n: i32 = loadInt(S);
    if ::core::mem::size_of::<i32>() as libc::c_ulong
        >= ::core::mem::size_of::<u64>() as libc::c_ulong
        && (n as u64).wrapping_add(1i32 as libc::c_ulong)
            > (!(0i32 as u64)).wrapping_div(::core::mem::size_of::<TValue>() as libc::c_ulong)
    {
        luaM_toobig((*S).L);
    } else {
    };
    (*f).k = luaM_malloc_(
        (*S).L,
        (n as libc::c_ulong).wrapping_mul(::core::mem::size_of::<TValue>() as libc::c_ulong),
        0i32,
    ) as *mut TValue;
    (*f).sizek = n;
    i = 0i32;
    while i < n {
        (*((*f).k).offset(i as isize)).tt_ = (0i32 | (0i32) << 4i32) as u8;
        i += 1;
    }
    i = 0i32;
    while i < n {
        let mut o: *mut TValue = &mut *((*f).k).offset(i as isize) as *mut TValue;
        let mut t: i32 = loadByte(S) as i32;
        match t {
            0 => {
                (*o).tt_ = (0i32 | (0i32) << 4i32) as u8;
            }
            1 => {
                (*o).tt_ = (1i32 | (0i32) << 4i32) as u8;
            }
            17 => {
                (*o).tt_ = (1i32 | (1i32) << 4i32) as u8;
            }
            19 => {
                let mut io: *mut TValue = o;
                (*io).value_.n = loadNumber(S);
                (*io).tt_ = (3i32 | (1i32) << 4i32) as u8;
            }
            3 => {
                let mut io_0: *mut TValue = o;
                (*io_0).value_.i = loadInteger(S);
                (*io_0).tt_ = (3i32 | (0i32) << 4i32) as u8;
            }
            4 | 20 => {
                let mut io_1: *mut TValue = o;
                let mut x_: *mut TString = loadString(S, f);
                (*io_1).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
                (*io_1).tt_ = ((*x_).tt as i32 | (1i32) << 6i32) as u8;
            }
            _ => {}
        }
        i += 1;
    }
}
unsafe extern "C" fn loadProtos(mut S: *mut LoadState, mut f: *mut Proto) {
    let mut i: i32 = 0;
    let mut n: i32 = loadInt(S);
    if ::core::mem::size_of::<i32>() as libc::c_ulong
        >= ::core::mem::size_of::<u64>() as libc::c_ulong
        && (n as u64).wrapping_add(1i32 as libc::c_ulong)
            > (!(0i32 as u64)).wrapping_div(::core::mem::size_of::<*mut Proto>() as libc::c_ulong)
    {
        luaM_toobig((*S).L);
    } else {
    };
    (*f).p = luaM_malloc_(
        (*S).L,
        (n as libc::c_ulong).wrapping_mul(::core::mem::size_of::<*mut Proto>() as libc::c_ulong),
        0i32,
    ) as *mut *mut Proto;
    (*f).sizep = n;
    i = 0i32;
    while i < n {
        let ref mut fresh2 = *((*f).p).offset(i as isize);
        *fresh2 = 0 as *mut Proto;
        i += 1;
    }
    i = 0i32;
    while i < n {
        let ref mut fresh3 = *((*f).p).offset(i as isize);
        *fresh3 = luaF_newproto((*S).L);
        if (*f).marked as i32 & (1i32) << 5i32 != 0
            && (**((*f).p).offset(i as isize)).marked as i32 & ((1i32) << 3i32 | (1i32) << 4i32)
                != 0
        {
            luaC_barrier_(
                (*S).L,
                &mut (*(f as *mut GCUnion)).gc,
                &mut (*(*((*f).p).offset(i as isize) as *mut GCUnion)).gc,
            );
        } else {
        };
        loadFunction(S, *((*f).p).offset(i as isize), (*f).source);
        i += 1;
    }
}
unsafe extern "C" fn loadUpvalues(mut S: *mut LoadState, mut f: *mut Proto) {
    let mut i: i32 = 0;
    let mut n: i32 = 0;
    n = loadInt(S);
    if ::core::mem::size_of::<i32>() as libc::c_ulong
        >= ::core::mem::size_of::<u64>() as libc::c_ulong
        && (n as u64).wrapping_add(1i32 as libc::c_ulong)
            > (!(0i32 as u64)).wrapping_div(::core::mem::size_of::<Upvaldesc>() as libc::c_ulong)
    {
        luaM_toobig((*S).L);
    } else {
    };
    (*f).upvalues = luaM_malloc_(
        (*S).L,
        (n as libc::c_ulong).wrapping_mul(::core::mem::size_of::<Upvaldesc>() as libc::c_ulong),
        0i32,
    ) as *mut Upvaldesc;
    (*f).sizeupvalues = n;
    i = 0i32;
    while i < n {
        let ref mut fresh4 = (*((*f).upvalues).offset(i as isize)).name;
        *fresh4 = 0 as *mut TString;
        i += 1;
    }
    i = 0i32;
    while i < n {
        (*((*f).upvalues).offset(i as isize)).instack = loadByte(S);
        (*((*f).upvalues).offset(i as isize)).index = loadByte(S);
        (*((*f).upvalues).offset(i as isize)).kind = loadByte(S);
        i += 1;
    }
}
unsafe extern "C" fn loadDebug(mut S: *mut LoadState, mut f: *mut Proto) {
    let mut i: i32 = 0;
    let mut n: i32 = 0;
    n = loadInt(S);
    if ::core::mem::size_of::<i32>() as libc::c_ulong
        >= ::core::mem::size_of::<u64>() as libc::c_ulong
        && (n as u64).wrapping_add(1i32 as libc::c_ulong)
            > (!(0i32 as u64)).wrapping_div(::core::mem::size_of::<ls_byte>() as libc::c_ulong)
    {
        luaM_toobig((*S).L);
    } else {
    };
    (*f).lineinfo = luaM_malloc_(
        (*S).L,
        (n as libc::c_ulong).wrapping_mul(::core::mem::size_of::<ls_byte>() as libc::c_ulong),
        0i32,
    ) as *mut ls_byte;
    (*f).sizelineinfo = n;
    loadBlock(
        S,
        (*f).lineinfo as *mut libc::c_void,
        (n as libc::c_ulong).wrapping_mul(::core::mem::size_of::<ls_byte>() as libc::c_ulong),
    );
    n = loadInt(S);
    if ::core::mem::size_of::<i32>() as libc::c_ulong
        >= ::core::mem::size_of::<u64>() as libc::c_ulong
        && (n as u64).wrapping_add(1i32 as libc::c_ulong)
            > (!(0i32 as u64)).wrapping_div(::core::mem::size_of::<AbsLineInfo>() as libc::c_ulong)
    {
        luaM_toobig((*S).L);
    } else {
    };
    (*f).abslineinfo = luaM_malloc_(
        (*S).L,
        (n as libc::c_ulong).wrapping_mul(::core::mem::size_of::<AbsLineInfo>() as libc::c_ulong),
        0i32,
    ) as *mut AbsLineInfo;
    (*f).sizeabslineinfo = n;
    i = 0i32;
    while i < n {
        (*((*f).abslineinfo).offset(i as isize)).pc = loadInt(S);
        (*((*f).abslineinfo).offset(i as isize)).line = loadInt(S);
        i += 1;
    }
    n = loadInt(S);
    if ::core::mem::size_of::<i32>() as libc::c_ulong
        >= ::core::mem::size_of::<u64>() as libc::c_ulong
        && (n as u64).wrapping_add(1i32 as libc::c_ulong)
            > (!(0i32 as u64)).wrapping_div(::core::mem::size_of::<LocVar>() as libc::c_ulong)
    {
        luaM_toobig((*S).L);
    } else {
    };
    (*f).locvars = luaM_malloc_(
        (*S).L,
        (n as libc::c_ulong).wrapping_mul(::core::mem::size_of::<LocVar>() as libc::c_ulong),
        0i32,
    ) as *mut LocVar;
    (*f).sizelocvars = n;
    i = 0i32;
    while i < n {
        let ref mut fresh5 = (*((*f).locvars).offset(i as isize)).varname;
        *fresh5 = 0 as *mut TString;
        i += 1;
    }
    i = 0i32;
    while i < n {
        let ref mut fresh6 = (*((*f).locvars).offset(i as isize)).varname;
        *fresh6 = loadStringN(S, f);
        (*((*f).locvars).offset(i as isize)).startpc = loadInt(S);
        (*((*f).locvars).offset(i as isize)).endpc = loadInt(S);
        i += 1;
    }
    n = loadInt(S);
    if n != 0i32 {
        n = (*f).sizeupvalues;
    }
    i = 0i32;
    while i < n {
        let ref mut fresh7 = (*((*f).upvalues).offset(i as isize)).name;
        *fresh7 = loadStringN(S, f);
        i += 1;
    }
}
unsafe extern "C" fn loadFunction(
    mut S: *mut LoadState,
    mut f: *mut Proto,
    mut psource: *mut TString,
) {
    (*f).source = loadStringN(S, f);
    if ((*f).source).is_null() {
        (*f).source = psource;
    }
    (*f).linedefined = loadInt(S);
    (*f).lastlinedefined = loadInt(S);
    (*f).numparams = loadByte(S);
    (*f).is_vararg = loadByte(S);
    (*f).maxstacksize = loadByte(S);
    loadCode(S, f);
    loadConstants(S, f);
    loadUpvalues(S, f);
    loadProtos(S, f);
    loadDebug(S, f);
}
unsafe extern "C" fn checkliteral(
    mut S: *mut LoadState,
    mut s: *const libc::c_char,
    mut msg: *const libc::c_char,
) {
    let mut buff: [libc::c_char; 12] = [0; 12];
    let mut len: u64 = strlen(s);
    loadBlock(
        S,
        buff.as_mut_ptr() as *mut libc::c_void,
        len.wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
    );
    if memcmp(
        s as *const libc::c_void,
        buff.as_mut_ptr() as *const libc::c_void,
        len,
    ) != 0i32
    {
        error(S, msg);
    }
}
unsafe extern "C" fn fchecksize(
    mut S: *mut LoadState,
    mut size: u64,
    mut tname: *const libc::c_char,
) {
    if loadByte(S) as libc::c_ulong != size {
        error(
            S,
            luaO_pushfstring(
                (*S).L,
                b"%s size mismatch\0" as *const u8 as *const libc::c_char,
                tname,
            ),
        );
    }
}
unsafe extern "C" fn checkHeader(mut S: *mut LoadState) {
    checkliteral(
        S,
        &*(b"\x1BLua\0" as *const u8 as *const libc::c_char).offset(1i32 as isize),
        b"not a binary chunk\0" as *const u8 as *const libc::c_char,
    );
    if loadByte(S) as i32 != 504i32 / 100i32 * 16i32 + 504i32 % 100i32 {
        error(S, b"version mismatch\0" as *const u8 as *const libc::c_char);
    }
    if loadByte(S) as i32 != 0i32 {
        error(S, b"format mismatch\0" as *const u8 as *const libc::c_char);
    }
    checkliteral(
        S,
        b"\x19\x93\r\n\x1A\n\0" as *const u8 as *const libc::c_char,
        b"corrupted chunk\0" as *const u8 as *const libc::c_char,
    );
    fchecksize(
        S,
        ::core::mem::size_of::<Instruction>() as libc::c_ulong,
        b"Instruction\0" as *const u8 as *const libc::c_char,
    );
    fchecksize(
        S,
        ::core::mem::size_of::<i64>() as libc::c_ulong,
        b"i64\0" as *const u8 as *const libc::c_char,
    );
    fchecksize(
        S,
        ::core::mem::size_of::<f64>() as libc::c_ulong,
        b"f64\0" as *const u8 as *const libc::c_char,
    );
    if loadInteger(S) != 0x5678 as i32 as i64 {
        error(
            S,
            b"integer format mismatch\0" as *const u8 as *const libc::c_char,
        );
    }
    if loadNumber(S) != 370.5f64 {
        error(
            S,
            b"float format mismatch\0" as *const u8 as *const libc::c_char,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaU_undump(
    mut L: *mut lua_State,
    mut Z: *mut ZIO,
    mut name: *const libc::c_char,
) -> *mut LClosure {
    let mut S: LoadState = LoadState {
        L: 0 as *mut lua_State,
        Z: 0 as *mut ZIO,
        name: 0 as *const libc::c_char,
    };
    let mut cl: *mut LClosure = 0 as *mut LClosure;
    if *name as i32 == '@' as i32 || *name as i32 == '=' as i32 {
        S.name = name.offset(1i32 as isize);
    } else if *name as i32
        == (*::core::mem::transmute::<&[u8; 5], &[libc::c_char; 5]>(b"\x1BLua\0"))[0i32 as usize]
            as i32
    {
        S.name = b"binary string\0" as *const u8 as *const libc::c_char;
    } else {
        S.name = name;
    }
    S.L = L;
    S.Z = Z;
    checkHeader(&mut S);
    cl = luaF_newLclosure(L, loadByte(&mut S) as i32);
    let mut io: *mut TValue = &mut (*(*L).top.p).val;
    let mut x_: *mut LClosure = cl;
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
    (*io).tt_ = (6i32 | (0i32) << 4i32 | (1i32) << 6i32) as u8;
    luaD_inctop(L);
    (*cl).p = luaF_newproto(L);
    if (*cl).marked as i32 & (1i32) << 5i32 != 0
        && (*(*cl).p).marked as i32 & ((1i32) << 3i32 | (1i32) << 4i32) != 0
    {
        luaC_barrier_(
            L,
            &mut (*(cl as *mut GCUnion)).gc,
            &mut (*((*cl).p as *mut GCUnion)).gc,
        );
    } else {
    };
    loadFunction(&mut S, (*cl).p, 0 as *mut TString);
    return cl;
}
