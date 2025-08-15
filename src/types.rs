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
pub type Offset = i64;
pub type TMS = u32;
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
