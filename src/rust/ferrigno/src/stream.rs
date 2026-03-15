use crate::functions::*;
use crate::iohandle::*;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Stream {
    pub stream_handle: *mut IoHandle,
    pub stream_cfunctionclose: CFunction,
}
