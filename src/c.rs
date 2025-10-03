unsafe extern "C" {
    pub type _IOWideData;
    pub type _IOCodeConvert;
    pub type _IOMarker;
    pub unsafe fn __ctype_b_loc() -> *mut *const u16;
    pub unsafe fn __errno_location() -> *mut i32;
    pub unsafe fn fmod(_: f64, _: f64) -> f64;
    pub unsafe fn _setjmp(_: *mut JumpBufferTag) -> i32;
    pub unsafe fn _longjmp(_: *mut JumpBufferTag, _: i32) -> !;
    pub unsafe fn sigemptyset(__set: *mut SIgnalSet) -> i32;
    pub unsafe fn sigaction(__sig: i32, __act: *const SignalAction, __oact: *mut SignalAction) -> i32;
    pub static mut stdin: *mut libc::FILE;
    pub static mut stdout: *mut libc::FILE;
    pub static mut stderr: *mut libc::FILE;
    pub fn tmpfile() -> *mut libc::FILE;
    pub fn freopen(__filename: *const i8, __modes: *const i8, __stream: *mut libc::FILE) -> *mut libc::FILE;
    pub fn setvbuf(__stream: *mut libc::FILE, __buf: *mut i8, __modes: i32, __n: usize) -> i32;
    pub fn fprintf(_: *mut libc::FILE, _: *const i8, _: ...) -> i32;
    pub fn getc(__stream: *mut libc::FILE) -> i32;
    pub fn getc_unlocked(__stream: *mut libc::FILE) -> i32;
    pub fn fgets(s: *mut i8, __n: i32, __stream: *mut libc::FILE) -> *mut i8;
    pub fn fputs(s: *const i8, __stream: *mut libc::FILE) -> i32;
    pub fn ungetc(__c: i32, __stream: *mut libc::FILE) -> i32;
    pub fn mkstemp(__template: *mut i8) -> i32;
    pub fn clock() -> i64;
    pub fn mktime(tp: *mut TM) -> i64;
    pub fn strftime(s: *mut i8, __maxsize: usize, __format: *const i8, tp: *const TM) -> usize;
    pub fn gmtime_r(timer: *const i64, tp: *mut TM) -> *mut TM;
    pub fn localtime_r(timer: *const i64, tp: *mut TM) -> *mut TM;
    pub fn flockfile(__stream: *mut libc::FILE);
    pub fn funlockfile(__stream: *mut libc::FILE);
}
pub const _ISALPHANUMERIC: u32 = 8;
pub const _ISPUNCTUATION: u32 = 4;
pub const _ISCONTROL: u32 = 2;
pub const _ISGRAPH: u32 = 32768;
pub const _ISSPACE: u32 = 8192;
pub const _ISXDIGIT: u32 = 4096;
pub const _ISDIGIT: u32 = 2048;
pub const _ISALPHA: u32 = 1024;
pub const _ISLOWER: u32 = 512;
pub const _ISUPPER: u32 = 256;
pub type __JumpBuffer = [u32; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SIgnalSet {
    pub __val: [usize; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JumpBufferTag {
    pub __mask_was_saved: i32,
    pub __saved_mask: SIgnalSet,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub _sival_int: i32,
    pub _sival_ptr: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SignalInfo {
    pub _si_signo: i32,
    pub _si_errno: i32,
    pub _si_code: i32,
    pub __pad0: i32,
    pub _sifields: SigInfoA,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union SigInfoA {
    pub _pad: [i32; 28],
    pub _kill: SigInfoAA,
    pub _timer: SigInfoAB,
    pub _rt: SigInfoAC,
    pub _sigchld: SigInfoAD,
    pub _sigfault: SigInfoAE,
    pub _sigpoll: SigInfoAF,
    pub _sigsys: SigInfoAG,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SigInfoAG {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: i32,
    pub _arch: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SigInfoAF {
    pub _si_band: i64,
    pub _si_fd: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SigInfoAE {
    pub _si_addr: *mut libc::c_void,
    pub _si_addr_lsb: i16,
    pub _bounds: SigInfoAEA,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union SigInfoAEA {
    pub _addr_bnd: SigInfoAEAA,
    pub _pkey: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SigInfoAEAA {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SigInfoAD {
    pub si_pid: i32,
    pub si_uid: u32,
    pub si_status: i32,
    pub si_utime: i64,
    pub si_stime: i64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SigInfoAC {
    pub si_pid: i32,
    pub si_uid: u32,
    pub si_sigval: sigval,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SigInfoAB {
    pub si_tid: i32,
    pub si_overrun: i32,
    pub si_sigval: sigval,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SigInfoAA {
    pub si_pid: i32,
    pub si_uid: u32,
}
pub type SignalHandler = Option<unsafe fn(i32) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SignalAction {
    pub __sigaction_handler: SigActionA,
    pub sa_mask: SIgnalSet,
    pub sa_flags: i32,
    pub sa_restorer: Option<unsafe fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union SigActionA {
    pub sa_handler: SignalHandler,
    pub sa_sigaction: Option<unsafe fn(i32, *mut SignalInfo, *mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TM {
    pub tm_sec: i32,
    pub tm_min: i32,
    pub tm_hour: i32,
    pub tm_mday: i32,
    pub tm_mon: i32,
    pub tm_year: i32,
    pub tm_wday: i32,
    pub tm_yday: i32,
    pub tm_isdst: i32,
    pub __tm_gmtoff: i64,
    pub __tm_zone: *const i8,
}
