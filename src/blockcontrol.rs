use crate::tdefaultnew::*;
use crate::interpreter::*;
use crate::lexicalstate::*;
use crate::functionstate::*;
use crate::opcode::*;
use crate::tstring::*;
use std::ptr::*;
#[repr(C)]
pub struct BlockControl {
    pub blockcontrol_previous: *mut BlockControl,
    pub blockcontrol_firstlabel: i32,
    pub blockcontrol_firstgoto: i32,
    pub blockcontrol_countactivevariables: u8,
    pub blockcontrol_countupvalues: u8,
    pub blockcontrol_isloop: bool,
    pub blockcontrol_isinsidetbc: bool,
}
impl TDefaultNew for BlockControl {
    fn new() -> Self {
        return BlockControl {
            blockcontrol_previous: null_mut(),
            blockcontrol_firstlabel: 0,
            blockcontrol_firstgoto: 0,
            blockcontrol_countactivevariables: 0,
            blockcontrol_countupvalues: 0,
            blockcontrol_isloop: false,
            blockcontrol_isinsidetbc: false,
            ..
        };
    }
}
impl BlockControl {
    pub unsafe fn mark_upvalue_delegated(&mut self, level: i32) {
        unsafe {
            let mut block_control: *mut BlockControl = self;
            while (*block_control).blockcontrol_countactivevariables as i32 > level {
                block_control = (*block_control).blockcontrol_previous;
            }
            (*block_control).blockcontrol_countupvalues = 1;
        }
    }
    pub unsafe fn enterblock(
        lexical_state: *mut LexicalState, function_state: *mut FunctionState, block_control: *mut BlockControl, is_loop: bool,
    ) {
        unsafe {
            (*block_control).blockcontrol_isloop = is_loop;
            (*block_control).blockcontrol_countactivevariables = (*function_state).functionstate_countactivevariables;
            (*block_control).blockcontrol_firstlabel =
                (*(*lexical_state).lexicalstate_dynamicdata).dynamicdata_labels.get_length() as i32;
            (*block_control).blockcontrol_firstgoto = (*(*lexical_state).lexicalstate_dynamicdata).dynamicdata_goto.get_length() as i32;
            (*block_control).blockcontrol_countupvalues = 0;
            (*block_control).blockcontrol_isinsidetbc = !((*function_state).functionstate_blockcontrol).is_null()
                && (*(*function_state).functionstate_blockcontrol).blockcontrol_isinsidetbc as i32 != 0;
            (*block_control).blockcontrol_previous = (*function_state).functionstate_blockcontrol;
            (*function_state).functionstate_blockcontrol = block_control;
        }
    }
    pub unsafe fn leaveblock(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState) {
        unsafe {
            let block_control: *mut BlockControl = (*function_state).functionstate_blockcontrol;
            let mut has_close = false;
            let stklevel: i32 = reglevel(
                lexical_state,
                function_state,
                (*block_control).blockcontrol_countactivevariables as i32,
            );
            removevars(
                lexical_state,
                function_state,
                (*block_control).blockcontrol_countactivevariables as i32,
            );
            if (*block_control).blockcontrol_isloop {
                has_close = (*lexical_state).create_label(
                    interpreter,
                    function_state,
                    luas_newlstr(interpreter, c"break".as_ptr(), "break".len()),
                    0,
                    false,
                );
            }
            if !has_close
                && !((*block_control).blockcontrol_previous).is_null()
                && (*block_control).blockcontrol_countupvalues as i32 != 0
            {
                code_abck(interpreter, lexical_state, function_state, OPCODE_CLOSE, stklevel, 0, 0, 0);
            }
            (*function_state).functionstate_freereg = stklevel as u8;
            (*(*lexical_state).lexicalstate_dynamicdata)
                .dynamicdata_labels
                .set_length((*block_control).blockcontrol_firstlabel as usize);
            (*function_state).functionstate_blockcontrol = (*block_control).blockcontrol_previous;
            if !((*block_control).blockcontrol_previous).is_null() {
                movegotosout(lexical_state, function_state, block_control);
            } else if (*block_control).blockcontrol_firstgoto
                < (*(*lexical_state).lexicalstate_dynamicdata).dynamicdata_goto.get_length() as i32
            {
                undefgoto(
                    interpreter,
                    lexical_state,
                    function_state,
                    &mut *((*(*lexical_state).lexicalstate_dynamicdata).dynamicdata_goto.vectort_pointer)
                        .offset((*block_control).blockcontrol_firstgoto as isize),
                );
            }
        }
    }
}
