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
    fn frexp(_: f64, _: *mut libc::c_int) -> f64;
    fn luaO_ceillog2(x: libc::c_uint) -> libc::c_int;
    fn luaM_realloc_(
        L: *mut lua_State,
        block: *mut libc::c_void,
        oldsize: size_t,
        size: size_t,
    ) -> *mut libc::c_void;
    fn luaM_free_(L: *mut lua_State, block: *mut libc::c_void, osize: size_t);
    fn luaM_malloc_(
        L: *mut lua_State,
        size: size_t,
        tag: libc::c_int,
    ) -> *mut libc::c_void;
    fn luaG_runerror(L: *mut lua_State, fmt: *const libc::c_char, _: ...) -> !;
    fn luaD_throw(L: *mut lua_State, errcode: libc::c_int) -> !;
    fn luaC_newobj(L: *mut lua_State, tt: libc::c_int, sz: size_t) -> *mut GCObject;
    fn luaC_barrierback_(L: *mut lua_State, o: *mut GCObject);
    fn luaS_hashlongstr(ts: *mut TString) -> libc::c_uint;
    fn luaS_eqlngstr(a: *mut TString, b: *mut TString) -> libc::c_int;
    fn luaV_flttointeger(
        n: lua_Number,
        p: *mut lua_Integer,
        mode: F2Imod,
    ) -> libc::c_int;
}
pub type __sig_atomic_t = libc::c_int;
pub type ptrdiff_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type intptr_t = libc::c_long;
pub type uintptr_t = libc::c_ulong;
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
pub type lua_Unsigned = libc::c_ulonglong;
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
pub struct Upvaldesc {
    pub name: *mut TString,
    pub instack: lu_byte,
    pub index: lu_byte,
    pub kind: lu_byte,
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
pub union Closure {
    pub c: CClosure,
    pub l: LClosure,
}
pub type C2RustUnnamed_9 = libc::c_uint;
pub const TM_N: C2RustUnnamed_9 = 25;
pub const TM_CLOSE: C2RustUnnamed_9 = 24;
pub const TM_CALL: C2RustUnnamed_9 = 23;
pub const TM_CONCAT: C2RustUnnamed_9 = 22;
pub const TM_LE: C2RustUnnamed_9 = 21;
pub const TM_LT: C2RustUnnamed_9 = 20;
pub const TM_BNOT: C2RustUnnamed_9 = 19;
pub const TM_UNM: C2RustUnnamed_9 = 18;
pub const TM_SHR: C2RustUnnamed_9 = 17;
pub const TM_SHL: C2RustUnnamed_9 = 16;
pub const TM_BXOR: C2RustUnnamed_9 = 15;
pub const TM_BOR: C2RustUnnamed_9 = 14;
pub const TM_BAND: C2RustUnnamed_9 = 13;
pub const TM_IDIV: C2RustUnnamed_9 = 12;
pub const TM_DIV: C2RustUnnamed_9 = 11;
pub const TM_POW: C2RustUnnamed_9 = 10;
pub const TM_MOD: C2RustUnnamed_9 = 9;
pub const TM_MUL: C2RustUnnamed_9 = 8;
pub const TM_SUB: C2RustUnnamed_9 = 7;
pub const TM_ADD: C2RustUnnamed_9 = 6;
pub const TM_EQ: C2RustUnnamed_9 = 5;
pub const TM_LEN: C2RustUnnamed_9 = 4;
pub const TM_MODE: C2RustUnnamed_9 = 3;
pub const TM_GC: C2RustUnnamed_9 = 2;
pub const TM_NEWINDEX: C2RustUnnamed_9 = 1;
pub const TM_INDEX: C2RustUnnamed_9 = 0;
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
pub type F2Imod = libc::c_uint;
pub const F2Iceil: F2Imod = 2;
pub const F2Ifloor: F2Imod = 1;
pub const F2Ieq: F2Imod = 0;
static mut dummynode_: Node = Node {
    u: {
        let mut init = NodeKey {
            value_: Value {
                gc: 0 as *const GCObject as *mut GCObject,
            },
            tt_: (0 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int) as lu_byte,
            key_tt: (0 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int)
                as lu_byte,
            next: 0 as libc::c_int,
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
        tt_: (0 as libc::c_int | (2 as libc::c_int) << 4 as libc::c_int) as lu_byte,
    };
    init
};
unsafe extern "C" fn hashint(mut t: *const Table, mut i: lua_Integer) -> *mut Node {
    let mut ui: lua_Unsigned = i as lua_Unsigned;
    if ui <= 2147483647 as libc::c_int as libc::c_uint as libc::c_ulonglong {
        return &mut *((*t).node)
            .offset(
                (ui as libc::c_int
                    % (((1 as libc::c_int) << (*t).lsizenode as libc::c_int)
                        - 1 as libc::c_int | 1 as libc::c_int)) as isize,
            ) as *mut Node
    } else {
        return &mut *((*t).node)
            .offset(
                ui
                    .wrapping_rem(
                        (((1 as libc::c_int) << (*t).lsizenode as libc::c_int)
                            - 1 as libc::c_int | 1 as libc::c_int) as libc::c_ulonglong,
                    ) as isize,
            ) as *mut Node
    };
}
unsafe extern "C" fn l_hashfloat(mut n: lua_Number) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ni: lua_Integer = 0;
    n = frexp(n, &mut i)
        * -((-(2147483647 as libc::c_int) - 1 as libc::c_int) as lua_Number);
    if !(n
        >= (-(9223372036854775807 as i64) - 1 as i64)
            as f64
        && n
            < -((-(9223372036854775807 as i64) - 1 as i64)
                as f64)
        && {
            ni = n as i64;
            1 as libc::c_int != 0
        })
    {
        return 0 as libc::c_int
    } else {
        let mut u: libc::c_uint = (i as libc::c_uint).wrapping_add(ni as libc::c_uint);
        return (if u <= 2147483647 as libc::c_int as libc::c_uint { u } else { !u })
            as libc::c_int;
    };
}
unsafe extern "C" fn mainpositionTV(
    mut t: *const Table,
    mut key: *const TValue,
) -> *mut Node {
    match (*key).tt_ as libc::c_int & 0x3f as libc::c_int {
        3 => {
            let mut i: lua_Integer = (*key).value_.i;
            return hashint(t, i);
        }
        19 => {
            let mut n: lua_Number = (*key).value_.n;
            return &mut *((*t).node)
                .offset(
                    ((l_hashfloat as unsafe extern "C" fn(lua_Number) -> libc::c_int)(n)
                        % (((1 as libc::c_int) << (*t).lsizenode as libc::c_int)
                            - 1 as libc::c_int | 1 as libc::c_int)) as isize,
                ) as *mut Node;
        }
        4 => {
            let mut ts: *mut TString = &mut (*((*key).value_.gc as *mut GCUnion)).ts;
            return &mut *((*t).node)
                .offset(
                    ((*ts).hash
                        & (((1 as libc::c_int) << (*t).lsizenode as libc::c_int)
                            - 1 as libc::c_int) as libc::c_uint) as libc::c_int as isize,
                ) as *mut Node;
        }
        20 => {
            let mut ts_0: *mut TString = &mut (*((*key).value_.gc as *mut GCUnion)).ts;
            return &mut *((*t).node)
                .offset(
                    ((luaS_hashlongstr
                        as unsafe extern "C" fn(*mut TString) -> libc::c_uint)(ts_0)
                        & (((1 as libc::c_int) << (*t).lsizenode as libc::c_int)
                            - 1 as libc::c_int) as libc::c_uint) as libc::c_int as isize,
                ) as *mut Node;
        }
        1 => {
            return &mut *((*t).node)
                .offset(
                    (0 as libc::c_int
                        & ((1 as libc::c_int) << (*t).lsizenode as libc::c_int)
                            - 1 as libc::c_int) as isize,
                ) as *mut Node;
        }
        17 => {
            return &mut *((*t).node)
                .offset(
                    (1 as libc::c_int
                        & ((1 as libc::c_int) << (*t).lsizenode as libc::c_int)
                            - 1 as libc::c_int) as isize,
                ) as *mut Node;
        }
        2 => {
            let mut p: *mut libc::c_void = (*key).value_.p;
            return &mut *((*t).node)
                .offset(
                    ((p as uintptr_t
                        & (2147483647 as libc::c_int as libc::c_uint)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_add(1 as libc::c_uint) as libc::c_ulong)
                        as libc::c_uint)
                        .wrapping_rem(
                            (((1 as libc::c_int) << (*t).lsizenode as libc::c_int)
                                - 1 as libc::c_int | 1 as libc::c_int) as libc::c_uint,
                        ) as isize,
                ) as *mut Node;
        }
        22 => {
            let mut f: lua_CFunction = (*key).value_.f;
            return &mut *((*t).node)
                .offset(
                    ((::core::mem::transmute::<lua_CFunction, uintptr_t>(f)
                        & (2147483647 as libc::c_int as libc::c_uint)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_add(1 as libc::c_uint) as libc::c_ulong)
                        as libc::c_uint)
                        .wrapping_rem(
                            (((1 as libc::c_int) << (*t).lsizenode as libc::c_int)
                                - 1 as libc::c_int | 1 as libc::c_int) as libc::c_uint,
                        ) as isize,
                ) as *mut Node;
        }
        _ => {
            let mut o: *mut GCObject = (*key).value_.gc;
            return &mut *((*t).node)
                .offset(
                    ((o as uintptr_t
                        & (2147483647 as libc::c_int as libc::c_uint)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_add(1 as libc::c_uint) as libc::c_ulong)
                        as libc::c_uint)
                        .wrapping_rem(
                            (((1 as libc::c_int) << (*t).lsizenode as libc::c_int)
                                - 1 as libc::c_int | 1 as libc::c_int) as libc::c_uint,
                        ) as isize,
                ) as *mut Node;
        }
    };
}
#[inline]
unsafe extern "C" fn mainpositionfromnode(
    mut t: *const Table,
    mut nd: *mut Node,
) -> *mut Node {
    let mut key: TValue = TValue {
        value_: Value { gc: 0 as *mut GCObject },
        tt_: 0,
    };
    let mut io_: *mut TValue = &mut key;
    let mut n_: *const Node = nd;
    (*io_).value_ = (*n_).u.key_val;
    (*io_).tt_ = (*n_).u.key_tt;
    return mainpositionTV(t, &mut key);
}
unsafe extern "C" fn equalkey(
    mut k1: *const TValue,
    mut n2: *const Node,
    mut deadok: libc::c_int,
) -> libc::c_int {
    if (*k1).tt_ as libc::c_int != (*n2).u.key_tt as libc::c_int
        && !(deadok != 0
            && (*n2).u.key_tt as libc::c_int == 9 as libc::c_int + 2 as libc::c_int
            && (*k1).tt_ as libc::c_int & (1 as libc::c_int) << 6 as libc::c_int != 0)
    {
        return 0 as libc::c_int;
    }
    match (*n2).u.key_tt as libc::c_int {
        0 | 1 | 17 => return 1 as libc::c_int,
        3 => return ((*k1).value_.i == (*n2).u.key_val.i) as libc::c_int,
        19 => return ((*k1).value_.n == (*n2).u.key_val.n) as libc::c_int,
        2 => return ((*k1).value_.p == (*n2).u.key_val.p) as libc::c_int,
        22 => return ((*k1).value_.f == (*n2).u.key_val.f) as libc::c_int,
        84 => {
            return luaS_eqlngstr(
                &mut (*((*k1).value_.gc as *mut GCUnion)).ts,
                &mut (*((*n2).u.key_val.gc as *mut GCUnion)).ts,
            );
        }
        _ => return ((*k1).value_.gc == (*n2).u.key_val.gc) as libc::c_int,
    };
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaH_realasize(mut t: *const Table) -> libc::c_uint {
    if (*t).flags as libc::c_int & (1 as libc::c_int) << 7 as libc::c_int == 0
        || (*t).alimit & ((*t).alimit).wrapping_sub(1 as libc::c_int as libc::c_uint)
            == 0 as libc::c_int as libc::c_uint
    {
        return (*t).alimit
    } else {
        let mut size: libc::c_uint = (*t).alimit;
        size |= size >> 1 as libc::c_int;
        size |= size >> 2 as libc::c_int;
        size |= size >> 4 as libc::c_int;
        size |= size >> 8 as libc::c_int;
        size |= size >> 16 as libc::c_int;
        size = size.wrapping_add(1);
        size;
        return size;
    };
}
unsafe extern "C" fn ispow2realasize(mut t: *const Table) -> libc::c_int {
    return ((*t).flags as libc::c_int & (1 as libc::c_int) << 7 as libc::c_int != 0
        || (*t).alimit & ((*t).alimit).wrapping_sub(1 as libc::c_int as libc::c_uint)
            == 0 as libc::c_int as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn setlimittosize(mut t: *mut Table) -> libc::c_uint {
    (*t).alimit = luaH_realasize(t);
    (*t)
        .flags = ((*t).flags as libc::c_int
        & !((1 as libc::c_int) << 7 as libc::c_int) as lu_byte as libc::c_int)
        as lu_byte;
    return (*t).alimit;
}
unsafe extern "C" fn getgeneric(
    mut t: *mut Table,
    mut key: *const TValue,
    mut deadok: libc::c_int,
) -> *const TValue {
    let mut n: *mut Node = mainpositionTV(t, key);
    loop {
        if equalkey(key, n, deadok) != 0 {
            return &mut (*n).i_val
        } else {
            let mut nx: libc::c_int = (*n).u.next;
            if nx == 0 as libc::c_int {
                return &absentkey;
            }
            n = n.offset(nx as isize);
        }
    };
}
unsafe extern "C" fn arrayindex(mut k: lua_Integer) -> libc::c_uint {
    if (k as lua_Unsigned).wrapping_sub(1 as libc::c_uint as libc::c_ulonglong)
        < (if ((1 as libc::c_uint)
            << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int)
            as size_t
            <= (!(0 as libc::c_int as size_t))
                .wrapping_div(::core::mem::size_of::<TValue>() as libc::c_ulong)
        {
            (1 as libc::c_uint)
                << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int
        } else {
            (!(0 as libc::c_int as size_t))
                .wrapping_div(::core::mem::size_of::<TValue>() as libc::c_ulong)
                as libc::c_uint
        }) as libc::c_ulonglong
    {
        return k as libc::c_uint
    } else {
        return 0 as libc::c_int as libc::c_uint
    };
}
unsafe extern "C" fn findindex(
    mut L: *mut lua_State,
    mut t: *mut Table,
    mut key: *mut TValue,
    mut asize: libc::c_uint,
) -> libc::c_uint {
    let mut i: libc::c_uint = 0;
    if (*key).tt_ as libc::c_int & 0xf as libc::c_int == 0 as libc::c_int {
        return 0 as libc::c_int as libc::c_uint;
    }
    i = if (*key).tt_ as libc::c_int
        == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
    {
        arrayindex((*key).value_.i)
    } else {
        0 as libc::c_int as libc::c_uint
    };
    if i.wrapping_sub(1 as libc::c_uint) < asize {
        return i
    } else {
        let mut n: *const TValue = getgeneric(t, key, 1 as libc::c_int);
        if (((*n).tt_ as libc::c_int
            == 0 as libc::c_int | (2 as libc::c_int) << 4 as libc::c_int) as libc::c_int
            != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
        {
            luaG_runerror(
                L,
                b"invalid key to 'next'\0" as *const u8 as *const libc::c_char,
            );
        }
        i = (n as *mut Node)
            .offset_from(
                &mut *((*t).node).offset(0 as libc::c_int as isize) as *mut Node,
            ) as libc::c_long as libc::c_int as libc::c_uint;
        return i.wrapping_add(1 as libc::c_int as libc::c_uint).wrapping_add(asize);
    };
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaH_next(
    mut L: *mut lua_State,
    mut t: *mut Table,
    mut key: StkId,
) -> libc::c_int {
    let mut asize: libc::c_uint = luaH_realasize(t);
    let mut i: libc::c_uint = findindex(L, t, &mut (*key).val, asize);
    while i < asize {
        if !((*((*t).array).offset(i as isize)).tt_ as libc::c_int & 0xf as libc::c_int
            == 0 as libc::c_int)
        {
            let mut io: *mut TValue = &mut (*key).val;
            (*io)
                .value_
                .i = i.wrapping_add(1 as libc::c_int as libc::c_uint) as lua_Integer;
            (*io)
                .tt_ = (3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int)
                as lu_byte;
            let mut io1: *mut TValue = &mut (*key.offset(1 as libc::c_int as isize)).val;
            let mut io2: *const TValue = &mut *((*t).array).offset(i as isize)
                as *mut TValue;
            (*io1).value_ = (*io2).value_;
            (*io1).tt_ = (*io2).tt_;
            return 1 as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    i = i.wrapping_sub(asize);
    while (i as libc::c_int) < (1 as libc::c_int) << (*t).lsizenode as libc::c_int {
        if !((*((*t).node).offset(i as isize)).i_val.tt_ as libc::c_int
            & 0xf as libc::c_int == 0 as libc::c_int)
        {
            let mut n: *mut Node = &mut *((*t).node).offset(i as isize) as *mut Node;
            let mut io_: *mut TValue = &mut (*key).val;
            let mut n_: *const Node = n;
            (*io_).value_ = (*n_).u.key_val;
            (*io_).tt_ = (*n_).u.key_tt;
            let mut io1_0: *mut TValue = &mut (*key.offset(1 as libc::c_int as isize))
                .val;
            let mut io2_0: *const TValue = &mut (*n).i_val;
            (*io1_0).value_ = (*io2_0).value_;
            (*io1_0).tt_ = (*io2_0).tt_;
            return 1 as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn freehash(mut L: *mut lua_State, mut t: *mut Table) {
    if !((*t).lastfree).is_null() {
        luaM_free_(
            L,
            (*t).node as *mut libc::c_void,
            (((1 as libc::c_int) << (*t).lsizenode as libc::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<Node>() as libc::c_ulong),
        );
    }
}
unsafe extern "C" fn computesizes(
    mut nums: *mut libc::c_uint,
    mut pna: *mut libc::c_uint,
) -> libc::c_uint {
    let mut i: libc::c_int = 0;
    let mut twotoi: libc::c_uint = 0;
    let mut a: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut na: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut optimal: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    i = 0 as libc::c_int;
    twotoi = 1 as libc::c_int as libc::c_uint;
    while twotoi > 0 as libc::c_int as libc::c_uint
        && *pna > twotoi.wrapping_div(2 as libc::c_int as libc::c_uint)
    {
        a = a.wrapping_add(*nums.offset(i as isize));
        if a > twotoi.wrapping_div(2 as libc::c_int as libc::c_uint) {
            optimal = twotoi;
            na = a;
        }
        i += 1;
        i;
        twotoi = twotoi.wrapping_mul(2 as libc::c_int as libc::c_uint);
    }
    *pna = na;
    return optimal;
}
unsafe extern "C" fn countint(
    mut key: lua_Integer,
    mut nums: *mut libc::c_uint,
) -> libc::c_int {
    let mut k: libc::c_uint = arrayindex(key);
    if k != 0 as libc::c_int as libc::c_uint {
        let ref mut fresh0 = *nums.offset(luaO_ceillog2(k) as isize);
        *fresh0 = (*fresh0).wrapping_add(1);
        *fresh0;
        return 1 as libc::c_int;
    } else {
        return 0 as libc::c_int
    };
}
unsafe extern "C" fn numusearray(
    mut t: *const Table,
    mut nums: *mut libc::c_uint,
) -> libc::c_uint {
    let mut lg: libc::c_int = 0;
    let mut ttlg: libc::c_uint = 0;
    let mut ause: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut i: libc::c_uint = 1 as libc::c_int as libc::c_uint;
    let mut asize: libc::c_uint = (*t).alimit;
    lg = 0 as libc::c_int;
    ttlg = 1 as libc::c_int as libc::c_uint;
    while lg
        <= (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int
    {
        let mut lc: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        let mut lim: libc::c_uint = ttlg;
        if lim > asize {
            lim = asize;
            if i > lim {
                break;
            }
        }
        while i <= lim {
            if !((*((*t).array)
                .offset(i.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize))
                .tt_ as libc::c_int & 0xf as libc::c_int == 0 as libc::c_int)
            {
                lc = lc.wrapping_add(1);
                lc;
            }
            i = i.wrapping_add(1);
            i;
        }
        let ref mut fresh1 = *nums.offset(lg as isize);
        *fresh1 = (*fresh1).wrapping_add(lc);
        ause = ause.wrapping_add(lc);
        lg += 1;
        lg;
        ttlg = ttlg.wrapping_mul(2 as libc::c_int as libc::c_uint);
    }
    return ause;
}
unsafe extern "C" fn numusehash(
    mut t: *const Table,
    mut nums: *mut libc::c_uint,
    mut pna: *mut libc::c_uint,
) -> libc::c_int {
    let mut totaluse: libc::c_int = 0 as libc::c_int;
    let mut ause: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = (1 as libc::c_int) << (*t).lsizenode as libc::c_int;
    loop {
        let fresh2 = i;
        i = i - 1;
        if !(fresh2 != 0) {
            break;
        }
        let mut n: *mut Node = &mut *((*t).node).offset(i as isize) as *mut Node;
        if !((*n).i_val.tt_ as libc::c_int & 0xf as libc::c_int == 0 as libc::c_int) {
            if (*n).u.key_tt as libc::c_int
                == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
            {
                ause += countint((*n).u.key_val.i, nums);
            }
            totaluse += 1;
            totaluse;
        }
    }
    *pna = (*pna).wrapping_add(ause as libc::c_uint);
    return totaluse;
}
unsafe extern "C" fn setnodevector(
    mut L: *mut lua_State,
    mut t: *mut Table,
    mut size: libc::c_uint,
) {
    if size == 0 as libc::c_int as libc::c_uint {
        (*t).node = &dummynode_ as *const Node as *mut Node;
        (*t).lsizenode = 0 as libc::c_int as lu_byte;
        (*t).lastfree = 0 as *mut Node;
    } else {
        let mut i: libc::c_int = 0;
        let mut lsize: libc::c_int = luaO_ceillog2(size);
        if lsize
            > (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int
                - 1 as libc::c_int
            || (1 as libc::c_uint) << lsize
                > (if ((1 as libc::c_uint)
                    << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int
                        - 1 as libc::c_int) as size_t
                    <= (!(0 as libc::c_int as size_t))
                        .wrapping_div(::core::mem::size_of::<Node>() as libc::c_ulong)
                {
                    (1 as libc::c_uint)
                        << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            as libc::c_int - 1 as libc::c_int
                } else {
                    (!(0 as libc::c_int as size_t))
                        .wrapping_div(::core::mem::size_of::<Node>() as libc::c_ulong)
                        as libc::c_uint
                })
        {
            luaG_runerror(L, b"table overflow\0" as *const u8 as *const libc::c_char);
        }
        size = ((1 as libc::c_int) << lsize) as libc::c_uint;
        (*t)
            .node = luaM_malloc_(
            L,
            (size as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<Node>() as libc::c_ulong),
            0 as libc::c_int,
        ) as *mut Node;
        i = 0 as libc::c_int;
        while i < size as libc::c_int {
            let mut n: *mut Node = &mut *((*t).node).offset(i as isize) as *mut Node;
            (*n).u.next = 0 as libc::c_int;
            (*n).u.key_tt = 0 as libc::c_int as lu_byte;
            (*n)
                .i_val
                .tt_ = (0 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int)
                as lu_byte;
            i += 1;
            i;
        }
        (*t).lsizenode = lsize as lu_byte;
        (*t).lastfree = &mut *((*t).node).offset(size as isize) as *mut Node;
    };
}
unsafe extern "C" fn reinsert(
    mut L: *mut lua_State,
    mut ot: *mut Table,
    mut t: *mut Table,
) {
    let mut j: libc::c_int = 0;
    let mut size: libc::c_int = (1 as libc::c_int) << (*ot).lsizenode as libc::c_int;
    j = 0 as libc::c_int;
    while j < size {
        let mut old: *mut Node = &mut *((*ot).node).offset(j as isize) as *mut Node;
        if !((*old).i_val.tt_ as libc::c_int & 0xf as libc::c_int == 0 as libc::c_int) {
            let mut k: TValue = TValue {
                value_: Value { gc: 0 as *mut GCObject },
                tt_: 0,
            };
            let mut io_: *mut TValue = &mut k;
            let mut n_: *const Node = old;
            (*io_).value_ = (*n_).u.key_val;
            (*io_).tt_ = (*n_).u.key_tt;
            luaH_set(L, t, &mut k, &mut (*old).i_val);
        }
        j += 1;
        j;
    }
}
unsafe extern "C" fn exchangehashpart(mut t1: *mut Table, mut t2: *mut Table) {
    let mut lsizenode: lu_byte = (*t1).lsizenode;
    let mut node: *mut Node = (*t1).node;
    let mut lastfree: *mut Node = (*t1).lastfree;
    (*t1).lsizenode = (*t2).lsizenode;
    (*t1).node = (*t2).node;
    (*t1).lastfree = (*t2).lastfree;
    (*t2).lsizenode = lsizenode;
    (*t2).node = node;
    (*t2).lastfree = lastfree;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaH_resize(
    mut L: *mut lua_State,
    mut t: *mut Table,
    mut newasize: libc::c_uint,
    mut nhsize: libc::c_uint,
) {
    let mut i: libc::c_uint = 0;
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
    let mut oldasize: libc::c_uint = setlimittosize(t);
    let mut newarray: *mut TValue = 0 as *mut TValue;
    setnodevector(L, &mut newt, nhsize);
    if newasize < oldasize {
        (*t).alimit = newasize;
        exchangehashpart(t, &mut newt);
        i = newasize;
        while i < oldasize {
            if !((*((*t).array).offset(i as isize)).tt_ as libc::c_int
                & 0xf as libc::c_int == 0 as libc::c_int)
            {
                luaH_setint(
                    L,
                    t,
                    i.wrapping_add(1 as libc::c_int as libc::c_uint) as lua_Integer,
                    &mut *((*t).array).offset(i as isize),
                );
            }
            i = i.wrapping_add(1);
            i;
        }
        (*t).alimit = oldasize;
        exchangehashpart(t, &mut newt);
    }
    newarray = luaM_realloc_(
        L,
        (*t).array as *mut libc::c_void,
        (oldasize as size_t)
            .wrapping_mul(::core::mem::size_of::<TValue>() as libc::c_ulong),
        (newasize as size_t)
            .wrapping_mul(::core::mem::size_of::<TValue>() as libc::c_ulong),
    ) as *mut TValue;
    if ((newarray.is_null() && newasize > 0 as libc::c_int as libc::c_uint)
        as libc::c_int != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
    {
        freehash(L, &mut newt);
        luaD_throw(L, 4 as libc::c_int);
    }
    exchangehashpart(t, &mut newt);
    (*t).array = newarray;
    (*t).alimit = newasize;
    i = oldasize;
    while i < newasize {
        (*((*t).array).offset(i as isize))
            .tt_ = (0 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int)
            as lu_byte;
        i = i.wrapping_add(1);
        i;
    }
    reinsert(L, &mut newt, t);
    freehash(L, &mut newt);
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaH_resizearray(
    mut L: *mut lua_State,
    mut t: *mut Table,
    mut nasize: libc::c_uint,
) {
    let mut nsize: libc::c_int = if ((*t).lastfree).is_null() {
        0 as libc::c_int
    } else {
        (1 as libc::c_int) << (*t).lsizenode as libc::c_int
    };
    luaH_resize(L, t, nasize, nsize as libc::c_uint);
}
unsafe extern "C" fn rehash(
    mut L: *mut lua_State,
    mut t: *mut Table,
    mut ek: *const TValue,
) {
    let mut asize: libc::c_uint = 0;
    let mut na: libc::c_uint = 0;
    let mut nums: [libc::c_uint; 32] = [0; 32];
    let mut i: libc::c_int = 0;
    let mut totaluse: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i
        <= (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int
    {
        nums[i as usize] = 0 as libc::c_int as libc::c_uint;
        i += 1;
        i;
    }
    setlimittosize(t);
    na = numusearray(t, nums.as_mut_ptr());
    totaluse = na as libc::c_int;
    totaluse += numusehash(t, nums.as_mut_ptr(), &mut na);
    if (*ek).tt_ as libc::c_int
        == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
    {
        na = na
            .wrapping_add(countint((*ek).value_.i, nums.as_mut_ptr()) as libc::c_uint);
    }
    totaluse += 1;
    totaluse;
    asize = computesizes(nums.as_mut_ptr(), &mut na);
    luaH_resize(L, t, asize, (totaluse as libc::c_uint).wrapping_sub(na));
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaH_new(mut L: *mut lua_State) -> *mut Table {
    let mut o: *mut GCObject = luaC_newobj(
        L,
        5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int,
        ::core::mem::size_of::<Table>() as libc::c_ulong,
    );
    let mut t: *mut Table = &mut (*(o as *mut GCUnion)).h;
    (*t).metatable = 0 as *mut Table;
    (*t)
        .flags = !(!(0 as libc::c_uint) << TM_EQ as libc::c_int + 1 as libc::c_int)
        as lu_byte;
    (*t).array = 0 as *mut TValue;
    (*t).alimit = 0 as libc::c_int as libc::c_uint;
    setnodevector(L, t, 0 as libc::c_int as libc::c_uint);
    return t;
}
#[unsafe (no_mangle)]
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
            if (*(*t).lastfree).u.key_tt as libc::c_int == 0 as libc::c_int {
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
        value_: Value { gc: 0 as *mut GCObject },
        tt_: 0,
    };
    if (((*key).tt_ as libc::c_int & 0xf as libc::c_int == 0 as libc::c_int)
        as libc::c_int != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
    {
        luaG_runerror(L, b"table index is nil\0" as *const u8 as *const libc::c_char);
    } else if (*key).tt_ as libc::c_int
        == 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
    {
        let mut f: lua_Number = (*key).value_.n;
        let mut k: lua_Integer = 0;
        if luaV_flttointeger(f, &mut k, F2Ieq) != 0 {
            let mut io: *mut TValue = &mut aux;
            (*io).value_.i = k;
            (*io)
                .tt_ = (3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int)
                as lu_byte;
            key = &mut aux;
        } else if (!(f == f) as libc::c_int != 0 as libc::c_int) as libc::c_int
            as libc::c_long != 0
        {
            luaG_runerror(
                L,
                b"table index is NaN\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    if (*value).tt_ as libc::c_int & 0xf as libc::c_int == 0 as libc::c_int {
        return;
    }
    mp = mainpositionTV(t, key);
    if !((*mp).i_val.tt_ as libc::c_int & 0xf as libc::c_int == 0 as libc::c_int)
        || ((*t).lastfree).is_null()
    {
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
            (*othern).u.next = f_0.offset_from(othern) as libc::c_long as libc::c_int;
            *f_0 = *mp;
            if (*mp).u.next != 0 as libc::c_int {
                (*f_0).u.next += mp.offset_from(f_0) as libc::c_long as libc::c_int;
                (*mp).u.next = 0 as libc::c_int;
            }
            (*mp)
                .i_val
                .tt_ = (0 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int)
                as lu_byte;
        } else {
            if (*mp).u.next != 0 as libc::c_int {
                (*f_0)
                    .u
                    .next = mp.offset((*mp).u.next as isize).offset_from(f_0)
                    as libc::c_long as libc::c_int;
            }
            (*mp).u.next = f_0.offset_from(mp) as libc::c_long as libc::c_int;
            mp = f_0;
        }
    }
    let mut n_: *mut Node = mp;
    let mut io_: *const TValue = key;
    (*n_).u.key_val = (*io_).value_;
    (*n_).u.key_tt = (*io_).tt_;
    if (*key).tt_ as libc::c_int & (1 as libc::c_int) << 6 as libc::c_int != 0 {
        if (*(t as *mut GCUnion)).gc.marked as libc::c_int
            & (1 as libc::c_int) << 5 as libc::c_int != 0
            && (*(*key).value_.gc).marked as libc::c_int
                & ((1 as libc::c_int) << 3 as libc::c_int
                    | (1 as libc::c_int) << 4 as libc::c_int) != 0
        {
            luaC_barrierback_(L, &mut (*(t as *mut GCUnion)).gc);
        } else {};
    } else {};
    let mut io1: *mut TValue = &mut (*mp).i_val;
    let mut io2: *const TValue = value;
    (*io1).value_ = (*io2).value_;
    (*io1).tt_ = (*io2).tt_;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaH_getint(
    mut t: *mut Table,
    mut key: lua_Integer,
) -> *const TValue {
    let mut alimit: lua_Unsigned = (*t).alimit as lua_Unsigned;
    if (key as lua_Unsigned).wrapping_sub(1 as libc::c_uint as libc::c_ulonglong)
        < alimit
    {
        return &mut *((*t).array)
            .offset((key - 1 as libc::c_int as i64) as isize) as *mut TValue
    } else if (*t).flags as libc::c_int & (1 as libc::c_int) << 7 as libc::c_int != 0
        && (key as lua_Unsigned).wrapping_sub(1 as libc::c_uint as libc::c_ulonglong)
            & !alimit.wrapping_sub(1 as libc::c_uint as libc::c_ulonglong) < alimit
    {
        (*t).alimit = key as libc::c_uint;
        return &mut *((*t).array)
            .offset((key - 1 as libc::c_int as i64) as isize)
            as *mut TValue;
    } else {
        let mut n: *mut Node = hashint(t, key);
        loop {
            if (*n).u.key_tt as libc::c_int
                == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                && (*n).u.key_val.i == key
            {
                return &mut (*n).i_val
            } else {
                let mut nx: libc::c_int = (*n).u.next;
                if nx == 0 as libc::c_int {
                    break;
                }
                n = n.offset(nx as isize);
            }
        }
        return &absentkey;
    };
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaH_getshortstr(
    mut t: *mut Table,
    mut key: *mut TString,
) -> *const TValue {
    let mut n: *mut Node = &mut *((*t).node)
        .offset(
            ((*key).hash
                & (((1 as libc::c_int) << (*t).lsizenode as libc::c_int)
                    - 1 as libc::c_int) as libc::c_uint) as libc::c_int as isize,
        ) as *mut Node;
    loop {
        if (*n).u.key_tt as libc::c_int
            == 4 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                | (1 as libc::c_int) << 6 as libc::c_int
            && &mut (*((*n).u.key_val.gc as *mut GCUnion)).ts as *mut TString == key
        {
            return &mut (*n).i_val
        } else {
            let mut nx: libc::c_int = (*n).u.next;
            if nx == 0 as libc::c_int {
                return &absentkey;
            }
            n = n.offset(nx as isize);
        }
    };
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaH_getstr(
    mut t: *mut Table,
    mut key: *mut TString,
) -> *const TValue {
    if (*key).tt as libc::c_int
        == 4 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
    {
        return luaH_getshortstr(t, key)
    } else {
        let mut ko: TValue = TValue {
            value_: Value { gc: 0 as *mut GCObject },
            tt_: 0,
        };
        let mut io: *mut TValue = &mut ko;
        let mut x_: *mut TString = key;
        (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
        (*io)
            .tt_ = ((*x_).tt as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int)
            as lu_byte;
        return getgeneric(t, &mut ko, 0 as libc::c_int);
    };
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaH_get(
    mut t: *mut Table,
    mut key: *const TValue,
) -> *const TValue {
    match (*key).tt_ as libc::c_int & 0x3f as libc::c_int {
        4 => return luaH_getshortstr(t, &mut (*((*key).value_.gc as *mut GCUnion)).ts),
        3 => return luaH_getint(t, (*key).value_.i),
        0 => return &absentkey,
        19 => {
            let mut k: lua_Integer = 0;
            if luaV_flttointeger((*key).value_.n, &mut k, F2Ieq) != 0 {
                return luaH_getint(t, k);
            }
        }
        _ => {}
    }
    return getgeneric(t, key, 0 as libc::c_int);
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaH_finishset(
    mut L: *mut lua_State,
    mut t: *mut Table,
    mut key: *const TValue,
    mut slot: *const TValue,
    mut value: *mut TValue,
) {
    if (*slot).tt_ as libc::c_int
        == 0 as libc::c_int | (2 as libc::c_int) << 4 as libc::c_int
    {
        luaH_newkey(L, t, key, value);
    } else {
        let mut io1: *mut TValue = slot as *mut TValue;
        let mut io2: *const TValue = value;
        (*io1).value_ = (*io2).value_;
        (*io1).tt_ = (*io2).tt_;
    };
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaH_set(
    mut L: *mut lua_State,
    mut t: *mut Table,
    mut key: *const TValue,
    mut value: *mut TValue,
) {
    let mut slot: *const TValue = luaH_get(t, key);
    luaH_finishset(L, t, key, slot, value);
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaH_setint(
    mut L: *mut lua_State,
    mut t: *mut Table,
    mut key: lua_Integer,
    mut value: *mut TValue,
) {
    let mut p: *const TValue = luaH_getint(t, key);
    if (*p).tt_ as libc::c_int
        == 0 as libc::c_int | (2 as libc::c_int) << 4 as libc::c_int
    {
        let mut k: TValue = TValue {
            value_: Value { gc: 0 as *mut GCObject },
            tt_: 0,
        };
        let mut io: *mut TValue = &mut k;
        (*io).value_.i = key;
        (*io)
            .tt_ = (3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int)
            as lu_byte;
        luaH_newkey(L, t, &mut k, value);
    } else {
        let mut io1: *mut TValue = p as *mut TValue;
        let mut io2: *const TValue = value;
        (*io1).value_ = (*io2).value_;
        (*io1).tt_ = (*io2).tt_;
    };
}
unsafe extern "C" fn hash_search(
    mut t: *mut Table,
    mut j: lua_Unsigned,
) -> lua_Unsigned {
    let mut i: lua_Unsigned = 0;
    if j == 0 as libc::c_int as libc::c_ulonglong {
        j = j.wrapping_add(1);
        j;
    }
    loop {
        i = j;
        if j
            <= (9223372036854775807 as i64 as lua_Unsigned)
                .wrapping_div(2 as libc::c_int as libc::c_ulonglong)
        {
            j = (j as libc::c_ulonglong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulonglong) as lua_Unsigned
                as lua_Unsigned;
            if (*luaH_getint(t, j as lua_Integer)).tt_ as libc::c_int
                & 0xf as libc::c_int == 0 as libc::c_int
            {
                break;
            }
        } else {
            j = 9223372036854775807 as i64 as lua_Unsigned;
            if (*luaH_getint(t, j as lua_Integer)).tt_ as libc::c_int
                & 0xf as libc::c_int == 0 as libc::c_int
            {
                break;
            }
            return j;
        }
    }
    while j.wrapping_sub(i) > 1 as libc::c_uint as libc::c_ulonglong {
        let mut m: lua_Unsigned = i
            .wrapping_add(j)
            .wrapping_div(2 as libc::c_int as libc::c_ulonglong);
        if (*luaH_getint(t, m as lua_Integer)).tt_ as libc::c_int & 0xf as libc::c_int
            == 0 as libc::c_int
        {
            j = m;
        } else {
            i = m;
        }
    }
    return i;
}
unsafe extern "C" fn binsearch(
    mut array: *const TValue,
    mut i: libc::c_uint,
    mut j: libc::c_uint,
) -> libc::c_uint {
    while j.wrapping_sub(i) > 1 as libc::c_uint {
        let mut m: libc::c_uint = i
            .wrapping_add(j)
            .wrapping_div(2 as libc::c_int as libc::c_uint);
        if (*array.offset(m.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize)).tt_
            as libc::c_int & 0xf as libc::c_int == 0 as libc::c_int
        {
            j = m;
        } else {
            i = m;
        }
    }
    return i;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaH_getn(mut t: *mut Table) -> lua_Unsigned {
    let mut limit: libc::c_uint = (*t).alimit;
    if limit > 0 as libc::c_int as libc::c_uint
        && (*((*t).array)
            .offset(limit.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize))
            .tt_ as libc::c_int & 0xf as libc::c_int == 0 as libc::c_int
    {
        if limit >= 2 as libc::c_int as libc::c_uint
            && !((*((*t).array)
                .offset(limit.wrapping_sub(2 as libc::c_int as libc::c_uint) as isize))
                .tt_ as libc::c_int & 0xf as libc::c_int == 0 as libc::c_int)
        {
            if ispow2realasize(t) != 0
                && !(limit.wrapping_sub(1 as libc::c_int as libc::c_uint)
                    & limit
                        .wrapping_sub(1 as libc::c_int as libc::c_uint)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    == 0 as libc::c_int as libc::c_uint)
            {
                (*t).alimit = limit.wrapping_sub(1 as libc::c_int as libc::c_uint);
                (*t)
                    .flags = ((*t).flags as libc::c_int
                    | (1 as libc::c_int) << 7 as libc::c_int) as lu_byte;
            }
            return limit.wrapping_sub(1 as libc::c_int as libc::c_uint) as lua_Unsigned;
        } else {
            let mut boundary: libc::c_uint = binsearch(
                (*t).array,
                0 as libc::c_int as libc::c_uint,
                limit,
            );
            if ispow2realasize(t) != 0
                && boundary
                    > (luaH_realasize(t)).wrapping_div(2 as libc::c_int as libc::c_uint)
            {
                (*t).alimit = boundary;
                (*t)
                    .flags = ((*t).flags as libc::c_int
                    | (1 as libc::c_int) << 7 as libc::c_int) as lu_byte;
            }
            return boundary as lua_Unsigned;
        }
    }
    if !((*t).flags as libc::c_int & (1 as libc::c_int) << 7 as libc::c_int == 0
        || (*t).alimit & ((*t).alimit).wrapping_sub(1 as libc::c_int as libc::c_uint)
            == 0 as libc::c_int as libc::c_uint)
    {
        if (*((*t).array).offset(limit as isize)).tt_ as libc::c_int & 0xf as libc::c_int
            == 0 as libc::c_int
        {
            return limit as lua_Unsigned;
        }
        limit = luaH_realasize(t);
        if (*((*t).array)
            .offset(limit.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize))
            .tt_ as libc::c_int & 0xf as libc::c_int == 0 as libc::c_int
        {
            let mut boundary_0: libc::c_uint = binsearch((*t).array, (*t).alimit, limit);
            (*t).alimit = boundary_0;
            return boundary_0 as lua_Unsigned;
        }
    }
    if ((*t).lastfree).is_null()
        || (*luaH_getint(
            t,
            limit.wrapping_add(1 as libc::c_int as libc::c_uint) as lua_Integer,
        ))
            .tt_ as libc::c_int & 0xf as libc::c_int == 0 as libc::c_int
    {
        return limit as lua_Unsigned
    } else {
        return hash_search(t, limit as lua_Unsigned)
    };
}
