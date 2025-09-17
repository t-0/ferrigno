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
use rlua::*;
use std::ptr::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DebugInfo {
    pub event: i32,
    pub name: *const i8,
    pub namewhat: *const i8,
    pub what: *const i8,
    pub source: *const i8,
    pub source_length: usize,
    pub currentline: i32,
    pub line_defined: i32,
    pub last_line_defined: i32,
    pub nups: u8,
    pub nparams: u8,
    pub is_variable_arguments: bool,
    pub is_tail_call: bool,
    pub ftransfer: u16,
    pub ntransfer: u16,
    pub short_src: [i8; 60],
    pub i_ci: *mut CallInfo,
}
pub unsafe fn lua_getlocal(interpreter: *mut Interpreter, ar: *const DebugInfo, n: i32) -> *const i8 {
    unsafe {
        let name;
        if ar.is_null() {
            if !((*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))).get_tag_variant() == TAG_VARIANT_CLOSURE_L) {
                name = null();
            } else {
                name = luaf_getlocalname((*((*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize))).value.object as *mut Closure)).payload.l_prototype, n, 0);
            }
        } else {
            let mut position: *mut TValue = null_mut();
            name = luag_findlocal(interpreter, (*ar).i_ci, n, &mut position);
            if !name.is_null() {
                let io1: *mut TValue = &mut (*(*interpreter).top.stkidrel_pointer);
                let io2: *const TValue = &mut (*position);
                (*io1).copy_from(&*io2);
                (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
            }
        }
        return name;
    }
}
pub unsafe fn lua_setlocal(interpreter: *mut Interpreter, ar: *const DebugInfo, n: i32) -> *const i8 {
    unsafe {
        let mut position: *mut TValue = null_mut();
        let name: *const i8 = luag_findlocal(interpreter, (*ar).i_ci, n, &mut position);
        if !name.is_null() {
            let io1: *mut TValue = &mut (*position);
            let io2: *const TValue = &mut (*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize)));
            (*io1).copy_from(&*io2);
            (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(-1);
        }
        return name;
    }
}
pub unsafe fn funcinfo(ar: *mut DebugInfo, cl: *mut Closure) {
    unsafe {
        if !(!cl.is_null() && (*cl).get_tag_variant() == TAG_VARIANT_CLOSURE_L) {
            (*ar).source = make_cstring!("=[C]");
            (*ar).source_length = (size_of::<[i8; 5]>() as usize).wrapping_sub(1 as usize);
            (*ar).line_defined = -1;
            (*ar).last_line_defined = -1;
            (*ar).what = make_cstring!("C");
        } else {
            let p: *const Prototype = (*cl).payload.l_prototype;
            if !((*p).prototype_source).is_null() {
                (*ar).source = (*(*p).prototype_source).get_contents_mut();
                (*ar).source_length = (*(*p).prototype_source).get_length() as usize;
            } else {
                (*ar).source = make_cstring!("=?");
                (*ar).source_length = (size_of::<[i8; 3]>() as usize).wrapping_sub(1 as usize);
            }
            (*ar).line_defined = (*p).prototype_line_defined;
            (*ar).last_line_defined = (*p).prototype_last_line_defined;
            (*ar).what = if (*ar).line_defined == 0 { make_cstring!("main") } else { make_cstring!("Lua") };
        }
        luao_chunkid(((*ar).short_src).as_mut_ptr(), (*ar).source, (*ar).source_length);
    }
}
pub unsafe fn lua_getinfo(interpreter: *mut Interpreter, mut what: *const i8, ar: *mut DebugInfo) -> i32 {
    unsafe {
        let status: i32;
        let function;
        let ci;
        if *what as i32 == CHARACTER_ANGLE_RIGHT as i32 {
            ci = null_mut();
            function = &mut (*(*interpreter).top.stkidrel_pointer.offset(-(1 as isize)));
            what = what.offset(1);
            (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(-1);
        } else {
            ci = (*ar).i_ci;
            function = &mut (*(*ci).call_info_function.stkidrel_pointer);
        }
        match (*function).get_tag_variant() {
            TAG_VARIANT_CLOSURE_L => {
                let cl: *mut Closure = &mut (*((*function).value.object as *mut Closure));
                status = auxgetinfo(interpreter, what, ar, cl, ci);
                if !(strchr(what, CHARACTER_LOWER_F as i32)).is_null() {
                    let io1: *mut TValue = &mut (*(*interpreter).top.stkidrel_pointer);
                    let io2: *const TValue = function;
                    (*io1).copy_from(&*io2);
                    (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
                }
                if !(strchr(what, CHARACTER_UPPER_L as i32)).is_null() {
                    collectvalidlines(interpreter, cl);
                }
                return status;
            },
            TAG_VARIANT_CLOSURE_C => {
                let cl: *mut Closure = &mut (*((*function).value.object as *mut Closure));
                status = auxgetinfo(interpreter, what, ar, cl, ci);
                if !(strchr(what, CHARACTER_LOWER_F as i32)).is_null() {
                    let io1: *mut TValue = &mut (*(*interpreter).top.stkidrel_pointer);
                    let io2: *const TValue = function;
                    (*io1).copy_from(&*io2);
                    (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
                }
                if !(strchr(what, CHARACTER_UPPER_L as i32)).is_null() {
                    collectvalidlines(interpreter, cl);
                }
                return status;
            },
            _ => {
                let cl: *mut Closure = null_mut();
                status = auxgetinfo(interpreter, what, ar, cl, ci);
                if !(strchr(what, CHARACTER_LOWER_F as i32)).is_null() {
                    let io1: *mut TValue = &mut (*(*interpreter).top.stkidrel_pointer);
                    let io2: *const TValue = function;
                    (*io1).copy_from(&*io2);
                    (*interpreter).top.stkidrel_pointer = (*interpreter).top.stkidrel_pointer.offset(1);
                }
                if !(strchr(what, CHARACTER_UPPER_L as i32)).is_null() {
                    collectvalidlines(interpreter, cl);
                }
                return status;
            },
        }
    }
}
