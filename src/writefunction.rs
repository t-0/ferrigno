#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
)]
use crate::state::*;
pub type WriteFunction = Option::<
    unsafe extern "C" fn(
        *mut State,
        *const libc::c_void,
        u64,
        *mut libc::c_void,
    ) -> i32,
>;
