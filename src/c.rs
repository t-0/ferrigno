use std::ptr::*;
use crate::functions::*;
unsafe extern "C" {
    #[cfg_attr(target_os = "macos", link_name = "__stdinp")]
    pub static mut stdin: *mut libc::FILE;
    #[cfg_attr(target_os = "macos", link_name = "__stdoutp")]
    pub static mut stdout: *mut libc::FILE;
    #[cfg_attr(target_os = "macos", link_name = "__stderrp")]
    pub static mut stderr: *mut libc::FILE;
    pub unsafe fn _setjmp(_: *mut JumpBuffer) -> i32;
    pub unsafe fn _longjmp(_: *mut JumpBuffer, _: i32) -> !;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SignalSet {
    m_value: [usize; 16],
}
impl SignalSet {
    pub fn new() -> Self {
        SignalSet {
            m_value: [0; 16],
        }
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JumpBuffer {
    m_mask_was_saved: i32,
    m_saved_mask: SignalSet,
}
impl JumpBuffer {
    pub fn new() -> Self {
        JumpBuffer { m_mask_was_saved: 0, m_saved_mask: SignalSet::new(), }
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SignalAction {
    m_handler: SignalHandlerFunction,
    m_mask: SignalSet,
    m_flags: i32,
    m_restorer: Option<unsafe fn() -> ()>,
}
impl SignalAction {
    pub fn new(handler: SignalHandlerFunction) -> Self {
        SignalAction {
            m_handler: handler,
            m_mask: SignalSet::new(),
            m_flags: 0,
            m_restorer: None,
        }
    }
    pub unsafe fn setsignal(sig: i32, handler: Option<unsafe fn(i32) -> ()>) {
        unsafe {
            let mut signalaction = SignalAction::new(handler);
            libc::sigemptyset(&mut signalaction.m_mask as *mut SignalSet as *mut libc::sigset_t);
            libc::sigaction(sig, &mut signalaction as *mut SignalAction as *mut libc::sigaction, null_mut());
        }
    }
}
