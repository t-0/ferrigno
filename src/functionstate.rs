use crate::absolutelineinfo::*;
use crate::blockcontrol::*;
use crate::constructorcontrol::*;
use crate::expressiondescription::*;
use crate::expressionkind::*;
use crate::f2i::*;
use crate::instruction::*;
use crate::interpreter::*;
use crate::lexicalstate::*;
use crate::localvariable::*;
use crate::object::*;
use crate::opcode::*;
use crate::operator_::*;
use crate::operatorbinary::*;
use crate::operatorunary::*;
use crate::opmode::*;
use crate::prototype::*;
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
use libc::*;
use std::ptr::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FunctionState {
    pub functionstate_prototype: *mut Prototype,
    pub functionstate_previous: *mut FunctionState,
    pub functionstate_blockcontrol: *mut BlockControl,
    pub functionstate_programcounter: i32,
    pub functionstate_lasttarget: i32,
    pub functionstate_previousline: i32,
    pub functionstate_countconstants: i32,
    pub functionstate_countprototypes: i32,
    pub functionstate_countabslineinfo: i32,
    pub functionstate_firstlocal: i32,
    pub functionstate_firstlabel: i32,
    pub functionstate_countdebugvariables: usize,
    pub functionstate_countactivevariables: usize,
    pub functionstate_countupvalues: usize,
    pub functionstate_freereg: u8,
    pub functionstate_iwthabs: u8,
    pub functionstate_needsclose: bool,
}
impl TDefaultNew for FunctionState {
    fn new() -> Self {
        return FunctionState {
            functionstate_prototype: null_mut(),
            functionstate_previous: null_mut(),
            functionstate_blockcontrol: null_mut(),
            functionstate_programcounter: 0,
            functionstate_lasttarget: 0,
            functionstate_previousline: 0,
            functionstate_countconstants: 0,
            functionstate_countprototypes: 0,
            functionstate_countabslineinfo: 0,
            functionstate_firstlocal: 0,
            functionstate_firstlabel: 0,
            functionstate_countdebugvariables: 0,
            functionstate_countactivevariables: 0,
            functionstate_countupvalues: 0,
            functionstate_freereg: 0,
            functionstate_iwthabs: 0,
            functionstate_needsclose: false,
        };
    }
}
impl FunctionState {
    pub unsafe fn marktobeclosed(& mut self) {
        unsafe {
            (*self.functionstate_blockcontrol).marktobeclosed();
            self.functionstate_needsclose = true;
        }
    }
    pub fn get_first_goto(& self) -> usize {
        unsafe {
            (*self.functionstate_blockcontrol).get_first_goto()
        }
    }
    pub fn code_get_label(&mut self) -> i32 {
        self.functionstate_lasttarget = self.functionstate_programcounter;
        return self.functionstate_programcounter;
    }
    pub unsafe fn mark_upvalue(&mut self, level: usize) {
        unsafe {
            (*self.functionstate_blockcontrol).mark_upvalue_delegated(level);
            self.functionstate_needsclose = true;
        }
    }
}
pub unsafe fn closelistfield(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState,
    constructor_control: *mut ConstructorControl,
) {
    unsafe {
        if (*constructor_control)
            .constructorcontrol_expressiondescription
            .expressiondescription_expressionkind
            == ExpressionKind::Void
        {
            return;
        }
        luak_exp2nextreg(
            interpreter,
            lexical_state,
            function_state,
            &mut (*constructor_control).constructorcontrol_expressiondescription,
        );
        (*constructor_control)
            .constructorcontrol_expressiondescription
            .expressiondescription_expressionkind = ExpressionKind::Void;
        if (*constructor_control).constructorcontrol_counttostore == 50 as i32 {
            luak_setlist(
                interpreter,
                lexical_state,
                function_state,
                (*(*constructor_control).constructorcontrol_table)
                    .expressiondescription_value
                    .value_info,
                (*constructor_control).constructorcontrol_countarray as usize,
                (*constructor_control).constructorcontrol_counttostore,
            );
            (*constructor_control).constructorcontrol_countarray += (*constructor_control).constructorcontrol_counttostore;
            (*constructor_control).constructorcontrol_counttostore = 0;
        }
    }
}
pub unsafe fn lastlistfield(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState,
    constructor_control: *mut ConstructorControl,
) {
    unsafe {
        if (*constructor_control).constructorcontrol_counttostore == 0 {
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
                interpreter,
                lexical_state,
                function_state,
                &mut (*constructor_control).constructorcontrol_expressiondescription,
                -1,
            );
            luak_setlist(
                interpreter,
                lexical_state,
                function_state,
                (*(*constructor_control).constructorcontrol_table)
                    .expressiondescription_value
                    .value_info,
                (*constructor_control).constructorcontrol_countarray as usize,
                -1,
            );
            (*constructor_control).constructorcontrol_countarray -= 1;
            (*constructor_control).constructorcontrol_countarray;
        } else {
            if (*constructor_control)
                .constructorcontrol_expressiondescription
                .expressiondescription_expressionkind
                != ExpressionKind::Void
            {
                luak_exp2nextreg(
                    interpreter,
                    lexical_state,
                    function_state,
                    &mut (*constructor_control).constructorcontrol_expressiondescription,
                );
            }
            luak_setlist(
                interpreter,
                lexical_state,
                function_state,
                (*(*constructor_control).constructorcontrol_table)
                    .expressiondescription_value
                    .value_info,
                (*constructor_control).constructorcontrol_countarray as usize,
                (*constructor_control).constructorcontrol_counttostore,
            );
        }
        (*constructor_control).constructorcontrol_countarray += (*constructor_control).constructorcontrol_counttostore;
    }
}
pub unsafe fn setvararg(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, nparams: i32,
) {
    unsafe {
        (*(*function_state).functionstate_prototype).prototype_isvariablearguments = true;
        code_abck(interpreter, lexical_state, function_state, OPCODE_VARARGPREP, nparams, 0, 0, 0);
    }
}
pub unsafe fn errorlimit(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, limit: i32,
    what: *const i8,
) -> ! {
    unsafe {
        let message: *const i8;
        let line: i32 = (*(*function_state).functionstate_prototype).prototype_linedefined;
        let where_0: *const i8 = if line == 0 {
            c"main function".as_ptr()
        } else {
            luao_pushfstring(interpreter, c"function at line %d".as_ptr(), line)
        };
        message = luao_pushfstring(interpreter, c"too many %s (limit is %d) in %s".as_ptr(), what, limit, where_0);
        luax_syntaxerror(interpreter, lexical_state, message);
    }
}
pub unsafe fn checklimit(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, v: i32, length: i32,
    what: *const i8,
) {
    unsafe {
        if v > length {
            errorlimit(interpreter, lexical_state, function_state, length, what);
        }
    }
}
pub unsafe fn getlocalvardesc(
    lexical_state: *mut LexicalState, function_state: *mut FunctionState, vidx: i32,
) -> *mut VariableDescription {
    unsafe {
        return &mut *((*(*lexical_state).lexicalstate_dynamicdata)
            .dynamicdata_activevariables
            .vectort_pointer)
            .offset(((*function_state).functionstate_firstlocal + vidx) as isize) as *mut VariableDescription;
    }
}
pub unsafe fn reglevel(lexical_state: *mut LexicalState, function_state: *mut FunctionState, mut nvar: i32) -> i32 {
    unsafe {
        loop {
            let fresh38 = nvar;
            nvar = nvar - 1;
            if !(fresh38 > 0) {
                break;
            }
            let variable_description: *mut VariableDescription = getlocalvardesc(lexical_state, function_state, nvar);
            if (*variable_description)
                .variabledescription_content
                .variabledescriptioncontent_kind as i32
                != 3
            {
                return (*variable_description)
                    .variabledescription_content
                    .variabledescriptioncontent_registerindex as i32
                    + 1;
            }
        }
        return 0;
    }
}
pub unsafe fn luay_nvarstack(lexical_state: *mut LexicalState, function_state: *mut FunctionState) -> i32 {
    unsafe {
        return reglevel(
            lexical_state,
            function_state,
            (*function_state).functionstate_countactivevariables as i32,
        );
    }
}
pub unsafe fn localdebuginfo(
    lexical_state: *mut LexicalState, function_state: *mut FunctionState, vidx: i32,
) -> *mut LocalVariable {
    unsafe {
        let variable_description: *mut VariableDescription = getlocalvardesc(lexical_state, function_state, vidx);
        if (*variable_description)
            .variabledescription_content
            .variabledescriptioncontent_kind as i32
            == 3
        {
            return null_mut();
        } else {
            let index: i32 = (*variable_description)
                .variabledescription_content
                .variabledescriptioncontent_pidx as i32;
            return (*(*function_state).functionstate_prototype)
                .prototype_localvariables
                .at_mut(index as isize);
        };
    }
}
pub unsafe fn init_var(
    lexical_state: *mut LexicalState, function_state: *mut FunctionState, expression_description: *mut ExpressionDescription,
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
            .dynamicdata_activevariables
            .subtract_length(((*function_state).functionstate_countactivevariables as i32 - tolevel) as usize);
        while (*function_state).functionstate_countactivevariables as i32 > tolevel {
            (*function_state).functionstate_countactivevariables -= 1;
            let localvariable: *mut LocalVariable = localdebuginfo(
                lexical_state,
                function_state,
                (*function_state).functionstate_countactivevariables as i32,
            );
            if !localvariable.is_null() {
                (*localvariable).localvariable_endprogramcounter = (*function_state).functionstate_programcounter;
            }
        }
    }
}
pub unsafe fn searchupvalue(function_state: *mut FunctionState, name: *mut TString) -> i32 {
    unsafe {
        let upvaluedescription: *mut UpValueDescription =
            (*(*function_state).functionstate_prototype).prototype_upvalues.vectort_pointer;
        for i in 0..(*function_state).functionstate_countupvalues {
            if (*upvaluedescription.offset(i as isize)).upvaluedescription_name == name {
                return i as i32;
            }
        }
        return -1;
    }
}
pub unsafe fn allocate_upvalue_description(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, prototype: *mut Prototype,
) -> *mut UpValueDescription {
    unsafe {
        let mut oldsize = (*prototype).prototype_upvalues.get_size();
        checklimit(
            interpreter,
            lexical_state,
            function_state,
            (*function_state).functionstate_countupvalues as i32 + 1,
            255 as i32,
            c"upvalues".as_ptr(),
        );
        (*prototype).prototype_upvalues.grow(
            interpreter,
            (*function_state).functionstate_countupvalues as usize,
            if 255 as usize <= (!0usize) / size_of::<UpValueDescription>() {
                255
            } else {
                (!0usize) / size_of::<UpValueDescription>()
            },
            c"upvalues".as_ptr(),
        );
        while oldsize < (*prototype).prototype_upvalues.get_size() {
            let fresh41 = oldsize;
            oldsize = oldsize + 1;
            let ref mut fresh42 =
                (*((*prototype).prototype_upvalues.vectort_pointer).offset(fresh41 as isize)).upvaluedescription_name;
            *fresh42 = null_mut();
        }
        let count_upvalues = (*function_state).functionstate_countupvalues;
        (*function_state).functionstate_countupvalues += 1;
        return &mut *((*prototype).prototype_upvalues.vectort_pointer).offset(count_upvalues as isize) as *mut UpValueDescription;
    }
}
pub unsafe fn newupvalue(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, name: *mut TString,
    v: *mut ExpressionDescription,
) -> i32 {
    unsafe {
        let upvalue_description: *mut UpValueDescription = allocate_upvalue_description(
            interpreter,
            lexical_state,
            function_state,
            (*function_state).functionstate_prototype,
        );
        let previous: *mut FunctionState = (*function_state).functionstate_previous;
        if (*v).expressiondescription_expressionkind == ExpressionKind::Local {
            (*upvalue_description).upvaluedescription_isinstack = true;
            (*upvalue_description).upvaluedescription_index =
                (*v).expressiondescription_value.value_variable.valueregister_registerindex;
            (*upvalue_description).upvaluedescription_kind = (*getlocalvardesc(
                lexical_state,
                previous,
                (*v).expressiondescription_value.value_variable.valueregister_valueindex as i32,
            ))
            .variabledescription_content
            .variabledescriptioncontent_kind;
        } else {
            (*upvalue_description).upvaluedescription_isinstack = false;
            (*upvalue_description).upvaluedescription_index = (*v).expressiondescription_value.value_info as u8;
            (*upvalue_description).upvaluedescription_kind =
                (*((*(*previous).functionstate_prototype).prototype_upvalues.vectort_pointer)
                    .offset((*v).expressiondescription_value.value_info as isize))
                .upvaluedescription_kind;
        }
        (*upvalue_description).upvaluedescription_name = name;
        if (*(*function_state).functionstate_prototype).get_marked() & 1 << 5 != 0 && (*name).get_marked() & (1 << 3 | 1 << 4) != 0
        {
            Object::luac_barrier_(
                interpreter,
                &mut (*((*function_state).functionstate_prototype as *mut Object)),
                &mut (*(name as *mut Object)),
            );
        } else {
        };
        return (*function_state).functionstate_countupvalues as i32 - 1;
    }
}
pub unsafe fn searchvar(
    lexical_state: *mut LexicalState, function_state: *mut FunctionState, n: *mut TString, var: *mut ExpressionDescription,
) -> i32 {
    unsafe {
        let mut i = (*function_state).functionstate_countactivevariables as i32 - 1;
        while i >= 0 {
            let variable_description: *mut VariableDescription = getlocalvardesc(lexical_state, function_state, i);
            if n == (*variable_description)
                .variabledescription_content
                .variabledescriptioncontent_name
            {
                if (*variable_description)
                    .variabledescription_content
                    .variabledescriptioncontent_kind as i32
                    == 3
                {
                    ExpressionDescription::init_exp(var, ExpressionKind::Constant2, (*function_state).functionstate_firstlocal + i);
                } else {
                    init_var(lexical_state, function_state, var, i);
                }
                return (*var).expressiondescription_expressionkind as i32;
            }
            i -= 1;
        }
        -1
    }
}
pub unsafe fn singlevaraux(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, n: *mut TString,
    var: *mut ExpressionDescription, base: i32,
) {
    unsafe {
        if function_state.is_null() {
            ExpressionDescription::init_exp(var, ExpressionKind::Void, 0);
        } else {
            let v: i32 = searchvar(lexical_state, function_state, n, var);
            if v >= 0 {
                if v == ExpressionKind::Local as i32 && base == 0 {
                    (*function_state)
                        .mark_upvalue((*var).expressiondescription_value.value_variable.valueregister_valueindex as usize);
                }
            } else {
                let mut index: i32 = searchupvalue(function_state, n);
                if index < 0 {
                    singlevaraux(interpreter, lexical_state, (*function_state).functionstate_previous, n, var, 0);
                    if (*var).expressiondescription_expressionkind == ExpressionKind::Local
                        || (*var).expressiondescription_expressionkind == ExpressionKind::UpValue
                    {
                        index = newupvalue(interpreter, lexical_state, function_state, n, var);
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
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, program_counter: i32,
    dest: i32, back: i32,
) {
    unsafe {
        let jmp: *mut u32 = &mut *((*(*function_state).functionstate_prototype).prototype_code.vectort_pointer)
            .offset(program_counter as isize) as *mut u32;
        let mut offset: i32 = dest - (program_counter + 1);
        if back != 0 {
            offset = -offset;
        }
        if offset > (1 << 8 + 8 + 1) - 1 {
            luax_syntaxerror(interpreter, lexical_state, c"control structure too long".as_ptr());
        }
        *jmp =
            *jmp & !(!(0xFFFFFFFFu32 << 8 + 8 + 1) << POSITION_K) | (offset as u32) << POSITION_K & !(0xFFFFFFFFu32 << 8 + 8 + 1) << POSITION_K;
    }
}
pub unsafe fn checktoclose(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, level: i32,
) {
    unsafe {
        if level != -1 {
            (*function_state).marktobeclosed();
            code_abck(
                interpreter,
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
        if (*function_state).functionstate_programcounter > (*function_state).functionstate_lasttarget {
            return &mut *((*(*function_state).functionstate_prototype).prototype_code.vectort_pointer)
                .offset(((*function_state).functionstate_programcounter - 1) as isize) as *mut u32;
        } else {
            return &INVALID_INSTRUCTION as *const u32 as *mut u32;
        };
    }
}
pub unsafe fn code_constant_nil(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, mut from: i32, n: i32,
) {
    unsafe {
        let mut length: i32 = from + n - 1;
        let previous: *mut u32 = previousinstruction(function_state);
        if (*previous >> 0 & !(0xFFFFFFFFu32 << 7) << 0) as u32 == OPCODE_LOADNIL as u32 {
            let pfrom: i32 = (*previous >> POSITION_A & !(0xFFFFFFFFu32 << 8) << 0) as i32;
            let pl: i32 = pfrom + (*previous >> POSITION_B & !(0xFFFFFFFFu32 << 8) << 0) as i32;
            if pfrom <= from && from <= pl + 1 || from <= pfrom && pfrom <= length + 1 {
                if pfrom < from {
                    from = pfrom;
                }
                if pl > length {
                    length = pl;
                }
                *previous =
                    *previous & !(!(0xFFFFFFFFu32 << 8) << POSITION_A) | (from as u32) << POSITION_A & !(0xFFFFFFFFu32 << 8) << POSITION_A;
                *previous = *previous & !(!(0xFFFFFFFFu32 << 8) << POSITION_B)
                    | ((length - from) as u32) << POSITION_B & !(0xFFFFFFFFu32 << 8) << POSITION_B;
                return;
            }
        }
        code_abck(interpreter, lexical_state, function_state, OPCODE_LOADNIL, from, n - 1, 0, 0);
    }
}
pub unsafe fn code_get_jump(function_state: *mut FunctionState, program_counter: i32) -> i32 {
    unsafe {
        let offset: i32 = (*((*(*function_state).functionstate_prototype).prototype_code.vectort_pointer)
            .offset(program_counter as isize)
            >> POSITION_A
            & !(0xFFFFFFFFu32 << 8 + 8 + 1 + 8) << 0) as i32
            - ((1 << 8 + 8 + 1 + 8) - 1 >> 1);
        if offset == -1 {
            return -1;
        } else {
            return program_counter + 1 + offset;
        };
    }
}
pub unsafe fn fixjump(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, program_counter: i32,
    dest: i32,
) {
    unsafe {
        let jmp: *mut u32 = &mut *((*(*function_state).functionstate_prototype).prototype_code.vectort_pointer)
            .offset(program_counter as isize) as *mut u32;
        let offset: i32 = dest - (program_counter + 1);
        if !(-((1 << 8 + 8 + 1 + 8) - 1 >> 1) <= offset && offset <= (1 << 8 + 8 + 1 + 8) - 1 - ((1 << 8 + 8 + 1 + 8) - 1 >> 1)) {
            luax_syntaxerror(interpreter, lexical_state, c"control structure too long".as_ptr());
        }
        *jmp = *jmp & !(!(0xFFFFFFFFu32 << 8 + 8 + 1 + 8) << POSITION_A)
            | ((offset + ((1 << 8 + 8 + 1 + 8) - 1 >> 1)) as u32) << POSITION_A & !(0xFFFFFFFFu32 << 8 + 8 + 1 + 8) << POSITION_A;
    }
}
pub unsafe fn luak_concat(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, l1: *mut i32, l2: i32,
) {
    unsafe {
        if l2 == -1 {
            return;
        } else if *l1 == -1 {
            *l1 = l2;
        } else {
            let mut list: i32 = *l1;
            let mut next: i32;
            loop {
                next = code_get_jump(function_state, list);
                if !(next != -1) {
                    break;
                }
                list = next;
            }
            fixjump(interpreter, lexical_state, function_state, list, l2);
        };
    }
}
pub unsafe fn luak_jump(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState,
) -> i32 {
    unsafe {
        return codesj(interpreter, lexical_state, function_state, OPCODE_JMP, -1, 0);
    }
}
pub unsafe fn luak_ret(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, first: i32, nret: i32,
) {
    unsafe {
        let op: u32;
        match nret {
            | 0 => {
                op = OPCODE_RETURN0;
            },
            | 1 => {
                op = OPCODE_RETURN1;
            },
            | _ => {
                op = OPCODE_RETURN;
            },
        }
        code_abck(interpreter, lexical_state, function_state, op, first, nret + 1, 0, 0);
    }
}
pub unsafe fn condjump(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, op: u32, a: i32, b: i32,
    c: i32, k: i32,
) -> i32 {
    unsafe {
        code_abck(interpreter, lexical_state, function_state, op, a, b, c, k);
        return luak_jump(interpreter, lexical_state, function_state);
    }
}
pub unsafe fn code_get_jump_control(function_state: *mut FunctionState, program_counter: i32) -> *mut u32 {
    unsafe {
        let pi: *mut u32 = &mut *((*(*function_state).functionstate_prototype).prototype_code.vectort_pointer)
            .offset(program_counter as isize) as *mut u32;
        if program_counter >= 1 && OPMODES[(*pi.offset(-(1 as isize)) >> 0 & !(0xFFFFFFFFu32 << 7) << 0) as usize] as i32 & 1 << 4 != 0 {
            return pi.offset(-(1 as isize));
        } else {
            return pi;
        };
    }
}
pub unsafe fn patchtestreg(function_state: *mut FunctionState, node: i32, reg: i32) -> i32 {
    unsafe {
        let i: *mut u32 = code_get_jump_control(function_state, node);
        if (*i >> 0 & !(0xFFFFFFFFu32 << 7) << 0) != OPCODE_TESTSET {
            return 0;
        }
        if reg != (1 << 8) - 1 && reg != (*i >> POSITION_B & !(0xFFFFFFFFu32 << 8) << 0) as i32 {
            *i = *i & !(!(0xFFFFFFFFu32 << 8) << POSITION_A) | (reg as u32) << POSITION_A & !(0xFFFFFFFFu32 << 8) << POSITION_A;
        } else {
            *i = OPCODE_TEST << 0
                | ((*i >> POSITION_B & !(0xFFFFFFFFu32 << 8) << 0)) << POSITION_A
                | 0 << POSITION_B
                | 0 << POSITION_C
                | ((*i >> POSITION_K & !(0xFFFFFFFFu32 << 1) << 0)) << POSITION_K;
        }
        return 1;
    }
}
pub unsafe fn removevalues(function_state: *mut FunctionState, mut list: i32) {
    unsafe {
        while list != -1 {
            patchtestreg(function_state, list, (1 << 8) - 1);
            list = code_get_jump(function_state, list);
        }
    }
}
pub unsafe fn patchlistaux(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, mut list: i32,
    vtarget: i32, reg: i32, dtarget: i32,
) {
    unsafe {
        while list != -1 {
            let next: i32 = code_get_jump(function_state, list);
            if patchtestreg(function_state, list, reg) != 0 {
                fixjump(interpreter, lexical_state, function_state, list, vtarget);
            } else {
                fixjump(interpreter, lexical_state, function_state, list, dtarget);
            }
            list = next;
        }
    }
}
pub unsafe fn luak_patchlist(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, list: i32, target: i32,
) {
    unsafe {
        patchlistaux(interpreter, lexical_state, function_state, list, target, (1 << 8) - 1, target);
    }
}
pub unsafe fn luak_patchtohere(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, list: i32,
) {
    unsafe {
        let hr: i32 = (*function_state).code_get_label();
        luak_patchlist(interpreter, lexical_state, function_state, list, hr);
    }
}
pub unsafe fn savelineinfo(
    interpreter: *mut Interpreter, _lexical_state: *mut LexicalState, function_state: *mut FunctionState,
    prototype: *mut Prototype, line: i32,
) {
    unsafe {
        let mut linedif: i32 = line - (*function_state).functionstate_previousline;
        let program_counter: i32 = (*function_state).functionstate_programcounter - 1;
        if abs(linedif) >= 0x80 as i32 || {
            let fresh132 = (*function_state).functionstate_iwthabs;
            (*function_state).functionstate_iwthabs = ((*function_state).functionstate_iwthabs).wrapping_add(1);
            fresh132 as i32 >= 128 as i32
        } {
            (*prototype).prototype_absolutelineinfo.grow(
                interpreter,
                (*function_state).functionstate_countabslineinfo as usize,
                if 0x7FFFFFFF as usize <= (!0usize) / size_of::<AbsoluteLineInfo>() {
                    0x7FFFFFFF
                } else {
                    (!0usize) / size_of::<AbsoluteLineInfo>()
                },
                c"lines".as_ptr(),
            );
            (*((*prototype).prototype_absolutelineinfo.vectort_pointer)
                .offset((*function_state).functionstate_countabslineinfo as isize))
            .absolutelineinfo_programcounter = program_counter;
            let fresh133 = (*function_state).functionstate_countabslineinfo;
            (*function_state).functionstate_countabslineinfo = (*function_state).functionstate_countabslineinfo + 1;
            (*((*prototype).prototype_absolutelineinfo.vectort_pointer).offset(fresh133 as isize)).absolutelineinfo_line = line;
            linedif = -(0x80 as i32);
            (*function_state).functionstate_iwthabs = 1;
        }
        (*prototype).prototype_lineinfo.grow(
            interpreter,
            program_counter as usize,
            if 0x7FFFFFFF <= (!(0usize)) { 0x7FFFFFFF } else { !(0usize) },
            c"opcodes".as_ptr(),
        );
        *((*prototype).prototype_lineinfo.vectort_pointer).offset(program_counter as isize) = linedif as i8;
        (*function_state).functionstate_previousline = line;
    }
}
pub unsafe fn removelastlineinfo(function_state: *mut FunctionState) {
    unsafe {
        let prototype: *mut Prototype = (*function_state).functionstate_prototype;
        let program_counter: i32 = (*function_state).functionstate_programcounter - 1;
        if *((*prototype).prototype_lineinfo.vectort_pointer).offset(program_counter as isize) as i32 != -(0x80 as i32) {
            (*function_state).functionstate_previousline -=
                *((*prototype).prototype_lineinfo.vectort_pointer).offset(program_counter as isize) as i32;
            (*function_state).functionstate_iwthabs = ((*function_state).functionstate_iwthabs).wrapping_sub(1);
            (*function_state).functionstate_iwthabs;
        } else {
            (*function_state).functionstate_countabslineinfo -= 1;
            (*function_state).functionstate_countabslineinfo;
            (*function_state).functionstate_iwthabs = (128 as i32 + 1) as u8;
        };
    }
}
pub unsafe fn removelastinstruction(function_state: *mut FunctionState) {
    unsafe {
        removelastlineinfo(function_state);
        (*function_state).functionstate_programcounter -= 1;
        (*function_state).functionstate_programcounter;
    }
}
pub unsafe fn luak_code(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, i: u32,
) -> i32 {
    unsafe {
        let prototype: *mut Prototype = (*function_state).functionstate_prototype;
        (*prototype).prototype_code.grow(
            interpreter,
            (*function_state).functionstate_programcounter as usize,
            if 0x7FFFFFFF as usize <= (!0usize) / size_of::<u32>() {
                0x7FFFFFFF
            } else {
                (!0usize) / size_of::<u32>()
            },
            c"opcodes".as_ptr(),
        );
        let fresh134 = (*function_state).functionstate_programcounter;
        (*function_state).functionstate_programcounter = (*function_state).functionstate_programcounter + 1;
        *((*prototype).prototype_code.vectort_pointer).offset(fresh134 as isize) = i;
        savelineinfo(
            interpreter,
            lexical_state,
            function_state,
            prototype,
            (*lexical_state).lexicalstate_lastline,
        );
        return (*function_state).functionstate_programcounter - 1;
    }
}
pub unsafe fn code_abck(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, o: u32, a: i32, b: i32,
    c: i32, k: i32,
) -> i32 {
    unsafe {
        return luak_code(
            interpreter,
            lexical_state,
            function_state,
            (o as u32) << 0
                | (a as u32) << POSITION_A
                | (b as u32) << POSITION_B
                | (c as u32) << POSITION_C
                | (k as u32) << POSITION_K,
        );
    }
}
pub unsafe fn luak_codeabx(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, o: u32, a: i32, bc: u32,
) -> i32 {
    unsafe {
        return luak_code(
            interpreter,
            lexical_state,
            function_state,
            (o as u32) << 0 | (a as u32) << POSITION_A | bc << POSITION_K,
        );
    }
}
pub unsafe fn codeasbx(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, o: u32, a: i32, bc: i32,
) -> i32 {
    unsafe {
        let b: u32 = (bc + ((1 << 8 + 8 + 1) - 1 >> 1)) as u32;
        return luak_code(
            interpreter,
            lexical_state,
            function_state,
            (o as u32) << 0 | (a as u32) << POSITION_A | b << POSITION_K,
        );
    }
}
pub unsafe fn codesj(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, o: u32, sj: i32, k: i32,
) -> i32 {
    unsafe {
        let j = (sj + ((1 << 8 + 8 + 1 + 8) - 1 >> 1)) as u32;
        return luak_code(
            interpreter,
            lexical_state,
            function_state,
            o << 0 | j << POSITION_A | (k as u32) << POSITION_K,
        );
    }
}
pub unsafe fn codeextraarg(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, a: i32,
) -> i32 {
    unsafe {
        return luak_code(
            interpreter,
            lexical_state,
            function_state,
            OPCODE_EXTRAARG << 0 | (a as u32) << POSITION_A,
        );
    }
}
pub unsafe fn code_constant(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, reg: i32, k: i32,
) -> i32 {
    unsafe {
        if k <= (1 << 8 + 8 + 1) - 1 {
            return luak_codeabx(interpreter, lexical_state, function_state, OPCODE_LOADK, reg, k as u32);
        } else {
            let p = luak_codeabx(interpreter, lexical_state, function_state, OPCODE_LOADKX, reg, 0);
            codeextraarg(interpreter, lexical_state, function_state, k);
            return p;
        };
    }
}
pub unsafe fn luak_checkstack(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, n: i32,
) {
    unsafe {
        let new_stack = (*function_state).functionstate_freereg as i32 + n;
        if new_stack > (*(*function_state).functionstate_prototype).prototype_maximumstacksize as i32 {
            if new_stack >= 255 as i32 {
                luax_syntaxerror(
                    interpreter,
                    lexical_state,
                    c"function or expression needs too many registers".as_ptr(),
                );
            }
            (*(*function_state).functionstate_prototype).prototype_maximumstacksize = new_stack as u8;
        }
    }
}
pub unsafe fn luak_reserveregs(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, n: i32,
) {
    unsafe {
        luak_checkstack(interpreter, lexical_state, function_state, n);
        (*function_state).functionstate_freereg = ((*function_state).functionstate_freereg as i32 + n) as u8;
    }
}
pub unsafe fn freereg(lexical_state: *mut LexicalState, function_state: *mut FunctionState, reg: i32) {
    unsafe {
        if reg >= luay_nvarstack(lexical_state, function_state) {
            (*function_state).functionstate_freereg = ((*function_state).functionstate_freereg).wrapping_sub(1);
            (*function_state).functionstate_freereg;
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
    lexical_state: *mut LexicalState, function_state: *mut FunctionState, expression_description: *mut ExpressionDescription,
) {
    unsafe {
        if (*expression_description).expressiondescription_expressionkind == ExpressionKind::Nonrelocatable {
            freereg(
                lexical_state,
                function_state,
                (*expression_description).expressiondescription_value.value_info,
            );
        }
    }
}
pub unsafe fn freeexps(
    lexical_state: *mut LexicalState, function_state: *mut FunctionState, e1: *mut ExpressionDescription,
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
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, key: *mut TValue,
    v: *mut TValue,
) -> i32 {
    unsafe {
        let mut value: TValue = TValue::new(TagVariant::NilNil);
        let prototype: *mut Prototype = (*function_state).functionstate_prototype;
        let index: *const TValue = luah_get((*lexical_state).lexicalstate_table, key);
        let mut count_constants: i32;
        if (*index).get_tagvariant() == TagVariant::NumericInteger {
            count_constants = (*index).tvalue_value.value_integer as i32;
            if count_constants < (*function_state).functionstate_countconstants
                && (*((*prototype).prototype_constants.vectort_pointer).offset(count_constants as isize)).get_tagvariant()
                    == (*v).get_tagvariant()
                && luav_equalobj(
                    null_mut(),
                    &mut *((*prototype).prototype_constants.vectort_pointer).offset(count_constants as isize),
                    v,
                )
            {
                return count_constants;
            }
        }
        let mut oldsize = (*prototype).prototype_constants.get_size();
        count_constants = (*function_state).functionstate_countconstants;
        let io: *mut TValue = &mut value;
        (*io).tvalue_value.value_integer = count_constants as i64;
        (*io).tvalue_set_tag_variant(TagVariant::NumericInteger);
        luah_finishset(interpreter, (*lexical_state).lexicalstate_table, key, index, &mut value);
        (*prototype).prototype_constants.grow(
            interpreter,
            count_constants as usize,
            if ((1 << 8 + 8 + 1 + 8) - 1) as usize <= (!0usize) / size_of::<TValue>() {
                (1 << 8 + 8 + 1 + 8) - 1
            } else {
                (!0usize) / size_of::<TValue>()
            },
            c"constants".as_ptr(),
        );
        while oldsize < (*prototype).prototype_constants.get_size() {
            let fresh135 = oldsize;
            oldsize = oldsize + 1;
            (*((*prototype).prototype_constants.vectort_pointer).offset(fresh135 as isize))
                .tvalue_set_tag_variant(TagVariant::NilNil);
        }
        let io1: *mut TValue =
            &mut *((*prototype).prototype_constants.vectort_pointer).offset(count_constants as isize) as *mut TValue;
        let io2: *const TValue = v;
        (*io1).copy_from(&*io2);
        (*function_state).functionstate_countconstants += 1;
        (*function_state).functionstate_countconstants;
        if (*v).is_collectable() {
            if (*prototype).get_marked() & 1 << 5 != 0 && (*(*v).tvalue_value.value_object).get_marked() & (1 << 3 | 1 << 4) != 0 {
                Object::luac_barrier_(
                    interpreter,
                    &mut (*(prototype as *mut Object)),
                    &mut (*((*v).tvalue_value.value_object as *mut Object)),
                );
            } else {
            };
        } else {
        };
        return count_constants;
    }
}
pub unsafe fn string_constant(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, s: *mut TString,
) -> i32 {
    unsafe {
        let mut o: TValue = TValue::new(TagVariant::NilNil);
        let io: *mut TValue = &mut o;
        (*io).tvalue_value.value_object = &mut (*(s as *mut Object));
        (*io).tvalue_set_tag_variant((*s).get_tagvariant());
        (*io).set_collectable(true);
        return addk(interpreter, lexical_state, function_state, &mut o, &mut o);
    }
}
pub unsafe fn luak_int_k(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, integer: i64,
) -> i32 {
    unsafe {
        let mut tvalue: TValue = TValue::new(TagVariant::NilNil);
        tvalue.tvalue_value.value_integer = integer;
        tvalue.tvalue_set_tag_variant(TagVariant::NumericInteger);
        return addk(interpreter, lexical_state, function_state, &mut tvalue, &mut tvalue);
    }
}
pub unsafe fn luak_number_k(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, number: f64,
) -> i32 {
    unsafe {
        let mut tvalue: TValue = TValue::new(TagVariant::NilNil);
        let mut ik: i64 = 0;
        tvalue.tvalue_value.value_number = number;
        tvalue.tvalue_set_tag_variant(TagVariant::NumericNumber);
        if !F2I::Equal.luav_flttointeger(number, &mut ik) {
            return addk(interpreter, lexical_state, function_state, &mut tvalue, &mut tvalue);
        } else {
            let nbm: i32 = 53 as i32;
            let q: f64 = ldexp_(1.0f64, -nbm + 1);
            let k: f64 = if ik == 0 { q } else { number + number * q };
            let mut kv: TValue = TValue::new(TagVariant::NumericNumber);
            kv.tvalue_value.value_number = k;
            return addk(interpreter, lexical_state, function_state, &mut kv, &mut tvalue);
        };
    }
}
pub unsafe fn bool_false(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState,
) -> i32 {
    unsafe {
        let mut tvalue: TValue = TValue::new(TagVariant::BooleanFalse);
        return addk(interpreter, lexical_state, function_state, &mut tvalue, &mut tvalue);
    }
}
pub unsafe fn bool_true(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState,
) -> i32 {
    unsafe {
        let mut value: TValue = TValue::new(TagVariant::BooleanTrue);
        return addk(interpreter, lexical_state, function_state, &mut value, &mut value);
    }
}
pub unsafe fn nil_k(interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState) -> i32 {
    unsafe {
        let mut key: TValue = TValue::new(TagVariant::Table);
        let mut value: TValue = TValue::new(TagVariant::NilNil);
        let table: *mut Table = (*lexical_state).lexicalstate_table;
        key.tvalue_value.value_object = &mut (*(table as *mut Object));
        key.set_collectable(true);
        return addk(interpreter, lexical_state, function_state, &mut key, &mut value);
    }
}
pub unsafe fn code_constant_integer(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, reg: i32, i: i64,
) {
    unsafe {
        if fits_bx(i) {
            codeasbx(interpreter, lexical_state, function_state, OPCODE_LOADI, reg, i as i32);
        } else {
            code_constant(
                interpreter,
                lexical_state,
                function_state,
                reg,
                luak_int_k(interpreter, lexical_state, function_state, i),
            );
        };
    }
}
pub unsafe fn code_constant_number(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, reg: i32, number: f64,
) {
    unsafe {
        let mut fi: i64 = 0;
        if F2I::Equal.luav_flttointeger(number, &mut fi) && fits_bx(fi) {
            codeasbx(interpreter, lexical_state, function_state, OPCODE_LOADF, reg, fi as i32);
        } else {
            code_constant(
                interpreter,
                lexical_state,
                function_state,
                reg,
                luak_number_k(interpreter, lexical_state, function_state, number),
            );
        };
    }
}
pub unsafe fn luak_setreturns(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState,
    expression_description: *mut ExpressionDescription, count_results: i32,
) {
    unsafe {
        let program_counter: *mut u32 = &mut *((*(*function_state).functionstate_prototype).prototype_code.vectort_pointer)
            .offset((*expression_description).expressiondescription_value.value_info as isize)
            as *mut u32;
        if (*expression_description).expressiondescription_expressionkind == ExpressionKind::Call {
            *program_counter = *program_counter & !(!(0xFFFFFFFFu32 << 8) << POSITION_C)
                | ((count_results + 1) as u32) << POSITION_C & !(0xFFFFFFFFu32 << 8) << POSITION_C;
        } else {
            *program_counter = *program_counter & !(!(0xFFFFFFFFu32 << 8) << POSITION_C)
                | ((count_results + 1) as u32) << POSITION_C & !(0xFFFFFFFFu32 << 8) << POSITION_C;
            *program_counter = *program_counter & !(!(0xFFFFFFFFu32 << 8) << POSITION_A)
                | ((*function_state).functionstate_freereg as u32) << POSITION_A & !(0xFFFFFFFFu32 << 8) << POSITION_A;
            luak_reserveregs(interpreter, lexical_state, function_state, 1);
        };
    }
}
pub unsafe fn string_to_constant(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState,
    expression_description: *mut ExpressionDescription,
) {
    unsafe {
        (*expression_description).expressiondescription_value.value_info = string_constant(
            interpreter,
            lexical_state,
            function_state,
            (*expression_description).expressiondescription_value.value_tstring,
        );
        (*expression_description).expressiondescription_expressionkind = ExpressionKind::Constant;
    }
}
pub unsafe fn luak_setoneret(function_state: *mut FunctionState, expression_description: *mut ExpressionDescription) {
    unsafe {
        if (*expression_description).expressiondescription_expressionkind == ExpressionKind::Call {
            (*expression_description).expressiondescription_expressionkind = ExpressionKind::Nonrelocatable;
            (*expression_description).expressiondescription_value.value_info =
                (*((*(*function_state).functionstate_prototype).prototype_code.vectort_pointer)
                    .offset((*expression_description).expressiondescription_value.value_info as isize)
                    >> POSITION_A
                    & !(0xFFFFFFFFu32 << 8) << 0) as i32;
        } else if (*expression_description).expressiondescription_expressionkind == ExpressionKind::VariableArguments {
            *((*(*function_state).functionstate_prototype).prototype_code.vectort_pointer)
                .offset((*expression_description).expressiondescription_value.value_info as isize) =
                *((*(*function_state).functionstate_prototype).prototype_code.vectort_pointer)
                    .offset((*expression_description).expressiondescription_value.value_info as isize)
                    & !(!(0xFFFFFFFFu32 << 8) << POSITION_C)
                    | (2 as u32) << POSITION_C & !(0xFFFFFFFFu32 << 8) << POSITION_C;
            (*expression_description).expressiondescription_expressionkind = ExpressionKind::Relocatable;
        }
    }
}
pub unsafe fn luak_dischargevars(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState,
    expression_description: *mut ExpressionDescription,
) {
    unsafe {
        match (*expression_description).expressiondescription_expressionkind {
            | ExpressionKind::Constant2 => {
                ExpressionDescription::const2exp(
                    ExpressionDescription::const2val(lexical_state, function_state, expression_description),
                    expression_description,
                );
            },
            | ExpressionKind::Local => {
                let temporary = (*expression_description)
                    .expressiondescription_value
                    .value_variable
                    .valueregister_registerindex as i32;
                (*expression_description).expressiondescription_value.value_info = temporary;
                (*expression_description).expressiondescription_expressionkind = ExpressionKind::Nonrelocatable;
            },
            | ExpressionKind::UpValue => {
                (*expression_description).expressiondescription_value.value_info = code_abck(
                    interpreter,
                    lexical_state,
                    function_state,
                    OPCODE_GET_UPVALUE,
                    0,
                    (*expression_description).expressiondescription_value.value_info,
                    0,
                    0,
                );
                (*expression_description).expressiondescription_expressionkind = ExpressionKind::Relocatable;
            },
            | ExpressionKind::IndexUpValue => {
                (*expression_description).expressiondescription_value.value_info = code_abck(
                    interpreter,
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
            },
            | ExpressionKind::IndexInteger => {
                freereg(
                    lexical_state,
                    function_state,
                    (*expression_description)
                        .expressiondescription_value
                        .value_index
                        .valuereference_tag as i32,
                );
                (*expression_description).expressiondescription_value.value_info = code_abck(
                    interpreter,
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
            },
            | ExpressionKind::Field => {
                freereg(
                    lexical_state,
                    function_state,
                    (*expression_description)
                        .expressiondescription_value
                        .value_index
                        .valuereference_tag as i32,
                );
                (*expression_description).expressiondescription_value.value_info = code_abck(
                    interpreter,
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
            },
            | ExpressionKind::Indexed => {
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
                (*expression_description).expressiondescription_value.value_info = code_abck(
                    interpreter,
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
            },
            | ExpressionKind::Call => {
                luak_setoneret(function_state, expression_description);
            },
            | ExpressionKind::VariableArguments => {
                luak_setoneret(function_state, expression_description);
            },
            | _ => {},
        };
    }
}
pub unsafe fn discharge2reg(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState,
    expression_description: *mut ExpressionDescription, register: i32,
) {
    unsafe {
        luak_dischargevars(interpreter, lexical_state, function_state, expression_description);
        match (*expression_description).expressiondescription_expressionkind {
            | ExpressionKind::Nil => {
                code_constant_nil(interpreter, lexical_state, function_state, register, 1);
            },
            | ExpressionKind::False => {
                code_abck(interpreter, lexical_state, function_state, OPCODE_LOAD_FALSE, register, 0, 0, 0);
            },
            | ExpressionKind::True => {
                code_abck(interpreter, lexical_state, function_state, OPCODE_LOAD_TRUE, register, 0, 0, 0);
            },
            | ExpressionKind::ConstantString => {
                string_to_constant(interpreter, lexical_state, function_state, expression_description);
                code_constant(
                    interpreter,
                    lexical_state,
                    function_state,
                    register,
                    (*expression_description).expressiondescription_value.value_info,
                );
            },
            | ExpressionKind::Constant => {
                code_constant(
                    interpreter,
                    lexical_state,
                    function_state,
                    register,
                    (*expression_description).expressiondescription_value.value_info,
                );
            },
            | ExpressionKind::ConstantNumber => {
                code_constant_number(
                    interpreter,
                    lexical_state,
                    function_state,
                    register,
                    (*expression_description).expressiondescription_value.value_number,
                );
            },
            | ExpressionKind::ConstantInteger => {
                code_constant_integer(
                    interpreter,
                    lexical_state,
                    function_state,
                    register,
                    (*expression_description).expressiondescription_value.value_integer,
                );
            },
            | ExpressionKind::Relocatable => {
                let program_counter = &mut *((*(*function_state).functionstate_prototype).prototype_code.vectort_pointer)
                    .offset((*expression_description).expressiondescription_value.value_info as isize);
                *program_counter = *program_counter & !(!(0xFFFFFFFFu32 << 8) << POSITION_A)
                    | (register as u32) << POSITION_A & !(0xFFFFFFFFu32 << 8) << POSITION_A;
            },
            | ExpressionKind::Nonrelocatable => {
                if register != (*expression_description).expressiondescription_value.value_info {
                    code_abck(
                        interpreter,
                        lexical_state,
                        function_state,
                        OPCODE_MOVE,
                        register,
                        (*expression_description).expressiondescription_value.value_info,
                        0,
                        0,
                    );
                }
            },
            | _ => return,
        }
        (*expression_description).expressiondescription_value.value_info = register;
        (*expression_description).expressiondescription_expressionkind = ExpressionKind::Nonrelocatable;
    }
}
pub unsafe fn discharge2anyreg(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState,
    expression_description: *mut ExpressionDescription,
) {
    unsafe {
        if (*expression_description).expressiondescription_expressionkind != ExpressionKind::Nonrelocatable {
            luak_reserveregs(interpreter, lexical_state, function_state, 1);
            discharge2reg(
                interpreter,
                lexical_state,
                function_state,
                expression_description,
                (*function_state).functionstate_freereg as i32 - 1,
            );
        }
    }
}
pub unsafe fn code_loadbool(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, a: i32, op: u32,
) -> i32 {
    unsafe {
        (*function_state).code_get_label();
        return code_abck(interpreter, lexical_state, function_state, op, a, 0, 0, 0);
    }
}
pub unsafe fn need_value(function_state: *mut FunctionState, mut list: i32) -> i32 {
    unsafe {
        while list != -1 {
            let i: u32 = *code_get_jump_control(function_state, list);
            if (i >> 0 & !(0xFFFFFFFFu32 << 7) << 0) as u32 != OPCODE_TESTSET as u32 {
                return 1;
            }
            list = code_get_jump(function_state, list);
        }
        return 0;
    }
}
pub unsafe fn exp2reg(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState,
    expression_description: *mut ExpressionDescription, reg: i32,
) {
    unsafe {
        discharge2reg(interpreter, lexical_state, function_state, expression_description, reg);
        if (*expression_description).expressiondescription_expressionkind == ExpressionKind::Jump {
            luak_concat(
                interpreter,
                lexical_state,
                function_state,
                &mut (*expression_description).expressiondescription_t,
                (*expression_description).expressiondescription_value.value_info,
            );
        }
        if (*expression_description).expressiondescription_t != (*expression_description).expressiondescription_f {
            let mut p_f: i32 = -1;
            let mut p_t: i32 = -1;
            if need_value(function_state, (*expression_description).expressiondescription_t) != 0
                || need_value(function_state, (*expression_description).expressiondescription_f) != 0
            {
                let fj: i32 = if (*expression_description).expressiondescription_expressionkind == ExpressionKind::Jump {
                    -1
                } else {
                    luak_jump(interpreter, lexical_state, function_state)
                };
                p_f = code_loadbool(interpreter, lexical_state, function_state, reg, OPCODE_LFALSESKIP);
                p_t = code_loadbool(interpreter, lexical_state, function_state, reg, OPCODE_LOAD_TRUE);
                luak_patchtohere(interpreter, lexical_state, function_state, fj);
            }
            let final_0: i32 = (*function_state).code_get_label();
            patchlistaux(
                interpreter,
                lexical_state,
                function_state,
                (*expression_description).expressiondescription_f,
                final_0,
                reg,
                p_f,
            );
            patchlistaux(
                interpreter,
                lexical_state,
                function_state,
                (*expression_description).expressiondescription_t,
                final_0,
                reg,
                p_t,
            );
        }
        (*expression_description).expressiondescription_t = -1;
        (*expression_description).expressiondescription_f = (*expression_description).expressiondescription_t;
        (*expression_description).expressiondescription_value.value_info = reg;
        (*expression_description).expressiondescription_expressionkind = ExpressionKind::Nonrelocatable;
    }
}
pub unsafe fn luak_exp2nextreg(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState,
    expression_description: *mut ExpressionDescription,
) {
    unsafe {
        luak_dischargevars(interpreter, lexical_state, function_state, expression_description);
        freeexp(lexical_state, function_state, expression_description);
        luak_reserveregs(interpreter, lexical_state, function_state, 1);
        exp2reg(
            interpreter,
            lexical_state,
            function_state,
            expression_description,
            (*function_state).functionstate_freereg as i32 - 1,
        );
    }
}
pub unsafe fn luak_exp2anyreg(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState,
    expression_description: *mut ExpressionDescription,
) -> i64 {
    unsafe {
        luak_dischargevars(interpreter, lexical_state, function_state, expression_description);
        if (*expression_description).expressiondescription_expressionkind == ExpressionKind::Nonrelocatable {
            if !((*expression_description).expressiondescription_t != (*expression_description).expressiondescription_f) {
                return (*expression_description).expressiondescription_value.value_info as i64;
            }
            if (*expression_description).expressiondescription_value.value_info >= luay_nvarstack(lexical_state, function_state) {
                exp2reg(
                    interpreter,
                    lexical_state,
                    function_state,
                    expression_description,
                    (*expression_description).expressiondescription_value.value_info,
                );
                return (*expression_description).expressiondescription_value.value_info as i64;
            }
        }
        luak_exp2nextreg(interpreter, lexical_state, function_state, expression_description);
        return (*expression_description).expressiondescription_value.value_info as i64;
    }
}
pub unsafe fn luak_exp2anyregup(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState,
    expression_description: *mut ExpressionDescription,
) {
    unsafe {
        if (*expression_description).expressiondescription_expressionkind != ExpressionKind::UpValue
            || (*expression_description).expressiondescription_t != (*expression_description).expressiondescription_f
        {
            luak_exp2anyreg(interpreter, lexical_state, function_state, expression_description);
        }
    }
}
pub unsafe fn code_expression_to_value(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState,
    expression_description: *mut ExpressionDescription,
) {
    unsafe {
        if (*expression_description).expressiondescription_expressionkind == ExpressionKind::Jump
            || (*expression_description).expressiondescription_t != (*expression_description).expressiondescription_f
        {
            luak_exp2anyreg(interpreter, lexical_state, function_state, expression_description);
        } else {
            luak_dischargevars(interpreter, lexical_state, function_state, expression_description);
        };
    }
}
pub unsafe fn code_expression_to_constant(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState,
    expression_description: *mut ExpressionDescription,
) -> i32 {
    unsafe {
        if (*expression_description).expressiondescription_t == (*expression_description).expressiondescription_f {
            let info: i32;
            match (*expression_description).expressiondescription_expressionkind {
                | ExpressionKind::True => {
                    info = bool_true(interpreter, lexical_state, function_state);
                    if info <= (1 << 8) - 1 {
                        (*expression_description).expressiondescription_expressionkind = ExpressionKind::Constant;
                        (*expression_description).expressiondescription_value.value_info = info;
                        return 1;
                    } else {
                        return 0;
                    }
                },
                | ExpressionKind::False => {
                    info = bool_false(interpreter, lexical_state, function_state);
                    if info <= (1 << 8) - 1 {
                        (*expression_description).expressiondescription_expressionkind = ExpressionKind::Constant;
                        (*expression_description).expressiondescription_value.value_info = info;
                        return 1;
                    } else {
                        return 0;
                    }
                },
                | ExpressionKind::Nil => {
                    info = nil_k(interpreter, lexical_state, function_state);
                    if info <= (1 << 8) - 1 {
                        (*expression_description).expressiondescription_expressionkind = ExpressionKind::Constant;
                        (*expression_description).expressiondescription_value.value_info = info;
                        return 1;
                    } else {
                        return 0;
                    }
                },
                | ExpressionKind::ConstantInteger => {
                    info = luak_int_k(
                        interpreter,
                        lexical_state,
                        function_state,
                        (*expression_description).expressiondescription_value.value_integer,
                    );
                    if info <= (1 << 8) - 1 {
                        (*expression_description).expressiondescription_expressionkind = ExpressionKind::Constant;
                        (*expression_description).expressiondescription_value.value_info = info;
                        return 1;
                    } else {
                        return 0;
                    }
                },
                | ExpressionKind::ConstantNumber => {
                    info = luak_number_k(
                        interpreter,
                        lexical_state,
                        function_state,
                        (*expression_description).expressiondescription_value.value_number,
                    );
                    if info <= (1 << 8) - 1 {
                        (*expression_description).expressiondescription_expressionkind = ExpressionKind::Constant;
                        (*expression_description).expressiondescription_value.value_info = info;
                        return 1;
                    } else {
                        return 0;
                    }
                },
                | ExpressionKind::ConstantString => {
                    info = string_constant(
                        interpreter,
                        lexical_state,
                        function_state,
                        (*expression_description).expressiondescription_value.value_tstring,
                    );
                    if info <= (1 << 8) - 1 {
                        (*expression_description).expressiondescription_expressionkind = ExpressionKind::Constant;
                        (*expression_description).expressiondescription_value.value_info = info;
                        return 1;
                    } else {
                        return 0;
                    }
                },
                | ExpressionKind::Constant => {
                    info = (*expression_description).expressiondescription_value.value_info;
                    if info <= (1 << 8) - 1 {
                        (*expression_description).expressiondescription_expressionkind = ExpressionKind::Constant;
                        (*expression_description).expressiondescription_value.value_info = info;
                        return 1;
                    } else {
                        return 0;
                    }
                },
                | _ => {
                    return 0;
                },
            }
        } else {
            return 0;
        }
    }
}
pub unsafe fn exp2rk(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState,
    expression_description: *mut ExpressionDescription,
) -> i32 {
    unsafe {
        if code_expression_to_constant(interpreter, lexical_state, function_state, expression_description) != 0 {
            return 1;
        } else {
            luak_exp2anyreg(interpreter, lexical_state, function_state, expression_description);
            return 0;
        };
    }
}
pub unsafe fn codeabrk(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, o: u32, a: i32, b: i32,
    ec: *mut ExpressionDescription,
) {
    unsafe {
        let k: i32 = exp2rk(interpreter, lexical_state, function_state, ec);
        code_abck(
            interpreter,
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
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState,
    var: *mut ExpressionDescription, ex: *mut ExpressionDescription,
) {
    unsafe {
        match (*var).expressiondescription_expressionkind {
            | ExpressionKind::Local => {
                freeexp(lexical_state, function_state, ex);
                exp2reg(
                    interpreter,
                    lexical_state,
                    function_state,
                    ex,
                    (*var).expressiondescription_value.value_variable.valueregister_registerindex as i32,
                );
                return;
            },
            | ExpressionKind::UpValue => {
                let e = luak_exp2anyreg(interpreter, lexical_state, function_state, ex);
                code_abck(
                    interpreter,
                    lexical_state,
                    function_state,
                    OPCODE_SETUPVAL,
                    e as i32,
                    (*var).expressiondescription_value.value_info,
                    0,
                    0,
                );
            },
            | ExpressionKind::IndexUpValue => {
                codeabrk(
                    interpreter,
                    lexical_state,
                    function_state,
                    OPCODE_SETTABUP,
                    (*var).expressiondescription_value.value_index.valuereference_tag as i32,
                    (*var).expressiondescription_value.value_index.valuereference_index as i32,
                    ex,
                );
            },
            | ExpressionKind::IndexInteger => {
                codeabrk(
                    interpreter,
                    lexical_state,
                    function_state,
                    OPCODE_SETI,
                    (*var).expressiondescription_value.value_index.valuereference_tag as i32,
                    (*var).expressiondescription_value.value_index.valuereference_index as i32,
                    ex,
                );
            },
            | ExpressionKind::Field => {
                codeabrk(
                    interpreter,
                    lexical_state,
                    function_state,
                    OPCODE_SETFIELD,
                    (*var).expressiondescription_value.value_index.valuereference_tag as i32,
                    (*var).expressiondescription_value.value_index.valuereference_index as i32,
                    ex,
                );
            },
            | ExpressionKind::Indexed => {
                codeabrk(
                    interpreter,
                    lexical_state,
                    function_state,
                    OPCODE_SETTABLE,
                    (*var).expressiondescription_value.value_index.valuereference_tag as i32,
                    (*var).expressiondescription_value.value_index.valuereference_index as i32,
                    ex,
                );
            },
            | _ => {},
        }
        freeexp(lexical_state, function_state, ex);
    }
}
pub unsafe fn luak_self(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState,
    expression_description: *mut ExpressionDescription, key: *mut ExpressionDescription,
) {
    unsafe {
        luak_exp2anyreg(interpreter, lexical_state, function_state, expression_description);
        let ereg: i32 = (*expression_description).expressiondescription_value.value_info;
        freeexp(lexical_state, function_state, expression_description);
        (*expression_description).expressiondescription_value.value_info = (*function_state).functionstate_freereg as i32;
        (*expression_description).expressiondescription_expressionkind = ExpressionKind::Nonrelocatable;
        luak_reserveregs(interpreter, lexical_state, function_state, 2);
        codeabrk(
            interpreter,
            lexical_state,
            function_state,
            OPCODE_SELF,
            (*expression_description).expressiondescription_value.value_info,
            ereg,
            key,
        );
        freeexp(lexical_state, function_state, key);
    }
}
pub unsafe fn negatecondition(function_state: *mut FunctionState, expression_description: *mut ExpressionDescription) {
    unsafe {
        let program_counter: *mut u32 =
            code_get_jump_control(function_state, (*expression_description).expressiondescription_value.value_info);
        *program_counter = *program_counter & !(!(0xFFFFFFFFu32 << 1) << POSITION_K)
            | (((*program_counter >> POSITION_K & !(0xFFFFFFFFu32 << 1) << 0) as i32 ^ 1) as u32) << POSITION_K
                & !(0xFFFFFFFFu32 << 1) << POSITION_K;
    }
}
pub unsafe fn jumponcond(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState,
    expression_description: *mut ExpressionDescription, cond_0: i32,
) -> i32 {
    unsafe {
        if (*expression_description).expressiondescription_expressionkind == ExpressionKind::Relocatable {
            let ie: u32 = *((*(*function_state).functionstate_prototype).prototype_code.vectort_pointer)
                .offset((*expression_description).expressiondescription_value.value_info as isize);
            if (ie >> 0 & !(0xFFFFFFFFu32 << 7) << 0) as u32 == OPCODE_NOT as u32 {
                removelastinstruction(function_state);
                return condjump(
                    interpreter,
                    lexical_state,
                    function_state,
                    OPCODE_TEST,
                    (ie >> POSITION_B & !(0xFFFFFFFFu32 << 8) << 0) as i32,
                    0,
                    0,
                    (cond_0 == 0) as i32,
                );
            }
        }
        discharge2anyreg(interpreter, lexical_state, function_state, expression_description);
        freeexp(lexical_state, function_state, expression_description);
        return condjump(
            interpreter,
            lexical_state,
            function_state,
            OPCODE_TESTSET,
            (1 << 8) - 1,
            (*expression_description).expressiondescription_value.value_info,
            0,
            cond_0,
        );
    }
}
pub unsafe fn luak_goiftrue(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState,
    expression_description: *mut ExpressionDescription,
) {
    unsafe {
        let program_counter: i32;
        luak_dischargevars(interpreter, lexical_state, function_state, expression_description);
        match (*expression_description).expressiondescription_expressionkind {
            | ExpressionKind::Jump => {
                negatecondition(function_state, expression_description);
                program_counter = (*expression_description).expressiondescription_value.value_info;
            },
            | ExpressionKind::True
            | ExpressionKind::Constant
            | ExpressionKind::ConstantNumber
            | ExpressionKind::ConstantInteger
            | ExpressionKind::ConstantString => {
                program_counter = -1;
            },
            | _ => {
                program_counter = jumponcond(interpreter, lexical_state, function_state, expression_description, 0);
            },
        }
        luak_concat(
            interpreter,
            lexical_state,
            function_state,
            &mut (*expression_description).expressiondescription_f,
            program_counter,
        );
        luak_patchtohere(
            interpreter,
            lexical_state,
            function_state,
            (*expression_description).expressiondescription_t,
        );
        (*expression_description).expressiondescription_t = -1;
    }
}
pub unsafe fn luak_goiffalse(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState,
    expression_description: *mut ExpressionDescription,
) {
    unsafe {
        let program_counter: i32;
        luak_dischargevars(interpreter, lexical_state, function_state, expression_description);
        match (*expression_description).expressiondescription_expressionkind {
            | ExpressionKind::Jump => {
                program_counter = (*expression_description).expressiondescription_value.value_info;
            },
            | ExpressionKind::Nil | ExpressionKind::False => {
                program_counter = -1;
            },
            | _ => {
                program_counter = jumponcond(interpreter, lexical_state, function_state, expression_description, 1);
            },
        }
        luak_concat(
            interpreter,
            lexical_state,
            function_state,
            &mut (*expression_description).expressiondescription_t,
            program_counter,
        );
        luak_patchtohere(
            interpreter,
            lexical_state,
            function_state,
            (*expression_description).expressiondescription_f,
        );
        (*expression_description).expressiondescription_f = -1;
    }
}
pub unsafe fn codenot(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState,
    expression_description: *mut ExpressionDescription,
) {
    unsafe {
        match (*expression_description).expressiondescription_expressionkind {
            | ExpressionKind::Nil | ExpressionKind::False => {
                (*expression_description).expressiondescription_expressionkind = ExpressionKind::True;
            },
            | ExpressionKind::Constant
            | ExpressionKind::ConstantNumber
            | ExpressionKind::ConstantInteger
            | ExpressionKind::ConstantString
            | ExpressionKind::True => {
                (*expression_description).expressiondescription_expressionkind = ExpressionKind::False;
            },
            | ExpressionKind::Jump => {
                negatecondition(function_state, expression_description);
            },
            | ExpressionKind::Relocatable | ExpressionKind::Nonrelocatable => {
                discharge2anyreg(interpreter, lexical_state, function_state, expression_description);
                freeexp(lexical_state, function_state, expression_description);
                (*expression_description).expressiondescription_value.value_info = code_abck(
                    interpreter,
                    lexical_state,
                    function_state,
                    OPCODE_NOT,
                    0,
                    (*expression_description).expressiondescription_value.value_info,
                    0,
                    0,
                );
                (*expression_description).expressiondescription_expressionkind = ExpressionKind::Relocatable;
            },
            | _ => {},
        }
        let temp: i32 = (*expression_description).expressiondescription_f;
        (*expression_description).expressiondescription_f = (*expression_description).expressiondescription_t;
        (*expression_description).expressiondescription_t = temp;
        removevalues(function_state, (*expression_description).expressiondescription_f);
        removevalues(function_state, (*expression_description).expressiondescription_t);
    }
}
pub unsafe fn is_k_string(function_state: *mut FunctionState, expression_description: *mut ExpressionDescription) -> bool {
    unsafe {
        return (*expression_description).expressiondescription_expressionkind == ExpressionKind::Constant
            && !((*expression_description).expressiondescription_t != (*expression_description).expressiondescription_f)
            && (*expression_description).expressiondescription_value.value_info <= ((1 << 8) - 1)
            && (*((*(*function_state).functionstate_prototype).prototype_constants.vectort_pointer)
                .offset((*expression_description).expressiondescription_value.value_info as isize))
            .get_tagvariant()
                == TagVariant::StringShort;
    }
}
pub unsafe fn constfolding(
    interpreter: *mut Interpreter, _lexical_state: *mut LexicalState, _function_state: *mut FunctionState, op: i32,
    e1: *mut ExpressionDescription, e2: *const ExpressionDescription,
) -> i32 {
    unsafe {
        let mut v1: TValue = TValue::new(TagVariant::NilNil);
        let mut v2: TValue = TValue::new(TagVariant::NilNil);
        let mut res: TValue = TValue::new(TagVariant::NilNil);
        if !ExpressionDescription::tonumeral(e1, &mut v1) || !ExpressionDescription::tonumeral(e2, &mut v2) || validop(op, &mut v1, &mut v2) == 0 {
            return 0;
        }
        luao_rawarith(interpreter, op, &mut v1, &mut v2, &mut res);
        if res.get_tagvariant() == TagVariant::NumericInteger {
            (*e1).expressiondescription_expressionkind = ExpressionKind::ConstantInteger;
            (*e1).expressiondescription_value.value_integer = res.tvalue_value.value_integer;
        } else {
            let n: f64 = res.tvalue_value.value_number;
            if !(n == n) || n == 0.0 {
                return 0;
            }
            (*e1).expressiondescription_expressionkind = ExpressionKind::ConstantNumber;
            (*e1).expressiondescription_value.value_number = n;
        }
        return 1;
    }
}
pub unsafe fn code_unary_expression_value(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, op: u32,
    expression_description: *mut ExpressionDescription, line: i32,
) {
    unsafe {
        let register = luak_exp2anyreg(interpreter, lexical_state, function_state, expression_description);
        freeexp(lexical_state, function_state, expression_description);
        (*expression_description).expressiondescription_value.value_info =
            code_abck(interpreter, lexical_state, function_state, op, 0, register as i32, 0, 0);
        (*expression_description).expressiondescription_expressionkind = ExpressionKind::Relocatable;
        luak_fixline(interpreter, lexical_state, function_state, line);
    }
}
pub unsafe fn finishbinexpval(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState,
    e1: *mut ExpressionDescription, e2: *mut ExpressionDescription, op: u32, v2: i32, flip: i32, line: i32, mmop: u32, event: u32,
) {
    unsafe {
        let v1 = luak_exp2anyreg(interpreter, lexical_state, function_state, e1);
        let program_counter: i32 = code_abck(interpreter, lexical_state, function_state, op, 0, v1 as i32, v2, 0);
        freeexps(lexical_state, function_state, e1, e2);
        (*e1).expressiondescription_value.value_info = program_counter;
        (*e1).expressiondescription_expressionkind = ExpressionKind::Relocatable;
        luak_fixline(interpreter, lexical_state, function_state, line);
        code_abck(
            interpreter, lexical_state, function_state, mmop, v1 as i32, v2, event as i32, flip,
        );
        luak_fixline(interpreter, lexical_state, function_state, line);
    }
}
pub unsafe fn codebinexpval(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, binary: OperatorBinary,
    e1: *mut ExpressionDescription, e2: *mut ExpressionDescription, line: i32,
) {
    unsafe {
        let op = binopr2op(binary, OperatorBinary::Add, OPCODE_ADD);
        let v2 = luak_exp2anyreg(interpreter, lexical_state, function_state, e2);
        finishbinexpval(
            interpreter,
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
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, op: u32,
    e1: *mut ExpressionDescription, e2: *mut ExpressionDescription, flip: i32, line: i32, event: u32,
) {
    unsafe {
        let v2: i32 = (*e2).expressiondescription_value.value_integer as i32 + ((1 << 8) - 1 >> 1);
        finishbinexpval(
            interpreter, lexical_state, function_state, e1, e2, op, v2, flip, line, OPCODE_MMBINI, event,
        );
    }
}
pub unsafe fn codebink(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, opr: OperatorBinary,
    e1: *mut ExpressionDescription, e2: *mut ExpressionDescription, flip: i32, line: i32,
) {
    unsafe {
        let event: u32 = binopr2tm(opr);
        let v2: i32 = (*e2).expressiondescription_value.value_info;
        let op: u32 = binopr2op(opr, OperatorBinary::Add, OPCODE_ADDK);
        finishbinexpval(
            interpreter, lexical_state, function_state, e1, e2, op, v2, flip, line, OPCODE_MMBINK, event,
        );
    }
}
pub unsafe fn finishbinexpneg(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState,
    e1: *mut ExpressionDescription, e2: *mut ExpressionDescription, op: u32, line: i32, event: u32,
) -> i32 {
    unsafe {
        if !ExpressionDescription::is_k_int(e2) {
            return 0;
        } else {
            let i2: i64 = (*e2).expressiondescription_value.value_integer;
            if !(fits_c(i2) && fits_c(-i2)) {
                return 0;
            } else {
                let v2: i32 = i2 as i32;
                finishbinexpval(
                    interpreter,
                    lexical_state,
                    function_state,
                    e1,
                    e2,
                    op,
                    -v2 + ((1 << 8) - 1 >> 1),
                    0,
                    line,
                    OPCODE_MMBINI,
                    event,
                );
                *((*(*function_state).functionstate_prototype).prototype_code.vectort_pointer)
                    .offset(((*function_state).functionstate_programcounter - 1) as isize) =
                    *((*(*function_state).functionstate_prototype).prototype_code.vectort_pointer)
                        .offset(((*function_state).functionstate_programcounter - 1) as isize)
                        & !(!(0xFFFFFFFFu32 << 8) << POSITION_B)
                        | ((v2 + ((1 << 8) - 1 >> 1)) as u32) << POSITION_B & !(0xFFFFFFFFu32 << 8) << POSITION_B;
                return 1;
            }
        };
    }
}
pub unsafe fn codebinnok(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, opr: OperatorBinary,
    e1: *mut ExpressionDescription, e2: *mut ExpressionDescription, flip: i32, line: i32,
) {
    unsafe {
        if flip != 0 {
            ExpressionDescription::swapexps(e1, e2);
        }
        codebinexpval(interpreter, lexical_state, function_state, opr, e1, e2, line);
    }
}
pub unsafe fn codearith(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, opr: OperatorBinary,
    e1: *mut ExpressionDescription, e2: *mut ExpressionDescription, flip: i32, line: i32,
) {
    unsafe {
        if ExpressionDescription::tonumeral(e2, null_mut()) && code_expression_to_constant(interpreter, lexical_state, function_state, e2) != 0 {
            codebink(interpreter, lexical_state, function_state, opr, e1, e2, flip, line);
        } else {
            codebinnok(interpreter, lexical_state, function_state, opr, e1, e2, flip, line);
        };
    }
}
pub unsafe fn codebitwise(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, opr: OperatorBinary,
    e1: *mut ExpressionDescription, e2: *mut ExpressionDescription, line: i32,
) {
    unsafe {
        let mut flip: i32 = 0;
        if (*e1).expressiondescription_expressionkind == ExpressionKind::ConstantInteger {
            ExpressionDescription::swapexps(e1, e2);
            flip = 1;
        }
        if (*e2).expressiondescription_expressionkind == ExpressionKind::ConstantInteger
            && code_expression_to_constant(interpreter, lexical_state, function_state, e2) != 0
        {
            codebink(interpreter, lexical_state, function_state, opr, e1, e2, flip, line);
        } else {
            codebinnok(interpreter, lexical_state, function_state, opr, e1, e2, flip, line);
        };
    }
}
pub unsafe fn codeorder(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, opr: OperatorBinary,
    e1: *mut ExpressionDescription, e2: *mut ExpressionDescription,
) {
    unsafe {
        let mut im: i64 = 0;
        let mut is_float = false;
        let r1: i64;
        let r2: i64;
        let op: u32;
        if ExpressionDescription::is_sc_number(e2, &mut im, &mut is_float) != 0 {
            r1 = luak_exp2anyreg(interpreter, lexical_state, function_state, e1) as i64;
            r2 = im;
            op = binopr2op(opr, OperatorBinary::Less, OPCODE_LTI);
        } else if ExpressionDescription::is_sc_number(e1, &mut im, &mut is_float) != 0 {
            r1 = luak_exp2anyreg(interpreter, lexical_state, function_state, e2) as i64;
            r2 = im;
            op = binopr2op(opr, OperatorBinary::Less, OPCODE_GTI);
        } else {
            r1 = luak_exp2anyreg(interpreter, lexical_state, function_state, e1) as i64;
            r2 = luak_exp2anyreg(interpreter, lexical_state, function_state, e2) as i64;
            op = binopr2op(opr, OperatorBinary::Less, OPCODE_LT);
        }
        freeexps(lexical_state, function_state, e1, e2);
        (*e1).expressiondescription_value.value_info = condjump(
            interpreter, lexical_state, function_state, op, r1 as i32, r2 as i32, is_float as i32, 1,
        );
        (*e1).expressiondescription_expressionkind = ExpressionKind::Jump;
    }
}
pub unsafe fn codeeq(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, opr: OperatorBinary,
    e1: *mut ExpressionDescription, e2: *mut ExpressionDescription,
) {
    unsafe {
        let mut im: i64 = 0;
        let mut is_float = false;
        let r1: i64;
        let r2: i64;
        let op: u32;
        if (*e1).expressiondescription_expressionkind != ExpressionKind::Nonrelocatable {
            ExpressionDescription::swapexps(e1, e2);
        }
        r1 = luak_exp2anyreg(interpreter, lexical_state, function_state, e1) as i64;
        if ExpressionDescription::is_sc_number(e2, &mut im, &mut is_float) != 0 {
            op = OPCODE_EQI;
            r2 = im;
        } else if exp2rk(interpreter, lexical_state, function_state, e2) != 0 {
            op = OPCODE_EQK;
            r2 = (*e2).expressiondescription_value.value_info as i64;
        } else {
            op = OPCODE_EQ;
            r2 = luak_exp2anyreg(interpreter, lexical_state, function_state, e2) as i64;
        }
        freeexps(lexical_state, function_state, e1, e2);
        (*e1).expressiondescription_value.value_info = condjump(
            interpreter,
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
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, unary: OperatorUnary,
    expression_description: *mut ExpressionDescription, line: i32,
) {
    unsafe {
        pub const EF: ExpressionDescription = ExpressionDescription::new_from_integer(0);
        luak_dischargevars(interpreter, lexical_state, function_state, expression_description);
        match unary {
            | OperatorUnary::BitwiseNot => {
                if constfolding(
                    interpreter,
                    lexical_state,
                    function_state,
                    (unary as u32).wrapping_add(12 as u32) as i32,
                    expression_description,
                    &EF,
                ) == 0
                {
                    code_unary_expression_value(
                        interpreter,
                        lexical_state,
                        function_state,
                        unopr2op(unary),
                        expression_description,
                        line,
                    );
                }
            },
            | OperatorUnary::Minus => {
                if constfolding(
                    interpreter,
                    lexical_state,
                    function_state,
                    (unary as u32).wrapping_add(12 as u32) as i32,
                    expression_description,
                    &EF,
                ) == 0
                {
                    code_unary_expression_value(
                        interpreter,
                        lexical_state,
                        function_state,
                        unopr2op(unary),
                        expression_description,
                        line,
                    );
                }
            },
            | OperatorUnary::Length => {
                code_unary_expression_value(
                    interpreter,
                    lexical_state,
                    function_state,
                    unopr2op(unary),
                    expression_description,
                    line,
                );
            },
            | OperatorUnary::Not => {
                codenot(interpreter, lexical_state, function_state, expression_description);
            },
            | _ => {},
        }
    }
}
pub unsafe fn luak_infix(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, op: OperatorBinary,
    v: *mut ExpressionDescription,
) {
    unsafe {
        luak_dischargevars(interpreter, lexical_state, function_state, v);
        match op as u32 {
            | OPCODE_NEWTABLE => {
                luak_goiftrue(interpreter, lexical_state, function_state, v);
            },
            | OPCODE_SELF => {
                luak_goiffalse(interpreter, lexical_state, function_state, v);
            },
            | OPCODE_GET_TABLE => {
                luak_exp2nextreg(interpreter, lexical_state, function_state, v);
            },
            | OPCODE_MOVE
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
                    luak_exp2anyreg(interpreter, lexical_state, function_state, v);
                }
            },
            | OPCODE_INDEX_INTEGER | OPCODE_SETTABLE => {
                if !ExpressionDescription::tonumeral(v, null_mut()) {
                    exp2rk(interpreter, lexical_state, function_state, v);
                }
            },
            | OPCODE_GET_FIELD | OPCODE_SETTABUP | OPCODE_SETI | OPCODE_SETFIELD => {
                let mut dummy1: i64 = 0;
                let mut dummy2 = false;
                if ExpressionDescription::is_sc_number(v, &mut dummy1, &mut dummy2) == 0 {
                    luak_exp2anyreg(interpreter, lexical_state, function_state, v);
                }
            },
            | _ => {},
        };
    }
}
pub unsafe fn codeconcat(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState,
    e1: *mut ExpressionDescription, e2: *mut ExpressionDescription, line: i32,
) {
    unsafe {
        let ie2 = previousinstruction(function_state);
        if (*ie2 >> 0 & !(0xFFFFFFFFu32 << 7) << 0) as u32 == OPCODE_CONCAT as u32 {
            let n: i32 = (*ie2 >> POSITION_B & !(0xFFFFFFFFu32 << 8) << 0) as i32;
            freeexp(lexical_state, function_state, e2);
            *ie2 = *ie2 & !(!(0xFFFFFFFFu32 << 8) << POSITION_A)
                | ((*e1).expressiondescription_value.value_info as u32) << POSITION_A & !(0xFFFFFFFFu32 << 8) << POSITION_A;
            *ie2 = *ie2 & !(!(0xFFFFFFFFu32 << 8) << POSITION_B) | ((n + 1) as u32) << POSITION_B & !(0xFFFFFFFFu32 << 8) << POSITION_B;
        } else {
            code_abck(
                interpreter,
                lexical_state,
                function_state,
                OPCODE_CONCAT,
                (*e1).expressiondescription_value.value_info,
                2,
                0,
                0,
            );
            freeexp(lexical_state, function_state, e2);
            luak_fixline(interpreter, lexical_state, function_state, line);
        };
    }
}
pub unsafe fn luak_posfix(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState,
    mut binary: OperatorBinary, expression_description_a: *mut ExpressionDescription,
    expression_description_b: *mut ExpressionDescription, line: i32,
) {
    unsafe {
        luak_dischargevars(interpreter, lexical_state, function_state, expression_description_b);
        if binary as u32 <= OperatorBinary::ShiftRight as u32
            && constfolding(
                interpreter, lexical_state, function_state, binary as i32, expression_description_a, expression_description_b,
            ) != 0
        {
            return;
        }
        match binary {
            | OperatorBinary::And => {
                luak_concat(
                    interpreter,
                    lexical_state,
                    function_state,
                    &mut (*expression_description_b).expressiondescription_f,
                    (*expression_description_a).expressiondescription_f,
                );
                *expression_description_a = *expression_description_b;
            },
            | OperatorBinary::Or => {
                luak_concat(
                    interpreter,
                    lexical_state,
                    function_state,
                    &mut (*expression_description_b).expressiondescription_t,
                    (*expression_description_a).expressiondescription_t,
                );
                *expression_description_a = *expression_description_b;
            },
            | OperatorBinary::Concatenate => {
                luak_exp2nextreg(interpreter, lexical_state, function_state, expression_description_b);
                codeconcat(
                    interpreter, lexical_state, function_state, expression_description_a, expression_description_b, line,
                );
            },
            | OperatorBinary::Add => {
                let mut flip: i32 = 0;
                if ExpressionDescription::tonumeral(expression_description_a, null_mut()) {
                    ExpressionDescription::swapexps(expression_description_a, expression_description_b);
                    flip = 1;
                }
                if ExpressionDescription::is_sc_int(expression_description_b) {
                    codebini(
                        interpreter, lexical_state, function_state, OPCODE_ADDI, expression_description_a,
                        expression_description_b, flip, line, TM_ADD,
                    );
                } else {
                    codearith(
                        interpreter, lexical_state, function_state, binary, expression_description_a, expression_description_b,
                        flip, line,
                    );
                };
            },
            | OperatorBinary::Multiply => {
                let mut flip: i32 = 0;
                if ExpressionDescription::tonumeral(expression_description_a, null_mut()) {
                    ExpressionDescription::swapexps(expression_description_a, expression_description_b);
                    flip = 1;
                }
                codearith(
                    interpreter, lexical_state, function_state, binary, expression_description_a, expression_description_b, flip,
                    line,
                );
            },
            | OperatorBinary::Subtract => {
                if finishbinexpneg(
                    interpreter, lexical_state, function_state, expression_description_a, expression_description_b, OPCODE_ADDI,
                    line, TM_SUB,
                ) == 0
                {
                    codearith(
                        interpreter, lexical_state, function_state, binary, expression_description_a, expression_description_b, 0,
                        line,
                    );
                }
            },
            | OperatorBinary::Power => {
                codearith(
                    interpreter, lexical_state, function_state, binary, expression_description_a, expression_description_b, 0, line,
                );
            },
            | OperatorBinary::Modulus => {
                codearith(
                    interpreter, lexical_state, function_state, binary, expression_description_a, expression_description_b, 0, line,
                );
            },
            | OperatorBinary::Divide => {
                codearith(
                    interpreter, lexical_state, function_state, binary, expression_description_a, expression_description_b, 0, line,
                );
            },
            | OperatorBinary::IntegralDivide => {
                codearith(
                    interpreter, lexical_state, function_state, binary, expression_description_a, expression_description_b, 0, line,
                );
            },
            | OperatorBinary::BitwiseAnd => {
                codebitwise(
                    interpreter, lexical_state, function_state, binary, expression_description_a, expression_description_b, line,
                );
            },
            | OperatorBinary::BitwiseOr => {
                codebitwise(
                    interpreter, lexical_state, function_state, binary, expression_description_a, expression_description_b, line,
                );
            },
            | OperatorBinary::BitwiseExclusiveOr => {
                codebitwise(
                    interpreter, lexical_state, function_state, binary, expression_description_a, expression_description_b, line,
                );
            },
            | OperatorBinary::ShiftLeft => {
                if ExpressionDescription::is_sc_int(expression_description_a) {
                    ExpressionDescription::swapexps(expression_description_a, expression_description_b);
                    codebini(
                        interpreter, lexical_state, function_state, OPCODE_SHLI, expression_description_a,
                        expression_description_b, 1, line, TM_SHL,
                    );
                } else if !(finishbinexpneg(
                    interpreter, lexical_state, function_state, expression_description_a, expression_description_b, OPCODE_SHRI,
                    line, TM_SHL,
                ) != 0)
                {
                    codebinexpval(
                        interpreter, lexical_state, function_state, binary, expression_description_a, expression_description_b,
                        line,
                    );
                }
            },
            | OperatorBinary::ShiftRight => {
                if ExpressionDescription::is_sc_int(expression_description_b) {
                    codebini(
                        interpreter, lexical_state, function_state, OPCODE_SHRI, expression_description_a,
                        expression_description_b, 0, line, TM_SHR,
                    );
                } else {
                    codebinexpval(
                        interpreter, lexical_state, function_state, binary, expression_description_a, expression_description_b,
                        line,
                    );
                }
            },
            | OperatorBinary::Inequal => {
                codeeq(
                    interpreter, lexical_state, function_state, binary, expression_description_a, expression_description_b,
                );
            },
            | OperatorBinary::Equal => {
                codeeq(
                    interpreter, lexical_state, function_state, binary, expression_description_a, expression_description_b,
                );
            },
            | OperatorBinary::GreaterEqual => {
                ExpressionDescription::swapexps(expression_description_a, expression_description_b);
                binary = OperatorBinary::LessEqual;
                codeorder(
                    interpreter, lexical_state, function_state, binary, expression_description_a, expression_description_b,
                );
            },
            | OperatorBinary::Greater => {
                ExpressionDescription::swapexps(expression_description_a, expression_description_b);
                binary = OperatorBinary::Less;
                codeorder(
                    interpreter, lexical_state, function_state, binary, expression_description_a, expression_description_b,
                );
            },
            | OperatorBinary::Less => {
                codeorder(
                    interpreter, lexical_state, function_state, binary, expression_description_a, expression_description_b,
                );
            },
            | OperatorBinary::LessEqual => {
                codeorder(
                    interpreter, lexical_state, function_state, binary, expression_description_a, expression_description_b,
                );
            },
            | _ => {},
        }
    }
}
pub unsafe fn luak_fixline(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, line: i32,
) {
    unsafe {
        removelastlineinfo(function_state);
        savelineinfo(
            interpreter,
            lexical_state,
            function_state,
            (*function_state).functionstate_prototype,
            line,
        );
    }
}
pub const POSITION_A: usize = 7;
pub const POSITION_K: usize = POSITION_A + 8;
pub const POSITION_B: usize = POSITION_K + 1;
pub const POSITION_C: usize = POSITION_B + 8;
pub unsafe fn luak_settablesize(
    _interpreter: *mut Interpreter, function_state: *mut FunctionState, program_counter: i32, ra: i32, asize: i32, hsize: i32,
) {
    unsafe {
        let inst: *mut u32 = &mut *((*(*function_state).functionstate_prototype).prototype_code.vectort_pointer)
            .offset(program_counter as isize) as *mut u32;
        let rb: i32 = if hsize == 0 { 0 } else { 1 + hsize.ilog2() as i32 };
        let extra: i32 = asize / ((1 << 8) - 1 + 1);
        let rc: i32 = asize % ((1 << 8) - 1 + 1);
        let k: i32 = (extra > 0) as i32;
        *inst = (OPCODE_NEWTABLE as u32) << 0
            | (ra as u32) << POSITION_A
            | (rb as u32) << POSITION_B
            | (rc as u32) << POSITION_C
            | (k as u32) << POSITION_K;
        *inst.offset(1 as isize) = (OPCODE_EXTRAARG as u32) << 0 | (extra as u32) << POSITION_A;
    }
}
pub unsafe fn luak_setlist(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, base: i32,
    mut count_elements: usize, mut tostore: i32,
) {
    unsafe {
        if tostore == -1 {
            tostore = 0;
        }
        if count_elements <= (1 << 8) - 1 {
            code_abck(
                interpreter, lexical_state, function_state, OPCODE_SETLIST, base, tostore, count_elements as i32, 0,
            );
        } else {
            let extra = count_elements / ((1 << 8) - 1 + 1);
            count_elements %= (1 << 8) - 1 + 1;
            code_abck(
                interpreter, lexical_state, function_state, OPCODE_SETLIST, base, tostore, count_elements as i32, 1,
            );
            codeextraarg(interpreter, lexical_state, function_state, extra as i32);
        }
        (*function_state).functionstate_freereg = (base + 1) as u8;
    }
}
pub unsafe fn luak_finish(
    interpreter: *mut Interpreter, lexical_state: *mut LexicalState, function_state: *mut FunctionState, prototype: *mut Prototype,
) {
    unsafe {
        for i in 0..(*function_state).functionstate_programcounter {
            let program_counter: *mut u32 = &mut *((*prototype).prototype_code.vectort_pointer).offset(i as isize) as *mut u32;
            match (*program_counter >> 0 & !(0xFFFFFFFFu32 << 7) << 0) as u32 {
                | OPCODE_RETURN0 | OPCODE_RETURN1 => {
                    if (*function_state).functionstate_needsclose || (*prototype).prototype_isvariablearguments {
                        *program_counter =
                            *program_counter & !(!(0xFFFFFFFFu32 << 7) << 0) | (OPCODE_RETURN as u32) << 0 & !(0xFFFFFFFFu32 << 7) << 0;
                        if (*function_state).functionstate_needsclose {
                            *program_counter = *program_counter & !(!(0xFFFFFFFFu32 << 1) << POSITION_K)
                                | (1 as u32) << POSITION_K & !(0xFFFFFFFFu32 << 1) << POSITION_K;
                        }
                        if (*prototype).prototype_isvariablearguments {
                            *program_counter = *program_counter & !(!(0xFFFFFFFFu32 << 8) << POSITION_C)
                                | (((*prototype).prototype_countparameters as i32 + 1) as u32) << POSITION_C
                                    & !(0xFFFFFFFFu32 << 8) << POSITION_C;
                        }
                    }
                },
                | OPCODE_RETURN | OPCODE_TAILCALL => {
                    if (*function_state).functionstate_needsclose {
                        *program_counter = *program_counter & !(!(0xFFFFFFFFu32 << 1) << POSITION_K)
                            | (1 as u32) << POSITION_K & !(0xFFFFFFFFu32 << 1) << POSITION_K;
                    }
                    if (*prototype).prototype_isvariablearguments {
                        *program_counter = *program_counter & !(!(0xFFFFFFFFu32 << 8) << POSITION_C)
                            | (((*prototype).prototype_countparameters as i32 + 1) as u32) << POSITION_C
                                & !(0xFFFFFFFFu32 << 8) << POSITION_C;
                    }
                },
                | OPCODE_JMP => {
                    let target: i32 = final_target((*prototype).prototype_code.vectort_pointer, i);
                    fixjump(interpreter, lexical_state, function_state, i, target);
                },
                | _ => {},
            }
        }
    }
}
