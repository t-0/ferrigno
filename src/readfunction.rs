use crate::state::*;
pub type ReadFunction = Option::<
    unsafe extern "C" fn(
        *mut State,
        *mut libc::c_void,
        *mut u64,
    ) -> *const i8,
>;
