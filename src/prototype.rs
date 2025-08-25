use crate::absolutelineinfo::*;
use crate::localvariable::*;
use crate::object::*;
use crate::onelua::*;
use crate::tvalue::*;
use crate::tstring::*;
use crate::state::*;
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
impl TObject for Prototype {
    fn get_class_name() -> String {
        "Prototype".to_string()
    }
}
impl Prototype {
pub unsafe extern "C" fn free_prototype(& mut self, state: *mut State) { unsafe {
(*state).free_memory(
        self.code as *mut libc::c_void,
        (self.sizecode as u64).wrapping_mul(::core::mem::size_of::<u32>() as u64),
    );
(*state).free_memory(
        self.p as *mut libc::c_void,
        (self.sizep as u64).wrapping_mul(::core::mem::size_of::<*mut Prototype>() as u64),
    );
(*state).free_memory(
        self.k as *mut libc::c_void,
        (self.sizek as u64).wrapping_mul(::core::mem::size_of::<TValue>() as u64),
    );
(*state).free_memory(
        self.lineinfo as *mut libc::c_void,
        (self.sizelineinfo as u64).wrapping_mul(::core::mem::size_of::<i8>() as u64),
    );
(*state).free_memory(
        self.abslineinfo as *mut libc::c_void,
        (self.sizeabslineinfo as u64)
            .wrapping_mul(::core::mem::size_of::<AbsoluteLineInfo>() as u64),
    );
(*state).free_memory(
        self.locvars as *mut libc::c_void,
        (self.sizelocvars as u64).wrapping_mul(::core::mem::size_of::<LocalVariable>() as u64),
    );
(*state).free_memory(
        self.upvalues as *mut libc::c_void,
        (self.sizeupvalues as u64).wrapping_mul(::core::mem::size_of::<Upvaldesc>() as u64),
    );
(*state).free_memory(
        self as *mut Prototype as *mut libc::c_void,
        ::core::mem::size_of::<Prototype>() as u64,
    );
}}
}
