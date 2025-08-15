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
    fn _setjmp(_: *mut __jmp_buf_tag) -> i32;
    fn _longjmp(__env: *mut __jmp_buf_tag, __val: i32) -> !;
    fn abort() -> !;
    fn strchr(_: *const libc::c_char, _: i32) -> *mut libc::c_char;
    fn luaE_extendCI(L: *mut lua_State) -> *mut CallInfo;
    fn luaE_shrinkCI(L: *mut lua_State);
    fn luaE_checkcstack(L: *mut lua_State);
    fn luaE_resetthread(L: *mut lua_State, status: i32) -> i32;
    fn luaO_pushfstring(L: *mut lua_State, fmt: *const libc::c_char, _: ...)
        -> *const libc::c_char;
    fn luaT_gettmbyobj(L: *mut lua_State, o: *const TValue, event: TMS) -> *const TValue;
    fn luaM_realloc_(
        L: *mut lua_State,
        block: *mut libc::c_void,
        oldsize: u64,
        size: u64,
    ) -> *mut libc::c_void;
    fn luaM_saferealloc_(
        L: *mut lua_State,
        block: *mut libc::c_void,
        oldsize: u64,
        size: u64,
    ) -> *mut libc::c_void;
    fn luaM_free_(L: *mut lua_State, block: *mut libc::c_void, osize: u64);
    fn luaZ_fill(z: *mut ZIO) -> i32;
    fn luaG_callerror(L: *mut lua_State, o: *const TValue) -> !;
    fn luaG_runerror(L: *mut lua_State, fmt: *const libc::c_char, _: ...) -> !;
    fn luaF_initupvals(L: *mut lua_State, cl: *mut LClosure);
    fn luaF_close(L: *mut lua_State, level: StkId, status: i32, yy: i32) -> StkId;
    fn luaC_step(L: *mut lua_State);
    fn luaY_parser(
        L: *mut lua_State,
        z: *mut ZIO,
        buff: *mut Mbuffer,
        dyd: *mut Dyndata,
        name: *const libc::c_char,
        firstchar: i32,
    ) -> *mut LClosure;
    fn luaS_newlstr(L: *mut lua_State, str: *const libc::c_char, l: u64) -> *mut TString;
    fn luaS_new(L: *mut lua_State, str: *const libc::c_char) -> *mut TString;
    fn luaU_undump(L: *mut lua_State, Z: *mut ZIO, name: *const libc::c_char) -> *mut LClosure;
    fn luaV_finishOp(L: *mut lua_State);
    fn luaV_execute(L: *mut lua_State, ci: *mut CallInfo);
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
pub type ls_byte = libc::c_schar;
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
pub union Closure {
    pub c: CClosure,
    pub l: LClosure,
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
pub union UValue {
    pub uv: TValue,
    pub n: f64,
    pub u: f64,
    pub s: *mut libc::c_void,
    pub i: i64,
    pub l: i64,
}
pub type Pfunc = Option<unsafe extern "C" fn(*mut lua_State, *mut libc::c_void) -> ()>;
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
pub struct Labeldesc {
    pub name: *mut TString,
    pub pc: i32,
    pub line: i32,
    pub nactvar: u8,
    pub close: u8,
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
pub struct Dyndata {
    pub actvar: C2RustUnnamed_9,
    pub gt: Labellist,
    pub label: Labellist,
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
pub struct SParser {
    pub z: *mut ZIO,
    pub buff: Mbuffer,
    pub dyd: Dyndata,
    pub mode: *const libc::c_char,
    pub name: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CloseP {
    pub level: StkId,
    pub status: i32,
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaD_seterrorobj(
    mut L: *mut lua_State,
    mut errcode: i32,
    mut oldtop: StkId,
) {
    match errcode {
        4 => {
            let mut io: *mut TValue = &mut (*oldtop).val;
            let mut x_: *mut TString = (*(*L).l_G).memerrmsg;
            (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
            (*io).tt_ = ((*x_).tt as i32 | (1i32) << 6i32) as u8;
        }
        0 => {
            (*oldtop).val.tt_ = (0i32 | (0i32) << 4i32) as u8;
        }
        _ => {
            let mut io1: *mut TValue = &mut (*oldtop).val;
            let mut io2: *const TValue = &mut (*((*L).top.p).offset(-(1i32 as isize))).val;
            (*io1).value_ = (*io2).value_;
            (*io1).tt_ = (*io2).tt_;
        }
    }
    (*L).top.p = oldtop.offset(1i32 as isize);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaD_throw(mut L: *mut lua_State, mut errcode: i32) -> ! {
    if !((*L).errorJmp).is_null() {
        ::core::ptr::write_volatile(&mut (*(*L).errorJmp).status as *mut i32, errcode);
        _longjmp(((*(*L).errorJmp).b).as_mut_ptr(), 1i32);
    } else {
        let mut g: *mut global_State = (*L).l_G;
        errcode = luaE_resetthread(L, errcode);
        (*L).status = errcode as u8;
        if !((*(*g).mainthread).errorJmp).is_null() {
            let fresh0 = (*(*g).mainthread).top.p;
            (*(*g).mainthread).top.p = ((*(*g).mainthread).top.p).offset(1);
            let mut io1: *mut TValue = &mut (*fresh0).val;
            let mut io2: *const TValue = &mut (*((*L).top.p).offset(-(1i32 as isize))).val;
            (*io1).value_ = (*io2).value_;
            (*io1).tt_ = (*io2).tt_;
            luaD_throw((*g).mainthread, errcode);
        } else {
            if ((*g).panic).is_some() {
                ((*g).panic).expect("non-null function pointer")(L);
            }
            abort();
        }
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaD_rawrunprotected(
    mut L: *mut lua_State,
    mut f: Pfunc,
    mut ud: *mut libc::c_void,
) -> i32 {
    let mut oldnCcalls: u32 = (*L).nCcalls;
    let mut lj: LongJump = LongJump {
        previous: 0 as *mut LongJump,
        b: [__jmp_buf_tag {
            __jmpbuf: [0; 8],
            __mask_was_saved: 0,
            __saved_mask: __sigset_t { __val: [0; 16] },
        }; 1],
        status: 0,
    };
    ::core::ptr::write_volatile(&mut lj.status as *mut i32, 0i32);
    lj.previous = (*L).errorJmp;
    (*L).errorJmp = &mut lj;
    if _setjmp((lj.b).as_mut_ptr()) == 0i32 {
        (Some(f.expect("non-null function pointer"))).expect("non-null function pointer")(L, ud);
    }
    (*L).errorJmp = lj.previous;
    (*L).nCcalls = oldnCcalls;
    return lj.status;
}
unsafe extern "C" fn relstack(mut L: *mut lua_State) {
    let mut ci: *mut CallInfo = 0 as *mut CallInfo;
    let mut up: *mut UpVal = 0 as *mut UpVal;
    (*L).top.offset =
        ((*L).top.p as *mut libc::c_char).offset_from((*L).stack.p as *mut libc::c_char) as i64;
    (*L).tbclist.offset =
        ((*L).tbclist.p as *mut libc::c_char).offset_from((*L).stack.p as *mut libc::c_char) as i64;
    up = (*L).openupval;
    while !up.is_null() {
        (*up).v.offset = ((*up).v.p as StkId as *mut libc::c_char)
            .offset_from((*L).stack.p as *mut libc::c_char) as i64;
        up = (*up).u.open.next;
    }
    ci = (*L).ci;
    while !ci.is_null() {
        (*ci).top.offset = ((*ci).top.p as *mut libc::c_char)
            .offset_from((*L).stack.p as *mut libc::c_char) as i64;
        (*ci).func.offset = ((*ci).func.p as *mut libc::c_char)
            .offset_from((*L).stack.p as *mut libc::c_char) as i64;
        ci = (*ci).previous;
    }
}
unsafe extern "C" fn correctstack(mut L: *mut lua_State) {
    let mut ci: *mut CallInfo = 0 as *mut CallInfo;
    let mut up: *mut UpVal = 0 as *mut UpVal;
    (*L).top.p = ((*L).stack.p as *mut libc::c_char).offset((*L).top.offset as isize) as StkId;
    (*L).tbclist.p =
        ((*L).stack.p as *mut libc::c_char).offset((*L).tbclist.offset as isize) as StkId;
    up = (*L).openupval;
    while !up.is_null() {
        (*up).v.p = &mut (*(((*L).stack.p as *mut libc::c_char).offset((*up).v.offset as isize)
            as StkId))
            .val;
        up = (*up).u.open.next;
    }
    ci = (*L).ci;
    while !ci.is_null() {
        (*ci).top.p =
            ((*L).stack.p as *mut libc::c_char).offset((*ci).top.offset as isize) as StkId;
        (*ci).func.p =
            ((*L).stack.p as *mut libc::c_char).offset((*ci).func.offset as isize) as StkId;
        if (*ci).callstatus as i32 & (1i32) << 1i32 == 0 {
            ::core::ptr::write_volatile(&mut (*ci).u.l.trap as *mut sig_atomic_t, 1i32);
        }
        ci = (*ci).previous;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaD_errerr(mut L: *mut lua_State) -> ! {
    let mut msg: *mut TString = luaS_newlstr(
        L,
        b"error in error handling\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong),
    );
    let mut io: *mut TValue = &mut (*(*L).top.p).val;
    let mut x_: *mut TString = msg;
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
    (*io).tt_ = ((*x_).tt as i32 | (1i32) << 6i32) as u8;
    (*L).top.p = ((*L).top.p).offset(1);
    (*L).top.p;
    luaD_throw(L, 5i32);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaD_reallocstack(
    mut L: *mut lua_State,
    mut newsize: i32,
    mut raiseerror: i32,
) -> i32 {
    let mut oldsize: i32 = ((*L).stack_last.p).offset_from((*L).stack.p) as i64 as i32;
    let mut i: i32 = 0;
    let mut newstack: StkId = 0 as *mut StackValue;
    let mut oldgcstop: i32 = (*(*L).l_G).gcstopem as i32;
    relstack(L);
    (*(*L).l_G).gcstopem = 1i32 as u8;
    newstack = luaM_realloc_(
        L,
        (*L).stack.p as *mut libc::c_void,
        ((oldsize + 5i32) as u64)
            .wrapping_mul(::core::mem::size_of::<StackValue>() as libc::c_ulong),
        ((newsize + 5i32) as u64)
            .wrapping_mul(::core::mem::size_of::<StackValue>() as libc::c_ulong),
    ) as *mut StackValue;
    (*(*L).l_G).gcstopem = oldgcstop as u8;
    if ((newstack == 0 as *mut libc::c_void as StkId) as i32 != 0i32) as i32 as i64 != 0 {
        correctstack(L);
        if raiseerror != 0 {
            luaD_throw(L, 4i32);
        } else {
            return 0i32;
        }
    }
    (*L).stack.p = newstack;
    correctstack(L);
    (*L).stack_last.p = ((*L).stack.p).offset(newsize as isize);
    i = oldsize + 5i32;
    while i < newsize + 5i32 {
        (*newstack.offset(i as isize)).val.tt_ = (0i32 | (0i32) << 4i32) as u8;
        i += 1;
    }
    return 1i32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaD_growstack(
    mut L: *mut lua_State,
    mut n: i32,
    mut raiseerror: i32,
) -> i32 {
    let mut size: i32 = ((*L).stack_last.p).offset_from((*L).stack.p) as i64 as i32;
    if ((size > 1000000i32) as i32 != 0i32) as i32 as i64 != 0 {
        if raiseerror != 0 {
            luaD_errerr(L);
        }
        return 0i32;
    } else if n < 1000000i32 {
        let mut newsize: i32 = 2i32 * size;
        let mut needed: i32 = ((*L).top.p).offset_from((*L).stack.p) as i64 as i32 + n;
        if newsize > 1000000i32 {
            newsize = 1000000i32;
        }
        if newsize < needed {
            newsize = needed;
        }
        if ((newsize <= 1000000i32) as i32 != 0i32) as i32 as i64 != 0 {
            return luaD_reallocstack(L, newsize, raiseerror);
        }
    }
    luaD_reallocstack(L, 1000000i32 + 200i32, raiseerror);
    if raiseerror != 0 {
        luaG_runerror(L, b"stack overflow\0" as *const u8 as *const libc::c_char);
    }
    return 0i32;
}
unsafe extern "C" fn stackinuse(mut L: *mut lua_State) -> i32 {
    let mut ci: *mut CallInfo = 0 as *mut CallInfo;
    let mut res: i32 = 0;
    let mut lim: StkId = (*L).top.p;
    ci = (*L).ci;
    while !ci.is_null() {
        if lim < (*ci).top.p {
            lim = (*ci).top.p;
        }
        ci = (*ci).previous;
    }
    res = lim.offset_from((*L).stack.p) as i64 as i32 + 1i32;
    if res < 20i32 {
        res = 20i32;
    }
    return res;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaD_shrinkstack(mut L: *mut lua_State) {
    let mut inuse: i32 = stackinuse(L);
    let mut max: i32 = if inuse > 1000000i32 / 3i32 {
        1000000i32
    } else {
        inuse * 3i32
    };
    if inuse <= 1000000i32 && ((*L).stack_last.p).offset_from((*L).stack.p) as i64 as i32 > max {
        let mut nsize: i32 = if inuse > 1000000i32 / 2i32 {
            1000000i32
        } else {
            inuse * 2i32
        };
        luaD_reallocstack(L, nsize, 0i32);
    }
    luaE_shrinkCI(L);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaD_inctop(mut L: *mut lua_State) {
    if ((((*L).stack_last.p).offset_from((*L).top.p) as i64 <= 1i32 as i64) as i32 != 0i32) as i32
        as i64
        != 0
    {
        luaD_growstack(L, 1i32, 1i32);
    }
    (*L).top.p = ((*L).top.p).offset(1);
    (*L).top.p;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaD_hook(
    mut L: *mut lua_State,
    mut event: i32,
    mut line: i32,
    mut ftransfer: i32,
    mut ntransfer: i32,
) {
    let mut hook: lua_Hook = (*L).hook;
    if hook.is_some() && (*L).allowhook as i32 != 0 {
        let mut mask: i32 = (1i32) << 3i32;
        let mut ci: *mut CallInfo = (*L).ci;
        let mut top: i64 =
            ((*L).top.p as *mut libc::c_char).offset_from((*L).stack.p as *mut libc::c_char) as i64;
        let mut ci_top: i64 = ((*ci).top.p as *mut libc::c_char)
            .offset_from((*L).stack.p as *mut libc::c_char) as i64;
        let mut ar: lua_Debug = lua_Debug {
            event: 0,
            name: 0 as *const libc::c_char,
            namewhat: 0 as *const libc::c_char,
            what: 0 as *const libc::c_char,
            source: 0 as *const libc::c_char,
            srclen: 0,
            currentline: 0,
            linedefined: 0,
            lastlinedefined: 0,
            nups: 0,
            nparams: 0,
            isvararg: 0,
            istailcall: 0,
            ftransfer: 0,
            ntransfer: 0,
            short_src: [0; 60],
            i_ci: 0 as *mut CallInfo,
        };
        ar.event = event;
        ar.currentline = line;
        ar.i_ci = ci;
        if ntransfer != 0i32 {
            mask |= (1i32) << 8i32;
            (*ci).u2.transferinfo.ftransfer = ftransfer as libc::c_ushort;
            (*ci).u2.transferinfo.ntransfer = ntransfer as libc::c_ushort;
        }
        if (*ci).callstatus as i32 & (1i32) << 1i32 == 0 && (*L).top.p < (*ci).top.p {
            (*L).top.p = (*ci).top.p;
        }
        if ((((*L).stack_last.p).offset_from((*L).top.p) as i64 <= 20i32 as i64) as i32 != 0i32)
            as i32 as i64
            != 0
        {
            luaD_growstack(L, 20i32, 1i32);
        }
        if (*ci).top.p < ((*L).top.p).offset(20i32 as isize) {
            (*ci).top.p = ((*L).top.p).offset(20i32 as isize);
        }
        (*L).allowhook = 0i32 as u8;
        (*ci).callstatus = ((*ci).callstatus as i32 | mask) as libc::c_ushort;
        (Some(hook.expect("non-null function pointer"))).expect("non-null function pointer")(
            L, &mut ar,
        );
        (*L).allowhook = 1i32 as u8;
        (*ci).top.p = ((*L).stack.p as *mut libc::c_char).offset(ci_top as isize) as StkId;
        (*L).top.p = ((*L).stack.p as *mut libc::c_char).offset(top as isize) as StkId;
        (*ci).callstatus = ((*ci).callstatus as i32 & !mask) as libc::c_ushort;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaD_hookcall(mut L: *mut lua_State, mut ci: *mut CallInfo) {
    (*L).oldpc = 0i32;
    if (*L).hookmask & (1i32) << 0i32 != 0 {
        let mut event: i32 = if (*ci).callstatus as i32 & (1i32) << 5i32 != 0 {
            4i32
        } else {
            0i32
        };
        let mut p: *mut Proto = (*((*(*ci).func.p).val.value_.gc as *mut GCUnion)).cl.l.p;
        (*ci).u.l.savedpc = ((*ci).u.l.savedpc).offset(1);
        (*ci).u.l.savedpc;
        luaD_hook(L, event, -(1i32), 1i32, (*p).numparams as i32);
        (*ci).u.l.savedpc = ((*ci).u.l.savedpc).offset(-1);
        (*ci).u.l.savedpc;
    }
}
unsafe extern "C" fn rethook(mut L: *mut lua_State, mut ci: *mut CallInfo, mut nres: i32) {
    if (*L).hookmask & (1i32) << 1i32 != 0 {
        let mut firstres: StkId = ((*L).top.p).offset(-(nres as isize));
        let mut delta: i32 = 0i32;
        let mut ftransfer: i32 = 0;
        if (*ci).callstatus as i32 & (1i32) << 1i32 == 0 {
            let mut p: *mut Proto = (*((*(*ci).func.p).val.value_.gc as *mut GCUnion)).cl.l.p;
            if (*p).is_vararg != 0 {
                delta = (*ci).u.l.nextraargs + (*p).numparams as i32 + 1i32;
            }
        }
        (*ci).func.p = ((*ci).func.p).offset(delta as isize);
        ftransfer = firstres.offset_from((*ci).func.p) as i64 as libc::c_ushort as i32;
        luaD_hook(L, 1i32, -(1i32), ftransfer, nres);
        (*ci).func.p = ((*ci).func.p).offset(-(delta as isize));
    }
    ci = (*ci).previous;
    if (*ci).callstatus as i32 & (1i32) << 1i32 == 0 {
        (*L).oldpc = ((*ci).u.l.savedpc)
            .offset_from((*(*((*(*ci).func.p).val.value_.gc as *mut GCUnion)).cl.l.p).code)
            as i64 as i32
            - 1i32;
    }
}
unsafe extern "C" fn tryfuncTM(mut L: *mut lua_State, mut func: StkId) -> StkId {
    let mut tm: *const TValue = 0 as *const TValue;
    let mut p: StkId = 0 as *mut StackValue;
    if ((((*L).stack_last.p).offset_from((*L).top.p) as i64 <= 1i32 as i64) as i32 != 0i32) as i32
        as i64
        != 0
    {
        let mut t__: i64 =
            (func as *mut libc::c_char).offset_from((*L).stack.p as *mut libc::c_char) as i64;
        if (*(*L).l_G).GCdebt > 0i32 as i64 {
            luaC_step(L);
        }
        luaD_growstack(L, 1i32, 1i32);
        func = ((*L).stack.p as *mut libc::c_char).offset(t__ as isize) as StkId;
    }
    tm = luaT_gettmbyobj(L, &mut (*func).val, TM_CALL);
    if (((*tm).tt_ as i32 & 0xf as i32 == 0i32) as i32 != 0i32) as i32 as i64 != 0 {
        luaG_callerror(L, &mut (*func).val);
    }
    p = (*L).top.p;
    while p > func {
        let mut io1: *mut TValue = &mut (*p).val;
        let mut io2: *const TValue = &mut (*p.offset(-(1i32 as isize))).val;
        (*io1).value_ = (*io2).value_;
        (*io1).tt_ = (*io2).tt_;
        p = p.offset(-1);
    }
    (*L).top.p = ((*L).top.p).offset(1);
    (*L).top.p;
    let mut io1_0: *mut TValue = &mut (*func).val;
    let mut io2_0: *const TValue = tm;
    (*io1_0).value_ = (*io2_0).value_;
    (*io1_0).tt_ = (*io2_0).tt_;
    return func;
}
#[inline]
unsafe extern "C" fn moveresults(
    mut L: *mut lua_State,
    mut res: StkId,
    mut nres: i32,
    mut wanted: i32,
) {
    let mut firstresult: StkId = 0 as *mut StackValue;
    let mut i: i32 = 0;
    match wanted {
        0 => {
            (*L).top.p = res;
            return;
        }
        1 => {
            if nres == 0i32 {
                (*res).val.tt_ = (0i32 | (0i32) << 4i32) as u8;
            } else {
                let mut io1: *mut TValue = &mut (*res).val;
                let mut io2: *const TValue = &mut (*((*L).top.p).offset(-(nres as isize))).val;
                (*io1).value_ = (*io2).value_;
                (*io1).tt_ = (*io2).tt_;
            }
            (*L).top.p = res.offset(1i32 as isize);
            return;
        }
        -1 => {
            wanted = nres;
        }
        _ => {
            if wanted < -(1i32) {
                (*(*L).ci).callstatus =
                    ((*(*L).ci).callstatus as i32 | (1i32) << 9i32) as libc::c_ushort;
                (*(*L).ci).u2.nres = nres;
                res = luaF_close(L, res, -(1i32), 1i32);
                (*(*L).ci).callstatus =
                    ((*(*L).ci).callstatus as i32 & !((1i32) << 9i32)) as libc::c_ushort;
                if (*L).hookmask != 0 {
                    let mut savedres: i64 = (res as *mut libc::c_char)
                        .offset_from((*L).stack.p as *mut libc::c_char)
                        as i64;
                    rethook(L, (*L).ci, nres);
                    res = ((*L).stack.p as *mut libc::c_char).offset(savedres as isize) as StkId;
                }
                wanted = -wanted - 3i32;
                if wanted == -(1i32) {
                    wanted = nres;
                }
            }
        }
    }
    firstresult = ((*L).top.p).offset(-(nres as isize));
    if nres > wanted {
        nres = wanted;
    }
    i = 0i32;
    while i < nres {
        let mut io1_0: *mut TValue = &mut (*res.offset(i as isize)).val;
        let mut io2_0: *const TValue = &mut (*firstresult.offset(i as isize)).val;
        (*io1_0).value_ = (*io2_0).value_;
        (*io1_0).tt_ = (*io2_0).tt_;
        i += 1;
    }
    while i < wanted {
        (*res.offset(i as isize)).val.tt_ = (0i32 | (0i32) << 4i32) as u8;
        i += 1;
    }
    (*L).top.p = res.offset(wanted as isize);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaD_poscall(mut L: *mut lua_State, mut ci: *mut CallInfo, mut nres: i32) {
    let mut wanted: i32 = (*ci).nresults as i32;
    if (((*L).hookmask != 0 && !(wanted < -(1i32))) as i32 != 0i32) as i32 as i64 != 0 {
        rethook(L, ci, nres);
    }
    moveresults(L, (*ci).func.p, nres, wanted);
    (*L).ci = (*ci).previous;
}
#[inline]
unsafe extern "C" fn prepCallInfo(
    mut L: *mut lua_State,
    mut func: StkId,
    mut nret: i32,
    mut mask: i32,
    mut top: StkId,
) -> *mut CallInfo {
    (*L).ci = if !((*(*L).ci).next).is_null() {
        (*(*L).ci).next
    } else {
        luaE_extendCI(L)
    };
    let mut ci: *mut CallInfo = (*L).ci;
    (*ci).func.p = func;
    (*ci).nresults = nret as libc::c_short;
    (*ci).callstatus = mask as libc::c_ushort;
    (*ci).top.p = top;
    return ci;
}
#[inline]
unsafe extern "C" fn precallC(
    mut L: *mut lua_State,
    mut func: StkId,
    mut nresults: i32,
    mut f: CFunction,
) -> i32 {
    let mut n: i32 = 0;
    let mut ci: *mut CallInfo = 0 as *mut CallInfo;
    if ((((*L).stack_last.p).offset_from((*L).top.p) as i64 <= 20i32 as i64) as i32 != 0i32) as i32
        as i64
        != 0
    {
        let mut t__: i64 =
            (func as *mut libc::c_char).offset_from((*L).stack.p as *mut libc::c_char) as i64;
        if (*(*L).l_G).GCdebt > 0i32 as i64 {
            luaC_step(L);
        }
        luaD_growstack(L, 20i32, 1i32);
        func = ((*L).stack.p as *mut libc::c_char).offset(t__ as isize) as StkId;
    }
    ci = prepCallInfo(
        L,
        func,
        nresults,
        (1i32) << 1i32,
        ((*L).top.p).offset(20i32 as isize),
    );
    (*L).ci = ci;
    if ((*L).hookmask & (1i32) << 0i32 != 0i32) as i32 as i64 != 0 {
        let mut narg: i32 = ((*L).top.p).offset_from(func) as i64 as i32 - 1i32;
        luaD_hook(L, 0i32, -(1i32), 1i32, narg);
    }
    n = (Some(f.expect("non-null function pointer"))).expect("non-null function pointer")(L);
    luaD_poscall(L, ci, n);
    return n;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaD_pretailcall(
    mut L: *mut lua_State,
    mut ci: *mut CallInfo,
    mut func: StkId,
    mut narg1: i32,
    mut delta: i32,
) -> i32 {
    loop {
        match (*func).val.tt_ as i32 & 0x3f as i32 {
            38 => {
                return precallC(
                    L,
                    func,
                    -(1i32),
                    (*((*func).val.value_.gc as *mut GCUnion)).cl.c.f,
                );
            }
            22 => return precallC(L, func, -(1i32), (*func).val.value_.f),
            6 => {
                let mut p: *mut Proto = (*((*func).val.value_.gc as *mut GCUnion)).cl.l.p;
                let mut fsize: i32 = (*p).maxstacksize as i32;
                let mut nfixparams: i32 = (*p).numparams as i32;
                let mut i: i32 = 0;
                if ((((*L).stack_last.p).offset_from((*L).top.p) as i64 <= (fsize - delta) as i64)
                    as i32
                    != 0i32) as i32 as i64
                    != 0
                {
                    let mut t__: i64 = (func as *mut libc::c_char)
                        .offset_from((*L).stack.p as *mut libc::c_char)
                        as i64;
                    if (*(*L).l_G).GCdebt > 0i32 as i64 {
                        luaC_step(L);
                    }
                    luaD_growstack(L, fsize - delta, 1i32);
                    func = ((*L).stack.p as *mut libc::c_char).offset(t__ as isize) as StkId;
                }
                (*ci).func.p = ((*ci).func.p).offset(-(delta as isize));
                i = 0i32;
                while i < narg1 {
                    let mut io1: *mut TValue = &mut (*((*ci).func.p).offset(i as isize)).val;
                    let mut io2: *const TValue = &mut (*func.offset(i as isize)).val;
                    (*io1).value_ = (*io2).value_;
                    (*io1).tt_ = (*io2).tt_;
                    i += 1;
                }
                func = (*ci).func.p;
                while narg1 <= nfixparams {
                    (*func.offset(narg1 as isize)).val.tt_ = (0i32 | (0i32) << 4i32) as u8;
                    narg1 += 1;
                }
                (*ci).top.p = func.offset(1i32 as isize).offset(fsize as isize);
                (*ci).u.l.savedpc = (*p).code;
                (*ci).callstatus = ((*ci).callstatus as i32 | (1i32) << 5i32) as libc::c_ushort;
                (*L).top.p = func.offset(narg1 as isize);
                return -(1i32);
            }
            _ => {
                func = tryfuncTM(L, func);
                narg1 += 1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaD_precall(
    mut L: *mut lua_State,
    mut func: StkId,
    mut nresults: i32,
) -> *mut CallInfo {
    loop {
        match (*func).val.tt_ as i32 & 0x3f as i32 {
            38 => {
                precallC(
                    L,
                    func,
                    nresults,
                    (*((*func).val.value_.gc as *mut GCUnion)).cl.c.f,
                );
                return 0 as *mut CallInfo;
            }
            22 => {
                precallC(L, func, nresults, (*func).val.value_.f);
                return 0 as *mut CallInfo;
            }
            6 => {
                let mut ci: *mut CallInfo = 0 as *mut CallInfo;
                let mut p: *mut Proto = (*((*func).val.value_.gc as *mut GCUnion)).cl.l.p;
                let mut narg: i32 = ((*L).top.p).offset_from(func) as i64 as i32 - 1i32;
                let mut nfixparams: i32 = (*p).numparams as i32;
                let mut fsize: i32 = (*p).maxstacksize as i32;
                if ((((*L).stack_last.p).offset_from((*L).top.p) as i64 <= fsize as i64) as i32
                    != 0i32) as i32 as i64
                    != 0
                {
                    let mut t__: i64 = (func as *mut libc::c_char)
                        .offset_from((*L).stack.p as *mut libc::c_char)
                        as i64;
                    if (*(*L).l_G).GCdebt > 0i32 as i64 {
                        luaC_step(L);
                    }
                    luaD_growstack(L, fsize, 1i32);
                    func = ((*L).stack.p as *mut libc::c_char).offset(t__ as isize) as StkId;
                }
                ci = prepCallInfo(
                    L,
                    func,
                    nresults,
                    0i32,
                    func.offset(1i32 as isize).offset(fsize as isize),
                );
                (*L).ci = ci;
                (*ci).u.l.savedpc = (*p).code;
                while narg < nfixparams {
                    let fresh1 = (*L).top.p;
                    (*L).top.p = ((*L).top.p).offset(1);
                    (*fresh1).val.tt_ = (0i32 | (0i32) << 4i32) as u8;
                    narg += 1;
                }
                return ci;
            }
            _ => {
                func = tryfuncTM(L, func);
            }
        }
    }
}
#[inline]
unsafe extern "C" fn ccall(
    mut L: *mut lua_State,
    mut func: StkId,
    mut nResults: i32,
    mut inc: u32,
) {
    let mut ci: *mut CallInfo = 0 as *mut CallInfo;
    (*L).nCcalls = ((*L).nCcalls as u32).wrapping_add(inc) as u32 as u32;
    if (((*L).nCcalls & 0xffff as i32 as u32 >= 200i32 as u32) as i32 != 0i32) as i32 as i64 != 0 {
        if ((((*L).stack_last.p).offset_from((*L).top.p) as i64 <= 0i32 as i64) as i32 != 0i32)
            as i32 as i64
            != 0
        {
            let mut t__: i64 =
                (func as *mut libc::c_char).offset_from((*L).stack.p as *mut libc::c_char) as i64;
            luaD_growstack(L, 0i32, 1i32);
            func = ((*L).stack.p as *mut libc::c_char).offset(t__ as isize) as StkId;
        }
        luaE_checkcstack(L);
    }
    ci = luaD_precall(L, func, nResults);
    if !ci.is_null() {
        (*ci).callstatus = ((1i32) << 2i32) as libc::c_ushort;
        luaV_execute(L, ci);
    }
    (*L).nCcalls = ((*L).nCcalls as u32).wrapping_sub(inc) as u32 as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaD_call(mut L: *mut lua_State, mut func: StkId, mut nResults: i32) {
    ccall(L, func, nResults, 1i32 as u32);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaD_callnoyield(
    mut L: *mut lua_State,
    mut func: StkId,
    mut nResults: i32,
) {
    ccall(L, func, nResults, (0x10000 as i32 | 1i32) as u32);
}
unsafe extern "C" fn finishpcallk(mut L: *mut lua_State, mut ci: *mut CallInfo) -> i32 {
    let mut status: i32 = (*ci).callstatus as i32 >> 10i32 & 7i32;
    if ((status == 0i32) as i32 != 0i32) as i32 as i64 != 0 {
        status = 1i32;
    } else {
        let mut func: StkId =
            ((*L).stack.p as *mut libc::c_char).offset((*ci).u2.funcidx as isize) as StkId;
        (*L).allowhook = ((*ci).callstatus as i32 & (1i32) << 0i32) as u8;
        func = luaF_close(L, func, status, 1i32);
        luaD_seterrorobj(L, status, func);
        luaD_shrinkstack(L);
        (*ci).callstatus =
            ((*ci).callstatus as i32 & !((7i32) << 10i32) | (0i32) << 10i32) as libc::c_ushort;
    }
    (*ci).callstatus = ((*ci).callstatus as i32 & !((1i32) << 4i32)) as libc::c_ushort;
    (*L).errfunc = (*ci).u.c.old_errfunc;
    return status;
}
unsafe extern "C" fn finishCcall(mut L: *mut lua_State, mut ci: *mut CallInfo) {
    let mut n: i32 = 0;
    if (*ci).callstatus as i32 & (1i32) << 9i32 != 0 {
        n = (*ci).u2.nres;
    } else {
        let mut status: i32 = 1i32;
        if (*ci).callstatus as i32 & (1i32) << 4i32 != 0 {
            status = finishpcallk(L, ci);
        }
        if -(1i32) <= -(1i32) && (*(*L).ci).top.p < (*L).top.p {
            (*(*L).ci).top.p = (*L).top.p;
        }
        n = (Some(((*ci).u.c.k).expect("non-null function pointer")))
            .expect("non-null function pointer")(L, status, (*ci).u.c.ctx);
    }
    luaD_poscall(L, ci, n);
}
unsafe extern "C" fn unroll(mut L: *mut lua_State, mut _ud: *mut libc::c_void) {
    let mut ci: *mut CallInfo = 0 as *mut CallInfo;
    loop {
        ci = (*L).ci;
        if !(ci != &mut (*L).base_ci as *mut CallInfo) {
            break;
        }
        if (*ci).callstatus as i32 & (1i32) << 1i32 != 0 {
            finishCcall(L, ci);
        } else {
            luaV_finishOp(L);
            luaV_execute(L, ci);
        }
    }
}
unsafe extern "C" fn findpcall(mut L: *mut lua_State) -> *mut CallInfo {
    let mut ci: *mut CallInfo = 0 as *mut CallInfo;
    ci = (*L).ci;
    while !ci.is_null() {
        if (*ci).callstatus as i32 & (1i32) << 4i32 != 0 {
            return ci;
        }
        ci = (*ci).previous;
    }
    return 0 as *mut CallInfo;
}
unsafe extern "C" fn resume_error(
    mut L: *mut lua_State,
    mut msg: *const libc::c_char,
    mut narg: i32,
) -> i32 {
    (*L).top.p = ((*L).top.p).offset(-(narg as isize));
    let mut io: *mut TValue = &mut (*(*L).top.p).val;
    let mut x_: *mut TString = luaS_new(L, msg);
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
    (*io).tt_ = ((*x_).tt as i32 | (1i32) << 6i32) as u8;
    (*L).top.p = ((*L).top.p).offset(1);
    (*L).top.p;
    return 2i32;
}
unsafe extern "C" fn resume(mut L: *mut lua_State, mut ud: *mut libc::c_void) {
    let mut n: i32 = *(ud as *mut i32);
    let mut firstArg: StkId = ((*L).top.p).offset(-(n as isize));
    let mut ci: *mut CallInfo = (*L).ci;
    if (*L).status as i32 == 0i32 {
        ccall(L, firstArg.offset(-(1i32 as isize)), -(1i32), 0i32 as u32);
    } else {
        (*L).status = 0i32 as u8;
        if (*ci).callstatus as i32 & (1i32) << 1i32 == 0 {
            (*ci).u.l.savedpc = ((*ci).u.l.savedpc).offset(-1);
            (*ci).u.l.savedpc;
            (*L).top.p = firstArg;
            luaV_execute(L, ci);
        } else {
            if ((*ci).u.c.k).is_some() {
                n = (Some(((*ci).u.c.k).expect("non-null function pointer")))
                    .expect("non-null function pointer")(L, 1i32, (*ci).u.c.ctx);
            }
            luaD_poscall(L, ci, n);
        }
        unroll(L, 0 as *mut libc::c_void);
    };
}
unsafe extern "C" fn precover(mut L: *mut lua_State, mut status: i32) -> i32 {
    let mut ci: *mut CallInfo = 0 as *mut CallInfo;
    while status > 1i32 && {
        ci = findpcall(L);
        !ci.is_null()
    } {
        (*L).ci = ci;
        (*ci).callstatus =
            ((*ci).callstatus as i32 & !((7i32) << 10i32) | status << 10i32) as libc::c_ushort;
        status = luaD_rawrunprotected(
            L,
            Some(unroll as unsafe extern "C" fn(*mut lua_State, *mut libc::c_void) -> ()),
            0 as *mut libc::c_void,
        );
    }
    return status;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_resume(
    mut L: *mut lua_State,
    mut from: *mut lua_State,
    mut nargs: i32,
    mut nresults: *mut i32,
) -> i32 {
    let mut status: i32 = 0;
    if (*L).status as i32 == 0i32 {
        if (*L).ci != &mut (*L).base_ci as *mut CallInfo {
            return resume_error(
                L,
                b"cannot resume non-suspended coroutine\0" as *const u8 as *const libc::c_char,
                nargs,
            );
        } else if ((*L).top.p).offset_from(((*(*L).ci).func.p).offset(1i32 as isize)) as i64
            == nargs as i64
        {
            return resume_error(
                L,
                b"cannot resume dead coroutine\0" as *const u8 as *const libc::c_char,
                nargs,
            );
        }
    } else if (*L).status as i32 != 1i32 {
        return resume_error(
            L,
            b"cannot resume dead coroutine\0" as *const u8 as *const libc::c_char,
            nargs,
        );
    }
    (*L).nCcalls = if !from.is_null() {
        (*from).nCcalls & 0xffff as i32 as u32
    } else {
        0i32 as u32
    };
    if (*L).nCcalls & 0xffff as i32 as u32 >= 200i32 as u32 {
        return resume_error(
            L,
            b"C stack overflow\0" as *const u8 as *const libc::c_char,
            nargs,
        );
    }
    (*L).nCcalls = ((*L).nCcalls).wrapping_add(1);
    (*L).nCcalls;
    status = luaD_rawrunprotected(
        L,
        Some(resume as unsafe extern "C" fn(*mut lua_State, *mut libc::c_void) -> ()),
        &mut nargs as *mut i32 as *mut libc::c_void,
    );
    status = precover(L, status);
    if !((!(status > 1i32) as i32 != 0i32) as i32 as i64 != 0) {
        (*L).status = status as u8;
        luaD_seterrorobj(L, status, (*L).top.p);
        (*(*L).ci).top.p = (*L).top.p;
    }
    *nresults = if status == 1i32 {
        (*(*L).ci).u2.nyield
    } else {
        ((*L).top.p).offset_from(((*(*L).ci).func.p).offset(1i32 as isize)) as i64 as i32
    };
    return status;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_isyieldable(mut L: *mut lua_State) -> i32 {
    return ((*L).nCcalls & 0xffff0000 as u32 == 0i32 as u32) as i32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_yieldk(
    mut L: *mut lua_State,
    mut nresults: i32,
    mut ctx: lua_KContext,
    mut k: lua_KFunction,
) -> i32 {
    let mut ci: *mut CallInfo = 0 as *mut CallInfo;
    ci = (*L).ci;
    if (!((*L).nCcalls & 0xffff0000 as u32 == 0i32 as u32) as i32 != 0i32) as i32 as i64 != 0 {
        if L != (*(*L).l_G).mainthread {
            luaG_runerror(
                L,
                b"attempt to yield across a C-call boundary\0" as *const u8 as *const libc::c_char,
            );
        } else {
            luaG_runerror(
                L,
                b"attempt to yield from outside a coroutine\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    (*L).status = 1i32 as u8;
    (*ci).u2.nyield = nresults;
    if (*ci).callstatus as i32 & (1i32) << 1i32 == 0 {
    } else {
        (*ci).u.c.k = k;
        if ((*ci).u.c.k).is_some() {
            (*ci).u.c.ctx = ctx;
        }
        luaD_throw(L, 1i32);
    }
    return 0i32;
}
unsafe extern "C" fn closepaux(mut L: *mut lua_State, mut ud: *mut libc::c_void) {
    let mut pcl: *mut CloseP = ud as *mut CloseP;
    luaF_close(L, (*pcl).level, (*pcl).status, 0i32);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaD_closeprotected(
    mut L: *mut lua_State,
    mut level: i64,
    mut status: i32,
) -> i32 {
    let mut old_ci: *mut CallInfo = (*L).ci;
    let mut old_allowhooks: u8 = (*L).allowhook;
    loop {
        let mut pcl: CloseP = CloseP {
            level: 0 as *mut StackValue,
            status: 0,
        };
        pcl.level = ((*L).stack.p as *mut libc::c_char).offset(level as isize) as StkId;
        pcl.status = status;
        status = luaD_rawrunprotected(
            L,
            Some(closepaux as unsafe extern "C" fn(*mut lua_State, *mut libc::c_void) -> ()),
            &mut pcl as *mut CloseP as *mut libc::c_void,
        );
        if ((status == 0i32) as i32 != 0i32) as i32 as i64 != 0 {
            return pcl.status;
        } else {
            (*L).ci = old_ci;
            (*L).allowhook = old_allowhooks;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaD_pcall(
    mut L: *mut lua_State,
    mut func: Pfunc,
    mut u: *mut libc::c_void,
    mut old_top: i64,
    mut ef: i64,
) -> i32 {
    let mut status: i32 = 0;
    let mut old_ci: *mut CallInfo = (*L).ci;
    let mut old_allowhooks: u8 = (*L).allowhook;
    let mut old_errfunc: i64 = (*L).errfunc;
    (*L).errfunc = ef;
    status = luaD_rawrunprotected(L, func, u);
    if ((status != 0i32) as i32 != 0i32) as i32 as i64 != 0 {
        (*L).ci = old_ci;
        (*L).allowhook = old_allowhooks;
        status = luaD_closeprotected(L, old_top, status);
        luaD_seterrorobj(
            L,
            status,
            ((*L).stack.p as *mut libc::c_char).offset(old_top as isize) as StkId,
        );
        luaD_shrinkstack(L);
    }
    (*L).errfunc = old_errfunc;
    return status;
}
unsafe extern "C" fn checkmode(
    mut L: *mut lua_State,
    mut mode: *const libc::c_char,
    mut x: *const libc::c_char,
) {
    if !mode.is_null() && (strchr(mode, *x.offset(0i32 as isize) as i32)).is_null() {
        luaO_pushfstring(
            L,
            b"attempt to load a %s chunk (mode is '%s')\0" as *const u8 as *const libc::c_char,
            x,
            mode,
        );
        luaD_throw(L, 3i32);
    }
}
unsafe extern "C" fn f_parser(mut L: *mut lua_State, mut ud: *mut libc::c_void) {
    let mut cl: *mut LClosure = 0 as *mut LClosure;
    let mut p: *mut SParser = ud as *mut SParser;
    let fresh2 = (*(*p).z).n;
    (*(*p).z).n = ((*(*p).z).n).wrapping_sub(1);
    let mut c: i32 = if fresh2 > 0i32 as libc::c_ulong {
        let fresh3 = (*(*p).z).p;
        (*(*p).z).p = ((*(*p).z).p).offset(1);
        *fresh3 as u8 as i32
    } else {
        luaZ_fill((*p).z)
    };
    if c == (*::core::mem::transmute::<&[u8; 5], &[libc::c_char; 5]>(b"\x1BLua\0"))[0i32 as usize]
        as i32
    {
        checkmode(
            L,
            (*p).mode,
            b"binary\0" as *const u8 as *const libc::c_char,
        );
        cl = luaU_undump(L, (*p).z, (*p).name);
    } else {
        checkmode(L, (*p).mode, b"text\0" as *const u8 as *const libc::c_char);
        cl = luaY_parser(L, (*p).z, &mut (*p).buff, &mut (*p).dyd, (*p).name, c);
    }
    luaF_initupvals(L, cl);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaD_protectedparser(
    mut L: *mut lua_State,
    mut z: *mut ZIO,
    mut name: *const libc::c_char,
    mut mode: *const libc::c_char,
) -> i32 {
    let mut p: SParser = SParser {
        z: 0 as *mut ZIO,
        buff: Mbuffer {
            buffer: 0 as *mut libc::c_char,
            n: 0,
            buffsize: 0,
        },
        dyd: Dyndata {
            actvar: C2RustUnnamed_9 {
                arr: 0 as *mut Vardesc,
                n: 0,
                size: 0,
            },
            gt: Labellist {
                arr: 0 as *mut Labeldesc,
                n: 0,
                size: 0,
            },
            label: Labellist {
                arr: 0 as *mut Labeldesc,
                n: 0,
                size: 0,
            },
        },
        mode: 0 as *const libc::c_char,
        name: 0 as *const libc::c_char,
    };
    let mut status: i32 = 0;
    (*L).nCcalls = ((*L).nCcalls as u32).wrapping_add(0x10000 as i32 as u32) as u32 as u32;
    p.z = z;
    p.name = name;
    p.mode = mode;
    p.dyd.actvar.arr = 0 as *mut Vardesc;
    p.dyd.actvar.size = 0i32;
    p.dyd.gt.arr = 0 as *mut Labeldesc;
    p.dyd.gt.size = 0i32;
    p.dyd.label.arr = 0 as *mut Labeldesc;
    p.dyd.label.size = 0i32;
    p.buff.buffer = 0 as *mut libc::c_char;
    p.buff.buffsize = 0i32 as u64;
    status = luaD_pcall(
        L,
        Some(f_parser as unsafe extern "C" fn(*mut lua_State, *mut libc::c_void) -> ()),
        &mut p as *mut SParser as *mut libc::c_void,
        ((*L).top.p as *mut libc::c_char).offset_from((*L).stack.p as *mut libc::c_char) as i64,
        (*L).errfunc,
    );
    p.buff.buffer = luaM_saferealloc_(
        L,
        p.buff.buffer as *mut libc::c_void,
        (p.buff.buffsize).wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
        (0i32 as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    p.buff.buffsize = 0i32 as u64;
    luaM_free_(
        L,
        p.dyd.actvar.arr as *mut libc::c_void,
        (p.dyd.actvar.size as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<Vardesc>() as libc::c_ulong),
    );
    luaM_free_(
        L,
        p.dyd.gt.arr as *mut libc::c_void,
        (p.dyd.gt.size as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<Labeldesc>() as libc::c_ulong),
    );
    luaM_free_(
        L,
        p.dyd.label.arr as *mut libc::c_void,
        (p.dyd.label.size as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<Labeldesc>() as libc::c_ulong),
    );
    (*L).nCcalls = ((*L).nCcalls as u32).wrapping_sub(0x10000 as i32 as u32) as u32 as u32;
    return status;
}
