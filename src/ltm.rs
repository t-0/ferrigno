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
    fn luaG_concaterror(L: *mut lua_State, p1: *const TValue, p2: *const TValue) -> !;
    fn luaG_opinterror(
        L: *mut lua_State,
        p1: *const TValue,
        p2: *const TValue,
        msg: *const libc::c_char,
    ) -> !;
    fn luaG_tointerror(L: *mut lua_State, p1: *const TValue, p2: *const TValue) -> !;
    fn luaG_ordererror(L: *mut lua_State, p1: *const TValue, p2: *const TValue) -> !;
    fn luaD_call(L: *mut lua_State, func: StkId, nResults: libc::c_int);
    fn luaD_callnoyield(L: *mut lua_State, func: StkId, nResults: libc::c_int);
    fn luaD_growstack(
        L: *mut lua_State,
        n: libc::c_int,
        raiseerror: libc::c_int,
    ) -> libc::c_int;
    fn luaC_fix(L: *mut lua_State, o: *mut GCObject);
    fn luaC_step(L: *mut lua_State);
    fn luaS_new(L: *mut lua_State, str: *const libc::c_char) -> *mut TString;
    fn luaH_getshortstr(t: *mut Table, key: *mut TString) -> *const TValue;
}
pub type size_t = libc::c_ulong;
pub type ptrdiff_t = libc::c_long;
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
pub type ls_byte = libc::c_schar;
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
static mut udatatypename: [libc::c_char; 9] = unsafe {
    *::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"userdata\0")
};
#[unsafe (no_mangle)]
pub static mut luaT_typenames_: [*const libc::c_char; 12] = unsafe {
    [
        b"no value\0" as *const u8 as *const libc::c_char,
        b"nil\0" as *const u8 as *const libc::c_char,
        b"boolean\0" as *const u8 as *const libc::c_char,
        udatatypename.as_ptr(),
        b"number\0" as *const u8 as *const libc::c_char,
        b"string\0" as *const u8 as *const libc::c_char,
        b"table\0" as *const u8 as *const libc::c_char,
        b"function\0" as *const u8 as *const libc::c_char,
        udatatypename.as_ptr(),
        b"thread\0" as *const u8 as *const libc::c_char,
        b"upvalue\0" as *const u8 as *const libc::c_char,
        b"proto\0" as *const u8 as *const libc::c_char,
    ]
};
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaT_init(mut L: *mut lua_State) {
    static mut luaT_eventname: [*const libc::c_char; 25] = [
        b"__index\0" as *const u8 as *const libc::c_char,
        b"__newindex\0" as *const u8 as *const libc::c_char,
        b"__gc\0" as *const u8 as *const libc::c_char,
        b"__mode\0" as *const u8 as *const libc::c_char,
        b"__len\0" as *const u8 as *const libc::c_char,
        b"__eq\0" as *const u8 as *const libc::c_char,
        b"__add\0" as *const u8 as *const libc::c_char,
        b"__sub\0" as *const u8 as *const libc::c_char,
        b"__mul\0" as *const u8 as *const libc::c_char,
        b"__mod\0" as *const u8 as *const libc::c_char,
        b"__pow\0" as *const u8 as *const libc::c_char,
        b"__div\0" as *const u8 as *const libc::c_char,
        b"__idiv\0" as *const u8 as *const libc::c_char,
        b"__band\0" as *const u8 as *const libc::c_char,
        b"__bor\0" as *const u8 as *const libc::c_char,
        b"__bxor\0" as *const u8 as *const libc::c_char,
        b"__shl\0" as *const u8 as *const libc::c_char,
        b"__shr\0" as *const u8 as *const libc::c_char,
        b"__unm\0" as *const u8 as *const libc::c_char,
        b"__bnot\0" as *const u8 as *const libc::c_char,
        b"__lt\0" as *const u8 as *const libc::c_char,
        b"__le\0" as *const u8 as *const libc::c_char,
        b"__concat\0" as *const u8 as *const libc::c_char,
        b"__call\0" as *const u8 as *const libc::c_char,
        b"__close\0" as *const u8 as *const libc::c_char,
    ];
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < TM_N as libc::c_int {
        (*(*L).l_G).tmname[i as usize] = luaS_new(L, luaT_eventname[i as usize]);
        luaC_fix(
            L,
            &mut (*(*((*(*L).l_G).tmname).as_mut_ptr().offset(i as isize)
                as *mut GCUnion))
                .gc,
        );
        i += 1;
        i;
    }
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaT_gettm(
    mut events: *mut Table,
    mut event: TMS,
    mut ename: *mut TString,
) -> *const TValue {
    let mut tm: *const TValue = luaH_getshortstr(events, ename);
    if (*tm).tt_ as libc::c_int & 0xf as libc::c_int == 0 as libc::c_int {
        (*events)
            .flags = ((*events).flags as libc::c_int
            | ((1 as libc::c_uint) << event as libc::c_uint) as u8 as libc::c_int)
            as u8;
        return 0 as *const TValue;
    } else {
        return tm
    };
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaT_gettmbyobj(
    mut L: *mut lua_State,
    mut o: *const TValue,
    mut event: TMS,
) -> *const TValue {
    let mut mt: *mut Table = 0 as *mut Table;
    match (*o).tt_ as libc::c_int & 0xf as libc::c_int {
        5 => {
            mt = (*((*o).value_.gc as *mut GCUnion)).h.metatable;
        }
        7 => {
            mt = (*((*o).value_.gc as *mut GCUnion)).u.metatable;
        }
        _ => {
            mt = (*(*L).l_G).mt[((*o).tt_ as libc::c_int & 0xf as libc::c_int) as usize];
        }
    }
    return if !mt.is_null() {
        luaH_getshortstr(mt, (*(*L).l_G).tmname[event as usize])
    } else {
        &mut (*(*L).l_G).nilvalue as *mut TValue as *const TValue
    };
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaT_objtypename(
    mut L: *mut lua_State,
    mut o: *const TValue,
) -> *const libc::c_char {
    let mut mt: *mut Table = 0 as *mut Table;
    if (*o).tt_ as libc::c_int
        == 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
            | (1 as libc::c_int) << 6 as libc::c_int
        && {
            mt = (*((*o).value_.gc as *mut GCUnion)).h.metatable;
            !mt.is_null()
        }
        || (*o).tt_ as libc::c_int
            == 7 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                | (1 as libc::c_int) << 6 as libc::c_int
            && {
                mt = (*((*o).value_.gc as *mut GCUnion)).u.metatable;
                !mt.is_null()
            }
    {
        let mut name: *const TValue = luaH_getshortstr(
            mt,
            luaS_new(L, b"__name\0" as *const u8 as *const libc::c_char),
        );
        if (*name).tt_ as libc::c_int & 0xf as libc::c_int == 4 as libc::c_int {
            return ((*((*name).value_.gc as *mut GCUnion)).ts.contents).as_mut_ptr();
        }
    }
    return luaT_typenames_[(((*o).tt_ as libc::c_int & 0xf as libc::c_int)
        + 1 as libc::c_int) as usize];
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaT_callTM(
    mut L: *mut lua_State,
    mut f: *const TValue,
    mut p1: *const TValue,
    mut p2: *const TValue,
    mut p3: *const TValue,
) {
    let mut func: StkId = (*L).top.p;
    let mut io1: *mut TValue = &mut (*func).val;
    let mut io2: *const TValue = f;
    (*io1).value_ = (*io2).value_;
    (*io1).tt_ = (*io2).tt_;
    let mut io1_0: *mut TValue = &mut (*func.offset(1 as libc::c_int as isize)).val;
    let mut io2_0: *const TValue = p1;
    (*io1_0).value_ = (*io2_0).value_;
    (*io1_0).tt_ = (*io2_0).tt_;
    let mut io1_1: *mut TValue = &mut (*func.offset(2 as libc::c_int as isize)).val;
    let mut io2_1: *const TValue = p2;
    (*io1_1).value_ = (*io2_1).value_;
    (*io1_1).tt_ = (*io2_1).tt_;
    let mut io1_2: *mut TValue = &mut (*func.offset(3 as libc::c_int as isize)).val;
    let mut io2_2: *const TValue = p3;
    (*io1_2).value_ = (*io2_2).value_;
    (*io1_2).tt_ = (*io2_2).tt_;
    (*L).top.p = func.offset(4 as libc::c_int as isize);
    if (*(*L).ci).callstatus as libc::c_int
        & ((1 as libc::c_int) << 1 as libc::c_int
            | (1 as libc::c_int) << 3 as libc::c_int) == 0
    {
        luaD_call(L, func, 0 as libc::c_int);
    } else {
        luaD_callnoyield(L, func, 0 as libc::c_int);
    };
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaT_callTMres(
    mut L: *mut lua_State,
    mut f: *const TValue,
    mut p1: *const TValue,
    mut p2: *const TValue,
    mut res: StkId,
) {
    let mut result: ptrdiff_t = (res as *mut libc::c_char)
        .offset_from((*L).stack.p as *mut libc::c_char) as libc::c_long;
    let mut func: StkId = (*L).top.p;
    let mut io1: *mut TValue = &mut (*func).val;
    let mut io2: *const TValue = f;
    (*io1).value_ = (*io2).value_;
    (*io1).tt_ = (*io2).tt_;
    let mut io1_0: *mut TValue = &mut (*func.offset(1 as libc::c_int as isize)).val;
    let mut io2_0: *const TValue = p1;
    (*io1_0).value_ = (*io2_0).value_;
    (*io1_0).tt_ = (*io2_0).tt_;
    let mut io1_1: *mut TValue = &mut (*func.offset(2 as libc::c_int as isize)).val;
    let mut io2_1: *const TValue = p2;
    (*io1_1).value_ = (*io2_1).value_;
    (*io1_1).tt_ = (*io2_1).tt_;
    (*L).top.p = ((*L).top.p).offset(3 as libc::c_int as isize);
    if (*(*L).ci).callstatus as libc::c_int
        & ((1 as libc::c_int) << 1 as libc::c_int
            | (1 as libc::c_int) << 3 as libc::c_int) == 0
    {
        luaD_call(L, func, 1 as libc::c_int);
    } else {
        luaD_callnoyield(L, func, 1 as libc::c_int);
    }
    res = ((*L).stack.p as *mut libc::c_char).offset(result as isize) as StkId;
    let mut io1_2: *mut TValue = &mut (*res).val;
    (*L).top.p = ((*L).top.p).offset(-1);
    let mut io2_2: *const TValue = &mut (*(*L).top.p).val;
    (*io1_2).value_ = (*io2_2).value_;
    (*io1_2).tt_ = (*io2_2).tt_;
}
unsafe extern "C" fn callbinTM(
    mut L: *mut lua_State,
    mut p1: *const TValue,
    mut p2: *const TValue,
    mut res: StkId,
    mut event: TMS,
) -> libc::c_int {
    let mut tm: *const TValue = luaT_gettmbyobj(L, p1, event);
    if (*tm).tt_ as libc::c_int & 0xf as libc::c_int == 0 as libc::c_int {
        tm = luaT_gettmbyobj(L, p2, event);
    }
    if (*tm).tt_ as libc::c_int & 0xf as libc::c_int == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    luaT_callTMres(L, tm, p1, p2, res);
    return 1 as libc::c_int;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaT_trybinTM(
    mut L: *mut lua_State,
    mut p1: *const TValue,
    mut p2: *const TValue,
    mut res: StkId,
    mut event: TMS,
) {
    if ((callbinTM(L, p1, p2, res, event) == 0) as libc::c_int != 0 as libc::c_int)
        as libc::c_int as libc::c_long != 0
    {
        match event as libc::c_uint {
            13 | 14 | 15 | 16 | 17 | 19 => {
                if (*p1).tt_ as libc::c_int & 0xf as libc::c_int == 3 as libc::c_int
                    && (*p2).tt_ as libc::c_int & 0xf as libc::c_int == 3 as libc::c_int
                {
                    luaG_tointerror(L, p1, p2);
                } else {
                    luaG_opinterror(
                        L,
                        p1,
                        p2,
                        b"perform bitwise operation on\0" as *const u8
                            as *const libc::c_char,
                    );
                }
            }
            _ => {
                luaG_opinterror(
                    L,
                    p1,
                    p2,
                    b"perform arithmetic on\0" as *const u8 as *const libc::c_char,
                );
            }
        }
    }
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaT_tryconcatTM(mut L: *mut lua_State) {
    let mut top: StkId = (*L).top.p;
    if ((callbinTM(
        L,
        &mut (*top.offset(-(2 as libc::c_int as isize))).val,
        &mut (*top.offset(-(1 as libc::c_int as isize))).val,
        top.offset(-(2 as libc::c_int as isize)),
        TM_CONCAT,
    ) == 0) as libc::c_int != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
    {
        luaG_concaterror(
            L,
            &mut (*top.offset(-(2 as libc::c_int as isize))).val,
            &mut (*top.offset(-(1 as libc::c_int as isize))).val,
        );
    }
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaT_trybinassocTM(
    mut L: *mut lua_State,
    mut p1: *const TValue,
    mut p2: *const TValue,
    mut flip: libc::c_int,
    mut res: StkId,
    mut event: TMS,
) {
    if flip != 0 {
        luaT_trybinTM(L, p2, p1, res, event);
    } else {
        luaT_trybinTM(L, p1, p2, res, event);
    };
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaT_trybiniTM(
    mut L: *mut lua_State,
    mut p1: *const TValue,
    mut i2: Integer,
    mut flip: libc::c_int,
    mut res: StkId,
    mut event: TMS,
) {
    let mut aux: TValue = TValue {
        value_: Value { gc: 0 as *mut GCObject },
        tt_: 0,
    };
    let mut io: *mut TValue = &mut aux;
    (*io).value_.i = i2;
    (*io).tt_ = (3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int) as u8;
    luaT_trybinassocTM(L, p1, &mut aux, flip, res, event);
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaT_callorderTM(
    mut L: *mut lua_State,
    mut p1: *const TValue,
    mut p2: *const TValue,
    mut event: TMS,
) -> libc::c_int {
    if callbinTM(L, p1, p2, (*L).top.p, event) != 0 {
        return !((*(*L).top.p).val.tt_ as libc::c_int
            == 1 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
            || (*(*L).top.p).val.tt_ as libc::c_int & 0xf as libc::c_int
                == 0 as libc::c_int) as libc::c_int;
    }
    luaG_ordererror(L, p1, p2);
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaT_callorderiTM(
    mut L: *mut lua_State,
    mut p1: *const TValue,
    mut v2: libc::c_int,
    mut flip: libc::c_int,
    mut isfloat: libc::c_int,
    mut event: TMS,
) -> libc::c_int {
    let mut aux: TValue = TValue {
        value_: Value { gc: 0 as *mut GCObject },
        tt_: 0,
    };
    let mut p2: *const TValue = 0 as *const TValue;
    if isfloat != 0 {
        let mut io: *mut TValue = &mut aux;
        (*io).value_.n = v2 as Number;
        (*io)
            .tt_ = (3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int)
            as u8;
    } else {
        let mut io_0: *mut TValue = &mut aux;
        (*io_0).value_.i = v2 as Integer;
        (*io_0)
            .tt_ = (3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int)
            as u8;
    }
    if flip != 0 {
        p2 = p1;
        p1 = &mut aux;
    } else {
        p2 = &mut aux;
    }
    return luaT_callorderTM(L, p1, p2, event);
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaT_adjustvarargs(
    mut L: *mut lua_State,
    mut nfixparams: libc::c_int,
    mut ci: *mut CallInfo,
    mut p: *const Proto,
) {
    let mut i: libc::c_int = 0;
    let mut actual: libc::c_int = ((*L).top.p).offset_from((*ci).func.p) as libc::c_long
        as libc::c_int - 1 as libc::c_int;
    let mut nextra: libc::c_int = actual - nfixparams;
    (*ci).u.l.nextraargs = nextra;
    if ((((*L).stack_last.p).offset_from((*L).top.p) as libc::c_long
        <= ((*p).maxstacksize as libc::c_int + 1 as libc::c_int) as libc::c_long)
        as libc::c_int != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
    {
        luaD_growstack(
            L,
            (*p).maxstacksize as libc::c_int + 1 as libc::c_int,
            1 as libc::c_int,
        );
    }
    let fresh0 = (*L).top.p;
    (*L).top.p = ((*L).top.p).offset(1);
    let mut io1: *mut TValue = &mut (*fresh0).val;
    let mut io2: *const TValue = &mut (*(*ci).func.p).val;
    (*io1).value_ = (*io2).value_;
    (*io1).tt_ = (*io2).tt_;
    i = 1 as libc::c_int;
    while i <= nfixparams {
        let fresh1 = (*L).top.p;
        (*L).top.p = ((*L).top.p).offset(1);
        let mut io1_0: *mut TValue = &mut (*fresh1).val;
        let mut io2_0: *const TValue = &mut (*((*ci).func.p).offset(i as isize)).val;
        (*io1_0).value_ = (*io2_0).value_;
        (*io1_0).tt_ = (*io2_0).tt_;
        (*((*ci).func.p).offset(i as isize))
            .val
            .tt_ = (0 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int)
            as u8;
        i += 1;
        i;
    }
    (*ci).func.p = ((*ci).func.p).offset((actual + 1 as libc::c_int) as isize);
    (*ci).top.p = ((*ci).top.p).offset((actual + 1 as libc::c_int) as isize);
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaT_getvarargs(
    mut L: *mut lua_State,
    mut ci: *mut CallInfo,
    mut where_0: StkId,
    mut wanted: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut nextra: libc::c_int = (*ci).u.l.nextraargs;
    if wanted < 0 as libc::c_int {
        wanted = nextra;
        if ((((*L).stack_last.p).offset_from((*L).top.p) as libc::c_long
            <= nextra as libc::c_long) as libc::c_int != 0 as libc::c_int) as libc::c_int
            as libc::c_long != 0
        {
            let mut t__: ptrdiff_t = (where_0 as *mut libc::c_char)
                .offset_from((*L).stack.p as *mut libc::c_char) as libc::c_long;
            if (*(*L).l_G).GCdebt > 0 as libc::c_int as libc::c_long {
                luaC_step(L);
            }
            luaD_growstack(L, nextra, 1 as libc::c_int);
            where_0 = ((*L).stack.p as *mut libc::c_char).offset(t__ as isize) as StkId;
        }
        (*L).top.p = where_0.offset(nextra as isize);
    }
    i = 0 as libc::c_int;
    while i < wanted && i < nextra {
        let mut io1: *mut TValue = &mut (*where_0.offset(i as isize)).val;
        let mut io2: *const TValue = &mut (*((*ci).func.p)
            .offset(-(nextra as isize))
            .offset(i as isize))
            .val;
        (*io1).value_ = (*io2).value_;
        (*io1).tt_ = (*io2).tt_;
        i += 1;
        i;
    }
    while i < wanted {
        (*where_0.offset(i as isize))
            .val
            .tt_ = (0 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int)
            as u8;
        i += 1;
        i;
    }
}
