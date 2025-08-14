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
pub type sig_atomic_t = i32;

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
    pub old_errfunc: i64,
    pub ctx: lua_KContext,
}
pub type lua_KFunction = Option<unsafe extern "C" fn(*mut lua_State, i32, lua_KContext) -> i32>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub savedpc: *const Instruction,
    pub trap: sig_atomic_t,
    pub nextraargs: i32,
}
pub type Instruction = u32;
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
pub type lua_WarnFunction =
    Option<unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, i32) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TString {
    pub next: *mut GCObject,
    pub tt: u8,
    pub marked: u8,
    pub extra: u8,
    pub shrlen: u8,
    pub hash: u32,
    pub u: C2RustUnnamed_8,
    pub contents: [libc::c_char; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
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
pub type lu_mem = u64;
pub type l_mem = i64;
pub type lua_Alloc = Option<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void, u64, u64) -> *mut libc::c_void,
>;
pub type lua_Writer = Option<
    unsafe extern "C" fn(*mut lua_State, *const libc::c_void, u64, *mut libc::c_void) -> i32,
>;
pub type ls_byte = libc::c_schar;
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
    pub startpc: i32,
    pub endpc: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AbsLineInfo {
    pub pc: i32,
    pub line: i32,
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
pub struct DumpState {
    pub L: *mut lua_State,
    pub writer: lua_Writer,
    pub data: *mut libc::c_void,
    pub strip: i32,
    pub status: i32,
}
unsafe extern "C" fn dumpBlock(mut D: *mut DumpState, mut b: *const libc::c_void, mut size: u64) {
    if (*D).status == 0i32 && size > 0i32 as libc::c_ulong {
        (*D).status = (Some(((*D).writer).expect("non-null function pointer")))
            .expect("non-null function pointer")((*D).L, b, size, (*D).data);
    }
}
unsafe extern "C" fn dumpByte(mut D: *mut DumpState, mut y: i32) {
    let mut x: u8 = y as u8;
    dumpBlock(
        D,
        &mut x as *mut u8 as *const libc::c_void,
        (1i32 as libc::c_ulong).wrapping_mul(::core::mem::size_of::<u8>() as libc::c_ulong),
    );
}
unsafe extern "C" fn dumpSize(mut D: *mut DumpState, mut x: u64) {
    let mut buff: [u8; 10] = [0; 10];
    let mut n: i32 = 0i32;
    loop {
        n += 1;
        buff[(::core::mem::size_of::<u64>() as libc::c_ulong)
            .wrapping_mul(8i32 as libc::c_ulong)
            .wrapping_add(6i32 as libc::c_ulong)
            .wrapping_div(7i32 as libc::c_ulong)
            .wrapping_sub(n as libc::c_ulong) as usize] = (x & 0x7f as i32 as libc::c_ulong) as u8;
        x >>= 7i32;
        if !(x != 0i32 as libc::c_ulong) {
            break;
        }
    }
    buff[(::core::mem::size_of::<u64>() as libc::c_ulong)
        .wrapping_mul(8i32 as libc::c_ulong)
        .wrapping_add(6i32 as libc::c_ulong)
        .wrapping_div(7i32 as libc::c_ulong)
        .wrapping_sub(1i32 as libc::c_ulong) as usize] =
        (buff[(::core::mem::size_of::<u64>() as libc::c_ulong)
            .wrapping_mul(8i32 as libc::c_ulong)
            .wrapping_add(6i32 as libc::c_ulong)
            .wrapping_div(7i32 as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong) as usize] as i32
            | 0x80 as i32) as u8;
    dumpBlock(
        D,
        buff.as_mut_ptr()
            .offset(
                (::core::mem::size_of::<u64>() as libc::c_ulong)
                    .wrapping_mul(8i32 as libc::c_ulong)
                    .wrapping_add(6i32 as libc::c_ulong)
                    .wrapping_div(7i32 as libc::c_ulong) as isize,
            )
            .offset(-(n as isize)) as *const libc::c_void,
        (n as libc::c_ulong).wrapping_mul(::core::mem::size_of::<u8>() as libc::c_ulong),
    );
}
unsafe extern "C" fn dumpInt(mut D: *mut DumpState, mut x: i32) {
    dumpSize(D, x as u64);
}
unsafe extern "C" fn dumpNumber(mut D: *mut DumpState, mut x: f64) {
    dumpBlock(
        D,
        &mut x as *mut f64 as *const libc::c_void,
        (1i32 as libc::c_ulong).wrapping_mul(::core::mem::size_of::<f64>() as libc::c_ulong),
    );
}
unsafe extern "C" fn dumpInteger(mut D: *mut DumpState, mut x: i64) {
    dumpBlock(
        D,
        &mut x as *mut i64 as *const libc::c_void,
        (1i32 as libc::c_ulong).wrapping_mul(::core::mem::size_of::<i64>() as libc::c_ulong),
    );
}
unsafe extern "C" fn dumpString(mut D: *mut DumpState, mut s: *const TString) {
    if s.is_null() {
        dumpSize(D, 0i32 as u64);
    } else {
        let mut size: u64 = if (*s).shrlen as i32 != 0xff as i32 {
            (*s).shrlen as libc::c_ulong
        } else {
            (*s).u.lnglen
        };
        let mut str: *const libc::c_char = ((*s).contents).as_ptr();
        dumpSize(D, size.wrapping_add(1i32 as libc::c_ulong));
        dumpBlock(
            D,
            str as *const libc::c_void,
            size.wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
        );
    };
}
unsafe extern "C" fn dumpCode(mut D: *mut DumpState, mut f: *const Proto) {
    dumpInt(D, (*f).sizecode);
    dumpBlock(
        D,
        (*f).code as *const libc::c_void,
        ((*f).sizecode as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<Instruction>() as libc::c_ulong),
    );
}
unsafe extern "C" fn dumpConstants(mut D: *mut DumpState, mut f: *const Proto) {
    let mut i: i32 = 0;
    let mut n: i32 = (*f).sizek;
    dumpInt(D, n);
    i = 0i32;
    while i < n {
        let mut o: *const TValue = &mut *((*f).k).offset(i as isize) as *mut TValue;
        let mut tt: i32 = (*o).tt_ as i32 & 0x3f as i32;
        dumpByte(D, tt);
        match tt {
            19 => {
                dumpNumber(D, (*o).value_.n);
            }
            3 => {
                dumpInteger(D, (*o).value_.i);
            }
            4 | 20 => {
                dumpString(D, &mut (*((*o).value_.gc as *mut GCUnion)).ts);
            }
            _ => {}
        }
        i += 1;
    }
}
unsafe extern "C" fn dumpProtos(mut D: *mut DumpState, mut f: *const Proto) {
    let mut i: i32 = 0;
    let mut n: i32 = (*f).sizep;
    dumpInt(D, n);
    i = 0i32;
    while i < n {
        dumpFunction(D, *((*f).p).offset(i as isize), (*f).source);
        i += 1;
    }
}
unsafe extern "C" fn dumpUpvalues(mut D: *mut DumpState, mut f: *const Proto) {
    let mut i: i32 = 0;
    let mut n: i32 = (*f).sizeupvalues;
    dumpInt(D, n);
    i = 0i32;
    while i < n {
        dumpByte(D, (*((*f).upvalues).offset(i as isize)).instack as i32);
        dumpByte(D, (*((*f).upvalues).offset(i as isize)).index as i32);
        dumpByte(D, (*((*f).upvalues).offset(i as isize)).kind as i32);
        i += 1;
    }
}
unsafe extern "C" fn dumpDebug(mut D: *mut DumpState, mut f: *const Proto) {
    let mut i: i32 = 0;
    let mut n: i32 = 0;
    n = if (*D).strip != 0 {
        0i32
    } else {
        (*f).sizelineinfo
    };
    dumpInt(D, n);
    dumpBlock(
        D,
        (*f).lineinfo as *const libc::c_void,
        (n as libc::c_ulong).wrapping_mul(::core::mem::size_of::<ls_byte>() as libc::c_ulong),
    );
    n = if (*D).strip != 0 {
        0i32
    } else {
        (*f).sizeabslineinfo
    };
    dumpInt(D, n);
    i = 0i32;
    while i < n {
        dumpInt(D, (*((*f).abslineinfo).offset(i as isize)).pc);
        dumpInt(D, (*((*f).abslineinfo).offset(i as isize)).line);
        i += 1;
    }
    n = if (*D).strip != 0 {
        0i32
    } else {
        (*f).sizelocvars
    };
    dumpInt(D, n);
    i = 0i32;
    while i < n {
        dumpString(D, (*((*f).locvars).offset(i as isize)).varname);
        dumpInt(D, (*((*f).locvars).offset(i as isize)).startpc);
        dumpInt(D, (*((*f).locvars).offset(i as isize)).endpc);
        i += 1;
    }
    n = if (*D).strip != 0 {
        0i32
    } else {
        (*f).sizeupvalues
    };
    dumpInt(D, n);
    i = 0i32;
    while i < n {
        dumpString(D, (*((*f).upvalues).offset(i as isize)).name);
        i += 1;
    }
}
unsafe extern "C" fn dumpFunction(
    mut D: *mut DumpState,
    mut f: *const Proto,
    mut psource: *mut TString,
) {
    if (*D).strip != 0 || (*f).source == psource {
        dumpString(D, 0 as *const TString);
    } else {
        dumpString(D, (*f).source);
    }
    dumpInt(D, (*f).linedefined);
    dumpInt(D, (*f).lastlinedefined);
    dumpByte(D, (*f).numparams as i32);
    dumpByte(D, (*f).is_vararg as i32);
    dumpByte(D, (*f).maxstacksize as i32);
    dumpCode(D, f);
    dumpConstants(D, f);
    dumpUpvalues(D, f);
    dumpProtos(D, f);
    dumpDebug(D, f);
}
unsafe extern "C" fn dumpHeader(mut D: *mut DumpState) {
    dumpBlock(
        D,
        b"\x1BLua\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        (::core::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
            .wrapping_sub(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
    );
    dumpByte(D, 504i32 / 100i32 * 16i32 + 504i32 % 100i32);
    dumpByte(D, 0i32);
    dumpBlock(
        D,
        b"\x19\x93\r\n\x1A\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        (::core::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
            .wrapping_sub(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
    );
    dumpByte(
        D,
        ::core::mem::size_of::<Instruction>() as libc::c_ulong as i32,
    );
    dumpByte(D, ::core::mem::size_of::<i64>() as libc::c_ulong as i32);
    dumpByte(D, ::core::mem::size_of::<f64>() as libc::c_ulong as i32);
    dumpInteger(D, 0x5678 as i32 as i64);
    dumpNumber(D, 370.5f64);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaU_dump(
    mut L: *mut lua_State,
    mut f: *const Proto,
    mut w: lua_Writer,
    mut data: *mut libc::c_void,
    mut strip: i32,
) -> i32 {
    let mut D: DumpState = DumpState {
        L: 0 as *mut lua_State,
        writer: None,
        data: 0 as *mut libc::c_void,
        strip: 0,
        status: 0,
    };
    D.L = L;
    D.writer = w;
    D.data = data;
    D.strip = strip;
    D.status = 0i32;
    dumpHeader(&mut D);
    dumpByte(&mut D, (*f).sizeupvalues);
    dumpFunction(&mut D, f, 0 as *mut TString);
    return D.status;
}
