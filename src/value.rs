use crate::object::*;
use crate::functions::*;
use crate::new::*;
#[derive(Copy, Clone)]
pub union Value {
    pub object: *mut Object,
    pub p: *mut libc::c_void,
    pub f: CFunction,
    pub i: i64,
    pub n: f64,
}
impl New for Value {
    fn new() -> Self {
        Value {
            object: std::ptr::null_mut(),
        }
    }
}
