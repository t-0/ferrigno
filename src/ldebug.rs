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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn luaO_pushvfstring(
        L: *mut lua_State,
        fmt: *const libc::c_char,
        argp: ::core::ffi::VaList,
    ) -> *const libc::c_char;
    fn luaO_pushfstring(
        L: *mut lua_State,
        fmt: *const libc::c_char,
        _: ...
    ) -> *const libc::c_char;
    fn luaO_chunkid(out: *mut libc::c_char, source: *const libc::c_char, srclen: size_t);
    fn luaT_objtypename(L: *mut lua_State, o: *const TValue) -> *const libc::c_char;
    static luaP_opmodes: [u8; 83];
    fn luaD_hook(
        L: *mut lua_State,
        event: libc::c_int,
        line: libc::c_int,
        fTransfer: libc::c_int,
        nTransfer: libc::c_int,
    );
    fn luaD_hookcall(L: *mut lua_State, ci: *mut CallInfo);
    fn luaD_callnoyield(L: *mut lua_State, func: StkId, nResults: libc::c_int);
    fn luaD_throw(L: *mut lua_State, errcode: libc::c_int) -> !;
    fn luaF_getlocalname(
        func: *const Proto,
        local_number: libc::c_int,
        pc: libc::c_int,
    ) -> *const libc::c_char;
    fn luaC_step(L: *mut lua_State);
    fn luaH_setint(
        L: *mut lua_State,
        t: *mut Table,
        key: Integer,
        value: *mut TValue,
    );
    fn luaH_new(L: *mut lua_State) -> *mut Table;
    fn luaV_tointegerns(
        obj: *const TValue,
        p: *mut Integer,
        mode: F2Imod,
    ) -> libc::c_int;
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
    pub instack: u8,
    pub index: u8,
    pub kind: u8,
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
pub const OP_RETURN: OpCode = 70;
pub const OP_CLOSE: OpCode = 54;
pub const OP_GEI: OpCode = 65;
pub const OP_LEI: OpCode = 63;
pub const OP_LE: OpCode = 59;
pub const OP_GTI: OpCode = 64;
pub const OP_LTI: OpCode = 62;
pub const OP_LT: OpCode = 58;
pub const OP_EQ: OpCode = 57;
pub const OP_CONCAT: OpCode = 53;
pub const OP_LEN: OpCode = 52;
pub const OP_BNOT: OpCode = 50;
pub const OP_UNM: OpCode = 49;
pub const OP_MMBINK: OpCode = 48;
pub const OP_MMBINI: OpCode = 47;
pub const OP_MMBIN: OpCode = 46;
pub const OP_SETFIELD: OpCode = 18;
pub const OP_SETI: OpCode = 17;
pub const OP_SETTABLE: OpCode = 16;
pub const OP_SETTABUP: OpCode = 15;
pub const OP_GETFIELD: OpCode = 14;
pub const OP_GETI: OpCode = 13;
pub const OP_GETTABLE: OpCode = 12;
pub const OP_GETTABUP: OpCode = 11;
pub const OP_SELF: OpCode = 20;
pub const OP_TFORCALL: OpCode = 76;
pub const OP_LOADKX: OpCode = 4;
pub const OP_LOADK: OpCode = 3;
pub const OP_GETUPVAL: OpCode = 9;
pub const OP_MOVE: OpCode = 0;
pub type OpCode = libc::c_uint;
pub const OP_EXTRAARG: OpCode = 82;
pub const OP_VARARGPREP: OpCode = 81;
pub const OP_VARARG: OpCode = 80;
pub const OP_CLOSURE: OpCode = 79;
pub const OP_SETLIST: OpCode = 78;
pub const OP_TFORLOOP: OpCode = 77;
pub const OP_TFORPREP: OpCode = 75;
pub const OP_FORPREP: OpCode = 74;
pub const OP_FORLOOP: OpCode = 73;
pub const OP_RETURN1: OpCode = 72;
pub const OP_RETURN0: OpCode = 71;
pub const OP_TAILCALL: OpCode = 69;
pub const OP_CALL: OpCode = 68;
pub const OP_TESTSET: OpCode = 67;
pub const OP_TEST: OpCode = 66;
pub const OP_EQI: OpCode = 61;
pub const OP_EQK: OpCode = 60;
pub const OP_JMP: OpCode = 56;
pub const OP_TBC: OpCode = 55;
pub const OP_NOT: OpCode = 51;
pub const OP_SHR: OpCode = 45;
pub const OP_SHL: OpCode = 44;
pub const OP_BXOR: OpCode = 43;
pub const OP_BOR: OpCode = 42;
pub const OP_BAND: OpCode = 41;
pub const OP_IDIV: OpCode = 40;
pub const OP_DIV: OpCode = 39;
pub const OP_POW: OpCode = 38;
pub const OP_MOD: OpCode = 37;
pub const OP_MUL: OpCode = 36;
pub const OP_SUB: OpCode = 35;
pub const OP_ADD: OpCode = 34;
pub const OP_SHLI: OpCode = 33;
pub const OP_SHRI: OpCode = 32;
pub const OP_BXORK: OpCode = 31;
pub const OP_BORK: OpCode = 30;
pub const OP_BANDK: OpCode = 29;
pub const OP_IDIVK: OpCode = 28;
pub const OP_DIVK: OpCode = 27;
pub const OP_POWK: OpCode = 26;
pub const OP_MODK: OpCode = 25;
pub const OP_MULK: OpCode = 24;
pub const OP_SUBK: OpCode = 23;
pub const OP_ADDK: OpCode = 22;
pub const OP_ADDI: OpCode = 21;
pub const OP_NEWTABLE: OpCode = 19;
pub const OP_SETUPVAL: OpCode = 10;
pub const OP_LOADNIL: OpCode = 8;
pub const OP_LOADTRUE: OpCode = 7;
pub const OP_LFALSESKIP: OpCode = 6;
pub const OP_LOADFALSE: OpCode = 5;
pub const OP_LOADF: OpCode = 2;
pub const OP_LOADI: OpCode = 1;
pub type F2Imod = libc::c_uint;
pub const F2Iceil: F2Imod = 2;
pub const F2Ifloor: F2Imod = 1;
pub const F2Ieq: F2Imod = 0;
static mut strlocal: [libc::c_char; 6] = unsafe {
    *::core::mem::transmute::<&[u8; 6], &[libc::c_char; 6]>(b"local\0")
};
static mut strupval: [libc::c_char; 8] = unsafe {
    *::core::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"upvalue\0")
};
unsafe extern "C" fn currentpc(mut ci: *mut CallInfo) -> libc::c_int {
    return ((*ci).u.l.savedpc)
        .offset_from((*(*((*(*ci).func.p).val.value_.gc as *mut GCUnion)).cl.l.p).code)
        as libc::c_long as libc::c_int - 1 as libc::c_int;
}
unsafe extern "C" fn getbaseline(
    mut f: *const Proto,
    mut pc: libc::c_int,
    mut basepc: *mut libc::c_int,
) -> libc::c_int {
    if (*f).sizeabslineinfo == 0 as libc::c_int
        || pc < (*((*f).abslineinfo).offset(0 as libc::c_int as isize)).pc
    {
        *basepc = -(1 as libc::c_int);
        return (*f).linedefined;
    } else {
        let mut i: libc::c_int = (pc as libc::c_uint)
            .wrapping_div(128 as libc::c_int as libc::c_uint)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int;
        while (i + 1 as libc::c_int) < (*f).sizeabslineinfo
            && pc >= (*((*f).abslineinfo).offset((i + 1 as libc::c_int) as isize)).pc
        {
            i += 1;
            i;
        }
        *basepc = (*((*f).abslineinfo).offset(i as isize)).pc;
        return (*((*f).abslineinfo).offset(i as isize)).line;
    };
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaG_getfuncline(
    mut f: *const Proto,
    mut pc: libc::c_int,
) -> libc::c_int {
    if ((*f).lineinfo).is_null() {
        return -(1 as libc::c_int)
    } else {
        let mut basepc: libc::c_int = 0;
        let mut baseline: libc::c_int = getbaseline(f, pc, &mut basepc);
        loop {
            let fresh0 = basepc;
            basepc = basepc + 1;
            if !(fresh0 < pc) {
                break;
            }
            baseline += *((*f).lineinfo).offset(basepc as isize) as libc::c_int;
        }
        return baseline;
    };
}
unsafe extern "C" fn getcurrentline(mut ci: *mut CallInfo) -> libc::c_int {
    return luaG_getfuncline(
        (*((*(*ci).func.p).val.value_.gc as *mut GCUnion)).cl.l.p,
        currentpc(ci),
    );
}
unsafe extern "C" fn settraps(mut ci: *mut CallInfo) {
    while !ci.is_null() {
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
pub unsafe extern "C" fn lua_sethook(
    mut L: *mut lua_State,
    mut func: lua_Hook,
    mut mask: libc::c_int,
    mut count: libc::c_int,
) {
    if func.is_none() || mask == 0 as libc::c_int {
        mask = 0 as libc::c_int;
        func = None;
    }
    ::core::ptr::write_volatile(&mut (*L).hook as *mut lua_Hook, func);
    (*L).basehookcount = count;
    (*L).hookcount = (*L).basehookcount;
    ::core::ptr::write_volatile(
        &mut (*L).hookmask as *mut sig_atomic_t,
        mask as u8 as sig_atomic_t,
    );
    if mask != 0 {
        settraps((*L).ci);
    }
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_gethook(mut L: *mut lua_State) -> lua_Hook {
    return (*L).hook;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_gethookmask(mut L: *mut lua_State) -> libc::c_int {
    return (*L).hookmask;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_gethookcount(mut L: *mut lua_State) -> libc::c_int {
    return (*L).basehookcount;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_getstack(
    mut L: *mut lua_State,
    mut level: libc::c_int,
    mut ar: *mut lua_Debug,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut ci: *mut CallInfo = 0 as *mut CallInfo;
    if level < 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    ci = (*L).ci;
    while level > 0 as libc::c_int && ci != &mut (*L).base_ci as *mut CallInfo {
        level -= 1;
        level;
        ci = (*ci).previous;
    }
    if level == 0 as libc::c_int && ci != &mut (*L).base_ci as *mut CallInfo {
        status = 1 as libc::c_int;
        (*ar).i_ci = ci;
    } else {
        status = 0 as libc::c_int;
    }
    return status;
}
unsafe extern "C" fn upvalname(
    mut p: *const Proto,
    mut uv: libc::c_int,
) -> *const libc::c_char {
    let mut s: *mut TString = (*((*p).upvalues).offset(uv as isize)).name;
    if s.is_null() {
        return b"?\0" as *const u8 as *const libc::c_char
    } else {
        return ((*s).contents).as_mut_ptr()
    };
}
unsafe extern "C" fn findvararg(
    mut ci: *mut CallInfo,
    mut n: libc::c_int,
    mut pos: *mut StkId,
) -> *const libc::c_char {
    if (*(*((*(*ci).func.p).val.value_.gc as *mut GCUnion)).cl.l.p).is_vararg != 0 {
        let mut nextra: libc::c_int = (*ci).u.l.nextraargs;
        if n >= -nextra {
            *pos = ((*ci).func.p)
                .offset(-(nextra as isize))
                .offset(-((n + 1 as libc::c_int) as isize));
            return b"(vararg)\0" as *const u8 as *const libc::c_char;
        }
    }
    return 0 as *const libc::c_char;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaG_findlocal(
    mut L: *mut lua_State,
    mut ci: *mut CallInfo,
    mut n: libc::c_int,
    mut pos: *mut StkId,
) -> *const libc::c_char {
    let mut base: StkId = ((*ci).func.p).offset(1 as libc::c_int as isize);
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    if (*ci).callstatus as libc::c_int & (1 as libc::c_int) << 1 as libc::c_int == 0 {
        if n < 0 as libc::c_int {
            return findvararg(ci, n, pos)
        } else {
            name = luaF_getlocalname(
                (*((*(*ci).func.p).val.value_.gc as *mut GCUnion)).cl.l.p,
                n,
                currentpc(ci),
            );
        }
    }
    if name.is_null() {
        let mut limit: StkId = if ci == (*L).ci {
            (*L).top.p
        } else {
            (*(*ci).next).func.p
        };
        if limit.offset_from(base) as libc::c_long >= n as libc::c_long
            && n > 0 as libc::c_int
        {
            name = if (*ci).callstatus as libc::c_int
                & (1 as libc::c_int) << 1 as libc::c_int == 0
            {
                b"(temporary)\0" as *const u8 as *const libc::c_char
            } else {
                b"(C temporary)\0" as *const u8 as *const libc::c_char
            };
        } else {
            return 0 as *const libc::c_char
        }
    }
    if !pos.is_null() {
        *pos = base.offset((n - 1 as libc::c_int) as isize);
    }
    return name;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_getlocal(
    mut L: *mut lua_State,
    mut ar: *const lua_Debug,
    mut n: libc::c_int,
) -> *const libc::c_char {
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    if ar.is_null() {
        if !((*((*L).top.p).offset(-(1 as libc::c_int as isize))).val.tt_ as libc::c_int
            == 6 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
                | (1 as libc::c_int) << 6 as libc::c_int)
        {
            name = 0 as *const libc::c_char;
        } else {
            name = luaF_getlocalname(
                (*((*((*L).top.p).offset(-(1 as libc::c_int as isize))).val.value_.gc
                    as *mut GCUnion))
                    .cl
                    .l
                    .p,
                n,
                0 as libc::c_int,
            );
        }
    } else {
        let mut pos: StkId = 0 as StkId;
        name = luaG_findlocal(L, (*ar).i_ci, n, &mut pos);
        if !name.is_null() {
            let mut io1: *mut TValue = &mut (*(*L).top.p).val;
            let mut io2: *const TValue = &mut (*pos).val;
            (*io1).value_ = (*io2).value_;
            (*io1).tt_ = (*io2).tt_;
            (*L).top.p = ((*L).top.p).offset(1);
            (*L).top.p;
        }
    }
    return name;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_setlocal(
    mut L: *mut lua_State,
    mut ar: *const lua_Debug,
    mut n: libc::c_int,
) -> *const libc::c_char {
    let mut pos: StkId = 0 as StkId;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    name = luaG_findlocal(L, (*ar).i_ci, n, &mut pos);
    if !name.is_null() {
        let mut io1: *mut TValue = &mut (*pos).val;
        let mut io2: *const TValue = &mut (*((*L).top.p)
            .offset(-(1 as libc::c_int as isize)))
            .val;
        (*io1).value_ = (*io2).value_;
        (*io1).tt_ = (*io2).tt_;
        (*L).top.p = ((*L).top.p).offset(-1);
        (*L).top.p;
    }
    return name;
}
unsafe extern "C" fn funcinfo(mut ar: *mut lua_Debug, mut cl: *mut Closure) {
    if !(!cl.is_null()
        && (*cl).c.tt as libc::c_int
            == 6 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int)
    {
        (*ar).source = b"=[C]\0" as *const u8 as *const libc::c_char;
        (*ar)
            .srclen = (::core::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        (*ar).linedefined = -(1 as libc::c_int);
        (*ar).lastlinedefined = -(1 as libc::c_int);
        (*ar).what = b"C\0" as *const u8 as *const libc::c_char;
    } else {
        let mut p: *const Proto = (*cl).l.p;
        if !((*p).source).is_null() {
            (*ar).source = ((*(*p).source).contents).as_mut_ptr();
            (*ar)
                .srclen = if (*(*p).source).shrlen as libc::c_int != 0xff as libc::c_int
            {
                (*(*p).source).shrlen as libc::c_ulong
            } else {
                (*(*p).source).u.lnglen
            };
        } else {
            (*ar).source = b"=?\0" as *const u8 as *const libc::c_char;
            (*ar)
                .srclen = (::core::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        }
        (*ar).linedefined = (*p).linedefined;
        (*ar).lastlinedefined = (*p).lastlinedefined;
        (*ar)
            .what = if (*ar).linedefined == 0 as libc::c_int {
            b"main\0" as *const u8 as *const libc::c_char
        } else {
            b"Lua\0" as *const u8 as *const libc::c_char
        };
    }
    luaO_chunkid(((*ar).short_src).as_mut_ptr(), (*ar).source, (*ar).srclen);
}
unsafe extern "C" fn nextline(
    mut p: *const Proto,
    mut currentline: libc::c_int,
    mut pc: libc::c_int,
) -> libc::c_int {
    if *((*p).lineinfo).offset(pc as isize) as libc::c_int != -(0x80 as libc::c_int) {
        return currentline + *((*p).lineinfo).offset(pc as isize) as libc::c_int
    } else {
        return luaG_getfuncline(p, pc)
    };
}
unsafe extern "C" fn collectvalidlines(mut L: *mut lua_State, mut f: *mut Closure) {
    if !(!f.is_null()
        && (*f).c.tt as libc::c_int
            == 6 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int)
    {
        (*(*L).top.p)
            .val
            .tt_ = (0 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int)
            as u8;
        (*L).top.p = ((*L).top.p).offset(1);
        (*L).top.p;
    } else {
        let mut p: *const Proto = (*f).l.p;
        let mut currentline: libc::c_int = (*p).linedefined;
        let mut t: *mut Table = luaH_new(L);
        let mut io: *mut TValue = &mut (*(*L).top.p).val;
        let mut x_: *mut Table = t;
        (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
        (*io)
            .tt_ = (5 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
            | (1 as libc::c_int) << 6 as libc::c_int) as u8;
        (*L).top.p = ((*L).top.p).offset(1);
        (*L).top.p;
        if !((*p).lineinfo).is_null() {
            let mut i: libc::c_int = 0;
            let mut v: TValue = TValue {
                value_: Value { gc: 0 as *mut GCObject },
                tt_: 0,
            };
            v
                .tt_ = (1 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int)
                as u8;
            if (*p).is_vararg == 0 {
                i = 0 as libc::c_int;
            } else {
                currentline = nextline(p, currentline, 0 as libc::c_int);
                i = 1 as libc::c_int;
            }
            while i < (*p).sizelineinfo {
                currentline = nextline(p, currentline, i);
                luaH_setint(L, t, currentline as Integer, &mut v);
                i += 1;
                i;
            }
        }
    };
}
unsafe extern "C" fn getfuncname(
    mut L: *mut lua_State,
    mut ci: *mut CallInfo,
    mut name: *mut *const libc::c_char,
) -> *const libc::c_char {
    if !ci.is_null()
        && (*ci).callstatus as libc::c_int & (1 as libc::c_int) << 5 as libc::c_int == 0
    {
        return funcnamefromcall(L, (*ci).previous, name)
    } else {
        return 0 as *const libc::c_char
    };
}
unsafe extern "C" fn auxgetinfo(
    mut L: *mut lua_State,
    mut what: *const libc::c_char,
    mut ar: *mut lua_Debug,
    mut f: *mut Closure,
    mut ci: *mut CallInfo,
) -> libc::c_int {
    let mut status: libc::c_int = 1 as libc::c_int;
    while *what != 0 {
        match *what as libc::c_int {
            83 => {
                funcinfo(ar, f);
            }
            108 => {
                (*ar)
                    .currentline = if !ci.is_null()
                    && (*ci).callstatus as libc::c_int
                        & (1 as libc::c_int) << 1 as libc::c_int == 0
                {
                    getcurrentline(ci)
                } else {
                    -(1 as libc::c_int)
                };
            }
            117 => {
                (*ar)
                    .nups = (if f.is_null() {
                    0 as libc::c_int
                } else {
                    (*f).c.nupvalues as libc::c_int
                }) as u8;
                if !(!f.is_null()
                    && (*f).c.tt as libc::c_int
                        == 6 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int)
                {
                    (*ar).isvararg = 1 as libc::c_int as libc::c_char;
                    (*ar).nparams = 0 as libc::c_int as u8;
                } else {
                    (*ar).isvararg = (*(*f).l.p).is_vararg as libc::c_char;
                    (*ar).nparams = (*(*f).l.p).numparams;
                }
            }
            116 => {
                (*ar)
                    .istailcall = (if !ci.is_null() {
                    (*ci).callstatus as libc::c_int
                        & (1 as libc::c_int) << 5 as libc::c_int
                } else {
                    0 as libc::c_int
                }) as libc::c_char;
            }
            110 => {
                (*ar).namewhat = getfuncname(L, ci, &mut (*ar).name);
                if ((*ar).namewhat).is_null() {
                    (*ar).namewhat = b"\0" as *const u8 as *const libc::c_char;
                    (*ar).name = 0 as *const libc::c_char;
                }
            }
            114 => {
                if ci.is_null()
                    || (*ci).callstatus as libc::c_int
                        & (1 as libc::c_int) << 8 as libc::c_int == 0
                {
                    (*ar).ntransfer = 0 as libc::c_int as libc::c_ushort;
                    (*ar).ftransfer = (*ar).ntransfer;
                } else {
                    (*ar).ftransfer = (*ci).u2.transferinfo.ftransfer;
                    (*ar).ntransfer = (*ci).u2.transferinfo.ntransfer;
                }
            }
            76 | 102 => {}
            _ => {
                status = 0 as libc::c_int;
            }
        }
        what = what.offset(1);
        what;
    }
    return status;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn lua_getinfo(
    mut L: *mut lua_State,
    mut what: *const libc::c_char,
    mut ar: *mut lua_Debug,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut cl: *mut Closure = 0 as *mut Closure;
    let mut ci: *mut CallInfo = 0 as *mut CallInfo;
    let mut func: *mut TValue = 0 as *mut TValue;
    if *what as libc::c_int == '>' as i32 {
        ci = 0 as *mut CallInfo;
        func = &mut (*((*L).top.p).offset(-(1 as libc::c_int as isize))).val;
        what = what.offset(1);
        what;
        (*L).top.p = ((*L).top.p).offset(-1);
        (*L).top.p;
    } else {
        ci = (*ar).i_ci;
        func = &mut (*(*ci).func.p).val;
    }
    cl = if (*func).tt_ as libc::c_int
        == 6 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
            | (1 as libc::c_int) << 6 as libc::c_int
        || (*func).tt_ as libc::c_int
            == 6 as libc::c_int | (2 as libc::c_int) << 4 as libc::c_int
                | (1 as libc::c_int) << 6 as libc::c_int
    {
        &mut (*((*func).value_.gc as *mut GCUnion)).cl
    } else {
        0 as *mut Closure
    };
    status = auxgetinfo(L, what, ar, cl, ci);
    if !(strchr(what, 'f' as i32)).is_null() {
        let mut io1: *mut TValue = &mut (*(*L).top.p).val;
        let mut io2: *const TValue = func;
        (*io1).value_ = (*io2).value_;
        (*io1).tt_ = (*io2).tt_;
        (*L).top.p = ((*L).top.p).offset(1);
        (*L).top.p;
    }
    if !(strchr(what, 'L' as i32)).is_null() {
        collectvalidlines(L, cl);
    }
    return status;
}
unsafe extern "C" fn filterpc(
    mut pc: libc::c_int,
    mut jmptarget: libc::c_int,
) -> libc::c_int {
    if pc < jmptarget { return -(1 as libc::c_int) } else { return pc };
}
unsafe extern "C" fn findsetreg(
    mut p: *const Proto,
    mut lastpc: libc::c_int,
    mut reg: libc::c_int,
) -> libc::c_int {
    let mut pc: libc::c_int = 0;
    let mut setreg: libc::c_int = -(1 as libc::c_int);
    let mut jmptarget: libc::c_int = 0 as libc::c_int;
    if luaP_opmodes[(*((*p).code).offset(lastpc as isize) >> 0 as libc::c_int
        & !(!(0 as libc::c_int as Instruction) << 7 as libc::c_int) << 0 as libc::c_int)
        as OpCode as usize] as libc::c_int & (1 as libc::c_int) << 7 as libc::c_int != 0
    {
        lastpc -= 1;
        lastpc;
    }
    pc = 0 as libc::c_int;
    while pc < lastpc {
        let mut i: Instruction = *((*p).code).offset(pc as isize);
        let mut op: OpCode = (i >> 0 as libc::c_int
            & !(!(0 as libc::c_int as Instruction) << 7 as libc::c_int)
                << 0 as libc::c_int) as OpCode;
        let mut a: libc::c_int = (i >> 0 as libc::c_int + 7 as libc::c_int
            & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                << 0 as libc::c_int) as libc::c_int;
        let mut change: libc::c_int = 0;
        match op as libc::c_uint {
            8 => {
                let mut b: libc::c_int = (i
                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                        + 1 as libc::c_int
                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                        << 0 as libc::c_int) as libc::c_int;
                change = (a <= reg && reg <= a + b) as libc::c_int;
            }
            76 => {
                change = (reg >= a + 2 as libc::c_int) as libc::c_int;
            }
            68 | 69 => {
                change = (reg >= a) as libc::c_int;
            }
            56 => {
                let mut b_0: libc::c_int = (i >> 0 as libc::c_int + 7 as libc::c_int
                    & !(!(0 as libc::c_int as Instruction)
                        << 8 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int
                            + 8 as libc::c_int) << 0 as libc::c_int) as libc::c_int
                    - (((1 as libc::c_int)
                        << 8 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int
                            + 8 as libc::c_int) - 1 as libc::c_int >> 1 as libc::c_int);
                let mut dest: libc::c_int = pc + 1 as libc::c_int + b_0;
                if dest <= lastpc && dest > jmptarget {
                    jmptarget = dest;
                }
                change = 0 as libc::c_int;
            }
            _ => {
                change = (luaP_opmodes[op as usize] as libc::c_int
                    & (1 as libc::c_int) << 3 as libc::c_int != 0 && reg == a)
                    as libc::c_int;
            }
        }
        if change != 0 {
            setreg = filterpc(pc, jmptarget);
        }
        pc += 1;
        pc;
    }
    return setreg;
}
unsafe extern "C" fn kname(
    mut p: *const Proto,
    mut index: libc::c_int,
    mut name: *mut *const libc::c_char,
) -> *const libc::c_char {
    let mut kvalue: *mut TValue = &mut *((*p).k).offset(index as isize) as *mut TValue;
    if (*kvalue).tt_ as libc::c_int & 0xf as libc::c_int == 4 as libc::c_int {
        *name = ((*((*kvalue).value_.gc as *mut GCUnion)).ts.contents).as_mut_ptr();
        return b"constant\0" as *const u8 as *const libc::c_char;
    } else {
        *name = b"?\0" as *const u8 as *const libc::c_char;
        return 0 as *const libc::c_char;
    };
}
unsafe extern "C" fn basicgetobjname(
    mut p: *const Proto,
    mut ppc: *mut libc::c_int,
    mut reg: libc::c_int,
    mut name: *mut *const libc::c_char,
) -> *const libc::c_char {
    let mut pc: libc::c_int = *ppc;
    *name = luaF_getlocalname(p, reg + 1 as libc::c_int, pc);
    if !(*name).is_null() {
        return strlocal.as_ptr();
    }
    pc = findsetreg(p, pc, reg);
    *ppc = pc;
    if pc != -(1 as libc::c_int) {
        let mut i: Instruction = *((*p).code).offset(pc as isize);
        let mut op: OpCode = (i >> 0 as libc::c_int
            & !(!(0 as libc::c_int as Instruction) << 7 as libc::c_int)
                << 0 as libc::c_int) as OpCode;
        match op as libc::c_uint {
            0 => {
                let mut b: libc::c_int = (i
                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                        + 1 as libc::c_int
                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                        << 0 as libc::c_int) as libc::c_int;
                if b
                    < (i >> 0 as libc::c_int + 7 as libc::c_int
                        & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                            << 0 as libc::c_int) as libc::c_int
                {
                    return basicgetobjname(p, ppc, b, name);
                }
            }
            9 => {
                *name = upvalname(
                    p,
                    (i
                        >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                            + 1 as libc::c_int
                        & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                            << 0 as libc::c_int) as libc::c_int,
                );
                return strupval.as_ptr();
            }
            3 => {
                return kname(
                    p,
                    (i >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                        & !(!(0 as libc::c_int as Instruction)
                            << 8 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int)
                            << 0 as libc::c_int) as libc::c_int,
                    name,
                );
            }
            4 => {
                return kname(
                    p,
                    (*((*p).code).offset((pc + 1 as libc::c_int) as isize)
                        >> 0 as libc::c_int + 7 as libc::c_int
                        & !(!(0 as libc::c_int as Instruction)
                            << 8 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int
                                + 8 as libc::c_int) << 0 as libc::c_int) as libc::c_int,
                    name,
                );
            }
            _ => {}
        }
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn rname(
    mut p: *const Proto,
    mut pc: libc::c_int,
    mut c: libc::c_int,
    mut name: *mut *const libc::c_char,
) {
    let mut what: *const libc::c_char = basicgetobjname(p, &mut pc, c, name);
    if !(!what.is_null() && *what as libc::c_int == 'c' as i32) {
        *name = b"?\0" as *const u8 as *const libc::c_char;
    }
}
unsafe extern "C" fn rkname(
    mut p: *const Proto,
    mut pc: libc::c_int,
    mut i: Instruction,
    mut name: *mut *const libc::c_char,
) {
    let mut c: libc::c_int = (i
        >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int
            + 8 as libc::c_int
        & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int) << 0 as libc::c_int)
        as libc::c_int;
    if (i >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
        & !(!(0 as libc::c_int as Instruction) << 1 as libc::c_int) << 0 as libc::c_int)
        as libc::c_int != 0
    {
        kname(p, c, name);
    } else {
        rname(p, pc, c, name);
    };
}
unsafe extern "C" fn isEnv(
    mut p: *const Proto,
    mut pc: libc::c_int,
    mut i: Instruction,
    mut isup: libc::c_int,
) -> *const libc::c_char {
    let mut t: libc::c_int = (i
        >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int
        & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int) << 0 as libc::c_int)
        as libc::c_int;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    if isup != 0 {
        name = upvalname(p, t);
    } else {
        let mut what: *const libc::c_char = basicgetobjname(p, &mut pc, t, &mut name);
        if what != strlocal.as_ptr() && what != strupval.as_ptr() {
            name = 0 as *const libc::c_char;
        }
    }
    return if !name.is_null()
        && strcmp(name, b"_ENV\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
    {
        b"global\0" as *const u8 as *const libc::c_char
    } else {
        b"field\0" as *const u8 as *const libc::c_char
    };
}
unsafe extern "C" fn getobjname(
    mut p: *const Proto,
    mut lastpc: libc::c_int,
    mut reg: libc::c_int,
    mut name: *mut *const libc::c_char,
) -> *const libc::c_char {
    let mut kind: *const libc::c_char = basicgetobjname(p, &mut lastpc, reg, name);
    if !kind.is_null() {
        return kind
    } else if lastpc != -(1 as libc::c_int) {
        let mut i: Instruction = *((*p).code).offset(lastpc as isize);
        let mut op: OpCode = (i >> 0 as libc::c_int
            & !(!(0 as libc::c_int as Instruction) << 7 as libc::c_int)
                << 0 as libc::c_int) as OpCode;
        match op as libc::c_uint {
            11 => {
                let mut k: libc::c_int = (i
                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                        + 1 as libc::c_int + 8 as libc::c_int
                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                        << 0 as libc::c_int) as libc::c_int;
                kname(p, k, name);
                return isEnv(p, lastpc, i, 1 as libc::c_int);
            }
            12 => {
                let mut k_0: libc::c_int = (i
                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                        + 1 as libc::c_int + 8 as libc::c_int
                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                        << 0 as libc::c_int) as libc::c_int;
                rname(p, lastpc, k_0, name);
                return isEnv(p, lastpc, i, 0 as libc::c_int);
            }
            13 => {
                *name = b"integer index\0" as *const u8 as *const libc::c_char;
                return b"field\0" as *const u8 as *const libc::c_char;
            }
            14 => {
                let mut k_1: libc::c_int = (i
                    >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                        + 1 as libc::c_int + 8 as libc::c_int
                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                        << 0 as libc::c_int) as libc::c_int;
                kname(p, k_1, name);
                return isEnv(p, lastpc, i, 0 as libc::c_int);
            }
            20 => {
                rkname(p, lastpc, i, name);
                return b"method\0" as *const u8 as *const libc::c_char;
            }
            _ => {}
        }
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn funcnamefromcode(
    mut L: *mut lua_State,
    mut p: *const Proto,
    mut pc: libc::c_int,
    mut name: *mut *const libc::c_char,
) -> *const libc::c_char {
    let mut tm: TMS = TM_INDEX;
    let mut i: Instruction = *((*p).code).offset(pc as isize);
    match (i >> 0 as libc::c_int
        & !(!(0 as libc::c_int as Instruction) << 7 as libc::c_int) << 0 as libc::c_int)
        as OpCode as libc::c_uint
    {
        68 | 69 => {
            return getobjname(
                p,
                pc,
                (i >> 0 as libc::c_int + 7 as libc::c_int
                    & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                        << 0 as libc::c_int) as libc::c_int,
                name,
            );
        }
        76 => {
            *name = b"for iterator\0" as *const u8 as *const libc::c_char;
            return b"for iterator\0" as *const u8 as *const libc::c_char;
        }
        20 | 11 | 12 | 13 | 14 => {
            tm = TM_INDEX;
        }
        15 | 16 | 17 | 18 => {
            tm = TM_NEWINDEX;
        }
        46 | 47 | 48 => {
            tm = (i
                >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int
                    + 1 as libc::c_int + 8 as libc::c_int
                & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                    << 0 as libc::c_int) as libc::c_int as TMS;
        }
        49 => {
            tm = TM_UNM;
        }
        50 => {
            tm = TM_BNOT;
        }
        52 => {
            tm = TM_LEN;
        }
        53 => {
            tm = TM_CONCAT;
        }
        57 => {
            tm = TM_EQ;
        }
        58 | 62 | 64 => {
            tm = TM_LT;
        }
        59 | 63 | 65 => {
            tm = TM_LE;
        }
        54 | 70 => {
            tm = TM_CLOSE;
        }
        _ => return 0 as *const libc::c_char,
    }
    *name = ((*(*(*L).l_G).tmname[tm as usize]).contents)
        .as_mut_ptr()
        .offset(2 as libc::c_int as isize);
    return b"metamethod\0" as *const u8 as *const libc::c_char;
}
unsafe extern "C" fn funcnamefromcall(
    mut L: *mut lua_State,
    mut ci: *mut CallInfo,
    mut name: *mut *const libc::c_char,
) -> *const libc::c_char {
    if (*ci).callstatus as libc::c_int & (1 as libc::c_int) << 3 as libc::c_int != 0 {
        *name = b"?\0" as *const u8 as *const libc::c_char;
        return b"hook\0" as *const u8 as *const libc::c_char;
    } else if (*ci).callstatus as libc::c_int & (1 as libc::c_int) << 7 as libc::c_int
        != 0
    {
        *name = b"__gc\0" as *const u8 as *const libc::c_char;
        return b"metamethod\0" as *const u8 as *const libc::c_char;
    } else if (*ci).callstatus as libc::c_int & (1 as libc::c_int) << 1 as libc::c_int
        == 0
    {
        return funcnamefromcode(
            L,
            (*((*(*ci).func.p).val.value_.gc as *mut GCUnion)).cl.l.p,
            currentpc(ci),
            name,
        )
    } else {
        return 0 as *const libc::c_char
    };
}
unsafe extern "C" fn instack(
    mut ci: *mut CallInfo,
    mut o: *const TValue,
) -> libc::c_int {
    let mut pos: libc::c_int = 0;
    let mut base: StkId = ((*ci).func.p).offset(1 as libc::c_int as isize);
    pos = 0 as libc::c_int;
    while base.offset(pos as isize) < (*ci).top.p {
        if o == &mut (*base.offset(pos as isize)).val as *mut TValue as *const TValue {
            return pos;
        }
        pos += 1;
        pos;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn getupvalname(
    mut ci: *mut CallInfo,
    mut o: *const TValue,
    mut name: *mut *const libc::c_char,
) -> *const libc::c_char {
    let mut c: *mut LClosure = &mut (*((*(*ci).func.p).val.value_.gc as *mut GCUnion))
        .cl
        .l;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*c).nupvalues as libc::c_int {
        if (**((*c).upvals).as_mut_ptr().offset(i as isize)).v.p == o as *mut TValue {
            *name = upvalname((*c).p, i);
            return strupval.as_ptr();
        }
        i += 1;
        i;
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn formatvarinfo(
    mut L: *mut lua_State,
    mut kind: *const libc::c_char,
    mut name: *const libc::c_char,
) -> *const libc::c_char {
    if kind.is_null() {
        return b"\0" as *const u8 as *const libc::c_char
    } else {
        return luaO_pushfstring(
            L,
            b" (%s '%s')\0" as *const u8 as *const libc::c_char,
            kind,
            name,
        )
    };
}
unsafe extern "C" fn varinfo(
    mut L: *mut lua_State,
    mut o: *const TValue,
) -> *const libc::c_char {
    let mut ci: *mut CallInfo = (*L).ci;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut kind: *const libc::c_char = 0 as *const libc::c_char;
    if (*ci).callstatus as libc::c_int & (1 as libc::c_int) << 1 as libc::c_int == 0 {
        kind = getupvalname(ci, o, &mut name);
        if kind.is_null() {
            let mut reg: libc::c_int = instack(ci, o);
            if reg >= 0 as libc::c_int {
                kind = getobjname(
                    (*((*(*ci).func.p).val.value_.gc as *mut GCUnion)).cl.l.p,
                    currentpc(ci),
                    reg,
                    &mut name,
                );
            }
        }
    }
    return formatvarinfo(L, kind, name);
}
unsafe extern "C" fn typeerror(
    mut L: *mut lua_State,
    mut o: *const TValue,
    mut op: *const libc::c_char,
    mut extra: *const libc::c_char,
) -> ! {
    let mut t: *const libc::c_char = luaT_objtypename(L, o);
    luaG_runerror(
        L,
        b"attempt to %s a %s value%s\0" as *const u8 as *const libc::c_char,
        op,
        t,
        extra,
    );
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaG_typeerror(
    mut L: *mut lua_State,
    mut o: *const TValue,
    mut op: *const libc::c_char,
) -> ! {
    typeerror(L, o, op, varinfo(L, o));
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaG_callerror(
    mut L: *mut lua_State,
    mut o: *const TValue,
) -> ! {
    let mut ci: *mut CallInfo = (*L).ci;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut kind: *const libc::c_char = funcnamefromcall(L, ci, &mut name);
    let mut extra: *const libc::c_char = if !kind.is_null() {
        formatvarinfo(L, kind, name)
    } else {
        varinfo(L, o)
    };
    typeerror(L, o, b"call\0" as *const u8 as *const libc::c_char, extra);
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaG_forerror(
    mut L: *mut lua_State,
    mut o: *const TValue,
    mut what: *const libc::c_char,
) -> ! {
    luaG_runerror(
        L,
        b"bad 'for' %s (number expected, got %s)\0" as *const u8 as *const libc::c_char,
        what,
        luaT_objtypename(L, o),
    );
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaG_concaterror(
    mut L: *mut lua_State,
    mut p1: *const TValue,
    mut p2: *const TValue,
) -> ! {
    if (*p1).tt_ as libc::c_int & 0xf as libc::c_int == 4 as libc::c_int
        || (*p1).tt_ as libc::c_int & 0xf as libc::c_int == 3 as libc::c_int
    {
        p1 = p2;
    }
    luaG_typeerror(L, p1, b"concatenate\0" as *const u8 as *const libc::c_char);
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaG_opinterror(
    mut L: *mut lua_State,
    mut p1: *const TValue,
    mut p2: *const TValue,
    mut msg: *const libc::c_char,
) -> ! {
    if !((*p1).tt_ as libc::c_int & 0xf as libc::c_int == 3 as libc::c_int) {
        p2 = p1;
    }
    luaG_typeerror(L, p2, msg);
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaG_tointerror(
    mut L: *mut lua_State,
    mut p1: *const TValue,
    mut p2: *const TValue,
) -> ! {
    let mut temp: Integer = 0;
    if luaV_tointegerns(p1, &mut temp, F2Ieq) == 0 {
        p2 = p1;
    }
    luaG_runerror(
        L,
        b"number%s has no integer representation\0" as *const u8 as *const libc::c_char,
        varinfo(L, p2),
    );
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaG_ordererror(
    mut L: *mut lua_State,
    mut p1: *const TValue,
    mut p2: *const TValue,
) -> ! {
    let mut t1: *const libc::c_char = luaT_objtypename(L, p1);
    let mut t2: *const libc::c_char = luaT_objtypename(L, p2);
    if strcmp(t1, t2) == 0 as libc::c_int {
        luaG_runerror(
            L,
            b"attempt to compare two %s values\0" as *const u8 as *const libc::c_char,
            t1,
        );
    } else {
        luaG_runerror(
            L,
            b"attempt to compare %s with %s\0" as *const u8 as *const libc::c_char,
            t1,
            t2,
        );
    };
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaG_addinfo(
    mut L: *mut lua_State,
    mut msg: *const libc::c_char,
    mut src: *mut TString,
    mut line: libc::c_int,
) -> *const libc::c_char {
    let mut buff: [libc::c_char; 60] = [0; 60];
    if !src.is_null() {
        luaO_chunkid(
            buff.as_mut_ptr(),
            ((*src).contents).as_mut_ptr(),
            if (*src).shrlen as libc::c_int != 0xff as libc::c_int {
                (*src).shrlen as libc::c_ulong
            } else {
                (*src).u.lnglen
            },
        );
    } else {
        buff[0 as libc::c_int as usize] = '?' as i32 as libc::c_char;
        buff[1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    }
    return luaO_pushfstring(
        L,
        b"%s:%d: %s\0" as *const u8 as *const libc::c_char,
        buff.as_mut_ptr(),
        line,
        msg,
    );
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaG_errormsg(mut L: *mut lua_State) -> ! {
    if (*L).errfunc != 0 as libc::c_int as libc::c_long {
        let mut errfunc: StkId = ((*L).stack.p as *mut libc::c_char)
            .offset((*L).errfunc as isize) as StkId;
        let mut io1: *mut TValue = &mut (*(*L).top.p).val;
        let mut io2: *const TValue = &mut (*((*L).top.p)
            .offset(-(1 as libc::c_int as isize)))
            .val;
        (*io1).value_ = (*io2).value_;
        (*io1).tt_ = (*io2).tt_;
        let mut io1_0: *mut TValue = &mut (*((*L).top.p)
            .offset(-(1 as libc::c_int as isize)))
            .val;
        let mut io2_0: *const TValue = &mut (*errfunc).val;
        (*io1_0).value_ = (*io2_0).value_;
        (*io1_0).tt_ = (*io2_0).tt_;
        (*L).top.p = ((*L).top.p).offset(1);
        (*L).top.p;
        luaD_callnoyield(
            L,
            ((*L).top.p).offset(-(2 as libc::c_int as isize)),
            1 as libc::c_int,
        );
    }
    luaD_throw(L, 2 as libc::c_int);
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaG_runerror(
    mut L: *mut lua_State,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> ! {
    let mut ci: *mut CallInfo = (*L).ci;
    let mut msg: *const libc::c_char = 0 as *const libc::c_char;
    let mut argp: ::core::ffi::VaListImpl;
    if (*(*L).l_G).GCdebt > 0 as libc::c_int as libc::c_long {
        luaC_step(L);
    }
    argp = args.clone();
    msg = luaO_pushvfstring(L, fmt, argp.as_va_list());
    if (*ci).callstatus as libc::c_int & (1 as libc::c_int) << 1 as libc::c_int == 0 {
        luaG_addinfo(
            L,
            msg,
            (*(*((*(*ci).func.p).val.value_.gc as *mut GCUnion)).cl.l.p).source,
            getcurrentline(ci),
        );
        let mut io1: *mut TValue = &mut (*((*L).top.p)
            .offset(-(2 as libc::c_int as isize)))
            .val;
        let mut io2: *const TValue = &mut (*((*L).top.p)
            .offset(-(1 as libc::c_int as isize)))
            .val;
        (*io1).value_ = (*io2).value_;
        (*io1).tt_ = (*io2).tt_;
        (*L).top.p = ((*L).top.p).offset(-1);
        (*L).top.p;
    }
    luaG_errormsg(L);
}
unsafe extern "C" fn changedline(
    mut p: *const Proto,
    mut oldpc: libc::c_int,
    mut newpc: libc::c_int,
) -> libc::c_int {
    if ((*p).lineinfo).is_null() {
        return 0 as libc::c_int;
    }
    if newpc - oldpc < 128 as libc::c_int / 2 as libc::c_int {
        let mut delta: libc::c_int = 0 as libc::c_int;
        let mut pc: libc::c_int = oldpc;
        loop {
            pc += 1;
            let mut lineinfo: libc::c_int = *((*p).lineinfo).offset(pc as isize)
                as libc::c_int;
            if lineinfo == -(0x80 as libc::c_int) {
                break;
            }
            delta += lineinfo;
            if pc == newpc {
                return (delta != 0 as libc::c_int) as libc::c_int;
            }
        }
    }
    return (luaG_getfuncline(p, oldpc) != luaG_getfuncline(p, newpc)) as libc::c_int;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaG_tracecall(mut L: *mut lua_State) -> libc::c_int {
    let mut ci: *mut CallInfo = (*L).ci;
    let mut p: *mut Proto = (*((*(*ci).func.p).val.value_.gc as *mut GCUnion)).cl.l.p;
    ::core::ptr::write_volatile(
        &mut (*ci).u.l.trap as *mut sig_atomic_t,
        1 as libc::c_int,
    );
    if (*ci).u.l.savedpc == (*p).code as *const Instruction {
        if (*p).is_vararg != 0 {
            return 0 as libc::c_int
        } else if (*ci).callstatus as libc::c_int
            & (1 as libc::c_int) << 6 as libc::c_int == 0
        {
            luaD_hookcall(L, ci);
        }
    }
    return 1 as libc::c_int;
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaG_traceexec(
    mut L: *mut lua_State,
    mut pc: *const Instruction,
) -> libc::c_int {
    let mut ci: *mut CallInfo = (*L).ci;
    let mut mask: u8 = (*L).hookmask as u8;
    let mut p: *const Proto = (*((*(*ci).func.p).val.value_.gc as *mut GCUnion)).cl.l.p;
    let mut counthook: libc::c_int = 0;
    if mask as libc::c_int
        & ((1 as libc::c_int) << 2 as libc::c_int
            | (1 as libc::c_int) << 3 as libc::c_int) == 0
    {
        ::core::ptr::write_volatile(
            &mut (*ci).u.l.trap as *mut sig_atomic_t,
            0 as libc::c_int,
        );
        return 0 as libc::c_int;
    }
    pc = pc.offset(1);
    pc;
    (*ci).u.l.savedpc = pc;
    counthook = (mask as libc::c_int & (1 as libc::c_int) << 3 as libc::c_int != 0
        && {
            (*L).hookcount -= 1;
            (*L).hookcount == 0 as libc::c_int
        }) as libc::c_int;
    if counthook != 0 {
        (*L).hookcount = (*L).basehookcount;
    } else if mask as libc::c_int & (1 as libc::c_int) << 2 as libc::c_int == 0 {
        return 1 as libc::c_int
    }
    if (*ci).callstatus as libc::c_int & (1 as libc::c_int) << 6 as libc::c_int != 0 {
        (*ci)
            .callstatus = ((*ci).callstatus as libc::c_int
            & !((1 as libc::c_int) << 6 as libc::c_int)) as libc::c_ushort;
        return 1 as libc::c_int;
    }
    if !(luaP_opmodes[(*((*ci).u.l.savedpc).offset(-(1 as libc::c_int as isize))
        >> 0 as libc::c_int
        & !(!(0 as libc::c_int as Instruction) << 7 as libc::c_int) << 0 as libc::c_int)
        as OpCode as usize] as libc::c_int & (1 as libc::c_int) << 5 as libc::c_int != 0
        && (*((*ci).u.l.savedpc).offset(-(1 as libc::c_int as isize))
            >> 0 as libc::c_int + 7 as libc::c_int + 8 as libc::c_int + 1 as libc::c_int
            & !(!(0 as libc::c_int as Instruction) << 8 as libc::c_int)
                << 0 as libc::c_int) as libc::c_int == 0 as libc::c_int)
    {
        (*L).top.p = (*ci).top.p;
    }
    if counthook != 0 {
        luaD_hook(
            L,
            3 as libc::c_int,
            -(1 as libc::c_int),
            0 as libc::c_int,
            0 as libc::c_int,
        );
    }
    if mask as libc::c_int & (1 as libc::c_int) << 2 as libc::c_int != 0 {
        let mut oldpc: libc::c_int = if (*L).oldpc < (*p).sizecode {
            (*L).oldpc
        } else {
            0 as libc::c_int
        };
        let mut npci: libc::c_int = pc.offset_from((*p).code) as libc::c_long
            as libc::c_int - 1 as libc::c_int;
        if npci <= oldpc || changedline(p, oldpc, npci) != 0 {
            let mut newline: libc::c_int = luaG_getfuncline(p, npci);
            luaD_hook(L, 2 as libc::c_int, newline, 0 as libc::c_int, 0 as libc::c_int);
        }
        (*L).oldpc = npci;
    }
    if (*L).status as libc::c_int == 1 as libc::c_int {
        if counthook != 0 {
            (*L).hookcount = 1 as libc::c_int;
        }
        (*ci)
            .callstatus = ((*ci).callstatus as libc::c_int
            | (1 as libc::c_int) << 6 as libc::c_int) as libc::c_ushort;
        luaD_throw(L, 1 as libc::c_int);
    }
    return 1 as libc::c_int;
}
