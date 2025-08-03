#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
unsafe extern "C" {
    fn _setjmp(_: *mut __jmp_buf_tag) -> libc::c_int;
    fn _longjmp(__env: *mut __jmp_buf_tag, __val: libc::c_int) -> !;
    fn abort() -> !;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn luaE_extendCI(L: *mut lua_State) -> *mut CallInfo;
    fn luaE_shrinkCI(L: *mut lua_State);
    fn luaE_checkcstack(L: *mut lua_State);
    fn luaE_resetthread(L: *mut lua_State, status: libc::c_int) -> libc::c_int;
    fn luaO_pushfstring(
        L: *mut lua_State,
        fmt: *const libc::c_char,
        _: ...
    ) -> *const libc::c_char;
    fn luaT_gettmbyobj(L: *mut lua_State, o: *const TValue, event: TMS) -> *const TValue;
    fn luaM_realloc_(
        L: *mut lua_State,
        block: *mut libc::c_void,
        oldsize: size_t,
        size: size_t,
    ) -> *mut libc::c_void;
    fn luaM_saferealloc_(
        L: *mut lua_State,
        block: *mut libc::c_void,
        oldsize: size_t,
        size: size_t,
    ) -> *mut libc::c_void;
    fn luaM_free_(L: *mut lua_State, block: *mut libc::c_void, osize: size_t);
    fn luaZ_fill(z: *mut ZIO) -> libc::c_int;
    fn luaG_callerror(L: *mut lua_State, o: *const TValue) -> !;
    fn luaG_runerror(L: *mut lua_State, fmt: *const libc::c_char, _: ...) -> !;
    fn luaF_initupvals(L: *mut lua_State, cl: *mut LClosure);
    fn luaF_close(
        L: *mut lua_State,
        level: StkId,
        status: libc::c_int,
        yy: libc::c_int,
    ) -> StkId;
    fn luaC_step(L: *mut lua_State);
    fn luaY_parser(
        L: *mut lua_State,
        z: *mut ZIO,
        buff: *mut Mbuffer,
        dyd: *mut Dyndata,
        name: *const libc::c_char,
        firstchar: libc::c_int,
    ) -> *mut LClosure;
    fn luaS_newlstr(
        L: *mut lua_State,
        str: *const libc::c_char,
        l: size_t,
    ) -> *mut TString;
    fn luaS_new(L: *mut lua_State, str: *const libc::c_char) -> *mut TString;
    fn luaU_undump(
        L: *mut lua_State,
        Z: *mut ZIO,
        name: *const libc::c_char,
    ) -> *mut LClosure;
    fn luaV_finishOp(L: *mut lua_State);
    fn luaV_execute(L: *mut lua_State, ci: *mut CallInfo);
}
pub type __jmp_buf = [libc::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
pub type size_t = libc::c_ulong;
pub type __sig_atomic_t = libc::c_int;
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
pub struct lua_longjmp {
    pub previous: *mut lua_longjmp,
    pub b: jmp_buf,
    pub status: libc::c_int,
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
pub type lua_Reader = Option::<
    unsafe extern "C" fn(
        *mut lua_State,
        *mut libc::c_void,
        *mut size_t,
    ) -> *const libc::c_char,
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
pub type ls_byte = libc::c_schar;
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
pub union Closure {
    pub c: CClosure,
    pub l: LClosure,
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
pub union UValue {
    pub uv: TValue,
    pub n: lua_Number,
    pub u: libc::c_double,
    pub s: *mut libc::c_void,
    pub i: lua_Integer,
    pub l: libc::c_long,
}
pub type Pfunc = Option::<unsafe extern "C" fn(*mut lua_State, *mut libc::c_void) -> ()>;
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
pub struct Labeldesc {
    pub name: *mut TString,
    pub pc: libc::c_int,
    pub line: libc::c_int,
    pub nactvar: lu_byte,
    pub close: lu_byte,
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
pub struct Dyndata {
    pub actvar: C2RustUnnamed_9,
    pub gt: Labellist,
    pub label: Labellist,
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
    pub tt_: lu_byte,
    pub kind: lu_byte,
    pub ridx: lu_byte,
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
    pub status: libc::c_int,
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaD_seterrorobj(
    mut L: *mut lua_State,
    mut errcode: libc::c_int,
    mut oldtop: StkId,
) {
    match errcode {
        4 => {
            let mut io: *mut TValue = &mut (*oldtop).val;
            let mut x_: *mut TString = (*(*L).l_G).memerrmsg;
            (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
            (*io)
                .tt_ = ((*x_).tt as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int)
                as lu_byte;
        }
        0 => {
            (*oldtop)
                .val
                .tt_ = (0 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int)
                as lu_byte;
        }
        _ => {
            let mut io1: *mut TValue = &mut (*oldtop).val;
            let mut io2: *const TValue = &mut (*((*L).top.p)
                .offset(-(1 as libc::c_int as isize)))
                .val;
            (*io1).value_ = (*io2).value_;
            (*io1).tt_ = (*io2).tt_;
        }
    }
    (*L).top.p = oldtop.offset(1 as libc::c_int as isize);
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaD_throw(
    mut L: *mut lua_State,
    mut errcode: libc::c_int,
) -> ! {
    if !((*L).errorJmp).is_null() {
        ::core::ptr::write_volatile(
            &mut (*(*L).errorJmp).status as *mut libc::c_int,
            errcode,
        );
        _longjmp(((*(*L).errorJmp).b).as_mut_ptr(), 1 as libc::c_int);
    } else {
        let mut g: *mut global_State = (*L).l_G;
        errcode = luaE_resetthread(L, errcode);
        (*L).status = errcode as lu_byte;
        if !((*(*g).mainthread).errorJmp).is_null() {
            let fresh0 = (*(*g).mainthread).top.p;
            (*(*g).mainthread).top.p = ((*(*g).mainthread).top.p).offset(1);
            let mut io1: *mut TValue = &mut (*fresh0).val;
            let mut io2: *const TValue = &mut (*((*L).top.p)
                .offset(-(1 as libc::c_int as isize)))
                .val;
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
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaD_rawrunprotected(
    mut L: *mut lua_State,
    mut f: Pfunc,
    mut ud: *mut libc::c_void,
) -> libc::c_int {
    let mut oldnCcalls: l_uint32 = (*L).nCcalls;
    let mut lj: lua_longjmp = lua_longjmp {
        previous: 0 as *mut lua_longjmp,
        b: [__jmp_buf_tag {
            __jmpbuf: [0; 8],
            __mask_was_saved: 0,
            __saved_mask: __sigset_t { __val: [0; 16] },
        }; 1],
        status: 0,
    };
    ::core::ptr::write_volatile(&mut lj.status as *mut libc::c_int, 0 as libc::c_int);
    lj.previous = (*L).errorJmp;
    (*L).errorJmp = &mut lj;
    if _setjmp((lj.b).as_mut_ptr()) == 0 as libc::c_int {
        (Some(f.expect("non-null function pointer")))
            .expect("non-null function pointer")(L, ud);
    }
    (*L).errorJmp = lj.previous;
    (*L).nCcalls = oldnCcalls;
    return lj.status;
}
unsafe extern "C" fn relstack(mut L: *mut lua_State) {
    let mut ci: *mut CallInfo = 0 as *mut CallInfo;
    let mut up: *mut UpVal = 0 as *mut UpVal;
    (*L)
        .top
        .offset = ((*L).top.p as *mut libc::c_char)
        .offset_from((*L).stack.p as *mut libc::c_char) as libc::c_long;
    (*L)
        .tbclist
        .offset = ((*L).tbclist.p as *mut libc::c_char)
        .offset_from((*L).stack.p as *mut libc::c_char) as libc::c_long;
    up = (*L).openupval;
    while !up.is_null() {
        (*up)
            .v
            .offset = ((*up).v.p as StkId as *mut libc::c_char)
            .offset_from((*L).stack.p as *mut libc::c_char) as libc::c_long;
        up = (*up).u.open.next;
    }
    ci = (*L).ci;
    while !ci.is_null() {
        (*ci)
            .top
            .offset = ((*ci).top.p as *mut libc::c_char)
            .offset_from((*L).stack.p as *mut libc::c_char) as libc::c_long;
        (*ci)
            .func
            .offset = ((*ci).func.p as *mut libc::c_char)
            .offset_from((*L).stack.p as *mut libc::c_char) as libc::c_long;
        ci = (*ci).previous;
    }
}
unsafe extern "C" fn correctstack(mut L: *mut lua_State) {
    let mut ci: *mut CallInfo = 0 as *mut CallInfo;
    let mut up: *mut UpVal = 0 as *mut UpVal;
    (*L)
        .top
        .p = ((*L).stack.p as *mut libc::c_char).offset((*L).top.offset as isize)
        as StkId;
    (*L)
        .tbclist
        .p = ((*L).stack.p as *mut libc::c_char).offset((*L).tbclist.offset as isize)
        as StkId;
    up = (*L).openupval;
    while !up.is_null() {
        (*up)
            .v
            .p = &mut (*(((*L).stack.p as *mut libc::c_char)
            .offset((*up).v.offset as isize) as StkId))
            .val;
        up = (*up).u.open.next;
    }
    ci = (*L).ci;
    while !ci.is_null() {
        (*ci)
            .top
            .p = ((*L).stack.p as *mut libc::c_char).offset((*ci).top.offset as isize)
            as StkId;
        (*ci)
            .func
            .p = ((*L).stack.p as *mut libc::c_char).offset((*ci).func.offset as isize)
            as StkId;
        if (*ci).callstatus as libc::c_int & (1 as libc::c_int) << 1 as libc::c_int == 0
        {
            ::core::ptr::write_volatile(
                &mut (*ci).u.l.trap as *mut sig_atomic_t,
                1 as libc::c_int,
            );
        }
        ci = (*ci).previous;
    }
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaD_errerr(mut L: *mut lua_State) -> ! {
    let mut msg: *mut TString = luaS_newlstr(
        L,
        b"error in error handling\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    let mut io: *mut TValue = &mut (*(*L).top.p).val;
    let mut x_: *mut TString = msg;
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
    (*io)
        .tt_ = ((*x_).tt as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int)
        as lu_byte;
    (*L).top.p = ((*L).top.p).offset(1);
    (*L).top.p;
    luaD_throw(L, 5 as libc::c_int);
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaD_reallocstack(
    mut L: *mut lua_State,
    mut newsize: libc::c_int,
    mut raiseerror: libc::c_int,
) -> libc::c_int {
    let mut oldsize: libc::c_int = ((*L).stack_last.p).offset_from((*L).stack.p)
        as libc::c_long as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut newstack: StkId = 0 as *mut StackValue;
    let mut oldgcstop: libc::c_int = (*(*L).l_G).gcstopem as libc::c_int;
    relstack(L);
    (*(*L).l_G).gcstopem = 1 as libc::c_int as lu_byte;
    newstack = luaM_realloc_(
        L,
        (*L).stack.p as *mut libc::c_void,
        ((oldsize + 5 as libc::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<StackValue>() as libc::c_ulong),
        ((newsize + 5 as libc::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<StackValue>() as libc::c_ulong),
    ) as *mut StackValue;
    (*(*L).l_G).gcstopem = oldgcstop as lu_byte;
    if ((newstack == 0 as *mut libc::c_void as StkId) as libc::c_int != 0 as libc::c_int)
        as libc::c_int as libc::c_long != 0
    {
        correctstack(L);
        if raiseerror != 0 {
            luaD_throw(L, 4 as libc::c_int);
        } else {
            return 0 as libc::c_int
        }
    }
    (*L).stack.p = newstack;
    correctstack(L);
    (*L).stack_last.p = ((*L).stack.p).offset(newsize as isize);
    i = oldsize + 5 as libc::c_int;
    while i < newsize + 5 as libc::c_int {
        (*newstack.offset(i as isize))
            .val
            .tt_ = (0 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int)
            as lu_byte;
        i += 1;
        i;
    }
    return 1 as libc::c_int;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaD_growstack(
    mut L: *mut lua_State,
    mut n: libc::c_int,
    mut raiseerror: libc::c_int,
) -> libc::c_int {
    let mut size: libc::c_int = ((*L).stack_last.p).offset_from((*L).stack.p)
        as libc::c_long as libc::c_int;
    if ((size > 1000000 as libc::c_int) as libc::c_int != 0 as libc::c_int)
        as libc::c_int as libc::c_long != 0
    {
        if raiseerror != 0 {
            luaD_errerr(L);
        }
        return 0 as libc::c_int;
    } else if n < 1000000 as libc::c_int {
        let mut newsize: libc::c_int = 2 as libc::c_int * size;
        let mut needed: libc::c_int = ((*L).top.p).offset_from((*L).stack.p)
            as libc::c_long as libc::c_int + n;
        if newsize > 1000000 as libc::c_int {
            newsize = 1000000 as libc::c_int;
        }
        if newsize < needed {
            newsize = needed;
        }
        if ((newsize <= 1000000 as libc::c_int) as libc::c_int != 0 as libc::c_int)
            as libc::c_int as libc::c_long != 0
        {
            return luaD_reallocstack(L, newsize, raiseerror);
        }
    }
    luaD_reallocstack(L, 1000000 as libc::c_int + 200 as libc::c_int, raiseerror);
    if raiseerror != 0 {
        luaG_runerror(L, b"stack overflow\0" as *const u8 as *const libc::c_char);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn stackinuse(mut L: *mut lua_State) -> libc::c_int {
    let mut ci: *mut CallInfo = 0 as *mut CallInfo;
    let mut res: libc::c_int = 0;
    let mut lim: StkId = (*L).top.p;
    ci = (*L).ci;
    while !ci.is_null() {
        if lim < (*ci).top.p {
            lim = (*ci).top.p;
        }
        ci = (*ci).previous;
    }
    res = lim.offset_from((*L).stack.p) as libc::c_long as libc::c_int
        + 1 as libc::c_int;
    if res < 20 as libc::c_int {
        res = 20 as libc::c_int;
    }
    return res;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaD_shrinkstack(mut L: *mut lua_State) {
    let mut inuse: libc::c_int = stackinuse(L);
    let mut max: libc::c_int = if inuse > 1000000 as libc::c_int / 3 as libc::c_int {
        1000000 as libc::c_int
    } else {
        inuse * 3 as libc::c_int
    };
    if inuse <= 1000000 as libc::c_int
        && ((*L).stack_last.p).offset_from((*L).stack.p) as libc::c_long as libc::c_int
            > max
    {
        let mut nsize: libc::c_int = if inuse > 1000000 as libc::c_int / 2 as libc::c_int
        {
            1000000 as libc::c_int
        } else {
            inuse * 2 as libc::c_int
        };
        luaD_reallocstack(L, nsize, 0 as libc::c_int);
    }
    luaE_shrinkCI(L);
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaD_inctop(mut L: *mut lua_State) {
    if ((((*L).stack_last.p).offset_from((*L).top.p) as libc::c_long
        <= 1 as libc::c_int as libc::c_long) as libc::c_int != 0 as libc::c_int)
        as libc::c_int as libc::c_long != 0
    {
        luaD_growstack(L, 1 as libc::c_int, 1 as libc::c_int);
    }
    (*L).top.p = ((*L).top.p).offset(1);
    (*L).top.p;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaD_hook(
    mut L: *mut lua_State,
    mut event: libc::c_int,
    mut line: libc::c_int,
    mut ftransfer: libc::c_int,
    mut ntransfer: libc::c_int,
) {
    let mut hook: lua_Hook = (*L).hook;
    if hook.is_some() && (*L).allowhook as libc::c_int != 0 {
        let mut mask: libc::c_int = (1 as libc::c_int) << 3 as libc::c_int;
        let mut ci: *mut CallInfo = (*L).ci;
        let mut top: ptrdiff_t = ((*L).top.p as *mut libc::c_char)
            .offset_from((*L).stack.p as *mut libc::c_char) as libc::c_long;
        let mut ci_top: ptrdiff_t = ((*ci).top.p as *mut libc::c_char)
            .offset_from((*L).stack.p as *mut libc::c_char) as libc::c_long;
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
        if ntransfer != 0 as libc::c_int {
            mask |= (1 as libc::c_int) << 8 as libc::c_int;
            (*ci).u2.transferinfo.ftransfer = ftransfer as libc::c_ushort;
            (*ci).u2.transferinfo.ntransfer = ntransfer as libc::c_ushort;
        }
        if (*ci).callstatus as libc::c_int & (1 as libc::c_int) << 1 as libc::c_int == 0
            && (*L).top.p < (*ci).top.p
        {
            (*L).top.p = (*ci).top.p;
        }
        if ((((*L).stack_last.p).offset_from((*L).top.p) as libc::c_long
            <= 20 as libc::c_int as libc::c_long) as libc::c_int != 0 as libc::c_int)
            as libc::c_int as libc::c_long != 0
        {
            luaD_growstack(L, 20 as libc::c_int, 1 as libc::c_int);
        }
        if (*ci).top.p < ((*L).top.p).offset(20 as libc::c_int as isize) {
            (*ci).top.p = ((*L).top.p).offset(20 as libc::c_int as isize);
        }
        (*L).allowhook = 0 as libc::c_int as lu_byte;
        (*ci).callstatus = ((*ci).callstatus as libc::c_int | mask) as libc::c_ushort;
        (Some(hook.expect("non-null function pointer")))
            .expect("non-null function pointer")(L, &mut ar);
        (*L).allowhook = 1 as libc::c_int as lu_byte;
        (*ci)
            .top
            .p = ((*L).stack.p as *mut libc::c_char).offset(ci_top as isize) as StkId;
        (*L).top.p = ((*L).stack.p as *mut libc::c_char).offset(top as isize) as StkId;
        (*ci).callstatus = ((*ci).callstatus as libc::c_int & !mask) as libc::c_ushort;
    }
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaD_hookcall(mut L: *mut lua_State, mut ci: *mut CallInfo) {
    (*L).oldpc = 0 as libc::c_int;
    if (*L).hookmask & (1 as libc::c_int) << 0 as libc::c_int != 0 {
        let mut event: libc::c_int = if (*ci).callstatus as libc::c_int
            & (1 as libc::c_int) << 5 as libc::c_int != 0
        {
            4 as libc::c_int
        } else {
            0 as libc::c_int
        };
        let mut p: *mut Proto = (*((*(*ci).func.p).val.value_.gc as *mut GCUnion))
            .cl
            .l
            .p;
        (*ci).u.l.savedpc = ((*ci).u.l.savedpc).offset(1);
        (*ci).u.l.savedpc;
        luaD_hook(
            L,
            event,
            -(1 as libc::c_int),
            1 as libc::c_int,
            (*p).numparams as libc::c_int,
        );
        (*ci).u.l.savedpc = ((*ci).u.l.savedpc).offset(-1);
        (*ci).u.l.savedpc;
    }
}
unsafe extern "C" fn rethook(
    mut L: *mut lua_State,
    mut ci: *mut CallInfo,
    mut nres: libc::c_int,
) {
    if (*L).hookmask & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        let mut firstres: StkId = ((*L).top.p).offset(-(nres as isize));
        let mut delta: libc::c_int = 0 as libc::c_int;
        let mut ftransfer: libc::c_int = 0;
        if (*ci).callstatus as libc::c_int & (1 as libc::c_int) << 1 as libc::c_int == 0
        {
            let mut p: *mut Proto = (*((*(*ci).func.p).val.value_.gc as *mut GCUnion))
                .cl
                .l
                .p;
            if (*p).is_vararg != 0 {
                delta = (*ci).u.l.nextraargs + (*p).numparams as libc::c_int
                    + 1 as libc::c_int;
            }
        }
        (*ci).func.p = ((*ci).func.p).offset(delta as isize);
        ftransfer = firstres.offset_from((*ci).func.p) as libc::c_long as libc::c_ushort
            as libc::c_int;
        luaD_hook(L, 1 as libc::c_int, -(1 as libc::c_int), ftransfer, nres);
        (*ci).func.p = ((*ci).func.p).offset(-(delta as isize));
    }
    ci = (*ci).previous;
    if (*ci).callstatus as libc::c_int & (1 as libc::c_int) << 1 as libc::c_int == 0 {
        (*L)
            .oldpc = ((*ci).u.l.savedpc)
            .offset_from(
                (*(*((*(*ci).func.p).val.value_.gc as *mut GCUnion)).cl.l.p).code,
            ) as libc::c_long as libc::c_int - 1 as libc::c_int;
    }
}
unsafe extern "C" fn tryfuncTM(mut L: *mut lua_State, mut func: StkId) -> StkId {
    let mut tm: *const TValue = 0 as *const TValue;
    let mut p: StkId = 0 as *mut StackValue;
    if ((((*L).stack_last.p).offset_from((*L).top.p) as libc::c_long
        <= 1 as libc::c_int as libc::c_long) as libc::c_int != 0 as libc::c_int)
        as libc::c_int as libc::c_long != 0
    {
        let mut t__: ptrdiff_t = (func as *mut libc::c_char)
            .offset_from((*L).stack.p as *mut libc::c_char) as libc::c_long;
        if (*(*L).l_G).GCdebt > 0 as libc::c_int as libc::c_long {
            luaC_step(L);
        }
        luaD_growstack(L, 1 as libc::c_int, 1 as libc::c_int);
        func = ((*L).stack.p as *mut libc::c_char).offset(t__ as isize) as StkId;
    }
    tm = luaT_gettmbyobj(L, &mut (*func).val, TM_CALL);
    if (((*tm).tt_ as libc::c_int & 0xf as libc::c_int == 0 as libc::c_int)
        as libc::c_int != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
    {
        luaG_callerror(L, &mut (*func).val);
    }
    p = (*L).top.p;
    while p > func {
        let mut io1: *mut TValue = &mut (*p).val;
        let mut io2: *const TValue = &mut (*p.offset(-(1 as libc::c_int as isize))).val;
        (*io1).value_ = (*io2).value_;
        (*io1).tt_ = (*io2).tt_;
        p = p.offset(-1);
        p;
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
    mut nres: libc::c_int,
    mut wanted: libc::c_int,
) {
    let mut firstresult: StkId = 0 as *mut StackValue;
    let mut i: libc::c_int = 0;
    match wanted {
        0 => {
            (*L).top.p = res;
            return;
        }
        1 => {
            if nres == 0 as libc::c_int {
                (*res)
                    .val
                    .tt_ = (0 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int)
                    as lu_byte;
            } else {
                let mut io1: *mut TValue = &mut (*res).val;
                let mut io2: *const TValue = &mut (*((*L).top.p)
                    .offset(-(nres as isize)))
                    .val;
                (*io1).value_ = (*io2).value_;
                (*io1).tt_ = (*io2).tt_;
            }
            (*L).top.p = res.offset(1 as libc::c_int as isize);
            return;
        }
        -1 => {
            wanted = nres;
        }
        _ => {
            if wanted < -(1 as libc::c_int) {
                (*(*L).ci)
                    .callstatus = ((*(*L).ci).callstatus as libc::c_int
                    | (1 as libc::c_int) << 9 as libc::c_int) as libc::c_ushort;
                (*(*L).ci).u2.nres = nres;
                res = luaF_close(L, res, -(1 as libc::c_int), 1 as libc::c_int);
                (*(*L).ci)
                    .callstatus = ((*(*L).ci).callstatus as libc::c_int
                    & !((1 as libc::c_int) << 9 as libc::c_int)) as libc::c_ushort;
                if (*L).hookmask != 0 {
                    let mut savedres: ptrdiff_t = (res as *mut libc::c_char)
                        .offset_from((*L).stack.p as *mut libc::c_char) as libc::c_long;
                    rethook(L, (*L).ci, nres);
                    res = ((*L).stack.p as *mut libc::c_char).offset(savedres as isize)
                        as StkId;
                }
                wanted = -wanted - 3 as libc::c_int;
                if wanted == -(1 as libc::c_int) {
                    wanted = nres;
                }
            }
        }
    }
    firstresult = ((*L).top.p).offset(-(nres as isize));
    if nres > wanted {
        nres = wanted;
    }
    i = 0 as libc::c_int;
    while i < nres {
        let mut io1_0: *mut TValue = &mut (*res.offset(i as isize)).val;
        let mut io2_0: *const TValue = &mut (*firstresult.offset(i as isize)).val;
        (*io1_0).value_ = (*io2_0).value_;
        (*io1_0).tt_ = (*io2_0).tt_;
        i += 1;
        i;
    }
    while i < wanted {
        (*res.offset(i as isize))
            .val
            .tt_ = (0 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int)
            as lu_byte;
        i += 1;
        i;
    }
    (*L).top.p = res.offset(wanted as isize);
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaD_poscall(
    mut L: *mut lua_State,
    mut ci: *mut CallInfo,
    mut nres: libc::c_int,
) {
    let mut wanted: libc::c_int = (*ci).nresults as libc::c_int;
    if (((*L).hookmask != 0 && !(wanted < -(1 as libc::c_int))) as libc::c_int
        != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
    {
        rethook(L, ci, nres);
    }
    moveresults(L, (*ci).func.p, nres, wanted);
    (*L).ci = (*ci).previous;
}
#[inline]
unsafe extern "C" fn prepCallInfo(
    mut L: *mut lua_State,
    mut func: StkId,
    mut nret: libc::c_int,
    mut mask: libc::c_int,
    mut top: StkId,
) -> *mut CallInfo {
    (*L)
        .ci = if !((*(*L).ci).next).is_null() {
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
    mut nresults: libc::c_int,
    mut f: lua_CFunction,
) -> libc::c_int {
    let mut n: libc::c_int = 0;
    let mut ci: *mut CallInfo = 0 as *mut CallInfo;
    if ((((*L).stack_last.p).offset_from((*L).top.p) as libc::c_long
        <= 20 as libc::c_int as libc::c_long) as libc::c_int != 0 as libc::c_int)
        as libc::c_int as libc::c_long != 0
    {
        let mut t__: ptrdiff_t = (func as *mut libc::c_char)
            .offset_from((*L).stack.p as *mut libc::c_char) as libc::c_long;
        if (*(*L).l_G).GCdebt > 0 as libc::c_int as libc::c_long {
            luaC_step(L);
        }
        luaD_growstack(L, 20 as libc::c_int, 1 as libc::c_int);
        func = ((*L).stack.p as *mut libc::c_char).offset(t__ as isize) as StkId;
    }
    ci = prepCallInfo(
        L,
        func,
        nresults,
        (1 as libc::c_int) << 1 as libc::c_int,
        ((*L).top.p).offset(20 as libc::c_int as isize),
    );
    (*L).ci = ci;
    if ((*L).hookmask & (1 as libc::c_int) << 0 as libc::c_int != 0 as libc::c_int)
        as libc::c_int as libc::c_long != 0
    {
        let mut narg: libc::c_int = ((*L).top.p).offset_from(func) as libc::c_long
            as libc::c_int - 1 as libc::c_int;
        luaD_hook(L, 0 as libc::c_int, -(1 as libc::c_int), 1 as libc::c_int, narg);
    }
    n = (Some(f.expect("non-null function pointer")))
        .expect("non-null function pointer")(L);
    luaD_poscall(L, ci, n);
    return n;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaD_pretailcall(
    mut L: *mut lua_State,
    mut ci: *mut CallInfo,
    mut func: StkId,
    mut narg1: libc::c_int,
    mut delta: libc::c_int,
) -> libc::c_int {
    loop {
        match (*func).val.tt_ as libc::c_int & 0x3f as libc::c_int {
            38 => {
                return precallC(
                    L,
                    func,
                    -(1 as libc::c_int),
                    (*((*func).val.value_.gc as *mut GCUnion)).cl.c.f,
                );
            }
            22 => return precallC(L, func, -(1 as libc::c_int), (*func).val.value_.f),
            6 => {
                let mut p: *mut Proto = (*((*func).val.value_.gc as *mut GCUnion))
                    .cl
                    .l
                    .p;
                let mut fsize: libc::c_int = (*p).maxstacksize as libc::c_int;
                let mut nfixparams: libc::c_int = (*p).numparams as libc::c_int;
                let mut i: libc::c_int = 0;
                if ((((*L).stack_last.p).offset_from((*L).top.p) as libc::c_long
                    <= (fsize - delta) as libc::c_long) as libc::c_int
                    != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
                {
                    let mut t__: ptrdiff_t = (func as *mut libc::c_char)
                        .offset_from((*L).stack.p as *mut libc::c_char) as libc::c_long;
                    if (*(*L).l_G).GCdebt > 0 as libc::c_int as libc::c_long {
                        luaC_step(L);
                    }
                    luaD_growstack(L, fsize - delta, 1 as libc::c_int);
                    func = ((*L).stack.p as *mut libc::c_char).offset(t__ as isize)
                        as StkId;
                }
                (*ci).func.p = ((*ci).func.p).offset(-(delta as isize));
                i = 0 as libc::c_int;
                while i < narg1 {
                    let mut io1: *mut TValue = &mut (*((*ci).func.p).offset(i as isize))
                        .val;
                    let mut io2: *const TValue = &mut (*func.offset(i as isize)).val;
                    (*io1).value_ = (*io2).value_;
                    (*io1).tt_ = (*io2).tt_;
                    i += 1;
                    i;
                }
                func = (*ci).func.p;
                while narg1 <= nfixparams {
                    (*func.offset(narg1 as isize))
                        .val
                        .tt_ = (0 as libc::c_int
                        | (0 as libc::c_int) << 4 as libc::c_int) as lu_byte;
                    narg1 += 1;
                    narg1;
                }
                (*ci)
                    .top
                    .p = func.offset(1 as libc::c_int as isize).offset(fsize as isize);
                (*ci).u.l.savedpc = (*p).code;
                (*ci)
                    .callstatus = ((*ci).callstatus as libc::c_int
                    | (1 as libc::c_int) << 5 as libc::c_int) as libc::c_ushort;
                (*L).top.p = func.offset(narg1 as isize);
                return -(1 as libc::c_int);
            }
            _ => {
                func = tryfuncTM(L, func);
                narg1 += 1;
                narg1;
            }
        }
    };
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaD_precall(
    mut L: *mut lua_State,
    mut func: StkId,
    mut nresults: libc::c_int,
) -> *mut CallInfo {
    loop {
        match (*func).val.tt_ as libc::c_int & 0x3f as libc::c_int {
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
                let mut p: *mut Proto = (*((*func).val.value_.gc as *mut GCUnion))
                    .cl
                    .l
                    .p;
                let mut narg: libc::c_int = ((*L).top.p).offset_from(func)
                    as libc::c_long as libc::c_int - 1 as libc::c_int;
                let mut nfixparams: libc::c_int = (*p).numparams as libc::c_int;
                let mut fsize: libc::c_int = (*p).maxstacksize as libc::c_int;
                if ((((*L).stack_last.p).offset_from((*L).top.p) as libc::c_long
                    <= fsize as libc::c_long) as libc::c_int != 0 as libc::c_int)
                    as libc::c_int as libc::c_long != 0
                {
                    let mut t__: ptrdiff_t = (func as *mut libc::c_char)
                        .offset_from((*L).stack.p as *mut libc::c_char) as libc::c_long;
                    if (*(*L).l_G).GCdebt > 0 as libc::c_int as libc::c_long {
                        luaC_step(L);
                    }
                    luaD_growstack(L, fsize, 1 as libc::c_int);
                    func = ((*L).stack.p as *mut libc::c_char).offset(t__ as isize)
                        as StkId;
                }
                ci = prepCallInfo(
                    L,
                    func,
                    nresults,
                    0 as libc::c_int,
                    func.offset(1 as libc::c_int as isize).offset(fsize as isize),
                );
                (*L).ci = ci;
                (*ci).u.l.savedpc = (*p).code;
                while narg < nfixparams {
                    let fresh1 = (*L).top.p;
                    (*L).top.p = ((*L).top.p).offset(1);
                    (*fresh1)
                        .val
                        .tt_ = (0 as libc::c_int
                        | (0 as libc::c_int) << 4 as libc::c_int) as lu_byte;
                    narg += 1;
                    narg;
                }
                return ci;
            }
            _ => {
                func = tryfuncTM(L, func);
            }
        }
    };
}
#[inline]
unsafe extern "C" fn ccall(
    mut L: *mut lua_State,
    mut func: StkId,
    mut nResults: libc::c_int,
    mut inc: l_uint32,
) {
    let mut ci: *mut CallInfo = 0 as *mut CallInfo;
    (*L)
        .nCcalls = ((*L).nCcalls as libc::c_uint).wrapping_add(inc) as l_uint32
        as l_uint32;
    if (((*L).nCcalls & 0xffff as libc::c_int as libc::c_uint
        >= 200 as libc::c_int as libc::c_uint) as libc::c_int != 0 as libc::c_int)
        as libc::c_int as libc::c_long != 0
    {
        if ((((*L).stack_last.p).offset_from((*L).top.p) as libc::c_long
            <= 0 as libc::c_int as libc::c_long) as libc::c_int != 0 as libc::c_int)
            as libc::c_int as libc::c_long != 0
        {
            let mut t__: ptrdiff_t = (func as *mut libc::c_char)
                .offset_from((*L).stack.p as *mut libc::c_char) as libc::c_long;
            luaD_growstack(L, 0 as libc::c_int, 1 as libc::c_int);
            func = ((*L).stack.p as *mut libc::c_char).offset(t__ as isize) as StkId;
        }
        luaE_checkcstack(L);
    }
    ci = luaD_precall(L, func, nResults);
    if !ci.is_null() {
        (*ci).callstatus = ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_ushort;
        luaV_execute(L, ci);
    }
    (*L)
        .nCcalls = ((*L).nCcalls as libc::c_uint).wrapping_sub(inc) as l_uint32
        as l_uint32;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaD_call(
    mut L: *mut lua_State,
    mut func: StkId,
    mut nResults: libc::c_int,
) {
    ccall(L, func, nResults, 1 as libc::c_int as l_uint32);
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaD_callnoyield(
    mut L: *mut lua_State,
    mut func: StkId,
    mut nResults: libc::c_int,
) {
    ccall(L, func, nResults, (0x10000 as libc::c_int | 1 as libc::c_int) as l_uint32);
}
unsafe extern "C" fn finishpcallk(
    mut L: *mut lua_State,
    mut ci: *mut CallInfo,
) -> libc::c_int {
    let mut status: libc::c_int = (*ci).callstatus as libc::c_int >> 10 as libc::c_int
        & 7 as libc::c_int;
    if ((status == 0 as libc::c_int) as libc::c_int != 0 as libc::c_int) as libc::c_int
        as libc::c_long != 0
    {
        status = 1 as libc::c_int;
    } else {
        let mut func: StkId = ((*L).stack.p as *mut libc::c_char)
            .offset((*ci).u2.funcidx as isize) as StkId;
        (*L)
            .allowhook = ((*ci).callstatus as libc::c_int
            & (1 as libc::c_int) << 0 as libc::c_int) as lu_byte;
        func = luaF_close(L, func, status, 1 as libc::c_int);
        luaD_seterrorobj(L, status, func);
        luaD_shrinkstack(L);
        (*ci)
            .callstatus = ((*ci).callstatus as libc::c_int
            & !((7 as libc::c_int) << 10 as libc::c_int)
            | (0 as libc::c_int) << 10 as libc::c_int) as libc::c_ushort;
    }
    (*ci)
        .callstatus = ((*ci).callstatus as libc::c_int
        & !((1 as libc::c_int) << 4 as libc::c_int)) as libc::c_ushort;
    (*L).errfunc = (*ci).u.c.old_errfunc;
    return status;
}
unsafe extern "C" fn finishCcall(mut L: *mut lua_State, mut ci: *mut CallInfo) {
    let mut n: libc::c_int = 0;
    if (*ci).callstatus as libc::c_int & (1 as libc::c_int) << 9 as libc::c_int != 0 {
        n = (*ci).u2.nres;
    } else {
        let mut status: libc::c_int = 1 as libc::c_int;
        if (*ci).callstatus as libc::c_int & (1 as libc::c_int) << 4 as libc::c_int != 0
        {
            status = finishpcallk(L, ci);
        }
        if -(1 as libc::c_int) <= -(1 as libc::c_int) && (*(*L).ci).top.p < (*L).top.p {
            (*(*L).ci).top.p = (*L).top.p;
        }
        n = (Some(((*ci).u.c.k).expect("non-null function pointer")))
            .expect("non-null function pointer")(L, status, (*ci).u.c.ctx);
    }
    luaD_poscall(L, ci, n);
}
unsafe extern "C" fn unroll(mut L: *mut lua_State, mut ud: *mut libc::c_void) {
    let mut ci: *mut CallInfo = 0 as *mut CallInfo;
    loop {
        ci = (*L).ci;
        if !(ci != &mut (*L).base_ci as *mut CallInfo) {
            break;
        }
        if (*ci).callstatus as libc::c_int & (1 as libc::c_int) << 1 as libc::c_int != 0
        {
            finishCcall(L, ci);
        } else {
            luaV_finishOp(L);
            luaV_execute(L, ci);
        }
    };
}
unsafe extern "C" fn findpcall(mut L: *mut lua_State) -> *mut CallInfo {
    let mut ci: *mut CallInfo = 0 as *mut CallInfo;
    ci = (*L).ci;
    while !ci.is_null() {
        if (*ci).callstatus as libc::c_int & (1 as libc::c_int) << 4 as libc::c_int != 0
        {
            return ci;
        }
        ci = (*ci).previous;
    }
    return 0 as *mut CallInfo;
}
unsafe extern "C" fn resume_error(
    mut L: *mut lua_State,
    mut msg: *const libc::c_char,
    mut narg: libc::c_int,
) -> libc::c_int {
    (*L).top.p = ((*L).top.p).offset(-(narg as isize));
    let mut io: *mut TValue = &mut (*(*L).top.p).val;
    let mut x_: *mut TString = luaS_new(L, msg);
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
    (*io)
        .tt_ = ((*x_).tt as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int)
        as lu_byte;
    (*L).top.p = ((*L).top.p).offset(1);
    (*L).top.p;
    return 2 as libc::c_int;
}
unsafe extern "C" fn resume(mut L: *mut lua_State, mut ud: *mut libc::c_void) {
    let mut n: libc::c_int = *(ud as *mut libc::c_int);
    let mut firstArg: StkId = ((*L).top.p).offset(-(n as isize));
    let mut ci: *mut CallInfo = (*L).ci;
    if (*L).status as libc::c_int == 0 as libc::c_int {
        ccall(
            L,
            firstArg.offset(-(1 as libc::c_int as isize)),
            -(1 as libc::c_int),
            0 as libc::c_int as l_uint32,
        );
    } else {
        (*L).status = 0 as libc::c_int as lu_byte;
        if (*ci).callstatus as libc::c_int & (1 as libc::c_int) << 1 as libc::c_int == 0
        {
            (*ci).u.l.savedpc = ((*ci).u.l.savedpc).offset(-1);
            (*ci).u.l.savedpc;
            (*L).top.p = firstArg;
            luaV_execute(L, ci);
        } else {
            if ((*ci).u.c.k).is_some() {
                n = (Some(((*ci).u.c.k).expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )(L, 1 as libc::c_int, (*ci).u.c.ctx);
            }
            luaD_poscall(L, ci, n);
        }
        unroll(L, 0 as *mut libc::c_void);
    };
}
unsafe extern "C" fn precover(
    mut L: *mut lua_State,
    mut status: libc::c_int,
) -> libc::c_int {
    let mut ci: *mut CallInfo = 0 as *mut CallInfo;
    while status > 1 as libc::c_int
        && {
            ci = findpcall(L);
            !ci.is_null()
        }
    {
        (*L).ci = ci;
        (*ci)
            .callstatus = ((*ci).callstatus as libc::c_int
            & !((7 as libc::c_int) << 10 as libc::c_int) | status << 10 as libc::c_int)
            as libc::c_ushort;
        status = luaD_rawrunprotected(
            L,
            Some(
                unroll as unsafe extern "C" fn(*mut lua_State, *mut libc::c_void) -> (),
            ),
            0 as *mut libc::c_void,
        );
    }
    return status;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_resume(
    mut L: *mut lua_State,
    mut from: *mut lua_State,
    mut nargs: libc::c_int,
    mut nresults: *mut libc::c_int,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    if (*L).status as libc::c_int == 0 as libc::c_int {
        if (*L).ci != &mut (*L).base_ci as *mut CallInfo {
            return resume_error(
                L,
                b"cannot resume non-suspended coroutine\0" as *const u8
                    as *const libc::c_char,
                nargs,
            )
        } else if ((*L).top.p)
            .offset_from(((*(*L).ci).func.p).offset(1 as libc::c_int as isize))
            as libc::c_long == nargs as libc::c_long
        {
            return resume_error(
                L,
                b"cannot resume dead coroutine\0" as *const u8 as *const libc::c_char,
                nargs,
            )
        }
    } else if (*L).status as libc::c_int != 1 as libc::c_int {
        return resume_error(
            L,
            b"cannot resume dead coroutine\0" as *const u8 as *const libc::c_char,
            nargs,
        )
    }
    (*L)
        .nCcalls = if !from.is_null() {
        (*from).nCcalls & 0xffff as libc::c_int as libc::c_uint
    } else {
        0 as libc::c_int as libc::c_uint
    };
    if (*L).nCcalls & 0xffff as libc::c_int as libc::c_uint
        >= 200 as libc::c_int as libc::c_uint
    {
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
        &mut nargs as *mut libc::c_int as *mut libc::c_void,
    );
    status = precover(L, status);
    if !((!(status > 1 as libc::c_int) as libc::c_int != 0 as libc::c_int) as libc::c_int
        as libc::c_long != 0)
    {
        (*L).status = status as lu_byte;
        luaD_seterrorobj(L, status, (*L).top.p);
        (*(*L).ci).top.p = (*L).top.p;
    }
    *nresults = if status == 1 as libc::c_int {
        (*(*L).ci).u2.nyield
    } else {
        ((*L).top.p).offset_from(((*(*L).ci).func.p).offset(1 as libc::c_int as isize))
            as libc::c_long as libc::c_int
    };
    return status;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_isyieldable(mut L: *mut lua_State) -> libc::c_int {
    return ((*L).nCcalls & 0xffff0000 as libc::c_uint
        == 0 as libc::c_int as libc::c_uint) as libc::c_int;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_yieldk(
    mut L: *mut lua_State,
    mut nresults: libc::c_int,
    mut ctx: lua_KContext,
    mut k: lua_KFunction,
) -> libc::c_int {
    let mut ci: *mut CallInfo = 0 as *mut CallInfo;
    ci = (*L).ci;
    if (!((*L).nCcalls & 0xffff0000 as libc::c_uint == 0 as libc::c_int as libc::c_uint)
        as libc::c_int != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
    {
        if L != (*(*L).l_G).mainthread {
            luaG_runerror(
                L,
                b"attempt to yield across a C-call boundary\0" as *const u8
                    as *const libc::c_char,
            );
        } else {
            luaG_runerror(
                L,
                b"attempt to yield from outside a coroutine\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    (*L).status = 1 as libc::c_int as lu_byte;
    (*ci).u2.nyield = nresults;
    if (*ci).callstatus as libc::c_int & (1 as libc::c_int) << 1 as libc::c_int == 0
    {} else {
        (*ci).u.c.k = k;
        if ((*ci).u.c.k).is_some() {
            (*ci).u.c.ctx = ctx;
        }
        luaD_throw(L, 1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn closepaux(mut L: *mut lua_State, mut ud: *mut libc::c_void) {
    let mut pcl: *mut CloseP = ud as *mut CloseP;
    luaF_close(L, (*pcl).level, (*pcl).status, 0 as libc::c_int);
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaD_closeprotected(
    mut L: *mut lua_State,
    mut level: ptrdiff_t,
    mut status: libc::c_int,
) -> libc::c_int {
    let mut old_ci: *mut CallInfo = (*L).ci;
    let mut old_allowhooks: lu_byte = (*L).allowhook;
    loop {
        let mut pcl: CloseP = CloseP {
            level: 0 as *mut StackValue,
            status: 0,
        };
        pcl.level = ((*L).stack.p as *mut libc::c_char).offset(level as isize) as StkId;
        pcl.status = status;
        status = luaD_rawrunprotected(
            L,
            Some(
                closepaux
                    as unsafe extern "C" fn(*mut lua_State, *mut libc::c_void) -> (),
            ),
            &mut pcl as *mut CloseP as *mut libc::c_void,
        );
        if ((status == 0 as libc::c_int) as libc::c_int != 0 as libc::c_int)
            as libc::c_int as libc::c_long != 0
        {
            return pcl.status
        } else {
            (*L).ci = old_ci;
            (*L).allowhook = old_allowhooks;
        }
    };
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaD_pcall(
    mut L: *mut lua_State,
    mut func: Pfunc,
    mut u: *mut libc::c_void,
    mut old_top: ptrdiff_t,
    mut ef: ptrdiff_t,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut old_ci: *mut CallInfo = (*L).ci;
    let mut old_allowhooks: lu_byte = (*L).allowhook;
    let mut old_errfunc: ptrdiff_t = (*L).errfunc;
    (*L).errfunc = ef;
    status = luaD_rawrunprotected(L, func, u);
    if ((status != 0 as libc::c_int) as libc::c_int != 0 as libc::c_int) as libc::c_int
        as libc::c_long != 0
    {
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
    if !mode.is_null()
        && (strchr(mode, *x.offset(0 as libc::c_int as isize) as libc::c_int)).is_null()
    {
        luaO_pushfstring(
            L,
            b"attempt to load a %s chunk (mode is '%s')\0" as *const u8
                as *const libc::c_char,
            x,
            mode,
        );
        luaD_throw(L, 3 as libc::c_int);
    }
}
unsafe extern "C" fn f_parser(mut L: *mut lua_State, mut ud: *mut libc::c_void) {
    let mut cl: *mut LClosure = 0 as *mut LClosure;
    let mut p: *mut SParser = ud as *mut SParser;
    let fresh2 = (*(*p).z).n;
    (*(*p).z).n = ((*(*p).z).n).wrapping_sub(1);
    let mut c: libc::c_int = if fresh2 > 0 as libc::c_int as libc::c_ulong {
        let fresh3 = (*(*p).z).p;
        (*(*p).z).p = ((*(*p).z).p).offset(1);
        *fresh3 as libc::c_uchar as libc::c_int
    } else {
        luaZ_fill((*p).z)
    };
    if c
        == (*::core::mem::transmute::<
            &[u8; 5],
            &[libc::c_char; 5],
        >(b"\x1BLua\0"))[0 as libc::c_int as usize] as libc::c_int
    {
        checkmode(L, (*p).mode, b"binary\0" as *const u8 as *const libc::c_char);
        cl = luaU_undump(L, (*p).z, (*p).name);
    } else {
        checkmode(L, (*p).mode, b"text\0" as *const u8 as *const libc::c_char);
        cl = luaY_parser(L, (*p).z, &mut (*p).buff, &mut (*p).dyd, (*p).name, c);
    }
    luaF_initupvals(L, cl);
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaD_protectedparser(
    mut L: *mut lua_State,
    mut z: *mut ZIO,
    mut name: *const libc::c_char,
    mut mode: *const libc::c_char,
) -> libc::c_int {
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
    let mut status: libc::c_int = 0;
    (*L)
        .nCcalls = ((*L).nCcalls as libc::c_uint)
        .wrapping_add(0x10000 as libc::c_int as libc::c_uint) as l_uint32 as l_uint32;
    p.z = z;
    p.name = name;
    p.mode = mode;
    p.dyd.actvar.arr = 0 as *mut Vardesc;
    p.dyd.actvar.size = 0 as libc::c_int;
    p.dyd.gt.arr = 0 as *mut Labeldesc;
    p.dyd.gt.size = 0 as libc::c_int;
    p.dyd.label.arr = 0 as *mut Labeldesc;
    p.dyd.label.size = 0 as libc::c_int;
    p.buff.buffer = 0 as *mut libc::c_char;
    p.buff.buffsize = 0 as libc::c_int as size_t;
    status = luaD_pcall(
        L,
        Some(f_parser as unsafe extern "C" fn(*mut lua_State, *mut libc::c_void) -> ()),
        &mut p as *mut SParser as *mut libc::c_void,
        ((*L).top.p as *mut libc::c_char).offset_from((*L).stack.p as *mut libc::c_char)
            as libc::c_long,
        (*L).errfunc,
    );
    p
        .buff
        .buffer = luaM_saferealloc_(
        L,
        p.buff.buffer as *mut libc::c_void,
        (p.buff.buffsize)
            .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
        (0 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    p.buff.buffsize = 0 as libc::c_int as size_t;
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
    (*L)
        .nCcalls = ((*L).nCcalls as libc::c_uint)
        .wrapping_sub(0x10000 as libc::c_int as libc::c_uint) as l_uint32 as l_uint32;
    return status;
}
