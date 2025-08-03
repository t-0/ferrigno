#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use crate::types::{Integer,Number};
unsafe extern "C" {
    pub type lua_longjmp;
    fn luaG_runerror(L: *mut lua_State, fmt: *const libc::c_char, _: ...) -> !;
    fn luaD_throw(L: *mut lua_State, errcode: i32) -> !;
    fn luaC_fullgc(L: *mut lua_State, isemergency: i32);
}
pub type ptrdiff_t = libc::c_long;
pub type size_t = libc::c_ulong;
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
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaM_growaux_(
    mut L: *mut lua_State,
    mut block: *mut libc::c_void,
    mut nelems: i32,
    mut psize: *mut i32,
    mut size_elems: i32,
    mut limit: i32,
    mut what: *const libc::c_char,
) -> *mut libc::c_void {
    let mut newblock: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut size: i32 = *psize;
    if nelems + 1i32 <= size {
        return block;
    }
    if size >= limit / 2i32 {
        if ((size >= limit) as i32 != 0i32) as i32
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
        size *= 2i32;
        if size < 4i32 {
            size = 4i32;
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
    mut size: *mut i32,
    mut final_n: i32,
    mut size_elem: i32,
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
        )((*g).ud, block, osize, 0i32 as size_t);
    (*g).GCdebt = ((*g).GCdebt as libc::c_ulong).wrapping_sub(osize) as l_mem as l_mem;
}
unsafe extern "C" fn tryagain(
    mut L: *mut lua_State,
    mut block: *mut libc::c_void,
    mut osize: size_t,
    mut nsize: size_t,
) -> *mut libc::c_void {
    let mut g: *mut global_State = (*L).l_G;
    if (*g).nilvalue.tt_ as i32 & 0xf as i32 == 0i32
        && (*g).gcstopem == 0
    {
        luaC_fullgc(L, 1i32);
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
    if ((newblock.is_null() && nsize > 0i32 as libc::c_ulong) as i32
        != 0i32) as i32 as libc::c_long != 0
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
    if ((newblock.is_null() && nsize > 0i32 as libc::c_ulong) as i32
        != 0i32) as i32 as libc::c_long != 0
    {
        luaD_throw(L, 4i32);
    }
    return newblock;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaM_malloc_(
    mut L: *mut lua_State,
    mut size: size_t,
    mut tag: i32,
) -> *mut libc::c_void {
    if size == 0i32 as libc::c_ulong {
        return 0 as *mut libc::c_void
    } else {
        let mut g: *mut global_State = (*L).l_G;
        let mut newblock: *mut libc::c_void = (Some(
            ((*g).frealloc).expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )((*g).ud, 0 as *mut libc::c_void, tag as size_t, size);
        if ((newblock == 0 as *mut libc::c_void) as i32 != 0i32)
            as i32 as libc::c_long != 0
        {
            newblock = tryagain(L, 0 as *mut libc::c_void, tag as size_t, size);
            if newblock.is_null() {
                luaD_throw(L, 4i32);
            }
        }
        (*g)
            .GCdebt = ((*g).GCdebt as libc::c_ulong).wrapping_add(size) as l_mem
            as l_mem;
        return newblock;
    };
}
