use crate::state::*;
use crate::gcobject::*;
use crate::tstring::*;
use crate::stkidrel::*;
use crate::table::*;
use crate::proto::*;
use crate::udata::*;
use crate::closure::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub union GCUnion {
    pub gc: GCObject,
    pub ts: TString,
    pub u: Udata,
    pub cl: Closure,
    pub h: Table,
    pub p: Proto,
    pub th: State,
    pub upv: UpVal,
}
