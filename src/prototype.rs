use crate::absolutelineinfo::*;
use crate::localvariable::*;
use crate::tag::*;
use crate::object::*;
use crate::tvalue::*;
use crate::table::*;
use crate::tstring::*;
use crate::state::*;
use crate::upvaldesc::*;
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct Prototype {
    pub next: *mut Object,
    pub tag: u8,
    pub marked: u8,
    pub count_parameters: u8,
    pub is_variable_arguments: bool,
    pub maximum_stack_size: u8,
    pub size_upvalues: i32,
    pub size_k: i32,
    pub size_code: i32,
    pub size_line_info: i32,
    pub size_p: i32,
    pub size_local_variables: i32,
    pub size_absolute_line_info: i32,
    pub line_defined: i32,
    pub last_line_defined: i32,
    pub k: *mut TValue,
    pub code: *mut u32,
    pub p: *mut *mut Prototype,
    pub upvalues: *mut Upvaldesc,
    pub line_info: *mut i8,
    pub absolute_line_info: *mut AbsoluteLineInfo,
    pub local_variables: *mut LocalVariable,
    pub source: *mut TString,
    pub gc_list: *mut Object,
}
impl TObject for Prototype {
    fn get_marked(& self) -> u8 {
        self.marked
    }
    fn set_marked(& mut self, marked_: u8) {
        self.marked = marked_;
    }
    fn set_tag(& mut self, tag: u8) {
        self.tag = tag;
    }
    fn is_collectable(&self) -> bool {
        return is_collectable(self.tag);
    }
    fn get_tag(&self) -> u8 {
        self.tag
    }
    fn get_tag_type(&self) -> u8 {
        get_tag_type(self.get_tag())
    }
    fn get_tag_variant(&self) -> u8 {
        get_tag_variant(self.get_tag())
    }
    fn get_class_name(& mut self) -> String {
        "prototype".to_string()
    }
    fn get_metatable(& mut self) -> *mut Table {
        std::ptr::null_mut()
    }
}
impl Prototype {
pub unsafe extern "C" fn free_prototype(& mut self, state: *mut State) { unsafe {
(*state).free_memory(
        self.code as *mut libc::c_void,
        (self.size_code as u64).wrapping_mul(::core::mem::size_of::<u32>() as u64),
    );
(*state).free_memory(
        self.p as *mut libc::c_void,
        (self.size_p as u64).wrapping_mul(::core::mem::size_of::<*mut Prototype>() as u64),
    );
(*state).free_memory(
        self.k as *mut libc::c_void,
        (self.size_k as u64).wrapping_mul(::core::mem::size_of::<TValue>() as u64),
    );
(*state).free_memory(
        self.line_info as *mut libc::c_void,
        (self.size_line_info as u64).wrapping_mul(::core::mem::size_of::<i8>() as u64),
    );
(*state).free_memory(
        self.absolute_line_info as *mut libc::c_void,
        (self.size_absolute_line_info as u64)
            .wrapping_mul(::core::mem::size_of::<AbsoluteLineInfo>() as u64),
    );
(*state).free_memory(
        self.local_variables as *mut libc::c_void,
        (self.size_local_variables as u64).wrapping_mul(::core::mem::size_of::<LocalVariable>() as u64),
    );
(*state).free_memory(
        self.upvalues as *mut libc::c_void,
        (self.size_upvalues as u64).wrapping_mul(::core::mem::size_of::<Upvaldesc>() as u64),
    );
(*state).free_memory(
        self as *mut Prototype as *mut libc::c_void,
        ::core::mem::size_of::<Prototype>() as u64,
    );
}}
}
