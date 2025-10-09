unsafe extern "C" {
    pub static mut stdin: *mut libc::FILE;
    pub static mut stdout: *mut libc::FILE;
    pub static mut stderr: *mut libc::FILE;
    pub unsafe fn _setjmp(_: *mut JumpBufferTag) -> i32;
    pub unsafe fn _longjmp(_: *mut JumpBufferTag, _: i32) -> !;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SignalSet {
    pub __val: [usize; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JumpBufferTag {
    m_mask_was_saved: i32,
    m_saved_mask: SignalSet,
}
impl JumpBufferTag {
    pub fn new() -> Self {
        JumpBufferTag { m_mask_was_saved: 0, m_saved_mask: SignalSet { __val: [0; 16] } }
    }
}
pub type SignalHandler = Option<unsafe fn(i32) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SignalAction {
    pub sa_handler: SigActionA,
    pub sa_mask: SignalSet,
    pub sa_flags: i32,
    pub sa_restorer: Option<unsafe fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union SigActionA {
    pub sa_handler: SignalHandler,
    pub sa_sigaction: Option<unsafe fn(i32, *mut libc::siginfo_t, *mut libc::c_void) -> ()>,
}
