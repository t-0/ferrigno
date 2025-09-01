#![allow(
    unpredictable_function_pointer_comparisons,
)]
use crate::callinfo::*;
use crate::utility::c::*;
use crate::state::*;
use crate::utility::*;
use crate::lclosure::*;
use crate::closure::*;
use crate::prototype::*;
use crate::tag::*;
use crate::object::*;
use crate::stackvalue::*;
use crate::tvalue::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DebugInfo {
    pub event: i32,
    pub name: *const i8,
    pub namewhat: *const i8,
    pub what: *const i8,
    pub source: *const i8,
    pub source_length: u64,
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
pub unsafe extern "C" fn lua_getlocal(state: *mut State, ar: *const DebugInfo, n: i32) -> *const i8 {
    unsafe {
        let name;
        if ar.is_null() {
            if !((*(*state).top.p.offset(-(1 as isize)))
                .value
                .get_tag_variant()
                == TAG_VARIANT_CLOSURE_L)
            {
                name = std::ptr::null();
            } else {
                name = luaf_getlocalname(
                    (*((*(*state).top.p.offset(-(1 as isize))).value.value.object as *mut LClosure))
                        .p,
                    n,
                    0,
                );
            }
        } else {
            let mut pos: StkId = std::ptr::null_mut();
            name = luag_findlocal(state, (*ar).i_ci, n, &mut pos);
            if !name.is_null() {
                let io1: *mut TValue = &mut (*(*state).top.p).value;
                let io2: *const TValue = &mut (*pos).value;
                (*io1).value = (*io2).value;
                (*io1).set_tag((*io2).get_tag());
                (*state).top.p = (*state).top.p.offset(1);
            }
        }
        return name;
    }
}
pub unsafe extern "C" fn lua_setlocal(state: *mut State, ar: *const DebugInfo, n: i32) -> *const i8 {
    unsafe {
        let mut pos: StkId = std::ptr::null_mut();
        let name: *const i8 = luag_findlocal(state, (*ar).i_ci, n, &mut pos);
        if !name.is_null() {
            let io1: *mut TValue = &mut (*pos).value;
            let io2: *const TValue = &mut (*(*state).top.p.offset(-(1 as isize))).value;
            (*io1).value = (*io2).value;
            (*io1).set_tag((*io2).get_tag());
            (*state).top.p = (*state).top.p.offset(-1);
        }
        return name;
    }
}
pub unsafe extern "C" fn funcinfo(ar: *mut DebugInfo, cl: *mut UClosure) {
    unsafe {
        if !(!cl.is_null() && (*cl).c.get_tag() == TAG_VARIANT_CLOSURE_L) {
            (*ar).source = b"=[C]\0" as *const u8 as *const i8;
            (*ar).source_length = (::core::mem::size_of::<[i8; 5]>() as u64)
                .wrapping_div(::core::mem::size_of::<i8>() as u64)
                .wrapping_sub(1 as u64);
            (*ar).line_defined = -1;
            (*ar).last_line_defined = -1;
            (*ar).what = b"C\0" as *const u8 as *const i8;
        } else {
            let p: *const Prototype = (*cl).l.p;
            if !((*p).source).is_null() {
                (*ar).source = (*(*p).source).get_contents();
                (*ar).source_length = (*(*p).source).get_length();
            } else {
                (*ar).source = b"=?\0" as *const u8 as *const i8;
                (*ar).source_length = (::core::mem::size_of::<[i8; 3]>() as u64)
                    .wrapping_div(::core::mem::size_of::<i8>() as u64)
                    .wrapping_sub(1 as u64);
            }
            (*ar).line_defined = (*p).line_defined;
            (*ar).last_line_defined = (*p).last_line_defined;
            (*ar).what = if (*ar).line_defined == 0 {
                b"main\0" as *const u8 as *const i8
            } else {
                b"Lua\0" as *const u8 as *const i8
            };
        }
        luao_chunkid(
            ((*ar).short_src).as_mut_ptr(),
            (*ar).source,
            (*ar).source_length,
        );
    }
}
pub unsafe extern "C" fn lua_getinfo(
    state: *mut State,
    mut what: *const i8,
    ar: *mut DebugInfo,
) -> i32 {
    unsafe {
        let status: i32;
        let function;
        let call_info;
        if *what as i32 == '>' as i32 {
            call_info = std::ptr::null_mut();
            function = &mut (*(*state).top.p.offset(-(1 as isize))).value;
            what = what.offset(1);
            (*state).top.p = (*state).top.p.offset(-1);
        } else {
            call_info = (*ar).i_ci;
            function = &mut (*(*call_info).function.p).value;
        }
        match (*function).get_tag_variant() {
            TAG_VARIANT_CLOSURE_L => {
                let cl: *mut UClosure = &mut (*((*function).value.object as *mut UClosure));
                status = auxgetinfo(state, what, ar, cl, call_info);
                if !(strchr(what, 'f' as i32)).is_null() {
                    let io1: *mut TValue = &mut (*(*state).top.p).value;
                    let io2: *const TValue = function;
                    (*io1).value = (*io2).value;
                    (*io1).set_tag((*io2).get_tag());
                    (*state).top.p = (*state).top.p.offset(1);
                }
                if !(strchr(what, 'L' as i32)).is_null() {
                    collectvalidlines(state, cl);
                }
                return status;
            }
            TAG_VARIANT_CLOSURE_C => {
                let cl: *mut UClosure = &mut (*((*function).value.object as *mut UClosure));
                status = auxgetinfo(state, what, ar, cl, call_info);
                if !(strchr(what, 'f' as i32)).is_null() {
                    let io1: *mut TValue = &mut (*(*state).top.p).value;
                    let io2: *const TValue = function;
                    (*io1).value = (*io2).value;
                    (*io1).set_tag((*io2).get_tag());
                    (*state).top.p = (*state).top.p.offset(1);
                }
                if !(strchr(what, 'L' as i32)).is_null() {
                    collectvalidlines(state, cl);
                }
                return status;
            }
            _ => {
                let cl: *mut UClosure = std::ptr::null_mut();
                status = auxgetinfo(state, what, ar, cl, call_info);
                if !(strchr(what, 'f' as i32)).is_null() {
                    let io1: *mut TValue = &mut (*(*state).top.p).value;
                    let io2: *const TValue = function;
                    (*io1).value = (*io2).value;
                    (*io1).set_tag((*io2).get_tag());
                    (*state).top.p = (*state).top.p.offset(1);
                }
                if !(strchr(what, 'L' as i32)).is_null() {
                    collectvalidlines(state, cl);
                }
                return status;
            }
        }
    }
}
