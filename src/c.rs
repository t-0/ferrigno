unsafe extern "C" {
    pub static mut stdin: *mut libc::FILE;
    pub static mut stdout: *mut libc::FILE;
    pub static mut stderr: *mut libc::FILE;

    pub unsafe fn __ctype_b_loc() -> *mut *const u16;
    pub unsafe fn fmod(_: f64, _: f64) -> f64;
    pub unsafe fn _setjmp(_: *mut JumpBufferTag) -> i32;
    pub unsafe fn _longjmp(_: *mut JumpBufferTag, _: i32) -> !;
    pub fn getc(__stream: *mut libc::FILE) -> i32;
    pub fn getc_unlocked(__stream: *mut libc::FILE) -> i32;
    pub fn mkstemp(__template: *mut i8) -> i32;
    pub fn clock() -> i64;
    pub fn mktime(tp: *mut libc::tm) -> i64;
    pub fn flockfile(__stream: *mut libc::FILE);
    pub fn funlockfile(__stream: *mut libc::FILE);
}
pub const _ISPUNCTUATION: i32 = 4;
pub const _ISCONTROL: i32 = 2;
pub const _ISGRAPH: i32 = 32768;
pub const _ISSPACE: i32 = 8192;
pub const _ISXDIGIT: i32 = 4096;
pub const _ISDIGIT: i32 = 2048;
pub const _ISALPHA: i32 = 1024;
pub const _ISLOWER: i32 = 512;
pub const _ISUPPER: i32 = 256;
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
    pub sa_sigaction: Option<unsafe fn(i32, *mut libc::siginfo_t, *mut libc::c_void) -> ()>,
}
