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
    pub static mut stdin: *mut FILE;
    pub static mut stdout: *mut FILE;
    pub static mut stderr: *mut FILE;
    pub fn fclose(__stream: *mut FILE) -> i32;
    pub fn tmpfile() -> *mut FILE;
    pub fn fflush(__stream: *mut FILE) -> i32;
    pub fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    pub fn freopen(__filename: *const libc::c_char, __modes: *const libc::c_char, __stream: *mut FILE) -> *mut FILE;
    pub fn setvbuf(__stream: *mut FILE, __buf: *mut libc::c_char, __modes: i32, __n: usize) -> i32;
    pub fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> i32;
    pub fn snprintf(_: *mut libc::c_char, _: usize, _: *const libc::c_char, _: ...) -> i32;
    pub fn getc(__stream: *mut FILE) -> i32;
    pub fn getc_unlocked(__stream: *mut FILE) -> i32;
    pub fn fgets(s: *mut libc::c_char, __n: i32, __stream: *mut FILE) -> *mut libc::c_char;
    pub fn fputs(s: *const libc::c_char, __stream: *mut FILE) -> i32;
    pub fn ungetc(__c: i32, __stream: *mut FILE) -> i32;
    pub fn fread(_: *mut libc::c_void, _: usize, _: usize, _: *mut FILE) -> usize;
    pub fn fwrite(_: *const libc::c_void, _: usize, _: usize, _: *mut FILE) -> usize;
    pub fn fseeko(__stream: *mut FILE, __off: i64, __whence: i32) -> i32;
    pub fn ftello(__stream: *mut FILE) -> i64;
    pub fn clearerr(__stream: *mut FILE);
    pub fn feof(__stream: *mut FILE) -> i32;
    pub fn ferror(__stream: *mut FILE) -> i32;
    pub fn pclose(__stream: *mut FILE) -> i32;
    pub fn popen(__command: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
    pub fn flockfile(__stream: *mut FILE);
    pub fn funlockfile(__stream: *mut FILE);
    pub fn realloc(_: *mut libc::c_void, _: usize) -> *mut libc::c_void;
    pub fn free(_: *mut libc::c_void);
    pub fn abort() -> !;
    pub fn exit(_: i32) -> !;
    pub fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    pub fn mkstemp(__template: *mut libc::c_char) -> i32;
    pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: usize) -> *mut libc::c_void;
    pub fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: usize) -> i32;
    pub fn memchr(_: *const libc::c_void, _: i32, _: usize) -> *mut libc::c_void;
    pub fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> i32;
    pub fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: usize) -> i32;
    pub fn strcoll(__s1: *const libc::c_char, __s2: *const libc::c_char) -> i32;
    pub fn strchr(_: *const libc::c_char, _: i32) -> *mut libc::c_char;
    pub fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> usize;
    pub fn strpbrk(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    pub fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    pub fn strlen(_: *const libc::c_char) -> usize;
    pub fn strerror(_: i32) -> *mut libc::c_char;
    pub fn clock() -> i64;
    pub fn time(timer: *mut i64) -> i64;
    pub fn difftime(time1: i64, time0: i64) -> f64;
    pub fn mktime(tp: *mut TM) -> i64;
    pub fn strftime(s: *mut libc::c_char, __maxsize: usize, __format: *const libc::c_char, tp: *const TM) -> usize;
    pub fn gmtime_r(timer: *const i64, tp: *mut TM) -> *mut TM;
    pub fn localtime_r(timer: *const i64, tp: *mut TM) -> *mut TM;
    pub fn close(fd: i32) -> i32;
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
pub type SignalHandler = Option<unsafe extern "C" fn(i32) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SignalAction {
    pub __sigaction_handler: SigActionA,
    pub sa_mask: SIgnalSet,
    pub sa_flags: i32,
    pub sa_restorer: Option<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union SigActionA {
    pub sa_handler: SignalHandler,
    pub sa_sigaction: Option<unsafe extern "C" fn(i32, *mut SignalInfo, *mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FILE {
    pub _flags: i32,
    pub _io_read_pointer: *mut libc::c_char,
    pub _io_read_end: *mut libc::c_char,
    pub _io_read_base: *mut libc::c_char,
    pub _io_write_base: *mut libc::c_char,
    pub _io_write_pointer: *mut libc::c_char,
    pub _io_write_end: *mut libc::c_char,
    pub _io_buf_base: *mut libc::c_char,
    pub _io_buf_end: *mut libc::c_char,
    pub _io_save_base: *mut libc::c_char,
    pub _io_backup_base: *mut libc::c_char,
    pub _io_save_end: *mut libc::c_char,
    pub _markers: *mut _IOMarker,
    pub _chain: *mut FILE,
    pub _fileno: i32,
    pub _flags2: i32,
    pub _old_offset: i64,
    pub _cur_column: u16,
    pub _vtable_offset: libc::c_char,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: i64,
    pub _codecvt: *mut _IOCodeConvert,
    pub _wide_data: *mut _IOWideData,
    pub _freeres_list: *mut FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: usize,
    pub _mode: i32,
    pub _unused2: [libc::c_char; 20],
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
    pub __tm_zone: *const libc::c_char,
}
