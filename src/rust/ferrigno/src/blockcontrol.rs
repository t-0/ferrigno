use crate::functionstate::*;
use crate::labeldescription::*;
use crate::lexicalstate::*;
use crate::opcode::*;
use crate::state::*;
use crate::tdefaultnew::*;
use crate::tstring::*;
use crate::vectort::*;
use std::ptr::*;
#[repr(C)]
pub struct BlockControl {
    blockcontrol_previous: *mut BlockControl,
    blockcontrol_is_loop: bool,
    blockcontrol_is_inside_tbc: bool,
    blockcontrol_first_label: usize,
    blockcontrol_first_goto: usize,
    blockcontrol_count_active_variables: usize,
    blockcontrol_count_upvalues: usize,
}
impl TDefaultNew for BlockControl {
    fn new() -> Self {
        BlockControl {
            blockcontrol_previous: null_mut(),
            blockcontrol_first_label: 0,
            blockcontrol_first_goto: 0,
            blockcontrol_count_active_variables: 0,
            blockcontrol_count_upvalues: 0,
            blockcontrol_is_loop: false,
            blockcontrol_is_inside_tbc: false,
        }
    }
}
impl BlockControl {
    pub unsafe fn mark_upvalue_delegated(&mut self, level: usize) {
        unsafe {
            let mut it: *mut BlockControl = self;
            while (*it).blockcontrol_count_active_variables > level {
                it = (*it).blockcontrol_previous;
            }
            (*it).blockcontrol_count_upvalues = 1;
        }
    }
    pub unsafe fn enter_block(&mut self, lexical_state: *mut LexicalState, function_state: *mut FunctionState, is_loop: bool) {
        unsafe {
            self.blockcontrol_is_loop = is_loop;
            self.blockcontrol_count_active_variables = (*function_state).functionstate_count_active_variables;
            self.blockcontrol_first_label = (*(*lexical_state).lexicalstate_dynamicdata).dynamicdata_labels.get_length();
            self.blockcontrol_first_goto = (*(*lexical_state).lexicalstate_dynamicdata).dynamicdata_goto.get_length();
            self.blockcontrol_count_upvalues = 0;
            self.blockcontrol_is_inside_tbc = !((*function_state).functionstate_blockcontrol).is_null()
                && (*(*function_state).functionstate_blockcontrol).blockcontrol_is_inside_tbc;
            self.blockcontrol_previous = (*function_state).functionstate_blockcontrol;
            (*function_state).functionstate_blockcontrol = self;
        }
    }
    pub unsafe fn leave_block(&mut self, state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState) {
        unsafe {
            let mut has_close = false;
            let stklevel: i32 = reglevel(lexical_state, function_state, self.blockcontrol_count_active_variables as i32);
            removevars(lexical_state, function_state, self.blockcontrol_count_active_variables as i32);
            if self.blockcontrol_is_loop {
                has_close = (*lexical_state).create_label(
                    state,
                    function_state,
                    luas_newlstr(state, c"break".as_ptr(), "break".len()),
                    0,
                    false,
                );
            }
            if !has_close && !(self.blockcontrol_previous).is_null() && self.blockcontrol_count_upvalues != 0 {
                code_abck(state, lexical_state, function_state, OPCODE_CLOSE, stklevel, 0, 0, 0);
            }
            (*function_state).functionstate_free_register = stklevel as u8;
            (*(*lexical_state).lexicalstate_dynamicdata)
                .dynamicdata_labels
                .set_length(self.blockcontrol_first_label);
            (*function_state).functionstate_blockcontrol = self.blockcontrol_previous;
            if !(self.blockcontrol_previous).is_null() {
                BlockControl::movegotosout(lexical_state, function_state, self);
            } else if self.blockcontrol_first_goto < (*(*lexical_state).lexicalstate_dynamicdata).dynamicdata_goto.get_length() {
                undefgoto(
                    state,
                    lexical_state,
                    function_state,
                    &mut *((*(*lexical_state).lexicalstate_dynamicdata).dynamicdata_goto.vectort_pointer)
                        .add(self.blockcontrol_first_goto),
                );
            }
        }
    }
    pub fn is_inside_tbc(&self) -> bool {
        self.blockcontrol_is_inside_tbc
    }
    pub fn get_count_active_variables(&self) -> usize {
        self.blockcontrol_count_active_variables
    }
    pub fn get_count_upvalues(&self) -> usize {
        self.blockcontrol_count_upvalues
    }
    pub unsafe fn movegotosout(
        lexical_state: *mut LexicalState, function_state: *mut FunctionState, block_control: *mut BlockControl,
    ) {
        unsafe {
            let gl: *mut VectorT<LabelDescription> = &mut (*(*lexical_state).lexicalstate_dynamicdata).dynamicdata_goto;
            for i in (*block_control).blockcontrol_first_goto..(*gl).get_length() {
                let gt = &mut *((*gl).vectort_pointer).add(i) as *mut LabelDescription;
                if reglevel(
                    lexical_state,
                    function_state,
                    (*gt).labeldescription_countactivevariables as i32,
                ) > reglevel(
                    lexical_state,
                    function_state,
                    (*block_control).get_count_active_variables() as i32,
                ) {
                    (*gt).labeldescription_close =
                        ((*gt).labeldescription_close as i32 | (*block_control).get_count_upvalues() as i32) as u8;
                }
                (*gt).labeldescription_countactivevariables = (*block_control).get_count_active_variables();
            }
        }
    }
    pub fn get_first_goto(&self) -> usize {
        self.blockcontrol_first_goto
    }
    pub unsafe fn marktobeclosed(&mut self) {
        self.blockcontrol_count_upvalues = 1;
        self.blockcontrol_is_inside_tbc = true;
    }
}
