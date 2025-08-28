use crate::cclosure::*;
use crate::closure::*;
use crate::lclosure::*;
use crate::object::*;
use crate::prototype::*;
use crate::state::*;
use crate::table::*;
use crate::tstring::*;
use crate::upvalue::*;
use crate::user::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub union GCUnion {
    pub object: Object,
    pub ts: TString,
    pub u: User,
    pub ccl: CClosure,
    pub lcl: LClosure,
    pub ucl: UClosure,
    pub h: Table,
    pub p: Prototype,
    pub th: State,
    pub upv: UpValue,
}
