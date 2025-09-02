use crate::functions::*;
use crate::object::*;
use crate::table::*;
use crate::tag::*;
use crate::global::*;
use crate::state::*;
use crate::tvalue::*;
use crate::closure::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CClosure {
    pub next: *mut Object,
    pub tag: u8,
    pub marked: u8,
    pub count_upvalues: u8,
    pub dummy1: u8,
    pub dummy2: u32,
    pub gc_list: *mut Object,
    pub function: CFunction,
    pub upvalues: ClosureUpValue,
}
impl TObject for CClosure {
    fn get_marked(&self) -> u8 {
        self.marked
    }
    fn set_marked(&mut self, marked_: u8) {
        self.marked = marked_;
    }
    fn set_tag(&mut self, tag: u8) {
        self.tag = tag;
    }
    fn set_collectable(&mut self) {
        self.set_tag(set_collectable(self.get_tag()));
    }
    fn get_tag(&self) -> u8 {
        return self.tag;
    }
    fn is_collectable(&self) -> bool {
        return is_collectable(self.get_tag());
    }
    fn get_tag_type(&self) -> u8 {
        return get_tag_type(self.get_tag());
    }
    fn get_tag_variant(&self) -> u8 {
        get_tag_variant(self.get_tag())
    }
    fn get_class_name(&mut self) -> String {
        "cclosure".to_string()
    }
    fn get_metatable(&mut self) -> *mut Table {
        std::ptr::null_mut()
    }
}
pub unsafe extern "C" fn traversecclosure(g: *mut Global, cl: *mut CClosure) -> u64 {
    unsafe {
        let mut i: u64;
        i = 0;
        while i < (*cl).count_upvalues as u64 {
            if ((*((*cl).upvalues).c_tvalues.as_mut_ptr().offset(i as isize)).is_collectable())
                && (*(*((*cl).upvalues).c_tvalues.as_mut_ptr().offset(i as isize))
                    .value
                    .object)
                    .get_marked()
                    & (1 << 3 | 1 << 4)
                    != 0
            {
                reallymarkobject(
                    g,
                    (*((*cl).upvalues).c_tvalues.as_mut_ptr().offset(i as isize))
                        .value
                        .object,
                );
            }
            i += 1;
        }
        return 1 + (*cl).count_upvalues as u64;
    }
}
pub unsafe extern "C" fn luaf_newcclosure(state: *mut State, nupvals: i32) -> *mut CClosure {
    unsafe {
        let o: *mut Object = luac_newobj(
            state,
            TAG_VARIANT_CLOSURE_C,
            (32 as i32 + ::core::mem::size_of::<TValue>() as i32 * nupvals) as u64,
        );
        let c: *mut CClosure = &mut (*(o as *mut CClosure));
        (*c).count_upvalues = nupvals as u8;
        return c;
    }
}
