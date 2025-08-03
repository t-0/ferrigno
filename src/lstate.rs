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
    pub type lua_longjmp;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn luaT_init(L: *mut lua_State);
    fn luaM_free_(L: *mut lua_State, block: *mut libc::c_void, osize: size_t);
    fn luaM_malloc_(
        L: *mut lua_State,
        size: size_t,
        tag: i32,
    ) -> *mut libc::c_void;
    fn luaG_runerror(L: *mut lua_State, fmt: *const libc::c_char, _: ...) -> !;
    fn luaD_errerr(L: *mut lua_State) -> !;
    fn luaD_seterrorobj(L: *mut lua_State, errcode: i32, oldtop: StkId);
    fn luaD_closeprotected(
        L: *mut lua_State,
        level: ptrdiff_t,
        status: i32,
    ) -> i32;
    fn luaD_reallocstack(
        L: *mut lua_State,
        newsize: i32,
        raiseerror: i32,
    ) -> i32;
    fn luaD_rawrunprotected(
        L: *mut lua_State,
        f: Pfunc,
        ud: *mut libc::c_void,
    ) -> i32;
    fn luaF_closeupval(L: *mut lua_State, level: StkId);
    fn luaC_freeallobjects(L: *mut lua_State);
    fn luaC_step(L: *mut lua_State);
    fn luaC_newobjdt(
        L: *mut lua_State,
        tt: i32,
        sz: size_t,
        offset: size_t,
    ) -> *mut GCObject;
    fn luaX_init(L: *mut lua_State);
    fn luaS_hash(
        str: *const libc::c_char,
        l: size_t,
        seed: libc::c_uint,
    ) -> libc::c_uint;
    fn luaS_init(L: *mut lua_State);
    fn luaH_new(L: *mut lua_State) -> *mut Table;
    fn luaH_resize(
        L: *mut lua_State,
        t: *mut Table,
        nasize: libc::c_uint,
        nhsize: libc::c_uint,
    );
    fn time(__timer: *mut time_t) -> time_t;
}
pub type ptrdiff_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type __time_t = libc::c_long;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LG {
    pub l: LX,
    pub g: global_State,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LX {
    pub extra_: [u8; 8],
    pub l: lua_State,
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
    pub len: size_t,
    pub metatable: *mut Table,
    pub gclist: *mut GCObject,
    pub uv: [UValue; 1],
}
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
pub type Pfunc = Option::<unsafe extern "C" fn(*mut lua_State, *mut libc::c_void) -> ()>;
pub type time_t = __time_t;
unsafe extern "C" fn luai_makeseed(mut L: *mut lua_State) -> libc::c_uint {
    let mut buff: [libc::c_char; 24] = [0; 24];
    let mut h: libc::c_uint = time(0 as *mut time_t) as libc::c_uint;
    let mut p: i32 = 0i32;
    let mut t: size_t = L as size_t;
    memcpy(
        buff.as_mut_ptr().offset(p as isize) as *mut libc::c_void,
        &mut t as *mut size_t as *const libc::c_void,
        ::core::mem::size_of::<size_t>() as libc::c_ulong,
    );
    p = (p as libc::c_ulong)
        .wrapping_add(::core::mem::size_of::<size_t>() as libc::c_ulong) as i32
        as i32;
    let mut t_0: size_t = &mut h as *mut libc::c_uint as size_t;
    memcpy(
        buff.as_mut_ptr().offset(p as isize) as *mut libc::c_void,
        &mut t_0 as *mut size_t as *const libc::c_void,
        ::core::mem::size_of::<size_t>() as libc::c_ulong,
    );
    p = (p as libc::c_ulong)
        .wrapping_add(::core::mem::size_of::<size_t>() as libc::c_ulong) as i32
        as i32;
    let mut t_1: size_t = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(lua_Alloc, *mut libc::c_void) -> *mut lua_State>,
        size_t,
    >(
        Some(
            lua_newstate
                as unsafe extern "C" fn(lua_Alloc, *mut libc::c_void) -> *mut lua_State,
        ),
    );
    memcpy(
        buff.as_mut_ptr().offset(p as isize) as *mut libc::c_void,
        &mut t_1 as *mut size_t as *const libc::c_void,
        ::core::mem::size_of::<size_t>() as libc::c_ulong,
    );
    p = (p as libc::c_ulong)
        .wrapping_add(::core::mem::size_of::<size_t>() as libc::c_ulong) as i32
        as i32;
    return luaS_hash(buff.as_mut_ptr(), p as size_t, h);
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaE_setdebt(mut g: *mut global_State, mut debt: l_mem) {
    let mut tb: l_mem = ((*g).totalbytes + (*g).GCdebt) as lu_mem as l_mem;
    if debt < tb - (!(0i32 as lu_mem) >> 1i32) as l_mem {
        debt = tb - (!(0i32 as lu_mem) >> 1i32) as l_mem;
    }
    (*g).totalbytes = tb - debt;
    (*g).GCdebt = debt;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_setcstacklimit(
    mut _L: *mut lua_State,
    mut _limit: libc::c_uint,
) -> i32 {
    return 200i32;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaE_extendCI(mut L: *mut lua_State) -> *mut CallInfo {
    let mut ci: *mut CallInfo = 0 as *mut CallInfo;
    ci = luaM_malloc_(
        L,
        ::core::mem::size_of::<CallInfo>() as libc::c_ulong,
        0i32,
    ) as *mut CallInfo;
    (*(*L).ci).next = ci;
    (*ci).previous = (*L).ci;
    (*ci).next = 0 as *mut CallInfo;
    ::core::ptr::write_volatile(
        &mut (*ci).u.l.trap as *mut sig_atomic_t,
        0i32,
    );
    (*L).nci = ((*L).nci).wrapping_add(1);
    (*L).nci;
    return ci;
}
unsafe extern "C" fn freeCI(mut L: *mut lua_State) {
    let mut ci: *mut CallInfo = (*L).ci;
    let mut next: *mut CallInfo = (*ci).next;
    (*ci).next = 0 as *mut CallInfo;
    loop {
        ci = next;
        if ci.is_null() {
            break;
        }
        next = (*ci).next;
        luaM_free_(
            L,
            ci as *mut libc::c_void,
            ::core::mem::size_of::<CallInfo>() as libc::c_ulong,
        );
        (*L).nci = ((*L).nci).wrapping_sub(1);
        (*L).nci;
    };
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaE_shrinkCI(mut L: *mut lua_State) {
    let mut ci: *mut CallInfo = (*(*L).ci).next;
    let mut next: *mut CallInfo = 0 as *mut CallInfo;
    if ci.is_null() {
        return;
    }
    loop {
        next = (*ci).next;
        if next.is_null() {
            break;
        }
        let mut next2: *mut CallInfo = (*next).next;
        (*ci).next = next2;
        (*L).nci = ((*L).nci).wrapping_sub(1);
        (*L).nci;
        luaM_free_(
            L,
            next as *mut libc::c_void,
            ::core::mem::size_of::<CallInfo>() as libc::c_ulong,
        );
        if next2.is_null() {
            break;
        }
        (*next2).previous = ci;
        ci = next2;
    };
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaE_checkcstack(mut L: *mut lua_State) {
    if (*L).nCcalls & 0xffff as i32 as libc::c_uint
        == 200i32 as libc::c_uint
    {
        luaG_runerror(L, b"C stack overflow\0" as *const u8 as *const libc::c_char);
    } else if (*L).nCcalls & 0xffff as i32 as libc::c_uint
        >= (200i32 / 10i32 * 11i32) as libc::c_uint
    {
        luaD_errerr(L);
    }
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaE_incCstack(mut L: *mut lua_State) {
    (*L).nCcalls = ((*L).nCcalls).wrapping_add(1);
    (*L).nCcalls;
    if (((*L).nCcalls & 0xffff as i32 as libc::c_uint
        >= 200i32 as libc::c_uint) as i32 != 0i32)
        as i32 as libc::c_long != 0
    {
        luaE_checkcstack(L);
    }
}
unsafe extern "C" fn stack_init(mut L1: *mut lua_State, mut L: *mut lua_State) {
    let mut i: i32 = 0;
    let mut ci: *mut CallInfo = 0 as *mut CallInfo;
    (*L1)
        .stack
        .p = luaM_malloc_(
        L,
        ((2i32 * 20i32 + 5i32) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<StackValue>() as libc::c_ulong),
        0i32,
    ) as *mut StackValue;
    (*L1).tbclist.p = (*L1).stack.p;
    i = 0i32;
    while i < 2i32 * 20i32 + 5i32 {
        (*((*L1).stack.p).offset(i as isize))
            .val
            .tt_ = (0i32 | (0i32) << 4i32)
            as u8;
        i += 1;
    }
    (*L1).top.p = (*L1).stack.p;
    (*L1)
        .stack_last
        .p = ((*L1).stack.p).offset((2i32 * 20i32) as isize);
    ci = &mut (*L1).base_ci;
    (*ci).previous = 0 as *mut CallInfo;
    (*ci).next = (*ci).previous;
    (*ci).callstatus = ((1i32) << 1i32) as libc::c_ushort;
    (*ci).func.p = (*L1).top.p;
    (*ci).u.c.k = None;
    (*ci).nresults = 0i32 as libc::c_short;
    (*(*L1).top.p)
        .val
        .tt_ = (0i32 | (0i32) << 4i32) as u8;
    (*L1).top.p = ((*L1).top.p).offset(1);
    (*L1).top.p;
    (*ci).top.p = ((*L1).top.p).offset(20i32 as isize);
    (*L1).ci = ci;
}
unsafe extern "C" fn freestack(mut L: *mut lua_State) {
    if ((*L).stack.p).is_null() {
        return;
    }
    (*L).ci = &mut (*L).base_ci;
    freeCI(L);
    luaM_free_(
        L,
        (*L).stack.p as *mut libc::c_void,
        ((((*L).stack_last.p).offset_from((*L).stack.p) as libc::c_long as i32
            + 5i32) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<StackValue>() as libc::c_ulong),
    );
}
unsafe extern "C" fn init_registry(mut L: *mut lua_State, mut g: *mut global_State) {
    let mut registry: *mut Table = luaH_new(L);
    let mut io: *mut TValue = &mut (*g).l_registry;
    let mut x_: *mut Table = registry;
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
    (*io)
        .tt_ = (5i32 | (0i32) << 4i32
        | (1i32) << 6i32) as u8;
    luaH_resize(
        L,
        registry,
        2i32 as libc::c_uint,
        0i32 as libc::c_uint,
    );
    let mut io_0: *mut TValue = &mut *((*registry).array)
        .offset((1i32 - 1i32) as isize) as *mut TValue;
    let mut x__0: *mut lua_State = L;
    (*io_0).value_.gc = &mut (*(x__0 as *mut GCUnion)).gc;
    (*io_0)
        .tt_ = (8i32 | (0i32) << 4i32
        | (1i32) << 6i32) as u8;
    let mut io_1: *mut TValue = &mut *((*registry).array)
        .offset((2i32 - 1i32) as isize) as *mut TValue;
    let mut x__1: *mut Table = luaH_new(L);
    (*io_1).value_.gc = &mut (*(x__1 as *mut GCUnion)).gc;
    (*io_1)
        .tt_ = (5i32 | (0i32) << 4i32
        | (1i32) << 6i32) as u8;
}
unsafe extern "C" fn f_luaopen(mut L: *mut lua_State, mut _ud: *mut libc::c_void) {
    let mut g: *mut global_State = (*L).l_G;
    stack_init(L, L);
    init_registry(L, g);
    luaS_init(L);
    luaT_init(L);
    luaX_init(L);
    (*g).gcstp = 0i32 as u8;
    (*g)
        .nilvalue
        .tt_ = (0i32 | (0i32) << 4i32) as u8;
}
unsafe extern "C" fn preinit_thread(mut L: *mut lua_State, mut g: *mut global_State) {
    (*L).l_G = g;
    (*L).stack.p = 0 as StkId;
    (*L).ci = 0 as *mut CallInfo;
    (*L).nci = 0i32 as libc::c_ushort;
    (*L).twups = L;
    (*L).nCcalls = 0i32 as l_uint32;
    (*L).errorJmp = 0 as *mut lua_longjmp;
    ::core::ptr::write_volatile(&mut (*L).hook as *mut lua_Hook, None);
    ::core::ptr::write_volatile(
        &mut (*L).hookmask as *mut sig_atomic_t,
        0i32,
    );
    (*L).basehookcount = 0i32;
    (*L).allowhook = 1i32 as u8;
    (*L).hookcount = (*L).basehookcount;
    (*L).openupval = 0 as *mut UpVal;
    (*L).status = 0i32 as u8;
    (*L).errfunc = 0i32 as ptrdiff_t;
    (*L).oldpc = 0i32;
}
unsafe extern "C" fn close_state(mut L: *mut lua_State) {
    let mut g: *mut global_State = (*L).l_G;
    if !((*g).nilvalue.tt_ as i32 & 0xf as i32 == 0i32) {
        luaC_freeallobjects(L);
    } else {
        (*L).ci = &mut (*L).base_ci;
        (*L).errfunc = 0i32 as ptrdiff_t;
        luaD_closeprotected(L, 1i32 as ptrdiff_t, 0i32);
        (*L).top.p = ((*L).stack.p).offset(1i32 as isize);
        luaC_freeallobjects(L);
    }
    luaM_free_(
        L,
        (*(*L).l_G).strt.hash as *mut libc::c_void,
        ((*(*L).l_G).strt.size as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut TString>() as libc::c_ulong),
    );
    freestack(L);
    (Some(((*g).frealloc).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*g).ud,
        (L as *mut u8).offset(-(8 as libc::c_ulong as isize)) as *mut LX
            as *mut libc::c_void,
        ::core::mem::size_of::<LG>() as libc::c_ulong,
        0i32 as size_t,
    );
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_newthread(mut L: *mut lua_State) -> *mut lua_State {
    let mut g: *mut global_State = (*L).l_G;
    let mut o: *mut GCObject = 0 as *mut GCObject;
    let mut L1: *mut lua_State = 0 as *mut lua_State;
    if (*(*L).l_G).GCdebt > 0i32 as libc::c_long {
        luaC_step(L);
    }
    o = luaC_newobjdt(
        L,
        8i32,
        ::core::mem::size_of::<LX>() as libc::c_ulong,
        8 as libc::c_ulong,
    );
    L1 = &mut (*(o as *mut GCUnion)).th;
    let mut io: *mut TValue = &mut (*(*L).top.p).val;
    let mut x_: *mut lua_State = L1;
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
    (*io)
        .tt_ = (8i32 | (0i32) << 4i32
        | (1i32) << 6i32) as u8;
    (*L).top.p = ((*L).top.p).offset(1);
    (*L).top.p;
    preinit_thread(L1, g);
    ::core::ptr::write_volatile(&mut (*L1).hookmask as *mut sig_atomic_t, (*L).hookmask);
    (*L1).basehookcount = (*L).basehookcount;
    ::core::ptr::write_volatile(&mut (*L1).hook as *mut lua_Hook, (*L).hook);
    (*L1).hookcount = (*L1).basehookcount;
    memcpy(
        (L1 as *mut libc::c_char)
            .offset(
                -(::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as isize),
            ) as *mut libc::c_void,
        ((*g).mainthread as *mut libc::c_char)
            .offset(
                -(::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as isize),
            ) as *mut libc::c_void,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
    );
    stack_init(L1, L);
    return L1;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaE_freethread(mut L: *mut lua_State, mut L1: *mut lua_State) {
    let mut l: *mut LX = (L1 as *mut u8).offset(-(8 as libc::c_ulong as isize))
        as *mut LX;
    luaF_closeupval(L1, (*L1).stack.p);
    freestack(L1);
    luaM_free_(L, l as *mut libc::c_void, ::core::mem::size_of::<LX>() as libc::c_ulong);
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaE_resetthread(
    mut L: *mut lua_State,
    mut status: i32,
) -> i32 {
    (*L).ci = &mut (*L).base_ci;
    let mut ci: *mut CallInfo = (*L).ci;
    (*(*L).stack.p)
        .val
        .tt_ = (0i32 | (0i32) << 4i32) as u8;
    (*ci).func.p = (*L).stack.p;
    (*ci).callstatus = ((1i32) << 1i32) as libc::c_ushort;
    if status == 1i32 {
        status = 0i32;
    }
    (*L).status = 0i32 as u8;
    (*L).errfunc = 0i32 as ptrdiff_t;
    status = luaD_closeprotected(L, 1i32 as ptrdiff_t, status);
    if status != 0i32 {
        luaD_seterrorobj(L, status, ((*L).stack.p).offset(1i32 as isize));
    } else {
        (*L).top.p = ((*L).stack.p).offset(1i32 as isize);
    }
    (*ci).top.p = ((*L).top.p).offset(20i32 as isize);
    luaD_reallocstack(
        L,
        ((*ci).top.p).offset_from((*L).stack.p) as libc::c_long as i32,
        0i32,
    );
    return status;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_closethread(
    mut L: *mut lua_State,
    mut from: *mut lua_State,
) -> i32 {
    let mut status: i32 = 0;
    (*L)
        .nCcalls = if !from.is_null() {
        (*from).nCcalls & 0xffff as i32 as libc::c_uint
    } else {
        0i32 as libc::c_uint
    };
    status = luaE_resetthread(L, (*L).status as i32);
    return status;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_resetthread(mut L: *mut lua_State) -> i32 {
    return lua_closethread(L, 0 as *mut lua_State);
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_newstate(
    mut f: lua_Alloc,
    mut ud: *mut libc::c_void,
) -> *mut lua_State {
    let mut i: i32 = 0;
    let mut L: *mut lua_State = 0 as *mut lua_State;
    let mut g: *mut global_State = 0 as *mut global_State;
    let mut l: *mut LG = (Some(f.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        ud,
        0 as *mut libc::c_void,
        8i32 as size_t,
        ::core::mem::size_of::<LG>() as libc::c_ulong,
    ) as *mut LG;
    if l.is_null() {
        return 0 as *mut lua_State;
    }
    L = &mut (*l).l.l;
    g = &mut (*l).g;
    (*L).tt = (8i32 | (0i32) << 4i32) as u8;
    (*g).currentwhite = ((1i32) << 3i32) as u8;
    (*L)
        .marked = ((*g).currentwhite as i32
        & ((1i32) << 3i32
            | (1i32) << 4i32)) as u8;
    preinit_thread(L, g);
    (*g).allgc = &mut (*(L as *mut GCUnion)).gc;
    (*L).next = 0 as *mut GCObject;
    (*L)
        .nCcalls = ((*L).nCcalls as libc::c_uint)
        .wrapping_add(0x10000 as i32 as libc::c_uint) as l_uint32 as l_uint32;
    (*g).frealloc = f;
    (*g).ud = ud;
    (*g).warnf = None;
    (*g).ud_warn = 0 as *mut libc::c_void;
    (*g).mainthread = L;
    (*g).seed = luai_makeseed(L);
    (*g).gcstp = 2i32 as u8;
    (*g).strt.nuse = 0i32;
    (*g).strt.size = (*g).strt.nuse;
    (*g).strt.hash = 0 as *mut *mut TString;
    (*g)
        .l_registry
        .tt_ = (0i32 | (0i32) << 4i32) as u8;
    (*g).panic = None;
    (*g).gcstate = 8i32 as u8;
    (*g).gckind = 0i32 as u8;
    (*g).gcstopem = 0i32 as u8;
    (*g).gcemergency = 0i32 as u8;
    (*g).fixedgc = 0 as *mut GCObject;
    (*g).tobefnz = (*g).fixedgc;
    (*g).finobj = (*g).tobefnz;
    (*g).reallyold = 0 as *mut GCObject;
    (*g).old1 = (*g).reallyold;
    (*g).survival = (*g).old1;
    (*g).firstold1 = (*g).survival;
    (*g).finobjrold = 0 as *mut GCObject;
    (*g).finobjold1 = (*g).finobjrold;
    (*g).finobjsur = (*g).finobjold1;
    (*g).sweepgc = 0 as *mut *mut GCObject;
    (*g).grayagain = 0 as *mut GCObject;
    (*g).gray = (*g).grayagain;
    (*g).allweak = 0 as *mut GCObject;
    (*g).ephemeron = (*g).allweak;
    (*g).weak = (*g).ephemeron;
    (*g).twups = 0 as *mut lua_State;
    (*g).totalbytes = ::core::mem::size_of::<LG>() as libc::c_ulong as l_mem;
    (*g).GCdebt = 0i32 as l_mem;
    (*g).lastatomic = 0i32 as lu_mem;
    let mut io: *mut TValue = &mut (*g).nilvalue;
    (*io).value_.i = 0i32 as Integer;
    (*io).tt_ = (3i32 | (0i32) << 4i32) as u8;
    (*g).gcpause = (200i32 / 4i32) as u8;
    (*g).gcstepmul = (100i32 / 4i32) as u8;
    (*g).gcstepsize = 13i32 as u8;
    (*g).genmajormul = (100i32 / 4i32) as u8;
    (*g).genminormul = 20i32 as u8;
    i = 0i32;
    while i < 9i32 {
        (*g).mt[i as usize] = 0 as *mut Table;
        i += 1;
    }
    if luaD_rawrunprotected(
        L,
        Some(f_luaopen as unsafe extern "C" fn(*mut lua_State, *mut libc::c_void) -> ()),
        0 as *mut libc::c_void,
    ) != 0i32
    {
        close_state(L);
        L = 0 as *mut lua_State;
    }
    return L;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_close(mut L: *mut lua_State) {
    L = (*(*L).l_G).mainthread;
    close_state(L);
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaE_warning(
    mut L: *mut lua_State,
    mut msg: *const libc::c_char,
    mut tocont: i32,
) {
    let mut wf: lua_WarnFunction = (*(*L).l_G).warnf;
    if wf.is_some() {
        wf.expect("non-null function pointer")((*(*L).l_G).ud_warn, msg, tocont);
    }
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaE_warnerror(
    mut L: *mut lua_State,
    mut where_0: *const libc::c_char,
) {
    let mut errobj: *mut TValue = &mut (*((*L).top.p)
        .offset(-(1i32 as isize)))
        .val;
    let mut msg: *const libc::c_char = if (*errobj).tt_ as i32
        & 0xf as i32 == 4i32
    {
        ((*((*errobj).value_.gc as *mut GCUnion)).ts.contents).as_mut_ptr()
            as *const libc::c_char
    } else {
        b"error object is not a string\0" as *const u8 as *const libc::c_char
    };
    luaE_warning(
        L,
        b"error in \0" as *const u8 as *const libc::c_char,
        1i32,
    );
    luaE_warning(L, where_0, 1i32);
    luaE_warning(L, b" (\0" as *const u8 as *const libc::c_char, 1i32);
    luaE_warning(L, msg, 1i32);
    luaE_warning(L, b")\0" as *const u8 as *const libc::c_char, 0i32);
}
