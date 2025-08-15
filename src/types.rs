pub type lu_mem = u64;
pub type lua_KContext = i64;
pub type sig_atomic_t = i32;
pub type Instruction = u32;
pub type l_mem = i64;
pub type F2Imod = u32;
pub const F2Iceil: F2Imod = 2;
pub const F2Ifloor: F2Imod = 1;
pub const F2Ieq: F2Imod = 0;
pub type lua_WarnFunction =
    Option<unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, i32) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: [i64; 8],
    pub __mask_was_saved: i32,
    pub __saved_mask: __sigset_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LongJump {
    pub previous: *mut LongJump,
    pub b: [__jmp_buf_tag; 1],
    pub status: i32,
}
pub const OP_ADD: OpCode = 34;
pub const OP_ADDI: OpCode = 21;
pub const OP_ADDK: OpCode = 22;
pub const OP_BAND: OpCode = 41;
pub const OP_BANDK: OpCode = 29;
pub const OP_BNOT: OpCode = 50;
pub const OP_BOR: OpCode = 42;
pub const OP_BORK: OpCode = 30;
pub const OP_BXOR: OpCode = 43;
pub const OP_BXORK: OpCode = 31;
pub const OP_CALL: OpCode = 68;
pub const OP_CLOSE: OpCode = 54;
pub const OP_CLOSURE: OpCode = 79;
pub const OP_CONCAT: OpCode = 53;
pub const OP_DIV: OpCode = 39;
pub const OP_DIVK: OpCode = 27;
pub const OP_EQ: OpCode = 57;
pub const OP_EQI: OpCode = 61;
pub const OP_EQK: OpCode = 60;
pub const OP_EXTRAARG: OpCode = 82;
pub const OP_FORLOOP: OpCode = 73;
pub const OP_FORPREP: OpCode = 74;
pub const OP_GEI: OpCode = 65;
pub const OP_GETFIELD: OpCode = 14;
pub const OP_GETI: OpCode = 13;
pub const OP_GETTABLE: OpCode = 12;
pub const OP_GETTABUP: OpCode = 11;
pub const OP_GETUPVAL: OpCode = 9;
pub const OP_GTI: OpCode = 64;
pub const OP_IDIV: OpCode = 40;
pub const OP_IDIVK: OpCode = 28;
pub const OP_JMP: OpCode = 56;
pub const OP_LE: OpCode = 59;
pub const OP_LEI: OpCode = 63;
pub const OP_LEN: OpCode = 52;
pub const OP_LFALSESKIP: OpCode = 6;
pub const OP_LOADF: OpCode = 2;
pub const OP_LOADFALSE: OpCode = 5;
pub const OP_LOADI: OpCode = 1;
pub const OP_LOADK: OpCode = 3;
pub const OP_LOADKX: OpCode = 4;
pub const OP_LOADNIL: OpCode = 8;
pub const OP_LOADTRUE: OpCode = 7;
pub const OP_LT: OpCode = 58;
pub const OP_LTI: OpCode = 62;
pub const OP_MMBIN: OpCode = 46;
pub const OP_MMBINI: OpCode = 47;
pub const OP_MMBINK: OpCode = 48;
pub const OP_MOD: OpCode = 37;
pub const OP_MODK: OpCode = 25;
pub const OP_MOVE: OpCode = 0;
pub const OP_MUL: OpCode = 36;
pub const OP_MULK: OpCode = 24;
pub const OP_NEWTABLE: OpCode = 19;
pub const OP_NOT: OpCode = 51;
pub const OP_POW: OpCode = 38;
pub const OP_POWK: OpCode = 26;
pub const OP_RETURN: OpCode = 70;
pub const OP_RETURN0: OpCode = 71;
pub const OP_RETURN1: OpCode = 72;
pub const OP_SELF: OpCode = 20;
pub const OP_SETFIELD: OpCode = 18;
pub const OP_SETI: OpCode = 17;
pub const OP_SETLIST: OpCode = 78;
pub const OP_SETTABLE: OpCode = 16;
pub const OP_SETTABUP: OpCode = 15;
pub const OP_SETUPVAL: OpCode = 10;
pub const OP_SHL: OpCode = 44;
pub const OP_SHLI: OpCode = 33;
pub const OP_SHR: OpCode = 45;
pub const OP_SHRI: OpCode = 32;
pub const OP_SUB: OpCode = 35;
pub const OP_SUBK: OpCode = 23;
pub const OP_TAILCALL: OpCode = 69;
pub const OP_TBC: OpCode = 55;
pub const OP_TEST: OpCode = 66;
pub const OP_TESTSET: OpCode = 67;
pub const OP_TFORCALL: OpCode = 76;
pub const OP_TFORLOOP: OpCode = 77;
pub const OP_TFORPREP: OpCode = 75;
pub const OP_UNM: OpCode = 49;
pub const OP_VARARG: OpCode = 80;
pub const OP_VARARGPREP: OpCode = 81;
pub const TM_ADD: TMS = 6;
pub const TM_BAND: TMS = 13;
pub const TM_BNOT: TMS = 19;
pub const TM_BOR: TMS = 14;
pub const TM_BXOR: TMS = 15;
pub const TM_CALL: TMS = 23;
pub const TM_CLOSE: TMS = 24;
pub const TM_CONCAT: TMS = 22;
pub const TM_DIV: TMS = 11;
pub const TM_EQ: TMS = 5;
pub const TM_GC: TMS = 2;
pub const TM_IDIV: TMS = 12;
pub const TM_INDEX: TMS = 0;
pub const TM_LE: TMS = 21;
pub const TM_LEN: TMS = 4;
pub const TM_LT: TMS = 20;
pub const TM_MOD: TMS = 9;
pub const TM_MODE: TMS = 3;
pub const TM_MUL: TMS = 8;
pub const TM_N: TMS = 25;
pub const TM_NEWINDEX: TMS = 1;
pub const TM_POW: TMS = 10;
pub const TM_SHL: TMS = 16;
pub const TM_SHR: TMS = 17;
pub const TM_SUB: TMS = 7;
pub const TM_UNM: TMS = 18;
pub type Offset = i64;
pub type OpCode = u32;
pub type TMS = u32;
