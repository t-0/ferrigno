use crate::object::*;
use crate::functions::*;
use crate::new::*;
#[derive(Copy, Clone)]
pub union Value {
    pub gc: *mut Object,
    pub p: *mut libc::c_void,
    pub f: CFunction,
    pub i: i64,
    pub n: f64,
}
impl New for Value {
    fn new() -> Self {
        Value {
            gc: std::ptr::null_mut(),
        }
    }
}
