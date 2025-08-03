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
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn luaM_toobig(L: *mut lua_State) -> !;
    fn luaM_realloc_(
        L: *mut lua_State,
        block: *mut libc::c_void,
        oldsize: size_t,
        size: size_t,
    ) -> *mut libc::c_void;
    fn luaM_malloc_(
        L: *mut lua_State,
        size: size_t,
        tag: libc::c_int,
    ) -> *mut libc::c_void;
    fn luaD_throw(L: *mut lua_State, errcode: libc::c_int) -> !;
    fn luaC_fix(L: *mut lua_State, o: *mut GCObject);
    fn luaC_fullgc(L: *mut lua_State, isemergency: libc::c_int);
    fn luaC_newobj(L: *mut lua_State, tt: libc::c_int, sz: size_t) -> *mut GCObject;
}
pub type size_t = libc::c_ulong;
pub type ptrdiff_t = libc::c_long;
pub type __sig_atomic_t = libc::c_int;
pub type intptr_t = libc::c_long;
pub type uintptr_t = libc::c_ulong;
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
    pub f: lua_CFunction,
    pub i: lua_Integer,
    pub n: lua_Number,
    pub ub: u8,
}
pub type lua_Number = f64;
pub type lua_Integer = i64;
pub type lua_CFunction = Option::<unsafe extern "C" fn(*mut lua_State) -> libc::c_int>;
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
pub type ls_byte = libc::c_schar;
#[derive(Copy, Clone)]
#[repr(C)]
pub union UValue {
    pub uv: TValue,
    pub n: lua_Number,
    pub u: f64,
    pub s: *mut libc::c_void,
    pub i: lua_Integer,
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
    pub startpc: libc::c_int,
    pub endpc: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AbsLineInfo {
    pub pc: libc::c_int,
    pub line: libc::c_int,
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
pub struct CClosure {
    pub next: *mut GCObject,
    pub tt: u8,
    pub marked: u8,
    pub nupvalues: u8,
    pub gclist: *mut GCObject,
    pub f: lua_CFunction,
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
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaS_eqlngstr(
    mut a: *mut TString,
    mut b: *mut TString,
) -> libc::c_int {
    let mut len: size_t = (*a).u.lnglen;
    return (a == b
        || len == (*b).u.lnglen
            && memcmp(
                ((*a).contents).as_mut_ptr() as *const libc::c_void,
                ((*b).contents).as_mut_ptr() as *const libc::c_void,
                len,
            ) == 0 as libc::c_int) as libc::c_int;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaS_hash(
    mut str: *const libc::c_char,
    mut l: size_t,
    mut seed: libc::c_uint,
) -> libc::c_uint {
    let mut h: libc::c_uint = seed ^ l as libc::c_uint;
    while l > 0 as libc::c_int as libc::c_ulong {
        h
            ^= (h << 5 as libc::c_int)
                .wrapping_add(h >> 2 as libc::c_int)
                .wrapping_add(
                    *str
                        .offset(
                            l.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        ) as u8 as libc::c_uint,
                );
        l = l.wrapping_sub(1);
        l;
    }
    return h;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaS_hashlongstr(mut ts: *mut TString) -> libc::c_uint {
    if (*ts).extra as libc::c_int == 0 as libc::c_int {
        let mut len: size_t = (*ts).u.lnglen;
        (*ts).hash = luaS_hash(((*ts).contents).as_mut_ptr(), len, (*ts).hash);
        (*ts).extra = 1 as libc::c_int as u8;
    }
    return (*ts).hash;
}
unsafe extern "C" fn tablerehash(
    mut vect: *mut *mut TString,
    mut osize: libc::c_int,
    mut nsize: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    i = osize;
    while i < nsize {
        let ref mut fresh0 = *vect.offset(i as isize);
        *fresh0 = 0 as *mut TString;
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < osize {
        let mut p: *mut TString = *vect.offset(i as isize);
        let ref mut fresh1 = *vect.offset(i as isize);
        *fresh1 = 0 as *mut TString;
        while !p.is_null() {
            let mut hnext: *mut TString = (*p).u.hnext;
            let mut h: libc::c_uint = ((*p).hash
                & (nsize - 1 as libc::c_int) as libc::c_uint) as libc::c_int
                as libc::c_uint;
            (*p).u.hnext = *vect.offset(h as isize);
            let ref mut fresh2 = *vect.offset(h as isize);
            *fresh2 = p;
            p = hnext;
        }
        i += 1;
        i;
    }
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaS_resize(mut L: *mut lua_State, mut nsize: libc::c_int) {
    let mut tb: *mut stringtable = &mut (*(*L).l_G).strt;
    let mut osize: libc::c_int = (*tb).size;
    let mut newvect: *mut *mut TString = 0 as *mut *mut TString;
    if nsize < osize {
        tablerehash((*tb).hash, osize, nsize);
    }
    newvect = luaM_realloc_(
        L,
        (*tb).hash as *mut libc::c_void,
        (osize as size_t)
            .wrapping_mul(::core::mem::size_of::<*mut TString>() as libc::c_ulong),
        (nsize as size_t)
            .wrapping_mul(::core::mem::size_of::<*mut TString>() as libc::c_ulong),
    ) as *mut *mut TString;
    if ((newvect == 0 as *mut libc::c_void as *mut *mut TString) as libc::c_int
        != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
    {
        if nsize < osize {
            tablerehash((*tb).hash, nsize, osize);
        }
    } else {
        (*tb).hash = newvect;
        (*tb).size = nsize;
        if nsize > osize {
            tablerehash(newvect, osize, nsize);
        }
    };
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaS_clearcache(mut g: *mut global_State) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 53 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 2 as libc::c_int {
            if (*(*g).strcache[i as usize][j as usize]).marked as libc::c_int
                & ((1 as libc::c_int) << 3 as libc::c_int
                    | (1 as libc::c_int) << 4 as libc::c_int) != 0
            {
                (*g).strcache[i as usize][j as usize] = (*g).memerrmsg;
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaS_init(mut L: *mut lua_State) {
    let mut g: *mut global_State = (*L).l_G;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut tb: *mut stringtable = &mut (*(*L).l_G).strt;
    (*tb)
        .hash = luaM_malloc_(
        L,
        (128 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut TString>() as libc::c_ulong),
        0 as libc::c_int,
    ) as *mut *mut TString;
    tablerehash((*tb).hash, 0 as libc::c_int, 128 as libc::c_int);
    (*tb).size = 128 as libc::c_int;
    (*g)
        .memerrmsg = luaS_newlstr(
        L,
        b"not enough memory\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    luaC_fix(L, &mut (*((*g).memerrmsg as *mut GCUnion)).gc);
    i = 0 as libc::c_int;
    while i < 53 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 2 as libc::c_int {
            (*g).strcache[i as usize][j as usize] = (*g).memerrmsg;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn createstrobj(
    mut L: *mut lua_State,
    mut l: size_t,
    mut tag: libc::c_int,
    mut h: libc::c_uint,
) -> *mut TString {
    let mut ts: *mut TString = 0 as *mut TString;
    let mut o: *mut GCObject = 0 as *mut GCObject;
    let mut totalsize: size_t = 0;
    totalsize = (24 as libc::c_ulong)
        .wrapping_add(
            l
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
        );
    o = luaC_newobj(L, tag, totalsize);
    ts = &mut (*(o as *mut GCUnion)).ts;
    (*ts).hash = h;
    (*ts).extra = 0 as libc::c_int as u8;
    *((*ts).contents).as_mut_ptr().offset(l as isize) = '\0' as i32 as libc::c_char;
    return ts;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaS_createlngstrobj(
    mut L: *mut lua_State,
    mut l: size_t,
) -> *mut TString {
    let mut ts: *mut TString = createstrobj(
        L,
        l,
        4 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int,
        (*(*L).l_G).seed,
    );
    (*ts).u.lnglen = l;
    (*ts).shrlen = 0xff as libc::c_int as u8;
    return ts;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaS_remove(mut L: *mut lua_State, mut ts: *mut TString) {
    let mut tb: *mut stringtable = &mut (*(*L).l_G).strt;
    let mut p: *mut *mut TString = &mut *((*tb).hash)
        .offset(
            ((*ts).hash & ((*tb).size - 1 as libc::c_int) as libc::c_uint) as libc::c_int
                as isize,
        ) as *mut *mut TString;
    while *p != ts {
        p = &mut (**p).u.hnext;
    }
    *p = (**p).u.hnext;
    (*tb).nuse -= 1;
    (*tb).nuse;
}
unsafe extern "C" fn growstrtab(mut L: *mut lua_State, mut tb: *mut stringtable) {
    if (((*tb).nuse == 2147483647 as libc::c_int) as libc::c_int != 0 as libc::c_int)
        as libc::c_int as libc::c_long != 0
    {
        luaC_fullgc(L, 1 as libc::c_int);
        if (*tb).nuse == 2147483647 as libc::c_int {
            luaD_throw(L, 4 as libc::c_int);
        }
    }
    if (*tb).size
        <= (if 2147483647 as libc::c_int as size_t
            <= (!(0 as libc::c_int as size_t))
                .wrapping_div(::core::mem::size_of::<*mut TString>() as libc::c_ulong)
        {
            2147483647 as libc::c_int as libc::c_uint
        } else {
            (!(0 as libc::c_int as size_t))
                .wrapping_div(::core::mem::size_of::<*mut TString>() as libc::c_ulong)
                as libc::c_uint
        }) as libc::c_int / 2 as libc::c_int
    {
        luaS_resize(L, (*tb).size * 2 as libc::c_int);
    }
}
unsafe extern "C" fn internshrstr(
    mut L: *mut lua_State,
    mut str: *const libc::c_char,
    mut l: size_t,
) -> *mut TString {
    let mut ts: *mut TString = 0 as *mut TString;
    let mut g: *mut global_State = (*L).l_G;
    let mut tb: *mut stringtable = &mut (*g).strt;
    let mut h: libc::c_uint = luaS_hash(str, l, (*g).seed);
    let mut list: *mut *mut TString = &mut *((*tb).hash)
        .offset(
            (h & ((*tb).size - 1 as libc::c_int) as libc::c_uint) as libc::c_int as isize,
        ) as *mut *mut TString;
    ts = *list;
    while !ts.is_null() {
        if l == (*ts).shrlen as libc::c_ulong
            && memcmp(
                str as *const libc::c_void,
                ((*ts).contents).as_mut_ptr() as *const libc::c_void,
                l.wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
            ) == 0 as libc::c_int
        {
            if (*ts).marked as libc::c_int
                & ((*g).currentwhite as libc::c_int
                    ^ ((1 as libc::c_int) << 3 as libc::c_int
                        | (1 as libc::c_int) << 4 as libc::c_int)) != 0
            {
                (*ts)
                    .marked = ((*ts).marked as libc::c_int
                    ^ ((1 as libc::c_int) << 3 as libc::c_int
                        | (1 as libc::c_int) << 4 as libc::c_int)) as u8;
            }
            return ts;
        }
        ts = (*ts).u.hnext;
    }
    if (*tb).nuse >= (*tb).size {
        growstrtab(L, tb);
        list = &mut *((*tb).hash)
            .offset(
                (h & ((*tb).size - 1 as libc::c_int) as libc::c_uint) as libc::c_int
                    as isize,
            ) as *mut *mut TString;
    }
    ts = createstrobj(
        L,
        l,
        4 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int,
        h,
    );
    (*ts).shrlen = l as u8;
    memcpy(
        ((*ts).contents).as_mut_ptr() as *mut libc::c_void,
        str as *const libc::c_void,
        l.wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
    );
    (*ts).u.hnext = *list;
    *list = ts;
    (*tb).nuse += 1;
    (*tb).nuse;
    return ts;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaS_newlstr(
    mut L: *mut lua_State,
    mut str: *const libc::c_char,
    mut l: size_t,
) -> *mut TString {
    if l <= 40 as libc::c_int as libc::c_ulong {
        return internshrstr(L, str, l)
    } else {
        let mut ts: *mut TString = 0 as *mut TString;
        if ((l.wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
            >= (if (::core::mem::size_of::<size_t>() as libc::c_ulong)
                < ::core::mem::size_of::<lua_Integer>() as libc::c_ulong
            {
                !(0 as libc::c_int as size_t)
            } else {
                9223372036854775807 as i64 as size_t
            })
                .wrapping_sub(::core::mem::size_of::<TString>() as libc::c_ulong))
            as libc::c_int != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
        {
            luaM_toobig(L);
        }
        ts = luaS_createlngstrobj(L, l);
        memcpy(
            ((*ts).contents).as_mut_ptr() as *mut libc::c_void,
            str as *const libc::c_void,
            l.wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
        );
        return ts;
    };
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaS_new(
    mut L: *mut lua_State,
    mut str: *const libc::c_char,
) -> *mut TString {
    let mut i: libc::c_uint = ((str as uintptr_t
        & (2147483647 as libc::c_int as libc::c_uint)
            .wrapping_mul(2 as libc::c_uint)
            .wrapping_add(1 as libc::c_uint) as libc::c_ulong) as libc::c_uint)
        .wrapping_rem(53 as libc::c_int as libc::c_uint);
    let mut j: libc::c_int = 0;
    let mut p: *mut *mut TString = ((*(*L).l_G).strcache[i as usize]).as_mut_ptr();
    j = 0 as libc::c_int;
    while j < 2 as libc::c_int {
        if strcmp(str, ((**p.offset(j as isize)).contents).as_mut_ptr())
            == 0 as libc::c_int
        {
            return *p.offset(j as isize);
        }
        j += 1;
        j;
    }
    j = 2 as libc::c_int - 1 as libc::c_int;
    while j > 0 as libc::c_int {
        let ref mut fresh3 = *p.offset(j as isize);
        *fresh3 = *p.offset((j - 1 as libc::c_int) as isize);
        j -= 1;
        j;
    }
    let ref mut fresh4 = *p.offset(0 as libc::c_int as isize);
    *fresh4 = luaS_newlstr(L, str, strlen(str));
    return *p.offset(0 as libc::c_int as isize);
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaS_newudata(
    mut L: *mut lua_State,
    mut s: size_t,
    mut nuvalue: libc::c_int,
) -> *mut Udata {
    let mut u: *mut Udata = 0 as *mut Udata;
    let mut i: libc::c_int = 0;
    let mut o: *mut GCObject = 0 as *mut GCObject;
    if ((s
        > (if (::core::mem::size_of::<size_t>() as libc::c_ulong)
            < ::core::mem::size_of::<lua_Integer>() as libc::c_ulong
        {
            !(0 as libc::c_int as size_t)
        } else {
            9223372036854775807 as i64 as size_t
        })
            .wrapping_sub(
                (if nuvalue == 0 as libc::c_int {
                    32 as libc::c_ulong
                } else {
                    (40 as libc::c_ulong)
                        .wrapping_add(
                            (::core::mem::size_of::<UValue>() as libc::c_ulong)
                                .wrapping_mul(nuvalue as libc::c_ulong),
                        )
                }),
            )) as libc::c_int != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
    {
        luaM_toobig(L);
    }
    o = luaC_newobj(
        L,
        7 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int,
        (if nuvalue == 0 as libc::c_int {
            32 as libc::c_ulong
        } else {
            (40 as libc::c_ulong)
                .wrapping_add(
                    (::core::mem::size_of::<UValue>() as libc::c_ulong)
                        .wrapping_mul(nuvalue as libc::c_ulong),
                )
        })
            .wrapping_add(s),
    );
    u = &mut (*(o as *mut GCUnion)).u;
    (*u).len = s;
    (*u).nuvalue = nuvalue as libc::c_ushort;
    (*u).metatable = 0 as *mut Table;
    i = 0 as libc::c_int;
    while i < nuvalue {
        (*((*u).uv).as_mut_ptr().offset(i as isize))
            .uv
            .tt_ = (0 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int)
            as u8;
        i += 1;
        i;
    }
    return u;
}
