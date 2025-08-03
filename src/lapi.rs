#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(c_variadic, extern_types)]
unsafe extern "C" {
    pub type lua_longjmp;
    fn luaE_setdebt(g: *mut global_State, debt: l_mem);
    fn luaE_warning(L: *mut lua_State, msg: *const libc::c_char, tocont: libc::c_int);
    fn luaO_arith(
        L: *mut lua_State,
        op: libc::c_int,
        p1: *const TValue,
        p2: *const TValue,
        res: StkId,
    );
    fn luaO_str2num(s: *const libc::c_char, o: *mut TValue) -> size_t;
    fn luaO_tostring(L: *mut lua_State, obj: *mut TValue);
    fn luaO_pushvfstring(
        L: *mut lua_State,
        fmt: *const libc::c_char,
        argp: ::core::ffi::VaList,
    ) -> *const libc::c_char;
    static luaT_typenames_: [*const libc::c_char; 12];
    fn luaZ_init(
        L: *mut lua_State,
        z: *mut ZIO,
        reader: lua_Reader,
        data: *mut libc::c_void,
    );
    fn luaG_errormsg(L: *mut lua_State) -> !;
    fn luaD_protectedparser(
        L: *mut lua_State,
        z: *mut ZIO,
        name: *const libc::c_char,
        mode: *const libc::c_char,
    ) -> libc::c_int;
    fn luaD_call(L: *mut lua_State, func: StkId, nResults: libc::c_int);
    fn luaD_callnoyield(L: *mut lua_State, func: StkId, nResults: libc::c_int);
    fn luaD_pcall(
        L: *mut lua_State,
        func: Pfunc,
        u: *mut libc::c_void,
        oldtop: ptrdiff_t,
        ef: ptrdiff_t,
    ) -> libc::c_int;
    fn luaD_growstack(
        L: *mut lua_State,
        n: libc::c_int,
        raiseerror: libc::c_int,
    ) -> libc::c_int;
    fn luaD_throw(L: *mut lua_State, errcode: libc::c_int) -> !;
    fn luaF_newCclosure(L: *mut lua_State, nupvals: libc::c_int) -> *mut CClosure;
    fn luaF_newtbcupval(L: *mut lua_State, level: StkId);
    fn luaF_close(
        L: *mut lua_State,
        level: StkId,
        status: libc::c_int,
        yy: libc::c_int,
    ) -> StkId;
    fn luaC_step(L: *mut lua_State);
    fn luaC_fullgc(L: *mut lua_State, isemergency: libc::c_int);
    fn luaC_barrier_(L: *mut lua_State, o: *mut GCObject, v: *mut GCObject);
    fn luaC_barrierback_(L: *mut lua_State, o: *mut GCObject);
    fn luaC_checkfinalizer(L: *mut lua_State, o: *mut GCObject, mt: *mut Table);
    fn luaC_changemode(L: *mut lua_State, newmode: libc::c_int);
    fn luaS_newudata(L: *mut lua_State, s: size_t, nuvalue: libc::c_int) -> *mut Udata;
    fn luaS_newlstr(
        L: *mut lua_State,
        str: *const libc::c_char,
        l: size_t,
    ) -> *mut TString;
    fn luaS_new(L: *mut lua_State, str: *const libc::c_char) -> *mut TString;
    fn luaH_getint(t: *mut Table, key: lua_Integer) -> *const TValue;
    fn luaH_setint(
        L: *mut lua_State,
        t: *mut Table,
        key: lua_Integer,
        value: *mut TValue,
    );
    fn luaH_getstr(t: *mut Table, key: *mut TString) -> *const TValue;
    fn luaH_get(t: *mut Table, key: *const TValue) -> *const TValue;
    fn luaH_set(
        L: *mut lua_State,
        t: *mut Table,
        key: *const TValue,
        value: *mut TValue,
    );
    fn luaH_new(L: *mut lua_State) -> *mut Table;
    fn luaH_resize(
        L: *mut lua_State,
        t: *mut Table,
        nasize: libc::c_uint,
        nhsize: libc::c_uint,
    );
    fn luaH_next(L: *mut lua_State, t: *mut Table, key: StkId) -> libc::c_int;
    fn luaH_getn(t: *mut Table) -> lua_Unsigned;
    fn luaU_dump(
        L: *mut lua_State,
        f: *const Proto,
        w: lua_Writer,
        data: *mut libc::c_void,
        strip: libc::c_int,
    ) -> libc::c_int;
    fn luaV_equalobj(
        L: *mut lua_State,
        t1: *const TValue,
        t2: *const TValue,
    ) -> libc::c_int;
    fn luaV_lessthan(
        L: *mut lua_State,
        l: *const TValue,
        r: *const TValue,
    ) -> libc::c_int;
    fn luaV_lessequal(
        L: *mut lua_State,
        l: *const TValue,
        r: *const TValue,
    ) -> libc::c_int;
    fn luaV_tonumber_(obj: *const TValue, n: *mut lua_Number) -> libc::c_int;
    fn luaV_tointeger(
        obj: *const TValue,
        p: *mut lua_Integer,
        mode: F2Imod,
    ) -> libc::c_int;
    fn luaV_finishget(
        L: *mut lua_State,
        t: *const TValue,
        key: *mut TValue,
        val: StkId,
        slot: *const TValue,
    );
    fn luaV_finishset(
        L: *mut lua_State,
        t: *const TValue,
        key: *mut TValue,
        val: *mut TValue,
        slot: *const TValue,
    );
    fn luaV_concat(L: *mut lua_State, total: libc::c_int);
    fn luaV_objlen(L: *mut lua_State, ra: StkId, rb: *const TValue);
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type va_list = __builtin_va_list;
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
pub type lua_Unsigned = libc::c_ulonglong;
pub type lua_Reader = Option::<
    unsafe extern "C" fn(
        *mut lua_State,
        *mut libc::c_void,
        *mut size_t,
    ) -> *const libc::c_char,
>;
pub type lua_Writer = Option::<
    unsafe extern "C" fn(
        *mut lua_State,
        *const libc::c_void,
        size_t,
        *mut libc::c_void,
    ) -> libc::c_int,
>;
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
    pub index: lu_byte,
    pub kind: lu_byte,
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
    pub u: f64,
    pub s: *mut libc::c_void,
    pub i: lua_Integer,
    pub l: libc::c_long,
}
pub type F2Imod = libc::c_uint;
pub const F2Iceil: F2Imod = 2;
pub const F2Ifloor: F2Imod = 1;
pub const F2Ieq: F2Imod = 0;
pub const TM_EQ: C2RustUnnamed_9 = 5;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CallS {
    pub func: StkId,
    pub nresults: libc::c_int,
}
pub type Pfunc = Option::<unsafe extern "C" fn(*mut lua_State, *mut libc::c_void) -> ()>;
pub type ZIO = Zio;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Zio {
    pub n: size_t,
    pub p: *const libc::c_char,
    pub reader: lua_Reader,
    pub data: *mut libc::c_void,
    pub L: *mut lua_State,
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
pub const TM_LEN: C2RustUnnamed_9 = 4;
pub const TM_MODE: C2RustUnnamed_9 = 3;
pub const TM_GC: C2RustUnnamed_9 = 2;
pub const TM_NEWINDEX: C2RustUnnamed_9 = 1;
pub const TM_INDEX: C2RustUnnamed_9 = 0;
#[unsafe (no_mangle)]
pub static mut lua_ident: [libc::c_char; 129] = unsafe {
    *::core::mem::transmute::<
        &[u8; 129],
        &[libc::c_char; 129],
    >(
        b"$LuaVersion: Lua 5.4.8  Copyright (C) 1994-2025 Lua.org, PUC-Rio $$LuaAuthors: R. Ierusalimschy, L. H. de Figueiredo, W. Celes $\0",
    )
};
unsafe extern "C" fn index2value(
    mut L: *mut lua_State,
    mut index: libc::c_int,
) -> *mut TValue {
    let mut ci: *mut CallInfo = (*L).ci;
    if index > 0 as libc::c_int {
        let mut o: StkId = ((*ci).func.p).offset(index as isize);
        if o >= (*L).top.p {
            return &mut (*(*L).l_G).nilvalue
        } else {
            return &mut (*o).val
        }
    } else if !(index <= -(1000000 as libc::c_int) - 1000 as libc::c_int) {
        return &mut (*((*L).top.p).offset(index as isize)).val
    } else if index == -(1000000 as libc::c_int) - 1000 as libc::c_int {
        return &mut (*(*L).l_G).l_registry
    } else {
        index = -(1000000 as libc::c_int) - 1000 as libc::c_int - index;
        if (*(*ci).func.p).val.tt_ as libc::c_int
            == 6 as libc::c_int | (2 as libc::c_int) << 4 as libc::c_int
                | (1 as libc::c_int) << 6 as libc::c_int
        {
            let mut func: *mut CClosure = &mut (*((*(*ci).func.p).val.value_.gc
                as *mut GCUnion))
                .cl
                .c;
            return if index <= (*func).nupvalues as libc::c_int {
                &mut *((*func).upvalue)
                    .as_mut_ptr()
                    .offset((index - 1 as libc::c_int) as isize) as *mut TValue
            } else {
                &mut (*(*L).l_G).nilvalue
            };
        } else {
            return &mut (*(*L).l_G).nilvalue
        }
    };
}
#[inline]
unsafe extern "C" fn index2stack(mut L: *mut lua_State, mut index: libc::c_int) -> StkId {
    let mut ci: *mut CallInfo = (*L).ci;
    if index > 0 as libc::c_int {
        let mut o: StkId = ((*ci).func.p).offset(index as isize);
        return o;
    } else {
        return ((*L).top.p).offset(index as isize)
    };
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_checkstack(
    mut L: *mut lua_State,
    mut n: libc::c_int,
) -> libc::c_int {
    let mut res: libc::c_int = 0;
    let mut ci: *mut CallInfo = 0 as *mut CallInfo;
    ci = (*L).ci;
    if ((*L).stack_last.p).offset_from((*L).top.p) as libc::c_long > n as libc::c_long {
        res = 1 as libc::c_int;
    } else {
        res = luaD_growstack(L, n, 0 as libc::c_int);
    }
    if res != 0 && (*ci).top.p < ((*L).top.p).offset(n as isize) {
        (*ci).top.p = ((*L).top.p).offset(n as isize);
    }
    return res;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_xmove(
    mut from: *mut lua_State,
    mut to: *mut lua_State,
    mut n: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    if from == to {
        return;
    }
    (*from).top.p = ((*from).top.p).offset(-(n as isize));
    i = 0 as libc::c_int;
    while i < n {
        let mut io1: *mut TValue = &mut (*(*to).top.p).val;
        let mut io2: *const TValue = &mut (*((*from).top.p).offset(i as isize)).val;
        (*io1).value_ = (*io2).value_;
        (*io1).tt_ = (*io2).tt_;
        (*to).top.p = ((*to).top.p).offset(1);
        (*to).top.p;
        i += 1;
        i;
    }
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_atpanic(
    mut L: *mut lua_State,
    mut panicf: lua_CFunction,
) -> lua_CFunction {
    let mut old: lua_CFunction = None;
    old = (*(*L).l_G).panic;
    (*(*L).l_G).panic = panicf;
    return old;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_version(mut L: *mut lua_State) -> lua_Number {
    return 504 as libc::c_int as lua_Number;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_absindex(
    mut L: *mut lua_State,
    mut index: libc::c_int,
) -> libc::c_int {
    return if index > 0 as libc::c_int
        || index <= -(1000000 as libc::c_int) - 1000 as libc::c_int
    {
        index
    } else {
        ((*L).top.p).offset_from((*(*L).ci).func.p) as libc::c_long as libc::c_int + index
    };
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_gettop(mut L: *mut lua_State) -> libc::c_int {
    return ((*L).top.p)
        .offset_from(((*(*L).ci).func.p).offset(1 as libc::c_int as isize))
        as libc::c_long as libc::c_int;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_settop(mut L: *mut lua_State, mut index: libc::c_int) {
    let mut ci: *mut CallInfo = 0 as *mut CallInfo;
    let mut func: StkId = 0 as *mut StackValue;
    let mut newtop: StkId = 0 as *mut StackValue;
    let mut diff: ptrdiff_t = 0;
    ci = (*L).ci;
    func = (*ci).func.p;
    if index >= 0 as libc::c_int {
        diff = func
            .offset(1 as libc::c_int as isize)
            .offset(index as isize)
            .offset_from((*L).top.p) as libc::c_long;
        while diff > 0 as libc::c_int as libc::c_long {
            let fresh0 = (*L).top.p;
            (*L).top.p = ((*L).top.p).offset(1);
            (*fresh0)
                .val
                .tt_ = (0 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int)
                as lu_byte;
            diff -= 1;
            diff;
        }
    } else {
        diff = (index + 1 as libc::c_int) as ptrdiff_t;
    }
    newtop = ((*L).top.p).offset(diff as isize);
    if diff < 0 as libc::c_int as libc::c_long && (*L).tbclist.p >= newtop {
        newtop = luaF_close(L, newtop, -(1 as libc::c_int), 0 as libc::c_int);
    }
    (*L).top.p = newtop;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_closeslot(mut L: *mut lua_State, mut index: libc::c_int) {
    let mut level: StkId = 0 as *mut StackValue;
    level = index2stack(L, index);
    level = luaF_close(L, level, -(1 as libc::c_int), 0 as libc::c_int);
    (*level)
        .val
        .tt_ = (0 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int) as lu_byte;
}
#[inline]
unsafe extern "C" fn reverse(mut L: *mut lua_State, mut from: StkId, mut to: StkId) {
    while from < to {
        let mut temp: TValue = TValue {
            value_: Value { gc: 0 as *mut GCObject },
            tt_: 0,
        };
        let mut io1: *mut TValue = &mut temp;
        let mut io2: *const TValue = &mut (*from).val;
        (*io1).value_ = (*io2).value_;
        (*io1).tt_ = (*io2).tt_;
        let mut io1_0: *mut TValue = &mut (*from).val;
        let mut io2_0: *const TValue = &mut (*to).val;
        (*io1_0).value_ = (*io2_0).value_;
        (*io1_0).tt_ = (*io2_0).tt_;
        let mut io1_1: *mut TValue = &mut (*to).val;
        let mut io2_1: *const TValue = &mut temp;
        (*io1_1).value_ = (*io2_1).value_;
        (*io1_1).tt_ = (*io2_1).tt_;
        from = from.offset(1);
        from;
        to = to.offset(-1);
        to;
    }
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_rotate(
    mut L: *mut lua_State,
    mut index: libc::c_int,
    mut n: libc::c_int,
) {
    let mut p: StkId = 0 as *mut StackValue;
    let mut t: StkId = 0 as *mut StackValue;
    let mut m: StkId = 0 as *mut StackValue;
    t = ((*L).top.p).offset(-(1 as libc::c_int as isize));
    p = index2stack(L, index);
    m = if n >= 0 as libc::c_int {
        t.offset(-(n as isize))
    } else {
        p.offset(-(n as isize)).offset(-(1 as libc::c_int as isize))
    };
    reverse(L, p, m);
    reverse(L, m.offset(1 as libc::c_int as isize), t);
    reverse(L, p, t);
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_copy(
    mut L: *mut lua_State,
    mut fromidx: libc::c_int,
    mut toidx: libc::c_int,
) {
    let mut fr: *mut TValue = 0 as *mut TValue;
    let mut to: *mut TValue = 0 as *mut TValue;
    fr = index2value(L, fromidx);
    to = index2value(L, toidx);
    let mut io1: *mut TValue = to;
    let mut io2: *const TValue = fr;
    (*io1).value_ = (*io2).value_;
    (*io1).tt_ = (*io2).tt_;
    if toidx < -(1000000 as libc::c_int) - 1000 as libc::c_int {
        if (*fr).tt_ as libc::c_int & (1 as libc::c_int) << 6 as libc::c_int != 0 {
            if (*((*(*(*L).ci).func.p).val.value_.gc as *mut GCUnion)).cl.c.marked
                as libc::c_int & (1 as libc::c_int) << 5 as libc::c_int != 0
                && (*(*fr).value_.gc).marked as libc::c_int
                    & ((1 as libc::c_int) << 3 as libc::c_int
                        | (1 as libc::c_int) << 4 as libc::c_int) != 0
            {
                luaC_barrier_(
                    L,
                    &mut (*(&mut (*((*(*(*L).ci).func.p).val.value_.gc as *mut GCUnion))
                        .cl
                        .c as *mut CClosure as *mut GCUnion))
                        .gc,
                    &mut (*((*fr).value_.gc as *mut GCUnion)).gc,
                );
            } else {};
        } else {};
    }
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_pushvalue(mut L: *mut lua_State, mut index: libc::c_int) {
    let mut io1: *mut TValue = &mut (*(*L).top.p).val;
    let mut io2: *const TValue = index2value(L, index);
    (*io1).value_ = (*io2).value_;
    (*io1).tt_ = (*io2).tt_;
    (*L).top.p = ((*L).top.p).offset(1);
    (*L).top.p;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_type(
    mut L: *mut lua_State,
    mut index: libc::c_int,
) -> libc::c_int {
    let mut o: *const TValue = index2value(L, index);
    return if !((*o).tt_ as libc::c_int & 0xf as libc::c_int == 0 as libc::c_int)
        || o != &mut (*(*L).l_G).nilvalue as *mut TValue as *const TValue
    {
        (*o).tt_ as libc::c_int & 0xf as libc::c_int
    } else {
        -(1 as libc::c_int)
    };
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_typename(
    mut L: *mut lua_State,
    mut t: libc::c_int,
) -> *const libc::c_char {
    return luaT_typenames_[(t + 1 as libc::c_int) as usize];
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_iscfunction(
    mut L: *mut lua_State,
    mut index: libc::c_int,
) -> libc::c_int {
    let mut o: *const TValue = index2value(L, index);
    return ((*o).tt_ as libc::c_int
        == 6 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
        || (*o).tt_ as libc::c_int
            == 6 as libc::c_int | (2 as libc::c_int) << 4 as libc::c_int
                | (1 as libc::c_int) << 6 as libc::c_int) as libc::c_int;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_isinteger(
    mut L: *mut lua_State,
    mut index: libc::c_int,
) -> libc::c_int {
    let mut o: *const TValue = index2value(L, index);
    return ((*o).tt_ as libc::c_int
        == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int) as libc::c_int;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_isnumber(
    mut L: *mut lua_State,
    mut index: libc::c_int,
) -> libc::c_int {
    let mut n: lua_Number = 0.;
    let mut o: *const TValue = index2value(L, index);
    return if (*o).tt_ as libc::c_int
        == 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
    {
        n = (*o).value_.n;
        1 as libc::c_int
    } else {
        luaV_tonumber_(o, &mut n)
    };
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_isstring(
    mut L: *mut lua_State,
    mut index: libc::c_int,
) -> libc::c_int {
    let mut o: *const TValue = index2value(L, index);
    return ((*o).tt_ as libc::c_int & 0xf as libc::c_int == 4 as libc::c_int
        || (*o).tt_ as libc::c_int & 0xf as libc::c_int == 3 as libc::c_int)
        as libc::c_int;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_isuserdata(
    mut L: *mut lua_State,
    mut index: libc::c_int,
) -> libc::c_int {
    let mut o: *const TValue = index2value(L, index);
    return ((*o).tt_ as libc::c_int
        == 7 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
            | (1 as libc::c_int) << 6 as libc::c_int
        || (*o).tt_ as libc::c_int
            == 2 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int) as libc::c_int;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_rawequal(
    mut L: *mut lua_State,
    mut index1: libc::c_int,
    mut index2: libc::c_int,
) -> libc::c_int {
    let mut o1: *const TValue = index2value(L, index1);
    let mut o2: *const TValue = index2value(L, index2);
    return if (!((*o1).tt_ as libc::c_int & 0xf as libc::c_int == 0 as libc::c_int)
        || o1 != &mut (*(*L).l_G).nilvalue as *mut TValue as *const TValue)
        && (!((*o2).tt_ as libc::c_int & 0xf as libc::c_int == 0 as libc::c_int)
            || o2 != &mut (*(*L).l_G).nilvalue as *mut TValue as *const TValue)
    {
        luaV_equalobj(0 as *mut lua_State, o1, o2)
    } else {
        0 as libc::c_int
    };
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_arith(mut L: *mut lua_State, mut op: libc::c_int) {
    if !(op != 12 as libc::c_int && op != 13 as libc::c_int) {
        let mut io1: *mut TValue = &mut (*(*L).top.p).val;
        let mut io2: *const TValue = &mut (*((*L).top.p)
            .offset(-(1 as libc::c_int as isize)))
            .val;
        (*io1).value_ = (*io2).value_;
        (*io1).tt_ = (*io2).tt_;
        (*L).top.p = ((*L).top.p).offset(1);
        (*L).top.p;
    }
    luaO_arith(
        L,
        op,
        &mut (*((*L).top.p).offset(-(2 as libc::c_int as isize))).val,
        &mut (*((*L).top.p).offset(-(1 as libc::c_int as isize))).val,
        ((*L).top.p).offset(-(2 as libc::c_int as isize)),
    );
    (*L).top.p = ((*L).top.p).offset(-1);
    (*L).top.p;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_compare(
    mut L: *mut lua_State,
    mut index1: libc::c_int,
    mut index2: libc::c_int,
    mut op: libc::c_int,
) -> libc::c_int {
    let mut o1: *const TValue = 0 as *const TValue;
    let mut o2: *const TValue = 0 as *const TValue;
    let mut i: libc::c_int = 0 as libc::c_int;
    o1 = index2value(L, index1);
    o2 = index2value(L, index2);
    if (!((*o1).tt_ as libc::c_int & 0xf as libc::c_int == 0 as libc::c_int)
        || o1 != &mut (*(*L).l_G).nilvalue as *mut TValue as *const TValue)
        && (!((*o2).tt_ as libc::c_int & 0xf as libc::c_int == 0 as libc::c_int)
            || o2 != &mut (*(*L).l_G).nilvalue as *mut TValue as *const TValue)
    {
        match op {
            0 => {
                i = luaV_equalobj(L, o1, o2);
            }
            1 => {
                i = luaV_lessthan(L, o1, o2);
            }
            2 => {
                i = luaV_lessequal(L, o1, o2);
            }
            _ => {}
        }
    }
    return i;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_stringtonumber(
    mut L: *mut lua_State,
    mut s: *const libc::c_char,
) -> size_t {
    let mut sz: size_t = luaO_str2num(s, &mut (*(*L).top.p).val);
    if sz != 0 as libc::c_int as libc::c_ulong {
        (*L).top.p = ((*L).top.p).offset(1);
        (*L).top.p;
    }
    return sz;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_tonumberx(
    mut L: *mut lua_State,
    mut index: libc::c_int,
    mut pisnum: *mut libc::c_int,
) -> lua_Number {
    let mut n: lua_Number = 0 as libc::c_int as lua_Number;
    let mut o: *const TValue = index2value(L, index);
    let mut isnum: libc::c_int = if (*o).tt_ as libc::c_int
        == 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
    {
        n = (*o).value_.n;
        1 as libc::c_int
    } else {
        luaV_tonumber_(o, &mut n)
    };
    if !pisnum.is_null() {
        *pisnum = isnum;
    }
    return n;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_tointegerx(
    mut L: *mut lua_State,
    mut index: libc::c_int,
    mut pisnum: *mut libc::c_int,
) -> lua_Integer {
    let mut res: lua_Integer = 0 as libc::c_int as lua_Integer;
    let mut o: *const TValue = index2value(L, index);
    let mut isnum: libc::c_int = if (((*o).tt_ as libc::c_int
        == 3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int) as libc::c_int
        != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
    {
        res = (*o).value_.i;
        1 as libc::c_int
    } else {
        luaV_tointeger(o, &mut res, F2Ieq)
    };
    if !pisnum.is_null() {
        *pisnum = isnum;
    }
    return res;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_toboolean(
    mut L: *mut lua_State,
    mut index: libc::c_int,
) -> libc::c_int {
    let mut o: *const TValue = index2value(L, index);
    return !((*o).tt_ as libc::c_int
        == 1 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        || (*o).tt_ as libc::c_int & 0xf as libc::c_int == 0 as libc::c_int)
        as libc::c_int;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_tolstring(
    mut L: *mut lua_State,
    mut index: libc::c_int,
    mut len: *mut size_t,
) -> *const libc::c_char {
    let mut o: *mut TValue = 0 as *mut TValue;
    o = index2value(L, index);
    if !((*o).tt_ as libc::c_int & 0xf as libc::c_int == 4 as libc::c_int) {
        if !((*o).tt_ as libc::c_int & 0xf as libc::c_int == 3 as libc::c_int) {
            if !len.is_null() {
                *len = 0 as libc::c_int as size_t;
            }
            return 0 as *const libc::c_char;
        }
        luaO_tostring(L, o);
        if (*(*L).l_G).GCdebt > 0 as libc::c_int as libc::c_long {
            luaC_step(L);
        }
        o = index2value(L, index);
    }
    if !len.is_null() {
        *len = if (*((*o).value_.gc as *mut GCUnion)).ts.shrlen as libc::c_int
            != 0xff as libc::c_int
        {
            (*((*o).value_.gc as *mut GCUnion)).ts.shrlen as libc::c_ulong
        } else {
            (*((*o).value_.gc as *mut GCUnion)).ts.u.lnglen
        };
    }
    return ((*((*o).value_.gc as *mut GCUnion)).ts.contents).as_mut_ptr();
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_rawlen(
    mut L: *mut lua_State,
    mut index: libc::c_int,
) -> lua_Unsigned {
    let mut o: *const TValue = index2value(L, index);
    match (*o).tt_ as libc::c_int & 0x3f as libc::c_int {
        4 => return (*((*o).value_.gc as *mut GCUnion)).ts.shrlen as lua_Unsigned,
        20 => return (*((*o).value_.gc as *mut GCUnion)).ts.u.lnglen as lua_Unsigned,
        7 => return (*((*o).value_.gc as *mut GCUnion)).u.len as lua_Unsigned,
        5 => return luaH_getn(&mut (*((*o).value_.gc as *mut GCUnion)).h),
        _ => return 0 as libc::c_int as lua_Unsigned,
    };
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_tocfunction(
    mut L: *mut lua_State,
    mut index: libc::c_int,
) -> lua_CFunction {
    let mut o: *const TValue = index2value(L, index);
    if (*o).tt_ as libc::c_int
        == 6 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
    {
        return (*o).value_.f
    } else if (*o).tt_ as libc::c_int
        == 6 as libc::c_int | (2 as libc::c_int) << 4 as libc::c_int
            | (1 as libc::c_int) << 6 as libc::c_int
    {
        return (*((*o).value_.gc as *mut GCUnion)).cl.c.f
    } else {
        return None
    };
}
#[inline]
unsafe extern "C" fn touserdata(mut o: *const TValue) -> *mut libc::c_void {
    match (*o).tt_ as libc::c_int & 0xf as libc::c_int {
        7 => {
            return (&mut (*((*o).value_.gc as *mut GCUnion)).u as *mut Udata
                as *mut libc::c_char)
                .offset(
                    (if (*((*o).value_.gc as *mut GCUnion)).u.nuvalue as libc::c_int
                        == 0 as libc::c_int
                    {
                        32 as libc::c_ulong
                    } else {
                        (40 as libc::c_ulong)
                            .wrapping_add(
                                (::core::mem::size_of::<UValue>() as libc::c_ulong)
                                    .wrapping_mul(
                                        (*((*o).value_.gc as *mut GCUnion)).u.nuvalue
                                            as libc::c_ulong,
                                    ),
                            )
                    }) as isize,
                ) as *mut libc::c_void;
        }
        2 => return (*o).value_.p,
        _ => return 0 as *mut libc::c_void,
    };
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_touserdata(
    mut L: *mut lua_State,
    mut index: libc::c_int,
) -> *mut libc::c_void {
    let mut o: *const TValue = index2value(L, index);
    return touserdata(o);
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_tothread(
    mut L: *mut lua_State,
    mut index: libc::c_int,
) -> *mut lua_State {
    let mut o: *const TValue = index2value(L, index);
    return if !((*o).tt_ as libc::c_int
        == 8 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
            | (1 as libc::c_int) << 6 as libc::c_int)
    {
        0 as *mut lua_State
    } else {
        &mut (*((*o).value_.gc as *mut GCUnion)).th
    };
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_topointer(
    mut L: *mut lua_State,
    mut index: libc::c_int,
) -> *const libc::c_void {
    let mut o: *const TValue = index2value(L, index);
    match (*o).tt_ as libc::c_int & 0x3f as libc::c_int {
        22 => {
            return ::core::mem::transmute::<lua_CFunction, size_t>((*o).value_.f)
                as *mut libc::c_void;
        }
        7 | 2 => return touserdata(o),
        _ => {
            if (*o).tt_ as libc::c_int & (1 as libc::c_int) << 6 as libc::c_int != 0 {
                return (*o).value_.gc as *const libc::c_void
            } else {
                return 0 as *const libc::c_void
            }
        }
    };
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_pushnil(mut L: *mut lua_State) {
    (*(*L).top.p)
        .val
        .tt_ = (0 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int) as lu_byte;
    (*L).top.p = ((*L).top.p).offset(1);
    (*L).top.p;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_pushnumber(mut L: *mut lua_State, mut n: lua_Number) {
    let mut io: *mut TValue = &mut (*(*L).top.p).val;
    (*io).value_.n = n;
    (*io).tt_ = (3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int) as lu_byte;
    (*L).top.p = ((*L).top.p).offset(1);
    (*L).top.p;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_pushinteger(mut L: *mut lua_State, mut n: lua_Integer) {
    let mut io: *mut TValue = &mut (*(*L).top.p).val;
    (*io).value_.i = n;
    (*io).tt_ = (3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int) as lu_byte;
    (*L).top.p = ((*L).top.p).offset(1);
    (*L).top.p;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_pushlstring(
    mut L: *mut lua_State,
    mut s: *const libc::c_char,
    mut len: size_t,
) -> *const libc::c_char {
    let mut ts: *mut TString = 0 as *mut TString;
    ts = if len == 0 as libc::c_int as libc::c_ulong {
        luaS_new(L, b"\0" as *const u8 as *const libc::c_char)
    } else {
        luaS_newlstr(L, s, len)
    };
    let mut io: *mut TValue = &mut (*(*L).top.p).val;
    let mut x_: *mut TString = ts;
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
    (*io)
        .tt_ = ((*x_).tt as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int)
        as lu_byte;
    (*L).top.p = ((*L).top.p).offset(1);
    (*L).top.p;
    if (*(*L).l_G).GCdebt > 0 as libc::c_int as libc::c_long {
        luaC_step(L);
    }
    return ((*ts).contents).as_mut_ptr();
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_pushstring(
    mut L: *mut lua_State,
    mut s: *const libc::c_char,
) -> *const libc::c_char {
    if s.is_null() {
        (*(*L).top.p)
            .val
            .tt_ = (0 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int)
            as lu_byte;
    } else {
        let mut ts: *mut TString = 0 as *mut TString;
        ts = luaS_new(L, s);
        let mut io: *mut TValue = &mut (*(*L).top.p).val;
        let mut x_: *mut TString = ts;
        (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
        (*io)
            .tt_ = ((*x_).tt as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int)
            as lu_byte;
        s = ((*ts).contents).as_mut_ptr();
    }
    (*L).top.p = ((*L).top.p).offset(1);
    (*L).top.p;
    if (*(*L).l_G).GCdebt > 0 as libc::c_int as libc::c_long {
        luaC_step(L);
    }
    return s;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_pushvfstring(
    mut L: *mut lua_State,
    mut fmt: *const libc::c_char,
    mut argp: ::core::ffi::VaList,
) -> *const libc::c_char {
    let mut ret: *const libc::c_char = 0 as *const libc::c_char;
    ret = luaO_pushvfstring(L, fmt, argp.as_va_list());
    if (*(*L).l_G).GCdebt > 0 as libc::c_int as libc::c_long {
        luaC_step(L);
    }
    return ret;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_pushfstring(
    mut L: *mut lua_State,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> *const libc::c_char {
    let mut ret: *const libc::c_char = 0 as *const libc::c_char;
    let mut argp: ::core::ffi::VaListImpl;
    argp = args.clone();
    ret = luaO_pushvfstring(L, fmt, argp.as_va_list());
    if (*(*L).l_G).GCdebt > 0 as libc::c_int as libc::c_long {
        luaC_step(L);
    }
    return ret;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_pushcclosure(
    mut L: *mut lua_State,
    mut fn_0: lua_CFunction,
    mut n: libc::c_int,
) {
    if n == 0 as libc::c_int {
        let mut io: *mut TValue = &mut (*(*L).top.p).val;
        (*io).value_.f = fn_0;
        (*io)
            .tt_ = (6 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int)
            as lu_byte;
        (*L).top.p = ((*L).top.p).offset(1);
        (*L).top.p;
    } else {
        let mut cl: *mut CClosure = 0 as *mut CClosure;
        cl = luaF_newCclosure(L, n);
        (*cl).f = fn_0;
        (*L).top.p = ((*L).top.p).offset(-(n as isize));
        loop {
            let fresh1 = n;
            n = n - 1;
            if !(fresh1 != 0) {
                break;
            }
            let mut io1: *mut TValue = &mut *((*cl).upvalue)
                .as_mut_ptr()
                .offset(n as isize) as *mut TValue;
            let mut io2: *const TValue = &mut (*((*L).top.p).offset(n as isize)).val;
            (*io1).value_ = (*io2).value_;
            (*io1).tt_ = (*io2).tt_;
        }
        let mut io_0: *mut TValue = &mut (*(*L).top.p).val;
        let mut x_: *mut CClosure = cl;
        (*io_0).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
        (*io_0)
            .tt_ = (6 as libc::c_int | (2 as libc::c_int) << 4 as libc::c_int
            | (1 as libc::c_int) << 6 as libc::c_int) as lu_byte;
        (*L).top.p = ((*L).top.p).offset(1);
        (*L).top.p;
        if (*(*L).l_G).GCdebt > 0 as libc::c_int as libc::c_long {
            luaC_step(L);
        }
    };
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_pushboolean(mut L: *mut lua_State, mut b: libc::c_int) {
    if b != 0 {
        (*(*L).top.p)
            .val
            .tt_ = (1 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int)
            as lu_byte;
    } else {
        (*(*L).top.p)
            .val
            .tt_ = (1 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int)
            as lu_byte;
    }
    (*L).top.p = ((*L).top.p).offset(1);
    (*L).top.p;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_pushlightuserdata(
    mut L: *mut lua_State,
    mut p: *mut libc::c_void,
) {
    let mut io: *mut TValue = &mut (*(*L).top.p).val;
    (*io).value_.p = p;
    (*io).tt_ = (2 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int) as lu_byte;
    (*L).top.p = ((*L).top.p).offset(1);
    (*L).top.p;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_pushthread(mut L: *mut lua_State) -> libc::c_int {
    let mut io: *mut TValue = &mut (*(*L).top.p).val;
    let mut x_: *mut lua_State = L;
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
    (*io)
        .tt_ = (8 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 6 as libc::c_int) as lu_byte;
    (*L).top.p = ((*L).top.p).offset(1);
    (*L).top.p;
    return ((*(*L).l_G).mainthread == L) as libc::c_int;
}
#[inline]
unsafe extern "C" fn auxgetstr(
    mut L: *mut lua_State,
    mut t: *const TValue,
    mut k: *const libc::c_char,
) -> libc::c_int {
    let mut slot: *const TValue = 0 as *const TValue;
    let mut str: *mut TString = luaS_new(L, k);
    if if !((*t).tt_ as libc::c_int
        == 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
            | (1 as libc::c_int) << 6 as libc::c_int)
    {
        slot = 0 as *const TValue;
        0 as libc::c_int
    } else {
        slot = luaH_getstr(&mut (*((*t).value_.gc as *mut GCUnion)).h, str);
        !((*slot).tt_ as libc::c_int & 0xf as libc::c_int == 0 as libc::c_int)
            as libc::c_int
    } != 0
    {
        let mut io1: *mut TValue = &mut (*(*L).top.p).val;
        let mut io2: *const TValue = slot;
        (*io1).value_ = (*io2).value_;
        (*io1).tt_ = (*io2).tt_;
        (*L).top.p = ((*L).top.p).offset(1);
        (*L).top.p;
    } else {
        let mut io: *mut TValue = &mut (*(*L).top.p).val;
        let mut x_: *mut TString = str;
        (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
        (*io)
            .tt_ = ((*x_).tt as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int)
            as lu_byte;
        (*L).top.p = ((*L).top.p).offset(1);
        (*L).top.p;
        luaV_finishget(
            L,
            t,
            &mut (*((*L).top.p).offset(-(1 as libc::c_int as isize))).val,
            ((*L).top.p).offset(-(1 as libc::c_int as isize)),
            slot,
        );
    }
    return (*((*L).top.p).offset(-(1 as libc::c_int as isize))).val.tt_ as libc::c_int
        & 0xf as libc::c_int;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_getglobal(
    mut L: *mut lua_State,
    mut name: *const libc::c_char,
) -> libc::c_int {
    let mut G: *const TValue = 0 as *const TValue;
    G = &mut *((*((*(*L).l_G).l_registry.value_.gc as *mut GCUnion)).h.array)
        .offset((2 as libc::c_int - 1 as libc::c_int) as isize) as *mut TValue;
    return auxgetstr(L, G, name);
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_gettable(
    mut L: *mut lua_State,
    mut index: libc::c_int,
) -> libc::c_int {
    let mut slot: *const TValue = 0 as *const TValue;
    let mut t: *mut TValue = 0 as *mut TValue;
    t = index2value(L, index);
    if if !((*t).tt_ as libc::c_int
        == 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
            | (1 as libc::c_int) << 6 as libc::c_int)
    {
        slot = 0 as *const TValue;
        0 as libc::c_int
    } else {
        slot = luaH_get(
            &mut (*((*t).value_.gc as *mut GCUnion)).h,
            &mut (*((*L).top.p).offset(-(1 as libc::c_int as isize))).val,
        );
        !((*slot).tt_ as libc::c_int & 0xf as libc::c_int == 0 as libc::c_int)
            as libc::c_int
    } != 0
    {
        let mut io1: *mut TValue = &mut (*((*L).top.p)
            .offset(-(1 as libc::c_int as isize)))
            .val;
        let mut io2: *const TValue = slot;
        (*io1).value_ = (*io2).value_;
        (*io1).tt_ = (*io2).tt_;
    } else {
        luaV_finishget(
            L,
            t,
            &mut (*((*L).top.p).offset(-(1 as libc::c_int as isize))).val,
            ((*L).top.p).offset(-(1 as libc::c_int as isize)),
            slot,
        );
    }
    return (*((*L).top.p).offset(-(1 as libc::c_int as isize))).val.tt_ as libc::c_int
        & 0xf as libc::c_int;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_getfield(
    mut L: *mut lua_State,
    mut index: libc::c_int,
    mut k: *const libc::c_char,
) -> libc::c_int {
    return auxgetstr(L, index2value(L, index), k);
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_geti(
    mut L: *mut lua_State,
    mut index: libc::c_int,
    mut n: lua_Integer,
) -> libc::c_int {
    let mut t: *mut TValue = 0 as *mut TValue;
    let mut slot: *const TValue = 0 as *const TValue;
    t = index2value(L, index);
    if if !((*t).tt_ as libc::c_int
        == 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
            | (1 as libc::c_int) << 6 as libc::c_int)
    {
        slot = 0 as *const TValue;
        0 as libc::c_int
    } else {
        slot = (if (n as lua_Unsigned)
            .wrapping_sub(1 as libc::c_uint as libc::c_ulonglong)
            < (*((*t).value_.gc as *mut GCUnion)).h.alimit as libc::c_ulonglong
        {
            &mut *((*((*t).value_.gc as *mut GCUnion)).h.array)
                .offset((n - 1 as libc::c_int as i64) as isize)
                as *mut TValue as *const TValue
        } else {
            luaH_getint(&mut (*((*t).value_.gc as *mut GCUnion)).h, n)
        });
        !((*slot).tt_ as libc::c_int & 0xf as libc::c_int == 0 as libc::c_int)
            as libc::c_int
    } != 0
    {
        let mut io1: *mut TValue = &mut (*(*L).top.p).val;
        let mut io2: *const TValue = slot;
        (*io1).value_ = (*io2).value_;
        (*io1).tt_ = (*io2).tt_;
    } else {
        let mut aux: TValue = TValue {
            value_: Value { gc: 0 as *mut GCObject },
            tt_: 0,
        };
        let mut io: *mut TValue = &mut aux;
        (*io).value_.i = n;
        (*io)
            .tt_ = (3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int)
            as lu_byte;
        luaV_finishget(L, t, &mut aux, (*L).top.p, slot);
    }
    (*L).top.p = ((*L).top.p).offset(1);
    (*L).top.p;
    return (*((*L).top.p).offset(-(1 as libc::c_int as isize))).val.tt_ as libc::c_int
        & 0xf as libc::c_int;
}
#[inline]
unsafe extern "C" fn finishrawget(
    mut L: *mut lua_State,
    mut val: *const TValue,
) -> libc::c_int {
    if (*val).tt_ as libc::c_int & 0xf as libc::c_int == 0 as libc::c_int {
        (*(*L).top.p)
            .val
            .tt_ = (0 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int)
            as lu_byte;
    } else {
        let mut io1: *mut TValue = &mut (*(*L).top.p).val;
        let mut io2: *const TValue = val;
        (*io1).value_ = (*io2).value_;
        (*io1).tt_ = (*io2).tt_;
    }
    (*L).top.p = ((*L).top.p).offset(1);
    (*L).top.p;
    return (*((*L).top.p).offset(-(1 as libc::c_int as isize))).val.tt_ as libc::c_int
        & 0xf as libc::c_int;
}
unsafe extern "C" fn gettable(
    mut L: *mut lua_State,
    mut index: libc::c_int,
) -> *mut Table {
    let mut t: *mut TValue = index2value(L, index);
    return &mut (*((*t).value_.gc as *mut GCUnion)).h;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_rawget(
    mut L: *mut lua_State,
    mut index: libc::c_int,
) -> libc::c_int {
    let mut t: *mut Table = 0 as *mut Table;
    let mut val: *const TValue = 0 as *const TValue;
    t = gettable(L, index);
    val = luaH_get(t, &mut (*((*L).top.p).offset(-(1 as libc::c_int as isize))).val);
    (*L).top.p = ((*L).top.p).offset(-1);
    (*L).top.p;
    return finishrawget(L, val);
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_rawgeti(
    mut L: *mut lua_State,
    mut index: libc::c_int,
    mut n: lua_Integer,
) -> libc::c_int {
    let mut t: *mut Table = 0 as *mut Table;
    t = gettable(L, index);
    return finishrawget(L, luaH_getint(t, n));
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_rawgetp(
    mut L: *mut lua_State,
    mut index: libc::c_int,
    mut p: *const libc::c_void,
) -> libc::c_int {
    let mut t: *mut Table = 0 as *mut Table;
    let mut k: TValue = TValue {
        value_: Value { gc: 0 as *mut GCObject },
        tt_: 0,
    };
    t = gettable(L, index);
    let mut io: *mut TValue = &mut k;
    (*io).value_.p = p as *mut libc::c_void;
    (*io).tt_ = (2 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int) as lu_byte;
    return finishrawget(L, luaH_get(t, &mut k));
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_createtable(
    mut L: *mut lua_State,
    mut narray: libc::c_int,
    mut nrec: libc::c_int,
) {
    let mut t: *mut Table = 0 as *mut Table;
    t = luaH_new(L);
    let mut io: *mut TValue = &mut (*(*L).top.p).val;
    let mut x_: *mut Table = t;
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
    (*io)
        .tt_ = (5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 6 as libc::c_int) as lu_byte;
    (*L).top.p = ((*L).top.p).offset(1);
    (*L).top.p;
    if narray > 0 as libc::c_int || nrec > 0 as libc::c_int {
        luaH_resize(L, t, narray as libc::c_uint, nrec as libc::c_uint);
    }
    if (*(*L).l_G).GCdebt > 0 as libc::c_int as libc::c_long {
        luaC_step(L);
    }
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_getmetatable(
    mut L: *mut lua_State,
    mut objindex: libc::c_int,
) -> libc::c_int {
    let mut obj: *const TValue = 0 as *const TValue;
    let mut mt: *mut Table = 0 as *mut Table;
    let mut res: libc::c_int = 0 as libc::c_int;
    obj = index2value(L, objindex);
    match (*obj).tt_ as libc::c_int & 0xf as libc::c_int {
        5 => {
            mt = (*((*obj).value_.gc as *mut GCUnion)).h.metatable;
        }
        7 => {
            mt = (*((*obj).value_.gc as *mut GCUnion)).u.metatable;
        }
        _ => {
            mt = (*(*L).l_G)
                .mt[((*obj).tt_ as libc::c_int & 0xf as libc::c_int) as usize];
        }
    }
    if !mt.is_null() {
        let mut io: *mut TValue = &mut (*(*L).top.p).val;
        let mut x_: *mut Table = mt;
        (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
        (*io)
            .tt_ = (5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
            | (1 as libc::c_int) << 6 as libc::c_int) as lu_byte;
        (*L).top.p = ((*L).top.p).offset(1);
        (*L).top.p;
        res = 1 as libc::c_int;
    }
    return res;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_getiuservalue(
    mut L: *mut lua_State,
    mut index: libc::c_int,
    mut n: libc::c_int,
) -> libc::c_int {
    let mut o: *mut TValue = 0 as *mut TValue;
    let mut t: libc::c_int = 0;
    o = index2value(L, index);
    if n <= 0 as libc::c_int
        || n > (*((*o).value_.gc as *mut GCUnion)).u.nuvalue as libc::c_int
    {
        (*(*L).top.p)
            .val
            .tt_ = (0 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int)
            as lu_byte;
        t = -(1 as libc::c_int);
    } else {
        let mut io1: *mut TValue = &mut (*(*L).top.p).val;
        let mut io2: *const TValue = &mut (*((*((*o).value_.gc as *mut GCUnion)).u.uv)
            .as_mut_ptr()
            .offset((n - 1 as libc::c_int) as isize))
            .uv;
        (*io1).value_ = (*io2).value_;
        (*io1).tt_ = (*io2).tt_;
        t = (*(*L).top.p).val.tt_ as libc::c_int & 0xf as libc::c_int;
    }
    (*L).top.p = ((*L).top.p).offset(1);
    (*L).top.p;
    return t;
}
unsafe extern "C" fn auxsetstr(
    mut L: *mut lua_State,
    mut t: *const TValue,
    mut k: *const libc::c_char,
) {
    let mut slot: *const TValue = 0 as *const TValue;
    let mut str: *mut TString = luaS_new(L, k);
    if if !((*t).tt_ as libc::c_int
        == 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
            | (1 as libc::c_int) << 6 as libc::c_int)
    {
        slot = 0 as *const TValue;
        0 as libc::c_int
    } else {
        slot = luaH_getstr(&mut (*((*t).value_.gc as *mut GCUnion)).h, str);
        !((*slot).tt_ as libc::c_int & 0xf as libc::c_int == 0 as libc::c_int)
            as libc::c_int
    } != 0
    {
        let mut io1: *mut TValue = slot as *mut TValue;
        let mut io2: *const TValue = &mut (*((*L).top.p)
            .offset(-(1 as libc::c_int as isize)))
            .val;
        (*io1).value_ = (*io2).value_;
        (*io1).tt_ = (*io2).tt_;
        if (*((*L).top.p).offset(-(1 as libc::c_int as isize))).val.tt_ as libc::c_int
            & (1 as libc::c_int) << 6 as libc::c_int != 0
        {
            if (*(*t).value_.gc).marked as libc::c_int
                & (1 as libc::c_int) << 5 as libc::c_int != 0
                && (*(*((*L).top.p).offset(-(1 as libc::c_int as isize))).val.value_.gc)
                    .marked as libc::c_int
                    & ((1 as libc::c_int) << 3 as libc::c_int
                        | (1 as libc::c_int) << 4 as libc::c_int) != 0
            {
                luaC_barrierback_(L, (*t).value_.gc);
            } else {};
        } else {};
        (*L).top.p = ((*L).top.p).offset(-1);
        (*L).top.p;
    } else {
        let mut io: *mut TValue = &mut (*(*L).top.p).val;
        let mut x_: *mut TString = str;
        (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
        (*io)
            .tt_ = ((*x_).tt as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int)
            as lu_byte;
        (*L).top.p = ((*L).top.p).offset(1);
        (*L).top.p;
        luaV_finishset(
            L,
            t,
            &mut (*((*L).top.p).offset(-(1 as libc::c_int as isize))).val,
            &mut (*((*L).top.p).offset(-(2 as libc::c_int as isize))).val,
            slot,
        );
        (*L).top.p = ((*L).top.p).offset(-(2 as libc::c_int as isize));
    };
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_setglobal(
    mut L: *mut lua_State,
    mut name: *const libc::c_char,
) {
    let mut G: *const TValue = 0 as *const TValue;
    G = &mut *((*((*(*L).l_G).l_registry.value_.gc as *mut GCUnion)).h.array)
        .offset((2 as libc::c_int - 1 as libc::c_int) as isize) as *mut TValue;
    auxsetstr(L, G, name);
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_settable(mut L: *mut lua_State, mut index: libc::c_int) {
    let mut t: *mut TValue = 0 as *mut TValue;
    let mut slot: *const TValue = 0 as *const TValue;
    t = index2value(L, index);
    if if !((*t).tt_ as libc::c_int
        == 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
            | (1 as libc::c_int) << 6 as libc::c_int)
    {
        slot = 0 as *const TValue;
        0 as libc::c_int
    } else {
        slot = luaH_get(
            &mut (*((*t).value_.gc as *mut GCUnion)).h,
            &mut (*((*L).top.p).offset(-(2 as libc::c_int as isize))).val,
        );
        !((*slot).tt_ as libc::c_int & 0xf as libc::c_int == 0 as libc::c_int)
            as libc::c_int
    } != 0
    {
        let mut io1: *mut TValue = slot as *mut TValue;
        let mut io2: *const TValue = &mut (*((*L).top.p)
            .offset(-(1 as libc::c_int as isize)))
            .val;
        (*io1).value_ = (*io2).value_;
        (*io1).tt_ = (*io2).tt_;
        if (*((*L).top.p).offset(-(1 as libc::c_int as isize))).val.tt_ as libc::c_int
            & (1 as libc::c_int) << 6 as libc::c_int != 0
        {
            if (*(*t).value_.gc).marked as libc::c_int
                & (1 as libc::c_int) << 5 as libc::c_int != 0
                && (*(*((*L).top.p).offset(-(1 as libc::c_int as isize))).val.value_.gc)
                    .marked as libc::c_int
                    & ((1 as libc::c_int) << 3 as libc::c_int
                        | (1 as libc::c_int) << 4 as libc::c_int) != 0
            {
                luaC_barrierback_(L, (*t).value_.gc);
            } else {};
        } else {};
    } else {
        luaV_finishset(
            L,
            t,
            &mut (*((*L).top.p).offset(-(2 as libc::c_int as isize))).val,
            &mut (*((*L).top.p).offset(-(1 as libc::c_int as isize))).val,
            slot,
        );
    }
    (*L).top.p = ((*L).top.p).offset(-(2 as libc::c_int as isize));
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_setfield(
    mut L: *mut lua_State,
    mut index: libc::c_int,
    mut k: *const libc::c_char,
) {
    auxsetstr(L, index2value(L, index), k);
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_seti(
    mut L: *mut lua_State,
    mut index: libc::c_int,
    mut n: lua_Integer,
) {
    let mut t: *mut TValue = 0 as *mut TValue;
    let mut slot: *const TValue = 0 as *const TValue;
    t = index2value(L, index);
    if if !((*t).tt_ as libc::c_int
        == 5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
            | (1 as libc::c_int) << 6 as libc::c_int)
    {
        slot = 0 as *const TValue;
        0 as libc::c_int
    } else {
        slot = (if (n as lua_Unsigned)
            .wrapping_sub(1 as libc::c_uint as libc::c_ulonglong)
            < (*((*t).value_.gc as *mut GCUnion)).h.alimit as libc::c_ulonglong
        {
            &mut *((*((*t).value_.gc as *mut GCUnion)).h.array)
                .offset((n - 1 as libc::c_int as i64) as isize)
                as *mut TValue as *const TValue
        } else {
            luaH_getint(&mut (*((*t).value_.gc as *mut GCUnion)).h, n)
        });
        !((*slot).tt_ as libc::c_int & 0xf as libc::c_int == 0 as libc::c_int)
            as libc::c_int
    } != 0
    {
        let mut io1: *mut TValue = slot as *mut TValue;
        let mut io2: *const TValue = &mut (*((*L).top.p)
            .offset(-(1 as libc::c_int as isize)))
            .val;
        (*io1).value_ = (*io2).value_;
        (*io1).tt_ = (*io2).tt_;
        if (*((*L).top.p).offset(-(1 as libc::c_int as isize))).val.tt_ as libc::c_int
            & (1 as libc::c_int) << 6 as libc::c_int != 0
        {
            if (*(*t).value_.gc).marked as libc::c_int
                & (1 as libc::c_int) << 5 as libc::c_int != 0
                && (*(*((*L).top.p).offset(-(1 as libc::c_int as isize))).val.value_.gc)
                    .marked as libc::c_int
                    & ((1 as libc::c_int) << 3 as libc::c_int
                        | (1 as libc::c_int) << 4 as libc::c_int) != 0
            {
                luaC_barrierback_(L, (*t).value_.gc);
            } else {};
        } else {};
    } else {
        let mut aux: TValue = TValue {
            value_: Value { gc: 0 as *mut GCObject },
            tt_: 0,
        };
        let mut io: *mut TValue = &mut aux;
        (*io).value_.i = n;
        (*io)
            .tt_ = (3 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int)
            as lu_byte;
        luaV_finishset(
            L,
            t,
            &mut aux,
            &mut (*((*L).top.p).offset(-(1 as libc::c_int as isize))).val,
            slot,
        );
    }
    (*L).top.p = ((*L).top.p).offset(-1);
    (*L).top.p;
}
unsafe extern "C" fn aux_rawset(
    mut L: *mut lua_State,
    mut index: libc::c_int,
    mut key: *mut TValue,
    mut n: libc::c_int,
) {
    let mut t: *mut Table = 0 as *mut Table;
    t = gettable(L, index);
    luaH_set(L, t, key, &mut (*((*L).top.p).offset(-(1 as libc::c_int as isize))).val);
    (*t)
        .flags = ((*t).flags as libc::c_uint
        & !!(!(0 as libc::c_uint) << TM_EQ as libc::c_int + 1 as libc::c_int))
        as lu_byte;
    if (*((*L).top.p).offset(-(1 as libc::c_int as isize))).val.tt_ as libc::c_int
        & (1 as libc::c_int) << 6 as libc::c_int != 0
    {
        if (*(t as *mut GCUnion)).gc.marked as libc::c_int
            & (1 as libc::c_int) << 5 as libc::c_int != 0
            && (*(*((*L).top.p).offset(-(1 as libc::c_int as isize))).val.value_.gc)
                .marked as libc::c_int
                & ((1 as libc::c_int) << 3 as libc::c_int
                    | (1 as libc::c_int) << 4 as libc::c_int) != 0
        {
            luaC_barrierback_(L, &mut (*(t as *mut GCUnion)).gc);
        } else {};
    } else {};
    (*L).top.p = ((*L).top.p).offset(-(n as isize));
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_rawset(mut L: *mut lua_State, mut index: libc::c_int) {
    aux_rawset(
        L,
        index,
        &mut (*((*L).top.p).offset(-(2 as libc::c_int as isize))).val,
        2 as libc::c_int,
    );
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_rawsetp(
    mut L: *mut lua_State,
    mut index: libc::c_int,
    mut p: *const libc::c_void,
) {
    let mut k: TValue = TValue {
        value_: Value { gc: 0 as *mut GCObject },
        tt_: 0,
    };
    let mut io: *mut TValue = &mut k;
    (*io).value_.p = p as *mut libc::c_void;
    (*io).tt_ = (2 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int) as lu_byte;
    aux_rawset(L, index, &mut k, 1 as libc::c_int);
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_rawseti(
    mut L: *mut lua_State,
    mut index: libc::c_int,
    mut n: lua_Integer,
) {
    let mut t: *mut Table = 0 as *mut Table;
    t = gettable(L, index);
    luaH_setint(L, t, n, &mut (*((*L).top.p).offset(-(1 as libc::c_int as isize))).val);
    if (*((*L).top.p).offset(-(1 as libc::c_int as isize))).val.tt_ as libc::c_int
        & (1 as libc::c_int) << 6 as libc::c_int != 0
    {
        if (*(t as *mut GCUnion)).gc.marked as libc::c_int
            & (1 as libc::c_int) << 5 as libc::c_int != 0
            && (*(*((*L).top.p).offset(-(1 as libc::c_int as isize))).val.value_.gc)
                .marked as libc::c_int
                & ((1 as libc::c_int) << 3 as libc::c_int
                    | (1 as libc::c_int) << 4 as libc::c_int) != 0
        {
            luaC_barrierback_(L, &mut (*(t as *mut GCUnion)).gc);
        } else {};
    } else {};
    (*L).top.p = ((*L).top.p).offset(-1);
    (*L).top.p;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_setmetatable(
    mut L: *mut lua_State,
    mut objindex: libc::c_int,
) -> libc::c_int {
    let mut obj: *mut TValue = 0 as *mut TValue;
    let mut mt: *mut Table = 0 as *mut Table;
    obj = index2value(L, objindex);
    if (*((*L).top.p).offset(-(1 as libc::c_int as isize))).val.tt_ as libc::c_int
        & 0xf as libc::c_int == 0 as libc::c_int
    {
        mt = 0 as *mut Table;
    } else {
        mt = &mut (*((*((*L).top.p).offset(-(1 as libc::c_int as isize))).val.value_.gc
            as *mut GCUnion))
            .h;
    }
    match (*obj).tt_ as libc::c_int & 0xf as libc::c_int {
        5 => {
            let ref mut fresh2 = (*((*obj).value_.gc as *mut GCUnion)).h.metatable;
            *fresh2 = mt;
            if !mt.is_null() {
                if (*(*obj).value_.gc).marked as libc::c_int
                    & (1 as libc::c_int) << 5 as libc::c_int != 0
                    && (*mt).marked as libc::c_int
                        & ((1 as libc::c_int) << 3 as libc::c_int
                            | (1 as libc::c_int) << 4 as libc::c_int) != 0
                {
                    luaC_barrier_(
                        L,
                        &mut (*((*obj).value_.gc as *mut GCUnion)).gc,
                        &mut (*(mt as *mut GCUnion)).gc,
                    );
                } else {};
                luaC_checkfinalizer(L, (*obj).value_.gc, mt);
            }
        }
        7 => {
            let ref mut fresh3 = (*((*obj).value_.gc as *mut GCUnion)).u.metatable;
            *fresh3 = mt;
            if !mt.is_null() {
                if (*((*obj).value_.gc as *mut GCUnion)).u.marked as libc::c_int
                    & (1 as libc::c_int) << 5 as libc::c_int != 0
                    && (*mt).marked as libc::c_int
                        & ((1 as libc::c_int) << 3 as libc::c_int
                            | (1 as libc::c_int) << 4 as libc::c_int) != 0
                {
                    luaC_barrier_(
                        L,
                        &mut (*(&mut (*((*obj).value_.gc as *mut GCUnion)).u
                            as *mut Udata as *mut GCUnion))
                            .gc,
                        &mut (*(mt as *mut GCUnion)).gc,
                    );
                } else {};
                luaC_checkfinalizer(L, (*obj).value_.gc, mt);
            }
        }
        _ => {
            (*(*L).l_G)
                .mt[((*obj).tt_ as libc::c_int & 0xf as libc::c_int) as usize] = mt;
        }
    }
    (*L).top.p = ((*L).top.p).offset(-1);
    (*L).top.p;
    return 1 as libc::c_int;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_setiuservalue(
    mut L: *mut lua_State,
    mut index: libc::c_int,
    mut n: libc::c_int,
) -> libc::c_int {
    let mut o: *mut TValue = 0 as *mut TValue;
    let mut res: libc::c_int = 0;
    o = index2value(L, index);
    if !((n as libc::c_uint).wrapping_sub(1 as libc::c_uint)
        < (*((*o).value_.gc as *mut GCUnion)).u.nuvalue as libc::c_uint)
    {
        res = 0 as libc::c_int;
    } else {
        let mut io1: *mut TValue = &mut (*((*((*o).value_.gc as *mut GCUnion)).u.uv)
            .as_mut_ptr()
            .offset((n - 1 as libc::c_int) as isize))
            .uv;
        let mut io2: *const TValue = &mut (*((*L).top.p)
            .offset(-(1 as libc::c_int as isize)))
            .val;
        (*io1).value_ = (*io2).value_;
        (*io1).tt_ = (*io2).tt_;
        if (*((*L).top.p).offset(-(1 as libc::c_int as isize))).val.tt_ as libc::c_int
            & (1 as libc::c_int) << 6 as libc::c_int != 0
        {
            if (*(*o).value_.gc).marked as libc::c_int
                & (1 as libc::c_int) << 5 as libc::c_int != 0
                && (*(*((*L).top.p).offset(-(1 as libc::c_int as isize))).val.value_.gc)
                    .marked as libc::c_int
                    & ((1 as libc::c_int) << 3 as libc::c_int
                        | (1 as libc::c_int) << 4 as libc::c_int) != 0
            {
                luaC_barrierback_(L, (*o).value_.gc);
            } else {};
        } else {};
        res = 1 as libc::c_int;
    }
    (*L).top.p = ((*L).top.p).offset(-1);
    (*L).top.p;
    return res;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_callk(
    mut L: *mut lua_State,
    mut nargs: libc::c_int,
    mut nresults: libc::c_int,
    mut ctx: lua_KContext,
    mut k: lua_KFunction,
) {
    let mut func: StkId = 0 as *mut StackValue;
    func = ((*L).top.p).offset(-((nargs + 1 as libc::c_int) as isize));
    if k.is_some()
        && (*L).nCcalls & 0xffff0000 as libc::c_uint == 0 as libc::c_int as libc::c_uint
    {
        (*(*L).ci).u.c.k = k;
        (*(*L).ci).u.c.ctx = ctx;
        luaD_call(L, func, nresults);
    } else {
        luaD_callnoyield(L, func, nresults);
    }
    if nresults <= -(1 as libc::c_int) && (*(*L).ci).top.p < (*L).top.p {
        (*(*L).ci).top.p = (*L).top.p;
    }
}
unsafe extern "C" fn f_call(mut L: *mut lua_State, mut ud: *mut libc::c_void) {
    let mut c: *mut CallS = ud as *mut CallS;
    luaD_callnoyield(L, (*c).func, (*c).nresults);
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_pcallk(
    mut L: *mut lua_State,
    mut nargs: libc::c_int,
    mut nresults: libc::c_int,
    mut errfunc: libc::c_int,
    mut ctx: lua_KContext,
    mut k: lua_KFunction,
) -> libc::c_int {
    let mut c: CallS = CallS {
        func: 0 as *mut StackValue,
        nresults: 0,
    };
    let mut status: libc::c_int = 0;
    let mut func: ptrdiff_t = 0;
    if errfunc == 0 as libc::c_int {
        func = 0 as libc::c_int as ptrdiff_t;
    } else {
        let mut o: StkId = index2stack(L, errfunc);
        func = (o as *mut libc::c_char).offset_from((*L).stack.p as *mut libc::c_char)
            as libc::c_long;
    }
    c.func = ((*L).top.p).offset(-((nargs + 1 as libc::c_int) as isize));
    if k.is_none()
        || !((*L).nCcalls & 0xffff0000 as libc::c_uint
            == 0 as libc::c_int as libc::c_uint)
    {
        c.nresults = nresults;
        status = luaD_pcall(
            L,
            Some(
                f_call as unsafe extern "C" fn(*mut lua_State, *mut libc::c_void) -> (),
            ),
            &mut c as *mut CallS as *mut libc::c_void,
            (c.func as *mut libc::c_char).offset_from((*L).stack.p as *mut libc::c_char)
                as libc::c_long,
            func,
        );
    } else {
        let mut ci: *mut CallInfo = (*L).ci;
        (*ci).u.c.k = k;
        (*ci).u.c.ctx = ctx;
        (*ci)
            .u2
            .funcidx = (c.func as *mut libc::c_char)
            .offset_from((*L).stack.p as *mut libc::c_char) as libc::c_long
            as libc::c_int;
        (*ci).u.c.old_errfunc = (*L).errfunc;
        (*L).errfunc = func;
        (*ci)
            .callstatus = ((*ci).callstatus as libc::c_int
            & !((1 as libc::c_int) << 0 as libc::c_int) | (*L).allowhook as libc::c_int)
            as libc::c_ushort;
        (*ci)
            .callstatus = ((*ci).callstatus as libc::c_int
            | (1 as libc::c_int) << 4 as libc::c_int) as libc::c_ushort;
        luaD_call(L, c.func, nresults);
        (*ci)
            .callstatus = ((*ci).callstatus as libc::c_int
            & !((1 as libc::c_int) << 4 as libc::c_int)) as libc::c_ushort;
        (*L).errfunc = (*ci).u.c.old_errfunc;
        status = 0 as libc::c_int;
    }
    if nresults <= -(1 as libc::c_int) && (*(*L).ci).top.p < (*L).top.p {
        (*(*L).ci).top.p = (*L).top.p;
    }
    return status;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_load(
    mut L: *mut lua_State,
    mut reader: lua_Reader,
    mut data: *mut libc::c_void,
    mut chunkname: *const libc::c_char,
    mut mode: *const libc::c_char,
) -> libc::c_int {
    let mut z: ZIO = ZIO {
        n: 0,
        p: 0 as *const libc::c_char,
        reader: None,
        data: 0 as *mut libc::c_void,
        L: 0 as *mut lua_State,
    };
    let mut status: libc::c_int = 0;
    if chunkname.is_null() {
        chunkname = b"?\0" as *const u8 as *const libc::c_char;
    }
    luaZ_init(L, &mut z, reader, data);
    status = luaD_protectedparser(L, &mut z, chunkname, mode);
    if status == 0 as libc::c_int {
        let mut f: *mut LClosure = &mut (*((*((*L).top.p)
            .offset(-(1 as libc::c_int as isize)))
            .val
            .value_
            .gc as *mut GCUnion))
            .cl
            .l;
        if (*f).nupvalues as libc::c_int >= 1 as libc::c_int {
            let mut gt: *const TValue = &mut *((*((*(*L).l_G).l_registry.value_.gc
                as *mut GCUnion))
                .h
                .array)
                .offset((2 as libc::c_int - 1 as libc::c_int) as isize) as *mut TValue;
            let mut io1: *mut TValue = (**((*f).upvals)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize))
                .v
                .p;
            let mut io2: *const TValue = gt;
            (*io1).value_ = (*io2).value_;
            (*io1).tt_ = (*io2).tt_;
            if (*gt).tt_ as libc::c_int & (1 as libc::c_int) << 6 as libc::c_int != 0 {
                if (**((*f).upvals).as_mut_ptr().offset(0 as libc::c_int as isize))
                    .marked as libc::c_int & (1 as libc::c_int) << 5 as libc::c_int != 0
                    && (*(*gt).value_.gc).marked as libc::c_int
                        & ((1 as libc::c_int) << 3 as libc::c_int
                            | (1 as libc::c_int) << 4 as libc::c_int) != 0
                {
                    luaC_barrier_(
                        L,
                        &mut (*(*((*f).upvals)
                            .as_mut_ptr()
                            .offset(0 as libc::c_int as isize) as *mut GCUnion))
                            .gc,
                        &mut (*((*gt).value_.gc as *mut GCUnion)).gc,
                    );
                } else {};
            } else {};
        }
    }
    return status;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_dump(
    mut L: *mut lua_State,
    mut writer: lua_Writer,
    mut data: *mut libc::c_void,
    mut strip: libc::c_int,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut o: *mut TValue = 0 as *mut TValue;
    o = &mut (*((*L).top.p).offset(-(1 as libc::c_int as isize))).val;
    if (*o).tt_ as libc::c_int
        == 6 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
            | (1 as libc::c_int) << 6 as libc::c_int
    {
        status = luaU_dump(
            L,
            (*((*o).value_.gc as *mut GCUnion)).cl.l.p,
            writer,
            data,
            strip,
        );
    } else {
        status = 1 as libc::c_int;
    }
    return status;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_status(mut L: *mut lua_State) -> libc::c_int {
    return (*L).status as libc::c_int;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_gc(
    mut L: *mut lua_State,
    mut what: libc::c_int,
    mut args: ...
) -> libc::c_int {
    let mut argp: ::core::ffi::VaListImpl;
    let mut res: libc::c_int = 0 as libc::c_int;
    let mut g: *mut global_State = (*L).l_G;
    if (*g).gcstp as libc::c_int & 2 as libc::c_int != 0 {
        return -(1 as libc::c_int);
    }
    argp = args.clone();
    match what {
        0 => {
            (*g).gcstp = 1 as libc::c_int as lu_byte;
        }
        1 => {
            luaE_setdebt(g, 0 as libc::c_int as l_mem);
            (*g).gcstp = 0 as libc::c_int as lu_byte;
        }
        2 => {
            luaC_fullgc(L, 0 as libc::c_int);
        }
        3 => {
            res = (((*g).totalbytes + (*g).GCdebt) as lu_mem >> 10 as libc::c_int)
                as libc::c_int;
        }
        4 => {
            res = (((*g).totalbytes + (*g).GCdebt) as lu_mem
                & 0x3ff as libc::c_int as libc::c_ulong) as libc::c_int;
        }
        5 => {
            let mut data: libc::c_int = argp.arg::<libc::c_int>();
            let mut debt: l_mem = 1 as libc::c_int as l_mem;
            let mut oldstp: lu_byte = (*g).gcstp;
            (*g).gcstp = 0 as libc::c_int as lu_byte;
            if data == 0 as libc::c_int {
                luaE_setdebt(g, 0 as libc::c_int as l_mem);
                luaC_step(L);
            } else {
                debt = data as l_mem * 1024 as libc::c_int as libc::c_long + (*g).GCdebt;
                luaE_setdebt(g, debt);
                if (*(*L).l_G).GCdebt > 0 as libc::c_int as libc::c_long {
                    luaC_step(L);
                }
            }
            (*g).gcstp = oldstp;
            if debt > 0 as libc::c_int as libc::c_long
                && (*g).gcstate as libc::c_int == 8 as libc::c_int
            {
                res = 1 as libc::c_int;
            }
        }
        6 => {
            let mut data_0: libc::c_int = argp.arg::<libc::c_int>();
            res = (*g).gcpause as libc::c_int * 4 as libc::c_int;
            (*g).gcpause = (data_0 / 4 as libc::c_int) as lu_byte;
        }
        7 => {
            let mut data_1: libc::c_int = argp.arg::<libc::c_int>();
            res = (*g).gcstepmul as libc::c_int * 4 as libc::c_int;
            (*g).gcstepmul = (data_1 / 4 as libc::c_int) as lu_byte;
        }
        9 => {
            res = ((*g).gcstp as libc::c_int == 0 as libc::c_int) as libc::c_int;
        }
        10 => {
            let mut minormul: libc::c_int = argp.arg::<libc::c_int>();
            let mut majormul: libc::c_int = argp.arg::<libc::c_int>();
            res = if (*g).gckind as libc::c_int == 1 as libc::c_int
                || (*g).lastatomic != 0 as libc::c_int as libc::c_ulong
            {
                10 as libc::c_int
            } else {
                11 as libc::c_int
            };
            if minormul != 0 as libc::c_int {
                (*g).genminormul = minormul as lu_byte;
            }
            if majormul != 0 as libc::c_int {
                (*g).genmajormul = (majormul / 4 as libc::c_int) as lu_byte;
            }
            luaC_changemode(L, 1 as libc::c_int);
        }
        11 => {
            let mut pause: libc::c_int = argp.arg::<libc::c_int>();
            let mut stepmul: libc::c_int = argp.arg::<libc::c_int>();
            let mut stepsize: libc::c_int = argp.arg::<libc::c_int>();
            res = if (*g).gckind as libc::c_int == 1 as libc::c_int
                || (*g).lastatomic != 0 as libc::c_int as libc::c_ulong
            {
                10 as libc::c_int
            } else {
                11 as libc::c_int
            };
            if pause != 0 as libc::c_int {
                (*g).gcpause = (pause / 4 as libc::c_int) as lu_byte;
            }
            if stepmul != 0 as libc::c_int {
                (*g).gcstepmul = (stepmul / 4 as libc::c_int) as lu_byte;
            }
            if stepsize != 0 as libc::c_int {
                (*g).gcstepsize = stepsize as lu_byte;
            }
            luaC_changemode(L, 0 as libc::c_int);
        }
        _ => {
            res = -(1 as libc::c_int);
        }
    }
    return res;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_error(mut L: *mut lua_State) -> libc::c_int {
    let mut errobj: *mut TValue = 0 as *mut TValue;
    errobj = &mut (*((*L).top.p).offset(-(1 as libc::c_int as isize))).val;
    if (*errobj).tt_ as libc::c_int
        == 4 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
            | (1 as libc::c_int) << 6 as libc::c_int
        && &mut (*((*errobj).value_.gc as *mut GCUnion)).ts as *mut TString
            == (*(*L).l_G).memerrmsg
    {
        luaD_throw(L, 4 as libc::c_int);
    } else {
        luaG_errormsg(L);
    };
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_next(
    mut L: *mut lua_State,
    mut index: libc::c_int,
) -> libc::c_int {
    let mut t: *mut Table = 0 as *mut Table;
    let mut more: libc::c_int = 0;
    t = gettable(L, index);
    more = luaH_next(L, t, ((*L).top.p).offset(-(1 as libc::c_int as isize)));
    if more != 0 {
        (*L).top.p = ((*L).top.p).offset(1);
        (*L).top.p;
    } else {
        (*L).top.p = ((*L).top.p).offset(-(1 as libc::c_int as isize));
    }
    return more;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_toclose(mut L: *mut lua_State, mut index: libc::c_int) {
    let mut nresults: libc::c_int = 0;
    let mut o: StkId = 0 as *mut StackValue;
    o = index2stack(L, index);
    nresults = (*(*L).ci).nresults as libc::c_int;
    luaF_newtbcupval(L, o);
    if !(nresults < -(1 as libc::c_int)) {
        (*(*L).ci).nresults = (-nresults - 3 as libc::c_int) as libc::c_short;
    }
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_concat(mut L: *mut lua_State, mut n: libc::c_int) {
    if n > 0 as libc::c_int {
        luaV_concat(L, n);
    } else {
        let mut io: *mut TValue = &mut (*(*L).top.p).val;
        let mut x_: *mut TString = luaS_newlstr(
            L,
            b"\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
        (*io)
            .tt_ = ((*x_).tt as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int)
            as lu_byte;
        (*L).top.p = ((*L).top.p).offset(1);
        (*L).top.p;
    }
    if (*(*L).l_G).GCdebt > 0 as libc::c_int as libc::c_long {
        luaC_step(L);
    }
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_len(mut L: *mut lua_State, mut index: libc::c_int) {
    let mut t: *mut TValue = 0 as *mut TValue;
    t = index2value(L, index);
    luaV_objlen(L, (*L).top.p, t);
    (*L).top.p = ((*L).top.p).offset(1);
    (*L).top.p;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_getallocf(
    mut L: *mut lua_State,
    mut ud: *mut *mut libc::c_void,
) -> lua_Alloc {
    let mut f: lua_Alloc = None;
    if !ud.is_null() {
        *ud = (*(*L).l_G).ud;
    }
    f = (*(*L).l_G).frealloc;
    return f;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_setallocf(
    mut L: *mut lua_State,
    mut f: lua_Alloc,
    mut ud: *mut libc::c_void,
) {
    (*(*L).l_G).ud = ud;
    (*(*L).l_G).frealloc = f;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_setwarnf(
    mut L: *mut lua_State,
    mut f: lua_WarnFunction,
    mut ud: *mut libc::c_void,
) {
    (*(*L).l_G).ud_warn = ud;
    (*(*L).l_G).warnf = f;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_warning(
    mut L: *mut lua_State,
    mut msg: *const libc::c_char,
    mut tocont: libc::c_int,
) {
    luaE_warning(L, msg, tocont);
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_newuserdatauv(
    mut L: *mut lua_State,
    mut size: size_t,
    mut nuvalue: libc::c_int,
) -> *mut libc::c_void {
    let mut u: *mut Udata = 0 as *mut Udata;
    u = luaS_newudata(L, size, nuvalue);
    let mut io: *mut TValue = &mut (*(*L).top.p).val;
    let mut x_: *mut Udata = u;
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
    (*io)
        .tt_ = (7 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 6 as libc::c_int) as lu_byte;
    (*L).top.p = ((*L).top.p).offset(1);
    (*L).top.p;
    if (*(*L).l_G).GCdebt > 0 as libc::c_int as libc::c_long {
        luaC_step(L);
    }
    return (u as *mut libc::c_char)
        .offset(
            (if (*u).nuvalue as libc::c_int == 0 as libc::c_int {
                32 as libc::c_ulong
            } else {
                (40 as libc::c_ulong)
                    .wrapping_add(
                        (::core::mem::size_of::<UValue>() as libc::c_ulong)
                            .wrapping_mul((*u).nuvalue as libc::c_ulong),
                    )
            }) as isize,
        ) as *mut libc::c_void;
}
unsafe extern "C" fn aux_upvalue(
    mut fi: *mut TValue,
    mut n: libc::c_int,
    mut val: *mut *mut TValue,
    mut owner: *mut *mut GCObject,
) -> *const libc::c_char {
    match (*fi).tt_ as libc::c_int & 0x3f as libc::c_int {
        38 => {
            let mut f: *mut CClosure = &mut (*((*fi).value_.gc as *mut GCUnion)).cl.c;
            if !((n as libc::c_uint).wrapping_sub(1 as libc::c_uint)
                < (*f).nupvalues as libc::c_uint)
            {
                return 0 as *const libc::c_char;
            }
            *val = &mut *((*f).upvalue)
                .as_mut_ptr()
                .offset((n - 1 as libc::c_int) as isize) as *mut TValue;
            if !owner.is_null() {
                *owner = &mut (*(f as *mut GCUnion)).gc;
            }
            return b"\0" as *const u8 as *const libc::c_char;
        }
        6 => {
            let mut f_0: *mut LClosure = &mut (*((*fi).value_.gc as *mut GCUnion)).cl.l;
            let mut name: *mut TString = 0 as *mut TString;
            let mut p: *mut Proto = (*f_0).p;
            if !((n as libc::c_uint).wrapping_sub(1 as libc::c_uint)
                < (*p).sizeupvalues as libc::c_uint)
            {
                return 0 as *const libc::c_char;
            }
            *val = (**((*f_0).upvals)
                .as_mut_ptr()
                .offset((n - 1 as libc::c_int) as isize))
                .v
                .p;
            if !owner.is_null() {
                *owner = &mut (*(*((*f_0).upvals)
                    .as_mut_ptr()
                    .offset((n - 1 as libc::c_int) as isize) as *mut GCUnion))
                    .gc;
            }
            name = (*((*p).upvalues).offset((n - 1 as libc::c_int) as isize)).name;
            return if name.is_null() {
                b"(no name)\0" as *const u8 as *const libc::c_char
            } else {
                ((*name).contents).as_mut_ptr() as *const libc::c_char
            };
        }
        _ => return 0 as *const libc::c_char,
    };
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_getupvalue(
    mut L: *mut lua_State,
    mut funcindex: libc::c_int,
    mut n: libc::c_int,
) -> *const libc::c_char {
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut val: *mut TValue = 0 as *mut TValue;
    name = aux_upvalue(index2value(L, funcindex), n, &mut val, 0 as *mut *mut GCObject);
    if !name.is_null() {
        let mut io1: *mut TValue = &mut (*(*L).top.p).val;
        let mut io2: *const TValue = val;
        (*io1).value_ = (*io2).value_;
        (*io1).tt_ = (*io2).tt_;
        (*L).top.p = ((*L).top.p).offset(1);
        (*L).top.p;
    }
    return name;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_setupvalue(
    mut L: *mut lua_State,
    mut funcindex: libc::c_int,
    mut n: libc::c_int,
) -> *const libc::c_char {
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut val: *mut TValue = 0 as *mut TValue;
    let mut owner: *mut GCObject = 0 as *mut GCObject;
    let mut fi: *mut TValue = 0 as *mut TValue;
    fi = index2value(L, funcindex);
    name = aux_upvalue(fi, n, &mut val, &mut owner);
    if !name.is_null() {
        (*L).top.p = ((*L).top.p).offset(-1);
        (*L).top.p;
        let mut io1: *mut TValue = val;
        let mut io2: *const TValue = &mut (*(*L).top.p).val;
        (*io1).value_ = (*io2).value_;
        (*io1).tt_ = (*io2).tt_;
        if (*val).tt_ as libc::c_int & (1 as libc::c_int) << 6 as libc::c_int != 0 {
            if (*owner).marked as libc::c_int & (1 as libc::c_int) << 5 as libc::c_int
                != 0
                && (*(*val).value_.gc).marked as libc::c_int
                    & ((1 as libc::c_int) << 3 as libc::c_int
                        | (1 as libc::c_int) << 4 as libc::c_int) != 0
            {
                luaC_barrier_(
                    L,
                    &mut (*(owner as *mut GCUnion)).gc,
                    &mut (*((*val).value_.gc as *mut GCUnion)).gc,
                );
            } else {};
        } else {};
    }
    return name;
}
unsafe extern "C" fn getupvalref(
    mut L: *mut lua_State,
    mut fidx: libc::c_int,
    mut n: libc::c_int,
    mut pf: *mut *mut LClosure,
) -> *mut *mut UpVal {
    static mut nullup: *const UpVal = 0 as *const UpVal;
    let mut f: *mut LClosure = 0 as *mut LClosure;
    let mut fi: *mut TValue = index2value(L, fidx);
    f = &mut (*((*fi).value_.gc as *mut GCUnion)).cl.l;
    if !pf.is_null() {
        *pf = f;
    }
    if 1 as libc::c_int <= n && n <= (*(*f).p).sizeupvalues {
        return &mut *((*f).upvals).as_mut_ptr().offset((n - 1 as libc::c_int) as isize)
            as *mut *mut UpVal
    } else {
        return &nullup as *const *const UpVal as *mut *mut UpVal
    };
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_upvalueid(
    mut L: *mut lua_State,
    mut fidx: libc::c_int,
    mut n: libc::c_int,
) -> *mut libc::c_void {
    let mut fi: *mut TValue = index2value(L, fidx);
    match (*fi).tt_ as libc::c_int & 0x3f as libc::c_int {
        6 => {
            return *getupvalref(L, fidx, n, 0 as *mut *mut LClosure) as *mut libc::c_void;
        }
        38 => {
            let mut f: *mut CClosure = &mut (*((*fi).value_.gc as *mut GCUnion)).cl.c;
            if 1 as libc::c_int <= n && n <= (*f).nupvalues as libc::c_int {
                return &mut *((*f).upvalue)
                    .as_mut_ptr()
                    .offset((n - 1 as libc::c_int) as isize) as *mut TValue
                    as *mut libc::c_void;
            }
        }
        22 => {}
        _ => return 0 as *mut libc::c_void,
    }
    return 0 as *mut libc::c_void;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_upvaluejoin(
    mut L: *mut lua_State,
    mut fidx1: libc::c_int,
    mut n1: libc::c_int,
    mut fidx2: libc::c_int,
    mut n2: libc::c_int,
) {
    let mut f1: *mut LClosure = 0 as *mut LClosure;
    let mut up1: *mut *mut UpVal = getupvalref(L, fidx1, n1, &mut f1);
    let mut up2: *mut *mut UpVal = getupvalref(L, fidx2, n2, 0 as *mut *mut LClosure);
    *up1 = *up2;
    if (*f1).marked as libc::c_int & (1 as libc::c_int) << 5 as libc::c_int != 0
        && (**up1).marked as libc::c_int
            & ((1 as libc::c_int) << 3 as libc::c_int
                | (1 as libc::c_int) << 4 as libc::c_int) != 0
    {
        luaC_barrier_(
            L,
            &mut (*(f1 as *mut GCUnion)).gc,
            &mut (*(*up1 as *mut GCUnion)).gc,
        );
    } else {};
}
