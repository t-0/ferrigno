use crate::node::*;
use crate::object::*;
use crate::tvalue::*;
use crate::tag::*;
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct Table {
    pub next: *mut Object,
    pub tag: u8,
    pub marked: u8,
    pub flags: u8,
    pub log_size_node: u8,
    pub array_limit: u32,
    pub array: *mut TValue,
    pub node: *mut Node,
    pub last_free: *mut Node,
    pub metatable: *mut Table,
    pub gc_list: *mut Object,
}
impl TObject for Table {
    fn get_tag_type(&self) -> u8 {
        self.tag & TAG_TYPE_MASK_
    }
    fn get_class_name(& mut self) -> String {
        "Table".to_string()
    }
    fn get_metatable(& mut self) -> *mut Table {
        self.metatable
    }
}
impl Table {
    pub unsafe extern "C" fn exchange_hash_part(t1: *mut Table, t2: *mut Table) {
        unsafe {
            let temporary_size_node: u8 = (*t1).log_size_node;
            (*t1).log_size_node = (*t2).log_size_node;
            (*t2).log_size_node = temporary_size_node;
            let temporary_node: *mut Node = (*t1).node;
            (*t1).node = (*t2).node;
            (*t2).node = temporary_node;
            let temporary_last_free: *mut Node = (*t1).last_free;
            (*t1).last_free = (*t2).last_free;
            (*t2).last_free = temporary_last_free;
        }
    }
    pub unsafe extern "C" fn get_free_position(& mut self) -> *mut Node {
        unsafe {
            if !self.last_free.is_null() {
                while self.last_free > self.node {
                    self.last_free = self.last_free.offset(-1);
                    self.last_free;
                    if (*self.last_free).u.key_tag == 0 {
                        return self.last_free;
                    }
                }
            }
            return std::ptr::null_mut();
        }
    }
}
