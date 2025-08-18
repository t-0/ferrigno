use crate::labellist::*;
use crate::labeldesc::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DynamicData {
    pub actvar: C2RustUnnamed21,
    pub gt: Labellist,
    pub label: Labellist,
}
