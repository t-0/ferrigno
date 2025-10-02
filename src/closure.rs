use crate::callinfo::*;
use crate::tobjectwithgclist::TObjectWithGCList;
use libc::*;
use crate::character::*;
use crate::debuginfo::*;
use crate::functions::*;
use crate::global::*;
use crate::object::*;
use crate::interpreter::*;
use crate::objectwithgclist::*;
use crate::tobject::*;
use crate::prototype::*;
use crate::table::*;
use crate::tag::*;
use crate::tvalue::*;
use crate::upvalue::*;
use std::ptr::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub union ClosureUpValue {
    pub c_tvalues: [TValue; 0],
    pub l_upvalues: [*mut UpValue; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union ClosurePayload {
    pub c_cfunction: CFunction,
    pub l_prototype: *mut Prototype,
}
pub type ClosureSuper = ObjectWithGCList;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Closure {
    pub super_: ClosureSuper,
    pub count_upvalues: u8,
    pub payload: ClosurePayload,
    pub upvalues: ClosureUpValue,
}
impl TObject for Closure {
    fn as_object(&self) -> &Object {
        &self.super_.as_object()
    }
    fn as_object_mut(&mut self) -> &mut Object {
        self.super_.as_object_mut()
    }
    fn get_class_name(&mut self) -> String {
        "closure".to_string()
    }
}
impl TObjectWithGCList for Closure {
    fn getgclist(&mut self) -> *mut *mut ObjectWithGCList {
        self.super_.getgclist()
    }
}
impl Closure {
    pub unsafe fn free_closure(&mut self, interpreter: *mut Interpreter) {
        unsafe {
            let size = match self.get_tag_variant() {
                TagVariant::ClosureC => size_cclosure(self.count_upvalues as usize),
                TagVariant::ClosureL => size_lclosure(self.count_upvalues as usize),
                _ => 0,
            };
            (*interpreter).free_memory(self as *mut Closure as *mut c_void, size);
        }
    }
    pub unsafe fn traversecclosure(global: *mut Global, closure: *mut Closure) -> usize {
        unsafe {
            for i in 0..(*closure).count_upvalues {
                if ((*((*closure).upvalues).c_tvalues.as_mut_ptr().offset(i as isize)).is_collectable()) && (*(*((*closure).upvalues).c_tvalues.as_mut_ptr().offset(i as isize)).value.value_object).get_marked() & (1 << 3 | 1 << 4) != 0 {
                    really_mark_object(global, (*((*closure).upvalues).c_tvalues.as_mut_ptr().offset(i as isize)).value.value_object);
                }
            }
            return 1 + (*closure).count_upvalues as usize;
        }
    }
    pub unsafe fn traverselclosure(global: *mut Global, closure: *mut Closure) -> usize {
        unsafe {
            if !((*closure).payload.l_prototype).is_null() {
                if (*(*closure).payload.l_prototype).get_marked() & (1 << 3 | 1 << 4) != 0 {
                    really_mark_object(global, &mut (*((*closure).payload.l_prototype as *mut Object)));
                }
            }
            for i in 0..(*closure).count_upvalues {
                let upvalue: *mut UpValue = *((*closure).upvalues).l_upvalues.as_mut_ptr().offset(i as isize);
                if !upvalue.is_null() {
                    if (*upvalue).get_marked() & (1 << 3 | 1 << 4) != 0 {
                        really_mark_object(global, &mut (*(upvalue as *mut Object)));
                    }
                }
            }
            return 1 + (*closure).count_upvalues as usize;
        }
    }
}
pub unsafe fn collectvalidlines(interpreter: *mut Interpreter, closure: *mut Closure) {
    unsafe {
        if !(!closure.is_null() && (*closure).get_tag_variant() == TagVariant::ClosureL) {
            (*(*interpreter).top.stkidrel_pointer).tvalue_set_tag_variant(TagVariant::NilNil);
            (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
        } else {
            let prototype: *const Prototype = (*closure).payload.l_prototype;
            let mut current_line = (*prototype).prototype_line_defined;
            let table: *mut Table = luah_new(interpreter);
            let io: *mut TValue = &mut (*(*interpreter).top.stkidrel_pointer);
            (*io).value.value_object = &mut (*(table as *mut Object));
            (*io).tvalue_set_tag_variant(TagVariant::Table);
            (*io).set_collectable(true);
            (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
            if !((*prototype).prototype_line_info.vectort_pointer).is_null() {
                let mut v: TValue = TValue::new(TagVariant::BooleanTrue);
                let start: i32 = if !(*prototype).prototype_is_variable_arguments {
                    0
                } else {
                    current_line = nextline(prototype, current_line, 0);
                    1
                };
                for i in start..(*prototype).prototype_line_info.get_size() as i32 {
                    current_line = nextline(prototype, current_line, i);
                    luah_setint(interpreter, table, current_line as i64, &mut v);
                }
            }
        };
    }
}
pub unsafe fn auxgetinfo(interpreter: *mut Interpreter, mut what: *const i8, debuginfo: *mut DebugInfo, closure: *mut Closure, callinfo: *mut CallInfo) -> i32 {
    unsafe {
        let mut status: i32 = 1;
        while *what != 0 {
            match Character::from(*what as i32) {
                Character::UpperS => {
                    funcinfo(debuginfo, closure);
                },
                Character::LowerL => {
                    (*debuginfo).debuginfo_currentline = if !callinfo.is_null() && (*callinfo).call_info_call_status as i32 & 1 << 1 == 0 { getcurrentline(callinfo) } else { -1 };
                },
                Character::LowerU => {
                    (*debuginfo).debuginfo_nups = (if closure.is_null() { 0 } else { (*closure).count_upvalues as i32 }) as u8;
                    if !(!closure.is_null() && (*closure).get_tag_variant() == TagVariant::ClosureL) {
                        (*debuginfo).debuginfo_isvariablearguments = true;
                        (*debuginfo).debuginfo_nparams = 0;
                    } else {
                        (*debuginfo).debuginfo_isvariablearguments = (*(*closure).payload.l_prototype).prototype_is_variable_arguments;
                        (*debuginfo).debuginfo_nparams = (*(*closure).payload.l_prototype).prototype_count_parameters;
                    }
                },
                Character::LowerT => {
                    (*debuginfo).debuginfo_istailcall = if !callinfo.is_null() { 0 != ((*callinfo).call_info_call_status as i32 & 1 << 5) } else { false };
                },
                Character::LowerN => {
                    (*debuginfo).debuginfo_namewhat = getfuncname(interpreter, callinfo, &mut (*debuginfo).debuginfo_name);
                    if ((*debuginfo).debuginfo_namewhat).is_null() {
                        (*debuginfo).debuginfo_namewhat = c"".as_ptr();
                        (*debuginfo).debuginfo_name = null();
                    }
                },
                Character::LowerR => {
                    if callinfo.is_null() || (*callinfo).call_info_call_status as i32 & 1 << 8 == 0 {
                        (*debuginfo).debuginfo_ntransfer = 0;
                        (*debuginfo).debuginfo_ftransfer = (*debuginfo).debuginfo_ntransfer;
                    } else {
                        (*debuginfo).debuginfo_ftransfer = (*callinfo).call_info_u2.transferinfo.ftransfer;
                        (*debuginfo).debuginfo_ntransfer = (*callinfo).call_info_u2.transferinfo.ntransfer;
                    }
                },
                Character::UpperL | Character::LowerF => {},
                _ => {
                    status = 0;
                },
            }
            what = what.offset(1);
        }
        return status;
    }
}
pub unsafe fn size_cclosure(count_upvalues: usize) -> usize {
    core::mem::size_of::<Closure>() + size_of::<TValue>() * count_upvalues
}
pub unsafe fn size_lclosure(count_upvalues: usize) -> usize {
    core::mem::size_of::<Closure>() + size_of::<*mut TValue>() * count_upvalues
}
pub unsafe fn luaf_newcclosure(interpreter: *mut Interpreter, count_upvalues: i32) -> *mut Closure {
    unsafe {
        let object: *mut Object = luac_newobj(interpreter, TagVariant::ClosureC, size_cclosure(count_upvalues as usize));
        let ret: *mut Closure = &mut (*(object as *mut Closure));
        (*ret).count_upvalues = count_upvalues as u8;
        return ret;
    }
}
pub unsafe fn luaf_newlclosure(interpreter: *mut Interpreter, mut count_upvalues: i32) -> *mut Closure {
    unsafe {
        let object: *mut Object = luac_newobj(interpreter, TagVariant::ClosureL, size_lclosure(count_upvalues as usize));
        let ret: *mut Closure = &mut (*(object as *mut Closure));
        (*ret).payload.l_prototype = null_mut();
        (*ret).count_upvalues = count_upvalues as u8;
        loop {
            let fresh = count_upvalues;
            count_upvalues = count_upvalues - 1;
            if fresh == 0 {
                break;
            }
            let ref mut fresh18 = *((*ret).upvalues).l_upvalues.as_mut_ptr().offset(count_upvalues as isize);
            *fresh18 = null_mut();
        }
        return ret;
    }
}
pub unsafe fn luaf_initupvals(interpreter: *mut Interpreter, closure: *mut Closure) {
    unsafe {
        for i in 0..(*closure).count_upvalues {
            let object: *mut Object = luac_newobj(interpreter, TagVariant::UpValue, size_of::<UpValue>());
            let upvalue: *mut UpValue = &mut (*(object as *mut UpValue));
            (*upvalue).v.p = &mut (*upvalue).u.value;
            (*(*upvalue).v.p).tvalue_set_tag_variant(TagVariant::NilNil);
            let ref mut fresh = *((*closure).upvalues).l_upvalues.as_mut_ptr().offset(i as isize);
            *fresh = upvalue;
            if (*closure).get_marked() & 1 << 5 != 0 && (*upvalue).get_marked() & (1 << 3 | 1 << 4) != 0 {
                luac_barrier_(interpreter, &mut (*(closure as *mut Object)), &mut (*(upvalue as *mut Object)));
            }
        }
    }
}
