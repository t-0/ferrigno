use crate::callinfo::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Debug {
    pub event: i32,
    pub name: *const i8,
    pub namewhat: *const i8,
    pub what: *const i8,
    pub source: *const i8,
    pub srclen: u64,
    pub currentline: i32,
    pub linedefined: i32,
    pub lastlinedefined: i32,
    pub nups: u8,
    pub nparams: u8,
    pub isvararg: i8,
    pub istailcall: i8,
    pub ftransfer: u16,
    pub ntransfer: u16,
    pub short_src: [i8; 60],
    pub i_ci: *mut CallInfo,
}
