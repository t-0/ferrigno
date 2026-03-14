use crate::table::*;
pub trait TObjectWithMetatable {
    fn set_metatable(&mut self, _metatable: *mut Table);
    fn get_metatable(&self) -> *mut Table;
}
