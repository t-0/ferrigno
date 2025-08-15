#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use crate::types::*;
unsafe extern "C" {
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> i32;
    fn strchr(_: *const libc::c_char, _: i32) -> *mut libc::c_char;
    fn luaO_pushvfstring(
        L: *mut lua_State,
        fmt: *const libc::c_char,
        argp: ::core::ffi::VaList,
    ) -> *const libc::c_char;
    fn luaO_pushfstring(L: *mut lua_State, fmt: *const libc::c_char, _: ...)
        -> *const libc::c_char;
    fn luaO_chunkid(out: *mut libc::c_char, source: *const libc::c_char, srclen: u64);
    fn luaT_objtypename(L: *mut lua_State, o: *const TValue) -> *const libc::c_char;
    static luaP_opmodes: [u8; 83];
    fn luaD_hook(L: *mut lua_State, event: i32, line: i32, fTransfer: i32, nTransfer: i32);
    fn luaD_hookcall(L: *mut lua_State, ci: *mut CallInfo);
    fn luaD_callnoyield(L: *mut lua_State, func: StkId, nResults: i32);
    fn luaD_throw(L: *mut lua_State, errcode: i32) -> !;
    fn luaF_getlocalname(func: *const Proto, local_number: i32, pc: i32) -> *const libc::c_char;
    fn luaC_step(L: *mut lua_State);
    fn luaH_setint(L: *mut lua_State, t: *mut Table, key: i64, value: *mut TValue);
    fn luaH_new(L: *mut lua_State) -> *mut Table;
    fn luaV_tointegerns(obj: *const TValue, p: *mut i64, mode: F2Imod) -> i32;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: u32,
    pub fp_offset: u32,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type va_list = __builtin_va_list;

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
    pub errorJmp: *mut LongJump,
    pub base_ci: CallInfo,
    pub hook: lua_Hook,
    pub errfunc: i64,
    pub nCcalls: u32,
    pub oldpc: i32,
    pub basehookcount: i32,
    pub hookcount: i32,
    pub hookmask: sig_atomic_t,
}

pub type lua_Hook = Option<unsafe extern "C" fn(*mut lua_State, *mut lua_Debug) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lua_Debug {
    pub event: i32,
    pub name: *const libc::c_char,
    pub namewhat: *const libc::c_char,
    pub what: *const libc::c_char,
    pub source: *const libc::c_char,
    pub srclen: u64,
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
    pub u: LDebugC2RustUnnamed_1,
    pub u2: LDebugC2RustUnnamed,
    pub nresults: libc::c_short,
    pub callstatus: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union LDebugC2RustUnnamed {
    pub funcidx: i32,
    pub nyield: i32,
    pub nres: i32,
    pub transferinfo: LDebugC2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LDebugC2RustUnnamed_0 {
    pub ftransfer: libc::c_ushort,
    pub ntransfer: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union LDebugC2RustUnnamed_1 {
    pub l: LDebugC2RustUnnamed_3,
    pub c: LDebugC2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LDebugC2RustUnnamed_2 {
    pub k: lua_KFunction,
    pub old_errfunc: i64,
    pub ctx: lua_KContext,
}
pub type lua_KFunction = Option<unsafe extern "C" fn(*mut lua_State, i32, lua_KContext) -> i32>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LDebugC2RustUnnamed_3 {
    pub savedpc: *const Instruction,
    pub trap: sig_atomic_t,
    pub nextraargs: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union StkIdRel {
    pub p: StkId,
    pub offset: i64,
}
pub type StkId = *mut StackValue;
#[derive(Copy, Clone)]
#[repr(C)]
pub union StackValue {
    pub val: TValue,
    pub tbclist: LDebugC2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LDebugC2RustUnnamed_4 {
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
    pub i: i64,
    pub n: f64,
    pub ub: u8,
}

pub type CFunction = Option<unsafe extern "C" fn(*mut lua_State) -> i32>;
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
    pub v: LDebugC2RustUnnamed_7,
    pub u: LDebugC2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union LDebugC2RustUnnamed_5 {
    pub open: LDebugC2RustUnnamed_6,
    pub value: TValue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LDebugC2RustUnnamed_6 {
    pub next: *mut UpVal,
    pub previous: *mut *mut UpVal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union LDebugC2RustUnnamed_7 {
    pub p: *mut TValue,
    pub offset: i64,
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
    pub seed: u32,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TString {
    pub next: *mut GCObject,
    pub tt: u8,
    pub marked: u8,
    pub extra: u8,
    pub shrlen: u8,
    pub hash: u32,
    pub u: LDebugC2RustUnnamed_8,
    pub contents: [libc::c_char; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union LDebugC2RustUnnamed_8 {
    pub lnglen: u64,
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
    pub alimit: u32,
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
pub type lua_Alloc = Option<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void, u64, u64) -> *mut libc::c_void,
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
    pub len: u64,
    pub metatable: *mut Table,
    pub gclist: *mut GCObject,
    pub uv: [UValue; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union UValue {
    pub uv: TValue,
    pub n: f64,
    pub u: f64,
    pub s: *mut libc::c_void,
    pub i: i64,
    pub l: i64,
}
static mut strlocal: [libc::c_char; 6] =
    unsafe { *::core::mem::transmute::<&[u8; 6], &[libc::c_char; 6]>(b"local\0") };
static mut strupval: [libc::c_char; 8] =
    unsafe { *::core::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"upvalue\0") };
unsafe extern "C" fn currentpc(mut ci: *mut CallInfo) -> i32 {
    return ((*ci).u.l.savedpc)
        .offset_from((*(*((*(*ci).func.p).val.value_.gc as *mut GCUnion)).cl.l.p).code)
        as i64 as i32
        - 1i32;
}
unsafe extern "C" fn getbaseline(mut f: *const Proto, mut pc: i32, mut basepc: *mut i32) -> i32 {
    if (*f).sizeabslineinfo == 0i32 || pc < (*((*f).abslineinfo).offset(0i32 as isize)).pc {
        *basepc = -(1i32);
        return (*f).linedefined;
    } else {
        let mut i: i32 = (pc as u32)
            .wrapping_div(128i32 as u32)
            .wrapping_sub(1i32 as u32) as i32;
        while (i + 1i32) < (*f).sizeabslineinfo
            && pc >= (*((*f).abslineinfo).offset((i + 1i32) as isize)).pc
        {
            i += 1;
        }
        *basepc = (*((*f).abslineinfo).offset(i as isize)).pc;
        return (*((*f).abslineinfo).offset(i as isize)).line;
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaG_getfuncline(mut f: *const Proto, mut pc: i32) -> i32 {
    if ((*f).lineinfo).is_null() {
        return -(1i32);
    } else {
        let mut basepc: i32 = 0;
        let mut baseline: i32 = getbaseline(f, pc, &mut basepc);
        loop {
            let fresh0 = basepc;
            basepc = basepc + 1;
            if !(fresh0 < pc) {
                break;
            }
            baseline += *((*f).lineinfo).offset(basepc as isize) as i32;
        }
        return baseline;
    };
}
unsafe extern "C" fn getcurrentline(mut ci: *mut CallInfo) -> i32 {
    return luaG_getfuncline(
        (*((*(*ci).func.p).val.value_.gc as *mut GCUnion)).cl.l.p,
        currentpc(ci),
    );
}
unsafe extern "C" fn settraps(mut ci: *mut CallInfo) {
    while !ci.is_null() {
        if (*ci).callstatus as i32 & (1i32) << 1i32 == 0 {
            ::core::ptr::write_volatile(&mut (*ci).u.l.trap as *mut sig_atomic_t, 1i32);
        }
        ci = (*ci).previous;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_sethook(
    mut L: *mut lua_State,
    mut func: lua_Hook,
    mut mask: i32,
    mut count: i32,
) {
    if func.is_none() || mask == 0i32 {
        mask = 0i32;
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_gethook(mut L: *mut lua_State) -> lua_Hook {
    return (*L).hook;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_gethookmask(mut L: *mut lua_State) -> i32 {
    return (*L).hookmask;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_gethookcount(mut L: *mut lua_State) -> i32 {
    return (*L).basehookcount;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_getstack(
    mut L: *mut lua_State,
    mut level: i32,
    mut ar: *mut lua_Debug,
) -> i32 {
    let mut status: i32 = 0;
    let mut ci: *mut CallInfo = 0 as *mut CallInfo;
    if level < 0i32 {
        return 0i32;
    }
    ci = (*L).ci;
    while level > 0i32 && ci != &mut (*L).base_ci as *mut CallInfo {
        level -= 1;
        ci = (*ci).previous;
    }
    if level == 0i32 && ci != &mut (*L).base_ci as *mut CallInfo {
        status = 1i32;
        (*ar).i_ci = ci;
    } else {
        status = 0i32;
    }
    return status;
}
unsafe extern "C" fn upvalname(mut p: *const Proto, mut uv: i32) -> *const libc::c_char {
    let mut s: *mut TString = (*((*p).upvalues).offset(uv as isize)).name;
    if s.is_null() {
        return b"?\0" as *const u8 as *const libc::c_char;
    } else {
        return ((*s).contents).as_mut_ptr();
    };
}
unsafe extern "C" fn findvararg(
    mut ci: *mut CallInfo,
    mut n: i32,
    mut pos: *mut StkId,
) -> *const libc::c_char {
    if (*(*((*(*ci).func.p).val.value_.gc as *mut GCUnion)).cl.l.p).is_vararg != 0 {
        let mut nextra: i32 = (*ci).u.l.nextraargs;
        if n >= -nextra {
            *pos = ((*ci).func.p)
                .offset(-(nextra as isize))
                .offset(-((n + 1i32) as isize));
            return b"(vararg)\0" as *const u8 as *const libc::c_char;
        }
    }
    return 0 as *const libc::c_char;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaG_findlocal(
    mut L: *mut lua_State,
    mut ci: *mut CallInfo,
    mut n: i32,
    mut pos: *mut StkId,
) -> *const libc::c_char {
    let mut base: StkId = ((*ci).func.p).offset(1i32 as isize);
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    if (*ci).callstatus as i32 & (1i32) << 1i32 == 0 {
        if n < 0i32 {
            return findvararg(ci, n, pos);
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
        if limit.offset_from(base) as i64 >= n as i64 && n > 0i32 {
            name = if (*ci).callstatus as i32 & (1i32) << 1i32 == 0 {
                b"(temporary)\0" as *const u8 as *const libc::c_char
            } else {
                b"(C temporary)\0" as *const u8 as *const libc::c_char
            };
        } else {
            return 0 as *const libc::c_char;
        }
    }
    if !pos.is_null() {
        *pos = base.offset((n - 1i32) as isize);
    }
    return name;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_getlocal(
    mut L: *mut lua_State,
    mut ar: *const lua_Debug,
    mut n: i32,
) -> *const libc::c_char {
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    if ar.is_null() {
        if !((*((*L).top.p).offset(-(1i32 as isize))).val.tt_ as i32
            == 6i32 | (0i32) << 4i32 | (1i32) << 6i32)
        {
            name = 0 as *const libc::c_char;
        } else {
            name = luaF_getlocalname(
                (*((*((*L).top.p).offset(-(1i32 as isize))).val.value_.gc as *mut GCUnion))
                    .cl
                    .l
                    .p,
                n,
                0i32,
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_setlocal(
    mut L: *mut lua_State,
    mut ar: *const lua_Debug,
    mut n: i32,
) -> *const libc::c_char {
    let mut pos: StkId = 0 as StkId;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    name = luaG_findlocal(L, (*ar).i_ci, n, &mut pos);
    if !name.is_null() {
        let mut io1: *mut TValue = &mut (*pos).val;
        let mut io2: *const TValue = &mut (*((*L).top.p).offset(-(1i32 as isize))).val;
        (*io1).value_ = (*io2).value_;
        (*io1).tt_ = (*io2).tt_;
        (*L).top.p = ((*L).top.p).offset(-1);
        (*L).top.p;
    }
    return name;
}
unsafe extern "C" fn funcinfo(mut ar: *mut lua_Debug, mut cl: *mut Closure) {
    if !(!cl.is_null() && (*cl).c.tt as i32 == 6i32 | (0i32) << 4i32) {
        (*ar).source = b"=[C]\0" as *const u8 as *const libc::c_char;
        (*ar).srclen = (::core::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong);
        (*ar).linedefined = -(1i32);
        (*ar).lastlinedefined = -(1i32);
        (*ar).what = b"C\0" as *const u8 as *const libc::c_char;
    } else {
        let mut p: *const Proto = (*cl).l.p;
        if !((*p).source).is_null() {
            (*ar).source = ((*(*p).source).contents).as_mut_ptr();
            (*ar).srclen = if (*(*p).source).shrlen as i32 != 0xff as i32 {
                (*(*p).source).shrlen as libc::c_ulong
            } else {
                (*(*p).source).u.lnglen
            };
        } else {
            (*ar).source = b"=?\0" as *const u8 as *const libc::c_char;
            (*ar).srclen = (::core::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_sub(1i32 as libc::c_ulong);
        }
        (*ar).linedefined = (*p).linedefined;
        (*ar).lastlinedefined = (*p).lastlinedefined;
        (*ar).what = if (*ar).linedefined == 0i32 {
            b"main\0" as *const u8 as *const libc::c_char
        } else {
            b"Lua\0" as *const u8 as *const libc::c_char
        };
    }
    luaO_chunkid(((*ar).short_src).as_mut_ptr(), (*ar).source, (*ar).srclen);
}
unsafe extern "C" fn nextline(mut p: *const Proto, mut currentline: i32, mut pc: i32) -> i32 {
    if *((*p).lineinfo).offset(pc as isize) as i32 != -(0x80 as i32) {
        return currentline + *((*p).lineinfo).offset(pc as isize) as i32;
    } else {
        return luaG_getfuncline(p, pc);
    };
}
unsafe extern "C" fn collectvalidlines(mut L: *mut lua_State, mut f: *mut Closure) {
    if !(!f.is_null() && (*f).c.tt as i32 == 6i32 | (0i32) << 4i32) {
        (*(*L).top.p).val.tt_ = (0i32 | (0i32) << 4i32) as u8;
        (*L).top.p = ((*L).top.p).offset(1);
        (*L).top.p;
    } else {
        let mut p: *const Proto = (*f).l.p;
        let mut currentline: i32 = (*p).linedefined;
        let mut t: *mut Table = luaH_new(L);
        let mut io: *mut TValue = &mut (*(*L).top.p).val;
        let mut x_: *mut Table = t;
        (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
        (*io).tt_ = (5i32 | (0i32) << 4i32 | (1i32) << 6i32) as u8;
        (*L).top.p = ((*L).top.p).offset(1);
        (*L).top.p;
        if !((*p).lineinfo).is_null() {
            let mut i: i32 = 0;
            let mut v: TValue = TValue {
                value_: Value {
                    gc: 0 as *mut GCObject,
                },
                tt_: 0,
            };
            v.tt_ = (1i32 | (1i32) << 4i32) as u8;
            if (*p).is_vararg == 0 {
                i = 0i32;
            } else {
                currentline = nextline(p, currentline, 0i32);
                i = 1i32;
            }
            while i < (*p).sizelineinfo {
                currentline = nextline(p, currentline, i);
                luaH_setint(L, t, currentline as i64, &mut v);
                i += 1;
            }
        }
    };
}
unsafe extern "C" fn getfuncname(
    mut L: *mut lua_State,
    mut ci: *mut CallInfo,
    mut name: *mut *const libc::c_char,
) -> *const libc::c_char {
    if !ci.is_null() && (*ci).callstatus as i32 & (1i32) << 5i32 == 0 {
        return funcnamefromcall(L, (*ci).previous, name);
    } else {
        return 0 as *const libc::c_char;
    };
}
unsafe extern "C" fn auxgetinfo(
    mut L: *mut lua_State,
    mut what: *const libc::c_char,
    mut ar: *mut lua_Debug,
    mut f: *mut Closure,
    mut ci: *mut CallInfo,
) -> i32 {
    let mut status: i32 = 1i32;
    while *what != 0 {
        match *what as i32 {
            83 => {
                funcinfo(ar, f);
            }
            108 => {
                (*ar).currentline =
                    if !ci.is_null() && (*ci).callstatus as i32 & (1i32) << 1i32 == 0 {
                        getcurrentline(ci)
                    } else {
                        -(1i32)
                    };
            }
            117 => {
                (*ar).nups = (if f.is_null() {
                    0i32
                } else {
                    (*f).c.nupvalues as i32
                }) as u8;
                if !(!f.is_null() && (*f).c.tt as i32 == 6i32 | (0i32) << 4i32) {
                    (*ar).isvararg = 1i32 as libc::c_char;
                    (*ar).nparams = 0i32 as u8;
                } else {
                    (*ar).isvararg = (*(*f).l.p).is_vararg as libc::c_char;
                    (*ar).nparams = (*(*f).l.p).numparams;
                }
            }
            116 => {
                (*ar).istailcall = (if !ci.is_null() {
                    (*ci).callstatus as i32 & (1i32) << 5i32
                } else {
                    0i32
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
                if ci.is_null() || (*ci).callstatus as i32 & (1i32) << 8i32 == 0 {
                    (*ar).ntransfer = 0i32 as libc::c_ushort;
                    (*ar).ftransfer = (*ar).ntransfer;
                } else {
                    (*ar).ftransfer = (*ci).u2.transferinfo.ftransfer;
                    (*ar).ntransfer = (*ci).u2.transferinfo.ntransfer;
                }
            }
            76 | 102 => {}
            _ => {
                status = 0i32;
            }
        }
        what = what.offset(1);
    }
    return status;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_getinfo(
    mut L: *mut lua_State,
    mut what: *const libc::c_char,
    mut ar: *mut lua_Debug,
) -> i32 {
    let mut status: i32 = 0;
    let mut cl: *mut Closure = 0 as *mut Closure;
    let mut ci: *mut CallInfo = 0 as *mut CallInfo;
    let mut func: *mut TValue = 0 as *mut TValue;
    if *what as i32 == '>' as i32 {
        ci = 0 as *mut CallInfo;
        func = &mut (*((*L).top.p).offset(-(1i32 as isize))).val;
        what = what.offset(1);
        (*L).top.p = ((*L).top.p).offset(-1);
        (*L).top.p;
    } else {
        ci = (*ar).i_ci;
        func = &mut (*(*ci).func.p).val;
    }
    cl = if (*func).tt_ as i32 == 6i32 | (0i32) << 4i32 | (1i32) << 6i32
        || (*func).tt_ as i32 == 6i32 | (2i32) << 4i32 | (1i32) << 6i32
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
unsafe extern "C" fn filterpc(mut pc: i32, mut jmptarget: i32) -> i32 {
    if pc < jmptarget {
        return -(1i32);
    } else {
        return pc;
    };
}
unsafe extern "C" fn findsetreg(mut p: *const Proto, mut lastpc: i32, mut reg: i32) -> i32 {
    let mut pc: i32 = 0;
    let mut setreg: i32 = -(1i32);
    let mut jmptarget: i32 = 0i32;
    if luaP_opmodes[(*((*p).code).offset(lastpc as isize) >> 0i32
        & !(!(0i32 as Instruction) << 7i32) << 0i32) as OpCode as usize] as i32
        & (1i32) << 7i32
        != 0
    {
        lastpc -= 1;
    }
    pc = 0i32;
    while pc < lastpc {
        let mut i: Instruction = *((*p).code).offset(pc as isize);
        let mut op: OpCode = (i >> 0i32 & !(!(0i32 as Instruction) << 7i32) << 0i32) as OpCode;
        let mut a: i32 = (i >> 0i32 + 7i32 & !(!(0i32 as Instruction) << 8i32) << 0i32) as i32;
        let mut change: i32 = 0;
        match op as u32 {
            8 => {
                let mut b: i32 = (i >> 0i32 + 7i32 + 8i32 + 1i32
                    & !(!(0i32 as Instruction) << 8i32) << 0i32)
                    as i32;
                change = (a <= reg && reg <= a + b) as i32;
            }
            76 => {
                change = (reg >= a + 2i32) as i32;
            }
            68 | 69 => {
                change = (reg >= a) as i32;
            }
            56 => {
                let mut b_0: i32 = (i >> 0i32 + 7i32
                    & !(!(0i32 as Instruction) << 8i32 + 8i32 + 1i32 + 8i32) << 0i32)
                    as i32
                    - (((1i32) << 8i32 + 8i32 + 1i32 + 8i32) - 1i32 >> 1i32);
                let mut dest: i32 = pc + 1i32 + b_0;
                if dest <= lastpc && dest > jmptarget {
                    jmptarget = dest;
                }
                change = 0i32;
            }
            _ => {
                change =
                    (luaP_opmodes[op as usize] as i32 & (1i32) << 3i32 != 0 && reg == a) as i32;
            }
        }
        if change != 0 {
            setreg = filterpc(pc, jmptarget);
        }
        pc += 1;
    }
    return setreg;
}
unsafe extern "C" fn kname(
    mut p: *const Proto,
    mut index: i32,
    mut name: *mut *const libc::c_char,
) -> *const libc::c_char {
    let mut kvalue: *mut TValue = &mut *((*p).k).offset(index as isize) as *mut TValue;
    if (*kvalue).tt_ as i32 & 0xf as i32 == 4i32 {
        *name = ((*((*kvalue).value_.gc as *mut GCUnion)).ts.contents).as_mut_ptr();
        return b"constant\0" as *const u8 as *const libc::c_char;
    } else {
        *name = b"?\0" as *const u8 as *const libc::c_char;
        return 0 as *const libc::c_char;
    };
}
unsafe extern "C" fn basicgetobjname(
    mut p: *const Proto,
    mut ppc: *mut i32,
    mut reg: i32,
    mut name: *mut *const libc::c_char,
) -> *const libc::c_char {
    let mut pc: i32 = *ppc;
    *name = luaF_getlocalname(p, reg + 1i32, pc);
    if !(*name).is_null() {
        return strlocal.as_ptr();
    }
    pc = findsetreg(p, pc, reg);
    *ppc = pc;
    if pc != -(1i32) {
        let mut i: Instruction = *((*p).code).offset(pc as isize);
        let mut op: OpCode = (i >> 0i32 & !(!(0i32 as Instruction) << 7i32) << 0i32) as OpCode;
        match op as u32 {
            0 => {
                let mut b: i32 = (i >> 0i32 + 7i32 + 8i32 + 1i32
                    & !(!(0i32 as Instruction) << 8i32) << 0i32)
                    as i32;
                if b < (i >> 0i32 + 7i32 & !(!(0i32 as Instruction) << 8i32) << 0i32) as i32 {
                    return basicgetobjname(p, ppc, b, name);
                }
            }
            9 => {
                *name = upvalname(
                    p,
                    (i >> 0i32 + 7i32 + 8i32 + 1i32 & !(!(0i32 as Instruction) << 8i32) << 0i32)
                        as i32,
                );
                return strupval.as_ptr();
            }
            3 => {
                return kname(
                    p,
                    (i >> 0i32 + 7i32 + 8i32
                        & !(!(0i32 as Instruction) << 8i32 + 8i32 + 1i32) << 0i32)
                        as i32,
                    name,
                );
            }
            4 => {
                return kname(
                    p,
                    (*((*p).code).offset((pc + 1i32) as isize) >> 0i32 + 7i32
                        & !(!(0i32 as Instruction) << 8i32 + 8i32 + 1i32 + 8i32) << 0i32)
                        as i32,
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
    mut pc: i32,
    mut c: i32,
    mut name: *mut *const libc::c_char,
) {
    let mut what: *const libc::c_char = basicgetobjname(p, &mut pc, c, name);
    if !(!what.is_null() && *what as i32 == 'c' as i32) {
        *name = b"?\0" as *const u8 as *const libc::c_char;
    }
}
unsafe extern "C" fn rkname(
    mut p: *const Proto,
    mut pc: i32,
    mut i: Instruction,
    mut name: *mut *const libc::c_char,
) {
    let mut c: i32 =
        (i >> 0i32 + 7i32 + 8i32 + 1i32 + 8i32 & !(!(0i32 as Instruction) << 8i32) << 0i32) as i32;
    if (i >> 0i32 + 7i32 + 8i32 & !(!(0i32 as Instruction) << 1i32) << 0i32) as i32 != 0 {
        kname(p, c, name);
    } else {
        rname(p, pc, c, name);
    };
}
unsafe extern "C" fn isEnv(
    mut p: *const Proto,
    mut pc: i32,
    mut i: Instruction,
    mut isup: i32,
) -> *const libc::c_char {
    let mut t: i32 =
        (i >> 0i32 + 7i32 + 8i32 + 1i32 & !(!(0i32 as Instruction) << 8i32) << 0i32) as i32;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    if isup != 0 {
        name = upvalname(p, t);
    } else {
        let mut what: *const libc::c_char = basicgetobjname(p, &mut pc, t, &mut name);
        if what != strlocal.as_ptr() && what != strupval.as_ptr() {
            name = 0 as *const libc::c_char;
        }
    }
    return if !name.is_null() && strcmp(name, b"_ENV\0" as *const u8 as *const libc::c_char) == 0i32
    {
        b"global\0" as *const u8 as *const libc::c_char
    } else {
        b"field\0" as *const u8 as *const libc::c_char
    };
}
unsafe extern "C" fn getobjname(
    mut p: *const Proto,
    mut lastpc: i32,
    mut reg: i32,
    mut name: *mut *const libc::c_char,
) -> *const libc::c_char {
    let mut kind: *const libc::c_char = basicgetobjname(p, &mut lastpc, reg, name);
    if !kind.is_null() {
        return kind;
    } else if lastpc != -(1i32) {
        let mut i: Instruction = *((*p).code).offset(lastpc as isize);
        let mut op: OpCode = (i >> 0i32 & !(!(0i32 as Instruction) << 7i32) << 0i32) as OpCode;
        match op as u32 {
            11 => {
                let mut k: i32 = (i >> 0i32 + 7i32 + 8i32 + 1i32 + 8i32
                    & !(!(0i32 as Instruction) << 8i32) << 0i32)
                    as i32;
                kname(p, k, name);
                return isEnv(p, lastpc, i, 1i32);
            }
            12 => {
                let mut k_0: i32 = (i >> 0i32 + 7i32 + 8i32 + 1i32 + 8i32
                    & !(!(0i32 as Instruction) << 8i32) << 0i32)
                    as i32;
                rname(p, lastpc, k_0, name);
                return isEnv(p, lastpc, i, 0i32);
            }
            13 => {
                *name = b"integer index\0" as *const u8 as *const libc::c_char;
                return b"field\0" as *const u8 as *const libc::c_char;
            }
            14 => {
                let mut k_1: i32 = (i >> 0i32 + 7i32 + 8i32 + 1i32 + 8i32
                    & !(!(0i32 as Instruction) << 8i32) << 0i32)
                    as i32;
                kname(p, k_1, name);
                return isEnv(p, lastpc, i, 0i32);
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
    mut pc: i32,
    mut name: *mut *const libc::c_char,
) -> *const libc::c_char {
    let mut tm: TMS = TM_INDEX;
    let mut i: Instruction = *((*p).code).offset(pc as isize);
    match (i >> 0i32 & !(!(0i32 as Instruction) << 7i32) << 0i32) as OpCode as u32 {
        68 | 69 => {
            return getobjname(
                p,
                pc,
                (i >> 0i32 + 7i32 & !(!(0i32 as Instruction) << 8i32) << 0i32) as i32,
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
            tm = (i >> 0i32 + 7i32 + 8i32 + 1i32 + 8i32 & !(!(0i32 as Instruction) << 8i32) << 0i32)
                as i32 as TMS;
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
        .offset(2i32 as isize);
    return b"metamethod\0" as *const u8 as *const libc::c_char;
}
unsafe extern "C" fn funcnamefromcall(
    mut L: *mut lua_State,
    mut ci: *mut CallInfo,
    mut name: *mut *const libc::c_char,
) -> *const libc::c_char {
    if (*ci).callstatus as i32 & (1i32) << 3i32 != 0 {
        *name = b"?\0" as *const u8 as *const libc::c_char;
        return b"hook\0" as *const u8 as *const libc::c_char;
    } else if (*ci).callstatus as i32 & (1i32) << 7i32 != 0 {
        *name = b"__gc\0" as *const u8 as *const libc::c_char;
        return b"metamethod\0" as *const u8 as *const libc::c_char;
    } else if (*ci).callstatus as i32 & (1i32) << 1i32 == 0 {
        return funcnamefromcode(
            L,
            (*((*(*ci).func.p).val.value_.gc as *mut GCUnion)).cl.l.p,
            currentpc(ci),
            name,
        );
    } else {
        return 0 as *const libc::c_char;
    };
}
unsafe extern "C" fn instack(mut ci: *mut CallInfo, mut o: *const TValue) -> i32 {
    let mut pos: i32 = 0;
    let mut base: StkId = ((*ci).func.p).offset(1i32 as isize);
    pos = 0i32;
    while base.offset(pos as isize) < (*ci).top.p {
        if o == &mut (*base.offset(pos as isize)).val as *mut TValue as *const TValue {
            return pos;
        }
        pos += 1;
    }
    return -(1i32);
}
unsafe extern "C" fn getupvalname(
    mut ci: *mut CallInfo,
    mut o: *const TValue,
    mut name: *mut *const libc::c_char,
) -> *const libc::c_char {
    let mut c: *mut LClosure = &mut (*((*(*ci).func.p).val.value_.gc as *mut GCUnion)).cl.l;
    let mut i: i32 = 0;
    i = 0i32;
    while i < (*c).nupvalues as i32 {
        if (**((*c).upvals).as_mut_ptr().offset(i as isize)).v.p == o as *mut TValue {
            *name = upvalname((*c).p, i);
            return strupval.as_ptr();
        }
        i += 1;
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn formatvarinfo(
    mut L: *mut lua_State,
    mut kind: *const libc::c_char,
    mut name: *const libc::c_char,
) -> *const libc::c_char {
    if kind.is_null() {
        return b"\0" as *const u8 as *const libc::c_char;
    } else {
        return luaO_pushfstring(
            L,
            b" (%s '%s')\0" as *const u8 as *const libc::c_char,
            kind,
            name,
        );
    };
}
unsafe extern "C" fn varinfo(mut L: *mut lua_State, mut o: *const TValue) -> *const libc::c_char {
    let mut ci: *mut CallInfo = (*L).ci;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut kind: *const libc::c_char = 0 as *const libc::c_char;
    if (*ci).callstatus as i32 & (1i32) << 1i32 == 0 {
        kind = getupvalname(ci, o, &mut name);
        if kind.is_null() {
            let mut reg: i32 = instack(ci, o);
            if reg >= 0i32 {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaG_typeerror(
    mut L: *mut lua_State,
    mut o: *const TValue,
    mut op: *const libc::c_char,
) -> ! {
    typeerror(L, o, op, varinfo(L, o));
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaG_callerror(mut L: *mut lua_State, mut o: *const TValue) -> ! {
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
#[unsafe(no_mangle)]
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaG_concaterror(
    mut L: *mut lua_State,
    mut p1: *const TValue,
    mut p2: *const TValue,
) -> ! {
    if (*p1).tt_ as i32 & 0xf as i32 == 4i32 || (*p1).tt_ as i32 & 0xf as i32 == 3i32 {
        p1 = p2;
    }
    luaG_typeerror(L, p1, b"concatenate\0" as *const u8 as *const libc::c_char);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaG_opinterror(
    mut L: *mut lua_State,
    mut p1: *const TValue,
    mut p2: *const TValue,
    mut msg: *const libc::c_char,
) -> ! {
    if !((*p1).tt_ as i32 & 0xf as i32 == 3i32) {
        p2 = p1;
    }
    luaG_typeerror(L, p2, msg);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaG_tointerror(
    mut L: *mut lua_State,
    mut p1: *const TValue,
    mut p2: *const TValue,
) -> ! {
    let mut temp: i64 = 0;
    if luaV_tointegerns(p1, &mut temp, F2Ieq) == 0 {
        p2 = p1;
    }
    luaG_runerror(
        L,
        b"number%s has no integer representation\0" as *const u8 as *const libc::c_char,
        varinfo(L, p2),
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaG_ordererror(
    mut L: *mut lua_State,
    mut p1: *const TValue,
    mut p2: *const TValue,
) -> ! {
    let mut t1: *const libc::c_char = luaT_objtypename(L, p1);
    let mut t2: *const libc::c_char = luaT_objtypename(L, p2);
    if strcmp(t1, t2) == 0i32 {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaG_addinfo(
    mut L: *mut lua_State,
    mut msg: *const libc::c_char,
    mut src: *mut TString,
    mut line: i32,
) -> *const libc::c_char {
    let mut buff: [libc::c_char; 60] = [0; 60];
    if !src.is_null() {
        luaO_chunkid(
            buff.as_mut_ptr(),
            ((*src).contents).as_mut_ptr(),
            if (*src).shrlen as i32 != 0xff as i32 {
                (*src).shrlen as libc::c_ulong
            } else {
                (*src).u.lnglen
            },
        );
    } else {
        buff[0i32 as usize] = '?' as i32 as libc::c_char;
        buff[1i32 as usize] = '\0' as i32 as libc::c_char;
    }
    return luaO_pushfstring(
        L,
        b"%s:%d: %s\0" as *const u8 as *const libc::c_char,
        buff.as_mut_ptr(),
        line,
        msg,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaG_errormsg(mut L: *mut lua_State) -> ! {
    if (*L).errfunc != 0i32 as i64 {
        let mut errfunc: StkId =
            ((*L).stack.p as *mut libc::c_char).offset((*L).errfunc as isize) as StkId;
        let mut io1: *mut TValue = &mut (*(*L).top.p).val;
        let mut io2: *const TValue = &mut (*((*L).top.p).offset(-(1i32 as isize))).val;
        (*io1).value_ = (*io2).value_;
        (*io1).tt_ = (*io2).tt_;
        let mut io1_0: *mut TValue = &mut (*((*L).top.p).offset(-(1i32 as isize))).val;
        let mut io2_0: *const TValue = &mut (*errfunc).val;
        (*io1_0).value_ = (*io2_0).value_;
        (*io1_0).tt_ = (*io2_0).tt_;
        (*L).top.p = ((*L).top.p).offset(1);
        (*L).top.p;
        luaD_callnoyield(L, ((*L).top.p).offset(-(2i32 as isize)), 1i32);
    }
    luaD_throw(L, 2i32);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaG_runerror(
    mut L: *mut lua_State,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> ! {
    let mut ci: *mut CallInfo = (*L).ci;
    let mut msg: *const libc::c_char = 0 as *const libc::c_char;
    let mut argp: ::core::ffi::VaListImpl;
    if (*(*L).l_G).GCdebt > 0i32 as i64 {
        luaC_step(L);
    }
    argp = args.clone();
    msg = luaO_pushvfstring(L, fmt, argp.as_va_list());
    if (*ci).callstatus as i32 & (1i32) << 1i32 == 0 {
        luaG_addinfo(
            L,
            msg,
            (*(*((*(*ci).func.p).val.value_.gc as *mut GCUnion)).cl.l.p).source,
            getcurrentline(ci),
        );
        let mut io1: *mut TValue = &mut (*((*L).top.p).offset(-(2i32 as isize))).val;
        let mut io2: *const TValue = &mut (*((*L).top.p).offset(-(1i32 as isize))).val;
        (*io1).value_ = (*io2).value_;
        (*io1).tt_ = (*io2).tt_;
        (*L).top.p = ((*L).top.p).offset(-1);
        (*L).top.p;
    }
    luaG_errormsg(L);
}
unsafe extern "C" fn changedline(mut p: *const Proto, mut oldpc: i32, mut newpc: i32) -> i32 {
    if ((*p).lineinfo).is_null() {
        return 0i32;
    }
    if newpc - oldpc < 128i32 / 2i32 {
        let mut delta: i32 = 0i32;
        let mut pc: i32 = oldpc;
        loop {
            pc += 1;
            let mut lineinfo: i32 = *((*p).lineinfo).offset(pc as isize) as i32;
            if lineinfo == -(0x80 as i32) {
                break;
            }
            delta += lineinfo;
            if pc == newpc {
                return (delta != 0i32) as i32;
            }
        }
    }
    return (luaG_getfuncline(p, oldpc) != luaG_getfuncline(p, newpc)) as i32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaG_tracecall(mut L: *mut lua_State) -> i32 {
    let mut ci: *mut CallInfo = (*L).ci;
    let mut p: *mut Proto = (*((*(*ci).func.p).val.value_.gc as *mut GCUnion)).cl.l.p;
    ::core::ptr::write_volatile(&mut (*ci).u.l.trap as *mut sig_atomic_t, 1i32);
    if (*ci).u.l.savedpc == (*p).code as *const Instruction {
        if (*p).is_vararg != 0 {
            return 0i32;
        } else if (*ci).callstatus as i32 & (1i32) << 6i32 == 0 {
            luaD_hookcall(L, ci);
        }
    }
    return 1i32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaG_traceexec(mut L: *mut lua_State, mut pc: *const Instruction) -> i32 {
    let mut ci: *mut CallInfo = (*L).ci;
    let mut mask: u8 = (*L).hookmask as u8;
    let mut p: *const Proto = (*((*(*ci).func.p).val.value_.gc as *mut GCUnion)).cl.l.p;
    let mut counthook: i32 = 0;
    if mask as i32 & ((1i32) << 2i32 | (1i32) << 3i32) == 0 {
        ::core::ptr::write_volatile(&mut (*ci).u.l.trap as *mut sig_atomic_t, 0i32);
        return 0i32;
    }
    pc = pc.offset(1);
    (*ci).u.l.savedpc = pc;
    counthook = (mask as i32 & (1i32) << 3i32 != 0 && {
        (*L).hookcount -= 1;
        (*L).hookcount == 0i32
    }) as i32;
    if counthook != 0 {
        (*L).hookcount = (*L).basehookcount;
    } else if mask as i32 & (1i32) << 2i32 == 0 {
        return 1i32;
    }
    if (*ci).callstatus as i32 & (1i32) << 6i32 != 0 {
        (*ci).callstatus = ((*ci).callstatus as i32 & !((1i32) << 6i32)) as libc::c_ushort;
        return 1i32;
    }
    if !(luaP_opmodes[(*((*ci).u.l.savedpc).offset(-(1i32 as isize)) >> 0i32
        & !(!(0i32 as Instruction) << 7i32) << 0i32) as OpCode as usize] as i32
        & (1i32) << 5i32
        != 0
        && (*((*ci).u.l.savedpc).offset(-(1i32 as isize)) >> 0i32 + 7i32 + 8i32 + 1i32
            & !(!(0i32 as Instruction) << 8i32) << 0i32) as i32
            == 0i32)
    {
        (*L).top.p = (*ci).top.p;
    }
    if counthook != 0 {
        luaD_hook(L, 3i32, -(1i32), 0i32, 0i32);
    }
    if mask as i32 & (1i32) << 2i32 != 0 {
        let mut oldpc: i32 = if (*L).oldpc < (*p).sizecode {
            (*L).oldpc
        } else {
            0i32
        };
        let mut npci: i32 = pc.offset_from((*p).code) as i64 as i32 - 1i32;
        if npci <= oldpc || changedline(p, oldpc, npci) != 0 {
            let mut newline: i32 = luaG_getfuncline(p, npci);
            luaD_hook(L, 2i32, newline, 0i32, 0i32);
        }
        (*L).oldpc = npci;
    }
    if (*L).status as i32 == 1i32 {
        if counthook != 0 {
            (*L).hookcount = 1i32;
        }
        (*ci).callstatus = ((*ci).callstatus as i32 | (1i32) << 6i32) as libc::c_ushort;
        luaD_throw(L, 1i32);
    }
    return 1i32;
}
