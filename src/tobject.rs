use crate::object::*;
use crate::tagvariant::*;
pub trait TObject {
    fn as_object(&self) -> &Object;
    fn as_object_mut(&mut self) -> &mut Object;
    fn set_tagvariant(&mut self, tagvariant: TagVariant) {
        self.as_object_mut().set_tagvariant(tagvariant);
    }
    fn get_marked(&self) -> u8 {
        self.as_object().get_marked()
    }
    fn set_marked(&mut self, marked: u8) {
        self.as_object_mut().set_marked(marked);
    }
    fn get_tagvariant(&self) -> TagVariant {
        self.as_object().get_tagvariant()
    }
}
