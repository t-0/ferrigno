use crate::dumpstate::*;
use crate::loadable::*;
use crate::loadstate::*;
use crate::tstring::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UpValueDescription {
    pub name: *mut TString,
    pub is_in_stack: bool,
    pub index: u8,
    pub kind: u8,
}
impl Loadable for UpValueDescription {
    unsafe fn dump(&self, dump_state: &mut DumpState) {
        unsafe {
            dump_state.dump_byte(self.is_in_stack as u8);
            dump_state.dump_byte(self.index);
            dump_state.dump_byte(self.kind);
        }
    }
    unsafe fn load(&mut self, load_state: &mut LoadState) {
        unsafe {
            self.is_in_stack = load_state.load_byte() != 0;
            self.index = load_state.load_byte();
            self.kind = load_state.load_byte();
        }
    }
}
