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
    fn luaG_runerror(L: *mut lua_State, fmt: *const libc::c_char, _: ...) -> !;
    fn luaD_throw(L: *mut lua_State, errcode: libc::c_int) -> !;
    fn luaC_fullgc(L: *mut lua_State, isemergency: libc::c_int);
}
pub type ptrdiff_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type __sig_atomic_t = libc::c_int;
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
    pub f: CFunction,
    pub i: Integer,
    pub n: Number,
    pub ub: u8,
}
pub type Number = f64;
pub type Integer = i64;
pub type CFunction = Option::<unsafe extern "C" fn(*mut lua_State) -> libc::c_int>;
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
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaM_growaux_(
    mut L: *mut lua_State,
    mut block: *mut libc::c_void,
    mut nelems: libc::c_int,
    mut psize: *mut libc::c_int,
    mut size_elems: libc::c_int,
    mut limit: libc::c_int,
    mut what: *const libc::c_char,
) -> *mut libc::c_void {
    let mut newblock: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut size: libc::c_int = *psize;
    if nelems + 1 as libc::c_int <= size {
        return block;
    }
    if size >= limit / 2 as libc::c_int {
        if ((size >= limit) as libc::c_int != 0 as libc::c_int) as libc::c_int
            as libc::c_long != 0
        {
            luaG_runerror(
                L,
                b"too many %s (limit is %d)\0" as *const u8 as *const libc::c_char,
                what,
                limit,
            );
        }
        size = limit;
    } else {
        size *= 2 as libc::c_int;
        if size < 4 as libc::c_int {
            size = 4 as libc::c_int;
        }
    }
    newblock = luaM_saferealloc_(
        L,
        block,
        (*psize as size_t).wrapping_mul(size_elems as libc::c_ulong),
        (size as size_t).wrapping_mul(size_elems as libc::c_ulong),
    );
    *psize = size;
    return newblock;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaM_shrinkvector_(
    mut L: *mut lua_State,
    mut block: *mut libc::c_void,
    mut size: *mut libc::c_int,
    mut final_n: libc::c_int,
    mut size_elem: libc::c_int,
) -> *mut libc::c_void {
    let mut newblock: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut oldsize: size_t = (*size * size_elem) as size_t;
    let mut newsize: size_t = (final_n * size_elem) as size_t;
    newblock = luaM_saferealloc_(L, block, oldsize, newsize);
    *size = final_n;
    return newblock;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaM_toobig(mut L: *mut lua_State) -> ! {
    luaG_runerror(
        L,
        b"memory allocation error: block too big\0" as *const u8 as *const libc::c_char,
    );
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaM_free_(
    mut L: *mut lua_State,
    mut block: *mut libc::c_void,
    mut osize: size_t,
) {
    let mut g: *mut global_State = (*L).l_G;
    (Some(((*g).frealloc).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )((*g).ud, block, osize, 0 as libc::c_int as size_t);
    (*g).GCdebt = ((*g).GCdebt as libc::c_ulong).wrapping_sub(osize) as l_mem as l_mem;
}
unsafe extern "C" fn tryagain(
    mut L: *mut lua_State,
    mut block: *mut libc::c_void,
    mut osize: size_t,
    mut nsize: size_t,
) -> *mut libc::c_void {
    let mut g: *mut global_State = (*L).l_G;
    if (*g).nilvalue.tt_ as libc::c_int & 0xf as libc::c_int == 0 as libc::c_int
        && (*g).gcstopem == 0
    {
        luaC_fullgc(L, 1 as libc::c_int);
        return (Some(((*g).frealloc).expect("non-null function pointer")))
            .expect("non-null function pointer")((*g).ud, block, osize, nsize);
    } else {
        return 0 as *mut libc::c_void
    };
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaM_realloc_(
    mut L: *mut lua_State,
    mut block: *mut libc::c_void,
    mut osize: size_t,
    mut nsize: size_t,
) -> *mut libc::c_void {
    let mut newblock: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut g: *mut global_State = (*L).l_G;
    newblock = (Some(((*g).frealloc).expect("non-null function pointer")))
        .expect("non-null function pointer")((*g).ud, block, osize, nsize);
    if ((newblock.is_null() && nsize > 0 as libc::c_int as libc::c_ulong) as libc::c_int
        != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
    {
        newblock = tryagain(L, block, osize, nsize);
        if newblock.is_null() {
            return 0 as *mut libc::c_void;
        }
    }
    (*g)
        .GCdebt = ((*g).GCdebt as libc::c_ulong).wrapping_add(nsize).wrapping_sub(osize)
        as l_mem;
    return newblock;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaM_saferealloc_(
    mut L: *mut lua_State,
    mut block: *mut libc::c_void,
    mut osize: size_t,
    mut nsize: size_t,
) -> *mut libc::c_void {
    let mut newblock: *mut libc::c_void = luaM_realloc_(L, block, osize, nsize);
    if ((newblock.is_null() && nsize > 0 as libc::c_int as libc::c_ulong) as libc::c_int
        != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
    {
        luaD_throw(L, 4 as libc::c_int);
    }
    return newblock;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaM_malloc_(
    mut L: *mut lua_State,
    mut size: size_t,
    mut tag: libc::c_int,
) -> *mut libc::c_void {
    if size == 0 as libc::c_int as libc::c_ulong {
        return 0 as *mut libc::c_void
    } else {
        let mut g: *mut global_State = (*L).l_G;
        let mut newblock: *mut libc::c_void = (Some(
            ((*g).frealloc).expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )((*g).ud, 0 as *mut libc::c_void, tag as size_t, size);
        if ((newblock == 0 as *mut libc::c_void) as libc::c_int != 0 as libc::c_int)
            as libc::c_int as libc::c_long != 0
        {
            newblock = tryagain(L, 0 as *mut libc::c_void, tag as size_t, size);
            if newblock.is_null() {
                luaD_throw(L, 4 as libc::c_int);
            }
        }
        (*g)
            .GCdebt = ((*g).GCdebt as libc::c_ulong).wrapping_add(size) as l_mem
            as l_mem;
        return newblock;
    };
}
