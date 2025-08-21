use crate::state::*;
use crate::object::*;
use crate::tstring::*;
use crate::stkidrel::*;
use crate::table::*;
use crate::prototype::*;
use crate::udata::*;
use crate::closure::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub union GCUnion {
    pub gc: Object,
    pub ts: TString,
    pub u: Udata,
    pub cl: Closure,
    pub h: Table,
    pub p: Prototype,
    pub th: State,
    pub upv: UpVal,
}
