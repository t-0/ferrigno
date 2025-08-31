use crate::blockcontrol::*;
use crate::lexicalstate::*;
use crate::labeldescription::*;
use crate::instruction::*;
use crate::constructorcontrol::*;
use crate::v::*;
use crate::labellist::*;
use crate::onelua::*;
use crate::new::*;
use crate::prototype::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FunctionState {
    pub f: *mut Prototype,
    pub previous: *mut FunctionState,
    pub lexical_state: *mut LexicalState,
    pub block_control: *mut BlockControl,
    pub program_counter: i32,
    pub lasttarget: i32,
    pub previousline: i32,
    pub nk: i32,
    pub np: i32,
    pub nabslineinfo: i32,
    pub firstlocal: i32,
    pub first_label: i32,
    pub ndebugvars: i16,
    pub count_active_variables: u8,
    pub nups: u8,
    pub freereg: u8,
    pub iwthabs: u8,
    pub needclose: u8,
}
impl New for FunctionState {
    fn new() -> Self {
        return FunctionState {
            f: std::ptr::null_mut(),
            previous: std::ptr::null_mut(),
            lexical_state: std::ptr::null_mut(),
            block_control: std::ptr::null_mut(),
            program_counter: 0,
            lasttarget: 0,
            previousline: 0,
            nk: 0,
            np: 0,
            nabslineinfo: 0,
            firstlocal: 0,
            first_label: 0,
            ndebugvars: 0,
            count_active_variables: 0,
            nups: 0,
            freereg: 0,
            iwthabs: 0,
            needclose: 0,
        };
    }
}
pub unsafe extern "C" fn movegotosout(fs: *mut FunctionState, block_control: *mut BlockControl) {
    unsafe {
        let mut i: i32;
        let gl: *mut LabelList = &mut (*(*(*fs).lexical_state).dynamic_data).gt;
        i = (*block_control).first_goto;
        while i < (*gl).n {
            let gt: *mut LabelDescription =
                &mut *((*gl).pointer).offset(i as isize) as *mut LabelDescription;
            if reglevel(fs, (*gt).count_active_variables as i32)
                > reglevel(fs, (*block_control).count_active_variables as i32)
            {
                (*gt).close = ((*gt).close as i32 | (*block_control).count_upvalues as i32) as u8;
            }
            (*gt).count_active_variables = (*block_control).count_active_variables;
            i += 1;
        }
    }
}
pub unsafe extern "C" fn enterblock(
    fs: *mut FunctionState,
    block_control: *mut BlockControl,
    is_loop: bool,
) {
    unsafe {
        (*block_control).is_loop = is_loop;
        (*block_control).count_active_variables = (*fs).count_active_variables;
        (*block_control).first_label = (*(*(*fs).lexical_state).dynamic_data).label.n;
        (*block_control).first_goto = (*(*(*fs).lexical_state).dynamic_data).gt.n;
        (*block_control).count_upvalues = 0;
        (*block_control).is_inside_tbc =
            !((*fs).block_control).is_null() && (*(*fs).block_control).is_inside_tbc as i32 != 0;
        (*block_control).previous = (*fs).block_control;
        (*fs).block_control = block_control;
    }
}
pub unsafe extern "C" fn leaveblock(fs: *mut FunctionState) {
    unsafe {
        let block_control: *mut BlockControl = (*fs).block_control;
        let lexical_state: *mut LexicalState = (*fs).lexical_state;
        let mut hasclose: i32 = 0;
        let stklevel: i32 = reglevel(fs, (*block_control).count_active_variables as i32);
        removevars(fs, (*block_control).count_active_variables as i32);
        if (*block_control).is_loop {
            hasclose = createlabel(
                lexical_state,
                luas_newlstr(
                    (*lexical_state).state,
                    b"break\0" as *const u8 as *const i8,
                    (::core::mem::size_of::<[i8; 6]>() as u64)
                        .wrapping_div(::core::mem::size_of::<i8>() as u64)
                        .wrapping_sub(1 as u64),
                ),
                0,
                0,
            );
        }
        if hasclose == 0
            && !((*block_control).previous).is_null()
            && (*block_control).count_upvalues as i32 != 0
        {
            luak_code_abck(fs, OP_CLOSE, stklevel, 0, 0, 0);
        }
        (*fs).freereg = stklevel as u8;
        (*(*lexical_state).dynamic_data).label.n = (*block_control).first_label;
        (*fs).block_control = (*block_control).previous;
        if !((*block_control).previous).is_null() {
            movegotosout(fs, block_control);
        } else if (*block_control).first_goto < (*(*lexical_state).dynamic_data).gt.n {
            undefgoto(
                lexical_state,
                &mut *((*(*lexical_state).dynamic_data).gt.pointer)
                    .offset((*block_control).first_goto as isize),
            );
        }
    }
}
pub unsafe extern "C" fn closelistfield(fs: *mut FunctionState, cc: *mut ConstructorControl) {
    unsafe {
        if (*cc).v.k as u32 == VVOID as u32 {
            return;
        }
        luak_exp2nextreg(fs, &mut (*cc).v);
        (*cc).v.k = VVOID;
        if (*cc).tostore == 50 as i32 {
            luak_setlist(fs, (*(*cc).t).u.info, (*cc).na, (*cc).tostore);
            (*cc).na += (*cc).tostore;
            (*cc).tostore = 0;
        }
    }
}
pub unsafe extern "C" fn lastlistfield(fs: *mut FunctionState, cc: *mut ConstructorControl) {
    unsafe {
        if (*cc).tostore == 0 {
            return;
        }
        if (*cc).v.k as u32 == VCALL as u32 || (*cc).v.k as u32 == VVARARG as u32 {
            luak_setreturns(fs, &mut (*cc).v, -1);
            luak_setlist(fs, (*(*cc).t).u.info, (*cc).na, -1);
            (*cc).na -= 1;
            (*cc).na;
        } else {
            if (*cc).v.k as u32 != VVOID as u32 {
                luak_exp2nextreg(fs, &mut (*cc).v);
            }
            luak_setlist(fs, (*(*cc).t).u.info, (*cc).na, (*cc).tostore);
        }
        (*cc).na += (*cc).tostore;
    }
}
pub unsafe extern "C" fn setvararg(fs: *mut FunctionState, nparams: i32) {
    unsafe {
        (*(*fs).f).is_variable_arguments = true;
        luak_code_abck(fs, OP_VARARGPREP, nparams, 0, 0, 0);
    }
}
