use crate::dumpstate::*;
use crate::loadstate::*;
use crate::tloadable::*;
use crate::tstring::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UpValueDescription {
    pub upvaluedescription_name: *mut TString,
    pub upvaluedescription_isinstack: bool,
    pub upvaluedescription_index: u8,
    pub upvaluedescription_kind: u8,
}
impl TLoadable for UpValueDescription {
    unsafe fn dump(&self, dump_state: &mut DumpState) {
        unsafe {
            dump_state.dump_byte(self.upvaluedescription_isinstack as u8);
            dump_state.dump_byte(self.upvaluedescription_index);
            dump_state.dump_byte(self.upvaluedescription_kind);
        }
    }
    unsafe fn load(&mut self, load_state: &mut LoadState) {
        unsafe {
            self.upvaluedescription_isinstack = load_state.load_byte() != 0;
            self.upvaluedescription_index = load_state.load_byte();
            self.upvaluedescription_kind = load_state.load_byte();
        }
    }
}
