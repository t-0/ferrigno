use crate::new::*;
use crate::node::*;
use crate::object::*;
use crate::tag::*;
use crate::tvalue::*;
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct Table {
    pub object: Object,
    pub collectable: bool,
    pub flags: u8,
    pub log_size_node: u8,
    pub dummy1: u8 = 0,
    pub array_limit: u32,
    pub array: *mut TValue,
    pub node: *mut Node,
    pub last_free: *mut Node,
    pub metatable: *mut Table,
    pub gc_list: *mut Object,
}
impl TObject for Table {
    fn get_marked(&self) -> u8 {
        self.object.marked
    }
    fn set_marked(&mut self, marked_: u8) {
        self.object.marked = marked_;
    }
    fn set_tag(&mut self, tag: u8) {
        self.object.set_tag(tag);
    }
    fn set_collectable(&mut self) {
        self.collectable = true;
    }
    fn is_collectable(&self) -> bool {
        self.collectable
    }
    fn get_tag(&self) -> u8 {
        self.object.get_tag()
    }
    fn get_tag_type(&self) -> u8 {
        get_tag_type(self.get_tag())
    }
    fn get_tag_variant(&self) -> u8 {
        self.object.get_tag_variant()
    }
    fn get_class_name(&mut self) -> String {
        "table".to_string()
    }
    fn get_metatable(&mut self) -> *mut Table {
        self.metatable
    }
}
impl New for Table {
    fn new() -> Self {
        Table {
            object: Object::new(),
            collectable: true,
            flags: 0,
            log_size_node: 0,
            array_limit: 0,
            array: std::ptr::null_mut(),
            node: std::ptr::null_mut(),
            last_free: std::ptr::null_mut(),
            metatable: std::ptr::null_mut(),
            gc_list: std::ptr::null_mut(),
            ..
        }
    }
}
impl Table {
    pub fn get_marked(&self) -> u8 {
        self.object.marked
    }
    pub fn set_marked(&mut self, marked_: u8) {
        self.object.marked = marked_;
    }
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
    pub unsafe extern "C" fn get_free_position(&mut self) -> *mut Node {
        unsafe {
            if !self.last_free.is_null() {
                while self.last_free > self.node {
                    self.last_free = self.last_free.offset(-1);
                    self.last_free;
                    if (*self.last_free).key.tag == TAG_VARIANT_NIL_NIL {
                        return self.last_free;
                    }
                }
            }
            return std::ptr::null_mut();
        }
    }
}
