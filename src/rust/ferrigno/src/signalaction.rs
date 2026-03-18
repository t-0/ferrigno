use crate::functions::*;
use crate::signalset::*;
use std::ptr::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SignalAction {
    signalaction_handler: SignalHandlerFunction,
    signalaction_mask: SignalSet,
    signalaction_flags: i32,
    signalaction_restorer: Option<unsafe extern "C" fn() -> ()>,
}
impl SignalAction {
    pub fn new(handler: SignalHandlerFunction) -> Self {
        SignalAction {
            signalaction_handler: handler,
            signalaction_mask: SignalSet::new(),
            signalaction_flags: 0,
            signalaction_restorer: None,
        }
    }
    pub unsafe fn setsignal(sig: i32, handler: Option<unsafe extern "C" fn(i32) -> ()>) {
        unsafe extern "C" {
            fn sigemptyset(set: *mut SignalSet) -> i32;
            fn sigaction(sig: i32, act: *const SignalAction, oact: *mut SignalAction) -> i32;
        }
        unsafe {
            let mut signalaction = SignalAction::new(handler);
            sigemptyset(&mut signalaction.signalaction_mask);
            sigaction(sig, &signalaction, null_mut());
        }
    }
}
