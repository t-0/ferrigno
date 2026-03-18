use crate::callinfo::*;
use crate::character::*;
use crate::debuginfo::*;
use crate::functions::*;
use crate::global::*;
use crate::object::*;
use crate::objectwithgclist::*;
use crate::prototype::*;
use crate::state::*;
use crate::table::*;
use crate::tagvariant::*;
use crate::tobject::*;
use crate::tobjectwithgclist::TObjectWithGCList;
use crate::tvalue::*;
use crate::upvalue::*;
use std::ptr::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub union ClosureUpValue {
    pub closureupvalue_tvalues: [TValue; 0],
    pub closureupvalue_lvalues: [*mut UpValue; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union ClosurePayload {
    pub closurepayload_cfunction: CFunction,
    pub closurepayload_lprototype: *mut Prototype,
}
type ClosureSuper = ObjectWithGCList;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Closure {
    pub closure_super: ClosureSuper,
    pub closure_count_upvalues: u8,
    pub closure_payload: ClosurePayload,
    pub closure_upvalues: ClosureUpValue,
}
impl TObject for Closure {
    fn as_object(&self) -> &Object {
        self.closure_super.as_object()
    }
    fn as_object_mut(&mut self) -> &mut Object {
        self.closure_super.as_object_mut()
    }
}
impl TObjectWithGCList for Closure {
    fn getgclist(&mut self) -> *mut *mut ObjectWithGCList {
        self.closure_super.getgclist()
    }
}
impl Closure {
    pub unsafe fn closure_free(&mut self, state: *mut State) {
        unsafe {
            let size = match self.get_tagvariant() {
                | TagVariant::ClosureC => Closure::size_cclosure(self.closure_count_upvalues as usize),
                | TagVariant::ClosureL => Closure::size_lclosure(self.closure_count_upvalues as usize),
                | _ => 0,
            };
            (*state).free_memory(self as *mut Closure as *mut std::ffi::c_void, size);
        }
    }
    pub unsafe fn traversecclosure(global: *mut Global, closure: *mut Closure) -> usize {
        unsafe {
            for i in 0..(*closure).closure_count_upvalues {
                let uv = &*((*closure).closure_upvalues)
                    .closureupvalue_tvalues
                    .as_mut_ptr()
                    .add(i as usize);
                if let Some(obj) = uv.as_object()
                    && (*obj).get_marked() & WHITEBITS != 0
                {
                    Object::really_mark_object(global, obj);
                }
            }
            1 + (*closure).closure_count_upvalues as usize
        }
    }
    pub unsafe fn traverselclosure(global: *mut Global, closure: *mut Closure) -> usize {
        unsafe {
            if !((*closure).closure_payload.closurepayload_lprototype).is_null()
                && (*(*closure).closure_payload.closurepayload_lprototype).get_marked() & WHITEBITS != 0
            {
                Object::really_mark_object(
                    global,
                    &mut *((*closure).closure_payload.closurepayload_lprototype as *mut Object),
                );
            }
            for i in 0..(*closure).closure_count_upvalues {
                let upvalue: *mut UpValue = *((*closure).closure_upvalues)
                    .closureupvalue_lvalues
                    .as_mut_ptr()
                    .add(i as usize);
                if !upvalue.is_null() && (*upvalue).get_marked() & WHITEBITS != 0 {
                    Object::really_mark_object(global, &mut *(upvalue as *mut Object));
                }
            }
            1 + (*closure).closure_count_upvalues as usize
        }
    }
    pub unsafe fn size_cclosure(closure_count_upvalues: usize) -> usize {
        core::mem::size_of::<Closure>() + size_of::<TValue>() * closure_count_upvalues
    }
    pub unsafe fn size_lclosure(closure_count_upvalues: usize) -> usize {
        core::mem::size_of::<Closure>() + size_of::<*mut TValue>() * closure_count_upvalues
    }
    pub unsafe fn collectvalidlines(state: *mut State, closure: *mut Closure) {
        unsafe {
            if !(!closure.is_null() && (*closure).get_tagvariant() == TagVariant::ClosureL) {
                (*(*state).interpreter_top.stkidrel_pointer).tvalue_set_tag_variant(TagVariant::NilNil);
                (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.add(1);
            } else {
                let prototype: *const Prototype = (*closure).closure_payload.closurepayload_lprototype;
                let mut current_line = (*prototype).prototype_linedefined;
                let table: *mut Table = luah_new(state);
                let io: *mut TValue = &mut (*(*state).interpreter_top.stkidrel_pointer);
                (*io).set_table(table);
                (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.add(1);
                if !((*prototype).prototype_lineinfo.vectort_pointer).is_null() {
                    let mut v: TValue = TValue::new(TagVariant::BooleanTrue);
                    let start: i32 = if !(*prototype).prototype_isvariablearguments {
                        0
                    } else {
                        current_line = nextline(prototype, current_line, 0);
                        1
                    };
                    for i in start..(*prototype).prototype_lineinfo.get_size() as i32 {
                        current_line = nextline(prototype, current_line, i);
                        luah_setint(state, table, current_line as i64, &mut v);
                    }
                }
            };
        }
    }
    pub unsafe fn auxgetinfo(
        state: *mut State, mut what: *const i8, debuginfo: *mut DebugInfo, closure: *mut Closure, callinfo: *mut CallInfo,
    ) -> i32 {
        unsafe {
            let mut status: i32 = 1;
            while *what != 0 {
                match Character::from(*what as i32) {
                    | Character::UpperS => {
                        funcinfo(debuginfo, closure);
                    },
                    | Character::LowerL => {
                        (*debuginfo).debuginfo_current_line =
                            if !callinfo.is_null() && (*callinfo).callinfo_callstatus as i32 & CALLSTATUS_LUA == 0 {
                                CallInfo::getcurrentline(callinfo)
                            } else {
                                -1
                            };
                    },
                    | Character::LowerU => {
                        (*debuginfo).debuginfo_count_upvalues =
                            (if closure.is_null() { 0 } else { (*closure).closure_count_upvalues as i32 }) as u8;
                        if !(!closure.is_null() && (*closure).get_tagvariant() == TagVariant::ClosureL) {
                            (*debuginfo).debuginfo_is_variable_arguments = true;
                            (*debuginfo).debuginfo_count_parameters = 0;
                        } else {
                            (*debuginfo).debuginfo_is_variable_arguments = (*(*closure).closure_payload.closurepayload_lprototype)
                                .prototype_isvariablearguments
                                || (*(*closure).closure_payload.closurepayload_lprototype).prototype_needsvarargtable;
                            (*debuginfo).debuginfo_count_parameters =
                                (*(*closure).closure_payload.closurepayload_lprototype).prototype_countparameters;
                        }
                    },
                    | Character::LowerT => {
                        (*debuginfo).debuginfo_is_tail_call = if !callinfo.is_null() {
                            0 != ((*callinfo).callinfo_callstatus as i32 & CALLSTATUS_TAIL)
                        } else {
                            false
                        };
                        (*debuginfo).debuginfo_extra_args =
                            if !callinfo.is_null() && (*callinfo).callinfo_callstatus as i32 & CALLSTATUS_LUA == 0 {
                                (*callinfo).callinfo_u.l.count_extra_arguments
                            } else {
                                0
                            };
                    },
                    | Character::LowerN => {
                        (*debuginfo).debuginfo_name_what = CallInfo::getfuncname(state, callinfo, &mut (*debuginfo).debuginfo_name);
                        if ((*debuginfo).debuginfo_name_what).is_null() {
                            (*debuginfo).debuginfo_name_what = c"".as_ptr();
                            (*debuginfo).debuginfo_name = null();
                        }
                    },
                    | Character::LowerR => {
                        if callinfo.is_null() || (*callinfo).callinfo_callstatus as i32 & CALLSTATUS_FIN == 0 {
                            (*debuginfo).debuginfo_count_transfer = 0;
                            (*debuginfo).debuginfo_transfer_function = (*debuginfo).debuginfo_count_transfer;
                        } else {
                            (*debuginfo).debuginfo_transfer_function = (*callinfo)
                                .callinfo_u2
                                .callinfoconstituentb_transferinfo
                                .callinfoconsistuentbtransferinfo_ftransfer;
                            (*debuginfo).debuginfo_count_transfer = (*callinfo)
                                .callinfo_u2
                                .callinfoconstituentb_transferinfo
                                .callinfoconsistuentbtransferinfo_ntransfer;
                        }
                    },
                    | Character::UpperL | Character::LowerF => {},
                    | _ => {
                        status = 0;
                    },
                }
                what = what.add(1);
            }
            status
        }
    }
    pub unsafe fn luaf_newcclosure(state: *mut State, count_upvalues: i32) -> *mut Closure {
        unsafe {
            let object: *mut Object = luac_newobj(state, TagVariant::ClosureC, Closure::size_cclosure(count_upvalues as usize));
            let ret: *mut Closure = &mut *(object as *mut Closure);
            (*ret).closure_count_upvalues = count_upvalues as u8;
            ret
        }
    }
    pub unsafe fn luaf_newlclosure(state: *mut State, mut count_upvalues: i32) -> *mut Closure {
        unsafe {
            let object: *mut Object = luac_newobj(state, TagVariant::ClosureL, Closure::size_lclosure(count_upvalues as usize));
            let ret: *mut Closure = &mut *(object as *mut Closure);
            (*ret).closure_payload.closurepayload_lprototype = null_mut();
            (*ret).closure_count_upvalues = count_upvalues as u8;
            loop {
                let fresh = count_upvalues;
                count_upvalues -= 1;
                if fresh == 0 {
                    break;
                }
                *((*ret).closure_upvalues)
                    .closureupvalue_lvalues
                    .as_mut_ptr()
                    .add(count_upvalues as usize) = null_mut();
            }
            ret
        }
    }
    pub unsafe fn luaf_initupvals(state: *mut State, closure: *mut Closure) {
        unsafe {
            for i in 0..(*closure).closure_count_upvalues {
                let object: *mut Object = luac_newobj(state, TagVariant::UpValue, size_of::<UpValue>());
                let upvalue: *mut UpValue = &mut *(object as *mut UpValue);
                (*upvalue).upvalue_v.upvaluea_p = std::ptr::addr_of_mut!((*upvalue).upvalue_u.upvalueb_value);
                (*(*upvalue).upvalue_v.upvaluea_p).tvalue_set_tag_variant(TagVariant::NilNil);
                let fresh = &mut *((*closure).closure_upvalues)
                    .closureupvalue_lvalues
                    .as_mut_ptr()
                    .add(i as usize);
                *fresh = upvalue;
                if (*closure).get_marked() & BLACKBIT != 0 && (*upvalue).get_marked() & WHITEBITS != 0 {
                    Object::luac_barrier_(state, &mut *(closure as *mut Object), &mut *(upvalue as *mut Object));
                }
            }
        }
    }
}
