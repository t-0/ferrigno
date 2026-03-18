#![allow(unpredictable_function_pointer_comparisons)]
use crate::callinfo::*;
use crate::character::*;
use crate::closure::*;
use crate::functionstate::LUA_IDSIZE;
use crate::prototype::*;
use crate::state::*;
use crate::strings::*;
use crate::tagtype::*;
use crate::tagvariant::*;
use crate::tobject::*;
use crate::tvalue::*;
use crate::utility::*;
use std::ptr::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DebugInfo {
    debuginfo_event: i32,
    pub debuginfo_name: *const i8,
    pub debuginfo_name_what: *const i8,
    pub debuginfo_what: *const i8,
    pub debuginfo_source: *const i8,
    pub debuginfo_source_length: usize,
    pub debuginfo_current_line: i32,
    pub debuginfo_line_defined: i32,
    pub debuginfo_last_line_defined: i32,
    pub debuginfo_count_upvalues: u8,
    pub debuginfo_count_parameters: usize,
    pub debuginfo_is_variable_arguments: bool,
    pub debuginfo_is_tail_call: bool,
    pub debuginfo_extra_args: i32,
    pub debuginfo_transfer_function: u16,
    pub debuginfo_count_transfer: u16,
    pub debuginfo_short_source: [i8; LUA_IDSIZE],
    pub debuginfo_callinfo: *mut CallInfo,
}
impl Default for DebugInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl DebugInfo {
    pub fn new() -> Self {
        DebugInfo {
            debuginfo_event: 0,
            debuginfo_name: null(),
            debuginfo_name_what: null(),
            debuginfo_what: null(),
            debuginfo_source: null(),
            debuginfo_source_length: 0,
            debuginfo_current_line: 0,
            debuginfo_line_defined: 0,
            debuginfo_last_line_defined: 0,
            debuginfo_count_upvalues: 0,
            debuginfo_count_parameters: 0,
            debuginfo_is_variable_arguments: false,
            debuginfo_is_tail_call: false,
            debuginfo_extra_args: 0,
            debuginfo_transfer_function: 0,
            debuginfo_count_transfer: 0,
            debuginfo_short_source: [0; LUA_IDSIZE],
            debuginfo_callinfo: null_mut(),
        }
    }
    pub fn new2(event: i32, currentline: i32, callinfo: *mut CallInfo) -> Self {
        DebugInfo {
            debuginfo_event: event,
            debuginfo_name: null(),
            debuginfo_name_what: null(),
            debuginfo_what: null(),
            debuginfo_source: null(),
            debuginfo_source_length: 0,
            debuginfo_current_line: currentline,
            debuginfo_line_defined: 0,
            debuginfo_last_line_defined: 0,
            debuginfo_count_upvalues: 0,
            debuginfo_count_parameters: 0,
            debuginfo_is_variable_arguments: false,
            debuginfo_is_tail_call: false,
            debuginfo_extra_args: 0,
            debuginfo_transfer_function: 0,
            debuginfo_count_transfer: 0,
            debuginfo_short_source: [0; LUA_IDSIZE],
            debuginfo_callinfo: callinfo,
        }
    }
    pub unsafe fn hookf(state: *mut State, debuginfo: *mut DebugInfo) {
        unsafe {
            const HOOK_NAMES: [*const i8; 5] = [
                c"call".as_ptr(),
                c"return".as_ptr(),
                c"line".as_ptr(),
                c"count".as_ptr(),
                c"tail call".as_ptr(),
            ];
            lua_getfield(state, LUA_REGISTRYINDEX, Strings::STRING_HOOKKEY);
            (*state).push_state();
            if lua_rawget(state, -2) == TagType::Closure {
                lua_pushstring(state, HOOK_NAMES[(*debuginfo).debuginfo_event as usize]);
                if (*debuginfo).debuginfo_current_line >= 0 {
                    (*state).push_integer((*debuginfo).debuginfo_current_line as i64);
                } else {
                    (*state).push_nil();
                }
                (*state).lua_callk(2, 0, 0, None);
            }
        }
    }
}
pub unsafe fn lua_getlocal(state: *mut State, debuginfo: *const DebugInfo, n: i32) -> *const i8 {
    unsafe {
        if debuginfo.is_null() {
            if (*(*state).interpreter_top.stkidrel_pointer.sub(1)).get_tagvariant() == TagVariant::ClosureL {
                luaf_getlocalname(
                    (*(*(*state).interpreter_top.stkidrel_pointer.sub(1)).as_closure().unwrap())
                        .closure_payload
                        .closurepayload_lprototype,
                    n,
                    0,
                )
            } else {
                null()
            }
        } else {
            let mut position: *mut TValue = null_mut();
            let ret = CallInfo::luag_findlocal(state, (*debuginfo).debuginfo_callinfo, n, &mut position);
            if !ret.is_null() {
                (*(*state).interpreter_top.stkidrel_pointer).copy_from(&*position);
                (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.add(1);
            }
            ret
        }
    }
}
pub unsafe fn lua_setlocal(state: *mut State, debuginfo: *const DebugInfo, n: i32) -> *const i8 {
    unsafe {
        let mut position: *mut TValue = null_mut();
        let ret: *const i8 = CallInfo::luag_findlocal(state, (*debuginfo).debuginfo_callinfo, n, &mut position);
        if !ret.is_null() {
            let io1: *mut TValue = &mut (*position);
            let io2: *const TValue = &mut (*(*state).interpreter_top.stkidrel_pointer.sub(1));
            (*io1).copy_from(&*io2);
            (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.sub(1);
        }
        ret
    }
}
pub unsafe fn funcinfo(debuginfo: *mut DebugInfo, closure: *mut Closure) {
    unsafe {
        if !(!closure.is_null() && (*closure).get_tagvariant() == TagVariant::ClosureL) {
            (*debuginfo).debuginfo_source = c"=[C]".as_ptr();
            (*debuginfo).debuginfo_source_length = size_of::<[i8; 5]>() - 1;
            (*debuginfo).debuginfo_line_defined = -1;
            (*debuginfo).debuginfo_last_line_defined = -1;
            (*debuginfo).debuginfo_what = c"C".as_ptr();
        } else {
            let prototype: *const Prototype = (*closure).closure_payload.closurepayload_lprototype;
            if !((*prototype).prototype_source).is_null() {
                (*debuginfo).debuginfo_source = (*(*prototype).prototype_source).get_contents_mut();
                (*debuginfo).debuginfo_source_length = (*(*prototype).prototype_source).get_length();
            } else {
                (*debuginfo).debuginfo_source = c"=?".as_ptr();
                (*debuginfo).debuginfo_source_length = size_of::<[i8; 3]>() - 1;
            }
            (*debuginfo).debuginfo_line_defined = (*prototype).prototype_linedefined;
            (*debuginfo).debuginfo_last_line_defined = (*prototype).prototype_lastlinedefined;
            (*debuginfo).debuginfo_what = if (*debuginfo).debuginfo_line_defined == 0 {
                c"main".as_ptr()
            } else {
                c"Lua".as_ptr()
            };
        }
        luao_chunkid(
            ((*debuginfo).debuginfo_short_source).as_mut_ptr(),
            (*debuginfo).debuginfo_source,
            (*debuginfo).debuginfo_source_length,
        );
    }
}
pub unsafe fn lua_getinfo(state: *mut State, mut what: *const i8, debuginfo: *mut DebugInfo) -> i32 {
    unsafe {
        let status: i32;
        let function;
        let callinfo;
        if *what as i32 == Character::AngleRight as i32 {
            callinfo = null_mut();
            function = &mut (*(*state).interpreter_top.stkidrel_pointer.sub(1));
            what = what.add(1);
            (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.sub(1);
        } else {
            callinfo = (*debuginfo).debuginfo_callinfo;
            function = &mut (*(*callinfo).callinfo_function.stkidrel_pointer);
        }
        match (*function).get_tagvariant() {
            | TagVariant::ClosureL => {
                let closure: *mut Closure = (*function).as_closure().unwrap();
                status = Closure::auxgetinfo(state, what, debuginfo, closure, callinfo);
                if !(cstr_chr(what, Character::LowerF as i8)).is_null() {
                    let io1: *mut TValue = &mut (*(*state).interpreter_top.stkidrel_pointer);
                    let io2: *const TValue = function;
                    (*io1).copy_from(&*io2);
                    (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.add(1);
                }
                if !(cstr_chr(what, Character::UpperL as i8)).is_null() {
                    Closure::collectvalidlines(state, closure);
                }
                status
            },
            | TagVariant::ClosureC => {
                let closure: *mut Closure = (*function).as_closure().unwrap();
                status = Closure::auxgetinfo(state, what, debuginfo, closure, callinfo);
                if !(cstr_chr(what, Character::LowerF as i8)).is_null() {
                    let io1: *mut TValue = &mut (*(*state).interpreter_top.stkidrel_pointer);
                    let io2: *const TValue = function;
                    (*io1).copy_from(&*io2);
                    (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.add(1);
                }
                if !(cstr_chr(what, Character::UpperL as i8)).is_null() {
                    Closure::collectvalidlines(state, closure);
                }
                status
            },
            | _ => {
                let closure: *mut Closure = null_mut();
                status = Closure::auxgetinfo(state, what, debuginfo, closure, callinfo);
                if !(cstr_chr(what, Character::LowerF as i8)).is_null() {
                    let io1: *mut TValue = &mut (*(*state).interpreter_top.stkidrel_pointer);
                    let io2: *const TValue = function;
                    (*io1).copy_from(&*io2);
                    (*state).interpreter_top.stkidrel_pointer = (*state).interpreter_top.stkidrel_pointer.add(1);
                }
                if !(cstr_chr(what, Character::UpperL as i8)).is_null() {
                    Closure::collectvalidlines(state, closure);
                }
                status
            },
        }
    }
}
