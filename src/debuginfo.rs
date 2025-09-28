#![allow(unpredictable_function_pointer_comparisons)]
use crate::callinfo::*;
use crate::character::*;
use crate::closure::*;
use crate::interpreter::*;
use crate::object::*;
use crate::prototype::*;
use crate::tag::*;
use crate::tvalue::*;
use crate::utility::c::*;
use crate::utility::*;
use std::ptr::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DebugInfo {
    debuginfo_event: i32,
    pub debuginfo_name: *const i8,
    pub debuginfo_namewhat: *const i8,
    pub debuginfo_what: *const i8,
    pub debuginfo_source: *const i8,
    pub debuginfo_sourcelength: usize,
    pub debuginfo_currentline: i32,
    pub debuginfo_linedefined: i32,
    pub debuginfo_lastlinedefined: i32,
    pub debuginfo_nups: u8,
    pub debuginfo_nparams: u8,
    pub debuginfo_isvariablearguments: bool,
    pub debuginfo_istailcall: bool,
    pub debuginfo_ftransfer: u16,
    pub debuginfo_ntransfer: u16,
    pub debuginfo_shortsrc: [i8; 60],
    pub debuginfo_callinfo: *mut CallInfo,
}
impl DebugInfo {
    pub fn new () -> Self {
        DebugInfo {
            debuginfo_event: 0,
            debuginfo_name: null(),
            debuginfo_namewhat: null(),
            debuginfo_what: null(),
            debuginfo_source: null(),
            debuginfo_sourcelength: 0,
            debuginfo_currentline: 0,
            debuginfo_linedefined: 0,
            debuginfo_lastlinedefined: 0,
            debuginfo_nups: 0,
            debuginfo_nparams: 0,
            debuginfo_isvariablearguments: false,
            debuginfo_istailcall: false,
            debuginfo_ftransfer: 0,
            debuginfo_ntransfer: 0,
            debuginfo_shortsrc: [0; 60],
            debuginfo_callinfo: null_mut(),
        }
    }
    pub fn new2 (event: i32, currentline: i32, callinfo: *mut CallInfo) -> Self {
        DebugInfo {
            debuginfo_event: event,
            debuginfo_name: null(),
            debuginfo_namewhat: null(),
            debuginfo_what: null(),
            debuginfo_source: null(),
            debuginfo_sourcelength: 0,
            debuginfo_currentline: currentline,
            debuginfo_linedefined: 0,
            debuginfo_lastlinedefined: 0,
            debuginfo_nups: 0,
            debuginfo_nparams: 0,
            debuginfo_isvariablearguments: false,
            debuginfo_istailcall: false,
            debuginfo_ftransfer: 0,
            debuginfo_ntransfer: 0,
            debuginfo_shortsrc: [0; 60],
            debuginfo_callinfo: callinfo,
        }
    }
    pub unsafe fn hookf(interpreter: *mut Interpreter, debuginfo: *mut DebugInfo) {
        unsafe {
            pub const HOOK_NAMES: [*const i8; 5] = [c"call".as_ptr(), c"return".as_ptr(), c"line".as_ptr(), c"count".as_ptr(), c"tail call".as_ptr()];
            lua_getfield(interpreter, -(1000000 as i32) - 1000 as i32, HOOKKEY);
            (*interpreter).push_state();
            if lua_rawget(interpreter, -2) == TagType::Closure {
                lua_pushstring(interpreter, HOOK_NAMES[(*debuginfo).debuginfo_event as usize]);
                if (*debuginfo).debuginfo_currentline >= 0 {
                    (*interpreter).push_integer((*debuginfo).debuginfo_currentline as i64);
                } else {
                    (*interpreter).push_nil();
                }
                (*interpreter).lua_callk(2, 0, 0, None);
            }
        }
    }
}
pub unsafe fn lua_getlocal(interpreter: *mut Interpreter, debuginfo: *const DebugInfo, n: i32) -> *const i8 {
    unsafe {
        return if debuginfo.is_null() {
            if (*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))).get_tag_variant() == TagVariant::ClosureL as u8 {
                luaf_getlocalname((*((*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))).value.value_object as *mut Closure)).payload.l_prototype, n, 0)
            } else {
                null()
            }
        } else {
            let mut position: *mut TValue = null_mut();
            let ret = luag_findlocal(interpreter, (*debuginfo).debuginfo_callinfo, n, &mut position);
            if !ret.is_null() {
                (*(*interpreter).top.stkidrel_pointer).copy_from(&*position);
                (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
            }
            ret
        }
    }
}
pub unsafe fn lua_setlocal(interpreter: *mut Interpreter, debuginfo: *const DebugInfo, n: i32) -> *const i8 {
    unsafe {
        let mut position: *mut TValue = null_mut();
        let ret: *const i8 = luag_findlocal(interpreter, (*debuginfo).debuginfo_callinfo, n, &mut position);
        if !ret.is_null() {
            let io1: *mut TValue = &mut (*position);
            let io2: *const TValue = &mut (*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize)));
            (*io1).copy_from(&*io2);
            (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(-1);
        }
        return ret;
    }
}
pub unsafe fn funcinfo(debuginfo: *mut DebugInfo, closure: *mut Closure) {
    unsafe {
        if !(!closure.is_null() && (*closure).get_tag_variant() == TagVariant::ClosureL as u8) {
            (*debuginfo).debuginfo_source = c"=[C]".as_ptr();
            (*debuginfo).debuginfo_sourcelength = (size_of::<[i8; 5]>() as usize) - 1;
            (*debuginfo).debuginfo_linedefined = -1;
            (*debuginfo).debuginfo_lastlinedefined = -1;
            (*debuginfo).debuginfo_what = c"C".as_ptr();
        } else {
            let p: *const Prototype = (*closure).payload.l_prototype;
            if !((*p).prototype_source).is_null() {
                (*debuginfo).debuginfo_source = (*(*p).prototype_source).get_contents_mut();
                (*debuginfo).debuginfo_sourcelength = (*(*p).prototype_source).get_length() as usize;
            } else {
                (*debuginfo).debuginfo_source = c"=?".as_ptr();
                (*debuginfo).debuginfo_sourcelength = (size_of::<[i8; 3]>() as usize) - 1;
            }
            (*debuginfo).debuginfo_linedefined = (*p).prototype_line_defined;
            (*debuginfo).debuginfo_lastlinedefined = (*p).prototype_last_line_defined;
            (*debuginfo).debuginfo_what = if (*debuginfo).debuginfo_linedefined == 0 { c"main".as_ptr() } else { c"Lua".as_ptr() };
        }
        luao_chunkid(((*debuginfo).debuginfo_shortsrc).as_mut_ptr(), (*debuginfo).debuginfo_source, (*debuginfo).debuginfo_sourcelength);
    }
}
pub unsafe fn lua_getinfo(interpreter: *mut Interpreter, mut what: *const i8, debuginfo: *mut DebugInfo) -> i32 {
    unsafe {
        let status: i32;
        let function;
        let callinfo;
        if *what as i32 == Character::AngleRight as i32 {
            callinfo = null_mut();
            function = &mut (*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize)));
            what = what.offset(1);
            (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(-1);
        } else {
            callinfo = (*debuginfo).debuginfo_callinfo;
            function = &mut (*(*callinfo).call_info_function.stkidrel_pointer);
        }
        const TAG_VARIANT_CLOSURE_C: u8 = TagVariant::ClosureC as u8;
        const TAG_VARIANT_CLOSURE_L: u8 = TagVariant::ClosureL as u8;
        match (*function).get_tag_variant() {
            TAG_VARIANT_CLOSURE_L => {
                let closure: *mut Closure = &mut (*((*function).value.value_object as *mut Closure));
                status = auxgetinfo(interpreter, what, debuginfo, closure, callinfo);
                if !(strchr(what, Character::LowerF as i32)).is_null() {
                    let io1: *mut TValue = &mut (*(*interpreter).top.stkidrel_pointer);
                    let io2: *const TValue = function;
                    (*io1).copy_from(&*io2);
                    (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
                }
                if !(strchr(what, Character::UpperL as i32)).is_null() {
                    collectvalidlines(interpreter, closure);
                }
                return status;
            },
            TAG_VARIANT_CLOSURE_C => {
                let closure: *mut Closure = &mut (*((*function).value.value_object as *mut Closure));
                status = auxgetinfo(interpreter, what, debuginfo, closure, callinfo);
                if !(strchr(what, Character::LowerF as i32)).is_null() {
                    let io1: *mut TValue = &mut (*(*interpreter).top.stkidrel_pointer);
                    let io2: *const TValue = function;
                    (*io1).copy_from(&*io2);
                    (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
                }
                if !(strchr(what, Character::UpperL as i32)).is_null() {
                    collectvalidlines(interpreter, closure);
                }
                return status;
            },
            _ => {
                let closure: *mut Closure = null_mut();
                status = auxgetinfo(interpreter, what, debuginfo, closure, callinfo);
                if !(strchr(what, Character::LowerF as i32)).is_null() {
                    let io1: *mut TValue = &mut (*(*interpreter).top.stkidrel_pointer);
                    let io2: *const TValue = function;
                    (*io1).copy_from(&*io2);
                    (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
                }
                if !(strchr(what, Character::UpperL as i32)).is_null() {
                    collectvalidlines(interpreter, closure);
                }
                return status;
            },
        }
    }
}
