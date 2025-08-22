use crate::absolutelineinfo::*;
use crate::localvariable::*;
use crate::object::*;
use crate::stkidrel::*;
use crate::tstring::*;
use crate::upvaldesc::*;
use crate::ObjectBase;
ObjectBase! {
#[derive(Debug, Copy, Clone)]
pub struct Prototype {
    pub count_parameters: u8,
    pub is_variable_arguments: bool,
    pub maxstacksize: u8,
    pub sizeupvalues: i32,
    pub sizek: i32,
    pub sizecode: i32,
    pub sizelineinfo: i32,
    pub sizep: i32,
    pub sizelocvars: i32,
    pub sizeabslineinfo: i32,
    pub line_defined: i32,
    pub last_line_defined: i32,
    pub k: *mut TValue,
    pub code: *mut u32,
    pub p: *mut *mut Prototype,
    pub upvalues: *mut Upvaldesc,
    pub lineinfo: *mut i8,
    pub abslineinfo: *mut AbsoluteLineInfo,
    pub locvars: *mut LocalVariable,
    pub source: *mut TString,
    pub gc_list: *mut Object,
}
}
