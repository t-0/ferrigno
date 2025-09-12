use crate::dumpstate::*;
use crate::loadstate::*;
pub trait Loadable {
    unsafe fn dump(&self, dump_state: &mut DumpState);
    unsafe fn load(&mut self, load_state: &mut LoadState);
}
