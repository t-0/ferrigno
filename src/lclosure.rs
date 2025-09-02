use crate::object::*;
use crate::table::*;
use crate::tag::*;
use crate::state::*;
use crate::tvalue::*;
use crate::closure::*;
use crate::global::*;
use crate::upvalue::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LClosure {
    pub next: *mut Object,
    pub tag: u8,
    pub marked: u8,
    pub count_upvalues: u8,
    pub dummy1: u8,
    pub dummy2: u32,
    pub gc_list: *mut Object,
    pub payload: ClosurePayload,
    pub upvalues: ClosureUpValue,
}
impl TObject for LClosure {
    fn get_marked(&self) -> u8 {
        self.marked
    }
    fn set_marked(&mut self, marked_: u8) {
        self.marked = marked_;
    }
    fn set_tag(&mut self, tag: u8) {
        self.tag = tag;
    }
    fn is_collectable(&self) -> bool {
        return is_collectable(self.get_tag());
    }
    fn set_collectable(&mut self) {
        self.set_tag(set_collectable(self.tag));
    }
    fn get_tag(&self) -> u8 {
        return self.tag;
    }
    fn get_tag_type(&self) -> u8 {
        return get_tag_type(self.get_tag());
    }
    fn get_tag_variant(&self) -> u8 {
        get_tag_variant(self.get_tag())
    }
    fn get_class_name(&mut self) -> String {
        "lclosure".to_string()
    }
    fn get_metatable(&mut self) -> *mut Table {
        std::ptr::null_mut()
    }
}
pub unsafe extern "C" fn traverselclosure(g: *mut Global, cl: *mut LClosure) -> u64 {
    unsafe {
        if !((*cl).payload.l_prototype).is_null() {
            if (*(*cl).payload.l_prototype).get_marked() & (1 << 3 | 1 << 4) != 0 {
                reallymarkobject(g, &mut (*((*cl).payload.l_prototype as *mut Object)));
            }
        }
        let mut i: u64 = 0;
        while i < (*cl).count_upvalues as u64 {
            let uv: *mut UpValue = *((*cl).upvalues).l_upvalues.as_mut_ptr().offset(i as isize);
            if !uv.is_null() {
                if (*uv).get_marked() & (1 << 3 | 1 << 4) != 0 {
                    reallymarkobject(g, &mut (*(uv as *mut Object)));
                }
            }
            i += 1;
        }
        return 1 + (*cl).count_upvalues as u64;
    }
}
pub unsafe extern "C" fn luaf_newlclosure(state: *mut State, mut nupvals: i32) -> *mut LClosure {
    unsafe {
        let o: *mut Object = luac_newobj(
            state,
            TAG_VARIANT_CLOSURE_L,
            (32 as i32 + ::core::mem::size_of::<*mut TValue>() as i32 * nupvals)
                as u64,
        );
        let c: *mut LClosure = &mut (*(o as *mut LClosure));
        (*c).payload.l_prototype = std::ptr::null_mut();
        (*c).count_upvalues = nupvals as u8;
        loop {
            let fresh17 = nupvals;
            nupvals = nupvals - 1;
            if !(fresh17 != 0) {
                break;
            }
            let ref mut fresh18 = *((*c).upvalues).l_upvalues.as_mut_ptr().offset(nupvals as isize);
            *fresh18 = std::ptr::null_mut();
        }
        return c;
    }
}
pub unsafe extern "C" fn luaf_initupvals(state: *mut State, cl: *mut LClosure) {
    unsafe {
        let mut i: i32;
        i = 0;
        while i < (*cl).count_upvalues as i32 {
            let o: *mut Object = luac_newobj(
                state,
                TAG_TYPE_UPVALUE,
                ::core::mem::size_of::<UpValue>() as u64,
            );
            let uv: *mut UpValue = &mut (*(o as *mut UpValue));
            (*uv).v.p = &mut (*uv).u.value;
            (*(*uv).v.p).set_tag(TAG_VARIANT_NIL_NIL);
            let ref mut fresh19 = *((*cl).upvalues).l_upvalues.as_mut_ptr().offset(i as isize);
            *fresh19 = uv;
            if (*cl).get_marked() & 1 << 5 != 0 && (*uv).get_marked() & (1 << 3 | 1 << 4) != 0 {
                luac_barrier_(
                    state,
                    &mut (*(cl as *mut Object)),
                    &mut (*(uv as *mut Object)),
                );
            } else {
            };
            i += 1;
        }
    }
}
