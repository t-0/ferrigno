use crate::rawvalue::*;
use crate::tstring::*;
use crate::v::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExpressionDescription {
    pub k: u32,
    pub u: RawValue,
    pub t: i32,
    pub f: i32,
}
pub unsafe extern "C" fn init_exp(e: *mut ExpressionDescription, k: u32, i: i32) {
    unsafe {
        (*e).t = -1;
        (*e).f = (*e).t;
        (*e).k = k;
        (*e).u.info = i;
    }
}
pub unsafe extern "C" fn codestring(e: *mut ExpressionDescription, s: *mut TString) {
    unsafe {
        (*e).t = -1;
        (*e).f = (*e).t;
        (*e).k = VKSTR;
        (*e).u.strval = s;
    }
}
