use crate::closure::*;
use crate::object::*;
use crate::prototype::*;
use crate::state::*;
use crate::stkidrel::*;
use crate::table::*;
use crate::tstring::*;
use crate::udata::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub union GCUnion {
    pub gc: Object,
    pub ts: TString,
    pub u: Udata,
    pub cl: UClosure,
    pub h: Table,
    pub p: Prototype,
    pub th: State,
    pub upv: UpVal,
}
