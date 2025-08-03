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
}
pub type size_t = libc::c_ulong;
pub type ptrdiff_t = libc::c_long;
pub type __sig_atomic_t = libc::c_int;
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
pub type lua_Number = f64;
pub type lua_Integer = i64;
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
pub struct Zio {
    pub n: size_t,
    pub p: *const libc::c_char,
    pub reader: lua_Reader,
    pub data: *mut libc::c_void,
    pub L: *mut lua_State,
}
pub type ZIO = Zio;
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaZ_fill(mut z: *mut ZIO) -> libc::c_int {
    let mut size: size_t = 0;
    let mut L: *mut lua_State = (*z).L;
    let mut buff: *const libc::c_char = 0 as *const libc::c_char;
    buff = ((*z).reader).expect("non-null function pointer")(L, (*z).data, &mut size);
    if buff.is_null() || size == 0 as libc::c_int as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    (*z).n = size.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    (*z).p = buff;
    let fresh0 = (*z).p;
    (*z).p = ((*z).p).offset(1);
    return *fresh0 as libc::c_uchar as libc::c_int;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaZ_init(
    mut L: *mut lua_State,
    mut z: *mut ZIO,
    mut reader: lua_Reader,
    mut data: *mut libc::c_void,
) {
    (*z).L = L;
    (*z).reader = reader;
    (*z).data = data;
    (*z).n = 0 as libc::c_int as size_t;
    (*z).p = 0 as *const libc::c_char;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaZ_read(
    mut z: *mut ZIO,
    mut b: *mut libc::c_void,
    mut n: size_t,
) -> size_t {
    while n != 0 {
        let mut m: size_t = 0;
        if (*z).n == 0 as libc::c_int as libc::c_ulong {
            if luaZ_fill(z) == -(1 as libc::c_int) {
                return n
            } else {
                (*z).n = ((*z).n).wrapping_add(1);
                (*z).n;
                (*z).p = ((*z).p).offset(-1);
                (*z).p;
            }
        }
        m = if n <= (*z).n { n } else { (*z).n };
        memcpy(b, (*z).p as *const libc::c_void, m);
        (*z).n = ((*z).n as libc::c_ulong).wrapping_sub(m) as size_t as size_t;
        (*z).p = ((*z).p).offset(m as isize);
        b = (b as *mut libc::c_char).offset(m as isize) as *mut libc::c_void;
        n = (n as libc::c_ulong).wrapping_sub(m) as size_t as size_t;
    }
    return 0 as libc::c_int as size_t;
}
