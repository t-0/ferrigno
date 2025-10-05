use crate::tdefaultnew::*;
use crate::interpreter::*;
use crate::lexicalstate::*;
use crate::functionstate::*;
use crate::opcode::*;
use crate::vectort::*;
use crate::labeldescription::*;
use crate::tstring::*;
use std::ptr::*;
#[repr(C)]
pub struct BlockControl {
    m_previous: *mut BlockControl,
    m_first_label: usize,
    m_first_goto: usize,
    m_count_active_variables: usize,
    m_count_up_values: usize,
    m_is_loop: bool,
    m_is_inside_tbc: bool,
}
impl TDefaultNew for BlockControl {
    fn new() -> Self {
        return BlockControl {
            m_previous: null_mut(),
            m_first_label: 0,
            m_first_goto: 0,
            m_count_active_variables: 0,
            m_count_up_values: 0,
            m_is_loop: false,
            m_is_inside_tbc: false,
            ..
        };
    }
}
impl BlockControl {
    pub unsafe fn mark_upvalue_delegated(&mut self, level: i32) {
        unsafe {
            let mut block_control: *mut BlockControl = self;
            while (*block_control).m_count_active_variables as i32 > level {
                block_control = (*block_control).m_previous;
            }
            (*block_control).m_count_up_values = 1;
        }
    }
    pub unsafe fn enterblock(& mut self, lexical_state: *mut LexicalState, function_state: *mut FunctionState, is_loop: bool,
    ) {
        unsafe {
            self.m_is_loop = is_loop;
            self.m_count_active_variables = (*function_state).functionstate_countactivevariables;
            self.m_first_label =
                (*(*lexical_state).lexicalstate_dynamicdata).dynamicdata_labels.get_length();
            self.m_first_goto = (*(*lexical_state).lexicalstate_dynamicdata).dynamicdata_goto.get_length();
            self.m_count_up_values = 0;
            self.m_is_inside_tbc = !((*function_state).functionstate_blockcontrol).is_null()
                && (*(*function_state).functionstate_blockcontrol).m_is_inside_tbc as i32 != 0;
            self.m_previous = (*function_state).functionstate_blockcontrol;
            (*function_state).functionstate_blockcontrol = self;
        }
    }
    pub unsafe fn leaveblock(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState) {
        unsafe {
            let block_control: *mut BlockControl = (*function_state).functionstate_blockcontrol;
            let mut has_close = false;
            let stklevel: i32 = reglevel(
                lexical_state,
                function_state,
                (*block_control).m_count_active_variables as i32,
            );
            removevars(
                lexical_state,
                function_state,
                (*block_control).m_count_active_variables as i32,
            );
            if (*block_control).m_is_loop {
                has_close = (*lexical_state).create_label(
                    interpreter,
                    function_state,
                    luas_newlstr(interpreter, c"break".as_ptr(), "break".len()),
                    0,
                    false,
                );
            }
            if !has_close
                && !((*block_control).m_previous).is_null()
                && (*block_control).m_count_up_values as i32 != 0
            {
                code_abck(interpreter, lexical_state, function_state, OPCODE_CLOSE, stklevel, 0, 0, 0);
            }
            (*function_state).functionstate_freereg = stklevel as u8;
            (*(*lexical_state).lexicalstate_dynamicdata)
                .dynamicdata_labels
                .set_length((*block_control).m_first_label as usize);
            (*function_state).functionstate_blockcontrol = (*block_control).m_previous;
            if !((*block_control).m_previous).is_null() {
                BlockControl::movegotosout(lexical_state, function_state, block_control);
            } else if (*block_control).m_first_goto
                < (*(*lexical_state).lexicalstate_dynamicdata).dynamicdata_goto.get_length()
            {
                undefgoto(
                    interpreter,
                    lexical_state,
                    function_state,
                    &mut *((*(*lexical_state).lexicalstate_dynamicdata).dynamicdata_goto.vectort_pointer)
                        .offset((*block_control).m_first_goto as isize),
                );
            }
        }
    }
    pub fn is_inside_tbc(& self) -> bool {
        self.m_is_inside_tbc
    }
    pub fn get_count_active_variables(& self) -> usize {
        self.m_count_active_variables
    }
    pub fn get_count_upvalues(& self) -> usize {
        self.m_count_up_values
    }
    pub unsafe fn movegotosout(lexical_state: *mut LexicalState, function_state: *mut FunctionState, block_control: *mut BlockControl) {
        unsafe {
            let gl: *mut VectorT<LabelDescription> = &mut (*(*lexical_state).lexicalstate_dynamicdata).dynamicdata_goto;
            for i in (*block_control).m_first_goto..(*gl).get_length() {
                let gt = &mut *((*gl).vectort_pointer).offset(i as isize) as *mut LabelDescription;
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
    pub fn get_first_goto(& self) -> usize {
        self.m_first_goto
    }
    pub unsafe fn marktobeclosed(& mut self) {
        self.m_count_up_values = 1;
        self.m_is_inside_tbc = true;
    }
}
