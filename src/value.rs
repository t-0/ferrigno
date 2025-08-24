use crate::object::*;
use crate::functions::*;
#[derive(Copy, Clone)]
pub union Value {
    pub gc: *mut Object,
    pub p: *mut libc::c_void,
    pub f: CFunction,
    pub i: i64,
    pub n: f64,
}
