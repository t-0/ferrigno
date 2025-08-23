#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
unsafe extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub unsafe fn __ctype_b_loc() -> *mut *const u16;
    pub unsafe fn __errno_location() -> *mut i32;
    pub unsafe fn fmod(_: f64, _: f64) -> f64;
    pub unsafe fn _setjmp(_: *mut __jmp_buf_tag) -> i32;
    pub unsafe fn _longjmp(_: *mut __jmp_buf_tag, _: i32) -> !;
    pub unsafe fn sigemptyset(__set: *mut __sigset_t) -> i32;
    pub unsafe fn sigaction(__sig: i32, __act: *const sigaction, __oact: *mut sigaction) -> i32;
    pub static mut stdin: *mut FILE;
    pub static mut stdout: *mut FILE;
    pub static mut stderr: *mut FILE;
    pub fn fclose(__stream: *mut FILE) -> i32;
    pub fn tmpfile() -> *mut FILE;
    pub fn fflush(__stream: *mut FILE) -> i32;
    pub fn fopen(_: *const i8, _: *const i8) -> *mut FILE;
    pub fn freopen(__filename: *const i8, __modes: *const i8, __stream: *mut FILE) -> *mut FILE;
    pub fn setvbuf(__stream: *mut FILE, __buf: *mut i8, __modes: i32, __n: u64) -> i32;
    pub fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    pub fn snprintf(_: *mut i8, _: u64, _: *const i8, _: ...) -> i32;
    pub fn getc(__stream: *mut FILE) -> i32;
    pub fn getc_unlocked(__stream: *mut FILE) -> i32;
    pub fn fgets(s: *mut i8, __n: i32, __stream: *mut FILE) -> *mut i8;
    pub fn fputs(s: *const i8, __stream: *mut FILE) -> i32;
    pub fn ungetc(__c: i32, __stream: *mut FILE) -> i32;
    pub fn fread(_: *mut libc::c_void, _: u64, _: u64, _: *mut FILE) -> u64;
    pub fn fwrite(_: *const libc::c_void, _: u64, _: u64, _: *mut FILE) -> u64;
    pub fn fseeko(__stream: *mut FILE, __off: i64, __whence: i32) -> i32;
    pub fn ftello(__stream: *mut FILE) -> i64;
    pub fn clearerr(__stream: *mut FILE);
    pub fn feof(__stream: *mut FILE) -> i32;
    pub fn ferror(__stream: *mut FILE) -> i32;
    pub fn pclose(__stream: *mut FILE) -> i32;
    pub fn popen(__command: *const i8, __modes: *const i8) -> *mut FILE;
    pub fn flockfile(__stream: *mut FILE);
    pub fn funlockfile(__stream: *mut FILE);
    pub fn strtod(_: *const i8, _: *mut *mut i8) -> f64;
    pub fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    pub fn free(_: *mut libc::c_void);
    pub fn abort() -> !;
    pub fn exit(_: i32) -> !;
    pub fn getenv(__name: *const i8) -> *mut i8;
    pub fn mkstemp(__template: *mut i8) -> i32;
    pub fn abs(_: i32) -> i32;
    pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    pub fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
    pub fn memchr(_: *const libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    pub fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    pub fn strcmp(_: *const i8, _: *const i8) -> i32;
    pub fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    pub fn strcoll(__s1: *const i8, __s2: *const i8) -> i32;
    pub fn strchr(_: *const i8, _: i32) -> *mut i8;
    pub fn strspn(_: *const i8, _: *const i8) -> u64;
    pub fn strpbrk(_: *const i8, _: *const i8) -> *mut i8;
    pub fn strstr(_: *const i8, _: *const i8) -> *mut i8;
    pub fn strlen(_: *const i8) -> u64;
    pub fn strerror(_: i32) -> *mut i8;
    pub fn clock() -> i64;
    pub fn time(timer: *mut i64) -> i64;
    pub fn difftime(time1: i64, time0: i64) -> f64;
    pub fn mktime(tp: *mut tm) -> i64;
    pub fn strftime(s: *mut i8, __maxsize: u64, __format: *const i8, tp: *const tm) -> u64;
    pub fn gmtime_r(timer: *const i64, tp: *mut tm) -> *mut tm;
    pub fn localtime_r(timer: *const i64, tp: *mut tm) -> *mut tm;
    pub fn dlopen(file: *const i8, __mode: i32) -> *mut libc::c_void;
    pub fn dlclose(handle: *mut libc::c_void) -> i32;
    pub fn dlsym(handle: *mut libc::c_void, __name: *const i8) -> *mut libc::c_void;
    pub fn dlerror() -> *mut i8;
    pub fn close(fd: i32) -> i32;
    pub fn isatty(fd: i32) -> i32;
}
pub const _ISalnum: u32 = 8;
pub const _ISpunct: u32 = 4;
pub const _IScntrl: u32 = 2;
pub const _ISgraph: u32 = 32768;
pub const _ISspace: u32 = 8192;
pub const _ISxdigit: u32 = 4096;
pub const _ISdigit: u32 = 2048;
pub const _ISalpha: u32 = 1024;
pub const _ISlower: u32 = 512;
pub const _ISupper: u32 = 256;
pub type __jmp_buf = [u32; 8];
#[derive(Copy, Clone)]
pub struct __sigset_t {
    pub __val: [u64; 16],
}
#[derive(Copy, Clone)]
pub struct __jmp_buf_tag {
    pub __mask_was_saved: i32,
    pub __saved_mask: __sigset_t,
}
#[derive(Copy, Clone)]
pub union sigval {
    pub _sival_int: i32,
    pub _sival_ptr: *mut libc::c_void,
}
#[derive(Copy, Clone)]
pub struct siginfo_t {
    pub _si_signo: i32,
    pub _si_errno: i32,
    pub _si_code: i32,
    pub __pad0: i32,
    pub _sifields: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
pub union C2RustUnnamed_0 {
    pub _pad: [i32; 28],
    pub _kill: C2RustUnnamed_9,
    pub _timer: C2RustUnnamed_8,
    pub _rt: C2RustUnnamed_7,
    pub _sigchld: C2RustUnnamed_6,
    pub _sigfault: C2RustUnnamed_3,
    pub _sigpoll: C2RustUnnamed_2,
    pub _sigsys: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_1 {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: i32,
    pub _arch: u32,
}
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_2 {
    pub _si_band: i64,
    pub _si_fd: i32,
}
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_3 {
    pub _si_addr: *mut libc::c_void,
    pub _si_addr_lsb: i16,
    pub _bounds: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub _addr_bnd: C2RustUnnamed_5,
    pub _pkey: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub si_pid: i32,
    pub si_uid: u32,
    pub si_status: i32,
    pub si_utime: i64,
    pub si_stime: i64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub si_pid: i32,
    pub si_uid: u32,
    pub si_sigval: sigval,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub si_tid: i32,
    pub si_overrun: i32,
    pub si_sigval: sigval,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub si_pid: i32,
    pub si_uid: u32,
}
pub type __sighandler_t = Option<unsafe extern "C" fn(i32) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_10,
    pub sa_mask: __sigset_t,
    pub sa_flags: i32,
    pub sa_restorer: Option<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option<unsafe extern "C" fn(i32, *mut siginfo_t, *mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FILE {
    pub _flags: i32,
    pub _IO_read_ptr: *mut i8,
    pub _IO_read_end: *mut i8,
    pub _IO_read_base: *mut i8,
    pub _IO_write_base: *mut i8,
    pub _IO_write_ptr: *mut i8,
    pub _IO_write_end: *mut i8,
    pub _IO_buf_base: *mut i8,
    pub _IO_buf_end: *mut i8,
    pub _IO_save_base: *mut i8,
    pub _IO_backup_base: *mut i8,
    pub _IO_save_end: *mut i8,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut FILE,
    pub _fileno: i32,
    pub _flags2: i32,
    pub _old_offset: i64,
    pub _cur_column: u16,
    pub _vtable_offset: i8,
    pub _shortbuf: [i8; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: i64,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: u64,
    pub _mode: i32,
    pub _unused2: [i8; 20],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
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
