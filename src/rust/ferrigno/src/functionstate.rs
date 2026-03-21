use crate::absolutelineinfo::*;
use crate::blockcontrol::*;
use crate::constructorcontrol::*;
use crate::expressiondescription::*;
use crate::expressionkind::*;
use crate::f2i::*;
use crate::instruction::*;
use crate::lexicalstate::*;
use crate::localvariable::*;
use crate::object::*;
use crate::opcode::*;
use crate::operator_::*;
use crate::operatorbinary::*;
use crate::operatorunary::*;
use crate::opmode::*;
use crate::prototype::*;
use crate::state::*;
use crate::table::*;
use crate::tagvariant::*;
use crate::tdefaultnew::*;
use crate::tm::*;
use crate::tobject::*;
use crate::tstring::*;
use crate::tvalue::*;
use crate::upvaluedescription::*;
use crate::utility::*;
use crate::variabledescription::*;
use std::ptr::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FunctionState {
    pub functionstate_prototype: *mut Prototype,
    pub functionstate_previous: *mut FunctionState,
    pub functionstate_blockcontrol: *mut BlockControl,
    pub functionstate_program_counter: i32,
    pub functionstate_last_target: i32,
    pub functionstate_previous_line: i32,
    pub functionstate_count_constants: i32,
    pub functionstate_count_prototypes: i32,
    pub functionstate_count_absolute_line_info: i32,
    pub functionstate_first_local: i32,
    pub functionstate_first_label: i32,
    pub functionstate_count_debug_variables: usize,
    pub functionstate_count_active_variables: usize,
    pub functionstate_count_upvalues: usize,
    pub functionstate_free_register: u8,
    pub functionstate_i_width_absolute: u8,
    pub functionstate_needs_close: bool,
}
impl TDefaultNew for FunctionState {
    fn new() -> Self {
        FunctionState {
            functionstate_prototype: null_mut(),
            functionstate_previous: null_mut(),
            functionstate_blockcontrol: null_mut(),
            functionstate_program_counter: 0,
            functionstate_last_target: 0,
            functionstate_previous_line: 0,
            functionstate_count_constants: 0,
            functionstate_count_prototypes: 0,
            functionstate_count_absolute_line_info: 0,
            functionstate_first_local: 0,
            functionstate_first_label: 0,
            functionstate_count_debug_variables: 0,
            functionstate_count_active_variables: 0,
            functionstate_count_upvalues: 0,
            functionstate_free_register: 0,
            functionstate_i_width_absolute: 0,
            functionstate_needs_close: false,
        }
    }
}
impl FunctionState {
    pub unsafe fn marktobeclosed(&mut self) {
        unsafe {
            (*self.functionstate_blockcontrol).marktobeclosed();
            self.functionstate_needs_close = true;
        }
    }
    pub fn get_first_goto(&self) -> usize {
        unsafe { (*self.functionstate_blockcontrol).get_first_goto() }
    }
    pub fn code_get_label(&mut self) -> i32 {
        self.functionstate_last_target = self.functionstate_program_counter;
        self.functionstate_program_counter
    }
    pub unsafe fn mark_upvalue(&mut self, level: usize) {
        unsafe {
            (*self.functionstate_blockcontrol).mark_upvalue_delegated(level);
            self.functionstate_needs_close = true;
        }
    }
}
pub unsafe fn closelistfield(
    state: *mut State,
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    constructor_control: *mut ConstructorControl,
) {
    unsafe {
        luak_exp2nextreg(
            state,
            lexical_state,
            function_state,
            &mut (*constructor_control).constructorcontrol_expressiondescription,
        );
        (*constructor_control)
            .constructorcontrol_expressiondescription
            .expressiondescription_expressionkind = ExpressionKind::Void;
        if (*constructor_control).constructorcontrol_count_to_store >= (*constructor_control).constructorcontrol_max_to_store {
            luak_setlist(
                state,
                lexical_state,
                function_state,
                (*(*constructor_control).constructorcontrol_table)
                    .expressiondescription_value
                    .value_info,
                (*constructor_control).constructorcontrol_count_array as usize,
                (*constructor_control).constructorcontrol_count_to_store,
            );
            (*constructor_control).constructorcontrol_count_array += (*constructor_control).constructorcontrol_count_to_store;
            (*constructor_control).constructorcontrol_count_to_store = 0;
        }
    }
}
pub unsafe fn lastlistfield(
    state: *mut State,
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    constructor_control: *mut ConstructorControl,
) {
    unsafe {
        if (*constructor_control).constructorcontrol_count_to_store == 0 {
            return;
        }
        if (*constructor_control)
            .constructorcontrol_expressiondescription
            .expressiondescription_expressionkind
            == ExpressionKind::Call
            || (*constructor_control)
                .constructorcontrol_expressiondescription
                .expressiondescription_expressionkind
                == ExpressionKind::VariableArguments
        {
            luak_setreturns(
                state,
                lexical_state,
                function_state,
                &mut (*constructor_control).constructorcontrol_expressiondescription,
                -1,
            );
            luak_setlist(
                state,
                lexical_state,
                function_state,
                (*(*constructor_control).constructorcontrol_table)
                    .expressiondescription_value
                    .value_info,
                (*constructor_control).constructorcontrol_count_array as usize,
                -1,
            );
            (*constructor_control).constructorcontrol_count_array -= 1;
            (*constructor_control).constructorcontrol_count_array;
        } else {
            if (*constructor_control)
                .constructorcontrol_expressiondescription
                .expressiondescription_expressionkind
                != ExpressionKind::Void
            {
                luak_exp2nextreg(
                    state,
                    lexical_state,
                    function_state,
                    &mut (*constructor_control).constructorcontrol_expressiondescription,
                );
            }
            luak_setlist(
                state,
                lexical_state,
                function_state,
                (*(*constructor_control).constructorcontrol_table)
                    .expressiondescription_value
                    .value_info,
                (*constructor_control).constructorcontrol_count_array as usize,
                (*constructor_control).constructorcontrol_count_to_store,
            );
        }
        (*constructor_control).constructorcontrol_count_array += (*constructor_control).constructorcontrol_count_to_store;
    }
}
pub unsafe fn setvararg(state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState, nparams: i32) {
    unsafe {
        (*(*function_state).functionstate_prototype).prototype_isvariablearguments = true;
        code_abck(
            state,
            lexical_state,
            function_state,
            OPCODE_VARARGPREP,
            nparams,
            0,
            0,
            0,
        );
    }
}
pub unsafe fn errorlimit(
    state: *mut State,
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    limit: i32,
    what: *const i8,
) -> ! {
    unsafe {
        let line: i32 = (*(*function_state).functionstate_prototype).prototype_linedefined;
        let location: *const i8 = if line == 0 {
            c"main function".as_ptr()
        } else {
            luao_pushfstring(state, c"function at line %d".as_ptr(), &[line.into()])
        };
        let message: *const i8 = luao_pushfstring(
            state,
            c"too many %s (limit is %d) in %s".as_ptr(),
            &[what.into(), limit.into(), location.into()],
        );
        luax_syntaxerror(state, lexical_state, message);
    }
}
pub unsafe fn checklimit(
    state: *mut State,
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    v: i32,
    length: i32,
    what: *const i8,
) {
    unsafe {
        if v > length {
            errorlimit(state, lexical_state, function_state, length, what);
        }
    }
}
pub unsafe fn getlocalvardesc(
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    vidx: i32,
) -> *mut VariableDescription {
    unsafe {
        &mut *((*(*lexical_state).lexicalstate_dynamicdata)
            .dynamicdata_active_variables
            .vectort_pointer)
            .add(((*function_state).functionstate_first_local + vidx) as usize) as *mut VariableDescription
    }
}
pub unsafe fn reglevel(lexical_state: *mut LexicalState, function_state: *mut FunctionState, mut nvar: i32) -> i32 {
    unsafe {
        loop {
            let prev_nvar = nvar;
            nvar -= 1;
            if prev_nvar <= 0 {
                break;
            }
            let variable_description: *mut VariableDescription = getlocalvardesc(lexical_state, function_state, nvar);
            let kind = (*variable_description)
                .variabledescription_content
                .variabledescriptioncontent_kind as i32;
            if kind != RDKCTC && kind < GDKREG {
                return (*variable_description)
                    .variabledescription_content
                    .variabledescriptioncontent_registerindex as i32
                    + 1;
            }
        }
        0
    }
}
pub unsafe fn luay_nvarstack(lexical_state: *mut LexicalState, function_state: *mut FunctionState) -> i32 {
    unsafe {
        reglevel(
            lexical_state,
            function_state,
            (*function_state).functionstate_count_active_variables as i32,
        )
    }
}
pub unsafe fn localdebuginfo(
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    vidx: i32,
) -> *mut LocalVariable {
    unsafe {
        let variable_description: *mut VariableDescription = getlocalvardesc(lexical_state, function_state, vidx);
        let kind = (*variable_description)
            .variabledescription_content
            .variabledescriptioncontent_kind as i32;
        if kind == RDKCTC || kind >= GDKREG {
            null_mut()
        } else {
            let index: i32 = (*variable_description)
                .variabledescription_content
                .variabledescriptioncontent_pidx as i32;
            (*(*function_state).functionstate_prototype)
                .prototype_localvariables
                .at_mut(index as isize)
        }
    }
}
pub unsafe fn init_var(
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    expression_description: *mut ExpressionDescription,
    vidx: i32,
) {
    unsafe {
        (*expression_description).expressiondescription_t = -1;
        (*expression_description).expressiondescription_f = (*expression_description).expressiondescription_t;
        (*expression_description).expressiondescription_expressionkind = ExpressionKind::Local;
        (*expression_description)
            .expressiondescription_value
            .value_variable
            .valueregister_valueindex = vidx as u16;
        (*expression_description)
            .expressiondescription_value
            .value_variable
            .valueregister_registerindex = (*getlocalvardesc(lexical_state, function_state, vidx))
            .variabledescription_content
            .variabledescriptioncontent_registerindex;
    }
}
pub unsafe fn removevars(lexical_state: *mut LexicalState, function_state: *mut FunctionState, tolevel: i32) {
    unsafe {
        (*(*lexical_state).lexicalstate_dynamicdata)
            .dynamicdata_active_variables
            .subtract_length(((*function_state).functionstate_count_active_variables as i32 - tolevel) as usize);
        while (*function_state).functionstate_count_active_variables as i32 > tolevel {
            (*function_state).functionstate_count_active_variables -= 1;
            let localvariable: *mut LocalVariable = localdebuginfo(
                lexical_state,
                function_state,
                (*function_state).functionstate_count_active_variables as i32,
            );
            if !localvariable.is_null() {
                (*localvariable).localvariable_endprogramcounter = (*function_state).functionstate_program_counter;
            }
        }
    }
}
pub unsafe fn searchupvalue(function_state: *mut FunctionState, name: *mut TString) -> i32 {
    unsafe {
        let upvaluedescription: *mut UpValueDescription = (*(*function_state).functionstate_prototype)
            .prototype_upvalues
            .vectort_pointer;
        for i in 0..(*function_state).functionstate_count_upvalues {
            if (*upvaluedescription.add(i)).upvaluedescription_name == name {
                return i as i32;
            }
        }
        -1
    }
}
pub unsafe fn allocate_upvalue_description(
    state: *mut State,
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    prototype: *mut Prototype,
) -> *mut UpValueDescription {
    unsafe {
        let mut oldsize = (*prototype).prototype_upvalues.get_size();
        checklimit(
            state,
            lexical_state,
            function_state,
            (*function_state).functionstate_count_upvalues as i32 + 1,
            MAXUPVAL as i32,
            c"upvalues".as_ptr(),
        );
        (*prototype).prototype_upvalues.grow(
            state,
            (*function_state).functionstate_count_upvalues,
            if MAXUPVAL <= (!0usize) / size_of::<UpValueDescription>() {
                MAXUPVAL
            } else {
                (!0usize) / size_of::<UpValueDescription>()
            },
            c"upvalues".as_ptr(),
        );
        while oldsize < (*prototype).prototype_upvalues.get_size() {
            let prev_size = oldsize;
            oldsize += 1;
            let name_slot = &mut (*((*prototype).prototype_upvalues.vectort_pointer).add(prev_size)).upvaluedescription_name;
            *name_slot = null_mut();
        }
        let count_upvalues = (*function_state).functionstate_count_upvalues;
        (*function_state).functionstate_count_upvalues += 1;
        &mut *((*prototype).prototype_upvalues.vectort_pointer).add(count_upvalues) as *mut UpValueDescription
    }
}
pub unsafe fn newupvalue(
    state: *mut State,
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    name: *mut TString,
    v: *mut ExpressionDescription,
) -> i32 {
    unsafe {
        let upvalue_description: *mut UpValueDescription = allocate_upvalue_description(
            state,
            lexical_state,
            function_state,
            (*function_state).functionstate_prototype,
        );
        let previous: *mut FunctionState = (*function_state).functionstate_previous;
        if (*v).expressiondescription_expressionkind == ExpressionKind::Local {
            (*upvalue_description).upvaluedescription_isinstack = true;
            (*upvalue_description).upvaluedescription_index = (*v)
                .expressiondescription_value
                .value_variable
                .valueregister_registerindex;
            (*upvalue_description).upvaluedescription_kind = (*getlocalvardesc(
                lexical_state,
                previous,
                (*v).expressiondescription_value
                    .value_variable
                    .valueregister_valueindex as i32,
            ))
            .variabledescription_content
            .variabledescriptioncontent_kind;
        } else {
            (*upvalue_description).upvaluedescription_isinstack = false;
            (*upvalue_description).upvaluedescription_index = (*v).expressiondescription_value.value_info as u8;
            (*upvalue_description).upvaluedescription_kind = (*((*(*previous).functionstate_prototype)
                .prototype_upvalues
                .vectort_pointer)
                .add((*v).expressiondescription_value.value_info as usize))
            .upvaluedescription_kind;
        }
        (*upvalue_description).upvaluedescription_name = name;
        if (*(*function_state).functionstate_prototype).get_marked() & BLACKBIT != 0
            && (*name).get_marked() & (WHITE0BIT | WHITE1BIT) != 0
        {
            Object::luac_barrier_(
                state,
                &mut *((*function_state).functionstate_prototype as *mut Object),
                &mut *(name as *mut Object),
            );
        }
        (*function_state).functionstate_count_upvalues as i32 - 1
    }
}
pub unsafe fn searchvar(
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    n: *mut TString,
    var: *mut ExpressionDescription,
) -> i32 {
    unsafe {
        let mut i = (*function_state).functionstate_count_active_variables as i32 - 1;
        while i >= 0 {
            let variable_description: *mut VariableDescription = getlocalvardesc(lexical_state, function_state, i);
            let kind = (*variable_description)
                .variabledescription_content
                .variabledescriptioncontent_kind as i32;
            if kind >= GDKREG {
                // global declaration
                if (*variable_description)
                    .variabledescription_content
                    .variabledescriptioncontent_name
                    .is_null()
                {
                    // collective declaration (global *)
                    if (*var).expressiondescription_value.value_info < 0 {
                        (*var).expressiondescription_value.value_info = (*function_state).functionstate_first_local + i;
                    }
                } else if n
                    == (*variable_description)
                        .variabledescription_content
                        .variabledescriptioncontent_name
                {
                    // found matching global name
                    ExpressionDescription::init_exp(
                        var,
                        ExpressionKind::Global,
                        (*function_state).functionstate_first_local + i,
                    );
                    return ExpressionKind::Global as i32;
                } else if (*var).expressiondescription_value.value_info == -1 {
                    // active preambular declaration invalidated
                    (*var).expressiondescription_value.value_info = -2;
                }
            } else if n
                == (*variable_description)
                    .variabledescription_content
                    .variabledescriptioncontent_name
            {
                if kind == RDKCTC {
                    ExpressionDescription::init_exp(
                        var,
                        ExpressionKind::Constant2,
                        (*function_state).functionstate_first_local + i,
                    );
                } else {
                    init_var(lexical_state, function_state, var, i);
                    if kind == RDKVAVAR {
                        (*var).expressiondescription_expressionkind = ExpressionKind::VarargVariable;
                    }
                }
                return (*var).expressiondescription_expressionkind as i32;
            }
            i -= 1;
        }
        -1
    }
}
pub unsafe fn singlevaraux(
    state: *mut State,
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    n: *mut TString,
    var: *mut ExpressionDescription,
    base: i32,
) {
    unsafe {
        if function_state.is_null() {
            // only set to Void if not already Global (preserves global declaration info)
            if (*var).expressiondescription_expressionkind != ExpressionKind::Global {
                ExpressionDescription::init_exp(var, ExpressionKind::Void, 0);
            }
        } else {
            let v: i32 = searchvar(lexical_state, function_state, n, var);
            if v >= 0 {
                if base == 0 {
                    if (*var).expressiondescription_expressionkind == ExpressionKind::VarargVariable {
                        // vararg parameter used as upvalue: needs a real table
                        (*(*function_state).functionstate_prototype).prototype_needsvarargtable = true;
                        (*var).expressiondescription_expressionkind = ExpressionKind::Local;
                    }
                    if (*var).expressiondescription_expressionkind == ExpressionKind::Local {
                        (*function_state).mark_upvalue(
                            (*var)
                                .expressiondescription_value
                                .value_variable
                                .valueregister_valueindex as usize,
                        );
                    }
                }
            } else {
                let mut index: i32 = searchupvalue(function_state, n);
                if index < 0 {
                    if !(*function_state).functionstate_previous.is_null() {
                        singlevaraux(
                            state,
                            lexical_state,
                            (*function_state).functionstate_previous,
                            n,
                            var,
                            0,
                        );
                    }
                    if (*var).expressiondescription_expressionkind == ExpressionKind::Local
                        || (*var).expressiondescription_expressionkind == ExpressionKind::UpValue
                    {
                        index = newupvalue(state, lexical_state, function_state, n, var);
                    } else {
                        return;
                    }
                }
                ExpressionDescription::init_exp(var, ExpressionKind::UpValue, index);
            }
        };
    }
}
pub unsafe fn fixforjump(
    state: *mut State,
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    program_counter: i32,
    dest: i32,
    back: i32,
) {
    unsafe {
        let jmp: *mut u32 = &mut *((*(*function_state).functionstate_prototype)
            .prototype_code
            .vectort_pointer)
            .add(program_counter as usize) as *mut u32;
        let mut offset: i32 = dest - (program_counter + 1);
        if back != 0 {
            offset = -offset;
        }
        if offset > MAXARG_BX {
            luax_syntaxerror(state, lexical_state, c"control structure too long".as_ptr());
        }
        *jmp = *jmp & !(MASK_BX << POSITION_K) | (offset as u32) << POSITION_K & MASK_BX << POSITION_K;
    }
}
pub unsafe fn checktoclose(state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState, level: i32) {
    unsafe {
        if level != -1 {
            (*function_state).marktobeclosed();
            code_abck(
                state,
                lexical_state,
                function_state,
                OPCODE_TBC,
                reglevel(lexical_state, function_state, level),
                0,
                0,
                0,
            );
        }
    }
}
pub unsafe fn previousinstruction(function_state: *mut FunctionState) -> *mut u32 {
    unsafe {
        pub const INVALID_INSTRUCTION: u32 = 0xFFFFFFFFu32;
        if (*function_state).functionstate_program_counter > (*function_state).functionstate_last_target {
            &mut *((*(*function_state).functionstate_prototype)
                .prototype_code
                .vectort_pointer)
                .add(((*function_state).functionstate_program_counter - 1) as usize) as *mut u32
        } else {
            &INVALID_INSTRUCTION as *const u32 as *mut u32
        }
    }
}
pub unsafe fn code_constant_nil(
    state: *mut State,
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    mut from: i32,
    n: i32,
) {
    unsafe {
        let mut length: i32 = from + n - 1;
        let previous: *mut u32 = previousinstruction(function_state);
        if (*previous & MASK_OP) == OPCODE_LOADNIL {
            let pfrom: i32 = (*previous >> POSITION_A & MASK_A) as i32;
            let pl: i32 = pfrom + (*previous >> POSITION_B & MASK_A) as i32;
            if pfrom <= from && from <= pl + 1 || from <= pfrom && pfrom <= length + 1 {
                if pfrom < from {
                    from = pfrom;
                }
                if pl > length {
                    length = pl;
                }
                *previous = *previous & !(MASK_A << POSITION_A) | (from as u32) << POSITION_A & MASK_A << POSITION_A;
                *previous = *previous & !(MASK_B << POSITION_B) | ((length - from) as u32) << POSITION_B & MASK_B << POSITION_B;
                return;
            }
        }
        code_abck(
            state,
            lexical_state,
            function_state,
            OPCODE_LOADNIL,
            from,
            n - 1,
            0,
            0,
        );
    }
}
pub unsafe fn code_get_jump(function_state: *mut FunctionState, program_counter: i32) -> i32 {
    unsafe {
        let offset: i32 = (*((*(*function_state).functionstate_prototype)
            .prototype_code
            .vectort_pointer)
            .add(program_counter as usize)
            >> POSITION_A
            & MASK_AX) as i32
            - OFFSET_SJ;
        if offset == -1 {
            -1
        } else {
            program_counter + 1 + offset
        }
    }
}
pub unsafe fn fixjump(
    state: *mut State,
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    program_counter: i32,
    dest: i32,
) {
    unsafe {
        let jmp: *mut u32 = &mut *((*(*function_state).functionstate_prototype)
            .prototype_code
            .vectort_pointer)
            .add(program_counter as usize) as *mut u32;
        let offset: i32 = dest - (program_counter + 1);
        if !(-OFFSET_SJ <= offset && offset <= MAXARG_AX as i32 - OFFSET_SJ) {
            luax_syntaxerror(state, lexical_state, c"control structure too long".as_ptr());
        }
        *jmp = *jmp & !(MASK_AX << POSITION_A) | ((offset + OFFSET_SJ) as u32) << POSITION_A & MASK_AX << POSITION_A;
    }
}
pub unsafe fn luak_concat(
    state: *mut State,
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    l1: *mut i32,
    l2: i32,
) {
    unsafe {
        if l2 == -1 {
        } else if *l1 == -1 {
            *l1 = l2;
        } else {
            let mut list: i32 = *l1;
            let mut next: i32;
            loop {
                next = code_get_jump(function_state, list);
                if next == -1 {
                    break;
                }
                list = next;
            }
            fixjump(state, lexical_state, function_state, list, l2);
        }
    }
}
pub unsafe fn luak_jump(state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState) -> i32 {
    unsafe { codesj(state, lexical_state, function_state, OPCODE_JMP, -1, 0) }
}
pub unsafe fn luak_ret(
    state: *mut State,
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    first: i32,
    nret: i32,
) {
    unsafe {
        let op: u32;
        match nret {
            0 => {
                op = OPCODE_RETURN0;
            }
            1 => {
                op = OPCODE_RETURN1;
            }
            _ => {
                op = OPCODE_RETURN;
            }
        }
        checklimit(
            state,
            lexical_state,
            function_state,
            nret + 1,
            MAXARG_B,
            c"returns".as_ptr(),
        );
        code_abck(
            state,
            lexical_state,
            function_state,
            op,
            first,
            nret + 1,
            0,
            0,
        );
    }
}
pub unsafe fn condjump(
    state: *mut State,
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    op: u32,
    a: i32,
    b: i32,
    c: i32,
    k: i32,
) -> i32 {
    unsafe {
        code_abck(state, lexical_state, function_state, op, a, b, c, k);
        luak_jump(state, lexical_state, function_state)
    }
}
pub unsafe fn code_get_jump_control(function_state: *mut FunctionState, program_counter: i32) -> *mut u32 {
    unsafe {
        let pi: *mut u32 = &mut *((*(*function_state).functionstate_prototype)
            .prototype_code
            .vectort_pointer)
            .add(program_counter as usize) as *mut u32;
        if program_counter >= 1 && OPMODES[(*pi.sub(1) & MASK_OP) as usize] as i32 & OPMODE_T != 0 {
            pi.sub(1)
        } else {
            pi
        }
    }
}
pub unsafe fn patchtestreg(function_state: *mut FunctionState, node: i32, reg: i32) -> i32 {
    unsafe {
        let i: *mut u32 = code_get_jump_control(function_state, node);
        if (*i & MASK_OP) != OPCODE_TESTSET {
            return 0;
        }
        if reg != NO_REG && reg != (*i >> POSITION_B & MASK_A) as i32 {
            *i = *i & !(MASK_A << POSITION_A) | (reg as u32) << POSITION_A & MASK_A << POSITION_A;
        } else {
            *i = OPCODE_TEST
                | (*i >> POSITION_B & MASK_A) << POSITION_A
                | 0 << POSITION_B
                | 0 << POSITION_C
                | (*i >> POSITION_K & MASK_K) << POSITION_K;
        }
        1
    }
}
pub unsafe fn removevalues(function_state: *mut FunctionState, mut list: i32) {
    unsafe {
        while list != -1 {
            patchtestreg(function_state, list, NO_REG);
            list = code_get_jump(function_state, list);
        }
    }
}
pub unsafe fn patchlistaux(
    state: *mut State,
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    mut list: i32,
    vtarget: i32,
    reg: i32,
    dtarget: i32,
) {
    unsafe {
        while list != -1 {
            let next: i32 = code_get_jump(function_state, list);
            if patchtestreg(function_state, list, reg) != 0 {
                fixjump(state, lexical_state, function_state, list, vtarget);
            } else {
                fixjump(state, lexical_state, function_state, list, dtarget);
            }
            list = next;
        }
    }
}
pub unsafe fn luak_patchlist(
    state: *mut State,
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    list: i32,
    target: i32,
) {
    unsafe {
        patchlistaux(
            state,
            lexical_state,
            function_state,
            list,
            target,
            NO_REG,
            target,
        );
    }
}
pub unsafe fn luak_patchtohere(state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState, list: i32) {
    unsafe {
        let hr: i32 = (*function_state).code_get_label();
        luak_patchlist(state, lexical_state, function_state, list, hr);
    }
}
pub unsafe fn savelineinfo(
    state: *mut State,
    _lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    prototype: *mut Prototype,
    line: i32,
) {
    unsafe {
        let mut linedif: i32 = line - (*function_state).functionstate_previous_line;
        let program_counter: i32 = (*function_state).functionstate_program_counter - 1;
        if linedif.abs() >= MAXLINEDIFF || {
            let prev_iwthabs = (*function_state).functionstate_i_width_absolute;
            (*function_state).functionstate_i_width_absolute += 1;
            prev_iwthabs as i32 >= MAXLINEDIFF
        } {
            (*prototype).prototype_absolutelineinfo.grow(
                state,
                (*function_state).functionstate_count_absolute_line_info as usize,
                if MAX_INT <= (!0usize) / size_of::<AbsoluteLineInfo>() {
                    MAX_INT
                } else {
                    (!0usize) / size_of::<AbsoluteLineInfo>()
                },
                c"lines".as_ptr(),
            );
            (*((*prototype).prototype_absolutelineinfo.vectort_pointer)
                .add((*function_state).functionstate_count_absolute_line_info as usize))
            .absolutelineinfo_program_counter = program_counter;
            let prev_count = (*function_state).functionstate_count_absolute_line_info;
            (*function_state).functionstate_count_absolute_line_info += 1;
            (*((*prototype).prototype_absolutelineinfo.vectort_pointer).add(prev_count as usize)).absolutelineinfo_line = line;
            linedif = -MAXLINEDIFF;
            (*function_state).functionstate_i_width_absolute = 1;
        }
        (*prototype).prototype_lineinfo.grow(
            state,
            program_counter as usize,
            if MAX_INT <= (!(0usize)) {
                MAX_INT
            } else {
                !(0usize)
            },
            c"opcodes".as_ptr(),
        );
        *((*prototype).prototype_lineinfo.vectort_pointer).add(program_counter as usize) = linedif as i8;
        (*function_state).functionstate_previous_line = line;
    }
}
pub unsafe fn removelastlineinfo(function_state: *mut FunctionState) {
    unsafe {
        let prototype: *mut Prototype = (*function_state).functionstate_prototype;
        let program_counter: i32 = (*function_state).functionstate_program_counter - 1;
        if *((*prototype).prototype_lineinfo.vectort_pointer).add(program_counter as usize) as i32 != -MAXLINEDIFF {
            (*function_state).functionstate_previous_line -=
                *((*prototype).prototype_lineinfo.vectort_pointer).add(program_counter as usize) as i32;
            (*function_state).functionstate_i_width_absolute -= 1;
        } else {
            (*function_state).functionstate_count_absolute_line_info -= 1;
            (*function_state).functionstate_count_absolute_line_info;
            (*function_state).functionstate_i_width_absolute = (MAXLINEDIFF + 1) as u8;
        };
    }
}
pub unsafe fn removelastinstruction(function_state: *mut FunctionState) {
    unsafe {
        removelastlineinfo(function_state);
        (*function_state).functionstate_program_counter -= 1;
        (*function_state).functionstate_program_counter;
    }
}
pub unsafe fn luak_code(state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState, i: u32) -> i32 {
    unsafe {
        let prototype: *mut Prototype = (*function_state).functionstate_prototype;
        (*prototype).prototype_code.grow(
            state,
            (*function_state).functionstate_program_counter as usize,
            if MAX_INT <= (!0usize) / size_of::<u32>() {
                MAX_INT
            } else {
                (!0usize) / size_of::<u32>()
            },
            c"opcodes".as_ptr(),
        );
        let prev_pc = (*function_state).functionstate_program_counter;
        (*function_state).functionstate_program_counter += 1;
        *((*prototype).prototype_code.vectort_pointer).add(prev_pc as usize) = i;
        savelineinfo(
            state,
            lexical_state,
            function_state,
            prototype,
            (*lexical_state).lexicalstate_lastline,
        );
        (*function_state).functionstate_program_counter - 1
    }
}
pub unsafe fn code_abck(
    state: *mut State,
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    o: u32,
    a: i32,
    b: i32,
    c: i32,
    k: i32,
) -> i32 {
    unsafe {
        luak_code(
            state,
            lexical_state,
            function_state,
            o | (a as u32) << POSITION_A | (b as u32) << POSITION_B | (c as u32) << POSITION_C | (k as u32) << POSITION_K,
        )
    }
}
pub unsafe fn luak_codeabx(
    state: *mut State,
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    o: u32,
    a: i32,
    bc: u32,
) -> i32 {
    unsafe {
        luak_code(
            state,
            lexical_state,
            function_state,
            o | (a as u32) << POSITION_A | bc << POSITION_K,
        )
    }
}
pub unsafe fn codeasbx(
    state: *mut State,
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    o: u32,
    a: i32,
    bc: i32,
) -> i32 {
    unsafe {
        let b: u32 = (bc + OFFSET_SBX) as u32;
        luak_code(
            state,
            lexical_state,
            function_state,
            o | (a as u32) << POSITION_A | b << POSITION_K,
        )
    }
}
pub unsafe fn codesj(
    state: *mut State,
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    o: u32,
    sj: i32,
    k: i32,
) -> i32 {
    unsafe {
        let j = (sj + OFFSET_SJ) as u32;
        luak_code(
            state,
            lexical_state,
            function_state,
            o | j << POSITION_A | (k as u32) << POSITION_K,
        )
    }
}
pub unsafe fn codeextraarg(state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState, a: i32) -> i32 {
    unsafe {
        luak_code(
            state,
            lexical_state,
            function_state,
            OPCODE_EXTRAARG | (a as u32) << POSITION_A,
        )
    }
}
pub unsafe fn code_constant(
    state: *mut State,
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    reg: i32,
    k: i32,
) -> i32 {
    unsafe {
        if k <= MAXARG_BX {
            luak_codeabx(
                state,
                lexical_state,
                function_state,
                OPCODE_LOADK,
                reg,
                k as u32,
            )
        } else {
            let p = luak_codeabx(state, lexical_state, function_state, OPCODE_LOADKX, reg, 0);
            codeextraarg(state, lexical_state, function_state, k);
            p
        }
    }
}
pub unsafe fn luak_checkstack(state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState, n: i32) {
    unsafe {
        let new_stack = (*function_state).functionstate_free_register as i32 + n;
        if new_stack > (*(*function_state).functionstate_prototype).prototype_maximumstacksize as i32 {
            if new_stack > MAXREGS {
                luax_syntaxerror(
                    state,
                    lexical_state,
                    c"function or expression needs too many registers".as_ptr(),
                );
            }
            (*(*function_state).functionstate_prototype).prototype_maximumstacksize = new_stack as u8;
        }
    }
}
pub unsafe fn luak_reserveregs(state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState, n: i32) {
    unsafe {
        luak_checkstack(state, lexical_state, function_state, n);
        (*function_state).functionstate_free_register = ((*function_state).functionstate_free_register as i32 + n) as u8;
    }
}
pub unsafe fn freereg(lexical_state: *mut LexicalState, function_state: *mut FunctionState, reg: i32) {
    unsafe {
        if reg >= luay_nvarstack(lexical_state, function_state) {
            (*function_state).functionstate_free_register -= 1;
        }
    }
}
pub unsafe fn freeregs(lexical_state: *mut LexicalState, function_state: *mut FunctionState, r1: i32, r2: i32) {
    unsafe {
        if r1 > r2 {
            freereg(lexical_state, function_state, r1);
            freereg(lexical_state, function_state, r2);
        } else {
            freereg(lexical_state, function_state, r2);
            freereg(lexical_state, function_state, r1);
        };
    }
}
pub unsafe fn freeexp(
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    expression_description: *mut ExpressionDescription,
) {
    unsafe {
        if (*expression_description).expressiondescription_expressionkind == ExpressionKind::Nonrelocatable {
            freereg(
                lexical_state,
                function_state,
                (*expression_description)
                    .expressiondescription_value
                    .value_info,
            );
        }
    }
}
pub unsafe fn freeexps(
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    e1: *mut ExpressionDescription,
    e2: *mut ExpressionDescription,
) {
    unsafe {
        let r1: i32 = if (*e1).expressiondescription_expressionkind == ExpressionKind::Nonrelocatable {
            (*e1).expressiondescription_value.value_info
        } else {
            -1
        };
        let r2: i32 = if (*e2).expressiondescription_expressionkind == ExpressionKind::Nonrelocatable {
            (*e2).expressiondescription_value.value_info
        } else {
            -1
        };
        freeregs(lexical_state, function_state, r1, r2);
    }
}
pub unsafe fn addk(
    state: *mut State,
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    key: *mut TValue,
    v: *mut TValue,
) -> i32 {
    unsafe {
        let mut value: TValue = TValue::new(TagVariant::NilNil);
        let prototype: *mut Prototype = (*function_state).functionstate_prototype;
        let mut index: TValue = TValue::new(TagVariant::NilNil);
        let index_tag = luah_get((*lexical_state).lexicalstate_table, key, &mut index);
        let mut count_constants: i32;
        if index_tag == TagVariant::NumericInteger {
            count_constants = index.as_integer().unwrap() as i32;
            if count_constants < (*function_state).functionstate_count_constants
                && (*((*prototype).prototype_constants.vectort_pointer).add(count_constants as usize)).get_tagvariant()
                    == (*v).get_tagvariant()
                && luav_equalobj(
                    null_mut(),
                    &*((*prototype).prototype_constants.vectort_pointer).add(count_constants as usize),
                    v,
                )
            {
                return count_constants;
            }
        }
        let mut oldsize = (*prototype).prototype_constants.get_size();
        count_constants = (*function_state).functionstate_count_constants;
        let io: *mut TValue = &mut value;
        (*io).set_integer(count_constants as i64);
        luah_set(state, (*lexical_state).lexicalstate_table, key, &mut value);
        (*prototype).prototype_constants.grow(
            state,
            count_constants as usize,
            if MAXARG_AX <= (!0usize) / size_of::<TValue>() {
                MAXARG_AX
            } else {
                (!0usize) / size_of::<TValue>()
            },
            c"constants".as_ptr(),
        );
        while oldsize < (*prototype).prototype_constants.get_size() {
            let prev_size = oldsize;
            oldsize += 1;
            (*((*prototype).prototype_constants.vectort_pointer).add(prev_size)).tvalue_set_tag_variant(TagVariant::NilNil);
        }
        let io1: *mut TValue =
            &mut *((*prototype).prototype_constants.vectort_pointer).add(count_constants as usize) as *mut TValue;
        let io2: *const TValue = v;
        (*io1).copy_from(&*io2);
        (*function_state).functionstate_count_constants += 1;
        (*function_state).functionstate_count_constants;
        if (*v).is_collectable()
            && (*prototype).get_marked() & BLACKBIT != 0
            && (*(*v).as_object().unwrap()).get_marked() & (WHITE0BIT | WHITE1BIT) != 0
        {
            Object::luac_barrier_(
                state,
                &mut *(prototype as *mut Object),
                &mut *(*v).as_object().unwrap(),
            );
        }
        count_constants
    }
}
pub unsafe fn string_constant(
    state: *mut State,
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    s: *mut TString,
) -> i32 {
    unsafe {
        let mut o: TValue = TValue::new(TagVariant::NilNil);
        let io: *mut TValue = &mut o;
        (*io).set_object(s as *mut Object, (*s).get_tagvariant());
        addk(state, lexical_state, function_state, &mut o, &mut o)
    }
}
pub unsafe fn luak_int_k(
    state: *mut State,
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    integer: i64,
) -> i32 {
    unsafe {
        let mut tvalue: TValue = TValue::new(TagVariant::NilNil);
        tvalue.set_integer(integer);
        addk(
            state,
            lexical_state,
            function_state,
            &mut tvalue,
            &mut tvalue,
        )
    }
}
pub unsafe fn luak_number_k(
    state: *mut State,
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    number: f64,
) -> i32 {
    unsafe {
        let mut tvalue: TValue = TValue::new(TagVariant::NilNil);
        let mut ik: i64 = 0;
        tvalue.set_number(number);
        if !F2I::Equal.convert_f64_i64(number, &mut ik) {
            addk(
                state,
                lexical_state,
                function_state,
                &mut tvalue,
                &mut tvalue,
            )
        } else {
            let nbm: i32 = NBM;
            let q: f64 = ldexp_(1.0f64, -nbm + 1);
            let k: f64 = if ik == 0 { q } else { number + number * q };
            let mut kv: TValue = TValue::new(TagVariant::NumericNumber);
            kv.set_number(k);
            addk(state, lexical_state, function_state, &mut kv, &mut tvalue)
        }
    }
}
pub unsafe fn bool_false(state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState) -> i32 {
    unsafe {
        let mut tvalue: TValue = TValue::new(TagVariant::BooleanFalse);
        addk(
            state,
            lexical_state,
            function_state,
            &mut tvalue,
            &mut tvalue,
        )
    }
}
pub unsafe fn bool_true(state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState) -> i32 {
    unsafe {
        let mut value: TValue = TValue::new(TagVariant::BooleanTrue);
        addk(state, lexical_state, function_state, &mut value, &mut value)
    }
}
pub unsafe fn nil_k(state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState) -> i32 {
    unsafe {
        let mut key: TValue = TValue::new(TagVariant::Table);
        let mut value: TValue = TValue::new(TagVariant::NilNil);
        let table: *mut Table = (*lexical_state).lexicalstate_table;
        key.set_object(table as *mut Object, TagVariant::Table);
        addk(state, lexical_state, function_state, &mut key, &mut value)
    }
}
pub unsafe fn code_constant_integer(
    state: *mut State,
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    reg: i32,
    i: i64,
) {
    unsafe {
        if fits_bx(i) {
            codeasbx(
                state,
                lexical_state,
                function_state,
                OPCODE_LOADI,
                reg,
                i as i32,
            );
        } else {
            code_constant(
                state,
                lexical_state,
                function_state,
                reg,
                luak_int_k(state, lexical_state, function_state, i),
            );
        };
    }
}
pub unsafe fn code_constant_number(
    state: *mut State,
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    reg: i32,
    number: f64,
) {
    unsafe {
        let mut fi: i64 = 0;
        if F2I::Equal.convert_f64_i64(number, &mut fi) && fits_bx(fi) {
            codeasbx(
                state,
                lexical_state,
                function_state,
                OPCODE_LOADF,
                reg,
                fi as i32,
            );
        } else {
            code_constant(
                state,
                lexical_state,
                function_state,
                reg,
                luak_number_k(state, lexical_state, function_state, number),
            );
        };
    }
}
pub unsafe fn luak_setreturns(
    state: *mut State,
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    expression_description: *mut ExpressionDescription,
    count_results: i32,
) {
    unsafe {
        let program_counter: *mut u32 = &mut *((*(*function_state).functionstate_prototype)
            .prototype_code
            .vectort_pointer)
            .add(
                (*expression_description)
                    .expressiondescription_value
                    .value_info as usize,
            ) as *mut u32;
        if (*expression_description).expressiondescription_expressionkind == ExpressionKind::Call {
            *program_counter =
                *program_counter & !(MASK_C << POSITION_C) | ((count_results + 1) as u32) << POSITION_C & MASK_C << POSITION_C;
        } else {
            *program_counter =
                *program_counter & !(MASK_C << POSITION_C) | ((count_results + 1) as u32) << POSITION_C & MASK_C << POSITION_C;
            *program_counter = *program_counter & !(MASK_A << POSITION_A)
                | ((*function_state).functionstate_free_register as u32) << POSITION_A & MASK_A << POSITION_A;
            luak_reserveregs(state, lexical_state, function_state, 1);
        };
    }
}
pub unsafe fn string_to_constant(
    state: *mut State,
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    expression_description: *mut ExpressionDescription,
) {
    unsafe {
        (*expression_description)
            .expressiondescription_value
            .value_info = string_constant(
            state,
            lexical_state,
            function_state,
            (*expression_description)
                .expressiondescription_value
                .value_tstring,
        );
        (*expression_description).expressiondescription_expressionkind = ExpressionKind::Constant;
    }
}
pub unsafe fn luak_setoneret(function_state: *mut FunctionState, expression_description: *mut ExpressionDescription) {
    unsafe {
        if (*expression_description).expressiondescription_expressionkind == ExpressionKind::Call {
            (*expression_description).expressiondescription_expressionkind = ExpressionKind::Nonrelocatable;
            (*expression_description)
                .expressiondescription_value
                .value_info = (*((*(*function_state).functionstate_prototype)
                .prototype_code
                .vectort_pointer)
                .add(
                    (*expression_description)
                        .expressiondescription_value
                        .value_info as usize,
                )
                >> POSITION_A
                & MASK_A) as i32;
        } else if (*expression_description).expressiondescription_expressionkind == ExpressionKind::VariableArguments {
            *((*(*function_state).functionstate_prototype)
                .prototype_code
                .vectort_pointer)
                .add(
                    (*expression_description)
                        .expressiondescription_value
                        .value_info as usize,
                ) = *((*(*function_state).functionstate_prototype)
                .prototype_code
                .vectort_pointer)
                .add(
                    (*expression_description)
                        .expressiondescription_value
                        .value_info as usize,
                )
                & !(MASK_C << POSITION_C)
                | 2_u32 << POSITION_C & MASK_C << POSITION_C;
            (*expression_description).expressiondescription_expressionkind = ExpressionKind::Relocatable;
        }
    }
}
pub unsafe fn luak_dischargevars(
    state: *mut State,
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    expression_description: *mut ExpressionDescription,
) {
    unsafe {
        match (*expression_description).expressiondescription_expressionkind {
            ExpressionKind::Constant2 => {
                ExpressionDescription::const2exp(
                    ExpressionDescription::const2val(lexical_state, function_state, expression_description),
                    expression_description,
                );
            }
            ExpressionKind::Local => {
                let temporary = (*expression_description)
                    .expressiondescription_value
                    .value_variable
                    .valueregister_registerindex as i32;
                (*expression_description)
                    .expressiondescription_value
                    .value_info = temporary;
                (*expression_description).expressiondescription_expressionkind = ExpressionKind::Nonrelocatable;
            }
            ExpressionKind::UpValue => {
                (*expression_description)
                    .expressiondescription_value
                    .value_info = code_abck(
                    state,
                    lexical_state,
                    function_state,
                    OPCODE_GET_UPVALUE,
                    0,
                    (*expression_description)
                        .expressiondescription_value
                        .value_info,
                    0,
                    0,
                );
                (*expression_description).expressiondescription_expressionkind = ExpressionKind::Relocatable;
            }
            ExpressionKind::IndexUpValue => {
                (*expression_description)
                    .expressiondescription_value
                    .value_info = code_abck(
                    state,
                    lexical_state,
                    function_state,
                    OPCODE_GET_TABLE_UPVALUE,
                    0,
                    (*expression_description)
                        .expressiondescription_value
                        .value_index
                        .valuereference_tag as i32,
                    (*expression_description)
                        .expressiondescription_value
                        .value_index
                        .valuereference_index as i32,
                    0,
                );
                (*expression_description).expressiondescription_expressionkind = ExpressionKind::Relocatable;
            }
            ExpressionKind::IndexInteger => {
                freereg(
                    lexical_state,
                    function_state,
                    (*expression_description)
                        .expressiondescription_value
                        .value_index
                        .valuereference_tag as i32,
                );
                (*expression_description)
                    .expressiondescription_value
                    .value_info = code_abck(
                    state,
                    lexical_state,
                    function_state,
                    OPCODE_INDEX_INTEGER,
                    0,
                    (*expression_description)
                        .expressiondescription_value
                        .value_index
                        .valuereference_tag as i32,
                    (*expression_description)
                        .expressiondescription_value
                        .value_index
                        .valuereference_index as i32,
                    0,
                );
                (*expression_description).expressiondescription_expressionkind = ExpressionKind::Relocatable;
            }
            ExpressionKind::Field => {
                freereg(
                    lexical_state,
                    function_state,
                    (*expression_description)
                        .expressiondescription_value
                        .value_index
                        .valuereference_tag as i32,
                );
                (*expression_description)
                    .expressiondescription_value
                    .value_info = code_abck(
                    state,
                    lexical_state,
                    function_state,
                    OPCODE_GET_FIELD,
                    0,
                    (*expression_description)
                        .expressiondescription_value
                        .value_index
                        .valuereference_tag as i32,
                    (*expression_description)
                        .expressiondescription_value
                        .value_index
                        .valuereference_index as i32,
                    0,
                );
                (*expression_description).expressiondescription_expressionkind = ExpressionKind::Relocatable;
            }
            ExpressionKind::VarargVariable => {
                // vararg parameter used as value: needs a real table
                (*(*function_state).functionstate_prototype).prototype_needsvarargtable = true;
                (*expression_description).expressiondescription_expressionkind = ExpressionKind::Local;
                // fallthrough to Local
                let temporary = (*expression_description)
                    .expressiondescription_value
                    .value_variable
                    .valueregister_registerindex as i32;
                (*expression_description)
                    .expressiondescription_value
                    .value_info = temporary;
                (*expression_description).expressiondescription_expressionkind = ExpressionKind::Nonrelocatable;
            }
            ExpressionKind::Indexed => {
                freeregs(
                    lexical_state,
                    function_state,
                    (*expression_description)
                        .expressiondescription_value
                        .value_index
                        .valuereference_tag as i32,
                    (*expression_description)
                        .expressiondescription_value
                        .value_index
                        .valuereference_index as i32,
                );
                (*expression_description)
                    .expressiondescription_value
                    .value_info = code_abck(
                    state,
                    lexical_state,
                    function_state,
                    OPCODE_GET_TABLE,
                    0,
                    (*expression_description)
                        .expressiondescription_value
                        .value_index
                        .valuereference_tag as i32,
                    (*expression_description)
                        .expressiondescription_value
                        .value_index
                        .valuereference_index as i32,
                    0,
                );
                (*expression_description).expressiondescription_expressionkind = ExpressionKind::Relocatable;
            }
            ExpressionKind::VarargIndex => {
                freeregs(
                    lexical_state,
                    function_state,
                    (*expression_description)
                        .expressiondescription_value
                        .value_index
                        .valuereference_tag as i32,
                    (*expression_description)
                        .expressiondescription_value
                        .value_index
                        .valuereference_index as i32,
                );
                (*expression_description)
                    .expressiondescription_value
                    .value_info = code_abck(
                    state,
                    lexical_state,
                    function_state,
                    OPCODE_GETVARG,
                    0,
                    (*expression_description)
                        .expressiondescription_value
                        .value_index
                        .valuereference_tag as i32,
                    (*expression_description)
                        .expressiondescription_value
                        .value_index
                        .valuereference_index as i32,
                    0,
                );
                (*expression_description).expressiondescription_expressionkind = ExpressionKind::Relocatable;
            }
            ExpressionKind::Call => {
                luak_setoneret(function_state, expression_description);
            }
            ExpressionKind::VariableArguments => {
                luak_setoneret(function_state, expression_description);
            }
            _ => {}
        };
    }
}
pub unsafe fn discharge2reg(
    state: *mut State,
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    expression_description: *mut ExpressionDescription,
    register: i32,
) {
    unsafe {
        luak_dischargevars(state, lexical_state, function_state, expression_description);
        match (*expression_description).expressiondescription_expressionkind {
            ExpressionKind::Nil => {
                code_constant_nil(state, lexical_state, function_state, register, 1);
            }
            ExpressionKind::False => {
                code_abck(
                    state,
                    lexical_state,
                    function_state,
                    OPCODE_LOAD_FALSE,
                    register,
                    0,
                    0,
                    0,
                );
            }
            ExpressionKind::True => {
                code_abck(
                    state,
                    lexical_state,
                    function_state,
                    OPCODE_LOAD_TRUE,
                    register,
                    0,
                    0,
                    0,
                );
            }
            ExpressionKind::ConstantString => {
                string_to_constant(state, lexical_state, function_state, expression_description);
                code_constant(
                    state,
                    lexical_state,
                    function_state,
                    register,
                    (*expression_description)
                        .expressiondescription_value
                        .value_info,
                );
            }
            ExpressionKind::Constant => {
                code_constant(
                    state,
                    lexical_state,
                    function_state,
                    register,
                    (*expression_description)
                        .expressiondescription_value
                        .value_info,
                );
            }
            ExpressionKind::ConstantNumber => {
                code_constant_number(
                    state,
                    lexical_state,
                    function_state,
                    register,
                    (*expression_description)
                        .expressiondescription_value
                        .value_number,
                );
            }
            ExpressionKind::ConstantInteger => {
                code_constant_integer(
                    state,
                    lexical_state,
                    function_state,
                    register,
                    (*expression_description)
                        .expressiondescription_value
                        .value_integer,
                );
            }
            ExpressionKind::Relocatable => {
                let program_counter = &mut *((*(*function_state).functionstate_prototype)
                    .prototype_code
                    .vectort_pointer)
                    .add(
                        (*expression_description)
                            .expressiondescription_value
                            .value_info as usize,
                    );
                *program_counter =
                    *program_counter & !(MASK_A << POSITION_A) | (register as u32) << POSITION_A & MASK_A << POSITION_A;
            }
            ExpressionKind::Nonrelocatable => {
                if register
                    != (*expression_description)
                        .expressiondescription_value
                        .value_info
                {
                    code_abck(
                        state,
                        lexical_state,
                        function_state,
                        OPCODE_MOVE,
                        register,
                        (*expression_description)
                            .expressiondescription_value
                            .value_info,
                        0,
                        0,
                    );
                }
            }
            _ => return,
        }
        (*expression_description)
            .expressiondescription_value
            .value_info = register;
        (*expression_description).expressiondescription_expressionkind = ExpressionKind::Nonrelocatable;
    }
}
pub unsafe fn discharge2anyreg(
    state: *mut State,
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    expression_description: *mut ExpressionDescription,
) {
    unsafe {
        if (*expression_description).expressiondescription_expressionkind != ExpressionKind::Nonrelocatable {
            luak_reserveregs(state, lexical_state, function_state, 1);
            discharge2reg(
                state,
                lexical_state,
                function_state,
                expression_description,
                (*function_state).functionstate_free_register as i32 - 1,
            );
        }
    }
}
pub unsafe fn code_loadbool(
    state: *mut State,
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    a: i32,
    op: u32,
) -> i32 {
    unsafe {
        (*function_state).code_get_label();
        code_abck(state, lexical_state, function_state, op, a, 0, 0, 0)
    }
}
pub unsafe fn need_value(function_state: *mut FunctionState, mut list: i32) -> i32 {
    unsafe {
        while list != -1 {
            let i: u32 = *code_get_jump_control(function_state, list);
            if (i & MASK_OP) != OPCODE_TESTSET {
                return 1;
            }
            list = code_get_jump(function_state, list);
        }
        0
    }
}
pub unsafe fn exp2reg(
    state: *mut State,
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    expression_description: *mut ExpressionDescription,
    reg: i32,
) {
    unsafe {
        discharge2reg(
            state,
            lexical_state,
            function_state,
            expression_description,
            reg,
        );
        if (*expression_description).expressiondescription_expressionkind == ExpressionKind::Jump {
            luak_concat(
                state,
                lexical_state,
                function_state,
                &mut (*expression_description).expressiondescription_t,
                (*expression_description)
                    .expressiondescription_value
                    .value_info,
            );
        }
        if (*expression_description).expressiondescription_t != (*expression_description).expressiondescription_f {
            let mut p_f: i32 = -1;
            let mut p_t: i32 = -1;
            if need_value(
                function_state,
                (*expression_description).expressiondescription_t,
            ) != 0
                || need_value(
                    function_state,
                    (*expression_description).expressiondescription_f,
                ) != 0
            {
                let fj: i32 = if (*expression_description).expressiondescription_expressionkind == ExpressionKind::Jump {
                    -1
                } else {
                    luak_jump(state, lexical_state, function_state)
                };
                p_f = code_loadbool(state, lexical_state, function_state, reg, OPCODE_LFALSESKIP);
                p_t = code_loadbool(state, lexical_state, function_state, reg, OPCODE_LOAD_TRUE);
                luak_patchtohere(state, lexical_state, function_state, fj);
            }
            let final_label: i32 = (*function_state).code_get_label();
            patchlistaux(
                state,
                lexical_state,
                function_state,
                (*expression_description).expressiondescription_f,
                final_label,
                reg,
                p_f,
            );
            patchlistaux(
                state,
                lexical_state,
                function_state,
                (*expression_description).expressiondescription_t,
                final_label,
                reg,
                p_t,
            );
        }
        (*expression_description).expressiondescription_t = -1;
        (*expression_description).expressiondescription_f = (*expression_description).expressiondescription_t;
        (*expression_description)
            .expressiondescription_value
            .value_info = reg;
        (*expression_description).expressiondescription_expressionkind = ExpressionKind::Nonrelocatable;
    }
}
pub unsafe fn luak_exp2nextreg(
    state: *mut State,
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    expression_description: *mut ExpressionDescription,
) {
    unsafe {
        luak_dischargevars(state, lexical_state, function_state, expression_description);
        freeexp(lexical_state, function_state, expression_description);
        luak_reserveregs(state, lexical_state, function_state, 1);
        exp2reg(
            state,
            lexical_state,
            function_state,
            expression_description,
            (*function_state).functionstate_free_register as i32 - 1,
        );
    }
}
pub unsafe fn luak_exp2anyreg(
    state: *mut State,
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    expression_description: *mut ExpressionDescription,
) -> i64 {
    unsafe {
        luak_dischargevars(state, lexical_state, function_state, expression_description);
        if (*expression_description).expressiondescription_expressionkind == ExpressionKind::Nonrelocatable {
            if (*expression_description).expressiondescription_t == (*expression_description).expressiondescription_f {
                return (*expression_description)
                    .expressiondescription_value
                    .value_info as i64;
            }
            if (*expression_description)
                .expressiondescription_value
                .value_info
                >= luay_nvarstack(lexical_state, function_state)
            {
                exp2reg(
                    state,
                    lexical_state,
                    function_state,
                    expression_description,
                    (*expression_description)
                        .expressiondescription_value
                        .value_info,
                );
                return (*expression_description)
                    .expressiondescription_value
                    .value_info as i64;
            }
        }
        luak_exp2nextreg(state, lexical_state, function_state, expression_description);
        (*expression_description)
            .expressiondescription_value
            .value_info as i64
    }
}
pub unsafe fn luak_exp2anyregup(
    state: *mut State,
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    expression_description: *mut ExpressionDescription,
) {
    unsafe {
        if ((*expression_description).expressiondescription_expressionkind != ExpressionKind::UpValue
            && (*expression_description).expressiondescription_expressionkind != ExpressionKind::VarargVariable)
            || (*expression_description).expressiondescription_t != (*expression_description).expressiondescription_f
        {
            luak_exp2anyreg(state, lexical_state, function_state, expression_description);
        }
    }
}
pub unsafe fn code_expression_to_value(
    state: *mut State,
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    expression_description: *mut ExpressionDescription,
) {
    unsafe {
        if (*expression_description).expressiondescription_expressionkind == ExpressionKind::Jump
            || (*expression_description).expressiondescription_t != (*expression_description).expressiondescription_f
        {
            luak_exp2anyreg(state, lexical_state, function_state, expression_description);
        } else {
            luak_dischargevars(state, lexical_state, function_state, expression_description);
        };
    }
}
pub unsafe fn code_expression_to_constant(
    state: *mut State,
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    expression_description: *mut ExpressionDescription,
) -> i32 {
    unsafe {
        if (*expression_description).expressiondescription_t == (*expression_description).expressiondescription_f {
            let info: i32;
            match (*expression_description).expressiondescription_expressionkind {
                ExpressionKind::True => {
                    info = bool_true(state, lexical_state, function_state);
                    if info <= MAXARG_C {
                        (*expression_description).expressiondescription_expressionkind = ExpressionKind::Constant;
                        (*expression_description)
                            .expressiondescription_value
                            .value_info = info;
                        1
                    } else {
                        0
                    }
                }
                ExpressionKind::False => {
                    info = bool_false(state, lexical_state, function_state);
                    if info <= MAXARG_C {
                        (*expression_description).expressiondescription_expressionkind = ExpressionKind::Constant;
                        (*expression_description)
                            .expressiondescription_value
                            .value_info = info;
                        1
                    } else {
                        0
                    }
                }
                ExpressionKind::Nil => {
                    info = nil_k(state, lexical_state, function_state);
                    if info <= MAXARG_C {
                        (*expression_description).expressiondescription_expressionkind = ExpressionKind::Constant;
                        (*expression_description)
                            .expressiondescription_value
                            .value_info = info;
                        1
                    } else {
                        0
                    }
                }
                ExpressionKind::ConstantInteger => {
                    info = luak_int_k(
                        state,
                        lexical_state,
                        function_state,
                        (*expression_description)
                            .expressiondescription_value
                            .value_integer,
                    );
                    if info <= MAXARG_C {
                        (*expression_description).expressiondescription_expressionkind = ExpressionKind::Constant;
                        (*expression_description)
                            .expressiondescription_value
                            .value_info = info;
                        1
                    } else {
                        0
                    }
                }
                ExpressionKind::ConstantNumber => {
                    info = luak_number_k(
                        state,
                        lexical_state,
                        function_state,
                        (*expression_description)
                            .expressiondescription_value
                            .value_number,
                    );
                    if info <= MAXARG_C {
                        (*expression_description).expressiondescription_expressionkind = ExpressionKind::Constant;
                        (*expression_description)
                            .expressiondescription_value
                            .value_info = info;
                        1
                    } else {
                        0
                    }
                }
                ExpressionKind::ConstantString => {
                    info = string_constant(
                        state,
                        lexical_state,
                        function_state,
                        (*expression_description)
                            .expressiondescription_value
                            .value_tstring,
                    );
                    if info <= MAXARG_C {
                        (*expression_description).expressiondescription_expressionkind = ExpressionKind::Constant;
                        (*expression_description)
                            .expressiondescription_value
                            .value_info = info;
                        1
                    } else {
                        0
                    }
                }
                ExpressionKind::Constant => {
                    info = (*expression_description)
                        .expressiondescription_value
                        .value_info;
                    if info <= MAXARG_C {
                        (*expression_description).expressiondescription_expressionkind = ExpressionKind::Constant;
                        (*expression_description)
                            .expressiondescription_value
                            .value_info = info;
                        1
                    } else {
                        0
                    }
                }
                _ => 0,
            }
        } else {
            0
        }
    }
}
pub unsafe fn exp2rk(
    state: *mut State,
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    expression_description: *mut ExpressionDescription,
) -> i32 {
    unsafe {
        if code_expression_to_constant(state, lexical_state, function_state, expression_description) != 0 {
            1
        } else {
            luak_exp2anyreg(state, lexical_state, function_state, expression_description);
            0
        }
    }
}
pub unsafe fn codeabrk(
    state: *mut State,
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    o: u32,
    a: i32,
    b: i32,
    ec: *mut ExpressionDescription,
) {
    unsafe {
        let k: i32 = exp2rk(state, lexical_state, function_state, ec);
        code_abck(
            state,
            lexical_state,
            function_state,
            o,
            a,
            b,
            (*ec).expressiondescription_value.value_info,
            k,
        );
    }
}
pub unsafe fn luak_storevar(
    state: *mut State,
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    var: *mut ExpressionDescription,
    ex: *mut ExpressionDescription,
) {
    unsafe {
        match (*var).expressiondescription_expressionkind {
            ExpressionKind::Local => {
                freeexp(lexical_state, function_state, ex);
                exp2reg(
                    state,
                    lexical_state,
                    function_state,
                    ex,
                    (*var)
                        .expressiondescription_value
                        .value_variable
                        .valueregister_registerindex as i32,
                );
                return;
            }
            ExpressionKind::UpValue => {
                let e = luak_exp2anyreg(state, lexical_state, function_state, ex);
                code_abck(
                    state,
                    lexical_state,
                    function_state,
                    OPCODE_SETUPVAL,
                    e as i32,
                    (*var).expressiondescription_value.value_info,
                    0,
                    0,
                );
            }
            ExpressionKind::IndexUpValue => {
                codeabrk(
                    state,
                    lexical_state,
                    function_state,
                    OPCODE_SETTABUP,
                    (*var)
                        .expressiondescription_value
                        .value_index
                        .valuereference_tag as i32,
                    (*var)
                        .expressiondescription_value
                        .value_index
                        .valuereference_index as i32,
                    ex,
                );
            }
            ExpressionKind::IndexInteger => {
                codeabrk(
                    state,
                    lexical_state,
                    function_state,
                    OPCODE_SETI,
                    (*var)
                        .expressiondescription_value
                        .value_index
                        .valuereference_tag as i32,
                    (*var)
                        .expressiondescription_value
                        .value_index
                        .valuereference_index as i32,
                    ex,
                );
            }
            ExpressionKind::Field => {
                codeabrk(
                    state,
                    lexical_state,
                    function_state,
                    OPCODE_SETFIELD,
                    (*var)
                        .expressiondescription_value
                        .value_index
                        .valuereference_tag as i32,
                    (*var)
                        .expressiondescription_value
                        .value_index
                        .valuereference_index as i32,
                    ex,
                );
            }
            ExpressionKind::VarargIndex => {
                // assignment to vararg index: needs a real table, then fall through to Indexed
                (*(*function_state).functionstate_prototype).prototype_needsvarargtable = true;
                codeabrk(
                    state,
                    lexical_state,
                    function_state,
                    OPCODE_SETTABLE,
                    (*var)
                        .expressiondescription_value
                        .value_index
                        .valuereference_tag as i32,
                    (*var)
                        .expressiondescription_value
                        .value_index
                        .valuereference_index as i32,
                    ex,
                );
            }
            ExpressionKind::Indexed => {
                codeabrk(
                    state,
                    lexical_state,
                    function_state,
                    OPCODE_SETTABLE,
                    (*var)
                        .expressiondescription_value
                        .value_index
                        .valuereference_tag as i32,
                    (*var)
                        .expressiondescription_value
                        .value_index
                        .valuereference_index as i32,
                    ex,
                );
            }
            _ => {}
        }
        freeexp(lexical_state, function_state, ex);
    }
}
pub unsafe fn luak_self(
    state: *mut State,
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    expression_description: *mut ExpressionDescription,
    key: *mut ExpressionDescription,
) {
    unsafe {
        luak_exp2anyreg(state, lexical_state, function_state, expression_description);
        let ereg: i32 = (*expression_description)
            .expressiondescription_value
            .value_info;
        freeexp(lexical_state, function_state, expression_description);
        let base: i32 = (*function_state).functionstate_free_register as i32;
        (*expression_description)
            .expressiondescription_value
            .value_info = base;
        (*expression_description).expressiondescription_expressionkind = ExpressionKind::Nonrelocatable;
        luak_reserveregs(state, lexical_state, function_state, 2);
        // In Lua 5.5, SELF always reads from K table, so we can only use SELF
        // when the key fits in K. Otherwise, fall back to MOVE + GETTABLE.
        if code_expression_to_constant(state, lexical_state, function_state, key) != 0 {
            code_abck(
                state,
                lexical_state,
                function_state,
                OPCODE_SELF,
                base,
                ereg,
                (*key).expressiondescription_value.value_info,
                0,
            );
        } else {
            luak_exp2anyreg(state, lexical_state, function_state, key);
            code_abck(
                state,
                lexical_state,
                function_state,
                OPCODE_MOVE,
                base + 1,
                ereg,
                0,
                0,
            );
            code_abck(
                state,
                lexical_state,
                function_state,
                OPCODE_GET_TABLE,
                base,
                ereg,
                (*key).expressiondescription_value.value_info,
                0,
            );
        }
        freeexp(lexical_state, function_state, key);
    }
}
pub unsafe fn negatecondition(function_state: *mut FunctionState, expression_description: *mut ExpressionDescription) {
    unsafe {
        let program_counter: *mut u32 = code_get_jump_control(
            function_state,
            (*expression_description)
                .expressiondescription_value
                .value_info,
        );
        *program_counter = *program_counter & !(MASK_K << POSITION_K)
            | (((*program_counter >> POSITION_K & MASK_K) as i32 ^ 1) as u32) << POSITION_K & MASK_K << POSITION_K;
    }
}
pub unsafe fn jumponcond(
    state: *mut State,
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    expression_description: *mut ExpressionDescription,
    cond_0: i32,
) -> i32 {
    unsafe {
        if (*expression_description).expressiondescription_expressionkind == ExpressionKind::Relocatable {
            let ie: u32 = *((*(*function_state).functionstate_prototype)
                .prototype_code
                .vectort_pointer)
                .add(
                    (*expression_description)
                        .expressiondescription_value
                        .value_info as usize,
                );
            if (ie & MASK_OP) == OPCODE_NOT {
                removelastinstruction(function_state);
                return condjump(
                    state,
                    lexical_state,
                    function_state,
                    OPCODE_TEST,
                    (ie >> POSITION_B & MASK_A) as i32,
                    0,
                    0,
                    (cond_0 == 0) as i32,
                );
            }
        }
        discharge2anyreg(state, lexical_state, function_state, expression_description);
        freeexp(lexical_state, function_state, expression_description);
        condjump(
            state,
            lexical_state,
            function_state,
            OPCODE_TESTSET,
            NO_REG,
            (*expression_description)
                .expressiondescription_value
                .value_info,
            0,
            cond_0,
        )
    }
}
pub unsafe fn luak_goiftrue(
    state: *mut State,
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    expression_description: *mut ExpressionDescription,
) {
    unsafe {
        let program_counter: i32;
        luak_dischargevars(state, lexical_state, function_state, expression_description);
        match (*expression_description).expressiondescription_expressionkind {
            ExpressionKind::Jump => {
                negatecondition(function_state, expression_description);
                program_counter = (*expression_description)
                    .expressiondescription_value
                    .value_info;
            }
            ExpressionKind::True
            | ExpressionKind::Constant
            | ExpressionKind::ConstantNumber
            | ExpressionKind::ConstantInteger
            | ExpressionKind::ConstantString => {
                program_counter = -1;
            }
            _ => {
                program_counter = jumponcond(
                    state,
                    lexical_state,
                    function_state,
                    expression_description,
                    0,
                );
            }
        }
        luak_concat(
            state,
            lexical_state,
            function_state,
            &mut (*expression_description).expressiondescription_f,
            program_counter,
        );
        luak_patchtohere(
            state,
            lexical_state,
            function_state,
            (*expression_description).expressiondescription_t,
        );
        (*expression_description).expressiondescription_t = -1;
    }
}
pub unsafe fn luak_goiffalse(
    state: *mut State,
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    expression_description: *mut ExpressionDescription,
) {
    unsafe {
        let program_counter: i32;
        luak_dischargevars(state, lexical_state, function_state, expression_description);
        match (*expression_description).expressiondescription_expressionkind {
            ExpressionKind::Jump => {
                program_counter = (*expression_description)
                    .expressiondescription_value
                    .value_info;
            }
            ExpressionKind::Nil | ExpressionKind::False => {
                program_counter = -1;
            }
            _ => {
                program_counter = jumponcond(
                    state,
                    lexical_state,
                    function_state,
                    expression_description,
                    1,
                );
            }
        }
        luak_concat(
            state,
            lexical_state,
            function_state,
            &mut (*expression_description).expressiondescription_t,
            program_counter,
        );
        luak_patchtohere(
            state,
            lexical_state,
            function_state,
            (*expression_description).expressiondescription_f,
        );
        (*expression_description).expressiondescription_f = -1;
    }
}
pub unsafe fn codenot(
    state: *mut State,
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    expression_description: *mut ExpressionDescription,
) {
    unsafe {
        match (*expression_description).expressiondescription_expressionkind {
            ExpressionKind::Nil | ExpressionKind::False => {
                (*expression_description).expressiondescription_expressionkind = ExpressionKind::True;
            }
            ExpressionKind::Constant
            | ExpressionKind::ConstantNumber
            | ExpressionKind::ConstantInteger
            | ExpressionKind::ConstantString
            | ExpressionKind::True => {
                (*expression_description).expressiondescription_expressionkind = ExpressionKind::False;
            }
            ExpressionKind::Jump => {
                negatecondition(function_state, expression_description);
            }
            ExpressionKind::Relocatable | ExpressionKind::Nonrelocatable => {
                discharge2anyreg(state, lexical_state, function_state, expression_description);
                freeexp(lexical_state, function_state, expression_description);
                (*expression_description)
                    .expressiondescription_value
                    .value_info = code_abck(
                    state,
                    lexical_state,
                    function_state,
                    OPCODE_NOT,
                    0,
                    (*expression_description)
                        .expressiondescription_value
                        .value_info,
                    0,
                    0,
                );
                (*expression_description).expressiondescription_expressionkind = ExpressionKind::Relocatable;
            }
            _ => {}
        }
        std::mem::swap(
            &mut (*expression_description).expressiondescription_f,
            &mut (*expression_description).expressiondescription_t,
        );
        removevalues(
            function_state,
            (*expression_description).expressiondescription_f,
        );
        removevalues(
            function_state,
            (*expression_description).expressiondescription_t,
        );
    }
}
pub unsafe fn is_k_string(function_state: *mut FunctionState, expression_description: *mut ExpressionDescription) -> bool {
    unsafe {
        (*expression_description).expressiondescription_expressionkind == ExpressionKind::Constant
            && ((*expression_description).expressiondescription_t == (*expression_description).expressiondescription_f)
            && (*expression_description)
                .expressiondescription_value
                .value_info
                <= MAXARG_C
            && (*((*(*function_state).functionstate_prototype)
                .prototype_constants
                .vectort_pointer)
                .add(
                    (*expression_description)
                        .expressiondescription_value
                        .value_info as usize,
                ))
            .get_tagvariant()
                == TagVariant::StringShort
    }
}
pub unsafe fn constfolding(
    state: *mut State,
    _lexical_state: *mut LexicalState,
    _function_state: *mut FunctionState,
    op: i32,
    e1: *mut ExpressionDescription,
    e2: *const ExpressionDescription,
) -> i32 {
    unsafe {
        let mut v1: TValue = TValue::new(TagVariant::NilNil);
        let mut v2: TValue = TValue::new(TagVariant::NilNil);
        let mut res: TValue = TValue::new(TagVariant::NilNil);
        if !ExpressionDescription::tonumeral(e1, &mut v1)
            || !ExpressionDescription::tonumeral(e2, &mut v2)
            || validop(op, &mut v1, &mut v2) == 0
        {
            return 0;
        }
        luao_rawarith(state, op, &v1, &v2, &mut res);
        if res.get_tagvariant() == TagVariant::NumericInteger {
            (*e1).expressiondescription_expressionkind = ExpressionKind::ConstantInteger;
            (*e1).expressiondescription_value.value_integer = res.as_integer().unwrap();
        } else {
            let n: f64 = res.as_number().unwrap();
            if !(n == n) || n == 0.0 {
                return 0;
            }
            (*e1).expressiondescription_expressionkind = ExpressionKind::ConstantNumber;
            (*e1).expressiondescription_value.value_number = n;
        }
        1
    }
}
pub unsafe fn code_unary_expression_value(
    state: *mut State,
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    op: u32,
    expression_description: *mut ExpressionDescription,
    line: i32,
) {
    unsafe {
        let register = luak_exp2anyreg(state, lexical_state, function_state, expression_description);
        freeexp(lexical_state, function_state, expression_description);
        (*expression_description)
            .expressiondescription_value
            .value_info = code_abck(
            state,
            lexical_state,
            function_state,
            op,
            0,
            register as i32,
            0,
            0,
        );
        (*expression_description).expressiondescription_expressionkind = ExpressionKind::Relocatable;
        luak_fixline(state, lexical_state, function_state, line);
    }
}
pub unsafe fn finishbinexpval(
    state: *mut State,
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    e1: *mut ExpressionDescription,
    e2: *mut ExpressionDescription,
    op: u32,
    v2: i32,
    flip: i32,
    line: i32,
    mmop: u32,
    event: u32,
) {
    unsafe {
        let v1 = luak_exp2anyreg(state, lexical_state, function_state, e1);
        let program_counter: i32 = code_abck(
            state,
            lexical_state,
            function_state,
            op,
            0,
            v1 as i32,
            v2,
            0,
        );
        freeexps(lexical_state, function_state, e1, e2);
        (*e1).expressiondescription_value.value_info = program_counter;
        (*e1).expressiondescription_expressionkind = ExpressionKind::Relocatable;
        luak_fixline(state, lexical_state, function_state, line);
        code_abck(
            state,
            lexical_state,
            function_state,
            mmop,
            v1 as i32,
            v2,
            event as i32,
            flip,
        );
        luak_fixline(state, lexical_state, function_state, line);
    }
}
pub unsafe fn codebinexpval(
    state: *mut State,
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    binary: OperatorBinary,
    e1: *mut ExpressionDescription,
    e2: *mut ExpressionDescription,
    line: i32,
) {
    unsafe {
        let op = binopr2op(binary, OperatorBinary::Add, OPCODE_ADD);
        let v2 = luak_exp2anyreg(state, lexical_state, function_state, e2);
        finishbinexpval(
            state,
            lexical_state,
            function_state,
            e1,
            e2,
            op,
            v2 as i32,
            0,
            line,
            OPCODE_MMBIN,
            binopr2tm(binary),
        );
    }
}
pub unsafe fn codebini(
    state: *mut State,
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    op: u32,
    e1: *mut ExpressionDescription,
    e2: *mut ExpressionDescription,
    flip: i32,
    line: i32,
    event: u32,
) {
    unsafe {
        let v2: i32 = (*e2).expressiondescription_value.value_integer as i32 + OFFSET_SC;
        finishbinexpval(
            state,
            lexical_state,
            function_state,
            e1,
            e2,
            op,
            v2,
            flip,
            line,
            OPCODE_MMBINI,
            event,
        );
    }
}
pub unsafe fn codebink(
    state: *mut State,
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    opr: OperatorBinary,
    e1: *mut ExpressionDescription,
    e2: *mut ExpressionDescription,
    flip: i32,
    line: i32,
) {
    unsafe {
        let event: u32 = binopr2tm(opr);
        let v2: i32 = (*e2).expressiondescription_value.value_info;
        let op: u32 = binopr2op(opr, OperatorBinary::Add, OPCODE_ADDK);
        finishbinexpval(
            state,
            lexical_state,
            function_state,
            e1,
            e2,
            op,
            v2,
            flip,
            line,
            OPCODE_MMBINK,
            event,
        );
    }
}
pub unsafe fn finishbinexpneg(
    state: *mut State,
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    e1: *mut ExpressionDescription,
    e2: *mut ExpressionDescription,
    op: u32,
    line: i32,
    event: u32,
) -> i32 {
    unsafe {
        if !ExpressionDescription::is_k_int(e2) {
            0
        } else {
            let i2: i64 = (*e2).expressiondescription_value.value_integer;
            if !(fits_c(i2) && fits_c(-i2)) {
                0
            } else {
                let v2: i32 = i2 as i32;
                finishbinexpval(
                    state,
                    lexical_state,
                    function_state,
                    e1,
                    e2,
                    op,
                    -v2 + OFFSET_SC,
                    0,
                    line,
                    OPCODE_MMBINI,
                    event,
                );
                *((*(*function_state).functionstate_prototype)
                    .prototype_code
                    .vectort_pointer)
                    .add(((*function_state).functionstate_program_counter - 1) as usize) = *((*(*function_state)
                    .functionstate_prototype)
                    .prototype_code
                    .vectort_pointer)
                    .add(((*function_state).functionstate_program_counter - 1) as usize)
                    & !(MASK_B << POSITION_B)
                    | ((v2 + OFFSET_SC) as u32) << POSITION_B & MASK_B << POSITION_B;
                1
            }
        }
    }
}
pub unsafe fn codebinnok(
    state: *mut State,
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    opr: OperatorBinary,
    e1: *mut ExpressionDescription,
    e2: *mut ExpressionDescription,
    flip: i32,
    line: i32,
) {
    unsafe {
        if flip != 0 {
            ExpressionDescription::swapexps(e1, e2);
        }
        codebinexpval(state, lexical_state, function_state, opr, e1, e2, line);
    }
}
pub unsafe fn codearith(
    state: *mut State,
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    opr: OperatorBinary,
    e1: *mut ExpressionDescription,
    e2: *mut ExpressionDescription,
    flip: i32,
    line: i32,
) {
    unsafe {
        if ExpressionDescription::tonumeral(e2, null_mut())
            && code_expression_to_constant(state, lexical_state, function_state, e2) != 0
        {
            codebink(
                state,
                lexical_state,
                function_state,
                opr,
                e1,
                e2,
                flip,
                line,
            );
        } else {
            codebinnok(
                state,
                lexical_state,
                function_state,
                opr,
                e1,
                e2,
                flip,
                line,
            );
        };
    }
}
pub unsafe fn codebitwise(
    state: *mut State,
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    opr: OperatorBinary,
    e1: *mut ExpressionDescription,
    e2: *mut ExpressionDescription,
    line: i32,
) {
    unsafe {
        let mut flip: i32 = 0;
        if (*e1).expressiondescription_expressionkind == ExpressionKind::ConstantInteger {
            ExpressionDescription::swapexps(e1, e2);
            flip = 1;
        }
        if (*e2).expressiondescription_expressionkind == ExpressionKind::ConstantInteger
            && code_expression_to_constant(state, lexical_state, function_state, e2) != 0
        {
            codebink(
                state,
                lexical_state,
                function_state,
                opr,
                e1,
                e2,
                flip,
                line,
            );
        } else {
            codebinnok(
                state,
                lexical_state,
                function_state,
                opr,
                e1,
                e2,
                flip,
                line,
            );
        };
    }
}
pub unsafe fn codeorder(
    state: *mut State,
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    opr: OperatorBinary,
    e1: *mut ExpressionDescription,
    e2: *mut ExpressionDescription,
) {
    unsafe {
        let mut im: i64 = 0;
        let mut is_float = false;
        let r1: i64;
        let r2: i64;
        let op: u32;
        if ExpressionDescription::is_sc_number(e2, &mut im, &mut is_float) != 0 {
            r1 = luak_exp2anyreg(state, lexical_state, function_state, e1);
            r2 = im;
            op = binopr2op(opr, OperatorBinary::Less, OPCODE_LTI);
        } else if ExpressionDescription::is_sc_number(e1, &mut im, &mut is_float) != 0 {
            r1 = luak_exp2anyreg(state, lexical_state, function_state, e2);
            r2 = im;
            op = binopr2op(opr, OperatorBinary::Less, OPCODE_GTI);
        } else {
            r1 = luak_exp2anyreg(state, lexical_state, function_state, e1);
            r2 = luak_exp2anyreg(state, lexical_state, function_state, e2);
            op = binopr2op(opr, OperatorBinary::Less, OPCODE_LT);
        }
        freeexps(lexical_state, function_state, e1, e2);
        (*e1).expressiondescription_value.value_info = condjump(
            state,
            lexical_state,
            function_state,
            op,
            r1 as i32,
            r2 as i32,
            is_float as i32,
            1,
        );
        (*e1).expressiondescription_expressionkind = ExpressionKind::Jump;
    }
}
pub unsafe fn codeeq(
    state: *mut State,
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    opr: OperatorBinary,
    e1: *mut ExpressionDescription,
    e2: *mut ExpressionDescription,
) {
    unsafe {
        let mut im: i64 = 0;
        let mut is_float = false;

        let r2: i64;
        let op: u32;
        if (*e1).expressiondescription_expressionkind != ExpressionKind::Nonrelocatable {
            ExpressionDescription::swapexps(e1, e2);
        }
        let r1: i64 = luak_exp2anyreg(state, lexical_state, function_state, e1);
        if ExpressionDescription::is_sc_number(e2, &mut im, &mut is_float) != 0 {
            op = OPCODE_EQI;
            r2 = im;
        } else if exp2rk(state, lexical_state, function_state, e2) != 0 {
            op = OPCODE_EQK;
            r2 = (*e2).expressiondescription_value.value_info as i64;
        } else {
            op = OPCODE_EQ;
            r2 = luak_exp2anyreg(state, lexical_state, function_state, e2);
        }
        freeexps(lexical_state, function_state, e1, e2);
        (*e1).expressiondescription_value.value_info = condjump(
            state,
            lexical_state,
            function_state,
            op,
            r1 as i32,
            r2 as i32,
            is_float as i32,
            (opr as u32 == OperatorBinary::Equal as u32) as i32,
        );
        (*e1).expressiondescription_expressionkind = ExpressionKind::Jump;
    }
}
pub unsafe fn luak_prefix(
    state: *mut State,
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    unary: OperatorUnary,
    expression_description: *mut ExpressionDescription,
    line: i32,
) {
    unsafe {
        pub const EF: ExpressionDescription = ExpressionDescription::new_from_integer(0);
        luak_dischargevars(state, lexical_state, function_state, expression_description);
        match unary {
            OperatorUnary::BitwiseNot => {
                if constfolding(
                    state,
                    lexical_state,
                    function_state,
                    (unary as u32).wrapping_add(ARITH_UNM as u32) as i32,
                    expression_description,
                    &EF,
                ) == 0
                {
                    code_unary_expression_value(
                        state,
                        lexical_state,
                        function_state,
                        unopr2op(unary),
                        expression_description,
                        line,
                    );
                }
            }
            OperatorUnary::Minus => {
                if constfolding(
                    state,
                    lexical_state,
                    function_state,
                    (unary as u32).wrapping_add(ARITH_UNM as u32) as i32,
                    expression_description,
                    &EF,
                ) == 0
                {
                    code_unary_expression_value(
                        state,
                        lexical_state,
                        function_state,
                        unopr2op(unary),
                        expression_description,
                        line,
                    );
                }
            }
            OperatorUnary::Length => {
                code_unary_expression_value(
                    state,
                    lexical_state,
                    function_state,
                    unopr2op(unary),
                    expression_description,
                    line,
                );
            }
            OperatorUnary::Not => {
                codenot(state, lexical_state, function_state, expression_description);
            }
            _ => {}
        }
    }
}
pub unsafe fn luak_infix(
    state: *mut State,
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    op: OperatorBinary,
    v: *mut ExpressionDescription,
) {
    unsafe {
        luak_dischargevars(state, lexical_state, function_state, v);
        match op as u32 {
            OPCODE_NEWTABLE => {
                luak_goiftrue(state, lexical_state, function_state, v);
            }
            OPCODE_SELF => {
                luak_goiffalse(state, lexical_state, function_state, v);
            }
            OPCODE_GET_TABLE => {
                luak_exp2nextreg(state, lexical_state, function_state, v);
            }
            OPCODE_MOVE
            | OPCODE_LOADI
            | OPCODE_LOADF
            | OPCODE_LOAD_FALSE
            | OPCODE_LFALSESKIP
            | OPCODE_LOADK
            | OPCODE_LOADKX
            | OPCODE_LOAD_TRUE
            | OPCODE_LOADNIL
            | OPCODE_GET_UPVALUE
            | OPCODE_SETUPVAL
            | OPCODE_GET_TABLE_UPVALUE => {
                if !ExpressionDescription::tonumeral(v, null_mut()) {
                    luak_exp2anyreg(state, lexical_state, function_state, v);
                }
            }
            OPCODE_INDEX_INTEGER | OPCODE_SETTABLE => {
                if !ExpressionDescription::tonumeral(v, null_mut()) {
                    exp2rk(state, lexical_state, function_state, v);
                }
            }
            OPCODE_GET_FIELD | OPCODE_SETTABUP | OPCODE_SETI | OPCODE_SETFIELD => {
                let mut dummy1: i64 = 0;
                let mut dummy2 = false;
                if ExpressionDescription::is_sc_number(v, &mut dummy1, &mut dummy2) == 0 {
                    luak_exp2anyreg(state, lexical_state, function_state, v);
                }
            }
            _ => {}
        };
    }
}
pub unsafe fn codeconcat(
    state: *mut State,
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    e1: *mut ExpressionDescription,
    e2: *mut ExpressionDescription,
    line: i32,
) {
    unsafe {
        let ie2 = previousinstruction(function_state);
        if (*ie2 & MASK_OP) == OPCODE_CONCAT {
            let n: i32 = (*ie2 >> POSITION_B & MASK_A) as i32;
            freeexp(lexical_state, function_state, e2);
            *ie2 = *ie2 & !(MASK_A << POSITION_A)
                | ((*e1).expressiondescription_value.value_info as u32) << POSITION_A & MASK_A << POSITION_A;
            *ie2 = *ie2 & !(MASK_B << POSITION_B) | ((n + 1) as u32) << POSITION_B & MASK_B << POSITION_B;
        } else {
            code_abck(
                state,
                lexical_state,
                function_state,
                OPCODE_CONCAT,
                (*e1).expressiondescription_value.value_info,
                2,
                0,
                0,
            );
            freeexp(lexical_state, function_state, e2);
            luak_fixline(state, lexical_state, function_state, line);
        };
    }
}
pub unsafe fn luak_posfix(
    state: *mut State,
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    mut binary: OperatorBinary,
    expression_description_a: *mut ExpressionDescription,
    expression_description_b: *mut ExpressionDescription,
    line: i32,
) {
    unsafe {
        luak_dischargevars(
            state,
            lexical_state,
            function_state,
            expression_description_b,
        );
        if binary as u32 <= OperatorBinary::ShiftRight as u32
            && constfolding(
                state,
                lexical_state,
                function_state,
                binary as i32,
                expression_description_a,
                expression_description_b,
            ) != 0
        {
            return;
        }
        match binary {
            OperatorBinary::And => {
                luak_concat(
                    state,
                    lexical_state,
                    function_state,
                    &mut (*expression_description_b).expressiondescription_f,
                    (*expression_description_a).expressiondescription_f,
                );
                *expression_description_a = *expression_description_b;
            }
            OperatorBinary::Or => {
                luak_concat(
                    state,
                    lexical_state,
                    function_state,
                    &mut (*expression_description_b).expressiondescription_t,
                    (*expression_description_a).expressiondescription_t,
                );
                *expression_description_a = *expression_description_b;
            }
            OperatorBinary::Concatenate => {
                luak_exp2nextreg(
                    state,
                    lexical_state,
                    function_state,
                    expression_description_b,
                );
                codeconcat(
                    state,
                    lexical_state,
                    function_state,
                    expression_description_a,
                    expression_description_b,
                    line,
                );
            }
            OperatorBinary::Add => {
                let mut flip: i32 = 0;
                if ExpressionDescription::tonumeral(expression_description_a, null_mut()) {
                    ExpressionDescription::swapexps(expression_description_a, expression_description_b);
                    flip = 1;
                }
                if ExpressionDescription::is_sc_int(expression_description_b) {
                    codebini(
                        state,
                        lexical_state,
                        function_state,
                        OPCODE_ADDI,
                        expression_description_a,
                        expression_description_b,
                        flip,
                        line,
                        TM_ADD,
                    );
                } else {
                    codearith(
                        state,
                        lexical_state,
                        function_state,
                        binary,
                        expression_description_a,
                        expression_description_b,
                        flip,
                        line,
                    );
                };
            }
            OperatorBinary::Multiply => {
                let mut flip: i32 = 0;
                if ExpressionDescription::tonumeral(expression_description_a, null_mut()) {
                    ExpressionDescription::swapexps(expression_description_a, expression_description_b);
                    flip = 1;
                }
                codearith(
                    state,
                    lexical_state,
                    function_state,
                    binary,
                    expression_description_a,
                    expression_description_b,
                    flip,
                    line,
                );
            }
            OperatorBinary::Subtract => {
                if finishbinexpneg(
                    state,
                    lexical_state,
                    function_state,
                    expression_description_a,
                    expression_description_b,
                    OPCODE_ADDI,
                    line,
                    TM_SUB,
                ) == 0
                {
                    codearith(
                        state,
                        lexical_state,
                        function_state,
                        binary,
                        expression_description_a,
                        expression_description_b,
                        0,
                        line,
                    );
                }
            }
            OperatorBinary::Power => {
                codearith(
                    state,
                    lexical_state,
                    function_state,
                    binary,
                    expression_description_a,
                    expression_description_b,
                    0,
                    line,
                );
            }
            OperatorBinary::Modulus => {
                codearith(
                    state,
                    lexical_state,
                    function_state,
                    binary,
                    expression_description_a,
                    expression_description_b,
                    0,
                    line,
                );
            }
            OperatorBinary::Divide => {
                codearith(
                    state,
                    lexical_state,
                    function_state,
                    binary,
                    expression_description_a,
                    expression_description_b,
                    0,
                    line,
                );
            }
            OperatorBinary::IntegralDivide => {
                codearith(
                    state,
                    lexical_state,
                    function_state,
                    binary,
                    expression_description_a,
                    expression_description_b,
                    0,
                    line,
                );
            }
            OperatorBinary::BitwiseAnd => {
                codebitwise(
                    state,
                    lexical_state,
                    function_state,
                    binary,
                    expression_description_a,
                    expression_description_b,
                    line,
                );
            }
            OperatorBinary::BitwiseOr => {
                codebitwise(
                    state,
                    lexical_state,
                    function_state,
                    binary,
                    expression_description_a,
                    expression_description_b,
                    line,
                );
            }
            OperatorBinary::BitwiseExclusiveOr => {
                codebitwise(
                    state,
                    lexical_state,
                    function_state,
                    binary,
                    expression_description_a,
                    expression_description_b,
                    line,
                );
            }
            OperatorBinary::ShiftLeft => {
                if ExpressionDescription::is_sc_int(expression_description_a) {
                    ExpressionDescription::swapexps(expression_description_a, expression_description_b);
                    codebini(
                        state,
                        lexical_state,
                        function_state,
                        OPCODE_SHLI,
                        expression_description_a,
                        expression_description_b,
                        1,
                        line,
                        TM_SHL,
                    );
                } else if finishbinexpneg(
                    state,
                    lexical_state,
                    function_state,
                    expression_description_a,
                    expression_description_b,
                    OPCODE_SHRI,
                    line,
                    TM_SHL,
                ) == 0
                {
                    codebinexpval(
                        state,
                        lexical_state,
                        function_state,
                        binary,
                        expression_description_a,
                        expression_description_b,
                        line,
                    );
                }
            }
            OperatorBinary::ShiftRight => {
                if ExpressionDescription::is_sc_int(expression_description_b) {
                    codebini(
                        state,
                        lexical_state,
                        function_state,
                        OPCODE_SHRI,
                        expression_description_a,
                        expression_description_b,
                        0,
                        line,
                        TM_SHR,
                    );
                } else {
                    codebinexpval(
                        state,
                        lexical_state,
                        function_state,
                        binary,
                        expression_description_a,
                        expression_description_b,
                        line,
                    );
                }
            }
            OperatorBinary::Inequal => {
                codeeq(
                    state,
                    lexical_state,
                    function_state,
                    binary,
                    expression_description_a,
                    expression_description_b,
                );
            }
            OperatorBinary::Equal => {
                codeeq(
                    state,
                    lexical_state,
                    function_state,
                    binary,
                    expression_description_a,
                    expression_description_b,
                );
            }
            OperatorBinary::GreaterEqual => {
                ExpressionDescription::swapexps(expression_description_a, expression_description_b);
                binary = OperatorBinary::LessEqual;
                codeorder(
                    state,
                    lexical_state,
                    function_state,
                    binary,
                    expression_description_a,
                    expression_description_b,
                );
            }
            OperatorBinary::Greater => {
                ExpressionDescription::swapexps(expression_description_a, expression_description_b);
                binary = OperatorBinary::Less;
                codeorder(
                    state,
                    lexical_state,
                    function_state,
                    binary,
                    expression_description_a,
                    expression_description_b,
                );
            }
            OperatorBinary::Less => {
                codeorder(
                    state,
                    lexical_state,
                    function_state,
                    binary,
                    expression_description_a,
                    expression_description_b,
                );
            }
            OperatorBinary::LessEqual => {
                codeorder(
                    state,
                    lexical_state,
                    function_state,
                    binary,
                    expression_description_a,
                    expression_description_b,
                );
            }
            _ => {}
        }
    }
}
pub unsafe fn luak_fixline(state: *mut State, lexical_state: *mut LexicalState, function_state: *mut FunctionState, line: i32) {
    unsafe {
        removelastlineinfo(function_state);
        savelineinfo(
            state,
            lexical_state,
            function_state,
            (*function_state).functionstate_prototype,
            line,
        );
    }
}
pub const SIZE_OP: usize = 7;
pub const SIZE_A: usize = 8;
pub const SIZE_B: usize = 8;
pub const SIZE_C: usize = 8;
pub const SIZE_K: usize = 1;
pub const SIZE_BX: usize = SIZE_K + SIZE_B + SIZE_C;
pub const SIZE_AX: usize = SIZE_A + SIZE_BX;
pub const POSITION_A: usize = SIZE_OP;
pub const POSITION_K: usize = POSITION_A + SIZE_A;
pub const POSITION_B: usize = POSITION_K + SIZE_K;
pub const POSITION_C: usize = POSITION_B + SIZE_B;
pub const MASK_OP: u32 = (1 << SIZE_OP) - 1;
pub const MASK_A: u32 = (1 << SIZE_A) - 1;
pub const MASK_B: u32 = (1 << SIZE_B) - 1;
pub const MASK_C: u32 = (1 << SIZE_C) - 1;
pub const MASK_K: u32 = (1 << SIZE_K) - 1;
pub const MASK_BX: u32 = (1 << SIZE_BX) - 1;
pub const MASK_AX: u32 = (1 << SIZE_AX) - 1;
pub const MAXARG_BX: i32 = MASK_BX as i32;
pub const MAXARG_AX: usize = MASK_AX as usize;
pub const OFFSET_SBX: i32 = MAXARG_BX >> 1;
pub const OFFSET_SJ: i32 = (MAXARG_AX >> 1) as i32;
pub const MAXARG_B: i32 = MASK_B as i32;
pub const MAXARG_C: i32 = MASK_C as i32;
pub const OFFSET_SC: i32 = MAXARG_C >> 1;
pub const NO_REG: i32 = MASK_A as i32;
pub const MAXREGS: i32 = MASK_A as i32;
pub const LUA_NUM_TYPES: usize = 9;
pub const LUAI_MAXCCALLS: u32 = 200;
pub const LUAI_MAXCCALLS_ERRERR: u32 = LUAI_MAXCCALLS / 10 * 11;
pub const LUAI_MAXSTACK: i32 = 1000000;
pub const LUA_EXTRA_SPACE: i32 = 1000;
pub const LUA_REGISTRYINDEX: i32 = -(LUAI_MAXSTACK + LUA_EXTRA_SPACE);
pub const MAXTAGLOOP: i32 = 2000;
pub const LUA_MINSTACK: i32 = 20;
pub const LUAL_BUFFERSIZE: usize = 8192;
pub const MAXUPVAL: usize = 255;
pub const BLACKBIT: u8 = 1 << 5;
pub const WHITE0BIT: u8 = 1 << 3;
pub const WHITE1BIT: u8 = 1 << 4;
pub const WHITEBITS: u8 = WHITE0BIT | WHITE1BIT;
pub const FINALIZEDBIT: u8 = 1 << 6;
pub const AGEBITS: u8 = 0x07;
pub const AGE_NEW: u8 = 0;
pub const AGE_SURVIVAL: u8 = 1;
pub const AGE_OLD0: u8 = 2;
pub const AGE_OLD1: u8 = 3;
pub const AGE_OLD: u8 = 4;
pub const AGE_TOUCHED1: u8 = 5;
pub const AGE_TOUCHED2: u8 = 6;
pub const CALLSTATUS_ALLOWHOOK: i32 = 1 << 0;
pub const CALLSTATUS_LUA: i32 = 1 << 1;
pub const CALLSTATUS_HOOKED: i32 = 1 << 2;
pub const CALLSTATUS_FRESH: i32 = 1 << 3;
pub const CALLSTATUS_YPCALL: i32 = 1 << 4;
pub const CALLSTATUS_TAIL: i32 = 1 << 5;
pub const CALLSTATUS_HOOKYIELD: i32 = 1 << 6;
pub const CALLSTATUS_LEQ: i32 = 1 << 7;
pub const CALLSTATUS_FIN: i32 = 1 << 8;
pub const CALLSTATUS_TRAN: i32 = 1 << 9;
pub const HOOKMASK_CALL: i32 = 1 << 0;
pub const HOOKMASK_RET: i32 = 1 << 1;
pub const HOOKMASK_LINE: i32 = 1 << 2;
pub const HOOKMASK_COUNT: i32 = 1 << 3;
pub const MAXLINEDIFF: i32 = 0x80;
pub const MAXIWTHABS: i32 = 128;
pub const MAX_INT: usize = 0x7FFFFFFF;
pub const LUAI_MAXSHORTLEN: usize = 40;
pub const LFIELDS_PER_FLUSH: i32 = 50;
pub const DOUBLE_MANTISSA_BITS: i32 = 53;
pub const LUA_IDSIZE: usize = 60;
pub const LUA_N2SBUFFSZ: usize = 44;
pub const BUFVFS: usize = LUA_IDSIZE + LUA_N2SBUFFSZ + 95;
const FREEREGS_LOTS: i32 = 160;
const FREEREGS_ENOUGH: i32 = 80;
const FREEREGS_DIVISOR: i32 = 5;
const FREEREGS_BATCH: i32 = 10;
pub unsafe fn maxtostore(function_state: *mut FunctionState) -> i32 {
    unsafe {
        let numfreeregs: i32 = MAXREGS - (*function_state).functionstate_free_register as i32;
        if numfreeregs >= FREEREGS_LOTS {
            numfreeregs / FREEREGS_DIVISOR
        } else if numfreeregs >= FREEREGS_ENOUGH {
            FREEREGS_BATCH
        } else {
            1
        }
    }
}
pub const ARITH_UNM: i32 = 12;
pub const ARITH_BNOT: i32 = 13;
pub const ARGS_INTERACTIVE: i32 = 2;
pub const ARGS_VERSION: i32 = 4;
pub const ARGS_EXECUTE: i32 = 8;
pub const ARGS_NOENV: i32 = 16;
pub const ARGS_BARE: i32 = 32;
pub const RDKCONST: i32 = 1;
pub const RDKVAVAR: i32 = 2;
pub const RDKTOCLOSE: i32 = 3;
pub const RDKCTC: i32 = 4;
pub const GDKREG: i32 = 5;
pub const GDKCONST: i32 = 6;
pub const NBM: i32 = 53;
pub unsafe fn luak_settablesize(
    _state: *mut State,
    function_state: *mut FunctionState,
    program_counter: i32,
    ra: i32,
    asize: i32,
    hsize: i32,
) {
    unsafe {
        let inst: *mut u32 = &mut *((*(*function_state).functionstate_prototype)
            .prototype_code
            .vectort_pointer)
            .add(program_counter as usize) as *mut u32;
        let rb: i32 = if hsize == 0 {
            0
        } else {
            1 + hsize.ilog2() as i32
        };
        let extra: i32 = asize / (MAXARG_C + 1);
        let rc: i32 = asize % (MAXARG_C + 1);
        let k: i32 = (extra > 0) as i32;
        *inst = OPCODE_NEWTABLE
            | (ra as u32) << POSITION_A
            | (rb as u32) << POSITION_B
            | (rc as u32) << POSITION_C
            | (k as u32) << POSITION_K;
        *inst.add(1) = OPCODE_EXTRAARG | (extra as u32) << POSITION_A;
    }
}
pub unsafe fn luak_setlist(
    state: *mut State,
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    base: i32,
    mut count_elements: usize,
    mut tostore: i32,
) {
    unsafe {
        if tostore == -1 {
            tostore = 0;
        }
        if count_elements <= MAXARG_C as usize {
            code_abck(
                state,
                lexical_state,
                function_state,
                OPCODE_SETLIST,
                base,
                tostore,
                count_elements as i32,
                0,
            );
        } else {
            let extra = count_elements / (MAXARG_C as usize + 1);
            count_elements %= MAXARG_C as usize + 1;
            code_abck(
                state,
                lexical_state,
                function_state,
                OPCODE_SETLIST,
                base,
                tostore,
                count_elements as i32,
                1,
            );
            codeextraarg(state, lexical_state, function_state, extra as i32);
        }
        (*function_state).functionstate_free_register = (base + 1) as u8;
    }
}
pub unsafe fn luak_finish(
    state: *mut State,
    lexical_state: *mut LexicalState,
    function_state: *mut FunctionState,
    prototype: *mut Prototype,
) {
    unsafe {
        // If function needs a vararg table, it won't use hidden args
        if (*prototype).prototype_needsvarargtable {
            (*prototype).prototype_isvariablearguments = false;
        }
        for i in 0..(*function_state).functionstate_program_counter {
            let program_counter: *mut u32 = &mut *((*prototype).prototype_code.vectort_pointer).add(i as usize) as *mut u32;
            match *program_counter & MASK_OP {
                OPCODE_RETURN0 | OPCODE_RETURN1 => {
                    if (*function_state).functionstate_needs_close || (*prototype).prototype_isvariablearguments {
                        *program_counter = *program_counter & !(MASK_OP) | OPCODE_RETURN & MASK_OP;
                        if (*function_state).functionstate_needs_close {
                            *program_counter =
                                *program_counter & !(MASK_K << POSITION_K) | 1_u32 << POSITION_K & MASK_K << POSITION_K;
                        }
                        if (*prototype).prototype_isvariablearguments {
                            *program_counter = *program_counter & !(MASK_C << POSITION_C)
                                | (((*prototype).prototype_countparameters as i32 + 1) as u32) << POSITION_C & MASK_C << POSITION_C;
                        }
                    }
                }
                OPCODE_RETURN | OPCODE_TAILCALL => {
                    if (*function_state).functionstate_needs_close {
                        *program_counter = *program_counter & !(MASK_K << POSITION_K) | 1_u32 << POSITION_K & MASK_K << POSITION_K;
                    }
                    if (*prototype).prototype_isvariablearguments {
                        *program_counter = *program_counter & !(MASK_C << POSITION_C)
                            | (((*prototype).prototype_countparameters as i32 + 1) as u32) << POSITION_C & MASK_C << POSITION_C;
                    }
                }
                OPCODE_GETVARG => {
                    if (*prototype).prototype_needsvarargtable {
                        // Rewrite to OP_GETTABLE since vararg is a real table
                        *program_counter = *program_counter & !(MASK_OP) | OPCODE_GET_TABLE & MASK_OP;
                    }
                }
                OPCODE_VARARG => {
                    if (*prototype).prototype_needsvarargtable {
                        // Set k bit to signal vararg table mode
                        *program_counter = *program_counter & !(MASK_K << POSITION_K) | 1_u32 << POSITION_K & MASK_K << POSITION_K;
                    }
                }
                OPCODE_JMP => {
                    let target: i32 = final_target((*prototype).prototype_code.vectort_pointer, i);
                    fixjump(state, lexical_state, function_state, i, target);
                }
                _ => {}
            }
        }
    }
}
