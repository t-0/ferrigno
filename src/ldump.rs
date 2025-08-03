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
}
pub type ptrdiff_t = libc::c_long;
pub type size_t = libc::c_ulong;
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
pub type lua_Number = libc::c_double;
pub type lua_Integer = libc::c_longlong;
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
pub type lua_Writer = Option::<
    unsafe extern "C" fn(
        *mut lua_State,
        *const libc::c_void,
        size_t,
        *mut libc::c_void,
    ) -> libc::c_int,
>;
pub type ls_byte = libc::c_schar;
#[derive(Copy, Clone)]
#[repr(C)]
pub union UValue {
    pub uv: TValue,
    pub n: lua_Number,
    pub u: libc::c_double,
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
    pub idx: lu_byte,
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
    pub strip: libc::c_int,
    pub status: libc::c_int,
}
unsafe extern "C" fn dumpBlock(
    mut D: *mut DumpState,
    mut b: *const libc::c_void,
    mut size: size_t,
) {
    if (*D).status == 0 as libc::c_int && size > 0 as libc::c_int as libc::c_ulong {
        (*D)
            .status = (Some(((*D).writer).expect("non-null function pointer")))
            .expect("non-null function pointer")((*D).L, b, size, (*D).data);
    }
}
unsafe extern "C" fn dumpByte(mut D: *mut DumpState, mut y: libc::c_int) {
    let mut x: lu_byte = y as lu_byte;
    dumpBlock(
        D,
        &mut x as *mut lu_byte as *const libc::c_void,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<lu_byte>() as libc::c_ulong),
    );
}
unsafe extern "C" fn dumpSize(mut D: *mut DumpState, mut x: size_t) {
    let mut buff: [lu_byte; 10] = [0; 10];
    let mut n: libc::c_int = 0 as libc::c_int;
    loop {
        n += 1;
        buff[(::core::mem::size_of::<size_t>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            .wrapping_add(6 as libc::c_int as libc::c_ulong)
            .wrapping_div(7 as libc::c_int as libc::c_ulong)
            .wrapping_sub(n as libc::c_ulong)
            as usize] = (x & 0x7f as libc::c_int as libc::c_ulong) as lu_byte;
        x >>= 7 as libc::c_int;
        if !(x != 0 as libc::c_int as libc::c_ulong) {
            break;
        }
    }
    buff[(::core::mem::size_of::<size_t>() as libc::c_ulong)
        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
        .wrapping_add(6 as libc::c_int as libc::c_ulong)
        .wrapping_div(7 as libc::c_int as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        as usize] = (buff[(::core::mem::size_of::<size_t>() as libc::c_ulong)
        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
        .wrapping_add(6 as libc::c_int as libc::c_ulong)
        .wrapping_div(7 as libc::c_int as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as usize] as libc::c_int
        | 0x80 as libc::c_int) as lu_byte;
    dumpBlock(
        D,
        buff
            .as_mut_ptr()
            .offset(
                (::core::mem::size_of::<size_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_add(6 as libc::c_int as libc::c_ulong)
                    .wrapping_div(7 as libc::c_int as libc::c_ulong) as isize,
            )
            .offset(-(n as isize)) as *const libc::c_void,
        (n as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<lu_byte>() as libc::c_ulong),
    );
}
unsafe extern "C" fn dumpInt(mut D: *mut DumpState, mut x: libc::c_int) {
    dumpSize(D, x as size_t);
}
unsafe extern "C" fn dumpNumber(mut D: *mut DumpState, mut x: lua_Number) {
    dumpBlock(
        D,
        &mut x as *mut lua_Number as *const libc::c_void,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<lua_Number>() as libc::c_ulong),
    );
}
unsafe extern "C" fn dumpInteger(mut D: *mut DumpState, mut x: lua_Integer) {
    dumpBlock(
        D,
        &mut x as *mut lua_Integer as *const libc::c_void,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<lua_Integer>() as libc::c_ulong),
    );
}
unsafe extern "C" fn dumpString(mut D: *mut DumpState, mut s: *const TString) {
    if s.is_null() {
        dumpSize(D, 0 as libc::c_int as size_t);
    } else {
        let mut size: size_t = if (*s).shrlen as libc::c_int != 0xff as libc::c_int {
            (*s).shrlen as libc::c_ulong
        } else {
            (*s).u.lnglen
        };
        let mut str: *const libc::c_char = ((*s).contents).as_ptr();
        dumpSize(D, size.wrapping_add(1 as libc::c_int as libc::c_ulong));
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
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = (*f).sizek;
    dumpInt(D, n);
    i = 0 as libc::c_int;
    while i < n {
        let mut o: *const TValue = &mut *((*f).k).offset(i as isize) as *mut TValue;
        let mut tt: libc::c_int = (*o).tt_ as libc::c_int & 0x3f as libc::c_int;
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
        i;
    }
}
unsafe extern "C" fn dumpProtos(mut D: *mut DumpState, mut f: *const Proto) {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = (*f).sizep;
    dumpInt(D, n);
    i = 0 as libc::c_int;
    while i < n {
        dumpFunction(D, *((*f).p).offset(i as isize), (*f).source);
        i += 1;
        i;
    }
}
unsafe extern "C" fn dumpUpvalues(mut D: *mut DumpState, mut f: *const Proto) {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = (*f).sizeupvalues;
    dumpInt(D, n);
    i = 0 as libc::c_int;
    while i < n {
        dumpByte(D, (*((*f).upvalues).offset(i as isize)).instack as libc::c_int);
        dumpByte(D, (*((*f).upvalues).offset(i as isize)).idx as libc::c_int);
        dumpByte(D, (*((*f).upvalues).offset(i as isize)).kind as libc::c_int);
        i += 1;
        i;
    }
}
unsafe extern "C" fn dumpDebug(mut D: *mut DumpState, mut f: *const Proto) {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    n = if (*D).strip != 0 { 0 as libc::c_int } else { (*f).sizelineinfo };
    dumpInt(D, n);
    dumpBlock(
        D,
        (*f).lineinfo as *const libc::c_void,
        (n as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<ls_byte>() as libc::c_ulong),
    );
    n = if (*D).strip != 0 { 0 as libc::c_int } else { (*f).sizeabslineinfo };
    dumpInt(D, n);
    i = 0 as libc::c_int;
    while i < n {
        dumpInt(D, (*((*f).abslineinfo).offset(i as isize)).pc);
        dumpInt(D, (*((*f).abslineinfo).offset(i as isize)).line);
        i += 1;
        i;
    }
    n = if (*D).strip != 0 { 0 as libc::c_int } else { (*f).sizelocvars };
    dumpInt(D, n);
    i = 0 as libc::c_int;
    while i < n {
        dumpString(D, (*((*f).locvars).offset(i as isize)).varname);
        dumpInt(D, (*((*f).locvars).offset(i as isize)).startpc);
        dumpInt(D, (*((*f).locvars).offset(i as isize)).endpc);
        i += 1;
        i;
    }
    n = if (*D).strip != 0 { 0 as libc::c_int } else { (*f).sizeupvalues };
    dumpInt(D, n);
    i = 0 as libc::c_int;
    while i < n {
        dumpString(D, (*((*f).upvalues).offset(i as isize)).name);
        i += 1;
        i;
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
    dumpByte(D, (*f).numparams as libc::c_int);
    dumpByte(D, (*f).is_vararg as libc::c_int);
    dumpByte(D, (*f).maxstacksize as libc::c_int);
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
    dumpByte(
        D,
        504 as libc::c_int / 100 as libc::c_int * 16 as libc::c_int
            + 504 as libc::c_int % 100 as libc::c_int,
    );
    dumpByte(D, 0 as libc::c_int);
    dumpBlock(
        D,
        b"\x19\x93\r\n\x1A\n\0" as *const u8 as *const libc::c_char
            as *const libc::c_void,
        (::core::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
            .wrapping_sub(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
    );
    dumpByte(D, ::core::mem::size_of::<Instruction>() as libc::c_ulong as libc::c_int);
    dumpByte(D, ::core::mem::size_of::<lua_Integer>() as libc::c_ulong as libc::c_int);
    dumpByte(D, ::core::mem::size_of::<lua_Number>() as libc::c_ulong as libc::c_int);
    dumpInteger(D, 0x5678 as libc::c_int as lua_Integer);
    dumpNumber(D, 370.5f64);
}
#[unsafe (no_mangle)]
pub unsafe extern "C" fn luaU_dump(
    mut L: *mut lua_State,
    mut f: *const Proto,
    mut w: lua_Writer,
    mut data: *mut libc::c_void,
    mut strip: libc::c_int,
) -> libc::c_int {
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
    D.status = 0 as libc::c_int;
    dumpHeader(&mut D);
    dumpByte(&mut D, (*f).sizeupvalues);
    dumpFunction(&mut D, f, 0 as *mut TString);
    return D.status;
}
